/// fr fr Digital Signatures Package - Production-ready crypto signatures bestie!
/// 
/// Comprehensive digital signature implementation supporting multiple algorithms,
/// key management, multi-signatures, and universal verification interfaces.

// Core modules
pub mod errors;
pub mod key_management;
pub mod digital_signature;
pub mod verification;

// Algorithm implementations  
pub mod ed25519;
pub mod ecdsa;
pub mod rsa_signatures;

// Advanced features
pub mod multisig;

// Re-export main types for convenience
pub use errors::*;
pub use key_management::{
    KeyType, KeyPair, PublicKey, KeyGenerator, KeyManager
};
pub use digital_signature::{
    DigitalSignature, UniversalSigner, SignatureManager, Ed25519Signature
};
pub use verification::{
    SignatureVerification, UniversalVerifier, BatchVerifier, VerificationRequest, VerificationResult
};

// Algorithm-specific exports
pub use ed25519::{
    Ed25519Signer, Ed25519Verifier, Ed25519BatchVerifier, Ed25519Stats,
    ED25519_SIGNATURE_SIZE, ED25519_PRIVATE_KEY_SIZE, ED25519_PUBLIC_KEY_SIZE
};
pub use ecdsa::{
    EcdsaSigner, EcdsaVerifier, EcdsaCurve, EcdsaStats,
    ECDSA_SIGNATURE_SIZE, ECDSA_PRIVATE_KEY_SIZE, ECDSA_PUBLIC_KEY_SIZE
};
pub use rsa_signatures::{
    RsaSigner, RsaVerifier, RsaSignatureScheme, RsaKeySize, RsaHashAlgorithm, RsaStats
};

// Multi-signature exports
pub use multisig::{
    MultiSigSigner, MultiSignature, MultiSigConfig, MultiSigScheme, 
    MultiSigAlgorithm, IndividualSignature, MultiSigStats
};

use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
use std::sync::{Arc, Mutex, LazyLock};
use std::collections::HashMap;

/// fr fr Global signature registry for managing signature algorithms
static SIGNATURE_REGISTRY: LazyLock<Arc<Mutex<SignatureRegistry>>> = 
    LazyLock::new(|| Arc::new(Mutex::new(SignatureRegistry::new())));

/// fr fr Signature algorithm registry
#[derive(Default)]
pub struct SignatureRegistry {
    algorithms: HashMap<String, SignatureAlgorithmInfo>,
    global_stats: SignatureGlobalStats,
}

/// Information about a registered signature algorithm
#[derive(Debug, Clone)]
pub struct SignatureAlgorithmInfo {
    pub name: String,
    pub key_sizes: Vec<usize>,
    pub signature_size: usize,
    pub is_quantum_resistant: bool,
    pub performance_tier: PerformanceTier,
    pub security_level: SecurityLevel,
}

/// Performance tier classification
#[derive(Debug, Clone, PartialEq)]
pub enum PerformanceTier {
    Fast,    // Ed25519
    Medium,  // ECDSA
    Slow,    // RSA
}

/// Security level classification
#[derive(Debug, Clone, PartialEq)]
pub enum SecurityLevel {
    Standard,  // 128-bit equivalent
    High,      // 192-bit equivalent  
    VeryHigh,  // 256-bit equivalent
}

/// Global signature statistics
#[derive(Debug, Default)]
pub struct SignatureGlobalStats {
    pub total_signatures_created: u64,
    pub total_verifications: u64,
    pub successful_verifications: u64,
    pub failed_verifications: u64,
    pub by_algorithm: HashMap<String, u64>,
    pub uptime: std::time::SystemTime,
}

impl SignatureRegistry {
    /// slay Create a new signature registry
    pub fn new() -> Self {
        let mut registry = Self {
            algorithms: HashMap::new(),
            global_stats: SignatureGlobalStats {
                uptime: std::time::SystemTime::now(),
                ..Default::default()
            },
        };
        
        // Register default algorithms
        registry.register_default_algorithms();
        registry
    }
    
    /// slay Register a signature algorithm
    pub fn register_algorithm(&mut self, info: SignatureAlgorithmInfo) {
        self.algorithms.insert(info.name.clone(), info);
    }
    
    /// slay Get algorithm information
    pub fn get_algorithm(&self, name: &str) -> Option<&SignatureAlgorithmInfo> {
        self.algorithms.get(name)
    }
    
    /// slay List all registered algorithms
    pub fn list_algorithms(&self) -> Vec<String> {
        self.algorithms.keys().cloned().collect()
    }
    
    /// slay Get algorithms by performance tier
    pub fn get_algorithms_by_performance(&self, tier: PerformanceTier) -> Vec<&SignatureAlgorithmInfo> {
        self.algorithms.values()
            .filter(|info| info.performance_tier == tier)
            .collect()
    }
    
    /// slay Get algorithms by security level
    pub fn get_algorithms_by_security(&self, level: SecurityLevel) -> Vec<&SignatureAlgorithmInfo> {
        self.algorithms.values()
            .filter(|info| info.security_level == level)
            .collect()
    }
    
