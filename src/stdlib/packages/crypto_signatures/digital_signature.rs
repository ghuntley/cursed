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
}

/// Universal signer that can handle any supported signature algorithm
#[derive(Debug)]
pub enum UniversalSigner {
    Ed25519(Ed25519Signer),
    Ecdsa(EcdsaSigner),
    Rsa(RsaSigner),
}

impl UniversalSigner {
    /// Create a universal signer from a key pair
    pub fn new(keypair: KeyPair) -> SignatureResult<Self> {
        match keypair.key_type {
            KeyType::Ed25519 => {
                let signer = Ed25519Signer::new(keypair)?;
                Ok(UniversalSigner::Ed25519(signer))
            },
            KeyType::EcdsaSecp256k1 | KeyType::EcdsaSecp256r1 => {
                let signer = EcdsaSigner::new(keypair)?;
                Ok(UniversalSigner::Ecdsa(signer))
            },
            KeyType::RsaPss2048 | KeyType::RsaPss3072 | KeyType::RsaPss4096 => {
                let signer = RsaSigner::new(keypair, RsaSignatureScheme::Pss, RsaHashAlgorithm::Sha256)?;
                Ok(UniversalSigner::Rsa(signer))
            },
            KeyType::RsaPkcs1v15_2048 | KeyType::RsaPkcs1v15_3072 | KeyType::RsaPkcs1v15_4096 => {
                let signer = RsaSigner::new(keypair, RsaSignatureScheme::Pkcs1v15, RsaHashAlgorithm::Sha256)?;
                Ok(UniversalSigner::Rsa(signer))
            },
        }
    }
    
    /// Create a universal signer with specific RSA parameters
    pub fn new_rsa(
        keypair: KeyPair, 
        scheme: RsaSignatureScheme, 
        hash_algorithm: RsaHashAlgorithm
    ) -> SignatureResult<Self> {
        let signer = RsaSigner::new(keypair, scheme, hash_algorithm)?;
        Ok(UniversalSigner::Rsa(signer))
    }
}

impl DigitalSignature for UniversalSigner {
    fn sign(&self, message: &[u8]) -> SignatureResult<Vec<u8>> {
        match self {
            UniversalSigner::Ed25519(signer) => signer.sign(message),
            UniversalSigner::Ecdsa(signer) => {
                // ECDSA signer needs to be mutable for nonce generation
                // This is a limitation of the current design
                Err(SignatureError::Internal(
                    "ECDSA signing requires mutable reference".to_string()
                ))
            },
            UniversalSigner::Rsa(signer) => signer.sign(message),
        }
    }
    
    fn verify(&self, message: &[u8], signature: &[u8]) -> SignatureResult<bool> {
        match self {
            UniversalSigner::Ed25519(signer) => signer.verify(message, signature),
            UniversalSigner::Ecdsa(signer) => signer.verify(message, signature),
            UniversalSigner::Rsa(signer) => signer.verify(message, signature),
        }
    }
    
    fn algorithm_name(&self) -> &str {
        match self {
            UniversalSigner::Ed25519(_) => "Ed25519",
            UniversalSigner::Ecdsa(signer) => signer.curve().name(),
            UniversalSigner::Rsa(signer) => signer.scheme().name(),
        }
    }
    
    fn key_id(&self) -> &str {
        match self {
            UniversalSigner::Ed25519(signer) => signer.key_id(),
            UniversalSigner::Ecdsa(signer) => signer.key_id(),
            UniversalSigner::Rsa(signer) => signer.key_id(),
        }
    }
}

/// Signature operation context for tracking operations
#[derive(Debug, Clone)]
pub struct SignatureContext {
    pub algorithm: String,
    pub key_id: String,
    pub timestamp: std::time::SystemTime,
    pub message_size: usize,
    pub signature_size: usize,
}

impl SignatureContext {
    /// Create a new signature context
    pub fn new(
        algorithm: String,
        key_id: String,
        message_size: usize,
        signature_size: usize,
    ) -> Self {
        Self {
            algorithm,
            key_id,
            timestamp: std::time::SystemTime::now(),
            message_size,
            signature_size,
        }
    }
}

/// Signature manager for handling multiple signers and operations
#[derive(Debug)]
pub struct SignatureManager {
    signers: Arc<Mutex<std::collections::HashMap<String, UniversalSigner>>>,
    contexts: Arc<Mutex<Vec<SignatureContext>>>,
    stats: Arc<Mutex<SignatureManagerStats>>,
}

/// Signature manager statistics
#[derive(Debug, Default)]
pub struct SignatureManagerStats {
    pub total_signatures: u64,
    pub total_verifications: u64,
    pub successful_verifications: u64,
    pub failed_verifications: u64,
    pub by_algorithm: std::collections::HashMap<String, u64>,
}

impl SignatureManager {
    /// Create a new signature manager
    pub fn new() -> Self {
        Self {
            signers: Arc::new(Mutex::new(std::collections::HashMap::new())),
            contexts: Arc::new(Mutex::new(Vec::new())),
            stats: Arc::new(Mutex::new(SignatureManagerStats::default())),
        }
    }
    
    /// Add a signer to the manager
    pub fn add_signer(&self, signer_id: String, signer: UniversalSigner) -> SignatureResult<()> {
        let mut signers = self.signers.lock()
            .map_err(|_| SignatureError::Internal("Failed to acquire signers lock".to_string()))?;
        
        signers.insert(signer_id, signer);
        Ok(())
    }
    
    /// Add a signer from a key pair
    pub fn add_signer_from_keypair(&self, signer_id: String, keypair: KeyPair) -> SignatureResult<()> {
        let signer = UniversalSigner::new(keypair)?;
        self.add_signer(signer_id, signer)
    }
    
