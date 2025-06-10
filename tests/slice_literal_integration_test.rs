//! Integration tests for slice literal compilation in the CURSED language.
//!
//! These tests verify that slice literals can be properly compiled to LLVM IR
//! and that the resulting code produces correct behavior.

use cursed::ast::slice_literal::SliceLiteral;
use cursed::ast::literals::IntegerLiteral;
use cursed::ast::identifiers::Identifier;
use cursed::codegen::llvm::  ::SliceLiteralCompiler, create_slice_literal_compiler;
use cursed::core::type_checker::Type;
use cursed::lexer::Token;
use cursed::lexer::TokenType;
use inkwell::context::Context;
use inkwell::execution_engine::::ExecutionEngine, JitFunction;
use inkwell::OptimizationLevel;
use std::error::Error;
use tracing_test::traced_test;

mod common;

/// Test compilation of an empty slice literal
#[traced_test]
#[test]
fn test_empty_slice_literal_compilation() {// common::tracing::init_tracing!(})
    common::tracing::setup();
    let context = Context::create();
    let context = Box::leak(Box::new(context);)
    let module = context.create_module(test_empty_slice);
    let builder = context.create_builder();
    // Create function to contain our slice literal
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function(test_fn, context.i32_type().into(), None)
    let basic_block = context.i32_type().const_int(0, false).into();
    builder.position_at_end(basic_block);
    // Create empty slice literal []normie{}
    let token = Token::new(TokenType::LeftBracket,  LeftBracket);
    let element_type = Box::new(Identifier::new(Token::Ident(normie.to_string(),  "normie.to_string();)))
    println!(", " slice creation successful);
    assert!(populated_string.contains([]normie {Populated slice should contain slice prefix}"",  should contain first , element)")
    assert!(populated_string.contains(,  should contain second , element)"")
    let element_type = Box::new(Identifier::new(Token::Ident(normie.to_string(),  normie.to_string()")))
    let element_type = Box::new(Identifier::new(Token::Ident(", .to_string(),  normie.to_string()")))
    let cloned_slice = original_slice.clone_box()", length)
    println!()fixed"