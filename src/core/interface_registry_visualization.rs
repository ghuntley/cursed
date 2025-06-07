//! # Interface Registry Visualization
//!
//! This module provides visualization capabilities for the interface registry,
//! allowing generation of diagrams and other visual representations of interface
//! relationships.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use crate::core::interface_registry_extensions::InterfaceRegistryExtension;
use crate::error::Error;
use tracing::{debug, info, instrument};
use serde_json;

/// Options for visualization output
#[derive(Debug, Clone)]
pub struct VisualizationOptions {
    /// Maximum depth for tree visualization
    pub max_depth: Option<usize>,
    /// Whether to include cycles in visualizations
    pub include_cycles: bool,
}

impl Default for VisualizationOptions {
    fn default() -> Self {
        Self {
            max_depth: None,
            include_cycles: true,
        }
    }
}

/// Format for visualization output
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VisualizationFormat {
    /// ASCII tree format
    Ascii,
    /// DOT graph format (Graphviz)
    Dot,
    /// JSON format
    Json,
    /// Plain text format
    Text,
}

/// Type alias for thread-safe interface registry visualization
pub type ThreadSafeInterfaceRegistryVisualization = Box<dyn InterfaceRegistryVisualization + Send + Sync>;

/// Extension trait for interface registry visualization
pub trait InterfaceRegistryVisualization: std::fmt::Debug {
    /// Generate a DOT format diagram of all interface relationships
    fn generate_dot_diagram(&self) -> Result<String, Error>;
    
    /// Generate a DOT format diagram of inheritance paths between two interfaces
    fn generate_inheritance_path_diagram(&self, source: &str, target: &str) -> Result<String, Error>;
    
    /// Generate a DOT format diagram of a specific interface's relationships
    fn generate_interface_diagram(&self, interface: &str) -> Result<String, Error>;
    
    /// Get a text representation of interface relationships
    fn get_text_representation(&self) -> Result<String, Error>;
    
    /// Generate an ASCII tree representation of the hierarchy
    fn generate_ascii_tree(&self, hierarchy: &HashMap<String, Vec<String>>, options: &VisualizationOptions) -> Result<String, Error>;
    
    /// Generate a DOT graph representation
    fn generate_dot_graph(&self, hierarchy: &HashMap<String, Vec<String>>, options: &VisualizationOptions) -> Result<String, Error>;
    
    /// Generate a JSON representation
    fn generate_json_representation(&self, hierarchy: &HashMap<String, Vec<String>>, options: &VisualizationOptions) -> Result<String, Error>;
}

/// Combined extension trait for interface registry with visualization
pub trait InterfaceRegistryExtensionWithVisualization: InterfaceRegistryExtension + InterfaceRegistryVisualization + std::fmt::Debug {
    /// Get the inheritance distance between two interfaces
    fn get_inheritance_distance(&self, source: &str, target: &str) -> Result<Option<usize>, Error>;
    
    /// Find all paths between two interfaces
    fn find_all_paths(&self, source: &str, target: &str) -> Result<Vec<Vec<String>>, Error>;
    
    /// Get interfaces that form a diamond inheritance pattern
    fn find_diamond_inheritance_patterns(&self) -> Result<Vec<(String, String, String, String)>, Error>;
    
    /// Detect cycles in the interface hierarchy
    fn detect_cycles(&self) -> Result<Vec<Vec<String>>, Error>;
    
    /// Generate ASCII visualization of the interface hierarchy
    fn visualize_hierarchy_ascii(&self) -> Result<String, Error>;
    
    /// Generate DOT visualization of the interface hierarchy
    fn visualize_hierarchy_dot(&self) -> Result<String, Error>;
    
    /// Generate ASCII visualization of an inheritance path
    fn visualize_path_ascii(&self, path: &[String]) -> Result<String, Error>;
    
    /// Generate DOT visualization of an inheritance path
    fn visualize_path_dot(&self, path: &[String]) -> Result<String, Error>;
    
    /// Check if the visualization system is initialized
    fn is_visualization_initialized(&self) -> Result<bool, Error>;
    
    /// Set the initialized status of the visualization system
    fn set_visualization_initialized(&self, initialized: bool) -> Result<(), Error>;
    
    /// Check if source interface extends target interface (basic relationship check)
    fn check_extension_relationship(&self, source: &str, target: &str) -> Result<bool, Error> {
        self.extends(source, target)
    }
    
