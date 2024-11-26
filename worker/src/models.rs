use std::str::FromStr;

use chrono::{DateTime, Utc};
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};

use crate::crypt::{PrivateServerKey, PublicServerKey, PublicWrappingKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
struct RandomId(String);

impl RandomId {
    fn new(len: usize) -> Self {
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

// The submission body as a base64-encoded encrypted JSON object. Because it's encrypted
// client-side, the shape of the JSON object is opaque to this worker.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EncryptedSubmissionBody(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EncryptedKeyComment(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WrappedPrivateClientKey(String);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ClientKeyId(u32);

impl FromStr for ClientKeyId {
    type Err = <u32 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(u32::from_str(s)?))
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ServerKeyId(u32);

impl FromStr for ServerKeyId {
    type Err = <u32 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(u32::from_str(s)?))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormTemplate {
    pub org_name: String,
    pub description: String,
    pub contact_methods: Vec<String>,
}

#[derive(Debug)]
pub struct Submission {
    pub encrypted_body: EncryptedSubmissionBody,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct ServerKeyPair {
    pub id: ServerKeyId,
    pub public_key: PublicServerKey,
    pub private_key: PrivateServerKey,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct ClientKeyPair {
    pub id: ClientKeyId,
    pub public_wrapping_key: PublicWrappingKey,
    pub wrapped_private_key: WrappedPrivateClientKey,
    pub encrypted_comment: EncryptedKeyComment,
    pub created_at: DateTime<Utc>,
}
