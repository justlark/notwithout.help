use serde::{Deserialize, Serialize};

use crate::secrets::Secret;

pub type EncryptedSubmission = String;
pub type FormId = String;
pub type SubmissionId = String;
pub type PublicEncryptionKey = String;
pub type ApiToken = Secret;

#[derive(Debug, Serialize, Deserialize)]
pub struct FormTemplate {
    pub public_key: PublicEncryptionKey,
    pub api_token: ApiToken,
    pub org_name: String,
    pub description: String,
    pub contact_methods: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct FormResponse {
    pub public_key: PublicEncryptionKey,
    pub org_name: String,
    pub description: String,
    pub contact_methods: Vec<String>,
}

impl From<FormTemplate> for FormResponse {
    fn from(template: FormTemplate) -> Self {
        Self {
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
