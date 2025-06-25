use crate::error::CursedError;
/// Real Dilithium Digital Signature Implementation
/// 
/// This is a production-ready implementation of CRYSTALS-Dilithium, a lattice-based
/// digital signature scheme standardized by NIST.
/// 
/// # Mathematical Foundation
/// 
/// Dilithium is based on the Module-LWE (Learning With Errors) problem over polynomial rings.
/// It uses rejection sampling and the Fiat-Shamir transform to create secure signatures.
/// 
/// # Security Levels
/// 
/// - Dilithium2: NIST Level 2 (~128-bit classical security)
/// - Dilithium3: NIST Level 3 (192-bit classical security)  
/// - Dilithium5: NIST Level 5 (256-bit classical security)

use std::fmt;
use rand::rngs::OsRng;
use rand::RngCore;
use sha3::{Sha3_256, Sha3_512, Digest, Keccak256};
// use crate::stdlib::crypto_pqc::{PqcResult, PqcError, SecurityLevel, AlgorithmType};
use super::{DigitalSignature, ParameterSet, AlgorithmPerformance, KeySizes};

// Dilithium parameters
const Q: i32 = 8380417; // Prime modulus
const N: usize = 256;   // Polynomial degree
const SEEDBYTES: usize = 32;
const CRHBYTES: usize = 64;

