//! Tests for if expressions with type inference in the LLVM code generator

use cursed::ast::expressions::identifiers::Identifier;
use cursed::ast::expressions::operators::InfixExpression;
use cursed::ast::expressions::literals::{IntegerLiteral, FloatLiteral, StringLiteral, BooleanLiteral};
use cursed::ast::expressions::if_expression::IfExpression;
use cursed::ast::control_flow::conditionals::IfStatement;
use cursed::ast::statements::block::BlockStatement;
use cursed::ast::statements::ExpressionStatement;
use cursed::ast::statements::declarations::LetStatement;
use cursed::ast::traits::{Expression, Statement};
use cursed::Type;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::{ExpressionCompilation, StatementCompilation};
use cursed::codegen::llvm::IfExpressionCompilation;
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;
use inkwell::context::Context;
use inkwell::values::BasicValueEnum;
use std::path::PathBuf;

#[test]
fn test_if_expression_with_mixed_types() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test_if_mixed_types", PathBuf::from("test_if_mixed_types.csd"));

    // Create a function for testing - use double return type since we expect float result
    let double_type = context.f64_type();
    let fn_type = double_type.fn_type(&[], false);
    let function = generator.module().add_function("test_if_mixed_types", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(entry_block);
    generator.set_current_function(function);
    
    // Create a condition: true
    let condition = BooleanLiteral {
        token: Token::new(TokenType::True, "true").token_literal(),
        value: true,
    };
    
    // Create the then expression: 42 (i32)
    let then_expr = IntegerLiteral {
        token: Token::new(TokenType::Int, "42").token_literal(),
        value: 42,
    };
    
    // Wrap in an expression statement
    let then_stmt = ExpressionStatement {
        token: Token::new(TokenType::Int, "42").token_literal(),
        expression: Some(Box::new(then_expr)),
    };
    
    // Create the else expression: 24.5 (f64)
    let else_expr = FloatLiteral {
        token: Token::new(TokenType::Float, "24.5").token_literal(),
        value: 24.5,
    };
    
    // Wrap in an expression statement
    let else_stmt = ExpressionStatement {
        token: Token::new(TokenType::Float, "24.5").token_literal(),
        expression: Some(Box::new(else_expr)),
    };
    
    // Create the BlockStatement for consequence
    let consequence = BlockStatement {
        token: Token::new(TokenType::LBrace, "{").token_literal(),
        statements: vec![Box::new(then_stmt)],
    };
    
    // Create the BlockStatement for alternative
    let alternative = BlockStatement {
        token: Token::new(TokenType::LBrace, "{").token_literal(),
        statements: vec![Box::new(else_stmt)],
    };
    
    // Create the IfStatement
    let if_stmt = IfStatement {
        token: Token::new(TokenType::If, "if").token_literal(),
        condition: Box::new(condition),
        consequence: Box::new(consequence),
        alternative: Some(Box::new(alternative)),
    };
    
    // Create the if expression adapter
    let if_expr = IfExpression::new(if_stmt);
    
    // Compile the if expression
    let result = generator.compile_if_expression(&if_expr);
    assert!(result.is_ok(), "Failed to compile if expression with mixed types: {:?}", result.err());
    
    // Get the result and verify it's proper type inference
    let value = result.unwrap();
    
    // The result should be a float since float is wider than integer
    assert!(value.is_float_value(), "Result should be a float value due to type inference");
    
    // Get the result from LLVM
    let ret_val = generator.builder().build_return(Some(&value));
    assert!(ret_val.is_ok(), "Failed to build return: {:?}", ret_val.err());
    
    // Verify the module
    let verification = generator.module().verify();
    assert!(verification.is_ok(), "Module verification failed: {:?}", verification.err());
}

#[test]
fn test_if_expression_with_string_and_int() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test_if_string_int", PathBuf::from("test_if_string_int.csd"));

    // Create a function for testing
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.module().add_function("test_if_string_int", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(entry_block);
    generator.set_current_function(function);
    
    // Create a condition: true
    let condition = BooleanLiteral {
        token: Token::new(TokenType::True, "true").token_literal(),
        value: true,
    };
    
    // Create the then expression: string literal
    let then_expr = StringLiteral {
        token: Token::new(TokenType::String, "\"Hello\"").token_literal(),
        value: "Hello".to_string(),
    };
    
    // Wrap in an expression statement
    let then_stmt = ExpressionStatement {
        token: Token::new(TokenType::String, "\"Hello\"").token_literal(),
        expression: Some(Box::new(then_expr)),
    };
    
    // Create the else expression: integer
    let else_expr = IntegerLiteral {
        token: Token::new(TokenType::Int, "42").token_literal(),
        value: 42,
    };
    
    // Wrap in an expression statement
    let else_stmt = ExpressionStatement {
        token: Token::new(TokenType::Int, "42").token_literal(),
        expression: Some(Box::new(else_expr)),
    };
    
    // Create the BlockStatement for consequence
    let consequence = BlockStatement {
        token: Token::new(TokenType::LBrace, "{").token_literal(),
        statements: vec![Box::new(then_stmt)],
    };
    
    // Create the BlockStatement for alternative
    let alternative = BlockStatement {
        token: Token::new(TokenType::LBrace, "{").token_literal(),
        statements: vec![Box::new(else_stmt)],
    };
    
    // Create the IfStatement
    let if_stmt = IfStatement {
        token: Token::new(TokenType::If, "if").token_literal(),
        condition: Box::new(condition),
        consequence: Box::new(consequence),
        alternative: Some(Box::new(alternative)),
    };
    
    // Create the if expression adapter
    let if_expr = IfExpression::new(if_stmt);
    
    // Compile the if expression
    let result = generator.compile_if_expression(&if_expr);
    
    // This test should detect and handle incompatible types - we expect an error
    // In a more sophisticated type system, we might use type inference to coerce the int to a string
    assert!(result.is_err(), "Should fail due to incompatible types");
    
    // Check that the error message mentions incompatible types
    if let Err(err) = result {
        assert!(err.to_string().contains("incompatible"), "Error message should mention incompatible types");
        println!("Got expected error: {}", err);
    }
}