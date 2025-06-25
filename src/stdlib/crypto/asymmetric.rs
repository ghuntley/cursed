
/// fr fr Asymmetric cryptography for CURSED - public key crypto with style periodt
/// 
/// This module provides comprehensive asymmetric cryptography including RSA,
/// ECDSA, ECDH, X25519, and Ed25519. Maximum security with Gen Z flair bestie!

use std::collections::HashMap;
use std::sync::Arc;
use std::fmt;
use crate::error::CursedError;

// use crate::stdlib::value::Value;

// External crypto crates
pub use rsa::{RsaPrivateKey, RsaPublicKey, Pkcs1v15Encrypt, Oaep, Pss};
use rsa::signature::{Keypair, RandomizedSigner, SignatureEncoding, Verifier};
use rsa::sha2::{Sha256, Sha384, Sha512};
use p256::{SecretKey as P256SecretKey, PublicKey as P256PublicKey, ecdsa::{Signature as P256Signature, SigningKey as P256SigningKey, VerifyingKey as P256VerifyingKey}};
use p384::{SecretKey as P384SecretKey, PublicKey as P384PublicKey, ecdsa::{Signature as P384Signature, SigningKey as P384SigningKey, VerifyingKey as P384VerifyingKey}};
use k256::{SecretKey as K256SecretKey, PublicKey as K256PublicKey, ecdsa::{Signature as K256Signature, SigningKey as K256SigningKey, VerifyingKey as K256VerifyingKey}};
use ed25519_dalek::{SigningKey as Ed25519SigningKey, VerifyingKey as Ed25519VerifyingKey, Signature as Ed25519SignatureInternal, Signer, Verifier as Ed25519Verifier};

// Type alias for backward compatibility - ed25519-dalek 2.0 uses separate keys
pub type Ed25519Keypair = Ed25519KeyPair;
use x25519_dalek::{EphemeralSecret, PublicKey as X25519PublicKeyInternal};
use signature::{Signer as SignatureSigner, Verifier as SignatureVerifier};
use rand::{rngs::OsRng, RngCore, CryptoRng};
use base64::{Engine as _, engine::general_purpose};
use pem::{Pem, encode, parse};
use der::{Encode, Decode};
use pkcs8::{EncodePrivateKey, DecodePrivateKey, EncodePublicKey, DecodePublicKey};
use spki::{SubjectPublicKeyInfoRef, SubjectPublicKeyInfo};
use rsa::pkcs1v15::{SigningKey as Pkcs1v15SigningKey, VerifyingKey as Pkcs1v15VerifyingKey};
use rsa::pss::{SigningKey as PssSigningKey, VerifyingKey as PssVerifyingKey};

// Re-export from packages for convenience
// TODO: Re-enable when crypto_asymmetric package is fully implemented
// pub use crate::stdlib::packages::crypto_asymmetric::*;

/// fr fr RSA key sizes in bits
pub const RSA_2048_BITS: usize = 2048;
pub const RSA_3072_BITS: usize = 3072; 
pub const RSA_4096_BITS: usize = 4096;

/// fr fr Elliptic curve parameters
pub const P256_CURVE: &str = "P-256";
pub const P384_CURVE: &str = "P-384";
pub const P521_CURVE: &str = "P-521";
pub const SECP256K1_CURVE: &str = "secp256k1";

/// fr fr Key sizes in bytes
pub const X25519_KEY_SIZE: usize = 32;
pub const ED25519_PUBLIC_KEY_SIZE: usize = 32;
pub const ED25519_PRIVATE_KEY_SIZE: usize = 32;
pub const ED25519_SIGNATURE_SIZE: usize = 64;

/// fr fr RSA implementation with multiple key sizes
#[derive(Debug, Clone)]
pub struct RsaKeyPair {
#[derive(Debug, Clone)]
pub struct RsaPublicKeyWrapper {
#[derive(Debug, Clone)]
pub struct RsaPrivateKeyWrapper {
/// fr fr RSA padding schemes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RsaPadding {
    Pkcs1v15,      // PKCS#1 v1.5 padding
    OaepSha256,    // OAEP with SHA-256
    OaepSha384,    // OAEP with SHA-384
    OaepSha512,    // OAEP with SHA-512
    Pss,           // PSS for signatures
impl RsaPadding {
    pub fn name(&self) -> &'static str {
        match self {
        }
    }
/// fr fr ECDSA implementation with multiple curves
#[derive(Debug, Clone)]
pub struct EcdsaKeyPair {
#[derive(Debug, Clone)]
pub struct EcdsaPublicKey {
#[derive(Debug, Clone)]
pub struct EcdsaPrivateKey {
#[derive(Debug, Clone)]
pub enum EcPublicKeyData {
#[derive(Debug, Clone)]
pub enum EcPrivateKeyData {
#[derive(Debug, Clone)]
pub struct EcdsaSignature {
#[derive(Debug, Clone)]
pub enum EcSignatureData {
/// fr fr Elliptic curve types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EcCurve {
    P256,      // NIST P-256 (secp256r1)
    P384,      // NIST P-384 (secp384r1)
    P521,      // NIST P-521 (secp521r1)
    Secp256k1, // Bitcoin curve
impl EcCurve {
    pub fn name(&self) -> &'static str {
        match self {
        }
    }
    
    pub fn key_size(&self) -> usize {
        match self {
        }
    }
    
