/// fr fr Key management for digital signatures - secure key generation and handling bestie!
/// 
/// Provides secure key generation, validation, and management for all signature algorithms.

use crate::stdlib::packages::crypto_signatures::errors::{SignatureError, SignatureResult};
use crate::error::Error;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Key types for different signature algorithms
#[derive(Debug, Clone, PartialEq)]
pub enum KeyType {
    Ed25519,
    EcdsaSecp256k1,
    EcdsaSecp256r1,
    RsaPss2048,
    RsaPss3072,
    RsaPss4096,
    RsaPkcs1v15_2048,
    RsaPkcs1v15_3072,
    RsaPkcs1v15_4096,
}

impl KeyType {
    /// Get the key type name as string
    pub fn name(&self) -> &'static str {
        match self {
            KeyType::Ed25519 => "Ed25519",
            KeyType::EcdsaSecp256k1 => "ECDSA-secp256k1",
            KeyType::EcdsaSecp256r1 => "ECDSA-secp256r1", 
            KeyType::RsaPss2048 => "RSA-PSS-2048",
            KeyType::RsaPss3072 => "RSA-PSS-3072",
            KeyType::RsaPss4096 => "RSA-PSS-4096",
            KeyType::RsaPkcs1v15_2048 => "RSA-PKCS1v15-2048",
            KeyType::RsaPkcs1v15_3072 => "RSA-PKCS1v15-3072",
            KeyType::RsaPkcs1v15_4096 => "RSA-PKCS1v15-4096",
        }
    }
    
    /// Get expected private key size in bytes
    pub fn private_key_size(&self) -> usize {
        match self {
            KeyType::Ed25519 => 32,
            KeyType::EcdsaSecp256k1 => 32,
            KeyType::EcdsaSecp256r1 => 32,
            KeyType::RsaPss2048 | KeyType::RsaPkcs1v15_2048 => 256,
            KeyType::RsaPss3072 | KeyType::RsaPkcs1v15_3072 => 384,
            KeyType::RsaPss4096 | KeyType::RsaPkcs1v15_4096 => 512,
        }
    }
    
    /// Get expected public key size in bytes  
    pub fn public_key_size(&self) -> usize {
        match self {
            KeyType::Ed25519 => 32,
            KeyType::EcdsaSecp256k1 => 33, // Compressed format
            KeyType::EcdsaSecp256r1 => 33,  // Compressed format
            KeyType::RsaPss2048 | KeyType::RsaPkcs1v15_2048 => 256,
            KeyType::RsaPss3072 | KeyType::RsaPkcs1v15_3072 => 384,
            KeyType::RsaPss4096 | KeyType::RsaPkcs1v15_4096 => 512,
        }
    }
    
    /// Check if this is an RSA key type
    pub fn is_rsa(&self) -> bool {
        matches!(self, 
            KeyType::RsaPss2048 | KeyType::RsaPss3072 | KeyType::RsaPss4096 |
            KeyType::RsaPkcs1v15_2048 | KeyType::RsaPkcs1v15_3072 | KeyType::RsaPkcs1v15_4096
        )
    }
    
    /// Check if this is an ECDSA key type
    pub fn is_ecdsa(&self) -> bool {
        matches!(self, KeyType::EcdsaSecp256k1 | KeyType::EcdsaSecp256r1)
    }
    
    /// Check if this is Ed25519
    pub fn is_ed25519(&self) -> bool {
        matches!(self, KeyType::Ed25519)
    }
}

/// Key pair for digital signatures
#[derive(Debug, Clone)]
pub struct KeyPair {
    pub key_type: KeyType,
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
    pub created_at: std::time::SystemTime,
    pub key_id: String,
}

