/// fr fr Lattice-based cryptography foundation for PQC
/// 
/// This module provides the mathematical foundation for lattice-based post-quantum
/// cryptographic algorithms including Learning With Errors (LWE), Ring-LWE, and
/// Module-LWE problems that form the basis for algorithms like Kyber, Dilithium, and NTRU.

use crate::error::CursedError;
use crate::stdlib::value::Value;
use crate::error::Error;
use std::collections::HashMap;
use std::fmt;

/// Result type for lattice operations
pub type LatticeResult<T> = Result<T, LatticeError>;

/// Lattice cryptography errors
#[derive(Debug, Clone)]
pub enum LatticeError {
    InvalidConfig(String),
    InvalidDimension(String),
    InvalidDimensions(String),
    InvalidModulus(String),
    KeyGenerationFailed(String),
    EncryptionFailed(String),
    DecryptionFailed(String),
    InvalidInput(String),
    ComputationError(String),
    SamplingError(String),
    Internal(String),
}

impl fmt::Display for LatticeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LatticeError::InvalidConfig(msg) => write!(f, "Invalid configuration: {}", msg),
            LatticeError::InvalidDimension(msg) => write!(f, "Invalid dimension: {}", msg),
            LatticeError::InvalidDimensions(msg) => write!(f, "Invalid dimensions: {}", msg),
            LatticeError::InvalidModulus(msg) => write!(f, "Invalid modulus: {}", msg),
            LatticeError::KeyGenerationFailed(msg) => write!(f, "Key generation failed: {}", msg),
            LatticeError::EncryptionFailed(msg) => write!(f, "Encryption failed: {}", msg),
            LatticeError::DecryptionFailed(msg) => write!(f, "Decryption failed: {}", msg),
            LatticeError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            LatticeError::ComputationError(msg) => write!(f, "Computation error: {}", msg),
            LatticeError::SamplingError(msg) => write!(f, "Sampling error: {}", msg),
            LatticeError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for LatticeError {}

/// fr fr Lattice cryptography configuration
#[derive(Debug, Clone)]
pub struct LatticeConfig {
    pub dimension: usize,
    pub modulus: u64,
    pub error_distribution_stddev: f64,
    pub security_level: LatticeSecurityLevel,
    pub variant: LatticeVariant,
}

impl LatticeConfig {
    /// slay Create lattice config with secure defaults
    pub fn new() -> Self {
        Self {
            dimension: 512,
            modulus: 3329, // Prime modulus commonly used in Kyber
            error_distribution_stddev: 1.0,
            security_level: LatticeSecurityLevel::Level128,
            variant: LatticeVariant::ModuleLwe,
        }
    }
    
    /// bestie Create lattice config for specific security level
    pub fn with_security_level(security_level: LatticeSecurityLevel) -> Self {
        let (dimension, modulus) = match security_level {
            LatticeSecurityLevel::Level128 => (512, 3329),
            LatticeSecurityLevel::Level192 => (768, 3329),
            LatticeSecurityLevel::Level256 => (1024, 3329),
        };
        
        Self {
            dimension,
            modulus,
            error_distribution_stddev: 1.0,
            security_level,
            variant: LatticeVariant::ModuleLwe,
        }
    }
    
    /// vibes Validate lattice configuration
    pub fn validate(&self) -> Result<(), LatticeError> {
        if self.dimension < 256 {
            return Err(LatticeError::InvalidConfig("Dimension must be at least 256 for security".to_string()));
        }
        
        if self.modulus < 256 {
            return Err(LatticeError::InvalidConfig("Modulus must be at least 256".to_string()));
        }
        
        if self.error_distribution_stddev <= 0.0 || self.error_distribution_stddev > 10.0 {
            return Err(LatticeError::InvalidConfig("Error distribution standard deviation must be in range (0, 10]".to_string()));
        }
        
        Ok(())
    }
}

impl Default for LatticeConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Lattice security levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LatticeSecurityLevel {
    Level128, // 128-bit classical security
    Level192, // 192-bit classical security
    Level256, // 256-bit classical security
}

impl LatticeSecurityLevel {
    pub fn bits(&self) -> u32 {
        match self {
            LatticeSecurityLevel::Level128 => 128,
            LatticeSecurityLevel::Level192 => 192,
            LatticeSecurityLevel::Level256 => 256,
        }
    }
}

