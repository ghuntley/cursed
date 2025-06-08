//! Error types for the package resolution system

use std::fmt;

/// Result type for resolver operations
pub type ResolverResult<T> = Result<T, ResolverError>;

/// Errors that can occur during package resolution
#[derive(Debug, Clone, PartialEq)]
pub enum ResolverError {
    /// Package not found
    PackageNotFound(String),
    /// Circular dependency detected
    CircularDependency(Vec<String>),
    /// Symbol not found in package
    SymbolNotFound {
        package: String,
        symbol: String,
    },
    /// General resolution error
    General(String),
}

impl fmt::Display for ResolverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResolverError::PackageNotFound(pkg) => {
                write!(f, "Package '{}' not found", pkg)
            },
            ResolverError::CircularDependency(packages) => {
                write!(f, "Circular dependency: {}", packages.join(" -> "))
            },
            ResolverError::SymbolNotFound { package, symbol } => {
                write!(f, "Symbol '{}' not found in package '{}'", symbol, package)
            },
            ResolverError::General(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for ResolverError {}

impl From<&str> for ResolverError {
    fn from(msg: &str) -> Self {
        ResolverError::General(msg.to_string())
    }
}

impl From<String> for ResolverError {
    fn from(msg: String) -> Self {
        ResolverError::General(msg)
    }
}

impl From<crate::error::Error> for ResolverError {
    fn from(err: crate::error::Error) -> Self {
        ResolverError::General(err.to_string())
    }
}

impl ResolverError {
    /// Create an invalid package path error
    pub fn invalid_package_path(path: &str, message: &str) -> Self {
        ResolverError::General(format!("Invalid package path '{}': {}", path, message))
    }

    /// Create a symbol not found error
    pub fn symbol_not_found(package: &str, symbol: &str) -> Self {
        ResolverError::SymbolNotFound {
            package: package.to_string(),
            symbol: symbol.to_string(),
        }
    }

    /// Create a package not found error
    pub fn package_not_found(package: &str, _searched_paths: Vec<String>) -> Self {
        ResolverError::PackageNotFound(package.to_string())
    }

    /// Create a compilation error
    pub fn compilation_error(path: &str, error: &str) -> Self {
        ResolverError::General(format!("Compilation error in '{}': {}", path, error))
    }

    /// Create an I/O error
    pub fn io_error(path: &str, error: &str) -> Self {
        ResolverError::General(format!("I/O error accessing '{}': {}", path, error))
    }

    /// Create a generic error
    pub fn generic(message: &str) -> Self {
        ResolverError::General(message.to_string())
    }
}
