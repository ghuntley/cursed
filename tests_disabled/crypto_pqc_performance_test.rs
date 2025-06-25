//! Performance Test Suite for Post-Quantum Cryptography Module
//! 
//! This test suite focuses on comprehensive performance validation of PQC algorithms
//! including benchmarking, memory profiling, throughput analysis, and scalability testing.

use cursed::stdlib::crypto::pqc::*;
use std::time::{Duration, Instant};
use std::collections::HashMap;

// ============================================================================
// PERFORMANCE BENCHMARKING TESTS
// ============================================================================

#[test]
fn test_kyber_keygen_performance() {
    let iterations = 100;
    let security_levels = [SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5];
    
    for level in security_levels {
        let start = Instant::now();
        
        for _ in 0..iterations {
            let result = KyberKem::keygen(level);
            assert!(result.is_ok(), "Kyber keygen failed for {:?}", level);
        }
        
        let elapsed = start.elapsed();
        let per_operation = elapsed / iterations;
        
        println!("Kyber {:?} keygen: {:?} per operation", level, per_operation);
        
        // Performance bounds (generous for simulation)
        assert!(per_operation < Duration::from_millis(100), 
               "Kyber {:?} keygen too slow: {:?}", level, per_operation);
    }
}

#[test]
fn test_kyber_encaps_decaps_performance() {
    let iterations = 1000;
    let (public_key, secret_key) = KyberKem::keygen(SecurityLevel::Level3).unwrap();
    
    // Benchmark encapsulation
    let start = Instant::now();
    let mut ciphertexts = Vec::new();
    let mut shared_secrets = Vec::new();
    
    for _ in 0..iterations {
        let (ciphertext, shared_secret) = KyberKem::encaps(&public_key).unwrap();
        ciphertexts.push(ciphertext);
        shared_secrets.push(shared_secret);
    }
    
    let encaps_elapsed = start.elapsed();
    let encaps_per_op = encaps_elapsed / iterations;
    
    // Benchmark decapsulation
    let start = Instant::now();
    
    for (ciphertext, expected_secret) in ciphertexts.iter().zip(shared_secrets.iter()) {
        let recovered_secret = KyberKem::decaps(&secret_key, ciphertext).unwrap();
        assert_eq!(&recovered_secret, expected_secret);
    }
    
    let decaps_elapsed = start.elapsed();
    let decaps_per_op = decaps_elapsed / iterations;
    
    println!("Kyber encaps: {:?} per operation", encaps_per_op);
    println!("Kyber decaps: {:?} per operation", decaps_per_op);
    
    // Performance assertions
    assert!(encaps_per_op < Duration::from_millis(10), "Encapsulation too slow");
    assert!(decaps_per_op < Duration::from_millis(10), "Decapsulation too slow");
}

#[test]
fn test_dilithium_signing_performance() {
    let iterations = 100;
    let (public_key, secret_key) = DilithiumSignature::keygen(SecurityLevel::Level3).unwrap();
    let message = b"Performance test message for Dilithium signatures";
    
    // Benchmark signing
    let start = Instant::now();
    let mut signatures = Vec::new();
    
    for _ in 0..iterations {
        let signature = DilithiumSignature::sign(&secret_key, message).unwrap();
        signatures.push(signature);
    }
    
    let signing_elapsed = start.elapsed();
    let signing_per_op = signing_elapsed / iterations;
    
    // Benchmark verification
    let start = Instant::now();
    
    for signature in &signatures {
        let _is_valid = DilithiumSignature::verify(&public_key, message, signature).unwrap();
    }
    
    let verify_elapsed = start.elapsed();
    let verify_per_op = verify_elapsed / iterations;
    
    println!("Dilithium signing: {:?} per operation", signing_per_op);
    println!("Dilithium verification: {:?} per operation", verify_per_op);
    
    // Performance assertions
    assert!(signing_per_op < Duration::from_millis(50), "Signing too slow");
    assert!(verify_per_op < Duration::from_millis(50), "Verification too slow");
}

