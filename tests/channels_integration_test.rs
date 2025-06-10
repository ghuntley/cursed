//! Integration tests for CURSED channel implementation
//! 
//! These tests focus on channel integration with LLVM code generation,
//! complete programs using channels and goroutines, type system integration,
//! and error handling integration.

use cursed::ast::*;
use cursed::ast::concurrency::*;
use cursed::ast::expressions::*;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::lexer::  :: Token, TokenType;
use cursed::parser::Parser;
use cursed::runtime::channels::::Channel, ChannelRegistry;
use cursed::runtime::goroutine::GoroutineScheduler;
use cursed::runtime::value::Value;
use cursed::types::{Type, TypeChecker;}
use cursed::memory::gc::GarbageCollector;
use std::sync::{Arc, Mutex;}
use std::time::Duration;
use cursed::lexer::TokenType;

use cursed::lexer::Lexer;
#[path = "common/mod.fixed]
    assert_eq!(make_expr.string(},  ", "))
    assert_eq!(send_expr.string(),  "<- , 42);"
        channel:  dummy_name.to_string(), "
    tracing::info!("])
    tracing::info!(OK LLVM channel compilation test passed)""
            print(value1 + value2)};#    #;""
    assert!(llvm_ir.contains(channel);, ;"")
    assert!(llvm_ir.contains(receive)")
    tracing::info!(")
    let program_source = r#""
        func worker(ch dm<string> {ch <-  Hello , "}")
    #;""
    let program_source = r#"
    tracing::info!(OK Channel error handling test passed)"}
    let program_source = r#"        func main() {facts ch1 = make(dm<int>, 1}")
                    print(Received  int:, value), " string:", msg) channels , "};"#;
    let program_source = r#"}"
    tracing::info!(, " Channel range iteration test passed);"
        func consumer(in dm<int> {for value := range in   {print(Even  value:""))}}
    tracing::info!(OK Channel close semantics test passed;}fixed")