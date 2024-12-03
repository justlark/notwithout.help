use common::{
    encoding::base64_encode,
    endpoints,
    http::{self, gen_challenge_response, FormResponse},
    matchers::{have_field, JsonString},
};
use ed25519_dalek::Signer;
use reqwest::StatusCode;
use serde_json::{json, Value as JsonValue};
use xpct::{be_ok, equal, expect};

mod common;

#[tokio::test]
async fn valid_signature_of_wrong_nonce_is_unauthorized() -> anyhow::Result<()> {
    let FormResponse {
        form_id,
        client_key_id,
        signing_key,
    } = http::create_form().await?;

    let resp = endpoints::get_challenge(&form_id, &client_key_id)
        .send()
        .await?;

    expect!(resp.status()).to(equal(StatusCode::OK));

    let challenge = expect!(resp.json::<JsonValue>().await)
        .to(be_ok())
        .to(have_field::<JsonString>("challenge"))
        .into_inner();

    let signature = signing_key.sign("invalid-nonce".as_bytes());

    let resp = http::client()
        .post(http::path("/tokens"))
        .json(&json!({
            "signature": base64_encode(signature.to_bytes()),
            "challenge": challenge,
        }))
        .send()
        .await?;

    expect!(resp.status()).to(equal(StatusCode::UNAUTHORIZED));

    Ok(())
}

#[tokio::test]
async fn not_returning_same_challenge_is_unauthorized() -> anyhow::Result<()> {
    let FormResponse {
        form_id,
        client_key_id,
        signing_key,
    } = http::create_form().await?;

    let challenge_response = gen_challenge_response(&form_id, &client_key_id, &signing_key).await?;

    let resp = http::client()
        .post(http::path("/tokens"))
        .json(&json!({
            "signature": challenge_response.signature,
            "challenge": "invalid-challenge",
        }))
        .send()
        .await?;

    expect!(resp.status()).to(equal(StatusCode::UNAUTHORIZED));

    Ok(())
}

#[tokio::test]
async fn challenge_token_can_not_be_used_as_access_token() -> anyhow::Result<()> {
    let FormResponse {
        form_id,
        client_key_id,
        ..
    } = http::create_form().await?;

    let resp = endpoints::get_challenge(&form_id, &client_key_id)
        .send()
        .await?;

    let challenge_token = expect!(resp.json::<JsonValue>().await)
        .to(be_ok())
        .to(have_field::<JsonString>("challenge"))
        .into_inner();

    let resp = endpoints::delete_form(&form_id)
        .bearer_auth(&challenge_token)
        .send()
        .await?;

    expect!(resp.status()).to(equal(StatusCode::UNAUTHORIZED));

    Ok(())
}
