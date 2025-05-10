//! # Improved Interface Type Assertion Integration
//!
//! This module provides enhanced integration between the interface type assertion
//! system and the expression compiler, with proper error propagation using the `?`
//! operator and consistent error handling throughout.
//!
//! The implementation provides:
//! 1. Consistent error propagation with the `?` operator
//! 2. Unified interface for all type assertion implementations
//! 3. Proper error context and message formatting
//! 4. Structured logging with span context
//! 5. Automatic selection of the appropriate implementation based on context

use inkwell::values::BasicValueEnum;
use crate::ast::expressions::TypeAssertion;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::error::Error;
use crate::codegen::llvm::interface_type_assertion_errors::TypeAssertionErrorHandler;
use crate::codegen::llvm::interface_type_assertion_debugging::RuntimeTypeAssertionDebugging;
use crate::codegen::llvm::interface_type_assertion_nesting::NestedTypeAssertion;
use crate::codegen::llvm::type_assertion_implementation::IntegratedTypeAssertion;

use tracing::{debug, error, info, instrument, span, warn, Level};

/// Enhanced interface for type assertions with proper error propagation
/// 
/// This trait provides a unified interface for all type assertion implementations
/// with consistent error handling and proper propagation using the `?` operator.
pub trait ImprovedTypeAssertionIntegration<'ctx> {
    /// Compile a type assertion with enhanced error propagation
    /// 
    /// This method automatically selects the appropriate implementation based on
    /// the context and debugging settings, ensuring consistent error handling.
    fn compile_type_assertion_with_propagation(
        &mut self,
        type_assertion: &TypeAssertion,
        source_loc: Option<&str>
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Check if debugging is enabled for type assertions
    fn is_type_assertion_debugging_enabled(&self) -> bool;
    
    /// Check if nesting tracking is enabled for type assertions
    fn is_type_assertion_nesting_enabled(&self) -> bool;
    
    /// Extract the type assertion result for consumption
    /// 
    /// This method extracts the actual value from a type assertion result,
    /// propagating errors if the assertion failed.
    fn extract_and_verify_type_assertion_result(
        &mut self,
        result: BasicValueEnum<'ctx>,
        target_type: &str
    ) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> ImprovedTypeAssertionIntegration<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn compile_type_assertion_with_propagation(
        &mut self,
        type_assertion: &TypeAssertion,
        source_loc: Option<&str>
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Compiling type assertion with propagation for {}", type_assertion.type_name);
        
        // Check which implementation to use based on environment variables and context
        if self.is_type_assertion_debugging_enabled() {
            debug!("Using debugging implementation for type assertion");
            // Use the debugging implementation
            return self.compile_type_assertion_with_debugging(type_assertion, source_loc);
        }
        
        if self.is_type_assertion_nesting_enabled() {
            debug!("Using nesting implementation for type assertion");
            // Use the nesting implementation
            return self.compile_nested_type_assertion(type_assertion, None);
        }
        
        // Use the integrated implementation as the default
        debug!("Using integrated implementation for type assertion");
        self.compile_integrated_type_assertion(type_assertion)
    }
    
    fn is_type_assertion_debugging_enabled(&self) -> bool {
        // Check environment variables for debugging settings
        std::env::var("CURSED_TYPE_DEBUG")
            .or_else(|_| std::env::var("CURSED_DEBUG"))
            .map(|val| !val.is_empty() && val != "0" && val.to_lowercase() != "false")
            .unwrap_or(false)
    }
    
    fn is_type_assertion_nesting_enabled(&self) -> bool {
        // Check environment variables for nesting settings
        // Usually this is the same as debugging, but could be different in the future
        std::env::var("CURSED_TYPE_NESTING")
            .map(|val| !val.is_empty() && val != "0" && val.to_lowercase() != "false")
            .unwrap_or(self.is_type_assertion_debugging_enabled())
    }
    
    #[instrument(skip(self, result), level = "debug")]
    fn extract_and_verify_type_assertion_result(
        &mut self,
        result: BasicValueEnum<'ctx>,
        target_type: &str
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Extracting and verifying type assertion result for {}", target_type);
        
        // The result is a tuple of (value, success)
        if !result.is_struct_value() {
            error!("Type assertion result is not a struct: {:?}", result);
            return Err(Error::Compilation(format!(
                "Type assertion result has unexpected type for '{}'", target_type
            )));
        }
        
        // Extract the success flag (second element)
        let success = self.builder().build_extract_value(
            result.into_struct_value(),
            1, // Success flag is the second element
            "success_flag"
        ).map_err(|e| {
            error!("Failed to extract success flag: {}", e);
            Error::Compilation(format!(
                "Failed to extract success flag from type assertion for '{}': {}", 
                target_type, e
            ))
        })?;
        
        // Check if the assertion succeeded
        if !success.is_int_value() {
            error!("Success flag has unexpected type: {:?}", success);
            return Err(Error::Compilation(format!(
                "Success flag has unexpected type for '{}'", target_type
            )));
        }
        
        // Convert success flag to a boolean
        let success_bool = success.into_int_value().get_zero_extended_constant();
        
        // If assertion failed, return an error
        if let Some(0) = success_bool {
            debug!("Type assertion failed for {}", target_type);
            return Err(Error::Compilation(format!(
                "Type assertion failed: value is not of type '{}'", target_type
            )));
        }
        
        // Assertion succeeded, extract the value (first element)
        let value = self.builder().build_extract_value(
            result.into_struct_value(),
            0, // Value is the first element
            "extracted_value"
        ).map_err(|e| {
            error!("Failed to extract value: {}", e);
            Error::Compilation(format!(
                "Failed to extract value from type assertion for '{}': {}", 
                target_type, e
            ))
        })?;
        
        debug!("Successfully extracted value from type assertion");
        Ok(value)
    }
}

/// Register the improved type assertion integration
pub fn register_improved_type_assertion_integration() {
    debug!("Registering improved type assertion integration");
    // This registration will be called during LlvmCodeGenerator initialization
}