/// fr fr RSA implementation with real cryptographic operations
/// 
/// This module provides production-ready RSA key generation, encryption, decryption,
/// and digital signature operations following PKCS#1 standards using the `rsa` crate.

use std::collections::HashMap;
use rand::rngs::OsRng;
use rsa::{RsaPrivateKey, RsaPublicKey, Pkcs1v15Encrypt, Pkcs1v15Sign, Oaep, Pss};
use rsa::pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey, DecodeRsaPrivateKey, DecodeRsaPublicKey};
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey, DecodePrivateKey, DecodePublicKey};
use rsa::signature::{RandomizedSigner, Verifier, SignatureEncoding};
use sha2::{Sha256, Sha384, Sha512, Digest};
use zeroize::Zeroizing;
use crate::error::CursedError;

/// fr fr RSA key sizes in bits
pub const RSA_2048_BITS: usize = 2048;
pub const RSA_3072_BITS: usize = 3072;
pub const RSA_4096_BITS: usize = 4096;

/// fr fr RSA key pair structure
#[derive(Debug, Clone)]
pub struct CursedRsaKeyPair {
    pub public_key: RsaPublicKey,
    pub private_key: RsaPrivateKey,
    pub key_size: usize,
}

/// fr fr Padding schemes for RSA operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RsaPadding {
    Pkcs1v15,      // PKCS#1 v1.5 padding
    OaepSha256,    // OAEP with SHA-256
    OaepSha384,    // OAEP with SHA-384
    OaepSha512,    // OAEP with SHA-512
    PssSha256,     // PSS with SHA-256 for signatures
    PssSha384,     // PSS with SHA-384 for signatures
    PssSha512,     // PSS with SHA-512 for signatures
}

impl RsaPadding {
    pub fn name(&self) -> &'static str {
        match self {
            RsaPadding::Pkcs1v15 => "PKCS1v15",
            RsaPadding::OaepSha256 => "OAEP-SHA256",
            RsaPadding::OaepSha384 => "OAEP-SHA384",
            RsaPadding::OaepSha512 => "OAEP-SHA512",
            RsaPadding::PssSha256 => "PSS-SHA256",
            RsaPadding::PssSha384 => "PSS-SHA384",
            RsaPadding::PssSha512 => "PSS-SHA512",
        }
    }
}

/// fr fr Key serialization formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyFormat {
    Pkcs1Pem,      // PKCS#1 PEM format
    Pkcs1Der,      // PKCS#1 DER format
    Pkcs8Pem,      // PKCS#8 PEM format
    Pkcs8Der,      // PKCS#8 DER format
}

/// fr fr RSA error types
#[derive(Debug, Clone, PartialEq)]
pub enum RsaError {
    InvalidKeySize(usize),
    KeyGenerationFailed(String),
    EncryptionFailed(String),
    DecryptionFailed(String),
    SigningFailed(String),
    VerificationFailed(String),
    InvalidPadding(String),
    InvalidInput(String),
    InvalidFormat(String),
    SerializationFailed(String),
    DeserializationFailed(String),
    InsufficientEntropy,
    Internal(String),
}

impl std::fmt::Display for RsaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RsaError::InvalidKeySize(size) => write!(f, "Invalid RSA key size: {} bits (minimum 2048)", size),
            RsaError::KeyGenerationFailed(msg) => write!(f, "RSA key generation failed: {}", msg),
            RsaError::EncryptionFailed(msg) => write!(f, "RSA encryption failed: {}", msg),
            RsaError::DecryptionFailed(msg) => write!(f, "RSA decryption failed: {}", msg),
            RsaError::SigningFailed(msg) => write!(f, "RSA signing failed: {}", msg),
            RsaError::VerificationFailed(msg) => write!(f, "RSA verification failed: {}", msg),
            RsaError::InvalidPadding(msg) => write!(f, "Invalid RSA padding: {}", msg),
            RsaError::InvalidInput(msg) => write!(f, "Invalid RSA input: {}", msg),
            RsaError::InvalidFormat(msg) => write!(f, "Invalid key format: {}", msg),
            RsaError::SerializationFailed(msg) => write!(f, "Key serialization failed: {}", msg),
            RsaError::DeserializationFailed(msg) => write!(f, "Key deserialization failed: {}", msg),
            RsaError::InsufficientEntropy => write!(f, "Insufficient entropy for RSA operation"),
            RsaError::Internal(msg) => write!(f, "Internal RSA error: {}", msg),
        }
    }
}

