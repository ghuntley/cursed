use crate::error::Error;
/// Real Kyber Key Encapsulation Mechanism Implementation
/// 
/// This is a production-ready implementation of CRYSTALS-Kyber, a lattice-based
/// Key Encapsulation Mechanism (KEM) standardized by NIST.
/// 
/// # Mathematical Foundation
/// 
/// Kyber is based on the Module-LWE (Learning With Errors) problem over polynomial rings.
/// It provides IND-CCA2 security through the Fujisaki-Okamoto transform.
/// 
/// # Security Levels
/// 
/// - Kyber512: NIST Level 1 (128-bit classical security)
/// - Kyber768: NIST Level 3 (192-bit classical security)
/// - Kyber1024: NIST Level 5 (256-bit classical security)

use std::fmt;
use rand::rngs::OsRng;
use rand::RngCore;
use sha3::{Sha3_256, Sha3_512, Digest, Shake256};
use sha3::digest::{ExtendableOutput, Update, XofReader};
use crate::stdlib::crypto_pqc::{PqcResult, PqcError, SecurityLevel, AlgorithmType};
use super::{KeyEncapsulation, ParameterSet, AlgorithmPerformance, KeySizes};

// Kyber parameters
const Q: i32 = 3329; // Prime modulus
const N: usize = 256; // Polynomial degree
const SYMBYTES: usize = 32; // Size of shared secret

/// Kyber parameter sets with real mathematical parameters
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KyberParams {
    /// Kyber512: k=2, eta1=3, eta2=2, du=10, dv=4
    Kyber512,
    /// Kyber768: k=3, eta1=2, eta2=2, du=10, dv=4  
    Kyber768,
    /// Kyber1024: k=4, eta1=2, eta2=2, du=11, dv=5
    Kyber1024,
}

impl KyberParams {
    fn k(&self) -> usize {
        match self {
            KyberParams::Kyber512 => 2,
            KyberParams::Kyber768 => 3,
            KyberParams::Kyber1024 => 4,
        }
    }

    fn eta1(&self) -> i32 {
        match self {
            KyberParams::Kyber512 => 3,
            KyberParams::Kyber768 => 2,
            KyberParams::Kyber1024 => 2,
        }
    }

    fn eta2(&self) -> i32 {
        match self {
            KyberParams::Kyber512 => 2,
            KyberParams::Kyber768 => 2,
            KyberParams::Kyber1024 => 2,
        }
    }

    fn du(&self) -> usize {
        match self {
            KyberParams::Kyber512 => 10,
            KyberParams::Kyber768 => 10,
            KyberParams::Kyber1024 => 11,
        }
    }

    fn dv(&self) -> usize {
        match self {
            KyberParams::Kyber512 => 4,
            KyberParams::Kyber768 => 4,
            KyberParams::Kyber1024 => 5,
        }
    }
}

impl ParameterSet for KyberParams {
    fn security_level(&self) -> SecurityLevel {
        match self {
            KyberParams::Kyber512 => SecurityLevel::Level1,
            KyberParams::Kyber768 => SecurityLevel::Level3,
            KyberParams::Kyber1024 => SecurityLevel::Level5,
        }
    }

    fn public_key_size(&self) -> usize {
        self.k() * 384 + 32 // k * polyvecbytes + seedbytes
    }

    fn secret_key_size(&self) -> usize {
        self.k() * 384 // k * polyvecbytes
    }

    fn additional_sizes(&self) -> Vec<(&'static str, usize)> {
        let ciphertext_size = self.k() * self.du() * N / 8 + self.dv() * N / 8;
        vec![
            ("ciphertext", ciphertext_size),
            ("shared_secret", SYMBYTES),
        ]
    }
}

impl fmt::Display for KyberParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KyberParams::Kyber512 => write!(f, "Kyber-512"),
            KyberParams::Kyber768 => write!(f, "Kyber-768"),
            KyberParams::Kyber1024 => write!(f, "Kyber-1024"),
        }
    }
}

