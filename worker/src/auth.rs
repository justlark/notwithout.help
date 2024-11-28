use anyhow::bail;
use axum::{
    body::Body,
    http::{header::AUTHORIZATION, Request, Response, StatusCode},
    response::IntoResponse,
};
use futures::future::{BoxFuture, FutureExt};
use jsonwebtoken as jwt;
use serde::Deserialize;
use tower_http::auth::AsyncRequireAuthorizationLayer;

use crate::{
    models::FormId,
    store::{Store, UnauthenticatedStore},
};

const BEARER_PREFIX: &str = "Bearer ";
const JWT_ALGORITHM: jwt::Algorithm = jwt::Algorithm::HS256;

const EXPECTED_AUD: [&str; 2] = [
    "https://api.notwithout.help",
    "https://api-dev.notwithout.help",
];
const EXPECTED_ISS: [&str; 2] = EXPECTED_AUD;

#[derive(Debug, Clone)]
pub struct ApiAccessToken(String);

#[derive(Debug, Deserialize)]
struct ApiAccessTokenClaims {
    pub sub: String,
}

impl ApiAccessToken {
    pub async fn validate(
        self,
        store: &UnauthenticatedStore,
        form_id: FormId,
    ) -> anyhow::Result<&Store> {
        let store = store.without_authenticating();

        let header = jwt::decode_header(&self.0)?;

        let server_key_id = header
            .kid
            .ok_or_else(|| anyhow::anyhow!("Access token is missing the `kid` claim."))?
            .into();

        let ephemeral_server_key = store.get_ephemeral_server_key(server_key_id).await?;

        let mut validation = jwt::Validation::new(JWT_ALGORITHM);
        validation.required_spec_claims = ["exp", "sub", "aud", "iss"]
            .into_iter()
            .map(String::from)
            .collect();
        validation.aud = Some(EXPECTED_AUD.into_iter().map(String::from).collect());
        validation.iss = Some(EXPECTED_ISS.into_iter().map(String::from).collect());
        validation.algorithms = vec![JWT_ALGORITHM];

        let token_claims = jwt::decode::<ApiAccessTokenClaims>(
            &self.0,
            &ephemeral_server_key.decoding_key(),
            &validation,
        )?
        .claims;

        let (sub_form_id, sub_client_key_id) = match token_claims
            .sub
            .splitn(2, '/')
            .collect::<Vec<_>>()
            .as_slice()
        {
            [form_id, client_key_id] => (form_id.to_string().into(), client_key_id.parse()?),
            _ => bail!("Access token `sub` claim is not in the expected format."),
        };

        if sub_form_id != form_id {
            bail!("Form ID in access token `sub` does not match the form being accessed.");
        }

        let client_keys = store
            .get_client_keys(sub_form_id, sub_client_key_id)
            .await?;

        if client_keys.is_none() {
            bail!("Client key in access token `sub` does not exist or has been revoked.");
        }

        Ok(store)
    }
}

// Extract the bearer token from the Authorization header and insert it into the request
// extensions.
pub fn auth_layer<'a>() -> AsyncRequireAuthorizationLayer<
    impl Fn(Request<Body>) -> BoxFuture<'a, Result<Request<Body>, Response<Body>>> + Clone,
> {
    AsyncRequireAuthorizationLayer::new(|mut req: Request<Body>| {
        async move {
            let auth_header = req
                .headers()
                .get(AUTHORIZATION)
                .ok_or_else(|| StatusCode::UNAUTHORIZED.into_response())?;

            let auth_header_value = auth_header
                .to_str()
                .map_err(|_| StatusCode::UNAUTHORIZED.into_response())?;

            let token = auth_header_value
                .strip_prefix(BEARER_PREFIX)
                .map(|token| ApiAccessToken(token.to_string()))
                .ok_or_else(|| StatusCode::UNAUTHORIZED.into_response())?;

            req.extensions_mut().insert(token);

            Ok(req)
        }
        .boxed()
    })
}
