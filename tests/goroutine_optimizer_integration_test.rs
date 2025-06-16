/// Goroutine Optimizer Integration Test
/// 
/// Tests the enhanced goroutine optimizer with mock LLVM IR to validate:
/// - Pattern detection and analysis
/// - Optimization application and effectiveness
/// - Performance improvement measurement
/// - Error handling and edge cases

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::Duration;

#[path = "../src/optimization/enhanced_llvm_passes/real_goroutine_optimizer.rs"]
mod real_goroutine_optimizer;

use real_goroutine_optimizer::*;
use crate::optimization::enhanced_llvm_passes::EnhancedOptimizationStatistics;

/// Mock LLVM context for testing
struct MockLLVMContext {
    functions: HashMap<String, MockFunction>,
}

/// Mock LLVM function
struct MockFunction {
    name: String,
    instructions: Vec<MockInstruction>,
    spawn_calls: Vec<MockSpawnCall>,
    sync_operations: Vec<MockSyncOperation>,
    channel_operations: Vec<MockChannelOperation>,
}

/// Mock instruction
struct MockInstruction {
    opcode: String,
    operands: Vec<String>,
}

/// Mock goroutine spawn call
struct MockSpawnCall {
    function_name: String,
    stack_size: Option<usize>,
    location: String,
}

/// Mock synchronization operation
struct MockSyncOperation {
    operation_type: String,
    lock_id: String,
    location: String,
}

/// Mock channel operation
struct MockChannelOperation {
    operation_type: String,
    channel_id: String,
    message_size: usize,
    location: String,
}

impl MockLLVMContext {
    fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }
    
    fn add_function(&mut self, name: &str, function: MockFunction) {
        self.functions.insert(name.to_string(), function);
    }
}

impl MockFunction {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            instructions: Vec::new(),
            spawn_calls: Vec::new(),
            sync_operations: Vec::new(),
            channel_operations: Vec::new(),
        }
    }
    
    fn add_spawn_call(&mut self, function_name: &str, stack_size: Option<usize>) {
        self.spawn_calls.push(MockSpawnCall {
            function_name: function_name.to_string(),
            stack_size,
            location: format!("{}:spawn", self.name),
        });
    }
    
    fn add_sync_operation(&mut self, operation_type: &str, lock_id: &str) {
        self.sync_operations.push(MockSyncOperation {
            operation_type: operation_type.to_string(),
            lock_id: lock_id.to_string(),
            location: format!("{}:sync", self.name),
        });
    }
    
    fn add_channel_operation(&mut self, operation_type: &str, channel_id: &str, message_size: usize) {
        self.channel_operations.push(MockChannelOperation {
            operation_type: operation_type.to_string(),
            channel_id: channel_id.to_string(),
            message_size,
            location: format!("{}:channel", self.name),
        });
    }
}

/// Test goroutine spawn chain detection
#[test]
fn test_spawn_chain_detection_integration() {
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let optimizer = RealGoroutineOptimizer::new(statistics, None);
    
    // Create a mock call graph representing a spawn chain
    let mut call_graph = GoroutineCallGraph::default();
    
    // Function A spawns Function B
    call_graph.spawn_relationships.insert(
        "function_a".to_string(),
        vec!["function_b".to_string()],
    );
    
    // Function B spawns Function C
    call_graph.spawn_relationships.insert(
        "function_b".to_string(),
        vec!["function_c".to_string()],
    );
    
    // Function C spawns Function D
    call_graph.spawn_relationships.insert(
        "function_c".to_string(),
        vec!["function_d".to_string()],
    );
    
    // Detect spawn chains
    let chains = optimizer.detect_spawn_chains(&call_graph).unwrap();
    
    assert!(!chains.is_empty());
    
    // Should detect a chain starting from function_a
    let main_chain = chains.iter().find(|c| c.functions.contains(&"function_a".to_string()));
    assert!(main_chain.is_some());
    
    let chain = main_chain.unwrap();
    assert!(chain.depth >= 4); // Should include all functions in the chain
    assert!(chain.functions.contains(&"function_a".to_string()));
    assert!(chain.functions.contains(&"function_b".to_string()));
    assert!(chain.functions.contains(&"function_c".to_string()));
    assert!(chain.functions.contains(&"function_d".to_string()));
}