/// fr fr Lattice problem variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LatticeVariant {
    /// Learning With Errors (standard LWE)
    Lwe,
    /// Ring Learning With Errors
    RingLwe,
    /// Module Learning With Errors
    ModuleLwe,
    /// Short Integer Solution
    Sis,
}

/// fr fr Lattice cryptography engine
#[derive(Debug)]
pub struct LatticeEngine {
    config: LatticeConfig,
    rng: Box<dyn LatticeRng>,
    polynomial_ring: PolynomialRing,
    gaussian_sampler: GaussianSampler,
}

impl LatticeEngine {
    /// slay Create new lattice engine
    pub fn new(config: LatticeConfig) -> Result<(), Error> {
        config.validate()?;
        
        let rng = Box::new(SecureRng::new()?);
        let polynomial_ring = PolynomialRing::new(config.dimension, config.modulus);
        let gaussian_sampler = GaussianSampler::new(config.error_distribution_stddev, 1000);
        
        Ok(Self {
            config,
            rng,
            polynomial_ring,
            gaussian_sampler,
        })
    }
    
    /// bestie Generate LWE problem instance
    pub fn generate_lwe_instance(&mut self) -> Result<(), Error> {
        let dimension = self.config.dimension;
        let modulus = self.config.modulus;
        
        // Generate random secret vector
        let secret = self.sample_secret_vector(dimension)?;
        
        // Generate random matrix A
        let matrix_a = self.sample_random_matrix(dimension, dimension)?;
        
        // Generate error vector
        let error = self.sample_error_vector(dimension)?;
        
        // Compute b = A*s + e (mod q)
        let b = self.compute_lwe_sample(&matrix_a, &secret, &error)?;
        
        Ok(LweInstance {
            matrix_a,
            vector_b: b,
            secret: Some(secret),
            error: Some(error),
            dimension,
            modulus,
        })
    }
    
    /// vibes Generate Ring-LWE problem instance
    pub fn generate_ring_lwe_instance(&mut self) -> Result<(), Error> {
        let degree = self.config.dimension;
        let modulus = self.config.modulus;
        
        // Generate random polynomial a(x)
        let poly_a = self.sample_random_polynomial(degree)?;
        
        // Generate secret polynomial s(x)
        let secret = self.sample_secret_polynomial(degree)?;
        
        // Generate error polynomial e(x)
        let error = self.sample_error_polynomial(degree)?;
        
        // Compute b(x) = a(x) * s(x) + e(x) (mod q)
        let poly_b = self.polynomial_ring.multiply_and_add(&poly_a, &secret, &error)?;
        
        Ok(RingLweInstance {
            poly_a,
            poly_b,
            secret: Some(secret),
            error: Some(error),
            degree,
            modulus,
        })
    }
    
    /// periodt Sample secret vector for LWE
    pub fn sample_secret_vector(&mut self, dimension: usize) -> Result<(), Error> {
        let mut secret = Vec::with_capacity(dimension);
        for _ in 0..dimension {
            // Sample from {-1, 0, 1} for security
            let value = match self.rng.next_u32() % 3 {
                0 => -1,
                1 => 0,
                _ => 1,
            };
            secret.push(value);
        }
        Ok(secret)
    }
    
    /// sus Sample error vector from Gaussian distribution
    pub fn sample_error_vector(&mut self, dimension: usize) -> Result<(), Error> {
        self.gaussian_sampler.sample_vector(dimension)
    }
    
    /// facts Sample random matrix
    pub fn sample_random_matrix(&mut self, rows: usize, cols: usize) -> Result<(), Error> {
        let mut matrix = Vec::with_capacity(rows);
        for _ in 0..rows {
            let mut row = Vec::with_capacity(cols);
            for _ in 0..cols {
                let value = (self.rng.next_u64() % self.config.modulus) as i64;
                row.push(value);
            }
            matrix.push(row);
        }
        Ok(matrix)
    }
    
    /// yolo Sample random polynomial
    pub fn sample_random_polynomial(&mut self, degree: usize) -> Result<(), Error> {
        let mut coefficients = Vec::with_capacity(degree);
        for _ in 0..degree {
            let coeff = (self.rng.next_u64() % self.config.modulus) as i64;
            coefficients.push(coeff);
        }
        Ok(Polynomial::new(coefficients, self.config.modulus))
    }
    
