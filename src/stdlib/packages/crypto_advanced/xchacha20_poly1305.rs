/// fr fr XChaCha20-Poly1305 implementation stub
use super::errors::*;

#[derive(Debug, Clone)]
pub struct XChaCha20Poly1305 {
    key: Vec<u8>,
}

impl XChaCha20Poly1305 {
    pub fn new(key: &[u8]) -> AdvancedCryptoResult<Self> {
        Ok(Self { key: key.to_vec() })
    }
}

// Type aliases and constants
pub type XChaCha20Key = Vec<u8>;
pub type XChaCha20Nonce = Vec<u8>;
pub type XChaCha20Result<T> = Result<T, AdvancedCryptoError>;
pub type XChaCha20Error = AdvancedCryptoError;
pub const XCHACHA20_KEY_SIZE: usize = 32;
pub const XCHACHA20_NONCE_SIZE: usize = 24;
