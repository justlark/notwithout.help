use std::sync::Arc;

use axum::{
    extract::{Extension, Json, Path, State},
    http::StatusCode,
    response::{ErrorResponse, NoContent},
    routing::{delete, get, post},
    Router,
};
use worker::{console_error, Env};

use crate::{
    api::{
        GetFormResponse, GetKeyResponse, ListKeysResponse, ListSubmissionsResponse, PostKeyRequest,
        PostKeyResponse, PublishFormRequest, PublishFormResponse,
    },
    auth::{auth_layer, ApiTokenParts},
    cors::cors_layer,
    keys::FormId,
    models::ClientKeyId,
    store::Store,
};

fn handle_err(err: anyhow::Error) -> ErrorResponse {
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
    pub store: Store,
    pub env: WorkerEnv,
}

pub fn new(state: AppState) -> Router {
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

#[axum::debug_handler]
async fn publish_form(
    State(state): State<Arc<AppState>>,
    Json(form): Json<PublishFormRequest>,
) -> Result<(StatusCode, Json<PublishFormResponse>), ErrorResponse> {
    todo!()
}

#[axum::debug_handler]
async fn get_form(
    State(state): State<Arc<AppState>>,
    Path(form_id): Path<FormId>,
) -> Result<Json<GetFormResponse>, ErrorResponse> {
    todo!()
}

#[axum::debug_handler]
async fn store_form_submission(
    State(state): State<Arc<AppState>>,
    Path(form_id): Path<FormId>,
    body: String,
) -> Result<StatusCode, ErrorResponse> {
    todo!()
}

#[axum::debug_handler]
async fn delete_form(
    State(state): State<Arc<AppState>>,
    Extension(token): Extension<ApiTokenParts>,
    Path(form_id): Path<FormId>,
) -> Result<NoContent, ErrorResponse> {
    todo!()
}

#[axum::debug_handler]
async fn list_form_submissions(
    State(state): State<Arc<AppState>>,
    Extension(token): Extension<ApiTokenParts>,
    Path(form_id): Path<FormId>,
) -> Result<Json<Vec<ListSubmissionsResponse>>, ErrorResponse> {
    todo!()
}

#[axum::debug_handler]
async fn get_key(
    State(state): State<Arc<AppState>>,
    Extension(token): Extension<ApiTokenParts>,
    Path((form_id, key_id)): Path<(FormId, ClientKeyId)>,
) -> Result<Json<GetKeyResponse>, ErrorResponse> {
    todo!()
}

#[axum::debug_handler]
async fn list_keys(
    State(state): State<Arc<AppState>>,
    Extension(token): Extension<ApiTokenParts>,
    Path(form_id): Path<FormId>,
) -> Result<Json<Vec<ListKeysResponse>>, ErrorResponse> {
    todo!()
}

#[axum::debug_handler]
async fn add_key(
    State(state): State<Arc<AppState>>,
    Extension(token): Extension<ApiTokenParts>,
    Path(form_id): Path<FormId>,
    Json(body): Json<PostKeyRequest>,
) -> Result<(StatusCode, Json<PostKeyResponse>), ErrorResponse> {
    todo!()
}

#[axum::debug_handler]
async fn delete_key(
    State(state): State<Arc<AppState>>,
    Extension(token): Extension<ApiTokenParts>,
    Path((form_id, key_id)): Path<(FormId, ClientKeyId)>,
) -> Result<NoContent, ErrorResponse> {
    todo!()
}
