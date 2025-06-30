
use crate::error::{Result, CursedError};

#[derive(Debug, Clone)]
pub enum PkcsError {
    InvalidFormat,
    DecryptionFailed,
    EncryptionFailed,
}

pub type PkcsResult<T> = Result<T>;

pub fn encrypt_private_key(key: &[u8], password: &str) -> PkcsResult<Vec<u8>> {
    Ok(key.to_vec())
}

pub fn decrypt_private_key(encrypted_key: &[u8], password: &str) -> PkcsResult<Vec<u8>> {
    Ok(encrypted_key.to_vec())
}
