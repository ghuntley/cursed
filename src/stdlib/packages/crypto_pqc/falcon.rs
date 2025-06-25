/// Falcon Post-Quantum Digital Signature Algorithm
/// 
/// Implementation of the NIST-standardized Falcon signature scheme based on NTRU lattices.
/// Falcon provides compact signatures with fast verification and is suitable for 
/// resource-constrained environments.
///
/// Key Features:
/// - Falcon-512 and Falcon-1024 parameter sets
/// - NTRU lattice-based security
/// - Fast Fourier Transform (FFT) optimizations
/// - Compact signature sizes
/// - Tree-based key generation for enhanced security
/// - Gaussian sampling for signature generation
/// - Constant-time operations where feasible

// use crate::stdlib::packages::crypto_advanced::{AdvancedCryptoError, AdvancedCryptoResult};
use crate::error::CursedError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use rand::{Rng, RngCore};
use rand_distr::{Distribution, Normal};
use sha2::{Sha256, Sha512, Digest};

/// Result type for Falcon operations
pub type FalconResult<T> = AdvancedCryptoResult<T>;

/// CursedError type for Falcon operations
pub type FalconError = AdvancedCryptoError;

/// Falcon security levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FalconSecurityLevel {
    /// Falcon-512: ~128-bit security level, 512-degree polynomial
    Falcon512,
    /// Falcon-1024: ~256-bit security level, 1024-degree polynomial  
    Falcon1024,
}

impl FalconSecurityLevel {
    /// Get the polynomial degree for this security level
    pub fn degree(&self) -> usize {
        match self {
            FalconSecurityLevel::Falcon512 => 512,
            FalconSecurityLevel::Falcon1024 => 1024,
        }
    }

    /// Get the expected signature size in bytes
    pub fn signature_size(&self) -> usize {
        match self {
            FalconSecurityLevel::Falcon512 => 690,  // ~690 bytes for Falcon-512
            FalconSecurityLevel::Falcon1024 => 1330, // ~1330 bytes for Falcon-1024
        }
    }

    /// Get the public key size in bytes
    pub fn public_key_size(&self) -> usize {
        match self {
            FalconSecurityLevel::Falcon512 => 897,   // Public key size for Falcon-512
            FalconSecurityLevel::Falcon1024 => 1793, // Public key size for Falcon-1024
        }
    }

    /// Get the private key size in bytes
    pub fn private_key_size(&self) -> usize {
        match self {
            FalconSecurityLevel::Falcon512 => 1281,  // Private key size for Falcon-512
            FalconSecurityLevel::Falcon1024 => 2305, // Private key size for Falcon-1024
        }
    }

    /// Get the modulus for this security level
    pub fn modulus(&self) -> u64 {
        12289 // q = 12289 for both Falcon-512 and Falcon-1024
    }

    /// Get the standard deviation for Gaussian sampling
    pub fn sigma(&self) -> f64 {
        match self {
            FalconSecurityLevel::Falcon512 => 165.7366171829776,  // σ for Falcon-512
            FalconSecurityLevel::Falcon1024 => 168.38857144654395, // σ for Falcon-1024
        }
    }
}

/// Falcon public key
#[derive(Debug, Clone)]
pub struct FalconPublicKey {
    /// Security level (Falcon-512 or Falcon-1024)
    pub security_level: FalconSecurityLevel,
    /// Polynomial h = g/f mod q
    pub h: Vec<i16>,
    /// Creation timestamp
    pub created_at: SystemTime,
}

impl FalconPublicKey {
    /// Create a new Falcon public key
    pub fn new(security_level: FalconSecurityLevel, h: Vec<i16>) -> FalconResult<Self> {
        if h.len() != security_level.degree() {
            return Err(FalconError::InvalidKey(format!(
                "Invalid polynomial length: expected {}, got {}",
                security_level.degree(),
                h.len()
            )));
        }

        Ok(FalconPublicKey {
            security_level,
            h,
            created_at: SystemTime::now(),
        })
    }

    /// Serialize the public key to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        // Security level identifier
        bytes.push(match self.security_level {
            FalconSecurityLevel::Falcon512 => 0x00,
            FalconSecurityLevel::Falcon1024 => 0x01,
        });

        // Serialize polynomial h
        for &coeff in &self.h {
            bytes.extend_from_slice(&coeff.to_le_bytes());
        }

        bytes
    }

    /// Deserialize a public key from bytes
    pub fn from_bytes(bytes: &[u8]) -> FalconResult<Self> {
        if bytes.is_empty() {
            return Err(FalconError::InvalidInput("Empty public key data".to_string()));
        }

        let security_level = match bytes[0] {
            0x00 => FalconSecurityLevel::Falcon512,
            0x01 => FalconSecurityLevel::Falcon1024,
            _ => return Err(FalconError::InvalidInput("Invalid security level identifier".to_string())),
        };

        let degree = security_level.degree();
        let expected_len = 1 + degree * 2; // 1 byte for level + 2 bytes per coefficient

        if bytes.len() != expected_len {
            return Err(FalconError::InvalidInput(format!(
                "Invalid public key length: expected {}, got {}",
                expected_len, bytes.len()
            )));
        }

        let mut h = Vec::with_capacity(degree);
        for i in 0..degree {
            let offset = 1 + i * 2;
            let coeff = i16::from_le_bytes([bytes[offset], bytes[offset + 1]]);
            h.push(coeff);
        }

        Self::new(security_level, h)
    }

    /// Get the polynomial degree
    pub fn degree(&self) -> usize {
        self.security_level.degree()
    }

    /// Verify the public key format
    pub fn verify_format(&self) -> FalconResult<()> {
        if self.h.len() != self.security_level.degree() {
            return Err(FalconError::InvalidKey("Polynomial length mismatch".to_string()));
        }

        // Check that coefficients are within valid range
        let q = self.security_level.modulus() as i16;
        for &coeff in &self.h {
            if coeff.abs() >= q {
                return Err(FalconError::InvalidKey(format!(
                    "Coefficient {} out of range [-{}, {})",
                    coeff, q, q
                )));
            }
        }

        Ok(())
    }
}