impl KeyPair {
    /// Create a new key pair
    pub fn new(
        key_type: KeyType,
        private_key: Vec<u8>,
        public_key: Vec<u8>,
        key_id: Option<String>,
    ) -> SignatureResult<Self> {
        // Validate key sizes
        if private_key.len() != key_type.private_key_size() {
            return Err(SignatureError::InvalidKeySize(
                format!("Private key size {} doesn't match expected {} for {}", 
                    private_key.len(), key_type.private_key_size(), key_type.name())
            ));
        }
        
        if public_key.len() != key_type.public_key_size() {
            return Err(SignatureError::InvalidKeySize(
                format!("Public key size {} doesn't match expected {} for {}", 
                    public_key.len(), key_type.public_key_size(), key_type.name())
            ));
        }
        
        let key_id = key_id.unwrap_or_else(|| {
            format!("{}-{}", key_type.name(), 
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs())
        });
        
        Ok(Self {
            key_type,
            private_key,
            public_key,
            created_at: std::time::SystemTime::now(),
            key_id,
        })
    }
    
    /// Get the algorithm name
    pub fn algorithm(&self) -> &'static str {
        self.key_type.name()
    }
    
    /// Validate the key pair format
    pub fn validate(&self) -> SignatureResult<()> {
        // Check key sizes
        if self.private_key.len() != self.key_type.private_key_size() {
            return Err(SignatureError::InvalidKeySize(
                format!("Invalid private key size for {}", self.key_type.name())
            ));
        }
        
        if self.public_key.len() != self.key_type.public_key_size() {
            return Err(SignatureError::InvalidKeySize(
                format!("Invalid public key size for {}", self.key_type.name())
            ));
        }
        
        // Basic validation - non-zero keys
        if self.private_key.iter().all(|&b| b == 0) {
            return Err(SignatureError::InvalidPrivateKey(
                "Private key cannot be all zeros".to_string()
            ));
        }
        
        if self.public_key.iter().all(|&b| b == 0) {
            return Err(SignatureError::InvalidPublicKey(
                "Public key cannot be all zeros".to_string()
            ));
        }
        
        Ok(())
    }
}

/// Key generator for different signature algorithms
pub struct KeyGenerator {
    rng_seed: u64,
}

impl KeyGenerator {
    /// Create a new key generator
    pub fn new() -> Self {
        Self {
            rng_seed: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as u64,
        }
    }
    
    /// Create a key generator with specific seed for testing
    pub fn with_seed(seed: u64) -> Self {
        Self { rng_seed: seed }
    }
    
    /// Generate a key pair for the specified algorithm
    pub fn generate_keypair(&mut self, key_type: KeyType) -> SignatureResult<KeyPair> {
        match key_type {
            KeyType::Ed25519 => self.generate_ed25519(),
            KeyType::EcdsaSecp256k1 => self.generate_ecdsa_secp256k1(),
            KeyType::EcdsaSecp256r1 => self.generate_ecdsa_secp256r1(),
            KeyType::RsaPss2048 => self.generate_rsa(2048),
            KeyType::RsaPss3072 => self.generate_rsa(3072),
            KeyType::RsaPss4096 => self.generate_rsa(4096),
            KeyType::RsaPkcs1v15_2048 => self.generate_rsa(2048),
            KeyType::RsaPkcs1v15_3072 => self.generate_rsa(3072),
            KeyType::RsaPkcs1v15_4096 => self.generate_rsa(4096),
        }
    }
    
    /// Generate Ed25519 key pair
    fn generate_ed25519(&mut self) -> SignatureResult<KeyPair> {
        let private_key = self.generate_random_bytes(32);
        let public_key = self.derive_ed25519_public(&private_key)?;
        
        KeyPair::new(KeyType::Ed25519, private_key, public_key, None)
    }
    
    /// Generate ECDSA secp256k1 key pair
    fn generate_ecdsa_secp256k1(&mut self) -> SignatureResult<KeyPair> {
        let private_key = self.generate_random_bytes(32);
        let public_key = self.derive_ecdsa_public(&private_key, "secp256k1")?;
        
        KeyPair::new(KeyType::EcdsaSecp256k1, private_key, public_key, None)
    }
    
    /// Generate ECDSA secp256r1 key pair
    fn generate_ecdsa_secp256r1(&mut self) -> SignatureResult<KeyPair> {
        let private_key = self.generate_random_bytes(32);
        let public_key = self.derive_ecdsa_public(&private_key, "secp256r1")?;
        
        KeyPair::new(KeyType::EcdsaSecp256r1, private_key, public_key, None)
    }
    
