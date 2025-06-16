//! Public key cryptography utilities
//! 
//! Provides comprehensive public key operations for the CURSED stdlib.
//! Supports key extraction, validation, format conversion, and fingerprinting.

use crate::stdlib::value::Value;
use crate::error::CursedError;
use std::collections::HashMap;
use sha2::{Sha256, Sha384, Sha512, Digest};
use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::pkcs8::{EncodePublicKey, DecodePublicKey, EncodePrivateKey, DecodePrivateKey};
use rsa::pkcs1::{EncodeRsaPublicKey, DecodeRsaPublicKey};
use p256::{SecretKey as P256SecretKey, PublicKey as P256PublicKey};
use p384::{SecretKey as P384SecretKey, PublicKey as P384PublicKey};
use ed25519_dalek::{SigningKey, VerifyingKey, PUBLIC_KEY_LENGTH as ED25519_PUBLIC_KEY_LENGTH};
use x25519_dalek::{EphemeralSecret, PublicKey as X25519PublicKey};
use elliptic_curve::pkcs8::{EncodePublicKey as EcEncodePublicKey, DecodePublicKey as EcDecodePublicKey};

/// Supported public key formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicKeyFormat {
    Pkcs1Pem,      // PKCS#1 PEM format (RSA only)
    Pkcs1Der,      // PKCS#1 DER format (RSA only)
    Pkcs8Pem,      // PKCS#8 PEM format
    Pkcs8Der,      // PKCS#8 DER format
    Raw,           // Raw bytes format
    Ssh,           // SSH public key format
    Jwk,           // JSON Web Key format
}

impl PublicKeyFormat {
    pub fn name(&self) -> &'static str {
        match self {
            PublicKeyFormat::Pkcs1Pem => "PKCS#1-PEM",
            PublicKeyFormat::Pkcs1Der => "PKCS#1-DER",
            PublicKeyFormat::Pkcs8Pem => "PKCS#8-PEM",
            PublicKeyFormat::Pkcs8Der => "PKCS#8-DER",
            PublicKeyFormat::Raw => "Raw",
            PublicKeyFormat::Ssh => "SSH",
            PublicKeyFormat::Jwk => "JWK",
        }
    }
    
    pub fn from_name(name: &str) -> Result<Self, CursedError> {
        match name.to_uppercase().as_str() {
            "PKCS1-PEM" | "PKCS#1-PEM" => Ok(PublicKeyFormat::Pkcs1Pem),
            "PKCS1-DER" | "PKCS#1-DER" => Ok(PublicKeyFormat::Pkcs1Der),
            "PKCS8-PEM" | "PKCS#8-PEM" => Ok(PublicKeyFormat::Pkcs8Pem),
            "PKCS8-DER" | "PKCS#8-DER" => Ok(PublicKeyFormat::Pkcs8Der),
            "RAW" => Ok(PublicKeyFormat::Raw),
            "SSH" => Ok(PublicKeyFormat::Ssh),
            "JWK" => Ok(PublicKeyFormat::Jwk),
            _ => Err(CursedError::InvalidArgument(format!("Unsupported public key format: {}", name))),
        }
    }
}

/// Supported key algorithms for public key operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicKeyAlgorithm {
    Rsa,
    EcdsaP256,
    EcdsaP384,
    Ed25519,
    X25519,
}

impl PublicKeyAlgorithm {
    pub fn name(&self) -> &'static str {
        match self {
            PublicKeyAlgorithm::Rsa => "RSA",
            PublicKeyAlgorithm::EcdsaP256 => "ECDSA-P256",
            PublicKeyAlgorithm::EcdsaP384 => "ECDSA-P384",
            PublicKeyAlgorithm::Ed25519 => "Ed25519",
            PublicKeyAlgorithm::X25519 => "X25519",
        }
    }
    
    pub fn from_name(name: &str) -> Result<Self, CursedError> {
        match name.to_uppercase().as_str() {
            "RSA" => Ok(PublicKeyAlgorithm::Rsa),
            "ECDSA-P256" | "P256" => Ok(PublicKeyAlgorithm::EcdsaP256),
            "ECDSA-P384" | "P384" => Ok(PublicKeyAlgorithm::EcdsaP384),
            "ED25519" => Ok(PublicKeyAlgorithm::Ed25519),
            "X25519" => Ok(PublicKeyAlgorithm::X25519),
            _ => Err(CursedError::InvalidArgument(format!("Unsupported public key algorithm: {}", name))),
        }
    }
}