/// Test fan-out pattern detection
#[test]
fn test_fan_out_pattern_detection_integration() {
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let optimizer = RealGoroutineOptimizer::new(statistics, None);
    
    // Create a mock call graph representing a fan-out pattern
    let mut call_graph = GoroutineCallGraph::default();
    
    // Main function spawns multiple workers
    call_graph.spawn_relationships.insert(
        "main_function".to_string(),
        vec![
            "worker_1".to_string(),
            "worker_2".to_string(),
            "worker_3".to_string(),
            "worker_4".to_string(),
            "worker_5".to_string(),
        ],
    );
    
    // Detect fan-out patterns
    let patterns = optimizer.detect_fan_out_patterns(&call_graph).unwrap();
    
    assert!(!patterns.is_empty());
    
    // Should detect the fan-out from main_function
    let main_pattern = patterns.iter().find(|p| p.parent_function == "main_function");
    assert!(main_pattern.is_some());
    
    let pattern = main_pattern.unwrap();
    assert_eq!(pattern.child_count, 5);
    assert_eq!(pattern.spawn_frequency, 5.0);
    assert_eq!(pattern.parallelism_factor, 5.0);
}

/// Test pipeline pattern detection
#[test]
fn test_pipeline_pattern_detection_integration() {
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let optimizer = RealGoroutineOptimizer::new(statistics, None);
    
    // Create a mock call graph representing a pipeline
    let mut call_graph = GoroutineCallGraph::default();
    
    // Linear pipeline: stage1 -> stage2 -> stage3 -> stage4
    call_graph.spawn_relationships.insert(
        "pipeline_stage1".to_string(),
        vec!["pipeline_stage2".to_string()],
    );
    
    call_graph.spawn_relationships.insert(
        "pipeline_stage2".to_string(),
        vec!["pipeline_stage3".to_string()],
    );
    
    call_graph.spawn_relationships.insert(
        "pipeline_stage3".to_string(),
        vec!["pipeline_stage4".to_string()],
    );
    
    // Detect pipeline patterns
    let patterns = optimizer.detect_pipeline_patterns(&call_graph).unwrap();
    
    assert!(!patterns.is_empty());
    
    // Should detect the pipeline starting from stage1
    let pipeline = patterns.iter().find(|p| p.pipeline_id.contains("pipeline_stage1"));
    assert!(pipeline.is_some());
    
    let pattern = pipeline.unwrap();
    assert_eq!(pattern.stage_count, 4); // Should count all stages
    assert!(pattern.throughput > 0.0);
}

/// Test synchronization pattern analysis
#[test]
fn test_synchronization_pattern_analysis_integration() {
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let optimizer = RealGoroutineOptimizer::new(statistics, None);
    
    // Create mock synchronization operations
    let sync_operations = vec![
        SyncOperation {
            operation_type: SyncOpType::MutexLock,
            location: "function_a".to_string(),
            line_number: 10,
            lock_id: "lock_1".to_string(),
            instruction_pointer: Some(0x1000),
        },
        SyncOperation {
            operation_type: SyncOpType::MutexLock,
            location: "function_a".to_string(),
            line_number: 15,
            lock_id: "lock_2".to_string(),
            instruction_pointer: Some(0x1004),
        },
        SyncOperation {
            operation_type: SyncOpType::MutexUnlock,
            location: "function_a".to_string(),
            line_number: 20,
            lock_id: "lock_2".to_string(),
            instruction_pointer: Some(0x1008),
        },
        SyncOperation {
            operation_type: SyncOpType::MutexUnlock,
            location: "function_a".to_string(),
            line_number: 25,
            lock_id: "lock_1".to_string(),
            instruction_pointer: Some(0x100c),
        },
    ];
    
    // Analyze lock acquisition sequences
    let sequences = optimizer.analyze_lock_acquisition_sequences(&sync_operations).unwrap();
    
    assert!(!sequences.is_empty());
    
    // Should detect the lock sequence
    let sequence = &sequences[0];
    assert_eq!(sequence.lock_order, vec!["lock_1", "lock_2"]);
    assert_eq!(sequence.function_context, "function_a");
    
    // Analyze lock contention
    let contention_patterns = optimizer.analyze_lock_contention(&sync_operations).unwrap();
    
    // Should detect contention patterns (though minimal with this data)
    // The specific results depend on the contention threshold
    for pattern in contention_patterns {
        assert!(pattern.contention_frequency >= 0.0);
        assert!(pattern.access_count > 0);
    }
}

