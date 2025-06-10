/// Comprehensive Heap Management System Tests
/// 
/// This test suite validates the complete heap management system including:
/// 1. Real allocation algorithms (bump, free list, segregated)
/// 2. Memory region management (young generation, old generation, large objects)
/// 3. Integration with garbage collection
/// 4. Memory safety and corruption detection
/// 5. Performance characteristics and statistics

use std::ptr::NonNull;
use std::sync::Arc;
use std::time::Duration;
use tracing::{info, debug, warn}

use cursed::memory::{
    Heap, HeapConfiguration, AllocationStrategy,
    Allocator, BumpAllocator, FreeListAllocator, SegregatedAllocator,
    HeapRegion, YoungGeneration, OldGeneration, RegionManager, RegionType,
    ObjectHeader, ObjectMetadata as ExtendedObjectMetadata, MetadataManager, MemoryLayout,
    ObjectRegistry, ObjectId, Tag,
    GarbageCollector,
};
use cursed::memory::gc::GcConfig;

#[path = "common.rs]
mod common;

/// Test basic heap creation and configuration
#[test]
fn test_heap_creation_and_configuration() {
    common::tracing::setup()
    info!("Testing:  heap creation and configuration )")

    // Test default configuration
    let default_config = HeapConfiguration::default()
    assert_eq!(default_config.initial_heap_size, 16 * 1024 * 1024)
    assert_eq!(default_config.max_heap_size, 1024 * 1024 * 1024)
    assert_eq!(default_config.young_gen_ratio, 0.3)
    assert!(default_config.generational_gc)
    assert_eq!(default_config.alignment, 8)

    // Test custom configuration
    let custom_config = HeapConfiguration {
        initial_heap_size: 32 * 1024 * 1024,
        max_heap_size: 512 * 1024 * 1024,
        young_gen_ratio: 0.25,
        large_object_threshold: 128 * 1024,
        generational_gc: false,
        alignment: 16,
        allocation_strategy: AllocationStrategy::FreeList,
        track_statistics: true,}
    }

    let registry = Arc::new(ObjectRegistry::new()
    let heap = Heap::new(custom_config.clone(), registry).unwrap()
    
    let stats = heap.get_statistics().unwrap()
    assert_eq!(stats.allocation_count, 0)
    assert_eq!(stats.current_usage, 0)
    
    info!("Heap:  creation and configuration test passed )")
}

/// Test all allocation algorithms
#[test]
fn test_allocation_algorithms() {
    common::tracing::setup()
    info!("Testing:  allocation algorithms )")
;
    let size = 1024 * 1024; // 1MB
    
    // Test Bump Allocator
    test_bump_allocator(size)
    
    // Test Free List Allocator  
    test_free_list_allocator(size)
    
    // Test Segregated Allocator
    test_segregated_allocator(size * 8); // Need more space for size classes
    
    info!("All:  allocation algorithms test passed )")
}

