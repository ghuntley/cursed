# Comprehensive GC Testing Strategy

## Testing Philosophy

The garbage collector is a critical component that must be thoroughly tested for correctness, performance, and integration. Our testing strategy covers multiple dimensions:

1. **Correctness**: Memory safety, object lifecycle, reference handling
2. **Performance**: Collection latency, memory efficiency, scalability
3. **Integration**: Goroutine compatibility, LLVM runtime, concurrent access
4. **Reliability**: Stress testing, edge cases, error recovery

## Test Categories

### 1. Unit Tests

#### 1.1 Object Management Tests (`tests/gc_object_test.rs`)
```rust
/// Test ObjectId generation and uniqueness
#[test]
fn test_object_id_uniqueness() {
    let ids: HashSet<ObjectId> = (0..10000)
        .map(|_| ObjectId::new())
        .collect();
    assert_eq!(ids.len(), 10000, "All ObjectIds should be unique");
}

/// Test HeapObject creation and metadata
#[test]
fn test_heap_object_creation() {
    let obj = HeapObject::new(
        TypeId::of::<i32>(),
        ObjectData::Primitive(PrimitiveValue::Integer(42)),
        1024
    );
    
    assert_eq!(obj.header.type_id, TypeId::of::<i32>());
    assert_eq!(obj.header.size, 1024);
    assert_eq!(obj.mark_state.load(Ordering::SeqCst), MarkState::Unmarked);
}

/// Test reference tracking
#[test]
fn test_reference_tracking() {
    let mut obj = HeapObject::new(TypeId::of::<TestStruct>(), test_data(), 100);
    
    let ref1 = ObjectId::new();
    let ref2 = ObjectId::new();
    
    obj.add_reference(ref1);
    obj.add_reference(ref2);
    
    let refs = obj.get_references();
    assert!(refs.contains(&ref1));
    assert!(refs.contains(&ref2));
    assert_eq!(refs.len(), 2);
}
```

#### 1.2 Memory Block Tests (`tests/gc_memory_block_test.rs`)
```rust
/// Test memory block allocation
#[test]
fn test_memory_block_allocation() {
    let mut block = MemoryBlock::new(4096);
    
    // Allocate objects of various sizes
    let alloc1 = block.allocate(64, ObjectId::new()).unwrap();
    let alloc2 = block.allocate(128, ObjectId::new()).unwrap();
    let alloc3 = block.allocate(256, ObjectId::new()).unwrap();
    
    assert_eq!(alloc1.size, 64);
    assert_eq!(alloc2.size, 128);
    assert_eq!(alloc3.size, 256);
    assert_eq!(block.allocated, 64 + 128 + 256);
}

/// Test free space management
#[test]
fn test_free_space_management() {
    let mut block = MemoryBlock::new(1024);
    
    let alloc1 = block.allocate(100, ObjectId::new()).unwrap();
    let alloc2 = block.allocate(200, ObjectId::new()).unwrap();
    
    // Free first allocation
    block.deallocate(alloc1.object_id).unwrap();
    
    // Should be able to allocate in freed space
    let alloc3 = block.allocate(50, ObjectId::new()).unwrap();
    assert_eq!(alloc3.offset, alloc1.offset);
}

/// Test fragmentation handling
#[test]
fn test_fragmentation_handling() {
    let mut block = MemoryBlock::new(1000);
    
    // Create fragmented memory pattern
    let allocs: Vec<_> = (0..10)
        .map(|_| block.allocate(50, ObjectId::new()).unwrap())
        .collect();
    
    // Free every other allocation
    for (i, alloc) in allocs.iter().enumerate() {
        if i % 2 == 0 {
            block.deallocate(alloc.object_id).unwrap();
        }
    }
    
    // Verify fragmentation is tracked
    let free_chunks = block.get_free_chunks();
    assert_eq!(free_chunks.len(), 5); // Should have 5 free chunks
}
```

