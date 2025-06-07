use std::thread;
use std::time::Duration;

//! Memory allocation profiling tests
//!
//! This test module verifies the memory allocation profiling functionality
//! by creating various allocation patterns and checking the results.

extern crate cursed;


use cursed::memory::{
    GarbageCollector, Traceable, Tag, Visitor,
    enable_profiling, disable_profiling, reset_profiling, global_profiler
};

#[test]
fn test_memory_profiling_basic() {
    // Make sure profiling is enabled
    reset_profiling();
    enable_profiling();
    
    // Create a GC
    let gc = GarbageCollector::new();
    
    // Helper struct for testing allocation patterns
    #[derive(Debug, Clone)]
    struct TestObject {
        data: Vec<u8>,
        value: i32,
    }
    
    impl Traceable for TestObject {
        fn trace(&self, _visitor: &mut dyn Visitor) {
            // No references to trace
        }
        
        fn size(&self) -> usize {
            std::mem::size_of::<Self>() + self.data.capacity()
        }
        
        fn tag(&self) -> Tag {
            Tag::Object
        }
    }
    
    // Allocate some objects
    for i in 0..10 {
        // Create objects of different sizes
        let obj = TestObject {
            data: vec![0; i * 100],
            value: i,
        };
        
        // Allocate in different patterns
        if i % 2 == 0 {
            let _ = gc.allocate(obj);
        } else {
            let _ = gc.allocate_thread_safe(obj);
        }
    }
    
    // The profiler should have recorded these allocations
    let profiler = global_profiler();
    
    // Check the stats
    let stats = profiler.get_stats_by_type();
    assert!(!stats.is_empty(), "Should have allocation statistics");
    
    // TestObject should be in the stats
    let type_name = std::any::type_name::<TestObject>();
    assert!(stats.contains_key(type_name), "TestObject should be in stats");
    
    // Check TestObject stats
    let obj_stats = &stats[type_name];
    assert_eq!(obj_stats.count, 10, "Should have 10 TestObject allocations");
    
    // Test hot spots - might not have many yet
    let hot_spots = profiler.get_hot_spots(1);
    assert!(!hot_spots.is_empty(), "Should have some allocation hot spots");
    
    // Generate a report (just make sure it doesn't crash)
    let _report = profiler.generate_report();
    
    // Clean up
    drop(profiler);
    disable_profiling();
}

#[test]
fn test_memory_profiling_patterns() {
    // Make sure profiling is reset and enabled
    reset_profiling();
    enable_profiling();
    
    // Create a GC
    let gc = GarbageCollector::new();
    
    // Helper functions to create different allocation patterns
    let create_string_pattern = || {
        for _ in 0..20 {
            let s = "x".repeat(100);
            let _ = gc.allocate(s);
        }
    };
    
    let create_vector_pattern = || {
        for i in 0..15 {
            let v = vec![0; i * 10];
            let _ = gc.allocate(v);
        }
    };
    
    // Create distinct patterns
    create_string_pattern();
    thread::sleep(Duration::from_millis(10)); // Small pause between patterns
    create_vector_pattern();
    thread::sleep(Duration::from_millis(10));
    create_string_pattern();
    
    // The profiler should have detected these patterns
    let profiler = global_profiler();
    
    // We should see String and Vec<u8> in the stats
    let stats = profiler.get_stats_by_type();
    assert!(stats.contains_key("alloc::string::String"), "String should be in stats");
    
    // Check for allocation patterns
    let patterns = profiler.get_allocation_patterns();
    assert!(!patterns.is_empty(), "Should have detected allocation patterns");
    
    // Check the report
    let report = profiler.generate_report();
    assert!(report.contains("String"), "Report should mention String allocations");
    
    // Clean up
    drop(profiler);
    disable_profiling();
}

#[test]
fn test_memory_hot_paths() {
    // Make sure profiling is reset and enabled
    reset_profiling();
    enable_profiling();
    
    // Create a GC
    let gc = GarbageCollector::new();
    
    // A function that allocates a lot in a tight loop (hot path)
    fn hot_allocation_path(gc: &GarbageCollector) {
        for i in 0..50 {
            let data = vec![i as u8; 64]; // Small allocations, but many of them
            let _ = gc.allocate(data);
        }
    }
    
    // Call the hot path multiple times
    for _ in 0..5 {
        hot_allocation_path(&gc);
    }
    
    // The profiler should identify this as a hot spot
    let profiler = global_profiler();
    
    // Check hot spots
    let hot_spots = profiler.get_hot_spots(10);
    assert!(!hot_spots.is_empty(), "Should have detected hot spots");
    
    // The hot spot should reference test_memory_hot_paths or hot_allocation_path
    if !hot_spots.is_empty() {
        let found_hot_path = hot_spots.iter().any(|hs| {
            hs.identifier.contains("hot_allocation_path") ||
            hs.identifier.contains("test_memory_hot_paths")
        });
        
        assert!(found_hot_path, "Hot spot from hot_allocation_path not found");
    }
    
    // Clean up
    drop(profiler);
    disable_profiling();
}

#[test]
fn test_optimize_allocation_pattern() {
    // Make sure profiling is reset and enabled
    reset_profiling();
    enable_profiling();
    
    let gc = GarbageCollector::new();
    
    // Unoptimized version: creates many small strings
    fn unoptimized_fn(gc: &GarbageCollector) -> Vec<String> {
        let mut result = Vec::new();
        
        for i in 0..30 {
            // This allocates a new string for each iteration
            let s = format!("Item {}", i);
            let gc_s = gc.allocate(s);
            result.push(gc_s.inner().unwrap().clone());
        }
        
        result
    }
    
    // Optimized version: pre-allocates capacity and reuses buffer
    fn optimized_fn(gc: &GarbageCollector) -> Vec<String> {
        // Pre-allocate the result vector
        let mut result = Vec::with_capacity(30);
        let mut buffer = String::with_capacity(10); // Reuse this buffer
        
        for i in 0..30 {
            // Clear the buffer instead of allocating a new string
            buffer.clear();
            buffer.push_str("Item ");
            buffer.push_str(&i.to_string());
            
            // Clone only when we need to store it
            let gc_s = gc.allocate(buffer.clone());
            result.push(gc_s.inner().unwrap().clone());
        }
        
        result
    }
    
    // Run the unoptimized version first
    reset_profiling();
    enable_profiling();
    let _ = unoptimized_fn(&gc);
    
    // Check profiling results for unoptimized version
    let profiler = global_profiler();
    let unopt_stats = profiler.get_stats_by_type();
    let unopt_string_count = unopt_stats.get("alloc::string::String")
        .map(|s| s.count)
        .unwrap_or(0);
    
    // Run the optimized version
    reset_profiling();
    enable_profiling();
    let _ = optimized_fn(&gc);
    
    // Check profiling results for optimized version
    let profiler = global_profiler();
    let opt_stats = profiler.get_stats_by_type();
    let opt_string_count = opt_stats.get("alloc::string::String")
        .map(|s| s.count)
        .unwrap_or(0);
    
    // Optimized version should allocate fewer strings
    // Each has 30 result strings, but optimized reuses the buffer
    assert!(opt_string_count <= unopt_string_count, 
            "Optimized version should have fewer allocations (opt: {}, unopt: {})", 
            opt_string_count, unopt_string_count);
    
    // Clean up
    drop(profiler);
    disable_profiling();
}