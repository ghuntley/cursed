//! # Interface Registry Extension Visualization Implementation
//!
//! This module provides an implementation of the `InterfaceRegistryExtensionWithVisualization`
//! trait for the `ThreadSafeInterfaceExtensionRegistry` struct. This enables proper
//! visualization capabilities for interface extension relationships in a thread-safe manner.
//!
//! The implementation addresses several borrowing issues by properly handling read/write
//! locks and ensuring thread safety throughout the codebase.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, RwLock};
use tracing::{debug, info, instrument, trace, warn};

use crate::core::interface_registry_extensions::{ThreadSafeInterfaceExtensionRegistry, InterfaceRegistryExtension};
use crate::core::interface_registry_visualization::{InterfaceRegistryExtensionWithVisualization, InterfaceRegistryVisualization, VisualizationOptions};
use crate::error::Error;

/// Extension methods for visualization capabilities of `ThreadSafeInterfaceExtensionRegistry`
impl ThreadSafeInterfaceExtensionRegistry {
    /// Helper method to find cycles using DFS
    fn find_cycles_dfs(
        &self,
        current: &str,
        hierarchy: &HashMap<String, Vec<String>>,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
        cycles: &mut Vec<Vec<String>>,
    ) {
        if path.contains(&current.to_string()) {
            // Found a cycle
            if let Some(start_idx) = path.iter().position(|x| x == current) {
                let cycle = path[start_idx..].to_vec();
                if !cycle.is_empty() && !cycles.contains(&cycle) {
                    cycles.push(cycle);
                }
            }
            return;
        }
        
        if visited.contains(current) {
            return;
        }
        
        visited.insert(current.to_string());
        path.push(current.to_string());
        
        if let Some(extends) = hierarchy.get(current) {
            for target in extends {
                self.find_cycles_dfs(target, hierarchy, visited, path, cycles);
            }
        }
        
        path.pop();
    }
    
    /// Find all paths between two interfaces
    fn find_all_paths_between(&self, source: &str, target: &str) -> Result<Vec<Vec<String>>, Error> {
        let mut all_paths = Vec::new();
        let hierarchy = self.get_extension_hierarchy()?;
        let mut visited = HashSet::new();
        let mut current_path = vec![source.to_string()];
        
        self.find_all_paths_dfs(source, target, &hierarchy, &mut visited, &mut current_path, &mut all_paths);
        
        Ok(all_paths)
    }
    
    /// DFS helper for finding all paths
    fn find_all_paths_dfs(
        &self,
        current: &str,
        target: &str,
        hierarchy: &HashMap<String, Vec<String>>,
        visited: &mut HashSet<String>,
        current_path: &mut Vec<String>,
        all_paths: &mut Vec<Vec<String>>,
    ) {
        if current == target {
            all_paths.push(current_path.clone());
            return;
        }
        
        if visited.contains(current) {
            return;
        }
        
        visited.insert(current.to_string());
        
        if let Some(extends) = hierarchy.get(current) {
            for next in extends {
                current_path.push(next.clone());
                self.find_all_paths_dfs(next, target, hierarchy, visited, current_path, all_paths);
                current_path.pop();
            }
        }
        
        visited.remove(current);
    }
    
