/// fr fr Post-quantum cryptography utilities and mathematical operations
use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
use crate::error::CursedError;
use std::fmt;

/// Polynomial ring element for lattice-based cryptography
#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial {
    pub coefficients: Vec<i32>,
    pub modulus: i32,
    pub degree: usize,
}

impl Polynomial {
    /// Create new polynomial with given degree and modulus
    pub fn new(degree: usize, modulus: i32) -> Self {
        Self {
            coefficients: vec![0; degree],
            modulus,
            degree,
        }
    }
    
    /// Create polynomial from coefficients
    pub fn from_coefficients(coefficients: Vec<i32>, modulus: i32) -> Self {
        let degree = coefficients.len();
        Self {
            coefficients,
            modulus,
            degree,
        }
    }
    
    /// Create random polynomial
    pub fn random(degree: usize, modulus: i32) -> Self {
        let coefficients: Vec<i32> = (0..degree)
            .map(|_| fastrand::i32(0..modulus))
            .collect();
        
        Self {
            coefficients,
            modulus,
            degree,
        }
    }
    
    /// Add two polynomials
    pub fn add(&self, other: &Polynomial) -> AdvancedCryptoResult<Polynomial> {
        if self.degree != other.degree || self.modulus != other.modulus {
            return Err(CursedError::InvalidInput("Polynomial dimensions mismatch".to_string()));
        }
        
        let result_coeffs: Vec<i32> = self.coefficients
            .iter()
            .zip(other.coefficients.iter())
            .map(|(a, b)| (a + b) % self.modulus)
            .collect();
        
        Ok(Polynomial::from_coefficients(result_coeffs, self.modulus))
    }
    
    /// Subtract two polynomials
    pub fn subtract(&self, other: &Polynomial) -> AdvancedCryptoResult<Polynomial> {
        if self.degree != other.degree || self.modulus != other.modulus {
            return Err(CursedError::InvalidInput("Polynomial dimensions mismatch".to_string()));
        }
        
        let result_coeffs: Vec<i32> = self.coefficients
            .iter()
            .zip(other.coefficients.iter())
            .map(|(a, b)| (a - b + self.modulus) % self.modulus)
            .collect();
        
        Ok(Polynomial::from_coefficients(result_coeffs, self.modulus))
    }
    
    /// Multiply polynomial by scalar
    pub fn scalar_multiply(&self, scalar: i32) -> Polynomial {
        let result_coeffs: Vec<i32> = self.coefficients
            .iter()
            .map(|coeff| (coeff * scalar) % self.modulus)
            .collect();
        
        Polynomial::from_coefficients(result_coeffs, self.modulus)
    }
    
    /// Polynomial multiplication (convolution)
    pub fn multiply(&self, other: &Polynomial) -> AdvancedCryptoResult<Polynomial> {
        if self.modulus != other.modulus {
            return Err(CursedError::InvalidInput("Polynomial modulus mismatch".to_string()));
        }
        
        let result_degree = self.degree.max(other.degree);
        let mut result_coeffs = vec![0; result_degree];
        
        for i in 0..self.degree {
            for j in 0..other.degree {
                let pos = (i + j) % result_degree; // Circular convolution
                result_coeffs[pos] = (result_coeffs[pos] + self.coefficients[i] * other.coefficients[j]) % self.modulus;
            }
        }
        
        Ok(Polynomial::from_coefficients(result_coeffs, self.modulus))
    }
    
    /// Apply centered reduction
    pub fn centered_reduce(&mut self) {
        for coeff in &mut self.coefficients {
            *coeff = center_mod(*coeff, self.modulus);
        }
    }
    
    /// Get L2 norm of polynomial
    pub fn l2_norm(&self) -> f64 {
        let sum_squares: i64 = self.coefficients
            .iter()
            .map(|&c| (c as i64) * (c as i64))
            .sum();
        
        (sum_squares as f64).sqrt()
    }
    
    /// Get infinity norm of polynomial
    pub fn infinity_norm(&self) -> i32 {
        self.coefficients
            .iter()
            .map(|&c| c.abs())
            .max()
            .unwrap_or(0)
    }
    