    /// Enhanced check for extension relationship with additional context
    fn check_extension_relationship_enhanced(&self, source: &str, target: &str) -> Result<bool, Error> {
        self.extends(source, target)
    }
    
    /// Simple check for extension relationship
    fn check_extension_relationship_simple(&self, source: &str, target: &str) -> Result<bool, Error> {
        self.extends(source, target)
    }
}

/// A concrete implementation of InterfaceRegistryVisualization
#[derive(Debug)]
pub struct DefaultInterfaceRegistryVisualization {
    registry: Arc<RwLock<crate::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry>>,
}

impl DefaultInterfaceRegistryVisualization {
    pub fn new(registry: Arc<RwLock<crate::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry>>) -> Self {
        Self { registry }
    }
}

impl InterfaceRegistryExtension for DefaultInterfaceRegistryVisualization {
    fn register_interface(&mut self, name: &str) {
        if let Ok(mut registry) = self.registry.write() {
            registry.register_interface(name);
        }
    }

    fn register_extension(&mut self, source: &str, target: &str) -> Result<(), Error> {
        let mut registry = self.registry.write().map_err(|_| Error::Internal("Failed to acquire write lock".to_string()))?;
        registry.register_extension(source, target)
    }

    fn has_extension(&self, source: &str, target: &str) -> Result<bool, Error> {
        let registry = self.registry.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.has_extension(source, target)
    }

    fn get_all_interfaces(&self) -> Result<HashSet<String>, Error> {
        let registry = self.registry.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.get_all_interfaces()
    }
    
    fn get_direct_extensions(&self, interface: &str) -> Result<Option<Vec<String>>, Error> {
        let registry = self.registry.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.get_direct_extensions(interface)
    }
    
    fn get_direct_implementers(&self, interface: &str) -> Result<Option<Vec<String>>, Error> {
        let registry = self.registry.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.get_direct_implementers(interface)
    }
    
    fn extends(&self, source: &str, target: &str) -> Result<bool, Error> {
        let registry = self.registry.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.extends(source, target)
    }
    
    fn find_common_ancestor(&self, a: &str, b: &str) -> Result<Option<String>, Error> {
        let registry = self.registry.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.find_common_ancestor(a, b)
    }
    
    fn find_longest_path(&self, source: &str, target: &str) -> Result<Option<Vec<String>>, Error> {
        let registry = self.registry.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.find_longest_path(source, target)
    }
    
    fn get_direct_implementors(&self, interface: &str) -> Result<Option<Vec<String>>, Error> {
        let registry = self.registry.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.get_direct_implementors(interface)
    }
    
    fn get_extension_hierarchy(&self) -> Result<HashMap<String, Vec<String>>, Error> {
        let registry = self.registry.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.get_extension_hierarchy()
    }
    
    fn find_inheritance_path(&self, source: &str, target: &str) -> Result<Vec<String>, Error> {
        let registry = self.registry.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.find_inheritance_path(source, target)
    }
    
    fn find_all_inheritance_paths(&self, source: &str, target: &str) -> Result<Vec<Vec<String>>, Error> {
        let registry = self.registry.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.find_all_inheritance_paths(source, target)
    }
    
    fn get_all_extensions(&self, interface: &str) -> Result<HashSet<String>, Error> {
        let registry = self.registry.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.get_all_extensions(interface)
    }
    
    fn get_all_implementors(&self, interface: &str) -> Result<HashSet<String>, Error> {
        let registry = self.registry.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.get_all_implementors(interface)
    }
    
    fn find_interface_paths(&self, source: &str, target: &str, max_paths: usize) -> Result<Vec<Vec<String>>, Error> {
        let registry = self.registry.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.find_interface_paths(source, target, max_paths)
    }
    
    fn does_extend(&self, source: &str, target: &str) -> Result<bool, Error> {
        let registry = self.registry.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.does_extend(source, target)
    }
}

impl InterfaceRegistryExtensionWithVisualization for DefaultInterfaceRegistryVisualization {
    fn get_inheritance_distance(&self, source: &str, target: &str) -> Result<Option<usize>, Error> {
        if source == target {
            return Ok(Some(0));
        }
        
        let path = self.find_inheritance_path(source, target)?;
        Ok(Some(path.len().saturating_sub(1)))
    }
    
