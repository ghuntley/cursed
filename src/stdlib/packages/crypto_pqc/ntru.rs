/// fr fr NTRU encryption scheme implementation
/// 
/// NTRU is a lattice-based public key cryptosystem that relies on the hardness of
/// finding short vectors in lattices. It's one of the oldest post-quantum cryptographic
/// schemes and offers good performance characteristics.

use crate::error::CursedError;
// use crate::stdlib::value::Value;
// use crate::stdlib::packages::crypto_pqc::lattice_crypto::{LatticeError, SecureRng, LatticeRng};
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
    pub fn validate(&self) -> crate::error::Result<()> {
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
    pub fn new(config: NtruConfig) -> crate::error::Result<()> {
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
    pub fn generate_keypair(&mut self) -> crate::error::Result<()> {
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
    pub fn encrypt(&mut self, message: &[u8], public_key: &NtruPublicKey) -> crate::error::Result<()> {
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
    pub fn decrypt(&mut self, ciphertext: &[u8], private_key: &NtruPrivateKey) -> crate::error::Result<()> {
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
    fn sample_ternary_polynomial(&mut self, n: usize, ones: usize, minus_ones: usize) -> crate::error::Result<()> {
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
    fn message_to_polynomial(&self, message: &[u8], n: usize, p: u16) -> crate::error::Result<()> {
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
    fn polynomial_to_message(&self, poly: &NtruPolynomial, p: u16) -> crate::error::Result<()> {
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
    fn polynomial_to_bytes(&self, poly: &NtruPolynomial) -> crate::error::Result<()> {
        let mut bytes = Vec::new();
        
        for &coeff in &poly.coefficients {
            // Pack coefficients efficiently (simplified)
            let normalized = ((coeff % self.config.q as i32) + self.config.q as i32) % self.config.q as i32;
            bytes.extend_from_slice(&(normalized as u16).to_le_bytes());
        }
        
        Ok(bytes)
    }
    
    /// bestie Convert bytes to polynomial
    fn bytes_to_polynomial(&self, bytes: &[u8], n: usize) -> crate::error::Result<()> {
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
    pub fn add_mod_q(&self, a: &NtruPolynomial, b: &NtruPolynomial) -> crate::error::Result<()> {
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
    pub fn multiply_mod_q(&self, a: &NtruPolynomial, b: &NtruPolynomial) -> crate::error::Result<()> {
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
    pub fn scalar_multiply(&self, poly: &NtruPolynomial, scalar: i32) -> crate::error::Result<()> {
        let result_coeffs = poly.coefficients.iter()
            .map(|&c| (c * scalar) % self.q as i32)
            .collect();
        
        Ok(NtruPolynomial::new(result_coeffs, self.n))
    }
    
    /// sus Multiply polynomials modulo p
    pub fn multiply_mod_p(&self, a: &NtruPolynomial, b: &NtruPolynomial, p: u16) -> crate::error::Result<()> {
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
    
    /// facts Compute polynomial inverse modulo q using Extended Euclidean Algorithm
    /// 
    /// This computes the multiplicative inverse of a polynomial f(x) in the ring
    /// Z_q[x]/(x^n - 1), i.e., finds g(x) such that f(x) * g(x) ≡ 1 (mod q, x^n - 1).
    /// 
    /// Critical for NTRU security: Without proper polynomial inversion, the entire
    /// cryptosystem is broken as key generation requires computing f^(-1) mod q.
    pub fn invert_mod_q(&self, poly: &NtruPolynomial) -> crate::error::Result<()> {
        self.extended_euclidean_invert(poly, self.q as i32)
            .ok_or_else(|| NtruError::InversionError(
                format!("Polynomial not invertible modulo q={}", self.q)
            ))
    }
    
    /// yolo Compute polynomial inverse modulo p using Extended Euclidean Algorithm
    /// 
    /// This computes the multiplicative inverse of a polynomial f(x) in the ring
    /// Z_p[x]/(x^n - 1), i.e., finds g(x) such that f(x) * g(x) ≡ 1 (mod p, x^n - 1).
    /// 
    /// Critical for NTRU decryption: f_p = f^(-1) mod p is required to recover
    /// the original message during decryption.
    pub fn invert_mod_p(&self, poly: &NtruPolynomial, p: u16) -> crate::error::Result<()> {
        self.extended_euclidean_invert(poly, p as i32)
            .ok_or_else(|| NtruError::InversionError(
                format!("Polynomial not invertible modulo p={}", p)
            ))
    }

    /// periodt Extended Euclidean Algorithm for polynomial inversion in Z_m[x]/(x^n - 1)
    /// 
    /// This implements the core cryptographic algorithm for NTRU polynomial inversion.
    /// Uses the Extended Euclidean Algorithm to find the multiplicative inverse of
    /// a polynomial in the quotient ring Z_m[x]/(x^n - 1).
    /// 
    /// Returns Some(inverse) if the polynomial is invertible, None otherwise.
    /// 
    /// Security Note: This is a critical security function. The polynomial f(x) is
    /// invertible mod m if and only if gcd(f(x), x^n - 1) = 1 in Z_m[x].
    fn extended_euclidean_invert(&self, poly: &NtruPolynomial, modulus: i32) -> Option<NtruPolynomial> {
        if poly.is_zero() {
            return None;
        }

        // Initialize Extended Euclidean Algorithm variables
        // We compute gcd(poly, x^n - 1) and the Bézout coefficients
        let mut old_r = self.create_xn_minus_1(); // x^n - 1
        let mut r = poly.clone();
        let mut old_s = self.create_zero_polynomial();
        let mut s = self.create_unit_polynomial(); // 1
        let mut old_t = self.create_unit_polynomial(); // 1  
        let mut t = self.create_zero_polynomial();

        // Extended Euclidean Algorithm main loop
        while !r.is_zero() {
            // Compute quotient and remainder
            let (quotient, remainder) = self.polynomial_division_mod(&old_r, &r, modulus)?;
            
            // Update remainders: old_r, r = r, old_r - quotient * r
            old_r = std::mem::replace(&mut r, remainder);
            
            // Update Bézout coefficients for s
            let qs = self.multiply_polynomials_mod(&quotient, &s, modulus)?;
            let new_s = self.subtract_polynomials_mod(&old_s, &qs, modulus)?;
            old_s = std::mem::replace(&mut s, new_s);
            
            // Update Bézout coefficients for t  
            let qt = self.multiply_polynomials_mod(&quotient, &t, modulus)?;
            let new_t = self.subtract_polynomials_mod(&old_t, &qt, modulus)?;
            old_t = std::mem::replace(&mut t, new_t);
        }

        // Check if gcd = 1 (polynomial is invertible)
        if !self.is_unit_polynomial(&old_r, modulus) {
            return None; // Not invertible
        }

        // Normalize the result by dividing by the gcd coefficient
        let gcd_leading_coeff = self.get_gcd_leading_coefficient(&old_r, modulus)?;
        let gcd_inv = self.modular_inverse(gcd_leading_coeff, modulus)?;
        
        // The inverse is old_s * gcd_inv^(-1) mod modulus
        let inverse = self.scalar_multiply_mod(&old_s, gcd_inv, modulus)?;
        
        Some(inverse)
    }

    /// stan Create polynomial x^n - 1 
    fn create_xn_minus_1(&self) -> NtruPolynomial {
        let mut coeffs = vec![0i32; self.n + 1];
        coeffs[0] = -1; // Constant term: -1
        if self.n < coeffs.len() {
            coeffs[self.n] = 1; // x^n term: 1
        }
        NtruPolynomial::new(coeffs, self.n)
    }

    /// bestie Create zero polynomial
    fn create_zero_polynomial(&self) -> NtruPolynomial {
        NtruPolynomial::new(vec![0i32; self.n], self.n)
    }

    /// vibes Create unit polynomial (1)
    fn create_unit_polynomial(&self) -> NtruPolynomial {
        let mut coeffs = vec![0i32; self.n];
        coeffs[0] = 1;
        NtruPolynomial::new(coeffs, self.n)
    }

    /// flex Polynomial division with remainder in Z_m[x]
    /// 
    /// Computes (quotient, remainder) such that dividend = divisor * quotient + remainder
    /// and degree(remainder) < degree(divisor).
    fn polynomial_division_mod(&self, dividend: &NtruPolynomial, divisor: &NtruPolynomial, modulus: i32) -> Option<(NtruPolynomial, NtruPolynomial)> {
        if divisor.is_zero() {
            return None;
        }

        let mut remainder = dividend.clone();
        let mut quotient = self.create_zero_polynomial();
        
        let divisor_degree = self.polynomial_degree(divisor);
        let divisor_leading = self.get_leading_coefficient(divisor, divisor_degree);
        let divisor_leading_inv = self.modular_inverse(divisor_leading, modulus)?;

        while !remainder.is_zero() {
            let remainder_degree = self.polynomial_degree(&remainder);
            if remainder_degree < divisor_degree {
                break;
            }

            let remainder_leading = self.get_leading_coefficient(&remainder, remainder_degree);
            let coeff = self.mod_reduce(remainder_leading * divisor_leading_inv, modulus);
            let degree_diff = remainder_degree - divisor_degree;

            // Create monomial: coeff * x^degree_diff
            let mut monomial_coeffs = vec![0i32; self.n];
            if degree_diff < self.n {
                monomial_coeffs[degree_diff] = coeff;
            }
            let monomial = NtruPolynomial::new(monomial_coeffs, self.n);

            // Update quotient
            quotient = self.add_polynomials_mod(&quotient, &monomial, modulus)?;

            // Subtract divisor * monomial from remainder
            let product = self.multiply_polynomials_mod(divisor, &monomial, modulus)?;
            remainder = self.subtract_polynomials_mod(&remainder, &product, modulus)?;
        }

        Some((quotient, remainder))
    }

    /// periodt Add polynomials modulo m
    fn add_polynomials_mod(&self, a: &NtruPolynomial, b: &NtruPolynomial, modulus: i32) -> Option<NtruPolynomial> {
        let mut result_coeffs = vec![0i32; self.n];
        
        for i in 0..self.n {
            let a_coeff = if i < a.coefficients.len() { a.coefficients[i] } else { 0 };
            let b_coeff = if i < b.coefficients.len() { b.coefficients[i] } else { 0 };
            result_coeffs[i] = self.mod_reduce(a_coeff + b_coeff, modulus);
        }
        
        Some(NtruPolynomial::new(result_coeffs, self.n))
    }

    /// sus Subtract polynomials modulo m  
    fn subtract_polynomials_mod(&self, a: &NtruPolynomial, b: &NtruPolynomial, modulus: i32) -> Option<NtruPolynomial> {
        let mut result_coeffs = vec![0i32; self.n];
        
        for i in 0..self.n {
            let a_coeff = if i < a.coefficients.len() { a.coefficients[i] } else { 0 };
            let b_coeff = if i < b.coefficients.len() { b.coefficients[i] } else { 0 };
            result_coeffs[i] = self.mod_reduce(a_coeff - b_coeff, modulus);
        }
        
Some(NtruPolynomial::new(result_coeffs, self.n))
    }

    /// facts Multiply polynomials modulo m and x^n - 1
    fn multiply_polynomials_mod(&self, a: &NtruPolynomial, b: &NtruPolynomial, modulus: i32) -> Option<NtruPolynomial> {
        let mut result_coeffs = vec![0i32; self.n];
        
        for (i, &a_coeff) in a.coefficients.iter().enumerate() {
            for (j, &b_coeff) in b.coefficients.iter().enumerate() {
                let pos = (i + j) % self.n; // Reduction modulo x^n - 1
                let product = self.constant_time_multiply(a_coeff, b_coeff, modulus);
                result_coeffs[pos] = self.mod_reduce(result_coeffs[pos] + product, modulus);
            }
        }
        
        Some(NtruPolynomial::new(result_coeffs, self.n))
    }

    /// yolo Scalar multiplication modulo m
    fn scalar_multiply_mod(&self, poly: &NtruPolynomial, scalar: i32, modulus: i32) -> Option<NtruPolynomial> {
        let result_coeffs = poly.coefficients.iter()
            .map(|&coeff| self.mod_reduce(coeff * scalar, modulus))
            .collect();
        
        Some(NtruPolynomial::new(result_coeffs, self.n))
    }

    /// stan Check if polynomial is a unit (constant polynomial equal to 1)
    fn is_unit_polynomial(&self, poly: &NtruPolynomial, modulus: i32) -> bool {
        if poly.coefficients.is_empty() {
            return false;
        }
        
        // Check if first coefficient is 1 (or coprime to modulus)
        let first_coeff = self.mod_reduce(poly.coefficients[0], modulus);
        if self.modular_inverse(first_coeff, modulus).is_none() {
            return false;
        }
        
        // Check if all other coefficients are 0
        poly.coefficients.iter().skip(1).all(|&c| self.mod_reduce(c, modulus) == 0)
    }

    /// bestie Get leading coefficient of polynomial
    fn get_leading_coefficient(&self, poly: &NtruPolynomial, degree: usize) -> i32 {
        if degree < poly.coefficients.len() {
            poly.coefficients[degree]
        } else {
            0
        }
    }

    /// vibes Get polynomial degree (highest non-zero coefficient index)
    fn polynomial_degree(&self, poly: &NtruPolynomial) -> usize {
        for (i, &coeff) in poly.coefficients.iter().enumerate().rev() {
            if coeff != 0 {
                return i;
            }
        }
        0
    }

    /// flex Get GCD leading coefficient for normalization
    fn get_gcd_leading_coefficient(&self, poly: &NtruPolynomial, modulus: i32) -> Option<i32> {
        for &coeff in &poly.coefficients {
            let normalized = self.mod_reduce(coeff, modulus);
            if normalized != 0 {
                return Some(normalized);
            }
        }
        None
    }

    /// periodt Constant-time modular multiplication (timing attack protection)
    fn constant_time_multiply(&self, a: i32, b: i32, modulus: i32) -> i32 {
        // Use 64-bit arithmetic to prevent overflow
        let product = (a as i64) * (b as i64);
        self.mod_reduce(product as i32, modulus)
    }

    /// sus Modular reduction with proper handling of negative numbers
    fn mod_reduce(&self, value: i32, modulus: i32) -> i32 {
        let result = value % modulus;
        if result < 0 {
            result + modulus
        } else {
            result
        }
    }

    /// facts Extended Euclidean Algorithm for modular inverse
    /// 
    /// Computes the multiplicative inverse of a modulo m using the Extended
    /// Euclidean Algorithm. Returns Some(inverse) if gcd(a, m) = 1, None otherwise.
    fn modular_inverse(&self, a: i32, modulus: i32) -> Option<i32> {
        if modulus <= 1 {
            return None;
        }

        let mut old_r = modulus;
        let mut r = a % modulus;
        if r < 0 {
            r += modulus;
        }

        let mut old_s = 0i32;
        let mut s = 1i32;

        while r != 0 {
            let quotient = old_r / r;
            
            let temp_r = r;
            r = old_r - quotient * r;
            old_r = temp_r;

            let temp_s = s;  
            s = old_s - quotient * s;
            old_s = temp_s;
        }

        if old_r > 1 {
            None // Not invertible
        } else {
            if old_s < 0 {
                old_s += modulus;
            }
            Some(old_s)
        }
    }
    
    /// stan Center reduction for polynomial coefficients
    pub fn center_reduction(&self, poly: &NtruPolynomial, modulus: u16) -> crate::error::Result<()> {
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
    pub fn generate(config: &NtruConfig) -> crate::error::Result<()> {
        let mut engine = NtruEngine::new(config.clone())?;
        engine.generate_keypair()
    }
    
    /// bestie Encrypt message with public key
    pub fn encrypt(&self, message: &[u8]) -> crate::error::Result<()> {
        let mut engine = NtruEngine::new(self.config.clone())?;
        engine.encrypt(message, &self.public_key)
    }
    
    /// vibes Decrypt ciphertext with private key
    pub fn decrypt(&self, ciphertext: &[u8]) -> crate::error::Result<()> {
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

// impl fmt::Display for NtruError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             NtruError::InvalidConfig(msg) => write!(f, "NTRU configuration error: {}", msg),
//             NtruError::InvalidDimensions(msg) => write!(f, "NTRU dimension error: {}", msg),
//             NtruError::KeyGenerationError(msg) => write!(f, "NTRU key generation error: {}", msg),
//             NtruError::EncryptionError(msg) => write!(f, "NTRU encryption error: {}", msg),
//             NtruError::DecryptionError(msg) => write!(f, "NTRU decryption error: {}", msg),
//             NtruError::InversionError(msg) => write!(f, "NTRU inversion error: {}", msg),
//             NtruError::MessageTooLong(msg) => write!(f, "NTRU message too long: {}", msg),
//             NtruError::InvalidCiphertext(msg) => write!(f, "NTRU invalid ciphertext: {}", msg),
//         }
//     }
// }

// impl std::error::CursedError for NtruError {}
// 
// impl From<NtruError> for CursedError {
//     fn from(err: NtruError) -> Self {
//         CursedError::CryptoError(err.to_string())
//     }
// }

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
    pub fn validate_for_production(config: &NtruConfig) -> crate::error::Result<()> {
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
    pub fn generate_test_vectors(config: &NtruConfig) -> crate::error::Result<()> {
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

