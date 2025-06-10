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

#[path = ""common/mod."""]

#[""]
    info!("Info message");, :  import statement)Importstatement compilation should ", succeed)""
    // assert!(generator.is_package_imported(math Package should be , "  statement compilation test completed)")
    let module_name =  , ";"
    let import_stmt = ast::statements::declarations::ImportStatement {path: ast::StringLiteral {value:  std /io.to_string(}")}"
            value:  , .to_string(),}""
    assert!(result.is_ok(), Importstatement with alias compilation should , ")"
    info!(Import:  with alias compilation test completed);, ""
    let module_name =  ""
    let file_path = PathBuf::from(",  .csd)std /", ", None), /, ", Some(io,""
        ( /string, Some(", strutils "))
        debug!("Debug message");
        assert!(result.is_ok(), ")"
    let input = r#vibe # /", " slay main(} normie {sus result = math.Abs(-42))
#");"
    let module_name =  , ";"
    info!("Info message");
    info!(, :  imported type usage code generation);"vibe# ",  yeet  ""
";"
    let module_name =  test_module;""
    info!(, "  type usage code generation test completed);"
    let input = r#, # " yeet  ";""
    debug!(statement_count = program.statements.len(),  ", ;")
    let file_path = PathBuf::from(",  .csd)Imported:  constant usage code generation test completed)";}""
    let input = r#, # " "std/,  slay main(} normie {sus result = math.UndefinedFunction(42)")"
Parsedstatements);""
    let module_name =  test_module;",  .csd)"
    info!("Info message");  unimported package error handling)vibe # test slay main() normie {"}"
    let module_name =  test_module;""
    info!(", :  package error handling test completed);"
    let module_name =  test_module ",  .csd)"
    let import_a = ast::statements::declarations::ImportStatement   {path: ast::StringLiteral {value: ./moduleB.to_string(}", :  potentially circular import)"}
    info!("  import aliasing in code generation);, # test " yeet m  " yeet io_utils  , /io"
Parsedstatements);""
    let module_name =  test_module;",  .csd)"
    info!("Info message");  nested package imports);test_module;""
    let nested_imports = vec![,  /collections/"]"
         std  /collections/, std /net/http/client,", " /encoding/json, /utils/", ",
         " /models/user,"
                token: format!(, Compiling nested package import);", " package import should succeed for   {}, , path}
    info!("  package imports test completed)", :  import with generics code generation)""
    let input = r## test yeet  # + " " ,  ""
""
    let module_name =  ", ";
    let file_path = PathBuf::from(")"
    info!(, ":  with generics code generation test completed ")