    /// Sign a message with a specific signer
    pub fn sign_with(&self, signer_id: &str, message: &[u8]) -> SignatureResult<Vec<u8>> {
        let signers = self.signers.lock()
            .map_err(|_| SignatureError::Internal("Failed to acquire signers lock".to_string()))?;
        
        let signer = signers.get(signer_id)
            .ok_or_else(|| SignatureError::InvalidPrivateKey(
                format!("Signer '{}' not found", signer_id)
            ))?;
        
        let signature = match signer {
            UniversalSigner::Ed25519(ed_signer) => ed_signer.sign(message)?,
            UniversalSigner::Ecdsa(_) => {
                return Err(SignatureError::Internal(
                    "ECDSA signing not supported through manager (requires mutable reference)".to_string()
                ));
            },
            UniversalSigner::Rsa(rsa_signer) => rsa_signer.sign(message)?,
        };
        
        // Record context
        let context = SignatureContext::new(
            signer.algorithm_name().to_string(),
            signer.key_id().to_string(),
            message.len(),
            signature.len(),
        );
        
        if let Ok(mut contexts) = self.contexts.lock() {
            contexts.push(context);
        }
        
        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            stats.total_signatures += 1;
            *stats.by_algorithm.entry(signer.algorithm_name().to_string()).or_insert(0) += 1;
        }
        
        Ok(signature)
    }
    
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
    }
    
    /// Get all signer IDs
    pub fn list_signers(&self) -> SignatureResult<Vec<String>> {
        let signers = self.signers.lock()
            .map_err(|_| SignatureError::Internal("Failed to acquire signers lock".to_string()))?;
        
        Ok(signers.keys().cloned().collect())
    }
    
    /// Get signature contexts (history)
    pub fn get_contexts(&self) -> Vec<SignatureContext> {
        self.contexts.lock()
            .map(|contexts| contexts.clone())
            .unwrap_or_default()
    }
    
    /// Get manager statistics
    pub fn get_stats(&self) -> SignatureManagerStats {
        self.stats.lock()
            .map(|stats| SignatureManagerStats {
                total_signatures: stats.total_signatures,
                total_verifications: stats.total_verifications,
                successful_verifications: stats.successful_verifications,
                failed_verifications: stats.failed_verifications,
                by_algorithm: stats.by_algorithm.clone(),
            })
            .unwrap_or_default()
    }
    
    /// Remove a signer
    pub fn remove_signer(&self, signer_id: &str) -> SignatureResult<bool> {
        let mut signers = self.signers.lock()
            .map_err(|_| SignatureError::Internal("Failed to acquire signers lock".to_string()))?;
        
        Ok(signers.remove(signer_id).is_some())
    }
    
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
    signer: Ed25519Signer,
}

impl Ed25519Signature {
    /// Create a new Ed25519 signature instance from a key pair
    pub fn new(keypair: KeyPair) -> SignatureResult<Self> {
        let signer = Ed25519Signer::new(keypair)?;
        Ok(Self { signer })
    }
    
    /// Create from private key bytes (for backward compatibility)
    pub fn from_private_key(private_key: Vec<u8>) -> SignatureResult<Self> {
        if private_key.len() != 32 {
            return Err(SignatureError::InvalidKeySize(
                "Ed25519 private key must be 32 bytes".to_string()
            ));
        }
        
        // Generate corresponding public key (simulated)
        let mut public_key = vec![0u8; 32];
        for (i, &byte) in private_key.iter().enumerate() {
            public_key[i] = byte.wrapping_add(i as u8 + 1);
        }
        
        let keypair = KeyPair::new(
            KeyType::Ed25519,
            private_key,
            public_key,
            None,
        )?;
        
        Self::new(keypair)
    }
}

impl DigitalSignature for Ed25519Signature {
    fn sign(&self, message: &[u8]) -> SignatureResult<Vec<u8>> {
        self.signer.sign(message)
    }
    
    fn verify(&self, message: &[u8], signature: &[u8]) -> SignatureResult<bool> {
        self.signer.verify(message, signature)
    }
    
    fn algorithm_name(&self) -> &str {
        "Ed25519"
    }
    
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
    }
    
    /// Create a signature manager with pre-generated signers
    pub fn create_test_manager(num_signers: usize) -> SignatureResult<SignatureManager> {
        let manager = SignatureManager::new();
        let mut generator = KeyGenerator::new();
        
        for i in 0..num_signers {
            let keypair = generator.generate_keypair(KeyType::Ed25519)?;
            let signer_id = format!("test-signer-{}", i + 1);
            manager.add_signer_from_keypair(signer_id, keypair)?;
        }
        
        Ok(manager)
    }
    
    /// Check if an algorithm is supported
    pub fn is_algorithm_supported(algorithm: &str) -> bool {
        matches!(algorithm, 
            "Ed25519" | "ECDSA-secp256k1" | "ECDSA-secp256r1" | 
            "RSA-PSS" | "RSA-PKCS1v15" |
            "RSA-PSS-2048" | "RSA-PSS-3072" | "RSA-PSS-4096" |
            "RSA-PKCS1v15-2048" | "RSA-PKCS1v15-3072" | "RSA-PKCS1v15-4096"
        )
    }
    
    /// Get recommended algorithms for different use cases
    pub fn get_recommended_algorithm(use_case: &str) -> &'static str {
        match use_case {
            "speed" => "Ed25519",
            "bitcoin" => "ECDSA-secp256k1", 
            "nist" => "ECDSA-secp256r1",
            "legacy" => "RSA-PKCS1v15-2048",
            "secure" => "RSA-PSS-4096",
            _ => "Ed25519", // Default recommendation
        }
    }
}
