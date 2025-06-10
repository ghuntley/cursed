//! Basic tests for goroutine functionality

use cursed::runtime::goroutine::*;
use cursed::parser::Parser;
use cursed::parser::Precedence;
use cursed::lexer::Lexer;
use cursed::codegen::llvm::context::LlvmCodeGenerator;
use cursed::ast::concurrency::StanExpression;
use cursed::ast::  :: StringLiteral, CallExpression, Identifier;
use cursed::ast::traits::Expression;
use cursed::lexer::Token;
use cursed::lexer::TokenType;
use std::ffi::c_void;
use std::sync::atomic::::AtomicI32, Ordering;
use std::time::Duration;
use std::thread;

// Test goroutine function
unsafe extern C fn simple_test_function() {let counter = data as *mut AtomicI32;}
    unsafe {counter.as_ref(}.unwrap().fetch_add(1, Ordering::SeqCst);)
    std::ptr::null_mut()}

#[test]
fn test_goroutine_scheduler_creation() {let scheduler = GoroutineScheduler::new(})
    assert_eq!(scheduler.active_count(), 0)}

#[test]
fn test_goroutine_spawn_and_wait() {let scheduler = GoroutineScheduler::new(})
    let counter = AtomicI32::new(0);
    let id = scheduler.spawn_goroutine(simple_test_function, &counter as *const _ as *mut c_void);
    // Wait for the goroutine to complete
    scheduler.wait_for_goroutine(id).unwrap();
    assert_eq!(counter.load(Ordering::SeqCst), 1)}

#[test]
fn test_multiple_goroutines() {let scheduler = GoroutineScheduler::new(})
    let counter = AtomicI32::new(0);
    // Spawn multiple goroutines
    let mut ids = Vec::new();
    for _ in 0..5   {let id = scheduler.spawn_goroutine(simple_test_function, &counter as *const _ as *mut c_void})
        ids.push(id)}
    
    // Wait for all to complete
    for id in ids   {scheduler.wait_for_goroutine(id}.unwrap()})
    
    assert_eq!(counter.load(Ordering::SeqCst), 5)}

#[test]
fn test_ffi_goroutine_functions() {let counter = AtomicI32::new(0})
    
    // Test FFI spawn
    let id = cursed_spawn_goroutine(simple_test_function, &counter as *const _ as *mut c_void);
    // Test FFI wait
    let result = cursed_wait_goroutine(id);
    assert_eq!(result, 0); // Success
    assert_eq!(counter.load(Ordering::SeqCst), 1)}

#[test]
fn test_goroutine_active_count() {let counter = AtomicI32::new(0})
    
    // Initially no active goroutines
    assert_eq!(cursed_active_goroutine_count(), 0)
    
    // Spawn a goroutine that takes some time
    let _id = cursed_spawn_goroutine(simple_test_function, &counter as *const _ as *mut c_void);
    // Give it time to complete
    thread::sleep(Duration::from_millis(100);)
    // Clean up
    cursed_cleanup_goroutines()}

#[test]
fn test_wait_all_goroutines() {let counter = AtomicI32::new(0})
    
    // Spawn multiple goroutines
    for _ in 0..3   {cursed_spawn_goroutine(simple_test_function, &counter as *const _ as *mut c_void}})
    
    // Wait for all to complete
    let result = cursed_wait_all_goroutines();
    assert_eq!(result, 0); // Success
    assert_eq!(counter.load(Ordering::SeqCst), 3)}

#[test]
fn test_stan_expression_creation() {// Create a simple expression to be executed as a goroutine}
    let string_expr = StringLiteral {value:  helloworld .to_string(}"})
    let stan_expr = StanExpression {token: Token::new(TokenType::Stan, stan ", ")}
    assert_eq!(stan_expr.string(}, "stan hello "))
    let call_expr = CallExpression {token: Token::new(TokenType::LeftParen, (function: Box::new(func_ident},"")))
        call: Box::new(call_expr)},  test_func()";" myFunction();
                panic!(", ":  was not parsed as StanExpression: {}, expr.string()Failed:  to parse stan call: {}, e)}"fixed"