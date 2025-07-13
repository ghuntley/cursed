use cursed::memory::profiling::*;
use cursed::memory::Tag;
use std::time::Duration;
use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[test]
#[ignore = "Memory profiling tests disabled for fast test runs"]
fn test_comprehensive_memory_profiling() {
    println!("=== Comprehensive Memory Profiling Integration Test ===");
    
    // Create profiler with comprehensive configuration
    let mut config = ProfilingConfig::default();
    config.enable_allocation_tracking = true;
    config.enable_leak_detection = true;
    config.enable_performance_profiling = true;
    config.enable_heap_analysis = true;
    config.sampling_rate = 1.0; // 100% sampling for test
    config.stack_trace_depth = 5;
    
    let profiler = MemoryProfiler::new(config).unwrap();
    
    // Test 1: Record various allocations
    println!("1. Recording memory allocations...");
    
    let mut allocation_ids = Vec::new();
    
    // Small objects
    for i in 0..5 {
        let id = profiler.record_allocation(
            0x1000 + i * 0x100,
            64 + i * 16,
            Tag::Object,
            format!("small_object_{}", i),
            8,
        ).unwrap();
        allocation_ids.push(id);
    }
    
    // Large allocations
    for i in 0..3 {
        let id = profiler.record_allocation(
            0x10000 + i * 0x1000,
            1024 + i * 512,
            Tag::Array,
            format!("large_array_{}", i),
            16,
        ).unwrap();
        allocation_ids.push(id);
    }
    
    // String allocations
    for i in 0..4 {
        let id = profiler.record_allocation(
            0x20000 + i * 0x200,
            128 + i * 32,
            Tag::String,
            format!("string_{}", i),
            8,
        ).unwrap();
        allocation_ids.push(id);
    }
    
    println!("   Recorded {} allocations", allocation_ids.len());
    
    // Test 2: Real-time monitoring
    println!("2. Testing real-time monitoring...");
    
    let callback_called = Arc::new(AtomicBool::new(false));
    let callback_called_clone = callback_called.clone();
    
    profiler.register_monitor_callback(move |snapshot| {
        println!("   Real-time update: {} bytes used, {} active allocations", 
                 snapshot.current_usage, snapshot.active_allocations);
        callback_called_clone.store(true, Ordering::Relaxed);
    });
    
    profiler.take_snapshot().unwrap();
    assert!(callback_called.load(Ordering::Relaxed));
    
    // Test 3: Memory snapshots
    println!("3. Analyzing memory snapshots...");
    
    let snapshots = profiler.get_snapshots(1);
    assert!(!snapshots.is_empty());
    
    let snapshot = &snapshots[0];
    println!("   Current usage: {} bytes", snapshot.current_usage);
    println!("   Active allocations: {}", snapshot.active_allocations);
    println!("   Fragmentation: {:.2}%", snapshot.fragmentation * 100.0);
    
    assert!(snapshot.current_usage > 0);
    assert!(snapshot.active_allocations > 0);
    assert!(snapshot.fragmentation >= 0.0 && snapshot.fragmentation <= 1.0);
    
    // Test 4: Performance metrics
    println!("4. Analyzing performance metrics...");
    
    profiler.update_performance_metrics().unwrap();
    
    let metrics = profiler.get_performance_metrics();
    println!("   Allocation throughput: {:.2} allocs/sec", metrics.allocation_throughput);
    println!("   Memory bandwidth: {:.2} MB/sec", metrics.memory_bandwidth / 1_000_000.0);
    println!("   Cache hit rate: {:.2}%", metrics.cache_hit_rate * 100.0);
    println!("   Memory pressure: {:.2}%", metrics.memory_pressure * 100.0);
    
    assert!(metrics.allocation_throughput > 0.0);
    assert!(metrics.memory_bandwidth > 0.0);
    assert!(metrics.cache_hit_rate >= 0.0 && metrics.cache_hit_rate <= 1.0);
    assert!(metrics.memory_pressure >= 0.0 && metrics.memory_pressure <= 1.0);
    
    // Test 5: Heap analysis
    println!("5. Performing heap analysis...");
    
    profiler.analyze_heap().unwrap();
    
    let analysis = profiler.get_heap_analysis().unwrap();
    println!("   Total heap size: {} bytes", analysis.total_heap_size);
    println!("   Used heap size: {} bytes", analysis.used_heap_size);
    println!("   GC efficiency: {:.2}%", analysis.gc_impact.gc_efficiency * 100.0);
    
    assert!(analysis.total_heap_size > 0);
    assert!(analysis.used_heap_size > 0);
    assert!(analysis.gc_impact.gc_efficiency > 0.0);
    
    // Test 6: Allocation patterns
    println!("6. Analyzing allocation patterns...");
    
    let patterns = profiler.analyze_allocation_patterns().unwrap();
    
    println!("   Size classes: {}", patterns.size_patterns.len());
    println!("   Tag types: {}", patterns.tag_patterns.len());
    
    assert!(!patterns.size_patterns.is_empty());
    assert!(!patterns.tag_patterns.is_empty());
    assert!(patterns.tag_patterns.contains_key(&Tag::Object));
    assert!(patterns.tag_patterns.contains_key(&Tag::Array));
    assert!(patterns.tag_patterns.contains_key(&Tag::String));
    
    // Test 7: Leak detection
    println!("7. Testing leak detection...");
    
    // Create potential leak
    let leak_id = profiler.record_allocation(
        0x30000,
        2048,
        Tag::Object,
        "potential_leak".to_string(),
        8,
    ).unwrap();
    
    // Make it appear old
    profiler.update_allocation_timestamp(leak_id, std::time::Instant::now() - Duration::from_secs(600)).unwrap();
    
    let leak_candidates = profiler.detect_leaks_with_stack_traces().unwrap();
    println!("   Found {} potential leaks", leak_candidates.len());
    
    assert!(!leak_candidates.is_empty());
    assert!(leak_candidates.iter().any(|c| c.allocation.id == leak_id));
    
    // Test 8: Optimization suggestions
    println!("8. Generating optimization suggestions...");
    
    let suggestions = profiler.generate_optimization_suggestions();
    println!("   Generated {} suggestions", suggestions.len());
    
    assert!(!suggestions.is_empty());
    
    // Test 9: Memory deallocations
    println!("9. Testing memory deallocations...");
    
    for &id in allocation_ids.iter().take(3) {
        profiler.record_deallocation(id, 0x1000).unwrap();
    }
    
    // Take final snapshot
    profiler.take_snapshot().unwrap();
    
    // Test 10: Report generation
    println!("10. Generating memory report...");
    
    let report = profiler.generate_report().unwrap();
    assert!(report.contains("CURSED Memory Profile Report"));
    assert!(report.contains("Current Memory Usage"));
    assert!(report.contains("Performance Metrics"));
    
    println!("    Generated {} character report", report.len());
    
    // Test 11: Statistics
    println!("11. Checking profiling statistics...");
    
    let stats = profiler.get_stats();
    println!("    Memory overhead: {} bytes", stats.memory_overhead);
    println!("    Profiling accuracy: {:.2}%", stats.accuracy * 100.0);
    
    // Clean shutdown
    profiler.shutdown().unwrap();
    
    println!("=== All tests passed! ===");
}

