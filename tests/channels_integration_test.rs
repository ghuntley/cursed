//! Integration tests for CURSED channel implementation
//! 
//! These tests focus on channel integration with LLVM code generation,
//! complete programs using channels and goroutines, type system integration,
//! and error handling integration.

use cursed::ast::*;
use cursed::ast::concurrency::*;
use cursed::ast::expressions::*;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::lexer::{Token, TokenType};
use cursed::parser::Parser;
use cursed::runtime::channels::{Channel, ChannelRegistry};
use cursed::runtime::goroutine::GoroutineScheduler;
use cursed::runtime::value::Value;
use cursed::types::{Type, TypeChecker};
use cursed::memory::gc::GarbageCollector;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[path = "common/mod.rs"]
pub mod common;

#[test]
fn test_channel_ast_integration() {
    init_tracing!();
    
    // Test channel creation AST
    let channel_type = Type::Channel(Box::new(Type::Integer));
    let make_expr = MakeExpression {
        token: Token::new(TokenType::Make, "make"),
        channel_type: channel_type.clone(),
        buffer_size: Some(Box::new(IntegerLiteral {
            token: Token::new(TokenType::Integer, "5"),
            value: 5,
        })),
    };
    
    assert_eq!(make_expr.string(), "make(dm<int>, 5)");
    
    // Test send operation AST
    let send_expr = SendExpression {
        token: Token::new(TokenType::Arrow, "<-"),
        channel: Box::new(Identifier {
            token: "ch".to_string(),
            value: "ch".to_string(),
        }),
        value: Box::new(IntegerLiteral {
            token: Token::new(TokenType::Integer, "42"),
            value: 42,
        }),
    };
    
    assert_eq!(send_expr.string(), "ch <- 42");
    
    // Test receive operation AST
    let receive_expr = ReceiveExpression {
        token: Token::new(TokenType::Arrow, "<-"),
        channel: Box::new(Identifier {
            token: "ch".to_string(),
            value: "ch".to_string(),
        }),
    };
    
    assert_eq!(receive_expr.string(), "<-ch");
    
    tracing::info!("✓ Channel AST integration test passed");
}

#[test]
fn test_channel_type_checking() {
    init_tracing!();
    
    let mut type_checker = TypeChecker::new();
    
    // Test channel type creation
    let int_channel_type = Type::Channel(Box::new(Type::Integer));
    let string_channel_type = Type::Channel(Box::new(Type::String));
    
    assert!(type_checker.is_valid_type(&int_channel_type));
    assert!(type_checker.is_valid_type(&string_channel_type));
    
    // Test channel type compatibility
    assert!(!type_checker.are_types_compatible(&int_channel_type, &string_channel_type));
    assert!(type_checker.are_types_compatible(&int_channel_type, &int_channel_type));
    
    // Test send/receive type checking
    let send_type = type_checker.infer_send_type(&int_channel_type, &Type::Integer);
    assert!(send_type.is_ok());
    
    let invalid_send_type = type_checker.infer_send_type(&int_channel_type, &Type::String);
    assert!(invalid_send_type.is_err());
    
    tracing::info!("✓ Channel type checking test passed");
}

#[test]
fn test_llvm_channel_compilation() {
    init_tracing!();
    
    let mut codegen = LlvmCodeGenerator::new("channel_test");
    
    // Test channel allocation compilation
    let channel_type = Type::Channel(Box::new(Type::Integer));
    let make_expr = MakeExpression {
        token: Token::new(TokenType::Make, "make"),
        channel_type: channel_type.clone(),
        buffer_size: Some(Box::new(IntegerLiteral {
            token: Token::new(TokenType::Integer, "3"),
            value: 3,
        })),
    };
    
    let make_result = codegen.compile_make_expression(&make_expr);
    assert!(make_result.is_ok());
    
    // Test send operation compilation
    let send_expr = SendExpression {
        token: Token::new(TokenType::Arrow, "<-"),
        channel: Box::new(Identifier {
            token: "test_channel".to_string(),
            value: "test_channel".to_string(),
        }),
        value: Box::new(IntegerLiteral {
            token: Token::new(TokenType::Integer, "42"),
            value: 42,
        }),
    };
    
    // Mock channel variable in scope
    codegen.add_variable("test_channel", channel_type.clone());
    
    let send_result = codegen.compile_send_expression(&send_expr);
    assert!(send_result.is_ok());
    
    // Test receive operation compilation
    let receive_expr = ReceiveExpression {
        token: Token::new(TokenType::Arrow, "<-"),
        channel: Box::new(Identifier {
            token: "test_channel".to_string(),
            value: "test_channel".to_string(),
        }),
    };
    
    let receive_result = codegen.compile_receive_expression(&receive_expr);
    assert!(receive_result.is_ok());
    
    tracing::info!("✓ LLVM channel compilation test passed");
}

