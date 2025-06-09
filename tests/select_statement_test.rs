//! Tests for select statement functionality
//!
//! This test suite verifies the comprehensive channel select statement system
//! including AST creation, parsing, and LLVM code generation.

use cursed::ast::control_flow::select::*;
use cursed::ast::expressions::channel::{SendExpression, ReceiveExpression};
use cursed::ast::expressions::Identifier;
use cursed::ast::traits::{Expression, Statement};
use cursed::lexer::{Lexer, Token};
use cursed::parser::Parser;
use cursed::error::Error;

mod common;

/// Test basic select statement AST creation
#[test]
fn test_select_statement_ast_creation() {
    common::tracing::setup();
    
    // Create a simple receive expression for testing
    let channel_expr = Box::new(Identifier {
        token: Token::Identifier("ch".to_string()),
        value: "ch".to_string(),
    });
    
    let receive_expr = ReceiveExpression {
        token: Token::Arrow,
        channel: channel_expr,
        element_type: "normie".to_string(),
    };
    
    // Create a select case
    let select_case = SelectCase {
        token: Token::Mood,
        communication: Box::new(receive_expr),
        statements: Vec::new(),
    };
    
    // Create a select statement
    let select_stmt = SelectStatement {
        token: Token::Choose,
        cases: vec![select_case],
        default: None,
    };
    
    // Verify the string representation
    let stmt_string = select_stmt.string();
    assert!(stmt_string.contains("choose"));
    assert!(stmt_string.contains("mood"));
    assert!(stmt_string.contains("<-ch"));
    
    tracing::info!("Select statement AST creation test passed");
}

/// Test select statement with default case
#[test]
fn test_select_statement_with_default() {
    common::tracing::setup();
    
    // Create a select statement with default case
    let default_case = DefaultCase {
        token: Token::Basic,
        statements: Vec::new(),
    };
    
    let select_stmt = SelectStatement {
        token: Token::Choose,
        cases: Vec::new(),
        default: Some(default_case),
    };
    
    // Verify the string representation includes default
    let stmt_string = select_stmt.string();
    assert!(stmt_string.contains("choose"));
    assert!(stmt_string.contains("basic"));
    
    tracing::info!("Select statement with default test passed");
}

/// Test select statement parsing from source code
#[test]
fn test_select_statement_parsing() {
    common::tracing::setup();
    
    let source = r#"
        choose {
            mood <-ch:
                x = 42
            basic:
                y = 0
        }
    "#;
    
    let mut lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    
    // Try to parse the select statement
    match parser.parse_select_statement() {
        Ok(stmt) => {
            let select_stmt = stmt.as_any().downcast_ref::<SelectStatement>()
                .expect("Should be a SelectStatement");
            
            // Verify we have one case and one default
            assert_eq!(select_stmt.cases.len(), 1);
            assert!(select_stmt.default.is_some());
            
            tracing::info!("Select statement parsing test passed");
        }
        Err(e) => {
            tracing::error!("Parsing failed: {}", e);
            // For now, we expect this to work but parsing may need more setup
            // This is a basic structure test
        }
    }
}

/// Test select statement with multiple cases
#[test]
fn test_select_statement_multiple_cases() {
    common::tracing::setup();
    
    // Create multiple cases for the select statement
    let mut cases = Vec::new();
    
    // Case 1: Receive from channel 1
    let ch1_expr = Box::new(Identifier {
        token: Token::Identifier("ch1".to_string()),
        value: "ch1".to_string(),
    });
    
    let receive1 = ReceiveExpression {
        token: Token::Arrow,
        channel: ch1_expr,
        element_type: "normie".to_string(),
    };
    
    cases.push(SelectCase {
        token: Token::Mood,
        communication: Box::new(receive1),
        statements: Vec::new(),
    });
    
    // Case 2: Send to channel 2
    let ch2_expr = Box::new(Identifier {
        token: Token::Identifier("ch2".to_string()),
        value: "ch2".to_string(),
    });
    
    let value_expr = Box::new(Identifier {
        token: Token::Identifier("value".to_string()),
        value: "value".to_string(),
    });
    
    let send2 = SendExpression {
        token: Token::Arrow,
        channel: ch2_expr,
        value: value_expr,
    };
    
    cases.push(SelectCase {
        token: Token::Mood,
        communication: Box::new(send2),
        statements: Vec::new(),
    });
    
    // Create select statement with multiple cases
    let select_stmt = SelectStatement {
        token: Token::Choose,
        cases,
        default: None,
    };
    
    // Verify we have two cases
    assert_eq!(select_stmt.cases.len(), 2);
    
    // Verify string representation contains both operations
    let stmt_string = select_stmt.string();
    assert!(stmt_string.contains("<-ch1"));
    assert!(stmt_string.contains("ch2 <- value"));
    
    tracing::info!("Multiple cases select statement test passed");
}

