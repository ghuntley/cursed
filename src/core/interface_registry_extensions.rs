//! Interface Extension Registry
//!
//! This module enhances the interface registry with support for interface extensions
//! and inheritance relationships. It tracks which interfaces extend other interfaces,
//! allowing for nested interface type assertions and interface hierarchy checks.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use tracing::{debug, trace, warn, info, instrument};

use crate::error::Error;
use crate::core::interface_registry::InterfaceRegistry;

/// Extension to the InterfaceRegistry that adds support for interface inheritance
#[derive(Debug, Default, Clone)]
pub struct InterfaceExtensionRegistry {
    /// Maps an interface name to a set of interfaces it directly extends
    extends: HashMap<String, HashSet<String>>,
    
    /// Maps an interface name to a set of interfaces that directly extend it
    extended_by: HashMap<String, HashSet<String>>,
    
    /// Cache of fully resolved interface extension chains
    extension_cache: HashMap<String, HashSet<String>>,
}

impl InterfaceExtensionRegistry {
    /// Create a new empty interface extension registry
    pub fn new() -> Self {
        Self {
            extends: HashMap::new(),
            extended_by: HashMap::new(),
            extension_cache: HashMap::new(),
        }
    }
    
    /// Register that an interface extends another interface
    pub fn register_extension(&mut self, interface_name: String, extends_interface: String) {
        // Record in the extends map
        self.extends
            .entry(interface_name.clone())
            .or_insert_with(HashSet::new)
            .insert(extends_interface.clone());
        
        // Record in the extended_by map
        self.extended_by
            .entry(extends_interface)
            .or_insert_with(HashSet::new)
            .insert(interface_name);
        
        // Clear the cache since we've modified the extension relationships
        self.extension_cache.clear();
        
        debug!("Registered interface extension: {} extends {}", interface_name, extends_interface);
    }
    
