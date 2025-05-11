//! # Interface Type Registry Extension Checking
//!
//! This module provides the InterfaceTypeRegistryExtensionChecking trait for reliable inheritance
//! verification in interface type assertions with proper integration with the interface path finder.
//! It enables enhanced error diagnostics and comprehensive inheritance checking.

use std::collections::{HashMap, HashSet};
use std::fmt::{self, Display, Formatter};
use std::sync::Arc;
use tracing::{debug, error, info, instrument, trace, warn};

use crate::core::interface_registry_visualization::InterfaceRegistryExtensionWithVisualization;
use crate::error::Error;

/// A trait for comprehensive interface relationship checking with enhanced error diagnostics
/// that integrates with the interface path finder for type assertions.
pub trait InterfaceTypeRegistryExtensionChecking {
    /// Check if one interface extends another with enhanced error diagnostics
    fn check_interface_extension(&self, source: &str, target: &str) -> Result<bool, Error>;
    
    /// Find all interfaces that extend a given interface
    fn find_extending_interfaces(&self, interface: &str) -> Result<HashSet<String>, Error>;
    
    /// Find all interfaces that are extended by a given interface
    fn find_extended_interfaces(&self, interface: &str) -> Result<HashSet<String>, Error>;
    
    /// Get the complete extension hierarchy as a map
    fn get_extension_hierarchy(&self) -> Result<HashMap<String, Vec<String>>, Error>;
    
    /// Detect any cycles in the interface inheritance hierarchy
    fn detect_inheritance_cycles(&self) -> Result<Vec<Vec<String>>, Error>;
    
    /// Visualize the interface hierarchy using the specified format
    fn visualize_hierarchy(&self, max_depth: Option<usize>) -> Result<String, Error>;
    
    /// Generate a DOT graph representation of the interface hierarchy
    fn generate_hierarchy_dot_graph(&self) -> Result<String, Error>;
    
    /// Check if a particular interface exists in the registry
    fn interface_exists_in_registry(&self, interface: &str) -> Result<bool, Error>;
    
    /// Get all interfaces registered in the system
    fn get_all_interfaces_in_registry(&self) -> Result<HashSet<String>, Error>;
}

/// Implementation of InterfaceTypeRegistryExtensionChecking trait for LlvmCodeGenerator
#[derive(Debug)]
pub struct InterfaceTypeRegistryExtensionChecker {
    /// Reference to the interface registry visualization
    pub registry_visualization: Option<Arc<dyn InterfaceRegistryExtensionWithVisualization + Send + Sync>>,
}

impl InterfaceTypeRegistryExtensionChecker {
    /// Create a new instance with the provided registry visualization
    pub fn new(registry_visualization: Option<Arc<dyn InterfaceRegistryExtensionWithVisualization + Send + Sync>>) -> Self {
        Self {
            registry_visualization,
        }
    }
}

