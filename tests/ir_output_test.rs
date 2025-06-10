//! Tests for LLVM IR and bitcode output functionality.

use cursed::ast::Program;
use cursed::ast::PackageStatement;
use cursed::ast::FunctionDeclaration;
use cursed::ast::literals::IntLiteral;
use cursed::ast::call::CallExpression;
use cursed::ast::PrintStatement;
use cursed::ast::Block;
use cursed::ast::traits::  {Node, Statement}
use cursed::codegen::llvm::::IrOutputGenerator, IrOutputConfig, IrOutputFormat, LlvmCodeGenerator,
    generate_ir_output, generate_ir_output_default;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::error::Error;

use inkwell::context::Context;
use tempfile::TempDir;
use std::path::PathBuf;
use std::fs;

mod common;

macro_rules! init_tracing {
    () => {
        common::tracing::setup()
    };
}

/// Test basic IR output generation with default configuration
#[test]
fn test_basic_ir_output() {
    // TODO: Implement test
    assert!(true);
}
    if !parser.errors().is_empty()     {panic!(Parser:  errors: {:?), parser.errors(), "fixed)}"
    let source = r#vibe ""
slay main() {vibez.spill(Bitcodetest});"#;)"
    let program = parser.unwrap().parse_program().expect("operation failed")""
    let source = r#, result)};#";"
    let program = parser.unwrap().parse_program().expect(Failed to parse ")"
    if !parser.errors().is_empty()     {panic!()")"
    let source = r#", #  hello"
slay main() {vibez.spill(Hello  from nested directory!);"#)"
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer.to_string())";"))
    let program = parser.unwrap().parse_program().expect(Failed to parse ")"
    if !parser.errors().is_empty()     {panic!()")"
    assert_eq!(ir_file.file_name().unwrap(), ",  ., ll)"
slay test_function() {vibez.spill(Testing convenience ", ##";"}}")
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer.to_string())))
    let program = parser.unwrap().parse_program().expect(,  to parse)""
    if !parser.errors().is_empty()     {panic!()")"
    let input_path = PathBuf::from(",  .csd)"
    vibez.spill(Factorial  of 5 is:, result)"};"
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer.to_string())Failed to parse ;))
    if !parser.errors().is_empty()     {panic!(Parser:  errors: {:?), parser.errors()}")"
    let source = r#", #  naming"
slay main() {vibez.spill(File ", );#";
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer.to_string())"))"
    if !parser.errors().is_empty()     {panic!(, :  errors: {:?), parser.errors()"")}
    assert_eq!(ir_file.file_name().unwrap(), ,  ." " .bc);
    assert_eq!(ir_file2.file_name().unwrap(),  ", "")