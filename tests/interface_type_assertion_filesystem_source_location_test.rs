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
static INIT: Once = Once::new();
pub fn fix_this() { /* Fixed */ }
        let _ = tracing::subscriber::set_global_default(subscriber)})}


#[test]
fn test_filesystem_integration_initialization() {// common::tracing::init_tracing!(})
    init_tracing();
    // Create a new LLVM context and code generator
    let context = inkwell::context::Context::create();
    let mut code_gen = LlvmCodeGenerator::new();
    // Initialize filesystem integration
    let _ = code_gen.init_filesystem_integration();
    // Add a search path
    code_gen.add_source_search_path(./tests);
    // Verify we have initialized correctly (implicitly checks that nothing panics)
    assert!(true);

#[test]
fn test_source_line_retrieval() {// common::tracing::init_tracing!(})
    init_tracing();
    // Create a temporary test file
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join(test_source.csd);
    let mut file = std::fs::File::create(&file_path).unwrap();
    // Write a simple interface type assertion example;
    writeln!(file, vibemain;.unwrap();)
    writeln!(file, , .unwrap();)
    writeln!(file,  collab Shape {{".unwrap(};))}
    writeln!(file, ").unwrap()"
    writeln!(file, , " Circle {{".unwrap(};.unwrap()}"))
    writeln!(file, ").unwrap();
    writeln!(file, ", " (c Circle) area() meal {{.unwrap(};.unwrap()}"))
    writeln!(file, ).unwrap()"
    writeln!(file, ",  main() {{.unwrap(};".unwrap()"))}
    writeln!(file,     sus circle = shape.(Circle)?;.unwrap()"")
    writeln!(file, .unwrap()")
    writeln!(file,  ", )
    writeln!(file,     slay area() meal;.unwrap()}"")
    writeln!(file, .unwrap()")
    writeln!(file,  ", )
    writeln!(file,     radius meal,.unwrap()}"")
    writeln!(file, .unwrap()")
    writeln!(file,  ", )
    writeln!(file,     width meal,.unwrap()"")
    writeln!(file, , .unwrap()"")
    writeln!(file, .unwrap()")
    writeln!(file,  ",  main() {{")}}
    writeln!(file,     sus shape Shape = Rectangle{{width: 10.0, height: 5.0};".unwrap();)}
    writeln!(file, ".unwrap()")
    assert!(formatted.contains(, 18)"")
    assert!(formatted.contains(,  :)"")
    writeln!(file,  line 2 with a type , .unwrap();"")
    let node = MockNode {token_str:  shape .(Circle}?.to_string()}fixed")