// Implementation of the extension checking trait for the checker
// This will be integrated with LlvmCodeGenerator via composition
impl InterfaceTypeRegistryExtensionChecking for InterfaceTypeRegistryExtensionChecker {
    #[instrument(skip(self), level = "debug")]
    fn check_interface_extension(&self, source: &str, target: &str) -> Result<bool, Error> {
        debug!("Checking if {} extends {}", source, target);
        
        // If source and target are the same, return true immediately
        if source == target {
            return Ok(true);
        }
        
        // Try to use the registry visualization if available
        if let Some(registry) = &self.registry_visualization {
            return registry.extends(source, target);
        }
        
        // Fallback behavior if registry not available
        warn!("Registry visualization not available for extension checking");
        Ok(false)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn find_extending_interfaces(&self, interface: &str) -> Result<HashSet<String>, Error> {
        debug!("Finding interfaces that extend {}", interface);
        
        // Try to use the registry visualization if available
        if let Some(registry) = &self.registry_visualization {
            return registry.get_all_extensions(interface);
        }
        
        // Fallback behavior
        warn!("Registry visualization not available for finding extending interfaces");
        Ok(HashSet::new())
    }
    
    #[instrument(skip(self), level = "debug")]
    fn find_extended_interfaces(&self, interface: &str) -> Result<HashSet<String>, Error> {
        debug!("Finding interfaces that are extended by {}", interface);
        
        // Try to use the registry visualization if available
        if let Some(registry) = &self.registry_visualization {
            return registry.get_all_implementors(interface);
        }
        
        // Fallback behavior
        warn!("Registry visualization not available for finding extended interfaces");
        Ok(HashSet::new())
    }
    
    #[instrument(skip(self), level = "debug")]
    fn get_extension_hierarchy(&self) -> Result<HashMap<String, Vec<String>>, Error> {
        debug!("Getting extension hierarchy");
        
        // Try to use the registry visualization if available
        if let Some(registry) = &self.registry_visualization {
            return registry.get_extension_hierarchy();
        }
        
        // Fallback behavior
        warn!("Registry visualization not available for getting extension hierarchy");
        Ok(HashMap::new())
    }
    
    #[instrument(skip(self), level = "debug")]
    fn detect_inheritance_cycles(&self) -> Result<Vec<Vec<String>>, Error> {
        debug!("Detecting inheritance cycles");
        
        // Try to use the registry visualization if available
        if let Some(registry) = &self.registry_visualization {
            return registry.detect_cycles();
        }
        
        // Fallback behavior
        warn!("Registry visualization not available for detecting cycles");
        Ok(Vec::new())
    }
    
    #[instrument(skip(self), level = "debug")]
    fn visualize_hierarchy(&self, max_depth: Option<usize>) -> Result<String, Error> {
        debug!("Visualizing hierarchy with max depth: {:?}", max_depth);
        
        // Try to use the registry visualization if available
        if let Some(registry) = &self.registry_visualization {
            // Get the hierarchy
            let hierarchy = registry.get_extension_hierarchy()?;
            
            // Create visualization options
            let options = crate::core::interface_registry_visualization::VisualizationOptions {
                include_cycles: true,
                max_depth,
                include_details: true,
            };
            
            // Generate ASCII tree visualization
            return registry.generate_ascii_tree(&hierarchy, &options);
        }
        
        // Fallback behavior
        warn!("Registry visualization not available for visualizing hierarchy");
        Ok(String::from("Interface hierarchy visualization not available"))
    }
    
    #[instrument(skip(self), level = "debug")]
    fn generate_hierarchy_dot_graph(&self) -> Result<String, Error> {
        debug!("Generating hierarchy DOT graph");
        
        // Try to use the registry visualization if available
        if let Some(registry) = &self.registry_visualization {
            return registry.visualize_hierarchy_dot();
        }
        
        // Fallback behavior
        warn!("Registry visualization not available for generating DOT graph");
        Ok(String::from("digraph interface_hierarchy {\n  // No data available\n}\n"))
    }
    
    #[instrument(skip(self), level = "debug")]
    fn interface_exists_in_registry(&self, interface: &str) -> Result<bool, Error> {
        debug!("Checking if interface exists in registry: {}", interface);
        
        // Try to use the registry visualization if available
        if let Some(registry) = &self.registry_visualization {
            return registry.interface_exists(interface);
        }
        
        // Fallback behavior
        warn!("Registry visualization not available for checking interface existence");
        Ok(false)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn get_all_interfaces_in_registry(&self) -> Result<HashSet<String>, Error> {
        debug!("Getting all interfaces in registry");
        
        // Try to use the registry visualization if available
        if let Some(registry) = &self.registry_visualization {
            return registry.get_all_interfaces();
        }
        
        // Fallback behavior
        warn!("Registry visualization not available for getting all interfaces");
        Ok(HashSet::new())
    }
}

/// Register the interface type registry extension checking components
pub fn register_interface_type_registry_extension_checking() {
    trace!("Interface type registry extension checking module registered");
    // This function is called during the compiler's initialization
    // to register this enhanced implementation for use throughout compilation
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Mock implementation of InterfaceRegistryExtensionWithVisualization for testing
    #[derive(Debug)]
    struct MockRegistry {
        extensions: HashMap<String, Vec<String>>,
    }
    
    impl MockRegistry {
        fn new() -> Self {
            let mut extensions = HashMap::new();
            
            // Set up some test data
            extensions.insert("Reader".to_string(), vec!["FileReader".to_string(), "NetworkReader".to_string()]);
            extensions.insert("FileReader".to_string(), vec!["JSONFileReader".to_string()]);
            extensions.insert("Serializable".to_string(), vec!["JSONSerializable".to_string()]);
            
            Self { extensions }
        }
    }
    
    impl InterfaceRegistryExtensionWithVisualization for MockRegistry {
        fn get_extension_hierarchy(&self) -> Result<HashMap<String, Vec<String>>, Error> {
            Ok(self.extensions.clone())
        }
        
        fn get_all_interfaces(&self) -> Result<HashSet<String>, Error> {
            let mut all_interfaces = HashSet::new();
            
            // Add all source interfaces
            for source in self.extensions.keys() {
                all_interfaces.insert(source.clone());
            }
            
            // Add all target interfaces
            for (_, targets) in &self.extensions {
                for target in targets {
                    all_interfaces.insert(target.clone());
                }
            }
            
            Ok(all_interfaces)
        }
        
        fn get_direct_extensions(&self, interface: &str) -> Result<Option<Vec<String>>, Error> {
            Ok(self.extensions.get(interface).cloned())
        }
        
        fn get_direct_implementors(&self, interface: &str) -> Result<Option<Vec<String>>, Error> {
            // Create reverse mapping
            let mut implementors = HashMap::new();
            
            for (source, targets) in &self.extensions {
                for target in targets {
                    implementors
                        .entry(target.clone())
                        .or_insert_with(Vec::new)
                        .push(source.clone());
                }
            }
            
            Ok(implementors.get(interface).cloned())
        }
        
        fn extends(&self, source: &str, target: &str) -> Result<bool, Error> {
            // If source and target are the same, return true
            if source == target {
                return Ok(true);
            }
            
            // Helper function for DFS
            fn dfs(
                extensions: &HashMap<String, Vec<String>>,
                current: &str,
                target: &str,
                visited: &mut HashSet<String>,
            ) -> bool {
                if current == target {
                    return true;
                }
                
                if visited.contains(current) {
                    return false;
                }
                
                visited.insert(current.to_string());
                
                if let Some(next_interfaces) = extensions.get(current) {
                    for next in next_interfaces {
                        if dfs(extensions, next, target, visited) {
                            return true;
                        }
                    }
                }
                
                false
            }
            
            let mut visited = HashSet::new();
            Ok(dfs(&self.extensions, source, target, &mut visited))
        }
        
        fn get_all_extensions(&self, interface: &str) -> Result<HashSet<String>, Error> {
            let mut result = HashSet::new();
            let mut queue = std::collections::VecDeque::new();
            let mut visited = HashSet::new();
            
            // Start with the interface itself
            queue.push_back(interface.to_string());
            visited.insert(interface.to_string());
            
            // Perform BFS
            while let Some(current) = queue.pop_front() {
                if let Some(extensions) = self.extensions.get(&current) {
                    for extension in extensions {
                        if !visited.contains(extension) {
                            visited.insert(extension.clone());
                            queue.push_back(extension.clone());
                            result.insert(extension.clone());
                        }
                    }
                }
            }
            
            Ok(result)
        }
        
        fn get_all_implementors(&self, interface: &str) -> Result<HashSet<String>, Error> {
            // Create reverse mapping
            let mut implementors = HashMap::new();
            
            for (source, targets) in &self.extensions {
                for target in targets {
                    implementors
                        .entry(target.clone())
                        .or_insert_with(Vec::new)
                        .push(source.clone());
                }
            }
            
            let mut result = HashSet::new();
            let mut queue = std::collections::VecDeque::new();
            let mut visited = HashSet::new();
            
            // Start with the interface itself
            queue.push_back(interface.to_string());
            visited.insert(interface.to_string());
            
            // Perform BFS
            while let Some(current) = queue.pop_front() {
                if let Some(impls) = implementors.get(&current) {
                    for impl_interface in impls {
                        if !visited.contains(impl_interface) {
                            visited.insert(impl_interface.clone());
                            queue.push_back(impl_interface.clone());
                            result.insert(impl_interface.clone());
                        }
                    }
                }
            }
            
            Ok(result)
        }
        
        fn find_inheritance_path(&self, source: &str, target: &str) -> Result<Vec<String>, Error> {
            // If source and target are the same, return a path with just one element
            if source == target {
                return Ok(vec![source.to_string()]);
            }
            
            // Use BFS to find the shortest path
            let mut visited = HashSet::new();
            let mut queue = std::collections::VecDeque::new();
            let mut parent_map = HashMap::new();
            
            // Start BFS from source
            queue.push_back(source.to_string());
            visited.insert(source.to_string());
            
            let mut found = false;
            
            // Perform BFS
            while let Some(current) = queue.pop_front() {
                if current == target {
                    found = true;
                    break;
                }
                
                if let Some(extensions) = self.extensions.get(&current) {
                    for extension in extensions {
                        if !visited.contains(extension) {
                            visited.insert(extension.clone());
                            queue.push_back(extension.clone());
                            parent_map.insert(extension.clone(), current.clone());
                        }
                    }
                }
            }
            
            if !found {
                return Err(Error::Compilation(format!(
                    "No inheritance path found from {} to {}",
                    source, target
                )));
            }
            
            // Reconstruct the path
            let mut path = Vec::new();
            let mut current = target.to_string();
            
            path.push(current.clone());
            
            while let Some(parent) = parent_map.get(&current) {
                path.push(parent.clone());
                current = parent.clone();
            }
            
            // Reverse the path to get it from source to target
            path.reverse();
            
            Ok(path)
        }
        
        fn find_all_inheritance_paths(&self, source: &str, target: &str) -> Result<Vec<Vec<String>>, Error> {
            // Just return a single path for simplicity in the mock
            let path = self.find_inheritance_path(source, target)?;
            Ok(vec![path])
        }
        
        fn detect_cycles(&self) -> Result<Vec<Vec<String>>, Error> {
            let mut cycles = Vec::new();
            let mut visited = HashSet::new();
            let mut path = Vec::new();
            let mut on_stack = HashSet::new();
            
            // Get all interfaces
            let all_interfaces = self.get_all_interfaces()?;
            
            // Helper function for cycle detection using DFS
            fn dfs_cycle(
                extensions: &HashMap<String, Vec<String>>,
                interface: &str,
                visited: &mut HashSet<String>,
                path: &mut Vec<String>,
                on_stack: &mut HashSet<String>,
                cycles: &mut Vec<Vec<String>>,
            ) {
                visited.insert(interface.to_string());
                path.push(interface.to_string());
                on_stack.insert(interface.to_string());
                
                if let Some(ext_interfaces) = extensions.get(interface) {
                    for ext in ext_interfaces {
                        if !visited.contains(ext) {
                            dfs_cycle(extensions, ext, visited, path, on_stack, cycles);
                        } else if on_stack.contains(ext) {
                            // Found a cycle
                            let cycle_start = path.iter().position(|x| x == ext).unwrap();
                            let cycle = path[cycle_start..].to_vec();
                            cycles.push(cycle);
                        }
                    }
                }
                
                // Backtrack
                path.pop();
                on_stack.remove(interface);
            }
            
            // Check each interface
            for interface in all_interfaces {
                if !visited.contains(&interface) {
                    dfs_cycle(
                        &self.extensions,
                        &interface,
                        &mut visited,
                        &mut path,
                        &mut on_stack,
                        &mut cycles,
                    );
                }
            }
            
            Ok(cycles)
        }
        
        fn visualize_hierarchy_ascii(&self) -> Result<String, Error> {
            Ok(String::from("Mock ASCII hierarchy visualization"))
        }
        
        fn visualize_hierarchy_dot(&self) -> Result<String, Error> {
            Ok(String::from("digraph mock_hierarchy { /* Mock hierarchy */ }"))
        }
        
        fn visualize_path_ascii(&self, path: &[String]) -> Result<String, Error> {
            Ok(format!("Mock ASCII path visualization for: {:?}", path))
        }
        
        fn visualize_path_dot(&self, path: &[String]) -> Result<String, Error> {
            Ok(format!("digraph mock_path {{ /* Mock path {:?} */ }}", path))
        }
        
        fn interface_exists(&self, interface: &str) -> Result<bool, Error> {
            // Check if the interface exists as a key in extensions or as a value in any extension list
            Ok(self.extensions.contains_key(interface) || 
               self.extensions.values().any(|v| v.contains(&interface.to_string())))
        }
        
        fn generate_ascii_tree(&self, _hierarchy: &HashMap<String, Vec<String>>, _options: &crate::core::interface_registry_visualization::VisualizationOptions) -> Result<String, Error> {
            Ok(String::from("Mock ASCII tree"))
        }
        
        fn generate_dot_graph(&self, _hierarchy: &HashMap<String, Vec<String>>, _options: &crate::core::interface_registry_visualization::VisualizationOptions) -> Result<String, Error> {
            Ok(String::from("digraph mock_graph { /* Mock graph */ }"))
        }
        
        fn generate_json_representation(&self, _hierarchy: &HashMap<String, Vec<String>>, _options: &crate::core::interface_registry_visualization::VisualizationOptions) -> Result<String, Error> {
            Ok(String::from("{\"mock\": true}"))
        }
        
        fn is_visualization_initialized(&self) -> Result<bool, Error> {
            Ok(true)
        }
        
        fn set_visualization_initialized(&self, _initialized: bool) -> Result<(), Error> {
            Ok(())
        }
        
        fn register_extension(&self, _source: &str, _target: &str) -> Result<(), Error> {
            // This is a read-only mock, so we don't actually register anything
            Ok(())
        }
    }
    
    #[test]
    fn test_interface_type_registry_extension_checker() {
        // Create a mock registry
        let mock_registry = Arc::new(MockRegistry::new());
        
        // Create the checker
        let checker = InterfaceTypeRegistryExtensionChecker::new(Some(mock_registry.clone() as Arc<dyn InterfaceRegistryExtensionWithVisualization + Send + Sync>));
        
        // Test basic extension checking
        assert!(checker.check_interface_extension("FileReader", "Reader").unwrap());
        assert!(checker.check_interface_extension("JSONFileReader", "Reader").unwrap());
        assert!(!checker.check_interface_extension("Reader", "FileReader").unwrap());
        
        // Test interface existence checking
        assert!(checker.interface_exists_in_registry("Reader").unwrap());
        assert!(checker.interface_exists_in_registry("FileReader").unwrap());
        assert!(checker.interface_exists_in_registry("JSONFileReader").unwrap());
        assert!(!checker.interface_exists_in_registry("NonExistentInterface").unwrap());
        
        // Test getting all interfaces
        let all_interfaces = checker.get_all_interfaces_in_registry().unwrap();
        assert!(all_interfaces.contains("Reader"));
        assert!(all_interfaces.contains("FileReader"));
        assert!(all_interfaces.contains("JSONFileReader"));
        
        // Test finding extending interfaces
        let reader_extensions = checker.find_extending_interfaces("Reader").unwrap();
        assert!(reader_extensions.contains("FileReader"));
        assert!(reader_extensions.contains("JSONFileReader"));
        assert!(reader_extensions.contains("NetworkReader"));
        
        // Test hierarchy visualization
        let hierarchy_visualization = checker.visualize_hierarchy(Some(2)).unwrap();
        assert!(hierarchy_visualization.contains("Mock ASCII tree"));
        
        // Test DOT graph generation
        let dot_graph = checker.generate_hierarchy_dot_graph().unwrap();
        assert!(dot_graph.contains("digraph"));
    }
}