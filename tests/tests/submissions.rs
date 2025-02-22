use reqwest::StatusCode;
use serde_json::{json, Value as JsonValue};
use xpct::{all, be_ok, equal, expect, have_len, match_elements};

use common::{
    endpoints,
    http::{self, FormResponse},
    matchers::{have_field, have_type, JsonArray, JsonString},
};

mod common;

#[tokio::test]
async fn post_encrypted_submission() -> anyhow::Result<()> {
    let FormResponse { form_id, .. } = http::create_form().await?;

    let resp = endpoints::post_submission(&form_id)
        .json(&json!({
            "encrypted_body": "<encrypted_body>",
        }))
        .send()
        .await?;

    expect!(resp.status()).to(equal(StatusCode::CREATED));

    Ok(())
}

#[tokio::test]
async fn post_encrypted_submission_form_not_found() -> anyhow::Result<()> {
    let resp = endpoints::post_submission("invalid-form-id")
        .json(&json!({
            "encrypted_body": "<encrypted_body>",
        }))
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

    let auth_token = http::authenticate(&form_id, &client_key_id, &signing_key).await?;
    let encrypted_body = "<encrypted_body>";

    let resp = endpoints::post_submission(&form_id)
        .json(&json!({
            "encrypted_body": encrypted_body,
        }))
        .send()
        .await?;

    expect!(resp.status()).to(equal(StatusCode::CREATED));

    let resp = endpoints::get_submissions(&form_id)
        .bearer_auth(auth_token)
        .send()
        .await?;

    expect!(resp.status()).to(equal(StatusCode::OK));

    let body = expect!(resp.json::<JsonValue>().await)
        .to(be_ok())
        .into_inner();

    expect!(body)
        .to(have_type::<JsonArray>())
        .to(have_len(1))
        .to(match_elements([all(|ctx| {
            ctx.to(have_field::<JsonString>("encrypted_body"))?
                .to(equal(encrypted_body))
        })]));

    Ok(())
}

#[tokio::test]
async fn get_encrypted_submission_form_not_found() -> anyhow::Result<()> {
    let FormResponse {
        form_id,
        client_key_id,
        signing_key,
    } = http::create_form().await?;

    let resp = endpoints::post_submission(&form_id)
        .json(&json!({
            "encrypted_body": "<encrypted_body>",
        }))
        .send()
        .await?;

    expect!(resp.status()).to(equal(StatusCode::CREATED));

    let auth_token = http::authenticate(&form_id, &client_key_id, &signing_key).await?;

    endpoints::delete_form(&form_id)
        .bearer_auth(&auth_token)
        .send()
        .await?;

    let resp = endpoints::get_submissions(&form_id)
        .bearer_auth(&auth_token)
        .send()
        .await?;

    expect!(resp.status()).to(equal(StatusCode::UNAUTHORIZED));

    Ok(())
}

#[tokio::test]
async fn get_encrypted_submission_when_there_are_none() -> anyhow::Result<()> {
    let FormResponse {
        form_id,
        client_key_id,
        signing_key,
    } = http::create_form().await?;

    let auth_token = http::authenticate(&form_id, &client_key_id, &signing_key).await?;

    let resp = endpoints::get_submissions(&form_id)
        .bearer_auth(&auth_token)
        .send()
        .await?;

    expect!(resp.status()).to(equal(StatusCode::OK));

    expect!(resp.json::<JsonValue>().await)
        .to(be_ok())
        .to(have_type::<JsonArray>())
        .to(have_len(0));

    Ok(())
}
