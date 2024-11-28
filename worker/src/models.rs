use std::{fmt, str::FromStr};

use chrono::{DateTime, Utc};
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};

use crate::keys::{PublicSigningKey, WrappedPrivatePrimaryKey};

//
// See the security architecture document for information on the purpose of these values and how
// they're used. The names of identifiers in this file generally match the terms defined in that
// document.
//
// https://github.com/justlark/notwithout.help/blob/main/docs/security-whitepaper.md
//

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
struct RandomId(String);

// While this is a CSPRNG, we don't rely on the randomness of these IDs for security.
impl RandomId {
    fn new(len: usize) -> Self {
        Self(Alphanumeric.sample_string(&mut rand::thread_rng(), len))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

impl From<String> for FormId {
    fn from(s: String) -> Self {
        Self(RandomId(s))
    }
}

impl fmt::Display for FormId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0 .0)
    }
}

// As of time of writing, the submission ID isn't used anywhere. It exists only for
// future-proofing.
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

//
// The Client Key ID is implemented as an auto-incrementing integer. It increments independently
// for each form, meaning that:
//
// - No two keys associated with the same form will ever have the same ID.
// - IDs are not recycled when keys are revoked.
// - Keys associated with different forms may have the same ID.
//

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ClientKeyId(u32);

impl FromStr for ClientKeyId {
    type Err = <u32 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(u32::from_str(s)?))
    }
}

impl fmt::Display for ClientKeyId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ServerKeyId(RandomId);

impl ServerKeyId {
    const LEN: usize = 16;

    pub fn new() -> Self {
        Self(RandomId::new(Self::LEN))
    }
}

impl Default for ServerKeyId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<String> for ServerKeyId {
    fn from(s: String) -> Self {
        Self(RandomId(s))
    }
}

impl fmt::Display for ServerKeyId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0 .0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ChallengeId(RandomId);

impl ChallengeId {
    const LEN: usize = 16;

    pub fn new() -> Self {
        Self(RandomId::new(Self::LEN))
    }
}

impl Default for ChallengeId {
    fn default() -> Self {
        Self::new()
    }
}

// This is opaque to the server, so no need to decode it.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EncryptedSubmissionBody(String);

// This is opaque to the server, so no need to decode it.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EncryptedKeyComment(String);

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
pub struct ClientKeys {
    pub id: ClientKeyId,
    pub public_signing_key: PublicSigningKey,
    pub wrapped_private_primary_key: WrappedPrivatePrimaryKey,
    pub encrypted_comment: EncryptedKeyComment,
    pub created_at: DateTime<Utc>,
}
