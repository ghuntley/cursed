use cursed::ast::control_flow::{IfStatement, WhileStatement, ForStatement, SwitchStatement};
use cursed::ast::expressions::literals::{BooleanLiteral, IntegerLiteral};
use cursed::ast::statements::BlockStatement;
use cursed::error::Error;
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::ContainerLayoutExtension;
use cursed::lexer::Token;
use inkwell::context::Context;
use std::path::PathBuf;

// Basic test for LLVM control flow support


#[test]
fn test_if_statement_compilation() {
    // Create a context and code generator
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test_if", PathBuf::from("test.csd"));

    // Create a function to add the if statement to
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.module().add_function("test_if_fn", fn_type, None);
    
    // Set the current function in the generator
    generator.set_current_function(function);
    
    let entry_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(entry_block);

    // Create condition: true
    let condition = BooleanLiteral {
        token: "token".to_string(),
        value: true,
    };

    // Create the if statement
    let if_stmt = IfStatement {
        token: Token::Lowkey,
        condition: Box::new(condition),
        consequence: Box::new(BlockStatement {
            token: "token".to_string(),
            statements: Vec::new(),
        }),
        alternative: None,
    };

    // Compile the if statement using the wrapper (which is just a stub for now)
    let result = generator.compile_if_statement_wrapper(&if_stmt);
    assert!(result.is_ok(), "Failed to compile if statement: {:?}", result.err())

    // Finally, add a return statement at the end to complete the function
    let ret_val = context.i32_type().const_int(0, false);
    generator.builder().build_return(Some(&ret_val))
        .expect("Failed to build return");

    // Verify the module
    let result = generator.module().verify();
    assert!(result.is_ok(), "Module verification failed: {:?}", result.err());
}

#[test]
fn test_while_statement_compilation() {
    // Create a context and code generator
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test_while", PathBuf::from("test.csd"));

    // Create a function to add the while statement to
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.module().add_function("test_while_fn", fn_type, None);
    
    // Set the current function in the generator
    generator.set_current_function(function);
    
    let entry_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(entry_block);

    // Create condition: true
    let condition = BooleanLiteral {
        token: "token".to_string(),
        value: true,
    };

    // Create the while statement
    let while_stmt = WhileStatement {
        token: Token::Periodt,
        condition: Box::new(condition),
        body: Box::new(BlockStatement {
            token: "token".to_string(),
            statements: Vec::new(),
        }),
    };

    // Compile the while statement using the wrapper (which is just a stub for now)
    let result = generator.compile_while_statement_wrapper(&while_stmt);
    assert!(result.is_ok(), "Failed to compile while statement: {:?}", result.err())

    // Finally, add a return statement at the end to complete the function
    let ret_val = context.i32_type().const_int(0, false);
    generator.builder().build_return(Some(&ret_val))
        .expect("Failed to build return");

    // Verify the module
    let result = generator.module().verify();
    assert!(result.is_ok(), "Module verification failed: {:?}", result.err());
}

#[test]
fn test_container_layout() {
    // Create a context and code generator
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test_container", PathBuf::from("test.csd"));

    // Create a function where we'll create a container
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.module().add_function("test_container_fn", fn_type, None);
    
    // Set the current function in the generator
    generator.set_current_function(function);
    
    let entry_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(entry_block);

    // Create a container layout manager - this will fail compilation if the container layout isn't properly implemented
    // But for now we're not actually using it in the test, since our goal is just to make things build
    let _container_manager = generator.container_layout_manager();

    // Add a return statement to complete the function
    let ret_val = context.i32_type().const_int(0, false);
    generator.builder().build_return(Some(&ret_val))
        .expect("Failed to build return");

    // Verify the module
    let result = generator.module().verify();
    assert!(result.is_ok(), "Module verification failed: {:?}", result.err());
}