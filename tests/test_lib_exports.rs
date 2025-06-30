/// Test to verify Phase 1B library exports work correctly
/// 
/// This test verifies that the critical re-exports added in Phase 1B
/// are accessible and can be imported correctly.

// Import everything from cursed to test our re-exports
extern crate cursed;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_exports_available() {
        // Test that we can access the key types through cursed::
        
        // Test Lexer export
        let source = "let x = 42;".to_string();
        let lexer = cursed::Lexer::new(source);
        // Just verify lexer creation works
        assert!(true);
        
        // Test TokenKind export  
        let _token_kind = cursed::TokenKind::Let;
        
        // Test Parser export
        let source2 = "let y = 24;".to_string();
        let lexer2 = cursed::Lexer::new(source2);
        let parser_result = cursed::Parser::new(lexer2);
        assert!(parser_result.is_ok());
    }
    
    #[test]
    fn test_optimization_exports_available() {
        // Test that optimization types are accessible
        let _config = cursed::OptimizationConfig::default();
        
        // Test that we can access performance metrics
        let metrics = cursed::PerformanceMetrics {
            execution_time: std::time::Duration::from_millis(50),
            memory_usage: 1024,
            code_size: 2048,
            optimization_level: "2".to_string(),
        };
        
        assert!(metrics.execution_time.as_millis() > 0);
    }
    
    #[test]
    fn test_package_manager_exports_available() {
        // Test that package manager types are accessible
        let config = cursed::PackageManagerConfig::default();
        assert!(!config.offline_mode); // Test a field that actually exists
    }
    
    #[test]
    fn test_llvm_codegen_export_available() {
        // Test that LlvmCodeGeneratorReal is accessible
        let codegen_result = cursed::LlvmCodeGeneratorReal::new();
        assert!(codegen_result.is_ok());
    }
}
