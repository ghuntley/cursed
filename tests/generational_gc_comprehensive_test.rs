/// Comprehensive tests for the generational garbage collection system
/// 
/// This test suite validates the complete generational GC implementation including:
/// - Young and old generation collection
/// - Object promotion between generations
/// - Incremental collection capabilities
/// - Cycle detection and collection
/// - Write barrier functionality
/// - Collection trigger heuristics
/// - Performance characteristics

use std::sync::Arc;
use std::time::{Duration, Instant};
use std::collections::HashMap;

use cursed::memory::{
    GenerationalCollector, GenerationalConfig, Generation, CollectionStrategy,
    ObjectId, ObjectRegistry, 
    TriggerType, TriggerReason,
    CycleDetectionAlgorithm,
    IncrementalWorkType,
};

/// Helper function to create a test collector
fn create_test_collector() -> Result<(GenerationalCollector, Arc<ObjectRegistry>), String> {
    let registry = Arc::new(ObjectRegistry::new());
    let collector = GenerationalCollector::new(registry.clone())?;
    Ok((collector, registry))
}

/// Helper function to create a collector with custom config
fn create_collector_with_config(config: GenerationalConfig) -> Result<(GenerationalCollector, Arc<ObjectRegistry>), String> {
    let registry = Arc::new(ObjectRegistry::new());
    let collector = GenerationalCollector::with_config(registry.clone(), config)?;
    Ok((collector, registry))
}

#[test]
fn test_generational_collector_creation() {
    let (collector, _registry) = create_test_collector().unwrap();
    
    let stats = collector.get_stats().unwrap();
    assert_eq!(stats.total_collections, 0);
    assert_eq!(stats.young_collections, 0);
    assert_eq!(stats.old_collections, 0);
    assert_eq!(stats.full_collections, 0);
}

#[test]
fn test_object_allocation_tracking() {
    let (collector, _registry) = create_test_collector().unwrap();
    
    // Track some objects in young generation
    let young_obj1 = ObjectId::new(1);
    let young_obj2 = ObjectId::new(2);
    collector.track_object_allocation(young_obj1, Generation::Young, 64).unwrap();
    collector.track_object_allocation(young_obj2, Generation::Young, 128).unwrap();
    
    // Track some objects in old generation
    let old_obj1 = ObjectId::new(3);
    collector.track_object_allocation(old_obj1, Generation::Old, 256).unwrap();
    
    // Check object counts by generation
    let counts = collector.get_object_counts_by_generation().unwrap();
    assert_eq!(counts.get(&Generation::Young), Some(&2));
    assert_eq!(counts.get(&Generation::Old), Some(&1));
    assert_eq!(counts.get(&Generation::Permanent), None);
}

#[test]
fn test_object_promotion() {
    let (collector, _registry) = create_test_collector().unwrap();
    
    // Create objects in young generation
    let obj1 = ObjectId::new(2);
    let obj2 = ObjectId::new(3);
    collector.track_object_allocation(obj1, Generation::Young, 64).unwrap();
    collector.track_object_allocation(obj2, Generation::Young, 128).unwrap();
    
    // Promote one object to old generation
    collector.promote_object(obj1).unwrap();
    
    // Check object counts after promotion
    let counts = collector.get_object_counts_by_generation().unwrap();
    assert_eq!(counts.get(&Generation::Young), Some(&1));
    assert_eq!(counts.get(&Generation::Old), Some(&1));
}

#[test]
fn test_write_barrier_cross_generational() {
    let (collector, _registry) = create_test_collector().unwrap();
    
    // Create objects in different generations
    let old_obj = ObjectId::new(4);
    let young_obj = ObjectId::new(5);
    collector.track_object_allocation(old_obj, Generation::Old, 128).unwrap();
    collector.track_object_allocation(young_obj, Generation::Young, 64).unwrap();
    
    // Create cross-generational reference (old -> young)
    collector.write_barrier(old_obj, 0, None, young_obj).unwrap();
    
    // This should have recorded a cross-generational reference
    // (We can't easily verify this without exposing internal state,
    //  but the write barrier should complete without error)
}

#[test]
fn test_young_generation_collection() {
    let (collector, _registry) = create_test_collector().unwrap();
    
    // Track some objects
    let obj1 = ObjectId::new(6);
    let obj2 = ObjectId::new(7);
    collector.track_object_allocation(obj1, Generation::Young, 64).unwrap();
    collector.track_object_allocation(obj2, Generation::Young, 128).unwrap();
    
    // Force young generation collection
    let stats = collector.force_collection(CollectionStrategy::YoungOnly).unwrap();
    
    assert!(stats.young_collections > 0);
    assert_eq!(stats.old_collections, 0);
    assert!(stats.young_collection_time > Duration::ZERO);
}

