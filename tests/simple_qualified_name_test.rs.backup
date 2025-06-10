//! Simple test for qualified name AST nodes
//!
//! This test validates just the AST components without complex integrations.

#[cfg(test)]
mod tests {
    use cursed::ast::{QualifiedName, QualifiedSymbolKind};
    use cursed::ast::Node;

    #[test]
    fn test_qualified_name_basic() {
        let qualified = QualifiedName::new(
            ".".to_string(),
            "math".to_string(),
            "sqrt".to_string()
        );
        
        assert_eq!(qualified.package, "math");
        assert_eq!(qualified.symbol, "sqrt");
        assert_eq!(qualified.symbol_kind, QualifiedSymbolKind::Unknown);
        assert_eq!(qualified.string(), "math.sqrt");
    }

    #[test]
    fn test_qualified_name_with_alias() {
        let qualified = QualifiedName::new_with_alias(
            ".".to_string(),
            "mathematics".to_string(),
            "sqrt".to_string(),
            "math".to_string()
        );
        
        assert_eq!(qualified.package, "mathematics");
        assert_eq!(qualified.symbol, "sqrt");
        assert_eq!(qualified.effective_package_name(), "math");
        assert_eq!(qualified.string(), "math.sqrt");
    }

    #[test]
    fn test_qualified_symbol_kinds() {
        let kinds = vec![
            QualifiedSymbolKind::Function,
            QualifiedSymbolKind::Type,
            QualifiedSymbolKind::Constant,
            QualifiedSymbolKind::Variable,
            QualifiedSymbolKind::Unknown,
        ];
        
        // Test that all kinds are different
        for (i, kind1) in kinds.iter().enumerate() {
            for (j, kind2) in kinds.iter().enumerate() {
                if i == j {
                    assert_eq!(kind1, kind2);
                } else {
                    assert_ne!(kind1, kind2);
                }
            }
        }
    }
}
