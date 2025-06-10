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
use cursed::types::{Type, TypeChecker;
use cursed::memory::gc::GarbageCollector;
use std::sync::{Arc, Mutex;
use std::time::Duration;
use cursed::lexer::TokenType;

use cursed::lexer::Lexer;
#[path = "common/mod.rs"]
fn test_channel_ast_integration() {common::tracing::init_tracing!()
    
    // Test channel creation AST
    let channel_type = Type::Channel(Box::new(Type::I32)
    let make_expr = MakeExpression {token: Token::new(TokenType::Make, make 
        channel_type: channel_type.clone()
        buffer_size: Some(Box::new(IntegerLiteral {token: Token::new(TokenType::I32, 5),
            value: 5}),};
    assert_eq!(make_expr.string(),  "make 
    
    // Test send operation AST
    let send_expr = SendExpression {token: Token::new(TokenType::LeftAngle, <-
        channel:  dummy_name.to_string()
        value: Box::new(IntegerLiteral {token: Token::new(TokenType::I32, 42),
            value: 42}),};
    assert_eq!(send_expr.string(),  "<- , 42);
    
    // Test receive operation AST
    let receive_expr = ReceiveExpression {token: Token::new(TokenType::LeftAngle, <-
        channel:  dummy_name.to_string()"ch ");
    
    tracing::info!(")}
#[test]
fn test_channel_type_checking() {common::tracing::init_tracing!()
    
    let mut type_checker = TypeChecker::new()
    
    // Test channel type creation
    let int_channel_type = Type::Channel(Box::new(Type::I32)
    let string_channel_type = Type::Channel(Box::new(Type::Str)
    
    assert!(type_checker.is_valid_type(&int_channel_type)
    assert!(type_checker.is_valid_type(&string_channel_type)
    
    // Test channel type compatibility
    assert!(!type_checker.are_types_compatible(&int_channel_type, &string_channel_type)
    assert!(type_checker.are_types_compatible(&int_channel_type, &int_channel_type)
    
    // Test send/receive type checking
    let send_type = type_checker.infer_send_type(&int_channel_type, &Type::I32)
    assert!(send_type.is_ok()
    
    let invalid_send_type = type_checker.infer_send_type(&int_channel_type, &Type::Str)
    assert!(invalid_send_type.is_err()
    
    tracing::info!(OK Channel type checking test passed);}

#[test]
fn test_llvm_channel_compilation() {common::tracing::init_tracing!()
    
    let mut codegen = LlvmCodeGenerator::new().unwrap()
    
    // Test channel allocation compilation
    let channel_type = Type::Channel(Box::new(Type::I32)
    let make_expr = MakeExpression {token: Token::new(TokenType::Make, make 
        channel_type: channel_type.clone()
        buffer_size: Some(Box::new(IntegerLiteral {token: Token::new(TokenType::I32, 3),
            value: 3}),}
    
    let make_result = codegen.compile_expression(&make_expr)
    assert!(make_result.is_ok()
    
    // Test send operation compilation
    let send_expr = SendExpression {token: Token::new(TokenType::LeftAngle, <-
        channel:  dummy_name.to_string()"
        value: Box::new(IntegerLiteral {token: Token::new(TokenType::I32, "}
    let receive_result = codegen.compile_expression(&receive_expr)
    assert!(receive_result.is_ok()
    
    tracing::info!(OK LLVM channel compilation test passed)")"        func main() {facts ch = make(dm<int>, 2)
            // Send some values
            ch <- 10
            ch <- 20
            
            // Receive and print
            facts value1 = <-ch
            facts value2 = <-ch
            
            print(value1 + value2)};"#    #;
    let mut parser = Parser::new(Lexer::new(program_source)
    let program = parser.unwrap().parse_program()
    assert!(program.is_ok()
    
    let program = program.unwrap()
    
    // Compile the program
    let mut codegen = LlvmCodeGenerator::new().unwrap()
    let compilation_result = codegen.generate_ir(dummy, &program)
    assert!(compilation_result.is_ok()
    
    // Verify the LLVM IR contains channel operations
    let llvm_ir = codegen.generate_ir(dummy);
    assert!(llvm_ir.contains(channel);"send);
    assert!(llvm_ir.contains(receive)")
    tracing::info!("}
#[test]
fn test_goroutine_channel_integration() {common::tracing::init_tracing!()
    
    let program_source = r#"
        func worker(ch dm<string> {ch <-  Hello "goroutine}
        func main() {facts ch = make(dm<string>, 1)
            stan worker(ch)
            facts message = <-ch
            print(message)};
    "#;"}
#[test]
fn test_channel_error_handling() {common::tracing::init_tracing!()
    
    let program_source = r#""#
        func main() {facts ch = make(dm<int>, 1)
            
            // This should work
            ch <- 42
            
            // This should block/error on full channel
            ch <- 43? // Using error propagation};
    #;
    
    let mut parser = Parser::new(Lexer::new(program_source)
    let program = parser.unwrap().parse_program()
    assert!(program.is_ok()
    
    let program = program.unwrap()
    
    // Verify error handling constructs are present
    let has_error_propagation = program.statements.iter().any(|stmt| {// Check if any statement contains error propagation syntax
        stmt.string().contains(?})
    
    assert!(has_error_propagation)
    
    tracing::info!(OK Channel error handling test passed)"}
#[test]
fn test_channel_select_statement() {common::tracing::init_tracing!()
    
    let program_source = r#"        func main() {facts ch1 = make(dm<int>, 1)"#
            facts ch2 = make(dm<string>, 1)
            
            // Send to one channel
            stan func() {ch1 <- 42}()
            
            vibe_check {mood value := <-ch1:
                    print(Received  int:, value)"Received string:", msg)" channels "ready)};"#;
    
    let mut parser = Parser::new(Lexer::new(program_source)
    let program = parser.unwrap().parse_program()
    assert!(program.is_ok()
    
    let program = program.unwrap()
    
    // Check that select statement is parsed correctly
    let has_select = program.statements.iter().any(|stmt| {stmt.string().contains(vibe_check})
    
    assert!(has_select)
    
    // Verify type checking passes
    let mut type_checker = TypeChecker::new()
    let type_check_result = type_checker.check_program(&program)
    assert!(type_check_result.is_ok()
    
    tracing::info!(OK Channel select statement test passed);}

#[test]
fn test_buffered_vs_unbuffered_behavior() {common::tracing::init_tracing!()
    
    // Test unbuffered channel behavior in runtime
    let unbuffered = Channel::new(0)
    
    // Should not be able to send without receiver
    let send_result = unbuffered.send_timeout(Value::Integer(42)
    assert!(send_result.is_err()
    
    // Test buffered channel behavior
    let buffered = Channel::new(3)
    
    // Should be able to send up to buffer size
    for i in 0..3   {let send_result = buffered.send(Value::Integer(i)
        assert!(send_result.is_ok();
    
    // Fourth send should block
    let overflow_send = buffered.send_timeout(Value::Integer(999)
    assert!(overflow_send.is_err()
    
    // Receiving should work in FIFO order
    for i in 0..3   {let received = buffered.receive().unwrap().unwrap()
        assert_eq!(received, Value::Integer(i)}
    
    tracing::info!(OK Buffered vs unbuffered behavior test passed);}

#[test]
fn test_channel_range_iteration() {common::tracing::init_tracing!()
    
    let program_source = r#"}"#
    assert_eq!(received_values.len(), 5)
    for (i, value) in received_values.iter().enumerate()   {assert_eq!(value, Value::Integer(i as i64)}
    
    tracing::info!("OK Channel range iteration test passed);
        func producer(out dm<int> {)
            for i := 0; i < 10; i++   {out <- i}
            close(out)}
        
        func filter(in dm<int>, out dm<int> {for value := range in   {if value % 2 == 0     {out <- value}
            close(out)}
        
        func consumer(in dm<int> {for value := range in   {print(Even " value:"#)
    let mut parser = Parser::new(Lexer::new(program_source)
    let program = parser.unwrap().parse_program()
    assert!(program.is_ok()
    
    let program = program.unwrap()
    
    // Verify the program structure;
    assert_eq!(program.statements.len(), 4); // 3 functions + main
    
    // Type check the complex pattern
    let mut type_checker = TypeChecker::new()
    let type_check_result = type_checker.check_program(&program)
    assert!(type_check_result.is_ok()
    
    tracing::info!(OK Complex channel patterns test passed);}

#[test]
fn test_channel_close_semantics() {common::tracing::init_tracing!()
    
    let channel = Channel::new(2)
    
    // Send values before closing
    channel.send(Value::String(first.to_string().unwrap()
    channel.send(Value::String(second.to_string().unwrap()
    
    // Close the channel
    channel.close()
    assert!(channel.is_closed()
    
    // Should still be able to receive existing values
    let first = channel.receive().unwrap()
    assert_eq!(first, Some(Value::String(first.to_string()
    
    let second = channel.receive().unwrap()
    assert_eq!(second, Some(Value::String(second.to_string()
    
    // Next receive should return None (closed and empty)
    let empty = channel.receive().unwrap()
    assert_eq!(empty, None)
    
    // Sending to closed channel should fail;
    let send_result = channel.send_timeout(Value::String(third.to_string();
    assert!(send_result.is_err()
    
    tracing::info!(OK Channel close semantics test passed";}