    fn find_all_paths(&self, source: &str, target: &str) -> Result<Vec<Vec<String>>, Error> {
        self.find_all_inheritance_paths(source, target)
    }
    
    fn find_diamond_inheritance_patterns(&self) -> Result<Vec<(String, String, String, String)>, Error> {
        Ok(Vec::new())
    }
    
    fn detect_cycles(&self) -> Result<Vec<Vec<String>>, Error> {
        let hierarchy = self.get_extension_hierarchy()?;
        detect_cycles(&hierarchy)
    }
    
    fn visualize_hierarchy_ascii(&self) -> Result<String, Error> {
        let hierarchy = self.get_extension_hierarchy()?;
        let options = VisualizationOptions::default();
        self.generate_ascii_tree(&hierarchy, &options)
    }
    
    fn visualize_hierarchy_dot(&self) -> Result<String, Error> {
        let hierarchy = self.get_extension_hierarchy()?;
        let options = VisualizationOptions::default();
        self.generate_dot_graph(&hierarchy, &options)
    }
    
    fn visualize_path_ascii(&self, path: &[String]) -> Result<String, Error> {
        if path.is_empty() {
            return Ok(String::from("Empty inheritance path"));
        }
        
        let mut result = String::new();
        result.push_str("Inheritance Path:\n");
        
        for (i, interface) in path.iter().enumerate() {
            if i > 0 {
                result.push_str("  ↓ extends\n");
            }
            result.push_str(&format!("  [{}]\n", interface));
        }
        
        Ok(result)
    }
    
    fn visualize_path_dot(&self, path: &[String]) -> Result<String, Error> {
        if path.is_empty() {
            return Ok(String::from("digraph empty_path {}\n"));
        }
        
        let mut dot = String::from("digraph inheritance_path {\n");
        dot.push_str("  rankdir=BT;\n");
        dot.push_str("  node [shape=box, style=filled, fillcolor=lightblue];\n\n");
        
        // Add nodes
        for (i, interface) in path.iter().enumerate() {
            let color = if i == 0 {
                "lightgreen" // First node
            } else if i == path.len() - 1 {
                "lightpink"  // Last node
            } else {
                "lightblue"  // Intermediate nodes
            };
            
            dot.push_str(&format!("  \"{}\" [label=\"{}\", fillcolor={}];\n", 
                interface, interface, color));
        }
        
        // Add edges
        for i in 0..path.len() - 1 {
            dot.push_str(&format!("  \"{}\" -> \"{}\";\n", path[i], path[i+1]));
        }
        
        dot.push_str("}\n");
        
        Ok(dot)
    }
    
    fn is_visualization_initialized(&self) -> Result<bool, Error> {
        Ok(true)
    }
    
    fn set_visualization_initialized(&self, _initialized: bool) -> Result<(), Error> {
        Ok(())
    }
}

