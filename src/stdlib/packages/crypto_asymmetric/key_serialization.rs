// Key serialization utilities
// 
// Provides comprehensive key serialization functions for the CURSED stdlib.
// Supports PEM, DER, JWK, SSH, and raw formats for various key types.

// use crate::stdlib::value::Value;
use crate::error::CursedError;
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
    /// DER format (binary ASN.1)
    /// JSON Web Key format
    /// SSH public key format
    /// Raw bytes format
    /// Hexadecimal string format
impl SerializationFormat {
    pub fn name(&self) -> &'static str {
        match self {
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
        }
    }
    
    pub fn file_extension(&self) -> &'static str {
        match self {
        }
    }
    
    pub fn from_name(name: &str) -> crate::error::Result<()> {
        match name.to_uppercase().as_str() {
        }
    }
/// Key type for serialization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyType {
impl KeyType {
    pub fn name(&self) -> &'static str {
        match self {
        }
    }
    
    pub fn algorithm(&self) -> &'static str {
        match self {
        }
    }
    
    pub fn is_private(&self) -> bool {
        match self {
            KeyType::RsaPrivate | 
            KeyType::EcdsaP256Private | 
            KeyType::EcdsaP384Private | 
            KeyType::Ed25519Private | 
        }
    }
    
    pub fn from_name(name: &str) -> crate::error::Result<()> {
        match name.to_uppercase().as_str() {
        }
    }