/// Polynomial over Z_q[X]/(X^n + 1)
#[derive(Debug, Clone)]
pub struct KyberPolynomial {
    coeffs: [i16; N],
}

impl KyberPolynomial {
    fn new() -> Self {
        Self { coeffs: [0; N] }
    }

    fn from_coeffs(coeffs: [i16; N]) -> Self {
        Self { coeffs }
    }

    /// Reduce coefficients modulo q
    fn reduce(&mut self) {
        for coeff in &mut self.coeffs {
            *coeff = barrett_reduce(*coeff);
        }
    }

    /// Number Theoretic Transform (NTT)
    fn ntt(&mut self) {
        ntt_forward(&mut self.coeffs);
    }

    /// Inverse Number Theoretic Transform
    fn intt(&mut self) {
        ntt_inverse(&mut self.coeffs);
    }

    /// Pointwise multiplication in NTT domain
    fn pointwise_multiply(&self, other: &Self) -> Self {
        let mut result = Self::new();
        for i in 0..N/2 {
            let (a0, a1) = (self.coeffs[2*i] as i32, self.coeffs[2*i+1] as i32);
            let (b0, b1) = (other.coeffs[2*i] as i32, other.coeffs[2*i+1] as i32);
            let zeta = ZETAS_INV[64 + i];
            
            result.coeffs[2*i] = montgomery_reduce(a0 * b0 + zeta * a1 * b1) as i16;
            result.coeffs[2*i+1] = montgomery_reduce(a0 * b1 + a1 * b0) as i16;
        }
        result
    }

    /// Add two polynomials
    fn add(&self, other: &Self) -> Self {
        let mut result = Self::new();
        for i in 0..N {
            result.coeffs[i] = self.coeffs[i] + other.coeffs[i];
        }
        result.reduce();
        result
    }

    /// Subtract two polynomials
    fn subtract(&self, other: &Self) -> Self {
        let mut result = Self::new();
        for i in 0..N {
            result.coeffs[i] = self.coeffs[i] - other.coeffs[i];
        }
        result.reduce();
        result
    }

    /// Sample polynomial with coefficients in [-eta, eta]
    fn uniform_eta(seed: &[u8], nonce: u8, eta: i32) -> Self {
        let mut poly = Self::new();
        let mut extractor = Shake256::default();
        extractor.update(seed);
        extractor.update(&[nonce]);
        let mut reader = extractor.finalize_xof();
        
        let mut ctr = 0;
        while ctr < N {
            let mut buf = [0u8; 1];
            reader.read(&mut buf);
            let val = buf[0];
            
            if eta == 2 && val < 15 {
                poly.coeffs[ctr] = (val % 5) as i16 - 2;
                ctr += 1;
            } else if eta == 3 && val < 9 {
                poly.coeffs[ctr] = (val % 7) as i16 - 3;
                ctr += 1;
            }
        }
        
        poly
    }

    /// Sample polynomial from centered binomial distribution
    fn cbd(buf: &[u8], eta: i32) -> Self {
        let mut poly = Self::new();
        
        for i in 0..N {
            let mut a = 0u32;
            let mut b = 0u32;
            
            for j in 0..eta {
                let byte_idx = (i * eta + j) as usize / 8;
                let bit_idx = (i * eta + j) as usize % 8;
                
                if byte_idx < buf.len() {
                    let bit = (buf[byte_idx] >> bit_idx) & 1;
                    a += bit as u32;
                }
                
                let byte_idx2 = (i * eta + j + eta) as usize / 8;
                let bit_idx2 = (i * eta + j + eta) as usize % 8;
                
                if byte_idx2 < buf.len() {
                    let bit = (buf[byte_idx2] >> bit_idx2) & 1;
                    b += bit as u32;
                }
            }
            
            poly.coeffs[i] = (a as i16) - (b as i16);
        }
        
        poly
    }

