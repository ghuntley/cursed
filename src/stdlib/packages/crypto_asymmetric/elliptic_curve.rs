// Elliptic curve cryptography
// 
// Provides comprehensive elliptic curve operations for the CURSED stdlib.
// Supports ECDSA signing/verification and ECDH key exchange with P-256, P-384 curves.

// use crate::stdlib::value::Value;
use crate::error::CursedError;
use std::collections::HashMap;
use rand::rngs::OsRng;
use p256::{
    ecdsa::{SigningKey as P256SigningKey, VerifyingKey as P256VerifyingKey, Signature as P256Signature}
// };

use p384::{
    ecdsa::{SigningKey as P384SigningKey, VerifyingKey as P384VerifyingKey, Signature as P384Signature}
// };

use signature::{Signer, Verifier};
use sha2::{Sha256, Sha384, Digest};
use elliptic_curve::sec1::{ToEncodedPoint, FromEncodedPoint};
use elliptic_curve::pkcs8::{EncodePrivateKey, EncodePublicKey, DecodePrivateKey, DecodePublicKey};

/// Supported elliptic curves
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EcCurve {
    P256,    // NIST P-256 (secp256r1)
    P384,    // NIST P-384 (secp384r1)
impl EcCurve {
    pub fn name(&self) -> &'static str {
        match self {
        }
    }
    
    pub fn key_size(&self) -> usize {
        match self {
        }
    }
    
    pub fn from_name(name: &str) -> crate::error::Result<()> {
        match name.to_uppercase().as_str() {
        }
    }
/// Hash algorithms for ECDSA
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EcHashAlgorithm {
impl EcHashAlgorithm {
    pub fn name(&self) -> &'static str {
        match self {
        }
    }
/// EC key pair container
#[derive(Debug, Clone)]
pub struct EcKeyPair {
impl EcKeyPair {
    pub fn new(curve: EcCurve, public_key: Vec<u8>, private_key: Vec<u8>) -> Self {
        let fingerprint = Self::compute_fingerprint(&public_key);
        Self {
        }
    }
    
    fn compute_fingerprint(public_key: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(public_key);
        let result = hasher.finalize();
        hex::encode(result)
    pub fn to_value(&self) -> crate::error::Result<()> {
        let mut map = HashMap::new();
        
        map.insert("curve".to_string(), Value::String(self.curve.name().to_string()));
        map.insert("key_size".to_string(), Value::Integer(self.curve.key_size() as i64));
        map.insert("public_key".to_string(), Value::String(hex::encode(&self.public_key)));
        map.insert("private_key".to_string(), Value::String(hex::encode(&self.private_key)));
        map.insert("fingerprint".to_string(), Value::String(self.fingerprint.clone()));
        
        Ok(Value::Object(map))
    }
}

/// ECDH shared secret result
#[derive(Debug, Clone)]
pub struct EcdhSharedSecret {
impl EcdhSharedSecret {
    pub fn to_value(&self) -> crate::error::Result<()> {
        let mut map = HashMap::new();
        
        map.insert("curve".to_string(), Value::String(self.curve.name().to_string()));
        map.insert("shared_secret".to_string(), Value::String(hex::encode(&self.shared_secret)));
        map.insert("key_size".to_string(), Value::Integer(self.key_size as i64));
        
        Ok(Value::Object(map))
    }
}

/// Generate elliptic curve key pair
pub fn ec_generate_keypair(args: Vec<Value>) -> crate::error::Result<()> {
    if args.is_empty() {
        return Err(CursedError::InvalidArgument("Curve name required".to_string()));
    let curve_name = match &args[0] {
    
    let curve = EcCurve::from_name(&curve_name)?;
    
    match curve {
    }
}

/// Generate P-256 key pair
fn generate_p256_keypair() -> crate::error::Result<()> {
    let mut rng = OsRng;
    let private_key = P256SecretKey::random(&mut rng);
    let public_key = P256PublicKey::from(&private_key);
    
    let private_der = private_key.to_pkcs8_der()
        .map_err(|e| CursedError::CryptoError(format!("P-256 private key serialization failed: {}", e)))?;
    
    let public_der = public_key.to_public_key_der()
        .map_err(|e| CursedError::CryptoError(format!("P-256 public key serialization failed: {}", e)))?;
    
    let keypair = EcKeyPair::new(
    );
    
    keypair.to_value()
/// Generate P-384 key pair
fn generate_p384_keypair() -> crate::error::Result<()> {
    let mut rng = OsRng;
    let private_key = P384SecretKey::random(&mut rng);
    let public_key = P384PublicKey::from(&private_key);
    
    let private_der = private_key.to_pkcs8_der()
        .map_err(|e| CursedError::CryptoError(format!("P-384 private key serialization failed: {}", e)))?;
    
    let public_der = public_key.to_public_key_der()
        .map_err(|e| CursedError::CryptoError(format!("P-384 public key serialization failed: {}", e)))?;
    
    let keypair = EcKeyPair::new(
    );
    
    keypair.to_value()
/// ECDSA signing
pub fn ecdsa_sign(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 3 {
        return Err(CursedError::InvalidArgument("ECDSA signing requires: curve, private_key, message".to_string()));
    let curve_name = match &args[0] {
    
    let private_key_hex = match &args[1] {
    
    let message = match &args[2] {
    
    let curve = EcCurve::from_name(&curve_name)?;
    let private_key_bytes = hex::decode(private_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid private key hex: {}", e)))?;
    
    match curve {
    }
}

/// ECDSA signing with P-256
fn ecdsa_sign_p256(private_key_bytes: &[u8], message: &[u8]) -> crate::error::Result<()> {
    let private_key = P256SecretKey::from_pkcs8_der(private_key_bytes)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode P-256 private key: {}", e)))?;
    
    let signing_key = P256SigningKey::from(private_key);
    
    // Hash the message
    let mut hasher = Sha256::new();
    hasher.update(message);
    let message_hash = hasher.finalize();
    
    let signature: P256Signature = signing_key.sign(&message_hash);
    
    let mut result = HashMap::new();
    result.insert("curve".to_string(), Value::String("P-256".to_string()));
    result.insert("signature".to_string(), Value::String(hex::encode(signature.to_bytes())));
    result.insert("hash_algorithm".to_string(), Value::String("SHA-256".to_string()));
    
    Ok(Value::Object(result))
/// ECDSA signing with P-384
fn ecdsa_sign_p384(private_key_bytes: &[u8], message: &[u8]) -> crate::error::Result<()> {
    let private_key = P384SecretKey::from_pkcs8_der(private_key_bytes)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode P-384 private key: {}", e)))?;
    
    let signing_key = P384SigningKey::from(private_key);
    
    // Hash the message
    let mut hasher = Sha384::new();
    hasher.update(message);
    let message_hash = hasher.finalize();
    
    let signature: P384Signature = signing_key.sign(&message_hash);
    
    let mut result = HashMap::new();
    result.insert("curve".to_string(), Value::String("P-384".to_string()));
    result.insert("signature".to_string(), Value::String(hex::encode(signature.to_bytes())));
    result.insert("hash_algorithm".to_string(), Value::String("SHA-384".to_string()));
    
    Ok(Value::Object(result))
/// ECDSA signature verification
pub fn ecdsa_verify(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 4 {
        return Err(CursedError::InvalidArgument("ECDSA verification requires: curve, public_key, message, signature".to_string()));
    let curve_name = match &args[0] {
    
    let public_key_hex = match &args[1] {
    
    let message = match &args[2] {
    
    let signature_hex = match &args[3] {
    
    let curve = EcCurve::from_name(&curve_name)?;
    let public_key_bytes = hex::decode(public_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid public key hex: {}", e)))?;
    let signature_bytes = hex::decode(signature_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid signature hex: {}", e)))?;
    
    match curve {
    }
}

/// ECDSA verification with P-256
fn ecdsa_verify_p256(public_key_bytes: &[u8], message: &[u8], signature_bytes: &[u8]) -> crate::error::Result<()> {
    let public_key = P256PublicKey::from_public_key_der(public_key_bytes)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode P-256 public key: {}", e)))?;
    
    let verifying_key = P256VerifyingKey::from(public_key);
    
    let signature = P256Signature::from_bytes(signature_bytes.into())
        .map_err(|e| CursedError::CryptoError(format!("Invalid P-256 signature: {}", e)))?;
    
    // Hash the message
    let mut hasher = Sha256::new();
    hasher.update(message);
    let message_hash = hasher.finalize();
    
    let is_valid = verifying_key.verify(&message_hash, &signature).is_ok();
    
    let mut result = HashMap::new();
    result.insert("curve".to_string(), Value::String("P-256".to_string()));
    result.insert("valid".to_string(), Value::Boolean(is_valid));
    result.insert("hash_algorithm".to_string(), Value::String("SHA-256".to_string()));
    
    Ok(Value::Object(result))
/// ECDSA verification with P-384
fn ecdsa_verify_p384(public_key_bytes: &[u8], message: &[u8], signature_bytes: &[u8]) -> crate::error::Result<()> {
    let public_key = P384PublicKey::from_public_key_der(public_key_bytes)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode P-384 public key: {}", e)))?;
    
    let verifying_key = P384VerifyingKey::from(public_key);
    
    let signature = P384Signature::from_bytes(signature_bytes.into())
        .map_err(|e| CursedError::CryptoError(format!("Invalid P-384 signature: {}", e)))?;
    
    // Hash the message
    let mut hasher = Sha384::new();
    hasher.update(message);
    let message_hash = hasher.finalize();
    
    let is_valid = verifying_key.verify(&message_hash, &signature).is_ok();
    
    let mut result = HashMap::new();
    result.insert("curve".to_string(), Value::String("P-384".to_string()));
    result.insert("valid".to_string(), Value::Boolean(is_valid));
    result.insert("hash_algorithm".to_string(), Value::String("SHA-384".to_string()));
    
    Ok(Value::Object(result))
/// ECDH key exchange
pub fn ecdh_key_exchange(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 3 {
        return Err(CursedError::InvalidArgument("ECDH requires: curve, private_key, public_key".to_string()));
    let curve_name = match &args[0] {
    
    let private_key_hex = match &args[1] {
    
    let public_key_hex = match &args[2] {
    
    let curve = EcCurve::from_name(&curve_name)?;
    let private_key_bytes = hex::decode(private_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid private key hex: {}", e)))?;
    let public_key_bytes = hex::decode(public_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid public key hex: {}", e)))?;
    
    match curve {
    }
}

/// ECDH with P-256
fn ecdh_p256(private_key_bytes: &[u8], public_key_bytes: &[u8]) -> crate::error::Result<()> {
    let private_key = P256SecretKey::from_pkcs8_der(private_key_bytes)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode P-256 private key: {}", e)))?;
    
    let public_key = P256PublicKey::from_public_key_der(public_key_bytes)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode P-256 public key: {}", e)))?;
    
    let shared_secret = elliptic_curve::ecdh::diffie_hellman(
    );
    
    let shared_secret_bytes = shared_secret.raw_secret_bytes().to_vec();
    
    let result = EcdhSharedSecret {
    
    result.to_value()
/// ECDH with P-384
fn ecdh_p384(private_key_bytes: &[u8], public_key_bytes: &[u8]) -> crate::error::Result<()> {
    let private_key = P384SecretKey::from_pkcs8_der(private_key_bytes)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode P-384 private key: {}", e)))?;
    
    let public_key = P384PublicKey::from_public_key_der(public_key_bytes)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode P-384 public key: {}", e)))?;
    
    let shared_secret = elliptic_curve::ecdh::diffie_hellman(
    );
    
    let shared_secret_bytes = shared_secret.raw_secret_bytes().to_vec();
    
    let result = EcdhSharedSecret {
    
    result.to_value()
/// List supported elliptic curves
pub fn list_supported_curves() -> Vec<String> {
    vec![
    ]
/// Validate curve and hash algorithm combination
pub fn validate_curve_hash_combination(curve: EcCurve, hash: EcHashAlgorithm) -> crate::error::Result<()> {
    match (curve, hash) {
        (EcCurve::P384, EcHashAlgorithm::Sha256) => Ok(()), // SHA-256 is also acceptable for P-384
        _ => Err(CursedError::InvalidArgument(format!(
            hash.name()
    }
}

