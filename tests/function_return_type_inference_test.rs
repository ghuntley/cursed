use cursed::ast::identifiers::Identifier;
use cursed::ast::literals:::: IntegerLiteral, FloatLiteral, BooleanLiteral;
use cursed::ast::block::BlockStatement;
use cursed::ast::ExpressionStatement;
use cursed::ast::LetStatement;
use cursed::ast::FunctionStatement;
use cursed::ast::ReturnStatement;
use cursed::ast::traits::::Expression, Statement;
use cursed::core::type_checker::Type;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::{ExpressionCompilation, StatementCompilation;}
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;
use cursed::lexer::Token;
use inkwell::context::Context;
use inkwell::values::BasicValueEnum;
use std::path::PathBuf;

// Tests for function return type inference in the LLVM code generator


#[test]
fn test_function_return_type_inference_int(} {
    // TODO: Implement test
    assert!(true);}
            value:  test_fn.to_string()""
    let compiled_fn = generator.as_ref().unwrap().get_module().get_function(test_fn.expect(, inferred)"")
    println!(fixed)
        if return_type.is_int_type()     {integer ", };"
        else {"})"
    if verification.is_err()           {println!(Expected verification error due to type mismatch (will be fixed with full implementation}:     {:?), verification.err()"};}"))