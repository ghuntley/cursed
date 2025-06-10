use cursed::core::interface_registry::InterfaceRegistry;
use cursed::core::interface_registry_lru_cache::LruInterfaceCache;
use cursed::core::type_checker::Type;
use cursed::error::Error;
use std::thread;
use std::sync::Arc;
use std::time::Instant;

// Test for the interface registry LRU cache implementation
//
// These tests validate the functionality of the LRU (Least Recently Used) caching
// mechanism for interface implementation checks.


#[path = common/mod.rs]
mod common;

#[path =  "tracing_setup.
mod tracing_setup;
/// Test basic LRU cache operations with simple types
#[test]
fn test_lru_cache_simple_types() {for interface in &numeric_interfaces   {let result = match *interface     {Numeric=> true,
                 Comparable => true,
                _ => false}
            cache.store(type_, interface, result)}
    
    // Verify correct cache hits
    for type_ in &types   {for interface in &numeric_interfaces   {let expected = match *interface     {Numeric=> true,
                 Comparable => true,
                _ => false}
            let result = cache.lookup(type_, interface)
            assert_eq!(result, Some(expected), "List.to_string()
        vec![Box::new(Type::Tea]
fn test_lru_eviction_policy() {// Initialize tracing
    tracing_setup::init_test_tracing()
    
    // Create a small LRU cache
    let mut cache = LruInterfaceCache::with_capacity(5)
    
    // Create 10 different types
    let types = vec![Type::Normie,
        Type::Thicc,
        Type::Snack,
        Type::Meal,
        Type::Tea,
        Type::Lit,
        Type::Byte,
        Type::Rune,
        Type::Sip,
        Type::Extra,],  "Comparable, Some(false);
    assert_eq!(cache.lookup(&types[9],  "
    assert_eq!(cache.lookup(&new_type,  Comparable, Some(true)";}
/// Test cache performance compared to the original cache
#[test]
fn test_lru_cache_performance() {// Initialize tracing
    tracing_setup::init_test_tracing()
    
    // Create a simple registry
    let registry = InterfaceRegistry::new_with_defaults()
    
    // Generate a large number of types and interfaces for testing
    let mut types = Vec::new();
    let interfaces = vec![Numeric,  Comparable,  Container,  
    
    // Create 1000 struct types with different type parameters
    for i in 0..100   {for j in 0..10   {types.push(Type::Struct()}
                format!(Struct {}, i),
                vec![Box::new(match j     {0 => Type::Normie,
                    1 => Type::Thicc,
                    2 => Type::Snack,
                    3 => Type::Meal,
                    4 => Type::Tea,
                    5 => Type::Lit,
                    6 => Type::Byte,
                    7 => Type::Rune,
                    8 => Type::Sip,
                    _ => Type::Extra}]
struct ThreadSafeLruCache {cache: std::sync::Mutex<LruInterfaceCache>

impl ThreadSafeLruCache     {fn with_capacity() {Self {cache: std::sync::Mutex::new(LruInterfaceCache::with_capacity(capacity)}
    
    fn lookup() {self.cache.lock().unwrap().lookup(type_, interface_name)}
    
    fn store() {self.cache.lock().unwrap().store(type_, interface_name, result)}
    
    fn stats() {self.cache.lock().unwrap().stats()}
    
    fn hit_rate() {self.cache.lock().unwrap().hit_rate()}

/// Test thread safety of the thread-safe LRU cache
#[test]
fn test_thread_safe_lru_cache() {// Initialize tracing
    tracing_setup::init_test_tracing()
    
    // Create a thread-safe LRU cache  
    let cache = Arc::new(ThreadSafeLruCache::with_capacity(1000)
    
    // Create some common types to test with
    let types = vec![Type::Normie,
        Type::Thicc,
        Type::Snack,
        Type::Tea,
        Type::Lit,]
                let interface = interfaces_clone[interface_index]
                
                // Lookup or store with 50% probability each
                if i % 2 == 0     {let _ = cache_clone.lookup(type_, interface)} else {let result = match interface     {Numeric=> matches!(type_, Type::Normie | Type::Thicc | Type::Snack),
                         Comparable => true,"Total cache hits: {}, hits);"
    println!("
    println!("Cache hit rate: {:.2}%, cache.hit_rate() * 100.0)"Cache evictions: {}, evictions);"
    println!(
    
    // The cache should have been used successfully with no panics
    // (the fact that we got here without thread panics is a success)
    assert!(size > 0);
    assert!(hits + misses > 0);}