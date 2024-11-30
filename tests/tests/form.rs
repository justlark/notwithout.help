mod common;

use reqwest::StatusCode;
use xpct::{be_ok, be_some, be_true, equal, expect};

use common::http;

#[tokio::test]
async fn create_form() -> anyhow::Result<()> {
    let req = http::create_form().await?;

    expect!(req.status()).to(equal(StatusCode::CREATED));

    let value = expect!(req.json::<serde_json::Value>().await)
        .to(be_ok())
        .into_inner();

    expect!(value.get("form_id"))
        .to(be_some())
        .map(|v| v.is_string())
        .to(be_true());

    expect!(value.get("client_key_id"))
        .to(be_some())
        .map(|v| v.is_number())
        .to(be_true());

    Ok(())
}
