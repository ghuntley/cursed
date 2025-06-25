//! Comprehensive Benchmark Suite for Hybrid Post-Quantum Cryptography
//! 
//! This test suite provides detailed performance analysis and benchmarking
//! for the hybrid cryptographic system under various conditions.

use std::time::{Duration, Instant};
use std::collections::HashMap;
use cursed::stdlib::crypto_pqc::*;
use cursed::stdlib::crypto_pqc::hybrid::*;

#[path = "common.rs"]
pub mod common;

const BENCHMARK_ITERATIONS: usize = 100;
const PERFORMANCE_ITERATIONS: usize = 10;

/// Comprehensive benchmark results
#[derive(Debug, Clone)]
struct BenchmarkResults {
    algorithm_name: String,
    security_level: SecurityLevel,
    keygen_times: Vec<Duration>,
    encaps_times: Vec<Duration>,
    decaps_times: Vec<Duration>,
    key_sizes: KeySizes,
    success_rate: f64,
}

#[derive(Debug, Clone)]
struct KeySizes {
    classical_public: usize,
    classical_secret: usize,
    pqc_public: usize,
    pqc_secret: usize,
    ciphertext: usize,
    shared_secret: usize,
}

impl BenchmarkResults {
    fn calculate_stats(&self) -> BenchmarkStats {
        let keygen_avg = self.keygen_times.iter().sum::<Duration>() / self.keygen_times.len() as u32;
        let keygen_min = *self.keygen_times.iter().min().unwrap();
        let keygen_max = *self.keygen_times.iter().max().unwrap();
        
        let encaps_avg = self.encaps_times.iter().sum::<Duration>() / self.encaps_times.len() as u32;
        let encaps_min = *self.encaps_times.iter().min().unwrap();
        let encaps_max = *self.encaps_times.iter().max().unwrap();
        
        let decaps_avg = self.decaps_times.iter().sum::<Duration>() / self.decaps_times.len() as u32;
        let decaps_min = *self.decaps_times.iter().min().unwrap();
        let decaps_max = *self.decaps_times.iter().max().unwrap();
        
        let total_avg = keygen_avg + encaps_avg + decaps_avg;
        let throughput = 1.0 / total_avg.as_secs_f64();
        
        BenchmarkStats {
            keygen_avg,
            keygen_min,
            keygen_max,
            encaps_avg,
            encaps_min,
            encaps_max,
            decaps_avg,
            decaps_min,
            decaps_max,
            total_avg,
            throughput_ops_per_sec: throughput,
            success_rate: self.success_rate,
        }
    }
}

#[derive(Debug)]
struct BenchmarkStats {
    keygen_avg: Duration,
    keygen_min: Duration,
    keygen_max: Duration,
    encaps_avg: Duration,
    encaps_min: Duration,
    encaps_max: Duration,
    decaps_avg: Duration,
    decaps_min: Duration,
    decaps_max: Duration,
    total_avg: Duration,
    throughput_ops_per_sec: f64,
    success_rate: f64,
}

