//! # Interface Registry LRU Cache Implementation
//!
//! This module provides an advanced LRU (Least Recently Used) caching mechanism for 
//! interface implementation checks to significantly improve performance.
//! It enhances the basic caching mechanism by implementing an efficient LRU strategy
//! to maintain a fixed-size cache of the most recently used entries.

use std::collections::{HashMap, VecDeque};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};
use crate::core::type_checker::Type;
use crate::error::Error;
use tracing::{debug, trace, instrument, info, warn};

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

/// A cache entry with a timestamp for LRU tracking
#[derive(Debug, Clone)]
struct CacheEntry {
    /// The result of the interface implementation check
    result: bool,
    /// When this entry was last accessed
    last_accessed: u64,
}

/// An advanced LRU cache for interface implementation checks
#[derive(Debug)]
pub struct LruInterfaceCache {
    /// Maps a (type, interface) pair to a boolean result with timestamp
    cache: HashMap<CacheKey, CacheEntry>,
    
    /// Ordered list of cache keys by access time for LRU eviction
    access_order: VecDeque<CacheKey>,
    
    /// Number of cache hits
    hits: usize,
    
    /// Number of cache misses
    misses: usize,
    
    /// Maximum number of entries in the cache
    max_size: usize,
    
    /// The current timestamp counter
    timestamp: u64,
    
    /// Number of cache evictions
    evictions: usize,
    
    /// Number of cache updates
    updates: usize,
}

impl LruInterfaceCache {
    /// Create a new interface implementation cache with default settings
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            access_order: VecDeque::new(),
            hits: 0,
            misses: 0,
            max_size: 1000, // Default cache size
            timestamp: 0,
            evictions: 0,
            updates: 0,
        }
    }
    
    /// Create a new interface implementation cache with the specified maximum size
    pub fn with_capacity(max_size: usize) -> Self {
        Self {
            cache: HashMap::with_capacity(max_size),
            access_order: VecDeque::with_capacity(max_size),
            hits: 0,
            misses: 0,
            max_size,
            timestamp: 0,
            evictions: 0,
            updates: 0,
        }
    }
    
    /// Look up a result in the cache
    #[instrument(skip(self), level = "trace")]
    pub fn lookup(&mut self, type_: &Type, interface_name: &str) -> Option<bool> {
        let key = CacheKey {
            type_: type_.clone(),
            interface_name: interface_name.to_string(),
        };
        
        if let Some(entry) = self.cache.get_mut(&key) {
            // Hit - update the timestamp
            self.timestamp += 1;
            entry.last_accessed = self.timestamp;
            
            // Update the entry position in the access order list by removing and re-adding
            if let Some(pos) = self.access_order.iter().position(|k| k == &key) {
                self.access_order.remove(pos);
            }
            self.access_order.push_back(key.clone());
            
            self.hits += 1;
            trace!("LRU cache hit for {:?} implements {}", type_, interface_name);
            return Some(entry.result);
        }
        
        self.misses += 1;
        trace!("LRU cache miss for {:?} implements {}", type_, interface_name);
        None
    }
    
    /// Store a result in the cache
    #[instrument(skip(self), level = "trace")]
    pub fn store(&mut self, type_: &Type, interface_name: &str, result: bool) {
        let key = CacheKey {
            type_: type_.clone(),
            interface_name: interface_name.to_string(),
        };
        
        // Increment timestamp
        self.timestamp += 1;
        
        // If the key already exists, update it and refresh its position
        if self.cache.contains_key(&key) {
            if let Some(entry) = self.cache.get_mut(&key) {
                entry.result = result;
                entry.last_accessed = self.timestamp;
                
                // Update position in access order list
                if let Some(pos) = self.access_order.iter().position(|k| k == &key) {
                    self.access_order.remove(pos);
                }
                self.access_order.push_back(key);
                
                self.updates += 1;
                trace!("Updated cache entry for {:?} implements {} = {}", type_, interface_name, result);
                return;
            }
        }
        
        // If cache is at max capacity, remove least recently used item
        if self.cache.len() >= self.max_size {
            self.evict_lru();
        }
        
        // Insert new entry
        let entry = CacheEntry {
            result,
            last_accessed: self.timestamp,
        };
        
        self.cache.insert(key.clone(), entry);
        self.access_order.push_back(key);
        
        trace!("Cached result for {:?} implements {} = {}", type_, interface_name, result);
    }
    
    /// Evict the least recently used entry from the cache
    fn evict_lru(&mut self) {
        if let Some(lru_key) = self.access_order.pop_front() {
            // Remove the entry from the cache
            if self.cache.remove(&lru_key).is_some() {
                self.evictions += 1;
                trace!("Evicted LRU cache entry for {:?} implements {}", 
                       lru_key.type_, lru_key.interface_name);
            }
        }
    }
    
    /// Clear the cache
    pub fn clear(&mut self) {
        debug!("Clearing LRU cache with {}/{} entries (hits: {}, misses: {}, evictions: {})", 
               self.cache.len(), self.max_size, self.hits, self.misses, self.evictions);
        self.cache.clear();
        self.access_order.clear();
        self.hits = 0;
        self.misses = 0;
        self.evictions = 0;
        self.updates = 0;
        self.timestamp = 0;
    }
    
    /// Get cache statistics
    pub fn stats(&self) -> (usize, usize, usize, usize, usize) {
        (self.cache.len(), self.hits, self.misses, self.evictions, self.updates)
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
    
    /// Get the eviction rate (evictions / total stores)
    pub fn eviction_rate(&self) -> f64 {
        let total = self.evictions + self.updates + self.cache.len();
        if total == 0 {
            0.0
        } else {
            self.evictions as f64 / total as f64
        }
    }
}

