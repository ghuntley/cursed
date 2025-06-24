// Key serialization utilities
// 
// Provides comprehensive key serialization functions for the CURSED stdlib.
// Supports PEM, DER, JWK, SSH, and raw formats for various key types.

use crate::stdlib::value::Value;
use crate::error::CursedError;
use crate::error::Error;
use std::collections::HashMap;
use base64::{Engine as _, engine::general_purpose};
use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey, DecodePrivateKey, DecodePublicKey};
use rsa::pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey, DecodeRsaPrivateKey, DecodeRsaPublicKey};
use p256::{SecretKey as P256SecretKey, PublicKey as P256PublicKey};
use p384::{SecretKey as P384SecretKey, PublicKey as P384PublicKey};
use ed25519_dalek::{SigningKey, VerifyingKey};
use x25519_dalek::{EphemeralSecret, PublicKey as X25519PublicKey};
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
    
    pub fn from_name(name: &str) -> Result<(), Error> {
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
    
    pub fn from_name(name: &str) -> Result<(), Error> {
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
    
    pub fn to_value(&self) -> Result<(), Error> {
        let mut map = HashMap::new();
        
        map.insert("format".to_string(), Value::String(self.format.to_string()().to_string()));
        map.insert("key_type".to_string(), Value::String(self.key_type.to_string()().to_string()));
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
pub fn serialize_key(args: Vec<Value>) -> Result<(), Error> {
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
fn serialize_rsa_private_key(key_data: &[u8], format: SerializationFormat) -> Result<(), Error> {
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
        SerializationFormat::Jwk => {
            create_jwk_rsa_private_key(&private_key)?
        },
        SerializationFormat::Ssh => {
            return Err(CursedError::InvalidArgument("SSH format not supported for private keys".to_string()));
        },
    };
    
    let result = SerializationResult::new(format, KeyType::RsaPrivate, serialized);
    result.to_value()
}

/// Serialize RSA public key
fn serialize_rsa_public_key(key_data: &[u8], format: SerializationFormat) -> Result<(), Error> {
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
        SerializationFormat::Jwk => {
            create_jwk_rsa_public_key(&public_key)?
        },
    };
    
    let result = SerializationResult::new(format, KeyType::RsaPublic, serialized);
    result.to_value()
}

/// Create SSH format for RSA public key
fn create_ssh_rsa_public_key(public_key: &RsaPublicKey) -> Result<(), Error> {
    // SSH RSA public key format: ssh-rsa <base64-encoded-key> [comment]
    let der = public_key.to_public_key_der()
        .map_err(|e| CursedError::CryptoError(format!("Failed to encode RSA public key: {}", e)))?;
    
    let base64_key = general_purpose::STANDARD.encode(der.as_bytes());
    let ssh_key = format!("ssh-rsa {} cursed-generated-key", base64_key);
    
    Ok(ssh_key.into_bytes())
}

/// Create JWK format for RSA public key
fn create_jwk_rsa_public_key(public_key: &RsaPublicKey) -> Result<(), Error> {
    use rsa::traits::PublicKeyParts;
    
    let n = public_key.n();
    let e = public_key.e();
    
    // Convert to base64url encoding (JWK standard)
    let n_bytes = n.to_bytes_be();
    let e_bytes = e.to_bytes_be();
    
    let n_b64 = general_purpose::URL_SAFE_NO_PAD.encode(&n_bytes);
    let e_b64 = general_purpose::URL_SAFE_NO_PAD.encode(&e_bytes);
    
    let jwk = serde_json::json!({
        "kty": "RSA",
        "use": "sig",
        "key_ops": ["verify"],
        "alg": "RS256",
        "n": n_b64,
        "e": e_b64
    });
    
    Ok(jwk.to_string().into_bytes())
}

/// Create JWK format for RSA private key  
fn create_jwk_rsa_private_key(private_key: &RsaPrivateKey) -> Result<(), Error> {
    use rsa::traits::PublicKeyParts;
    
    let public_key = private_key.to_public_key();
    let n = public_key.n();
    let e = public_key.e();
    let d = private_key.d();
    
    // Convert to base64url encoding
    let n_bytes = n.to_bytes_be();
    let e_bytes = e.to_bytes_be();
    let d_bytes = d.to_bytes_be();
    
    let n_b64 = general_purpose::URL_SAFE_NO_PAD.encode(&n_bytes);
    let e_b64 = general_purpose::URL_SAFE_NO_PAD.encode(&e_bytes);
    let d_b64 = general_purpose::URL_SAFE_NO_PAD.encode(&d_bytes);
    
    let jwk = serde_json::json!({
        "kty": "RSA",
        "use": "sig",
        "key_ops": ["sign"],
        "alg": "RS256",
        "n": n_b64,
        "e": e_b64,
        "d": d_b64
    });
    
    Ok(jwk.to_string().into_bytes())
}

/// Create JWK format for P-256 private key
fn create_jwk_p256_private_key(private_key: &P256SecretKey) -> Result<(), Error> {
    use elliptic_curve::sec1::ToEncodedPoint;
    
    let public_key = private_key.public_key();
    let point = public_key.to_encoded_point(false);
    
    // Extract x and y coordinates (skip the first byte which is 0x04 for uncompressed)
    let coords = point.as_bytes();
    if coords.len() != 65 || coords[0] != 0x04 {
        return Err(CursedError::CryptoError("Invalid P-256 public key point".to_string()));
    }
    
    let x = &coords[1..33];
    let y = &coords[33..65];
    let d = private_key.to_bytes();
    
    let x_b64 = general_purpose::URL_SAFE_NO_PAD.encode(x);
    let y_b64 = general_purpose::URL_SAFE_NO_PAD.encode(y);
    let d_b64 = general_purpose::URL_SAFE_NO_PAD.encode(&d);
    
    let jwk = serde_json::json!({
        "kty": "EC",
        "crv": "P-256",
        "use": "sig",
        "key_ops": ["sign"],
        "alg": "ES256",
        "x": x_b64,
        "y": y_b64,
        "d": d_b64
    });
    
    Ok(jwk.to_string().into_bytes())
}

/// Create JWK format for P-256 public key
fn create_jwk_p256_public_key(public_key: &P256PublicKey) -> Result<(), Error> {
    use elliptic_curve::sec1::ToEncodedPoint;
    
    let point = public_key.to_encoded_point(false);
    let coords = point.as_bytes();
    
    if coords.len() != 65 || coords[0] != 0x04 {
        return Err(CursedError::CryptoError("Invalid P-256 public key point".to_string()));
    }
    
    let x = &coords[1..33];
    let y = &coords[33..65];
    
    let x_b64 = general_purpose::URL_SAFE_NO_PAD.encode(x);
    let y_b64 = general_purpose::URL_SAFE_NO_PAD.encode(y);
    
    let jwk = serde_json::json!({
        "kty": "EC",
        "crv": "P-256",
        "use": "sig",
        "key_ops": ["verify"],
        "alg": "ES256",
        "x": x_b64,
        "y": y_b64
    });
    
    Ok(jwk.to_string().into_bytes())
}

/// Create SSH format for P-256 public key
fn create_ssh_p256_public_key(public_key: &P256PublicKey) -> Result<(), Error> {
    use elliptic_curve::sec1::ToEncodedPoint;
    
    let point = public_key.to_encoded_point(false);
    let base64_key = general_purpose::STANDARD.encode(point.as_bytes());
    let ssh_key = format!("ecdsa-sha2-nistp256 {} cursed-generated-key", base64_key);
    
    Ok(ssh_key.into_bytes())
}

/// Create JWK format for P-384 private key
fn create_jwk_p384_private_key(private_key: &P384SecretKey) -> Result<(), Error> {
    use elliptic_curve::sec1::ToEncodedPoint;
    
    let public_key = private_key.public_key();
    let point = public_key.to_encoded_point(false);
    
    // Extract x and y coordinates (skip the first byte which is 0x04 for uncompressed)
    let coords = point.as_bytes();
    if coords.len() != 97 || coords[0] != 0x04 {
        return Err(CursedError::CryptoError("Invalid P-384 public key point".to_string()));
    }
    
    let x = &coords[1..49];
    let y = &coords[49..97];
    let d = private_key.to_bytes();
    
    let x_b64 = general_purpose::URL_SAFE_NO_PAD.encode(x);
    let y_b64 = general_purpose::URL_SAFE_NO_PAD.encode(y);
    let d_b64 = general_purpose::URL_SAFE_NO_PAD.encode(&d);
    
    let jwk = serde_json::json!({
        "kty": "EC",
        "crv": "P-384",
        "use": "sig",
        "key_ops": ["sign"],
        "alg": "ES384",
        "x": x_b64,
        "y": y_b64,
        "d": d_b64
    });
    
    Ok(jwk.to_string().into_bytes())
}

/// Create JWK format for P-384 public key
fn create_jwk_p384_public_key(public_key: &P384PublicKey) -> Result<(), Error> {
    use elliptic_curve::sec1::ToEncodedPoint;
    
    let point = public_key.to_encoded_point(false);
    let coords = point.as_bytes();
    
    if coords.len() != 97 || coords[0] != 0x04 {
        return Err(CursedError::CryptoError("Invalid P-384 public key point".to_string()));
    }
    
    let x = &coords[1..49];
    let y = &coords[49..97];
    
    let x_b64 = general_purpose::URL_SAFE_NO_PAD.encode(x);
    let y_b64 = general_purpose::URL_SAFE_NO_PAD.encode(y);
    
    let jwk = serde_json::json!({
        "kty": "EC",
        "crv": "P-384",
        "use": "sig",
        "key_ops": ["verify"],
        "alg": "ES384",
        "x": x_b64,
        "y": y_b64
    });
    
    Ok(jwk.to_string().into_bytes())
}

/// Create SSH format for P-384 public key
fn create_ssh_p384_public_key(public_key: &P384PublicKey) -> Result<(), Error> {
    use elliptic_curve::sec1::ToEncodedPoint;
    
    let point = public_key.to_encoded_point(false);
    let base64_key = general_purpose::STANDARD.encode(point.as_bytes());
    let ssh_key = format!("ecdsa-sha2-nistp384 {} cursed-generated-key", base64_key);
    
    Ok(ssh_key.into_bytes())
}

/// Create PEM format for Ed25519 private key
fn create_ed25519_private_key_pem(key_data: &[u8]) -> Result<(), Error> {
    let der = create_ed25519_private_key_der(key_data)?;
    let pem = format!(
        "-----BEGIN PRIVATE KEY-----\n{}\n-----END PRIVATE KEY-----\n",
        general_purpose::STANDARD.encode(&der).chars()
            .collect::<Vec<char>>()
            .chunks(64)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    );
    Ok(pem.into_bytes())
}

/// Create DER format for Ed25519 private key  
fn create_ed25519_private_key_der(key_data: &[u8]) -> Result<(), Error> {
    // Ed25519 PKCS#8 DER format
    let mut der = Vec::new();
    
    // SEQUENCE tag and length (will be calculated)
    der.push(0x30);
    
    // Version: INTEGER 0
    der.extend_from_slice(&[0x02, 0x01, 0x00]);
    
    // AlgorithmIdentifier SEQUENCE
    der.extend_from_slice(&[0x30, 0x05]);
    // Ed25519 OID: 1.3.101.112
    der.extend_from_slice(&[0x06, 0x03, 0x2b, 0x65, 0x70]);
    
    // PrivateKey OCTET STRING
    der.extend_from_slice(&[0x04, 0x22]); // Length: 34 bytes
    der.extend_from_slice(&[0x04, 0x20]); // OCTET STRING, 32 bytes
    der.extend_from_slice(key_data);
    
    // Update sequence length
    let total_len = der.len() - 1;
    der[1] = total_len as u8;
    
    Ok(der)
}

/// Create PEM format for Ed25519 public key
fn create_ed25519_public_key_pem(key_data: &[u8]) -> Result<(), Error> {
    let der = create_ed25519_public_key_der(key_data)?;
    let pem = format!(
        "-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----\n",
        general_purpose::STANDARD.encode(&der).chars()
            .collect::<Vec<char>>()
            .chunks(64)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    );
    Ok(pem.into_bytes())
}

/// Create DER format for Ed25519 public key
fn create_ed25519_public_key_der(key_data: &[u8]) -> Result<(), Error> {
    // Ed25519 SubjectPublicKeyInfo DER format
    let mut der = Vec::new();
    
    // SEQUENCE tag and length
    der.push(0x30);
    der.push(0x2a); // Total length: 42 bytes
    
    // AlgorithmIdentifier SEQUENCE
    der.extend_from_slice(&[0x30, 0x05]);
    // Ed25519 OID: 1.3.101.112
    der.extend_from_slice(&[0x06, 0x03, 0x2b, 0x65, 0x70]);
    
    // subjectPublicKey BIT STRING
    der.extend_from_slice(&[0x03, 0x21, 0x00]); // BIT STRING, 33 bytes (32 + padding)
    der.extend_from_slice(key_data);
    
    Ok(der)
}

/// Create JWK format for Ed25519 private key
fn create_jwk_ed25519_private_key(key_data: &[u8]) -> Result<(), Error> {
    let signing_key = SigningKey::from_bytes(
        key_data.try_into()
            .map_err(|_| CursedError::InvalidArgument("Invalid Ed25519 private key length".to_string()))?
    );
    
    let public_key = signing_key.verifying_key();
    let d_b64 = general_purpose::URL_SAFE_NO_PAD.encode(key_data);
    let x_b64 = general_purpose::URL_SAFE_NO_PAD.encode(public_key.as_bytes());
    
    let jwk = serde_json::json!({
        "kty": "OKP",
        "crv": "Ed25519",
        "use": "sig",
        "key_ops": ["sign"],
        "alg": "EdDSA",
        "x": x_b64,
        "d": d_b64
    });
    
    Ok(jwk.to_string().into_bytes())
}

/// Create JWK format for Ed25519 public key
fn create_jwk_ed25519_public_key(key_data: &[u8]) -> Result<(), Error> {
    let x_b64 = general_purpose::URL_SAFE_NO_PAD.encode(key_data);
    
    let jwk = serde_json::json!({
        "kty": "OKP",
        "crv": "Ed25519",
        "use": "sig",
        "key_ops": ["verify"],
        "alg": "EdDSA",
        "x": x_b64
    });
    
    Ok(jwk.to_string().into_bytes())
}

/// Create PEM format for X25519 private key
fn create_x25519_private_key_pem(key_data: &[u8]) -> Result<(), Error> {
    let der = create_x25519_private_key_der(key_data)?;
    let pem = format!(
        "-----BEGIN PRIVATE KEY-----\n{}\n-----END PRIVATE KEY-----\n",
        general_purpose::STANDARD.encode(&der).chars()
            .collect::<Vec<char>>()
            .chunks(64)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    );
    Ok(pem.into_bytes())
}

/// Create DER format for X25519 private key  
fn create_x25519_private_key_der(key_data: &[u8]) -> Result<(), Error> {
    // X25519 PKCS#8 DER format
    let mut der = Vec::new();
    
    // SEQUENCE tag and length
    der.push(0x30);
    
    // Version: INTEGER 0
    der.extend_from_slice(&[0x02, 0x01, 0x00]);
    
    // AlgorithmIdentifier SEQUENCE
    der.extend_from_slice(&[0x30, 0x05]);
    // X25519 OID: 1.3.101.110
    der.extend_from_slice(&[0x06, 0x03, 0x2b, 0x65, 0x6e]);
    
    // PrivateKey OCTET STRING
    der.extend_from_slice(&[0x04, 0x22]); // Length: 34 bytes
    der.extend_from_slice(&[0x04, 0x20]); // OCTET STRING, 32 bytes
    der.extend_from_slice(key_data);
    
    // Update sequence length
    let total_len = der.len() - 1;
    der[1] = total_len as u8;
    
    Ok(der)
}

/// Create PEM format for X25519 public key
fn create_x25519_public_key_pem(key_data: &[u8]) -> Result<(), Error> {
    let der = create_x25519_public_key_der(key_data)?;
    let pem = format!(
        "-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----\n",
        general_purpose::STANDARD.encode(&der).chars()
            .collect::<Vec<char>>()
            .chunks(64)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    );
    Ok(pem.into_bytes())
}

/// Create DER format for X25519 public key
fn create_x25519_public_key_der(key_data: &[u8]) -> Result<(), Error> {
    // X25519 SubjectPublicKeyInfo DER format
    let mut der = Vec::new();
    
    // SEQUENCE tag and length
    der.push(0x30);
    der.push(0x2a); // Total length: 42 bytes
    
    // AlgorithmIdentifier SEQUENCE
    der.extend_from_slice(&[0x30, 0x05]);
    // X25519 OID: 1.3.101.110
    der.extend_from_slice(&[0x06, 0x03, 0x2b, 0x65, 0x6e]);
    
    // subjectPublicKey BIT STRING
    der.extend_from_slice(&[0x03, 0x21, 0x00]); // BIT STRING, 33 bytes (32 + padding)
    der.extend_from_slice(key_data);
    
    Ok(der)
}

/// Create JWK format for X25519 private key
fn create_jwk_x25519_private_key(key_data: &[u8]) -> Result<(), Error> {
    let key_array: [u8; 32] = key_data.try_into()
        .map_err(|_| CursedError::InvalidArgument("Invalid X25519 private key length".to_string()))?;
    let private_key = EphemeralSecret::from(key_array);
    let public_key = X25519PublicKey::from(&private_key);
    
    let d_b64 = general_purpose::URL_SAFE_NO_PAD.encode(key_data);
    let x_b64 = general_purpose::URL_SAFE_NO_PAD.encode(public_key.as_bytes());
    
    let jwk = serde_json::json!({
        "kty": "OKP",
        "crv": "X25519",
        "use": "enc",
        "key_ops": ["deriveKey"],
        "alg": "ECDH-ES",
        "x": x_b64,
        "d": d_b64
    });
    
    Ok(jwk.to_string().into_bytes())
}

/// Create JWK format for X25519 public key
fn create_jwk_x25519_public_key(key_data: &[u8]) -> Result<(), Error> {
    let x_b64 = general_purpose::URL_SAFE_NO_PAD.encode(key_data);
    
    let jwk = serde_json::json!({
        "kty": "OKP",
        "crv": "X25519",
        "use": "enc",
        "key_ops": ["deriveKey"],
        "alg": "ECDH-ES",
        "x": x_b64
    });
    
    Ok(jwk.to_string().into_bytes())
}

/// Serialize P-256 private key
fn serialize_p256_private_key(key_data: &[u8], format: SerializationFormat) -> Result<(), Error> {
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
        SerializationFormat::Jwk => {
            create_jwk_p256_private_key(&private_key)?
        },
        SerializationFormat::Ssh => {
            return Err(CursedError::InvalidArgument("SSH format not supported for private keys".to_string()));
        },
    };
    
    let result = SerializationResult::new(format, KeyType::EcdsaP256Private, serialized);
    result.to_value()
}

/// Serialize P-256 public key
fn serialize_p256_public_key(key_data: &[u8], format: SerializationFormat) -> Result<(), Error> {
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
        SerializationFormat::Jwk => {
            create_jwk_p256_public_key(&public_key)?
        },
        SerializationFormat::Ssh => {
            create_ssh_p256_public_key(&public_key)?
        },
    };
    
    let result = SerializationResult::new(format, KeyType::EcdsaP256Public, serialized);
    result.to_value()
}

