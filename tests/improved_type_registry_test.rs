#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use cursed::codegen::llvm::ImprovedTypeRegistry;
    
    
    // Helper to initialize tracing for tests
    fn init_tracing() {
        let _ = tracing_subscriber::fmt()
            .with_env_filter("debug")
            .try_init();
    }
    
    #[test]
    fn test_improved_type_registry() {
        init_tracing();
        
        // Create a new registry
        let mut registry = ImprovedTypeRegistry::new();
        
        // Register some types
        registry.register_type(1001, "Person".to_string());
        registry.register_type(1002, "Employee".to_string());
        registry.register_type(1003, "Manager".to_string());
        
        // Check that the registry has the expected types
        assert_eq!(registry.type_count(), 3);
        
        assert_eq!(registry.get_type_name(1001).map(|s| s.as_str()), Some("Person"));
        assert_eq!(registry.get_type_name(1002).map(|s| s.as_str()), Some("Employee"));
        assert_eq!(registry.get_type_name(1003).map(|s| s.as_str()), Some("Manager"));
        
        // Test type report generation
        let report = registry.generate_type_report();
        assert!(report.contains("Person"));
        assert!(report.contains("Employee"));
        assert!(report.contains("Manager"));
        assert!(report.contains("Total types registered: 3"));
        
        // Non-existent types should return None
        assert_eq!(registry.get_type_name(9999), None);
    }
    
    #[test]
    fn test_all_types() {
        init_tracing();
        
        // Create a new registry
        let mut registry = ImprovedTypeRegistry::new();
        
        // Register some types
        registry.register_type(1001, "Person".to_string());
        registry.register_type(1002, "Employee".to_string());
        
        // Get all types and check they match expected values
        let all_types = registry.all_types();
        assert_eq!(all_types.len(), 2);
        
        // Create a map for easier lookup
        let type_map: HashMap<_, _> = all_types.into_iter().collect();
        
        assert_eq!(type_map.get(&1001).map(|s| s.as_str()), Some("Person"));
        assert_eq!(type_map.get(&1002).map(|s| s.as_str()), Some("Employee"));
    }
    
    #[test]
    fn test_report_formatting() {
        init_tracing();
        
        // Create a new registry
        let mut registry = ImprovedTypeRegistry::new();
        
        // Add types with varying ID lengths to test alignment
        registry.register_type(1, "Short".to_string());
        registry.register_type(1000000, "Long".to_string());
        
        let report = registry.generate_type_report();
        
        // Verify report contains appropriate formatting
        assert!(report.contains("Type ID:"));
        assert!(report.contains("Name:"));
        assert!(report.contains("Short"));
        assert!(report.contains("Long"));
        
        // Verify report is properly formatted
        println!("{}", report); // For visual inspection in test output
    }
}