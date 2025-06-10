/// fr fr Authenticated encryption stub
use super::errors::*;

pub trait AuthenticatedEncryption {
    fn encrypt_with_auth(&self, plaintext: &[u8]) -> AdvancedCryptoResult<Vec<u8>>;
    fn decrypt_with_auth(&self, ciphertext: &[u8]) -> AdvancedCryptoResult<Vec<u8>>;
}

#[derive(Debug, Clone)]
pub struct AuthenticationTag(pub Vec<u8>);

pub type EncryptionResult<T> = Result<T, AdvancedCryptoError>;
pub type DecryptionResult<T> = Result<T, AdvancedCryptoError>;
pub type AuthenticationError = AdvancedCryptoError;
pub type TagMismatchError = AdvancedCryptoError;