/// Serialize P-384 private key
fn serialize_p384_private_key(key_data: &[u8], format: SerializationFormat) -> Result<(), Error> {
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
        SerializationFormat::Jwk => {
            create_jwk_p384_private_key(&private_key)?
        },
        SerializationFormat::Ssh => {
            return Err(CursedError::InvalidArgument("SSH format not supported for private keys".to_string()));
        },
    };
    
    let result = SerializationResult::new(format, KeyType::EcdsaP384Private, serialized);
    result.to_value()
}

/// Serialize P-384 public key
fn serialize_p384_public_key(key_data: &[u8], format: SerializationFormat) -> Result<(), Error> {
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
        SerializationFormat::Jwk => {
            create_jwk_p384_public_key(&public_key)?
        },
        SerializationFormat::Ssh => {
            create_ssh_p384_public_key(&public_key)?
        },
    };
    
    let result = SerializationResult::new(format, KeyType::EcdsaP384Public, serialized);
    result.to_value()
}

/// Serialize Ed25519 private key
fn serialize_ed25519_private_key(key_data: &[u8], format: SerializationFormat) -> Result<(), Error> {
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
        SerializationFormat::Pem => {
            create_ed25519_private_key_pem(key_data)?
        },
        SerializationFormat::Der => {
            create_ed25519_private_key_der(key_data)?
        },
        SerializationFormat::Jwk => {
            create_jwk_ed25519_private_key(key_data)?
        },
        SerializationFormat::Ssh => {
            return Err(CursedError::InvalidArgument("SSH format not supported for private keys".to_string()));
        },
    };
    
    let result = SerializationResult::new(format, KeyType::Ed25519Private, serialized);
    result.to_value()
}

