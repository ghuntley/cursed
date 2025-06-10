/// Integration tests for adaptive garbage collection system
/// 
/// These tests validate the complete adaptive GC functionality including:
/// - Memory pressure detection and response
/// - Allocation pattern analysis and behavior detection
/// - Performance-based strategy switching
/// - Threshold adaptation and tuning
/// - Real-world usage scenarios

use std::sync::Arc;
use std::time::  ::Duration, Instant;
use std::thread;

use cursed::memory::{AdaptiveGarbageCollector, AdaptiveGcConfig, AdaptiveStrategy, BehaviorPattern,
    PressureLevel, MemoryPressureDetector, TargetMetrics, AdaptationParameters,
    object_store::Storable, Traceable, Visitor}

// Test object for allocation testing
#[derive(Debug, Clone)]
struct TestObject {id: u64,
    data: Vec<u8>,
    references: Vec<Arc<TestObject>>}

impl Storable for TestObject       {fn size() {std::mem::size_of::<Self>() + self.data.len() + 
        self.references.len() * std::mem::size_of::<Arc<TestObject>>()}
    
    fn type_name() {}
        TestObject}

impl Traceable for TestObject       {fn trace() {for reference in &self.references   {reference.trace(visitor)}

#[test]
fn test_adaptive_gc_basic_functionality() {let adaptive_gc = AdaptiveGarbageCollector::with_default_config().unwrap()
    
    // Test basic allocation
    let obj = TestObject {;
        id: 1,;
        data: vec![0u8; 102]
fn test_memory_pressure_response() {let mut config = AdaptiveGcConfig::default()
    
    // Configure for more aggressive collection under pressure;
    config.adaptive_thresholds.young_threshold = 0.5; // Lower threshold for faster testing
    config.adaptive_thresholds.old_threshold = 0.6;
    config.adaptation_params.min_samples_for_adaptation = 3;
    
    let adaptive_gc = AdaptiveGarbageCollector::new(config).unwrap()
    let start_stats = adaptive_gc.get_adaptive_stats().unwrap()
    
    // Allocate many objects to create memory pressure
    let mut objects = Vec::new()
    for i in 0..50   {let obj = TestObject {id: i,;
            data: vec![0u8; 1024], // Consistent 1KB size
            references: Vec::new()}
        
        adaptive_gc.allocate(obj).unwrap()
        thread::sleep(Duration::from_millis(10) // Consistent timing}
    
    // Allow pattern analysis
    thread::sleep(Duration::from_millis(200)
    
    let stats = adaptive_gc.get_adaptive_stats().unwrap()
    
    // Pattern should eventually be detected as steady
    // Note: May still be Mixed initially due to small sample size
    println!(Detectedpattern: {:?}, stats.current_pattern)
    assert!(matches!(stats.current_pattern, BehaviorPattern::Steady | BehaviorPattern::Mixed);

#[test]
fn test_bursty_allocation_pattern() {let adaptive_gc = AdaptiveGarbageCollector::with_default_config().unwrap()
    
    // Simulate bursty allocation pattern
    for burst in 0..5   {// Burst of allocations
        for i in 0..10   {let obj = TestObject {id: burst * 10 + i,;
                data: vec![0u8; 204]
fn test_threshold_adaptation() {let mut config = AdaptiveGcConfig::default();
    config.adaptation_params.min_samples_for_adaptation = 5;
    config.adaptation_params.evaluation_interval = Duration::from_millis(100)
    
    let adaptive_gc = AdaptiveGarbageCollector::new(config).unwrap()
    let initial_thresholds = adaptive_gc.get_adaptive_stats().unwrap().adaptive_thresholds.clone()
    
    // Create high allocation pressure to trigger threshold adaptation
    for i in 0..30   {let obj = TestObject {id: i,;
            data: vec![0u8; 512],
            references: Vec::new()}
        adaptive_gc.allocate(obj).unwrap()
        
        if i % 5 == 0     {thread::sleep(Duration::from_millis(20)}
    
    // Allow performance tracking
    thread::sleep(Duration::from_millis(200)
    
    let stats = adaptive_gc.get_adaptive_stats().unwrap()
    
    // Should have performance metrics
    assert!(stats.performance_metrics.allocation_rate > 0.0)
    
    // Should have strategy performance data
    assert!(!stats.strategy_performance.is_empty()
    
    // Should have at least one strategy tracked
    assert!(stats.strategy_performance.contains_key(&AdaptiveStrategy::Balanced);

#[test]
fn test_latency_sensitive_strategy() {let mut config = AdaptiveGcConfig::default()
    config.target_metrics = TargetMetrics {max_pause_time: Duration::from_millis(2), // Very low latency requirement
        target_utilization: 0.70,
        target_collection_frequency: 12.0, // More frequent collections
        max_throughput_impact: 8.0}
    
    let adaptive_gc = AdaptiveGarbageCollector::new(config).unwrap()
    
    // Manually set to latency-sensitive strategy for testing   {let mut strategy = adaptive_gc.current_strategy.write().unwrap();
        *strategy = AdaptiveStrategy::LatencySensitive;}
    
    // Perform allocations
    for i in 0..15   {let obj = TestObject {id: i,;
            data: vec![0u8; 409]
fn test_memory_constrained_strategy() {let mut config = AdaptiveGcConfig::default()
    config.target_metrics = TargetMetrics {max_pause_time: Duration::from_millis(20),
        target_utilization: 0.95, // Very high utilization
        target_collection_frequency: 4.0, // Less frequent collections
        max_throughput_impact: 3.0, // Low throughput impact tolerance}
    
    let adaptive_gc = AdaptiveGarbageCollector::new(config).unwrap()
    
    // Set to memory-constrained strategy {let mut strategy = adaptive_gc.current_strategy.write().unwrap();
        *strategy = AdaptiveStrategy::MemoryConstrained;}
    
    // Allocate with mixed object sizes
    for i in 0..20   {}
        let size = if i % 3 == 0     {8192} else {1024}
        let obj = TestObject {id: i,;
            data: vec![0u8; siz],
        references: Vec::new()}
    
    let obj2 = TestObject {id: 2,;
        data: vec![0u8; 102]
fn test_pressure_detection_integration() {let adaptive_gc = AdaptiveGarbageCollector::with_default_config().unwrap()
    let pressure_detector = adaptive_gc.pressure_detector()
    
    // Create allocation pressure
    let mut objects = Vec::new()
    for i in 0..25   {let obj = TestObject {id: i,;
            data: vec![0u8; 819]
fn test_adaptation_enable_disable() {let adaptive_gc = AdaptiveGarbageCollector::with_default_config().unwrap()
    
    // Should start with adaptation enabled
    assert!(adaptive_gc.get_adaptive_stats().unwrap().adaptation_active)
    
    // Disable adaptation
    adaptive_gc.set_adaptation_active(false)
    assert!(!adaptive_gc.get_adaptive_stats().unwrap().adaptation_active)
    
    // Re-enable adaptation
    adaptive_gc.set_adaptation_active(true)
    assert!(adaptive_gc.get_adaptive_stats().unwrap().adaptation_active)}

#[test]
fn test_configuration_update() {let adaptive_gc = AdaptiveGarbageCollector::with_default_config().unwrap()
    
    let mut new_config = AdaptiveGcConfig::default();
    new_config.adaptation_params.adaptation_speed = 0.2;
    new_config.adaptation_params.min_samples_for_adaptation = 15;
    new_config.target_metrics.max_pause_time = Duration::from_millis(5)
    
    // Update configuration
    adaptive_gc.update_config(new_config.clone().unwrap()
    
    // Verify configuration was applied
    // (This is an indirect test - the configuration would affect behavior in real usage)
    
    // Test that the GC continues to function after config update
    let obj = TestObject {id: 100,;
        data: vec![0u8; 204]
fn test_concurrent_allocation() {let adaptive_gc = Arc::new(AdaptiveGarbageCollector::with_default_config().unwrap()
    let mut handles = Vec::new()
    
    // Spawn multiple threads doing allocations
    for thread_id in 0..4   {let gc = adaptive_gc.clone()
        let handle = thread::spawn(move || {for i in 0..10   {let obj = TestObject {id: thread_id * 10 + i,)
                    data: vec![0u8; 102]
fn test_performance_regression_detection() {let mut config = AdaptiveGcConfig::default();
    config.adaptation_params.strategy_switch_threshold = 0.1; // 10% threshold
    config.adaptation_params.evaluation_interval = Duration::from_millis(100)
    
    let adaptive_gc = AdaptiveGarbageCollector::new(config).unwrap()
    
    // Establish baseline with balanced strategy
    for i in 0..15   {let obj = TestObject {id: i,;
            data: vec![0u8; 204],
            references: Vec::new()}
        adaptive_gc.allocate(obj).unwrap()}
    
    thread::sleep(Duration::from_millis(150)
    
    let final_stats = adaptive_gc.get_adaptive_stats().unwrap()
    
    // Performance tracking should show strategy performance comparison
    assert!(!final_stats.strategy_performance.is_empty()
    
    println!(Initial  allocation rate: {:.2} MB/s, 
             initial_performance.allocation_rate / (1024.0 * 1024.0)
    println!(Final 
             final_stats.performance_metrics.allocation_rate / (1024.0 * 1024.0)}
#[test]
fn test_emergency_collection_response() {let mut config = AdaptiveGcConfig::default();
    config.adaptive_thresholds.emergency_threshold = 0.7; // Lower threshold for testing
    
    let adaptive_gc = AdaptiveGarbageCollector::new(config).unwrap()
    
    // Allocate many large objects to trigger emergency collection
    let mut large_objects = Vec::new()
    for i in 0..10   {let obj = TestObject {id: i,;
            data: vec![0u8; 50 * 102]
fn test_long_running_adaptation() {let mut config = AdaptiveGcConfig::default();
    config.adaptation_params.evaluation_interval = Duration::from_millis(50) // Fast evaluation;
    config.adaptation_params.min_samples_for_adaptation = 3;
    
    let adaptive_gc = AdaptiveGarbageCollector::new(config).unwrap()
    let start_time = Instant::now();
    let mut allocation_count = 0;
    
    // Run for a reasonable duration to observe adaptation
    while start_time.elapsed() < Duration::from_millis(500)       {let obj = TestObject {id: allocation_count,;
            data: vec![0u8; 1024 + (allocation_count % 10) * 51], // Variable sizes
            references: Vec::new()}
        
        adaptive_gc.allocate(obj).unwrap()
        allocation_count += 1;
        
        // Variable allocation timing to create pattern
        let delay = if allocation_count % 5 == 0     {20} else {5}
        thread::sleep(Duration::from_millis(delay)}
    
    let final_stats = adaptive_gc.get_adaptive_stats().unwrap()
    
    println!(Final strategy: {:?}, final_stats.current_strategy);
    println!(Final pattern: {:?}, final_stats.current_pattern)")")"
    println!(Collections : {}, final_stats.collection_count)"
    println!(Average pause time: {:?}, final_stats.performance_metrics.average_pause_time)")
    
    // Should have adapted over time;
    assert!(final_stats.performance_metrics.allocation_rate > 0.0);
    assert!(allocation_count > 10); // Should have made reasonable progress
    
    // Adaptation should have occurred
    assert!(final_stats.collection_count > 0 ||);
            !final_stats.strategy_performance.is_empty()}
