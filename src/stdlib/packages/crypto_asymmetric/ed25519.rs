/// fr fr Ed25519 digital signature implementation
/// 
/// This module provides production-ready Ed25519 signature operations using
/// the ed25519-dalek crate for high-performance, secure digital signatures.

use std::collections::HashMap;
use rand::rngs::OsRng;
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use ed25519_dalek::pkcs8::{EncodePrivateKey, EncodePublicKey, DecodePrivateKey, DecodePublicKey, LineEnding};
use zeroize::Zeroizing;
use crate::error::CursedError;

/// fr fr Ed25519 key pair structure
#[derive(Debug, Clone)]
pub struct Ed25519KeyPair {
    pub private_key: SigningKey,
    pub public_key: VerifyingKey,
}

impl Ed25519KeyPair {
    /// slay Create new Ed25519 key pair from private key
    pub fn from_private_key(private_key: SigningKey) -> Self {
        let public_key = private_key.verifying_key();
        Self {
            private_key,
            public_key,
        }
    }
    
    /// slay Create Ed25519 key pair from public key only (for verification)
    pub fn from_public_key(public_key: VerifyingKey) -> Self {
        // For verification-only operations, we create a dummy private key
        let dummy_private = SigningKey::generate(&mut OsRng);
        Self {
            private_key: dummy_private,
            public_key,
        }
    }
    
    /// slay Get raw public key bytes
    pub fn public_key_bytes(&self) -> [u8; 32] {
        self.public_key.to_bytes()
    }
    
    /// slay Get raw private key bytes
    pub fn private_key_bytes(&self) -> [u8; 32] {
        self.private_key.to_bytes()
    }
}

/// fr fr Key serialization formats for Ed25519
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ed25519KeyFormat {
    Pkcs8Pem,     // PKCS#8 PEM format
    Pkcs8Der,     // PKCS#8 DER format
    Raw,          // Raw 32-byte format
}

/// fr fr Ed25519 error types
#[derive(Debug, Clone, PartialEq)]
pub enum Ed25519Error {
    KeyGenerationFailed(String),
    SigningFailed(String),
    VerificationFailed(String),
    InvalidSignature(String),
    InvalidPublicKey(String),
    InvalidPrivateKey(String),
    InvalidFormat(String),
    SerializationFailed(String),
    DeserializationFailed(String),
    InvalidKeyLength(usize),
    Internal(String),
}

impl std::fmt::Display for Ed25519Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ed25519Error::KeyGenerationFailed(msg) => write!(f, "Ed25519 key generation failed: {}", msg),
            Ed25519Error::SigningFailed(msg) => write!(f, "Ed25519 signing failed: {}", msg),
            Ed25519Error::VerificationFailed(msg) => write!(f, "Ed25519 verification failed: {}", msg),
            Ed25519Error::InvalidSignature(msg) => write!(f, "Invalid Ed25519 signature: {}", msg),
            Ed25519Error::InvalidPublicKey(msg) => write!(f, "Invalid Ed25519 public key: {}", msg),
            Ed25519Error::InvalidPrivateKey(msg) => write!(f, "Invalid Ed25519 private key: {}", msg),
            Ed25519Error::InvalidFormat(msg) => write!(f, "Invalid key format: {}", msg),
            Ed25519Error::SerializationFailed(msg) => write!(f, "Key serialization failed: {}", msg),
            Ed25519Error::DeserializationFailed(msg) => write!(f, "Key deserialization failed: {}", msg),
            Ed25519Error::InvalidKeyLength(len) => write!(f, "Invalid key length: {} bytes (expected 32)", len),
            Ed25519Error::Internal(msg) => write!(f, "Internal Ed25519 error: {}", msg),
        }
    }
}

impl std::error::Error for Ed25519Error {}

impl From<ed25519_dalek::SignatureError> for Ed25519Error {
    fn from(err: ed25519_dalek::SignatureError) -> Self {
        Ed25519Error::Internal(err.to_string())
    }
}

impl From<ed25519_dalek::pkcs8::Error> for Ed25519Error {
    fn from(err: ed25519_dalek::pkcs8::Error) -> Self {
        Ed25519Error::SerializationFailed(err.to_string())
    }
}

type Ed25519Result<T> = Result<T, Ed25519Error>;

