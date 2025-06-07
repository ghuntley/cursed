//! # Interface Registry LRU Cache Extension
//!
//! This module provides an extension to use the LRU caching mechanism with
//! the interface registry, improving performance for constraint checking.

use crate::core::type_checker_interface_registry::CachedInterfaceRegistry;
use crate::core::interface_registry_lru_cache::LruInterfaceCache;
use crate::core::interface_registry::InterfaceRegistry;
use crate::core::type_checker::Type;
use crate::codegen::llvm::interface_registry::InterfaceTypeRegistry;
use crate::error::Error;
use std::sync::{Arc, Mutex};
use tracing::{debug, trace, instrument};

/// An extension of the interface registry that uses LRU caching
#[derive(Debug)]
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
    cache: LruInterfaceCache,
}

impl ThreadSafeLruRegistry {
    /// Create a new thread-safe LRU cached registry
    pub fn new(registry: InterfaceRegistry) -> Self {
        Self {
            registry: Arc::new(Mutex::new(registry)),
            cache: LruInterfaceCache::new(),
        }
    }
    
    /// Create a new thread-safe LRU cached registry with default implementations
    pub fn new_with_defaults() -> Self {
        Self {
            registry: Arc::new(Mutex::new(InterfaceRegistry::new_with_defaults())),
            cache: LruInterfaceCache::new(),
        }
    }
    
    /// Create a new thread-safe LRU cached registry with a specific cache capacity
    pub fn with_capacity(registry: InterfaceRegistry, capacity: usize) -> Self {
        Self {
            registry: Arc::new(Mutex::new(registry)),
            cache: LruInterfaceCache::with_capacity(capacity),
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
    
    /// Get cache statistics (size, hits, misses, evictions, updates)
    pub fn cache_stats(&self) -> (usize, usize, usize, usize, usize) {
        self.cache.stats()
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
    
    use crate::tests::common;
    
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
        let stats = registry.cache_stats();
        assert_eq!(stats.0, 1); // size
        assert_eq!(stats.1, 1); // hits
        assert_eq!(stats.2, 1); // misses
    }
}

impl InterfaceTypeRegistry for LruCachedRegistry {
    fn register_interface(&mut self, _name: &str) -> Result<(), Error> {
        // Simple stub implementation
        Ok(())
    }
    
    fn register_extension(&mut self, _source: &str, _target: &str) -> Result<(), Error> {
        // Simple stub implementation
        Ok(())
    }
    
    fn extends(&self, _source: &str, _target: &str) -> Result<bool, Error> {
        // Simple stub implementation
        Ok(false)
    }
    
    fn find_path(&self, _source: &str, _target: &str) -> Result<Option<Vec<String>>, Error> {
        // Simple stub implementation
        Ok(None)
    }
    
    fn get_all_interfaces(&self) -> Result<std::collections::HashSet<String>, Error> {
        // Simple stub implementation
        Ok(std::collections::HashSet::new())
    }
    
    fn interface_exists(&self, _name: &str) -> Result<bool, Error> {
        // Simple stub implementation
        Ok(false)
    }
}