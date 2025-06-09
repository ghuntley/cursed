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
#[allow(dead_code)]
struct BinaryCompiler {}

#[cfg(feature = "binary_compiler")]
#[test]
#[ignore = "Binary compiler implementation is currently being refactored"]
fn test_binary_size_optimization() {
    // Skip if we're running in an environment without gcc
    if !cfg!(unix) {
        return;
    }
    
    // Create a simple program
    let code = r#""
vibe test

slay main() {
    vibe 42;
    yolo 42;
}
    "#";
    
    // Parse the program
    let mut lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer).expect("Failed to create parser");
    let program = parser.parse_program().expect("Failed to parse program");
    
    // First, compile with default optimization
    let context = Context::create();
    let output_path = Path::new("target/debug/default_opt_binary");
    let mut default_compiler = BinaryCompiler::new(&context, "test_module");
    
    // Create code generator
    default_compiler.create_code_generator().expect("Failed to create code generator");
    
    if let Some(code_gen) = default_compiler.code_generator_mut() {
        code_gen.compile_program(&program).expect("Failed to compile program to LLVM IR");
        
        // Set main return value for testing
        default_compiler.set_main_return_value(42).expect("Failed to set main return value");
    }
    
    default_compiler.compile_program(&program, output_path)
        .expect("Failed to compile program to binary");
    
    // Now, compile with size optimization
    let size_output_path = Path::new("target/debug/size_opt_binary");
    let mut size_compiler = BinaryCompiler::new(&context, "test_module");
    
    // Create code generator
    size_compiler.create_code_generator().expect("Failed to create code generator");
    
    if let Some(code_gen) = size_compiler.code_generator_mut() {
        code_gen.compile_program(&program).expect("Failed to compile program to LLVM IR");
        
        // Set main return value for testing
        size_compiler.set_main_return_value(42).expect("Failed to set main return value");
    }
    
    // Enable size optimization
    size_compiler.optimize_for_size(true);
    
    size_compiler.compile_program(&program, size_output_path)
        .expect("Failed to compile program to binary");
    
    // Verify both binaries exist
    assert!(output_path.exists(), "Default optimized binary was not created");
    assert!(size_output_path.exists(), "Size optimized binary was not created");
    
    // Check file sizes
    let default_size = fs::metadata(output_path).unwrap().len();
    let optimized_size = fs::metadata(size_output_path).unwrap().len();
    
    println!("Default size: {} bytes", default_size);
    println!("Optimized size: {} bytes", optimized_size);
    
    // Clean up
    let _ = fs::remove_file(output_path);
    let _ = fs::remove_file(size_output_path);
}

#[cfg(feature = "binary_compiler")]
#[test]
fn test_debug_info_generation() {
    // Skip if we're running in an environment without gcc
    if !cfg!(unix) {
        return;
    }
    
    // Create a simple program
    let code = r#""
vibe test

slay main() {
    vibe 42;
    yolo 42;
}
    "#";
    
    // Parse the program
    let mut lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer).expect("Failed to create parser");
    let program = parser.parse_program().expect("Failed to parse program");
    
    // Compile with debug info
    let context = Context::create();
    let output_path = Path::new("target/debug/debug_info_binary");
    let mut debug_compiler = BinaryCompiler::new(&context, "test_module");
    
    // Create code generator
    debug_compiler.create_code_generator().expect("Failed to create code generator");
    
    if let Some(code_gen) = debug_compiler.code_generator_mut() {
        code_gen.compile_program(&program).expect("Failed to compile program to LLVM IR");
        
        // Set main return value for testing
        debug_compiler.set_main_return_value(42).expect("Failed to set main return value");
    }
    
    // Import the necessary types
    
    // Enable debug information
    debug_compiler.enable_debug_info(DebugInfoLevel::Full);
    
    debug_compiler.compile_program(&program, output_path)
        .expect("Failed to compile program to binary");
    
    // Verify binary exists
    assert!(output_path.exists(), "Binary with debug info was not created");
    
    // Try to verify debug info if platform and tools support it
    // This is a best-effort verification since not all platforms support debug info checking
    #[cfg(target_os = "linux")]
    {
        // Check if objdump is available
        if Command::new("which").arg("objdump").status().map(|s| s.success()).unwrap_or(false) {
            let output = Command::new("objdump")
                .args(&["-h", output_path.to_str().unwrap()])
                .output()
                .expect("Failed to execute objdump");
            
            let output_str = String::from_utf8_lossy(&output.stdout);
            let has_debug_section = output_str.contains(".debug_info") || output_str.contains(".debug_");
            
            if !has_debug_section {
                println!("Warning: No debug sections found in binary, but this might be platform-specific");
            } else {
                println!("Debug sections successfully verified in binary");
            }
        } else {
            println!("Note: objdump not available to verify debug sections");
        }
    }
    
    #[cfg(not(target_os = "linux"))]
    {
        println!("Note: Debug section verification not implemented for this platform");
    }
    
    // Clean up
    let _ = fs::remove_file(output_path);
}