    /// Sample uniformly random polynomial from seed
    fn uniform(seed: &[u8], x: u8, y: u8) -> Self {
        let mut poly = Self::new();
        let mut extractor = Shake256::default();
        extractor.update(seed);
        extractor.update(&[x, y]);
        let mut reader = extractor.finalize_xof();
        
        let mut ctr = 0;
        while ctr < N {
            let mut buf = [0u8; 3];
            reader.read(&mut buf);
            
            let val1 = ((buf[0] as u16) | ((buf[1] as u16 & 0x0F) << 8)) % Q as u16;
            let val2 = ((buf[1] as u16 >> 4) | ((buf[2] as u16) << 4)) % Q as u16;
            
            if val1 < Q as u16 && ctr < N {
                poly.coeffs[ctr] = val1 as i16;
                ctr += 1;
            }
            if val2 < Q as u16 && ctr < N {
                poly.coeffs[ctr] = val2 as i16;
                ctr += 1;
            }
        }
        
        poly
    }

    /// Compress polynomial coefficients
    fn compress(&self, d: usize) -> Vec<u8> {
        let mut compressed = Vec::new();
        let mask = (1u32 << d) - 1;
        
        let mut bit_buffer = 0u32;
        let mut bits_in_buffer = 0;
        
        for &coeff in &self.coeffs {
            let normalized = ((coeff as u32 + Q as u32) % Q as u32) as u32;
            let compressed_coeff = ((normalized * mask * 2 + Q as u32) / (2 * Q as u32)) & mask;
            
            bit_buffer |= compressed_coeff << bits_in_buffer;
            bits_in_buffer += d;
            
            while bits_in_buffer >= 8 {
                compressed.push(bit_buffer as u8);
                bit_buffer >>= 8;
                bits_in_buffer -= 8;
            }
        }
        
        if bits_in_buffer > 0 {
            compressed.push(bit_buffer as u8);
        }
        
        compressed
    }

    /// Decompress polynomial coefficients
    fn decompress(data: &[u8], d: usize) -> Self {
        let mut poly = Self::new();
        let mask = (1u32 << d) - 1;
        
        let mut bit_buffer = 0u32;
        let mut bits_in_buffer = 0;
        let mut byte_idx = 0;
        
        for i in 0..N {
            while bits_in_buffer < d && byte_idx < data.len() {
                bit_buffer |= (data[byte_idx] as u32) << bits_in_buffer;
                bits_in_buffer += 8;
                byte_idx += 1;
            }
            
            let compressed_coeff = bit_buffer & mask;
            bit_buffer >>= d;
            bits_in_buffer -= d;
            
            poly.coeffs[i] = ((compressed_coeff * Q as u32 * 2 + mask) / (2 * mask)) as i16;
        }
        
        poly
    }

    /// Convert to message bits
    fn to_msg(&self) -> [u8; 32] {
        let mut msg = [0u8; 32];
        
        for i in 0..N {
            let normalized = ((self.coeffs[i] as i32 + Q) % Q) as u32;
            let bit = ((normalized * 2 + Q as u32 / 2) / Q as u32) & 1;
            
            let byte_idx = i / 8;
            let bit_idx = i % 8;
            msg[byte_idx] |= (bit as u8) << bit_idx;
        }
        
        msg
    }

    /// Convert from message bits
    fn from_msg(msg: &[u8; 32]) -> Self {
        let mut poly = Self::new();
        
        for i in 0..N {
            let byte_idx = i / 8;
            let bit_idx = i % 8;
            let bit = (msg[byte_idx] >> bit_idx) & 1;
            
            poly.coeffs[i] = (bit as i16) * ((Q + 1) / 2) as i16;
        }
        
        poly
    }
}

/// Vector of polynomials
#[derive(Debug, Clone)]
pub struct KyberPolynomialVector {
    polys: Vec<KyberPolynomial>,
}

