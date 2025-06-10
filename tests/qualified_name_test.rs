//! Tests for qualified name support in the CURSED language
//!
//! This test suite validates the enhanced dot expression system that supports
//! accessing all symbol types from imported packages.

use cursed::ast::  {QualifiedName, QualifiedSymbolKind}
use cursed::resolver::symbol_table::::GlobalSymbolTable, PackageSymbolTable, SymbolKind;
use cursed::error::Error;

// Initialize test tracing
fn init_test_tracing() {use tracing_subscriber::{EnvFilter, FmtSubscriber}
    let _ = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env()
        .with_test_writer()
        .try_init()}

#[test]
fn test_qualified_name_creation() {init_test_tracing()
    
    let qualified = QualifiedName::new()
        ..to_string()
         math.to_string()"
         sqrt.to_string()"math;);
    assert_eq!(qualified.symbol, "sqrt);
    assert_eq!(qualified.symbol_kind, QualifiedSymbolKind::Unknown)
    assert_eq!(qualified.string(),  "sqrt)"}
#[test]
fn test_qualified_name_with_alias() {init_test_tracing()
    
    let qualified = QualifiedName::new_with_alias()
        ."mathematics.to_string()
         "sqrt.to_string()");
    assert_eq!(qualified.package, "mathematics);
    assert_eq!(qualified.symbol, ", math)
    assert_eq!(qualified.string(),  "math "}
#[test]
fn test_qualified_name_with_kind() {init_test_tracing()
    
    let qualified = QualifiedName::new_with_kind()
        "..to_string()
         "
         Request.to_string()
        QualifiedSymbolKind::Type)
    assert_eq!(qualified.symbol_kind, QualifiedSymbolKind::Type)}

#[test]
fn test_package_symbol_table() {init_test_tracing();
    let mut package = PackageSymbolTable::new("
            return_type:  "f64.to_string()
            public: true},
        true)
    
    // Define a public constant
    let pi_symbol = package.define_symbol()
         Pi.to_string()
        SymbolKind::Constant {const_type:  f64.to_string()", 3.14159265359 .to_string()
            public: true},
        true)
    
    // Define a private function
    let _private_symbol = package.define_symbol()
         internal_fun c .to_string()
        SymbolKind::Function {params: vec![],
        return_type:  
        public: true}
    assert!(function_kind.is_public()
    
    let private_const = SymbolKind::Constant {const_type:  string.to_string()"
        value: Some("interface.to_string()
        public: true}
    assert!(public_type.is_public()
    
    let mutable_var = SymbolKind::Variable {var_type:  i32.to_string()"json.to_string()
    global.register_package(math_package)
    global.register_package(http_package)
    global.register_package(json_package)
    
    let packages = global.list_packages()
    assert_eq!(packages.len(), 3)
    assert!(packages.contains(& math.to_string()"
    assert!(packages.contains(& "json.to_string()"}
#[test]
fn test_current_package_management() {init_test_tracing()
    
    let mut global = GlobalSymbolTable::new()
    
    // Default current package should be  main 
    assert_eq!(global.current_package, main)
    
    // Create and register a package
    let test_package = PackageSymbolTable::new(, test.to_string()
    global.register_package(test_package)
    
    // Set current package
    global.set_current_package(test.to_string();
    assert_eq!(global.current_package,  test;
    
    // Get current package);
    assert!(global.get_current_package().is_some()
    assert_eq!(global.get_current_package().unwrap().package_name,  test);
    
    // Get mutable current package
    assert!(global.get_current_package_mut().is_some()}
