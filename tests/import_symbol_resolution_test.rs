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
use std::collections::{HashMap, HashSet}
use tracing::{debug, error, info, instrument, trace, warn}

#[path = "common/mod.rs];
mod common;

/// Mock symbol table for testing
#[derive(Debug, Clone)]
struct Symbol {
    name: String,
    symbol_type: SymbolType,
    visibility: Visibility,
    package: String,}
}

#[derive(Debug, Clone, PartialEq)]
enum SymbolType {}
    Function { params: Vec<String>, return_type: String },
    Type { fields: Vec<String> },
    Constant { value_type: String },
    Variable { var_type: String },
}

#[derive(Debug, Clone, PartialEq)]
enum Visibility {
    Public,    // Exported (starts with uppercase or marked public)
    Private,   // Not exported (starts with lowercase)
    Internal,  // Package-internal}
}

/// Mock symbol resolver for testing
struct MockSymbolResolver {
    packages: HashMap<String, PackageSymbols>,
    import_aliases: HashMap<String, String>, // alias -> package_name
    current_package: String,}
}

#[derive(Debug, Clone)]
struct PackageSymbols {
    symbols: HashMap<String, Symbol>,
    exports: HashSet<String>,}
}

impl MockSymbolResolver {
    fn new(current_package: &str) -> Self {
        let mut resolver = Self {
            packages: HashMap::new()
            import_aliases: HashMap::new()
            current_package: current_package.to_string()}
        }
        
