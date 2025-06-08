//! AST node for qualified name expressions in the CURSED language.
//!
//! This module defines the AST representation for qualified names, which are used to
//! access symbols from imported packages. Qualified names support access to all symbol
//! types including functions, types, constants, and variables.
//!
//! In CURSED, qualified names appear in code like: `package.Symbol` where Symbol can be:
//! - Functions: `math.sqrt(25)`
//! - Types: `http.Request`  
//! - Constants: `math.Pi`
//! - Variables: `config.Debug`

use crate::ast::{Node, Expression};
use std::any::Any;

/// Symbol types that can be accessed through qualified names
#[derive(Debug, Clone, PartialEq)]
pub enum QualifiedSymbolKind {
    /// Function symbol (e.g., `math.sqrt`)
    Function,
    /// Type symbol (e.g., `http.Request`)
    Type,
    /// Constant symbol (e.g., `math.Pi`)
    Constant,
    /// Variable symbol (e.g., `config.Debug`)
    Variable,
    /// Unknown symbol type (resolved during compilation)
    Unknown,
}

/// Represents a qualified name expression (package.symbol) in the AST.
///
/// A qualified name consists of a package identifier followed by a dot and a symbol name.
/// It is used to access exported symbols from imported packages.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// math.sqrt(25)        // Function access
/// http.Request{}       // Type access for struct creation
/// math.Pi              // Constant access
/// config.Debug         // Variable access
/// ```
///
/// The AST would have a `QualifiedName` with:
/// - package: the package identifier (e.g., `math`, `http`, `config`)
/// - symbol: the symbol name after the dot (e.g., `sqrt`, `Request`, `Pi`, `Debug`)
/// - symbol_kind: the type of symbol being accessed
#[derive(Debug)]
pub struct QualifiedName {
    pub token: String,
    pub package: String,
    pub symbol: String,
    pub symbol_kind: QualifiedSymbolKind,
    /// Optional alias if the package was imported with an alias
    pub package_alias: Option<String>,
}

impl Node for QualifiedName {
    fn token_literal(&self) -> String {
        self.token.clone()
    }
    
    fn string(&self) -> String {
        if let Some(alias) = &self.package_alias {
            format!("{}.{}", alias, self.symbol)
        } else {
            format!("{}.{}", self.package, self.symbol)
        }
    }
}

impl Expression for QualifiedName {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(QualifiedName {
            token: self.token.clone(),
            package: self.package.clone(),
            symbol: self.symbol.clone(),
            symbol_kind: self.symbol_kind.clone(),
            package_alias: self.package_alias.clone(),
        })
    }
}

impl QualifiedName {
    /// Create a new qualified name expression
    pub fn new(token: String, package: String, symbol: String) -> Self {
        Self {
            token,
            package,
            symbol,
            symbol_kind: QualifiedSymbolKind::Unknown,
            package_alias: None,
        }
    }
    
    /// Create a qualified name with a specific symbol kind
    pub fn new_with_kind(token: String, package: String, symbol: String, kind: QualifiedSymbolKind) -> Self {
        Self {
            token,
            package,
            symbol,
            symbol_kind: kind,
            package_alias: None,
        }
    }
    
    /// Create a qualified name with a package alias
    pub fn new_with_alias(token: String, package: String, symbol: String, alias: String) -> Self {
        Self {
            token,
            package,
            symbol,
            symbol_kind: QualifiedSymbolKind::Unknown,
            package_alias: Some(alias),
        }
    }
    
    /// Get the effective package name (alias if present, otherwise package name)
    pub fn effective_package_name(&self) -> &str {
        self.package_alias.as_ref().unwrap_or(&self.package)
    }
    
    /// Set the symbol kind after resolution
    pub fn set_symbol_kind(&mut self, kind: QualifiedSymbolKind) {
        self.symbol_kind = kind;
    }
}
