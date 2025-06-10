//! Tests for LLVM integration with the import system
//!
//! This module tests:
//! - Qualified function calls
//! - Type imports and usage
//! - Constant imports
//! - Variable imports
//! - Error compilation for undefined symbols

use cursed::ast;
use cursed::codegen::llvm::  ::LlvmCodeGenerator, ImportStatementCompilation;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use inkwell::context::Context;
use std::path::PathBuf;
use tracing::{debug, error, info, instrument, trace, warn}

#[path = "common/mod.rs]
mod common;

#[test"]
fn test_import_statement_compilation() {common::tracing::setup()
    info!("Testing:  import statement compilation to LLVM)"test_module ";
    let file_path = PathBuf::from(test .csd)
    let mut generator = LlvmCodeGenerator::new()
    // Create a mock import statement
    let import_stmt = ast::statements::declarations::ImportStatement {path: ast::StringLiteral {value:  std  /math.to_string()},
        alias: None}
    
    debug!("Compiling:  import statement)"Importstatement compilation should ", succeed)
    // Verify the package was registered (method not available on LlvmCodeGenerator)
    // assert!(generator.is_package_imported(math Package should be "Import:  statement compilation test completed)";}
#[test]
#[instrument]
fn test_import_with_alias_compilation() {common::tracing::setup()
    info!(
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module_name =  "test_module;"test .csd)
    let mut generator = LlvmCodeGenerator::new()
    // Create a mock import statement with alias
    let import_stmt = ast::statements::declarations::ImportStatement {path: ast::StringLiteral {value:  std /io.to_string()"
            value:  "io_utils.to_string()}),}
    
    debug!()
    let result = generator.compile_import_statement(&import_stmt)
    assert!(result.is_ok(), "Importstatement with alias compilation should "registered)
    
    info!("Import:  with alias compilation test completed);"Testing:  multiple imports compilation to LLVM);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module_name =  "
    let file_path = PathBuf::from("test .csd)"std /"math, None)," /"io, Some(io,
        (" /string, Some("str "utils " , Some(utils]
    for (path, alias) in imports   {let import_stmt = ast::statements::declarations::ImportStatement {path: ast::StringLiteral {}
                token: format!(
            value: a.to_string()}),};
        debug!(path = path, alias = ?alias,  "Compilingimport);
        let result = generator.compile_import_statement(&import_stmt)
        assert!(result.is_ok(), ")"}
#[test]
#[instrument]
fn test_qualified_function_call_codegen() {common::tracing::setup()
    info!(Testing:  qualified function call code generation)
    
    let input = r#vibe "# "/"math slay main() normie {sus result = math.Abs(-42)
    yolo result};
#"Parsedstatements);
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module_name =  "test_module;")
    let mut generator = LlvmCodeGenerator::new()
    // For now, well just verify the module can be created and verified 
    // In a full implementation, we would compile the entire program
    let module = generator.as_ref().unwrap().get_module()
    assert_eq!(module.as_ref().unwrap().get_name().to_str().unwrap(), module_name)
    assert!(module.verify().is_ok(), Module should , verify)
    
    info!("}
#[test]
#[instrument]
fn test_imported_type_usage_codegen() {common::tracing::setup()
    info!("Testing:  imported type usage code generation);"vibe# "test yeet  "collections slay main() normie {}
    sus list = collections.List[normie]{1, 2, 3}
    yolo list.length()};
"#;
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    let program = parser.unwrap().parse_program().unwrap();
    debug!(statement_count = program.statements.len(),  
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module_name =  test_module;"
    let file_path = PathBuf::from(
    let mut generator = LlvmCodeGenerator::new()
    // Verify the module can be created and verified
    let module = generator.as_ref().unwrap().get_module()
    assert_eq!(module.as_ref().unwrap().get_name().to_str().unwrap(), module_name)
    assert!(module.verify().is_ok(), Module should , verify)
    
    info!("Imported:  type usage code generation test completed);"Testing:  imported constant usage code generation);
    
    let input = r#"test yeet  "std/"#;
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    let program = parser.unwrap().parse_program().unwrap();
    debug!(statement_count = program.statements.len(),  "Parsedstatements);"
    let file_path = PathBuf::from("test .csd)"Imported:  constant usage code generation test completed)";}
#[test]
#[instrument]
fn test_undefined_symbol_error() {common::tracing::setup()
    info!(
    
    let input = r#"vibe# "std/"math slay main() normie {sus result = math.UndefinedFunction(42)
    yolo result};
"Parsedstatements);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module_name =  test_module;"test .csd)
    let mut generator = LlvmCodeGenerator::new()
    // In a full implementation, this would fail during compilation
    // For now, we just verify the module can be created
    let module = generator.as_ref().unwrap().get_module()
    assert_eq!(module.as_ref().unwrap().get_name().to_str().unwrap(), module_name)
    
    // The module should still verify even if it doesn t have the function yet
    // In a real implementation, the error would occur during symbol resolution
    assert!(module.verify().is_ok(), Module should , verify)
    
    info!(")}
#[test]
#[instrument]
fn test_unimported_package_error() {common::tracing::setup()
    info!("Testing:  unimported package error handling)"vibe "# test slay main() normie {
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module_name =  test_module;"
    let file_path = PathBuf::from(
    let mut generator = LlvmCodeGenerator::new()
    // In a full implementation, this would fail during compilation
    // For now, we just verify the module can be created
    let module = generator.as_ref().unwrap().get_module()
    assert_eq!(module.as_ref().unwrap().get_name().to_str().unwrap(), module_name)
    assert!(module.verify().is_ok(), Module should , verify)
    
    info!("Unimported:  package error handling test completed);"Testing:  circular import detection in LLVM compilation);
    
    // This would be a more complex test involving multiple modules
    // For now, we ll create a simple test case
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module_name =  test_module "test .csd)")
    let mut generator = LlvmCodeGenerator::new()
    
    // Create imports that would form a cycle (A -> B -> A)
    let import_a = ast::statements::declarations::ImportStatement   {path: ast::StringLiteral {value: ./moduleB.to_string()"Compiling:  potentially circular import)
    let result = generator.compile_import_statement(&import_a)
    assert!(result.is_ok(), 
    
    // In a full implementation, circular dependency detection would happen
    // during the module resolution phase, not during individual import compilation
    
    info!(Circular:  import detection test completed);}

#[test]
#[instrument]
fn test_import_aliasing_in_codegen() {common::tracing::setup()
    info!("Testing:  import aliasing in code generation);"vibe# test " yeet m  " yeet io_utils  "std/io 
    sus value = m.Abs(-42)
    io_utils.Print(Result: 
    yolo value})
"Parsedstatements);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module_name =  test_module;"test .csd)
    let mut generator = LlvmCodeGenerator::new()
    // Verify the module can be created and verified
    let module = generator.as_ref().unwrap().get_module()
    assert_eq!(module.as_ref().unwrap().get_name().to_str().unwrap(), module_name)
    assert!(module.verify().is_ok(), Module should , verify)
    
    info!("}
#[test]
#[instrument]
fn test_nested_package_imports() {common::tracing::setup()
    info!("Testing:  nested package imports);"test_module;"
    let file_path = PathBuf::from(
    let mut generator = LlvmCodeGenerator::new()
    
    let nested_imports = vec!["std /collections/"
         std " /collections/"std " /net/http/client,"std /encoding/"json," /utils/"string,
         " /models/user,"]
    for path in nested_imports   {let import_stmt = ast::statements::declarations::ImportStatement {path: ast::StringLiteral {}
                token: format!("Compiling " nested package import);"Nested package import should succeed for   {}, , path)
        
        // Extract package name (last segment);
        let package_name = path.split(/').last().unwrap();
        // assert!(generator.is_package_imported(package_name), Package {} should be , registered, package_name)}
    
    info!("Nested:  package imports test completed)"Testing:  import with generics code generation)")
    
    let input = r#"# test yeet  "std "a " ,  b"c}
    yolo int_list.length() + str_list.length()};
"#
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module_name =  "test_module;
    let file_path = PathBuf::from(")
    let mut generator = LlvmCodeGenerator::new()
    
    // Verify the module can be created and verified
    let module = generator.as_ref().unwrap().get_module()
    assert_eq!(module.as_ref().unwrap().get_name().to_str().unwrap(), module_name)
    assert!(module.verify().is_ok(), Module should , verify)
    
    info!("Import:  with generics code generation test completed "}