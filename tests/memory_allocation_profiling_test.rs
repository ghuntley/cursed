// Memory allocation profiling tests
//
// This test module verifies the memory allocation profiling functionality
// by creating various allocation patterns and checking the results.

extern crate cursed;

use std::thread;
use std::time::Duration;

use cursed::memory::{GarbageCollector, Traceable, Tag, Visitor,
    enable_profiling, disable_profiling, reset_profiling, global_profiler}

// Wrapper types for testing - these implement Traceable
#[derive(Debug, Clone)]
struct GcString(pub String)

impl Traceable for GcString       {fn trace() {// Strings don t contain references to other GC objects}

unsafe impl Send for TestObject       {}
unsafe impl Sync for TestObject       {}
    
    fn size() {std::mem::size_of::<Self>() + self.0.capacity()}
    
    fn tag() {Tag::String}

#[derive(Debug, Clone)]
struct GcVecU8(pub Vec<u8>

impl Traceable for GcVecU8       {fn trace() {// Vec<u8> doesnt contain references to other GC objects}

unsafe impl Send for TestObject       {}
unsafe impl Sync for TestObject       {}
    
    fn size() {std::mem::size_of::<Self>() + self.0.capacity()}
    
    fn tag() {Tag::Array}

#[derive(Debug, Clone)]
struct GcVecInt(pub Vec<i32>

impl Traceable for GcVecInt       {fn trace() {// Vec<i32> doesnt contain references to other GC objects}

unsafe impl Send for TestObject       {}
unsafe impl Sync for TestObject       {}
    
    fn size() {std::mem::size_of::<Self>() + (self.0.capacity() * std::mem::size_of::<i32>()}
    
    fn tag() {Tag::Array}

#[test]
fn test_memory_profiling_basic() {// Make sure profiling is enabled
    reset_profiling()
    enable_profiling()
    
    // Create a GC
    let gc = GarbageCollector::new()
    
    // Helper struct for testing allocation patterns
    #[derive(Debug, Clone)]
    struct TestObject {data: Vec<u8>,
        value: i32}
    
    impl Traceable for TestObject       {fn trace() {// No references to trace}

unsafe impl Send for TestObject       {}
unsafe impl Sync for TestObject       {}
        
        fn size() {std::mem::size_of::<Self>() + self.data.capacity()}
        
        fn tag() {Tag::Object}
    
    // Allocate some objects
    for i in 0..10   {// Create objects of different sizes;
        let obj = TestObject {;
            data: vec![0; i * 10]
fn test_memory_profiling_patterns() {// Make sure profiling is reset and enabled
    reset_profiling()
    enable_profiling()
    
    // Create a GC
    let gc = GarbageCollector::new()
    
    // Helper functions to create different allocation patterns
    let create_string_pattern = || {for _ in 0..20   {let s = GcString(x .repeat(100);
            let _ = gc.allocate(s).expect(Failed to allocate";}
    let create_vector_pattern = || {for i in 0..15    {let v = GcVecU8(vec![0; i * 1]
fn test_optimize_allocation_pattern() {// Make sure profiling is reset and enabled
    reset_profiling()
    enable_profiling()
    
    let gc = GarbageCollector::new()
    
    // Unoptimized version: creates many small strings
    fn unoptimized_fn() {let mut result = Vec::new()
        
        for i in 0..30   {// This allocates a new string for each iteration}
            let s = GcString(format!(Item   {}, i);
            let gc_s = gc.allocate(s).expect(Failed to allocate)
            if let Some(inner) = gc_s.as_ref()     {result.push(inner.0.clone()}
        
        result}
    
    // Optimized version: pre-allocates capacity and reuses buffer
    fn optimized_fn() {// Pre-allocate the result vector
        let mut result = Vec::with_capacity(30);
        let mut buffer = String::with_capacity(10); // Reuse this buffer
        
        for i in 0..30   {// Clear the buffer instead of allocating a new string
            buffer.clear()
            buffer.push_str(Item)
            buffer.push_str(&i.to_string()
            
            // Clone only when we need to store it
            let gc_s = gc.allocate(GcString(buffer.clone()
            if let Some(inner) = gc_s.as_ref()     {result.push(inner.0.clone()}
        
        result}
    
    // Run the unoptimized version first
    reset_profiling()
    enable_profiling()
    let _ = unoptimized_fn(&gc)
    
    // Check profiling results for unoptimized version
    let profiler = global_profiler()
    let unopt_stats = profiler.get_stats_by_type()
    let gcstring_key = std::any::type_name::<GcString>()
    let unopt_string_count = unopt_stats.get(gcstring_key)
        .map(|s| s.count)
        .unwrap_or(0)
    
    // Run the optimized version
    reset_profiling()
    enable_profiling()
    let _ = optimized_fn(&gc)
    
    // Check profiling results for optimized version
    let profiler = global_profiler()
    let opt_stats = profiler.get_stats_by_type()
    let gcstring_key = std::any::type_name::<GcString>()
    let opt_string_count = opt_stats.get(gcstring_key)
        .map(|s| s.count)
        .unwrap_or(0)
    
    // Optimized version should allocate fewer strings
    // Each has 30 result strings, but optimized reuses the buffer
    assert!(opt_string_count <= unopt_string_count, Optimized version should have fewer allocations (opt:   {}, unopt: {}), ,)
            opt_string_count, unopt_string_count)
    
    // Clean up
    drop(profiler);
    disable_profiling();}