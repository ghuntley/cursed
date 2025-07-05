//! Comprehensive tests for tuple functionality in CURSED

use cursed::parser::new_parser;
use cursed::execution::{CursedExecutionEngine, CursedValue};
use cursed::ast::{Expression, TupleExpression, TupleAccessExpression};

/// Test basic tuple creation and parsing
#[test]
fn test_tuple_parsing() {
    let source = r#"
        sus tuple_var = (1, "hello", based);
        tuple_var
    "#;
    
    let mut parser = new_parser(source).expect("Failed to create parser");
    let program = parser.parse_program().expect("Failed to parse tuple program");
    
    // Check that we have the expected statements
    assert_eq!(program.statements.len(), 2);
    
    // Verify the tuple expression was parsed correctly
    if let cursed::ast::Statement::Let(let_stmt) = &program.statements[0] {
        if let Expression::Tuple(tuple_expr) = &let_stmt.value {
            assert_eq!(tuple_expr.elements.len(), 3);
        } else {
            panic!("Expected tuple expression");
        }
    } else {
        panic!("Expected let statement");
    }
}

/// Test empty tuple
#[test]
fn test_empty_tuple() {
    let source = r#"
        sus empty = ();
        empty
    "#;
    
    let mut parser = new_parser(source).expect("Failed to create parser");
    let program = parser.parse_program().expect("Failed to parse empty tuple");
    
    if let cursed::ast::Statement::Let(let_stmt) = &program.statements[0] {
        if let Expression::Tuple(tuple_expr) = &let_stmt.value {
            assert_eq!(tuple_expr.elements.len(), 0);
        } else {
            panic!("Expected tuple expression");
        }
    }
}

/// Test tuple access parsing
#[test]
fn test_tuple_access_parsing() {
    let source = r#"
        sus tuple_var = (1, "hello", based);
        sus first = tuple_var.0;
        sus second = tuple_var.1;
        first
    "#;
    
    let mut parser = new_parser(source).expect("Failed to create parser");
    let program = parser.parse_program().expect("Failed to parse tuple access");
    
    // Check tuple access expressions
    if let cursed::ast::Statement::Let(let_stmt) = &program.statements[1] {
        if let Expression::TupleAccess(tuple_access) = &let_stmt.value {
            assert_eq!(tuple_access.index, 0);
        } else {
            panic!("Expected tuple access expression");
        }
    }
    
    if let cursed::ast::Statement::Let(let_stmt) = &program.statements[2] {
        if let Expression::TupleAccess(tuple_access) = &let_stmt.value {
            assert_eq!(tuple_access.index, 1);
        } else {
            panic!("Expected tuple access expression");
        }
    }
}

/// Test tuple destructuring assignment parsing
#[test]
fn test_tuple_destructuring_parsing() {
    let source = r#"
        sus tuple_var = (1, "hello", based);
        (a, b, c) = tuple_var;
        a
    "#;
    
    let mut parser = new_parser(source).expect("Failed to create parser");
    let program = parser.parse_program().expect("Failed to parse tuple destructuring");
    
    // Check destructuring assignment
    if let cursed::ast::Statement::Assignment(assign_stmt) = &program.statements[1] {
        if let cursed::ast::AssignmentTarget::Tuple(names) = &assign_stmt.target {
            assert_eq!(names.len(), 3);
            assert_eq!(names[0], "a");
            assert_eq!(names[1], "b");
            assert_eq!(names[2], "c");
        } else {
            panic!("Expected tuple assignment target");
        }
    } else {
        panic!("Expected assignment statement");
    }
}

/// Test nested tuples
#[test]
fn test_nested_tuples() {
    let source = r#"
        sus nested = ((1, 2), (3, 4));
        sus inner = nested.0;
        sus value = inner.1;
        value
    "#;
    
    let mut parser = new_parser(source).expect("Failed to create parser");
    let program = parser.parse_program().expect("Failed to parse nested tuples");
    
    // Verify the nested structure was parsed
    if let cursed::ast::Statement::Let(let_stmt) = &program.statements[0] {
        if let Expression::Tuple(tuple_expr) = &let_stmt.value {
            assert_eq!(tuple_expr.elements.len(), 2);
            // Each element should also be a tuple
            for element in &tuple_expr.elements {
                if let Expression::Tuple(inner_tuple) = element {
                    assert_eq!(inner_tuple.elements.len(), 2);
                } else {
                    panic!("Expected inner tuple");
                }
            }
        } else {
            panic!("Expected tuple expression");
        }
    }
}

