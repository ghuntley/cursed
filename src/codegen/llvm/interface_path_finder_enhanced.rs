//! # Enhanced Interface Path Finder
//!
//! This module provides an enhanced implementation of the interface path finder with
//! improved error handling, visualization, and integration with the interface registry.
//! It helps developers understand complex interface inheritance relationships and debug
//! type assertion errors with rich diagnostics.
//!
//! ## Key Features
//!
//! 1. Path finding between interfaces with comprehensive error handling
//! 2. Alternative path discovery for debugging inheritance relationships
//! 3. Cycle detection in interface hierarchies
//! 4. Reversed inheritance detection with helpful guidance
//! 5. Rich error messages with visual representation of paths
//! 6. Integration with the interface registry for relationship lookups
//! 7. Consistent error propagation with `?` operator throughout
//! 8. Support for DOT graph generation for interface hierarchies

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{self, Display, Formatter, Write};
use std::sync::Arc;
use tracing::{debug, error, info, instrument, trace, warn};

use crate::codegen::llvm::LlvmCodeGenerator;
use crate::core::interface_registry_visualization::InterfaceRegistryExtensionWithVisualization;
use crate::error::Error;

/// Represents a path between interfaces in the inheritance hierarchy.
/// This structure provides methods for visualizing and querying the path.
#[derive(Debug, Clone)]
pub struct InterfaceInheritancePath {
    /// The interfaces in the path, from source to target
    path: Vec<String>,
    /// The source interface (starting point)
    source: String,
    /// The target interface (destination)
    target: String,
}

impl InterfaceInheritancePath {
    /// Create a new path between interfaces
    pub fn new(path: Vec<String>, source: String, target: String) -> Self {
        Self {
            path,
            source,
            target,
        }
    }

    /// Get a reference to the path
    pub fn path(&self) -> &Vec<String> {
        &self.path
    }

    /// Get a reference to the source interface
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Get a reference to the target interface
    pub fn target(&self) -> &str {
        &self.target
    }

    /// Check if the path is empty
    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }

    /// Get the length of the path
    pub fn len(&self) -> usize {
        self.path.len()
    }

    /// Convert the path to a string representation
    pub fn to_string_representation(&self) -> String {
        if self.path.is_empty() {
            return format!("No path from '{}' to '{}'.", self.source, self.target);
        }

        self.path.join(" -> ")
    }

    /// Generate a visual representation of the path using Unicode box-drawing characters
    pub fn to_visual_representation(&self) -> String {
        if self.path.is_empty() {
            return format!("No inheritance path exists from '{}' to '{}'.", self.source, self.target);
        }

        let mut result = String::from("Interface Inheritance Path:\n");

        for (i, interface) in self.path.iter().enumerate() {
            // Add appropriate box-drawing characters based on position
            if i == 0 {
                // First element
                result.push_str(&format!("\u{250c}\u{2500}\u{2500} {}\n", interface));
            } else if i == self.path.len() - 1 {
                // Last element
                result.push_str(&format!("\u{2514}\u{2500}\u{2500} {}\n", interface));
            } else {
                // Middle elements
                result.push_str(&format!("\u{251c}\u{2500}\u{2500} {}\n", interface));
            }

            // Add connecting lines between elements
            if i < self.path.len() - 1 {
                result.push_str("\u{2502}\n");
            }
        }

        result
    }

    /// Generate a DOT graph representation of the path
    pub fn to_dot_representation(&self) -> String {
        if self.path.is_empty() {
            return String::from("digraph empty_path {}\n");
        }

        let mut dot = String::from("digraph path {\n");
        dot.push_str("  rankdir=BT;\n"); // Bottom to top direction
        dot.push_str("  node [shape=box, style=filled, fillcolor=lightblue];\n");

        // Add nodes
        for interface in &self.path {
            dot.push_str(&format!("  \"{}\" [label=\"{}\"];\n", interface, interface));
        }

        // Add edges
        for i in 0..self.path.len() - 1 {
            dot.push_str(&format!("  \"{}\" -> \"{}\";\n", self.path[i], self.path[i + 1]));
        }

        dot.push_str("}\n");

        dot
    }
}

impl Display for InterfaceInheritancePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n", self.to_string_representation())?;
        write!(f, "{}\n", self.to_visual_representation())
    }
}

/// Extension trait for LlvmCodeGenerator to add enhanced interface path finding capabilities
pub trait EnhancedInterfacePathFinder {
    /// Find a path between interfaces with enhanced error handling
    fn find_interface_path_enhanced(&self, source: &str, target: &str) -> Result<InterfaceInheritancePath, Error>;

