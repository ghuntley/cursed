//! Cryptographic functionality for message_digest

use crate::error::CursedError;
use std::collections::HashMap;

/// Result type for crypto operations
pub type CryptoResult<T> = Result<T, CursedError>;

/// Digest processing modes
#[derive(Debug, Clone, PartialEq)]
pub enum DigestMode {
    Incremental,
    Streaming,
    Oneshot,
    Chunked,
}

/// Message format types
#[derive(Debug, Clone, PartialEq)]
pub enum MessageFormat {
    Raw,
    Base64,
    Hex,
    Json,
    Xml,
    Protobuf,
}

/// Digest computation options
#[derive(Debug, Clone)]
pub struct DigestOptions {
    pub format: MessageFormat,
    pub mode: DigestMode,
    pub chunk_size: usize,
    pub include_metadata: bool,
    pub salt: Option<Vec<u8>>,
}

impl Default for DigestOptions {
    fn default() -> Self {
        Self {
            format: MessageFormat::Raw,
            mode: DigestMode::Oneshot,
            chunk_size: 8192,
            include_metadata: false,
            salt: None,
        }
    }
}

/// Message digest result
#[derive(Debug, Clone)]
pub struct MessageDigest {
    pub digest: Vec<u8>,
    pub format: MessageFormat,
    pub algorithm: String,
    pub input_size: usize,
    pub metadata: HashMap<String, String>,
}

impl MessageDigest {
    pub fn new(digest: Vec<u8>, format: MessageFormat, algorithm: String, input_size: usize) -> Self {
        Self {
            digest,
            format,
            algorithm,
            input_size,
            metadata: HashMap::new(),
        }
    }

    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    pub fn to_hex(&self) -> String {
        hex::encode(&self.digest)
    }

    pub fn to_base64(&self) -> String {
        base64::encode(&self.digest)
    }
}

/// Message digest manager
pub struct MessageDigestManager {
    options: DigestOptions,
    supported_formats: Vec<MessageFormat>,
    supported_modes: Vec<DigestMode>,
}

impl MessageDigestManager {
    pub fn new() -> Self {
        Self {
            options: DigestOptions::default(),
            supported_formats: vec![
                MessageFormat::Raw,
                MessageFormat::Base64,
                MessageFormat::Hex,
                MessageFormat::Json,
            ],
            supported_modes: vec![
                DigestMode::Oneshot,
                DigestMode::Streaming,
                DigestMode::Incremental,
            ],
        }
    }

    pub fn with_options(options: DigestOptions) -> Self {
        Self {
            options,
            supported_formats: vec![
                MessageFormat::Raw,
                MessageFormat::Base64,
                MessageFormat::Hex,
                MessageFormat::Json,
            ],
            supported_modes: vec![
                DigestMode::Oneshot,
                DigestMode::Streaming,
                DigestMode::Incremental,
            ],
        }
    }

    pub fn digest_message(&self, data: &[u8], algorithm: &str) -> CryptoResult<MessageDigest> {
        let digest = match algorithm {
            "sha256" => {
                use sha2::{Sha256, Digest};
                let mut hasher = Sha256::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            },
            _ => {
                // Default to SHA-256
                use sha2::{Sha256, Digest};
                let mut hasher = Sha256::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
        };

        let mut message_digest = MessageDigest::new(
            digest,
            self.options.format.clone(),
            algorithm.to_string(),
            data.len(),
        );

        if self.options.include_metadata {
            message_digest.add_metadata("processing_mode".to_string(), format!("{:?}", self.options.mode));
            message_digest.add_metadata("format".to_string(), format!("{:?}", self.options.format));
        }

        Ok(message_digest)
    }

    pub fn set_options(&mut self, options: DigestOptions) {
        self.options = options;
    }

    pub fn get_supported_formats(&self) -> &[MessageFormat] {
        &self.supported_formats
    }

    pub fn get_supported_modes(&self) -> &[DigestMode] {
        &self.supported_modes
    }
}

impl Default for MessageDigestManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Cryptographic operations handler
pub struct CryptoHandler {
    key_size: usize,
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
pub fn init_message_digest() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let key = handler.generate_key()?;
    if key.len() != 32 {
        return Err(CursedError::runtime_error("Crypto key generation test failed"));
    }
    println!("🔐 Crypto processing (message_digest) initialized");
    Ok(())
}

/// Test crypto functionality
pub fn test_message_digest() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let data = b"Hello, CURSED Crypto!";
    let hash = handler.hash_sha256(data);
    if hash.len() != 32 {
        return Err(CursedError::runtime_error("Crypto hash test failed"));
    }
    Ok(())
}
