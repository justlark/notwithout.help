use rand::{
    distributions::{Alphanumeric, DistString},
    RngCore,
};
use secrecy::{ExposeSecret, SecretSlice};

use crate::models::{FormId, SubmissionId};

#[derive(Debug, Clone)]
pub struct Secret(SecretSlice<u8>);

impl From<Vec<u8>> for Secret {
    fn from(bytes: Vec<u8>) -> Self {
        Self(SecretSlice::from(bytes))
    }
}

impl Secret {
    pub fn generate(bytes: usize) -> Self {
        let mut bytes = vec![0u8; bytes];
        rand::thread_rng().fill_bytes(&mut bytes);
        Self(SecretSlice::from(bytes))
    }
}

impl ExposeSecret<[u8]> for Secret {
    fn expose_secret(&self) -> &[u8] {
        self.0.expose_secret()
    }
}

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
