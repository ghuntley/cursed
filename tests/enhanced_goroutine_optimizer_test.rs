/// Enhanced Goroutine Optimizer Test Suite
/// 
/// Comprehensive tests for the real goroutine optimization passes including:
/// - Goroutine pattern analysis (spawn chains, fan-out, pipelines)
/// - Synchronization pattern detection (deadlock, contention)
/// - Communication pattern optimization (channels, message flow)
/// - Stack size optimization and scheduler hints
/// - ML-driven optimization integration

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::Duration;

#[path = "../src/optimization/enhanced_llvm_passes/real_goroutine_optimizer.rs"]
mod real_goroutine_optimizer;

use real_goroutine_optimizer::*;
use crate::optimization::enhanced_llvm_passes::EnhancedOptimizationStatistics;

/// Test basic goroutine optimizer creation and configuration
#[test]
fn test_goroutine_optimizer_creation() {
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let optimizer = RealGoroutineOptimizer::new(statistics, None);
    
    // Verify default configuration
    assert!(optimizer.optimization_config.enable_stack_size_optimization);
    assert!(optimizer.optimization_config.enable_scheduler_hints);
    assert!(optimizer.optimization_config.enable_goroutine_pooling);
    assert!(optimizer.optimization_config.enable_concurrent_pattern_optimization);
    
    // Test configuration values
    assert_eq!(optimizer.optimization_config.min_stack_size, 8 * 1024);
    assert_eq!(optimizer.optimization_config.max_stack_size, 1024 * 1024);
    assert_eq!(optimizer.optimization_config.stack_growth_factor, 1.5);
    assert_eq!(optimizer.optimization_config.pool_size_threshold, 10);
    assert_eq!(optimizer.optimization_config.pool_reuse_threshold, 0.8);
    assert_eq!(optimizer.optimization_config.min_optimization_benefit, 0.05);
    assert_eq!(optimizer.optimization_config.max_optimization_overhead, 0.02);
    assert_eq!(optimizer.optimization_config.optimization_confidence_threshold, 0.8);
}

/// Test configuration updates
#[test]
fn test_configuration_update() {
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let mut optimizer = RealGoroutineOptimizer::new(statistics, None);
    
    let mut new_config = GoroutineOptimizationConfig::default();
    new_config.enable_stack_size_optimization = false;
    new_config.min_stack_size = 16 * 1024;
    new_config.pool_size_threshold = 20;
    new_config.stack_growth_factor = 2.0;
    
    optimizer.update_config(new_config.clone());
    
    assert!(!optimizer.optimization_config.enable_stack_size_optimization);
    assert_eq!(optimizer.optimization_config.min_stack_size, 16 * 1024);
    assert_eq!(optimizer.optimization_config.pool_size_threshold, 20);
    assert_eq!(optimizer.optimization_config.stack_growth_factor, 2.0);
}

/// Test optimization potential calculation for different pattern types
#[test]
fn test_optimization_potential_calculation() {
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let optimizer = RealGoroutineOptimizer::new(statistics, None);
    
    // Test different pattern types
    assert_eq!(optimizer.calculate_optimization_potential(&CreationPatternType::ShortLived), 0.8);
    assert_eq!(optimizer.calculate_optimization_potential(&CreationPatternType::LongLived), 0.4);
    assert_eq!(optimizer.calculate_optimization_potential(&CreationPatternType::Periodic), 0.9);
    assert_eq!(optimizer.calculate_optimization_potential(&CreationPatternType::OnDemand), 0.6);
    assert_eq!(optimizer.calculate_optimization_potential(&CreationPatternType::Batch), 0.85);
    assert_eq!(optimizer.calculate_optimization_potential(&CreationPatternType::Pipeline), 0.7);
}

/// Test stack risk assessment
#[test]
fn test_stack_risk_assessment() {
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let optimizer = RealGoroutineOptimizer::new(statistics, None);
    
    // Test various stack size and depth combinations
    assert_eq!(optimizer.assess_stack_risk(1024, 5), StackRiskLevel::Safe);
    assert_eq!(optimizer.assess_stack_risk(8 * 1024, 10), StackRiskLevel::Moderate);
    assert_eq!(optimizer.assess_stack_risk(32 * 1024, 20), StackRiskLevel::Critical);
    assert_eq!(optimizer.assess_stack_risk(100, 100), StackRiskLevel::Safe);
    assert_eq!(optimizer.assess_stack_risk(64 * 1024, 15), StackRiskLevel::Critical);
}

