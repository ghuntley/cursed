//! # Interface Type Assertion Result Integration
//!
//! This module enhances interface type assertions by providing integrated support for
//! the Rust `Result` type pattern throughout the type assertion process. It builds on
//! the existing error propagation system but adds deeper integration with Rust's ?
//! operator and Result handling patterns.
//!
//! ## Features
//!
//! 1. Full Result-based error propagation using the ? operator
//! 2. Unified error context with rich diagnostic information
//! 3. Integration with existing type assertion modules
//! 4. Proper error conversion between different error types
//! 5. Support for collecting and reporting multiple related errors

use inkwell::values::BasicValueEnum;
use inkwell::IntPredicate;
use inkwell::AddressSpace;
use tracing::{debug, error, info, trace, warn, instrument, span, Level};
use crate::codegen::llvm::basic_value_extensions::{BasicValueExt, BoolValueExt, StructTypeExt};

use crate::ast::expressions::TypeAssertion;
use crate::ast::traits::Node;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::error::Error;
use crate::codegen::llvm::type_assertion::InterfaceTypeAssertion;
use crate::codegen::llvm::llvm_code_generator_extensions::{SymbolLookupExtensions, ErrorPathExtensions};
use crate::codegen::llvm::interface_type_assertion_error_propagation::TypeAssertionErrorPropagation;
use crate::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;

/// A trait that integrates Result-based error handling for interface type assertions
pub trait TypeAssertionResultIntegration<'ctx> {
    /// Compile a type assertion with full Result integration
    fn compile_type_assertion_with_result(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Convert between different error types in the type assertion process
    fn convert_type_assertion_error(&self, error: Error, context: &str) -> Error;
    
    /// Collect multiple errors from a type assertion process
    fn collect_type_assertion_errors(&mut self, errors: Vec<Error>) -> Error;
    
    /// Create a detailed error report for type assertion failures
    fn create_type_assertion_error_report(
        &self,
        interface_type: &str,
        target_type: &str,
        source_location: &str,
        errors: &[Error]
    ) -> String;
}

