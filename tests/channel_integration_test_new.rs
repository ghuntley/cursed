//! Integration tests for channel codegen functionality

use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::codegen::llvm::LlvmCodeGenerator;
use inkwell::context::Context;
use std::path::PathBuf;

#[test]
fn test_channel_operations_codegen() {
    // Create a simple channel program
    let program = r"
    let ch = chan thicc;
    ch <- 42;
    let val = <-ch;
    ";

    // Parse the program
    let mut lexer = Lexer::new(program);
    let mut parser = Parser::new(&mut lexer).unwrap();
    let ast = parser.parse_program().unwrap();

    // Generate LLVM IR
    let context = Context::create();
    let dummy_path = PathBuf::from("./test_channel_ops.csd");
    let mut codegen = LlvmCodeGenerator::new(&context, "test_channel_ops", dummy_path);
    
    // Compile the program
    let result = codegen.compile(&ast);
    
    // Verify it compiles successfully
    assert!(result.is_ok(), "Failed to compile channel program: {:?}", result.err());
    
    // The module should verify correctly
    // We use the public .module() method to access the module
    assert!(codegen.module().verify().is_ok(), "Generated LLVM IR module failed verification");
    
    // Print the IR for debugging if needed
    // println!("{}", codegen.module().print_to_string());
}

// Test a non-blocking channel program
#[test]
fn test_nonblocking_channel_operations() {
    // Since our language might not have direct syntax for non-blocking ops yet,
    // we can at least verify the module builds correctly by manually compiling
    // a program containing channels
    
    let program = r"
    let ch = chan thicc;
    ch <- 42;
    let val = <-ch;
    close(ch);
    ";

    // Parse the program
    let mut lexer = Lexer::new(program);
    let mut parser = Parser::new(&mut lexer).unwrap();
    let ast = parser.parse_program().unwrap();

    // Generate LLVM IR
    let context = Context::create();
    let dummy_path = PathBuf::from("./test_nonblocking_ops.csd");
    let mut codegen = LlvmCodeGenerator::new(&context, "test_nonblocking_ops", dummy_path);
    
    // Compile the program
    let result = codegen.compile(&ast);
    
    // Verify it compiles successfully
    assert!(result.is_ok(), "Failed to compile nonblocking channel program: {:?}", result.err());
    
    // The module should verify correctly
    assert!(codegen.module().verify().is_ok(), "Generated LLVM IR module failed verification");
}

// Test a buffered channel program
#[test]
fn test_buffered_channel_operations() {
    // Create a buffered channel program
    let program = r"
    let ch = chan thicc(5);
    ch <- 42;
    ch <- 43;
    let val1 = <-ch;
    let val2 = <-ch;
    ";

    // Parse the program
    let mut lexer = Lexer::new(program);
    let mut parser = Parser::new(&mut lexer).unwrap();
    let ast = parser.parse_program().unwrap();

    // Generate LLVM IR
    let context = Context::create();
    let dummy_path = PathBuf::from("./test_buffered_channel.csd");
    let mut codegen = LlvmCodeGenerator::new(&context, "test_buffered_channel", dummy_path);
    
    // Compile the program
    let result = codegen.compile(&ast);
    
    // Verify it compiles successfully
    assert!(result.is_ok(), "Failed to compile buffered channel program: {:?}", result.err());
    
    // The module should verify correctly
    assert!(codegen.module().verify().is_ok(), "Generated LLVM IR module failed verification");
}