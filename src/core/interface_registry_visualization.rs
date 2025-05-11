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

/// Options for visualization output
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
pub enum VisualizationFormat {
    /// ASCII tree format
    Ascii,
    /// DOT graph format (Graphviz)
    Dot,
    /// JSON format
    Json,
}

/// Type alias for thread-safe interface registry visualization
pub type ThreadSafeInterfaceRegistryVisualization = dyn InterfaceRegistryVisualization + Send + Sync;

/// Extension trait for interface registry visualization
pub trait InterfaceRegistryVisualization {
    /// Generate a DOT format diagram of all interface relationships
    fn generate_dot_diagram(&self) -> Result<String, Error>;
    
    /// Generate a DOT format diagram of inheritance paths between two interfaces
    fn generate_inheritance_path_diagram(&self, source: &str, target: &str) -> Result<String, Error>;
    
    /// Generate a DOT format diagram of a specific interface's relationships
    fn generate_interface_diagram(&self, interface: &str) -> Result<String, Error>;
    
    /// Get a text representation of interface relationships
    fn get_text_representation(&self) -> Result<String, Error>;
}

/// Combined extension trait for interface registry with visualization
pub trait InterfaceRegistryExtensionWithVisualization: InterfaceRegistryExtension + InterfaceRegistryVisualization {
    /// Get the inheritance distance between two interfaces
    fn get_inheritance_distance(&self, source: &str, target: &str) -> Result<Option<usize>, Error>;
    
    /// Find all paths between two interfaces
    fn find_all_paths(&self, source: &str, target: &str) -> Result<Vec<Vec<String>>, Error>;
    
    /// Get interfaces that form a diamond inheritance pattern
    fn find_diamond_inheritance_patterns(&self) -> Result<Vec<(String, String, String, String)>, Error>;
    
    /// Register an extension relationship between interfaces
    fn register_extension(&self, source: &str, target: &str) -> Result<(), Error>;
    
    /// Get the complete extension hierarchy
    fn get_extension_hierarchy(&self) -> Result<HashMap<String, Vec<String>>, Error>;
    
    /// Get all interfaces in the registry
    fn get_all_interfaces(&self) -> Result<HashSet<String>, Error>;
    
    /// Get all direct extensions of an interface
    fn get_direct_extensions(&self, interface: &str) -> Result<Option<Vec<String>>, Error>;
    
    /// Get all direct implementors of an interface
    fn get_direct_implementors(&self, interface: &str) -> Result<Option<Vec<String>>, Error>;
    
    /// Check if an interface extends another interface
    fn extends(&self, source: &str, target: &str) -> Result<bool, Error>;
    
    /// Get all extensions of an interface (transitive closure)
    fn get_all_extensions(&self, interface: &str) -> Result<HashSet<String>, Error>;
    
    /// Get all implementors of an interface (transitive closure)
    fn get_all_implementors(&self, interface: &str) -> Result<HashSet<String>, Error>;
    
    /// Find the inheritance path from source to target
    fn find_inheritance_path(&self, source: &str, target: &str) -> Result<Vec<String>, Error>;
    
    /// Find all inheritance paths from source to target
    fn find_all_inheritance_paths(&self, source: &str, target: &str) -> Result<Vec<Vec<String>>, Error>;
    
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
    
    /// Check if an interface exists in the registry
    fn interface_exists(&self, interface: &str) -> Result<bool, Error>;
    
    /// Generate ASCII tree visualization of a hierarchy
    fn generate_ascii_tree(&self, hierarchy: &HashMap<String, Vec<String>>, options: &VisualizationOptions) -> Result<String, Error>;
    
    /// Generate DOT graph visualization of a hierarchy
    fn generate_dot_graph(&self, hierarchy: &HashMap<String, Vec<String>>, options: &VisualizationOptions) -> Result<String, Error>;
    
    /// Generate JSON representation of a hierarchy
    fn generate_json_representation(&self, hierarchy: &HashMap<String, Vec<String>>, options: &VisualizationOptions) -> Result<String, Error>;
    
    /// Check if the visualization system is initialized
    fn is_visualization_initialized(&self) -> Result<bool, Error>;
    
    /// Set the initialized status of the visualization system
    fn set_visualization_initialized(&self, initialized: bool) -> Result<(), Error>;
}

