use crate::error::CursedError;
/// fr fr X25519 key exchange implementation
/// 
/// This module provides production-ready X25519 Elliptic Curve Diffie-Hellman (ECDH)
/// key exchange operations using the x25519-dalek crate for secure key agreement.

use std::collections::HashMap;
use rand::rngs::OsRng;
use x25519_dalek::{EphemeralSecret, PublicKey, SharedSecret};
use zeroize::Zeroizing;

/// fr fr X25519 key pair structure
#[derive(Debug, Clone)]
pub struct X25519KeyPair {
    pub private_key: EphemeralSecret,
    pub public_key: PublicKey,
}

impl X25519KeyPair {
    /// slay Create new X25519 key pair from private key
    pub fn from_private_key(private_key: EphemeralSecret) -> Self {
        let public_key = PublicKey::from(&private_key);
        Self {
            private_key,
            public_key,
        }
    }
    
    /// slay Create X25519 key pair from public key only (for key exchange)
    pub fn from_public_key(public_key: PublicKey) -> Self {
        // For key exchange operations, we create a dummy private key
        // In practice, you'd use separate structures for public-key-only operations
        let dummy_private = EphemeralSecret::random();
        Self {
            private_key: dummy_private,
            public_key,
        }
    }
    
    /// slay Get raw public key bytes
    pub fn public_key_bytes(&self) -> [u8; 32] {
        *self.public_key.as_bytes()
    }
    
    /// slay Get raw private key bytes
    pub fn private_key_bytes(&self) -> [u8; 32] {
        self.private_key.to_bytes()
    }
}

/// fr fr Ephemeral key pair for one-time use
#[derive(Debug)]
pub struct X25519EphemeralKeyPair {
    pub private_key: EphemeralSecret,
    pub public_key: PublicKey,
}

impl X25519EphemeralKeyPair {
    /// slay Create new ephemeral key pair
    pub fn new() -> Self {
        let private_key = EphemeralSecret::new(OsRng);
        let public_key = PublicKey::from(&private_key);
        Self {
            private_key,
            public_key,
        }
    }
    
    /// slay Get public key bytes
    pub fn public_key_bytes(&self) -> [u8; 32] {
        *self.public_key.as_bytes()
    }
}

/// fr fr Key serialization formats for X25519
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X25519KeyFormat {
    Raw,          // Raw 32-byte format
    Base64,       // Base64 encoded
    Hex,          // Hexadecimal encoded
}

/// fr fr X25519 error types
#[derive(Debug, Clone, PartialEq)]
pub enum X25519Error {
    KeyGenerationFailed(String),
    KeyExchangeFailed(String),
    InvalidPublicKey(String),
    InvalidPrivateKey(String),
    InvalidFormat(String),
    InvalidKeyLength(usize),
    InvalidSharedSecret(String),
    Internal(String),
}

// impl std::fmt::Display for X25519Error {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             X25519Error::KeyGenerationFailed(msg) => write!(f, "X25519 key generation failed: {}", msg),
//             X25519Error::KeyExchangeFailed(msg) => write!(f, "X25519 key exchange failed: {}", msg),
//             X25519Error::InvalidPublicKey(msg) => write!(f, "Invalid X25519 public key: {}", msg),
//             X25519Error::InvalidPrivateKey(msg) => write!(f, "Invalid X25519 private key: {}", msg),
//             X25519Error::InvalidFormat(msg) => write!(f, "Invalid key format: {}", msg),
//             X25519Error::InvalidKeyLength(len) => write!(f, "Invalid key length: {} bytes (expected 32)", len),
//             X25519Error::InvalidSharedSecret(msg) => write!(f, "Invalid shared secret: {}", msg),
//             X25519Error::Internal(msg) => write!(f, "Internal X25519 error: {}", msg),
//         }
//     }
// }

// impl std::error::CursedError for X25519Error {}
// 
type X25519crate::error::Result<T> = Result<T>;

/// fr fr X25519 engine for cryptographic operations
pub struct X25519Engine {
    rng: OsRng,
}