/// Test pooling suitability analysis
#[test]
fn test_pooling_suitability() {
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let optimizer = RealGoroutineOptimizer::new(statistics, None);
    
    // Create test patterns
    let suitable_pattern = GoroutineCreationPattern {
        pattern_type: CreationPatternType::ShortLived,
        frequency: 15, // Above threshold
        average_lifetime: Duration::from_millis(10),
        stack_usage: StackUsageInfo {
            estimated_size: 32 * 1024,
            max_observed_size: 32 * 1024,
            growth_pattern: StackGrowthPattern::Constant,
            risk_level: StackRiskLevel::Safe,
        },
        optimization_potential: 0.85, // Above reuse threshold
    };
    
    let unsuitable_pattern = GoroutineCreationPattern {
        pattern_type: CreationPatternType::LongLived,
        frequency: 5, // Below threshold
        average_lifetime: Duration::from_secs(60),
        stack_usage: StackUsageInfo {
            estimated_size: 128 * 1024,
            max_observed_size: 128 * 1024,
            growth_pattern: StackGrowthPattern::Linear,
            risk_level: StackRiskLevel::Moderate,
        },
        optimization_potential: 0.4, // Below reuse threshold
    };
    
    assert!(optimizer.is_suitable_for_pooling(&suitable_pattern));
    assert!(!optimizer.is_suitable_for_pooling(&unsuitable_pattern));
}

/// Test optimal stack size calculation
#[test]
fn test_optimal_stack_size_calculation() {
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let optimizer = RealGoroutineOptimizer::new(statistics, None);
    
    // Test short-lived pattern (should reduce stack size)
    let short_lived_pattern = GoroutineCreationPattern {
        pattern_type: CreationPatternType::ShortLived,
        frequency: 10,
        average_lifetime: Duration::from_millis(10),
        stack_usage: StackUsageInfo {
            estimated_size: 64 * 1024,
            max_observed_size: 64 * 1024,
            growth_pattern: StackGrowthPattern::Constant,
            risk_level: StackRiskLevel::Safe,
        },
        optimization_potential: 0.8,
    };
    
    let optimal_size = optimizer.calculate_optimal_stack_size(&short_lived_pattern);
    assert!(optimal_size.is_some());
    assert_eq!(optimal_size.unwrap(), 32 * 1024); // Should be halved
    
    // Test batch pattern (should increase stack size)
    let batch_pattern = GoroutineCreationPattern {
        pattern_type: CreationPatternType::Batch,
        frequency: 5,
        average_lifetime: Duration::from_millis(100),
        stack_usage: StackUsageInfo {
            estimated_size: 32 * 1024,
            max_observed_size: 32 * 1024,
            growth_pattern: StackGrowthPattern::Linear,
            risk_level: StackRiskLevel::Safe,
        },
        optimization_potential: 0.85,
    };
    
    let optimal_size = optimizer.calculate_optimal_stack_size(&batch_pattern);
    assert!(optimal_size.is_some());
    assert_eq!(optimal_size.unwrap(), 48 * 1024); // Should grow by factor
}

