//! Private key operations
//! 
//! Provides comprehensive private key utilities for the CURSED stdlib.
//! Supports private key generation, validation, format conversion, and security operations.

use crate::stdlib::value::Value;
use crate::error::CursedError;
use std::collections::HashMap;
use rand::rngs::OsRng;
use zeroize::{Zeroize, Zeroizing};
use sha2::{Sha256, Digest};
use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::pkcs8::{EncodePrivateKey, DecodePrivateKey, EncodePublicKey};
use rsa::pkcs1::{EncodeRsaPrivateKey, DecodeRsaPrivateKey};
use p256::{SecretKey as P256SecretKey, PublicKey as P256PublicKey};
use p384::{SecretKey as P384SecretKey, PublicKey as P384PublicKey};
use ed25519_dalek::{SigningKey, VerifyingKey, SECRET_KEY_LENGTH as ED25519_SECRET_KEY_LENGTH};
use x25519_dalek::{EphemeralSecret, PublicKey as X25519PublicKey};
use elliptic_curve::pkcs8::{EncodePrivateKey as EcEncodePrivateKey, DecodePrivateKey as EcDecodePrivateKey};

/// Supported private key formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivateKeyFormat {
    Pkcs1Pem,      // PKCS#1 PEM format (RSA only)
    Pkcs1Der,      // PKCS#1 DER format (RSA only)
    Pkcs8Pem,      // PKCS#8 PEM format
    Pkcs8Der,      // PKCS#8 DER format
    Raw,           // Raw bytes format
    OpenSsh,       // OpenSSH private key format
}

impl PrivateKeyFormat {
    pub fn name(&self) -> &'static str {
        match self {
            PrivateKeyFormat::Pkcs1Pem => "PKCS#1-PEM",
            PrivateKeyFormat::Pkcs1Der => "PKCS#1-DER",
            PrivateKeyFormat::Pkcs8Pem => "PKCS#8-PEM",
            PrivateKeyFormat::Pkcs8Der => "PKCS#8-DER",
            PrivateKeyFormat::Raw => "Raw",
            PrivateKeyFormat::OpenSsh => "OpenSSH",
        }
    }
    
    pub fn from_name(name: &str) -> Result<Self, CursedError> {
        match name.to_uppercase().as_str() {
            "PKCS1-PEM" | "PKCS#1-PEM" => Ok(PrivateKeyFormat::Pkcs1Pem),
            "PKCS1-DER" | "PKCS#1-DER" => Ok(PrivateKeyFormat::Pkcs1Der),
            "PKCS8-PEM" | "PKCS#8-PEM" => Ok(PrivateKeyFormat::Pkcs8Pem),
            "PKCS8-DER" | "PKCS#8-DER" => Ok(PrivateKeyFormat::Pkcs8Der),
            "RAW" => Ok(PrivateKeyFormat::Raw),
            "OPENSSH" | "SSH" => Ok(PrivateKeyFormat::OpenSsh),
            _ => Err(CursedError::InvalidArgument(format!("Unsupported private key format: {}", name))),
        }
    }
}

/// Supported private key algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivateKeyAlgorithm {
    Rsa2048,
    Rsa3072,
    Rsa4096,
    EcdsaP256,
    EcdsaP384,
    Ed25519,
    X25519,
}

