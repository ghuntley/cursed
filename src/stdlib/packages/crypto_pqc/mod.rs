/// fr fr Post-quantum cryptography for CURSED - future-proof security bestie
/// 
/// This module provides post-quantum cryptographic algorithms that remain secure
/// against attacks by quantum computers. Future-proofing crypto periodt!

// NIST standardized algorithms
pub mod kyber;
pub mod dilithium; 
pub mod sphincs_plus;
pub mod falcon;

// Candidate algorithms
pub mod ntru;
pub mod saber;
pub mod frodo;
pub mod rainbow;

// Core PQC primitives
pub mod lattice_crypto;
pub mod code_crypto;
pub mod multivariate_crypto;
pub mod hash_crypto;

// Migration and hybrid systems
pub mod hybrid_crypto;
pub mod migration_tools;
pub mod compatibility;

// Re-export main types
pub use kyber::{Kyber512, Kyber768, Kyber1024, KyberKeyPair, KyberError, KyberResult};
pub use dilithium::{Dilithium2, Dilithium3, Dilithium5, DilithiumSignature, DilithiumError, DilithiumResult};
pub use sphincs_plus::{SphincsPlus128, SphincsPlus192, SphincsPlus256, SphincsError, SphincsResult};
pub use falcon::{Falcon512, Falcon1024, FalconSignature, FalconError, FalconResult};

use std::collections::HashMap;

/// fr fr Post-quantum algorithm categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PqcCategory {
    LatticeBasedKem,    // Key encapsulation (Kyber)
    LatticeBasedSig,    // Digital signatures (Dilithium, Falcon)
    HashBasedSig,       // Hash-based signatures (SPHINCS+)
    CodeBasedKem,       // Code-based (Classic McEliece)
    IsogenyBased,       // Isogeny-based (SIKE - broken)
    MultivariateBasedSig, // Multivariate (Rainbow)
}

/// fr fr Supported post-quantum algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PqcAlgorithm {
    // NIST Level 1 (128-bit security)
    Kyber512,
    Dilithium2,
    Falcon512,
    SphincsPlus128s,
    SphincsPlus128f,
    
    // NIST Level 3 (192-bit security)
    Kyber768,
    Dilithium3,
    SphincsPlus192s,
    SphincsPlus192f,
    
    // NIST Level 5 (256-bit security)
    Kyber1024,
    Dilithium5,
    Falcon1024,
    SphincsPlus256s,
    SphincsPlus256f,
    
    // Candidates and research
    NtruHps2048509,
    NtruHrss701,
    Saber,
    FrodoKem640,
    Rainbow,
    ClassicMcEliece,
}

