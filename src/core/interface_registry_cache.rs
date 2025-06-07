//! # Interface Registry Cache
//!
//! This module provides caching functionality for interface registry operations to improve performance
//! of interface type assertions and relationship checks.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use crate::core::interface_registry_extensions::InterfaceRegistryExtension;
use crate::core::interface_registry_visualization::InterfaceRegistryVisualization;
use crate::error::Error;
use tracing::{debug, trace, instrument};

/// Re-export LRU cache implementations
pub use crate::core::interface_registry_lru_cache::ThreadSafeInterfaceRegistryLruCache;

/// Thread-safe implementation of a cached interface registry
#[derive(Debug)]
pub struct ThreadSafeInterfaceRegistryCache {
    /// The underlying registry to delegate to
    registry: Arc<RwLock<dyn InterfaceRegistryExtension + Send + Sync>>,
    
    /// Cache for direct extensions (interface -> set of direct extensions)
    direct_extensions_cache: RwLock<HashMap<String, HashSet<String>>>,
    
    /// Cache for extends relationship (source, target) -> bool
    extends_cache: RwLock<HashMap<(String, String), bool>>,
    
    /// Cache for inheritance paths (source, target) -> path
    path_cache: RwLock<HashMap<(String, String), Vec<String>>>,
}

impl ThreadSafeInterfaceRegistryCache {
    /// Create a new cached registry wrapping the given registry
    pub fn new(registry: Arc<RwLock<dyn InterfaceRegistryExtension + Send + Sync>>) -> Self {
        Self {
            registry,
            direct_extensions_cache: RwLock::new(HashMap::new()),
            extends_cache: RwLock::new(HashMap::new()),
            path_cache: RwLock::new(HashMap::new()),
        }
    }
    
    /// Clear all caches
    #[instrument(skip(self), level = "debug")]
    pub fn clear_caches(&self) {
        let mut direct_extensions = self.direct_extensions_cache.write().unwrap();
        direct_extensions.clear();
        
        let mut extends = self.extends_cache.write().unwrap();
        extends.clear();
        
        let mut path = self.path_cache.write().unwrap();
        path.clear();
        
        debug!("All caches cleared");
    }
}

impl InterfaceRegistryExtension for ThreadSafeInterfaceRegistryCache {
    #[instrument(skip(self), level = "debug")]
    fn register_interface(&mut self, name: &str) {
        // Clear caches on modification
        self.clear_caches();
        
        // Delegate to underlying registry
        self.registry.write().unwrap().register_interface(name);
    }
    
    #[instrument(skip(self), level = "debug")]
    fn register_extension(&mut self, source: &str, target: &str) -> Result<(), Error> {
        // Clear caches on modification
        self.clear_caches();
        
        // Delegate to underlying registry
        self.registry.write().unwrap().register_extension(source, target)
    }
    
    #[instrument(skip(self), level = "trace")]
    fn has_extension(&self, source: &str, target: &str) -> Result<bool, Error> {
        // Check cache first
        let cache_key = (source.to_string(), target.to_string());
        let extends_cache = self.extends_cache.read().unwrap();
        
        if let Some(&result) = extends_cache.get(&cache_key) {
            trace!("Cache hit for has_extension({}, {})", source, target);
            return Ok(result);
        }
        
        drop(extends_cache);
        
        // Delegate to underlying registry
        let result = self.registry.read().unwrap().has_extension(source, target)?;
        
        // Cache result
        let mut extends_cache = self.extends_cache.write().unwrap();
        extends_cache.insert(cache_key, result);
        
        Ok(result)
    }
    
    #[instrument(skip(self), level = "trace")]
    fn get_all_interfaces(&self) -> Result<HashSet<String>, Error> {
        // No caching for this operation
        self.registry.read().unwrap().get_all_interfaces()
    }
    
    #[instrument(skip(self), level = "trace")]
    fn get_direct_extensions(&self, interface: &str) -> Result<Option<Vec<String>>, Error> {
        // Check cache first
        let cache_key = interface.to_string();
        let direct_extensions_cache = self.direct_extensions_cache.read().unwrap();
        
        if let Some(extensions) = direct_extensions_cache.get(&cache_key) {
            trace!("Cache hit for get_direct_extensions({})", interface);
            return Ok(Some(extensions.iter().cloned().collect()));
        }
        
        drop(direct_extensions_cache);
        
        // Delegate to underlying registry
        let result = self.registry.read().unwrap().get_direct_extensions(interface)?;
        
        // Cache result if there are extensions
        if let Some(extensions) = &result {
            let mut direct_extensions_cache = self.direct_extensions_cache.write().unwrap();
            direct_extensions_cache.insert(cache_key, extensions.iter().cloned().collect());
        }
        
        Ok(result)
    }
    
