use axum::http::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    HeaderName, Method,
};
use tower_http::cors::CorsLayer;

const CORS_ALLOWED_ORIGINS: [&str; 3] = [
    // Local development
    "http://localhost:5173",
    // Prod environment
    "https://notwithout.help",
    // Dev environment
    "https://main.notwithouthelp.pages.dev",
];

const CORS_ALLOWED_METHODS: [Method; 4] =
    [Method::GET, Method::POST, Method::PATCH, Method::DELETE];

const CORS_ALLOWED_HEADERS: [HeaderName; 2] = [CONTENT_TYPE, AUTHORIZATION];

pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_methods(CORS_ALLOWED_METHODS)
        .allow_headers(CORS_ALLOWED_HEADERS)
        .allow_origin(
            CORS_ALLOWED_ORIGINS
                .iter()
                .map(|origin| origin.parse().unwrap())
                .collect::<Vec<_>>(),
        )
}