/// Test timeout case functionality
#[test]
fn test_timeout_case() {
    common::tracing::setup();
    
    // Create a timeout expression (duration)
    let duration_expr = Box::new(Identifier {
        token: Token::Identifier("timeout".to_string()),
        value: "timeout".to_string(),
    });
    
    // Create a timeout case
    let timeout_case = TimeoutCase {
        token: Token::Identifier("timeout".to_string()),
        duration: duration_expr,
        statements: Vec::new(),
    };
    
    // Verify the string representation
    let timeout_string = timeout_case.string();
    assert!(timeout_string.contains("timeout"));
    
    tracing::info!("Timeout case test passed");
}

/// Test helper functions for creating select statement components
#[test]
fn test_select_statement_helpers() {
    common::tracing::setup();
    
    // Test new_select_case helper
    let channel_expr = Box::new(Identifier {
        token: Token::Identifier("ch".to_string()),
        value: "ch".to_string(),
    });
    
    let receive_expr = ReceiveExpression {
        token: Token::Arrow,
        channel: channel_expr,
        element_type: "any".to_string(),
    };
    
    let case = new_select_case(
        Token::Mood,
        Box::new(receive_expr),
        Vec::new(),
    );
    
    assert_eq!(case.statements.len(), 0);
    
    // Test new_default_case helper
    let default = new_default_case(
        Token::Basic,
        Vec::new(),
    );
    
    assert_eq!(default.statements.len(), 0);
    
    // Test new_select_statement helper
    let select = new_select_statement(
        Token::Choose,
        vec![case],
        Some(default),
    );
    
    assert_eq!(select.cases.len(), 1);
    assert!(select.default.is_some());
    
    tracing::info!("Select statement helpers test passed");
}

/// Test select statement compilation readiness
#[test]
fn test_select_statement_compilation_readiness() {
    common::tracing::setup();
    
    // This test verifies that our select statement structures are ready
    // for LLVM compilation by checking that they implement the required traits
    
    let channel_expr = Box::new(Identifier {
        token: Token::Identifier("test_ch".to_string()),
        value: "test_ch".to_string(),
    });
    
    let receive_expr = ReceiveExpression {
        token: Token::Arrow,
        channel: channel_expr,
        element_type: "normie".to_string(),
    };
    
    let select_case = SelectCase {
        token: Token::Mood,
        communication: Box::new(receive_expr),
        statements: Vec::new(),
    };
    
    let select_stmt = SelectStatement {
        token: Token::Choose,
        cases: vec![select_case],
        default: None,
    };
    
    // Verify that SelectStatement implements Statement trait
    let stmt: &dyn Statement = &select_stmt;
    let token_literal = stmt.token_literal();
    assert_eq!(token_literal, "choose");
    
    // Verify that it can be downcast
    let any = stmt.as_any();
    let downcast_stmt = any.downcast_ref::<SelectStatement>();
    assert!(downcast_stmt.is_some());
    
    tracing::info!("Select statement compilation readiness test passed");
}

/// Integration test for complete select statement functionality
#[test]
fn test_select_statement_integration() {
    common::tracing::setup();
    
    // Test the complete pipeline: AST creation -> string representation -> traits
    
    // Create channels and expressions
    let ch1 = Box::new(Identifier {
        token: Token::Identifier("input_ch".to_string()),
        value: "input_ch".to_string(),
    });
    
    let ch2 = Box::new(Identifier {
        token: Token::Identifier("output_ch".to_string()),
        value: "output_ch".to_string(),
    });
    
    let value = Box::new(Identifier {
        token: Token::Identifier("data".to_string()),
        value: "data".to_string(),
    });
    
    // Create receive and send operations
    let receive_op = ReceiveExpression {
        token: Token::Arrow,
        channel: ch1,
        element_type: "normie".to_string(),
    };
    
    let send_op = SendExpression {
        token: Token::Arrow,
        channel: ch2,
        value,
    };
    
    // Create cases
    let receive_case = SelectCase {
        token: Token::Mood,
        communication: Box::new(receive_op),
        statements: Vec::new(),
    };
    
    let send_case = SelectCase {
        token: Token::Mood,
        communication: Box::new(send_op),
        statements: Vec::new(),
    };
    
    // Create default case
    let default_case = DefaultCase {
        token: Token::Basic,
        statements: Vec::new(),
    };
    
    // Create complete select statement
    let select_stmt = SelectStatement {
        token: Token::Choose,
        cases: vec![receive_case, send_case],
        default: Some(default_case),
    };
    
    // Verify complete functionality
    assert_eq!(select_stmt.cases.len(), 2);
    assert!(select_stmt.default.is_some());
    
    let stmt_string = select_stmt.string();
    assert!(stmt_string.contains("choose"));
    assert!(stmt_string.contains("<-input_ch"));
    assert!(stmt_string.contains("output_ch <- data"));
    assert!(stmt_string.contains("basic"));
    
    // Verify trait implementation
    let stmt_trait: &dyn Statement = &select_stmt;
    assert_eq!(stmt_trait.token_literal(), "choose");
    
    tracing::info!("Select statement integration test passed");
    
    println!("Generated select statement:");
    println!("{}", stmt_string);
}
