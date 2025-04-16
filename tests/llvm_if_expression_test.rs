//! Tests for if expressions in the LLVM code generator

use cursed::ast::expressions::literals::{IntegerLiteral, BooleanLiteral};
use cursed::ast::expressions::IfExpression;
use cursed::ast::statements::ExpressionStatement;
use cursed::ast::traits::{Expression, Statement};
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::ExpressionCompilation;
use cursed::codegen::llvm::IfExpressionCompilation;
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;
use inkwell::context::Context;
use std::path::PathBuf;

#[test]
fn test_simple_if_expression() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test", PathBuf::from("test.csd"));

    // Create a function context for testing
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.module().add_function("test_if", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(entry_block);
    
    // Create a simple condition: true
    let condition = BooleanLiteral {
        token: Token::new(TokenType::True, "true").token_literal(),
        value: true,
    };
    
    // Create the then expression: 42
    let then_expr = IntegerLiteral {
        token: Token::new(TokenType::Int, "42").token_literal(),
        value: 42,
    };
    
    // Wrap in an expression statement
    let then_stmt = ExpressionStatement {
        token: Token::new(TokenType::Int, "42").token_literal(),
        expression: Some(Box::new(then_expr)),
    };
    
    // Create the else expression: 24
    let else_expr = IntegerLiteral {
        token: Token::new(TokenType::Int, "24").token_literal(),
        value: 24,
    };
    
    // Wrap in an expression statement
    let else_stmt = ExpressionStatement {
        token: Token::new(TokenType::Int, "24").token_literal(),
        expression: Some(Box::new(else_expr)),
    };
    
    // Create the if expression
    let if_expr = IfExpression {
        token: Token::new(TokenType::If, "if").token_literal(),
        condition: Box::new(condition),
        consequence: vec![Box::new(then_stmt)],
        alternative: Some(vec![Box::new(else_stmt)]),
    };
    
    // Compile the if expression
    let result = generator.compile_if_expression(&if_expr);
    assert!(result.is_ok(), "Failed to compile if expression: {:?}", result.err());
    
    // Since the condition is true, the result should be 42
    let value = result.unwrap();
    assert!(value.is_int_value(), "Result should be an integer");
    
    let int_value = value.into_int_value();
    assert_eq!(int_value.get_zero_extended_constant().unwrap(), 42);
}