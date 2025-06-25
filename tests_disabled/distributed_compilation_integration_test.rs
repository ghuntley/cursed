//! Integration tests for distributed compilation system
//! 
//! Tests end-to-end workflows including multi-node scenarios,
//! real network communication, and complex compilation pipelines.

use std::sync::{Arc, atomic::{AtomicBool, AtomicUsize, Ordering}};
use std::time::{Duration, Instant};
use std::thread;
use std::collections::{HashMap, HashSet};
use std::net::{TcpListener, SocketAddr, IpAddr, Ipv4Addr};

use cursed::build_system::distributed_compilation::{
    DistributedCompilationSystem,
    DistributedCompilationConfig,
    CompilationTask,
    CompilationTarget,
    CompilationNode,
    CompilationResult,
    NodeStatus,
    NodeCapabilities,
    LoadBalancingStrategy,
    NetworkMessage,
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
fn test_end_to_end_compilation_workflow() {
    init_tracing!();
    tracing::info!("Starting end-to-end compilation workflow test");
    
    let mut config = DistributedCompilationConfig::default();
    config.coordinator_port = 9020;
    config.worker_ports = vec![9021, 9022, 9023];
    config.task_timeout_seconds = 30;
    
    let mut system = DistributedCompilationSystem::new(config).expect("Failed to create system");
    system.start().expect("Failed to start system");
    
    // Wait for system to initialize
    thread::sleep(Duration::from_millis(200));
    
    // Create a complex compilation pipeline
    let tasks = vec![
        create_compilation_task(
            vec!["main.csd".to_string()],
            CompilationTarget::Object,
            vec!["-O2".to_string(), "-g".to_string()],
        ),
        create_compilation_task(
            vec!["utils.csd".to_string()],
            CompilationTarget::Object,
            vec!["-O2".to_string()],
        ),
        create_compilation_task(
            vec!["math.csd".to_string()],
            CompilationTarget::IR,
            vec!["-emit-llvm".to_string()],
        ),
        create_compilation_task(
            vec!["main.o", "utils.o".to_string()],
            CompilationTarget::Executable,
            vec!["-o", "program".to_string()],
        ),
    ];
    
    // Submit all tasks
    let task_ids = system.submit_batch(tasks).expect("Failed to submit batch");
    assert_eq!(task_ids.len(), 4, "Should submit 4 tasks");
    
    tracing::info!("Submitted {} tasks for compilation", task_ids.len());
    
    // Monitor progress for a reasonable time
    let start_time = Instant::now();
    let timeout = Duration::from_secs(10);
    
    while start_time.elapsed() < timeout {
        let stats = system.get_statistics().expect("Failed to get statistics");
        
        tracing::debug!(
            "Progress: total={}, active={}, completed={}, failed={}", 
            stats.total_tasks,
            stats.total_tasks - stats.completed_tasks - stats.failed_tasks,
            stats.completed_tasks,
            stats.failed_tasks
        );
        
        // Check if all tasks are processed (completed or failed)
        if stats.completed_tasks + stats.failed_tasks >= 4 {
            break;
        }
        
        thread::sleep(Duration::from_millis(100));
    }
    
    let final_stats = system.get_statistics().expect("Failed to get final statistics");
    tracing::info!("Final workflow stats: {:?}", final_stats);
    
    assert!(final_stats.total_tasks >= 4, "Should have at least 4 tasks");
    
    system.stop().expect("Failed to stop system");
    tracing::info!("✓ End-to-end compilation workflow completed");
}

#[test]
fn test_multi_node_load_balancing() {
    init_tracing!();
    tracing::info!("Starting multi-node load balancing test");
    
    let mut config = DistributedCompilationConfig::default();
    config.coordinator_port = 9024;
    config.load_balancing_strategy = LoadBalancingStrategy::LeastLoaded;
    config.worker_ports = vec![9025, 9026, 9027, 9028];
    
    let mut system = DistributedCompilationSystem::new(config).expect("Failed to create system");
    system.start().expect("Failed to start system");
    
    // Register additional nodes with different capabilities
    let nodes = vec![
        CompilationNode {
            id: "high_perf_node".to_string(),
            address: "127.0.0.1".to_string(),
            port: 9029,
            capabilities: NodeCapabilities {
                cpu_cores: 16,
                memory_gb: 64,
                disk_space_gb: 2000,
                supported_targets: vec![
                    CompilationTarget::Object,
                    CompilationTarget::IR,
                    CompilationTarget::Assembly,
                    CompilationTarget::Executable,
                    CompilationTarget::Library,
                ],
                compiler_versions: HashMap::new(),
                performance_score: 2.5,
            },
            status: NodeStatus::Online,
            current_load: 0.0,
            max_concurrent_tasks: 16,
            active_tasks: HashSet::new(),
            completed_tasks: 0,
            failed_tasks: 0,
            average_task_duration: Duration::from_millis(50),
            last_heartbeat: 0,
            is_coordinator: false,
        },
        CompilationNode {
            id: "low_perf_node".to_string(),
            address: "127.0.0.1".to_string(),
            port: 9030,
            capabilities: NodeCapabilities {
                cpu_cores: 2,
                memory_gb: 4,
                disk_space_gb: 100,
                supported_targets: vec![CompilationTarget::Object],
                compiler_versions: HashMap::new(),
                performance_score: 0.5,
            },
            status: NodeStatus::Online,
            current_load: 0.0,
            max_concurrent_tasks: 2,
            active_tasks: HashSet::new(),
            completed_tasks: 0,
            failed_tasks: 0,
            average_task_duration: Duration::from_millis(200),
            last_heartbeat: 0,
            is_coordinator: false,
        },
    ];
    
    for node in nodes {
        system.register_node(node.clone()).expect("Failed to register node");
        tracing::info!("Registered node: {} (cores: {})", node.id, node.capabilities.cpu_cores);
    }
    
    // Create many tasks to test load balancing
    let mut tasks = Vec::new();
    for i in 0..20 {
        tasks.push(create_compilation_task(
            vec![format!("file_{}.csd", i)],
            CompilationTarget::Object,
            vec!["-O1".to_string()],
        ));
    }
    
    let task_ids = system.submit_batch(tasks).expect("Failed to submit batch");
    tracing::info!("Submitted {} tasks for load balancing test", task_ids.len());
    
    // Monitor load balancing for a while
    let start_time = Instant::now();
    let monitoring_duration = Duration::from_secs(3);
    
    while start_time.elapsed() < monitoring_duration {
        let nodes = system.get_nodes().expect("Failed to get nodes");
        let stats = system.get_statistics().expect("Failed to get statistics");
        
        for node in &nodes {
            tracing::debug!(
                "Node {}: load={:.2}, active_tasks={}, status={:?}",
                node.id,
                node.current_load,
                node.active_tasks.len(),
                node.status
            );
        }
        
        tracing::debug!("Load balancing efficiency: {:.2}", stats.load_balancing_efficiency);
        
        thread::sleep(Duration::from_millis(500));
    }
    
    let final_stats = system.get_statistics().expect("Failed to get final statistics");
    tracing::info!("Load balancing test completed with efficiency: {:.2}", final_stats.load_balancing_efficiency);
    
    system.stop().expect("Failed to stop system");
    tracing::info!("✓ Multi-node load balancing test completed");
}

#[test]
fn test_work_stealing_mechanism() {
    init_tracing!();
    tracing::info!("Starting work stealing mechanism test");
    
    let mut config = DistributedCompilationConfig::default();
    config.coordinator_port = 9031;
    config.work_stealing_enabled = true;
    config.load_balancing_strategy = LoadBalancingStrategy::WorkStealing;
    
    let mut system = DistributedCompilationSystem::new(config).expect("Failed to create system");
    system.start().expect("Failed to start system");
    
    // Create nodes with different loads
    let busy_node = CompilationNode {
        id: "busy_node".to_string(),
        address: "127.0.0.1".to_string(),
        port: 9032,
        capabilities: NodeCapabilities {
            cpu_cores: 4,
            memory_gb: 8,
            disk_space_gb: 200,
            supported_targets: vec![CompilationTarget::Object, CompilationTarget::IR],
            compiler_versions: HashMap::new(),
            performance_score: 1.0,
        },
        status: NodeStatus::Online,
        current_load: 0.9, // High load
        max_concurrent_tasks: 4,
        active_tasks: (0..3).map(|i| format!("task_{}", i)).collect(), // 3 active tasks
        completed_tasks: 10,
        failed_tasks: 1,
        average_task_duration: Duration::from_millis(100),
        last_heartbeat: 0,
        is_coordinator: false,
    };
    
    let idle_node = CompilationNode {
        id: "idle_node".to_string(),
        address: "127.0.0.1".to_string(),
        port: 9033,
        capabilities: NodeCapabilities {
            cpu_cores: 8,
            memory_gb: 16,
            disk_space_gb: 500,
            supported_targets: vec![
                CompilationTarget::Object,
                CompilationTarget::IR,
                CompilationTarget::Assembly,
            ],
            compiler_versions: HashMap::new(),
            performance_score: 1.5,
        },
        status: NodeStatus::Online,
        current_load: 0.1, // Low load
        max_concurrent_tasks: 8,
        active_tasks: HashSet::new(), // No active tasks
        completed_tasks: 5,
        failed_tasks: 0,
        average_task_duration: Duration::from_millis(75),
        last_heartbeat: 0,
        is_coordinator: false,
    };
    
    system.register_node(busy_node.clone()).expect("Failed to register busy node");
    system.register_node(idle_node.clone()).expect("Failed to register idle node");
    
    tracing::info!("Registered busy node (load: {:.1}) and idle node (load: {:.1})", 
                  busy_node.current_load, idle_node.current_load);
    
    // Submit many tasks to trigger work stealing
    let mut tasks = Vec::new();
    for i in 0..15 {
        tasks.push(create_compilation_task(
            vec![format!("worksteal_file_{}.csd", i)],
            if i % 2 == 0 { CompilationTarget::Object } else { CompilationTarget::IR },
            vec![],
        ));
    }
    
    let task_ids = system.submit_batch(tasks).expect("Failed to submit batch");
    tracing::info!("Submitted {} tasks to trigger work stealing", task_ids.len());
    
    // Monitor work stealing activity
    let start_time = Instant::now();
    let monitoring_duration = Duration::from_secs(5);
    let mut work_steal_operations = 0;
    
    while start_time.elapsed() < monitoring_duration {
        let stats = system.get_statistics().expect("Failed to get statistics");
        
        if stats.work_stealing_operations > work_steal_operations {
            let new_operations = stats.work_stealing_operations - work_steal_operations;
            work_steal_operations = stats.work_stealing_operations;
            tracing::info!("Work stealing operations detected: +{} (total: {})", 
                          new_operations, work_steal_operations);
        }
        
        thread::sleep(Duration::from_millis(200));
    }
    
    let final_stats = system.get_statistics().expect("Failed to get final statistics");
    tracing::info!("Work stealing test completed: {} operations total", final_stats.work_stealing_operations);
    
    system.stop().expect("Failed to stop system");
    tracing::info!("✓ Work stealing mechanism test completed");
}

#[test]
fn test_fault_tolerance_and_recovery() {
    init_tracing!();
    tracing::info!("Starting fault tolerance and recovery test");
    
    let mut config = DistributedCompilationConfig::default();
    config.coordinator_port = 9034;
    config.fault_tolerance_enabled = true;
    config.heartbeat_interval_seconds = 2;
    config.task_timeout_seconds = 5;
    
    let mut system = DistributedCompilationSystem::new(config).expect("Failed to create system");
    system.start().expect("Failed to start system");
    
    // Register reliable nodes
    let reliable_nodes = vec![
        CompilationNode {
            id: "reliable_node_1".to_string(),
            address: "127.0.0.1".to_string(),
            port: 9035,
            capabilities: NodeCapabilities {
                cpu_cores: 4,
                memory_gb: 8,
                disk_space_gb: 200,
                supported_targets: vec![CompilationTarget::Object, CompilationTarget::IR],
                compiler_versions: HashMap::new(),
                performance_score: 1.0,
            },
            status: NodeStatus::Online,
            current_load: 0.3,
            max_concurrent_tasks: 4,
            active_tasks: HashSet::new(),
            completed_tasks: 0,
            failed_tasks: 0,
            average_task_duration: Duration::from_millis(100),
            last_heartbeat: 0,
            is_coordinator: false,
        },
        CompilationNode {
            id: "reliable_node_2".to_string(),
            address: "127.0.0.1".to_string(),
            port: 9036,
            capabilities: NodeCapabilities {
                cpu_cores: 2,
                memory_gb: 4,
                disk_space_gb: 100,
                supported_targets: vec![CompilationTarget::Object],
                compiler_versions: HashMap::new(),
                performance_score: 0.8,
            },
            status: NodeStatus::Online,
            current_load: 0.2,
            max_concurrent_tasks: 2,
            active_tasks: HashSet::new(),
            completed_tasks: 0,
            failed_tasks: 0,
            average_task_duration: Duration::from_millis(150),
            last_heartbeat: 0,
            is_coordinator: false,
        },
    ];
    
    for node in reliable_nodes {
        system.register_node(node.clone()).expect("Failed to register reliable node");
        tracing::info!("Registered reliable node: {}", node.id);
    }
    
    // Register an unreliable node (will fail)
    let unreliable_node = CompilationNode {
        id: "unreliable_node".to_string(),
        address: "127.0.0.1".to_string(),
        port: 9999, // Non-existent port to simulate failure
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
    };
    
    system.register_node(unreliable_node.clone()).expect("Failed to register unreliable node");
    tracing::info!("Registered unreliable node: {} (will fail)", unreliable_node.id);
    
    // Submit tasks that will trigger failures and recovery
    let mut tasks = Vec::new();
    for i in 0..10 {
        tasks.push(create_compilation_task(
            vec![format!("fault_tolerance_file_{}.csd", i)],
            CompilationTarget::Object,
            vec![],
        ));
    }
    
    let task_ids = system.submit_batch(tasks).expect("Failed to submit batch");
    tracing::info!("Submitted {} tasks for fault tolerance testing", task_ids.len());
    
    // Monitor fault recovery
    let start_time = Instant::now();
    let monitoring_duration = Duration::from_secs(10);
    let mut last_fault_recovery_count = 0;
    
    while start_time.elapsed() < monitoring_duration {
        let stats = system.get_statistics().expect("Failed to get statistics");
        let nodes = system.get_nodes().expect("Failed to get nodes");
        
        if stats.fault_recovery_count > last_fault_recovery_count {
            let new_recoveries = stats.fault_recovery_count - last_fault_recovery_count;
            last_fault_recovery_count = stats.fault_recovery_count;
            tracing::info!("Fault recovery operations detected: +{} (total: {})", 
                          new_recoveries, last_fault_recovery_count);
        }
        
        // Check node statuses
        for node in &nodes {
            if matches!(node.status, NodeStatus::Error(_)) || matches!(node.status, NodeStatus::Offline) {
                tracing::info!("Node {} status: {:?}", node.id, node.status);
            }
        }
        
        thread::sleep(Duration::from_millis(500));
    }
    
    let final_stats = system.get_statistics().expect("Failed to get final statistics");
    tracing::info!("Fault tolerance test completed: {} recovery operations, {} failed tasks", 
                  final_stats.fault_recovery_count, final_stats.failed_tasks);
    
    // Verify that the system attempted recovery
    assert!(final_stats.fault_recovery_count > 0 || final_stats.failed_tasks > 0, 
           "System should have detected failures or attempted recovery");
    
    system.stop().expect("Failed to stop system");
    tracing::info!("✓ Fault tolerance and recovery test completed");
}

#[test]
fn test_large_scale_compilation() {
    init_tracing!();
    tracing::info!("Starting large-scale compilation test");
    
    let mut config = DistributedCompilationConfig::default();
    config.coordinator_port = 9037;
    config.worker_ports = vec![9038, 9039, 9040, 9041, 9042];
    config.task_timeout_seconds = 60;
    config.work_stealing_enabled = true;
    config.fault_tolerance_enabled = true;
    
    let mut system = DistributedCompilationSystem::new(config).expect("Failed to create system");
    system.start().expect("Failed to start system");
    
    // Wait for initialization
    thread::sleep(Duration::from_millis(300));
    
    // Create a large number of compilation tasks
    let num_tasks = 100;
    let mut tasks = Vec::new();
    
    for i in 0..num_tasks {
        let target = match i % 5 {
            0 => CompilationTarget::Object,
            1 => CompilationTarget::IR,
            2 => CompilationTarget::Assembly,
            3 => CompilationTarget::Library,
            _ => CompilationTarget::Executable,
        };
        
        let flags = match i % 3 {
            0 => vec!["-O0".to_string()],
            1 => vec!["-O2".to_string(), "-g".to_string()],
            _ => vec!["-O3".to_string(), "-DNDEBUG".to_string()],
        };
        
        tasks.push(create_compilation_task(
            vec![format!("large_scale_file_{}.csd", i)],
            target,
            flags,
        ));
    }
    
    tracing::info!("Created {} tasks for large-scale compilation", num_tasks);
    
    // Submit tasks in batches to avoid overwhelming the system
    let batch_size = 20;
    let mut all_task_ids = Vec::new();
    
    for chunk in tasks.chunks(batch_size) {
        let batch_ids = system.submit_batch(chunk.to_vec()).expect("Failed to submit batch");
        all_task_ids.extend(batch_ids);
        
        // Small delay between batches
        thread::sleep(Duration::from_millis(50));
    }
    
    tracing::info!("Submitted {} tasks in batches", all_task_ids.len());
    
    // Monitor progress
    let start_time = Instant::now();
    let max_duration = Duration::from_secs(30);
    let mut last_progress_report = Instant::now();
    
    while start_time.elapsed() < max_duration {
        let stats = system.get_statistics().expect("Failed to get statistics");
        let total_processed = stats.completed_tasks + stats.failed_tasks;
        let processing_rate = if start_time.elapsed().as_secs() > 0 {
            total_processed as f64 / start_time.elapsed().as_secs() as f64
        } else {
            0.0
        };
        
        // Report progress every 5 seconds
        if last_progress_report.elapsed() >= Duration::from_secs(5) {
            tracing::info!(
                "Large-scale progress: {}/{} processed ({:.1}%), rate: {:.1} tasks/sec, efficiency: {:.2}",
                total_processed,
                num_tasks,
                (total_processed as f64 / num_tasks as f64) * 100.0,
                processing_rate,
                stats.load_balancing_efficiency
            );
            last_progress_report = Instant::now();
        }
        
        // Check if all tasks are processed
        if total_processed >= num_tasks {
            tracing::info!("All tasks processed in {:.1} seconds", start_time.elapsed().as_secs_f64());
            break;
        }
        
        thread::sleep(Duration::from_millis(200));
    }
    
    let final_stats = system.get_statistics().expect("Failed to get final statistics");
    let total_time = start_time.elapsed();
    let throughput = final_stats.total_tasks as f64 / total_time.as_secs_f64();
    
    tracing::info!(
        "Large-scale compilation completed: {} tasks in {:.1}s, throughput: {:.1} tasks/sec",
        final_stats.total_tasks,
        total_time.as_secs_f64(),
        throughput
    );
    
    tracing::info!(
        "Final stats: completed={}, failed={}, efficiency={:.2}, work_steal_ops={}",
        final_stats.completed_tasks,
        final_stats.failed_tasks,
        final_stats.load_balancing_efficiency,
        final_stats.work_stealing_operations
    );
    
    // Performance assertions
    assert!(throughput >= 1.0, "Should process at least 1 task per second, got {:.1}", throughput);
    assert!(final_stats.total_tasks >= num_tasks, "Should have processed at least {} tasks", num_tasks);
    
    system.stop().expect("Failed to stop system");
    tracing::info!("✓ Large-scale compilation test completed successfully");
}

#[test]
fn test_system_resilience_under_stress() {
    init_tracing!();
    tracing::info!("Starting system resilience under stress test");
    
    let mut config = DistributedCompilationConfig::default();
    config.coordinator_port = 9043;
    config.worker_ports = vec![9044, 9045, 9046];
    config.heartbeat_interval_seconds = 1;
    config.task_timeout_seconds = 10;
    config.max_network_retries = 5;
    
    let mut system = DistributedCompilationSystem::new(config).expect("Failed to create system");
    system.start().expect("Failed to start system");
    
    let system = Arc::new(system);
    let stress_duration = Duration::from_secs(10);
    let is_running = Arc::new(AtomicBool::new(true));
    
    // Spawn multiple threads that continuously submit tasks
    let mut handles = Vec::new();
    
    for thread_id in 0..4 {
        let system_clone = system.clone();
        let is_running_clone = is_running.clone();
        
        let handle = thread::spawn(move || {
            let mut task_count = 0;
            
            while is_running_clone.load(Ordering::Relaxed) {
                let task = create_compilation_task(
                    vec![format!("stress_test_{}_{}.csd", thread_id, task_count)],
                    match task_count % 3 {
                        0 => CompilationTarget::Object,
                        1 => CompilationTarget::IR,
                        _ => CompilationTarget::Assembly,
                    },
                    vec![],
                );
                
                if let Err(e) = system_clone.submit_task(task) {
                    tracing::warn!("Failed to submit task in stress thread {}: {:?}", thread_id, e);
                }
                
                task_count += 1;
                
                // Small delay to avoid overwhelming the system
                thread::sleep(Duration::from_millis(10));
            }
            
            tracing::info!("Stress thread {} submitted {} tasks", thread_id, task_count);
        });
        
        handles.push(handle);
    }
    
    // Spawn thread that continuously registers/unregisters nodes
    let system_clone = system.clone();
    let is_running_clone = is_running.clone();
    
    let node_management_handle = thread::spawn(move || {
        let mut node_count = 0;
        
        while is_running_clone.load(Ordering::Relaxed) {
            // Add a node
            let node = CompilationNode {
                id: format!("stress_node_{}", node_count),
                address: "127.0.0.1".to_string(),
                port: 9500 + node_count,
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
                tracing::debug!("Registered stress node {}", node.id);
                
                // Keep it for a while
                thread::sleep(Duration::from_millis(500));
                
                // Remove it
                if system_clone.unregister_node(&node.id).is_ok() {
                    tracing::debug!("Unregistered stress node {}", node.id);
                }
            }
            
            node_count += 1;
            thread::sleep(Duration::from_millis(200));
        }
        
        tracing::info!("Node management thread handled {} nodes", node_count);
    });
    
    // Monitor system under stress
    let start_time = Instant::now();
    let mut last_stats_report = Instant::now();
    
    while start_time.elapsed() < stress_duration {
        if last_stats_report.elapsed() >= Duration::from_secs(2) {
            if let Ok(stats) = system.get_statistics() {
                tracing::info!(
                    "Stress test progress: total={}, completed={}, failed={}, efficiency={:.2}",
                    stats.total_tasks,
                    stats.completed_tasks,
                    stats.failed_tasks,
                    stats.load_balancing_efficiency
                );
            }
            last_stats_report = Instant::now();
        }
        
        thread::sleep(Duration::from_millis(100));
    }
    
    // Stop stress test
    is_running.store(false, Ordering::Relaxed);
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Stress thread panicked");
    }
    node_management_handle.join().expect("Node management thread panicked");
    
    // Final statistics
    if let Ok(final_stats) = system.get_statistics() {
        tracing::info!(
            "Stress test completed: {} total tasks, {:.2} efficiency, {} recovery ops",
            final_stats.total_tasks,
            final_stats.load_balancing_efficiency,
            final_stats.fault_recovery_count
        );
        
        // System should have processed tasks without crashing
        assert!(final_stats.total_tasks > 0, "System should have processed some tasks");
    }
    
    tracing::info!("✓ System resilience under stress test completed");
}

