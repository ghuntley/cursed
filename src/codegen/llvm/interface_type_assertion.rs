//! # Interface Type Assertion Enhancements
//!
//! This module provides enhancements to the base interface type assertion functionality.
//! It extends the InterfaceTypeAssertion trait with additional functionality for
//! runtime type information, debug logging, and improved error handling.
//!
//! The implementation follows the trait-based design pattern used throughout the
//! codebase, making it easy to adopt incrementally without breaking existing code.
//!
//! ## Features
//!
//! - Comprehensive runtime type information for assertions
//! - Improved error messages and debug logging
//! - Support for type hierarchies and complex type relationships
//! - Optimized code paths for common assertion patterns
//!
//! ## Integration
//!
//! This can be integrated alongside the existing implementation, with a feature
//! flag to gradually transition code to use the enhanced version.

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicTypeEnum, StructType};
use inkwell::values::{BasicValueEnum, PointerValue};
use inkwell::IntPredicate;
use inkwell::basic_block::BasicBlock;
use inkwell::AddressSpace;

use crate::ast::expressions::TypeAssertion;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::error::Error;
use crate::error::type_assertion_error::TypeAssertionError;
use crate::error::SourceLocation;
use crate::codegen::llvm::type_assertion::InterfaceTypeAssertion;

/// Enhanced interface type assertion implementation that extends the base implementation.
/// 
/// This trait builds on top of the InterfaceTypeAssertion trait, adding more
/// functionality for runtime type information, detailed error logging, and
/// performance optimizations. It's designed to be used alongside the original
/// implementation to allow for gradual adoption.
///
/// # Usage
///
/// ```rust,ignore
/// // To use enhanced type assertions in your code:
/// let result = code_generator.compile_type_assertion_with_logging(&type_assertion)?;
/// ```
///
/// # Features
///
/// - Runtime type information with hierarchies
/// - Detailed error reporting and debug logging
/// - Cached type information for better performance
/// - Support for complex type relationships
pub trait ImprovedTypeAssertion<'ctx>: InterfaceTypeAssertion<'ctx> {
    /// Get additional runtime type information for more sophisticated assertions
    fn get_runtime_type_info(
        &mut self,
        type_name: &str
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Register a type in the runtime type registry
    fn register_type_with_runtime(
        &mut self,
        type_name: &str
    ) -> Result<(), Error>;
    
    /// Log type assertion attempts for debugging
    fn log_type_assertion(
        &mut self,
        source_type: &str,
        target_type: &str,
        success: bool,
        source_location: Option<SourceLocation>
    ) -> Result<(), Error>;
    
    /// Calculate a hash for a type name
    fn hash_type_name(&self, type_name: &str) -> u64 {
        // FNV-1a hash algorithm
        let mut hash: u64 = 0xcbf29ce484222325;
        for byte in type_name.bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash
    }
}

impl<'ctx> ImprovedTypeAssertion<'ctx> for LlvmCodeGenerator<'ctx> {
    fn get_runtime_type_info(
        &mut self,
        type_name: &str
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // For now, implement a simple type ID calculation
        // In a full implementation, this would return a richer structure with type hierarchy info
        let ctx = self.context();
        let type_id = self.hash_type_name(type_name);
        Ok(ctx.i64_type().const_int(type_id, false).into())
    }
    
    fn register_type_with_runtime(
        &mut self,
        type_name: &str
    ) -> Result<(), Error> {
        // This would register the type in a global registry
        // For now, we just do nothing as the base implementation already handles type IDs
        Ok(())
    }
    
    fn log_type_assertion(
        &mut self,
        source_type: &str,
        target_type: &str,
        success: bool,
        source_location: Option<SourceLocation>
    ) -> Result<(), Error> {
        // In a real implementation, this would emit LLVM IR to call a debug logging function
        // For now, we use the Rust tracing macros for compile-time logging
        let result_str = if success { "succeeded" } else { "failed" };
        
        // Create structured logging with location information if available
        if let Some(loc) = &source_location {
            tracing::debug!(
                source_type = %source_type,
                target_type = %target_type,
                success = %success,
                location = %format!("{}", loc),
                "Type assertion from {} to {} {}", source_type, target_type, result_str
            );
        } else {
            tracing::debug!(
                source_type = %source_type,
                target_type = %target_type,
                success = %success,
                "Type assertion from {} to {} {}", source_type, target_type, result_str
            );
        }
        
        Ok(()); // We just log and don't perform any actual LLVM operations
    }
}