impl PrivateKeyAlgorithm {
    pub fn name(&self) -> &'static str {
        match self {
            PrivateKeyAlgorithm::Rsa2048 => "RSA-2048",
            PrivateKeyAlgorithm::Rsa3072 => "RSA-3072",
            PrivateKeyAlgorithm::Rsa4096 => "RSA-4096",
            PrivateKeyAlgorithm::EcdsaP256 => "ECDSA-P256",
            PrivateKeyAlgorithm::EcdsaP384 => "ECDSA-P384",
            PrivateKeyAlgorithm::Ed25519 => "Ed25519",
            PrivateKeyAlgorithm::X25519 => "X25519",
        }
    }
    
    pub fn key_size(&self) -> usize {
        match self {
            PrivateKeyAlgorithm::Rsa2048 => 2048,
            PrivateKeyAlgorithm::Rsa3072 => 3072,
            PrivateKeyAlgorithm::Rsa4096 => 4096,
            PrivateKeyAlgorithm::EcdsaP256 => 256,
            PrivateKeyAlgorithm::EcdsaP384 => 384,
            PrivateKeyAlgorithm::Ed25519 => 255,
            PrivateKeyAlgorithm::X25519 => 255,
        }
    }
    
    pub fn from_name(name: &str) -> Result<Self, CursedError> {
        match name.to_uppercase().as_str() {
            "RSA-2048" | "RSA2048" => Ok(PrivateKeyAlgorithm::Rsa2048),
            "RSA-3072" | "RSA3072" => Ok(PrivateKeyAlgorithm::Rsa3072),
            "RSA-4096" | "RSA4096" => Ok(PrivateKeyAlgorithm::Rsa4096),
            "ECDSA-P256" | "P256" => Ok(PrivateKeyAlgorithm::EcdsaP256),
            "ECDSA-P384" | "P384" => Ok(PrivateKeyAlgorithm::EcdsaP384),
            "ED25519" => Ok(PrivateKeyAlgorithm::Ed25519),
            "X25519" => Ok(PrivateKeyAlgorithm::X25519),
            _ => Err(CursedError::InvalidArgument(format!("Unsupported private key algorithm: {}", name))),
        }
    }
}

/// Private key information container
#[derive(Debug, Clone)]
pub struct PrivateKeyInfo {
    pub algorithm: PrivateKeyAlgorithm,
    pub key_size: usize,
    pub fingerprint: String,
    pub has_public_key: bool,
    pub is_encrypted: bool,
    pub format: PrivateKeyFormat,
}

impl PrivateKeyInfo {
    pub fn new(
        algorithm: PrivateKeyAlgorithm,
        format: PrivateKeyFormat,
        key_data: &[u8],
        is_encrypted: bool,
    ) -> Self {
        let key_size = algorithm.key_size();
        let fingerprint = Self::compute_fingerprint(key_data);
        
        Self {
            algorithm,
            key_size,
            fingerprint,
            has_public_key: true, // Most formats include public key info
            is_encrypted,
            format,
        }
    }
    
    fn compute_fingerprint(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        hex::encode(result)
    }
    
    pub fn to_value(&self) -> Result<Value, CursedError> {
        let mut map = HashMap::new();
        
        map.insert("algorithm".to_string(), Value::String(self.algorithm.name().to_string()));
        map.insert("key_size".to_string(), Value::Integer(self.key_size as i64));
        map.insert("fingerprint".to_string(), Value::String(self.fingerprint.clone()));
        map.insert("has_public_key".to_string(), Value::Boolean(self.has_public_key));
        map.insert("is_encrypted".to_string(), Value::Boolean(self.is_encrypted));
        map.insert("format".to_string(), Value::String(self.format.name().to_string()));
        
        Ok(Value::Object(map))
    }
}

/// Secure private key container with automatic zeroization
#[derive(Debug)]
pub struct SecurePrivateKey {
    algorithm: PrivateKeyAlgorithm,
    key_data: Zeroizing<Vec<u8>>,
    format: PrivateKeyFormat,
}

impl SecurePrivateKey {
    pub fn new(
        algorithm: PrivateKeyAlgorithm,
        key_data: Vec<u8>,
        format: PrivateKeyFormat,
    ) -> Self {
        Self {
            algorithm,
            key_data: Zeroizing::new(key_data),
            format,
        }
    }
    
    pub fn algorithm(&self) -> PrivateKeyAlgorithm {
        self.algorithm
    }
    
    pub fn format(&self) -> PrivateKeyFormat {
        self.format
    }
    
    pub fn key_data(&self) -> &[u8] {
        &self.key_data
    }
    
    pub fn info(&self) -> PrivateKeyInfo {
        PrivateKeyInfo::new(self.algorithm, self.format, &self.key_data, false)
    }
}

impl Drop for SecurePrivateKey {
    fn drop(&mut self) {
        // Zeroizing<Vec<u8>> automatically zeros memory on drop
    }
}

