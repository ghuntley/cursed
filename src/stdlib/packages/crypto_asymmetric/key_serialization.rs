//! Key serialization utilities
//! 
//! Provides comprehensive key serialization functions for the CURSED stdlib.
//! Supports PEM, DER, JWK, SSH, and raw formats for various key types.

use crate::stdlib::value::Value;
use crate::error::CursedError;
use std::collections::HashMap;
use base64::{Engine as _, engine::general_purpose};
use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey, DecodePrivateKey, DecodePublicKey};
use rsa::pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey, DecodeRsaPrivateKey, DecodeRsaPublicKey};
use p256::{SecretKey as P256SecretKey, PublicKey as P256PublicKey};
use p384::{SecretKey as P384SecretKey, PublicKey as P384PublicKey};
use ed25519_dalek::{SigningKey, VerifyingKey};
use x25519_dalek::{StaticSecret, PublicKey as X25519PublicKey};
use elliptic_curve::pkcs8::{EncodePrivateKey as EcEncodePrivateKey, EncodePublicKey as EcEncodePublicKey};
use elliptic_curve::pkcs8::{DecodePrivateKey as EcDecodePrivateKey, DecodePublicKey as EcDecodePublicKey};
use serde_json;

/// Supported serialization formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SerializationFormat {
    /// PEM format (Base64 with headers)
    Pem,
    /// DER format (binary ASN.1)
    Der,
    /// JSON Web Key format
    Jwk,
    /// SSH public key format
    Ssh,
    /// Raw bytes format
    Raw,
    /// Hexadecimal string format
    Hex,
}

impl SerializationFormat {
    pub fn name(&self) -> &'static str {
        match self {
            SerializationFormat::Pem => "PEM",
            SerializationFormat::Der => "DER",
            SerializationFormat::Jwk => "JWK",
            SerializationFormat::Ssh => "SSH",
            SerializationFormat::Raw => "Raw",
            SerializationFormat::Hex => "Hex",
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            SerializationFormat::Pem => "Privacy-Enhanced Mail format (Base64 with headers)",
            SerializationFormat::Der => "Distinguished Encoding Rules (binary ASN.1)",
            SerializationFormat::Jwk => "JSON Web Key format",
            SerializationFormat::Ssh => "SSH public key format",
            SerializationFormat::Raw => "Raw binary bytes",
            SerializationFormat::Hex => "Hexadecimal string representation",
        }
    }
    
    pub fn file_extension(&self) -> &'static str {
        match self {
            SerializationFormat::Pem => ".pem",
            SerializationFormat::Der => ".der",
            SerializationFormat::Jwk => ".jwk",
            SerializationFormat::Ssh => ".pub",
            SerializationFormat::Raw => ".bin",
            SerializationFormat::Hex => ".hex",
        }
    }
    
    pub fn from_name(name: &str) -> Result<Self, CursedError> {
        match name.to_uppercase().as_str() {
            "PEM" => Ok(SerializationFormat::Pem),
            "DER" => Ok(SerializationFormat::Der),
            "JWK" => Ok(SerializationFormat::Jwk),
            "SSH" => Ok(SerializationFormat::Ssh),
            "RAW" => Ok(SerializationFormat::Raw),
            "HEX" => Ok(SerializationFormat::Hex),
            _ => Err(CursedError::InvalidArgument(format!("Unsupported serialization format: {}", name))),
        }
    }
}

/// Key type for serialization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyType {
    RsaPrivate,
    RsaPublic,
    EcdsaP256Private,
    EcdsaP256Public,
    EcdsaP384Private,
    EcdsaP384Public,
    Ed25519Private,
    Ed25519Public,
    X25519Private,
    X25519Public,
}

