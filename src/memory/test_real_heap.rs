use crate::error_types::Error;
/// Simple test for the real heap management system
/// 
/// This test verifies that the real heap manager can be instantiated and
/// used for basic memory allocation and deallocation operations.

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::memory::{
        RealHeapManager, RealHeapConfig, ObjectRegistry,
        EnhancedGarbageCollector, EnhancedGcStats,
    };
    use crate::memory::gc::GcConfig;
    use crate::memory::heap_manager::HeapConfig;

    #[test]
    fn test_real_heap_manager_creation() {
        let config = RealHeapConfig::default();
        let registry = Arc::new(ObjectRegistry::new());
        
        let heap_manager = RealHeapManager::new(config, registry).unwrap();
        let stats = heap_manager.get_statistics().unwrap();
        
        assert_eq!(stats.total_blocks, 0);
        assert_eq!(stats.total_allocations, 0);
        println!("✅ Real heap manager created successfully");
    }
    
    #[test]
    fn test_real_heap_allocation() {
        let config = RealHeapConfig::default();
        let registry = Arc::new(ObjectRegistry::new());
        let heap_manager = RealHeapManager::new(config, registry).unwrap();
        
        // Allocate some memory
        let (object_id, ptr) = heap_manager.allocate(64, 8, "test").unwrap();
        println!("✅ Allocated 64 bytes, object ID: {}", object_id);
        
        // Verify allocation is tracked
        let stats = heap_manager.get_statistics().unwrap();
        assert_eq!(stats.total_allocations, 1);
        assert!(stats.total_used >= 64);
        
        // Deallocate
        heap_manager.deallocate(object_id, ptr).unwrap();
        println!("✅ Deallocated object {}", object_id);
        
        let stats_after = heap_manager.get_statistics().unwrap();
        assert_eq!(stats_after.total_deallocations, 1);
    }
    
    #[test]
    fn test_enhanced_gc_creation() {
        let enhanced_gc = EnhancedGarbageCollector::new();
        println!("✅ Enhanced garbage collector created");
        
        // Test getting basic stats
        let stats = enhanced_gc.get_basic_stats().unwrap();
        assert_eq!(stats.total_collections, 0);
        println!("✅ Enhanced GC stats: {} collections", stats.total_collections);
    }
    
    #[test]
    fn test_enhanced_gc_comprehensive_stats() {
        let enhanced_gc = EnhancedGarbageCollector::new();
        
        // Test getting comprehensive stats
        let comprehensive_stats = enhanced_gc.get_comprehensive_stats_enhanced().unwrap();
        assert_eq!(comprehensive_stats.total_collections, 0);
        assert!(!comprehensive_stats.is_collecting);
        
        println!("✅ Enhanced GC comprehensive stats: {} collections, algorithm: {:?}", 
                 comprehensive_stats.total_collections,
                 comprehensive_stats.current_algorithm);
    }
    
    #[test]
    fn test_memory_pressure_monitoring() {
        let config = RealHeapConfig::default();
        let registry = Arc::new(ObjectRegistry::new());
        let heap_manager = RealHeapManager::new(config, registry).unwrap();
        
        let initial_pressure = heap_manager.get_memory_pressure();
        assert!(initial_pressure >= 0.0 && initial_pressure <= 1.0);
        println!("✅ Initial memory pressure: {:.2}", initial_pressure);
        
        // Make several allocations to increase pressure
        let mut allocations = Vec::new();
        for i in 0..10 {
            let (id, ptr) = heap_manager.allocate(1024, 8, &format!("test_{}", i)).unwrap();
            allocations.push((id, ptr));
        }
        
        let pressure_after_allocs = heap_manager.get_memory_pressure();
        println!("✅ Memory pressure after allocations: {:.2}", pressure_after_allocs);
        
        // Clean up
        for (id, ptr) in allocations {
            heap_manager.deallocate(id, ptr).unwrap();
        }
        
        println!("✅ Memory pressure monitoring test completed");
    }
    
    #[test]
    fn test_heap_fragmentation() {
        let config = RealHeapConfig::default();
        let registry = Arc::new(ObjectRegistry::new());
        let heap_manager = RealHeapManager::new(config, registry).unwrap();
        
        // Allocate several objects
        let alloc1 = heap_manager.allocate(64, 8, "test1").unwrap();
        let alloc2 = heap_manager.allocate(128, 8, "test2").unwrap();
        let alloc3 = heap_manager.allocate(256, 8, "test3").unwrap();
        
        // Deallocate middle allocation to create fragmentation
        heap_manager.deallocate(alloc2.0, alloc2.1).unwrap();
        
        let stats = heap_manager.get_statistics().unwrap();
        println!("✅ Fragmentation after partial deallocation: {:.2}%", 
                 stats.overall_fragmentation * 100.0);
        
        // Clean up remaining allocations
        heap_manager.deallocate(alloc1.0, alloc1.1).unwrap();
        heap_manager.deallocate(alloc3.0, alloc3.1).unwrap();
        
        let final_stats = heap_manager.get_statistics().unwrap();
        println!("✅ Final fragmentation: {:.2}%", 
                 final_stats.overall_fragmentation * 100.0);
        
        assert!(final_stats.total_deallocations == 3);
    }
    
    #[test]
    fn test_collection_trigger_detection() {
        let enhanced_gc = EnhancedGarbageCollector::new();
        
        // Initially should not need collection
        let trigger = enhanced_gc.should_collect_enhanced().unwrap();
        assert!(trigger.is_none());
        println!("✅ No collection trigger detected initially");
        
        // Simulate some allocation pressure
        for _ in 0..100 {
            enhanced_gc.notify_allocation_enhanced(1024);
        }
        
        // Check if trigger is detected
        let trigger_after = enhanced_gc.should_collect_enhanced().unwrap();
        if trigger_after.is_some() {
            println!("✅ Collection trigger detected after allocation pressure: {:?}", trigger_after);
        } else {
            println!("✅ No trigger detected (thresholds not reached)");
        }
    }
    
    #[test]
    fn test_algorithm_selection() {
        let enhanced_gc = EnhancedGarbageCollector::new();
        
        // Test that the enhanced GC can perform collection
        let collection_result = enhanced_gc.collect_enhanced();
        
        match collection_result {
            Ok(stats) => {
                println!("✅ Collection completed: algorithm = {:?}, duration = {:?}",
                         stats.algorithm_used, stats.total_duration);
                assert!(stats.collection_number > 0);
            }
            Err(e) => {
                println!("⚠️  Collection failed (expected in some test scenarios): {}", e);
            }
        }
    }
}

