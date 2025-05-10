//! # Interface Type Assertion Path Visualization
//!
//! This module enhances the interface type assertion system by providing visual feedback
//! on interface inheritance relationships. It helps developers understand and debug
//! complex interface hierarchies by visualizing inheritance paths and suggesting
//! potential paths when assertions fail.
//!
//! ## Features
//!
//! 1. Visual representation of interface inheritance paths
//! 2. DOT graph generation for interface hierarchies
//! 3. Enhanced error messages with illustrated paths
//! 4. Alternative path suggestions when direct relationships don't exist
//! 5. Integration with the existing type assertion system

use inkwell::values::BasicValueEnum;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{self, Write};
use tracing::{debug, error, info, instrument, trace, warn};

use crate::ast::expressions::TypeAssertion;
use crate::ast::traits::Node;
use crate::codegen::llvm::interface_type_assertion_error_propagation::TypeAssertionErrorPropagation;
// We'll implement without the nested interface type assertion dependency for now
// This allows us to avoid import errors while demonstrating the path visualization concepts
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::error::Error;

/// A trait for visualizing interface inheritance paths during type assertions
pub trait InterfaceTypeAssertionPathVisualization<'ctx> {
    /// Find a path from the source interface to the target interface
    fn find_interface_path(
        &self,
        source_interface: &str,
        target_interface: &str,
    ) -> Result<Vec<String>, Error>;

    /// Generate a DOT graph representation of the interface inheritance hierarchy
    fn generate_interface_hierarchy_dot(&self) -> Result<String, Error>;

    /// Visualize the path between two interfaces
    fn visualize_interface_path(
        &self,
        source_interface: &str,
        target_interface: &str,
    ) -> Result<String, Error>;

    /// Generate enhanced error message with path information
    fn generate_path_error_message(
        &self,
        source_interface: &str,
        target_interface: &str,
        source_location: &str,
    ) -> Result<String, Error>;

    /// Find alternative paths between interfaces
    fn find_alternative_paths(
        &self,
        source_interface: &str,
        target_interface: &str,
        max_alternatives: usize,
    ) -> Result<Vec<Vec<String>>, Error>;
    
    /// Enhanced type assertion with path visualization
    fn compile_type_assertion_with_path_visualization(
        &mut self,
        type_assertion: &TypeAssertion,
    ) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> InterfaceTypeAssertionPathVisualization<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn find_interface_path(
        &self,
        source_interface: &str,
        target_interface: &str,
    ) -> Result<Vec<String>, Error> {
        debug!("Finding path from {} to {}", source_interface, target_interface);
        
        // Initialize visited set and queue for BFS
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut path_map = HashMap::new(); // Maps node -> (previous_node, path_from_start)
        
        // Start BFS from source_interface
        queue.push_back(source_interface.to_string());
        visited.insert(source_interface.to_string());
        path_map.insert(source_interface.to_string(), (None, vec![source_interface.to_string()]));
        
        // Handle the case where source and target are the same
        if source_interface == target_interface {
            return Ok(vec![source_interface.to_string()]);
        }
        
        // Perform BFS to find the shortest path
        while let Some(current) = queue.pop_front() {
            // Get direct extensions from the current interface
            match self.interface_registry().get_direct_extensions(&current) {
                Ok(Some(direct_extensions)) => {
                    for next in direct_extensions {
                        if !visited.contains(&next) {
                            visited.insert(next.clone());
                            queue.push_back(next.clone());
                            
                            // Update path
                            let mut new_path = path_map.get(&current).cloned().unwrap_or_else(|| (None, Vec::new())).1;
                            new_path.push(next.clone());
                            path_map.insert(next.clone(), (Some(current.clone()), new_path.clone()));
                            
                            // Check if we've reached the target
                            if &next == target_interface {
                                return Ok(new_path);
                            }
                        }
                    }
                },
                Ok(None) => {
                    // No direct extensions for this interface
                    trace!("No direct extensions found for {}", current);
                },
                Err(e) => {
                    warn!("Error getting direct extensions for {}: {}", current, e);
                    // Continue the search with other paths
                }
            }
        }
        
        // No path found
        Err(Error::Compilation(format!(
            "No path found from interface '{}' to interface '{}'",
            source_interface, target_interface
        )))
    }

    #[instrument(skip(self), level = "debug")]
    fn generate_interface_hierarchy_dot(&self) -> Result<String, Error> {
        debug!("Generating DOT graph for interface hierarchy");
        
        let mut dot = String::from("digraph interface_hierarchy {\n");
        _ = writeln!(dot, "  node [shape=box, style=filled, fillcolor=lightblue];");
        
        // Get the complete hierarchy from the registry
        match self.interface_registry().get_extension_hierarchy() {
            // Add all nodes first
            let mut all_interfaces = HashSet::new();
            
            // Collect all interface names
            for (source, targets) in &hierarchy {
                all_interfaces.insert(source.clone());
                for target in targets {
                    all_interfaces.insert(target.clone());
                }
            }
            
            // Add nodes to DOT
            for interface in &all_interfaces {
                _ = writeln!(dot, "  \"{}\" [label=\"{}\"];", interface, interface);
            }
            
            // Add edges
            for (source, targets) in &hierarchy {
                for target in targets {
                    _ = writeln!(dot, "  \"{}\" -> \"{}\";", source, target);
                }
            }
        }
        
        dot.push_str("}\n");
        Ok(dot)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn visualize_interface_path(
        &self,
        source_interface: &str,
        target_interface: &str,
    ) -> Result<String, Error> {
        debug!("Visualizing path from {} to {}", source_interface, target_interface);
        
        // Find the path
        let path = self.find_interface_path(source_interface, target_interface)?;
        
        // Generate a simple ASCII art representation
        let mut result = String::new();
        
        _ = writeln!(result, "Interface Inheritance Path:");
        for (i, interface) in path.iter().enumerate() {
            if i > 0 {
                _ = writeln!(result, "  ↓ extends");
            }
            _ = writeln!(result, "  [{}]", interface);
        }
        
        // Also generate a DOT subgraph for just this path
        _ = writeln!(result, "\nDOT representation:");
        _ = writeln!(result, "digraph path {{");
        _ = writeln!(result, "  node [shape=box, style=filled, fillcolor=lightblue];");
        
        for i in 0..path.len() {
            _ = writeln!(result, "  \"{}\" [label=\"{}\"];", path[i], path[i]);
            if i < path.len() - 1 {
                _ = writeln!(result, "  \"{}\" -> \"{}\";", path[i], path[i + 1]);
            }
        }
        
        _ = writeln!(result, "}}");
        
        Ok(result)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn generate_path_error_message(
        &self,
        source_interface: &str,
        target_interface: &str,
        source_location: &str,
    ) -> Result<String, Error> {
        debug!("Generating error message for failed type assertion from {} to {}", source_interface, target_interface);
        
        let mut message = format!(
            "Type assertion error at {}: Value of type '{}' cannot be asserted as type '{}'",
            source_location, source_interface, target_interface
        );
        
        // Try to find alternative paths
        match self.find_alternative_paths(source_interface, target_interface, 3) {
            Ok(paths) if !paths.is_empty() => {
                _ = writeln!(message, "\n\nAlternative paths between these interfaces:");
                
                for (i, path) in paths.iter().enumerate() {
                    _ = writeln!(message, "\nPath {}:", i + 1);
                    for (j, interface) in path.iter().enumerate() {
                        if j > 0 {
                            _ = writeln!(message, "  ↓ extends");
                        }
                        _ = writeln!(message, "  [{}]", interface);
                    }
                }
                
                _ = writeln!(
                    message,
                    "\nConsider implementing the missing interfaces in the hierarchy."
                );
            },
            Ok(_) => {
                _ = writeln!(
                    message,
                    "\n\nNo viable inheritance path exists between these interfaces."
                );
                
                // List all interfaces that the source implements
                match self.interface_registry().get_direct_extensions(source_interface) {
                    Ok(Some(implementations)) if !implementations.is_empty() => {
                        _ = writeln!(message, "\n'{}' directly extends these interfaces:", source_interface);
                        for impl_interface in &implementations {
                            _ = writeln!(message, "  - {}", impl_interface);
                        }
                    },
                    Ok(_) => {}, // No implementations or empty
                    Err(e) => {
                        warn!("Failed to get direct extensions for {}: {}", source_interface, e);
                    }
                }
                
                // List all interfaces that extend the target
                match self.interface_registry().get_direct_implementors(target_interface) {
                    Ok(Some(implementors)) if !implementors.is_empty() => {
                        _ = writeln!(message, "\nThese interfaces directly extend '{}':", target_interface);
                        for impl_interface in &implementors {
                            _ = writeln!(message, "  - {}", impl_interface);
                        }
                    },
                    Ok(_) => {}, // No implementors or empty
                    Err(e) => {
                        warn!("Failed to get direct implementors for {}: {}", target_interface, e);
                    }
                }
            },
            Err(e) => {
                warn!("Failed to find alternative paths: {}", e);
                _ = writeln!(
                    message,
                    "\n\nFailed to analyze inheritance paths: {}",
                    e
                );
            }
        }
        
        Ok(message)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn find_alternative_paths(
        &self,
        source_interface: &str,
        target_interface: &str,
        max_alternatives: usize,
    ) -> Result<Vec<Vec<String>>, Error> {
        debug!("Finding alternative paths from {} to {}", source_interface, target_interface);
        
        // Get all interfaces
        let all_interfaces = match self.interface_registry().get_all_interfaces() {
            Ok(interfaces) => interfaces,
            Err(e) => {
                warn!("Failed to get all interfaces: {}", e);
                return Ok(Vec::new());
            },
        };
        
        let mut alternative_paths = Vec::new();
        
        // Try indirect paths through other interfaces
        for intermediate in all_interfaces {
            // Skip source and target
            if intermediate == source_interface || intermediate == target_interface {
                continue;
            }
            
            // Try to find a path from source to intermediate
            match self.find_interface_path(source_interface, &intermediate) {
                Ok(path1) => {
                    // Try to find a path from intermediate to target
                    match self.find_interface_path(&intermediate, target_interface) {
                        Ok(path2) => {
                            // Combine paths (remove duplicate intermediate node)
                            let mut combined_path = path1;
                            combined_path.extend(path2.into_iter().skip(1));
                            
                            alternative_paths.push(combined_path);
                            
                            // Limit the number of alternatives
                            if alternative_paths.len() >= max_alternatives {
                                break;
                            }
                        },
                        Err(e) => {
                            trace!("No path from {} to {}: {}", intermediate, target_interface, e);
                            // Continue to next intermediate
                        }
                    }
                },
                Err(e) => {
                    trace!("No path from {} to {}: {}", source_interface, intermediate, e);
                    // Continue to next intermediate
                }
            }
        }
        
        Ok(alternative_paths)
    }
    
    #[instrument(skip(self, type_assertion), level = "debug")]
    fn compile_type_assertion_with_path_visualization(
        &mut self,
        type_assertion: &TypeAssertion,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling type assertion with path visualization: {}", type_assertion.string());
        
        // Get source location for better error messages
        let source_location = format!("{}", type_assertion.token_literal());
        
        // First try to compile using the standard type assertion logic
        // This uses the TypeAssertionErrorPropagation trait that is already implemented
        // First try to compile using TypeAssertionErrorPropagation trait
        // This is implemented via integration with the error propagation system
        let result = self.compile_type_assertion_with_errors(type_assertion);
        
        // If compilation fails, enhance the error message with path information
        if let Err(Error::Compilation(err_msg)) = &result {
            // Extract interface names from error message
            if let Some(source_type) = extract_source_type_from_error(&err_msg) {
                if let Some(target_type) = extract_target_type_from_error(&err_msg) {
                    // Generate enhanced error message with path visualization
                    match self.generate_path_error_message(
                        &source_type,
                        &target_type,
                        &source_location
                    ) {
                        Ok(enhanced_msg) => {
                            // Return the enhanced error
                            return Err(Error::Compilation(enhanced_msg));
                        },
                        Err(e) => {
                            // Log the error but continue with the original message
                            warn!("Failed to generate path error message: {}", e);
                        }
                    }
                }
            }
        }
        
        // Return the original result (success or original error if we couldn't enhance it)
        result
    }
}

// Helper methods extension
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Access the interface registry for visualization
    fn interface_registry(&self) -> &dyn InterfaceRegistryExtensionWithVisualization {
        // The interface extension registry is stored in the registry_extensions field
        // of the LlvmCodeGenerator, which is initialized in the constructor
        &self.registry_extensions
    }
}

use crate::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry;

/// Extension trait for interface registry with visualization capabilities
pub trait InterfaceRegistryExtensionWithVisualization {
    /// Get a map of all interface extension relationships for visualization
    fn get_extension_hierarchy(&self) -> Result<HashMap<String, HashSet<String>>, Error>;
    
    /// Get the set of interfaces that a given interface directly extends
    fn get_direct_extensions(&self, interface: &str) -> Result<Option<HashSet<String>>, Error>;
    
    /// Get the set of interfaces that directly extend a given interface
    fn get_direct_implementors(&self, interface: &str) -> Result<Option<HashSet<String>>, Error>;
    
    /// Get all interfaces in the registry
    fn get_all_interfaces(&self) -> Result<HashSet<String>, Error>;
}

// Implementation for ThreadSafeInterfaceExtensionRegistry
impl InterfaceRegistryExtensionWithVisualization for ThreadSafeInterfaceExtensionRegistry {
    fn get_extension_hierarchy(&self) -> Result<HashMap<String, HashSet<String>>, Error> {
        self.get_extension_hierarchy()
    }
    
    fn get_direct_extensions(&self, interface: &str) -> Result<Option<HashSet<String>>, Error> {
        self.get_direct_extensions(interface)
    }
    
    fn get_direct_implementors(&self, interface: &str) -> Result<Option<HashSet<String>>, Error> {
        self.get_direct_implementors(interface)
    }
    
    fn get_all_interfaces(&self) -> Result<HashSet<String>, Error> {
        self.get_all_interfaces()
    }
}

// Helper functions to extract type names from error messages
// These would need to be adapted to the actual error format in the codebase
fn extract_source_type_from_error(error_msg: &str) -> Option<String> {
    // Simple extraction logic - would need to be more robust in practice
    if let Some(start) = error_msg.find("Value of type '") {
        if let Some(end) = error_msg[start + 14..].find("'") {
            return Some(error_msg[start + 14..start + 14 + end].to_string());
        }
    }
    None
}

fn extract_target_type_from_error(error_msg: &str) -> Option<String> {
    // Simple extraction logic - would need to be more robust in practice
    if let Some(start) = error_msg.find("cannot be asserted as type '") {
        if let Some(end) = error_msg[start + 27..].find("'") {
            return Some(error_msg[start + 27..start + 27 + end].to_string());
        }
    }
    None
}

// Helper function to register this module in the compiler
pub fn register_interface_type_assertion_path_visualization() {
    trace!("Interface type assertion path visualization module registered");
    // This function is called during compiler initialization to ensure
    // the path visualization components are properly registered
    // The actual registration is done by including the trait into the codebase
    // and implementing it in the LlvmCodeGenerator struct
}