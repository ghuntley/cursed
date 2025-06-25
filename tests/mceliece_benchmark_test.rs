//! McEliece Performance Benchmark Tests
//!
//! This module provides comprehensive benchmarking for the Classic McEliece implementation
//! including key generation, encapsulation, decapsulation performance, and memory usage analysis.

use std::time::{Duration, Instant};
use cursed::stdlib::crypto_pqc::{SecurityLevel, AlgorithmType};
use cursed::stdlib::crypto_pqc::algorithms::mceliece::{
    ClassicMcEliece, McElieceParams
};
use cursed::stdlib::crypto_pqc::algorithms::{KeyEncapsulation, ParameterSet};

/// Benchmark results for McEliece operations
#[derive(Debug)]
pub struct BenchmarkResults {
    pub parameter_set: McElieceParams,
    pub keygen_avg_ms: f64,
    pub keygen_std_ms: f64,
    pub encaps_avg_ms: f64,
    pub encaps_std_ms: f64,
    pub decaps_avg_ms: f64,
    pub decaps_std_ms: f64,
    pub encaps_per_sec: f64,
    pub decaps_per_sec: f64,
    pub memory_usage_mb: f64,
}

impl BenchmarkResults {
    fn new(params: McElieceParams) -> Self {
        Self {
            parameter_set: params,
            keygen_avg_ms: 0.0,
            keygen_std_ms: 0.0,
            encaps_avg_ms: 0.0,
            encaps_std_ms: 0.0,
            decaps_avg_ms: 0.0,
            decaps_std_ms: 0.0,
            encaps_per_sec: 0.0,
            decaps_per_sec: 0.0,
            memory_usage_mb: 0.0,
        }
    }
}

/// Calculate mean and standard deviation
fn calculate_stats(times: &[f64]) -> (f64, f64) {
    let mean = times.iter().sum::<f64>() / times.len() as f64;
    let variance = times.iter()
        .map(|&x| (x - mean).powi(2))
        .sum::<f64>() / times.len() as f64;
    let std_dev = variance.sqrt();
    (mean, std_dev)
}

/// Estimate memory usage for key materials
fn estimate_memory_usage(params: McElieceParams) -> f64 {
    let pub_key_size = params.public_key_size();
    let sec_key_size = params.secret_key_size();
    let ciphertext_size = params.additional_sizes()
        .iter()
        .find(|(name, _)| *name == "ciphertext")
        .map(|(_, size)| *size)
        .unwrap_or(0);
    
    // Estimate total memory including internal structures
    let total_bytes = pub_key_size + sec_key_size + ciphertext_size + 32; // +32 for shared secret
    let estimated_overhead = (total_bytes as f64) * 1.5; // 50% overhead estimate
    estimated_overhead / (1024.0 * 1024.0) // Convert to MB
}

/// Run comprehensive benchmark for a parameter set
fn benchmark_parameter_set(params: McElieceParams, iterations: usize) -> BenchmarkResults {
    let mut results = BenchmarkResults::new(params);
    
    println!("Benchmarking {} with {} iterations...", params, iterations);
    
    // Benchmark key generation
    let mut keygen_times = Vec::with_capacity(iterations);
    let mut key_pairs = Vec::with_capacity(iterations);
    
    for i in 0..iterations {
        let start = Instant::now();
        let keypair = ClassicMcEliece::keygen_with_params(params).unwrap();
        let duration = start.elapsed();
        
        keygen_times.push(duration.as_secs_f64() * 1000.0);
        key_pairs.push(keypair);
        
        if (i + 1) % (iterations / 10).max(1) == 0 {
            println!("  Key generation: {}/{} completed", i + 1, iterations);
        }
    }
    
    let (keygen_avg, keygen_std) = calculate_stats(&keygen_times);
    results.keygen_avg_ms = keygen_avg;
    results.keygen_std_ms = keygen_std;
    
    // Benchmark encapsulation
    let mut encaps_times = Vec::with_capacity(iterations);
    let mut ciphertexts_and_secrets = Vec::with_capacity(iterations);
    
    for i in 0..iterations {
        let (public_key, _) = &key_pairs[i];
        
        let start = Instant::now();
        let encaps_result = ClassicMcEliece::encaps(public_key).unwrap();
        let duration = start.elapsed();
        
        encaps_times.push(duration.as_secs_f64() * 1000.0);
        ciphertexts_and_secrets.push(encaps_result);
        
        if (i + 1) % (iterations / 10).max(1) == 0 {
            println!("  Encapsulation: {}/{} completed", i + 1, iterations);
        }
    }
    
    let (encaps_avg, encaps_std) = calculate_stats(&encaps_times);
    results.encaps_avg_ms = encaps_avg;
    results.encaps_std_ms = encaps_std;
    results.encaps_per_sec = 1000.0 / encaps_avg;
    
    // Benchmark decapsulation
    let mut decaps_times = Vec::with_capacity(iterations);
    
    for i in 0..iterations {
        let (_, secret_key) = &key_pairs[i];
        let (ciphertext, expected_secret) = &ciphertexts_and_secrets[i];
        
        let start = Instant::now();
        let decaps_result = ClassicMcEliece::decaps(secret_key, ciphertext).unwrap();
        let duration = start.elapsed();
        
        decaps_times.push(duration.as_secs_f64() * 1000.0);
        
        // Verify correctness
        assert_eq!(
            decaps_result.as_bytes(),
            expected_secret.as_bytes(),
            "Decapsulation verification failed at iteration {}", i
        );
        
        if (i + 1) % (iterations / 10).max(1) == 0 {
            println!("  Decapsulation: {}/{} completed", i + 1, iterations);
        }
    }
    
    let (decaps_avg, decaps_std) = calculate_stats(&decaps_times);
    results.decaps_avg_ms = decaps_avg;
    results.decaps_std_ms = decaps_std;
    results.decaps_per_sec = 1000.0 / decaps_avg;
    
    // Estimate memory usage
    results.memory_usage_mb = estimate_memory_usage(params);
    
    results
}

