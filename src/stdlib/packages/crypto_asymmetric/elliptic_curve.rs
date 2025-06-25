// Elliptic curve cryptography
// 
// Provides comprehensive elliptic curve operations for the CURSED stdlib.
// Supports ECDSA signing/verification and ECDH key exchange with P-256, P-384 curves.

// use crate::stdlib::value::Value;
use crate::error::CursedError;
use std::collections::HashMap;
use rand::rngs::OsRng;
use p256::{
    SecretKey as P256SecretKey, 
    PublicKey as P256PublicKey,
    ecdsa::{SigningKey as P256SigningKey, VerifyingKey as P256VerifyingKey, Signature as P256Signature}
};

use p384::{
    SecretKey as P384SecretKey, 
    PublicKey as P384PublicKey,
    ecdsa::{SigningKey as P384SigningKey, VerifyingKey as P384VerifyingKey, Signature as P384Signature}
};

use signature::{Signer, Verifier};
use sha2::{Sha256, Sha384, Digest};
use elliptic_curve::sec1::{ToEncodedPoint, FromEncodedPoint};
use elliptic_curve::pkcs8::{EncodePrivateKey, EncodePublicKey, DecodePrivateKey, DecodePublicKey};

/// Supported elliptic curves
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EcCurve {
    P256,    // NIST P-256 (secp256r1)
    P384,    // NIST P-384 (secp384r1)
}

impl EcCurve {
    pub fn name(&self) -> &'static str {
        match self {
            EcCurve::P256 => "P-256",
            EcCurve::P384 => "P-384",
        }
    }
    
    pub fn key_size(&self) -> usize {
        match self {
            EcCurve::P256 => 256,
            EcCurve::P384 => 384,
        }
    }
    
    pub fn from_name(name: &str) -> crate::error::Result<()> {
        match name.to_uppercase().as_str() {
            "P-256" | "P256" | "SECP256R1" => Ok(EcCurve::P256),
            "P-384" | "P384" | "SECP384R1" => Ok(EcCurve::P384),
            _ => Err(CursedError::InvalidArgument(format!("Unsupported curve: {}", name))),
        }
    }
}

/// Hash algorithms for ECDSA
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EcHashAlgorithm {
    Sha256,
    Sha384,
}

impl EcHashAlgorithm {
    pub fn name(&self) -> &'static str {
        match self {
            EcHashAlgorithm::Sha256 => "SHA-256",
            EcHashAlgorithm::Sha384 => "SHA-384",
        }
    }
}

/// EC key pair container
#[derive(Debug, Clone)]
pub struct EcKeyPair {
    pub curve: EcCurve,
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
    pub fingerprint: String,
}

impl EcKeyPair {
    pub fn new(curve: EcCurve, public_key: Vec<u8>, private_key: Vec<u8>) -> Self {
        let fingerprint = Self::compute_fingerprint(&public_key);
        Self {
            curve,
            public_key,
            private_key,
            fingerprint,
        }
    }
    
    fn compute_fingerprint(public_key: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(public_key);
        let result = hasher.finalize();
        hex::encode(result)
    }
    
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
    pub curve: EcCurve,
    pub shared_secret: Vec<u8>,
    pub key_size: usize,
}

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
    }
    
    let curve_name = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Curve name must be a string".to_string())),
    };
    
    let curve = EcCurve::from_name(&curve_name)?;
    
    match curve {
        EcCurve::P256 => generate_p256_keypair(),
        EcCurve::P384 => generate_p384_keypair(),
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
        EcCurve::P256,
        public_der.as_bytes().to_vec(),
        private_der.as_bytes().to_vec(),
    );
    
    keypair.to_value()
}

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
        EcCurve::P384,
        public_der.as_bytes().to_vec(),
        private_der.as_bytes().to_vec(),
    );
    
    keypair.to_value()
}

/// ECDSA signing
pub fn ecdsa_sign(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 3 {
        return Err(CursedError::InvalidArgument("ECDSA signing requires: curve, private_key, message".to_string()));
    }
    
    let curve_name = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Curve name must be a string".to_string())),
    };
    
    let private_key_hex = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Private key must be a string".to_string())),
    };
    
    let message = match &args[2] {
        Value::String(s) => s.as_bytes().to_vec(),
        Value::Binary(b) => b.clone(),
        _ => return Err(CursedError::InvalidArgument("Message must be a string or binary data".to_string())),
    };
    
    let curve = EcCurve::from_name(&curve_name)?;
    let private_key_bytes = hex::decode(private_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid private key hex: {}", e)))?;
    
    match curve {
        EcCurve::P256 => ecdsa_sign_p256(&private_key_bytes, &message),
        EcCurve::P384 => ecdsa_sign_p384(&private_key_bytes, &message),
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
}

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
}

