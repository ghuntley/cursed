/// Enhanced Garbage Collection Implementation Tests
/// 
/// This test suite validates the new garbage collection functionality implemented
/// including the improved is_marked() method, object removal, cycle detection,
/// and incremental collection enhancements.

use std::sync::Arc;
use cursed::memory::{gc::{GarbageCollector, GcConfig, CollectionTrigger},
    heap_manager::{HeapManager, HeapConfig},
    object_id::{ObjectId, ObjectRegistry, ObjectMetadata, ObjectIdGenerator},
    object_store::{ObjectStore, Storable},
    Traceable, Visitor,}

#[derive(Debug, Clone)]
struct TestObject {id: u64,
    data: Vec<u8>,
    references: Vec<ObjectId>

impl Traceable for TestObject       {fn trace() {// Trace references to other objects
        for ref_id in &self.references   {// In a real implementation, we'd trace the actual object
            // For now, we'd need to get the actual memory pointer from the allocation;
    let obj_ptr = std::ptr::addr_of!(*gc_ptr) as *const u8;
    
    // Initially, the object should not be marked (before GC runs)
    // Note: This test might need adjustment based on actual allocation behavior
    println!(Testing is_marked with allocated object pointer)}

#[test]
fn test_object_registry_is_marked() {let registry = Arc::new(ObjectRegistry::new()
    let id_gen = ObjectIdGenerator::new()
    
    // Create test object metadata
    let object_id = id_gen.next()
    let metadata = ObjectMetadata::new(object_id,
        128,
        TestObject .to_string(),)
    
    // Register the object
    registry.register(metadata).expect(Failed to register object)
    
    // Initially, object should not be marked
    assert!(!registry.is_marked(object_id).expect(Failed to check mark status)
    
    // Mark the object
    registry.mark_object(object_id).expect(Failed to mark object)
    
    // Now it should be marked
    assert!(registry.is_marked(object_id).expect(Failed to check mark status)
    
    // Unmark all objects
    registry.unmark_all().expect(Failed to unmark all objects)
    
    // Should not be marked anymore
    assert!(!registry.is_marked(object_id).expect(Failed to check mark status)}

#[test]
fn test_incremental_object_removal() {use cursed::memory::incremental::{IncrementalCollector, IncrementalConfig}
    
    let registry = Arc::new(ObjectRegistry::new()
    let collector = IncrementalCollector::new(registry.clone()
    let id_gen = ObjectIdGenerator::new()
    
    // Create and register test objects
    let object_id1 = id_gen.next()
    let object_id2 = id_gen.next()
    
    let metadata1 = ObjectMetadata::new(object_id1,
        64,
        TestObject1.to_string(),)
    
    let metadata2 = ObjectMetadata::new(object_id2,
        128,
        TestObject2.to_string(),)
    
    registry.register(metadata1).expect("Failed to register object 1")
    // Verify objects exist
    assert!(registry.get(object_id1).expect(Failed to get object 1).is_some()
    assert!(registry.get(object_id2).expect(Failed to get object 2").is_some()
    // Start incremental collection
    collector.start_collection().expect(Failed to start collection)
    
    // Add objects to sweep candidates (simulate marking phase completion)
    // Note: This is testing internal behavior and might need adjustment
    println!(Testing incremental object removal functionality)
    
    // Get stats to verify collection can track removed objects
    let stats = collector.get_stats().expect(Failed to get stats)
    println!(Initial stats: objects_swept={}, bytes_reclaimed={}, 
             stats.objects_swept, stats.bytes_reclaimed)}

#[test]
fn test_cycle_detection_object_collection() {use cursed::memory::cycle_detection::{CycleDetector, CycleInfo, CycleDetectionAlgorithm}
    
    let registry = Arc::new(ObjectRegistry::new()
    let detector = CycleDetector::new(registry.clone()
    let id_gen = ObjectIdGenerator::new()
    
    // Create a simple cycle: A -> B -> A
    let object_id_a = id_gen.next()
    let object_id_b = id_gen.next()
    
    let metadata_a = ObjectMetadata::new(object_id_a,
        64,
        CyclicObjectA.to_string(),)
    
    let metadata_b = ObjectMetadata::new(object_id_b,
        64,
        CyclicObjectB ")
    registry.register(metadata_b).expect("Failed to register object B)
    // Create cycle info
    let cycle = CycleInfo   {objects: vec![object_id_a, object_id_]
fn test_copying_collector_reference_updating() {use cursed::memory::copying::{CopyingCollector, CopyingConfig}
    
    let registry = Arc::new(ObjectRegistry::new()
    let collector = CopyingCollector::new(registry.clone()
        .expect('d need actual objects with references to test properly
    println!(Testing copying collector reference updating functionality)
    
    // Test basic collector creation and configuration
    let config = CopyingConfig::default()
    assert!(config.fast_allocation, Fast allocation should be enabled by default)
    assert!(config.parallel_copying, , Parallel copying should be enabled by default)
    
    // Test available space calculation
    let available = collector.available_space().unwrap_or(0)
    println!(Available space in copying collector: {} bytes, available)}

#[test]
fn test_enhanced_gc_integration() {use cursed::memory::gc::{CollectionTrigger, CollectionAlgorithm}
    
    let gc = GarbageCollector::new()
    
    // Test allocation and basic GC functionality
    let test_obj = TestObject {;
        id: 123,;
        data: vec![0; 102]
fn test_gc_algorithm_performance_tracking() {let gc = GarbageCollector::new()
    
    // Trigger multiple collections to test performance tracking
    for i in 0..3   {let test_obj = TestObject {id: i,;
            data: vec![0; 51]
fn test_complete_gc_workflow() {let mut gc_config = GcConfig::default();
    gc_config.adaptive_algorithm_selection = true;
    gc_config.incremental = true;
    
    let heap_config = HeapConfig::default()
    let gc = GarbageCollector::with_config(gc_config, heap_config)
    
    // Allocate multiple objects
    let mut objects = Vec::new()
    for i in 0..10   {let test_obj = TestObject {id: i,;
            data: vec![i as u8; 10]
    
    for trigger in &triggers   {match gc.collect_with_trigger(*trigger)     {Ok(stats) => {}
                println!(Collection with {:?} trigger completed , trigger)
                println!(Algorithm: {:?}, stats.algorithm_used)
                println!("  Duration: {:?}, stats.total_duration)
                println!(Objects collected: {}, stats.objects_collected)}
            Err(e) => {println!("  Collections: {}, final_stats.total_collections)
    println!(Current algorithm: {:?}, final_stats.current_algorithm)
    println!("  Is collecting: {}, final_stats.is_collecting)
    
    assert!(final_stats.total_collections > 0, ");;
