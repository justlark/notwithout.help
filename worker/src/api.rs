use serde::{Deserialize, Serialize};

use crate::{
    keys::{FormId, PublicClientKey, PublicServerKey, PublicWrappingKey},
    models::{
        ClientKeyId, EncryptedKeyComment, EncryptedSubmissionBody, ServerKeyId, Submission,
        WrappedPrivateClientKey,
    },
};

#[derive(Debug, Serialize)]
pub struct GetFormResponse {
    pub pub_server_key: PublicServerKey,
    pub server_key_id: ServerKeyId,
    pub org_name: String,
    pub description: String,
    pub contact_methods: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct PublishFormRequest {
    pub pub_client_key: PublicClientKey,
    pub pub_wrapping_key: PublicWrappingKey,
    pub org_name: String,
    pub description: String,
    pub contact_methods: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct PublishFormResponse {
    pub form_id: FormId,
    pub pub_server_key: PublicServerKey,
    pub server_key_id: ServerKeyId,
    pub client_key_id: ClientKeyId,
}

pub type PostSubmissionRequest = EncryptedSubmissionBody;

#[derive(Debug, Serialize)]
pub struct ListSubmissionsResponse {
    pub encrypted_body: EncryptedSubmissionBody,
    pub created_at: String,
}

impl From<Submission> for ListSubmissionsResponse {
    fn from(submission: Submission) -> Self {
        Self {
            encrypted_body: submission.encrypted_body,
            created_at: submission.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct GetKeyResponse {
    pub wrapped_client_key: WrappedPrivateClientKey,
}

#[derive(Debug, Serialize)]
pub struct ListKeysResponse {
    pub client_key_id: ClientKeyId,
    pub wrapped_client_key: WrappedPrivateClientKey,
    pub comment: EncryptedKeyComment,
}

#[derive(Debug, Deserialize)]
pub struct PostKeyRequest {
    pub wrapped_client_key: WrappedPrivateClientKey,
    pub comment: EncryptedKeyComment,
}

#[derive(Debug, Serialize)]
pub struct PostKeyResponse {
    pub client_key_id: ClientKeyId,
}
