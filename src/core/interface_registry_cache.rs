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
    #[path = "../../tests/common.rs"]
    pub mod test_common;
    use self::test_common as common;
    use super::*;
    
    #[test]
    fn test_cache_basic_operations() {
        let mut cache = InterfaceImplementationCache::new();
        
        // First lookup should be a miss
        assert_eq!(cache.lookup(&Type::Normie, "Numeric"), None);
        
        // Store a result
        cache.store(&Type::Normie, "Numeric", true);
        
        // Second lookup should be a hit
        assert_eq!(cache.lookup(&Type::Normie, "Numeric"), Some(true));
        
        // Verify stats
        let (size, hits, misses) = cache.stats();
        assert_eq!(size, 1);
        assert_eq!(hits, 1);
        assert_eq!(misses, 1);
        
        // Clear the cache
        cache.clear();
        
        // Stats should be reset
        let (size, hits, misses) = cache.stats();
        assert_eq!(size, 0);
        assert_eq!(hits, 0);
        assert_eq!(misses, 0);
    }
    
    #[test]
    fn test_thread_safe_cache() {
        let cache = ThreadSafeInterfaceCache::new();
        
        // First lookup should be a miss
        assert_eq!(cache.lookup(&Type::Normie, "Numeric"), None);
        
        // Store a result
        cache.store(&Type::Normie, "Numeric", true);
        
        // Second lookup should be a hit
        assert_eq!(cache.lookup(&Type::Normie, "Numeric"), Some(true));
        
        // Verify stats
        let (size, hits, misses) = cache.stats();
        assert_eq!(size, 1);
        assert_eq!(hits, 1);
        assert_eq!(misses, 1);
        
        // Test hit rate
        assert_eq!(cache.hit_rate(), 0.5);
    }
    
    #[test]
    fn test_cache_with_tracing() {
        // Initialize tracing infrastructure
        common::tracing::setup();
        
        let mut cache = InterfaceImplementationCache::new();
        
        // Operations with tracing enabled
        assert_eq!(cache.lookup(&Type::Normie, "Numeric"), None);
        cache.store(&Type::Normie, "Numeric", true);
        assert_eq!(cache.lookup(&Type::Normie, "Numeric"), Some(true));
        
        // Track timing using the Timer utility
        let _timer = common::timing::Timer::new("cache_operations");
        
        // Do some more operations while timing
        cache.store(&Type::Tea, "Stringer", false);
        cache.lookup(&Type::Tea, "Stringer");
        
        // Timer automatically logs completion on drop
        
        // Clear should log information
        cache.clear();
    }
    
    #[test]
    fn test_cache_capacity() {
        let mut cache = InterfaceImplementationCache::with_capacity(2);
        
        // Store two results
        cache.store(&Type::Normie, "Numeric", true);
        cache.store(&Type::Tea, "Stringer", true);
        
        // Verify size
        let (size, _, _) = cache.stats();
        assert_eq!(size, 2);
        
        // Try to store a third result (should be ignored due to capacity)
        cache.store(&Type::Lit, "Boolean", true);
        
        // Size should still be 2
        let (size, _, _) = cache.stats();
        assert_eq!(size, 2);
    }
}