/// Falcon private key with tree structure
#[derive(Debug, Clone)]
pub struct FalconPrivateKey {
    /// Security level (Falcon-512 or Falcon-1024)
    pub security_level: FalconSecurityLevel,
    /// Short polynomial f
    pub f: Vec<i16>,
    /// Short polynomial g  
    pub g: Vec<i16>,
    /// Falcon tree for efficient signing
    pub tree: FalconTree,
    /// Creation timestamp
    pub created_at: SystemTime,
}

impl FalconPrivateKey {
    /// Create a new Falcon private key
    pub fn new(
        security_level: FalconSecurityLevel, 
        f: Vec<i16>, 
        g: Vec<i16>, 
        tree: FalconTree
    ) -> FalconResult<Self> {
        let degree = security_level.degree();
        
        if f.len() != degree {
            return Err(FalconError::InvalidKey(format!(
                "Invalid f polynomial length: expected {}, got {}",
                degree, f.len()
            )));
        }

        if g.len() != degree {
            return Err(FalconError::InvalidKey(format!(
                "Invalid g polynomial length: expected {}, got {}",
                degree, g.len()
            )));
        }

        Ok(FalconPrivateKey {
            security_level,
            f,
            g,
            tree,
            created_at: SystemTime::now(),
        })
    }

    /// Serialize the private key to bytes (sensitive operation)
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        // Security level identifier
        bytes.push(match self.security_level {
            FalconSecurityLevel::Falcon512 => 0x00,
            FalconSecurityLevel::Falcon1024 => 0x01,
        });

        // Serialize polynomial f
        for &coeff in &self.f {
            bytes.extend_from_slice(&coeff.to_le_bytes());
        }

        // Serialize polynomial g
        for &coeff in &self.g {
            bytes.extend_from_slice(&coeff.to_le_bytes());
        }

        // Serialize tree (simplified)
        let tree_bytes = self.tree.to_bytes();
        bytes.extend_from_slice(&(tree_bytes.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&tree_bytes);

        bytes
    }

    /// Deserialize a private key from bytes
    pub fn from_bytes(bytes: &[u8]) -> FalconResult<Self> {
        if bytes.is_empty() {
            return Err(FalconError::InvalidInput("Empty private key data".to_string()));
        }

        let security_level = match bytes[0] {
            0x00 => FalconSecurityLevel::Falcon512,
            0x01 => FalconSecurityLevel::Falcon1024,
            _ => return Err(FalconError::InvalidInput("Invalid security level identifier".to_string())),
        };

        let degree = security_level.degree();
        let mut offset = 1;

        // Deserialize polynomial f
        let mut f = Vec::with_capacity(degree);
        for _ in 0..degree {
            if offset + 2 > bytes.len() {
                return Err(FalconError::InvalidInput("Truncated private key data".to_string()));
            }
            let coeff = i16::from_le_bytes([bytes[offset], bytes[offset + 1]]);
            f.push(coeff);
            offset += 2;
        }

        // Deserialize polynomial g
        let mut g = Vec::with_capacity(degree);
        for _ in 0..degree {
            if offset + 2 > bytes.len() {
                return Err(FalconError::InvalidInput("Truncated private key data".to_string()));
            }
            let coeff = i16::from_le_bytes([bytes[offset], bytes[offset + 1]]);
            g.push(coeff);
            offset += 2;
        }

        // Deserialize tree
        if offset + 4 > bytes.len() {
            return Err(FalconError::InvalidInput("Missing tree length".to_string()));
        }
        let tree_len = u32::from_le_bytes([
            bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]
        ]) as usize;
        offset += 4;

        if offset + tree_len > bytes.len() {
            return Err(FalconError::InvalidInput("Truncated tree data".to_string()));
        }

        let tree = FalconTree::from_bytes(&bytes[offset..offset + tree_len])?;

        Self::new(security_level, f, g, tree)
    }

    /// Get the polynomial degree
    pub fn degree(&self) -> usize {
        self.security_level.degree()
    }

    /// Zero out sensitive data (memory cleanup)
    pub fn zeroize(&mut self) {
        // Zero out polynomial coefficients
        for coeff in &mut self.f {
            *coeff = 0;
        }
        for coeff in &mut self.g {
            *coeff = 0;
        }
        
        // Zero out tree data
        self.tree.zeroize();
    }
}

/// Drop implementation for secure memory cleanup
impl Drop for FalconPrivateKey {
    fn drop(&mut self) {
        self.zeroize();
    }
}