#### 1.3 Root Manager Tests (`tests/gc_root_manager_test.rs`)
```rust
/// Test global root management
#[test]
fn test_global_root_management() {
    let root_manager = RootManager::new();
    
    let obj1 = ObjectId::new();
    let obj2 = ObjectId::new();
    
    root_manager.add_global_root(obj1);
    root_manager.add_global_root(obj2);
    
    let roots = root_manager.get_global_roots();
    assert!(roots.contains(&obj1));
    assert!(roots.contains(&obj2));
    
    root_manager.remove_global_root(obj1);
    let roots = root_manager.get_global_roots();
    assert!(!roots.contains(&obj1));
    assert!(roots.contains(&obj2));
}

/// Test stack root management
#[test]
fn test_stack_root_management() {
    let root_manager = RootManager::new();
    let thread_id = std::thread::current().id();
    
    let obj1 = ObjectId::new();
    let obj2 = ObjectId::new();
    
    root_manager.add_stack_root(thread_id, obj1);
    root_manager.add_stack_root(thread_id, obj2);
    
    let roots = root_manager.get_stack_roots(thread_id);
    assert_eq!(roots.len(), 2);
    assert!(roots.contains(&obj1));
    assert!(roots.contains(&obj2));
}

/// Test root enumeration
#[test]
fn test_root_enumeration() {
    let root_manager = RootManager::new();
    let thread_id = std::thread::current().id();
    
    let global_root = ObjectId::new();
    let stack_root = ObjectId::new();
    let temp_root = ObjectId::new();
    
    root_manager.add_global_root(global_root);
    root_manager.add_stack_root(thread_id, stack_root);
    root_manager.add_temp_root(temp_root);
    
    let all_roots = root_manager.get_all_roots();
    assert_eq!(all_roots.len(), 3);
    assert!(all_roots.contains(&global_root));
    assert!(all_roots.contains(&stack_root));
    assert!(all_roots.contains(&temp_root));
}
```

### 2. Algorithm Tests

#### 2.1 Marking Phase Tests (`tests/gc_marking_test.rs`)
```rust
/// Test basic marking from roots
#[test]
fn test_basic_marking() {
    let mut marking = MarkingPhase::new();
    let registry = create_test_registry();
    
    // Create object graph: root -> obj1 -> obj2
    let root = create_test_object(&registry);
    let obj1 = create_test_object(&registry);
    let obj2 = create_test_object(&registry);
    
    add_reference(&registry, root, obj1);
    add_reference(&registry, obj1, obj2);
    
    // Mark from root
    let stats = marking.mark_from_roots(&[root]).unwrap();
    
    // All objects should be marked
    assert!(is_marked(&registry, root));
    assert!(is_marked(&registry, obj1));
    assert!(is_marked(&registry, obj2));
    assert_eq!(stats.objects_marked, 3);
}

/// Test cycle detection
#[test]
fn test_cycle_detection() {
    let mut marking = MarkingPhase::new();
    let registry = create_test_registry();
    
    // Create circular reference: obj1 -> obj2 -> obj1
    let obj1 = create_test_object(&registry);
    let obj2 = create_test_object(&registry);
    
    add_reference(&registry, obj1, obj2);
    add_reference(&registry, obj2, obj1);
    
    // Mark from obj1 (should not infinite loop)
    let stats = marking.mark_from_roots(&[obj1]).unwrap();
    
    assert!(is_marked(&registry, obj1));
    assert!(is_marked(&registry, obj2));
    assert_eq!(stats.objects_marked, 2);
}

/// Test unreachable object handling
#[test]
fn test_unreachable_objects() {
    let mut marking = MarkingPhase::new();
    let registry = create_test_registry();
    
    let reachable = create_test_object(&registry);
    let unreachable = create_test_object(&registry);
    
    // Mark from reachable root only
    let stats = marking.mark_from_roots(&[reachable]).unwrap();
    
    assert!(is_marked(&registry, reachable));
    assert!(!is_marked(&registry, unreachable));
    assert_eq!(stats.objects_marked, 1);
}
```

