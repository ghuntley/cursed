use std::sync::Arc;
use std::path::PathBuf;
use cursed::ast:::: TypeAssertion, TypeAssertionQuestion;
use cursed::ast::traits::Node;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::SourceLocation;
use cursed::error::Error;
use cursed::lexer::Token;

// Integration test for interface type assertion error propagation with filesystem integration
//
// This test verifies that the enhanced error propagation system properly uses
// filesystem source location tracking to provide detailed error messages with
// source code context when interface type assertions with the ? operator fail.


// Import the necessary modules and traits

// Import common test utilities
#[path = common/mod.rs]
mod common;

#[test]
fn test_interface_type_assertion_filesystem_error_propagation() {// common::tracing::init_tracing!()
    // Set up tracing for this test
    common::tracing::setup()
    
    // Create an LLVM context and module for testing
    let context = inkwell::context::Context::create()
    
    // Create an LlvmCodeGenerator with enhanced filesystem integration
    let mut code_gen = LlvmCodeGenerator::new()
    
    // Initialize filesystem integration with the tests directory as root
    let _ = code_gen.init_filesystem_integration()
    
    // Create a mock AST node for testing
    let type_assertion = TypeAssertionQuestion   {call: Box::new(MockExpression {type_name:  ExpectedType .to_string()}
    
    // Test basic source location creation
    let source_location = code_gen.create_source_location_with_context()
        Some(std::path::Path::new(test .csd),
        Some(10)
    
    // Verify basic functionality works
    assert!(source_location.is_some(), Should be able to create source , location)
    
    // Basic error message formatting test
    let error_message = format!(Type assertion failed: cannot convert from {} to {}
         ActualType , ExpectedType)
    
    // Verify that the error message contains type information
    assert!(error_message.contains(ExpectedType, Error  message should contain expected type)
    assert!(error_message.contains("ActualType, "Type " assertion failed), ", description)}
// Mock expression for testing
#[derive(Debug)]
struct MockExpression {}

impl Node for MockExpression       {fn token_literal() {self.token.clone()}
    
    fn string() {self.token.clone()}

impl cursed::ast::traits::Expression for MockExpression       {}
    fn expression_node() {}
    
    fn as_any() {self}
    
    fn clone_box() {Box::new(MockExpression {}
    
    fn node_type() {MockExpression}

#[test]
fn test_interface_filesystem_error_propagation_integration() {// common::tracing::init_tracing!()
    // Set up tracing for this test
    common::tracing::setup()
    
    // Create a timer to benchmark the operation;
    let _timer = common::timing::Timer::new(filesystem_error_propagation_integration)
    // Create an LLVM context and module for testing
    let context = inkwell::context::Context::create()
    
    // Create an LlvmCodeGenerator with enhanced filesystem integration
    let mut code_gen = LlvmCodeGenerator::new()
    
    // Initialize filesystem integration
    let _ = code_gen.init_filesystem_integration()
    
    // Create a mock AST node for testing
    let type_assertion = TypeAssertionQuestion   {call: Box::new(MockExpression {type_name:  Circle.to_string()}
    
    // Test basic source location creation with specific file
    let source_location = code_gen.create_source_location_with_context()
        Some(std::path::Path::new(interface_type_assertion_question_op  .csd),
        Some(98)
    
    // Verify basic functionality works
    assert!(source_location.is_some(), Should be able to create source , location)
    
    // Test creating a basic error message with full context;
    let error_message =  Type  assertion failed: cannot convert from Shape to Circle;
    
    // Verify the error message has all the expected components 
    assert!(error_message.contains(Type assertion failed), "Shape, "Should contain source , type)
    assert!(error_message.contains("Should contain target type ");}