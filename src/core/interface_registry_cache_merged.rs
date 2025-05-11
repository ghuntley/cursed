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

/// Test utilities for interface registry cache
pub mod test_common {
    //! Test utilities for interface registry cache
    //!
    //! This module provides common test functions and fixtures for testing
    //! the interface registry cache functionality.
    
    use std::collections::{HashMap, HashSet};
    use std::sync::{Arc, RwLock};
    use tracing::{debug, error, info, instrument, trace, warn};
    
    use crate::core::interface_registry::ThreadSafeInterfaceExtensionRegistry;
    use crate::core::interface_registry_extensions::InterfaceRegistryExtension;
    use crate::core::interface_registry_lru_cache::ThreadSafeInterfaceRegistryLruCache;
    use crate::core::interface_registry_visualization::InterfaceRegistryExtensionWithVisualization;
    use crate::error::Error;
    
    /// Create a test interface registry with predefined interface relationships
    #[instrument(level = "trace")]
    pub fn create_test_registry() -> Arc<RwLock<dyn InterfaceRegistryExtension + Send + Sync>> {
        trace!("Creating test interface registry for tests");
        let registry = ThreadSafeInterfaceExtensionRegistry::new();
        
        // Add some test interfaces
        registry.write().unwrap().register_interface("Reader");
        registry.write().unwrap().register_interface("Writer");
        registry.write().unwrap().register_interface("Closer");
        registry.write().unwrap().register_interface("FileReader");
        registry.write().unwrap().register_interface("FileWriter");
        registry.write().unwrap().register_interface("File");
        
        // Add some inheritance relationships
        registry.write().unwrap().register_extension("FileReader", "Reader").unwrap();
        registry.write().unwrap().register_extension("FileWriter", "Writer").unwrap();
        registry.write().unwrap().register_extension("File", "FileReader").unwrap();
        registry.write().unwrap().register_extension("File", "FileWriter").unwrap();
        registry.write().unwrap().register_extension("File", "Closer").unwrap();
        
        registry
    }
    
    /// Create a test registry with visualization capabilities for interface assertion testing
    #[instrument(level = "trace")]
    pub fn create_test_registry_with_visualization() -> Arc<RwLock<dyn InterfaceRegistryExtensionWithVisualization + Send + Sync>> {
        trace!("Creating test registry with visualization for tests");
        let registry = ThreadSafeInterfaceExtensionRegistry::new();
        
        // Add some test interfaces
        registry.write().unwrap().register_interface("Reader");
        registry.write().unwrap().register_interface("Writer");
        registry.write().unwrap().register_interface("Closer");
        registry.write().unwrap().register_interface("FileReader");
        registry.write().unwrap().register_interface("FileWriter");
        registry.write().unwrap().register_interface("File");
        registry.write().unwrap().register_interface("NetworkReader");
        registry.write().unwrap().register_interface("NetworkWriter");
        registry.write().unwrap().register_interface("Network");
        
        // Add some inheritance relationships
        registry.write().unwrap().register_extension("FileReader", "Reader").unwrap();
        registry.write().unwrap().register_extension("FileWriter", "Writer").unwrap();
        registry.write().unwrap().register_extension("File", "FileReader").unwrap();
        registry.write().unwrap().register_extension("File", "FileWriter").unwrap();
        registry.write().unwrap().register_extension("File", "Closer").unwrap();
        registry.write().unwrap().register_extension("NetworkReader", "Reader").unwrap();
        registry.write().unwrap().register_extension("NetworkWriter", "Writer").unwrap();
        registry.write().unwrap().register_extension("Network", "NetworkReader").unwrap();
        registry.write().unwrap().register_extension("Network", "NetworkWriter").unwrap();
        registry.write().unwrap().register_extension("Network", "Closer").unwrap();
        
        registry
    }
    
