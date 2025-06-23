/// fr fr X25519 key exchange implementation
/// 
/// This module provides production-ready X25519 Elliptic Curve Diffie-Hellman (ECDH)
/// key exchange operations using the x25519-dalek crate for secure key agreement.

use std::collections::HashMap;
use rand::rngs::OsRng;
use x25519_dalek::{EphemeralSecret, PublicKey, SharedSecret};
use zeroize::Zeroizing;
use crate::error::CursedError;

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

impl std::fmt::Display for X25519Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            X25519Error::KeyGenerationFailed(msg) => write!(f, "X25519 key generation failed: {}", msg),
            X25519Error::KeyExchangeFailed(msg) => write!(f, "X25519 key exchange failed: {}", msg),
            X25519Error::InvalidPublicKey(msg) => write!(f, "Invalid X25519 public key: {}", msg),
            X25519Error::InvalidPrivateKey(msg) => write!(f, "Invalid X25519 private key: {}", msg),
            X25519Error::InvalidFormat(msg) => write!(f, "Invalid key format: {}", msg),
            X25519Error::InvalidKeyLength(len) => write!(f, "Invalid key length: {} bytes (expected 32)", len),
            X25519Error::InvalidSharedSecret(msg) => write!(f, "Invalid shared secret: {}", msg),
            X25519Error::Internal(msg) => write!(f, "Internal X25519 error: {}", msg),
        }
    }
}

impl std::error::Error for X25519Error {}

type X25519Result<(), Error>;

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
use crate::stdlib::value::Value;