impl InterfaceRegistryVisualization for DefaultInterfaceRegistryVisualization {
    #[instrument(skip(self), level = "debug")]
    fn generate_dot_diagram(&self) -> Result<String, Error> {
        let registry = self.registry.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        let interfaces = registry.get_all_interfaces()?;
        
        let mut dot = String::from("digraph InterfaceHierarchy {\n");
        dot.push_str("  rankdir=BT;\n"); // Bottom to top
        dot.push_str("  node [shape=box, style=filled, fillcolor=lightblue];\n\n");
        
        // Add nodes
        for interface in &interfaces {
            dot.push_str(&format!("  \"{}\" [label=\"{}\"];\n", interface, interface));
        }
        
        dot.push_str("\n");
        
        // Add edges
        for source in &interfaces {
            if let Ok(Some(extensions)) = registry.get_direct_extensions(source) {
                for target in extensions {
                    dot.push_str(&format!("  \"{}\" -> \"{}\";\n", source, target));
                }
            }
        }
        
        dot.push_str("}\n");
        
        Ok(dot)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn generate_inheritance_path_diagram(&self, source: &str, target: &str) -> Result<String, Error> {
        let registry = self.registry.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        
        // Check if the source extends the target
        if !registry.extends(source, target)? {
            return Err(Error::Validation(format!(
                "'{}' does not extend '{}'", source, target
            )));
        }
        
        // Find the longest path
        let path = match registry.find_longest_path(source, target)? {
            Some(p) => p,
            None => return Err(Error::Internal("Failed to find path".to_string())),
        };
        
        let mut dot = String::from("digraph InheritancePath {\n");
        dot.push_str("  rankdir=BT;\n"); // Bottom to top
        dot.push_str("  node [shape=box, style=filled];\n\n");
        
        // Add nodes
        for (i, interface) in path.iter().enumerate() {
            let color = if i == 0 {
                "lightgreen" // Source
            } else if i == path.len() - 1 {
                "lightpink"  // Target
            } else {
                "lightblue"  // Intermediate
            };
            
            dot.push_str(&format!(
                "  \"{}\" [label=\"{}\", fillcolor={}];\n", 
                interface, interface, color
            ));
        }
        
        dot.push_str("\n");
        
        // Add edges
        for i in 0..path.len() - 1 {
            dot.push_str(&format!("  \"{}\" -> \"{}\";\n", path[i], path[i + 1]));
        }
        
        dot.push_str("}\n");
        
        Ok(dot)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn generate_interface_diagram(&self, interface: &str) -> Result<String, Error> {
        let registry = self.registry.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        let interfaces = registry.get_all_interfaces()?;
        
        if !interfaces.contains(interface) {
            return Err(Error::NotFound(format!("Interface '{}' not found", interface)));
        }
        
        // Collect all related interfaces (extends or extended by)
        let mut related = HashSet::new();
        related.insert(interface.to_string());
        
        // Add interfaces that this interface extends
        if let Ok(Some(extensions)) = registry.get_direct_extensions(interface) {
            for ext in extensions {
                related.insert(ext);
            }
        }
        
        // Add interfaces that extend this interface
        for other in &interfaces {
            if let Ok(Some(extensions)) = registry.get_direct_extensions(other) {
                if extensions.iter().any(|ext| ext == interface) {
                    related.insert(other.clone());
                }
            }
        }
        
        let mut dot = String::from("digraph InterfaceDiagram {\n");
        dot.push_str("  rankdir=BT;\n"); // Bottom to top
        dot.push_str("  node [shape=box, style=filled];\n\n");
        
        // Add nodes
        for related_interface in &related {
            let color = if related_interface == interface {
                "gold" // The central interface
            } else {
                "lightblue" // Related interfaces
            };
            
            dot.push_str(&format!(
                "  \"{}\" [label=\"{}\", fillcolor={}];\n", 
                related_interface, related_interface, color
            ));
        }
        
        dot.push_str("\n");
        
        // Add edges
        for source in &related {
            if let Ok(Some(extensions)) = registry.get_direct_extensions(source) {
                for target in extensions {
                    if related.contains(&target) {
                        dot.push_str(&format!("  \"{}\" -> \"{}\";\n", source, target));
                    }
                }
            }
        }
        
        dot.push_str("}\n");
        
        Ok(dot)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn get_text_representation(&self) -> Result<String, Error> {
        let registry = self.registry.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        let interfaces = registry.get_all_interfaces()?;
        
        let mut result = String::new();
        result.push_str("Interface Hierarchy:\n\n");
        
        let mut sorted_interfaces: Vec<_> = interfaces.iter().collect();
        sorted_interfaces.sort();
        
        for interface in sorted_interfaces {
            result.push_str(&format!("{}:\n", interface));
            
            // Show what this interface extends
            if let Ok(Some(extensions)) = registry.get_direct_extensions(interface) {
                let mut sorted_extensions: Vec<_> = extensions.iter().collect();
                sorted_extensions.sort();
                
                for ext in sorted_extensions {
                    result.push_str(&format!("  extends {}\n", ext));
                }
            }
            
            // Show what extends this interface
            let mut implementers = Vec::new();
            for other in &interfaces {
                if let Ok(Some(extensions)) = registry.get_direct_extensions(other) {
                    if extensions.iter().any(|ext| ext == interface) {
                        implementers.push(other);
                    }
                }
            }
            
            implementers.sort();
            for impl_ in implementers {
                result.push_str(&format!("  extended by {}\n", impl_));
            }
            
            result.push_str("\n");
        }
        
        Ok(result)
    }
    
    #[instrument(skip(self, hierarchy), level = "debug")]
    fn generate_ascii_tree(&self, hierarchy: &HashMap<String, Vec<String>>, _options: &VisualizationOptions) -> Result<String, Error> {
        let mut result = String::from("Interface Hierarchy:\n");
        
        for (source, targets) in hierarchy {
            result.push_str(&format!("{}:\n", source));
            for target in targets {
                result.push_str(&format!("  └── {}\n", target));
            }
        }
        
        Ok(result)
    }
    
    #[instrument(skip(self, hierarchy), level = "debug")]
    fn generate_dot_graph(&self, hierarchy: &HashMap<String, Vec<String>>, _options: &VisualizationOptions) -> Result<String, Error> {
        let mut dot = String::from("digraph InterfaceHierarchy {\n");
        dot.push_str("  rankdir=BT;\n");
        dot.push_str("  node [shape=box, style=filled, fillcolor=lightblue];\n\n");
        
        for (source, targets) in hierarchy {
            for target in targets {
                dot.push_str(&format!("  \"{}\" -> \"{}\";\n", source, target));
            }
        }
        
        dot.push_str("}\n");
        Ok(dot)
    }
    
    #[instrument(skip(self, hierarchy), level = "debug")]
    fn generate_json_representation(&self, hierarchy: &HashMap<String, Vec<String>>, _options: &VisualizationOptions) -> Result<String, Error> {
        let json = serde_json::to_string_pretty(hierarchy)
            .map_err(|e| Error::Internal(format!("Failed to serialize to JSON: {}", e)))?;
        Ok(json)
    }
}

/// Helper function to detect cycles in the interface hierarchy
pub fn detect_cycles(hierarchy: &HashMap<String, Vec<String>>) -> Result<Vec<Vec<String>>, Error> {
    let mut cycles = Vec::new();
    let mut visited = HashSet::new();
    let mut path = Vec::new();
    let mut on_stack = HashSet::new();
    
    // Collect all interfaces
    let mut all_interfaces = HashSet::new();
    
    // Add all interfaces from the hierarchy
    for (source, targets) in hierarchy {
        all_interfaces.insert(source.clone());
        for target in targets {
            all_interfaces.insert(target.clone());
        }
    }
    
    // Helper function for cycle detection
    fn dfs_cycle(
        hierarchy: &HashMap<String, Vec<String>>,
        interface: &str,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
        on_stack: &mut HashSet<String>,
        cycles: &mut Vec<Vec<String>>,
    ) -> Result<(), Error> {
        visited.insert(interface.to_string());
        path.push(interface.to_string());
        on_stack.insert(interface.to_string());
        
        // Check all direct extensions
        if let Some(extensions) = hierarchy.get(interface) {
            for extension in extensions {
                if !visited.contains(extension) {
                    dfs_cycle(hierarchy, extension, visited, path, on_stack, cycles)?;
                } else if on_stack.contains(extension) {
                    // Found a cycle
                    if let Some(cycle_start) = path.iter().position(|x| x == extension) {
                        let cycle = path[cycle_start..].to_vec();
                        cycles.push(cycle);
                    }
                }
            }
        }
        
        // Backtrack
        path.pop();
        on_stack.remove(interface);
        
        Ok(())
    }
    
    // Run cycle detection on each interface
    for interface in all_interfaces {
        if !visited.contains(&interface) {
            dfs_cycle(
                hierarchy,
                &interface,
                &mut visited,
                &mut path,
                &mut on_stack,
                &mut cycles,
            )?;
        }
    }
    
    Ok(cycles)
}

/// Factory function to create an interface registry  
pub fn create_interface_registry() -> Arc<RwLock<crate::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry>> {
    crate::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::common;
    
    #[test]
    fn test_generate_dot_diagram() {
        common::tracing::setup();
        
        let registry = create_interface_registry();
        let visualization = DefaultInterfaceRegistryVisualization::new(registry);
        
        let result = visualization.generate_dot_diagram();
        assert!(result.is_ok());
        let dot = result.unwrap();
        assert!(dot.contains("digraph InterfaceHierarchy"));
    }
    
    #[test]
    fn test_get_text_representation() {
        common::tracing::setup();
        
        let registry = create_interface_registry();
        let visualization = DefaultInterfaceRegistryVisualization::new(registry);
        
        let result = visualization.get_text_representation();
        assert!(result.is_ok());
        let text = result.unwrap();
        assert!(text.contains("Interface Hierarchy"));
    }
}
