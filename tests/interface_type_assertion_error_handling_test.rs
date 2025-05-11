#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use inkwell::context::Context;
    
    use cursed::codegen::llvm::LlvmCodeGenerator;
    use cursed::codegen::llvm::interface_type_assertion_error_handling::*;
    use cursed::error::Error;
    use cursed::ast::expressions::TypeAssertion;
    use cursed::ast::expressions::Identifier;
    use cursed::ast::traits::{Expression, Node};
    
    // Import common test utilities
    mod common;
    use common::test_utils::create_test_code_generator;
    use common::tracing::setup as setup_tracing;
    
    #[test]
    fn test_enhanced_error_handling_creation() {
        // Initialize tracing for this test
        setup_tracing();
        
        // Create a test code generator
        let mut code_generator = create_test_code_generator();
        
        // Register some test interfaces
        code_generator.interface_registry().register_interface("Animal").unwrap();
        code_generator.interface_registry().register_interface("Mammal").unwrap();
        code_generator.interface_registry().register_interface("Dog").unwrap();
        
        // Register the inheritance relationships
        code_generator.interface_registry().register_extension("Animal", "Mammal").unwrap();
        code_generator.interface_registry().register_extension("Mammal", "Dog").unwrap();
        
        // Test creating an error context
        let result = code_generator.create_assertion_context_error(
            "Cat", 
            "Dog", 
            "test.csd:42", 
            Some("Additional error info")
        );
        
        assert!(result.is_ok(), "Error creation failed: {:?}", result);
        
        let error = result.unwrap();
        
        // Verify error fields
        assert_eq!(error.source_type, "Cat");
        assert_eq!(error.target_type, "Dog");
        assert_eq!(error.source_location, "test.csd:42");
        assert!(error.message.contains("Additional error info"));
        
        // Verify error conversion to string works
        let error_str = error.to_string();
        assert!(error_str.contains("Cat is not a Dog"));
    }
    
    #[test]
    fn test_recovery_options_generation() {
        // Initialize tracing for this test
        setup_tracing();
        
        // Create a test code generator
        let mut code_generator = create_test_code_generator();
        
        // Register some interfaces with a hierarchy
        code_generator.interface_registry().register_interface("Animal").unwrap();
        code_generator.interface_registry().register_interface("Mammal").unwrap();
        code_generator.interface_registry().register_interface("Canine").unwrap();
        code_generator.interface_registry().register_interface("Dog").unwrap();
        code_generator.interface_registry().register_interface("Cat").unwrap();
        
        // Register the inheritance relationships
        code_generator.interface_registry().register_extension("Animal", "Mammal").unwrap();
        code_generator.interface_registry().register_extension("Mammal", "Canine").unwrap();
        code_generator.interface_registry().register_extension("Mammal", "Cat").unwrap();
        code_generator.interface_registry().register_extension("Canine", "Dog").unwrap();
        
        // Test getting recovery options for reversed relationship
        let reversed_result = code_generator.get_assertion_recovery_options("Dog", "Canine");
        assert!(reversed_result.is_ok(), "Recovery options generation failed: {:?}", reversed_result);
        
        let reversed_options = reversed_result.unwrap();
        assert!(reversed_options.is_some(), "Should have recovery options");
        assert!(reversed_options.unwrap().contains("reversed"), "Should mention reversed relationship");
        
        // Test getting recovery options for common interface
        let common_result = code_generator.get_assertion_recovery_options("Cat", "Dog");
        assert!(common_result.is_ok(), "Recovery options generation failed: {:?}", common_result);
        
        let common_options = common_result.unwrap();
        assert!(common_options.is_some(), "Should have recovery options");
        assert!(common_options.unwrap().contains("Mammal"), "Should suggest common interface Mammal");
    }
    
    #[test]
    fn test_find_common_interfaces() {
        // Initialize tracing for this test
        setup_tracing();
        
        // Create a test code generator
        let mut code_generator = create_test_code_generator();
        
        // Register a more complex hierarchy
        code_generator.interface_registry().register_interface("Animal").unwrap();
        code_generator.interface_registry().register_interface("Mammal").unwrap();
        code_generator.interface_registry().register_interface("Bird").unwrap();
        code_generator.interface_registry().register_interface("Cat").unwrap();
        code_generator.interface_registry().register_interface("Dog").unwrap();
        code_generator.interface_registry().register_interface("Duck").unwrap();
        code_generator.interface_registry().register_interface("Penguin").unwrap();
        
        // Register extensions
        code_generator.interface_registry().register_extension("Mammal", "Animal").unwrap();
        code_generator.interface_registry().register_extension("Bird", "Animal").unwrap();
        code_generator.interface_registry().register_extension("Cat", "Mammal").unwrap();
        code_generator.interface_registry().register_extension("Dog", "Mammal").unwrap();
        code_generator.interface_registry().register_extension("Duck", "Bird").unwrap();
        code_generator.interface_registry().register_extension("Penguin", "Bird").unwrap();
        
        // Find common interfaces between cat and dog
        let common_mammal = code_generator.find_common_interfaces("Cat", "Dog");
        assert!(common_mammal.is_ok(), "Failed to find common interfaces");
        
        let common_mammal_interfaces = common_mammal.unwrap();
        assert!(!common_mammal_interfaces.is_empty(), "Should find common interfaces");
        assert!(common_mammal_interfaces.contains(&"Mammal".to_string()), "Should contain Mammal");
        
        // Find common interfaces between cat and duck
        let common_animal = code_generator.find_common_interfaces("Cat", "Duck");
        assert!(common_animal.is_ok(), "Failed to find common interfaces");
        
        let common_animal_interfaces = common_animal.unwrap();
        assert!(!common_animal_interfaces.is_empty(), "Should find common interfaces");
        assert!(common_animal_interfaces.contains(&"Animal".to_string()), "Should contain Animal");
    }
    
    #[test]
    fn test_compile_type_assertion_with_enhanced_errors() {
        // Initialize tracing for this test
        setup_tracing();
        
        // Create a test code generator
        let mut code_generator = create_test_code_generator();
        
        // Register a type hierarchy
        code_generator.interface_registry().register_interface("Readable").unwrap();
        code_generator.interface_registry().register_interface("Document").unwrap();
        code_generator.interface_registry().register_interface("Book").unwrap();
        code_generator.interface_registry().register_interface("Article").unwrap();
        
        code_generator.interface_registry().register_extension("Document", "Readable").unwrap();
        code_generator.interface_registry().register_extension("Book", "Document").unwrap();
        code_generator.interface_registry().register_extension("Article", "Document").unwrap();
        
        // Create a simple type assertion expression
        let obj_expr = Box::new(Identifier {
            token: "Article".to_string(),
            value: "article".to_string(),
        }) as Box<dyn Expression>;
        
        let type_assertion = TypeAssertion {
            token: "test.csd:42".to_string(),
            expression: obj_expr,
            type_name: "Book".to_string(),
        };
        
        // Try compiling the type assertion - it should fail but with enhanced error
        let result = code_generator.compile_type_assertion_with_enhanced_errors(&type_assertion);
        
        // The compilation should fail because we're in a test environment without proper LLVM setup
        // but we can check that our error handler runs and returns a detailed error
        assert!(result.is_err(), "Should fail with detailed error");
        
        if let Err(Error::Compilation(msg)) = result {
            // The error message should be structured and include recovery options
            assert!(msg.contains("Article is not a Book"), "Error should mention type mismatch");
            assert!(msg.contains("Document"), "Error should mention common parent 'Document'");
        } else {
            panic!("Expected Compilation error with enhanced message");
        }
    }
}