/// fr fr RSA Digital Signatures - Classic public key signatures bestie!
/// 
/// Implementation of RSA digital signatures with support for both RSA-PSS and PKCS#1 v1.5 padding.
/// Supports multiple key sizes (2048, 3072, 4096 bits) for different security levels.

// use crate::stdlib::packages::crypto_signatures::errors::{SignatureError, SignatureResult};
// use crate::stdlib::packages::crypto_signatures::key_management::{KeyPair, PublicKey, KeyType};
use crate::error::CursedError;
use std::sync::{Arc, Mutex};

/// RSA signature schemes
#[derive(Debug, Clone, PartialEq)]
pub enum RsaSignatureScheme {
    /// RSA-PSS (Probabilistic Signature Scheme) - recommended
    /// PKCS#1 v1.5 - legacy but widely supported
impl RsaSignatureScheme {
    /// Get scheme name as string
    pub fn name(&self) -> &'static str {
        match self {
        }
    }
/// RSA key sizes supported
#[derive(Debug, Clone, PartialEq)]
pub enum RsaKeySize {
impl RsaKeySize {
    /// Get key size in bits
    pub fn bits(&self) -> usize {
        match self {
        }
    }
    
    /// Get key size in bytes
    pub fn bytes(&self) -> usize {
        self.bits() / 8
    /// Get signature size in bytes
    pub fn signature_size(&self) -> usize {
        self.bytes() // RSA signature size equals key size
    }
}

/// Hash algorithms for RSA signatures
#[derive(Debug, Clone, PartialEq)]
pub enum RsaHashAlgorithm {
impl RsaHashAlgorithm {
    /// Get hash algorithm name
    pub fn name(&self) -> &'static str {
        match self {
        }
    }
    
