//! # Enhanced Interface Type Assertion Module
//!
//! This module aggregates all the enhanced interface type assertion implementations
//! and provides a unified interface for the compiler.

pub mod interface_type_assertion_error_propagation_enhanced;
pub mod interface_type_assertion_enhanced_integration;

pub use interface_type_assertion_error_propagation_enhanced::EnhancedInterfaceTypeAssertionErrorPropagation;
pub use interface_type_assertion_error_propagation_enhanced::register_enhanced_error_propagation;
pub use interface_type_assertion_enhanced_integration::EnhancedTypeAssertionIntegration;
pub use interface_type_assertion_enhanced_integration::register_enhanced_type_assertion_integration;

/// Initialize the enhanced interface type assertion system
pub fn initialize_enhanced_type_assertion() {
    register_enhanced_error_propagation();
    register_enhanced_type_assertion_integration();
}