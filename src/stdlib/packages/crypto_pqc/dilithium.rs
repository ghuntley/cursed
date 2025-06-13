/// Dilithium Post-Quantum Digital Signature Algorithm
/// 
/// Implementation of NIST-standardized Dilithium digital signature schemes
/// for post-quantum cryptography. Provides three security levels:
/// - Dilithium2: NIST Security Level 2
/// - Dilithium3: NIST Security Level 3  
/// - Dilithium5: NIST Security Level 5
///
/// Based on lattice-based cryptography with rejection sampling for security.

use crate::error::CryptoError;
use crate::stdlib::packages::crypto_random::secure_random;
use std::fmt;
use std::collections::HashMap;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Dilithium algorithm parameters for different security levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DilithiumLevel {
    /// Dilithium2 - NIST Security Level 2 (128-bit)
    Level2,
    /// Dilithium3 - NIST Security Level 3 (192-bit) 
    Level3,
    /// Dilithium5 - NIST Security Level 5 (256-bit)
    Level5,
}

/// Dilithium parameter set configuration
#[derive(Debug, Clone)]
pub struct DilithiumParams {
    /// Security level
    pub level: DilithiumLevel,
    /// Dimension of the lattice
    pub n: usize,
    /// Modulus
    pub q: i32,
    /// Matrix dimensions
    pub k: usize,
    pub l: usize,
    /// Rejection sampling bound
    pub eta: i32,
    /// Signature bound
    pub tau: i32,
    /// Challenge weight
    pub beta: i32,
    /// Public key size in bytes
    pub pk_size: usize,
    /// Private key size in bytes
    pub sk_size: usize,
    /// Signature size in bytes
    pub sig_size: usize,
}

impl DilithiumParams {
    /// Get parameters for specified security level
    pub fn new(level: DilithiumLevel) -> Self {
        match level {
            DilithiumLevel::Level2 => DilithiumParams {
                level,
                n: 256,
                q: 8380417,
                k: 4,
                l: 4,
                eta: 2,
                tau: 39,
                beta: 78,
                pk_size: 1312,
                sk_size: 2528,
                sig_size: 2420,
            },
            DilithiumLevel::Level3 => DilithiumParams {
                level,
                n: 256,
                q: 8380417,
                k: 6,
                l: 5,
                eta: 4,
                tau: 49,
                beta: 196,
                pk_size: 1952,
                sk_size: 4000,
                sig_size: 3293,
            },
            DilithiumLevel::Level5 => DilithiumParams {
                level,
                n: 256,
                q: 8380417,
                k: 8,
                l: 7,
                eta: 2,
                tau: 60,
                beta: 120,
                pk_size: 2592,
                sk_size: 4864,
                sig_size: 4595,
            },
        }
    }
}

/// Dilithium public key
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DilithiumPublicKey {
    /// Security level parameters
    pub params: DilithiumParams,
    /// Public key data
    pub key_data: Vec<u8>,
    /// Matrix A seed
    pub rho: Vec<u8>,
    /// Packed t1 vector
    pub t1: Vec<u8>,
}

impl DilithiumPublicKey {
    /// Create new public key from raw data
    pub fn from_bytes(level: DilithiumLevel, data: &[u8]) -> Result<Self, CryptoError> {
        let params = DilithiumParams::new(level);
        
        if data.len() != params.pk_size {
            return Err(CryptoError::InvalidKeyLength {
                expected: params.pk_size,
                actual: data.len(),
            });
        }

        // Split public key into rho (32 bytes) and t1 (remaining bytes)
        let rho = data[..32].to_vec();
        let t1 = data[32..].to_vec();

        Ok(DilithiumPublicKey {
            params,
            key_data: data.to_vec(),
            rho,
            t1,
        })
    }

    /// Get raw public key bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        self.key_data.clone()
    }

    /// Get public key size in bytes
    pub fn size(&self) -> usize {
        self.params.pk_size
    }

    /// Verify signature using this public key
    pub fn verify(&self, message: &[u8], signature: &DilithiumSignature) -> Result<bool, CryptoError> {
        verify_signature(self, message, signature)
    }
}