    /// slay Update global statistics
    pub fn update_stats(&mut self, algorithm: &str, signatures: u64, verifications: u64, successes: u64) {
        self.global_stats.total_signatures_created += signatures;
        self.global_stats.total_verifications += verifications;
        self.global_stats.successful_verifications += successes;
        self.global_stats.failed_verifications += verifications - successes;
        
        *self.global_stats.by_algorithm.entry(algorithm.to_string()).or_insert(0) += signatures + verifications;
    }
    
    /// slay Get global statistics
    pub fn get_global_stats(&self) -> &SignatureGlobalStats {
        &self.global_stats
    }
    
    /// Register default algorithms
    fn register_default_algorithms(&mut self) {
        // Ed25519
        self.register_algorithm(SignatureAlgorithmInfo {
            name: "Ed25519".to_string(),
            key_sizes: vec![32],
            signature_size: 64,
            is_quantum_resistant: false,
            performance_tier: PerformanceTier::Fast,
            security_level: SecurityLevel::Standard,
        });
        
        // ECDSA secp256k1
        self.register_algorithm(SignatureAlgorithmInfo {
            name: "ECDSA-secp256k1".to_string(),
            key_sizes: vec![32],
            signature_size: 64,
            is_quantum_resistant: false,
            performance_tier: PerformanceTier::Medium,
            security_level: SecurityLevel::Standard,
        });
        
        // ECDSA secp256r1
        self.register_algorithm(SignatureAlgorithmInfo {
            name: "ECDSA-secp256r1".to_string(),
            key_sizes: vec![32],
            signature_size: 64,
            is_quantum_resistant: false,
            performance_tier: PerformanceTier::Medium,
            security_level: SecurityLevel::Standard,
        });
        
        // RSA-PSS variants
        for &key_size in &[2048, 3072, 4096] {
            let security_level = match key_size {
                2048 => SecurityLevel::Standard,
                3072 => SecurityLevel::High,
                4096 => SecurityLevel::VeryHigh,
                _ => SecurityLevel::Standard,
            };
            
            self.register_algorithm(SignatureAlgorithmInfo {
                name: format!("RSA-PSS-{}", key_size),
                key_sizes: vec![key_size / 8],
                signature_size: key_size / 8,
                is_quantum_resistant: false,
                performance_tier: PerformanceTier::Slow,
                security_level,
            });
            
            self.register_algorithm(SignatureAlgorithmInfo {
                name: format!("RSA-PKCS1v15-{}", key_size),
                key_sizes: vec![key_size / 8],
                signature_size: key_size / 8,
                is_quantum_resistant: false,
                performance_tier: PerformanceTier::Slow,
                security_level,
            });
        }
    }
}

/// slay Register an algorithm globally
pub fn register_algorithm(info: SignatureAlgorithmInfo) -> SignatureResult<()> {
    let mut registry = SIGNATURE_REGISTRY.lock()
        .map_err(|_| SignatureError::Internal("Failed to acquire registry lock".to_string()))?;
    
    registry.register_algorithm(info);
    Ok(())
}

/// slay Get algorithm information globally
pub fn get_algorithm_info(name: &str) -> Option<SignatureAlgorithmInfo> {
    SIGNATURE_REGISTRY.lock()
        .ok()?
        .get_algorithm(name)
        .cloned()
}

/// slay List all available algorithms globally
pub fn list_algorithms() -> Vec<String> {
    SIGNATURE_REGISTRY.lock()
        .map(|registry| registry.list_algorithms())
        .unwrap_or_default()
}

/// slay Update global statistics
pub fn update_global_stats(algorithm: &str, signatures: u64, verifications: u64, successes: u64) {
    if let Ok(mut registry) = SIGNATURE_REGISTRY.lock() {
        registry.update_stats(algorithm, signatures, verifications, successes);
    }
}

/// slay Get global statistics
pub fn get_global_stats() -> Option<SignatureGlobalStats> {
    SIGNATURE_REGISTRY.lock()
        .ok()
        .map(|registry| SignatureGlobalStats {
            total_signatures_created: registry.global_stats.total_signatures_created,
            total_verifications: registry.global_stats.total_verifications,
            successful_verifications: registry.global_stats.successful_verifications,
            failed_verifications: registry.global_stats.failed_verifications,
            by_algorithm: registry.global_stats.by_algorithm.clone(),
            uptime: registry.global_stats.uptime,
        })
}

/// fr fr Crypto utilities and helper functions
pub mod utils {
    use super::*;
    
    /// slay Quick Ed25519 signature generation and verification
    pub fn quick_ed25519_sign_verify(message: &[u8]) -> SignatureResult<bool> {
        let mut generator = KeyGenerator::new();
        let keypair = generator.generate_keypair(KeyType::Ed25519)?;
        
        let signer = Ed25519Signer::new(keypair.clone())?;
        let signature = signer.sign(message)?;
        
        let verifier = Ed25519Verifier::new(PublicKey::from_keypair(&keypair))?;
        verifier.verify(message, &signature)
    }
    