    /// Find alternative paths between interfaces for debugging purposes
    fn find_alternative_paths_enhanced(
        &self,
        source: &str,
        target: &str,
        max_alternatives: usize,
    ) -> Result<Vec<InterfaceInheritancePath>, Error>;

    /// Check if one interface extends another with proper error handling
    fn check_extension_relationship_enhanced(&self, source: &str, target: &str) -> Result<bool, Error>;

    /// Detect if an inheritance relationship is reversed (common error in type assertions)
    fn detect_reversed_inheritance_enhanced(&self, source: &str, target: &str) -> Result<(bool, String), Error>;

    /// Generate a visualization of the inheritance hierarchy for a specific interface
    fn visualize_interface_hierarchy(&self, interface: &str, max_depth: usize) -> Result<String, Error>;

    /// Generate a DOT graph representation of the entire interface hierarchy
    fn generate_interface_hierarchy_dot_graph(&self) -> Result<String, Error>;
}

impl<'ctx> EnhancedInterfacePathFinder for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn find_interface_path_enhanced(&self, source: &str, target: &str) -> Result<InterfaceInheritancePath, Error> {
        debug!("Finding path from interface {} to {}", source, target);

        // Check if both interfaces exist in the registry
        if !self.interface_exists_in_registry(source)? {
            return Err(Error::Compilation(format!(
                "Interface '{}' does not exist in the registry",
                source
            )));
        }

        if !self.interface_exists_in_registry(target)? {
            return Err(Error::Compilation(format!(
                "Interface '{}' does not exist in the registry",
                target
            )));
        }

        // If source and target are the same, return a path with just the interface
        if source == target {
            return Ok(InterfaceInheritancePath::new(
                vec![source.to_string()],
                source.to_string(),
                target.to_string(),
            ));
        }

        // Use BFS to find the shortest path
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut parent_map: HashMap<String, String> = HashMap::new();

        // Start BFS from source
        queue.push_back(source.to_string());
        visited.insert(source.to_string());

        // Flag to handle the case where no path is found
        let mut path_found = false;

        // Perform BFS
        while let Some(current) = queue.pop_front() {
            // Check if we've reached the target
            if current == target {
                path_found = true;
                break;
            }

            // Get direct extensions of current interface
            if let Some(extensions) = self.get_direct_extensions_for_interface(&current)? {
                for extension in extensions {
                    if !visited.contains(&extension) {
                        visited.insert(extension.clone());
                        queue.push_back(extension.clone());
                        parent_map.insert(extension.clone(), current.clone());
                    }
                }
            }
        }

        // If no path was found, check if a reversed path exists to provide better error messages
        if !path_found {
            // Try to detect a reversed inheritance relationship
            let (reversed, _) = self.detect_reversed_inheritance_enhanced(source, target)?;

            if reversed {
                return Err(Error::Compilation(format!(
                    "No path found from interface '{}' to interface '{}'. Did you mean to assert as the other way around? '{}' actually extends '{}'.",
                    source, target, target, source
                )));
            }

            return Err(Error::Compilation(format!(
                "No path found from interface '{}' to interface '{}'",
                source, target
            )));
        }

        // Reconstruct the path
        let mut path = Vec::new();
        let mut curr = target.to_string();

        path.push(curr.clone());

        while let Some(parent) = parent_map.get(&curr) {
            path.push(parent.clone());
            curr = parent.clone();
        }

        // Reverse the path to get it in the correct order (source to target)
        path.reverse();

        Ok(InterfaceInheritancePath::new(
            path,
            source.to_string(),
            target.to_string(),
        ))
    }

    #[instrument(skip(self), level = "debug")]
    fn find_alternative_paths_enhanced(
        &self,
        source: &str,
        target: &str,
        max_alternatives: usize,
    ) -> Result<Vec<InterfaceInheritancePath>, Error> {
        debug!("Finding alternative paths from {} to {}", source, target);

        // Check if both interfaces exist in the registry
        if !self.interface_exists_in_registry(source)? {
            return Err(Error::Compilation(format!(
                "Interface '{}' does not exist in the registry",
                source
            )));
        }

        if !self.interface_exists_in_registry(target)? {
            return Err(Error::Compilation(format!(
                "Interface '{}' does not exist in the registry",
                target
            )));
        }

        // First try to find the direct path
        let direct_path = match self.find_interface_path_enhanced(source, target) {
            Ok(path) => Some(path),
            Err(_) => None,
        };

        let mut paths = Vec::new();

        // Add the direct path if it exists
        if let Some(path) = direct_path {
            paths.push(path);
        }

        // Get all interfaces to try as intermediate points
        let all_interfaces = self.get_all_interfaces_in_registry()?;

        // Try indirect paths through other interfaces
        for intermediate in all_interfaces {
            // Skip source and target
            if intermediate == source || intermediate == target {
                continue;
            }

            // Try to find a path from source to intermediate
            let path1 = match self.find_interface_path_enhanced(source, &intermediate) {
                Ok(path) => path,
                Err(_) => continue,
            };

            // Try to find a path from intermediate to target
            let path2 = match self.find_interface_path_enhanced(&intermediate, target) {
                Ok(path) => path,
                Err(_) => continue,
            };

            // Combine paths
            let mut combined_path = Vec::new();
            combined_path.extend(path1.path[..path1.path.len() - 1].iter().cloned());
            combined_path.extend(path2.path.iter().cloned());

            paths.push(InterfaceInheritancePath::new(
                combined_path,
                source.to_string(),
                target.to_string(),
            ));

            // Limit the number of alternatives
            if paths.len() >= max_alternatives {
                break;
            }
        }

        // If no paths were found, return an error
        if paths.is_empty() {
            // Check if there's a reversed relationship
            let (reversed, _) = self.detect_reversed_inheritance_enhanced(source, target)?;

            if reversed {
                return Err(Error::Compilation(format!(
                    "No alternative paths found from interface '{}' to interface '{}'. Did you mean to assert as the other way around? '{}' actually extends '{}'.",
                    source, target, target, source
                )));
            }

            return Err(Error::Compilation(format!(
                "No alternative paths found from interface '{}' to interface '{}'",
                source, target
            )));
        }

        Ok(paths)
    }

    #[instrument(skip(self), level = "debug")]
    fn check_extension_relationship_enhanced(&self, source: &str, target: &str) -> Result<bool, Error> {
        debug!("Checking if {} extends {}", source, target);

        // If source and target are the same, return true immediately
        if source == target {
            return Ok(true);
        }

        // Use find_interface_path_enhanced to determine if source extends target
        match self.find_interface_path_enhanced(source, target) {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.to_string().contains("No path found") {
                    Ok(false)
                } else {
                    Err(e)
                }
            }
        }
    }

    #[instrument(skip(self), level = "debug")]
    fn detect_reversed_inheritance_enhanced(&self, source: &str, target: &str) -> Result<(bool, String), Error> {
        debug!("Detecting reversed inheritance between {} and {}", source, target);

        // Check if both interfaces exist in the registry
        if !self.interface_exists_in_registry(source)? {
            return Err(Error::Compilation(format!(
                "Interface '{}' does not exist in the registry",
                source
            )));
        }

        if !self.interface_exists_in_registry(target)? {
            return Err(Error::Compilation(format!(
                "Interface '{}' does not exist in the registry",
                target
            )));
        }

        // Try to find a path from target to source (the reverse direction)
        match self.find_interface_path_enhanced(target, source) {
            Ok(path) => {
                // If a path exists, then the inheritance is reversed
                let message = format!(
                    "Reversed inheritance detected. '{}' does not extend '{}', but '{}' extends '{}'\n\nThe actual inheritance path is:\n{}",
                    source,
                    target,
                    target,
                    source,
                    path.to_visual_representation()
                );

                Ok((true, message))
            }
            Err(_) => {
                // If no path exists in either direction, then the interfaces are unrelated
                Ok((false, format!("Interfaces '{}' and '{}' are not related by inheritance", source, target)))
            }
        }
    }

    #[instrument(skip(self), level = "debug")]
    fn visualize_interface_hierarchy(&self, interface: &str, max_depth: usize) -> Result<String, Error> {
        debug!("Visualizing interface hierarchy for {} with max depth {}", interface, max_depth);

        // Check if the interface exists in the registry
        if !self.interface_exists_in_registry(interface)? {
            return Err(Error::Compilation(format!(
                "Interface '{}' does not exist in the registry",
                interface
            )));
        }

        let mut result = format!("Interface Hierarchy for '{}':\n", interface);

        // Helper function to recursively build the hierarchy visualization
        fn build_hierarchy(
            codegen: &LlvmCodeGenerator,
            interface: &str,
            depth: usize,
            max_depth: usize,
            prefix: &str,
            is_last: bool,
            result: &mut String,
            visited: &mut HashSet<String>,
        ) -> Result<(), Error> {
            // Avoid cycles
            if visited.contains(interface) {
                writeln!(result, "{}{} {} (cycle)", prefix, if is_last { "└── " } else { "├── " }, interface)
                    .map_err(|e| Error::Compilation(format!("Failed to write to hierarchy: {}", e)))?;
                return Ok(());
            }

            // Print current interface
            writeln!(
                result,
                "{}{}{}({})",
                prefix,
                if is_last { "└── " } else { "├── " },
                interface,
                depth
            )
            .map_err(|e| Error::Compilation(format!("Failed to write to hierarchy: {}", e)))?;

            // Stop recursion if max depth is reached
            if depth >= max_depth {
                return Ok(());
            }

            // Mark as visited to prevent cycles
            visited.insert(interface.to_string());

            // Get direct extensions
            let extensions = match codegen.get_direct_extensions_for_interface(interface)? {
                Some(ext) => ext,
                None => Vec::new(),
            };

            // Sort extensions for consistent output
            let mut sorted_extensions = extensions.clone();
            sorted_extensions.sort();

            for (i, extension) in sorted_extensions.iter().enumerate() {
                let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
                let is_last_child = i == sorted_extensions.len() - 1;

                build_hierarchy(
                    codegen,
                    extension,
                    depth + 1,
                    max_depth,
                    &new_prefix,
                    is_last_child,
                    result,
                    visited,
                )?;
            }

            // Remove from visited to allow the same interface to appear in different branches
            visited.remove(interface);

            Ok(())
        }

        // Start building the hierarchy
        let mut visited = HashSet::new();
        build_hierarchy(self, interface, 0, max_depth, "", true, &mut result, &mut visited)?;

        Ok(result)
    }

    #[instrument(skip(self), level = "debug")]
    fn generate_interface_hierarchy_dot_graph(&self) -> Result<String, Error> {
        debug!("Generating DOT graph for entire interface hierarchy");

        let mut dot = String::from("digraph interface_hierarchy {\n");
        dot.push_str("  rankdir=BT;\n"); // Bottom to top direction
        dot.push_str("  node [shape=box, style=filled, fillcolor=lightblue];\n");

        // Get all interfaces
        let all_interfaces = self.get_all_interfaces_in_registry()?;

        // Add nodes
        for interface in &all_interfaces {
            writeln!(dot, "  \"{}\" [label=\"{}\"];", interface, interface)
                .map_err(|e| Error::Compilation(format!("Failed to write to DOT graph: {}", e)))?;
        }

        // Add edges
        for interface in &all_interfaces {
            if let Some(extensions) = self.get_direct_extensions_for_interface(interface)? {
                for extension in extensions {
                    writeln!(dot, "  \"{}\" -> \"{}\";", interface, extension)
                        .map_err(|e| Error::Compilation(format!("Failed to write to DOT graph: {}", e)))?;
                }
            }
        }

        // Detect and highlight cycles
        let cycles = self.detect_cycles_in_inheritance_hierarchy()?;
        if !cycles.is_empty() {
            writeln!(dot, "\n  // Cycles").map_err(|e| {
                Error::Compilation(format!("Failed to write to DOT graph: {}", e))
            })?;

            for (i, cycle) in cycles.iter().enumerate() {
                writeln!(dot, "  subgraph cluster_cycle_{} {{\n    style=filled;\n    color=lightpink;\n    label=\"Cycle {}\";\n", i, i + 1)
                    .map_err(|e| Error::Compilation(format!("Failed to write to DOT graph: {}", e)))?;

                for interface in cycle {
                    writeln!(dot, "    \"{}\";", interface)
                        .map_err(|e| Error::Compilation(format!("Failed to write to DOT graph: {}", e)))?;
                }

                writeln!(dot, "  }}").map_err(|e| {
                    Error::Compilation(format!("Failed to write to DOT graph: {}", e))
                })?;
            }
        }

        writeln!(dot, "}}").map_err(|e| {
            Error::Compilation(format!("Failed to write to DOT graph: {}", e))
        })?;

        Ok(dot)
    }
}

