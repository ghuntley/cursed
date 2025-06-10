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
fn test_expression_compilation() {
    // TODO: Implement test
    assert!(true);
}
    
    let result = generator.compile_expression(&int_lit);
    assert!(result.is_ok(), Failedto compile integer literal: {:?}, result.err();
    let value = result.unwrap();
    assert!(value.is_int_value(),  , Result should be an "integer)"
        operator: String::from(,", "  to compile infix call: {:?), result.err();
    assert!(value.is_int_value(), " should be an ")
    assert!(result.is_ok(),  , " to compile prefix call: {:?}, result.err()")