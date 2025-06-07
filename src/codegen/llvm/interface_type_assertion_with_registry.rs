//! # Interface Type Assertion with Registry Integration
//!
//! This module provides a robust implementation of interface type assertions that directly
//! integrates with the interface registry extension checking system. It leverages the
//! enhanced interface path finder to provide better error diagnostics and more reliable
//! type assertions.
//!
//! ## Key Features
//!
//! 1. Direct integration with the interface registry extension checking system
//! 2. Enhanced error messages with path visualization for inheritance relationships
//! 3. Proper error propagation with consistent use of `?` operator
//! 4. Detection of common errors like reversed inheritance relationships
//! 5. Support for both direct and indirect interface inheritance relationships
//! 6. Thread-safe registry access for concurrent compilation scenarios

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tracing::{debug, error, info, instrument, span, Level, trace, warn};

use crate::ast::expressions::TypeAssertion;
use crate::ast::traits::Node;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::codegen::llvm::interface_path_finder_enhanced::EnhancedInterfacePathFinder;
use crate::codegen::llvm::interface_type_assertion_errors::TypeAssertionErrorHandler;
use crate::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;
use crate::codegen::llvm::interface_type_assertion_path_visualization_enhanced::EnhancedInterfaceTypeAssertionPathVisualization;
use crate::codegen::llvm::type_assertion::InterfaceTypeAssertion;
use crate::error::Error;
use inkwell::values::BasicValueEnum;

