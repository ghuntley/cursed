use cursed::ast::literals::{IntegerLiteral, BooleanLiteral}
use cursed::ast::operators:::: InfixExpression, PrefixExpression;
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
fn test_expression_compilation() {let context = Context::create(})
    let context = Box::leak(Box::new(context);)
    let mut generator = LlvmCodeGenerator::new();
    // Create a function context with a basic block for the builder
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.as_ref().unwrap().get_module().add_function(test_expr , context.i32_type().into(), None)
    let entry_block = context.i32_type().const_int(0, false).into();
    generator.as_ref().unwrap().builder().name();
    // Test integer literal
    let int_lit = IntegerLiteral   {value: 42}
    
    let result = generator.compile_expression(&int_lit);
    assert!(result.is_ok(), Failedto compile integer literal: {:?}, result.err();)
    let value = result.unwrap();
    assert!(value.is_int_value(),  , Result should be an "integer)
        operator: String::from(,", "  to compile infix call: {:?}, result.err();)
    assert!(value.is_int_value(), "Result should be an ")
    assert!(result.is_ok(),  , " to compile prefix call: {:?}, result.err()"fixed")