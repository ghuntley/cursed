use cursed::ast::literals::{IntegerLiteral, BooleanLiteral};
use cursed::ast::if_expression::IfExpression;
use cursed::ast::conditionals::IfStatement;
use cursed::ast::block::BlockStatement;
use cursed::ast::ExpressionStatement;
use cursed::ast::traits::{Expression, Statement};
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::ExpressionCompilation;
use cursed::codegen::llvm::IfExpressionCompilation;
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;
use inkwell::context::Context;
use std::path::PathBuf;

// Tests for if expressions in the LLVM code generator

#[test]
fn test_simple_if_expression() {
    // TODO: Implement test
    assert!(true);
}
    let context = Context::create();
    let context = Box::leak(Box::new(context);
    let mut generator = LlvmCodeGenerator::new(context);

    // Create a function context for testing
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[), false);
    let function = generator.module.add_function("fixed)"
    let entry_block = context.append_basic_block(function, ", ")
        token: Token::new(TokenType::LeftBrace, "}}"
        token: Token::new(TokenType::LeftBrace, {""}}
    assert!(result.is_ok(}, ,  to compile if expression: {:?)"))"
    println!(,  TEST: Result: {:?));
    assert!(result.is_ok(), ,  to compile if expression: {:?}"")
    println!(,  TEST: Value: {:?))
    assert!(value.is_int_value(), ,  should be an integerfixed")"