impl std::error::Error for RsaError {}

impl From<rsa::Error> for RsaError {
    fn from(err: rsa::Error) -> Self {
        RsaError::Internal(err.to_string())
    }
}

impl From<rsa::pkcs1::Error> for RsaError {
    fn from(err: rsa::pkcs1::Error) -> Self {
        RsaError::SerializationFailed(err.to_string())
    }
}

impl From<rsa::pkcs8::Error> for RsaError {
    fn from(err: rsa::pkcs8::Error) -> Self {
        RsaError::SerializationFailed(err.to_string())
    }
}

type RsaResult<T> = Result<T, RsaError>;

/// fr fr RSA implementation with real cryptographic operations
pub struct RsaEngine {
    rng: OsRng,
}

impl RsaEngine {
    /// slay Create new RSA engine with cryptographically secure RNG
    pub fn new() -> Self {
        Self {
            rng: OsRng,
        }
    }
    
    /// slay Generate RSA key pair with specified bit length
    /// 
    /// # Security Notes
    /// - Minimum key size is 2048 bits for security
    /// - Uses cryptographically secure random number generation
    /// - Keys are generated with proper prime selection
    pub fn generate_keypair(&mut self, key_size: usize) -> RsaResult<CursedRsaKeyPair> {
        // Validate key size - enforce minimum security standards
        if key_size < RSA_2048_BITS {
            return Err(RsaError::InvalidKeySize(key_size));
        }
        
        match key_size {
            RSA_2048_BITS | RSA_3072_BITS | RSA_4096_BITS => {},
            _ => return Err(RsaError::InvalidKeySize(key_size)),
        }
        
        // Generate RSA key pair using cryptographically secure methods
        let private_key = RsaPrivateKey::new(&mut self.rng, key_size)
            .map_err(|e| RsaError::KeyGenerationFailed(e.to_string()))?;
        
        let public_key = private_key.to_public_key();
        
        // Validate key generation
        self.validate_keypair(&public_key, &private_key)?;
        
        Ok(CursedRsaKeyPair {
            public_key,
            private_key,
            key_size,
        })
    }
    
    /// slay RSA encrypt with public key using specified padding
    /// 
    /// # Security Notes
    /// - Always use OAEP padding for new applications
    /// - PKCS#1 v1.5 included for compatibility but discouraged
    /// - Input size automatically validated against key size and padding overhead
    pub fn encrypt(&self, public_key: &RsaPublicKey, plaintext: &[u8], padding: RsaPadding) -> RsaResult<Vec<u8>> {
        // Validate input size based on padding scheme
        self.validate_encryption_input(public_key, plaintext, padding)?;
        
        let mut rng = OsRng;
        
        match padding {
            RsaPadding::Pkcs1v15 => {
                public_key.encrypt(&mut rng, Pkcs1v15Encrypt, plaintext)
                    .map_err(|e| RsaError::EncryptionFailed(e.to_string()))
            },
            RsaPadding::OaepSha256 => {
                let padding = Oaep::new::<Sha256>();
                public_key.encrypt(&mut rng, padding, plaintext)
                    .map_err(|e| RsaError::EncryptionFailed(e.to_string()))
            },
            RsaPadding::OaepSha384 => {
                let padding = Oaep::new::<Sha384>();
                public_key.encrypt(&mut rng, padding, plaintext)
                    .map_err(|e| RsaError::EncryptionFailed(e.to_string()))
            },
            RsaPadding::OaepSha512 => {
                let padding = Oaep::new::<Sha512>();
                public_key.encrypt(&mut rng, padding, plaintext)
                    .map_err(|e| RsaError::EncryptionFailed(e.to_string()))
            },
            _ => Err(RsaError::InvalidPadding(format!("Padding {} not supported for encryption", padding.name()))),
        }
    }
    
