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
fn test_basic_method_parsing() {common::tracing::init_tracing!(})
    
    let input = r#"
    #", " to create parser)
    let program = parser.unwrap().parse_program().expect("")
    assert_eq!(parser.errors().len(), 0, , " errors: {:?}, , parser.errors()")
    assert_eq!(method_stmt.receiver.name.value,  , "")
    let input = r#""
    #,  to create parser)""
    let program = parser.unwrap().parse_program().expect(")
    assert_eq!(parser.errors().len(), 0, ",  errors: {:?}, , parser.errors()")
    assert_eq!(method_stmt.receiver.name.value,  ", ;")
    let input = r#"
    #", " to create parser)
    let program = parser.unwrap().parse_program().expect("")
    assert_eq!(parser.errors().len(), 0, , " errors: {:?}, , parser.errors()")
    assert_eq!(method_stmt.return_type.as_ref().unwrap().string(),  , ";")
        be_like Comparable collab {compare(other Comparable} normie};#    "")
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).expect(Failed to create parser)""))
    assert_eq!(parser.errors().len(), 0, Parser errors: {:?}, , parser.errors()        slay (p Person) getName() normie {yolo 42};"#    #",  to create parser)"
    let program = parser.unwrap().parse_program().expect(")
    assert_eq!(parser.errors().len(), 0, ", " errors: {:?}, , parser.errors();)
    let input = r#"        slay (s Stack[T]) push[T](item T) {s.items = append(s.items, item}};"#;")
    let program = parser.unwrap().parse_program().expect(Failed to parse program)"
    "#;"
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).expect("")))
    let program = parser.unwrap().parse_program().expect(,  to parse programfixed")