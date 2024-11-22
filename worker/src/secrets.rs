use constant_time_eq::constant_time_eq;
use secrecy::ExposeSecret;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct Secret(secrecy::SecretString);

impl Secret {
    pub fn from(secret: &str) -> Self {
        Self(secrecy::SecretString::from(secret))
    }
}

impl PartialEq for Secret {
    fn eq(&self, other: &Self) -> bool {
        constant_time_eq(
            self.0.expose_secret().as_bytes(),
            other.0.expose_secret().as_bytes(),
        )
    }
}

impl Eq for Secret {}
