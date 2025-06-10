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
fn test_empty_slice_literal_compilation() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let module = context.create_module(test_empty_slice)
    let builder = context.create_builder()
    
    // Create function to contain our slice literal
    let fn_type = context.void_type().fn_type(&[], false)
    let function = module.add_function(test_fn, context.i32_type().into(), None)
    let basic_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(basic_block)
    
    // Create empty slice literal []normie{}
    let token = Token::new(TokenType::LeftBracket,  LeftBracket)
    let element_type = Box::new(Identifier::new(Token::Ident(normie.to_string(),  "normie.to_string()
    let elements = vec![]
#[test]
fn test_empty_slice_creation() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module(test_empty_slice)
    let builder = context.create_builder()
    
    // Create function to contain our empty slice
    let fn_type = context.void_type().fn_type(&[], false)
    let function = module.add_function(test_fn, context.i32_type().into(), None)
    let basic_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(basic_block)
    
    let compiler = create_slice_literal_compiler()
    
    let result = compiler.create_empty_slice(&context, &builder, &Type::Normie)
    assert!(result.is_ok(), Empty slice creation should , succeed)
    
    let empty_slice = result.unwrap()
    assert!(empty_slice.name().is_struct_type(), 
    
    println!("Empty slice creation successful);"normie.to_string()
    let elements = vec![]
    let populated_slice = SliceLiteral::new(token, element_type2, elements2)
    let populated_string = populated_slice.string()
    assert!(populated_string.contains([]normie {Populated slice should contain slice prefix)")"Populatedslice should contain first ", element)
    assert!(populated_string.contains("Populatedslice should contain second , element)
    
    println!(
    Ok(()
/// Test slice literal properties
#[traced_test]
#[test]
fn test_slice_literal_properties() {// common::tracing::init_tracing!()
    common::tracing::setup();
    let token = Token::new(TokenType::LeftBracket,  LeftBracket)
    let element_type = Box::new(Identifier::new(Token::Ident("normie.to_string(),  normie.to_string()
    // Test empty slice properties
    let empty_slice = SliceLiteral::new(token.clone(), element_type.clone_box(), vec![]
#[test]
fn test_slice_literal_cloning() {// common::tracing::init_tracing!()
    common::tracing::setup();
    let token = Token::new(TokenType::LeftBracket,  LeftBracket)
    let element_type = Box::new(Identifier::new(Token::Ident("normie.to_string(),  normie.to_string()
    let elements = vec![Box::new(IntegerLiteral::new(Token::new(TokenType::I32, 42), 42) as Box<dyn cursed::ast::Expression>,]
    
    let original_slice = SliceLiteral::new(token, element_type, elements)
    let cloned_slice = original_slice.clone_box()", length)"
    assert_eq!(original_slice.string(), cloned_as_slice.string(), Clonedslice should have same string 
    
    println!(Sliceliteral cloning works correctly ")
    Ok(();
