use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use worker::wasm_bindgen::JsValue;

use crate::secrets::Secret;

macro_rules! string_newtype {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $name(String);

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                Self(s)
            }
        }

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                Self(s.to_string())
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<$name> for JsValue {
            fn from(s: $name) -> Self {
                s.0.into()
            }
        }
    };
}

string_newtype!(SubmissionId);
string_newtype!(FormId);

// The submission body as a base64-encoded encrypted JSON object. Because it's encrypted
// client-side, the shape of the JSON object is opaque to this worker.
string_newtype!(EncryptedSubmissionBody);

// The organizers' public encryption key used by clients to encrypt their submissions.
string_newtype!(PublicEncryptionKey);

pub type ApiToken = Secret;

// The form template, which is serialized to JSON and stored in the database.
#[derive(Debug, Serialize, Deserialize)]
pub struct FormTemplate {
    pub public_key: PublicEncryptionKey,
    pub api_token: ApiToken,
    pub org_name: String,
    pub description: String,
    pub contact_methods: Vec<String>,
}

// The form template response object sent to clients which **DOES NOT** include the API token.
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

#[derive(Debug)]
pub struct Submission {
    pub encrypted_body: EncryptedSubmissionBody,
    pub created_at: DateTime<Utc>,
}

impl Serialize for Submission {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct Inner {
            encrypted_body: EncryptedSubmissionBody,
            created_at: String,
        }

        let inner = Inner {
            encrypted_body: self.encrypted_body.clone(),
            created_at: self.created_at.to_rfc3339(),
        };

        inner.serialize(serializer)
    }
}