/// Serialize Ed25519 public key
fn serialize_ed25519_public_key(key_data: &[u8], format: SerializationFormat) -> Result<(), Error> {
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
        SerializationFormat::Pem => {
            create_ed25519_public_key_pem(key_data)?
        },
        SerializationFormat::Der => {
            create_ed25519_public_key_der(key_data)?
        },
        SerializationFormat::Jwk => {
            create_jwk_ed25519_public_key(key_data)?
        },
    };
    
    let result = SerializationResult::new(format, KeyType::Ed25519Public, serialized);
    result.to_value()
}

/// Serialize X25519 private key
fn serialize_x25519_private_key(key_data: &[u8], format: SerializationFormat) -> Result<(), Error> {
    if key_data.len() != 32 {
        return Err(CursedError::InvalidArgument("X25519 private key must be 32 bytes".to_string()));
    }
    
    let serialized = match format {
        SerializationFormat::Raw => key_data.to_vec(),
        SerializationFormat::Hex => hex::encode(key_data).into_bytes(),
        SerializationFormat::Pem => {
            create_x25519_private_key_pem(key_data)?
        },
        SerializationFormat::Der => {
            create_x25519_private_key_der(key_data)?
        },
        SerializationFormat::Jwk => {
            create_jwk_x25519_private_key(key_data)?
        },
        SerializationFormat::Ssh => {
            return Err(CursedError::InvalidArgument("SSH format not supported for private keys".to_string()));
        },
    };
    
    let result = SerializationResult::new(format, KeyType::X25519Private, serialized);
    result.to_value()
}

