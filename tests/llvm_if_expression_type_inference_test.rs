//! Tests for if expressions with type inference in the LLVM code generator

use cursed::ast::expressions::identifiers::Identifier;
use cursed::ast::expressions::operators::InfixExpression;
use cursed::ast::expressions::literals::{IntegerLiteral, FloatLiteral, StringLiteral, BooleanLiteral};
use cursed::ast::expressions::if_expression::IfExpression;
use cursed::ast::control_flow::conditionals::IfStatement;
use cursed::ast::statements::block::BlockStatement;
use cursed::ast::statements::ExpressionStatement;
use cursed::ast::statements::declarations::LetStatement;
use cursed::ast::traits::{Expression, Statement};

use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::{ExpressionCompilation, StatementCompilation};
use cursed::codegen::llvm::IfExpressionCompilation;
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;
use inkwell::context::Context;
use inkwell::values::BasicValueEnum;
use std::path::PathBuf;

#[test]
fn test_assignment_type_inference() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test_assignment_inference", PathBuf::from("test_assignment_inference.csd"));

    // Create a function for testing with float return type
    let f64_type = context.f64_type();
    let fn_type = f64_type.fn_type(&[], false);
    let function = generator.module().add_function("test_assignment_inference", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(entry_block);
    generator.set_current_function(function);
    
    // Create a variable without explicit type annotation
    let var_name = Identifier {
        token: "x".to_string(),
        value: "x".to_string(),
    };
    
    // Declare and initialize with integer
    let let_stmt = LetStatement {
        token: "sus".to_string(),
        name: var_name.clone(),
        type_annotation: None, // No explicit type - should infer from value
        value: Some(Box::new(IntegerLiteral {
            token: "42".to_string(),
            value: 42,
        })),
    };
    
    // Compile the declaration
    let result = generator.compile_statement(&let_stmt);
    assert!(result.is_ok(), "Failed to compile let statement: {:?}", result.err());
    
    // Now assign a float value to the variable
    let assign_expr = InfixExpression {
        token: Token::Assign,
        left: Box::new(Identifier {
            token: "x".to_string(),
            value: "x".to_string(),
        }),
        operator: "=".to_string(),
        right: Box::new(FloatLiteral {
            token: "3.14".to_string(),
            value: 3.14,
        }),
    };
    
    // Compile the assignment expression
    let assign_result = generator.compile_expression(&assign_expr);
    
    // Currently the implementation doesn't support type coercion in assignments
    // so we expect an error about incompatible types
    assert!(assign_result.is_err(), "Should fail due to type mismatch");
    
    // Check that the error message mentions type mismatch
    if let Err(err) = assign_result {
        assert!(err.to_string().contains("Type mismatch"), "Error message should mention type mismatch");
        println!("Got expected error: {}", err);
    }
    
    // Return a dummy value and verify the module
    let ret_val = generator.builder().build_return(Some(&generator.context().f64_type().const_float(0.0)));
    assert!(ret_val.is_ok(), "Failed to build return: {:?}", ret_val.err());
    
    let verification = generator.module().verify();
    assert!(verification.is_ok(), "Module verification failed: {:?}", verification.err());
}

