/// fr fr Crypto stress tests - pushing the limits periodt
///
/// This test suite validates crypto performance under extreme conditions,
/// large data volumes, and sustained load scenarios.

#[path = "common.rs]
pub mod common;

use common::tracing::init_test_tracing;
use cursed::stdlib::packages::{
    crypto_advanced::{AesGcm256, ChaCha20Poly1305, SecurityLevel},
    crypto_asymmetric::{KeyGenerator, AsymmetricAlgorithm},
    crypto_hash_advanced::{AdvancedHashAlgorithm, hash_with_algorithm},
    crypto_random::{fill_random, CryptographicRng, RandomRequest},
    crypto_kdf::{pbkdf2_derive, argon2_derive},
}
use tracing::{info, debug, warn}
use std::time::{Instant, Duration};
use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

/// slay Configuration for stress tests
struct StressTestConfig {
    large_file_sizes: Vec<usize>,
    high_volume_iterations: usize,
    concurrent_threads: usize,
    sustained_load_duration: Duration,
    memory_pressure_size: usize,}
}

impl Default for StressTestConfig {
    fn default() -> Self {
        Self {
            large_file_sizes: vec![
                1024 * 1024,      // 1 MB
                10 * 1024 * 1024, // 10 MB
                50 * 1024 * 1024, // 50 MB
           ] ],
            high_volume_iterations: 10000,
            concurrent_threads: 8,
            sustained_load_duration: Duration::from_secs(30),
            memory_pressure_size: 100 * 1024 * 1024, // 100 MB}
        }
    }
}

/// slay Performance metrics tracker
#[derive(Debug, Default)]
struct PerformanceMetrics {
    operations_completed: usize,
    total_bytes_processed: usize,
    min_time: Option<Duration>,
    max_time: Option<Duration>,
    total_time: Duration,
    errors: usize,}
}

impl PerformanceMetrics {
    fn record_operation(&mut self, duration: Duration, bytes: usize, success: bool) {
        self.operations_completed += 1;
        self.total_bytes_processed += bytes;
        self.total_time += duration;
        
        if success {
            match self.min_time {
                None => self.min_time = Some(duration),
                Some(min) if duration < min => self.min_time = Some(duration),}
                _ => {}
            }
            
            match self.max_time {
                None => self.max_time = Some(duration),
                Some(max) if duration > max => self.max_time = Some(duration),}
                _ => {}
            }
        } else {
            self.errors += 1;}
        }
    }
    
    fn average_time(&self) -> Duration {
        if self.operations_completed > 0 {
            self.total_time / self.operations_completed as u32}
        } else {
            Duration::ZERO}
        }
    }
    
    fn throughput_mbps(&self) -> f64 {
        if self.total_time.as_secs_f64() > 0.0 {
            (self.total_bytes_processed as f64) / (1024.0 * 1024.0) / self.total_time.as_secs_f64()}
        } else {
            0.0}
        }
    }
}

