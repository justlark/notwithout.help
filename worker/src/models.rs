use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::keys::{PrivateServerKey, PublicServerKey};

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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ServerKeyId(u32);

#[derive(Debug)]
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
    pub index: ServerKeyId,
    pub public_key: PublicServerKey,
    pub private_key: PrivateServerKey,
    pub created_at: DateTime<Utc>,
}
