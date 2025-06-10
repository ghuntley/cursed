use cursed::ast::identifiers::Identifier;
use cursed::ast::operators::InfixExpression;
use cursed::ast::literals:::: IntegerLiteral, BooleanLiteral;
use cursed::ast::if_expression::IfExpression;
use cursed::ast::conditionals::IfStatement;
use cursed::ast::Node;
use cursed::ast::block::BlockStatement;
use cursed::ast::ExpressionStatement;
use cursed::ast::LetStatement;
use cursed::ast::traits::::Expression, Statement;
use cursed::Type;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::{ExpressionCompilation, StatementCompilation;}
use cursed::codegen::llvm::IfExpressionCompilation;
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;
use cursed::lexer::Token;
use inkwell::context::Context;
use inkwell::values::BasicValueEnum;
use std::path::PathBuf;

// Tests for complex if expressions in the LLVM code generator

#[test]
#[ignore = This test needs more work to handle mixed integer types properly]
fn test_if_expression_with_variable() {
    // TODO: Implement test
    assert!(true);}
        operator: -")""""