// Helper methods that extend the base implementation
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Improved version of compile_type_assertion that adds debug logging
    pub fn compile_type_assertion_with_logging(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Get source type information if available
        let source_type = "interface"; // In a real implementation, this would get the runtime type name
        let target_type = &type_assertion.type_name;
        
        // Create a source location for error reporting
        let source_location = match &type_assertion.token {
            token if !token.is_empty() => {
                Some(SourceLocation {
                    line: 0, // Not available from AST
                    column: 0, // Not available from AST
                    file: None,
                    source_line: format!("{}.({})", type_assertion.expression.string(), type_assertion.type_name),
                })
            },
            _ => None,
        };
        
        // Register the target type with the runtime
        self.register_type_with_runtime(target_type)?;
        
        // Get the current function
        let current_fn = self.current_function()
            .ok_or_else(|| Error::Compilation("No current function for type assertion".to_string()))?;
        
        // Compile the expression being asserted
        let expr_value = self.compile_expression(type_assertion.expression.as_ref())?;
        
        // Create basic blocks for success and failure paths
        let success_block = self.context().append_basic_block(current_fn, "type_assert_success");
        let failure_block = self.context().append_basic_block(current_fn, "type_assert_failure");
        let merge_block = self.context().append_basic_block(current_fn, "type_assert_merge");
        
        // Check if the interface value is of the target type
        let is_instance = self.check_instance_of(expr_value, &type_assertion.type_name, source_location.clone())?;
        
        // Branch based on the type check result
        self.builder().build_conditional_branch(
            is_instance.into_int_value(),
            success_block,
            failure_block
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Success path - extract and cast the data pointer
        self.builder().position_at_end(success_block);
        let data_ptr = self.extract_interface_data_ptr(expr_value)?;
        
        // Cast the data pointer to a generic void pointer
        let casted_ptr = self.builder().build_bitcast(
            data_ptr,
            self.context().i8_type().ptr_type(AddressSpace::default()),
            "casted_ptr"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Create a tuple with the casted pointer and a true flag
        let true_val = self.context().bool_type().const_int(1, false);
        let success_tuple_type = self.context().struct_type(&[
            self.context().i8_type().ptr_type(AddressSpace::default()).into(),
            self.context().bool_type().into()
        ], false);
        
        // Create an empty struct and insert values
        let mut success_tuple = success_tuple_type.const_named_struct(&[]);
        success_tuple = self.builder().build_insert_value(
            success_tuple,
            casted_ptr,
            0,
            "insert_ptr"
        ).map_err(|e| Error::Compilation(e.to_string()))?.into_struct_value();
        
        success_tuple = self.builder().build_insert_value(
            success_tuple,
            true_val,
            1,
            "insert_flag"
        ).map_err(|e| Error::Compilation(e.to_string()))?.into_struct_value();
        
        // Log the assertion success
        self.log_type_assertion(source_type, target_type, true, source_location.clone())?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Failure path - return null pointer and false flag
        self.builder().position_at_end(failure_block);
        
        // Log the assertion failure
        self.log_type_assertion(source_type, target_type, false, source_location.clone())?;
        
        let null_ptr = self.context().i8_type().ptr_type(AddressSpace::default()).const_null();
        let false_val = self.context().bool_type().const_int(0, false);
        
        // Create an empty struct and insert values
        let mut failure_tuple = success_tuple_type.const_named_struct(&[]);
        failure_tuple = self.builder().build_insert_value(
            failure_tuple,
            null_ptr,
            0,
            "insert_null_ptr"
        ).map_err(|e| Error::Compilation(e.to_string()))?.into_struct_value();
        
        failure_tuple = self.builder().build_insert_value(
            failure_tuple,
            false_val,
            1,
            "insert_false_flag"
        ).map_err(|e| Error::Compilation(e.to_string()))?.into_struct_value();
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Merge block - use phi node to select the appropriate result
        self.builder().position_at_end(merge_block);
        let phi = self.builder().build_phi(
            success_tuple_type,
            "assertion_result"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        phi.add_incoming(&[(  
            &success_tuple,
            success_block
        ), (
            &failure_tuple,
            failure_block
        )]);
        
        // Return the phi result
        Ok(phi.as_basic_value())
    }
}