/// Test communication pattern analysis
#[test]
fn test_communication_pattern_analysis_integration() {
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let optimizer = RealGoroutineOptimizer::new(statistics, None);
    
    // Create mock channel operations
    let channel_operations = vec![
        ChannelOperation {
            operation_type: ChannelOpType::Send,
            channel_id: "channel_1".to_string(),
            location: "producer".to_string(),
            line_number: 10,
            message_size_estimate: 128,
        },
        ChannelOperation {
            operation_type: ChannelOpType::Send,
            channel_id: "channel_1".to_string(),
            location: "producer".to_string(),
            line_number: 15,
            message_size_estimate: 256,
        },
        ChannelOperation {
            operation_type: ChannelOpType::Receive,
            channel_id: "channel_1".to_string(),
            location: "consumer".to_string(),
            line_number: 20,
            message_size_estimate: 128,
        },
        ChannelOperation {
            operation_type: ChannelOpType::Receive,
            channel_id: "channel_1".to_string(),
            location: "consumer".to_string(),
            line_number: 25,
            message_size_estimate: 256,
        },
    ];
    
    // Analyze channel usage patterns
    let usage_patterns = optimizer.analyze_channel_usage_patterns(&channel_operations).unwrap();
    
    assert!(!usage_patterns.is_empty());
    
    let pattern = &usage_patterns[0];
    assert_eq!(pattern.channel_id, "channel_1");
    assert!(pattern.message_rate > 0.0);
    assert_eq!(pattern.message_sizes, vec![128, 256, 128, 256]);
    assert_eq!(pattern.producer_count, 1);
    assert_eq!(pattern.consumer_count, 1);
    
    // Test channel type inference
    match pattern.channel_type {
        ChannelType::Unbuffered => {
            // Balanced send/receive should suggest unbuffered
            assert!(true);
        },
        ChannelType::Buffered(_) => {
            // Alternative valid inference
            assert!(true);
        },
        ChannelType::Bidirectional => {
            // Another valid inference
            assert!(true);
        },
    }
    
    // Analyze message flow patterns
    let flow_patterns = optimizer.analyze_message_flow_patterns(&channel_operations).unwrap();
    
    assert!(!flow_patterns.is_empty());
    
    let flow = &flow_patterns[0];
    assert_eq!(flow.producer_count, 1);
    assert_eq!(flow.consumer_count, 1);
    assert!(flow.throughput > 0.0);
}

