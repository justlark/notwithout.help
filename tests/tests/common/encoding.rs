use base64::prelude::*;

pub fn base64_encode(value: impl AsRef<[u8]>) -> String {
    BASE64_STANDARD.encode(value.as_ref())
}