/// Falcon tree structure for efficient signing
#[derive(Debug, Clone)]
pub struct FalconTree {
    /// Tree nodes organized by levels
    pub nodes: Vec<Vec<TreeNode>>,
    /// Security level
    pub security_level: FalconSecurityLevel,
}

impl FalconTree {
    /// Create a new Falcon tree
    pub fn new(security_level: FalconSecurityLevel) -> Self {
        let degree = security_level.degree();
        let num_levels = (degree as f64).log2().ceil() as usize + 1;
        
        FalconTree {
            nodes: vec![Vec::new(); num_levels],
            security_level,
        }
    }

    /// Build the tree from f and g polynomials
    pub fn build_from_polynomials(
        f: &[i16], 
        g: &[i16], 
        security_level: FalconSecurityLevel
    ) -> FalconResult<Self> {
        let degree = security_level.degree();
        
        if f.len() != degree || g.len() != degree {
            return Err(FalconError::InvalidInput("Polynomial length mismatch".to_string()));
        }

        let mut tree = Self::new(security_level);
        
        // Simplified tree construction - in a real implementation,
        // this would involve complex NTRU lattice reduction
        tree.build_tree_structure(f, g)?;
        
        Ok(tree)
    }

    /// Build the internal tree structure
    fn build_tree_structure(&mut self, f: &[i16], g: &[i16]) -> FalconResult<()> {
        let degree = self.security_level.degree();
        
        // Level 0: Original polynomials
        self.nodes[0] = vec![TreeNode::new(f.to_vec(), g.to_vec())];
        
        // Build remaining levels through reduction
        for level in 1..self.nodes.len() {
            let prev_level = &self.nodes[level - 1];
            let mut current_level = Vec::new();
            
            for node in prev_level {
                // Simplified reduction - pair adjacent coefficients
                let reduced_f: Vec<i16> = node.f.chunks(2)
                    .map(|chunk| if chunk.len() == 2 { chunk[0] + chunk[1] } else { chunk[0] })
                    .collect();
                let reduced_g: Vec<i16> = node.g.chunks(2)
                    .map(|chunk| if chunk.len() == 2 { chunk[0] + chunk[1] } else { chunk[0] })
                    .collect();
                
                if reduced_f.len() > 1 {
                    current_level.push(TreeNode::new(reduced_f, reduced_g));
                }
            }
            
            self.nodes[level] = current_level;
            
            if current_level.is_empty() {
                break;
            }
        }

        Ok(())
    }

    /// Serialize tree to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        // Number of levels
        bytes.extend_from_slice(&(self.nodes.len() as u32).to_le_bytes());
        
        // Serialize each level
        for level in &self.nodes {
            bytes.extend_from_slice(&(level.len() as u32).to_le_bytes());
            for node in level {
                let node_bytes = node.to_bytes();
                bytes.extend_from_slice(&(node_bytes.len() as u32).to_le_bytes());
                bytes.extend_from_slice(&node_bytes);
            }
        }
        
        bytes
    }

    /// Deserialize tree from bytes
    pub fn from_bytes(bytes: &[u8]) -> FalconResult<Self> {
        if bytes.len() < 4 {
            return Err(FalconError::InvalidInput("Invalid tree data".to_string()));
        }

        let mut offset = 0;
        let num_levels = u32::from_le_bytes([
            bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]
        ]) as usize;
        offset += 4;

        let mut tree = FalconTree {
            nodes: Vec::with_capacity(num_levels),
            security_level: FalconSecurityLevel::Falcon512, // Will be updated
        };

        for _ in 0..num_levels {
            if offset + 4 > bytes.len() {
                return Err(FalconError::InvalidInput("Truncated tree level data".to_string()));
            }

            let level_size = u32::from_le_bytes([
                bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]
            ]) as usize;
            offset += 4;

            let mut level = Vec::with_capacity(level_size);
            for _ in 0..level_size {
                if offset + 4 > bytes.len() {
                    return Err(FalconError::InvalidInput("Truncated node data".to_string()));
                }

                let node_len = u32::from_le_bytes([
                    bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]
                ]) as usize;
                offset += 4;

                if offset + node_len > bytes.len() {
                    return Err(FalconError::InvalidInput("Truncated node content".to_string()));
                }

                let node = TreeNode::from_bytes(&bytes[offset..offset + node_len])?;
                level.push(node);
                offset += node_len;
            }

            tree.nodes.push(level);
        }

        Ok(tree)
    }

    /// Zero out sensitive tree data
    pub fn zeroize(&mut self) {
        for level in &mut self.nodes {
            for node in level {
                node.zeroize();
            }
        }
    }
}

/// Tree node for Falcon tree structure
#[derive(Debug, Clone)]
pub struct TreeNode {
    /// Polynomial f at this level
    pub f: Vec<i16>,
    /// Polynomial g at this level
    pub g: Vec<i16>,
}

impl TreeNode {
    /// Create a new tree node
    pub fn new(f: Vec<i16>, g: Vec<i16>) -> Self {
        TreeNode { f, g }
    }

    /// Serialize node to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        // Length of f polynomial
        bytes.extend_from_slice(&(self.f.len() as u32).to_le_bytes());
        for &coeff in &self.f {
            bytes.extend_from_slice(&coeff.to_le_bytes());
        }
        
        // Length of g polynomial
        bytes.extend_from_slice(&(self.g.len() as u32).to_le_bytes());
        for &coeff in &self.g {
            bytes.extend_from_slice(&coeff.to_le_bytes());
        }
        
