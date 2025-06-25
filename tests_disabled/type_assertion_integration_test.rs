use cursed::runtime::type_assertion_runtime::{TypeAssertionRuntime, RuntimeTypeInfo};
use cursed::codegen::llvm::type_assertion::InterfaceTypeAssertion;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::CursedError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_assertion_runtime_creation() -> Result<(), CursedError> {
        let runtime = TypeAssertionRuntime::new();
        
        // Test runtime initialization
        assert!(runtime.get_statistics().total_assertions == 0);
        Ok(())
    }

    #[test]
    fn test_runtime_type_registration() -> Result<(), CursedError> {
        let mut runtime = TypeAssertionRuntime::new();
        
        // Register a type
        let type_info = RuntimeTypeInfo {
            type_id: 123,
            type_name: "TestType".to_string(),
            interface_implementations: vec![],
        };
        
        runtime.register_type(type_info)?;
        
        // Verify type is registered
        let retrieved = runtime.lookup_type(123);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().type_name, "TestType");
        
        Ok(())
    }

    #[test]
    fn test_type_assertion_basic() -> Result<(), CursedError> {
        let mut runtime = TypeAssertionRuntime::new();
        
        // Register test types
        let type_a = RuntimeTypeInfo {
            type_id: 1,
            type_name: "TypeA".to_string(),
            interface_implementations: vec![],
        };
        let type_b = RuntimeTypeInfo {
            type_id: 2, 
            type_name: "TypeB".to_string(),
            interface_implementations: vec![],
        };
        
        runtime.register_type(type_a)?;
        runtime.register_type(type_b)?;
        
        // Test type assertions
        let result_a = runtime.perform_assertion(1, 1);
        assert!(result_a.success);
        
        let result_b = runtime.perform_assertion(1, 2);
        assert!(!result_b.success);
        
        Ok(())
    }

    #[test]
    fn test_type_assertion_with_llvm() -> Result<(), CursedError> {
        let mut codegen = LlvmCodeGenerator::new()?;
        
        // Test LLVM integration for type assertions
        let type_id = codegen.generate_type_hash("TestInterface")?;
        assert!(type_id > 0);
        
        // Test tuple building for type assertion results
        let tuple_type = codegen.build_tuple_type(&[
            codegen.get_i64_type(),
            codegen.get_i1_type()
        ])?;
        assert!(tuple_type.is_some());
        
        Ok(())
    }

    #[test]
    fn test_interface_registry_integration() -> Result<(), CursedError> {
        let mut runtime = TypeAssertionRuntime::new();
        
        // Test interface implementation tracking
        let interface_impl = RuntimeTypeInfo {
            type_id: 42,
            type_name: "Drawable".to_string(),
            interface_implementations: vec!["Display".to_string(), "Debug".to_string()],
        };
        
        runtime.register_type(interface_impl)?;
        
        // Verify interface implementations are tracked
        let type_info = runtime.lookup_type(42).unwrap();
        assert_eq!(type_info.interface_implementations.len(), 2);
        assert!(type_info.interface_implementations.contains(&"Display".to_string()));
        
        Ok(())
    }

    #[test]
    fn test_assertion_statistics() -> Result<(), CursedError> {
        let mut runtime = TypeAssertionRuntime::new();
        
        // Perform some assertions to test statistics
        let initial_stats = runtime.get_statistics();
        assert_eq!(initial_stats.total_assertions, 0);
        
        // Simulate successful assertion
        let _result = runtime.perform_assertion(1, 1);
        let after_stats = runtime.get_statistics();
        assert_eq!(after_stats.total_assertions, 1);
        assert_eq!(after_stats.successful_assertions, 1);
        
        // Simulate failed assertion
        let _result = runtime.perform_assertion(1, 2);
        let final_stats = runtime.get_statistics();
        assert_eq!(final_stats.total_assertions, 2);
        assert_eq!(final_stats.failed_assertions, 1);
        
        Ok(())
    }
}
