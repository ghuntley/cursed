/// fr fr NTRU encryption scheme implementation
/// 
/// NTRU is a lattice-based public key cryptosystem that relies on the hardness of
/// finding short vectors in lattices. It's one of the oldest post-quantum cryptographic
/// schemes and offers good performance characteristics.

use crate::error::CursedError;
use crate::stdlib::value::Value;
use crate::stdlib::packages::crypto_pqc::lattice_crypto::{LatticeError, SecureRng, LatticeRng};
use std::collections::HashMap;
use std::fmt;

/// fr fr NTRU configuration parameters
#[derive(Debug, Clone)]
pub struct NtruConfig {
    pub n: usize,           // Polynomial degree (security parameter)
    pub p: u16,             // Small modulus for encryption
    pub q: u16,             // Large modulus for key generation
    pub df: usize,          // Number of +1 coefficients in f
    pub dg: usize,          // Number of +1 coefficients in g  
    pub dr: usize,          // Number of +1 coefficients in r
    pub dm: usize,          // Number of +1 coefficients in message
    pub security_level: NtruSecurityLevel,
}

impl NtruConfig {
    /// slay Create NTRU config with secure defaults (NTRU-HPS-2048-509)
    pub fn new() -> Self {
        Self {
            n: 509,
            p: 3,
            q: 2048,
            df: 254,
            dg: 84,
            dr: 254,
            dm: 254,
            security_level: NtruSecurityLevel::Level128,
        }
    }
    
    /// bestie Create NTRU config for specific security level
    pub fn with_security_level(security_level: NtruSecurityLevel) -> Self {
        match security_level {
            NtruSecurityLevel::Level128 => Self {
                n: 509, p: 3, q: 2048, df: 254, dg: 84, dr: 254, dm: 254,
                security_level,
            },
            NtruSecurityLevel::Level192 => Self {
                n: 677, p: 3, q: 2048, df: 338, dg: 113, dr: 338, dm: 338,
                security_level,
            },
            NtruSecurityLevel::Level256 => Self {
                n: 821, p: 3, q: 4096, df: 410, dg: 136, dr: 410, dm: 410,
                security_level,
            },
        }
    }
    
    /// vibes Validate NTRU configuration
    pub fn validate(&self) -> Result<(), NtruError> {
        if self.n < 256 {
            return Err(NtruError::InvalidConfig("N must be at least 256 for security".to_string()));
        }
        
        if self.p < 2 || self.p >= self.q {
            return Err(NtruError::InvalidConfig("p must be >= 2 and < q".to_string()));
        }
        
        if self.q < 256 {
            return Err(NtruError::InvalidConfig("q must be at least 256".to_string()));
        }
        
        if self.df >= self.n || self.dg >= self.n || self.dr >= self.n {
            return Err(NtruError::InvalidConfig("Weight parameters must be < n".to_string()));
        }
        
        // Check that gcd(p, q) = 1
        if gcd(self.p as u64, self.q as u64) != 1 {
            return Err(NtruError::InvalidConfig("p and q must be coprime".to_string()));
        }
        
        Ok(())
    }
}

impl Default for NtruConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr NTRU security levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NtruSecurityLevel {
    Level128, // NTRU-HPS-2048-509
    Level192, // NTRU-HPS-2048-677
    Level256, // NTRU-HPS-4096-821
}

impl NtruSecurityLevel {
    pub fn bits(&self) -> u32 {
        match self {
            NtruSecurityLevel::Level128 => 128,
            NtruSecurityLevel::Level192 => 192,
            NtruSecurityLevel::Level256 => 256,
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            NtruSecurityLevel::Level128 => "NTRU-HPS-2048-509",
            NtruSecurityLevel::Level192 => "NTRU-HPS-2048-677",
            NtruSecurityLevel::Level256 => "NTRU-HPS-4096-821",
        }
    }
}

/// fr fr NTRU encryption engine
#[derive(Debug)]
pub struct NtruEngine {
    config: NtruConfig,
    rng: Box<dyn LatticeRng>,
    polynomial_ring: NtruPolynomialRing,
}

impl NtruEngine {
    /// slay Create new NTRU engine
    pub fn new(config: NtruConfig) -> Result<Self, NtruError> {
        config.validate()?;
        
        let rng = Box::new(SecureRng::new()
            .map_err(|e| NtruError::KeyGenerationError(format!("RNG initialization failed: {}", e)))?);
        let polynomial_ring = NtruPolynomialRing::new(config.n, config.q);
        
        Ok(Self {
            config,
            rng,
            polynomial_ring,
        })
    }
    