/// Serialize X25519 public key
fn serialize_x25519_public_key(key_data: &[u8], format: SerializationFormat) -> Result<(), Error> {
    if key_data.len() != 32 {
        return Err(CursedError::InvalidArgument("X25519 public key must be 32 bytes".to_string()));
    }
    
    let serialized = match format {
        SerializationFormat::Raw => key_data.to_vec(),
        SerializationFormat::Hex => hex::encode(key_data).into_bytes(),
        SerializationFormat::Pem => {
            create_x25519_public_key_pem(key_data)?
        },
        SerializationFormat::Der => {
            create_x25519_public_key_der(key_data)?
        },
        SerializationFormat::Jwk => {
            create_jwk_x25519_public_key(key_data)?
        },
        SerializationFormat::Ssh => {
            return Err(CursedError::InvalidArgument("SSH format not supported for X25519 keys".to_string()));
        },
    };
    
    let result = SerializationResult::new(format, KeyType::X25519Public, serialized);
    result.to_value()
}

/// Deserialize key from format
pub fn deserialize_key(args: Vec<Value>) -> Result<(), Error> {
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
        SerializationFormat::Pem => {
            parse_pem_to_bytes(&serialized_data)?
        },
        SerializationFormat::Ssh => {
            parse_ssh_to_bytes(&serialized_data)?
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
            parse_jwk_to_bytes(&serialized_data, &expected_key_type)?
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
    result.insert("format".to_string(), Value::String(format.to_string()().to_string()));
    result.insert("key_type".to_string(), Value::String(expected_key_type.to_string()().to_string()));
    result.insert("valid".to_string(), Value::Boolean(validation_result.is_ok()));
    result.insert("key_data".to_string(), Value::String(hex::encode(&data_bytes)));
    
    if let Err(error_msg) = validation_result {
        result.insert("error".to_string(), Value::String(error_msg));
    }
    
    Ok(Value::Object(result))
}

/// Parse PEM to raw bytes
fn parse_pem_to_bytes(pem_data: &str) -> Result<(), Error> {
    // Remove PEM headers and decode base64 content
    let lines: Vec<&str> = pem_data.split("\n").collect();
    
    // Find content between -----BEGIN and -----END lines
    let mut start_idx = None;
    let mut end_idx = None;
    
    for (i, line) in lines.iter().enumerate() {
        if line.starts_with("-----BEGIN") {
            start_idx = Some(i + 1);
        } else if line.starts_with("-----END") {
            end_idx = Some(i);
            break;
        }
    }
    
    let (start, end) = match (start_idx, end_idx) {
        (Some(s), Some(e)) => (s, e),
        _ => return Err(CursedError::InvalidArgument("Invalid PEM format: missing BEGIN/END markers".to_string())),
    };
    
    if start >= end {
        return Err(CursedError::InvalidArgument("Invalid PEM format: empty content".to_string()));
    }
    
    // Concatenate base64 content lines
    let base64_content = lines[start..end].join("");
    
    // Decode base64
    general_purpose::STANDARD.decode(base64_content)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid base64 in PEM: {}", e)))
}

/// Parse SSH key to raw bytes
fn parse_ssh_to_bytes(ssh_data: &str) -> Result<(), Error> {
    // SSH public key format: <algorithm> <base64-key> [comment]
    let parts: Vec<&str> = ssh_data.trim().split_whitespace().collect();
    
    if parts.len() < 2 {
        return Err(CursedError::InvalidArgument("Invalid SSH key format: expected algorithm and key data".to_string()));
    }
    
    let algorithm = parts[0];
    let key_data = parts[1];
    
    // Validate algorithm
    match algorithm {
        "ssh-rsa" | "ssh-ed25519" | "ecdsa-sha2-nistp256" | "ecdsa-sha2-nistp384" => {},
        _ => return Err(CursedError::InvalidArgument(format!("Unsupported SSH key algorithm: {}", algorithm))),
    }
    
    // Decode base64 key data
    general_purpose::STANDARD.decode(key_data)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid base64 in SSH key: {}", e)))
}

/// Parse JWK to raw bytes
fn parse_jwk_to_bytes(jwk_str: &str, expected_key_type: &KeyType) -> Result<(), Error> {
    let jwk: serde_json::Value = serde_json::from_str(jwk_str)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid JWK JSON: {}", e)))?;
    
    let kty = jwk.get("kty")
        .and_then(|v| v.as_str())
        .ok_or_else(|| CursedError::InvalidArgument("Missing 'kty' field in JWK".to_string()))?;
    
    match (kty, expected_key_type) {
        ("RSA", KeyType::RsaPrivate) => {
            let d = jwk.get("d")
                .and_then(|v| v.as_str())
                .ok_or_else(|| CursedError::InvalidArgument("Missing 'd' field for RSA private key".to_string()))?;
            
            general_purpose::URL_SAFE_NO_PAD.decode(d)
                .map_err(|e| CursedError::InvalidArgument(format!("Invalid base64url in RSA private key: {}", e)))
        },
        ("RSA", KeyType::RsaPublic) => {
            let n = jwk.get("n")
                .and_then(|v| v.as_str())
                .ok_or_else(|| CursedError::InvalidArgument("Missing 'n' field for RSA public key".to_string()))?;
            
            general_purpose::URL_SAFE_NO_PAD.decode(n)
                .map_err(|e| CursedError::InvalidArgument(format!("Invalid base64url in RSA public key: {}", e)))
        },
        ("EC", KeyType::EcdsaP256Private) | ("EC", KeyType::EcdsaP384Private) => {
            let d = jwk.get("d")
                .and_then(|v| v.as_str())
                .ok_or_else(|| CursedError::InvalidArgument("Missing 'd' field for EC private key".to_string()))?;
            
            general_purpose::URL_SAFE_NO_PAD.decode(d)
                .map_err(|e| CursedError::InvalidArgument(format!("Invalid base64url in EC private key: {}", e)))
        },
        ("EC", KeyType::EcdsaP256Public) | ("EC", KeyType::EcdsaP384Public) => {
            let x = jwk.get("x")
                .and_then(|v| v.as_str())
                .ok_or_else(|| CursedError::InvalidArgument("Missing 'x' field for EC public key".to_string()))?;
            let y = jwk.get("y")
                .and_then(|v| v.as_str())
                .ok_or_else(|| CursedError::InvalidArgument("Missing 'y' field for EC public key".to_string()))?;
            
            let x_bytes = general_purpose::URL_SAFE_NO_PAD.decode(x)
                .map_err(|e| CursedError::InvalidArgument(format!("Invalid base64url in EC x coordinate: {}", e)))?;
            let y_bytes = general_purpose::URL_SAFE_NO_PAD.decode(y)
                .map_err(|e| CursedError::InvalidArgument(format!("Invalid base64url in EC y coordinate: {}", e)))?;
            
            // Combine x and y coordinates with uncompressed point format (0x04 prefix)
            let mut point = vec![0x04u8];
            point.extend_from_slice(&x_bytes);
            point.extend_from_slice(&y_bytes);
            Ok(point)
        },
        ("OKP", KeyType::Ed25519Private) | ("OKP", KeyType::X25519Private) => {
            let d = jwk.get("d")
                .and_then(|v| v.as_str())
                .ok_or_else(|| CursedError::InvalidArgument("Missing 'd' field for OKP private key".to_string()))?;
            
            general_purpose::URL_SAFE_NO_PAD.decode(d)
                .map_err(|e| CursedError::InvalidArgument(format!("Invalid base64url in OKP private key: {}", e)))
        },
        ("OKP", KeyType::Ed25519Public) | ("OKP", KeyType::X25519Public) => {
            let x = jwk.get("x")
                .and_then(|v| v.as_str())
                .ok_or_else(|| CursedError::InvalidArgument("Missing 'x' field for OKP public key".to_string()))?;
            
            general_purpose::URL_SAFE_NO_PAD.decode(x)
                .map_err(|e| CursedError::InvalidArgument(format!("Invalid base64url in OKP public key: {}", e)))
        },
        _ => Err(CursedError::InvalidArgument(format!("Unsupported key type combination: {} for {:?}", kty, expected_key_type))),
    }
}

/// Validation helper functions
fn validate_rsa_private_key_data(data: &[u8]) -> Result<(), String> {
    // Try PKCS#8 first, then PKCS#1 if that fails
    match RsaPrivateKey::from_pkcs8_der(data) {
        Ok(_) => Ok(()),
        Err(_) => {
            // Try PKCS#1 format
            RsaPrivateKey::from_pkcs1_der(data)
                .map_err(|e| format!("Invalid RSA private key (tried PKCS#8 and PKCS#1): {}", e))?;
            Ok(())
        }
    }
}

fn validate_rsa_public_key_data(data: &[u8]) -> Result<(), String> {
    // Try SubjectPublicKeyInfo first, then PKCS#1 if that fails
    match RsaPublicKey::from_public_key_der(data) {
        Ok(_) => Ok(()),
        Err(_) => {
            // Try PKCS#1 format
            RsaPublicKey::from_pkcs1_der(data)
                .map_err(|e| format!("Invalid RSA public key (tried SubjectPublicKeyInfo and PKCS#1): {}", e))?;
            Ok(())
        }
    }
}

fn validate_p256_private_key_data(data: &[u8]) -> Result<(), String> {
    // Try PKCS#8 first, then raw key if length matches
    match P256SecretKey::from_pkcs8_der(data) {
        Ok(_) => Ok(()),
        Err(_) => {
            if data.len() == 32 {
                // Try as raw 32-byte key
                P256SecretKey::from_slice(data)
                    .map_err(|e| format!("Invalid P-256 private key (tried PKCS#8 and raw): {}", e))?;
                Ok(())
            } else {
                Err(format!("Invalid P-256 private key: wrong length {} (expected 32 for raw or PKCS#8 DER)", data.len()))
            }
        }
    }
}

fn validate_p256_public_key_data(data: &[u8]) -> Result<(), String> {
    // Try SubjectPublicKeyInfo first, then raw key formats
    match P256PublicKey::from_public_key_der(data) {
        Ok(_) => Ok(()),
        Err(_) => {
            if data.len() == 65 && data[0] == 0x04 {
                // Try as uncompressed SEC1 point
                use elliptic_curve::sec1::{FromEncodedPoint, EncodedPoint};
                let point = EncodedPoint::<p256::NistP256>::from_bytes(data)
                    .map_err(|e| format!("Invalid SEC1 encoded point: {}", e))?;
                P256PublicKey::from_encoded_point(&point)
                    .map_err(|e| format!("Invalid P-256 public key point: {}", e))?;
                Ok(())
            } else if data.len() == 33 && (data[0] == 0x02 || data[0] == 0x03) {
                // Try as compressed SEC1 point
                use elliptic_curve::sec1::{FromEncodedPoint, EncodedPoint};
                let point = EncodedPoint::<p256::NistP256>::from_bytes(data)
                    .map_err(|e| format!("Invalid SEC1 encoded point: {}", e))?;
                P256PublicKey::from_encoded_point(&point)
                    .map_err(|e| format!("Invalid P-256 public key point: {}", e))?;
                Ok(())
            } else {
                Err(format!("Invalid P-256 public key: unsupported format or length {}", data.len()))
            }
        }
    }
}

fn validate_p384_private_key_data(data: &[u8]) -> Result<(), String> {
    // Try PKCS#8 first, then raw key if length matches
    match P384SecretKey::from_pkcs8_der(data) {
        Ok(_) => Ok(()),
        Err(_) => {
            if data.len() == 48 {
                // Try as raw 48-byte key
                P384SecretKey::from_slice(data)
                    .map_err(|e| format!("Invalid P-384 private key (tried PKCS#8 and raw): {}", e))?;
                Ok(())
            } else {
                Err(format!("Invalid P-384 private key: wrong length {} (expected 48 for raw or PKCS#8 DER)", data.len()))
            }
        }
    }
}

fn validate_p384_public_key_data(data: &[u8]) -> Result<(), String> {
    // Try SubjectPublicKeyInfo first, then raw key formats
    match P384PublicKey::from_public_key_der(data) {
        Ok(_) => Ok(()),
        Err(_) => {
            if data.len() == 97 && data[0] == 0x04 {
                // Try as uncompressed SEC1 point
                use elliptic_curve::sec1::{FromEncodedPoint, EncodedPoint};
                let point = EncodedPoint::<p384::NistP384>::from_bytes(data)
                    .map_err(|e| format!("Invalid SEC1 encoded point: {}", e))?;
                P384PublicKey::from_encoded_point(&point)
                    .map_err(|e| format!("Invalid P-384 public key point: {}", e))?;
                Ok(())
            } else if data.len() == 49 && (data[0] == 0x02 || data[0] == 0x03) {
                // Try as compressed SEC1 point
                use elliptic_curve::sec1::{FromEncodedPoint, EncodedPoint};
                let point = EncodedPoint::<p384::NistP384>::from_bytes(data)
                    .map_err(|e| format!("Invalid SEC1 encoded point: {}", e))?;
                P384PublicKey::from_encoded_point(&point)
                    .map_err(|e| format!("Invalid P-384 public key point: {}", e))?;
                Ok(())
            } else {
                Err(format!("Invalid P-384 public key: unsupported format or length {}", data.len()))
            }
        }
    }
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
        SerializationFormat::Pem.to_string()().to_string(),
        SerializationFormat::Der.to_string()().to_string(),
        SerializationFormat::Jwk.to_string()().to_string(),
        SerializationFormat::Ssh.to_string()().to_string(),
        SerializationFormat::Raw.to_string()().to_string(),
        SerializationFormat::Hex.to_string()().to_string(),
    ]
}

