use std::io::{self, Read};
use std::str;

use crate::cli::{Cli, Commands, GenSigningKey, RespondChallenge};

use anyhow::bail;
use base64::prelude::*;
use ed25519_dalek::{
    self as ed25519,
    pkcs8::{spki::der::pem::LineEnding, DecodePrivateKey, EncodePrivateKey},
    Signer,
};
use serde::{Deserialize, Serialize};

impl GenSigningKey {
    pub fn run(&self) -> anyhow::Result<()> {
        let key = ed25519::SigningKey::generate(&mut rand::thread_rng());
        key.write_pkcs8_pem_file(&self.output, LineEnding::LF)?;

        let public_key_bytes = key.as_ref().as_bytes();
        let encoded = BASE64_STANDARD.encode(public_key_bytes);
        println!("Public key: {}", encoded);

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct ApiChallenge {
    pub nonce: String,
}

#[derive(Debug, Serialize)]
pub struct ApiChallengeResponse {
    pub signature: String,
    pub challenge: String,
}

impl RespondChallenge {
    pub fn run(&self) -> anyhow::Result<()> {
        let key = ed25519::SigningKey::read_pkcs8_pem_file(&self.key)?;

        let encoded_challenge = if self.token == "-" {
            let mut stdin = String::new();
            io::stdin().read_to_string(&mut stdin)?;
            stdin
        } else {
            self.token.clone()
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

        let nonce_signature = BASE64_STANDARD.encode(key.sign(&nonce).to_bytes());

        let response = ApiChallengeResponse {
            signature: nonce_signature,
            challenge: encoded_challenge,
        };

        println!("{}", serde_json::to_string_pretty(&response)?);

        Ok(())
    }
}

impl Cli {
    pub fn dispatch(&self) -> anyhow::Result<()> {
        match &self.command {
            Commands::GenSigningKey(cmd) => cmd.run(),
            Commands::RespondChallenge(cmd) => cmd.run(),
        }
    }
}