/// Generate private key
pub fn generate_private_key(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.is_empty() {
        return Err(CursedError::InvalidArgument("Algorithm name required".to_string()));
    }
    
    let algorithm_name = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Algorithm name must be a string".to_string())),
    };
    
    let algorithm = PrivateKeyAlgorithm::from_name(&algorithm_name)?;
    
    match algorithm {
        PrivateKeyAlgorithm::Rsa2048 => generate_rsa_private_key(2048),
        PrivateKeyAlgorithm::Rsa3072 => generate_rsa_private_key(3072),
        PrivateKeyAlgorithm::Rsa4096 => generate_rsa_private_key(4096),
        PrivateKeyAlgorithm::EcdsaP256 => generate_p256_private_key(),
        PrivateKeyAlgorithm::EcdsaP384 => generate_p384_private_key(),
        PrivateKeyAlgorithm::Ed25519 => generate_ed25519_private_key(),
        PrivateKeyAlgorithm::X25519 => generate_x25519_private_key(),
    }
}

/// Generate RSA private key
fn generate_rsa_private_key(key_size: usize) -> Result<Value, CursedError> {
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, key_size)
        .map_err(|e| CursedError::CryptoError(format!("RSA key generation failed: {}", e)))?;
    
    let der_bytes = private_key.to_pkcs8_der()
        .map_err(|e| CursedError::CryptoError(format!("RSA private key encoding failed: {}", e)))?;
    
    let algorithm = match key_size {
        2048 => PrivateKeyAlgorithm::Rsa2048,
        3072 => PrivateKeyAlgorithm::Rsa3072,
        4096 => PrivateKeyAlgorithm::Rsa4096,
        _ => return Err(CursedError::InvalidArgument(format!("Unsupported RSA key size: {}", key_size))),
    };
    
    let info = PrivateKeyInfo::new(
        algorithm,
        PrivateKeyFormat::Pkcs8Der,
        der_bytes.as_bytes(),
        false,
    );
    
    let mut result = info.to_value()?;
    if let Value::Object(ref mut map) = result {
        map.insert("private_key".to_string(), Value::String(hex::encode(der_bytes.as_bytes())));
    }
    
    Ok(result)
}

/// Generate P-256 private key
fn generate_p256_private_key() -> Result<Value, CursedError> {
    let mut rng = OsRng;
    let private_key = P256SecretKey::random(&mut rng);
    
    let der_bytes = private_key.to_pkcs8_der()
        .map_err(|e| CursedError::CryptoError(format!("P-256 private key encoding failed: {}", e)))?;
    
    let info = PrivateKeyInfo::new(
        PrivateKeyAlgorithm::EcdsaP256,
        PrivateKeyFormat::Pkcs8Der,
        der_bytes.as_bytes(),
        false,
    );
    
    let mut result = info.to_value()?;
    if let Value::Object(ref mut map) = result {
        map.insert("private_key".to_string(), Value::String(hex::encode(der_bytes.as_bytes())));
    }
    
    Ok(result)
}

/// Generate P-384 private key
fn generate_p384_private_key() -> Result<Value, CursedError> {
    let mut rng = OsRng;
    let private_key = P384SecretKey::random(&mut rng);
    
    let der_bytes = private_key.to_pkcs8_der()
        .map_err(|e| CursedError::CryptoError(format!("P-384 private key encoding failed: {}", e)))?;
    
    let info = PrivateKeyInfo::new(
        PrivateKeyAlgorithm::EcdsaP384,
        PrivateKeyFormat::Pkcs8Der,
        der_bytes.as_bytes(),
        false,
    );
    
    let mut result = info.to_value()?;
    if let Value::Object(ref mut map) = result {
        map.insert("private_key".to_string(), Value::String(hex::encode(der_bytes.as_bytes())));
    }
    
    Ok(result)
}

/// Generate Ed25519 private key
fn generate_ed25519_private_key() -> Result<Value, CursedError> {
    let mut rng = OsRng;
    let signing_key = SigningKey::generate(&mut rng);
    let private_bytes = signing_key.to_bytes();
    
    let info = PrivateKeyInfo::new(
        PrivateKeyAlgorithm::Ed25519,
        PrivateKeyFormat::Raw,
        &private_bytes,
        false,
    );
    
    let mut result = info.to_value()?;
    if let Value::Object(ref mut map) = result {
        map.insert("private_key".to_string(), Value::String(hex::encode(private_bytes)));
    }
    
    Ok(result)
}