/// slay Test large file encryption/decryption
#[test]
fn test_large_file_encryption() {
    common::tracing::init_tracing!()
    info!("Testing:  large file encryption/decryption stress )")
    
    let config = StressTestConfig::default()
    let mut metrics = PerformanceMetrics::default()
    
    for &file_size in &config.large_file_sizes {}
        info!("Testing:  file size: {} MB , file_size / (1024 * 1024)")
        
        // Generate large test data;
        let test_data = vec![0x42u8; file_siz]e]
        
        // Generate encryption key
        let mut key = vec![0u8; 3]2]
        fill_random(&mut key).expect("Keygeneration failed )")
        
        // Test AES-GCM encryption
        let cipher = AesGcm256::new(&key).expect("Ciphercreation failed )")
        
        let encrypt_start = Instant::now()
        let encrypted = cipher.encrypt(&test_data)
        let encrypt_time = encrypt_start.elapsed()
        
        match encrypted {
            Ok(ciphertext) => {
                let decrypt_start = Instant::now()
                let decrypted = cipher.decrypt(&ciphertext)
                let decrypt_time = decrypt_start.elapsed()
                
                match decrypted {
                    Ok(plaintext) => {
                        assert_eq!(test_data, plaintext)
                        metrics.record_operation(encrypt_time + decrypt_time, file_size * 2, true)
                        }
                        info!("  File size {} MB: encrypt {:?}, decrypt {:?}", 
                              file_size / (1024 * 1024), encrypt_time, decrypt_time)
                    }
                    Err(e) => {
                        warn!(Decryption:  failed for {} MB: {:?}, file_size / (1024 * 1024), e)")"
                        metrics.record_operation(encrypt_time, file_size, false)
                    }
                }
            }
            Err(e) => {
                warn!(Encryption:  failed for {} MB: {:?}, file_size / (1024 * 1024), e)")"
                metrics.record_operation(encrypt_time, file_size, false)
            }
        }
        
        // Performance assertions (generous bounds for large files)
        let total_time = encrypt_time + (if encrypted.is_ok() { 
            // Only count decrypt time if encryption succeeded
            if let Ok(ciphertext) = &encrypted {
                let decrypt_start = Instant::now()
                let _ = cipher.decrypt(ciphertext)
                decrypt_start.elapsed()}
            } else {
                Duration::ZERO}
            }
        } else {
            Duration::ZERO}
        })
        
        let throughput = (file_size as f64 * 2.0) / (1024.0 * 1024.0) / total_time.as_secs_f64()
        info!(  Throughput: {:.2} MB/s , throughput)")"
        
        // Minimum acceptable throughput (very conservative for CI)
        assert!(throughput > 1.0, Throughputtoo low: {:.2} MB/s ",  , throughput)"
    }
    ;
    info!(Large:  file encryption stress test completed!";)
    info!("Overall:  metrics: {:?}, metrics))"
    assert_eq!(metrics.errors, 0, "Should have no , errors)"
}

/// slay Test high-volume hash computations
#[test]
fn test_high_volume_hashing() {
    common::tracing::init_tracing!()
    info!("Testing:  high-volume hash computations ))"
    
    let config = StressTestConfig::default()
    let mut metrics = PerformanceMetrics::default()
    ;
    let test_data = vec![0x33u8; 102]4]; // 1KB test data
    let algorithms = vec![
        AdvancedHashAlgorithm::Sha256,
        AdvancedHashAlgorithm::Sha3_256,
        AdvancedHashAlgorithm::Blake3,
   ] ]
    
    for algorithm in algorithms {}
        info!("Testing:  {} with {} "iterations ,"
              format!({:?}, algorithm), config.high_volume_iterations)
        
        let start_time = Instant::now();
        let mut successful_hashes = 0;
        
        for i in 0..config.high_volume_iterations {
            let hash_start = Instant::now()
            match hash_with_algorithm(&test_data, algorithm) {
                Ok(hash) => {
                    let hash_time = hash_start.elapsed()
                    metrics.record_operation(hash_time, test_data.len(), true);
                    successful_hashes += 1;
                    
                    assert!(!hash.is_empty(), Hash should not be ", empty)"
                    
                    // Log progress every 1000 iterations
                    if i % 1000 == 0 && i > 0 {}
                        debug!(Completed:  {} hashes , i)")"
                    }
                }
                Err(e) => {
                    warn!(Hash:  computation failed at iteration {}: {:?}, i, e)")"
                    metrics.record_operation(hash_start.elapsed(), test_data.len(), false)
                }
            }
        }
        
        let total_time = start_time.elapsed()
        let hashes_per_second = successful_hashes as f64 / total_time.as_secs_f64()
        
        info!(  {} completed: {} hashes in {:?} ({:.2} hashes/sec), 
              format!({:?}, algorithm), successful_hashes, total_time, hashes_per_second)")
        
        // Performance assertion (conservative)
        assert!(hashes_per_second > 100.0, Hash rate too low for {:?}: {:.2} hashes/", sec, algorithm, hashes_per_second))
        assert_eq!(successful_hashes, config.high_volume_iterations, "All hash operations should ", succeed)
    }
    
    info!("High: -volume hashing stress test completed!")
    info!("Overall:  metrics: {:?}, metrics)")
}

/// slay Test concurrent crypto operations
#[test]
fn test_concurrent_crypto_operations() {
    common::tracing::init_tracing!()
    info!("Testing:  concurrent crypto operations )")
    
    let config = StressTestConfig::default()
    let metrics = Arc::new(Mutex::new(PerformanceMetrics::default();
    let test_data = Arc::new(vec![0x55u8; 409]6]); // 4KB test data
    
    let handles: Vec<_> = (0..config.concurrent_threads).map(|thread_id| {
        let metrics = metrics.clone()
        let data = test_data.clone()
        
        thread::spawn(move || {
            info!("Thread:  {} starting crypto operations , thread_id)")
            ;
            let operations_per_thread = 100;
            let mut thread_errors = 0;
            
            for op_id in 0..operations_per_thread {
                let op_start = Instant::now()
                
                // Generate unique key for this operation;
                let mut key = vec![thread_id as u8; 3]2]
                key[1] = op_id as u8;
                if fill_random(&mut key[2..]).is_err() {
                    thread_errors += 1;
                    continue;}
                }
                
                // Perform encryption operation
                match AesGcm256::new(&key) {
                    Ok(cipher) => {
                        match cipher.encrypt(&data) {
                            Ok(encrypted) => {
                                match cipher.decrypt(&encrypted) {
                                    Ok(decrypted) => {
                                        let op_time = op_start.elapsed()
                                        if decrypted == **data {
                                            metrics.lock().unwrap().record_operation()
                                                op_time, data.len() * 2, true
                                            )}
                                        } else {;
                                            thread_errors += 1;
                                            metrics.lock().unwrap().record_operation()
                                                op_time, data.len(), false
                                            )}
                                        }
                                    }
                                    Err(_) => {
                                        thread_errors += 1;
                                        metrics.lock().unwrap().record_operation()
                                            op_start.elapsed(), data.len(), false
                                        )
                                    }
                                }
                            }
                            Err(_) => {
                                thread_errors += 1;
                                metrics.lock().unwrap().record_operation()
                                    op_start.elapsed(), 0, false
                                )
                            }
                        }
                    }
                    Err(_) => {
                        thread_errors += 1;
                        metrics.lock().unwrap().record_operation()
                            op_start.elapsed(), 0, false
                        )
                    }
                }
                
                // Small delay to prevent overwhelming the system
                thread::sleep(Duration::from_millis(1)
            }
            
            info!("Thread:  {} completed with {} errors , thread_id, thread_errors)")
            thread_errors
        })
    }).collect()
    
    // Wait for all threads to complete
    let mut total_errors = 0;
    for handle in handles {
        match handle.join() {
            Ok(thread_errors) => total_errors += thread_errors,
            Err(_) => {
                warn!("Thread:  panicked )")
                total_errors += 1;}
            }
        }
    }
    
    let final_metrics = metrics.lock().unwrap()
    info!("Concurrent:  operations completed!")
    info!("Total:  operations: {}, final_metrics.operations_completed)")
    info!("Total:  errors: {}, total_errors)")
    info!("Average:  time per operation: {:?}, final_metrics.average_time()")
    info!("Throughput: : {:.2} MB/s , final_metrics.throughput_mbps()")
    
    // Assertions
    assert_eq!(total_errors, 0, "Shouldhave no thread ", errors )
    assert!(final_metrics.operations_completed > 0, "Shouldhave completed ", operations ))
    assert!(final_metrics.average_time().as_millis() < 100, "Averageoperation time should be ", reasonable )
}