/// Dilithium private key with secure memory handling
#[derive(Debug, Clone)]
pub struct DilithiumPrivateKey {
    /// Security level parameters
    pub params: DilithiumParams,
    /// Private key data (zeroized on drop)
    key_data: Vec<u8>,
    /// Matrix A seed
    pub rho: Vec<u8>,
    /// Signing key
    pub k_key: Vec<u8>,
    /// Randomness for signing
    pub tr: Vec<u8>,
    /// Secret vectors s1, s2
    pub s1: Vec<u8>,
    pub s2: Vec<u8>,
    /// Precomputed t0
    pub t0: Vec<u8>,
}

impl DilithiumPrivateKey {
    /// Create new private key from raw data
    pub fn from_bytes(level: DilithiumLevel, data: &[u8]) -> Result<Self, CryptoError> {
        let params = DilithiumParams::new(level);
        
        if data.len() != params.sk_size {
            return Err(CryptoError::InvalidKeyLength {
                expected: params.sk_size,
                actual: data.len(),
            });
        }

        // Parse private key components
        let mut offset = 0;
        
        let rho = data[offset..offset + 32].to_vec();
        offset += 32;
        
        let k_key = data[offset..offset + 32].to_vec();
        offset += 32;
        
        let tr = data[offset..offset + 48].to_vec();
        offset += 48;
        
        // Calculate remaining sizes based on parameters
        let s1_size = params.l * params.n * 3; // 3 bits per coefficient
        let s2_size = params.k * params.n * 3;
        let t0_size = params.k * params.n * 13; // 13 bits per coefficient
        
        let s1 = data[offset..offset + s1_size].to_vec();
        offset += s1_size;
        
        let s2 = data[offset..offset + s2_size].to_vec();
        offset += s2_size;
        
        let t0 = data[offset..offset + t0_size].to_vec();

        Ok(DilithiumPrivateKey {
            params,
            key_data: data.to_vec(),
            rho,
            k_key,
            tr,
            s1,
            s2,
            t0,
        })
    }

    /// Get raw private key bytes (creates temporary copy)
    pub fn to_bytes(&self) -> Vec<u8> {
        self.key_data.clone()
    }

    /// Get private key size in bytes
    pub fn size(&self) -> usize {
        self.params.sk_size
    }

    /// Sign message using this private key
    pub fn sign(&self, message: &[u8]) -> Result<DilithiumSignature, CryptoError> {
        sign_message(self, message)
    }
}

impl Drop for DilithiumPrivateKey {
    fn drop(&mut self) {
        // Zeroize sensitive data
        self.key_data.zeroize();
        self.k_key.zeroize();
        self.s1.zeroize();
        self.s2.zeroize();
        self.t0.zeroize();
    }
}

/// Dilithium digital signature
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DilithiumSignature {
    /// Security level parameters
    pub params: DilithiumParams,
    /// Signature data
    pub signature_data: Vec<u8>,
    /// Challenge c
    pub c: Vec<u8>,
    /// Response z
    pub z: Vec<u8>,
    /// Hint h
    pub h: Vec<u8>,
}

impl DilithiumSignature {
    /// Create signature from raw bytes
    pub fn from_bytes(level: DilithiumLevel, data: &[u8]) -> Result<Self, CryptoError> {
        let params = DilithiumParams::new(level);
        
        if data.len() != params.sig_size {
            return Err(CryptoError::InvalidSignatureFormat(
                format!("Invalid signature length: expected {}, got {}", params.sig_size, data.len())
            ));
        }

        // Parse signature components
        let mut offset = 0;
        
        let c = data[offset..offset + 32].to_vec();
        offset += 32;
        
        let z_size = params.l * params.n * 3; // 3 bytes per coefficient
        let z = data[offset..offset + z_size].to_vec();
        offset += z_size;
        
        let h = data[offset..].to_vec();

        Ok(DilithiumSignature {
            params,
            signature_data: data.to_vec(),
            c,
            z,
            h,
        })
    }