    /// slay RSA decrypt with private key using specified padding
    /// 
    /// # Security Notes
    /// - Uses constant-time operations where possible
    /// - Private key operations are protected against timing attacks
    /// - Padding validation prevents padding oracle attacks
    pub fn decrypt(&self, private_key: &RsaPrivateKey, ciphertext: &[u8], padding: RsaPadding) -> RsaResult<Vec<u8>> {
        // Validate ciphertext size
        let expected_size = private_key.size();
        if ciphertext.len() != expected_size {
            return Err(RsaError::DecryptionFailed(format!(
                "Invalid ciphertext size: expected {} bytes, got {}", 
                expected_size, ciphertext.len()
            )));
        }
        
        match padding {
            RsaPadding::Pkcs1v15 => {
                private_key.decrypt(Pkcs1v15Encrypt, ciphertext)
                    .map_err(|e| RsaError::DecryptionFailed(e.to_string()))
            },
            RsaPadding::OaepSha256 => {
                let padding = Oaep::new::<Sha256>();
                private_key.decrypt(padding, ciphertext)
                    .map_err(|e| RsaError::DecryptionFailed(e.to_string()))
            },
            RsaPadding::OaepSha384 => {
                let padding = Oaep::new::<Sha384>();
                private_key.decrypt(padding, ciphertext)
                    .map_err(|e| RsaError::DecryptionFailed(e.to_string()))
            },
            RsaPadding::OaepSha512 => {
                let padding = Oaep::new::<Sha512>();
                private_key.decrypt(padding, ciphertext)
                    .map_err(|e| RsaError::DecryptionFailed(e.to_string()))
            },
            _ => Err(RsaError::InvalidPadding(format!("Padding {} not supported for decryption", padding.name()))),
        }
    }
    
    /// slay RSA sign with private key using specified padding and hash
    /// 
    /// # Security Notes
    /// - PSS padding recommended for new applications
    /// - PKCS#1 v1.5 included for compatibility
    /// - Message is automatically hashed before signing
    /// - Uses secure random salt for PSS signatures
    pub fn sign(&self, private_key: &RsaPrivateKey, message: &[u8], padding: RsaPadding) -> RsaResult<Vec<u8>> {
        let mut rng = OsRng;
        
        match padding {
            RsaPadding::Pkcs1v15 => {
                let signing_key = rsa::pkcs1v15::SigningKey::<Sha256>::new(private_key.clone());
                let signature = signing_key.sign_with_rng(&mut rng, message);
                Ok(signature.to_bytes().to_vec())
            },
            RsaPadding::PssSha256 => {
                let signing_key = rsa::pss::SigningKey::<Sha256>::new(private_key.clone());
                let signature = signing_key.sign_with_rng(&mut rng, message);
                Ok(signature.to_bytes().to_vec())
            },
            RsaPadding::PssSha384 => {
                let signing_key = rsa::pss::SigningKey::<Sha384>::new(private_key.clone());
                let signature = signing_key.sign_with_rng(&mut rng, message);
                Ok(signature.to_bytes().to_vec())
            },
            RsaPadding::PssSha512 => {
                let signing_key = rsa::pss::SigningKey::<Sha512>::new(private_key.clone());
                let signature = signing_key.sign_with_rng(&mut rng, message);
                Ok(signature.to_bytes().to_vec())
            },
            _ => Err(RsaError::InvalidPadding(format!("Padding {} not supported for signing", padding.name()))),
        }
    }
    