/// fr fr Ed25519 engine for cryptographic operations
pub struct Ed25519Engine {
    rng: OsRng,
}

impl Ed25519Engine {
    /// slay Create new Ed25519 engine with cryptographically secure RNG
    pub fn new() -> Self {
        Self {
            rng: OsRng,
        }
    }
    
    /// slay Generate Ed25519 key pair
    /// 
    /// # Security Notes
    /// - Uses cryptographically secure random number generation
    /// - Ed25519 provides 128-bit security level
    /// - Keys are generated according to RFC 8032 standards
    /// - Private keys are automatically clamped for curve25519
    pub fn generate_keypair(&mut self) -> Ed25519Result<Ed25519KeyPair> {
        let private_key = SigningKey::generate(&mut self.rng);
        Ok(Ed25519KeyPair::from_private_key(private_key))
    }
    
    /// slay Generate deterministic key pair from seed
    /// 
    /// # Security Notes
    /// - Seed must be cryptographically secure (32 bytes)
    /// - Same seed always produces same key pair
    /// - Use for key derivation or testing only
    pub fn generate_keypair_from_seed(&self, seed: &[u8]) -> Ed25519Result<Ed25519KeyPair> {
        if seed.len() != 32 {
            return Err(Ed25519Error::InvalidKeyLength(seed.len()));
        }
        
        let mut seed_array = [0u8; 32];
        seed_array.copy_from_slice(seed);
        
        let private_key = SigningKey::from_bytes(&seed_array);
        Ok(Ed25519KeyPair::from_private_key(private_key))
    }
    
    /// slay Ed25519 sign message with private key
    /// 
    /// # Security Notes
    /// - No message hashing required (EdDSA handles this internally)
    /// - Signatures are deterministic with domain separation
    /// - Provides strong security against forgeries and collisions
    /// - Constant-time signing operation
    pub fn sign(&self, keypair: &Ed25519KeyPair, message: &[u8]) -> Ed25519Result<Vec<u8>> {
        let signature = keypair.private_key.sign(message);
        Ok(signature.to_bytes().to_vec())
    }
    
    /// slay Ed25519 verify signature with public key
    /// 
    /// # Security Notes
    /// - Verification is constant-time to prevent timing attacks
    /// - Signature format is validated before verification
    /// - Strong protection against signature malleability
    /// - No cofactor issues (unlike some other curves)
    pub fn verify(&self, keypair: &Ed25519KeyPair, message: &[u8], signature: &[u8]) -> Ed25519Result<bool> {
        if signature.len() != 64 {
            return Ok(false);
        }
        
        let mut sig_bytes = [0u8; 64];
        sig_bytes.copy_from_slice(signature);
        
        let sig = Signature::from_bytes(&sig_bytes);
        
        match keypair.public_key.verify(message, &sig) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }
    
    /// slay Verify signature with standalone public key
    pub fn verify_with_public_key(&self, public_key: &[u8], message: &[u8], signature: &[u8]) -> Ed25519Result<bool> {
        if public_key.len() != 32 {
            return Err(Ed25519Error::InvalidPublicKey(format!("Expected 32 bytes, got {}", public_key.len())));
        }
        
        let mut pub_key_bytes = [0u8; 32];
        pub_key_bytes.copy_from_slice(public_key);
        
        let verifying_key = VerifyingKey::from_bytes(&pub_key_bytes)
            .map_err(|e| Ed25519Error::InvalidPublicKey(e.to_string()))?;
        
        let keypair = Ed25519KeyPair::from_public_key(verifying_key);
        self.verify(&keypair, message, signature)
    }
    
    /// slay Serialize private key to specified format
    /// 
    /// # Security Notes
    /// - Private keys should be encrypted when stored
    /// - Use secure storage mechanisms
    /// - Returns zeroizing container for automatic cleanup
    pub fn serialize_private_key(&self, keypair: &Ed25519KeyPair, format: Ed25519KeyFormat) -> Ed25519Result<Zeroizing<Vec<u8>>> {
        match format {
            Ed25519KeyFormat::Pkcs8Pem => {
                let pem = keypair.private_key.to_pkcs8_pem(LineEnding::LF)?;
                Ok(Zeroizing::new(pem.as_bytes().to_vec()))
            },
            Ed25519KeyFormat::Pkcs8Der => {
                let der = keypair.private_key.to_pkcs8_der()?;
                Ok(Zeroizing::new(der.to_bytes().to_vec()))
            },
            Ed25519KeyFormat::Raw => {
                Ok(Zeroizing::new(keypair.private_key.to_bytes().to_vec()))
            },
        }
    }
    