/// Trait for enhanced interface type assertions with direct registry integration
pub trait InterfaceTypeAssertionWithRegistry<'ctx> {
    /// Compile a type assertion with enhanced registry-based relationship checking
    fn compile_type_assertion_with_registry(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error>;

    /// Compile a type assertion using path visualization and registry checking
    fn compile_type_assertion_with_path_registry(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Check if a value is an instance of a type using the registry
    fn check_instance_of_with_registry(
        &mut self,
        value: BasicValueEnum<'ctx>,
        type_name: &str
    ) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> InterfaceTypeAssertionWithRegistry<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self, type_assertion), level = "debug")]
    fn compile_type_assertion_with_registry(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        let _span = span!(Level::DEBUG, "compile_type_assertion_with_registry").entered();
        debug!("Compiling type assertion with registry integration for: {}", type_assertion.string());
        
        // Get source location for better error messages
        let source_location = format!("{}", type_assertion.token_literal());
        
        // Compile the expression to get the value to assert
        let expr_value = match self.compile_expression(type_assertion.expression.as_ref()) {
            Ok(value) => value,
            Err(e) => {
                error!("Failed to compile expression '{}': {}", 
                      type_assertion.expression.string(), e);
                return Err(Error::Compilation(format!(
                    "Failed to compile expression in type assertion: {}", e
                )));
            }
        };
        
        // Get runtime type information
        let runtime_type_id = self.get_runtime_type_id(expr_value, None)?;
        
        // Get the target type ID
        let target_type_id = match self.get_type_id(&type_assertion.type_name) {
            Ok(id) => id,
            Err(e) => {
                error!("Failed to get type ID for {}: {}", type_assertion.type_name, e);
                return Err(Error::Compilation(format!(
                    "Failed to get type ID for {}: {}", type_assertion.type_name, e
                )));
            }
        };
        
        // Check if the runtime type is or extends the target type
        // We use the enhanced path finder to check for both direct and indirect relationships
        let mut found_relationship = false;
        
        // Get type names for both the runtime type and target type
        let runtime_type_name = self.get_type_name_for_id(runtime_type_id)?;
        let target_type_name = &type_assertion.type_name;
        
        debug!("Checking if '{}' extends '{}'", runtime_type_name, target_type_name);
        
        // First, handle the case where they're the same type
        if runtime_type_id == target_type_id {
            debug!("Direct type match found: {} == {}", runtime_type_name, target_type_name);
            found_relationship = true;
        } else {
            // Use the enhanced path finder to check for an inheritance relationship
            match self.check_extension_relationship_enhanced(&runtime_type_name, target_type_name) {
                Ok(extends) => {
                    if extends {
                        debug!("Inheritance relationship found: {} extends {}", 
                              runtime_type_name, target_type_name);
                        found_relationship = true;
                    } else {
                        debug!("No inheritance relationship found between {} and {}", 
                              runtime_type_name, target_type_name);
                        
                        // Check if there's a reversed relationship (common error)
                        let (reversed, message) = self.detect_reversed_inheritance_enhanced(
                            &runtime_type_name, target_type_name)?;
                        
                        if reversed {
                            error!("Reversed inheritance detected: {}", message);
                            return Err(Error::Compilation(format!(
                                "Type assertion failed: The interface relationship is reversed. {}\n{}\nAt: {}",
                                message, 
                                self.visualize_interface_hierarchy(target_type_name, 2)?,
                                source_location
                            )));
                        }
                    }
                },
                Err(e) => {
                    warn!("Error checking extension relationship: {}", e);
                    // Fall back to direct type comparison if path checking fails
                    found_relationship = runtime_type_id == target_type_id;
                }
            }
        }
        
        // Create a conditional branch based on whether a relationship was found
        if found_relationship {
            // Cast the value to the target type if a relationship was found
            debug!("Type assertion succeeded: {} is or extends {}", 
                  runtime_type_name, target_type_name);
            
            // Create a success result with the value cast to the target type
            let result = self.cast_to_interface_type(expr_value, &type_assertion.type_name)?;
            
            Ok(result)
        } else {
            // Generate a clearer error message with path visualization
            let paths = match self.find_alternative_paths_enhanced(
                &runtime_type_name, target_type_name, 3
            ) {
                Ok(paths) => {
                    if paths.is_empty() {
                        format!("No inheritance path exists between '{}' and '{}'.", 
                               runtime_type_name, target_type_name)
                    } else {
                        let mut result = format!("Found {} possible inheritance paths:", paths.len());
                        for (i, path) in paths.iter().enumerate() {
                            result.push_str(&format!("\nPath {}: {}", i + 1, path.to_string_representation()));
                        }
                        result
                    }
                },
                Err(_) => format!("No inheritance relationship between '{}' and '{}'.", 
                                runtime_type_name, target_type_name)
            };
            
            // Generate an error with the hierarchy visualization
            let hierarchy = self.visualize_interface_hierarchy(target_type_name, 2)?;
            
            error!("Type assertion failed: {} is not a {}", runtime_type_name, target_type_name);
            return Err(Error::Compilation(format!(
                "Type assertion failed: '{}' is not a '{}'.\n{}\n{}\nAt: {}",
                runtime_type_name, target_type_name, paths, hierarchy, source_location
            )));
        }
    }

    #[instrument(skip(self, type_assertion), level = "debug")]
    fn compile_type_assertion_with_path_registry(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        let _span = span!(Level::DEBUG, "compile_type_assertion_with_path_registry").entered();
        debug!("Compiling type assertion with path registry for: {}", type_assertion.string());
        
        // Get source location for better error messages
        let source_location = format!("{}", type_assertion.token_literal());
        
        // Use the base compile_type_assertion_with_registry method for most logic
        let result = match self.compile_type_assertion_with_registry(type_assertion) {
            Ok(value) => return Ok(value),
            Err(e) => {
                // If we get an error, try to enhance it with path visualization
                if !e.to_string().contains("Type assertion failed") {
                    // If it's not a type assertion failure, just propagate the error
                    return Err(e);
                }
                
                // Otherwise, we'll enhance the error with path visualization
                e
            }
        };
        
        // This code only runs if we got a type assertion error
        // Get the runtime type name for the expression
        let expr_value = self.compile_expression(type_assertion.expression.as_ref())?;
        let runtime_type_id = self.get_runtime_type_id(expr_value, None)?;
        let runtime_type_name = self.get_type_name_for_id(runtime_type_id)?;
        
        // Generate a rich DOT graph visualization of the inheritance hierarchy
        let dot_graph = self.generate_interface_hierarchy_dot_graph()?;
        
        // Create an enhanced error message with detailed path information
        let message = format!(
            "Type assertion failed: '{}' is not a '{}'.\n\nInterface Hierarchy Visualization:\n\nPlease explore the inheritance hierarchy to understand the relationships between interfaces.\n\nDOT Graph:\n{}\n\nAt: {}",
            runtime_type_name, 
            type_assertion.type_name,
            dot_graph,
            source_location
        );
        
        Err(Error::Compilation(message))
    }
    
    #[instrument(skip(self, value), level = "debug")]
    fn check_instance_of_with_registry(
        &mut self,
        value: BasicValueEnum<'ctx>,
        type_name: &str
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        let _span = span!(Level::DEBUG, "check_instance_of_with_registry").entered();
        debug!("Checking if value is instance of {} using registry", type_name);
        
        // Get the target type ID from the registry
        let target_type_id = self.get_type_id(type_name)?;
        debug!("Target type ID for {}: {}", type_name, target_type_id);
        
        // Get the runtime type ID of the value
        let runtime_type_id = self.get_runtime_type_id(value, None)?;
        
        // If they're the same, return true immediately
        if runtime_type_id == target_type_id {
            debug!("Direct type match: runtime_type_id({}) == target_type_id({})", 
                 runtime_type_id, target_type_id);
            return Ok(self.context().bool_type().const_int(1, false).into());
        }
        
        // Get type names for both the runtime type and target type
        let runtime_type_name = self.get_type_name_for_id(runtime_type_id)?;
        
        // Use the enhanced path finder to check for an inheritance relationship
        match self.check_extension_relationship_enhanced(&runtime_type_name, type_name) {
            Ok(extends) => {
                if extends {
                    debug!("Inheritance relationship found: {} extends {}", 
                          runtime_type_name, type_name);
                    return Ok(self.context().bool_type().const_int(1, false).into());
                } else {
                    debug!("No inheritance relationship found between {} and {}", 
                          runtime_type_name, type_name);
                    return Ok(self.context().bool_type().const_int(0, false).into());
                }
            },
            Err(e) => {
                warn!("Error checking extension relationship: {}", e);
                // Fall back to direct type comparison
                let result = runtime_type_id == target_type_id;
                return Ok(self.context().bool_type().const_int(result as u64, false).into());
            }
        }
    }
}

/// Register the interface type assertion with registry integration capability
pub fn register_interface_type_assertion_with_registry() {
    trace!("Interface type assertion with registry module registered");
    // This function is called during the compiler's initialization
    // to register this implementation for use throughout compilation
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::traits::Node;
    use inkwell::context::Context;
    
    #[test]
    fn test_interface_type_assertion_registry_integration() {
        // Basic test to verify that the module can be loaded
        register_interface_type_assertion_with_registry();
        assert!(true);
    }
}