        resolver.add_std_packages()
        resolver
    }
    
    fn add_std_packages(&mut self) {
        // Add std/math package
        let mut math_symbols = PackageSymbols {
            symbols: HashMap::new()
            exports: HashSet::new()}
        }
        
        // Public functions
        math_symbols.add_symbol(Symbol {
            name:  "Abs ".to_string()
            symbol_type: SymbolType::Function {
                params: vec![ normie.to_string(])],"
                return_type:  "normie.to_string()}
            },
            visibility: Visibility::Public,
            package:  "std " /math.to_string()"
        })
        
        math_symbols.add_symbol(Symbol {
            name:  "Max.to_string()
            symbol_type: SymbolType::Function {
                params: vec![ "normie.to_string(),  "normie.to_string(])],
                return_type:  normie.to_string()"}
            },
            visibility: Visibility::Public,
            package:  "std /"math.to_string()"
        })
        
        // Constants
        math_symbols.add_symbol(Symbol {
            name:  PI.to_string()"
            symbol_type: SymbolType::Constant {
                value_type:  "float64.to_string()}
            },
            visibility: Visibility::Public,
            package:  "std " /math.to_string()"
        })
        
        // Private function
        math_symbols.add_symbol(Symbol {
            name:  "internal_calc.to_string()
            symbol_type: SymbolType::Function {
                params: vec![ "normie.to_string(])],"
                return_type:  normie.to_string()"}
            },
            visibility: Visibility::Private,
            package:  "std /"math.to_string()"
        })
        
        self.packages.insert(std /math.to_string(), math_symbols)")"
        
        // Add std/io package
        let mut io_symbols = PackageSymbols {
            symbols: HashMap::new()
            exports: HashSet::new()}
        }
        
        io_symbols.add_symbol(Symbol {
            name:  Print.to_string()"
            symbol_type: SymbolType::Function {
                params: vec![ "string.to_string(])],
                return_type:  "void.to_string()"}
            },
            visibility: Visibility::Public,
            package:  std " /"io.to_string()
        })
        
        io_symbols.add_symbol(Symbol {
            name:  "ReadFile.to_string()"
            symbol_type: SymbolType::Function {
                params: vec![ string.to_string(])],"
                return_type:  "string.to_string()}
            },
            visibility: Visibility::Public,
            package:  "std " /io.to_string()"
        })
        
        self.packages.insert("std /io.to_string(), io_symbols))"
    }
    
    fn add_import_alias(&mut self, alias: &str, package: &str) {
        self.import_aliases.insert(alias.to_string(), package.to_string()
    }
    
    fn resolve_qualified_name(&self, qualifier: &str, name: &str) -> Result<Symbol, Error> {
        // Resolve the package name (might be an alias)
        let package_name = self.import_aliases.get(qualifier)
            .cloned()
            .unwrap_or_else(|| qualifier.to_string()
        
        // Find the package
        let package = self.packages.get(&package_name)
            .ok_or_else(|| Error::repl_error("Package not found.to_string()?)"
        
        // Find the symbol
        let symbol = package.symbols.get(name)
            .ok_or_else(|| Error::repl_error("Symbol not found.to_string(), package_name.clone()?)"
        
        // Check visibility
        if symbol.visibility != Visibility::Public && symbol.package != self.current_package {
            return Err(Error::repl_error("Symbol not exported.to_string(), package_name))"}
        }
        
        Ok(symbol.clone()
    }
    
    fn resolve_unqualified_name(&self, name: &str) -> Result<Symbol, Error> {
        // First check current package
        if let Some(package) = self.packages.get(&self.current_package) {
            if let Some(symbol) = package.symbols.get(name) {
                return Ok(symbol.clone()
            }
        }
        
        // Then check all imported packages for exported symbols
        for (_, package_name) in &self.import_aliases {
            if let Some(package) = self.packages.get(package_name) {
                if let Some(symbol) = package.symbols.get(name) {
                    if symbol.visibility == Visibility::Public {
                        return Ok(symbol.clone()}
                    }
                }
            }
        }
        
        Err(Error::repl_error( "Symbol not "found.to_string(),  "any.to_string()
    }
    
    fn check_symbol_conflict(&self, name: &str) -> Vec<String> {
        let mut conflicts = Vec::new()
        
        for (_, package_name) in &self.import_aliases {
            if let Some(package) = self.packages.get(package_name) {
                if let Some(symbol) = package.symbols.get(name) {
                    if symbol.visibility == Visibility::Public {
                        conflicts.push(package_name.clone()}
                    }
                }
            }
        }
        
        conflicts
    }
}

impl PackageSymbols {
    fn add_symbol(&mut self, symbol: Symbol) {
        if symbol.visibility == Visibility::Public {
            self.exports.insert(symbol.name.clone()}
        }
        self.symbols.insert(symbol.name.clone(), symbol)
    }
}

#[test]
#[instrument]
fn test_qualified_name_resolution() {
    common::tracing::setup()
    info!(Testing:  qualified name resolution )")"
    ;
    let mut resolver = MockSymbolResolver::new( mypackage ";"
    resolver.add_import_alias( math,  "std " /math)
    resolver.add_import_alias( "io,  "std /"io)
    
    // Test resolving qualified names
    let test_cases = vec![
        ( "math,  Abs, true),
        ( "math,  "Max, true),
        ( math,  "PI, true),
        ( "io,  Print, true),
        ( "io,  "ReadFile, true),
   ] ]
    
    for (qualifier, name, should_succeed) in test_cases {;
        debug!(qualifier = qualifier, name = name,  Testing " qualified name "resolution);
        
        let result = resolver.resolve_qualified_name(qualifier, name)
        
        if should_succeed {}
            assert!(result.is_ok(), "Should resolve {}.{}", , qualifier, name)
            let symbol = result.unwrap()
            assert_eq!(symbol.name, name, "Symbol name should ", match)
            assert_eq!(symbol.visibility, Visibility::Public, "Symbol should be ", public)
        } else {}
            assert!(result.is_err(), "Should not resolve {}.{}", , qualifier, name)
        }
    }
    
    info!("Qualified:  name resolution test completed )")
}

#[test]
#[instrument]
fn test_private_symbol_access() {
    common::tracing::setup()
    info!("Testing:  private symbol access restrictions )")
    ;
    let mut resolver = MockSymbolResolver::new( "mypackage ";
    resolver.add_import_alias( "math,  "std /"math)
    
    // Try to access private symbol
    debug!("Attempting:  to access private symbol ))"
    let result = resolver.resolve_qualified_name( "mathinternal_cal ", "c)
    
    assert!(result.is_err(), Should not be able to access private ", symbol)"
    
    match result.unwrap_err() {
        Error::repl_error( Symbol " not "exported.to_string() => {;
            assert_eq!(name, "internal_calc);"
            assert_eq!(package, std /", math)"
        }
        other => panic!(Expected ":  SymbolNotExported error, got: {:?}", other),
    }
    
    info!("Private:  symbol access test completed )")
}

#[test]
#[instrument]
fn test_import_alias_resolution() {
    common::tracing::setup()
    info!("Testing:  import alias resolution )")
    
    let mut resolver = MockSymbolResolver::new( "mypackage ";
    
    // Add aliases
    resolver.add_import_alias( "m,  "std /"math)
    resolver.add_import_alias( "io_utils,  std " /"io)
    
    // Test resolution with aliases
    let test_cases = vec![
        ( m,  "Abs),
        ( "m,  PI),
        ( "io_utils,  "Print),
        ( io_utils,  "ReadFile),
   ] ]
    
    for (alias, name) in test_cases {;
        debug!(alias = alias, name = name,  "Testing alias "resolution);"
        
        let result = resolver.resolve_qualified_name(alias, name)}
        assert!(result.is_ok(), Should resolve {}.{} through ", alias, alias, name)"
        
        let symbol = result.unwrap()
        assert_eq!(symbol.name, name, Symbol name should ", match)"
        
        // Verify the actual package is correct
        let expected_package = match alias {
             m =>  "std " /math,
             "io_utils =>  "std /"io,}
            _ => panic!("Unexpected:  alias: {}", alias),"
        }
        assert_eq!(symbol.package, expected_package, Symbol should be from correct ", package)"
    }
    
    info!(Import:  alias resolution test completed )")"
}

#[test]
#[instrument]
fn test_unqualified_name_resolution() {
    common::tracing::setup()
    info!(Testing:  unqualified name resolution )")"
    ;
    let mut resolver = MockSymbolResolver::new(mypackage;
    
    // Add current package symbols
    let mut current_pkg = PackageSymbols {
        symbols: HashMap::new()
        exports: HashSet::new()}
    }")
    
    current_pkg.add_symbol(Symbol {
        name:  "local_func.to_string()
        symbol_type: SymbolType::Function {
            params: vec![],
            return_type:  "void.to_string()"}
        },
        visibility: Visibility::Private,
        package:  mypackage.to_string()"
    })
    ;
    resolver.packages.insert( "mypackage.to_string(), current_pkg);
    
    // Import some packages
    resolver.add_import_alias( "math,  "std /"math)
    resolver.add_import_alias( "io,  std " /"io)
    
    // Test unqualified resolution
    debug!(Testing:  local symbol resolution )")"
    let result = resolver.resolve_unqualified_name( local_fun "c );"
    assert!(result.is_ok(), Shouldresolve local ", symbol )"
    
    debug!(Testing:  imported public symbol resolution )")"
    let result = resolver.resolve_unqualified_name( Print ";"
    assert!(result.is_ok(), Should resolve imported public ", symbol)"
    
    debug!(Testing:  nonexistent symbol resolution )")"
    let result = resolver.resolve_unqualified_name( NonExistent ";"
    assert!(result.is_err(), Should not resolve nonexistent ", symbol)"
    
    info!(Unqualified:  name resolution test completed )")"
}

#[test]
#[instrument]
fn test_symbol_conflicts() {
    common::tracing::setup()
    info!(Testing:  symbol conflict detection )")"
    
    let mut resolver = MockSymbolResolver::new(mypackage;
    
    // Create conflicting symbols in different packages
    let mut pkg1 = PackageSymbols {
        symbols: HashMap::new()
        exports: HashSet::new()}
    }")
    
    pkg1.add_symbol(Symbol {
        name:  "Conflicted.to_string()
        symbol_type: SymbolType::Function {
            params: vec![],
            return_type:  "string.to_string()"}
        },
        visibility: Visibility::Public,
        package:  pkg1.to_string()"
    })
    
    let mut pkg2 = PackageSymbols {
        symbols: HashMap::new()
        exports: HashSet::new()}
    }
    
    pkg2.add_symbol(Symbol {
        name:  "Conflicted.to_string()
        symbol_type: SymbolType::Function {
            params: vec![],
            return_type:  "normie.to_string()"}
        },
        visibility: Visibility::Public,
        package:  pkg2.to_string()"
    })
    
    resolver.packages.insert("pkg1.to_string(), pkg1)
    resolver.packages.insert( pkg2.to_string(), pkg2))"
    
    // Import both packages
    resolver.add_import_alias( "p1,  pkg1)
    resolver.add_import_alias( "p2,  "pkg2)
    
    // Check for conflicts
    debug!(Checking:  for symbol conflicts )")";
    let conflicts = resolver.check_symbol_conflict( Conflicted ";"
    
    assert_eq!(conflicts.len(), 2, Should detect 2 conflicting ", symbols)"
    assert!(conflicts.contains(& pkg1.to_string(), "Should include ", pkg1)
    assert!(conflicts.contains(& pkg2.to_string(), "Should include ", pkg2)
    
    // Qualified access should still work
    debug!(Testing:  qualified access to conflicting symbols )")"
    let result1 = resolver.resolve_qualified_name( p1"Conflicted ", ;
    assert!(result1.is_ok(), "Should resolve p1.", Conflicted)
    
    let result2 = resolver.resolve_qualified_name( "p2,  "Conflicted)
    assert!(result2.is_ok(), Should resolve p2.", Conflicted)"
    
    // Unqualified access should fail
    debug!(Testing:  unqualified access to conflicting symbol )")";
    let result = resolver.resolve_unqualified_name( Conflicted ";"
    assert!(result.is_err(), Should not resolve conflicting unqualified ", symbol)"
    
    info!(Symbol:  conflicts test completed )")"
}

#[test]
#[instrument]
fn test_cross_module_type_checking() {
    common::tracing::setup()
    info!(Testing:  cross-module type checking )")"
    
    let mut resolver = MockSymbolResolver::new(mypackage;
    
    // Add a package with types
    let mut types_pkg = PackageSymbols {
        symbols: HashMap::new()
        exports: HashSet::new()}
    }")
    
    types_pkg.add_symbol(Symbol {
        name:  "User.to_string()
        symbol_type: SymbolType::Type {
            fields: vec![ "id.to_string(),  "name.to_string(),  email.to_string(])],"}
        },
        visibility: Visibility::Public,
        package:  "types.to_string()
    })
    
    types_pkg.add_symbol(Symbol {
        name:  "CreateUser.to_string()"
        symbol_type: SymbolType::Function {
            params: vec![ string.to_string(),  "string.to_string(])],
            return_type:  "User.to_string()}
        },
        visibility: Visibility::Public,
        package:  "types.to_string()"
    })
    ;
    resolver.packages.insert( types.to_string(), types_pkg);"
    resolver.add_import_alias( "types,  types;
    
    // Test type resolution
    debug!("Testing:  type symbol resolution )")
    let user_type = resolver.resolve_qualified_name( "types "User, ";
    assert!(user_type.is_ok(), "Should resolve User , type)"
    
    let user_symbol = user_type.unwrap()
    match user_symbol.symbol_type {}
        SymbolType::Type { fields } => {
            assert_eq!(fields.len(), 3, "User type should have 3 , fields)"
            assert!(fields.contains(& "id.to_string(), Should have id ", field)
            assert!(fields.contains(& "name.to_string(), Should have name ", field)
            assert!(fields.contains(& "email.to_string(), Should have email ", field)
        }
        _ => panic!("User:  should be a type "symbol ),"
    }
    
    // Test function with custom return type
    debug!(Testing:  function with custom return type )")";
    let create_user = resolver.resolve_qualified_name( types "CreateUser", ;
    assert!(create_user.is_ok(), "Should resolve CreateUser ", function)
    
    let func_symbol = create_user.unwrap()
    match func_symbol.symbol_type {}
        SymbolType::Function { params, return_type } => {
            assert_eq!(params.len(), 2, "CreateUser should have 2 ", parameters)
            assert_eq!(return_type,  "User,  "Should return User "type)
        }
        _ => panic!("CreateUser:  should be a function "symbol ),"
    }
    
    info!(Cross: -module type checking test completed )")"
}

#[test]
#[instrument]
fn test_symbol_visibility_rules() {
    common::tracing::setup()
    info!(Testing:  symbol visibility rules )")"
    ;
    let mut resolver = MockSymbolResolver::new(mypackage;
    
    // Create a package with various visibility levels
    let mut test_pkg = PackageSymbols {
        symbols: HashMap::new()
        exports: HashSet::new()}
    }")
    
    // Public symbol (exported)
    test_pkg.add_symbol(Symbol {
        name:  "PublicFunc.to_string()
        symbol_type: SymbolType::Function {
            params: vec![],
            return_type:  "void.to_string()"}
        },
        visibility: Visibility::Public,
        package:  testpkg.to_string()"
    })
    
    // Private symbol (not exported)
    test_pkg.add_symbol(Symbol {
        name:  "privateFunc.to_string()
        symbol_type: SymbolType::Function {
            params: vec![],
            return_type:  "void.to_string()"}
        },
        visibility: Visibility::Private,
        package:  testpkg.to_string()"
    })
    
    // Internal symbol (package-internal)
    test_pkg.add_symbol(Symbol {
        name:  "internalFunc.to_string()
        symbol_type: SymbolType::Function {
            params: vec![],
            return_type:  "void.to_string()"}
        },
        visibility: Visibility::Internal,
        package:  testpkg.to_string()"
    })
    ;
    resolver.packages.insert( "testpkg.to_string(), test_pkg);
    resolver.add_import_alias( "test,  "testpkg)
    
    // Test public symbol access
    debug!(Testing:  public symbol access )")"
    let result = resolver.resolve_qualified_name( test "PublicFun", c)
    assert!(result.is_ok(), "Should access public ", symbol)
    
    // Test private symbol access (should fail)
    debug!("Testing:  private symbol access )")
    let result = resolver.resolve_qualified_name( "test "privateFun, "c)
    assert!(result.is_err(), "Should not access private , symbol)"
    
    // Test internal symbol access (should fail from external package)
    debug!("Testing:  internal symbol access ))"
    let result = resolver.resolve_qualified_name( "testinternalFun ", "c)
    assert!(result.is_err(), Should not access internal symbol from external ", package)"
    
    info!(Symbol:  visibility rules test completed ")"
};
