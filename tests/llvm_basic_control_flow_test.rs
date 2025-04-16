//! Tests for basic control flow compilation in the LLVM code generator

use cursed::ast::control_flow::{BreakStatement, ContinueStatement, WhileStatement};
use cursed::ast::statements::{BlockStatement, LaterStatement};
use cursed::ast::expressions::literals::IntegerLiteral;
use cursed::ast::expressions::operators::InfixExpression;
use cursed::ast::expressions::BooleanLiteral;
use cursed::ast::traits::{Statement, Expression};
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::statement::StatementCompilation;
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;
use inkwell::context::Context;
use std::path::PathBuf;

#[test]
fn test_while_with_break_continue() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test", PathBuf::from("test.csd"));

    // Create a function context with a basic block for the builder
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.module().add_function("test_loop", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(entry_block);

    // Create a while loop with condition true
    let condition = BooleanLiteral {
        token: Token::new(TokenType::True, "true").token_literal(),
        value: true,
    };
    
    // Create a block with break statement
    let break_stmt = BreakStatement {
        token: Token::new(TokenType::Break, "break"),
    };
    
    // Create a continue statement
    let continue_stmt = ContinueStatement {
        token: Token::new(TokenType::Continue, "continue"),
    };
    
    // Create a block with both statements
    let block = BlockStatement {
        token: Token::new(TokenType::LBrace, "{"),
        statements: vec![Box::new(break_stmt), Box::new(continue_stmt)],
    };
    
    // Create the while statement
    let while_stmt = WhileStatement {
        token: Token::new(TokenType::While, "while"),
        condition: Box::new(condition),
        body: Box::new(block),
    };
    
    // Compile the while statement
    let result = generator.compile_statement(&while_stmt);
    
    // This should succeed even though the code isn't practical
    // (break followed by unreachable continue)
    assert!(result.is_ok(), "Failed to compile while statement: {:?}", result.err());
    
    // Terminate the function with a return
    let return_val = i32_type.const_int(0, false);
    generator.builder().build_return(Some(&return_val)).unwrap();
    
    // Verify the module
    assert!(generator.module().verify().is_ok());
}

#[test]
fn test_later_statement() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test", PathBuf::from("test.csd"));

    // Create a function context with a basic block for the builder
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.module().add_function("test_later", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(entry_block);
    
    // Create a simple statement to be deferred
    let deferred_block = BlockStatement {
        token: Token::new(TokenType::LBrace, "{"),
        statements: vec![],  // Empty block for simplicity
    };
    
    // Create the later statement
    let later_stmt = LaterStatement {
        token: Token::new(TokenType::Later, "later"),
        statement: Box::new(deferred_block),
    };
    
    // Compile the later statement
    let result = generator.compile_statement(&later_stmt);
    assert!(result.is_ok(), "Failed to compile later statement: {:?}", result.err());
    
    // Terminate the function with a return
    let return_val = i32_type.const_int(0, false);
    generator.builder().build_return(Some(&return_val)).unwrap();
    
    // Verify the module
    assert!(generator.module().verify().is_ok());
}