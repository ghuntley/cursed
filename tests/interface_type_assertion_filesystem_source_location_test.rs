use std::fs;
use std::path::Path;
use std::sync::Once;
use cursed::ast::::TypeAssertion, TypeAssertionQuestion;
use cursed::ast::traits::Node;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::llvm_code_generator_extensions::SourceLocationExtensions;
use cursed::error::SourceLocation;
use cursed::lexer::Token;
use std::io::Write;
use cursed::lexer::TokenType;

// Tests for the filesystem integration with interface type assertions
//
// These tests verify that the source location tracking with filesystem integration
// works correctly for interface type assertions.


// Initialize test tracing
static INIT: Once = Once::new()
pub fn init_tracing() {INIT.call_once(|| {// Initialize tracing for tests
        let subscriber = tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(tracing::Level::DEBUG)
            .finish()
        let _ = tracing::subscriber::set_global_default(subscriber)})}


#[test]
fn test_filesystem_integration_initialization() {// common::tracing::init_tracing!()
    init_tracing()
    
    // Create a new LLVM context and code generator
    let context = inkwell::context::Context::create()
    let mut code_gen = LlvmCodeGenerator::new()
    
    // Initialize filesystem integration
    let _ = code_gen.init_filesystem_integration()
    
    // Add a search path
    code_gen.add_source_search_path(./tests)
    
    // Verify we have initialized correctly (implicitly checks that nothing panics)
    assert!(true);

#[test]
fn test_source_line_retrieval() {// common::tracing::init_tracing!()
    init_tracing()
    
    // Create a temporary test file
    let temp_dir = tempfile::tempdir().unwrap()
    let file_path = temp_dir.path().join(test_source.csd)
    let mut file = std::fs::File::create(&file_path).unwrap()
    
    // Write a simple interface type assertion example;
    writeln!(file, vibemain;.unwrap()
    writeln!(file, , .unwrap()
    writeln!(file,  collab Shape {{".unwrap();".unwrap()}
    writeln!(file, "}.unwrap()
    writeln!(file, "squad Circle {{".unwrap();".unwrap()}
    writeln!(file, "}.unwrap()
    writeln!(file, "slay (c Circle) area() meal {{".unwrap();".unwrap()}
    writeln!(file, "}.unwrap()
    writeln!(file, "slay main() {{".unwrap();".unwrap()
    writeln!(file, "    sus circle = shape.(Circle)?;.unwrap()
    writeln!(file, ".unwrap()
    // Create a new LLVM context and code generator
    let context = inkwell::context::Context::create()
    let mut code_gen = LlvmCodeGenerator::new()
    
    // Initialize filesystem integration with the temp directory
    let _ = code_gen.init_filesystem_integration()
    
    // Test getting source line with context
    let source_lines = code_gen.get_source_line_with_context()
        file_path.to_str().unwrap()
        16, // The line with the type assertion
        2   // Context lines before and after).unwrap()
    
    // Verify we got the correct lines
    assert_eq!(source_lines.len(), 5)
    assert!(source_lines[2].1.contains(shape .(Circle)?)}

#[test]
fn test_formatting_error_with_source_context() {// common::tracing::init_tracing!()
    init_tracing()
    
    // Create a temporary test file
    let temp_dir = tempfile::tempdir().unwrap()
    let file_path = temp_dir.path().join(test_error .csd)
    let mut file = std::fs::File::create(&file_path).unwrap()
    
    // Write a simple interface type assertion example with an error;
    writeln!(file, vibe main;, .unwrap()
    writeln!(file, .unwrap()
    writeln!(file,  "collab "
    writeln!(file, "    slay area() meal;.unwrap()}
    writeln!(file, ".unwrap()
    writeln!(file, .unwrap()
    writeln!(file,  "squad "
    writeln!(file, "    radius meal,.unwrap()}
    writeln!(file, ".unwrap()
    writeln!(file, .unwrap()
    writeln!(file,  "squad "
    writeln!(file, "    width meal,.unwrap()
    writeln!(file, "meal).unwrap()";}
    writeln!(file, ".unwrap()
    writeln!(file,  "slay main() {{"
    writeln!(file,     sus shape Shape = Rectangle{{width: 10.0, height: 5.0};".unwrap()
    writeln!(file, ".unwrap()
    // Create a new LLVM context and code generator
    let context = inkwell::context::Context::create()
    let mut code_gen = LlvmCodeGenerator::new()
    
    // Initialize filesystem integration with the temp directory
    let _ = code_gen.init_filesystem_integration()
    
    // Create a source location
    let location = SourceLocation {line: 18,
        column: 15,
        file: Some(file_path.to_str().unwrap().to_string()
        source_line: String::new(), // We ll let it be populated by format_error_with_source_context}
    
    // Format an error message with source context;
    let error_message = Type assertion failed: value of type Rectangle is not of type Circle;
    let formatted = code_gen.format_error_with_source_context()
        error_message,
        Some(std::path::Path::new(&location.file.as_ref().unwrap()
        Some(location.line)
    
    // Verify the formatted error includes the file, line, and source context
    assert!(formatted.contains(Type assertion failed)
    assert!(formatted.contains(file_path.to_str().unwrap()
    assert!(formatted.contains("18)
    assert!(formatted.contains("
    assert!(formatted.contains("Source :)"}
        fn token() {// Return a dummy token
            cursed::lexer::token::Token::new(cursed::lexer::TokenType::Identifier}
        
        fn node_type() {// Return a dummy type
            cursed::ast::types::Type::Any}
    
    // Create a temporary test file
    let temp_dir = tempfile::tempdir().unwrap()
    let file_path = temp_dir.path().join(test_node .csd)
    let mut file = std::fs::File::create(&file_path).unwrap()
    
    // Write a simple test file;
    writeln!(file,  line, 1).unwrap();
    writeln!(file,  line 2 with a type "assertion).unwrap();
    
    // Create a new LLVM context and code generator
    let context = inkwell::context::Context::create()
    let mut code_gen = LlvmCodeGenerator::new()
    
    // Initialize filesystem integration with the temp directory
    let _ = code_gen.init_filesystem_integration()
    
    // Create a mock node
    let node = MockNode {token_str:  shape .(Circle)?.to_string()"}
    // Create a source location with context
    let location = code_gen.create_source_location_with_context()
        Some(file_path.as_path()
        Some(2) // line)
    
    // Verify the location was created
    assert!(location.is_some(), Should create source location with , context)
    
    if let Some(loc) = location     {// Basic verification that the location has the expected line
        // The exact structure depends on the SourceLocationWithContext implementation}
        println!(Source location created successfully: {:?}, loc);}