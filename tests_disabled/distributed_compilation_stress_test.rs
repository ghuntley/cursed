//! Stress tests for distributed compilation system
//! 
//! Tests the system under extreme conditions including large-scale
//! compilation, network failures, high concurrency, and long-running stability.

use std::sync::{Arc, atomic::{AtomicBool, AtomicUsize, Ordering}};
use std::time::{Duration, Instant};
use std::thread;
use std::collections::{HashMap, HashSet};

use cursed::build_system::distributed_compilation::{
    DistributedCompilationSystem,
    DistributedCompilationConfig,
    CompilationTask,
    CompilationTarget,
    CompilationNode,
    NodeStatus,
    NodeCapabilities,
    LoadBalancingStrategy,
    create_compilation_task,
};
use cursed::error::Result;

#[path = "common.rs"]
mod common;

// Initialize tracing for tests
macro_rules! init_tracing {
    () => {
        let _ = tracing_subscriber::fmt()
            .with_env_filter("debug")
            .with_test_writer()
            .try_init();
    };
}

#[test]
#[ignore] // Stress test - run with `cargo test -- --ignored`
fn test_massive_task_submission() {
    init_tracing!();
    tracing::info!("Starting massive task submission stress test");
    
    let mut config = DistributedCompilationConfig::default();
    config.coordinator_port = 9400;
    config.worker_ports = vec![9401, 9402, 9403, 9404, 9405, 9406, 9407, 9408];
    config.task_timeout_seconds = 120;
    config.work_stealing_enabled = true;
    config.fault_tolerance_enabled = true;
    
    let mut system = DistributedCompilationSystem::new(config).expect("Failed to create system");
    system.start().expect("Failed to start system");
    
    // Wait for system initialization
    thread::sleep(Duration::from_millis(500));
    
    let num_tasks = 1000; // Large number of tasks
    let batch_size = 50;
    let total_batches = num_tasks / batch_size;
    
    tracing::info!("Submitting {} tasks in {} batches of {}", num_tasks, total_batches, batch_size);
    
    let start_time = Instant::now();
    let submitted_count = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();
    
    // Submit tasks concurrently from multiple threads
    for batch_id in 0..total_batches {
        let system_arc = Arc::new(&system);
        let submitted_count_clone = submitted_count.clone();
        
        let handle = thread::spawn(move || {
            let mut tasks = Vec::new();
            
            for task_id in 0..batch_size {
                let global_task_id = batch_id * batch_size + task_id;
                let target = match global_task_id % 5 {
                    0 => CompilationTarget::Object,
                    1 => CompilationTarget::IR,
                    2 => CompilationTarget::Assembly,
                    3 => CompilationTarget::Library,
                    _ => CompilationTarget::Executable,
                };
                
                tasks.push(create_compilation_task(
                    vec![format!("massive_file_{}.csd", global_task_id)],
                    target,
                    vec![format!("-DTASK_ID={}", global_task_id)],
                ));
            }
            
            if let Ok(task_ids) = system_arc.submit_batch(tasks) {
                submitted_count_clone.fetch_add(task_ids.len(), Ordering::Relaxed);
                tracing::debug!("Batch {} submitted {} tasks", batch_id, task_ids.len());
            } else {
                tracing::warn!("Failed to submit batch {}", batch_id);
            }
        });
        
        handles.push(handle);
        
        // Small delay between thread spawns to avoid overwhelming
        thread::sleep(Duration::from_millis(10));
    }
    
    // Wait for all submission threads to complete
    for handle in handles {
        handle.join().expect("Submission thread panicked");
    }
    
    let submission_time = start_time.elapsed();
    let final_submitted = submitted_count.load(Ordering::Relaxed);
    
    tracing::info!(
        "Submitted {} tasks in {:.2} seconds ({:.1} tasks/sec)",
        final_submitted,
        submission_time.as_secs_f64(),
        final_submitted as f64 / submission_time.as_secs_f64()
    );
    
    // Monitor system under load
    let monitoring_start = Instant::now();
    let max_monitoring_time = Duration::from_secs(60);
    let mut last_stats_time = Instant::now();
    
    while monitoring_start.elapsed() < max_monitoring_time {
        if last_stats_time.elapsed() >= Duration::from_secs(5) {
            if let Ok(stats) = system.get_statistics() {
                let processed = stats.completed_tasks + stats.failed_tasks;
                let processing_rate = if monitoring_start.elapsed().as_secs() > 0 {
                    processed as f64 / monitoring_start.elapsed().as_secs() as f64
                } else {
                    0.0
                };
                
                tracing::info!(
                    "Processing: {}/{} ({:.1}%), rate: {:.1}/sec, efficiency: {:.3}",
                    processed,
                    stats.total_tasks,
                    (processed as f64 / stats.total_tasks.max(1) as f64) * 100.0,
                    processing_rate,
                    stats.load_balancing_efficiency
                );
                
                // Check if we've processed most tasks
                if processed >= (final_submitted * 80 / 100) {
                    tracing::info!("Processed 80% of tasks, ending monitoring early");
                    break;
                }
            }
            last_stats_time = Instant::now();
        }
        
        thread::sleep(Duration::from_millis(500));
    }
    
    let final_stats = system.get_statistics().expect("Failed to get final statistics");
    let total_time = start_time.elapsed();
    
    tracing::info!(
        "Massive task submission completed: {} total tasks, {} processed in {:.1}s",
        final_stats.total_tasks,
        final_stats.completed_tasks + final_stats.failed_tasks,
        total_time.as_secs_f64()
    );
    
    // Performance assertions
    assert!(final_submitted >= 800, "Should submit at least 800 tasks, got {}", final_submitted);
    assert!(final_stats.total_tasks >= final_submitted, "System should track all submitted tasks");
    
    system.stop().expect("Failed to stop system");
    tracing::info!("✓ Massive task submission stress test completed");
}