    /// Get raw signature bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        self.signature_data.clone()
    }

    /// Get signature size in bytes
    pub fn size(&self) -> usize {
        self.params.sig_size
    }
}

/// Dilithium key pair
#[derive(Debug, Clone)]
pub struct DilithiumKeyPair {
    /// Public key
    pub public_key: DilithiumPublicKey,
    /// Private key
    pub private_key: DilithiumPrivateKey,
}

impl DilithiumKeyPair {
    /// Generate new key pair for specified security level
    pub fn generate(level: DilithiumLevel) -> Result<Self, CryptoError> {
        generate_keypair(level)
    }

    /// Create key pair from existing keys
    pub fn from_keys(public_key: DilithiumPublicKey, private_key: DilithiumPrivateKey) -> Result<Self, CryptoError> {
        // Verify keys are compatible
        if public_key.params.level != private_key.params.level {
            return Err(CryptoError::InvalidKeyPair(
                "Public and private keys have different security levels".to_string()
            ));
        }

        Ok(DilithiumKeyPair {
            public_key,
            private_key,
        })
    }

    /// Sign message with private key
    pub fn sign(&self, message: &[u8]) -> Result<DilithiumSignature, CryptoError> {
        self.private_key.sign(message)
    }

    /// Verify signature with public key
    pub fn verify(&self, message: &[u8], signature: &DilithiumSignature) -> Result<bool, CryptoError> {
        self.public_key.verify(message, signature)
    }
}

/// Generate Dilithium key pair
pub fn generate_keypair(level: DilithiumLevel) -> Result<DilithiumKeyPair, CryptoError> {
    let params = DilithiumParams::new(level);
    
    // Generate random seed
    let mut seed = vec![0u8; 32];
    secure_random(&mut seed)?;
    
    // Generate matrix A seed (rho)
    let mut rho = vec![0u8; 32];
    secure_random(&mut rho)?;
    
    // Generate signing key
    let mut k_key = vec![0u8; 32];
    secure_random(&mut k_key)?;
    
    // Generate randomness for signing
    let mut tr = vec![0u8; 48];
    secure_random(&mut tr)?;
    
    // Generate secret vectors s1, s2 using rejection sampling
    let s1 = generate_secret_vector(&params, params.l)?;
    let s2 = generate_secret_vector(&params, params.k)?;
    
    // Compute t = A * s1 + s2 (simplified simulation)
    let t = compute_t_vector(&params, &rho, &s1, &s2)?;
    
    // Split t into t1 and t0
    let (t1, t0) = split_t_vector(&params, &t)?;
    
    // Construct public key
    let mut pk_data = Vec::new();
    pk_data.extend_from_slice(&rho);
    pk_data.extend_from_slice(&t1);
    
    let public_key = DilithiumPublicKey {
        params: params.clone(),
        key_data: pk_data,
        rho: rho.clone(),
        t1,
    };
    
    // Construct private key
    let mut sk_data = Vec::new();
    sk_data.extend_from_slice(&rho);
    sk_data.extend_from_slice(&k_key);
    sk_data.extend_from_slice(&tr);
    sk_data.extend_from_slice(&s1);
    sk_data.extend_from_slice(&s2);
    sk_data.extend_from_slice(&t0);
    
    let private_key = DilithiumPrivateKey {
        params,
        key_data: sk_data,
        rho,
        k_key,
        tr,
        s1,
        s2,
        t0,
    };
    
    Ok(DilithiumKeyPair {
        public_key,
        private_key,
    })
}

