mod auth;
mod cors;
mod models;
mod secrets;
mod store;

use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Extension, Json, Path, State},
    http::{header::AUTHORIZATION, Response, StatusCode},
    response::{ErrorResponse, NoContent},
    routing::{delete, get, post},
    Router,
};
use tower_http::{sensitive_headers::SetSensitiveHeadersLayer, trace::TraceLayer};
use tower_service::Service;
use worker::{self, d1::D1Database, event, Context, Env, HttpRequest};

use auth::{auth_layer, authorize};
use cors::cors_layer;
use models::{ApiToken, FormId, FormResponse, FormTemplate, PublishFormResponse, Submission};
use store::Store;

const D1_BINDING: &str = "DB";

#[derive(Debug)]
pub struct AppState {
    store: Store,
}

fn router(db: D1Database) -> Router {
    Router::new()
        // Authenticated endpoints.
        .route("/forms/:form_id", delete(delete_form))
        .route("/submissions/:form_id", get(list_form_submissions))
        .route_layer(auth_layer())
        // Unauthenticated endpoints.
        .route("/forms", post(publish_form))
        .route("/forms/:form_id", get(get_form))
        .route("/submissions/:form_id", post(store_form_submission))
        .layer(cors_layer())
        .layer(SetSensitiveHeadersLayer::new([AUTHORIZATION]))
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::new(AppState {
            store: Store::new(db),
        }))
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
) -> Result<Json<Vec<Submission>>, ErrorResponse> {
    authorize(form_id, api_token, Arc::clone(&state)).await?;
    todo!()
}