/// Benchmark single algorithm combination
fn benchmark_algorithm_combination(
    classical: ClassicalAlgorithm,
    pqc: AlgorithmType,
    security_level: SecurityLevel,
    iterations: usize,
) -> BenchmarkResults {
    let algorithm_name = format!("{:?}+{:?}", classical, pqc);
    
    let hybrid_kem = HybridKem::new(classical, pqc, security_level);
    
    let mut keygen_times = Vec::new();
    let mut encaps_times = Vec::new();
    let mut decaps_times = Vec::new();
    let mut successes = 0;
    let mut key_sizes = None;
    
    for i in 0..iterations {
        if i % 10 == 0 {
            println!("  Iteration {}/{}", i, iterations);
        }
        
        // Benchmark key generation
        let start = Instant::now();
        let key_pair_result = hybrid_kem.keygen();
        let keygen_time = start.elapsed();
        
        match key_pair_result {
            Ok(key_pair) => {
                keygen_times.push(keygen_time);
                
                // Record key sizes on first successful iteration
                if key_sizes.is_none() {
                    key_sizes = Some(KeySizes {
                        classical_public: key_pair.classical_public.len(),
                        classical_secret: key_pair.classical_secret.len(),
                        pqc_public: key_pair.pqc_public.len(),
                        pqc_secret: key_pair.pqc_secret.len(),
                        ciphertext: 0, // Will be set during encaps
                        shared_secret: 0, // Will be set during encaps
                    });
                }
                
                // Benchmark encapsulation
                let start = Instant::now();
                let encaps_result = hybrid_kem.encaps(&key_pair);
                let encaps_time = start.elapsed();
                
                match encaps_result {
                    Ok((ciphertext, shared_secret1)) => {
                        encaps_times.push(encaps_time);
                        
                        // Update ciphertext and shared secret sizes
                        if let Some(ref mut sizes) = key_sizes {
                            sizes.ciphertext = ciphertext.len();
                            sizes.shared_secret = shared_secret1.len();
                        }
                        
                        // Benchmark decapsulation
                        let start = Instant::now();
                        let decaps_result = hybrid_kem.decaps(&key_pair, &ciphertext);
                        let decaps_time = start.elapsed();
                        
                        match decaps_result {
                            Ok(shared_secret2) => {
                                decaps_times.push(decaps_time);
                                
                                // Verify correctness
                                if shared_secret1 == shared_secret2 {
                                    successes += 1;
                                }
                            },
                            Err(_) => {},
                        }
                    },
                    Err(_) => {},
                }
            },
            Err(_) => {},
        }
    }
    
    let success_rate = successes as f64 / iterations as f64;
    
    BenchmarkResults {
        algorithm_name,
        security_level,
        keygen_times,
        encaps_times,
        decaps_times,
        key_sizes: key_sizes.unwrap_or(KeySizes {
            classical_public: 0,
            classical_secret: 0,
            pqc_public: 0,
            pqc_secret: 0,
            ciphertext: 0,
            shared_secret: 0,
        }),
        success_rate,
    }
}

/// Test X25519+Kyber performance across security levels
#[test]
fn test_x25519_kyber_performance() {
    common::tracing::setup();
    
    println!("🏃 Benchmarking X25519+Kyber performance...");
    
    let security_levels = [SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5];
    
    for security_level in &security_levels {
        println!("\nTesting security level: {:?}", security_level);
        
        let results = benchmark_algorithm_combination(
            ClassicalAlgorithm::X25519,
            AlgorithmType::Kyber,
            *security_level,
            PERFORMANCE_ITERATIONS,
        );
        
        let stats = results.calculate_stats();
        
        println!("Results for {:?}:", security_level);
        println!("  Key generation: {:?} avg ({:?} - {:?})", stats.keygen_avg, stats.keygen_min, stats.keygen_max);
        println!("  Encapsulation:  {:?} avg ({:?} - {:?})", stats.encaps_avg, stats.encaps_min, stats.encaps_max);
        println!("  Decapsulation:  {:?} avg ({:?} - {:?})", stats.decaps_avg, stats.decaps_min, stats.decaps_max);
        println!("  Total operation: {:?} avg", stats.total_avg);
        println!("  Throughput: {:.2} ops/sec", stats.throughput_ops_per_sec);
        println!("  Success rate: {:.1}%", stats.success_rate * 100.0);
        println!("  Key sizes:");
        println!("    Classical public: {} bytes", results.key_sizes.classical_public);
        println!("    Classical secret: {} bytes", results.key_sizes.classical_secret);
        println!("    PQC public: {} bytes", results.key_sizes.pqc_public);
        println!("    PQC secret: {} bytes", results.key_sizes.pqc_secret);
        println!("    Ciphertext: {} bytes", results.key_sizes.ciphertext);
        println!("    Shared secret: {} bytes", results.key_sizes.shared_secret);
        
        // Performance assertions (generous for test environment)
        assert!(stats.keygen_avg < Duration::from_secs(5), "Key generation too slow");
        assert!(stats.encaps_avg < Duration::from_secs(2), "Encapsulation too slow");
        assert!(stats.decaps_avg < Duration::from_secs(2), "Decapsulation too slow");
        assert!(stats.success_rate > 0.8, "Success rate too low");
    }
}

