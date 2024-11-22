mod models;

use axum::{
    body::Body,
    extract::{Json, Path},
    http::{
        header::{HeaderName, CONTENT_TYPE},
        HeaderValue, Method, Request, Response, StatusCode,
    },
    routing::{delete, get, post},
    Router,
};
use tower_http::{auth::AsyncRequireAuthorizationLayer, cors::CorsLayer, trace::TraceLayer};
use tower_service::Service;
use worker::{self, event, kv::KvStore, Context, Env, HttpRequest};

use models::{EncryptedFormSubmission, FormId, FormResponse, FormTemplate, PublishFormResponse};

const CORS_ALLOWED_ORIGINS: [&str; 1] = ["https://example.com"];
const CORS_ALLOWED_METHODS: [Method; 3] = [Method::GET, Method::POST, Method::DELETE];
const CORS_ALLOWED_HEADERS: [HeaderName; 1] = [CONTENT_TYPE];

const KV_BINDING: &str = "KV";

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

async fn auth_layer(req: Request<Body>) -> Result<Request<Body>, Response<Body>> {
    todo!()
}

fn router(kv: KvStore) -> Router {
    Router::new()
        .layer(TraceLayer::new_for_http())
        .layer(cors_layer())
        .with_state(kv)
        // Authenticated endpoints.
        .route("/forms/:form_id", delete(delete_form))
        .route("/submissions/:form_id", get(list_form_submissions))
        .route("/submissions/:form_id", delete(delete_form_submission))
        .route_layer(AsyncRequireAuthorizationLayer::new(auth_layer))
        // Unauthenticated endpoints.
        .route("/forms", post(publish_form))
        .route("/forms/:form_id", get(get_form))
        .route("/submissions/:form_id", post(store_form_submission))
}

#[event(fetch)]
async fn fetch(req: HttpRequest, env: Env, _ctx: Context) -> worker::Result<Response<Body>> {
    console_error_panic_hook::set_once();
    let kv = env.kv(KV_BINDING)?;
    Ok(router(kv).call(req).await?)
}

pub async fn publish_form(Json(template): Json<FormTemplate>) -> Json<PublishFormResponse> {
    todo!()
}

pub async fn get_form(Path(form_id): Path<FormId>) -> Json<FormResponse> {
    todo!()
}

pub async fn delete_form(Path(form_id): Path<FormId>) -> StatusCode {
    todo!()
}

pub async fn store_form_submission(
    Path(form_id): Path<FormId>,
    body: EncryptedFormSubmission,
) -> StatusCode {
    todo!()
}

pub async fn list_form_submissions(
    Path(form_id): Path<FormId>,
) -> Json<Vec<EncryptedFormSubmission>> {
    todo!()
}

pub async fn delete_form_submission(Path(form_id): Path<FormId>) -> StatusCode {
    todo!()
}
