//! # Interface Registry Extension Visualization Implementation
//!
//! This module provides an implementation of the `InterfaceRegistryExtensionWithVisualization`
//! trait for the `ThreadSafeInterfaceExtensionRegistry` struct. This enables proper
//! visualization capabilities for interface extension relationships in a thread-safe manner.
//!
//! The implementation addresses several borrowing issues by properly handling read/write
//! locks and ensuring thread safety throughout the codebase.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use tracing::{debug, info, instrument, trace, warn};

use crate::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry;
use crate::core::interface_registry_visualization::{InterfaceRegistryExtensionWithVisualization, VisualizationOptions};
use crate::error::Error;

/// Extension methods for visualization capabilities of `ThreadSafeInterfaceExtensionRegistry`
impl ThreadSafeInterfaceExtensionRegistry {
    #[instrument(level = "debug")]
    fn register_extension(&self, source: &str, target: &str) -> Result<(), Error> {
        // Delegate to the instance method
        self.register_extension(source, target)
    }
    
    #[instrument(level = "debug")]
    pub fn get_extension_hierarchy(&self) -> Result<HashMap<String, Vec<String>>, Error> {
        debug!("Getting complete extension hierarchy");
        
        let direct_extensions = self.direct_extensions().read().map_err(|e| {
            Error::Compilation(format!("Failed to acquire read lock on extension registry: {}", e))
        })?;
        
        // Create a deep copy to avoid holding the lock longer than necessary
        let mut hierarchy = HashMap::new();
        
        for (key, value) in &*direct_extensions {
            let extensions: Vec<String> = value.iter().cloned().collect();
            hierarchy.insert(key.clone(), extensions);
        }
        
        Ok(hierarchy)
    }
    
    #[instrument(level = "debug")]
    pub fn get_all_interfaces(&self) -> Result<HashSet<String>, Error> {
        debug!("Getting all interfaces in the registry");
        
        let interfaces = self.interfaces().read().map_err(|e| {
            Error::Compilation(format!("Failed to acquire read lock on extension registry: {}", e))
        })?;
        
        Ok(interfaces.clone())
    }
    
    #[instrument(level = "debug")]
    pub fn get_direct_extensions(&self, interface: &str) -> Result<Option<Vec<String>>, Error> {
        debug!("Getting direct extensions of interface: {}", interface);
        
        let direct_extensions = self.direct_extensions().read().map_err(|e| {
            Error::Compilation(format!("Failed to acquire read lock on extension registry: {}", e))
        })?;
        
        if let Some(extensions) = direct_extensions.get(interface) {
            // Convert HashSet to Vec for the return type
            let extensions_vec: Vec<String> = extensions.iter().cloned().collect();
            Ok(Some(extensions_vec))
        } else {
            Ok(None)
        }
    }
    
    #[instrument(level = "debug")]
    pub fn get_direct_implementors(&self, interface: &str) -> Result<Option<Vec<String>>, Error> {
        debug!("Getting direct implementors of interface: {}", interface);
        
        let direct_implementers = self.direct_implementers().read().map_err(|e| {
            Error::Compilation(format!("Failed to acquire read lock on extension registry: {}", e))
        })?;
        
        if let Some(implementors) = direct_implementers.get(interface) {
            // Convert HashSet to Vec for the return type
            let implementors_vec: Vec<String> = implementors.iter().cloned().collect();
            Ok(Some(implementors_vec))
        } else {
            Ok(None)
        }
    }
    
    #[instrument(level = "debug")]
    fn extends(&self, source: &str, target: &str) -> Result<bool, Error> {
        debug!("Checking if {} extends {}", source, target);
        
        // If source and target are the same, return true immediately
        if source == target {
            return Ok(true);
        }
        
        // Use the extends method from the trait implementation
        self.extends(source, target)
    }
    
