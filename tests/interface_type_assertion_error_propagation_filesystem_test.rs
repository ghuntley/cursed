use std::path::PathBuf;
use std::sync::Arc;
use std::cell::RefCell;
use inkwell::context::Context;
use tracing::{debug, info};
use cursed::ast::expressions::{TypeAssertion, TypeAssertionQuestion};
use cursed::ast::traits::{Expression, Node};
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::interface_type_assertion_error_propagation_filesystem::EnhancedErrorPropagationWithFilesystem;
use cursed::codegen::llvm::interface_type_assertion_error_propagation_filesystem_integration::ComprehensiveErrorFilesystemIntegration;
use cursed::codegen::llvm::interface_type_assertion_error_visualization::ErrorVisualization;
use cursed::error::Error;
use cursed::error::SourceLocation;
use cursed::lexer::Token;

// Tests for the comprehensive interface type assertion error propagation with filesystem integration
//
// These tests ensure that the error propagation for interface type assertions with the ? operator
// effectively leverages filesystem source location information to generate rich error messages.


#[path = "common.rs"]
mod common;



// Initialize tracing for tests
fn init_tracing() {
    let _ = common::tracing::setup();
}

#[test]
fn test_comprehensive_error_propagation_initialization() {
    init_tracing();
    info!("Running test_comprehensive_error_propagation_initialization");
    
    // Create LLVM context and code generator
    let context = Context::create();
    let mut code_generator = LlvmCodeGenerator::new(&context);
    
    // Initialize comprehensive error filesystem integration
    code_generator.init_comprehensive_error_filesystem_integration();
    
    // Verify internal fields are set correctly
    let initialized = code_generator.internal_fields.get("comprehensive_error_fs_integration_initialized")
        .and_then(|boxed| boxed.downcast_ref::<bool>())
        .cloned()
        .unwrap_or(false);
    
    assert!(initialized, "Comprehensive error filesystem integration should be initialized");
}

#[test]
fn test_comprehensive_error_message_creation() {
    init_tracing();
    info!("Running test_comprehensive_error_message_creation");
    
    // Create LLVM context and code generator
    let context = Context::create();
    let mut code_generator = LlvmCodeGenerator::new(&context);
    
    // Initialize comprehensive error filesystem integration
    code_generator.init_comprehensive_error_filesystem_integration();
    
    // Create a mock type assertion question
    let expr = MockExpression::new("interface_value".to_string());
    let type_assertion = TypeAssertionQuestion {
        token: "token".to_string(),
        expression: Box::new(expr),
        type_name: "ExpectedType".to_string(),
    };
    
    // Create a source location
    let source_location = SourceLocation {
        line: 42,
        column: 10,
        file: Some("test_file.csd".to_string(),
        source_line: "interface_value.(ExpectedType)?".to_string(),
    };
    
    // Create a comprehensive error message
    let error_message = code_generator.create_comprehensive_error_message(
        &type_assertion,
        "ExpectedType",
        Some("ActualType"),
        &source_location
    ).unwrap();
    
    // Check that the error message contains expected information
    assert!(error_message.contains("Type assertion failed"), "Error message should mention type assertion failure");
    assert!(error_message.contains("ExpectedType"), "Error message should contain expected type");
    assert!(error_message.contains("ActualType"), "Error message should contain actual type");
    assert!(error_message.contains("test_file.csd"), "Error message should contain file path");
    assert!(error_message.contains("42"), "Error message should contain line number");
    assert!(error_message.contains("10"), "Error message should contain column number");
}

#[test]
fn test_visual_error_formatting() {
    init_tracing();
    info!("Running test_visual_error_formatting");
    
    // Create LLVM context and code generator
    let context = Context::create();
    let mut code_generator = LlvmCodeGenerator::new(&context);
    
    // Initialize comprehensive error filesystem integration
    code_generator.init_comprehensive_error_filesystem_integration();
    
    // Create a mock type assertion question
    let expr = MockExpression::new("interface_value".to_string());
    let type_assertion = TypeAssertionQuestion {
        token: "token".to_string(),
        expression: Box::new(expr),
        type_name: "ExpectedType".to_string(),
    };
    
    // Create a source location
    let source_location = SourceLocation {
        line: 42,
        column: 10,
        file: Some("test_file.csd".to_string(),
        source_line: "interface_value.(ExpectedType)?".to_string(),
    };
    
    // Create context lines
    let context_lines = vec![
        (41, "    // Previous line\n".to_string(),
        (42, "    interface_value.(ExpectedType)? // Error line\n".to_string(),
        (43, "    // Next line\n".to_string(),
    ];
    
    // Create a visual error
    let visual_error = code_generator.create_visual_type_assertion_error(
        "Type assertion failed: cannot convert interface to ExpectedType",
        &source_location,
        "ExpectedType",
        Some("ActualType"),
        context_lines
    );
    
    // Format the visual error message
    let formatted_error = code_generator.format_visual_error_message(&visual_error);
    
    // Check that the formatted error contains expected information
    assert!(formatted_error.contains("Error"), "Formatted error should contain 'Error'");
    assert!(formatted_error.contains("ExpectedType"), "Formatted error should contain expected type");
    assert!(formatted_error.contains("ActualType"), "Formatted error should contain actual type");
    assert!(formatted_error.contains("test_file.csd"), "Formatted error should contain file path");
    assert!(formatted_error.contains("42"), "Formatted error should contain line number");
    assert!(formatted_error.contains("Error occurs here"), "Formatted error should indicate error location");
}

// Mock Expression implementation for testing
#[derive(Debug)]
struct MockExpression {
    value: String,
}

impl MockExpression {
    fn new(value: String) -> Self {
        Self { value }
    }
}

impl Node for MockExpression {
    fn token_literal(&self) -> String {
        "mock".to_string()
    }
    
    fn string(&self) -> String {
        self.value.clone()
    }
}

impl Expression for MockExpression {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(Self {
            value: self.value.clone(),
        })
    }
    
    fn node_type(&self) -> &str {
        "MockExpression"
    }
}