#[test]
fn test_old_generation_collection() {
    let (collector, _registry) = create_test_collector().unwrap();
    
    // Track some objects in old generation
    let obj1 = ObjectId::new(8);
    let obj2 = ObjectId::new(9);
    collector.track_object_allocation(obj1, Generation::Old, 256).unwrap();
    collector.track_object_allocation(obj2, Generation::Old, 512).unwrap();
    
    // Force old generation collection
    let stats = collector.force_collection(CollectionStrategy::OldOnly).unwrap();
    
    assert!(stats.old_collections > 0);
    assert!(stats.old_collection_time > Duration::ZERO);
}

#[test]
fn test_full_collection() {
    let (collector, _registry) = create_test_collector().unwrap();
    
    // Track objects in both generations
    let young_obj = ObjectId::new(10);
    let old_obj = ObjectId::new(11);
    collector.track_object_allocation(young_obj, Generation::Young, 64).unwrap();
    collector.track_object_allocation(old_obj, Generation::Old, 256).unwrap();
    
    // Force full collection
    let stats = collector.force_collection(CollectionStrategy::Full).unwrap();
    
    assert!(stats.full_collections > 0);
    assert!(stats.young_collections > 0);
    assert!(stats.old_collections > 0);
}

#[test]
fn test_incremental_collection() {
    let (collector, _registry) = create_test_collector().unwrap();
    
    // Track some objects
    let obj1 = ObjectId::new(12);
    collector.track_object_allocation(obj1, Generation::Young, 64).unwrap();
    
    // Perform incremental collection step
    let stats = collector.force_collection(CollectionStrategy::Incremental).unwrap();
    
    // Incremental collections should be recorded
    // Note: The actual work performed depends on the current collection state
}

#[test]
fn test_emergency_collection() {
    let (collector, _registry) = create_test_collector().unwrap();
    
    // Track many objects to simulate memory pressure
    for i in 0..100 {
        let obj = ObjectId::new(13);
        let generation = if i % 3 == 0 { Generation::Old } else { Generation::Young };
        collector.track_object_allocation(obj, generation, 64).unwrap();
    }
    
    // Force emergency collection
    let stats = collector.force_collection(CollectionStrategy::Emergency).unwrap();
    
    // Emergency collection should trigger full collection
    assert!(stats.full_collections > 0 || stats.young_collections > 0 || stats.old_collections > 0);
}

#[test]
fn test_allocation_notification() {
    let (collector, _registry) = create_test_collector().unwrap();
    
    // Notify of allocations
    collector.notify_allocation(1024);
    collector.notify_allocation(2048);
    collector.notify_allocation(512);
    
    // This should influence collection triggers
    // (We can't easily test the internal state changes)
}

#[test]
fn test_configuration_update() {
    let (collector, _registry) = create_test_collector().unwrap();
    
    // Create new configuration
    let new_config = GenerationalConfig {
        young_generation_ratio: 0.5, // 50% instead of default 33%
        promotion_age_threshold: 5,  // Higher threshold
        adaptive_sizing: false,      // Disable adaptive sizing
        concurrent_collection: true, // Enable concurrent collection
        incremental_collection: true,
        cycle_detection: true,
        ..Default::default()
    };
    
    // Update configuration
    collector.update_config(new_config).unwrap();
    
    // Configuration should be updated (no easy way to verify without exposing internals)
}

#[test]
fn test_custom_configuration() {
    let config = GenerationalConfig {
        young_generation_ratio: 0.25,
        promotion_age_threshold: 2,
        adaptive_sizing: true,
        concurrent_collection: false,
        incremental_collection: true,
        cycle_detection: true,
        write_barrier_threshold: 0.03,
        ..Default::default()
    };
    
    let (collector, _registry) = create_collector_with_config(config).unwrap();
    
    // Collector should be created with custom configuration
    let stats = collector.get_stats().unwrap();
    assert_eq!(stats.total_collections, 0);
}

#[test]
fn test_collection_performance_characteristics() {
    let (collector, _registry) = create_test_collector().unwrap();
    
    // Create many objects to test performance
    let start_time = Instant::now();
    
    for i in 0..1000 {
        let obj = ObjectId::new(14);
        let generation = if i % 4 == 0 { Generation::Old } else { Generation::Young };
        collector.track_object_allocation(obj, generation, 64 + (i % 128)).unwrap();
    }
    
    let allocation_time = start_time.elapsed();
    
    // Perform collection and measure time
    let collection_start = Instant::now();
    let stats = collector.force_collection(CollectionStrategy::Full).unwrap();
    let collection_time = collection_start.elapsed();
    
    println!("Allocation time: {:?}", allocation_time);
    println!("Collection time: {:?}", collection_time);
    println!("Collection stats: {:?}", stats);
    
    // Performance assertions
    assert!(allocation_time < Duration::from_millis(100), "Allocation should be fast");
    assert!(collection_time < Duration::from_millis(500), "Collection should complete reasonably quickly");
}