    #[instrument(level = "debug")]
    fn get_all_extensions(&self, interface: &str) -> Result<HashSet<String>, Error> {
        debug!("Getting all extensions of interface: {}", interface);
        
        let mut result = HashSet::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        
        // Start with the interface itself
        queue.push_back(interface.to_string());
        visited.insert(interface.to_string());
        
        // Perform BFS to find all extensions
        while let Some(current) = queue.pop_front() {
            if let Some(extensions) = self.get_direct_extensions(&current)? {
                for extension in extensions {
                    if !visited.contains(&extension) {
                        visited.insert(extension.clone());
                        queue.push_back(extension.clone());
                        result.insert(extension);
                    }
                }
            }
        }
        
        Ok(result)
    }
    
    #[instrument(level = "debug")]
    fn get_all_implementors(&self, interface: &str) -> Result<HashSet<String>, Error> {
        debug!("Getting all implementors of interface: {}", interface);
        
        let mut result = HashSet::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        
        // Start with the interface itself
        queue.push_back(interface.to_string());
        visited.insert(interface.to_string());
        
        // Perform BFS to find all implementors
        while let Some(current) = queue.pop_front() {
            if let Some(implementors) = self.get_direct_implementors(&current)? {
                for implementor in implementors {
                    if !visited.contains(&implementor) {
                        visited.insert(implementor.clone());
                        queue.push_back(implementor.clone());
                        result.insert(implementor);
                    }
                }
            }
        }
        
        Ok(result)
    }
    
    #[instrument(level = "debug")]
    fn find_inheritance_path(&self, source: &str, target: &str) -> Result<Vec<String>, Error> {
        debug!("Finding inheritance path from {} to {}", source, target);
        
        // If source and target are the same, return a path with just the interface
        if source == target {
            return Ok(vec![source.to_string()]);
        }
        
        // Use BFS to find the shortest path
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut parent_map: HashMap<String, String> = HashMap::new();
        
        // Start BFS from source
        queue.push_back(source.to_string());
        visited.insert(source.to_string());
        
        // Perform BFS
        while let Some(current) = queue.pop_front() {
            // Check if we've reached the target
            if current == target {
                // Reconstruct the path
                let mut path = Vec::new();
                let mut curr = current.clone();
                
                path.push(curr.clone());
                
                while let Some(parent) = parent_map.get(&curr) {
                    path.push(parent.clone());
                    curr = parent.clone();
                }
                
                // Reverse the path to get it in the correct order
                path.reverse();
                
                return Ok(path);
            }
            
            // Get direct extensions of current interface
            if let Some(extensions) = self.get_direct_extensions(&current)? {
                for extension in extensions {
                    if !visited.contains(&extension) {
                        visited.insert(extension.clone());
                        queue.push_back(extension.clone());
                        parent_map.insert(extension.clone(), current.clone());
                    }
                }
            }
        }
        
        // No path found
        Err(Error::Compilation(format!(
            "No inheritance path found from interface '{}' to interface '{}'",
            source, target
        )))
    }
    
    #[instrument(level = "debug")]
    fn find_all_inheritance_paths(&self, source: &str, target: &str) -> Result<Vec<Vec<String>>, Error> {
        debug!("Finding all inheritance paths from {} to {}", source, target);
        
        // If source and target are the same, return a path with just the interface
        if source == target {
            return Ok(vec![vec![source.to_string()]]);
        }
        
        // Use DFS to find all paths
        let mut paths = Vec::new();
        let mut visited = HashSet::new();
        let mut current_path = Vec::new();
        
        // Helper function for DFS
        fn dfs(
            registry: &ThreadSafeInterfaceExtensionRegistry,
            source: &str,
            target: &str,
            current_path: &mut Vec<String>,
            visited: &mut HashSet<String>,
            paths: &mut Vec<Vec<String>>,
        ) -> Result<(), Error> {
            // Mark current interface as visited
            visited.insert(source.to_string());
            current_path.push(source.to_string());
            
            // Check if we've reached the target
            if source == target {
                paths.push(current_path.clone());
            } else {
                // Get direct extensions of current interface
                if let Some(extensions) = registry.get_direct_extensions(source)? {
                    for extension in extensions {
                        if !visited.contains(&extension) {
                            dfs(registry, &extension, target, current_path, visited, paths)?;
                        }
                    }
                }
            }
            
            // Backtrack
            visited.remove(source);
            current_path.pop();
            
            Ok(())
        }
        
        // Start DFS
        dfs(self, source, target, &mut current_path, &mut visited, &mut paths)?;
        
        // Check if we found any paths
        if paths.is_empty() {
            return Err(Error::Compilation(format!(
                "No inheritance paths found from interface '{}' to interface '{}'",
                source, target
            )));
        }
        
        Ok(paths)
    }
    