/// Print benchmark results in a formatted table
fn print_results(results: &[BenchmarkResults]) {
    println!("\n{}", "=".repeat(120));
    println!("CLASSIC MCELIECE PERFORMANCE BENCHMARK RESULTS");
    println!("{}", "=".repeat(120));
    
    println!(
        "{:<20} {:<12} {:<12} {:<12} {:<12} {:<12} {:<12} {:<12}",
        "Parameter Set", "KeyGen(ms)", "Encaps(ms)", "Decaps(ms)",
        "Enc/sec", "Dec/sec", "Memory(MB)", "Security"
    );
    println!("{}", "-".repeat(120));
    
    for result in results {
        println!(
            "{:<20} {:>8.2}±{:<2.1} {:>8.3}±{:<2.2} {:>8.2}±{:<2.1} {:>9.0} {:>9.0} {:>9.1} {:<12}",
            format!("{}", result.parameter_set),
            result.keygen_avg_ms, result.keygen_std_ms,
            result.encaps_avg_ms, result.encaps_std_ms,
            result.decaps_avg_ms, result.decaps_std_ms,
            result.encaps_per_sec,
            result.decaps_per_sec,
            result.memory_usage_mb,
            format!("Level {:?}", result.parameter_set.security_level() as u8 + 1)
        );
    }
    
    println!("{}", "=".repeat(120));
}

/// Print key size information
fn print_key_sizes(params: &[McElieceParams]) {
    println!("\nKEY SIZE ANALYSIS");
    println!("{}", "=".repeat(80));
    println!(
        "{:<20} {:<15} {:<15} {:<15} {:<10}",
        "Parameter Set", "Public Key", "Secret Key", "Ciphertext", "Shared Secret"
    );
    println!("{}", "-".repeat(80));
    
    for param in params {
        let additional = param.additional_sizes();
        let ciphertext_size = additional
            .iter()
            .find(|(name, _)| *name == "ciphertext")
            .map(|(_, size)| *size)
            .unwrap_or(0);
        
        println!(
            "{:<20} {:>12} B {:>12} B {:>12} B {:>8} B",
            format!("{}", param),
            param.public_key_size(),
            param.secret_key_size(),
            ciphertext_size,
            32 // Shared secret is always 32 bytes
        );
    }
    println!("{}", "=".repeat(80));
}

