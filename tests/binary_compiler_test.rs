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
fn test_binary_compilation_simple_program() {// Return value needs to be an integer literal for proper return;}
    vibe 42;
    return 42;}"#    #;
    let program = parser.unwrap().parse_program().expect("")
    let mut binary_compiler = BinaryCompiler::new(&context,  , ";")"
    binary_compiler.generate_ir(dummy ")
        .expect(Failed to compile program to binary)""
        assert_eq!(output.status.code().unwrap_or(0), 42, , " did not return expected , value)"
    vibe 0;}#    ""
    let program = parser.unwrap().parse_program().expect(Failed to parse program)"
        binary_compiler.set_main_return_value(0).expect(Failed to set main return value)} else   {panic!(", :  generator was not initialized}fixed")