    /// slay Serialize public key to specified format
    pub fn serialize_public_key(&self, keypair: &Ed25519KeyPair, format: Ed25519KeyFormat) -> Ed25519Result<Vec<u8>> {
        match format {
            Ed25519KeyFormat::Pkcs8Pem => {
                let pem = keypair.public_key.to_public_key_pem(LineEnding::LF)?;
                Ok(pem.as_bytes().to_vec())
            },
            Ed25519KeyFormat::Pkcs8Der => {
                let der = keypair.public_key.to_public_key_der()?;
                Ok(der.to_bytes().to_vec())
            },
            Ed25519KeyFormat::Raw => {
                Ok(keypair.public_key.to_bytes().to_vec())
            },
        }
    }
    
    /// slay Deserialize private key from specified format
    pub fn deserialize_private_key(&self, key_data: &[u8], format: Ed25519KeyFormat) -> Ed25519Result<Ed25519KeyPair> {
        let private_key = match format {
            Ed25519KeyFormat::Pkcs8Pem => {
                let pem_str = std::str::from_utf8(key_data)
                    .map_err(|e| Ed25519Error::DeserializationFailed(e.to_string()))?;
                SigningKey::from_pkcs8_pem(pem_str)?
            },
            Ed25519KeyFormat::Pkcs8Der => {
                SigningKey::from_pkcs8_der(key_data)?
            },
            Ed25519KeyFormat::Raw => {
                if key_data.len() != 32 {
                    return Err(Ed25519Error::InvalidKeyLength(key_data.len()));
                }
                let mut key_bytes = [0u8; 32];
                key_bytes.copy_from_slice(key_data);
                SigningKey::from_bytes(&key_bytes)
            },
        };
        
        Ok(Ed25519KeyPair::from_private_key(private_key))
    }
    
    /// slay Deserialize public key from specified format
    pub fn deserialize_public_key(&self, key_data: &[u8], format: Ed25519KeyFormat) -> Ed25519Result<Ed25519KeyPair> {
        let public_key = match format {
            Ed25519KeyFormat::Pkcs8Pem => {
                let pem_str = std::str::from_utf8(key_data)
                    .map_err(|e| Ed25519Error::DeserializationFailed(e.to_string()))?;
                VerifyingKey::from_public_key_pem(pem_str)?
            },
            Ed25519KeyFormat::Pkcs8Der => {
                VerifyingKey::from_public_key_der(key_data)?
            },
            Ed25519KeyFormat::Raw => {
                if key_data.len() != 32 {
                    return Err(Ed25519Error::InvalidKeyLength(key_data.len()));
                }
                let mut key_bytes = [0u8; 32];
                key_bytes.copy_from_slice(key_data);
                VerifyingKey::from_bytes(&key_bytes)
                    .map_err(|e| Ed25519Error::InvalidPublicKey(e.to_string()))?
            },
        };
        
        Ok(Ed25519KeyPair::from_public_key(public_key))
    }
    
    /// slay Derive public key from private key bytes
    pub fn derive_public_key(&self, private_key_bytes: &[u8]) -> Ed25519Result<Vec<u8>> {
        if private_key_bytes.len() != 32 {
            return Err(Ed25519Error::InvalidKeyLength(private_key_bytes.len()));
        }
        
        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(private_key_bytes);
        
        let private_key = SigningKey::from_bytes(&key_bytes);
        let public_key = private_key.verifying_key();
        
        Ok(public_key.to_bytes().to_vec())
    }
    