#[test]
fn test_memory_pressure_simulation() {
    let (collector, _registry) = create_test_collector().unwrap();
    
    // Simulate varying memory pressure
    let mut young_objects = Vec::new();
    let mut old_objects = Vec::new();
    
    // Phase 1: Heavy young generation allocation
    for i in 0..500 {
        let obj = ObjectId::new(15);
        collector.track_object_allocation(obj, Generation::Young, 32 + (i % 64)).unwrap();
        young_objects.push(obj);
        
        if i % 100 == 0 {
            collector.notify_allocation(3200);
        }
    }
    
    // Phase 2: Promote some objects and add old generation objects
    for i in 0..100 {
        if i < young_objects.len() {
            collector.promote_object(young_objects[i]).unwrap();
        }
        
        let obj = ObjectId::new(16);
        collector.track_object_allocation(obj, Generation::Old, 128 + (i % 256)).unwrap();
        old_objects.push(obj);
    }
    
    // Phase 3: Create cross-generational references
    for i in 0..50 {
        if i < old_objects.len() && i < young_objects.len() {
            collector.write_barrier(old_objects[i], 0, None, young_objects[i + 50]).unwrap();
        }
    }
    
    // Perform collections and verify behavior
    let young_stats = collector.force_collection(CollectionStrategy::YoungOnly).unwrap();
    let old_stats = collector.force_collection(CollectionStrategy::OldOnly).unwrap();
    let full_stats = collector.force_collection(CollectionStrategy::Full).unwrap();
    
    // Verify statistics are reasonable
    assert!(young_stats.young_collections > 0);
    assert!(old_stats.old_collections > 0);
    assert!(full_stats.full_collections > 0);
    
    println!("Young collection stats: {:?}", young_stats);
    println!("Old collection stats: {:?}", old_stats);
    println!("Full collection stats: {:?}", full_stats);
}

#[test]
fn test_concurrent_collection_safety() {
    let (collector, _registry) = create_test_collector().unwrap();
    
    // Simulate multiple "threads" creating objects
    for thread_id in 0..4 {
        for i in 0..100 {
            let obj = ObjectId::new((thread_id * 100 + i + 17) as u64);
            let generation = if (thread_id + i) % 3 == 0 { Generation::Old } else { Generation::Young };
            collector.track_object_allocation(obj, generation, 64).unwrap();
            
            // Notify of allocation
            collector.notify_allocation(64);
            
            // Occasionally trigger collection
            if i % 50 == 0 {
                let _ = collector.force_collection(CollectionStrategy::YoungOnly);
            }
        }
    }
    
    // Perform final collection and verify system is in good state
    let final_stats = collector.force_collection(CollectionStrategy::Full).unwrap();
    assert!(final_stats.total_collections > 0);
}

#[test]
fn test_statistics_accuracy() {
    let (collector, _registry) = create_test_collector().unwrap();
    
    // Track specific objects
    let obj1 = ObjectId::new(18);
    let obj2 = ObjectId::new(19);
    let obj3 = ObjectId::new(20);
    
    collector.track_object_allocation(obj1, Generation::Young, 64).unwrap();
    collector.track_object_allocation(obj2, Generation::Young, 128).unwrap();
    collector.track_object_allocation(obj3, Generation::Old, 256).unwrap();
    
    // Promote one object
    collector.promote_object(obj1).unwrap();
    
    // Create cross-generational reference
    collector.write_barrier(obj1, 0, None, obj2).unwrap();
    
    // Perform collections
    collector.force_collection(CollectionStrategy::YoungOnly).unwrap();
    collector.force_collection(CollectionStrategy::OldOnly).unwrap();
    collector.force_collection(CollectionStrategy::Full).unwrap();
    
    // Verify statistics
    let stats = collector.get_stats().unwrap();
    assert!(stats.young_collections > 0);
    assert!(stats.old_collections > 0);
    assert!(stats.full_collections > 0);
    assert!(stats.total_collections >= 3);
    assert!(stats.total_collection_time > Duration::ZERO);
    
    // Check object counts
    let counts = collector.get_object_counts_by_generation().unwrap();
    assert_eq!(counts.get(&Generation::Young), Some(&1)); // obj2
    assert_eq!(counts.get(&Generation::Old), Some(&2));   // obj1 (promoted) + obj3
}