    /// slay RSA verify signature with public key using specified padding and hash
    /// 
    /// # Security Notes
    /// - Verification is constant-time to prevent timing attacks
    /// - Signature format is validated before verification
    /// - Hash algorithm must match the one used for signing
    pub fn verify(&self, public_key: &RsaPublicKey, message: &[u8], signature: &[u8], padding: RsaPadding) -> RsaResult<bool> {
        // Validate signature size
        let expected_size = public_key.size();
        if signature.len() != expected_size {
            return Ok(false);
        }
        
        let result = match padding {
            RsaPadding::Pkcs1v15 => {
                let verifying_key = rsa::pkcs1v15::VerifyingKey::<Sha256>::new(public_key.clone());
                let sig = rsa::pkcs1v15::Signature::try_from(signature)
                    .map_err(|e| RsaError::VerificationFailed(e.to_string()))?;
                verifying_key.verify(message, &sig)
            },
            RsaPadding::PssSha256 => {
                let verifying_key = rsa::pss::VerifyingKey::<Sha256>::new(public_key.clone());
                let sig = rsa::pss::Signature::try_from(signature)
                    .map_err(|e| RsaError::VerificationFailed(e.to_string()))?;
                verifying_key.verify(message, &sig)
            },
            RsaPadding::PssSha384 => {
                let verifying_key = rsa::pss::VerifyingKey::<Sha384>::new(public_key.clone());
                let sig = rsa::pss::Signature::try_from(signature)
                    .map_err(|e| RsaError::VerificationFailed(e.to_string()))?;
                verifying_key.verify(message, &sig)
            },
            RsaPadding::PssSha512 => {
                let verifying_key = rsa::pss::VerifyingKey::<Sha512>::new(public_key.clone());
                let sig = rsa::pss::Signature::try_from(signature)
                    .map_err(|e| RsaError::VerificationFailed(e.to_string()))?;
                verifying_key.verify(message, &sig)
            },
            _ => return Err(RsaError::InvalidPadding(format!("Padding {} not supported for verification", padding.name()))),
        };
        
        Ok(result.is_ok())
    }
    
    /// slay Serialize private key to specified format
    /// 
    /// # Security Notes
    /// - Private keys should be encrypted when stored
    /// - Use secure storage mechanisms
    /// - Consider key derivation for password-based encryption
    pub fn serialize_private_key(&self, private_key: &RsaPrivateKey, format: KeyFormat) -> RsaResult<Zeroizing<Vec<u8>>> {
        match format {
            KeyFormat::Pkcs1Pem => {
                let pem = private_key.to_pkcs1_pem(rsa::pkcs8::LineEnding::LF)
                    .map_err(|e| RsaError::SerializationFailed(e.to_string()))?;
                Ok(Zeroizing::new(pem.as_bytes().to_vec()))
            },
            KeyFormat::Pkcs1Der => {
                let der = private_key.to_pkcs1_der()?;
                Ok(Zeroizing::new(der.to_bytes().to_vec()))
            },
            KeyFormat::Pkcs8Pem => {
                let pem = private_key.to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
                    .map_err(|e| RsaError::SerializationFailed(e.to_string()))?;
                Ok(Zeroizing::new(pem.as_bytes().to_vec()))
            },
            KeyFormat::Pkcs8Der => {
                let der = private_key.to_pkcs8_der()
                    .map_err(|e| RsaError::SerializationFailed(e.to_string()))?;
                Ok(Zeroizing::new(der.to_bytes().to_vec()))
            },
        }
    }
    
    /// slay Serialize public key to specified format
    pub fn serialize_public_key(&self, public_key: &RsaPublicKey, format: KeyFormat) -> RsaResult<Vec<u8>> {
        match format {
            KeyFormat::Pkcs1Pem => {
                let pem = public_key.to_pkcs1_pem(rsa::pkcs8::LineEnding::LF)
                    .map_err(|e| RsaError::SerializationFailed(e.to_string()))?;
                Ok(pem.as_bytes().to_vec())
            },
            KeyFormat::Pkcs1Der => {
                let der = public_key.to_pkcs1_der()?;
                Ok(der.to_bytes().to_vec())
            },
            KeyFormat::Pkcs8Pem => {
                let pem = public_key.to_public_key_pem(rsa::pkcs8::LineEnding::LF)
                    .map_err(|e| RsaError::SerializationFailed(e.to_string()))?;
                Ok(pem.as_bytes().to_vec())
            },
            KeyFormat::Pkcs8Der => {
                let der = public_key.to_public_key_der()
                    .map_err(|e| RsaError::SerializationFailed(e.to_string()))?;
                Ok(der.to_bytes().to_vec())
            },
        }
    }
    