/// Generate X25519 private key
fn generate_x25519_private_key() -> Result<Value, CursedError> {
    let mut rng = OsRng;
    let private_key = EphemeralSecret::random();
    let private_bytes = private_key.to_bytes();
    
    let info = PrivateKeyInfo::new(
        PrivateKeyAlgorithm::X25519,
        PrivateKeyFormat::Raw,
        &private_bytes,
        false,
    );
    
    let mut result = info.to_value()?;
    if let Value::Object(ref mut map) = result {
        map.insert("private_key".to_string(), Value::String(hex::encode(private_bytes)));
    }
    
    Ok(result)
}

/// Validate private key
pub fn validate_private_key(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 2 {
        return Err(CursedError::InvalidArgument("Private key validation requires: algorithm, private_key".to_string()));
    }
    
    let algorithm_name = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Algorithm must be a string".to_string())),
    };
    
    let private_key_hex = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Private key must be a string".to_string())),
    };
    
    let algorithm = PrivateKeyAlgorithm::from_name(&algorithm_name)?;
    let private_key_bytes = hex::decode(private_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid private key hex: {}", e)))?;
    
    let validation_result = match algorithm {
        PrivateKeyAlgorithm::Rsa2048 | 
        PrivateKeyAlgorithm::Rsa3072 | 
        PrivateKeyAlgorithm::Rsa4096 => validate_rsa_private_key(&private_key_bytes, algorithm.key_size()),
        PrivateKeyAlgorithm::EcdsaP256 => validate_p256_private_key(&private_key_bytes),
        PrivateKeyAlgorithm::EcdsaP384 => validate_p384_private_key(&private_key_bytes),
        PrivateKeyAlgorithm::Ed25519 => validate_ed25519_private_key(&private_key_bytes),
        PrivateKeyAlgorithm::X25519 => validate_x25519_private_key(&private_key_bytes),
    };
    
    let mut result = HashMap::new();
    result.insert("algorithm".to_string(), Value::String(algorithm.name().to_string()));
    result.insert("valid".to_string(), Value::Boolean(validation_result.is_ok()));
    
    if let Err(error_msg) = validation_result {
        result.insert("error".to_string(), Value::String(error_msg));
    }
    
    Ok(Value::Object(result))
}

/// Validate RSA private key
fn validate_rsa_private_key(private_key_bytes: &[u8], expected_size: usize) -> Result<(), String> {
    let private_key = RsaPrivateKey::from_pkcs8_der(private_key_bytes)
        .map_err(|e| format!("Invalid RSA private key format: {}", e))?;
    
    let actual_size = private_key.size() * 8; // Convert bytes to bits
    if actual_size != expected_size {
        return Err(format!("Key size mismatch: expected {}, got {}", expected_size, actual_size));
    }
    
    // Validate key components
    private_key.validate()
        .map_err(|e| format!("RSA key validation failed: {}", e))?;
    
    Ok(())
}

/// Validate P-256 private key
fn validate_p256_private_key(private_key_bytes: &[u8]) -> Result<(), String> {
    P256SecretKey::from_pkcs8_der(private_key_bytes)
        .map_err(|e| format!("Invalid P-256 private key: {}", e))?;
    
    Ok(())
}

/// Validate P-384 private key
fn validate_p384_private_key(private_key_bytes: &[u8]) -> Result<(), String> {
    P384SecretKey::from_pkcs8_der(private_key_bytes)
        .map_err(|e| format!("Invalid P-384 private key: {}", e))?;
    
    Ok(())
}

/// Validate Ed25519 private key
fn validate_ed25519_private_key(private_key_bytes: &[u8]) -> Result<(), String> {
    if private_key_bytes.len() != ED25519_SECRET_KEY_LENGTH {
        return Err(format!("Ed25519 private key must be {} bytes, got {}", ED25519_SECRET_KEY_LENGTH, private_key_bytes.len()));
    }
    
    let key_array: [u8; ED25519_SECRET_KEY_LENGTH] = private_key_bytes.try_into()
        .map_err(|_| "Invalid key length".to_string())?;
    
    SigningKey::from_bytes(&key_array);
    
    Ok(())
}

