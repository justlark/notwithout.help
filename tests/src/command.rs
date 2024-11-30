use crate::cli::{Cli, Commands, GenSigningKey};

use base64::prelude::*;
use ed25519_dalek::{
    self as ed25519,
    pkcs8::{spki::der::pem::LineEnding, EncodePrivateKey},
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

impl Cli {
    pub fn dispatch(&self) -> anyhow::Result<()> {
        match &self.command {
            Commands::GenSigningKey(cmd) => cmd.run(),
        }
    }
}