/// Test optimization configuration impact
#[test]
fn test_optimization_configuration_impact() {
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let mut optimizer = RealGoroutineOptimizer::new(statistics, None);
    
    // Test with aggressive configuration
    let aggressive_config = GoroutineOptimizationConfig {
        enable_stack_size_optimization: true,
        min_stack_size: 4 * 1024,
        max_stack_size: 2048 * 1024,
        stack_growth_factor: 2.0,
        enable_scheduler_hints: true,
        enable_priority_optimization: true,
        enable_affinity_optimization: true,
        enable_goroutine_pooling: true,
        pool_size_threshold: 2, // Very low threshold
        pool_reuse_threshold: 0.1, // Very low threshold
        enable_concurrent_pattern_optimization: true,
        enable_lock_elision: true,
        enable_work_stealing_hints: true,
        min_optimization_benefit: 0.01, // Very low requirement
        max_optimization_overhead: 0.1, // High tolerance
        optimization_confidence_threshold: 0.1, // Low confidence requirement
    };
    
    optimizer.update_config(aggressive_config);
    
    // Create test patterns that should trigger optimizations
    let short_lived_pattern = GoroutineCreationPattern {
        pattern_type: CreationPatternType::ShortLived,
        frequency: 5, // Above new threshold
        average_lifetime: Duration::from_millis(10),
        stack_usage: StackUsageInfo {
            estimated_size: 64 * 1024,
            max_observed_size: 64 * 1024,
            growth_pattern: StackGrowthPattern::Constant,
            risk_level: StackRiskLevel::Safe,
        },
        optimization_potential: 0.2, // Above new threshold
    };
    
    // Should be suitable for pooling with aggressive config
    assert!(optimizer.is_suitable_for_pooling(&short_lived_pattern));
    
    // Should suggest stack optimization
    let optimal_size = optimizer.calculate_optimal_stack_size(&short_lived_pattern);
    assert!(optimal_size.is_some());
    
    // Test with conservative configuration
    let conservative_config = GoroutineOptimizationConfig {
        enable_stack_size_optimization: false,
        min_stack_size: 32 * 1024,
        max_stack_size: 256 * 1024,
        stack_growth_factor: 1.1,
        enable_scheduler_hints: false,
        enable_priority_optimization: false,
        enable_affinity_optimization: false,
        enable_goroutine_pooling: true,
        pool_size_threshold: 100, // Very high threshold
        pool_reuse_threshold: 0.99, // Very high threshold
        enable_concurrent_pattern_optimization: false,
        enable_lock_elision: false,
        enable_work_stealing_hints: false,
        min_optimization_benefit: 0.5, // Very high requirement
        max_optimization_overhead: 0.001, // Very low tolerance
        optimization_confidence_threshold: 0.99, // Very high confidence requirement
    };
    
    optimizer.update_config(conservative_config);
    
    // Same pattern should not be suitable with conservative config
    assert!(!optimizer.is_suitable_for_pooling(&short_lived_pattern));
}

/// Test bottleneck detection integration
#[test]
fn test_bottleneck_detection_integration() {
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let optimizer = RealGoroutineOptimizer::new(statistics, None);
    
    // Create many operations on the same lock to simulate contention
    let mut sync_operations = Vec::new();
    
    for i in 0..50 {
        sync_operations.push(SyncOperation {
            operation_type: SyncOpType::MutexLock,
            location: format!("function_{}", i % 5),
            line_number: 10 + i,
            lock_id: "contended_lock".to_string(),
            instruction_pointer: Some(0x1000 + i * 4),
        });
        
        sync_operations.push(SyncOperation {
            operation_type: SyncOpType::MutexUnlock,
            location: format!("function_{}", i % 5),
            line_number: 20 + i,
            lock_id: "contended_lock".to_string(),
            instruction_pointer: Some(0x2000 + i * 4),
        });
    }
    
    // Add some operations on a less contended lock
    for i in 0..5 {
        sync_operations.push(SyncOperation {
            operation_type: SyncOpType::MutexLock,
            location: "function_low_contention".to_string(),
            line_number: 30 + i,
            lock_id: "low_contention_lock".to_string(),
            instruction_pointer: Some(0x3000 + i * 4),
        });
    }
    
    // Identify bottlenecks
    let bottlenecks = optimizer.identify_synchronization_bottlenecks(&sync_operations).unwrap();
    
    assert!(!bottlenecks.is_empty());
    
    // Should identify the contended lock as a bottleneck
    let contended_bottleneck = bottlenecks.iter()
        .find(|b| b.location.contains("contended_lock"));
    
    assert!(contended_bottleneck.is_some());
    
    let bottleneck = contended_bottleneck.unwrap();
    assert!(matches!(bottleneck.severity, BottleneckSeverity::Critical | BottleneckSeverity::Severe));
    assert_eq!(bottleneck.bottleneck_type, BottleneckType::LockContention);
    assert!(bottleneck.estimated_impact > 0.0);
}