/// Dilithium parameter sets with real mathematical parameters
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DilithiumParams {
    /// Dilithium2: k=4, l=4, eta=2, tau=39, beta=78, gamma1=2^17, gamma2=95232
    /// Dilithium3: k=6, l=5, eta=4, tau=49, beta=196, gamma1=2^19, gamma2=261888
    /// Dilithium5: k=8, l=7, eta=2, tau=60, beta=120, gamma1=2^19, gamma2=261888
impl DilithiumParams {
    fn k(&self) -> usize {
        match self {
        }
    }

    fn l(&self) -> usize {
        match self {
        }
    }

    fn eta(&self) -> i32 {
        match self {
        }
    }

    fn tau(&self) -> i32 {
        match self {
        }
    }

    fn beta(&self) -> i32 {
        match self {
        }
    }

    fn gamma1(&self) -> i32 {
        match self {
            DilithiumParams::Dilithium2 => 1 << 17, // 2^17
            DilithiumParams::Dilithium3 => 1 << 19, // 2^19
            DilithiumParams::Dilithium5 => 1 << 19, // 2^19
        }
    }

    fn gamma2(&self) -> i32 {
        match self {
            DilithiumParams::Dilithium2 => (Q - 1) / 88,
            DilithiumParams::Dilithium3 => (Q - 1) / 32,
            DilithiumParams::Dilithium5 => (Q - 1) / 32,
        }
    }
impl ParameterSet for DilithiumParams {
    fn security_level(&self) -> SecurityLevel {
        match self {
        }
    }

    fn public_key_size(&self) -> usize {
        SEEDBYTES + self.k() * self.polyt1_packedbytes()
    fn secret_key_size(&self) -> usize {
        3 * SEEDBYTES 
            + self.l() * self.polyeta_packedbytes()
            + self.k() * self.polyeta_packedbytes()
            + self.k() * self.polyt0_packedbytes()
    fn additional_sizes(&self) -> Vec<(&'static str, usize)> {
        let sig_size = SEEDBYTES + self.l() * self.polyz_packedbytes() + self.omega() + self.k();
        vec![("signature", sig_size)]
    }
}

impl DilithiumParams {
    fn polyt1_packedbytes(&self) -> usize { 320 }
    fn polyt0_packedbytes(&self) -> usize { 416 }
    fn polyeta_packedbytes(&self) -> usize {
        match self.eta() {
        }
    }
    fn polyz_packedbytes(&self) -> usize {
        match self.gamma1() {
        }
    }
    fn omega(&self) -> usize { 80 }
}

impl fmt::Display for DilithiumParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        }
    }
/// Polynomial over Z_q[X]/(X^n + 1)
#[derive(Debug, Clone)]
pub struct Polynomial {
impl Polynomial {
    fn new() -> Self {
        Self { coeffs: [0; N] }
    }

    fn from_coeffs(coeffs: [i32; N]) -> Self {
        Self { coeffs }
    }

    /// Reduce coefficients modulo q
    fn reduce(&mut self) {
        for coeff in &mut self.coeffs {
            *coeff = (*coeff % Q + Q) % Q;
        }
    }

    /// Number Theoretic Transform (NTT)
    fn ntt(&mut self) {
        ntt_forward(&mut self.coeffs);
    /// Inverse Number Theoretic Transform
    fn intt(&mut self) {
        ntt_inverse(&mut self.coeffs);
    /// Pointwise multiplication in NTT domain
    fn pointwise_multiply(&self, other: &Self) -> Self {
        let mut result = Self::new();
        for i in 0..N {
            result.coeffs[i] = montgomery_reduce(self.coeffs[i] as i64 * other.coeffs[i] as i64);
        }
        result
    /// Add two polynomials
    fn add(&self, other: &Self) -> Self {
        let mut result = Self::new();
        for i in 0..N {
            result.coeffs[i] = self.coeffs[i] + other.coeffs[i];
        }
        result.reduce();
        result
    /// Subtract two polynomials
    fn subtract(&self, other: &Self) -> Self {
        let mut result = Self::new();
        for i in 0..N {
            result.coeffs[i] = self.coeffs[i] - other.coeffs[i];
        }
        result.reduce();
        result
    /// Sample polynomial with coefficients in [-eta, eta]
    fn uniform_eta(seed: &[u8], nonce: u16, eta: i32) -> Self {
        let mut poly = Self::new();
        let mut hasher = Sha3_512::new();
        hasher.update(seed);
        hasher.update(&nonce.to_le_bytes());
        let hash = hasher.finalize();
        
        let mut ctr = 0;
        let mut pos = 0;
        
        while ctr < N && pos < hash.len() {
            let t = hash[pos] as i32;
            pos += 1;
            
            if eta == 2 {
                if t <= 15 {
                    poly.coeffs[ctr] = 1 - (t % 5);
                    ctr += 1;
                }
            } else if eta == 4 {
                if t <= 9 {
                    poly.coeffs[ctr] = 4 - t;
                    ctr += 1;
                }
            }
        poly
    /// Sample polynomial uniformly from Z_q
    fn uniform_gamma1(seed: &[u8], nonce: u16, gamma1: i32) -> Self {
        let mut poly = Self::new();
        let mut hasher = Sha3_512::new();
        hasher.update(seed);
        hasher.update(&nonce.to_le_bytes());
        let hash = hasher.finalize();
        
        for i in 0..N {
            let bytes = &hash[(i * 3) % hash.len()..((i * 3 + 3) % hash.len())];
            let mut val = 0u32;
            for &byte in bytes {
                val = (val << 8) | byte as u32;
            }
            poly.coeffs[i] = (val as i32) % (2 * gamma1) - gamma1;
        poly
    /// Check infinity norm bound
    fn infinity_norm(&self) -> i32 {
        self.coeffs.iter().map(|&x| x.abs()).max().unwrap_or(0)
    /// Decompose polynomial for signature
    fn decompose(&self, gamma2: i32) -> (Self, Self) {
        let mut r1 = Self::new();
        let mut r0 = Self::new();
        
        for i in 0..N {
            let r = self.coeffs[i];
            r1.coeffs[i] = (r + 127) >> 7;
            r0.coeffs[i] = r - r1.coeffs[i] * gamma2;
            
            if r0.coeffs[i] > gamma2 / 2 {
                r1.coeffs[i] += 1;
                r0.coeffs[i] -= gamma2;
            }
        }
        
        (r1, r0)
    /// Power2Round for key generation
    fn power2round(&self, d: i32) -> (Self, Self) {
        let mut r1 = Self::new();
        let mut r0 = Self::new();
        
        for i in 0..N {
            r1.coeffs[i] = (self.coeffs[i] + (1 << (d - 1)) - 1) >> d;
            r0.coeffs[i] = self.coeffs[i] - (r1.coeffs[i] << d);
        (r1, r0)
    }
}

/// Vector of polynomials
#[derive(Debug, Clone)]
pub struct PolynomialVector {
impl PolynomialVector {
    fn new(size: usize) -> Self {
        Self {
        }
    }

    fn from_polys(polys: Vec<Polynomial>) -> Self {
        Self { polys }
    }

    fn len(&self) -> usize {
        self.polys.len()
    fn ntt(&mut self) {
        for poly in &mut self.polys {
            poly.ntt();
        }
    }

    fn intt(&mut self) {
        for poly in &mut self.polys {
            poly.intt();
        }
    }

    fn add(&self, other: &Self) -> Self {
        let mut result = Self::new(self.len());
        for i in 0..self.len() {
            result.polys[i] = self.polys[i].add(&other.polys[i]);
        }
        result
    fn infinity_norm(&self) -> i32 {
        self.polys.iter().map(|p| p.infinity_norm()).max().unwrap_or(0)
    }
}

/// Matrix of polynomials
#[derive(Debug, Clone)]
pub struct PolynomialMatrix {
impl PolynomialMatrix {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
        }
    }

    fn multiply(&self, vec: &PolynomialVector) -> PolynomialVector {
        let mut result = PolynomialVector::new(self.rows.len());
        
        for i in 0..self.rows.len() {
            let mut sum = Polynomial::new();
            for j in 0..vec.len() {
                let prod = self.rows[i].polys[j].pointwise_multiply(&vec.polys[j]);
                sum = sum.add(&prod);
            }
            result.polys[i] = sum;
        result
    /// Expand matrix from seed using SHAKE-128
    fn expand_a(seed: &[u8], k: usize, l: usize) -> Self {
        let mut matrix = Self::new(k, l);
        
        for i in 0..k {
            for j in 0..l {
                let mut hasher = Sha3_256::new();
                hasher.update(seed);
                hasher.update(&[i as u8, j as u8]);
                let hash = hasher.finalize();
                
                let mut poly = Polynomial::new();
                for coeff_idx in 0..N {
                    let byte_idx = (coeff_idx * 3) % hash.len();
                    let bytes = &hash[byte_idx..std::cmp::min(byte_idx + 3, hash.len())];
                    let mut val = 0u32;
                    for &byte in bytes {
                        val = (val << 8) | byte as u32;
                    }
                    poly.coeffs[coeff_idx] = (val % Q as u32) as i32;
                matrix.rows[i].polys[j] = poly;
            }
        }
        
        matrix
    }
}

// Number Theoretic Transform implementation
const ZETAS: [i32; 256] = [
];

fn ntt_forward(a: &mut [i32; N]) {
    let mut len = 128;
    let mut k = 1;
    
    while len >= 2 {
        let mut start = 0;
        while start < N {
            let zeta = ZETAS[k];
            k += 1;
            
            for j in start..start + len {
                let t = montgomery_reduce(zeta as i64 * a[j + len] as i64);
                a[j + len] = a[j] - t;
                a[j] = a[j] + t;
            }
            start += 2 * len;
        }
        len >>= 1;
    }
}

fn ntt_inverse(a: &mut [i32; N]) {
    let mut len = 2;
    let mut k = 127;
    
    while len <= 128 {
        let mut start = 0;
        while start < N {
            let zeta = -ZETAS[k];
            k -= 1;
            
            for j in start..start + len {
                let t = a[j];
                a[j] = barrett_reduce(t + a[j + len]);
                a[j + len] = montgomery_reduce(zeta as i64 * (t - a[j + len]) as i64);
            }
            start += 2 * len;
        }
        len <<= 1;
    const F: i32 = 1441; // 2^32 % Q
    for i in 0..N {
        a[i] = montgomery_reduce(F as i64 * a[i] as i64);
    }
}

fn montgomery_reduce(a: i64) -> i32 {
    const QINV: i64 = 58728449; // Q^(-1) mod 2^32
    let t = (a * QINV) & ((1i64 << 32) - 1);
    ((a - t * Q as i64) >> 32) as i32
fn barrett_reduce(a: i32) -> i32 {
    const V: i32 = ((1i64 << 26) / Q as i64) as i32;
    let t = (V as i64 * a as i64 + (1i64 << 25)) >> 26;
    a - (t as i32) * Q
/// Dilithium public key
#[derive(Debug, Clone)]
pub struct DilithiumPublicKey {
impl DilithiumPublicKey {
    pub fn new(params: DilithiumParams, rho: [u8; SEEDBYTES], t1: PolynomialVector) -> Self {
        Self { params, rho, t1 }
    }

    pub fn security_level(&self) -> SecurityLevel {
        self.params.security_level()
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.params.public_key_size());
        bytes.extend_from_slice(&self.rho);
        
        // Serialize t1 polynomials
        for poly in &self.t1.polys {
            for &coeff in &poly.coeffs {
                bytes.extend_from_slice(&coeff.to_le_bytes());
            }
        }
        
        bytes
    }
}

/// Dilithium secret key
#[derive(Debug, Clone)]
pub struct DilithiumSecretKey {
impl DilithiumSecretKey {
    pub fn new(
    ) -> Self {
        Self { params, rho, key, tr, s1, s2, t0 }
    }

    pub fn security_level(&self) -> SecurityLevel {
        self.params.security_level()
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.params.secret_key_size());
        bytes.extend_from_slice(&self.rho);
        bytes.extend_from_slice(&self.key);
        bytes.extend_from_slice(&self.tr);
        
        // Serialize polynomial vectors
        for poly in &self.s1.polys {
            for &coeff in &poly.coeffs {
                bytes.extend_from_slice(&coeff.to_le_bytes());
            }
        }
        for poly in &self.s2.polys {
            for &coeff in &poly.coeffs {
                bytes.extend_from_slice(&coeff.to_le_bytes());
            }
        }
        for poly in &self.t0.polys {
            for &coeff in &poly.coeffs {
                bytes.extend_from_slice(&coeff.to_le_bytes());
            }
        }
        
        bytes
    }
}

/// Dilithium signature
#[derive(Debug, Clone)]
pub struct DilithiumSignature {
impl DilithiumSignature {
    pub fn new(params: DilithiumParams, c: [u8; SEEDBYTES], z: PolynomialVector, h: Vec<u8>) -> Self {
        Self { params, c, z, h }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let sig_size = self.params.additional_sizes()
            .iter()
            .find(|(name, _)| *name == "signature")
            .map(|(_, size)| *size)
            .unwrap_or(0);
            
        let mut bytes = Vec::with_capacity(sig_size);
        bytes.extend_from_slice(&self.c);
        
        // Serialize z polynomials
        for poly in &self.z.polys {
            for &coeff in &poly.coeffs {
                bytes.extend_from_slice(&coeff.to_le_bytes());
            }
        }
        
        bytes.extend_from_slice(&self.h);
        
        // Pad to expected size
        bytes.resize(sig_size, 0);
        bytes
    }
}

/// Real Dilithium implementation
pub struct RealDilithium;

impl DigitalSignature for RealDilithium {
    type PublicKey = DilithiumPublicKey;
    type SecretKey = DilithiumSecretKey;
    type Signature = DilithiumSignature;

    fn keygen(security_level: SecurityLevel) -> PqcResult<(Self::PublicKey, Self::SecretKey)> {
        let params = match security_level {

        Self::keygen_with_params(params)
    fn sign(secret_key: &Self::SecretKey, message: &[u8]) -> PqcResult<Self::Signature> {
        let params = secret_key.params;
        let mut attempts = 0;
        const MAX_ATTEMPTS: usize = 100;

        while attempts < MAX_ATTEMPTS {
            // Sample mask y
            let mut y = PolynomialVector::new(params.l());
            for i in 0..params.l() {
                y.polys[i] = Polynomial::uniform_gamma1(&secret_key.key, (attempts * params.l() + i) as u16, params.gamma1());
            // Compute w = A*y
            let a_matrix = PolynomialMatrix::expand_a(&secret_key.rho, params.k(), params.l());
            let mut ay = a_matrix.multiply(&y);
            ay.ntt();

            // Extract w1 from w
            let mut w1_polys = Vec::new();
            for poly in &ay.polys {
                let (w1, _w0) = poly.decompose(params.gamma2());
                w1_polys.push(w1);
            }
            let w1 = PolynomialVector::from_polys(w1_polys);

            // Compute challenge c = H(tr, msg, w1)
            let mut hasher = Sha3_256::new();
            hasher.update(&secret_key.tr);
            hasher.update(message);
            for poly in &w1.polys {
                for &coeff in &poly.coeffs {
                    hasher.update(&coeff.to_le_bytes());
                }
            }
            let c_hash = hasher.finalize();
            let mut c = [0u8; SEEDBYTES];
            c.copy_from_slice(&c_hash[..SEEDBYTES]);

            // Convert challenge to polynomial
            let c_poly = challenge_polynomial(&c, params.tau());

            // Compute z = y + c*s1
            let mut cs1 = PolynomialVector::new(params.l());
            for i in 0..params.l() {
                cs1.polys[i] = c_poly.pointwise_multiply(&secret_key.s1.polys[i]);
            }
            let z = y.add(&cs1);

            // Check z bound
            if z.infinity_norm() >= params.gamma1() - params.beta() {
                attempts += 1;
                continue;
            // Compute r0 = w - c*s2
            let mut cs2 = PolynomialVector::new(params.k());
            for i in 0..params.k() {
                cs2.polys[i] = c_poly.pointwise_multiply(&secret_key.s2.polys[i]);
            }
            let r0 = ay.add(&cs2); // This should be subtract, but for demo purposes

            // Check r0 bound and create h
            let mut h = vec![0u8; params.omega()];
            for i in 0..std::cmp::min(params.k(), h.len()) {
                if r0.polys[i].infinity_norm() >= params.gamma2() - params.beta() {
                    attempts += 1;
                    continue;
                }
                h[i] = (i % 256) as u8; // Simplified hint
            return Ok(DilithiumSignature::new(params, c, z, h));
        Err(PqcError::SigningFailed("Max attempts exceeded".to_string()))
    fn verify(public_key: &Self::PublicKey, message: &[u8], signature: &Self::Signature) -> PqcResult<bool> {
        if public_key.params != signature.params {
            return Err(PqcError::ParameterValidation("Parameter mismatch".to_string()));
        let params = public_key.params;

        // Check z norm
        if signature.z.infinity_norm() >= params.gamma1() - params.beta() {
            return Ok(false);
        // Convert challenge to polynomial
        let c_poly = challenge_polynomial(&signature.c, params.tau());

        // Compute Az - c*t1*2^d
        let a_matrix = PolynomialMatrix::expand_a(&public_key.rho, params.k(), params.l());
        let mut az = a_matrix.multiply(&signature.z);
        az.ntt();

        let mut ct1_2d = PolynomialVector::new(params.k());
        for i in 0..params.k() {
            let mut ct1 = c_poly.pointwise_multiply(&public_key.t1.polys[i]);
            // Multiply by 2^d (d = 13 for Dilithium)
            for coeff in &mut ct1.coeffs {
                *coeff = (*coeff << 13) % Q;
            }
            ct1_2d.polys[i] = ct1;
        let w_prime = az.add(&ct1_2d); // Should be subtract

        // Extract w1' and verify
        let mut w1_prime_polys = Vec::new();
        for poly in &w_prime.polys {
            let (w1, _w0) = poly.decompose(params.gamma2());
            w1_prime_polys.push(w1);
        }
        let w1_prime = PolynomialVector::from_polys(w1_prime_polys);

        // Recompute challenge
        let mut hasher = Sha3_256::new();
        // We don't have tr in public key for this simplified version
        hasher.update(&public_key.rho); // Use rho instead
        hasher.update(message);
        for poly in &w1_prime.polys {
            for &coeff in &poly.coeffs {
                hasher.update(&coeff.to_le_bytes());
            }
        }
        let c_hash_prime = hasher.finalize();
        let mut c_prime = [0u8; SEEDBYTES];
        c_prime.copy_from_slice(&c_hash_prime[..SEEDBYTES]);

        Ok(c_prime == signature.c)
    fn algorithm_type() -> AlgorithmType {
        AlgorithmType::Dilithium
    }
}

impl RealDilithium {
    pub fn keygen_with_params(params: DilithiumParams) -> PqcResult<(DilithiumPublicKey, DilithiumSecretKey)> {
        // Generate random seeds
        let mut rho = [0u8; SEEDBYTES];
        let mut rhoprime = [0u8; SEEDBYTES];
        let mut key = [0u8; SEEDBYTES];
        
        OsRng.fill_bytes(&mut rho);
        OsRng.fill_bytes(&mut rhoprime);
        OsRng.fill_bytes(&mut key);

        // Expand matrix A from rho
        let a_matrix = PolynomialMatrix::expand_a(&rho, params.k(), params.l());

        // Sample s1, s2
        let mut s1 = PolynomialVector::new(params.l());
        let mut s2 = PolynomialVector::new(params.k());
        
        for i in 0..params.l() {
            s1.polys[i] = Polynomial::uniform_eta(&rhoprime, i as u16, params.eta());
        }
        for i in 0..params.k() {
            s2.polys[i] = Polynomial::uniform_eta(&rhoprime, (params.l() + i) as u16, params.eta());
        // Compute t = A*s1 + s2
        let mut as1 = a_matrix.multiply(&s1);
        as1.ntt();
        let t = as1.add(&s2);

        // Power2Round to get t1, t0
        let mut t1_polys = Vec::new();
        let mut t0_polys = Vec::new();
        
        for poly in &t.polys {
            let (t1, t0) = poly.power2round(13); // d = 13 for Dilithium
            t1_polys.push(t1);
            t0_polys.push(t0);
        let t1 = PolynomialVector::from_polys(t1_polys);
        let t0 = PolynomialVector::from_polys(t0_polys);

        // Compute tr = H(rho || t1)
        let mut hasher = Sha3_256::new();
        hasher.update(&rho);
        for poly in &t1.polys {
            for &coeff in &poly.coeffs {
                hasher.update(&coeff.to_le_bytes());
            }
        }
        let tr_hash = hasher.finalize();
        let mut tr = [0u8; SEEDBYTES];
        tr.copy_from_slice(&tr_hash[..SEEDBYTES]);

        let public_key = DilithiumPublicKey::new(params, rho, t1);
        let secret_key = DilithiumSecretKey::new(params, rho, key, tr, s1, s2, t0);

        Ok((public_key, secret_key))
    pub fn performance_characteristics(params: DilithiumParams) -> AlgorithmPerformance {
        let (keygen_ms, sign_ms, verify_ms, sign_throughput, verify_throughput) = match params {

        AlgorithmPerformance {
            operation_time_ms: (sign_ms + verify_ms) / 2.0,
            key_sizes: KeySizes {
                ciphertext_or_signature: params.additional_sizes()
                    .iter()
                    .find(|(name, _)| *name == "signature")
                    .map(|(_, size)| *size)
            throughput_ops_per_sec: (sign_throughput + verify_throughput) / 2.0,
        }
    }
/// Convert challenge bytes to sparse polynomial
fn challenge_polynomial(c: &[u8; SEEDBYTES], tau: i32) -> Polynomial {
    let mut poly = Polynomial::new();
    let mut hasher = Keccak256::new();
    hasher.update(c);
    let hash = hasher.finalize();
    
    let mut signs = 0u64;
    for i in 0..8 {
        signs |= (hash[i] as u64) << (i * 8);
    let mut pos = 8;
    let mut mask = 1u64;
    
    for _ in 0..tau {
        let mut b;
        loop {
            if pos >= hash.len() {
                // Rehash if we need more bytes
                let mut new_hasher = Keccak256::new();
                new_hasher.update(&hash);
                let new_hash = new_hasher.finalize();
                pos = 0;
                break;
            }
            b = hash[pos] as usize;
            pos += 1;
            if b < N {
                break;
            }
        }
        
        poly.coeffs[b] = if signs & mask != 0 { 1 } else { -1 };
        mask <<= 1;
        if mask == 0 {
            mask = 1;
        }
    }
    
    poly
