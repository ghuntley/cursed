/// Process Management and IPC Performance Tests
/// 
/// This test suite validates the performance characteristics of process
/// management and IPC operations under various load conditions, ensuring
/// scalability and efficient resource utilization.

use std::thread;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex, Barrier};

use cursed::stdlib::exec_slay::{SlayCommand, SlayProcessState};
use cursed::stdlib::process::background_tasks::{
    TaskManager, TaskConfig, ManagerConfig, run_background, TaskPriority
};
use cursed::stdlib::ipc::real_ipc::{
    RealIpcManager, IpcConnection, IpcMessage, MessagePriority
};
use cursed::stdlib::ipc::IpcConfig;

// Performance test configuration
const PERFORMANCE_TIMEOUT: Duration = Duration::from_secs(30);
const LIGHT_LOAD_SIZE: usize = 10;
const MEDIUM_LOAD_SIZE: usize = 50;
const HEAVY_LOAD_SIZE: usize = 100;

#[test]
#[ignore] // Performance test - run with --ignored
fn test_process_spawn_throughput() {
    let test_start = Instant::now();
    let mut processes = Vec::new();
    
    // Measure process spawn throughput
    let spawn_start = Instant::now();
    
    for i in 0..MEDIUM_LOAD_SIZE {
        let mut command = SlayCommand::new("echo");
        command.arg(&format!("throughput_test_{}", i));
        
        let process = command.spawn().expect("Failed to spawn process");
        processes.push(process);
    }
    
    let spawn_duration = spawn_start.elapsed();
    let spawn_rate = MEDIUM_LOAD_SIZE as f64 / spawn_duration.as_secs_f64();
    
    println!("Process spawn rate: {:.2} processes/second", spawn_rate);
    assert!(spawn_rate > 10.0, "Spawn rate too low: {:.2} proc/sec", spawn_rate);
    
    // Wait for all processes to complete
    let wait_start = Instant::now();
    let mut successful = 0;
    
    for process in processes {
        if let Ok(result) = process.wait() {
            if result.success() {
                successful += 1;
            }
        }
    }
    
    let wait_duration = wait_start.elapsed();
    let completion_rate = successful as f64 / wait_duration.as_secs_f64();
    
    println!("Process completion rate: {:.2} processes/second", completion_rate);
    println!("Success rate: {}/{} ({:.1}%)", successful, MEDIUM_LOAD_SIZE, 
             (successful as f64 / MEDIUM_LOAD_SIZE as f64) * 100.0);
    
    assert_eq!(successful, MEDIUM_LOAD_SIZE);
    assert!(test_start.elapsed() < PERFORMANCE_TIMEOUT);
}

