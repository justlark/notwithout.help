use std::sync::Arc;

use anyhow::anyhow;
use axum::{
    extract::{DefaultBodyLimit, Extension, Json, Path, State},
    http::StatusCode,
    response::{ErrorResponse, NoContent},
    routing::{delete, get, patch, post},
    Router,
};
use chrono::DateTime;
use worker::console_error;

use crate::{
    api::{
        GetApiChallengeResponse, GetFormResponse, GetKeyResponse, ListKeysResponse,
        ListSubmissionsResponse, PatchFormRequest, PatchKeyRequest, PostFormRequest,
        PostFormResponse, PostKeyRequest, PostKeyResponse, PostSubmissionRequest, PostTokenRequest,
        PostTokenResponse,
    },
    auth::{
        auth_layer, AccessRole, ApiChallenge, ApiChallengeResponse, AuthError, AuthErrorType,
        SignedApiAccessToken,
    },
    config,
    cors::cors_layer,
    keys::{ApiChallengeNonce, EphemeralServerKey},
    models::{
        ChallengeId, ClientKeyId, EncryptedKeyComment, FormId, FormTemplate, FormUpdate,
        ServerKeyId, SubmissionId,
    },
    store::{UnauthenticatedStore, FORM_TEMPLATE_CURRENT_VERSION},
};

fn internal_err(err: anyhow::Error) -> ErrorResponse {
    console_error!("Error: {}", err);
    StatusCode::INTERNAL_SERVER_ERROR.into()
}

fn auth_err(err: AuthError) -> ErrorResponse {
    console_error!("Error: {}", err);
    match err.kind() {
        AuthErrorType::Unauthorized => StatusCode::UNAUTHORIZED.into(),
        AuthErrorType::Forbidden => StatusCode::FORBIDDEN.into(),
    }
}

#[derive(Debug)]
pub struct AppState {
    pub store: UnauthenticatedStore,
}

pub fn new(state: AppState) -> Router {
    Router::new()
        // AUTHENTICATED ENDPOINTS
        .route("/submissions/:form_id", get(list_form_submissions))
        .route("/forms/:form_id", delete(delete_form))
        .route("/forms/:form_id", patch(edit_form))
        .route("/keys/:form_id/:client_key_id", get(get_key))
        .route("/keys/:form_id", get(list_keys))
        .route("/keys/:form_id", post(add_key))
        .route("/keys/:form_id/:client_key_id", patch(update_key))
        .route("/keys/:form_id/:client_key_id", delete(delete_key))
        .route_layer(auth_layer())
        // UNAUTHENTICATED ENDPOINTS
        .route("/forms/:form_id", get(get_form))
        .route("/forms", post(publish_form))
        .route("/submissions/:form_id", post(store_form_submission))
        .route(
            "/challenges/:form_id/:client_key_id",
            post(request_challenge),
        )
        .route("/tokens", post(request_access_token))
        .layer(cors_layer())
        .layer(DefaultBodyLimit::max(config::max_request_body_len()))
        .with_state(Arc::new(state))
}

