/// Stress tests for CURSED process management and IPC systems
/// 
/// This test suite validates system behavior under extreme conditions including:
/// - High volume process spawning and management
/// - Concurrent IPC operations under load
/// - Resource exhaustion scenarios
/// - Memory pressure testing
/// - Performance degradation analysis
/// - System stability validation

#[path = "common.rs"]
pub mod common;

use cursed::stdlib::process::{
    ProcessConfig, spawn_process, run_command, get_process_info, get_process_usage,
    get_system_info, get_load_average, ProcessError, ProcessResult
};

use cursed::stdlib::ipc::{
    SharedMemory, SharedMemoryConfig, NamedPipe, PipeMode, MessageQueue,
    Message, MessagePriority, Semaphore, SemaphoreConfig,
    initialize, shutdown, get_ipc_statistics
};

use std::sync::{Arc, Mutex, Barrier};
use std::thread;
use std::time::{Duration, Instant, SystemTime};
use std::collections::HashMap;

/// Initialize tracing for tests
macro_rules! init_tracing {
    () => {
        common::tracing::setup();
    };
}

#[test]
#[ignore = "stress test - run with cargo test -- --ignored"]
fn test_massive_process_spawning() {
    init_tracing!();
    
    let timer = common::timing::Timer::new("massive_process_spawning");
    
    let num_processes = 100;
    let batch_size = 10;
    let mut total_success = 0;
    let mut total_failures = 0;
    
    tracing::info!("Starting massive process spawning test: {} processes in batches of {}", 
                  num_processes, batch_size);
    
    for batch in 0..(num_processes / batch_size) {
        let batch_start = Instant::now();
        let mut batch_handles = Vec::new();
        
        // Spawn batch of processes
        for i in 0..batch_size {
            let process_id = batch * batch_size + i;
            let handle = thread::spawn(move || {
                let config = ProcessConfig::new("echo")
                    .arg(&format!("batch_{}_process_{}", batch, i))
                    .timeout(Duration::from_secs(10));
                
                let result = run_command(config);
                (process_id, result.is_ok(), result.err())
            });
            batch_handles.push(handle);
        }
        
        // Collect batch results
        for handle in batch_handles {
            let (process_id, success, error) = handle.join().expect("Thread should complete");
            if success {
                total_success += 1;
            } else {
                total_failures += 1;
                if let Some(err) = error {
                    tracing::warn!("Process {} failed: {:?}", process_id, err);
                }
            }
        }
        
        let batch_duration = batch_start.elapsed();
        tracing::debug!("Batch {} completed in {:?} - {} successes", 
                       batch, batch_duration, batch_size);
        
        // Brief pause between batches to prevent system overload
        thread::sleep(Duration::from_millis(100));
    }
    
    let success_rate = (total_success as f64 / num_processes as f64) * 100.0;
    
    tracing::info!("Massive process spawning results: {}/{} succeeded ({:.1}%)", 
                  total_success, num_processes, success_rate);
    
    // Should have high success rate even under stress
    assert!(success_rate >= 85.0, 
           "Success rate should be at least 85% under stress, got {:.1}%", success_rate);
    
    drop(timer);
    
    tracing::info!("✅ Massive process spawning stress test passed");
}

