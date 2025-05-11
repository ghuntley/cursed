//! # Interface Registry Visualization
//!
//! This module defines the trait for visualizing interface type assertions with improved
//! error handling and consistent error propagation. It extends the interface registry
//! with visualization capabilities for interface hierarchies, inheritance paths, and
//! improved error messages.
//!
//! ## Features
//!
//! 1. Thread-safe implementation for concurrent compilation scenarios
//! 2. Comprehensive error handling with rich context information
//! 3. Proper error propagation using the `?` operator throughout
//! 4. Support for generating DOT graphs for interface hierarchies
//! 5. Support for finding inheritance paths between interfaces
//! 6. Detailed error messages with inheritance path information

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use tracing::{debug, info, instrument, trace, warn};

use crate::error::Error;

/// Visualization format options for interface registry
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualizationFormat {
    /// ASCII art visualization
    Ascii,
    /// DOT graph visualization (for GraphViz)
    Dot,
    /// JSON representation
    Json,
    /// Plain text representation
    Text,
}

/// Options for customizing the visualization output
#[derive(Debug, Clone)]
pub struct VisualizationOptions {
    /// Whether to include cycle information in the visualization
    pub include_cycles: bool,
    /// Maximum depth to visualize (None for unlimited)
    pub max_depth: Option<usize>,
    /// Whether to include implementation details
    pub include_details: bool,
}

impl Default for VisualizationOptions {
    fn default() -> Self {
        Self {
            include_cycles: true,
            max_depth: None,
            include_details: true,
        }
    }
}

/// A trait for visualizing interface registry relationships with enhanced error handling.
pub trait InterfaceRegistryExtensionWithVisualization {
    /// Get the complete extension hierarchy with proper error propagation
    fn get_extension_hierarchy(&self) -> Result<HashMap<String, Vec<String>>, Error>;
    
    /// Get all interfaces in the registry with proper error handling
    fn get_all_interfaces(&self) -> Result<HashSet<String>, Error>;
    
    /// Get all direct extensions of an interface with proper error handling
    fn get_direct_extensions(&self, interface: &str) -> Result<Option<Vec<String>>, Error>;
    
    /// Get all direct implementors of an interface with proper error handling
    fn get_direct_implementors(&self, interface: &str) -> Result<Option<Vec<String>>, Error>;
    
    /// Check if an interface extends another interface (directly or indirectly)
    fn extends(&self, source: &str, target: &str) -> Result<bool, Error>;
    
    /// Get all interfaces that extend a given interface
    fn get_all_extensions(&self, interface: &str) -> Result<HashSet<String>, Error>;
    
    /// Get all interfaces that are extended by a given interface
    fn get_all_implementors(&self, interface: &str) -> Result<HashSet<String>, Error>;
    
    /// Find the shortest path from one interface to another through inheritance
    fn find_inheritance_path(&self, source: &str, target: &str) -> Result<Vec<String>, Error>;
    
    /// Find all possible paths from one interface to another through inheritance
    fn find_all_inheritance_paths(&self, source: &str, target: &str) -> Result<Vec<Vec<String>>, Error>;
    
    /// Detect if there are any cycles in the interface inheritance hierarchy
    fn detect_cycles(&self) -> Result<Vec<Vec<String>>, Error>;
    
    /// Generate a visualization of the interface hierarchy as ASCII art
    fn visualize_hierarchy_ascii(&self) -> Result<String, Error>;
    
    /// Generate a DOT graph representation of the interface hierarchy
    fn visualize_hierarchy_dot(&self) -> Result<String, Error>;
    
    /// Generate a visualization of a specific inheritance path as ASCII art
    fn visualize_path_ascii(&self, path: &[String]) -> Result<String, Error>;
    
    /// Generate a DOT graph representation of a specific inheritance path
    fn visualize_path_dot(&self, path: &[String]) -> Result<String, Error>;
    
    /// Check if an interface exists in the registry
    fn interface_exists(&self, interface: &str) -> Result<bool, Error>;
    
    /// Generate a visualization with the specified format and options
    fn generate_ascii_tree(&self, hierarchy: &HashMap<String, Vec<String>>, options: &VisualizationOptions) -> Result<String, Error>;
    
    /// Generate a DOT graph with the specified options
    fn generate_dot_graph(&self, hierarchy: &HashMap<String, Vec<String>>, options: &VisualizationOptions) -> Result<String, Error>;
    
    /// Generate a JSON representation with the specified options
    fn generate_json_representation(&self, hierarchy: &HashMap<String, Vec<String>>, options: &VisualizationOptions) -> Result<String, Error>;
    
