//! # Interface Type Assertion Integration
//!
//! This module integrates the enhanced interface type assertion implementation
//! with the main compiler pipeline. It connects the type assertion functionality
//! from `interface_type_assertion_errors.rs` with the statement compiler.
//!
//! This implementation provides:
//! 1. Proper error propagation
//! 2. Robust null checking
//! 3. Better error messages
//! 4. Structured logging for debugging

use inkwell::values::BasicValueEnum;
use crate::ast::expressions::TypeAssertion;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::error::Error;
use crate::codegen::llvm::interface_type_assertion_errors::TypeAssertionErrorHandler;

/// Extension trait for the LlvmCodeGenerator that provides the integration point
/// between the type assertion implementation and the main compiler pipeline.
pub trait TypeAssertionIntegration<'ctx> {
    /// Compile a type assertion expression and handle any errors appropriately.
    /// This is the main entry point for type assertions in the compiler pipeline.
    fn compile_type_assertion_integrated(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> TypeAssertionIntegration<'ctx> for LlvmCodeGenerator<'ctx> {
    #[tracing::instrument(skip(self), level = "debug")]
    fn compile_type_assertion_integrated(
        &mut self,
        type_assertion: &TypeAssertion
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        tracing::debug!("Compiling type assertion for type {}", type_assertion.type_name);
        
        // Call the implementation from interface_type_assertion_errors.rs
        // which has the most complete implementation with error handling
        let result = self.compile_type_assertion_with_errors(type_assertion)?;
        
        tracing::debug!("Type assertion compiled successfully");
        Ok(result)
    }
}

// Register the module in the codegen directory
pub fn register_type_assertion_integration() {
    // This function doesn't need to do anything at runtime,
    // it just ensures the module is included in the build
    tracing::trace!("Type assertion integration registered");
}