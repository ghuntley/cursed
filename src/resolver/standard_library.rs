//! Standard library package resolver
//!
//! This module provides resolution for built-in standard library packages
//! such as vibez (output), stringz (string manipulation), mathz (math functions), etc.

use std::collections::HashMap;
use std::sync::Arc;
use crate::ast::base::Program;
use crate::ast::statements::declarations::PackageStatement;
use crate::ast::traits::Statement;

use super::errors::{ResolverError, ResolverResult};

use super::symbol_table::{PackageSymbolTable, Symbol, SymbolKind};

/// Standard library package resolver
#[derive(Debug)]
pub struct StandardLibraryResolver {
    /// Cache of standard library packages
    stdlib_cache: HashMap<String, Arc<PackageSymbolTable>>,
}

impl StandardLibraryResolver {
    /// Create a new standard library resolver
    pub fn new() -> Self {
        let mut resolver = Self {
            stdlib_cache: HashMap::new(),
        };
        
        // Pre-populate with known standard library packages
        resolver.initialize_stdlib_packages();
        resolver
    }

    /// Get a package from the standard library
    pub fn get_package(&self, package_name: &str) -> ResolverResult<&PackageSymbolTable> {
        if let Some(package) = self.stdlib_cache.get(package_name) {
            Ok(package.as_ref())
        } else {
            Err(ResolverError::PackageNotFound(package_name.to_string()))
        }
    }

    /// Initialize standard library packages
    fn initialize_stdlib_packages(&mut self) {
        let stdlib_packages = vec![
            ("vibez", "Core output and formatting functions"),
            ("stringz", "String manipulation utilities"),
            ("mathz", "Mathematical functions and constants"),
            ("timez", "Time and date utilities"),
            ("dropz", "File system operations"),
            ("concurrenz", "Concurrency primitives"),
            ("web_vibez", "Web-related functionality"),
            ("cryptz", "Cryptographic functions"),
            ("rizztemplate", "Template processing"),
            ("quick_test", "Testing utilities"),
            ("vector2d", "2D vector operations"),
            ("reflectz", "Reflection utilities"),
        ];

        for (package_name, _description) in stdlib_packages {
            if let Ok(package) = self.create_stdlib_package(package_name) {
                self.stdlib_cache.insert(package_name.to_string(), Arc::new(package));
            }
        }
    }

    /// Check if a package name is a standard library package
    pub fn is_stdlib_package(&self, package_name: &str) -> bool {
        self.stdlib_cache.contains_key(package_name)
    }

    /// Get available stdlib packages
    pub fn available_packages(&self) -> Vec<String> {
        self.stdlib_cache.keys().cloned().collect()
    }

    /// Create a standard library package
    fn create_stdlib_package(&self, package_name: &str) -> ResolverResult<PackageSymbolTable> {
        // Create a synthetic program for this package
        let program = self.create_stdlib_program(package_name)?;
        
        // Create symbol table from the program
        let mut symbol_table = PackageSymbolTable::from_program_with_name(&program, package_name.to_string());
        
        // Add specific symbols for this package
        self.add_package_symbols(package_name, &mut symbol_table);
        
        Ok(symbol_table)
    }

    /// Create a synthetic program for a standard library package
    fn create_stdlib_program(&self, package_name: &str) -> ResolverResult<Program> {
        let package_stmt = PackageStatement::new(package_name.to_string());
        let mut program = Program::new();
        program.statements.push(Box::new(package_stmt) as Box<dyn Statement>);
        Ok(program)
    }

    /// Add symbols for a specific standard library package
    fn add_package_symbols(&self, package_name: &str, symbol_table: &mut PackageSymbolTable) {
        match package_name {
            "vibez" => self.add_vibez_symbols(symbol_table),
            "stringz" => self.add_stringz_symbols(symbol_table),
            "mathz" => self.add_mathz_symbols(symbol_table),
            "timez" => self.add_timez_symbols(symbol_table),
            "dropz" => self.add_dropz_symbols(symbol_table),
            "concurrenz" => self.add_concurrenz_symbols(symbol_table),
            "web_vibez" => self.add_web_vibez_symbols(symbol_table),
            "cryptz" => self.add_cryptz_symbols(symbol_table),
            "rizztemplate" => self.add_rizztemplate_symbols(symbol_table),
            "quick_test" => self.add_quick_test_symbols(symbol_table),
            _ => {
                tracing::debug!("No specific symbols defined for stdlib package: {}", package_name);
            }
        }
    }

