use std::{fmt, str::FromStr};

use chrono::{DateTime, Utc};
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Debug, Clone, Copy)]
pub struct ClientKeyId(u64);

impl FromStr for ClientKeyId {
    type Err = <u64 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(u64::from_str(s)?))
    }
}

impl fmt::Display for ClientKeyId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// The format of the client ID should be opaque to the client (the fact that it's an
// auto-incrementing integer is an implementation detail), so we serialize it as a string rather
// than as an integer.
impl Serialize for ClientKeyId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

// The client ID is stored in the database as an integer, so we need to be able to deserialize it
// from an integer.
impl<'de> Deserialize<'de> for ClientKeyId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self(u64::deserialize(deserializer)?))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ServerKeyId(Uuid);

impl ServerKeyId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ServerKeyId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for ServerKeyId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl fmt::Display for ServerKeyId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ChallengeId(Uuid);

impl ChallengeId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ChallengeId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ChallengeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// This is opaque to the server, so no need to decode it.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EncryptedSubmissionBody(String);

impl From<String> for EncryptedSubmissionBody {
    fn from(s: String) -> Self {
        Self(s)
    }
}

// This is opaque to the server, so no need to decode it.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
    pub wrapped_private_primary_key: Option<WrappedPrivatePrimaryKey>,
    pub encrypted_comment: EncryptedKeyComment,
    pub accessed_at: Option<DateTime<Utc>>,
}
