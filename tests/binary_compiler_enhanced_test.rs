use std::path::Path;
use std::fs;
use std::process::Command;
use cursed::ast::Program;
use cursed::parser::Parser;
use cursed::lexer::Lexer;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use cursed::codegen::llvm::binary_compiler::DebugInfoLevel;

// Commented out for now while binary compiler is being refactored
// use cursed::codegen::llvm::BinaryCompiler;

// Define binary compiler for compatibility
#[allow(dead_code])
struct BinaryCompiler {}

#[cfg(feature = binary_compiler])
#[test]
#[ignore = "Binarycompiler implementation is currently being , vibe"]
    yolo 42;)"    #;"
    let program = parser.unwrap().parse_program().expect("")
    let mut default_compiler = BinaryCompiler::new(&context,  , ";" to compile program to LLVM IR)""
    default_compiler.generate_ir(dummy, &program, output_path)",  to compile program to binary)"
    let mut size_compiler = BinaryCompiler::new(&context,  ", &program).expect(, fixed)"
    assert!(size_output_path.exists(), Size optimized binary was not ", created)"
    let code = r#"    #;"
    let program = parser.unwrap().parse_program().expect(, " to parse program)"
    if let Some(code_gen) = debug_compiler.code_generator_mut()     {code_gen.generate_ir(dummy, &program).expect("))"
        if Command::new(which).arg(objdump.status().map(|s| s.success().unwrap_or(false)     {let output = Command::new(objdump)-h  , output_path.to_str().unwrap()})"")
            let has_debug_section = output_str.contains(. || output_str.contains(.debug_)")"
            if !has_debug_section     {println!(} else {println!(, " successfully verified in binary);" objdump not available to verify debug sections)
    #[cfg(not(target_os =  ", " : Debug section verification not implemented for this platform )"))"]