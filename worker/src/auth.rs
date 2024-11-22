use axum::{
    body::Body,
    http::{header::AUTHORIZATION, Request, Response, StatusCode},
};
use futures::future::{BoxFuture, FutureExt};
use tower_http::auth::AsyncRequireAuthorizationLayer;

use crate::models::ApiToken;

const BEARER_PREFIX: &str = "Bearer ";

fn unauthorized_response() -> Response<Body> {
    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .body(Body::empty())
        .unwrap()
}

pub fn auth_layer<'a>() -> AsyncRequireAuthorizationLayer<
    impl Fn(Request<Body>) -> BoxFuture<'a, Result<Request<Body>, Response<Body>>> + Clone,
> {
    AsyncRequireAuthorizationLayer::new(|mut req: Request<Body>| {
        async move {
            let auth_header = req
                .headers()
                .get(AUTHORIZATION)
                .ok_or_else(unauthorized_response)?;

            let auth_header_value = auth_header.to_str().map_err(|_| unauthorized_response())?;

            let api_token = auth_header_value
                .strip_prefix(BEARER_PREFIX)
                .map(ApiToken::from)
                .ok_or_else(unauthorized_response)?;

            req.extensions_mut().insert(api_token);

            Ok(req)
        }
        .boxed()
    })
}
