//! Tests for complex if expressions in the LLVM code generator

use cursed::ast::expressions::identifiers::Identifier;
use cursed::ast::expressions::operators::InfixExpression;
use cursed::ast::expressions::literals::{IntegerLiteral, BooleanLiteral};
use cursed::ast::expressions::if_expression::IfExpression;
use cursed::ast::control_flow::conditionals::IfStatement;
use cursed::ast::Node;
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
fn test_if_expression_with_variable() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test_if_var", PathBuf::from("test_if_var.csd"));

    // Create a function for testing
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.module().add_function("test_if_var", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(entry_block);
    generator.set_current_function(function);
    
    // Set up a variable 'x' with value 10
    let x_ident = Identifier {
        token: Token::new(TokenType::Identifier, "x").token_literal(),
        value: "x".to_string(),
    };
    
    let x_value = IntegerLiteral {
        token: Token::new(TokenType::Int, "10").token_literal(),
        value: 10,
    };
    
    let let_stmt = LetStatement {
        token: Token::new(TokenType::Sus, "sus").token_literal(),
        name: x_ident.clone(),
        value: Some(Box::new(x_value)),
        type_annotation: None,
    };
    
    // Compile the let statement to create the variable
    let result = generator.compile_statement(&let_stmt);
    assert!(result.is_ok(), "Failed to compile let statement: {:?}", result.err());
    
    // Create a condition that compares x > 5
    let x_expr = x_ident.clone();
    let five = IntegerLiteral {
        token: Token::new(TokenType::Int, "5").token_literal(),
        value: 5,
    };
    
    // Make sure we use consistent integer types
    let five = IntegerLiteral {
        token: Token::new(TokenType::Int, "5").token_literal(),
        value: 5,
    };

    // Convert to int for comparison
    let condition = InfixExpression {
        token: Token::new(TokenType::Gt, ">"),
        left: Box::new(x_expr),
        operator: ">".to_string(),
        right: Box::new(five),
    };
    
    // Create the then expression: x + 20
    let then_x = x_ident.clone();
    let twenty = IntegerLiteral {
        token: Token::new(TokenType::Int, "20").token_literal(),
        value: 20,
    };
    
    let then_expr = InfixExpression {
        token: Token::new(TokenType::Plus, "+"),
        left: Box::new(then_x),
        operator: "+".to_string(),
        right: Box::new(twenty),
    };
    
    // Wrap in an expression statement
    let then_stmt = ExpressionStatement {
        token: then_expr.token_literal(),
        expression: Some(Box::new(then_expr)),
    };
    
    // Create the else expression: x - 5
    let else_x = x_ident.clone();
    let five_2 = IntegerLiteral {
        token: Token::new(TokenType::Int, "5").token_literal(),
        value: 5,
    };
    
    let else_expr = InfixExpression {
        token: Token::new(TokenType::Minus, "-"),
        left: Box::new(else_x),
        operator: "-".to_string(),
        right: Box::new(five_2),
    };
    
    // Wrap in an expression statement
    let else_stmt = ExpressionStatement {
        token: else_expr.token_literal(),
        expression: Some(Box::new(else_expr)),
    };
    
    // Create block statements
    let consequence = BlockStatement {
        token: Token::new(TokenType::LBrace, "{").token_literal(),
        statements: vec![Box::new(then_stmt)],
    };
    
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
    assert!(result.is_ok(), "Failed to compile if expression: {:?}", result.err());
    
    // Since x = 10 and condition is x > 5, the result should be x + 20 = 30
    let value = result.unwrap();
    assert!(value.is_int_value(), "Result should be an integer");
    
    // Get the result from LLVM
    let ret_val = generator.builder().build_return(Some(&value));
    assert!(ret_val.is_ok(), "Failed to build return: {:?}", ret_val.err());
    
    // Verify the module
    let verification = generator.module().verify();
    assert!(verification.is_ok(), "Module verification failed: {:?}", verification.err());
}

#[test]
fn test_nested_if_expressions() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test_nested_if", PathBuf::from("test_nested_if.csd"));

    // Create a function for testing
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.module().add_function("test_nested_if", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(entry_block);
    generator.set_current_function(function);
    
    // Create outer condition: true
    let outer_condition = BooleanLiteral {
        token: Token::new(TokenType::True, "true").token_literal(),
        value: true,
    };
    
    // Create inner condition: false
    let inner_condition = BooleanLiteral {
        token: Token::new(TokenType::Lit, "false").token_literal(),
        value: false,
    };
    
    // Create values for different branches
    let value_1 = IntegerLiteral {
        token: Token::new(TokenType::Int, "1").token_literal(),
        value: 1,
    };
    
    let value_2 = IntegerLiteral {
        token: Token::new(TokenType::Int, "2").token_literal(),
        value: 2,
    };
    
    let value_3 = IntegerLiteral {
        token: Token::new(TokenType::Int, "3").token_literal(),
        value: 3,
    };
    
    // Wrap values in expression statements
    let stmt_1 = ExpressionStatement {
        token: value_1.token.clone(),
        expression: Some(Box::new(value_1)),
    };
    
    let stmt_2 = ExpressionStatement {
        token: value_2.token.clone(),
        expression: Some(Box::new(value_2)),
    };
    
    let stmt_3 = ExpressionStatement {
        token: value_3.token.clone(),
        expression: Some(Box::new(value_3)),
    };
    
    // Create inner if blocks
    let inner_consequence = BlockStatement {
        token: Token::new(TokenType::LBrace, "{").token_literal(),
        statements: vec![Box::new(stmt_1)],
    };
    
    let inner_alternative = BlockStatement {
        token: Token::new(TokenType::LBrace, "{").token_literal(),
        statements: vec![Box::new(stmt_2)],
    };
    
    let inner_if = IfStatement {
        token: Token::new(TokenType::If, "if").token_literal(),
        condition: Box::new(inner_condition),
        consequence: Box::new(inner_consequence),
        alternative: Some(Box::new(inner_alternative)),
    };
    
    // Wrap inner if in expression statement
    let inner_if_expr = IfExpression::new(inner_if);
    let inner_if_stmt = ExpressionStatement {
        token: Token::new(TokenType::If, "if").token_literal(),
        expression: Some(Box::new(inner_if_expr)),
    };
    
    // Create outer if blocks
    let outer_consequence = BlockStatement {
        token: Token::new(TokenType::LBrace, "{").token_literal(),
        statements: vec![Box::new(inner_if_stmt)],
    };
    
    let outer_alternative = BlockStatement {
        token: Token::new(TokenType::LBrace, "{").token_literal(),
        statements: vec![Box::new(stmt_3)],
    };
    
    let outer_if = IfStatement {
        token: Token::new(TokenType::If, "if").token_literal(),
        condition: Box::new(outer_condition),
        consequence: Box::new(outer_consequence),
        alternative: Some(Box::new(outer_alternative)),
    };
    
    // Create outer if expression
    let outer_if_expr = IfExpression::new(outer_if);
    
    // Compile the nested if expressions
    let result = generator.compile_if_expression(&outer_if_expr);
    assert!(result.is_ok(), "Failed to compile nested if expressions: {:?}", result.err());
    
    // Since outer is true and inner is false, the result should be 2
    let value = result.unwrap();
    assert!(value.is_int_value(), "Result should be an integer");
    
    // Get the result from LLVM
    let ret_val = generator.builder().build_return(Some(&value));
    assert!(ret_val.is_ok(), "Failed to build return: {:?}", ret_val.err());
    
    // Verify the module
    let verification = generator.module().verify();
    assert!(verification.is_ok(), "Module verification failed: {:?}", verification.err());
}