/// Main benchmark test for basic performance
#[test]
fn test_mceliece_performance_basic() {
    let test_params = vec![
        McElieceParams::McEliece348864,
        McElieceParams::McEliece460896,
    ];
    
    let iterations = 5; // Small number for basic testing
    let mut all_results = Vec::new();
    
    for params in &test_params {
        let results = benchmark_parameter_set(*params, iterations);
        all_results.push(results);
    }
    
    print_results(&all_results);
    print_key_sizes(&test_params);
    
    // Basic performance assertions
    for result in &all_results {
        assert!(result.keygen_avg_ms > 0.0, "Key generation time should be positive");
        assert!(result.encaps_avg_ms > 0.0, "Encapsulation time should be positive");
        assert!(result.decaps_avg_ms > 0.0, "Decapsulation time should be positive");
        assert!(result.encaps_per_sec > 0.0, "Encapsulation throughput should be positive");
        assert!(result.decaps_per_sec > 0.0, "Decapsulation throughput should be positive");
        
        // McEliece should have fast encaps/decaps relative to keygen
        assert!(
            result.encaps_avg_ms < result.keygen_avg_ms / 10.0,
            "Encapsulation should be much faster than key generation"
        );
        assert!(
            result.decaps_avg_ms < result.keygen_avg_ms / 2.0,
            "Decapsulation should be faster than key generation"
        );
    }
}

/// Comprehensive benchmark test (run with --ignored)
#[test]
#[ignore]
fn test_mceliece_performance_comprehensive() {
    let test_params = vec![
        McElieceParams::McEliece348864,
        McElieceParams::McEliece460896,
        McElieceParams::McEliece6688128,
        McElieceParams::McEliece6960119,
        McElieceParams::McEliece8192128,
    ];
    
    let iterations = 50; // More iterations for accurate measurements
    let mut all_results = Vec::new();
    
    println!("Running comprehensive McEliece benchmark...");
    println!("This may take several minutes for larger parameter sets.");
    
    for params in &test_params {
        let results = benchmark_parameter_set(*params, iterations);
        all_results.push(results);
    }
    
    print_results(&all_results);
    print_key_sizes(&test_params);
    
    // Performance regression checks
    for result in &all_results {
        match result.parameter_set {
            McElieceParams::McEliece348864 => {
                assert!(result.keygen_avg_ms < 100.0, "Level 1 keygen too slow");
                assert!(result.encaps_avg_ms < 1.0, "Level 1 encaps too slow");
                assert!(result.decaps_avg_ms < 5.0, "Level 1 decaps too slow");
            },
            McElieceParams::McEliece460896 => {
                assert!(result.keygen_avg_ms < 200.0, "Level 3 keygen too slow");
                assert!(result.encaps_avg_ms < 2.0, "Level 3 encaps too slow");
                assert!(result.decaps_avg_ms < 10.0, "Level 3 decaps too slow");
            },
            _ => {
                // Level 5 parameters can be slower
                assert!(result.keygen_avg_ms < 1000.0, "Level 5 keygen extremely slow");
                assert!(result.encaps_avg_ms < 5.0, "Level 5 encaps too slow");
                assert!(result.decaps_avg_ms < 20.0, "Level 5 decaps too slow");
            }
        }
    }
}

/// Memory usage analysis test
#[test]
fn test_memory_usage_analysis() {
    let test_params = vec![
        McElieceParams::McEliece348864,
        McElieceParams::McEliece460896,
        McElieceParams::McEliece6688128,
    ];
    
    println!("\nMEMORY USAGE ANALYSIS");
    println!("{}", "=".repeat(100));
    
    for params in &test_params {
        let pub_key_size = params.public_key_size();
        let sec_key_size = params.secret_key_size();
        let additional = params.additional_sizes();
        
        println!("Parameter Set: {}", params);
        println!("  Public Key Size:  {:>8} bytes ({:>6.1} KB)", pub_key_size, pub_key_size as f64 / 1024.0);
        println!("  Secret Key Size:  {:>8} bytes ({:>6.1} KB)", sec_key_size, sec_key_size as f64 / 1024.0);
        
        for (name, size) in &additional {
            println!("  {:>15}:  {:>8} bytes ({:>6.1} KB)", 
                    format!("{} Size", name), size, *size as f64 / 1024.0);
        }
        
        let total_size = pub_key_size + sec_key_size + 
            additional.iter().map(|(_, size)| size).sum::<usize>();
        println!("  Total Key Material: {:>8} bytes ({:>6.1} KB)", 
                total_size, total_size as f64 / 1024.0);
        println!();
    }
    
    // Memory efficiency checks
    for params in &test_params {
        let pub_key_mb = params.public_key_size() as f64 / (1024.0 * 1024.0);
        let sec_key_mb = params.secret_key_size() as f64 / (1024.0 * 1024.0);
        
        // McEliece has large public keys but small secret keys
        assert!(sec_key_mb < 1.0, "Secret key should be less than 1MB");
        
        match params {
            McElieceParams::McEliece348864 => {
                assert!(pub_key_mb < 0.5, "Level 1 public key should be < 0.5MB");
            },
            McElieceParams::McEliece460896 => {
                assert!(pub_key_mb < 1.0, "Level 3 public key should be < 1MB");
            },
            _ => {
                assert!(pub_key_mb < 2.0, "Level 5 public key should be < 2MB");
            }
        }
    }
}

