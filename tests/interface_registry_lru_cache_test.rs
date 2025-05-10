//! Test for the interface registry LRU cache implementation
//!
//! These tests validate the functionality of the LRU (Least Recently Used) caching
//! mechanism for interface implementation checks.

use cursed::core::interface_registry::InterfaceRegistry;
use cursed::core::interface_registry_lru_cache::{LruInterfaceCache, ThreadSafeLruCache};
use cursed::core::type_checker::Type;
use cursed::error::Error;
use std::thread;
use std::sync::Arc;
use std::time::Instant;

mod common;

#[path = "tracing_setup.rs"]
mod tracing_setup;

/// Test basic LRU cache operations with simple types
#[test]
fn test_lru_cache_simple_types() {
    // Initialize tracing
    tracing_setup::init_test_tracing();
    
    // Create a new LRU cache
    let mut cache = LruInterfaceCache::new();
    
    // Test with simple types
    let numeric_interfaces = vec!["Numeric", "Comparable", "Printable"];
    let types = vec![Type::Normie, Type::Thicc, Type::Snack, Type::Meal];
    
    // Store some cache entries
    for type_ in &types {
        for interface in &numeric_interfaces {
            let result = match *interface {
                "Numeric" => true,
                "Comparable" => true,
                _ => false,
            };
            cache.store(type_, interface, result);
        }
    }
    
    // Verify correct cache hits
    for type_ in &types {
        for interface in &numeric_interfaces {
            let expected = match *interface {
                "Numeric" => true,
                "Comparable" => true,
                _ => false,
            };
            let result = cache.lookup(type_, interface);
            assert_eq!(result, Some(expected), "{:?} implements {}", type_, interface);
        }
    }
    
    // Check cache statistics
    let (size, hits, misses, evictions, updates) = cache.stats();
    assert_eq!(size, types.len() * numeric_interfaces.len());
    assert_eq!(hits, types.len() * numeric_interfaces.len());
    assert_eq!(misses, 0);
    assert_eq!(evictions, 0);
    assert_eq!(updates, 0);
}

/// Test LRU cache with complex generic types
#[test]
fn test_lru_cache_generic_types() {
    // Initialize tracing
    tracing_setup::init_test_tracing();
    
    // Create a new LRU cache
    let mut cache = LruInterfaceCache::new();
    
    // Create some generic type instances
    let stack_string = Type::Struct(
        "Stack".to_string(),
        vec![Box::new(Type::Tea)]
    );
    
    let stack_int = Type::Struct(
        "Stack".to_string(),
        vec![Box::new(Type::Normie)]
    );
    
    let list_string = Type::Struct(
        "List".to_string(),
        vec![Box::new(Type::Tea)]
    );
    
    let map_string_int = Type::Struct(
        "Map".to_string(),
        vec![Box::new(Type::Tea), Box::new(Type::Normie)]
    );
    
    // Store cache entries for generic types
    cache.store(&stack_string, "Container", true);
    cache.store(&stack_int, "Container", true);
    cache.store(&list_string, "Container", true);
    cache.store(&map_string_int, "Container", true);
    
    // Store additional entries for some types
    cache.store(&list_string, "List", true);
    cache.store(&map_string_int, "Map", true);
    
    // Verify correct cache hits
    assert_eq!(cache.lookup(&stack_string, "Container"), Some(true));
    assert_eq!(cache.lookup(&stack_int, "Container"), Some(true));
    assert_eq!(cache.lookup(&list_string, "Container"), Some(true));
    assert_eq!(cache.lookup(&map_string_int, "Container"), Some(true));
    assert_eq!(cache.lookup(&list_string, "List"), Some(true));
    assert_eq!(cache.lookup(&map_string_int, "Map"), Some(true));
    
    // These should be cache misses
    assert_eq!(cache.lookup(&stack_string, "List"), None);
    assert_eq!(cache.lookup(&stack_int, "Map"), None);
    
    // Check cache statistics
    let (size, hits, misses, _, _) = cache.stats();
    assert_eq!(size, 6);
    assert_eq!(hits, 6);
    assert_eq!(misses, 2);
}

/// Test LRU eviction policy
#[test]
fn test_lru_eviction_policy() {
    // Initialize tracing
    tracing_setup::init_test_tracing();
    
    // Create a small LRU cache
    let mut cache = LruInterfaceCache::with_capacity(5);
    
    // Create 10 different types
    let types = vec![
        Type::Normie,
        Type::Thicc,
        Type::Snack,
        Type::Meal,
        Type::Tea,
        Type::Lit,
        Type::Byte,
        Type::Rune,
        Type::Sip,
        Type::Extra,
    ];
    
    // Store cache entries for all types
    for (i, type_) in types.iter().enumerate() {
        cache.store(type_, "Comparable", i % 2 == 0);
    }
    
    // The cache should have only kept the 5 most recent entries
    let (size, _, _, evictions, _) = cache.stats();
    assert_eq!(size, 5);
    assert_eq!(evictions, 5);
    
    // The first 5 entries should have been evicted
    for i in 0..5 {
        assert_eq!(cache.lookup(&types[i], "Comparable"), None, "Type at index {} should have been evicted", i);
    }
    
    // The last 5 entries should still be in the cache
    for i in 5..10 {
        assert_eq!(cache.lookup(&types[i], "Comparable"), Some(i % 2 == 0), "Type at index {} should be in cache", i);
    }
    
    // Now access the first remaining item (index 5) to make it most recently used
    cache.lookup(&types[5], "Comparable");
    
    // Add one more entry, which should evict the entry at index 6 (least recently used)
    let new_type = Type::Struct("NewStruct".to_string(), vec![]);
    cache.store(&new_type, "Comparable", true);
    
    // Check that index 6 was evicted
    assert_eq!(cache.lookup(&types[6], "Comparable"), None, "Type at index 6 should have been evicted");
    
    // Other entries should still be in the cache
    assert_eq!(cache.lookup(&types[5], "Comparable"), Some(true));
    assert_eq!(cache.lookup(&types[7], "Comparable"), Some(true));
    assert_eq!(cache.lookup(&types[8], "Comparable"), Some(false));
    assert_eq!(cache.lookup(&types[9], "Comparable"), Some(false));
    assert_eq!(cache.lookup(&new_type, "Comparable"), Some(true));
}