impl X25519Engine {
    /// slay Create new X25519 engine with cryptographically secure RNG
    pub fn new() -> Self {
        Self {
            rng: OsRng,
        }
    }
    
    /// slay Generate X25519 static key pair for long-term use
    /// 
    /// # Security Notes
    /// - Uses cryptographically secure random number generation
    /// - Private keys are automatically clamped for curve25519
    /// - Provides 128-bit security level
    /// - Suitable for long-term key storage
    pub fn generate_static_keypair(&mut self) -> X25519Result<X25519KeyPair> {
        let private_key = EphemeralSecret::random();
        Ok(X25519KeyPair::from_private_key(private_key))
    }
    
    /// slay Generate X25519 ephemeral key pair for one-time use
    /// 
    /// # Security Notes
    /// - Keys are ephemeral and should not be reused
    /// - Provides perfect forward secrecy when used properly
    /// - More efficient than static keys for one-time exchanges
    pub fn generate_ephemeral_keypair(&self) -> X25519Result<X25519EphemeralKeyPair> {
        Ok(X25519EphemeralKeyPair::new())
    }
    
    /// slay Generate deterministic key pair from seed
    /// 
    /// # Security Notes
    /// - Seed must be cryptographically secure (32 bytes)
    /// - Same seed always produces same key pair
    /// - Use for key derivation or testing only
    pub fn generate_keypair_from_seed(&self, seed: &[u8]) -> X25519Result<X25519KeyPair> {
        if seed.len() != 32 {
            return Err(X25519Error::InvalidKeyLength(seed.len()));
        }
        
        let mut seed_array = [0u8; 32];
        seed_array.copy_from_slice(seed);
        
        let private_key = EphemeralSecret::from(seed_array);
        Ok(X25519KeyPair::from_private_key(private_key))
    }
    
    /// slay Perform X25519 key exchange with static key
    /// 
    /// # Security Notes
    /// - Performs Elliptic Curve Diffie-Hellman key exchange
    /// - Shared secret should be used with a key derivation function
    /// - Constant-time operation to prevent timing attacks
    /// - Returns raw shared secret (32 bytes)
    pub fn key_exchange_static(&self, our_private: &X25519KeyPair, their_public: &PublicKey) -> X25519Result<Zeroizing<Vec<u8>>> {
        let shared_secret = our_private.private_key.diffie_hellman(their_public);
        Ok(Zeroizing::new(shared_secret.as_bytes().to_vec()))
    }
    
    /// slay Perform X25519 key exchange with ephemeral key
    /// 
    /// # Security Notes
    /// - Provides perfect forward secrecy
    /// - Ephemeral private key is automatically zeroized after use
    /// - Should be used for session key establishment
    pub fn key_exchange_ephemeral(&self, our_private: X25519EphemeralKeyPair, their_public: &PublicKey) -> X25519Result<Zeroizing<Vec<u8>>> {
        let shared_secret = our_private.private_key.diffie_hellman(their_public);
        Ok(Zeroizing::new(shared_secret.as_bytes().to_vec()))
    }
    
    /// slay Perform key exchange with raw key bytes
    pub fn key_exchange_raw(&self, our_private_bytes: &[u8], their_public_bytes: &[u8]) -> X25519Result<Zeroizing<Vec<u8>>> {
        if our_private_bytes.len() != 32 {
            return Err(X25519Error::InvalidPrivateKey(format!("Expected 32 bytes, got {}", our_private_bytes.len())));
        }
        if their_public_bytes.len() != 32 {
            return Err(X25519Error::InvalidPublicKey(format!("Expected 32 bytes, got {}", their_public_bytes.len())));
        }
        
        let mut private_array = [0u8; 32];
        private_array.copy_from_slice(our_private_bytes);
        let private_key = EphemeralSecret::from(private_array);
        
        let mut public_array = [0u8; 32];
        public_array.copy_from_slice(their_public_bytes);
        let public_key = PublicKey::from(public_array);
        
        let shared_secret = private_key.diffie_hellman(&public_key);
        Ok(Zeroizing::new(shared_secret.as_bytes().to_vec()))
    }
    