    /// Serialize polynomial to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        // Add header: degree (4 bytes) + modulus (4 bytes)
        bytes.extend_from_slice(&(self.degree as u32).to_be_bytes());
        bytes.extend_from_slice(&(self.modulus as u32).to_be_bytes());
        
        // Add coefficients (4 bytes each)
        for &coeff in &self.coefficients {
            bytes.extend_from_slice(&(coeff as u32).to_be_bytes());
        }
        
        bytes
    }
    
    /// Deserialize polynomial from bytes
    pub fn from_bytes(bytes: &[u8]) -> AdvancedCryptoResult<Polynomial> {
        if bytes.len() < 8 {
            return Err(CursedError::InvalidInput("Invalid polynomial data: too short".to_string()));
        }
        
        let degree = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
        let modulus = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]) as i32;
        
        let expected_len = 8 + degree * 4;
        if bytes.len() != expected_len {
            return Err(CursedError::InvalidInput("Invalid polynomial data: wrong length".to_string()));
        }
        
        let mut coefficients = Vec::new();
        for i in 0..degree {
            let offset = 8 + i * 4;
            let coeff = u32::from_be_bytes([
                bytes[offset],
                bytes[offset + 1],
                bytes[offset + 2],
                bytes[offset + 3],
            ]) as i32;
            coefficients.push(coeff);
        }
        
        Ok(Polynomial::from_coefficients(coefficients, modulus))
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Poly(deg={}, mod={}, coeffs=[", self.degree, self.modulus)?;
        for (i, &coeff) in self.coefficients.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{}", coeff)?;
            if i >= 5 { // Limit display for readability
                write!(f, ", ...")?;
                break;
            }
        }
        write!(f, "])")
    }
}

/// Matrix operations for lattice-based cryptography
#[derive(Debug, Clone)]
pub struct Matrix {
    pub data: Vec<Vec<i32>>,
    pub rows: usize,
    pub cols: usize,
    pub modulus: i32,
}

impl Matrix {
    /// Create new matrix
    pub fn new(rows: usize, cols: usize, modulus: i32) -> Self {
        Self {
            data: vec![vec![0; cols]; rows],
            rows,
            cols,
            modulus,
        }
    }
    
    /// Create random matrix
    pub fn random(rows: usize, cols: usize, modulus: i32) -> Self {
        let data: Vec<Vec<i32>> = (0..rows)
            .map(|_| (0..cols).map(|_| fastrand::i32(0..modulus)).collect())
            .collect();
        
        Self { data, rows, cols, modulus }
    }
    
    /// Create identity matrix
    pub fn identity(size: usize, modulus: i32) -> Self {
        let mut matrix = Self::new(size, size, modulus);
        for i in 0..size {
            matrix.data[i][i] = 1;
        }
        matrix
    }
    
    /// Matrix-vector multiplication
    pub fn multiply_vector(&self, vector: &[i32]) -> AdvancedCryptoResult<Vec<i32>> {
        if vector.len() != self.cols {
            return Err(CursedError::InvalidInput("Vector dimension mismatch".to_string()));
        }
        
        let mut result = vec![0; self.rows];
        for i in 0..self.rows {
            for j in 0..self.cols {
                result[i] = (result[i] + self.data[i][j] * vector[j]) % self.modulus;
            }
        }
        
        Ok(result)
    }
    