    /// bestie Generate NTRU key pair
    pub fn generate_keypair(&mut self) -> Result<NtruKeyPair, NtruError> {
        let n = self.config.n;
        let p = self.config.p;
        let q = self.config.q;
        let df = self.config.df;
        let dg = self.config.dg;
        
        // Step 1: Generate polynomials f and g
        let f = self.sample_ternary_polynomial(n, df, df)?;
        let g = self.sample_ternary_polynomial(n, dg, 0)?; // g has dg ones and 0 minus ones
        
        // Step 2: Compute f_q = f^(-1) mod q
        let f_q = self.polynomial_ring.invert_mod_q(&f)?;
        
        // Step 3: Compute f_p = f^(-1) mod p  
        let f_p = self.polynomial_ring.invert_mod_p(&f, p)?;
        
        // Step 4: Compute public key h = p * f_q * g mod q
        let pfq = self.polynomial_ring.scalar_multiply(&f_q, p as i32)?;
        let h = self.polynomial_ring.multiply_mod_q(&pfq, &g)?;
        
        // Step 5: Create key pair
        let public_key = NtruPublicKey {
            h: h.clone(),
            n,
            p,
            q,
        };
        
        let private_key = NtruPrivateKey {
            f,
            f_p,
            g: g.clone(),
            n,
            p,
            q,
        };
        
        Ok(NtruKeyPair {
            public_key,
            private_key,
            config: self.config.clone(),
        })
    }
    
    /// vibes Encrypt message using NTRU
    pub fn encrypt(&mut self, message: &[u8], public_key: &NtruPublicKey) -> Result<Vec<u8>, NtruError> {
        let n = public_key.n;
        let p = public_key.p;
        let q = public_key.q;
        
        // Convert message to polynomial
        let m = self.message_to_polynomial(message, n, p)?;
        
        // Sample random polynomial r
        let r = self.sample_ternary_polynomial(n, self.config.dr, self.config.dr)?;
        
        // Compute ciphertext e = r * h + m mod q
        let rh = self.polynomial_ring.multiply_mod_q(&r, &public_key.h)?;
        let e = self.polynomial_ring.add_mod_q(&rh, &m)?;
        
        // Convert polynomial to bytes
        self.polynomial_to_bytes(&e)
    }
    
    /// periodt Decrypt ciphertext using NTRU
    pub fn decrypt(&mut self, ciphertext: &[u8], private_key: &NtruPrivateKey) -> Result<Vec<u8>, NtruError> {
        let n = private_key.n;
        let p = private_key.p;
        let q = private_key.q;
        
        // Convert ciphertext to polynomial
        let e = self.bytes_to_polynomial(ciphertext, n)?;
        
        // Compute a = f * e mod q (centered reduction)
        let a = self.polynomial_ring.multiply_mod_q(&private_key.f, &e)?;
        let a_centered = self.polynomial_ring.center_reduction(&a, q)?;
        
        // Compute m = f_p * a mod p
        let m = self.polynomial_ring.multiply_mod_p(&private_key.f_p, &a_centered, p)?;
        
        // Convert polynomial to message bytes
        self.polynomial_to_message(&m, p)
    }
    
    /// sus Sample ternary polynomial with specified weights
    fn sample_ternary_polynomial(&mut self, n: usize, ones: usize, minus_ones: usize) -> Result<NtruPolynomial, NtruError> {
        if ones + minus_ones > n {
            return Err(NtruError::InvalidConfig("Too many non-zero coefficients".to_string()));
        }
        
        let mut coefficients = vec![0i32; n];
        let mut positions: Vec<usize> = (0..n).collect();
        
        // Shuffle positions
        for i in 0..n {
            let j = (self.rng.next_u32() as usize) % (n - i) + i;
            positions.swap(i, j);
        }
        
        // Set +1 coefficients
        for i in 0..ones {
            coefficients[positions[i]] = 1;
        }
        
        // Set -1 coefficients
        for i in ones..(ones + minus_ones) {
            coefficients[positions[i]] = -1;
        }
        
        Ok(NtruPolynomial::new(coefficients, n))
    }
    