#### 2.2 Sweep Phase Tests (`tests/gc_sweeping_test.rs`)
```rust
/// Test basic sweeping
#[test]
fn test_basic_sweeping() {
    let mut sweeping = SweepingPhase::new();
    let registry = create_test_registry();
    
    let marked_obj = create_test_object(&registry);
    let unmarked_obj = create_test_object(&registry);
    
    // Mark one object
    mark_object(&registry, marked_obj);
    
    // Sweep should free unmarked object
    let stats = sweeping.sweep_heap(&registry).unwrap();
    
    assert!(registry.contains(marked_obj));
    assert!(!registry.contains(unmarked_obj));
    assert_eq!(stats.objects_freed, 1);
}

/// Test memory reclamation
#[test]
fn test_memory_reclamation() {
    let mut sweeping = SweepingPhase::new();
    let registry = create_test_registry();
    
    // Create objects of known sizes
    let obj1 = create_test_object_with_size(&registry, 100);
    let obj2 = create_test_object_with_size(&registry, 200);
    let obj3 = create_test_object_with_size(&registry, 300);
    
    // Mark only obj2
    mark_object(&registry, obj2);
    
    let stats = sweeping.sweep_heap(&registry).unwrap();
    
    assert_eq!(stats.objects_freed, 2);
    assert_eq!(stats.memory_freed, 400); // obj1 + obj3
}

/// Test mark state reset
#[test]
fn test_mark_state_reset() {
    let mut sweeping = SweepingPhase::new();
    let registry = create_test_registry();
    
    let obj = create_test_object(&registry);
    mark_object(&registry, obj);
    
    // After sweep, marked objects should be reset to unmarked
    sweeping.sweep_heap(&registry).unwrap();
    
    assert!(!is_marked(&registry, obj));
    assert!(registry.contains(obj));
}
```

### 3. Integration Tests

#### 3.1 End-to-End Collection Tests (`tests/gc_collection_test.rs`)
```rust
/// Test complete mark-and-sweep cycle
#[test]
fn test_complete_collection_cycle() {
    init_tracing!();
    let collector = MarkSweepCollector::new(GcConfig::default());
    
    // Create object graph with mixed reachability
    let root = collector.allocate_object(TestStruct::new()).unwrap();
    let reachable = collector.allocate_object(TestStruct::new()).unwrap();
    let unreachable = collector.allocate_object(TestStruct::new()).unwrap();
    
    // Make root reference reachable object
    collector.add_reference(root, reachable).unwrap();
    
    // Add root to root set
    collector.root_manager.add_global_root(root);
    
    // Perform collection
    let stats = collector.collect().unwrap();
    
    // Should free unreachable object
    assert_eq!(stats.objects_freed, 1);
    assert!(collector.registry.contains(root));
    assert!(collector.registry.contains(reachable));
    assert!(!collector.registry.contains(unreachable));
}

/// Test collection with complex object graph
#[test]
fn test_complex_object_graph() {
    init_tracing!();
    let collector = MarkSweepCollector::new(GcConfig::default());
    
    // Create complex graph: A -> B -> C, A -> D -> E, F (isolated)
    let a = collector.allocate_object(TestStruct::new()).unwrap();
    let b = collector.allocate_object(TestStruct::new()).unwrap();
    let c = collector.allocate_object(TestStruct::new()).unwrap();
    let d = collector.allocate_object(TestStruct::new()).unwrap();
    let e = collector.allocate_object(TestStruct::new()).unwrap();
    let f = collector.allocate_object(TestStruct::new()).unwrap();
    
    collector.add_reference(a, b).unwrap();
    collector.add_reference(b, c).unwrap();
    collector.add_reference(a, d).unwrap();
    collector.add_reference(d, e).unwrap();
    // f is isolated
    
    collector.root_manager.add_global_root(a);
    
    let stats = collector.collect().unwrap();
    
    // Should keep A, B, C, D, E and free F
    assert_eq!(stats.objects_freed, 1);
    assert_eq!(stats.objects_marked, 5);
}
```