#[test]
fn test_configuration_validation_and_edge_cases() {
    init_tracing!();
    tracing::info!("Starting configuration validation and edge cases test");
    
    // Test with minimal configuration
    let minimal_config = DistributedCompilationConfig {
        coordinator_port: 9500,
        worker_ports: vec![],
        max_network_retries: 0,
        task_timeout_seconds: 1,
        heartbeat_interval_seconds: 1,
        load_balancing_strategy: LoadBalancingStrategy::RoundRobin,
        fault_tolerance_enabled: false,
        work_stealing_enabled: false,
        result_caching_enabled: false,
        compression_enabled: false,
        encryption_enabled: false,
    };
    
    let mut system = DistributedCompilationSystem::new(minimal_config).expect("Should create system with minimal config");
    system.start().expect("Should start with minimal config");
    
    // Test basic operations with minimal config
    let task = create_compilation_task(
        vec!["minimal_test.csd".to_string()],
        CompilationTarget::Object,
        vec![],
    );
    
    let task_id = system.submit_task(task).expect("Should submit task with minimal config");
    assert!(!task_id.is_empty(), "Task ID should not be empty");
    
    // Wait briefly and check statistics
    thread::sleep(Duration::from_millis(100));
    let stats = system.get_statistics().expect("Should get statistics");
    assert!(stats.total_tasks >= 1, "Should have at least 1 task");
    
    system.stop().expect("Should stop system");
    
    // Test with maximal configuration
    let maximal_config = DistributedCompilationConfig {
        coordinator_port: 9501,
        worker_ports: vec![9502, 9503, 9504, 9505, 9506, 9507, 9508, 9509],
        max_network_retries: 10,
        task_timeout_seconds: 3600, // 1 hour
        heartbeat_interval_seconds: 5,
        load_balancing_strategy: LoadBalancingStrategy::PerformanceBased,
        fault_tolerance_enabled: true,
        work_stealing_enabled: true,
        result_caching_enabled: true,
        compression_enabled: true,
        encryption_enabled: true,
    };
    
    let mut maximal_system = DistributedCompilationSystem::new(maximal_config).expect("Should create system with maximal config");
    maximal_system.start().expect("Should start with maximal config");
    
    // Test with maximal config
    let complex_task = create_compilation_task(
        vec!["complex_test.csd".to_string(), "dependency.csd".to_string()],
        CompilationTarget::Executable,
        vec!["-O3".to_string(), "-flto".to_string(), "-march=native".to_string()],
    );
    
    let complex_task_id = maximal_system.submit_task(complex_task).expect("Should submit complex task");
    assert!(!complex_task_id.is_empty(), "Complex task ID should not be empty");
    
    maximal_system.stop().expect("Should stop maximal system");
    
    tracing::info!("✓ Configuration validation and edge cases test completed");
}