#[test]
#[ignore = "Memory profiling tests disabled for fast test runs"]
fn test_fragmentation_calculation_accuracy() {
    let config = ProfilingConfig::default();
    let profiler = MemoryProfiler::new(config).unwrap();
    
    // Test with known fragmentation scenario
    let mut allocations = std::collections::HashMap::new();
    
    // Create allocations with specific gaps
    allocations.insert(1, AllocationRecord {
        id: 1,
        address: 0x1000,
        size: 100,
        tag: Tag::Object,
        timestamp: std::time::Instant::now(),
        thread_id: thread::current().id(),
        stack_trace: Vec::new(),
        source: "test".to_string(),
        alignment: 8,
        lifetime: None,
    });
    
    allocations.insert(2, AllocationRecord {
        id: 2,
        address: 0x2000, // 3996 byte gap
        size: 200,
        tag: Tag::String,
        timestamp: std::time::Instant::now(),
        thread_id: thread::current().id(),
        stack_trace: Vec::new(),
        source: "test".to_string(),
        alignment: 8,
        lifetime: None,
    });
    
    let fragmentation = profiler.calculate_fragmentation(&allocations);
    
    // Should detect significant fragmentation
    assert!(fragmentation > 0.5, "Expected high fragmentation, got {:.2}%", fragmentation * 100.0);
    assert!(fragmentation < 1.0, "Fragmentation should be less than 100%");
    
    println!("Fragmentation test: {:.2}% (expected > 50%)", fragmentation * 100.0);
}

