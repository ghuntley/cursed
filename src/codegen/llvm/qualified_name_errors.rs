//! Error handling for qualified name compilation
//!
//! This module provides specialized error types and handling for qualified name
//! compilation, including detailed error messages for common issues like
//! undefined symbols, private access violations, and type mismatches.

use crate::error::Error;
use std::fmt;

/// Errors that can occur during qualified name compilation
#[derive(Debug, Clone, PartialEq)]
pub enum QualifiedNameError {
    /// Package not found or not imported
    PackageNotFound {
        package_name: String,
    },
    /// Symbol not found in package
    SymbolNotFound {
        package_name: String,
        symbol_name: String,
        available_symbols: Vec<String>,
    },
    /// Symbol is private and cannot be accessed
    PrivateAccess {
        package_name: String,
        symbol_name: String,
    },
    /// Symbol type mismatch (e.g., trying to use a type as a value)
    TypeMismatch {
        package_name: String,
        symbol_name: String,
        expected_kind: String,
        actual_kind: String,
    },
    /// Package alias not found
    AliasNotFound {
        alias: String,
    },
    /// Circular dependency in package imports
    CircularDependency {
        packages: Vec<String>,
    },
    /// Symbol ambiguity (multiple symbols with same name)
    Ambiguous {
        symbol_name: String,
        candidates: Vec<String>,
    },
}

impl fmt::Display for QualifiedNameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QualifiedNameError::PackageNotFound { package_name } => {
                write!(f, "Package '{}' not found. Make sure it's imported with 'yeet \"{}\"'", 
                    package_name, package_name)
            },
            QualifiedNameError::SymbolNotFound { package_name, symbol_name, available_symbols } => {
                write!(f, "Symbol '{}' not found in package '{}'", symbol_name, package_name)?;
                if !available_symbols.is_empty() {
                    write!(f, ". Available symbols: {}", available_symbols.join(", "))?;
                }
                Ok(())
            },
            QualifiedNameError::PrivateAccess { package_name, symbol_name } => {
                write!(f, "Symbol '{}' in package '{}' is private and cannot be accessed", 
                    symbol_name, package_name)
            },
            QualifiedNameError::TypeMismatch { package_name, symbol_name, expected_kind, actual_kind } => {
                write!(f, "Type mismatch for '{}.{}': expected {}, found {}", 
                    package_name, symbol_name, expected_kind, actual_kind)
            },
            QualifiedNameError::AliasNotFound { alias } => {
                write!(f, "Package alias '{}' not found", alias)
            },
            QualifiedNameError::CircularDependency { packages } => {
                write!(f, "Circular dependency detected: {}", packages.join(" -> "))
            },
            QualifiedNameError::Ambiguous { symbol_name, candidates } => {
                write!(f, "Ambiguous symbol '{}'. Could be: {}", symbol_name, candidates.join(", "))
            },
        }
    }
}

impl From<QualifiedNameError> for Error {
    fn from(err: QualifiedNameError) -> Self {
        Error::from_str(&err.to_string())
    }
}

/// Helper functions for creating qualified name errors
impl QualifiedNameError {
    /// Create a package not found error
    pub fn package_not_found(package_name: &str) -> Self {
        Self::PackageNotFound {
            package_name: package_name.to_string(),
        }
    }
    
    /// Create a symbol not found error
    pub fn symbol_not_found(package_name: &str, symbol_name: &str, available: Vec<String>) -> Self {
        Self::SymbolNotFound {
            package_name: package_name.to_string(),
            symbol_name: symbol_name.to_string(),
            available_symbols: available,
        }
    }
    
    /// Create a private access error
    pub fn private_access(package_name: &str, symbol_name: &str) -> Self {
        Self::PrivateAccess {
            package_name: package_name.to_string(),
            symbol_name: symbol_name.to_string(),
        }
    }
    
    /// Create a type mismatch error
    pub fn type_mismatch(package_name: &str, symbol_name: &str, expected: &str, actual: &str) -> Self {
        Self::TypeMismatch {
            package_name: package_name.to_string(),
            symbol_name: symbol_name.to_string(),
            expected_kind: expected.to_string(),
            actual_kind: actual.to_string(),
        }
    }
    
    /// Create an alias not found error
    pub fn alias_not_found(alias: &str) -> Self {
        Self::AliasNotFound {
            alias: alias.to_string(),
        }
    }
    
    /// Create a circular dependency error
    pub fn circular_dependency(packages: Vec<String>) -> Self {
        Self::CircularDependency { packages }
    }
    
    /// Create an ambiguous symbol error
    pub fn ambiguous(symbol_name: &str, candidates: Vec<String>) -> Self {
        Self::Ambiguous {
            symbol_name: symbol_name.to_string(),
            candidates,
        }
    }
}

/// Extension trait for converting standard errors to qualified name errors
pub trait QualifiedNameErrorExt {
    /// Convert to a qualified name error with context
    fn with_qualified_context(self, package: &str, symbol: &str) -> QualifiedNameError;
}

impl QualifiedNameErrorExt for Error {
    fn with_qualified_context(self, package: &str, symbol: &str) -> QualifiedNameError {
        QualifiedNameError::SymbolNotFound {
            package_name: package.to_string(),
            symbol_name: symbol.to_string(),
            available_symbols: vec![],
        }
    }
}
