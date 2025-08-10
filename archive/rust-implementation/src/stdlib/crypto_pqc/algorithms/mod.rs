// Post-Quantum Cryptographic Algorithm Implementations
// 
// This module contains implementations of various post-quantum cryptographic algorithms
// organized by cryptographic family.

// Lattice-based algorithms
pub mod kyber;
pub mod dilithium;
pub mod ntru;
pub mod frodo;

// Real implementations (production-ready)
pub mod kyber_real;
pub mod dilithium_real;
pub mod ntru_real;
pub mod frodo_real;
pub mod lms_real;
pub mod xmss_real;
pub mod falcon_real;
pub mod sphincs_real;
pub mod mceliece_real;

// Hash-based signatures
pub mod sphincs;
pub mod lms;
pub mod xmss;

// Multivariate algorithms
pub mod rainbow;
pub mod gemss;

// Code-based algorithms
pub mod mceliece;
pub mod bike;
pub mod hqc;

// Isogeny-based algorithms (research/deprecated)
pub mod sike;

// use crate::stdlib::crypto_pqc::{PqcResult, SecurityLevel, AlgorithmType};

/// Common trait for Key Encapsulation Mechanisms (KEMs)
pub trait KeyEncapsulation {
    type PublicKey;
    type SecretKey;
    type Ciphertext;
    type SharedSecret;

    /// Generate a new key pair
    fn keygen(security_level: SecurityLevel) -> PqcResult<(Self::PublicKey, Self::SecretKey)>;

    /// Encapsulate a shared secret using the public key
    fn encaps(public_key: &Self::PublicKey) -> PqcResult<(Self::Ciphertext, Self::SharedSecret)>;

    /// Decapsulate the shared secret using the secret key and ciphertext
    fn decaps(secret_key: &Self::SecretKey, ciphertext: &Self::Ciphertext) -> PqcResult<Self::SharedSecret>;

    /// Get the algorithm type
    fn algorithm_type() -> AlgorithmType;
/// Common trait for Digital Signature schemes
pub trait DigitalSignature {
    type PublicKey;
    type SecretKey;
    type Signature;

    /// Generate a new key pair
    fn keygen(security_level: SecurityLevel) -> PqcResult<(Self::PublicKey, Self::SecretKey)>;

    /// Sign a message with the secret key
    fn sign(secret_key: &Self::SecretKey, message: &[u8]) -> PqcResult<Self::Signature>;

    /// Verify a signature with the public key
    fn verify(public_key: &Self::PublicKey, message: &[u8], signature: &Self::Signature) -> PqcResult<bool>;

    /// Get the algorithm type
    fn algorithm_type() -> AlgorithmType;
/// Common trait for Public Key Encryption schemes
pub trait PublicKeyEncryption {
    type PublicKey;
    type SecretKey;
    type Ciphertext;
    type Plaintext;

    /// Generate a new key pair
    fn keygen(security_level: SecurityLevel) -> PqcResult<(Self::PublicKey, Self::SecretKey)>;

    /// Encrypt a plaintext with the public key
    fn encrypt(public_key: &Self::PublicKey, plaintext: &Self::Plaintext) -> PqcResult<Self::Ciphertext>;

    /// Decrypt a ciphertext with the secret key
    fn decrypt(secret_key: &Self::SecretKey, ciphertext: &Self::Ciphertext) -> PqcResult<Self::Plaintext>;

    /// Get the algorithm type
    fn algorithm_type() -> AlgorithmType;
/// Parameter set information for algorithms
pub trait ParameterSet {
    /// Get the security level
    fn security_level(&self) -> SecurityLevel;

    /// Get the public key size in bytes
    fn public_key_size(&self) -> usize;

    /// Get the secret key size in bytes
    fn secret_key_size(&self) -> usize;

    /// Get additional size information (ciphertext, signature, etc.)
    fn additional_sizes(&self) -> Vec<(&'static str, usize)>;
/// Performance characteristics for algorithms
#[derive(Debug, Clone)]
pub struct AlgorithmPerformance {
/// Key size information
#[derive(Debug, Clone)]
pub struct KeySizes {
impl AlgorithmPerformance {
    pub fn new() -> Self {
        Self {
            key_sizes: KeySizes {
        }
    }
}
