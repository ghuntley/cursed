//! Integration test for interface type assertion error propagation with filesystem integration
//!
//! This test verifies that the enhanced error propagation system properly uses
//! filesystem source location tracking to provide detailed error messages with
//! source code context when interface type assertions with the ? operator fail.

use std::sync::Arc;
use std::path::PathBuf;

// Import the necessary modules and traits
use cursed::ast::expressions::{TypeAssertion, TypeAssertionQuestion};
use cursed::ast::traits::Node;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::interface_type_assertion_error_propagation::InterfaceTypeAssertionErrorPropagation;
use cursed::codegen::llvm::interface_type_assertion_filesystem_integration::FilesystemSourceLocationIntegration;
use cursed::codegen::llvm::interface_type_assertion_error_propagation_filesystem::EnhancedErrorPropagationWithFilesystem;
use cursed::error::SourceLocation;
use cursed::error::type_assertion_error::TypeAssertionError;

// Import common test utilities
#[path = "common.rs"]
mod common;

#[test]
fn test_interface_type_assertion_filesystem_error_propagation() {
    // Set up tracing for this test
    common::tracing::setup();
    
    // Create an LLVM context and module for testing
    let context = inkwell::context::Context::create();
    let module = context.create_module("test");
    
    // Create an LlvmCodeGenerator with enhanced filesystem integration
    let mut code_gen = LlvmCodeGenerator::new(&context, module, None);
    
    // Initialize filesystem integration with the tests directory as root
    code_gen.init_filesystem_integration(Some("./tests"));
    
    // Add example files to the search paths
    code_gen.add_source_search_path("./examples");
    
    // Create a mock AST node for testing
    let type_assertion = TypeAssertionQuestion {
        token: "test.csd:10:15".to_string(),
        expression: Box::new(MockExpression { token: "someInterface".to_string() }),
        type_name: "ExpectedType".to_string(),
    };
    
    // Create an enhanced source location
    let result = code_gen.create_enhanced_source_location(
        &type_assertion as &dyn Node,
        Some("./examples/interface_type_assertion_question_op.csd")
    );
    
    assert!(result.is_ok(), "Failed to create enhanced source location");
    let source_location = result.unwrap();
    
    // Verify that the source location has file context
    assert!(source_location.file.is_some(), "Source location should have a file path");
    assert!(source_location.source_line.contains("Source"), "Source location should contain source context");
    
    // Test creating an enhanced error message
    let error_message = code_gen.create_enhanced_error_message(
        &type_assertion,
        "ExpectedType",
        Some("ActualType")
    ).unwrap();
    
    // Verify that the error message contains type information
    assert!(error_message.contains("ExpectedType"), "Error message should contain expected type");
    assert!(error_message.contains("ActualType"), "Error message should contain actual type");
    assert!(error_message.contains("expression"), "Error message should contain expression context");
    
    // Test formatting an error with source context
    let formatted_error = code_gen.format_error_with_source_context(
        "Type assertion failed",
        &source_location,
        2 // Include 2 lines of context
    ).unwrap();
    
    // Verify that the formatted error includes source context
    assert!(formatted_error.contains("Type assertion failed"), "Formatted error should contain the error message");
    assert!(formatted_error.contains("Source"), "Formatted error should contain source context");
    assert!(formatted_error.contains("at "), "Formatted error should contain location information");
    
    // Test with a real example file
    if let Ok(example_location) = code_gen.create_source_location_with_context(
        &type_assertion as &dyn Node,
        10, // Line
        15, // Column
        Some("./examples/interface_type_assertion_question_op.csd"),
        2   // Context lines
    ) {
        // Verify that we got source context from the example file
        assert!(example_location.source_line.contains("line"), "Should contain line indicators");
        assert!(example_location.source_line.contains(">"), "Should mark the current line");
        assert!(example_location.source_line.contains("^"), "Should mark the error column");
    }
}

// Mock expression for testing
struct MockExpression {
    token: String,
}

impl Node for MockExpression {
    fn token_literal(&self) -> String {
        self.token.clone()
    }
    
    fn string(&self) -> String {
        self.token.clone()
    }
}

impl cursed::ast::traits::Expression for MockExpression {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn cursed::ast::traits::Expression> {
        Box::new(MockExpression { token: self.token.clone() })
    }
    
    fn node_type(&self) -> &str {
        "MockExpression"
    }
}

#[test]
fn test_interface_filesystem_error_propagation_integration() {
    // Set up tracing for this test
    common::tracing::setup();
    
    // Create a timer to benchmark the operation
    let _timer = common::timing::Timer::new("filesystem_error_propagation_integration");
    
    // Create an LLVM context and module for testing
    let context = inkwell::context::Context::create();
    let module = context.create_module("test");
    
    // Create an LlvmCodeGenerator with enhanced filesystem integration
    let mut code_gen = LlvmCodeGenerator::new(&context, module, None);
    
    // Initialize filesystem integration
    code_gen.ensure_filesystem_integration_initialized();
    
    // Create a mock AST node for testing
    let type_assertion = TypeAssertionQuestion {
        token: "interface_type_assertion_question_op.csd:98:20".to_string(),
        expression: Box::new(MockExpression { token: "shape".to_string() }),
        type_name: "Circle".to_string(),
    };
    
    // Set current file path to help with resolution
    code_gen.set_current_file_path("./examples/interface_type_assertion_question_op.csd");
    
    // Create an enhanced source location
    let source_location = code_gen.create_enhanced_source_location(
        &type_assertion as &dyn Node,
        Some("./examples/interface_type_assertion_question_op.csd")
    ).unwrap();
    
    // Verify source location has relevant information
    assert_eq!(source_location.line, 98, "Line number should be extracted from token");
    assert_eq!(source_location.column, 20, "Column number should be extracted from token");
    assert!(source_location.file.is_some(), "File path should be set");
    
    // Test creating an enhanced error message with full context
    let error_message = code_gen.format_error_with_source_context(
        "Type assertion failed: cannot convert from Shape to Circle",
        &source_location,
        2 // Context lines
    ).unwrap();
    
    // Verify the error message has all the expected components
    assert!(error_message.contains("Type assertion failed"), "Should contain error message");
    assert!(error_message.contains("Shape"), "Should contain source type");
    assert!(error_message.contains("Circle"), "Should contain target type");
    assert!(error_message.contains("Source"), "Should contain source context section");
    
    // Verify source code excerpts if file exists
    if std::path::Path::new("./examples/interface_type_assertion_question_op.csd").exists() {
        assert!(error_message.contains("line"), "Should show line numbers");
        assert!(error_message.contains("|"), "Should have line separator");
    }
}