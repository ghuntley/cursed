//! # Interface Type Assertion Path Visualization
//!
//! This module provides visualization capabilities for interface type assertion paths,
//! helping to understand and debug type assertions by showing the inheritance paths
//! between types and generating visual representations of type relationships.
//!
//! Key features:
//! - Visualize inheritance paths between types
//! - Find alternative paths between types when a direct assertion fails
//! - Generate structured visualizations of type hierarchies
//! - Support for debugging complex type assertion failures

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use tracing::{debug, info, instrument, trace, warn};

use crate::error::Error;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::interface_registry_integration::InterfaceRegistryIntegration;

/// Path visualization data for interface inheritance
pub struct InterfacePathData {
    pub source_type: String,
    pub target_type: String,
    pub path: Vec<String>,
    pub direct_path_exists: bool,
    pub alternative_paths: Vec<Vec<String>>,
}

impl fmt::Display for InterfacePathData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Path from '{}' to '{}':\n", self.source_type, self.target_type)?;
        
        if self.direct_path_exists {
            write!(f, "Direct path: ")?;
            for (i, node) in self.path.iter().enumerate() {
                if i > 0 {
                    write!(f, " -> ")?;
                }
                write!(f, "{}", node)?;
            }
            writeln!(f)?;
        } else {
            writeln!(f, "No direct path exists.")?;
        }
        
        if !self.alternative_paths.is_empty() {
            writeln!(f, "\nAlternative paths:")?;
            for (i, path) in self.alternative_paths.iter().enumerate() {
                write!(f, "  Path {}: ", i + 1)?;
                for (j, node) in path.iter().enumerate() {
                    if j > 0 {
                        write!(f, " -> ")?;
                    }
                    write!(f, "{}", node)?;
                }
                writeln!(f)?;
            }
        }
        
        Ok(())
    }
}

/// Trait for visualizing interface type assertion paths
pub trait InterfaceTypeAssertionPathVisualization {
    /// Visualize the inheritance path from one type to another
    fn visualize_interface_path(
        &mut self,
        target_type: &str,
        max_depth: usize
    ) -> Result<String, Error>;
    
    /// Find alternative paths between two types
    fn find_alternative_paths(
        &mut self,
        source_type: &str,
        target_type: &str,
        max_paths: usize
    ) -> Result<Vec<String>, Error>;
    
    /// Check if one type extends another type through inheritance
    fn check_extension_relationship_simple(
        &mut self,
        source_type: &str,
        target_type: &str
    ) -> Result<bool, Error>;
    
    /// Find the shortest path between two types
    fn find_shortest_path(
        &mut self,
        source_type: &str,
        target_type: &str
    ) -> Result<Option<Vec<String>>, Error>;
    
    /// Generate a textual tree visualization of the inheritance hierarchy
    fn generate_inheritance_tree(
        &mut self,
        root_type: &str,
        max_depth: usize
    ) -> Result<String, Error>;
    
    /// Get all types that implement a specific interface
    fn get_implementors(
        &mut self,
        interface_type: &str
    ) -> Result<Vec<String>, Error>;
    
    /// Helper to get runtime type ID
    fn get_runtime_type_id(&mut self,
        value: inkwell::values::BasicValueEnum<'_>
    ) -> Result<u64, Error>;
    
    /// Helper to get type name for a type ID
    fn get_type_name_for_id(
        &mut self,
        type_id: u64
    ) -> Result<String, Error>;
    
    /// Extract interface type ID from a value
    fn extract_interface_type_id(
        &mut self,
        value: inkwell::values::BasicValueEnum<'_>
    ) -> Result<inkwell::values::BasicValueEnum<'_>, Error>;
}

