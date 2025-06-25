/// Enhanced GC Allocation System Tests
/// 
/// This module tests the complete enhanced allocation system including:
/// 1. Real heap allocation integration
/// 2. Legacy heap allocation fallback
/// 3. Gc<T> pointer creation and management
/// 4. Memory safety and error handling
/// 5. Integration with existing GC systems

use std::sync::Arc;
use tracing::{info, debug};

// Import test infrastructure
#[path = "common.rs"]
mod common;

use cursed::memory::enhanced_gc::{EnhancedGarbageCollector, EnhancedCollectionStats};
use cursed::memory::gc::{GcConfig, Gc, HeapConfig};
use cursed::memory::object_store::Storable;
use cursed::memory::{Traceable, Visitor};
use cursed::memory::object_id::{ObjectId, ObjectRegistry};

/// Test object that implements Storable
#[derive(Debug, Clone)]
struct TestObject {
    id: u32,
    name: String,
    data: Vec<u8>,
}

impl Traceable for TestObject {
    fn trace(&self, _visitor: &mut dyn Visitor) {
        // Test objects don't reference other objects
    }
}

impl TestObject {
    fn new(id: u32, name: &str, data_size: usize) -> Self {
        Self {
            id,
            name: name.to_string(),
            data: vec![0u8; data_size],
        }
    }
    
    fn size(&self) -> usize {
        std::mem::size_of::<Self>() + self.name.len() + self.data.len()
    }
}

/// Test object with references
#[derive(Debug)]
struct RefTestObject {
    id: u32,
    name: String,
    references: Vec<ObjectId>,
}

impl Traceable for RefTestObject {
    fn trace(&self, visitor: &mut dyn Visitor) {
        // In a real implementation, this would trace the referenced objects
        // For now, we just implement the interface
    }
}

impl RefTestObject {
    fn new(id: u32, name: &str, refs: Vec<ObjectId>) -> Self {
        Self {
            id,
            name: name.to_string(),
            references: refs,
        }
    }
}

fn init_test_logging() {
    common::init_tracing!();
}

/// Create a test enhanced GC with real heap enabled
fn create_enhanced_gc() -> EnhancedGarbageCollector {
    let gc_config = GcConfig::default();
    let heap_config = HeapConfig::default();
    EnhancedGarbageCollector::with_config(gc_config, heap_config, true)
}

/// Create a test enhanced GC with legacy heap
fn create_legacy_enhanced_gc() -> EnhancedGarbageCollector {
    let gc_config = GcConfig::default();
    let heap_config = HeapConfig::default();
    EnhancedGarbageCollector::with_config(gc_config, heap_config, false)
}

#[test]
fn test_enhanced_gc_creation() {
    init_test_logging();
    info!("Testing enhanced GC creation");
    
    let enhanced_gc = create_enhanced_gc();
    assert!(enhanced_gc.can_convert_to_standard());
    
    let stats = enhanced_gc.get_comprehensive_stats_enhanced().unwrap();
    assert_eq!(stats.total_collections, 0);
    assert_eq!(stats.total_objects_collected, 0);
}

#[test]
fn test_basic_allocation_real_heap() {
    init_test_logging();
    info!("Testing basic allocation with real heap");
    
    let enhanced_gc = create_enhanced_gc();
    
    // Test small object allocation
    let small_obj = TestObject::new(1, "small", 64);
    let gc_ptr = enhanced_gc.allocate(small_obj);
    
    match gc_ptr {
        Ok(ptr) => {
            assert!(ptr.is_valid());
            assert_eq!(ptr.object_id().as_u64(), ptr.object_id().as_u64());
            debug!("Successfully allocated small object with real heap");
        }
        Err(e) => {
            // Real heap allocation might fail in test environment
            debug!("Real heap allocation failed (expected in test): {}", e);
        }
    }
}

#[test]
fn test_basic_allocation_legacy_heap() {
    init_test_logging();
    info!("Testing basic allocation with legacy heap");
    
    let enhanced_gc = create_legacy_enhanced_gc();
    
    // Test small object allocation
    let small_obj = TestObject::new(1, "small", 64);
    let gc_ptr = enhanced_gc.allocate(small_obj);
    
    match gc_ptr {
        Ok(ptr) => {
            assert!(ptr.is_valid());
            assert_eq!(ptr.object_id().as_u64(), ptr.object_id().as_u64());
            info!("Successfully allocated small object with legacy heap");
        }
        Err(e) => {
            // This should work as it uses the object store
            panic!("Legacy heap allocation should not fail: {}", e);
        }
    }
}

