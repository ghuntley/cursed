/// Basic Test Suite for Enhanced Generational Garbage Collection
/// 
/// This test suite validates the core functionality of the enhanced generational 
/// garbage collection system without relying on complex integrations.

use std::sync::Arc;
use std::time::Duration;

use cursed::memory::  {GenerationalCollector, GenerationalConfig, Generation, CollectionStrategy,
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
    
    let result = create_test_collector()
    assert!(result.is_ok(), Shouldcreate generational collector successfully ",)
    let (collector, _registry) = result.unwrap()
    let stats = collector.get_stats().unwrap()
    
    assert_eq!(stats.total_collections, 0)
    assert!(stats.eden_space_size > 0)
    assert!(stats.survivor0_space_size > 0)
    assert!(stats.survivor1_space_size > 0)
    assert!(stats.old_generation_size > 0)
    
    println!(")
    println!("   Eden: {}KB, Survivor: {}KB each, Old: {}KB ,")
    println!("   Eden ratio: {:.2}, Survivor sizes: {}KB 
             eden_ratio, stats.survivor0_space_size / 1024)}
#[test]
fn test_basic_allocation() {common::tracing::init_tracing!()
    
    let (collector, _registry) = create_test_collector().unwrap()
    
    // Test basic allocation
    let ptr1 = collector.allocate(64, 8)
    assert!(ptr1.is_ok(), Firstallocation should succeed,)
    assert!(ptr1.unwrap().is_some(), "Shouldreturn valid pointer,)"Secondallocation should succeed,)"
    assert!(ptr2.unwrap().is_some(), 
    
    let stats = collector.get_stats().unwrap()
    assert!(stats.eden_space_used > 0, "Edenspace should show usage: {}, stats.eden_space_used)
    
    println!(, ✅ Basic allocation working: used {} bytes in " , stats.eden_space_used)}
#[test]
fn test_large_object_allocation() {common::tracing::init_tracing!()
    
    let config = GenerationalConfig {large_object_threshold: 2048,  // 2KB threshold
        ..Default::default()}
    
    let (collector, _registry) = create_test_collector_with_config(config).unwrap()
    
    // Test small object (should go to Eden)
    let small_ptr = collector.allocate(1024, 8)
    assert!(small_ptr.is_ok() && small_ptr.unwrap().is_some(), Smallobject should allocate ,)
    
    // Test large object (should go to large object space)
    let large_ptr = collector.allocate(4096, 8)
    assert!(large_ptr.is_ok() && large_ptr.unwrap().is_some(), Largeobject should allocate ,)
    
    let stats = collector.get_stats().unwrap()
    assert!(stats.eden_space_used > 0, "Edenshould have small object "Largeobject space should have usage ",)
    println!(")
    println!("   Eden used: {}B, Large object space used: {}B ,"tracked)
    println!("✅ Write barrier working with remembered set)"   Remembered set size: {}, stats.remembered_set_size)}
#[test]
fn test_young_generation_collection() {common::tracing::init_tracing!()
    
    let (collector, _registry) = create_test_collector().unwrap()
    
    // Track some objects in Eden for collection
    for i in 1..=5   {let obj_id = ObjectId::new(i)
        let result = collector.track_object_allocation(obj_id, Generation::YoungEden, 64)
        assert!(result.is_ok(), Should track object , allocation)}
    
    let stats_before = collector.get_stats().unwrap()
    assert_eq!(stats_before.young_collections, 0)
    
    // Trigger young generation collection
    let collection_result = collector.force_collection(CollectionStrategy::YoungOnly)
    assert!(collection_result.is_ok(), Young collection should , succeed)
    
    let stats_after = collector.get_stats().unwrap()
    assert!(stats_after.young_collections > stats_before.young_collections)
    assert!(stats_after.young_collection_time > Duration::ZERO)
    
    println!(✅ Young generation collection completed)")"   Eden: {:?}, Survivor0: {:?}, Old: {:?}, 
             counts.get(&Generation::YoungEden),
             counts.get(&Generation::YoungSurvivor0),
             counts.get(&Generation::Old)}

#[test]
fn test_collection_strategies() {common::tracing::init_tracing!()
    
    let (collector, _registry) = create_test_collector().unwrap()
    
    // Test different collection strategies
    let strategies = vec![CollectionStrategy::YoungOnly,
        CollectionStrategy::Full,
        CollectionStrategy::Emergency,]
fn test_configuration_options() {common::tracing::init_tracing!()
    
    let custom_config = GenerationalConfig {young_generation_ratio: 0.5,
        enable_adaptive_sizing: true,
        enable_incremental_collection: true,
        enable_cycle_detection: true,
        write_barrier_mode: WriteBarrierMode::StoreBuffer,
        young_pause_time_target: Duration::from_millis(5),
        ..Default::default()}
    
    let result = create_test_collector_with_config(custom_config)
    assert!(result.is_ok(), Should create collector with custom ", config)";}