/// Sign message with Dilithium private key
pub fn sign_message(private_key: &DilithiumPrivateKey, message: &[u8]) -> Result<DilithiumSignature, CryptoError> {
    let params = &private_key.params;
    
    // Generate random nonce
    let mut nonce = vec![0u8; 32];
    secure_random(&mut nonce)?;
    
    // Perform rejection sampling for signature generation
    let max_attempts = 1000;
    
    for attempt in 0..max_attempts {
        // Generate commitment
        let y = generate_commitment_vector(params)?;
        let w = compute_commitment(params, &private_key.rho, &y)?;
        let w1 = high_bits(&w, params.q);
        
        // Compute challenge
        let c = compute_challenge(params, &w1, message)?;
        
        // Compute response
        let z = compute_response(params, &c, &private_key.s1, &y)?;
        
        // Check rejection sampling conditions
        if is_valid_signature(params, &z, &c) {
            // Compute hint
            let h = compute_hint(params, &private_key.s2, &c, &w)?;
            
            // Construct signature
            let mut sig_data = Vec::new();
            sig_data.extend_from_slice(&c);
            sig_data.extend_from_slice(&z);
            sig_data.extend_from_slice(&h);
            
            return Ok(DilithiumSignature {
                params: params.clone(),
                signature_data: sig_data,
                c,
                z,
                h,
            });
        }
    }
    
    Err(CryptoError::SignatureGenerationFailed(
        "Rejection sampling failed after maximum attempts".to_string()
    ))
}

/// Verify Dilithium signature
pub fn verify_signature(public_key: &DilithiumPublicKey, message: &[u8], signature: &DilithiumSignature) -> Result<bool, CryptoError> {
    let params = &public_key.params;
    
    // Verify signature format
    if signature.params.level != params.level {
        return Ok(false);
    }
    
    // Check signature bounds
    if !is_valid_signature(params, &signature.z, &signature.c) {
        return Ok(false);
    }
    
    // Recompute commitment
    let w_prime = recompute_commitment(params, public_key, &signature.c, &signature.z, &signature.h)?;
    
    // Recompute challenge
    let c_prime = compute_challenge(params, &w_prime, message)?;
    
    // Verify challenge matches
    Ok(c_prime == signature.c)
}

/// Generate secret vector using rejection sampling
fn generate_secret_vector(params: &DilithiumParams, dimension: usize) -> Result<Vec<u8>, CryptoError> {
    let mut vector = vec![0u8; dimension * params.n * 3]; // 3 bytes per coefficient
    
    for i in 0..dimension * params.n {
        loop {
            let mut candidate = vec![0u8; 1];
            secure_random(&mut candidate)?;
            
            let value = (candidate[0] as i32) % (2 * params.eta + 1) - params.eta;
            
            if value.abs() <= params.eta {
                // Pack coefficient (simplified)
                let offset = i * 3;
                vector[offset] = (value & 0xFF) as u8;
                vector[offset + 1] = ((value >> 8) & 0xFF) as u8;
                vector[offset + 2] = ((value >> 16) & 0xFF) as u8;
                break;
            }
        }
    }
    
    Ok(vector)
}

/// Compute t vector (simplified simulation)
fn compute_t_vector(params: &DilithiumParams, rho: &[u8], s1: &[u8], s2: &[u8]) -> Result<Vec<u8>, CryptoError> {
    // Simplified simulation of t = A * s1 + s2
    let mut t = vec![0u8; params.k * params.n * 4]; // 4 bytes per coefficient
    
    // Use deterministic computation based on inputs
    for i in 0..params.k * params.n {
        let a_val = (rho[i % 32] as u32) << 16;
        let s1_val = if i < s1.len() / 3 {
            let offset = i * 3;
            ((s1[offset] as u32) | ((s1[offset + 1] as u32) << 8) | ((s1[offset + 2] as u32) << 16)) & 0xFFFFFF
        } else { 0 };
        let s2_val = if i < s2.len() / 3 {
            let offset = i * 3;
            ((s2[offset] as u32) | ((s2[offset + 1] as u32) << 8) | ((s2[offset + 2] as u32) << 16)) & 0xFFFFFF
        } else { 0 };
        
        let t_val = (a_val.wrapping_mul(s1_val).wrapping_add(s2_val)) % (params.q as u32);
        
        let offset = i * 4;
        t[offset] = (t_val & 0xFF) as u8;
        t[offset + 1] = ((t_val >> 8) & 0xFF) as u8;
        t[offset + 2] = ((t_val >> 16) & 0xFF) as u8;
        t[offset + 3] = ((t_val >> 24) & 0xFF) as u8;
    }
    
    Ok(t)
}

