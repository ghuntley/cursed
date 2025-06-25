// Comprehensive Post-Quantum Cryptography Module for CURSED
// 
// This module provides production-ready implementations of post-quantum cryptographic algorithms
// that are believed to be secure against attacks by quantum computers.
// 
// # Algorithm Families Supported
// 
// - **Lattice-based**: Kyber, Dilithium, NTRU, FrodoKEM
// - **Hash-based**: SPHINCS+, LMS, XMSS
// - **Multivariate**: Rainbow, GeMSS  
// - **Code-based**: Classic McEliece, BIKE, HQC
// - **Isogeny-based**: SIKE/SIDH (deprecated but included for research)
// 
// # Features
// 
// - NIST PQC standardization compliance
// - Hybrid classical+PQC protocols for migration
// - Performance benchmarking and algorithm selection
// - Comprehensive key management with multiple formats
// - Integration with existing PKI infrastructure
// 
// # Security Levels
// 
// All algorithms provide multiple security levels:
// - Level 1: Equivalent to AES-128 (128-bit classical security)
// - Level 3: Equivalent to AES-192 (192-bit classical security)
// - Level 5: Equivalent to AES-256 (256-bit classical security)

pub mod algorithms;
pub mod key_management;
pub mod protocols;
pub mod benchmarks;
pub mod benchmarks_real;
pub mod hybrid;
pub mod analysis;
pub mod formats;
pub mod agility;
use crate::error::CursedError;

// Re-export real implementations as primary API
pub use algorithms::kyber_real::*;
pub use algorithms::dilithium_real::*;
pub use algorithms::ntru_real::*;
pub use algorithms::frodo_real::*;
pub use algorithms::lms_real::*;
pub use algorithms::xmss_real::*;
pub use algorithms::falcon_real::*;
pub use algorithms::sphincs_real::*;
pub use algorithms::mceliece_real::*;

// Re-export placeholder implementations for compatibility
pub use algorithms::kyber::*;
pub use algorithms::dilithium::*;
pub use algorithms::ntru::*;
pub use algorithms::frodo::*;
pub use algorithms::sphincs::*;
pub use algorithms::lms::*;
pub use algorithms::xmss::*;
pub use algorithms::rainbow::*;
pub use algorithms::gemss::*;
pub use algorithms::mceliece::*;
pub use algorithms::bike::*;
pub use algorithms::hqc::*;
pub use algorithms::sike::*;

pub use key_management::*;
pub use protocols::*;
pub use benchmarks::*;
pub use benchmarks_real::*;
pub use hybrid::*;
pub use analysis::*;
pub use formats::*;
pub use agility::*;

use std::fmt;

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
    /// Algorithm not available
    /// Hybrid protocol error
    /// Format conversion error
    /// Benchmark error
    /// Analysis error
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
//             PqcError::AlgorithmNotAvailable(msg) => write!(f, "Algorithm not available: {}", msg),
//             PqcError::HybridError(msg) => write!(f, "Hybrid protocol error: {}", msg),
//             PqcError::FormatError(msg) => write!(f, "Format error: {}", msg),
//             PqcError::BenchmarkError(msg) => write!(f, "Benchmark error: {}", msg),
//             PqcError::AnalysisError(msg) => write!(f, "Analysis error: {}", msg),
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
    // Lattice-based
    
    // Hash-based
    
    // Multivariate
    
    // Code-based
    
    // Isogeny-based (deprecated/research)
impl fmt::Display for AlgorithmType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        }
    }
/// Algorithm family classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlgorithmFamily {
impl AlgorithmFamily {
    /// Get the family for a given algorithm
    pub fn from_algorithm(alg: AlgorithmType) -> Self {
        match alg {
            AlgorithmType::Kyber | AlgorithmType::Dilithium | 
            
            AlgorithmType::Sphincs | AlgorithmType::Lms | 
            
            
            AlgorithmType::ClassicMcEliece | AlgorithmType::Bike | 
            
        }
    }

    /// Get a description of the algorithm family
    pub fn description(&self) -> &'static str {
        match self {
            AlgorithmFamily::IsogenyBased => "Isogeny-based (deprecated/research only)",
        }
    }

    /// Get the quantum resistance confidence level
    pub fn quantum_confidence(&self) -> &'static str {
        match self {
        }
    }
/// Standardization status of algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandardizationStatus {
    /// NIST standardized (final)
    /// NIST finalist (under consideration)
    /// NIST alternate candidate
    /// Research/experimental only
    /// Deprecated/broken
impl StandardizationStatus {
    /// Get standardization status for an algorithm
    pub fn for_algorithm(alg: AlgorithmType) -> Self {
        match alg {
            AlgorithmType::Kyber | AlgorithmType::Dilithium | 
            
            
            
            AlgorithmType::FrodoKem | AlgorithmType::Bike | 
            AlgorithmType::Hqc | AlgorithmType::Rainbow | 
            AlgorithmType::GeMSS | AlgorithmType::Lms | 
            
        }
    }

    /// Get a description of the standardization status
    pub fn description(&self) -> &'static str {
        match self {
            StandardizationStatus::Research => "Research/experimental - not for production",
            StandardizationStatus::Deprecated => "Deprecated/broken - do not use",
        }
    }

    /// Whether the algorithm is suitable for production use
    pub fn is_production_ready(&self) -> bool {
        matches!(self, StandardizationStatus::NistStandardized | 
                       StandardizationStatus::NistFinalist)
    }
}