    /// Get all interfaces that an interface directly extends
    pub fn get_direct_extensions(&self, interface_name: &str) -> HashSet<String> {
        self.extends
            .get(interface_name)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Get all interfaces that directly extend an interface
    pub fn get_direct_extenders(&self, interface_name: &str) -> HashSet<String> {
        self.extended_by
            .get(interface_name)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Get all interfaces that an interface extends, directly or indirectly
    #[instrument(skip(self), level = "debug")]
    pub fn get_all_extensions(&mut self, interface_name: &str) -> HashSet<String> {
        // Check if we have a cached result
        if let Some(cached) = self.extension_cache.get(interface_name) {
            return cached.clone();
        }
        
        // Start with direct extensions
        let mut result = self.get_direct_extensions(interface_name);
        let mut to_process: Vec<String> = result.iter().cloned().collect();
        
        // Iteratively add all indirect extensions
        while let Some(current) = to_process.pop() {
            let direct_extensions = self.get_direct_extensions(&current);
            
            for extension in direct_extensions {
                if !result.contains(&extension) {
                    result.insert(extension.clone());
                    to_process.push(extension);
                }
            }
        }
        
        // Cache the result
        self.extension_cache.insert(interface_name.to_string(), result.clone());
        
        result
    }
    
    /// Check if one interface extends another, directly or indirectly
    pub fn check_extends(&mut self, interface_name: &str, potential_parent: &str) -> bool {
        // Get all extensions
        let all_extensions = self.get_all_extensions(interface_name);
        
        // Check if the potential parent is among them
        all_extensions.contains(potential_parent)
    }
    
    /// Populate the registry with common interface extension relationships
    pub fn populate_with_defaults(&mut self) {
        // Examples of common interface extension relationships
        
        // Reader/Writer hierarchy
        self.register_extension("BufferedReader".to_string(), "Reader".to_string());
        self.register_extension("FileReader".to_string(), "Reader".to_string());
        self.register_extension("NetworkReader".to_string(), "Reader".to_string());
        
        self.register_extension("BufferedWriter".to_string(), "Writer".to_string());
        self.register_extension("FileWriter".to_string(), "Writer".to_string());
        self.register_extension("NetworkWriter".to_string(), "Writer".to_string());
        
        // IO interface that extends both Reader and Writer
        self.register_extension("IO".to_string(), "Reader".to_string());
        self.register_extension("IO".to_string(), "Writer".to_string());
        
        // Collection hierarchy
        self.register_extension("List".to_string(), "Collection".to_string());
        self.register_extension("Set".to_string(), "Collection".to_string());
        self.register_extension("Map".to_string(), "Collection".to_string());
        
        self.register_extension("ArrayList".to_string(), "List".to_string());
        self.register_extension("LinkedList".to_string(), "List".to_string());
        
        self.register_extension("HashSet".to_string(), "Set".to_string());
        self.register_extension("TreeSet".to_string(), "Set".to_string());
        
        self.register_extension("HashMap".to_string(), "Map".to_string());
        self.register_extension("TreeMap".to_string(), "Map".to_string());
        
        // UI component hierarchy
        self.register_extension("Component".to_string(), "Visible".to_string());
        self.register_extension("Container".to_string(), "Component".to_string());
        self.register_extension("Button".to_string(), "Component".to_string());
        self.register_extension("Panel".to_string(), "Container".to_string());
        self.register_extension("Frame".to_string(), "Container".to_string());
        
        debug!("Populated interface extension registry with default relationships");
    }
    
    /// Get all interfaces registered in the system
    pub fn get_all_interfaces(&self) -> HashSet<String> {
        let mut result = HashSet::new();
        
        // Add all interfaces that extend other interfaces
        for interface in self.extends.keys() {
            result.insert(interface.clone());
        }
        
        // Add all interfaces that are extended by other interfaces
        for interface in self.extended_by.keys() {
            result.insert(interface.clone());
        }
        
        result
    }
}

/// Extension trait to add interface extension support to InterfaceRegistry
pub trait InterfaceRegistryExtensions {
    /// Register that an interface extends another interface
    fn register_interface_extension(
        &mut self,
        interface_name: String,
        extends_interface: String
    );
    
    /// Get all interfaces that an interface extends, directly or indirectly
    fn get_interface_extensions(&self, interface_name: &str) -> Vec<String>;
    
    /// Check if one interface extends another, directly or indirectly
    fn check_interface_extension(
        &self,
        interface_name: &str,
        potential_parent: &str
    ) -> bool;
    
    /// Get all interfaces registered in the system
    fn get_all_interfaces(&self) -> Vec<String>;
}

/// Implementation of InterfaceRegistryExtensions for InterfaceRegistry
impl InterfaceRegistryExtensions for InterfaceRegistry {
    fn register_interface_extension(
        &mut self,
        interface_name: String,
        extends_interface: String
    ) {
        // Get or create the extension registry
        let mut extension_registry = self.get_extension_registry_mut();
        extension_registry.register_extension(interface_name, extends_interface);
    }
    
    fn get_interface_extensions(&self, interface_name: &str) -> Vec<String> {
        // Get the extension registry
        let mut extension_registry = self.get_extension_registry_mut();
        extension_registry.get_all_extensions(interface_name)
            .into_iter()
            .collect()
    }
    
    fn check_interface_extension(
        &self,
        interface_name: &str,
        potential_parent: &str
    ) -> bool {
        // Get the extension registry
        let mut extension_registry = self.get_extension_registry_mut();
        extension_registry.check_extends(interface_name, potential_parent)
    }
    
    fn get_all_interfaces(&self) -> Vec<String> {
        // Get the extension registry
        let extension_registry = self.get_extension_registry();
        extension_registry.get_all_interfaces()
            .into_iter()
            .collect()
    }
}

// Helper methods for InterfaceRegistry to work with the extension registry
impl InterfaceRegistry {
    /// Get or create the extension registry (mutable)
    fn get_extension_registry_mut(&self) -> &mut InterfaceExtensionRegistry {
        // In a real implementation, this would use a mutable field or a thread-safe reference
        // For testing purposes, we'll use a thread-local storage or a static ONCE_CELL
        thread_local! {
            static EXTENSION_REGISTRY: std::cell::RefCell<InterfaceExtensionRegistry> = 
                std::cell::RefCell::new(InterfaceExtensionRegistry::new());
        }
        
        EXTENSION_REGISTRY.with(|registry| {
            registry.borrow_mut().deref_mut()
        })
    }
    
    /// Get the extension registry (immutable)
    fn get_extension_registry(&self) -> &InterfaceExtensionRegistry {
        // Similar to the mutable version, but returns an immutable reference
        thread_local! {
            static EXTENSION_REGISTRY: std::cell::RefCell<InterfaceExtensionRegistry> = 
                std::cell::RefCell::new(InterfaceExtensionRegistry::new());
        }
        
        EXTENSION_REGISTRY.with(|registry| {
            registry.borrow().deref()
        })
    }
}

// Add this trait and implementation for InterfaceRegistry
use std::ops::{Deref, DerefMut};

impl Deref for InterfaceExtensionRegistry {
    type Target = InterfaceExtensionRegistry;
    
    fn deref(&self) -> &Self::Target {
        self
    }
}

impl DerefMut for InterfaceExtensionRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_interface_extension_registry() {
        // Create a new extension registry
        let mut registry = InterfaceExtensionRegistry::new();
        
        // Register some extensions
        registry.register_extension("ArrayList".to_string(), "List".to_string());
        registry.register_extension("List".to_string(), "Collection".to_string());
        
        // Test direct extensions
        let list_extensions = registry.get_direct_extensions("List");
        assert!(list_extensions.contains("Collection"));
        
        // Test all extensions
        let arraylist_extensions = registry.get_all_extensions("ArrayList");
        assert!(arraylist_extensions.contains("List"));
        assert!(arraylist_extensions.contains("Collection"));
        
        // Test check_extends
        assert!(registry.check_extends("ArrayList", "Collection"));
        assert!(registry.check_extends("ArrayList", "List"));
        assert!(!registry.check_extends("Collection", "ArrayList"));
    }
    
    #[test]
    fn test_interface_registry_extensions() {
        // Create a new interface registry
        let mut registry = InterfaceRegistry::new();
        
        // Register some interface extensions
        registry.register_interface_extension("ArrayList".to_string(), "List".to_string());
        registry.register_interface_extension("List".to_string(), "Collection".to_string());
        
        // Test getting extensions
        let arraylist_extensions = registry.get_interface_extensions("ArrayList");
        assert!(arraylist_extensions.contains(&"List".to_string()));
        assert!(arraylist_extensions.contains(&"Collection".to_string()));
        
        // Test checking extensions
        assert!(registry.check_interface_extension("ArrayList", "Collection"));
        assert!(registry.check_interface_extension("ArrayList", "List"));
        assert!(!registry.check_interface_extension("Collection", "ArrayList"));
    }
}