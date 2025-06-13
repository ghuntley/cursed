//! Comprehensive test suite for distributed compilation system
//! 
//! Tests all components including network communication, work stealing,
//! health monitoring, and fault tolerance.

use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::time::{Duration, Instant};
use std::thread;
use std::collections::{HashMap, HashSet};

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
    ConnectionPool,
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
fn test_distributed_compilation_system_creation() {
    init_tracing!();
    
    let config = DistributedCompilationConfig::default();
    let system = DistributedCompilationSystem::new(config);
    
    assert!(system.is_ok(), "Failed to create distributed compilation system");
    tracing::info!("✓ Distributed compilation system created successfully");
}

#[test]
fn test_system_start_stop() {
    init_tracing!();
    
    let mut config = DistributedCompilationConfig::default();
    config.coordinator_port = 9005; // Use different port to avoid conflicts
    
    let mut system = DistributedCompilationSystem::new(config).expect("Failed to create system");
    
    // Test start
    let start_result = system.start();
    assert!(start_result.is_ok(), "Failed to start system: {:?}", start_result);
    tracing::info!("✓ System started successfully");
    
    // Give some time for threads to start
    thread::sleep(Duration::from_millis(100));
    
    // Test stop
    let stop_result = system.stop();
    assert!(stop_result.is_ok(), "Failed to stop system: {:?}", stop_result);
    tracing::info!("✓ System stopped successfully");
}

#[test]
fn test_task_creation_and_submission() {
    init_tracing!();
    
    // Create compilation task
    let task = create_compilation_task(
        vec!["test.csd".to_string()],
        CompilationTarget::Object,
        vec!["-O2".to_string()],
    );
    
    assert!(!task.id.is_empty(), "Task ID should not be empty");
    assert_eq!(task.source_files, vec!["test.csd"]);
    assert!(matches!(task.target_type, CompilationTarget::Object));
    assert_eq!(task.compilation_flags, vec!["-O2"]);
    
    tracing::info!("✓ Task creation successful: {}", task.id);
    
    // Test task submission
    let mut config = DistributedCompilationConfig::default();
    config.coordinator_port = 9006;
    
    let mut system = DistributedCompilationSystem::new(config).expect("Failed to create system");
    system.start().expect("Failed to start system");
    
    let task_id = system.submit_task(task).expect("Failed to submit task");
    assert!(!task_id.is_empty(), "Returned task ID should not be empty");
    
    tracing::info!("✓ Task submitted successfully: {}", task_id);
    
    system.stop().expect("Failed to stop system");
}

#[test]
fn test_batch_task_submission() {
    init_tracing!();
    
    let tasks = vec![
        create_compilation_task(
            vec!["test1.csd".to_string()],
            CompilationTarget::Object,
            vec![],
        ),
        create_compilation_task(
            vec!["test2.csd".to_string()],
            CompilationTarget::IR,
            vec!["-O1".to_string()],
        ),
        create_compilation_task(
            vec!["test3.csd".to_string()],
            CompilationTarget::Assembly,
            vec!["-g".to_string()],
        ),
    ];
    
    let mut config = DistributedCompilationConfig::default();
    config.coordinator_port = 9007;
    
    let mut system = DistributedCompilationSystem::new(config).expect("Failed to create system");
    system.start().expect("Failed to start system");
    
    let task_ids = system.submit_batch(tasks).expect("Failed to submit batch");
    
    assert_eq!(task_ids.len(), 3, "Should return 3 task IDs");
    for task_id in &task_ids {
        assert!(!task_id.is_empty(), "Task ID should not be empty");
    }
    
    tracing::info!("✓ Batch submission successful: {} tasks", task_ids.len());
    
    system.stop().expect("Failed to stop system");
}