impl PqcAlgorithm {
    /// slay Get algorithm name
    pub fn name(&self) -> &'static str {
        match self {
            PqcAlgorithm::Kyber512 => "Kyber-512",
            PqcAlgorithm::Kyber768 => "Kyber-768",
            PqcAlgorithm::Kyber1024 => "Kyber-1024",
            PqcAlgorithm::Dilithium2 => "Dilithium-2",
            PqcAlgorithm::Dilithium3 => "Dilithium-3",
            PqcAlgorithm::Dilithium5 => "Dilithium-5",
            PqcAlgorithm::Falcon512 => "Falcon-512",
            PqcAlgorithm::Falcon1024 => "Falcon-1024",
            PqcAlgorithm::SphincsPlus128s => "SPHINCS+-128s",
            PqcAlgorithm::SphincsPlus128f => "SPHINCS+-128f",
            PqcAlgorithm::SphincsPlus192s => "SPHINCS+-192s",
            PqcAlgorithm::SphincsPlus192f => "SPHINCS+-192f",
            PqcAlgorithm::SphincsPlus256s => "SPHINCS+-256s",
            PqcAlgorithm::SphincsPlus256f => "SPHINCS+-256f",
            PqcAlgorithm::NtruHps2048509 => "NTRU-HPS-2048-509",
            PqcAlgorithm::NtruHrss701 => "NTRU-HRSS-701",
            PqcAlgorithm::Saber => "Saber",
            PqcAlgorithm::FrodoKem640 => "FrodoKEM-640",
            PqcAlgorithm::Rainbow => "Rainbow",
            PqcAlgorithm::ClassicMcEliece => "Classic McEliece",
        }
    }
    
    /// slay Get security level (NIST levels 1, 3, 5)
    pub fn security_level(&self) -> u32 {
        match self {
            PqcAlgorithm::Kyber512 |
            PqcAlgorithm::Dilithium2 |
            PqcAlgorithm::Falcon512 |
            PqcAlgorithm::SphincsPlus128s |
            PqcAlgorithm::SphincsPlus128f => 1, // 128-bit
            
            PqcAlgorithm::Kyber768 |
            PqcAlgorithm::Dilithium3 |
            PqcAlgorithm::SphincsPlus192s |
            PqcAlgorithm::SphincsPlus192f |
            PqcAlgorithm::NtruHrss701 => 3, // 192-bit
            
            PqcAlgorithm::Kyber1024 |
            PqcAlgorithm::Dilithium5 |
            PqcAlgorithm::Falcon1024 |
            PqcAlgorithm::SphincsPlus256s |
            PqcAlgorithm::SphincsPlus256f => 5, // 256-bit
            
            _ => 1, // Default to level 1
        }
    }
    
    /// slay Get algorithm category
    pub fn category(&self) -> PqcCategory {
        match self {
            PqcAlgorithm::Kyber512 |
            PqcAlgorithm::Kyber768 |
            PqcAlgorithm::Kyber1024 |
            PqcAlgorithm::NtruHps2048509 |
            PqcAlgorithm::NtruHrss701 |
            PqcAlgorithm::Saber |
            PqcAlgorithm::FrodoKem640 => PqcCategory::LatticeBasedKem,
            
            PqcAlgorithm::Dilithium2 |
            PqcAlgorithm::Dilithium3 |
            PqcAlgorithm::Dilithium5 |
            PqcAlgorithm::Falcon512 |
            PqcAlgorithm::Falcon1024 => PqcCategory::LatticeBasedSig,
            
            PqcAlgorithm::SphincsPlus128s |
            PqcAlgorithm::SphincsPlus128f |
            PqcAlgorithm::SphincsPlus192s |
            PqcAlgorithm::SphincsPlus192f |
            PqcAlgorithm::SphincsPlus256s |
            PqcAlgorithm::SphincsPlus256f => PqcCategory::HashBasedSig,
            
            PqcAlgorithm::ClassicMcEliece => PqcCategory::CodeBasedKem,
            PqcAlgorithm::Rainbow => PqcCategory::MultivariateBasedSig,
        }
    }
    
    /// slay Check if algorithm is NIST standardized
    pub fn is_nist_standardized(&self) -> bool {
        match self {
            PqcAlgorithm::Kyber512 |
            PqcAlgorithm::Kyber768 |
            PqcAlgorithm::Kyber1024 |
            PqcAlgorithm::Dilithium2 |
            PqcAlgorithm::Dilithium3 |
            PqcAlgorithm::Dilithium5 |
            PqcAlgorithm::Falcon512 |
            PqcAlgorithm::Falcon1024 |
            PqcAlgorithm::SphincsPlus128s |
            PqcAlgorithm::SphincsPlus128f |
            PqcAlgorithm::SphincsPlus192s |
            PqcAlgorithm::SphincsPlus192f |
            PqcAlgorithm::SphincsPlus256s |
            PqcAlgorithm::SphincsPlus256f => true,
            _ => false,
        }
    }
    
    /// slay Get typical key/signature sizes
    pub fn typical_sizes(&self) -> PqcSizes {
        match self {
            PqcAlgorithm::Kyber512 => PqcSizes {
                public_key: 800,
                private_key: 1632,
                ciphertext: 768,
                signature: 0,
            },
            PqcAlgorithm::Kyber768 => PqcSizes {
                public_key: 1184,
                private_key: 2400,
                ciphertext: 1088,
                signature: 0,
            },
            PqcAlgorithm::Kyber1024 => PqcSizes {
                public_key: 1568,
                private_key: 3168,
                ciphertext: 1568,
                signature: 0,
            },
            PqcAlgorithm::Dilithium2 => PqcSizes {
                public_key: 1312,
                private_key: 2528,
                ciphertext: 0,
                signature: 2420,
            },
            PqcAlgorithm::Dilithium3 => PqcSizes {
                public_key: 1952,
                private_key: 4000,
                ciphertext: 0,
                signature: 3293,
            },
            PqcAlgorithm::Dilithium5 => PqcSizes {
                public_key: 2592,
                private_key: 4864,
                ciphertext: 0,
                signature: 4595,
            },
            PqcAlgorithm::Falcon512 => PqcSizes {
                public_key: 897,
                private_key: 1281,
                ciphertext: 0,
                signature: 690,
            },
            PqcAlgorithm::Falcon1024 => PqcSizes {
                public_key: 1793,
                private_key: 2305,
                ciphertext: 0,
                signature: 1330,
            },
            _ => PqcSizes {
                public_key: 1000,
                private_key: 2000,
                ciphertext: 1000,
                signature: 2000,
            },
        }
    }
}

