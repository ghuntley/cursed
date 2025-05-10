//! # Interface Registry LRU Cache Extension
//!
//! This module provides an extension to use the LRU caching mechanism with
//! the interface registry, improving performance for constraint checking.

use crate::core::type_checker_interface_registry::CachedInterfaceRegistry;
use crate::core::interface_registry_lru_cache::{LruInterfaceCache, ThreadSafeLruCache};
use crate::core::interface_registry::InterfaceRegistry;
use crate::core::type_checker::Type;
use crate::error::Error;
use std::sync::{Arc, Mutex};
use tracing::{debug, trace, instrument};

/// An extension of the interface registry that uses LRU caching
pub struct LruCachedRegistry {
    /// The underlying registry
    registry: InterfaceRegistry,
    
    /// The LRU cache for implementation checks
    cache: LruInterfaceCache,
}

impl LruCachedRegistry {
    /// Create a new LRU cached registry
    pub fn new(registry: InterfaceRegistry) -> Self {
        Self {
            registry,
            cache: LruInterfaceCache::new(),
        }
    }
    
    /// Create a new LRU cached registry with default implementations
    pub fn new_with_defaults() -> Self {
        Self {
            registry: InterfaceRegistry::new_with_defaults(),
            cache: LruInterfaceCache::new(),
        }
    }
    
    /// Create a new LRU cached registry with a specific cache capacity
    pub fn with_capacity(registry: InterfaceRegistry, capacity: usize) -> Self {
        Self {
            registry,
            cache: LruInterfaceCache::with_capacity(capacity),
        }
    }
    
    /// Get a reference to the underlying registry
    pub fn registry(&self) -> &InterfaceRegistry {
        &self.registry
    }
    
    /// Get a mutable reference to the underlying registry
    pub fn registry_mut(&mut self) -> &mut InterfaceRegistry {
        &mut self.registry
    }
    
    /// Get cache eviction statistics
    pub fn eviction_stats(&self) -> (usize, f64) {
        let (_, _, _, evictions, updates) = self.cache.stats();
        (evictions, self.cache.eviction_rate())
    }
}

/// A thread-safe LRU cached interface registry
pub struct ThreadSafeLruRegistry {
    /// The registry with a mutex for thread safety
    registry: Arc<Mutex<InterfaceRegistry>>,
    
    /// The thread-safe LRU cache
    cache: ThreadSafeLruCache,
}

impl ThreadSafeLruRegistry {
    /// Create a new thread-safe LRU cached registry
    pub fn new(registry: InterfaceRegistry) -> Self {
        Self {
            registry: Arc::new(Mutex::new(registry)),
            cache: ThreadSafeLruCache::new(),
        }
    }
    
    /// Create a new thread-safe LRU cached registry with default implementations
    pub fn new_with_defaults() -> Self {
        Self {
            registry: Arc::new(Mutex::new(InterfaceRegistry::new_with_defaults())),
            cache: ThreadSafeLruCache::new(),
        }
    }
    
    /// Create a new thread-safe LRU cached registry with a specific cache capacity
    pub fn with_capacity(registry: InterfaceRegistry, capacity: usize) -> Self {
        Self {
            registry: Arc::new(Mutex::new(registry)),
            cache: ThreadSafeLruCache::with_capacity(capacity),
        }
    }
    
    /// Get the underlying registry (acquires lock)
    pub fn registry(&self) -> Arc<Mutex<InterfaceRegistry>> {
        self.registry.clone()
    }
    
    /// Check if a type implements an interface with LRU caching
    #[instrument(skip(self), level = "debug")]
    pub fn check_implementation(&self, type_: &Type, interface_name: &str) -> Result<bool, Error> {
        // First check the cache
        if let Some(result) = self.cache.lookup(type_, interface_name) {
            debug!("Thread-safe LRU cache hit for {:?} implements {}: {}", type_, interface_name, result);
            return Ok(result);
        }
        
        // Not in cache, check the implementation (acquires lock)
        debug!("Thread-safe LRU cache miss for {:?} implements {}", type_, interface_name);
        let registry = self.registry.lock().unwrap();
        let result = registry.check_implementation(type_, interface_name)?;
        
        // Store the result in the cache
        self.cache.store(type_, interface_name, result);
        
        Ok(result)
    }
    
    /// Get cache eviction statistics
    pub fn eviction_stats(&self) -> f64 {
        self.cache.eviction_rate()
    }
}

// Implement CachedInterfaceRegistry for LruCachedRegistry
impl CachedInterfaceRegistry for LruCachedRegistry {
    #[instrument(skip(self), level = "debug")]
    fn check_implementation_cached(
        &mut self,
        type_: &Type,
        interface_name: &str
    ) -> Result<bool, Error> {
        // First check the LRU cache
        if let Some(result) = self.cache.lookup(type_, interface_name) {
            debug!("LRU cache hit for {:?} implements {}: {}", type_, interface_name, result);
            return Ok(result);
        }
        
        // Not in cache, check the implementation
        debug!("LRU cache miss for {:?} implements {}, checking implementation", type_, interface_name);
        let result = self.registry.check_implementation(type_, interface_name)?;
        
        // Store the result in the cache
        self.cache.store(type_, interface_name, result);
        
        Ok(result)
    }
    
    fn clear_cache(&mut self) {
        debug!("Clearing LRU interface implementation cache");
        self.cache.clear();
    }
    
    fn cache_stats(&self) -> (usize, usize, usize) {
        let (size, hits, misses, _, _) = self.cache.stats();
        (size, hits, misses)
    }
    
    fn cache_hit_rate(&self) -> f64 {
        self.cache.hit_rate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[path = "../../tests/common.rs"]
    mod common;
    
    #[test]
    fn test_lru_cached_registry_basic() {
        // Set up tracing for the test
        common::tracing::setup();
        
        // Create a new registry with LRU caching
        let mut registry = LruCachedRegistry::new_with_defaults();
        
        // Test basic operations
        let type_ = Type::Normie;
        let interface = "Numeric";
        
        // First check should be a cache miss
        let result = registry.check_implementation_cached(&type_, interface).unwrap();
        assert!(result, "Normie should implement Numeric");
        
        // Second check should be a cache hit
        let result = registry.check_implementation_cached(&type_, interface).unwrap();
        assert!(result, "Normie should implement Numeric");
        
        // Check that cache stats have been updated
        let (size, hits, misses) = registry.cache_stats();
        assert_eq!(size, 1);
        assert_eq!(hits, 1);
        assert_eq!(misses, 1);
    }
    
    #[test]
    fn test_thread_safe_lru_registry() {
        // Set up tracing for the test
        common::tracing::setup();
        
        // Create a thread-safe registry with LRU caching
        let registry = ThreadSafeLruRegistry::new_with_defaults();
        
        // Test basic operations
        let type_ = Type::Normie;
        let interface = "Numeric";
        
        // First check should succeed (cache miss)
        let result = registry.check_implementation(&type_, interface).unwrap();
        assert!(result, "Normie should implement Numeric");
        
        // Second check should be a cache hit
        let result = registry.check_implementation(&type_, interface).unwrap();
        assert!(result, "Normie should implement Numeric");
        
        // Check that cache stats have been updated
        let stats = registry.cache.stats();
        assert_eq!(stats.0, 1); // size
        assert_eq!(stats.1, 1); // hits
        assert_eq!(stats.2, 1); // misses
    }
}