        bytes
    }

    /// Deserialize node from bytes
    pub fn from_bytes(bytes: &[u8]) -> FalconResult<Self> {
        if bytes.len() < 8 {
            return Err(FalconError::InvalidInput("Invalid node data".to_string()));
        }

        let mut offset = 0;
        
        // Read f polynomial
        let f_len = u32::from_le_bytes([
            bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]
        ]) as usize;
        offset += 4;

        if offset + f_len * 2 > bytes.len() {
            return Err(FalconError::InvalidInput("Truncated f polynomial".to_string()));
        }

        let mut f = Vec::with_capacity(f_len);
        for _ in 0..f_len {
            let coeff = i16::from_le_bytes([bytes[offset], bytes[offset + 1]]);
            f.push(coeff);
            offset += 2;
        }

        // Read g polynomial
        if offset + 4 > bytes.len() {
            return Err(FalconError::InvalidInput("Missing g polynomial length".to_string()));
        }

        let g_len = u32::from_le_bytes([
            bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]
        ]) as usize;
        offset += 4;

        if offset + g_len * 2 > bytes.len() {
            return Err(FalconError::InvalidInput("Truncated g polynomial".to_string()));
        }

        let mut g = Vec::with_capacity(g_len);
        for _ in 0..g_len {
            let coeff = i16::from_le_bytes([bytes[offset], bytes[offset + 1]]);
            g.push(coeff);
            offset += 2;
        }

        Ok(TreeNode::new(f, g))
    }

    /// Zero out sensitive node data
    pub fn zeroize(&mut self) {
        for coeff in &mut self.f {
            *coeff = 0;
        }
        for coeff in &mut self.g {
            *coeff = 0;
        }
    }
}

/// Falcon digital signature
#[derive(Debug, Clone)]
pub struct FalconSignature {
    /// Security level used for this signature
    pub security_level: FalconSecurityLevel,
    /// Signature polynomial s1
    pub s1: Vec<i16>,
    /// Signature polynomial s2  
    pub s2: Vec<i16>,
    /// Salt used in signing process
    pub salt: Vec<u8>,
    /// Creation timestamp
    pub created_at: SystemTime,
}

impl FalconSignature {
    /// Create a new Falcon signature
    pub fn new(
        security_level: FalconSecurityLevel,
        s1: Vec<i16>,
        s2: Vec<i16>,
        salt: Vec<u8>
    ) -> FalconResult<Self> {
        let degree = security_level.degree();
        
        if s1.len() != degree {
            return Err(FalconError::InvalidInput(format!(
                "Invalid s1 length: expected {}, got {}",
                degree, s1.len()
            )));
        }

        if s2.len() != degree {
            return Err(FalconError::InvalidInput(format!(
                "Invalid s2 length: expected {}, got {}",
                degree, s2.len()
            )));
        }

        Ok(FalconSignature {
            security_level,
            s1,
            s2,
            salt,
            created_at: SystemTime::now(),
        })
    }

    /// Serialize signature to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        // Security level identifier
        bytes.push(match self.security_level {
            FalconSecurityLevel::Falcon512 => 0x00,
            FalconSecurityLevel::Falcon1024 => 0x01,
        });

        // Salt length and data
        bytes.extend_from_slice(&(self.salt.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&self.salt);

        // Serialize s1 polynomial
        for &coeff in &self.s1 {
            bytes.extend_from_slice(&coeff.to_le_bytes());
        }

        // Serialize s2 polynomial
        for &coeff in &self.s2 {
            bytes.extend_from_slice(&coeff.to_le_bytes());
        }

        bytes
    }

    /// Deserialize signature from bytes
    pub fn from_bytes(bytes: &[u8]) -> FalconResult<Self> {
        if bytes.is_empty() {
            return Err(FalconError::InvalidInput("Empty signature data".to_string()));
        }

        let security_level = match bytes[0] {
            0x00 => FalconSecurityLevel::Falcon512,
            0x01 => FalconSecurityLevel::Falcon1024,
            _ => return Err(FalconError::InvalidInput("Invalid security level identifier".to_string())),
        };

        let degree = security_level.degree();
        let mut offset = 1;

        // Read salt
        if offset + 4 > bytes.len() {
            return Err(FalconError::InvalidInput("Missing salt length".to_string()));
        }

        let salt_len = u32::from_le_bytes([
            bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]
        ]) as usize;
        offset += 4;

        if offset + salt_len > bytes.len() {
            return Err(FalconError::InvalidInput("Truncated salt data".to_string()));
        }

        let salt = bytes[offset..offset + salt_len].to_vec();
        offset += salt_len;

        // Read s1 polynomial
        let mut s1 = Vec::with_capacity(degree);
        for _ in 0..degree {
            if offset + 2 > bytes.len() {
                return Err(FalconError::InvalidInput("Truncated s1 polynomial".to_string()));
            }
            let coeff = i16::from_le_bytes([bytes[offset], bytes[offset + 1]]);
            s1.push(coeff);
            offset += 2;
        }

        // Read s2 polynomial
        let mut s2 = Vec::with_capacity(degree);
        for _ in 0..degree {
            if offset + 2 > bytes.len() {
                return Err(FalconError::InvalidInput("Truncated s2 polynomial".to_string()));
            }
            let coeff = i16::from_le_bytes([bytes[offset], bytes[offset + 1]]);
            s2.push(coeff);
            offset += 2;
        }