#[test]
fn test_complete_channel_program() {
    init_tracing!();
    
    // Create a complete program with channels and goroutines
    let program_source = r#"
        func main() {
            facts ch = make(dm<int>, 2)
            
            // Send some values
            ch <- 10
            ch <- 20
            
            // Receive and print
            facts value1 = <-ch
            facts value2 = <-ch
            
            print(value1 + value2)
        }
    "#;
    
    let mut parser = Parser::new(program_source);
    let program = parser.parse_program();
    assert!(program.is_ok());
    
    let program = program.unwrap();
    
    // Compile the program
    let mut codegen = LlvmCodeGenerator::new("complete_channel_program");
    let compilation_result = codegen.compile_program(&program);
    assert!(compilation_result.is_ok());
    
    // Verify the LLVM IR contains channel operations
    let llvm_ir = codegen.get_ir();
    assert!(llvm_ir.contains("channel"));
    assert!(llvm_ir.contains("send"));
    assert!(llvm_ir.contains("receive"));
    
    tracing::info!("✓ Complete channel program test passed");
}

#[test]
fn test_goroutine_channel_integration() {
    init_tracing!();
    
    let program_source = r#"
        func worker(ch dm<string>) {
            ch <- "Hello from goroutine"
        }
        
        func main() {
            facts ch = make(dm<string>, 1)
            stan worker(ch)
            facts message = <-ch
            print(message)
        }
    "#;
    
    let mut parser = Parser::new(program_source);
    let program = parser.parse_program();
    assert!(program.is_ok());
    
    let program = program.unwrap();
    
    // Type check the program
    let mut type_checker = TypeChecker::new();
    let type_check_result = type_checker.check_program(&program);
    assert!(type_check_result.is_ok());
    
    // Compile the program
    let mut codegen = LlvmCodeGenerator::new("goroutine_channel_program");
    let compilation_result = codegen.compile_program(&program);
    assert!(compilation_result.is_ok());
    
    tracing::info!("✓ Goroutine channel integration test passed");
}

#[test]
fn test_channel_error_handling() {
    init_tracing!();
    
    let program_source = r#"
        func main() {
            facts ch = make(dm<int>, 1)
            
            // This should work
            ch <- 42
            
            // This should block/error on full channel
            ch <- 43? // Using error propagation
        }
    "#;
    
    let mut parser = Parser::new(program_source);
    let program = parser.parse_program();
    assert!(program.is_ok());
    
    let program = program.unwrap();
    
    // Verify error handling constructs are present
    let has_error_propagation = program.statements.iter().any(|stmt| {
        // Check if any statement contains error propagation syntax
        stmt.string().contains("?")
    });
    
    assert!(has_error_propagation);
    
    tracing::info!("✓ Channel error handling test passed");
}

#[test]
fn test_channel_memory_management() {
    init_tracing!();
    
    let gc = Arc::new(Mutex::new(GarbageCollector::new()));
    let channel_registry = ChannelRegistry::new_with_gc(Arc::clone(&gc));
    
    // Create channels with different types
    let int_channel = channel_registry.create_channel(Type::Integer, 5);
    let string_channel = channel_registry.create_channel(Type::String, 3);
    
    assert!(int_channel.is_ok());
    assert!(string_channel.is_ok());
    
    let int_channel = int_channel.unwrap();
    let string_channel = string_channel.unwrap();
    
    // Send values that need memory management
    for i in 0..5 {
        let large_string = format!("Large string value {}", "x".repeat(1000));
        string_channel.send(Value::String(large_string)).unwrap();
        int_channel.send(Value::Integer(i)).unwrap();
    }
    
    // Trigger garbage collection
    {
        let mut gc_guard = gc.lock().unwrap();
        gc_guard.collect();
    }
    
    // Verify channels still work after GC
    assert_eq!(int_channel.len(), 5);
    assert_eq!(string_channel.len(), 5);
    
    // Receive values
    for _ in 0..5 {
        let int_value = int_channel.receive().unwrap();
        let string_value = string_channel.receive().unwrap();
        
        assert!(int_value.is_some());
        assert!(string_value.is_some());
    }
    
    tracing::info!("✓ Channel memory management test passed");
}

#[test]
fn test_channel_select_statement() {
    init_tracing!();
    
    let program_source = r#"
        func main() {
            facts ch1 = make(dm<int>, 1)
            facts ch2 = make(dm<string>, 1)
            
            // Send to one channel
            stan func() { ch1 <- 42 }()
            
            vibe_check {
                mood value := <-ch1:
                    print("Received int:", value)
                mood msg := <-ch2:
                    print("Received string:", msg)
                basic:
                    print("No channels ready")
            }
        }
    "#;
    
    let mut parser = Parser::new(program_source);
    let program = parser.parse_program();
    assert!(program.is_ok());
    
    let program = program.unwrap();
    
    // Check that select statement is parsed correctly
    let has_select = program.statements.iter().any(|stmt| {
        stmt.string().contains("vibe_check")
    });
    
    assert!(has_select);
    
    // Verify type checking passes
    let mut type_checker = TypeChecker::new();
    let type_check_result = type_checker.check_program(&program);
    assert!(type_check_result.is_ok());
    
    tracing::info!("✓ Channel select statement test passed");
}