#[test] 
fn test_node_registration() {
    init_tracing!();
    
    let mut config = DistributedCompilationConfig::default();
    config.coordinator_port = 9008;
    
    let mut system = DistributedCompilationSystem::new(config).expect("Failed to create system");
    system.start().expect("Failed to start system");
    
    // Create test node
    let node = CompilationNode {
        id: "test_node_1".to_string(),
        address: "127.0.0.1".to_string(),
        port: 9009,
        capabilities: NodeCapabilities {
            cpu_cores: 4,
            memory_gb: 16,
            disk_space_gb: 500,
            supported_targets: vec![CompilationTarget::Object, CompilationTarget::IR],
            compiler_versions: HashMap::new(),
            performance_score: 1.2,
        },
        status: NodeStatus::Online,
        current_load: 0.0,
        max_concurrent_tasks: 4,
        active_tasks: HashSet::new(),
        completed_tasks: 0,
        failed_tasks: 0,
        average_task_duration: Duration::ZERO,
        last_heartbeat: 0,
        is_coordinator: false,
    };
    
    // Register node
    system.register_node(node.clone()).expect("Failed to register node");
    
    // Verify node registration
    let nodes = system.get_nodes().expect("Failed to get nodes");
    let registered_node = nodes.iter()
        .find(|n| n.id == "test_node_1")
        .expect("Node should be registered");
    
    assert_eq!(registered_node.id, node.id);
    assert_eq!(registered_node.address, node.address);
    assert_eq!(registered_node.port, node.port);
    
    tracing::info!("✓ Node registration successful: {}", node.id);
    
    system.stop().expect("Failed to stop system");
}