/// Validate X25519 private key
fn validate_x25519_private_key(private_key_bytes: &[u8]) -> Result<(), String> {
    if private_key_bytes.len() != 32 {
        return Err(format!("X25519 private key must be 32 bytes, got {}", private_key_bytes.len()));
    }
    
    Ok(())
}

/// Convert private key format
pub fn convert_private_key_format(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 4 {
        return Err(CursedError::InvalidArgument("Format conversion requires: algorithm, private_key, from_format, to_format".to_string()));
    }
    
    let algorithm_name = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Algorithm must be a string".to_string())),
    };
    
    let private_key_hex = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Private key must be a string".to_string())),
    };
    
    let from_format_name = match &args[2] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("From format must be a string".to_string())),
    };
    
    let to_format_name = match &args[3] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("To format must be a string".to_string())),
    };
    
    let algorithm = PrivateKeyAlgorithm::from_name(&algorithm_name)?;
    let from_format = PrivateKeyFormat::from_name(&from_format_name)?;
    let to_format = PrivateKeyFormat::from_name(&to_format_name)?;
    
    let private_key_bytes = hex::decode(private_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid private key hex: {}", e)))?;
    
    match algorithm {
        PrivateKeyAlgorithm::Rsa2048 | 
        PrivateKeyAlgorithm::Rsa3072 | 
        PrivateKeyAlgorithm::Rsa4096 => convert_rsa_private_key_format(&private_key_bytes, from_format, to_format),
        PrivateKeyAlgorithm::EcdsaP256 => convert_p256_private_key_format(&private_key_bytes, from_format, to_format),
        PrivateKeyAlgorithm::EcdsaP384 => convert_p384_private_key_format(&private_key_bytes, from_format, to_format),
        PrivateKeyAlgorithm::Ed25519 => convert_ed25519_private_key_format(&private_key_bytes, from_format, to_format),
        PrivateKeyAlgorithm::X25519 => convert_x25519_private_key_format(&private_key_bytes, from_format, to_format),
    }
}

/// Convert RSA private key format
fn convert_rsa_private_key_format(
    private_key_bytes: &[u8],
    from_format: PrivateKeyFormat,
    to_format: PrivateKeyFormat,
) -> Result<Value, CursedError> {
    // Parse based on from_format
    let private_key = match from_format {
        PrivateKeyFormat::Pkcs8Der => {
            RsaPrivateKey::from_pkcs8_der(private_key_bytes)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse PKCS#8 DER: {}", e)))?
        },
        PrivateKeyFormat::Pkcs1Der => {
            RsaPrivateKey::from_pkcs1_der(private_key_bytes)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse PKCS#1 DER: {}", e)))?
        },
        _ => return Err(CursedError::NotImplemented(format!("Parsing {} format not implemented", from_format.name()))),
    };
    
    // Encode to target format
    let converted_bytes = match to_format {
        PrivateKeyFormat::Pkcs8Der => {
            private_key.to_pkcs8_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode PKCS#8 DER: {}", e)))?
                .as_bytes().to_vec()
        },
        PrivateKeyFormat::Pkcs1Der => {
            private_key.to_pkcs1_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode PKCS#1 DER: {}", e)))?
                .as_bytes().to_vec()
        },
        _ => return Err(CursedError::NotImplemented(format!("Encoding {} format not implemented", to_format.name()))),
    };
    
    let mut result = HashMap::new();
    result.insert("algorithm".to_string(), Value::String("RSA".to_string()));
    result.insert("from_format".to_string(), Value::String(from_format.name().to_string()));
    result.insert("to_format".to_string(), Value::String(to_format.name().to_string()));
    result.insert("converted_key".to_string(), Value::String(hex::encode(converted_bytes)));
    
    Ok(Value::Object(result))
}