    /// slay Deserialize private key from specified format
    pub fn deserialize_private_key(&self, key_data: &[u8], format: KeyFormat) -> RsaResult<RsaPrivateKey> {
        match format {
            KeyFormat::Pkcs1Pem | KeyFormat::Pkcs1Der => {
                if format == KeyFormat::Pkcs1Pem {
                    let pem_str = std::str::from_utf8(key_data)
                        .map_err(|e| RsaError::DeserializationFailed(e.to_string()))?;
                    RsaPrivateKey::from_pkcs1_pem(pem_str)
                        .map_err(|e| RsaError::DeserializationFailed(e.to_string()))
                } else {
                    RsaPrivateKey::from_pkcs1_der(key_data)
                        .map_err(|e| RsaError::DeserializationFailed(e.to_string()))
                }
            },
            KeyFormat::Pkcs8Pem | KeyFormat::Pkcs8Der => {
                if format == KeyFormat::Pkcs8Pem {
                    let pem_str = std::str::from_utf8(key_data)
                        .map_err(|e| RsaError::DeserializationFailed(e.to_string()))?;
                    RsaPrivateKey::from_pkcs8_pem(pem_str)
                        .map_err(|e| RsaError::DeserializationFailed(e.to_string()))
                } else {
                    RsaPrivateKey::from_pkcs8_der(key_data)
                        .map_err(|e| RsaError::DeserializationFailed(e.to_string()))
                }
            },
        }
    }
    
    /// slay Deserialize public key from specified format
    pub fn deserialize_public_key(&self, key_data: &[u8], format: KeyFormat) -> RsaResult<RsaPublicKey> {
        match format {
            KeyFormat::Pkcs1Pem | KeyFormat::Pkcs1Der => {
                if format == KeyFormat::Pkcs1Pem {
                    let pem_str = std::str::from_utf8(key_data)
                        .map_err(|e| RsaError::DeserializationFailed(e.to_string()))?;
                    RsaPublicKey::from_pkcs1_pem(pem_str)
                        .map_err(|e| RsaError::DeserializationFailed(e.to_string()))
                } else {
                    RsaPublicKey::from_pkcs1_der(key_data)
                        .map_err(|e| RsaError::DeserializationFailed(e.to_string()))
                }
            },
            KeyFormat::Pkcs8Pem | KeyFormat::Pkcs8Der => {
                if format == KeyFormat::Pkcs8Pem {
                    let pem_str = std::str::from_utf8(key_data)
                        .map_err(|e| RsaError::DeserializationFailed(e.to_string()))?;
                    RsaPublicKey::from_public_key_pem(pem_str)
                        .map_err(|e| RsaError::DeserializationFailed(e.to_string()))
                } else {
                    RsaPublicKey::from_public_key_der(key_data)
                        .map_err(|e| RsaError::DeserializationFailed(e.to_string()))
                }
            },
        }
    }
    
    // Helper methods
    
    fn validate_keypair(&self, public_key: &RsaPublicKey, private_key: &RsaPrivateKey) -> RsaResult<()> {
        // Validate that public and private keys match
        let test_message = b"validation_test_message";
        let mut rng = OsRng;
        
        // Test encryption/decryption
        let encrypted = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, test_message)
            .map_err(|e| RsaError::KeyGenerationFailed(format!("Key validation failed: {}", e)))?;
        
        let decrypted = private_key.decrypt(Pkcs1v15Encrypt, &encrypted)
            .map_err(|e| RsaError::KeyGenerationFailed(format!("Key validation failed: {}", e)))?;
        
        if decrypted != test_message {
            return Err(RsaError::KeyGenerationFailed("Generated keys do not match".to_string()));
        }
        
        Ok(())
    }
    
    fn validate_encryption_input(&self, public_key: &RsaPublicKey, plaintext: &[u8], padding: RsaPadding) -> RsaResult<()> {
        let key_size = public_key.size();
        let max_plaintext_len = match padding {
            RsaPadding::Pkcs1v15 => key_size - 11,
            RsaPadding::OaepSha256 => key_size - 2 * 32 - 2, // 2 * hash_len + 2
            RsaPadding::OaepSha384 => key_size - 2 * 48 - 2, // 2 * hash_len + 2
            RsaPadding::OaepSha512 => key_size - 2 * 64 - 2, // 2 * hash_len + 2
            _ => return Err(RsaError::InvalidPadding("Invalid padding for encryption".to_string())),
        };
        
        if plaintext.len() > max_plaintext_len {
            return Err(RsaError::InvalidInput(format!(
                "Plaintext too large: {} bytes (max {} bytes for {} with {}-bit key)",
                plaintext.len(), max_plaintext_len, padding.name(), key_size * 8
            )));
        }
        
        Ok(())
    }
}

