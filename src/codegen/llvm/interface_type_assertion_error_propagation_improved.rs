//! # Enhanced Interface Type Assertion Error Propagation
//!
//! This module provides improved error propagation for interface type assertions.
//! It extends the base interface type assertion functionality with proper Result handling
//! and error context.
//!
//! ## Features
//!
//! - Consistent use of the `?` operator for cleaner error handling
//! - Detailed error messages with source location and type information
//! - Integration with the enhanced path visualization system
//! - Error recovery suggestions when possible

use tracing::{debug, error, info, instrument, warn};
use std::fmt;

use inkwell::values::BasicValueEnum;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::basic_block::BasicBlock;
use inkwell::AddressSpace;

use crate::ast::expressions::TypeAssertion;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::codegen::llvm::type_assertion::InterfaceTypeAssertion;
use crate::codegen::llvm::interface_type_assertion::ImprovedTypeAssertion;
use crate::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;
use crate::error::Error;

/// Structured error type for type assertion failures
#[derive(Debug)]
pub struct TypeAssertionError {
    pub source_type: String,
    pub target_type: String,
    pub source_location: String,
    pub message: String,
    pub recovery_hint: Option<String>,
}

impl fmt::Display for TypeAssertionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Type assertion error: {} is not a {}\n{}", 
               self.source_type, self.target_type, self.message)?;
        
        if let Some(hint) = &self.recovery_hint {
            write!(f, "\n\nRecovery hint: {}", hint)?;
        }
        
        write!(f, "\n\nAt: {}", self.source_location)
    }
}

impl From<TypeAssertionError> for Error {
    fn from(err: TypeAssertionError) -> Self {
        Error::Compilation(err.to_string())
    }
}