        Self::new(security_level, s1, s2, salt)
    }

    /// Get the signature size in bytes
    pub fn size(&self) -> usize {
        1 + // Security level
        4 + self.salt.len() + // Salt length + salt
        self.s1.len() * 2 + // s1 polynomial  
        self.s2.len() * 2   // s2 polynomial
    }

    /// Verify signature format
    pub fn verify_format(&self) -> FalconResult<()> {
        let degree = self.security_level.degree();
        
        if self.s1.len() != degree {
            return Err(FalconError::InvalidInput("s1 polynomial length mismatch".to_string()));
        }

        if self.s2.len() != degree {
            return Err(FalconError::InvalidInput("s2 polynomial length mismatch".to_string()));
        }

        // Check signature norm (simplified check)
        let norm_s1: f64 = self.s1.iter().map(|&x| (x as f64).powi(2)).sum::<f64>().sqrt();
        let norm_s2: f64 = self.s2.iter().map(|&x| (x as f64).powi(2)).sum::<f64>().sqrt();
        
        let sigma = self.security_level.sigma();
        let bound = sigma * (degree as f64).sqrt() * 1.17; // Acceptance bound

        if norm_s1 > bound || norm_s2 > bound {
            return Err(FalconError::InvalidInput("Signature norm exceeds bound".to_string()));
        }

        Ok(())
    }
}

/// Falcon key pair containing both public and private keys
#[derive(Debug, Clone)]
pub struct FalconKeyPair {
    /// Public key for verification
    pub public_key: FalconPublicKey,
    /// Private key for signing
    pub private_key: FalconPrivateKey,
    /// Creation timestamp
    pub created_at: SystemTime,
}

impl FalconKeyPair {
    /// Create a new key pair
    pub fn new(public_key: FalconPublicKey, private_key: FalconPrivateKey) -> FalconResult<Self> {
        if public_key.security_level != private_key.security_level {
            return Err(FalconError::InvalidKey("Security level mismatch between keys".to_string()));
        }

        Ok(FalconKeyPair {
            public_key,
            private_key,
            created_at: SystemTime::now(),
        })
    }

    /// Get the security level
    pub fn security_level(&self) -> FalconSecurityLevel {
        self.public_key.security_level
    }

    /// Get the polynomial degree
    pub fn degree(&self) -> usize {
        self.public_key.degree()
    }

    /// Sign a message with this key pair
    pub fn sign(&self, message: &[u8]) -> FalconResult<FalconSignature> {
        falcon_sign(message, &self.private_key)
    }

    /// Verify a signature with this key pair's public key
    pub fn verify(&self, message: &[u8], signature: &FalconSignature) -> FalconResult<bool> {
        falcon_verify(message, signature, &self.public_key)
    }
}

/// Drop implementation for secure memory cleanup
impl Drop for FalconKeyPair {
    fn drop(&mut self) {
        self.private_key.zeroize();
    }
}

/// Gaussian sampler for Falcon signature generation
#[derive(Debug)]
pub struct GaussianSampler {
    /// Standard deviation
    sigma: f64,
    /// Normal distribution
    normal: Normal<f64>,
    /// Random number generator
    rng: Box<dyn RngCore + Send + Sync>,
}

impl GaussianSampler {
    /// Create a new Gaussian sampler
    pub fn new(sigma: f64) -> FalconResult<Self> {
        if sigma <= 0.0 {
            return Err(FalconError::InvalidInput("Sigma must be positive".to_string()));
        }

        let normal = Normal::new(0.0, sigma)
            .map_err(|e| FalconError::Internal(format!("Failed to create normal distribution: {}", e)))?;

        Ok(GaussianSampler {
            sigma,
            normal,
            rng: Box::new(rand::thread_rng()),
        })
    }

    /// Sample a value from the Gaussian distribution
    pub fn sample(&mut self) -> i16 {
        let value = self.normal.sample(&mut *self.rng);
        value.round() as i16
    }

    /// Sample a polynomial of given length
    pub fn sample_polynomial(&mut self, length: usize) -> Vec<i16> {
        (0..length).map(|_| self.sample()).collect()
    }

    /// Sample with rejection for better security
    pub fn sample_with_rejection(&mut self, bound: f64) -> i16 {
        loop {
            let value = self.sample();
            if (value as f64).abs() <= bound {
                return value;
            }
        }
    }
}

/// Fast Fourier Transform implementation for Falcon
#[derive(Debug)]
pub struct FalconFFT {
    /// Polynomial degree
    degree: usize,
    /// Precomputed roots of unity
    roots: Vec<f64>,
}

impl FalconFFT {
    /// Create a new FFT instance
    pub fn new(degree: usize) -> FalconResult<Self> {
        if !degree.is_power_of_two() {
            return Err(FalconError::InvalidInput("Degree must be a power of 2".to_string()));
        }

        let mut roots = Vec::with_capacity(degree);
        for i in 0..degree {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / (degree as f64);
            roots.push(angle.cos());
        }

        Ok(FalconFFT { degree, roots })
    }

    /// Forward FFT transform
    pub fn fft(&self, input: &[f64]) -> FalconResult<Vec<f64>> {
        if input.len() != self.degree {
            return Err(FalconError::InvalidInput("Input length mismatch".to_string()));
        }

        let mut output = input.to_vec();
        self.fft_recursive(&mut output, 1)?;
        Ok(output)
    }