    /// slay Quick ECDSA signature generation and verification
    pub fn quick_ecdsa_sign_verify(message: &[u8], curve: EcdsaCurve) -> SignatureResult<bool> {
        let mut generator = KeyGenerator::new();
        let key_type = match curve {
            EcdsaCurve::Secp256k1 => KeyType::EcdsaSecp256k1,
            EcdsaCurve::Secp256r1 => KeyType::EcdsaSecp256r1,
        };
        let keypair = generator.generate_keypair(key_type)?;
        
        let mut signer = EcdsaSigner::new(keypair.clone())?;
        let signature = signer.sign(message)?;
        
        let verifier = EcdsaVerifier::new(PublicKey::from_keypair(&keypair))?;
        verifier.verify(message, &signature)
    }
    
    /// slay Quick RSA signature generation and verification
    pub fn quick_rsa_sign_verify(
        message: &[u8], 
        key_size: RsaKeySize, 
        scheme: RsaSignatureScheme
    ) -> SignatureResult<bool> {
        let mut generator = KeyGenerator::new();
        let key_type = match (&scheme, &key_size) {
            (RsaSignatureScheme::Pss, RsaKeySize::Bits2048) => KeyType::RsaPss2048,
            (RsaSignatureScheme::Pss, RsaKeySize::Bits3072) => KeyType::RsaPss3072,
            (RsaSignatureScheme::Pss, RsaKeySize::Bits4096) => KeyType::RsaPss4096,
            (RsaSignatureScheme::Pkcs1v15, RsaKeySize::Bits2048) => KeyType::RsaPkcs1v15_2048,
            (RsaSignatureScheme::Pkcs1v15, RsaKeySize::Bits3072) => KeyType::RsaPkcs1v15_3072,
            (RsaSignatureScheme::Pkcs1v15, RsaKeySize::Bits4096) => KeyType::RsaPkcs1v15_4096,
        };
        let keypair = generator.generate_keypair(key_type)?;
        
        let signer = RsaSigner::new(keypair.clone(), scheme.clone(), RsaHashAlgorithm::Sha256)?;
        let signature = signer.sign(message)?;
        
        let verifier = RsaVerifier::new(PublicKey::from_keypair(&keypair), scheme, RsaHashAlgorithm::Sha256)?;
        verifier.verify(message, &signature)
    }
    
    /// slay Get recommended algorithm for use case
    pub fn get_recommended_algorithm(use_case: &str) -> &'static str {
        match use_case {
            "speed" | "performance" => "Ed25519",
            "bitcoin" | "ethereum" => "ECDSA-secp256k1",
            "nist" | "government" => "ECDSA-secp256r1",
            "legacy" | "compatibility" => "RSA-PKCS1v15-2048",
            "security" | "high-security" => "RSA-PSS-4096",
            "multisig" | "threshold" => "Ed25519",
            _ => "Ed25519", // Default recommendation
        }
    }
    
    /// slay Check if algorithm supports multi-signatures
    pub fn supports_multisig(algorithm: &str) -> bool {
        matches!(algorithm, 
            "Ed25519" | "ECDSA-secp256k1" | "ECDSA-secp256r1"
        )
    }
    
    /// slay Get algorithm security level
    pub fn get_security_level(algorithm: &str) -> SecurityLevel {
        match algorithm {
            "Ed25519" | "ECDSA-secp256k1" | "ECDSA-secp256r1" | 
            "RSA-PSS-2048" | "RSA-PKCS1v15-2048" => SecurityLevel::Standard,
            "RSA-PSS-3072" | "RSA-PKCS1v15-3072" => SecurityLevel::High,
            "RSA-PSS-4096" | "RSA-PKCS1v15-4096" => SecurityLevel::VeryHigh,
            _ => SecurityLevel::Standard,
        }
    }
}

/// fr fr Initialize the crypto_signatures package
pub fn init_crypto_signatures() -> AdvancedCryptoResult<()> {
    // Initialize the global registry (lazy initialization will happen automatically)
    let _registry = SIGNATURE_REGISTRY.lock()
        .map_err(|_| crate::stdlib::packages::crypto_advanced::AdvancedCryptoError::Internal(
            "Failed to initialize signature registry".to_string()
        ))?;
    
    println!("🔐 crypto_signatures package initialized - digital signatures ready bestie!");
    println!("   📝 Algorithms: Ed25519, ECDSA (secp256k1, secp256r1), RSA (PSS, PKCS#1 v1.5)");
    println!("   🔑 Key management: Generation, storage, validation");
    println!("   ✅ Verification: Universal interface, batch processing");
    println!("   🤝 Multi-signatures: Threshold, aggregated, Schnorr-style");
    println!("   📊 Statistics: Performance monitoring, global stats");
    
    Ok(())
}
