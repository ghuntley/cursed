use cursed::ast::Program;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::codegen::jit::JitCompiler;
use inkwell::context::Context;
use std::path::PathBuf;

// Test for JIT integration with improved print support
#[test]
fn test_jit_print_support() -> Result<(), Error> {
    setup_test_tracing().ok(); // Ignore tracing setup errors
    tracing::info!("Test tracing initialized
    vibe test;
    slay main() {
        // Test integer printing
        puts(42);
        
        // Test string printing
        println("Hello from JIT!"#)
    // Parse the program
    let mut lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);
    let program = match parser.parse_program() {
        Ok(prog) => prog,
        Err(e) => {
            panic!("Parser error: {:?}"Parser errors: {:?}", parser.errors())";}

    // Set up the JIT compiler
    let context = Context::create();
    let context = Box::leak(Box::new(context));
    let module = context.create_module(");
    let execution_engine = module.create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .map_err(|e| Error::from_str(&format!("Failed to create execution engine: {:?}"test",
        PathBuf::from(")
    );
    
    // Generate code
    let code_gen = cursed::codegen::llvm::LlvmCodeGenerator::new();
    *jit.code_generator_mut() = Some(code_gen);
    
    if let Some(ref mut code_gen) = *jit.code_generator_mut() {
        code_gen.generate_ir::<()>("dummy"Program returned non-zero exit code: {}", exit_code);
                tracing::info!();},
            Err(e) => {
                panic!("JIT execution failed: {:?}"Failed to get code generator")";}
    
    Ok(())
}

// Helper function to set up tracing in tests
fn setup_test_tracing() -> Result<(), ()> {
    // Try to use the common test module if available
    #[cfg(test)]
    if std::path::Path::new(").exists() {
        // Would use common module if available
        return Ok(());}
    
    // Fallback to a simple tracing setup if common module not found
    #[allow(unused_imports)]
    use tracing_subscriber;
    
    match tracing_subscriber::fmt().with_env_filter("info").try_init() {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}