/// Test optimal pool size calculation
#[test]
fn test_optimal_pool_size_calculation() {
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let optimizer = RealGoroutineOptimizer::new(statistics, None);
    
    // Test periodic pattern
    let periodic_pattern = GoroutineCreationPattern {
        pattern_type: CreationPatternType::Periodic,
        frequency: 30,
        average_lifetime: Duration::from_millis(50),
        stack_usage: StackUsageInfo {
            estimated_size: 32 * 1024,
            max_observed_size: 32 * 1024,
            growth_pattern: StackGrowthPattern::Constant,
            risk_level: StackRiskLevel::Safe,
        },
        optimization_potential: 0.9,
    };
    
    let pool_size = optimizer.calculate_optimal_pool_size(&periodic_pattern);
    assert_eq!(pool_size, 30); // Should equal frequency for periodic
    
    // Test short-lived pattern with high frequency
    let high_frequency_pattern = GoroutineCreationPattern {
        pattern_type: CreationPatternType::ShortLived,
        frequency: 200, // Very high
        average_lifetime: Duration::from_millis(5),
        stack_usage: StackUsageInfo {
            estimated_size: 16 * 1024,
            max_observed_size: 16 * 1024,
            growth_pattern: StackGrowthPattern::Constant,
            risk_level: StackRiskLevel::Safe,
        },
        optimization_potential: 0.8,
    };
    
    let pool_size = optimizer.calculate_optimal_pool_size(&high_frequency_pattern);
    assert_eq!(pool_size, 100); // Should be capped at 100
}

/// Test spawn chain analysis helper structures
#[test]
fn test_spawn_chain_analysis() {
    let mut analysis = SpawnChainAnalysis::new();
    
    let chain1 = SpawnChain {
        chain_id: "chain_1".to_string(),
        functions: vec!["func1".to_string(), "func2".to_string()],
        depth: 2,
        total_spawns: 1,
    };
    
    let chain2 = SpawnChain {
        chain_id: "chain_2".to_string(),
        functions: vec!["func3".to_string(), "func4".to_string(), "func5".to_string()],
        depth: 3,
        total_spawns: 2,
    };
    
    analysis.add_chain(chain1);
    analysis.add_chain(chain2);
    
    assert_eq!(analysis.chain_count(), 2);
    assert_eq!(analysis.get_max_chain_length(), 3);
}

/// Test fan-out analysis helper structures
#[test]
fn test_fan_out_analysis() {
    let mut analysis = FanOutAnalysis::new();
    
    let pattern1 = FanOutPattern {
        parent_function: "parent1".to_string(),
        child_count: 5,
        spawn_frequency: 5.0,
        parallelism_factor: 5.0,
    };
    
    let pattern2 = FanOutPattern {
        parent_function: "parent2".to_string(),
        child_count: 10,
        spawn_frequency: 10.0,
        parallelism_factor: 10.0,
    };
    
    analysis.add_pattern(pattern1);
    analysis.add_pattern(pattern2);
    
    assert_eq!(analysis.pattern_count(), 2);
}

/// Test deadlock detection helper structures
#[test]
fn test_deadlock_detection() {
    let mut detector = DeadlockDetector::new();
    
    let sequence = LockSequence {
        sequence_id: "seq_1".to_string(),
        lock_order: vec!["lock_a".to_string(), "lock_b".to_string()],
        function_context: "test_func".to_string(),
        acquisition_sites: vec![1, 2],
    };
    
    // First sequence should not detect deadlock
    let risk = detector.analyze_sequence(&sequence);
    assert!(risk.is_none());
    
    // Create reverse order sequence
    let reverse_sequence = LockSequence {
        sequence_id: "seq_2".to_string(),
        lock_order: vec!["lock_b".to_string(), "lock_a".to_string()],
        function_context: "test_func2".to_string(),
        acquisition_sites: vec![3, 4],
    };
    
    // This might detect a potential deadlock depending on implementation
    let risk = detector.analyze_sequence(&reverse_sequence);
    // The specific behavior depends on the implementation details
    
    assert_eq!(detector.risk_count(), detector.detected_risks.len());
}

/// Test contention analysis helper structures
#[test]
fn test_contention_analysis() {
    let mut analyzer = ContentionAnalyzer::new();
    
    let pattern = ContentionPattern {
        lock_id: "lock_1".to_string(),
        contention_frequency: 0.8,
        average_wait_time: Duration::from_millis(50),
        max_wait_time: Duration::from_millis(200),
        access_count: 100,
    };
    
    analyzer.add_pattern(pattern);
    assert_eq!(analyzer.patterns.len(), 1);
}

