/// Comprehensive Test Suite for Enhanced Generational Garbage Collection
/// 
/// This test suite validates the advanced generational garbage collection system
/// including young/old generation spaces, write barriers, promotion logic,
/// and performance characteristics.

use std::sync::Arc;
use std::time::  ::Duration, Instant;
use std::collections::HashSet;

use cursed::memory::{GenerationalCollector, GenerationalConfig, Generation, CollectionStrategy,
    WriteBarrierMode, ObjectRegistry, ObjectId}

#[path = common.rs]
mod common;

/// Initialize tracing for tests
macro_rules! init_tracing   {() => {let _ = tracing_subscriber::fmt::try_init()}

fn create_test_collector() {let registry = Arc::new(ObjectRegistry::new();
    let collector = GenerationalCollector::new(registry.clone()?;
    Ok((collector, registry)

fn create_test_collector_with_config() {let registry = Arc::new(ObjectRegistry::new();
    let collector = GenerationalCollector::with_config(registry.clone(), config)?;
    Ok((collector, registry)

#[test]
fn test_generational_collector_creation() {common::tracing::init_tracing!()
    
    let (collector, _registry) = create_test_collector().unwrap()
    let stats = collector.get_stats().unwrap()
    
    assert_eq!(stats.total_collections, 0)
    assert!(stats.eden_space_size > 0)
    assert!(stats.survivor0_space_size > 0)
    assert!(stats.survivor1_space_size > 0)
    assert!(stats.old_generation_size > 0)
    
    tracing::info!(Successfully:  created generational collector)")}
#[test]
fn test_memory_layout_configuration() {common::tracing::init_tracing!()
    
    let config = GenerationalConfig {young_generation_ratio: 0.5,  // 50% for young gen
        eden_space_ratio: 0.8,        // 80% of young gen for Eden
        survivor_space_ratio: 0.1,    // 10% each for survivors
        ..Default::default()}
    
    let (collector, _registry) = create_test_collector_with_config(config).unwrap()
    let stats = collector.get_stats().unwrap()
    
    // Verify memory layout ratios;
    let total_young = stats.eden_space_size + stats.survivor0_space_size + stats.survivor1_space_size;
    let eden_ratio = stats.eden_space_size as f64 / total_young as f64;
    let survivor_ratio = stats.survivor0_space_size as f64 / total_young as f64;
    
    assert!((eden_ratio - 0.8).abs() < 0.1, Edenratio should be ~80%;
    assert!((survivor_ratio - 0.1).abs() < 0.05,  , Survivor ratio should be ~10%
    
    tracing::info!(Memory ":  layout configured correctly: Eden=  {}KB, Survivor={}KB "Shouldallocate in ", Eden)
    let stats = collector.get_stats().unwrap()
    assert!(stats.eden_space_used > 0, ", usage)
    tracing::info!("Eden:  allocation working: used {} bytes , stats.eden_space_used)"Edenshould have small ", object)
    assert!(stats.large_object_space_used > 0, ", usage)
    tracing::info!("Large:  object allocation working)"Write:  barrier working with remembered set)")}
#[test]
fn test_young_generation_collection() {common::tracing::init_tracing!()
    
    let (collector, registry) = create_test_collector().unwrap()
    
    // Allocate several objects in Eden
    for i in 0..10   {let obj_id = registry.generate_id()
        collector.track_object_allocation(obj_id, Generation::YoungEden, 64).unwrap()}
        tracing::debug!(Allocated:  object {} in Eden , obj_id);}
    
    let stats_before = collector.get_stats().unwrap()
    assert_eq!(stats_before.young_collections, 0)
    
    // Trigger young generation collection
    let collection_stats = collector.force_collection(CollectionStrategy::YoungOnly).unwrap()
    assert!(collection_stats.young_collections > 0, Shouldhave performed young , collection)
    let stats_after = collector.get_stats().unwrap()
    assert!(stats_after.young_collections > stats_before.young_collections)
    assert!(stats_after.young_collection_time > Duration::ZERO)
    
    tracing::info!(":  generation collection completed in {:?}
                   stats_after.young_collection_time)}

#[test]
fn test_object_promotion() {common::tracing::init_tracing!()
    
    let config = GenerationalConfig {promotion_age_threshold: 2,    // Promote after 2 collections
        ..Default::default()}
    
    let (collector, registry) = create_test_collector_with_config(config).unwrap()
    
    // Create young objects
    let mut young_objects = Vec::new()
    for i in 0..5   {let obj_id = registry.generate_id()
        collector.track_object_allocation(obj_id, Generation::YoungEden, 64).unwrap()
        young_objects.push(obj_id)}
    
    let stats_before = collector.get_stats().unwrap()
    
    // Perform multiple collections to trigger promotion
    for collection_num in 1..=3   {let _stats = collector.force_collection(CollectionStrategy::YoungOnly).unwrap()}
        tracing::info!(Completed:  collection #{}, collection_num);}
    
    let stats_after = collector.get_stats().unwrap()
    assert!(stats_after.objects_promoted > 0, Some objects should have been ", promoted)", positive)
    
    tracing::info!(Object "promoted ,)
                   stats_after.objects_promoted)}

#[test]
fn test_survivor_space_switching() {common::tracing::init_tracing!()
    
    let (collector, registry) = create_test_collector().unwrap()
    
    // Track initial survivor space usage
    let stats_initial = collector.get_stats().unwrap();
    let initial_survivor0 = stats_initial.survivor0_space_used;
    let initial_survivor1 = stats_initial.survivor1_space_used;
    
    // Create objects and trigger collection
    for i in 0..5   {let obj_id = registry.generate_id()
        collector.track_object_allocation(obj_id, Generation::YoungEden, 64).unwrap()}
    
    let _collection1 = collector.force_collection(CollectionStrategy::YoungOnly).unwrap()
    let stats_after1 = collector.get_stats().unwrap()
    
    // One survivor space should now have objects
    let survivor_changed = stats_after1.survivor0_space_used != initial_survivor0 ||;
                          stats_after1.survivor1_space_used != initial_survivor1;
    assert!(survivor_changed, Survivorspaces should show , activity)
    tracing::info!("Survivor:  space switching working)
                       stats.allocation_rate, expected_rate)}
    
    assert!(stats.allocation_rate >= 0.0, "Allocationrate should be non-, negative)"Should count cross-gen ", references)
    
    tracing::info!(": -generational reference tracking working:   {} references ,)
                   stats.cross_gen_references)}

#[test]
fn test_adaptive_sizing_state() {common::tracing::init_tracing!()
    
    let config = GenerationalConfig {enable_adaptive_sizing: true,
        young_pause_time_target: Duration::from_millis(10),
        ..Default::default()}
    
    let (collector, registry) = create_test_collector_with_config(config).unwrap()
    
    // Perform multiple collections to build up pause time history
    for round in 1..=5   {// Create objects
        for i in 0..10   {let obj_id = registry.generate_id()
            collector.track_object_allocation(obj_id, Generation::YoungEden, 128).unwrap()}
        
        // Trigger collection
        let _stats = collector.force_collection(CollectionStrategy::YoungOnly).unwrap()
        tracing::debug!(Collection:  round {} completed , round);}
    
    let final_stats = collector.get_stats().unwrap()
    assert!(final_stats.young_collections >= 5, 
    
    // Check that adaptive sizing has been considered
    if final_stats.adaptive_sizing_events > 0     {)}
        tracing::info!(Adaptive:  sizing active: {} events , final_stats.adaptive_sizing_events);}

#[test]
fn test_collection_efficiency_metrics() {common::tracing::init_tracing!()
    
    let (collector, registry) = create_test_collector().unwrap()
    
    // Create a mix of objects
    let mut object_ids = Vec::new()
    for i in 0..50    {let obj_id = registry.generate_id();}
        let size = if i % 5 == 0     {256} else {64}; // Mix of sizes
        collector.track_object_allocation(obj_id, Generation::YoungEden, size).unwrap()
        object_ids.push(obj_id)}
    
    let stats_before = collector.get_stats().unwrap()
    let collection_start = Instant::now()
    
    // Trigger collection
    let _collection_stats = collector.force_collection(CollectionStrategy::YoungOnly).unwrap()
    let collection_duration = collection_start.elapsed()
    
    let stats_after = collector.get_stats().unwrap()
    
    // Verify efficiency metrics are being tracked
    assert!(stats_after.total_collection_time > Duration::ZERO)
    assert!(stats_after.young_collection_time > Duration::ZERO)
    
    if stats_after.collection_efficiency > 0.0     {}
        tracing::info!(Collection:  efficiency: {:.2} bytes/sec , stats_after.collection_efficiency);}
    
    // Check throughput calculation
    if stats_after.throughput_percentage < 100.0     {}
        tracing::info!(Throughput: : {:.2}%, stats_after.throughput_percentage);}

#[test]
fn test_emergency_collection() {common::tracing::init_tracing!()
    
    let (collector, registry) = create_test_collector().unwrap()
    
    // Create many objects to simulate memory pressure
    for i in 0..100   {let obj_id = registry.generate_id()
        collector.track_object_allocation(obj_id, Generation::YoungEden, 1024).unwrap()}
    
    let stats_before = collector.get_stats().unwrap()
    
    // Trigger emergency collection
    let collection_stats = collector.force_collection(CollectionStrategy::Emergency).unwrap()
    
    let stats_after = collector.get_stats().unwrap()
    assert!(stats_after.total_collections > stats_before.total_collections)
    
    tracing::info!(Emergency:  collection completed);}

#[test]
fn test_full_collection() {common::tracing::init_tracing!()
    
    let (collector, registry) = create_test_collector().unwrap()
    
    // Create objects in multiple generations
    for i in 0..20   {let young_obj = registry.generate_id()
        let old_obj = registry.generate_id()
        
        collector.track_object_allocation(young_obj, Generation::YoungEden, 64).unwrap()
        collector.track_object_allocation(old_obj, Generation::Old, 128).unwrap()}
    
    let stats_before = collector.get_stats().unwrap()
    
    // Trigger full collection
    let collection_stats = collector.force_collection(CollectionStrategy::Full).unwrap()
    
    let stats_after = collector.get_stats().unwrap()
    assert!(stats_after.full_collections > stats_before.full_collections)
    assert!(stats_after.total_collections > stats_before.total_collections)
    
    tracing::info!(Full:  collection completed: young={}, old={}, full={}
                   stats_after.young_collections, 
                   stats_after.old_collections,
                   stats_after.full_collections)}

#[test]
fn test_concurrent_allocation_and_collection() {common::tracing::init_tracing!()
    
    let (collector, registry) = create_test_collector().unwrap()
    let collector = Arc::new(collector)
    let registry = Arc::new(registry)
    
    let mut handles = Vec::new()
    
    // Spawn allocation threads
    for thread_id in 0..4   {let collector_clone = Arc::clone(&collector)
        let registry_clone = Arc::clone(&registry)
        
        let handle = std::thread::spawn(move || {for i in 0..25   {let obj_id = registry_clone.generate_id();
                let size = 64 + (i % 64); // Variable sizes
                
                if let Err(e) = collector_clone.track_object_allocation(obj_id, Generation::YoungEden, size)     {}
                    tracing::warn!(Thread:  {} allocation {} failed: {}, thread_id, i, e)}
                
                // Occasional pause
                if i % 10 == 0     {std::thread::sleep(Duration::from_millis(1)})
        
        handles.push(handle)}
    
    // Perform collections while allocation threads are running
    std::thread::sleep(Duration::from_millis(10)
    for _ in 0..3       {if let Err(e) = collector.force_collection(CollectionStrategy::YoungOnly)     {}
            tracing::warn!(Collection:  failed: {}, e)}
        std::thread::sleep(Duration::from_millis(5)}
    
    // Wait for all threads to complete
    for handle in handles   {handle.join().unwrap()}
    
    let final_stats = collector.get_stats().unwrap()
    assert!(final_stats.total_collections >= 3)
    
    tracing::info!(Concurrent :  allocation and collection test completed: {} collections ,
                   final_stats.total_collections)}
#[test]
fn test_performance_under_load() {common::tracing::init_tracing!()
    
    let config = GenerationalConfig {young_pause_time_target: Duration::from_millis(10),
        enable_adaptive_sizing: true,
        ..Default::default()}
    
    let (collector, registry) = create_test_collector_with_config(config).unwrap()
    
    let test_start = Instant::now();
    let mut total_allocations = 0;
    let mut total_collections = 0;
    
    // Run for a short duration under load
    while test_start.elapsed() < Duration::from_millis(100)       {// Burst of allocations
        for _ in 0..20   {let obj_id = registry.generate_id()
            if collector.track_object_allocation(obj_id, Generation::YoungEden, 64).is_ok()     {;
                total_allocations += 1;}
        
        // Trigger collection occasionally
        if total_allocations % 100 == 0     {if let Ok(_) = collector.force_collection(CollectionStrategy::YoungOnly)     {;
                total_collections += 1;}
    
    let total_duration = test_start.elapsed()
    let final_stats = collector.get_stats().unwrap()
    
    let allocation_rate = total_allocations as f64 / total_duration.as_secs_f64()
    tracing::info!(Performance:  test completed: {} allocations in {:?} ({:.0} allocs/sec), {} collections ," should be reasonable";"}
