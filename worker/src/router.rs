use std::sync::Arc;

use anyhow::anyhow;
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
    auth::{auth_layer, SignedApiAccessToken},
    cors::cors_layer,
    models::{ClientKeyId, EncryptedKeyComment, FormId, FormTemplate, SubmissionId},
    store::UnauthenticatedStore,
};

fn internal_err(err: anyhow::Error) -> ErrorResponse {
    console_error!("Error: {:?}", err);
    StatusCode::INTERNAL_SERVER_ERROR.into()
}

fn auth_err(err: anyhow::Error) -> ErrorResponse {
    console_error!("Error: {:?}", err);
    StatusCode::UNAUTHORIZED.into()
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
    pub store: UnauthenticatedStore,
    pub env: WorkerEnv,
}

pub fn new(state: AppState) -> Router {
    Router::new()
        // AUTHENTICATED ENDPOINTS
        .route("/submissions/:form_id", get(list_form_submissions))
        .route("/forms/:form_id", delete(delete_form))
        .route("/keys/:form_id/:key_index", get(get_key))
        .route("/keys/:form_id", get(list_keys))
        .route("/keys/:form_id", post(add_key))
        .route("/keys/:form_id/:key_index", delete(delete_key))
        .route_layer(auth_layer())
        // UNAUTHENTICATED ENDPOINTS
        .route("/forms/:form_id", get(get_form))
        .route("/forms", post(publish_form))
        .route("/submissions/:form_id", post(store_form_submission))
        .layer(cors_layer())
        .with_state(Arc::new(state))
}

#[axum::debug_handler]
async fn publish_form(
    State(state): State<Arc<AppState>>,
    Json(form): Json<PublishFormRequest>,
) -> Result<(StatusCode, Json<PublishFormResponse>), ErrorResponse> {
    let store = state.store.without_authenticating();

    let template = FormTemplate {
        org_name: form.org_name,
        description: form.description,
        contact_methods: form.contact_methods,
    };

    let form_id = FormId::new();

    store
        .put_form_template(form_id.clone(), template, form.public_primary_key)
        .await
        .map_err(internal_err)?;

    let client_key_id = store
        .store_client_keys(
            form_id.clone(),
            form.public_signing_key,
            None,
            EncryptedKeyComment::default(),
        )
        .await
        .map_err(internal_err)?
        .ok_or_else(|| {
            anyhow!("Could not find form associated with form ID, even though we just created it.")
        })
        .map_err(internal_err)?;

    let response = PublishFormResponse {
        form_id,
        client_key_id,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

#[axum::debug_handler]
async fn get_form(
    State(state): State<Arc<AppState>>,
    Path(form_id): Path<FormId>,
) -> Result<Json<GetFormResponse>, ErrorResponse> {
    let store = state.store.without_authenticating();

    Ok(Json(
        store
            .get_form_template(form_id)
            .await
            .map_err(internal_err)?
            .ok_or(StatusCode::NOT_FOUND)?
            .into(),
    ))
}

#[axum::debug_handler]
async fn store_form_submission(
    State(state): State<Arc<AppState>>,
    Path(form_id): Path<FormId>,
    body: String,
) -> Result<StatusCode, ErrorResponse> {
    let store = state.store.without_authenticating();

    let submission_id = SubmissionId::new();

    let changed = store
        .put_submission(form_id, submission_id, body.into())
        .await
        .map_err(internal_err)?;

    if changed {
        Ok(StatusCode::CREATED)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}

#[axum::debug_handler]
async fn delete_form(
    State(state): State<Arc<AppState>>,
    Extension(token): Extension<SignedApiAccessToken>,
    Path(form_id): Path<FormId>,
) -> Result<NoContent, ErrorResponse> {
    let store = token
        .validate(&state.store, form_id)
        .await
        .map_err(auth_err)?;

    todo!()
}

#[axum::debug_handler]
async fn list_form_submissions(
    State(state): State<Arc<AppState>>,
    Extension(token): Extension<SignedApiAccessToken>,
    Path(form_id): Path<FormId>,
) -> Result<Json<Vec<ListSubmissionsResponse>>, ErrorResponse> {
    let store = token
        .validate(&state.store, form_id)
        .await
        .map_err(auth_err)?;

    todo!()
}

#[axum::debug_handler]
async fn get_key(
    State(state): State<Arc<AppState>>,
    Extension(token): Extension<SignedApiAccessToken>,
    Path((form_id, key_id)): Path<(FormId, ClientKeyId)>,
) -> Result<Json<GetKeyResponse>, ErrorResponse> {
    let store = token
        .validate(&state.store, form_id)
        .await
        .map_err(auth_err)?;

    todo!()
}

#[axum::debug_handler]
async fn list_keys(
    State(state): State<Arc<AppState>>,
    Extension(token): Extension<SignedApiAccessToken>,
    Path(form_id): Path<FormId>,
) -> Result<Json<Vec<ListKeysResponse>>, ErrorResponse> {
    let store = token
        .validate(&state.store, form_id)
        .await
        .map_err(auth_err)?;

    todo!()
}

#[axum::debug_handler]
async fn add_key(
    State(state): State<Arc<AppState>>,
    Extension(token): Extension<SignedApiAccessToken>,
    Path(form_id): Path<FormId>,
    Json(body): Json<PostKeyRequest>,
) -> Result<(StatusCode, Json<PostKeyResponse>), ErrorResponse> {
    let store = token
        .validate(&state.store, form_id)
        .await
        .map_err(auth_err)?;

    todo!()
}

#[axum::debug_handler]
async fn delete_key(
    State(state): State<Arc<AppState>>,
    Extension(token): Extension<SignedApiAccessToken>,
    Path((form_id, key_id)): Path<(FormId, ClientKeyId)>,
) -> Result<NoContent, ErrorResponse> {
    let store = token
        .validate(&state.store, form_id)
        .await
        .map_err(auth_err)?;

    todo!()
}