/// Get format compatibility matrix
pub fn get_format_compatibility() -> HashMap<String, Vec<String>> {
    let mut compatibility = HashMap::new();
    
    // RSA keys support all formats including SSH for public keys
    compatibility.insert(
        "RSA-Private".to_string(),
        vec!["PEM".to_string(), "DER".to_string(), "JWK".to_string(), "Hex".to_string(), "Raw".to_string()]
    );
    compatibility.insert(
        "RSA-Public".to_string(),
        vec!["PEM".to_string(), "DER".to_string(), "JWK".to_string(), "SSH".to_string(), "Hex".to_string(), "Raw".to_string()]
    );
    
    // ECDSA P-256 keys
    compatibility.insert(
        "ECDSA-P256-Private".to_string(),
        vec!["PEM".to_string(), "DER".to_string(), "JWK".to_string(), "Hex".to_string(), "Raw".to_string()]
    );
    compatibility.insert(
        "ECDSA-P256-Public".to_string(),
        vec!["PEM".to_string(), "DER".to_string(), "JWK".to_string(), "SSH".to_string(), "Hex".to_string(), "Raw".to_string()]
    );
    
    // ECDSA P-384 keys
    compatibility.insert(
        "ECDSA-P384-Private".to_string(),
        vec!["PEM".to_string(), "DER".to_string(), "JWK".to_string(), "Hex".to_string(), "Raw".to_string()]
    );
    compatibility.insert(
        "ECDSA-P384-Public".to_string(),
        vec!["PEM".to_string(), "DER".to_string(), "JWK".to_string(), "SSH".to_string(), "Hex".to_string(), "Raw".to_string()]
    );
    
    // Ed25519 keys
    compatibility.insert(
        "Ed25519-Private".to_string(),
        vec!["PEM".to_string(), "DER".to_string(), "JWK".to_string(), "Hex".to_string(), "Raw".to_string()]
    );
    compatibility.insert(
        "Ed25519-Public".to_string(),
        vec!["PEM".to_string(), "DER".to_string(), "JWK".to_string(), "SSH".to_string(), "Hex".to_string(), "Raw".to_string()]
    );
    
    // X25519 keys (no SSH support as it's for key exchange, not authentication)
    compatibility.insert(
        "X25519-Private".to_string(),
        vec!["PEM".to_string(), "DER".to_string(), "JWK".to_string(), "Hex".to_string(), "Raw".to_string()]
    );
    compatibility.insert(
        "X25519-Public".to_string(),
        vec!["PEM".to_string(), "DER".to_string(), "JWK".to_string(), "Hex".to_string(), "Raw".to_string()]
    );
    
    compatibility
}

