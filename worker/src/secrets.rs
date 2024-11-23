use constant_time_eq::constant_time_eq;
use rand::distributions::{Alphanumeric, DistString};
use secrecy::{zeroize::Zeroize, CloneableSecret, ExposeSecret, SecretBox, SerializableSecret};
use serde::{Deserialize, Serialize};

use crate::models::{FormId, SubmissionId};

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SerializableString(String);

impl SerializableSecret for SerializableString {}

impl CloneableSecret for SerializableString {}

impl Zeroize for SerializableString {
    fn zeroize(&mut self) {
        self.0.zeroize()
    }
}

// A string which does constant-time equality checks and is zeroized when dropped.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Secret(SecretBox<SerializableString>);

impl Secret {
    pub fn from(secret: &str) -> Self {
        Self(SecretBox::new(Box::new(SerializableString(
            secret.to_string(),
        ))))
    }
}

impl PartialEq for Secret {
    fn eq(&self, other: &Self) -> bool {
        constant_time_eq(
            self.0.expose_secret().0.as_bytes(),
            other.0.expose_secret().0.as_bytes(),
        )
    }
}

impl Eq for Secret {}

fn random_id(len: usize) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), len)
}

const FORM_ID_LEN: usize = 8;
const SUBMISSION_ID_LEN: usize = 8;

pub fn new_form_id() -> FormId {
    random_id(FORM_ID_LEN).into()
}

pub fn new_submission_id() -> SubmissionId {
    random_id(SUBMISSION_ID_LEN).into()
}