/// Test cache performance compared to the original cache
#[test]
fn test_lru_cache_performance() {
    // Initialize tracing
    tracing_setup::init_test_tracing();
    
    // Create a simple registry
    let registry = InterfaceRegistry::new_with_defaults();
    
    // Generate a large number of types and interfaces for testing
    let mut types = Vec::new();
    let interfaces = vec!["Numeric", "Comparable", "Container", "List", "Map"];
    
    // Create 1000 struct types with different type parameters
    for i in 0..100 {
        for j in 0..10 {
            types.push(Type::Struct(
                format!("Struct{}", i),
                vec![Box::new(match j {
                    0 => Type::Normie,
                    1 => Type::Thicc,
                    2 => Type::Snack,
                    3 => Type::Meal,
                    4 => Type::Tea,
                    5 => Type::Lit,
                    6 => Type::Byte,
                    7 => Type::Rune,
                    8 => Type::Sip,
                    _ => Type::Extra,
                })]
            ));
        }
    }
    
    // First measure without cache
    let start_time = Instant::now();
    
    // Perform 5000 lookups without cache
    let mut count = 0;
    for _ in 0..5 {
        for type_ in &types {
            for interface in &interfaces {
                let _ = registry.check_implementation(type_, interface);
                count += 1;
            }
        }
    }
    
    let no_cache_time = start_time.elapsed();
    
    // Now measure with LRU cache
    let mut lru_cache = LruInterfaceCache::with_capacity(1000);
    let start_time = Instant::now();
    
    // Perform the same lookups with cache
    for _ in 0..5 {
        for type_ in &types {
            for interface in &interfaces {
                // First check cache
                if let Some(result) = lru_cache.lookup(type_, interface) {
                    // Use the cached result
                } else {
                    // Cache miss, perform the check and store result
                    let result = registry.check_implementation(type_, interface).unwrap();
                    lru_cache.store(type_, interface, result);
                }
            }
        }
    }
    
    let lru_cache_time = start_time.elapsed();
    
    println!("Performed {} lookups", count);
    println!("Time without cache: {:?}", no_cache_time);
    println!("Time with LRU cache: {:?}", lru_cache_time);
    println!("Speedup: {:.2}x", no_cache_time.as_micros() as f64 / lru_cache_time.as_micros() as f64);
    
    // Get cache statistics
    let (size, hits, misses, evictions, updates) = lru_cache.stats();
    println!("Cache size: {}", size);
    println!("Cache hits: {}", hits);
    println!("Cache misses: {}", misses);
    println!("Cache hit rate: {:.2}%", lru_cache.hit_rate() * 100.0);
    println!("Cache evictions: {}", evictions);
    println!("Cache updates: {}", updates);
    
    // The LRU cache should be significantly faster (at least 2x)
    assert!(no_cache_time.as_micros() > lru_cache_time.as_micros() * 2);
    
    // Cache hit rate should be high (over 80%)
    assert!(lru_cache.hit_rate() > 0.8);
}

/// Test thread safety of the thread-safe LRU cache
#[test]
fn test_thread_safe_lru_cache() {
    // Initialize tracing
    tracing_setup::init_test_tracing();
    
    // Create a thread-safe LRU cache
    let cache = Arc::new(ThreadSafeLruCache::with_capacity(1000));
    
    // Create some common types to test with
    let types = vec![
        Type::Normie,
        Type::Thicc,
        Type::Snack,
        Type::Tea,
        Type::Lit,
    ];
    
    let interfaces = vec!["Numeric", "Comparable", "Printable"];
    
    // Spawn multiple threads that all use the cache
    let mut handles = Vec::new();
    
    for thread_id in 0..4 {
        let cache_clone = Arc::clone(&cache);
        let types_clone = types.clone();
        let interfaces_clone = interfaces.clone();
        
        let handle = thread::spawn(move || {
            for i in 0..100 {
                let type_index = (i + thread_id) % types_clone.len();
                let interface_index = (i + thread_id) % interfaces_clone.len();
                
                let type_ = &types_clone[type_index];
                let interface = interfaces_clone[interface_index];
                
                // Lookup or store with 50% probability each
                if i % 2 == 0 {
                    let _ = cache_clone.lookup(type_, interface);
                } else {
                    let result = match interface {
                        "Numeric" => matches!(type_, Type::Normie | Type::Thicc | Type::Snack),
                        "Comparable" => true,
                        _ => false,
                    };
                    cache_clone.store(type_, interface, result);
                }
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Get final cache statistics
    let (size, hits, misses, evictions, updates) = cache.stats();
    println!("Final cache size: {}", size);
    println!("Total cache hits: {}", hits);
    println!("Total cache misses: {}", misses);
    println!("Cache hit rate: {:.2}%", cache.hit_rate() * 100.0);
    println!("Cache evictions: {}", evictions);
    println!("Cache updates: {}", updates);
    
    // The cache should have been used successfully with no panics
    // (the fact that we got here without thread panics is a success)
    assert!(size > 0);
    assert!(hits + misses > 0);
}