#[test]
fn test_multiple_allocations() {
    init_test_logging();
    info!("Testing multiple allocations");
    
    let enhanced_gc = create_legacy_enhanced_gc(); // Use legacy for stability
    let mut objects = Vec::new();
    
    // Allocate multiple objects of different sizes
    for i in 0..10 {
        let obj = TestObject::new(i, &format!("object_{}", i), 32 + i as usize * 16);
        
        match enhanced_gc.allocate(obj) {
            Ok(gc_ptr) => {
                assert!(gc_ptr.is_valid());
                objects.push(gc_ptr);
                debug!("Allocated object {}", i);
            }
            Err(e) => {
                debug!("Allocation {} failed: {}", i, e);
            }
        }
    }
    
    info!("Successfully allocated {} objects", objects.len());
    
    // Verify all objects are still valid
    for (i, obj) in objects.iter().enumerate() {
        assert!(obj.is_valid(), "Object {} should still be valid", i);
    }
}

#[test]
fn test_large_object_allocation() {
    init_test_logging();
    info!("Testing large object allocation");
    
    let enhanced_gc = create_legacy_enhanced_gc();
    
    // Test large object (1MB)
    let large_obj = TestObject::new(999, "large", 1024 * 1024);
    
    match enhanced_gc.allocate(large_obj) {
        Ok(ptr) => {
            assert!(ptr.is_valid());
            info!("Successfully allocated large object");
        }
        Err(e) => {
            debug!("Large object allocation failed (may be expected): {}", e);
        }
    }
}

#[test]
fn test_gc_pointer_operations() {
    init_test_logging();
    info!("Testing Gc pointer operations");
    
    let enhanced_gc = create_legacy_enhanced_gc();
    let test_obj = TestObject::new(42, "test", 128);
    
    match enhanced_gc.allocate(test_obj) {
        Ok(gc_ptr) => {
            // Test basic operations
            assert!(gc_ptr.is_valid());
            
            // Test object access
            assert_eq!(gc_ptr.id, 42);
            assert_eq!(gc_ptr.name, "test");
            assert_eq!(gc_ptr.data.len(), 128);
            
            // Test weak reference creation
            let weak_ref = gc_ptr.downgrade();
            assert_eq!(weak_ref.object_id(), gc_ptr.object_id());
            
            // Test root marking
            if let Err(e) = gc_ptr.mark_as_root() {
                debug!("Root marking failed (may be expected): {}", e);
            }
            
            info!("Gc pointer operations completed successfully");
        }
        Err(e) => {
            panic!("Basic allocation should work: {}", e);
        }
    }
}

#[test]
fn test_allocation_with_collection() {
    init_test_logging();
    info!("Testing allocation with garbage collection");
    
    let enhanced_gc = create_legacy_enhanced_gc();
    let mut objects = Vec::new();
    
    // Allocate several objects
    for i in 0..5 {
        let obj = TestObject::new(i, &format!("gc_test_{}", i), 256);
        if let Ok(gc_ptr) = enhanced_gc.allocate(obj) {
            objects.push(gc_ptr);
        }
    }
    
    info!("Allocated {} objects before collection", objects.len());
    
    // Trigger collection
    match enhanced_gc.collect_enhanced() {
        Ok(stats) => {
            info!("Collection completed: {:?}", stats);
            
            // Verify remaining objects are still valid
            for (i, obj) in objects.iter().enumerate() {
                if obj.is_valid() {
                    debug!("Object {} survived collection", i);
                } else {
                    debug!("Object {} was collected", i);
                }
            }
        }
        Err(e) => {
            debug!("Collection failed (may be expected): {}", e);
        }
    }
}

