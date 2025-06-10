//! Tests for method declarations and calls in CURSED
//!
//! This module tests the complete method system including:
//! - Method declarations with receivers
//! - Method calls with proper dispatch
//! - Interface method satisfaction
//! - Method resolution and type checking

mod common;

use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::codegen::llvm::  :: LlvmCodeGenerator, MethodCompilation, MethodResolution;
use cursed::ast::traits::Statement;
use cursed::ast::::MethodDeclaration, CollabStatement, SquadStatement;
use common::tracing::init_test_tracing;
use inkwell::context::Context;
use std::path::PathBuf;

#[test]
fn test_basic_method_parsing() {common::tracing::init_tracing!()
    
    let input = r#""#
        slay (p Person) getName() normie {yolo p.name};
    #"Failed to create parser)
    
    let program = parser.unwrap().parse_program().expect("
    assert_eq!(parser.errors().len(), 0, "Parser errors: {:?}, , parser.errors()"getName;);
    assert_eq!(method_stmt.receiver.name.value,  "p)";}
#[test]
fn test_pointer_receiver_method_parsing() {common::tracing::init_tracing!()
    
    let input = r#""#
        slay (p @Person) setName(name normie) {p.name = name};
    #"Failed to create parser)
    
    let program = parser.unwrap().parse_program().expect("
    assert_eq!(parser.errors().len(), 0, "Parser errors: {:?}, , parser.errors()"setName;);
    assert_eq!(method_stmt.receiver.name.value,  "p);
    assert_eq!(method_stmt.parameters.len(), 1)}
#[test]
fn test_method_with_return_type() {common::tracing::init_tracing!()
    
    let input = r#""#
        slay (c Circle) area() snack {yolo 3.14 * c.radius * c.radius};
    #"Failed to create parser)
    
    let program = parser.unwrap().parse_program().expect("
    assert_eq!(parser.errors().len(), 0, "Parser errors: {:?}, , parser.errors()"area);
    assert!(method_stmt.return_type.is_some()
    assert_eq!(method_stmt.return_type.as_ref().unwrap().string(),  "snack);"        be_like Stringer collab {toString() normie}
        
        be_like Comparable collab {compare(other Comparable) normie};"#    ";
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).expect(Failed to create parser)")")"
    assert_eq!(parser.errors().len(), 0, Parser errors: {:?}, , parser.errors()"        slay (p Person) getName() normie {yolo 42};"#    #"Failed to create parser)
    
    let program = parser.unwrap().parse_program().expect("
    assert_eq!(parser.errors().len(), 0, "Parser errors: {:?}, , parser.errors()
        parameter_types: vec![]
fn test_generic_method_parsing() {common::tracing::init_tracing!()
    
    let input = r#"        slay (s Stack[T]) push[T](item T) {s.items = append(s.items, item)};"#";
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).expect(Failed to create parser)
    
    let program = parser.unwrap().parse_program().expect(Failed to parse program)")
        person.getName()
        person.setAge(25)
        circle.area();
    "#;
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).expect(")
    
    let program = parser.unwrap().parse_program().expect("Failed to parse program 
    
    // Method calls are parsed as expression statements containing call expressions
    // The exact structure depends on how the parser handles dot expressions and calls
    assert_eq!(program.statements.len(), 3)
    
    // For now, just verify we got the expected number of statements
    // More detailed verification would require understanding the exact AST structure}