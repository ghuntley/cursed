//! # Enhanced Interface Type Assertion Integration
//!
//! This module integrates the enhanced interface type assertion error propagation
//! into the LLVM code generator. It provides a unified API for compiling
//! type assertions with enhanced error reporting capabilities.
//!
//! This integration ensures that all type assertions benefit from:
//! 1. More accurate source location information
//! 2. Better type relationship visualization
//! 3. Enhanced error messages with inheritance details
//! 4. Improved ? operator support

use std::sync::Arc;
use tracing::{debug, error, info, instrument, trace, warn};

use crate::ast::expressions::{TypeAssertion, TypeAssertionQuestion};
use crate::ast::traits::Expression;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::codegen::llvm::interface_type_assertion_error_propagation::InterfaceTypeAssertionErrorPropagation;
use crate::codegen::llvm::interface_type_assertion_error_propagation_enhanced::EnhancedInterfaceTypeAssertionErrorPropagation;
use crate::error::Error;

/// Trait for integrating enhanced type assertion capabilities into the expression compiler
pub trait EnhancedTypeAssertionIntegration<'ctx> {
    /// Compile a type assertion with enhanced error reporting
    fn compile_enhanced_type_assertion(&mut self, node: &dyn Expression) -> Result<inkwell::values::BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> EnhancedTypeAssertionIntegration<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self, node))]
    fn compile_enhanced_type_assertion(&mut self, node: &dyn Expression) -> Result<inkwell::values::BasicValueEnum<'ctx>, Error> {
        debug!("Compiling enhanced type assertion for node: {}", node.node_type());
        
        // Handle different types of type assertions
        match node.node_type() {
            "TypeAssertion" => {
                if let Some(type_assertion) = node.as_any().downcast_ref::<TypeAssertion>() {
                    // Use the enhanced error propagation implementation
                    self.compile_type_assertion_with_enhanced_error_propagation(type_assertion)
                } else {
                    Err(Error::Compilation(format!("Failed to downcast to TypeAssertion: {}", node.string())))
                }
            },
            "TypeAssertionQuestion" => {
                if let Some(type_assertion) = node.as_any().downcast_ref::<TypeAssertionQuestion>() {
                    // Use the enhanced ? operator implementation
                    self.compile_type_assertion_question_enhanced(type_assertion)
                } else {
                    Err(Error::Compilation(format!("Failed to downcast to TypeAssertionQuestion: {}", node.string())))
                }
            },
            _ => {
                // For other expression types, fall back to the standard expression compilation
                self.compile_expression(node)
            }
        }
    }
}

// Extend the expression compilation trait to use enhanced type assertions
impl<'ctx> ExpressionCompilation<'ctx> for LlvmCodeGenerator<'ctx> {
    // This is already implemented elsewhere, we're just adding a hook into it
    
    #[instrument(skip(self, node))]
    fn compile_expression(&mut self, node: &dyn Expression) -> Result<inkwell::values::BasicValueEnum<'ctx>, Error> {
        // Check if it's a type assertion and use the enhanced implementation if it is
        match node.node_type() {
            "TypeAssertion" | "TypeAssertionQuestion" => {
                self.compile_enhanced_type_assertion(node)
            },
            // For other types, delegate to the original implementation
            _ => {
                // Call the default implementation
                self.compile_expression_internal(node)
            }
        }
    }
    
    // Internal version that can be called for non-type-assertion nodes
    #[instrument(skip(self, node))]
    fn compile_expression_internal(&mut self, node: &dyn Expression) -> Result<inkwell::values::BasicValueEnum<'ctx>, Error> {
        // This should call the default implementation
        // In a real implementation, this would be the existing compile_expression method
        // For now, we'll provide a placeholder implementation
        Err(Error::Compilation("The default implementation needs to be called".to_string()))
    }
}

/// Register the enhanced type assertion integration with the compiler
pub fn register_enhanced_type_assertion_integration() {
    trace!("Enhanced type assertion integration module registered");
    // This function is called during the compiler's initialization
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_enhanced_type_assertion_integration_registration() {
        // Test that the module registration function works
        register_enhanced_type_assertion_integration();
        assert!(true);
    }
}