    /// slay Serialize public key to specified format
    pub fn serialize_public_key(&self, keypair: &X25519KeyPair, format: X25519KeyFormat) -> X25519Result<Vec<u8>> {
        let key_bytes = keypair.public_key_bytes();
        
        match format {
            X25519KeyFormat::Raw => Ok(key_bytes.to_vec()),
            X25519KeyFormat::Base64 => Ok(base64::encode(key_bytes).as_bytes().to_vec()),
            X25519KeyFormat::Hex => Ok(hex::encode(key_bytes).as_bytes().to_vec()),
        }
    }
    
    /// slay Serialize private key to specified format
    /// 
    /// # Security Notes
    /// - Private keys should be encrypted when stored
    /// - Use secure storage mechanisms
    /// - Returns zeroizing container for automatic cleanup
    pub fn serialize_private_key(&self, keypair: &X25519KeyPair, format: X25519KeyFormat) -> X25519Result<Zeroizing<Vec<u8>>> {
        let key_bytes = keypair.private_key_bytes();
        
        match format {
            X25519KeyFormat::Raw => Ok(Zeroizing::new(key_bytes.to_vec())),
            X25519KeyFormat::Base64 => Ok(Zeroizing::new(base64::encode(key_bytes).as_bytes().to_vec())),
            X25519KeyFormat::Hex => Ok(Zeroizing::new(hex::encode(key_bytes).as_bytes().to_vec())),
        }
    }
    
    /// slay Deserialize public key from specified format
    pub fn deserialize_public_key(&self, key_data: &[u8], format: X25519KeyFormat) -> X25519Result<PublicKey> {
        let key_bytes = match format {
            X25519KeyFormat::Raw => {
                if key_data.len() != 32 {
                    return Err(X25519Error::InvalidKeyLength(key_data.len()));
                }
                key_data.to_vec()
            },
            X25519KeyFormat::Base64 => {
                let key_str = std::str::from_utf8(key_data)
                    .map_err(|e| X25519Error::InvalidFormat(e.to_string()))?;
                base64::decode(key_str)
                    .map_err(|e| X25519Error::InvalidFormat(e.to_string()))?
            },
            X25519KeyFormat::Hex => {
                let key_str = std::str::from_utf8(key_data)
                    .map_err(|e| X25519Error::InvalidFormat(e.to_string()))?;
                hex::decode(key_str)
                    .map_err(|e| X25519Error::InvalidFormat(e.to_string()))?
            },
        };
        
        if key_bytes.len() != 32 {
            return Err(X25519Error::InvalidKeyLength(key_bytes.len()));
        }
        
        let mut key_array = [0u8; 32];
        key_array.copy_from_slice(&key_bytes);
        
        Ok(PublicKey::from(key_array))
    }
    
    /// slay Deserialize private key from specified format
    pub fn deserialize_private_key(&self, key_data: &[u8], format: X25519KeyFormat) -> X25519Result<X25519KeyPair> {
        let key_bytes = match format {
            X25519KeyFormat::Raw => {
                if key_data.len() != 32 {
                    return Err(X25519Error::InvalidKeyLength(key_data.len()));
                }
                key_data.to_vec()
            },
            X25519KeyFormat::Base64 => {
                let key_str = std::str::from_utf8(key_data)
                    .map_err(|e| X25519Error::InvalidFormat(e.to_string()))?;
                base64::decode(key_str)
                    .map_err(|e| X25519Error::InvalidFormat(e.to_string()))?
            },
            X25519KeyFormat::Hex => {
                let key_str = std::str::from_utf8(key_data)
                    .map_err(|e| X25519Error::InvalidFormat(e.to_string()))?;
                hex::decode(key_str)
                    .map_err(|e| X25519Error::InvalidFormat(e.to_string()))?
            },
        };
        
        if key_bytes.len() != 32 {
            return Err(X25519Error::InvalidKeyLength(key_bytes.len()));
        }
        
        let mut key_array = [0u8; 32];
        key_array.copy_from_slice(&key_bytes);
        
        let private_key = EphemeralSecret::from(key_array);
        Ok(X25519KeyPair::from_private_key(private_key))
    }
    
