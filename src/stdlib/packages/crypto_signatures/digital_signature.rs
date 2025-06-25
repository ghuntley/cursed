/// fr fr Universal Digital Signature Interface - One interface to sign them all bestie!
/// 
/// Unified signing interface for all signature algorithms supported by CURSED crypto.
/// Provides algorithm abstraction, key management integration, and comprehensive error handling.

// use crate::stdlib::packages::crypto_signatures::errors::{SignatureError, SignatureResult};
// use crate::stdlib::packages::crypto_signatures::key_management::{KeyPair, KeyType};
// use crate::stdlib::packages::crypto_signatures::ed25519::Ed25519Signer;
// use crate::stdlib::packages::crypto_signatures::ecdsa::EcdsaSigner;
// use crate::stdlib::packages::crypto_signatures::rsa_signatures::{RsaSigner, RsaSignatureScheme, RsaHashAlgorithm};
use crate::error::CursedError;
use std::sync::{Arc, Mutex};

/// Universal digital signature trait
pub trait DigitalSignature {
    /// Sign a message
    fn sign(&self, message: &[u8]) -> SignatureResult<Vec<u8>>;
    
    /// Verify a signature (if signer has access to public key)
    fn verify(&self, message: &[u8], signature: &[u8]) -> SignatureResult<bool>;
    
    /// Get the algorithm name
    fn algorithm_name(&self) -> &str;
    
