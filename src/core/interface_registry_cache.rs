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

// Tests temporarily removed for simplicity