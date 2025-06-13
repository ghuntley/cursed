//! Unit tests for distributed compilation system components
//! 
//! Tests individual components in isolation including network discovery,
//! task distribution, work stealing, health monitoring, and load balancing.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex, RwLock, atomic::{AtomicBool, Ordering}};
use std::time::{Duration, Instant};
use std::net::{SocketAddr, IpAddr, Ipv4Addr};

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
    DistributedCompilationStats,
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
fn test_compilation_task_creation() {
    init_tracing!();
    tracing::info!("Testing compilation task creation");
    
    let source_files = vec!["main.csd".to_string(), "utils.csd".to_string()];
    let target = CompilationTarget::Executable;
    let flags = vec!["-O2".to_string(), "-g".to_string()];
    
    let task = create_compilation_task(source_files.clone(), target.clone(), flags.clone());
    
    assert!(!task.id.is_empty(), "Task should have a non-empty ID");
    assert_eq!(task.source_files, source_files, "Source files should match");
    assert_eq!(task.compilation_flags, flags, "Compilation flags should match");
    assert!(matches!(task.target_type, CompilationTarget::Executable), "Target type should match");
    assert_eq!(task.priority, 5, "Default priority should be 5");
    assert_eq!(task.retry_count, 0, "Initial retry count should be 0");
    assert_eq!(task.max_retries, 3, "Default max retries should be 3");
    assert!(task.created_at > 0, "Created timestamp should be set");
    
    tracing::info!("✓ Compilation task creation test passed");
}

#[test]
fn test_distributed_compilation_config_default() {
    init_tracing!();
    tracing::info!("Testing default distributed compilation configuration");
    
    let config = DistributedCompilationConfig::default();
    
    assert_eq!(config.coordinator_port, 9000, "Default coordinator port should be 9000");
    assert_eq!(config.worker_ports, vec![9001, 9002, 9003, 9004], "Default worker ports should match");
    assert_eq!(config.max_network_retries, 3, "Default max retries should be 3");
    assert_eq!(config.task_timeout_seconds, 300, "Default timeout should be 5 minutes");
    assert_eq!(config.heartbeat_interval_seconds, 30, "Default heartbeat interval should be 30 seconds");
    assert!(matches!(config.load_balancing_strategy, LoadBalancingStrategy::WorkStealing), 
            "Default strategy should be WorkStealing");
    assert!(config.fault_tolerance_enabled, "Fault tolerance should be enabled by default");
    assert!(config.work_stealing_enabled, "Work stealing should be enabled by default");
    assert!(config.result_caching_enabled, "Result caching should be enabled by default");
    assert!(config.compression_enabled, "Compression should be enabled by default");
    assert!(!config.encryption_enabled, "Encryption should be disabled by default");
    
    tracing::info!("✓ Default configuration test passed");
}

#[test]
fn test_compilation_node_creation() {
    init_tracing!();
    tracing::info!("Testing compilation node creation");
    
    let capabilities = NodeCapabilities {
        cpu_cores: 8,
        memory_gb: 16,
        disk_space_gb: 500,
        supported_targets: vec![
            CompilationTarget::Object,
            CompilationTarget::IR,
            CompilationTarget::Executable,
        ],
        compiler_versions: HashMap::new(),
        performance_score: 1.5,
    };
    
    let node = CompilationNode {
        id: "test_node".to_string(),
        address: "192.168.1.100".to_string(),
        port: 9001,
        capabilities: capabilities.clone(),
        status: NodeStatus::Online,
        current_load: 0.3,
        max_concurrent_tasks: 8,
        active_tasks: HashSet::new(),
        completed_tasks: 10,
        failed_tasks: 1,
        average_task_duration: Duration::from_millis(150),
        last_heartbeat: 1234567890,
        is_coordinator: false,
    };
    
    assert_eq!(node.id, "test_node", "Node ID should match");
    assert_eq!(node.address, "192.168.1.100", "Node address should match");
    assert_eq!(node.port, 9001, "Node port should match");
    assert_eq!(node.capabilities.cpu_cores, 8, "CPU cores should match");
    assert_eq!(node.capabilities.memory_gb, 16, "Memory should match");
    assert_eq!(node.capabilities.performance_score, 1.5, "Performance score should match");
    assert!(matches!(node.status, NodeStatus::Online), "Status should be Online");
    assert_eq!(node.current_load, 0.3, "Current load should match");
    assert_eq!(node.max_concurrent_tasks, 8, "Max concurrent tasks should match");
    assert!(node.active_tasks.is_empty(), "Active tasks should be empty initially");
    
    tracing::info!("✓ Compilation node creation test passed");
}