impl KeyType {
    pub fn name(&self) -> &'static str {
        match self {
            KeyType::RsaPrivate => "RSA-Private",
            KeyType::RsaPublic => "RSA-Public",
            KeyType::EcdsaP256Private => "ECDSA-P256-Private",
            KeyType::EcdsaP256Public => "ECDSA-P256-Public",
            KeyType::EcdsaP384Private => "ECDSA-P384-Private",
            KeyType::EcdsaP384Public => "ECDSA-P384-Public",
            KeyType::Ed25519Private => "Ed25519-Private",
            KeyType::Ed25519Public => "Ed25519-Public",
            KeyType::X25519Private => "X25519-Private",
            KeyType::X25519Public => "X25519-Public",
        }
    }
    
    pub fn algorithm(&self) -> &'static str {
        match self {
            KeyType::RsaPrivate | KeyType::RsaPublic => "RSA",
            KeyType::EcdsaP256Private | KeyType::EcdsaP256Public => "ECDSA-P256",
            KeyType::EcdsaP384Private | KeyType::EcdsaP384Public => "ECDSA-P384",
            KeyType::Ed25519Private | KeyType::Ed25519Public => "Ed25519",
            KeyType::X25519Private | KeyType::X25519Public => "X25519",
        }
    }
    
    pub fn is_private(&self) -> bool {
        match self {
            KeyType::RsaPrivate | 
            KeyType::EcdsaP256Private | 
            KeyType::EcdsaP384Private | 
            KeyType::Ed25519Private | 
            KeyType::X25519Private => true,
            _ => false,
        }
    }
    
    pub fn from_name(name: &str) -> Result<Self, CursedError> {
        match name.to_uppercase().as_str() {
            "RSA-PRIVATE" => Ok(KeyType::RsaPrivate),
            "RSA-PUBLIC" => Ok(KeyType::RsaPublic),
            "ECDSA-P256-PRIVATE" => Ok(KeyType::EcdsaP256Private),
            "ECDSA-P256-PUBLIC" => Ok(KeyType::EcdsaP256Public),
            "ECDSA-P384-PRIVATE" => Ok(KeyType::EcdsaP384Private),
            "ECDSA-P384-PUBLIC" => Ok(KeyType::EcdsaP384Public),
            "ED25519-PRIVATE" => Ok(KeyType::Ed25519Private),
            "ED25519-PUBLIC" => Ok(KeyType::Ed25519Public),
            "X25519-PRIVATE" => Ok(KeyType::X25519Private),
            "X25519-PUBLIC" => Ok(KeyType::X25519Public),
            _ => Err(CursedError::InvalidArgument(format!("Unsupported key type: {}", name))),
        }
    }
}

/// Serialization result container
#[derive(Debug, Clone)]
pub struct SerializationResult {
    pub format: SerializationFormat,
    pub key_type: KeyType,
    pub serialized_data: Vec<u8>,
    pub encoding: String, // Base64, hex, etc.
}

impl SerializationResult {
    pub fn new(
        format: SerializationFormat,
        key_type: KeyType,
        serialized_data: Vec<u8>,
    ) -> Self {
        let encoding = match format {
            SerializationFormat::Pem => "Base64 with PEM headers".to_string(),
            SerializationFormat::Der => "Binary DER".to_string(),
            SerializationFormat::Jwk => "JSON".to_string(),
            SerializationFormat::Ssh => "Base64 SSH format".to_string(),
            SerializationFormat::Raw => "Binary".to_string(),
            SerializationFormat::Hex => "Hexadecimal".to_string(),
        };
        
        Self {
            format,
            key_type,
            serialized_data,
            encoding,
        }
    }
    
    pub fn to_value(&self) -> Result<Value, CursedError> {
        let mut map = HashMap::new();
        
        map.insert("format".to_string(), Value::String(self.format.name().to_string()));
        map.insert("key_type".to_string(), Value::String(self.key_type.name().to_string()));
        map.insert("encoding".to_string(), Value::String(self.encoding.clone()));
        
        // Include both raw bytes and appropriate string representation
        map.insert("raw_bytes".to_string(), Value::String(hex::encode(&self.serialized_data)));
        
        match self.format {
            SerializationFormat::Pem | SerializationFormat::Ssh => {
                map.insert("data".to_string(), Value::String(String::from_utf8_lossy(&self.serialized_data).to_string()));
            },
            SerializationFormat::Jwk => {
                map.insert("data".to_string(), Value::String(String::from_utf8_lossy(&self.serialized_data).to_string()));
            },
            SerializationFormat::Hex => {
                map.insert("data".to_string(), Value::String(hex::encode(&self.serialized_data)));
            },
            SerializationFormat::Der | SerializationFormat::Raw => {
                map.insert("data".to_string(), Value::String(general_purpose::STANDARD.encode(&self.serialized_data)));
            },
        }
        
        Ok(Value::Object(map))
    }
}

