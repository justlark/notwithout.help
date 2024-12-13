use axum::http::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    HeaderName, HeaderValue, Method,
};
use tower_http::cors::CorsLayer;

use crate::config;

const CORS_ALLOWED_METHODS: [Method; 4] =
    [Method::GET, Method::POST, Method::PATCH, Method::DELETE];

const CORS_ALLOWED_HEADERS: [HeaderName; 2] = [CONTENT_TYPE, AUTHORIZATION];

pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_methods(CORS_ALLOWED_METHODS)
        .allow_headers(CORS_ALLOWED_HEADERS)
        .allow_origin(
            config::cors_allowed_origin()
                .parse::<HeaderValue>()
                .unwrap(),
        )
}
