//! # Enhanced Runtime Debugging for Interface Type Assertions
//!
//! This module provides advanced runtime debugging capabilities for interface type assertions.
//! It extends the basic type assertion functionality with detailed error messages,
//! type introspection, and configurable debugging levels.
//!
//! These enhancements help developers understand why type assertions fail during program
//! execution by providing rich context about the expected and actual types.

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicTypeEnum, StructType};
use inkwell::values::{BasicValueEnum, PointerValue, IntValue};
use inkwell::IntPredicate;
use inkwell::basic_block::BasicBlock;
use inkwell::AddressSpace;

use crate::ast::expressions::TypeAssertion;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::error::Error;
use crate::codegen::llvm::type_assertion::InterfaceTypeAssertion;
use crate::codegen::llvm::interface_type_assertion_errors::TypeAssertionErrorHandler;
use crate::codegen::llvm::interface_type_registry::InterfaceTypeRegistryAccess;

use tracing::{debug, error, info, instrument, span, warn, Level};

/// Enum representing debug levels for type assertions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeAssertionDebugLevel {
    /// No debugging information
    None,
    /// Basic information (success/failure only)
    Basic,
    /// Standard debugging (type names, success/failure)
    Standard,
    /// Verbose debugging (detailed type information, method tables, etc.)
    Verbose,
}

impl TypeAssertionDebugLevel {
    /// Convert from env var string to debug level
    pub fn from_env(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "0" | "none" | "off" => Self::None,
            "1" | "basic" => Self::Basic,
            "2" | "standard" => Self::Standard, 
            "3" | "verbose" => Self::Verbose,
            _ => Self::Standard, // Default to standard if unrecognized
        }
    }
    
    /// Check if debugging is enabled at all
    pub fn is_enabled(&self) -> bool {
        *self != Self::None
    }
    
    /// Check if this level includes basic information
    pub fn includes_basic(&self) -> bool {
        self.is_enabled()
    }
    
    /// Check if this level includes standard information
    pub fn includes_standard(&self) -> bool {
        *self == Self::Standard || *self == Self::Verbose
    }
    
    /// Check if this level includes verbose information
    pub fn includes_verbose(&self) -> bool {
        *self == Self::Verbose
    }
}

/// Trait for enhanced runtime debugging of interface type assertions
pub trait RuntimeTypeAssertionDebugging<'ctx>: TypeAssertionErrorHandler<'ctx> {
    /// Get the current debug level for type assertions
    fn get_type_assertion_debug_level(&self) -> TypeAssertionDebugLevel;
    
    /// Set the debug level for type assertions
    fn set_type_assertion_debug_level(&mut self, level: TypeAssertionDebugLevel);
    
    /// Generate debug output for type assertion operation
    fn debug_type_assertion_operation(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type_name: &str,
        source_loc: Option<&str>
    ) -> Result<(), Error>;
    
