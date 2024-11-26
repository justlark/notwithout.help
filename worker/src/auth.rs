use std::str::FromStr;

use axum::{
    body::Body,
    http::{header::AUTHORIZATION, Request, Response, StatusCode},
    response::IntoResponse,
};
use futures::future::{BoxFuture, FutureExt};
use serde::Deserialize;
use tower_http::auth::AsyncRequireAuthorizationLayer;

use crate::models::{ClientKeyId, ServerKeyId};

const BEARER_PREFIX: &str = "Bearer ";

#[derive(Debug, Clone, Deserialize)]
pub struct ApiProof(String);

#[derive(Debug, Clone)]
pub struct ApiTokenParts {
    pub client_key_id: ClientKeyId,
    pub server_key_id: ServerKeyId,
    pub proof: ApiProof,
}

impl FromStr for ApiTokenParts {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split('.').collect::<Vec<_>>().as_slice() {
            [client_key_id, server_key_id, proof] => Ok(Self {
                client_key_id: client_key_id.parse()?,
                server_key_id: server_key_id.parse()?,
                proof: ApiProof(proof.to_string()),
            }),
            _ => Err(anyhow::anyhow!("API token is malformed.")),
        }
    }
}

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

            let token_parts = auth_header_value
                .strip_prefix(BEARER_PREFIX)
                .map(ApiTokenParts::from_str)
                .ok_or_else(|| StatusCode::UNAUTHORIZED.into_response())?
                .map_err(|_| StatusCode::UNAUTHORIZED.into_response())?;

            req.extensions_mut().insert(token_parts);

            Ok(req)
        }
        .boxed()
    })
}
