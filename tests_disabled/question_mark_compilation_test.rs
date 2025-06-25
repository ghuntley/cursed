/// Test for question mark operator compilation in LLVM expression compiler

use cursed::ast::expressions::{QuestionMarkExpression, Literal, LiteralValue};
use cursed::ast::traits::Expression;
use cursed::codegen::llvm::{LlvmCodeGenerator, QuestionMarkCompiler};

#[test]
fn test_question_mark_operator_integration() {
    // Create a simple question mark expression
    let literal = Literal::new(LiteralValue::Integer(42));
    let question_mark_expr = QuestionMarkExpression::new(
        Box::new(literal),
        1, // line
        1  // column
    );
    
    // Create LLVM code generator
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create code generator");
    
    // Test that the expression can be compiled through the main pipeline
    let result = generator.compile_expression(&question_mark_expr);
    assert!(result.is_ok(), "Question mark expression should compile successfully");
    
    // Test that the specialized question mark compiler methods work
    let result_compilation = generator.compile_result_question_mark(&question_mark_expr);
    assert!(result_compilation.is_ok(), "Result question mark compilation should work");
    
    let option_compilation = generator.compile_option_question_mark(&question_mark_expr);
    assert!(option_compilation.is_ok(), "Option question mark compilation should work");
}

#[test]
fn test_question_mark_expression_pipeline_integration() {
    let literal = Literal::new(LiteralValue::String("test".to_string()));
    let question_mark_expr = QuestionMarkExpression::new(
        Box::new(literal),
        1, // line
        1  // column
    );
    
    let mut generator = LlvmCodeGenerator::new().expect("Failed to create code generator");
    
    // Test the main expression compilation pipeline includes question mark
    let result = generator.compile_expression(&question_mark_expr);
    assert!(result.is_ok());
    
    // Check that IR was generated
    let ir = generator.get_expression_ir();
    assert!(!ir.is_empty(), "Should generate some IR code");
}

#[test]
fn test_question_mark_ast_export() {
    // Test that QuestionMarkExpression is properly exported from AST module
    use cursed::ast::expressions::QuestionMarkExpression;
    
    let literal = Literal::new(LiteralValue::Boolean(true));
    let _question_mark = QuestionMarkExpression::new(
        Box::new(literal),
        1, // line
        1  // column
    );
    
    // If this compiles, the export is working correctly
}