#[test]
fn test_memory_usage_analysis() {
    // Test memory usage patterns for different algorithms
    
    // Kyber memory usage
    let (kyber_pub, kyber_sec) = KyberKem::keygen(SecurityLevel::Level5).unwrap();
    let kyber_memory = kyber_pub.key_data.len() + kyber_sec.key_data.len();
    
    // Dilithium memory usage
    let (dilithium_pub, dilithium_sec) = DilithiumSignature::keygen(SecurityLevel::Level5).unwrap();
    let dilithium_memory = dilithium_pub.key_data.len() + dilithium_sec.key_data.len();
    
    // NTRU memory usage
    let (ntru_pub, ntru_sec) = NtruEncryption::keygen(SecurityLevel::Level5).unwrap();
    let ntru_memory = ntru_pub.key_data.len() + ntru_sec.key_data.len();
    
    println!("Memory usage analysis:");
    println!("Kyber-1024: {} bytes", kyber_memory);
    println!("Dilithium5: {} bytes", dilithium_memory);
    println!("NTRU: {} bytes", ntru_memory);
    
    // Memory bounds (should be reasonable)
    assert!(kyber_memory < 10000, "Kyber memory usage excessive");
    assert!(dilithium_memory < 20000, "Dilithium memory usage excessive");
    assert!(ntru_memory < 15000, "NTRU memory usage excessive");
}

#[test]
fn test_throughput_analysis() {
    let test_duration = Duration::from_secs(5);
    let message = b"Throughput test message";
    
    // Kyber throughput
    let (kyber_pub, kyber_sec) = KyberKem::keygen(SecurityLevel::Level3).unwrap();
    let start = Instant::now();
    let mut kyber_ops = 0;
    
    while start.elapsed() < test_duration {
        let (ciphertext, _) = KyberKem::encaps(&kyber_pub).unwrap();
        let _ = KyberKem::decaps(&kyber_sec, &ciphertext).unwrap();
        kyber_ops += 1;
    }
    
    let kyber_throughput = kyber_ops as f64 / test_duration.as_secs_f64();
    
    // Dilithium throughput
    let (dilithium_pub, dilithium_sec) = DilithiumSignature::keygen(SecurityLevel::Level3).unwrap();
    let start = Instant::now();
    let mut dilithium_ops = 0;
    
    while start.elapsed() < test_duration {
        let signature = DilithiumSignature::sign(&dilithium_sec, message).unwrap();
        let _ = DilithiumSignature::verify(&dilithium_pub, message, &signature).unwrap();
        dilithium_ops += 1;
    }
    
    let dilithium_throughput = dilithium_ops as f64 / test_duration.as_secs_f64();
    
    println!("Throughput analysis:");
    println!("Kyber: {:.2} ops/sec", kyber_throughput);
    println!("Dilithium: {:.2} ops/sec", dilithium_throughput);
    
    // Minimum throughput requirements
    assert!(kyber_throughput > 10.0, "Kyber throughput too low: {:.2}", kyber_throughput);
    assert!(dilithium_throughput > 5.0, "Dilithium throughput too low: {:.2}", dilithium_throughput);
}

// ============================================================================
// SCALABILITY TESTS
// ============================================================================

#[test]
fn test_concurrent_performance() {
    use std::thread;
    use std::sync::Arc;
    
    let thread_count = 4;
    let operations_per_thread = 50;
    
    let handles: Vec<_> = (0..thread_count).map(|thread_id| {
        thread::spawn(move || {
            let start = Instant::now();
            
            for _ in 0..operations_per_thread {
                let (pub_key, sec_key) = KyberKem::keygen(SecurityLevel::Level1).unwrap();
                let (ciphertext, shared_secret1) = KyberKem::encaps(&pub_key).unwrap();
                let shared_secret2 = KyberKem::decaps(&sec_key, &ciphertext).unwrap();
                assert_eq!(shared_secret1, shared_secret2);
            }
            
            let elapsed = start.elapsed();
            (thread_id, elapsed, operations_per_thread)
        })
    }).collect();
    
    let mut total_operations = 0;
    let mut total_time = Duration::from_secs(0);
    
    for handle in handles {
        let (thread_id, elapsed, ops) = handle.join().unwrap();
        println!("Thread {}: {} ops in {:?}", thread_id, ops, elapsed);
        total_operations += ops;
        total_time = total_time.max(elapsed); // Use maximum time for parallel execution
    }
    
    let overall_throughput = total_operations as f64 / total_time.as_secs_f64();
    println!("Concurrent throughput: {:.2} ops/sec", overall_throughput);
    
    // Should scale reasonably with concurrent execution
    assert!(overall_throughput > 50.0, "Concurrent throughput too low");
}

#[test]
fn test_message_size_performance() {
    let (public_key, secret_key) = DilithiumSignature::keygen(SecurityLevel::Level1).unwrap();
    
    let message_sizes = [64, 1024, 4096, 16384, 65536]; // 64 bytes to 64KB
    
    for size in message_sizes {
        let message = vec![42u8; size];
        
        let start = Instant::now();
        let signature = DilithiumSignature::sign(&secret_key, &message).unwrap();
        let signing_time = start.elapsed();
        
        let start = Instant::now();
        let _is_valid = DilithiumSignature::verify(&public_key, &message, &signature).unwrap();
        let verify_time = start.elapsed();
        
        println!("Message size {}: sign={:?}, verify={:?}", size, signing_time, verify_time);
        
        // Performance should not degrade significantly with message size
        assert!(signing_time < Duration::from_millis(200), 
               "Signing time excessive for size {}: {:?}", size, signing_time);
        assert!(verify_time < Duration::from_millis(200), 
               "Verification time excessive for size {}: {:?}", size, verify_time);
    }
}

