use argon2::{
    password_hash::{Output as Argon2Output, PasswordHasher, Salt},
    Argon2,
};
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

#[derive(Debug, PartialEq, Eq)]
pub struct PasswordHash(Argon2Output);

impl PasswordHash {
    // TODO: Document
    const APPLICATION_SALT: &str = "oPnyEaWz/Zq2SNZlzGjVCQ";

    pub fn new(bytes: &[u8]) -> anyhow::Result<Self> {
        Ok(Self(Argon2Output::new(bytes)?))
    }

    pub fn from_password(password: &[u8]) -> anyhow::Result<Self> {
        let argon2 = Argon2::default();
        let salt = Salt::from_b64(Self::APPLICATION_SALT)?;
        Ok(PasswordHash(
            argon2
                .hash_password(password, salt)?
                .hash
                .expect("password hash output missing"),
        ))
    }
}

impl AsRef<[u8]> for PasswordHash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}
