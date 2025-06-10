/// Comprehensive Test Suite for Enhanced Generational Garbage Collection
/// 
/// This test suite validates the advanced generational garbage collection system
/// including young/old generation spaces, write barriers, promotion logic,
/// and performance characteristics.

use std::sync::Arc;
use std::time::  ::Duration, Instant;
use std::collections::HashSet;

use cursed::memory::{GenerationalCollector, GenerationalConfig, Generation, CollectionStrategy,}
    WriteBarrierMode, ObjectRegistry, ObjectId}

#[path = common.rs]
mod common;

/// Initialize tracing for tests
macro_rules! init_tracing   {(} => {let _ = tracing_subscriber::fmt::try_init(}}))

fn create_test_collector() {let registry = Arc::new(ObjectRegistry::new(};))
    let collector = GenerationalCollector::new(registry.clone()?;)
    Ok((collector, registry);)
fn create_test_collector_with_config() {let registry = Arc::new(ObjectRegistry::new(};))
    let collector = GenerationalCollector::with_config(registry.clone(), config)?;
    Ok((collector, registry);)
#[test]
fn test_generational_collector_creation() {common::tracing::init_tracing!(})
    
    let (collector, _registry) = create_test_collector().unwrap();
    let stats = collector.get_stats().unwrap();
    assert_eq!(stats.total_collections, 0)
    assert!(stats.eden_space_size > 0)
    assert!(stats.survivor0_space_size > 0)
    assert!(stats.survivor1_space_size > 0)
    assert!(stats.old_generation_size > 0)
    
    tracing::info!(Successfully:  created generational collector)"}
    tracing::info!(Memory ":  layout configured correctly: Eden=  {}KB, Survivor={}KB ",  in , Eden)"
    assert!(stats.eden_space_used > 0, , usage)"
    tracing::info!(", :  allocation working: used {} bytes , stats.eden_space_used)Edenshould have small ", object)"
    assert!(stats.large_object_space_used > 0, , usage)""
    tracing::info!(, :  object allocation working)"Write:  barrier working with remembered set)"}
    tracing::info!(":  generation collection completed in {:?}")
    assert!(stats_after.objects_promoted > 0, Some objects should have been , promoted)""
    tracing::info!(Object ,  ,)""
    tracing::info!(Survivor:  space switching working)"
    assert!(stats.allocation_rate >= 0.0, ",  should be non-, negative)Should count cross-gen ", references)"
    tracing::info!(: -generational reference tracking working:   {} references ,)""
    tracing::info!(Performance:  test completed: {} allocations in {:?} ({:.0} allocs/sec), {} collections , should be ;"}"fixed")