// Helper methods for the LlvmCodeGenerator
impl<'ctx> LlvmCodeGenerator<'ctx> {
    // Check if an interface exists in the registry
    fn interface_exists_in_registry(&self, interface: &str) -> Result<bool, Error> {
        // First try using the interface registry visualization
        if let Some(registry) = &self.registry_visualization {
            return registry.interface_exists(interface);
        }

        // Fallback to the test inheritance map
        if let Some(inheritance_map) = &self.test_inheritance_map {
            // Check if it's a source in the map
            if inheritance_map.contains_key(interface) {
                return Ok(true);
            }

            // Check if it's a target in any entry
            for (_, extensions) in inheritance_map {
                if extensions.contains(interface) {
                    return Ok(true);
                }
            }
        }

        // As a last resort, check if it's registered in the type registry
        let type_id = self.get_type_id_by_name(interface);
        Ok(type_id.is_some())
    }

    // Get all interfaces in the registry
    fn get_all_interfaces_in_registry(&self) -> Result<HashSet<String>, Error> {
        // First try using the interface registry visualization
        if let Some(registry) = &self.registry_visualization {
            return registry.get_all_interfaces();
        }

        // Fallback to the test inheritance map
        let mut all_interfaces = HashSet::new();

        if let Some(inheritance_map) = &self.test_inheritance_map {
            // Add all source interfaces
            for source in inheritance_map.keys() {
                all_interfaces.insert(source.clone());
            }

            // Add all target interfaces
            for (_, extensions) in inheritance_map {
                for extension in extensions {
                    all_interfaces.insert(extension.clone());
                }
            }
        }

        Ok(all_interfaces)
    }

