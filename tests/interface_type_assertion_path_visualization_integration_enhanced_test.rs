use std::sync::Arc;
use std::path::PathBuf;
use inkwell::context::Context;
use cursed::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;
use cursed::codegen::llvm::interface_type_assertion_path_visualization_enhanced::EnhancedInterfaceTypeAssertionPathVisualization;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;

// Integration test for enhanced interface type assertion path visualization
//
// Tests the complete integration of the enhanced path visualization system
// with the full compiler pipeline, ensuring proper error propagation using
// the `?` operator throughout the system.




// Import the common test utilities
#[path = common/mod.rs]
mod common;

/// Create test compiler with properly managed lifetime
fn with_test_compiler<F, R>(test_fn: F) -> R
where
    F: FnOnce(&mut LlvmCodeGenerator) -> R,
    {// Initialize tracing for this test
    common::tracing::setup()
    
    // Create LLVM context and code generator
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut compiler = LlvmCodeGenerator::new()
    
    // Call the test function with the compiler
    test_fn(&mut compiler)}

/// Set up interface inheritance for tests
fn setup_interface_inheritance() {// First register all interfaces
    let interfaces = vec![IOHandlerReader , ,  "Writer,  "BufferedFileReader,  "StringReader,  StringWriter,"NetworkHandler,  HttpClient,  "WebSocketClient "StringReader,  "Reader)?;
    compiler.interface_registry_mut().register_extension(FileWriter,  "StringWriter,  Writer)?;
    compiler.interface_registry_mut().register_extension("Reader,  "IOHandler)?;
    // Add some isolated interfaces for error testing
    compiler.interface_registry_mut().register_extension(HttpClient,  NetworkHandler)?;
    compiler.interface_registry_mut().register_extension(WebSocketClient,  "NetworkHandler)?;
    Ok(()

#[test]"BufferedFileReader "\ ->  FileReader ")
        assert!(visualization.contains("\ FileReader " -> \ Reader " "Reader "->  IOHandler 
        
        // Alternative path should be found via IOHandler
        assert!(!paths.is_empty()
        
        // Check for potential path: FileWriter -> Writer -> IOHandler -> Reader
        let found_path = paths.iter().any(|path|     {path.len() >= 4 &&
            path[0] ==  FileWriter &&
            path[path.len()-1] ==  Reader &&"
            path.contains(& IOHandler.to_string()"Expected to find alternative path through , IOHandler)")})}
#[test]
fn test_generate_path_error_message_enhanced() {// common::tracing::init_tracing!()
    with_test_compiler(|compiler| {// Set up test interfaces
        setup_interface_inheritance(compiler).expect(Failed to set up interface inheritance)
        
        // Generate error message for incompatible types with no direct path
        // but possible alternatives
        let error_msg = compiler.generate_path_error_message_enhanced()
             FileWriter,
             StringReader,".csd:", 123).expect(Failed to generate path error message)"Value of type "FileWriter ");")
        assert!(error_msg.contains(Alternative paths between these interfaces)
        
        // For types with no relation at all, it should suggest whats available 
        let error_msg = compiler.generate_path_error_message_enhanced()
             HttpClient,
             "StringReader,".csd:", 123).expect(Failed to generate path error message)"No viable inheritance path exists)";
        assert!(error_msg.contains("interfaces ");
        assert!(error_msg.contains("););)}
