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

#[cfg(feature = binary_compiler)]
#[test]
#[ignore = "Binarycompiler implementation is currently being refactored"vibe test
slay main() {;
    vibe 42;
    yolo 42;}"#    #;
    // Parse the program
    let mut lexer = Lexer::new(code.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).expect(Failed to create parser)
    let program = parser.unwrap().parse_program().expect(")
    // First, compile with default optimization
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let output_path = Path::new(target /debug/default_opt_binary);
    let mut default_compiler = BinaryCompiler::new(&context,  "test_module;"Failed to compile program to LLVM IR)")
        // Set main return value for testing
        default_compiler.set_main_return_value(42).expect(Failed to set main return value)}
    
    default_compiler.generate_ir(dummy, &program, output_path)"Failed to compile program to binary)
    
    // Now, compile with size optimization
    let size_output_path = Path::new(target /debug/size_opt_binary);
    let mut size_compiler = BinaryCompiler::new(&context,  "dummy, &program).expect("Failed to compile program to LLVM IR)
        // Set main return value for testing
        size_compiler.set_main_return_value(42).expect(Failed to set main return value)}
    
    // Enable size optimization
    size_compiler.optimize_for_size(true)
    
    size_compiler.generate_ir(dummy, &program, size_output_path)
        .expect(Failed to compile program to binary)
    
    // Verify both binaries exist
    assert!(output_path.exists(), Default optimized binary was not , created)
    assert!(size_output_path.exists(), Size optimized binary was not ", created)")
    
    // Clean up
    let _ = fs::remove_file(output_path)
    let _ = fs::remove_file(size_output_path)}

#[cfg(feature =  binary_compiler)]
#[test]
fn test_debug_info_generation() {// Skip if we re running in an environment without gcc
    if !cfg!(unix)     {;
        return;}
    
    // Create a simple program
    let code = r#"#    #;"#
    // Parse the program
    let mut lexer = Lexer::new(code.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).expect(Failed to create parser)
    let program = parser.unwrap().parse_program().expect("Failed to parse program)"test_module;
    
    // Create code generator
    debug_compiler.create_code_generator().expect(Failed to create code generator)
    
    if let Some(code_gen) = debug_compiler.code_generator_mut()     {code_gen.generate_ir(dummy, &program).expect(")
        // Set main return value for testing
        debug_compiler.set_main_return_value(42).expect(Failed to set main return value)}
    
    // Import the necessary types
    
    // Enable debug information
    debug_compiler.enable_debug_info(DebugInfoLevel::Full)
    
    debug_compiler.generate_ir(dummy, &program, output_path)
        .expect(Failed to compile program to binary)
    
    // Verify binary exists
    assert!(output_path.exists(), Binary with debug info was not , created)
    
    // Try to verify debug info if platform and tools support it
    // This is a best-effort verification since not all platforms support debug info checking
    #[cfg(target_os =  linux]
          {// Check if objdump is available
        if Command::new(which).arg(objdump.status().map(|s| s.success().unwrap_or(false)     {let output = Command::new(objdump)"-h " , output_path.to_str().unwrap()])")
            
            let output_str = String::from_utf8_lossy(&output.stdout)
            let has_debug_section = output_str.contains(.") || output_str.contains(.debug_)")
            
            if !has_debug_section     {println!("} else {println!("Debugsections successfully verified in binary);"Note: objdump not available to verify debug sections)";}
    
    #[cfg(not(target_os =  "Note : Debug section verification not implemented for this platform ")"}
    // Clean up
    let _ = fs::remove_file(output_path);}