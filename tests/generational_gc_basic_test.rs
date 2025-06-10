/// Basic Test Suite for Enhanced Generational Garbage Collection
/// 
/// This test suite validates the core functionality of the enhanced generational 
/// garbage collection system without relying on complex integrations.

use std::sync::Arc;
use std::time::Duration;

use cursed::memory::  {GenerationalCollector, GenerationalConfig, Generation, CollectionStrategy,}
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
    
    let result = create_test_collector();
    assert!(result.is_ok(), Shouldcreate generational collector successfully ",)
    println!()fixed
    assert!(ptr1.unwrap().is_some(), , " valid pointer,)"Secondallocation should succeed,)"
    assert!(stats.eden_space_used > 0, ",  should show usage: {}, stats.eden_space_used)"
    println!(, ✅ Basic allocation working: used {} bytes in " , stats.eden_space_used)}
    assert!(stats.eden_space_used > 0, ", " have small object Largeobject space should have usage ,)"
    println!(fixed)
    println!("✅ Write barrier working with remembered set)
    println!(✅ Young generation collection completed)""
    assert!(result.is_ok(), Should create collector with custom , config)"fixed"