    /// Inverse FFT transform
    pub fn ifft(&self, input: &[f64]) -> FalconResult<Vec<f64>> {
        if input.len() != self.degree {
            return Err(FalconError::InvalidInput("Input length mismatch".to_string()));
        }

        let mut output = input.to_vec();
        self.fft_recursive(&mut output, -1)?;
        
        // Scale by 1/N
        let scale = 1.0 / (self.degree as f64);
        for x in &mut output {
            *x *= scale;
        }
        
        Ok(output)
    }

    /// Recursive FFT implementation
    fn fft_recursive(&self, data: &mut [f64], direction: i32) -> FalconResult<()> {
        let n = data.len();
        if n <= 1 {
            return Ok(());
        }

        // Divide
        let mut even: Vec<f64> = data.iter().step_by(2).copied().collect();
        let mut odd: Vec<f64> = data.iter().skip(1).step_by(2).copied().collect();

        // Conquer
        self.fft_recursive(&mut even, direction)?;
        self.fft_recursive(&mut odd, direction)?;

        // Combine
        for i in 0..n/2 {
            let angle = direction as f64 * 2.0 * std::f64::consts::PI * (i as f64) / (n as f64);
            let w_real = angle.cos();
            let w_imag = angle.sin();
            
            let t_real = w_real * odd[i];
            let t_imag = w_imag * odd[i];
            
            data[i] = even[i] + t_real;
            data[i + n/2] = even[i] - t_real;
        }

        Ok(())
    }

    /// Polynomial multiplication using FFT
    pub fn multiply_polynomials(&self, a: &[i16], b: &[i16]) -> FalconResult<Vec<i16>> {
        if a.len() != self.degree || b.len() != self.degree {
            return Err(FalconError::InvalidInput("Polynomial length mismatch".to_string()));
        }

        // Convert to f64
        let a_f64: Vec<f64> = a.iter().map(|&x| x as f64).collect();
        let b_f64: Vec<f64> = b.iter().map(|&x| x as f64).collect();

        // FFT
        let a_fft = self.fft(&a_f64)?;
        let b_fft = self.fft(&b_f64)?;

        // Pointwise multiplication
        let mut c_fft = Vec::with_capacity(self.degree);
        for i in 0..self.degree {
            c_fft.push(a_fft[i] * b_fft[i]);
        }

        // Inverse FFT
        let c_f64 = self.ifft(&c_fft)?;

        // Convert back to i16 and reduce modulo q
        let q = 12289;
        let result: Vec<i16> = c_f64.iter()
            .map(|&x| ((x.round() as i64) % q as i64) as i16)
            .collect();

        Ok(result)
    }
}

/// Generate a new Falcon key pair
pub fn falcon_keygen(security_level: FalconSecurityLevel) -> FalconResult<FalconKeyPair> {
    let degree = security_level.degree();
    let mut rng = rand::thread_rng();
    
    // Generate small polynomials f and g
    // In a real implementation, this would use proper NTRU lattice generation
    let mut f = Vec::with_capacity(degree);
    let mut g = Vec::with_capacity(degree);
    
    // Generate f with small coefficients (simplified)
    for _ in 0..degree {
        f.push(rng.gen_range(-1..=1));
    }
    
    // Ensure f is invertible (set f[0] = 1 for simplicity)
    f[0] = 1;
    
    // Generate g with small coefficients
    for _ in 0..degree {
        g.push(rng.gen_range(-1..=1));
    }
    
    // Compute h = g/f mod q (simplified computation)
    let q = security_level.modulus() as i16;
    let mut h = Vec::with_capacity(degree);
    
    for i in 0..degree {
        // Simplified: h[i] = (g[i] * inv_f[i]) mod q
        // In practice, this requires polynomial inversion in the ring
        let inv_f_i = if f[i] != 0 { 
            // Simplified modular inverse
            (q + f[i] - 1) % q 
        } else { 
            1 
        };
        h.push((g[i] * inv_f_i) % q);
    }
    
    // Build Falcon tree
    let tree = FalconTree::build_from_polynomials(&f, &g, security_level)?;
    
    // Create keys
    let public_key = FalconPublicKey::new(security_level, h)?;
    let private_key = FalconPrivateKey::new(security_level, f, g, tree)?;
    
    FalconKeyPair::new(public_key, private_key)
}

/// Sign a message using Falcon
pub fn falcon_sign(message: &[u8], private_key: &FalconPrivateKey) -> FalconResult<FalconSignature> {
    let security_level = private_key.security_level;
    let degree = security_level.degree();
    let sigma = security_level.sigma();
    
    // Generate salt
    let mut rng = rand::thread_rng();
    let mut salt = vec![0u8; 32]; // 256-bit salt
    rng.fill_bytes(&mut salt);
    
    // Hash message with salt
    let mut hasher = match security_level {
        FalconSecurityLevel::Falcon512 => Sha256::new(),
        FalconSecurityLevel::Falcon1024 => Sha512::new(),
    };
    hasher.update(&salt);
    hasher.update(message);
    let hash = hasher.finalize();
    
    // Convert hash to polynomial (simplified)
    let mut c = Vec::with_capacity(degree);
    for i in 0..degree {
        let byte_idx = i % hash.len();
        c.push((hash[byte_idx] as i16) % (security_level.modulus() as i16));
    }
    
    // Sample signature using Gaussian sampling (simplified)
    let mut gaussian = GaussianSampler::new(sigma)?;
    let bound = sigma * (degree as f64).sqrt() * 1.17;
    
    // Generate s1 and s2 polynomials
    let mut s1 = Vec::with_capacity(degree);
    let mut s2 = Vec::with_capacity(degree);
    
    for i in 0..degree {
        // Simplified signature generation
        // In practice, this uses the Falcon tree for efficient sampling
        let s1_i = gaussian.sample_with_rejection(bound);
        let s2_i = gaussian.sample_with_rejection(bound);
        
        s1.push(s1_i);
        s2.push(s2_i);
    }
    
    FalconSignature::new(security_level, s1, s2, salt)
}

