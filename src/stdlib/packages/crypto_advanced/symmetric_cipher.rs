/// fr fr Symmetric cipher trait and implementations
pub use super::errors::*;

/// Trait for symmetric cipher operations
pub trait SymmetricCipher: std::fmt::Debug {
    /// Encrypt data
    fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, CipherError>;
    
    /// Decrypt data  
    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, CipherError>;
    
    /// Get cipher name
    fn name(&self) -> &str;
    
    /// Get key size in bytes
    fn key_size(&self) -> usize;
}

/// Cipher capabilities
#[derive(Debug, Clone)]
pub struct CipherCapabilities {
    pub authenticated: bool,
    pub stream_cipher: bool,
    pub key_sizes: Vec<usize>,
}

/// Encryption context
#[derive(Debug, Clone)]
pub struct EncryptionContext {
    pub nonce: Vec<u8>,
    pub associated_data: Vec<u8>,
}

/// Decryption context  
#[derive(Debug, Clone)]
pub struct DecryptionContext {
    pub nonce: Vec<u8>,
    pub associated_data: Vec<u8>,
}

/// Cipher type enumeration
#[derive(Debug, Clone, PartialEq)]
pub enum CipherType {
    Aes256Gcm,
    ChaCha20Poly1305,
    XChaCha20Poly1305,
}