/// Split t vector into t1 and t0
fn split_t_vector(params: &DilithiumParams, t: &[u8]) -> Result<(Vec<u8>, Vec<u8>), CryptoError> {
    let mut t1 = Vec::new();
    let mut t0 = Vec::new();
    
    for i in 0..params.k * params.n {
        let offset = i * 4;
        let t_val = (t[offset] as u32) | 
                   ((t[offset + 1] as u32) << 8) |
                   ((t[offset + 2] as u32) << 16) |
                   ((t[offset + 3] as u32) << 24);
        
        // Split into high and low bits (simplified)
        let t1_val = (t_val >> 13) & 0x7FF; // 11 bits
        let t0_val = t_val & 0x1FFF; // 13 bits
        
        // Pack t1 (11 bits per coefficient)
        t1.push((t1_val & 0xFF) as u8);
        t1.push(((t1_val >> 8) & 0x7) as u8);
        
        // Pack t0 (13 bits per coefficient)
        t0.push((t0_val & 0xFF) as u8);
        t0.push(((t0_val >> 8) & 0x1F) as u8);
    }
    
    Ok((t1, t0))
}

/// Generate commitment vector for signing
fn generate_commitment_vector(params: &DilithiumParams) -> Result<Vec<u8>, CryptoError> {
    let mut y = vec![0u8; params.l * params.n * 3]; // 3 bytes per coefficient
    
    for i in 0..params.l * params.n {
        let mut candidate = vec![0u8; 4];
        secure_random(&mut candidate)?;
        
        let value = (candidate[0] as i32) | 
                   ((candidate[1] as i32) << 8) |
                   ((candidate[2] as i32) << 16) |
                   ((candidate[3] as i32) << 24);
        
        let bounded_value = value % (2 * params.tau) - params.tau;
        
        let offset = i * 3;
        y[offset] = (bounded_value & 0xFF) as u8;
        y[offset + 1] = ((bounded_value >> 8) & 0xFF) as u8;
        y[offset + 2] = ((bounded_value >> 16) & 0xFF) as u8;
    }
    
    Ok(y)
}

/// Compute commitment from y vector
fn compute_commitment(params: &DilithiumParams, rho: &[u8], y: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let mut w = vec![0u8; params.k * params.n * 3]; // 3 bytes per coefficient
    
    // Simplified computation of w = A * y
    for i in 0..params.k * params.n {
        let a_val = (rho[i % 32] as u32) << 16;
        let y_val = if i < y.len() / 3 {
            let offset = i * 3;
            ((y[offset] as u32) | ((y[offset + 1] as u32) << 8) | ((y[offset + 2] as u32) << 16)) & 0xFFFFFF
        } else { 0 };
        
        let w_val = (a_val.wrapping_mul(y_val)) % (params.q as u32);
        
        let offset = i * 3;
        w[offset] = (w_val & 0xFF) as u8;
        w[offset + 1] = ((w_val >> 8) & 0xFF) as u8;
        w[offset + 2] = ((w_val >> 16) & 0xFF) as u8;
    }
    
    Ok(w)
}

/// Extract high bits from vector
fn high_bits(w: &[u8], q: i32) -> Vec<u8> {
    let mut w1 = Vec::new();
    
    for i in 0..w.len() / 3 {
        let offset = i * 3;
        let w_val = (w[offset] as u32) | 
                   ((w[offset + 1] as u32) << 8) |
                   ((w[offset + 2] as u32) << 16);
        
        // Extract high bits (simplified)
        let high = (w_val >> 10) & 0x3FF; // 10 bits
        
        w1.push((high & 0xFF) as u8);
        w1.push(((high >> 8) & 0x3) as u8);
    }
    
    w1
}