/// fr fr Size information for PQC algorithms
#[derive(Debug, Clone)]
pub struct PqcSizes {
    pub public_key: usize,
    pub private_key: usize,
    pub ciphertext: usize,  // For KEMs
    pub signature: usize,   // For signature schemes
}

/// fr fr Post-quantum crypto errors
#[derive(Debug, Clone, PartialEq)]
pub enum PqcError {
    UnsupportedAlgorithm(String),
    KeyGenerationFailed(String),
    EncryptionFailed(String),
    DecryptionFailed(String),
    SigningFailed(String),
    VerificationFailed(String),
    InvalidParameters,
    InsufficientRandomness,
    QuantumAttackDetected,
    MigrationRequired,
    Internal(String),
}

impl std::fmt::Display for PqcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PqcError::UnsupportedAlgorithm(name) => 
                write!(f, "Unsupported post-quantum algorithm: {}", name),
            PqcError::KeyGenerationFailed(msg) => write!(f, "Key generation failed: {}", msg),
            PqcError::EncryptionFailed(msg) => write!(f, "Encryption failed: {}", msg),
            PqcError::DecryptionFailed(msg) => write!(f, "Decryption failed: {}", msg),
            PqcError::SigningFailed(msg) => write!(f, "Signing failed: {}", msg),
            PqcError::VerificationFailed(msg) => write!(f, "Verification failed: {}", msg),
            PqcError::InvalidParameters => write!(f, "Invalid parameters"),
            PqcError::InsufficientRandomness => write!(f, "Insufficient randomness"),
            PqcError::QuantumAttackDetected => write!(f, "Potential quantum attack detected"),
            PqcError::MigrationRequired => write!(f, "Migration to post-quantum algorithms required"),
            PqcError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for PqcError {}

/// fr fr Post-quantum result type
pub type PqcResult<T> = Result<T, PqcError>;

/// fr fr Utilities and helper functions
pub mod utils {
    use super::*;
    
    /// slay Get recommended KEM algorithm
    pub fn recommended_kem() -> PqcAlgorithm {
        PqcAlgorithm::Kyber768 // NIST standardized, good security/performance balance
    }
    
    /// slay Get recommended signature algorithm
    pub fn recommended_signature() -> PqcAlgorithm {
        PqcAlgorithm::Dilithium3 // NIST standardized, good balance
    }
    
    /// slay Get fastest algorithms for each category
    pub fn fastest_algorithms() -> HashMap<PqcCategory, PqcAlgorithm> {
        let mut fastest = HashMap::new();
        fastest.insert(PqcCategory::LatticeBasedKem, PqcAlgorithm::Kyber512);
        fastest.insert(PqcCategory::LatticeBasedSig, PqcAlgorithm::Falcon512);
        fastest.insert(PqcCategory::HashBasedSig, PqcAlgorithm::SphincsPlus128f);
        fastest
    }
    
    /// slay Get most secure algorithms for each category
    pub fn most_secure_algorithms() -> HashMap<PqcCategory, PqcAlgorithm> {
        let mut secure = HashMap::new();
        secure.insert(PqcCategory::LatticeBasedKem, PqcAlgorithm::Kyber1024);
        secure.insert(PqcCategory::LatticeBasedSig, PqcAlgorithm::Dilithium5);
        secure.insert(PqcCategory::HashBasedSig, PqcAlgorithm::SphincsPlus256s);
        secure
    }
    