/// ECDSA signature verification
pub fn ecdsa_verify(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 4 {
        return Err(CursedError::InvalidArgument("ECDSA verification requires: curve, public_key, message, signature".to_string()));
    }
    
    let curve_name = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Curve name must be a string".to_string())),
    };
    
    let public_key_hex = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Public key must be a string".to_string())),
    };
    
    let message = match &args[2] {
        Value::String(s) => s.as_bytes().to_vec(),
        Value::Binary(b) => b.clone(),
        _ => return Err(CursedError::InvalidArgument("Message must be a string or binary data".to_string())),
    };
    
    let signature_hex = match &args[3] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Signature must be a string".to_string())),
    };
    
    let curve = EcCurve::from_name(&curve_name)?;
    let public_key_bytes = hex::decode(public_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid public key hex: {}", e)))?;
    let signature_bytes = hex::decode(signature_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid signature hex: {}", e)))?;
    
    match curve {
        EcCurve::P256 => ecdsa_verify_p256(&public_key_bytes, &message, &signature_bytes),
        EcCurve::P384 => ecdsa_verify_p384(&public_key_bytes, &message, &signature_bytes),
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
}

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
}

/// ECDH key exchange
pub fn ecdh_key_exchange(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 3 {
        return Err(CursedError::InvalidArgument("ECDH requires: curve, private_key, public_key".to_string()));
    }
    
    let curve_name = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Curve name must be a string".to_string())),
    };
    
    let private_key_hex = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Private key must be a string".to_string())),
    };
    
    let public_key_hex = match &args[2] {
        Value::String(s) => s.clone(),
        _ => return Err(CursedError::InvalidArgument("Public key must be a string".to_string())),
    };
    
    let curve = EcCurve::from_name(&curve_name)?;
    let private_key_bytes = hex::decode(private_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid private key hex: {}", e)))?;
    let public_key_bytes = hex::decode(public_key_hex)
        .map_err(|e| CursedError::InvalidArgument(format!("Invalid public key hex: {}", e)))?;
    
    match curve {
        EcCurve::P256 => ecdh_p256(&private_key_bytes, &public_key_bytes),
        EcCurve::P384 => ecdh_p384(&private_key_bytes, &public_key_bytes),
    }
}

/// ECDH with P-256
fn ecdh_p256(private_key_bytes: &[u8], public_key_bytes: &[u8]) -> crate::error::Result<()> {
    let private_key = P256SecretKey::from_pkcs8_der(private_key_bytes)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode P-256 private key: {}", e)))?;
    
    let public_key = P256PublicKey::from_public_key_der(public_key_bytes)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode P-256 public key: {}", e)))?;
    
    let shared_secret = elliptic_curve::ecdh::diffie_hellman(
        private_key.to_nonzero_scalar(),
        public_key.as_affine(),
    );
    
    let shared_secret_bytes = shared_secret.raw_secret_bytes().to_vec();
    
    let result = EcdhSharedSecret {
        curve: EcCurve::P256,
        shared_secret: shared_secret_bytes,
        key_size: 256,
    };
    
    result.to_value()
}

/// ECDH with P-384
fn ecdh_p384(private_key_bytes: &[u8], public_key_bytes: &[u8]) -> crate::error::Result<()> {
    let private_key = P384SecretKey::from_pkcs8_der(private_key_bytes)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode P-384 private key: {}", e)))?;
    
    let public_key = P384PublicKey::from_public_key_der(public_key_bytes)
        .map_err(|e| CursedError::CryptoError(format!("Failed to decode P-384 public key: {}", e)))?;
    
    let shared_secret = elliptic_curve::ecdh::diffie_hellman(
        private_key.to_nonzero_scalar(),
        public_key.as_affine(),
    );
    
    let shared_secret_bytes = shared_secret.raw_secret_bytes().to_vec();
    
    let result = EcdhSharedSecret {
        curve: EcCurve::P384,
        shared_secret: shared_secret_bytes,
        key_size: 384,
    };
    
    result.to_value()
}

/// List supported elliptic curves
pub fn list_supported_curves() -> Vec<String> {
    vec![
        EcCurve::P256.name().to_string(),
        EcCurve::P384.name().to_string(),
    ]
}

/// Validate curve and hash algorithm combination
pub fn validate_curve_hash_combination(curve: EcCurve, hash: EcHashAlgorithm) -> crate::error::Result<()> {
    match (curve, hash) {
        (EcCurve::P256, EcHashAlgorithm::Sha256) => Ok(()),
        (EcCurve::P384, EcHashAlgorithm::Sha384) => Ok(()),
        (EcCurve::P384, EcHashAlgorithm::Sha256) => Ok(()), // SHA-256 is also acceptable for P-384
        _ => Err(CursedError::InvalidArgument(format!(
            "Incompatible curve {} with hash {}", 
            curve.name(), 
            hash.name()
        ))),
    }
}

