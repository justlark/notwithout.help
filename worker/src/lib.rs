#![deny(unsafe_code)]

mod api;
mod auth;
mod config;
mod cors;
mod keys;
mod models;
mod router;
mod store;

use axum::{body::Body, http::Response};
use router::AppState;
use store::UnauthenticatedStore;
use tower_service::Service;
use worker::{self, event, Context, Env, HttpRequest, ScheduleContext, ScheduledEvent};

const D1_BINDING: &str = "DB";
const KV_BINDING: &str = "KV";

#[event(fetch)]
async fn fetch(req: HttpRequest, env: Env, _ctx: Context) -> worker::Result<Response<Body>> {
    console_error_panic_hook::set_once();

    config::init(&env).expect("failed to initialize config");

    let state = AppState {
        store: UnauthenticatedStore::new(env.d1(D1_BINDING)?, env.kv(KV_BINDING)?),
    };

    Ok(router::new(state).call(req).await?)
}

#[event(scheduled)]
async fn scheduled(_event: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    console_error_panic_hook::set_once();

    config::init(&env).expect("failed to initialize config");

    let d1 = env.d1(D1_BINDING).expect("failed to get D1 binding");
    let kv = env.kv(KV_BINDING).expect("failed to get KV binding");

    let store = UnauthenticatedStore::new(d1, kv);

    store
        .without_authenticating()
        .delete_expired_forms()
        .await
        .expect("failed to delete expired forms");
}