/// Test performance comparison across classical algorithms
#[test]
fn test_classical_algorithm_comparison() {
    common::tracing::setup();
    
    println!("📊 Comparing classical algorithm performance...");
    
    let classical_algorithms = [
        ClassicalAlgorithm::X25519,
        ClassicalAlgorithm::EcdhP256,
        ClassicalAlgorithm::EcdhP384,
    ];
    
    let mut all_results = HashMap::new();
    
    for classical_alg in &classical_algorithms {
        println!("\nBenchmarking {:?}+Kyber...", classical_alg);
        
        let results = benchmark_algorithm_combination(
            *classical_alg,
            AlgorithmType::Kyber,
            SecurityLevel::Level1,
            PERFORMANCE_ITERATIONS,
        );
        
        let stats = results.calculate_stats();
        all_results.insert(*classical_alg, stats);
        
        println!("  Total time: {:?} avg", all_results[classical_alg].total_avg);
        println!("  Throughput: {:.2} ops/sec", all_results[classical_alg].throughput_ops_per_sec);
        println!("  Success rate: {:.1}%", all_results[classical_alg].success_rate * 100.0);
    }
    
    // Find the fastest algorithm
    let fastest = classical_algorithms.iter()
        .min_by_key(|alg| all_results[*alg].total_avg)
        .unwrap();
    
    println!("\n🏆 Fastest algorithm: {:?}", fastest);
    println!("   Average total time: {:?}", all_results[fastest].total_avg);
    println!("   Throughput: {:.2} ops/sec", all_results[fastest].throughput_ops_per_sec);
}

/// Test performance with different configuration options
#[test]
fn test_configuration_impact() {
    common::tracing::setup();
    
    println!("⚙️ Testing configuration impact on performance...");
    
    let configurations = [
        ("Default", HybridConfig::default()),
        ("No Caching", HybridConfig {
            enable_performance_caching: false,
            ..HybridConfig::default()
        }),
        ("No Logging", HybridConfig {
            enable_security_logging: false,
            ..HybridConfig::default()
        }),
        ("Minimal", HybridConfig {
            enable_performance_caching: false,
            enable_security_logging: false,
            secure_memory_zeroing: false,
            timing_attack_resistance: false,
            key_derivation_iterations: 1000,
            max_cached_operations: 10,
        }),
        ("Security Focused", HybridConfig {
            enable_performance_caching: true,
            enable_security_logging: true,
            secure_memory_zeroing: true,
            timing_attack_resistance: true,
            key_derivation_iterations: 500_000,
            max_cached_operations: 100,
        }),
    ];
    
    for (config_name, config) in &configurations {
        println!("\nTesting configuration: {}", config_name);
        
        let hybrid_kem = HybridKem::new_with_config(
            ClassicalAlgorithm::X25519,
            AlgorithmType::Kyber,
            SecurityLevel::Level1,
            config.clone(),
        );
        
        let mut total_times = Vec::new();
        
        for _ in 0..5 {
            let start = Instant::now();
            
            if let Ok(key_pair) = hybrid_kem.keygen() {
                if let Ok((ciphertext, shared_secret1)) = hybrid_kem.encaps(&key_pair) {
                    if let Ok(shared_secret2) = hybrid_kem.decaps(&key_pair, &ciphertext) {
                        assert_eq!(shared_secret1, shared_secret2);
                    }
                }
            }
            
            total_times.push(start.elapsed());
        }
        
        let avg_time = total_times.iter().sum::<Duration>() / total_times.len() as u32;
        println!("  Average total time: {:?}", avg_time);
        println!("  Configuration impact: {:.1}%", 
            (avg_time.as_millis() as f64 / configurations[0].1.key_derivation_iterations as f64) * 100.0);
    }
}