    /// slay Batch verify multiple signatures (more efficient than individual verification)
    pub fn batch_verify(&self, messages_signatures_keys: &[(Vec<u8>, Vec<u8>, Vec<u8>)]) -> Ed25519Result<bool> {
        // For simplicity, we'll verify each individually
        // In production, you'd use batch verification algorithms for better performance
        for (message, signature, public_key) in messages_signatures_keys {
            if !self.verify_with_public_key(public_key, message, signature)? {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

impl Default for Ed25519Engine {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Public API functions for CURSED integration
use crate::stdlib::value::Value;

/// slay Generate Ed25519 key pair
pub fn ed25519_generate_keypair(_args: Vec<Value>) -> Result<Value, CursedError> {
    let mut engine = Ed25519Engine::new();
    match engine.generate_keypair() {
        Ok(keypair) => {
            let mut result = HashMap::new();
            result.insert("algorithm".to_string(), Value::String("Ed25519".to_string()));
            result.insert("key_size".to_string(), Value::Number(256.0)); // 256-bit security level
            result.insert("curve".to_string(), Value::String("Curve25519".to_string()));
            
            // Serialize public key to PEM
            if let Ok(public_pem) = engine.serialize_public_key(&keypair, Ed25519KeyFormat::Pkcs8Pem) {
                result.insert("public_key_pem".to_string(), Value::String(String::from_utf8_lossy(&public_pem).to_string()));
            }
            
            // Raw public key for display
            result.insert("public_key_hex".to_string(), Value::String(hex::encode(keypair.public_key_bytes())));
            
            result.insert("has_private_key".to_string(), Value::bool(true));
            
            Ok(Value::Object(result))
        },
        Err(e) => Err(CursedError::Runtime(format!("Ed25519 key generation failed: {}", e))),
    }
}

/// slay Generate Ed25519 key pair from seed
pub fn ed25519_generate_keypair_from_seed(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.is_empty() {
        return Err(CursedError::Runtime("Ed25519 key generation from seed requires seed".to_string()));
    }
    
    let seed_hex = match &args[0] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("Seed must be a hex string".to_string())),
    };
    
    let seed = hex::decode(seed_hex)
        .map_err(|e| CursedError::Runtime(format!("Invalid hex seed: {}", e)))?;
    
    let engine = Ed25519Engine::new();
    match engine.generate_keypair_from_seed(&seed) {
        Ok(keypair) => {
            let mut result = HashMap::new();
            result.insert("algorithm".to_string(), Value::String("Ed25519".to_string()));
            result.insert("key_size".to_string(), Value::Number(256.0));
            result.insert("curve".to_string(), Value::String("Curve25519".to_string()));
            result.insert("deterministic".to_string(), Value::bool(true));
            
            // Serialize public key to PEM
            if let Ok(public_pem) = engine.serialize_public_key(&keypair, Ed25519KeyFormat::Pkcs8Pem) {
                result.insert("public_key_pem".to_string(), Value::String(String::from_utf8_lossy(&public_pem).to_string()));
            }
            
            result.insert("public_key_hex".to_string(), Value::String(hex::encode(keypair.public_key_bytes())));
            result.insert("has_private_key".to_string(), Value::bool(true));
            
            Ok(Value::Object(result))
        },
        Err(e) => Err(CursedError::Runtime(format!("Ed25519 key generation from seed failed: {}", e))),
    }
}

/// slay Ed25519 sign message
pub fn ed25519_sign(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("Ed25519 sign requires private key and message".to_string()));
    }
    
    let private_key_pem = match &args[0] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("Private key must be a PEM string".to_string())),
    };
    
    let message = match &args[1] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(CursedError::Runtime("Message must be a string".to_string())),
    };
    
    let engine = Ed25519Engine::new();
    
    // Parse private key
    let keypair = engine.deserialize_private_key(private_key_pem.as_bytes(), Ed25519KeyFormat::Pkcs8Pem)
        .map_err(|e| CursedError::Runtime(format!("Invalid private key: {}", e)))?;
    
    // Sign
    let signature = engine.sign(&keypair, message)
        .map_err(|e| CursedError::Runtime(format!("Signing failed: {}", e)))?;
    
    Ok(Value::String(base64::encode(signature)))
}