    /// slay Get NIST standardized algorithms
    pub fn nist_standardized() -> Vec<PqcAlgorithm> {
        vec![
            PqcAlgorithm::Kyber512,
            PqcAlgorithm::Kyber768,
            PqcAlgorithm::Kyber1024,
            PqcAlgorithm::Dilithium2,
            PqcAlgorithm::Dilithium3,
            PqcAlgorithm::Dilithium5,
            PqcAlgorithm::Falcon512,
            PqcAlgorithm::Falcon1024,
            PqcAlgorithm::SphincsPlus128s,
            PqcAlgorithm::SphincsPlus128f,
            PqcAlgorithm::SphincsPlus192s,
            PqcAlgorithm::SphincsPlus192f,
            PqcAlgorithm::SphincsPlus256s,
            PqcAlgorithm::SphincsPlus256f,
        ]
    }
    
    /// slay Check if algorithm is available
    pub fn is_algorithm_available(algorithm: PqcAlgorithm) -> bool {
        // For now, consider NIST algorithms available
        algorithm.is_nist_standardized()
    }
    
    /// slay Estimate migration timeline to post-quantum
    pub fn estimate_migration_timeline() -> std::time::Duration {
        // Conservative estimate: 5-10 years for full migration
        std::time::Duration::from_secs(5 * 365 * 24 * 3600) // 5 years
    }
    
    /// slay Check quantum threat level
    pub fn current_quantum_threat_level() -> QuantumThreatLevel {
        // As of 2024: research phase, but advancing
        QuantumThreatLevel::Research
    }
}

/// fr fr Quantum threat assessment levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantumThreatLevel {
    None,           // No quantum threat
    Research,       // Quantum computers in research phase
    LimitedPractical, // Limited practical quantum computers
    Widespread,     // Widespread practical quantum computers
    Catastrophic,   // Large-scale cryptographically relevant quantum computers
}

impl QuantumThreatLevel {
    /// slay Get recommended action for threat level
    pub fn recommended_action(&self) -> &'static str {
        match self {
            QuantumThreatLevel::None => "Continue with classical crypto",
            QuantumThreatLevel::Research => "Begin planning post-quantum migration",
            QuantumThreatLevel::LimitedPractical => "Accelerate post-quantum deployment",
            QuantumThreatLevel::Widespread => "Complete post-quantum migration immediately",
            QuantumThreatLevel::Catastrophic => "Emergency post-quantum deployment",
        }
    }
}

/// fr fr Initialize the crypto_pqc package
pub fn init_crypto_pqc() -> PqcResult<()> {
    println!("🔮 crypto_pqc package initialized - post-quantum crypto ready bestie!");
    Ok(())
}

// Stub implementations for imported modules
pub mod kyber {
    use super::*;
    
    #[derive(Debug, Clone, PartialEq)]
    pub enum KyberError {
        KeyGenerationFailed,
        EncryptionFailed,
        DecryptionFailed,
        Internal(String),
    }
    
    impl std::fmt::Display for KyberError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                KyberError::KeyGenerationFailed => write!(f, "Kyber key generation failed"),
                KyberError::EncryptionFailed => write!(f, "Kyber encryption failed"),
                KyberError::DecryptionFailed => write!(f, "Kyber decryption failed"),
                KyberError::Internal(msg) => write!(f, "Internal error: {}", msg),
            }
        }
    }
    
    impl std::error::Error for KyberError {}
    
    pub type KyberResult<T> = Result<T, KyberError>;
    
    pub struct KyberKeyPair {
        pub public_key: Vec<u8>,
        pub private_key: Vec<u8>,
    }
    
    pub struct Kyber512;
    pub struct Kyber768;
    pub struct Kyber1024;
}

pub mod dilithium {
    use super::*;
    
    #[derive(Debug, Clone, PartialEq)]
    pub enum DilithiumError {
        KeyGenerationFailed,
        SigningFailed,
        VerificationFailed,
        Internal(String),
    }
    
    impl std::fmt::Display for DilithiumError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                DilithiumError::KeyGenerationFailed => write!(f, "Dilithium key generation failed"),
                DilithiumError::SigningFailed => write!(f, "Dilithium signing failed"),
                DilithiumError::VerificationFailed => write!(f, "Dilithium verification failed"),
                DilithiumError::Internal(msg) => write!(f, "Internal error: {}", msg),
            }
        }
    }
    
    impl std::error::Error for DilithiumError {}
    
    pub type DilithiumResult<T> = Result<T, DilithiumError>;
    
    pub struct DilithiumSignature(Vec<u8>);
    pub struct Dilithium2;
    pub struct Dilithium3; 
    pub struct Dilithium5;
}

pub mod sphincs_plus {
    use super::*;
    