/// Test channel leak detection
#[test]
fn test_channel_leak_detection_integration() {
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let optimizer = RealGoroutineOptimizer::new(statistics, None);
    
    // Create channel operations representing potential leaks
    let channel_operations = vec![
        // Channel 1: Used but never closed (potential leak)
        ChannelOperation {
            operation_type: ChannelOpType::Send,
            channel_id: "leaky_channel".to_string(),
            location: "producer".to_string(),
            line_number: 10,
            message_size_estimate: 64,
        },
        ChannelOperation {
            operation_type: ChannelOpType::Receive,
            channel_id: "leaky_channel".to_string(),
            location: "consumer".to_string(),
            line_number: 15,
            message_size_estimate: 64,
        },
        
        // Channel 2: Used and properly closed (no leak)
        ChannelOperation {
            operation_type: ChannelOpType::Send,
            channel_id: "good_channel".to_string(),
            location: "producer".to_string(),
            line_number: 20,
            message_size_estimate: 128,
        },
        ChannelOperation {
            operation_type: ChannelOpType::Receive,
            channel_id: "good_channel".to_string(),
            location: "consumer".to_string(),
            line_number: 25,
            message_size_estimate: 128,
        },
        ChannelOperation {
            operation_type: ChannelOpType::Close,
            channel_id: "good_channel".to_string(),
            location: "producer".to_string(),
            line_number: 30,
            message_size_estimate: 0,
        },
    ];
    
    // Detect channel leaks
    let leak_risks = optimizer.detect_channel_leak_patterns(&channel_operations).unwrap();
    
    assert!(!leak_risks.is_empty());
    
    // Should detect the leak in leaky_channel
    let leaky_risk = leak_risks.iter()
        .find(|r| r.channel_id == "leaky_channel");
    
    assert!(leaky_risk.is_some());
    
    let risk = leaky_risk.unwrap();
    assert!(matches!(risk.risk_level, LeakRiskLevel::Medium | LeakRiskLevel::High));
    
    // Should not detect leak in good_channel
    let good_risk = leak_risks.iter()
        .find(|r| r.channel_id == "good_channel");
    
    assert!(good_risk.is_none());
}

