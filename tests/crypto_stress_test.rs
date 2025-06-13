/// fr fr Crypto stress tests for CURSED - pushing limits periodt
/// 
/// This test suite validates crypto operations under extreme conditions:
/// - Large file encryption/decryption
/// - High-volume operations
/// - Concurrent stress testing
/// - Memory pressure scenarios
/// - Sustained load testing
/// 
/// These tests ensure the crypto infrastructure remains stable under stress.

use cursed::stdlib::crypto::*;
use cursed::stdlib::packages::crypto_random::*;
use std::time::{Instant, Duration};
use std::thread;
use std::sync::{Arc, Barrier};
use std::collections::HashMap;

// Initialize tracing for tests
macro_rules! init_tracing {
    () => {
        let _ = tracing_subscriber::fmt()
            .with_env_filter("info")
            .with_test_writer()
            .try_init();
    };
}

#[test]
#[ignore] // Run with --ignored flag for stress tests
fn test_large_file_encryption() {
    init_tracing!();
    tracing::info!("Testing large file encryption/decryption");
    
    // Test with progressively larger data sizes
    let sizes = [
        1024,           // 1KB
        10 * 1024,      // 10KB  
        100 * 1024,     // 100KB
        1024 * 1024,    // 1MB
        10 * 1024 * 1024, // 10MB
        100 * 1024 * 1024, // 100MB
    ];
    
    let key = vec![42u8; 32];
    let cipher = Aes256Gcm::new(&key).expect("Failed to create cipher");
    
    for &size in &sizes {
        tracing::info!(size_mb = size / (1024 * 1024), "Testing encryption of large data");
        
        // Generate test data
        let start_time = Instant::now();
        let mut test_data = vec![0u8; size];
        fill_random(&mut test_data).expect("Failed to generate test data");
        let generation_time = start_time.elapsed();
        
        // Encrypt
        let start_time = Instant::now();
        let encrypted = cipher.encrypt(&test_data, b"")
            .expect("Failed to encrypt large data");
        let encryption_time = start_time.elapsed();
        
        // Decrypt
        let start_time = Instant::now();
        let decrypted = cipher.decrypt(&encrypted.ciphertext, b"", &encrypted)
            .expect("Failed to decrypt large data");
        let decryption_time = start_time.elapsed();
        
        // Verify correctness
        assert_eq!(decrypted.plaintext, test_data, "Large data encryption/decryption failed");
        
        // Calculate throughput
        let encryption_mbps = (size as f64) / (1024.0 * 1024.0) / encryption_time.as_secs_f64();
        let decryption_mbps = (size as f64) / (1024.0 * 1024.0) / decryption_time.as_secs_f64();
        
        tracing::info!(
            size_mb = size / (1024 * 1024),
            generation_ms = generation_time.as_millis(),
            encryption_ms = encryption_time.as_millis(),
            decryption_ms = decryption_time.as_millis(),
            encryption_mbps = encryption_mbps,
            decryption_mbps = decryption_mbps,
            "Large file encryption completed"
        );
        
        // Performance thresholds (should achieve reasonable throughput)
        if size >= 1024 * 1024 { // For 1MB+ files
            assert!(encryption_mbps > 10.0, "Encryption too slow: {:.2} MB/s", encryption_mbps);
            assert!(decryption_mbps > 10.0, "Decryption too slow: {:.2} MB/s", decryption_mbps);
        }
    }
}