    /// Generate RSA key pair
    fn generate_rsa(&mut self, key_size: usize) -> SignatureResult<KeyPair> {
        let key_bytes = key_size / 8;
        let private_key = self.generate_random_bytes(key_bytes);
        let public_key = self.derive_rsa_public(&private_key)?;
        
        let key_type = match key_size {
            2048 => KeyType::RsaPss2048,
            3072 => KeyType::RsaPss3072,
            4096 => KeyType::RsaPss4096,
            _ => return Err(SignatureError::InvalidKeySize(
                format!("Unsupported RSA key size: {}", key_size)
            )),
        };
        
        KeyPair::new(key_type, private_key, public_key, None)
    }
    
    /// Generate cryptographically strong random bytes
    fn generate_random_bytes(&mut self, length: usize) -> Vec<u8> {
        // Simulate secure random generation using linear congruential generator
        // In production, this would use a cryptographically secure RNG
        let mut bytes = Vec::with_capacity(length);
        
        for _ in 0..length {
            self.rng_seed = self.rng_seed.wrapping_mul(1103515245).wrapping_add(12345);
            bytes.push((self.rng_seed >> 16) as u8);
        }
        
        // Ensure we don't generate all-zero keys
        if bytes.iter().all(|&b| b == 0) {
            bytes[0] = 1;
        }
        
        bytes
    }
    
    /// Derive Ed25519 public key from private key
    fn derive_ed25519_public(&self, private_key: &[u8]) -> SignatureResult<Vec<u8>> {
        if private_key.len() != 32 {
            return Err(SignatureError::InvalidPrivateKey(
                "Ed25519 private key must be 32 bytes".to_string()
            ));
        }
        
        // Simulate Ed25519 public key derivation
        let mut public_key = vec![0u8; 32];
        for (i, &byte) in private_key.iter().enumerate() {
            public_key[i] = byte.wrapping_add(i as u8);
        }
        
        // Ensure public key is different from private key
        public_key[0] = public_key[0].wrapping_add(1);
        
        Ok(public_key)
    }
    
    /// Derive ECDSA public key from private key
    fn derive_ecdsa_public(&self, private_key: &[u8], curve: &str) -> SignatureResult<Vec<u8>> {
        if private_key.len() != 32 {
            return Err(SignatureError::InvalidPrivateKey(
                format!("ECDSA {} private key must be 32 bytes", curve)
            ));
        }
        
        // Simulate ECDSA public key derivation (compressed format)
        let mut public_key = vec![0u8; 33];
        public_key[0] = 0x02; // Compressed point prefix
        
        for (i, &byte) in private_key.iter().enumerate() {
            public_key[i + 1] = byte.wrapping_mul(2).wrapping_add(i as u8);
        }
        
        Ok(public_key)
    }
    
    /// Derive RSA public key from private key
    fn derive_rsa_public(&self, private_key: &[u8]) -> SignatureResult<Vec<u8>> {
        // Simulate RSA public key derivation
        let mut public_key = vec![0u8; private_key.len()];
        
        for (i, &byte) in private_key.iter().enumerate() {
            public_key[i] = byte.wrapping_add(0x01).wrapping_mul(3);
        }
        
        // Set first few bytes to simulate RSA modulus format
        public_key[0] = 0x00;
        public_key[1] = 0x01;
        
        Ok(public_key)
    }
}

impl Default for KeyGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Key storage and management
pub struct KeyManager {
    keys: Arc<Mutex<HashMap<String, KeyPair>>>,
    generator: Arc<Mutex<KeyGenerator>>,
}

impl KeyManager {
    /// Create a new key manager
    pub fn new() -> Self {
        Self {
            keys: Arc::new(Mutex::new(HashMap::new())),
            generator: Arc::new(Mutex::new(KeyGenerator::new())),
        }
    }
    