/// Serialize key to format
pub fn serialize_key(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 3 {
        return Err(CursedError::InvalidArgument("Key serialization requires: key_type, key_data, format".to_string()));
    }
    
    let key_type_name = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Key type must be a string".to_string())),
    };
    
    let key_data_hex = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Key data must be a string".to_string())),
    };
    
    let format_name = match &args[2] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Format must be a string".to_string())),
    };
    
    let key_type = KeyType::from_name(&key_type_name)?;
    let format = SerializationFormat::from_name(&format_name)?;
    let key_data = hex::decode(key_data_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid key data hex: {}", e)))?;
    
    match key_type {
        KeyType::RsaPrivate => serialize_rsa_private_key(&key_data, format),
        KeyType::RsaPublic => serialize_rsa_public_key(&key_data, format),
        KeyType::EcdsaP256Private => serialize_p256_private_key(&key_data, format),
        KeyType::EcdsaP256Public => serialize_p256_public_key(&key_data, format),
        KeyType::EcdsaP384Private => serialize_p384_private_key(&key_data, format),
        KeyType::EcdsaP384Public => serialize_p384_public_key(&key_data, format),
        KeyType::Ed25519Private => serialize_ed25519_private_key(&key_data, format),
        KeyType::Ed25519Public => serialize_ed25519_public_key(&key_data, format),
        KeyType::X25519Private => serialize_x25519_private_key(&key_data, format),
        KeyType::X25519Public => serialize_x25519_public_key(&key_data, format),
    }
}

/// Serialize RSA private key
fn serialize_rsa_private_key(key_data: &[u8], format: SerializationFormat) -> Result<Value, CursedError> {
    let private_key = RsaPrivateKey::from_pkcs8_der(key_data)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode RSA private key: {}", e)))?;
    
    let serialized = match format {
        SerializationFormat::Pem => {
            private_key.to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode RSA private key to PEM: {}", e)))?
                .as_bytes().to_vec()
        },
        SerializationFormat::Der => {
            private_key.to_pkcs8_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode RSA private key to DER: {}", e)))?
                .as_bytes().to_vec()
        },
        SerializationFormat::Hex => {
            let der = private_key.to_pkcs8_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode RSA private key to DER: {}", e)))?;
            hex::encode(der.as_bytes()).into_bytes()
        },
        SerializationFormat::Raw => {
            private_key.to_pkcs8_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode RSA private key to DER: {}", e)))?
                .as_bytes().to_vec()
        },
        _ => return Err(CursedError::NotImplemented(format!("{} format not implemented for RSA private keys", format.name()))),
    };
    
    let result = SerializationResult::new(format, KeyType::RsaPrivate, serialized);
    result.to_value()
}

/// Serialize RSA public key
fn serialize_rsa_public_key(key_data: &[u8], format: SerializationFormat) -> Result<Value, CursedError> {
    let public_key = RsaPublicKey::from_public_key_der(key_data)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode RSA public key: {}", e)))?;
    
    let serialized = match format {
        SerializationFormat::Pem => {
            public_key.to_public_key_pem(rsa::pkcs8::LineEnding::LF)
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode RSA public key to PEM: {}", e)))?
                .as_bytes().to_vec()
        },
        SerializationFormat::Der => {
            public_key.to_public_key_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode RSA public key to DER: {}", e)))?
                .as_bytes().to_vec()
        },
        SerializationFormat::Hex => {
            let der = public_key.to_public_key_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode RSA public key to DER: {}", e)))?;
            hex::encode(der.as_bytes()).into_bytes()
        },
        SerializationFormat::Raw => {
            public_key.to_public_key_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode RSA public key to DER: {}", e)))?
                .as_bytes().to_vec()
        },
        SerializationFormat::Ssh => {
            create_ssh_rsa_public_key(&public_key)?
        },
        _ => return Err(CursedError::NotImplemented(format!("{} format not implemented for RSA public keys", format.name()))),
    };
    
    let result = SerializationResult::new(format, KeyType::RsaPublic, serialized);
    result.to_value()
}

/// Create SSH format for RSA public key
fn create_ssh_rsa_public_key(public_key: &RsaPublicKey) -> Result<Vec<u8>, CursedError> {
    // SSH RSA public key format: ssh-rsa <base64-encoded-key> [comment]
    let der = public_key.to_public_key_der()
        .map_err(|e| CursedError::CryptoError(format!("Failed to encode RSA public key: {}", e)))?;
    
    let base64_key = general_purpose::STANDARD.encode(der.as_bytes());
    let ssh_key = format!("ssh-rsa {} cursed-generated-key", base64_key);
    
    Ok(ssh_key.into_bytes())
}

