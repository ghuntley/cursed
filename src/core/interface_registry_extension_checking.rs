//! # Interface Registry Extension Checking
//!
//! This module provides functionality for checking and visualizing extension relationships
//! between interfaces in the type registry. It helps with runtime type assertions and
//! provides detailed error messages when type assertions fail.
//!
//! ## Key Features
//!
//! 1. Thread-safe implementation for concurrent compilation
//! 2. Comprehensive error handling with rich context
//! 3. Visual representation of interface inheritance paths
//! 4. Detection of reversed inheritance relationships (common error)
//! 5. Support for interface hierarchy visualization
//! 6. Integration with the interface registry for type assertions

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Write;
use std::sync::{Arc, RwLock};
use tracing::{debug, error, info, instrument, trace, warn};

use crate::error::Error;
use crate::core::interface_registry_visualization::InterfaceRegistryExtensionWithVisualization;

/// A trait for checking and visualizing extension relationships between interfaces
pub trait InterfaceTypeRegistryExtensionChecking {
    /// Check if an interface extends another interface with detailed error messages
    fn check_extension_with_details(&self, source: &str, target: &str) -> Result<(bool, Option<String>), Error>;
    
    /// Visualize the path between two interfaces with better formatting
    fn visualize_path(&self, source: &str, target: &str) -> Result<String, Error>;
    
    /// Find alternative extension paths between interfaces
    fn find_alternative_paths(&self, source: &str, target: &str, max_paths: usize) -> Result<Vec<Vec<String>>, Error>;
    
    /// Check if inheritance relationship is reversed (common error)
    fn check_reversed_relationship(&self, source: &str, target: &str) -> Result<(bool, String), Error>;
    
    /// Generate a visualization of the entire interface hierarchy
    fn generate_hierarchy_visualization(&self, format: &str) -> Result<String, Error>;
    
    /// Find all interfaces that share a common ancestor
    fn find_common_ancestors(&self, interfaces: &[String]) -> Result<HashSet<String>, Error>;
}

/// Implementation of extension checking for thread-safe interface registry
impl InterfaceTypeRegistryExtensionChecking for Arc<RwLock<dyn InterfaceRegistryExtensionWithVisualization + Send + Sync>> {
    #[instrument(level = "debug", skip(self))]
    fn check_extension_with_details(&self, source: &str, target: &str) -> Result<(bool, Option<String>), Error> {
        debug!("Checking extension with details: {} → {}", source, target);
        
        // Lock the registry for reading
        let registry = self.read().map_err(|e| {
            Error::Compilation(format!("Failed to acquire read lock on interface registry: {}", e))
        })?;
        
        // If source and target are the same, return true immediately
        if source == target {
            return Ok((true, None));
        }
        
        // Check if both interfaces exist
        if !registry.interface_exists(source)? {
            return Err(Error::Compilation(format!(
                "Source interface '{}' does not exist in the registry",
                source
            )));
        }
        
        if !registry.interface_exists(target)? {
            return Err(Error::Compilation(format!(
                "Target interface '{}' does not exist in the registry",
                target
            )));
        }
        
        // Try to find an inheritance path
        match registry.find_inheritance_path(source, target) {
            Ok(path) => {
                // Create a visual representation of the path
                let path_visual = registry.visualize_path_ascii(&path)?;
                Ok((true, Some(path_visual)))
            },
            Err(_) => {
                // Check if there's a reversed relationship
                let (reversed, _) = self.check_reversed_relationship(source, target)?;
                
                if reversed {
                    // Generate a helpful error message
                    let error_message = format!(
                        "Type assertion error: Interface '{}' does not extend '{}'. However, '{}' does extend '{}'.",
                        source, target, target, source
                    );
                    
                    Ok((false, Some(error_message)))
                } else {
                    Ok((false, None))
                }
            }
        }
    }
    
    #[instrument(level = "debug", skip(self))]
    fn visualize_path(&self, source: &str, target: &str) -> Result<String, Error> {
        debug!("Visualizing path from {} to {}", source, target);
        
        // Lock the registry for reading
        let registry = self.read().map_err(|e| {
            Error::Compilation(format!("Failed to acquire read lock on interface registry: {}", e))
        })?;
        
        // Find the inheritance path
        let path = registry.find_inheritance_path(source, target)?;
        
        // Generate a visual representation
        let mut result = String::from("Interface Inheritance Path:\n");
        
        for (i, interface) in path.iter().enumerate() {
            // Add appropriate box-drawing characters based on position
            if i == 0 {
                // First element
                result.push_str(&format!("\u{250c}\u{2500}\u{2500} {}\n", interface));
            } else if i == path.len() - 1 {
                // Last element
                result.push_str(&format!("\u{2514}\u{2500}\u{2500} {}\n", interface));
            } else {
                // Middle elements
                result.push_str(&format!("\u{251c}\u{2500}\u{2500} {}\n", interface));
            }
            
            // Add connecting lines between elements
            if i < path.len() - 1 {
                result.push_str("\u{2502}\n");
            }
        }
        
        Ok(result)
    }
    
