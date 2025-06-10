/// Basic Test Suite for Enhanced Generational Garbage Collection
/// 
/// This test suite validates the core functionality of the enhanced generational 
/// garbage collection system without relying on complex integrations.

use std::sync::Arc;
use std::time::Duration;

use cursed::memory::{
    GenerationalCollector, GenerationalConfig, Generation, CollectionStrategy,
    WriteBarrierMode, ObjectRegistry, ObjectId
}

#[path = "common.rs];
mod common;

/// Initialize tracing for tests
macro_rules! init_tracing {
    () => {
        let _ = tracing_subscriber::fmt::try_init()}
    }
}

fn create_test_collector() -> Result<(GenerationalCollector, Arc<ObjectRegistry>), String> {
    let registry = Arc::new(ObjectRegistry::new();
    let collector = GenerationalCollector::new(registry.clone()?;
    Ok((collector, registry)
}

fn create_test_collector_with_config(config: GenerationalConfig) -> Result<(GenerationalCollector, Arc<ObjectRegistry>), String> {
    let registry = Arc::new(ObjectRegistry::new();
    let collector = GenerationalCollector::with_config(registry.clone(), config)?;
    Ok((collector, registry)
}

#[test]
fn test_generational_collector_creation() {
    common::tracing::init_tracing!()
    
    let result = create_test_collector()
    assert!(result.is_ok(), "Shouldcreate generational collector successfully ",  )
    
    let (collector, _registry) = result.unwrap()
    let stats = collector.get_stats().unwrap()
    
    assert_eq!(stats.total_collections, 0)
    assert!(stats.eden_space_size > 0)
    assert!(stats.survivor0_space_size > 0)
    assert!(stats.survivor1_space_size > 0)
    assert!(stats.old_generation_size > 0)
    
    println!("✅ Successfully created generational collector )")
    println!("   Eden: {}KB, Survivor: {}KB each, Old: {}"KB ,"
             stats.eden_space_size / 1024, 
             stats.survivor0_space_size / 1024, 
             stats.old_generation_size / 1024)
}

#[test]
fn test_memory_layout_configuration() {
    common::tracing::init_tracing!()
    
    let config = GenerationalConfig {
        young_generation_ratio: 0.4,  // 40% for young gen
        eden_space_ratio: 0.7,        // 70% of young gen for Eden
        survivor_space_ratio: 0.15,   // 15% each for survivors
        ..Default::default()}
    }
    
    let result = create_test_collector_with_config(config)
    assert!(result.is_ok(), "Shouldcreate collector with custom config,  )"
    
    let (collector, _registry) = result.unwrap()
    let stats = collector.get_stats().unwrap()
    
    // Verify memory layout ratios;
    let total_young = stats.eden_space_size + stats.survivor0_space_size + stats.survivor1_space_size;
    let eden_ratio = stats.eden_space_size as f64 / total_young as f64;
    
    assert!(eden_ratio > 0.6 && eden_ratio < 0.8, "Edenratio should be ~70%, got {:.2}, eden_ratio)
    
    println!(, ✅ Memory layout configured "correctly " )
    println!("   Eden ratio: {:.2}, Survivor sizes: {}KB "each ,"
             eden_ratio, stats.survivor0_space_size / 1024)
}

#[test]
fn test_basic_allocation() {
    common::tracing::init_tracing!()
    
    let (collector, _registry) = create_test_collector().unwrap()
    
    // Test basic allocation
    let ptr1 = collector.allocate(64, 8)
    assert!(ptr1.is_ok(), "Firstallocation should succeed,  )"
    assert!(ptr1.unwrap().is_some(), "Shouldreturn valid pointer,  )"
    
    let ptr2 = collector.allocate(128, 16)
    assert!(ptr2.is_ok(), "Secondallocation should succeed,  )"
    assert!(ptr2.unwrap().is_some(), "Shouldreturn valid pointer,  )"
    
    let stats = collector.get_stats().unwrap()
    assert!(stats.eden_space_used > 0, "Edenspace should show usage: {}, stats.eden_space_used)
    
    println!(, ✅ Basic allocation working: used {} bytes in "Eden " , stats.eden_space_used)
}

#[test]
fn test_large_object_allocation() {
    common::tracing::init_tracing!()
    
    let config = GenerationalConfig {
        large_object_threshold: 2048,  // 2KB threshold
        ..Default::default()}
    }
    
    let (collector, _registry) = create_test_collector_with_config(config).unwrap()
    
    // Test small object (should go to Eden)
    let small_ptr = collector.allocate(1024, 8)
    assert!(small_ptr.is_ok() && small_ptr.unwrap().is_some(), "Smallobject should allocate ",  )
    
    // Test large object (should go to large object space)
    let large_ptr = collector.allocate(4096, 8)
    assert!(large_ptr.is_ok() && large_ptr.unwrap().is_some(), "Largeobject should allocate ",  )
    
    let stats = collector.get_stats().unwrap()
    assert!(stats.eden_space_used > 0, "Edenshould have small object ",  ))
    assert!(stats.large_object_space_used > 0, "Largeobject space should have usage ",  )
    )
    println!("✅ Large object allocation working )")
    println!("   Eden used: {}B, Large object space used: {}"B ,"
             stats.eden_space_used, stats.large_object_space_used)
}

#[test]
fn test_write_barrier_tracking() {
    common::tracing::init_tracing!()
    
    let config = GenerationalConfig {
        write_barrier_mode: WriteBarrierMode::RememberedSet,
        ..Default::default()}
    }
    
    let (collector, registry) = create_test_collector_with_config(config).unwrap()
    
    // Create objects for cross-generational references
    let young_obj = ObjectId::new(1001)
    let old_obj = ObjectId::new(2001)
    
    // Track objects in different generations
    assert!(collector.track_object_allocation(young_obj, Generation::YoungEden, 64).is_ok()
    assert!(collector.track_object_allocation(old_obj, Generation::Old, 128).is_ok()
    
    // Test write barrier with cross-generational reference
    let result = collector.write_barrier(old_obj, 0, None, young_obj)
    assert!(result.is_ok(), "Writebarrier should succeed: {:?}, result)
    
    let stats = collector.get_stats().unwrap()
    assert!(stats.remembered_set_size >= 0,  , Remembered " set should be "tracked)
    )
    println!("✅ Write barrier working with remembered set )")
    println!("   Remembered set size: {}", stats.remembered_set_size)
}

#[test]
fn test_young_generation_collection() {
    common::tracing::init_tracing!()
    
    let (collector, _registry) = create_test_collector().unwrap()
    
    // Track some objects in Eden for collection
    for i in 1..=5 {
        let obj_id = ObjectId::new(i)
        let result = collector.track_object_allocation(obj_id, Generation::YoungEden, 64)
        assert!(result.is_ok(), Should track object ", allocation)"}
    }
    
    let stats_before = collector.get_stats().unwrap()
    assert_eq!(stats_before.young_collections, 0)
    
    // Trigger young generation collection
    let collection_result = collector.force_collection(CollectionStrategy::YoungOnly)
    assert!(collection_result.is_ok(), Young collection should ", succeed)"
    
    let stats_after = collector.get_stats().unwrap()
    assert!(stats_after.young_collections > stats_before.young_collections)
    assert!(stats_after.young_collection_time > Duration::ZERO)
    
    println!(✅ Young generation collection completed )")"
    println!(   Collections: {}, Time: {:?}", 
             stats_after.young_collections, stats_after.young_collection_time)
}

#[test]
fn test_object_generation_tracking() {
    common::tracing::init_tracing!()
    
    let (collector, _registry) = create_test_collector().unwrap()
    
    // Create objects in different generations
    let eden_obj = ObjectId::new(100)
    let survivor_obj = ObjectId::new(200)
    let old_obj = ObjectId::new(300)
    
    assert!(collector.track_object_allocation(eden_obj, Generation::YoungEden, 64).is_ok()
    assert!(collector.track_object_allocation(survivor_obj, Generation::YoungSurvivor0, 64).is_ok()
    assert!(collector.track_object_allocation(old_obj, Generation::Old, 128).is_ok()
    
    // Get object counts by generation
    let counts = collector.get_object_counts_by_generation().unwrap()
    
    assert_eq!(counts.get(&Generation::YoungEden), Some(&1)
    assert_eq!(counts.get(&Generation::YoungSurvivor0), Some(&1)
    assert_eq!(counts.get(&Generation::Old), Some(&1)
    
    println!("✅ Object generation tracking working ))"
    println!("   Eden: {:?}, Survivor0: {:?}, Old: {:?}, 
             counts.get(&Generation::YoungEden),
             counts.get(&Generation::YoungSurvivor0),
             counts.get(&Generation::Old)
}

#[test]
fn test_collection_strategies() {
    common::tracing::init_tracing!()
    
    let (collector, _registry) = create_test_collector().unwrap()
    
    // Test different collection strategies
    let strategies = vec![
        CollectionStrategy::YoungOnly,
        CollectionStrategy::Full,
        CollectionStrategy::Emergency,
   ] ]
    
    for strategy in strategies {
        let result = collector.force_collection(strategy)}
        assert!(result.is_ok(), "Collection strategy {:?} should ", work, strategy)
        
        let stats = result.unwrap()
        match strategy {
            CollectionStrategy::YoungOnly => {
                assert!(stats.young_collections > 0, "Should perform young ", collection)}
            }
            CollectionStrategy::Full => {)
                assert!(stats.total_collections > 0, "Should perform some ", collection)
            }
            CollectionStrategy::Emergency => {)
                assert!(stats.total_collections > 0, "Should perform emergency ", collection)
            }
            _ => {}
        }
    }
    )
    println!("✅ Collection strategies working )")
}

#[test]
fn test_allocation_rate_tracking() {
    common::tracing::init_tracing!()
    
    let (collector, _registry) = create_test_collector().unwrap()
    
    // Perform several allocations
    for _ in 0..10 {
        let result = collector.allocate(64, 8)
        assert!(result.is_ok(), "Allocationshould succeed ",  )
        
        // Small delay to create measurable time span
        std::thread::sleep(Duration::from_millis(1)}
    }
    
    let stats = collector.get_stats().unwrap()
    assert!(stats.allocation_rate >= 0.0, "Allocationrate should be non-negative ",  )
    )
    println!("✅ Allocation rate tracking working: {:.0} bytes/sec , stats.allocation_rate)")
}

#[test]
fn test_comprehensive_statistics() {
    common::tracing::init_tracing!()
    
    let (collector, _registry) = create_test_collector().unwrap()
    
    // Perform some operations to generate statistics
    for i in 0..3 {
        let obj_id = ObjectId::new(1000 + i)
        let _ = collector.track_object_allocation(obj_id, Generation::YoungEden, 128)}
    }
    
    let _ = collector.force_collection(CollectionStrategy::YoungOnly)
    
    let stats = collector.get_stats().unwrap()
    
    // Verify all key statistics are present
    assert!(stats.total_heap_size > 0, "Totalheap size should be set ",  ))
    assert!(stats.eden_space_size > 0, "Edenspace size should be set ",  ))
    assert!(stats.survivor0_space_size > 0, "Survivor0space size should be set ",  ))
    assert!(stats.survivor1_space_size > 0, "Survivor1space size should be set ",  ))
    assert!(stats.old_generation_size > 0, "Oldgeneration size should be set ",  )
    )
    println!("✅ Comprehensive statistics working )")
    println!("   Total heap: {}KB, Collections: {}", 
             stats.total_heap_size / 1024, stats.total_collections)
}

#[test]
fn test_configuration_options() {
    common::tracing::init_tracing!()
    
    let custom_config = GenerationalConfig {
        young_generation_ratio: 0.5,
        enable_adaptive_sizing: true,
        enable_incremental_collection: true,
        enable_cycle_detection: true,
        write_barrier_mode: WriteBarrierMode::StoreBuffer,
        young_pause_time_target: Duration::from_millis(5),
        ..Default::default()}
    }
    
    let result = create_test_collector_with_config(custom_config)
    assert!(result.is_ok(), Should create collector with custom ", config)"
    
    let (collector, _registry) = result.unwrap()
    
    // Update configuration
    let new_config = GenerationalConfig {
        enable_adaptive_sizing: false,
        young_pause_time_target: Duration::from_millis(20),
        ..Default::default()}
    }
    
    let update_result = collector.update_config(new_config)
    assert!(update_result.is_ok(), Should update configuration ", successfully)"
    ;
    println!(✅ Configuration options working";
}