#[test]
#[ignore] // Run with --ignored flag for stress tests
fn test_high_volume_hash_operations() {
    init_tracing!();
    tracing::info!("Testing high-volume hash operations");
    
    let iterations = 10000;
    let test_data = b"The quick brown fox jumps over the lazy dog";
    
    // Test SHA-256 performance
    let start_time = Instant::now();
    let mut hashes = Vec::with_capacity(iterations);
    for i in 0..iterations {
        let mut data = test_data.to_vec();
        data.extend_from_slice(&i.to_le_bytes()); // Make each hash unique
        let hash = Sha256::hash(&data);
        hashes.push(hash);
    }
    let sha256_duration = start_time.elapsed();
    
    // Test SHA-512 performance
    let start_time = Instant::now();
    for i in 0..iterations {
        let mut data = test_data.to_vec();
        data.extend_from_slice(&i.to_le_bytes());
        let _hash = Sha512::hash(&data);
    }
    let sha512_duration = start_time.elapsed();
    
    // Test MD5 performance (for comparison)
    let start_time = Instant::now();
    for i in 0..iterations {
        let mut data = test_data.to_vec();
        data.extend_from_slice(&i.to_le_bytes());
        let _hash = Md5::hash(&data);
    }
    let md5_duration = start_time.elapsed();
    
    // Calculate rates
    let sha256_rate = iterations as f64 / sha256_duration.as_secs_f64();
    let sha512_rate = iterations as f64 / sha512_duration.as_secs_f64();
    let md5_rate = iterations as f64 / md5_duration.as_secs_f64();
    
    tracing::info!(
        iterations = iterations,
        sha256_total_ms = sha256_duration.as_millis(),
        sha512_total_ms = sha512_duration.as_millis(),
        md5_total_ms = md5_duration.as_millis(),
        sha256_per_sec = sha256_rate,
        sha512_per_sec = sha512_rate,
        md5_per_sec = md5_rate,
        "High-volume hash operations completed"
    );
    
    // Verify all hashes are unique (basic collision test)
    let mut unique_hashes = std::collections::HashSet::new();
    for hash in &hashes {
        assert!(unique_hashes.insert(hash), "Hash collision detected in test data");
    }
    
    // Performance assertions
    assert!(sha256_rate > 1000.0, "SHA-256 rate too low: {:.0} hashes/sec", sha256_rate);
    assert!(sha512_rate > 500.0, "SHA-512 rate too low: {:.0} hashes/sec", sha512_rate);
    assert!(md5_rate > 5000.0, "MD5 rate too low: {:.0} hashes/sec", md5_rate);
}

#[test]
#[ignore] // Run with --ignored flag for stress tests
fn test_concurrent_crypto_stress() {
    init_tracing!();
    tracing::info!("Testing concurrent crypto stress scenarios");
    
    let num_threads = 12; // Higher than typical core count
    let operations_per_thread = 1000;
    let barrier = Arc::new(Barrier::new(num_threads));
    
    let test_data = b"Concurrent stress test data for crypto operations".repeat(10);
    let key = vec![42u8; 32];
    
    let handles: Vec<_> = (0..num_threads).map(|thread_id| {
        let barrier = Arc::clone(&barrier);
        let data = test_data.clone();
        let key = key.clone();
        
        thread::spawn(move || {
            // Wait for all threads to be ready
            barrier.wait();
            
            let cipher = Aes256Gcm::new(&key).expect("Failed to create cipher");
            let start_time = Instant::now();
            
            for op_id in 0..operations_per_thread {
                // Mix different operations
                match op_id % 4 {
                    0 => {
                        // Encryption/Decryption
                        let encrypted = cipher.encrypt(&data, b"").expect("Encryption failed");
                        let decrypted = cipher.decrypt(&encrypted.ciphertext, b"", &encrypted)
                            .expect("Decryption failed");
                        assert_eq!(decrypted.plaintext, data);
                    },
                    1 => {
                        // Hashing
                        let _hash = Sha256::hash(&data);
                    },
                    2 => {
                        // Random generation
                        let _random = generate_random_bytes(64).expect("Random generation failed");
                    },
                    3 => {
                        // Key derivation (lightweight version)
                        let manager = KeyManager::new().expect("Failed to create key manager");
                        let config = KeyDerivationConfig {
                            iterations: 1000, // Reduced for stress test
                            salt: b"stress_test_salt".to_vec(),
                            key_length: 32,
                        };
                        let _key = manager.derive_key_pbkdf2(b"password", &config)
                            .expect("Key derivation failed");
                    },
                    _ => unreachable!(),
                }
                
                if op_id % 100 == 0 {
                    tracing::trace!(thread_id = thread_id, operation = op_id, "Progress update");
                }
            }
            
            let duration = start_time.elapsed();
            let ops_per_sec = operations_per_thread as f64 / duration.as_secs_f64();
            
            tracing::info!(
                thread_id = thread_id,
                operations = operations_per_thread,
                duration_ms = duration.as_millis(),
                ops_per_sec = ops_per_sec,
                "Thread completed stress test"
            );
            
            ops_per_sec
        })
    }).collect();
    
    // Wait for all threads and collect results
    let mut total_ops_per_sec = 0.0;
    for handle in handles {
        let ops_per_sec = handle.join().expect("Thread panicked");
        total_ops_per_sec += ops_per_sec;
    }
    
    let total_operations = num_threads * operations_per_thread;
    
    tracing::info!(
        threads = num_threads,
        operations_per_thread = operations_per_thread,
        total_operations = total_operations,
        combined_ops_per_sec = total_ops_per_sec,
        "Concurrent crypto stress test completed"
    );
    
    // Should maintain reasonable performance under stress
    assert!(total_ops_per_sec > 1000.0, 
           "Combined throughput too low: {:.0} ops/sec", total_ops_per_sec);
}

