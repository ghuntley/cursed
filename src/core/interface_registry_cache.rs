//! # Interface Registry Cache Implementation
//!
//! This module provides caching for interface implementation checks to improve performance
//! by remembering the results of previous checks.

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};
use crate::core::type_checker::Type;
use crate::error::Error;
use tracing::{debug, trace, instrument};

/// A key for the interface implementation cache
#[derive(Debug, Clone, Eq)]
struct CacheKey {
    /// The type being checked
    type_: Type,
    /// The name of the interface
    interface_name: String,
}

impl PartialEq for CacheKey {
    fn eq(&self, other: &Self) -> bool {
        self.type_ == other.type_ && self.interface_name == other.interface_name
    }
}

impl Hash for CacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash both the type and interface name
        self.type_.hash(state);
        self.interface_name.hash(state);
    }
}

/// A cache for interface implementation checks
#[derive(Debug, Default)]
pub struct InterfaceImplementationCache {
    /// Maps a (type, interface) pair to a boolean result
    cache: HashMap<CacheKey, bool>,
    
    /// Number of cache hits
    hits: usize,
    
    /// Number of cache misses
    misses: usize,
    
    /// Maximum number of entries in the cache
    max_size: usize,
}

impl InterfaceImplementationCache {
    /// Create a new interface implementation cache with default settings
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            hits: 0,
            misses: 0,
            max_size: 1000, // Default cache size
        }
    }
    
    /// Create a new interface implementation cache with the specified maximum size
    pub fn with_capacity(max_size: usize) -> Self {
        Self {
            cache: HashMap::with_capacity(max_size),
            hits: 0,
            misses: 0,
            max_size,
        }
    }
    
    /// Look up a result in the cache
    #[instrument(skip(self), level = "trace")]
    pub fn lookup(&mut self, type_: &Type, interface_name: &str) -> Option<bool> {
        let key = CacheKey {
            type_: type_.clone(),
            interface_name: interface_name.to_string(),
        };
        
        let result = self.cache.get(&key).copied();
        
        if result.is_some() {
            self.hits += 1;
            trace!("Cache hit for {:?} implements {}", type_, interface_name);
        } else {
            self.misses += 1;
            trace!("Cache miss for {:?} implements {}", type_, interface_name);
        }
        
        result
    }
    
    /// Store a result in the cache
    #[instrument(skip(self), level = "trace")]
    pub fn store(&mut self, type_: &Type, interface_name: &str, result: bool) {
        // If cache is at max capacity, don't store more
        if self.cache.len() >= self.max_size {
            trace!("Cache at max capacity ({}/{}), not storing result", self.cache.len(), self.max_size);
            return;
        }
        
        let key = CacheKey {
            type_: type_.clone(),
            interface_name: interface_name.to_string(),
        };
        
        self.cache.insert(key, result);
        trace!("Cached result for {:?} implements {} = {}", type_, interface_name, result);
    }
    
    /// Clear the cache
    pub fn clear(&mut self) {
        debug!("Clearing cache with {}/{} entries (hits: {}, misses: {})", 
               self.cache.len(), self.max_size, self.hits, self.misses);
        self.cache.clear();
        self.hits = 0;
        self.misses = 0;
    }
    
    /// Get cache statistics
    pub fn stats(&self) -> (usize, usize, usize) {
        (self.cache.len(), self.hits, self.misses)
    }
    
    /// Get the hit rate (hits / total lookups)
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
}

/// A thread-safe version of the interface implementation cache
pub struct ThreadSafeInterfaceCache {
    /// The wrapped cache
    cache: Arc<Mutex<InterfaceImplementationCache>>,
}

