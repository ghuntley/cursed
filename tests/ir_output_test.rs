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

macro_rules! init_tracing {() => {common::tracing::setup()}

/// Test basic IR output generation with default configuration
#[test]
fn test_basic_ir_output() {common::tracing::init_tracing!()
    let _timer = common::timing::Timer::new(test_basic_ir_output)

    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let temp_dir = TempDir::new().unwrap()

    // Create a simple CURSED program
    let source = r#"vibe test"#

slay main() {vibez.spill(Hello , World!"#"#)
    // Parse the program
    let mut lexer = Lexer::new(source.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).expect(Failed to create parser);
    let program = parser.unwrap().parse_program().expect(")
    if !parser.errors().is_empty()     {panic!(Parser:  errors: {:?}, parser.errors()")"define);}
/// Test bitcode output generation
#[test]
fn test_bitcode_output() {common::tracing::init_tracing!()
    let _timer = common::timing::Timer::new(test_bitcode_output)

    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let temp_dir = TempDir::new().unwrap()

    // Create a simple CURSED program
    let source = r#"vibe test"#

slay main() {vibez.spill(Bitcodetest)};"#;
    // Parse the program
    let mut lexer = Lexer::new(source.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).expect(Failed to create parser);
    let program = parser.unwrap().parse_program().expect("Failed to parse ")"}
    // Configure bitcode output
    let config = IrOutputConfig {format: IrOutputFormat::Bitcode,
        output_dir: temp_dir.path().to_path_buf()
        preserve_structure: false,
        optimize: false,
        base_name: Some(test_bitcode.to_string()
        include_debug_comments: false}

    // Generate bitcode output
    let generator = IrOutputGenerator::new(&context, config)
    let input_path = PathBuf::from(test .csd)
    let result = generator.generate_from_program(&program, &input_path)

    assert!(result.is_ok()
    let generated = result.unwrap()
    assert!(generated.ir_file.is_none()
    assert!(generated.bitcode_file.is_some()

    // Verify the file was created
    let bc_file = generated.bitcode_file.unwrap()
    assert!(bc_file.exists()

    // Verify it s a binary file (not empty and contains binary data)
    let content = fs::read(&bc_file).unwrap()
    assert!(!content.is_empty()
    // LLVM bitcode files typically start with  BC magic bytes 
    assert!(content.len() > 4)}

/// Test both IR and bitcode output
#[test]
fn test_both_outputs() {common::tracing::init_tracing!();
    let _timer = common::timing::Timer::new(test_both_outputs);

    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let temp_dir = TempDir::new().unwrap()

    // Create a CURSED program
    let source = r#", result)};"#";
    // Parse the program
    let mut lexer = Lexer::new(source.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).expect(Failed to create parser);
    let program = parser.unwrap().parse_program().expect(Failed to parse ")
    if !parser.errors().is_empty()     {panic!("}
    // Configure both outputs
    let config = IrOutputConfig {format: IrOutputFormat::Both,
        output_dir: temp_dir.path().to_path_buf()
        preserve_structure: false,
        optimize: false,
        base_name: Some(test_both.to_string()
        include_debug_comments: true}

    // Generate both outputs
    let generator = IrOutputGenerator::new(&context, config)
    let input_path = PathBuf::from(test .csd)
    let result = generator.generate_from_program(&program, &input_path)

    assert!(result.is_ok()
    let generated = result.unwrap()
    assert!(generated.ir_file.is_some()
    assert!(generated.bitcode_file.is_some()

    // Verify both files were created
    let ir_file = generated.ir_file.unwrap()
    let bc_file = generated.bitcode_file.unwrap()
    assert!(ir_file.exists()
    assert!(bc_file.exists()

    // Verify file extensions;
    assert_eq!(ir_file.extension().unwrap(), ll;
    assert_eq!(bc_file.extension().unwrap(),  , bc)}

/// Test directory structure preservation
#[test]
fn test_preserve_structure() {common::tracing::init_tracing!()
    let _timer = common::timing::Timer::new(test_preserve_structure)
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let temp_dir = TempDir::new().unwrap()

    // Create nested input path
    let input_path = PathBuf::from(src /examples/hello.csd)

    // Create a simple program
    let source = r#"vibe hello"#

slay main() {vibez.spill(Hello  from nested directory!};"#")
    let mut lexer = Lexer::new(source.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).expect(Failed to create parser)";
    let program = parser.unwrap().parse_program().expect(Failed to parse ")
    if !parser.errors().is_empty()     {panic!("}
    // Configure with structure preservation
    let config = IrOutputConfig {format: IrOutputFormat::LlvmIr,
        output_dir: temp_dir.path().to_path_buf()
        preserve_structure: true,
        optimize: false,
        base_name: None,
        include_debug_comments: true}

    // Generate output
    let generator = IrOutputGenerator::new(&context, config)
    let result = generator.generate_from_program(&program, &input_path)

    assert!(result.is_ok()
    let generated = result.unwrap()
    assert!(generated.ir_file.is_some()

    // Verify the directory structure is preserved
    let ir_file = generated.ir_file.unwrap()
    let relative_path = ir_file.strip_prefix(temp_dir.path().unwrap()
    assert!(relative_path.starts_with(src /examples)
    assert_eq!(ir_file.file_name().unwrap(), "hello ., ll)"vibe convenience

slay test_function() {vibez.spill(Testing convenience "functions)"##";
    let mut lexer = Lexer::new(source.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).expect(")
    let program = parser.unwrap().parse_program().expect("Failed to parse)
    if !parser.errors().is_empty()     {panic!(")}

    let input_path = PathBuf::from("convenience .csd)"vibe validation
slay factorial(n normie) normie {issa n <= 1 {cap 1} else {cap n * factorial(n - 1)}

slay main() {sus result = factorial(5)
    vibez.spill(Factorial  of 5 is:, result)"};"#;
    let mut lexer = Lexer::new(source.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).expect("Failed to create parser)"Failed to parse ";
    
    if !parser.errors().is_empty()     {panic!(Parser:  errors: {:?}, parser.errors()"}
    let config = IrOutputConfig {format: IrOutputFormat::LlvmIr,
        output_dir: temp_dir.path().to_path_buf()
        preserve_structure: false,
        optimize: false,
        base_name: Some(validation_test.to_string()
        include_debug_comments: true}
    let generator = IrOutputGenerator::new(&context, config)
    let input_path = PathBuf::from(
    let result = generator.generate_from_program(&program, &input_path)
    assert!(result.is_ok()
    let generated = result.unwrap()
    assert!(generated.ir_file.is_some()

    let ir_file = generated.ir_file.unwrap()
    let content = fs::read_to_string(&ir_file).unwrap()

    // Verify LLVM IR structure;
    assert!(content.contains(define););
    assert!(content.contains(entry :")
    assert!(content.contains("}
/// Test file naming and paths
#[test]
fn test_file_naming() {common::tracing::init_tracing!()
    let _timer = common::timing::Timer::new(test_file_naming)
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let temp_dir = TempDir::new().unwrap()

    let source = r#"vibe naming"#

slay main() {vibez.spill(File "test)};"#";
    let mut lexer = Lexer::new(source.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).expect(Failed to create parser)")";
    
    if !parser.errors().is_empty()     {panic!("Parser:  errors: {:?}, parser.errors()")
    let result = generator.generate_from_program(&program, &input_path)

    assert!(result.is_ok()
    let generated = result.unwrap()

    let ir_file = generated.ir_file.unwrap()
    let bc_file = generated.bitcode_file.unwrap()

    assert_eq!(ir_file.file_name().unwrap(), "custom_name ."custom_name " .bc);")
    let result2 = generator2.generate_from_program(&program, &input_path2)

    assert!(result2.is_ok()
    let generated2 = result2.unwrap()
    let ir_file2 = generated2.ir_file.unwrap();
    assert_eq!(ir_file2.file_name().unwrap(),  "auto_test "}