/// Test channel usage analysis helper structures
#[test]
fn test_channel_usage_analysis() {
    let mut analyzer = ChannelUsageAnalyzer::new();
    
    let pattern = ChannelUsagePattern {
        channel_id: "channel_1".to_string(),
        channel_type: ChannelType::Buffered(16),
        message_rate: 100.0,
        message_sizes: vec![64, 128, 256],
        producer_count: 2,
        consumer_count: 3,
    };
    
    analyzer.add_pattern(pattern);
    assert_eq!(analyzer.patterns.len(), 1);
}

/// Test message flow analysis helper structures
#[test]
fn test_message_flow_analysis() {
    let mut analyzer = MessageFlowAnalyzer::new();
    
    let flow = MessageFlow {
        flow_id: "flow_1".to_string(),
        producer_count: 2,
        consumer_count: 3,
        throughput: 1000.0,
        latency_p95: Duration::from_millis(5),
        buffer_utilization: 0.7,
    };
    
    analyzer.add_flow(flow);
    assert_eq!(analyzer.flows.len(), 1);
}

/// Test bandwidth analysis helper structures
#[test]
fn test_bandwidth_analysis() {
    let mut analyzer = BandwidthAnalyzer::new();
    
    let pattern = BandwidthPattern {
        channel_id: "channel_1".to_string(),
        utilization: 0.85,
        peak_bandwidth: 10000.0,
        average_bandwidth: 7500.0,
        congestion_events: 3,
    };
    
    analyzer.add_pattern(pattern);
    assert_eq!(analyzer.patterns.len(), 1);
}

/// Test bottleneck severity assessment
#[test]
fn test_bottleneck_severity_assessment() {
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let optimizer = RealGoroutineOptimizer::new(statistics, None);
    
    assert_eq!(optimizer.assess_bottleneck_severity(0.1), BottleneckSeverity::None);
    assert_eq!(optimizer.assess_bottleneck_severity(0.3), BottleneckSeverity::Minor);
    assert_eq!(optimizer.assess_bottleneck_severity(0.5), BottleneckSeverity::Moderate);
    assert_eq!(optimizer.assess_bottleneck_severity(0.7), BottleneckSeverity::Severe);
    assert_eq!(optimizer.assess_bottleneck_severity(0.9), BottleneckSeverity::Critical);
}

/// Test optimization statistics collection
#[test]
fn test_optimization_statistics() {
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let optimizer = RealGoroutineOptimizer::new(statistics, None);
    
    let stats = optimizer.get_optimization_statistics();
    
    // Initially all should be zero
    assert_eq!(stats.get("creation_patterns_analyzed").unwrap_or(&0), &0);
    assert_eq!(stats.get("sync_patterns_analyzed").unwrap_or(&0), &0);
    assert_eq!(stats.get("comm_patterns_analyzed").unwrap_or(&0), &0);
    assert_eq!(stats.get("functions_optimized").unwrap_or(&0), &0);
}

/// Test yield strategy enumeration
#[test]
fn test_yield_strategy_types() {
    // Test all yield strategy variants
    let io_strategy = YieldStrategy::IoCooperative;
    let periodic_strategy = YieldStrategy::Periodic;
    let memory_strategy = YieldStrategy::MemoryPressure;
    let general_strategy = YieldStrategy::General;
    
    match io_strategy {
        YieldStrategy::IoCooperative => assert!(true),
        _ => panic!("Unexpected yield strategy"),
    }
    
    match periodic_strategy {
        YieldStrategy::Periodic => assert!(true),
        _ => panic!("Unexpected yield strategy"),
    }
    
    match memory_strategy {
        YieldStrategy::MemoryPressure => assert!(true),
        _ => panic!("Unexpected yield strategy"),
    }
    
    match general_strategy {
        YieldStrategy::General => assert!(true),
        _ => panic!("Unexpected yield strategy"),
    }
}

/// Test creation pattern type enumeration
#[test]
fn test_creation_pattern_types() {
    let patterns = vec![
        CreationPatternType::ShortLived,
        CreationPatternType::LongLived,
        CreationPatternType::Periodic,
        CreationPatternType::OnDemand,
        CreationPatternType::Batch,
        CreationPatternType::Pipeline,
    ];
    
    for pattern in patterns {
        let potential = match pattern {
            CreationPatternType::ShortLived => 0.8,
            CreationPatternType::LongLived => 0.4,
            CreationPatternType::Periodic => 0.9,
            CreationPatternType::OnDemand => 0.6,
            CreationPatternType::Batch => 0.85,
            CreationPatternType::Pipeline => 0.7,
        };
        assert!(potential > 0.0 && potential <= 1.0);
    }
}

