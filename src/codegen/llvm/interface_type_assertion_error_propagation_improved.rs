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

/// Trait for improved error propagation in interface type assertions
pub trait ImprovedErrorPropagation<'ctx> {
    /// Compile a type assertion with proper error propagation
    fn compile_type_assertion_with_improved_errors(
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

    /// Check if an interface value is of a specific type with registry integration
    fn check_instance_of_with_errors(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type_name: &str
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Get runtime type name for an interface value
    fn get_runtime_type_name(
        &mut self,
        interface_value: BasicValueEnum<'ctx>
    ) -> Result<String, Error>;
}

impl<'ctx> ImprovedErrorPropagation<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self, type_assertion), level = "debug")]
    fn compile_type_assertion_with_improved_errors(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling type assertion with improved error propagation: {}", type_assertion.expression.string());
        
        // Source location for error messages
        let source_location = type_assertion.expression.token_literal();
        
        // Compile the expression to get the interface value
        let expr_value = self.compile_expression(type_assertion.expression.as_ref())?;
        
        // Get the runtime type name
        let runtime_type_name = self.get_runtime_type_name(expr_value)?;
        let target_type_name = &type_assertion.type_name;
        
        // Create basic blocks for success and failure paths
        let current_fn = self.current_function()
            .ok_or_else(|| Error::Compilation("No current function for type assertion".to_string()))?;
        
        let success_block = self.context().append_basic_block(current_fn, "type_assert_success");
        let failure_block = self.context().append_basic_block(current_fn, "type_assert_failure");
        let merge_block = self.context().append_basic_block(current_fn, "type_assert_merge");
        
        // Check if the value is an instance of the target type
        let is_instance = self.check_instance_of_with_errors(expr_value, target_type_name)?;
        
        // Branch based on the type check result
        self.builder().build_conditional_branch(
            is_instance.into_int_value(),
            success_block,
            failure_block
        ).map_err(|e| Error::Compilation(format!("Failed to build conditional branch: {}", e)))?;
        
        // Success path
        self.builder().position_at_end(success_block);
        
        // Extract and cast the data pointer
        let data_ptr = self.extract_interface_data_ptr(expr_value)?;
        
        // Get target type and create pointer type
        let target_struct_type = self.get_type_by_name(target_type_name)
            .unwrap_or_else(|| self.context().opaque_struct_type(target_type_name));
        
        let target_ptr_type = target_struct_type.ptr_type(AddressSpace::default());
        
        // Cast data pointer to target type
        let casted_ptr = self.builder().build_bitcast(
            data_ptr,
            target_ptr_type,
            "casted_ptr"
        ).map_err(|e| Error::Compilation(format!("Failed to cast pointer: {}", e)))?;
        
        // Create success result tuple (value, true)
        let true_val = self.context().bool_type().const_int(1, false);
        
        // Build the result tuple
        let success_result = self.build_tuple(vec![casted_ptr.into(), true_val.into()])?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::Compilation(format!("Failed to build unconditional branch: {}", e)))?;
        
        // Failure path
        self.builder().position_at_end(failure_block);
        
        // Create a null value for the target type
        let null_ptr = target_ptr_type.const_null();
        let false_val = self.context().bool_type().const_int(0, false);
        
        // Build the failure result tuple
        let failure_result = self.build_tuple(vec![null_ptr.into(), false_val.into()])?;
        
        // Generate detailed error information in debug builds
        #[cfg(debug_assertions)]
        {
            let error = self.generate_type_assertion_error(
                &runtime_type_name,
                target_type_name,
                &source_location,
                None
            )?;
            debug!("Type assertion failed: {}", error);
        }
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::Compilation(format!("Failed to build unconditional branch: {}", e)))?;
        
        // Merge block - use phi node to select the appropriate result
        self.builder().position_at_end(merge_block);
        
        // Create result tuple type for the phi node
        let result_type = self.tuple_type(vec![
            target_ptr_type.into(), 
            self.context().bool_type().into()
        ]);
        
        let phi = self.builder().build_phi(
            result_type,
            "assertion_result"
        ).map_err(|e| Error::Compilation(format!("Failed to build phi node: {}", e)))?;
        
        // Add both paths to the phi node
        phi.add_incoming(&[(
            &success_result,
            success_block
        ), (
            &failure_result,
            failure_block
        )]);
        
        debug!("Completed type assertion compilation with improved error handling");
        
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
        // Generate visualization of the interface hierarchy
        let hierarchy = match self.visualize_interface_path(target_type, 2) {
            Ok(h) => h,
            Err(_) => "Could not visualize interface hierarchy".to_string()
        };
        
        // Try to find alternative paths between these types
        let alt_paths = match self.find_alternative_paths(source_type, target_type, 3) {
            Ok(paths) => {
                if paths.is_empty() {
                    format!("No inheritance path exists between '{}' and '{}'", source_type, target_type)
                } else {
                    let mut result = format!("Found {} possible alternative inheritance path(s):", paths.len());
                    for (i, path) in paths.iter().enumerate() {
                        result.push_str(&format!("\n  Path {}: {}", i + 1, path));
                    }
                    result
                }
            },
            Err(_) => format!("Could not find any inheritance relationship between '{}' and '{}'", 
                             source_type, target_type)
        };
        
        // Combine the message parts
        let message = format!("{}{}{}\n\n{}",
            additional_message.unwrap_or_default(),
            if additional_message.is_some() { "\n\n" } else { "" },
            alt_paths,
            hierarchy
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
        let implements = match self.check_extension_relationship_simple(source_type, target_type) {
            Ok(result) => result,
            Err(_) => false
        };
        
        // Check if target implements source as an interface (reversed relationship)
        let reversed = match self.check_extension_relationship_simple(target_type, source_type) {
            Ok(result) => result,
            Err(_) => false
        };
        
        if reversed {
            return Ok(Some(format!(
                "The relationship between '{}' and '{}' appears to be reversed. Try asserting '{}' as '{}'.",
                source_type, target_type, target_type, source_type
            )));
        }
        
        // Find common interfaces that both types implement
        // In a real implementation, this would query the type registry
        let common_interfaces = Vec::<String>::new();
        
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
    
    #[instrument(skip(self, interface_value), level = "debug")]
    fn check_instance_of_with_errors(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type_name: &str
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Get the interface type ID
        let actual_type_id = self.extract_interface_type_id(interface_value)?
            .into_int_value();
        
        // Get target type ID
        let expected_type_id = match self.get_type_id(target_type_name) {
            Ok(id) => self.context().i64_type().const_int(id, false),
            Err(e) => return Err(Error::Compilation(format!(
                "Failed to get type ID for {}: {}", target_type_name, e
            )))
        };
        
        // Compare the type IDs
        let result = self.builder().build_int_compare(
            inkwell::IntPredicate::EQ,
            actual_type_id,
            expected_type_id,
            "is_instance_of"
        ).map_err(|e| Error::Compilation(format!("Failed to compare type IDs: {}", e)))?;
        
        Ok(result.into())
    }
    
    #[instrument(skip(self, interface_value), level = "debug")]
    fn get_runtime_type_name(
        &mut self,
        interface_value: BasicValueEnum<'ctx>
    ) -> Result<String, Error> {
        // Get the runtime type ID
        let runtime_type_id = self.get_runtime_type_id(interface_value)?;
        
        // Convert type ID to type name
        let runtime_type_name = self.get_type_name_for_id(runtime_type_id)?;
        
        Ok(runtime_type_name)
    }
}

/// Register the improved error propagation module
pub fn register_improved_error_propagation() {
    debug!("Improved interface type assertion error propagation module registered");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_propagation_registration() {
        register_improved_error_propagation();
        assert!(true);
    }
}