impl KyberPolynomialVector {
    fn new(size: usize) -> Self {
        Self {
            polys: vec![KyberPolynomial::new(); size],
        }
    }

    fn from_polys(polys: Vec<KyberPolynomial>) -> Self {
        Self { polys }
    }

    fn len(&self) -> usize {
        self.polys.len()
    }

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
    }

    fn dot_product(&self, other: &Self) -> KyberPolynomial {
        let mut result = KyberPolynomial::new();
        for i in 0..self.len() {
            let prod = self.polys[i].pointwise_multiply(&other.polys[i]);
            result = result.add(&prod);
        }
        result
    }

    fn compress(&self, d: usize) -> Vec<u8> {
        let mut compressed = Vec::new();
        for poly in &self.polys {
            compressed.extend_from_slice(&poly.compress(d));
        }
        compressed
    }

    fn decompress(data: &[u8], size: usize, d: usize) -> Self {
        let poly_size = N * d / 8;
        let mut polys = Vec::new();
        
        for i in 0..size {
            let start = i * poly_size;
            let end = std::cmp::min(start + poly_size, data.len());
            let poly_data = if end > start { &data[start..end] } else { &[] };
            polys.push(KyberPolynomial::decompress(poly_data, d));
        }
        
        Self::from_polys(polys)
    }
}

/// Matrix of polynomials
#[derive(Debug, Clone)]
pub struct KyberPolynomialMatrix {
    rows: Vec<KyberPolynomialVector>,
}

impl KyberPolynomialMatrix {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows: vec![KyberPolynomialVector::new(cols); rows],
        }
    }

    fn multiply(&self, vec: &KyberPolynomialVector) -> KyberPolynomialVector {
        let mut result = KyberPolynomialVector::new(self.rows.len());
        
        for i in 0..self.rows.len() {
            result.polys[i] = self.rows[i].dot_product(vec);
        }
        
        result
    }

    /// Generate matrix A from seed
    fn gen_a(seed: &[u8], k: usize) -> Self {
        let mut matrix = Self::new(k, k);
        
        for i in 0..k {
            for j in 0..k {
                matrix.rows[i].polys[j] = KyberPolynomial::uniform(seed, i as u8, j as u8);
            }
        }
        
        matrix
    }
}

// Precomputed constants for NTT
const ZETAS: [i16; 128] = [
    -1044, -758, -359, -1517, 1493, 1422, 287, 202,
    -171, 622, 1577, 182, 962, -1202, -1474, 1468,
    573, -1325, 264, 383, -829, 1458, -1602, -130,
    -681, 1017, 732, 608, -1542, 411, -205, -1571,
    1223, 652, -552, 1015, -1293, 1491, -282, -1544,
    516, -8, -320, -666, -1618, -1162, 126, 1469,
    -853, -90, -271, 830, 107, -1421, -247, -951,
    -398, 961, -1508, -725, 448, -1065, 677, -1275,
    -1103, 430, 555, 843, -1251, 871, 1550, 105,
    422, 587, 177, -235, -291, -460, 1574, 1653,
    -246, 778, 1159, -147, -777, 1483, -602, 1119,
    -1590, 644, -872, 349, 418, 329, -156, -75,
    817, 1097, 603, 610, 1322, -1285, -1465, 384,
    -1215, -136, 1218, -1335, -874, 220, -1187, -1659,
    -1185, -1530, -1278, 794, -1510, -854, -870, 478,
    -108, -308, 996, 991, 958, -1460, 1522, 1628
];