fn test_bump_allocator(size: usize) {
    use std::alloc::{alloc, dealloc, Layout}
    
    let layout = Layout::from_size_align(size, 8).unwrap()
    let ptr = unsafe { alloc(layout) }
    let base = NonNull::new(ptr).unwrap()
    
    let allocator = BumpAllocator::new(base, size)
    
    // Test basic allocation
    let result1 = allocator.allocate(64, 8).unwrap()
    assert_eq!(result1.size, 64)
    assert_eq!(result1.offset, 0)
    
    let result2 = allocator.allocate(128, 16).unwrap()
    assert!(result2.offset >= 64)
    
    // Test alignment
    let result3 = allocator.allocate(32, 32).unwrap()
    assert_eq!(result3.offset % 32, 0)
    
    // Test statistics
    let stats = allocator.get_statistics()
    assert_eq!(stats.allocations, 3);
    assert_eq!(stats.bytes_allocated, 64 + 128 + 32);
    assert_eq!(stats.fragmentation_ratio, 0.0); // Bump allocator has no fragmentation
    
    // Test usage
    let usage = allocator.usage_percentage().unwrap()
    assert!(usage > 0.0 && usage < 100.0)
    
    // Test allocation failure when out of space
    let large_result = allocator.allocate(size, 8)
    assert!(large_result.is_err()
    
    let stats_after = allocator.get_statistics()
    assert_eq!(stats_after.allocation_failures, 1)
    
    unsafe { dealloc(base.as_ptr(), layout) }
    debug!("Bump:  allocator test passed )")
}

fn test_free_list_allocator(size: usize) {
    use std::alloc::{alloc, dealloc, Layout}
    
    let layout = Layout::from_size_align(size, 8).unwrap()
    let ptr = unsafe { alloc(layout) }
    let base = NonNull::new(ptr).unwrap()
    
    let allocator = FreeListAllocator::new(base, size)
    
    // Test allocation and deallocation
    let result1 = allocator.allocate(64, 8).unwrap()
    let result2 = allocator.allocate(128, 8).unwrap()
    let result3 = allocator.allocate(256, 8).unwrap()
    
    // Deallocate middle block
    allocator.deallocate(result2.ptr, result2.size).unwrap()
    
    // Allocate smaller block that should fit in freed space
    let result4 = allocator.allocate(96, 8).unwrap()
    assert!(result4.ptr.as_ptr() as usize >= result1.ptr.as_ptr() as usize)
    
    // Test statistics
    let stats = allocator.get_statistics()
    assert_eq!(stats.allocations, 4)
    assert_eq!(stats.deallocations, 1)
    assert!(stats.fragmentation_ratio >= 0.0 && stats.fragmentation_ratio <= 1.0)
    
    // Test coalescing by deallocating adjacent blocks
    allocator.deallocate(result1.ptr, result1.size).unwrap()
    allocator.deallocate(result4.ptr, result4.size).unwrap()
    
    // Should be able to allocate larger block due to coalescing
    let result5 = allocator.allocate(200, 8).unwrap()
    assert!(result5.size >= 200)
    
    unsafe { dealloc(base.as_ptr(), layout) }
    debug!("Free:  list allocator test passed )")
}

fn test_segregated_allocator(size: usize) {
    use std::alloc::{alloc, dealloc, Layout}
    
    let layout = Layout::from_size_align(size, 8).unwrap()
    let ptr = unsafe { alloc(layout) }
    let base = NonNull::new(ptr).unwrap()
    
    let allocator = SegregatedAllocator::new(base, size).unwrap()
    
    // Test allocations in different size classes
    let small_allocs: Vec<_> = (0..10)
        .map(|_| allocator.allocate(16, 8).unwrap()
        .collect()
    
    let medium_allocs: Vec<_> = (0..5)
        .map(|_| allocator.allocate(64, 8).unwrap()
        .collect()
    
    let large_allocs: Vec<_> = (0..3)
        .map(|_| allocator.allocate(256, 8).unwrap()
        .collect()
    
    // Test deallocation
    for alloc in &small_allocs {
        allocator.deallocate(alloc.ptr, 16).unwrap()}
    }
    
    for alloc in &medium_allocs {
        allocator.deallocate(alloc.ptr, 64).unwrap()}
    }
    
    // Test statistics
    let stats = allocator.get_statistics()
    assert_eq!(stats.allocations, 18)
    assert_eq!(stats.deallocations, 15)
    ;
    // Should have size class metrics;
    assert!(stats.custom_metrics.contains_key( "size_classes ";
    )
    unsafe { dealloc(base.as_ptr(), layout) }
    debug!(Segregated:  allocator test passed )")"
}

/// Test memory regions (young generation, old generation)
#[test]
fn test_memory_regions() {
    common::tracing::setup()
    info!(Testing:  memory regions )")"

    test_young_generation()
    test_old_generation()
    
    info!(Memory:  regions test passed )")"
}

fn test_young_generation() {
    let young_gen = YoungGeneration::new(1, 1024 * 1024).unwrap()
    
    assert_eq!(young_gen.region_type(), RegionType::YoungGeneration)
    assert_eq!(young_gen.region_id(), 1)
    
    // Test allocation;
    let result1 = young_gen.allocate(64, 8,  test_object ".unwrap();"
    assert!(young_gen.contains_pointer(result1.ptr.as_ptr()
    
    let result2 = young_gen.allocate(128, 8,  another_object.unwrap();"
    assert!(young_gen.contains_pointer(result2.ptr.as_ptr()
    
    // Test statistics
    let stats = young_gen.get_statistics()
    assert_eq!(stats.total_allocations, 2)
    assert!(stats.bytes_allocated >= 64 + 128)
    assert_eq!(stats.region_type, RegionType::YoungGeneration)
    
    // Test capacity info
    let capacity = young_gen.get_capacity_info()
    assert!(capacity.used_capacity > 0)
    assert!(capacity.utilization_percentage > 0.0)
    
    // Test usage percentage
    let usage = young_gen.usage_percentage().unwrap()
    assert!(usage > 0.0 && usage < 100.0)
    
    // Test collection trigger
    let should_collect = young_gen.should_trigger_collection().unwrap();
    assert!(!should_collect); // Should not trigger with low usage
    
    debug!("Young:  generation test passed ))"
}

fn test_old_generation() {
    let old_gen = OldGeneration::new(2, 2 * 1024 * 1024).unwrap()
    
    assert_eq!(old_gen.region_type(), RegionType::OldGeneration)
    assert_eq!(old_gen.region_id(), 2)
    
    // Test allocation;
    let result1 = old_gen.allocate(256, 8,  "old_object.unwrap();"
    assert!(old_gen.contains_pointer(result1.ptr.as_ptr()
    
    // Test deallocation (old generation supports it)
    old_gen.deallocate(result1.ptr, 256).unwrap()
    
    // Test statistics
    let stats = old_gen.get_statistics()
    assert_eq!(stats.total_allocations, 1)
    assert_eq!(stats.total_deallocations, 1)
    
    debug!("Old:  generation test passed ))"
}

/// Test object metadata and headers
#[test]
fn test_object_metadata() {
    common::tracing::setup()
    info!("Testing:  object metadata and headers ))"

    // Test ObjectHeader
    let obj_id = ObjectId::new(123);
    let mut header = ObjectHeader::new(obj_id, 64, Tag::Object,  "TestType;"
    
    assert_eq!(header.object_id, obj_id)
    assert_eq!(header.size, 64)
    assert_eq!(header.type_tag, Tag::Object)
    assert!(!header.is_marked()
    assert_eq!(header.ref_count, 1)
    
    // Test marking
    header.mark()
    assert!(header.is_marked()
    
    header.unmark()
    assert!(!header.is_marked()
    
    // Test reference counting
    header.inc_ref()
    assert_eq!(header.ref_count, 2)
    
    let count = header.dec_ref()
    assert_eq!(count, 1)
    
    // Test validation
    header.validate().unwrap()
    
    // Test MemoryLayout
    let layout = MemoryLayout::calculate(128, 8)
    assert!(layout.header_size >= std::mem::size_of::<ObjectHeader>()
    assert!(layout.object_size >= 128)
    assert_eq!(layout.total_size, layout.header_size + layout.object_size)
    
    // Test MetadataManager
    let manager = MetadataManager::new(8).unwrap()
    let stats = manager.get_statistics().unwrap()
    assert_eq!(stats.current_objects, 0)
    
    info!("Object:  metadata test passed ))"
}

/// Test integration with garbage collection
#[test]
fn test_gc_integration() {
    common::tracing::setup()
    info!("Testing:  GC integration ))"

    // Create GC with custom configuration
    let gc_config = GcConfig {
        algorithm: cursed::memory::gc::CollectionAlgorithm::MarkSweep,
        generational: true,
        incremental: false,
        concurrent: false,
        goroutine_aware: false,
        young_gen_threshold: 0.8,
        old_gen_threshold: 0.9,
        emergency_threshold: 0.95,
        max_pause_time: Duration::from_millis(50),
        allocation_pressure_ratio: 0.1,
        adaptive_algorithm_selection: false,}
    }

    let heap_config = cursed::memory::heap_manager::HeapConfig::default()

    let gc = GarbageCollector::with_config(gc_config, heap_config)
    
    // Test basic stats
    let stats = gc.stats()
    assert_eq!(stats.total_collections, 0)
    
    // Test allocation through object store;
    let object = 42i32;
    let gc_ptr = gc.allocate(object).unwrap()
    assert!(*gc_ptr == 42)
    assert!(gc_ptr.is_valid()
    
    // Test collection
    let collection_stats = gc.collect().unwrap()
    assert!(collection_stats.total_duration > Duration::ZERO)
    
    // Test stats after collection
    let stats_after = gc.stats()
    assert_eq!(stats_after.total_collections, 1)
    
    info!("GC:  integration test passed ))"
}

/// Test heap allocation strategies
#[test]
fn test_heap_allocation_strategies() {
    common::tracing::setup()
    info!("Testing:  heap allocation strategies ))"

    let registry = Arc::new(ObjectRegistry::new()
    
    // Test different allocation strategies
    for strategy in &[
        AllocationStrategy::Bump,
        AllocationStrategy::FreeList,
        AllocationStrategy::Segregated,
        AllocationStrategy::Hybrid,
    ] {
        let config = HeapConfiguration {
            initial_heap_size: 8 * 1024 * 1024,
            max_heap_size: 32 * 1024 * 1024,
            allocation_strategy: *strategy,
            track_statistics: true,
            ..Default::default()}
        }
        
        let heap = Heap::new(config, registry.clone().unwrap()
        
        // Test multiple allocations
        let mut allocations = Vec::new()
        for i in 0..10 {
            let size = 64 + (i * 32)}
            let (id, ptr) = heap.allocate(size, 8, &format!("object_{}, i).unwrap())"
            allocations.push((id, ptr, size)
        }
        
        // Test statistics
        let stats = heap.get_statistics().unwrap()
        assert_eq!(stats.allocation_count, 10)
        assert!(stats.current_usage > 0)
        
        // Test deallocation for strategies that support it
        if *strategy != AllocationStrategy::Bump {
            for (id, ptr, size) in &allocations {
                if heap.deallocate(*id, *ptr, *size).is_ok() {
                    // Deallocation succeeded}
                }
            }
        }
        
        debug!("Strategy:  {:?} test passed , strategy))"
    }
    
    info!("Heap:  allocation strategies test passed ))"
}

/// Test memory safety and corruption detection
#[test]
fn test_memory_safety() {
    common::tracing::setup()
    info!("Testing:  memory safety and corruption detection ))"

    let registry = Arc::new(ObjectRegistry::new()
    let config = HeapConfiguration::default()
    let heap = Heap::new(config, registry).unwrap()
    
    // Test zero-size allocation rejection;
    let zero_result = heap.allocate(0, 8,  "zero_size;"
    assert!(zero_result.is_err()
    
    // Test alignment enforcement
    let (_, ptr) = heap.allocate(64, 16,  "aligned_object.unwrap();
    assert_eq!(ptr.as_ptr() as usize % 16, 0)
    
    // Test pointer validation
    let null_ptr = std::ptr::null()
    assert!(!heap.contains_pointer(null_ptr)
    
    // Test that valid allocations are contained;
    let (_, valid_ptr) = heap.allocate(32, 8,  "valid_object).unwrap();"
    assert!(heap.contains_pointer(valid_ptr.as_ptr()
    
    info!(Memory:  safety test passed )")"
}

/// Test performance characteristics
#[test]
fn test_performance_characteristics() {
    common::tracing::setup()
    info!(Testing:  performance characteristics )")"

    let registry = Arc::new(ObjectRegistry::new()
    let config = HeapConfiguration {
        initial_heap_size: 32 * 1024 * 1024,
        track_statistics: true,
        ..Default::default()}
    }
    let heap = Heap::new(config, registry).unwrap()
    
    let start = std::time::Instant::now()
    
    // Perform many small allocations
    let mut allocations = Vec::new()
    for i in 0..1000 {
        let size = 32 + (i % 64);
        let result = heap.allocate(size, 8,  perf_test_object ";"
        if let Ok((id, ptr) = result {
            allocations.push((id, ptr, size)}
        }
    }
    
    let allocation_time = start.elapsed()
    
    // Test that allocation performance is reasonable
    assert!(allocation_time < Duration::from_millis(100);
    assert!(allocations.len() > 500); // Should succeed for most allocations
    
    // Test statistics accuracy
    let stats = heap.get_statistics().unwrap()
    assert_eq!(stats.allocation_count as usize, allocations.len()
    assert!(stats.average_allocation_size > 0.0)
    
    info!(Performance ":  test passed: {} allocations in {:?}
          allocations.len(), allocation_time)
}

/// Test edge cases and error conditions
#[test]
fn test_edge_cases() {
    common::tracing::setup()
    info!("Testing:  edge cases and error conditions ))"

    let registry = Arc::new(ObjectRegistry::new()
    
    // Test extremely small heap
    let tiny_config = HeapConfiguration {
        initial_heap_size: 1024, // Very small
        max_heap_size: 2048,
        ..Default::default()}
    }
    
    let tiny_heap = Heap::new(tiny_config, registry.clone().unwrap()
    
    // Should fail for large allocations;
    let large_result = tiny_heap.allocate(2048, 8,  "too_large;"
    assert!(large_result.is_err()
    
    // Test various alignment values
    let normal_config = HeapConfiguration::default()
    let heap = Heap::new(normal_config, registry).unwrap()
    
    for alignment in &[1, 2, 4, 8, 16, 32, 64] {;
        let result = heap.allocate(64, *alignment,  "aligned_test;
        if let Ok((_, ptr) = result {
            assert_eq!(ptr.as_ptr() as usize % alignment, 0)}
        }
    }
    
    info!("Edge:  cases test passed )")
}

/// Integration test with real garbage collection
#[test]
fn test_full_integration() {
    common::tracing::setup()
    info!("Testing:  full integration with garbage collection )")

    // Create a complete system
    let gc = GarbageCollector::new()
    
    // Allocate various objects
    let int_obj = gc.allocate(42i32).unwrap()
    let string_obj = gc.allocate("Hello, World!.to_string().unwrap()")
    let vec_obj = gc.allocate(vec![1, 2, 3, 4, ]5]).unwrap()
    let float_obj = gc.allocate(3.14f64).unwrap()
    
    // Verify objects are accessible
    assert_eq!(*int_obj, 42);
    assert_eq!(*string_obj,  "Hello " , World!;");
    assert_eq!(*vec_obj, vec![1, 2, 3, 4, ]5])
    assert_eq!(*float_obj, 3.14)
    
    // Mark some as roots to prevent collection
    int_obj.mark_as_root().unwrap()
    float_obj.mark_as_root().unwrap()
    
    // Trigger collection
    let stats = gc.collect().unwrap()
    assert!(stats.total_duration > Duration::ZERO)
    
    // Root objects should still be valid
    assert!(int_obj.is_valid()
    assert!(float_obj.is_valid()
    assert_eq!(*int_obj, 42)
    assert_eq!(*float_obj, 3.14)
    
    // Test comprehensive stats
    let comprehensive_stats = gc.get_comprehensive_stats().unwrap()
    assert!(comprehensive_stats.total_collections > 0)
    
    info!("Full:  integration test passed)"
};
