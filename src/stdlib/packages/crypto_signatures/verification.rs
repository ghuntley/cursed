/// fr fr Universal Signature Verification - One interface to verify them all bestie!
/// 
/// Unified verification interface for all signature algorithms supported by CURSED crypto.
/// Provides algorithm detection, batch verification, and comprehensive error handling.

// use crate::stdlib::packages::crypto_signatures::errors::{SignatureError, SignatureResult};
// use crate::stdlib::packages::crypto_signatures::key_management::{PublicKey, KeyType};
// use crate::stdlib::packages::crypto_signatures::ed25519::{Ed25519Verifier, utils as ed25519_utils};
// use crate::stdlib::packages::crypto_signatures::ecdsa::{EcdsaVerifier, EcdsaCurve, utils as ecdsa_utils};
// Placeholder imports disabled
    RsaVerifier, RsaSignatureScheme, RsaKeySize, RsaHashAlgorithm, utils as rsa_utils
// };
use crate::error::CursedError;
// use crate::stdlib::packages::crypto_signatures::multisig::{MultiSigSigner, MultiSignature};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Universal signature verification trait
pub trait SignatureVerification {
    /// Verify a signature against a message
    fn verify(&self, message: &[u8], signature: &[u8]) -> SignatureResult<bool>;
    
    /// Get the algorithm name
    fn algorithm(&self) -> &str;
    