#### 3.2 Goroutine Integration Tests (`tests/gc_goroutine_integration_test.rs`)
```rust
/// Test fallback to core GC when no goroutines active
#[test]
fn test_core_gc_fallback() {
    init_tracing!();
    let mut gc = GarbageCollector::new();
    
    // Should use core collection when no goroutines
    let stats = gc.collect().unwrap();
    assert_eq!(stats.collection_type, CollectionType::MarkSweep);
}

/// Test goroutine-aware collection when goroutines active
#[test]
fn test_goroutine_aware_collection() {
    init_tracing!();
    let mut gc = GarbageCollector::new();
    
    // Enable goroutine awareness
    gc.enable_goroutine_awareness();
    
    // Simulate active goroutines
    gc.simulate_active_goroutines(true);
    
    let stats = gc.collect().unwrap();
    assert_eq!(stats.collection_type, CollectionType::GorutineAware);
}

/// Test seamless transition between collection modes
#[test]
fn test_collection_mode_transition() {
    init_tracing!();
    let mut gc = GarbageCollector::new();
    gc.enable_goroutine_awareness();
    
    // Start with no goroutines (core collection)
    let stats1 = gc.collect().unwrap();
    assert_eq!(stats1.collection_type, CollectionType::MarkSweep);
    
    // Add goroutines (goroutine-aware collection)
    gc.simulate_active_goroutines(true);
    let stats2 = gc.collect().unwrap();
    assert_eq!(stats2.collection_type, CollectionType::GorutineAware);
    
    // Remove goroutines (back to core collection)
    gc.simulate_active_goroutines(false);
    let stats3 = gc.collect().unwrap();
    assert_eq!(stats3.collection_type, CollectionType::MarkSweep);
}
```

### 4. Performance Tests

#### 4.1 Collection Latency Tests (`tests/gc_performance_test.rs`)
```rust
/// Test collection pause time with various heap sizes
#[test]
fn test_collection_pause_time() {
    init_tracing!();
    let heap_sizes = [1_000, 10_000, 100_000];
    
    for &size in &heap_sizes {
        let collector = MarkSweepCollector::new(GcConfig::default());
        
        // Allocate objects
        let objects: Vec<_> = (0..size)
            .map(|_| collector.allocate_object(TestStruct::new()).unwrap())
            .collect();
        
        // Add some roots
        for (i, &obj) in objects.iter().enumerate() {
            if i % 100 == 0 {
                collector.root_manager.add_global_root(obj);
            }
        }
        
        // Measure collection time
        let start = Instant::now();
        let stats = collector.collect().unwrap();
        let duration = start.elapsed();
        
        info!("Heap size: {}, Collection time: {:?}, Objects freed: {}", 
              size, duration, stats.objects_freed);
        
        // Assert reasonable pause time (< 10ms for small heaps)
        if size <= 10_000 {
            assert!(duration < Duration::from_millis(10));
        }
    }
}

/// Test memory efficiency
#[test]
fn test_memory_efficiency() {
    init_tracing!();
    let collector = MarkSweepCollector::new(GcConfig::default());
    
    let initial_memory = get_memory_usage();
    
    // Allocate many objects
    let objects: Vec<_> = (0..10_000)
        .map(|_| collector.allocate_object(TestStruct::new()).unwrap())
        .collect();
    
    let allocated_memory = get_memory_usage();
    
    // Drop references (make objects collectible)
    drop(objects);
    
    // Collect garbage
    collector.collect().unwrap();
    
    let final_memory = get_memory_usage();
    
    // Should reclaim most memory
    let reclaimed = allocated_memory - final_memory;
    let allocated = allocated_memory - initial_memory;
    let efficiency = reclaimed as f64 / allocated as f64;
    
    assert!(efficiency > 0.9, "Should reclaim >90% of allocated memory");
}
```

