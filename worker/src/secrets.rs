use constant_time_eq::constant_time_eq;
use secrecy::{zeroize::Zeroize, CloneableSecret, ExposeSecret, SecretBox, SerializableSecret};
use serde::{Deserialize, Serialize};

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
