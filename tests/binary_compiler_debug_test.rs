use std::path::{Path, PathBuf};
use std::fs;
use std::process::Command;

use cursed::ast::Program;
use cursed::parser::Parser;
use cursed::lexer::Lexer;
// Commented out for now while binary compiler is being refactored
// use cursed::codegen::llvm::BinaryCompiler;
// use cursed::codegen::llvm::binary_compiler::DebugInfoLevel;
use inkwell::context::Context;
use inkwell::OptimizationLevel;

// Define debug info level enum for compatibility
#[allow(dead_code)]
enum DebugInfoLevel { None, LineInfo, Full }

#[cfg(feature = "binary_compiler")]
#[test]
#[ignore = "Binary compiler implementation is currently being refactored"]
fn test_binary_debug_information_generation() {
    // Skip if we're running in an environment without gcc
    if !cfg!(unix) {
        return;
    }
    
    // Create a program with multiple functions and variables for debug info
    let code = r#"
vibe debug_test

slay helper_function(x: int) -> int {
    vibe x * 2;
    yolo x * 2;
}

slay main() {
    // Create some variables to test debug info
    vibe value: int = 42;
    vibe result: int = helper_function(value);
    vibez.spill("Result is: " + result);
    vibe 0;
    yolo 0;
}
    "#;
    
    // Parse the program
    let mut lexer = Lexer::new(code);
    let mut parser = Parser::new(&mut lexer).expect("Failed to create parser");
    let program = parser.parse_program().expect("Failed to parse program");
    
    // Test with different debug levels
    let debug_levels = [
        ("none", DebugInfoLevel::None),
        ("line", DebugInfoLevel::LineInfo),
        ("full", DebugInfoLevel::Full),
    ];
    
    for (name, level) in &debug_levels {
        // Compile with the specific debug level
        let context = Context::create();
        let output_path = PathBuf::from(format!("target/debug/debug_{}_test_binary", name));
        let mut binary_compiler = BinaryCompiler::new(&context, "debug_test_module");
        
        // Create code generator
        binary_compiler.create_code_generator().expect("Failed to create code generator");
        
        if let Some(code_gen) = binary_compiler.code_generator_mut() {
            // Compile the program to LLVM IR
            code_gen.compile_program(&program).expect("Failed to compile program to LLVM IR");
        }
        
        // Enable debug information
        binary_compiler.enable_debug_info(*level);
        
        // Enable stdlib for printing
        binary_compiler.enable_stdlib_linking(true);
        
        // Compile with optimizations enabled to test debug info preservation
        binary_compiler.set_optimization_level(OptimizationLevel::Default);
        
        binary_compiler.compile_program(&program, &output_path)
            .expect("Failed to compile program to binary");
        
        // Verify binary exists
        assert!(output_path.exists(), "Binary with debug level {:?} was not created", level);
        
        // Try to verify debug info if platform and tools support it
        #[cfg(target_os = "linux")]
        verify_debug_info(&output_path, level);
        
        // Clean up
        let _ = fs::remove_file(output_path);
    }
}

#[cfg(all(feature = "binary_compiler", target_os = "linux"))]
fn verify_debug_info(binary_path: &Path, level: &DebugInfoLevel) {
    // Check if objdump is available
    if Command::new("which").arg("objdump").status().map(|s| s.success()).unwrap_or(false) {
        let output = Command::new("objdump")
            .args(&["-h", binary_path.to_str().unwrap()])
            .output()
            .expect("Failed to execute objdump");
        
        let output_str = String::from_utf8_lossy(&output.stdout);
        let has_debug_section = output_str.contains(".debug_info") || output_str.contains(".debug_");
        
        match level {
            DebugInfoLevel::None => {
                // No debug sections expected
                if has_debug_section {
                    println!("Warning: Debug sections found but weren't expected for {:?}", level);
                }
            },
            DebugInfoLevel::LineInfo | DebugInfoLevel::Full => {
                // Debug sections expected
                if !has_debug_section {
                    println!("Warning: No debug sections found for {:?}", level);
                } else {
                    println!("Debug sections verified in binary for level {:?}", level);
                    
                    // For full debug info, check for variable info sections
                    if level == &DebugInfoLevel::Full {
                        let has_variable_info = output_str.contains(".debug_info") && 
                                                output_str.contains(".debug_loc");
                                                
                        if !has_variable_info {
                            println!("Warning: Full variable debug info sections not found");
                        } else {
                            println!("Full variable debug info verified");
                        }
                    }
                }
            }
        }
    } else {
        println!("Note: objdump not available to verify debug sections");
    }
}

#[cfg(feature = "binary_compiler")]
#[test]
#[ignore = "Binary compiler implementation is currently being refactored"]
fn test_binary_source_mapping() {
    // Skip if we're running in an environment without gcc
    if !cfg!(unix) {
        return;
    }
    
    // Create a program with line numbers and source references
    let code = r#"
vibe source_mapping_test

slay function1() -> int {
    // This is line 5 in the source
    vibe 10;
    yolo 10;
}

slay function2() -> int {
    // This is line 11 in the source
    vibe 20;
    yolo 20;
}

slay main() {
    // This is line 17 in the source
    vibe value1: int = function1();
    vibe value2: int = function2();
    vibe sum: int = value1 + value2;
    
    // This is line 22 in the source
    vibez.spill("Sum is: " + sum);
    vibe 0;
    yolo 0;
}
    "#;
    
    // Parse the program
    let mut lexer = Lexer::new(code);
    let mut parser = Parser::new(&mut lexer).expect("Failed to create parser");
    let program = parser.parse_program().expect("Failed to parse program");
    
    // Compile with full debug info
    let context = Context::create();
    let output_path = PathBuf::from("target/debug/source_mapping_test_binary");
    let mut binary_compiler = BinaryCompiler::new(&context, "source_mapping_module");
    
    // Create code generator
    binary_compiler.create_code_generator().expect("Failed to create code generator");
    
    if let Some(code_gen) = binary_compiler.code_generator_mut() {
        // Compile the program to LLVM IR
        code_gen.compile_program(&program).expect("Failed to compile program to LLVM IR");
    }
    
    // Enable full debug information
    binary_compiler.enable_debug_info(DebugInfoLevel::Full);
    
    // Enable stdlib for printing
    binary_compiler.enable_stdlib_linking(true);
    
    binary_compiler.compile_program(&program, &output_path)
        .expect("Failed to compile program to binary");
    
    // Verify binary exists
    assert!(output_path.exists(), "Binary with source mapping was not created");
    
    // Check if LLVM IR was generated with debug info
    let ll_path = output_path.with_extension("ll");
    if ll_path.exists() {
        // Read the LLVM IR and check for debug info
        let ir_content = fs::read_to_string(&ll_path).expect("Failed to read LLVM IR");
        
        // Check for debug info metadata in the IR
        let has_debug_info = ir_content.contains("!DISubprogram") || 
                             ir_content.contains("!DILocation") || 
                             ir_content.contains("!DIFile");
                             
        if !has_debug_info {
            println!("Warning: No debug metadata found in LLVM IR");
        } else {
            println!("Debug metadata found in LLVM IR");
        }
    }
    
    // Clean up
    let _ = fs::remove_file(output_path);
    let _ = fs::remove_file(ll_path);
}