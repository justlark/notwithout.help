use std::io::{self, Read};

use anyhow::bail;
use base64::prelude::*;
use ed25519_dalek::{self as ed25519, Signer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct ApiChallenge {
    pub nonce: String,
}

#[derive(Debug, Serialize)]
pub struct ApiChallengeResponse {
    pub signature: String,
    pub challenge: String,
}

pub fn respond_challenge(
    token: &str,
    signing_key: &ed25519::SigningKey,
) -> anyhow::Result<ApiChallengeResponse> {
    let encoded_challenge = if token == "-" {
        let mut stdin = String::new();
        io::stdin().read_to_string(&mut stdin)?;
        stdin
    } else {
        token.to_string()
    };

    let nonce = match encoded_challenge
        .splitn(3, '.')
        .collect::<Vec<_>>()
        .as_slice()
    {
        [_, payload, _] => {
            let decoded = BASE64_STANDARD.decode(payload)?;
            let encoded_nonce = serde_json::from_slice::<ApiChallenge>(&decoded)?.nonce;
            BASE64_STANDARD.decode(encoded_nonce)?
        }
        _ => {
            bail!("Challenge token is not in the expected format.");
        }
    };

    let nonce_signature = BASE64_STANDARD.encode(signing_key.sign(&nonce).to_bytes());

    Ok(ApiChallengeResponse {
        signature: nonce_signature,
        challenge: encoded_challenge,
    })
}
