//! Integration test for enhanced interface type assertion path visualization
//!
//! Tests the complete integration of the enhanced path visualization system
//! with the full compiler pipeline, ensuring proper error propagation using
//! the `?` operator throughout the system.

use std::sync::Arc;
use std::path::PathBuf;

use inkwell::context::Context;

use cursed::codegen::llvm::interface_type_assertion_path_visualization_enhanced::EnhancedInterfaceTypeAssertionPathVisualization;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;

// Import the common test utilities
#[path = "common.rs"]
mod common;

// Test context setup
fn setup_test_compiler() -> LlvmCodeGenerator {
    // Initialize tracing for this test
    common::tracing::setup();
    
    // Create LLVM context and code generator
    let context = Context::create();
    let compiler = LlvmCodeGenerator::new(&context, "test_module", PathBuf::from("test.csd"));
    
    // Return the configured compiler
    compiler
}

/// Set up interface inheritance for tests
fn setup_interface_inheritance(compiler: &mut LlvmCodeGenerator) -> Result<(), Error> {
    // Set up a test inheritance hierarchy similar to:
    // Reader -> FileReader -> BufferedFileReader
    // Writer -> FileWriter
    // Reader -> StringReader
    // Writer -> StringWriter
    // IOHandler -> Reader
    // IOHandler -> Writer
    compiler.interface_registry_mut().register_extension("FileReader", "Reader")?;
    compiler.interface_registry_mut().register_extension("BufferedFileReader", "FileReader")?;
    compiler.interface_registry_mut().register_extension("StringReader", "Reader")?;
    compiler.interface_registry_mut().register_extension("FileWriter", "Writer")?;
    compiler.interface_registry_mut().register_extension("StringWriter", "Writer")?;
    compiler.interface_registry_mut().register_extension("Reader", "IOHandler")?;
    compiler.interface_registry_mut().register_extension("Writer", "IOHandler")?;
    
    // Add some isolated interfaces for error testing
    compiler.interface_registry_mut().register_extension("HttpClient", "NetworkHandler")?;
    compiler.interface_registry_mut().register_extension("WebSocketClient", "NetworkHandler")?;
    
    Ok(())
}

#[test]
fn test_generate_interface_hierarchy_dot_enhanced() {
    // Create compiler instance
    let mut compiler = setup_test_compiler();
    
    // Set up test interfaces
    setup_interface_inheritance(&mut compiler).expect("Failed to set up interface inheritance");
    
    // Test the enhanced DOT generation
    let dot = compiler.generate_interface_hierarchy_dot_enhanced()
        .expect("Failed to generate interface hierarchy DOT");
    
    // Verify DOT graph content
    assert!(dot.contains("digraph interface_hierarchy"));
    assert!(dot.contains("\"Reader\" [label=\"Reader\"]"));
    assert!(dot.contains("\"FileReader\" [label=\"FileReader\"]"));
    assert!(dot.contains("\"FileReader\" -> \"Reader\";"));
    assert!(dot.contains("\"BufferedFileReader\" -> \"FileReader\";"));
    
    // Verify proper error propagation by checking the error message format
    // for a generated path that doesn't exist
    let result = compiler.find_interface_path("HttpClient", "StringReader");
    assert!(result.is_err());
    let err = result.unwrap_err();
    match err {
        Error::Compilation(msg) => {
            assert!(msg.contains("No path found from interface 'HttpClient' to interface 'StringReader'"));
        },
        _ => panic!("Expected Compilation error")
    }
}