    /// stan Sample secret polynomial
    pub fn sample_secret_polynomial(&mut self, degree: usize) -> Result<(), Error> {
        let mut coefficients = Vec::with_capacity(degree);
        for _ in 0..degree {
            // Sample from small coefficients for security
            let coeff = match self.rng.next_u32() % 3 {
                0 => -1,
                1 => 0,
                _ => 1,
            };
            coefficients.push(coeff);
        }
        Ok(Polynomial::new(coefficients, self.config.modulus))
    }
    
    /// bestie Sample error polynomial
    pub fn sample_error_polynomial(&mut self, degree: usize) -> Result<(), Error> {
        let error_coeffs = self.gaussian_sampler.sample_vector(degree)?;
        Ok(Polynomial::new(error_coeffs, self.config.modulus))
    }
    
    /// vibes Compute LWE sample b = A*s + e
    fn compute_lwe_sample(&self, matrix_a: &[Vec<i64>], secret: &[i64], error: &[i64]) -> Result<(), Error> {
        if matrix_a.len() != secret.len() || secret.len() != error.len() {
            return Err(LatticeError::InvalidDimensions("Matrix and vector dimensions don't match".to_string()));
        }
        
        let mut result = Vec::with_capacity(matrix_a.len());
        for (i, row) in matrix_a.iter().enumerate() {
            let mut dot_product = 0i64;
            for (j, &a_ij) in row.iter().enumerate() {
                dot_product = (dot_product + a_ij * secret[j]) % self.config.modulus as i64;
            }
            let b_i = (dot_product + error[i]) % self.config.modulus as i64;
            result.push(if b_i < 0 { b_i + self.config.modulus as i64 } else { b_i });
        }
        Ok(result)
    }
    
    /// periodt Get configuration
    pub fn get_config(&self) -> &LatticeConfig {
        &self.config
    }
}

/// fr fr LWE problem instance
#[derive(Debug, Clone)]
pub struct LweInstance {
    pub matrix_a: Vec<Vec<i64>>,
    pub vector_b: Vec<i64>,
    pub secret: Option<Vec<i64>>,
    pub error: Option<Vec<i64>>,
    pub dimension: usize,
    pub modulus: u64,
}

/// fr fr Ring-LWE problem instance
#[derive(Debug, Clone)]
pub struct RingLweInstance {
    pub poly_a: Polynomial,
    pub poly_b: Polynomial,
    pub secret: Option<Polynomial>,
    pub error: Option<Polynomial>,
    pub degree: usize,
    pub modulus: u64,
}

/// fr fr Polynomial structure for Ring-LWE
#[derive(Debug, Clone)]
pub struct Polynomial {
    pub coefficients: Vec<i64>,
    pub modulus: u64,
}

impl Polynomial {
    /// slay Create new polynomial
    pub fn new(coefficients: Vec<i64>, modulus: u64) -> Self {
        let normalized_coeffs = coefficients.iter()
            .map(|&c| {
                let reduced = c % modulus as i64;
                if reduced < 0 { reduced + modulus as i64 } else { reduced }
            })
            .collect();
        
        Self {
            coefficients: normalized_coeffs,
            modulus,
        }
    }
    
    /// bestie Add polynomials
    pub fn add(&self, other: &Polynomial) -> Result<(), Error> {
        if self.coefficients.len() != other.coefficients.len() {
            return Err(LatticeError::InvalidDimensions("Polynomial degrees don't match".to_string()));
        }
        
        let result_coeffs = self.coefficients.iter()
            .zip(other.coefficients.iter())
            .map(|(&a, &b)| (a + b) % self.modulus as i64)
            .collect();
        
        Ok(Polynomial::new(result_coeffs, self.modulus))
    }
    
    /// vibes Multiply polynomials (simplified for demonstration)
    pub fn multiply(&self, other: &Polynomial) -> Result<(), Error> {
        let degree = self.coefficients.len();
        let mut result = vec![0i64; degree];
        
        for (i, &a) in self.coefficients.iter().enumerate() {
            for (j, &b) in other.coefficients.iter().enumerate() {
                if i + j < degree {
                    result[i + j] = (result[i + j] + a * b) % self.modulus as i64;
                }
            }
        }
        
        Ok(Polynomial::new(result, self.modulus))
    }
}