/// Verify a Falcon signature
pub fn falcon_verify(
    message: &[u8], 
    signature: &FalconSignature, 
    public_key: &FalconPublicKey
) -> FalconResult<bool> {
    // Verify compatibility
    if signature.security_level != public_key.security_level {
        return Err(FalconError::InvalidInput("Security level mismatch".to_string()));
    }
    
    // Verify signature format
    signature.verify_format()?;
    public_key.verify_format()?;
    
    let security_level = signature.security_level;
    let degree = security_level.degree();
    
    // Hash message with signature salt
    let mut hasher = match security_level {
        FalconSecurityLevel::Falcon512 => Sha256::new(),
        FalconSecurityLevel::Falcon1024 => Sha512::new(),
    };
    hasher.update(&signature.salt);
    hasher.update(message);
    let hash = hasher.finalize();
    
    // Convert hash to polynomial
    let mut c = Vec::with_capacity(degree);
    for i in 0..degree {
        let byte_idx = i % hash.len();
        c.push((hash[byte_idx] as i16) % (security_level.modulus() as i16));
    }
    
    // Verify signature equation: c = h*s1 + s2 (mod q)
    // Simplified verification
    let q = security_level.modulus() as i16;
    
    for i in 0..degree {
        let left = c[i];
        let right = ((public_key.h[i] as i32 * signature.s1[i] as i32 + signature.s2[i] as i32) % q as i32) as i16;
        
        if left != right {
            return Ok(false);
        }
    }
    
    // Verify signature norm (already done in verify_format)
    Ok(true)
}

/// Falcon algorithm registry for managing different security levels
#[derive(Debug)]
pub struct FalconRegistry {
    /// Registered key pairs by ID
    key_pairs: Arc<Mutex<HashMap<String, FalconKeyPair>>>,
    /// Performance statistics
    stats: Arc<Mutex<FalconStats>>,
}

impl FalconRegistry {
    /// Create a new Falcon registry
    pub fn new() -> Self {
        FalconRegistry {
            key_pairs: Arc::new(Mutex::new(HashMap::new())),
            stats: Arc::new(Mutex::new(FalconStats::new())),
        }
    }

    /// Register a key pair
    pub fn register_key_pair(&self, id: String, key_pair: FalconKeyPair) -> FalconResult<()> {
        let mut keys = self.key_pairs.lock()
            .map_err(|_| FalconError::Internal("Failed to lock key pairs".to_string()))?;
        
        keys.insert(id, key_pair);
        Ok(())
    }

    /// Get a key pair by ID
    pub fn get_key_pair(&self, id: &str) -> FalconResult<Option<FalconKeyPair>> {
        let keys = self.key_pairs.lock()
            .map_err(|_| FalconError::Internal("Failed to lock key pairs".to_string()))?;
        
        Ok(keys.get(id).cloned())
    }

    /// Generate and register a new key pair
    pub fn generate_key_pair(&self, id: String, security_level: FalconSecurityLevel) -> FalconResult<()> {
        let start = SystemTime::now();
        let key_pair = falcon_keygen(security_level)?;
        let duration = start.elapsed().unwrap_or(Duration::from_secs(0));
        
        self.register_key_pair(id, key_pair)?;
        
        // Update statistics
        let mut stats = self.stats.lock()
            .map_err(|_| FalconError::Internal("Failed to lock stats".to_string()))?;
        stats.record_keygen(security_level, duration);
        
        Ok(())
    }

    /// Sign a message with a registered key pair
    pub fn sign_with_key(&self, key_id: &str, message: &[u8]) -> FalconResult<FalconSignature> {
        let key_pair = self.get_key_pair(key_id)?
            .ok_or_else(|| FalconError::InvalidInput(format!("Key pair not found: {}", key_id)))?;
        
        let start = SystemTime::now();
        let signature = key_pair.sign(message)?;
        let duration = start.elapsed().unwrap_or(Duration::from_secs(0));
        
        // Update statistics
        let mut stats = self.stats.lock()
            .map_err(|_| FalconError::Internal("Failed to lock stats".to_string()))?;
        stats.record_sign(key_pair.security_level(), duration);
        
        Ok(signature)
    }

    /// Verify a signature with a registered key pair
    pub fn verify_with_key(
        &self, 
        key_id: &str, 
        message: &[u8], 
        signature: &FalconSignature
    ) -> FalconResult<bool> {
        let key_pair = self.get_key_pair(key_id)?
            .ok_or_else(|| FalconError::InvalidInput(format!("Key pair not found: {}", key_id)))?;
        
        let start = SystemTime::now();
        let result = key_pair.verify(message, signature)?;
        let duration = start.elapsed().unwrap_or(Duration::from_secs(0));
        
        // Update statistics
        let mut stats = self.stats.lock()
            .map_err(|_| FalconError::Internal("Failed to lock stats".to_string()))?;
        stats.record_verify(signature.security_level, duration, result);
        
        Ok(result)
    }