// ============================================================================
// ALGORITHM COMPARISON TESTS
// ============================================================================

#[test]
fn test_security_level_performance_comparison() {
    let levels = [SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5];
    let iterations = 50;
    
    for level in levels {
        // Kyber performance
        let start = Instant::now();
        for _ in 0..iterations {
            let (pub_key, sec_key) = KyberKem::keygen(level).unwrap();
            let (ciphertext, _) = KyberKem::encaps(&pub_key).unwrap();
            let _ = KyberKem::decaps(&sec_key, &ciphertext).unwrap();
        }
        let kyber_time = start.elapsed() / iterations;
        
        // Dilithium performance
        let start = Instant::now();
        for _ in 0..iterations {
            let (pub_key, sec_key) = DilithiumSignature::keygen(level).unwrap();
            let signature = DilithiumSignature::sign(&sec_key, b"test").unwrap();
            let _ = DilithiumSignature::verify(&pub_key, b"test", &signature).unwrap();
        }
        let dilithium_time = start.elapsed() / iterations;
        
        println!("Security {:?}: Kyber={:?}, Dilithium={:?}", level, kyber_time, dilithium_time);
        
        // Higher security levels should take more time, but not excessively
        assert!(kyber_time < Duration::from_millis(500), 
               "Kyber {:?} too slow: {:?}", level, kyber_time);
        assert!(dilithium_time < Duration::from_millis(500), 
               "Dilithium {:?} too slow: {:?}", level, dilithium_time);
    }
}

#[test]
fn test_signature_algorithm_comparison() {
    let message = b"Signature algorithm comparison test";
    let iterations = 20;
    
    // Dilithium
    let (dilithium_pub, dilithium_sec) = DilithiumSignature::keygen(SecurityLevel::Level1).unwrap();
    let start = Instant::now();
    for _ in 0..iterations {
        let sig = DilithiumSignature::sign(&dilithium_sec, message).unwrap();
        let _ = DilithiumSignature::verify(&dilithium_pub, message, &sig).unwrap();
    }
    let dilithium_time = start.elapsed() / iterations;
    
    // Falcon
    let (falcon_pub, falcon_sec) = FalconSignature::keygen(SecurityLevel::Level1).unwrap();
    let start = Instant::now();
    for _ in 0..iterations {
        let sig = FalconSignature::sign(&falcon_sec, message).unwrap();
        let _ = FalconSignature::verify(&falcon_pub, message, &sig).unwrap();
    }
    let falcon_time = start.elapsed() / iterations;
    
    // SPHINCS+
    let (sphincs_pub, sphincs_sec) = SphincsPlusSignature::keygen(SecurityLevel::Level1).unwrap();
    let start = Instant::now();
    for _ in 0..iterations {
        let sig = SphincsPlusSignature::sign(&sphincs_sec, message).unwrap();
        let _ = SphincsPlusSignature::verify(&sphincs_pub, message, &sig).unwrap();
    }
    let sphincs_time = start.elapsed() / iterations;
    
    println!("Signature algorithm comparison:");
    println!("Dilithium: {:?} per sign+verify", dilithium_time);
    println!("Falcon: {:?} per sign+verify", falcon_time);
    println!("SPHINCS+: {:?} per sign+verify", sphincs_time);
    
    // All should complete in reasonable time
    assert!(dilithium_time < Duration::from_millis(100));
    assert!(falcon_time < Duration::from_millis(100));
    assert!(sphincs_time < Duration::from_millis(500)); // SPHINCS+ is typically slower
}

// ============================================================================
// STRESS TESTING
// ============================================================================

