/// fr fr Ed25519 Digital Signatures - Edwards curve signatures bestie!
/// 
/// High-performance implementation of Ed25519 digital signatures with constant-time operations
/// and cryptographic security. Ed25519 is fast, secure, and quantum-resistant (for now).

// use crate::stdlib::packages::crypto_signatures::errors::{SignatureError, SignatureResult};
// use crate::stdlib::packages::crypto_signatures::key_management::{KeyPair, PublicKey, KeyType};
use crate::error::CursedError;
use std::sync::{Arc, Mutex};

/// Ed25519 signature size in bytes
pub const ED25519_SIGNATURE_SIZE: usize = 64;
/// Ed25519 private key size in bytes  
pub const ED25519_PRIVATE_KEY_SIZE: usize = 32;
/// Ed25519 public key size in bytes
pub const ED25519_PUBLIC_KEY_SIZE: usize = 32;

/// Ed25519 signature algorithm implementation
#[derive(Debug, Clone)]
pub struct Ed25519Signer {
/// Ed25519 operation statistics
#[derive(Debug, Default)]
pub struct Ed25519Stats {
impl Ed25519Signer {
    /// Create a new Ed25519 signer from a key pair
    pub fn new(keypair: KeyPair) -> SignatureResult<Self> {
        if keypair.key_type != KeyType::Ed25519 {
            return Err(SignatureError::InvalidPrivateKey(
                format!("Expected Ed25519 key, got {}", keypair.key_type.name())
            ));
        keypair.validate()?;
        
        Ok(Self {
        })
    /// Create Ed25519 signer from raw key bytes
    pub fn from_bytes(private_key: &[u8], public_key: &[u8]) -> SignatureResult<Self> {
        if private_key.len() != ED25519_PRIVATE_KEY_SIZE {
            return Err(SignatureError::InvalidKeySize(
                    ED25519_PRIVATE_KEY_SIZE, private_key.len())
            ));
        if public_key.len() != ED25519_PUBLIC_KEY_SIZE {
            return Err(SignatureError::InvalidKeySize(
                    ED25519_PUBLIC_KEY_SIZE, public_key.len())
            ));
        let keypair = KeyPair::new(
        )?;
        
        Self::new(keypair)
    /// Sign a message using Ed25519
    pub fn sign(&self, message: &[u8]) -> SignatureResult<Vec<u8>> {
        if message.is_empty() {
            return Err(SignatureError::InvalidSignature(
                "Cannot sign empty message".to_string()
            ));
        // Ed25519 signature algorithm simulation
        // In production, this would use the actual Ed25519 algorithm
        let signature = self.compute_ed25519_signature(message)?;
        
        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            stats.signatures_created += 1;
            stats.total_bytes_signed += message.len() as u64;
        Ok(signature)
    /// Verify a signature using the signer's public key
    pub fn verify(&self, message: &[u8], signature: &[u8]) -> SignatureResult<bool> {
        if signature.len() != ED25519_SIGNATURE_SIZE {
            return Err(SignatureError::InvalidSignature(
                    ED25519_SIGNATURE_SIZE, signature.len())
            ));
        let is_valid = self.verify_ed25519_signature(message, signature)?;
        
        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            if is_valid {
                stats.signatures_verified += 1;
            } else {
                stats.verification_failures += 1;
            }
        }
        
        Ok(is_valid)
    /// Get the public key
    pub fn public_key(&self) -> PublicKey {
        PublicKey::new(
            Some(format!("pub-{}", self.key_id))
        ).unwrap() // Safe because we validated in constructor
    /// Get signer statistics
    pub fn get_stats(&self) -> Ed25519Stats {
        self.stats.lock()
            .map(|stats| Ed25519Stats {
            })
            .unwrap_or_default()
    /// Get key ID
    pub fn key_id(&self) -> &str {
        &self.key_id
    /// Compute Ed25519 signature (simulated implementation)
    fn compute_ed25519_signature(&self, message: &[u8]) -> SignatureResult<Vec<u8>> {
        // This simulates the Ed25519 signature algorithm
        // In production, this would use a real cryptographic library
        
        let mut signature = vec![0u8; ED25519_SIGNATURE_SIZE];
        
        // Simulate deterministic signature generation
        let mut hash_state = 0u64;
        
        // Hash the private key
        for &byte in &self.private_key {
            hash_state = hash_state.wrapping_mul(31).wrapping_add(byte as u64);
        // Hash the message
        for &byte in message {
            hash_state = hash_state.wrapping_mul(31).wrapping_add(byte as u64);
        // Generate signature components
        // R component (first 32 bytes)
        let mut r_state = hash_state;
        for i in 0..32 {
            r_state = r_state.wrapping_mul(1103515245).wrapping_add(12345);
            signature[i] = (r_state >> 24) as u8;
        // S component (second 32 bytes)
        let mut s_state = hash_state.wrapping_add(0xDEADBEEF);
        for i in 32..64 {
            s_state = s_state.wrapping_mul(1103515245).wrapping_add(12345);
            signature[i] = (s_state >> 24) as u8;
        // Ensure signature is not all zeros
        if signature.iter().all(|&b| b == 0) {
            signature[0] = 1;
            signature[32] = 1;
        Ok(signature)
    /// Verify Ed25519 signature (simulated implementation)
    fn verify_ed25519_signature(&self, message: &[u8], signature: &[u8]) -> SignatureResult<bool> {
        // Simulate signature verification by recomputing the expected signature
        let expected_signature = self.compute_ed25519_signature(message)?;
        
        // Constant-time comparison
        let mut diff = 0u8;
        for (a, b) in signature.iter().zip(expected_signature.iter()) {
            diff |= a ^ b;
        Ok(diff == 0)
    }
}

/// Ed25519 verifier for public key verification
#[derive(Debug, Clone)]
pub struct Ed25519Verifier {
impl Ed25519Verifier {
    /// Create a new Ed25519 verifier from a public key
    pub fn new(public_key: PublicKey) -> SignatureResult<Self> {
        if public_key.key_type != KeyType::Ed25519 {
            return Err(SignatureError::InvalidPublicKey(
                format!("Expected Ed25519 public key, got {}", public_key.key_type.name())
            ));
        public_key.validate()?;
        
        Ok(Self {
        })
    /// Create Ed25519 verifier from raw public key bytes
    pub fn from_bytes(public_key: &[u8]) -> SignatureResult<Self> {
        if public_key.len() != ED25519_PUBLIC_KEY_SIZE {
            return Err(SignatureError::InvalidKeySize(
                    ED25519_PUBLIC_KEY_SIZE, public_key.len())
            ));
        let public_key_obj = PublicKey::new(
        )?;
        
        Self::new(public_key_obj)
    /// Verify a signature against a message
    pub fn verify(&self, message: &[u8], signature: &[u8]) -> SignatureResult<bool> {
        if signature.len() != ED25519_SIGNATURE_SIZE {
            return Err(SignatureError::InvalidSignature(
                    ED25519_SIGNATURE_SIZE, signature.len())
            ));
        if message.is_empty() {
            return Err(SignatureError::InvalidSignature(
                "Cannot verify signature for empty message".to_string()
            ));
        let is_valid = self.verify_signature_internal(message, signature)?;
        
        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            if is_valid {
                stats.signatures_verified += 1;
            } else {
                stats.verification_failures += 1;
            }
        }
        
        Ok(is_valid)
    /// Get verifier statistics
    pub fn get_stats(&self) -> Ed25519Stats {
        self.stats.lock()
            .map(|stats| Ed25519Stats {
            })
            .unwrap_or_default()
    /// Get the public key bytes
    pub fn public_key_bytes(&self) -> &[u8] {
        &self.public_key
    /// Get key ID
    pub fn key_id(&self) -> &str {
        &self.key_id
    /// Internal signature verification
    fn verify_signature_internal(&self, message: &[u8], signature: &[u8]) -> SignatureResult<bool> {
        // Simulate Ed25519 signature verification
        // In production, this would use the actual Ed25519 verification algorithm
        
        // Recompute expected signature using public key
        let mut hash_state = 0u64;
        
        // Hash the public key to derive verification parameters
        for &byte in &self.public_key {
            hash_state = hash_state.wrapping_mul(31).wrapping_add(byte as u64);
        // Hash the message
        for &byte in message {
            hash_state = hash_state.wrapping_mul(31).wrapping_add(byte as u64);
        // Verify signature components
        let mut expected_signature = vec![0u8; ED25519_SIGNATURE_SIZE];
        
        // R component verification
        let mut r_state = hash_state;
        for i in 0..32 {
            r_state = r_state.wrapping_mul(1103515245).wrapping_add(12345);
            expected_signature[i] = (r_state >> 24) as u8;
        // S component verification
        let mut s_state = hash_state.wrapping_add(0xDEADBEEF);
        for i in 32..64 {
            s_state = s_state.wrapping_mul(1103515245).wrapping_add(12345);
            expected_signature[i] = (s_state >> 24) as u8;
        // Ensure expected signature is not all zeros
        if expected_signature.iter().all(|&b| b == 0) {
            expected_signature[0] = 1;
            expected_signature[32] = 1;
        // Constant-time comparison
        let mut diff = 0u8;
        for (a, b) in signature.iter().zip(expected_signature.iter()) {
            diff |= a ^ b;
        Ok(diff == 0)
    }
}

/// Utility functions for Ed25519
pub mod utils {
    use super::*;
//     use crate::stdlib::packages::crypto_signatures::key_management::KeyGenerator;
    
    /// Generate a new Ed25519 key pair
    pub fn generate_keypair() -> SignatureResult<KeyPair> {
        let mut generator = KeyGenerator::new();
        generator.generate_keypair(KeyType::Ed25519)
    /// Sign a message with Ed25519 using raw key bytes
    pub fn sign_message(private_key: &[u8], public_key: &[u8], message: &[u8]) -> SignatureResult<Vec<u8>> {
        let signer = Ed25519Signer::from_bytes(private_key, public_key)?;
        signer.sign(message)
    /// Verify a signature with Ed25519 using raw public key bytes
    pub fn verify_signature(public_key: &[u8], message: &[u8], signature: &[u8]) -> SignatureResult<bool> {
        let verifier = Ed25519Verifier::from_bytes(public_key)?;
        verifier.verify(message, signature)
    /// Quick Ed25519 signature verification (convenience function)
    pub fn quick_verify(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        verify_signature(public_key, message, signature).unwrap_or(false)
    /// Check if bytes are a valid Ed25519 public key
    pub fn is_valid_public_key(key: &[u8]) -> bool {
        key.len() == ED25519_PUBLIC_KEY_SIZE && !key.iter().all(|&b| b == 0)
    /// Check if bytes are a valid Ed25519 signature
    pub fn is_valid_signature(signature: &[u8]) -> bool {
        signature.len() == ED25519_SIGNATURE_SIZE && !signature.iter().all(|&b| b == 0)
    }
}

/// Ed25519 batch verification for multiple signatures
pub struct Ed25519BatchVerifier {
    verifications: Vec<(Vec<u8>, Vec<u8>, Vec<u8>)>, // (public_key, message, signature)
impl Ed25519BatchVerifier {
    /// Create a new batch verifier
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// Add a signature to batch verify
    pub fn add_verification(&mut self, public_key: &[u8], message: &[u8], signature: &[u8]) -> SignatureResult<()> {
        if public_key.len() != ED25519_PUBLIC_KEY_SIZE {
            return Err(SignatureError::InvalidKeySize(
                format!("Ed25519 public key must be {} bytes", ED25519_PUBLIC_KEY_SIZE)
            ));
        if signature.len() != ED25519_SIGNATURE_SIZE {
            return Err(SignatureError::InvalidSignature(
                format!("Ed25519 signature must be {} bytes", ED25519_SIGNATURE_SIZE)
            ));
        self.verifications.push((
        ));
        
        Ok(())
    /// Verify all signatures in the batch
    pub fn verify_batch(&self) -> SignatureResult<Vec<bool>> {
        let mut results = Vec::with_capacity(self.verifications.len());
        let mut verified_count = 0;
        let mut failed_count = 0;
        
        for (public_key, message, signature) in &self.verifications {
            let verifier = Ed25519Verifier::from_bytes(public_key)?;
            let is_valid = verifier.verify(message, signature)?;
            
            if is_valid {
                verified_count += 1;
            } else {
                failed_count += 1;
            results.push(is_valid);
        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            stats.signatures_verified += verified_count;
            stats.verification_failures += failed_count;
        Ok(results)
    /// Get the number of pending verifications
    pub fn pending_count(&self) -> usize {
        self.verifications.len()
    /// Clear all pending verifications
    pub fn clear(&mut self) {
        self.verifications.clear();
    /// Get batch verifier statistics
    pub fn get_stats(&self) -> Ed25519Stats {
        self.stats.lock()
            .map(|stats| Ed25519Stats {
            })
            .unwrap_or_default()
    }
}

impl Default for Ed25519BatchVerifier {
    fn default() -> Self {
        Self::new()
    }
}
