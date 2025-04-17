//! Tests for function return type inference in the LLVM code generator

use cursed::ast::expressions::identifiers::Identifier;
use cursed::ast::expressions::literals::{IntegerLiteral, FloatLiteral, BooleanLiteral};
use cursed::ast::statements::block::BlockStatement;
use cursed::ast::statements::ExpressionStatement;
use cursed::ast::statements::declarations::LetStatement;
use cursed::ast::declarations::FunctionStatement;
use cursed::ast::statements::declarations::ReturnStatement;
use cursed::ast::traits::{Expression, Statement};
use cursed::Type;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::{ExpressionCompilation, StatementCompilation};
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;
use inkwell::context::Context;
use inkwell::values::BasicValueEnum;
use std::path::PathBuf;

#[test]
#[ignore = "Function return type inference not yet implemented"]
fn test_function_return_type_inference_int() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test_function_return_int", PathBuf::from("test_function_return_int.csd"));

    // Create a function with no explicit return type annotation
    let fn_name = Identifier {
        token: Token::new(TokenType::Identifier, "test_fn").token_literal(),
        value: "test_fn".to_string(),
    };
    
    // Return statement with integer literal
    let return_stmt = ReturnStatement {
        token: Token::new(TokenType::Return, "return").token_literal(),
        return_value: Some(Box::new(IntegerLiteral {
            token: Token::new(TokenType::Int, "42").token_literal(),
            value: 42,
        })),
    };
    
    // Create function body
    let body = BlockStatement {
        token: Token::new(TokenType::LBrace, "{").token_literal(),
        statements: vec![Box::new(return_stmt)],
    };
    
    // Create function with no return type annotation
    let function = FunctionStatement {
        token: Token::new(TokenType::Sus, "sus").token_literal(),
        parameters: vec![],
        body: body,
        name: fn_name.clone(),
        return_type: None, // No explicit return type - should infer from return statements
        type_parameters: vec![],
        generic_constraints: vec![],
    };
    
    // Declare the function
    let result = generator.compile_statement(&function);
    assert!(result.is_ok(), "Failed to compile function with inferred return type: {:?}", result.err());
    
    // Verify that the function's return type was inferred as i32
    let compiled_fn = generator.module().get_function("test_fn").expect("Function should exist");
    let fn_type = compiled_fn.get_type();
    let return_type = fn_type.get_return_type();
    
    assert!(return_type.is_some(), "Return type should be inferred");
    let return_type = return_type.unwrap();
    // Currently fails because return type inference isn't implemented yet
    // This test defines the expected behavior for when it is implemented
    assert!(return_type.is_int_type(), "Return type should be inferred as integer");
    
    // Verify the module
    let verification = generator.module().verify();
    assert!(verification.is_ok(), "Module verification failed: {:?}", verification.err());
}

#[test]
#[ignore = "Function return type inference not yet implemented"]
fn test_function_return_type_inference_float() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test_function_return_float", PathBuf::from("test_function_return_float.csd"));

    // Create a function with no explicit return type annotation
    let fn_name = Identifier {
        token: Token::new(TokenType::Identifier, "test_fn").token_literal(),
        value: "test_fn".to_string(),
    };
    
    // Return statement with float literal
    let return_stmt = ReturnStatement {
        token: Token::new(TokenType::Return, "return").token_literal(),
        return_value: Some(Box::new(FloatLiteral {
            token: Token::new(TokenType::Float, "3.14").token_literal(),
            value: 3.14,
        })),
    };
    
    // Create function body
    let body = BlockStatement {
        token: Token::new(TokenType::LBrace, "{").token_literal(),
        statements: vec![Box::new(return_stmt)],
    };
    
    // Create function with no return type annotation
    let function = FunctionStatement {
        token: Token::new(TokenType::Sus, "sus").token_literal(),
        parameters: vec![],
        body: body,
        name: fn_name.clone(),
        return_type: None, // No explicit return type - should infer from return statements
        type_parameters: vec![],
        generic_constraints: vec![],
    };
    
    // Declare the function
    let result = generator.compile_statement(&function);
    assert!(result.is_ok(), "Failed to compile function with inferred return type: {:?}", result.err());
    
    // Verify that the function's return type was inferred as f64
    let compiled_fn = generator.module().get_function("test_fn").expect("Function should exist");
    let fn_type = compiled_fn.get_type();
    let return_type = fn_type.get_return_type();
    
    assert!(return_type.is_some(), "Return type should be inferred");
    let return_type = return_type.unwrap();
    // Currently fails because return type inference isn't implemented yet
    // This test defines the expected behavior for when it is implemented
    assert!(return_type.is_float_type(), "Return type should be inferred as float");
    
    // Verify the module
    let verification = generator.module().verify();
    assert!(verification.is_ok(), "Module verification failed: {:?}", verification.err());
}

