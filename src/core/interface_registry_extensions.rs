//! # Interface Registry Extensions
//!
//! This module provides support for tracking interface extension relationships
//! in the compiler's interface registry. It allows the compiler to determine
//! if one interface extends another, either directly or indirectly.
//!
//! The registry maintains a graph of interface extension relationships and
//! provides methods to query these relationships efficiently.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, RwLock};
use tracing::{debug, info, trace, warn, instrument};

use crate::error::Error;

/// Tracks interface extension relationships
pub struct InterfaceExtensionRegistry {
    /// Maps interface names to the set of interfaces they directly extend
    direct_extensions: HashMap<String, HashSet<String>>,
    
    /// Maps interface names to the set of interfaces they indirectly extend (transitive closure)
    transitive_extensions: HashMap<String, HashSet<String>>,
    
    /// Maps interface names to the set of interfaces that directly extend them
    reverse_extensions: HashMap<String, HashSet<String>>,
}

impl InterfaceExtensionRegistry {
    /// Creates a new empty interface extension registry
    pub fn new() -> Self {
        InterfaceExtensionRegistry {
            direct_extensions: HashMap::new(),
            transitive_extensions: HashMap::new(),
            reverse_extensions: HashMap::new(),
        }
    }
    
    /// Registers that one interface extends another
    pub fn register_extension(&mut self, interface: &str, extends: &str) -> Result<(), Error> {
        trace!("Registering that {} extends {}", interface, extends);
        
        // Skip self-extension (not allowed)
        if interface == extends {
            return Err(Error::SemanticError(format!(
                "Interface {} cannot extend itself", interface
            )));
        }
        
        // Update direct extensions mapping
        self.direct_extensions
            .entry(interface.to_string())
            .or_insert_with(HashSet::new)
            .insert(extends.to_string());
        
        // Update reverse extensions mapping
        self.reverse_extensions
            .entry(extends.to_string())
            .or_insert_with(HashSet::new)
            .insert(interface.to_string());
        
        // Clear transitive extensions cache to force recomputation
        self.transitive_extensions.clear();
        
        Ok(())
    }
    
    /// Gets the set of interfaces that a given interface directly extends
    pub fn get_direct_extensions(&self, interface: &str) -> Option<HashSet<String>> {
        self.direct_extensions.get(interface).cloned()
    }
    
    /// Gets the set of interfaces that directly extend a given interface
    pub fn get_direct_implementors(&self, interface: &str) -> Option<HashSet<String>> {
        self.reverse_extensions.get(interface).cloned()
    }
    
    /// Checks if one interface extends another (directly or indirectly)
    #[instrument(level = "trace", skip(self))]
    pub fn does_extend(&mut self, interface: &str, extends: &str) -> bool {
        trace!("Checking if {} extends {}", interface, extends);
        
        // Trivial case: every interface extends itself
        if interface == extends {
            return true;
        }
        
        // Check the transitive extensions cache
        if let Some(extensions) = self.transitive_extensions.get(interface) {
            if extensions.contains(extends) {
                return true;
            }
        }
        
        // If not in cache, compute transitive extensions for this interface
        let transitive = self.compute_transitive_extensions(interface);
        
        // Cache the result
        self.transitive_extensions.insert(interface.to_string(), transitive.clone());
        
        // Check if the target interface is in the transitive set
        transitive.contains(extends)
    }
    
    /// Computes the transitive closure of interface extensions
    fn compute_transitive_extensions(&self, interface: &str) -> HashSet<String> {
        let mut result = HashSet::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        
        // Start with direct extensions
        if let Some(direct) = self.direct_extensions.get(interface) {
            for ext in direct {
                queue.push_back(ext.clone());
                result.insert(ext.clone());
                visited.insert(ext.clone());
            }
        }
        
        // Breadth-first search to find all transitive extensions
        while let Some(current) = queue.pop_front() {
            if let Some(next_level) = self.direct_extensions.get(&current) {
                for ext in next_level {
                    if !visited.contains(ext) {
                        queue.push_back(ext.clone());
                        result.insert(ext.clone());
                        visited.insert(ext.clone());
                    }
                }
            }
        }
        
        result
    }
    