#[test]
#[ignore = "stress test - run with cargo test -- --ignored"]
fn test_concurrent_ipc_operations() {
    init_tracing!();
    
    let timer = common::timing::Timer::new("concurrent_ipc_operations");
    
    let _init = initialize();
    
    let num_threads = 16;
    let operations_per_thread = 50;
    let barrier = Arc::new(Barrier::new(num_threads));
    
    tracing::info!("Starting concurrent IPC operations: {} threads × {} ops", 
                  num_threads, operations_per_thread);
    
    let handles: Vec<_> = (0..num_threads)
        .map(|thread_id| {
            let barrier = barrier.clone();
            thread::spawn(move || {
                barrier.wait(); // Synchronize start
                
                let mut results = Vec::new();
                
                for op_id in 0..operations_per_thread {
                    let operation_start = Instant::now();
                    
                    // Test shared memory operations
                    let shm_result = test_shared_memory_stress(thread_id, op_id);
                    
                    // Test message queue operations
                    let mq_result = test_message_queue_stress(thread_id, op_id);
                    
                    // Test semaphore operations
                    let sem_result = test_semaphore_stress(thread_id, op_id);
                    
                    let operation_duration = operation_start.elapsed();
                    
                    results.push((
                        thread_id,
                        op_id,
                        shm_result.is_ok(),
                        mq_result.is_ok(),
                        sem_result.is_ok(),
                        operation_duration
                    ));
                    
                    // Brief pause to prevent overwhelming the system
                    if op_id % 10 == 0 {
                        thread::sleep(Duration::from_millis(1));
                    }
                }
                
                results
            })
        })
        .collect();
    
    // Collect all results
    let mut total_operations = 0;
    let mut successful_shm = 0;
    let mut successful_mq = 0;
    let mut successful_sem = 0;
    let mut total_duration = Duration::from_secs(0);
    
    for handle in handles {
        let results = handle.join().expect("Thread should complete");
        for (thread_id, op_id, shm_ok, mq_ok, sem_ok, duration) in results {
            total_operations += 1;
            if shm_ok { successful_shm += 1; }
            if mq_ok { successful_mq += 1; }
            if sem_ok { successful_sem += 1; }
            total_duration += duration;
            
            if !shm_ok || !mq_ok || !sem_ok {
                tracing::warn!("Thread {} op {} had failures: SHM={}, MQ={}, SEM={}", 
                              thread_id, op_id, shm_ok, mq_ok, sem_ok);
            }
        }
    }
    
    let avg_operation_time = total_duration / total_operations;
    let shm_success_rate = (successful_shm as f64 / total_operations as f64) * 100.0;
    let mq_success_rate = (successful_mq as f64 / total_operations as f64) * 100.0;
    let sem_success_rate = (successful_sem as f64 / total_operations as f64) * 100.0;
    
    tracing::info!("Concurrent IPC results:");
    tracing::info!("  Shared Memory: {}/{} ({:.1}%)", successful_shm, total_operations, shm_success_rate);
    tracing::info!("  Message Queue: {}/{} ({:.1}%)", successful_mq, total_operations, mq_success_rate);
    tracing::info!("  Semaphore: {}/{} ({:.1}%)", successful_sem, total_operations, sem_success_rate);
    tracing::info!("  Average operation time: {:?}", avg_operation_time);
    
    // Should maintain high success rates under concurrent load
    assert!(shm_success_rate >= 90.0, "Shared memory success rate should be >= 90%");
    assert!(mq_success_rate >= 90.0, "Message queue success rate should be >= 90%");
    assert!(sem_success_rate >= 90.0, "Semaphore success rate should be >= 90%");
    
    // Operations should complete in reasonable time
    assert!(avg_operation_time < Duration::from_millis(100), 
           "Average operation time should be < 100ms");
    
    let _shutdown = shutdown();
    drop(timer);
    
    tracing::info!("✅ Concurrent IPC operations stress test passed");
}

fn test_shared_memory_stress(thread_id: usize, op_id: usize) -> Result<(), Box<dyn std::error::Error>> {
    let name = format!("stress_shm_{}_{}", thread_id, op_id);
    let config = SharedMemoryConfig::new(&name, 1024).with_remove_on_drop();
    
    let mut shm = SharedMemory::create(config)?;
    
    let test_data = format!("Thread {} Operation {} Data", thread_id, op_id);
    shm.write_bytes(test_data.as_bytes())?;
    
    let mut read_buffer = vec![0u8; test_data.len()];
    let bytes_read = shm.read_bytes(&mut read_buffer, 0)?;
    
    if bytes_read != test_data.len() {
        return Err("Read length mismatch".into());
    }
    
    let read_data = String::from_utf8(read_buffer)?;
    if read_data != test_data {
        return Err("Data mismatch".into());
    }
    
    Ok(())
}