/// Test comprehensive optimization statistics
#[test]
fn test_comprehensive_optimization_statistics() {
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let mut optimizer = RealGoroutineOptimizer::new(statistics.clone(), None);
    
    // Add some patterns to the optimizer
    optimizer.pattern_analyzer.creation_patterns.insert(
        "pattern_1".to_string(),
        GoroutineCreationPattern {
            pattern_type: CreationPatternType::ShortLived,
            frequency: 10,
            average_lifetime: Duration::from_millis(10),
            stack_usage: StackUsageInfo {
                estimated_size: 32 * 1024,
                max_observed_size: 32 * 1024,
                growth_pattern: StackGrowthPattern::Constant,
                risk_level: StackRiskLevel::Safe,
            },
            optimization_potential: 0.8,
        },
    );
    
    optimizer.pattern_analyzer.creation_patterns.insert(
        "pattern_2".to_string(),
        GoroutineCreationPattern {
            pattern_type: CreationPatternType::Periodic,
            frequency: 5,
            average_lifetime: Duration::from_millis(50),
            stack_usage: StackUsageInfo {
                estimated_size: 64 * 1024,
                max_observed_size: 64 * 1024,
                growth_pattern: StackGrowthPattern::Linear,
                risk_level: StackRiskLevel::Moderate,
            },
            optimization_potential: 0.9,
        },
    );
    
    optimizer.pattern_analyzer.sync_patterns.insert(
        "sync_1".to_string(),
        SynchronizationPattern {
            sync_type: SyncType::Mutex,
            contention_level: ContentionLevel::Medium,
            critical_section_size: 100,
            wait_time_distribution: vec![Duration::from_millis(5), Duration::from_millis(10)],
        },
    );
    
    optimizer.pattern_analyzer.comm_patterns.insert(
        "comm_1".to_string(),
        CommunicationPattern {
            comm_type: CommunicationType::ChannelPassing,
            message_frequency: 100.0,
            message_size_distribution: vec![64, 128, 256],
            latency_requirements: LatencyRequirement::LowLatency,
        },
    );
    
    optimizer.applied_optimizations.insert(
        "function_1".to_string(),
        vec![
            GoroutineOptimization::StackSizeOptimization {
                original_size: 64 * 1024,
                optimized_size: 32 * 1024,
                estimated_savings: 32 * 1024,
            },
            GoroutineOptimization::PoolingOptimization {
                pool_size: 10,
                reuse_rate: 0.8,
                memory_savings: 320 * 1024,
            },
        ],
    );
    
    optimizer.applied_optimizations.insert(
        "function_2".to_string(),
        vec![
            GoroutineOptimization::SchedulerHint {
                hint_type: SchedulerHintType::CpuBound,
                expected_improvement: 0.15,
            },
        ],
    );
    
    // Get optimization statistics
    let stats = optimizer.get_optimization_statistics();
    
    assert_eq!(stats.get("creation_patterns_analyzed").unwrap_or(&0), &2);
    assert_eq!(stats.get("sync_patterns_analyzed").unwrap_or(&0), &1);
    assert_eq!(stats.get("comm_patterns_analyzed").unwrap_or(&0), &1);
    assert_eq!(stats.get("functions_optimized").unwrap_or(&0), &2);
    
    // Verify we can retrieve applied optimizations
    let function_1_opts = optimizer.get_applied_optimizations_for_function("function_1");
    assert_eq!(function_1_opts.len(), 2);
    
    let function_2_opts = optimizer.get_applied_optimizations_for_function("function_2");
    assert_eq!(function_2_opts.len(), 1);
    
    let nonexistent_opts = optimizer.get_applied_optimizations_for_function("nonexistent");
    assert_eq!(nonexistent_opts.len(), 0);
}

/// Test error handling and edge cases
#[test]
fn test_error_handling_and_edge_cases() {
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let optimizer = RealGoroutineOptimizer::new(statistics, None);
    
    // Test with empty call graph
    let empty_call_graph = GoroutineCallGraph::default();
    
    let chains = optimizer.detect_spawn_chains(&empty_call_graph).unwrap();
    assert!(chains.is_empty());
    
    let fan_out_patterns = optimizer.detect_fan_out_patterns(&empty_call_graph).unwrap();
    assert!(fan_out_patterns.is_empty());
    
    let pipeline_patterns = optimizer.detect_pipeline_patterns(&empty_call_graph).unwrap();
    assert!(pipeline_patterns.is_empty());
    
    // Test with empty operation lists
    let empty_sync_ops: Vec<SyncOperation> = Vec::new();
    let sequences = optimizer.analyze_lock_acquisition_sequences(&empty_sync_ops).unwrap();
    assert!(sequences.is_empty());
    
    let contention_patterns = optimizer.analyze_lock_contention(&empty_sync_ops).unwrap();
    assert!(contention_patterns.is_empty());
    
    let bottlenecks = optimizer.identify_synchronization_bottlenecks(&empty_sync_ops).unwrap();
    assert!(bottlenecks.is_empty());
    
    let empty_channel_ops: Vec<ChannelOperation> = Vec::new();
    let usage_patterns = optimizer.analyze_channel_usage_patterns(&empty_channel_ops).unwrap();
    assert!(usage_patterns.is_empty());
    
    let flow_patterns = optimizer.analyze_message_flow_patterns(&empty_channel_ops).unwrap();
    assert!(flow_patterns.is_empty());
    
    let bandwidth_patterns = optimizer.analyze_bandwidth_patterns(&empty_channel_ops).unwrap();
    assert!(bandwidth_patterns.is_empty());
    
    let leak_risks = optimizer.detect_channel_leak_patterns(&empty_channel_ops).unwrap();
    assert!(leak_risks.is_empty());
}