/// Get detailed format information
pub fn get_format_info(args: Vec<Value>) -> Result<(), Error> {
    if args.is_empty() {
        return Err(CursedError::InvalidArgument("Format name required".to_string()));
    }
    
    let format_name = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Format name must be a string".to_string())),
    };
    
    let format = SerializationFormat::from_name(&format_name)?;
    
    let mut info = HashMap::new();
    info.insert("name".to_string(), Value::String(format.to_string()().to_string()));
    info.insert("description".to_string(), Value::String(format.description().to_string()));
    info.insert("file_extension".to_string(), Value::String(format.file_extension().to_string()));
    
    let encoding_info = match format {
        SerializationFormat::Pem => "Base64 with ASCII armor headers (BEGIN/END)",
        SerializationFormat::Der => "Binary ASN.1 Distinguished Encoding Rules",
        SerializationFormat::Jwk => "JSON Web Key standard (RFC 7517)",
        SerializationFormat::Ssh => "OpenSSH public key format",
        SerializationFormat::Raw => "Raw binary key material",
        SerializationFormat::Hex => "Hexadecimal string representation",
    };
    info.insert("encoding_details".to_string(), Value::String(encoding_info.to_string()));
    
    let supported_ops = match format {
        SerializationFormat::Ssh => vec!["Public key export only"],
        _ => vec!["Serialization", "Deserialization"],
    };
    let ops_values: Vec<Value> = supported_ops.iter().map(|s| Value::String(s.to_string())).collect();
    info.insert("supported_operations".to_string(), Value::Array(ops_values));
    
    Ok(Value::Object(info))
}