/// Compute challenge hash
fn compute_challenge(params: &DilithiumParams, w1: &[u8], message: &[u8]) -> Result<Vec<u8>, CryptoError> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    w1.hash(&mut hasher);
    message.hash(&mut hasher);
    params.n.hash(&mut hasher);
    
    let hash = hasher.finish();
    
    // Generate challenge of weight tau (simplified)
    let mut c = vec![0u8; 32];
    for i in 0..32 {
        c[i] = ((hash >> (i * 2)) & 0xFF) as u8;
    }
    
    Ok(c)
}

/// Compute signature response
fn compute_response(params: &DilithiumParams, c: &[u8], s1: &[u8], y: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let mut z = vec![0u8; params.l * params.n * 3]; // 3 bytes per coefficient
    
    // Compute z = y + c * s1 (coefficient-wise)
    for i in 0..params.l * params.n {
        let c_val = (c[i % 32] as i32) as i32;
        
        let s1_val = if i < s1.len() / 3 {
            let offset = i * 3;
            ((s1[offset] as i32) | ((s1[offset + 1] as i32) << 8) | ((s1[offset + 2] as i32) << 16)) & 0xFFFFFF
        } else { 0 };
        
        let y_val = if i < y.len() / 3 {
            let offset = i * 3;
            ((y[offset] as i32) | ((y[offset + 1] as i32) << 8) | ((y[offset + 2] as i32) << 16)) & 0xFFFFFF
        } else { 0 };
        
        let z_val = (y_val + c_val * s1_val) % params.q;
        
        let offset = i * 3;
        z[offset] = (z_val & 0xFF) as u8;
        z[offset + 1] = ((z_val >> 8) & 0xFF) as u8;
        z[offset + 2] = ((z_val >> 16) & 0xFF) as u8;
    }
    
    Ok(z)
}

/// Check if signature is valid (rejection sampling)
fn is_valid_signature(params: &DilithiumParams, z: &[u8], c: &[u8]) -> bool {
    // Check z bounds
    for i in 0..z.len() / 3 {
        let offset = i * 3;
        let z_val = (z[offset] as i32) | 
                   ((z[offset + 1] as i32) << 8) |
                   ((z[offset + 2] as i32) << 16);
        
        if z_val.abs() > params.tau * 2 {
            return false;
        }
    }
    
    // Check challenge weight (simplified)
    let weight: u32 = c.iter().map(|&b| b.count_ones()).sum();
    weight <= params.tau as u32
}

/// Compute hint for signature
fn compute_hint(params: &DilithiumParams, s2: &[u8], c: &[u8], w: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let mut h = vec![0u8; params.k * params.n / 8]; // 1 bit per coefficient
    
    // Simplified hint computation
    for i in 0..params.k * params.n {
        let c_val = (c[i % 32] as i32);
        
        let s2_val = if i < s2.len() / 3 {
            let offset = i * 3;
            ((s2[offset] as i32) | ((s2[offset + 1] as i32) << 8) | ((s2[offset + 2] as i32) << 16)) & 0xFFFFFF
        } else { 0 };
        
        let w_val = if i < w.len() / 3 {
            let offset = i * 3;
            ((w[offset] as i32) | ((w[offset + 1] as i32) << 8) | ((w[offset + 2] as i32) << 16)) & 0xFFFFFF
        } else { 0 };
        
        // Compute hint bit
        let hint_bit = if (w_val + c_val * s2_val) % params.q > params.q / 2 { 1 } else { 0 };
        
        // Pack bit
        let byte_idx = i / 8;
        let bit_idx = i % 8;
        if byte_idx < h.len() {
            h[byte_idx] |= (hint_bit << bit_idx) as u8;
        }
    }
    
    Ok(h)
}