#[test]
fn test_assignment_type_coercion() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test_assignment_coercion", PathBuf::from("test_assignment_coercion.csd"));

    // Create a function for testing with float return type
    let f64_type = context.f64_type();
    let fn_type = f64_type.fn_type(&[], false);
    let function = generator.module().add_function("test_assignment_coercion", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(entry_block);
    generator.set_current_function(function);
    
    // Create a variable with explicit float type annotation
    let var_name = Identifier {
        token: "x".to_string(),
        value: "x".to_string(),
    };
    
    // Declare a float variable with float type annotation
    let let_stmt = LetStatement {
        token: "sus".to_string(),
        name: var_name.clone(),
        type_annotation: Some(Token::Meal), // Explicitly float (f64)
        value: Some(Box::new(FloatLiteral {
            token: "0.0".to_string(),
            value: 0.0,
        })),
    };
    
    // Print to help debug
    println!("DEBUG: Creating variable with type annotation 'meal' (f64)");
    
    // Compile the declaration
    let result = generator.compile_statement(&let_stmt);
    assert!(result.is_ok(), "Failed to compile let statement: {:?}", result.err());
    
    // Now assign an integer value to the float variable - should be coerced
    let assign_expr = InfixExpression {
        token: Token::Assign,
        left: Box::new(Identifier {
            token: "x".to_string(),
            value: "x".to_string(),
        }),
        operator: "=".to_string(),
        right: Box::new(IntegerLiteral {
            token: "42".to_string(),
            value: 42,
        }),
    };
    
    // Compile the assignment expression
    let assign_result = generator.compile_expression(&assign_expr);
    
    // With proper type coercion, this should succeed
    assert!(assign_result.is_ok(), "Assignment with type coercion failed: {:?}", assign_result.err());
    
    // The result should be the coerced integer value (now a float)
    if let Ok(value) = assign_result {
        assert!(value.is_float_value(), "Result should be a float value after coercion");
    }
    
    // Add debug print for variable type
    println!("DEBUG: After assignment, checking variable type");
    
    // Load the variable's value to verify it's properly coerced
    let load_expr = Identifier {
        token: "x".to_string(),
        value: "x".to_string(),
    };
    
    let load_result = generator.compile_expression(&load_expr);
    assert!(load_result.is_ok(), "Failed to load variable: {:?}", load_result.err());
    
    let loaded_value = load_result.unwrap();
    println!("DEBUG: Loaded value type: {}", 
             if loaded_value.is_float_value() { "float" } 
             else if loaded_value.is_int_value() { "integer" } 
             else { "other" });
    assert!(loaded_value.is_float_value(), "Loaded value should be a float after coercion");
    
    // Return the loaded value and verify the module
    let ret_val = generator.builder().build_return(Some(&loaded_value));
    assert!(ret_val.is_ok(), "Failed to build return: {:?}", ret_val.err());
    
    let verification = generator.module().verify();
    assert!(verification.is_ok(), "Module verification failed: {:?}", verification.err());
}

#[test]
fn test_if_expression_with_assignment_type_inference() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test_if_assignment_inference", PathBuf::from("test_if_assignment_inference.csd"));

    // Create a function for testing
    let f64_type = context.f64_type();
    let fn_type = f64_type.fn_type(&[], false);
    let function = generator.module().add_function("test_if_assignment_inference", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(entry_block);
    generator.set_current_function(function);
    
    // Create a condition: true
    let condition = BooleanLiteral {
        token: "true".to_string(),
        value: true,
    };
    
    // Create the then expression: 42.0 (explicitly as float)
    let then_expr = FloatLiteral {
        token: "42.0".to_string(),
        value: 42.0,
    };
    
    // Wrap in an expression statement
    let then_stmt = ExpressionStatement {
        token: "42.0".to_string(),
        expression: Some(Box::new(then_expr)),
    };
    
    // Create the else expression: 99.5 (f64)
    let else_expr = FloatLiteral {
        token: "99.5".to_string(),
        value: 99.5,
    };
    
    // Wrap in an expression statement
    let else_stmt = ExpressionStatement {
        token: "99.5".to_string(),
        expression: Some(Box::new(else_expr)),
    };
    
    // Create the BlockStatement for consequence
    let consequence = BlockStatement {
        token: Token::LBrace,
        statements: vec![Box::new(then_stmt)],
    };
    
    // Create the BlockStatement for alternative
    let alternative = BlockStatement {
        token: Token::LBrace,
        statements: vec![Box::new(else_stmt)],
    };
    
    // Create the IfStatement
    let if_stmt = IfStatement {
        token: "if".to_string(),
        condition: Box::new(condition),
        consequence: Box::new(consequence),
        alternative: Some(Box::new(alternative)),
    };
    
    // Create the if expression adapter
    let if_expr = IfExpression::new(if_stmt);
    
    // Compile the if expression
    let result = generator.compile_if_expression(&if_expr);
    assert!(result.is_ok(), "Failed to compile if expression with assignment type inference: {:?}", result.err());
    
    // Get the result and verify it's proper type inference
    let value = result.unwrap();
    
    // Both branches should result in a float value
    assert!(value.is_float_value(), "Result should be a float value due to type inference");
    
    // Get the result from LLVM - Important: This serves as a terminator for the merge block
    let ret_val = generator.builder().build_return(Some(&value));
    assert!(ret_val.is_ok(), "Failed to build return: {:?}", ret_val.err());
    
    // Verify the module
    let verification = generator.module().verify();
    assert!(verification.is_ok(), "Module verification failed: {:?}", verification.err());
}

