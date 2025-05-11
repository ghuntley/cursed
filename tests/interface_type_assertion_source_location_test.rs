//! Tests for improved interface type assertion source location support
//!
//! This test suite verifies that source location information is correctly
//! captured and reported in interface type assertion errors.

use std::path::Path;
use cursed::ast::expressions::{TypeAssertion, TypeAssertionQuestion};
use cursed::ast::traits::Node;
use cursed::codegen::llvm::{LlvmCodeGenerator, EnhancedSourceLocationSupport};
use cursed::error::SourceLocation;

#[path = "common.rs"]
mod common;

#[test]
fn test_enhanced_source_location_extraction() {
    // Initialize tracing for this test
    common::tracing::setup();
    
    // Create a code generator instance
    let context = inkwell::context::Context::create();
    let mut code_gen = LlvmCodeGenerator::new("test_module", &context);
    
    // Initialize the enhanced source location tracking with a test file path
    code_gen.init_enhanced_source_location_tracking(Some("test_file.csd"));
    
    // Create a mock token with position information
    let token_str = "token@42:10";
    
    // Extract position information from the token
    let (line, column) = code_gen.extract_token_position(token_str);
    
    // Verify the extracted position
    assert_eq!(line, 42);
    assert_eq!(column, 10);
    
    // Create a source location from the token
    let location = code_gen.extract_source_location_from_token(token_str, line, column);
    
    // Verify the generated source location
    assert_eq!(location.line, 42);
    assert_eq!(location.column, 10);
    assert_eq!(location.file.as_ref().unwrap(), "test_file.csd");
    assert_eq!(location.source_line, token_str); // Since we can't read from a non-existent file
}

#[test]
fn test_create_enhanced_source_location() {
    // Initialize tracing for this test
    common::tracing::setup();
    
    // Create a code generator instance
    let context = inkwell::context::Context::create();
    let mut code_gen = LlvmCodeGenerator::new("test_module", &context);
    
    // Initialize the enhanced source location tracking with a test file path
    code_gen.init_enhanced_source_location_tracking(Some("test_file.csd"));
    
    // Create a mock TypeAssertion node (simplified for testing)
    let mock_node = MockNode::new("mockToken@15:20");
    
    // Create an enhanced source location from the node
    let location = code_gen.create_enhanced_source_location(
        &mock_node,
        Some("Type assertion to TestType")
    );
    
    // Verify the enhanced source location
    assert_eq!(location.line, 15);
    assert_eq!(location.column, 20);
    assert_eq!(location.file.as_ref().unwrap(), "test_file.csd");
    assert!(location.source_line.contains("Type assertion to TestType"));
}

// Mock implementation for testing
struct MockNode {
    token: String,
}

impl MockNode {
    fn new(token: &str) -> Self {
        Self { token: token.to_string() }
    }
}

impl Node for MockNode {
    fn token_literal(&self) -> String {
        self.token.clone()
    }
    
    fn string(&self) -> String {
        self.token.clone()
    }
}

#[test]
fn test_source_location_cache() {
    // Initialize tracing for this test
    common::tracing::setup();
    
    // Create a code generator instance
    let context = inkwell::context::Context::create();
    let mut code_gen = LlvmCodeGenerator::new("test_module", &context);
    
    // Initialize the enhanced source location tracking
    code_gen.init_enhanced_source_location_tracking(None);
    
    // Create a temporary test file
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("test_source.csd");
    std::fs::write(&file_path, "line1\nline2\nline3\nline4\n").unwrap();
    
    // Cache the file
    code_gen.cache_source_file(file_path.to_str().unwrap()).unwrap();
    
    // Get a line from the cache
    let line = code_gen.get_cached_source_line(file_path.to_str().unwrap(), 3).unwrap();
    
    // Verify the line content
    assert_eq!(line, "line3");
}