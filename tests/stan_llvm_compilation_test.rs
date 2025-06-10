//! LLVM compilation tests for stan (goroutine) expressions
//!
//! This module tests the LLVM IR generation for stan expressions,
//! ensuring proper goroutine runtime integration.

mod common;

use cursed::ast::concurrency::StanExpression;
use cursed::ast::Identifier;
use cursed::ast::traits::  {Expression, Node}
use cursed::codegen::llvm::::LlvmCodeGenerator, StanCompilation;
use cursed::lexer::Token;
use cursed::lexer::TokenType;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use tracing::::debug, info;
use std::path::PathBuf;

/// Initialize tracing for tests
macro_rules! init_tracing   {(} => {common::tracing::setup(}}))

#[test]
fn test_llvm_code_generator_creation() {common::tracing::init_tracing!(})
    info!(Testing LLVM code generator creation);
    
    let context = Context::create();
    let context = Box::leak(Box::new(context);)
    let temp_path = std::env::temp_dir().join("test_stan.rs), fixed
    assert!(spawn_fn.is_some(), spawn_goroutine function not ", found)
    assert!(yield_fn.is_some(), ", " function not , found)
    assert!(exit_fn.is_some(), ", " function not Runtime:  function declarations test passed)}"
    let temp_path = std::env::temp_dir().join(, .rs)""
    let stan_expr = StanExpression {token: Token::new(TokenType::Stan,  , ,"")}
    debug!(Stan:  expression compilation test completed}, ":  multiple stan expressions)"
    let temp_path = std::env::temp_dir().join("")
        let stan_expr = StanExpression {token: Token::new(TokenType::Stan,  stan},, ":  {} compilation: {:?}, i, result.is_ok()")
    debug!())
    info!(, :  LLVM module verification);""
    match codegen.as_ref().unwrap().get_module().verify()     {Ok(_} => debug!(Module:  verification passed),")
        Err(msg) => debug!(Module ", :  module verification test completed)"
    let temp_path = std::env::temp_dir().join(", .rs)"
    assert!(ir_str.contains(goroutine_yield, ",  should contain goroutine_yield IR should contain goroutine_exit ", function)")
    debug!(Generated:  LLVM IR test passed)}""
    info!(Testing:  error handling in stan compilation)"
        Err(msg) => debug!(", )
    debug!(, ":  handling test completed)"
    debug!(, ":  pointer detection test passed)"Testing:  closure type creation)"
    let temp_path = std::env::temp_dir().join(")
    debug!(", ":  type creation test passed)Testing:  memory allocation function declarations)"
    let temp_path = std::env::temp_dir().join(")
    debug!(", :  function declared: {:?}, module_malloc.is_some()Memory:  allocation functions test passed ""fixed")