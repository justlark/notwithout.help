use ed25519_dalek as ed25519;
use notwithouttests::respond_challenge;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;
use xpct::{be_ok, be_some, equal, expect};

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

#[derive(Debug, Deserialize)]
pub struct FormResponse {
    pub form_id: String,
    pub client_key_id: u64,
}

pub async fn create_form() -> anyhow::Result<FormResponse> {
    let req = client()
        .post(path("/forms"))
        .json(&json!({
            "public_primary_key": "<public_primary_key>",
            "public_signing_key": "Vp0SD6ySAex2vXtsaA8SbXKS3gS35yWO56MTWk2aJzw=",
            "org_name": "<org_name>",
            "description": "<description>",
            "contact_methods": ["<contact_method>"]
        }))
        .send()
        .await?;

    expect!(req.status()).to(equal(StatusCode::CREATED));

    let value = expect!(req.json::<serde_json::Value>().await)
        .to(be_ok())
        .into_inner();

    let form_id = expect!(value.get("form_id"))
        .to(be_some())
        .map(|v| v.as_str())
        .to(be_some())
        .map(|v| v.to_string())
        .into_inner();

    let client_key_id = expect!(value.get("client_key_id"))
        .to(be_some())
        .map(|v| v.as_u64())
        .to(be_some())
        .into_inner();

    Ok(FormResponse {
        form_id,
        client_key_id,
    })
}

pub async fn challenge_response() -> anyhow::Result<String> {
    let FormResponse {
        form_id,
        client_key_id,
    } = create_form().await?;

    let req = client()
        .get(path(&format!("/challenges/{}/{}", form_id, client_key_id)))
        .send()
        .await?;

    expect!(req.status()).to(equal(StatusCode::OK));

    let challenge_token = expect!(req.text().await)
        .to(be_ok())
        .map(|v| v.to_string())
        .into_inner();

    let signing_key = ed25519::SigningKey::generate(&mut rand::thread_rng());

    let challenge_response = respond_challenge(challenge_token, signing_key)?;

    Ok(challenge_response)
}