    /// facts Convert message bytes to polynomial
    fn message_to_polynomial(&self, message: &[u8], n: usize, p: u16) -> Result<NtruPolynomial, NtruError> {
        let max_bytes = n * 2; // Rough estimate for capacity
        if message.len() > max_bytes {
            return Err(NtruError::MessageTooLong(format!("Message too long: {} > {}", message.len(), max_bytes)));
        }
        
        let mut coefficients = vec![0i32; n];
        
        // Simple encoding: each byte becomes multiple coefficients
        for (i, &byte) in message.iter().enumerate() {
            if i >= n {
                break;
            }
            coefficients[i] = (byte as i32) % (p as i32);
        }
        
        Ok(NtruPolynomial::new(coefficients, n))
    }
    
    /// yolo Convert polynomial to message bytes
    fn polynomial_to_message(&self, poly: &NtruPolynomial, p: u16) -> Result<Vec<u8>, NtruError> {
        let mut message = Vec::new();
        
        for &coeff in &poly.coefficients {
            let byte_val = ((coeff % p as i32) + p as i32) % p as i32;
            if byte_val < 256 {
                message.push(byte_val as u8);
            }
        }
        
        // Remove trailing zeros
        while let Some(&0) = message.last() {
            message.pop();
        }
        
        Ok(message)
    }
    
    /// stan Convert polynomial to bytes for transmission
    fn polynomial_to_bytes(&self, poly: &NtruPolynomial) -> Result<Vec<u8>, NtruError> {
        let mut bytes = Vec::new();
        
        for &coeff in &poly.coefficients {
            // Pack coefficients efficiently (simplified)
            let normalized = ((coeff % self.config.q as i32) + self.config.q as i32) % self.config.q as i32;
            bytes.extend_from_slice(&(normalized as u16).to_le_bytes());
        }
        
        Ok(bytes)
    }
    
    /// bestie Convert bytes to polynomial
    fn bytes_to_polynomial(&self, bytes: &[u8], n: usize) -> Result<NtruPolynomial, NtruError> {
        if bytes.len() != n * 2 {
            return Err(NtruError::InvalidCiphertext("Invalid ciphertext length".to_string()));
        }
        
        let mut coefficients = Vec::with_capacity(n);
        
        for chunk in bytes.chunks_exact(2) {
            let coeff = u16::from_le_bytes([chunk[0], chunk[1]]) as i32;
            coefficients.push(coeff);
        }
        
        Ok(NtruPolynomial::new(coefficients, n))
    }
    
    /// vibes Get configuration
    pub fn get_config(&self) -> &NtruConfig {
        &self.config
    }
}

/// fr fr NTRU polynomial representation
#[derive(Debug, Clone)]
pub struct NtruPolynomial {
    pub coefficients: Vec<i32>,
    pub degree: usize,
}

impl NtruPolynomial {
    /// slay Create new NTRU polynomial
    pub fn new(coefficients: Vec<i32>, degree: usize) -> Self {
        let mut normalized_coeffs = coefficients;
        normalized_coeffs.resize(degree, 0);
        
        Self {
            coefficients: normalized_coeffs,
            degree,
        }
    }
    
    /// bestie Check if polynomial is zero
    pub fn is_zero(&self) -> bool {
        self.coefficients.iter().all(|&c| c == 0)
    }
    
    /// vibes Get weight (number of non-zero coefficients)
    pub fn weight(&self) -> usize {
        self.coefficients.iter().filter(|&&c| c != 0).count()
    }
}

/// fr fr NTRU polynomial ring operations
#[derive(Debug)]
pub struct NtruPolynomialRing {
    n: usize,
    q: u16,
}

impl NtruPolynomialRing {
    /// slay Create new polynomial ring
    pub fn new(n: usize, q: u16) -> Self {
        Self { n, q }
    }
    
    /// bestie Add polynomials modulo q
    pub fn add_mod_q(&self, a: &NtruPolynomial, b: &NtruPolynomial) -> Result<NtruPolynomial, NtruError> {
        if a.degree != b.degree {
            return Err(NtruError::InvalidDimensions("Polynomial degrees don't match".to_string()));
        }
        
        let result_coeffs = a.coefficients.iter()
            .zip(b.coefficients.iter())
            .map(|(&a_i, &b_i)| (a_i + b_i) % self.q as i32)
            .collect();
        
        Ok(NtruPolynomial::new(result_coeffs, self.n))
    }
    
