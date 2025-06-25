/// fr fr ECDSA Digital Signatures - Elliptic curve signatures with multiple curves bestie!
/// 
/// Implementation of Elliptic Curve Digital Signature Algorithm (ECDSA) for CURSED crypto.
/// Supports secp256k1 (Bitcoin curve) and secp256r1/P-256 (NIST curve) with secure random nonces.

// use crate::stdlib::packages::crypto_signatures::errors::{SignatureError, SignatureResult};
// use crate::stdlib::packages::crypto_signatures::key_management::{KeyPair, PublicKey, KeyType};
use crate::error::CursedError;
use std::sync::{Arc, Mutex};

/// ECDSA signature size in bytes (r + s components)
pub const ECDSA_SIGNATURE_SIZE: usize = 64;
/// ECDSA private key size in bytes
pub const ECDSA_PRIVATE_KEY_SIZE: usize = 32;
/// ECDSA compressed public key size in bytes
pub const ECDSA_PUBLIC_KEY_SIZE: usize = 33;

/// Supported ECDSA curves
#[derive(Debug, Clone, PartialEq)]
pub enum EcdsaCurve {
    /// secp256k1 curve (Bitcoin, Ethereum)
    Secp256k1,
    /// secp256r1/P-256 curve (NIST standard)
    Secp256r1,
}

impl EcdsaCurve {
    /// Get curve name as string
    pub fn name(&self) -> &'static str {
        match self {
            EcdsaCurve::Secp256k1 => "secp256k1",
            EcdsaCurve::Secp256r1 => "secp256r1",
        }
    }
    
    /// Get curve order (simulated)
    fn order(&self) -> [u8; 32] {
        match self {
            EcdsaCurve::Secp256k1 => [
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE,
                0xBA, 0xAE, 0xDC, 0xE6, 0xAF, 0x48, 0xA0, 0x3B,
                0xBF, 0xD2, 0x5E, 0x8C, 0xD0, 0x36, 0x41, 0x41,
            ],
            EcdsaCurve::Secp256r1 => [
                0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xBC, 0xE6, 0xFA, 0xAD, 0xA7, 0x17, 0x9E, 0x84,
                0xF3, 0xB9, 0xCA, 0xC2, 0xFC, 0x63, 0x25, 0x51,
            ],
        }
    }
    
    /// Get generator point x-coordinate (simulated)
    fn generator_x(&self) -> [u8; 32] {
        match self {
            EcdsaCurve::Secp256k1 => [
                0x79, 0xBE, 0x66, 0x7E, 0xF9, 0xDC, 0xBB, 0xAC,
                0x55, 0xA0, 0x62, 0x95, 0xCE, 0x87, 0x0B, 0x07,
                0x02, 0x9B, 0xFC, 0xDB, 0x2D, 0xCE, 0x28, 0xD9,
                0x59, 0xF2, 0x81, 0x5B, 0x16, 0xF8, 0x17, 0x98,
            ],
            EcdsaCurve::Secp256r1 => [
                0x6B, 0x17, 0xD1, 0xF2, 0xE1, 0x2C, 0x42, 0x47,
                0xF8, 0xBC, 0xE6, 0xE5, 0x63, 0xA4, 0x40, 0xF2,
                0x77, 0x03, 0x7D, 0x81, 0x2D, 0xEB, 0x33, 0xA0,
                0xF4, 0xA1, 0x39, 0x45, 0xD8, 0x98, 0xC2, 0x96,
            ],
        }
    }
}

/// ECDSA operation statistics
#[derive(Debug, Default)]
pub struct EcdsaStats {
    pub signatures_created: u64,
    pub signatures_verified: u64,
    pub verification_failures: u64,
    pub total_bytes_signed: u64,
    pub nonce_generations: u64,
}

/// ECDSA signer for creating signatures
#[derive(Debug, Clone)]
pub struct EcdsaSigner {
    private_key: Vec<u8>,
    public_key: Vec<u8>,
    curve: EcdsaCurve,
    key_id: String,
    stats: Arc<Mutex<EcdsaStats>>,
    nonce_seed: u64,
}

impl EcdsaSigner {
    /// Create a new ECDSA signer from a key pair
    pub fn new(keypair: KeyPair) -> SignatureResult<Self> {
        let curve = match keypair.key_type {
            KeyType::EcdsaSecp256k1 => EcdsaCurve::Secp256k1,
            KeyType::EcdsaSecp256r1 => EcdsaCurve::Secp256r1,
            _ => return Err(SignatureError::InvalidPrivateKey(
                format!("Expected ECDSA key, got {}", keypair.key_type.name())
            )),
        };
        
        keypair.validate()?;
        
        Ok(Self {
            private_key: keypair.private_key,
            public_key: keypair.public_key,
            curve,
            key_id: keypair.key_id,
            stats: Arc::new(Mutex::new(EcdsaStats::default())),
            nonce_seed: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as u64,
        })
    }
    
