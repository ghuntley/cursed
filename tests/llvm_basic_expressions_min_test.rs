//! Minimal test for basic expression compilation in the LLVM code generator

use cursed::ast::expressions::literals::{IntegerLiteral};
use cursed::ast::traits::Expression;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::ExpressionCompilation;
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;
use inkwell::context::Context;
use std::path::PathBuf;

#[test]
fn test_integer_literal_expression() {
    let context = Context::create();
    let mut generator = LlvmCodeGenerator::new(&context, "test", PathBuf::from("test.csd"));

    // Create a function context with a basic block for the builder
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.module().add_function("test_int", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(entry_block);

    // Create a simple expression: 42
    let int_lit = IntegerLiteral {
        token: Token::new(TokenType::Int, "42"),
        value: 42,
    };

    // Generate code for the expression
    let result = generator.compile_expression(&int_lit);
    assert!(result.is_ok(), "Failed to compile integer literal: {:?}", result.err());

    // Check the result is an i32 with value 42
    let value = result.unwrap();
    assert!(value.is_int_value(), "Result should be an integer");

    let int_value = value.into_int_value();
    assert_eq!(int_value.get_zero_extended_constant().unwrap(), 42);
}