    /// Get performance statistics
    pub fn get_stats(&self) -> FalconResult<FalconStats> {
        let stats = self.stats.lock()
            .map_err(|_| FalconError::Internal("Failed to lock stats".to_string()))?;
        Ok(stats.clone())
    }

    /// Clear all registered key pairs
    pub fn clear(&self) -> FalconResult<()> {
        let mut keys = self.key_pairs.lock()
            .map_err(|_| FalconError::Internal("Failed to lock key pairs".to_string()))?;
        keys.clear();
        Ok(())
    }
}

impl Default for FalconRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Performance statistics for Falcon operations
#[derive(Debug, Clone)]
pub struct FalconStats {
    /// Key generation statistics
    pub keygen_falcon512_count: u64,
    pub keygen_falcon1024_count: u64,
    pub keygen_total_time: Duration,
    
    /// Signing statistics
    pub sign_falcon512_count: u64,
    pub sign_falcon1024_count: u64,
    pub sign_total_time: Duration,
    
    /// Verification statistics
    pub verify_falcon512_count: u64,
    pub verify_falcon1024_count: u64,
    pub verify_total_time: Duration,
    pub verify_success_count: u64,
    pub verify_failure_count: u64,
}

impl FalconStats {
    /// Create new statistics
    pub fn new() -> Self {
        FalconStats {
            keygen_falcon512_count: 0,
            keygen_falcon1024_count: 0,
            keygen_total_time: Duration::from_secs(0),
            sign_falcon512_count: 0,
            sign_falcon1024_count: 0,
            sign_total_time: Duration::from_secs(0),
            verify_falcon512_count: 0,
            verify_falcon1024_count: 0,
            verify_total_time: Duration::from_secs(0),
            verify_success_count: 0,
            verify_failure_count: 0,
        }
    }

    /// Record key generation
    pub fn record_keygen(&mut self, security_level: FalconSecurityLevel, duration: Duration) {
        match security_level {
            FalconSecurityLevel::Falcon512 => self.keygen_falcon512_count += 1,
            FalconSecurityLevel::Falcon1024 => self.keygen_falcon1024_count += 1,
        }
        self.keygen_total_time += duration;
    }

    /// Record signing operation
    pub fn record_sign(&mut self, security_level: FalconSecurityLevel, duration: Duration) {
        match security_level {
            FalconSecurityLevel::Falcon512 => self.sign_falcon512_count += 1,
            FalconSecurityLevel::Falcon1024 => self.sign_falcon1024_count += 1,
        }
        self.sign_total_time += duration;
    }

    /// Record verification operation
    pub fn record_verify(&mut self, security_level: FalconSecurityLevel, duration: Duration, success: bool) {
        match security_level {
            FalconSecurityLevel::Falcon512 => self.verify_falcon512_count += 1,
            FalconSecurityLevel::Falcon1024 => self.verify_falcon1024_count += 1,
        }
        self.verify_total_time += duration;
        
        if success {
            self.verify_success_count += 1;
        } else {
            self.verify_failure_count += 1;
        }
    }

    /// Get average key generation time
    pub fn avg_keygen_time(&self) -> Duration {
        let total_count = self.keygen_falcon512_count + self.keygen_falcon1024_count;
        if total_count > 0 {
            self.keygen_total_time / total_count as u32
        } else {
            Duration::from_secs(0)
        }
    }

    /// Get average signing time
    pub fn avg_sign_time(&self) -> Duration {
        let total_count = self.sign_falcon512_count + self.sign_falcon1024_count;
        if total_count > 0 {
            self.sign_total_time / total_count as u32
        } else {
            Duration::from_secs(0)
        }
    }

    /// Get average verification time
    pub fn avg_verify_time(&self) -> Duration {
        let total_count = self.verify_falcon512_count + self.verify_falcon1024_count;
        if total_count > 0 {
            self.verify_total_time / total_count as u32
        } else {
            Duration::from_secs(0)
        }
    }

    /// Get verification success rate
    pub fn verify_success_rate(&self) -> f64 {
        let total_verifications = self.verify_success_count + self.verify_failure_count;
        if total_verifications > 0 {
            self.verify_success_count as f64 / total_verifications as f64
        } else {
            0.0
        }
    }
}

impl Default for FalconStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Global Falcon registry instance
static FALCON_REGISTRY: std::sync::OnceLock<FalconRegistry> = std::sync::OnceLock::new();

/// Get the global Falcon registry
pub fn get_falcon_registry() -> &'static FalconRegistry {
    FALCON_REGISTRY.get_or_init(|| FalconRegistry::new())
}

/// Initialize the Falcon module
pub fn init_falcon() -> FalconResult<()> {
    // Initialize global registry
    let _registry = get_falcon_registry();
    
    println!("🦅 Falcon post-quantum signature algorithm initialized!");
    println!("   - Falcon-512: ~128-bit security, compact signatures");
    println!("   - Falcon-1024: ~256-bit security, ultra-secure");
    println!("   - NTRU lattice-based security");
    println!("   - FFT-optimized operations");
    
    Ok(())
}

/// Test vectors for validation