#[test]
fn test_if_expression_with_mixed_types() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test_if_mixed_types", PathBuf::from("test_if_mixed_types.csd"));

    // Create a function for testing - use double return type since we expect float result
    let double_type = context.f64_type();
    let fn_type = double_type.fn_type(&[], false);
    let function = generator.module().add_function("test_if_mixed_types", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(entry_block);
    generator.set_current_function(function);
    
    // Create a condition: true
    let condition = BooleanLiteral {
        token: "true".to_string(),
        value: true,
    };
    
    // Create the then expression: 42 (i32)
    let then_expr = IntegerLiteral {
        token: "42".to_string(),
        value: 42,
    };
    
    // Wrap in an expression statement
    let then_stmt = ExpressionStatement {
        token: "42".to_string(),
        expression: Some(Box::new(then_expr)),
    };
    
    // Create the else expression: 24.5 (f64)
    let else_expr = FloatLiteral {
        token: "24.5".to_string(),
        value: 24.5,
    };
    
    // Wrap in an expression statement
    let else_stmt = ExpressionStatement {
        token: "24.5".to_string(),
        expression: Some(Box::new(else_expr)),
    };
    
    // Create the BlockStatement for consequence
    let consequence = BlockStatement {
        token: Token::LBrace,
        statements: vec![Box::new(then_stmt)],
    };
    
    // Create the BlockStatement for alternative
    let alternative = BlockStatement {
        token: Token::LBrace,
        statements: vec![Box::new(else_stmt)],
    };
    
    // Create the IfStatement
    let if_stmt = IfStatement {
        token: "if".to_string(),
        condition: Box::new(condition),
        consequence: Box::new(consequence),
        alternative: Some(Box::new(alternative)),
    };
    
    // Create the if expression adapter
    let if_expr = IfExpression::new(if_stmt);
    
    // Compile the if expression
    let result = generator.compile_if_expression(&if_expr);
    assert!(result.is_ok(), "Failed to compile if expression with mixed types: {:?}", result.err());
    
    // Get the result and verify it's proper type inference
    let value = result.unwrap();
    
    // The result should be a float since float is wider than integer
    assert!(value.is_float_value(), "Result should be a float value due to type inference");
    
    // Get the result from LLVM
    let ret_val = generator.builder().build_return(Some(&value));
    assert!(ret_val.is_ok(), "Failed to build return: {:?}", ret_val.err());
    
    // Verify the module
    let verification = generator.module().verify();
    assert!(verification.is_ok(), "Module verification failed: {:?}", verification.err());
}

#[test]
fn test_if_expression_with_string_and_int() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test_if_string_int", PathBuf::from("test_if_string_int.csd"));

    // Create a function for testing
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.module().add_function("test_if_string_int", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(entry_block);
    generator.set_current_function(function);
    
    // Create a condition: true
    let condition = BooleanLiteral {
        token: "true".to_string(),
        value: true,
    };
    
    // Create the then expression: string literal
    let then_expr = StringLiteral {
        token: "Hello".to_string(),
        value: "Hello".to_string(),
    };
    
    // Wrap in an expression statement
    let then_stmt = ExpressionStatement {
        token: "Hello".to_string(),
        expression: Some(Box::new(then_expr)),
    };
    
    // Create the else expression: integer
    let else_expr = IntegerLiteral {
        token: "42".to_string(),
        value: 42,
    };
    
    // Wrap in an expression statement
    let else_stmt = ExpressionStatement {
        token: "42".to_string(),
        expression: Some(Box::new(else_expr)),
    };
    
    // Create the BlockStatement for consequence
    let consequence = BlockStatement {
        token: Token::LBrace,
        statements: vec![Box::new(then_stmt)],
    };
    
    // Create the BlockStatement for alternative
    let alternative = BlockStatement {
        token: Token::LBrace,
        statements: vec![Box::new(else_stmt)],
    };
    
    // Create the IfStatement
    let if_stmt = IfStatement {
        token: "if".to_string(),
        condition: Box::new(condition),
        consequence: Box::new(consequence),
        alternative: Some(Box::new(alternative)),
    };
    
    // Create the if expression adapter
    let if_expr = IfExpression::new(if_stmt);
    
    // Compile the if expression
    let result = generator.compile_if_expression(&if_expr);
    
    // This test should detect and handle incompatible types - we expect an error
    // In a more sophisticated type system, we might use type inference to coerce the int to a string
    assert!(result.is_err(), "Should fail due to incompatible types");
    
    // Check that the error message mentions incompatible types
    if let Err(err) = result {
        assert!(err.to_string().contains("incompatible"), "Error message should mention incompatible types");
        println!("Got expected error: {}", err);
    }
}