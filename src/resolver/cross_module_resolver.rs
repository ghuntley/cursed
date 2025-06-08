//! Package-aware symbol table for cross-module symbol resolution
//!
//! This module provides symbol tables that can handle symbols from multiple packages,
//! with proper visibility controls and qualified name resolution.

use std::collections::HashMap;
use crate::error::Error;
use super::{SymbolType, ResolvedSymbol};
use super::visibility::SymbolVisibility;

/// A symbol with package context
#[derive(Debug, Clone, PartialEq)]
pub struct PackageSymbol {
    pub name: String,
    pub symbol_type: SymbolType,
    pub package: String,
    pub visibility: SymbolVisibility,
    pub index: usize,
}

/// Symbol table for a single package
#[derive(Debug, Clone)]
pub struct PackageSymbolTable {
    /// Package name
    pub package_name: String,
    /// Symbols defined in this package
    pub symbols: HashMap<String, PackageSymbol>,
    /// Number of symbols defined
    pub num_definitions: usize,
}

impl PackageSymbolTable {
    /// Create a new package symbol table
    pub fn new(package_name: String) -> Self {
        Self {
            package_name,
            symbols: HashMap::new(),
            num_definitions: 0,
        }
    }

    /// Define a symbol in this package
    #[tracing::instrument(skip(self), fields(package = %self.package_name, symbol_name = %name), level = "debug")]
    pub fn define_symbol(
        &mut self,
        name: &str,
        symbol_type: SymbolType,
        visibility: SymbolVisibility,
    ) -> PackageSymbol {
        let symbol = PackageSymbol {
            name: name.to_string(),
            symbol_type,
            package: self.package_name.clone(),
            visibility,
            index: self.num_definitions,
        };

        self.symbols.insert(name.to_string(), symbol.clone());
        self.num_definitions += 1;

        symbol
    }

    /// Resolve a symbol in this package
    #[tracing::instrument(skip(self), fields(package = %self.package_name, symbol_name = %name), level = "debug")]
    pub fn resolve_symbol(&self, name: &str) -> Option<&PackageSymbol> {
        self.symbols.get(name)
    }

    /// Get all exported symbols
    pub fn get_exported_symbols(&self) -> Vec<String> {
        self.symbols
            .values()
            .filter(|symbol| symbol.visibility == SymbolVisibility::Public)
            .map(|symbol| symbol.name.clone())
            .collect()
    }

    /// Check if a symbol exists and is accessible
    pub fn is_symbol_accessible(&self, name: &str, from_package: &str) -> bool {
        if let Some(symbol) = self.symbols.get(name) {
            match symbol.visibility {
                SymbolVisibility::Public => true,
                SymbolVisibility::Private => from_package == &self.package_name,
            }
        } else {
            false
        }
    }
}

/// Global symbol table managing all packages
#[derive(Debug, Clone)]
pub struct GlobalSymbolTable {
    /// Package symbol tables
    pub packages: HashMap<String, PackageSymbolTable>,
    /// Quick lookup for public symbols: symbol_name -> (package_name, symbol)
    pub public_symbols: HashMap<String, Vec<(String, PackageSymbol)>>,
}

impl GlobalSymbolTable {
    /// Create a new global symbol table
    pub fn new() -> Self {
        Self {
            packages: HashMap::new(),
            public_symbols: HashMap::new(),
        }
    }

    /// Register a new package
    #[tracing::instrument(skip(self), fields(package_name = %package_name), level = "debug")]
    pub fn register_package(&mut self, package_name: &str) -> Result<(), Error> {
        if self.packages.contains_key(package_name) {
            return Err(Error::new(&format!("Package '{}' already registered", package_name)));
        }

        self.packages.insert(
            package_name.to_string(),
            PackageSymbolTable::new(package_name.to_string()),
        );

        Ok(())
    }

    /// Register a public symbol for global lookup
    #[tracing::instrument(skip(self), fields(package_name = %package_name, symbol_name = %symbol_name), level = "debug")]
    pub fn register_symbol(
        &mut self,
        package_name: &str,
        symbol_name: &str,
        symbol_type: SymbolType,
    ) -> Result<(), Error> {
        let package = self.packages.get_mut(package_name)
            .ok_or_else(|| Error::new(&format!("Package '{}' not found", package_name)))?;

        let symbol = package.define_symbol(symbol_name, symbol_type, SymbolVisibility::Public);

        // Add to public symbols lookup
        self.public_symbols
            .entry(symbol_name.to_string())
            .or_insert_with(Vec::new)
            .push((package_name.to_string(), symbol));

        Ok(())
    }

    /// Get a package symbol table
    pub fn get_package(&self, package_name: &str) -> Option<&PackageSymbolTable> {
        self.packages.get(package_name)
    }

    /// Get a mutable package symbol table
    pub fn get_package_mut(&mut self, package_name: &str) -> Option<&mut PackageSymbolTable> {
        self.packages.get_mut(package_name)
    }

    /// Resolve a qualified symbol (package.symbol)
    #[tracing::instrument(skip(self), fields(package_name = %package_name, symbol_name = %symbol_name), level = "debug")]
    pub fn resolve_qualified_symbol(
        &self,
        package_name: &str,
        symbol_name: &str,
    ) -> Result<ResolvedSymbol, Error> {
        let package = self.get_package(package_name)
            .ok_or_else(|| Error::new(&format!("Package '{}' not found", package_name)))?;

        let symbol = package.resolve_symbol(symbol_name)
            .ok_or_else(|| Error::new(&format!("Symbol '{}' not found in package '{}'", symbol_name, package_name)))?;

        Ok(ResolvedSymbol::new(
            symbol.name.clone(),
            symbol.symbol_type.clone(),
            symbol.package.clone(),
            symbol.visibility,
        ))
    }

    /// Find all packages that export a given symbol
    pub fn find_symbol_in_packages(&self, symbol_name: &str) -> Vec<&PackageSymbol> {
        if let Some(entries) = self.public_symbols.get(symbol_name) {
            entries.iter().map(|(_, symbol)| symbol).collect()
        } else {
            Vec::new()
        }
    }
}

impl Default for GlobalSymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Types of symbols in the language
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolType {
    Function,
    Variable,
    Constant,
    Type,
    Interface,
    Package,
}

/// Resolved symbol with metadata
#[derive(Debug, Clone)]
pub struct ResolvedSymbol {
    pub name: String,
    pub symbol_type: SymbolType,
    pub package: String,
    pub visibility: SymbolVisibility,
    pub qualified_name: String,
}

impl ResolvedSymbol {
    pub fn new(name: String, symbol_type: SymbolType, package: String, visibility: SymbolVisibility) -> Self {
        let qualified_name = format!("{}.{}", package, name);
        Self {
            name,
            symbol_type,
            package,
            visibility,
            qualified_name,
        }
    }
}