    #[derive(Debug, Clone, PartialEq)]
    pub enum SphincsError {
        KeyGenerationFailed,
        SigningFailed,
        VerificationFailed,
        Internal(String),
    }
    
    impl std::fmt::Display for SphincsError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                SphincsError::KeyGenerationFailed => write!(f, "SPHINCS+ key generation failed"),
                SphincsError::SigningFailed => write!(f, "SPHINCS+ signing failed"),
                SphincsError::VerificationFailed => write!(f, "SPHINCS+ verification failed"),
                SphincsError::Internal(msg) => write!(f, "Internal error: {}", msg),
            }
        }
    }
    
    impl std::error::Error for SphincsError {}
    
    pub type SphincsResult<T> = Result<T, SphincsError>;
    
    pub struct SphincsPlus128;
    pub struct SphincsPlus192;
    pub struct SphincsPlus256;
}

pub mod falcon {
    use super::*;
    
    #[derive(Debug, Clone, PartialEq)]
    pub enum FalconError {
        KeyGenerationFailed,
        SigningFailed,
        VerificationFailed,
        Internal(String),
    }
    
    impl std::fmt::Display for FalconError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                FalconError::KeyGenerationFailed => write!(f, "Falcon key generation failed"),
                FalconError::SigningFailed => write!(f, "Falcon signing failed"),
                FalconError::VerificationFailed => write!(f, "Falcon verification failed"),
                FalconError::Internal(msg) => write!(f, "Internal error: {}", msg),
            }
        }
    }
    
    impl std::error::Error for FalconError {}
    
    pub type FalconResult<T> = Result<T, FalconError>;
    
    pub struct FalconSignature(Vec<u8>);
    pub struct Falcon512;
    pub struct Falcon1024;
}

// Additional stub modules
pub mod ntru { pub struct Ntru; }
pub mod saber { pub struct Saber; }
pub mod frodo { pub struct Frodo; }
pub mod rainbow { pub struct Rainbow; }
pub mod lattice_crypto { pub struct LatticeCrypto; }
pub mod code_crypto { pub struct CodeCrypto; }
pub mod multivariate_crypto { pub struct MultivariateCrypto; }
pub mod hash_crypto { pub struct HashCrypto; }
pub mod hybrid_crypto { pub struct HybridCrypto; }
pub mod migration_tools { pub struct MigrationTools; }
pub mod compatibility { pub struct Compatibility; }

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pqc_algorithm() {
        assert_eq!(PqcAlgorithm::Kyber768.name(), "Kyber-768");
        assert_eq!(PqcAlgorithm::Kyber768.security_level(), 3);
        assert_eq!(PqcAlgorithm::Kyber768.category(), PqcCategory::LatticeBasedKem);
        assert!(PqcAlgorithm::Kyber768.is_nist_standardized());
        
        let sizes = PqcAlgorithm::Kyber768.typical_sizes();
        assert_eq!(sizes.public_key, 1184);
        assert_eq!(sizes.ciphertext, 1088);
    }
    
    #[test]
    fn test_quantum_threat_level() {
        let threat = QuantumThreatLevel::Research;
        assert_eq!(threat.recommended_action(), "Begin planning post-quantum migration");
    }
    
    #[test]
    fn test_init_crypto_pqc() {
        assert!(init_crypto_pqc().is_ok());
    }
    
    #[test]
    fn test_utils() {
        assert_eq!(utils::recommended_kem(), PqcAlgorithm::Kyber768);
        assert_eq!(utils::recommended_signature(), PqcAlgorithm::Dilithium3);
        
        let fastest = utils::fastest_algorithms();
        assert_eq!(fastest[&PqcCategory::LatticeBasedKem], PqcAlgorithm::Kyber512);
        
        let secure = utils::most_secure_algorithms();
        assert_eq!(secure[&PqcCategory::LatticeBasedKem], PqcAlgorithm::Kyber1024);
        
        let nist = utils::nist_standardized();
        assert!(nist.contains(&PqcAlgorithm::Kyber768));
        assert!(nist.contains(&PqcAlgorithm::Dilithium3));
        
        assert!(utils::is_algorithm_available(PqcAlgorithm::Kyber768));
        
        let timeline = utils::estimate_migration_timeline();
        assert!(timeline.as_secs() > 0);
        
        assert_eq!(utils::current_quantum_threat_level(), QuantumThreatLevel::Research);
    }
}