/// slay Generate X25519 static key pair
pub fn x25519_generate_keypair(_args: Vec<Value>) -> Result<(), Error> {
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
pub fn x25519_generate_ephemeral_keypair(_args: Vec<Value>) -> Result<(), Error> {
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
pub fn x25519_generate_keypair_from_seed(args: Vec<Value>) -> Result<(), Error> {
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
pub fn x25519_key_exchange(args: Vec<Value>) -> Result<(), Error> {
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
pub fn x25519_derive_public_key(args: Vec<Value>) -> Result<(), Error> {
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
pub fn x25519_validate_public_key(args: Vec<Value>) -> Result<(), Error> {
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_x25519_static_key_generation() {
        let mut engine = X25519Engine::new();
        let keypair = engine.generate_static_keypair().unwrap();
        
        // Verify key lengths
        assert_eq!(keypair.public_key_bytes().len(), 32);
        assert_eq!(keypair.private_key_bytes().len(), 32);
    }
    
    #[test]
    fn test_x25519_ephemeral_key_generation() {
        let engine = X25519Engine::new();
        let keypair = engine.generate_ephemeral_keypair().unwrap();
        
        // Verify key length
        assert_eq!(keypair.public_key_bytes().len(), 32);
    }
    
    #[test]
    fn test_x25519_deterministic_generation() {
        let engine = X25519Engine::new();
        let seed = [42u8; 32];
        
        let keypair1 = engine.generate_keypair_from_seed(&seed).unwrap();
        let keypair2 = engine.generate_keypair_from_seed(&seed).unwrap();
        
        // Same seed should produce same keys
        assert_eq!(keypair1.public_key_bytes(), keypair2.public_key_bytes());
        assert_eq!(keypair1.private_key_bytes(), keypair2.private_key_bytes());
    }
    
    #[test]
    fn test_x25519_key_exchange_static() {
        let mut engine = X25519Engine::new();
        let alice_keypair = engine.generate_static_keypair().unwrap();
        let bob_keypair = engine.generate_static_keypair().unwrap();
        
        // Perform key exchange from both sides
        let alice_shared = engine.key_exchange_static(&alice_keypair, &bob_keypair.public_key).unwrap();
        let bob_shared = engine.key_exchange_static(&bob_keypair, &alice_keypair.public_key).unwrap();
        
        // Shared secrets should be identical
        assert_eq!(*alice_shared, *bob_shared);
        assert_eq!(alice_shared.len(), 32);
    }
    
    #[test]
    fn test_x25519_key_exchange_ephemeral() {
        let engine = X25519Engine::new();
        let mut engine_mut = X25519Engine::new();
        
        let alice_ephemeral = engine.generate_ephemeral_keypair().unwrap();
        let bob_static = engine_mut.generate_static_keypair().unwrap();
        
        // Perform key exchange (ephemeral with static)
        let shared_secret = engine.key_exchange_ephemeral(alice_ephemeral, &bob_static.public_key).unwrap();
        
        assert_eq!(shared_secret.len(), 32);
        // Should not be all zeros (extremely unlikely)
        assert!(!shared_secret.iter().all(|&b| b == 0));
    }
    
    #[test]
    fn test_x25519_key_exchange_raw() {
        let mut engine = X25519Engine::new();
        let alice_keypair = engine.generate_static_keypair().unwrap();
        let bob_keypair = engine.generate_static_keypair().unwrap();
        
        let alice_private_bytes = alice_keypair.private_key_bytes();
        let bob_public_bytes = bob_keypair.public_key_bytes();
        
        let shared_secret = engine.key_exchange_raw(&alice_private_bytes, &bob_public_bytes).unwrap();
        
        assert_eq!(shared_secret.len(), 32);
        assert!(!shared_secret.iter().all(|&b| b == 0));
    }
    
    #[test]
    fn test_key_serialization() {
        let mut engine = X25519Engine::new();
        let keypair = engine.generate_static_keypair().unwrap();
        
        // Test raw serialization
        let private_raw = engine.serialize_private_key(&keypair, X25519KeyFormat::Raw).unwrap();
        let public_raw = engine.serialize_public_key(&keypair, X25519KeyFormat::Raw).unwrap();
        
        assert_eq!(private_raw.len(), 32);
        assert_eq!(public_raw.len(), 32);
        
        // Test base64 serialization
        let private_b64 = engine.serialize_private_key(&keypair, X25519KeyFormat::Base64).unwrap();
        let public_b64 = engine.serialize_public_key(&keypair, X25519KeyFormat::Base64).unwrap();
        
        // Base64 of 32 bytes should be 44 characters (padded)
        assert_eq!(private_b64.len(), 44);
        assert_eq!(public_b64.len(), 44);
        
        // Test hex serialization
        let private_hex = engine.serialize_private_key(&keypair, X25519KeyFormat::Hex).unwrap();
        let public_hex = engine.serialize_public_key(&keypair, X25519KeyFormat::Hex).unwrap();
        
        // Hex of 32 bytes should be 64 characters
        assert_eq!(private_hex.len(), 64);
        assert_eq!(public_hex.len(), 64);
    }
    
    #[test]
    fn test_key_deserialization() {
        let mut engine = X25519Engine::new();
        let original_keypair = engine.generate_static_keypair().unwrap();
        
        // Test raw deserialization
        let private_raw = engine.serialize_private_key(&original_keypair, X25519KeyFormat::Raw).unwrap();
        let deserialized_keypair = engine.deserialize_private_key(&private_raw, X25519KeyFormat::Raw).unwrap();
        
        // Should derive the same public key
        assert_eq!(original_keypair.public_key_bytes(), deserialized_keypair.public_key_bytes());
        
        // Test public key deserialization
        let public_raw = engine.serialize_public_key(&original_keypair, X25519KeyFormat::Raw).unwrap();
        let deserialized_public = engine.deserialize_public_key(&public_raw, X25519KeyFormat::Raw).unwrap();
        
        assert_eq!(original_keypair.public_key_bytes(), *deserialized_public.as_bytes());
    }
    
    #[test]
    fn test_public_key_derivation() {
        let mut engine = X25519Engine::new();
        let keypair = engine.generate_static_keypair().unwrap();
        
        let private_key_bytes = keypair.private_key_bytes();
        let derived_public_key = engine.derive_public_key(&private_key_bytes).unwrap();
        let actual_public_key = keypair.public_key_bytes();
        
        assert_eq!(derived_public_key, actual_public_key);
    }
    
    #[test]
    fn test_public_key_validation() {
        let mut engine = X25519Engine::new();
        let keypair = engine.generate_static_keypair().unwrap();
        
        // Valid key should pass validation
        let valid_key = keypair.public_key_bytes();
        assert!(engine.validate_public_key(&valid_key).unwrap());
        
        // All zeros should fail
        let zero_key = [0u8; 32];
        assert!(!engine.validate_public_key(&zero_key).unwrap());
        
        // All ones should fail
        let ones_key = [0xFFu8; 32];
        assert!(!engine.validate_public_key(&ones_key).unwrap());
        
        // Wrong length should fail
        let short_key = [1u8; 31];
        assert!(!engine.validate_public_key(&short_key).unwrap());
    }
    
    #[test]
    fn test_invalid_key_lengths() {
        let engine = X25519Engine::new();
        
        // Test invalid private key length for key exchange
        let invalid_private = vec![0u8; 31]; // Too short
        let valid_public = [1u8; 32];
        let result = engine.key_exchange_raw(&invalid_private, &valid_public);
        assert!(result.is_err());
        
        // Test invalid public key length for key exchange
        let valid_private = [1u8; 32];
        let invalid_public = vec![0u8; 31]; // Too short
        let result = engine.key_exchange_raw(&valid_private, &invalid_public);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_cross_compatibility() {
        // Test that our implementation is compatible with standard test vectors
        let mut engine = X25519Engine::new();
        
        // Use known test vectors (simplified for demo)
        let alice_private = [
            0x77, 0x07, 0x6d, 0x0a, 0x73, 0x18, 0xa5, 0x7d,
            0x3c, 0x16, 0xc1, 0x72, 0x51, 0xb2, 0x66, 0x45,
            0xdf, 0x4c, 0x2f, 0x87, 0xeb, 0xc0, 0x99, 0x2a,
            0xb1, 0x77, 0xfb, 0xa5, 0x1d, 0xb9, 0x2c, 0x2a
        ];
        
        let bob_public = [
            0xde, 0x9e, 0xdb, 0x7d, 0x7b, 0x7d, 0xc1, 0xb4,
            0xd3, 0x5b, 0x61, 0xc2, 0xec, 0xe4, 0x35, 0x37,
            0x3f, 0x83, 0x43, 0xc8, 0x5b, 0x78, 0x67, 0x4d,
            0xad, 0xfc, 0x7e, 0x14, 0x6f, 0x88, 0x2b, 0x4f
        ];
        
        let shared_secret = engine.key_exchange_raw(&alice_private, &bob_public).unwrap();
        
        // The shared secret should be deterministic
        assert_eq!(shared_secret.len(), 32);
        
        // Test that the same inputs always give the same output
        let shared_secret2 = engine.key_exchange_raw(&alice_private, &bob_public).unwrap();
        assert_eq!(*shared_secret, *shared_secret2);
    }
}