/// Serialization result container
#[derive(Debug, Clone)]
pub struct SerializationResult {
    pub encoding: String, // Base64, hex, etc.
impl SerializationResult {
    pub fn new(
    ) -> Self {
        let encoding = match format {
        
        Self {
        }
    }
    
    pub fn to_value(&self) -> crate::error::Result<()> {
        let mut map = HashMap::new();
        
        map.insert("format".to_string(), Value::String(self.format.to_string()().to_string()));
        map.insert("key_type".to_string(), Value::String(self.key_type.to_string()().to_string()));
        map.insert("encoding".to_string(), Value::String(self.encoding.clone()));
        
        // Include both raw bytes and appropriate string representation
        map.insert("raw_bytes".to_string(), Value::String(hex::encode(&self.serialized_data)));
        
        match self.format {
            SerializationFormat::Pem | SerializationFormat::Ssh => {
                map.insert("data".to_string(), Value::String(String::from_utf8_lossy(&self.serialized_data).to_string()));
            SerializationFormat::Jwk => {
                map.insert("data".to_string(), Value::String(String::from_utf8_lossy(&self.serialized_data).to_string()));
            SerializationFormat::Hex => {
                map.insert("data".to_string(), Value::String(hex::encode(&self.serialized_data)));
            SerializationFormat::Der | SerializationFormat::Raw => {
                map.insert("data".to_string(), Value::String(general_purpose::STANDARD.encode(&self.serialized_data)));
        Ok(Value::Object(map))
    }
}

/// Serialize key to format
pub fn serialize_key(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 3 {
        return Err(CursedError::InvalidArgument("Key serialization requires: key_type, key_data, format".to_string()));
    let key_type_name = match &args[0] {
    
    let key_data_hex = match &args[1] {
    
    let format_name = match &args[2] {
    
    let key_type = KeyType::from_name(&key_type_name)?;
    let format = SerializationFormat::from_name(&format_name)?;
    let key_data = hex::decode(key_data_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid key data hex: {}", e)))?;
    
    match key_type {
    }
}

/// Serialize RSA private key
fn serialize_rsa_private_key(key_data: &[u8], format: SerializationFormat) -> crate::error::Result<()> {
    let private_key = RsaPrivateKey::from_pkcs8_der(key_data)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode RSA private key: {}", e)))?;
    
    let serialized = match format {
        SerializationFormat::Pem => {
            private_key.to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode RSA private key to PEM: {}", e)))?
                .as_bytes().to_vec()
        SerializationFormat::Der => {
            private_key.to_pkcs8_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode RSA private key to DER: {}", e)))?
                .as_bytes().to_vec()
        SerializationFormat::Hex => {
            let der = private_key.to_pkcs8_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode RSA private key to DER: {}", e)))?;
            hex::encode(der.as_bytes()).into_bytes()
        SerializationFormat::Raw => {
            private_key.to_pkcs8_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode RSA private key to DER: {}", e)))?
                .as_bytes().to_vec()
        SerializationFormat::Jwk => {
            create_jwk_rsa_private_key(&private_key)?
        SerializationFormat::Ssh => {
            return Err(CursedError::InvalidArgument("SSH format not supported for private keys".to_string()));
    
    let result = SerializationResult::new(format, KeyType::RsaPrivate, serialized);
    result.to_value()
/// Serialize RSA public key
fn serialize_rsa_public_key(key_data: &[u8], format: SerializationFormat) -> crate::error::Result<()> {
    let public_key = RsaPublicKey::from_public_key_der(key_data)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode RSA public key: {}", e)))?;
    
    let serialized = match format {
        SerializationFormat::Pem => {
            public_key.to_public_key_pem(rsa::pkcs8::LineEnding::LF)
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode RSA public key to PEM: {}", e)))?
                .as_bytes().to_vec()
        SerializationFormat::Der => {
            public_key.to_public_key_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode RSA public key to DER: {}", e)))?
                .as_bytes().to_vec()
        SerializationFormat::Hex => {
            let der = public_key.to_public_key_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode RSA public key to DER: {}", e)))?;
            hex::encode(der.as_bytes()).into_bytes()
        SerializationFormat::Raw => {
            public_key.to_public_key_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode RSA public key to DER: {}", e)))?
                .as_bytes().to_vec()
        SerializationFormat::Ssh => {
            create_ssh_rsa_public_key(&public_key)?
        SerializationFormat::Jwk => {
            create_jwk_rsa_public_key(&public_key)?
    
    let result = SerializationResult::new(format, KeyType::RsaPublic, serialized);
    result.to_value()
/// Create SSH format for RSA public key
fn create_ssh_rsa_public_key(public_key: &RsaPublicKey) -> crate::error::Result<()> {
    // SSH RSA public key format: ssh-rsa <base64-encoded-key> [comment]
    let der = public_key.to_public_key_der()
        .map_err(|e| CursedError::CryptoError(format!("Failed to encode RSA public key: {}", e)))?;
    
    let base64_key = general_purpose::STANDARD.encode(der.as_bytes());
    let ssh_key = format!("ssh-rsa {} cursed-generated-key", base64_key);
    
    Ok(ssh_key.into_bytes())
/// Create JWK format for RSA public key
fn create_jwk_rsa_public_key(public_key: &RsaPublicKey) -> crate::error::Result<()> {
    use rsa::traits::PublicKeyParts;
    
    let n = public_key.n();
    let e = public_key.e();
    
    // Convert to base64url encoding (JWK standard)
    let n_bytes = n.to_bytes_be();
    let e_bytes = e.to_bytes_be();
    
    let n_b64 = general_purpose::URL_SAFE_NO_PAD.encode(&n_bytes);
    let e_b64 = general_purpose::URL_SAFE_NO_PAD.encode(&e_bytes);
    
    let jwk = serde_json::json!({
        "e": e_b64
    });
    
    Ok(jwk.to_string().into_bytes())
/// Create JWK format for RSA private key  
fn create_jwk_rsa_private_key(private_key: &RsaPrivateKey) -> crate::error::Result<()> {
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
        "d": d_b64
    });
    
    Ok(jwk.to_string().into_bytes())
/// Create JWK format for P-256 private key
fn create_jwk_p256_private_key(private_key: &P256SecretKey) -> crate::error::Result<()> {
    use elliptic_curve::sec1::ToEncodedPoint;
    
    let public_key = private_key.public_key();
    let point = public_key.to_encoded_point(false);
    
    // Extract x and y coordinates (skip the first byte which is 0x04 for uncompressed)
    let coords = point.as_bytes();
    if coords.len() != 65 || coords[0] != 0x04 {
        return Err(CursedError::CryptoError("Invalid P-256 public key point".to_string()));
    let x = &coords[1..33];
    let y = &coords[33..65];
    let d = private_key.to_bytes();
    
    let x_b64 = general_purpose::URL_SAFE_NO_PAD.encode(x);
    let y_b64 = general_purpose::URL_SAFE_NO_PAD.encode(y);
    let d_b64 = general_purpose::URL_SAFE_NO_PAD.encode(&d);
    
    let jwk = serde_json::json!({
        "d": d_b64
    });
    
    Ok(jwk.to_string().into_bytes())
/// Create JWK format for P-256 public key
fn create_jwk_p256_public_key(public_key: &P256PublicKey) -> crate::error::Result<()> {
    use elliptic_curve::sec1::ToEncodedPoint;
    
    let point = public_key.to_encoded_point(false);
    let coords = point.as_bytes();
    
    if coords.len() != 65 || coords[0] != 0x04 {
        return Err(CursedError::CryptoError("Invalid P-256 public key point".to_string()));
    let x = &coords[1..33];
    let y = &coords[33..65];
    
    let x_b64 = general_purpose::URL_SAFE_NO_PAD.encode(x);
    let y_b64 = general_purpose::URL_SAFE_NO_PAD.encode(y);
    
    let jwk = serde_json::json!({
        "y": y_b64
    });
    
    Ok(jwk.to_string().into_bytes())
/// Create SSH format for P-256 public key
fn create_ssh_p256_public_key(public_key: &P256PublicKey) -> crate::error::Result<()> {
    use elliptic_curve::sec1::ToEncodedPoint;
    
    let point = public_key.to_encoded_point(false);
    let base64_key = general_purpose::STANDARD.encode(point.as_bytes());
    let ssh_key = format!("ecdsa-sha2-nistp256 {} cursed-generated-key", base64_key);
    
    Ok(ssh_key.into_bytes())
/// Create JWK format for P-384 private key
fn create_jwk_p384_private_key(private_key: &P384SecretKey) -> crate::error::Result<()> {
    use elliptic_curve::sec1::ToEncodedPoint;
    
    let public_key = private_key.public_key();
    let point = public_key.to_encoded_point(false);
    
    // Extract x and y coordinates (skip the first byte which is 0x04 for uncompressed)
    let coords = point.as_bytes();
    if coords.len() != 97 || coords[0] != 0x04 {
        return Err(CursedError::CryptoError("Invalid P-384 public key point".to_string()));
    let x = &coords[1..49];
    let y = &coords[49..97];
    let d = private_key.to_bytes();
    
    let x_b64 = general_purpose::URL_SAFE_NO_PAD.encode(x);
    let y_b64 = general_purpose::URL_SAFE_NO_PAD.encode(y);
    let d_b64 = general_purpose::URL_SAFE_NO_PAD.encode(&d);
    
    let jwk = serde_json::json!({
        "d": d_b64
    });
    
    Ok(jwk.to_string().into_bytes())
/// Create JWK format for P-384 public key
fn create_jwk_p384_public_key(public_key: &P384PublicKey) -> crate::error::Result<()> {
    use elliptic_curve::sec1::ToEncodedPoint;
    
    let point = public_key.to_encoded_point(false);
    let coords = point.as_bytes();
    
    if coords.len() != 97 || coords[0] != 0x04 {
        return Err(CursedError::CryptoError("Invalid P-384 public key point".to_string()));
    let x = &coords[1..49];
    let y = &coords[49..97];
    
    let x_b64 = general_purpose::URL_SAFE_NO_PAD.encode(x);
    let y_b64 = general_purpose::URL_SAFE_NO_PAD.encode(y);
    
    let jwk = serde_json::json!({
        "y": y_b64
    });
    
    Ok(jwk.to_string().into_bytes())
/// Create SSH format for P-384 public key
fn create_ssh_p384_public_key(public_key: &P384PublicKey) -> crate::error::Result<()> {
    use elliptic_curve::sec1::ToEncodedPoint;
    
    let point = public_key.to_encoded_point(false);
    let base64_key = general_purpose::STANDARD.encode(point.as_bytes());
    let ssh_key = format!("ecdsa-sha2-nistp384 {} cursed-generated-key", base64_key);
    
    Ok(ssh_key.into_bytes())
/// Create PEM format for Ed25519 private key
fn create_ed25519_private_key_pem(key_data: &[u8]) -> crate::error::Result<()> {
    let der = create_ed25519_private_key_der(key_data)?;
    let pem = format!(
        general_purpose::STANDARD.encode(&der).chars()
            .collect::<Vec<char>>()
            .chunks(64)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    );
    Ok(pem.into_bytes())
/// Create DER format for Ed25519 private key  
fn create_ed25519_private_key_der(key_data: &[u8]) -> crate::error::Result<()> {
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
/// Create PEM format for Ed25519 public key
fn create_ed25519_public_key_pem(key_data: &[u8]) -> crate::error::Result<()> {
    let der = create_ed25519_public_key_der(key_data)?;
    let pem = format!(
        general_purpose::STANDARD.encode(&der).chars()
            .collect::<Vec<char>>()
            .chunks(64)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    );
    Ok(pem.into_bytes())
/// Create DER format for Ed25519 public key
fn create_ed25519_public_key_der(key_data: &[u8]) -> crate::error::Result<()> {
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
/// Create JWK format for Ed25519 private key
fn create_jwk_ed25519_private_key(key_data: &[u8]) -> crate::error::Result<()> {
    let signing_key = SigningKey::from_bytes(
        key_data.try_into()
            .map_err(|_| CursedError::InvalidArgument("Invalid Ed25519 private key length".to_string()))?
    );
    
    let public_key = signing_key.verifying_key();
    let d_b64 = general_purpose::URL_SAFE_NO_PAD.encode(key_data);
    let x_b64 = general_purpose::URL_SAFE_NO_PAD.encode(public_key.as_bytes());
    
    let jwk = serde_json::json!({
        "d": d_b64
    });
    
    Ok(jwk.to_string().into_bytes())
/// Create JWK format for Ed25519 public key
fn create_jwk_ed25519_public_key(key_data: &[u8]) -> crate::error::Result<()> {
    let x_b64 = general_purpose::URL_SAFE_NO_PAD.encode(key_data);
    
    let jwk = serde_json::json!({
        "x": x_b64
    });
    
    Ok(jwk.to_string().into_bytes())
/// Create PEM format for X25519 private key
fn create_x25519_private_key_pem(key_data: &[u8]) -> crate::error::Result<()> {
    let der = create_x25519_private_key_der(key_data)?;
    let pem = format!(
        general_purpose::STANDARD.encode(&der).chars()
            .collect::<Vec<char>>()
            .chunks(64)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    );
    Ok(pem.into_bytes())
/// Create DER format for X25519 private key  
fn create_x25519_private_key_der(key_data: &[u8]) -> crate::error::Result<()> {
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
/// Create PEM format for X25519 public key
fn create_x25519_public_key_pem(key_data: &[u8]) -> crate::error::Result<()> {
    let der = create_x25519_public_key_der(key_data)?;
    let pem = format!(
        general_purpose::STANDARD.encode(&der).chars()
            .collect::<Vec<char>>()
            .chunks(64)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    );
    Ok(pem.into_bytes())
/// Create DER format for X25519 public key
fn create_x25519_public_key_der(key_data: &[u8]) -> crate::error::Result<()> {
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
/// Create JWK format for X25519 private key
fn create_jwk_x25519_private_key(key_data: &[u8]) -> crate::error::Result<()> {
    let key_array: [u8; 32] = key_data.try_into()
        .map_err(|_| CursedError::InvalidArgument("Invalid X25519 private key length".to_string()))?;
    let private_key = EphemeralSecret::from(key_array);
    let public_key = X25519PublicKey::from(&private_key);
    
    let d_b64 = general_purpose::URL_SAFE_NO_PAD.encode(key_data);
    let x_b64 = general_purpose::URL_SAFE_NO_PAD.encode(public_key.as_bytes());
    
    let jwk = serde_json::json!({
        "d": d_b64
    });
    
    Ok(jwk.to_string().into_bytes())
/// Create JWK format for X25519 public key
fn create_jwk_x25519_public_key(key_data: &[u8]) -> crate::error::Result<()> {
    let x_b64 = general_purpose::URL_SAFE_NO_PAD.encode(key_data);
    
    let jwk = serde_json::json!({
        "x": x_b64
    });
    
    Ok(jwk.to_string().into_bytes())
/// Serialize P-256 private key
fn serialize_p256_private_key(key_data: &[u8], format: SerializationFormat) -> crate::error::Result<()> {
    let private_key = P256SecretKey::from_pkcs8_der(key_data)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode P-256 private key: {}", e)))?;
    
    let serialized = match format {
        SerializationFormat::Pem => {
            private_key.to_pkcs8_pem(elliptic_curve::pkcs8::LineEnding::LF)
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-256 private key to PEM: {}", e)))?
                .as_bytes().to_vec()
        SerializationFormat::Der => {
            private_key.to_pkcs8_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-256 private key to DER: {}", e)))?
                .as_bytes().to_vec()
        SerializationFormat::Hex => {
            let der = private_key.to_pkcs8_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-256 private key to DER: {}", e)))?;
            hex::encode(der.as_bytes()).into_bytes()
        SerializationFormat::Raw => {
            private_key.to_bytes().to_vec()
        SerializationFormat::Jwk => {
            create_jwk_p256_private_key(&private_key)?
        SerializationFormat::Ssh => {
            return Err(CursedError::InvalidArgument("SSH format not supported for private keys".to_string()));
    
    let result = SerializationResult::new(format, KeyType::EcdsaP256Private, serialized);
    result.to_value()
/// Serialize P-256 public key
fn serialize_p256_public_key(key_data: &[u8], format: SerializationFormat) -> crate::error::Result<()> {
    let public_key = P256PublicKey::from_public_key_der(key_data)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode P-256 public key: {}", e)))?;
    
    let serialized = match format {
        SerializationFormat::Pem => {
            public_key.to_public_key_pem(elliptic_curve::pkcs8::LineEnding::LF)
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-256 public key to PEM: {}", e)))?
                .as_bytes().to_vec()
        SerializationFormat::Der => {
            public_key.to_public_key_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-256 public key to DER: {}", e)))?
                .as_bytes().to_vec()
        SerializationFormat::Hex => {
            let der = public_key.to_public_key_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-256 public key to DER: {}", e)))?;
            hex::encode(der.as_bytes()).into_bytes()
        SerializationFormat::Raw => {
            use elliptic_curve::sec1::ToEncodedPoint;
            public_key.to_encoded_point(false).as_bytes().to_vec()
        SerializationFormat::Jwk => {
            create_jwk_p256_public_key(&public_key)?
        SerializationFormat::Ssh => {
            create_ssh_p256_public_key(&public_key)?
    
    let result = SerializationResult::new(format, KeyType::EcdsaP256Public, serialized);
    result.to_value()
/// Serialize P-384 private key
fn serialize_p384_private_key(key_data: &[u8], format: SerializationFormat) -> crate::error::Result<()> {
    let private_key = P384SecretKey::from_pkcs8_der(key_data)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode P-384 private key: {}", e)))?;
    
    let serialized = match format {
        SerializationFormat::Pem => {
            private_key.to_pkcs8_pem(elliptic_curve::pkcs8::LineEnding::LF)
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-384 private key to PEM: {}", e)))?
                .as_bytes().to_vec()
        SerializationFormat::Der => {
            private_key.to_pkcs8_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-384 private key to DER: {}", e)))?
                .as_bytes().to_vec()
        SerializationFormat::Hex => {
            let der = private_key.to_pkcs8_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-384 private key to DER: {}", e)))?;
            hex::encode(der.as_bytes()).into_bytes()
        SerializationFormat::Raw => {
            private_key.to_bytes().to_vec()
        SerializationFormat::Jwk => {
            create_jwk_p384_private_key(&private_key)?
        SerializationFormat::Ssh => {
            return Err(CursedError::InvalidArgument("SSH format not supported for private keys".to_string()));
    
    let result = SerializationResult::new(format, KeyType::EcdsaP384Private, serialized);
    result.to_value()
/// Serialize P-384 public key
fn serialize_p384_public_key(key_data: &[u8], format: SerializationFormat) -> crate::error::Result<()> {
    let public_key = P384PublicKey::from_public_key_der(key_data)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode P-384 public key: {}", e)))?;
    
    let serialized = match format {
        SerializationFormat::Pem => {
            public_key.to_public_key_pem(elliptic_curve::pkcs8::LineEnding::LF)
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-384 public key to PEM: {}", e)))?
                .as_bytes().to_vec()
        SerializationFormat::Der => {
            public_key.to_public_key_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-384 public key to DER: {}", e)))?
                .as_bytes().to_vec()
        SerializationFormat::Hex => {
            let der = public_key.to_public_key_der()
                .map_err(|e| CursedError::CryptoError(format!("Failed to encode P-384 public key to DER: {}", e)))?;
            hex::encode(der.as_bytes()).into_bytes()
        SerializationFormat::Raw => {
            use elliptic_curve::sec1::ToEncodedPoint;
            public_key.to_encoded_point(false).as_bytes().to_vec()
        SerializationFormat::Jwk => {
            create_jwk_p384_public_key(&public_key)?
        SerializationFormat::Ssh => {
            create_ssh_p384_public_key(&public_key)?
    
    let result = SerializationResult::new(format, KeyType::EcdsaP384Public, serialized);
    result.to_value()
/// Serialize Ed25519 private key
fn serialize_ed25519_private_key(key_data: &[u8], format: SerializationFormat) -> crate::error::Result<()> {
    if key_data.len() != 32 {
        return Err(CursedError::InvalidArgument("Ed25519 private key must be 32 bytes".to_string()));
    let signing_key = SigningKey::from_bytes(
        key_data.try_into()
            .map_err(|_| CursedError::InvalidArgument("Invalid Ed25519 private key length".to_string()))?
    );
    
    let serialized = match format {
        SerializationFormat::Pem => {
            create_ed25519_private_key_pem(key_data)?
        SerializationFormat::Der => {
            create_ed25519_private_key_der(key_data)?
        SerializationFormat::Jwk => {
            create_jwk_ed25519_private_key(key_data)?
        SerializationFormat::Ssh => {
            return Err(CursedError::InvalidArgument("SSH format not supported for private keys".to_string()));
    
    let result = SerializationResult::new(format, KeyType::Ed25519Private, serialized);
    result.to_value()
/// Serialize Ed25519 public key
fn serialize_ed25519_public_key(key_data: &[u8], format: SerializationFormat) -> crate::error::Result<()> {
    if key_data.len() != 32 {
        return Err(CursedError::InvalidArgument("Ed25519 public key must be 32 bytes".to_string()));
    let verifying_key = VerifyingKey::from_bytes(
        key_data.try_into()
            .map_err(|_| CursedError::InvalidArgument("Invalid Ed25519 public key length".to_string()))?
    ).map_err(|e| CursedError::CryptoError(format!("Invalid Ed25519 public key: {}", e)))?;
    
    let serialized = match format {
        SerializationFormat::Ssh => {
            let base64_key = general_purpose::STANDARD.encode(key_data);
            let ssh_key = format!("ssh-ed25519 {} cursed-generated-key", base64_key);
            ssh_key.into_bytes()
        SerializationFormat::Pem => {
            create_ed25519_public_key_pem(key_data)?
        SerializationFormat::Der => {
            create_ed25519_public_key_der(key_data)?
        SerializationFormat::Jwk => {
            create_jwk_ed25519_public_key(key_data)?
    
    let result = SerializationResult::new(format, KeyType::Ed25519Public, serialized);
    result.to_value()
/// Serialize X25519 private key
fn serialize_x25519_private_key(key_data: &[u8], format: SerializationFormat) -> crate::error::Result<()> {
    if key_data.len() != 32 {
        return Err(CursedError::InvalidArgument("X25519 private key must be 32 bytes".to_string()));
    let serialized = match format {
        SerializationFormat::Pem => {
            create_x25519_private_key_pem(key_data)?
        SerializationFormat::Der => {
            create_x25519_private_key_der(key_data)?
        SerializationFormat::Jwk => {
            create_jwk_x25519_private_key(key_data)?
        SerializationFormat::Ssh => {
            return Err(CursedError::InvalidArgument("SSH format not supported for private keys".to_string()));
    
    let result = SerializationResult::new(format, KeyType::X25519Private, serialized);
    result.to_value()
/// Serialize X25519 public key
fn serialize_x25519_public_key(key_data: &[u8], format: SerializationFormat) -> crate::error::Result<()> {
    if key_data.len() != 32 {
        return Err(CursedError::InvalidArgument("X25519 public key must be 32 bytes".to_string()));
    let serialized = match format {
        SerializationFormat::Pem => {
            create_x25519_public_key_pem(key_data)?
        SerializationFormat::Der => {
            create_x25519_public_key_der(key_data)?
        SerializationFormat::Jwk => {
            create_jwk_x25519_public_key(key_data)?
        SerializationFormat::Ssh => {
            return Err(CursedError::InvalidArgument("SSH format not supported for X25519 keys".to_string()));
    
    let result = SerializationResult::new(format, KeyType::X25519Public, serialized);
    result.to_value()
/// Deserialize key from format
pub fn deserialize_key(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 3 {
        return Err(CursedError::InvalidArgument("Key deserialization requires: format, serialized_data, expected_key_type".to_string()));
    let format_name = match &args[0] {
    
    let serialized_data = match &args[1] {
    
    let expected_key_type_name = match &args[2] {
    
    let format = SerializationFormat::from_name(&format_name)?;
    let expected_key_type = KeyType::from_name(&expected_key_type_name)?;
    
    // Convert serialized data to bytes based on format
    let data_bytes = match format {
        SerializationFormat::Pem => {
            parse_pem_to_bytes(&serialized_data)?
        SerializationFormat::Ssh => {
            parse_ssh_to_bytes(&serialized_data)?
        SerializationFormat::Der | SerializationFormat::Raw => {
            general_purpose::STANDARD.decode(serialized_data)
                .map_err(|e| CursedError::InvalidArgument(format!("Invalid base64 data: {}", e)))?
        SerializationFormat::Hex => {
            hex::decode(serialized_data)
                .map_err(|e| CursedError::InvalidArgument(format!("Invalid hex data: {}", e)))?
        SerializationFormat::Jwk => {
            parse_jwk_to_bytes(&serialized_data, &expected_key_type)?
    
    // Attempt to deserialize based on expected key type
    let validation_result = match expected_key_type {
    
    let mut result = HashMap::new();
    result.insert("format".to_string(), Value::String(format.to_string()().to_string()));
    result.insert("key_type".to_string(), Value::String(expected_key_type.to_string()().to_string()));
    result.insert("valid".to_string(), Value::Boolean(validation_result.is_ok()));
    result.insert("key_data".to_string(), Value::String(hex::encode(&data_bytes)));
    
    if let Err(error_msg) = validation_result {
        result.insert("error".to_string(), Value::String(error_msg));
    Ok(Value::Object(result))
/// Parse PEM to raw bytes
fn parse_pem_to_bytes(pem_data: &str) -> crate::error::Result<()> {
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
        _ => return Err(CursedError::InvalidArgument("Invalid PEM format: missing BEGIN/END markers".to_string())),
    
    if start >= end {
        return Err(CursedError::InvalidArgument("Invalid PEM format: empty content".to_string()));
    // Concatenate base64 content lines
    let base64_content = lines[start..end].join("");
    
    // Decode base64
    general_purpose::STANDARD.decode(base64_content)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid base64 in PEM: {}", e)))
/// Parse SSH key to raw bytes
fn parse_ssh_to_bytes(ssh_data: &str) -> crate::error::Result<()> {
    // SSH public key format: <algorithm> <base64-key> [comment]
    let parts: Vec<&str> = ssh_data.trim().split_whitespace().collect();
    
    if parts.len() < 2 {
        return Err(CursedError::InvalidArgument("Invalid SSH key format: expected algorithm and key data".to_string()));
    let algorithm = parts[0];
    let key_data = parts[1];
    
    // Validate algorithm
    match algorithm {
    // Decode base64 key data
    general_purpose::STANDARD.decode(key_data)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid base64 in SSH key: {}", e)))
/// Parse JWK to raw bytes
fn parse_jwk_to_bytes(jwk_str: &str, expected_key_type: &KeyType) -> crate::error::Result<()> {
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
        ("RSA", KeyType::RsaPublic) => {
            let n = jwk.get("n")
                .and_then(|v| v.as_str())
                .ok_or_else(|| CursedError::InvalidArgument("Missing 'n' field for RSA public key".to_string()))?;
            
            general_purpose::URL_SAFE_NO_PAD.decode(n)
                .map_err(|e| CursedError::InvalidArgument(format!("Invalid base64url in RSA public key: {}", e)))
        ("EC", KeyType::EcdsaP256Private) | ("EC", KeyType::EcdsaP384Private) => {
            let d = jwk.get("d")
                .and_then(|v| v.as_str())
                .ok_or_else(|| CursedError::InvalidArgument("Missing 'd' field for EC private key".to_string()))?;
            
            general_purpose::URL_SAFE_NO_PAD.decode(d)
                .map_err(|e| CursedError::InvalidArgument(format!("Invalid base64url in EC private key: {}", e)))
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
        ("OKP", KeyType::Ed25519Private) | ("OKP", KeyType::X25519Private) => {
            let d = jwk.get("d")
                .and_then(|v| v.as_str())
                .ok_or_else(|| CursedError::InvalidArgument("Missing 'd' field for OKP private key".to_string()))?;
            
            general_purpose::URL_SAFE_NO_PAD.decode(d)
                .map_err(|e| CursedError::InvalidArgument(format!("Invalid base64url in OKP private key: {}", e)))
        ("OKP", KeyType::Ed25519Public) | ("OKP", KeyType::X25519Public) => {
            let x = jwk.get("x")
                .and_then(|v| v.as_str())
                .ok_or_else(|| CursedError::InvalidArgument("Missing 'x' field for OKP public key".to_string()))?;
            
            general_purpose::URL_SAFE_NO_PAD.decode(x)
                .map_err(|e| CursedError::InvalidArgument(format!("Invalid base64url in OKP public key: {}", e)))
    }
}

/// Validation helper functions
fn validate_rsa_private_key_data(data: &[u8]) -> Result<(), String> {
    // Try PKCS#8 first, then PKCS#1 if that fails
    match RsaPrivateKey::from_pkcs8_der(data) {
        Err(_) => {
            // Try PKCS#1 format
            RsaPrivateKey::from_pkcs1_der(data)
                .map_err(|e| format!("Invalid RSA private key (tried PKCS#8 and PKCS#1): {}", e))?;
            Ok(())
        }
    }
fn validate_rsa_public_key_data(data: &[u8]) -> Result<(), String> {
    // Try SubjectPublicKeyInfo first, then PKCS#1 if that fails
    match RsaPublicKey::from_public_key_der(data) {
        Err(_) => {
            // Try PKCS#1 format
            RsaPublicKey::from_pkcs1_der(data)
                .map_err(|e| format!("Invalid RSA public key (tried SubjectPublicKeyInfo and PKCS#1): {}", e))?;
            Ok(())
        }
    }
fn validate_p256_private_key_data(data: &[u8]) -> Result<(), String> {
    // Try PKCS#8 first, then raw key if length matches
    match P256SecretKey::from_pkcs8_der(data) {
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
fn validate_ed25519_public_key_data(data: &[u8]) -> Result<(), String> {
    if data.len() != 32 {
        return Err(format!("Ed25519 public key must be 32 bytes, got {}", data.len()));
    let key_array: [u8; 32] = data.try_into()
        .map_err(|_| "Invalid key length".to_string())?;
    
    VerifyingKey::from_bytes(&key_array)
        .map_err(|e| format!("Invalid Ed25519 public key: {}", e))?;
    
    Ok(())
fn validate_x25519_private_key_data(data: &[u8]) -> Result<(), String> {
    if data.len() != 32 {
        return Err(format!("X25519 private key must be 32 bytes, got {}", data.len()));
    }
    Ok(())
fn validate_x25519_public_key_data(data: &[u8]) -> Result<(), String> {
    if data.len() != 32 {
        return Err(format!("X25519 public key must be 32 bytes, got {}", data.len()));
    }
    Ok(())
/// List supported serialization formats
pub fn list_serialization_formats() -> Vec<String> {
    vec![
    ]
/// Get format compatibility matrix
pub fn get_format_compatibility() -> HashMap<String, Vec<String>> {
    let mut compatibility = HashMap::new();
    
    // RSA keys support all formats including SSH for public keys
    compatibility.insert(
        vec!["PEM".to_string(), "DER".to_string(), "JWK".to_string(), "Hex".to_string(), "Raw".to_string()]
    );
    compatibility.insert(
        vec!["PEM".to_string(), "DER".to_string(), "JWK".to_string(), "SSH".to_string(), "Hex".to_string(), "Raw".to_string()]
    );
    
    // ECDSA P-256 keys
    compatibility.insert(
        vec!["PEM".to_string(), "DER".to_string(), "JWK".to_string(), "Hex".to_string(), "Raw".to_string()]
    );
    compatibility.insert(
        vec!["PEM".to_string(), "DER".to_string(), "JWK".to_string(), "SSH".to_string(), "Hex".to_string(), "Raw".to_string()]
    );
    
    // ECDSA P-384 keys
    compatibility.insert(
        vec!["PEM".to_string(), "DER".to_string(), "JWK".to_string(), "Hex".to_string(), "Raw".to_string()]
    );
    compatibility.insert(
        vec!["PEM".to_string(), "DER".to_string(), "JWK".to_string(), "SSH".to_string(), "Hex".to_string(), "Raw".to_string()]
    );
    
    // Ed25519 keys
    compatibility.insert(
        vec!["PEM".to_string(), "DER".to_string(), "JWK".to_string(), "Hex".to_string(), "Raw".to_string()]
    );
    compatibility.insert(
        vec!["PEM".to_string(), "DER".to_string(), "JWK".to_string(), "SSH".to_string(), "Hex".to_string(), "Raw".to_string()]
    );
    
    // X25519 keys (no SSH support as it's for key exchange, not authentication)
    compatibility.insert(
        vec!["PEM".to_string(), "DER".to_string(), "JWK".to_string(), "Hex".to_string(), "Raw".to_string()]
    );
    compatibility.insert(
        vec!["PEM".to_string(), "DER".to_string(), "JWK".to_string(), "Hex".to_string(), "Raw".to_string()]
    );
    
    compatibility
/// Get detailed format information
pub fn get_format_info(args: Vec<Value>) -> crate::error::Result<()> {
    if args.is_empty() {
        return Err(CursedError::InvalidArgument("Format name required".to_string()));
    let format_name = match &args[0] {
    
    let format = SerializationFormat::from_name(&format_name)?;
    
    let mut info = HashMap::new();
    info.insert("name".to_string(), Value::String(format.to_string()().to_string()));
    info.insert("description".to_string(), Value::String(format.description().to_string()));
    info.insert("file_extension".to_string(), Value::String(format.file_extension().to_string()));
    
    let encoding_info = match format {
        SerializationFormat::Pem => "Base64 with ASCII armor headers (BEGIN/END)",
    info.insert("encoding_details".to_string(), Value::String(encoding_info.to_string()));
    
    let supported_ops = match format {
    let ops_values: Vec<Value> = supported_ops.iter().map(|s| Value::String(s.to_string())).collect();
    info.insert("supported_operations".to_string(), Value::Array(ops_values));
    
    Ok(Value::Object(info))
/// Convert between key formats
pub fn convert_key_format(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 4 {
        return Err(CursedError::InvalidArgument("Key format conversion requires: key_type, key_data, source_format, target_format".to_string()));
    let key_type_name = match &args[0] {
    
    let key_data = match &args[1] {
    
    let source_format_name = match &args[2] {
    
    let target_format_name = match &args[3] {
    
    // First deserialize from source format
    let deserialize_result = deserialize_key(vec![
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
            map.remove("key_data")
                .and_then(|v| if let Value::String(s) = v { Some(s) } else { None })
                .ok_or_else(|| CursedError::InvalidArgument("No key data in deserialization result".to_string()))?
    
    // Then serialize to target format
    serialize_key(vec![
    ])
