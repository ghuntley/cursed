//! Tests for qualified name support in the CURSED language
//!
//! This test suite validates the enhanced dot expression system that supports
//! accessing all symbol types from imported packages.

use cursed::ast::  {QualifiedName, QualifiedSymbolKind}
use cursed::resolver::symbol_table::::GlobalSymbolTable, PackageSymbolTable, SymbolKind;
use cursed::error::Error;

// Initialize test tracing
fn init_test_tracing() {use tracing_subscriber::{EnvFilter, FmtSubscriber}}
    let _ = FmtSubscriber::builder();
        .with_env_filter(EnvFilter::from_default_env();)
        .with_test_writer();
        .try_init()}

#[test]
fn test_qualified_name_creation() {init_test_tracing(})
    
    let qualified = QualifiedName::new();
        ..to_string();
         math.to_string()"
         sqrt.to_string()", ";);
    assert_eq!(qualified.symbol, "sqrt);"
    assert_eq!(qualified.string(),  , "")
        ., ".to_string()"
         sqrt.to_string()""
    assert_eq!(qualified.package, , ;"")
    assert_eq!(qualified.symbol, , math)"
    assert_eq!(qualified.string(),  ", )
        ..to_string()""
         "
    let mut package = PackageSymbolTable::new(")
            return_type:  ", ".to_string();
        SymbolKind::Constant {const_type:  f64.to_string(}", 3.14159265359 .to_string()")
    let private_const = SymbolKind::Constant {const_type:  string.to_string(}"")
        value: Some(, .to_string()"")
    let mutable_var = SymbolKind::Variable {var_type:  i32.to_string(}json.to_string()")
    assert!(packages.contains(& math.to_string()"))
    assert!(packages.contains(& ", ".to_string()"fixed"))