#![deny(unsafe_code)]
#![warn(missing_debug_implementations)]
#![allow(dead_code)] // TODO: Remove
#![allow(unused_variables)] // TODO: Remove

mod api;
mod auth;
mod cors;
mod keys;
mod models;
mod router;
mod store;

use axum::{body::Body, http::Response};
use router::{AppState, WorkerEnv};
use store::UnauthenticatedStore;
use tower_service::Service;
use worker::{self, event, Context, Env, HttpRequest};

const D1_BINDING: &str = "DB";

#[event(fetch)]
async fn fetch(req: HttpRequest, env: Env, _ctx: Context) -> worker::Result<Response<Body>> {
    console_error_panic_hook::set_once();

    let state = AppState {
        store: UnauthenticatedStore::new(env.d1(D1_BINDING)?),
        env: WorkerEnv::get(&env),
    };

    Ok(router::new(state).call(req).await?)
}