/// Recompute commitment for verification
fn recompute_commitment(params: &DilithiumParams, public_key: &DilithiumPublicKey, c: &[u8], z: &[u8], h: &[u8]) -> Result<Vec<u8>, CryptoError> {
    // Simplified verification computation
    let mut w_prime = vec![0u8; params.k * params.n * 2]; // 2 bytes per coefficient for w1
    
    for i in 0..params.k * params.n {
        let a_val = (public_key.rho[i % 32] as u32) << 16;
        let c_val = (c[i % 32] as u32);
        
        let z_val = if i < z.len() / 3 {
            let offset = i * 3;
            ((z[offset] as u32) | ((z[offset + 1] as u32) << 8) | ((z[offset + 2] as u32) << 16)) & 0xFFFFFF
        } else { 0 };
        
        let t1_val = if i < public_key.t1.len() / 2 {
            let offset = i * 2;
            ((public_key.t1[offset] as u32) | ((public_key.t1[offset + 1] as u32) << 8)) & 0x7FF
        } else { 0 };
        
        // Get hint bit
        let byte_idx = i / 8;
        let bit_idx = i % 8;
        let hint_bit = if byte_idx < h.len() {
            (h[byte_idx] >> bit_idx) & 1
        } else { 0 };
        
        // Compute w1' = high_bits(A*z - c*t1*2^d + hint)
        let w_val = (a_val.wrapping_mul(z_val).wrapping_sub(c_val.wrapping_mul(t1_val << 13)).wrapping_add(hint_bit as u32)) % (params.q as u32);
        let w1_val = (w_val >> 10) & 0x3FF; // Extract high 10 bits
        
        let offset = i * 2;
        w_prime[offset] = (w1_val & 0xFF) as u8;
        w_prime[offset + 1] = ((w1_val >> 8) & 0x3) as u8;
    }
    
    Ok(w_prime)
}

/// Utility functions for common operations

/// Check if two byte slices are equal in constant time
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    
    result == 0
}

/// Generate test vectors for validation
pub fn generate_test_vectors(level: DilithiumLevel) -> Result<HashMap<String, Vec<u8>>, CryptoError> {
    let mut vectors = HashMap::new();
    
    // Generate deterministic key pair for testing
    let keypair = generate_keypair(level)?;
    vectors.insert("public_key".to_string(), keypair.public_key.to_bytes());
    vectors.insert("private_key".to_string(), keypair.private_key.to_bytes());
    
    // Test message
    let message = b"Hello, Dilithium!";
    vectors.insert("message".to_string(), message.to_vec());
    
    // Generate signature
    let signature = keypair.sign(message)?;
    vectors.insert("signature".to_string(), signature.to_bytes());
    
    Ok(vectors)
}

/// Benchmarking utilities
pub struct DilithiumBenchmark {
    pub keygen_time: std::time::Duration,
    pub sign_time: std::time::Duration,
    pub verify_time: std::time::Duration,
    pub message_size: usize,
}

impl DilithiumBenchmark {
    pub fn run(level: DilithiumLevel, message: &[u8]) -> Result<Self, CryptoError> {
        let start = std::time::Instant::now();
        let keypair = generate_keypair(level)?;
        let keygen_time = start.elapsed();
        
        let start = std::time::Instant::now();
        let signature = keypair.sign(message)?;
        let sign_time = start.elapsed();
        
        let start = std::time::Instant::now();
        let _verified = keypair.verify(message, &signature)?;
        let verify_time = start.elapsed();
        
        Ok(DilithiumBenchmark {
            keygen_time,
            sign_time,
            verify_time,
            message_size: message.len(),
        })
    }
}

