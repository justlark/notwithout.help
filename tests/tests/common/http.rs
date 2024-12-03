use base64::prelude::*;
use ed25519_dalek as ed25519;
use notwithouttests::{respond_challenge, ApiChallengeResponse};
use reqwest::StatusCode;
use serde_json::{json, Value as JsonValue};
use xpct::{be_ok, equal, expect};

use super::{
    endpoints,
    matchers::{have_field, JsonString},
};

const DEFAULT_API_URL: &str = "http://localhost:8787";

pub fn api_url() -> String {
    dotenv::var("API_URL").unwrap_or_else(|_| DEFAULT_API_URL.to_string())
}

pub fn client() -> reqwest::Client {
    reqwest::Client::new()
}
pub fn path(path: &str) -> String {
    format!("{}{}", api_url(), path)
}

#[derive(Debug)]
pub struct FormResponse {
    pub form_id: String,
    pub client_key_id: String,
    pub signing_key: ed25519::SigningKey,
}

pub async fn create_form() -> anyhow::Result<FormResponse> {
    let signing_key = ed25519::SigningKey::generate(&mut rand::thread_rng());
    let public_signing_key = BASE64_STANDARD.encode(signing_key.as_ref().to_bytes());

    let resp = endpoints::post_form()
        .json(&json!({
            "public_primary_key": "<public_primary_key>",
            "public_signing_key": public_signing_key,
            "org_name": "<org_name>",
            "description": "<description>",
            "contact_methods": ["<contact_method>"]
        }))
        .send()
        .await?;

    expect!(resp.status()).to(equal(StatusCode::CREATED));

    let body = expect!(resp.json::<JsonValue>().await)
        .to(be_ok())
        .into_inner();

    let form_id = expect!(body.clone())
        .to(have_field::<JsonString>("form_id"))
        .into_inner();

    let client_key_id = expect!(body)
        .to(have_field::<JsonString>("client_key_id"))
        .into_inner();

    Ok(FormResponse {
        form_id,
        client_key_id,
        signing_key,
    })
}

pub async fn gen_challenge_response(
    form_id: &str,
    client_key_id: &str,
    signing_key: &ed25519::SigningKey,
) -> anyhow::Result<ApiChallengeResponse> {
    let resp = endpoints::get_challenge(form_id, client_key_id)
        .send()
        .await?;

    expect!(resp.status()).to(equal(StatusCode::OK));

    let body = expect!(resp.json::<JsonValue>().await)
        .to(be_ok())
        .into_inner();

    let challenge = expect!(body)
        .to(have_field::<JsonString>("challenge"))
        .into_inner();

    respond_challenge(&challenge, signing_key)
}

pub async fn authenticate(
    form_id: &str,
    client_key_id: &str,
    signing_key: &ed25519::SigningKey,
) -> anyhow::Result<String> {
    let challenge_response = gen_challenge_response(form_id, client_key_id, signing_key).await?;

    let resp = endpoints::post_token()
        .json(&challenge_response)
        .send()
        .await?;

    expect!(resp.status()).to(equal(StatusCode::OK));

    let body = expect!(resp.json::<JsonValue>().await)
        .to(be_ok())
        .into_inner();

    let token = expect!(body)
        .to(have_field::<JsonString>("token"))
        .into_inner();

    Ok(token)
}