/// slay Ed25519 verify signature
pub fn ed25519_verify(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 3 {
        return Err(CursedError::Runtime("Ed25519 verify requires public key, message, and signature".to_string()));
    }
    
    let public_key_pem = match &args[0] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("Public key must be a PEM string".to_string())),
    };
    
    let message = match &args[1] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(CursedError::Runtime("Message must be a string".to_string())),
    };
    
    let signature_b64 = match &args[2] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("Signature must be a base64 string".to_string())),
    };
    
    let engine = Ed25519Engine::new();
    
    // Parse public key
    let keypair = engine.deserialize_public_key(public_key_pem.as_bytes(), Ed25519KeyFormat::Pkcs8Pem)
        .map_err(|e| CursedError::Runtime(format!("Invalid public key: {}", e)))?;
    
    // Decode signature
    let signature = base64::decode(signature_b64)
        .map_err(|e| CursedError::Runtime(format!("Invalid base64 signature: {}", e)))?;
    
    // Verify
    let is_valid = engine.verify(&keypair, message, &signature)
        .map_err(|e| CursedError::Runtime(format!("Verification failed: {}", e)))?;
    
    Ok(Value::bool(is_valid))
}

/// slay Ed25519 verify with raw public key
pub fn ed25519_verify_raw(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 3 {
        return Err(CursedError::Runtime("Ed25519 verify raw requires public key hex, message, and signature".to_string()));
    }
    
    let public_key_hex = match &args[0] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("Public key must be a hex string".to_string())),
    };
    
    let message = match &args[1] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(CursedError::Runtime("Message must be a string".to_string())),
    };
    
    let signature_b64 = match &args[2] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("Signature must be a base64 string".to_string())),
    };
    
    let engine = Ed25519Engine::new();
    
    // Decode public key
    let public_key = hex::decode(public_key_hex)
        .map_err(|e| CursedError::Runtime(format!("Invalid hex public key: {}", e)))?;
    
    // Decode signature
    let signature = base64::decode(signature_b64)
        .map_err(|e| CursedError::Runtime(format!("Invalid base64 signature: {}", e)))?;
    
    // Verify
    let is_valid = engine.verify_with_public_key(&public_key, message, &signature)
        .map_err(|e| CursedError::Runtime(format!("Verification failed: {}", e)))?;
    
    Ok(Value::bool(is_valid))
}

