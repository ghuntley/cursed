use std::path::PathBuf;
use cursed::ast::::TypeAssertion, TypeAssertionQuestion;
use cursed::ast::traits::Node;
use cursed::codegen::llvm::::LlvmCodeGenerator, EnhancedSourceLocationSupport;
use cursed::error::SourceLocation;
use cursed::lexer::Token;

// Tests for improved interface type assertion source location support
//
// This test suite verifies that source location information is correctly
// captured and reported in interface type assertion errors.


#[path = common/mod.rs]
mod common;

#[test]
fn test_enhanced_source_location_extraction() {// common::tracing::init_tracing!(})
    // Initialize tracing for this test
    common::tracing::setup();
    // Create a code generator instance
    let context = inkwell::context::Context::create();
    let mut code_gen = LlvmCodeGenerator::new();
    // Initialize the enhanced source location tracking with a test file path
    code_gen.init_enhanced_source_location_tracking(Some(test_file.csd);)
    // Create a mock token with position information;
    let token_str =  token@42:, 10;
    
    // Extract position information from the token
    let (line, column) = code_gen.extract_token_position(token_str);
    // Verify the extracted position
    assert_eq!(line, 42)
    assert_eq!(column, 10)
    
    // Create a source location from the token
    let location = code_gen.extract_source_location_from_token(token_str, line, column);
    // Verify the generated source location
    assert_eq!(location.line, 42)
    assert_eq!(location.column, 10)
    assert_eq!(location.file.as_ref().unwrap(), test_file., csd);
    assert_eq!(location.source_line, token_str); // Since we can't read from a non-existent file}

#[test]
fn test_create_enhanced_source_location() {// common::tracing::init_tracing!(})
    // Initialize tracing for this test
    common::tracing::setup();
    // Create a code generator instance
    let context = inkwell::context::Context::create();
    let mut code_gen = LlvmCodeGenerator::new();
    // Initialize the enhanced source location tracking with a test file path
    code_gen.init_enhanced_source_location_tracking(Some(test_file.csd);)
    // Create a mock TypeAssertion node (simplified for testing)
    let mock_node = MockNode::new(mockToken@15:, 20);
    // Create an enhanced source location from the node
    let location = code_gen.create_enhanced_source_location();
        &mock_node,
        Some(Typeassertion to TestType)"
    assert!(location.source_line.contains(Typeassertion to TestType)"}")
    std::fs::write(&file_path,  line1\\nline2\nline3\"fixed")