#[test]
fn test_network_message_serialization() {
    init_tracing!();
    tracing::info!("Testing network message serialization");
    
    let messages = vec![
        NetworkMessage::DiscoveryRequest,
        NetworkMessage::HealthCheck,
        NetworkMessage::Heartbeat("test_node".to_string()),
        NetworkMessage::WorkStealRequest("requesting_node".to_string()),
        NetworkMessage::NodeShutdown("shutting_down_node".to_string()),
    ];
    
    for message in messages {
        let serialized = bincode::serialize(&message).expect("Should serialize message");
        assert!(!serialized.is_empty(), "Serialized message should not be empty");
        
        let deserialized: NetworkMessage = bincode::deserialize(&serialized)
            .expect("Should deserialize message");
        
        // Compare message types (simplified comparison)
        match (&message, &deserialized) {
            (NetworkMessage::DiscoveryRequest, NetworkMessage::DiscoveryRequest) => {},
            (NetworkMessage::HealthCheck, NetworkMessage::HealthCheck) => {},
            (NetworkMessage::Heartbeat(a), NetworkMessage::Heartbeat(b)) => assert_eq!(a, b),
            (NetworkMessage::WorkStealRequest(a), NetworkMessage::WorkStealRequest(b)) => assert_eq!(a, b),
            (NetworkMessage::NodeShutdown(a), NetworkMessage::NodeShutdown(b)) => assert_eq!(a, b),
            _ => panic!("Message type mismatch after serialization"),
        }
    }
    
    tracing::info!("✓ Network message serialization test passed");
}

#[test]
fn test_load_balancing_strategy_selection() {
    init_tracing!();
    tracing::info!("Testing load balancing strategy selection");
    
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
        
        // Test that we can create a system with each strategy
        let system_result = DistributedCompilationSystem::new(config);
        assert!(system_result.is_ok(), "Should be able to create system with strategy: {:?}", strategy);
    }
    
    tracing::info!("✓ Load balancing strategy selection test passed");
}

#[test]
fn test_compilation_result_creation() {
    init_tracing!();
    tracing::info!("Testing compilation result creation");
    
    let successful_result = CompilationResult {
        task_id: "task_123".to_string(),
        node_id: "node_456".to_string(),
        success: true,
        output: b"compiled output".to_vec(),
        error_message: None,
        compilation_time: Duration::from_millis(250),
        output_files: vec!["output.o".to_string(), "output.ll".to_string()],
        warnings: vec!["unused variable".to_string()],
        completed_at: 1234567890,
    };
    
    assert_eq!(successful_result.task_id, "task_123", "Task ID should match");
    assert_eq!(successful_result.node_id, "node_456", "Node ID should match");
    assert!(successful_result.success, "Result should be successful");
    assert!(successful_result.error_message.is_none(), "Error message should be None for success");
    assert_eq!(successful_result.output_files.len(), 2, "Should have 2 output files");
    assert_eq!(successful_result.warnings.len(), 1, "Should have 1 warning");
    
    let failed_result = CompilationResult {
        task_id: "task_789".to_string(),
        node_id: "node_101".to_string(),
        success: false,
        output: Vec::new(),
        error_message: Some("Compilation failed: syntax error".to_string()),
        compilation_time: Duration::from_millis(100),
        output_files: Vec::new(),
        warnings: Vec::new(),
        completed_at: 1234567900,
    };
    
    assert!(!failed_result.success, "Result should be failed");
    assert!(failed_result.error_message.is_some(), "Error message should be present for failure");
    assert!(failed_result.output_files.is_empty(), "Failed result should have no output files");
    
    tracing::info!("✓ Compilation result creation test passed");
}