#[test]
#[ignore] // Stress test - run with `cargo test -- --ignored`
fn test_high_concurrency_operations() {
    init_tracing!();
    tracing::info!("Starting high concurrency operations stress test");
    
    let mut config = DistributedCompilationConfig::default();
    config.coordinator_port = 9410;
    config.worker_ports = vec![9411, 9412, 9413];
    config.heartbeat_interval_seconds = 2;
    config.task_timeout_seconds = 30;
    
    let mut system = DistributedCompilationSystem::new(config).expect("Failed to create system");
    system.start().expect("Failed to start system");
    
    let system = Arc::new(system);
    let test_duration = Duration::from_secs(20);
    let is_running = Arc::new(AtomicBool::new(true));
    
    let task_submission_count = Arc::new(AtomicUsize::new(0));
    let node_operations_count = Arc::new(AtomicUsize::new(0));
    let stats_queries_count = Arc::new(AtomicUsize::new(0));
    
    let mut handles = Vec::new();
    
    // Spawn task submission threads
    for thread_id in 0..8 {
        let system_clone = system.clone();
        let is_running_clone = is_running.clone();
        let submission_count = task_submission_count.clone();
        
        let handle = thread::spawn(move || {
            let mut local_count = 0;
            
            while is_running_clone.load(Ordering::Relaxed) {
                let task = create_compilation_task(
                    vec![format!("concurrent_{}_{}.csd", thread_id, local_count)],
                    match local_count % 3 {
                        0 => CompilationTarget::Object,
                        1 => CompilationTarget::IR,
                        _ => CompilationTarget::Assembly,
                    },
                    vec![],
                );
                
                if system_clone.submit_task(task).is_ok() {
                    local_count += 1;
                    submission_count.fetch_add(1, Ordering::Relaxed);
                }
                
                thread::sleep(Duration::from_millis(5));
            }
            
            tracing::info!("Task submission thread {} submitted {} tasks", thread_id, local_count);
        });
        
        handles.push(handle);
    }
    
    // Spawn node management threads
    for thread_id in 0..4 {
        let system_clone = system.clone();
        let is_running_clone = is_running.clone();
        let operations_count = node_operations_count.clone();
        
        let handle = thread::spawn(move || {
            let mut local_count = 0;
            
            while is_running_clone.load(Ordering::Relaxed) {
                // Register a temporary node
                let node = CompilationNode {
                    id: format!("temp_node_{}_{}", thread_id, local_count),
                    address: "127.0.0.1".to_string(),
                    port: 9500 + thread_id * 100 + local_count % 100,
                    capabilities: NodeCapabilities {
                        cpu_cores: 2,
                        memory_gb: 4,
                        disk_space_gb: 50,
                        supported_targets: vec![CompilationTarget::Object],
                        compiler_versions: HashMap::new(),
                        performance_score: 1.0,
                    },
                    status: NodeStatus::Online,
                    current_load: 0.0,
                    max_concurrent_tasks: 2,
                    active_tasks: HashSet::new(),
                    completed_tasks: 0,
                    failed_tasks: 0,
                    average_task_duration: Duration::from_millis(100),
                    last_heartbeat: 0,
                    is_coordinator: false,
                };
                
                if system_clone.register_node(node.clone()).is_ok() {
                    operations_count.fetch_add(1, Ordering::Relaxed);
                    
                    // Keep it briefly, then remove
                    thread::sleep(Duration::from_millis(100));
                    
                    if system_clone.unregister_node(&node.id).is_ok() {
                        operations_count.fetch_add(1, Ordering::Relaxed);
                    }
                }
                
                local_count += 1;
                thread::sleep(Duration::from_millis(50));
            }
            
            tracing::info!("Node management thread {} performed {} operations", thread_id, local_count * 2);
        });
        
        handles.push(handle);
    }
    
    // Spawn statistics monitoring threads
    for thread_id in 0..3 {
        let system_clone = system.clone();
        let is_running_clone = is_running.clone();
        let queries_count = stats_queries_count.clone();
        
        let handle = thread::spawn(move || {
            let mut local_count = 0;
            
            while is_running_clone.load(Ordering::Relaxed) {
                if system_clone.get_statistics().is_ok() {
                    local_count += 1;
                    queries_count.fetch_add(1, Ordering::Relaxed);
                }
                
                if system_clone.get_nodes().is_ok() {
                    local_count += 1;
                    queries_count.fetch_add(1, Ordering::Relaxed);
                }
                
                thread::sleep(Duration::from_millis(20));
            }
            
            tracing::info!("Statistics thread {} performed {} queries", thread_id, local_count);
        });
        
        handles.push(handle);
    }
    
    // Let the test run for the specified duration
    let start_time = Instant::now();
    while start_time.elapsed() < test_duration {
        thread::sleep(Duration::from_millis(500));
        
        // Log progress every 5 seconds
        if start_time.elapsed().as_secs() % 5 == 0 {
            tracing::info!(
                "Concurrency test progress: {} tasks, {} node ops, {} stat queries",
                task_submission_count.load(Ordering::Relaxed),
                node_operations_count.load(Ordering::Relaxed),
                stats_queries_count.load(Ordering::Relaxed)
            );
        }
    }
    
    // Stop all threads
    is_running.store(false, Ordering::Relaxed);
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Concurrent operation thread panicked");
    }
    
    let final_tasks = task_submission_count.load(Ordering::Relaxed);
    let final_node_ops = node_operations_count.load(Ordering::Relaxed);
    let final_stat_queries = stats_queries_count.load(Ordering::Relaxed);
    
    tracing::info!(
        "High concurrency test completed: {} tasks, {} node operations, {} stat queries",
        final_tasks,
        final_node_ops,
        final_stat_queries
    );
    
    // Performance assertions
    assert!(final_tasks >= 100, "Should submit at least 100 tasks under high concurrency");
    assert!(final_node_ops >= 20, "Should perform at least 20 node operations");
    assert!(final_stat_queries >= 50, "Should perform at least 50 statistics queries");
    
    tracing::info!("✓ High concurrency operations stress test completed");
}

