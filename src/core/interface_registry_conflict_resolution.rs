//! # Interface Registry Conflict Resolution
//!
//! This module provides a resolution for the conflicting implementations of
//! `InterfaceRegistryExtensionWithVisualization` trait. It uses a wrapping adapter
//! to ensure that the correct implementation is used without causing conflicts.
//!
//! The resolution strategy is to:
//! 1. Create an adapter/wrapper that delegates to the appropriate implementation
//! 2. Update the relevant trait bounds to use this adapter
//! 3. Ensure proper method delegation between implementations

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use tracing::{debug, info, instrument, trace, warn};

use crate::core::interface_registry_extensions::{InterfaceRegistryExtension, ThreadSafeInterfaceExtensionRegistry};
use crate::core::interface_registry_visualization::{
    InterfaceRegistryVisualization, InterfaceRegistryExtensionWithVisualization, 
    ThreadSafeInterfaceRegistryVisualization, VisualizationOptions,
};
use crate::error::Error;

/// Resolver for the conflicting implementations of InterfaceRegistryExtensionWithVisualization
#[derive(Debug)]
pub struct InterfaceRegistryAdapter {
    /// The extension registry to delegate to
    extension_registry: ThreadSafeInterfaceExtensionRegistry,
    /// Optional visualization registry to delegate to when available
    visualization_registry: Option<Box<dyn InterfaceRegistryVisualization + Send + Sync>>,
}

impl InterfaceRegistryAdapter {
    /// Create a new adapter with the extension registry
    pub fn new(extension_registry: ThreadSafeInterfaceExtensionRegistry) -> Self {
        // Create a new adapter
        Self {
            extension_registry,
            visualization_registry: None,
        }
    }

    /// Add visualization registry to the adapter
    pub fn with_visualization(mut self, visualization: Box<dyn InterfaceRegistryVisualization + Send + Sync>) -> Self {
        self.visualization_registry = Some(visualization);
        self
    }
}

/// Implementation of InterfaceRegistryExtension for the adapter
impl InterfaceRegistryExtension for InterfaceRegistryAdapter {
    /// Register a new interface
    fn register_interface(&mut self, name: &str) {
        self.extension_registry.register_interface(name);
    }
    
    /// Register an extension relationship between interfaces
    fn register_extension(&mut self, source: &str, target: &str) -> Result<(), Error> {
        self.extension_registry.register_extension(source, target)
    }
    
    /// Check if an extension relationship exists between interfaces
    fn has_extension(&self, source: &str, target: &str) -> Result<bool, Error> {
        self.extension_registry.has_extension(source, target)
    }
    
    /// Get all registered interfaces
    fn get_all_interfaces(&self) -> Option<HashSet<String>> {
        self.extension_registry.get_all_interfaces()
    }
    
    /// Get all direct extensions of an interface
    fn get_direct_extensions(&self, interface: &str) -> Result<Option<HashSet<String>>, Error> {
        self.extension_registry.get_direct_extensions(interface)
    }
    
    /// Get all direct implementers of an interface
    fn get_direct_implementers(&self, interface: &str) -> Result<Option<HashSet<String>>, Error> {
        self.extension_registry.get_direct_implementers(interface)
    }
    
    /// Check if an interface extends another interface (direct or indirect)
    fn extends(&self, source: &str, target: &str) -> Result<bool, Error> {
        self.extension_registry.extends(source, target)
    }
    
    /// Find a common ancestor between two interfaces
    fn find_common_ancestor(&self, a: &str, b: &str) -> Result<Option<String>, Error> {
        self.extension_registry.find_common_ancestor(a, b)
    }
    
    /// Find the longest path between two interfaces
    fn find_longest_path(&self, source: &str, target: &str) -> Result<Option<Vec<String>>, Error> {
        self.extension_registry.find_longest_path(source, target)
    }
}

    /// Create a new adapter with both registries
    pub fn new_with_visualization(
        extension_registry: ThreadSafeInterfaceExtensionRegistry,
        visualization_registry: Box<dyn InterfaceRegistryVisualization + Send + Sync>,
    ) -> InterfaceRegistryAdapter {
        InterfaceRegistryAdapter {
            extension_registry,
            visualization_registry: Some(visualization_registry),
        }
    }