#[test]
#[ignore = "Memory profiling tests disabled for fast test runs"]
fn test_cache_hit_rate_analysis() {
    let config = ProfilingConfig::default();
    let profiler = MemoryProfiler::new(config).unwrap();
    
    let mut allocations = std::collections::HashMap::new();
    let now = std::time::Instant::now();
    
    // Create allocations that should have good cache locality
    for i in 0..10 {
        allocations.insert(i, AllocationRecord {
            id: i,
            address: 0x1000 + i * 64, // Sequential addresses
            size: 64,
            tag: Tag::Object,
            timestamp: now + Duration::from_millis(i as u64 * 10), // Close timing
            thread_id: thread::current().id(),
            stack_trace: Vec::new(),
            source: "test".to_string(),
            alignment: 8,
            lifetime: None,
        });
    }
    
    let cache_hit_rate = profiler.calculate_cache_hit_rate(&allocations);
    
    // Should have good cache hit rate due to locality
    assert!(cache_hit_rate > 0.5, "Expected good cache hit rate, got {:.2}%", cache_hit_rate * 100.0);
    
    println!("Cache hit rate test: {:.2}% (expected > 50%)", cache_hit_rate * 100.0);
}

#[test]
#[ignore = "Memory profiling tests disabled for fast test runs"]
fn test_memory_pressure_calculation() {
    let config = ProfilingConfig::default();
    let profiler = MemoryProfiler::new(config).unwrap();
    
    // Simulate memory usage
    profiler.get_real_time_monitor().record_allocation(1024);
    profiler.get_real_time_monitor().record_allocation(2048);
    profiler.get_real_time_monitor().record_allocation(4096);
    
    let pressure = profiler.calculate_memory_pressure(7168);
    
    assert!(pressure >= 0.0 && pressure <= 1.0, "Memory pressure should be between 0 and 1, got {}", pressure);
    
    println!("Memory pressure test: {:.2}% (valid range)", pressure * 100.0);
}

#[test]
#[ignore = "Memory profiling tests disabled for fast test runs"]
fn test_enhanced_leak_detection_accuracy() {
    let mut config = ProfilingConfig::default();
    config.enable_leak_detection = true;
    config.sampling_rate = 1.0;
    
    let profiler = MemoryProfiler::new(config).unwrap();
    
    // Record allocation with specific characteristics for leak detection
    let allocation_id = profiler.record_allocation(
        0x1000,
        1024,
        Tag::Object,
        "leak_test".to_string(),
        8,
    ).unwrap();
    
    // Make it appear very old
    profiler.update_allocation_timestamp(allocation_id, std::time::Instant::now() - Duration::from_secs(700)).unwrap();
    
    // Add stack trace that should increase leak probability (simulated in test)
    // In real usage, stack traces would be captured automatically
    
    let leak_candidates = profiler.detect_leaks_with_stack_traces().unwrap();
    
    assert!(!leak_candidates.is_empty(), "Should detect at least one leak candidate");
    
    let leak_candidate = &leak_candidates[0];
    assert_eq!(leak_candidate.allocation.id, allocation_id);
    assert!(leak_candidate.probability > 0.5, "Leak probability should be high for old allocation with malloc stack trace");
    
    println!("Enhanced leak detection test: {:.2}% probability (expected > 50%)", leak_candidate.probability * 100.0);
}
