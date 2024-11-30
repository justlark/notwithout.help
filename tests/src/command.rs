use crate::challenge::respond_challenge;
use crate::cli::{Cli, Commands, GenSigningKey, RespondChallenge};

use base64::prelude::*;
use ed25519_dalek::{
    self as ed25519,
    pkcs8::{spki::der::pem::LineEnding, DecodePrivateKey, EncodePrivateKey},
};

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

impl RespondChallenge {
    pub fn run(&self) -> anyhow::Result<()> {
        let key = ed25519::SigningKey::read_pkcs8_pem_file(&self.key)?;
        let challenge_response = respond_challenge(self.token.clone(), key)?;
        println!("{}", challenge_response);
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