#[test]
#[ignore] // Stress test - run with `cargo test -- --ignored`
fn test_memory_pressure_and_limits() {
    init_tracing!();
    tracing::info!("Starting memory pressure and limits stress test");
    
    let mut config = DistributedCompilationConfig::default();
    config.coordinator_port = 9420;
    config.worker_ports = vec![9421, 9422];
    config.task_timeout_seconds = 60;
    
    let mut system = DistributedCompilationSystem::new(config).expect("Failed to create system");
    system.start().expect("Failed to start system");
    
    // Create tasks with large amounts of data to pressure memory
    let num_large_tasks = 200;
    let large_data_size = 1024 * 1024; // 1MB per task
    
    tracing::info!("Creating {} tasks with {}MB of data each", num_large_tasks, large_data_size / (1024 * 1024));
    
    let start_time = Instant::now();
    let mut task_ids = Vec::new();
    
    for i in 0..num_large_tasks {
        // Create tasks with large source file names and compilation flags
        let mut large_source_files = Vec::new();
        let mut large_flags = Vec::new();
        
        // Add many source files to increase memory usage
        for j in 0..50 {
            large_source_files.push(format!("large_file_{}_{}_with_very_long_name_that_consumes_memory.csd", i, j));
        }
        
        // Add many compilation flags
        for k in 0..100 {
            large_flags.push(format!("-DLARGE_DEFINE_{}_{}_WITH_LONG_VALUE={}", i, k, "x".repeat(100)));
        }
        
        let mut task = create_compilation_task(
            large_source_files,
            CompilationTarget::Object,
            large_flags,
        );
        
        // Add large dependencies list
        for d in 0..50 {
            task.dependencies.push(format!("dependency_{}_{}_with_long_path.so", i, d));
        }
        
        match system.submit_task(task) {
            Ok(task_id) => {
                task_ids.push(task_id);
            }
            Err(e) => {
                tracing::warn!("Failed to submit task {} due to memory pressure: {:?}", i, e);
                break;
            }
        }
        
        // Check memory usage periodically
        if i % 20 == 0 {
            if let Ok(stats) = system.get_statistics() {
                tracing::debug!(
                    "Memory pressure test progress: {} tasks submitted, {} total in system",
                    task_ids.len(),
                    stats.total_tasks
                );
            }
        }
        
        // Small delay to allow memory management
        if i % 10 == 0 {
            thread::sleep(Duration::from_millis(10));
        }
    }
    
    let submission_time = start_time.elapsed();
    tracing::info!(
        "Submitted {} memory-intensive tasks in {:.2} seconds",
        task_ids.len(),
        submission_time.as_secs_f64()
    );
    
    // Monitor memory usage and system behavior
    let monitoring_start = Instant::now();
    let max_monitoring_time = Duration::from_secs(30);
    
    while monitoring_start.elapsed() < max_monitoring_time {
        if let Ok(stats) = system.get_statistics() {
            let processed = stats.completed_tasks + stats.failed_tasks;
            
            tracing::info!(
                "Memory pressure monitoring: {}/{} processed, efficiency: {:.3}",
                processed,
                stats.total_tasks,
                stats.load_balancing_efficiency
            );
            
            // If we've processed a good portion, we can stop early
            if processed >= task_ids.len() / 2 {
                tracing::info!("Processed half of memory-intensive tasks, ending early");
                break;
            }
        }
        
        thread::sleep(Duration::from_secs(2));
    }
    
    let final_stats = system.get_statistics().expect("Failed to get final statistics");
    
    tracing::info!(
        "Memory pressure test completed: {} tasks created, {} processed",
        task_ids.len(),
        final_stats.completed_tasks + final_stats.failed_tasks
    );
    
    // System should handle memory pressure gracefully
    assert!(task_ids.len() >= 50, "Should handle at least 50 memory-intensive tasks");
    assert!(final_stats.total_tasks >= task_ids.len(), "System should track all submitted tasks");
    
    system.stop().expect("Failed to stop system");
    tracing::info!("✓ Memory pressure and limits stress test completed");
}