    /// vibes Multiply polynomials modulo q and x^n - 1
    pub fn multiply_mod_q(&self, a: &NtruPolynomial, b: &NtruPolynomial) -> Result<NtruPolynomial, NtruError> {
        if a.degree != b.degree {
            return Err(NtruError::InvalidDimensions("Polynomial degrees don't match".to_string()));
        }
        
        let mut result = vec![0i32; self.n];
        
        for (i, &a_i) in a.coefficients.iter().enumerate() {
            for (j, &b_j) in b.coefficients.iter().enumerate() {
                let pos = (i + j) % self.n; // Reduction modulo x^n - 1
                result[pos] = (result[pos] + a_i * b_j) % self.q as i32;
            }
        }
        
        Ok(NtruPolynomial::new(result, self.n))
    }
    
    /// periodt Multiply polynomial by scalar
    pub fn scalar_multiply(&self, poly: &NtruPolynomial, scalar: i32) -> Result<NtruPolynomial, NtruError> {
        let result_coeffs = poly.coefficients.iter()
            .map(|&c| (c * scalar) % self.q as i32)
            .collect();
        
        Ok(NtruPolynomial::new(result_coeffs, self.n))
    }
    
    /// sus Multiply polynomials modulo p
    pub fn multiply_mod_p(&self, a: &NtruPolynomial, b: &NtruPolynomial, p: u16) -> Result<NtruPolynomial, NtruError> {
        if a.degree != b.degree {
            return Err(NtruError::InvalidDimensions("Polynomial degrees don't match".to_string()));
        }
        
        let mut result = vec![0i32; self.n];
        
        for (i, &a_i) in a.coefficients.iter().enumerate() {
            for (j, &b_j) in b.coefficients.iter().enumerate() {
                let pos = (i + j) % self.n;
                result[pos] = (result[pos] + a_i * b_j) % p as i32;
            }
        }
        
        Ok(NtruPolynomial::new(result, self.n))
    }
    
    /// facts Compute polynomial inverse modulo q (simplified)
    pub fn invert_mod_q(&self, poly: &NtruPolynomial) -> Result<NtruPolynomial, NtruError> {
        // This is a simplified implementation
        // In practice, use extended Euclidean algorithm for polynomial rings
        if poly.is_zero() {
            return Err(NtruError::InversionError("Cannot invert zero polynomial".to_string()));
        }
        
        // For demonstration, assume we can compute inverse
        // In real implementation, this would use the extended Euclidean algorithm
        let mut result_coeffs = vec![0i32; self.n];
        result_coeffs[0] = 1; // Placeholder - this is not a real inverse
        
        Ok(NtruPolynomial::new(result_coeffs, self.n))
    }
    
    /// yolo Compute polynomial inverse modulo p
    pub fn invert_mod_p(&self, poly: &NtruPolynomial, p: u16) -> Result<NtruPolynomial, NtruError> {
        // Simplified implementation
        if poly.is_zero() {
            return Err(NtruError::InversionError("Cannot invert zero polynomial".to_string()));
        }
        
        let mut result_coeffs = vec![0i32; self.n];
        result_coeffs[0] = 1; // Placeholder
        
        Ok(NtruPolynomial::new(result_coeffs, self.n))
    }
    
    /// stan Center reduction for polynomial coefficients
    pub fn center_reduction(&self, poly: &NtruPolynomial, modulus: u16) -> Result<NtruPolynomial, NtruError> {
        let half_mod = modulus as i32 / 2;
        let result_coeffs = poly.coefficients.iter()
            .map(|&c| {
                let reduced = c % modulus as i32;
                if reduced > half_mod {
                    reduced - modulus as i32
                } else if reduced < -half_mod {
                    reduced + modulus as i32
                } else {
                    reduced
                }
            })
            .collect();
        
        Ok(NtruPolynomial::new(result_coeffs, self.n))
    }
}

/// fr fr NTRU key pair
#[derive(Debug, Clone)]
pub struct NtruKeyPair {
    pub public_key: NtruPublicKey,
    pub private_key: NtruPrivateKey,
    pub config: NtruConfig,
}

impl NtruKeyPair {
    /// slay Generate new NTRU key pair
    pub fn generate(config: &NtruConfig) -> Result<Self, NtruError> {
        let mut engine = NtruEngine::new(config.clone())?;
        engine.generate_keypair()
    }
    
    /// bestie Encrypt message with public key
    pub fn encrypt(&self, message: &[u8]) -> Result<Vec<u8>, NtruError> {
        let mut engine = NtruEngine::new(self.config.clone())?;
        engine.encrypt(message, &self.public_key)
    }
    