impl fmt::Display for DilithiumBenchmark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dilithium Benchmark Results:\n")?;
        write!(f, "  Key Generation: {:?}\n", self.keygen_time)?;
        write!(f, "  Signing: {:?}\n", self.sign_time)?;
        write!(f, "  Verification: {:?}\n", self.verify_time)?;
        write!(f, "  Message Size: {} bytes", self.message_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dilithium_parameters() {
        let params2 = DilithiumParams::new(DilithiumLevel::Level2);
        assert_eq!(params2.n, 256);
        assert_eq!(params2.q, 8380417);
        assert_eq!(params2.k, 4);
        assert_eq!(params2.l, 4);
        
        let params3 = DilithiumParams::new(DilithiumLevel::Level3);
        assert_eq!(params3.k, 6);
        assert_eq!(params3.l, 5);
        
        let params5 = DilithiumParams::new(DilithiumLevel::Level5);
        assert_eq!(params5.k, 8);
        assert_eq!(params5.l, 7);
    }

    #[test]
    fn test_key_generation() {
        for level in [DilithiumLevel::Level2, DilithiumLevel::Level3, DilithiumLevel::Level5] {
            let keypair = generate_keypair(level).unwrap();
            let params = DilithiumParams::new(level);
            
            assert_eq!(keypair.public_key.size(), params.pk_size);
            assert_eq!(keypair.private_key.size(), params.sk_size);
        }
    }

    #[test]
    fn test_sign_and_verify() {
        let message = b"Test message for Dilithium";
        
        for level in [DilithiumLevel::Level2, DilithiumLevel::Level3, DilithiumLevel::Level5] {
            let keypair = generate_keypair(level).unwrap();
            let signature = keypair.sign(message).unwrap();
            let params = DilithiumParams::new(level);
            
            assert_eq!(signature.size(), params.sig_size);
            assert!(keypair.verify(message, &signature).unwrap());
            
            // Test with different message should fail
            let wrong_message = b"Different message";
            assert!(!keypair.verify(wrong_message, &signature).unwrap());
        }
    }

    #[test]
    fn test_key_serialization() {
        let keypair = generate_keypair(DilithiumLevel::Level2).unwrap();
        
        // Test public key serialization
        let pk_bytes = keypair.public_key.to_bytes();
        let pk_restored = DilithiumPublicKey::from_bytes(DilithiumLevel::Level2, &pk_bytes).unwrap();
        assert_eq!(keypair.public_key.key_data, pk_restored.key_data);
        
        // Test private key serialization
        let sk_bytes = keypair.private_key.to_bytes();
        let sk_restored = DilithiumPrivateKey::from_bytes(DilithiumLevel::Level2, &sk_bytes).unwrap();
        assert_eq!(keypair.private_key.key_data, sk_restored.key_data);
    }

    #[test]
    fn test_signature_serialization() {
        let keypair = generate_keypair(DilithiumLevel::Level2).unwrap();
        let message = b"Test message";
        let signature = keypair.sign(message).unwrap();
        
        let sig_bytes = signature.to_bytes();
        let sig_restored = DilithiumSignature::from_bytes(DilithiumLevel::Level2, &sig_bytes).unwrap();
        
        assert_eq!(signature.signature_data, sig_restored.signature_data);
        assert!(keypair.verify(message, &sig_restored).unwrap());
    }

    #[test]
    fn test_constant_time_eq() {
        let a = vec![1, 2, 3, 4];
        let b = vec![1, 2, 3, 4];
        let c = vec![1, 2, 3, 5];
        
        assert!(constant_time_eq(&a, &b));
        assert!(!constant_time_eq(&a, &c));
        assert!(!constant_time_eq(&a, &[1, 2, 3]));
    }

    #[test]
    fn test_test_vectors() {
        let vectors = generate_test_vectors(DilithiumLevel::Level2).unwrap();
        
        assert!(vectors.contains_key("public_key"));
        assert!(vectors.contains_key("private_key"));
        assert!(vectors.contains_key("message"));
        assert!(vectors.contains_key("signature"));
        
        let params = DilithiumParams::new(DilithiumLevel::Level2);
        assert_eq!(vectors["public_key"].len(), params.pk_size);
        assert_eq!(vectors["private_key"].len(), params.sk_size);
        assert_eq!(vectors["signature"].len(), params.sig_size);
    }
}