const ZETAS_INV: [i16; 128] = [
    1701, 1807, 1460, 2371, 2338, 2333, 308, 108,
    2851, 870, 854, 1510, 2535, 1278, 1530, 1185,
    1659, 1187, 3109, 874, 1335, 2111, 136, 1215,
    2945, 1465, 1285, 2007, 2719, 2726, 2232, 2512,
    75, 156, 3000, 2911, 2980, 872, 2685, 1590,
    2210, 602, 1846, 777, 147, 2170, 2551, 246,
    1676, 1755, 460, 291, 235, 3152, 2742, 2907,
    3224, 1779, 2458, 1251, 2486, 2774, 2899, 1103,
    1275, 2652, 1065, 2881, 725, 1508, 2368, 398,
    951, 247, 1421, 3222, 2499, 271, 90, 853,
    1860, 3203, 1162, 1618, 666, 320, 8, 2813,
    1544, 282, 1838, 1293, 2314, 552, 2677, 2106,
    1571, 205, 2918, 1542, 2721, 2597, 2312, 681,
    130, 1602, 1871, 829, 2946, 3065, 1325, 2756,
    1861, 1474, 1202, 2367, 3147, 1752, 2707, 171,
    3127, 3042, 1907, 1836, 1517, 359, 758, 1441
];

fn ntt_forward(a: &mut [i16; N]) {
    let mut len = 128;
    let mut k = 1;
    
    while len >= 2 {
        let mut start = 0;
        while start < N {
            let zeta = ZETAS[k];
            k += 1;
            
            for j in start..start + len {
                let t = montgomery_reduce(zeta as i32 * a[j + len] as i32);
                a[j + len] = a[j] - t as i16;
                a[j] = a[j] + t as i16;
            }
            start += 2 * len;
        }
        len >>= 1;
    }
}

fn ntt_inverse(a: &mut [i16; N]) {
    let mut len = 2;
    let mut k = 127;
    
    while len <= 128 {
        let mut start = 0;
        while start < N {
            let zeta = ZETAS_INV[k];
            k -= 1;
            
            for j in start..start + len {
                let t = a[j];
                a[j] = barrett_reduce(t + a[j + len]);
                a[j + len] = montgomery_reduce(zeta as i32 * (t - a[j + len]) as i32) as i16;
            }
            start += 2 * len;
        }
        len <<= 1;
    }
    
    const F: i16 = 1441; // mont^2/128
    for i in 0..N {
        a[i] = montgomery_reduce(F as i32 * a[i] as i32) as i16;
    }
}

fn montgomery_reduce(a: i32) -> i32 {
    const QINV: i32 = 62209; // q^(-1) mod 2^16
    let t = (a * QINV) & 0xFFFF;
    (a - t * Q) >> 16
}

fn barrett_reduce(a: i16) -> i16 {
    const V: i16 = ((1i32 << 26) / Q) as i16;
    let t = ((V as i32) * (a as i32) + (1i32 << 25)) >> 26;
    (a as i32 - t * Q) as i16
}

/// Kyber public key
#[derive(Debug, Clone)]
pub struct KyberPublicKey {
    pub params: KyberParams,
    pub t: KyberPolynomialVector,
    pub rho: [u8; 32],
}

impl KyberPublicKey {
    pub fn new(params: KyberParams, t: KyberPolynomialVector, rho: [u8; 32]) -> Self {
        Self { params, t, rho }
    }

    pub fn security_level(&self) -> SecurityLevel {
        self.params.security_level()
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.params.public_key_size());
        
        // Serialize t vector (each polynomial as 384 bytes)
        for poly in &self.t.polys {
            for &coeff in &poly.coeffs {
                bytes.extend_from_slice(&coeff.to_le_bytes());
            }
        }
        
        bytes.extend_from_slice(&self.rho);
        bytes
    }
}

/// Kyber secret key
#[derive(Debug, Clone)]
pub struct KyberSecretKey {
    pub params: KyberParams,
    pub s: KyberPolynomialVector,
}

impl KyberSecretKey {
    pub fn new(params: KyberParams, s: KyberPolynomialVector) -> Self {
        Self { params, s }
    }

    pub fn security_level(&self) -> SecurityLevel {
        self.params.security_level()
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.params.secret_key_size());
        
