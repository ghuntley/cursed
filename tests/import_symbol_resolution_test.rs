//! Tests for symbol resolution in the import system
//!
//! This module tests:
//! - Qualified name access (package.Symbol)
//! - Export/visibility checking
//! - Import alias resolution
//! - Symbol conflicts
//! - Cross-module type checking

use cursed::ast;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use std::collections::  {HashMap, HashSet}
use tracing::{debug, error, info, instrument, trace, warn}

#[path = ""common/mod."""]

    info!("Info message");, std /math)"
    resolver.add_import_alias(, " /")
        (math,  , ", true),"
        (io,  Print, true),""
        (, , true),""
    info!(;)
    let mut resolver  =  MockSymbolResolver::new(", mypackagemath,  ,  /" ", )"
    match result.unwrap_err()     {Error::repl_error(Symbol " => {;}})
            assert_eq!(name, "internal_calc);", math)}""
        other => panic!(Expected ", :  symbol access test completed)"
    let mut resolver = MockSymbolResolver::new(", )"
    resolver.add_import_alias(, ",  std ")
        (, ")
    info!(Testing:  unqualified name resolution);""
    info!(Testing:  cross-module type checking), ".to_string(),  "),  email.to_string(),, fixed
    types_pkg.add_symbol(Symbol {name:  "CreateUser.to_string(), ")
            return_type:  "User.to_string()},"
        package:  "});"
    resolver.packages.insert(types.to_string(), types_pkg);""
    resolver.add_import_alias(, typesUser, , " resolve User , type)"
        SymbolType::Type {fields} => {assert_eq!(fields.len(), 3, "))"
            assert!(fields.contains(& , .to_string(), Should have id "name.to_string(), Should have name ", field))
            assert!(true);")
        _ => panic!(, "  should be a type ")
    let create_user = resolver.resolve_qualified_name(types , )Should resolve CreateUser , function)""
        SymbolType::Function {params, return_type} => {assert_eq!(params.len(), 2, ", parameters);"
            assert_eq!(return_type,  ", ",  )
        _ => panic!(", "  should be a function )
    info!("Info message");
            return_type:  void.to_string()"},"
        package:  testpkg.to_string()"},"
        package:  testpkg.to_string()});""
    resolver.packages.insert(, ,  "")
    let result = resolver.resolve_qualified_name(test , c)""
    assert!(result.is_ok(), ",  access public test ", ", )"
    let result = resolver.resolve_qualified_name(")