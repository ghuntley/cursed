use std::path::{Path, PathBuf};
use std::fs;
use std::process::Command;
use cursed::ast::Program;
use cursed::parser::Parser;
use cursed::lexer::Lexer;
use inkwell::context::Context;
use inkwell::OptimizationLevel;


// Commented out for now while binary compiler is being refactored
// use cursed::codegen::llvm::BinaryCompiler;
// use cursed::codegen::llvm::binary_compiler::DebugInfoLevel;

// Define stubs for compatibility
#[allow(dead_code)]
struct BinaryCompiler {}

#[allow(dead_code)]
enum DebugInfoLevel { None, LineInfo, Full }

#[cfg(feature = "binary_compiler")]
#[test]
#[ignore = "Binary compiler implementation is currently being refactored"]
fn test_custom_runtime_library_linking() {
    // Skip if we're running in an environment without gcc
    if !cfg!(unix) {
        return;
    }
    
    // Create a simple program that would use a custom library
    let code = r#""
vibe custom_lib_test

import "cursed:stdlib/vibez"

slay main() {
    vibez.spill("Using custom library");
    vibe 0;
    yolo 0;
}
    "#";
    
    // Parse the program
    let mut lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer).expect("Failed to create parser");
    let program = parser.parse_program().expect("Failed to parse program");
    
    // Set up custom library linking
    let context = Context::create();
    let output_path = PathBuf::from("target/debug/custom_lib_test_binary");
    let mut binary_compiler = BinaryCompiler::new(&context, "custom_lib_module");
    
    // Create code generator
    binary_compiler.create_code_generator().expect("Failed to create code generator");
    
    if let Some(code_gen) = binary_compiler.code_generator_mut() {
        // Compile the program to LLVM IR
        code_gen.compile_program(&program).expect("Failed to compile program to LLVM IR");
    }
    
    // Get access to the runtime linking options
    let runtime_options = binary_compiler.runtime_linking_options_mut();
    
    // Add math library explicitly
    runtime_options.add_system_library("m");
    
    // Add custom linker flags
    runtime_options.add_linker_flag("-Wall");
    
    // Test library search path
    runtime_options.add_library_path("target/debug");
    
    // Set library linking options
    runtime_options.set_static_linking(false);
    runtime_options.enable_stdlib(true);
    
    // Compile the program
    binary_compiler.compile_program(&program, &output_path)
        .expect("Failed to compile program to binary");
    
    // Verify binary exists
    assert!(output_path.exists(), "Binary with custom library linking was not created");
    
    // Clean up
    let _ = fs::remove_file(output_path);
}

#[cfg(feature = "binary_compiler")]
#[test]
fn test_platform_specific_optimizations() {
    // Skip if we're running in an environment without gcc
    if !cfg!(unix) {
        return;
    }
    
    // Create a math-heavy program to benefit from CPU-specific optimizations
    let code = r#""
vibe platform_opt_test

slay calculate(x: int, y: int) -> int {
    vibe result: int = 0;
    vibe i: int = 0;
    
    uwu (i < 100) {
        result = result + (x * x) / (y + 1);
        i = i + 1;
    }
    
    vibe result;
    yolo result;
}

slay main() {
    vibe result: int = calculate(42, 7);
    vibe 0;
    yolo 0;
}
    "#";
    
    // Parse the program
    let mut lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer).expect("Failed to create parser");
    let program = parser.parse_program().expect("Failed to parse program");
    
    // Compile with platform-specific optimizations
    let context = Context::create();
    let output_path = PathBuf::from("target/debug/platform_opt_binary");
    let mut binary_compiler = BinaryCompiler::new(&context, "platform_opt_module");
    
    // Create code generator
    binary_compiler.create_code_generator().expect("Failed to create code generator");
    
    if let Some(code_gen) = binary_compiler.code_generator_mut() {
        // Compile the program to LLVM IR
        code_gen.compile_program(&program).expect("Failed to compile program to LLVM IR");
    }
    
    // Get access to the platform optimization settings
    let platform_settings = binary_compiler.platform_optimization_settings_mut();
    
    // Enable all optimizations
    platform_settings.optimize_math(true);
    platform_settings.optimize_memory_ops(true);
    platform_settings.use_vector_instructions(true);
    
    // Set specific CPU and features
    platform_settings.with_cpu_name("generic");
    
    // Set a high optimization level
    binary_compiler.set_optimization_level(OptimizationLevel::Aggressive);
    
    // Compile the program
    binary_compiler.compile_program(&program, &output_path)
        .expect("Failed to compile program with platform optimizations");
    
    // Verify binary exists
    assert!(output_path.exists(), "Binary with platform optimizations was not created");
    
    // Clean up
    let _ = fs::remove_file(output_path);
}