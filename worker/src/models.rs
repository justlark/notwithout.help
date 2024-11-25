use std::fmt;

use anyhow::anyhow;
use base64::prelude::*;
use chrono::{DateTime, Utc};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use sha2::{digest::Digest, Sha256};
use worker::wasm_bindgen::JsValue;

use crate::secrets::Secret;

macro_rules! string_newtype {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $name(String);

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                Self(s)
            }
        }

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                Self(s.to_string())
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<$name> for JsValue {
            fn from(s: $name) -> Self {
                s.0.into()
            }
        }
    };
}

string_newtype!(SubmissionId);
string_newtype!(FormId);

// The submission body as a base64-encoded encrypted JSON object. Because it's encrypted
// client-side, the shape of the JSON object is opaque to this worker.
string_newtype!(EncryptedSubmissionBody);

// The organizers' public encryption key used by clients to encrypt their submissions.
string_newtype!(PublicEncryptionKey);

impl PublicEncryptionKey {
    fn key(&self) -> anyhow::Result<crypto_box::PublicKey> {
        Ok(crypto_box::PublicKey::from_slice(
            &BASE64_STANDARD.decode(&self.0)?,
        )?)
    }
}

// TODO: Document
string_newtype!(ApiChallenge);

// TODO: Document
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
        let digest = Sha256::digest(self.0.expose_secret());
        let mut arr = [0u8; HashedApiSecret::LEN_BYTES];
        arr.copy_from_slice(&digest);
        Ok(HashedApiSecret(arr))
    }
}

// TODO: Document
#[derive(Debug, PartialEq, Eq)]
pub struct HashedApiSecret([u8; Self::LEN_BYTES]);

impl HashedApiSecret {
    pub const LEN_BYTES: usize = 32;
}

impl Serialize for HashedApiSecret {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        hex::encode(self.0).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HashedApiSecret {
    fn deserialize<D>(deserializer: D) -> Result<HashedApiSecret, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let bytes = hex::decode(&s).map_err(serde::de::Error::custom)?;
        if bytes.len() != Self::LEN_BYTES {
            return Err(serde::de::Error::custom(format!(
                "expected {} bytes but got {} bytes",
                Self::LEN_BYTES,
                bytes.len()
            )));
        }

        let mut arr = [0u8; Self::LEN_BYTES];
        arr.copy_from_slice(&bytes);

        Ok(HashedApiSecret(arr))
    }
}

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