/// Serialize P-256 private key
fn serialize_p256_private_key(key_data: &[u8], format: SerializationFormat) -> Result<Value, CursedError> {
    let private_key = P256SecretKey::from_pkcs8_der(key_data)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode P-256 private key: {}", e)))?;
    
    let serialized = match format {
        SerializationFormat::Pem => {
            private_key.to_pkcs8_pem(elliptic_curve::pkcs8::LineEnding::LF)
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-256 private key to PEM: {}", e)))?
                .as_bytes().to_vec()
        },
        SerializationFormat::Der => {
            private_key.to_pkcs8_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-256 private key to DER: {}", e)))?
                .as_bytes().to_vec()
        },
        SerializationFormat::Hex => {
            let der = private_key.to_pkcs8_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-256 private key to DER: {}", e)))?;
            hex::encode(der.as_bytes()).into_bytes()
        },
        SerializationFormat::Raw => {
            private_key.to_bytes().to_vec()
        },
        _ => return Err(CursedError::NotImplemented(format!("{} format not implemented for P-256 private keys", format.name()))),
    };
    
    let result = SerializationResult::new(format, KeyType::EcdsaP256Private, serialized);
    result.to_value()
}

/// Serialize P-256 public key
fn serialize_p256_public_key(key_data: &[u8], format: SerializationFormat) -> Result<Value, CursedError> {
    let public_key = P256PublicKey::from_public_key_der(key_data)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode P-256 public key: {}", e)))?;
    
    let serialized = match format {
        SerializationFormat::Pem => {
            public_key.to_public_key_pem(elliptic_curve::pkcs8::LineEnding::LF)
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-256 public key to PEM: {}", e)))?
                .as_bytes().to_vec()
        },
        SerializationFormat::Der => {
            public_key.to_public_key_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-256 public key to DER: {}", e)))?
                .as_bytes().to_vec()
        },
        SerializationFormat::Hex => {
            let der = public_key.to_public_key_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-256 public key to DER: {}", e)))?;
            hex::encode(der.as_bytes()).into_bytes()
        },
        SerializationFormat::Raw => {
            use elliptic_curve::sec1::ToEncodedPoint;
            public_key.to_encoded_point(false).as_bytes().to_vec()
        },
        _ => return Err(CursedError::NotImplemented(format!("{} format not implemented for P-256 public keys", format.name()))),
    };
    
    let result = SerializationResult::new(format, KeyType::EcdsaP256Public, serialized);
    result.to_value()
}

/// Serialize P-384 private key
fn serialize_p384_private_key(key_data: &[u8], format: SerializationFormat) -> Result<Value, CursedError> {
    let private_key = P384SecretKey::from_pkcs8_der(key_data)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode P-384 private key: {}", e)))?;
    
    let serialized = match format {
        SerializationFormat::Pem => {
            private_key.to_pkcs8_pem(elliptic_curve::pkcs8::LineEnding::LF)
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-384 private key to PEM: {}", e)))?
                .as_bytes().to_vec()
        },
        SerializationFormat::Der => {
            private_key.to_pkcs8_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-384 private key to DER: {}", e)))?
                .as_bytes().to_vec()
        },
        SerializationFormat::Hex => {
            let der = private_key.to_pkcs8_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-384 private key to DER: {}", e)))?;
            hex::encode(der.as_bytes()).into_bytes()
        },
        SerializationFormat::Raw => {
            private_key.to_bytes().to_vec()
        },
        _ => return Err(CursedError::NotImplemented(format!("{} format not implemented for P-384 private keys", format.name()))),
    };
    
    let result = SerializationResult::new(format, KeyType::EcdsaP384Private, serialized);
    result.to_value()
}

/// Serialize P-384 public key
fn serialize_p384_public_key(key_data: &[u8], format: SerializationFormat) -> Result<Value, CursedError> {
    let public_key = P384PublicKey::from_public_key_der(key_data)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode P-384 public key: {}", e)))?;
    
    let serialized = match format {
        SerializationFormat::Pem => {
            public_key.to_public_key_pem(elliptic_curve::pkcs8::LineEnding::LF)
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-384 public key to PEM: {}", e)))?
                .as_bytes().to_vec()
        },
        SerializationFormat::Der => {
            public_key.to_public_key_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-384 public key to DER: {}", e)))?
                .as_bytes().to_vec()
        },
        SerializationFormat::Hex => {
            let der = public_key.to_public_key_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-384 public key to DER: {}", e)))?;
            hex::encode(der.as_bytes()).into_bytes()
        },
        SerializationFormat::Raw => {
            use elliptic_curve::sec1::ToEncodedPoint;
            public_key.to_encoded_point(false).as_bytes().to_vec()
        },
        _ => return Err(CursedError::NotImplemented(format!("{} format not implemented for P-384 public keys", format.name()))),
    };
    
    let result = SerializationResult::new(format, KeyType::EcdsaP384Public, serialized);
    result.to_value()
}