    /// Add symbols for the vibez package (output/formatting)
    fn add_vibez_symbols(&self, symbol_table: &mut PackageSymbolTable) {
        let symbols = vec![
            Symbol::new(SymbolKind::Function {
                name: "spill".to_string(),
                parameters: vec!["...any".to_string()],
                return_type: None,
                is_public: true,
            }),
            Symbol::new(SymbolKind::Function {
                name: "spillf".to_string(),
                parameters: vec!["string".to_string(), "...any".to_string()],
                return_type: None,
                is_public: true,
            }),
            Symbol::new(SymbolKind::Function {
                name: "spill_error".to_string(),
                parameters: vec!["...any".to_string()],
                return_type: None,
                is_public: true,
            }),
        ];

        for symbol in symbols {
            symbol_table.add_symbol(symbol);
        }
    }

    /// Add symbols for the stringz package
    fn add_stringz_symbols(&self, symbol_table: &mut PackageSymbolTable) {
        let symbols = vec![
            Symbol::new(SymbolKind::Function {
                name: "len".to_string(),
                parameters: vec!["string".to_string()],
                return_type: Some("int".to_string()),
                is_public: true,
            }),
            Symbol::new(SymbolKind::Function {
                name: "concat".to_string(),
                parameters: vec!["string".to_string(), "string".to_string()],
                return_type: Some("string".to_string()),
                is_public: true,
            }),
        ];

        for symbol in symbols {
            symbol_table.add_symbol(symbol);
        }
    }

    /// Add symbols for the mathz package
    fn add_mathz_symbols(&self, symbol_table: &mut PackageSymbolTable) {
        let symbols = vec![
            Symbol::new(SymbolKind::Function {
                name: "sqrt".to_string(),
                parameters: vec!["float".to_string()],
                return_type: Some("float".to_string()),
                is_public: true,
            }),
            Symbol::new(SymbolKind::Function {
                name: "pow".to_string(),
                parameters: vec!["float".to_string(), "float".to_string()],
                return_type: Some("float".to_string()),
                is_public: true,
            }),
        ];

        for symbol in symbols {
            symbol_table.add_symbol(symbol);
        }
    }

    /// Add symbols for the timez package
    fn add_timez_symbols(&self, symbol_table: &mut PackageSymbolTable) {
        let symbols = vec![
            Symbol::new(SymbolKind::Function {
                name: "now".to_string(),
                parameters: vec![],
                return_type: Some("DateTime".to_string()),
                is_public: true,
            }),
            Symbol::new(SymbolKind::Function {
                name: "format".to_string(),
                parameters: vec!["DateTime".to_string(), "string".to_string()],
                return_type: Some("string".to_string()),
                is_public: true,
            }),
        ];

        for symbol in symbols {
            symbol_table.add_symbol(symbol);
        }
    }

    /// Add symbols for other packages (simplified)
    fn add_dropz_symbols(&self, symbol_table: &mut PackageSymbolTable) {
        // File system operations
        let symbols = vec![
            Symbol::new(SymbolKind::Function {
                name: "read_file".to_string(),
                parameters: vec!["string".to_string()],
                return_type: Some("string".to_string()),
                is_public: true,
            }),
        ];

        for symbol in symbols {
            symbol_table.add_symbol(symbol);
        }
    }

    fn add_concurrenz_symbols(&self, _symbol_table: &mut PackageSymbolTable) {
        // Concurrency primitives - placeholder
    }

    fn add_web_vibez_symbols(&self, _symbol_table: &mut PackageSymbolTable) {
        // Web functionality - placeholder
    }

    fn add_cryptz_symbols(&self, _symbol_table: &mut PackageSymbolTable) {
        // Cryptographic functions - placeholder
    }

    fn add_rizztemplate_symbols(&self, _symbol_table: &mut PackageSymbolTable) {
        // Template processing - placeholder
    }

    fn add_quick_test_symbols(&self, _symbol_table: &mut PackageSymbolTable) {
        // Testing utilities - placeholder
    }
}