/// Throughput scaling test
#[test]
fn test_throughput_scaling() {
    let params = McElieceParams::McEliece348864;
    let (public_key, secret_key) = ClassicMcEliece::keygen_with_params(params).unwrap();
    
    let test_sizes = vec![1, 10, 50, 100];
    
    println!("\nTHROUGHPUT SCALING ANALYSIS");
    println!("{}", "=".repeat(60));
    println!("{:<15} {:<15} {:<15} {:<15}", "Operations", "Encaps/sec", "Decaps/sec", "Total Time(s)");
    println!("{}", "-".repeat(60));
    
    for &size in &test_sizes {
        // Benchmark encapsulation throughput
        let start = Instant::now();
        let mut ciphertexts = Vec::with_capacity(size);
        
        for _ in 0..size {
            let (ciphertext, _) = ClassicMcEliece::encaps(&public_key).unwrap();
            ciphertexts.push(ciphertext);
        }
        
        let encaps_duration = start.elapsed();
        let encaps_per_sec = size as f64 / encaps_duration.as_secs_f64();
        
        // Benchmark decapsulation throughput
        let start = Instant::now();
        
        for ciphertext in &ciphertexts {
            let _ = ClassicMcEliece::decaps(&secret_key, ciphertext).unwrap();
        }
        
        let decaps_duration = start.elapsed();
        let decaps_per_sec = size as f64 / decaps_duration.as_secs_f64();
        let total_time = encaps_duration.as_secs_f64() + decaps_duration.as_secs_f64();
        
        println!("{:<15} {:<15.1} {:<15.1} {:<15.3}", 
                size, encaps_per_sec, decaps_per_sec, total_time);
    }
    
    println!("{}", "=".repeat(60));
}

/// Stress test for sustained operations
#[test]
#[ignore]
fn test_sustained_performance() {
    let params = McElieceParams::McEliece348864;
    let (public_key, secret_key) = ClassicMcEliece::keygen_with_params(params).unwrap();
    
    let duration = Duration::from_secs(30); // Run for 30 seconds
    let start_time = Instant::now();
    let mut operation_count = 0;
    let mut total_encaps_time = Duration::new(0, 0);
    let mut total_decaps_time = Duration::new(0, 0);
    
    println!("Running sustained performance test for 30 seconds...");
    
    while start_time.elapsed() < duration {
        // Encapsulation
        let encaps_start = Instant::now();
        let (ciphertext, _) = ClassicMcEliece::encaps(&public_key).unwrap();
        total_encaps_time += encaps_start.elapsed();
        
        // Decapsulation
        let decaps_start = Instant::now();
        let _ = ClassicMcEliece::decaps(&secret_key, &ciphertext).unwrap();
        total_decaps_time += decaps_start.elapsed();
        
        operation_count += 1;
        
        if operation_count % 100 == 0 {
            println!("  Completed {} operation pairs", operation_count);
        }
    }
    
    let actual_duration = start_time.elapsed().as_secs_f64();
    let ops_per_sec = operation_count as f64 / actual_duration;
    let avg_encaps_ms = total_encaps_time.as_secs_f64() * 1000.0 / operation_count as f64;
    let avg_decaps_ms = total_decaps_time.as_secs_f64() * 1000.0 / operation_count as f64;
    
    println!("\nSUSTAINED PERFORMANCE RESULTS");
    println!("{}", "=".repeat(50));
    println!("Total Operations: {}", operation_count);
    println!("Duration: {:.1} seconds", actual_duration);
    println!("Operations per second: {:.1}", ops_per_sec);
    println!("Average Encapsulation: {:.3} ms", avg_encaps_ms);
    println!("Average Decapsulation: {:.3} ms", avg_decaps_ms);
    println!("{}", "=".repeat(50));
    
    // Performance stability checks
    assert!(operation_count > 100, "Should complete at least 100 operations in 30 seconds");
    assert!(ops_per_sec > 10.0, "Should maintain at least 10 ops/sec");
    assert!(avg_encaps_ms < 10.0, "Encapsulation should remain fast under sustained load");
    assert!(avg_decaps_ms < 50.0, "Decapsulation should remain reasonable under sustained load");
}