    pub fn security_level(&self) -> u32 {
        match self {
        }
    }
/// fr fr ECDH key exchange implementation
#[derive(Debug, Clone)]
pub struct EcdhKeyPair {
#[derive(Debug, Clone)]
pub struct EcdhPublicKey {
#[derive(Debug, Clone)]
pub struct EcdhPrivateKey {
/// fr fr X25519 key exchange
#[derive(Debug, Clone)]
pub struct X25519KeyPair {
#[derive(Debug, Clone)]
pub struct X25519PublicKey {
#[derive(Debug, Clone)]
pub struct X25519PrivateKey {
/// fr fr Ed25519 digital signatures
#[derive(Debug, Clone)]
pub struct Ed25519KeyPair {
impl Ed25519KeyPair {
    /// Generate a new Ed25519 key pair
    pub fn generate<R: CryptoRng + RngCore>(rng: &mut R) -> Self {
        let signing_key = Ed25519SigningKey::generate(rng);
        let verifying_key = signing_key.verifying_key();
        
        Ed25519KeyPair {
            public_key: Ed25519PublicKey {
            private_key: Ed25519PrivateKey {
        }
    }
    
    /// Get the public key
    pub fn public(&self) -> &Ed25519PublicKey {
        &self.public_key
    /// Get the verifying key
    pub fn verifying_key(&self) -> &Ed25519VerifyingKey {
        &self.public_key.inner
    }
}

#[derive(Debug, Clone)]
pub struct Ed25519PublicKey {
#[derive(Debug, Clone)]
pub struct Ed25519PrivateKey {
#[derive(Debug, Clone)]
pub struct Ed25519Signature {
/// fr fr Asymmetric operation results
pub type AsymmetricResult<T> = std::result::Result<T, AsymmetricError>;

/// fr fr Asymmetric crypto errors
#[derive(Debug, Clone, PartialEq)]
pub enum AsymmetricError {
// impl fmt::Display for AsymmetricError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             AsymmetricError::InvalidKeySize(size) => 
//                 write!(f, "Invalid key size: {}", size),
//             AsymmetricError::InvalidCurve(curve) => 
//                 write!(f, "Invalid elliptic curve: {}", curve),
//             AsymmetricError::InvalidPadding(padding) => 
//                 write!(f, "Invalid padding scheme: {}", padding),
//             AsymmetricError::KeyGenerationFailed(msg) => 
//                 write!(f, "Key generation failed: {}", msg),
//             AsymmetricError::EncryptionFailed(msg) => 
//                 write!(f, "Encryption failed: {}", msg),
//             AsymmetricError::DecryptionFailed(msg) => 
//                 write!(f, "Decryption failed: {}", msg),
//             AsymmetricError::SigningFailed(msg) => 
//                 write!(f, "Signing failed: {}", msg),
//             AsymmetricError::VerificationFailed(msg) => 
//                 write!(f, "Verification failed: {}", msg),
//             AsymmetricError::KeyExchangeFailed(msg) => 
//                 write!(f, "Key exchange failed: {}", msg),
//             AsymmetricError::InvalidSignature => 
//                 write!(f, "Invalid signature"),
//             AsymmetricError::InvalidPublicKey => 
//                 write!(f, "Invalid public key"),
//             AsymmetricError::InvalidPrivateKey => 
//                 write!(f, "Invalid private key"),
//             AsymmetricError::UnsupportedOperation(op) => 
//                 write!(f, "Unsupported operation: {}", op),
//             AsymmetricError::InsufficientEntropy => 
//                 write!(f, "Insufficient entropy for key generation"),
//             AsymmetricError::Internal(msg) => 
//                 write!(f, "Internal error: {}", msg),
//         }
//     }
// }

// impl std::error::CursedError for AsymmetricError {}
// 
/// fr fr Configuration for asymmetric operations
#[derive(Debug, Clone)]
pub struct AsymmetricConfig {
impl Default for AsymmetricConfig {
    fn default() -> Self {
        Self {
            default_rsa_key_size: RSA_4096_BITS, // Secure default
        }
    }
/// fr fr Main asymmetric crypto engine
pub struct AsymmetricCrypto {
impl AsymmetricCrypto {
    /// slay Create new asymmetric crypto engine
    pub fn new() -> Self {
        Self::with_config(AsymmetricConfig::default())
    /// slay Create asymmetric crypto engine with custom config
    pub fn with_config(config: AsymmetricConfig) -> Self {
        Self { config }
    }

    // RSA Operations
    
    /// slay Generate RSA key pair
    pub fn rsa_generate_keypair(&self, key_size: Option<usize>) -> AsymmetricResult<RsaKeyPair> {
        let size = key_size.unwrap_or(self.config.default_rsa_key_size);
        
        // Validate key size
        match size {
        let mut rng = OsRng;
        let private_key = RsaPrivateKey::new(&mut rng, size)
            .map_err(|e| AsymmetricError::KeyGenerationFailed(format!("RSA key generation failed: {}", e)))?;
        
        let public_key = private_key.to_public_key();
        
        Ok(RsaKeyPair {
            public_key: RsaPublicKeyWrapper {
            private_key: RsaPrivateKeyWrapper {
        })
    /// slay RSA encrypt with public key
    pub fn rsa_encrypt(&self, public_key: &RsaPublicKeyWrapper, plaintext: &[u8], padding: Option<RsaPadding>) -> AsymmetricResult<Vec<u8>> {
        let pad = padding.unwrap_or(self.config.default_rsa_padding);
        let mut rng = OsRng;
        
        let ciphertext = match pad {
            RsaPadding::Pkcs1v15 => {
                public_key.inner.encrypt(&mut rng, Pkcs1v15Encrypt, plaintext)
                    .map_err(|e| AsymmetricError::EncryptionFailed(format!("PKCS1v15 encryption failed: {}", e)))?
            RsaPadding::OaepSha256 => {
                public_key.inner.encrypt(&mut rng, Oaep::new::<Sha256>(), plaintext)
                    .map_err(|e| AsymmetricError::EncryptionFailed(format!("OAEP-SHA256 encryption failed: {}", e)))?
            RsaPadding::OaepSha384 => {
                public_key.inner.encrypt(&mut rng, Oaep::new::<Sha384>(), plaintext)
                    .map_err(|e| AsymmetricError::EncryptionFailed(format!("OAEP-SHA384 encryption failed: {}", e)))?
            RsaPadding::OaepSha512 => {
                public_key.inner.encrypt(&mut rng, Oaep::new::<Sha512>(), plaintext)
                    .map_err(|e| AsymmetricError::EncryptionFailed(format!("OAEP-SHA512 encryption failed: {}", e)))?
        
        Ok(ciphertext)
    /// slay RSA decrypt with private key
    pub fn rsa_decrypt(&self, private_key: &RsaPrivateKeyWrapper, ciphertext: &[u8], padding: Option<RsaPadding>) -> AsymmetricResult<Vec<u8>> {
        let pad = padding.unwrap_or(self.config.default_rsa_padding);
        
        let plaintext = match pad {
            RsaPadding::Pkcs1v15 => {
                private_key.inner.decrypt(Pkcs1v15Encrypt, ciphertext)
                    .map_err(|e| AsymmetricError::DecryptionFailed(format!("PKCS1v15 decryption failed: {}", e)))?
            RsaPadding::OaepSha256 => {
                private_key.inner.decrypt(Oaep::new::<Sha256>(), ciphertext)
                    .map_err(|e| AsymmetricError::DecryptionFailed(format!("OAEP-SHA256 decryption failed: {}", e)))?
            RsaPadding::OaepSha384 => {
                private_key.inner.decrypt(Oaep::new::<Sha384>(), ciphertext)
                    .map_err(|e| AsymmetricError::DecryptionFailed(format!("OAEP-SHA384 decryption failed: {}", e)))?
            RsaPadding::OaepSha512 => {
                private_key.inner.decrypt(Oaep::new::<Sha512>(), ciphertext)
                    .map_err(|e| AsymmetricError::DecryptionFailed(format!("OAEP-SHA512 decryption failed: {}", e)))?
        
        Ok(plaintext)
    /// slay RSA sign with private key
    pub fn rsa_sign(&self, private_key: &RsaPrivateKeyWrapper, message: &[u8], padding: Option<RsaPadding>) -> AsymmetricResult<Vec<u8>> {
        let pad = padding.unwrap_or(RsaPadding::Pss);
        let mut rng = OsRng;
        
        let signature = match pad {
            RsaPadding::Pkcs1v15 => {
                let signing_key = rsa::pkcs1v15::SigningKey::<Sha256>::new(private_key.inner.clone());
                signing_key.sign_with_rng(&mut rng, message).to_bytes().as_ref().to_vec()
            RsaPadding::Pss => {
                let signing_key = rsa::pss::SigningKey::<Sha256>::new(private_key.inner.clone());
                signing_key.sign_with_rng(&mut rng, message).to_bytes().as_ref().to_vec()
        
        Ok(signature)
    /// slay RSA verify signature with public key
    pub fn rsa_verify(&self, public_key: &RsaPublicKeyWrapper, message: &[u8], signature: &[u8], padding: Option<RsaPadding>) -> AsymmetricResult<bool> {
        let pad = padding.unwrap_or(RsaPadding::Pss);
        
        let result = match pad {
            RsaPadding::Pkcs1v15 => {
                let verifying_key = rsa::pkcs1v15::VerifyingKey::<Sha256>::new(public_key.inner.clone());
                let sig = rsa::pkcs1v15::Signature::try_from(signature)
                    .map_err(|_| AsymmetricError::InvalidSignature)?;
                verifying_key.verify(message, &sig).is_ok()
            RsaPadding::Pss => {
                let verifying_key = rsa::pss::VerifyingKey::<Sha256>::new(public_key.inner.clone());
                let sig = rsa::pss::Signature::try_from(signature)
                    .map_err(|_| AsymmetricError::InvalidSignature)?;
                verifying_key.verify(message, &sig).is_ok()
        
        Ok(result)
    // ECDSA Operations
    
    /// slay Generate ECDSA key pair
    pub fn ecdsa_generate_keypair(&self, curve: Option<EcCurve>) -> AsymmetricResult<EcdsaKeyPair> {
        let ec_curve = curve.unwrap_or(self.config.default_ec_curve);
        let mut rng = OsRng;
        
        match ec_curve {
            EcCurve::P256 => {
                let secret_key = P256SecretKey::random(&mut rng);
                let public_key = secret_key.public_key();
                
                Ok(EcdsaKeyPair {
                    public_key: EcdsaPublicKey {
                    private_key: EcdsaPrivateKey {
                })
            EcCurve::P384 => {
                let secret_key = P384SecretKey::random(&mut rng);
                let public_key = secret_key.public_key();
                
                Ok(EcdsaKeyPair {
                    public_key: EcdsaPublicKey {
                    private_key: EcdsaPrivateKey {
                })
            EcCurve::Secp256k1 => {
                let secret_key = K256SecretKey::random(&mut rng);
                let public_key = secret_key.public_key();
                
                Ok(EcdsaKeyPair {
                    public_key: EcdsaPublicKey {
                    private_key: EcdsaPrivateKey {
                })
            EcCurve::P521 => {
                // P-521 implementation would require p521 crate
                Err(AsymmetricError::UnsupportedOperation("P-521 requires p521 crate dependency".to_string()))
        }
    }
    
    /// slay ECDSA sign message
    pub fn ecdsa_sign(&self, private_key: &EcdsaPrivateKey, message: &[u8]) -> AsymmetricResult<EcdsaSignature> {
        match &private_key.data {
            EcPrivateKeyData::P256(secret_key) => {
                let signing_key = P256SigningKey::from(secret_key);
                let signature: P256Signature = signing_key.sign(message);
                
                Ok(EcdsaSignature {
                })
            EcPrivateKeyData::P384(secret_key) => {
                let signing_key = P384SigningKey::from(secret_key);
                let signature: P384Signature = signing_key.sign(message);
                
                Ok(EcdsaSignature {
                })
            EcPrivateKeyData::K256(secret_key) => {
                let signing_key = K256SigningKey::from(secret_key);
                let signature: K256Signature = signing_key.sign(message);
                
                Ok(EcdsaSignature {
                })
        }
    }
    
    /// slay ECDSA verify signature
    pub fn ecdsa_verify(&self, public_key: &EcdsaPublicKey, message: &[u8], signature: &EcdsaSignature) -> AsymmetricResult<bool> {
        if public_key.curve != signature.curve {
            return Ok(false);
        let result = match (&public_key.data, &signature.data) {
            (EcPublicKeyData::P256(pk), EcSignatureData::P256(sig)) => {
                let verifying_key = P256VerifyingKey::from(pk);
                verifying_key.verify(message, sig).is_ok()
            (EcPublicKeyData::P384(pk), EcSignatureData::P384(sig)) => {
                let verifying_key = P384VerifyingKey::from(pk);
                verifying_key.verify(message, sig).is_ok()
            (EcPublicKeyData::K256(pk), EcSignatureData::K256(sig)) => {
                let verifying_key = K256VerifyingKey::from(pk);
                verifying_key.verify(message, sig).is_ok()
        
        Ok(result)
    // ECDH Operations
    
    /// slay Generate ECDH key pair
    pub fn ecdh_generate_keypair(&self, curve: Option<EcCurve>) -> AsymmetricResult<EcdhKeyPair> {
        let ec_curve = curve.unwrap_or(self.config.default_ec_curve);
        let mut rng = OsRng;
        
        match ec_curve {
            EcCurve::P256 => {
                let secret_key = P256SecretKey::random(&mut rng);
                let public_key = secret_key.public_key();
                
                Ok(EcdhKeyPair {
                    public_key: EcdhPublicKey {
                    private_key: EcdhPrivateKey {
                })
            EcCurve::Secp256k1 => {
                let secret_key = K256SecretKey::random(&mut rng);
                let public_key = secret_key.public_key();
                
                Ok(EcdhKeyPair {
                    public_key: EcdhPublicKey {
                    private_key: EcdhPrivateKey {
                })
        }
    }
    
    /// slay ECDH key exchange
    pub fn ecdh_exchange(&self, private_key: &EcdhPrivateKey, public_key: &EcdhPublicKey) -> AsymmetricResult<Vec<u8>> {
        if private_key.curve != public_key.curve {
            return Err(AsymmetricError::KeyExchangeFailed("Curve mismatch".to_string()));
        match (&private_key.data, &public_key.data) {
            (EcPrivateKeyData::P256(sk), EcPublicKeyData::P256(pk)) => {
                let shared_secret = elliptic_curve::ecdh::diffie_hellman(sk.to_nonzero_scalar(), pk.as_affine());
                Ok(shared_secret.raw_secret_bytes().to_vec())
            (EcPrivateKeyData::K256(sk), EcPublicKeyData::K256(pk)) => {
                let shared_secret = elliptic_curve::ecdh::diffie_hellman(sk.to_nonzero_scalar(), pk.as_affine());
                Ok(shared_secret.raw_secret_bytes().to_vec())
        }
    }

    // X25519 Operations
    
    /// slay Generate X25519 key pair
    pub fn x25519_generate_keypair(&self) -> AsymmetricResult<X25519KeyPair> {
        let mut rng = OsRng;
        let secret = EphemeralSecret::random();
        let public: X25519PublicKeyInternal = (&secret).into();
        
        Ok(X25519KeyPair {
            public_key: X25519PublicKey {
            private_key: X25519PrivateKey {
        })
    /// slay X25519 key exchange
    pub fn x25519_exchange(&self, private_key: &X25519PrivateKey, public_key: &X25519PublicKey) -> AsymmetricResult<[u8; 32]> {
        let public = X25519PublicKeyInternal::from(public_key.bytes);
        let shared_secret = private_key.secret.diffie_hellman(&public);
        Ok(*shared_secret.as_bytes())
    // Ed25519 Operations
    
    /// slay Generate Ed25519 key pair
    pub fn ed25519_generate_keypair(&self) -> AsymmetricResult<Ed25519KeyPair> {
        let mut rng = OsRng;
        let signing_key = Ed25519SigningKey::generate(&mut rng);
        let verifying_key = signing_key.verifying_key();
        
        Ok(Ed25519KeyPair {
            public_key: Ed25519PublicKey {
            private_key: Ed25519PrivateKey {
        })
    /// slay Ed25519 sign message
    pub fn ed25519_sign(&self, private_key: &Ed25519PrivateKey, message: &[u8]) -> AsymmetricResult<Ed25519Signature> {
        let signature = private_key.inner.sign(message);
        
        Ok(Ed25519Signature {
        })
    /// slay Ed25519 verify signature
    pub fn ed25519_verify(&self, public_key: &Ed25519PublicKey, message: &[u8], signature: &Ed25519Signature) -> AsymmetricResult<bool> {
        let result = public_key.inner.verify(message, &signature.inner).is_ok();
        Ok(result)
    // Key Serialization Operations

    /// slay Serialize RSA public key to PEM format
    pub fn rsa_public_key_to_pem(&self, public_key: &RsaPublicKeyWrapper) -> AsymmetricResult<String> {
        let der_bytes = public_key.inner.to_public_key_der()
            .map_err(|e| AsymmetricError::Internal(format!("DER encoding failed: {}", e)))?;
        
        let pem = encode(&Pem {
        });
        
        Ok(pem)
    /// slay Serialize RSA private key to PEM format
    pub fn rsa_private_key_to_pem(&self, private_key: &RsaPrivateKeyWrapper) -> AsymmetricResult<String> {
        let der_bytes = private_key.inner.to_pkcs8_der()
            .map_err(|e| AsymmetricError::Internal(format!("PKCS8 encoding failed: {}", e)))?;
        
        let pem = encode(&Pem {
        });
        
        Ok(pem)
    /// slay Parse RSA public key from PEM format
    pub fn rsa_public_key_from_pem(&self, pem_data: &str) -> AsymmetricResult<RsaPublicKeyWrapper> {
        let pem = parse(pem_data)
            .map_err(|e| AsymmetricError::InvalidPublicKey)?;
        
        if pem.tag != "PUBLIC KEY" {
            return Err(AsymmetricError::InvalidPublicKey);
        let public_key = RsaPublicKey::from_public_key_der(&pem.contents)
            .map_err(|e| AsymmetricError::InvalidPublicKey)?;
        
        let key_size = public_key.size() * 8; // Convert bytes to bits
        
        Ok(RsaPublicKeyWrapper {
        })
    /// slay Parse RSA private key from PEM format
    pub fn rsa_private_key_from_pem(&self, pem_data: &str) -> AsymmetricResult<RsaPrivateKeyWrapper> {
        let pem = parse(pem_data)
            .map_err(|e| AsymmetricError::InvalidPrivateKey)?;
        
        if pem.tag != "PRIVATE KEY" {
            return Err(AsymmetricError::InvalidPrivateKey);
        let private_key = RsaPrivateKey::from_pkcs8_der(&pem.contents)
            .map_err(|e| AsymmetricError::InvalidPrivateKey)?;
        
        let key_size = private_key.size() * 8; // Convert bytes to bits
        
        Ok(RsaPrivateKeyWrapper {
        })
    /// slay Serialize ECDSA public key to base64 format
    pub fn ecdsa_public_key_to_base64(&self, public_key: &EcdsaPublicKey) -> AsymmetricResult<String> {
        let bytes = match &public_key.data {
            EcPublicKeyData::P256(pk) => {
                pk.to_encoded_point(false).as_bytes().to_vec()
            EcPublicKeyData::P384(pk) => {
                pk.to_encoded_point(false).as_bytes().to_vec()
            EcPublicKeyData::K256(pk) => {
                pk.to_encoded_point(false).as_bytes().to_vec()
        
        Ok(general_purpose::STANDARD.encode(&bytes))
    /// slay Serialize ECDSA private key to base64 format (WARNING: Insecure)
    pub fn ecdsa_private_key_to_base64(&self, private_key: &EcdsaPrivateKey) -> AsymmetricResult<String> {
        let bytes = match &private_key.data {
            EcPrivateKeyData::P256(sk) => {
                sk.to_bytes().to_vec()
            EcPrivateKeyData::P384(sk) => {
                sk.to_bytes().to_vec()
            EcPrivateKeyData::K256(sk) => {
                sk.to_bytes().to_vec()
        
        Ok(general_purpose::STANDARD.encode(&bytes))
    /// slay Serialize X25519 public key to base64 format
    pub fn x25519_public_key_to_base64(&self, public_key: &X25519PublicKey) -> AsymmetricResult<String> {
        Ok(general_purpose::STANDARD.encode(&public_key.bytes))
    /// slay Serialize Ed25519 public key to base64 format
    pub fn ed25519_public_key_to_base64(&self, public_key: &Ed25519PublicKey) -> AsymmetricResult<String> {
        Ok(general_purpose::STANDARD.encode(&public_key.inner.to_bytes()))
    /// slay Serialize Ed25519 signature to base64 format
    pub fn ed25519_signature_to_base64(&self, signature: &Ed25519Signature) -> AsymmetricResult<String> {
        Ok(general_purpose::STANDARD.encode(&signature.inner.to_bytes()))
    /// slay Parse Ed25519 signature from base64 format
    pub fn ed25519_signature_from_base64(&self, signature_data: &str) -> AsymmetricResult<Ed25519Signature> {
        let bytes = general_purpose::STANDARD.decode(signature_data)
            .map_err(|e| AsymmetricError::InvalidSignature)?;
        
        if bytes.len() != ED25519_SIGNATURE_SIZE {
            return Err(AsymmetricError::InvalidSignature);
        let signature_bytes: [u8; 64] = bytes.try_into()
            .map_err(|_| AsymmetricError::InvalidSignature)?;
        
        let signature = Ed25519SignatureInternal::from_bytes(&signature_bytes)
            .map_err(|e| AsymmetricError::InvalidSignature)?;
        
        Ok(Ed25519Signature { inner: signature })
    /// slay Parse ECDSA private key from base64 format
    pub fn ecdsa_private_key_from_base64(&self, key_data: &str, curve: EcCurve) -> AsymmetricResult<EcdsaPrivateKey> {
        let bytes = general_purpose::STANDARD.decode(key_data)
            .map_err(|e| AsymmetricError::InvalidPrivateKey)?;
        
        let data = match curve {
            EcCurve::P256 => {
                if bytes.len() != 32 {
                    return Err(AsymmetricError::InvalidPrivateKey);
                }
                let secret_key = P256SecretKey::from_slice(&bytes)
                    .map_err(|e| AsymmetricError::InvalidPrivateKey)?;
                EcPrivateKeyData::P256(secret_key)
            EcCurve::P384 => {
                if bytes.len() != 48 {
                    return Err(AsymmetricError::InvalidPrivateKey);
                }
                let secret_key = P384SecretKey::from_slice(&bytes)
                    .map_err(|e| AsymmetricError::InvalidPrivateKey)?;
                EcPrivateKeyData::P384(secret_key)
            EcCurve::Secp256k1 => {
                if bytes.len() != 32 {
                    return Err(AsymmetricError::InvalidPrivateKey);
                }
                let secret_key = K256SecretKey::from_slice(&bytes)
                    .map_err(|e| AsymmetricError::InvalidPrivateKey)?;
                EcPrivateKeyData::K256(secret_key)
        
        Ok(EcdsaPrivateKey { curve, data })
    /// slay Parse ECDSA public key from base64 format
    pub fn ecdsa_public_key_from_base64(&self, key_data: &str, curve: EcCurve) -> AsymmetricResult<EcdsaPublicKey> {
        let bytes = general_purpose::STANDARD.decode(key_data)
            .map_err(|e| AsymmetricError::InvalidPublicKey)?;
        
        let data = match curve {
            EcCurve::P256 => {
                let public_key = P256PublicKey::from_sec1_bytes(&bytes)
                    .map_err(|e| AsymmetricError::InvalidPublicKey)?;
                EcPublicKeyData::P256(public_key)
            EcCurve::P384 => {
                let public_key = P384PublicKey::from_sec1_bytes(&bytes)
                    .map_err(|e| AsymmetricError::InvalidPublicKey)?;
                EcPublicKeyData::P384(public_key)
            EcCurve::Secp256k1 => {
                let public_key = K256PublicKey::from_sec1_bytes(&bytes)
                    .map_err(|e| AsymmetricError::InvalidPublicKey)?;
                EcPublicKeyData::K256(public_key)
        
        Ok(EcdsaPublicKey { curve, data })
    /// slay Parse ECDSA signature from base64 format
    pub fn ecdsa_signature_from_base64(&self, signature_data: &str, curve: EcCurve) -> AsymmetricResult<EcdsaSignature> {
        let bytes = general_purpose::STANDARD.decode(signature_data)
            .map_err(|e| AsymmetricError::InvalidSignature)?;
        
        let data = match curve {
            EcCurve::P256 => {
                let signature = P256Signature::from_der(&bytes)
                    .map_err(|e| AsymmetricError::InvalidSignature)?;
                EcSignatureData::P256(signature)
            EcCurve::P384 => {
                let signature = P384Signature::from_der(&bytes)
                    .map_err(|e| AsymmetricError::InvalidSignature)?;
                EcSignatureData::P384(signature)
            EcCurve::Secp256k1 => {
                let signature = K256Signature::from_der(&bytes)
                    .map_err(|e| AsymmetricError::InvalidSignature)?;
                EcSignatureData::K256(signature)
        
        Ok(EcdsaSignature { curve, data })
    /// slay Serialize ECDSA signature to base64 format
    pub fn ecdsa_signature_to_base64(&self, signature: &EcdsaSignature) -> AsymmetricResult<String> {
        let bytes = match &signature.data {
            EcSignatureData::P256(sig) => {
                sig.to_der().as_ref().to_vec()
            EcSignatureData::P384(sig) => {
                sig.to_der().as_ref().to_vec()
            EcSignatureData::K256(sig) => {
                sig.to_der().as_ref().to_vec()
        
        Ok(general_purpose::STANDARD.encode(&bytes))
    /// slay Parse Ed25519 private key from base64 format
    pub fn ed25519_private_key_from_base64(&self, key_data: &str) -> AsymmetricResult<Ed25519PrivateKey> {
        let bytes = general_purpose::STANDARD.decode(key_data)
            .map_err(|e| AsymmetricError::InvalidPrivateKey)?;
        
        if bytes.len() != ED25519_PRIVATE_KEY_SIZE {
            return Err(AsymmetricError::InvalidPrivateKey);
        let signing_key = Ed25519SigningKey::from_bytes(&bytes.try_into().unwrap())
            .map_err(|e| AsymmetricError::InvalidPrivateKey)?;
        
        Ok(Ed25519PrivateKey { inner: signing_key })
    /// slay Parse Ed25519 public key from base64 format
    pub fn ed25519_public_key_from_base64(&self, key_data: &str) -> AsymmetricResult<Ed25519PublicKey> {
        let bytes = general_purpose::STANDARD.decode(key_data)
            .map_err(|e| AsymmetricError::InvalidPublicKey)?;
        
        if bytes.len() != ED25519_PUBLIC_KEY_SIZE {
            return Err(AsymmetricError::InvalidPublicKey);
        let verifying_key = Ed25519VerifyingKey::from_bytes(&bytes.try_into().unwrap())
            .map_err(|e| AsymmetricError::InvalidPublicKey)?;
        
        Ok(Ed25519PublicKey { inner: verifying_key })
    }
}

impl Default for AsymmetricCrypto {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Public API functions for CURSED stdlib integration

/// slay RSA key generation
pub fn rsa_generate_keypair(args: Vec<Value>) -> crate::error::Result<()> {
    let crypto = AsymmetricCrypto::new();
    
    let key_size = if args.is_empty() {
        None
    } else {
        match &args[0] {
            _ => Some(RSA_4096_BITS), // Default to secure size
        }
    
    match crypto.rsa_generate_keypair(key_size) {
        Ok(keypair) => {
            let mut result = HashMap::new();
            result.insert("key_size".to_string(), Value::Number(keypair.key_size as f64));
            result.insert("algorithm".to_string(), Value::String("RSA".to_string()));
            Ok(Value::Object(result))
        }
        Err(e) => Err(CursedError::Runtime(format!("RSA key generation failed: {}", e)))
    }
}

/// slay RSA encryption
pub fn rsa_encrypt(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("RSA encrypt requires public key and plaintext".to_string()));
    let public_key_pem = match &args[0] {
    
    let plaintext = match &args[1] {
    
    let crypto = AsymmetricCrypto::new();
    match crypto.rsa_public_key_from_pem(public_key_pem) {
        Ok(public_key) => {
            match crypto.rsa_encrypt(&public_key, plaintext, None) {
                Err(e) => Err(CursedError::Runtime(format!("RSA encryption failed: {}", e)))
            }
        }
        Err(e) => Err(CursedError::Runtime(format!("Invalid public key: {}", e)))
    }
}

/// slay RSA decryption  
pub fn rsa_decrypt(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("RSA decrypt requires private key and ciphertext".to_string()));
    let private_key_pem = match &args[0] {
    
    let ciphertext_b64 = match &args[1] {
    
    let ciphertext = general_purpose::STANDARD.decode(ciphertext_b64)
        .map_err(|e| CursedError::Runtime(format!("Invalid base64 ciphertext: {}", e)))?;
    
    let crypto = AsymmetricCrypto::new();
    match crypto.rsa_private_key_from_pem(private_key_pem) {
        Ok(private_key) => {
            match crypto.rsa_decrypt(&private_key, &ciphertext, None) {
                Err(e) => Err(CursedError::Runtime(format!("RSA decryption failed: {}", e)))
            }
        }
        Err(e) => Err(CursedError::Runtime(format!("Invalid private key: {}", e)))
    }
}

/// slay RSA signing
pub fn rsa_sign(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("RSA sign requires private key and message".to_string()));
    let private_key_pem = match &args[0] {
    
    let message = match &args[1] {
    
    // Optional padding scheme (default to PSS)
    let padding = if args.len() >= 3 {
        match &args[2] {
            Value::String(p) => match p.as_str() {
        }
    } else {
        Some(RsaPadding::Pss)
    
    let crypto = AsymmetricCrypto::new();
    match crypto.rsa_private_key_from_pem(private_key_pem) {
        Ok(private_key) => {
            match crypto.rsa_sign(&private_key, message, padding) {
                Err(e) => Err(CursedError::Runtime(format!("RSA signing failed: {}", e)))
            }
        }
        Err(e) => Err(CursedError::Runtime(format!("Invalid private key: {}", e)))
    }
}

/// slay RSA signature verification
pub fn rsa_verify(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 3 {
        return Err(CursedError::Runtime("RSA verify requires public key, message, and signature".to_string()));
    let public_key_pem = match &args[0] {
    
    let message = match &args[1] {
    
    let signature_b64 = match &args[2] {
    
    // Optional padding scheme (default to PSS)
    let padding = if args.len() >= 4 {
        match &args[3] {
            Value::String(p) => match p.as_str() {
        }
    } else {
        Some(RsaPadding::Pss)
    
    let signature = general_purpose::STANDARD.decode(signature_b64)
        .map_err(|e| CursedError::Runtime(format!("Invalid base64 signature: {}", e)))?;
    
    let crypto = AsymmetricCrypto::new();
    match crypto.rsa_public_key_from_pem(public_key_pem) {
        Ok(public_key) => {
            match crypto.rsa_verify(&public_key, message, &signature, padding) {
                Err(e) => Err(CursedError::Runtime(format!("RSA verification failed: {}", e)))
            }
        }
        Err(e) => Err(CursedError::Runtime(format!("Invalid public key: {}", e)))
    }
}

/// slay ECDSA key generation
pub fn ecdsa_generate_keypair(args: Vec<Value>) -> crate::error::Result<()> {
    let crypto = AsymmetricCrypto::new();
    
    let curve = if args.is_empty() {
        None
    } else {
        match &args[0] {
            Value::String(curve_name) => {
                match curve_name.as_str() {
                    _ => Some(EcCurve::P256), // Default
                }
            _ => Some(EcCurve::P256), // Default to P-256
        }
    
    match crypto.ecdsa_generate_keypair(curve) {
        Ok(keypair) => {
            let mut result = HashMap::new();
            result.insert("curve".to_string(), Value::String(keypair.curve.name().to_string()));
            result.insert("algorithm".to_string(), Value::String("ECDSA".to_string()));
            Ok(Value::Object(result))
        }
        Err(e) => Err(CursedError::Runtime(format!("ECDSA key generation failed: {}", e)))
    }
}

/// slay ECDSA signing
pub fn ecdsa_sign(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("ECDSA sign requires private key and message".to_string()));
    let private_key_b64 = match &args[0] {
    
    let message = match &args[1] {
    
    // Curve specification (default to P-256)
    let curve = if args.len() >= 3 {
        match &args[2] {
            Value::String(curve_name) => {
                match curve_name.as_str() {
                }
        }
    } else {
        EcCurve::P256
    
    let crypto = AsymmetricCrypto::new();
    match crypto.ecdsa_private_key_from_base64(private_key_b64, curve) {
        Ok(private_key) => {
            match crypto.ecdsa_sign(&private_key, message) {
                Ok(signature) => {
                    match crypto.ecdsa_signature_to_base64(&signature) {
                        Err(e) => Err(CursedError::Runtime(format!("Signature encoding failed: {}", e)))
                    }
                Err(e) => Err(CursedError::Runtime(format!("ECDSA signing failed: {}", e)))
            }
        }
        Err(e) => Err(CursedError::Runtime(format!("Invalid private key: {}", e)))
    }
}

/// slay ECDSA verification
pub fn ecdsa_verify(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 3 {
        return Err(CursedError::Runtime("ECDSA verify requires public key, message, and signature".to_string()));
    let public_key_b64 = match &args[0] {
    
    let message = match &args[1] {
    
    let signature_b64 = match &args[2] {
    
    // Curve specification (default to P-256)
    let curve = if args.len() >= 4 {
        match &args[3] {
            Value::String(curve_name) => {
                match curve_name.as_str() {
                }
        }
    } else {
        EcCurve::P256
    
    let crypto = AsymmetricCrypto::new();
    match crypto.ecdsa_public_key_from_base64(public_key_b64, curve) {
        Ok(public_key) => {
            match crypto.ecdsa_signature_from_base64(signature_b64, curve) {
                Ok(signature) => {
                    match crypto.ecdsa_verify(&public_key, message, &signature) {
                        Err(e) => Err(CursedError::Runtime(format!("ECDSA verification failed: {}", e)))
                    }
                Err(e) => Err(CursedError::Runtime(format!("Invalid signature: {}", e)))
            }
        }
        Err(e) => Err(CursedError::Runtime(format!("Invalid public key: {}", e)))
    }
}

/// slay ECDH key exchange
pub fn ecdh_key_exchange(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("ECDH requires private key and public key".to_string()));
    let private_key_b64 = match &args[0] {
    
    let public_key_b64 = match &args[1] {
    
    // Curve specification (default to P-256)
    let curve = if args.len() >= 3 {
        match &args[2] {
            Value::String(curve_name) => {
                match curve_name.as_str() {
                }
        }
    } else {
        EcCurve::P256
    
    let crypto = AsymmetricCrypto::new();
    
    // Parse private key
    let private_key_data = match curve {
        EcCurve::P256 => {
            let bytes = general_purpose::STANDARD.decode(private_key_b64)
                .map_err(|e| CursedError::Runtime(format!("Invalid private key base64: {}", e)))?;
            if bytes.len() != 32 {
                return Err(CursedError::Runtime("P-256 private key must be 32 bytes".to_string()));
            }
            let secret_key = P256SecretKey::from_slice(&bytes)
                .map_err(|e| CursedError::Runtime(format!("Invalid P-256 private key: {}", e)))?;
            EcPrivateKeyData::P256(secret_key)
        EcCurve::Secp256k1 => {
            let bytes = general_purpose::STANDARD.decode(private_key_b64)
                .map_err(|e| CursedError::Runtime(format!("Invalid private key base64: {}", e)))?;
            if bytes.len() != 32 {
                return Err(CursedError::Runtime("secp256k1 private key must be 32 bytes".to_string()));
            }
            let secret_key = K256SecretKey::from_slice(&bytes)
                .map_err(|e| CursedError::Runtime(format!("Invalid secp256k1 private key: {}", e)))?;
            EcPrivateKeyData::K256(secret_key)
    
    let private_key = EcdhPrivateKey {
    
    // Parse public key
    let public_key_data = match curve {
        EcCurve::P256 => {
            let bytes = general_purpose::STANDARD.decode(public_key_b64)
                .map_err(|e| CursedError::Runtime(format!("Invalid public key base64: {}", e)))?;
            let public_key = P256PublicKey::from_sec1_bytes(&bytes)
                .map_err(|e| CursedError::Runtime(format!("Invalid P-256 public key: {}", e)))?;
            EcPublicKeyData::P256(public_key)
        EcCurve::Secp256k1 => {
            let bytes = general_purpose::STANDARD.decode(public_key_b64)
                .map_err(|e| CursedError::Runtime(format!("Invalid public key base64: {}", e)))?;
            let public_key = K256PublicKey::from_sec1_bytes(&bytes)
                .map_err(|e| CursedError::Runtime(format!("Invalid secp256k1 public key: {}", e)))?;
            EcPublicKeyData::K256(public_key)
    
    let public_key = EcdhPublicKey {
    
    match crypto.ecdh_exchange(&private_key, &public_key) {
        Err(e) => Err(CursedError::Runtime(format!("ECDH key exchange failed: {}", e)))
    }
}

/// slay X25519 key generation
pub fn x25519_generate_keypair(_args: Vec<Value>) -> crate::error::Result<()> {
    let crypto = AsymmetricCrypto::new();
    
    match crypto.x25519_generate_keypair() {
        Ok(_keypair) => {
            let mut result = HashMap::new();
            result.insert("algorithm".to_string(), Value::String("X25519".to_string()));
            result.insert("key_size".to_string(), Value::Number(32.0));
            Ok(Value::Object(result))
        }
        Err(e) => Err(CursedError::Runtime(format!("X25519 key generation failed: {}", e)))
    }
}

/// slay X25519 key exchange
pub fn x25519_key_exchange(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("X25519 exchange requires private key and public key".to_string()));
    let private_key_b64 = match &args[0] {
    
    let public_key_b64 = match &args[1] {
    
    let crypto = AsymmetricCrypto::new();
    
    // Parse private key
    let private_key_bytes = general_purpose::STANDARD.decode(private_key_b64)
        .map_err(|e| CursedError::Runtime(format!("Invalid private key base64: {}", e)))?;
    
    if private_key_bytes.len() != 32 {
        return Err(CursedError::Runtime("X25519 private key must be 32 bytes".to_string()));
    let private_key_array: [u8; 32] = private_key_bytes.try_into()
        .map_err(|_| CursedError::Runtime("Failed to convert private key to 32-byte array".to_string()))?;
    
    let private_key = X25519PrivateKey {
    
    // Parse public key
    let public_key_bytes = general_purpose::STANDARD.decode(public_key_b64)
        .map_err(|e| CursedError::Runtime(format!("Invalid public key base64: {}", e)))?;
    
    if public_key_bytes.len() != 32 {
        return Err(CursedError::Runtime("X25519 public key must be 32 bytes".to_string()));
    let public_key_array: [u8; 32] = public_key_bytes.try_into()
        .map_err(|_| CursedError::Runtime("Failed to convert public key to 32-byte array".to_string()))?;
    
    let public_key = X25519PublicKey {
    
    match crypto.x25519_exchange(&private_key, &public_key) {
        Err(e) => Err(CursedError::Runtime(format!("X25519 key exchange failed: {}", e)))
    }
}

/// slay Ed25519 key generation
pub fn ed25519_generate_keypair(_args: Vec<Value>) -> crate::error::Result<()> {
    let crypto = AsymmetricCrypto::new();
    
    match crypto.ed25519_generate_keypair() {
        Ok(_keypair) => {
            let mut result = HashMap::new();
            result.insert("algorithm".to_string(), Value::String("Ed25519".to_string()));
            result.insert("key_size".to_string(), Value::Number(32.0));
            Ok(Value::Object(result))
        }
        Err(e) => Err(CursedError::Runtime(format!("Ed25519 key generation failed: {}", e)))
    }
}

/// slay Ed25519 signing
pub fn ed25519_sign(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("Ed25519 sign requires private key and message".to_string()));
    let private_key_b64 = match &args[0] {
    
    let message = match &args[1] {
    
    let crypto = AsymmetricCrypto::new();
    match crypto.ed25519_private_key_from_base64(private_key_b64) {
        Ok(private_key) => {
            match crypto.ed25519_sign(&private_key, message) {
                Ok(signature) => {
                    match crypto.ed25519_signature_to_base64(&signature) {
                        Err(e) => Err(CursedError::Runtime(format!("Signature encoding failed: {}", e)))
                    }
                Err(e) => Err(CursedError::Runtime(format!("Ed25519 signing failed: {}", e)))
            }
        }
        Err(e) => Err(CursedError::Runtime(format!("Invalid private key: {}", e)))
    }
}

/// slay Ed25519 verification
pub fn ed25519_verify(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 3 {
        return Err(CursedError::Runtime("Ed25519 verify requires public key, message, and signature".to_string()));
    let public_key_b64 = match &args[0] {
    
    let message = match &args[1] {
    
    let signature_b64 = match &args[2] {
    
    let crypto = AsymmetricCrypto::new();
    match crypto.ed25519_public_key_from_base64(public_key_b64) {
        Ok(public_key) => {
            match crypto.ed25519_signature_from_base64(signature_b64) {
                Ok(signature) => {
                    match crypto.ed25519_verify(&public_key, message, &signature) {
                        Err(e) => Err(CursedError::Runtime(format!("Ed25519 verification failed: {}", e)))
                    }
                Err(e) => Err(CursedError::Runtime(format!("Invalid signature: {}", e)))
            }
        }
        Err(e) => Err(CursedError::Runtime(format!("Invalid public key: {}", e)))
    }
}