#[test] 
#[ignore] // Run with --ignored flag for stress tests
fn test_memory_pressure_crypto_operations() {
    init_tracing!();
    tracing::info!("Testing crypto operations under memory pressure");
    
    let num_large_allocations = 10;
    let allocation_size = 10 * 1024 * 1024; // 10MB each = 100MB total
    
    // Allocate large amounts of memory to create pressure
    let mut large_allocations = Vec::new();
    for i in 0..num_large_allocations {
        let mut allocation = vec![i as u8; allocation_size];
        fill_random(&mut allocation).expect("Failed to fill allocation with random data");
        large_allocations.push(allocation);
        
        tracing::debug!(allocation = i, size_mb = allocation_size / (1024 * 1024), 
                       "Allocated large memory block");
    }
    
    // Perform crypto operations under memory pressure
    let key = vec![42u8; 32];
    let cipher = Aes256Gcm::new(&key).expect("Failed to create cipher under memory pressure");
    
    let test_data = b"Testing crypto under memory pressure".repeat(1000); // ~36KB
    let iterations = 100;
    
    let start_time = Instant::now();
    for i in 0..iterations {
        // Encryption
        let encrypted = cipher.encrypt(&test_data, b"")
            .expect("Encryption failed under memory pressure");
        
        // Decryption
        let decrypted = cipher.decrypt(&encrypted.ciphertext, b"", &encrypted)
            .expect("Decryption failed under memory pressure");
        
        assert_eq!(decrypted.plaintext, test_data, "Data corruption under memory pressure");
        
        // Additional operations
        let _hash = Sha256::hash(&test_data);
        let _random = generate_random_bytes(32).expect("Random generation failed under pressure");
        
        if i % 10 == 0 {
            tracing::debug!(iteration = i, "Memory pressure test progress");
        }
    }
    let duration = start_time.elapsed();
    
    let ops_per_sec = iterations as f64 / duration.as_secs_f64();
    let memory_used_mb = (num_large_allocations * allocation_size) / (1024 * 1024);
    
    tracing::info!(
        iterations = iterations,
        duration_ms = duration.as_millis(),
        ops_per_sec = ops_per_sec,
        memory_pressure_mb = memory_used_mb,
        "Memory pressure crypto test completed"
    );
    
    // Should still maintain reasonable performance under memory pressure
    assert!(ops_per_sec > 10.0, "Performance degraded too much under memory pressure: {:.1} ops/sec", ops_per_sec);
    
    // Clean up large allocations
    drop(large_allocations);
    tracing::info!("Memory pressure test cleanup completed");
}

