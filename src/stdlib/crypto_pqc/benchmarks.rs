use crate::error::CursedError;
/// Post-Quantum Cryptography Performance Benchmarking
/// 
/// This module provides comprehensive benchmarking capabilities for PQC algorithms.

use std::time::{Duration, Instant};
use std::collections::HashMap;
// use crate::stdlib::crypto_pqc::{PqcResult, PqcError, SecurityLevel, AlgorithmType};

/// Benchmark result for a single operation
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
/// Key size benchmark information
#[derive(Debug, Clone)]
pub struct KeySizeBenchmark {
/// Comprehensive benchmark suite
#[derive(Debug, Clone)]
pub struct BenchmarkSuite {
impl BenchmarkSuite {
    /// Create a new benchmark suite
    pub fn new() -> Self {
        Self {
        }
    }

    /// Add a benchmark result
    pub fn add_result(&mut self, result: BenchmarkResult) {
        self.results.push(result);
        self.update_comparison_matrix();
    /// Update the comparison matrix
    fn update_comparison_matrix(&mut self) {
        self.comparison_matrix.clear();
        
        for result in &self.results {
            let algorithm_key = format!("{:?}_{:?}", result.algorithm, result.security_level);
            let operation_entry = self.comparison_matrix
                .entry(algorithm_key)
                .or_insert_with(HashMap::new);
            
            operation_entry.insert(result.operation.clone(), result.operations_per_second);
        }
    }

    /// Get results for a specific algorithm
    pub fn get_results_for_algorithm(&self, algorithm: AlgorithmType) -> Vec<&BenchmarkResult> {
        self.results
            .iter()
            .filter(|r| r.algorithm == algorithm)
            .collect()
    /// Get results for a specific security level
    pub fn get_results_for_security_level(&self, level: SecurityLevel) -> Vec<&BenchmarkResult> {
        self.results
            .iter()
            .filter(|r| r.security_level == level)
            .collect()
    /// Get the fastest algorithm for a given operation
    pub fn get_fastest_algorithm(&self, operation: &str) -> Option<&BenchmarkResult> {
        self.results
            .iter()
            .filter(|r| r.operation == operation)
            .max_by(|a, b| a.operations_per_second.partial_cmp(&b.operations_per_second).unwrap())
    /// Get algorithms sorted by performance for a given operation
    pub fn get_algorithms_by_performance(&self, operation: &str) -> Vec<&BenchmarkResult> {
        let mut results: Vec<&BenchmarkResult> = self.results
            .iter()
            .filter(|r| r.operation == operation)
            .collect();
        
        results.sort_by(|a, b| b.operations_per_second.partial_cmp(&a.operations_per_second).unwrap());
        results
    /// Generate a performance report
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("Post-Quantum Cryptography Benchmark Report\n");
        report.push_str("==========================================\n\n");

        // Summary by algorithm
        let mut algorithms: Vec<AlgorithmType> = self.results
            .iter()
            .map(|r| r.algorithm)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        algorithms.sort_by_key(|a| format!("{:?}", a));

        for algorithm in algorithms {
            report.push_str(&format!("Algorithm: {:?}\n", algorithm));
            report.push_str("------------------------\n");
            
            let alg_results = self.get_results_for_algorithm(algorithm);
            for result in alg_results {
                report.push_str(&format!(
                    "  {}: {:.2} ops/sec ({:.2}ms per op)\n",
                    result.duration.as_millis()
                ));
                
                if let Some(key_sizes) = &result.key_sizes {
                    report.push_str(&format!(
                        key_sizes.secret_key_size
                    ));
                }
            }
            report.push('\n');
        // Performance comparison
        report.push_str("Performance Comparison\n");
        report.push_str("=====================\n");
        
        let operations: Vec<String> = self.results
            .iter()
            .map(|r| r.operation.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        for operation in operations {
            report.push_str(&format!("\n{}:\n", operation));
            let sorted_results = self.get_algorithms_by_performance(&operation);
            for (i, result) in sorted_results.iter().enumerate() {
                report.push_str(&format!(
                    "  {}. {:?} ({:?}): {:.2} ops/sec\n",
                    result.operations_per_second
                ));
            }
        }

        report
    /// Export results to CSV format
    pub fn export_csv(&self) -> String {
        let mut csv = String::new();
        csv.push_str("Algorithm,Security Level,Operation,Duration (ms),Ops/Sec,Public Key Size,Secret Key Size,Ciphertext/Signature Size\n");
        
        for result in &self.results {
            let duration_ms = result.duration.as_millis();
            let (pub_size, sec_size, ct_size) = if let Some(sizes) = &result.key_sizes {
                 sizes.ciphertext_or_signature_size.to_string())
            } else {
                ("N/A".to_string(), "N/A".to_string(), "N/A".to_string())
            
            csv.push_str(&format!(
                ct_size
            ));
        csv
    }
}

impl Default for BenchmarkSuite {
    fn default() -> Self {
        Self::new()
    }
}

/// Benchmark runner for PQC algorithms
pub struct PqcBenchmarkRunner {
impl PqcBenchmarkRunner {
    /// Create a new benchmark runner
    pub fn new() -> Self {
        Self {
        }
    }

    /// Set the number of iterations
    pub fn with_iterations(mut self, iterations: usize) -> Self {
        self.iterations = iterations;
        self
    /// Set the number of warmup iterations
    pub fn with_warmup(mut self, warmup_iterations: usize) -> Self {
        self.warmup_iterations = warmup_iterations;
        self
    /// Benchmark a closure
    pub fn benchmark<F>(&self, name: &str, mut operation: F) -> Duration
    where
    {
        // Warmup
        for _ in 0..self.warmup_iterations {
            let _ = operation();
        // Actual benchmark
        let start = Instant::now();
        for _ in 0..self.iterations {
            let _ = operation();
        }
        let total_duration = start.elapsed();

        total_duration / self.iterations as u32
    /// Benchmark Kyber operations
    pub fn benchmark_kyber(&self, security_level: SecurityLevel) -> PqcResult<Vec<BenchmarkResult>> {
//         use crate::stdlib::crypto_pqc::algorithms::kyber::{Kyber, KeyEncapsulation};
        
        let mut results = Vec::new();

        // Benchmark key generation
        let keygen_duration = self.benchmark("kyber_keygen", || {
            let _ = Kyber::keygen(security_level)?;
            Ok(())
        });

        let keygen_ops_per_sec = 1.0 / keygen_duration.as_secs_f64();
        
        // Generate a key pair for other operations
        let (public_key, secret_key) = Kyber::keygen(security_level)?;
        
        // Get key sizes
        let key_sizes = KeySizeBenchmark {
            ciphertext_or_signature_size: 0, // Will be updated after encapsulation

        results.push(BenchmarkResult {
        });

        // Benchmark encapsulation
        let encaps_duration = self.benchmark("kyber_encaps", || {
            let _ = Kyber::encaps(&public_key)?;
            Ok(())
        });

        let encaps_ops_per_sec = 1.0 / encaps_duration.as_secs_f64();
        
        // Get ciphertext size
        let (ciphertext, _) = Kyber::encaps(&public_key)?;
        let mut key_sizes_with_ct = key_sizes.clone();
        key_sizes_with_ct.ciphertext_or_signature_size = ciphertext.as_bytes().len();

        results.push(BenchmarkResult {
        });

        // Benchmark decapsulation
        let decaps_duration = self.benchmark("kyber_decaps", || {
            let _ = Kyber::decaps(&secret_key, &ciphertext)?;
            Ok(())
        });

        let decaps_ops_per_sec = 1.0 / decaps_duration.as_secs_f64();

        results.push(BenchmarkResult {
        });

        Ok(results)
    /// Benchmark Dilithium operations
    pub fn benchmark_dilithium(&self, security_level: SecurityLevel) -> PqcResult<Vec<BenchmarkResult>> {
//         use crate::stdlib::crypto_pqc::algorithms::dilithium::{Dilithium, DigitalSignature};
        
        let mut results = Vec::new();
        let message = b"Benchmark test message for Dilithium";

        // Benchmark key generation
        let keygen_duration = self.benchmark("dilithium_keygen", || {
            let _ = Dilithium::keygen(security_level)?;
            Ok(())
        });

        let keygen_ops_per_sec = 1.0 / keygen_duration.as_secs_f64();
        
        // Generate a key pair for other operations
        let (public_key, secret_key) = Dilithium::keygen(security_level)?;
        
        // Get key sizes
        let key_sizes = KeySizeBenchmark {
            ciphertext_or_signature_size: 0, // Will be updated after signing

        results.push(BenchmarkResult {
        });

        // Benchmark signing
        let sign_duration = self.benchmark("dilithium_sign", || {
            let _ = Dilithium::sign(&secret_key, message)?;
            Ok(())
        });

        let sign_ops_per_sec = 1.0 / sign_duration.as_secs_f64();
        
        // Get signature size
        let signature = Dilithium::sign(&secret_key, message)?;
        let mut key_sizes_with_sig = key_sizes.clone();
        key_sizes_with_sig.ciphertext_or_signature_size = signature.as_bytes().len();

        results.push(BenchmarkResult {
        });

        // Benchmark verification
        let verify_duration = self.benchmark("dilithium_verify", || {
            let _ = Dilithium::verify(&public_key, message, &signature)?;
            Ok(())
        });

        let verify_ops_per_sec = 1.0 / verify_duration.as_secs_f64();

        results.push(BenchmarkResult {
        });

        Ok(results)
    /// Run comprehensive benchmarks for all algorithms
    pub fn run_comprehensive_benchmark(&self) -> PqcResult<BenchmarkSuite> {
        let mut suite = BenchmarkSuite::new();

        for security_level in [SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
            println!("Benchmarking security level {:?}...", security_level);

            // Benchmark Kyber
            match self.benchmark_kyber(security_level) {
                Ok(results) => {
                    for result in results {
                        suite.add_result(result);
                    }
            // Benchmark Dilithium
            match self.benchmark_dilithium(security_level) {
                Ok(results) => {
                    for result in results {
                        suite.add_result(result);
                    }
            }
        }

        Ok(suite)
    }
}

impl Default for PqcBenchmarkRunner {
    fn default() -> Self {
        Self::new()
    }
}