#[test]
#[ignore] // Stress test - run with `cargo test -- --ignored`
fn test_network_failure_simulation() {
    init_tracing!();
    tracing::info!("Starting network failure simulation stress test");
    
    let mut config = DistributedCompilationConfig::default();
    config.coordinator_port = 9430;
    config.worker_ports = vec![9431, 9432, 9433];
    config.heartbeat_interval_seconds = 2;
    config.task_timeout_seconds = 10;
    config.max_network_retries = 2;
    config.fault_tolerance_enabled = true;
    
    let mut system = DistributedCompilationSystem::new(config).expect("Failed to create system");
    system.start().expect("Failed to start system");
    
    // Register some reliable nodes
    let reliable_nodes = vec![
        CompilationNode {
            id: "reliable_1".to_string(),
            address: "127.0.0.1".to_string(),
            port: 9434,
            capabilities: NodeCapabilities {
                cpu_cores: 4,
                memory_gb: 8,
                disk_space_gb: 200,
                supported_targets: vec![CompilationTarget::Object, CompilationTarget::IR],
                compiler_versions: HashMap::new(),
                performance_score: 1.0,
            },
            status: NodeStatus::Online,
            current_load: 0.2,
            max_concurrent_tasks: 4,
            active_tasks: HashSet::new(),
            completed_tasks: 0,
            failed_tasks: 0,
            average_task_duration: Duration::from_millis(100),
            last_heartbeat: 0,
            is_coordinator: false,
        },
        CompilationNode {
            id: "reliable_2".to_string(),
            address: "127.0.0.1".to_string(),
            port: 9435,
            capabilities: NodeCapabilities {
                cpu_cores: 2,
                memory_gb: 4,
                disk_space_gb: 100,
                supported_targets: vec![CompilationTarget::Object],
                compiler_versions: HashMap::new(),
                performance_score: 0.8,
            },
            status: NodeStatus::Online,
            current_load: 0.1,
            max_concurrent_tasks: 2,
            active_tasks: HashSet::new(),
            completed_tasks: 0,
            failed_tasks: 0,
            average_task_duration: Duration::from_millis(150),
            last_heartbeat: 0,
            is_coordinator: false,
        },
    ];
    
    for node in &reliable_nodes {
        system.register_node(node.clone()).expect("Failed to register reliable node");
        tracing::info!("Registered reliable node: {}", node.id);
    }
    
    // Register unreliable nodes (with non-existent ports to simulate network failures)
    let unreliable_nodes = vec![
        CompilationNode {
            id: "unreliable_1".to_string(),
            address: "127.0.0.1".to_string(),
            port: 9999, // Non-existent port
            capabilities: NodeCapabilities {
                cpu_cores: 8,
                memory_gb: 16,
                disk_space_gb: 500,
                supported_targets: vec![CompilationTarget::Object, CompilationTarget::IR],
                compiler_versions: HashMap::new(),
                performance_score: 2.0,
            },
            status: NodeStatus::Online,
            current_load: 0.0,
            max_concurrent_tasks: 8,
            active_tasks: HashSet::new(),
            completed_tasks: 0,
            failed_tasks: 0,
            average_task_duration: Duration::from_millis(50),
            last_heartbeat: 0,
            is_coordinator: false,
        },
        CompilationNode {
            id: "unreliable_2".to_string(),
            address: "192.168.99.99".to_string(), // Non-existent IP
            port: 9436,
            capabilities: NodeCapabilities {
                cpu_cores: 4,
                memory_gb: 8,
                disk_space_gb: 200,
                supported_targets: vec![CompilationTarget::Object],
                compiler_versions: HashMap::new(),
                performance_score: 1.5,
            },
            status: NodeStatus::Online,
            current_load: 0.0,
            max_concurrent_tasks: 4,
            active_tasks: HashSet::new(),
            completed_tasks: 0,
            failed_tasks: 0,
            average_task_duration: Duration::from_millis(75),
            last_heartbeat: 0,
            is_coordinator: false,
        },
    ];
    
    for node in &unreliable_nodes {
        system.register_node(node.clone()).expect("Failed to register unreliable node");
        tracing::info!("Registered unreliable node: {} (will fail)", node.id);
    }
    
    // Submit tasks that will trigger network failures
    let num_tasks = 50;
    let mut tasks = Vec::new();
    
    for i in 0..num_tasks {
        tasks.push(create_compilation_task(
            vec![format!("network_failure_test_{}.csd", i)],
            CompilationTarget::Object,
            vec![],
        ));
    }
    
    let task_ids = system.submit_batch(tasks).expect("Failed to submit batch");
    tracing::info!("Submitted {} tasks for network failure testing", task_ids.len());
    
    // Monitor network failure handling
    let start_time = Instant::now();
    let monitoring_duration = Duration::from_secs(20);
    let mut last_stats_time = Instant::now();
    let mut failure_count = 0;
    let mut recovery_count = 0;
    
    while start_time.elapsed() < monitoring_duration {
        if last_stats_time.elapsed() >= Duration::from_secs(2) {
            let stats = system.get_statistics().expect("Failed to get statistics");
            let nodes = system.get_nodes().expect("Failed to get nodes");
            
            // Count failed nodes
            let mut current_failures = 0;
            for node in &nodes {
                match node.status {
                    NodeStatus::Error(_) | NodeStatus::Offline => {
                        current_failures += 1;
                    }
                    _ => {}
                }
            }
            
            if current_failures > failure_count {
                failure_count = current_failures;
                tracing::info!("Network failures detected: {} nodes failed", failure_count);
            }
            
            if stats.fault_recovery_count > recovery_count {
                recovery_count = stats.fault_recovery_count;
                tracing::info!("Fault recovery operations: {} total", recovery_count);
            }
            
            let processed = stats.completed_tasks + stats.failed_tasks;
            tracing::info!(
                "Network failure test: {}/{} processed, {} failures, {} recoveries",
                processed,
                stats.total_tasks,
                failure_count,
                recovery_count
            );
            
            last_stats_time = Instant::now();
        }
        
        thread::sleep(Duration::from_millis(500));
    }
    
    let final_stats = system.get_statistics().expect("Failed to get final statistics");
    tracing::info!(
        "Network failure simulation completed: {} failures detected, {} recovery operations",
        failure_count,
        final_stats.fault_recovery_count
    );
    
    // Verify that the system detected failures and attempted recovery
    assert!(failure_count > 0, "Should have detected network failures");
    assert!(final_stats.fault_recovery_count > 0 || final_stats.failed_tasks > 0, 
            "System should have attempted recovery or recorded failures");
    
    system.stop().expect("Failed to stop system");
    tracing::info!("✓ Network failure simulation stress test completed");
}