#[test]
#[ignore] // Run with --ignored flag for stress tests
fn test_key_generation_under_load() {
    init_tracing!();
    tracing::info!("Testing key generation under sustained load");
    
    let duration = Duration::from_secs(30); // 30 second test
    let start_time = Instant::now();
    let mut key_count = 0;
    let mut total_key_bytes = 0;
    
    // Test different key generation methods
    let key_sizes = [16, 32, 64, 128];
    let mut key_type_counts = HashMap::new();
    
    while start_time.elapsed() < duration {
        let key_size = key_sizes[key_count % key_sizes.len()];
        
        // Generate encryption key
        let key_result = EncryptionKey::generate("Test", key_size);
        assert!(key_result.is_ok(), "Key generation failed under load");
        
        let key = key_result.unwrap();
        assert_eq!(key.size(), key_size, "Generated key has wrong size");
        
        // Test key derivation periodically
        if key_count % 50 == 0 {
            let manager = KeyManager::new().expect("Failed to create key manager");
            let config = KeyDerivationConfig {
                iterations: 5000, // Moderate iteration count
                salt: generate_random_bytes(32).expect("Salt generation failed"),
                key_length: 32,
            };
            
            let derived_key = manager.derive_key_pbkdf2(b"load_test_password", &config);
            assert!(derived_key.is_ok(), "Key derivation failed under load");
        }
        
        key_count += 1;
        total_key_bytes += key_size;
        *key_type_counts.entry(key_size).or_insert(0) += 1;
        
        if key_count % 1000 == 0 {
            let elapsed = start_time.elapsed();
            let rate = key_count as f64 / elapsed.as_secs_f64();
            tracing::debug!(keys_generated = key_count, rate_per_sec = rate, "Key generation progress");
        }
    }
    
    let final_duration = start_time.elapsed();
    let keys_per_sec = key_count as f64 / final_duration.as_secs_f64();
    let bytes_per_sec = total_key_bytes as f64 / final_duration.as_secs_f64();
    
    tracing::info!(
        duration_sec = final_duration.as_secs(),
        total_keys = key_count,
        total_key_bytes = total_key_bytes,
        keys_per_sec = keys_per_sec,
        bytes_per_sec = bytes_per_sec,
        key_type_distribution = ?key_type_counts,
        "Sustained key generation test completed"
    );
    
    // Should generate reasonable number of keys under sustained load
    assert!(keys_per_sec > 100.0, "Key generation rate too low: {:.1} keys/sec", keys_per_sec);
    assert!(key_count > 1000, "Should generate at least 1000 keys in 30 seconds");
}