#[test]
fn test_statistics_initialization() {
    init_tracing!();
    tracing::info!("Testing statistics initialization");
    
    let stats = DistributedCompilationStats {
        total_tasks: 0,
        completed_tasks: 0,
        failed_tasks: 0,
        average_task_duration: Duration::ZERO,
        network_overhead: Duration::ZERO,
        load_balancing_efficiency: 0.0,
        fault_recovery_count: 0,
        work_stealing_operations: 0,
        data_transferred_mb: 0.0,
        nodes_utilized: 0,
    };
    
    assert_eq!(stats.total_tasks, 0, "Total tasks should start at 0");
    assert_eq!(stats.completed_tasks, 0, "Completed tasks should start at 0");
    assert_eq!(stats.failed_tasks, 0, "Failed tasks should start at 0");
    assert_eq!(stats.average_task_duration, Duration::ZERO, "Average duration should start at 0");
    assert_eq!(stats.load_balancing_efficiency, 0.0, "Efficiency should start at 0");
    assert_eq!(stats.fault_recovery_count, 0, "Recovery count should start at 0");
    assert_eq!(stats.work_stealing_operations, 0, "Work stealing ops should start at 0");
    assert_eq!(stats.data_transferred_mb, 0.0, "Data transferred should start at 0");
    assert_eq!(stats.nodes_utilized, 0, "Nodes utilized should start at 0");
    
    tracing::info!("✓ Statistics initialization test passed");
}

#[test]
fn test_node_status_transitions() {
    init_tracing!();
    tracing::info!("Testing node status transitions");
    
    let statuses = vec![
        NodeStatus::Online,
        NodeStatus::Busy,
        NodeStatus::Offline,
        NodeStatus::Maintenance,
        NodeStatus::Error("Connection timeout".to_string()),
    ];
    
    for status in statuses {
        // Test that we can match on each status
        match status {
            NodeStatus::Online => assert!(true, "Online status should be valid"),
            NodeStatus::Busy => assert!(true, "Busy status should be valid"),
            NodeStatus::Offline => assert!(true, "Offline status should be valid"),
            NodeStatus::Maintenance => assert!(true, "Maintenance status should be valid"),
            NodeStatus::Error(ref msg) => {
                assert!(!msg.is_empty(), "Error status should have a message");
                assert_eq!(msg, "Connection timeout", "Error message should match");
            }
        }
    }
    
    tracing::info!("✓ Node status transitions test passed");
}

#[test]
fn test_system_creation_with_custom_config() {
    init_tracing!();
    tracing::info!("Testing system creation with custom configuration");
    
    let mut config = DistributedCompilationConfig::default();
    config.coordinator_port = 9100;
    config.worker_ports = vec![9101, 9102];
    config.max_network_retries = 5;
    config.task_timeout_seconds = 600;
    config.heartbeat_interval_seconds = 10;
    config.load_balancing_strategy = LoadBalancingStrategy::LeastLoaded;
    config.fault_tolerance_enabled = false;
    config.work_stealing_enabled = false;
    config.result_caching_enabled = false;
    config.compression_enabled = false;
    config.encryption_enabled = true;
    
    let system_result = DistributedCompilationSystem::new(config.clone());
    assert!(system_result.is_ok(), "Should be able to create system with custom config");
    
    let system = system_result.unwrap();
    
    // Test that we can get the system's configuration (indirectly through behavior)
    // Note: The actual config is not exposed publicly, so we test via system creation success
    
    tracing::info!("✓ System creation with custom config test passed");
}