/// Custom implementation for the adapter - doesn't actually implement the trait directly
impl InterfaceRegistryAdapter {
    fn get_inheritance_distance(&self, source: &str, target: &str) -> Result<Option<usize>, Error> {
        // Use the internal extension registry implementation
        match InterfaceRegistryExtensionWithVisualization::get_inheritance_distance(&self.extension_registry, source, target) {
            Ok(distance) => Ok(distance),
            Err(e) => Err(e),
        }
    }
    
    fn find_all_paths(&self, source: &str, target: &str) -> Result<Vec<Vec<String>>, Error> {
        // Use the internal extension registry implementation
        match InterfaceRegistryExtensionWithVisualization::find_all_paths(&self.extension_registry, source, target) {
            Ok(paths) => Ok(paths),
            Err(e) => Err(e),
        }
    }
    
    fn find_diamond_inheritance_patterns(&self) -> Result<Vec<(String, String, String, String)>, Error> {
        // Use the internal extension registry implementation
        match InterfaceRegistryExtensionWithVisualization::find_diamond_inheritance_patterns(&self.extension_registry) {
            Ok(patterns) => Ok(patterns),
            Err(e) => Err(e),
        }
    }
    #[instrument(level = "debug")]
    fn register_extension(&self, source: &str, target: &str) -> Result<(), Error> {
        // Always delegate to the extension registry
        self.extension_registry.register_extension(source, target)
    }

    #[instrument(level = "debug")]
    fn get_extension_hierarchy(&self) -> Result<HashMap<String, Vec<String>>, Error> {
        // Prefer visualization registry if available
        if let Some(vis_registry) = &self.visualization_registry {
            vis_registry.get_extension_hierarchy()
        } else {
            self.extension_registry.get_extension_hierarchy()
        }
    }

    #[instrument(level = "debug")]
    fn get_all_interfaces(&self) -> Result<HashSet<String>, Error> {
        // Prefer visualization registry if available
        if let Some(vis_registry) = &self.visualization_registry {
            vis_registry.get_all_interfaces()
        } else {
            self.extension_registry.get_all_interfaces()
        }
    }

    #[instrument(level = "debug")]
    fn get_direct_extensions(&self, interface: &str) -> Result<Option<Vec<String>>, Error> {
        // Prefer visualization registry if available
        if let Some(vis_registry) = &self.visualization_registry {
            vis_registry.get_direct_extensions(interface)
        } else {
            self.extension_registry.get_direct_extensions(interface)
        }
    }

    #[instrument(level = "debug")]
    fn get_direct_implementors(&self, interface: &str) -> Result<Option<Vec<String>>, Error> {
        // Prefer visualization registry if available
        if let Some(vis_registry) = &self.visualization_registry {
            vis_registry.get_direct_implementors(interface)
        } else {
            self.extension_registry.get_direct_implementors(interface)
        }
    }

    #[instrument(level = "debug")]
    fn extends(&self, source: &str, target: &str) -> Result<bool, Error> {
        // Prefer visualization registry if available
        if let Some(vis_registry) = &self.visualization_registry {
            vis_registry.extends(source, target)
        } else {
            self.extension_registry.extends(source, target)
        }
    }

    #[instrument(level = "debug")]
    fn get_all_extensions(&self, interface: &str) -> Result<HashSet<String>, Error> {
        // Prefer visualization registry if available
        if let Some(vis_registry) = &self.visualization_registry {
            vis_registry.get_all_extensions(interface)
        } else {
            self.extension_registry.get_all_extensions(interface)
        }
    }

    #[instrument(level = "debug")]
    fn get_all_implementors(&self, interface: &str) -> Result<HashSet<String>, Error> {
        // Prefer visualization registry if available
        if let Some(vis_registry) = &self.visualization_registry {
            vis_registry.get_all_implementors(interface)
        } else {
            self.extension_registry.get_all_implementors(interface)
        }
    }