    /// Get the public key identifier
    fn key_id(&self) -> &str;
/// Universal verifier that can handle any supported signature algorithm
#[derive(Debug)]
pub enum UniversalVerifier {
impl UniversalVerifier {
    /// Create a universal verifier from a public key
    pub fn new(public_key: PublicKey) -> SignatureResult<Self> {
        match public_key.key_type {
            KeyType::Ed25519 => {
                let verifier = Ed25519Verifier::new(public_key)?;
                Ok(UniversalVerifier::Ed25519(verifier))
            KeyType::EcdsaSecp256k1 | KeyType::EcdsaSecp256r1 => {
                let verifier = EcdsaVerifier::new(public_key)?;
                Ok(UniversalVerifier::Ecdsa(verifier))
            KeyType::RsaPss2048 | KeyType::RsaPss3072 | KeyType::RsaPss4096 |
            KeyType::RsaPkcs1v15_2048 | KeyType::RsaPkcs1v15_3072 | KeyType::RsaPkcs1v15_4096 => {
                // Default to RSA-PSS with SHA-256 for RSA verifiers
                    KeyType::RsaPss2048 | KeyType::RsaPss3072 | KeyType::RsaPss4096) {
                    RsaSignatureScheme::Pss
                } else {
                    RsaSignatureScheme::Pkcs1v15
                
                let verifier = RsaVerifier::new(public_key, scheme, RsaHashAlgorithm::Sha256)?;
                Ok(UniversalVerifier::Rsa(verifier))
        }
    }
    
    /// Create an RSA verifier with specific parameters
    pub fn new_rsa(
        hash_algorithm: RsaHashAlgorithm
    ) -> SignatureResult<Self> {
        let verifier = RsaVerifier::new(public_key, scheme, hash_algorithm)?;
        Ok(UniversalVerifier::Rsa(verifier))
    /// Auto-detect algorithm and create verifier
    pub fn auto_detect(public_key_bytes: &[u8]) -> SignatureResult<Self> {
        let key_type = detect_key_type(public_key_bytes)?;
        let public_key = PublicKey::new(key_type, public_key_bytes.to_vec(), None)?;
        Self::new(public_key)
    }
}

impl SignatureVerification for UniversalVerifier {
    fn verify(&self, message: &[u8], signature: &[u8]) -> SignatureResult<bool> {
        match self {
        }
    }
    
    fn algorithm(&self) -> &str {
        match self {
        }
    }
    
    fn key_id(&self) -> &str {
        match self {
        }
    }
/// Verification request for batch processing
#[derive(Debug, Clone)]
pub struct VerificationRequest {
impl VerificationRequest {
    /// Create a new verification request
    pub fn new(
        request_id: Option<String>
    ) -> Self {
        let request_id = request_id.unwrap_or_else(|| {
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos())
        });
        
        Self {
        }
    }
/// Verification result
#[derive(Debug, Clone)]
pub struct VerificationResult {
/// Batch verifier for processing multiple signatures efficiently
#[derive(Debug)]
pub struct BatchVerifier {
/// Batch verification statistics
#[derive(Debug, Default)]
pub struct BatchVerificationStats {
impl BatchVerifier {
    /// Create a new batch verifier
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// Add a verification request to the batch
    pub fn add_request(&mut self, request: VerificationRequest) {
        self.requests.push(request);
    /// Add a request from raw components
    pub fn add_verification(
        request_id: Option<String>
    ) {
        let request = VerificationRequest::new(
        );
        self.add_request(request);
    /// Process all verification requests
    pub fn verify_batch(&mut self) -> SignatureResult<Vec<VerificationResult>> {
        let start_time = std::time::Instant::now();
        let mut results = Vec::with_capacity(self.requests.len());
        
        for request in &self.requests {
            let request_start = std::time::Instant::now();
            
            let result = match self.verify_single_request(request) {
                Ok(is_valid) => VerificationResult {
                Err(e) => VerificationResult {
            
            results.push(result);
        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            stats.total_requests += self.requests.len() as u64;
            stats.total_processing_time += start_time.elapsed();
            
            for result in &results {
                if result.is_valid {
                    stats.successful_verifications += 1;
                } else {
                    stats.failed_verifications += 1;
                *stats.by_algorithm.entry(result.algorithm.clone()).or_insert(0) += 1;
            }
        }
        
        self.requests.clear();
        Ok(results)
    /// Get the number of pending requests
    pub fn pending_count(&self) -> usize {
        self.requests.len()
    /// Clear all pending requests
    pub fn clear(&mut self) {
        self.requests.clear();
    /// Get batch verification statistics
    pub fn get_stats(&self) -> BatchVerificationStats {
        self.stats.lock()
            .map(|stats| BatchVerificationStats {
            })
            .unwrap_or_default()
    /// Verify a single request
    fn verify_single_request(&self, request: &VerificationRequest) -> SignatureResult<bool> {
        let verifier = UniversalVerifier::new(request.public_key.clone())?;
        verifier.verify(&request.message, &request.signature)
    /// Detect algorithm from public key
    fn detect_algorithm(&self, public_key: &PublicKey) -> String {
        match public_key.key_type {
        }
    }
impl Default for BatchVerifier {
    fn default() -> Self {
        Self::new()
    }
}

/// Multi-signature verification wrapper
#[derive(Debug)]
pub struct MultiSigVerificationWrapper {
impl MultiSigVerificationWrapper {
    /// Create a new multi-signature verification wrapper
    pub fn new(signer: MultiSigSigner) -> Self {
        Self { signer }
    }
    
    /// Verify a multi-signature
    pub fn verify_multisig(&self, multisig: &MultiSignature, message: &[u8]) -> SignatureResult<bool> {
        self.signer.verify_multisig(multisig, message)
    }
}

/// Utility functions for signature verification
pub mod utils {
    use super::*;
    
    /// Quick signature verification with automatic algorithm detection
    pub fn quick_verify(public_key_bytes: &[u8], message: &[u8], signature: &[u8]) -> bool {
        let verifier = match UniversalVerifier::auto_detect(public_key_bytes) {
        
        verifier.verify(message, signature).unwrap_or(false)
    /// Verify Ed25519 signature
    pub fn verify_ed25519(public_key: &[u8], message: &[u8], signature: &[u8]) -> SignatureResult<bool> {
        ed25519_utils::verify_signature(public_key, message, signature)
    /// Verify ECDSA signature
    pub fn verify_ecdsa(
        curve: EcdsaCurve
    ) -> SignatureResult<bool> {
        ecdsa_utils::verify_signature(public_key, message, signature, curve)
    /// Verify RSA signature
    pub fn verify_rsa(
        hash_algorithm: RsaHashAlgorithm
    ) -> SignatureResult<bool> {
        rsa_utils::verify_signature(public_key, message, signature, key_size, scheme, hash_algorithm)
    /// Check if a signature format is valid for any algorithm
    pub fn is_valid_signature_format(signature: &[u8]) -> bool {
        match signature.len() {
            64 => true,  // Ed25519, ECDSA
            256 => true, // RSA-2048
            384 => true, // RSA-3072
            512 => true, // RSA-4096
        }
    }
    
    /// Detect signature algorithm from signature size
    pub fn detect_algorithm_from_signature(signature: &[u8]) -> Option<String> {
        match signature.len() {
        }
    }
/// Detect key type from public key bytes
fn detect_key_type(public_key_bytes: &[u8]) -> SignatureResult<KeyType> {
    match public_key_bytes.len() {
        33 => {
            // ECDSA compressed format - need more heuristics to distinguish curves
            if public_key_bytes[0] == 0x02 || public_key_bytes[0] == 0x03 {
                // Default to secp256k1 for now
                Ok(KeyType::EcdsaSecp256k1)
            } else {
                Err(SignatureError::InvalidPublicKey(
                    "Invalid ECDSA public key format".to_string()
                ))
            }
        256 => Ok(KeyType::RsaPss2048), // Default to PSS for RSA
        _ => Err(SignatureError::InvalidPublicKey(
            format!("Unsupported public key size: {}", public_key_bytes.len())
    }
}
