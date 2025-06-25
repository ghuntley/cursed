/// fr fr SABER key encapsulation mechanism implementation
/// 
/// SABER is a lattice-based key encapsulation mechanism (KEM) that was a finalist
/// in the NIST Post-Quantum Cryptography standardization process. It's based on
/// the Module Learning With Rounding (Mod-LWR) problem.

use crate::error::CursedError;
// use crate::stdlib::value::Value;
// use crate::stdlib::packages::crypto_pqc::lattice_crypto::{SecureRng, LatticeRng, LatticeError};
use std::collections::HashMap;
use std::fmt;

/// fr fr SABER configuration parameters
#[derive(Debug, Clone)]
pub struct SaberConfig {
    pub l: usize,           // Module dimension
    pub n: usize,           // Polynomial degree
    pub q: u32,             // Modulus q
    pub p: u32,             // Modulus p (for rounding)
    pub t: u32,             // Modulus t (for message encoding)
    pub mu: usize,          // Message length in bits
    pub epsilon_q: usize,   // Log2(q)
    pub epsilon_p: usize,   // Log2(p)
    pub epsilon_t: usize,   // Log2(t)
impl SaberConfig {
    /// slay Create SABER config with secure defaults (LightSaber)
    pub fn new() -> Self {
        Self::lightsaber()
    /// bestie LightSaber parameters (NIST Level 1)
    pub fn lightsaber() -> Self {
        Self {
            l: 2,         // Module dimension
            n: 256,       // Polynomial degree
            q: 8192,      // 2^13
            p: 1024,      // 2^10
            t: 4,         // 2^2
            mu: 256,      // Message length
        }
    }
    
    /// vibes Saber parameters (NIST Level 3)
    pub fn saber() -> Self {
        Self {
            l: 3,         // Module dimension
            n: 256,       // Polynomial degree
            q: 8192,      // 2^13
            p: 1024,      // 2^10
            t: 8,         // 2^3
            mu: 256,      // Message length
        }
    }
    
    /// periodt FireSaber parameters (NIST Level 5)
    pub fn firesaber() -> Self {
        Self {
            l: 4,         // Module dimension
            n: 256,       // Polynomial degree
            q: 8192,      // 2^13
            p: 1024,      // 2^10
            t: 16,        // 2^4
            mu: 256,      // Message length
        }
    }
    
    /// sus Create SABER config for specific variant
    pub fn with_variant(variant: SaberVariant) -> Self {
        match variant {
        }
    }
    
    /// facts Validate SABER configuration
    pub fn validate(&self) -> crate::error::Result<()> {
        if self.l == 0 || self.l > 10 {
            return Err(SaberError::InvalidConfig("l must be between 1 and 10".to_string()));
        if self.n == 0 || !self.n.is_power_of_two() || self.n > 1024 {
            return Err(SaberError::InvalidConfig("n must be power of 2 between 1 and 1024".to_string()));
        if !self.q.is_power_of_two() || self.q < 256 {
            return Err(SaberError::InvalidConfig("q must be power of 2 >= 256".to_string()));
        if !self.p.is_power_of_two() || self.p >= self.q {
            return Err(SaberError::InvalidConfig("p must be power of 2 and < q".to_string()));
        if !self.t.is_power_of_two() || self.t < 2 {
            return Err(SaberError::InvalidConfig("t must be power of 2 >= 2".to_string()));
        if self.mu == 0 || self.mu > 512 {
            return Err(SaberError::InvalidConfig("mu must be between 1 and 512".to_string()));
        // Verify logarithm consistency
        if (1u32 << self.epsilon_q) != self.q {
            return Err(SaberError::InvalidConfig("epsilon_q doesn't match q".to_string()));
        if (1u32 << self.epsilon_p) != self.p {
            return Err(SaberError::InvalidConfig("epsilon_p doesn't match p".to_string()));
        if (1u32 << self.epsilon_t) != self.t {
            return Err(SaberError::InvalidConfig("epsilon_t doesn't match t".to_string()));
        Ok(())
    /// yolo Calculate public key size
    pub fn public_key_size(&self) -> usize {
        self.l * self.n * self.epsilon_p / 8 // In bytes
    /// stan Calculate private key size
    pub fn private_key_size(&self) -> usize {
        self.l * self.n * self.epsilon_q / 8 // In bytes
    /// bestie Calculate ciphertext size
    pub fn ciphertext_size(&self) -> usize {
        let ct_size = self.l * self.n * self.epsilon_p / 8;
        let cm_size = self.mu * self.epsilon_t / 8;
        ct_size + cm_size
    /// vibes Calculate shared secret size
    pub fn shared_secret_size(&self) -> usize {
        32 // 256 bits for all variants
    }
}

impl Default for SaberConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr SABER variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaberVariant {
    LightSaber, // NIST Level 1 security
    Saber,      // NIST Level 3 security
    FireSaber,  // NIST Level 5 security
impl SaberVariant {
    pub fn name(&self) -> &'static str {
        match self {
        }
    }
    
    pub fn parameter_set(&self) -> &'static str {
        match self {
        }
    }
/// fr fr SABER security levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaberSecurityLevel {
    Level128, // 128-bit classical security (NIST Level 1)
    Level192, // 192-bit classical security (NIST Level 3)
    Level256, // 256-bit classical security (NIST Level 5)
impl SaberSecurityLevel {
    pub fn bits(&self) -> u32 {
        match self {
        }
    }
/// fr fr SABER engine
#[derive(Debug)]
pub struct SaberEngine {
impl SaberEngine {
    /// slay Create new SABER engine
    pub fn new(config: SaberConfig) -> crate::error::Result<()> {
        config.validate()?;
        
        let rng = Box::new(SecureRng::new()
            .map_err(|e| SaberError::InitializationError(format!("RNG initialization failed: {}", e)))?);
        let polynomial_ring = SaberPolynomialRing::new(config.n, config.q);
        
        Ok(Self {
        })
    /// bestie Generate SABER key pair
    pub fn generate_keypair(&mut self) -> crate::error::Result<()> {
        let l = self.config.l;
        let n = self.config.n;
        let q = self.config.q;
        let p = self.config.p;
        
        // Step 1: Sample matrix A from uniform distribution
        let matrix_a = self.sample_uniform_matrix(l, l, n, q)?;
        
        // Step 2: Sample secret vector s from centered binomial distribution
        let secret_s = self.sample_secret_vector(l, n)?;
        
        // Step 3: Compute b = A * s + h (mod q), then round to get public key
        let as_product = self.matrix_vector_multiply(&matrix_a, &secret_s)?;
        let noise_h = self.sample_noise_vector(l, n)?;
        let b_full = self.add_vectors(&as_product, &noise_h)?;
        
        // Step 4: Round from Z_q to Z_p to get public key
        let public_b = self.round_vector(&b_full, q, p)?;
        
        // Step 5: Serialize public key (A is public parameter, only store b)
        let public_key = SaberPublicKey {
        
        let private_key = SaberPrivateKey {
        
        Ok(SaberKeyPair {
        })
    /// vibes Encapsulate (generate shared secret and ciphertext)
    pub fn encapsulate(&mut self, public_key: &SaberPublicKey) -> crate::error::Result<()> {
        let l = self.config.l;
        let n = self.config.n;
        let q = self.config.q;
        let p = self.config.p;
        let t = self.config.t;
        let mu = self.config.mu;
        
        // Step 1: Generate random message m
        let message_m = self.generate_random_message(mu)?;
        
        // Step 2: Sample matrix A (same as in key generation)
        let matrix_a = self.sample_uniform_matrix(l, l, n, q)?;
        
        // Step 3: Sample ephemeral secret r from centered binomial distribution
        let secret_r = self.sample_secret_vector(l, n)?;
        
        // Step 4: Compute u = A^T * r + h' (mod q), then round to Z_p
        let at_r_product = self.matrix_transpose_vector_multiply(&matrix_a, &secret_r)?;
        let noise_h_prime = self.sample_noise_vector(l, n)?;
        let u_full = self.add_vectors(&at_r_product, &noise_h_prime)?;
        let u = self.round_vector(&u_full, q, p)?;
        
        // Step 5: Compute v = b^T * r + h'' + m * (q/t) (mod q), then round to Z_t
        let bt_r_product = self.vector_dot_product(&public_key.b, &secret_r)?;
        let noise_h_double_prime = self.sample_noise_scalar(n)?;
        let message_scaled = self.scale_message(&message_m, q, t)?;
        let v_temp = self.add_polynomials(&bt_r_product, &noise_h_double_prime)?;
        let v_full = self.add_polynomials(&v_temp, &message_scaled)?;
        let v = self.round_polynomial(&v_full, q, t)?;
        
        // Step 6: Derive shared secret from message
        let shared_secret = self.derive_shared_secret(&message_m)?;
        
        // Step 7: Serialize ciphertext
        let ciphertext = self.serialize_ciphertext(&u, &v)?;
        
        Ok((shared_secret, ciphertext))
    /// periodt Decapsulate (recover shared secret from ciphertext)
    pub fn decapsulate(&mut self, ciphertext: &[u8], private_key: &SaberPrivateKey) -> crate::error::Result<()> {
        let l = self.config.l;
        let n = self.config.n;
        let q = self.config.q;
        let t = self.config.t;
        
        // Step 1: Deserialize ciphertext
        let (u, v) = self.deserialize_ciphertext(ciphertext)?;
        
        // Step 2: Compute w = v - s^T * u (mod q)
        let st_u_product = self.vector_dot_product(&private_key.s, &u)?;
        let w = self.subtract_polynomials(&v, &st_u_product)?;
        
        // Step 3: Round w to recover message
        let recovered_message = self.recover_message(&w, q, t)?;
        
        // Step 4: Derive shared secret from recovered message
        let shared_secret = self.derive_shared_secret(&recovered_message)?;
        
        Ok(shared_secret)
    /// sus Sample uniform matrix A
    fn sample_uniform_matrix(&mut self, rows: usize, cols: usize, n: usize, modulus: u32) -> crate::error::Result<()> {
        let mut matrix = Vec::new();
        
        for _ in 0..rows {
            let mut row = Vec::new();
            for _ in 0..cols {
                let poly = self.sample_uniform_polynomial(n, modulus)?;
                row.push(poly);
            }
            matrix.push(row);
        Ok(matrix)
    /// facts Sample uniform polynomial
    fn sample_uniform_polynomial(&mut self, degree: usize, modulus: u32) -> crate::error::Result<()> {
        let mut coefficients = Vec::with_capacity(degree);
        
        for _ in 0..degree {
            let coeff = (self.rng.next_u32() % modulus) as i32;
            coefficients.push(coeff);
        Ok(SaberPolynomial::new(coefficients, modulus))
    /// yolo Sample secret vector from centered binomial distribution
    fn sample_secret_vector(&mut self, length: usize, degree: usize) -> crate::error::Result<()> {
        let mut vector = Vec::new();
        
        for _ in 0..length {
            let poly = self.sample_centered_binomial_polynomial(degree)?;
            vector.push(poly);
        Ok(vector)
    /// stan Sample centered binomial polynomial
    fn sample_centered_binomial_polynomial(&mut self, degree: usize) -> crate::error::Result<()> {
        let mut coefficients = Vec::with_capacity(degree);
        
        for _ in 0..degree {
            // Sample from centered binomial distribution CBD_eta
            // Simplified: sample two uniform bits and compute difference
            let a = (self.rng.next_u32() & 1) as i32;
            let b = (self.rng.next_u32() & 1) as i32;
            let coeff = a - b; // Results in {-1, 0, 1}
            coefficients.push(coeff);
        Ok(SaberPolynomial::new(coefficients, self.config.q))
    /// bestie Sample noise vector
    fn sample_noise_vector(&mut self, length: usize, degree: usize) -> crate::error::Result<()> {
        // For SABER, noise is sampled from centered binomial distribution
        self.sample_secret_vector(length, degree)
    /// vibes Sample noise scalar (single polynomial)
    fn sample_noise_scalar(&mut self, degree: usize) -> crate::error::Result<()> {
        self.sample_centered_binomial_polynomial(degree)
    /// periodt Matrix-vector multiplication
    fn matrix_vector_multiply(&self, matrix: &[Vec<SaberPolynomial>], vector: &[SaberPolynomial]) -> crate::error::Result<()> {
        if matrix[0].len() != vector.len() {
            return Err(SaberError::DimensionError("Matrix-vector dimensions don't match".to_string()));
        let mut result = Vec::new();
        
        for row in matrix {
            let mut sum = SaberPolynomial::zero(self.config.n, self.config.q);
            
            for (matrix_elem, vector_elem) in row.iter().zip(vector.iter()) {
                let product = self.polynomial_ring.multiply(matrix_elem, vector_elem)?;
                sum = self.polynomial_ring.add(&sum, &product)?;
            result.push(sum);
        Ok(result)
    /// sus Matrix transpose-vector multiplication
    fn matrix_transpose_vector_multiply(&self, matrix: &[Vec<SaberPolynomial>], vector: &[SaberPolynomial]) -> crate::error::Result<()> {
        if matrix.len() != vector.len() {
            return Err(SaberError::DimensionError("Matrix transpose-vector dimensions don't match".to_string()));
        let cols = matrix[0].len();
        let mut result = Vec::new();
        
        for j in 0..cols {
            let mut sum = SaberPolynomial::zero(self.config.n, self.config.q);
            
            for (i, vector_elem) in vector.iter().enumerate() {
                let product = self.polynomial_ring.multiply(&matrix[i][j], vector_elem)?;
                sum = self.polynomial_ring.add(&sum, &product)?;
            result.push(sum);
        Ok(result)
    /// facts Vector dot product
    fn vector_dot_product(&self, vector1: &[SaberPolynomial], vector2: &[SaberPolynomial]) -> crate::error::Result<()> {
        if vector1.len() != vector2.len() {
            return Err(SaberError::DimensionError("Vector dimensions don't match".to_string()));
        let mut result = SaberPolynomial::zero(self.config.n, self.config.q);
        
        for (elem1, elem2) in vector1.iter().zip(vector2.iter()) {
            let product = self.polynomial_ring.multiply(elem1, elem2)?;
            result = self.polynomial_ring.add(&result, &product)?;
        Ok(result)
    /// yolo Add vectors
    fn add_vectors(&self, vector1: &[SaberPolynomial], vector2: &[SaberPolynomial]) -> crate::error::Result<()> {
        if vector1.len() != vector2.len() {
            return Err(SaberError::DimensionError("Vector dimensions don't match".to_string()));
        let mut result = Vec::new();
        
        for (elem1, elem2) in vector1.iter().zip(vector2.iter()) {
            let sum = self.polynomial_ring.add(elem1, elem2)?;
            result.push(sum);
        Ok(result)
    /// stan Add polynomials
    fn add_polynomials(&self, poly1: &SaberPolynomial, poly2: &SaberPolynomial) -> crate::error::Result<()> {
        self.polynomial_ring.add(poly1, poly2)
    /// bestie Subtract polynomials
    fn subtract_polynomials(&self, poly1: &SaberPolynomial, poly2: &SaberPolynomial) -> crate::error::Result<()> {
        self.polynomial_ring.subtract(poly1, poly2)
    /// vibes Round vector from Z_q to Z_p
    fn round_vector(&self, vector: &[SaberPolynomial], from_mod: u32, to_mod: u32) -> crate::error::Result<()> {
        let mut result = Vec::new();
        
        for poly in vector {
            let rounded = self.round_polynomial(poly, from_mod, to_mod)?;
            result.push(rounded);
        Ok(result)
    /// periodt Round polynomial from one modulus to another
    fn round_polynomial(&self, poly: &SaberPolynomial, from_mod: u32, to_mod: u32) -> crate::error::Result<()> {
        let scale_factor = from_mod / to_mod;
        let mut rounded_coeffs = Vec::new();
        
        for &coeff in &poly.coefficients {
            let rounded = ((coeff as u32 + scale_factor / 2) / scale_factor) % to_mod;
            rounded_coeffs.push(rounded as i32);
        Ok(SaberPolynomial::new(rounded_coeffs, to_mod))
    /// sus Generate random message
    fn generate_random_message(&mut self, length_bits: usize) -> crate::error::Result<()> {
        let length_bytes = (length_bits + 7) / 8;
        let mut message = vec![0u8; length_bytes];
        
        for byte in &mut message {
            *byte = (self.rng.next_u32() & 0xFF) as u8;
        Ok(message)
    /// facts Scale message for encoding
    fn scale_message(&self, message: &[u8], q: u32, t: u32) -> crate::error::Result<()> {
        let scale_factor = q / t;
        let mut coefficients = Vec::new();
        
        for &byte in message {
            for i in 0..8 {
                let bit = (byte >> i) & 1;
                let scaled_bit = (bit as u32 * scale_factor) as i32;
                coefficients.push(scaled_bit);
                
                if coefficients.len() >= self.config.n {
                    break;
                }
            }
            if coefficients.len() >= self.config.n {
                break;
            }
        }
        
        // Pad with zeros if necessary
        coefficients.resize(self.config.n, 0);
        
        Ok(SaberPolynomial::new(coefficients, q))
    /// yolo Recover message from polynomial
    fn recover_message(&self, poly: &SaberPolynomial, q: u32, t: u32) -> crate::error::Result<()> {
        let scale_factor = q / t;
        let threshold = scale_factor / 2;
        let mut bits = Vec::new();
        
        for &coeff in &poly.coefficients {
            let normalized = ((coeff as u32) + threshold) / scale_factor;
            let bit = (normalized % 2) as u8;
            bits.push(bit);
        // Convert bits to bytes
        let mut message = Vec::new();
        for chunk in bits.chunks(8) {
            let mut byte = 0u8;
            for (i, &bit) in chunk.iter().enumerate() {
                byte |= bit << i;
            }
            message.push(byte);
        Ok(message)
    /// stan Derive shared secret from message
    fn derive_shared_secret(&self, message: &[u8]) -> crate::error::Result<()> {
        // Use a simple hash of the message as shared secret
        // In practice, use a proper key derivation function
        let mut hash_input = message.to_vec();
        hash_input.extend_from_slice(b"SABER_SHARED_SECRET");
        
        // Simplified hash function (use proper cryptographic hash in production)
        let mut secret = vec![0u8; 32];
        for (i, &byte) in hash_input.iter().enumerate() {
            secret[i % 32] ^= byte.wrapping_add(i as u8);
        Ok(secret)
    /// bestie Serialize ciphertext
    fn serialize_ciphertext(&self, u: &[SaberPolynomial], v: &SaberPolynomial) -> crate::error::Result<()> {
        let mut ciphertext = Vec::new();
        
        // Serialize u vector
        for poly in u {
            let poly_bytes = self.serialize_polynomial(poly)?;
            ciphertext.extend(poly_bytes);
        // Serialize v polynomial
        let v_bytes = self.serialize_polynomial(v)?;
        ciphertext.extend(v_bytes);
        
        Ok(ciphertext)
    /// vibes Deserialize ciphertext
    fn deserialize_ciphertext(&self, ciphertext: &[u8]) -> crate::error::Result<()> {
        let poly_size = self.config.n * self.config.epsilon_p / 8;
        let u_size = self.config.l * poly_size;
        
        if ciphertext.len() < u_size + poly_size {
            return Err(SaberError::InvalidCiphertext("Ciphertext too short".to_string()));
        // Deserialize u vector
        let mut u = Vec::new();
        for i in 0..self.config.l {
            let start = i * poly_size;
            let end = start + poly_size;
            let poly = self.deserialize_polynomial(&ciphertext[start..end], self.config.p)?;
            u.push(poly);
        // Deserialize v polynomial
        let v_start = u_size;
        let v_end = v_start + poly_size;
        let v = self.deserialize_polynomial(&ciphertext[v_start..v_end], self.config.t)?;
        
        Ok((u, v))
    /// periodt Serialize polynomial
    fn serialize_polynomial(&self, poly: &SaberPolynomial) -> crate::error::Result<()> {
        let mut bytes = Vec::new();
        
        // Pack coefficients efficiently (simplified)
        for &coeff in &poly.coefficients {
            let normalized = ((coeff % poly.modulus as i32) + poly.modulus as i32) % poly.modulus as i32;
            bytes.extend_from_slice(&(normalized as u16).to_le_bytes());
        Ok(bytes)
    /// sus Deserialize polynomial
    fn deserialize_polynomial(&self, bytes: &[u8], modulus: u32) -> crate::error::Result<()> {
        let expected_len = self.config.n * 2; // 2 bytes per coefficient
        if bytes.len() != expected_len {
            return Err(SaberError::InvalidCiphertext("Invalid polynomial serialization".to_string()));
        let mut coefficients = Vec::new();
        
        for chunk in bytes.chunks_exact(2) {
            let coeff = u16::from_le_bytes([chunk[0], chunk[1]]) as i32;
            coefficients.push(coeff % modulus as i32);
        Ok(SaberPolynomial::new(coefficients, modulus))
    /// facts Get configuration
    pub fn get_config(&self) -> &SaberConfig {
        &self.config
    }
}

/// fr fr SABER polynomial representation
#[derive(Debug, Clone)]
pub struct SaberPolynomial {
impl SaberPolynomial {
    /// slay Create new SABER polynomial
    pub fn new(coefficients: Vec<i32>, modulus: u32) -> Self {
        let degree = coefficients.len();
        Self {
        }
    }
    
    /// bestie Create zero polynomial
    pub fn zero(degree: usize, modulus: u32) -> Self {
        Self::new(vec![0; degree], modulus)
    /// vibes Check if polynomial is zero
    pub fn is_zero(&self) -> bool {
        self.coefficients.iter().all(|&c| c == 0)
    }
}

/// fr fr SABER polynomial ring operations
#[derive(Debug)]
pub struct SaberPolynomialRing {
impl SaberPolynomialRing {
    /// slay Create new polynomial ring
    pub fn new(degree: usize, modulus: u32) -> Self {
        Self { degree, modulus }
    }
    
    /// bestie Add polynomials
    pub fn add(&self, a: &SaberPolynomial, b: &SaberPolynomial) -> crate::error::Result<()> {
        if a.degree != b.degree {
            return Err(SaberError::DimensionError("Polynomial degrees don't match".to_string()));
        let result_coeffs = a.coefficients.iter()
            .zip(b.coefficients.iter())
            .map(|(&a_i, &b_i)| (a_i + b_i) % self.modulus as i32)
            .collect();
        
        Ok(SaberPolynomial::new(result_coeffs, self.modulus))
    /// vibes Subtract polynomials
    pub fn subtract(&self, a: &SaberPolynomial, b: &SaberPolynomial) -> crate::error::Result<()> {
        if a.degree != b.degree {
            return Err(SaberError::DimensionError("Polynomial degrees don't match".to_string()));
        let result_coeffs = a.coefficients.iter()
            .zip(b.coefficients.iter())
            .map(|(&a_i, &b_i)| {
                let diff = (a_i - b_i) % self.modulus as i32;
                if diff < 0 { diff + self.modulus as i32 } else { diff }
            })
            .collect();
        
        Ok(SaberPolynomial::new(result_coeffs, self.modulus))
    /// periodt Multiply polynomials modulo x^n + 1
    pub fn multiply(&self, a: &SaberPolynomial, b: &SaberPolynomial) -> crate::error::Result<()> {
        if a.degree != b.degree {
            return Err(SaberError::DimensionError("Polynomial degrees don't match".to_string()));
        let mut result = vec![0i32; self.degree];
        
        for (i, &a_i) in a.coefficients.iter().enumerate() {
            for (j, &b_j) in b.coefficients.iter().enumerate() {
                let pos = i + j;
                if pos < self.degree {
                    result[pos] = (result[pos] + a_i * b_j) % self.modulus as i32;
                } else {
                    // Reduction modulo x^n + 1: x^(n+k) = -x^k
                    let reduced_pos = pos - self.degree;
                    result[reduced_pos] = (result[reduced_pos] - a_i * b_j) % self.modulus as i32;
                    if result[reduced_pos] < 0 {
                        result[reduced_pos] += self.modulus as i32;
                    }
                }
            }
        }
        
        Ok(SaberPolynomial::new(result, self.modulus))
    }
}

/// fr fr SABER key pair
#[derive(Debug)]
pub struct SaberKeyPair {
impl SaberKeyPair {
    /// slay Generate new SABER key pair
    pub fn generate(config: &SaberConfig) -> crate::error::Result<()> {
        let mut engine = SaberEngine::new(config.clone())?;
        engine.generate_keypair()
    /// bestie Encapsulate shared secret
    pub fn encapsulate(&self) -> crate::error::Result<()> {
        let mut engine = SaberEngine::new(self.config.clone())?;
        engine.encapsulate(&self.public_key)
    /// vibes Decapsulate shared secret
    pub fn decapsulate(&self, ciphertext: &[u8]) -> crate::error::Result<()> {
        let mut engine = SaberEngine::new(self.config.clone())?;
        engine.decapsulate(ciphertext, &self.private_key)
    }
}

/// fr fr SABER public key
#[derive(Debug, Clone)]
pub struct SaberPublicKey {
/// fr fr SABER private key
#[derive(Debug, Clone)]
pub struct SaberPrivateKey {
/// fr fr SABER errors
#[derive(Debug, Clone)]
pub enum SaberError {
// impl fmt::Display for SaberError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             SaberError::InvalidConfig(msg) => write!(f, "SABER configuration error: {}", msg),
//             SaberError::InitializationError(msg) => write!(f, "SABER initialization error: {}", msg),
//             SaberError::KeyGenerationError(msg) => write!(f, "SABER key generation error: {}", msg),
//             SaberError::EncapsulationError(msg) => write!(f, "SABER encapsulation error: {}", msg),
//             SaberError::DecapsulationError(msg) => write!(f, "SABER decapsulation error: {}", msg),
//             SaberError::DimensionError(msg) => write!(f, "SABER dimension error: {}", msg),
//             SaberError::InvalidCiphertext(msg) => write!(f, "SABER invalid ciphertext: {}", msg),
//             SaberError::PolynomialError(msg) => write!(f, "SABER polynomial error: {}", msg),
//         }
//     }
// }

// impl std::error::CursedError for SaberError {}
// 
// impl From<SaberError> for CursedError {
//     fn from(err: SaberError) -> Self {
//         CursedError::CryptoError(err.to_string())
//     }
// }

impl From<LatticeError> for SaberError {
    fn from(err: LatticeError) -> Self {
        SaberError::InitializationError(err.to_string())
    }
}

/// fr fr SABER utility functions
pub struct SaberUtils;

impl SaberUtils {
    /// slay Estimate security level for SABER parameters
    pub fn estimate_security_level(config: &SaberConfig) -> f64 {
        // Simplified security estimation based on Mod-LWR hardness
        let n = config.n as f64;
        let l = config.l as f64;
        let log_q = (config.q as f64).log2();
        let log_p = (config.p as f64).log2();
        
        // Rough estimate: security grows with dimension and decreases with modulus ratio
        let dimension_factor = (n * l).log2();
        let modulus_factor = log_q - log_p;
        
        dimension_factor * 15.0 - modulus_factor * 5.0
    /// bestie Validate SABER parameters for production
    pub fn validate_for_production(config: &SaberConfig) -> crate::error::Result<()> {
        let security_bits = Self::estimate_security_level(config);
        let is_secure = security_bits >= 128.0;
        
        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        
        if security_bits < 128.0 {
            warnings.push("Security level below 128 bits".to_string());
            recommendations.push("Use higher security variant".to_string());
        if config.n < 256 {
            warnings.push("Small polynomial degree may affect security".to_string());
        if config.l < 2 {
            warnings.push("Small module dimension may be vulnerable".to_string());
        let key_sizes = (config.public_key_size(), config.private_key_size());
        if key_sizes.0 > 10000 || key_sizes.1 > 10000 {
            warnings.push("Large key sizes may affect performance".to_string());
        recommendations.push("Use constant-time implementations".to_string());
        recommendations.push("Implement proper random number generation".to_string());
        recommendations.push("Consider NTT optimizations for performance".to_string());
        
        Ok(SaberSecurityValidation {
        })
    /// vibes Compare SABER variants
    pub fn compare_variants() -> Vec<VariantComparison> {
        vec![
            VariantComparison {
            VariantComparison {
            VariantComparison {
        ]
    }
}

/// fr fr SABER security validation result
#[derive(Debug, Clone)]
pub struct SaberSecurityValidation {
/// fr fr Variant comparison result
#[derive(Debug, Clone)]
pub struct VariantComparison {