#[test]
fn test_sustained_load_performance() {
    let test_duration = Duration::from_secs(10);
    let (public_key, secret_key) = KyberKem::keygen(SecurityLevel::Level1).unwrap();
    
    let start = Instant::now();
    let mut operations = 0;
    let mut max_operation_time = Duration::from_nanos(0);
    let mut min_operation_time = Duration::from_secs(1);
    
    while start.elapsed() < test_duration {
        let op_start = Instant::now();
        
        let (ciphertext, shared_secret1) = KyberKem::encaps(&public_key).unwrap();
        let shared_secret2 = KyberKem::decaps(&secret_key, &ciphertext).unwrap();
        assert_eq!(shared_secret1, shared_secret2);
        
        let op_time = op_start.elapsed();
        max_operation_time = max_operation_time.max(op_time);
        min_operation_time = min_operation_time.min(op_time);
        operations += 1;
    }
    
    let total_time = start.elapsed();
    let avg_throughput = operations as f64 / total_time.as_secs_f64();
    
    println!("Sustained load test results:");
    println!("Total operations: {}", operations);
    println!("Average throughput: {:.2} ops/sec", avg_throughput);
    println!("Min operation time: {:?}", min_operation_time);
    println!("Max operation time: {:?}", max_operation_time);
    
    // Performance should be consistent under sustained load
    assert!(operations > 100, "Not enough operations completed");
    assert!(max_operation_time < Duration::from_millis(100), "Operation time spiked too high");
    assert!(avg_throughput > 10.0, "Average throughput too low under sustained load");
}

#[test]
fn test_memory_pressure_performance() {
    // Test performance under memory pressure
    let mut large_allocations = Vec::new();
    
    // Allocate significant memory to create pressure
    for _ in 0..10 {
        large_allocations.push(vec![0u8; 1024 * 1024]); // 1MB each
    }
    
    // Test operations under memory pressure
    let start = Instant::now();
    let (public_key, secret_key) = KyberKem::keygen(SecurityLevel::Level3).unwrap();
    let keygen_time = start.elapsed();
    
    let start = Instant::now();
    let (ciphertext, shared_secret1) = KyberKem::encaps(&public_key).unwrap();
    let encaps_time = start.elapsed();
    
    let start = Instant::now();
    let shared_secret2 = KyberKem::decaps(&secret_key, &ciphertext).unwrap();
    let decaps_time = start.elapsed();
    
    assert_eq!(shared_secret1, shared_secret2);
    
    println!("Performance under memory pressure:");
    println!("Keygen: {:?}", keygen_time);
    println!("Encaps: {:?}", encaps_time);
    println!("Decaps: {:?}", decaps_time);
    
    // Should still perform reasonably under memory pressure
    assert!(keygen_time < Duration::from_millis(500));
    assert!(encaps_time < Duration::from_millis(200));
    assert!(decaps_time < Duration::from_millis(200));
    
    // Clean up memory
    drop(large_allocations);
}

// ============================================================================
// PERFORMANCE REGRESSION DETECTION
// ============================================================================

#[test]
fn test_performance_baseline_kyber() {
    // Establish performance baselines for regression detection
    let iterations = 100;
    let (public_key, secret_key) = KyberKem::keygen(SecurityLevel::Level3).unwrap();
    
    // Measure encapsulation
    let start = Instant::now();
    let mut results = Vec::new();
    for _ in 0..iterations {
        let result = KyberKem::encaps(&public_key).unwrap();
        results.push(result);
    }
    let encaps_baseline = start.elapsed() / iterations;
    
    // Measure decapsulation
    let start = Instant::now();
    for (ciphertext, expected) in &results {
        let recovered = KyberKem::decaps(&secret_key, ciphertext).unwrap();
        assert_eq!(&recovered, expected);
    }
    let decaps_baseline = start.elapsed() / iterations;
    
    println!("Kyber performance baselines:");
    println!("Encapsulation: {:?} per operation", encaps_baseline);
    println!("Decapsulation: {:?} per operation", decaps_baseline);
    
    // Store baselines for future regression testing
    // In a real implementation, these would be stored and compared
    assert!(encaps_baseline < Duration::from_millis(50));
    assert!(decaps_baseline < Duration::from_millis(50));
}

#[test]
fn test_performance_baseline_dilithium() {
    let iterations = 50;
    let message = b"Performance baseline test message";
    let (public_key, secret_key) = DilithiumSignature::keygen(SecurityLevel::Level3).unwrap();
    
    // Measure signing
    let start = Instant::now();
    let mut signatures = Vec::new();
    for _ in 0..iterations {
        let signature = DilithiumSignature::sign(&secret_key, message).unwrap();
        signatures.push(signature);
    }
    let signing_baseline = start.elapsed() / iterations;
    
    // Measure verification
    let start = Instant::now();
    for signature in &signatures {
        let _ = DilithiumSignature::verify(&public_key, message, signature).unwrap();
    }
    let verify_baseline = start.elapsed() / iterations;
    
    println!("Dilithium performance baselines:");
    println!("Signing: {:?} per operation", signing_baseline);
    println!("Verification: {:?} per operation", verify_baseline);
    
    // Performance baselines
    assert!(signing_baseline < Duration::from_millis(100));
    assert!(verify_baseline < Duration::from_millis(100));
}
