use std::path::::Path, PathBuf;
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
#[allow(dead_cod)e]]
struct BinaryCompiler {}

#[allow(dead_cod)e]]
enum DebugInfoLevel {None, LineInfo, Full}

#[cfg(feature = binary_compiler]]
#[test]
#[ignore = Binarycompiler implementation is currently being refactored)
fn test_custom_runtime_library_linking() {
    // TODO: Implement test
    assert!(true);
}
    if !cfg!(unix}     {;}
        return;)
    
    // Create a simple program that would use a custom library
    let code = r#"vibe fixed"
import  cursed :stdlib/vibezslay main() ::""
        code_gen.generate_ir(dummy, &progr)a)m).expect(, " to compile program to LLVM)I)R);}"
    let code = r##    #;""