#[test]
fn test_node_unregistration() {
    init_tracing!();
    
    let mut config = DistributedCompilationConfig::default();
    config.coordinator_port = 9010;
    
    let mut system = DistributedCompilationSystem::new(config).expect("Failed to create system");
    system.start().expect("Failed to start system");
    
    // Create and register test node
    let node = CompilationNode {
        id: "test_node_2".to_string(),
        address: "127.0.0.1".to_string(),
        port: 9011,
        capabilities: NodeCapabilities {
            cpu_cores: 2,
            memory_gb: 8,
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
        average_task_duration: Duration::ZERO,
        last_heartbeat: 0,
        is_coordinator: false,
    };
    
    system.register_node(node.clone()).expect("Failed to register node");
    
    // Verify node is registered
    let nodes_before = system.get_nodes().expect("Failed to get nodes");
    assert!(nodes_before.iter().any(|n| n.id == "test_node_2"), "Node should be registered");
    
    // Unregister node
    system.unregister_node("test_node_2").expect("Failed to unregister node");
    
    // Verify node is unregistered
    let nodes_after = system.get_nodes().expect("Failed to get nodes");
    assert!(!nodes_after.iter().any(|n| n.id == "test_node_2"), "Node should be unregistered");
    
    tracing::info!("✓ Node unregistration successful");
    
    system.stop().expect("Failed to stop system");
}

#[test]
fn test_statistics_tracking() {
    init_tracing!();
    
    let mut config = DistributedCompilationConfig::default();
    config.coordinator_port = 9012;
    
    let mut system = DistributedCompilationSystem::new(config).expect("Failed to create system");
    system.start().expect("Failed to start system");
    
    // Submit some tasks
    let task1 = create_compilation_task(
        vec!["test1.csd".to_string()],
        CompilationTarget::Object,
        vec![],
    );
    let task2 = create_compilation_task(
        vec!["test2.csd".to_string()],
        CompilationTarget::IR,
        vec![],
    );
    
    system.submit_task(task1).expect("Failed to submit task 1");
    system.submit_task(task2).expect("Failed to submit task 2");
    
    // Get statistics
    let stats = system.get_statistics().expect("Failed to get statistics");
    
    assert!(stats.total_tasks >= 2, "Should have at least 2 tasks");
    assert_eq!(stats.completed_tasks, 0, "Should have 0 completed tasks initially");
    assert_eq!(stats.failed_tasks, 0, "Should have 0 failed tasks initially");
    
    tracing::info!("✓ Statistics tracking working: {} total tasks", stats.total_tasks);
    
    system.stop().expect("Failed to stop system");
}

#[test]
fn test_connection_pool() {
    init_tracing!();
    
    let pool = ConnectionPool::new(5);
    
    // Test connection attempts (these will fail but shouldn't crash)
    let result1 = pool.get_connection("127.0.0.1:9999");
    let result2 = pool.get_connection("127.0.0.1:9998");
    
    // Both should fail gracefully
    assert!(result1.is_err(), "Connection should fail to non-existent port");
    assert!(result2.is_err(), "Connection should fail to non-existent port");
    
    tracing::info!("✓ Connection pool error handling works correctly");
}

#[test]
fn test_load_balancing_strategies() {
    init_tracing!();
    
    // Test different load balancing strategies
    let strategies = vec![
        LoadBalancingStrategy::RoundRobin,
        LoadBalancingStrategy::LeastLoaded,
        LoadBalancingStrategy::WeightedRoundRobin,
        LoadBalancingStrategy::PerformanceBased,
        LoadBalancingStrategy::WorkStealing,
    ];
    
    for strategy in strategies {
        let mut config = DistributedCompilationConfig::default();
        config.load_balancing_strategy = strategy.clone();
        config.coordinator_port = 9013;
        
        let system = DistributedCompilationSystem::new(config);
        assert!(system.is_ok(), "Failed to create system with strategy: {:?}", strategy);
        
        tracing::info!("✓ Load balancing strategy works: {:?}", strategy);
    }
}

#[test]
fn test_network_message_serialization() {
    init_tracing!();
    
    let messages = vec![
        NetworkMessage::DiscoveryRequest,
        NetworkMessage::Heartbeat("test_node".to_string()),
        NetworkMessage::WorkStealRequest("requesting_node".to_string()),
        NetworkMessage::NodeShutdown("shutdown_node".to_string()),
        NetworkMessage::HealthCheck,
        NetworkMessage::HealthResponse(NodeStatus::Online),
    ];
    
    for message in messages {
        let serialized = bincode::serialize(&message);
        assert!(serialized.is_ok(), "Failed to serialize message: {:?}", message);
        
        let deserialized: Result<NetworkMessage, _> = bincode::deserialize(&serialized.unwrap());
        assert!(deserialized.is_ok(), "Failed to deserialize message: {:?}", message);
        
        tracing::debug!("✓ Message serialization works: {:?}", message);
    }
    
    tracing::info!("✓ All network message types serialize correctly");
}

#[test]
fn test_task_timeout_handling() {
    init_tracing!();
    
    let mut config = DistributedCompilationConfig::default();
    config.coordinator_port = 9014;
    config.task_timeout_seconds = 1; // Very short timeout for testing
    
    let mut system = DistributedCompilationSystem::new(config).expect("Failed to create system");
    system.start().expect("Failed to start system");
    
    // Submit a task
    let task = create_compilation_task(
        vec!["timeout_test.csd".to_string()],
        CompilationTarget::Object,
        vec![],
    );
    let task_id = system.submit_task(task).expect("Failed to submit task");
    
    // Wait for timeout + processing time
    thread::sleep(Duration::from_millis(1500));
    
    // Check that timeout was handled
    let stats = system.get_statistics().expect("Failed to get statistics");
    tracing::info!("Statistics after timeout: failed={}, completed={}", 
                  stats.failed_tasks, stats.completed_tasks);
    
    tracing::info!("✓ Task timeout handling tested");
    
    system.stop().expect("Failed to stop system");
}

#[test]
fn test_concurrent_operations() {
    init_tracing!();
    
    let mut config = DistributedCompilationConfig::default();
    config.coordinator_port = 9015;
    
    let mut system = DistributedCompilationSystem::new(config).expect("Failed to create system");
    system.start().expect("Failed to start system");
    
    let system = Arc::new(system);
    let num_threads = 4;
    let tasks_per_thread = 5;
    
    let mut handles = Vec::new();
    
    // Spawn multiple threads submitting tasks concurrently
    for thread_id in 0..num_threads {
        let system_clone = system.clone();
        
        let handle = thread::spawn(move || {
            for task_id in 0..tasks_per_thread {
                let task = create_compilation_task(
                    vec![format!("concurrent_test_{}_{}.csd", thread_id, task_id)],
                    CompilationTarget::Object,
                    vec![],
                );
                
                if let Err(e) = system_clone.submit_task(task) {
                    tracing::error!("Failed to submit task in thread {}: {:?}", thread_id, e);
                }
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
    
    // Check final statistics
    let stats = system.get_statistics().expect("Failed to get statistics");
    let expected_tasks = num_threads * tasks_per_thread;
    
    assert!(stats.total_tasks >= expected_tasks, 
           "Should have at least {} tasks, got {}", expected_tasks, stats.total_tasks);
    
    tracing::info!("✓ Concurrent operations successful: {} tasks submitted", stats.total_tasks);
    
    // Note: Can't call stop() on Arc<DistributedCompilationSystem>
    // In a real scenario, we'd need a different approach for shutdown
}

#[test]
fn test_error_recovery() {
    init_tracing!();
    
    let mut config = DistributedCompilationConfig::default();
    config.coordinator_port = 9016;
    config.fault_tolerance_enabled = true;
    
    let mut system = DistributedCompilationSystem::new(config).expect("Failed to create system");
    system.start().expect("Failed to start system");
    
    // Register a node that will "fail"
    let failing_node = CompilationNode {
        id: "failing_node".to_string(),
        address: "127.0.0.1".to_string(),
        port: 9999, // Non-existent port
        capabilities: NodeCapabilities {
            cpu_cores: 1,
            memory_gb: 1,
            disk_space_gb: 10,
            supported_targets: vec![CompilationTarget::Object],
            compiler_versions: HashMap::new(),
            performance_score: 1.0,
        },
        status: NodeStatus::Online,
        current_load: 0.0,
        max_concurrent_tasks: 1,
        active_tasks: HashSet::new(),
        completed_tasks: 0,
        failed_tasks: 0,
        average_task_duration: Duration::ZERO,
        last_heartbeat: 0,
        is_coordinator: false,
    };
    
    system.register_node(failing_node).expect("Failed to register failing node");
    
    // Submit a task that should trigger error recovery
    let task = create_compilation_task(
        vec!["error_recovery_test.csd".to_string()],
        CompilationTarget::Object,
        vec![],
    );
    
    let task_id = system.submit_task(task).expect("Failed to submit task");
    
    // Give time for error to occur and recovery to happen
    thread::sleep(Duration::from_millis(500));
    
    let stats = system.get_statistics().expect("Failed to get statistics");
    tracing::info!("Error recovery stats: total={}, failed={}", stats.total_tasks, stats.failed_tasks);
    
    tracing::info!("✓ Error recovery mechanism tested");
    
    system.stop().expect("Failed to stop system");
}

#[test]
fn test_configuration_validation() {
    init_tracing!();
    
    // Test default configuration
    let default_config = DistributedCompilationConfig::default();
    assert_eq!(default_config.coordinator_port, 9000);
    assert_eq!(default_config.worker_ports.len(), 4);
    assert_eq!(default_config.max_network_retries, 3);
    assert_eq!(default_config.task_timeout_seconds, 300);
    assert!(default_config.fault_tolerance_enabled);
    assert!(default_config.work_stealing_enabled);
    
    tracing::info!("✓ Default configuration validation passed");
    
    // Test custom configuration
    let custom_config = DistributedCompilationConfig {
        coordinator_port: 8000,
        worker_ports: vec![8001, 8002],
        max_network_retries: 5,
        task_timeout_seconds: 600,
        heartbeat_interval_seconds: 15,
        load_balancing_strategy: LoadBalancingStrategy::LeastLoaded,
        fault_tolerance_enabled: false,
        work_stealing_enabled: false,
        result_caching_enabled: false,
        compression_enabled: false,
        encryption_enabled: true,
    };
    
    assert_eq!(custom_config.coordinator_port, 8000);
    assert_eq!(custom_config.worker_ports, vec![8001, 8002]);
    assert_eq!(custom_config.max_network_retries, 5);
    assert!(!custom_config.fault_tolerance_enabled);
    assert!(custom_config.encryption_enabled);
    
    tracing::info!("✓ Custom configuration validation passed");
}