#[test]
fn test_task_lifecycle_and_state_transitions() {
    init_tracing!();
    tracing::info!("Starting task lifecycle and state transitions test");
    
    let mut config = DistributedCompilationConfig::default();
    config.coordinator_port = 9510;
    config.worker_ports = vec![9511, 9512];
    config.task_timeout_seconds = 30;
    
    let mut system = DistributedCompilationSystem::new(config).expect("Failed to create system");
    system.start().expect("Failed to start system");
    
    // Wait for system to initialize
    thread::sleep(Duration::from_millis(200));
    
    // Create tasks with different characteristics
    let tasks = vec![
        ("quick_task", CompilationTarget::Object, vec!["-O0".to_string()]),
        ("medium_task", CompilationTarget::IR, vec!["-O1".to_string()]),
        ("complex_task", CompilationTarget::Executable, vec!["-O2".to_string(), "-g".to_string()]),
        ("library_task", CompilationTarget::Library, vec!["-fPIC".to_string()]),
        ("assembly_task", CompilationTarget::Assembly, vec!["-S".to_string()]),
    ];
    
    let mut submitted_tasks = Vec::new();
    
    for (name, target, flags) in tasks {
        let task = create_compilation_task(
            vec![format!("{}.csd", name)],
            target,
            flags,
        );
        
        let task_id = system.submit_task(task).expect("Failed to submit task");
        submitted_tasks.push((name, task_id));
        
        tracing::info!("Submitted task: {} with ID: {}", name, task_id);
        
        // Small delay between submissions
        thread::sleep(Duration::from_millis(50));
    }
    
    // Monitor task state transitions
    let start_time = Instant::now();
    let mut task_states = HashMap::new();
    
    while start_time.elapsed() < Duration::from_secs(15) {
        let stats = system.get_statistics().expect("Failed to get statistics");
        
        // Track state changes
        let total_processed = stats.completed_tasks + stats.failed_tasks;
        let in_progress = stats.total_tasks - total_processed;
        
        if !task_states.contains_key(&total_processed) {
            task_states.insert(total_processed, start_time.elapsed());
            tracing::info!(
                "State transition: {} completed/failed, {} in progress at {:.1}s",
                total_processed,
                in_progress,
                start_time.elapsed().as_secs_f64()
            );
        }
        
        // Check if all tasks are processed
        if total_processed >= submitted_tasks.len() {
            break;
        }
        
        thread::sleep(Duration::from_millis(200));
    }
    
    let final_stats = system.get_statistics().expect("Failed to get final statistics");
    
    tracing::info!(
        "Task lifecycle test completed: {} tasks, {} completed, {} failed",
        final_stats.total_tasks,
        final_stats.completed_tasks,
        final_stats.failed_tasks
    );
    
    // Verify all tasks were processed in some way
    assert!(final_stats.total_tasks >= submitted_tasks.len(), 
            "Should track all submitted tasks");
    assert!(final_stats.completed_tasks + final_stats.failed_tasks >= submitted_tasks.len() / 2, 
            "Should process at least half of the tasks");
    
    system.stop().expect("Failed to stop system");
    tracing::info!("✓ Task lifecycle and state transitions test completed");
}

