use std::fmt;

use anyhow::Context;
use base64::prelude::*;
use crypto_box::{
    aead::{Aead, AeadCore},
    Nonce, SalsaBox,
};
use serde::{Deserialize, Serialize};
use typenum::Unsigned;

//
// See the security architecture document for information on the purpose of these keys and how
// they're used. The names of identifiers in this file generally match the terms defined in that
// document.
//
// https://github.com/justlark/notwithout.help/blob/main/docs/security-whitepaper.md
//

#[derive(Debug, Clone)]
pub struct PublicKey(crypto_box::PublicKey);

impl Serialize for PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        BASE64_STANDARD
            .encode(self.0.to_bytes())
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let decoded = &BASE64_STANDARD
            .decode(s)
            .context("Public key is not a valid base64-encoded string.")
            .map_err(serde::de::Error::custom)?;
        let key = crypto_box::PublicKey::from_slice(decoded)
            .context("This is not a valid libsodium box public key.")
            .map_err(serde::de::Error::custom)?;

        Ok(Self(key))
    }
}

#[derive(Debug, Clone)]
pub struct PrivateKey(crypto_box::SecretKey);

impl Serialize for PrivateKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        BASE64_STANDARD
            .encode(self.0.to_bytes())
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PrivateKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let decoded = &BASE64_STANDARD
            .decode(s)
            .context("Private key is not a valid base64-encoded string.")
            .map_err(serde::de::Error::custom)?;
        let key = crypto_box::SecretKey::from_slice(decoded)
            .context("This is not a valid libsodium box private key.")
            .map_err(serde::de::Error::custom)?;

        Ok(Self(key))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PublicClientKey(PublicKey);

impl AsRef<PublicKey> for PublicClientKey {
    fn as_ref(&self) -> &PublicKey {
        &self.0
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PublicWrappingKey(PublicKey);

impl AsRef<PublicKey> for PublicWrappingKey {
    fn as_ref(&self) -> &PublicKey {
        &self.0
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PrivateServerKey(PrivateKey);

impl AsRef<PrivateKey> for PrivateServerKey {
    fn as_ref(&self) -> &PrivateKey {
        &self.0
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PublicServerKey(PublicKey);

impl AsRef<PublicKey> for PublicServerKey {
    fn as_ref(&self) -> &PublicKey {
        &self.0
    }
}

// This abstraction handles prepending the nonce to the ciphertext.
pub struct CryptBox(SalsaBox);

impl fmt::Debug for CryptBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CryptBox").finish_non_exhaustive()
    }
}

impl CryptBox {
    pub fn new(public_key: &PublicKey, private_key: &PrivateKey) -> Self {
        Self(crypto_box::SalsaBox::new(&public_key.0, &private_key.0))
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> anyhow::Result<Vec<u8>> {
        let nonce = SalsaBox::generate_nonce(rand::thread_rng());
        let ciphertext = self.0.encrypt(&nonce, plaintext)?;

        let mut output = nonce.to_vec();
        output.extend_from_slice(&ciphertext);

        Ok(output)
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> anyhow::Result<Vec<u8>> {
        let nonce_len = <SalsaBox as AeadCore>::NonceSize::to_usize();
        let nonce = Nonce::from_slice(&ciphertext[..nonce_len]);
        Ok(self.0.decrypt(nonce, &ciphertext[nonce_len..])?)
    }
}
