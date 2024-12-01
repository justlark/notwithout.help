mod common;

use reqwest::StatusCode;
use serde_json::Value as JsonValue;
use xpct::{be_ok, be_some, be_true, equal, expect};

use common::http::{self, FormResponse};

#[tokio::test]
async fn get_form_template() -> anyhow::Result<()> {
    let FormResponse { form_id, .. } = http::create_form().await?;

    let resp = http::client()
        .get(http::path(&format!("/forms/{}", form_id)))
        .send()
        .await?;

    let value = expect!(resp.json::<JsonValue>().await)
        .to(be_ok())
        .into_inner();

    expect!(value.get("org_name"))
        .to(be_some())
        .map(|v| v.is_string())
        .to(be_true());

    expect!(value.get("description"))
        .to(be_some())
        .map(|v| v.is_string())
        .to(be_true());

    expect!(value.get("contact_methods"))
        .to(be_some())
        .map(|v| v.is_array())
        .to(be_true());

    Ok(())
}

#[tokio::test]
async fn get_form_template_form_not_found() -> anyhow::Result<()> {
    let resp = http::client()
        .get(http::path("/forms/invalid-form-id"))
        .send()
        .await?;

    expect!(resp.status()).to(equal(StatusCode::NOT_FOUND));

    Ok(())
}

#[tokio::test]
async fn delete_form() -> anyhow::Result<()> {
    let FormResponse {
        form_id,
        client_key_id,
        signing_key,
    } = http::create_form().await?;

    let auth_token = http::authenticate(&form_id, client_key_id, &signing_key).await?;

    let resp = http::client()
        .delete(http::path(&format!("/forms/{}", form_id)))
        .bearer_auth(auth_token)
        .send()
        .await?;

    expect!(resp.status()).to(equal(StatusCode::NO_CONTENT));

    let resp = http::client()
        .get(http::path(&format!("/forms/{}", form_id)))
        .send()
        .await?;

    expect!(resp.status()).to(equal(StatusCode::NOT_FOUND));

    Ok(())
}