impl ThreadSafeInterfaceCache {
    /// Create a new thread-safe interface cache
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(InterfaceImplementationCache::new())),
        }
    }
    
    /// Create a new thread-safe interface cache with the specified maximum size
    pub fn with_capacity(max_size: usize) -> Self {
        Self {
            cache: Arc::new(Mutex::new(InterfaceImplementationCache::with_capacity(max_size))),
        }
    }
    
    /// Look up a result in the cache
    pub fn lookup(&self, type_: &Type, interface_name: &str) -> Option<bool> {
        let mut cache = self.cache.lock().unwrap();
        cache.lookup(type_, interface_name)
    }
    
    /// Store a result in the cache
    pub fn store(&self, type_: &Type, interface_name: &str, result: bool) {
        let mut cache = self.cache.lock().unwrap();
        cache.store(type_, interface_name, result);
    }
    
    /// Clear the cache
    pub fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }
    
    /// Get cache statistics
    pub fn stats(&self) -> (usize, usize, usize) {
        let cache = self.cache.lock().unwrap();
        cache.stats()
    }
    
    /// Get the hit rate (hits / total lookups)
    pub fn hit_rate(&self) -> f64 {
        let cache = self.cache.lock().unwrap();
        cache.hit_rate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[path = "../../tests/common.rs"]
    mod common;
    
    #[test]
    fn test_cache_basic_operations() {
        // Set up tracing for the test
        common::tracing::setup();
        
        let mut cache = InterfaceImplementationCache::new();
        
        let type_ = Type::Normie;
        let interface = "Numeric";
        
        // Initially, the cache should be empty
        assert_eq!(cache.lookup(&type_, interface), None);
        
        // Store a result
        cache.store(&type_, interface, true);
        
        // Now we should get a hit
        assert_eq!(cache.lookup(&type_, interface), Some(true));
        
        // Check stats
        let (size, hits, misses) = cache.stats();
        assert_eq!(size, 1);
        assert_eq!(hits, 1);
        assert_eq!(misses, 1);
    }
    
    #[test]
    fn test_cache_multiple_entries() {
        // Set up tracing for the test
        common::tracing::setup();
        
        let mut cache = InterfaceImplementationCache::new();
        
        // Store multiple entries
        cache.store(&Type::Normie, "Numeric", true);
        cache.store(&Type::Tea, "Comparable", true);
        cache.store(&Type::Lit, "Numeric", false);
        
        // Check lookups
        assert_eq!(cache.lookup(&Type::Normie, "Numeric"), Some(true));
        assert_eq!(cache.lookup(&Type::Tea, "Comparable"), Some(true));
        assert_eq!(cache.lookup(&Type::Lit, "Numeric"), Some(false));
        assert_eq!(cache.lookup(&Type::Lit, "Comparable"), None);  // Not cached
        
        // Check stats
        let (size, hits, misses) = cache.stats();
        assert_eq!(size, 3);
        assert_eq!(hits, 3);
        assert_eq!(misses, 1);
        
        // Clear the cache
        cache.clear();
        
        // Cache should now be empty
        assert_eq!(cache.lookup(&Type::Normie, "Numeric"), None);
        
        // Stats should be reset
        let (size, hits, misses) = cache.stats();
        assert_eq!(size, 0);
        assert_eq!(hits, 0);
        assert_eq!(misses, 1);
    }
    
    #[test]
    fn test_thread_safe_cache() {
        // Set up tracing for the test
        common::tracing::setup();
        
        let cache = ThreadSafeInterfaceCache::new();
        
        // Store a result
        cache.store(&Type::Normie, "Numeric", true);
        
        // Look up the result
        assert_eq!(cache.lookup(&Type::Normie, "Numeric"), Some(true));
        
        // Stats should show 1 hit
        let (size, hits, misses) = cache.stats();
        assert_eq!(size, 1);
        assert_eq!(hits, 1);
        assert_eq!(misses, 0);
    }
    
    #[test]
    fn test_complex_types_in_cache() {
        // Set up tracing for the test
        common::tracing::setup();
        
        let mut cache = InterfaceImplementationCache::new();
        
        // Create complex types
        let stack_type = Type::Struct(
            "Stack".to_string(),
            vec![Box::new(Type::Tea)]
        );
        
        // Store results for complex types
        cache.store(&stack_type, "Container", true);
        
        // Look up results
        assert_eq!(cache.lookup(&stack_type, "Container"), Some(true));
        
        // Different type argument should be a miss
        let stack_int = Type::Struct(
            "Stack".to_string(),
            vec![Box::new(Type::Normie)]
        );
        assert_eq!(cache.lookup(&stack_int, "Container"), None);
    }
    
    #[test]
    fn test_cache_capacity_limit() {
        // Set up tracing for the test
        common::tracing::setup();
        
        // Create a cache with a small capacity
        let mut cache = InterfaceImplementationCache::with_capacity(2);
        
        // Store items up to capacity
        cache.store(&Type::Normie, "Numeric", true);
        cache.store(&Type::Tea, "Comparable", true);
        
        // These should be in the cache
        assert_eq!(cache.lookup(&Type::Normie, "Numeric"), Some(true));
        assert_eq!(cache.lookup(&Type::Tea, "Comparable"), Some(true));
        
        // Try to store more than capacity
        cache.store(&Type::Lit, "Comparable", true);
        
        // This should not be added because we're at capacity
        assert_eq!(cache.lookup(&Type::Lit, "Comparable"), None);
        
        // Check stats
        let (size, _, _) = cache.stats();
        assert_eq!(size, 2); // Still only 2 items
    }
}