impl<'ctx> InterfaceTypeAssertionPathVisualization for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn visualize_interface_path(
        &mut self,
        target_type: &str,
        max_depth: usize
    ) -> Result<String, Error> {
        // Ensure registry is initialized
        self.ensure_registry_initialized()?;
        
        if let Some(registry) = &self.interface_type_registry {
            // Generate hierarchy visualization
            let mut result = format!("Interface hierarchy for '{}':\n", target_type);
            
            // Get all direct interfaces that the target type implements
            let interfaces = registry.get_implemented_interfaces(target_type)
                .map_err(|e| Error::Compilation(format!("Failed to get interfaces: {}", e)))?;
            
            if interfaces.is_empty() {
                result.push_str("  No interfaces implemented directly.\n");
            } else {
                result.push_str("  Directly implements:\n");
                for interface in &interfaces {
                    result.push_str(&format!("    - {}\n", interface));
                }
            }
            
            // For each interface, visualize deeper relationships if depth permits
            if max_depth > 1 {
                for interface in &interfaces {
                    let sub_visualization = self.visualize_interface_hierarchy(interface, max_depth - 1, 4)?
                        .lines().map(|line| format!("  {}", line)).collect::<Vec<_>>().join("\n");
                    
                    if !sub_visualization.is_empty() {
                        result.push_str("\n");
                        result.push_str(&sub_visualization);
                    }
                }
            }
            
            Ok(result)
        } else {
            Err(Error::Compilation("Interface type registry not initialized".to_string()))
        }
    }
    
    #[instrument(skip(self), level = "debug")]
    fn find_alternative_paths(
        &mut self,
        source_type: &str,
        target_type: &str,
        max_paths: usize
    ) -> Result<Vec<String>, Error> {
        // Ensure registry is initialized
        self.ensure_registry_initialized()?;
        
        if let Some(registry) = &self.interface_type_registry {
            let mut paths = Vec::new();
            
            // Check if a direct path exists
            if let Ok(Some(path)) = self.find_shortest_path(source_type, target_type) {
                paths.push(path.join(" -> "));
                if paths.len() >= max_paths {
                    return Ok(paths);
                }
            }
            
            // Check for common interfaces that both types implement
            let source_interfaces = registry.get_implemented_interfaces(source_type)
                .map_err(|e| Error::Compilation(format!("Failed to get interfaces for source: {}", e)))?;
                
            let target_interfaces = registry.get_implemented_interfaces(target_type)
                .map_err(|e| Error::Compilation(format!("Failed to get interfaces for target: {}", e)))?;
            
            // Find common interfaces
            for source_interface in &source_interfaces {
                for target_interface in &target_interfaces {
                    if source_interface == target_interface {
                        // Both types implement the same interface
                        let path = format!("{} -> {} -> {}", source_type, source_interface, target_type);
                        if !paths.contains(&path) {
                            paths.push(path);
                            if paths.len() >= max_paths {
                                return Ok(paths);
                            }
                        }
                    }
                }
            }
            
            // Try to find indirect paths through multiple interfaces
            for source_interface in &source_interfaces {
                for target_interface in &target_interfaces {
                    if let Ok(Some(intermediate_path)) = self.find_shortest_path(source_interface, target_interface) {
                        let mut path = vec![source_type.to_string()];
                        path.extend(intermediate_path);
                        path.push(target_type.to_string());
                        
                        let path_str = path.join(" -> ");
                        if !paths.contains(&path_str) {
                            paths.push(path_str);
                            if paths.len() >= max_paths {
                                return Ok(paths);
                            }
                        }
                    }
                }
            }
            
            Ok(paths)
        } else {
            Err(Error::Compilation("Interface type registry not initialized".to_string()))
        }
    }
    
    #[instrument(skip(self), level = "debug")]
    fn check_extension_relationship_simple(
        &mut self,
        source_type: &str,
        target_type: &str
    ) -> Result<bool, Error> {
        // Ensure registry is initialized
        self.ensure_registry_initialized()?;
        
        if let Some(registry) = &self.interface_type_registry {
            // Check if source directly implements target
            let interfaces = registry.get_implemented_interfaces(source_type)
                .map_err(|e| Error::Compilation(format!("Failed to get interfaces: {}", e)))?;
            
            if interfaces.contains(&target_type.to_string()) {
                return Ok(true);
            }
            
            // Check if source indirectly implements target
            for interface in interfaces {
                if self.check_extension_relationship_simple(&interface, target_type)? {
                    return Ok(true);
                }
            }
            
            Ok(false)
        } else {
            Err(Error::Compilation("Interface type registry not initialized".to_string()))
        }
    }
    
    #[instrument(skip(self), level = "debug")]
    fn find_shortest_path(
        &mut self,
        source_type: &str,
        target_type: &str
    ) -> Result<Option<Vec<String>>, Error> {
        // Ensure registry is initialized
        self.ensure_registry_initialized()?;
        
        if let Some(registry) = &self.interface_type_registry {
            // Use breadth-first search to find the shortest path
            let mut queue = VecDeque::new();
            let mut visited = HashSet::new();
            let mut parent_map = HashMap::new();
            
            // Initialize with the source type
            queue.push_back(source_type.to_string());
            visited.insert(source_type.to_string());
            
            // BFS to find the target type
            while let Some(current) = queue.pop_front() {
                if current == target_type {
                    // Found the target, reconstruct the path
                    let mut path = Vec::new();
                    let mut current_node = current;
                    
                    while current_node != source_type {
                        path.push(current_node.clone());
                        current_node = parent_map.get(&current_node)
                            .ok_or_else(|| Error::Compilation("Invalid path reconstruction".to_string()))?
                            .clone();
                    }
                    
                    path.push(source_type.to_string());
                    path.reverse();
                    return Ok(Some(path));
                }
                
                // Get all interfaces that the current type implements
                let interfaces = registry.get_implemented_interfaces(&current)
                    .map_err(|e| Error::Compilation(format!("Failed to get interfaces: {}", e)))?;
                
                for interface in interfaces {
                    if !visited.contains(&interface) {
                        visited.insert(interface.clone());
                        queue.push_back(interface.clone());
                        parent_map.insert(interface, current.clone());
                    }
                }
            }
            
            // No path found
            Ok(None)
        } else {
            Err(Error::Compilation("Interface type registry not initialized".to_string()))
        }
    }
    
    #[instrument(skip(self), level = "debug")]
    fn generate_inheritance_tree(
        &mut self,
        root_type: &str,
        max_depth: usize
    ) -> Result<String, Error> {
        // Ensure registry is initialized
        self.ensure_registry_initialized()?;
        
        if let Some(registry) = &self.interface_type_registry {
            let mut result = String::new();
            self.generate_inheritance_tree_recursive(root_type, &mut result, "", max_depth, 0)?;
            Ok(result)
        } else {
            Err(Error::Compilation("Interface type registry not initialized".to_string()))
        }
    }
    
    #[instrument(skip(self), level = "debug")]
    fn get_implementors(
        &mut self,
        interface_type: &str
    ) -> Result<Vec<String>, Error> {
        // Ensure registry is initialized
        self.ensure_registry_initialized()?;
        
        if let Some(registry) = &self.interface_type_registry {
            // Get all types that implement this interface
            let implementors = registry.get_interface_implementors(interface_type)
                .map_err(|e| Error::Compilation(format!("Failed to get implementors: {}", e)))?;
            
            Ok(implementors)
        } else {
            Err(Error::Compilation("Interface type registry not initialized".to_string()))
        }
    }
    
    fn get_runtime_type_id(
        &mut self,
        value: inkwell::values::BasicValueEnum<'_>
    ) -> Result<u64, Error> {
        // Get type ID from interface value (stub implementation)
        let type_id = 0u64;  // Actual implementation would extract this from the value
        Ok(type_id)
    }
    
    fn get_type_name_for_id(
        &mut self,
        type_id: u64
    ) -> Result<String, Error> {
        // Ensure registry is initialized
        self.ensure_registry_initialized()?;
        
        if let Some(registry) = &self.interface_type_registry {
            // Get type name from ID
            let type_name = registry.get_type_name_for_id(type_id)
                .map_err(|e| Error::Compilation(format!("Failed to get type name: {}", e)))?;
            
            Ok(type_name)
        } else {
            Err(Error::Compilation("Interface type registry not initialized".to_string()))
        }
    }
    
    fn extract_interface_type_id(
        &mut self,
        value: inkwell::values::BasicValueEnum<'_>
    ) -> Result<inkwell::values::BasicValueEnum<'_>, Error> {
        // Extract type ID from interface value
        self.get_interface_type_id(value)
    }
}

