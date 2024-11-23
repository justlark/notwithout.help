use std::sync::Arc;

use axum::{
    body::Body,
    http::{header::AUTHORIZATION, Request, Response, StatusCode},
    response::{ErrorResponse, IntoResponse},
};
use futures::future::{BoxFuture, FutureExt};
use tower_http::auth::AsyncRequireAuthorizationLayer;

use crate::{
    models::{ApiToken, FormId},
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

            let api_token = auth_header_value
                .strip_prefix(BEARER_PREFIX)
                .map(ApiToken::from)
                .ok_or_else(|| StatusCode::UNAUTHORIZED.into_response())?;

            req.extensions_mut().insert(api_token);

            Ok(req)
        }
        .boxed()
    })
}

pub async fn authorize(
    form_id: FormId,
    api_token: ApiToken,
    state: Arc<AppState>,
) -> Result<(), ErrorResponse> {
    let form = state
        .store
        .get_form(form_id)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if form.api_token == api_token {
        Ok(())
    } else {
        Err(StatusCode::UNAUTHORIZED.into())
    }
}
