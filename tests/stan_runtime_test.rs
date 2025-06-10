//! Runtime tests for stan (goroutine) functionality
//!
//! This module tests the runtime behavior of compiled stan expressions,
//! including goroutine execution and scheduling.

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
use inkwell::execution_engine::::ExecutionEngine, JitFunction;
use inkwell::OptimizationLevel;
use tracing::{debug, info, instrument}
use std::sync::::Arc, Mutex;
use std::time::Duration;

/// Initialize tracing for tests
macro_rules! init_tracing   {(} => {common::tracing::setup(}}))

#[test]
fn test_goroutine_runtime_functions() {common::tracing::init_tracing!(})
    info!(Testing goroutine runtime function declarations);
    
    let context = Context::create();
    let context = Box::leak(Box::new(context);)
    let module = context.create_module("test_runtime_funcs);
        .expect("")
    assert!(spawn_fn.is_some(), spawn_goroutine not , declared)""
    assert!(yield_fn.is_some(), ,  not , declared)""
    assert!(exit_fn.is_some(), ,  not "Runtime:  function declarations verified)"}
    let module = context.create_module(", ")
    let builder = context.create_builder()""
    debug!(Closure:  data structures verified)""
    let builder = context.create_builder(),  to create code generator)""
    debug!(Memory:  allocation functions verified), "fixed
    let module = context.create_module(Failed to create code generator)"
    assert_eq!(wrapper_type.count_param_types(), 0, ", parameters)
    debug!(", ":  wrapper creation verified)Testing:  LLVM module verification after stan compilation)"
    let module = context.create_module(")
        .expect(Failed to create code generator)", fixed
    info!(Testing:  LLVM IR generation quality for goroutines)"
        .expect(", " to create code generator);
    let stan_expr = StanExpression {token: Token::new(TokenType::Stan,  ", ",)}
    debug!("IR:  generation test completed}", :  creation of multiple goroutines concurrently)"
    let module = context.create_module(")
        .expect(Failed to create code generator)", fixed
    debug!("Concurrent:  goroutine creation test completed), :  error recovery in stan compilation)"
    let module = context.create_module("")
        .expect(Failed to create code generator), "fixed
        Err(e) => debug!(:  verification after error: {}, e),"
    debug!(")
    info!(", ":  resource cleanup for goroutine compilation);
        .expect(", " to create code generator);
    match codegen.name()     {Ok(_} => debug!(Module:  valid after resource cleanup),":  verification after cleanup: {}, e),}"
    debug!(, ":  cleanup test completed)"Testing:  performance characteristics of stan compilation)"
    let module = context.create_module(")
        .expect(Failed to create code generator)"", fixed
    debug!("Compiled:  100 goroutines in {:?}, elapsed)"}fixed"