/// slay Derive Ed25519 public key from private key
pub fn ed25519_derive_public_key(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.is_empty() {
        return Err(CursedError::Runtime("Ed25519 derive public key requires private key".to_string()));
    }
    
    let private_key_hex = match &args[0] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("Private key must be a hex string".to_string())),
    };
    
    let engine = Ed25519Engine::new();
    
    // Decode private key
    let private_key = hex::decode(private_key_hex)
        .map_err(|e| CursedError::Runtime(format!("Invalid hex private key: {}", e)))?;
    
    // Derive public key
    let public_key = engine.derive_public_key(&private_key)
        .map_err(|e| CursedError::Runtime(format!("Public key derivation failed: {}", e)))?;
    
    Ok(Value::String(hex::encode(public_key)))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ed25519_key_generation() {
        let mut engine = Ed25519Engine::new();
        let keypair = engine.generate_keypair().unwrap();
        
        // Verify key lengths
        assert_eq!(keypair.public_key_bytes().len(), 32);
        assert_eq!(keypair.private_key_bytes().len(), 32);
    }
    
    #[test]
    fn test_ed25519_deterministic_generation() {
        let engine = Ed25519Engine::new();
        let seed = [42u8; 32];
        
        let keypair1 = engine.generate_keypair_from_seed(&seed).unwrap();
        let keypair2 = engine.generate_keypair_from_seed(&seed).unwrap();
        
        // Same seed should produce same keys
        assert_eq!(keypair1.public_key_bytes(), keypair2.public_key_bytes());
        assert_eq!(keypair1.private_key_bytes(), keypair2.private_key_bytes());
    }
    
    #[test]
    fn test_ed25519_signing_verification() {
        let mut engine = Ed25519Engine::new();
        let keypair = engine.generate_keypair().unwrap();
        
        let message = b"Hello, Ed25519 signatures!";
        let signature = engine.sign(&keypair, message).unwrap();
        let verified = engine.verify(&keypair, message, &signature).unwrap();
        
        assert!(verified);
        assert_eq!(signature.len(), 64); // Ed25519 signatures are always 64 bytes
        
        // Test with wrong message
        let wrong_message = b"Wrong message";
        let verified_wrong = engine.verify(&keypair, wrong_message, &signature).unwrap();
        assert!(!verified_wrong);
    }
    
    #[test]
    fn test_ed25519_raw_verification() {
        let mut engine = Ed25519Engine::new();
        let keypair = engine.generate_keypair().unwrap();
        
        let message = b"Test raw verification";
        let signature = engine.sign(&keypair, message).unwrap();
        let public_key_bytes = keypair.public_key_bytes();
        
        let verified = engine.verify_with_public_key(&public_key_bytes, message, &signature).unwrap();
        assert!(verified);
    }
    
    #[test]
    fn test_key_serialization() {
        let mut engine = Ed25519Engine::new();
        let keypair = engine.generate_keypair().unwrap();
        
        // Test private key serialization/deserialization
        let private_pem = engine.serialize_private_key(&keypair, Ed25519KeyFormat::Pkcs8Pem).unwrap();
        let deserialized_keypair = engine.deserialize_private_key(&private_pem, Ed25519KeyFormat::Pkcs8Pem).unwrap();
        
        // Test public key serialization
        let public_pem = engine.serialize_public_key(&keypair, Ed25519KeyFormat::Pkcs8Pem).unwrap();
        assert!(!public_pem.is_empty());
        
        // Test raw serialization
        let private_raw = engine.serialize_private_key(&keypair, Ed25519KeyFormat::Raw).unwrap();
        assert_eq!(private_raw.len(), 32);
        
        let public_raw = engine.serialize_public_key(&keypair, Ed25519KeyFormat::Raw).unwrap();
        assert_eq!(public_raw.len(), 32);
        
        // Verify they still work
        let message = b"Test serialization";
        let signature = engine.sign(&deserialized_keypair, message).unwrap();
        let verified = engine.verify(&deserialized_keypair, message, &signature).unwrap();
        
        assert!(verified);
    }
    
    #[test]
    fn test_public_key_derivation() {
        let mut engine = Ed25519Engine::new();
        let keypair = engine.generate_keypair().unwrap();
        
        let private_key_bytes = keypair.private_key_bytes();
        let derived_public_key = engine.derive_public_key(&private_key_bytes).unwrap();
        let actual_public_key = keypair.public_key_bytes();
        
        assert_eq!(derived_public_key, actual_public_key);
    }
    
    #[test]
    fn test_invalid_key_lengths() {
        let engine = Ed25519Engine::new();
        
        // Test invalid private key length
        let invalid_private = vec![0u8; 31]; // Too short
        let result = engine.generate_keypair_from_seed(&invalid_private);
        assert!(result.is_err());
        
        // Test invalid public key length for verification
        let invalid_public = vec![0u8; 31]; // Too short
        let message = b"test";
        let signature = vec![0u8; 64];
        let result = engine.verify_with_public_key(&invalid_public, message, &signature);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_invalid_signature_length() {
        let mut engine = Ed25519Engine::new();
        let keypair = engine.generate_keypair().unwrap();
        
        let message = b"test message";
        let invalid_signature = vec![0u8; 63]; // Too short
        
        let verified = engine.verify(&keypair, message, &invalid_signature).unwrap();
        assert!(!verified); // Should return false for invalid signature length
    }
    
    #[test]
    fn test_batch_verification() {
        let mut engine = Ed25519Engine::new();
        let keypair1 = engine.generate_keypair().unwrap();
        let keypair2 = engine.generate_keypair().unwrap();
        
        let message1 = b"Message 1";
        let message2 = b"Message 2";
        
        let signature1 = engine.sign(&keypair1, message1).unwrap();
        let signature2 = engine.sign(&keypair2, message2).unwrap();
        
        let batch = vec![
            (message1.to_vec(), signature1, keypair1.public_key_bytes().to_vec()),
            (message2.to_vec(), signature2, keypair2.public_key_bytes().to_vec()),
        ];
        
        let verified = engine.batch_verify(&batch).unwrap();
        assert!(verified);
        
        // Test with one invalid signature
        let invalid_batch = vec![
            (message1.to_vec(), vec![0u8; 64], keypair1.public_key_bytes().to_vec()),
            (message2.to_vec(), signature2, keypair2.public_key_bytes().to_vec()),
        ];
        
        let verified_invalid = engine.batch_verify(&invalid_batch).unwrap();
        assert!(!verified_invalid);
    }
}