// Helper methods
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Helper function for recursively building the inheritance tree visualization
    fn visualize_interface_hierarchy(
        &mut self,
        interface_type: &str,
        max_depth: usize,
        indent: usize
    ) -> Result<String, Error> {
        if max_depth == 0 {
            return Ok(String::new());
        }
        
        let indent_str = " ".repeat(indent);
        let mut result = format!("{} Interface {}:\n", indent_str, interface_type);
        
        if let Some(registry) = &self.interface_type_registry {
            // Get all interfaces that this interface extends
            let extended_interfaces = registry.get_extended_interfaces(interface_type)
                .map_err(|e| Error::Compilation(format!("Failed to get extended interfaces: {}", e)))?;
            
            if extended_interfaces.is_empty() {
                result.push_str(&format!("{} Extends no other interfaces\n", indent_str));
            } else {
                result.push_str(&format!("{} Extends:\n", indent_str));
                for extended in &extended_interfaces {
                    result.push_str(&format!("{} - {}\n", indent_str, extended));
                }
            }
            
            // Recurse for each extended interface if depth permits
            if max_depth > 1 {
                for extended in &extended_interfaces {
                    let sub_viz = self.visualize_interface_hierarchy(extended, max_depth - 1, indent + 2)?;
                    if !sub_viz.is_empty() {
                        result.push_str("\n");
                        result.push_str(&sub_viz);
                    }
                }
            }
        }
        
        Ok(result)
    }
    
    /// Recursively generate a tree visualization of the inheritance hierarchy
    fn generate_inheritance_tree_recursive(
        &mut self,
        type_name: &str,
        result: &mut String,
        prefix: &str,
        max_depth: usize,
        current_depth: usize
    ) -> Result<(), Error> {
        if current_depth >= max_depth {
            return Ok(());
        }
        
        // Add this type to the result
        result.push_str(&format!("{}{} {}\n", prefix, if prefix.is_empty() { "" } else { "└── " }, type_name));
        
        if let Some(registry) = &self.interface_type_registry {
            // Get all interfaces that this type implements
            let interfaces = registry.get_implemented_interfaces(type_name)
                .map_err(|e| Error::Compilation(format!("Failed to get interfaces: {}", e)))?;
            
            // Generate new prefix for children
            let new_prefix = if prefix.is_empty() {
                "    ".to_string()
            } else {
                format!("{}{}", prefix, "    ")
            };
            
            // Recursively add all interfaces
            for (i, interface) in interfaces.iter().enumerate() {
                let is_last = i == interfaces.len() - 1;
                let child_prefix = if is_last {
                    format!("{}", new_prefix)
                } else {
                    format!("{}{}", new_prefix, "│   ")
                };
                
                self.generate_inheritance_tree_recursive(
                    interface,
                    result,
                    &child_prefix,
                    max_depth,
                    current_depth + 1
                )?;
            }
        }
        
        Ok(())
    }
}

/// Register type assertion path visualization
pub fn register_type_assertion_path_visualization() {
    debug!("Type assertion path visualization module registered");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_path_visualization_registration() {
        register_type_assertion_path_visualization();
        assert!(true);
    }
}