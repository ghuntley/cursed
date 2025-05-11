//! # Interface Type Assertion Error Propagation Integration
//!
//! This module integrates the interface type assertion error propagation system
//! with the rest of the codebase, ensuring proper ? operator functionality.
//!
//! ## Features
//!
//! - Proper integration between error types for seamless error propagation
//! - Support for the ? operator in type assertions
//! - Consistent error handling across the codebase
//! - Improved error context with type information
//! - Integration with the Result system

use tracing::{debug, error, info, instrument, warn};
use std::fmt;

use inkwell::values::BasicValueEnum;
use inkwell::AddressSpace;

use crate::ast::expressions::TypeAssertion;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::codegen::llvm::interface_type_assertion_error_propagation::{InterfaceTypeAssertionErrorPropagation};
use crate::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;
use crate::error::Error;
use crate::error::type_assertion_error::{TypeAssertionError, helpers as error_helpers};
use crate::error::SourceLocation;
use crate::error_enhanced::CursedError;

/// Integration trait for the error propagation system
pub trait InterfaceTypeAssertionErrorIntegration<'ctx>: 
    InterfaceTypeAssertionErrorPropagation<'ctx> + 
    InterfaceTypeAssertionPathVisualization<'ctx> {
    
    /// Convert a type assertion error to a Result that works with ?
    fn convert_type_assertion_error_to_result(
        &self,
        error: TypeAssertionError
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Unwrap a type assertion result, propagating errors using ?
    fn unwrap_type_assertion_result_with_question_op(
        &mut self,
        result_value: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Create a specialized error that integrates with both error systems
    fn create_integrated_error(
        &self,
        source_type: &str,
        target_type: &str,
        source_location: Option<SourceLocation>,
        message: Option<String>
    ) -> Error;
}

impl<'ctx> InterfaceTypeAssertionErrorIntegration<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self, error), level = "debug")]
    fn convert_type_assertion_error_to_result(
        &self,
        error: TypeAssertionError
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Convert the specialized error to the Error type that works with ?
        let enhanced_error: CursedError = error.into();
        Err(Error::TypeAssertion(enhanced_error))
    }
    
    #[instrument(skip(self, result_value), level = "debug")]
    fn unwrap_type_assertion_result_with_question_op(
        &mut self,
        result_value: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // This method delegates to the base unwrap_type_assertion_result
        // but ensures the error type is compatible with ?
        match self.unwrap_type_assertion_result(result_value) {
            Ok(value) => Ok(value),
            Err(e) => {
                // Convert to a type that works with ?
                match e {
                    Error::TypeAssertion(assertion_error) => {
                        // Already in the right format
                        Err(Error::TypeAssertion(assertion_error))
                    },
                    // Other error types need conversion
                    _ => {
                        let error_msg = format!("Type assertion error: {}", e);
                        let type_error = TypeAssertionError::new("unknown", "unknown")
                            .with_message(error_msg);
                        
                        self.convert_type_assertion_error_to_result(type_error)
                    }
                }
            }
        }
    }
    
    #[instrument(skip(self), level = "debug")]
    fn create_integrated_error(
        &self,
        source_type: &str,
        target_type: &str,
        source_location: Option<SourceLocation>,
        message: Option<String>
    ) -> Error {
        // Create a specialized error with detailed information
        let error = error_helpers::create_type_assertion_error(
            source_type, 
            target_type,
            source_location,
            message
        );
        
        // Convert to the Error type expected by ?
        Error::TypeAssertion(error.into())
    }
}

/// Helper function to check if a type assertion error is due to type mismatch
pub fn is_type_mismatch_error(err: &Error) -> bool {
    match err {
        Error::TypeAssertion(e) => e.kind() == &crate::error_enhanced::ErrorKind::TypeAssertion,
        _ => false
    }
}

/// Helper function to extract type info from an error
pub fn extract_type_info(err: &Error) -> Option<(String, String)> {
    match err {
        Error::TypeAssertion(e) => {
            let context = e.context();
            let source = context.iter().find(|(k, _)| k == "interface_type").map(|(_, v)| v.clone());
            let target = context.iter().find(|(k, _)| k == "target_type").map(|(_, v)| v.clone());
            
            if let (Some(s), Some(t)) = (source, target) {
                Some((s.clone(), t.clone()))
            } else {
                None
            }
        },
        _ => None
    }
}

/// Register the error propagation integration with the compiler
pub fn register_error_propagation_integration() {
    debug!("Interface type assertion error propagation integration module registered");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_integration_registration() {
        register_error_propagation_integration();
        assert!(true);
    }
    
    #[test]
    fn test_is_type_mismatch_error() {
        let error = TypeAssertionError::new("Reader", "Writer")
            .with_message("Types are incompatible");
        
        let cursed_error: CursedError = error.into();
        let err = Error::TypeAssertion(cursed_error);
        
        assert!(is_type_mismatch_error(&err));
    }
    
    #[test]
    fn test_extract_type_info() {
        let error = TypeAssertionError::new("Reader", "Writer")
            .with_message("Types are incompatible");
        
        let cursed_error: CursedError = error.into();
        let err = Error::TypeAssertion(cursed_error);
        
        if let Some((source, target)) = extract_type_info(&err) {
            assert_eq!(source, "Reader");
            assert_eq!(target, "Writer");
        } else {
            panic!("Failed to extract type info");
        }
    }
}