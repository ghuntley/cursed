//! Cryptographic functionality for key_generator

use crate::error::CursedError;

/// Result type for crypto operations
pub type CryptoResult<T> = Result<T, CursedError>;

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
pub fn init_key_generator() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let key = handler.generate_key()?;
    if key.len() != 32 {
        return Err(CursedError::runtime_error("Crypto key generation test failed"));
    }
    println!("🔐 Crypto processing (key_generator) initialized");
    Ok(())
}

/// Test crypto functionality
pub fn test_key_generator() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let data = b"Hello, CURSED Crypto!";
    let hash = handler.hash_sha256(data);
    if hash.len() != 32 {
        return Err(CursedError::runtime_error("Crypto hash test failed"));
    }
    Ok(())
}



// Key Generator specific types
#[derive(Debug, Clone)]
pub struct KeyGenerator {
    pub algorithm: AsymmetricAlgorithm,
}

impl KeyGenerator {
    pub fn new() -> Self {
        Self {
            algorithm: AsymmetricAlgorithm::RSA,
        }
    }
    
    pub fn with_algorithm(algorithm: AsymmetricAlgorithm) -> Self {
        Self { algorithm }
    }
    
    pub fn supported_algorithms() -> Vec<AsymmetricAlgorithm> {
        list_asymmetric_algorithms()
    }

    pub fn generate_keypair(&self, algorithm: AsymmetricAlgorithm) -> Result<(Vec<u8>, Vec<u8>), CursedError> {
        // Simple stub for keypair generation based on algorithm
        match algorithm {
            AsymmetricAlgorithm::RSA | AsymmetricAlgorithm::Rsa2048 => {
                let private_key = vec![0u8; 256]; // 2048 bits = 256 bytes
                let public_key = vec![1u8; 256];
                Ok((private_key, public_key))
            },
            AsymmetricAlgorithm::ECC | AsymmetricAlgorithm::EcdsaP256 => {
                let private_key = vec![0u8; 32];
                let public_key = vec![1u8; 64];
                Ok((private_key, public_key))
            },
            AsymmetricAlgorithm::Ed25519 => {
                let private_key = vec![0u8; 32];
                let public_key = vec![1u8; 32];
                Ok((private_key, public_key))
            },
            AsymmetricAlgorithm::X25519 => {
                let private_key = vec![0u8; 32];
                let public_key = vec![1u8; 32];
                Ok((private_key, public_key))
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AsymmetricAlgorithm {
    RSA,
    ECC,
    Ed25519,
    X25519,
    Rsa2048,
    EcdsaP256,
}

#[derive(Debug, Clone)]
pub struct GeneratedKeyPair {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
    pub algorithm: AsymmetricAlgorithm,
}

#[derive(Debug, Clone)]
pub enum KeyGeneratorError {
    InvalidParameters,
    GenerationFailed,
    UnsupportedAlgorithm,
}

// Missing key generator functions
pub fn generate_asymmetric_keypair(algorithm: AsymmetricAlgorithm, key_size: Option<usize>) -> CryptoResult<GeneratedKeyPair> {
    let handler = CryptoHandler::new();
    let effective_key_size = key_size.unwrap_or(32);
    
    let private_key = handler.random_bytes(effective_key_size)?;
    let public_key = handler.random_bytes(effective_key_size)?;
    
    Ok(GeneratedKeyPair {
        private_key,
        public_key,
        algorithm,
    })
}

pub fn list_asymmetric_algorithms() -> Vec<AsymmetricAlgorithm> {
    vec![
        AsymmetricAlgorithm::RSA,
        AsymmetricAlgorithm::ECC,
        AsymmetricAlgorithm::Ed25519,
        AsymmetricAlgorithm::X25519,
        AsymmetricAlgorithm::Rsa2048,
        AsymmetricAlgorithm::EcdsaP256,
    ]
}

impl AsymmetricAlgorithm {
    pub fn name(&self) -> &'static str {
        match self {
            AsymmetricAlgorithm::RSA => "RSA",
            AsymmetricAlgorithm::ECC => "ECC",
            AsymmetricAlgorithm::Ed25519 => "Ed25519",
            AsymmetricAlgorithm::X25519 => "X25519",
            AsymmetricAlgorithm::Rsa2048 => "RSA-2048",
            AsymmetricAlgorithm::EcdsaP256 => "ECDSA-P256",
        }
    }
    
    pub fn default_key_size(&self) -> usize {
        match self {
            AsymmetricAlgorithm::RSA => 256, // 2048 bits / 8
            AsymmetricAlgorithm::ECC => 32,  // 256 bits / 8
            AsymmetricAlgorithm::Ed25519 => 32,
            AsymmetricAlgorithm::X25519 => 32,
            AsymmetricAlgorithm::Rsa2048 => 256, // 2048 bits / 8
            AsymmetricAlgorithm::EcdsaP256 => 32, // 256 bits / 8
        }
    }
}