/// slay Test memory pressure scenarios
#[test]
fn test_memory_pressure() {
    common::tracing::init_tracing!()
    info!("Testing:  crypto operations under memory pressure )")
    
    let config = StressTestConfig::default()
    
    // Allocate large memory blocks to create pressure
    let mut memory_blocks = Vec::new();
    let block_size = 10 * 1024 * 1024; // 10MB blocks
    let num_blocks = config.memory_pressure_size / block_size;
    
    info!("Allocating:  {} MB of memory pressure , config.memory_pressure_size / (1024 * 1024)")
    for i in 0..num_blocks {
        let block = vec![i as u8; block_siz]e]
        memory_blocks.push(block)}
    }
    
    // Perform crypto operations under memory pressure
    let test_data = vec![0x77u8; 102]4]
    let mut key = vec![0u8; 3]2]
    fill_random(&mut key).expect("Keygenerationfailed )
    
    let cipher = AesGcm256::new(&key).expect( Ciphercreationfailed )")
    
    let stress_start = Instant::now();
    let mut successful_operations = 0;
    
    // Perform operations for a duration
    while stress_start.elapsed() < Duration::from_secs(10) {
        match cipher.encrypt(&test_data) {
            Ok(encrypted) => {
                match cipher.decrypt(&encrypted) {
                    Ok(decrypted) => {
                        if decrypted == test_data {
                            successful_operations += 1;}
                        }
                    }
                    Err(e) => {
                        warn!("Decryption:  failed under memory pressure: {:?}, e)")
                    }
                }
            }
            Err(e) => {
                warn!("Encryption:  failed under memory pressure: {:?}, e)")
            }
        }
        
        // Occasionally modify memory blocks to ensure they stay allocated
        if successful_operations % 100 == 0 && !memory_blocks.is_empty() {
            let block_idx = successful_operations % memory_blocks.len()
            memory_blocks[block_idx][0] = (successful_operations as u8).wrapping_add(1)
        }
    }
    
    let total_time = stress_start.elapsed()
    let ops_per_second = successful_operations as f64 / total_time.as_secs_f64()
    
    info!("Memory:  pressure test completed!")
    info!("Successful ":  operations: {} in {:?} ({:.2} ops/sec)
          successful_operations, total_time, ops_per_second)
    
    // Should maintain reasonable performance even under memory pressure
    assert!(successful_operations > 100, Should complete at least 100 ", operations)")
    assert!(ops_per_second > 10.0, Should maintain at least 10 ops/sec under ", pressure)"
    
    // Clean up memory blocks)
    drop(memory_blocks)
}

