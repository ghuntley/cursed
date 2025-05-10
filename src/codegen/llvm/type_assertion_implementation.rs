//! # Interface Type Assertion Implementation
//! 
//! This module implements the integration between interface type assertions
//! and the LLVM code generator. It combines the different implementations
//! into a single coherent approach with proper error handling.
//!
//! The implementation provides:
//! 1. Proper error propagation with structured logging
//! 2. Null checking for interface values and vtables
//! 3. Integration with the interface implementation system
//! 4. Consistent return value handling
//! 5. Support for both success and failure paths
//! 6. Nesting level tracking for complex interface hierarchies

use inkwell::values::{BasicValueEnum, PointerValue};
use inkwell::AddressSpace;
use crate::ast::expressions::TypeAssertion;
use crate::error::Error;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::interface_type_assertion_errors::TypeAssertionErrorHandler;
use crate::codegen::llvm::interface_type_assertion_nesting::{NestedTypeAssertion, TypeAssertionNestingContext};
use crate::core::type_checker::Type as CursedType;
use crate::codegen::llvm::interface_implementation::InterfaceImplementation;
use crate::codegen::llvm::expression::ExpressionCompilation;

use tracing::{debug, error, info, instrument, trace, warn, Level};

/// Trait for implementing the integrated interface type assertion functionality
pub trait IntegratedTypeAssertion<'ctx> {
    /// Main entry point for type assertions in the compiler
    fn compile_integrated_type_assertion(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Convert a successful type assertion result into a usable value
    fn extract_assertion_success_value(
        &mut self,
        result: BasicValueEnum<'ctx>,
        target_type: &str
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Convert a type assertion result into a tuple of value and success flag
    fn create_type_assertion_result(
        &mut self,
        value: Option<BasicValueEnum<'ctx>>,
        success: bool
    ) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> IntegratedTypeAssertion<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn compile_integrated_type_assertion(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling integrated type assertion for {}", type_assertion.type_name);
        
        // Check if we should use nesting tracking, which is determined by environment variables
        let use_nesting = std::env::var("CURSED_TYPE_DEBUG")
            .or_else(|_| std::env::var("CURSED_DEBUG"))
            .map(|val| !val.is_empty() && val != "0" && val.to_lowercase() != "false")
            .unwrap_or(false);
            
        if use_nesting {
            // Use the nested type assertion implementation with proper tracking
            return self.compile_nested_type_assertion(type_assertion, None);
        }
        
        // First compile the expression being asserted to check for initial errors
        let expr_value = match ExpressionCompilation::compile_expression(self, type_assertion.expression.as_ref()) {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to compile expression for type assertion: {}", e);
                return Err(Error::Compilation(format!(
                    "Failed to compile expression for type assertion '{}': {}", 
                    type_assertion.expression.string(), e
                )));
            }
        };
        
        // Check if the expression is null before proceeding
        if expr_value.is_pointer_value() {
            let ptr = expr_value.into_pointer_value();
            if let Ok(is_null) = self.builder().build_is_null(ptr, "ptr_null_check") {
                // Check if the is_null value is a constant true
                if is_null.is_int_value() {
                    let int_val = is_null.into_int_value();
                    if int_val.get_zero_extended_value() != 0 {
                        error!("Type assertion attempted on null interface value");
                        // Return a tuple with null value and false flag to indicate failure
                        return self.create_type_assertion_result(None, false);
                    }
                }
            }
        }
        
        // Use the error-handling implementation which is the most complete
        let result = match TypeAssertionErrorHandler::compile_type_assertion_with_errors(self, type_assertion) {
            Ok(val) => val,
            Err(e) => {
                error!("Type assertion error: {}", e);
                return Err(Error::Compilation(format!(
                    "Type assertion failed for type '{}': {}", 
                    type_assertion.type_name, e
                )));
            }
        };
        
        debug!("Type assertion compiled successfully");
        Ok(result)
    }
    
    #[instrument(skip(self, result), level = "debug")]
    fn extract_assertion_success_value(
        &mut self,
        result: BasicValueEnum<'ctx>,
        target_type: &str
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Extracting value from successful type assertion");
        
        // The result is a tuple of (value, success)
        // Extract the value (first element)
        if result.is_struct_value() {
            let value = self.builder().build_extract_value(
                result.into_struct_value(),
                0,
                "extracted_value"
            ).map_err(|e| {
                error!("Failed to extract value from type assertion result: {}", e);
                Error::Compilation(format!("Failed to extract value from type assertion result: {}", e))
            })?;
            
            debug!("Successfully extracted value from type assertion");
            Ok(value)
        } else {
            error!("Type assertion result is not a struct: {:?}", result);
            Err(Error::Compilation(format!(
                "Expected struct result from type assertion, got {:?}", result
            )))
        }
    }
    
    #[instrument(skip(self, value), level = "debug")]
    fn create_type_assertion_result(
        &mut self,
        value: Option<BasicValueEnum<'ctx>>,
        success: bool
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Creating type assertion result tuple");
        
        // Create a default value if none was provided
        let value = value.unwrap_or_else(|| {
            self.context().i8_type().ptr_type(AddressSpace::default()).const_null().into()
        });
        
        // Create success flag
        let success_val = self.context().bool_type().const_int(if success { 1 } else { 0 }, false);
        
        // Build the tuple (value, success)
        self.build_tuple(vec![value, success_val.into()])
    }
}

// Helper methods for LlvmCodeGenerator for type assertions
impl<'ctx> LlvmCodeGenerator<'ctx> {
    // Uses the existing is_null_pointer from pointer_ops.rs
}

// Register the module in the LLVM code generator
pub fn register_type_assertion_implementation() {
    trace!("Registering integrated type assertion implementation");
    // This function is called during LlvmCodeGenerator initialization
    // and ensures that the type assertion implementation is properly registered
    // with the compiler. This enables proper interface type assertions.
    
    // Also register the nested type assertion implementation
    crate::codegen::llvm::interface_type_assertion_nesting::register_nested_type_assertion();
}