#[axum::debug_handler]
async fn publish_form(
    State(state): State<Arc<AppState>>,
    Json(form): Json<PostFormRequest>,
) -> Result<(StatusCode, Json<PostFormResponse>), ErrorResponse> {
    let store = state.store.without_authenticating();

    let template = FormTemplate {
        version: FORM_TEMPLATE_CURRENT_VERSION,
        org_name: form.org_name,
        description: form.description,
        contact_methods: form.contact_methods,
    };

    let form_id = FormId::new();

    store
        .put_form_template(
            &form_id,
            &template,
            &form.public_primary_key,
            match form.expires_at {
                Some(expires_at) => Some(
                    DateTime::parse_from_rfc3339(&expires_at)
                        .map_err(|err| internal_err(err.into()))?
                        .to_utc(),
                ),
                None => None,
            },
        )
        .await
        .map_err(internal_err)?;

    let client_key_id = store
        .store_client_keys(
            &form_id,
            &form.public_signing_key,
            None,
            &EncryptedKeyComment::default(),
            // The initial secret link will always have admin access.
            AccessRole::Admin,
        )
        .await
        .map_err(internal_err)?
        .ok_or_else(|| {
            anyhow!("Could not find form associated with form ID, even though we just created it.")
        })
        .map_err(internal_err)?;

    let response = PostFormResponse {
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
            .get_form_data(&form_id)
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
    Json(body): Json<PostSubmissionRequest>,
) -> Result<StatusCode, ErrorResponse> {
    let store = state.store.without_authenticating();

    let submission_id = SubmissionId::new();

    let changed = store
        .put_submission(&form_id, &submission_id, &body.encrypted_body)
        .await
        .map_err(internal_err)?;

    if changed {
        Ok(StatusCode::CREATED)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}

#[axum::debug_handler]
async fn request_challenge(
    State(state): State<Arc<AppState>>,
    Path((form_id, client_key_id)): Path<(FormId, ClientKeyId)>,
) -> Result<Json<GetApiChallengeResponse>, ErrorResponse> {
    let store = state.store.without_authenticating();

    let server_key_id = ServerKeyId::new();
    let ephemeral_server_key = EphemeralServerKey::generate();

    store
        .store_ephemeral_server_key(&server_key_id, &ephemeral_server_key)
        .await
        .map_err(internal_err)?;

    let challenge_id = ChallengeId::new();

    store
        .store_challenge_id(&challenge_id)
        .await
        .map_err(internal_err)?;

    let challenge = ApiChallenge {
        server_key_id,
        form_id,
        client_key_id,
        challenge_id,
        nonce: ApiChallengeNonce::generate(),
        origin: config::current_origin(),
        exp: config::challenge_token_exp(),
    };

    let signed_challenge = challenge
        .encode(&ephemeral_server_key.encoding_key())
        .map_err(internal_err)?;

    Ok(Json(GetApiChallengeResponse {
        challenge: signed_challenge,
    }))
}

#[axum::debug_handler]
async fn request_access_token(
    State(state): State<Arc<AppState>>,
    Json(body): Json<PostTokenRequest>,
) -> Result<Json<PostTokenResponse>, ErrorResponse> {
    let store = state.store.without_authenticating();

    let validated_challenge = ApiChallengeResponse::from(body)
        .validate(store)
        .await
        .map_err(|err| {
            console_error!("{}", err);
            StatusCode::UNAUTHORIZED
        })?;

    let ephemeral_server_key = store
        .get_ephemeral_server_key(&validated_challenge.server_key_id())
        .await
        .map_err(internal_err)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = validated_challenge
        .into_access_token(
            &ephemeral_server_key.encoding_key(),
            config::access_token_exp(),
        )
        .map_err(internal_err)?;

    Ok(Json(PostTokenResponse { token }))
}

#[axum::debug_handler]
async fn list_form_submissions(
    State(state): State<Arc<AppState>>,
    Extension(token): Extension<SignedApiAccessToken>,
    Path(form_id): Path<FormId>,
) -> Result<Json<Vec<ListSubmissionsResponse>>, ErrorResponse> {
    let store = token
        .validate(&state.store, &form_id, AccessRole::Read)
        .await
        .map_err(auth_err)?;

    let submissions = store
        .list_submissions(&form_id)
        .await
        .map_err(internal_err)?;

    Ok(Json(submissions.into_iter().map(From::from).collect()))
}

#[axum::debug_handler]
async fn delete_form(
    State(state): State<Arc<AppState>>,
    Extension(token): Extension<SignedApiAccessToken>,
    Path(form_id): Path<FormId>,
) -> Result<NoContent, ErrorResponse> {
    let store = token
        .validate(&state.store, &form_id, AccessRole::Admin)
        .await
        .map_err(auth_err)?;

    store.delete_form(&form_id).await.map_err(internal_err)?;

    Ok(NoContent)
}

#[axum::debug_handler]
async fn edit_form(
    State(state): State<Arc<AppState>>,
    Extension(token): Extension<SignedApiAccessToken>,
    Path(form_id): Path<FormId>,
    Json(body): Json<PatchFormRequest>,
) -> Result<NoContent, ErrorResponse> {
    let store = token
        .validate(&state.store, &form_id, AccessRole::Admin)
        .await
        .map_err(auth_err)?;

    let form_update = FormUpdate {
        template: FormTemplate {
            version: FORM_TEMPLATE_CURRENT_VERSION,
            org_name: body.org_name,
            description: body.description,
            contact_methods: body.contact_methods,
        },
        expires_at: match body.expires_at {
            Some(expires_at) => Some(
                DateTime::parse_from_rfc3339(&expires_at)
                    .map_err(|err| internal_err(err.into()))?
                    .to_utc(),
            ),
            None => None,
        },
    };

    store
        .edit_form(&form_id, &form_update)
        .await
        .map_err(internal_err)?;

    Ok(NoContent)
}

#[axum::debug_handler]
async fn get_key(
    State(state): State<Arc<AppState>>,
    Extension(token): Extension<SignedApiAccessToken>,
    Path((form_id, key_id)): Path<(FormId, ClientKeyId)>,
) -> Result<Json<GetKeyResponse>, ErrorResponse> {
    let store = token
        .validate(&state.store, &form_id, AccessRole::Read)
        .await
        .map_err(auth_err)?;

    let client_keys = store
        .get_client_keys(&form_id, &key_id)
        .await
        .map_err(internal_err)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(GetKeyResponse {
        wrapped_private_primary_key: client_keys.wrapped_private_primary_key,
    }))
}

#[axum::debug_handler]
async fn list_keys(
    State(state): State<Arc<AppState>>,
    Extension(token): Extension<SignedApiAccessToken>,
    Path(form_id): Path<FormId>,
) -> Result<Json<Vec<ListKeysResponse>>, ErrorResponse> {
    let store = token
        .validate(&state.store, &form_id, AccessRole::Admin)
        .await
        .map_err(auth_err)?;

    let client_keys = store
        .list_client_keys(&form_id)
        .await
        .map_err(internal_err)?;

    Ok(Json(client_keys.into_iter().map(From::from).collect()))
}

#[axum::debug_handler]
async fn add_key(
    State(state): State<Arc<AppState>>,
    Extension(token): Extension<SignedApiAccessToken>,
    Path(form_id): Path<FormId>,
    Json(body): Json<PostKeyRequest>,
) -> Result<(StatusCode, Json<PostKeyResponse>), ErrorResponse> {
    let store = token
        .validate(&state.store, &form_id, AccessRole::Admin)
        .await
        .map_err(auth_err)?;

    let client_key_id = store
        .store_client_keys(
            &form_id,
            &body.public_signing_key,
            Some(&body.wrapped_private_primary_key),
            &body.encrypted_comment,
            body.role,
        )
        .await
        .map_err(internal_err)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let response = PostKeyResponse { client_key_id };

    Ok((StatusCode::CREATED, Json(response)))
}

#[axum::debug_handler]
async fn update_key(
    State(state): State<Arc<AppState>>,
    Extension(token): Extension<SignedApiAccessToken>,
    Path((form_id, key_id)): Path<(FormId, ClientKeyId)>,
    Json(body): Json<PatchKeyRequest>,
) -> Result<NoContent, ErrorResponse> {
    let store = token
        .validate(&state.store, &form_id, AccessRole::Admin)
        .await
        .map_err(auth_err)?;

    store
        .update_client_keys(
            &form_id,
            &key_id,
            body.wrapped_private_primary_key.as_ref(),
            body.encrypted_comment.as_ref(),
        )
        .await
        .map_err(internal_err)?;

    Ok(NoContent)
}

#[axum::debug_handler]
async fn delete_key(
    State(state): State<Arc<AppState>>,
    Extension(token): Extension<SignedApiAccessToken>,
    Path((form_id, key_id)): Path<(FormId, ClientKeyId)>,
) -> Result<NoContent, ErrorResponse> {
    let store = token
        .validate(&state.store, &form_id, AccessRole::Admin)
        .await
        .map_err(auth_err)?;

    store
        .delete_client_keys(&form_id, &key_id)
        .await
        .map_err(internal_err)?;

    Ok(NoContent)
}