#### 4.2 Scalability Tests (`tests/gc_scalability_test.rs`)
```rust
/// Test performance scaling with heap size
#[test]
fn test_heap_size_scaling() {
    init_tracing!();
    let sizes = [1_000, 10_000, 100_000, 1_000_000];
    let mut durations = Vec::new();
    
    for &size in &sizes {
        let collector = MarkSweepCollector::new(GcConfig::default());
        
        // Create realistic object graph
        create_realistic_heap(&collector, size);
        
        // Measure collection time
        let start = Instant::now();
        collector.collect().unwrap();
        let duration = start.elapsed();
        
        durations.push(duration);
        info!("Size: {}, Duration: {:?}", size, duration);
    }
    
    // Assert roughly linear scaling
    for i in 1..durations.len() {
        let ratio = durations[i].as_nanos() as f64 / durations[i-1].as_nanos() as f64;
        let size_ratio = sizes[i] as f64 / sizes[i-1] as f64;
        
        // Performance should scale better than quadratically
        assert!(ratio < size_ratio * size_ratio, 
                "Collection time should not scale quadratically");
    }
}

/// Test concurrent allocation performance
#[test]
fn test_concurrent_allocation() {
    init_tracing!();
    let collector = Arc::new(MarkSweepCollector::new(GcConfig::default()));
    let num_threads = 8;
    let allocations_per_thread = 10_000;
    
    let start = Instant::now();
    
    let handles: Vec<_> = (0..num_threads)
        .map(|_| {
            let collector = collector.clone();
            thread::spawn(move || {
                for _ in 0..allocations_per_thread {
                    let _obj = collector.allocate_object(TestStruct::new()).unwrap();
                }
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let duration = start.elapsed();
    let total_allocations = num_threads * allocations_per_thread;
    let rate = total_allocations as f64 / duration.as_secs_f64();
    
    info!("Allocation rate: {:.0} objects/sec", rate);
    assert!(rate > 10_000.0, "Should achieve >10K allocations/sec");
}
```

### 5. Stress Tests

#### 5.1 Memory Pressure Tests (`tests/gc_stress_test.rs`)
```rust
/// Test behavior under extreme memory pressure
#[test]
fn test_extreme_memory_pressure() {
    init_tracing!();
    let collector = MarkSweepCollector::new(GcConfig::default());
    
    // Allocate until memory pressure triggers frequent collections
    let mut objects = Vec::new();
    let mut collection_count = 0;
    
    for i in 0..1_000_000 {
        let obj = collector.allocate_object(TestStruct::new()).unwrap();
        objects.push(obj);
        
        // Occasionally trigger collection
        if i % 10_000 == 0 {
            let stats = collector.collect().unwrap();
            collection_count += 1;
            
            // Verify collector stability
            assert!(stats.objects_freed <= objects.len());
            
            info!("Collection {}: freed {} objects", collection_count, stats.objects_freed);
        }
        
        // Occasionally drop some objects
        if i % 5_000 == 0 && objects.len() > 1000 {
            objects.truncate(objects.len() / 2);
        }
    }
    
    assert!(collection_count > 50, "Should have performed many collections");
}

/// Test long-running stability
#[test]
fn test_long_running_stability() {
    init_tracing!();
    let collector = MarkSweepCollector::new(GcConfig::default());
    
    let test_duration = Duration::from_secs(30);
    let start_time = Instant::now();
    
    let mut total_allocations = 0;
    let mut total_collections = 0;
    
    while start_time.elapsed() < test_duration {
        // Allocation burst
        let objects: Vec<_> = (0..1000)
            .map(|_| {
                total_allocations += 1;
                collector.allocate_object(TestStruct::new()).unwrap()
            })
            .collect();
        
        // Keep some objects as roots
        for (i, &obj) in objects.iter().enumerate() {
            if i % 10 == 0 {
                collector.root_manager.add_global_root(obj);
            }
        }
        
        // Periodic collection
        if total_allocations % 5000 == 0 {
            let stats = collector.collect().unwrap();
            total_collections += 1;
            
            // Verify collector health
            assert!(stats.collection_time < Duration::from_millis(100));
            
            info!("Collection {}: {} objects freed in {:?}", 
                  total_collections, stats.objects_freed, stats.collection_time);
        }
        
        // Clean up some roots
        if total_allocations % 10000 == 0 {
            collector.root_manager.clear_temp_roots();
        }
    }
    
    assert!(total_collections > 10, "Should have performed multiple collections");
    assert!(total_allocations > 100_000, "Should have allocated many objects");
}
```

### 6. Error Handling Tests