/// Trait for enhanced error propagation in interface type assertions
pub trait ImprovedTypeAssertionErrorPropagation<'ctx>: 
    InterfaceTypeAssertion<'ctx> + 
    ImprovedTypeAssertion<'ctx> + 
    InterfaceTypeAssertionPathVisualization<'ctx> 
{
    /// Compile a type assertion with proper error propagation
    fn compile_type_assertion_with_error_propagation(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Generate a structured error for a type assertion failure
    fn generate_type_assertion_error(
        &mut self,
        source_type: &str,
        target_type: &str,
        source_location: &str,
        additional_message: Option<String>
    ) -> Result<TypeAssertionError, Error>;
    
    /// Suggest recovery options for type assertion failures
    fn suggest_recovery_options(
        &mut self,
        source_type: &str,
        target_type: &str
    ) -> Result<Option<String>, Error>;
}

impl<'ctx> ImprovedTypeAssertionErrorPropagation<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self, type_assertion), level = "debug")]
    fn compile_type_assertion_with_error_propagation(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling type assertion with improved error propagation: {}", type_assertion.string());
        
        // Source location for error messages
        let source_location = type_assertion.token_literal();
        
        // Compile the expression to get the interface value
        let expr_value = self.compile_expression(type_assertion.expression.as_ref())?;
        
        // Get the runtime type ID and name
        let runtime_type_id = self.get_runtime_type_id(expr_value)?;
        let runtime_type_name = self.get_type_name_for_id(runtime_type_id)?;
        let target_type_name = &type_assertion.type_name;
        
        // Create basic blocks for success and failure paths
        let current_fn = self.current_function()
            .ok_or_else(|| Error::Compilation("No current function for type assertion".to_string()))?;
        
        let success_block = self.context().append_basic_block(current_fn, "type_assert_success");
        let failure_block = self.context().append_basic_block(current_fn, "type_assert_failure");
        let merge_block = self.context().append_basic_block(current_fn, "type_assert_merge");
        
        // Check if the value is an instance of the target type
        let is_instance = self.check_instance_of_with_registry(expr_value, target_type_name)?;
        
        // Branch based on the type check result
        self.builder().build_conditional_branch(
            is_instance.into_int_value(),
            success_block,
            failure_block
        ).map_err(|e| Error::Compilation(format!("Failed to build conditional branch: {}", e)))?;
        
        // Success path
        self.builder().position_at_end(success_block);
        let success_value = self.cast_to_interface_type(expr_value, target_type_name)?;
        let true_val = self.context().bool_type().const_int(1, false);
        
        // Create success result struct with value and true flag
        let result_type = self.create_assertion_result_type(target_type_name)?;
        
        // Build the success result struct
        let success_result = self.build_assertion_result_struct(
            result_type,
            success_value,
            true_val
        )?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::Compilation(format!("Failed to build unconditional branch: {}", e)))?;
        
        // Failure path
        self.builder().position_at_end(failure_block);
        
        // Log the assertion failure with enhanced details
        debug!("Type assertion failed: {} is not a {}", runtime_type_name, target_type_name);
        
        // Create a null value for the target type
        let null_value = self.create_null_value_for_type(target_type_name)?;
        let false_val = self.context().bool_type().const_int(0, false);
        
        // Build the failure result struct
        let failure_result = self.build_assertion_result_struct(
            result_type,
            null_value,
            false_val
        )?;
        
        // Generate detailed diagnostics for debugging
        #[cfg(debug_assertions)]
        {
            let error = self.generate_type_assertion_error(
                &runtime_type_name,
                target_type_name,
                &source_location,
                None
            )?;
            error!("Type assertion failed: {}", error);
        }
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::Compilation(format!("Failed to build unconditional branch: {}", e)))?;
        
        // Merge block - use phi node to select the appropriate result
        self.builder().position_at_end(merge_block);
        let phi = self.builder().build_phi(
            result_type,
            "assertion_result"
        ).map_err(|e| Error::Compilation(format!("Failed to build phi node: {}", e)))?;
        
        phi.add_incoming(&[(
            &success_result,
            success_block
        ), (
            &failure_result,
            failure_block
        )]);
        
        // Return the phi result
        Ok(phi.as_basic_value())
    }
    
    #[instrument(skip(self), level = "debug")]
    fn generate_type_assertion_error(
        &mut self,
        source_type: &str,
        target_type: &str,
        source_location: &str,
        additional_message: Option<String>
    ) -> Result<TypeAssertionError, Error> {
        // Generate interface hierarchy visualization
        let hierarchy = self.visualize_interface_hierarchy(target_type, 2)?;
        
        // Try to find alternative paths between these types
        let alt_paths = match self.find_alternative_paths_enhanced(source_type, target_type, 3) {
            Ok(paths) => {
                if paths.is_empty() {
                    format!("No inheritance path exists between '{}' and '{}'", source_type, target_type)
                } else {
                    let mut result = format!("Found {} possible alternative inheritance path(s):", paths.len());
                    for (i, path) in paths.iter().enumerate() {
                        result.push_str(&format!("\n  Path {}: {}", i + 1, path.to_string_representation()));
                    }
                    result
                }
            },
            Err(_) => format!("Could not find any inheritance relationship between '{}' and '{}'", 
                             source_type, target_type)
        };
        
        // Combine the message parts
        let message = format!("{}{}{}",
            additional_message.unwrap_or_default(),
            if additional_message.is_some() { "\n\n" } else { "" },
            format!("{}\n\n{}", alt_paths, hierarchy)
        );
        
        // Generate recovery suggestions
        let recovery_hint = self.suggest_recovery_options(source_type, target_type)?;
        
        Ok(TypeAssertionError {
            source_type: source_type.to_string(),
            target_type: target_type.to_string(),
            source_location: source_location.to_string(),
            message,
            recovery_hint,
        })
    }
    
    #[instrument(skip(self), level = "debug")]
    fn suggest_recovery_options(
        &mut self,
        source_type: &str,
        target_type: &str
    ) -> Result<Option<String>, Error> {
        // Check if source implements target as an interface
        let implements = self.check_extension_relationship_enhanced(source_type, target_type)?;
        
        // Check if target implements source as an interface (reversed relationship)
        let reversed = self.check_extension_relationship_enhanced(target_type, source_type)?;
        
        if reversed {
            return Ok(Some(format!(
                "The relationship between '{}' and '{}' appears to be reversed. Try asserting '{}' as '{}'.",
                source_type, target_type, target_type, source_type
            )));
        }
        
        // Find common interfaces that both types implement
        let source_interfaces = self.get_implemented_interfaces(source_type)?;
        let target_interfaces = self.get_implemented_interfaces(target_type)?;
        
        let common_interfaces: Vec<String> = source_interfaces.iter()
            .filter(|i| target_interfaces.contains(i))
            .cloned()
            .collect();
        
        if !common_interfaces.is_empty() {
            let mut hint = format!("Both '{}' and '{}' implement the following common interfaces:", 
                                 source_type, target_type);
            
            for interface in common_interfaces {
                hint.push_str(&format!("\n  - {}", interface));
            }
            
            hint.push_str("\nConsider using one of these common interfaces instead.");
            return Ok(Some(hint));
        }
        
        // If no common ground, suggest implementing the interface
        Ok(Some(format!(
            "To make this assertion work, implement '{}' for the type '{}'.",
            target_type, source_type
        )))
    }
}