/// Serialize Ed25519 private key
fn serialize_ed25519_private_key(key_data: &[u8], format: SerializationFormat) -> Result<Value, CursedError> {
    if key_data.len() != 32 {
        return Err(CursedError::InvalidArgument("Ed25519 private key must be 32 bytes".to_string()));
    }
    
    let signing_key = SigningKey::from_bytes(
        key_data.try_into()
            .map_err(|_| CursedError::InvalidArgument("Invalid Ed25519 private key length".to_string()))?
    );
    
    let serialized = match format {
        SerializationFormat::Raw => key_data.to_vec(),
        SerializationFormat::Hex => hex::encode(key_data).into_bytes(),
        _ => return Err(CursedError::NotImplemented(format!("{} format not implemented for Ed25519 private keys", format.name()))),
    };
    
    let result = SerializationResult::new(format, KeyType::Ed25519Private, serialized);
    result.to_value()
}

/// Serialize Ed25519 public key
fn serialize_ed25519_public_key(key_data: &[u8], format: SerializationFormat) -> Result<Value, CursedError> {
    if key_data.len() != 32 {
        return Err(CursedError::InvalidArgument("Ed25519 public key must be 32 bytes".to_string()));
    }
    
    let verifying_key = VerifyingKey::from_bytes(
        key_data.try_into()
            .map_err(|_| CursedError::InvalidArgument("Invalid Ed25519 public key length".to_string()))?
    ).map_err(|e| CursedError::CryptoError(format!("Invalid Ed25519 public key: {}", e)))?;
    
    let serialized = match format {
        SerializationFormat::Raw => key_data.to_vec(),
        SerializationFormat::Hex => hex::encode(key_data).into_bytes(),
        SerializationFormat::Ssh => {
            let base64_key = general_purpose::STANDARD.encode(key_data);
            let ssh_key = format!("ssh-ed25519 {} cursed-generated-key", base64_key);
            ssh_key.into_bytes()
        },
        _ => return Err(CursedError::NotImplemented(format!("{} format not implemented for Ed25519 public keys", format.name()))),
    };
    
    let result = SerializationResult::new(format, KeyType::Ed25519Public, serialized);
    result.to_value()
}

/// Serialize X25519 private key
fn serialize_x25519_private_key(key_data: &[u8], format: SerializationFormat) -> Result<Value, CursedError> {
    if key_data.len() != 32 {
        return Err(CursedError::InvalidArgument("X25519 private key must be 32 bytes".to_string()));
    }
    
    let serialized = match format {
        SerializationFormat::Raw => key_data.to_vec(),
        SerializationFormat::Hex => hex::encode(key_data).into_bytes(),
        _ => return Err(CursedError::NotImplemented(format!("{} format not implemented for X25519 private keys", format.name()))),
    };
    
    let result = SerializationResult::new(format, KeyType::X25519Private, serialized);
    result.to_value()
}

/// Serialize X25519 public key
fn serialize_x25519_public_key(key_data: &[u8], format: SerializationFormat) -> Result<Value, CursedError> {
    if key_data.len() != 32 {
        return Err(CursedError::InvalidArgument("X25519 public key must be 32 bytes".to_string()));
    }
    
    let serialized = match format {
        SerializationFormat::Raw => key_data.to_vec(),
        SerializationFormat::Hex => hex::encode(key_data).into_bytes(),
        _ => return Err(CursedError::NotImplemented(format!("{} format not implemented for X25519 public keys", format.name()))),
    };
    
    let result = SerializationResult::new(format, KeyType::X25519Public, serialized);
    result.to_value()
}

