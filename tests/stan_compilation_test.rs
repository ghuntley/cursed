use cursed::lexer::TokenType;
use cursed::lexer::Lexer;
//! Integration tests for stan (goroutine) compilation
//!
//! This module tests the complete compilation pipeline for stan expressions,
//! including parsing, AST creation, and LLVM IR generation.

mod common;

use cursed::ast::concurrency::StanExpression;
use cursed::ast::  {Identifier, FunctionLiteral, CallExpression}
use cursed::ast::traits::{Expression, Node}
use cursed::codegen::llvm::{LlvmCodeGenerator, StanCompilation}
use cursed::lexer::::Lexer, Token;
use cursed::parser::Parser;
use cursed::parser::Precedence;
use cursed::error::Error;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::execution_engine::::ExecutionEngine, JitFunction;
use inkwell::OptimizationLevel;
use tracing::{debug, info, instrument}

/// Initialize tracing for tests
macro_rules! init_tracing   {(} => {common::tracing::setup(}}))

#[test]
fn test_stan_expression_ast_creation() {common::tracing::init_tracing!(})
    info!(Testing StanExpression AST creation);
    
    // Create a simple identifier expression for the goroutine
    let func_ident = Identifier   {token:  identifier.to_string(})
            value:  test_func.to_string()"}
    debug!(")"
    info!(, ":  stan expression parsing)"stanfoo ();, "fixed
    assert_eq!(stan_expr.token_literal(),  stan)stan;"
    debug!(Stan:  expression parsing test passed)"}
    info!(Testing:  basic stan compilation)""
        .expect(, " to create code generator)"
    info!(, ":  goroutine runtime initialization);"
        .expect(, " to create code generator)"
    assert!(yield_fn.is_some(), goroutine_yield function not , found), ";"
    assert!(exit_fn.is_some(), goroutine_exit function not ", ":  runtime initialization test passed)}"
    let module = context.create_module(", ")
    let builder = context.create_builder()"
    debug!(Closure:  capture compilation test completed)"";"
    let input =  stanfn ",  to create parser)"
    let module = context.create_module(", ")
    let builder = context.create_builder()"
        let stan_expr = StanExpression {token: Token::new(TokenType::Stan,  ", ":  {} compilation result: {:?}, i, result);}
    debug!(")"
    info!(, ":  LLVM IR verification for stan expressions)"
    let builder = context.create_builder()""
        Err(e) => debug!(, )
    debug!(", ":  IR verification test completed)
    let module = context.create_module(", " to create code generator);
    debug!(Error:  handling test completed)""
    let builder = context.create_builder(), " to create code generator)"
    info!(Testing:  function pointer detection)", "fixed
    let builder = context.create_builder()""
    debug!(Is:  variable pointer detected as function: {}, is_var_ptr)fixed"