use axum::http::{
    header::{HeaderName, CONTENT_TYPE},
    HeaderValue, Method,
};
use axum::{body::Bytes, extract::Path, http::StatusCode, routing::put, Router};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tower_service::Service;
use worker::*;

const CORS_ALLOWED_ORIGINS: [&str; 1] = ["https://example.com"];
const CORS_ALLOWED_METHODS: [Method; 3] = [Method::GET, Method::PUT, Method::DELETE];
const CORS_ALLOWED_HEADERS: [HeaderName; 1] = [CONTENT_TYPE];

fn router() -> Router {
    let cors_layer = CorsLayer::new()
        .allow_methods(CORS_ALLOWED_METHODS)
        .allow_headers(CORS_ALLOWED_HEADERS);

    let cors_layer = CORS_ALLOWED_ORIGINS
        .iter()
        .fold(cors_layer, |cors_layer, origin| {
            cors_layer.allow_origin(origin.parse::<HeaderValue>().unwrap())
        });

    Router::new()
        .route("/submissions/:id", put(store_form_response))
        .layer(TraceLayer::new_for_http())
        .layer(cors_layer)
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    Ok(router().call(req).await?)
}

pub async fn store_form_response(Path(id): Path<String>, body: Bytes) -> StatusCode {
    StatusCode::CREATED
}
