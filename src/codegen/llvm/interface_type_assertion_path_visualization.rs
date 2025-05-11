//! Interface Type Assertion Path Visualization
//!
//! This module provides functionality for visualizing the inheritance path
//! between interface types during type assertions. It helps with debugging
//! and error reporting when type assertions fail, by showing the path that
//! would be required for a successful type assertion.
//!
//! The visualization shows both the current actual type and the target type,
//! as well as any intermediate types in the inheritance hierarchy that would
//! need to be traversed to make the assertion valid.

use crate::error::SourceLocation;
use crate::error::type_assertion_error::TypeAssertionError;
use inkwell::values::BasicValueEnum;
use inkwell::IntPredicate;
use std::collections::{HashMap, HashSet};

use crate::codegen::llvm::LlvmCodeGenerator;
use crate::error::Error;

/// Extension trait for LlvmCodeGenerator to add path visualization capabilities
pub trait InterfaceTypeAssertionPathVisualization<'ctx> {
    /// Get a visualization of the inheritance path between two types
    fn visualize_type_path(
        &self,
        from_type: &str,
        to_type: &str,
    ) -> Result<String, Error>;

    /// Get the runtime type ID from an interface value with enhanced error checking
    fn get_runtime_type_id(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        source_location: Option<SourceLocation>,
    ) -> Result<(u64, String), Error>;

    /// Check if a type assertion is valid and generate visualization on failure
    fn check_type_assertion_with_visualization(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type: &str,
        source_location: Option<SourceLocation>,
    ) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> InterfaceTypeAssertionPathVisualization<'ctx> for LlvmCodeGenerator<'ctx> {
    fn visualize_type_path(
        &self,
        from_type: &str,
        to_type: &str,
    ) -> Result<String, Error> {
        // Ensure the registry is initialized
        if self.interface_type_registry.is_none() {
            return Ok(format!(
                """
                Type Path Visualization:
                
                {} --?--> {}
                
                No type registry available to determine the path.
                """,
                from_type, to_type
            ));
        }

        let registry = self.interface_type_registry.as_ref().unwrap();
        
        // Try to get the type information from the registry
        let from_id = match registry.get_type_id(from_type) {
            Ok(id) => id,
            Err(_) => return Ok(format!(
                """
                Type Path Visualization:
                
                {} (unknown type) --?--> {}
                
                The source type is not registered in the type registry.
                """,
                from_type, to_type
            )),
        };
        
        let to_id = match registry.get_type_id(to_type) {
            Ok(id) => id,
            Err(_) => return Ok(format!(
                """
                Type Path Visualization:
                
                {} --?--> {} (unknown type)
                
                The target type is not registered in the type registry.
                """,
                from_type, to_type
            )),
        };
        
        // If the types are the same, it's a direct match
        if from_id == to_id {
            return Ok(format!(
                """
                Type Path Visualization:
                
                {} --DIRECT MATCH--> {}
                
                Types are identical, no conversion needed.
                """,
                from_type, to_type
            ));
        }
        
        // Get the inheritance map from the registry if available
        let inheritance_map = match registry.get_inheritance_map() {
            Some(map) => map,
            None => return Ok(format!(
                """
                Type Path Visualization:
                
                {} --?--> {}
                
                No inheritance information available in the registry.
                """,
                from_type, to_type
            )),
        };
        
        // Try to find a path from from_type to to_type
        let path = self.find_inheritance_path(from_id, to_id, &inheritance_map)?;
        
        // Build the visualization based on the path
        if path.is_empty() {
            Ok(format!(
                """
                Type Path Visualization:
                
                {} --INCOMPATIBLE--> {}
                
                No valid conversion path exists between these types.
                """,
                from_type, to_type
            ))
        } else {
            // Convert type IDs to names
            let mut path_names = Vec::new();
            for type_id in &path {
                let type_name = registry.get_type_name(*type_id)
                    .unwrap_or_else(|_| format!("unknown(0x{:x})", type_id));
                path_names.push(type_name);
            }
            
            // Create the visualization with the path
            let mut path_str = String::new();
            path_str.push_str(&format!("\nType Path Visualization:\n\n"));
            
            // Build path diagram
            if path_names.len() <= 2 {
                // Direct inheritance
                path_str.push_str(&format!("{} --DIRECT--> {}\n", from_type, to_type));
            } else {
                // Path with intermediate types
                path_str.push_str(&format!("{} \n", from_type));
                for i in 1..path_names.len() - 1 {
                    path_str.push_str(&format!("  |\n  v\n{} \n", path_names[i]));
                }
                path_str.push_str(&format!("  |\n  v\n{}\n", to_type));
            }
            
            // Add explanation
            path_str.push_str("\n");
            if path_names.len() <= 2 {
                path_str.push_str("Direct implementation relationship exists between these types.\n");
            } else {
                path_str.push_str(&format!("Conversion requires {} intermediate types.\n", path_names.len() - 2));
            }
            
            Ok(path_str)
        }
    }
    fn get_runtime_type_id(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        source_location: Option<SourceLocation>,
    ) -> Result<(u64, String), Error> {
        // First, extract the type ID using the base functionality
        let type_id = match self.get_interface_type_id(interface_value) {
            Ok(id) => id,
            Err(e) => return Err(Error::TypeAssertion(
                TypeAssertionError::new("interface", "any")
                    .with_message(format!("Failed to extract type ID from interface value: {}", e))
                    .with_location(source_location.unwrap_or_else(|| SourceLocation {
                        line: 0,
                        column: 0,
                        file: None,
                        source_line: String::new(),
                    }))
            )),
        };
        
        // Convert the type ID to a runtime constant if possible
        let const_id = match type_id.as_int_value().get_zero_extended_constant() {
            Some(id) => id,
            None => {
                // For dynamic IDs, create a debug print for the ID
                // NOTE: In a real implementation, you'd add LLVM code to print this value at runtime
                return Ok((0, "<dynamic-type-id>".to_string()));
            }
        };
        
        // Try to look up the type name in the registry
        let type_name = if let Some(registry) = &self.interface_type_registry {
            match registry.get_type_name(const_id) {
                Ok(name) => name,
                Err(_) => format!("unknown(0x{:x})", const_id),
            }
        } else {
            format!("unknown(0x{:x})", const_id)
        };
        
        Ok((const_id, type_name))
    }
    
    fn check_type_assertion_with_visualization(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type: &str,
        source_location: Option<SourceLocation>,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Get the runtime type information
        let (actual_type_id, actual_type_name) = self.get_runtime_type_id(interface_value, source_location.clone())?;
        
        // Get the target type ID
        let target_type_id = match &self.interface_type_registry {
            Some(registry) => registry.get_type_id(target_type).unwrap_or_else(|_| self.hash_type_name(target_type)),
            None => self.hash_type_name(target_type),
        };
        
        // Check if the types are compatible
        if self.is_type_compatible(actual_type_id, target_type_id) {
            // Create a constant true value for compatible types
            let true_value = self.context().bool_type().const_int(1, false);
            return Ok(true_value.into());
        }
        
        // Create the visualization
        let visualization = self.visualize_type_path(&actual_type_name, target_type)?;
        
        // Create a detailed error with the visualization
        let detailed_error = TypeAssertionError::new("interface", target_type)
            .with_message(format!("Type assertion failed: {} is not a {}", actual_type_name, target_type))
            .with_actual_type(actual_type_name, Some(actual_type_id))
            .with_target_type_id(target_type_id);
            
        if let Some(loc) = source_location {
            return Err(Error::TypeAssertion(
                detailed_error.with_location(loc).with_message(visualization)
            ));
        }
        
        // Create a constant false value for incompatible types
        let false_value = self.context().bool_type().const_int(0, false);
        Ok(false_value.into())
    }
}