/// Deserialize key from format
pub fn deserialize_key(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 3 {
        return Err(CursedError::InvalidArgument("Key deserialization requires: format, serialized_data, expected_key_type".to_string()));
    }
    
    let format_name = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Format must be a string".to_string())),
    };
    
    let serialized_data = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Serialized data must be a string".to_string())),
    };
    
    let expected_key_type_name = match &args[2] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Expected key type must be a string".to_string())),
    };
    
    let format = SerializationFormat::from_name(&format_name)?;
    let expected_key_type = KeyType::from_name(&expected_key_type_name)?;
    
    // Convert serialized data to bytes based on format
    let data_bytes = match format {
        SerializationFormat::Pem | SerializationFormat::Ssh => {
            serialized_data.as_bytes().to_vec()
        },
        SerializationFormat::Der | SerializationFormat::Raw => {
            general_purpose::STANDARD.decode(serialized_data)
                .map_err(|e| CursedError::InvalidArgument(format!("Invalid base64 data: {}", e)))?
        },
        SerializationFormat::Hex => {
            hex::decode(serialized_data)
                .map_err(|e| CursedError::InvalidArgument(format!("Invalid hex data: {}", e)))?
        },
        SerializationFormat::Jwk => {
            // JWK deserialization would require more complex parsing
            return Err(CursedError::NotImplemented("JWK deserialization not yet implemented".to_string()));
        },
    };
    
    // Attempt to deserialize based on expected key type
    let validation_result = match expected_key_type {
        KeyType::RsaPrivate => validate_rsa_private_key_data(&data_bytes),
        KeyType::RsaPublic => validate_rsa_public_key_data(&data_bytes),
        KeyType::EcdsaP256Private => validate_p256_private_key_data(&data_bytes),
        KeyType::EcdsaP256Public => validate_p256_public_key_data(&data_bytes),
        KeyType::EcdsaP384Private => validate_p384_private_key_data(&data_bytes),
        KeyType::EcdsaP384Public => validate_p384_public_key_data(&data_bytes),
        KeyType::Ed25519Private => validate_ed25519_private_key_data(&data_bytes),
        KeyType::Ed25519Public => validate_ed25519_public_key_data(&data_bytes),
        KeyType::X25519Private => validate_x25519_private_key_data(&data_bytes),
        KeyType::X25519Public => validate_x25519_public_key_data(&data_bytes),
    };
    
    let mut result = HashMap::new();
    result.insert("format".to_string(), Value::String(format.name().to_string()));
    result.insert("key_type".to_string(), Value::String(expected_key_type.name().to_string()));
    result.insert("valid".to_string(), Value::Boolean(validation_result.is_ok()));
    result.insert("key_data".to_string(), Value::String(hex::encode(&data_bytes)));
    
    if let Err(error_msg) = validation_result {
        result.insert("error".to_string(), Value::String(error_msg));
    }
    
    Ok(Value::Object(result))
}

/// Validation helper functions
fn validate_rsa_private_key_data(data: &[u8]) -> Result<(), String> {
    RsaPrivateKey::from_pkcs8_der(data)
        .map_err(|e| format!("Invalid RSA private key: {}", e))?;
    Ok(())
}

fn validate_rsa_public_key_data(data: &[u8]) -> Result<(), String> {
    RsaPublicKey::from_public_key_der(data)
        .map_err(|e| format!("Invalid RSA public key: {}", e))?;
    Ok(())
}

fn validate_p256_private_key_data(data: &[u8]) -> Result<(), String> {
    P256SecretKey::from_pkcs8_der(data)
        .map_err(|e| format!("Invalid P-256 private key: {}", e))?;
    Ok(())
}

fn validate_p256_public_key_data(data: &[u8]) -> Result<(), String> {
    P256PublicKey::from_public_key_der(data)
        .map_err(|e| format!("Invalid P-256 public key: {}", e))?;
    Ok(())
}

fn validate_p384_private_key_data(data: &[u8]) -> Result<(), String> {
    P384SecretKey::from_pkcs8_der(data)
        .map_err(|e| format!("Invalid P-384 private key: {}", e))?;
    Ok(())
}

fn validate_p384_public_key_data(data: &[u8]) -> Result<(), String> {
    P384PublicKey::from_public_key_der(data)
        .map_err(|e| format!("Invalid P-384 public key: {}", e))?;
    Ok(())
}

fn validate_ed25519_private_key_data(data: &[u8]) -> Result<(), String> {
    if data.len() != 32 {
        return Err(format!("Ed25519 private key must be 32 bytes, got {}", data.len()));
    }
    Ok(())
}

