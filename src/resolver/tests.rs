//! Tests for the symbol resolution system

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;

    #[test]
    fn test_visibility_rules() {
        // Test uppercase = public
        assert!(is_exported("PublicFunction"));
        assert!(is_exported("ExportedVariable"));
        assert!(is_exported("MyType"));
        
        // Test lowercase = private
        assert!(!is_exported("privateFunction"));
        assert!(!is_exported("localVariable"));
        assert!(!is_exported("myType"));
        
        // Test edge cases
        assert!(!is_exported(""));
        assert!(!is_exported("_underscore"));
        assert!(is_exported("_UppercaseUnderscore"));
    }

    #[test]
    fn test_basic_symbol_resolution() {
        let mut resolver = Resolver::new();
        
        // Register a package
        resolver.register_package("test_package").unwrap();
        
        // Define a public function
        resolver.define_symbol(
            "test_package",
            "PublicFunction",
            SymbolType::Function,
        ).unwrap();
        
        // Define a private function
        resolver.define_symbol(
            "test_package",
            "privateFunction",
            SymbolType::Function,
        ).unwrap();
        
        // Test resolution within same package
        let resolved = resolver.resolve_identifier("test_package", "PublicFunction").unwrap();
        assert_eq!(resolved.name, "PublicFunction");
        assert_eq!(resolved.symbol_type, SymbolType::Function);
        assert_eq!(resolved.visibility, SymbolVisibility::Public);
        
        let resolved = resolver.resolve_identifier("test_package", "privateFunction").unwrap();
        assert_eq!(resolved.name, "privateFunction");
        assert_eq!(resolved.symbol_type, SymbolType::Function);
        assert_eq!(resolved.visibility, SymbolVisibility::Private);
    }

    #[test] 
    fn test_qualified_name_resolution() {
        let mut resolver = Resolver::new();
        
        // Set up packages
        resolver.register_package("main").unwrap();
        resolver.register_package("utils").unwrap();
        
        // Define symbols in utils package
        resolver.define_symbol("utils", "Helper", SymbolType::Function).unwrap();
        resolver.define_symbol("utils", "privateHelper", SymbolType::Function).unwrap();
        
        // Register import in main package
        resolver.register_import("main", "utils", None).unwrap();
        
        // Test qualified resolution
        let resolved = resolver.resolve_qualified("main", "utils.Helper").unwrap();
        assert_eq!(resolved.name, "Helper");
        assert_eq!(resolved.package, "utils");
        assert_eq!(resolved.symbol_type, SymbolType::Function);
        
        // Private symbols should not be accessible
        let result = resolver.resolve_qualified("main", "utils.privateHelper");
        assert!(result.is_err()); // Should fail due to visibility
    }

    #[test]
    fn test_import_alias_resolution() {
        let mut resolver = Resolver::new();
        
        // Set up packages
        resolver.register_package("main").unwrap();
        resolver.register_package("utility_package").unwrap();
        
        // Define symbol in utility package
        resolver.define_symbol("utility_package", "DoSomething", SymbolType::Function).unwrap();
        
        // Register import with alias
        resolver.register_import("main", "utility_package", Some("utils")).unwrap();
        
        // Test qualified resolution with alias
        let resolved = resolver.resolve_qualified("main", "utils.DoSomething").unwrap();
        assert_eq!(resolved.name, "DoSomething");
        assert_eq!(resolved.package, "utility_package");
    }

    #[test]
    fn test_symbol_accessibility() {
        let mut resolver = Resolver::new();
        
        resolver.register_package("package1").unwrap();
        resolver.register_package("package2").unwrap();
        
        resolver.define_symbol("package1", "PublicFunc", SymbolType::Function).unwrap();
        resolver.define_symbol("package1", "privateFunc", SymbolType::Function).unwrap();
        
        // Public symbols should be accessible from other packages if imported
        resolver.register_import("package2", "package1", None).unwrap();
        assert!(resolver.is_symbol_accessible("package2", "package1", "PublicFunc"));
        assert!(!resolver.is_symbol_accessible("package2", "package1", "privateFunc"));
        
        // All symbols accessible within same package
        assert!(resolver.is_symbol_accessible("package1", "package1", "PublicFunc"));
        assert!(resolver.is_symbol_accessible("package1", "package1", "privateFunc"));
    }

    #[test]
    fn test_error_cases() {
        let mut resolver = Resolver::new();
        
        // Test resolving from non-existent package
        let result = resolver.resolve_identifier("nonexistent", "symbol");
        assert!(result.is_err());
        
        // Test resolving non-existent symbol
        resolver.register_package("test").unwrap();
        let result = resolver.resolve_identifier("test", "NonExistent");
        assert!(result.is_err());
        
        // Test duplicate package registration
        let result = resolver.register_package("test");
        assert!(result.is_err());
    }
}