impl<T: InterfaceRegistryExtension + ?Sized> InterfaceRegistryVisualization for T {
    #[instrument(skip(self), level = "debug")]
    fn generate_dot_diagram(&self) -> Result<String, Error> {
        let interfaces = match self.get_all_interfaces() {
            Some(ifs) => ifs,
            None => return Err(Error::Internal("No interfaces found".to_string())),
        };
        
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
            if let Ok(Some(extensions)) = self.get_direct_extensions(source) {
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
        let interfaces = match self.get_all_interfaces() {
            Some(ifs) => ifs,
            None => return Err(Error::Internal("No interfaces found".to_string())),
        };
        
        if !interfaces.contains(interface) {
            return Err(Error::NotFound(format!("Interface '{}' not found", interface)));
        }
        
        // Collect all related interfaces (extends or extended by)
        let mut related = HashSet::new();
        related.insert(interface.to_string());
        
        // Add interfaces that this interface extends
        if let Ok(Some(extensions)) = self.get_direct_extensions(interface) {
            for ext in extensions {
                related.insert(ext);
            }
        }
        
        // Add interfaces that extend this interface
        for other in &interfaces {
            if let Ok(Some(extensions)) = self.get_direct_extensions(other) {
                if extensions.contains(interface) {
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
            if let Ok(Some(extensions)) = self.get_direct_extensions(source) {
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
        let interfaces = match self.get_all_interfaces() {
            Some(ifs) => ifs,
            None => return Err(Error::Internal("No interfaces found".to_string())),
        };
        
        let mut result = String::new();
        result.push_str("Interface Hierarchy:\n\n");
        
        let mut sorted_interfaces: Vec<_> = interfaces.iter().collect();
        sorted_interfaces.sort();
        
        for interface in sorted_interfaces {
            result.push_str(&format!("{}:\n", interface));
            
            // Show what this interface extends
            if let Ok(Some(extensions)) = self.get_direct_extensions(interface) {
                let mut sorted_extensions: Vec<_> = extensions.iter().collect();
                sorted_extensions.sort();
                
                for ext in sorted_extensions {
                    result.push_str(&format!("  extends {}\n", ext));
                }
            }
            
            // Show what extends this interface
            let mut implementers = Vec::new();
            for other in &interfaces {
                if let Ok(Some(extensions)) = self.get_direct_extensions(other) {
                    if extensions.contains(interface) {
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

/// Helper method for finding all paths between interfaces using DFS
trait FindAllPathsDfs {
    fn find_all_paths_dfs(
        &self,
        source: &str,
        target: &str,
        path: &mut Vec<String>,
        visited: &mut HashSet<String>,
        all_paths: &mut Vec<Vec<String>>
    ) -> Result<(), Error>;
}

impl<T: InterfaceRegistryExtension + ?Sized> FindAllPathsDfs for T {
    fn find_all_paths_dfs(
        &self,
        source: &str,
        target: &str,
        path: &mut Vec<String>,
        visited: &mut HashSet<String>,
        all_paths: &mut Vec<Vec<String>>
    ) -> Result<(), Error> {
        // Check if we've reached the target
        if source == target {
            all_paths.push(path.clone());
            return Ok(());
        }
        
        // Explore all direct extensions
        if let Ok(Some(extensions)) = self.get_direct_extensions(source) {
            for ext in extensions {
                if !visited.contains(&ext) {
                    // Mark as visited to avoid cycles
                    visited.insert(ext.clone());
                    path.push(ext.clone());
                    
                    // Recursive DFS
                    self.find_all_paths_dfs(&ext, target, path, visited, all_paths)?;
                    
                    // Backtrack
                    path.pop();
                    visited.remove(&ext);
                }
            }
        }
        
        Ok(())
    }
}

impl<T: InterfaceRegistryExtension + ?Sized> InterfaceRegistryExtensionWithVisualization for T {
    #[instrument(skip(self), level = "debug")]
    fn register_extension(&self, source: &str, target: &str) -> Result<(), Error> {
        // This is a stub implementation that just delegates to the trait method
        // In a real implementation, this would register the extension in the registry
        Err(Error::Internal("Extension registration through visualization trait not implemented".to_string()))
    }
    
    #[instrument(skip(self), level = "debug")]
    fn get_extension_hierarchy(&self) -> Result<HashMap<String, Vec<String>>, Error> {
        let interfaces = match self.get_all_interfaces() {
            Some(ifs) => ifs,
            None => return Err(Error::Internal("No interfaces found".to_string())),
        };
        
        let mut hierarchy = HashMap::new();
        
        for interface in &interfaces {
            if let Ok(Some(extensions)) = self.get_direct_extensions(interface) {
                hierarchy.insert(interface.to_string(), extensions);
            }
        }
        
        Ok(hierarchy)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn get_all_interfaces(&self) -> Result<HashSet<String>, Error> {
        match InterfaceRegistryExtension::get_all_interfaces(self) {
            Some(interfaces) => Ok(interfaces),
            None => Err(Error::Internal("No interfaces found".to_string())),
        }
    }
    
    #[instrument(skip(self), level = "debug")]
    fn get_direct_extensions(&self, interface: &str) -> Result<Option<Vec<String>>, Error> {
        match InterfaceRegistryExtension::get_direct_extensions(self, interface)? {
            Some(extensions) => Ok(Some(extensions.into_iter().collect())),
            None => Ok(None),
        }
    }
    
    #[instrument(skip(self), level = "debug")]
    fn get_direct_implementors(&self, interface: &str) -> Result<Option<Vec<String>>, Error> {
        match InterfaceRegistryExtension::get_direct_implementers(self, interface)? {
            Some(implementers) => Ok(Some(implementers.into_iter().collect())),
            None => Ok(None),
        }
    }
    
    #[instrument(skip(self), level = "debug")]
    fn extends(&self, source: &str, target: &str) -> Result<bool, Error> {
        InterfaceRegistryExtension::extends(self, source, target)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn get_all_extensions(&self, interface: &str) -> Result<HashSet<String>, Error> {
        // Get all interfaces that this one extends directly or indirectly
        let mut result = HashSet::new();
        let mut visited = HashSet::new();
        let mut queue = Vec::new();
        
        queue.push(interface.to_string());
        visited.insert(interface.to_string());
        
        while let Some(current) = queue.pop() {
            if let Ok(Some(extensions)) = self.get_direct_extensions(&current) {
                for ext in extensions {
                    if !visited.contains(&ext) {
                        visited.insert(ext.clone());
                        queue.push(ext.clone());
                        result.insert(ext);
                    }
                }
            }
        }
        
        Ok(result)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn get_all_implementors(&self, interface: &str) -> Result<HashSet<String>, Error> {
        // Get all interfaces that extend this one directly or indirectly
        let mut result = HashSet::new();
        let mut visited = HashSet::new();
        let mut queue = Vec::new();
        
        queue.push(interface.to_string());
        visited.insert(interface.to_string());
        
        while let Some(current) = queue.pop() {
            if let Ok(Some(implementers)) = self.get_direct_implementors(&current) {
                for impl_ in implementers {
                    if !visited.contains(&impl_) {
                        visited.insert(impl_.clone());
                        queue.push(impl_.clone());
                        result.insert(impl_);
                    }
                }
            }
        }
        
        Ok(result)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn find_inheritance_path(&self, source: &str, target: &str) -> Result<Vec<String>, Error> {
        if !self.extends(source, target)? {
            return Err(Error::Validation(format!("No inheritance path from '{}' to '{}'", source, target)));
        }
        
        // If source and target are the same, return a trivial path
        if source == target {
            return Ok(vec![source.to_string()]);
        }
        
        // Use BFS to find the shortest path
        let mut visited = HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        let mut parent = HashMap::new();
        
        queue.push_back(source.to_string());
        visited.insert(source.to_string());
        
        // Find the path
        while let Some(current) = queue.pop_front() {
            if current == target {
                // We found the target, reconstruct the path
                let mut path = Vec::new();
                let mut node = current;
                
                // Work backwards from target to source
                while node != source {
                    path.push(node.clone());
                    node = parent.get(&node).unwrap().clone();
                }
                
                // Add the source
                path.push(source.to_string());
                
                // Reverse to get the path from source to target
                path.reverse();
                
                return Ok(path);
            }
            
            // Explore neighbors
            if let Ok(Some(extensions)) = self.get_direct_extensions(&current) {
                for ext in extensions {
                    if !visited.contains(&ext) {
                        visited.insert(ext.clone());
                        queue.push_back(ext.clone());
                        parent.insert(ext, current.clone());
                    }
                }
            }
        }
        
        // No path found
        Err(Error::Validation(format!("No inheritance path from '{}' to '{}'", source, target)))
    }
    
    #[instrument(skip(self), level = "debug")]
    fn find_all_inheritance_paths(&self, source: &str, target: &str) -> Result<Vec<Vec<String>>, Error> {
        if !self.extends(source, target)? {
            return Err(Error::Validation(format!("No inheritance paths from '{}' to '{}'", source, target)));
        }
        
        // If source and target are the same, return a trivial path
        if source == target {
            return Ok(vec![vec![source.to_string()]]);
        }
        
        // Use DFS to find all paths
        let mut all_paths = Vec::new();
        let mut current_path = vec![source.to_string()];
        let mut visited = HashSet::new();
        visited.insert(source.to_string());
        
        self.find_all_paths_dfs(source, target, &mut current_path, &mut visited, &mut all_paths)?;
        
        if all_paths.is_empty() {
            Err(Error::Validation(format!("No inheritance paths from '{}' to '{}'", source, target)))
        } else {
            Ok(all_paths)
        }
    }
    
    #[instrument(skip(self), level = "debug")]
    fn detect_cycles(&self) -> Result<Vec<Vec<String>>, Error> {
        // Get the complete hierarchy
        let hierarchy = self.get_extension_hierarchy()?;
        
        // Use the helper function
        detect_cycles(&hierarchy)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn visualize_hierarchy_ascii(&self) -> Result<String, Error> {
        let hierarchy = self.get_extension_hierarchy()?;
        let options = VisualizationOptions::default();
        self.generate_ascii_tree(&hierarchy, &options)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn visualize_hierarchy_dot(&self) -> Result<String, Error> {
        let hierarchy = self.get_extension_hierarchy()?;
        let options = VisualizationOptions::default();
        self.generate_dot_graph(&hierarchy, &options)
    }
    
    #[instrument(skip(self), level = "debug")]
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
    
    #[instrument(skip(self), level = "debug")]
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
    
    #[instrument(skip(self), level = "debug")]
    fn interface_exists(&self, interface: &str) -> Result<bool, Error> {
        match self.get_all_interfaces() {
            Ok(interfaces) => Ok(interfaces.contains(interface)),
            Err(e) => Err(e),
        }
    }
    
    #[instrument(skip(self, hierarchy, options), level = "debug")]
    fn generate_ascii_tree(&self, hierarchy: &HashMap<String, Vec<String>>, options: &VisualizationOptions) -> Result<String, Error> {
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
    
    #[instrument(skip(self, hierarchy, options), level = "debug")]
    fn generate_dot_graph(&self, hierarchy: &HashMap<String, Vec<String>>, options: &VisualizationOptions) -> Result<String, Error> {
        let mut dot = String::new();
        dot.push_str("digraph interface_hierarchy {\n");
        dot.push_str("  rankdir=BT;\n");
        dot.push_str("  node [shape=box, style=filled, fillcolor=lightblue];\n\n");
        
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
            dot.push_str(&format!("  \"{}\" [label=\"{}\"];\n", interface, interface));
        }
        
        dot.push_str("\n");
        
        // Add edges
        for (source, targets) in hierarchy {
            for target in targets {
                dot.push_str(&format!("  \"{}\" -> \"{}\";\n", source, target));
            }
        }
        
        // If include_cycles is enabled, detect and highlight cycles
        if options.include_cycles {
            if let Ok(cycles) = detect_cycles(hierarchy) {
                if !cycles.is_empty() {
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
                }
            }
        }
        
        dot.push_str("}\n");
        
        Ok(dot)
    }
    
    #[instrument(skip(self, hierarchy, options), level = "debug")]
    fn generate_json_representation(&self, hierarchy: &HashMap<String, Vec<String>>, options: &VisualizationOptions) -> Result<String, Error> {
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
            
            if let Ok(cycles) = detect_cycles(hierarchy) {
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
            }
            
            result.push_str("]\n");
        }
        
        result.push_str("}\n");
        
        Ok(result)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn is_visualization_initialized(&self) -> Result<bool, Error> {
        // Determine if this registry is initialized by checking for interfaces
        match self.get_all_interfaces() {
            Ok(interfaces) => Ok(!interfaces.is_empty()),
            Err(e) => Err(e),
        }
    }
    
    #[instrument(skip(self), level = "debug")]
    fn set_visualization_initialized(&self, _initialized: bool) -> Result<(), Error> {
        // This is a no-op for the default implementation
        Ok(())
    }
    
    #[instrument(skip(self), level = "debug")]
    fn get_inheritance_distance(&self, source: &str, target: &str) -> Result<Option<usize>, Error> {
        if !self.extends(source, target)? {
            return Ok(None);
        }
        
        // Find the shortest path
        let mut visited = HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        let mut distances = HashMap::new();
        
        queue.push_back(source.to_string());
        distances.insert(source.to_string(), 0);
        visited.insert(source.to_string());
        
        while let Some(current) = queue.pop_front() {
            let current_dist = *distances.get(&current).unwrap_or(&0);
            
            if current == target {
                return Ok(Some(current_dist));
            }
            
            if let Ok(Some(extensions)) = self.get_direct_extensions(&current) {
                for next in extensions {
                    if !visited.contains(&next) {
                        visited.insert(next.clone());
                        queue.push_back(next.clone());
                        distances.insert(next, current_dist + 1);
                    }
                }
            }
        }
        
        Ok(None)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn find_all_paths(&self, source: &str, target: &str) -> Result<Vec<Vec<String>>, Error> {
        if !self.extends(source, target)? {
            return Ok(Vec::new());
        }
        
        let mut all_paths = Vec::new();
        let mut path = vec![source.to_string()];
        let mut visited = HashSet::new();
        visited.insert(source.to_string());
        
        self.find_all_paths_recursive(source, target, &mut path, &mut visited, &mut all_paths)?;
        
        Ok(all_paths)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn find_diamond_inheritance_patterns(&self) -> Result<Vec<(String, String, String, String)>, Error> {
        let interfaces = match self.get_all_interfaces() {
            Some(ifs) => ifs,
            None => return Ok(Vec::new()),
        };
        
        let mut diamonds = Vec::new();
        
        // For each interface D, check if it extends two different interfaces B and C
        // that both extend a common interface A
        for d in &interfaces {
            if let Ok(Some(d_extensions)) = self.get_direct_extensions(d) {
                if d_extensions.len() >= 2 {
                    // Get all pairs of direct extensions
                    let d_exts: Vec<_> = d_extensions.iter().collect();
                    
                    for i in 0..d_exts.len() {
                        for j in i+1..d_exts.len() {
                            let b = d_exts[i];
                            let c = d_exts[j];
                            
                            // Check if B and C have a common ancestor
                            if let Ok(Some(common_ancestor)) = self.find_common_ancestor(b, c) {
                                // Check that both B and C directly or indirectly extend A
                                if self.extends(b, &common_ancestor)? && self.extends(c, &common_ancestor)? {
                                    // Ensure B and C are different paths to A
                                    if !self.extends(b, c)? && !self.extends(c, b)? {
                                        diamonds.push((
                                            common_ancestor.clone(),
                                            b.clone(),
                                            c.clone(),
                                            d.clone()
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(diamonds)
    }
}

// Helper methods for recursive path finding
#[instrument(skip(registry, path, visited, all_paths), level = "trace")]
fn find_all_paths_recursive<T: InterfaceRegistryExtension + ?Sized>(
    registry: &T,
    current: &str,
    target: &str,
    path: &mut Vec<String>,
    visited: &mut HashSet<String>,
    all_paths: &mut Vec<Vec<String>>
) -> Result<(), Error> {
    if current == target {
        all_paths.push(path.clone());
        return Ok(());
    }
    
    if let Ok(Some(extensions)) = registry.get_direct_extensions(current) {
        for next in extensions {
            if !visited.contains(&next) {
                // Mark as visited
                visited.insert(next.clone());
                path.push(next.clone());
                
                // Recursively find paths
                find_all_paths_recursive(registry, &next, target, path, visited, all_paths)?;
                
                // Backtrack
                path.pop();
                visited.remove(&next);
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::interface_registry_extensions::create_interface_registry;
    use crate::tests::common;
    
    #[test]
    fn test_generate_dot_diagram() {
        common::tracing::setup();
        
        let registry = create_interface_registry("test", &[
            "A", "B", "C", "D"
        ]);
        
        let mut reg = registry.write().unwrap();
        reg.register_extension("B", "A").unwrap();
        reg.register_extension("C", "A").unwrap();
        reg.register_extension("D", "B").unwrap();
        reg.register_extension("D", "C").unwrap();
        
        let dot = reg.generate_dot_diagram().unwrap();
        
        // Basic validation of the DOT output
        assert!(dot.contains("digraph InterfaceHierarchy"));
        assert!(dot.contains("\"A\" [label=\"A\"];"));
        assert!(dot.contains("\"B\" -> \"A\";"));
        assert!(dot.contains("\"C\" -> \"A\";"));
        assert!(dot.contains("\"D\" -> \"B\";"));
        assert!(dot.contains("\"D\" -> \"C\";"));
    }
    
    #[test]
    fn test_generate_inheritance_path_diagram() {
        common::tracing::setup();
        
        let registry = create_interface_registry("test", &[
            "A", "B", "C", "D", "E"
        ]);
        
        let mut reg = registry.write().unwrap();
        reg.register_extension("B", "A").unwrap();
        reg.register_extension("C", "B").unwrap();
        reg.register_extension("D", "C").unwrap();
        reg.register_extension("E", "D").unwrap();
        
        let dot = reg.generate_inheritance_path_diagram("E", "A").unwrap();
        
        // Basic validation of the DOT output
        assert!(dot.contains("digraph InheritancePath"));
        assert!(dot.contains("\"E\" [label=\"E\", fillcolor=lightgreen];"));
        assert!(dot.contains("\"A\" [label=\"A\", fillcolor=lightpink];"));
        assert!(dot.contains("\"E\" -> \"D\";"));
        assert!(dot.contains("\"D\" -> \"C\";"));
        assert!(dot.contains("\"C\" -> \"B\";"));
        assert!(dot.contains("\"B\" -> \"A\";"));
    }
    
    #[test]
    fn test_generate_interface_diagram() {
        common::tracing::setup();
        
        let registry = create_interface_registry("test", &[
            "A", "B", "C", "D"
        ]);
        
        let mut reg = registry.write().unwrap();
        reg.register_extension("B", "A").unwrap();
        reg.register_extension("C", "A").unwrap();
        reg.register_extension("D", "B").unwrap();
        reg.register_extension("D", "C").unwrap();
        
        let dot = reg.generate_interface_diagram("A").unwrap();
        
        // Basic validation of the DOT output
        assert!(dot.contains("digraph InterfaceDiagram"));
        assert!(dot.contains("\"A\" [label=\"A\", fillcolor=gold];"));
        assert!(dot.contains("\"B\" [label=\"B\", fillcolor=lightblue];"));
        assert!(dot.contains("\"C\" [label=\"C\", fillcolor=lightblue];"));
        assert!(dot.contains("\"B\" -> \"A\";"));
        assert!(dot.contains("\"C\" -> \"A\";"));
    }
    
    #[test]
    fn test_get_text_representation() {
        common::tracing::setup();
        
        let registry = create_interface_registry("test", &[
            "A", "B", "C", "D"
        ]);
        
        let mut reg = registry.write().unwrap();
        reg.register_extension("B", "A").unwrap();
        reg.register_extension("C", "A").unwrap();
        reg.register_extension("D", "B").unwrap();
        reg.register_extension("D", "C").unwrap();
        
        let text = reg.get_text_representation().unwrap();
        
        // Basic validation of the text output
        assert!(text.contains("Interface Hierarchy:"));
        assert!(text.contains("A:"));
        assert!(text.contains("extended by B"));
        assert!(text.contains("extended by C"));
        assert!(text.contains("B:"));
        assert!(text.contains("extends A"));
        assert!(text.contains("extended by D"));
        assert!(text.contains("C:"));
        assert!(text.contains("extends A"));
        assert!(text.contains("extended by D"));
        assert!(text.contains("D:"));
        assert!(text.contains("extends B"));
        assert!(text.contains("extends C"));
    }
    
    #[test]
    fn test_get_inheritance_distance() {
        common::tracing::setup();
        
        let registry = create_interface_registry("test", &[
            "A", "B", "C", "D", "E"
        ]);
        
        let mut reg = registry.write().unwrap();
        reg.register_extension("B", "A").unwrap();
        reg.register_extension("C", "B").unwrap();
        reg.register_extension("D", "C").unwrap();
        reg.register_extension("E", "D").unwrap();
        
        assert_eq!(reg.get_inheritance_distance("E", "E").unwrap(), Some(0));
        assert_eq!(reg.get_inheritance_distance("E", "D").unwrap(), Some(1));
        assert_eq!(reg.get_inheritance_distance("E", "C").unwrap(), Some(2));
        assert_eq!(reg.get_inheritance_distance("E", "B").unwrap(), Some(3));
        assert_eq!(reg.get_inheritance_distance("E", "A").unwrap(), Some(4));
        assert_eq!(reg.get_inheritance_distance("A", "E").unwrap(), None);
    }
    
    #[test]
    fn test_find_all_paths() {
        common::tracing::setup();
        
        let registry = create_interface_registry("test", &[
            "A", "B", "C", "D", "E", "F"
        ]);
        
        let mut reg = registry.write().unwrap();
        reg.register_extension("B", "A").unwrap();
        reg.register_extension("C", "A").unwrap();
        reg.register_extension("D", "B").unwrap();
        reg.register_extension("D", "C").unwrap();
        reg.register_extension("E", "C").unwrap();
        reg.register_extension("F", "D").unwrap();
        reg.register_extension("F", "E").unwrap();
        
        let paths = reg.find_all_paths("F", "A").unwrap();
        
        // There should be two paths: F->D->B->A and F->D->C->A or F->E->C->A
        assert_eq!(paths.len(), 3);
        
        // Check if the expected paths are present
        let path1 = vec!["F", "D", "B", "A"];
        let path2 = vec!["F", "D", "C", "A"];
        let path3 = vec!["F", "E", "C", "A"];
        
        let has_path1 = paths.iter().any(|p| {
            p.len() == path1.len() && 
            p.iter().zip(path1.iter()).all(|(a, b)| a == b)
        });
        
        let has_path2 = paths.iter().any(|p| {
            p.len() == path2.len() && 
            p.iter().zip(path2.iter()).all(|(a, b)| a == b)
        });
        
        let has_path3 = paths.iter().any(|p| {
            p.len() == path3.len() && 
            p.iter().zip(path3.iter()).all(|(a, b)| a == b)
        });
        
        assert!(has_path1, "Path F->D->B->A not found");
        assert!(has_path2, "Path F->D->C->A not found");
        assert!(has_path3, "Path F->E->C->A not found");
    }
    
    #[test]
    fn test_find_diamond_inheritance_patterns() {
        common::tracing::setup();
        
        let registry = create_interface_registry("test", &[
            "A", "B", "C", "D", "E", "F", "G"
        ]);
        
        let mut reg = registry.write().unwrap();
        // Create diamond A->B->D and A->C->D
        reg.register_extension("B", "A").unwrap();
        reg.register_extension("C", "A").unwrap();
        reg.register_extension("D", "B").unwrap();
        reg.register_extension("D", "C").unwrap();
        
        // Create another diamond E->F->G and E->B->G
        reg.register_extension("F", "E").unwrap();
        reg.register_extension("B", "E").unwrap();
        reg.register_extension("G", "F").unwrap();
        reg.register_extension("G", "B").unwrap();
        
        let diamonds = reg.find_diamond_inheritance_patterns().unwrap();
        
        // There should be at least two diamond patterns
        assert!(diamonds.len() >= 2);
        
        // Check if the expected diamonds are present
        let diamond1 = ("A".to_string(), "B".to_string(), "C".to_string(), "D".to_string());
        let diamond2 = ("E".to_string(), "F".to_string(), "B".to_string(), "G".to_string());
        
        let has_diamond1 = diamonds.iter().any(|(a, b, c, d)| {
            (a == &diamond1.0 && (
                (b == &diamond1.1 && c == &diamond1.2) || 
                (b == &diamond1.2 && c == &diamond1.1)
            ) && d == &diamond1.3)
        });
        
        let has_diamond2 = diamonds.iter().any(|(a, b, c, d)| {
            (a == &diamond2.0 && (
                (b == &diamond2.1 && c == &diamond2.2) ||
                (b == &diamond2.2 && c == &diamond2.1)
            ) && d == &diamond2.3)
        });
        
        assert!(has_diamond1, "Diamond A->B->D and A->C->D not found");
        assert!(has_diamond2, "Diamond E->F->G and E->B->G not found");
    }
}