    /// Create a test registry with diamond inheritance pattern for testing diamond inheritance
    #[instrument(level = "trace")]
    pub fn create_diamond_inheritance_registry() -> Arc<RwLock<dyn InterfaceRegistryExtensionWithVisualization + Send + Sync>> {
        trace!("Creating diamond inheritance test registry");
        let registry = ThreadSafeInterfaceExtensionRegistry::new();
        
        // Create a diamond inheritance pattern:
        //       A
        //      / \
        //     B   C
        //      \ /
        //       D
        
        registry.write().unwrap().register_interface("A");
        registry.write().unwrap().register_interface("B");
        registry.write().unwrap().register_interface("C");
        registry.write().unwrap().register_interface("D");
        
        registry.write().unwrap().register_extension("B", "A").unwrap();
        registry.write().unwrap().register_extension("C", "A").unwrap();
        registry.write().unwrap().register_extension("D", "B").unwrap();
        registry.write().unwrap().register_extension("D", "C").unwrap();
        
        registry
    }
    
    /// Create a test registry cache with visualization capabilities
    #[instrument(level = "trace")]
    pub fn create_test_registry_cache() -> Arc<RwLock<ThreadSafeInterfaceRegistryLruCache>> {
        trace!("Creating test registry cache");
        let registry = create_test_registry_with_visualization();
        let cache = ThreadSafeInterfaceRegistryLruCache::new(100, registry.clone());
        Arc::new(RwLock::new(cache))
    }
    
    /// Verify that the test registry contains expected interfaces and relationships
    #[instrument(level = "trace")]
    pub fn verify_test_registry(registry: &Arc<RwLock<dyn InterfaceRegistryExtension + Send + Sync>>) -> Result<(), Error> {
        trace!("Verifying test registry contents");
        
        // Check if all expected interfaces are registered
        let interfaces = registry.read()
            .map_err(|_| Error::Internal("Failed to acquire read lock on registry".to_string()))?
            .get_all_interfaces()
            .unwrap_or_default();
        
        let expected_interfaces = vec![
            "Reader", "Writer", "Closer", "FileReader", "FileWriter", "File"
        ];
        
        for interface in expected_interfaces {
            if !interfaces.contains(interface) {
                return Err(Error::Validation(format!(
                    "Expected interface '{}' not found in registry", interface
                )));
            }
        }
        
        // Check if expected extensions are registered
        let extensions = vec![
            ("FileReader", "Reader"),
            ("FileWriter", "Writer"),
            ("File", "FileReader"),
            ("File", "FileWriter"),
            ("File", "Closer"),
        ];
        
        for (source, target) in extensions {
            let extension_exists = registry.read()
                .map_err(|_| Error::Internal("Failed to acquire read lock on registry".to_string()))?
                .has_extension(source, target)?;
            
            if !extension_exists {
                return Err(Error::Validation(format!(
                    "Expected extension relationship from '{}' to '{}' not found", source, target
                )));
            }
        }
        
        Ok(())
    }
    
    /// Calculate the transitive closure of interface relationships
    #[instrument(level = "trace")]
    pub fn calculate_transitive_closure(
        registry: &Arc<RwLock<dyn InterfaceRegistryExtension + Send + Sync>>
    ) -> Result<HashMap<String, HashSet<String>>, Error> {
        trace!("Calculating transitive closure of interface relationships");
        
        let registry_guard = registry.read()
            .map_err(|_| Error::Internal("Failed to acquire read lock on registry".to_string()))?;
        
        let interfaces = registry_guard.get_all_interfaces()
            .unwrap_or_default();
        
        let mut closure = HashMap::new();
        
        // Initialize closure with direct extensions
        for source in &interfaces {
            let mut reachable = HashSet::new();
            
            if let Some(extensions) = registry_guard.get_direct_extensions(source)? {
                for target in extensions {
                    reachable.insert(target);
                }
            }
            
            closure.insert(source.clone(), reachable);
        }
        
        // Compute transitive closure (Floyd-Warshall algorithm)
        for k in &interfaces {
            for i in &interfaces {
                for j in &interfaces {
                    if closure.get(i).unwrap_or(&HashSet::new()).contains(k) &&
                       closure.get(k).unwrap_or(&HashSet::new()).contains(j) {
                        // i can reach j through k
                        closure.entry(i.clone())
                            .or_insert_with(HashSet::new)
                            .insert(j.clone());
                    }
                }
            }
        }
        
        Ok(closure)
    }
    