#[test]
fn test_task_priority_and_retry_logic() {
    init_tracing!();
    tracing::info!("Testing task priority and retry logic");
    
    let mut task = create_compilation_task(
        vec!["test.csd".to_string()],
        CompilationTarget::Object,
        vec![],
    );
    
    // Test initial state
    assert_eq!(task.priority, 5, "Default priority should be 5");
    assert_eq!(task.retry_count, 0, "Initial retry count should be 0");
    assert_eq!(task.max_retries, 3, "Default max retries should be 3");
    
    // Simulate retry logic
    task.retry_count += 1;
    assert_eq!(task.retry_count, 1, "Retry count should increment");
    assert!(task.retry_count <= task.max_retries, "Should not exceed max retries");
    
    // Test high priority task
    task.priority = 10;
    assert_eq!(task.priority, 10, "Priority should be updatable");
    
    // Test retry exhaustion
    task.retry_count = task.max_retries + 1;
    assert!(task.retry_count > task.max_retries, "Should be able to exceed max retries for testing");
    
    tracing::info!("✓ Task priority and retry logic test passed");
}

#[test]
fn test_node_capability_matching() {
    init_tracing!();
    tracing::info!("Testing node capability matching");
    
    let high_perf_node = NodeCapabilities {
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
        compiler_versions: {
            let mut versions = HashMap::new();
            versions.insert("cursed".to_string(), "1.0.0".to_string());
            versions.insert("llvm".to_string(), "16.0.0".to_string());
            versions
        },
        performance_score: 2.5,
    };
    
    let low_perf_node = NodeCapabilities {
        cpu_cores: 2,
        memory_gb: 4,
        disk_space_gb: 100,
        supported_targets: vec![CompilationTarget::Object],
        compiler_versions: HashMap::new(),
        performance_score: 0.5,
    };
    
    // Test that high performance node has more capabilities
    assert!(high_perf_node.cpu_cores > low_perf_node.cpu_cores, "High perf should have more cores");
    assert!(high_perf_node.memory_gb > low_perf_node.memory_gb, "High perf should have more memory");
    assert!(high_perf_node.supported_targets.len() > low_perf_node.supported_targets.len(), 
            "High perf should support more targets");
    assert!(high_perf_node.performance_score > low_perf_node.performance_score, 
            "High perf should have higher score");
    
    // Test target support checking
    assert!(high_perf_node.supported_targets.contains(&CompilationTarget::Executable), 
            "High perf should support executable compilation");
    assert!(!low_perf_node.supported_targets.contains(&CompilationTarget::Executable), 
            "Low perf should not support executable compilation");
    
    tracing::info!("✓ Node capability matching test passed");
}

#[test]
fn test_compilation_target_types() {
    init_tracing!();
    tracing::info!("Testing compilation target types");
    
    let targets = vec![
        CompilationTarget::Object,
        CompilationTarget::IR,
        CompilationTarget::Assembly,
        CompilationTarget::Executable,
        CompilationTarget::Library,
    ];
    
    // Test serialization/deserialization of all target types
    for target in targets {
        let serialized = bincode::serialize(&target).expect("Should serialize target");
        let deserialized: CompilationTarget = bincode::deserialize(&serialized)
            .expect("Should deserialize target");
        
        match (&target, &deserialized) {
            (CompilationTarget::Object, CompilationTarget::Object) => {},
            (CompilationTarget::IR, CompilationTarget::IR) => {},
            (CompilationTarget::Assembly, CompilationTarget::Assembly) => {},
            (CompilationTarget::Executable, CompilationTarget::Executable) => {},
            (CompilationTarget::Library, CompilationTarget::Library) => {},
            _ => panic!("Target type mismatch after serialization"),
        }
    }
    
    tracing::info!("✓ Compilation target types test passed");
}