#### 6.1 Recovery Tests (`tests/gc_error_recovery_test.rs`)
```rust
/// Test recovery from allocation failures
#[test]
fn test_allocation_failure_recovery() {
    init_tracing!();
    let config = GcConfig {
        max_heap_size: Some(1024), // Very small heap
        ..Default::default()
    };
    let collector = MarkSweepCollector::new(config);
    
    // Fill heap to capacity
    let mut objects = Vec::new();
    loop {
        match collector.allocate_object(TestStruct::new()) {
            Ok(obj) => objects.push(obj),
            Err(GcError::OutOfMemory) => break,
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }
    
    // Drop some objects
    objects.truncate(objects.len() / 2);
    
    // Should be able to collect and allocate again
    let stats = collector.collect().unwrap();
    assert!(stats.objects_freed > 0);
    
    let new_obj = collector.allocate_object(TestStruct::new()).unwrap();
    assert!(collector.registry.contains(new_obj));
}

/// Test handling of corrupted object references
#[test]
fn test_corrupted_reference_handling() {
    init_tracing!();
    let collector = MarkSweepCollector::new(GcConfig::default());
    
    let obj = collector.allocate_object(TestStruct::new()).unwrap();
    
    // Corrupt reference (invalid ObjectId)
    collector.add_reference(obj, ObjectId::null()).unwrap();
    collector.add_reference(obj, ObjectId::new()).unwrap(); // Non-existent
    
    // Collection should handle invalid references gracefully
    let stats = collector.collect().unwrap();
    
    // Should complete without panic
    assert!(stats.collection_time > Duration::ZERO);
}
```

## Test Infrastructure

### Test Utilities (`tests/gc_test_utils.rs`)
```rust
/// Common test object
#[derive(Debug, Clone)]
pub struct TestStruct {
    pub value: i32,
    pub references: Vec<ObjectId>,
}

impl TestStruct {
    pub fn new() -> Self {
        Self {
            value: rand::random(),
            references: Vec::new(),
        }
    }
    
    pub fn with_value(value: i32) -> Self {
        Self {
            value,
            references: Vec::new(),
        }
    }
}

impl Traceable for TestStruct {
    fn trace(&self, visitor: &mut dyn Visitor) {
        for &ref_id in &self.references {
            if let Some(obj) = get_object(ref_id) {
                visitor.visit(obj.as_ref());
            }
        }
    }
}

/// Create realistic object graph for testing
pub fn create_realistic_heap(collector: &MarkSweepCollector, size: usize) {
    let mut objects = Vec::new();
    
    // Create objects with realistic reference patterns
    for i in 0..size {
        let obj = collector.allocate_object(TestStruct::new()).unwrap();
        objects.push(obj);
        
        // Add some references to existing objects
        if i > 0 {
            let refs_count = rand::random::<usize>() % 5;
            for _ in 0..refs_count {
                let target_idx = rand::random::<usize>() % i;
                let target = objects[target_idx];
                collector.add_reference(obj, target).unwrap();
            }
        }
        
        // Make some objects roots
        if i % 100 == 0 {
            collector.root_manager.add_global_root(obj);
        }
    }
}

/// Memory usage measurement
pub fn get_memory_usage() -> usize {
    // Platform-specific memory measurement
    #[cfg(unix)]
    {
        use std::fs;
        let status = fs::read_to_string("/proc/self/status").unwrap();
        for line in status.lines() {
            if line.starts_with("VmRSS:") {
                let kb: usize = line.split_whitespace()
                    .nth(1).unwrap()
                    .parse().unwrap();
                return kb * 1024;
            }
        }
    }
    
    0 // Fallback
}
```

## Continuous Integration

### Test Execution Strategy
```bash
# Quick validation tests (< 5 minutes)
make gc-test-quick

# Complete test suite (< 30 minutes)
make gc-test-all

# Performance regression tests
make gc-test-performance

# Stress tests (extended)
make gc-test-stress

# Coverage analysis
make gc-test-coverage
```

### Performance Monitoring
- **Collection latency**: Track pause times across test runs
- **Memory efficiency**: Monitor overhead and fragmentation
- **Regression detection**: Alert on performance degradation
- **Scalability validation**: Verify linear performance scaling

This comprehensive testing strategy ensures the garbage collector is robust, performant, and ready for production use while maintaining seamless integration with the existing CURSED infrastructure.