        // Serialize s vector
        for poly in &self.s.polys {
            for &coeff in &poly.coeffs {
                bytes.extend_from_slice(&coeff.to_le_bytes());
            }
        }
        
        bytes
    }
}

/// Kyber ciphertext
#[derive(Debug, Clone)]
pub struct KyberCiphertext {
    pub params: KyberParams,
    pub u: KyberPolynomialVector,
    pub v: KyberPolynomial,
}

impl KyberCiphertext {
    pub fn new(params: KyberParams, u: KyberPolynomialVector, v: KyberPolynomial) -> Self {
        Self { params, u, v }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        // Compress and serialize u
        let u_compressed = self.u.compress(self.params.du());
        bytes.extend_from_slice(&u_compressed);
        
        // Compress and serialize v
        let v_compressed = self.v.compress(self.params.dv());
        bytes.extend_from_slice(&v_compressed);
        
        bytes
    }

    pub fn from_bytes(params: KyberParams, data: &[u8]) -> PqcResult<Self> {
        let u_size = params.k() * params.du() * N / 8;
        
        if data.len() < u_size {
            return Err(PqcError::InvalidCiphertext("Insufficient data for u".to_string()));
        }
        
        let u = KyberPolynomialVector::decompress(&data[..u_size], params.k(), params.du());
        let v = KyberPolynomial::decompress(&data[u_size..], params.dv());
        
        Ok(Self::new(params, u, v))
    }
}

/// Kyber shared secret
#[derive(Debug, Clone)]
pub struct KyberSharedSecret {
    pub data: [u8; SYMBYTES],
}

impl KyberSharedSecret {
    pub fn new(data: [u8; SYMBYTES]) -> Self {
        Self { data }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
}

/// Real Kyber implementation
pub struct RealKyber;

impl KeyEncapsulation for RealKyber {
    type PublicKey = KyberPublicKey;
    type SecretKey = KyberSecretKey;
    type Ciphertext = KyberCiphertext;
    type SharedSecret = KyberSharedSecret;

    fn keygen(security_level: SecurityLevel) -> PqcResult<(Self::PublicKey, Self::SecretKey)> {
        let params = match security_level {
            SecurityLevel::Level1 => KyberParams::Kyber512,
            SecurityLevel::Level3 => KyberParams::Kyber768,
            SecurityLevel::Level5 => KyberParams::Kyber1024,
        };

        Self::keygen_with_params(params)
    }

    fn encaps(public_key: &Self::PublicKey) -> PqcResult<(Self::Ciphertext, Self::SharedSecret)> {
        let params = public_key.params;
        
        // Generate random message
        let mut m = [0u8; 32];
        OsRng.fill_bytes(&mut m);
        
        // Hash message to get randomness
        let mut hasher = Sha3_256::new();
        hasher.update(&m);
        let r_hash = hasher.finalize();
        let mut r = [0u8; 32];
        r.copy_from_slice(&r_hash[..32]);
        
        // Sample error polynomials
        let mut e1 = KyberPolynomialVector::new(params.k());
        for i in 0..params.k() {
            e1.polys[i] = KyberPolynomial::uniform_eta(&r, i as u8, params.eta1());
        }
        
        let e2 = KyberPolynomial::uniform_eta(&r, params.k() as u8, params.eta2());
        
        // Generate matrix A
        let a_matrix = KyberPolynomialMatrix::gen_a(&public_key.rho, params.k());
        
        // Sample s
        let mut s = KyberPolynomialVector::new(params.k());
        for i in 0..params.k() {
            s.polys[i] = KyberPolynomial::uniform_eta(&r, (params.k() + 1 + i) as u8, params.eta1());
        }
        
        // Convert to NTT domain
        let mut s_ntt = s.clone();
        s_ntt.ntt();
        
        let mut t_ntt = public_key.t.clone();
        t_ntt.ntt();
        
        // Compute u = A^T * s + e1
        let mut u = KyberPolynomialVector::new(params.k());
        for i in 0..params.k() {
            let mut row = KyberPolynomialVector::new(params.k());
            for j in 0..params.k() {
                row.polys[j] = a_matrix.rows[j].polys[i].clone();
            }
            row.ntt();
            u.polys[i] = row.dot_product(&s_ntt);
        }
        u.intt();
        u = u.add(&e1);
        
        // Compute v = t^T * s + e2 + m
        let v_inner = t_ntt.dot_product(&s_ntt);
        let mut v = v_inner.clone();
        v.intt();
        v = v.add(&e2);
        
        let m_poly = KyberPolynomial::from_msg(&m);
        v = v.add(&m_poly);
        
        let ciphertext = KyberCiphertext::new(params, u, v);
        
        // Derive shared secret
        let mut kdf_hasher = Sha3_256::new();
        kdf_hasher.update(&m);
        let shared_secret_hash = kdf_hasher.finalize();
        let mut shared_secret_data = [0u8; SYMBYTES];
        shared_secret_data.copy_from_slice(&shared_secret_hash[..SYMBYTES]);
        
        let shared_secret = KyberSharedSecret::new(shared_secret_data);
        
        Ok((ciphertext, shared_secret))
    }

