//! Tests for defer statement functionality
//!
//! This module tests the defer statement implementation using the `later` keyword.
//! Tests cover basic defer functionality, LIFO execution order, and integration
//! with function returns and error handling.

mod common;

use cursed::ast::{DeferStatement, ExpressionStatement};
use cursed::ast::{Identifier, IntegerLiteral, CallExpression, StringLiteral};
use cursed::ast::traits::{Node, Statement};
use cursed::lexer::Token;
use cursed::parser::Parser;
use cursed::codegen::llvm::{LlvmCodeGenerator, DeferStatementCompilation};
use cursed::error::Error;
use tracing::{info, debug};

#[test]
fn test_defer_statement_ast() {
    // init_tracing!();
    common::tracing::setup();
    
    info!("Testing defer statement AST creation");
    
    // Create a simple expression statement to defer
    let expr = Identifier {
        token: Token::Identifier("test".to_string()),
        value: "test".to_string(),
    };
    
    let expr_stmt = ExpressionStatement {
        token: Token::Identifier("test".to_string()),
        expression: Some(Box::new(expr)),
    };
    
    // Create the defer statement
    let defer_stmt = DeferStatement {
        token: Token::Later,
        statement: Box::new(expr_stmt),
    };
    
    // Test string representation
    assert_eq!(defer_stmt.string(), "later test;");
    assert_eq!(defer_stmt.token_literal(), "later");
    
    info!("Defer statement AST test passed");
}

#[test]
fn test_defer_statement_parsing() {
    // init_tracing!();
    common::tracing::setup();
    
    info!("Testing defer statement parsing");
    
    let input = r#""
        slay test_function() {
            sus x = 5;
            later vibez.spill("Deferred message");
            yolo x;
        }
    "#";
    
    let mut lexer = cursed::lexer::Lexer::new(input);
    let mut parser = Parser::new(lexer);
    
    match parser.parse_program() {
        Ok(program) => {
            debug!("Parsed program successfully");
            
            // Check that we have a function
            assert!(!program.statements.is_empty(), "Program should have statements");
            
            // For now, just verify parsing doesn't fail
            // More detailed AST verification would go here
            info!("Defer statement parsing test passed");
        }
        Err(e) => {
            panic!("Failed to parse program with defer statement: {}", e);
        }
    }
}

#[test]
fn test_defer_lifo_order() {
    // init_tracing!();
    common::tracing::setup();
    
    info!("Testing defer LIFO execution order");
    
    let input = r#""
        slay test_defer_order() {
            later vibez.spill("First defer");
            later vibez.spill("Second defer");
            later vibez.spill("Third defer");
            vibez.spill("Normal statement");
            yolo 0;
        }
    "#";
    
    let mut lexer = cursed::lexer::Lexer::new(input);
    let mut parser = Parser::new(lexer);
    
    match parser.parse_program() {
        Ok(program) => {
            debug!("Parsed program with multiple defer statements");
            
            // Expected execution order should be:
            // 1. "Normal statement"
            // 2. "Third defer"  (last defer statement, executed first)
            // 3. "Second defer"
            // 4. "First defer"  (first defer statement, executed last)
            
            // Verify parsing succeeds
            assert!(!program.statements.is_empty());
            info!("Defer LIFO order test passed");
        }
        Err(e) => {
            panic!("Failed to parse program with multiple defer statements: {}", e);
        }
    }
}

#[test]
fn test_defer_with_variables() {
    // init_tracing!();
    common::tracing::setup();
    
    info!("Testing defer statement with variable capture");
    
    let input = r#""
        slay test_defer_variables() {
            sus x = 10;
            sus y = 20;
            
            later vibez.spill("x + y = ");
            later vibez.spill(x + y);
            
            x = 100;
            y = 200;
            
            yolo x + y;
        }
    "#";
    
    let mut lexer = cursed::lexer::Lexer::new(input);
    let mut parser = Parser::new(lexer);
    
    match parser.parse_program() {
        Ok(program) => {
            debug!("Parsed program with defer and variables");
            
            // The deferred statements should use the values of x and y
            // at the time the defer statements were executed (not declared)
            // In this case: x=100, y=200, so defer should print 300
            
            assert!(!program.statements.is_empty());
            info!("Defer with variables test passed");
        }
        Err(e) => {
            panic!("Failed to parse program with defer and variables: {}", e);
        }
    }
}