#[test]
fn test_visualize_interface_path_enhanced() {
    // Create compiler instance
    let mut compiler = setup_test_compiler();
    
    // Set up test interfaces
    setup_interface_inheritance(&mut compiler).expect("Failed to set up interface inheritance");
    
    // Test the enhanced path visualization
    let visualization = compiler.visualize_interface_path_enhanced("BufferedFileReader", "IOHandler")
        .expect("Failed to visualize interface path");
    
    // Check that the visualization contains the expected content
    assert!(visualization.contains("Interface Inheritance Path:"));
    assert!(visualization.contains("  [BufferedFileReader]"));
    assert!(visualization.contains("  u2193 extends"));
    assert!(visualization.contains("  [FileReader]"));
    assert!(visualization.contains("  [Reader]"));
    assert!(visualization.contains("  [IOHandler]"));
    
    // Check DOT representation
    assert!(visualization.contains("digraph path"));
    assert!(visualization.contains("\"BufferedFileReader\" -> \"FileReader\";"));
    assert!(visualization.contains("\"FileReader\" -> \"Reader\";"));
    assert!(visualization.contains("\"Reader\" -> \"IOHandler\";"));
}

#[test]
fn test_find_alternative_paths_enhanced() {
    // Create compiler instance
    let mut compiler = setup_test_compiler();
    
    // Set up test interfaces
    setup_interface_inheritance(&mut compiler).expect("Failed to set up interface inheritance");
    
    // Test finding alternative paths from FileWriter to Reader (should fail directly)
    // But alternative paths might exist through IOHandler
    let paths = compiler.find_alternative_paths_enhanced("FileWriter", "Reader", 3)
        .expect("Failed to find alternative paths");
    
    // Alternative path should be found via IOHandler
    assert!(!paths.is_empty());
    
    // Check for potential path: FileWriter -> Writer -> IOHandler -> Reader
    let found_path = paths.iter().any(|path| {
        path.len() >= 4 &&
        path[0] == "FileWriter" &&
        path[path.len()-1] == "Reader" &&
        path.contains(&"IOHandler".to_string())
    });
    
    assert!(found_path, "Expected to find alternative path through IOHandler");
}

#[test]
fn test_generate_path_error_message_enhanced() {
    // Create compiler instance
    let mut compiler = setup_test_compiler();
    
    // Set up test interfaces
    setup_interface_inheritance(&mut compiler).expect("Failed to set up interface inheritance");
    
    // Generate error message for incompatible types with no direct path
    // but possible alternatives
    let error_msg = compiler.generate_path_error_message_enhanced(
        "FileWriter", 
        "StringReader", 
        "test.csd:123"
    ).expect("Failed to generate path error message");
    
    // Verify error message format
    assert!(error_msg.contains("Type assertion error at test.csd:123"));
    assert!(error_msg.contains("Value of type 'FileWriter' cannot be asserted as type 'StringReader'"));
    assert!(error_msg.contains("Alternative paths between these interfaces"));
    
    // For types with no relation at all, it should suggest what's available
    let error_msg = compiler.generate_path_error_message_enhanced(
        "HttpClient", 
        "StringReader", 
        "test.csd:123"
    ).expect("Failed to generate path error message");
    
    assert!(error_msg.contains("No viable inheritance path exists"));
    assert!(error_msg.contains("'HttpClient' directly extends these interfaces"));
    assert!(error_msg.contains("- NetworkHandler"));
}

// Test error extraction functions independently
#[test]
fn test_extract_error_type_information() {
    // Test source type extraction from different error message formats
    let error1 = "Type assertion error at test.csd:123: Value of type 'MyStruct' cannot be asserted as type 'Reader'";
    let error2 = "Cannot convert from 'FileWriter' to 'StringReader' at test.csd:123";
    
    // Test with module-level functions directly
    let source1 = cursed::codegen::llvm::interface_type_assertion_path_visualization_enhanced::extract_source_type_from_error(error1);
    let source2 = cursed::codegen::llvm::interface_type_assertion_path_visualization_enhanced::extract_source_type_from_error(error2);
    
    assert_eq!(source1, Some("MyStruct".to_string()));
    assert_eq!(source2, Some("FileWriter".to_string()));
    
    // Test target type extraction similarly
    let target1 = cursed::codegen::llvm::interface_type_assertion_path_visualization_enhanced::extract_target_type_from_error(error1);
    let target2 = cursed::codegen::llvm::interface_type_assertion_path_visualization_enhanced::extract_target_type_from_error(error2);
    
    assert_eq!(target1, Some("Reader".to_string()));
    assert_eq!(target2, Some("StringReader".to_string()));
}