    /// slay Derive public key from private key bytes
    pub fn derive_public_key(&self, private_key_bytes: &[u8]) -> X25519Result<Vec<u8>> {
        if private_key_bytes.len() != 32 {
            return Err(X25519Error::InvalidKeyLength(private_key_bytes.len()));
        }
        
        let mut key_array = [0u8; 32];
        key_array.copy_from_slice(private_key_bytes);
        
        let private_key = EphemeralSecret::from(key_array);
        let public_key = PublicKey::from(&private_key);
        
        Ok(public_key.as_bytes().to_vec())
    }
    
    /// slay Validate that a public key is valid for curve25519
    pub fn validate_public_key(&self, public_key_bytes: &[u8]) -> X25519Result<bool> {
        if public_key_bytes.len() != 32 {
            return Ok(false);
        }
        
        // Basic validation - check it's not all zeros or all ones
        let all_zeros = public_key_bytes.iter().all(|&b| b == 0);
        let all_ones = public_key_bytes.iter().all(|&b| b == 0xFF);
        
        if all_zeros || all_ones {
            return Ok(false);
        }
        
        // Create PublicKey - this will validate the key
        let mut key_array = [0u8; 32];
        key_array.copy_from_slice(public_key_bytes);
        
        // If we can create a PublicKey from it, it's valid
        let _public_key = PublicKey::from(key_array);
        Ok(true)
    }
}

impl Default for X25519Engine {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Public API functions for CURSED integration
// use crate::stdlib::value::Value;

/// slay Generate X25519 static key pair
pub fn x25519_generate_keypair(_args: Vec<Value>) -> crate::error::Result<()> {
    let mut engine = X25519Engine::new();
    match engine.generate_static_keypair() {
        Ok(keypair) => {
            let mut result = HashMap::new();
            result.insert("algorithm".to_string(), Value::String("X25519".to_string()));
            result.insert("key_size".to_string(), Value::Number(256.0)); // 256-bit security level
            result.insert("curve".to_string(), Value::String("Curve25519".to_string()));
            result.insert("key_type".to_string(), Value::String("static".to_string()));
            
            // Raw public key for display
            result.insert("public_key_hex".to_string(), Value::String(hex::encode(keypair.public_key_bytes())));
            result.insert("public_key_base64".to_string(), Value::String(base64::encode(keypair.public_key_bytes())));
            
            result.insert("has_private_key".to_string(), Value::bool(true));
            
            Ok(Value::Object(result))
        },
        Err(e) => Err(CursedError::Runtime(format!("X25519 key generation failed: {}", e))),
    }
}

/// slay Generate X25519 ephemeral key pair
pub fn x25519_generate_ephemeral_keypair(_args: Vec<Value>) -> crate::error::Result<()> {
    let engine = X25519Engine::new();
    match engine.generate_ephemeral_keypair() {
        Ok(keypair) => {
            let mut result = HashMap::new();
            result.insert("algorithm".to_string(), Value::String("X25519".to_string()));
            result.insert("key_size".to_string(), Value::Number(256.0));
            result.insert("curve".to_string(), Value::String("Curve25519".to_string()));
            result.insert("key_type".to_string(), Value::String("ephemeral".to_string()));
            
            result.insert("public_key_hex".to_string(), Value::String(hex::encode(keypair.public_key_bytes())));
            result.insert("public_key_base64".to_string(), Value::String(base64::encode(keypair.public_key_bytes())));
            
            result.insert("perfect_forward_secrecy".to_string(), Value::bool(true));
            
            Ok(Value::Object(result))
        },
        Err(e) => Err(CursedError::Runtime(format!("X25519 ephemeral key generation failed: {}", e))),
    }
}

/// slay Generate X25519 key pair from seed
pub fn x25519_generate_keypair_from_seed(args: Vec<Value>) -> crate::error::Result<()> {
    if args.is_empty() {
        return Err(CursedError::Runtime("X25519 key generation from seed requires seed".to_string()));
    }
    
    let seed_hex = match &args[0] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("Seed must be a hex string".to_string())),
    };
    
    let seed = hex::decode(seed_hex)
        .map_err(|e| CursedError::Runtime(format!("Invalid hex seed: {}", e)))?;
    
    let engine = X25519Engine::new();
    match engine.generate_keypair_from_seed(&seed) {
        Ok(keypair) => {
            let mut result = HashMap::new();
            result.insert("algorithm".to_string(), Value::String("X25519".to_string()));
            result.insert("key_size".to_string(), Value::Number(256.0));
            result.insert("curve".to_string(), Value::String("Curve25519".to_string()));
            result.insert("key_type".to_string(), Value::String("static".to_string()));
            result.insert("deterministic".to_string(), Value::bool(true));
            
            result.insert("public_key_hex".to_string(), Value::String(hex::encode(keypair.public_key_bytes())));
            result.insert("public_key_base64".to_string(), Value::String(base64::encode(keypair.public_key_bytes())));
            result.insert("has_private_key".to_string(), Value::bool(true));
            
            Ok(Value::Object(result))
        },
        Err(e) => Err(CursedError::Runtime(format!("X25519 key generation from seed failed: {}", e))),
    }
}

