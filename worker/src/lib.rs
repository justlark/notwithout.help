mod auth;
mod models;
mod secrets;
mod store;

use std::{fmt, sync::Arc};

use axum::{
    body::Body,
    extract::{Extension, Json, Path, State},
    http::{
        header::{HeaderName, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method, Response, StatusCode,
    },
    response::{ErrorResponse, NoContent},
    routing::{delete, get, post},
    Router,
};
use tower_http::{cors::CorsLayer, sensitive_headers::SetSensitiveHeadersLayer, trace::TraceLayer};
use tower_service::Service;
use worker::{self, d1::D1Database, event, Context, Env, HttpRequest};

use auth::{auth_layer, authorize};
use models::{
    ApiToken, EncryptedSubmission, FormId, FormResponse, FormTemplate, PublishFormResponse,
};

const CORS_ALLOWED_ORIGINS: [&str; 1] = ["https://example.com"];
const CORS_ALLOWED_METHODS: [Method; 3] = [Method::GET, Method::POST, Method::DELETE];
const CORS_ALLOWED_HEADERS: [HeaderName; 1] = [CONTENT_TYPE];

const D1_BINDING: &str = "DB";

pub struct AppState {
    db: D1Database,
}

impl fmt::Debug for AppState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppState").finish_non_exhaustive()
    }
}

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

fn router(db: D1Database) -> Router {
    Router::new()
        // Authenticated endpoints.
        .route("/forms/:form_id", delete(delete_form))
        .route("/submissions/:form_id", get(list_form_submissions))
        .route("/submissions/:form_id", delete(delete_form_submission))
        .route_layer(auth_layer())
        // Unauthenticated endpoints.
        .route("/forms", post(publish_form))
        .route("/forms/:form_id", get(get_form))
        .route("/submissions/:form_id", post(store_form_submission))
        .layer(cors_layer())
        .layer(SetSensitiveHeadersLayer::new([AUTHORIZATION]))
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::new(AppState { db }))
}

#[event(fetch)]
async fn fetch(req: HttpRequest, env: Env, _ctx: Context) -> worker::Result<Response<Body>> {
    console_error_panic_hook::set_once();
    let kv = env.d1(D1_BINDING)?;
    Ok(router(kv).call(req).await?)
}

#[axum::debug_handler]
pub async fn publish_form(
    State(state): State<Arc<AppState>>,
    Json(template): Json<FormTemplate>,
) -> Result<Json<PublishFormResponse>, ErrorResponse> {
    todo!()
}

#[axum::debug_handler]
pub async fn get_form(
    State(state): State<Arc<AppState>>,
    Path(form_id): Path<FormId>,
) -> Result<Json<FormResponse>, ErrorResponse> {
    todo!()
}

#[axum::debug_handler]
pub async fn delete_form(
    State(state): State<Arc<AppState>>,
    Extension(api_token): Extension<ApiToken>,
    Path(form_id): Path<FormId>,
) -> Result<NoContent, ErrorResponse> {
    authorize(form_id, api_token, Arc::clone(&state)).await?;
    todo!()
}

#[axum::debug_handler]
pub async fn store_form_submission(
    State(state): State<Arc<AppState>>,
    Path(form_id): Path<FormId>,
    body: String,
) -> Result<StatusCode, ErrorResponse> {
    todo!()
}

#[axum::debug_handler]
pub async fn list_form_submissions(
    State(state): State<Arc<AppState>>,
    Extension(api_token): Extension<ApiToken>,
    Path(form_id): Path<FormId>,
) -> Result<Json<Vec<EncryptedSubmission>>, ErrorResponse> {
    authorize(form_id, api_token, Arc::clone(&state)).await?;
    todo!()
}

#[axum::debug_handler]
pub async fn delete_form_submission(
    State(state): State<Arc<AppState>>,
    Extension(api_token): Extension<ApiToken>,
    Path(form_id): Path<FormId>,
) -> Result<NoContent, ErrorResponse> {
    authorize(form_id, api_token, Arc::clone(&state)).await?;
    todo!()
}
