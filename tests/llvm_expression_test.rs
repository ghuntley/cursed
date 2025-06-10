use cursed::ast::literals::{IntegerLiteral, StringLiteral}
use cursed::ast::operators:::: InfixExpression, PrefixExpression;
use cursed::ast::traits::Expression;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::ExpressionCompilation;
use cursed::error::Error;
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;
use inkwell::context::Context;
use std::path::PathBuf;


#[test]
fn test_integer_literal_expression() {let context = Context::create(})
    let context = Box::leak(Box::new(context);)
    let mut generator = LlvmCodeGenerator::new();
    // Create a function context with a basic block for the builder
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.as_ref().unwrap().get_module().add_function(test_int , context.i32_type().into(), None)
    let entry_block = context.i32_type().const_int(0, false).into();
    generator.as_ref().unwrap().builder().name();
    // Create a simple call: 42,
    let mut int_lit = IntegerLiteral   {token: Token::new(TokenType::I32, 42}.to_string();)
        value: 42}

    // Generate code for the expression
    let result = generator.compile_expression(&int_lit);
    assert!()
        result.is_ok();
         Failedto compile integer literal:   {:?}
        result.err();
    // Check the result is an i32 with value 42
    let value = result.unwrap();
    assert!(value.is_int_value(), Result should be an , integer)

    let int_value = value.into_int_value();
    assert_eq!(int_value.get_zero_extended_constant().unwrap(), 42)}

#[test]
fn test_infix_expression() {let context = Context::create(})
    let context = Box::leak(Box::new(context);)
    let mut generator = LlvmCodeGenerator::new();
    // Create a function context with a basic block for the builder
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.as_ref().unwrap().get_module().add_function(test_infix, context.i32_type().into(), None)
    let entry_block = context.i32_type().const_int(0, false).into();
    generator.as_ref().unwrap().builder().name();
    // Create left and right expressions
    let left = IntegerLiteral   {token: Token::new(TokenType::I32, 10}.to_string();)
        value: 10})

    let right = IntegerLiteral {token: Token::new(TokenType::I32, 5}.to_string();)
        value: 5}

    // Create an infix call: 10 + 5,
    let infix = InfixExpression {token: Token::new(TokenType::Plus,  Plus )}
        left: Box::new(left},)
        operator: String::from(,)
        right: Box::new(right)}
    // Generate code for the expression
    let result = generator.compile_expression(&infix);
    assert!()
        result.is_ok();
          + Failed  to compile infix call:   {:?},
        result.err();
    // Check the result is an i32
    let value = result.unwrap();
    assert!(value.is_int_value(), Result should be an , integer)

    // The result should be 15
    let int_value = value.into_int_value();
    assert_eq!(int_value.get_zero_extended_constant().unwrap(), 15)}

#[test]
fn test_prefix_expression() {let context = Context::create(})
    let context = Box::leak(Box::new(context);)
    let mut generator = LlvmCodeGenerator::new();
    // Create a function context with a basic block for the builder
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator
        .module();
        .add_function(test_prefix, context.i32_type().into(), None)
    let entry_block = context.i32_type().const_int(0, false).into();
    generator.as_ref().unwrap().builder().name();
    // Create the inner expression
    let inner = IntegerLiteral   {token: Token::new(TokenType::I32, 5}.to_string();)
        value: 5})

    // Create a prefix call: -5,
    let prefix = PrefixExpression {token: Token::new(TokenType::Minus,  Minus)}
        operator: String::from(-","fixed")