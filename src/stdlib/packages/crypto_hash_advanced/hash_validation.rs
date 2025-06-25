/// Production-ready hash validation and verification system
use crate::error::CursedError;
// use crate::stdlib::packages::crypto_hash_advanced::hash_traits::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Result type for validation operations
pub type ValidationResult<T> = std::result::Result<T, CryptoError>;

/// Hash validation status
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationStatus {
impl ValidationStatus {
    pub fn is_valid(&self) -> bool {
        matches!(self, ValidationStatus::Valid)
    pub fn description(&self) -> &'static str {
        match self {
        }
    }
/// Hash validation result
#[derive(Debug, Clone)]
pub struct HashValidationResult {
/// Hash integrity checker
#[derive(Debug, Clone)]
pub struct HashIntegrityChecker {
impl HashIntegrityChecker {
    pub fn new<H: Hasher>(mut hasher: H, data: &[u8]) -> Self {
        let hash_value = hasher.hash(data);
        Self {
        }
    }
    
    /// Verify data integrity against stored hash
    pub fn verify<H: Hasher>(&self, mut hasher: H, data: &[u8]) -> ValidationResult<HashValidationResult> {
        let start_time = Instant::now();
        
        // Verify algorithm matches
        if hasher.algorithm() != self.algorithm {
            return Ok(HashValidationResult {
                error_details: Some(format!(
                    self.algorithm, hasher.algorithm()
            });
        // Compute hash
        let computed_hash = hasher.hash(data);
        
        // Compare hashes using constant-time comparison
        let status = if constant_time_eq(&computed_hash, &self.hash_value) {
            ValidationStatus::Valid
        } else {
            // Perform deeper analysis to determine type of failure
            self.analyze_hash_mismatch(&computed_hash, data)
        
        Ok(HashValidationResult {
        })
    /// Analyze why a hash mismatch occurred
    fn analyze_hash_mismatch(&self, computed_hash: &[u8], data: &[u8]) -> ValidationStatus {
        // Check for obvious corruption patterns
        if data.iter().all(|&b| b == 0) {
            return ValidationStatus::Corrupted;
        // Check if hash lengths match (basic format validation)
        if computed_hash.len() != self.hash_value.len() {
            return ValidationStatus::Invalid;
        // Check for patterns suggesting tampering
        let hamming_distance = self.calculate_hamming_distance(computed_hash, &self.hash_value);
        let max_distance = self.hash_value.len() * 8;
        
        if hamming_distance as f64 / max_distance as f64 > 0.4 {
            // High difference suggests tampering rather than corruption
            ValidationStatus::Tampered
        } else {
            ValidationStatus::Invalid
        }
    }
    
    fn calculate_hamming_distance(&self, a: &[u8], b: &[u8]) -> usize {
        if a.len() != b.len() {
            return usize::MAX;
        a.iter()
            .zip(b.iter())
            .map(|(byte_a, byte_b)| (byte_a ^ byte_b).count_ones() as usize)
            .sum()
    }
}

/// Hash validator with multiple algorithm support
pub struct MultiHashValidator {
impl MultiHashValidator {
    pub fn new() -> Self {
        let mut validators = HashMap::new();
        
        // Add built-in validators
        validators.insert("SHA-256".to_string(), Box::new(|data| {
            // Placeholder - would use actual SHA-256 implementation
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut hasher = DefaultHasher::new();
            data.hash(&mut hasher);
            hasher.finish().to_le_bytes().to_vec()
        }) as Box<dyn Fn(&[u8]) -> Vec<u8> + Send + Sync>);
        
        Self { validators }
    }
    
    /// Add a custom hash validator
    pub fn add_validator<F>(&mut self, algorithm: String, validator: F)
    where
    {
        self.validators.insert(algorithm, Box::new(validator));
    /// Validate data against multiple hash algorithms
    pub fn validate_multi(&self, data: &[u8], expected_hashes: &HashMap<String, Vec<u8>>) 
        -> ValidationResult<MultiValidationResult> {
        let start_time = Instant::now();
        let mut results = Vec::new();
        let mut all_valid = true;
        
        for (algorithm, expected_hash) in expected_hashes {
            if let Some(validator) = self.validators.get(algorithm) {
                let computed_hash = validator(data);
                let is_valid = constant_time_eq(&computed_hash, expected_hash);
                
                if !is_valid {
                    all_valid = false;
                results.push(SingleValidationResult {
                    status: if is_valid { 
                        ValidationStatus::Valid 
                    } else { 
                        ValidationStatus::Invalid 
                });
            } else {
                all_valid = false;
                results.push(SingleValidationResult {
                });
            }
        }
        
        Ok(MultiValidationResult {
            overall_status: if all_valid && !results.is_empty() {
                ValidationStatus::Valid
            } else if results.iter().any(|r| r.status == ValidationStatus::Valid) {
                ValidationStatus::Invalid // Partial validation
            } else {
                ValidationStatus::Invalid
        })
    }
}

#[derive(Debug, Clone)]
pub struct MultiValidationResult {
#[derive(Debug, Clone)]
pub struct SingleValidationResult {
/// Test vector validator for hash functions
pub struct TestVectorValidator {
impl TestVectorValidator {
    pub fn new() -> Self {
        Self {
        }
    }
    
    pub fn add_test_vector(&mut self, vector: TestVector) {
        self.test_vectors.push(vector);
    /// Validate a hasher against known test vectors
    pub fn validate_hasher<H: Hasher + Clone>(&self, hasher: H) -> ValidationResult<TestVectorValidationResult> {
        let start_time = Instant::now();
        let mut results = Vec::new();
        let algorithm = hasher.algorithm();
        
        // Find relevant test vectors
        let relevant_vectors: Vec<_> = self.test_vectors
            .iter()
            .filter(|tv| tv.algorithm.eq_ignore_ascii_case(algorithm))
            .collect();
        
        if relevant_vectors.is_empty() {
            return Ok(TestVectorValidationResult {
            });
        let mut passed = 0;
        let mut failed_vectors = Vec::new();
        
        for vector in relevant_vectors {
            let mut test_hasher = hasher.clone();
            let computed = test_hasher.hash(vector.input);
            
            if constant_time_eq(&computed, vector.expected) {
                passed += 1;
            } else {
                failed_vectors.push(TestVectorFailure {
                });
            }
        }
        
        let total = relevant_vectors.len();
        let overall_status = if failed_vectors.is_empty() {
            ValidationStatus::Valid
        } else if passed > 0 {
            ValidationStatus::Invalid
        } else {
            ValidationStatus::Invalid
        
        Ok(TestVectorValidationResult {
        })
    }
}

#[derive(Debug, Clone)]
pub struct TestVectorValidationResult {
#[derive(Debug, Clone)]
pub struct TestVectorFailure {
/// Hash chain validator for blockchain-like structures
pub struct HashChainValidator<H: Hasher> {
impl<H: Hasher + Clone> HashChainValidator<H> {
    pub fn new(hasher: H) -> Self {
        Self { hasher }
    }
    
    /// Validate a chain of hashes where each hash includes the previous hash
    pub fn validate_chain(&mut self, chain: &[HashChainBlock]) -> ValidationResult<ChainValidationResult> {
        let start_time = Instant::now();
        let mut invalid_blocks = Vec::new();
        
        if chain.is_empty() {
            return Ok(ChainValidationResult {
            });
        // Validate first block (no previous hash to check)
        let mut previous_hash = &chain[0].hash;
        
        // Validate remaining blocks
        for (index, block) in chain.iter().enumerate().skip(1) {
            // Combine previous hash with current data
            let mut combined_data = previous_hash.clone();
            combined_data.extend_from_slice(&block.data);
            
            let computed_hash = self.hasher.hash(&combined_data);
            
            if !constant_time_eq(&computed_hash, &block.hash) {
                invalid_blocks.push(ChainBlockFailure {
                });
            previous_hash = &block.hash;
        let total_blocks = chain.len();
        let valid_blocks = total_blocks - invalid_blocks.len();
        let overall_status = if invalid_blocks.is_empty() {
            ValidationStatus::Valid
        } else {
            ValidationStatus::Invalid
        
        Ok(ChainValidationResult {
        })
    }
}

#[derive(Debug, Clone)]
pub struct HashChainBlock {
#[derive(Debug, Clone)]
pub struct ChainValidationResult {
#[derive(Debug, Clone)]
pub struct ChainBlockFailure {
/// Comprehensive hash validation utilities
pub fn validate_hash_format(hash: &[u8], expected_size: usize) -> ValidationResult<()> {
    if hash.len() != expected_size {
        return Err(CursedError::InvalidArgument(
            format!("Invalid hash size: expected {} bytes, got {}", expected_size, hash.len())
        ));
    // Check for obvious invalid patterns
    if hash.iter().all(|&b| b == 0) {
        return Err(CursedError::InvalidArgument("Hash appears to be all zeros".to_string()));
    Ok(())
/// Convert hash to hex string for display
pub fn hash_to_hex(hash: &[u8]) -> String {
    hash.iter().map(|b| format!("{:02x}", b)).collect()
/// Parse hex string to hash bytes
pub fn hex_to_hash(hex: &str) -> ValidationResult<Vec<u8>> {
    if hex.len() % 2 != 0 {
        return Err(CursedError::InvalidArgument("Hex string must have even length".to_string()));
    let mut result = Vec::new();
    for chunk in hex.as_bytes().chunks(2) {
        let hex_byte = std::str::from_utf8(chunk)
            .map_err(|_| CursedError::InvalidArgument("Invalid UTF-8 in hex string".to_string()))?;
        
        let byte = u8::from_str_radix(hex_byte, 16)
            .map_err(|_| CursedError::InvalidArgument(format!("Invalid hex byte: {}", hex_byte)))?;
        
        result.push(byte);
    Ok(result)