fn validate_ed25519_public_key_data(data: &[u8]) -> Result<(), String> {
    if data.len() != 32 {
        return Err(format!("Ed25519 public key must be 32 bytes, got {}", data.len()));
    }
    
    let key_array: [u8; 32] = data.try_into()
        .map_err(|_| "Invalid key length".to_string())?;
    
    VerifyingKey::from_bytes(&key_array)
        .map_err(|e| format!("Invalid Ed25519 public key: {}", e))?;
    
    Ok(())
}

fn validate_x25519_private_key_data(data: &[u8]) -> Result<(), String> {
    if data.len() != 32 {
        return Err(format!("X25519 private key must be 32 bytes, got {}", data.len()));
    }
    Ok(())
}

fn validate_x25519_public_key_data(data: &[u8]) -> Result<(), String> {
    if data.len() != 32 {
        return Err(format!("X25519 public key must be 32 bytes, got {}", data.len()));
    }
    Ok(())
}

/// List supported serialization formats
pub fn list_serialization_formats() -> Vec<String> {
    vec![
        SerializationFormat::Pem.name().to_string(),
        SerializationFormat::Der.name().to_string(),
        SerializationFormat::Jwk.name().to_string(),
        SerializationFormat::Ssh.name().to_string(),
        SerializationFormat::Raw.name().to_string(),
        SerializationFormat::Hex.name().to_string(),
    ]
}

/// Get format compatibility matrix
pub fn get_format_compatibility() -> HashMap<String, Vec<String>> {
    let mut compatibility = HashMap::new();
    
    compatibility.insert(
        "RSA".to_string(),
        vec!["PEM".to_string(), "DER".to_string(), "SSH".to_string(), "Hex".to_string(), "Raw".to_string()]
    );
    
    compatibility.insert(
        "ECDSA".to_string(),
        vec!["PEM".to_string(), "DER".to_string(), "Hex".to_string(), "Raw".to_string()]
    );
    
    compatibility.insert(
        "Ed25519".to_string(),
        vec!["SSH".to_string(), "Hex".to_string(), "Raw".to_string()]
    );
    
    compatibility.insert(
        "X25519".to_string(),
        vec!["Hex".to_string(), "Raw".to_string()]
    );
    
    compatibility
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_from_name() {
        assert_eq!(SerializationFormat::from_name("PEM").unwrap(), SerializationFormat::Pem);
        assert_eq!(SerializationFormat::from_name("der").unwrap(), SerializationFormat::Der);
        assert!(SerializationFormat::from_name("invalid").is_err());
    }

    #[test]
    fn test_key_type_from_name() {
        assert_eq!(KeyType::from_name("RSA-PRIVATE").unwrap(), KeyType::RsaPrivate);
        assert_eq!(KeyType::from_name("ed25519-public").unwrap(), KeyType::Ed25519Public);
        assert!(KeyType::from_name("invalid").is_err());
    }

    #[test]
    fn test_key_type_properties() {
        assert!(KeyType::RsaPrivate.is_private());
        assert!(!KeyType::RsaPublic.is_private());
        assert_eq!(KeyType::RsaPrivate.algorithm(), "RSA");
        assert_eq!(KeyType::Ed25519Public.algorithm(), "Ed25519");
    }

    #[test]
    fn test_validate_ed25519_key_data() {
        let valid_key = vec![0u8; 32];
        assert!(validate_ed25519_private_key_data(&valid_key).is_ok());
        assert!(validate_ed25519_public_key_data(&valid_key).is_ok());
        
        let invalid_key = vec![0u8; 16];
        assert!(validate_ed25519_private_key_data(&invalid_key).is_err());
        assert!(validate_ed25519_public_key_data(&invalid_key).is_err());
    }

    #[test]
    fn test_list_serialization_formats() {
        let formats = list_serialization_formats();
        assert!(formats.contains(&"PEM".to_string()));
        assert!(formats.contains(&"DER".to_string()));
        assert!(formats.contains(&"SSH".to_string()));
    }

    #[test]
    fn test_get_format_compatibility() {
        let compatibility = get_format_compatibility();
        assert!(compatibility.contains_key("RSA"));
        assert!(compatibility.contains_key("Ed25519"));
        
        let rsa_formats = &compatibility["RSA"];
        assert!(rsa_formats.contains(&"PEM".to_string()));
        assert!(rsa_formats.contains(&"SSH".to_string()));
    }
}