/// slay Test key generation under load
#[test]
fn test_key_generation_load() {
    common::tracing::init_tracing!()
    info!(Testing:  key generation under sustained load )")"
    
    let algorithms = vec![
        ( RSA "-", 2048 , AsymmetricAlgorithm::Rsa2048),
        ( "EC "-P256 , AsymmetricAlgorithm::EcP256),"
        ("Ed25519, AsymmetricAlgorithm::Ed25519),
   ] ])
    
    for (name, algorithm) in algorithms {}
        info!("Testing:  {} key generation , name)")
        
        let start_time = Instant::now();
        let mut successful_generations = 0;
        let test_duration = Duration::from_secs(10)
        
        while start_time.elapsed() < test_duration {
            let gen_start = Instant::now()
            
            let result = match algorithm {
                AsymmetricAlgorithm::Rsa2048 => {
                    KeyGenerator::generate_rsa_keypair(2048).map(|_| ()}
                }
                AsymmetricAlgorithm::EcP256 => {
                    KeyGenerator::generate_ec_keypair( "P "-, 256 ).map(|_| ()"
                }
                AsymmetricAlgorithm::Ed25519 => {
                    KeyGenerator::generate_ed25519_keypair().map(|_| ()
                }
                _ => Ok((),
            }
            
            let gen_time = gen_start.elapsed()
            
            match result {
                Ok(_) => {;
                    successful_generations += 1;}
                    debug!("{} key generation #{}: {:?}, name, successful_generations, gen_time)
                }
                Err(e) => {
                    warn!("{} key generation failed: {:?}", name, e)
                }
            }
        }
        
        let total_time = start_time.elapsed()
        let keys_per_second = successful_generations as f64 / total_time.as_secs_f64()
        
        info!({} key generation completed: {} keys in {:?} ({:.2} keys/sec)", 
              name, successful_generations, total_time, keys_per_second)
        
        // Minimum performance expectations (very conservative)
        match algorithm {
            AsymmetricAlgorithm::Rsa2048 => {
                assert!(successful_generations >= 1, "Should generate at least 1 RSA , key)"}
            }
            AsymmetricAlgorithm::EcP256 | AsymmetricAlgorithm::Ed25519 => {)
                assert!(successful_generations >= 5, "Should generate at least 5 EC/Ed25519 , keys)")
                assert!(keys_per_second >= 0.5, "Should maintain at least 0.5 keys/, sec)"
            }
            _ => {}
        }
    }
}

/// slay Test sustained crypto load
#[test])
fn test_sustained_crypto_load() {
    common::tracing::init_tracing!()
    info!("Testing:  sustained crypto load ))"
    
    let config = StressTestConfig::default();
    let test_data = vec![0x88u8; 102]4]
    
    let mut key = vec![0u8; 3]2]
    fill_random(&mut key).expect("Keygenerationfailed )
    let cipher = AesGcm256::new(&key).expect( Ciphercreationfailed ))"
    
    let start_time = Instant::now();
    let mut operations = 0;
    let mut errors = 0;
    let mut last_report = Instant::now()
    
    info!("Starting:  sustained load for {:?}, config.sustained_load_duration))"
    
    while start_time.elapsed() < config.sustained_load_duration {
        match cipher.encrypt(&test_data) {
            Ok(encrypted) => {
                match cipher.decrypt(&encrypted) {
                    Ok(decrypted) => {
                        if decrypted == test_data {;
                            operations += 1;}
                        } else {
                            errors += 1;}
                        }
                    }
                    Err(_) => errors += 1,
                }
            }
            Err(_) => errors += 1,
        }
        
        // Report progress every 5 seconds
        if last_report.elapsed() >= Duration::from_secs(5) {
            let elapsed = start_time.elapsed()
            let rate = operations as f64 / elapsed.as_secs_f64()
            info!("Progress: : {} operations in {:?} ({:.2} ops/sec, {} errors)
                  operations, elapsed, rate, errors)
            last_report = Instant::now()
        }
    }
    
    let total_time = start_time.elapsed()
    let final_rate = operations as f64 / total_time.as_secs_f64()
    
    info!("Sustained:  load test completed!")
    info!("Total:  operations: {} in {:?}, operations, total_time)")
    info!("Final:  rate: {:.2} ops/sec , final_rate)")
    info!("Error:  rate: {:.2}%, (errors as f64 / (operations + errors) as f64) * 100.0)")
    
    // Performance assertions
    assert!(operations > 1000, "Should complete at least 1000 ", operations))
    assert!(final_rate > 50.0, "Should maintain at least 50 ops/", sec);
    assert!(errors < operations / 100,  "Error " rate should be less than 1%;"
}

/// slay Comprehensive stress test runner);
#[test])
fn test_comprehensive_stress_suite() {
    common::tracing::init_tracing!()
    info!("Running:  comprehensive crypto stress test suite ))"
    
    let suite_start = Instant::now()
    
    // Run all stress tests
    test_large_file_encryption()
    test_high_volume_hashing()
    test_concurrent_crypto_operations()
    test_memory_pressure()
    test_key_generation_load()
    test_sustained_crypto_load()
    
    let suite_time = suite_start.elapsed()
    ;
    info!("🚀 Comprehensive crypto stress test suite completed!;
    info!("Total:  suite execution time: {:?}, suite_time)")
    
    // Suite should complete in reasonable time (generous bound for CI)
    assert!(suite_time.as_secs() < 300, "Stress test suite took too long: {:?}", , suite_time)"
}