/// Convert between key formats
pub fn convert_key_format(args: Vec<Value>) -> Result<(), Error> {
    if args.len() < 4 {
        return Err(CursedError::InvalidArgument("Key format conversion requires: key_type, key_data, source_format, target_format".to_string()));
    }
    
    let key_type_name = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Key type must be a string".to_string())),
    };
    
    let key_data = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Key data must be a string".to_string())),
    };
    
    let source_format_name = match &args[2] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Source format must be a string".to_string())),
    };
    
    let target_format_name = match &args[3] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Target format must be a string".to_string())),
    };
    
    // First deserialize from source format
    let deserialize_result = deserialize_key(vec![
        Value::String(source_format_name),
        Value::String(key_data),
        Value::String(key_type_name.clone()),
    ])?;
    
    // Extract the raw key data
    let raw_key_data = match deserialize_result {
        Value::Object(mut map) => {
            let valid = map.get("valid").and_then(|v| {
                if let Value::Boolean(b) = v { Some(*b) } else { None }
            }).unwrap_or(false);
            
            if !valid {
                let error_msg = map.get("error")
                    .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
                    .unwrap_or_else(|| "Unknown validation error".to_string());
                return Err(CursedError::InvalidArgument(format!("Invalid source key data: {}", error_msg)));
            }
            
            map.remove("key_data")
                .and_then(|v| if let Value::String(s) = v { Some(s) } else { None })
                .ok_or_else(|| CursedError::InvalidArgument("No key data in deserialization result".to_string()))?
        },
        _ => return Err(CursedError::InvalidArgument("Invalid deserialization result format".to_string())),
    };
    
    // Then serialize to target format
    serialize_key(vec![
        Value::String(key_type_name),
        Value::String(raw_key_data),
        Value::String(target_format_name),
    ])
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
        assert!(compatibility.contains_key("RSA-Private"));
        assert!(compatibility.contains_key("RSA-Public"));
        assert!(compatibility.contains_key("Ed25519-Private"));
        assert!(compatibility.contains_key("Ed25519-Public"));
        assert!(compatibility.contains_key("ECDSA-P256-Private"));
        assert!(compatibility.contains_key("ECDSA-P256-Public"));
        assert!(compatibility.contains_key("ECDSA-P384-Private"));
        assert!(compatibility.contains_key("ECDSA-P384-Public"));
        assert!(compatibility.contains_key("X25519-Private"));
        assert!(compatibility.contains_key("X25519-Public"));
        
        let rsa_public_formats = &compatibility["RSA-Public"];
        assert!(rsa_public_formats.contains(&"PEM".to_string()));
        assert!(rsa_public_formats.contains(&"DER".to_string()));
        assert!(rsa_public_formats.contains(&"JWK".to_string()));
        assert!(rsa_public_formats.contains(&"SSH".to_string()));
        
        let rsa_private_formats = &compatibility["RSA-Private"];
        assert!(rsa_private_formats.contains(&"PEM".to_string()));
        assert!(rsa_private_formats.contains(&"JWK".to_string()));
        assert!(!rsa_private_formats.contains(&"SSH".to_string())); // Private keys don't support SSH
        
        let ed25519_public_formats = &compatibility["Ed25519-Public"];
        assert!(ed25519_public_formats.contains(&"PEM".to_string()));
        assert!(ed25519_public_formats.contains(&"JWK".to_string()));
        assert!(ed25519_public_formats.contains(&"SSH".to_string()));
        
        let x25519_formats = &compatibility["X25519-Public"];
        assert!(x25519_formats.contains(&"JWK".to_string()));
        assert!(x25519_formats.contains(&"PEM".to_string()));
        assert!(!x25519_formats.contains(&"SSH".to_string())); // X25519 doesn't support SSH
    }

    #[test]
    fn test_jwk_parsing() {
        // Test Ed25519 public key JWK
        let ed25519_jwk = r#"{"kty":"OKP","crv":"Ed25519","x":"GpwbGQhqNWGJBIIqwByCdtKEYGCk0G0nfFfFwCl_-DE"}"#;
        let result = parse_jwk_to_bytes(ed25519_jwk, &KeyType::Ed25519Public);
        assert!(result.is_ok());
        let bytes = result.unwrap();
        assert_eq!(bytes.len(), 32);
        
        // Test RSA public key JWK
        let rsa_jwk = r#"{"kty":"RSA","n":"0vx7agoebGcQSuuPiLJXZptN9nndrQmbXEps2aiAFbWhM78LhWx4cbbfAAtVT86zwu1RK7aPFFxuhDR1L6tSoc_BJECPebWKRXjBZCiFV4n3oknjhMstn64tZ_2W-5JsGY4Hc5n9yBXArwl93lqt7_RN5w6Cf0h4QyQ5v-65YGjQR0_FDW2QvzqY368QQMicAtaSqzs8KJZgnYb9c7d0zgdAZHzu6qMQvRL5hajrn1n91CbOpbIS"}"#;
        let result = parse_jwk_to_bytes(rsa_jwk, &KeyType::RsaPublic);
        assert!(result.is_ok());
        
        // Test invalid JWK
        let invalid_jwk = r#"{"invalid": "json"}"#;
        let result = parse_jwk_to_bytes(invalid_jwk, &KeyType::Ed25519Public);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_ed25519_der() {
        let key_data = vec![0u8; 32];
        let result = create_ed25519_private_key_der(&key_data);
        assert!(result.is_ok());
        let der = result.unwrap();
        
        // Should be valid PKCS#8 DER format
        assert!(der.len() > 32); // Should be longer than just the key
        assert_eq!(der[0], 0x30); // Should start with SEQUENCE tag
        
        let public_result = create_ed25519_public_key_der(&key_data);
        assert!(public_result.is_ok());
        let public_der = public_result.unwrap();
        assert_eq!(public_der[0], 0x30); // Should start with SEQUENCE tag
    }

    #[test]
    fn test_create_x25519_jwk() {
        let key_data = vec![1u8; 32];
        let result = create_jwk_x25519_private_key(&key_data);
        assert!(result.is_ok());
        
        let jwk_bytes = result.unwrap();
        let jwk_str = String::from_utf8(jwk_bytes).unwrap();
        let jwk: serde_json::Value = serde_json::from_str(&jwk_str).unwrap();
        
        assert_eq!(jwk["kty"], "OKP");
        assert_eq!(jwk["crv"], "X25519");
        assert!(jwk["d"].is_string());
        assert!(jwk["x"].is_string());
    }

    #[test]
    fn test_format_round_trip() {
        let test_key = vec![42u8; 32];
        
        // Test Ed25519 key round trip through various formats
        for format in &[SerializationFormat::Hex, SerializationFormat::Raw] {
            let serialized = match format {
                SerializationFormat::Hex => hex::encode(&test_key).into_bytes(),
                SerializationFormat::Raw => test_key.clone(),
                _ => continue,
            };
            
            let serialized_str = String::from_utf8_lossy(&serialized);
            let parsed = match format {
                SerializationFormat::Hex => hex::decode(serialized_str.as_ref()).unwrap(),
                SerializationFormat::Raw => serialized_str.as_bytes().to_vec(),
                _ => continue,
            };
            
            assert_eq!(parsed, test_key);
        }
    }

    #[test]
    fn test_pem_parsing() {
        let pem_data = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA4f5wg5l2hKsTeNem/V41
fGnJm6gOdrj8ym3rFkEjWT2btf/FuFAIWTrR7N2FHR1JiYTjhp0wI4lKRXp1ObVc
1YA8y8A2cM3HFdF4zX7l5Z0JQ8AuVtVjRa5Fhf4YkfznF5jNg2D6JjZKY6BVbZQ
-----END PUBLIC KEY-----"#;
        
        let result = parse_pem_to_bytes(pem_data);
        assert!(result.is_ok());
        let bytes = result.unwrap();
        assert!(!bytes.is_empty());
        
        // Test invalid PEM
        let invalid_pem = "This is not a PEM";
        let result = parse_pem_to_bytes(invalid_pem);
        assert!(result.is_err());
    }

    #[test]
    fn test_ssh_parsing() {
        let ssh_data = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIGpwbGQhqNWGJBIIqwByCdtKEYGCk0G0nfFfFwCl_-DE test@example.com";
        
        let result = parse_ssh_to_bytes(ssh_data);
        assert!(result.is_ok());
        let bytes = result.unwrap();
        assert!(!bytes.is_empty());
        
        // Test invalid SSH
        let invalid_ssh = "invalid-key format";
        let result = parse_ssh_to_bytes(invalid_ssh);
        assert!(result.is_err());
        
        // Test unsupported algorithm
        let unsupported_ssh = "unsupported-algo AAAAC3NzaC1lZDI1NTE5AAAAIGpwbGQhqNWGJBIIqwByCdtKEYGCk0G0nfFfFwCl_-DE";
        let result = parse_ssh_to_bytes(unsupported_ssh);
        assert!(result.is_err());
    }

    #[test]
    fn test_format_info() {
        let result = get_format_info(vec![Value::String("PEM".to_string())]);
        assert!(result.is_ok());
        
        if let Ok(Value::Object(info)) = result {
            assert!(info.contains_key("name"));
            assert!(info.contains_key("description"));
            assert!(info.contains_key("file_extension"));
            assert!(info.contains_key("encoding_details"));
            assert!(info.contains_key("supported_operations"));
        }
        
        // Test invalid format
        let result = get_format_info(vec![Value::String("INVALID".to_string())]);
        assert!(result.is_err());
    }

    #[test]
    fn test_enhanced_validation() {
        // Test Ed25519 validation with correct length
        let valid_ed25519 = vec![1u8; 32];
        assert!(validate_ed25519_private_key_data(&valid_ed25519).is_ok());
        assert!(validate_ed25519_public_key_data(&valid_ed25519).is_ok());
        
        // Test Ed25519 validation with incorrect length
        let invalid_ed25519 = vec![1u8; 16];
        assert!(validate_ed25519_private_key_data(&invalid_ed25519).is_err());
        assert!(validate_ed25519_public_key_data(&invalid_ed25519).is_err());
        
        // Test X25519 validation
        let valid_x25519 = vec![2u8; 32];
        assert!(validate_x25519_private_key_data(&valid_x25519).is_ok());
        assert!(validate_x25519_public_key_data(&valid_x25519).is_ok());
    }
}
