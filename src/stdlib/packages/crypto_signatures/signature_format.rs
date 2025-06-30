//! Cryptographic functionality for signature_format

use crate::error::CursedError;

/// Result type for crypto operations
pub type CryptoResult<T> = Result<T, CursedError>;

/// Cryptographic operations handler
pub struct CryptoHandler {
    key_size: usize,
}

#[derive(Debug, Clone)]
pub enum SignatureFormat {
    Der,
    Pkcs7,
    Jwk,
    Raw,
}

#[derive(Debug, Clone)]
pub struct SignatureFormatHandler {
    format: SignatureFormat,
    options: EncodingOptions,
}

#[derive(Debug, Clone, Default)]
pub struct EncodingOptions {
    pub compression: bool,
    pub base64_encoding: bool,
    pub metadata_included: bool,
}

#[derive(Debug, Clone)]
pub struct SignatureMetadata {
    pub timestamp: u64,
    pub signer_info: String,
    pub algorithm: String,
}

#[derive(Debug, Clone)]
pub struct EncodedSignature {
    pub data: Vec<u8>,
    pub format: SignatureFormat,
    pub metadata: Option<SignatureMetadata>,
}

impl SignatureFormatHandler {
    pub fn new(format: SignatureFormat) -> Self {
        Self { 
            format, 
            options: EncodingOptions::default() 
        }
    }
    
    pub fn encode(&self, signature: &[u8]) -> CryptoResult<EncodedSignature> {
        Ok(EncodedSignature {
            data: signature.to_vec(),
            format: self.format.clone(),
            metadata: None,
        })
    }
    
    pub fn decode(&self, encoded: &EncodedSignature) -> CryptoResult<Vec<u8>> {
        Ok(encoded.data.clone())
    }
}

impl CryptoHandler {
    /// Create a new crypto handler
    pub fn new() -> Self {
        Self {
            key_size: 32,
        }
    }
    
    /// Set key size
    pub fn key_size(mut self, size: usize) -> Self {
        self.key_size = size;
        self
    }
    
    /// Generate random bytes
    pub fn random_bytes(&self, size: usize) -> CryptoResult<Vec<u8>> {
        use rand::RngCore;
        let mut rng = rand::thread_rng();
        let mut bytes = vec![0u8; size];
        rng.fill_bytes(&mut bytes);
        Ok(bytes)
    }
    
    /// Hash data using SHA-256
    pub fn hash_sha256(&self, data: &[u8]) -> Vec<u8> {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
    
    /// Generate a key
    pub fn generate_key(&self) -> CryptoResult<Vec<u8>> {
        self.random_bytes(self.key_size)
    }
    
    /// Encode to hex
    pub fn to_hex(&self, data: &[u8]) -> String {
        hex::encode(data)
    }
    
    /// Decode from hex
    pub fn from_hex(&self, hex_str: &str) -> CryptoResult<Vec<u8>> {
        hex::decode(hex_str).map_err(|e| CursedError::runtime_error(&format!("Hex decode error: {}", e)))
    }
}

impl Default for CryptoHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize crypto processing
pub fn init_signature_format() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let key = handler.generate_key()?;
    if key.len() != 32 {
        return Err(CursedError::runtime_error("Crypto key generation test failed"));
    }
    println!("🔐 Crypto processing (signature_format) initialized");
    Ok(())
}

/// Test crypto functionality
pub fn test_signature_format() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let data = b"Hello, CURSED Crypto!";
    let hash = handler.hash_sha256(data);
    if hash.len() != 32 {
        return Err(CursedError::runtime_error("Crypto hash test failed"));
    }
    Ok(())
}