/// fr fr Polynomial ring operations
#[derive(Debug)]
pub struct PolynomialRing {
    degree: usize,
    modulus: u64,
}

impl PolynomialRing {
    /// slay Create new polynomial ring
    pub fn new(degree: usize, modulus: u64) -> Self {
        Self { degree, modulus }
    }
    
    /// bestie Multiply and add polynomials: a*b + c
    pub fn multiply_and_add(&self, a: &Polynomial, b: &Polynomial, c: &Polynomial) -> Result<(), Error> {
        let product = a.multiply(b)?;
        product.add(c)
    }
}

/// fr fr Gaussian sampler for error distribution
#[derive(Debug)]
pub struct GaussianSampler {
    stddev: f64,
    precision: usize,
}

impl GaussianSampler {
    /// slay Create new Gaussian sampler
    pub fn new(stddev: f64, precision: usize) -> Self {
        Self { stddev, precision }
    }
    
    /// bestie Sample vector from Gaussian distribution
    pub fn sample_vector(&self, length: usize) -> Result<(), Error> {
        let mut result = Vec::with_capacity(length);
        for _ in 0..length {
            // Simplified Gaussian sampling (Box-Muller transform)
            let value = self.sample_gaussian();
            result.push(value);
        }
        Ok(result)
    }
    
    /// vibes Sample single Gaussian value
    fn sample_gaussian(&self) -> i64 {
        // Simplified implementation using rejection sampling
        // In production, use proper discrete Gaussian sampling
        use std::f64::consts::PI;
        
        let u1: f64 = rand::random();
        let u2: f64 = rand::random();
        
        let z = (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).cos();
        let scaled = z * self.stddev;
        
        // Round and clamp to reasonable range
        let result = scaled.round() as i64;
        result.clamp(-1000, 1000) // Practical bounds
    }
}

/// fr fr Secure random number generator trait
pub trait LatticeRng {
    fn next_u32(&mut self) -> u32;
    fn next_u64(&mut self) -> u64;
}

/// fr fr Secure RNG implementation
#[derive(Debug)]
pub struct SecureRng {
    state: u64,
}

impl SecureRng {
    /// slay Create new secure RNG
    pub fn new() -> Result<(), Error> {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| LatticeError::InvalidConfig("Failed to initialize RNG".to_string()))?
            .as_nanos() as u64;
        
        Ok(Self { state: seed })
    }
}

impl LatticeRng for SecureRng {
    fn next_u32(&mut self) -> u32 {
        // Simple LCG for demonstration (use proper CSPRNG in production)
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
        (self.state >> 16) as u32
    }
    
    fn next_u64(&mut self) -> u64 {
        let high = self.next_u32() as u64;
        let low = self.next_u32() as u64;
        (high << 32) | low
    }
}



impl From<LatticeError> for CursedError {
    fn from(err: LatticeError) -> Self {
        CursedError::CryptoError(err.to_string())
    }
}

/// fr fr Lattice utility functions
pub struct LatticeUtils;

impl LatticeUtils {
    /// slay Compute hardness of LWE instance
    pub fn estimate_lwe_hardness(dimension: usize, modulus: u64, stddev: f64) -> f64 {
        // Simplified hardness estimation based on dimension and noise
        let log_q = (modulus as f64).log2();
        let noise_ratio = stddev / (modulus as f64);
        
        // Rough estimate: hardness grows exponentially with dimension
        // and decreases with noise ratio
        dimension as f64 * log_q - 10.0 * noise_ratio.log2()
    }
    
    /// bestie Check if parameters provide sufficient security
    pub fn validate_security_parameters(config: &LatticeConfig) -> Result<(), Error> {
        let hardness = Self::estimate_lwe_hardness(
            config.dimension,
            config.modulus,
            config.error_distribution_stddev,
        );
        
        let security_level = if hardness >= 256.0 {
            LatticeSecurityLevel::Level256
        } else if hardness >= 192.0 {
            LatticeSecurityLevel::Level192
        } else if hardness >= 128.0 {
            LatticeSecurityLevel::Level128
        } else {
            return Err(LatticeError::InvalidConfig("Parameters provide insufficient security".to_string()));
        };
        
        Ok(SecurityAssessment {
            estimated_hardness: hardness,
            security_level,
            is_secure: hardness >= 128.0,
            recommendations: Self::generate_recommendations(config, hardness),
        })
    }
    