/// Public key information container
#[derive(Debug, Clone)]
pub struct PublicKeyInfo {
    pub algorithm: PublicKeyAlgorithm,
    pub key_size: usize,
    pub fingerprint_sha256: String,
    pub fingerprint_md5: String,
    pub raw_bytes: Vec<u8>,
    pub der_encoding: Vec<u8>,
}

impl PublicKeyInfo {
    pub fn new(
        algorithm: PublicKeyAlgorithm,
        key_size: usize,
        raw_bytes: Vec<u8>,
        der_encoding: Vec<u8>,
    ) -> Self {
        let fingerprint_sha256 = Self::compute_sha256_fingerprint(&der_encoding);
        let fingerprint_md5 = Self::compute_md5_fingerprint(&der_encoding);
        
        Self {
            algorithm,
            key_size,
            fingerprint_sha256,
            fingerprint_md5,
            raw_bytes,
            der_encoding,
        }
    }
    
    fn compute_sha256_fingerprint(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        hex::encode(result)
    }
    
    fn compute_md5_fingerprint(data: &[u8]) -> String {
        let mut hasher = md5::Md5::new();
        hasher.update(data);
        let result = hasher.finalize();
        hex::encode(result)
    }
    
    pub fn to_value(&self) -> Result<Value, CursedError> {
        let mut map = HashMap::new();
        
        map.insert("algorithm".to_string(), Value::String(self.algorithm.name().to_string()));
        map.insert("key_size".to_string(), Value::Integer(self.key_size as i64));
        map.insert("fingerprint_sha256".to_string(), Value::String(self.fingerprint_sha256.clone()));
        map.insert("fingerprint_md5".to_string(), Value::String(self.fingerprint_md5.clone()));
        map.insert("raw_bytes".to_string(), Value::String(hex::encode(&self.raw_bytes)));
        map.insert("der_encoding".to_string(), Value::String(hex::encode(&self.der_encoding)));
        
        Ok(Value::Object(map))
    }
}

/// Extract public key from private key
pub fn extract_public_key(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 2 {
        return Err(CursedError::InvalidArgument("Public key extraction requires: algorithm, private_key".to_string()));
    }
    
    let algorithm_name = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Algorithm must be a string".to_string())),
    };
    
    let private_key_hex = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Private key must be a string".to_string())),
    };
    
    let algorithm = PublicKeyAlgorithm::from_name(&algorithm_name)?;
    let private_key_bytes = hex::decode(private_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid private key hex: {}", e)))?;
    
    match algorithm {
        PublicKeyAlgorithm::Rsa => extract_rsa_public_key(&private_key_bytes),
        PublicKeyAlgorithm::EcdsaP256 => extract_p256_public_key(&private_key_bytes),
        PublicKeyAlgorithm::EcdsaP384 => extract_p384_public_key(&private_key_bytes),
        PublicKeyAlgorithm::Ed25519 => extract_ed25519_public_key(&private_key_bytes),
        PublicKeyAlgorithm::X25519 => extract_x25519_public_key(&private_key_bytes),
    }
}

/// Extract RSA public key
fn extract_rsa_public_key(private_key_bytes: &[u8]) -> Result<Value, CursedError> {
    let private_key = RsaPrivateKey::from_pkcs8_der(private_key_bytes)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode RSA private key: {}", e)))?;
    
    let public_key = RsaPublicKey::from(&private_key);
    let key_size = public_key.size() * 8; // Convert bytes to bits
    
    let der_encoding = public_key.to_public_key_der()
        .map_err(|e| CursedError::CryptoError(format!("Failed to encode RSA public key: {}", e)))?;
    
    // For RSA, raw bytes would be the modulus
    let raw_bytes = public_key.n().to_bytes_be();
    
    let info = PublicKeyInfo::new(
        PublicKeyAlgorithm::Rsa,
        key_size,
        raw_bytes,
        der_encoding.as_bytes().to_vec(),
    );
    
    info.to_value()
}

