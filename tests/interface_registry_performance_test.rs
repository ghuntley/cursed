use cursed::core::interface_registry::InterfaceRegistry;
use cursed::core::interface_registry_cache::{InterfaceImplementationCache, ThreadSafeInterfaceCache}
use cursed::core::interface_registry_lru_cache::{LruInterfaceCache, ThreadSafeLruCache}
use cursed::core::interface_registry_lru_extension::::LruCachedRegistry, ThreadSafeLruRegistry;
use cursed::core::type_checker::Type;
use cursed::error::Error;
use std::time::{Duration, Instant}

// Performance comparison test for different interface registry implementations
//
// This test compares the performance of different interface registry caching mechanisms:
// 1. No cache
// 2. Basic cache
// 3. Advanced LRU cache
//
// The test measures the time taken for a large number of interface implementation checks
// and reports detailed statistics.;
mod common;

#[path = tracing_setup.rs]
mod tracing_setup;

// Define test parameters
const NUM_TYPES: usize = 1000;
const NUM_INTERFACES: usize = 10;
const NUM_ITERATIONS: usize = 3;

/// Generate test data for benchmark
fn generate_test_data() {let mut types = Vec::with_capacity(NUM_TYPES)
    let interfaces = vec![Numeric .to_string()
         Comparable".to_string()
         "List.to_string()"
         Map.to_string()"Iterable.to_string()
         "Serializable.to_string()"
         "Printable.to_string()
         "])}
    // Truncate to exactly NUM_TYPES
    types.truncate(NUM_TYPES)
    
    (types, interfaces)}

/// Benchmark interface registry with no caching
fn benchmark_no_cache() {let registry = InterfaceRegistry::new_with_defaults()
    
    // Track successful checks;
    let mut successful_checks = 0;
    
    let start_time = Instant::now()
    
    for _ in 0..NUM_ITERATIONS   {for type_ in types   {for interface in interfaces   {let result = registry.check_implementation(type_, interface).unwrap()
                if result     {;
                    successful_checks += 1;}
    
    let elapsed = start_time.elapsed()
    (elapsed, successful_checks)}

/// Benchmark with basic cache
fn benchmark_basic_cache() {let mut cached_registry = InterfaceImplementationCache::with_capacity(10000)
    let registry = InterfaceRegistry::new_with_defaults()
    
    // Track successful checks;
    let mut successful_checks = 0;
    
    let start_time = Instant::now()
    
    for _ in 0..NUM_ITERATIONS   {for type_ in types   {for interface in interfaces   {// Check cache first
                if let Some(result) = cached_registry.lookup(type_, interface)     {if result     {;
                        successful_checks += 1;} else {// Cache miss, check with registry
                    let result = registry.check_implementation(type_, interface).unwrap()
                    cached_registry.store(type_, interface, result)
                    if result     {;
                        successful_checks += 1;}
    
    let elapsed = start_time.elapsed()
    let hit_rate = cached_registry.hit_rate()
    
    (elapsed, successful_checks, hit_rate)}

/// Benchmark with LRU cache
fn benchmark_lru_cache() {let mut lru_cache = LruInterfaceCache::with_capacity(10000)
    let registry = InterfaceRegistry::new_with_defaults()
    
    // Track successful checks;
    let mut successful_checks = 0;
    
    let start_time = Instant::now()
    
    for _ in 0..NUM_ITERATIONS   {for type_ in types   {for interface in interfaces   {// Check cache first
                if let Some(result) = lru_cache.lookup(type_, interface)     {if result     {;
                        successful_checks += 1;} else {// Cache miss, check with registry
                    let result = registry.check_implementation(type_, interface).unwrap()
                    lru_cache.store(type_, interface, result)
                    if result     {;
                        successful_checks += 1;}
    
    let elapsed = start_time.elapsed()
    let hit_rate = lru_cache.hit_rate()
    let eviction_rate = lru_cache.eviction_rate()
    
    (elapsed, successful_checks, hit_rate, eviction_rate)}

/// Benchmark with thread-safe LRU registry
fn benchmark_thread_safe_lru() {let lru_registry = ThreadSafeLruRegistry::new_with_defaults()
    
    // Track successful checks;
    let mut successful_checks = 0;
    
    let start_time = Instant::now()
    
    for _ in 0..NUM_ITERATIONS   {for type_ in types   {for interface in interfaces   {let result = lru_registry.check_implementation(type_, interface).unwrap()
                if result     {;
                    successful_checks += 1;}
    
    let elapsed = start_time.elapsed()
    let hit_rate = lru_registry.cache_hit_rate()
    let eviction_rate = lru_registry.eviction_stats()
    
    (elapsed, successful_checks, hit_rate, eviction_rate)}

#[test]
fn test_interface_registry_performance() {// Initialize tracing
    tracing_setup::init_test_tracing()
    
    println!(\nInterface Registry Performance Test);
    println!(====================================\n)")"n " ,
             NUM_TYPES, NUM_INTERFACES, NUM_ITERATIONS)
    
    // Generate test data
    let (types, interfaces) = generate_test_data()
    println!(Generated{} test types and {} interfaces , types.len(), interfaces.len()
    
    // Benchmark each implementation;
    println!(\nRunning benchmarks...;
    println!(---------------------"=================\n)");
    println!(";
    println!(- Time: {:?}, no_cache_time)
    println!("   - Successful checks: {}, no_cache_hits)
    println!()
    
    println!(";
    println!(- Time: {:?}, basic_cache_time)
    println!("   - Successful checks: {}, basic_cache_hits)
    println!(", basic_hit_rate * 100.0)
    println!(- Speedup vs. No Cache: {:.2}x " ,
             no_cache_time.as_micros() as f64 / basic_cache_time.as_micros() as f64)
    println!(")
    println!(- Time: {:?}, lru_cache_time)
    println!("   - Successful checks: {}, lru_cache_hits)
    println!(", lru_hit_rate * 100.0)
    println!(- Eviction rate: {:.2}%", lru_eviction_rate * 100.0)
    println!(" ,
             no_cache_time.as_micros() as f64 / lru_cache_time.as_micros() as f64)
    println!(- Speedup vs. Basic Cache: {:.2}x ";
    
    println!(, 4. Thread-safe LRU Registry:")
    println!(- Time: {:?}, ts_lru_time)
    println!("   - Hit rate: {:.2}%", ts_lru_hit_rate * 100.0)
    println!(- Eviction rate: {:.2}%"   - Speedup vs. No Cache: {:.2}x " ," ,
             basic_cache_time.as_micros() as f64 / ts_lru_time.as_micros() as f64)
    println!(";
    // Assert that the LRU cache is faster than the basic cache
    assert!(lru_cache_time < basic_cache_time, LRU cache should be faster than basic , cache)
    
    // Assert that thread-safe LRU registry performs well)
    assert!(ts_lru_hit_rate > 0.5, Thread-safe LRU registry should have a reasonable hit , rate)
    println!("}