    #[instrument(level = "debug")]
    fn detect_cycles(&self) -> Result<Vec<Vec<String>>, Error> {
        debug!("Detecting cycles in interface inheritance hierarchy");
        
        // Get the hierarchy first
        let hierarchy = self.get_extension_hierarchy()?;
        
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
    
    #[instrument(level = "debug")]
    fn visualize_hierarchy_ascii(&self) -> Result<String, Error> {
        debug!("Visualizing interface hierarchy as ASCII art");
        
        // Get the extension hierarchy
        let hierarchy = self.get_extension_hierarchy()?;
        
        // Create default visualization options
        let options = VisualizationOptions::default();
        
        // Generate ASCII tree
        self.generate_ascii_tree(&hierarchy, &options)
    }
    
    #[instrument(level = "debug")]
    fn visualize_hierarchy_dot(&self) -> Result<String, Error> {
        debug!("Visualizing interface hierarchy as DOT graph");
        
        // Get the extension hierarchy
        let hierarchy = self.get_extension_hierarchy()?;
        
        // Create default visualization options
        let options = VisualizationOptions::default();
        
        // Generate DOT graph
        self.generate_dot_graph(&hierarchy, &options)
    }
    
    #[instrument(level = "debug")]
    fn visualize_path_ascii(&self, path: &[String]) -> Result<String, Error> {
        debug!("Visualizing inheritance path as ASCII art");
        
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
    
    #[instrument(level = "debug")]
    fn visualize_path_dot(&self, path: &[String]) -> Result<String, Error> {
        debug!("Visualizing inheritance path as DOT graph");
        
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
    
    #[instrument(level = "debug")]
    fn interface_exists(&self, interface: &str) -> Result<bool, Error> {
        debug!("Checking if interface exists: {}", interface);
        
        // Get all interfaces
        let all_interfaces = self.get_all_interfaces()?;
        
        // Check if the interface exists
        Ok(all_interfaces.contains(interface))
    }
    
    #[instrument(level = "debug", skip(self, hierarchy, options))]
    fn generate_ascii_tree(&self, hierarchy: &HashMap<String, Vec<String>>, options: &VisualizationOptions) -> Result<String, Error> {
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
    
    #[instrument(level = "debug", skip(self, hierarchy, options))]
    fn generate_dot_graph(&self, hierarchy: &HashMap<String, Vec<String>>, options: &VisualizationOptions) -> Result<String, Error> {
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
        
        // If include_cycles is enabled, detect and highlight cycles
        if options.include_cycles {
            match self.detect_cycles(hierarchy) {
                Ok(cycles) if !cycles.is_empty() => {
                    dot.push_str("\n  // Cycles\n");
                    
                    for (i, cycle) in cycles.iter().enumerate() {
                        dot.push_str(&format!("  subgraph cluster_cycle_{} {{\n", i));
                        dot.push_str("    style=filled;\n");
                        dot.push_str("    color=lightpink;\n");
                        dot.push_str(&format!("    label=\"Cycle {}\";\n", i + 1));
                        
                        for interface in cycle {
                            dot.push_str(&format!("    \"{}\";\n", interface));
                        }
                        
                        dot.push_str("  }\n");
                    }
                },
                _ => {}
            }
        }
        
        dot.push_str("}\n");
        
        Ok(dot)
    }
    
    #[instrument(level = "debug", skip(self, hierarchy, options))]
    fn generate_json_representation(&self, hierarchy: &HashMap<String, Vec<String>>, options: &VisualizationOptions) -> Result<String, Error> {
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
        
        result.push_str("  },\n");
        
        // Add cycles if enabled
        if options.include_cycles {
            result.push_str("  \"cycles\": [");
            
            match self.detect_cycles(hierarchy) {
                Ok(cycles) => {
                    for (i, cycle) in cycles.iter().enumerate() {
                        if i > 0 {
                            result.push_str(", ");
                        }
                        result.push_str("[\n");
                        
                        for (j, interface) in cycle.iter().enumerate() {
                            if j > 0 {
                                result.push_str(", ");
                            }
                            result.push_str(&format!("        \"{}\"", interface));
                        }
                        
                        result.push_str("\n      ]");
                    }
                },
                _ => {}
            }
            
            result.push_str("]\n");
        }
        
        result.push_str("}\n");
        
        Ok(result)
    }
    
    #[instrument(level = "debug")]
    fn is_visualization_initialized(&self) -> Result<bool, Error> {
        debug!("Checking if visualization system is initialized");
        
        // For the ThreadSafeInterfaceExtensionRegistry, we don't have a separate 
        // initialized flag, so we'll consider it initialized if it has any entries
        let direct_extensions = self.direct_extensions().read().map_err(|e| {
            Error::Compilation(format!("Failed to acquire read lock on extension registry: {}", e))
        })?;
        let direct_implementers = self.direct_implementers().read().map_err(|e| {
            Error::Compilation(format!("Failed to acquire read lock on extension registry: {}", e))
        })?;
        
        // Check if there are any entries in the registry
        Ok(!direct_extensions.is_empty() || !direct_implementers.is_empty())
    }
    
    #[instrument(level = "debug")]
    fn set_visualization_initialized(&self, _initialized: bool) -> Result<(), Error> {
        // For ThreadSafeInterfaceExtensionRegistry, this is a no-op since we don't have
        // a separate initialization flag
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_visualization_interface_implementation() {
        // Create a new ThreadSafeInterfaceExtensionRegistry
        let registry = ThreadSafeInterfaceExtensionRegistry::new();
        
        // Register some extensions
        registry.register_extension("A", "B").unwrap();
        registry.register_extension("B", "C").unwrap();
        registry.register_extension("A", "D").unwrap();
        
        // Test visualization methods
        
        // Check if visualization is initialized
        assert!(registry.is_visualization_initialized().unwrap());
        
        // Get all interfaces
        let interfaces = registry.get_all_interfaces().unwrap();
        assert!(interfaces.contains("A"));
        assert!(interfaces.contains("B"));
        assert!(interfaces.contains("C"));
        assert!(interfaces.contains("D"));
        
        // Check extension relationship
        assert!(registry.extends("A", "B").unwrap());
        assert!(registry.extends("A", "C").unwrap());
        assert!(registry.extends("B", "C").unwrap());
        assert!(!registry.extends("C", "A").unwrap());
        
        // Get inheritance path
        let path = registry.find_inheritance_path("A", "C").unwrap();
        assert_eq!(path, vec!["A", "B", "C"]);
        
        // Generate ASCII tree
        let hierarchy = registry.get_extension_hierarchy().unwrap();
        let options = VisualizationOptions::default();
        let ascii = registry.generate_ascii_tree(&hierarchy, &options).unwrap();
        
        // Check that the ASCII tree contains all interfaces
        assert!(ascii.contains("A"));
        assert!(ascii.contains("B"));
        assert!(ascii.contains("C"));
        assert!(ascii.contains("D"));
        
        // Generate DOT graph
        let dot = registry.generate_dot_graph(&hierarchy, &options).unwrap();
        
        // Check that the DOT graph contains all interfaces
        assert!(dot.contains("A"));
        assert!(dot.contains("B"));
        assert!(dot.contains("C"));
        assert!(dot.contains("D"));
        
        // Test cyclic extensions
        registry.register_extension("C", "A").unwrap();
        
        // Detect cycles
        let cycles = registry.detect_cycles(&registry.get_extension_hierarchy().unwrap()).unwrap();
        assert!(!cycles.is_empty());
    }
}