    /// vibes Generate security recommendations
    fn generate_recommendations(config: &LatticeConfig, hardness: f64) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if hardness < 128.0 {
            recommendations.push("Increase dimension for better security".to_string());
        }
        
        if config.error_distribution_stddev > 2.0 {
            recommendations.push("Consider reducing error distribution for better security".to_string());
        }
        
        if config.modulus < 1024 {
            recommendations.push("Consider using larger modulus for production use".to_string());
        }
        
        recommendations.push("Use constant-time implementations for side-channel resistance".to_string());
        recommendations.push("Implement proper random number generation".to_string());
        
        recommendations
    }
}

/// fr fr Security assessment result
#[derive(Debug, Clone)]
pub struct SecurityAssessment {
    pub estimated_hardness: f64,
    pub security_level: LatticeSecurityLevel,
    pub is_secure: bool,
    pub recommendations: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lattice_config_creation() {
        let config = LatticeConfig::new();
        assert_eq!(config.dimension, 512);
        assert_eq!(config.modulus, 3329);
        assert_eq!(config.security_level, LatticeSecurityLevel::Level128);
        
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_lattice_config_security_levels() {
        let config128 = LatticeConfig::with_security_level(LatticeSecurityLevel::Level128);
        assert_eq!(config128.dimension, 512);
        
        let config192 = LatticeConfig::with_security_level(LatticeSecurityLevel::Level192);
        assert_eq!(config192.dimension, 768);
        
        let config256 = LatticeConfig::with_security_level(LatticeSecurityLevel::Level256);
        assert_eq!(config256.dimension, 1024);
    }
    
    #[test]
    fn test_lattice_config_validation() {
        let mut config = LatticeConfig::new();
        
        // Valid config should pass
        assert!(config.validate().is_ok());
        
        // Invalid dimension
        config.dimension = 100;
        assert!(config.validate().is_err());
        
        // Reset and test invalid modulus
        config.dimension = 512;
        config.modulus = 10;
        assert!(config.validate().is_err());
        
        // Reset and test invalid stddev
        config.modulus = 3329;
        config.error_distribution_stddev = 0.0;
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_lattice_engine_creation() {
        let config = LatticeConfig::new();
        let engine = LatticeEngine::new(config);
        assert!(engine.is_ok());
    }
    
    #[test]
    fn test_polynomial_operations() {
        let poly1 = Polynomial::new(vec![1, 2, 3], 5);
        let poly2 = Polynomial::new(vec![2, 1, 4], 5);
        
        let sum = poly1.add(&poly2).unwrap();
        assert_eq!(sum.coefficients, vec![3, 3, 2]); // (1+2, 2+1, 3+4) mod 5
        
        let product = poly1.multiply(&poly2).unwrap();
        assert!(product.coefficients.len() == 3); // Same degree for simplified multiply
    }
    
    #[test]
    fn test_gaussian_sampler() {
        let sampler = GaussianSampler::new(1.0, 1000);
        let samples = sampler.sample_vector(10).unwrap();
        assert_eq!(samples.len(), 10);
        
        // Check that samples are in reasonable range
        assert!(samples.iter().all(|&x| x >= -1000 && x <= 1000));
    }
    
    #[test]
    fn test_secure_rng() {
        let mut rng = SecureRng::new().unwrap();
        let val1 = rng.next_u32();
        let val2 = rng.next_u32();
        
        // Should produce different values
        assert_ne!(val1, val2);
    }
    
    #[test]
    fn test_security_assessment() {
        let config = LatticeConfig::new();
        let assessment = LatticeUtils::validate_security_parameters(&config).unwrap();
        
        assert!(assessment.estimated_hardness > 0.0);
        assert!(assessment.is_secure);
        assert!(!assessment.recommendations.is_empty());
    }
    
    #[test]
    fn test_lwe_hardness_estimation() {
        let hardness = LatticeUtils::estimate_lwe_hardness(512, 3329, 1.0);
        assert!(hardness > 100.0); // Should provide reasonable security
        
        let weak_hardness = LatticeUtils::estimate_lwe_hardness(256, 256, 10.0);
        assert!(weak_hardness < hardness); // Weaker parameters
    }
}