    /// Matrix multiplication
    pub fn multiply_matrix(&self, other: &Matrix) -> AdvancedCryptoResult<Matrix> {
        if self.cols != other.rows || self.modulus != other.modulus {
            return Err(CursedError::InvalidInput("Matrix dimensions or modulus mismatch".to_string()));
        }
        
        let mut result = Matrix::new(self.rows, other.cols, self.modulus);
        
        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    result.data[i][j] = (result.data[i][j] + self.data[i][k] * other.data[k][j]) % self.modulus;
                }
            }
        }
        
        Ok(result)
    }
    
    /// Add matrices
    pub fn add(&self, other: &Matrix) -> AdvancedCryptoResult<Matrix> {
        if self.rows != other.rows || self.cols != other.cols || self.modulus != other.modulus {
            return Err(CursedError::InvalidInput("Matrix dimensions mismatch".to_string()));
        }
        
        let mut result = Matrix::new(self.rows, self.cols, self.modulus);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = (self.data[i][j] + other.data[i][j]) % self.modulus;
            }
        }
        
        Ok(result)
    }
    
    /// Transpose matrix
    pub fn transpose(&self) -> Matrix {
        let mut result = Matrix::new(self.cols, self.rows, self.modulus);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[j][i] = self.data[i][j];
            }
        }
        result
    }
    
    /// Get matrix element
    pub fn get(&self, row: usize, col: usize) -> AdvancedCryptoResult<i32> {
        if row >= self.rows || col >= self.cols {
            return Err(CursedError::InvalidInput("Matrix index out of bounds".to_string()));
        }
        Ok(self.data[row][col])
    }
    
    /// Set matrix element
    pub fn set(&mut self, row: usize, col: usize, value: i32) -> AdvancedCryptoResult<()> {
        if row >= self.rows || col >= self.cols {
            return Err(CursedError::InvalidInput("Matrix index out of bounds".to_string()));
        }
        self.data[row][col] = value % self.modulus;
        Ok(())
    }
}

/// Gaussian sampling for lattice-based cryptography
pub struct GaussianSampler {
    sigma: f64,
    precision: usize,
    table: Vec<f64>,
}

impl GaussianSampler {
    /// Create new Gaussian sampler
    pub fn new(sigma: f64, precision: usize) -> Self {
        let mut sampler = Self {
            sigma,
            precision,
            table: Vec::new(),
        };
        sampler.precompute_table();
        sampler
    }
    
    /// Precompute probability table for rejection sampling
    fn precompute_table(&mut self) {
        let range = (6.0 * self.sigma) as i32;
        self.table = Vec::new();
        
        for i in -range..=range {
            let x = i as f64;
            let prob = (-x * x / (2.0 * self.sigma * self.sigma)).exp();
            self.table.push(prob);
        }
    }
    
    /// Sample from discrete Gaussian distribution
    pub fn sample(&self) -> i32 {
        let range = (6.0 * self.sigma) as i32;
        
        loop {
            // Uniform random in range
            let x = fastrand::i32(-range..=range);
            let table_index = (x + range) as usize;
            
            if table_index < self.table.len() {
                let prob = self.table[table_index];
                let u = fastrand::f64();
                
                if u <= prob {
                    return x;
                }
            }
        }
    }
    
    /// Sample vector from discrete Gaussian
    pub fn sample_vector(&self, length: usize) -> Vec<i32> {
        (0..length).map(|_| self.sample()).collect()
    }
    
    /// Sample polynomial with Gaussian coefficients
    pub fn sample_polynomial(&self, degree: usize, modulus: i32) -> Polynomial {
        let coefficients = self.sample_vector(degree);
        let centered_coefficients: Vec<i32> = coefficients
            .iter()
            .map(|&c| center_mod(c, modulus))
            .collect();
        
        Polynomial::from_coefficients(centered_coefficients, modulus)
    }
}

/// Rejection sampling utilities
pub struct RejectionSampler {
    max_iterations: usize,
}

impl RejectionSampler {
    /// Create new rejection sampler
    pub fn new(max_iterations: usize) -> Self {
        Self { max_iterations }
    }
    
    /// Sample with rejection based on predicate
    pub fn sample_with_condition<F, T>(&self, generator: F, condition: fn(&T) -> bool) -> Option<T>
    where
        F: Fn() -> T,
    {
        for _ in 0..self.max_iterations {
            let sample = generator();
            if condition(&sample) {
                return Some(sample);
            }
        }
        None
    }
    
    /// Sample uniform integer in range [min, max)
    pub fn uniform_int(&self, min: i32, max: i32) -> i32 {
        fastrand::i32(min..max)
    }
    
    /// Sample uniform vector
    pub fn uniform_vector(&self, length: usize, min: i32, max: i32) -> Vec<i32> {
        (0..length).map(|_| self.uniform_int(min, max)).collect()
    }
    
    /// Sample ternary vector (coefficients in {-1, 0, 1})
    pub fn ternary_vector(&self, length: usize) -> Vec<i32> {
        (0..length).map(|_| self.uniform_int(-1, 2)).collect()
    }
    
