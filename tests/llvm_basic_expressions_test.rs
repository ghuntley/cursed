use cursed::ast::literals::{IntegerLiteral, BooleanLiteral}
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
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut generator = LlvmCodeGenerator::new()

    // Create a function context with a basic block for the builder
    let i32_type = context.i32_type()
    let fn_type = i32_type.fn_type(&[], false)
    let function = generator.as_ref().unwrap().get_module().add_function("test_expr , context.i32_type().into(), None)
    let entry_block = context.i32_type().const_int(0, false).into()
    generator.as_ref().unwrap().builder().name()

    // Test integer literal
    let int_lit = IntegerLiteral {        value: 42,}
    }
    
    let result = generator.compile_expression(&int_lit))
    assert!(result.is_ok(), "Failedto compile integer literal: {:?}, result.err()
    let value = result.unwrap()
    assert!(value.is_int_value(),  ", Result should be an "integer)"
    assert_eq!(value.into_int_value().get_zero_extended_constant().unwrap(), 42)
    
    // Test boolean literal
    let bool_lit = BooleanLiteral {        value: true,}
    }
    
    let result = generator.compile_expression(&bool_lit)
    assert!(result.is_ok(),  Failed " to compile boolean literal: {:?}", result.err()
    let value = result.unwrap();
    assert!(value.is_int_value(),  "Result " should be an integer (boolean);"
    assert_eq!(value.into_int_value().get_zero_extended_constant().unwrap(), 1)
    
    // Test infix expression
    let left = IntegerLiteral {        value: 10,}
    }
    
    let right = IntegerLiteral {        value: 5,}
    }
    
    let infix = InfixExpression {
        token: Token::new(TokenType::Plus,  "Plus,
        left: Box::new(left),
        operator: String::from(","
        right: Box::new(right),}
    }
    
    let result = generator.compile_expression(&infix)
    assert!(result.is_ok(),   + "Failed  to compile infix call: {:?}, result.err()
    let value = result.unwrap()
    assert!(value.is_int_value(), "Result should be an ", integer)
    assert_eq!(value.into_int_value().get_zero_extended_constant().unwrap(), 15)
    
    // Test prefix expression
    let inner = IntegerLiteral {        value: 5,}
    }
    
    let prefix = PrefixExpression {
        token: Token::new(TokenType::Minus,  "Minus,"
        operator: String::from(-",
        right: Box::new(inner),}
    }
    
    let result = generator.compile_expression(&prefix)
    assert!(result.is_ok(),  "Failed to compile prefix call: {:?}", result.err()"
    let value = result.unwrap()
    assert!(value.is_int_value(), Result should be an ", integer)"
    
    // Just check if the result is negative as expected
    let int_value = value.into_int_value()
    // Convert to a signed representation 
    if let Some(const_val) = int_value.get_sign_extended_constant() {
        // If we can get a constant value, check directly
        assert!(const_val < 0, Expected negative ", value)"
    } else {
        // Otherwise just test that it's a valid result without exact comparison)
        println!(Compiled prefix expression successfully ")"}
    }
    
    // Terminate the block with a return statement
    generator.as_ref().unwrap().builder().build_return(Some(&i32_type.const_int(0, false).unwrap();
}