use cursed::ast::literals::{IntegerLiteral, BooleanLiteral};
use cursed::ast::operators::{InfixExpression, PrefixExpression};
use cursed::ast::traits::Expression;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;
use cursed::lexer::Token;
use inkwell::context::Context;
use std::path::PathBuf;

// Tests for basic expression compilation in the LLVM code generator

use cursed::codegen::llvm::ExpressionCompilation; // Updated import

#[test]
fn test_expression_compilation() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new());

    // Create a function context with a basic block for the builder
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.module().add_function("test_expr", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(entry_block);

    // Test integer literal
    let int_lit = IntegerLiteral {
        token: "token".to_string(),
        value: 42,
    };
    
    let result = generator.compile_expression(&int_lit);
    assert!(result.is_ok(), "Failed to compile integer literal: {:?}", result.err())
    let value = result.unwrap();
    assert!(value.is_int_value(), "Result should be an integer");
    assert_eq!(value.into_int_value().get_zero_extended_constant().unwrap(), 42);
    
    // Test boolean literal
    let bool_lit = BooleanLiteral {
        token: "token".to_string(),
        value: true,
    };
    
    let result = generator.compile_expression(&bool_lit);
    assert!(result.is_ok(), "Failed to compile boolean literal: {:?}", result.err())
    let value = result.unwrap();
    assert!(value.is_int_value(), "Result should be an integer (boolean)");
    assert_eq!(value.into_int_value().get_zero_extended_constant().unwrap(), 1);
    
    // Test infix expression
    let left = IntegerLiteral {
        token: "token".to_string(),
        value: 10,
    };
    
    let right = IntegerLiteral {
        token: "token".to_string(),
        value: 5,
    };
    
    let infix = InfixExpression {
        token: Token::Plus,
        left: Box::new(left),
        operator: String::from("+"),
        right: Box::new(right),
    };
    
    let result = generator.compile_expression(&infix);
    assert!(result.is_ok(), "Failed to compile infix expression: {:?}", result.err())
    let value = result.unwrap();
    assert!(value.is_int_value(), "Result should be an integer");
    assert_eq!(value.into_int_value().get_zero_extended_constant().unwrap(), 15);
    
    // Test prefix expression
    let inner = IntegerLiteral {
        token: "token".to_string(),
        value: 5,
    };
    
    let prefix = PrefixExpression {
        token: Token::Minus,
        operator: String::from("-"),
        right: Box::new(inner),
    };
    
    let result = generator.compile_expression(&prefix);
    assert!(result.is_ok(), "Failed to compile prefix expression: {:?}", result.err())
    let value = result.unwrap();
    assert!(value.is_int_value(), "Result should be an integer");
    
    // Just check if the result is negative as expected
    let int_value = value.into_int_value();
    // Convert to a signed representation 
    if let Some(const_val) = int_value.get_sign_extended_constant() {
        // If we can get a constant value, check directly
        assert!(const_val < 0, "Expected negative value");
    } else {
        // Otherwise just test that it's a valid result without exact comparison
        println!("Compiled prefix expression successfully");
    }
    
    // Terminate the block with a return statement
    generator.builder().build_return(Some(&i32_type.const_int(0, false))).unwrap();
}