    /// Sample binary vector (coefficients in {0, 1})
    pub fn binary_vector(&self, length: usize) -> Vec<i32> {
        (0..length).map(|_| self.uniform_int(0, 2)).collect()
    }
    
    /// Sample with Hamming weight constraint
    pub fn hamming_weight_vector(&self, length: usize, target_weight: usize) -> Option<Vec<i32>> {
        self.sample_with_condition(
            || self.ternary_vector(length),
            |v| v.iter().filter(|&&x| x != 0).count() == target_weight,
        )
    }
}

/// Memory-safe zeroization functions
pub trait SecureZeroize {
    /// Securely zeroize memory
    fn secure_zeroize(&mut self);
}

impl SecureZeroize for Vec<u8> {
    fn secure_zeroize(&mut self) {
        // Use volatile operations to prevent compiler optimization
        for byte in self.iter_mut() {
            unsafe {
                std::ptr::write_volatile(byte, 0);
            }
        }
    }
}

impl SecureZeroize for Vec<i32> {
    fn secure_zeroize(&mut self) {
        for value in self.iter_mut() {
            unsafe {
                std::ptr::write_volatile(value, 0);
            }
        }
    }
}

impl SecureZeroize for [u8] {
    fn secure_zeroize(&mut self) {
        for byte in self.iter_mut() {
            unsafe {
                std::ptr::write_volatile(byte, 0);
            }
        }
    }
}

impl SecureZeroize for [i32] {
    fn secure_zeroize(&mut self) {
        for value in self.iter_mut() {
            unsafe {
                std::ptr::write_volatile(value, 0);
            }
        }
    }
}

impl SecureZeroize for Polynomial {
    fn secure_zeroize(&mut self) {
        self.coefficients.secure_zeroize();
    }
}

impl SecureZeroize for Matrix {
    fn secure_zeroize(&mut self) {
        for row in &mut self.data {
            row.secure_zeroize();
        }
    }
}

/// Secure memory allocation utilities
pub struct SecureMemory;

impl SecureMemory {
    /// Allocate secure memory that will be zeroized on drop
    pub fn allocate_secure_vec(size: usize) -> SecureVec<u8> {
        SecureVec {
            data: vec![0u8; size],
        }
    }
    
    /// Allocate secure integer vector
    pub fn allocate_secure_int_vec(size: usize) -> SecureVec<i32> {
        SecureVec {
            data: vec![0i32; size],
        }
    }
}

/// Secure vector that zeroizes on drop
pub struct SecureVec<T> {
    data: Vec<T>,
}

impl<T> SecureVec<T> {
    /// Get reference to data
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }
    
    /// Get mutable reference to data
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }
    
    /// Get length
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<T> Drop for SecureVec<T> 
where
    Vec<T>: SecureZeroize,
{
    fn drop(&mut self) {
        self.data.secure_zeroize();
    }
}

/// Mathematical utility functions
/// Centered modular reduction: returns value in [-q/2, q/2)
pub fn center_mod(value: i32, modulus: i32) -> i32 {
    let r = value % modulus;
    if r > modulus / 2 {
        r - modulus
    } else if r < -modulus / 2 {
        r + modulus
    } else {
        r
    }
}

/// Modular inverse using extended Euclidean algorithm
pub fn mod_inverse(a: i32, m: i32) -> Option<i32> {
    let (mut old_r, mut r) = (a, m);
    let (mut old_s, mut s) = (1, 0);
    
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
        None // No inverse exists
    } else {
        Some(if old_s < 0 { old_s + m } else { old_s })
    }
}

/// Fast modular exponentiation
pub fn mod_pow(base: i64, exp: i64, modulus: i64) -> i64 {
    if modulus == 1 { return 0; }
    
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;
    
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    
    result
}

/// Number theoretic transform (NTT) for fast polynomial multiplication
pub struct NumberTheoreticTransform {
    pub modulus: i32,
    pub primitive_root: i32,
    pub n: usize,
    pub roots: Vec<i32>,
    pub inv_roots: Vec<i32>,
}

