use reqwest::StatusCode;
use serde_json::Value as JsonValue;
use xpct::{be_ok, be_true, equal, expect};

use common::http::{self, FormResponse};

mod common;

#[tokio::test]
async fn post_encrypted_submission() -> anyhow::Result<()> {
    let FormResponse { form_id, .. } = http::create_form().await?;

    let resp = http::client()
        .post(http::path(&format!("/submissions/{}", form_id)))
        .send()
        .await?;

    expect!(resp.status()).to(equal(StatusCode::CREATED));

    Ok(())
}

#[tokio::test]
async fn post_encrypted_submission_form_not_found() -> anyhow::Result<()> {
    let resp = http::client()
        .post(http::path("/submissions/invalid-form-id"))
        .send()
        .await?;

    expect!(resp.status()).to(equal(StatusCode::NOT_FOUND));

    Ok(())
}

#[tokio::test]
async fn get_encrypted_submission() -> anyhow::Result<()> {
    let FormResponse {
        form_id,
        client_key_id,
        signing_key,
    } = http::create_form().await?;

    let auth_token = http::authenticate(&form_id, client_key_id, &signing_key).await?;

    let resp = http::client()
        .get(http::path(&format!("/submissions/{}", form_id)))
        .bearer_auth(auth_token)
        .send()
        .await?;

    expect!(resp.status()).to(equal(StatusCode::OK));

    expect!(resp.json::<JsonValue>().await)
        .to(be_ok())
        .map(|v| v.is_array())
        .to(be_true())
        .into_inner();

    Ok(())
}

#[tokio::test]
async fn get_encrypted_submission_form_not_found() -> anyhow::Result<()> {
    let FormResponse {
        form_id,
        client_key_id,
        signing_key,
    } = http::create_form().await?;

    let auth_token = http::authenticate(&form_id, client_key_id, &signing_key).await?;

    http::client()
        .delete(http::path(&format!("/forms/{}", form_id)))
        .bearer_auth(&auth_token)
        .send()
        .await?;

    let resp = http::client()
        .get(http::path(&format!("/submissions/{}", form_id)))
        .bearer_auth(&auth_token)
        .send()
        .await?;

    expect!(resp.status()).to(equal(StatusCode::UNAUTHORIZED));

    Ok(())
}
