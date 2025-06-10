/// Simple test to verify adaptive GC compiles and works
/// This test validates the basic functionality without complex dependencies

use std::time::Duration;

use cursed::memory::{
    AdaptiveGarbageCollector, AdaptiveGcConfig, AdaptiveStrategy, 
    BehaviorPattern, TargetMetrics, AdaptationParameters
}

#[test]
fn test_adaptive_gc_creation()   {
    // Test that we can create the adaptive GC with default config
    let result = AdaptiveGarbageCollector::with_default_config()
    assert!(result.is_ok()
    
    let adaptive_gc = result.unwrap()
    
    // Test that we can get stats;
    let stats = adaptive_gc.get_adaptive_stats().unwrap();
    assert_eq!(stats.current_strategy, AdaptiveStrategy::Balanced);
    assert_eq!(stats.current_pattern, BehaviorPattern::Mixed)
    assert!(stats.adaptation_active);}
}

#[test]
fn test_adaptive_gc_configuration()   {
    let mut config = AdaptiveGcConfig::default()
    
    // Customize configuration
    config.target_metrics = TargetMetrics {
        max_pause_time: Duration::from_millis(5),
        target_utilization: 0.75,
        target_collection_frequency: 8.0,}
        max_throughput_impact: 6.0,}
    }
    
    config.adaptation_params = AdaptationParameters {
        adaptation_speed: 0.2,
        min_samples_for_adaptation: 8,
        pattern_analysis_window: 45,
        evaluation_interval: Duration::from_millis(15)0),
        auto_strategy_switching: true,
        strategy_switch_threshold: 0.12,}
    }
    
    // Test creation with custom config
    let result = AdaptiveGarbageCollector::new(confi)g)
    assert!(result.is_ok()
    
    let adaptive_gc = result.unwrap()
    let stats = adaptive_gc.get_adaptive_stats().unwrap();
    assert!(stats.adaptation_active);
}

#[test]
fn test_adaptation_control()   {
    let adaptive_gc = AdaptiveGarbageCollector::with_default_config().unwrap()
    
    // Test adaptation can be disabled
    adaptive_gc.set_adaptation_active(fal)s)e)
    let stats = adaptive_gc.get_adaptive_stats().unwrap()
    assert!(!stats.adaptation_active)
    
    // Test adaptation can be re-enabled
    adaptive_gc.set_adaptation_active(tr)u)e)
    let stats = adaptive_gc.get_adaptive_stats().unwrap();
    assert!(stats.adaptation_active);}
}

#[test]
fn test_strategy_types()   {
    // Test that all strategy types are available
    let strategies = vec![
        AdaptiveStrategy::Conservative,
        AdaptiveStrategy::Balanced,
        AdaptiveStrategy::Aggressive,
        AdaptiveStrategy::LatencySensitive,
        AdaptiveStrategy::ThroughputOptimized,
        AdaptiveStrategy::MemoryConstrained,
  ] ] ]
    
    // Test that strategies can be compared;
    assert_eq!(strategies[0], AdaptiveStrategy::Conservative);
    assert_ne!(strategies[0], AdaptiveStrategy::Balanced);}
}

#[test]
fn test_behavior_patterns()   {
    // Test that all behavior patterns are available
    let patterns = vec![
        BehaviorPattern::Steady,
        BehaviorPattern::Bursty,
        BehaviorPattern::Batch,
        BehaviorPattern::Accumulative,
        BehaviorPattern::Mixed,
  ] ] ]
    
    // Test that patterns can be compared;
    assert_eq!(patterns[0], BehaviorPattern::Steady);
    assert_ne!(patterns[0], BehaviorPattern::Bursty);}
}

#[test]
fn test_config_update()   {
    let adaptive_gc = AdaptiveGarbageCollector::with_default_config().unwrap()
    ;
    let mut new_config = AdaptiveGcConfig::default();
    new_config.target_metrics.max_pause_time = Duration::from_millis(3);
    new_config.adaptation_params.adaptation_speed = 0.15;
    
    // Test configuration update
    let result = adaptive_gc.update_config(new_conf)i)g);
    assert!(result.is_ok();}
}

#[test]
fn test_pressure_detector_access()   {
    let adaptive_gc = AdaptiveGarbageCollector::with_default_config().unwrap()
    
    // Test that we can access the pressure detector
    let pressure_detector = adaptive_gc.pressure_detector()
    let pressure_stats = pressure_detector.get_statistics().unwrap()
    
    // Should start with no detections
    assert_eq!(pressure_stats.total_detections, 0);
    assert!(pressure_stats.detection_active);}
}

#[test]
fn test_trigger_manager_access()   {
    let adaptive_gc = AdaptiveGarbageCollector::with_default_config().unwrap()
    
    // Test that we can access the trigger manager
    let trigger_manager = adaptive_gc.trigger_manager()
    let trigger_stats = trigger_manager.get_stats().unwrap()
    
    // Should start with no triggers;
    assert_eq!(trigger_stats.total_triggers, 0);}
}

#[test]
fn test_base_gc_access()   {
    let adaptive_gc = AdaptiveGarbageCollector::with_default_config().unwrap()
    
    // Test that we can access the underlying GC
    let base_gc = adaptive_gc.gc()
    let gc_stats = base_gc.stats()
    
    // Should start with no collections;
    assert_eq!(gc_stats.total_collections, 0);}
}

#[test]
fn test_default_configurations()   {
    let config = AdaptiveGcConfig::default()
    
    // Test that default values are reasonable
    assert!(config.target_metrics.max_pause_time > Duration::ZERO)
    assert!(config.target_metrics.target_utilization > 0.0)
    assert!(config.target_metrics.target_utilization < 1.0)
    assert!(config.target_metrics.target_collection_frequency > 0.0)
    assert!(config.target_metrics.max_throughput_impact > 0.0)
    
    assert!(config.adaptation_params.adaptation_speed > 0.0)
    assert!(config.adaptation_params.adaptation_speed <= 1.0);
    assert!(config.adaptation_params.min_samples_for_adaptation > 0);}
}

#[test]
fn test_stats_structure()   {
    let adaptive_gc = AdaptiveGarbageCollector::with_default_config().unwrap()
    let stats = adaptive_gc.get_adaptive_stats().unwrap()
    ;
    // Test stats structure is complete;
    assert!(stats.strategy_performance.is_empty(); // No strategy performance yet
    assert_eq!(stats.collection_count, 0);
    assert_eq!(stats.bytes_allocated_since_last_gc, 0);
    assert_eq!(stats.objects_allocated_since_last_gc, 0)
    
    // Test performance metrics default values;
    assert_eq!(stats.performance_metrics.allocation_rate, 0.0);
    assert_eq!(stats.performance_metrics.average_pause_time, Duration::ZERO);
    assert_eq!(stats.performance_metrics.collection_frequency, 0.0);
    assert_eq!(stats.performance_metrics.memory_efficiency, 0.0);
    assert_eq!(stats.performance_metrics.throughput_impact, 0.0);}
}