    #[instrument(level = "debug")]
    fn find_inheritance_path(&self, source: &str, target: &str) -> Result<Vec<String>, Error> {
        // Prefer visualization registry if available
        if let Some(vis_registry) = &self.visualization_registry {
            vis_registry.find_inheritance_path(source, target)
        } else {
            self.extension_registry.find_inheritance_path(source, target)
        }
    }

    #[instrument(level = "debug")]
    fn find_all_inheritance_paths(
        &self,
        source: &str,
        target: &str,
    ) -> Result<Vec<Vec<String>>, Error> {
        // Prefer visualization registry if available
        if let Some(vis_registry) = &self.visualization_registry {
            vis_registry.find_all_inheritance_paths(source, target)
        } else {
            self.extension_registry.find_all_inheritance_paths(source, target)
        }
    }

    #[instrument(level = "debug")]
    fn detect_cycles(&self) -> Result<Vec<Vec<String>>, Error> {
        // Prefer visualization registry if available
        if let Some(vis_registry) = &self.visualization_registry {
            vis_registry.detect_cycles()
        } else {
            // Get the hierarchy from extension registry
            let hierarchy = self.get_extension_hierarchy()?;
            
            // Run cycle detection on the hierarchy
            let mut cycles = Vec::new();
            let mut visited = HashSet::new();
            let mut path = Vec::new();
            let mut on_stack = HashSet::new();
            
            // Get all interfaces
            let mut all_interfaces = HashSet::new();
            
            // Collect all interfaces from the hierarchy
            for (source, targets) in &hierarchy {
                all_interfaces.insert(source.clone());
                for target in targets {
                    all_interfaces.insert(target.clone());
                }
            }
            
            // Helper function for cycle detection using DFS
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
                
                // Get direct extensions
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
            
            // Check each interface
            for interface in all_interfaces {
                if !visited.contains(&interface) {
                    dfs_cycle(
                        &hierarchy,
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
    }

    #[instrument(level = "debug")]
    fn visualize_hierarchy_ascii(&self) -> Result<String, Error> {
        // Prefer visualization registry if available
        if let Some(vis_registry) = &self.visualization_registry {
            vis_registry.visualize_hierarchy_ascii()
        } else {
            // Get the extension hierarchy
            let hierarchy = self.get_extension_hierarchy()?;
            
            // Create default visualization options
            let options = VisualizationOptions::default();
            
            // Generate ASCII tree
            self.generate_ascii_tree(&hierarchy, &options)
        }
    }

    #[instrument(level = "debug")]
    fn visualize_hierarchy_dot(&self) -> Result<String, Error> {
        // Prefer visualization registry if available
        if let Some(vis_registry) = &self.visualization_registry {
            vis_registry.visualize_hierarchy_dot()
        } else {
            // Get the extension hierarchy
            let hierarchy = self.get_extension_hierarchy()?;
            
            // Create default visualization options
            let options = VisualizationOptions::default();
            
            // Generate DOT graph
            self.generate_dot_graph(&hierarchy, &options)
        }
    }

    #[instrument(level = "debug")]
    fn visualize_path_ascii(&self, path: &[String]) -> Result<String, Error> {
        // Prefer visualization registry if available
        if let Some(vis_registry) = &self.visualization_registry {
            vis_registry.visualize_path_ascii(path)
        } else {
            if path.is_empty() {
                return Ok(String::from("Empty path\n"));
            }
            
            let mut result = String::from("Interface Inheritance Path:\n");
            
            for (i, interface) in path.iter().enumerate() {
                if i > 0 {
                    result.push_str("  ↓ extends\n");
                }
                result.push_str(&format!("  [{}]\n", interface));
            }
            
            Ok(result)
        }
    }

    #[instrument(level = "debug")]
    fn visualize_path_dot(&self, path: &[String]) -> Result<String, Error> {
        // Prefer visualization registry if available
        if let Some(vis_registry) = &self.visualization_registry {
            vis_registry.visualize_path_dot(path)
        } else {
            if path.is_empty() {
                return Ok(String::from("digraph empty_path {}\n"));
            }
            
            let mut dot = String::from("digraph path {\n");
            dot.push_str("  node [shape=box, style=filled, fillcolor=lightblue];\n");
            
            // Add nodes
            for interface in path {
                dot.push_str(&format!("  \"{}\" [label=\"{}\"];\n", interface, interface));
            }
            
            // Add edges
            for i in 0..path.len() - 1 {
                dot.push_str(&format!("  \"{}\" -> \"{}\";\n", path[i], path[i + 1]));
            }
            
            dot.push_str("}\n");
            
            Ok(dot)
        }
    }

    #[instrument(level = "debug")]
    fn interface_exists(&self, interface: &str) -> Result<bool, Error> {
        // Prefer visualization registry if available
        if let Some(vis_registry) = &self.visualization_registry {
            vis_registry.interface_exists(interface)
        } else {
            // Get all interfaces
            let all_interfaces = self.get_all_interfaces()?;
            
            // Check if the interface exists
            Ok(all_interfaces.contains(interface))
        }
    }

    #[instrument(level = "debug", skip(self, hierarchy, options))]
    fn generate_ascii_tree(
        &self,
        hierarchy: &HashMap<String, Vec<String>>,
        options: &VisualizationOptions,
    ) -> Result<String, Error> {
        // Prefer visualization registry if available
        if let Some(vis_registry) = &self.visualization_registry {
            vis_registry.generate_ascii_tree(hierarchy, options)
        } else {
            // Implement ASCII tree visualization
            debug!("Generating ASCII tree visualization");
            
            let mut result = String::new();
            result.push_str("Interface Hierarchy:\n");
            
            // Get all interfaces
            let mut all_interfaces = HashSet::new();
            
            // Collect all interfaces from the hierarchy
            for (source, targets) in hierarchy {
                all_interfaces.insert(source.clone());
                for target in targets {
                    all_interfaces.insert(target.clone());
                }
            }
            
            // Sort interfaces for consistent output
            let mut sorted_interfaces: Vec<_> = all_interfaces.into_iter().collect();
            sorted_interfaces.sort();
            
            // Find root interfaces (not extended by any other interface)
            let mut roots = HashSet::new();
            let mut has_parent = HashSet::new();
            
            for (_, targets) in hierarchy {
                for target in targets {
                    has_parent.insert(target.clone());
                }
            }
            
            for interface in &sorted_interfaces {
                if !has_parent.contains(interface) {
                    roots.insert(interface.clone());
                }
            }
            
            // If no roots found, use all interfaces as roots
            if roots.is_empty() {
                roots = sorted_interfaces.into_iter().collect();
            }
            
            let mut sorted_roots: Vec<_> = roots.into_iter().collect();
            sorted_roots.sort();
            
            // Helper function to print the tree recursively
            fn print_tree(
                hierarchy: &HashMap<String, Vec<String>>,
                interface: &str,
                prefix: &str,
                is_last: bool,
                result: &mut String,
                visited: &mut HashSet<String>,
                depth: usize,
                max_depth: Option<usize>,
                include_cycles: bool,
            ) -> Result<(), Error> {
                // Check depth limit
                if let Some(limit) = max_depth {
                    if depth > limit {
                        return Ok(());
                    }
                }
                
                // Check for cycles
                if visited.contains(interface) {
                    if include_cycles {
                        result.push_str(&format!("{}{} {} (cycle)\n", 
                                                prefix, 
                                                if is_last { "└── " } else { "├── " },
                                                interface));
                    }
                    return Ok(());
                }
                
                // Print current interface
                result.push_str(&format!("{}{} {}\n", 
                                        prefix, 
                                        if is_last { "└── " } else { "├── " },
                                        interface));
                
                // Mark as visited to prevent cycles
                visited.insert(interface.to_string());
                
                // Get direct extensions
                if let Some(extensions) = hierarchy.get(interface) {
                    let mut sorted_extensions = extensions.clone();
                    sorted_extensions.sort();
                    
                    for (i, extension) in sorted_extensions.iter().enumerate() {
                        let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
                        let is_last_child = i == sorted_extensions.len() - 1;
                        
                        print_tree(
                            hierarchy,
                            extension,
                            &new_prefix,
                            is_last_child,
                            result,
                            visited,
                            depth + 1,
                            max_depth,
                            include_cycles,
                        )?;
                    }
                }
                
                // Remove from visited to allow the same interface to appear in different branches
                visited.remove(interface);
                
                Ok(())
            }
            
            // Print the tree for each root
            for (i, root) in sorted_roots.iter().enumerate() {
                let is_last = i == sorted_roots.len() - 1;
                let mut visited = HashSet::new();
                
                print_tree(
                    hierarchy,
                    root,
                    "",
                    is_last,
                    &mut result,
                    &mut visited,
                    0,
                    options.max_depth,
                    options.include_cycles,
                )?;
            }
            
            Ok(result)
        }
    }

    #[instrument(level = "debug", skip(self, hierarchy, options))]
    fn generate_dot_graph(
        &self,
        hierarchy: &HashMap<String, Vec<String>>,
        options: &VisualizationOptions,
    ) -> Result<String, Error> {
        // Prefer visualization registry if available
        if let Some(vis_registry) = &self.visualization_registry {
            vis_registry.generate_dot_graph(hierarchy, options)
        } else {
            // Implement DOT graph visualization
            debug!("Generating DOT graph visualization");
            
            let mut dot = String::new();
            dot.push_str("digraph interface_hierarchy {\n");
            dot.push_str("  node [shape=box, style=filled, fillcolor=lightblue];\n");
            
            // Add nodes for all interfaces
            let mut all_interfaces = HashSet::new();
            
            // Collect all interfaces from the hierarchy
            for (source, targets) in hierarchy {
                all_interfaces.insert(source.clone());
                for target in targets {
                    all_interfaces.insert(target.clone());
                }
            }
            
            // Add nodes
            for interface in &all_interfaces {
                dot.push_str(&format!("  \"{}\" [label=\"{}\"]; /* Interface node */\n", interface, interface));
            }
            
            // Add edges
            for (source, targets) in hierarchy {
                for target in targets {
                    dot.push_str(&format!("  \"{}\" -> \"{}\"; /* Extension relationship */\n", source, target));
                }
            }
            
            dot.push_str("}\n");
            
            Ok(dot)
        }
    }

    #[instrument(level = "debug", skip(self, hierarchy, options))]
    fn generate_json_representation(
        &self,
        hierarchy: &HashMap<String, Vec<String>>,
        options: &VisualizationOptions,
    ) -> Result<String, Error> {
        // Prefer visualization registry if available
        if let Some(vis_registry) = &self.visualization_registry {
            vis_registry.generate_json_representation(hierarchy, options)
        } else {
            // Implement JSON representation
            debug!("Generating JSON representation");
            
            let mut result = String::new();
            result.push_str("{\n");
            
            // Convert hierarchy to JSON
            result.push_str("  \"interfaces\": {\n");
            
            // Get all interfaces
            let mut all_interfaces = HashSet::new();
            
            // Collect all interfaces from the hierarchy
            for (source, targets) in hierarchy {
                all_interfaces.insert(source.clone());
                for target in targets {
                    all_interfaces.insert(target.clone());
                }
            }
            
            // Sort interfaces for consistent output
            let mut sorted_interfaces: Vec<_> = all_interfaces.into_iter().collect();
            sorted_interfaces.sort();
            
            for (i, interface) in sorted_interfaces.iter().enumerate() {
                result.push_str(&format!("    \"{}\": {{\n", interface));
                
                // Get extensions
                result.push_str("      \"extends\": [");
                
                if let Some(extensions) = hierarchy.get(interface) {
                    let mut sorted_extensions = extensions.clone();
                    sorted_extensions.sort();
                    
                    for (j, extension) in sorted_extensions.iter().enumerate() {
                        if j > 0 {
                            result.push_str(", ");
                        }
                        result.push_str(&format!("\"{}\"", extension));
                    }
                }
                
                result.push_str("]\n");
                
                result.push_str("    }");
                if i < sorted_interfaces.len() - 1 {
                    result.push_str(",\n");
                } else {
                    result.push_str("\n");
                }
            }
            
            result.push_str("  }\n");
            result.push_str("}\n");
            
            Ok(result)
        }
    }

    #[instrument(level = "debug")]
    fn is_visualization_initialized(&self) -> Result<bool, Error> {
        // Prefer visualization registry if available
        if let Some(vis_registry) = &self.visualization_registry {
            vis_registry.is_visualization_initialized()
        } else {
            // For the extension registry, we consider it initialized if it has any entries
            let registry = self.extension_registry.registry.read().map_err(|e| {
                Error::Compilation(format!("Failed to acquire read lock on extension registry: {}", e))
            })?;
            
            // Check if there are any entries in the registry
            Ok(!registry.direct_extensions.is_empty() || !registry.reverse_extensions.is_empty())
        }
    }

    #[instrument(level = "debug")]
    fn set_visualization_initialized(&self, initialized: bool) -> Result<(), Error> {
        // Prefer visualization registry if available
        if let Some(vis_registry) = &self.visualization_registry {
            vis_registry.set_visualization_initialized(initialized)
        } else {
            // For the extension registry, this is a no-op
            Ok(())
        }
    }
}

/// Helper function to create a detector for cycles in the interface hierarchy
fn detect_cycles(
    hierarchy: &HashMap<String, Vec<String>>,
) -> Result<Vec<Vec<String>>, Error> {
    let mut cycles = Vec::new();
    let mut visited = HashSet::new();
    let mut path = Vec::new();
    let mut on_stack = HashSet::new();
    
    // Get all interfaces
    let mut all_interfaces = HashSet::new();
    
    // Collect all interfaces from the hierarchy
    for (source, targets) in hierarchy {
        all_interfaces.insert(source.clone());
        for target in targets {
            all_interfaces.insert(target.clone());
        }
    }
    
    // Helper function for cycle detection using DFS
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
        
        // Get direct extensions
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
    
    // Check each interface
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_adapter_with_only_extension_registry() {
        // Create a new extension registry
        let registry = ThreadSafeInterfaceExtensionRegistry::new();
        
        // Register some extensions
        registry.register_extension("A", "B").unwrap();
        registry.register_extension("B", "C").unwrap();
        
        // Create adapter with only extension registry
        let adapter = InterfaceRegistryAdapter::new(registry);
        
        // Test basic functionality
        let all_interfaces = adapter.get_all_interfaces().unwrap();
        assert!(all_interfaces.contains("A"));
        assert!(all_interfaces.contains("B"));
        assert!(all_interfaces.contains("C"));
        
        // Test inheritance path
        let path = adapter.find_inheritance_path("A", "C").unwrap();
        assert_eq!(path, vec!["A", "B", "C"]);
    }
    
    #[test]
    fn test_adapter_with_both_registries() {
        // Create a new extension registry
        let ext_registry = ThreadSafeInterfaceExtensionRegistry::new();
        
        // Register some extensions
        ext_registry.register_extension("A", "B").unwrap();
        ext_registry.register_extension("B", "C").unwrap();
        
        // Create a new visualization registry
        let vis_registry = ThreadSafeInterfaceRegistryVisualization::new();
        
        // Register the same extensions
        vis_registry.register_extension("A", "B").unwrap();
        vis_registry.register_extension("B", "C").unwrap();
        
        // Create adapter with both registries
        let adapter = InterfaceRegistryAdapter::new_with_visualization(ext_registry, vis_registry);
        
        // Test basic functionality
        let all_interfaces = adapter.get_all_interfaces().unwrap();
        assert!(all_interfaces.contains("A"));
        assert!(all_interfaces.contains("B"));
        assert!(all_interfaces.contains("C"));
        
        // Test inheritance path
        let path = adapter.find_inheritance_path("A", "C").unwrap();
        assert_eq!(path, vec!["A", "B", "C"]);
    }
}