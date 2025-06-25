// Post-Quantum Cryptography Module for CURSED
// 
// This module provides implementations of post-quantum cryptographic algorithms
// that are believed to be secure against attacks by quantum computers.
// 
// # Algorithms Supported
// 
// - **Kyber**: Key Encapsulation Mechanism (KEM) based on Module-LWE
// - **Dilithium**: Digital signatures based on Module-LWE
// - **SPHINCS+**: Hash-based signatures (stateless)
// - **Falcon**: Compact signatures based on NTRU lattices
// - **NTRU**: Encryption based on NTRU lattices
// 
// # Security Considerations
// 
// All implementations follow NIST PQC standardization guidelines and provide
// multiple security levels corresponding to classical cryptographic strength:
// - Level 1: Equivalent to AES-128
// - Level 3: Equivalent to AES-192  
// - Level 5: Equivalent to AES-256
// 
// # Usage Example
// 
// ```rust
// use cursed::stdlib::crypto::pqc::{KyberKem, SecurityLevel};
// 
// // Generate Kyber-768 key pair (NIST Level 3)
// let (public_key, secret_key) = KyberKem::keygen(SecurityLevel::Level3)?;
// 
// // Encapsulation
// let (ciphertext, shared_secret) = KyberKem::encaps(&public_key)?;
// 
// // Decapsulation
// let decaps_secret = KyberKem::decaps(&secret_key, &ciphertext)?;
// assert_eq!(shared_secret, decaps_secret);
// ```

use std::collections::HashMap;
use std::fmt;
use std::time::{Duration, Instant};
use rand::{RngCore, CryptoRng};
use rand::rngs::OsRng;
use sha3::{Sha3_256, Sha3_512, Digest};
use hmac::{Hmac, Mac};

// Post-quantum cryptography imports
use pqcrypto_traits::kem::{PublicKey as KemPublicKey, SecretKey as KemSecretKey, SharedSecret, Ciphertext};
use pqcrypto_traits::sign::{PublicKey as SignPublicKey, SecretKey as SignSecretKey, DetachedSignature};
use pqcrypto_kyber::{kyber512, kyber768, kyber1024};
use pqcrypto_dilithium::{dilithium2, dilithium3, dilithium5};
use pqcrypto_sphincsplus::{sphincssha2128fsimple, sphincssha2128ssimple, sphincssha2192fsimple, sphincssha2192ssimple, sphincssha2256fsimple, sphincssha2256ssimple};
use pqcrypto_falcon::{falcon512, falcon1024};
use pqcrypto_ntru::{ntruhps2048509, ntruhps2048677, ntruhps4096821, ntruhrss701};

// Additional cryptography for hybrid encryption
use aes_gcm::{Aes256Gcm, Key, Nonce, KeyInit};
use aes_gcm::aead::Aead;

use crate::error::CursedError;

/// Post-Quantum Cryptography specific errors
#[derive(Debug, Clone, PartialEq)]
pub enum PqcError {
    /// Invalid key size or format
    /// Invalid ciphertext or signature
    /// Invalid signature or verification failed
    /// Unsupported parameter set or security level
    /// Random number generation failed
    /// Key generation failed
    /// Encapsulation failed
    /// Decapsulation failed
    /// Signing operation failed
    /// Verification operation failed
    /// Encryption failed
    /// Decryption failed
    /// Parameter validation failed
    /// Internal algorithm error
// impl fmt::Display for PqcError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             PqcError::InvalidKey(msg) => write!(f, "Invalid key: {}", msg),
//             PqcError::InvalidCiphertext(msg) => write!(f, "Invalid ciphertext: {}", msg),
//             PqcError::InvalidSignature(msg) => write!(f, "Invalid signature: {}", msg),
//             PqcError::UnsupportedParameters(msg) => write!(f, "Unsupported parameters: {}", msg),
//             PqcError::RandomGenerationFailed(msg) => write!(f, "Random generation failed: {}", msg),
//             PqcError::KeyGenerationFailed(msg) => write!(f, "Key generation failed: {}", msg),
//             PqcError::EncapsulationFailed(msg) => write!(f, "Encapsulation failed: {}", msg),
//             PqcError::DecapsulationFailed(msg) => write!(f, "Decapsulation failed: {}", msg),
//             PqcError::SigningFailed(msg) => write!(f, "Signing failed: {}", msg),
//             PqcError::VerificationFailed(msg) => write!(f, "Verification failed: {}", msg),
//             PqcError::EncryptionFailed(msg) => write!(f, "Encryption failed: {}", msg),
//             PqcError::DecryptionFailed(msg) => write!(f, "Decryption failed: {}", msg),
//             PqcError::ParameterValidation(msg) => write!(f, "Parameter validation failed: {}", msg),
//             PqcError::InternalError(msg) => write!(f, "Internal error: {}", msg),
//         }
//     }
// }

// impl std::error::CursedError for PqcError {}
// 
// impl From<PqcError> for CursedError {
//     fn from(err: PqcError) -> Self {
//         CursedError::Runtime(format!("PQC error: {}", err))
//     }
// }

/// Result type for PQC operations
pub type PqcResult<T> = std::result::Result<T, PqcError>;

/// Security levels corresponding to classical cryptographic strength
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SecurityLevel {
    /// NIST Level 1 - Equivalent to AES-128
    /// NIST Level 3 - Equivalent to AES-192
    /// NIST Level 5 - Equivalent to AES-256
impl SecurityLevel {
    /// Get the equivalent classical security strength in bits
    pub fn classical_bits(&self) -> u32 {
        match self {
        }
    }

    /// Get a description of the security level
    pub fn description(&self) -> &'static str {
        match self {
        }
    }
/// Algorithm type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlgorithmType {
impl fmt::Display for AlgorithmType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        }
    }