#[test]
#[ignore] // Run with --ignored flag for stress tests  
fn test_sustained_crypto_load() {
    init_tracing!();
    tracing::info!("Testing sustained crypto load across all operations");
    
    let duration = Duration::from_secs(30); // 30 second sustained test
    let num_threads = 4;
    let barrier = Arc::new(Barrier::new(num_threads));
    
    let handles: Vec<_> = (0..num_threads).map(|thread_id| {
        let barrier = Arc::clone(&barrier);
        
        thread::spawn(move || {
            barrier.wait(); // Synchronized start
            
            let start_time = Instant::now();
            let mut operations = 0u64;
            let mut operation_counts = HashMap::new();
            
            // Different operation types per thread
            let thread_operations = match thread_id {
                0 => "encryption", // Thread 0: Focus on encryption
                1 => "hashing",    // Thread 1: Focus on hashing
                2 => "random",     // Thread 2: Focus on random generation
                3 => "mixed",      // Thread 3: Mixed operations
                _ => "mixed",
            };
            
            let key = vec![(thread_id as u8).wrapping_mul(42); 32];
            let cipher = Aes256Gcm::new(&key).expect("Failed to create cipher");
            let test_data = format!("Thread {} test data", thread_id).repeat(100);
            
            while start_time.elapsed() < duration {
                match thread_operations {
                    "encryption" => {
                        let encrypted = cipher.encrypt(test_data.as_bytes(), b"").unwrap();
                        let _decrypted = cipher.decrypt(&encrypted.ciphertext, b"", &encrypted).unwrap();
                        *operation_counts.entry("encrypt_decrypt").or_insert(0) += 1;
                    },
                    "hashing" => {
                        match operations % 3 {
                            0 => { let _h = Sha256::hash(test_data.as_bytes()); *operation_counts.entry("sha256").or_insert(0) += 1; },
                            1 => { let _h = Sha512::hash(test_data.as_bytes()); *operation_counts.entry("sha512").or_insert(0) += 1; },
                            2 => { let _h = Md5::hash(test_data.as_bytes()); *operation_counts.entry("md5").or_insert(0) += 1; },
                            _ => unreachable!(),
                        }
                    },
                    "random" => {
                        let size = 32 + (operations % 96) as usize; // 32-128 bytes
                        let _random = generate_random_bytes(size).unwrap();
                        *operation_counts.entry("random_gen").or_insert(0) += 1;
                    },
                    "mixed" => {
                        match operations % 4 {
                            0 => { 
                                let encrypted = cipher.encrypt(test_data.as_bytes(), b"").unwrap();
                                let _decrypted = cipher.decrypt(&encrypted.ciphertext, b"", &encrypted).unwrap();
                                *operation_counts.entry("mixed_encrypt").or_insert(0) += 1;
                            },
                            1 => { let _h = Sha256::hash(test_data.as_bytes()); *operation_counts.entry("mixed_hash").or_insert(0) += 1; },
                            2 => { let _r = generate_random_bytes(64).unwrap(); *operation_counts.entry("mixed_random").or_insert(0) += 1; },
                            3 => { 
                                let manager = KeyManager::new().unwrap();
                                let _k = manager.generate_key(32).unwrap(); 
                                *operation_counts.entry("mixed_keygen").or_insert(0) += 1;
                            },
                            _ => unreachable!(),
                        }
                    },
                    _ => unreachable!(),
                }
                
                operations += 1;
                
                if operations % 1000 == 0 {
                    let elapsed = start_time.elapsed();
                    let rate = operations as f64 / elapsed.as_secs_f64();
                    tracing::trace!(
                        thread_id = thread_id,
                        operations = operations,
                        rate_per_sec = rate,
                        "Sustained load progress"
                    );
                }
            }
            
            let final_duration = start_time.elapsed();
            let ops_per_sec = operations as f64 / final_duration.as_secs_f64();
            
            tracing::info!(
                thread_id = thread_id,
                thread_type = thread_operations,
                total_operations = operations,
                duration_sec = final_duration.as_secs(),
                ops_per_sec = ops_per_sec,
                operation_breakdown = ?operation_counts,
                "Thread completed sustained load test"
            );
            
            (operations, ops_per_sec)
        })
    }).collect();
    
    // Collect results from all threads
    let mut total_operations = 0u64;
    let mut total_ops_per_sec = 0.0;
    
    for handle in handles {
        let (ops, ops_per_sec) = handle.join().expect("Thread panicked");
        total_operations += ops;
        total_ops_per_sec += ops_per_sec;
    }
    
    tracing::info!(
        total_threads = num_threads,
        total_operations = total_operations,
        combined_ops_per_sec = total_ops_per_sec,
        duration_sec = 30,
        "Sustained crypto load test completed successfully"
    );
    
    // Performance assertions for sustained load
    assert!(total_operations > 50000, "Should complete at least 50k operations in 30 seconds");
    assert!(total_ops_per_sec > 2000.0, "Combined throughput should exceed 2000 ops/sec");
}