    #[instrument(skip(self), level = "trace")]
    fn get_direct_implementers(&self, interface: &str) -> Result<Option<Vec<String>>, Error> {
        // No caching for this operation, convert HashSet to Vec
        let result = self.registry.read().unwrap().get_direct_implementers(interface)?;
        Ok(result.map(|set| set.into_iter().collect()))
    }
    
    #[instrument(skip(self), level = "trace")]
    fn find_inheritance_path(&self, source: &str, target: &str) -> Result<Vec<String>, Error> {
        self.registry.read().unwrap().find_inheritance_path(source, target)
    }
    
    #[instrument(skip(self), level = "trace")]
    fn find_all_inheritance_paths(&self, source: &str, target: &str) -> Result<Vec<Vec<String>>, Error> {
        self.registry.read().unwrap().find_all_inheritance_paths(source, target)
    }
    
    #[instrument(skip(self), level = "trace")]
    fn get_all_extensions(&self, interface: &str) -> Result<HashSet<String>, Error> {
        self.registry.read().unwrap().get_all_extensions(interface)
    }
    
    #[instrument(skip(self), level = "trace")]
    fn get_all_implementors(&self, interface: &str) -> Result<HashSet<String>, Error> {
        self.registry.read().unwrap().get_all_implementors(interface)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn extends(&self, source: &str, target: &str) -> Result<bool, Error> {
        // Check cache first
        let cache_key = (source.to_string(), target.to_string());
        let extends_cache = self.extends_cache.read().unwrap();
        
        if let Some(&result) = extends_cache.get(&cache_key) {
            trace!("Cache hit for extends({}, {})", source, target);
            return Ok(result);
        }
        
        drop(extends_cache);
        
        // Delegate to underlying registry
        let result = self.registry.read().unwrap().extends(source, target)?;
        
        // Cache result
        let mut extends_cache = self.extends_cache.write().unwrap();
        extends_cache.insert(cache_key, result);
        
        Ok(result)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn find_common_ancestor(&self, a: &str, b: &str) -> Result<Option<String>, Error> {
        // No caching for this operation
        self.registry.read().unwrap().find_common_ancestor(a, b)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn find_longest_path(&self, source: &str, target: &str) -> Result<Option<Vec<String>>, Error> {
        // Check cache first
        let cache_key = (source.to_string(), target.to_string());
        let path_cache = self.path_cache.read().unwrap();
        
        if let Some(path) = path_cache.get(&cache_key) {
            trace!("Cache hit for find_longest_path({}, {})", source, target);
            return Ok(Some(path.clone()));
        }
        
        drop(path_cache);
        
        // Delegate to underlying registry
        let result = self.registry.read().unwrap().find_longest_path(source, target)?;
        
        // Cache result
        if let Some(path) = &result {
            let mut path_cache = self.path_cache.write().unwrap();
            path_cache.insert(cache_key, path.clone());
        }
        
        Ok(result)
    }
    
    fn get_direct_implementors(&self, interface: &str) -> Result<Option<Vec<String>>, Error> {
        match self.get_direct_implementers(interface)? {
            Some(implementers) => Ok(Some(implementers.into_iter().collect())),
            None => Ok(None),
        }
    }
    
    fn get_extension_hierarchy(&self) -> Result<HashMap<String, Vec<String>>, Error> {
        use std::collections::HashMap;
        let mut hierarchy = HashMap::new();
        
        if let Ok(interfaces) = self.get_all_interfaces() {
            for interface in interfaces {
                if let Ok(Some(extensions)) = self.get_direct_extensions(&interface) {
                    hierarchy.insert(interface, extensions);
                }
            }
        }
        
        Ok(hierarchy)
    }
}

/// Cache-related helper functions
mod cache_utils {
    //! Internal helper functions for caching operations
    
    /// Why test the interface registry cache?
    /// Testing the registry cache is important because:
    /// 1. It ensures cached results match the underlying registry
    /// 2. It verifies cache invalidation on registry modifications
    /// 3. It confirms performance improvements with benchmarks
    /// 4. It tests for race conditions in multi-threaded environments
    /// 5. It validates memory usage remains reasonable with large type hierarchies
    #[cfg(test)]
    fn test_requirements() {}
    
    // Reserved for future helper functions
}