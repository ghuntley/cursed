//! Symbol visibility rules for the CURSED language
//!
//! This module implements the visibility system where symbols starting with uppercase
//! letters are public (exported) and symbols starting with lowercase letters are private.

/// Symbol visibility levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolVisibility {
    /// Public symbols (uppercase first letter) - accessible from other packages
    Public,
    /// Private symbols (lowercase first letter) - only accessible within same package
    Private,
}

/// Check if a symbol is exported (public) based on CURSED naming rules
///
/// In CURSED, symbols starting with uppercase letters are public/exported,
/// while symbols starting with lowercase letters are private.
///
/// # Examples
/// ```
/// assert_eq!(is_exported("PublicFunction"), true);
/// assert_eq!(is_exported("privateFunction"), false);
/// assert_eq!(is_exported("MyStruct"), true);
/// assert_eq!(is_exported("myVariable"), false);
/// ```
#[tracing::instrument(fields(symbol_name = %name), level = "trace")]
pub fn is_exported(name: &str) -> bool {
    name.chars()
        .next()
        .map(|c| c.is_uppercase())
        .unwrap_or(false)
}

/// Get the visibility of a symbol based on its name
pub fn get_symbol_visibility(name: &str) -> SymbolVisibility {
    if is_exported(name) {
        SymbolVisibility::Public
    } else {
        SymbolVisibility::Private
    }
}

/// Check if a symbol is accessible from a given package
#[tracing::instrument(fields(
    symbol_name = %symbol_name,
    symbol_package = %symbol_package,
    accessing_package = %accessing_package
), level = "debug")]
pub fn is_symbol_accessible(
    symbol_name: &str,
    symbol_package: &str,
    accessing_package: &str,
) -> bool {
    // Same package - all symbols accessible
    if symbol_package == accessing_package {
        return true;
    }

    // Different package - only public symbols accessible
    is_exported(symbol_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_exported() {
        // Public symbols (uppercase first)
        assert!(is_exported("PublicFunction"));
        assert!(is_exported("MyStruct"));
        assert!(is_exported("HTTP_STATUS"));
        assert!(is_exported("A"));

        // Private symbols (lowercase first)
        assert!(!is_exported("privateFunction"));
        assert!(!is_exported("myVariable"));
        assert!(!is_exported("internal"));
        assert!(!is_exported("a"));

        // Edge cases
        assert!(!is_exported(""));
        assert!(is_exported("_PublicWithUnderscore")); // Underscore + uppercase
        assert!(!is_exported("_privateWithUnderscore")); // Underscore + lowercase
    }

    #[test]
    fn test_symbol_visibility() {
        assert_eq!(get_symbol_visibility("PublicFunction"), SymbolVisibility::Public);
        assert_eq!(get_symbol_visibility("privateFunction"), SymbolVisibility::Private);
    }

    #[test]
    fn test_symbol_accessibility() {
        // Same package - all accessible
        assert!(is_symbol_accessible("privateFunc", "package1", "package1"));
        assert!(is_symbol_accessible("PublicFunc", "package1", "package1"));

        // Different packages - only public accessible
        assert!(!is_symbol_accessible("privateFunc", "package1", "package2"));
        assert!(is_symbol_accessible("PublicFunc", "package1", "package2"));
    }
}