/// Convert P-256 private key format
fn convert_p256_private_key_format(
    private_key_bytes: &[u8],
    from_format: PrivateKeyFormat,
    to_format: PrivateKeyFormat,
) -> Result<Value, CursedError> {
    // Parse based on from_format
    let private_key = match from_format {
        PrivateKeyFormat::Pkcs8Der => {
            P256SecretKey::from_pkcs8_der(private_key_bytes)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse P-256 PKCS#8 DER: {}", e)))?
        },
        PrivateKeyFormat::Raw => {
            if private_key_bytes.len() != 32 {
                return Err(CursedError::InvalidArgument("P-256 private key must be 32 bytes in raw format".to_string()));
            }
            P256SecretKey::from_bytes(&private_key_bytes.into())
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse P-256 raw bytes: {}", e)))?
        },
        _ => return Err(CursedError::NotImplemented(format!("Parsing {} format for P-256 private key not implemented", from_format.name()))),
    };
    
    // Encode to target format
    let converted_bytes = match to_format {
        PrivateKeyFormat::Pkcs8Der => {
            private_key.to_pkcs8_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-256 PKCS#8 DER: {}", e)))?
                .as_bytes().to_vec()
        },
        PrivateKeyFormat::Raw => {
            private_key.to_bytes().to_vec()
        },
        _ => return Err(CursedError::NotImplemented(format!("Encoding {} format for P-256 private key not implemented", to_format.name()))),
    };
    
    let mut result = HashMap::new();
    result.insert("algorithm".to_string(), Value::String("ECDSA-P256".to_string()));
    result.insert("from_format".to_string(), Value::String(from_format.name().to_string()));
    result.insert("to_format".to_string(), Value::String(to_format.name().to_string()));
    result.insert("converted_key".to_string(), Value::String(hex::encode(converted_bytes)));
    
    Ok(Value::Object(result))
}

/// Convert P-384 private key format
fn convert_p384_private_key_format(
    private_key_bytes: &[u8],
    from_format: PrivateKeyFormat,
    to_format: PrivateKeyFormat,
) -> Result<Value, CursedError> {
    // Parse based on from_format
    let private_key = match from_format {
        PrivateKeyFormat::Pkcs8Der => {
            P384SecretKey::from_pkcs8_der(private_key_bytes)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse P-384 PKCS#8 DER: {}", e)))?
        },
        PrivateKeyFormat::Raw => {
            if private_key_bytes.len() != 48 {
                return Err(CursedError::InvalidArgument("P-384 private key must be 48 bytes in raw format".to_string()));
            }
            P384SecretKey::from_bytes(&private_key_bytes.into())
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse P-384 raw bytes: {}", e)))?
        },
        _ => return Err(CursedError::NotImplemented(format!("Parsing {} format for P-384 private key not implemented", from_format.name()))),
    };
    
    // Encode to target format
    let converted_bytes = match to_format {
        PrivateKeyFormat::Pkcs8Der => {
            private_key.to_pkcs8_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-384 PKCS#8 DER: {}", e)))?
                .as_bytes().to_vec()
        },
        PrivateKeyFormat::Raw => {
            private_key.to_bytes().to_vec()
        },
        _ => return Err(CursedError::NotImplemented(format!("Encoding {} format for P-384 private key not implemented", to_format.name()))),
    };
    
    let mut result = HashMap::new();
    result.insert("algorithm".to_string(), Value::String("ECDSA-P384".to_string()));
    result.insert("from_format".to_string(), Value::String(from_format.name().to_string()));
    result.insert("to_format".to_string(), Value::String(to_format.name().to_string()));
    result.insert("converted_key".to_string(), Value::String(hex::encode(converted_bytes)));
    
    Ok(Value::Object(result))
}

