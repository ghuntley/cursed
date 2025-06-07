use cursed::core::interface_registry::InterfaceRegistry;
use cursed::core::interface_registry_cache_merged::{InterfaceImplementationCache, ThreadSafeInterfaceCache};
use cursed::core::type_checker::Type;
use cursed::core::type_checker_interface_registry::{CachedInterfaceRegistry, CachedRegistry, ThreadSafeCachedRegistry};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

#[path = "common/mod.rs"]
mod common;

#[macro_export]
macro_rules! init_tracing {
    () => {
        common::tracing::setup();
    };
}

// Tests for the interface registry cache implementation


#[test]
fn test_cached_registry_performance() {
    // Set up tracing for the test
    init_tracing!();
    
    // Create a registry with defaults
    let registry = InterfaceRegistry::new_with_defaults();
    
    // Create a cached version
    let mut cached_registry = CachedRegistry::new(registry);
    
    // Perform lookup operations multiple times
    let types = vec![
        Type::Normie,
        Type::Tea,
        Type::Lit,
        Type::Struct("Point".to_string(), vec![]),
        Type::Struct("IntList".to_string(), vec![]),
        Type::Struct("StringStack".to_string(), vec![]),
    ];
    
    let interfaces = vec!["Numeric", "Comparable", "Container", "List"];
    
    // First pass - should all be cache misses
    for type_ in &types {
        for interface in &interfaces {
            let _ = cached_registry.check_implementation_cached(type_, interface);
        }
    }
    
    // Get stats after first pass
    let (size, hits, misses) = cached_registry.cache_stats();
    println!("After first pass: cache size={}, hits={}, misses={}", size, hits, misses);
    assert!(misses > 0, "Should have cache misses on first pass");
    assert_eq!(hits, 0, "Should have no cache hits on first pass");
    
    // Second pass - should have many cache hits
    for type_ in &types {
        for interface in &interfaces {
            let _ = cached_registry.check_implementation_cached(type_, interface);
        }
    }
    
    // Get stats after second pass
    let (size, hits, misses) = cached_registry.cache_stats();
    println!("After second pass: cache size={}, hits={}, misses={}", size, hits, misses);
    assert!(hits > 0, "Should have cache hits on second pass");
    
    // Clear the cache
    cached_registry.clear_cache();
    
    // Get stats after clearing
    let (size, hits, misses) = cached_registry.cache_stats();
    assert_eq!(size, 0, "Cache should be empty after clearing");
    assert_eq!(hits, 0, "Hits should be reset after clearing");
    assert_eq!(misses, 0, "Misses should be reset after clearing");
}

#[test]
fn test_thread_safe_cached_registry() {
    // Set up tracing for the test
    init_tracing!();
    
    // Create a thread-safe cached registry
    let registry = InterfaceRegistry::new_with_defaults();
    let cached_registry = Arc::new(ThreadSafeCachedRegistry::new(registry));
    
    // Create a counter for successes
    let successful_checks = Arc::new(AtomicUsize::new(0));
    
    // Spawn multiple threads to perform lookups concurrently
    let mut handles = vec![];
    let num_threads = 4;
    for _ in 0..num_threads {
        let registry_clone = cached_registry.clone();
        let success_counter = successful_checks.clone();
        
        let handle = thread::spawn(move || {
            // Types to check
            let types = vec![
                Type::Normie,
                Type::Tea,
                Type::Struct("Point".to_string(), vec![]),
            ];
            
            // Interfaces to check
            let interfaces = vec!["Numeric", "Comparable"];
            
            // Perform lookups
            for _ in 0..10 { // 10 iterations per thread
                for type_ in &types {
                    for interface in &interfaces {
                        let result = registry_clone.check_implementation(type_, interface).unwrap();
                        if result {
                            success_counter.fetch_add(1, Ordering::SeqCst);
                        }
                    }
                }
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Get the number of successful checks
    let total_successes = successful_checks.load(Ordering::SeqCst);
    println!("Total successful interface implementation checks: {}", total_successes);
    
    // Get cache statistics
    let (size, hits, misses) = cached_registry.cache_stats();
    println!("Cache stats: size={}, hits={}, misses={}", size, hits, misses);
    
    // We should have some cache hits
    assert!(hits > 0, "Should have cache hits after concurrent access");
    
    // Calculate hit rate
    let hit_rate = cached_registry.cache_hit_rate();
    println!("Cache hit rate: {:.2}%", hit_rate * 100.0);
}

#[test]
fn test_complex_type_caching() {
    // Set up tracing for the test
    init_tracing!();
    
    // Create registries
    let registry = InterfaceRegistry::new_with_defaults();
    let mut cached_registry = CachedRegistry::new(registry);
    
    // Create a complex generic type
    let generic_stack_tea = Type::Struct(
        "GenericStack".to_string(),
        vec![Box::new(Type::Tea)]
    );
    
    // Check if it implements Container interface
    let _ = cached_registry.check_implementation_cached(&generic_stack_tea, "Container");
    
    // Same check again should hit the cache
    let _ = cached_registry.check_implementation_cached(&generic_stack_tea, "Container");
    
    // Different type argument should be a miss
    let generic_stack_int = Type::Struct(
        "GenericStack".to_string(),
        vec![Box::new(Type::Normie)]
    );
    let _ = cached_registry.check_implementation_cached(&generic_stack_int, "Container");
    
    // Get stats
    let (size, hits, misses) = cached_registry.cache_stats();
    println!("Cache stats: size={}, hits={}, misses={}", size, hits, misses);
    
    // We should have exactly one hit (the second check for GenericStack[Tea])
    assert_eq!(hits, 1, "Should have exactly one cache hit");
    
    // We should have exactly two misses (the first check for GenericStack[Tea] and the check for GenericStack[Normie])
    assert_eq!(misses, 2, "Should have exactly two cache misses");
}

// Test that the InterfaceRegistry implementation of CachedInterfaceRegistry works correctly
#[test]
fn test_interface_registry_cached_implementation() {
    // Set up tracing for the test
    init_tracing!();
    
    // Create a regular registry
    let mut registry = InterfaceRegistry::new_with_defaults();
    
    // Test the cached implementation (this should create a temporary cache internally)
    let result1 = registry.check_implementation_cached(&Type::Normie, "Numeric").unwrap();
    let result2 = registry.check_implementation_cached(&Type::Tea, "Numeric").unwrap();
    
    // Verify the results are correct
    assert!(result1, "Normie should implement Numeric");
    assert!(!result2, "Tea should not implement Numeric");
    
    // Since this implementation doesn't maintain a persistent cache,
    // these operations should work but won't show any cache benefits
    let (size, hits, misses) = registry.cache_stats();
    assert_eq!(size, 0);
    assert_eq!(hits, 0);
    assert_eq!(misses, 0);
}