impl Default for RsaEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Public API functions for CURSED integration
use crate::stdlib::value::Value;

/// slay Generate RSA key pair
pub fn rsa_generate_keypair(args: Vec<Value>) -> Result<Value, CursedError> {
    let key_size = if args.is_empty() {
        RSA_2048_BITS
    } else {
        match &args[0] {
            Value::Number(n) => *n as usize,
            _ => RSA_2048_BITS,
        }
    };
    
    let mut engine = RsaEngine::new();
    match engine.generate_keypair(key_size) {
        Ok(keypair) => {
            let mut result = HashMap::new();
            result.insert("algorithm".to_string(), Value::String("RSA".to_string()));
            result.insert("key_size".to_string(), Value::Number(keypair.key_size as f64));
            
            // Serialize public key to PEM for easy transport
            if let Ok(public_pem) = engine.serialize_public_key(&keypair.public_key, KeyFormat::Pkcs8Pem) {
                result.insert("public_key_pem".to_string(), Value::String(String::from_utf8_lossy(&public_pem).to_string()));
            }
            
            // Don't expose private key in API response for security
            result.insert("has_private_key".to_string(), Value::bool(true));
            
            Ok(Value::Object(result))
        },
        Err(e) => Err(CursedError::Runtime(format!("RSA key generation failed: {}", e))),
    }
}

/// slay RSA encrypt data with public key
pub fn rsa_encrypt(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("RSA encrypt requires public key and plaintext".to_string()));
    }
    
    let public_key_pem = match &args[0] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("Public key must be a PEM string".to_string())),
    };
    
    let plaintext = match &args[1] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(CursedError::Runtime("Plaintext must be a string".to_string())),
    };
    
    let padding = if args.len() > 2 {
        match &args[2] {
            Value::String(s) => match s.as_str() {
                "PKCS1v15" => RsaPadding::Pkcs1v15,
                "OAEP-SHA256" => RsaPadding::OaepSha256,
                "OAEP-SHA384" => RsaPadding::OaepSha384,
                "OAEP-SHA512" => RsaPadding::OaepSha512,
                _ => RsaPadding::OaepSha256, // Default to secure padding
            },
            _ => RsaPadding::OaepSha256,
        }
    } else {
        RsaPadding::OaepSha256 // Default to secure padding
    };
    
    let engine = RsaEngine::new();
    
    // Parse public key
    let public_key = engine.deserialize_public_key(public_key_pem.as_bytes(), KeyFormat::Pkcs8Pem)
        .map_err(|e| CursedError::Runtime(format!("Invalid public key: {}", e)))?;
    
    // Encrypt
    let ciphertext = engine.encrypt(&public_key, plaintext, padding)
        .map_err(|e| CursedError::Runtime(format!("Encryption failed: {}", e)))?;
    
    Ok(Value::String(base64::encode(ciphertext)))
}

/// slay RSA decrypt data with private key  
pub fn rsa_decrypt(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("RSA decrypt requires private key and ciphertext".to_string()));
    }
    
    let private_key_pem = match &args[0] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("Private key must be a PEM string".to_string())),
    };
    
    let ciphertext_b64 = match &args[1] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("Ciphertext must be a base64 string".to_string())),
    };
    
    let padding = if args.len() > 2 {
        match &args[2] {
            Value::String(s) => match s.as_str() {
                "PKCS1v15" => RsaPadding::Pkcs1v15,
                "OAEP-SHA256" => RsaPadding::OaepSha256,
                "OAEP-SHA384" => RsaPadding::OaepSha384,
                "OAEP-SHA512" => RsaPadding::OaepSha512,
                _ => RsaPadding::OaepSha256,
            },
            _ => RsaPadding::OaepSha256,
        }
    } else {
        RsaPadding::OaepSha256
    };
    
    let engine = RsaEngine::new();
    
    // Parse private key
    let private_key = engine.deserialize_private_key(private_key_pem.as_bytes(), KeyFormat::Pkcs8Pem)
        .map_err(|e| CursedError::Runtime(format!("Invalid private key: {}", e)))?;
    
    // Decode ciphertext
    let ciphertext = base64::decode(ciphertext_b64)
        .map_err(|e| CursedError::Runtime(format!("Invalid base64 ciphertext: {}", e)))?;
    
    // Decrypt
    let plaintext = engine.decrypt(&private_key, &ciphertext, padding)
        .map_err(|e| CursedError::Runtime(format!("Decryption failed: {}", e)))?;
    
    Ok(Value::String(String::from_utf8_lossy(&plaintext).to_string()))
}

