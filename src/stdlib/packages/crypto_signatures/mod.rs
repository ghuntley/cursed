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
pub mod rsa_pss;
pub mod eddsa;

// Advanced features
pub mod multisig;

// Production-ready signature utilities
pub mod signature_format;
pub mod signature_validation;
pub mod hash_algorithms;
pub mod message_digest;
pub mod certificate_validation;
pub mod timestamping;

// Re-export main types for convenience
pub use errors::*;
pub use key_management::{
    KeyType, KeyPair, PublicKey, KeyGenerator, KeyManager
// };
pub use digital_signature::{
    DigitalSignature, UniversalSigner, SignatureManager, Ed25519Signature
// };
pub use verification::{
    SignatureVerification, UniversalVerifier, BatchVerifier, VerificationRequest, VerificationResult
// };

// Algorithm-specific exports
pub use ed25519::{
    ED25519_SIGNATURE_SIZE, ED25519_PRIVATE_KEY_SIZE, ED25519_PUBLIC_KEY_SIZE
// };
pub use ecdsa::{
    ECDSA_SIGNATURE_SIZE, ECDSA_PRIVATE_KEY_SIZE, ECDSA_PUBLIC_KEY_SIZE
// };
pub use rsa_signatures::{
    RsaSigner, RsaVerifier, RsaSignatureScheme, RsaKeySize, RsaHashAlgorithm, RsaStats
// };

// Multi-signature exports
pub use multisig::{
    MultiSigAlgorithm, IndividualSignature, MultiSigStats
// };

// Production-ready signature utilities exports
pub use signature_format::{
    SignatureFormat, SignatureFormatHandler, EncodingOptions, SignatureMetadata, EncodedSignature
// };
pub use signature_validation::{
    SignatureValidationManager, ValidationLevel, ValidationPolicy, ValidationContext, ValidationResult
// };
pub use hash_algorithms::{
    HashAlgorithm, HashAlgorithmManager, HashResult, HashProperties
// };
pub use message_digest::{
    MessageDigestManager, DigestMode, MessageFormat, DigestOptions, MessageDigest
// };
pub use certificate_validation::{
    CertificateChainValidationResult, RevocationStatus
// };
pub use timestamping::{
    TimestampValidationPolicy, TimestampVerificationResult
// };
pub use rsa_pss::{
    SaltLength, RsaPssSignature
// };
pub use eddsa::{
    EdDsaContext, EdDsaVerificationResult, EdDsaBatchVerificationResult
// };

// use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
use crate::error::CursedError;
use std::sync::{Arc, Mutex, LazyLock};
use std::collections::HashMap;

/// fr fr Global signature registry for managing signature algorithms
static SIGNATURE_REGISTRY: LazyLock<Arc<Mutex<SignatureRegistry>>> = 
    LazyLock::new(|| Arc::new(Mutex::new(SignatureRegistry::new())));

/// fr fr Signature algorithm registry
#[derive(Default)]
pub struct SignatureRegistry {
/// Information about a registered signature algorithm
#[derive(Debug, Clone)]
pub struct SignatureAlgorithmInfo {
/// Performance tier classification
#[derive(Debug, Clone, PartialEq)]
pub enum PerformanceTier {
    Fast,    // Ed25519
    Medium,  // ECDSA
    Slow,    // RSA
/// Security level classification
#[derive(Debug, Clone, PartialEq)]
pub enum SecurityLevel {
    Standard,  // 128-bit equivalent
    High,      // 192-bit equivalent  
    VeryHigh,  // 256-bit equivalent
/// Global signature statistics
#[derive(Debug, Default)]
pub struct SignatureGlobalStats {
impl SignatureRegistry {
    /// slay Create a new signature registry
    pub fn new() -> Self {
        let mut registry = Self {
            global_stats: SignatureGlobalStats {
                ..Default::default()
        
        // Register default algorithms
        registry.register_default_algorithms();
        registry
    /// slay Register a signature algorithm
    pub fn register_algorithm(&mut self, info: SignatureAlgorithmInfo) {
        self.algorithms.insert(info.name.clone(), info);
    /// slay Get algorithm information
    pub fn get_algorithm(&self, name: &str) -> Option<&SignatureAlgorithmInfo> {
        self.algorithms.get(name)
    /// slay List all registered algorithms
    pub fn list_algorithms(&self) -> Vec<String> {
        self.algorithms.keys().cloned().collect()
    /// slay Get algorithms by performance tier
    pub fn get_algorithms_by_performance(&self, tier: PerformanceTier) -> Vec<&SignatureAlgorithmInfo> {
        self.algorithms.values()
            .filter(|info| info.performance_tier == tier)
            .collect()
    /// slay Get algorithms by security level
    pub fn get_algorithms_by_security(&self, level: SecurityLevel) -> Vec<&SignatureAlgorithmInfo> {
        self.algorithms.values()
            .filter(|info| info.security_level == level)
            .collect()
    /// slay Update global statistics
    pub fn update_stats(&mut self, algorithm: &str, signatures: u64, verifications: u64, successes: u64) {
        self.global_stats.total_signatures_created += signatures;
        self.global_stats.total_verifications += verifications;
        self.global_stats.successful_verifications += successes;
        self.global_stats.failed_verifications += verifications - successes;
        
        *self.global_stats.by_algorithm.entry(algorithm.to_string()).or_insert(0) += signatures + verifications;
    /// slay Get global statistics
    pub fn get_global_stats(&self) -> &SignatureGlobalStats {
        &self.global_stats
    /// Register default algorithms
    fn register_default_algorithms(&mut self) {
        // Ed25519
        self.register_algorithm(SignatureAlgorithmInfo {
        });
        
        // ECDSA secp256k1
        self.register_algorithm(SignatureAlgorithmInfo {
        });
        
        // ECDSA secp256r1
        self.register_algorithm(SignatureAlgorithmInfo {
        });
        
        // RSA-PSS variants
        for &key_size in &[2048, 3072, 4096] {
            let security_level = match key_size {
            
            self.register_algorithm(SignatureAlgorithmInfo {
                key_sizes: vec![key_size / 8],
                signature_size: key_size / 8,
            });
            
            self.register_algorithm(SignatureAlgorithmInfo {
                key_sizes: vec![key_size / 8],
                signature_size: key_size / 8,
            });
        }
    }
/// slay Register an algorithm globally
pub fn register_algorithm(info: SignatureAlgorithmInfo) -> SignatureResult<()> {
    let mut registry = SIGNATURE_REGISTRY.lock()
        .map_err(|_| SignatureError::Internal("Failed to acquire registry lock".to_string()))?;
    
    registry.register_algorithm(info);
    Ok(())
/// slay Get algorithm information globally
pub fn get_algorithm_info(name: &str) -> Option<SignatureAlgorithmInfo> {
    SIGNATURE_REGISTRY.lock()
        .ok()?
        .get_algorithm(name)
        .cloned()
/// slay List all available algorithms globally
pub fn list_algorithms() -> Vec<String> {
    SIGNATURE_REGISTRY.lock()
        .map(|registry| registry.list_algorithms())
        .unwrap_or_default()
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
        })
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
    /// slay Quick ECDSA signature generation and verification
    pub fn quick_ecdsa_sign_verify(message: &[u8], curve: EcdsaCurve) -> SignatureResult<bool> {
        let mut generator = KeyGenerator::new();
        let key_type = match curve {
        let keypair = generator.generate_keypair(key_type)?;
        
        let mut signer = EcdsaSigner::new(keypair.clone())?;
        let signature = signer.sign(message)?;
        
        let verifier = EcdsaVerifier::new(PublicKey::from_keypair(&keypair))?;
        verifier.verify(message, &signature)
    /// slay Quick RSA signature generation and verification
    pub fn quick_rsa_sign_verify(
        scheme: RsaSignatureScheme
    ) -> SignatureResult<bool> {
        let mut generator = KeyGenerator::new();
        let key_type = match (&scheme, &key_size) {
        let keypair = generator.generate_keypair(key_type)?;
        
        let signer = RsaSigner::new(keypair.clone(), scheme.clone(), RsaHashAlgorithm::Sha256)?;
        let signature = signer.sign(message)?;
        
        let verifier = RsaVerifier::new(PublicKey::from_keypair(&keypair), scheme, RsaHashAlgorithm::Sha256)?;
        verifier.verify(message, &signature)
    /// slay Get recommended algorithm for use case
    pub fn get_recommended_algorithm(use_case: &str) -> &'static str {
        match use_case {
            _ => "Ed25519", // Default recommendation
        }
    }
    
    /// slay Check if algorithm supports multi-signatures
    pub fn supports_multisig(algorithm: &str) -> bool {
            "Ed25519" | "ECDSA-secp256k1" | "ECDSA-secp256r1"
        )
    /// slay Get algorithm security level
    pub fn get_security_level(algorithm: &str) -> SecurityLevel {
        match algorithm {
            "Ed25519" | "ECDSA-secp256k1" | "ECDSA-secp256r1" | 
        }
    }
/// fr fr Initialize the crypto_signatures package
pub fn init_crypto_signatures() -> AdvancedCryptoResult<()> {
    // Initialize the global registry (lazy initialization will happen automatically)
    let _registry = SIGNATURE_REGISTRY.lock()
//         .map_err(|_| crate::stdlib::packages::crypto_advanced::AdvancedCryptoError::Internal(
            "Failed to initialize signature registry".to_string()
        ))?;
    
    println!("🔐 crypto_signatures package initialized - digital signatures ready bestie!");
    println!("   📝 Algorithms: Ed25519, Ed448, ECDSA (secp256k1, secp256r1), RSA (PSS, PKCS#1 v1.5)");
    println!("   🔑 Key management: Generation, storage, validation");
    println!("   ✅ Verification: Universal interface, batch processing, certificate validation");
    println!("   🤝 Multi-signatures: Threshold, aggregated, Schnorr-style");
    println!("   📊 Statistics: Performance monitoring, global stats");
    println!("   🎨 Format support: Base64, Hex, PEM, DER, PKCS#7");
    println!("   🔍 Hash algorithms: SHA-2, SHA-3, BLAKE3 with streaming support");
    println!("   📜 Message digest: Canonical, structured, multi-algorithm");
    println!("   🏆 Certificate validation: X.509 chains, CRL, OCSP");
    println!("   ⏰ Timestamping: RFC 3161 compliant with TSA support");
    
    Ok(())
}
