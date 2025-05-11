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