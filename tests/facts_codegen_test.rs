use std::path::Path;
use cursed::parser::Parser;
use cursed::lexer::Lexer;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::ast::Node;

#[test]
fn test_facts_codegen() {
    let input = r#"
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
    assert!(result.is_ok(), "Code generation for facts statements failed: {}", result.err().unwrap_or_default());
    
    // We can't easily run the code in this test, but we can at least verify it compiles
    // and check if the LLVM module looks correct
    let module_str = code_gen.module().print_to_string().to_string();
    assert!(module_str.contains("main"), "Module should contain a main function");
}