/// A thread-safe version of the LRU interface cache
pub struct ThreadSafeLruCache {
    /// The wrapped cache
    cache: Arc<Mutex<LruInterfaceCache>>,
}

impl ThreadSafeLruCache {
    /// Create a new thread-safe LRU interface cache
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(LruInterfaceCache::new())),
        }
    }
    
    /// Create a new thread-safe LRU interface cache with the specified maximum size
    pub fn with_capacity(max_size: usize) -> Self {
        Self {
            cache: Arc::new(Mutex::new(LruInterfaceCache::with_capacity(max_size))),
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
    pub fn stats(&self) -> (usize, usize, usize, usize, usize) {
        let cache = self.cache.lock().unwrap();
        cache.stats()
    }
    
    /// Get the hit rate (hits / total lookups)
    pub fn hit_rate(&self) -> f64 {
        let cache = self.cache.lock().unwrap();
        cache.hit_rate()
    }
    
    /// Get the eviction rate (evictions / total stores)
    pub fn eviction_rate(&self) -> f64 {
        let cache = self.cache.lock().unwrap();
        cache.eviction_rate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    use crate::tests::common;
    
    #[test]
    fn test_lru_cache_basic_operations() {
        // Set up tracing for the test
        common::tracing::setup();
        
        let mut cache = LruInterfaceCache::new();
        
        let type_ = Type::Normie;
        let interface = "Numeric";
        
        // Initially, the cache should be empty
        assert_eq!(cache.lookup(&type_, interface), None);
        
        // Store a result
        cache.store(&type_, interface, true);
        
        // Now we should get a hit
        assert_eq!(cache.lookup(&type_, interface), Some(true));
        
        // Check stats
        let (size, hits, misses, evictions, updates) = cache.stats();
        assert_eq!(size, 1);
        assert_eq!(hits, 1);
        assert_eq!(misses, 1);
        assert_eq!(evictions, 0);
        assert_eq!(updates, 0);
        
        // Hit rate should be 0.5 (1/2)
        assert_eq!(cache.hit_rate(), 0.5);
    }
    
    #[test]
    fn test_lru_cache_eviction() {
        // Set up tracing for the test
        common::tracing::setup();
        
        // Create a small cache
        let mut cache = LruInterfaceCache::with_capacity(2);
        
        let type1 = Type::Normie;
        let type2 = Type::Tea;
        let type3 = Type::Lit;
        
        // Add first two entries
        cache.store(&type1, "Numeric", true);
        cache.store(&type2, "Comparable", true);
        
        // These should be cache hits
        assert_eq!(cache.lookup(&type1, "Numeric"), Some(true));
        assert_eq!(cache.lookup(&type2, "Comparable"), Some(true));
        
        // Now access type1 to make it more recently used than type2
        cache.lookup(&type1, "Numeric");
        
        // Add a third entry - this should evict type2 (least recently used)
        cache.store(&type3, "Comparable", false);
        
        // Check what's in the cache
        assert_eq!(cache.lookup(&type1, "Numeric"), Some(true)); // Still in cache
        assert_eq!(cache.lookup(&type3, "Comparable"), Some(false)); // New entry
        assert_eq!(cache.lookup(&type2, "Comparable"), None); // Evicted
        
        // Check eviction stats
        let (size, _, _, evictions, _) = cache.stats();
        assert_eq!(size, 2); // Still at max size
        assert_eq!(evictions, 1); // One eviction occurred
    }
    
    #[test]
    fn test_lru_cache_updates() {
        // Set up tracing for the test
        common::tracing::setup();
        
        let mut cache = LruInterfaceCache::new();
        
        let type_ = Type::Normie;
        let interface = "Numeric";
        
        // Store initial value
        cache.store(&type_, interface, true);
        assert_eq!(cache.lookup(&type_, interface), Some(true));
        
        // Update the value
        cache.store(&type_, interface, false);
        
        // Should get the updated value
        assert_eq!(cache.lookup(&type_, interface), Some(false));
        
        // Check update stats
        let (size, _, _, _, updates) = cache.stats();
        assert_eq!(size, 1); // Still one entry
        assert_eq!(updates, 1); // One update occurred
    }
    
    #[test]
    fn test_thread_safe_lru_cache() {
        // Set up tracing for the test
        common::tracing::setup();
        
        let cache = ThreadSafeLruCache::new();
        
        // Store a result
        cache.store(&Type::Normie, "Numeric", true);
        
        // Look up the result
        assert_eq!(cache.lookup(&Type::Normie, "Numeric"), Some(true));
        
        // Check stats
        let (size, hits, misses, evictions, updates) = cache.stats();
        assert_eq!(size, 1);
        assert_eq!(hits, 1);
        assert_eq!(misses, 0);
        assert_eq!(evictions, 0);
        assert_eq!(updates, 0);
    }
    
    #[test]
    fn test_lru_cache_complex_types() {
        // Set up tracing for the test
        common::tracing::setup();
        
        let mut cache = LruInterfaceCache::new();
        
        // Create complex types
        let stack_tea = Type::Struct(
            "Stack".to_string(),
            vec![Box::new(Type::Tea)]
        );
        
        let stack_int = Type::Struct(
            "Stack".to_string(),
            vec![Box::new(Type::Normie)]
        );
        
        // Store result for stack_tea
        cache.store(&stack_tea, "Container", true);
        
        // Should hit for stack_tea
        assert_eq!(cache.lookup(&stack_tea, "Container"), Some(true));
        
        // Should miss for stack_int (different type argument)
        assert_eq!(cache.lookup(&stack_int, "Container"), None);
        
        // Store result for stack_int
        cache.store(&stack_int, "Container", true);
        
        // Now both should hit
        assert_eq!(cache.lookup(&stack_tea, "Container"), Some(true));
        assert_eq!(cache.lookup(&stack_int, "Container"), Some(true));
        
        // Check stats
        let (size, hits, misses, _, _) = cache.stats();
        assert_eq!(size, 2);
        assert_eq!(hits, 3);
        assert_eq!(misses, 1);
    }
    
    #[test]
    fn test_lru_cache_clear() {
        // Set up tracing for the test
        common::tracing::setup();
        
        let mut cache = LruInterfaceCache::new();
        
        // Add several entries
        cache.store(&Type::Normie, "Numeric", true);
        cache.store(&Type::Tea, "Comparable", true);
        cache.store(&Type::Lit, "Comparable", false);
        
        // Verify they're in the cache
        assert_eq!(cache.lookup(&Type::Normie, "Numeric"), Some(true));
        assert_eq!(cache.lookup(&Type::Tea, "Comparable"), Some(true));
        assert_eq!(cache.lookup(&Type::Lit, "Comparable"), Some(false));
        
        // Clear the cache
        cache.clear();
        
        // All lookups should now miss
        assert_eq!(cache.lookup(&Type::Normie, "Numeric"), None);
        assert_eq!(cache.lookup(&Type::Tea, "Comparable"), None);
        assert_eq!(cache.lookup(&Type::Lit, "Comparable"), None);
        
        // Stats should be reset
        let (size, hits, misses, evictions, updates) = cache.stats();
        assert_eq!(size, 0);
        assert_eq!(hits, 0);
        assert_eq!(misses, 3); // New misses after clearing
        assert_eq!(evictions, 0);
        assert_eq!(updates, 0);
    }
}