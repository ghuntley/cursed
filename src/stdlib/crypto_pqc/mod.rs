//! Comprehensive Post-Quantum Cryptography Module for CURSED
//! 
//! This module provides production-ready implementations of post-quantum cryptographic algorithms
//! that are believed to be secure against attacks by quantum computers.
//! 
//! # Algorithm Families Supported
//! 
//! - **Lattice-based**: Kyber, Dilithium, NTRU, FrodoKEM
//! - **Hash-based**: SPHINCS+, LMS, XMSS
//! - **Multivariate**: Rainbow, GeMSS  
//! - **Code-based**: Classic McEliece, BIKE, HQC
//! - **Isogeny-based**: SIKE/SIDH (deprecated but included for research)
//! 
//! # Features
//! 
//! - NIST PQC standardization compliance
//! - Hybrid classical+PQC protocols for migration
//! - Performance benchmarking and algorithm selection
//! - Comprehensive key management with multiple formats
//! - Integration with existing PKI infrastructure
//! 
//! # Security Levels
//! 
//! All algorithms provide multiple security levels:
//! - Level 1: Equivalent to AES-128 (128-bit classical security)
//! - Level 3: Equivalent to AES-192 (192-bit classical security)
//! - Level 5: Equivalent to AES-256 (256-bit classical security)

pub mod algorithms;
pub mod key_management;
pub mod protocols;
pub mod benchmarks;
pub mod benchmarks_real;
pub mod hybrid;
pub mod analysis;
pub mod formats;
pub mod agility;

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
use crate::error::CursedError;

/// Post-Quantum Cryptography specific errors
#[derive(Debug, Clone, PartialEq)]
pub enum PqcError {
    /// Invalid key size or format
    InvalidKey(String),
    /// Invalid ciphertext or signature
    InvalidCiphertext(String),
    /// Invalid signature or verification failed
    InvalidSignature(String),
    /// Unsupported parameter set or security level
    UnsupportedParameters(String),
    /// Random number generation failed
    RandomGenerationFailed(String),
    /// Key generation failed
    KeyGenerationFailed(String),
    /// Encapsulation failed
    EncapsulationFailed(String),
    /// Decapsulation failed
    DecapsulationFailed(String),
    /// Signing operation failed
    SigningFailed(String),
    /// Verification operation failed
    VerificationFailed(String),
    /// Encryption failed
    EncryptionFailed(String),
    /// Decryption failed
    DecryptionFailed(String),
    /// Parameter validation failed
    ParameterValidation(String),
    /// Internal algorithm error
    InternalError(String),
    /// Algorithm not available
    AlgorithmNotAvailable(String),
    /// Hybrid protocol error
    HybridError(String),
    /// Format conversion error
    FormatError(String),
    /// Benchmark error
    BenchmarkError(String),
    /// Analysis error
    AnalysisError(String),
}

impl fmt::Display for PqcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PqcError::InvalidKey(msg) => write!(f, "Invalid key: {}", msg),
            PqcError::InvalidCiphertext(msg) => write!(f, "Invalid ciphertext: {}", msg),
            PqcError::InvalidSignature(msg) => write!(f, "Invalid signature: {}", msg),
            PqcError::UnsupportedParameters(msg) => write!(f, "Unsupported parameters: {}", msg),
            PqcError::RandomGenerationFailed(msg) => write!(f, "Random generation failed: {}", msg),
            PqcError::KeyGenerationFailed(msg) => write!(f, "Key generation failed: {}", msg),
            PqcError::EncapsulationFailed(msg) => write!(f, "Encapsulation failed: {}", msg),
            PqcError::DecapsulationFailed(msg) => write!(f, "Decapsulation failed: {}", msg),
            PqcError::SigningFailed(msg) => write!(f, "Signing failed: {}", msg),
            PqcError::VerificationFailed(msg) => write!(f, "Verification failed: {}", msg),
            PqcError::EncryptionFailed(msg) => write!(f, "Encryption failed: {}", msg),
            PqcError::DecryptionFailed(msg) => write!(f, "Decryption failed: {}", msg),
            PqcError::ParameterValidation(msg) => write!(f, "Parameter validation failed: {}", msg),
            PqcError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            PqcError::AlgorithmNotAvailable(msg) => write!(f, "Algorithm not available: {}", msg),
            PqcError::HybridError(msg) => write!(f, "Hybrid protocol error: {}", msg),
            PqcError::FormatError(msg) => write!(f, "Format error: {}", msg),
            PqcError::BenchmarkError(msg) => write!(f, "Benchmark error: {}", msg),
            PqcError::AnalysisError(msg) => write!(f, "Analysis error: {}", msg),
        }
    }
}

impl std::error::Error for PqcError {}

impl From<PqcError> for CursedError {
    fn from(err: PqcError) -> Self {
        CursedError::Runtime(format!("PQC error: {}", err))
    }
}

/// Result type for PQC operations
pub type PqcResult<(), Error>;

/// Security levels corresponding to classical cryptographic strength
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SecurityLevel {
    /// NIST Level 1 - Equivalent to AES-128
    Level1,
    /// NIST Level 3 - Equivalent to AES-192  
    Level3,
    /// NIST Level 5 - Equivalent to AES-256
    Level5,
}

impl SecurityLevel {
    /// Get the equivalent classical security strength in bits
    pub fn classical_bits(&self) -> u32 {
        match self {
            SecurityLevel::Level1 => 128,
            SecurityLevel::Level3 => 192,
            SecurityLevel::Level5 => 256,
        }
    }

