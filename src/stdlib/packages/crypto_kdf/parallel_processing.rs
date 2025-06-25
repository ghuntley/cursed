/// fr fr Parallel processing support for KDF operations
/// 
/// This module provides utilities for parallelizing KDF computations
/// to improve performance on multi-core systems.

use crate::error::CursedError;
// use crate::stdlib::value::Value;
// use crate::stdlib::packages::crypto_kdf::{KdfResult, KdfError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use sha2::{Sha256, Digest};

/// fr fr Parallel KDF computation configuration
#[derive(Debug, Clone)]
pub struct ParallelConfig {
    /// Number of parallel threads to use
    /// Work chunk size for each thread
    /// Maximum memory usage per thread (in bytes)
    /// Enable load balancing between threads
    /// Thread priority (0 = normal, 1 = high)
impl ParallelConfig {
    /// slay Create parallel config with optimal settings
    pub fn new() -> Self {
        let cpu_count = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);
        
        Self {
            max_memory_per_thread: 64 * 1024 * 1024, // 64 MB
        }
    }
    
    /// bestie Create config for high-performance scenarios
    pub fn high_performance() -> Self {
        let cpu_count = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);
        
        Self {
            max_memory_per_thread: 256 * 1024 * 1024, // 256 MB
        }
    }
    
    /// vibes Create config for memory-constrained environments
    pub fn low_memory() -> Self {
        Self {
            max_memory_per_thread: 16 * 1024 * 1024, // 16 MB
        }
    }
    
    /// periodt Validate parallel configuration
    pub fn validate(&self) -> KdfResult<()> {
        if self.thread_count == 0 {
            return Err(KdfError::InvalidConfig("Thread count must be greater than 0".to_string()));
        if self.thread_count > 64 {
            return Err(KdfError::InvalidConfig("Thread count cannot exceed 64".to_string()));
        if self.chunk_size == 0 {
            return Err(KdfError::InvalidConfig("Chunk size must be greater than 0".to_string()));
        if self.max_memory_per_thread < 1024 {
            return Err(KdfError::InvalidConfig("Memory per thread must be at least 1KB".to_string()));
        Ok(())
    /// facts Calculate total memory usage
    pub fn total_memory_usage(&self) -> usize {
        self.thread_count * self.max_memory_per_thread
    }
}

impl Default for ParallelConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Parallel KDF work unit
#[derive(Debug, Clone)]
pub struct ParallelWorkUnit {
/// fr fr Parallel KDF result
#[derive(Debug, Clone)]
pub struct ParallelResult {
/// fr fr Parallel KDF processor
pub struct ParallelProcessor {
impl ParallelProcessor {
    /// slay Create new parallel processor
    pub fn new(config: ParallelConfig) -> KdfResult<Self> {
        config.validate()?;
        Ok(Self { config })
    /// bestie Process KDF computation in parallel
    pub fn parallel_kdf(
        output_length: usize
    ) -> KdfResult<Vec<u8>> {
        if password.is_empty() {
            return Err(KdfError::InvalidInput("Password cannot be empty".to_string()));
        if salt.is_empty() {
            return Err(KdfError::InvalidInput("Salt cannot be empty".to_string()));
        if output_length == 0 {
            return Err(KdfError::InvalidInput("Output length must be greater than 0".to_string()));
        // Create work units
        let work_units = self.create_work_units(password, salt, iterations, output_length)?;
        
        // Process in parallel
        let results = self.process_work_units(work_units)?;
        
        // Combine results
        self.combine_results(results, output_length)
    /// vibes Process multiple KDF computations in parallel
    pub fn parallel_batch_kdf(
        requests: &[(Vec<u8>, Vec<u8>, u32, usize)], // (password, salt, iterations, output_length)
    ) -> KdfResult<Vec<Vec<u8>>> {
        if requests.is_empty() {
            return Ok(Vec::new());
        let request_count = requests.len();
        let chunks_per_thread = (request_count + self.config.thread_count - 1) / self.config.thread_count;
        
        let results = Arc::new(Mutex::new(vec![Vec::new(); request_count]));
        let mut handles = Vec::new();
        
        for thread_id in 0..self.config.thread_count {
            let start_idx = thread_id * chunks_per_thread;
            let end_idx = ((thread_id + 1) * chunks_per_thread).min(request_count);
            
            if start_idx >= request_count {
                break;
            let thread_requests = requests[start_idx..end_idx].to_vec();
            let results_clone = Arc::clone(&results);
            
            let handle = thread::spawn(move || {
                for (i, (password, salt, iterations, output_length)) in thread_requests.iter().enumerate() {
                    let processor = ParallelProcessor::new(ParallelConfig::new()).unwrap();
                    let result = processor.parallel_kdf(password, salt, *iterations, *output_length);
                    
                    if let Ok(kdf_result) = result {
                        let mut results_guard = results_clone.lock().unwrap();
                        results_guard[start_idx + i] = kdf_result;
                    }
                }
            });
            
            handles.push(handle);
        // Wait for all threads to complete
        for handle in handles {
            handle.join().map_err(|_| KdfError::CryptographicError("Thread join failed".to_string()))?;
        let final_results = results.lock().unwrap().clone();
        Ok(final_results)
    /// periodt Parallel PBKDF2 implementation
    pub fn parallel_pbkdf2(
    ) -> KdfResult<Vec<u8>> {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;
        
        type HmacSha256 = Hmac<Sha256>;
        
        if iterations == 0 {
            return Err(KdfError::InvalidInput("Iterations must be greater than 0".to_string()));
        let hlen = 32; // SHA-256 output length
        let l = (output_length + hlen - 1) / hlen; // Number of blocks
        
        // Distribute blocks among threads
        let blocks_per_thread = (l + self.config.thread_count - 1) / self.config.thread_count;
        let results = Arc::new(Mutex::new(vec![vec![0u8; hlen]; l]));
        let mut handles = Vec::new();
        
        for thread_id in 0..self.config.thread_count {
            let start_block = thread_id * blocks_per_thread;
            let end_block = ((thread_id + 1) * blocks_per_thread).min(l);
            
            if start_block >= l {
                break;
            let password = password.to_vec();
            let salt = salt.to_vec();
            let results_clone = Arc::clone(&results);
            
            let handle = thread::spawn(move || {
                for block_idx in start_block..end_block {
                    let i = (block_idx + 1) as u32;
                    
                    // U1 = PRF(password, salt || INT(i))
                    let mut mac = HmacSha256::new_from_slice(&password).unwrap();
                    mac.update(&salt);
                    mac.update(&i.to_be_bytes());
                    let u1 = mac.finalize().into_bytes();
                    
                    let mut u = u1.to_vec();
                    let mut result = u.clone();
                    
                    // U2, U3, ... iterations
                    for _ in 1..iterations {
                        let mut mac = HmacSha256::new_from_slice(&password).unwrap();
                        mac.update(&u);
                        u = mac.finalize().into_bytes().to_vec();
                        
                        // XOR with result
                        for (r, &ui) in result.iter_mut().zip(u.iter()) {
                            *r ^= ui;
                        }
                    }
                    
                    // Store result
                    let mut results_guard = results_clone.lock().unwrap();
                    results_guard[block_idx] = result;
                }
            });
            
            handles.push(handle);
        // Wait for all threads
        for handle in handles {
            handle.join().map_err(|_| KdfError::CryptographicError("Thread join failed".to_string()))?;
        // Combine results
        let block_results = results.lock().unwrap();
        let mut output = Vec::new();
        for block in block_results.iter() {
            output.extend_from_slice(block);
        }
        output.truncate(output_length);
        
        Ok(output)
    // Helper methods
    
    fn create_work_units(
    ) -> KdfResult<Vec<ParallelWorkUnit>> {
        let total_data_size = password.len() + salt.len();
        let chunk_count = (total_data_size + self.config.chunk_size - 1) / self.config.chunk_size;
        let actual_chunk_count = chunk_count.max(self.config.thread_count);
        
        let mut work_units = Vec::new();
        let chunk_size = (output_length + actual_chunk_count - 1) / actual_chunk_count;
        
        for i in 0..actual_chunk_count {
            let start_offset = i * chunk_size;
            let end_offset = ((i + 1) * chunk_size).min(output_length);
            
            if start_offset >= output_length {
                break;
            let mut data = Vec::new();
            data.extend_from_slice(password);
            data.extend_from_slice(salt);
            data.extend_from_slice(&(i as u32).to_le_bytes());
            
            work_units.push(ParallelWorkUnit {
            });
        Ok(work_units)
    fn process_work_units(&self, work_units: Vec<ParallelWorkUnit>) -> KdfResult<Vec<ParallelResult>> {
        let results = Arc::new(Mutex::new(Vec::new()));
        let mut handles = Vec::new();
        
        let work_units = Arc::new(work_units);
        let units_per_thread = (work_units.len() + self.config.thread_count - 1) / self.config.thread_count;
        
        for thread_id in 0..self.config.thread_count {
            let start_idx = thread_id * units_per_thread;
            let end_idx = ((thread_id + 1) * units_per_thread).min(work_units.len());
            
            if start_idx >= work_units.len() {
                break;
            let work_units_clone = Arc::clone(&work_units);
            let results_clone = Arc::clone(&results);
            
            let handle = thread::spawn(move || {
                for idx in start_idx..end_idx {
                    let unit = &work_units_clone[idx];
                    let start_time = Instant::now();
                    
                    // Process this work unit
                    let mut hasher = Sha256::new();
                    hasher.update(&unit.data);
                    
                    let mut current_hash = hasher.finalize().to_vec();
                    
                    // Apply iterations
                    for _ in 0..unit.iterations {
                        let mut hasher = Sha256::new();
                        hasher.update(&current_hash);
                        current_hash = hasher.finalize().to_vec();
                    let processing_time = start_time.elapsed().as_millis() as u64;
                    
                    let result = ParallelResult {
                    
                    results_clone.lock().unwrap().push(result);
                }
            });
            
            handles.push(handle);
        // Wait for all threads
        for handle in handles {
            handle.join().map_err(|_| KdfError::CryptographicError("Thread join failed".to_string()))?;
        let mut final_results = results.lock().unwrap().clone();
        final_results.sort_by_key(|r| r.chunk_id);
        
        Ok(final_results)
    fn combine_results(&self, results: Vec<ParallelResult>, output_length: usize) -> KdfResult<Vec<u8>> {
        let mut combined = Vec::new();
        
        for result in results {
            combined.extend_from_slice(&result.result);
        // Use SHA-256 to mix all results
        let mut hasher = Sha256::new();
        hasher.update(&combined);
        hasher.update(b"parallel_combine");
        hasher.update(&(output_length as u32).to_le_bytes());
        
        let mut final_result = hasher.finalize().to_vec();
        
        // Expand to desired length if needed
        while final_result.len() < output_length {
            let mut hasher = Sha256::new();
            hasher.update(&final_result);
            hasher.update(b"expand");
            let additional = hasher.finalize();
            final_result.extend_from_slice(&additional);
        final_result.truncate(output_length);
        Ok(final_result)
    }
}

/// fr fr Parallel processing utilities
pub struct ParallelUtils;

impl ParallelUtils {
    /// bestie Get optimal thread count for system
    pub fn optimal_thread_count() -> usize {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
    /// vibes Estimate parallel speedup
    pub fn estimate_speedup(thread_count: usize, iterations: u32) -> f64 {
        if thread_count <= 1 {
            return 1.0;
        // Amdahl's law approximation
        let parallel_fraction = 0.9; // Assume 90% of work is parallelizable
        let overhead = 0.1 * (thread_count as f64).sqrt(); // Thread overhead
        
        let speedup = 1.0 / (1.0 - parallel_fraction + parallel_fraction / (thread_count as f64));
        (speedup * (1.0 - overhead)).max(1.0)
    /// facts Calculate memory requirements
    pub fn calculate_memory_requirements(config: &ParallelConfig, data_size: usize) -> usize {
        let base_memory = config.thread_count * config.max_memory_per_thread;
        let data_memory = data_size * config.thread_count * 2; // Input + output buffers
        base_memory + data_memory
    }
}

/// fr fr Public API functions for CURSED integration

/// slay Parallel KDF computation
pub fn parallel_kdf(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 3 {
        return Err(CursedError::Runtime("parallel_kdf requires password, salt, and iterations arguments".to_string()));
    let password = match &args[0] {
    
    let salt = match &args[1] {
    
    let iterations = match &args[2] {
    
    let output_length = if args.len() > 3 {
        match &args[3] {
        }
    } else {
        32
    
    let config = ParallelConfig::new();
    let processor = ParallelProcessor::new(config)
        .map_err(|e| CursedError::Runtime(format!("Parallel processor creation failed: {}", e)))?;
    
    let result = processor.parallel_kdf(password, salt, iterations, output_length)
        .map_err(|e| CursedError::Runtime(format!("Parallel KDF failed: {}", e)))?;
    
    Ok(Value::String(hex::encode(result)))
