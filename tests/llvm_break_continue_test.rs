use cursed::ast::control_flow::{BreakStatement, ContinueStatement, ForStatement, WhileStatement};
use cursed::ast::expressions::literals::BooleanLiteral;
use cursed::ast::statements::{BlockStatement, ExpressionStatement};
use cursed::ast::traits::{Expression, Statement};
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;
use inkwell::context::Context;
use std::path::PathBuf;

#[test]
fn test_break_statement() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test", PathBuf::from("test.csd"));

    // Create a function to add the break statement to
    let void_type = context.void_type();
    let fn_type = void_type.fn_type(&[], false);
    let function = generator.module().add_function("test_break", fn_type, None);
    
    // Set the current function in the generator
    generator.set_current_function(function);
    
    let entry_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(entry_block);

    // Create a while loop with a break statement
    let condition = BooleanLiteral {
        token: Token::new(TokenType::True, "based").token_literal(),
        value: true,
    };

    // Create the break statement
    let break_stmt = BreakStatement {
        token: Token::new(TokenType::Break, "ghosted").token_literal(),
    };

    // Create body with the break statement
    let body = BlockStatement {
        token: Token::new(TokenType::LBrace, "{").token_literal(),
        statements: vec![Box::new(break_stmt)],
    };

    // Create the while statement
    let while_stmt = WhileStatement {
        token: Token::new(TokenType::While, "periodt").token_literal(),
        condition: Box::new(condition),
        body: Box::new(body),
    };

    // Generate code for the while statement with break
    // In the implementation of compile_while_statement, the loop context is already created
    // so we don't need to push it here
    let result = generator.compile_while_statement(&while_stmt);

    assert!(
        result.is_ok(),
        "Failed to compile while statement with break: {:?}",
        result.err()
    );

    // Add a return void instruction to terminate the function
    let return_void = generator.builder().build_return(None);
    assert!(return_void.is_ok(), "Failed to build return: {:?}", return_void.err());
    
    // Verify the module
    let verify_result = generator.module().verify();
    assert!(
        verify_result.is_ok(),
        "Module verification failed: {:?}",
        verify_result.err()
    );
}

#[test]
fn test_continue_statement() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test", PathBuf::from("test.csd"));

    // Create a function to add the continue statement to
    let void_type = context.void_type();
    let fn_type = void_type.fn_type(&[], false);
    let function = generator
        .module()
        .add_function("test_continue", fn_type, None);
    
    // Set the current function in the generator
    generator.set_current_function(function);
    
    let entry_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(entry_block);

    // Create a for loop with a continue statement
    let condition = BooleanLiteral {
        token: Token::new(TokenType::True, "based").token_literal(),
        value: true,
    };

    // Create the continue statement
    let continue_stmt = ContinueStatement {
        token: Token::new(TokenType::Continue, "simp").token_literal(),
    };

    // Create body with the continue statement
    let body = BlockStatement {
        token: Token::new(TokenType::LBrace, "{").token_literal(),
        statements: vec![Box::new(continue_stmt)],
    };

    // Create the for statement
    let for_stmt = ForStatement {
        token: Token::new(TokenType::For, "bestie").token_literal(),
        init: None,
        condition: Some(Box::new(condition)),
        post: None,
        body: Box::new(body),
    };

    // Generate code for the for statement with continue
    // In the implementation of compile_for_statement, the loop context is already created
    // so we don't need to push it here
    let result = generator.compile_for_statement(&for_stmt);

    assert!(
        result.is_ok(),
        "Failed to compile for statement with continue: {:?}",
        result.err()
    );
    
    // Add a return void instruction to terminate the function
    let return_void = generator.builder().build_return(None);
    assert!(return_void.is_ok(), "Failed to build return: {:?}", return_void.err());

    // Verify the module
    let verify_result = generator.module().verify();
    assert!(
        verify_result.is_ok(),
        "Module verification failed: {:?}",
        verify_result.err()
    );
}
