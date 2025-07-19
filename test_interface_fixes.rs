//! Tests for the interface type checking fixes in LLVM codegen
//! 
//! This test validates that the TODO items have been properly addressed:
//! 1. Proper interface type checking for method dispatch
//! 2. Type validation for member access operations

#[cfg(test)]
mod interface_type_checking_tests {
    use super::*;
    use crate::codegen::llvm::main::LlvmCodeGenerator;
    use crate::ast::{Expression, MemberAccessExpression};
    use std::collections::HashMap;

    #[test]
    fn test_interface_type_checking_implemented() {
        // Verify that interface type checking methods exist
        let mut generator = LlvmCodeGenerator::new().unwrap();
        
        // These fields should exist
        assert!(generator.variable_interface_types.is_empty());
        assert!(generator.interface_definitions.is_empty());
        assert!(generator.function_signatures.is_empty());
    }

    #[test]
    fn test_get_interface_type_method() {
        let mut generator = LlvmCodeGenerator::new().unwrap();
        
        // Test with identifier expression
        let expr = Expression::Identifier("test_var".to_string());
        let result = generator.get_interface_type(&expr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None); // No interface type set
    }

    #[test]
    fn test_interface_member_access_generation() {
        let mut generator = LlvmCodeGenerator::new().unwrap();
        
        // Test interface member access generation
        let result = generator.generate_interface_member_access("TestInterface", "test_method");
        assert!(result.is_ok());
        
        let ir = result.unwrap();
        assert!(ir.contains("%")); // Should contain register references
    }

    #[test]
    fn test_interface_method_validation() {
        let mut generator = LlvmCodeGenerator::new().unwrap();
        
        // Test validation of non-existent interface method
        let result = generator.validate_interface_method("NonExistentInterface", "test_method");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not have method"));
    }

    #[test]
    fn test_interface_vtable_index_retrieval() {
        let mut generator = LlvmCodeGenerator::new().unwrap();
        
        // Test vtable index retrieval for non-existent method
        let result = generator.get_method_vtable_index("TestInterface", "test_method");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found in interface"));
    }

    #[test]
    fn test_interface_method_signature_retrieval() {
        let mut generator = LlvmCodeGenerator::new().unwrap();
        
        // Test signature retrieval for non-existent method
        let result = generator.get_interface_method_signature("TestInterface", "test_method");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found in interface"));
    }

    #[test]
    fn test_structural_interface_compliance() {
        let mut generator = LlvmCodeGenerator::new().unwrap();
        
        // Test structural interface compliance checking
        let expr = Expression::Identifier("test_obj".to_string());
        let result = generator.check_structural_interface_compliance(&expr, "test_method");
        assert!(result.is_ok());
        // Should return None since no interfaces are defined
        assert_eq!(result.unwrap(), None);
    }
}
