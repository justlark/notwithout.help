use rand::RngCore;
use secrecy::{ExposeSecret, SecretSlice};

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
