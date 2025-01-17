use std::str::FromStr;

use anyhow::Context;
use base64::prelude::*;
use ed25519_dalek::{self as ed25519, Verifier};
use jsonwebtoken as jwt;
use rand::RngCore;
use secrecy::{ExposeSecret, SecretSlice, SecretString};
use serde::{Deserialize, Serialize};

//
// See the security architecture document for information on the purpose of these values and how
// they're used. The names of identifiers in this file generally match the terms defined in that
// document.
//
// https://github.com/justlark/notwithout.help/blob/main/docs/security-whitepaper.md
//

#[derive(Debug, Clone)]
pub struct EphemeralServerKey {
    bytes: SecretSlice<u8>,
    encoded: SecretString,
}

impl EphemeralServerKey {
    pub const LEN: usize = 32;

    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();
        let mut buf = vec![0u8; Self::LEN];
        rng.fill_bytes(&mut buf);

        let encoded = BASE64_STANDARD.encode(&buf);

        Self {
            bytes: SecretSlice::from(buf),
            encoded: SecretString::from(encoded),
        }
    }

    pub fn encoding_key(&self) -> jwt::EncodingKey {
        jwt::EncodingKey::from_secret(self.bytes.expose_secret())
    }

    pub fn decoding_key(&self) -> jwt::DecodingKey {
        jwt::DecodingKey::from_secret(self.bytes.expose_secret())
    }
}

impl ExposeSecret<str> for EphemeralServerKey {
    fn expose_secret(&self) -> &str {
        self.encoded.expose_secret()
    }
}

impl FromStr for EphemeralServerKey {
    type Err = base64::DecodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let decoded = BASE64_STANDARD.decode(s)?;
        Ok(Self {
            bytes: SecretSlice::from(decoded),
            encoded: SecretString::from(s),
        })
    }
}

impl<'de> Deserialize<'de> for ApiChallengeNonce {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let decoded = BASE64_STANDARD
            .decode(s)
            .context("API challenge nonce is not a valid base64-encoded string.")
            .map_err(serde::de::Error::custom)?;
        Ok(Self(decoded))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiChallengeNonce(Vec<u8>);

impl ApiChallengeNonce {
    pub const LEN: usize = 16;

    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();
        let mut buf = vec![0u8; Self::LEN];
        rng.fill_bytes(&mut buf);
        Self(buf)
    }
}

impl Serialize for ApiChallengeNonce {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        BASE64_STANDARD.encode(&self.0).serialize(serializer)
    }
}

#[derive(Debug, Clone)]
pub struct ClientNonceSignature(Vec<u8>);

impl Serialize for ClientNonceSignature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        BASE64_STANDARD.encode(&self.0).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ClientNonceSignature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let decoded = BASE64_STANDARD
            .decode(s)
            .context("Client nonce signature is not a valid base64-encoded string.")
            .map_err(serde::de::Error::custom)?;
        Ok(Self(decoded))
    }
}

#[derive(Debug, Clone)]
pub struct PublicSigningKey(ed25519::VerifyingKey);

impl PublicSigningKey {
    pub fn verify(
        &self,
        nonce: &ApiChallengeNonce,
        signature: &ClientNonceSignature,
    ) -> Result<(), ed25519::SignatureError> {
        self.0
            .verify(&nonce.0, &ed25519::Signature::from_slice(&signature.0)?)?;

        Ok(())
    }
}

impl Serialize for PublicSigningKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        BASE64_STANDARD.encode(self.0).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PublicSigningKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let decoded = BASE64_STANDARD
            .decode(s)
            .context("Public signing key is not a valid base64-encoded string.")
            .map_err(serde::de::Error::custom)?;

        if decoded.len() != ed25519::PUBLIC_KEY_LENGTH {
            return Err(serde::de::Error::custom(format!(
                "Public signing key is not {} bytes long.",
                ed25519::PUBLIC_KEY_LENGTH,
            )));
        }

        let mut bytes = [0u8; ed25519::PUBLIC_KEY_LENGTH];
        bytes.copy_from_slice(&decoded);

        Ok(Self(
            ed25519::VerifyingKey::from_bytes(&bytes).map_err(serde::de::Error::custom)?,
        ))
    }
}

// This is opaque to the server, so no need to decode it.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WrappedPrivatePrimaryKey(String);

// This is opaque to the server, so no need to decode it.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PublicPrimaryKey(String);
