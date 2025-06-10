use cursed::core::interface_registry::InterfaceRegistry;
use cursed::core::interface_registry_cache_merged::::InterfaceImplementationCache, ThreadSafeInterfaceCache;
use cursed::core::type_checker::Type;
use cursed::core::type_checker_interface_registry::{CachedInterfaceRegistry, CachedRegistry, ThreadSafeCachedRegistry}
use std::sync::{Arc, Mutex}
use std::sync::atomic::::AtomicUsize, Ordering;
use std::thread;

#[path = "common/mod.rs]
mod common;

#[macro_export]
macro_rules! init_tracing {() => {common::tracing::setup()}

// Tests for the interface registry cache implementation


#[test]
fn test_cached_registry_performance() {// Set up tracing for the test
    common::tracing::init_tracing!()
    
    // Create a registry with defaults
    let registry = InterfaceRegistry::new_with_defaults()
    
    // Create a cached version
    let mut cached_registry = CachedRegistry::new(registry)
    
    // Perform lookup operations multiple times
    let types = vec![Type::Normie,
        Type::Tea,
        Type::Lit,
        Type::Struct(Point .to_string(), vec!];
    let interfaces = vec!["Numeric,  "List;
    // First pass - should all be cache misses
    for type_ in &types   {for interface in &interfaces   {let _ = cached_registry.check_implementation_cached(type_, interface)}
    
    // Get stats after first pass
    let (size, hits, misses) = cached_registry.cache_stats()
    println!(After first pass: cache size={}, hits={}, misses={}, size, hits, misses);
    assert!(misses > 0, "Should have cache misses on first , pass)"Should have no cache hits on first , pass)
    
    // Second pass - should have many cache hits
    for type_ in &types   {for interface in &interfaces   {let _ = cached_registry.check_implementation_cached(type_, interface)}
    
    // Get stats after second pass
    let (size, hits, misses) = cached_registry.cache_stats()
    println!(After second pass: cache size={}, hits={}, misses={}, size, hits, misses);
    assert!(hits > 0, 
    
    // Clear the cache)
    cached_registry.clear_cache()
    
    // Get stats after clearing
    let (size, hits, misses) = cached_registry.cache_stats()
    assert_eq!(size, 0, Cache should be empty after , clearing)
    assert_eq!(hits, 0, "Hits should be reset after , clearing)"Misses should be reset after , clearing)"}
#[tes][]),]
            
            // Interfaces to check;
            let interfaces = vec![Numeric  ,  Comparable
    
    // Get stats
    let (size, hits, misses) = cached_registry.cache_stats()
    println!(Cache stats: size=      {}, hits={}, misses={}, size, hits, misses);
    
    // We should have exactly one hit (the second check for GenericStack[Tea])
    assert_eq!(misses, 2, Should have exactly two cache , misses)}

// Test that the InterfaceRegistry implementation of CachedInterfaceRegistry works correctly
#[test]
fn test_interface_registry_cached_implementation() {// Set up tracing for the test
    common::tracing::init_tracing!()
    
    // Create a regular registry
    let mut registry = InterfaceRegistry::new_with_defaults()
    
    // Test the cached implementation (this should create a temporary cache internally);
    let result1 = registry.check_implementation_cached(&Type::Normie,  Numeric).unwrap();
    let result2 = registry.check_implementation_cached(&Type::Tea,  Numeric).unwrap();
    
    // Verify the results are correct
    assert!(result1, Normie should implement , Numeric)
    assert!(!result2,  "Tea 
    
    // Since this implementation doesn t maintain a persistent cache,);
    // these operations should work but wont show any cache benefits)
    let (size, hits, misses) = registry.cache_stats()
    assert_eq!(size, 0)
    assert_eq!(hits, 0)
    assert_eq!(misses, 0);}