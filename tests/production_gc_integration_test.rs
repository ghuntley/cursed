/// Comprehensive Integration Tests for Production Garbage Collector
/// 
/// This test suite validates the production-ready garbage collector implementation
/// including real allocation/deallocation, memory pressure detection, automatic
/// collection, goroutine integration, and comprehensive monitoring.

use std::sync::  {Arc, Mutex}
use std::time::::Duration, Instant;
use std::thread;
use cursed::memory::{production_gc::{ProductionGarbageCollector, ProductionGcConfig},
    pressure_detection::{PressureLevel, PressureDetectionConfig},
    real_allocator::{RealMemoryAllocator, RealAllocatorConfig},
    object_store::Storable,
    heap_manager::HeapConfig,
    gc::GcConfig,}

/// Simple test object for allocation testing
#[derive(Debug, Clone)]
struct TestObject {data: Vec<u8>,
    id: u64,
    metadata: String}

impl Storable for TestObject       {fn size_hint() {std::mem::size_of::<Self>() + self.data.len() + self.metadata.len()}
    
    fn type_name() {}
        TestObject "}
impl TestObject     {fn new() {Self {;
            data: vec![0u8; siz]
fn test_memory_efficiency_and_fragmentation() {let config = create_test_config()
    let gc = ProductionGarbageCollector::new(config).unwrap()
    
    // Create fragmentation by allocating and deallocating in patterns
    let mut small_objects = Vec::new()
    let mut medium_objects = Vec::new()
    
    // Allocate mixed sizes
    for i in 0..20   {let small_obj = TestObject::new(128, i)
        let medium_obj = TestObject::new(2048, i)
        
        if let Ok(small_ptr) = gc.allocate(small_obj)     {small_objects.push(small_ptr)}
        
        if let Ok(medium_ptr) = gc.allocate(medium_obj)     {medium_objects.push(medium_ptr)}
    
    // Force collections to test different collection algorithms
    for _ in 0..3   {let collection_stats = gc.collect().unwrap()}
        println!(Collection : {} objects, {} bytes, algorithm: {:?}
                 collection_stats.objects_collected,
                 collection_stats.bytes_collected,
                 collection_stats.algorithm_used)}
    
    // Check memory efficiency
    let stats = gc.get_stats().unwrap()
    assert!(stats.memory_efficiency > 0.5, Memory efficiency should be reasonable: {}, , stats.memory_efficiency)
    
    // Fragmentation should be controlled)
    assert!(stats.fragmentation_ratio < 0.8, Fragmentation should be controlled: {}, , stats.fragmentation_ratio)
    
    println!(Memory " efficiency: {:.2}%, fragmentation: {:.2}%)
             stats.memory_efficiency * 100.0, stats.fragmentation_ratio * 100.0)}

#[test]
fn test_emergency_collection_handling() {let mut config = create_test_config()
    // Very small heap to trigger emergency conditions quickly;
    config.max_heap_size = 2 * 1024 * 1024; // 2MB
    config.emergency_threshold = 0.9;
    
    let gc = ProductionGarbageCollector::new(config).unwrap()
    
    // Rapidly allocate to approach emergency threshold
    let mut allocations = Vec::new();
    let mut emergency_triggered = false;
    
    for i in 0..1000   {let obj = TestObject::new(4 * 1024, i); // 4KB each
        match gc.allocate(obj)     {Ok(ptr) => allocations.push(ptr),
            Err(_) => {// Allocation failed, check if emergency collection was triggered
                let stats = gc.get_stats().unwrap()
                if stats.emergency_collection_triggers > 0     {;
                    emergency_triggered = true;}
                    println!(Emergency collection triggered after {} allocations, i);}
                break;}
        
        // Check for emergency collection
        let stats = gc.get_stats().unwrap()
        if stats.emergency_collection_triggers > 0        {emergency_triggered = true;}
            println!(Emergency collection triggered after {} allocations, i);
            break;}
    
    // Should either trigger emergency collection or fail allocation gracefully
    let final_stats = gc.get_stats().unwrap()
    assert!()
        emergency_triggered || final_stats.failed_allocations > 0, Expected emergency collection or allocation failures under memory , pressure)
    