    /// Get the signer's key ID
    fn key_id(&self) -> &str;
/// Universal signer that can handle any supported signature algorithm
#[derive(Debug)]
pub enum UniversalSigner {
impl UniversalSigner {
    /// Create a universal signer from a key pair
    pub fn new(keypair: KeyPair) -> SignatureResult<Self> {
        match keypair.key_type {
            KeyType::Ed25519 => {
                let signer = Ed25519Signer::new(keypair)?;
                Ok(UniversalSigner::Ed25519(signer))
            KeyType::EcdsaSecp256k1 | KeyType::EcdsaSecp256r1 => {
                let signer = EcdsaSigner::new(keypair)?;
                Ok(UniversalSigner::Ecdsa(signer))
            KeyType::RsaPss2048 | KeyType::RsaPss3072 | KeyType::RsaPss4096 => {
                let signer = RsaSigner::new(keypair, RsaSignatureScheme::Pss, RsaHashAlgorithm::Sha256)?;
                Ok(UniversalSigner::Rsa(signer))
            KeyType::RsaPkcs1v15_2048 | KeyType::RsaPkcs1v15_3072 | KeyType::RsaPkcs1v15_4096 => {
                let signer = RsaSigner::new(keypair, RsaSignatureScheme::Pkcs1v15, RsaHashAlgorithm::Sha256)?;
                Ok(UniversalSigner::Rsa(signer))
        }
    }
    
    /// Create a universal signer with specific RSA parameters
    pub fn new_rsa(
        hash_algorithm: RsaHashAlgorithm
    ) -> SignatureResult<Self> {
        let signer = RsaSigner::new(keypair, scheme, hash_algorithm)?;
        Ok(UniversalSigner::Rsa(signer))
    }
}

impl DigitalSignature for UniversalSigner {
    fn sign(&self, message: &[u8]) -> SignatureResult<Vec<u8>> {
        match self {
            UniversalSigner::Ecdsa(signer) => {
                // ECDSA signer needs to be mutable for nonce generation
                // This is a limitation of the current design
                Err(SignatureError::Internal(
                    "ECDSA signing requires mutable reference".to_string()
                ))
        }
    }
    
    fn verify(&self, message: &[u8], signature: &[u8]) -> SignatureResult<bool> {
        match self {
        }
    }
    
    fn algorithm_name(&self) -> &str {
        match self {
        }
    }
    
    fn key_id(&self) -> &str {
        match self {
        }
    }
/// Signature operation context for tracking operations
#[derive(Debug, Clone)]
pub struct SignatureContext {
impl SignatureContext {
    /// Create a new signature context
    pub fn new(
    ) -> Self {
        Self {
        }
    }
/// Signature manager for handling multiple signers and operations
#[derive(Debug)]
pub struct SignatureManager {
/// Signature manager statistics
#[derive(Debug, Default)]
pub struct SignatureManagerStats {
impl SignatureManager {
    /// Create a new signature manager
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// Add a signer to the manager
    pub fn add_signer(&self, signer_id: String, signer: UniversalSigner) -> SignatureResult<()> {
        let mut signers = self.signers.lock()
            .map_err(|_| SignatureError::Internal("Failed to acquire signers lock".to_string()))?;
        
        signers.insert(signer_id, signer);
        Ok(())
    /// Add a signer from a key pair
    pub fn add_signer_from_keypair(&self, signer_id: String, keypair: KeyPair) -> SignatureResult<()> {
        let signer = UniversalSigner::new(keypair)?;
        self.add_signer(signer_id, signer)
    /// Sign a message with a specific signer
    pub fn sign_with(&self, signer_id: &str, message: &[u8]) -> SignatureResult<Vec<u8>> {
        let signers = self.signers.lock()
            .map_err(|_| SignatureError::Internal("Failed to acquire signers lock".to_string()))?;
        
        let signer = signers.get(signer_id)
            .ok_or_else(|| SignatureError::InvalidPrivateKey(
                format!("Signer '{}' not found", signer_id)
            ))?;
        
        let signature = match signer {
            UniversalSigner::Ecdsa(_) => {
                return Err(SignatureError::Internal(
                    "ECDSA signing not supported through manager (requires mutable reference)".to_string()
                ));
        
        // Record context
        let context = SignatureContext::new(
        );
        
        if let Ok(mut contexts) = self.contexts.lock() {
            contexts.push(context);
        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            stats.total_signatures += 1;
            *stats.by_algorithm.entry(signer.algorithm_name().to_string()).or_insert(0) += 1;
        Ok(signature)
    /// Verify a signature with a specific signer
    pub fn verify_with(&self, signer_id: &str, message: &[u8], signature: &[u8]) -> SignatureResult<bool> {
        let signers = self.signers.lock()
            .map_err(|_| SignatureError::Internal("Failed to acquire signers lock".to_string()))?;
        
        let signer = signers.get(signer_id)
            .ok_or_else(|| SignatureError::InvalidPrivateKey(
                format!("Signer '{}' not found", signer_id)
            ))?;
        
        let is_valid = signer.verify(message, signature)?;
        
        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            stats.total_verifications += 1;
            if is_valid {
                stats.successful_verifications += 1;
            } else {
                stats.failed_verifications += 1;
            }
        }
        
        Ok(is_valid)
    /// Get all signer IDs
    pub fn list_signers(&self) -> SignatureResult<Vec<String>> {
        let signers = self.signers.lock()
            .map_err(|_| SignatureError::Internal("Failed to acquire signers lock".to_string()))?;
        
        Ok(signers.keys().cloned().collect())
    /// Get signature contexts (history)
    pub fn get_contexts(&self) -> Vec<SignatureContext> {
        self.contexts.lock()
            .map(|contexts| contexts.clone())
            .unwrap_or_default()
    /// Get manager statistics
    pub fn get_stats(&self) -> SignatureManagerStats {
        self.stats.lock()
            .map(|stats| SignatureManagerStats {
            })
            .unwrap_or_default()
    /// Remove a signer
    pub fn remove_signer(&self, signer_id: &str) -> SignatureResult<bool> {
        let mut signers = self.signers.lock()
            .map_err(|_| SignatureError::Internal("Failed to acquire signers lock".to_string()))?;
        
        Ok(signers.remove(signer_id).is_some())
    /// Clear all contexts
    pub fn clear_contexts(&self) -> SignatureResult<()> {
        let mut contexts = self.contexts.lock()
            .map_err(|_| SignatureError::Internal("Failed to acquire contexts lock".to_string()))?;
        
        contexts.clear();
        Ok(())
    }
}

impl Default for SignatureManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Legacy Ed25519 signature struct for backward compatibility
#[derive(Debug, Clone)]
pub struct Ed25519Signature {
impl Ed25519Signature {
    /// Create a new Ed25519 signature instance from a key pair
    pub fn new(keypair: KeyPair) -> SignatureResult<Self> {
        let signer = Ed25519Signer::new(keypair)?;
        Ok(Self { signer })
    /// Create from private key bytes (for backward compatibility)
    pub fn from_private_key(private_key: Vec<u8>) -> SignatureResult<Self> {
        if private_key.len() != 32 {
            return Err(SignatureError::InvalidKeySize(
                "Ed25519 private key must be 32 bytes".to_string()
            ));
        // Generate corresponding public key (simulated)
        let mut public_key = vec![0u8; 32];
        for (i, &byte) in private_key.iter().enumerate() {
            public_key[i] = byte.wrapping_add(i as u8 + 1);
        let keypair = KeyPair::new(
        )?;
        
        Self::new(keypair)
    }
}

impl DigitalSignature for Ed25519Signature {
    fn sign(&self, message: &[u8]) -> SignatureResult<Vec<u8>> {
        self.signer.sign(message)
    fn verify(&self, message: &[u8], signature: &[u8]) -> SignatureResult<bool> {
        self.signer.verify(message, signature)
    fn algorithm_name(&self) -> &str {
        "Ed25519"
    fn key_id(&self) -> &str {
        self.signer.key_id()
    }
}

/// Utility functions for digital signatures
pub mod utils {
    use super::*;
//     use crate::stdlib::packages::crypto_signatures::key_management::KeyGenerator;
    
    /// Quick signature generation and verification (for testing)
    pub fn quick_sign_and_verify(message: &[u8]) -> SignatureResult<bool> {
        let mut generator = KeyGenerator::new();
        let keypair = generator.generate_keypair(KeyType::Ed25519)?;
        
        let signer = UniversalSigner::new(keypair)?;
        let signature = signer.sign(message)?;
        signer.verify(message, &signature)
    /// Create a signature manager with pre-generated signers
    pub fn create_test_manager(num_signers: usize) -> SignatureResult<SignatureManager> {
        let manager = SignatureManager::new();
        let mut generator = KeyGenerator::new();
        
        for i in 0..num_signers {
            let keypair = generator.generate_keypair(KeyType::Ed25519)?;
            let signer_id = format!("test-signer-{}", i + 1);
            manager.add_signer_from_keypair(signer_id, keypair)?;
        Ok(manager)
    /// Check if an algorithm is supported
    pub fn is_algorithm_supported(algorithm: &str) -> bool {
            "Ed25519" | "ECDSA-secp256k1" | "ECDSA-secp256r1" | 
            "RSA-PSS" | "RSA-PKCS1v15" |
            "RSA-PSS-2048" | "RSA-PSS-3072" | "RSA-PSS-4096" |
            "RSA-PKCS1v15-2048" | "RSA-PKCS1v15-3072" | "RSA-PKCS1v15-4096"
        )
    /// Get recommended algorithms for different use cases
    pub fn get_recommended_algorithm(use_case: &str) -> &'static str {
        match use_case {
            _ => "Ed25519", // Default recommendation
        }
    }
}
