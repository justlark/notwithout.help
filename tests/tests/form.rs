mod common;

use reqwest::StatusCode;
use xpct::{be_err, be_ok, be_some, be_true, equal, expect};

use common::http::{self, FormResponse};

#[tokio::test]
async fn get_form_template() -> anyhow::Result<()> {
    let FormResponse { form_id, .. } = http::create_form().await?;

    let req = http::client()
        .get(http::path(&format!("/forms/{}", form_id)))
        .send()
        .await?;

    let value = expect!(req.json::<serde_json::Value>().await)
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
async fn get_form_template_not_found() -> anyhow::Result<()> {
    let req = http::client()
        .get(http::path("/forms/invalid-form-id"))
        .send()
        .await?;

    expect!(req.status()).to(equal(StatusCode::NOT_FOUND));

    Ok(())
}