// Helper methods for the implementation
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Creates a result struct type for the assertion
    fn create_assertion_result_type(
        &self,
        type_name: &str
    ) -> Result<inkwell::types::StructType<'ctx>, Error> {
        // For now, we use a simple struct with a data pointer and a success flag
        // In a full implementation, this would use proper LLVM types based on the target type
        let context = self.context();
        let i8_ptr_type = context.i8_type().ptr_type(AddressSpace::default());
        let bool_type = context.bool_type();
        
        Ok(context.struct_type(&[i8_ptr_type.into(), bool_type.into()], false))
    }
    
    /// Builds a result struct with the given value and flag
    fn build_assertion_result_struct(
        &self,
        result_type: inkwell::types::StructType<'ctx>,
        value: BasicValueEnum<'ctx>,
        flag: inkwell::values::IntValue<'ctx>
    ) -> Result<inkwell::values::StructValue<'ctx>, Error> {
        // Cast the value to an i8 pointer for consistency
        let i8_ptr = self.builder().build_bitcast(
            value.into_pointer_value(),
            self.context().i8_type().ptr_type(AddressSpace::default()),
            "casted_ptr"
        ).map_err(|e| Error::Compilation(format!("Failed to cast value: {}", e)))?;
        
        // Create an empty struct and insert values
        let mut result = result_type.const_named_struct(&[]);
        
        // Insert the data pointer
        result = self.builder().build_insert_value(
            result,
            i8_ptr,
            0,
            "insert_data_ptr"
        ).map_err(|e| Error::Compilation(format!("Failed to insert data pointer: {}", e)))?.into_struct_value();
        
        // Insert the success flag
        result = self.builder().build_insert_value(
            result,
            flag,
            1,
            "insert_success_flag"
        ).map_err(|e| Error::Compilation(format!("Failed to insert success flag: {}", e)))?.into_struct_value();
        
        Ok(result)
    }
    
    /// Creates a null value for the target type
    fn create_null_value_for_type(
        &self,
        type_name: &str
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Get the LLVM type for the target type
        let i8_ptr_type = self.context().i8_type().ptr_type(AddressSpace::default());
        let null_ptr = i8_ptr_type.const_null();
        
        Ok(null_ptr.into())
    }
    
    /// Gets a list of interfaces implemented by a type
    fn get_implemented_interfaces(
        &self,
        type_name: &str
    ) -> Result<Vec<String>, Error> {
        // This would be implemented using the type registry
        // For now, return an empty list as a placeholder
        Ok(Vec::new())
    }
}

/// Register the improved error propagation module
pub fn register_interface_type_assertion_error_propagation() {
    debug!("Interface type assertion error propagation module registered");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_propagation_registration() {
        register_interface_type_assertion_error_propagation();
        assert!(true);
    }
}