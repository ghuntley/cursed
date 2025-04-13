//! Test file for verifying loop context in LLVM code generation

use cursed::ast::control_flow::{BreakStatement, ContinueStatement, ForStatement};
use cursed::ast::expressions::literals::BooleanLiteral;
use cursed::ast::statements::BlockStatement;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;
use inkwell::context::Context;
use std::path::PathBuf;

#[test]
fn test_break_continue_context_management() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test", PathBuf::from("test.csd"));

    // Create a function
    let void_type = context.void_type();
    let fn_type = void_type.fn_type(&[], false);
    let function = generator.module().add_function("test_loop_context", fn_type, None);
    generator.set_current_function(function);
    
    let entry_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(entry_block);

    // Create a simple loop
    let true_condition = BooleanLiteral {
        token: Token::new(TokenType::True, "based").token_literal(),
        value: true,
    };
    
    // Create basic loop with break
    let break_stmt = BreakStatement {
        token: Token::new(TokenType::Break, "ghosted").token_literal(),
    };
    
    let loop_body = BlockStatement {
        token: Token::new(TokenType::LBrace, "{").token_literal(),
        statements: vec![Box::new(break_stmt)],
    };
    
    let loop_stmt = ForStatement {
        token: Token::new(TokenType::For, "bestie").token_literal(),
        init: None,
        condition: Some(Box::new(true_condition)),
        post: None,
        body: Box::new(loop_body),
    };
    
    // Before running the loop, verify that there are no loop contexts
    assert!(generator.current_loop_context().is_none(), "Should start with no loop contexts");
    
    // Compile the loop
    let loop_result = generator.compile_for_statement(&loop_stmt);
    assert!(loop_result.is_ok(), "Loop compilation failed");
    
    // After the loop, the loop context should have been properly cleaned up
    assert!(generator.current_loop_context().is_none(), "Loop context should be popped after loop completes");
    
    // Create a nested loop with a new boolean literal
    let inner_break = BreakStatement {
        token: Token::new(TokenType::Break, "ghosted").token_literal(),
    };
    
    let inner_body = BlockStatement {
        token: Token::new(TokenType::LBrace, "{").token_literal(),
        statements: vec![Box::new(inner_break)],
    };
    
    let inner_condition = BooleanLiteral {
        token: Token::new(TokenType::True, "based").token_literal(),
        value: true,
    };
    
    let inner_loop = ForStatement {
        token: Token::new(TokenType::For, "bestie").token_literal(),
        init: None,
        condition: Some(Box::new(inner_condition)),
        post: None,
        body: Box::new(inner_body),
    };
    
    let outer_body = BlockStatement {
        token: Token::new(TokenType::LBrace, "{").token_literal(),
        statements: vec![Box::new(inner_loop)],
    };
    
    let outer_condition = BooleanLiteral {
        token: Token::new(TokenType::True, "based").token_literal(),
        value: true,
    };
    
    let outer_loop = ForStatement {
        token: Token::new(TokenType::For, "bestie").token_literal(),
        init: None,
        condition: Some(Box::new(outer_condition)),
        post: None,
        body: Box::new(outer_body),
    };
    
    // Compile the nested loops
    let nested_result = generator.compile_for_statement(&outer_loop);
    assert!(nested_result.is_ok(), "Nested loop compilation failed");
    
    // After the outer loop, the loop context should have been properly cleaned up
    assert!(generator.current_loop_context().is_none(), "All loop contexts should be popped");
    
    // Add a return void instruction to terminate the function
    let return_void = generator.builder().build_return(None);
    assert!(return_void.is_ok(), "Failed to build return");

    // Verify the module
    let verify_result = generator.module().verify();
    assert!(verify_result.is_ok(), "Module verification failed");
}