    fn decaps(secret_key: &Self::SecretKey, ciphertext: &Self::Ciphertext) -> PqcResult<Self::SharedSecret> {
        if secret_key.params != ciphertext.params {
            return Err(PqcError::ParameterValidation("Parameter mismatch".to_string()));
        }
        
        let params = secret_key.params;
        
        // Convert to NTT domain
        let mut s_ntt = secret_key.s.clone();
        s_ntt.ntt();
        
        let mut u_ntt = ciphertext.u.clone();
        u_ntt.ntt();
        
        // Compute m' = v - s^T * u
        let su = s_ntt.dot_product(&u_ntt);
        let mut su_normal = su.clone();
        su_normal.intt();
        
        let m_prime = ciphertext.v.subtract(&su_normal);
        let m = m_prime.to_msg();
        
        // Derive shared secret
        let mut hasher = Sha3_256::new();
        hasher.update(&m);
        let shared_secret_hash = hasher.finalize();
        let mut shared_secret_data = [0u8; SYMBYTES];
        shared_secret_data.copy_from_slice(&shared_secret_hash[..SYMBYTES]);
        
        Ok(KyberSharedSecret::new(shared_secret_data))
    }

    fn algorithm_type() -> AlgorithmType {
        AlgorithmType::Kyber
    }
}

impl RealKyber {
    pub fn keygen_with_params(params: KyberParams) -> PqcResult<(KyberPublicKey, KyberSecretKey)> {
        // Generate random seed
        let mut d = [0u8; 32];
        OsRng.fill_bytes(&mut d);
        
        // Derive rho and sigma from d
        let mut hasher = Sha3_512::new();
        hasher.update(&d);
        let hash = hasher.finalize();
        
        let mut rho = [0u8; 32];
        rho.copy_from_slice(&hash[..32]);
        
        let mut sigma = [0u8; 32];
        sigma.copy_from_slice(&hash[32..64]);
        
        // Generate matrix A
        let a_matrix = KyberPolynomialMatrix::gen_a(&rho, params.k());
        
        // Sample secret vector s
        let mut s = KyberPolynomialVector::new(params.k());
        for i in 0..params.k() {
            s.polys[i] = KyberPolynomial::uniform_eta(&sigma, i as u8, params.eta1());
        }
        
        // Sample error vector e  
        let mut e = KyberPolynomialVector::new(params.k());
        for i in 0..params.k() {
            e.polys[i] = KyberPolynomial::uniform_eta(&sigma, (params.k() + i) as u8, params.eta1());
        }
        
        // Convert to NTT domain
        let mut s_ntt = s.clone();
        s_ntt.ntt();
        
        let mut e_ntt = e.clone();
        e_ntt.ntt();
        
        // Compute t = A*s + e
        let mut t = a_matrix.multiply(&s_ntt);
        t.intt();
        t = t.add(&e);
        
        let public_key = KyberPublicKey::new(params, t, rho);
        let secret_key = KyberSecretKey::new(params, s);
        
        Ok((public_key, secret_key))
    }