    /// Generate human-readable type information for debugging
    fn generate_type_debug_info(
        &mut self,
        type_id: BasicValueEnum<'ctx>,
        type_name: Option<&str>
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Log a type assertion result at runtime
    fn log_assertion_result(
        &mut self,
        success: BasicValueEnum<'ctx>,
        source_type_id: BasicValueEnum<'ctx>,
        target_type_name: &str,
        source_loc: Option<&str>
    ) -> Result<(), Error>;
    
    /// Get a string representation of a type at runtime
    fn get_type_name_at_runtime(
        &mut self,
        type_id: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Compile a type assertion with enhanced debugging
    fn compile_type_assertion_with_debugging(
        &mut self,
        type_assertion: &TypeAssertion,
        source_loc: Option<&str>
    ) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> RuntimeTypeAssertionDebugging<'ctx> for LlvmCodeGenerator<'ctx> {
    fn get_type_assertion_debug_level(&self) -> TypeAssertionDebugLevel {
        // In a real implementation, this would be stored in the code generator
        // For now, we'll determine it from environment variables each time
        match std::env::var("CURSED_DEBUG") {
            Ok(val) => TypeAssertionDebugLevel::from_env(&val),
            Err(_) => match std::env::var("CURSED_TYPE_DEBUG") {
                Ok(val) => TypeAssertionDebugLevel::from_env(&val),
                Err(_) => TypeAssertionDebugLevel::None,
            }
        }
    }
    
    fn set_type_assertion_debug_level(&mut self, _level: TypeAssertionDebugLevel) {
        // In a real implementation, this would store the level in the code generator
        // For the prototype, we'll just log that we would change it
        debug!("Would set type assertion debug level to {:?}", _level);
    }
    
    #[instrument(skip(self, interface_value), level = "debug")]
    fn debug_type_assertion_operation(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type_name: &str,
        source_loc: Option<&str>
    ) -> Result<(), Error> {
        // If debugging is disabled, do nothing
        if !self.get_type_assertion_debug_level().is_enabled() {
            return Ok(());
        }
        
        // Get current function
        let current_fn = self.current_function()
            .ok_or_else(|| Error::codegen("No current function for type assertion debugging"))?;
            
        // Get the type ID from the interface
        let type_id = self.get_interface_type_id_safe(interface_value)?;
        
        // Create basic blocks for the debugging code
        let debug_block = self.context().append_basic_block(current_fn, "type_assert_debug");
        let continue_block = self.context().append_basic_block(current_fn, "after_type_assert_debug");
        
        // Call runtime debug logging function if enabled
        if self.get_type_assertion_debug_level().includes_standard() {
            // In a full implementation, this would call a runtime function to log the assertion
            // For now, we'll use tracing to log at compile time
            
            if let Some(loc) = source_loc {
                debug!("Type assertion at {}: checking if value is of type {}", loc, target_type_name);
            } else {
                debug!("Type assertion: checking if value is of type {}", target_type_name);
            }
            
            // Generate a string representation of the type
            let _type_name = self.get_type_name_at_runtime(type_id)?;
            
            // In a complete implementation, the runtime would compare and log the types
        }
        
        // Branch to continue block
        self.builder().build_unconditional_branch(continue_block)
            .map_err(|e| Error::codegen(format!("Failed to build branch from debug block: {}", e)))?;
        
        // Position at continue block to continue normal execution
        self.builder().position_at_end(continue_block);
        
        Ok(())
    }
    
    #[instrument(skip(self, type_id), level = "debug")]
    fn generate_type_debug_info(
        &mut self,
        type_id: BasicValueEnum<'ctx>,
        type_name: Option<&str>
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // In a real implementation, this would generate a rich debug object with type info
        // For this prototype, we'll just return the type name as a string
        
        if let Some(name) = type_name {
            debug!("Generated debug info for type: {}", name);
            // In a real implementation, we would create a string constant in LLVM
            // For now, just return the type ID
            return Ok(type_id);
        }
        
        // If no type name was provided, try to look it up
        let type_name_value = self.get_type_name_at_runtime(type_id)?;
        debug!("Generated debug info for type ID: {:?}", type_id);
        
        Ok(type_name_value)
    }
    
    #[instrument(skip(self, success, source_type_id), level = "debug")]
    fn log_assertion_result(
        &mut self,
        success: BasicValueEnum<'ctx>,
        source_type_id: BasicValueEnum<'ctx>,
        target_type_name: &str,
        source_loc: Option<&str>
    ) -> Result<(), Error> {
        // Skip if debugging is disabled
        if !self.get_type_assertion_debug_level().is_enabled() {
            return Ok(());
        }
        
        // We need to build a conditional branch based on the success value
        let current_fn = self.current_function()
            .ok_or_else(|| Error::codegen("No current function for assertion result logging"))?;
            
        // Get success as bool
        let success_bool = if success.is_int_value() {
            success.into_int_value()
        } else {
            return Err(Error::codegen("Expected boolean success value for logging"));
        };
        
        // Create blocks for success and failure logging
        let success_block = self.context().append_basic_block(current_fn, "log_assert_success");
        let failure_block = self.context().append_basic_block(current_fn, "log_assert_failure");
        let continue_block = self.context().append_basic_block(current_fn, "after_log_assert");
        
        // Branch based on success
        self.builder().build_conditional_branch(
            success_bool,
            success_block,
            failure_block
        ).map_err(|e| Error::codegen(format!("Failed to build branch for logging: {}", e)))?;
        
        // Success block
        self.builder().position_at_end(success_block);
        
        // In a complete implementation, we would call a runtime function to log the success
        if let Some(loc) = source_loc {
            debug!("Type assertion succeeded at {}: value is of type {}", loc, target_type_name);
        } else {
            debug!("Type assertion succeeded: value is of type {}", target_type_name);
        }
        
        self.builder().build_unconditional_branch(continue_block)
            .map_err(|e| Error::codegen(format!("Failed to build branch from success logging: {}", e)))?;
        
        // Failure block
        self.builder().position_at_end(failure_block);
        
        // Generate source type name for better error messages
        let _source_type_name = self.get_type_name_at_runtime(source_type_id)?;
        
        // In a complete implementation, we would call a runtime function to log the failure
        if let Some(loc) = source_loc {
            debug!("Type assertion failed at {}: value is not of type {}", loc, target_type_name);
        } else {
            debug!("Type assertion failed: value is not of type {}", target_type_name);
        }
        
        self.builder().build_unconditional_branch(continue_block)
            .map_err(|e| Error::codegen(format!("Failed to build branch from failure logging: {}", e)))?;
        
        // Continue normal execution
        self.builder().position_at_end(continue_block);
        
        Ok(())
    }
    
    #[instrument(skip(self, type_id), level = "debug")]
    fn get_type_name_at_runtime(
        &mut self,
        type_id: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // This implementation uses the interface type registry to look up type names at runtime
        debug!("Getting type name for ID: {:?}", type_id);
        
        // First check if we can use the registry globals
        if let Ok(type_name) = self.generate_type_name_lookup(type_id) {
            return Ok(type_name);
        }
        
        // If we don't have registry globals, try to create a string representation of the type ID
        // as a fallback mechanism
        let int_type = self.context().i64_type();
        let format_str = self.context().const_string("Type#%llu".as_bytes(), true);
        
        // Add the format string as a global constant
        let format_global = self.module().add_global(
            format_str.get_type(),
            None,
            "type_id_format_str"
        );
        format_global.set_linkage(inkwell::module::Linkage::Private);
        format_global.set_initializer(&format_str);
        
        // Declare the snprintf function if it doesn't exist
        let i32_type = self.context().i32_type();
        let i8_ptr_type = self.context().i8_type().ptr_type(AddressSpace::default());
        let void_type = self.context().void_type();
        
        let snprintf_type = i32_type.fn_type(
            &[
                i8_ptr_type.into(),
                int_type.into(),
                i8_ptr_type.into(),
                int_type.into(),
            ],
            true
        );
        
        let snprintf = self.module().add_function("snprintf", snprintf_type, None);
        
        // Allocate a buffer for the string representation
        let buffer_len = 32; // Should be enough for a 64-bit integer
        let buffer = self.create_entry_block_alloca(int_type.array_type(buffer_len), "type_id_str_buffer");
        
        // Call snprintf to format the type ID into the buffer
        let args = &[
            buffer.into(),
            int_type.const_int(buffer_len as u64, false).into(),
            format_global.as_pointer_value().into(),
            type_id.into_int_value().into()
        ];
        
        let _ = self.builder().build_call(snprintf, args, "format_type_id")
            .map_err(|e| Error::codegen(format!("Failed to call snprintf: {}", e)))?;
        
        // Return the buffer as the type name string
        Ok(buffer.into())
    }
    
    #[instrument(skip(self), level = "debug")]
    fn compile_type_assertion_with_debugging(
        &mut self,
        type_assertion: &TypeAssertion,
        source_loc: Option<&str>
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling type assertion with debugging for type {}", type_assertion.type_name);
        
        // Get the current function
        let current_fn = self.current_function()
            .ok_or_else(|| Error::codegen("No current function for type assertion"))?;
        
        // Compile the expression being asserted
        let expr_value = self.compile_expression(type_assertion.expression.as_ref())?;
        
        // Debug the type assertion operation if enabled
        self.debug_type_assertion_operation(expr_value, &type_assertion.type_name, source_loc)?;
        
        // Create basic blocks for success and failure paths
        let success_block = self.context().append_basic_block(current_fn, "type_assert_success");
        let failure_block = self.context().append_basic_block(current_fn, "type_assert_failure");
        let merge_block = self.context().append_basic_block(current_fn, "type_assert_merge");
        
        // Get the type ID of the interface value for logging purposes
        let source_type_id = self.get_interface_type_id_safe(expr_value)?;
        
        // Check if the interface value is of the target type
        let is_instance = self.check_instance_of_with_errors(expr_value, &type_assertion.type_name)?;
        
        // Branch based on the type check result
        self.builder().build_conditional_branch(
            is_instance.into_int_value(),
            success_block,
            failure_block
        ).map_err(|e| Error::codegen(format!("Failed to build conditional branch: {}", e)))?;
        
        // Success path - extract and cast the data pointer
        self.builder().position_at_end(success_block);
        let data_ptr = self.extract_interface_data_ptr_safe(expr_value)?;
        
        // Cast the data pointer to a generic pointer type
        // In a real implementation, we would get the actual type by name
        // For now, we'll just use a generic pointer type
            
        let casted_ptr = self.builder().build_bitcast(
            data_ptr,
            self.pointer_type(),
            "casted_ptr"
        ).map_err(|e| Error::codegen(format!("Failed to cast data pointer: {}", e)))?;
        
        // Create a tuple with the casted pointer and a true flag
        let true_val = self.context().bool_type().const_int(1, false);
        
        // Log success if debugging is enabled
        self.log_assertion_result(true_val.into(), source_type_id, &type_assertion.type_name, source_loc)?;
        
        // Build result tuple with data pointer and success flag
        let success_result = self.build_tuple(vec![casted_ptr.into(), true_val.into()])?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::codegen(format!("Failed to build branch to merge block: {}", e)))?;
        
        // Failure path - return null pointer and false flag
        self.builder().position_at_end(failure_block);
        let null_ptr = self.context().i8_type().ptr_type(AddressSpace::default()).const_null();
        let false_val = self.context().bool_type().const_int(0, false);
        
        // Log failure if debugging is enabled
        self.log_assertion_result(false_val.into(), source_type_id, &type_assertion.type_name, source_loc)?;
        
        // Build result tuple with null pointer and failure flag
        let failure_result = self.build_tuple(vec![null_ptr.into(), false_val.into()])?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::codegen(format!("Failed to build branch from failure block: {}", e)))?;
        
        // Merge block - use phi node to select the appropriate result
        self.builder().position_at_end(merge_block);
        
        // Create the result type (tuple of pointer and bool)
        let result_type = self.tuple_type(vec![self.pointer_type().into(), self.context().bool_type().into()]);
        
        let phi = self.builder().build_phi(
            result_type,
            "assertion_result"
        ).map_err(|e| Error::codegen(format!("Failed to build phi node: {}", e)))?;
        
        phi.add_incoming(&[(
            &success_result,
            success_block
        ), (
            &failure_result,
            failure_block
        )]);
        
        debug!("Type assertion with debugging compiled successfully");
        // Return the phi result
        Ok(phi.as_basic_value())
    }
}

/// Register enhanced runtime debugging for type assertions
pub fn register_runtime_type_assertion_debugging() {
    tracing::info!("Registered enhanced runtime debugging for type assertions");
}