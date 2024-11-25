use anyhow::anyhow;
use base64::prelude::*;
use chrono::{DateTime, Utc};
use rand::distributions::{Alphanumeric, DistString};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use worker::console_log;

use crate::{
    secrets::{PasswordHash, Secret},
    WorkerEnv,
};

pub fn random_id(len: usize) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), len)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FormId(String);

impl FormId {
    const LEN: usize = 8;

    pub fn new() -> Self {
        Self(random_id(Self::LEN))
    }
}

impl Default for FormId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SubmissionId(String);

impl SubmissionId {
    const LEN: usize = 8;

    pub fn new() -> Self {
        Self(random_id(Self::LEN))
    }
}

impl Default for SubmissionId {
    fn default() -> Self {
        Self::new()
    }
}

// The submission body as a base64-encoded encrypted JSON object. Because it's encrypted
// client-side, the shape of the JSON object is opaque to this worker.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EncryptedSubmissionBody(String);

impl From<String> for EncryptedSubmissionBody {
    fn from(s: String) -> Self {
        Self(s)
    }
}

// The organizers' public encryption key used by clients to encrypt their submissions.
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PublicEncryptionKey(String);

impl PublicEncryptionKey {
    fn key(&self) -> anyhow::Result<crypto_box::PublicKey> {
        Ok(crypto_box::PublicKey::from_slice(
            &BASE64_STANDARD.decode(&self.0)?,
        )?)
    }
}

// The base64-encoded ciphertext of the API secret, encrypted with the public key for the form.
// Clients can decrypt this challenge with their corresponding private key to reveal the API secret
// and make authenticated API requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ApiChallenge(String);

// The API secret, a bearer token used to make authenticated API requests.
#[derive(Debug, Clone)]
pub struct ApiSecret(Secret);

impl ApiSecret {
    const LEN_BYTES: usize = 32;

    pub fn generate() -> Self {
        Self(Secret::generate(Self::LEN_BYTES))
    }

    pub fn from_base64(encoded: &str) -> anyhow::Result<Self> {
        Ok(Self(Secret::from(BASE64_STANDARD.decode(encoded)?)))
    }

    pub fn to_challenge(&self, public_key: &PublicEncryptionKey) -> anyhow::Result<ApiChallenge> {
        let key = public_key.key()?;
        let mut rng = rand::thread_rng();

        let ciphertext = key
            .seal(&mut rng, self.0.expose_secret())
            .map_err(|_| anyhow!("Failed generating the API challenge from the API secret."))?;

        let encoded = BASE64_STANDARD.encode(&ciphertext);

        Ok(ApiChallenge(encoded))
    }

    pub fn to_hashed(&self) -> anyhow::Result<HashedApiSecret> {
        PasswordHash::from_password(self.0.expose_secret()).map(HashedApiSecret)
    }

    // This is for debugging purposes only. When running the worker locally, this allows us to hit
    // the authenticated API endpoints manually without needing to complete the challenge to reveal
    // the API secret, which requires libsodium.
    pub fn dev_expose_secret_in_debug_log(&self, env: WorkerEnv) {
        if env != WorkerEnv::Dev {
            panic!("Attempted to expose API secret in a non-development environment.");
        }

        console_log!(
            "API secret: {}",
            BASE64_STANDARD.encode(self.0.expose_secret())
        );
    }
}

// The hashed API secret, which is stored in the database to validate authenticated API requests.
#[derive(Debug, PartialEq, Eq)]
pub struct HashedApiSecret(PasswordHash);

impl Serialize for HashedApiSecret {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        hex::encode(self.0.as_ref()).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HashedApiSecret {
    fn deserialize<D>(deserializer: D) -> Result<HashedApiSecret, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let decoded = hex::decode(&s).map_err(serde::de::Error::custom)?;
        let output = PasswordHash::new(&decoded).map_err(serde::de::Error::custom)?;

        Ok(HashedApiSecret(output))
    }
}

// The private key for the form, encrypted with a symmetric key that only the client knows.
#[derive(Debug)]
pub struct WrappedPrivateKey(Vec<u8>);

impl WrappedPrivateKey {
    pub fn from_base64(encoded: &str) -> anyhow::Result<Self> {
        Ok(Self(BASE64_STANDARD.decode(encoded)?))
    }
}

impl Serialize for WrappedPrivateKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        BASE64_STANDARD.encode(&self.0).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for WrappedPrivateKey {
    fn deserialize<D>(deserializer: D) -> Result<WrappedPrivateKey, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let bytes = BASE64_STANDARD
            .decode(s)
            .map_err(serde::de::Error::custom)?;

        Ok(WrappedPrivateKey(bytes))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct KeyIndex(u32);

#[derive(Debug, Serialize, Deserialize)]
pub struct FormTemplate {
    pub hashed_api_secret: HashedApiSecret,
    pub api_challenge: ApiChallenge,
    pub public_key: PublicEncryptionKey,
    pub org_name: String,
    pub description: String,
    pub contact_methods: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormRequest {
    pub public_key: PublicEncryptionKey,
    pub org_name: String,
    pub description: String,
    pub contact_methods: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct FormResponse {
    pub api_challenge: ApiChallenge,
    pub public_key: PublicEncryptionKey,
    pub org_name: String,
    pub description: String,
    pub contact_methods: Vec<String>,
}

impl From<FormTemplate> for FormResponse {
    fn from(template: FormTemplate) -> Self {
        Self {
            api_challenge: template.api_challenge,
            public_key: template.public_key,
            org_name: template.org_name,
            description: template.description,
            contact_methods: template.contact_methods,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PublishFormResponse {
    pub form_id: FormId,
    pub api_challenge: ApiChallenge,
}

#[derive(Debug)]
pub struct Submission {
    pub encrypted_body: EncryptedSubmissionBody,
    pub created_at: DateTime<Utc>,
}

impl Serialize for Submission {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct Inner {
            encrypted_body: EncryptedSubmissionBody,
            created_at: String,
        }

        let inner = Inner {
            encrypted_body: self.encrypted_body.clone(),
            created_at: self.created_at.to_rfc3339(),
        };

        inner.serialize(serializer)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EncryptedKeyComment(String);

#[derive(Debug, Serialize)]
pub struct GetKeyResponse {
    pub key: WrappedPrivateKey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyMetadata {
    pub key_index: KeyIndex,
    pub comment: EncryptedKeyComment,
}

#[derive(Debug, Deserialize)]
pub struct PostKeyRequest {
    pub key: WrappedPrivateKey,
    pub comment: EncryptedKeyComment,
}

#[derive(Debug, Serialize)]
pub struct PostKeyResponse {
    pub key_index: KeyIndex,
}