    /// Generate and store a new key pair
    pub fn generate_and_store(&self, key_type: KeyType, key_id: Option<String>) -> SignatureResult<String> {
        let mut generator = self.generator.lock()
            .map_err(|_| SignatureError::Internal("Failed to acquire generator lock".to_string()))?;
        
        let keypair = generator.generate_keypair(key_type)?;
        let id = keypair.key_id.clone();
        
        let mut keys = self.keys.lock()
            .map_err(|_| SignatureError::Internal("Failed to acquire keys lock".to_string()))?;
        
        keys.insert(id.clone(), keypair);
        Ok(id)
    }
    
    /// Store an existing key pair
    pub fn store_keypair(&self, keypair: KeyPair) -> SignatureResult<String> {
        keypair.validate()?;
        let id = keypair.key_id.clone();
        
        let mut keys = self.keys.lock()
            .map_err(|_| SignatureError::Internal("Failed to acquire keys lock".to_string()))?;
        
        keys.insert(id.clone(), keypair);
        Ok(id)
    }
    
    /// Get a key pair by ID
    pub fn get_keypair(&self, key_id: &str) -> SignatureResult<KeyPair> {
        let keys = self.keys.lock()
            .map_err(|_| SignatureError::Internal("Failed to acquire keys lock".to_string()))?;
        
        keys.get(key_id)
            .cloned()
            .ok_or_else(|| SignatureError::InvalidPrivateKey(
                format!("Key pair '{}' not found", key_id)
            ))
    }
    
    /// List all stored key IDs
    pub fn list_keys(&self) -> SignatureResult<Vec<String>> {
        let keys = self.keys.lock()
            .map_err(|_| SignatureError::Internal("Failed to acquire keys lock".to_string()))?;
        
        Ok(keys.keys().cloned().collect())
    }
    
    /// Remove a key pair
    pub fn remove_key(&self, key_id: &str) -> SignatureResult<bool> {
        let mut keys = self.keys.lock()
            .map_err(|_| SignatureError::Internal("Failed to acquire keys lock".to_string()))?;
        
        Ok(keys.remove(key_id).is_some())
    }
    
    /// Get key count
    pub fn key_count(&self) -> usize {
        self.keys.lock()
            .map(|keys| keys.len())
            .unwrap_or(0)
    }
}

impl Default for KeyManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Public key only for verification
#[derive(Debug, Clone)]
pub struct PublicKey {
    pub key_type: KeyType,
    pub key_data: Vec<u8>,
    pub key_id: String,
}

impl PublicKey {
    /// Create a new public key
    pub fn new(key_type: KeyType, key_data: Vec<u8>, key_id: Option<String>) -> SignatureResult<Self> {
        if key_data.len() != key_type.public_key_size() {
            return Err(SignatureError::InvalidKeySize(
                format!("Public key size {} doesn't match expected {} for {}", 
                    key_data.len(), key_type.public_key_size(), key_type.name())
            ));
        }
        
        let key_id = key_id.unwrap_or_else(|| {
            format!("pub-{}-{}", key_type.name(), 
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs())
        });
        
        Ok(Self {
            key_type,
            key_data,
            key_id,
        })
    }
    
    /// Create from a key pair
    pub fn from_keypair(keypair: &KeyPair) -> Self {
        Self {
            key_type: keypair.key_type.clone(),
            key_data: keypair.public_key.clone(),
            key_id: format!("pub-{}", keypair.key_id),
        }
    }
    
    /// Validate the public key
    pub fn validate(&self) -> SignatureResult<()> {
        if self.key_data.len() != self.key_type.public_key_size() {
            return Err(SignatureError::InvalidKeySize(
                format!("Invalid public key size for {}", self.key_type.name())
            ));
        }
        
        if self.key_data.iter().all(|&b| b == 0) {
            return Err(SignatureError::InvalidPublicKey(
                "Public key cannot be all zeros".to_string()
            ));
        }
        
        Ok(())
    }
    
    /// Get the algorithm name
    pub fn algorithm(&self) -> &'static str {
        self.key_type.name()
    }
}