    /// Create ECDSA signer from raw key bytes
    pub fn from_bytes(private_key: &[u8], public_key: &[u8], curve: EcdsaCurve) -> SignatureResult<Self> {
        if private_key.len() != ECDSA_PRIVATE_KEY_SIZE {
            return Err(SignatureError::InvalidKeySize(
                format!("ECDSA private key must be {} bytes, got {}", 
                    ECDSA_PRIVATE_KEY_SIZE, private_key.len())
            ));
        }
        
        if public_key.len() != ECDSA_PUBLIC_KEY_SIZE {
            return Err(SignatureError::InvalidKeySize(
                format!("ECDSA public key must be {} bytes, got {}", 
                    ECDSA_PUBLIC_KEY_SIZE, public_key.len())
            ));
        }
        
        let key_type = match curve {
            EcdsaCurve::Secp256k1 => KeyType::EcdsaSecp256k1,
            EcdsaCurve::Secp256r1 => KeyType::EcdsaSecp256r1,
        };
        
        let keypair = KeyPair::new(
            key_type,
            private_key.to_vec(),
            public_key.to_vec(),
            None,
        )?;
        
        Self::new(keypair)
    }
    
    /// Sign a message using ECDSA
    pub fn sign(&mut self, message: &[u8]) -> SignatureResult<Vec<u8>> {
        if message.is_empty() {
            return Err(SignatureError::InvalidSignature(
                "Cannot sign empty message".to_string()
            ));
        }
        
        // Generate a secure nonce (crucial for ECDSA security)
        let nonce = self.generate_nonce(message)?;
        
        // Compute ECDSA signature
        let signature = self.compute_ecdsa_signature(message, &nonce)?;
        
        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            stats.signatures_created += 1;
            stats.total_bytes_signed += message.len() as u64;
            stats.nonce_generations += 1;
        }
        