/// Convert Ed25519 private key format
fn convert_ed25519_private_key_format(
    private_key_bytes: &[u8],
    from_format: PrivateKeyFormat,
    to_format: PrivateKeyFormat,
) -> Result<Value, CursedError> {
    // Parse based on from_format
    let private_key = match from_format {
        PrivateKeyFormat::Raw => {
            if private_key_bytes.len() != 32 {
                return Err(CursedError::InvalidArgument("Ed25519 private key must be 32 bytes".to_string()));
            }
            let key_bytes: [u8; 32] = private_key_bytes.try_into()
                .map_err(|_| CursedError::InvalidArgument("Invalid Ed25519 key length".to_string()))?;
            SigningKey::from_bytes(&key_bytes)
        },
        _ => return Err(CursedError::NotImplemented(format!("Parsing {} format for Ed25519 private key not implemented", from_format.name()))),
    };
    
    // Encode to target format
    let converted_bytes = match to_format {
        PrivateKeyFormat::Raw => {
            private_key.to_bytes().to_vec()
        },
        _ => return Err(CursedError::NotImplemented(format!("Encoding {} format for Ed25519 private key not implemented", to_format.name()))),
    };
    
    let mut result = HashMap::new();
    result.insert("algorithm".to_string(), Value::String("Ed25519".to_string()));
    result.insert("from_format".to_string(), Value::String(from_format.name().to_string()));
    result.insert("to_format".to_string(), Value::String(to_format.name().to_string()));
    result.insert("converted_key".to_string(), Value::String(hex::encode(converted_bytes)));
    
    Ok(Value::Object(result))
}

/// Convert X25519 private key format
fn convert_x25519_private_key_format(
    private_key_bytes: &[u8],
    from_format: PrivateKeyFormat,
    to_format: PrivateKeyFormat,
) -> Result<Value, CursedError> {
    // Validate key length
    if private_key_bytes.len() != 32 {
        return Err(CursedError::InvalidArgument("X25519 private key must be 32 bytes".to_string()));
    }
    
    // Parse based on from_format (X25519 only has raw format)
    match from_format {
        PrivateKeyFormat::Raw => {},
        _ => return Err(CursedError::InvalidArgument(format!("X25519 only supports raw format, not {}", from_format.name()))),
    };
    
    // Encode to target format
    let converted_bytes = match to_format {
        PrivateKeyFormat::Raw => {
            private_key_bytes.to_vec()
        },
        _ => return Err(CursedError::InvalidArgument(format!("X25519 only supports raw format, not {}", to_format.name()))),
    };
    
    let mut result = HashMap::new();
    result.insert("algorithm".to_string(), Value::String("X25519".to_string()));
    result.insert("from_format".to_string(), Value::String(from_format.name().to_string()));
    result.insert("to_format".to_string(), Value::String(to_format.name().to_string()));
    result.insert("converted_key".to_string(), Value::String(hex::encode(converted_bytes)));
    
    Ok(Value::Object(result))
}

/// Check if private key is encrypted
pub fn is_private_key_encrypted(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.is_empty() {
        return Err(CursedError::InvalidArgument("Private key required".to_string()));
    }
    
    let private_key_hex = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Private key must be a string".to_string())),
    };
    
    let private_key_bytes = hex::decode(private_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid private key hex: {}", e)))?;
    
    // Basic heuristics to detect encryption
    let is_encrypted = private_key_bytes.starts_with(b"-----BEGIN ENCRYPTED PRIVATE KEY-----") ||
                      private_key_bytes.contains(&b"ENCRYPTED"[..]) ||
                      detect_pkcs8_encryption(&private_key_bytes);
    
    let mut result = HashMap::new();
    result.insert("encrypted".to_string(), Value::Boolean(is_encrypted));
    
    Ok(Value::Object(result))
}

/// Detect PKCS#8 encryption by trying to parse
fn detect_pkcs8_encryption(data: &[u8]) -> bool {
    // Try to parse as unencrypted PKCS#8 - if it fails, might be encrypted
    RsaPrivateKey::from_pkcs8_der(data).is_err() &&
    P256SecretKey::from_pkcs8_der(data).is_err() &&
    P384SecretKey::from_pkcs8_der(data).is_err()
}