// Helper methods for path visualization
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Find a path between two types in the inheritance hierarchy
    fn find_inheritance_path(
        &self,
        from_type_id: u64,
        to_type_id: u64,
        inheritance_map: &HashMap<u64, HashSet<u64>>,
    ) -> Result<Vec<u64>, Error> {
        let mut visited = HashSet::new();
        let mut queue = vec![(from_type_id, vec![from_type_id])];
        
        while let Some((current_id, current_path)) = queue.pop() {
            // If we reached the target type, return the path
            if current_id == to_type_id {
                return Ok(current_path);
            }
            
            // Mark the current type as visited
            visited.insert(current_id);
            
            // Add all direct implementations/extensions to the queue
            if let Some(implementations) = inheritance_map.get(&current_id) {
                for &impl_id in implementations {
                    if !visited.contains(&impl_id) {
                        let mut new_path = current_path.clone();
                        new_path.push(impl_id);
                        queue.push((impl_id, new_path));
                    }
                }
            }
        }
        
        // No path found
        Ok(vec![])
    }
    
    /// Check if one type is compatible with another (either same type or implements the interface)
    fn is_type_compatible(&self, actual_type_id: u64, target_type_id: u64) -> bool {
        // Same type is always compatible
        if actual_type_id == target_type_id {
            return true;
        }
        
        // Check inheritance relationship
        if let Some(registry) = &self.interface_type_registry {
            if let Some(inheritance_map) = registry.get_inheritance_map() {
                // Check if there's a path from actual to target type
                match self.find_inheritance_path(actual_type_id, target_type_id, &inheritance_map) {
                    Ok(path) => return !path.is_empty(),
                    Err(_) => return false,
                }
            }
        }
        
        false
    }
}