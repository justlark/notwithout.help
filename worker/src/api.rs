use serde::{Deserialize, Serialize};

use crate::{
    auth::{ApiChallengeResponse, SignedApiChallenge},
    keys::{ClientNonceSignature, PublicPrimaryKey, PublicSigningKey, WrappedPrivatePrimaryKey},
    models::{
        ClientKeyId, ClientKeys, EncryptedKeyComment, EncryptedSubmissionBody, FormId,
        FormTemplate, Submission,
    },
};

#[derive(Debug, Serialize)]
pub struct GetFormResponse {
    pub org_name: String,
    pub description: String,
    pub contact_methods: Vec<String>,
}

impl From<FormTemplate> for GetFormResponse {
    fn from(template: FormTemplate) -> Self {
        Self {
            org_name: template.org_name,
            description: template.description,
            contact_methods: template.contact_methods,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PublishFormRequest {
    pub public_primary_key: PublicPrimaryKey,
    pub public_signing_key: PublicSigningKey,
    pub org_name: String,
    pub description: String,
    pub contact_methods: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct PublishFormResponse {
    pub form_id: FormId,
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
    pub wrapped_private_primary_key: WrappedPrivatePrimaryKey,
}

#[derive(Debug, Serialize)]
pub struct ListKeysResponse {
    pub client_key_id: ClientKeyId,
    pub encrypted_comment: EncryptedKeyComment,
}

impl From<ClientKeys> for ListKeysResponse {
    fn from(keys: ClientKeys) -> Self {
        Self {
            client_key_id: keys.id,
            encrypted_comment: keys.encrypted_comment,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PostKeyRequest {
    pub wrapped_private_primary_key: WrappedPrivatePrimaryKey,
    pub encrypted_comment: EncryptedKeyComment,
}

#[derive(Debug, Serialize)]
pub struct PostKeyResponse {
    pub client_key_id: ClientKeyId,
}

pub type GetApiChallengeResponse = SignedApiChallenge;

#[derive(Debug, Deserialize)]
pub struct PostTokenRequest {
    pub signature: ClientNonceSignature,
    pub challenge: SignedApiChallenge,
}

impl From<PostTokenRequest> for ApiChallengeResponse {
    fn from(request: PostTokenRequest) -> Self {
        Self {
            signature: request.signature,
            challenge: request.challenge,
        }
    }
}
