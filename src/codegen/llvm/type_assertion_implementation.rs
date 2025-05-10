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

use inkwell::values::{BasicValueEnum, PointerValue};
use inkwell::AddressSpace;
use crate::ast::expressions::TypeAssertion;
use crate::error::Error;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::interface_type_assertion_errors::TypeAssertionErrorHandler;
use crate::core::type_checker::Type as CursedType;
use crate::codegen::llvm::interface_implementation::InterfaceImplementation;

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
        
        // Use the error-handling implementation which is the most complete
        let result = self.compile_type_assertion_with_errors(type_assertion)?;
        
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

// Register the module in the LLVM code generator
pub fn register_type_assertion_implementation() {
    trace!("Registering integrated type assertion implementation");
}