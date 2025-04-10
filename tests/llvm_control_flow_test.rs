use std::path::PathBuf;
use inkwell::context::Context;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::ast::traits::{Statement, Expression};
use cursed::ast::control_flow::{IfStatement, WhileStatement, ForStatement};
use cursed::ast::expressions::literals::{IntegerLiteral, BooleanLiteral};
use cursed::ast::expressions::operators::InfixExpression;
use cursed::ast::statements::{BlockStatement, ExpressionStatement, ReturnStatement};
use cursed::error::Error;
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;

#[test]
fn test_if_statement() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test", PathBuf::from("test.csd"));
    
    // Create a function to add the if statement to
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.module().add_function("test_if", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(entry_block);
    
    // Create condition: true
    let condition = BooleanLiteral {
        token: Token::new(TokenType::True, "based").token_literal(),
        value: true,
    };
    
    // Create then block with return 42
    let return_value = IntegerLiteral {
        token: Token::new(TokenType::Int, "42").token_literal(),
        value: 42,
    };
    let return_stmt = ReturnStatement {
        token: Token::new(TokenType::Return, "yolo").token_literal(),
        return_value: Some(Box::new(return_value)),
    };
    let then_block = BlockStatement {
        token: Token::new(TokenType::LBrace, "{").token_literal(),
        statements: vec![Box::new(return_stmt)],
    };
    
    // Create else block with return 0
    let else_return_value = IntegerLiteral {
        token: Token::new(TokenType::Int, "0").token_literal(),
        value: 0,
    };
    let else_return_stmt = ReturnStatement {
        token: Token::new(TokenType::Return, "yolo").token_literal(),
        return_value: Some(Box::new(else_return_value)),
    };
    let else_block = BlockStatement {
        token: Token::new(TokenType::LBrace, "{").token_literal(),
        statements: vec![Box::new(else_return_stmt)],
    };
    
    // Create the if statement
    let if_stmt = IfStatement {
        token: Token::new(TokenType::If, "lowkey").token_literal(),
        condition: Box::new(condition),
        consequence: Box::new(then_block),
        alternative: Some(Box::new(else_block)),
    };
    
    // Generate code for the if statement
    let result = generator.compile_if_statement(&if_stmt);
    assert!(result.is_ok(), "Failed to compile if statement: {:?}", result.err());
    
    // Verify the module
    let verify_result = generator.module().verify();
    assert!(verify_result.is_ok(), "Module verification failed: {:?}", verify_result.err());
}

#[test]
fn test_while_statement() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test", PathBuf::from("test.csd"));
    
    // Create a function to add the while statement to
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.module().add_function("test_while", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(entry_block);
    
    // Create condition: true
    let condition = BooleanLiteral {
        token: Token::new(TokenType::True, "based").token_literal(),
        value: true,
    };
    
    // Create body with a return statement
    let return_value = IntegerLiteral {
        token: Token::new(TokenType::Int, "42").token_literal(),
        value: 42,
    };
    let return_stmt = ReturnStatement {
        token: Token::new(TokenType::Return, "yolo").token_literal(),
        return_value: Some(Box::new(return_value)),
    };
    let body = BlockStatement {
        token: Token::new(TokenType::LBrace, "{").token_literal(),
        statements: vec![Box::new(return_stmt)],
    };
    
    // Create the while statement
    let while_stmt = WhileStatement {
        token: Token::new(TokenType::While, "periodt").token_literal(),
        condition: Box::new(condition),
        body: Box::new(body),
    };
    
    // Generate code for the while statement
    let result = generator.compile_while_statement(&while_stmt);
    assert!(result.is_ok(), "Failed to compile while statement: {:?}", result.err());
    
    // Verify the module
    let verify_result = generator.module().verify();
    assert!(verify_result.is_ok(), "Module verification failed: {:?}", verify_result.err());
}