    /// vibes Decrypt ciphertext with private key
    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, NtruError> {
        let mut engine = NtruEngine::new(self.config.clone())?;
        engine.decrypt(ciphertext, &self.private_key)
    }
}

/// fr fr NTRU public key
#[derive(Debug, Clone)]
pub struct NtruPublicKey {
    pub h: NtruPolynomial,
    pub n: usize,
    pub p: u16,
    pub q: u16,
}

/// fr fr NTRU private key
#[derive(Debug, Clone)]
pub struct NtruPrivateKey {
    pub f: NtruPolynomial,
    pub f_p: NtruPolynomial,
    pub g: NtruPolynomial,
    pub n: usize,
    pub p: u16,
    pub q: u16,
}

/// fr fr NTRU errors
#[derive(Debug, Clone)]
pub enum NtruError {
    InvalidConfig(String),
    InvalidDimensions(String),
    KeyGenerationError(String),
    EncryptionError(String),
    DecryptionError(String),
    InversionError(String),
    MessageTooLong(String),
    InvalidCiphertext(String),
}

impl fmt::Display for NtruError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NtruError::InvalidConfig(msg) => write!(f, "NTRU configuration error: {}", msg),
            NtruError::InvalidDimensions(msg) => write!(f, "NTRU dimension error: {}", msg),
            NtruError::KeyGenerationError(msg) => write!(f, "NTRU key generation error: {}", msg),
            NtruError::EncryptionError(msg) => write!(f, "NTRU encryption error: {}", msg),
            NtruError::DecryptionError(msg) => write!(f, "NTRU decryption error: {}", msg),
            NtruError::InversionError(msg) => write!(f, "NTRU inversion error: {}", msg),
            NtruError::MessageTooLong(msg) => write!(f, "NTRU message too long: {}", msg),
            NtruError::InvalidCiphertext(msg) => write!(f, "NTRU invalid ciphertext: {}", msg),
        }
    }
}

impl std::error::Error for NtruError {}

impl From<NtruError> for CursedError {
    fn from(err: NtruError) -> Self {
        CursedError::CryptoError(err.to_string())
    }
}

impl From<LatticeError> for NtruError {
    fn from(err: LatticeError) -> Self {
        NtruError::KeyGenerationError(err.to_string())
    }
}

/// fr fr Utility function for GCD
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// fr fr NTRU utility functions
pub struct NtruUtils;

impl NtruUtils {
    /// slay Estimate NTRU security level
    pub fn estimate_security_level(config: &NtruConfig) -> f64 {
        // Simplified security estimation based on lattice dimension
        // Real estimation would consider attack algorithms like BKZ
        let log_n = (config.n as f64).log2();
        let log_q = (config.q as f64).log2();
        
        // Rough estimate: security grows with n and decreases with log q
        log_n * 15.0 - log_q * 2.0
    }
    
    /// bestie Validate NTRU parameters for production use
    pub fn validate_for_production(config: &NtruConfig) -> Result<SecurityValidation, NtruError> {
        let security_bits = Self::estimate_security_level(config);
        
        let is_secure = security_bits >= 128.0;
        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        
        if security_bits < 128.0 {
            warnings.push("Security level below 128 bits".to_string());
            recommendations.push("Increase n parameter for better security".to_string());
        }
        
        if config.q < 2048 {
            warnings.push("Small q value may affect security".to_string());
        }
        
        if gcd(config.p as u64, config.q as u64) != 1 {
            return Err(NtruError::InvalidConfig("p and q must be coprime".to_string()));
        }
        
        recommendations.push("Use constant-time implementations".to_string());
        recommendations.push("Implement proper random number generation".to_string());
        
        Ok(SecurityValidation {
            is_secure,
            estimated_security_bits: security_bits,
            warnings,
            recommendations,
            parameter_set: config.security_level.name().to_string(),
        })
    }
    
    /// vibes Generate test vectors for NTRU implementation
    pub fn generate_test_vectors(config: &NtruConfig) -> Result<NtruTestVectors, NtruError> {
        let mut engine = NtruEngine::new(config.clone())?;
        let keypair = engine.generate_keypair()?;
        
        let test_message = b"Hello, NTRU!";
        let ciphertext = engine.encrypt(test_message, &keypair.public_key)?;
        let decrypted = engine.decrypt(&ciphertext, &keypair.private_key)?;
        
        Ok(NtruTestVectors {
            config: config.clone(),
            public_key: keypair.public_key,
            private_key: keypair.private_key,
            message: test_message.to_vec(),
            ciphertext,
            decrypted_message: decrypted,
        })
    }
}

