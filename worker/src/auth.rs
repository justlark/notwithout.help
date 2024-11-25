use std::sync::Arc;

use axum::{
    body::Body,
    http::{header::AUTHORIZATION, Request, Response, StatusCode},
    response::{ErrorResponse, IntoResponse},
};
use futures::future::{BoxFuture, FutureExt};
use tower_http::auth::AsyncRequireAuthorizationLayer;

use crate::{
    models::{ApiSecret, FormId},
    AppState,
};

const BEARER_PREFIX: &str = "Bearer ";

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

            let api_secret = auth_header_value
                .strip_prefix(BEARER_PREFIX)
                .map(ApiSecret::from_base64)
                .ok_or_else(|| StatusCode::UNAUTHORIZED.into_response())?
                .map_err(|_| StatusCode::UNAUTHORIZED.into_response())?;

            req.extensions_mut().insert(api_secret);

            Ok(req)
        }
        .boxed()
    })
}

pub async fn authorize(
    form_id: FormId,
    api_secret: ApiSecret,
    state: Arc<AppState>,
) -> Result<(), ErrorResponse> {
    let form = state
        .store
        .get_form(form_id)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let hashed_api_secret = api_secret
        .to_hashed()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if form.hashed_api_secret == hashed_api_secret {
        Ok(())
    } else {
        Err(StatusCode::UNAUTHORIZED.into())
    }
}
