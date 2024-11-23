use axum::http::{header::CONTENT_TYPE, HeaderName, HeaderValue, Method};
use tower_http::cors::CorsLayer;

const CORS_ALLOWED_ORIGINS: [&str; 1] = ["https://example.com"];
const CORS_ALLOWED_METHODS: [Method; 3] = [Method::GET, Method::POST, Method::DELETE];
const CORS_ALLOWED_HEADERS: [HeaderName; 1] = [CONTENT_TYPE];

pub fn cors_layer() -> CorsLayer {
    let cors_layer = CorsLayer::new()
        .allow_methods(CORS_ALLOWED_METHODS)
        .allow_headers(CORS_ALLOWED_HEADERS);

    CORS_ALLOWED_ORIGINS
        .iter()
        .fold(cors_layer, |cors_layer, origin| {
            cors_layer.allow_origin(origin.parse::<HeaderValue>().unwrap())
        })
}
