use constant_time_eq::constant_time_eq;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};

// Base64-encoded encrypted form response.
pub type EncryptedFormSubmission = String;
pub type FormId = String;
pub type SubmissionId = String;
pub type PublicEncryptionKey = String;

#[derive(Debug, Clone, Deserialize)]
pub struct ApiToken(SecretString);

impl ApiToken {
    pub fn from(token: &str) -> Self {
        Self(SecretString::from(token))
    }
}

impl PartialEq for ApiToken {
    fn eq(&self, other: &Self) -> bool {
        constant_time_eq(
            self.0.expose_secret().as_bytes(),
            other.0.expose_secret().as_bytes(),
        )
    }
}

impl Eq for ApiToken {}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Submission {
    pub name: String,
    pub contact: String,
    pub contact_method: String,
}
