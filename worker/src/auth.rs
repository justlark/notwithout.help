use std::str::FromStr;

use anyhow::{anyhow, Context};
use axum::{
    body::Body,
    http::{header::AUTHORIZATION, Request, Response, StatusCode},
    response::IntoResponse,
};
use base64::prelude::*;
use futures::future::{BoxFuture, FutureExt};
use tower_http::auth::AsyncRequireAuthorizationLayer;

use crate::{
    crypt::CryptBox,
    models::{ClientKeyId, FormId, ServerKeyId},
    store::Store,
};

const BEARER_PREFIX: &str = "Bearer ";

#[derive(Debug, Clone)]
pub struct ApiProof(Vec<u8>);

impl FromStr for ApiProof {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let decoded = BASE64_STANDARD
            .decode(s)
            .context("API proof is not a valid base64-encoded string.")?;

        Ok(Self(decoded))
    }
}

// The API bearer token has the following format:
//
//   <client_key_id>.<server_key_id>.<proof>
//
#[derive(Debug, Clone)]
pub struct ApiToken {
    client_key_id: ClientKeyId,
    server_key_id: ServerKeyId,
    proof: ApiProof,
}

impl ApiToken {
    pub async fn verify(self, store: &Store, form_id: FormId) -> anyhow::Result<()> {
        let client_keys = store
            .get_client_keys(form_id.clone(), self.client_key_id)
            .await?
            .ok_or_else(|| anyhow!("Client keys with id {:?} not found.", self.client_key_id))?;

        let server_keys = store
            .get_server_keys(form_id, self.server_key_id)
            .await?
            .ok_or_else(|| anyhow!("Server keys with id {:?} not found.", self.server_key_id))?;

        let crypt_box = CryptBox::new(
            client_keys.public_wrapping_key.as_ref(),
            server_keys.private_key.as_ref(),
        );

        crypt_box
            .decrypt(&self.proof.0)
            .context("API proof is invalid. Failed to authenticate.")?;

        Ok(())
    }
}

impl FromStr for ApiToken {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split('.').collect::<Vec<_>>().as_slice() {
            [client_key_id, server_key_id, proof] => Ok(Self {
                client_key_id: client_key_id.parse()?,
                server_key_id: server_key_id.parse()?,
                proof: proof.parse()?,
            }),
            _ => Err(anyhow::anyhow!("API token is malformed.")),
        }
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
                .map(ApiToken::from_str)
                .ok_or_else(|| StatusCode::UNAUTHORIZED.into_response())?
                .map_err(|_| StatusCode::UNAUTHORIZED.into_response())?;

            req.extensions_mut().insert(token);

            Ok(req)
        }
        .boxed()
    })
}
