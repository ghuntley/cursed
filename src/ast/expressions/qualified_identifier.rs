//! AST node for qualified identifiers (package.Symbol) in the CURSED language.
//!
//! This module defines the AST representation for qualified identifiers, which allow
//! accessing symbols from specific packages using dot notation (e.g., math.sqrt, json.Parse).

use crate::ast::{Expression, Node};
use std::any::Any;

/// Represents a qualified identifier node in the AST.
///
/// A qualified identifier consists of a package qualifier followed by a dot and a symbol name.
/// This is used to access exported symbols from other packages.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// math.sqrt(25)        // Accessing sqrt function from math package
/// json.Parse(data)     // Accessing Parse function from json package  
/// utils.Helper         // Accessing Helper type from utils package
/// ```
///
/// The AST would have a `QualifiedIdentifier` with:
/// - package: the package name or alias (e.g., `math`, `json`, `utils`)
/// - symbol: the symbol name within the package (e.g., `sqrt`, `Parse`, `Helper`)
#[derive(Clone)]
pub struct QualifiedIdentifier {
    /// The token that represents this qualified identifier
    pub token: String,
    /// The package name or alias
    pub package: String,
    /// The symbol name within the package
    pub symbol: String,
}

impl QualifiedIdentifier {
    /// Create a new qualified identifier
    pub fn new(token: String, package: String, symbol: String) -> Self {
        Self {
            token,
            package,
            symbol,
        }
    }

    /// Get the fully qualified name as a string
    pub fn qualified_name(&self) -> String {
        format!("{}.{}", self.package, self.symbol)
    }

    /// Check if this represents access to a specific package
    pub fn is_from_package(&self, package_name: &str) -> bool {
        self.package == package_name
    }

    /// Check if this accesses a specific symbol
    pub fn is_symbol(&self, symbol_name: &str) -> bool {
        self.symbol == symbol_name
    }
}

impl Node for QualifiedIdentifier {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        self.qualified_name()
    }
}

impl Expression for QualifiedIdentifier {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(QualifiedIdentifier {
            token: self.token.clone(),
            package: self.package.clone(),
            symbol: self.symbol.clone(),
        })
    }
}

impl std::fmt::Debug for QualifiedIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QualifiedIdentifier")
            .field("package", &self.package)
            .field("symbol", &self.symbol)
            .field("qualified_name", &self.qualified_name())
            .finish()
    }
}

impl PartialEq for QualifiedIdentifier {
    fn eq(&self, other: &Self) -> bool {
        self.package == other.package && self.symbol == other.symbol
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qualified_identifier_creation() {
        let qual_id = QualifiedIdentifier::new(
            "math.sqrt".to_string(),
            "math".to_string(),
            "sqrt".to_string(),
        );
        
        assert_eq!(qual_id.package, "math");
        assert_eq!(qual_id.symbol, "sqrt");
        assert_eq!(qual_id.qualified_name(), "math.sqrt");
        assert_eq!(qual_id.string(), "math.sqrt");
    }

    #[test]
    fn test_qualified_identifier_checks() {
        let qual_id = QualifiedIdentifier::new(
            "json.Parse".to_string(),
            "json".to_string(),
            "Parse".to_string(),
        );
        
        assert!(qual_id.is_from_package("json"));
        assert!(!qual_id.is_from_package("math"));
        
        assert!(qual_id.is_symbol("Parse"));
        assert!(!qual_id.is_symbol("Marshal"));
    }

    #[test]
    fn test_qualified_identifier_equality() {
        let qual_id1 = QualifiedIdentifier::new(
            "utils.Helper".to_string(),
            "utils".to_string(),
            "Helper".to_string(),
        );
        
        let qual_id2 = QualifiedIdentifier::new(
            "utils.Helper".to_string(),
            "utils".to_string(),
            "Helper".to_string(),
        );
        
        let qual_id3 = QualifiedIdentifier::new(
            "math.Helper".to_string(),
            "math".to_string(),
            "Helper".to_string(),
        );
        
        assert_eq!(qual_id1, qual_id2);
        assert_ne!(qual_id1, qual_id3);
    }
}