/// Get private key strength assessment
pub fn assess_private_key_strength(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 2 {
        return Err(CursedError::InvalidArgument("Strength assessment requires: algorithm, private_key".to_string()));
    }
    
    let algorithm_name = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Algorithm must be a string".to_string())),
    };
    
    let private_key_hex = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Private key must be a string".to_string())),
    };
    
    let algorithm = PrivateKeyAlgorithm::from_name(&algorithm_name)?;
    let private_key_bytes = hex::decode(private_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid private key hex: {}", e)))?;
    
    let (strength, recommendation) = match algorithm {
        PrivateKeyAlgorithm::Rsa2048 => ("Medium", "Consider upgrading to RSA-3072 for long-term security"),
        PrivateKeyAlgorithm::Rsa3072 => ("Good", "Adequate for most current applications"),
        PrivateKeyAlgorithm::Rsa4096 => ("Excellent", "High security for sensitive applications"),
        PrivateKeyAlgorithm::EcdsaP256 => ("Good", "Equivalent to ~3072-bit RSA"),
        PrivateKeyAlgorithm::EcdsaP384 => ("Excellent", "Equivalent to ~7680-bit RSA"),
        PrivateKeyAlgorithm::Ed25519 => ("Excellent", "High security with excellent performance"),
        PrivateKeyAlgorithm::X25519 => ("Excellent", "Secure key exchange algorithm"),
    };
    
    let mut result = HashMap::new();
    result.insert("algorithm".to_string(), Value::String(algorithm.name().to_string()));
    result.insert("key_size".to_string(), Value::Integer(algorithm.key_size() as i64));
    result.insert("strength".to_string(), Value::String(strength.to_string()));
    result.insert("recommendation".to_string(), Value::String(recommendation.to_string()));
    
    Ok(Value::Object(result))
}

/// List supported private key algorithms
pub fn list_private_key_algorithms() -> Vec<String> {
    vec![
        PrivateKeyAlgorithm::Rsa2048.name().to_string(),
        PrivateKeyAlgorithm::Rsa3072.name().to_string(),
        PrivateKeyAlgorithm::Rsa4096.name().to_string(),
        PrivateKeyAlgorithm::EcdsaP256.name().to_string(),
        PrivateKeyAlgorithm::EcdsaP384.name().to_string(),
        PrivateKeyAlgorithm::Ed25519.name().to_string(),
        PrivateKeyAlgorithm::X25519.name().to_string(),
    ]
}

/// List supported private key formats
pub fn list_private_key_formats() -> Vec<String> {
    vec![
        PrivateKeyFormat::Pkcs1Pem.name().to_string(),
        PrivateKeyFormat::Pkcs1Der.name().to_string(),
        PrivateKeyFormat::Pkcs8Pem.name().to_string(),
        PrivateKeyFormat::Pkcs8Der.name().to_string(),
        PrivateKeyFormat::Raw.name().to_string(),
        PrivateKeyFormat::OpenSsh.name().to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm_from_name() {
        assert_eq!(PrivateKeyAlgorithm::from_name("RSA-2048").unwrap(), PrivateKeyAlgorithm::Rsa2048);
        assert_eq!(PrivateKeyAlgorithm::from_name("ed25519").unwrap(), PrivateKeyAlgorithm::Ed25519);
        assert!(PrivateKeyAlgorithm::from_name("invalid").is_err());
    }

    #[test]
    fn test_format_from_name() {
        assert_eq!(PrivateKeyFormat::from_name("PKCS#8-DER").unwrap(), PrivateKeyFormat::Pkcs8Der);
        assert_eq!(PrivateKeyFormat::from_name("raw").unwrap(), PrivateKeyFormat::Raw);
        assert!(PrivateKeyFormat::from_name("invalid").is_err());
    }

    #[test]
    fn test_validate_ed25519_private_key() {
        let valid_key = vec![0u8; 32];
        assert!(validate_ed25519_private_key(&valid_key).is_ok());
        
        let invalid_key = vec![0u8; 16];
        assert!(validate_ed25519_private_key(&invalid_key).is_err());
    }

    #[test]
    fn test_validate_x25519_private_key() {
        let valid_key = vec![0u8; 32];
        assert!(validate_x25519_private_key(&valid_key).is_ok());
        
        let invalid_key = vec![0u8; 16];
        assert!(validate_x25519_private_key(&invalid_key).is_err());
    }

    #[test]
    fn test_list_private_key_algorithms() {
        let algorithms = list_private_key_algorithms();
        assert!(algorithms.contains(&"RSA-2048".to_string()));
        assert!(algorithms.contains(&"Ed25519".to_string()));
    }

    #[test]
    fn test_list_private_key_formats() {
        let formats = list_private_key_formats();
        assert!(formats.contains(&"PKCS#8-DER".to_string()));
        assert!(formats.contains(&"Raw".to_string()));
    }
}