/// Extract P-256 public key
fn extract_p256_public_key(private_key_bytes: &[u8]) -> Result<Value, CursedError> {
    let private_key = P256SecretKey::from_pkcs8_der(private_key_bytes)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode P-256 private key: {}", e)))?;
    
    let public_key = P256PublicKey::from(&private_key);
    
    let der_encoding = public_key.to_public_key_der()
        .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-256 public key: {}", e)))?;
    
    use elliptic_curve::sec1::ToEncodedPoint;
    let encoded_point = public_key.to_encoded_point(false);
    let raw_bytes = encoded_point.as_bytes().to_vec();
    
    let info = PublicKeyInfo::new(
        PublicKeyAlgorithm::EcdsaP256,
        256,
        raw_bytes,
        der_encoding.as_bytes().to_vec(),
    );
    
    info.to_value()
}

/// Extract P-384 public key
fn extract_p384_public_key(private_key_bytes: &[u8]) -> Result<Value, CursedError> {
    let private_key = P384SecretKey::from_pkcs8_der(private_key_bytes)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode P-384 private key: {}", e)))?;
    
    let public_key = P384PublicKey::from(&private_key);
    
    let der_encoding = public_key.to_public_key_der()
        .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-384 public key: {}", e)))?;
    
    use elliptic_curve::sec1::ToEncodedPoint;
    let encoded_point = public_key.to_encoded_point(false);
    let raw_bytes = encoded_point.as_bytes().to_vec();
    
    let info = PublicKeyInfo::new(
        PublicKeyAlgorithm::EcdsaP384,
        384,
        raw_bytes,
        der_encoding.as_bytes().to_vec(),
    );
    
    info.to_value()
}

/// Extract Ed25519 public key
fn extract_ed25519_public_key(private_key_bytes: &[u8]) -> Result<Value, CursedError> {
    if private_key_bytes.len() != 32 {
        return Err(CursedError::InvalidArgument("Ed25519 private key must be 32 bytes".to_string()));
    }
    
    let signing_key = SigningKey::from_bytes(
        private_key_bytes.try_into()
            .map_err(|_| CursedError::InvalidArgument("Invalid Ed25519 private key length".to_string()))?
    );
    
    let verifying_key = signing_key.verifying_key();
    let raw_bytes = verifying_key.to_bytes().to_vec();
    
    // Ed25519 doesn't have standard DER encoding, use raw bytes
    let der_encoding = raw_bytes.clone();
    
    let info = PublicKeyInfo::new(
        PublicKeyAlgorithm::Ed25519,
        255, // Ed25519 key size
        raw_bytes,
        der_encoding,
    );
    
    info.to_value()
}

/// Extract X25519 public key
fn extract_x25519_public_key(private_key_bytes: &[u8]) -> Result<Value, CursedError> {
    if private_key_bytes.len() != 32 {
        return Err(CursedError::InvalidArgument("X25519 private key must be 32 bytes".to_string()));
    }
    
    let private_key = EphemeralSecret::from(
        <[u8; 32]>::try_from(private_key_bytes)
            .map_err(|_| CursedError::InvalidArgument("Invalid X25519 private key length".to_string()))?
    );
    
    let public_key = X25519PublicKey::from(&private_key);
    let raw_bytes = public_key.to_bytes().to_vec();
    
    // X25519 doesn't have standard DER encoding, use raw bytes
    let der_encoding = raw_bytes.clone();
    
    let info = PublicKeyInfo::new(
        PublicKeyAlgorithm::X25519,
        255, // X25519 key size
        raw_bytes,
        der_encoding,
    );
    
    info.to_value()
}