#[test]
#[ignore = "Function return type inference not yet implemented"]
fn test_function_return_type_inference_mixed() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test_function_return_mixed", PathBuf::from("test_function_return_mixed.csd"));

    // Create a function with no explicit return type annotation
    let fn_name = Identifier {
        token: Token::new(TokenType::Identifier, "test_fn").token_literal(),
        value: "test_fn".to_string(),
    };
    
    // Create a condition for if statement
    let condition = BooleanLiteral {
        token: Token::new(TokenType::True, "true").token_literal(),
        value: true,
    };
    
    // Return statement with integer literal (first branch)
    let return_int = ReturnStatement {
        token: Token::new(TokenType::Return, "return").token_literal(),
        return_value: Some(Box::new(IntegerLiteral {
            token: Token::new(TokenType::Int, "42").token_literal(),
            value: 42,
        })),
    };
    
    // Return statement with float literal (second branch)
    let return_float = ReturnStatement {
        token: Token::new(TokenType::Return, "return").token_literal(),
        return_value: Some(Box::new(FloatLiteral {
            token: Token::new(TokenType::Float, "3.14").token_literal(),
            value: 3.14,
        })),
    };
    
    // Create if statement for the function body
    let if_stmt = cursed::ast::control_flow::conditionals::IfStatement {
        token: Token::new(TokenType::If, "if").token_literal(),
        condition: Box::new(condition),
        consequence: Box::new(BlockStatement {
            token: Token::new(TokenType::LBrace, "{").token_literal(),
            statements: vec![Box::new(return_int)],
        }),
        alternative: Some(Box::new(BlockStatement {
            token: Token::new(TokenType::LBrace, "{").token_literal(), 
            statements: vec![Box::new(return_float)],
        })),
    };
    
    // Create function body with the if statement
    let body = BlockStatement {
        token: Token::new(TokenType::LBrace, "{").token_literal(),
        statements: vec![Box::new(if_stmt)],
    };
    
    // Create function with no return type annotation
    let function = FunctionStatement {
        token: Token::new(TokenType::Sus, "sus").token_literal(),
        parameters: vec![],
        body: body,
        name: fn_name.clone(),
        return_type: None, // No explicit return type - should infer from return statements
        type_parameters: vec![],
        generic_constraints: vec![],
    };
    
    // Declare the function
    let result = generator.compile_statement(&function);
    assert!(result.is_ok(), "Failed to compile function with mixed return types: {:?}", result.err());
    
    // Verify that the function's return type was inferred as f64 (the wider type)
    let compiled_fn = generator.module().get_function("test_fn").expect("Function should exist");
    let fn_type = compiled_fn.get_type();
    let return_type = fn_type.get_return_type();
    
    assert!(return_type.is_some(), "Return type should be inferred");
    let return_type = return_type.unwrap();
    // Currently fails because return type inference isn't implemented yet
    // This test defines the expected behavior for when it is implemented
    assert!(return_type.is_float_type(), "Return type should be inferred as float (wider type)");
    
    // Verify the module
    let verification = generator.module().verify();
    assert!(verification.is_ok(), "Module verification failed: {:?}", verification.err());
}