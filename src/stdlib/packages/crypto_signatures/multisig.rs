/// fr fr Multi-Signature Support - Threshold signatures bestie!
/// 
/// Implementation of multi-signature schemes that require multiple signatures to authorize
/// a transaction or message. Supports threshold signatures (m-of-n) with aggregation.

// use crate::stdlib::packages::crypto_signatures::errors::{SignatureError, SignatureResult};
// use crate::stdlib::packages::crypto_signatures::key_management::{KeyPair, PublicKey, KeyType};
// use crate::stdlib::packages::crypto_signatures::ed25519::{Ed25519Signer, Ed25519Verifier};
// use crate::stdlib::packages::crypto_signatures::ecdsa::{EcdsaSigner, EcdsaVerifier, EcdsaCurve};
use crate::error::CursedError;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

/// Multi-signature scheme types
#[derive(Debug, Clone, PartialEq)]
pub enum MultiSigScheme {
    /// Simple threshold signatures - collect individual signatures
    /// Aggregated signatures - combine signatures into single signature
    /// Schnorr-style multi-signatures (simulated)
impl MultiSigScheme {
    /// Get scheme name as string
    pub fn name(&self) -> &'static str {
        match self {
        }
    }
/// Multi-signature configuration
#[derive(Debug, Clone)]
pub struct MultiSigConfig {
    /// Required number of signatures (threshold)
    /// Total number of possible signers
    /// Multi-signature scheme to use
    /// Algorithm for individual signatures
impl MultiSigConfig {
    /// Create a new multi-signature configuration
    pub fn new(
        signature_algorithm: MultiSigAlgorithm
    ) -> SignatureResult<Self> {
        if threshold == 0 {
            return Err(SignatureError::InvalidMultiSigConfig(
                "Threshold must be greater than 0".to_string()
            ));
        if threshold > total_signers {
            return Err(SignatureError::InvalidMultiSigConfig(
                format!("Threshold {} cannot exceed total signers {}", threshold, total_signers)
            ));
        if total_signers > 100 {
            return Err(SignatureError::InvalidMultiSigConfig(
                "Cannot support more than 100 signers".to_string()
            ));
        Ok(Self {
        })
    /// Validate the configuration
    pub fn validate(&self) -> SignatureResult<()> {
        if self.threshold == 0 || self.threshold > self.total_signers {
            return Err(SignatureError::InvalidMultiSigConfig(
                "Invalid threshold configuration".to_string()
            ));
        }
        Ok(())
    /// Check if this is a simple majority configuration
    pub fn is_majority(&self) -> bool {
        self.threshold > self.total_signers / 2
    /// Check if this requires unanimous consent
    pub fn is_unanimous(&self) -> bool {
        self.threshold == self.total_signers
    }
}

/// Supported algorithms for multi-signatures
#[derive(Debug, Clone, PartialEq)]
pub enum MultiSigAlgorithm {
impl MultiSigAlgorithm {
    /// Get algorithm name
    pub fn name(&self) -> &'static str {
        match self {
        }
    }
    
    /// Get expected signature size
    pub fn signature_size(&self) -> usize {
        match self {
        }
    }
/// Individual signature in a multi-signature
#[derive(Debug, Clone)]
pub struct IndividualSignature {
    /// Signer's public key identifier
    /// The actual signature bytes
    /// Timestamp when signature was created
    /// Optional metadata
impl IndividualSignature {
    /// Create a new individual signature
    pub fn new(signer_id: String, signature: Vec<u8>) -> Self {
        Self {
        }
    }
    
    /// Add metadata to the signature
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    /// Validate the signature format
    pub fn validate(&self, algorithm: &MultiSigAlgorithm) -> SignatureResult<()> {
        if self.signature.len() != algorithm.signature_size() {
            return Err(SignatureError::InvalidSignature(
                format!("Invalid signature size for {}", algorithm.name())
            ));
        if self.signer_id.is_empty() {
            return Err(SignatureError::InvalidSignature(
                "Signer ID cannot be empty".to_string()
            ));
        Ok(())
    }
}

/// Multi-signature that combines multiple individual signatures
#[derive(Debug, Clone)]
pub struct MultiSignature {
    /// Configuration for this multi-signature
    /// Individual signatures collected
    /// Message that was signed (hash)
    /// Aggregated signature (for aggregated schemes)
    /// Creation timestamp
    /// Multi-signature ID
impl MultiSignature {
    /// Create a new multi-signature
    pub fn new(config: MultiSigConfig, message_hash: Vec<u8>) -> SignatureResult<Self> {
        config.validate()?;
        
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        );
        
        Ok(Self {
        })
    /// Add an individual signature
    pub fn add_signature(&mut self, signature: IndividualSignature) -> SignatureResult<()> {
        signature.validate(&self.config.signature_algorithm)?;
        
        // Check for duplicate signers
        if self.signatures.iter().any(|s| s.signer_id == signature.signer_id) {
            return Err(SignatureError::InvalidSignature(
                format!("Signer '{}' has already signed", signature.signer_id)
            ));
        // Check if we're at capacity
        if self.signatures.len() >= self.config.total_signers {
            return Err(SignatureError::InvalidMultiSigConfig(
                "Cannot add more signatures than total signers".to_string()
            ));
        self.signatures.push(signature);
        
        // If we have enough signatures and using aggregated scheme, create aggregated signature
        if self.is_complete() && self.config.scheme == MultiSigScheme::Aggregated {
            self.create_aggregated_signature()?;
        Ok(())
    /// Check if the multi-signature is complete (threshold reached)
    pub fn is_complete(&self) -> bool {
        self.signatures.len() >= self.config.threshold
    /// Check if the multi-signature is valid
    pub fn is_valid(&self) -> bool {
        self.is_complete() && self.signatures.len() <= self.config.total_signers
    /// Get the completion percentage
    pub fn completion_percentage(&self) -> f64 {
        (self.signatures.len() as f64 / self.config.threshold as f64 * 100.0).min(100.0)
    /// Get remaining signatures needed
    pub fn remaining_needed(&self) -> usize {
        if self.signatures.len() >= self.config.threshold {
            0
        } else {
            self.config.threshold - self.signatures.len()
        }
    }
    
    /// Get list of signer IDs who have signed
    pub fn get_signers(&self) -> Vec<String> {
        self.signatures.iter().map(|s| s.signer_id.clone()).collect()
    /// Create aggregated signature from individual signatures
    fn create_aggregated_signature(&mut self) -> SignatureResult<()> {
        if self.signatures.is_empty() {
            return Err(SignatureError::InvalidSignature(
                "No signatures to aggregate".to_string()
            ));
        let signature_size = self.config.signature_algorithm.signature_size();
        let mut aggregated = vec![0u8; signature_size];
        
        // Simple aggregation: XOR all signatures (in production, use proper aggregation)
        for signature in &self.signatures {
            for (i, &byte) in signature.signature.iter().enumerate() {
                if i < aggregated.len() {
                    aggregated[i] ^= byte;
                }
            }
        // Ensure aggregated signature is not all zeros
        if aggregated.iter().all(|&b| b == 0) {
            aggregated[0] = 1;
            aggregated[signature_size - 1] = self.signatures.len() as u8;
        self.aggregated_signature = Some(aggregated);
        Ok(())
    /// Get the final signature for verification
    pub fn get_signature(&self) -> SignatureResult<Vec<u8>> {
        if !self.is_complete() {
            return Err(SignatureError::ThresholdNotMet(
                format!("Need {} signatures, have {}", self.config.threshold, self.signatures.len())
            ));
        match self.config.scheme {
            MultiSigScheme::Threshold => {
                // For threshold, return concatenated signatures
                let mut combined = Vec::new();
                for signature in &self.signatures[..self.config.threshold] {
                    combined.extend_from_slice(&signature.signature);
                }
                Ok(combined)
            MultiSigScheme::Aggregated | MultiSigScheme::Schnorr => {
                // Return aggregated signature
                self.aggregated_signature.clone()
                    .ok_or_else(|| SignatureError::Internal(
                        "Aggregated signature not created".to_string()
                    ))
        }
    }
/// Multi-signature signer that manages multiple signers
#[derive(Debug)]
pub struct MultiSigSigner {
    /// Configuration
    /// Signer information (signer_id -> public_key)
    /// Statistics
/// Multi-signature operation statistics
#[derive(Debug, Default)]
pub struct MultiSigStats {
impl MultiSigSigner {
    /// Create a new multi-signature signer
    pub fn new(config: MultiSigConfig) -> SignatureResult<Self> {
        config.validate()?;
        
        Ok(Self {
        })
    /// Add a signer to the multi-signature scheme
    pub fn add_signer(&mut self, signer_id: String, public_key: PublicKey) -> SignatureResult<()> {
        if self.signers.len() >= self.config.total_signers {
            return Err(SignatureError::InvalidMultiSigConfig(
                "Cannot add more signers than configured total".to_string()
            ));
        // Validate public key matches algorithm
        let expected_key_type = match self.config.signature_algorithm {
        
        if public_key.key_type != expected_key_type {
            return Err(SignatureError::InvalidPublicKey(
                    public_key.key_type.name(), self.config.signature_algorithm.name())
            ));
        public_key.validate()?;
        self.signers.insert(signer_id, public_key);
        Ok(())
    /// Create a new multi-signature for a message
    pub fn create_multisig(&self, message: &[u8]) -> SignatureResult<MultiSignature> {
        if self.signers.len() < self.config.threshold {
            return Err(SignatureError::InvalidMultiSigConfig(
                    self.config.threshold, self.signers.len())
            ));
        // Hash the message
        let message_hash = self.hash_message(message);
        
        let multisig = MultiSignature::new(self.config.clone(), message_hash)?;
        
        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            stats.multisigs_created += 1;
        Ok(multisig)
    /// Sign a message with a specific signer's key pair
    pub fn sign_with_keypair(
        message: &[u8]
    ) -> SignatureResult<()> {
        // Verify signer is registered
        let public_key = self.signers.get(signer_id)
            .ok_or_else(|| SignatureError::InvalidPrivateKey(
                format!("Signer '{}' not registered", signer_id)
            ))?;
        
        // Verify keypair matches registered public key
        if keypair.public_key != public_key.key_data {
            return Err(SignatureError::InvalidPrivateKey(
                "Key pair doesn't match registered public key".to_string()
            ));
        // Create signature based on algorithm
        let signature = match self.config.signature_algorithm {
            MultiSigAlgorithm::Ed25519 => {
                let signer = Ed25519Signer::new(keypair.clone())?;
                signer.sign(message)?
            MultiSigAlgorithm::EcdsaSecp256k1 => {
                let mut signer = EcdsaSigner::new(keypair.clone())?;
                signer.sign(message)?
            MultiSigAlgorithm::EcdsaSecp256r1 => {
                let mut signer = EcdsaSigner::new(keypair.clone())?;
                signer.sign(message)?
        
        let individual_sig = IndividualSignature::new(signer_id.to_string(), signature);
        multisig.add_signature(individual_sig)?;
        
        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            stats.signatures_added += 1;
            if multisig.is_complete() {
                stats.completed_multisigs += 1;
            }
        }
        
        Ok(())
    /// Verify a completed multi-signature
    pub fn verify_multisig(
        message: &[u8]
    ) -> SignatureResult<bool> {
        if !multisig.is_complete() {
            return Err(SignatureError::ThresholdNotMet(
                format!("Multi-signature not complete: {}/{}", 
                    multisig.signatures.len(), multisig.config.threshold)
            ));
        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            stats.verification_attempts += 1;
        let is_valid = match multisig.config.scheme {
        
        if is_valid {
            if let Ok(mut stats) = self.stats.lock() {
                stats.verification_successes += 1;
            }
        }
        
        Ok(is_valid)
    /// Get multi-signature statistics
    pub fn get_stats(&self) -> MultiSigStats {
        self.stats.lock()
            .map(|stats| MultiSigStats {
            })
            .unwrap_or_default()
    /// Get registered signers
    pub fn get_signers(&self) -> Vec<String> {
        self.signers.keys().cloned().collect()
    /// Get configuration
    pub fn config(&self) -> &MultiSigConfig {
        &self.config
    /// Verify threshold multi-signature
    fn verify_threshold(&self, multisig: &MultiSignature, message: &[u8]) -> SignatureResult<bool> {
        let required_signatures = multisig.config.threshold.min(multisig.signatures.len());
        let mut verified_count = 0;
        
        for signature in &multisig.signatures[..required_signatures] {
            let public_key = self.signers.get(&signature.signer_id)
                .ok_or_else(|| SignatureError::InvalidSignature(
                    format!("Unknown signer: {}", signature.signer_id)
                ))?;
            
            let is_valid = match self.config.signature_algorithm {
                MultiSigAlgorithm::Ed25519 => {
                    let verifier = Ed25519Verifier::new(public_key.clone())?;
                    verifier.verify(message, &signature.signature)?
                MultiSigAlgorithm::EcdsaSecp256k1 => {
                    let verifier = EcdsaVerifier::new(public_key.clone())?;
                    verifier.verify(message, &signature.signature)?
                MultiSigAlgorithm::EcdsaSecp256r1 => {
                    let verifier = EcdsaVerifier::new(public_key.clone())?;
                    verifier.verify(message, &signature.signature)?
            
            if is_valid {
                verified_count += 1;
            }
        }
        
        Ok(verified_count >= multisig.config.threshold)
    /// Verify aggregated multi-signature
    fn verify_aggregated(&self, multisig: &MultiSignature, message: &[u8]) -> SignatureResult<bool> {
        let aggregated_sig = multisig.aggregated_signature.as_ref()
            .ok_or_else(|| SignatureError::InvalidSignature(
                "No aggregated signature available".to_string()
            ))?;
        
        // For simulation, we reconstruct the expected aggregated signature
        let signature_size = self.config.signature_algorithm.signature_size();
        let mut expected_aggregated = vec![0u8; signature_size];
        
        // Recreate signatures for comparison
        for signature in &multisig.signatures[..multisig.config.threshold] {
            for (i, &byte) in signature.signature.iter().enumerate() {
                if i < expected_aggregated.len() {
                    expected_aggregated[i] ^= byte;
                }
            }
        if expected_aggregated.iter().all(|&b| b == 0) {
            expected_aggregated[0] = 1;
            expected_aggregated[signature_size - 1] = multisig.signatures.len() as u8;
        // Constant-time comparison
        let mut diff = 0u8;
        for (a, b) in aggregated_sig.iter().zip(expected_aggregated.iter()) {
            diff |= a ^ b;
        Ok(diff == 0)
    /// Verify Schnorr multi-signature (simulated)
    fn verify_schnorr(&self, multisig: &MultiSignature, message: &[u8]) -> SignatureResult<bool> {
        // Schnorr signatures have special aggregation properties
        // For simulation, we use a simplified verification
        self.verify_aggregated(multisig, message)
    /// Hash a message
    fn hash_message(&self, message: &[u8]) -> Vec<u8> {
        let mut hash = vec![0u8; 32]; // SHA-256 size
        let mut state = 0x6a09e667u64;
        
        for &byte in message {
            state = state.wrapping_mul(31).wrapping_add(byte as u64);
        for i in 0..32 {
            state = state.wrapping_mul(1103515245).wrapping_add(12345);
            hash[i] = (state >> 24) as u8;
        hash
    }
}

/// Utility functions for multi-signatures
pub mod utils {
    use super::*;
//     use crate::stdlib::packages::crypto_signatures::key_management::KeyGenerator;
    
    /// Create a simple threshold multi-signature setup
    pub fn create_threshold_setup(
        algorithm: MultiSigAlgorithm
    ) -> SignatureResult<(MultiSigSigner, Vec<KeyPair>)> {
        let config = MultiSigConfig::new(threshold, total_signers, MultiSigScheme::Threshold, algorithm)?;
        let mut multisig_signer = MultiSigSigner::new(config)?;
        
        let mut keypairs = Vec::new();
        let mut generator = KeyGenerator::new();
        
        // Generate key pairs for all signers
        for i in 0..total_signers {
            let key_type = match algorithm {
            
            let keypair = generator.generate_keypair(key_type)?;
            let public_key = PublicKey::from_keypair(&keypair);
            let signer_id = format!("signer-{}", i + 1);
            
            multisig_signer.add_signer(signer_id, public_key)?;
            keypairs.push(keypair);
        Ok((multisig_signer, keypairs))
    /// Quick multi-signature verification
    pub fn quick_verify_multisig(
        message: &[u8]
    ) -> bool {
        let mut signer = match MultiSigSigner::new(config.clone()) {
        
        // Add all signers
        for (id, public_key) in signers {
            if signer.add_signer(id.clone(), public_key.clone()).is_err() {
                return false;
            }
        }
        
        signer.verify_multisig(multisig, message).unwrap_or(false)
    /// Check if a multi-signature configuration is valid
    pub fn is_valid_config(threshold: usize, total_signers: usize) -> bool {
        threshold > 0 && threshold <= total_signers && total_signers <= 100
    /// Calculate the security level of a multi-signature configuration
    pub fn calculate_security_level(config: &MultiSigConfig) -> f64 {
        // Simple metric: ratio of threshold to total signers
        config.threshold as f64 / config.total_signers as f64
    }
}
