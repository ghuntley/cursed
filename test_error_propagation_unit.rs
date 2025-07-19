#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::llvm::error_propagation::ErrorPropagationCodegen;
    use crate::codegen::llvm::question_mark::QuestionMarkCodegen;
    use crate::ast::{Type, Expression, Literal};

    #[test]
    fn test_error_propagation_codegen_creation() {
        let mut codegen = ErrorPropagationCodegen::new();
        
        // Test that we can create the codegen instance
        assert!(codegen.error_variables.is_empty());
        assert!(codegen.propagation_context.is_empty());
        assert!(codegen.recovery_blocks.is_empty());
    }

    #[test]
    fn test_question_mark_codegen_creation() {
        let mut codegen = QuestionMarkCodegen::new();
        
        // Test that we can create the codegen instance
        assert!(codegen.type_context.is_empty());
        assert!(codegen.function_return_type.is_none());
    }

    #[test]
    fn test_result_type_identification() {
        let codegen = ErrorPropagationCodegen::new();
        
        // Test Result type identification
        let result_type = Type::Result(
            Box::new(Type::Integer),
            Box::new(Type::String)
        );
        assert!(codegen.is_result_type(&result_type));
        
        // Test non-Result type
        let int_type = Type::Integer;
        assert!(!codegen.is_result_type(&int_type));
    }

    #[test]
    fn test_type_size_calculation() {
        let codegen = ErrorPropagationCodegen::new();
        
        // Test basic type sizes
        assert_eq!(codegen.calculate_type_size(&Type::Integer).unwrap(), 4);
        assert_eq!(codegen.calculate_type_size(&Type::Boolean).unwrap(), 1);
        assert_eq!(codegen.calculate_type_size(&Type::Float).unwrap(), 8);
        assert_eq!(codegen.calculate_type_size(&Type::String).unwrap(), 8);
    }

    #[test]
    fn test_union_size_calculation() {
        let codegen = ErrorPropagationCodegen::new();
        
        // Test union size is max of both types
        let size = codegen.calculate_union_size(&Type::Integer, &Type::Float).unwrap();
        assert_eq!(size, 8); // max(4, 8) = 8
        
        let size2 = codegen.calculate_union_size(&Type::Boolean, &Type::String).unwrap();
        assert_eq!(size2, 8); // max(1, 8) = 8
    }

    #[test]
    fn test_result_type_extraction() {
        let codegen = ErrorPropagationCodegen::new();
        
        let result_type = Type::Result(
            Box::new(Type::Integer),
            Box::new(Type::String)
        );
        
        let (ok_type, err_type) = codegen.extract_result_types(&result_type).unwrap();
        assert!(matches!(ok_type, Type::Integer));
        assert!(matches!(err_type, Type::String));
    }

    #[test]
    fn test_llvm_type_conversion() {
        let codegen = ErrorPropagationCodegen::new();
        
        // Test basic type conversions
        assert_eq!(codegen.type_to_llvm(&Type::Integer).unwrap(), "i32");
        assert_eq!(codegen.type_to_llvm(&Type::String).unwrap(), "i8*");
        assert_eq!(codegen.type_to_llvm(&Type::Boolean).unwrap(), "i1");
        assert_eq!(codegen.type_to_llvm(&Type::Float).unwrap(), "double");
    }

    #[test]
    fn test_result_llvm_type_conversion() {
        let codegen = ErrorPropagationCodegen::new();
        
        let result_type = Type::Result(
            Box::new(Type::Integer),
            Box::new(Type::String)
        );
        
        let llvm_type = codegen.type_to_llvm(&result_type).unwrap();
        assert!(llvm_type.contains("{ i1, [8] }"));
    }

    #[test]
    fn test_function_return_type_setting() {
        let mut codegen = ErrorPropagationCodegen::new();
        assert!(codegen.function_return_type.is_none());
        
        let return_type = Type::Result(
            Box::new(Type::Integer),
            Box::new(Type::String)
        );
        
        codegen.set_function_return_type(return_type.clone());
        assert!(codegen.function_return_type.is_some());
        assert!(codegen.is_result_type(&codegen.function_return_type.unwrap()));
    }

    #[test]
    fn test_error_context_preservation() {
        let mut codegen = ErrorPropagationCodegen::new();
        
        let result = codegen.generate_error_context_preservation("%error_1", "function call");
        assert!(result.is_ok());
        
        let ir = result.unwrap();
        assert!(ir.contains("cursed_preserve_error_context"));
        assert!(ir.contains("function call"));
    }

    #[test]
    fn test_error_chain_generation() {
        let mut codegen = ErrorPropagationCodegen::new();
        
        let errors = vec!["%error_1".to_string(), "%error_2".to_string()];
        let result = codegen.generate_error_chain(&errors);
        assert!(result.is_ok());
        
        let ir = result.unwrap();
        assert!(ir.contains("cursed_create_error_chain"));
        assert!(ir.contains("cursed_add_to_error_chain"));
    }
}