/// slay RSA sign data with private key
pub fn rsa_sign(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("RSA sign requires private key and message".to_string()));
    }
    
    let private_key_pem = match &args[0] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("Private key must be a PEM string".to_string())),
    };
    
    let message = match &args[1] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(CursedError::Runtime("Message must be a string".to_string())),
    };
    
    let padding = if args.len() > 2 {
        match &args[2] {
            Value::String(s) => match s.as_str() {
                "PKCS1v15" => RsaPadding::Pkcs1v15,
                "PSS-SHA256" => RsaPadding::PssSha256,
                "PSS-SHA384" => RsaPadding::PssSha384,
                "PSS-SHA512" => RsaPadding::PssSha512,
                _ => RsaPadding::PssSha256, // Default to secure PSS padding
            },
            _ => RsaPadding::PssSha256,
        }
    } else {
        RsaPadding::PssSha256 // Default to secure PSS padding
    };
    
    let engine = RsaEngine::new();
    
    // Parse private key
    let private_key = engine.deserialize_private_key(private_key_pem.as_bytes(), KeyFormat::Pkcs8Pem)
        .map_err(|e| CursedError::Runtime(format!("Invalid private key: {}", e)))?;
    
    // Sign
    let signature = engine.sign(&private_key, message, padding)
        .map_err(|e| CursedError::Runtime(format!("Signing failed: {}", e)))?;
    
    Ok(Value::String(base64::encode(signature)))
}

