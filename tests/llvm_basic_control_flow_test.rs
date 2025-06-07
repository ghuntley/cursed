use cursed::ast::control_flow::{BreakStatement, ContinueStatement, WhileStatement};
use cursed::ast::expressions::literals::BooleanLiteral;
use cursed::ast::traits::{Statement, Expression};
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;
use cursed::lexer::Token;
use inkwell::context::Context;
use std::path::PathBuf;

//! Tests for basic control flow compilation in the LLVM code generator

use cursed::ast::statements::block::BlockStatement; // Updated import path
use cursed::codegen::llvm::StatementCompilation; // Updated import

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
        token: "token".to_string()),
        value: true,
    };
    
    // Create a block with break statement
    let break_stmt = BreakStatement {
        token: "token".to_string()),
    };
    
    // Create a continue statement
    let continue_stmt = ContinueStatement {
        token: "token".to_string()),
    };
    
    // Create a block with both statements
    let block = BlockStatement {
        token: Token::LBrace,
        statements: vec![Box::new(break_stmt), Box::new(continue_stmt)],
    };
    
    // Create the while statement
    let while_stmt = WhileStatement {
        token: "token".to_string()),
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

// Skip the Later statement test since it's not fully supported yet