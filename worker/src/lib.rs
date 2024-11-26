#![deny(unsafe_code)]
#![warn(missing_debug_implementations)]
#![allow(dead_code)] // TODO: Remove
#![allow(unused_variables)] // TODO: Remove

mod api;
mod auth;
mod cors;
mod keys;
mod models;
mod store;

use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Extension, Json, Path, State},
    http::{Response, StatusCode},
    response::{ErrorResponse, NoContent},
    routing::{delete, get, post},
    Router,
};
use models::ClientKeyId;
use tower_service::Service;
use worker::{self, console_error, event, Context, Env, HttpRequest};

use api::{
    GetFormResponse, GetKeyResponse, ListKeysResponse, ListSubmissionsResponse, PostKeyRequest,
    PostKeyResponse, PublishFormRequest, PublishFormResponse,
};
use auth::{auth_layer, ApiTokenParts};
use cors::cors_layer;
use keys::FormId;
use store::Store;

const D1_BINDING: &str = "DB";

fn handle_error(err: anyhow::Error) -> ErrorResponse {
    console_error!("Error: {:?}", err);
    StatusCode::INTERNAL_SERVER_ERROR.into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorkerEnv {
    Dev,
    Prod,
}

impl WorkerEnv {
    pub fn get(env: &Env) -> Self {
        match env.var("WORKER_ENV").unwrap().to_string().as_str() {
            "dev" => Self::Dev,
            "prod" => Self::Prod,
            _ => panic!("var WORKER_ENV must be either 'prod' or 'dev'"),
        }
    }
}

#[derive(Debug)]
pub struct AppState {
    store: Store,
    env: WorkerEnv,
}

fn router(state: AppState) -> Router {
    Router::new()
        // AUTHENTICATED ENDPOINTS
        .route("/submissions/:form_id", get(list_form_submissions))
        .route("/forms/:form_id", delete(delete_form))
        .route("/keys/:form_id/:key_index", get(get_key))
        .route("/keys/:form_id", post(add_key))
        .route("/keys/:form_id", get(list_keys))
        .route("/keys/:form_id/:key_index", delete(delete_key))
        .route_layer(auth_layer())
        // UNAUTHENTICATED ENDPOINTS
        .route("/forms", post(publish_form))
        .route("/forms/:form_id", get(get_form))
        .route("/submissions/:form_id", post(store_form_submission))
        .layer(cors_layer())
        .with_state(Arc::new(state))
}

#[event(fetch)]
async fn fetch(req: HttpRequest, env: Env, _ctx: Context) -> worker::Result<Response<Body>> {
    console_error_panic_hook::set_once();

    let state = AppState {
        store: Store::new(env.d1(D1_BINDING)?),
        env: WorkerEnv::get(&env),
    };

    Ok(router(state).call(req).await?)
}

#[axum::debug_handler]
pub async fn publish_form(
    State(state): State<Arc<AppState>>,
    Json(form): Json<PublishFormRequest>,
) -> Result<(StatusCode, Json<PublishFormResponse>), ErrorResponse> {
    todo!()
}

#[axum::debug_handler]
pub async fn get_form(
    State(state): State<Arc<AppState>>,
    Path(form_id): Path<FormId>,
) -> Result<Json<GetFormResponse>, ErrorResponse> {
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
pub async fn delete_form(
    State(state): State<Arc<AppState>>,
    Extension(token): Extension<ApiTokenParts>,
    Path(form_id): Path<FormId>,
) -> Result<NoContent, ErrorResponse> {
    todo!()
}

#[axum::debug_handler]
pub async fn list_form_submissions(
    State(state): State<Arc<AppState>>,
    Extension(token): Extension<ApiTokenParts>,
    Path(form_id): Path<FormId>,
) -> Result<Json<Vec<ListSubmissionsResponse>>, ErrorResponse> {
    todo!()
}

#[axum::debug_handler]
pub async fn get_key(
    State(state): State<Arc<AppState>>,
    Extension(token): Extension<ApiTokenParts>,
    Path((form_id, key_id)): Path<(FormId, ClientKeyId)>,
) -> Result<Json<GetKeyResponse>, ErrorResponse> {
    todo!()
}

#[axum::debug_handler]
pub async fn list_keys(
    State(state): State<Arc<AppState>>,
    Extension(token): Extension<ApiTokenParts>,
    Path(form_id): Path<FormId>,
) -> Result<Json<Vec<ListKeysResponse>>, ErrorResponse> {
    todo!()
}

#[axum::debug_handler]
pub async fn add_key(
    State(state): State<Arc<AppState>>,
    Extension(token): Extension<ApiTokenParts>,
    Path(form_id): Path<FormId>,
    Json(body): Json<PostKeyRequest>,
) -> Result<(StatusCode, Json<PostKeyResponse>), ErrorResponse> {
    todo!()
}

#[axum::debug_handler]
pub async fn delete_key(
    State(state): State<Arc<AppState>>,
    Extension(token): Extension<ApiTokenParts>,
    Path((form_id, key_id)): Path<(FormId, ClientKeyId)>,
) -> Result<NoContent, ErrorResponse> {
    todo!()
}
