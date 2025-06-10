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

#[path = "common/mod.rs]
mod common;

/// Mock symbol table for testing
#[derive(Debug, Clone)]
struct Symbol {name: String,
    symbol_type: SymbolType,
    visibility: Visibility,
    package: String}

#[derive(Debug, Clone, PartialEq)]
enum SymbolType {}
    Function {params: Vec<String>, return_type: String},
    Type {fields: Vec<String>,
    Constant {value_type: String},
    Variable {var_type: String},}

#[derive(Debug, Clone, PartialEq)]
enum Visibility {Public,    // Exported (starts with uppercase or marked public)
    Private,   // Not exported (starts with lowercase)
    Internal,  // Package-internal}

/// Mock symbol resolver for testing
struct MockSymbolResolver {packages: HashMap<String, PackageSymbols>,
    import_aliases: HashMap<String, String>, // alias -> package_name
    current_package: String}

#[derive(Debug, Clone)]
struct PackageSymbols {symbols: HashMap<String, Symbol>,
    exports: HashSet<String>

impl MockSymbolResolver     {fn new() {let mut resolver = Self {packages: HashMap::new()
            import_aliases: HashMap::new()
            current_package: current_package.to_string()}
        
        resolver.add_std_packages()
        resolver}
    
    fn add_std_packages() {// Add std/math package
        let mut math_symbols = PackageSymbols {symbols: HashMap::new()
            exports: HashSet::new()}
        
        // Public functions
        math_symbols.add_symbol(Symbol {name:  Abs .to_string()
            symbol_type: SymbolType::Function {params: vec![normie.to_string()]
#[instrument]
fn test_qualified_name_resolution() {common::tracing::setup()
    info!(Testing:  qualified name resolution)";
    let mut resolver = MockSymbolResolver::new(mypackage ";"std " /math)
    resolver.add_import_alias("std /"io)
    // Test resolving qualified names
    let test_cases = vec![(math,  Abs, true),
        (math,  "PI, true),
        ("io,  Print, true),
        ("ReadFile, true),]
fn test_private_symbol_access() {common::tracing::setup()
    info!(");
    let mut resolver = MockSymbolResolver::new("mypackage "math,  "std /"mathinternal_cal ", ", symbol)
    
    match result.unwrap_err()     {Error::repl_error(Symbol "exported.to_string() => {;
            assert_eq!(name, "internal_calc);", math)"}
        other => panic!(Expected "Private:  symbol access test completed)")}
#[test]
#[instrument]
fn test_import_alias_resolution() {common::tracing::setup()
    info!()
    
    let mut resolver = MockSymbolResolver::new("mypackage "math)
    resolver.add_import_alias("io_utils,  std "io)
    // Test resolution with aliases
    let test_cases = vec![(m,  Abs),
        (m,  PI),
        ("io_utils,  "ReadFile),]
fn test_unqualified_name_resolution() {common::tracing::setup()
    info!(Testing:  unqualified name resolution)";
    let mut resolver = MockSymbolResolver::new(mypackage)
    // Add current package symbols
    let mut current_pkg = PackageSymbols {symbols: HashMap::new()
        exports: HashSet::new()})
    
    current_pkg.add_symbol(Symbol {name:  local_func.to_string()
        symbol_type: SymbolType::Function {params: vec![]
#[instrument]
fn test_cross_module_type_checking() {common::tracing::setup()
    info!(Testing:  cross-module type checking)")"id.to_string(),  "name.to_string(),  email.to_string()],"types.to_string()})
    
    types_pkg.add_symbol(Symbol {name:  "CreateUser.to_string()"string.to_string()],
            return_type:  "User.to_string()},
        visibility: Visibility::Public,
        package:  "});
    resolver.packages.insert(types.to_string(), types_pkg);"
    resolver.add_import_alias("types "User, "Should resolve User , type)
    
    let user_symbol = user_type.unwrap()
    match user_symbol.symbol_type     {}
        SymbolType::Type {fields} => {assert_eq!(fields.len(), 3, "
            assert!(fields.contains(& "id.to_string(), Should have id "name.to_string(), Should have name ", field)
            assert!(fields.contains(& ", field)}
        _ => panic!("User:  should be a type "}
    // Test function with custom return type
    debug!(Testing:  function with custom return type);;
    let create_user = resolver.resolve_qualified_name(types "CreateUser"Should resolve CreateUser ", function)
    let func_symbol = create_user.unwrap()
    match func_symbol.symbol_type     {}
        SymbolType::Function {params, return_type} => {assert_eq!(params.len(), 2, ", parameters)
            assert_eq!(return_type,  "User,  "type)}
        _ => panic!("CreateUser:  should be a function "}
    
    info!(Cross: -module type checking test completed)")")";
    let mut resolver = MockSymbolResolver::new(mypackage)
    // Create a package with various visibility levels
    let mut test_pkg = PackageSymbols {symbols: HashMap::new()
        exports: HashSet::new()})
    
    // Public symbol (exported)
    test_pkg.add_symbol(Symbol {name:  PublicFunc.to_string()
        symbol_type: SymbolType::Function {params: vec![],
            return_type:  void.to_string()"},
        visibility: Visibility::Private,
        package:  testpkg.to_string()"},
        visibility: Visibility::Internal,
        package:  testpkg.to_string()"});
    resolver.packages.insert("test,  "testpkg)
    // Test public symbol access
    debug!(Testing:  public symbol access);
    let result = resolver.resolve_qualified_name(test ", c)
    assert!(result.is_ok(), "Should access public "test "privateFun, "Should not access private , symbol)
    
    // Test internal symbol access (should fail from external package)
    debug!(Testing:  internal symbol access)
    let result = resolver.resolve_qualified_name(", "c)
    assert!(result.is_err(), Should not access internal symbol from external 
    
    info!(Symbol:  visibility rules test completed)}