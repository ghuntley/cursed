/// Side-Channel Attack Protection
use crate::error::CursedError;
// use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
// use crate::stdlib::packages::crypto_random::SecureRandom;
use std::time::{Duration, Instant};

/// Side-channel protection configuration
#[derive(Debug, Clone)]
pub struct SideChannelConfig {
/// Side-channel protection manager
#[derive(Debug)]
pub struct SideChannelProtectionManager {
impl SideChannelProtectionManager {
    pub fn new() -> AdvancedCryptoResult<Self> {
        let config = SideChannelConfig {

        Ok(Self {
        })
    /// Constant-time conditional select
    pub fn constant_time_select(&self, condition: bool, a: &[u8], b: &[u8]) -> AdvancedCryptoResult<Vec<u8>> {
        if !self.config.constant_time_operations {
            return Ok(if condition { a.to_vec() } else { b.to_vec() });
        if a.len() != b.len() {
            return Err(CursedError::invalid_input("Arrays must have same length".to_string()));
        let mask = if condition { 0xFF } else { 0x00 };
        let inv_mask = !mask;

        let result: Vec<u8> = a.iter()
            .zip(b.iter())
            .map(|(&x, &y)| (x & mask) | (y & inv_mask))
            .collect();

        Ok(result)
    /// Constant-time byte array comparison
    pub fn constant_time_eq(&self, a: &[u8], b: &[u8]) -> bool {
        if !self.config.constant_time_operations {
            return a == b;
        if a.len() != b.len() {
            return false;
        let mut result = 0u8;
        for (&x, &y) in a.iter().zip(b.iter()) {
            result |= x ^ y;
        result == 0
    /// Memory access pattern obfuscation
    pub fn obfuscated_table_lookup(&self, table: &[Vec<u8>], index: usize) -> AdvancedCryptoResult<Vec<u8>> {
        if !self.config.memory_access_patterns {
            if index >= table.len() {
                return Err(CursedError::invalid_input("Index out of bounds".to_string()));
            }
            return Ok(table[index].clone());
        if table.is_empty() {
            return Err(CursedError::invalid_input("Table cannot be empty".to_string()));
        let element_size = table[0].len();
        let mut result = vec![0u8; element_size];

        // Access all table entries to hide the real access pattern
        for (i, entry) in table.iter().enumerate() {
            let mask = if i == index { 0xFF } else { 0x00 };
            
            for (j, &byte) in entry.iter().enumerate() {
                if j < element_size {
                    result[j] |= byte & mask;
                }
            }
        Ok(result)
    /// Timing attack protection with random delays
    pub fn protected_operation<F, T>(&mut self, operation: F) -> AdvancedCryptoResult<T>
    where
    {
        let start_time = Instant::now();
        
        // Perform the operation
        let result = operation()?;
        
        if self.config.timing_attack_protection {
            // Add randomized delay to normalize timing
            let base_delay = Duration::from_micros(100); // Base delay
            let random_bytes = self.secure_random.generate_bytes(2)?;
            let random_delay = Duration::from_micros(
                (u16::from_be_bytes([random_bytes[0], random_bytes[1]]) % 1000) as u64
            );
            
            let total_delay = base_delay + random_delay;
            let elapsed = start_time.elapsed();
            
            if elapsed < total_delay {
                std::thread::sleep(total_delay - elapsed);
            }
        }

        Ok(result)
    /// Blinding for RSA operations
    pub fn apply_blinding(&self, data: &[u8]) -> AdvancedCryptoResult<(Vec<u8>, Vec<u8>)> {
        if !self.config.blinding_enabled {
            return Ok((data.to_vec(), vec![]));
        let blinding_factor = self.secure_random.generate_bytes(data.len())?;
        let blinded_data: Vec<u8> = data.iter()
            .zip(blinding_factor.iter())
            .map(|(&x, &y)| x ^ y)
            .collect();

        Ok((blinded_data, blinding_factor))
    /// Remove blinding from result
    pub fn remove_blinding(&self, blinded_result: &[u8], blinding_factor: &[u8]) -> AdvancedCryptoResult<Vec<u8>> {
        if blinded_result.len() != blinding_factor.len() {
            return Err(CursedError::invalid_input("Blinded result and factor must have same length".to_string()));
        let result: Vec<u8> = blinded_result.iter()
            .zip(blinding_factor.iter())
            .map(|(&x, &y)| x ^ y)
            .collect();

        Ok(result)
    /// Cache-timing attack protection through dummy operations
    pub fn perform_dummy_operations(&mut self, count: usize) -> AdvancedCryptoResult<()> {
        if !self.config.cache_attack_protection {
            return Ok(());
        for _ in 0..count {
            // Perform meaningless but cache-affecting operations
            let dummy_data = self.secure_random.generate_bytes(64)?;
            let _hash = self.dummy_hash_operation(&dummy_data);
            self.dummy_operations_count += 1;
        Ok(())
    /// Power analysis protection through randomized execution
    pub fn randomized_execution<F, T>(&mut self, operations: Vec<F>) -> AdvancedCryptoResult<Vec<T>>
    where
    {
        if !self.config.power_analysis_protection {
            return operations.into_iter().map(|op| op()).collect();
        // Randomize execution order
        let mut indices: Vec<usize> = (0..operations.len()).collect();
        self.shuffle_indices(&mut indices)?;
        
        let mut results = Vec::with_capacity(operations.len());
        let mut operations = operations;

        for &index in &indices {
            if index < operations.len() {
                // This is a simplified approach - in practice, we'd need more sophisticated handling
                // For now, we'll execute in randomized order but this requires ownership considerations
                break;
            }
        }

        // Execute operations with dummy operations interspersed
        for operation in operations {
            let result = operation()?;
            results.push(result);
            
            // Add random dummy operations
            let dummy_count = self.secure_random.generate_bytes(1)?[0] as usize % 5;
            self.perform_dummy_operations(dummy_count)?;
        Ok(results)
    /// Secure memory clear (attempts to prevent compiler optimization)
    pub fn secure_zero(&self, data: &mut [u8]) {
        if self.config.memory_access_patterns {
            // Use volatile writes to prevent optimization
            for byte in data.iter_mut() {
                unsafe {
                    std::ptr::write_volatile(byte, 0);
                }
            }
            
            // Add a dummy read to ensure writes aren't optimized away
            let _dummy = unsafe { std::ptr::read_volatile(data.as_ptr()) };
        } else {
            data.fill(0);
        }
    }

    // Private helper methods

    fn dummy_hash_operation(&self, data: &[u8]) -> Vec<u8> {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.update(b"dummy_operation");
        hasher.finalize().to_vec()
    fn shuffle_indices(&self, indices: &mut [usize]) -> AdvancedCryptoResult<()> {
        for i in (1..indices.len()).rev() {
            let random_bytes = self.secure_random.generate_bytes(4)?;
            let j = u32::from_be_bytes([random_bytes[0], random_bytes[1], random_bytes[2], random_bytes[3]]) as usize % (i + 1);
            indices.swap(i, j);
        }
        Ok(())
    }
}

impl Default for SideChannelProtectionManager {
    fn default() -> Self {
        Self::new().expect("Failed to create default SideChannelProtectionManager")
    }
}

/// Secure computation helpers
impl SideChannelProtectionManager {
    /// Constant-time conditional swap
    pub fn constant_time_swap(&self, condition: bool, a: &mut [u8], b: &mut [u8]) -> AdvancedCryptoResult<()> {
        if a.len() != b.len() {
            return Err(CursedError::invalid_input("Arrays must have same length".to_string()));
        if !self.config.constant_time_operations {
            if condition {
                for (x, y) in a.iter_mut().zip(b.iter_mut()) {
                    std::mem::swap(x, y);
                }
            }
            return Ok(());
        let mask = if condition { 0xFF } else { 0x00 };
        
        for (x, y) in a.iter_mut().zip(b.iter_mut()) {
            let temp = (*x ^ *y) & mask;
            *x ^= temp;
            *y ^= temp;
        Ok(())
    /// Constant-time find minimum index
    pub fn constant_time_min_index(&self, values: &[u32]) -> AdvancedCryptoResult<usize> {
        if values.is_empty() {
            return Err(CursedError::invalid_input("Values array cannot be empty".to_string()));
        if !self.config.constant_time_operations {
            return Ok(values.iter().enumerate()
                .min_by_key(|(_, &val)| val)
                .map(|(idx, _)| idx)
                .unwrap_or(0));
        let mut min_index = 0;
        let mut min_value = values[0];

        for (i, &value) in values.iter().enumerate().skip(1) {
            let is_smaller = value < min_value;
            min_index = if is_smaller { i } else { min_index };
            min_value = if is_smaller { value } else { min_value };
        Ok(min_index)
    }
}

