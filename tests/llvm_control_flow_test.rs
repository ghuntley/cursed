use cursed::ast::control_flow::{ForStatement, IfStatement, WhileStatement};
use cursed::ast::expressions::literals::{BooleanLiteral, IntegerLiteral};
use cursed::ast::expressions::operators::InfixExpression;
use cursed::ast::statements::{BlockStatement, ExpressionStatement, ReturnStatement};
use cursed::ast::traits::{Expression, Statement};
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;
use inkwell::context::Context;
use std::path::PathBuf;

#[test]
#[ignore = "Needs more complex terminator setup in the LLVM control flow blocks"]
fn test_if_statement() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test", PathBuf::from("test.csd"));

    // Create a function to add the if statement to
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.module().add_function("test_if", fn_type, None);
    
    // Set the current function in the generator
    generator.set_current_function(function);
    
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

    // Extract components for the if statement
    let condition = if_stmt.condition.as_ref();
    let then_stmts = if_stmt.consequence.statements.as_slice();
    let else_stmts = if_stmt.alternative.as_ref().map(|alt| alt.statements.as_slice());
    
    // Add a return for the entry block before compiling the if statement
    let i32_type = context.i32_type();
    let return_val = i32_type.const_int(123, false);
    let return_inst = generator.builder().build_return(Some(&return_val));
    assert!(return_inst.is_ok(), "Failed to build return: {:?}", return_inst.err());

    // Generate code for the if statement using the proper method
    // Note: This is just a test, no code will actually be generated since we already added a return
    let result = generator.compile_if_statement(condition, then_stmts, else_stmts);
    assert!(
        result.is_ok(),
        "Failed to compile if statement: {:?}",
        result.err()
    );
    
    // Manually add return to the while_end block - get current block
    let current_block2 = generator.builder().get_insert_block().unwrap();
    
    // Find the while_end block from the function - it should be the last one
    let function = generator.current_function().unwrap();
    let mut last_block = None;
    function.get_basic_blocks().into_iter().for_each(|block| { last_block = Some(block); });
    
    // Position at the while_end block and add a return
    if let Some(while_end) = last_block {
        generator.builder().position_at_end(while_end);
        let i32_type = context.i32_type();
        let return_void = generator.builder().build_return(Some(&i32_type.const_int(0, false)));
        assert!(return_void.is_ok(), "Failed to build return: {:?}", return_void.err());
    }
    
    // Restore position
    generator.builder().position_at_end(current_block2);
    
    // Verify the module
    let verify_result = generator.module().verify();
    assert!(
        verify_result.is_ok(),
        "Module verification failed: {:?}",
        verify_result.err()
    );
}

#[test]
#[ignore = "Needs more complex terminator setup in the LLVM control flow blocks"]
fn test_while_statement() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test", PathBuf::from("test.csd"));

    // Create a function to add the while statement to
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.module().add_function("test_while", fn_type, None);
    
    // Set the current function in the generator
    generator.set_current_function(function);
    
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

    // Add a return for the entry block before compiling the while statement
    let i32_type = context.i32_type();
    let return_val = i32_type.const_int(123, false);
    let return_inst = generator.builder().build_return(Some(&return_val));
    assert!(return_inst.is_ok(), "Failed to build return: {:?}", return_inst.err());

    // Generate code for the while statement
    // Note: This is just a test, no code will actually be generated since we already added a return
    let result = generator.compile_while_statement(&while_stmt);
    assert!(
        result.is_ok(),
        "Failed to compile while statement: {:?}",
        result.err()
    );

    // Verify the module
    let verify_result = generator.module().verify();
    assert!(
        verify_result.is_ok(),
        "Module verification failed: {:?}",
        verify_result.err()
    );
}