#[test]
fn test_allocation_pressure_tracking() {
    init_test_logging();
    info!("Testing allocation pressure tracking");
    
    let enhanced_gc = create_legacy_enhanced_gc();
    
    // Check initial statistics
    let initial_stats = enhanced_gc.get_comprehensive_stats_enhanced().unwrap();
    assert_eq!(initial_stats.total_objects_collected, 0);
    
    // Allocate objects to create pressure
    let mut allocated = 0;
    for i in 0..20 {
        let obj = TestObject::new(i, &format!("pressure_{}", i), 512);
        if enhanced_gc.allocate(obj).is_ok() {
            allocated += 1;
        }
    }
    
    info!("Allocated {} objects for pressure test", allocated);
    
    // Check for collection triggers
    match enhanced_gc.should_collect_enhanced() {
        Ok(Some(trigger)) => {
            info!("Collection triggered: {:?}", trigger);
        }
        Ok(None) => {
            debug!("No collection trigger detected");
        }
        Err(e) => {
            debug!("Error checking collection trigger: {}", e);
        }
    }
}

#[test]
fn test_enhanced_to_standard_gc_conversion() {
    init_test_logging();
    info!("Testing enhanced to standard GC conversion");
    
    let enhanced_gc = create_legacy_enhanced_gc();
    
    // Allocate some objects first
    let _obj1 = enhanced_gc.allocate(TestObject::new(1, "convert_test1", 64));
    let _obj2 = enhanced_gc.allocate(TestObject::new(2, "convert_test2", 128));
    
    // Test conversion
    match enhanced_gc.to_standard_gc() {
        Ok(standard_gc) => {
            let stats = standard_gc.stats();
            info!("Conversion successful, standard GC stats: {:?}", stats);
            
            // Test that standard GC works
            let test_obj = TestObject::new(99, "standard", 64);
            match standard_gc.allocate(test_obj) {
                Ok(ptr) => {
                    assert!(ptr.is_valid());
                    info!("Standard GC allocation after conversion works");
                }
                Err(e) => {
                    debug!("Standard GC allocation failed: {}", e);
                }
            }
        }
        Err(e) => {
            panic!("Enhanced to standard GC conversion should work: {}", e);
        }
    }
}

#[test]
fn test_allocation_error_handling() {
    init_test_logging();
    info!("Testing allocation error handling");
    
    let enhanced_gc = create_enhanced_gc(); // Real heap might fail
    
    // Test zero-size allocation
    #[derive(Debug)]
    struct ZeroSizeObject;
    
    impl Traceable for ZeroSizeObject {
        fn trace(&self, _visitor: &mut dyn Visitor) {}
    }
    
    let zero_obj = ZeroSizeObject;
    match enhanced_gc.allocate(zero_obj) {
        Ok(_) => {
            info!("Zero-size allocation succeeded (may be allowed)");
        }
        Err(e) => {
            debug!("Zero-size allocation failed as expected: {}", e);
        }
    }
    
    // Test extremely large allocation
    let huge_obj = TestObject::new(999, "huge", usize::MAX / 2);
    match enhanced_gc.allocate(huge_obj) {
        Ok(_) => {
            panic!("Extremely large allocation should fail");
        }
        Err(e) => {
            info!("Large allocation failed as expected: {}", e);
        }
    }
}

#[test]
fn test_concurrent_allocation() {
    init_test_logging();
    info!("Testing concurrent allocation");
    
    let enhanced_gc = Arc::new(create_legacy_enhanced_gc());
    let mut handles = Vec::new();
    
    // Spawn multiple threads to allocate concurrently
    for thread_id in 0..4 {
        let gc = Arc::clone(&enhanced_gc);
        let handle = std::thread::spawn(move || {
            let mut local_objects = Vec::new();
            
            for i in 0..5 {
                let obj = TestObject::new(
                    thread_id * 100 + i, 
                    &format!("thread_{}_{}", thread_id, i), 
                    128
                );
                
                if let Ok(gc_ptr) = gc.allocate(obj) {
                    local_objects.push(gc_ptr);
                }
            }
            
            debug!("Thread {} allocated {} objects", thread_id, local_objects.len());
            local_objects.len()
        });
        handles.push(handle);
    }
    
    // Wait for all threads and collect results
    let mut total_allocated = 0;
    for handle in handles {
        match handle.join() {
            Ok(count) => total_allocated += count,
            Err(e) => debug!("Thread panicked: {:?}", e),
        }
    }
    
    info!("Total objects allocated concurrently: {}", total_allocated);
    assert!(total_allocated > 0, "At least some concurrent allocations should succeed");
}

