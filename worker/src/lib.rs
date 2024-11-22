mod models;

use axum::{
    body::Body,
    extract::{Json, Path},
    http::{
        header::{HeaderName, CONTENT_TYPE},
        HeaderValue, Method, Response, StatusCode,
    },
    routing::{delete, get, post},
    Router,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tower_service::Service;
use worker::*;

use models::{EncryptedFormResponse, FormId, FormResponse, FormTemplate};

const CORS_ALLOWED_ORIGINS: [&str; 1] = ["https://example.com"];
const CORS_ALLOWED_METHODS: [Method; 3] = [Method::GET, Method::POST, Method::DELETE];
const CORS_ALLOWED_HEADERS: [HeaderName; 1] = [CONTENT_TYPE];

fn cors_layer() -> CorsLayer {
    let cors_layer = CorsLayer::new()
        .allow_methods(CORS_ALLOWED_METHODS)
        .allow_headers(CORS_ALLOWED_HEADERS);

    CORS_ALLOWED_ORIGINS
        .iter()
        .fold(cors_layer, |cors_layer, origin| {
            cors_layer.allow_origin(origin.parse::<HeaderValue>().unwrap())
        })
}

fn router() -> Router {
    Router::new()
        .route("/forms", post(publish_form))
        .route("/forms/:form_id", get(get_form))
        .route("/forms/:form_id", delete(delete_form))
        .route("/responses/:form_id", post(store_form_response))
        .route("/responses/:form_id", get(list_form_responses))
        .route("/responses/:form_id", delete(delete_form_response))
        .layer(TraceLayer::new_for_http())
        .layer(cors_layer())
}

#[event(fetch)]
async fn fetch(req: HttpRequest, _env: Env, _ctx: Context) -> Result<Response<Body>> {
    console_error_panic_hook::set_once();
    Ok(router().call(req).await?)
}

pub async fn publish_form(Json(template): Json<FormTemplate>) -> Json<FormResponse> {
    todo!()
}

pub async fn get_form(Path(form_id): Path<FormId>) -> Json<FormTemplate> {
    todo!()
}

pub async fn delete_form(Path(form_id): Path<FormId>) -> StatusCode {
    todo!()
}

pub async fn store_form_response(
    Path(form_id): Path<FormId>,
    body: EncryptedFormResponse,
) -> StatusCode {
    todo!()
}

pub async fn list_form_responses(Path(form_id): Path<FormId>) -> Json<Vec<EncryptedFormResponse>> {
    todo!()
}

pub async fn delete_form_response(Path(form_id): Path<FormId>) -> StatusCode {
    todo!()
}