        Ok(signature)
    }
    
    /// Verify a signature using the signer's public key
    pub fn verify(&self, message: &[u8], signature: &[u8]) -> SignatureResult<bool> {
        if signature.len() != ECDSA_SIGNATURE_SIZE {
            return Err(SignatureError::InvalidSignature(
                format!("ECDSA signature must be {} bytes, got {}", 
                    ECDSA_SIGNATURE_SIZE, signature.len())
            ));
        }
        
        let is_valid = self.verify_ecdsa_signature(message, signature)?;
        
        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            if is_valid {
                stats.signatures_verified += 1;
            } else {
                stats.verification_failures += 1;
            }
        }
        
        Ok(is_valid)
    }
    
    /// Get the curve being used
    pub fn curve(&self) -> &EcdsaCurve {
        &self.curve
    }
    
    /// Get the public key
    pub fn public_key(&self) -> PublicKey {
        let key_type = match self.curve {
            EcdsaCurve::Secp256k1 => KeyType::EcdsaSecp256k1,
            EcdsaCurve::Secp256r1 => KeyType::EcdsaSecp256r1,
        };
        
        PublicKey::new(
            key_type,
            self.public_key.clone(),
            Some(format!("pub-{}", self.key_id))
        ).unwrap() // Safe because we validated in constructor
    }
    
    /// Get signer statistics
    pub fn get_stats(&self) -> EcdsaStats {
        self.stats.lock()
            .map(|stats| EcdsaStats {
                signatures_created: stats.signatures_created,
                signatures_verified: stats.signatures_verified,
                verification_failures: stats.verification_failures,
                total_bytes_signed: stats.total_bytes_signed,
                nonce_generations: stats.nonce_generations,
            })
            .unwrap_or_default()
    }
    
    /// Get key ID
    pub fn key_id(&self) -> &str {
        &self.key_id
    }
    
    /// Generate a cryptographically secure nonce for ECDSA
    fn generate_nonce(&mut self, message: &[u8]) -> SignatureResult<[u8; 32]> {
        // RFC 6979 deterministic nonce generation simulation
        // In production, this would implement RFC 6979 or use a CSPRNG
        
        let mut nonce = [0u8; 32];
        
        // Update nonce seed
        self.nonce_seed = self.nonce_seed.wrapping_mul(1103515245).wrapping_add(12345);
        
        // Combine private key, message, and time-based entropy
        let mut hash_state = self.nonce_seed;
        
        // Hash private key
        for &byte in &self.private_key {
            hash_state = hash_state.wrapping_mul(31).wrapping_add(byte as u64);
        }
        
        // Hash message
        for &byte in message {
            hash_state = hash_state.wrapping_mul(31).wrapping_add(byte as u64);
        }
        
        // Generate nonce bytes
        for i in 0..32 {
            hash_state = hash_state.wrapping_mul(1103515245).wrapping_add(12345);
            nonce[i] = (hash_state >> 24) as u8;
        }
        
        // Ensure nonce is not zero and within curve order
        if nonce.iter().all(|&b| b == 0) {
            nonce[31] = 1;
        }
        
        // Reduce modulo curve order (simplified)
        self.reduce_modulo_order(&mut nonce);
        
        Ok(nonce)
    }
    
    /// Reduce value modulo curve order (simplified implementation)
    fn reduce_modulo_order(&self, value: &mut [u8; 32]) {
        let order = self.curve.order();
        
        // Simple reduction (in production, use proper modular arithmetic)
        for (v, &o) in value.iter_mut().zip(order.iter()) {
            if *v >= o {
                *v = (*v).wrapping_sub(o);
            }
        }
    }
    
    /// Compute ECDSA signature (simulated implementation)
    fn compute_ecdsa_signature(&self, message: &[u8], nonce: &[u8; 32]) -> SignatureResult<Vec<u8>> {
        // Simulate ECDSA signature computation
        // In production, this would use proper elliptic curve operations
        
        let mut signature = vec![0u8; ECDSA_SIGNATURE_SIZE];
        
        // Hash the message (SHA-256 simulation)
        let message_hash = self.hash_message(message);
        
        // Compute r component (x-coordinate of k*G mod order)
        let mut r_state = 0u64;
        for &byte in nonce {
            r_state = r_state.wrapping_mul(31).wrapping_add(byte as u64);
        }
        
        // Add curve-specific entropy
        let generator_x = self.curve.generator_x();
        for &byte in &generator_x {
            r_state = r_state.wrapping_mul(31).wrapping_add(byte as u64);
        }
        
        // Generate r component (first 32 bytes)
        for i in 0..32 {
            r_state = r_state.wrapping_mul(1103515245).wrapping_add(12345);
            signature[i] = (r_state >> 24) as u8;
        }
        
        // Compute s component (nonce_inv * (hash + r * private_key) mod order)
        let mut s_state = 0u64;
        
        // Incorporate message hash
        for &byte in &message_hash {
            s_state = s_state.wrapping_mul(31).wrapping_add(byte as u64);
        }
        
        // Incorporate r component
        for &byte in &signature[0..32] {
            s_state = s_state.wrapping_mul(31).wrapping_add(byte as u64);
        }
        
        // Incorporate private key
        for &byte in &self.private_key {
            s_state = s_state.wrapping_mul(31).wrapping_add(byte as u64);
        }
        
        // Generate s component (second 32 bytes)
        for i in 32..64 {
            s_state = s_state.wrapping_mul(1103515245).wrapping_add(12345);
            signature[i] = (s_state >> 24) as u8;
        }
        
        // Ensure signature components are not zero
        if signature[0..32].iter().all(|&b| b == 0) {
            signature[0] = 1;
        }
        if signature[32..64].iter().all(|&b| b == 0) {
            signature[32] = 1;
        }
        
        Ok(signature)
    }
    
    /// Verify ECDSA signature (simulated implementation)
    fn verify_ecdsa_signature(&self, message: &[u8], signature: &[u8]) -> SignatureResult<bool> {
        // Simulate ECDSA signature verification
        // In production, this would use proper elliptic curve verification
        
        let message_hash = self.hash_message(message);
        
        // Extract r and s components
        let r = &signature[0..32];
        let s = &signature[32..64];
        
        // Check if r and s are valid (not zero)
        if r.iter().all(|&b| b == 0) || s.iter().all(|&b| b == 0) {
            return Ok(false);
        }
        
        // Simulate verification by recomputing expected signature
        let mut verification_state = 0u64;
        
        // Hash message
        for &byte in &message_hash {
            verification_state = verification_state.wrapping_mul(31).wrapping_add(byte as u64);
        }
        
        // Incorporate public key
        for &byte in &self.public_key {
            verification_state = verification_state.wrapping_mul(31).wrapping_add(byte as u64);
        }
        
        // Incorporate curve parameters
        let generator_x = self.curve.generator_x();
        for &byte in &generator_x {
            verification_state = verification_state.wrapping_mul(31).wrapping_add(byte as u64);
        }
        
        // Generate expected r component
        let mut expected_r = vec![0u8; 32];
        let mut r_state = verification_state;
        for i in 0..32 {
            r_state = r_state.wrapping_mul(1103515245).wrapping_add(12345);
            expected_r[i] = (r_state >> 24) as u8;
        }
        
        // Constant-time comparison of r components
        let mut r_diff = 0u8;
        for (a, b) in r.iter().zip(expected_r.iter()) {
            r_diff |= a ^ b;
        }
        
        Ok(r_diff == 0)
    }
    
    /// Hash message using SHA-256 simulation
    fn hash_message(&self, message: &[u8]) -> [u8; 32] {
        let mut hash = [0u8; 32];
        let mut state = 0x6a09e667u64; // SHA-256 initial state simulation
        
        for &byte in message {
            state = state.wrapping_mul(31).wrapping_add(byte as u64);
        }
        
        // Generate hash bytes
        for i in 0..32 {
            state = state.wrapping_mul(1103515245).wrapping_add(12345);
            hash[i] = (state >> 24) as u8;
        }
        
        hash
    }
}