    /// Gets all interfaces in the registry
    pub fn get_all_interfaces(&self) -> HashSet<String> {
        let mut result = HashSet::new();
        
        // Add all interfaces from direct_extensions (as keys)
        for key in self.direct_extensions.keys() {
            result.insert(key.clone());
        }
        
        // Add all interfaces from direct_extensions (as values)
        for values in self.direct_extensions.values() {
            for value in values {
                result.insert(value.clone());
            }
        }
        
        // Add all interfaces from reverse_extensions (as keys)
        for key in self.reverse_extensions.keys() {
            result.insert(key.clone());
        }
        
        result
    }
    
    /// Gets the complete extension hierarchy for visualization or debugging
    pub fn get_extension_hierarchy(&self) -> HashMap<String, HashSet<String>> {
        self.direct_extensions.clone()
    }
}

/// Thread-safe version of InterfaceExtensionRegistry
pub struct ThreadSafeInterfaceExtensionRegistry {
    /// The registry wrapped in a read-write lock
    registry: Arc<RwLock<InterfaceExtensionRegistry>>,
}

impl ThreadSafeInterfaceExtensionRegistry {
    /// Creates a new empty thread-safe interface extension registry
    pub fn new() -> Self {
        ThreadSafeInterfaceExtensionRegistry {
            registry: Arc::new(RwLock::new(InterfaceExtensionRegistry::new())),
        }
    }
    
    /// Registers that one interface extends another
    pub fn register_extension(&self, interface: &str, extends: &str) -> Result<(), Error> {
        let mut registry = self.registry.write().map_err(|e| {
            Error::Compilation(format!(
                "Failed to acquire write lock on interface registry: {}", e
            ))
        })?;
        
        registry.register_extension(interface, extends)
    }
    
    /// Gets the set of interfaces that a given interface directly extends
    pub fn get_direct_extensions(&self, interface: &str) -> Result<Option<HashSet<String>>, Error> {
        let registry = self.registry.read().map_err(|e| {
            Error::Compilation(format!(
                "Failed to acquire read lock on interface registry: {}", e
            ))
        })?;
        
        Ok(registry.get_direct_extensions(interface))
    }
    
    /// Gets the set of interfaces that directly extend a given interface
    pub fn get_direct_implementors(&self, interface: &str) -> Result<Option<HashSet<String>>, Error> {
        let registry = self.registry.read().map_err(|e| {
            Error::Compilation(format!(
                "Failed to acquire read lock on interface registry: {}", e
            ))
        })?;
        
        Ok(registry.get_direct_implementors(interface))
    }
    
    /// Checks if one interface extends another (directly or indirectly)
    pub fn does_extend(&self, interface: &str, extends: &str) -> Result<bool, Error> {
        let mut registry = self.registry.write().map_err(|e| {
            Error::Compilation(format!(
                "Failed to acquire write lock on interface registry: {}", e
            ))
        })?;
        
        Ok(registry.does_extend(interface, extends))
    }
    
    /// Gets all interfaces in the registry
    pub fn get_all_interfaces(&self) -> Result<HashSet<String>, Error> {
        let registry = self.registry.read().map_err(|e| {
            Error::Compilation(format!(
                "Failed to acquire read lock on interface registry: {}", e
            ))
        })?;
        
        Ok(registry.get_all_interfaces())
    }
    
    /// Gets the complete extension hierarchy for visualization or debugging
    pub fn get_extension_hierarchy(&self) -> Result<HashMap<String, HashSet<String>>, Error> {
        let registry = self.registry.read().map_err(|e| {
            Error::Compilation(format!(
                "Failed to acquire read lock on interface registry: {}", e
            ))
        })?;
        
        Ok(registry.get_extension_hierarchy())
    }
}