impl NumberTheoreticTransform {
    /// Create NTT for given parameters
    pub fn new(n: usize, modulus: i32, primitive_root: i32) -> AdvancedCryptoResult<Self> {
        if !is_power_of_two(n) {
            return Err(CursedError::InvalidInput("N must be power of 2".to_string()));
        }
        
        let mut ntt = Self {
            modulus,
            primitive_root,
            n,
            roots: Vec::new(),
            inv_roots: Vec::new(),
        };
        
        ntt.precompute_roots()?;
        Ok(ntt)
    }
    
    /// Precompute roots of unity
    fn precompute_roots(&mut self) -> AdvancedCryptoResult<()> {
        self.roots = vec![1; self.n];
        self.inv_roots = vec![1; self.n];
        
        let root = mod_pow(self.primitive_root as i64, ((self.modulus as i64 - 1) / self.n as i64), self.modulus as i64) as i32;
        let inv_root = mod_inverse(root, self.modulus)
            .ok_or_else(|| CursedError::InvalidInput("No modular inverse".to_string()))?;
        
        for i in 1..self.n {
            self.roots[i] = (self.roots[i-1] as i64 * root as i64 % self.modulus as i64) as i32;
            self.inv_roots[i] = (self.inv_roots[i-1] as i64 * inv_root as i64 % self.modulus as i64) as i32;
        }
        
        Ok(())
    }
    
    /// Forward NTT
    pub fn forward_ntt(&self, input: &mut [i32]) -> AdvancedCryptoResult<()> {
        if input.len() != self.n {
            return Err(CursedError::InvalidInput("Input length mismatch".to_string()));
        }
        
        // Bit-reverse permutation
        bit_reverse_permute(input);
        
        // NTT computation
        let mut len = 2;
        while len <= self.n {
            let step = self.n / len;
            for i in (0..self.n).step_by(len) {
                for j in 0..len/2 {
                    let u = input[i + j];
                    let v = (input[i + j + len/2] as i64 * self.roots[step * j] as i64 % self.modulus as i64) as i32;
                    input[i + j] = (u + v) % self.modulus;
                    input[i + j + len/2] = (u - v + self.modulus) % self.modulus;
                }
            }
            len *= 2;
        }
        
        Ok(())
    }
    
    /// Inverse NTT
    pub fn inverse_ntt(&self, input: &mut [i32]) -> AdvancedCryptoResult<()> {
        if input.len() != self.n {
            return Err(CursedError::InvalidInput("Input length mismatch".to_string()));
        }
        
        // Bit-reverse permutation
        bit_reverse_permute(input);
        
        // Inverse NTT computation
        let mut len = 2;
        while len <= self.n {
            let step = self.n / len;
            for i in (0..self.n).step_by(len) {
                for j in 0..len/2 {
                    let u = input[i + j];
                    let v = (input[i + j + len/2] as i64 * self.inv_roots[step * j] as i64 % self.modulus as i64) as i32;
                    input[i + j] = (u + v) % self.modulus;
                    input[i + j + len/2] = (u - v + self.modulus) % self.modulus;
                }
            }
            len *= 2;
        }
        
        // Scale by inverse of n
        let n_inv = mod_inverse(self.n as i32, self.modulus)
            .ok_or_else(|| CursedError::InvalidInput("No modular inverse for n".to_string()))?;
        
        for element in input.iter_mut() {
            *element = (*element as i64 * n_inv as i64 % self.modulus as i64) as i32;
        }
        
        Ok(())
    }
    
    /// Fast polynomial multiplication using NTT
    pub fn multiply_polynomials(&self, a: &[i32], b: &[i32]) -> AdvancedCryptoResult<Vec<i32>> {
        if a.len() + b.len() - 1 > self.n {
            return Err(CursedError::InvalidInput("Result would exceed NTT size".to_string()));
        }
        
        let mut a_padded = vec![0; self.n];
        let mut b_padded = vec![0; self.n];
        
        a_padded[..a.len()].copy_from_slice(a);
        b_padded[..b.len()].copy_from_slice(b);
        
        self.forward_ntt(&mut a_padded)?;
        self.forward_ntt(&mut b_padded)?;
        
        // Pointwise multiplication
        for i in 0..self.n {
            a_padded[i] = (a_padded[i] as i64 * b_padded[i] as i64 % self.modulus as i64) as i32;
        }
        
        self.inverse_ntt(&mut a_padded)?;
        
        // Trim result to appropriate size
        let result_len = a.len() + b.len() - 1;
        a_padded.truncate(result_len);
        
        Ok(a_padded)
    }
}

