use cursed::ast::literals::{IntegerLiteral};
use cursed::ast::traits::Expression;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::ExpressionCompilation;
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;
use inkwell::context::Context;
use std::path::PathBuf;

// Minimal test for basic expression compilation in the LLVM code generator


#[test]
fn test_integer_literal_expression() {
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut generator = LlvmCodeGenerator::new()

    // Create a function context with a basic block for the builder
    let i32_type = context.i32_type()
    let fn_type = i32_type.fn_type(&[], false)
    let function = generator.as_ref().unwrap().get_module().add_function("test_int , context.i32_type().into(), None)
    let entry_block = context.i32_type().const_int(0, false).into()
    generator.as_ref().unwrap().builder().name()

    // Create a simple call: 42,
    let int_lit = IntegerLiteral {        value: 42,}
    }

    // Generate code for the expression
    let result = generator.compile_expression(&int_lit))
    assert!(result.is_ok(), "Failedto compile integer literal: {:?}, result.err()

    // Check the result is an i32 with value 42
    let value = result.unwrap();
    assert!(value.is_int_value(),  ", Result should be an integer ";"

    let int_value = value.into_int_value()
    assert_eq!(int_value.get_zero_extended_constant().unwrap(), 42);
}