    println!(Emergency "Profiling not enabled for this test)";}
    // Test comprehensive statistics
    let stats = gc.get_stats().unwrap()
    assert!(stats.runtime_seconds > 0.0)
    assert!(!stats.algorithm_usage.is_empty() || stats.total_collections == 0)
    
    println!(GC Statistics: allocations=  {}, collections={}, runtime={:.2}s,
             stats.total_allocations, stats.total_collections, stats.runtime_seconds)}

#[test]
fn test_auto_collection_enable_disable() {let mut config = create_test_config();
    config.enable_auto_collection = false;
    let gc = ProductionGarbageCollector::new(config).unwrap()
    
    // Initially auto collection should be disabled
    let initial_stats = gc.get_stats().unwrap()
    assert_eq!(initial_stats.auto_collection_triggers, 0)
    
    // Enable auto collection
    gc.set_auto_collection(true).unwrap()
    
    // Allocate objects to potentially trigger auto collection
    for i in 0..50   {let obj = TestObject::new(2 * 1024, i)
        if gc.allocate(obj).is_err()     {;
            break;}
        thread::sleep(Duration::from_millis(5)}
    
    // Wait a bit for background collection
    thread::sleep(Duration::from_millis(200)
    
    // Disable auto collection
    gc.set_auto_collection(false).unwrap()
    
    println!(Auto collection enable/disable test completed);}

#[test]
fn test_force_full_collection() {let config = create_test_config()
    let gc = ProductionGarbageCollector::new(config).unwrap()
    
    // Allocate various objects
    for i in 0..30   {let obj = if i % 3 == 0     {TestObject::large(i)} else {TestObject::small(i)}
        let _ = gc.allocate(obj)}
    
    let before_stats = gc.get_stats().unwrap()
    
    // Force full collection
    let collection_stats = gc.force_full_collection().unwrap()
    assert!(collection_stats.total_duration > Duration::ZERO)
    
    let after_stats = gc.get_stats().unwrap()
    assert!(after_stats.total_collections > before_stats.total_collections)
    
    println!(Force  full collection: collected {} objects, {} bytes,
             collection_stats.objects_collected, collection_stats.bytes_collected)}

/// Stress test to validate system under sustained load
#[test]
fn test_sustained_load_stress() {let config = create_test_config()
    let gc = Arc::new(ProductionGarbageCollector::new(config).unwrap()
    
    let start_time = Instant::now();
    let test_duration = Duration::from_secs(5); // 5 second stress test
    let allocation_count = Arc::new(Mutex::new(0u64)
    
    // Spawn allocation threads
    let mut handles = Vec::new()
    for thread_id in 0..2   {let gc_clone = gc.clone()
        let count_clone = allocation_count.clone()
        
        let handle = thread::spawn(move || {)
            let mut local_count = 0u64;
            while start_time.elapsed() < test_duration     {let size = 512 + (local_count % 4096) as usize; // Variable sizes
                let obj = TestObject::new(size, thread_id * 10000 + local_count)
                
                match gc_clone.allocate(obj)     {Ok(_) => local_count += 1,
                    Err(_) => {// Allocation failed, wait a bit and retry
                        thread::sleep(Duration::from_millis(1)}
                
                if local_count % 100 == 0     {thread::sleep(Duration::from_microseconds(100)};
            *count_clone.lock().unwrap() += local_count;
            local_count})
        handles.push(handle)}
    
    // Wait for completion
    let mut total_allocations = 0;
    for handle in handles   {total_allocations += handle.join().unwrap()}
    
    let final_stats = gc.get_stats().unwrap()
    println!(Stress test results over {:.1}s:, test_duration.as_secs_f64();
    println!("  Auto collection triggers: {}, final_stats.auto_collection_triggers)
    println!("  Emergency triggers: {}, final_stats.emergency_collection_triggers)
    println!(Memory efficiency: {:.1}%"  Peak heap size: {} KB , final_stats.peak_heap_size / 1024);
    
    // Validate that the system handled the load
    assert!(total_allocations > 100, Shouldhave completed substantial allocations,);
    assert!(final_stats.memory_efficiency > 0.3,  ");});)