    /// Populate a registry with complex interface hierarchies for comprehensive testing
    #[instrument(level = "trace")]
    pub fn populate_complex_hierarchy(
        registry: &Arc<RwLock<dyn InterfaceRegistryExtension + Send + Sync>>
    ) -> Result<(), Error> {
        trace!("Populating complex interface hierarchy for testing");
        
        let mut registry_guard = registry.write()
            .map_err(|_| Error::Internal("Failed to acquire write lock on registry".to_string()))?;
        
        // Create a more complex hierarchy:
        //
        //             Object
        //            /      \
        //     Container     Comparable
        //        /  \        /    \
        //     List   Map   Sortable Equatable
        //     /  \
        // Array  LinkedList
        
        registry_guard.register_interface("Object");
        registry_guard.register_interface("Container");
        registry_guard.register_interface("Comparable");
        registry_guard.register_interface("List");
        registry_guard.register_interface("Map");
        registry_guard.register_interface("Sortable");
        registry_guard.register_interface("Equatable");
        registry_guard.register_interface("Array");
        registry_guard.register_interface("LinkedList");
        
        registry_guard.register_extension("Container", "Object")?;
        registry_guard.register_extension("Comparable", "Object")?;
        registry_guard.register_extension("List", "Container")?;
        registry_guard.register_extension("Map", "Container")?;
        registry_guard.register_extension("Sortable", "Comparable")?;
        registry_guard.register_extension("Equatable", "Comparable")?;
        registry_guard.register_extension("Array", "List")?;
        registry_guard.register_extension("LinkedList", "List")?;
        
        // Add some diamond inheritance pattern:
        //       Iterable
        //      /        \
        // Sequence    Collection
        //      \        /
        //     ArrayList
        
        registry_guard.register_interface("Iterable");
        registry_guard.register_interface("Sequence");
        registry_guard.register_interface("Collection");
        registry_guard.register_interface("ArrayList");
        
        registry_guard.register_extension("Sequence", "Iterable")?;
        registry_guard.register_extension("Collection", "Iterable")?;
        registry_guard.register_extension("ArrayList", "Sequence")?;
        registry_guard.register_extension("ArrayList", "Collection")?;
        
        Ok(())
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::tests::common;
        
        #[test]
        fn test_create_test_registry() {
            let registry = create_test_registry();
            assert!(verify_test_registry(&registry).is_ok());
        }
        
        #[test]
        fn test_create_registry_with_visualization() {
            let registry = create_test_registry_with_visualization();
            let registry_arc = registry.clone() as Arc<RwLock<dyn InterfaceRegistryExtension + Send + Sync>>;
            assert!(verify_test_registry(&registry_arc).is_ok());
        }
        
        #[test]
        fn test_diamond_inheritance_registry() {
            let registry = create_diamond_inheritance_registry();
            let registry_guard = registry.read().unwrap();
            
            // Check if diamond inheritance pattern is created correctly
            assert!(registry_guard.has_extension("B", "A").unwrap());
            assert!(registry_guard.has_extension("C", "A").unwrap());
            assert!(registry_guard.has_extension("D", "B").unwrap());
            assert!(registry_guard.has_extension("D", "C").unwrap());
        }
        
        #[test]
        fn test_transitive_closure() {
            let registry = create_test_registry();
            let closure = calculate_transitive_closure(&registry).unwrap();
            
            // Check some expected transitive relationships
            assert!(closure.get("File").unwrap().contains("Reader"));
            assert!(closure.get("File").unwrap().contains("Writer"));
            assert!(closure.get("File").unwrap().contains("Closer"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::common;
    
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

// Re-export key components for convenience
pub use test_common::{
    create_test_registry,
    create_test_registry_with_visualization,
    create_diamond_inheritance_registry,
    create_test_registry_cache,
    verify_test_registry,
    calculate_transitive_closure,
    populate_complex_hierarchy
};