    // Get direct extensions of an interface
    fn get_direct_extensions_for_interface(&self, interface: &str) -> Result<Option<Vec<String>>, Error> {
        // First try using the interface registry visualization
        if let Some(registry) = &self.registry_visualization {
            return registry.get_direct_extensions(interface);
        }

        // Fallback to the test inheritance map
        if let Some(inheritance_map) = &self.test_inheritance_map {
            if let Some(extensions) = inheritance_map.get(interface) {
                return Ok(Some(extensions.iter().cloned().collect()));
            }
        }

        Ok(None)
    }

    // Detect cycles in the inheritance hierarchy
    fn detect_cycles_in_inheritance_hierarchy(&self) -> Result<Vec<Vec<String>>, Error> {
        // First try using the interface registry visualization
        if let Some(registry) = &self.registry_visualization {
            return registry.detect_cycles();
        }

        // Fallback implementation for test inheritance map
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut path = Vec::new();
        let mut on_stack = HashSet::new();

        // Get all interfaces
        let all_interfaces = self.get_all_interfaces_in_registry()?;

        // Helper function for cycle detection using DFS
        fn dfs_cycle(
            codegen: &LlvmCodeGenerator,
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
            if let Some(extensions) = codegen.get_direct_extensions_for_interface(interface)? {
                for extension in extensions {
                    if !visited.contains(&extension) {
                        dfs_cycle(codegen, &extension, visited, path, on_stack, cycles)?;
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
}

/// Register the enhanced interface path finder functionality with the compiler
pub fn register_enhanced_interface_path_finder() {
    trace!("Enhanced interface path finder module registered");
    // This function is called during the compiler's initialization
    // to register this enhanced implementation for use throughout compilation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interface_inheritance_path_basics() {
        // Test creating a path
        let path = InterfaceInheritancePath::new(
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            "A".to_string(),
            "C".to_string(),
        );

        assert_eq!(path.source(), "A");
        assert_eq!(path.target(), "C");
        assert_eq!(path.len(), 3);
        assert_eq!(path.is_empty(), false);
        assert_eq!(path.to_string_representation(), "A -> B -> C");

        // Test empty path
        let empty_path = InterfaceInheritancePath::new(
            vec![],
            "X".to_string(),
            "Y".to_string(),
        );

        assert_eq!(empty_path.source(), "X");
        assert_eq!(empty_path.target(), "Y");
        assert_eq!(empty_path.len(), 0);
        assert_eq!(empty_path.is_empty(), true);
        assert!(empty_path.to_string_representation().contains("No path"));
    }

    #[test]
    fn test_interface_inheritance_path_visualization() {
        let path = InterfaceInheritancePath::new(
            vec!["Child".to_string(), "Parent".to_string(), "GrandParent".to_string()],
            "Child".to_string(),
            "GrandParent".to_string(),
        );

        let visual = path.to_visual_representation();
        assert!(visual.contains("Interface Inheritance Path:"));
        assert!(visual.contains("Child"));
        assert!(visual.contains("Parent"));
        assert!(visual.contains("GrandParent"));

        // Box drawing characters should be present
        assert!(visual.contains("\u{250c}") || visual.contains("┌")); // First element
        assert!(visual.contains("\u{251c}") || visual.contains("├")); // Middle element
        assert!(visual.contains("\u{2514}") || visual.contains("└")); // Last element

        // Test DOT representation
        let dot = path.to_dot_representation();
        assert!(dot.contains("digraph path"));
        assert!(dot.contains("Child")); 
        assert!(dot.contains("Parent"));
        assert!(dot.contains("GrandParent"));
        assert!(dot.contains("Child" -> "Parent"));
        assert!(dot.contains("Parent" -> "GrandParent"));
    }
}