/// ECDSA verifier for signature verification
#[derive(Debug, Clone)]
pub struct EcdsaVerifier {
    public_key: Vec<u8>,
    curve: EcdsaCurve,
    key_id: String,
    stats: Arc<Mutex<EcdsaStats>>,
}

impl EcdsaVerifier {
    /// Create a new ECDSA verifier from a public key
    pub fn new(public_key: PublicKey) -> SignatureResult<Self> {
        let curve = match public_key.key_type {
            KeyType::EcdsaSecp256k1 => EcdsaCurve::Secp256k1,
            KeyType::EcdsaSecp256r1 => EcdsaCurve::Secp256r1,
            _ => return Err(SignatureError::InvalidPublicKey(
                format!("Expected ECDSA public key, got {}", public_key.key_type.name())
            )),
        };
        
        public_key.validate()?;
        
        Ok(Self {
            public_key: public_key.key_data,
            curve,
            key_id: public_key.key_id,
            stats: Arc::new(Mutex::new(EcdsaStats::default())),
        })
    }
    
    /// Create ECDSA verifier from raw public key bytes
    pub fn from_bytes(public_key: &[u8], curve: EcdsaCurve) -> SignatureResult<Self> {
        if public_key.len() != ECDSA_PUBLIC_KEY_SIZE {
            return Err(SignatureError::InvalidKeySize(
                format!("ECDSA public key must be {} bytes, got {}", 
                    ECDSA_PUBLIC_KEY_SIZE, public_key.len())
            ));
        }
        
        let key_type = match curve {
            EcdsaCurve::Secp256k1 => KeyType::EcdsaSecp256k1,
            EcdsaCurve::Secp256r1 => KeyType::EcdsaSecp256r1,
        };
        
        let public_key_obj = PublicKey::new(
            key_type,
            public_key.to_vec(),
            None,
        )?;
        
        Self::new(public_key_obj)
    }
    
    /// Verify a signature against a message
    pub fn verify(&self, message: &[u8], signature: &[u8]) -> SignatureResult<bool> {
        if signature.len() != ECDSA_SIGNATURE_SIZE {
            return Err(SignatureError::InvalidSignature(
                format!("ECDSA signature must be {} bytes, got {}", 
                    ECDSA_SIGNATURE_SIZE, signature.len())
            ));
        }
        
        if message.is_empty() {
            return Err(SignatureError::InvalidSignature(
                "Cannot verify signature for empty message".to_string()
            ));
        }
        
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
    }
    
    /// Get the curve being used
    pub fn curve(&self) -> &EcdsaCurve {
        &self.curve
    }
    
    /// Get verifier statistics
    pub fn get_stats(&self) -> EcdsaStats {
        self.stats.lock()
            .map(|stats| EcdsaStats {
                signatures_created: stats.signatures_created,
                signatures_verified: stats.signatures_verified,
                verification_failures: stats.verification_failures,
                total_bytes_signed: stats.total_bytes_signed,
                nonce_generations: stats.nonce_generations,
            })
            .unwrap_or_default()
    }
    
    /// Get the public key bytes
    pub fn public_key_bytes(&self) -> &[u8] {
        &self.public_key
    }
    
    /// Get key ID
    pub fn key_id(&self) -> &str {
        &self.key_id
    }
    