/// Validate public key format
pub fn validate_public_key(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 2 {
        return Err(CursedError::InvalidArgument("Public key validation requires: algorithm, public_key".to_string()));
    }
    
    let algorithm_name = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Algorithm must be a string".to_string())),
    };
    
    let public_key_hex = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Public key must be a string".to_string())),
    };
    
    let algorithm = PublicKeyAlgorithm::from_name(&algorithm_name)?;
    let public_key_bytes = hex::decode(public_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid public key hex: {}", e)))?;
    
    let is_valid = match algorithm {
        PublicKeyAlgorithm::Rsa => validate_rsa_public_key(&public_key_bytes),
        PublicKeyAlgorithm::EcdsaP256 => validate_p256_public_key(&public_key_bytes),
        PublicKeyAlgorithm::EcdsaP384 => validate_p384_public_key(&public_key_bytes),
        PublicKeyAlgorithm::Ed25519 => validate_ed25519_public_key(&public_key_bytes),
        PublicKeyAlgorithm::X25519 => validate_x25519_public_key(&public_key_bytes),
    };
    
    let mut result = HashMap::new();
    result.insert("algorithm".to_string(), Value::String(algorithm.name().to_string()));
    result.insert("valid".to_string(), Value::Boolean(is_valid));
    
    Ok(Value::Object(result))
}

/// Validate RSA public key
fn validate_rsa_public_key(public_key_bytes: &[u8]) -> bool {
    RsaPublicKey::from_public_key_der(public_key_bytes).is_ok()
}

/// Validate P-256 public key
fn validate_p256_public_key(public_key_bytes: &[u8]) -> bool {
    P256PublicKey::from_public_key_der(public_key_bytes).is_ok()
}

/// Validate P-384 public key
fn validate_p384_public_key(public_key_bytes: &[u8]) -> bool {
    P384PublicKey::from_public_key_der(public_key_bytes).is_ok()
}

/// Validate Ed25519 public key
fn validate_ed25519_public_key(public_key_bytes: &[u8]) -> bool {
    if public_key_bytes.len() != ED25519_PUBLIC_KEY_LENGTH {
        return false;
    }
    
    VerifyingKey::from_bytes(
        public_key_bytes.try_into().unwrap_or_default()
    ).is_ok()
}

/// Validate X25519 public key
fn validate_x25519_public_key(public_key_bytes: &[u8]) -> bool {
    public_key_bytes.len() == 32
}

/// Convert public key between formats
pub fn convert_public_key_format(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 4 {
        return Err(CursedError::InvalidArgument("Format conversion requires: algorithm, public_key, from_format, to_format".to_string()));
    }
    
    let algorithm_name = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Algorithm must be a string".to_string())),
    };
    
    let public_key_hex = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Public key must be a string".to_string())),
    };
    
    let from_format_name = match &args[2] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("From format must be a string".to_string())),
    };
    
    let to_format_name = match &args[3] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("To format must be a string".to_string())),
    };
    
    let algorithm = PublicKeyAlgorithm::from_name(&algorithm_name)?;
    let from_format = PublicKeyFormat::from_name(&from_format_name)?;
    let to_format = PublicKeyFormat::from_name(&to_format_name)?;
    
    let public_key_bytes = hex::decode(public_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid public key hex: {}", e)))?;
    
    match algorithm {
        PublicKeyAlgorithm::Rsa => convert_rsa_public_key_format(&public_key_bytes, from_format, to_format),
        PublicKeyAlgorithm::EcdsaP256 => convert_p256_public_key_format(&public_key_bytes, from_format, to_format),
        PublicKeyAlgorithm::EcdsaP384 => convert_p384_public_key_format(&public_key_bytes, from_format, to_format),
        PublicKeyAlgorithm::Ed25519 => convert_ed25519_public_key_format(&public_key_bytes, from_format, to_format),
        PublicKeyAlgorithm::X25519 => convert_x25519_public_key_format(&public_key_bytes, from_format, to_format),
    }
}