#[test]
#[ignore] // Performance test - run with --ignored
fn test_concurrent_process_execution() {
    let num_threads = 8;
    let processes_per_thread = 10;
    let barrier = Arc::new(Barrier::new(num_threads));
    let results = Arc::new(Mutex::new(Vec::new()));
    
    let test_start = Instant::now();
    let mut handles = Vec::new();
    
    for thread_id in 0..num_threads {
        let barrier = barrier.clone();
        let results = results.clone();
        
        let handle = thread::spawn(move || {
            // Wait for all threads to be ready
            barrier.wait();
            
            let thread_start = Instant::now();
            let mut thread_results = Vec::new();
            
            for i in 0..processes_per_thread {
                let mut command = SlayCommand::new("echo");
                command.arg(&format!("concurrent_{}_{}", thread_id, i));
                
                let process_start = Instant::now();
                let process = command.spawn().expect("Failed to spawn process");
                let result = process.wait().expect("Failed to wait for process");
                let process_duration = process_start.elapsed();
                
                thread_results.push((result.success(), process_duration));
            }
            
            let thread_duration = thread_start.elapsed();
            
            // Store results
            let mut results_guard = results.lock().unwrap();
            results_guard.push((thread_id, thread_duration, thread_results));
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
    
    let total_duration = test_start.elapsed();
    let results_guard = results.lock().unwrap();
    
    // Analyze results
    let total_processes = num_threads * processes_per_thread;
    let mut successful = 0;
    let mut total_process_time = Duration::from_millis(0);
    
    for (thread_id, thread_duration, thread_results) in results_guard.iter() {
        for (success, duration) in thread_results {
            if *success {
                successful += 1;
            }
            total_process_time += *duration;
        }
        
        println!("Thread {}: completed in {:?}", thread_id, thread_duration);
    }
    
    let avg_process_time = total_process_time / total_processes as u32;
    let throughput = total_processes as f64 / total_duration.as_secs_f64();
    
    println!("Concurrent execution results:");
    println!("  Total processes: {}", total_processes);
    println!("  Successful: {} ({:.1}%)", successful, 
             (successful as f64 / total_processes as f64) * 100.0);
    println!("  Total time: {:?}", total_duration);
    println!("  Average process time: {:?}", avg_process_time);
    println!("  Throughput: {:.2} processes/second", throughput);
    
    assert_eq!(successful, total_processes);
    assert!(throughput > 5.0, "Concurrent throughput too low: {:.2}", throughput);
    assert!(total_duration < PERFORMANCE_TIMEOUT);
}

#[test]
#[ignore] // Performance test - run with --ignored
fn test_background_task_manager_performance() {
    let config = ManagerConfig {
        max_concurrent_tasks: 20,
        cleanup_interval: Duration::from_millis(100),
        auto_monitor: false, // Disable monitoring for performance test
        ..Default::default()
    };
    
    let mut manager = TaskManager::new(config);
    manager.start().expect("Failed to start task manager");
    
    let test_start = Instant::now();
    let mut task_ids = Vec::new();
    
    // Submit tasks rapidly
    let submit_start = Instant::now();
    for i in 0..MEDIUM_LOAD_SIZE {
        let mut command = SlayCommand::new("echo");
        command.arg(&format!("bg_perf_test_{}", i));
        
        let task_config = TaskConfig {
            capture_output: false, // Disable output capture for performance
            monitor_resources: false,
            ..Default::default()
        };
        
        let task_id = manager.submit_task(command, Some(task_config))
            .expect("Failed to submit task");
        task_ids.push(task_id);
    }
    
    let submit_duration = submit_start.elapsed();
    let submit_rate = MEDIUM_LOAD_SIZE as f64 / submit_duration.as_secs_f64();
    
    // Wait for all tasks to complete
    let wait_start = Instant::now();
    manager.wait_for_all().expect("Failed to wait for all tasks");
    let wait_duration = wait_start.elapsed();
    
    let total_duration = test_start.elapsed();
    let stats = manager.get_task_stats();
    
    println!("Background task manager performance:");
    println!("  Tasks submitted: {}", MEDIUM_LOAD_SIZE);
    println!("  Submit rate: {:.2} tasks/second", submit_rate);
    println!("  Submit time: {:?}", submit_duration);
    println!("  Wait time: {:?}", wait_duration);
    println!("  Total time: {:?}", total_duration);
    println!("  Completed tasks: {}", stats.completed_tasks);
    println!("  Failed tasks: {}", stats.failed_tasks);
    
    manager.stop().expect("Failed to stop task manager");
    
    assert!(submit_rate > 50.0, "Task submit rate too low: {:.2}", submit_rate);
    assert_eq!(stats.completed_tasks, MEDIUM_LOAD_SIZE);
    assert_eq!(stats.failed_tasks, 0);
    assert!(total_duration < PERFORMANCE_TIMEOUT);
}

#[test]
#[ignore] // Performance test - run with --ignored
fn test_ipc_message_queue_throughput() {
    let config = IpcConfig {
        max_queue_size: 1000,
        max_message_size: 1024,
        ..Default::default()
    };
    
    let manager = RealIpcManager::new(config).expect("Failed to create IPC manager");
    let queue_name = format!("perf_queue_{}", std::process::id());
    let connection = manager.create_message_queue(&queue_name)
        .expect("Failed to create message queue");
    
    if let IpcConnection::MessageQueue(ref queue_conn) = *connection {
        let test_start = Instant::now();
        let message_count = HEAVY_LOAD_SIZE;
        
        // Send messages rapidly
        let send_start = Instant::now();
        for i in 0..message_count {
            let message = IpcMessage {
                id: i as u64,
                sender_id: std::process::id(),
                priority: MessagePriority::Normal,
                timestamp: std::time::SystemTime::now(),
                data: format!("performance_test_message_{}", i).into_bytes(),
                message_type: "perf_test".to_string(),
                reply_to: None,
                ttl: None,
            };
            
            queue_conn.send(message).expect("Failed to send message");
        }
        
        let send_duration = send_start.elapsed();
        let send_rate = message_count as f64 / send_duration.as_secs_f64();
        
        // Receive messages rapidly
        let receive_start = Instant::now();
        let mut received_count = 0;
        
        for _ in 0..message_count {
            if let Ok(_message) = queue_conn.receive(Some(Duration::from_millis(100))) {
                received_count += 1;
            }
        }
        
        let receive_duration = receive_start.elapsed();
        let receive_rate = received_count as f64 / receive_duration.as_secs_f64();
        let total_duration = test_start.elapsed();
        
        println!("IPC message queue performance:");
        println!("  Messages sent: {}", message_count);
        println!("  Messages received: {}", received_count);
        println!("  Send rate: {:.2} messages/second", send_rate);
        println!("  Receive rate: {:.2} messages/second", receive_rate);
        println!("  Send time: {:?}", send_duration);
        println!("  Receive time: {:?}", receive_duration);
        println!("  Total time: {:?}", total_duration);
        
        assert!(send_rate > 100.0, "Send rate too low: {:.2}", send_rate);
        assert!(receive_rate > 100.0, "Receive rate too low: {:.2}", receive_rate);
        assert_eq!(received_count, message_count);
        assert!(total_duration < PERFORMANCE_TIMEOUT);
    }
    
    manager.remove_connection(&queue_name).expect("Failed to remove connection");
}

#[test]
#[ignore] // Performance test - run with --ignored
fn test_shared_memory_throughput() {
    let config = IpcConfig::default();
    let manager = RealIpcManager::new(config).expect("Failed to create IPC manager");
    
    let segment_name = format!("perf_shm_{}", std::process::id());
    let segment_size = 1024 * 1024; // 1MB
    let connection = manager.create_shared_memory(&segment_name, segment_size)
        .expect("Failed to create shared memory");
    
    if let IpcConnection::SharedMemory(ref shm_conn) = *connection {
        let test_start = Instant::now();
        let operation_count = 1000;
        let data_size = 1024; // 1KB per operation
        let test_data = vec![0xAA; data_size];
        
        // Write performance test
        let write_start = Instant::now();
        for i in 0..operation_count {
            let offset = (i * data_size) % (segment_size - data_size);
            shm_conn.write(offset, &test_data).expect("Failed to write to shared memory");
        }
        let write_duration = write_start.elapsed();
        let write_rate = operation_count as f64 / write_duration.as_secs_f64();
        let write_bandwidth = (operation_count * data_size) as f64 / write_duration.as_secs_f64();
        
        // Read performance test
        let mut read_buffer = vec![0u8; data_size];
        let read_start = Instant::now();
        for i in 0..operation_count {
            let offset = (i * data_size) % (segment_size - data_size);
            shm_conn.read(offset, &mut read_buffer).expect("Failed to read from shared memory");
        }
        let read_duration = read_start.elapsed();
        let read_rate = operation_count as f64 / read_duration.as_secs_f64();
        let read_bandwidth = (operation_count * data_size) as f64 / read_duration.as_secs_f64();
        
        let total_duration = test_start.elapsed();
        
        println!("Shared memory performance:");
        println!("  Operations: {}", operation_count);
        println!("  Data size per operation: {} bytes", data_size);
        println!("  Write rate: {:.2} operations/second", write_rate);
        println!("  Read rate: {:.2} operations/second", read_rate);
        println!("  Write bandwidth: {:.2} MB/s", write_bandwidth / 1024.0 / 1024.0);
        println!("  Read bandwidth: {:.2} MB/s", read_bandwidth / 1024.0 / 1024.0);
        println!("  Total time: {:?}", total_duration);
        
        assert!(write_rate > 100.0, "Write rate too low: {:.2}", write_rate);
        assert!(read_rate > 100.0, "Read rate too low: {:.2}", read_rate);
        assert!(total_duration < PERFORMANCE_TIMEOUT);
    }
    
    manager.remove_connection(&segment_name).expect("Failed to remove connection");
}

#[test]
#[ignore] // Performance test - run with --ignored
fn test_concurrent_ipc_operations() {
    let config = IpcConfig {
        max_queue_size: 500,
        max_message_size: 512,
        ..Default::default()
    };
    
    let manager = Arc::new(RealIpcManager::new(config).expect("Failed to create IPC manager"));
    let num_threads = 4;
    let messages_per_thread = 25;
    let barrier = Arc::new(Barrier::new(num_threads));
    let results = Arc::new(Mutex::new(Vec::new()));
    
    let test_start = Instant::now();
    let mut handles = Vec::new();
    
    for thread_id in 0..num_threads {
        let manager = manager.clone();
        let barrier = barrier.clone();
        let results = results.clone();
        
        let handle = thread::spawn(move || {
            let queue_name = format!("concurrent_queue_{}_{}", thread_id, std::process::id());
            let connection = manager.create_message_queue(&queue_name)
                .expect("Failed to create message queue");
            
            // Wait for all threads to be ready
            barrier.wait();
            
            let thread_start = Instant::now();
            
            if let IpcConnection::MessageQueue(ref queue_conn) = *connection {
                // Send and receive messages
                for i in 0..messages_per_thread {
                    let message = IpcMessage {
                        id: i as u64,
                        sender_id: thread_id as u32,
                        priority: MessagePriority::Normal,
                        timestamp: std::time::SystemTime::now(),
                        data: format!("thread_{}_message_{}", thread_id, i).into_bytes(),
                        message_type: "concurrent_test".to_string(),
                        reply_to: None,
                        ttl: None,
                    };
                    
                    queue_conn.send(message).expect("Failed to send message");
                    
                    let received = queue_conn.receive(Some(Duration::from_millis(100)))
                        .expect("Failed to receive message");
                    
                    assert_eq!(received.id, i as u64);
                }
            }
            
            let thread_duration = thread_start.elapsed();
            
            // Store results
            let mut results_guard = results.lock().unwrap();
            results_guard.push((thread_id, thread_duration));
            
            // Clean up
            manager.remove_connection(&queue_name).expect("Failed to remove connection");
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
    
    let total_duration = test_start.elapsed();
    let results_guard = results.lock().unwrap();
    
    let total_operations = num_threads * messages_per_thread * 2; // Send + receive
    let avg_thread_time: Duration = results_guard.iter()
        .map(|(_, duration)| *duration)
        .sum::<Duration>() / num_threads as u32;
    let throughput = total_operations as f64 / total_duration.as_secs_f64();
    
    println!("Concurrent IPC performance:");
    println!("  Threads: {}", num_threads);
    println!("  Messages per thread: {}", messages_per_thread);
    println!("  Total operations: {}", total_operations);
    println!("  Average thread time: {:?}", avg_thread_time);
    println!("  Total time: {:?}", total_duration);
    println!("  Throughput: {:.2} operations/second", throughput);
    
    for (thread_id, duration) in results_guard.iter() {
        println!("  Thread {}: {:?}", thread_id, duration);
    }
    
    assert!(throughput > 50.0, "Concurrent throughput too low: {:.2}", throughput);
    assert!(total_duration < PERFORMANCE_TIMEOUT);
}

#[test]
#[ignore] // Performance test - run with --ignored
fn test_memory_usage_and_cleanup() {
    // Test that memory usage doesn't grow excessively during heavy operations
    let initial_memory = get_memory_usage();
    
    // Perform many operations that should be cleaned up
    for round in 0..10 {
        let mut processes = Vec::new();
        
        // Spawn processes
        for i in 0..20 {
            let mut command = SlayCommand::new("echo");
            command.arg(&format!("memory_test_{}_{}", round, i));
            
            let process = command.spawn().expect("Failed to spawn process");
            processes.push(process);
        }
        
        // Wait for completion
        for process in processes {
            let _result = process.wait().expect("Failed to wait for process");
        }
        
        // IPC operations
        let config = IpcConfig::default();
        let manager = RealIpcManager::new(config).expect("Failed to create IPC manager");
        
        for i in 0..10 {
            let queue_name = format!("memory_queue_{}_{}", round, i);
            let _connection = manager.create_message_queue(&queue_name)
                .expect("Failed to create message queue");
            
            manager.remove_connection(&queue_name).expect("Failed to remove connection");
        }
        
        // Give GC time to clean up
        thread::sleep(Duration::from_millis(100));
    }
    
    let final_memory = get_memory_usage();
    let memory_growth = final_memory.saturating_sub(initial_memory);
    
    println!("Memory usage test:");
    println!("  Initial memory: {} MB", initial_memory / 1024 / 1024);
    println!("  Final memory: {} MB", final_memory / 1024 / 1024);
    println!("  Memory growth: {} MB", memory_growth / 1024 / 1024);
    
    // Memory growth should be reasonable (less than 50MB for this test)
    assert!(memory_growth < 50 * 1024 * 1024, 
            "Excessive memory growth: {} MB", memory_growth / 1024 / 1024);
}

// Helper function to get current memory usage (approximate)
fn get_memory_usage() -> usize {
    #[cfg(unix)]
    {
        if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
            for line in status.lines() {
                if line.starts_with("VmRSS:") {
                    if let Some(parts) = line.split_whitespace().nth(1) {
                        if let Ok(kb) = parts.parse::<usize>() {
                            return kb * 1024; // Convert KB to bytes
                        }
                    }
                }
            }
        }
    }
    
    // Fallback: return 0 if we can't get actual memory usage
    0
}