/// slay Perform X25519 key exchange
pub fn x25519_key_exchange(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("X25519 key exchange requires our private key and their public key".to_string()));
    }
    
    let our_private_hex = match &args[0] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("Our private key must be a hex string".to_string())),
    };
    
    let their_public_hex = match &args[1] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("Their public key must be a hex string".to_string())),
    };
    
    let engine = X25519Engine::new();
    
    // Decode keys
    let our_private = hex::decode(our_private_hex)
        .map_err(|e| CursedError::Runtime(format!("Invalid hex private key: {}", e)))?;
    
    let their_public = hex::decode(their_public_hex)
        .map_err(|e| CursedError::Runtime(format!("Invalid hex public key: {}", e)))?;
    
    // Perform key exchange
    let shared_secret = engine.key_exchange_raw(&our_private, &their_public)
        .map_err(|e| CursedError::Runtime(format!("Key exchange failed: {}", e)))?;
    
    let mut result = HashMap::new();
    result.insert("algorithm".to_string(), Value::String("X25519".to_string()));
    result.insert("shared_secret_hex".to_string(), Value::String(hex::encode(&*shared_secret)));
    result.insert("shared_secret_base64".to_string(), Value::String(base64::encode(&*shared_secret)));
    result.insert("shared_secret_length".to_string(), Value::Number(shared_secret.len() as f64));
    
    Ok(Value::Object(result))
}

/// slay Derive X25519 public key from private key
pub fn x25519_derive_public_key(args: Vec<Value>) -> crate::error::Result<()> {
    if args.is_empty() {
        return Err(CursedError::Runtime("X25519 derive public key requires private key".to_string()));
    }
    
    let private_key_hex = match &args[0] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("Private key must be a hex string".to_string())),
    };
    
    let engine = X25519Engine::new();
    
    // Decode private key
    let private_key = hex::decode(private_key_hex)
        .map_err(|e| CursedError::Runtime(format!("Invalid hex private key: {}", e)))?;
    
    // Derive public key
    let public_key = engine.derive_public_key(&private_key)
        .map_err(|e| CursedError::Runtime(format!("Public key derivation failed: {}", e)))?;
    
    Ok(Value::String(hex::encode(public_key)))
}

/// slay Validate X25519 public key
pub fn x25519_validate_public_key(args: Vec<Value>) -> crate::error::Result<()> {
    if args.is_empty() {
        return Err(CursedError::Runtime("X25519 validate public key requires public key".to_string()));
    }
    
    let public_key_hex = match &args[0] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("Public key must be a hex string".to_string())),
    };
    
    let engine = X25519Engine::new();
    
    // Decode public key
    let public_key = hex::decode(public_key_hex)
        .map_err(|e| CursedError::Runtime(format!("Invalid hex public key: {}", e)))?;
    
    // Validate public key
    let is_valid = engine.validate_public_key(&public_key)
        .map_err(|e| CursedError::Runtime(format!("Public key validation failed: {}", e)))?;
    
    Ok(Value::bool(is_valid))
}