    /// Check if the visualization system is initialized
    fn is_visualization_initialized(&self) -> Result<bool, Error>;
    
    /// Set whether the visualization system is initialized
    fn set_visualization_initialized(&self, initialized: bool) -> Result<(), Error>;
    
    /// Register a new extension relationship between interfaces
    fn register_extension(&self, source: &str, target: &str) -> Result<(), Error>;
}

/// A thread-safe implementation of the interface registry visualization
#[derive(Debug, Default)]
pub struct ThreadSafeInterfaceRegistryVisualization {
    /// The interface extension registry, stored as a mapping from interface to its direct extensions
    extensions: Arc<RwLock<HashMap<String, Vec<String>>>>,
    /// The interface implementor registry, stored as a mapping from interface to interfaces that extend it
    implementors: Arc<RwLock<HashMap<String, Vec<String>>>>,
    /// Flag indicating whether the visualization system is initialized
    initialized: Arc<RwLock<bool>>,
}

impl ThreadSafeInterfaceRegistryVisualization {
    /// Create a new thread-safe interface registry visualization
    pub fn new() -> Self {
        Self {
            extensions: Arc::new(RwLock::new(HashMap::new())),
            implementors: Arc::new(RwLock::new(HashMap::new())),
            initialized: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Register a new extension relationship between interfaces
    #[instrument(level = "debug")]
    pub fn register_extension(&self, source: &str, target: &str) -> Result<(), Error> {
        debug!("Registering extension relationship: {} extends {}", source, target);
        
        // Update extensions mapping
        {
            let mut extensions = self.extensions.write().map_err(|e| {
                Error::Compilation(format!("Failed to acquire write lock on extensions: {}", e))
            })?;
            
            extensions
                .entry(source.to_string())
                .or_insert_with(Vec::new)
                .push(target.to_string());
        }
        
        // Update implementors mapping
        {
            let mut implementors = self.implementors.write().map_err(|e| {
                Error::Compilation(format!("Failed to acquire write lock on implementors: {}", e))
            })?;
            
            implementors
                .entry(target.to_string())
                .or_insert_with(Vec::new)
                .push(source.to_string());
        }
        
        Ok(())
    }
}

impl InterfaceRegistryExtensionWithVisualization for ThreadSafeInterfaceRegistryVisualization {
    #[instrument(level = "debug")]
    fn register_extension(&self, source: &str, target: &str) -> Result<(), Error> {
        // Delegate to the instance method
        self.register_extension(source, target)
    }
    
    #[instrument(level = "debug")]
    fn get_extension_hierarchy(&self) -> Result<HashMap<String, Vec<String>>, Error> {
        debug!("Getting complete extension hierarchy");
        
        let extensions = self.extensions.read().map_err(|e| {
            Error::Compilation(format!("Failed to acquire read lock on extensions: {}", e))
        })?;
        
        // Create a deep copy to avoid holding the lock longer than necessary
        let hierarchy = extensions.clone();
        
        Ok(hierarchy)
    }
    
    #[instrument(level = "debug")]
    fn get_all_interfaces(&self) -> Result<HashSet<String>, Error> {
        debug!("Getting all interfaces in the registry");
        
        let extensions = self.extensions.read().map_err(|e| {
            Error::Compilation(format!("Failed to acquire read lock on extensions: {}", e))
        })?;
        
        let implementors = self.implementors.read().map_err(|e| {
            Error::Compilation(format!("Failed to acquire read lock on implementors: {}", e))
        })?;
        
        let mut all_interfaces = HashSet::new();
        
        // Add all source interfaces
        for source in extensions.keys() {
            all_interfaces.insert(source.clone());
        }
        
        // Add all target interfaces
        for (_, targets) in &*extensions {
            for target in targets {
                all_interfaces.insert(target.clone());
            }
        }
        
        // Add all interfaces from implementors mapping
        for target in implementors.keys() {
            all_interfaces.insert(target.clone());
        }
        
        Ok(all_interfaces)
    }
    
    #[instrument(level = "debug")]
    fn get_direct_extensions(&self, interface: &str) -> Result<Option<Vec<String>>, Error> {
        debug!("Getting direct extensions of interface: {}", interface);
        
        let extensions = self.extensions.read().map_err(|e| {
            Error::Compilation(format!("Failed to acquire read lock on extensions: {}", e))
        })?;
        
        Ok(extensions.get(interface).cloned())
    }
    
    #[instrument(level = "debug")]
    fn get_direct_implementors(&self, interface: &str) -> Result<Option<Vec<String>>, Error> {
        debug!("Getting direct implementors of interface: {}", interface);
        
        let implementors = self.implementors.read().map_err(|e| {
            Error::Compilation(format!("Failed to acquire read lock on implementors: {}", e))
        })?;
        
        Ok(implementors.get(interface).cloned())
    }
    
    #[instrument(level = "debug")]
    fn extends(&self, source: &str, target: &str) -> Result<bool, Error> {
        debug!("Checking if {} extends {}", source, target);
        
        // If source and target are the same, return true immediately
        if source == target {
            return Ok(true);
        }
        
        // Use find_inheritance_path to determine if source extends target
        match self.find_inheritance_path(source, target) {
            Ok(_) => Ok(true),
            Err(Error::Compilation(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }
    
    #[instrument(level = "debug")]
    fn get_all_extensions(&self, interface: &str) -> Result<HashSet<String>, Error> {
        debug!("Getting all extensions of interface: {}", interface);
        
        let mut result = HashSet::new();
        let mut queue = std::collections::VecDeque::new();
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
        let mut queue = std::collections::VecDeque::new();
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
        let mut queue = std::collections::VecDeque::new();
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
            registry: &ThreadSafeInterfaceRegistryVisualization,
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
        
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut path = Vec::new();
        let mut on_stack = HashSet::new();
        
        // Get all interfaces
        let all_interfaces = self.get_all_interfaces()?;
        
        // Helper function for cycle detection using DFS
        fn dfs_cycle(
            registry: &ThreadSafeInterfaceRegistryVisualization,
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
            if let Some(extensions) = registry.get_direct_extensions(interface)? {
                for extension in extensions {
                    if !visited.contains(&extension) {
                        dfs_cycle(registry, &extension, visited, path, on_stack, cycles)?;
                    } else if on_stack.contains(&extension) {
                        // Found a cycle
                        let cycle_start = path.iter().position(|x| x == &extension).unwrap();
                        let cycle = path[cycle_start..].to_vec();
                        cycles.push(cycle);
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
                    self,
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
        
        let mut result = String::from("Interface Hierarchy:\n");
        
        // Get all interfaces
        let all_interfaces = self.get_all_interfaces()?;
        
        // Sort interfaces alphabetically for consistent output
        let mut sorted_interfaces: Vec<_> = all_interfaces.into_iter().collect();
        sorted_interfaces.sort();
        
        // Helper function to print the tree recursively
        fn print_tree(
            registry: &ThreadSafeInterfaceRegistryVisualization,
            interface: &str,
            prefix: &str,
            is_last: bool,
            result: &mut String,
            visited: &mut HashSet<String>,
        ) -> Result<(), Error> {
            // Avoid cycles
            if visited.contains(interface) {
                result.push_str(&format!("{}{} (cycle)\n", prefix, interface));
                return Ok(());
            }
            
            // Print current interface
            result.push_str(&format!("{}{} [{}]\n", prefix, if is_last { "└── " } else { "├── " }, interface));
            
            // Mark as visited to prevent cycles
            visited.insert(interface.to_string());
            
            // Get direct extensions
            if let Some(extensions) = registry.get_direct_extensions(interface)? {
                let mut sorted_extensions = extensions.clone();
                sorted_extensions.sort();
                
                for (i, extension) in sorted_extensions.iter().enumerate() {
                    let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
                    let is_last_child = i == sorted_extensions.len() - 1;
                    
                    print_tree(
                        registry,
                        extension,
                        &new_prefix,
                        is_last_child,
                        result,
                        visited,
                    )?;
                }
            }
            
            // Remove from visited to allow the same interface to appear in different branches
            visited.remove(interface);
            
            Ok(())
        }
        
        // Find root interfaces (those that aren't extended by any other interface)
        let mut roots = HashSet::new();
        
        for interface in &sorted_interfaces {
            if let Ok(Some(v)) = self.get_direct_implementors(interface) {
                if v.is_empty() {
                    roots.insert(interface.clone());
                }
            }
        }
        
        // If no roots found, use all interfaces as potential roots
        if roots.is_empty() {
            roots = sorted_interfaces.into_iter().collect();
        }
        
        // Sort roots for consistent output
        let mut sorted_roots: Vec<_> = roots.into_iter().collect();
        sorted_roots.sort();
        
        // Print the tree starting from each root
        for (i, root) in sorted_roots.iter().enumerate() {
            let is_last = i == sorted_roots.len() - 1;
            let mut visited = HashSet::new();
            
            print_tree(self, root, "", is_last, &mut result, &mut visited)?;
        }
        
        Ok(result)
    }
    
    #[instrument(level = "debug")]
    fn visualize_hierarchy_dot(&self) -> Result<String, Error> {
        debug!("Visualizing interface hierarchy as DOT graph");
        
        let mut dot = String::from("digraph interface_hierarchy {\n");
        dot.push_str("  node [shape=box, style=filled, fillcolor=lightblue];\n");
        
        // Get all interfaces
        let all_interfaces = self.get_all_interfaces()?;
        
        // Add nodes
        for interface in &all_interfaces {
            dot.push_str(&format!("  \"{}\" [label=\"{}\"];\n", interface, interface));
        }
        
        // Add edges
        for interface in &all_interfaces {
            if let Some(extensions) = self.get_direct_extensions(interface)? {
                for extension in extensions {
                    dot.push_str(&format!("  \"{}\" -> \"{}\";\n", interface, extension));
                }
            }
        }
        
        dot.push_str("}\n");
        
        Ok(dot)
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
        
        let extensions = self.extensions.read().map_err(|e| {
            Error::Compilation(format!("Failed to acquire read lock on extensions: {}", e))
        })?;
        
        let implementors = self.implementors.read().map_err(|e| {
            Error::Compilation(format!("Failed to acquire read lock on implementors: {}", e))
        })?;
        
        // Check if the interface is a key in extensions or implementors
        Ok(extensions.contains_key(interface) || 
           implementors.contains_key(interface) ||
           extensions.values().any(|v| v.contains(&interface.to_string())) ||
           implementors.values().any(|v| v.contains(&interface.to_string())))
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
            dot.push_str(&format!("  \"{}\" [label=\"{}\"];\n", interface, interface));
        }
        
        // Add edges
        for (source, targets) in hierarchy {
            for target in targets {
                dot.push_str(&format!("  \"{}\" -> \"{}\";\n", source, target));
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
        
        let initialized = self.initialized.read().map_err(|e| {
            Error::Compilation(format!("Failed to acquire read lock on initialized flag: {}", e))
        })?;
        
        Ok(*initialized)
    }
    
    #[instrument(level = "debug")]
    fn set_visualization_initialized(&self, initialized: bool) -> Result<(), Error> {
        debug!("Setting visualization system initialized to: {}", initialized);
        
        let mut init_guard = self.initialized.write().map_err(|e| {
            Error::Compilation(format!("Failed to acquire write lock on initialized flag: {}", e))
        })?;
        
        *init_guard = initialized;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_visualize_hierarchy_ascii() {
        let registry = ThreadSafeInterfaceRegistryVisualization::new();
        
        // Set up a simple hierarchy
        registry.register_extension("A", "B").unwrap();
        registry.register_extension("A", "C").unwrap();
        registry.register_extension("B", "D").unwrap();
        registry.register_extension("C", "E").unwrap();
        
        // Visualize the hierarchy
        let ascii = registry.visualize_hierarchy_ascii().unwrap();
        
        // Check that the visualization contains all interfaces
        assert!(ascii.contains("A"));
        assert!(ascii.contains("B"));
        assert!(ascii.contains("C"));
        assert!(ascii.contains("D"));
        assert!(ascii.contains("E"));
    }
    
    #[test]
    fn test_find_inheritance_path() {
        let registry = ThreadSafeInterfaceRegistryVisualization::new();
        
        // Set up a simple hierarchy
        registry.register_extension("A", "B").unwrap();
        registry.register_extension("B", "C").unwrap();
        registry.register_extension("C", "D").unwrap();
        
        // Find a path
        let path = registry.find_inheritance_path("A", "D").unwrap();
        
        // Check the path
        assert_eq!(path, vec!["A", "B", "C", "D"]);
        
        // Check a non-existent path
        let result = registry.find_inheritance_path("A", "X");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_detect_cycles() {
        let registry = ThreadSafeInterfaceRegistryVisualization::new();
        
        // Set up a hierarchy with a cycle
        registry.register_extension("A", "B").unwrap();
        registry.register_extension("B", "C").unwrap();
        registry.register_extension("C", "A").unwrap();
        
        // Detect cycles
        let cycles = registry.detect_cycles().unwrap();
        
        // Check that we found at least one cycle
        assert!(!cycles.is_empty());
        
        // Check that the cycle contains all three interfaces
        let mut has_cycle = false;
        for cycle in &cycles {
            if cycle.len() == 3 && cycle.contains(&"A".to_string()) && cycle.contains(&"B".to_string()) && cycle.contains(&"C".to_string()) {
                has_cycle = true;
                break;
            }
        }
        
        assert!(has_cycle);
    }
}

// Register this module in the compiler's initialization
pub fn register_interface_registry_visualization() {
    trace!("Interface registry visualization module registered");
}