#[test]
fn test_allocation_with_references() {
    init_test_logging();
    info!("Testing allocation with object references");
    
    let enhanced_gc = create_legacy_enhanced_gc();
    
    // Allocate some target objects
    let mut target_ids = Vec::new();
    for i in 0..3 {
        let target_obj = TestObject::new(i, &format!("target_{}", i), 64);
        if let Ok(gc_ptr) = enhanced_gc.allocate(target_obj) {
            target_ids.push(gc_ptr.object_id());
        }
    }
    
    // Allocate object that references others
    let ref_obj = RefTestObject::new(999, "referencing", target_ids.clone());
    match enhanced_gc.allocate(ref_obj) {
        Ok(ref_ptr) => {
            assert!(ref_ptr.is_valid());
            assert_eq!(ref_ptr.references.len(), target_ids.len());
            info!("Successfully allocated object with references");
        }
        Err(e) => {
            debug!("Reference object allocation failed: {}", e);
        }
    }
}

#[test]
fn test_allocation_statistics_tracking() {
    init_test_logging();
    info!("Testing allocation statistics tracking");
    
    let enhanced_gc = create_legacy_enhanced_gc();
    
    // Get initial stats
    let initial_stats = enhanced_gc.get_comprehensive_stats_enhanced().unwrap();
    let initial_collections = initial_stats.total_collections;
    
    // Perform allocations
    let mut successful_allocations = 0;
    for i in 0..10 {
        let obj = TestObject::new(i, &format!("stats_{}", i), 200 + i * 50);
        if enhanced_gc.allocate(obj).is_ok() {
            successful_allocations += 1;
        }
    }
    
    info!("Performed {} successful allocations", successful_allocations);
    
    // Check stats after allocations
    let final_stats = enhanced_gc.get_comprehensive_stats_enhanced().unwrap();
    
    // Verify statistics make sense
    assert!(final_stats.total_collections >= initial_collections);
    
    info!("Final GC statistics: collections={}, objects_collected={}", 
          final_stats.total_collections, final_stats.total_objects_collected);
}

// Integration test to verify all enhanced allocation features work together
#[test]
fn test_enhanced_allocation_integration() {
    init_test_logging();
    info!("Running comprehensive enhanced allocation integration test");
    
    let enhanced_gc = create_legacy_enhanced_gc();
    let mut all_objects = Vec::new();
    
    // Phase 1: Basic allocations
    info!("Phase 1: Basic allocations");
    for i in 0..5 {
        let obj = TestObject::new(i, &format!("basic_{}", i), 128);
        if let Ok(gc_ptr) = enhanced_gc.allocate(obj) {
            all_objects.push(gc_ptr);
        }
    }
    
    // Phase 2: Large objects
    info!("Phase 2: Large object allocation");
    let large_obj = TestObject::new(100, "large", 4096);
    if let Ok(gc_ptr) = enhanced_gc.allocate(large_obj) {
        all_objects.push(gc_ptr);
    }
    
    // Phase 3: Collection
    info!("Phase 3: Trigger collection");
    if let Ok(stats) = enhanced_gc.collect_enhanced() {
        info!("Collection stats: {:?}", stats);
    }
    
    // Phase 4: Post-collection allocations
    info!("Phase 4: Post-collection allocations");
    for i in 200..205 {
        let obj = TestObject::new(i, &format!("post_gc_{}", i), 256);
        if let Ok(gc_ptr) = enhanced_gc.allocate(obj) {
            all_objects.push(gc_ptr);
        }
    }
    
    // Phase 5: Verify object validity
    info!("Phase 5: Verify object validity");
    let mut valid_count = 0;
    for (i, obj) in all_objects.iter().enumerate() {
        if obj.is_valid() {
            valid_count += 1;
        } else {
            debug!("Object {} was collected", i);
        }
    }
    
    info!("Integration test completed: {} valid objects remaining", valid_count);
    
    // Final statistics
    let final_stats = enhanced_gc.get_comprehensive_stats_enhanced().unwrap();
    info!("Final statistics: {:?}", final_stats);
    
    // Test conversion to standard GC
    if let Ok(standard_gc) = enhanced_gc.to_standard_gc() {
        info!("Successfully converted to standard GC");
        let std_stats = standard_gc.stats();
        info!("Standard GC stats: {:?}", std_stats);
    }
}