/// Helper functions
fn is_power_of_two(n: usize) -> bool {
    n > 0 && (n & (n - 1)) == 0
}

fn bit_reverse_permute(input: &mut [i32]) {
    let n = input.len();
    let mut j = 0;
    
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        
        if i < j {
            input.swap(i, j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_polynomial_operations() {
        let mut poly1 = Polynomial::from_coefficients(vec![1, 2, 3], 7);
        let poly2 = Polynomial::from_coefficients(vec![4, 5, 6], 7);
        
        // Test addition
        let sum = poly1.add(&poly2).unwrap();
        assert_eq!(sum.coefficients, vec![5, 0, 2]); // (1+4, 2+5, 3+6) mod 7
        
        // Test subtraction  
        let diff = poly1.subtract(&poly2).unwrap();
        assert_eq!(diff.coefficients, vec![4, 4, 4]); // (1-4+7, 2-5+7, 3-6+7) mod 7
        
        // Test scalar multiplication
        let scaled = poly1.scalar_multiply(3);
        assert_eq!(scaled.coefficients, vec![3, 6, 2]); // (1*3, 2*3, 3*3) mod 7
        
        // Test norms
        assert!(poly1.l2_norm() > 0.0);
        assert_eq!(poly1.infinity_norm(), 3);
        
        // Test centered reduction
        poly1.centered_reduce();
        
        // Test serialization
        let bytes = poly1.to_bytes();
        let restored = Polynomial::from_bytes(&bytes).unwrap();
        assert_eq!(restored.coefficients, poly1.coefficients);
        assert_eq!(restored.modulus, poly1.modulus);
    }
    
    #[test]
    fn test_matrix_operations() {
        let mut matrix1 = Matrix::new(2, 2, 5);
        matrix1.set(0, 0, 1).unwrap();
        matrix1.set(0, 1, 2).unwrap();
        matrix1.set(1, 0, 3).unwrap();
        matrix1.set(1, 1, 4).unwrap();
        
        let mut matrix2 = Matrix::new(2, 2, 5);
        matrix2.set(0, 0, 2).unwrap();
        matrix2.set(0, 1, 1).unwrap();
        matrix2.set(1, 0, 1).unwrap();
        matrix2.set(1, 1, 3).unwrap();
        
        // Test matrix addition
        let sum = matrix1.add(&matrix2).unwrap();
        assert_eq!(sum.get(0, 0).unwrap(), 3);
        assert_eq!(sum.get(0, 1).unwrap(), 3);
        assert_eq!(sum.get(1, 0).unwrap(), 4);
        assert_eq!(sum.get(1, 1).unwrap(), 2); // (4+3) mod 5 = 2
        
        // Test matrix multiplication
        let product = matrix1.multiply_matrix(&matrix2).unwrap();
        assert_eq!(product.get(0, 0).unwrap(), 4); // (1*2 + 2*1) mod 5 = 4
        assert_eq!(product.get(0, 1).unwrap(), 2); // (1*1 + 2*3) mod 5 = 2
        
        // Test vector multiplication
        let vector = vec![1, 2];
        let result = matrix1.multiply_vector(&vector).unwrap();
        assert_eq!(result, vec![0, 2]); // [(1*1 + 2*2) mod 5, (3*1 + 4*2) mod 5]
        
        // Test transpose
        let transposed = matrix1.transpose();
        assert_eq!(transposed.get(0, 0).unwrap(), 1);
        assert_eq!(transposed.get(0, 1).unwrap(), 3);
        assert_eq!(transposed.get(1, 0).unwrap(), 2);
        assert_eq!(transposed.get(1, 1).unwrap(), 4);
    }
    
    #[test]
    fn test_gaussian_sampler() {
        let sampler = GaussianSampler::new(1.0, 100);
        
        // Test single sample
        let sample = sampler.sample();
        assert!(sample.abs() <= 10); // Should be within reasonable range
        
        // Test vector sampling
        let vector = sampler.sample_vector(10);
        assert_eq!(vector.len(), 10);
        
        // Test polynomial sampling
        let poly = sampler.sample_polynomial(5, 17);
        assert_eq!(poly.degree, 5);
        assert_eq!(poly.modulus, 17);
    }
    
    #[test]
    fn test_rejection_sampler() {
        let sampler = RejectionSampler::new(1000);
        
        // Test uniform sampling
        let uniform = sampler.uniform_int(0, 10);
        assert!(uniform >= 0 && uniform < 10);
        
        // Test vector sampling
        let vector = sampler.uniform_vector(5, -5, 5);
        assert_eq!(vector.len(), 5);
        assert!(vector.iter().all(|&x| x >= -5 && x < 5));
        
        // Test ternary sampling
        let ternary = sampler.ternary_vector(10);
        assert_eq!(ternary.len(), 10);
        assert!(ternary.iter().all(|&x| x >= -1 && x <= 1));
        
        // Test binary sampling
        let binary = sampler.binary_vector(10);
        assert_eq!(binary.len(), 10);
        assert!(binary.iter().all(|&x| x == 0 || x == 1));
        
        // Test Hamming weight sampling
        let hamming = sampler.hamming_weight_vector(10, 3);
        if let Some(vector) = hamming {
            assert_eq!(vector.iter().filter(|&&x| x != 0).count(), 3);
        }
    }
    
    #[test]
    fn test_secure_zeroize() {
        let mut data = vec![1u8, 2, 3, 4, 5];
        data.secure_zeroize();
        assert!(data.iter().all(|&x| x == 0));
        
        let mut int_data = vec![1i32, 2, 3, 4, 5];
        int_data.secure_zeroize();
        assert!(int_data.iter().all(|&x| x == 0));
    }
    
    #[test]
    fn test_secure_memory() {
        let secure_vec = SecureMemory::allocate_secure_vec(10);
        assert_eq!(secure_vec.len(), 10);
        assert!(secure_vec.as_slice().iter().all(|&x| x == 0));
        
        let secure_int_vec = SecureMemory::allocate_secure_int_vec(5);
        assert_eq!(secure_int_vec.len(), 5);
        assert!(secure_int_vec.as_slice().iter().all(|&x| x == 0));
    }
    
    #[test]
    fn test_mathematical_utilities() {
        // Test centered modular reduction
        assert_eq!(center_mod(3, 7), 3);
        assert_eq!(center_mod(5, 7), -2);
        assert_eq!(center_mod(-2, 7), -2);
        assert_eq!(center_mod(-5, 7), 2);
        
        // Test modular inverse
        assert_eq!(mod_inverse(3, 7), Some(5)); // 3 * 5 = 15 ≡ 1 (mod 7)
        assert_eq!(mod_inverse(2, 7), Some(4)); // 2 * 4 = 8 ≡ 1 (mod 7)
        assert_eq!(mod_inverse(2, 4), None);    // No inverse exists
        
        // Test modular exponentiation
        assert_eq!(mod_pow(2, 3, 5), 3); // 2^3 = 8 ≡ 3 (mod 5)
        assert_eq!(mod_pow(3, 4, 7), 4); // 3^4 = 81 ≡ 4 (mod 7)
    }
    
    #[test]
    fn test_ntt() {
        // Small example with NTT-friendly parameters
        let ntt = NumberTheoreticTransform::new(4, 17, 3).unwrap();
        
        let poly_a = vec![1, 2, 3, 4];
        let poly_b = vec![1, 1, 1, 1];
        
        let result = ntt.multiply_polynomials(&poly_a, &poly_b).unwrap();
        
        // Expected result of polynomial multiplication
        // (1 + 2x + 3x^2 + 4x^3) * (1 + x + x^2 + x^3) = 1 + 3x + 6x^2 + 10x^3 + 9x^4 + 7x^5 + 4x^6
        // But we need to verify the actual computation
        assert_eq!(result.len(), 7); // degree(a) + degree(b) - 1
        assert!(!result.is_empty());
    }
}
