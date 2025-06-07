use cursed::ast::Node;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use std::path::Path;
use tracing::{debug, info};


// Import the common test utilities
#[path = "common/mod.rs"]
#[allow(unused_imports)]
mod common;

#[test]
#[ignore = "Facts/const codegen needs implementation"]  // TODO: Fix the issue with facts declaration
fn test_facts_codegen() {
    // Initialize tracing for this test
    common::tracing::setup();
    info!("Testing facts codegen compilation");
    let input = r#"
    vibe main; // Add a package declaration to make it more valid
    
    facts PI = 3.14159;
    facts E = 2.71828;
    facts ANSWER = 42;

    slay main() normie {
        yolo ANSWER;
    }
    "#;

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();

    let program = parser.parse_program().unwrap();

    // Create an LLVM context and code generator
    let context = Context::create();
    let file_path = Path::new("facts_test.ll").to_path_buf();
    let mut code_gen = LlvmCodeGenerator::new(&context, "facts_test", file_path);

    // This should not panic if code generation for facts statements is properly implemented
    let result = code_gen.compile(&program);
    if let Err(err) = &result {
        tracing::error!(error = ?err, "Code generation for facts statements failed");
    }
    assert!(
        result.is_ok(),
        "Code generation for facts statements failed: {}",
        result.err().unwrap_or_default()
    );

    // We can't easily run the code in this test, but we can at least verify it compiles
    // and check if the LLVM module looks correct
    let module_str = code_gen.module().print_to_string().to_string();
    
    // The test requires either a main function or at least some constant declarations
    let has_constants = module_str.contains("ANSWER") && module_str.contains("PI") && module_str.contains("E");
    if !has_constants {
        tracing::error!("Module is missing expected constants");
    } else {
        tracing::debug!("Module contains all expected constants");
    }
    
    assert!(
        has_constants,
        "Module should contain the declared constants"
    );
    
    info!("Facts codegen test completed successfully");
}