impl<'ctx> TypeAssertionResultIntegration<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn compile_type_assertion_with_result(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        let _span = span!(Level::DEBUG, "compile_type_assertion_with_result").entered();
        debug!("Compiling type assertion with full Result integration for: {}", type_assertion.string());
        
        // Get source location for better error messages
        let source_location = format!("{}", type_assertion.token_literal());
        
        // Get the current function using ? operator
        let current_fn = self.current_function()
            .ok_or_else(|| Error::Compilation(
                format!("No current function for type assertion at {}", source_location)
            ))?;
        
        // Compile the expression being asserted with proper ? operator
        let expr_value = self.compile_expression(type_assertion.expression.as_ref())?;
        
        debug!("Compiled expression value of type: {:?}", expr_value.get_type());
        
        // Create basic blocks for success and failure paths
        let success_block = self.context().append_basic_block(current_fn, "type_assert_success");
        let failure_block = self.context().append_basic_block(current_fn, "type_assert_failure");
        let merge_block = self.context().append_basic_block(current_fn, "type_assert_merge");
        
        // Check if the source is an interface type
        let is_interface = self.is_interface_type(&type_assertion.expression.node_type())?;
        
        if !is_interface {
            return Err(Error::Compilation(format!(
                "Type assertion can only be used on interface values, but {} is not an interface",
                type_assertion.expression.string()
            )));
        }
        
        // Use PathVisualization if available to get more detailed type information
        let type_path = if let Ok(path) = self.find_interface_type_path(
            &type_assertion.expression.node_type(),
            &type_assertion.type_name
        ) {
            Some(path)
        } else {
            None
        };
        
        // Check if the interface value is of the target type with proper error propagation
        let is_instance = self.check_instance_of_with_propagation(
            expr_value, 
            &type_assertion.type_name,
            &source_location
        )?;
        
        // Branch based on the type check result
        let condition_value = is_instance.into_int_value(self.context());
        self.builder().build_conditional_branch(
            condition_value,
            success_block,
            failure_block
        )?;
        
        // Success path - extract and cast the data pointer
        self.builder().position_at_end(success_block);
        let data_ptr = self.extract_interface_data_with_propagation(
            expr_value, 
            &source_location
        )?;
        
        // Look up the target type and create appropriate pointer type
        let target_struct_type = self
            .get_type_by_name(&type_assertion.type_name)
            .unwrap_or_else(|| self.context().opaque_struct_type(&type_assertion.type_name));
        
        let target_ptr_type = target_struct_type.ptr_type(AddressSpace::default());
        
        // Cast the data pointer to the concrete type pointer with ? operator
        let casted_ptr = self.builder().build_bitcast(
            data_ptr,
            target_ptr_type,
            "casted_ptr"
        )?;
        
        // Pack the result into a tuple structure
        let true_val = self.context().bool_type().const_int(1, false);
        let success_result = self.build_tuple(vec![casted_ptr.into(), true_val.into()])?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)?;
        
        // Failure path - return null pointer and false flag
        self.builder().position_at_end(failure_block);
        
        // Add enhanced error logging with path information if available
        if let Some(path) = &type_path {
            debug!(
                "Type assertion failed: {} is not of type {}. Path information: {}", 
                type_assertion.expression.string(), 
                type_assertion.type_name,
                path
            );
            
            // Emit runtime diagnostics in debug builds
            if cfg!(debug_assertions) {
                self.emit_type_assertion_debug_info(
                    &type_assertion.expression.string(),
                    &type_assertion.type_name,
                    path,
                    &source_location
                )?;
            }
        } else {
            debug!(
                "Type assertion failed: {} is not of type {}", 
                type_assertion.expression.string(), 
                type_assertion.type_name
            );
        }
        
        let null_ptr = target_ptr_type.const_null();
        let false_val = self.context().bool_type().const_int(0, false);
        let failure_result = self.build_tuple(vec![null_ptr.into(), false_val.into()])?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)?;
        
        // Merge block - use phi node to select the appropriate result
        self.builder().position_at_end(merge_block);
        
        // Create appropriate tuple type for the phi node
        let result_type = self.tuple_type(vec![
            target_ptr_type.into(), 
            self.context().bool_type().into()
        ]);
        
        let phi = self.builder().build_phi(
            result_type,
            "assertion_result"
        )?;
        
        phi.add_incoming(&[(
            &success_result,
            success_block
        ), (
            &failure_result,
            failure_block
        )]);
        
        debug!("Completed type assertion compilation with result type: {:?}", result_type);
        
        // Return the phi result
        Ok(phi.as_basic_value())
    }
    
    fn convert_type_assertion_error(&self, error: Error, context: &str) -> Error {
        match error {
            Error::Compilation(msg) => {
                Error::Compilation(format!("{} in {}", msg, context))
            },
            Error::Runtime(msg) => {
                Error::Runtime(format!("{} in {}", msg, context))
            },
            _ => error
        }
    }
    
    fn collect_type_assertion_errors(&mut self, errors: Vec<Error>) -> Error {
        if errors.is_empty() {
            return Error::Compilation("Unknown type assertion error".to_string());
        }
        
        if errors.len() == 1 {
            return errors[0].clone();
        }
        
        // Combine multiple errors into a single detailed error
        let mut combined = String::new();
        combined.push_str("Multiple type assertion errors occurred:\n");
        
        for (i, err) in errors.iter().enumerate() {
            match err {
                Error::Compilation(msg) | Error::Runtime(msg) => {
                    combined.push_str(&format!("  {}. {}\n", i + 1, msg));
                },
                _ => {
                    combined.push_str(&format!("  {}. Unknown error\n", i + 1));
                }
            }
        }
        
        Error::Compilation(combined)
    }
    
    fn create_type_assertion_error_report(
        &self,
        interface_type: &str,
        target_type: &str,
        source_location: &str,
        errors: &[Error]
    ) -> String {
        let mut report = format!(
            "Type Assertion Error Report:\n\n"
        );
        
        report.push_str(&format!("Location: {}\n", source_location));
        report.push_str(&format!("Attempted conversion: {} -> {}\n\n", interface_type, target_type));
        
        // Find any available type path information
        if let Ok(path) = self.find_interface_type_path(interface_type, target_type) {
            report.push_str(&format!("Type path information: {}\n\n", path));
        } else {
            report.push_str("No type path information available.\n\n");
        }
        
        // Add errors if any
        if !errors.is_empty() {
            report.push_str("Errors encountered:\n");
            
            for (i, err) in errors.iter().enumerate() {
                match err {
                    Error::Compilation(msg) | Error::Runtime(msg) => {
                        report.push_str(&format!("  {}. {}\n", i + 1, msg));
                    },
                    _ => {
                        report.push_str(&format!("  {}. Unknown error\n", i + 1));
                    }
                }
            }
        }
        
        report
    }
}

// Helper method to emit runtime debug information for type assertions
// (This would be implemented if needed in a real system)
impl<'ctx> LlvmCodeGenerator<'ctx> {
    fn emit_type_assertion_debug_info(
        &mut self,
        source_type: &str,
        target_type: &str,
        path_info: &str,
        source_location: &str
    ) -> Result<(), Error> {
        // This is a placeholder for actual implementation that would emit
        // runtime diagnostic information for failed type assertions
        
        // In a real implementation, this would generate LLVM code to:
        // 1. Format a debug message with all the type information
        // 2. Call a runtime function to log the message
        // 3. Potentially trigger breakpoints or other debugging aids
        
        trace!("Would emit debug info for type assertion: {} -> {} ({})", 
               source_type, target_type, source_location);
        
        Ok(())
    }
}

// Register this module in the compiler's initialization
pub fn register_type_assertion_result_integration() {
    trace!("Type assertion result integration module registered");
}