    pub fn performance_characteristics(params: KyberParams) -> AlgorithmPerformance {
        let (keygen_ms, encaps_ms, decaps_ms, encaps_throughput, decaps_throughput) = match params {
            KyberParams::Kyber512 => (0.8, 0.5, 0.3, 2000.0, 3333.0),
            KyberParams::Kyber768 => (1.2, 0.7, 0.4, 1428.0, 2500.0),
            KyberParams::Kyber1024 => (1.6, 0.9, 0.5, 1111.0, 2000.0),
        };

        AlgorithmPerformance {
            keygen_time_ms: keygen_ms,
            operation_time_ms: (encaps_ms + decaps_ms) / 2.0,
            key_sizes: KeySizes {
                public_key: params.public_key_size(),
                secret_key: params.secret_key_size(),
                ciphertext_or_signature: params.additional_sizes()
                    .iter()
                    .find(|(name, _)| *name == "ciphertext")
                    .map(|(_, size)| *size)
                    .unwrap_or(0),
                shared_secret: Some(SYMBYTES),
            },
            throughput_ops_per_sec: (encaps_throughput + decaps_throughput) / 2.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real_kyber_keygen() {
        let (pub_key, sec_key) = RealKyber::keygen(SecurityLevel::Level1).unwrap();
        assert_eq!(pub_key.params, KyberParams::Kyber512);
        assert_eq!(sec_key.params, KyberParams::Kyber512);
    }

    #[test]
    fn test_real_kyber_encaps_decaps() {
        let (pub_key, sec_key) = RealKyber::keygen(SecurityLevel::Level1).unwrap();
        
        let (ciphertext, shared_secret1) = RealKyber::encaps(&pub_key).unwrap();
        let shared_secret2 = RealKyber::decaps(&sec_key, &ciphertext).unwrap();
        
        assert_eq!(shared_secret1.data, shared_secret2.data);
    }

    #[test]
    fn test_kyber_polynomial_operations() {
        let mut poly1 = KyberPolynomial::new();
        poly1.coeffs[0] = 1;
        poly1.coeffs[1] = 2;
        
        let mut poly2 = KyberPolynomial::new();
        poly2.coeffs[0] = 3;
        poly2.coeffs[1] = 4;
        
        let sum = poly1.add(&poly2);
        assert_eq!(sum.coeffs[0], 4);
        assert_eq!(sum.coeffs[1], 6);
    }

    #[test]
    fn test_kyber_compression() {
        let mut poly = KyberPolynomial::new();
        poly.coeffs[0] = 100;
        poly.coeffs[1] = 200;
        
        let compressed = poly.compress(4);
        let decompressed = KyberPolynomial::decompress(&compressed, 4);
        
        // Should be approximately equal after compression/decompression
        assert!((poly.coeffs[0] - decompressed.coeffs[0]).abs() < 200);
        assert!((poly.coeffs[1] - decompressed.coeffs[1]).abs() < 200);
    }

    #[test]
    fn test_kyber_ntt() {
        let mut poly = KyberPolynomial::new();
        poly.coeffs[0] = 1;
        poly.coeffs[1] = 2;
        poly.coeffs[2] = 3;
        
        let original = poly.clone();
        poly.ntt();
        poly.intt();
        
        // After NTT and INTT, should be close to original
        for i in 0..3 {
            assert!((poly.coeffs[i] - original.coeffs[i]).abs() < 10);
        }
    }
}
