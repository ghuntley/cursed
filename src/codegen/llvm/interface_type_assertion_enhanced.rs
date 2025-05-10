//! # Enhanced Interface Type Assertion Error Handling
//!
//! This module provides improved error handling for interface type assertions
//! by integrating with the enhanced type registry. It enables better error messages
//! with proper type information during runtime type assertions.

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicType, BasicTypeEnum, StructType};
use inkwell::values::{BasicValueEnum, PointerValue};
use inkwell::IntPredicate;
use inkwell::basic_block::BasicBlock;
use inkwell::AddressSpace;

use tracing::{debug, error, info, instrument, trace, warn, Level};

use crate::ast::expressions::TypeAssertion;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::error::Error;
use crate::codegen::llvm::interface_type_registry_enhanced::EnhancedTypeRegistry;
use crate::codegen::llvm::interface_type_assertion_errors::TypeAssertionErrorHandler;

/// Trait for implementing enhanced type assertions with rich type information
pub trait EnhancedTypeAssertion<'ctx> {
    /// Compile a type assertion with enhanced error reporting and debugging
    fn compile_enhanced_type_assertion(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Report detailed type information for assertion failures
    fn report_assertion_failure(
        &mut self,
        actual_type_id: BasicValueEnum<'ctx>,
        expected_type_name: &str
    ) -> Result<(), Error>;
    
    /// Generate type assertion with runtime error messages
    fn check_instance_of_with_enhanced_errors(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type_name: &str
    ) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> EnhancedTypeAssertion<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self, type_assertion), level = "debug")]
    fn compile_enhanced_type_assertion(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling enhanced type assertion for {}", type_assertion.type_name);
        
        // Get the current function
        let current_fn = self.current_function()
            .ok_or_else(|| Error::codegen("No current function for enhanced type assertion"))?;
        
        // Compile the expression being asserted
        let expr_value = match self.compile_expression(type_assertion.expression.as_ref()) {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to compile expression for enhanced type assertion: {}", e);
                return Err(Error::codegen(
                    format!("Failed to compile expression for enhanced type assertion: {}", e)
                ));
            }
        };
        
        // Create basic blocks for success and failure paths
        let success_block = self.context().append_basic_block(current_fn, "type_assert_success");
        let failure_block = self.context().append_basic_block(current_fn, "type_assert_failure");
        let merge_block = self.context().append_basic_block(current_fn, "type_assert_merge");
        
        // Check if the interface value is of the target type with enhanced error reporting
        let is_instance = self.check_instance_of_with_enhanced_errors(expr_value, &type_assertion.type_name)?;
        
        // Branch based on the type check result
        self.builder().build_conditional_branch(
            is_instance.into_int_value(),
            success_block,
            failure_block
        ).map_err(|e| {
            error!("Failed to build conditional branch: {}", e);
            Error::codegen(format!("Failed to build conditional branch: {}", e))
        })?;
        
        // Success path - extract and cast the data pointer
        self.builder().position_at_end(success_block);
        let data_ptr = self.extract_interface_data_ptr_safe(expr_value)?;
        
        // Get the type ID for logging
        let type_id = self.get_type_id(&type_assertion.type_name).map_err(|e| {
            error!("Failed to get type ID for {}: {}", type_assertion.type_name, e);
            Error::codegen(format!("Failed to get type ID for {}: {}", type_assertion.type_name, e))
        })?;
        
        // Create the result structure (value and true flag)
        let casted_ptr = self.builder().build_bitcast(
            data_ptr,
            self.pointer_type(),
            "casted_ptr"
        ).map_err(|e| {
            error!("Failed to cast data pointer: {}", e);
            Error::codegen(format!("Failed to cast data pointer: {}", e))
        })?;
        
        // Pack the result into a tuple structure
        let true_val = self.context().bool_type().const_int(1, false);
        let success_result = self.build_tuple(vec![casted_ptr.into(), true_val.into()])?;
        
        // Log success with detailed type information if debugging is enabled
        if self.is_type_debug_enabled() {
            // Get the type name string pointer
            let type_name_ptr = self.lookup_type_name_enhanced(type_id)?;
            
            // Log the successful assertion
            info!(
                "Type assertion SUCCESS: value successfully converted to {}", 
                type_assertion.type_name
            );
        }
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| {
                error!("Failed to build branch to merge block: {}", e);
                Error::codegen(format!("Failed to build branch to merge block: {}", e))
            })?;
        
        // Failure path - return null pointer and false flag
        self.builder().position_at_end(failure_block);
        
        // Extract the actual type information for better error messages
        let actual_type_id = self.get_interface_type_id_safe(expr_value).unwrap_or_else(|_| {
            self.context().i64_type().const_int(u64::MAX, false).into()
        });
        
        // Report the type assertion failure with detailed information
        if self.is_type_debug_enabled() {
            let _ = self.report_assertion_failure(actual_type_id, &type_assertion.type_name);
        }
        
        // Create null result with false flag
        let null_ptr = self.context().i8_type().ptr_type(AddressSpace::default()).const_null();
        let false_val = self.context().bool_type().const_int(0, false);
        let failure_result = self.build_tuple(vec![null_ptr.into(), false_val.into()])?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| {
                error!("Failed to build branch from failure block: {}", e);
                Error::codegen(format!("Failed to build branch from failure block: {}", e))
            })?;
        
        // Merge block - use phi node to select the appropriate result
        self.builder().position_at_end(merge_block);
        
        // Create the result type (tuple of pointer and bool)
        let result_type = self.tuple_type(vec![self.pointer_type().into(), self.context().bool_type().into()]);
        
        let phi = self.builder().build_phi(
            result_type,
            "enhanced_assertion_result"
        ).map_err(|e| {
            error!("Failed to build phi node: {}", e);
            Error::codegen(format!("Failed to build phi node: {}", e))
        })?;
        
        phi.add_incoming(&[(
            &success_result,
            success_block
        ), (
            &failure_result,
            failure_block
        )]);
        
        debug!("Enhanced type assertion compiled successfully");
        Ok(phi.as_basic_value())
    }
    
    #[instrument(skip(self, actual_type_id), level = "debug")]
    fn report_assertion_failure(
        &mut self,
        actual_type_id: BasicValueEnum<'ctx>,
        expected_type_name: &str
    ) -> Result<(), Error> {
        // Only log if debugging is enabled
        if !self.is_type_debug_enabled() {
            return Ok(());
        }
        
        // Convert the actual type ID to a human-readable name
        let actual_type_name = self.lookup_type_name_enhanced(actual_type_id)?;
        
        // Log failure with both actual and expected type information
        let actual_id_val = if actual_type_id.is_int_value() {
            actual_type_id.into_int_value().get_zero_extended_constant().unwrap_or(u64::MAX)
        } else {
            u64::MAX
        };
        
        // Get the actual type name
        let actual_type_str = if let Some(registry) = &self.interface_type_registry {
            registry.get_type_name(actual_id_val)
                .map(|s| s.clone())
                .unwrap_or_else(|| "Unknown Type".to_string())
        } else {
            "Unknown Type".to_string()
        };
        
        warn!(
            "Type assertion FAILED: Cannot convert from '{}' to '{}'", 
            actual_type_str, expected_type_name
        );
        
        Ok(())
    }
    
    #[instrument(skip(self, interface_value), level = "debug")]
    fn check_instance_of_with_enhanced_errors(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type_name: &str
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Enhanced type check for {} with detailed error reporting", target_type_name);
        
        // Ensure type registry globals are properly initialized
        if let Some(registry) = &self.interface_type_registry {
            if registry.type_ids_global.is_none() || registry.type_names_global.is_none() {
                self.initialize_type_registry_globals()?;
            }
        }
        
        // Get the type ID from the interface value
        let actual_type_id = self.get_interface_type_id_safe(interface_value)?;
        
        // Get the expected type ID for the target type
        let expected_type_id = self.get_type_id(target_type_name).map_err(|e| {
            error!("Failed to get type ID for {}: {}", target_type_name, e);
            Error::codegen(format!("Failed to get type ID for {}: {}", target_type_name, e))
        })?;
        
        // Log enhanced type comparison details if debugging is enabled
        if self.is_type_debug_enabled() {
            let actual_id_const = if actual_type_id.is_int_value() {
                actual_type_id.into_int_value().get_zero_extended_constant().unwrap_or(u64::MAX)
            } else {
                u64::MAX
            };
            
            // Look up the actual type name
            let actual_type_name = if let Some(registry) = &self.interface_type_registry {
                registry.get_type_name(actual_id_const)
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown")
            } else {
                "Unknown"
            };
            
            debug!("Comparing types: actual '{}' (ID: {}) with expected '{}'", 
                   actual_type_name, actual_id_const, target_type_name);
        }
        
        // Compare the type IDs
        let result = self.builder().build_int_compare(
            IntPredicate::EQ,
            actual_type_id.into_int_value(),
            expected_type_id.into_int_value(),
            "enhanced_is_instance_of"
        ).map_err(|e| {
            error!("Failed to compare type IDs: {}", e);
            Error::codegen(format!("Failed to compare type IDs: {}", e))
        })?;
        
        debug!("Enhanced instance check completed");
        Ok(result.into())
    }
}

// Helper methods for enhanced type assertions
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Check if type debugging is enabled
    pub fn is_type_debug_enabled(&self) -> bool {
        std::env::var("CURSED_TYPE_DEBUG")
            .or_else(|_| std::env::var("CURSED_DEBUG"))
            .map(|val| !val.is_empty() && val != "0" && val.to_lowercase() != "false")
            .unwrap_or(false)
    }
    
    /// Create entry block allocas for temporary variables
    pub fn create_entry_block_alloca<T: BasicType<'ctx>>(
        &self,
        ty: T,
        name: &str
    ) -> PointerValue<'ctx> {
        let builder = self.context.create_builder();
        
        let entry = self.current_function()
            .and_then(|f| f.get_first_basic_block())
            .expect("Unable to get entry block to create alloca");
        
        let entry_terminator = entry.get_terminator();
        if let Some(terminator) = entry_terminator {
            builder.position_before(&terminator);
        } else {
            builder.position_at_end(entry);
        }
        
        builder.build_alloca(ty, name).expect("Failed to create alloca")
    }
}

/// Register the enhanced type assertion implementation
pub fn register_enhanced_type_assertion() {
    debug!("Registering enhanced type assertion implementation");
    // This function is called during LlvmCodeGenerator initialization
}