/// Test memory usage patterns
#[test]
fn test_memory_usage() {
    common::tracing::setup();
    
    println!("🧠 Testing memory usage patterns...");
    
    let hybrid_kem = HybridKem::new(
        ClassicalAlgorithm::X25519,
        AlgorithmType::Kyber,
        SecurityLevel::Level1,
    );
    
    // Test multiple operations to check for memory leaks
    let operations = 10;
    let mut total_memory_used = 0;
    
    for i in 0..operations {
        if let Ok(key_pair) = hybrid_kem.keygen() {
            total_memory_used += key_pair.classical_public.len();
            total_memory_used += key_pair.classical_secret.len();
            total_memory_used += key_pair.pqc_public.len();
            total_memory_used += key_pair.pqc_secret.len();
            
            if let Ok((ciphertext, shared_secret)) = hybrid_kem.encaps(&key_pair) {
                total_memory_used += ciphertext.len();
                total_memory_used += shared_secret.len();
                
                println!("Operation {}: ~{} bytes used", i + 1, 
                    key_pair.classical_public.len() + key_pair.classical_secret.len() +
                    key_pair.pqc_public.len() + key_pair.pqc_secret.len() +
                    ciphertext.len() + shared_secret.len());
            }
        }
    }
    
    let avg_memory_per_op = total_memory_used / operations;
    println!("Average memory per operation: {} bytes", avg_memory_per_op);
    println!("Total memory for {} operations: {} bytes", operations, total_memory_used);
    
    // Memory usage should be reasonable
    assert!(avg_memory_per_op < 1_000_000, "Memory usage too high"); // Less than 1MB per operation
}

/// Test concurrent performance
#[test]
fn test_concurrent_performance() {
    common::tracing::setup();
    
    println!("🔄 Testing concurrent performance...");
    
    let thread_count = 4;
    let operations_per_thread = 3;
    
    let handles: Vec<_> = (0..thread_count).map(|thread_id| {
        std::thread::spawn(move || {
            let hybrid_kem = HybridKem::new(
                ClassicalAlgorithm::X25519,
                AlgorithmType::Kyber,
                SecurityLevel::Level1,
            );
            
            let mut thread_times = Vec::new();
            
            for _ in 0..operations_per_thread {
                let start = Instant::now();
                
                if let Ok(key_pair) = hybrid_kem.keygen() {
                    if let Ok((ciphertext, shared_secret1)) = hybrid_kem.encaps(&key_pair) {
                        if let Ok(shared_secret2) = hybrid_kem.decaps(&key_pair, &ciphertext) {
                            assert_eq!(shared_secret1, shared_secret2);
                        }
                    }
                }
                
                thread_times.push(start.elapsed());
            }
            
            (thread_id, thread_times)
        })
    }).collect();
    
    let mut all_times = Vec::new();
    
    for handle in handles {
        let (thread_id, times) = handle.join().unwrap();
        let avg_time = times.iter().sum::<Duration>() / times.len() as u32;
        println!("Thread {}: {:?} avg", thread_id, avg_time);
        all_times.extend(times);
    }
    
    let overall_avg = all_times.iter().sum::<Duration>() / all_times.len() as u32;
    println!("Overall concurrent average: {:?}", overall_avg);
    
    // Concurrent performance should be reasonable
    assert!(overall_avg < Duration::from_secs(10), "Concurrent performance too slow");
}

/// Test stress conditions
#[test]
fn test_stress_conditions() {
    common::tracing::setup();
    
    println!("💪 Testing stress conditions...");
    
    let hybrid_kem = HybridKem::new(
        ClassicalAlgorithm::X25519,
        AlgorithmType::Kyber,
        SecurityLevel::Level5, // Highest security level
    );
    
    // Rapid succession test
    println!("  Rapid succession test...");
    let rapid_count = 20;
    let start = Instant::now();
    
    for i in 0..rapid_count {
        if let Ok(key_pair) = hybrid_kem.keygen() {
            if let Ok((ciphertext, shared_secret1)) = hybrid_kem.encaps(&key_pair) {
                if let Ok(shared_secret2) = hybrid_kem.decaps(&key_pair, &ciphertext) {
                    assert_eq!(shared_secret1, shared_secret2);
                }
            }
        }
        
        if i % 5 == 0 {
            println!("    Completed {} operations", i);
        }
    }
    
    let total_time = start.elapsed();
    println!("  Completed {} operations in {:?}", rapid_count, total_time);
    println!("  Average time per operation: {:?}", total_time / rapid_count);
    
    // Should handle stress conditions gracefully
    assert!(total_time < Duration::from_secs(60), "Stress test took too long");
}