    /// Find shortest path between two interfaces
    fn find_shortest_path(&self, source: &str, target: &str) -> Result<Option<Vec<String>>, Error> {
        if source == target {
            return Ok(Some(vec![source.to_string()]));
        }
        
        let hierarchy = self.get_extension_hierarchy()?;
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent_map: HashMap<String, String> = HashMap::new();
        
        queue.push_back(source.to_string());
        visited.insert(source.to_string());
        
        while let Some(current) = queue.pop_front() {
            if current == target {
                // Reconstruct path
                let mut path = Vec::new();
                let mut curr = target.to_string();
                path.push(curr.clone());
                
                while let Some(parent) = parent_map.get(&curr) {
                    path.push(parent.clone());
                    curr = parent.clone();
                }
                
                path.reverse();
                return Ok(Some(path));
            }
            
            if let Some(extends) = hierarchy.get(&current) {
                for next in extends {
                    if !visited.contains(next) {
                        visited.insert(next.clone());
                        parent_map.insert(next.clone(), current.clone());
                        queue.push_back(next.clone());
                    }
                }
            }
        }
        
        Ok(None)
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
            match self.detect_cycles() {
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
            
            match self.detect_cycles() {
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
        use crate::core::interface_registry_extensions::InterfaceRegistryExtension;
        
        // Create a new ThreadSafeInterfaceExtensionRegistry
        let arc_registry = ThreadSafeInterfaceExtensionRegistry::new();
        
        // Register interfaces first
        arc_registry.write().unwrap().register_interface("A");
        arc_registry.write().unwrap().register_interface("B");
        arc_registry.write().unwrap().register_interface("C");
        arc_registry.write().unwrap().register_interface("D");
        
        // Register some extensions
        arc_registry.write().unwrap().register_extension("A", "B").unwrap();
        arc_registry.write().unwrap().register_extension("B", "C").unwrap();
        arc_registry.write().unwrap().register_extension("A", "D").unwrap();
        
        // Test visualization methods
        
        // Check if visualization is initialized
        assert!(arc_registry.read().unwrap().is_visualization_initialized().unwrap());
        
        // Get all interfaces
        let interfaces = arc_registry.read().unwrap().get_all_interfaces().unwrap();
        assert!(interfaces.contains("A"));
        assert!(interfaces.contains("B"));
        assert!(interfaces.contains("C"));
        assert!(interfaces.contains("D"));
        
        // Check extension relationship (using the trait method via Arc)
        assert!(arc_registry.extends("A", "B").unwrap());
        assert!(arc_registry.extends("A", "C").unwrap());
        assert!(arc_registry.extends("B", "C").unwrap());
        assert!(!arc_registry.extends("C", "A").unwrap());
        
        // Get inheritance path
        let path = arc_registry.find_inheritance_path("A", "C").unwrap();
        assert_eq!(path, vec!["A", "B", "C"]);
        
        // Generate ASCII tree
        let registry_ref = arc_registry.read().unwrap();
        let hierarchy = registry_ref.get_extension_hierarchy().unwrap();
        let options = VisualizationOptions::default();
        let ascii = registry_ref.generate_ascii_tree(&hierarchy, &options).unwrap();
        
        // Check that the ASCII tree contains all interfaces
        assert!(ascii.contains("A"));
        assert!(ascii.contains("B"));
        assert!(ascii.contains("C"));
        assert!(ascii.contains("D"));
        
        // Generate DOT graph
        let dot = registry_ref.generate_dot_graph(&hierarchy, &options).unwrap();
        
        // Check that the DOT graph contains all interfaces
        assert!(dot.contains("A"));
        assert!(dot.contains("B"));
        assert!(dot.contains("C"));
        assert!(dot.contains("D"));
        
        drop(registry_ref); // Release the read lock
        
        // Test cyclic extensions
        arc_registry.write().unwrap().register_extension("C", "A").unwrap();
        
        // Detect cycles
        let registry_ref2 = arc_registry.read().unwrap();
        let cycles = registry_ref2.detect_cycles(&registry_ref2.get_extension_hierarchy().unwrap()).unwrap();
        assert!(!cycles.is_empty());
    }
}

/// Implementation of InterfaceRegistryVisualization trait for ThreadSafeInterfaceExtensionRegistry
impl InterfaceRegistryVisualization for ThreadSafeInterfaceExtensionRegistry {
    #[instrument(skip(self), level = "debug")]
    fn generate_dot_diagram(&self) -> Result<String, Error> {
        debug!("Generating DOT diagram for interface hierarchy");
        
        let interfaces = self.get_all_interfaces()?;
        let hierarchy = self.get_extension_hierarchy()?;
        
        let mut dot = String::from("digraph InterfaceHierarchy {\n");
        dot.push_str("  rankdir=BT;\n"); // Bottom to top
        dot.push_str("  node [shape=box, style=filled, fillcolor=lightblue];\n\n");
        
        // Add nodes
        for interface in &interfaces {
            dot.push_str(&format!("  \"{}\" [label=\"{}\"];\n", interface, interface));
        }
        
        dot.push_str("\n");
        
        // Add edges
        for (source, targets) in &hierarchy {
            for target in targets {
                dot.push_str(&format!("  \"{}\" -> \"{}\";\n", source, target));
            }
        }
        
        dot.push_str("}\n");
        
        Ok(dot)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn generate_inheritance_path_diagram(&self, source: &str, target: &str) -> Result<String, Error> {
        debug!("Generating inheritance path diagram from {} to {}", source, target);
        
        // Check if the source extends the target
        if !self.extends(source, target)? {
            return Err(Error::Validation(format!(
                "'{}' does not extend '{}'", source, target
            )));
        }
        
        // Find the longest path
        let path = match self.find_longest_path(source, target)? {
            Some(p) => p,
            None => return Err(Error::Internal("Failed to find path".to_string())),
        };
        
        let mut dot = String::from("digraph InheritancePath {\n");
        dot.push_str("  rankdir=BT;\n"); // Bottom to top
        dot.push_str("  node [shape=box, style=filled];\n\n");
        
        // Add nodes with colors
        for (i, interface) in path.iter().enumerate() {
            let color = if i == 0 {
                "lightgreen" // Source
            } else if i == path.len() - 1 {
                "lightpink"  // Target
            } else {
                "lightblue"  // Intermediate
            };
            dot.push_str(&format!("  \"{}\" [label=\"{}\", fillcolor={}];\n", 
                interface, interface, color));
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
        debug!("Generating interface diagram for: {}", interface);
        
        if !self.interface_exists(interface)? {
            return Err(Error::NotFound(format!("Interface '{}' not found", interface)));
        }
        
        let mut dot = String::from("digraph InterfaceRelationships {\n");
        dot.push_str("  rankdir=BT;\n"); // Bottom to top
        dot.push_str("  node [shape=box, style=filled];\n\n");
        
        // Add the target interface in a different color
        dot.push_str(&format!("  \"{}\" [label=\"{}\", fillcolor=yellow];\n", interface, interface));
        
        // Add direct extensions
        if let Ok(Some(extensions)) = self.get_direct_extensions(interface) {
            for extension in extensions {
                dot.push_str(&format!("  \"{}\" [label=\"{}\", fillcolor=lightgreen];\n", 
                    extension, extension));
                dot.push_str(&format!("  \"{}\" -> \"{}\";\n", interface, extension));
            }
        }
        
        // Add direct implementors
        if let Ok(Some(implementors)) = self.get_direct_implementors(interface) {
            for implementor in implementors {
                dot.push_str(&format!("  \"{}\" [label=\"{}\", fillcolor=lightpink];\n", 
                    implementor, implementor));
                dot.push_str(&format!("  \"{}\" -> \"{}\";\n", implementor, interface));
            }
        }
        
        dot.push_str("}\n");
        
        Ok(dot)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn get_text_representation(&self) -> Result<String, Error> {
        debug!("Generating text representation of interface relationships");
        
        let hierarchy = self.get_extension_hierarchy()?;
        let mut result = String::from("Interface Extension Relationships:\n\n");
        
        if hierarchy.is_empty() {
            result.push_str("No interface relationships found.\n");
            return Ok(result);
        }
        
        let mut sorted_interfaces: Vec<_> = hierarchy.keys().collect();
        sorted_interfaces.sort();
        
        for interface in sorted_interfaces {
            let extensions = hierarchy.get(interface).unwrap();
            if !extensions.is_empty() {
                result.push_str(&format!("{} extends:\n", interface));
                for extension in extensions {
                    result.push_str(&format!("  - {}\n", extension));
                }
                result.push_str("\n");
            }
        }
        
        Ok(result)
    }
    
    #[instrument(skip(self, hierarchy, options), level = "debug")]
    fn generate_ascii_tree(&self, hierarchy: &HashMap<String, Vec<String>>, options: &VisualizationOptions) -> Result<String, Error> {
        // Use the existing implementation
        self.generate_ascii_tree(hierarchy, options)
    }
    
    #[instrument(skip(self, hierarchy, options), level = "debug")]
    fn generate_dot_graph(&self, hierarchy: &HashMap<String, Vec<String>>, options: &VisualizationOptions) -> Result<String, Error> {
        debug!("Generating DOT graph with custom hierarchy");
        
        let mut dot = String::from("digraph CustomHierarchy {\n");
        dot.push_str("  rankdir=BT;\n"); // Bottom to top
        dot.push_str("  node [shape=box, style=filled, fillcolor=lightblue];\n\n");
        
        // Collect all interfaces
        let mut all_interfaces = HashSet::new();
        for (source, targets) in hierarchy {
            all_interfaces.insert(source.clone());
            for target in targets {
                all_interfaces.insert(target.clone());
            }
        }
        
        // Add nodes
        for interface in &all_interfaces {
            dot.push_str(&format!("  \"{}\" [label=\"{}\"];\n", interface, interface));
        }
        
        dot.push_str("\n");
        
        // Add edges
        for (source, targets) in hierarchy {
            for target in targets {
                dot.push_str(&format!("  \"{}\" -> \"{}\";\n", source, target));
            }
        }
        
        dot.push_str("}\n");
        
        Ok(dot)
    }
    
    #[instrument(skip(self, hierarchy, options), level = "debug")]
    fn generate_json_representation(&self, hierarchy: &HashMap<String, Vec<String>>, options: &VisualizationOptions) -> Result<String, Error> {
        debug!("Generating JSON representation");
        
        // Convert hierarchy to a simple JSON structure
        let mut json = String::from("{\n");
        json.push_str("  \"interfaces\": {\n");
        
        let mut first = true;
        for (interface, extensions) in hierarchy {
            if !first {
                json.push_str(",\n");
            }
            first = false;
            
            json.push_str(&format!("    \"{}\": [", interface));
            
            for (i, extension) in extensions.iter().enumerate() {
                if i > 0 {
                    json.push_str(", ");
                }
                json.push_str(&format!("\"{}\"", extension));
            }
            
            json.push_str("]");
        }
        
        json.push_str("\n  }\n");
        json.push_str("}\n");
        
        Ok(json)
    }
}

/// Implementation of InterfaceRegistryExtensionWithVisualization trait for ThreadSafeInterfaceExtensionRegistry
impl InterfaceRegistryExtensionWithVisualization for ThreadSafeInterfaceExtensionRegistry {
    fn get_inheritance_distance(&self, source: &str, target: &str) -> Result<Option<usize>, Error> {
        // Use shortest path to find distance
        if let Some(path) = self.find_shortest_path(source, target)? {
            Ok(Some(path.len() - 1)) // Distance is path length minus 1
        } else {
            Ok(None)
        }
    }
    
    fn find_all_paths(&self, source: &str, target: &str) -> Result<Vec<Vec<String>>, Error> {
        self.find_all_paths_between(source, target)
    }
    
    fn find_diamond_inheritance_patterns(&self) -> Result<Vec<(String, String, String, String)>, Error> {
        let mut patterns = Vec::new();
        let hierarchy = self.get_extension_hierarchy()?;
        let interfaces = self.get_all_interfaces()?;
        
        // Look for diamond patterns (A -> B, A -> C, B -> D, C -> D)
        for a in &interfaces {
            if let Some(a_extends) = hierarchy.get(a) {
                for b in a_extends {
                    for c in a_extends {
                        if b != c {
                            if let (Some(b_extends), Some(c_extends)) = (hierarchy.get(b), hierarchy.get(c)) {
                                for d in b_extends {
                                    if c_extends.contains(d) {
                                        patterns.push((a.clone(), b.clone(), c.clone(), d.clone()));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(patterns)
    }
    
    fn detect_cycles(&self) -> Result<Vec<Vec<String>>, Error> {
        let mut cycles = Vec::new();
        let hierarchy = self.get_extension_hierarchy()?;
        let interfaces = self.get_all_interfaces()?;
        
        for start in &interfaces {
            let mut visited = HashSet::new();
            let mut path = Vec::new();
            self.find_cycles_dfs(start, &hierarchy, &mut visited, &mut path, &mut cycles);
        }
        
        Ok(cycles)
    }
    
    fn visualize_hierarchy_ascii(&self) -> Result<String, Error> {
        let hierarchy = self.get_extension_hierarchy()?;
        let options = VisualizationOptions::default();
        self.generate_ascii_tree(&hierarchy, &options)
    }
    
    fn visualize_hierarchy_dot(&self) -> Result<String, Error> {
        self.generate_dot_diagram()
    }
    
    fn visualize_path_ascii(&self, path: &[String]) -> Result<String, Error> {
        let mut result = String::new();
        for (i, interface) in path.iter().enumerate() {
            if i > 0 {
                result.push_str(" -> ");
            }
            result.push_str(interface);
        }
        Ok(result)
    }
    
    fn visualize_path_dot(&self, path: &[String]) -> Result<String, Error> {
        let mut dot = String::from("digraph InheritancePath {\n");
        dot.push_str("  rankdir=BT;\n");
        dot.push_str("  node [shape=box, style=filled, fillcolor=lightblue];\n\n");
        
        for interface in path {
            dot.push_str(&format!("  \"{}\";\n", interface));
        }
        
        for i in 0..path.len()-1 {
            dot.push_str(&format!("  \"{}\" -> \"{}\";\n", path[i], path[i+1]));
        }
        
        dot.push_str("}\n");
        Ok(dot)
    }
    
    fn is_visualization_initialized(&self) -> Result<bool, Error> {
        // For ThreadSafeInterfaceExtensionRegistry, always return true as it doesn't need initialization
        Ok(true)
    }
    
    fn set_visualization_initialized(&self, _initialized: bool) -> Result<(), Error> {
        // For ThreadSafeInterfaceExtensionRegistry, this is a no-op
        Ok(())
    }
}

/// Implementation of InterfaceRegistryExtensionWithVisualization trait for Arc<RwLock<ThreadSafeInterfaceExtensionRegistry>>
impl InterfaceRegistryExtensionWithVisualization for Arc<RwLock<ThreadSafeInterfaceExtensionRegistry>> {
    fn get_inheritance_distance(&self, source: &str, target: &str) -> Result<Option<usize>, Error> {
        let registry = self.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.get_inheritance_distance(source, target)
    }
    
    fn find_all_paths(&self, source: &str, target: &str) -> Result<Vec<Vec<String>>, Error> {
        let registry = self.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.find_all_paths(source, target)
    }
    
    fn find_diamond_inheritance_patterns(&self) -> Result<Vec<(String, String, String, String)>, Error> {
        let registry = self.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.find_diamond_inheritance_patterns()
    }
    
    fn detect_cycles(&self) -> Result<Vec<Vec<String>>, Error> {
        let registry = self.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.detect_cycles()
    }
    
    fn visualize_hierarchy_ascii(&self) -> Result<String, Error> {
        let registry = self.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.visualize_hierarchy_ascii()
    }
    
    fn visualize_hierarchy_dot(&self) -> Result<String, Error> {
        let registry = self.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.visualize_hierarchy_dot()
    }
    
    fn visualize_path_ascii(&self, path: &[String]) -> Result<String, Error> {
        let registry = self.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.visualize_path_ascii(path)
    }
    
    fn visualize_path_dot(&self, path: &[String]) -> Result<String, Error> {
        let registry = self.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.visualize_path_dot(path)
    }
    
    fn is_visualization_initialized(&self) -> Result<bool, Error> {
        let registry = self.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.is_visualization_initialized()
    }
    
    fn set_visualization_initialized(&self, initialized: bool) -> Result<(), Error> {
        let registry = self.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.set_visualization_initialized(initialized)
    }
}

/// Implementation of InterfaceRegistryVisualization trait for Arc<RwLock<ThreadSafeInterfaceExtensionRegistry>>
impl InterfaceRegistryVisualization for Arc<RwLock<ThreadSafeInterfaceExtensionRegistry>> {
    #[instrument(skip(self), level = "debug")]
    fn generate_dot_diagram(&self) -> Result<String, Error> {
        let registry = self.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.generate_dot_diagram()
    }
    
    #[instrument(skip(self), level = "debug")]
    fn generate_inheritance_path_diagram(&self, source: &str, target: &str) -> Result<String, Error> {
        let registry = self.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.generate_inheritance_path_diagram(source, target)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn generate_interface_diagram(&self, interface: &str) -> Result<String, Error> {
        let registry = self.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.generate_interface_diagram(interface)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn get_text_representation(&self) -> Result<String, Error> {
        let registry = self.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.get_text_representation()
    }
    
    #[instrument(skip(self, hierarchy, options), level = "debug")]
    fn generate_ascii_tree(&self, hierarchy: &HashMap<String, Vec<String>>, options: &VisualizationOptions) -> Result<String, Error> {
        let registry = self.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.generate_ascii_tree(hierarchy, options)
    }
    
    #[instrument(skip(self, hierarchy, options), level = "debug")]
    fn generate_dot_graph(&self, hierarchy: &HashMap<String, Vec<String>>, options: &VisualizationOptions) -> Result<String, Error> {
        let registry = self.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.generate_dot_graph(hierarchy, options)
    }
    
    #[instrument(skip(self, hierarchy, options), level = "debug")]
    fn generate_json_representation(&self, hierarchy: &HashMap<String, Vec<String>>, options: &VisualizationOptions) -> Result<String, Error> {
        let registry = self.read().map_err(|_| Error::Internal("Failed to acquire read lock".to_string()))?;
        registry.generate_json_representation(hierarchy, options)
    }
}