/// slay RSA verify signature with public key
pub fn rsa_verify(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 3 {
        return Err(CursedError::Runtime("RSA verify requires public key, message, and signature".to_string()));
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
    
    let padding = if args.len() > 3 {
        match &args[3] {
            Value::String(s) => match s.as_str() {
                "PKCS1v15" => RsaPadding::Pkcs1v15,
                "PSS-SHA256" => RsaPadding::PssSha256,
                "PSS-SHA384" => RsaPadding::PssSha384,
                "PSS-SHA512" => RsaPadding::PssSha512,
                _ => RsaPadding::PssSha256,
            },
            _ => RsaPadding::PssSha256,
        }
    } else {
        RsaPadding::PssSha256
    };
    
    let engine = RsaEngine::new();
    
    // Parse public key
    let public_key = engine.deserialize_public_key(public_key_pem.as_bytes(), KeyFormat::Pkcs8Pem)
        .map_err(|e| CursedError::Runtime(format!("Invalid public key: {}", e)))?;
    
    // Decode signature
    let signature = base64::decode(signature_b64)
        .map_err(|e| CursedError::Runtime(format!("Invalid base64 signature: {}", e)))?;
    
    // Verify
    let is_valid = engine.verify(&public_key, message, &signature, padding)
        .map_err(|e| CursedError::Runtime(format!("Verification failed: {}", e)))?;
    
    Ok(Value::bool(is_valid))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rsa_key_generation() {
        let mut engine = RsaEngine::new();
        let result = engine.generate_keypair(RSA_2048_BITS);
        assert!(result.is_ok());
        
        let keypair = result.unwrap();
        assert_eq!(keypair.key_size, RSA_2048_BITS);
        assert_eq!(keypair.public_key.size(), RSA_2048_BITS / 8);
        assert_eq!(keypair.private_key.size(), RSA_2048_BITS / 8);
    }
    
    #[test]
    fn test_rsa_encryption_decryption() {
        let mut engine = RsaEngine::new();
        let keypair = engine.generate_keypair(RSA_2048_BITS).unwrap();
        
        let plaintext = b"Hello, RSA encryption with OAEP!";
        let encrypted = engine.encrypt(&keypair.public_key, plaintext, RsaPadding::OaepSha256).unwrap();
        let decrypted = engine.decrypt(&keypair.private_key, &encrypted, RsaPadding::OaepSha256).unwrap();
        
        assert_eq!(plaintext, &decrypted[..]);
    }
    
    #[test]
    fn test_rsa_signing_verification() {
        let mut engine = RsaEngine::new();
        let keypair = engine.generate_keypair(RSA_2048_BITS).unwrap();
        
        let message = b"Hello, RSA signatures with PSS!";
        let signature = engine.sign(&keypair.private_key, message, RsaPadding::PssSha256).unwrap();
        let verified = engine.verify(&keypair.public_key, message, &signature, RsaPadding::PssSha256).unwrap();
        
        assert!(verified);
        
        // Test with wrong message
        let wrong_message = b"Wrong message";
        let verified_wrong = engine.verify(&keypair.public_key, wrong_message, &signature, RsaPadding::PssSha256).unwrap();
        assert!(!verified_wrong);
    }
    
    #[test]
    fn test_key_serialization() {
        let mut engine = RsaEngine::new();
        let keypair = engine.generate_keypair(RSA_2048_BITS).unwrap();
        
        // Test private key serialization/deserialization
        let private_pem = engine.serialize_private_key(&keypair.private_key, KeyFormat::Pkcs8Pem).unwrap();
        let deserialized_private = engine.deserialize_private_key(&private_pem, KeyFormat::Pkcs8Pem).unwrap();
        
        // Test public key serialization/deserialization
        let public_pem = engine.serialize_public_key(&keypair.public_key, KeyFormat::Pkcs8Pem).unwrap();
        let deserialized_public = engine.deserialize_public_key(&public_pem, KeyFormat::Pkcs8Pem).unwrap();
        
        // Verify they still work
        let message = b"Test serialization";
        let encrypted = engine.encrypt(&deserialized_public, message, RsaPadding::OaepSha256).unwrap();
        let decrypted = engine.decrypt(&deserialized_private, &encrypted, RsaPadding::OaepSha256).unwrap();
        
        assert_eq!(message, &decrypted[..]);
    }
    
    #[test]
    fn test_invalid_key_size() {
        let mut engine = RsaEngine::new();
        let result = engine.generate_keypair(1024); // Too small
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RsaError::InvalidKeySize(1024)));
    }
    
    #[test]
    fn test_input_validation() {
        let mut engine = RsaEngine::new();
        let keypair = engine.generate_keypair(RSA_2048_BITS).unwrap();
        
        // Test oversized plaintext for OAEP-SHA256
        let large_plaintext = vec![0u8; 300]; // Too large for 2048-bit key with OAEP-SHA256
        let result = engine.encrypt(&keypair.public_key, &large_plaintext, RsaPadding::OaepSha256);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_multiple_padding_schemes() {
        let mut engine = RsaEngine::new();
        let keypair = engine.generate_keypair(RSA_2048_BITS).unwrap();
        
        let message = b"Test message";
        
        // Test PKCS#1 v1.5 signing and verification
        let sig_pkcs1 = engine.sign(&keypair.private_key, message, RsaPadding::Pkcs1v15).unwrap();
        let verified_pkcs1 = engine.verify(&keypair.public_key, message, &sig_pkcs1, RsaPadding::Pkcs1v15).unwrap();
        assert!(verified_pkcs1);
        
        // Test PSS with different hash algorithms
        let sig_pss256 = engine.sign(&keypair.private_key, message, RsaPadding::PssSha256).unwrap();
        let verified_pss256 = engine.verify(&keypair.public_key, message, &sig_pss256, RsaPadding::PssSha256).unwrap();
        assert!(verified_pss256);
        
        let sig_pss384 = engine.sign(&keypair.private_key, message, RsaPadding::PssSha384).unwrap();
        let verified_pss384 = engine.verify(&keypair.public_key, message, &sig_pss384, RsaPadding::PssSha384).unwrap();
        assert!(verified_pss384);
    }
}
