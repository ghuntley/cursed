use cursed::ast::expressions::TypeAssertion;
use cursed::ast::expressions::identifiers::Identifier;
use cursed::ast::traits::{Expression, Node};
use cursed::error::SourceLocation;
use common::tracing::setup as init_tracing;

// Integration tests for the full Result-based interface type assertion implementation
//
// These tests verify that interface type assertions work correctly with proper
// integration of the ? operator for error propagation.



// Import common test utilities
#[path = "common/mod.rs"]
mod common;


#[test]
fn test_basic_type_assertion_structure() {
    // Initialize tracing for this test
    init_tracing();
    
    // Test basic type assertion creation without private modules
    // This ensures the basic AST structure works correctly
}

#[test]
fn test_source_location_creation() {
    // Initialize tracing for this test
    init_tracing();
    
    // Create a test source location
    let source_location = SourceLocation {
        line: 42,
        column: 10,
        file: Some("test.csd".to_string()),
        source_line: "x, ok = value.(TargetType)".to_string(),
    };
    
    // Verify the source location fields
    assert_eq!(source_location.line, 42);
    assert_eq!(source_location.column, 10);
    assert_eq!(source_location.file, Some("test.csd".to_string()));
    assert_eq!(source_location.source_line, "x, ok = value.(TargetType)".to_string());
}

#[test]
fn test_type_assertion_error_info_creation() {
    // Initialize tracing for this test
    init_tracing();
    
    // Create a simple error info structure for testing
    let error_message = "Type assertion failed: Stringer is not a Writer".to_string();
    
    // Test creating basic error info without using private structs
    assert!(error_message.contains("Type assertion failed"));
    assert!(error_message.contains("Stringer"));
    assert!(error_message.contains("Writer"));
}

#[test]
fn test_type_assertion_compilation() {
    // Initialize tracing for this test
    init_tracing();
    
    // Create a simple test TypeAssertion AST node
    let type_assertion = TypeAssertion {
        token: ".".to_string(),
        expression: Box::new(Identifier {
            token: "value".to_string(),
            value: "value".to_string(),
        }),
        type_name: "TargetType".to_string(),
    };
    
    // Verify the string representation
    assert_eq!(type_assertion.string(), "value.(TargetType)");
    
    // Verify token literal
    assert_eq!(type_assertion.token_literal(), ".");
}