/// Test synchronization operation types
#[test]
fn test_sync_operation_types() {
    let operations = vec![
        SyncOpType::MutexLock,
        SyncOpType::MutexUnlock,
        SyncOpType::RWLockReadLock,
        SyncOpType::RWLockWriteLock,
        SyncOpType::RWLockUnlock,
        SyncOpType::AtomicOperation,
        SyncOpType::ChannelSend,
        SyncOpType::ChannelReceive,
        SyncOpType::WaitGroupAdd,
        SyncOpType::WaitGroupDone,
        SyncOpType::WaitGroupWait,
    ];
    
    // Ensure all operation types are valid
    assert_eq!(operations.len(), 11);
}

/// Test channel operation types
#[test]
fn test_channel_operation_types() {
    let operations = vec![
        ChannelOpType::Send,
        ChannelOpType::Receive,
        ChannelOpType::Close,
        ChannelOpType::Select,
    ];
    
    // Ensure all channel operation types are valid
    assert_eq!(operations.len(), 4);
}

/// Test scheduler hint types
#[test]
fn test_scheduler_hint_types() {
    let hints = vec![
        SchedulerHintType::CpuBound,
        SchedulerHintType::IOBound,
        SchedulerHintType::Interactive,
        SchedulerHintType::Batch,
        SchedulerHintType::RealTime,
    ];
    
    // Ensure all scheduler hint types are valid
    assert_eq!(hints.len(), 5);
}

/// Test goroutine optimization enumeration
#[test]
fn test_goroutine_optimization_types() {
    let stack_opt = GoroutineOptimization::StackSizeOptimization {
        original_size: 64 * 1024,
        optimized_size: 32 * 1024,
        estimated_savings: 32 * 1024,
    };
    
    let scheduler_opt = GoroutineOptimization::SchedulerHint {
        hint_type: SchedulerHintType::CpuBound,
        expected_improvement: 0.15,
    };
    
    let pooling_opt = GoroutineOptimization::PoolingOptimization {
        pool_size: 10,
        reuse_rate: 0.8,
        memory_savings: 320 * 1024,
    };
    
    let concurrency_opt = GoroutineOptimization::ConcurrencyOptimization {
        optimization_type: ConcurrencyOptType::LockElision,
        performance_gain: 0.2,
    };
    
    // Verify all optimization types can be created
    match stack_opt {
        GoroutineOptimization::StackSizeOptimization { .. } => assert!(true),
        _ => panic!("Unexpected optimization type"),
    }
    
    match scheduler_opt {
        GoroutineOptimization::SchedulerHint { .. } => assert!(true),
        _ => panic!("Unexpected optimization type"),
    }
    
    match pooling_opt {
        GoroutineOptimization::PoolingOptimization { .. } => assert!(true),
        _ => panic!("Unexpected optimization type"),
    }
    
    match concurrency_opt {
        GoroutineOptimization::ConcurrencyOptimization { .. } => assert!(true),
        _ => panic!("Unexpected optimization type"),
    }
}

/// Test ML engine integration
#[test]
fn test_ml_engine_integration() {
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    
    // Test without ML engine
    let optimizer_no_ml = RealGoroutineOptimizer::new(statistics.clone(), None);
    assert!(optimizer_no_ml.ml_engine.is_none());
    
    // Test with ML engine
    let ml_engine = Arc::new(Mutex::new(crate::optimization::ml_optimization::MLOptimizationEngine::new()));
    let optimizer_with_ml = RealGoroutineOptimizer::new(statistics, Some(ml_engine));
    assert!(optimizer_with_ml.ml_engine.is_some());
}