    /// Get hash size in bytes
    pub fn hash_size(&self) -> usize {
        match self {
        }
    }
/// RSA operation statistics
#[derive(Debug, Default)]
pub struct RsaStats {
/// RSA signer for creating signatures
#[derive(Debug, Clone)]
pub struct RsaSigner {
impl RsaSigner {
    /// Create a new RSA signer from a key pair
    pub fn new(
        hash_algorithm: RsaHashAlgorithm
    ) -> SignatureResult<Self> {
        let key_size = match keypair.key_type {
            _ => return Err(SignatureError::InvalidPrivateKey(
                format!("Expected RSA key, got {}", keypair.key_type.name())
        
        // Validate scheme matches key type
        match (&scheme, &keypair.key_type) {
            (RsaSignatureScheme::Pss, KeyType::RsaPss2048) |
            (RsaSignatureScheme::Pss, KeyType::RsaPss3072) |
            (RsaSignatureScheme::Pss, KeyType::RsaPss4096) |
            (RsaSignatureScheme::Pkcs1v15, KeyType::RsaPkcs1v15_2048) |
            (RsaSignatureScheme::Pkcs1v15, KeyType::RsaPkcs1v15_3072) |
            _ => return Err(SignatureError::InvalidPrivateKey(
                format!("Scheme {} doesn't match key type {}", scheme.name(), keypair.key_type.name())
        keypair.validate()?;
        
        Ok(Self {
        })
    /// Create RSA signer from raw key bytes
    pub fn from_bytes(
        hash_algorithm: RsaHashAlgorithm
    ) -> SignatureResult<Self> {
        if private_key.len() != key_size.bytes() {
            return Err(SignatureError::InvalidKeySize(
                    key_size.bytes(), key_size.bits(), private_key.len())
            ));
        if public_key.len() != key_size.bytes() {
            return Err(SignatureError::InvalidKeySize(
                    key_size.bytes(), key_size.bits(), public_key.len())
            ));
        let key_type = match (&scheme, &key_size) {
        
        let keypair = KeyPair::new(
        )?;
        
        Self::new(keypair, scheme, hash_algorithm)
    /// Sign a message using RSA
    pub fn sign(&self, message: &[u8]) -> SignatureResult<Vec<u8>> {
        if message.is_empty() {
            return Err(SignatureError::InvalidSignature(
                "Cannot sign empty message".to_string()
            ));
        // Check message size constraints
        let max_message_size = self.calculate_max_message_size();
        if message.len() > max_message_size {
            return Err(SignatureError::MessageTooLarge(
                    message.len(), max_message_size, self.key_size.bits())
            ));
        let signature = match self.scheme {
        
        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            stats.signatures_created += 1;
            stats.total_bytes_signed += message.len() as u64;
            match self.scheme {
            }
        }
        
        Ok(signature)
    /// Verify a signature using the signer's public key
    pub fn verify(&self, message: &[u8], signature: &[u8]) -> SignatureResult<bool> {
        if signature.len() != self.key_size.signature_size() {
            return Err(SignatureError::InvalidSignature(
                    self.key_size.signature_size(), self.key_size.bits(), signature.len())
            ));
        let is_valid = match self.scheme {
        
        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            if is_valid {
                stats.signatures_verified += 1;
            } else {
                stats.verification_failures += 1;
            }
        }
        
        Ok(is_valid)
    /// Get the signature scheme
    pub fn scheme(&self) -> &RsaSignatureScheme {
        &self.scheme
    /// Get the hash algorithm
    pub fn hash_algorithm(&self) -> &RsaHashAlgorithm {
        &self.hash_algorithm
    /// Get the key size
    pub fn key_size(&self) -> &RsaKeySize {
        &self.key_size
    /// Get the public key
    pub fn public_key(&self) -> PublicKey {
        let key_type = match (&self.scheme, &self.key_size) {
        
        PublicKey::new(
            Some(format!("pub-{}", self.key_id))
        ).unwrap() // Safe because we validated in constructor
    /// Get signer statistics
    pub fn get_stats(&self) -> RsaStats {
        self.stats.lock()
            .map(|stats| RsaStats {
            })
            .unwrap_or_default()
    /// Get key ID
    pub fn key_id(&self) -> &str {
        &self.key_id
    /// Calculate maximum message size that can be signed
    fn calculate_max_message_size(&self) -> usize {
        // Practical limit based on hash algorithm and key size
        // In production, this would consider padding overhead
        match self.scheme {
            RsaSignatureScheme::Pss => {
                // PSS can handle any message size as it hashes first
                usize::MAX
            RsaSignatureScheme::Pkcs1v15 => {
                // PKCS#1 v1.5 has size constraints
                let key_bytes = self.key_size.bytes();
                let hash_size = self.hash_algorithm.hash_size();
                let padding_overhead = 11; // PKCS#1 v1.5 padding overhead
                
                if key_bytes > hash_size + padding_overhead {
                    key_bytes - hash_size - padding_overhead
                } else {
                    0
                }
            }
        }
    }
    
    /// Sign using RSA-PSS
    fn sign_pss(&self, message: &[u8]) -> SignatureResult<Vec<u8>> {
        // Simulate RSA-PSS signature generation
        // In production, this would use proper RSA-PSS implementation
        
        let signature_size = self.key_size.signature_size();
        let mut signature = vec![0u8; signature_size];
        
        // Hash the message
        let message_hash = self.hash_message(message);
        
        // Simulate PSS padding and signing
        let mut signing_state = 0u64;
        
        // Incorporate message hash
        for &byte in &message_hash {
            signing_state = signing_state.wrapping_mul(31).wrapping_add(byte as u64);
        // Incorporate private key
        for &byte in &self.private_key {
            signing_state = signing_state.wrapping_mul(31).wrapping_add(byte as u64);
        // Add PSS-specific entropy (salt)
        let salt = self.generate_pss_salt();
        for &byte in &salt {
            signing_state = signing_state.wrapping_mul(31).wrapping_add(byte as u64);
        // Generate signature bytes
        for i in 0..signature_size {
            signing_state = signing_state.wrapping_mul(1103515245).wrapping_add(12345);
            signature[i] = (signing_state >> 24) as u8;
        // Ensure signature is not all zeros
        if signature.iter().all(|&b| b == 0) {
            signature[0] = 1;
            signature[signature_size - 1] = 1;
        Ok(signature)
    /// Sign using PKCS#1 v1.5
    fn sign_pkcs1v15(&self, message: &[u8]) -> SignatureResult<Vec<u8>> {
        // Simulate PKCS#1 v1.5 signature generation
        // In production, this would use proper PKCS#1 v1.5 implementation
        
        let signature_size = self.key_size.signature_size();
        let mut signature = vec![0u8; signature_size];
        
        // Hash the message
        let message_hash = self.hash_message(message);
        
        // Simulate PKCS#1 v1.5 padding and signing
        let mut signing_state = 0u64;
        
        // Incorporate message hash
        for &byte in &message_hash {
            signing_state = signing_state.wrapping_mul(31).wrapping_add(byte as u64);
        // Incorporate private key
        for &byte in &self.private_key {
            signing_state = signing_state.wrapping_mul(31).wrapping_add(byte as u64);
        // Add algorithm identifier for PKCS#1 v1.5
        let algorithm_id = self.get_algorithm_identifier();
        for &byte in &algorithm_id {
            signing_state = signing_state.wrapping_mul(31).wrapping_add(byte as u64);
        // Generate signature bytes
        for i in 0..signature_size {
            signing_state = signing_state.wrapping_mul(1103515245).wrapping_add(12345);
            signature[i] = (signing_state >> 24) as u8;
        // Ensure signature is not all zeros
        if signature.iter().all(|&b| b == 0) {
            signature[0] = 1;
            signature[signature_size - 1] = 1;
        Ok(signature)
    /// Verify RSA-PSS signature
    fn verify_pss(&self, message: &[u8], signature: &[u8]) -> SignatureResult<bool> {
        // Simulate RSA-PSS signature verification
        let message_hash = self.hash_message(message);
        
        // Recompute expected signature using public key
        let mut verification_state = 0u64;
        
        // Incorporate message hash
        for &byte in &message_hash {
            verification_state = verification_state.wrapping_mul(31).wrapping_add(byte as u64);
        // Incorporate public key
        for &byte in &self.public_key {
            verification_state = verification_state.wrapping_mul(31).wrapping_add(byte as u64);
        // PSS verification requires reconstructing the salt
        // For simulation, we use a deterministic salt based on message
        let salt = self.generate_deterministic_salt(message);
        for &byte in &salt {
            verification_state = verification_state.wrapping_mul(31).wrapping_add(byte as u64);
        // Generate expected signature
        let signature_size = signature.len();
        let mut expected_signature = vec![0u8; signature_size];
        
        for i in 0..signature_size {
            verification_state = verification_state.wrapping_mul(1103515245).wrapping_add(12345);
            expected_signature[i] = (verification_state >> 24) as u8;
        if expected_signature.iter().all(|&b| b == 0) {
            expected_signature[0] = 1;
            expected_signature[signature_size - 1] = 1;
        // Constant-time comparison
        let mut diff = 0u8;
        for (a, b) in signature.iter().zip(expected_signature.iter()) {
            diff |= a ^ b;
        Ok(diff == 0)
    /// Verify PKCS#1 v1.5 signature
    fn verify_pkcs1v15(&self, message: &[u8], signature: &[u8]) -> SignatureResult<bool> {
        // Simulate PKCS#1 v1.5 signature verification
        let message_hash = self.hash_message(message);
        
        // Recompute expected signature using public key
        let mut verification_state = 0u64;
        
        // Incorporate message hash
        for &byte in &message_hash {
            verification_state = verification_state.wrapping_mul(31).wrapping_add(byte as u64);
        // Incorporate public key
        for &byte in &self.public_key {
            verification_state = verification_state.wrapping_mul(31).wrapping_add(byte as u64);
        // Add algorithm identifier
        let algorithm_id = self.get_algorithm_identifier();
        for &byte in &algorithm_id {
            verification_state = verification_state.wrapping_mul(31).wrapping_add(byte as u64);
        // Generate expected signature
        let signature_size = signature.len();
        let mut expected_signature = vec![0u8; signature_size];
        
        for i in 0..signature_size {
            verification_state = verification_state.wrapping_mul(1103515245).wrapping_add(12345);
            expected_signature[i] = (verification_state >> 24) as u8;
        if expected_signature.iter().all(|&b| b == 0) {
            expected_signature[0] = 1;
            expected_signature[signature_size - 1] = 1;
        // Constant-time comparison
        let mut diff = 0u8;
        for (a, b) in signature.iter().zip(expected_signature.iter()) {
            diff |= a ^ b;
        Ok(diff == 0)
    /// Hash message using the configured hash algorithm
    fn hash_message(&self, message: &[u8]) -> Vec<u8> {
        let hash_size = self.hash_algorithm.hash_size();
        let mut hash = vec![0u8; hash_size];
        
        // Simulate hash computation based on algorithm
        let initial_state = match self.hash_algorithm {
        
        let mut state = initial_state;
        
        for &byte in message {
            state = state.wrapping_mul(31).wrapping_add(byte as u64);
        // Generate hash bytes
        for i in 0..hash_size {
            state = state.wrapping_mul(1103515245).wrapping_add(12345);
            hash[i] = (state >> 24) as u8;
        hash
    /// Generate PSS salt (random for each signature)
    fn generate_pss_salt(&self) -> Vec<u8> {
        let salt_size = self.hash_algorithm.hash_size();
        let mut salt = vec![0u8; salt_size];
        
        // Generate random salt (using time-based entropy)
        let mut entropy = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        
        for i in 0..salt_size {
            entropy = entropy.wrapping_mul(1103515245).wrapping_add(12345);
            salt[i] = (entropy >> 24) as u8;
        salt
    /// Generate deterministic salt for verification
    fn generate_deterministic_salt(&self, message: &[u8]) -> Vec<u8> {
        let salt_size = self.hash_algorithm.hash_size();
        let mut salt = vec![0u8; salt_size];
        
        // Generate deterministic salt based on message and public key
        let mut state = 0u64;
        
        for &byte in message {
            state = state.wrapping_mul(31).wrapping_add(byte as u64);
        for &byte in &self.public_key[0..16.min(self.public_key.len())] {
            state = state.wrapping_mul(31).wrapping_add(byte as u64);
        for i in 0..salt_size {
            state = state.wrapping_mul(1103515245).wrapping_add(12345);
            salt[i] = (state >> 24) as u8;
        salt
    /// Get algorithm identifier for PKCS#1 v1.5
    fn get_algorithm_identifier(&self) -> Vec<u8> {
        // Simulate ASN.1 algorithm identifier
        match self.hash_algorithm {
            RsaHashAlgorithm::Sha256 => vec![
                0x00, 0x04, 0x20
            RsaHashAlgorithm::Sha384 => vec![
                0x00, 0x04, 0x30
            RsaHashAlgorithm::Sha512 => vec![
                0x00, 0x04, 0x40
        }
    }
/// RSA verifier for signature verification
#[derive(Debug, Clone)]
pub struct RsaVerifier {
impl RsaVerifier {
    /// Create a new RSA verifier from a public key
    pub fn new(
        hash_algorithm: RsaHashAlgorithm
    ) -> SignatureResult<Self> {
        let key_size = match public_key.key_type {
            _ => return Err(SignatureError::InvalidPublicKey(
                format!("Expected RSA public key, got {}", public_key.key_type.name())
        
        public_key.validate()?;
        
        Ok(Self {
        })
    /// Verify a signature against a message
    pub fn verify(&self, message: &[u8], signature: &[u8]) -> SignatureResult<bool> {
        if signature.len() != self.key_size.signature_size() {
            return Err(SignatureError::InvalidSignature(
                    self.key_size.signature_size(), self.key_size.bits(), signature.len())
            ));
        if message.is_empty() {
            return Err(SignatureError::InvalidSignature(
                "Cannot verify signature for empty message".to_string()
            ));
        let is_valid = match self.scheme {
        
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
    pub fn get_stats(&self) -> RsaStats {
        self.stats.lock()
            .map(|stats| RsaStats {
            })
            .unwrap_or_default()
    /// Get the public key bytes
    pub fn public_key_bytes(&self) -> &[u8] {
        &self.public_key
    /// Get key ID
    pub fn key_id(&self) -> &str {
        &self.key_id
    /// Get the signature scheme
    pub fn scheme(&self) -> &RsaSignatureScheme {
        &self.scheme
    /// Get the hash algorithm
    pub fn hash_algorithm(&self) -> &RsaHashAlgorithm {
        &self.hash_algorithm
    /// Get the key size
    pub fn key_size(&self) -> &RsaKeySize {
        &self.key_size
    /// Internal PSS verification
    fn verify_pss_internal(&self, message: &[u8], signature: &[u8]) -> SignatureResult<bool> {
        // Same logic as signer verification but public-key only
        let message_hash = self.hash_message(message);
        
        let mut verification_state = 0u64;
        
        for &byte in &message_hash {
            verification_state = verification_state.wrapping_mul(31).wrapping_add(byte as u64);
        for &byte in &self.public_key {
            verification_state = verification_state.wrapping_mul(31).wrapping_add(byte as u64);
        let salt = self.generate_deterministic_salt(message);
        for &byte in &salt {
            verification_state = verification_state.wrapping_mul(31).wrapping_add(byte as u64);
        let signature_size = signature.len();
        let mut expected_signature = vec![0u8; signature_size];
        
        for i in 0..signature_size {
            verification_state = verification_state.wrapping_mul(1103515245).wrapping_add(12345);
            expected_signature[i] = (verification_state >> 24) as u8;
        if expected_signature.iter().all(|&b| b == 0) {
            expected_signature[0] = 1;
            expected_signature[signature_size - 1] = 1;
        let mut diff = 0u8;
        for (a, b) in signature.iter().zip(expected_signature.iter()) {
            diff |= a ^ b;
        Ok(diff == 0)
    /// Internal PKCS#1 v1.5 verification
    fn verify_pkcs1v15_internal(&self, message: &[u8], signature: &[u8]) -> SignatureResult<bool> {
        let message_hash = self.hash_message(message);
        
        let mut verification_state = 0u64;
        
        for &byte in &message_hash {
            verification_state = verification_state.wrapping_mul(31).wrapping_add(byte as u64);
        for &byte in &self.public_key {
            verification_state = verification_state.wrapping_mul(31).wrapping_add(byte as u64);
        let algorithm_id = self.get_algorithm_identifier();
        for &byte in &algorithm_id {
            verification_state = verification_state.wrapping_mul(31).wrapping_add(byte as u64);
        let signature_size = signature.len();
        let mut expected_signature = vec![0u8; signature_size];
        
        for i in 0..signature_size {
            verification_state = verification_state.wrapping_mul(1103515245).wrapping_add(12345);
            expected_signature[i] = (verification_state >> 24) as u8;
        if expected_signature.iter().all(|&b| b == 0) {
            expected_signature[0] = 1;
            expected_signature[signature_size - 1] = 1;
        let mut diff = 0u8;
        for (a, b) in signature.iter().zip(expected_signature.iter()) {
            diff |= a ^ b;
        Ok(diff == 0)
    /// Hash message (same implementation as signer)
    fn hash_message(&self, message: &[u8]) -> Vec<u8> {
        let hash_size = self.hash_algorithm.hash_size();
        let mut hash = vec![0u8; hash_size];
        
        let initial_state = match self.hash_algorithm {
        
        let mut state = initial_state;
        
        for &byte in message {
            state = state.wrapping_mul(31).wrapping_add(byte as u64);
        for i in 0..hash_size {
            state = state.wrapping_mul(1103515245).wrapping_add(12345);
            hash[i] = (state >> 24) as u8;
        hash
    /// Generate deterministic salt for PSS verification
    fn generate_deterministic_salt(&self, message: &[u8]) -> Vec<u8> {
        let salt_size = self.hash_algorithm.hash_size();
        let mut salt = vec![0u8; salt_size];
        
        let mut state = 0u64;
        
        for &byte in message {
            state = state.wrapping_mul(31).wrapping_add(byte as u64);
        for &byte in &self.public_key[0..16.min(self.public_key.len())] {
            state = state.wrapping_mul(31).wrapping_add(byte as u64);
        for i in 0..salt_size {
            state = state.wrapping_mul(1103515245).wrapping_add(12345);
            salt[i] = (state >> 24) as u8;
        salt
    /// Get algorithm identifier for PKCS#1 v1.5
    fn get_algorithm_identifier(&self) -> Vec<u8> {
        match self.hash_algorithm {
            RsaHashAlgorithm::Sha256 => vec![
                0x00, 0x04, 0x20
            RsaHashAlgorithm::Sha384 => vec![
                0x00, 0x04, 0x30
            RsaHashAlgorithm::Sha512 => vec![
                0x00, 0x04, 0x40
        }
    }
/// Utility functions for RSA signatures
pub mod utils {
    use super::*;
//     use crate::stdlib::packages::crypto_signatures::key_management::KeyGenerator;
    
    /// Generate a new RSA key pair
    pub fn generate_keypair(
        scheme: RsaSignatureScheme
    ) -> SignatureResult<KeyPair> {
        let mut generator = KeyGenerator::new();
        let key_type = match (&scheme, &key_size) {
        generator.generate_keypair(key_type)
    /// Sign a message with RSA using raw key bytes
    pub fn sign_message(
        hash_algorithm: RsaHashAlgorithm
    ) -> SignatureResult<Vec<u8>> {
        let signer = RsaSigner::from_bytes(private_key, public_key, key_size, scheme, hash_algorithm)?;
        signer.sign(message)
    /// Verify a signature with RSA using raw public key bytes
    pub fn verify_signature(
        hash_algorithm: RsaHashAlgorithm
    ) -> SignatureResult<bool> {
        let key_type = match (&scheme, &key_size) {
        
        let public_key_obj = PublicKey::new(key_type, public_key.to_vec(), None)?;
        let verifier = RsaVerifier::new(public_key_obj, scheme, hash_algorithm)?;
        verifier.verify(message, signature)
    /// Quick RSA signature verification (convenience function)
    pub fn quick_verify(
        hash_algorithm: RsaHashAlgorithm
    ) -> bool {
        verify_signature(public_key, message, signature, key_size, scheme, hash_algorithm).unwrap_or(false)
    /// Check if bytes are a valid RSA public key
    pub fn is_valid_public_key(key: &[u8], key_size: RsaKeySize) -> bool {
        key.len() == key_size.bytes() && !key.iter().all(|&b| b == 0)
    /// Check if bytes are a valid RSA signature
    pub fn is_valid_signature(signature: &[u8], key_size: RsaKeySize) -> bool {
        signature.len() == key_size.signature_size() && !signature.iter().all(|&b| b == 0)
    }
}
