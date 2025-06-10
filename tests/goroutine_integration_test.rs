//! Comprehensive integration tests for the complete goroutine system in CURSED
//!
//! This test suite provides end-to-end testing of goroutines, covering:
//! - Basic goroutine creation and execution
//! - Scheduler behavior under various loads  
//! - Interaction with garbage collector
//! - Synchronization primitives
//! - Performance benchmarks and stress tests
//! - Edge cases and error scenarios
//! - Resource cleanup and leak detection
//! - Integration with other language features

use std::sync::  {Arc, Mutex, atomic::{AtomicUsize, AtomicBool, Ordering}}
use std::time::::Duration, Instant;
use std::thread;
use cursed::runtime::goroutine::*;
use cursed::ast::concurrency::StanExpression;
use cursed::ast::*;
use cursed::lexer::::Token, TokenType;
use cursed::object::Object;
use cursed::memory::{GarbageCollector, ThreadSafeGc;}
use cursed::codegen::jit;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use tracing::{info, debug, warn, error;}
use cursed::lexer::TokenType;

/// Test initialization and cleanup
mod common;

/// Initialize tracing for tests
macro_rules! init_tracing   {(} => {common::tracing::setup(}}))

// =============================================================================
// PART 1: BASIC GOROUTINE FUNCTIONALITY TESTS
// =============================================================================

#[test]
fn test_basic_goroutine_parsing() {common::tracing::init_tracing!(})
    info!(Testing basic goroutine parsing functionality);
    
    // Verify the basic goroutine test file exists
    assert!()
        std::path::Path::new(tests/basic_goroutine.csd).exists();
         Testfile basic_goroutine.csd not "found)
    assert_eq!(stan_expr.string(),  ", stantest_func)"}"
    info!(OK Goroutine scheduler initialization test passed)"
    assert_eq!(result, 0, Goroutine should complete , successfully)", " should be , incremented)
    info!(")"
    assert!(duration < Duration::from_secs(5), }"")
    info!(, :  high goroutine load (1000 goroutines);"")
    info!(,  High load test passed in   {:?}, duration)""
    assert!(duration < Duration::from_secs(10), Should complete within reasonable }")
    info!(Testing:  stress goroutine creation (5000 goroutines)")
    info!(", " Stress creation test passed in   {:?}, duration)
    warn!(")"
    unsafe extern  C  fn gc_task() {",}"}
    unsafe extern  C " fn memory_task() {let counter = data as *const AtomicUsize;"}
    unsafe extern  , " Coordination test passed: final value = {}, final_value)}"
    info!(Testing:  producer-consumer pattern with goroutines)""
    unsafe extern  C fn consumer_task() {,  consume more than produced,}""
    unsafe extern  :  execution time: {:?} (avg: {}ns per goroutine)"
    info!(", :  goroutine context switching performance);"
    unsafe extern  " fn success_task() {let counter = data as *const AtomicUsize;}
    unsafe extern  ", " fn panic_task(} {,}}")
    unsafe extern  C  fn resource_task() {let counter = data as *const AtomicUsize;"}
    assert!(duration < Duration::from_secs(30}, ", indefinitely)})
    info!(", ":  rapid goroutine creation and completion cycles)C  fn quick_task() {let counter = data as *const AtomicUsize;"}
        assert_eq!(cycle_total, goroutines_per_cycle, , complete, cycle}}")
        ",  Rapid creation test: {} cycles × {} goroutines = {} total in {:?},"
    unsafe extern  C fn receiver_task() {"Shouldnot receive more messages than sent,}
    unsafe extern  ", C)"}
    unsafe extern  C " fn allocating_task() {let tracker = data as *const AtomicUsize;"}
    unsafe extern  , ":  deadlock prevention in goroutine synchronization}"
    unsafe extern   fn lock_order_task1() {let (resource1, resource2, completion_count} = &*(data as *const (Arc<Mutex<i32>>, Arc<Mutex<i32>>, Arc<AtomicUsize>"")))
    info!(,  Deadlock prevention test: {} tasks completed in {:?}, completed, duration)""
    assert_eq!(completed, 10, , complete)}"
         ", LoadPerformance ,  ,"
         Integration ,"
         ", Synchronization, , ,"
         "
         ConcurrencySafety,";
         ", " Test categories covered: {:?}, test_categories)
    assert_eq!(test_categories.len(), 9, All major categories should be ", covered)"fixed"