    #[instrument(level = "debug", skip(self))]
    fn find_alternative_paths(&self, source: &str, target: &str, max_paths: usize) -> Result<Vec<Vec<String>>, Error> {
        debug!("Finding up to {} alternative paths from {} to {}", max_paths, source, target);
        
        // Lock the registry for reading
        let registry = self.read().map_err(|e| {
            Error::Compilation(format!("Failed to acquire read lock on interface registry: {}", e))
        })?;
        
        // Check if both interfaces exist
        if !registry.interface_exists(source)? {
            return Err(Error::Compilation(format!(
                "Source interface '{}' does not exist in the registry",
                source
            )));
        }
        
        if !registry.interface_exists(target)? {
            return Err(Error::Compilation(format!(
                "Target interface '{}' does not exist in the registry",
                target
            )));
        }
        
        // If source and target are the same, return a single path
        if source == target {
            return Ok(vec![vec![source.to_string()]]);
        }
        
        // Try to find all possible paths using BFS with path recording
        let mut paths = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        
        // Start with the source
        queue.push_back(vec![source.to_string()]);
        
        while let Some(current_path) = queue.pop_front() {
            let current_interface = current_path.last().unwrap().clone();
            
            // Check if we've reached the target
            if &current_interface == target {
                paths.push(current_path.clone());
                
                // Stop if we've found enough paths
                if paths.len() >= max_paths {
                    break;
                }
                
                continue;
            }
            
            // Get direct extensions of the current interface
            if let Some(extensions) = registry.get_direct_extensions(&current_interface)? {
                for extension in extensions {
                    // Avoid cycles and already visited paths
                    if !current_path.contains(&extension) {
                        let mut new_path = current_path.clone();
                        new_path.push(extension.clone());
                        
                        let path_key = new_path.join("->");
                        if !visited.contains(&path_key) {
                            visited.insert(path_key);
                            queue.push_back(new_path);
                        }
                    }
                }
            }
        }
        
        // Check if we found any paths
        if paths.is_empty() {
            // Try finding paths through a common parent
            let all_interfaces = registry.get_all_interfaces()?;
            
            for interface in all_interfaces {
                if interface == source || interface == target {
                    continue;
                }
                
                // Check if both source and target extend this interface
                let source_extends = match registry.extends(source, &interface) {
                    Ok(extends) => extends,
                    Err(_) => continue,
                };
                
                let target_extends = match registry.extends(target, &interface) {
                    Ok(extends) => extends,
                    Err(_) => continue,
                };
                
                if source_extends && target_extends {
                    // Found a common parent, create paths
                    let path_source_to_common = registry.find_inheritance_path(source, &interface)?;
                    let path_target_to_common = registry.find_inheritance_path(target, &interface)?;
                    
                    // Combine paths
                    let mut full_path = path_source_to_common;
                    // Remove the common interface to avoid duplication
                    full_path.pop();
                    // Add the target path in reverse
                    let mut reversed_target_path = path_target_to_common;
                    reversed_target_path.reverse();
                    full_path.extend(reversed_target_path);
                    
                    paths.push(full_path);
                    
                    // Stop if we've found enough paths
                    if paths.len() >= max_paths {
                        break;
                    }
                }
            }
        }
        
        Ok(paths)
    }
    