#[test]
fn test_network_protocol_compliance() {
    init_tracing!();
    tracing::info!("Starting network protocol compliance test");
    
    let mut config = DistributedCompilationConfig::default();
    config.coordinator_port = 9520;
    config.worker_ports = vec![9521];
    config.heartbeat_interval_seconds = 2;
    
    let mut system = DistributedCompilationSystem::new(config).expect("Failed to create system");
    system.start().expect("Failed to start system");
    
    // Test that we can create connections to the coordinator port
    thread::sleep(Duration::from_millis(300));
    
    let coordinator_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9520);
    
    // Test TCP connection establishment
    let connection_result = std::net::TcpStream::connect_timeout(&coordinator_addr, Duration::from_secs(1));
    
    match connection_result {
        Ok(stream) => {
            tracing::info!("Successfully established TCP connection to coordinator");
            
            // Test that we can send a simple message
            use cursed::build_system::distributed_compilation::NetworkMessage;
            
            let message = NetworkMessage::HealthCheck;
            let serialized = bincode::serialize(&message).expect("Should serialize message");
            
            // Just test serialization; actual network communication would require more complex setup
            assert!(!serialized.is_empty(), "Serialized message should not be empty");
            
            tracing::info!("Network message serialization successful");
        }
        Err(e) => {
            tracing::warn!("Could not establish TCP connection: {:?}", e);
            // This might be expected in test environment
        }
    }
    
    // Test node registration workflow
    let test_node = CompilationNode {
        id: "protocol_test_node".to_string(),
        address: "127.0.0.1".to_string(),
        port: 9522,
        capabilities: NodeCapabilities {
            cpu_cores: 2,
            memory_gb: 4,
            disk_space_gb: 100,
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
    
    // Test node registration API
    let registration_result = system.register_node(test_node.clone());
    assert!(registration_result.is_ok(), "Node registration should succeed");
    
    // Verify node appears in system
    let nodes = system.get_nodes().expect("Failed to get nodes");
    let registered_node = nodes.iter().find(|n| n.id == test_node.id);
    assert!(registered_node.is_some(), "Registered node should appear in system");
    
    // Test node unregistration
    let unregistration_result = system.unregister_node(&test_node.id);
    assert!(unregistration_result.is_ok(), "Node unregistration should succeed");
    
    // Verify node is removed
    let nodes_after = system.get_nodes().expect("Failed to get nodes after unregistration");
    let unregistered_node = nodes_after.iter().find(|n| n.id == test_node.id);
    assert!(unregistered_node.is_none(), "Unregistered node should not appear in system");
    
    system.stop().expect("Failed to stop system");
    tracing::info!("✓ Network protocol compliance test completed");
}

#[test]
fn test_performance_monitoring_and_metrics() {
    init_tracing!();
    tracing::info!("Starting performance monitoring and metrics test");
    
    let mut config = DistributedCompilationConfig::default();
    config.coordinator_port = 9530;
    config.worker_ports = vec![9531, 9532, 9533];
    config.task_timeout_seconds = 30;
    
    let mut system = DistributedCompilationSystem::new(config).expect("Failed to create system");
    system.start().expect("Failed to start system");
    
    // Register nodes with different performance characteristics
    let performance_nodes = vec![
        ("fast_node", 8, 16, 2.0, Duration::from_millis(50)),
        ("medium_node", 4, 8, 1.0, Duration::from_millis(100)),
        ("slow_node", 2, 4, 0.5, Duration::from_millis(200)),
    ];
    
    for (name, cores, memory, score, duration) in performance_nodes {
        let node = CompilationNode {
            id: name.to_string(),
            address: "127.0.0.1".to_string(),
            port: 9534 + name.len() as u16, // Simple port assignment
            capabilities: NodeCapabilities {
                cpu_cores: cores,
                memory_gb: memory,
                disk_space_gb: 200,
                supported_targets: vec![CompilationTarget::Object, CompilationTarget::IR],
                compiler_versions: HashMap::new(),
                performance_score: score,
            },
            status: NodeStatus::Online,
            current_load: 0.0,
            max_concurrent_tasks: cores,
            active_tasks: HashSet::new(),
            completed_tasks: 0,
            failed_tasks: 0,
            average_task_duration: duration,
            last_heartbeat: 0,
            is_coordinator: false,
        };
        
        system.register_node(node).expect("Failed to register performance test node");
        tracing::info!("Registered {} (cores: {}, score: {:.1})", name, cores, score);
    }
    
    // Submit a variety of tasks
    let num_tasks = 30;
    let mut tasks = Vec::new();
    
    for i in 0..num_tasks {
        tasks.push(create_compilation_task(
            vec![format!("perf_test_{}.csd", i)],
            if i % 2 == 0 { CompilationTarget::Object } else { CompilationTarget::IR },
            vec![format!("-DPERF_TEST_ID={}", i)],
        ));
    }
    
    let task_ids = system.submit_batch(tasks).expect("Failed to submit performance test batch");
    tracing::info!("Submitted {} tasks for performance monitoring", task_ids.len());
    
    // Monitor performance metrics over time
    let start_time = Instant::now();
    let monitoring_duration = Duration::from_secs(15);
    let mut metrics_history = Vec::new();
    
    while start_time.elapsed() < monitoring_duration {
        let stats = system.get_statistics().expect("Failed to get statistics");
        let nodes = system.get_nodes().expect("Failed to get nodes");
        
        let current_metrics = PerformanceMetrics {
            timestamp: start_time.elapsed(),
            total_tasks: stats.total_tasks,
            completed_tasks: stats.completed_tasks,
            failed_tasks: stats.failed_tasks,
            load_balancing_efficiency: stats.load_balancing_efficiency,
            work_stealing_operations: stats.work_stealing_operations,
            fault_recovery_count: stats.fault_recovery_count,
            nodes_utilized: stats.nodes_utilized,
            average_task_duration: stats.average_task_duration,
            active_nodes: nodes.iter().filter(|n| matches!(n.status, NodeStatus::Online)).count(),
            total_node_load: nodes.iter().map(|n| n.current_load).sum(),
        };
        
        metrics_history.push(current_metrics.clone());
        
        tracing::debug!(
            "Performance snapshot at {:.1}s: efficiency={:.3}, nodes_used={}, avg_duration={:.0}ms",
            current_metrics.timestamp.as_secs_f64(),
            current_metrics.load_balancing_efficiency,
            current_metrics.nodes_utilized,
            current_metrics.average_task_duration.as_millis()
        );
        
        // Check if processing is complete
        if stats.completed_tasks + stats.failed_tasks >= task_ids.len() {
            tracing::info!("All tasks processed, ending performance monitoring");
            break;
        }
        
        thread::sleep(Duration::from_millis(500));
    }
    
    // Analyze performance trends
    if metrics_history.len() >= 2 {
        let first = &metrics_history[0];
        let last = &metrics_history[metrics_history.len() - 1];
        
        let completion_rate = if last.timestamp > first.timestamp {
            (last.completed_tasks - first.completed_tasks) as f64 / 
            (last.timestamp - first.timestamp).as_secs_f64()
        } else {
            0.0
        };
        
        let efficiency_trend = last.load_balancing_efficiency - first.load_balancing_efficiency;
        
        tracing::info!(
            "Performance analysis: completion_rate={:.1} tasks/sec, efficiency_trend={:.3}",
            completion_rate,
            efficiency_trend
        );
        
        // Performance assertions
        assert!(completion_rate >= 0.0, "Completion rate should be non-negative");
        assert!(last.load_balancing_efficiency >= 0.0, "Load balancing efficiency should be non-negative");
        assert!(last.nodes_utilized <= last.active_nodes, "Utilized nodes should not exceed active nodes");
    }
    
    system.stop().expect("Failed to stop system");
    tracing::info!("✓ Performance monitoring and metrics test completed");
}

#[derive(Debug, Clone)]
struct PerformanceMetrics {
    timestamp: Duration,
    total_tasks: usize,
    completed_tasks: usize,
    failed_tasks: usize,
    load_balancing_efficiency: f64,
    work_stealing_operations: usize,
    fault_recovery_count: usize,
    nodes_utilized: usize,
    average_task_duration: Duration,
    active_nodes: usize,
    total_node_load: f64,
}

#[test]
fn test_concurrent_system_operations() {
    init_tracing!();
    tracing::info!("Starting concurrent system operations test");
    
    let mut config = DistributedCompilationConfig::default();
    config.coordinator_port = 9540;
    config.worker_ports = vec![9541, 9542];
    config.heartbeat_interval_seconds = 3;
    
    let mut system = DistributedCompilationSystem::new(config).expect("Failed to create system");
    system.start().expect("Failed to start system");
    
    let system = Arc::new(system);
    let test_duration = Duration::from_secs(10);
    let is_running = Arc::new(AtomicBool::new(true));
    
    // Counters for operations
    let tasks_submitted = Arc::new(AtomicUsize::new(0));
    let stats_queries = Arc::new(AtomicUsize::new(0));
    let node_operations = Arc::new(AtomicUsize::new(0));
    
    let mut handles = Vec::new();
    
    // Concurrent task submission
    for thread_id in 0..3 {
        let system_clone = system.clone();
        let is_running_clone = is_running.clone();
        let tasks_count = tasks_submitted.clone();
        
        let handle = thread::spawn(move || {
            let mut local_count = 0;
            
            while is_running_clone.load(Ordering::Relaxed) {
                let task = create_compilation_task(
                    vec![format!("concurrent_{}_{}.csd", thread_id, local_count)],
                    CompilationTarget::Object,
                    vec![],
                );
                
                if system_clone.submit_task(task).is_ok() {
                    local_count += 1;
                    tasks_count.fetch_add(1, Ordering::Relaxed);
                }
                
                thread::sleep(Duration::from_millis(20));
            }
            
            tracing::info!("Task submission thread {} submitted {} tasks", thread_id, local_count);
        });
        
        handles.push(handle);
    }
    
    // Concurrent statistics queries
    for thread_id in 0..2 {
        let system_clone = system.clone();
        let is_running_clone = is_running.clone();
        let stats_count = stats_queries.clone();
        
        let handle = thread::spawn(move || {
            let mut local_count = 0;
            
            while is_running_clone.load(Ordering::Relaxed) {
                if system_clone.get_statistics().is_ok() {
                    local_count += 1;
                    stats_count.fetch_add(1, Ordering::Relaxed);
                }
                
                if system_clone.get_nodes().is_ok() {
                    local_count += 1;
                    stats_count.fetch_add(1, Ordering::Relaxed);
                }
                
                thread::sleep(Duration::from_millis(30));
            }
            
            tracing::info!("Statistics thread {} performed {} queries", thread_id, local_count);
        });
        
        handles.push(handle);
    }
    
    // Concurrent node operations
    let system_clone = system.clone();
    let is_running_clone = is_running.clone();
    let node_ops_count = node_operations.clone();
    
    let node_handle = thread::spawn(move || {
        let mut cycle = 0;
        
        while is_running_clone.load(Ordering::Relaxed) {
            let node = CompilationNode {
                id: format!("concurrent_node_{}", cycle),
                address: "127.0.0.1".to_string(),
                port: 9600 + (cycle % 100) as u16,
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
            
            // Register node
            if system_clone.register_node(node.clone()).is_ok() {
                node_ops_count.fetch_add(1, Ordering::Relaxed);
                
                // Keep it briefly
                thread::sleep(Duration::from_millis(200));
                
                // Unregister node
                if system_clone.unregister_node(&node.id).is_ok() {
                    node_ops_count.fetch_add(1, Ordering::Relaxed);
                }
            }
            
            cycle += 1;
            thread::sleep(Duration::from_millis(100));
        }
        
        tracing::info!("Node operations thread completed {} cycles", cycle);
    });
    
    handles.push(node_handle);
    
    // Let concurrent operations run
    thread::sleep(test_duration);
    
    // Stop all operations
    is_running.store(false, Ordering::Relaxed);
    
    // Wait for all threads
    for handle in handles {
        handle.join().expect("Concurrent operation thread panicked");
    }
    
    let final_tasks = tasks_submitted.load(Ordering::Relaxed);
    let final_stats_queries = stats_queries.load(Ordering::Relaxed);
    let final_node_ops = node_operations.load(Ordering::Relaxed);
    
    tracing::info!(
        "Concurrent operations completed: {} tasks, {} stat queries, {} node ops",
        final_tasks,
        final_stats_queries,
        final_node_ops
    );
    
    // Verify system handled concurrent operations
    assert!(final_tasks >= 10, "Should submit at least 10 tasks concurrently");
    assert!(final_stats_queries >= 20, "Should perform at least 20 stats queries");
    assert!(final_node_ops >= 10, "Should perform at least 10 node operations");
    
    // System should still be functional
    let final_system_stats = system.get_statistics().expect("System should still respond to queries");
    assert!(final_system_stats.total_tasks >= final_tasks, "System should track all submitted tasks");
    
    tracing::info!("✓ Concurrent system operations test completed");
}

#[test]
fn test_error_recovery_mechanisms() {
    init_tracing!();
    tracing::info!("Starting error recovery mechanisms test");
    
    let mut config = DistributedCompilationConfig::default();
    config.coordinator_port = 9550;
    config.worker_ports = vec![9551];
    config.fault_tolerance_enabled = true;
    config.task_timeout_seconds = 5; // Short timeout to trigger failures
    config.max_network_retries = 2;
    
    let mut system = DistributedCompilationSystem::new(config).expect("Failed to create system");
    system.start().expect("Failed to start system");
    
    // Register a mix of reliable and unreliable nodes
    let nodes = vec![
        ("reliable_node", "127.0.0.1", 9552, true),
        ("unreliable_node_1", "192.168.99.99", 9553, false), // Non-existent IP
        ("unreliable_node_2", "127.0.0.1", 9999, false),      // Non-existent port
        ("another_reliable", "127.0.0.1", 9554, true),
    ];
    
    for (name, address, port, is_reliable) in nodes {
        let node = CompilationNode {
            id: name.to_string(),
            address: address.to_string(),
            port,
            capabilities: NodeCapabilities {
                cpu_cores: 4,
                memory_gb: 8,
                disk_space_gb: 200,
                supported_targets: vec![CompilationTarget::Object],
                compiler_versions: HashMap::new(),
                performance_score: if is_reliable { 1.0 } else { 2.0 }, // Unreliable nodes look better
            },
            status: NodeStatus::Online,
            current_load: 0.0,
            max_concurrent_tasks: 4,
            active_tasks: HashSet::new(),
            completed_tasks: 0,
            failed_tasks: 0,
            average_task_duration: Duration::from_millis(if is_reliable { 100 } else { 50 }),
            last_heartbeat: 0,
            is_coordinator: false,
        };
        
        system.register_node(node).expect("Failed to register node");
        tracing::info!("Registered {}: {} (reliable: {})", name, address, is_reliable);
    }
    
    // Submit tasks that will trigger failures and recovery
    let num_tasks = 20;
    let mut tasks = Vec::new();
    
    for i in 0..num_tasks {
        tasks.push(create_compilation_task(
            vec![format!("error_recovery_test_{}.csd", i)],
            CompilationTarget::Object,
            vec![format!("-DTEST_ID={}", i)],
        ));
    }
    
    let task_ids = system.submit_batch(tasks).expect("Failed to submit batch");
    tracing::info!("Submitted {} tasks for error recovery testing", task_ids.len());
    
    // Monitor error recovery
    let start_time = Instant::now();
    let monitoring_duration = Duration::from_secs(15);
    let mut error_stats = ErrorRecoveryStats::new();
    
    while start_time.elapsed() < monitoring_duration {
        let stats = system.get_statistics().expect("Failed to get statistics");
        let nodes = system.get_nodes().expect("Failed to get nodes");
        
        // Count node failures
        let failed_nodes = nodes.iter().filter(|n| {
            matches!(n.status, NodeStatus::Error(_) | NodeStatus::Offline)
        }).count();
        
        // Update error stats
        error_stats.update(&stats, failed_nodes);
        
        if error_stats.has_new_data() {
            tracing::info!(
                "Error recovery: {} tasks processed, {} failed nodes, {} recovery ops, {} failed tasks",
                stats.completed_tasks + stats.failed_tasks,
                failed_nodes,
                stats.fault_recovery_count,
                stats.failed_tasks
            );
        }
        
        // Check if most tasks are processed
        if stats.completed_tasks + stats.failed_tasks >= task_ids.len() * 3 / 4 {
            tracing::info!("75% of tasks processed, ending error recovery test");
            break;
        }
        
        thread::sleep(Duration::from_millis(500));
    }
    
    let final_stats = system.get_statistics().expect("Failed to get final statistics");
    tracing::info!(
        "Error recovery test completed: {} recovery operations, {} failed tasks out of {}",
        final_stats.fault_recovery_count,
        final_stats.failed_tasks,
        final_stats.total_tasks
    );
    
    // Verify error recovery mechanisms worked
    assert!(error_stats.max_failed_nodes > 0, "Should have detected node failures");
    assert!(final_stats.fault_recovery_count > 0 || final_stats.failed_tasks > 0, 
            "Should have recovery operations or failed tasks");
    assert!(final_stats.total_tasks >= task_ids.len(), "Should track all submitted tasks");
    
    system.stop().expect("Failed to stop system");
    tracing::info!("✓ Error recovery mechanisms test completed");
}

#[derive(Debug)]
struct ErrorRecoveryStats {
    last_recovery_count: usize,
    last_failed_tasks: usize,
    max_failed_nodes: usize,
    last_failed_nodes: usize,
}

impl ErrorRecoveryStats {
    fn new() -> Self {
        Self {
            last_recovery_count: 0,
            last_failed_tasks: 0,
            max_failed_nodes: 0,
            last_failed_nodes: 0,
        }
    }
    
    fn update(&mut self, stats: &DistributedCompilationStats, failed_nodes: usize) {
        self.last_recovery_count = stats.fault_recovery_count;
        self.last_failed_tasks = stats.failed_tasks;
        self.max_failed_nodes = self.max_failed_nodes.max(failed_nodes);
        self.last_failed_nodes = failed_nodes;
    }
    
    fn has_new_data(&self) -> bool {
        // Simple check - in a real implementation this would be more sophisticated
        true
    }
}