    /// Get a description of the security level
    pub fn description(&self) -> &'static str {
        match self {
            SecurityLevel::Level1 => "NIST Level 1 (AES-128 equivalent)",
            SecurityLevel::Level3 => "NIST Level 3 (AES-192 equivalent)", 
            SecurityLevel::Level5 => "NIST Level 5 (AES-256 equivalent)",
        }
    }
}

/// Algorithm type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlgorithmType {
    // Lattice-based
    Kyber,
    Dilithium,
    Ntru,
    FrodoKem,
    
    // Hash-based
    Sphincs,
    Lms,
    Xmss,
    
    // Multivariate
    Rainbow,
    GeMSS,
    
    // Code-based
    ClassicMcEliece,
    Bike,
    Hqc,
    
    // Isogeny-based (deprecated/research)
    Sike,
}

impl fmt::Display for AlgorithmType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlgorithmType::Kyber => write!(f, "Kyber"),
            AlgorithmType::Dilithium => write!(f, "Dilithium"),
            AlgorithmType::Ntru => write!(f, "NTRU"),
            AlgorithmType::FrodoKem => write!(f, "FrodoKEM"),
            AlgorithmType::Sphincs => write!(f, "SPHINCS+"),
            AlgorithmType::Lms => write!(f, "LMS"),
            AlgorithmType::Xmss => write!(f, "XMSS"),
            AlgorithmType::Rainbow => write!(f, "Rainbow"),
            AlgorithmType::GeMSS => write!(f, "GeMSS"),
            AlgorithmType::ClassicMcEliece => write!(f, "Classic McEliece"),
            AlgorithmType::Bike => write!(f, "BIKE"),
            AlgorithmType::Hqc => write!(f, "HQC"),
            AlgorithmType::Sike => write!(f, "SIKE"),
        }
    }
}

/// Algorithm family classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlgorithmFamily {
    LatticeBased,
    HashBased,
    Multivariate,
    CodeBased,
    IsogenyBased,
}

impl AlgorithmFamily {
    /// Get the family for a given algorithm
    pub fn from_algorithm(alg: AlgorithmType) -> Self {
        match alg {
            AlgorithmType::Kyber | AlgorithmType::Dilithium | 
            AlgorithmType::Ntru | AlgorithmType::FrodoKem => AlgorithmFamily::LatticeBased,
            
            AlgorithmType::Sphincs | AlgorithmType::Lms | 
            AlgorithmType::Xmss => AlgorithmFamily::HashBased,
            
            AlgorithmType::Rainbow | AlgorithmType::GeMSS => AlgorithmFamily::Multivariate,
            
            AlgorithmType::ClassicMcEliece | AlgorithmType::Bike | 
            AlgorithmType::Hqc => AlgorithmFamily::CodeBased,
            
            AlgorithmType::Sike => AlgorithmFamily::IsogenyBased,
        }
    }

    /// Get a description of the algorithm family
    pub fn description(&self) -> &'static str {
        match self {
            AlgorithmFamily::LatticeBased => "Lattice-based algorithms (NTRU, Module-LWE, etc.)",
            AlgorithmFamily::HashBased => "Hash-based signatures (stateless and stateful)",
            AlgorithmFamily::Multivariate => "Multivariate polynomial equations",
            AlgorithmFamily::CodeBased => "Error-correcting codes",
            AlgorithmFamily::IsogenyBased => "Isogeny-based (deprecated/research only)",
        }
    }

    /// Get the quantum resistance confidence level
    pub fn quantum_confidence(&self) -> &'static str {
        match self {
            AlgorithmFamily::LatticeBased => "High confidence - well-studied",
            AlgorithmFamily::HashBased => "Very high confidence - provable security",
            AlgorithmFamily::Multivariate => "Medium confidence - ongoing research",
            AlgorithmFamily::CodeBased => "High confidence - well-established",
            AlgorithmFamily::IsogenyBased => "Broken - not quantum secure",
        }
    }
}

/// Standardization status of algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandardizationStatus {
    /// NIST standardized (final)
    NistStandardized,
    /// NIST finalist (under consideration)
    NistFinalist,
    /// NIST alternate candidate
    NistAlternate,
    /// Research/experimental only
    Research,
    /// Deprecated/broken
    Deprecated,
}

impl StandardizationStatus {
    /// Get standardization status for an algorithm
    pub fn for_algorithm(alg: AlgorithmType) -> Self {
        match alg {
            AlgorithmType::Kyber | AlgorithmType::Dilithium | 
            AlgorithmType::Sphincs => StandardizationStatus::NistStandardized,
            
            AlgorithmType::Ntru => StandardizationStatus::NistFinalist,
            
            AlgorithmType::ClassicMcEliece => StandardizationStatus::NistAlternate,
            
            AlgorithmType::FrodoKem | AlgorithmType::Bike | 
            AlgorithmType::Hqc | AlgorithmType::Rainbow | 
            AlgorithmType::GeMSS | AlgorithmType::Lms | 
            AlgorithmType::Xmss => StandardizationStatus::Research,
            
            AlgorithmType::Sike => StandardizationStatus::Deprecated,
        }
    }

    /// Get a description of the standardization status
    pub fn description(&self) -> &'static str {
        match self {
            StandardizationStatus::NistStandardized => "NIST standardized - production ready",
            StandardizationStatus::NistFinalist => "NIST finalist - nearly production ready",
            StandardizationStatus::NistAlternate => "NIST alternate - backup option",
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