/// Performance metrics for PQC operations
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
        }
    }
/// Quantum resistance assessment
#[derive(Debug, Clone)]
pub struct QuantumResistanceAssessment {
// ============================================================================
// KYBER KEY ENCAPSULATION MECHANISM (KEM)
// ============================================================================

/// Kyber parameter sets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KyberParameterSet {
    /// Kyber-512 (NIST Level 1)
    /// Kyber-768 (NIST Level 3)
    /// Kyber-1024 (NIST Level 5)
impl KyberParameterSet {
    pub fn security_level(&self) -> SecurityLevel {
        match self {
        }
    }

    pub fn public_key_size(&self) -> usize {
        match self {
        }
    }

    pub fn secret_key_size(&self) -> usize {
        match self {
        }
    }

    pub fn ciphertext_size(&self) -> usize {
        match self {
        }
    }

    pub fn shared_secret_size(&self) -> usize {
        32 // All Kyber variants use 32-byte shared secrets
    }
}

/// Kyber public key
#[derive(Debug, Clone)]
pub struct KyberPublicKey {
/// Kyber secret key
#[derive(Debug, Clone)]
pub struct KyberSecretKey {
/// Kyber Key Encapsulation Mechanism implementation
pub struct KyberKem;

impl KyberKem {
    /// Generate a Kyber key pair
    pub fn keygen(security_level: SecurityLevel) -> PqcResult<(KyberPublicKey, KyberSecretKey)> {
        let parameter_set = match security_level {

        Self::keygen_with_params(parameter_set)
    /// Generate a Kyber key pair with specific parameter set
    pub fn keygen_with_params(params: KyberParameterSet) -> PqcResult<(KyberPublicKey, KyberSecretKey)> {
        match params {
            KyberParameterSet::Kyber512 => {
                let (pk, sk) = kyber512::keypair();
                let public_key = KyberPublicKey {
                let secret_key = KyberSecretKey {
                Ok((public_key, secret_key))
            KyberParameterSet::Kyber768 => {
                let (pk, sk) = kyber768::keypair();
                let public_key = KyberPublicKey {
                let secret_key = KyberSecretKey {
                Ok((public_key, secret_key))
            KyberParameterSet::Kyber1024 => {
                let (pk, sk) = kyber1024::keypair();
                let public_key = KyberPublicKey {
                let secret_key = KyberSecretKey {
                Ok((public_key, secret_key))
        }
    }

    /// Encapsulate a shared secret using a public key
    pub fn encaps(public_key: &KyberPublicKey) -> PqcResult<(Vec<u8>, Vec<u8>)> {
        match public_key.parameter_set {
            KyberParameterSet::Kyber512 => {
                let pk = kyber512::PublicKey::from_bytes(&public_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Kyber512 public key".to_string()))?;
                let (ss, ct) = kyber512::encapsulate(&pk);
                Ok((ct.as_bytes().to_vec(), ss.as_bytes().to_vec()))
            KyberParameterSet::Kyber768 => {
                let pk = kyber768::PublicKey::from_bytes(&public_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Kyber768 public key".to_string()))?;
                let (ss, ct) = kyber768::encapsulate(&pk);
                Ok((ct.as_bytes().to_vec(), ss.as_bytes().to_vec()))
            KyberParameterSet::Kyber1024 => {
                let pk = kyber1024::PublicKey::from_bytes(&public_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Kyber1024 public key".to_string()))?;
                let (ss, ct) = kyber1024::encapsulate(&pk);
                Ok((ct.as_bytes().to_vec(), ss.as_bytes().to_vec()))
        }
    }

    /// Decapsulate a shared secret using a secret key and ciphertext
    pub fn decaps(secret_key: &KyberSecretKey, ciphertext: &[u8]) -> PqcResult<Vec<u8>> {
        // Validate ciphertext size
        let expected_size = secret_key.parameter_set.ciphertext_size();
        if ciphertext.len() != expected_size {
            return Err(PqcError::InvalidCiphertext(
                format!("Expected {} bytes, got {}", expected_size, ciphertext.len())
            ));
        match secret_key.parameter_set {
            KyberParameterSet::Kyber512 => {
                let sk = kyber512::SecretKey::from_bytes(&secret_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Kyber512 secret key".to_string()))?;
                let ct = kyber512::Ciphertext::from_bytes(ciphertext)
                    .map_err(|_| PqcError::InvalidCiphertext("Invalid Kyber512 ciphertext".to_string()))?;
                let ss = kyber512::decapsulate(&ct, &sk);
                Ok(ss.as_bytes().to_vec())
            KyberParameterSet::Kyber768 => {
                let sk = kyber768::SecretKey::from_bytes(&secret_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Kyber768 secret key".to_string()))?;
                let ct = kyber768::Ciphertext::from_bytes(ciphertext)
                    .map_err(|_| PqcError::InvalidCiphertext("Invalid Kyber768 ciphertext".to_string()))?;
                let ss = kyber768::decapsulate(&ct, &sk);
                Ok(ss.as_bytes().to_vec())
            KyberParameterSet::Kyber1024 => {
                let sk = kyber1024::SecretKey::from_bytes(&secret_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Kyber1024 secret key".to_string()))?;
                let ct = kyber1024::Ciphertext::from_bytes(ciphertext)
                    .map_err(|_| PqcError::InvalidCiphertext("Invalid Kyber1024 ciphertext".to_string()))?;
                let ss = kyber1024::decapsulate(&ct, &sk);
                Ok(ss.as_bytes().to_vec())
        }
    }


// ============================================================================
// DILITHIUM DIGITAL SIGNATURES
// ============================================================================

/// Dilithium parameter sets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DilithiumParameterSet {
    /// Dilithium2 (NIST Level 2)
    /// Dilithium3 (NIST Level 3)
    /// Dilithium5 (NIST Level 5)
impl DilithiumParameterSet {
    pub fn security_level(&self) -> SecurityLevel {
        match self {
        }
    }

    pub fn public_key_size(&self) -> usize {
        match self {
        }
    }

    pub fn secret_key_size(&self) -> usize {
        match self {
        }
    }

    pub fn signature_size(&self) -> usize {
        match self {
        }
    }
/// Dilithium public key
#[derive(Debug, Clone)]
pub struct DilithiumPublicKey {
/// Dilithium secret key
#[derive(Debug, Clone)]
pub struct DilithiumSecretKey {
/// Dilithium Digital Signature implementation
pub struct DilithiumSignature;

impl DilithiumSignature {
    /// Generate a Dilithium key pair
    pub fn keygen(security_level: SecurityLevel) -> PqcResult<(DilithiumPublicKey, DilithiumSecretKey)> {
        let parameter_set = match security_level {

        Self::keygen_with_params(parameter_set)
    /// Generate a Dilithium key pair with specific parameter set
    pub fn keygen_with_params(params: DilithiumParameterSet) -> PqcResult<(DilithiumPublicKey, DilithiumSecretKey)> {
        match params {
            DilithiumParameterSet::Dilithium2 => {
                let (pk, sk) = dilithium2::keypair();
                let public_key = DilithiumPublicKey {
                let secret_key = DilithiumSecretKey {
                Ok((public_key, secret_key))
            DilithiumParameterSet::Dilithium3 => {
                let (pk, sk) = dilithium3::keypair();
                let public_key = DilithiumPublicKey {
                let secret_key = DilithiumSecretKey {
                Ok((public_key, secret_key))
            DilithiumParameterSet::Dilithium5 => {
                let (pk, sk) = dilithium5::keypair();
                let public_key = DilithiumPublicKey {
                let secret_key = DilithiumSecretKey {
                Ok((public_key, secret_key))
        }
    }

    /// Sign a message using Dilithium
    pub fn sign(secret_key: &DilithiumSecretKey, message: &[u8]) -> PqcResult<Vec<u8>> {
        match secret_key.parameter_set {
            DilithiumParameterSet::Dilithium2 => {
                let sk = dilithium2::SecretKey::from_bytes(&secret_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Dilithium2 secret key".to_string()))?;
                let signature = dilithium2::detached_sign(message, &sk);
                Ok(signature.as_bytes().to_vec())
            DilithiumParameterSet::Dilithium3 => {
                let sk = dilithium3::SecretKey::from_bytes(&secret_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Dilithium3 secret key".to_string()))?;
                let signature = dilithium3::detached_sign(message, &sk);
                Ok(signature.as_bytes().to_vec())
            DilithiumParameterSet::Dilithium5 => {
                let sk = dilithium5::SecretKey::from_bytes(&secret_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Dilithium5 secret key".to_string()))?;
                let signature = dilithium5::detached_sign(message, &sk);
                Ok(signature.as_bytes().to_vec())
        }
    }

    /// Verify a Dilithium signature
    pub fn verify(public_key: &DilithiumPublicKey, message: &[u8], signature: &[u8]) -> PqcResult<bool> {
        // Validate signature size
        let expected_size = public_key.parameter_set.signature_size();
        if signature.len() != expected_size {
            return Err(PqcError::InvalidSignature(
                format!("Expected {} bytes, got {}", expected_size, signature.len())
            ));
        match public_key.parameter_set {
            DilithiumParameterSet::Dilithium2 => {
                let pk = dilithium2::PublicKey::from_bytes(&public_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Dilithium2 public key".to_string()))?;
                let sig = dilithium2::DetachedSignature::from_bytes(signature)
                    .map_err(|_| PqcError::InvalidSignature("Invalid Dilithium2 signature".to_string()))?;
                match dilithium2::verify_detached_signature(message, &sig, &pk) {
                }
            DilithiumParameterSet::Dilithium3 => {
                let pk = dilithium3::PublicKey::from_bytes(&public_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Dilithium3 public key".to_string()))?;
                let sig = dilithium3::DetachedSignature::from_bytes(signature)
                    .map_err(|_| PqcError::InvalidSignature("Invalid Dilithium3 signature".to_string()))?;
                match dilithium3::verify_detached_signature(message, &sig, &pk) {
                }
            DilithiumParameterSet::Dilithium5 => {
                let pk = dilithium5::PublicKey::from_bytes(&public_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Dilithium5 public key".to_string()))?;
                let sig = dilithium5::DetachedSignature::from_bytes(signature)
                    .map_err(|_| PqcError::InvalidSignature("Invalid Dilithium5 signature".to_string()))?;
                match dilithium5::verify_detached_signature(message, &sig, &pk) {
                }
        }
    }


// ============================================================================
// SPHINCS+ HASH-BASED SIGNATURES
// ============================================================================

/// SPHINCS+ parameter sets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SphincsPlusParameterSet {
    /// SPHINCS+-128s (Small signatures, NIST Level 1)
    /// SPHINCS+-192s (Small signatures, NIST Level 3)
    /// SPHINCS+-256s (Small signatures, NIST Level 5)
impl SphincsPlusParameterSet {
    pub fn security_level(&self) -> SecurityLevel {
        match self {
        }
    }

    pub fn public_key_size(&self) -> usize {
        match self {
        }
    }

    pub fn secret_key_size(&self) -> usize {
        match self {
        }
    }

    pub fn signature_size(&self) -> usize {
        match self {
        }
    }
/// SPHINCS+ public key
#[derive(Debug, Clone)]
pub struct SphincsPlusPublicKey {
/// SPHINCS+ secret key
#[derive(Debug, Clone)]
pub struct SphincsPlusSecretKey {
/// SPHINCS+ Hash-based Signature implementation
pub struct SphincsPlusSignature;

impl SphincsPlusSignature {
    /// Generate a SPHINCS+ key pair
    pub fn keygen(security_level: SecurityLevel) -> PqcResult<(SphincsPlusPublicKey, SphincsPlusSecretKey)> {
        let parameter_set = match security_level {

        Self::keygen_with_params(parameter_set)
    /// Generate a SPHINCS+ key pair with specific parameter set
    pub fn keygen_with_params(params: SphincsPlusParameterSet) -> PqcResult<(SphincsPlusPublicKey, SphincsPlusSecretKey)> {
        match params {
            SphincsPlusParameterSet::Sphincs128s => {
                let (pk, sk) = sphincssha2128ssimple::keypair();
                let public_key = SphincsPlusPublicKey {
                let secret_key = SphincsPlusSecretKey {
                Ok((public_key, secret_key))
            SphincsPlusParameterSet::Sphincs192s => {
                let (pk, sk) = sphincssha2192ssimple::keypair();
                let public_key = SphincsPlusPublicKey {
                let secret_key = SphincsPlusSecretKey {
                Ok((public_key, secret_key))
            SphincsPlusParameterSet::Sphincs256s => {
                let (pk, sk) = sphincssha2256ssimple::keypair();
                let public_key = SphincsPlusPublicKey {
                let secret_key = SphincsPlusSecretKey {
                Ok((public_key, secret_key))
        }
    }

    /// Sign a message using SPHINCS+
    pub fn sign(secret_key: &SphincsPlusSecretKey, message: &[u8]) -> PqcResult<Vec<u8>> {
        match secret_key.parameter_set {
            SphincsPlusParameterSet::Sphincs128s => {
                let sk = sphincssha2128ssimple::SecretKey::from_bytes(&secret_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid SPHINCS+128s secret key".to_string()))?;
                let signature = sphincssha2128ssimple::detached_sign(message, &sk);
                Ok(signature.as_bytes().to_vec())
            SphincsPlusParameterSet::Sphincs192s => {
                let sk = sphincssha2192ssimple::SecretKey::from_bytes(&secret_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid SPHINCS+192s secret key".to_string()))?;
                let signature = sphincssha2192ssimple::detached_sign(message, &sk);
                Ok(signature.as_bytes().to_vec())
            SphincsPlusParameterSet::Sphincs256s => {
                let sk = sphincssha2256ssimple::SecretKey::from_bytes(&secret_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid SPHINCS+256s secret key".to_string()))?;
                let signature = sphincssha2256ssimple::detached_sign(message, &sk);
                Ok(signature.as_bytes().to_vec())
        }
    }

    /// Verify a SPHINCS+ signature
    pub fn verify(public_key: &SphincsPlusPublicKey, message: &[u8], signature: &[u8]) -> PqcResult<bool> {
        let expected_size = public_key.parameter_set.signature_size();
        if signature.len() != expected_size {
            return Err(PqcError::InvalidSignature(
                format!("Expected {} bytes, got {}", expected_size, signature.len())
            ));
        match public_key.parameter_set {
            SphincsPlusParameterSet::Sphincs128s => {
                let pk = sphincssha2128ssimple::PublicKey::from_bytes(&public_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid SPHINCS+128s public key".to_string()))?;
                let sig = sphincssha2128ssimple::DetachedSignature::from_bytes(signature)
                    .map_err(|_| PqcError::InvalidSignature("Invalid SPHINCS+128s signature".to_string()))?;
                match sphincssha2128ssimple::verify_detached_signature(message, &sig, &pk) {
                }
            SphincsPlusParameterSet::Sphincs192s => {
                let pk = sphincssha2192ssimple::PublicKey::from_bytes(&public_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid SPHINCS+192s public key".to_string()))?;
                let sig = sphincssha2192ssimple::DetachedSignature::from_bytes(signature)
                    .map_err(|_| PqcError::InvalidSignature("Invalid SPHINCS+192s signature".to_string()))?;
                match sphincssha2192ssimple::verify_detached_signature(message, &sig, &pk) {
                }
            SphincsPlusParameterSet::Sphincs256s => {
                let pk = sphincssha2256ssimple::PublicKey::from_bytes(&public_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid SPHINCS+256s public key".to_string()))?;
                let sig = sphincssha2256ssimple::DetachedSignature::from_bytes(signature)
                    .map_err(|_| PqcError::InvalidSignature("Invalid SPHINCS+256s signature".to_string()))?;
                match sphincssha2256ssimple::verify_detached_signature(message, &sig, &pk) {
                }
        }
    }


// ============================================================================
// FALCON COMPACT SIGNATURES
// ============================================================================

/// Falcon parameter sets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FalconParameterSet {
    /// Falcon-512 (NIST Level 1)
    /// Falcon-1024 (NIST Level 5)
impl FalconParameterSet {
    pub fn security_level(&self) -> SecurityLevel {
        match self {
        }
    }

    pub fn public_key_size(&self) -> usize {
        match self {
        }
    }

    pub fn secret_key_size(&self) -> usize {
        match self {
        }
    }

    pub fn signature_size(&self) -> usize {
        match self {
        }
    }
/// Falcon public key
#[derive(Debug, Clone)]
pub struct FalconPublicKey {
/// Falcon secret key
#[derive(Debug, Clone)]
pub struct FalconSecretKey {
/// Falcon Compact Signature implementation
pub struct FalconSignature;

impl FalconSignature {
    /// Generate a Falcon key pair
    pub fn keygen(security_level: SecurityLevel) -> PqcResult<(FalconPublicKey, FalconSecretKey)> {
        let parameter_set = match security_level {

        Self::keygen_with_params(parameter_set)
    /// Generate a Falcon key pair with specific parameter set
    pub fn keygen_with_params(params: FalconParameterSet) -> PqcResult<(FalconPublicKey, FalconSecretKey)> {
        match params {
            FalconParameterSet::Falcon512 => {
                let (pk, sk) = falcon512::keypair();
                let public_key = FalconPublicKey {
                let secret_key = FalconSecretKey {
                Ok((public_key, secret_key))
            FalconParameterSet::Falcon1024 => {
                let (pk, sk) = falcon1024::keypair();
                let public_key = FalconPublicKey {
                let secret_key = FalconSecretKey {
                Ok((public_key, secret_key))
        }
    }

    /// Sign a message using Falcon
    pub fn sign(secret_key: &FalconSecretKey, message: &[u8]) -> PqcResult<Vec<u8>> {
        match secret_key.parameter_set {
            FalconParameterSet::Falcon512 => {
                let sk = falcon512::SecretKey::from_bytes(&secret_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Falcon512 secret key".to_string()))?;
                let signature = falcon512::detached_sign(message, &sk);
                Ok(signature.as_bytes().to_vec())
            FalconParameterSet::Falcon1024 => {
                let sk = falcon1024::SecretKey::from_bytes(&secret_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Falcon1024 secret key".to_string()))?;
                let signature = falcon1024::detached_sign(message, &sk);
                Ok(signature.as_bytes().to_vec())
        }
    }

    /// Verify a Falcon signature
    pub fn verify(public_key: &FalconPublicKey, message: &[u8], signature: &[u8]) -> PqcResult<bool> {
        let expected_size = public_key.parameter_set.signature_size();
        if signature.len() != expected_size {
            return Err(PqcError::InvalidSignature(
                format!("Expected {} bytes, got {}", expected_size, signature.len())
            ));
        match public_key.parameter_set {
            FalconParameterSet::Falcon512 => {
                let pk = falcon512::PublicKey::from_bytes(&public_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Falcon512 public key".to_string()))?;
                let sig = falcon512::DetachedSignature::from_bytes(signature)
                    .map_err(|_| PqcError::InvalidSignature("Invalid Falcon512 signature".to_string()))?;
                match falcon512::verify_detached_signature(message, &sig, &pk) {
                }
            FalconParameterSet::Falcon1024 => {
                let pk = falcon1024::PublicKey::from_bytes(&public_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Falcon1024 public key".to_string()))?;
                let sig = falcon1024::DetachedSignature::from_bytes(signature)
                    .map_err(|_| PqcError::InvalidSignature("Invalid Falcon1024 signature".to_string()))?;
                match falcon1024::verify_detached_signature(message, &sig, &pk) {
                }
        }
    }


// ============================================================================
// NTRU ENCRYPTION
// ============================================================================

/// NTRU parameter sets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NtruParameterSet {
    /// NTRU-HPS-2048-509 (NIST Level 1)
    /// NTRU-HPS-2048-677 (NIST Level 3)
    /// NTRU-HPS-4096-821 (NIST Level 5)
    /// NTRU-HRSS-701 (NIST Level 1)
impl NtruParameterSet {
    pub fn security_level(&self) -> SecurityLevel {
        match self {
        }
    }

    pub fn public_key_size(&self) -> usize {
        match self {
        }
    }

    pub fn secret_key_size(&self) -> usize {
        match self {
        }
    }

    pub fn ciphertext_size(&self) -> usize {
        match self {
        }
    }
/// NTRU public key
#[derive(Debug, Clone)]
pub struct NtruPublicKey {
/// NTRU secret key
#[derive(Debug, Clone)]
pub struct NtruSecretKey {
/// NTRU Encryption implementation
pub struct NtruEncryption;

impl NtruEncryption {
    /// Generate an NTRU key pair
    pub fn keygen(security_level: SecurityLevel) -> PqcResult<(NtruPublicKey, NtruSecretKey)> {
        let parameter_set = match security_level {

        Self::keygen_with_params(parameter_set)
    /// Generate an NTRU key pair with specific parameter set
    pub fn keygen_with_params(params: NtruParameterSet) -> PqcResult<(NtruPublicKey, NtruSecretKey)> {
        match params {
            NtruParameterSet::NtruHps2048509 => {
                let (pk, sk) = ntruhps2048509::keypair();
                let public_key = NtruPublicKey {
                let secret_key = NtruSecretKey {
                Ok((public_key, secret_key))
            NtruParameterSet::NtruHps2048677 => {
                let (pk, sk) = ntruhps2048677::keypair();
                let public_key = NtruPublicKey {
                let secret_key = NtruSecretKey {
                Ok((public_key, secret_key))
            NtruParameterSet::NtruHps4096821 => {
                let (pk, sk) = ntruhps4096821::keypair();
                let public_key = NtruPublicKey {
                let secret_key = NtruSecretKey {
                Ok((public_key, secret_key))
            NtruParameterSet::NtruHrss701 => {
                let (pk, sk) = ntruhrss701::keypair();
                let public_key = NtruPublicKey {
                let secret_key = NtruSecretKey {
                Ok((public_key, secret_key))
        }
    }

    /// Encapsulate a shared secret using NTRU (KEM operation)
    pub fn encapsulate(public_key: &NtruPublicKey) -> PqcResult<(Vec<u8>, Vec<u8>)> {
        match public_key.parameter_set {
            NtruParameterSet::NtruHps2048509 => {
                let pk = ntruhps2048509::PublicKey::from_bytes(&public_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid NTRU-HPS-2048-509 public key".to_string()))?;
                let (ss, ct) = ntruhps2048509::encapsulate(&pk);
                Ok((ct.as_bytes().to_vec(), ss.as_bytes().to_vec()))
            NtruParameterSet::NtruHps2048677 => {
                let pk = ntruhps2048677::PublicKey::from_bytes(&public_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid NTRU-HPS-2048-677 public key".to_string()))?;
                let (ss, ct) = ntruhps2048677::encapsulate(&pk);
                Ok((ct.as_bytes().to_vec(), ss.as_bytes().to_vec()))
            NtruParameterSet::NtruHps4096821 => {
                let pk = ntruhps4096821::PublicKey::from_bytes(&public_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid NTRU-HPS-4096-821 public key".to_string()))?;
                let (ss, ct) = ntruhps4096821::encapsulate(&pk);
                Ok((ct.as_bytes().to_vec(), ss.as_bytes().to_vec()))
            NtruParameterSet::NtruHrss701 => {
                let pk = ntruhrss701::PublicKey::from_bytes(&public_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid NTRU-HRSS-701 public key".to_string()))?;
                let (ss, ct) = ntruhrss701::encapsulate(&pk);
                Ok((ct.as_bytes().to_vec(), ss.as_bytes().to_vec()))
        }
    }

    /// Decapsulate a shared secret using NTRU (KEM operation)
    pub fn decapsulate(secret_key: &NtruSecretKey, ciphertext: &[u8]) -> PqcResult<Vec<u8>> {
        let expected_size = secret_key.parameter_set.ciphertext_size();
        if ciphertext.len() != expected_size {
            return Err(PqcError::InvalidCiphertext(
                format!("Expected {} bytes, got {}", expected_size, ciphertext.len())
            ));
        match secret_key.parameter_set {
            NtruParameterSet::NtruHps2048509 => {
                let sk = ntruhps2048509::SecretKey::from_bytes(&secret_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid NTRU-HPS-2048-509 secret key".to_string()))?;
                let ct = ntruhps2048509::Ciphertext::from_bytes(ciphertext)
                    .map_err(|_| PqcError::InvalidCiphertext("Invalid NTRU-HPS-2048-509 ciphertext".to_string()))?;
                let ss = ntruhps2048509::decapsulate(&ct, &sk);
                Ok(ss.as_bytes().to_vec())
            NtruParameterSet::NtruHps2048677 => {
                let sk = ntruhps2048677::SecretKey::from_bytes(&secret_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid NTRU-HPS-2048-677 secret key".to_string()))?;
                let ct = ntruhps2048677::Ciphertext::from_bytes(ciphertext)
                    .map_err(|_| PqcError::InvalidCiphertext("Invalid NTRU-HPS-2048-677 ciphertext".to_string()))?;
                let ss = ntruhps2048677::decapsulate(&ct, &sk);
                Ok(ss.as_bytes().to_vec())
            NtruParameterSet::NtruHps4096821 => {
                let sk = ntruhps4096821::SecretKey::from_bytes(&secret_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid NTRU-HPS-4096-821 secret key".to_string()))?;
                let ct = ntruhps4096821::Ciphertext::from_bytes(ciphertext)
                    .map_err(|_| PqcError::InvalidCiphertext("Invalid NTRU-HPS-4096-821 ciphertext".to_string()))?;
                let ss = ntruhps4096821::decapsulate(&ct, &sk);
                Ok(ss.as_bytes().to_vec())
            NtruParameterSet::NtruHrss701 => {
                let sk = ntruhrss701::SecretKey::from_bytes(&secret_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid NTRU-HRSS-701 secret key".to_string()))?;
                let ct = ntruhrss701::Ciphertext::from_bytes(ciphertext)
                    .map_err(|_| PqcError::InvalidCiphertext("Invalid NTRU-HRSS-701 ciphertext".to_string()))?;
                let ss = ntruhrss701::decapsulate(&ct, &sk);
                Ok(ss.as_bytes().to_vec())
        }
    }

    /// Legacy encryption method for backward compatibility
    /// (Now uses KEM with AES-GCM for actual message encryption)
    pub fn encrypt(public_key: &NtruPublicKey, plaintext: &[u8]) -> PqcResult<Vec<u8>> {
        // Use KEM for shared secret, then AES-GCM for actual encryption
        let (ciphertext, shared_secret) = Self::encapsulate(public_key)?;
        
        // Use the shared secret with AES-GCM to encrypt the plaintext
        
        let key = Key::from_slice(&shared_secret[..32]);
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(&shared_secret[..12]); // Use part of shared secret as nonce
        
        let encrypted_data = cipher.encrypt(nonce, plaintext)
            .map_err(|_| PqcError::EncryptionFailed("AES-GCM encryption failed".to_string()))?;
        
        // Prepend the ciphertext with the KEM ciphertext
        let mut result = ciphertext;
        result.extend_from_slice(&encrypted_data);
        Ok(result)
    /// Legacy decryption method for backward compatibility  
    /// (Now uses KEM with AES-GCM for actual message decryption)
    pub fn decrypt(secret_key: &NtruSecretKey, ciphertext: &[u8]) -> PqcResult<Vec<u8>> {
        let kem_ciphertext_size = secret_key.parameter_set.ciphertext_size();
        if ciphertext.len() < kem_ciphertext_size {
            return Err(PqcError::InvalidCiphertext("Ciphertext too short".to_string()));
        // Split KEM ciphertext and encrypted data
        let (kem_ciphertext, encrypted_data) = ciphertext.split_at(kem_ciphertext_size);
        
        // Decapsulate shared secret
        let shared_secret = Self::decapsulate(secret_key, kem_ciphertext)?;
        
        // Use the shared secret with AES-GCM to decrypt the data
        
        let key = Key::from_slice(&shared_secret[..32]);
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(&shared_secret[..12]); // Use part of shared secret as nonce
        
        let decrypted_data = cipher.decrypt(nonce, encrypted_data)
            .map_err(|_| PqcError::DecryptionFailed("AES-GCM decryption failed".to_string()))?;
        
        Ok(decrypted_data)

// ============================================================================
// PERFORMANCE BENCHMARKING
// ============================================================================

/// Performance benchmarking utilities for PQC algorithms
pub struct PqcBenchmark;

impl PqcBenchmark {
    /// Benchmark Kyber KEM operations
    pub fn benchmark_kyber(security_level: SecurityLevel, iterations: usize) -> PqcResult<PerformanceMetrics> {
        let mut total_keygen_time = Duration::from_nanos(0);
        let mut total_encaps_time = Duration::from_nanos(0);
        let mut total_decaps_time = Duration::from_nanos(0);

        let parameter_set = match security_level {

        for _ in 0..iterations {
            // Benchmark key generation
            let keygen_start = Instant::now();
            let (public_key, secret_key) = KyberKem::keygen_with_params(parameter_set)?;
            total_keygen_time += keygen_start.elapsed();

            // Benchmark encapsulation
            let encaps_start = Instant::now();
            let (ciphertext, _shared_secret) = KyberKem::encaps(&public_key)?;
            total_encaps_time += encaps_start.elapsed();

            // Benchmark decapsulation
            let decaps_start = Instant::now();
            let _decaps_secret = KyberKem::decaps(&secret_key, &ciphertext)?;
            total_decaps_time += decaps_start.elapsed();
        let avg_keygen_time = total_keygen_time / iterations as u32;
        let avg_operation_time = (total_encaps_time + total_decaps_time) / (iterations * 2) as u32;
        let operations_per_second = 1.0 / avg_operation_time.as_secs_f64();

        Ok(PerformanceMetrics {
        })
    /// Benchmark Dilithium signature operations
    pub fn benchmark_dilithium(security_level: SecurityLevel, iterations: usize) -> PqcResult<PerformanceMetrics> {
        let mut total_keygen_time = Duration::from_nanos(0);
        let mut total_sign_time = Duration::from_nanos(0);
        let mut total_verify_time = Duration::from_nanos(0);

        let parameter_set = match security_level {

        let test_message = b"This is a test message for benchmarking Dilithium signatures";

        for _ in 0..iterations {
            // Benchmark key generation
            let keygen_start = Instant::now();
            let (public_key, secret_key) = DilithiumSignature::keygen_with_params(parameter_set)?;
            total_keygen_time += keygen_start.elapsed();

            // Benchmark signing
            let sign_start = Instant::now();
            let signature = DilithiumSignature::sign(&secret_key, test_message)?;
            total_sign_time += sign_start.elapsed();

            // Benchmark verification
            let verify_start = Instant::now();
            let _is_valid = DilithiumSignature::verify(&public_key, test_message, &signature)?;
            total_verify_time += verify_start.elapsed();
        let avg_keygen_time = total_keygen_time / iterations as u32;
        let avg_operation_time = (total_sign_time + total_verify_time) / (iterations * 2) as u32;
        let operations_per_second = 1.0 / avg_operation_time.as_secs_f64();

        Ok(PerformanceMetrics {
        })
    /// Benchmark all PQC algorithms
    pub fn benchmark_all(iterations: usize) -> PqcResult<HashMap<String, PerformanceMetrics>> {
        let mut results = HashMap::new();

        // Benchmark Kyber
        for level in [SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
            let kyber_metrics = Self::benchmark_kyber(level, iterations)?;
            results.insert(format!("Kyber-{}", level.classical_bits()), kyber_metrics);
        // Benchmark Dilithium
        for level in [SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
            let dilithium_metrics = Self::benchmark_dilithium(level, iterations)?;
            results.insert(format!("Dilithium-{}", level.classical_bits()), dilithium_metrics);
        Ok(results)
    }
}

// ============================================================================
// QUANTUM RESISTANCE ASSESSMENT
// ============================================================================

/// Quantum resistance assessment utilities
pub struct QuantumResistanceAssessmentUtility;

impl QuantumResistanceAssessmentUtility {
    /// Assess the quantum resistance of all supported algorithms
//     pub fn assess_all_algorithms() -> Vec<crate::stdlib::crypto::pqc::QuantumResistanceAssessment> {
        vec![
        ]
    /// Assess Kyber quantum resistance
//     pub fn assess_kyber() -> crate::stdlib::crypto::pqc::QuantumResistanceAssessment {
//         crate::stdlib::crypto::pqc::QuantumResistanceAssessment {
            key_size_overhead: 3.5, // Compared to classical ECDH
            performance_overhead: 2.1, // Compared to classical ECDH
        }
    }

    /// Assess Dilithium quantum resistance
//     pub fn assess_dilithium() -> crate::stdlib::crypto::pqc::QuantumResistanceAssessment {
//         crate::stdlib::crypto::pqc::QuantumResistanceAssessment {
            key_size_overhead: 15.2, // Compared to classical ECDSA
            performance_overhead: 3.8, // Compared to classical ECDSA
        }
    }

    /// Assess SPHINCS+ quantum resistance
//     pub fn assess_sphincs_plus() -> crate::stdlib::crypto::pqc::QuantumResistanceAssessment {
//         crate::stdlib::crypto::pqc::QuantumResistanceAssessment {
            key_size_overhead: 2.1, // Smaller keys, large signatures
            performance_overhead: 25.7, // Much slower signing
        }
    }

    /// Assess Falcon quantum resistance
//     pub fn assess_falcon() -> crate::stdlib::crypto::pqc::QuantumResistanceAssessment {
//         crate::stdlib::crypto::pqc::QuantumResistanceAssessment {
            key_size_overhead: 8.9, // Compared to classical ECDSA
            performance_overhead: 4.2, // Compared to classical ECDSA
        }
    }

    /// Assess NTRU quantum resistance
//     pub fn assess_ntru() -> crate::stdlib::crypto::pqc::QuantumResistanceAssessment {
//         crate::stdlib::crypto::pqc::QuantumResistanceAssessment {
            key_size_overhead: 4.7, // Compared to classical RSA
            performance_overhead: 1.8, // Compared to classical RSA
        }
    }

    /// Generate a comprehensive quantum readiness report
    pub fn generate_readiness_report() -> String {
        let assessments = Self::assess_all_algorithms();
        let mut report = String::new();
        
        report.push_str("# Post-Quantum Cryptography Readiness Report\n\n");
        report.push_str("This report provides an assessment of post-quantum cryptographic algorithms and their readiness for deployment.\n\n");
        
        for assessment in assessments {
            report.push_str(&format!("## {}\n\n", assessment.algorithm));
            report.push_str(&format!("- **Security Level**: {}\n", assessment.security_level.description()));
            report.push_str(&format!("- **Quantum Secure**: {}\n", if assessment.quantum_secure { "Yes" } else { "No" }));
            report.push_str(&format!("- **Estimated Quantum Break Time**: {}\n", assessment.estimated_quantum_break_time));
            report.push_str(&format!("- **Classical Break Time**: {}\n", assessment.classical_break_time));
            report.push_str(&format!("- **Key Size Overhead**: {:.1}x\n", assessment.key_size_overhead));
            report.push_str(&format!("- **Performance Overhead**: {:.1}x\n", assessment.performance_overhead));
            report.push_str(&format!("- **Standardization Status**: {}\n\n", assessment.standardization_status));
        report.push_str("## Recommendations\n\n");
        report.push_str("1. **Immediate Deployment**: Kyber and Dilithium are ready for production use\n");
        report.push_str("2. **Hybrid Approach**: Consider combining classical and post-quantum algorithms during transition\n");
        report.push_str("3. **Performance Testing**: Benchmark algorithms in your specific environment\n");
        report.push_str("4. **Key Management**: Update key management systems for larger key sizes\n");
        report.push_str("5. **Regular Updates**: Monitor NIST PQC standardization progress\n\n");
        
        report
    }
}

// ============================================================================
// HELPER FUNCTIONS AND UTILITIES
// ============================================================================

/// Validate input parameters for PQC operations
pub fn validate_security_level(level: SecurityLevel) -> PqcResult<()> {
    // All security levels are valid
    Ok(())
/// Get recommended algorithm for specific use case
pub fn get_recommended_algorithm(use_case: &str, security_level: SecurityLevel) -> PqcResult<AlgorithmType> {
    match use_case.to_lowercase().as_str() {
        "signature" | "digital_signature" | "signing" => {
            match security_level {
            }
    }
}

/// Convert bytes to hexadecimal string for display
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>()
/// Convert hexadecimal string to bytes
pub fn hex_to_bytes(hex: &str) -> PqcResult<Vec<u8>> {
    if hex.len() % 2 != 0 {
        return Err(PqcError::ParameterValidation("Hex string must have even length".to_string()));
    let mut bytes = Vec::with_capacity(hex.len() / 2);
    for chunk in hex.as_bytes().chunks(2) {
        let hex_byte = std::str::from_utf8(chunk)
            .map_err(|_| PqcError::ParameterValidation("Invalid hex character".to_string()))?;
        let byte = u8::from_str_radix(hex_byte, 16)
            .map_err(|_| PqcError::ParameterValidation("Invalid hex digit".to_string()))?;
        bytes.push(byte);
    Ok(bytes)
