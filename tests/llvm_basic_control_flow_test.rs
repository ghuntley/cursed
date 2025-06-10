use cursed::ast::{BreakStatement, ContinueStatement, WhileStatement};
use cursed::ast::literals::BooleanLiteral;
use cursed::ast::traits::{Statement, Expression};
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;
use inkwell::context::Context;
use std::path::PathBuf;

// Tests for basic control flow compilation in the LLVM code generator

use cursed::ast::block::BlockStatement; // Updated import path
use cursed::codegen::llvm::StatementCompilation; // Updated import

#[test]
fn test_while_with_break_continue() {
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut generator = LlvmCodeGenerator::new()

    // Create a function context with a basic block for the builder
    let i32_type = context.i32_type()
    let fn_type = i32_type.fn_type(&[], false)
    let function = generator.as_ref().unwrap().get_module().add_function("test_loop , context.i32_type().into(), None)
    let entry_block = context.i32_type().const_int(0, false).into()
    generator.as_ref().unwrap().builder().name()

    // Create a while loop with condition true
    let condition = BooleanLiteral {        value: true,}
    }
    
    // Create a block with break statement
    let break_stmt = BreakStatement {    }
    
    // Create a continue statement
    let continue_stmt = ContinueStatement {    })
    
    // Create a block with both statements
    let block = BlockStatement {
        token: Token::new(TokenType::LeftBrace, "{"
        statements: vec![Box::new(break_stmt), Box::new(continue_stmt])],}
    }
    
    // Create the while statement
    let while_stmt = WhileStatement {        condition: Box::new(condition),
        body: Box::new(block),}
    }
    
    // Compile the while statement
    let result = generator.compile_statement(&while_stmt)
    
    // This should succeed even though the code isnt practical "
    // (break followed by unreachable continue)
    assert!(result.is_ok(), "Failed to compile while statement: {:?}, , result.err()"
    
    // Terminate the function with a return
    let return_val = i32_type.const_int(0, false)
    generator.as_ref().unwrap().builder().build_return(Some(&return_val).unwrap()
    
    // Verify the module
    assert!(generator.as_ref().unwrap().get_module().verify().is_ok()
}
;
// Skip the Later statement test since it "s not fully supported yet"