/// Convert RSA public key format
fn convert_rsa_public_key_format(
    public_key_bytes: &[u8],
    from_format: PublicKeyFormat,
    to_format: PublicKeyFormat,
) -> Result<Value, CursedError> {
    // Parse based on from_format
    let public_key = match from_format {
        PublicKeyFormat::Pkcs8Der => {
            RsaPublicKey::from_public_key_der(public_key_bytes)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse PKCS#8 DER: {}", e)))?
        },
        PublicKeyFormat::Pkcs1Der => {
            RsaPublicKey::from_pkcs1_der(public_key_bytes)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse PKCS#1 DER: {}", e)))?
        },
        _ => return Err(CursedError::NotImplemented(format!("Parsing {} format not implemented", from_format.name()))),
    };
    
    // Encode to target format
    let converted_bytes = match to_format {
        PublicKeyFormat::Pkcs8Der => {
            public_key.to_public_key_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode PKCS#8 DER: {}", e)))?
                .as_bytes().to_vec()
        },
        PublicKeyFormat::Pkcs1Der => {
            public_key.to_pkcs1_der()
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

/// Convert P-256 public key format
fn convert_p256_public_key_format(
    public_key_bytes: &[u8],
    from_format: PublicKeyFormat,
    to_format: PublicKeyFormat,
) -> Result<Value, CursedError> {
    // Parse based on from_format
    let public_key = match from_format {
        PublicKeyFormat::Pkcs8Der => {
            P256PublicKey::from_public_key_der(public_key_bytes)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse P-256 PKCS#8 DER: {}", e)))?
        },
        PublicKeyFormat::Raw => {
            P256PublicKey::from_sec1_bytes(public_key_bytes)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse P-256 raw bytes: {}", e)))?
        },
        _ => return Err(CursedError::NotImplemented(format!("Parsing {} format for P-256 not implemented", from_format.name()))),
    };
    
    // Encode to target format
    let converted_bytes = match to_format {
        PublicKeyFormat::Pkcs8Der => {
            public_key.to_public_key_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-256 PKCS#8 DER: {}", e)))?
                .as_bytes().to_vec()
        },
        PublicKeyFormat::Raw => {
            use elliptic_curve::sec1::ToEncodedPoint;
            public_key.to_encoded_point(false).as_bytes().to_vec()
        },
        _ => return Err(CursedError::NotImplemented(format!("Encoding {} format for P-256 not implemented", to_format.name()))),
    };
    
    let mut result = HashMap::new();
    result.insert("algorithm".to_string(), Value::String("ECDSA-P256".to_string()));
    result.insert("from_format".to_string(), Value::String(from_format.name().to_string()));
    result.insert("to_format".to_string(), Value::String(to_format.name().to_string()));
    result.insert("converted_key".to_string(), Value::String(hex::encode(converted_bytes)));
    
    Ok(Value::Object(result))
}

/// Convert P-384 public key format
fn convert_p384_public_key_format(
    public_key_bytes: &[u8],
    from_format: PublicKeyFormat,
    to_format: PublicKeyFormat,
) -> Result<Value, CursedError> {
    // Parse based on from_format
    let public_key = match from_format {
        PublicKeyFormat::Pkcs8Der => {
            P384PublicKey::from_public_key_der(public_key_bytes)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse P-384 PKCS#8 DER: {}", e)))?
        },
        PublicKeyFormat::Raw => {
            P384PublicKey::from_sec1_bytes(public_key_bytes)
                .map_err(|e| CursedError::CryptoError(format!("Failed to parse P-384 raw bytes: {}", e)))?
        },
        _ => return Err(CursedError::NotImplemented(format!("Parsing {} format for P-384 not implemented", from_format.name()))),
    };
    
    // Encode to target format
    let converted_bytes = match to_format {
        PublicKeyFormat::Pkcs8Der => {
            public_key.to_public_key_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-384 PKCS#8 DER: {}", e)))?
                .as_bytes().to_vec()
        },
        PublicKeyFormat::Raw => {
            use elliptic_curve::sec1::ToEncodedPoint;
            public_key.to_encoded_point(false).as_bytes().to_vec()
        },
        _ => return Err(CursedError::NotImplemented(format!("Encoding {} format for P-384 not implemented", to_format.name()))),
    };
    
    let mut result = HashMap::new();
    result.insert("algorithm".to_string(), Value::String("ECDSA-P384".to_string()));
    result.insert("from_format".to_string(), Value::String(from_format.name().to_string()));
    result.insert("to_format".to_string(), Value::String(to_format.name().to_string()));
    result.insert("converted_key".to_string(), Value::String(hex::encode(converted_bytes)));
    
    Ok(Value::Object(result))
}

/// Convert Ed25519 public key format
fn convert_ed25519_public_key_format(
    public_key_bytes: &[u8],
    from_format: PublicKeyFormat,
    to_format: PublicKeyFormat,
) -> Result<Value, CursedError> {
    // Parse based on from_format
    let public_key = match from_format {
        PublicKeyFormat::Raw => {
            if public_key_bytes.len() != 32 {
                return Err(CursedError::InvalidArgument("Ed25519 public key must be 32 bytes".to_string()));
            }
            let key_bytes: [u8; 32] = public_key_bytes.try_into()
                .map_err(|_| CursedError::InvalidArgument("Invalid Ed25519 key length".to_string()))?;
            VerifyingKey::from_bytes(&key_bytes)
                .map_err(|e| CursedError::CryptoError(format!("Invalid Ed25519 public key: {}", e)))?
        },
        _ => return Err(CursedError::NotImplemented(format!("Parsing {} format for Ed25519 not implemented", from_format.name()))),
    };
    
    // Encode to target format
    let converted_bytes = match to_format {
        PublicKeyFormat::Raw => {
            public_key.as_bytes().to_vec()
        },
        _ => return Err(CursedError::NotImplemented(format!("Encoding {} format for Ed25519 not implemented", to_format.name()))),
    };
    
    let mut result = HashMap::new();
    result.insert("algorithm".to_string(), Value::String("Ed25519".to_string()));
    result.insert("from_format".to_string(), Value::String(from_format.name().to_string()));
    result.insert("to_format".to_string(), Value::String(to_format.name().to_string()));
    result.insert("converted_key".to_string(), Value::String(hex::encode(converted_bytes)));
    
    Ok(Value::Object(result))
}

/// Convert X25519 public key format
fn convert_x25519_public_key_format(
    public_key_bytes: &[u8],
    from_format: PublicKeyFormat,
    to_format: PublicKeyFormat,
) -> Result<Value, CursedError> {
    // Validate key length
    if public_key_bytes.len() != 32 {
        return Err(CursedError::InvalidArgument("X25519 public key must be 32 bytes".to_string()));
    }
    
    // Parse based on from_format (X25519 only has raw format)
    match from_format {
        PublicKeyFormat::Raw => {},
        _ => return Err(CursedError::InvalidArgument(format!("X25519 only supports raw format, not {}", from_format.name()))),
    };
    
    // Encode to target format
    let converted_bytes = match to_format {
        PublicKeyFormat::Raw => {
            public_key_bytes.to_vec()
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

/// Generate public key fingerprint
pub fn public_key_fingerprint(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 1 {
        return Err(CursedError::InvalidArgument("Fingerprint generation requires: public_key".to_string()));
    }
    
    let public_key_hex = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Public key must be a string".to_string())),
    };
    
    let hash_algorithm = if args.len() > 1 {
        match &args[1] {
            Value::String(s) => s.clone(),
            _ => "SHA256".to_string(),
        }
    } else {
        "SHA256".to_string()
    };
    
    let public_key_bytes = hex::decode(public_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid public key hex: {}", e)))?;
    
    let fingerprint = match hash_algorithm.to_uppercase().as_str() {
        "SHA256" => {
            let mut hasher = Sha256::new();
            hasher.update(&public_key_bytes);
            hex::encode(hasher.finalize())
        },
        "SHA384" => {
            let mut hasher = Sha384::new();
            hasher.update(&public_key_bytes);
            hex::encode(hasher.finalize())
        },
        "SHA512" => {
            let mut hasher = Sha512::new();
            hasher.update(&public_key_bytes);
            hex::encode(hasher.finalize())
        },
        "MD5" => {
            let mut hasher = md5::Md5::new();
            hasher.update(&public_key_bytes);
            hex::encode(hasher.finalize())
        },
        _ => return Err(CursedError::InvalidArgument(format!("Unsupported hash algorithm: {}", hash_algorithm))),
    };
    
    let mut result = HashMap::new();
    result.insert("fingerprint".to_string(), Value::String(fingerprint));
    result.insert("algorithm".to_string(), Value::String(hash_algorithm));
    result.insert("key_size".to_string(), Value::Integer(public_key_bytes.len() as i64));
    
    Ok(Value::Object(result))
}

/// List supported public key formats
pub fn list_public_key_formats() -> Vec<String> {
    vec![
        PublicKeyFormat::Pkcs1Pem.name().to_string(),
        PublicKeyFormat::Pkcs1Der.name().to_string(),
        PublicKeyFormat::Pkcs8Pem.name().to_string(),
        PublicKeyFormat::Pkcs8Der.name().to_string(),
        PublicKeyFormat::Raw.name().to_string(),
        PublicKeyFormat::Ssh.name().to_string(),
        PublicKeyFormat::Jwk.name().to_string(),
    ]
}

/// Get public key algorithm information
pub fn get_public_key_algorithm_info(algorithm: PublicKeyAlgorithm) -> HashMap<String, Value> {
    let mut info = HashMap::new();
    
    info.insert("name".to_string(), Value::String(algorithm.name().to_string()));
    
    let (key_size, signature_support, encryption_support, key_exchange_support) = match algorithm {
        PublicKeyAlgorithm::Rsa => ("Variable (2048+)".to_string(), true, true, false),
        PublicKeyAlgorithm::EcdsaP256 => ("256".to_string(), true, false, false),
        PublicKeyAlgorithm::EcdsaP384 => ("384".to_string(), true, false, false),
        PublicKeyAlgorithm::Ed25519 => ("255".to_string(), true, false, false),
        PublicKeyAlgorithm::X25519 => ("255".to_string(), false, false, true),
    };
    
    info.insert("key_size".to_string(), Value::String(key_size));
    info.insert("signature_support".to_string(), Value::Boolean(signature_support));
    info.insert("encryption_support".to_string(), Value::Boolean(encryption_support));
    info.insert("key_exchange_support".to_string(), Value::Boolean(key_exchange_support));
    
    info
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm_from_name() {
        assert_eq!(PublicKeyAlgorithm::from_name("RSA").unwrap(), PublicKeyAlgorithm::Rsa);
        assert_eq!(PublicKeyAlgorithm::from_name("ed25519").unwrap(), PublicKeyAlgorithm::Ed25519);
        assert!(PublicKeyAlgorithm::from_name("invalid").is_err());
    }

    #[test]
    fn test_format_from_name() {
        assert_eq!(PublicKeyFormat::from_name("PKCS#8-DER").unwrap(), PublicKeyFormat::Pkcs8Der);
        assert_eq!(PublicKeyFormat::from_name("raw").unwrap(), PublicKeyFormat::Raw);
        assert!(PublicKeyFormat::from_name("invalid").is_err());
    }

    #[test]
    fn test_validate_ed25519_public_key() {
        let valid_key = vec![0u8; 32];
        assert!(validate_ed25519_public_key(&valid_key));
        
        let invalid_key = vec![0u8; 16];
        assert!(!validate_ed25519_public_key(&invalid_key));
    }

    #[test]
    fn test_validate_x25519_public_key() {
        let valid_key = vec![0u8; 32];
        assert!(validate_x25519_public_key(&valid_key));
        
        let invalid_key = vec![0u8; 16];
        assert!(!validate_x25519_public_key(&invalid_key));
    }

    #[test]
    fn test_list_public_key_formats() {
        let formats = list_public_key_formats();
        assert!(formats.contains(&"PKCS#8-DER".to_string()));
        assert!(formats.contains(&"Raw".to_string()));
    }
}