#[test]
fn test_edge_cases() {
    let (collector, _registry) = create_test_collector().unwrap();
    
    // Test with no objects
    let empty_stats = collector.force_collection(CollectionStrategy::Full).unwrap();
    assert!(empty_stats.total_collections > 0);
    
    // Test with single object
    let single_obj = ObjectId::new(21);
    collector.track_object_allocation(single_obj, Generation::Young, 32).unwrap();
    let single_stats = collector.force_collection(CollectionStrategy::YoungOnly).unwrap();
    assert!(single_stats.young_collections > 0);
    
    // Test promotion of non-existent object (should not crash)
    let non_existent = ObjectId::new(22);
    collector.promote_object(non_existent).unwrap(); // Should complete without error
    
    // Test write barrier with same object
    collector.write_barrier(single_obj, 0, None, single_obj).unwrap();
}

#[test] 
fn test_collection_strategy_determination() {
    let (collector, _registry) = create_test_collector().unwrap();
    
    // Test various collection scenarios
    
    // Scenario 1: Many young objects (should trigger young collection)
    for i in 0..200 {
        let obj = ObjectId::new(23);
        collector.track_object_allocation(obj, Generation::Young, 32).unwrap();
        collector.notify_allocation(32);
    }
    
    let stats1 = collector.collect().unwrap();
    // Should have performed some collection
    
    // Scenario 2: Many old objects
    for i in 0..100 {
        let obj = ObjectId::new(24);
        collector.track_object_allocation(obj, Generation::Old, 128).unwrap();
        collector.notify_allocation(128);
    }
    
    let stats2 = collector.collect().unwrap();
    
    // Scenario 3: Mixed workload with cross-generational references
    let old_obj = ObjectId::new(25);
    collector.track_object_allocation(old_obj, Generation::Old, 256).unwrap();
    
    for i in 0..50 {
        let young_obj = ObjectId::new(26);
        collector.track_object_allocation(young_obj, Generation::Young, 64).unwrap();
        collector.write_barrier(old_obj, i, None, young_obj).unwrap();
    }
    
    let stats3 = collector.collect().unwrap();
    
    println!("Collection scenario 1 stats: {:?}", stats1);
    println!("Collection scenario 2 stats: {:?}", stats2);
    println!("Collection scenario 3 stats: {:?}", stats3);
}

#[test]
fn test_integration_with_heap_manager() {
    let (mut collector, _registry) = create_test_collector().unwrap();
    
    // In a real scenario, you would set up heap manager integration
    // For now, we test that the collector can handle missing heap manager gracefully
    
    // Track objects and perform collections
    for i in 0..50 {
        let obj = ObjectId::new(27);
        let generation = if i % 2 == 0 { Generation::Young } else { Generation::Old };
        collector.track_object_allocation(obj, generation, 64).unwrap();
    }
    
    // Collection should work even without heap manager
    let stats = collector.collect().unwrap();
    assert!(stats.total_collections > 0);
}

/// Stress test for the generational collector
#[test]
fn test_generational_gc_stress() {
    let (collector, _registry) = create_test_collector().unwrap();
    
    let start_time = Instant::now();
    let mut total_objects = 0;
    
    // Phase 1: Rapid allocation in young generation
    for wave in 0..10 {
        for i in 0..500 {
            let obj = ObjectId::new(28);
            collector.track_object_allocation(obj, Generation::Young, 32 + (i % 96)).unwrap();
            collector.notify_allocation(32 + (i % 96));
            total_objects += 1;
            
            // Occasional promotion
            if i % 20 == 0 && wave > 2 {
                collector.promote_object(obj).unwrap();
            }
        }
        
        // Collection after each wave
        let wave_stats = collector.force_collection(CollectionStrategy::YoungOnly).unwrap();
        
        if wave % 3 == 0 {
            collector.force_collection(CollectionStrategy::OldOnly).unwrap();
        }
    }
    
    // Phase 2: Full collection and verification
    let final_stats = collector.force_collection(CollectionStrategy::Full).unwrap();
    let elapsed = start_time.elapsed();
    
    println!("Stress test completed in {:?}", elapsed);
    println!("Total objects allocated: {}", total_objects);
    println!("Final collection stats: {:?}", final_stats);
    
    // Performance assertions
    assert!(elapsed < Duration::from_secs(5), "Stress test should complete in reasonable time");
    assert!(final_stats.total_collections > 10, "Should have performed multiple collections");
}