/// fr fr Security validation result
#[derive(Debug, Clone)]
pub struct SecurityValidation {
    pub is_secure: bool,
    pub estimated_security_bits: f64,
    pub warnings: Vec<String>,
    pub recommendations: Vec<String>,
    pub parameter_set: String,
}

/// fr fr NTRU test vectors
#[derive(Debug)]
pub struct NtruTestVectors {
    pub config: NtruConfig,
    pub public_key: NtruPublicKey,
    pub private_key: NtruPrivateKey,
    pub message: Vec<u8>,
    pub ciphertext: Vec<u8>,
    pub decrypted_message: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ntru_config_creation() {
        let config = NtruConfig::new();
        assert_eq!(config.n, 509);
        assert_eq!(config.p, 3);
        assert_eq!(config.q, 2048);
        assert_eq!(config.security_level, NtruSecurityLevel::Level128);
        
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_ntru_config_security_levels() {
        let config128 = NtruConfig::with_security_level(NtruSecurityLevel::Level128);
        assert_eq!(config128.n, 509);
        assert_eq!(config128.security_level.bits(), 128);
        
        let config192 = NtruConfig::with_security_level(NtruSecurityLevel::Level192);
        assert_eq!(config192.n, 677);
        assert_eq!(config192.security_level.bits(), 192);
        
        let config256 = NtruConfig::with_security_level(NtruSecurityLevel::Level256);
        assert_eq!(config256.n, 821);
        assert_eq!(config256.security_level.bits(), 256);
    }
    
    #[test]
    fn test_ntru_config_validation() {
        let mut config = NtruConfig::new();
        
        // Valid config should pass
        assert!(config.validate().is_ok());
        
        // Invalid n
        config.n = 100;
        assert!(config.validate().is_err());
        
        // Reset and test invalid p
        config.n = 509;
        config.p = 0;
        assert!(config.validate().is_err());
        
        // Reset and test p >= q
        config.p = 3;
        config.q = 2;
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_ntru_polynomial() {
        let poly = NtruPolynomial::new(vec![1, 2, 3, 0, 0], 5);
        assert_eq!(poly.degree, 5);
        assert_eq!(poly.weight(), 3);
        assert!(!poly.is_zero());
        
        let zero_poly = NtruPolynomial::new(vec![0, 0, 0], 3);
        assert!(zero_poly.is_zero());
    }
    
    #[test]
    fn test_ntru_polynomial_ring_operations() {
        let ring = NtruPolynomialRing::new(3, 7);
        
        let poly1 = NtruPolynomial::new(vec![1, 2, 3], 3);
        let poly2 = NtruPolynomial::new(vec![2, 1, 4], 3);
        
        // Test addition
        let sum = ring.add_mod_q(&poly1, &poly2).unwrap();
        assert_eq!(sum.coefficients, vec![3, 3, 0]); // (1+2, 2+1, 3+4) mod 7 = (3, 3, 0)
        
        // Test scalar multiplication
        let scaled = ring.scalar_multiply(&poly1, 2).unwrap();
        assert_eq!(scaled.coefficients, vec![2, 4, 6]);
    }
    
    #[test]
    fn test_ntru_engine_creation() {
        let config = NtruConfig::new();
        let engine = NtruEngine::new(config);
        assert!(engine.is_ok());
    }
    
    #[test]
    fn test_gcd_function() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(17, 13), 1);
        assert_eq!(gcd(3, 2048), 1);
    }
    
    #[test]
    fn test_security_estimation() {
        let config = NtruConfig::new();
        let security_bits = NtruUtils::estimate_security_level(&config);
        assert!(security_bits > 100.0); // Should provide reasonable security
    }
    
    #[test]
    fn test_security_validation() {
        let config = NtruConfig::new();
        let validation = NtruUtils::validate_for_production(&config).unwrap();
        
        assert!(validation.estimated_security_bits > 0.0);
        assert!(!validation.recommendations.is_empty());
        assert_eq!(validation.parameter_set, "NTRU-HPS-2048-509");
    }
    
    #[test]
    fn test_ntru_security_level_names() {
        assert_eq!(NtruSecurityLevel::Level128.name(), "NTRU-HPS-2048-509");
        assert_eq!(NtruSecurityLevel::Level192.name(), "NTRU-HPS-2048-677");
        assert_eq!(NtruSecurityLevel::Level256.name(), "NTRU-HPS-4096-821");
    }
}