#[test]
#[ignore] // Stress test - run with `cargo test -- --ignored`
fn test_long_running_stability() {
    init_tracing!();
    tracing::info!("Starting long-running stability stress test");
    
    let mut config = DistributedCompilationConfig::default();
    config.coordinator_port = 9440;
    config.worker_ports = vec![9441, 9442];
    config.heartbeat_interval_seconds = 5;
    config.task_timeout_seconds = 30;
    config.work_stealing_enabled = true;
    config.fault_tolerance_enabled = true;
    
    let mut system = DistributedCompilationSystem::new(config).expect("Failed to create system");
    system.start().expect("Failed to start system");
    
    let system = Arc::new(system);
    let test_duration = Duration::from_secs(60); // Run for 1 minute
    let is_running = Arc::new(AtomicBool::new(true));
    
    let total_tasks_submitted = Arc::new(AtomicUsize::new(0));
    let total_statistics_queries = Arc::new(AtomicUsize::new(0));
    
    // Continuous task submission thread
    let system_clone = system.clone();
    let is_running_clone = is_running.clone();
    let tasks_submitted = total_tasks_submitted.clone();
    
    let task_submission_handle = thread::spawn(move || {
        let mut task_count = 0;
        
        while is_running_clone.load(Ordering::Relaxed) {
            // Create a small batch of tasks
            let mut tasks = Vec::new();
            for i in 0..5 {
                tasks.push(create_compilation_task(
                    vec![format!("stability_test_{}_{}.csd", task_count, i)],
                    match (task_count + i) % 4 {
                        0 => CompilationTarget::Object,
                        1 => CompilationTarget::IR,
                        2 => CompilationTarget::Assembly,
                        _ => CompilationTarget::Library,
                    },
                    vec![format!("-DSTABILITY_TEST={}", task_count)],
                ));
            }
            
            if let Ok(task_ids) = system_clone.submit_batch(tasks) {
                tasks_submitted.fetch_add(task_ids.len(), Ordering::Relaxed);
                task_count += 1;
            }
            
            // Variable delay to simulate realistic workload
            thread::sleep(Duration::from_millis(100 + (task_count % 500)));
        }
        
        tracing::info!("Task submission thread completed {} batches", task_count);
    });
    
    // Continuous monitoring thread
    let system_clone = system.clone();
    let is_running_clone = is_running.clone();
    let stats_queries = total_statistics_queries.clone();
    
    let monitoring_handle = thread::spawn(move || {
        let mut query_count = 0;
        
        while is_running_clone.load(Ordering::Relaxed) {
            // Query statistics
            if system_clone.get_statistics().is_ok() {
                query_count += 1;
                stats_queries.fetch_add(1, Ordering::Relaxed);
            }
            
            // Query nodes
            if system_clone.get_nodes().is_ok() {
                query_count += 1;
                stats_queries.fetch_add(1, Ordering::Relaxed);
            }
            
            thread::sleep(Duration::from_millis(1000));
        }
        
        tracing::info!("Monitoring thread completed {} queries", query_count);
    });
    
    // Dynamic node management thread
    let system_clone = system.clone();
    let is_running_clone = is_running.clone();
    
    let node_management_handle = thread::spawn(move || {
        let mut cycle_count = 0;
        
        while is_running_clone.load(Ordering::Relaxed) {
            // Add a temporary node
            let node = CompilationNode {
                id: format!("dynamic_node_{}", cycle_count),
                address: "127.0.0.1".to_string(),
                port: 9600 + (cycle_count % 100),
                capabilities: NodeCapabilities {
                    cpu_cores: 2 + (cycle_count % 4),
                    memory_gb: 4 + (cycle_count % 8),
                    disk_space_gb: 100,
                    supported_targets: vec![CompilationTarget::Object],
                    compiler_versions: HashMap::new(),
                    performance_score: 0.5 + (cycle_count as f64 % 1.0),
                },
                status: NodeStatus::Online,
                current_load: 0.0,
                max_concurrent_tasks: 2,
                active_tasks: HashSet::new(),
                completed_tasks: 0,
                failed_tasks: 0,
                average_task_duration: Duration::from_millis(100),
                last_heartbeat: 0,
                is_coordinator: false,
            };
            
            if system_clone.register_node(node.clone()).is_ok() {
                // Keep it for a while
                thread::sleep(Duration::from_millis(5000));
                
                // Remove it
                let _ = system_clone.unregister_node(&node.id);
            }
            
            cycle_count += 1;
            thread::sleep(Duration::from_millis(2000));
        }
        
        tracing::info!("Node management thread completed {} cycles", cycle_count);
    });
    
    // Main monitoring loop
    let start_time = Instant::now();
    let mut last_report_time = Instant::now();
    let report_interval = Duration::from_secs(10);
    
    while start_time.elapsed() < test_duration {
        if last_report_time.elapsed() >= report_interval {
            let current_tasks = total_tasks_submitted.load(Ordering::Relaxed);
            let current_queries = total_statistics_queries.load(Ordering::Relaxed);
            
            if let Ok(stats) = system.get_statistics() {
                let processed = stats.completed_tasks + stats.failed_tasks;
                let elapsed = start_time.elapsed().as_secs_f64();
                
                tracing::info!(
                    "Stability test ({:.0}s): {} tasks submitted, {} processed, {} queries, efficiency: {:.3}",
                    elapsed,
                    current_tasks,
                    processed,
                    current_queries,
                    stats.load_balancing_efficiency
                );
            }
            
            last_report_time = Instant::now();
        }
        
        thread::sleep(Duration::from_millis(1000));
    }
    
    // Stop all threads
    is_running.store(false, Ordering::Relaxed);
    
    // Wait for threads to complete
    task_submission_handle.join().expect("Task submission thread panicked");
    monitoring_handle.join().expect("Monitoring thread panicked");
    node_management_handle.join().expect("Node management thread panicked");
    
    let final_tasks = total_tasks_submitted.load(Ordering::Relaxed);
    let final_queries = total_statistics_queries.load(Ordering::Relaxed);
    let final_stats = system.get_statistics().expect("Failed to get final statistics");
    
    tracing::info!(
        "Long-running stability test completed: {} tasks submitted, {} processed, {} queries over {:.0} seconds",
        final_tasks,
        final_stats.completed_tasks + final_stats.failed_tasks,
        final_queries,
        test_duration.as_secs_f64()
    );
    
    // Stability assertions
    assert!(final_tasks >= 50, "Should submit at least 50 tasks during long run");
    assert!(final_queries >= 30, "Should perform at least 30 statistics queries");
    assert!(final_stats.total_tasks >= final_tasks, "System should track all submitted tasks");
    
    tracing::info!("✓ Long-running stability stress test completed");
}