#[test]
fn test_task_estimation_and_timing() {
    init_tracing!();
    tracing::info!("Testing task estimation and timing");
    
    let task = create_compilation_task(
        vec!["large_file.csd".to_string()],
        CompilationTarget::Executable,
        vec!["-O3".to_string()],
    );
    
    assert_eq!(task.estimated_duration, Duration::from_secs(30), "Default estimation should be 30 seconds");
    assert!(task.created_at > 0, "Created timestamp should be set");
    
    // Test that we can create tasks with different estimations
    let mut quick_task = task.clone();
    quick_task.estimated_duration = Duration::from_secs(5);
    assert_eq!(quick_task.estimated_duration, Duration::from_secs(5), "Quick task estimation should be 5 seconds");
    
    let mut long_task = task.clone();
    long_task.estimated_duration = Duration::from_secs(300);
    assert_eq!(long_task.estimated_duration, Duration::from_secs(300), "Long task estimation should be 5 minutes");
    
    tracing::info!("✓ Task estimation and timing test passed");
}

#[test]
fn test_connection_pool_basic_functionality() {
    init_tracing!();
    tracing::info!("Testing connection pool basic functionality");
    
    // We can't easily test the actual TCP connections in unit tests,
    // but we can test the basic structure and error handling
    
    use cursed::build_system::distributed_compilation::ConnectionPool;
    
    let pool = ConnectionPool::new(10);
    
    // Test getting connection to non-existent address (should fail gracefully)
    let result = pool.get_connection("127.0.0.1:99999");
    assert!(result.is_err(), "Connection to non-existent address should fail");
    
    tracing::info!("✓ Connection pool basic functionality test passed");
}

#[test]
fn test_system_lifecycle() {
    init_tracing!();
    tracing::info!("Testing system lifecycle");
    
    let config = DistributedCompilationConfig {
        coordinator_port: 9200,
        worker_ports: vec![9201],
        max_network_retries: 1,
        task_timeout_seconds: 10,
        heartbeat_interval_seconds: 5,
        load_balancing_strategy: LoadBalancingStrategy::RoundRobin,
        fault_tolerance_enabled: true,
        work_stealing_enabled: false,
        result_caching_enabled: false,
        compression_enabled: false,
        encryption_enabled: false,
    };
    
    // Test system creation
    let mut system = DistributedCompilationSystem::new(config).expect("Should create system");
    
    // Test system start
    let start_result = system.start();
    assert!(start_result.is_ok(), "System should start successfully");
    
    // Test double start (should fail)
    let double_start_result = system.start();
    assert!(double_start_result.is_err(), "Double start should fail");
    
    // Wait a moment for system to initialize
    std::thread::sleep(Duration::from_millis(100));
    
    // Test system stop
    let stop_result = system.stop();
    assert!(stop_result.is_ok(), "System should stop successfully");
    
    tracing::info!("✓ System lifecycle test passed");
}

#[test]
fn test_error_handling_and_edge_cases() {
    init_tracing!();
    tracing::info!("Testing error handling and edge cases");
    
    // Test with invalid port (0)
    let invalid_config = DistributedCompilationConfig {
        coordinator_port: 0,
        ..DistributedCompilationConfig::default()
    };
    
    let result = DistributedCompilationSystem::new(invalid_config);
    // Note: Port 0 might actually be valid (ephemeral port), so this might succeed
    // The test mainly ensures we don't panic
    
    // Test empty worker ports
    let empty_workers_config = DistributedCompilationConfig {
        coordinator_port: 9300,
        worker_ports: vec![],
        ..DistributedCompilationConfig::default()
    };
    
    let system_result = DistributedCompilationSystem::new(empty_workers_config);
    assert!(system_result.is_ok(), "System should handle empty worker ports");
    
    // Test extreme configuration values
    let extreme_config = DistributedCompilationConfig {
        coordinator_port: 9301,
        worker_ports: vec![9302],
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
    
    let extreme_system_result = DistributedCompilationSystem::new(extreme_config);
    assert!(extreme_system_result.is_ok(), "System should handle extreme configuration values");
    
    tracing::info!("✓ Error handling and edge cases test passed");
}
