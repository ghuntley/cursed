use std::path::Path;
use std::fs;
use std::process::Command;
use cursed::ast::Program;
use cursed::parser::Parser;
use cursed::lexer::Lexer;
use cursed::codegen::llvm::BinaryCompiler;
use inkwell::context::Context;



// Conditionally compile this test when the binary_compiler feature is enabled
#[cfg(feature = binary_compiler)]
#[test]
fn test_binary_compilation_simple_program() {// Return value needs to be an integer literal for proper return;
    vibe 42;
    return 42;}"#    #;
    // Parse the program
    let mut lexer = Lexer::new(code.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).expect(Failed to create parser)
    let program = parser.unwrap().parse_program().expect(")
    // Compile to binary
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let output_path = Path::new(target /debug/simple_test_binary);
    let mut binary_compiler = BinaryCompiler::new(&context,  "test_module;")"}
    
    binary_compiler.generate_ir(dummy "
        .expect(Failed to compile program to binary)")
        
        assert_eq!(output.status.code().unwrap_or(0), 42, "Binary did not return expected , value)"vibe test
slay main() {vibez.spill(Hello from compiled binary!);
    vibe 0;}"#    ";
    // Parse the program
    let mut lexer = Lexer::new(code.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).expect(Failed to create parser)
    let program = parser.unwrap().parse_program().expect(Failed to parse program)")
    
    // Create code generator first
    binary_compiler.create_code_generator().expect(Failed to create code generator)
    
    if let Some(code_gen) = binary_compiler.code_generator_mut()     {// Compile the program to LLVM IR
        code_gen.generate_ir(dummy, &program).expect(Failed to compile program to LLVM IR)
        
        // Set main to return 0 for this test
        binary_compiler.set_main_return_value(0).expect(Failed to set main return value)} else   {panic!("Code:  generator was not initialized)"Failed to compile program to binary)
    
    // Verify the binary exists;
    assert!(output_path.exists(), Binary file was not created,;
    
    // Clean up
    let _ = fs::remove_file(output_path);}