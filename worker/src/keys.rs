use anyhow::Context;
use base64::prelude::*;
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
struct RandomId(String);

impl RandomId {
    pub fn new(len: usize) -> Self {
        Self(Alphanumeric.sample_string(&mut rand::thread_rng(), len))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FormId(RandomId);

impl FormId {
    const LEN: usize = 8;

    pub fn new() -> Self {
        Self(RandomId::new(Self::LEN))
    }
}

impl Default for FormId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SubmissionId(RandomId);

impl SubmissionId {
    const LEN: usize = 8;

    pub fn new() -> Self {
        Self(RandomId::new(Self::LEN))
    }
}

impl Default for SubmissionId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
struct PublicBoxKey(crypto_box::PublicKey);

impl AsRef<crypto_box::PublicKey> for PublicBoxKey {
    fn as_ref(&self) -> &crypto_box::PublicKey {
        &self.0
    }
}

impl Serialize for PublicBoxKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        BASE64_STANDARD
            .encode(&self.0.to_bytes())
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PublicBoxKey {
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
struct PrivateBoxKey(crypto_box::SecretKey);

impl AsRef<crypto_box::SecretKey> for PrivateBoxKey {
    fn as_ref(&self) -> &crypto_box::SecretKey {
        &self.0
    }
}

impl Serialize for PrivateBoxKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        BASE64_STANDARD
            .encode(&self.0.to_bytes())
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PrivateBoxKey {
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
pub struct PublicClientKey(PublicBoxKey);

impl AsRef<crypto_box::PublicKey> for PublicClientKey {
    fn as_ref(&self) -> &crypto_box::PublicKey {
        self.0.as_ref()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PublicWrappingKey(PublicBoxKey);

impl AsRef<crypto_box::PublicKey> for PublicWrappingKey {
    fn as_ref(&self) -> &crypto_box::PublicKey {
        self.0.as_ref()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PrivateServerKey(PrivateBoxKey);

impl AsRef<crypto_box::SecretKey> for PrivateServerKey {
    fn as_ref(&self) -> &crypto_box::SecretKey {
        self.0.as_ref()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PublicServerKey(PublicBoxKey);

impl AsRef<crypto_box::PublicKey> for PublicServerKey {
    fn as_ref(&self) -> &crypto_box::PublicKey {
        self.0.as_ref()
    }
}