    #[instrument(level = "debug", skip(self))]
    fn check_reversed_relationship(&self, source: &str, target: &str) -> Result<(bool, String), Error> {
        debug!("Checking for reversed relationship: {} → {}", source, target);
        
        // Lock the registry for reading
        let registry = self.read().map_err(|e| {
            Error::Compilation(format!("Failed to acquire read lock on interface registry: {}", e))
        })?;
        
        // Check if both interfaces exist
        if !registry.interface_exists(source)? {
            return Err(Error::Compilation(format!(
                "Source interface '{}' does not exist in the registry",
                source
            )));
        }
        
        if !registry.interface_exists(target)? {
            return Err(Error::Compilation(format!(
                "Target interface '{}' does not exist in the registry",
                target
            )));
        }
        
        // Try to find a path in the reverse direction
        match registry.find_inheritance_path(target, source) {
            Ok(path) => {
                // Found a reversed relationship
                let path_visual = registry.visualize_path_ascii(&path)?;
                
                let message = format!(
                    "Reversed inheritance relationship detected!\n\n
                    '{}' does not extend '{}', but '{}' actually extends '{}'.\n\n
                    The correct inheritance path is:\n{}\n\n
                    You may need to swap the types in your type assertion.",
                    source, target, target, source, path_visual
                );
                
                Ok((true, message))
            },
            Err(_) => {
                // No relationship in either direction
                let message = format!(
                    "No inheritance relationship found between '{}' and '{}'. 
                    These interfaces are unrelated.",
                    source, target
                );
                
                Ok((false, message))
            }
        }
    }
    
    #[instrument(level = "debug", skip(self))]
    fn generate_hierarchy_visualization(&self, format: &str) -> Result<String, Error> {
        debug!("Generating hierarchy visualization in {} format", format);
        
        // Lock the registry for reading
        let registry = self.read().map_err(|e| {
            Error::Compilation(format!("Failed to acquire read lock on interface registry: {}", e))
        })?;
        
        // Generate the appropriate visualization based on format
        match format.to_lowercase().as_str() {
            "ascii" => registry.visualize_hierarchy_ascii(),
            "dot" => registry.visualize_hierarchy_dot(),
            _ => Err(Error::Compilation(format!(
                "Unsupported visualization format: {}", format
            ))),
        }
    }
    
