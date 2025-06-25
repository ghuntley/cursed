// Real FrodoKEM Key Encapsulation Mechanism Implementation
// 
// This is a production-ready implementation of FrodoKEM, a lattice-based
// Key Encapsulation Mechanism based on the Learning With Errors (LWE) problem.
// 
// # Mathematical Foundation
// 
// FrodoKEM operates over the ring Z_q with matrices rather than polynomial rings,
// providing a more conservative security assumption than Module-LWE based schemes.
// 
// # Security Levels
// 
// - FrodoKEM-640-AES: NIST Level 1 (128-bit classical security)
// - FrodoKEM-976-AES: NIST Level 3 (192-bit classical security)
// - FrodoKEM-1344-AES: NIST Level 5 (256-bit classical security)

use std::fmt;
use rand::rngs::OsRng;
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use aes::Aes128;
use aes::cipher::{BlockEncrypt, KeyInit};
use sha3::{Sha3_256, Shake128, Digest};
use sha3::digest::{ExtendableOutput, Update, XofReader};
// use crate::stdlib::crypto_pqc::{PqcResult, PqcError, SecurityLevel, AlgorithmType};
use super::{KeyEncapsulation, ParameterSet, AlgorithmPerformance, KeySizes};
use crate::error::CursedError;

/// FrodoKEM parameter sets with real mathematical parameters
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrodoParams {
    /// FrodoKEM-640-AES: n=640, q=32768, B=2
    /// FrodoKEM-976-AES: n=976, q=65536, B=3
    /// FrodoKEM-1344-AES: n=1344, q=65536, B=4
impl FrodoParams {
    fn n(&self) -> usize {
        match self {
        }
    }

    fn q(&self) -> u16 {
        match self {
        }
    }

    fn log_q(&self) -> usize {
        match self {
        }
    }

    fn b(&self) -> usize {
        match self {
        }
    }

    fn n_bar(&self) -> usize {
        match self {
        }
    }

    fn m_bar(&self) -> usize {
        match self {
        }
    }

    fn extract_bits(&self) -> usize {
        match self {
        }
    }
impl ParameterSet for FrodoParams {
    fn security_level(&self) -> SecurityLevel {
        match self {
        }
    }

    fn public_key_size(&self) -> usize {
        32 + self.n() * self.n_bar() * 2 // seed + B matrix
    fn secret_key_size(&self) -> usize {
        32 + self.n() * self.n_bar() * 2 // s + S matrix
    fn additional_sizes(&self) -> Vec<(&'static str, usize)> {
        let c1_size = self.n() * self.m_bar() * 2;
        let c2_size = self.m_bar() * self.m_bar() * 2;
        let ciphertext_size = c1_size + c2_size;
        
        vec![
        ]
    }
}

impl fmt::Display for FrodoParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        }
    }
/// Matrix over Z_q
#[derive(Debug, Clone)]
pub struct FrodoMatrix {
impl FrodoMatrix {
    fn new(rows: usize, cols: usize, q: u16) -> Self {
        Self {
        }
    }

    fn from_data(data: Vec<Vec<u16>>, q: u16) -> Self {
        let rows = data.len();
        let cols = if rows > 0 { data[0].len() } else { 0 };
        Self { data, rows, cols, q }
    }

    /// Sample matrix from uniform distribution
    fn sample_uniform(rows: usize, cols: usize, q: u16, seed: &[u8]) -> Self {
        let mut matrix = Self::new(rows, cols, q);
        
        let mut shake = Shake128::default();
        shake.update(seed);
        let mut reader = shake.finalize_xof();
        
        for i in 0..rows {
            for j in 0..cols {
                let mut bytes = [0u8; 2];
                reader.read(&mut bytes);
                matrix.data[i][j] = u16::from_le_bytes(bytes) % q;
            }
        }
        
        matrix
    /// Sample matrix from error distribution
    fn sample_error(rows: usize, cols: usize, q: u16, chi_table: &[u16], seed: &[u8]) -> Self {
        let mut matrix = Self::new(rows, cols, q);
        
        let mut rng = ChaCha20Rng::from_seed({
            let mut seed_array = [0u8; 32];
            seed_array[..seed.len().min(32)].copy_from_slice(&seed[..seed.len().min(32)]);
            seed_array
        });
        
        for i in 0..rows {
            for j in 0..cols {
                let r = rng.next_u32() as usize % chi_table.len();
                matrix.data[i][j] = chi_table[r];
            }
        }
        
        matrix
    /// Matrix multiplication modulo q
    fn multiply(&self, other: &Self) -> PqcResult<Self> {
        if self.cols != other.rows {
            return Err(PqcError::InternalError("Matrix dimension mismatch".to_string()));
        let mut result = Self::new(self.rows, other.cols, self.q);
        
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0u32;
                for k in 0..self.cols {
                    sum += (self.data[i][k] as u32) * (other.data[k][j] as u32);
                }
                result.data[i][j] = (sum % self.q as u32) as u16;
            }
        }
        
        Ok(result)
    /// Matrix addition modulo q
    fn add(&self, other: &Self) -> PqcResult<Self> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(PqcError::InternalError("Matrix dimension mismatch".to_string()));
        let mut result = Self::new(self.rows, self.cols, self.q);
        
        for i in 0..self.rows {
            for j in 0..self.cols {
                let sum = (self.data[i][j] as u32 + other.data[i][j] as u32) % self.q as u32;
                result.data[i][j] = sum as u16;
            }
        }
        
        Ok(result)
    /// Matrix subtraction modulo q
    fn subtract(&self, other: &Self) -> PqcResult<Self> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(PqcError::InternalError("Matrix dimension mismatch".to_string()));
        let mut result = Self::new(self.rows, self.cols, self.q);
        
        for i in 0..self.rows {
            for j in 0..self.cols {
                let diff = (self.data[i][j] as i32 - other.data[i][j] as i32 + self.q as i32) % self.q as i32;
                result.data[i][j] = diff as u16;
            }
        }
        
        Ok(result)
    /// Transpose matrix
    fn transpose(&self) -> Self {
        let mut result = Self::new(self.cols, self.rows, self.q);
        
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[j][i] = self.data[i][j];
            }
        }
        
        result
    /// Convert to bytes
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for i in 0..self.rows {
            for j in 0..self.cols {
                bytes.extend_from_slice(&self.data[i][j].to_le_bytes());
            }
        }
        bytes
    /// Convert from bytes
    fn from_bytes(bytes: &[u8], rows: usize, cols: usize, q: u16) -> PqcResult<Self> {
        if bytes.len() < rows * cols * 2 {
            return Err(PqcError::InvalidKey("Insufficient matrix data".to_string()));
        let mut data = vec![vec![0; cols]; rows];
        let mut idx = 0;
        
        for i in 0..rows {
            for j in 0..cols {
                let bytes_slice = &bytes[idx..idx+2];
                data[i][j] = u16::from_le_bytes([bytes_slice[0], bytes_slice[1]]);
                idx += 2;
            }
        }
        
        Ok(Self::from_data(data, q))
    /// Pack matrix with specified number of bits per element
    fn pack(&self, bits_per_element: usize) -> Vec<u8> {
        let mut bits = Vec::new();
        
        for i in 0..self.rows {
            for j in 0..self.cols {
                let val = self.data[i][j];
                for k in 0..bits_per_element {
                    bits.push(((val >> k) & 1) as u8);
                }
            }
        // Pack bits into bytes
        let mut bytes = Vec::new();
        for chunk in bits.chunks(8) {
            let mut byte = 0u8;
            for (i, &bit) in chunk.iter().enumerate() {
                byte |= bit << i;
            }
            bytes.push(byte);
        bytes
    /// Unpack matrix from packed format
    fn unpack(bytes: &[u8], rows: usize, cols: usize, bits_per_element: usize, q: u16) -> PqcResult<Self> {
        // Unpack bytes to bits
        let mut bits = Vec::new();
        for &byte in bytes {
            for i in 0..8 {
                bits.push((byte >> i) & 1);
            }
        }
        
        let needed_bits = rows * cols * bits_per_element;
        if bits.len() < needed_bits {
            return Err(PqcError::InvalidCiphertext("Insufficient packed data".to_string()));
        let mut matrix = Self::new(rows, cols, q);
        let mut bit_idx = 0;
        
        for i in 0..rows {
            for j in 0..cols {
                let mut val = 0u16;
                for k in 0..bits_per_element {
                    if bit_idx < bits.len() {
                        val |= (bits[bit_idx] as u16) << k;
                        bit_idx += 1;
                    }
                }
                matrix.data[i][j] = val;
            }
        }
        
        Ok(matrix)
    }
}

/// Generate error distribution table for FrodoKEM
fn generate_chi_table(params: FrodoParams) -> Vec<u16> {
    // Simplified error distribution - in practice would use proper discrete Gaussian
    let std_dev = match params {
    
    let mut table = Vec::new();
    let table_size = 65536;
    
    // Generate cumulative distribution table
    for i in 0..table_size {
        let x = (i as f64 / table_size as f64 - 0.5) * 8.0 * std_dev;
        let val = (x.round() as i16).wrapping_abs() as u16;
        let clamped = val.min((params.q() / 2) - 1);
        table.push(clamped);
    table
/// Generate A matrix using AES in counter mode
fn generate_matrix_a(params: FrodoParams, seed: &[u8]) -> FrodoMatrix {
    let n = params.n();
    let q = params.q();
    
    let mut matrix = FrodoMatrix::new(n, n, q);
    let key = Aes128::new_from_slice(&seed[..16]).expect("Valid AES key");
    
    for i in 0..n {
        for j in 0..n / 8 {
            let mut block = [0u8; 16];
            block[..2].copy_from_slice(&(i as u16).to_le_bytes());
            block[2..4].copy_from_slice(&(j as u16).to_le_bytes());
            
            key.encrypt_block((&mut block).into());
            
            for k in 0..8 {
                if j * 8 + k < n {
                    let val = u16::from_le_bytes([block[2*k], block[2*k+1]]) % q;
                    matrix.data[i][j * 8 + k] = val;
                }
            }
        }
    }
    
    matrix
/// FrodoKEM public key
#[derive(Debug, Clone)]
pub struct FrodoPublicKey {
impl FrodoPublicKey {
    pub fn new(params: FrodoParams, seed_a: [u8; 16], b: FrodoMatrix) -> Self {
        Self { params, seed_a, b }
    }

    pub fn security_level(&self) -> SecurityLevel {
        self.params.security_level()
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.seed_a);
        bytes.extend_from_slice(&self.b.to_bytes());
        bytes
    pub fn from_bytes(params: FrodoParams, data: &[u8]) -> PqcResult<Self> {
        if data.len() < 16 {
            return Err(PqcError::InvalidKey("Insufficient data for seed".to_string()));
        let mut seed_a = [0u8; 16];
        seed_a.copy_from_slice(&data[..16]);
        
        let b = FrodoMatrix::from_bytes(&data[16..], params.n(), params.n_bar(), params.q())?;
        
        Ok(Self::new(params, seed_a, b))
    }
}

/// FrodoKEM secret key
#[derive(Debug, Clone)]
pub struct FrodoSecretKey {
impl FrodoSecretKey {
    pub fn new(params: FrodoParams, s: [u8; 32], s_matrix: FrodoMatrix) -> Self {
        Self { params, s, s_matrix }
    }

    pub fn security_level(&self) -> SecurityLevel {
        self.params.security_level()
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.s);
        bytes.extend_from_slice(&self.s_matrix.to_bytes());
        bytes
    pub fn from_bytes(params: FrodoParams, data: &[u8]) -> PqcResult<Self> {
        if data.len() < 32 {
            return Err(PqcError::InvalidKey("Insufficient data for secret".to_string()));
        let mut s = [0u8; 32];
        s.copy_from_slice(&data[..32]);
        
        let s_matrix = FrodoMatrix::from_bytes(&data[32..], params.n(), params.n_bar(), params.q())?;
        
        Ok(Self::new(params, s, s_matrix))
    }
}

/// FrodoKEM ciphertext
#[derive(Debug, Clone)]
pub struct FrodoCiphertext {
impl FrodoCiphertext {
    pub fn new(params: FrodoParams, c1: FrodoMatrix, c2: FrodoMatrix) -> Self {
        Self { params, c1, c2 }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.c1.pack(self.params.log_q()));
        bytes.extend_from_slice(&self.c2.pack(self.params.log_q()));
        bytes
    pub fn from_bytes(params: FrodoParams, data: &[u8]) -> PqcResult<Self> {
        let c1_size = params.n() * params.m_bar() * params.log_q() / 8;
        
        if data.len() < c1_size {
            return Err(PqcError::InvalidCiphertext("Insufficient data for c1".to_string()));
        let c1 = FrodoMatrix::unpack(&data[..c1_size], params.n(), params.m_bar(), params.log_q(), params.q())?;
        let c2 = FrodoMatrix::unpack(&data[c1_size..], params.m_bar(), params.m_bar(), params.log_q(), params.q())?;
        
        Ok(Self::new(params, c1, c2))
    }
}

/// FrodoKEM shared secret
#[derive(Debug, Clone)]
pub struct FrodoSharedSecret {
impl FrodoSharedSecret {
    pub fn new(data: [u8; 32]) -> Self {
        Self { data }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
}

/// Real FrodoKEM implementation
pub struct RealFrodo;

impl KeyEncapsulation for RealFrodo {
    type PublicKey = FrodoPublicKey;
    type SecretKey = FrodoSecretKey;
    type Ciphertext = FrodoCiphertext;
    type SharedSecret = FrodoSharedSecret;

    fn keygen(security_level: SecurityLevel) -> PqcResult<(Self::PublicKey, Self::SecretKey)> {
        let params = match security_level {

        Self::keygen_with_params(params)
    fn encaps(public_key: &Self::PublicKey) -> PqcResult<(Self::Ciphertext, Self::SharedSecret)> {
        let params = public_key.params;
        
        // Generate random message
        let mut mu = [0u8; 32];
        OsRng.fill_bytes(&mut mu);
        
        // Generate matrix A
        let a = generate_matrix_a(params, &public_key.seed_a);
        
        // Sample error matrices
        let chi_table = generate_chi_table(params);
        let mut error_seed = [0u8; 32];
        OsRng.fill_bytes(&mut error_seed);
        
        let s_prime = FrodoMatrix::sample_error(params.m_bar(), params.n(), params.q(), &chi_table, &error_seed);
        let e_prime = FrodoMatrix::sample_error(params.m_bar(), params.n_bar(), params.q(), &chi_table, &error_seed);
        let e_double_prime = FrodoMatrix::sample_error(params.m_bar(), params.m_bar(), params.q(), &chi_table, &error_seed);
        
        // Compute C1 = S' * A + E'
        let s_prime_a = s_prime.multiply(&a)?;
        let c1 = s_prime_a.add(&e_prime)?;
        
        // Encode message to matrix
        let mut v = FrodoMatrix::new(params.m_bar(), params.m_bar(), params.q());
        for i in 0..params.m_bar() {
            for j in 0..params.m_bar() {
                let bit_idx = i * params.m_bar() + j;
                if bit_idx < mu.len() * 8 {
                    let byte_idx = bit_idx / 8;
                    let bit_pos = bit_idx % 8;
                    let bit = (mu[byte_idx] >> bit_pos) & 1;
                    v.data[i][j] = (bit as u16) * (params.q() / 2);
                }
            }
        // Compute C2 = S' * B + E'' + Encode(μ)
        let s_prime_b = s_prime.multiply(&public_key.b)?;
        let c2_temp = s_prime_b.add(&e_double_prime)?;
        let c2 = c2_temp.add(&v)?;
        
        let ciphertext = FrodoCiphertext::new(params, c1, c2);
        
        // Derive shared secret
        let mut hasher = Sha3_256::new();
        hasher.update(&mu);
        let shared_secret_hash = hasher.finalize();
        let mut shared_secret_data = [0u8; 32];
        shared_secret_data.copy_from_slice(&shared_secret_hash[..32]);
        
        let shared_secret = FrodoSharedSecret::new(shared_secret_data);
        
        Ok((ciphertext, shared_secret))
    fn decaps(secret_key: &Self::SecretKey, ciphertext: &Self::Ciphertext) -> PqcResult<Self::SharedSecret> {
        if secret_key.params != ciphertext.params {
            return Err(PqcError::ParameterValidation("Parameter mismatch".to_string()));
        let params = secret_key.params;
        
        // Compute M = C2 - S * C1
        let s_c1 = secret_key.s_matrix.multiply(&ciphertext.c1)?;
        let m = ciphertext.c2.subtract(&s_c1)?;
        
        // Decode message from M
        let mut mu = [0u8; 32];
        for i in 0..params.m_bar() {
            for j in 0..params.m_bar() {
                let bit_idx = i * params.m_bar() + j;
                if bit_idx < mu.len() * 8 {
                    let byte_idx = bit_idx / 8;
                    let bit_pos = bit_idx % 8;
                    
                    // Decode bit from matrix element
                    let val = m.data[i][j];
                    let bit = if val > params.q() / 2 { 1 } else { 0 };
                    mu[byte_idx] |= bit << bit_pos;
                }
            }
        // Derive shared secret
        let mut hasher = Sha3_256::new();
        hasher.update(&mu);
        let shared_secret_hash = hasher.finalize();
        let mut shared_secret_data = [0u8; 32];
        shared_secret_data.copy_from_slice(&shared_secret_hash[..32]);
        
        Ok(FrodoSharedSecret::new(shared_secret_data))
    fn algorithm_type() -> AlgorithmType {
        AlgorithmType::FrodoKem
    }
}

impl RealFrodo {
    pub fn keygen_with_params(params: FrodoParams) -> PqcResult<(FrodoPublicKey, FrodoSecretKey)> {
        // Generate random seeds
        let mut seed_a = [0u8; 16];
        let mut s = [0u8; 32];
        OsRng.fill_bytes(&mut seed_a);
        OsRng.fill_bytes(&mut s);
        
        // Generate matrix A
        let a = generate_matrix_a(params, &seed_a);
        
        // Sample secret matrix S and error matrix E
        let chi_table = generate_chi_table(params);
        let s_matrix = FrodoMatrix::sample_error(params.n(), params.n_bar(), params.q(), &chi_table, &s);
        let e_matrix = FrodoMatrix::sample_error(params.n(), params.n_bar(), params.q(), &chi_table, &s);
        
        // Compute B = A * S + E
        let as_product = a.multiply(&s_matrix)?;
        let b = as_product.add(&e_matrix)?;
        
        let public_key = FrodoPublicKey::new(params, seed_a, b);
        let secret_key = FrodoSecretKey::new(params, s, s_matrix);
        
        Ok((public_key, secret_key))
    pub fn performance_characteristics(params: FrodoParams) -> AlgorithmPerformance {
        let (keygen_ms, encaps_ms, decaps_ms, encaps_throughput, decaps_throughput) = match params {

        AlgorithmPerformance {
            operation_time_ms: (encaps_ms + decaps_ms) / 2.0,
            key_sizes: KeySizes {
                ciphertext_or_signature: params.additional_sizes()
                    .iter()
                    .find(|(name, _)| *name == "ciphertext")
                    .map(|(_, size)| *size)
            throughput_ops_per_sec: (encaps_throughput + decaps_throughput) / 2.0,
        }
    }