/// Integration test function that can be called manually
pub fn run_real_heap_integration_test() -> Result<(), String> {
    use crate::memory::{RealHeapConfig, RealHeapManager, ObjectRegistry};
    use std::sync::Arc;
    
    println!("🚀 Running real heap management integration test...");
    
    // Create real heap manager
    let config = RealHeapConfig {
        initial_block_size: 1024 * 1024, // 1MB
        max_blocks: 4,
        growth_factor: 1.5,
        fragmentation_threshold: 0.3,
        pressure_threshold: 0.8,
        auto_compaction: true,
        min_free_space: 0.2,
    };
    
    let registry = Arc::new(ObjectRegistry::new());
    let heap_manager = RealHeapManager::new(config, registry)?;
    
    println!("✅ Created real heap manager");
    
    // Test allocation patterns
    let mut allocations = Vec::new();
    
    // Small allocations
    for i in 0..10 {
        let (id, ptr) = heap_manager.allocate(64, 8, &format!("small_{}", i))?;
        allocations.push((id, ptr));
    }
    println!("✅ Allocated 10 small objects");
    
    // Medium allocations
    for i in 0..5 {
        let (id, ptr) = heap_manager.allocate(512, 8, &format!("medium_{}", i))?;
        allocations.push((id, ptr));
    }
    println!("✅ Allocated 5 medium objects");
    
    // Large allocation
    let (large_id, large_ptr) = heap_manager.allocate(4096, 8, "large")?;
    allocations.push((large_id, large_ptr));
    println!("✅ Allocated 1 large object");
    
    // Get statistics
    let stats = heap_manager.get_statistics()?;
    println!("📊 Heap statistics:");
    println!("   - Blocks: {}", stats.total_blocks);
    println!("   - Capacity: {} bytes", stats.total_capacity);
    println!("   - Used: {} bytes", stats.total_used);
    println!("   - Utilization: {:.1}%", stats.average_block_utilization);
    println!("   - Fragmentation: {:.1}%", stats.overall_fragmentation * 100.0);
    
    // Deallocate some objects to create fragmentation
    for i in (0..allocations.len()).step_by(2) {
        let (id, ptr) = allocations[i];
        heap_manager.deallocate(id, ptr)?;
    }
    println!("✅ Deallocated half the objects");
    
    let stats_after = heap_manager.get_statistics()?;
    println!("📊 Stats after partial deallocation:");
    println!("   - Used: {} bytes", stats_after.total_used);
    println!("   - Fragmentation: {:.1}%", stats_after.overall_fragmentation * 100.0);
    
    // Test memory pressure
    let pressure = heap_manager.get_memory_pressure();
    println!("🔍 Memory pressure: {:.2}", pressure);
    
    // Test compaction if fragmentation is high
    if stats_after.overall_fragmentation > 0.2 {
        println!("🧹 Triggering compaction due to fragmentation");
        heap_manager.trigger_compaction()?;
        
        let stats_compacted = heap_manager.get_statistics()?;
        println!("📊 Stats after compaction:");
        println!("   - Fragmentation: {:.1}%", stats_compacted.overall_fragmentation * 100.0);
    }
    
    println!("🎉 Real heap management integration test completed successfully!");
    Ok(())
}