#[test]
#[ignore] // Stress test - run with `cargo test -- --ignored`
fn test_extreme_load_balancing_scenarios() {
    init_tracing!();
    tracing::info!("Starting extreme load balancing scenarios stress test");
    
    let mut config = DistributedCompilationConfig::default();
    config.coordinator_port = 9450;
    config.worker_ports = vec![];
    config.load_balancing_strategy = LoadBalancingStrategy::WorkStealing;
    config.work_stealing_enabled = true;
    config.task_timeout_seconds = 60;
    
    let mut system = DistributedCompilationSystem::new(config).expect("Failed to create system");
    system.start().expect("Failed to start system");
    
    // Create nodes with vastly different capabilities
    let nodes = vec![
        // Super high-performance node
        CompilationNode {
            id: "super_node".to_string(),
            address: "127.0.0.1".to_string(),
            port: 9451,
            capabilities: NodeCapabilities {
                cpu_cores: 32,
                memory_gb: 128,
                disk_space_gb: 5000,
                supported_targets: vec![
                    CompilationTarget::Object,
                    CompilationTarget::IR,
                    CompilationTarget::Assembly,
                    CompilationTarget::Executable,
                    CompilationTarget::Library,
                ],
                compiler_versions: HashMap::new(),
                performance_score: 10.0,
            },
            status: NodeStatus::Online,
            current_load: 0.0,
            max_concurrent_tasks: 32,
            active_tasks: HashSet::new(),
            completed_tasks: 0,
            failed_tasks: 0,
            average_task_duration: Duration::from_millis(25),
            last_heartbeat: 0,
            is_coordinator: false,
        },
        // Multiple low-performance nodes
        CompilationNode {
            id: "slow_node_1".to_string(),
            address: "127.0.0.1".to_string(),
            port: 9452,
            capabilities: NodeCapabilities {
                cpu_cores: 1,
                memory_gb: 1,
                disk_space_gb: 10,
                supported_targets: vec![CompilationTarget::Object],
                compiler_versions: HashMap::new(),
                performance_score: 0.1,
            },
            status: NodeStatus::Online,
            current_load: 0.0,
            max_concurrent_tasks: 1,
            active_tasks: HashSet::new(),
            completed_tasks: 0,
            failed_tasks: 0,
            average_task_duration: Duration::from_millis(1000),
            last_heartbeat: 0,
            is_coordinator: false,
        },
        CompilationNode {
            id: "slow_node_2".to_string(),
            address: "127.0.0.1".to_string(),
            port: 9453,
            capabilities: NodeCapabilities {
                cpu_cores: 1,
                memory_gb: 2,
                disk_space_gb: 20,
                supported_targets: vec![CompilationTarget::Object, CompilationTarget::IR],
                compiler_versions: HashMap::new(),
                performance_score: 0.2,
            },
            status: NodeStatus::Online,
            current_load: 0.0,
            max_concurrent_tasks: 1,
            active_tasks: HashSet::new(),
            completed_tasks: 0,
            failed_tasks: 0,
            average_task_duration: Duration::from_millis(800),
            last_heartbeat: 0,
            is_coordinator: false,
        },
        // Medium performance nodes
        CompilationNode {
            id: "medium_node_1".to_string(),
            address: "127.0.0.1".to_string(),
            port: 9454,
            capabilities: NodeCapabilities {
                cpu_cores: 4,
                memory_gb: 8,
                disk_space_gb: 200,
                supported_targets: vec![CompilationTarget::Object, CompilationTarget::IR],
                compiler_versions: HashMap::new(),
                performance_score: 1.0,
            },
            status: NodeStatus::Online,
            current_load: 0.0,
            max_concurrent_tasks: 4,
            active_tasks: HashSet::new(),
            completed_tasks: 0,
            failed_tasks: 0,
            average_task_duration: Duration::from_millis(200),
            last_heartbeat: 0,
            is_coordinator: false,
        },
        CompilationNode {
            id: "medium_node_2".to_string(),
            address: "127.0.0.1".to_string(),
            port: 9455,
            capabilities: NodeCapabilities {
                cpu_cores: 6,
                memory_gb: 12,
                disk_space_gb: 300,
                supported_targets: vec![
                    CompilationTarget::Object,
                    CompilationTarget::IR,
                    CompilationTarget::Assembly,
                ],
                compiler_versions: HashMap::new(),
                performance_score: 1.5,
            },
            status: NodeStatus::Online,
            current_load: 0.0,
            max_concurrent_tasks: 6,
            active_tasks: HashSet::new(),
            completed_tasks: 0,
            failed_tasks: 0,
            average_task_duration: Duration::from_millis(150),
            last_heartbeat: 0,
            is_coordinator: false,
        },
    ];
    
    for node in &nodes {
        system.register_node(node.clone()).expect("Failed to register node");
        tracing::info!(
            "Registered node: {} (cores: {}, score: {:.1})",
            node.id,
            node.capabilities.cpu_cores,
            node.capabilities.performance_score
        );
    }
    
    // Create a large number of mixed tasks
    let num_tasks = 300;
    let mut tasks = Vec::new();
    
    for i in 0..num_tasks {
        let target = match i % 5 {
            0 => CompilationTarget::Object,
            1 => CompilationTarget::IR,
            2 => CompilationTarget::Assembly,
            3 => CompilationTarget::Library,
            _ => CompilationTarget::Executable,
        };
        
        // Vary task complexity
        let flags = if i % 3 == 0 {
            vec!["-O0".to_string()] // Simple task
        } else if i % 3 == 1 {
            vec!["-O2".to_string(), "-g".to_string()] // Medium task
        } else {
            vec!["-O3".to_string(), "-flto".to_string(), "-march=native".to_string()] // Complex task
        };
        
        tasks.push(create_compilation_task(
            vec![format!("load_balance_test_{}.csd", i)],
            target,
            flags,
        ));
    }
    
    // Submit all tasks at once to test load balancing under pressure
    let task_ids = system.submit_batch(tasks).expect("Failed to submit batch");
    tracing::info!("Submitted {} mixed tasks for extreme load balancing test", task_ids.len());
    
    // Monitor load distribution
    let start_time = Instant::now();
    let monitoring_duration = Duration::from_secs(30);
    let mut last_report = Instant::now();
    
    while start_time.elapsed() < monitoring_duration {
        if last_report.elapsed() >= Duration::from_secs(3) {
            let stats = system.get_statistics().expect("Failed to get statistics");
            let nodes = system.get_nodes().expect("Failed to get nodes");
            
            // Calculate load distribution metrics
            let total_active_tasks: usize = nodes.iter().map(|n| n.active_tasks.len()).sum();
            let max_load = nodes.iter().map(|n| n.current_load).fold(0.0, f64::max);
            let min_load = nodes.iter().map(|n| n.current_load).fold(1.0, f64::min);
            let load_variance = if nodes.len() > 1 {
                let avg_load: f64 = nodes.iter().map(|n| n.current_load).sum::<f64>() / nodes.len() as f64;
                nodes.iter().map(|n| (n.current_load - avg_load).powi(2)).sum::<f64>() / nodes.len() as f64
            } else {
                0.0
            };
            
            tracing::info!(
                "Load balancing: processed={}/{}, active={}, efficiency={:.3}, load range=[{:.2}, {:.2}], variance={:.3}",
                stats.completed_tasks + stats.failed_tasks,
                stats.total_tasks,
                total_active_tasks,
                stats.load_balancing_efficiency,
                min_load,
                max_load,
                load_variance
            );
            
            // Log individual node loads
            for node in &nodes {
                tracing::debug!(
                    "Node {}: load={:.2}, active={}, completed={}, score={:.1}",
                    node.id,
                    node.current_load,
                    node.active_tasks.len(),
                    node.completed_tasks,
                    node.capabilities.performance_score
                );
            }
            
            last_report = Instant::now();
        }
        
        thread::sleep(Duration::from_millis(500));
    }
    
    let final_stats = system.get_statistics().expect("Failed to get final statistics");
    let final_nodes = system.get_nodes().expect("Failed to get final nodes");
    
    // Calculate final distribution metrics
    let total_completed: usize = final_nodes.iter().map(|n| n.completed_tasks).sum();
    let super_node_completed = final_nodes
        .iter()
        .find(|n| n.id == "super_node")
        .map(|n| n.completed_tasks)
        .unwrap_or(0);
    
    tracing::info!(
        "Extreme load balancing completed: {} total processed, super node handled {} ({}%)",
        final_stats.completed_tasks + final_stats.failed_tasks,
        super_node_completed,
        if total_completed > 0 { super_node_completed * 100 / total_completed } else { 0 }
    );
    
    // Performance assertions
    assert!(final_stats.load_balancing_efficiency >= 0.3, 
            "Load balancing efficiency should be at least 0.3, got {:.3}", 
            final_stats.load_balancing_efficiency);
    assert!(final_stats.total_tasks >= num_tasks, "Should track all submitted tasks");
    
    // The super node should handle a significant portion due to its high performance
    assert!(super_node_completed >= total_completed / 10, 
            "Super node should handle at least 10% of tasks");
    
    system.stop().expect("Failed to stop system");
    tracing::info!("✓ Extreme load balancing scenarios stress test completed");
}