fn test_message_queue_stress(thread_id: usize, op_id: usize) -> Result<(), Box<dyn std::error::Error>> {
    let name = format!("stress_mq_{}_{}", thread_id, op_id);
    let mq = MessageQueue::create(&name, 5)?;
    
    let test_message = format!("Thread {} Op {}", thread_id, op_id);
    let msg = Message::new(&test_message, MessagePriority::Normal)?;
    
    mq.send(msg)?;
    
    let received = mq.receive()?;
    if received.data() != test_message {
        return Err("Message data mismatch".into());
    }
    
    Ok(())
}

fn test_semaphore_stress(thread_id: usize, op_id: usize) -> Result<(), Box<dyn std::error::Error>> {
    let name = format!("stress_sem_{}_{}", thread_id, op_id);
    let config = SemaphoreConfig::new(&name, 1, 1);
    let semaphore = Semaphore::create(config)?;
    
    semaphore.acquire()?;
    
    // Verify we have the lock
    let value = semaphore.get_value()?;
    if value != 0 {
        semaphore.release()?;
        return Err("Semaphore value should be 0 after acquire".into());
    }
    
    semaphore.release()?;
    
    let value = semaphore.get_value()?;
    if value != 1 {
        return Err("Semaphore value should be 1 after release".into());
    }
    
    Ok(())
}

#[test]
#[ignore = "stress test - run with cargo test -- --ignored"]
fn test_memory_pressure_scenarios() {
    init_tracing!();
    
    let timer = common::timing::Timer::new("memory_pressure_scenarios");
    
    let _init = initialize();
    
    // Test creating many shared memory regions
    let num_regions = 50;
    let region_size = 1024 * 1024; // 1MB each
    let mut memory_regions = Vec::new();
    
    tracing::info!("Creating {} shared memory regions of {} bytes each", 
                  num_regions, region_size);
    
    for i in 0..num_regions {
        let name = format!("pressure_test_{}", i);
        let config = SharedMemoryConfig::new(&name, region_size);
        
        match SharedMemory::create(config) {
            Ok(shm) => {
                memory_regions.push(shm);
                if i % 10 == 0 {
                    tracing::debug!("Created {} memory regions", i + 1);
                }
            }
            Err(e) => {
                tracing::warn!("Failed to create memory region {}: {:?}", i, e);
                break;
            }
        }
    }
    
    let successful_regions = memory_regions.len();
    let total_memory_mb = (successful_regions * region_size) / (1024 * 1024);
    
    tracing::info!("Successfully created {} memory regions ({} MB total)", 
                  successful_regions, total_memory_mb);
    
    // Test writing to all regions
    let write_start = Instant::now();
    let mut write_failures = 0;
    
    for (i, shm) in memory_regions.iter_mut().enumerate() {
        let test_data = format!("Region {} test data", i);
        let large_data = test_data.repeat(1000); // Make it larger
        
        if let Err(e) = shm.write_bytes(large_data.as_bytes()) {
            write_failures += 1;
            tracing::warn!("Write failed for region {}: {:?}", i, e);
        }
    }
    
    let write_duration = write_start.elapsed();
    
    tracing::info!("Memory write test: {}/{} regions succeeded in {:?}", 
                  successful_regions - write_failures, successful_regions, write_duration);
    
    // Test reading from all regions
    let read_start = Instant::now();
    let mut read_failures = 0;
    
    for (i, shm) in memory_regions.iter_mut().enumerate() {
        let mut buffer = vec![0u8; 1000];
        if let Err(e) = shm.read_bytes(&mut buffer, 0) {
            read_failures += 1;
            tracing::warn!("Read failed for region {}: {:?}", i, e);
        }
    }
    
    let read_duration = read_start.elapsed();
    
    tracing::info!("Memory read test: {}/{} regions succeeded in {:?}", 
                  successful_regions - read_failures, successful_regions, read_duration);
    
    // Check system statistics
    let stats = get_ipc_statistics();
    tracing::info!("IPC Statistics: {} active regions, {} bytes total", 
                  stats.active_shared_memory_regions, stats.total_memory_usage);
    
    // Should successfully create and use a reasonable number of memory regions
    assert!(successful_regions >= 20, "Should create at least 20 memory regions");
    assert!(write_failures < successful_regions / 4, "Write failure rate should be < 25%");
    assert!(read_failures < successful_regions / 4, "Read failure rate should be < 25%");
    
    drop(memory_regions);
    let _shutdown = shutdown();
    drop(timer);
    
    tracing::info!("✅ Memory pressure scenarios stress test passed");
}

