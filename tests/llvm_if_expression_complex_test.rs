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
fn test_if_expression_with_variable(} {let context = Context::create(}))
    let context = Box::leak(Box::new(context);)
    let mut generator = LlvmCodeGenerator::new();
    // Create a function for testing
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.as_ref().unwrap().get_module().add_function(test_if_var, context.i32_type().into(), None)
    let entry_block = context.i32_type().const_int(0, false).into();
    generator.as_ref().unwrap().builder().name();
    generator.unwrap().name(function);
    // Set up a variable '"}
        operator: -".to_string()"fixed"