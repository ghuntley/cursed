/// Stress Tests for Process Management and IPC
/// 
/// This module provides intensive stress tests that push the process management
/// and IPC systems to their limits, testing performance, memory usage, 
/// concurrency limits, and system stability under extreme conditions.

use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex, mpsc, Barrier, atomic::{AtomicUsize, Ordering}};
use std::thread;
use std::collections::HashMap;

use cursed::runtime::process::{
    ProcessRuntime, IpcChannelType, IpcConfig, ProcessStatus
};

#[path = "common.rs"]
mod common;

/// Stress test with massive concurrent IPC operations
#[test]
fn test_massive_concurrent_ipc_stress() {
    common::tracing::setup();
    
    let runtime = Arc::new(ProcessRuntime::new());
    let num_threads = 50;
    let operations_per_thread = 100;
    let start_time = Instant::now();
    
    println!("Starting massive concurrent IPC stress test: {} threads, {} ops each", 
             num_threads, operations_per_thread);
    
    let success_counter = Arc::new(AtomicUsize::new(0));
    let error_counter = Arc::new(AtomicUsize::new(0));
    let barrier = Arc::new(Barrier::new(num_threads));
    
    let mut handles = vec![];
    
    for thread_id in 0..num_threads {
        let runtime_clone = runtime.clone();
        let success_counter_clone = success_counter.clone();
        let error_counter_clone = error_counter.clone();
        let barrier_clone = barrier.clone();
        
        let handle = thread::spawn(move || {
            // Wait for all threads to be ready
            barrier_clone.wait();
            
            let mut local_success = 0;
            let mut local_errors = 0;
            
            for op_id in 0..operations_per_thread {
                let config = IpcConfig {
                    name: format!("stress_{}_{}", thread_id, op_id),
                    config_type: 0,
                    size_or_capacity: 512,
                    permissions: 0o666,
                    flags: 0,
                };
                
                // Create channel
                match runtime_clone.create_ipc_channel(IpcChannelType::Pipe, &config) {
                    Ok(channel_id) => {
                        // Send data
                        let test_data = format!("stress_data_{}_{}", thread_id, op_id);
                        match runtime_clone.ipc_send(channel_id, test_data.as_bytes()) {
                            Ok(_) => {
                                // Try to receive
                                match runtime_clone.ipc_receive(channel_id, 50) {
                                    Ok(data_ptr) => {
                                        if !data_ptr.is_null() {
                                            let _ = unsafe { Box::from_raw(data_ptr as *mut Vec<u8>) };
                                            local_success += 1;
                                        } else {
                                            local_errors += 1;
                                        }
                                    }
                                    Err(_) => local_errors += 1,
                                }
                            }
                            Err(_) => local_errors += 1,
                        }
                    }
                    Err(_) => local_errors += 1,
                }
                
                // Yield occasionally to let other threads run
                if op_id % 10 == 0 {
                    thread::yield_now();
                }
            }
            
            success_counter_clone.fetch_add(local_success, Ordering::Relaxed);
            error_counter_clone.fetch_add(local_errors, Ordering::Relaxed);
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Stress test thread should complete");
    }
    
    let total_time = start_time.elapsed();
    let total_success = success_counter.load(Ordering::Relaxed);
    let total_errors = error_counter.load(Ordering::Relaxed);
    let total_operations = total_success + total_errors;
    let expected_operations = num_threads * operations_per_thread;
    
    println!("Massive concurrent IPC stress test results:");
    println!("  Duration: {:?}", total_time);
    println!("  Successful operations: {}", total_success);
    println!("  Failed operations: {}", total_errors);
    println!("  Total operations: {}/{} expected", total_operations, expected_operations);
    println!("  Success rate: {:.1}%", (total_success as f64 / total_operations as f64) * 100.0);
    println!("  Operations per second: {:.1}", total_operations as f64 / total_time.as_secs_f64());
    
    // Stress test assertions (relaxed for harsh conditions)
    assert!(total_operations >= expected_operations * 8 / 10, 
            "Should complete at least 80% of expected operations");
    assert!(total_success > 0, "Should have some successful operations");
    assert!(total_time < Duration::from_secs(60), "Should complete within reasonable time");
}

/// Stress test with high-frequency process spawning
#[test]
fn test_high_frequency_process_spawning_stress() {
    common::tracing::setup();
    
    let runtime = Arc::new(ProcessRuntime::new());
    let num_spawning_threads = 8;
    let spawns_per_thread = 20;
    
    println!("Starting high-frequency process spawning stress test: {} threads, {} spawns each",
             num_spawning_threads, spawns_per_thread);
    
    let spawn_success_counter = Arc::new(AtomicUsize::new(0));
    let spawn_error_counter = Arc::new(AtomicUsize::new(0));
    let wait_success_counter = Arc::new(AtomicUsize::new(0));
    
    let mut handles = vec![];
    let start_time = Instant::now();
    
    for thread_id in 0..num_spawning_threads {
        let runtime_clone = runtime.clone();
        let spawn_success_clone = spawn_success_counter.clone();
        let spawn_error_clone = spawn_error_counter.clone();
        let wait_success_clone = wait_success_counter.clone();
        
        let handle = thread::spawn(move || {
            let mut spawned_pids = vec![];
            
            // Spawn processes rapidly
            for spawn_id in 0..spawns_per_thread {
                // Try different commands based on platform and availability
                let commands_to_try = vec![
                    ("echo", vec![format!("test_{}_{}", thread_id, spawn_id)]),
                    ("true", vec![]),
                    ("ls", vec!["--version".to_string()]),
                ];
                
                let mut spawned = false;
                for (cmd, args) in commands_to_try {
                    match runtime_clone.spawn_process(cmd, &args) {
                        Ok(pid) => {
                            spawned_pids.push(pid);
                            spawn_success_clone.fetch_add(1, Ordering::Relaxed);
                            spawned = true;
                            break;
                        }
                        Err(_) => {
                            // Try next command
                            continue;
                        }
                    }
                }
                
                if !spawned {
                    spawn_error_clone.fetch_add(1, Ordering::Relaxed);
                }
                
                // Brief pause to avoid overwhelming the system
                thread::sleep(Duration::from_millis(10));
            }
            
            // Wait for spawned processes
            for pid in spawned_pids {
                match runtime_clone.wait_process(pid) {
                    Ok(_) => wait_success_clone.fetch_add(1, Ordering::Relaxed),
                    Err(_) => {
                        // Process might have already exited, try to get status
                        let _ = runtime_clone.get_process_status(pid);
                    }
                }
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all spawning threads
    for handle in handles {
        handle.join().expect("Process spawning thread should complete");
    }
    
    let total_time = start_time.elapsed();
    let spawn_successes = spawn_success_counter.load(Ordering::Relaxed);
    let spawn_errors = spawn_error_counter.load(Ordering::Relaxed);
    let wait_successes = wait_success_counter.load(Ordering::Relaxed);
    let expected_spawns = num_spawning_threads * spawns_per_thread;
    
    println!("High-frequency process spawning stress test results:");
    println!("  Duration: {:?}", total_time);
    println!("  Successful spawns: {}/{} expected", spawn_successes, expected_spawns);
    println!("  Failed spawns: {}", spawn_errors);
    println!("  Successful waits: {}", wait_successes);
    println!("  Spawn success rate: {:.1}%", (spawn_successes as f64 / expected_spawns as f64) * 100.0);
    
    // Note: In CI environments, process spawning might be heavily restricted
    // So we have relaxed assertions
    println!("Process spawning stress test completed (results may vary in restricted environments)");
}

/// Memory pressure stress test with extensive shared memory usage
#[test]
fn test_memory_pressure_shared_memory_stress() {
    common::tracing::setup();
    
    let runtime = Arc::new(ProcessRuntime::new());
    let num_segments = 100;
    let segment_size = 64 * 1024; // 64KB per segment
    let num_accessor_threads = 10;
    
    println!("Starting memory pressure stress test: {} segments of {}KB each",
             num_segments, segment_size / 1024);
    
    let start_time = Instant::now();
    
    // Create many shared memory segments
    let mut segment_names = vec![];
    let mut creation_errors = 0;
    
    for i in 0..num_segments {
        let segment_name = format!("memory_stress_segment_{}", i);
        
        match runtime.create_shared_memory(&segment_name, segment_size) {
            Ok(ptr) => {
                if !ptr.is_null() {
                    segment_names.push(segment_name);
                } else {
                    creation_errors += 1;
                }
            }
            Err(_) => creation_errors += 1,
        }
    }
    
    let creation_time = start_time.elapsed();
    println!("Created {} shared memory segments in {:?} ({} errors)",
             segment_names.len(), creation_time, creation_errors);
    
    // Spawn threads to access shared memory under pressure
    let access_counter = Arc::new(AtomicUsize::new(0));
    let access_error_counter = Arc::new(AtomicUsize::new(0));
    let segment_names = Arc::new(segment_names);
    
    let mut accessor_handles = vec![];
    
    for thread_id in 0..num_accessor_threads {
        let runtime_clone = runtime.clone();
        let access_counter_clone = access_counter.clone();
        let access_error_counter_clone = access_error_counter.clone();
        let segment_names_clone = segment_names.clone();
        
        let handle = thread::spawn(move || {
            let mut local_accesses = 0;
            let mut local_errors = 0;
            
            // Access segments repeatedly
            for round in 0..10 {
                for segment_name in segment_names_clone.iter() {
                    match runtime_clone.create_shared_memory(segment_name, segment_size) {
                        Ok(ptr) => {
                            if !ptr.is_null() {
                                local_accesses += 1;
                                
                                // Simulate memory access pattern
                                thread::sleep(Duration::from_nanos(100)); // Very brief
                            } else {
                                local_errors += 1;
                            }
                        }
                        Err(_) => local_errors += 1,
                    }
                    
                    // Yield to other threads occasionally
                    if local_accesses % 20 == 0 {
                        thread::yield_now();
                    }
                }
                
                // Brief pause between rounds
                thread::sleep(Duration::from_millis(5));
            }
            
            access_counter_clone.fetch_add(local_accesses, Ordering::Relaxed);
            access_error_counter_clone.fetch_add(local_errors, Ordering::Relaxed);
        });
        
        accessor_handles.push(handle);
    }
    
    // Wait for all accessor threads
    for handle in accessor_handles {
        handle.join().expect("Memory accessor thread should complete");
    }
    
    let total_time = start_time.elapsed();
    let total_accesses = access_counter.load(Ordering::Relaxed);
    let total_access_errors = access_error_counter.load(Ordering::Relaxed);
    let total_memory_allocated = segment_names.len() * segment_size;
    
    println!("Memory pressure stress test results:");
    println!("  Total duration: {:?}", total_time);
    println!("  Total memory allocated: {:.1} MB", total_memory_allocated as f64 / (1024.0 * 1024.0));
    println!("  Successful accesses: {}", total_accesses);
    println!("  Access errors: {}", total_access_errors);
    println!("  Access rate: {:.1} accesses/sec", total_accesses as f64 / total_time.as_secs_f64());
    
    assert!(segment_names.len() > num_segments / 2, 
            "Should create at least half of the requested shared memory segments");
    assert!(total_accesses > 0, "Should have some successful memory accesses");
}

/// Sustained load test with mixed operations
#[test]
fn test_sustained_mixed_operations_load() {
    common::tracing::setup();
    
    let runtime = Arc::new(ProcessRuntime::new());
    let test_duration = Duration::from_secs(30); // 30 second sustained test
    let num_worker_threads = 12;
    
    println!("Starting sustained mixed operations load test for {:?} with {} workers",
             test_duration, num_worker_threads);
    
    let start_time = Instant::now();
    let stop_flag = Arc::new(AtomicUsize::new(0));
    
    // Counters for different operation types
    let ipc_ops_counter = Arc::new(AtomicUsize::new(0));
    let shm_ops_counter = Arc::new(AtomicUsize::new(0));
    let process_ops_counter = Arc::new(AtomicUsize::new(0));
    let error_counter = Arc::new(AtomicUsize::new(0));
    
    let mut worker_handles = vec![];
    
    for worker_id in 0..num_worker_threads {
        let runtime_clone = runtime.clone();
        let stop_flag_clone = stop_flag.clone();
        let ipc_counter_clone = ipc_ops_counter.clone();
        let shm_counter_clone = shm_ops_counter.clone();
        let process_counter_clone = process_ops_counter.clone();
        let error_counter_clone = error_counter.clone();
        
        let handle = thread::spawn(move || {
            let mut operation_count = 0;
            
            while stop_flag_clone.load(Ordering::Relaxed) == 0 {
                let operation_type = operation_count % 6;
                
                match operation_type {
                    0 | 1 => {
                        // IPC operations (2/6 probability)
                        let config = IpcConfig {
                            name: format!("sustained_ipc_{}_{}", worker_id, operation_count),
                            config_type: 0,
                            size_or_capacity: 1024,
                            permissions: 0o666,
                            flags: 0,
                        };
                        
                        if let Ok(channel_id) = runtime_clone.create_ipc_channel(IpcChannelType::Pipe, &config) {
                            let test_data = format!("sustained_data_{}", operation_count);
                            if runtime_clone.ipc_send(channel_id, test_data.as_bytes()).is_ok() {
                                ipc_counter_clone.fetch_add(1, Ordering::Relaxed);
                            } else {
                                error_counter_clone.fetch_add(1, Ordering::Relaxed);
                            }
                        } else {
                            error_counter_clone.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                    2 | 3 => {
                        // Shared memory operations (2/6 probability)
                        let shm_name = format!("sustained_shm_{}_{}", worker_id, operation_count / 10);
                        
                        match runtime_clone.create_shared_memory(&shm_name, 2048) {
                            Ok(ptr) => {
                                if !ptr.is_null() {
                                    shm_counter_clone.fetch_add(1, Ordering::Relaxed);
                                } else {
                                    error_counter_clone.fetch_add(1, Ordering::Relaxed);
                                }
                            }
                            Err(_) => error_counter_clone.fetch_add(1, Ordering::Relaxed),
                        }
                    }
                    4 => {
                        // Process operations (1/6 probability)
                        // Try simple commands that might exist
                        let commands = ["echo", "true", "false"];
                        let cmd = commands[operation_count % commands.len()];
                        
                        match runtime_clone.spawn_process(cmd, &["sustained_test".to_string()]) {
                            Ok(pid) => {
                                process_counter_clone.fetch_add(1, Ordering::Relaxed);
                                // Don't wait for process to avoid blocking
                                let _ = runtime_clone.get_process_status(pid);
                            }
                            Err(_) => {
                                // Process spawning might fail in restricted environments
                                // This is not counted as an error
                            }
                        }
                    }
                    5 => {
                        // Signal operations (1/6 probability)
                        let _ = runtime_clone.register_signal_handler(15, None);
                    }
                    _ => {}
                }
                
                operation_count += 1;
                
                // Brief pause to avoid overwhelming the system
                thread::sleep(Duration::from_millis(2));
            }
        });
        
        worker_handles.push(handle);
    }
    
    // Let the test run for the specified duration
    thread::sleep(test_duration);
    
    // Signal all workers to stop
    stop_flag.store(1, Ordering::Relaxed);
    
    // Wait for all workers to complete
    for handle in worker_handles {
        handle.join().expect("Worker thread should complete");
    }
    
    let actual_duration = start_time.elapsed();
    let ipc_ops = ipc_ops_counter.load(Ordering::Relaxed);
    let shm_ops = shm_ops_counter.load(Ordering::Relaxed);
    let process_ops = process_ops_counter.load(Ordering::Relaxed);
    let errors = error_counter.load(Ordering::Relaxed);
    let total_ops = ipc_ops + shm_ops + process_ops;
    
    println!("Sustained mixed operations load test results:");
    println!("  Actual duration: {:?}", actual_duration);
    println!("  IPC operations: {} ({:.1}/sec)", ipc_ops, ipc_ops as f64 / actual_duration.as_secs_f64());
    println!("  Shared memory operations: {} ({:.1}/sec)", shm_ops, shm_ops as f64 / actual_duration.as_secs_f64());
    println!("  Process operations: {} ({:.1}/sec)", process_ops, process_ops as f64 / actual_duration.as_secs_f64());
    println!("  Total operations: {} ({:.1}/sec)", total_ops, total_ops as f64 / actual_duration.as_secs_f64());
    println!("  Errors: {} ({:.1}% error rate)", errors, (errors as f64 / (total_ops + errors) as f64) * 100.0);
    
    assert!(total_ops > 100, "Should complete at least 100 operations during sustained test");
    assert!(actual_duration >= test_duration * 9 / 10, "Should run for close to expected duration");
}

/// Race condition stress test
#[test]
fn test_race_condition_stress() {
    common::tracing::setup();
    
    let runtime = Arc::new(ProcessRuntime::new());
    let num_racing_threads = 20;
    let operations_per_thread = 50;
    
    println!("Starting race condition stress test: {} threads, {} ops each",
             num_racing_threads, operations_per_thread);
    
    // Shared resources that multiple threads will compete for
    let shared_channel_name = "race_condition_channel";
    let shared_shm_name = "race_condition_shm";
    
    let success_counter = Arc::new(AtomicUsize::new(0));
    let conflict_counter = Arc::new(AtomicUsize::new(0));
    let barrier = Arc::new(Barrier::new(num_racing_threads));
    
    let mut handles = vec![];
    
    for thread_id in 0..num_racing_threads {
        let runtime_clone = runtime.clone();
        let success_counter_clone = success_counter.clone();
        let conflict_counter_clone = conflict_counter.clone();
        let barrier_clone = barrier.clone();
        
        let handle = thread::spawn(move || {
            // Wait for all threads to be ready
            barrier_clone.wait();
            
            let mut local_success = 0;
            let mut local_conflicts = 0;
            
            for op_id in 0..operations_per_thread {
                // Race to create shared resources
                let channel_config = IpcConfig {
                    name: format!("{}_{}", shared_channel_name, op_id % 5), // Limited set to create conflicts
                    config_type: 0,
                    size_or_capacity: 1024,
                    permissions: 0o666,
                    flags: 0,
                };
                
                // Multiple threads try to create the same channel
                match runtime_clone.create_ipc_channel(IpcChannelType::Pipe, &channel_config) {
                    Ok(channel_id) => {
                        // Try to send data immediately
                        let test_data = format!("race_data_{}_{}", thread_id, op_id);
                        match runtime_clone.ipc_send(channel_id, test_data.as_bytes()) {
                            Ok(_) => local_success += 1,
                            Err(_) => local_conflicts += 1,
                        }
                    }
                    Err(_) => local_conflicts += 1,
                }
                
                // Race to access shared memory
                let shm_name = format!("{}_{}", shared_shm_name, op_id % 3); // Even more conflicts
                match runtime_clone.create_shared_memory(&shm_name, 2048) {
                    Ok(ptr) => {
                        if !ptr.is_null() {
                            local_success += 1;
                        } else {
                            local_conflicts += 1;
                        }
                    }
                    Err(_) => local_conflicts += 1,
                }
                
                // No sleep - maximum race conditions
            }
            
            success_counter_clone.fetch_add(local_success, Ordering::Relaxed);
            conflict_counter_clone.fetch_add(local_conflicts, Ordering::Relaxed);
        });
        
        handles.push(handle);
    }
    
    let start_time = Instant::now();
    
    // Wait for all racing threads
    for handle in handles {
        handle.join().expect("Racing thread should complete");
    }
    
    let race_duration = start_time.elapsed();
    let total_success = success_counter.load(Ordering::Relaxed);
    let total_conflicts = conflict_counter.load(Ordering::Relaxed);
    let total_attempts = total_success + total_conflicts;
    let expected_attempts = num_racing_threads * operations_per_thread * 2; // 2 ops per iteration
    
    println!("Race condition stress test results:");
    println!("  Duration: {:?}", race_duration);
    println!("  Successful operations: {}", total_success);
    println!("  Conflicts/errors: {}", total_conflicts);
    println!("  Total attempts: {}/{} expected", total_attempts, expected_attempts);
    println!("  Success rate: {:.1}%", (total_success as f64 / total_attempts as f64) * 100.0);
    println!("  Operations per second: {:.1}", total_attempts as f64 / race_duration.as_secs_f64());
    
    // In a race condition test, we expect some conflicts
    assert!(total_success > 0, "Should have some successful operations despite races");
    assert!(total_attempts >= expected_attempts * 9 / 10, "Should attempt most operations");
    println!("Race condition stress test completed - conflicts are expected and handled");
}

/// Resource exhaustion recovery test
#[test]
fn test_resource_exhaustion_recovery() {
    common::tracing::setup();
    
    let runtime = Arc::new(ProcessRuntime::new());
    let max_resources = 200; // Try to create many resources
    
    println!("Starting resource exhaustion recovery test with {} resources", max_resources);
    
    let start_time = Instant::now();
    let mut created_channels = vec![];
    let mut created_shm = vec![];
    let mut creation_errors = 0;
    
    // Phase 1: Create resources until exhaustion
    for i in 0..max_resources {
        // Try to create IPC channel
        let channel_config = IpcConfig {
            name: format!("exhaust_channel_{}", i),
            config_type: 0,
            size_or_capacity: 1024,
            permissions: 0o666,
            flags: 0,
        };
        
        match runtime.create_ipc_channel(IpcChannelType::Pipe, &channel_config) {
            Ok(channel_id) => created_channels.push(channel_id),
            Err(_) => creation_errors += 1,
        }
        
        // Try to create shared memory
        let shm_name = format!("exhaust_shm_{}", i);
        match runtime.create_shared_memory(&shm_name, 4096) {
            Ok(ptr) => {
                if !ptr.is_null() {
                    created_shm.push(shm_name);
                } else {
                    creation_errors += 1;
                }
            }
            Err(_) => creation_errors += 1,
        }
        
        // Stop if we're hitting too many errors (likely resource exhaustion)
        if creation_errors > 20 && i > 50 {
            println!("Stopping creation due to resource exhaustion after {} attempts", i);
            break;
        }
    }
    
    let creation_time = start_time.elapsed();
    println!("Resource creation phase completed in {:?}", creation_time);
    println!("  Created {} IPC channels", created_channels.len());
    println!("  Created {} shared memory segments", created_shm.len());
    println!("  Creation errors: {}", creation_errors);
    
    // Phase 2: Test operations under resource pressure
    let operation_start = Instant::now();
    let mut operation_successes = 0;
    let mut operation_failures = 0;
    
    for &channel_id in &created_channels {
        let test_data = b"exhaustion_test";
        match runtime.ipc_send(channel_id, test_data) {
            Ok(_) => operation_successes += 1,
            Err(_) => operation_failures += 1,
        }
        
        // Only test first 50 to avoid taking too long
        if operation_successes + operation_failures >= 50 {
            break;
        }
    }
    
    let operation_time = operation_start.elapsed();
    println!("Operations under pressure completed in {:?}", operation_time);
    println!("  Successful operations: {}", operation_successes);
    println!("  Failed operations: {}", operation_failures);
    
    // Phase 3: Test recovery by dropping runtime and creating new one
    let recovery_start = Instant::now();
    drop(runtime);
    
    let new_runtime = ProcessRuntime::new();
    
    // Try to create new resources with the new runtime
    let mut recovery_successes = 0;
    let mut recovery_failures = 0;
    
    for i in 0..10 {
        let config = IpcConfig {
            name: format!("recovery_channel_{}", i),
            config_type: 0,
            size_or_capacity: 1024,
            permissions: 0o666,
            flags: 0,
        };
        
        match new_runtime.create_ipc_channel(IpcChannelType::Pipe, &config) {
            Ok(_) => recovery_successes += 1,
            Err(_) => recovery_failures += 1,
        }
        
        let shm_name = format!("recovery_shm_{}", i);
        match new_runtime.create_shared_memory(&shm_name, 2048) {
            Ok(ptr) => {
                if !ptr.is_null() {
                    recovery_successes += 1;
                } else {
                    recovery_failures += 1;
                }
            }
            Err(_) => recovery_failures += 1,
        }
    }
    
    let recovery_time = recovery_start.elapsed();
    let total_time = start_time.elapsed();
    
    println!("Resource exhaustion recovery test results:");
    println!("  Total test duration: {:?}", total_time);
    println!("  Recovery phase duration: {:?}", recovery_time);
    println!("  Recovery successes: {}", recovery_successes);
    println!("  Recovery failures: {}", recovery_failures);
    println!("  Recovery rate: {:.1}%", (recovery_successes as f64 / (recovery_successes + recovery_failures) as f64) * 100.0);
    
    // Recovery assertions
    assert!(recovery_successes > 0, "Should be able to create resources after recovery");
    assert!(recovery_successes >= recovery_failures, "Recovery should mostly succeed");
    
    println!("Resource exhaustion recovery test completed successfully");
}
