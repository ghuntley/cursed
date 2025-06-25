use crate::error::CursedError;
/// fr fr Elliptic Curve Cryptography implementation with ECDSA signatures
/// 
/// This module provides production-ready ECC key generation, ECDSA signing/verification
/// for NIST P-256, P-384, and P-521 curves using established cryptographic libraries.

use std::collections::HashMap;
use rand::rngs::OsRng;
use p256::{NistP256, ecdsa::SigningKey as P256SigningKey, ecdsa::VerifyingKey as P256VerifyingKey};
use p384::{NistP384, ecdsa::SigningKey as P384SigningKey, ecdsa::VerifyingKey as P384VerifyingKey};
use p521::{NistP521, ecdsa::SigningKey as P521SigningKey, ecdsa::VerifyingKey as P521VerifyingKey};
use signature::{Signer, Verifier, SignatureEncoding};
use elliptic_curve::{
// };

use zeroize::Zeroizing;
use sha2::{Sha256, Sha384, Sha512, Digest};

/// fr fr Supported NIST curves
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EccCurve {
    P256,  // secp256r1 / prime256v1
    P384,  // secp384r1
    P521,  // secp521r1
impl EccCurve {
    pub fn name(&self) -> &'static str {
        match self {
        }
    }
    
    pub fn key_size_bits(&self) -> usize {
        match self {
        }
    }
    
    pub fn coordinate_size_bytes(&self) -> usize {
        match self {
        }
    }
/// fr fr Key serialization formats for ECC
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EccKeyFormat {
    Pkcs8Pem,     // PKCS#8 PEM format
    Pkcs8Der,     // PKCS#8 DER format
    Sec1Der,      // SEC1 DER format (for public keys)
    CompressedSec1, // SEC1 compressed format
    UncompressedSec1, // SEC1 uncompressed format
/// fr fr Hash algorithms for ECDSA
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EccHashAlgorithm {
impl EccHashAlgorithm {
    pub fn name(&self) -> &'static str {
        match self {
        }
    }
    
    pub fn digest_size(&self) -> usize {
        match self {
        }
    }
/// fr fr ECC key pair wrapper
#[derive(Debug, Clone)]
pub enum EccKeyPair {
    P256 {
    P384 {
    P521 {
impl EccKeyPair {
    pub fn curve(&self) -> EccCurve {
        match self {
        }
    }
    
    pub fn key_size_bits(&self) -> usize {
        self.curve().key_size_bits()
    }
}

/// fr fr ECC error types
#[derive(Debug, Clone, PartialEq)]
pub enum EccError {
// impl std::fmt::Display for EccError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             EccError::UnsupportedCurve(curve) => write!(f, "Unsupported ECC curve: {}", curve),
//             EccError::KeyGenerationFailed(msg) => write!(f, "ECC key generation failed: {}", msg),
//             EccError::SigningFailed(msg) => write!(f, "ECDSA signing failed: {}", msg),
//             EccError::VerificationFailed(msg) => write!(f, "ECDSA verification failed: {}", msg),
//             EccError::InvalidSignature(msg) => write!(f, "Invalid ECDSA signature: {}", msg),
//             EccError::InvalidPublicKey(msg) => write!(f, "Invalid ECC public key: {}", msg),
//             EccError::InvalidPrivateKey(msg) => write!(f, "Invalid ECC private key: {}", msg),
//             EccError::InvalidFormat(msg) => write!(f, "Invalid key format: {}", msg),
//             EccError::SerializationFailed(msg) => write!(f, "Key serialization failed: {}", msg),
//             EccError::DeserializationFailed(msg) => write!(f, "Key deserialization failed: {}", msg),
//             EccError::UnsupportedHashAlgorithm(algo) => write!(f, "Unsupported hash algorithm: {}", algo),
//             EccError::Internal(msg) => write!(f, "Internal ECC error: {}", msg),
//         }
//     }
// }

// impl std::error::CursedError for EccError {}
// 
type Ecccrate::error::Result<T> = Result<T>;

/// fr fr ECC engine for cryptographic operations
pub struct EccEngine {
impl EccEngine {
    /// slay Create new ECC engine with cryptographically secure RNG
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// slay Generate ECC key pair for specified curve
    /// 
    /// # Security Notes
    /// - Uses cryptographically secure random number generation
    /// - Keys are generated according to NIST standards
    /// - Private keys are properly validated for curve membership
    pub fn generate_keypair(&mut self, curve: EccCurve) -> EccResult<EccKeyPair> {
        match curve {
            EccCurve::P256 => {
                let private_key = P256SigningKey::random(&mut self.rng);
                let public_key = private_key.verifying_key();
                Ok(EccKeyPair::P256 {
                })
            EccCurve::P384 => {
                let private_key = P384SigningKey::random(&mut self.rng);
                let public_key = private_key.verifying_key();
                Ok(EccKeyPair::P384 {
                })
            EccCurve::P521 => {
                let private_key = P521SigningKey::random(&mut self.rng);
                let public_key = private_key.verifying_key();
                Ok(EccKeyPair::P521 {
                })
        }
    }
    
    /// slay ECDSA sign message with private key
    /// 
    /// # Security Notes
    /// - Message is automatically hashed before signing
    /// - Uses deterministic nonce generation (RFC 6979) for security
    /// - Signatures are DER-encoded for interoperability
    pub fn sign(&self, keypair: &EccKeyPair, message: &[u8], hash_algo: EccHashAlgorithm) -> EccResult<Vec<u8>> {
        // Hash the message first
        let hash = self.hash_message(message, hash_algo)?;
        
        match keypair {
            EccKeyPair::P256 { private, .. } => {
                let signature: p256::ecdsa::Signature = private.sign(&hash);
                Ok(signature.to_der().to_bytes().to_vec())
            EccKeyPair::P384 { private, .. } => {
                let signature: p384::ecdsa::Signature = private.sign(&hash);
                Ok(signature.to_der().to_bytes().to_vec())
            EccKeyPair::P521 { private, .. } => {
                let signature: p521::ecdsa::Signature = private.sign(&hash);
                Ok(signature.to_der().to_bytes().to_vec())
        }
    }
    
    /// slay ECDSA verify signature with public key
    /// 
    /// # Security Notes
    /// - Verification is constant-time to prevent timing attacks
    /// - Signature format is validated before verification
    /// - Hash algorithm must match the one used for signing
    pub fn verify(&self, keypair: &EccKeyPair, message: &[u8], signature: &[u8], hash_algo: EccHashAlgorithm) -> EccResult<bool> {
        // Hash the message
        let hash = self.hash_message(message, hash_algo)?;
        
        let result = match keypair {
            EccKeyPair::P256 { public, .. } => {
                let sig = p256::ecdsa::Signature::from_der(signature)
                    .map_err(|e| EccError::InvalidSignature(e.to_string()))?;
                public.verify(&hash, &sig)
            EccKeyPair::P384 { public, .. } => {
                let sig = p384::ecdsa::Signature::from_der(signature)
                    .map_err(|e| EccError::InvalidSignature(e.to_string()))?;
                public.verify(&hash, &sig)
            EccKeyPair::P521 { public, .. } => {
                let sig = p521::ecdsa::Signature::from_der(signature)
                    .map_err(|e| EccError::InvalidSignature(e.to_string()))?;
                public.verify(&hash, &sig)
        
        Ok(result.is_ok())
    /// slay Verify signature with standalone public key
    pub fn verify_with_public_key(&self, public_key: &[u8], curve: EccCurve, message: &[u8], signature: &[u8], hash_algo: EccHashAlgorithm) -> EccResult<bool> {
        let keypair = self.create_keypair_from_public_key(public_key, curve)?;
        self.verify(&keypair, message, signature, hash_algo)
    /// slay Serialize private key to specified format
    /// 
    /// # Security Notes
    /// - Private keys should be encrypted when stored
    /// - Use secure storage mechanisms
    /// - Returns zeroizing container for automatic cleanup
    pub fn serialize_private_key(&self, keypair: &EccKeyPair, format: EccKeyFormat) -> EccResult<Zeroizing<Vec<u8>>> {
        match (keypair, format) {
            (EccKeyPair::P256 { private, .. }, EccKeyFormat::Pkcs8Pem) => {
                let pem = private.to_pkcs8_pem(LineEnding::LF)
                    .map_err(|e| EccError::SerializationFailed(e.to_string()))?;
                Ok(Zeroizing::new(pem.as_bytes().to_vec()))
            (EccKeyPair::P256 { private, .. }, EccKeyFormat::Pkcs8Der) => {
                let der = private.to_pkcs8_der()
                    .map_err(|e| EccError::SerializationFailed(e.to_string()))?;
                Ok(Zeroizing::new(der.to_bytes().to_vec()))
            (EccKeyPair::P384 { private, .. }, EccKeyFormat::Pkcs8Pem) => {
                let pem = private.to_pkcs8_pem(LineEnding::LF)
                    .map_err(|e| EccError::SerializationFailed(e.to_string()))?;
                Ok(Zeroizing::new(pem.as_bytes().to_vec()))
            (EccKeyPair::P384 { private, .. }, EccKeyFormat::Pkcs8Der) => {
                let der = private.to_pkcs8_der()
                    .map_err(|e| EccError::SerializationFailed(e.to_string()))?;
                Ok(Zeroizing::new(der.to_bytes().to_vec()))
            (EccKeyPair::P521 { private, .. }, EccKeyFormat::Pkcs8Pem) => {
                let pem = private.to_pkcs8_pem(LineEnding::LF)
                    .map_err(|e| EccError::SerializationFailed(e.to_string()))?;
                Ok(Zeroizing::new(pem.as_bytes().to_vec()))
            (EccKeyPair::P521 { private, .. }, EccKeyFormat::Pkcs8Der) => {
                let der = private.to_pkcs8_der()
                    .map_err(|e| EccError::SerializationFailed(e.to_string()))?;
                Ok(Zeroizing::new(der.to_bytes().to_vec()))
        }
    }
    
    /// slay Serialize public key to specified format
    pub fn serialize_public_key(&self, keypair: &EccKeyPair, format: EccKeyFormat) -> EccResult<Vec<u8>> {
        match (keypair, format) {
            (EccKeyPair::P256 { public, .. }, EccKeyFormat::Pkcs8Pem) => {
                let pem = public.to_public_key_pem(LineEnding::LF)
                    .map_err(|e| EccError::SerializationFailed(e.to_string()))?;
                Ok(pem.as_bytes().to_vec())
            (EccKeyPair::P256 { public, .. }, EccKeyFormat::Pkcs8Der) => {
                let der = public.to_public_key_der()
                    .map_err(|e| EccError::SerializationFailed(e.to_string()))?;
                Ok(der.to_bytes().to_vec())
            (EccKeyPair::P256 { public, .. }, EccKeyFormat::UncompressedSec1) => {
                let point = public.to_encoded_point(false);
                Ok(point.as_bytes().to_vec())
            (EccKeyPair::P256 { public, .. }, EccKeyFormat::CompressedSec1) => {
                let point = public.to_encoded_point(true);
                Ok(point.as_bytes().to_vec())
            (EccKeyPair::P384 { public, .. }, EccKeyFormat::Pkcs8Pem) => {
                let pem = public.to_public_key_pem(LineEnding::LF)
                    .map_err(|e| EccError::SerializationFailed(e.to_string()))?;
                Ok(pem.as_bytes().to_vec())
            (EccKeyPair::P384 { public, .. }, EccKeyFormat::Pkcs8Der) => {
                let der = public.to_public_key_der()
                    .map_err(|e| EccError::SerializationFailed(e.to_string()))?;
                Ok(der.to_bytes().to_vec())
            (EccKeyPair::P384 { public, .. }, EccKeyFormat::UncompressedSec1) => {
                let point = public.to_encoded_point(false);
                Ok(point.as_bytes().to_vec())
            (EccKeyPair::P384 { public, .. }, EccKeyFormat::CompressedSec1) => {
                let point = public.to_encoded_point(true);
                Ok(point.as_bytes().to_vec())
            (EccKeyPair::P521 { public, .. }, EccKeyFormat::Pkcs8Pem) => {
                let pem = public.to_public_key_pem(LineEnding::LF)
                    .map_err(|e| EccError::SerializationFailed(e.to_string()))?;
                Ok(pem.as_bytes().to_vec())
            (EccKeyPair::P521 { public, .. }, EccKeyFormat::Pkcs8Der) => {
                let der = public.to_public_key_der()
                    .map_err(|e| EccError::SerializationFailed(e.to_string()))?;
                Ok(der.to_bytes().to_vec())
            (EccKeyPair::P521 { public, .. }, EccKeyFormat::UncompressedSec1) => {
                let point = public.to_encoded_point(false);
                Ok(point.as_bytes().to_vec())
            (EccKeyPair::P521 { public, .. }, EccKeyFormat::CompressedSec1) => {
                let point = public.to_encoded_point(true);
                Ok(point.as_bytes().to_vec())
        }
    }
    
    /// slay Deserialize private key from specified format
    pub fn deserialize_private_key(&self, key_data: &[u8], curve: EccCurve, format: EccKeyFormat) -> EccResult<EccKeyPair> {
        match (curve, format) {
            (EccCurve::P256, EccKeyFormat::Pkcs8Pem) => {
                let pem_str = std::str::from_utf8(key_data)
                    .map_err(|e| EccError::DeserializationFailed(e.to_string()))?;
                let private_key = P256SigningKey::from_pkcs8_pem(pem_str)
                    .map_err(|e| EccError::DeserializationFailed(e.to_string()))?;
                let public_key = *private_key.verifying_key();
                Ok(EccKeyPair::P256 { private: private_key, public: public_key })
            (EccCurve::P256, EccKeyFormat::Pkcs8Der) => {
                let private_key = P256SigningKey::from_pkcs8_der(key_data)
                    .map_err(|e| EccError::DeserializationFailed(e.to_string()))?;
                let public_key = *private_key.verifying_key();
                Ok(EccKeyPair::P256 { private: private_key, public: public_key })
            (EccCurve::P384, EccKeyFormat::Pkcs8Pem) => {
                let pem_str = std::str::from_utf8(key_data)
                    .map_err(|e| EccError::DeserializationFailed(e.to_string()))?;
                let private_key = P384SigningKey::from_pkcs8_pem(pem_str)
                    .map_err(|e| EccError::DeserializationFailed(e.to_string()))?;
                let public_key = *private_key.verifying_key();
                Ok(EccKeyPair::P384 { private: private_key, public: public_key })
            (EccCurve::P384, EccKeyFormat::Pkcs8Der) => {
                let private_key = P384SigningKey::from_pkcs8_der(key_data)
                    .map_err(|e| EccError::DeserializationFailed(e.to_string()))?;
                let public_key = *private_key.verifying_key();
                Ok(EccKeyPair::P384 { private: private_key, public: public_key })
            (EccCurve::P521, EccKeyFormat::Pkcs8Pem) => {
                let pem_str = std::str::from_utf8(key_data)
                    .map_err(|e| EccError::DeserializationFailed(e.to_string()))?;
                let private_key = P521SigningKey::from_pkcs8_pem(pem_str)
                    .map_err(|e| EccError::DeserializationFailed(e.to_string()))?;
                let public_key = *private_key.verifying_key();
                Ok(EccKeyPair::P521 { private: private_key, public: public_key })
            (EccCurve::P521, EccKeyFormat::Pkcs8Der) => {
                let private_key = P521SigningKey::from_pkcs8_der(key_data)
                    .map_err(|e| EccError::DeserializationFailed(e.to_string()))?;
                let public_key = *private_key.verifying_key();
                Ok(EccKeyPair::P521 { private: private_key, public: public_key })
        }
    }
    
    /// slay Deserialize public key from specified format
    pub fn deserialize_public_key(&self, key_data: &[u8], curve: EccCurve, format: EccKeyFormat) -> EccResult<EccKeyPair> {
        match (curve, format) {
            (EccCurve::P256, EccKeyFormat::Pkcs8Pem) => {
                let pem_str = std::str::from_utf8(key_data)
                    .map_err(|e| EccError::DeserializationFailed(e.to_string()))?;
                let public_key = P256VerifyingKey::from_public_key_pem(pem_str)
                    .map_err(|e| EccError::DeserializationFailed(e.to_string()))?;
                // Note: We only have public key, so we create a keypair with dummy private key
                // In real use, you'd want separate methods for public-key-only operations
                let dummy_private = P256SigningKey::random(&mut OsRng);
                Ok(EccKeyPair::P256 { private: dummy_private, public: public_key })
            (EccCurve::P256, EccKeyFormat::Pkcs8Der) => {
                let public_key = P256VerifyingKey::from_public_key_der(key_data)
                    .map_err(|e| EccError::DeserializationFailed(e.to_string()))?;
                let dummy_private = P256SigningKey::random(&mut OsRng);
                Ok(EccKeyPair::P256 { private: dummy_private, public: public_key })
        }
    }
    
    /// slay Get public key coordinates for curve point operations
    pub fn get_public_key_coordinates(&self, keypair: &EccKeyPair) -> EccResult<(Vec<u8>, Vec<u8>)> {
        match keypair {
            EccKeyPair::P256 { public, .. } => {
                let point = public.to_encoded_point(false);
                if let Coordinates::Uncompressed { x, y } = point.coordinates() {
                    Ok((x.to_vec(), y.to_vec()))
                } else {
                    Err(EccError::InvalidPublicKey("Failed to extract coordinates".to_string()))
                }
            EccKeyPair::P384 { public, .. } => {
                let point = public.to_encoded_point(false);
                if let Coordinates::Uncompressed { x, y } = point.coordinates() {
                    Ok((x.to_vec(), y.to_vec()))
                } else {
                    Err(EccError::InvalidPublicKey("Failed to extract coordinates".to_string()))
                }
            EccKeyPair::P521 { public, .. } => {
                let point = public.to_encoded_point(false);
                if let Coordinates::Uncompressed { x, y } = point.coordinates() {
                    Ok((x.to_vec(), y.to_vec()))
                } else {
                    Err(EccError::InvalidPublicKey("Failed to extract coordinates".to_string()))
                }
        }
    }
    
    // Helper methods
    
    fn hash_message(&self, message: &[u8], hash_algo: EccHashAlgorithm) -> EccResult<Vec<u8>> {
        match hash_algo {
        }
    }
    
    fn create_keypair_from_public_key(&self, public_key_bytes: &[u8], curve: EccCurve) -> EccResult<EccKeyPair> {
        // This is a simplified implementation for verification-only operations
        // In practice, you'd implement proper public key deserialization
        match curve {
            EccCurve::P256 => {
                let point = p256::EncodedPoint::from_bytes(public_key_bytes)
                    .map_err(|e| EccError::InvalidPublicKey(e.to_string()))?;
                let public_key = P256VerifyingKey::from_encoded_point(&point)
                    .map_err(|e| EccError::InvalidPublicKey(e.to_string()))?;
                let dummy_private = P256SigningKey::random(&mut OsRng);
                Ok(EccKeyPair::P256 { private: dummy_private, public: public_key })
            EccCurve::P384 => {
                let point = p384::EncodedPoint::from_bytes(public_key_bytes)
                    .map_err(|e| EccError::InvalidPublicKey(e.to_string()))?;
                let public_key = P384VerifyingKey::from_encoded_point(&point)
                    .map_err(|e| EccError::InvalidPublicKey(e.to_string()))?;
                let dummy_private = P384SigningKey::random(&mut OsRng);
                Ok(EccKeyPair::P384 { private: dummy_private, public: public_key })
            EccCurve::P521 => {
                let point = p521::EncodedPoint::from_bytes(public_key_bytes)
                    .map_err(|e| EccError::InvalidPublicKey(e.to_string()))?;
                let public_key = P521VerifyingKey::from_encoded_point(&point)
                    .map_err(|e| EccError::InvalidPublicKey(e.to_string()))?;
                let dummy_private = P521SigningKey::random(&mut OsRng);
                Ok(EccKeyPair::P521 { private: dummy_private, public: public_key })
        }
    }
impl Default for EccEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Public API functions for CURSED integration
// use crate::stdlib::value::Value;

/// slay Generate ECC key pair
pub fn ecc_generate_keypair(args: Vec<Value>) -> crate::error::Result<()> {
    let curve_name = if args.is_empty() {
        "P-256".to_string()
    } else {
        match &args[0] {
        }
    
    let curve = match curve_name.as_str() {
    
    let mut engine = EccEngine::new();
    match engine.generate_keypair(curve) {
        Ok(keypair) => {
            let mut result = HashMap::new();
            result.insert("algorithm".to_string(), Value::String("ECDSA".to_string()));
            result.insert("curve".to_string(), Value::String(curve.name().to_string()));
            result.insert("key_size".to_string(), Value::Number(keypair.key_size_bits() as f64));
            
            // Serialize public key to PEM
            if let Ok(public_pem) = engine.serialize_public_key(&keypair, EccKeyFormat::Pkcs8Pem) {
                result.insert("public_key_pem".to_string(), Value::String(String::from_utf8_lossy(&public_pem).to_string()));
            // Get coordinates for display
            if let Ok((x, y)) = engine.get_public_key_coordinates(&keypair) {
                result.insert("public_x".to_string(), Value::String(hex::encode(x)));
                result.insert("public_y".to_string(), Value::String(hex::encode(y)));
            result.insert("has_private_key".to_string(), Value::bool(true));
            
            Ok(Value::Object(result))
    }
}

/// slay ECDSA sign message
pub fn ecdsa_sign(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 3 {
        return Err(CursedError::Runtime("ECDSA sign requires private key, message, and curve".to_string()));
    let private_key_pem = match &args[0] {
    
    let message = match &args[1] {
    
    let curve_name = match &args[2] {
    
    let curve = match curve_name.as_str() {
    
    let hash_algo = if args.len() > 3 {
        match &args[3] {
            Value::String(s) => match s.as_str() {
        }
    } else {
        EccHashAlgorithm::Sha256
    
    let engine = EccEngine::new();
    
    // Parse private key
    let keypair = engine.deserialize_private_key(private_key_pem.as_bytes(), curve, EccKeyFormat::Pkcs8Pem)
        .map_err(|e| CursedError::Runtime(format!("Invalid private key: {}", e)))?;
    
    // Sign
    let signature = engine.sign(&keypair, message, hash_algo)
        .map_err(|e| CursedError::Runtime(format!("Signing failed: {}", e)))?;
    
    Ok(Value::String(base64::encode(signature)))
/// slay ECDSA verify signature
pub fn ecdsa_verify(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 4 {
        return Err(CursedError::Runtime("ECDSA verify requires public key, message, signature, and curve".to_string()));
    let public_key_pem = match &args[0] {
    
    let message = match &args[1] {
    
    let signature_b64 = match &args[2] {
    
    let curve_name = match &args[3] {
    
    let curve = match curve_name.as_str() {
    
    let hash_algo = if args.len() > 4 {
        match &args[4] {
            Value::String(s) => match s.as_str() {
        }
    } else {
        EccHashAlgorithm::Sha256
    
    let engine = EccEngine::new();
    
    // Parse public key
    let keypair = engine.deserialize_public_key(public_key_pem.as_bytes(), curve, EccKeyFormat::Pkcs8Pem)
        .map_err(|e| CursedError::Runtime(format!("Invalid public key: {}", e)))?;
    
    // Decode signature
    let signature = base64::decode(signature_b64)
        .map_err(|e| CursedError::Runtime(format!("Invalid base64 signature: {}", e)))?;
    
    // Verify
    let is_valid = engine.verify(&keypair, message, &signature, hash_algo)
        .map_err(|e| CursedError::Runtime(format!("Verification failed: {}", e)))?;
    
    Ok(Value::bool(is_valid))