/// Test tuple execution - basic creation and access
#[test]
fn test_tuple_execution_basic() {
    let source = r#"
        sus my_tuple = (42, "world");
        sus first = my_tuple.0;
        first
    "#;
    
    let mut engine = CursedExecutionEngine::new().expect("Failed to create execution engine");
    let result = engine.execute(source).expect("Failed to execute tuple code");
    
    match result {
        CursedValue::Integer(value) => assert_eq!(value, 42),
        _ => panic!("Expected integer value"),
    }
}

/// Test tuple execution - access second element
#[test] 
fn test_tuple_execution_second_element() {
    let source = r#"
        sus my_tuple = (42, "world");
        sus second = my_tuple.1;
        second
    "#;
    
    let mut engine = CursedExecutionEngine::new().expect("Failed to create execution engine");
    let result = engine.execute(source).expect("Failed to execute tuple code");
    
    match result {
        CursedValue::String(value) => assert_eq!(value, "world"),
        _ => panic!("Expected string value"),
    }
}

/// Test tuple destructuring execution
#[test]
fn test_tuple_destructuring_execution() {
    let source = r#"
        sus my_tuple = (42, "world", based);
        (a, b, c) = my_tuple;
        a
    "#;
    
    let mut engine = CursedExecutionEngine::new().expect("Failed to create execution engine");
    let result = engine.execute(source).expect("Failed to execute tuple destructuring");
    
    match result {
        CursedValue::Integer(value) => assert_eq!(value, 42),
        _ => panic!("Expected integer value"),
    }
}

/// Test tuple execution with variables
#[test]
fn test_tuple_with_variables() {
    let source = r#"
        sus x = 10;
        sus y = 20;
        sus my_tuple = (x, y);
        sus sum = my_tuple.0 + my_tuple.1;
        sum
    "#;
    
    let mut engine = CursedExecutionEngine::new().expect("Failed to create execution engine");
    let result = engine.execute(source).expect("Failed to execute tuple with variables");
    
    match result {
        CursedValue::Integer(value) => assert_eq!(value, 30),
        _ => panic!("Expected integer value 30"),
    }
}

/// Test empty tuple execution
#[test]
fn test_empty_tuple_execution() {
    let source = r#"
        sus empty = ();
        empty
    "#;
    
    let mut engine = CursedExecutionEngine::new().expect("Failed to create execution engine");
    let result = engine.execute(source).expect("Failed to execute empty tuple");
    
    match result {
        CursedValue::Tuple(elements) => assert_eq!(elements.len(), 0),
        _ => panic!("Expected empty tuple"),
    }
}

/// Test tuple out of bounds access (should fail)
#[test]
fn test_tuple_out_of_bounds() {
    let source = r#"
        sus my_tuple = (42, "world");
        sus invalid = my_tuple.5;
        invalid
    "#;
    
    let mut engine = CursedExecutionEngine::new().expect("Failed to create execution engine");
    let result = engine.execute(source);
    
    assert!(result.is_err(), "Expected error for out of bounds access");
}

/// Test tuple destructuring mismatch (should fail)
#[test]
fn test_tuple_destructuring_mismatch() {
    let source = r#"
        sus my_tuple = (42, "world");
        (a, b, c) = my_tuple;
        a
    "#;
    
    let mut engine = CursedExecutionEngine::new().expect("Failed to create execution engine");
    let result = engine.execute(source);
    
    assert!(result.is_err(), "Expected error for destructuring mismatch");
}

/// Test complex tuple with different types
#[test]
fn test_complex_tuple_types() {
    let source = r#"
        sus complex = (42, "hello", based, 314);
        complex.1
    "#;
    
    let mut engine = CursedExecutionEngine::new().expect("Failed to create execution engine");
    let result = engine.execute(source).expect("Failed to execute complex tuple");
    
    match result {
        CursedValue::String(value) => assert_eq!(value, "hello"),
        _ => panic!("Expected string value"),
    }
}

/// Test nested tuple execution
#[test]
fn test_nested_tuple_execution() {
    let source = r#"
        sus inner1 = (1, 2);
        sus inner2 = (3, 4);
        sus nested = (inner1, inner2);
        sus first_tuple = nested.0;
        sus value = first_tuple.1;
        value
    "#;
    
    let mut engine = CursedExecutionEngine::new().expect("Failed to create execution engine");
    let result = engine.execute(source).expect("Failed to execute nested tuple");
    
    match result {
        CursedValue::Integer(value) => assert_eq!(value, 2),
        _ => panic!("Expected integer value 2"),
    }
}