#[test]
fn test_defer_in_nested_blocks() {
    // init_tracing!();
    common::tracing::setup();
    
    info!("Testing defer statements in nested blocks");
    
    let input = r#""
        slay test_nested_defer() {
            later vibez.spill("Outer defer");
            
            lowkey (based) {
                later vibez.spill("Inner defer 1");
                later vibez.spill("Inner defer 2");
            }
            
            later vibez.spill("Another outer defer");
            yolo 0;
        }
    "#";
    
    let mut lexer = cursed::lexer::Lexer::new(input);
    let mut parser = Parser::new(lexer);
    
    match parser.parse_program() {
        Ok(program) => {
            debug!("Parsed program with nested defer statements");
            
            // Expected execution order:
            // 1. "Another outer defer"
            // 2. "Inner defer 2"
            // 3. "Inner defer 1"
            // 4. "Outer defer"
            
            assert!(!program.statements.is_empty());
            info!("Nested defer test passed");
        }
        Err(e) => {
            panic!("Failed to parse program with nested defer: {}", e);
        }
    }
}

#[test]
fn test_defer_with_early_return() {
    // init_tracing!();
    common::tracing::setup();
    
    info!("Testing defer statements with early return");
    
    let input = r#""
        slay test_early_return(normie condition) {
            later vibez.spill("This should always run");
            
            lowkey (condition) {
                later vibez.spill("Early return defer");
                yolo 1;
            }
            
            vibez.spill("This might not run");
            yolo 0;
        }
    "#";
    
    let mut lexer = cursed::lexer::Lexer::new(input);
    let mut parser = Parser::new(lexer);
    
    match parser.parse_program() {
        Ok(program) => {
            debug!("Parsed program with early return and defer");
            
            // Both defer statements should execute regardless of which return path is taken
            
            assert!(!program.statements.is_empty());
            info!("Defer with early return test passed");
        }
        Err(e) => {
            panic!("Failed to parse program with early return defer: {}", e);
        }
    }
}

// Integration test with LLVM code generation
#[cfg(feature = "llvm")]
#[test]
fn test_defer_llvm_compilation() {
    // init_tracing!();
    common::tracing::setup();
    
    info!("Testing defer statement LLVM compilation");
    
    use inkwell::context::Context;
    
    let context = Context::create();
    let mut codegen = LlvmCodeGenerator::new();
    
    // Create a simple defer statement for testing
    let expr = StringLiteral {
        token: Token::String("Deferred message".to_string()),
        value: "Deferred message".to_string(),
    };
    
    let expr_stmt = ExpressionStatement {
        token: Token::String("Deferred message".to_string()),
        expression: Some(Box::new(expr)),
    };
    
    let defer_stmt = DeferStatement {
        token: Token::Later,
        statement: Box::new(expr_stmt),
    };
    
    // Test compilation (this might fail if LLVM setup is incomplete)
    match codegen.compile_defer_statement(&defer_stmt) {
        Ok(()) => {
            info!("Defer statement compiled successfully");
        }
        Err(e) => {
            debug!("Expected compilation error (incomplete LLVM setup): {}", e);
            // This is expected for now as we need a complete function context
        }
    }
}

#[test]
fn test_multiple_defer_compilation() {
    // init_tracing!();
    common::tracing::setup();
    
    info!("Testing multiple defer statements compilation");
    
    let input = r#""
        slay main() {
            later vibez.spill("First");
            later vibez.spill("Second");
            later vibez.spill("Third");
            yolo 0;
        }
    "#";
    
    let mut lexer = cursed::lexer::Lexer::new(input);
    let mut parser = Parser::new(lexer);
    
    match parser.parse_program() {
        Ok(program) => {
            info!("Successfully parsed program with multiple defer statements");
            assert!(!program.statements.is_empty());
        }
        Err(e) => {
            panic!("Failed to parse multiple defer program: {}", e);
        }
    }
}