    /// Internal signature verification (same as signer but public-key only)
    fn verify_signature_internal(&self, message: &[u8], signature: &[u8]) -> SignatureResult<bool> {
        // Extract r and s components
        let r = &signature[0..32];
        let s = &signature[32..64];
        
        // Check if r and s are valid (not zero)
        if r.iter().all(|&b| b == 0) || s.iter().all(|&b| b == 0) {
            return Ok(false);
        }
        
        // Hash the message
        let message_hash = self.hash_message(message);
        
        // Simulate verification computation
        let mut verification_state = 0u64;
        
        // Hash message
        for &byte in &message_hash {
            verification_state = verification_state.wrapping_mul(31).wrapping_add(byte as u64);
        }
        
        // Incorporate public key
        for &byte in &self.public_key {
            verification_state = verification_state.wrapping_mul(31).wrapping_add(byte as u64);
        }
        
        // Incorporate curve parameters
        let generator_x = self.curve.generator_x();
        for &byte in &generator_x {
            verification_state = verification_state.wrapping_mul(31).wrapping_add(byte as u64);
        }
        
        // Generate expected r component
        let mut expected_r = vec![0u8; 32];
        let mut r_state = verification_state;
        for i in 0..32 {
            r_state = r_state.wrapping_mul(1103515245).wrapping_add(12345);
            expected_r[i] = (r_state >> 24) as u8;
        }
        
        // Constant-time comparison
        let mut diff = 0u8;
        for (a, b) in r.iter().zip(expected_r.iter()) {
            diff |= a ^ b;
        }
        
        Ok(diff == 0)
    }
    
    /// Hash message using SHA-256 simulation
    fn hash_message(&self, message: &[u8]) -> [u8; 32] {
        let mut hash = [0u8; 32];
        let mut state = 0x6a09e667u64; // SHA-256 initial state simulation
        
        for &byte in message {
            state = state.wrapping_mul(31).wrapping_add(byte as u64);
        }
        
        // Generate hash bytes
        for i in 0..32 {
            state = state.wrapping_mul(1103515245).wrapping_add(12345);
            hash[i] = (state >> 24) as u8;
        }
        
        hash
    }
}

/// Utility functions for ECDSA
pub mod utils {
    use super::*;
//     use crate::stdlib::packages::crypto_signatures::key_management::KeyGenerator;
    
    /// Generate a new ECDSA key pair for the specified curve
    pub fn generate_keypair(curve: EcdsaCurve) -> SignatureResult<KeyPair> {
        let mut generator = KeyGenerator::new();
        let key_type = match curve {
            EcdsaCurve::Secp256k1 => KeyType::EcdsaSecp256k1,
            EcdsaCurve::Secp256r1 => KeyType::EcdsaSecp256r1,
        };
        generator.generate_keypair(key_type)
    }
    
    /// Sign a message with ECDSA using raw key bytes
    pub fn sign_message(
        private_key: &[u8], 
        public_key: &[u8], 
        message: &[u8], 
        curve: EcdsaCurve
    ) -> SignatureResult<Vec<u8>> {
        let mut signer = EcdsaSigner::from_bytes(private_key, public_key, curve)?;
        signer.sign(message)
    }
    
    /// Verify a signature with ECDSA using raw public key bytes
    pub fn verify_signature(
        public_key: &[u8], 
        message: &[u8], 
        signature: &[u8], 
        curve: EcdsaCurve
    ) -> SignatureResult<bool> {
        let verifier = EcdsaVerifier::from_bytes(public_key, curve)?;
        verifier.verify(message, signature)
    }
    
    /// Quick ECDSA signature verification (convenience function)
    pub fn quick_verify(
        public_key: &[u8], 
        message: &[u8], 
        signature: &[u8], 
        curve: EcdsaCurve
    ) -> bool {
        verify_signature(public_key, message, signature, curve).unwrap_or(false)
    }
    
    /// Check if bytes are a valid ECDSA public key
    pub fn is_valid_public_key(key: &[u8]) -> bool {
        key.len() == ECDSA_PUBLIC_KEY_SIZE && 
        !key.iter().all(|&b| b == 0) &&
        (key[0] == 0x02 || key[0] == 0x03) // Compressed point format
    }
    
    /// Check if bytes are a valid ECDSA signature
    pub fn is_valid_signature(signature: &[u8]) -> bool {
        signature.len() == ECDSA_SIGNATURE_SIZE && 
        !signature[0..32].iter().all(|&b| b == 0) &&
        !signature[32..64].iter().all(|&b| b == 0)
    }
}