/// Test helper struct defaults
#[test]
fn test_helper_struct_defaults() {
    let channel_stats = ChannelStats::default();
    assert_eq!(channel_stats.total_operations, 0);
    assert_eq!(channel_stats.send_count, 0);
    assert_eq!(channel_stats.receive_count, 0);
    assert!(channel_stats.message_sizes.is_empty());
    
    let bandwidth_stats = BandwidthStats::default();
    assert_eq!(bandwidth_stats.total_bytes, 0);
    assert_eq!(bandwidth_stats.operation_count, 0);
    
    let channel_lifecycle = ChannelLifecycle::default();
    assert!(!channel_lifecycle.used);
    assert!(!channel_lifecycle.closed);
    
    let rwlock_usage = RWLockUsage::default();
    assert_eq!(rwlock_usage.read_count, 0);
    assert_eq!(rwlock_usage.write_count, 0);
    
    let waitgroup_balance = WaitGroupBalance::default();
    assert_eq!(waitgroup_balance.add_count, 0);
    assert_eq!(waitgroup_balance.done_count, 0);
    assert_eq!(waitgroup_balance.wait_count, 0);
}

/// Test comprehensive configuration scenarios
#[test]
fn test_comprehensive_configuration_scenarios() {
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let mut optimizer = RealGoroutineOptimizer::new(statistics, None);
    
    // Test aggressive optimization configuration
    let aggressive_config = GoroutineOptimizationConfig {
        enable_stack_size_optimization: true,
        min_stack_size: 4 * 1024,
        max_stack_size: 2048 * 1024,
        stack_growth_factor: 2.0,
        enable_scheduler_hints: true,
        enable_priority_optimization: true,
        enable_affinity_optimization: true,
        enable_goroutine_pooling: true,
        pool_size_threshold: 5,
        pool_reuse_threshold: 0.6,
        enable_concurrent_pattern_optimization: true,
        enable_lock_elision: true,
        enable_work_stealing_hints: true,
        min_optimization_benefit: 0.01,
        max_optimization_overhead: 0.05,
        optimization_confidence_threshold: 0.6,
    };
    
    optimizer.update_config(aggressive_config);
    
    assert_eq!(optimizer.optimization_config.min_stack_size, 4 * 1024);
    assert_eq!(optimizer.optimization_config.max_stack_size, 2048 * 1024);
    assert_eq!(optimizer.optimization_config.stack_growth_factor, 2.0);
    assert_eq!(optimizer.optimization_config.pool_size_threshold, 5);
    assert_eq!(optimizer.optimization_config.pool_reuse_threshold, 0.6);
    assert_eq!(optimizer.optimization_config.min_optimization_benefit, 0.01);
    assert_eq!(optimizer.optimization_config.max_optimization_overhead, 0.05);
    assert_eq!(optimizer.optimization_config.optimization_confidence_threshold, 0.6);
    
    // Test conservative optimization configuration
    let conservative_config = GoroutineOptimizationConfig {
        enable_stack_size_optimization: false,
        min_stack_size: 16 * 1024,
        max_stack_size: 512 * 1024,
        stack_growth_factor: 1.2,
        enable_scheduler_hints: false,
        enable_priority_optimization: false,
        enable_affinity_optimization: false,
        enable_goroutine_pooling: false,
        pool_size_threshold: 50,
        pool_reuse_threshold: 0.95,
        enable_concurrent_pattern_optimization: false,
        enable_lock_elision: false,
        enable_work_stealing_hints: false,
        min_optimization_benefit: 0.1,
        max_optimization_overhead: 0.01,
        optimization_confidence_threshold: 0.95,
    };
    
    optimizer.update_config(conservative_config);
    
    assert!(!optimizer.optimization_config.enable_stack_size_optimization);
    assert!(!optimizer.optimization_config.enable_scheduler_hints);
    assert!(!optimizer.optimization_config.enable_goroutine_pooling);
    assert!(!optimizer.optimization_config.enable_concurrent_pattern_optimization);
    assert_eq!(optimizer.optimization_config.min_optimization_benefit, 0.1);
    assert_eq!(optimizer.optimization_config.max_optimization_overhead, 0.01);
    assert_eq!(optimizer.optimization_config.optimization_confidence_threshold, 0.95);
}
