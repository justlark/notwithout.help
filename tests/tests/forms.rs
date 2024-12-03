mod common;

use reqwest::StatusCode;
use serde_json::Value as JsonValue;
use xpct::{be_ok, equal, expect};

use common::{
    endpoints,
    http::{self, FormResponse},
    matchers::{have_field, JsonArray, JsonString},
};

#[tokio::test]
async fn get_form_template() -> anyhow::Result<()> {
    let FormResponse { form_id, .. } = http::create_form().await?;

    let resp = endpoints::get_form(&form_id).send().await?;

    let body = expect!(resp.json::<JsonValue>().await)
        .to(be_ok())
        .into_inner();

    expect!(body.clone()).to(have_field::<JsonString>("org_name"));
    expect!(body.clone()).to(have_field::<JsonString>("description"));
    expect!(body).to(have_field::<JsonArray<JsonString>>("contact_methods"));

    Ok(())
}

#[tokio::test]
async fn get_form_template_form_not_found() -> anyhow::Result<()> {
    let resp = endpoints::get_form("invalid-form-id").send().await?;

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

    let auth_token = http::authenticate(&form_id, &client_key_id, &signing_key).await?;

    let resp = endpoints::delete_form(&form_id)
        .bearer_auth(auth_token)
        .send()
        .await?;

    expect!(resp.status()).to(equal(StatusCode::NO_CONTENT));

    let resp = endpoints::get_form(&form_id).send().await?;

    expect!(resp.status()).to(equal(StatusCode::NOT_FOUND));

    Ok(())
}
