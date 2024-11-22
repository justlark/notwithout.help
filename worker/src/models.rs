use serde::{Deserialize, Serialize};

// Base64-encoded encrypted form response.
pub type EncryptedFormResponse = String;

pub type FormId = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct FormTemplate {
    pub public_key: String,
    pub org_name: String,
    pub description: String,
    pub contact_methods: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct FormResponse {
    pub form_id: FormId,
}