#[test]
#[ignore = "stress test - run with cargo test -- --ignored"]
fn test_sustained_process_load() {
    init_tracing!();
    
    let timer = common::timing::Timer::new("sustained_process_load");
    
    let test_duration = Duration::from_secs(30);
    let spawn_interval = Duration::from_millis(100);
    let max_concurrent_processes = 20;
    
    tracing::info!("Starting sustained process load test for {:?}", test_duration);
    
    let start_time = Instant::now();
    let active_processes = Arc::new(Mutex::new(Vec::new()));
    let stats = Arc::new(Mutex::new((0usize, 0usize))); // (spawned, completed)
    
    let active_processes_cleanup = active_processes.clone();
    let stats_spawner = stats.clone();
    
    // Process spawning thread
    let spawner_handle = thread::spawn(move || {
        let mut spawn_count = 0;
        
        while start_time.elapsed() < test_duration {
            let current_active = active_processes_cleanup.lock().unwrap().len();
            
            if current_active < max_concurrent_processes {
                let config = ProcessConfig::new("sleep").arg("2");
                
                match spawn_process(config) {
                    Ok(process) => {
                        spawn_count += 1;
                        active_processes_cleanup.lock().unwrap().push(process);
                        
                        if spawn_count % 10 == 0 {
                            tracing::debug!("Spawned {} processes, {} active", 
                                          spawn_count, current_active + 1);
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Failed to spawn process: {:?}", e);
                    }
                }
            }
            
            thread::sleep(spawn_interval);
        }
        
        stats_spawner.lock().unwrap().0 = spawn_count;
    });
    
    // Process cleanup thread
    let active_processes_monitor = active_processes.clone();
    let stats_monitor = stats.clone();
    
    let cleanup_handle = thread::spawn(move || {
        let mut completed_count = 0;
        
        while start_time.elapsed() < test_duration + Duration::from_secs(5) {
            let mut active = active_processes_monitor.lock().unwrap();
            let mut i = 0;
            
            while i < active.len() {
                match active[i].is_running() {
                    Ok(false) => {
                        // Process completed
                        active.remove(i);
                        completed_count += 1;
                        
                        if completed_count % 10 == 0 {
                            tracing::debug!("Completed {} processes", completed_count);
                        }
                    }
                    Ok(true) => {
                        i += 1;
                    }
                    Err(e) => {
                        tracing::warn!("Error checking process status: {:?}", e);
                        active.remove(i);
                    }
                }
            }
            
            drop(active);
            thread::sleep(Duration::from_millis(200));
        }
        
        stats_monitor.lock().unwrap().1 = completed_count;
    });
    
    // Wait for spawner to complete
    spawner_handle.join().expect("Spawner thread should complete");
    
    // Wait for cleanup to complete
    cleanup_handle.join().expect("Cleanup thread should complete");
    
    // Clean up any remaining processes
    let mut remaining_processes = active_processes.lock().unwrap();
    for mut process in remaining_processes.drain(..) {
        let _ = process.kill();
    }
    
    let final_stats = stats.lock().unwrap();
    let (total_spawned, total_completed) = *final_stats;
    
    let completion_rate = if total_spawned > 0 {
        (total_completed as f64 / total_spawned as f64) * 100.0
    } else {
        0.0
    };
    
    tracing::info!("Sustained load results: {}/{} processes completed ({:.1}%)", 
                  total_completed, total_spawned, completion_rate);
    
    // Should maintain reasonable completion rate under sustained load
    assert!(total_spawned > 50, "Should have spawned significant number of processes");
    assert!(completion_rate >= 80.0, "Completion rate should be >= 80%");
    
    drop(timer);
    
    tracing::info!("✅ Sustained process load stress test passed");
}

#[test]
#[ignore = "stress test - run with cargo test -- --ignored"]
fn test_resource_exhaustion_recovery() {
    init_tracing!();
    
    let timer = common::timing::Timer::new("resource_exhaustion_recovery");
    
    let _init = initialize();
    
    // Test file descriptor exhaustion with pipes
    tracing::info!("Testing file descriptor exhaustion with named pipes");
    
    let mut pipes = Vec::new();
    let mut pipe_failures = 0;
    
    for i in 0..200 { // Try to create many pipes
        let pipe_name = format!("/tmp/stress_pipe_{}", i);
        match NamedPipe::create(&pipe_name, PipeMode::ReadWrite) {
            Ok(pipe) => pipes.push(pipe),
            Err(e) => {
                pipe_failures += 1;
                if pipe_failures == 1 {
                    tracing::info!("First pipe failure at {}: {:?}", i, e);
                }
                if i < 50 {
                    // If we fail too early, something is wrong
                    panic!("Unexpected early pipe failure at {}: {:?}", i, e);
                }
                break;
            }
        }
    }
    
    let successful_pipes = pipes.len();
    tracing::info!("Created {} pipes before exhaustion", successful_pipes);
    
    // Test recovery by releasing some resources
    let release_count = successful_pipes / 2;
    pipes.truncate(successful_pipes - release_count);
    
    tracing::info!("Released {} pipes, testing recovery", release_count);
    
    // Try to create new pipes after release
    let mut recovery_pipes = Vec::new();
    for i in 0..20 {
        let pipe_name = format!("/tmp/recovery_pipe_{}", i);
        match NamedPipe::create(&pipe_name, PipeMode::ReadWrite) {
            Ok(pipe) => recovery_pipes.push(pipe),
            Err(e) => {
                tracing::warn!("Recovery pipe {} failed: {:?}", i, e);
                break;
            }
        }
    }
    
    let recovery_count = recovery_pipes.len();
    tracing::info!("Successfully created {} pipes during recovery", recovery_count);
    
    // Should be able to create some pipes after releasing resources
    assert!(recovery_count >= 5, "Should recover and create at least 5 pipes");
    
    // Test process limit exhaustion
    tracing::info!("Testing process limit handling");
    
    let mut processes = Vec::new();
    let mut process_failures = 0;
    
    for i in 0..100 { // Try to create many long-running processes
        let config = ProcessConfig::new("sleep").arg("30");
        match spawn_process(config) {
            Ok(process) => processes.push(process),
            Err(e) => {
                process_failures += 1;
                if process_failures == 1 {
                    tracing::info!("First process failure at {}: {:?}", i, e);
                }
                if process_failures > 10 {
                    break; // Stop if too many failures
                }
            }
        }
    }
    
    let successful_processes = processes.len();
    tracing::info!("Created {} processes before limits", successful_processes);
    
    // Clean up processes
    for mut process in processes {
        let _ = process.kill();
    }
    
    // Should be able to create a reasonable number of processes
    assert!(successful_processes >= 20, "Should create at least 20 processes");
    
    let _shutdown = shutdown();
    drop(timer);
    
    tracing::info!("✅ Resource exhaustion recovery stress test passed");
}

#[test]
#[ignore = "stress test - run with cargo test -- --ignored"]
fn test_performance_degradation_analysis() {
    init_tracing!();
    
    let timer = common::timing::Timer::new("performance_degradation_analysis");
    
    let _init = initialize();
    
    // Test performance under increasing load
    let load_levels = vec![1, 5, 10, 20, 50];
    let operations_per_level = 20;
    
    let mut performance_data = HashMap::new();
    
    for load_level in load_levels {
        tracing::info!("Testing performance at load level: {}", load_level);
        
        let level_start = Instant::now();
        let barrier = Arc::new(Barrier::new(load_level));
        
        let handles: Vec<_> = (0..load_level)
            .map(|thread_id| {
                let barrier = barrier.clone();
                thread::spawn(move || {
                    barrier.wait();
                    
                    let mut operation_times = Vec::new();
                    
                    for op_id in 0..operations_per_level {
                        let op_start = Instant::now();
                        
                        // Perform a complex operation combining process and IPC
                        let config = ProcessConfig::new("echo")
                            .arg(&format!("load_{}_{}", thread_id, op_id));
                        
                        let process_result = run_command(config);
                        
                        // Also do some IPC operation
                        let shm_name = format!("perf_test_{}_{}", thread_id, op_id);
                        let shm_config = SharedMemoryConfig::new(&shm_name, 1024)
                            .with_remove_on_drop();
                        
                        let ipc_result = if let Ok(mut shm) = SharedMemory::create(shm_config) {
                            shm.write_bytes(b"performance test data").is_ok()
                        } else {
                            false
                        };
                        
                        let op_duration = op_start.elapsed();
                        operation_times.push((
                            op_duration,
                            process_result.is_ok(),
                            ipc_result
                        ));
                    }
                    
                    operation_times
                })
            })
            .collect();
        
        // Collect results for this load level
        let mut all_times = Vec::new();
        let mut process_success_count = 0;
        let mut ipc_success_count = 0;
        let total_operations = load_level * operations_per_level;
        
        for handle in handles {
            let operation_times = handle.join().expect("Thread should complete");
            for (duration, process_ok, ipc_ok) in operation_times {
                all_times.push(duration);
                if process_ok { process_success_count += 1; }
                if ipc_ok { ipc_success_count += 1; }
            }
        }
        
        let level_duration = level_start.elapsed();
        
        // Calculate statistics
        all_times.sort();
        let median_time = all_times[all_times.len() / 2];
        let p95_time = all_times[(all_times.len() * 95) / 100];
        let max_time = all_times[all_times.len() - 1];
        let avg_time = all_times.iter().sum::<Duration>() / all_times.len() as u32;
        
        let process_success_rate = (process_success_count as f64 / total_operations as f64) * 100.0;
        let ipc_success_rate = (ipc_success_count as f64 / total_operations as f64) * 100.0;
        
        performance_data.insert(load_level, (
            avg_time,
            median_time,
            p95_time,
            max_time,
            process_success_rate,
            ipc_success_rate,
            level_duration
        ));
        
        tracing::info!("Load level {} results:", load_level);
        tracing::info!("  Average: {:?}, Median: {:?}, P95: {:?}, Max: {:?}", 
                      avg_time, median_time, p95_time, max_time);
        tracing::info!("  Process success: {:.1}%, IPC success: {:.1}%", 
                      process_success_rate, ipc_success_rate);
        tracing::info!("  Total duration: {:?}", level_duration);
    }
    
    // Analyze performance degradation
    let baseline_avg = performance_data[&1].0;
    let high_load_avg = performance_data[&50].0;
    
    let degradation_factor = high_load_avg.as_nanos() as f64 / baseline_avg.as_nanos() as f64;
    
    tracing::info!("Performance analysis:");
    tracing::info!("  Baseline (load 1): {:?}", baseline_avg);
    tracing::info!("  High load (load 50): {:?}", high_load_avg);
    tracing::info!("  Degradation factor: {:.2}x", degradation_factor);
    
    // Performance shouldn't degrade too much under load
    assert!(degradation_factor < 10.0, 
           "Performance degradation should be < 10x, got {:.2}x", degradation_factor);
    
    // Success rates should remain high
    for (load_level, (_, _, _, _, proc_rate, ipc_rate, _)) in &performance_data {
        assert!(*proc_rate >= 90.0, 
               "Process success rate at load {} should be >= 90%, got {:.1}%", 
               load_level, proc_rate);
        assert!(*ipc_rate >= 85.0, 
               "IPC success rate at load {} should be >= 85%, got {:.1}%", 
               load_level, ipc_rate);
    }
    
    let _shutdown = shutdown();
    drop(timer);
    
    tracing::info!("✅ Performance degradation analysis stress test passed");
}