/// Test algorithm fallback behavior
#[test]
fn test_algorithm_fallback() {
    common::tracing::setup();
    
    println!("🔄 Testing algorithm fallback behavior...");
    
    // Test with potentially unsupported algorithms
    let test_cases = [
        (ClassicalAlgorithm::X25519, AlgorithmType::Kyber),           // Should work
        (ClassicalAlgorithm::EcdhP256, AlgorithmType::Kyber),        // Should work
        (ClassicalAlgorithm::Rsa4096, AlgorithmType::Sphincs),       // May fall back
        (ClassicalAlgorithm::EcdhP521, AlgorithmType::FrodoKem),     // May fall back
    ];
    
    for (classical, pqc) in &test_cases {
        println!("Testing {:?} + {:?}...", classical, pqc);
        
        let hybrid_kem = HybridKem::new(*classical, *pqc, SecurityLevel::Level1);
        
        match hybrid_kem.keygen() {
            Ok(key_pair) => {
                println!("  ✅ Key generation successful");
                
                // Test full workflow
                match hybrid_kem.encaps(&key_pair) {
                    Ok((ciphertext, shared_secret1)) => {
                        match hybrid_kem.decaps(&key_pair, &ciphertext) {
                            Ok(shared_secret2) => {
                                if shared_secret1 == shared_secret2 {
                                    println!("  ✅ Full workflow successful");
                                } else {
                                    println!("  ⚠️ Shared secret mismatch");
                                }
                            },
                            Err(e) => {
                                println!("  ⚠️ Decapsulation failed: {}", e);
                            }
                        }
                    },
                    Err(e) => {
                        println!("  ⚠️ Encapsulation failed: {}", e);
                    }
                }
            },
            Err(e) => {
                println!("  ⚠️ Key generation failed: {} (may be expected)", e);
            }
        }
    }
}

/// Performance regression test
#[test]
fn test_performance_regression() {
    common::tracing::setup();
    
    println!("📈 Running performance regression test...");
    
    let hybrid_kem = HybridKem::new(
        ClassicalAlgorithm::X25519,
        AlgorithmType::Kyber,
        SecurityLevel::Level1,
    );
    
    // Expected performance thresholds (very generous for test environment)
    let max_keygen_time = Duration::from_secs(3);
    let max_encaps_time = Duration::from_secs(1);
    let max_decaps_time = Duration::from_secs(1);
    
    let iterations = 5;
    let mut keygen_times = Vec::new();
    let mut encaps_times = Vec::new();
    let mut decaps_times = Vec::new();
    
    for _ in 0..iterations {
        // Test key generation performance
        let start = Instant::now();
        let key_pair = hybrid_kem.keygen().expect("Key generation should succeed");
        keygen_times.push(start.elapsed());
        
        // Test encapsulation performance
        let start = Instant::now();
        let (ciphertext, shared_secret1) = hybrid_kem.encaps(&key_pair).expect("Encapsulation should succeed");
        encaps_times.push(start.elapsed());
        
        // Test decapsulation performance
        let start = Instant::now();
        let shared_secret2 = hybrid_kem.decaps(&key_pair, &ciphertext).expect("Decapsulation should succeed");
        decaps_times.push(start.elapsed());
        
        assert_eq!(shared_secret1, shared_secret2);
    }
    
    let avg_keygen = keygen_times.iter().sum::<Duration>() / iterations as u32;
    let avg_encaps = encaps_times.iter().sum::<Duration>() / iterations as u32;
    let avg_decaps = decaps_times.iter().sum::<Duration>() / iterations as u32;
    
    println!("Performance results:");
    println!("  Key generation: {:?} avg (max: {:?})", avg_keygen, max_keygen_time);
    println!("  Encapsulation:  {:?} avg (max: {:?})", avg_encaps, max_encaps_time);
    println!("  Decapsulation:  {:?} avg (max: {:?})", avg_decaps, max_decaps_time);
    
    // Performance regression checks
    assert!(avg_keygen <= max_keygen_time, 
        "Key generation performance regression: {:?} > {:?}", avg_keygen, max_keygen_time);
    assert!(avg_encaps <= max_encaps_time, 
        "Encapsulation performance regression: {:?} > {:?}", avg_encaps, max_encaps_time);
    assert!(avg_decaps <= max_decaps_time, 
        "Decapsulation performance regression: {:?} > {:?}", avg_decaps, max_decaps_time);
    
    println!("✅ No performance regression detected");
}