    #[instrument(level = "debug", skip(self))]
    fn find_common_ancestors(&self, interfaces: &[String]) -> Result<HashSet<String>, Error> {
        debug!("Finding common ancestors for {} interfaces", interfaces.len());
        
        if interfaces.is_empty() {
            return Ok(HashSet::new());
        }
        
        // Lock the registry for reading
        let registry = self.read().map_err(|e| {
            Error::Compilation(format!("Failed to acquire read lock on interface registry: {}", e))
        })?;
        
        // Find ancestors of the first interface
        let mut common_ancestors = HashSet::new();
        
        // Helper function to find all ancestors of an interface
        fn find_ancestors(
            registry: &dyn InterfaceRegistryExtensionWithVisualization,
            interface: &str,
            ancestors: &mut HashSet<String>,
        ) -> Result<(), Error> {
            // Add the interface itself as an ancestor
            ancestors.insert(interface.to_string());
            
            // Add all interfaces that this interface extends
            if let Some(extensions) = registry.get_direct_extensions(interface)? {
                for extension in extensions {
                    ancestors.insert(extension.clone());
                    find_ancestors(registry, &extension, ancestors)?;
                }
            }
            
            Ok(())
        }
        
        // Find ancestors of the first interface
        find_ancestors(&*registry, &interfaces[0], &mut common_ancestors)?;
        
        // Intersect with ancestors of remaining interfaces
        for interface in &interfaces[1..] {
            let mut interface_ancestors = HashSet::new();
            find_ancestors(&*registry, interface, &mut interface_ancestors)?;
            
            // Update common ancestors to be the intersection
            common_ancestors = common_ancestors
                .intersection(&interface_ancestors)
                .cloned()
                .collect();
            
            // Early exit if there are no common ancestors
            if common_ancestors.is_empty() {
                break;
            }
        }
        
        Ok(common_ancestors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::interface_registry_visualization::ThreadSafeInterfaceRegistryVisualization;
    use std::sync::Arc;
    
    #[test]
    fn test_check_extension_with_details() {
        // Create a registry with some test interfaces
        let registry = ThreadSafeInterfaceRegistryVisualization::new();
        let arc_registry: Arc<RwLock<dyn InterfaceRegistryExtensionWithVisualization + Send + Sync>> = 
            Arc::new(RwLock::new(registry));
        
        // Register interfaces and extensions
        {
            let mut registry = arc_registry.write().unwrap();
            registry.register_extension("Animal", "Mammal").unwrap();
            registry.register_extension("Mammal", "Dog").unwrap();
            registry.register_extension("Animal", "Bird").unwrap();
        }
        
        // Test a valid extension relationship
        let (extends, details) = arc_registry.check_extension_with_details("Animal", "Dog").unwrap();
        assert!(extends, "Animal should extend Dog");
        assert!(details.is_some(), "Should provide path details");
        
        // Test a non-existent extension relationship
        let (extends, details) = arc_registry.check_extension_with_details("Dog", "Bird").unwrap();
        assert!(!extends, "Dog should not extend Bird");
        assert!(details.is_none() || details.unwrap().contains("unrelated"), 
                "Should indicate unrelated interfaces");
        
        // Test a reversed relationship
        let (extends, details) = arc_registry.check_extension_with_details("Dog", "Animal").unwrap();
        assert!(!extends, "Dog should not extend Animal");
        assert!(details.is_some() && details.unwrap().contains("does not extend"), 
                "Should indicate reversed relationship");
    }
    
    #[test]
    fn test_visualize_path() {
        // Create a registry with some test interfaces
        let registry = ThreadSafeInterfaceRegistryVisualization::new();
        let arc_registry: Arc<RwLock<dyn InterfaceRegistryExtensionWithVisualization + Send + Sync>> = 
            Arc::new(RwLock::new(registry));
        
        // Register interfaces and extensions
        {
            let mut registry = arc_registry.write().unwrap();
            registry.register_extension("Base", "Derived1").unwrap();
            registry.register_extension("Derived1", "Derived2").unwrap();
            registry.register_extension("Derived2", "Derived3").unwrap();
        }
        
        // Test visualizing a path
        let visual = arc_registry.visualize_path("Base", "Derived3").unwrap();
        
        // Verify the visualization contains expected content
        assert!(visual.contains("Interface Inheritance Path"), "Should have a title");
        assert!(visual.contains("Base"), "Should include Base");
        assert!(visual.contains("Derived1"), "Should include Derived1");
        assert!(visual.contains("Derived2"), "Should include Derived2");
        assert!(visual.contains("Derived3"), "Should include Derived3");
        
        // Should have box-drawing characters
        assert!(visual.contains("\u{250c}") || visual.contains("┌"), "Should have top connector");
        assert!(visual.contains("\u{251c}") || visual.contains("├"), "Should have middle connector");
        assert!(visual.contains("\u{2514}") || visual.contains("└"), "Should have bottom connector");
    }
    
    #[test]
    fn test_find_alternative_paths() {
        // Create a registry with interfaces having multiple paths
        let registry = ThreadSafeInterfaceRegistryVisualization::new();
        let arc_registry: Arc<RwLock<dyn InterfaceRegistryExtensionWithVisualization + Send + Sync>> = 
            Arc::new(RwLock::new(registry));
        
        // Register interfaces and extensions to create multiple paths
        {
            let mut registry = arc_registry.write().unwrap();
            // Path 1: A -> B -> D
            registry.register_extension("A", "B").unwrap();
            registry.register_extension("B", "D").unwrap();
            
            // Path 2: A -> C -> D
            registry.register_extension("A", "C").unwrap();
            registry.register_extension("C", "D").unwrap();
        }
        
        // Test finding alternative paths
        let paths = arc_registry.find_alternative_paths("A", "D", 5).unwrap();
        
        // Should find both paths
        assert_eq!(paths.len(), 2, "Should find 2 paths");
        
        // Verify first path
        assert!(paths[0].contains(&"A".to_string()), "First path should contain A");
        assert!(paths[0].contains(&"D".to_string()), "First path should contain D");
        
        // Verify second path
        assert!(paths[1].contains(&"A".to_string()), "Second path should contain A");
        assert!(paths[1].contains(&"D".to_string()), "Second path should contain D");
        
        // Verify the paths are different
        assert_ne!(paths[0], paths[1], "Paths should be different");
    }
    
    #[test]
    fn test_check_reversed_relationship() {
        // Create a registry with some test interfaces
        let registry = ThreadSafeInterfaceRegistryVisualization::new();
        let arc_registry: Arc<RwLock<dyn InterfaceRegistryExtensionWithVisualization + Send + Sync>> = 
            Arc::new(RwLock::new(registry));
        
        // Register interfaces and extensions
        {
            let mut registry = arc_registry.write().unwrap();
            registry.register_extension("Parent", "Child").unwrap();
            registry.register_extension("Child", "GrandChild").unwrap();
        }
        
        // Test a correctly ordered relationship
        let (reversed, _) = arc_registry.check_reversed_relationship("Parent", "Child").unwrap();
        assert!(!reversed, "Parent -> Child is correctly ordered");
        
        // Test a reversed relationship
        let (reversed, message) = arc_registry.check_reversed_relationship("Child", "Parent").unwrap();
        assert!(reversed, "Child -> Parent is reversed");
        assert!(message.contains("Reversed inheritance"), "Should mention reversed inheritance");
        assert!(message.contains("Parent actually extends Child"), "Should explain the correct order");
        
        // Test unrelated interfaces
        let registry = ThreadSafeInterfaceRegistryVisualization::new();
        let arc_registry: Arc<RwLock<dyn InterfaceRegistryExtensionWithVisualization + Send + Sync>> = 
            Arc::new(RwLock::new(registry));
        
        {
            let mut registry = arc_registry.write().unwrap();
            registry.register_extension("A", "B").unwrap();
            registry.register_extension("C", "D").unwrap();
        }
        
        let (reversed, message) = arc_registry.check_reversed_relationship("A", "C").unwrap();
        assert!(!reversed, "A and C are unrelated");
        assert!(message.contains("unrelated"), "Should mention they are unrelated");
    }
    
    #[test]
    fn test_generate_hierarchy_visualization() {
        // Create a registry with a simple hierarchy
        let registry = ThreadSafeInterfaceRegistryVisualization::new();
        let arc_registry: Arc<RwLock<dyn InterfaceRegistryExtensionWithVisualization + Send + Sync>> = 
            Arc::new(RwLock::new(registry));
        
        // Register interfaces and extensions
        {
            let mut registry = arc_registry.write().unwrap();
            registry.register_extension("Root", "Child1").unwrap();
            registry.register_extension("Root", "Child2").unwrap();
            registry.register_extension("Child1", "GrandChild1").unwrap();
            registry.register_extension("Child2", "GrandChild2").unwrap();
        }
        
        // Test ASCII visualization
        let ascii = arc_registry.generate_hierarchy_visualization("ascii").unwrap();
        assert!(ascii.contains("Interface Hierarchy"), "Should have a title");
        assert!(ascii.contains("Root"), "Should include Root");
        assert!(ascii.contains("Child1"), "Should include Child1");
        assert!(ascii.contains("Child2"), "Should include Child2");
        
        // Test DOT visualization
        let dot = arc_registry.generate_hierarchy_visualization("dot").unwrap();
        assert!(dot.contains("digraph interface_hierarchy"), "Should be a DOT digraph");
        assert!(dot.contains("Root"), "Should include Root");
        assert!(dot.contains("Child1"), "Should include Child1");
        assert!(dot.contains("Child2"), "Should include Child2");
        assert!(dot.contains("Root -> Child1"), "Should have edge from Root to Child1");
        assert!(dot.contains("Root -> Child2"), "Should have edge from Root to Child2");
    }
    
    #[test]
    fn test_find_common_ancestors() {
        // Create a registry with interfaces having common ancestors
        let registry = ThreadSafeInterfaceRegistryVisualization::new();
        let arc_registry: Arc<RwLock<dyn InterfaceRegistryExtensionWithVisualization + Send + Sync>> = 
            Arc::new(RwLock::new(registry));
        
        // Register interfaces and extensions
        {
            let mut registry = arc_registry.write().unwrap();
            registry.register_extension("Animal", "Mammal").unwrap();
            registry.register_extension("Animal", "Bird").unwrap();
            registry.register_extension("Mammal", "Dog").unwrap();
            registry.register_extension("Mammal", "Cat").unwrap();
            registry.register_extension("Bird", "Eagle").unwrap();
            registry.register_extension("Bird", "Penguin").unwrap();
        }
        
        // Test finding common ancestors for Dog and Cat
        let ancestors = arc_registry.find_common_ancestors(&["Dog".to_string(), "Cat".to_string()]).unwrap();
        assert!(ancestors.contains("Animal"), "Animal should be a common ancestor");
        assert!(ancestors.contains("Mammal"), "Mammal should be a common ancestor");
        assert_eq!(ancestors.len(), 2, "Should find 2 common ancestors");
        
        // Test finding common ancestors for Dog and Eagle
        let ancestors = arc_registry.find_common_ancestors(&["Dog".to_string(), "Eagle".to_string()]).unwrap();
        assert!(ancestors.contains("Animal"), "Animal should be a common ancestor");
        assert_eq!(ancestors.len(), 1, "Should find 1 common ancestor");
        
        // Test finding common ancestors for unrelated interfaces
        let unrelated = arc_registry.find_common_ancestors(&["UnknownA".to_string(), "UnknownB".to_string()]).unwrap();
        assert!(unrelated.is_empty(), "Unrelated interfaces should have no common ancestors");
    }
}