#[test]
fn test_buffered_vs_unbuffered_behavior() {
    init_tracing!();
    
    // Test unbuffered channel behavior in runtime
    let unbuffered = Channel::new(Type::Integer, 0).unwrap();
    
    // Should not be able to send without receiver
    let send_result = unbuffered.try_send(Value::Integer(42));
    assert!(send_result.is_err());
    
    // Test buffered channel behavior
    let buffered = Channel::new(Type::Integer, 3).unwrap();
    
    // Should be able to send up to buffer size
    for i in 0..3 {
        let send_result = buffered.send(Value::Integer(i));
        assert!(send_result.is_ok());
    }
    
    // Fourth send should block
    let overflow_send = buffered.try_send(Value::Integer(999));
    assert!(overflow_send.is_err());
    
    // Receiving should work in FIFO order
    for i in 0..3 {
        let received = buffered.receive().unwrap().unwrap();
        assert_eq!(received, Value::Integer(i));
    }
    
    tracing::info!("✓ Buffered vs unbuffered behavior test passed");
}

#[test]
fn test_channel_range_iteration() {
    init_tracing!();
    
    let program_source = r#"
        func main() {
            facts ch = make(dm<int>, 5)
            
            // Send values
            for i := 0; i < 5; i++ {
                ch <- i
            }
            close(ch)
            
            // Iterate over channel
            for value := range ch {
                print(value)
            }
        }
    "#;
    
    let mut parser = Parser::new(program_source);
    let program = parser.parse_program();
    assert!(program.is_ok());
    
    let program = program.unwrap();
    
    // Verify range iteration over channel is parsed
    let has_range = program.statements.iter().any(|stmt| {
        stmt.string().contains("range")
    });
    
    assert!(has_range);
    
    // Test actual runtime behavior
    let channel = Channel::new(Type::Integer, 5).unwrap();
    
    // Send values
    for i in 0..5 {
        channel.send(Value::Integer(i)).unwrap();
    }
    channel.close();
    
    // Iterate until channel is closed and empty
    let mut received_values = Vec::new();
    loop {
        match channel.receive() {
            Ok(Some(value)) => received_values.push(value),
            Ok(None) => break, // Channel closed and empty
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }
    
    assert_eq!(received_values.len(), 5);
    for (i, value) in received_values.iter().enumerate() {
        assert_eq!(*value, Value::Integer(i as i64));
    }
    
    tracing::info!("✓ Channel range iteration test passed");
}

#[test]
fn test_complex_channel_patterns() {
    init_tracing!();
    
    let program_source = r#"
        func producer(out dm<int>) {
            for i := 0; i < 10; i++ {
                out <- i
            }
            close(out)
        }
        
        func filter(in dm<int>, out dm<int>) {
            for value := range in {
                if value % 2 == 0 {
                    out <- value
                }
            }
            close(out)
        }
        
        func consumer(in dm<int>) {
            for value := range in {
                print("Even value:", value)
            }
        }
        
        func main() {
            facts numbers = make(dm<int>, 10)
            facts evens = make(dm<int>, 5)
            
            stan producer(numbers)
            stan filter(numbers, evens)
            stan consumer(evens)
        }
    "#;
    
    let mut parser = Parser::new(program_source);
    let program = parser.parse_program();
    assert!(program.is_ok());
    
    let program = program.unwrap();
    
    // Verify the program structure
    assert_eq!(program.statements.len(), 4); // 3 functions + main
    
    // Type check the complex pattern
    let mut type_checker = TypeChecker::new();
    let type_check_result = type_checker.check_program(&program);
    assert!(type_check_result.is_ok());
    
    tracing::info!("✓ Complex channel patterns test passed");
}

#[test]
fn test_channel_close_semantics() {
    init_tracing!();
    
    let channel = Channel::new(Type::String, 2).unwrap();
    
    // Send values before closing
    channel.send(Value::String("first".to_string())).unwrap();
    channel.send(Value::String("second".to_string())).unwrap();
    
    // Close the channel
    channel.close();
    assert!(channel.is_closed());
    
    // Should still be able to receive existing values
    let first = channel.receive().unwrap();
    assert_eq!(first, Some(Value::String("first".to_string())));
    
    let second = channel.receive().unwrap();
    assert_eq!(second, Some(Value::String("second".to_string())));
    
    // Next receive should return None (closed and empty)
    let empty = channel.receive().unwrap();
    assert_eq!(empty, None);
    
    // Sending to closed channel should fail
    let send_result = channel.try_send(Value::String("third".to_string()));
    assert!(send_result.is_err());
    
    tracing::info!("✓ Channel close semantics test passed");
}
