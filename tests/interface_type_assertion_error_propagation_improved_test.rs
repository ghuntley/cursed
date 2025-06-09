use std::sync::Arc;
use std::cell::RefCell;
use cursed::ast::expressions::TypeAssertion;
use cursed::ast::traits::{Expression, Node};
use cursed::codegen::llvm::interface_type_assertion_error_propagation_improved::*;
use cursed::codegen::llvm::interface_type_assertion_error_propagation_improved::ImprovedTypeAssertionErrorPropagation;
use cursed::codegen::llvm::interface_type_assertion::ImprovedTypeAssertion;
use cursed::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;
use cursed::codegen::llvm::type_assertion::InterfaceTypeAssertion;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use inkwell::context::Context;
use tracing::{debug, info, warn};

// Integration tests for the improved error propagation in interface type assertions



// Import common test utilities
#[path = "common/mod.rs"]
mod common;


#[test]
fn test_improved_error_propagation_registration() {
    // Initialize tracing for this test
    common::tracing::setup();
    
    // Call the registration function to ensure it exists and doesn't panic
    register_interface_type_assertion_error_propagation();
}

#[test]
fn test_generate_type_assertion_error() {
    // Initialize tracing for this test
    common::tracing::setup();
    
    // Create a simple test context
    let context = Context::create();
    let module = context.create_module("test");
    
    // Create a mock code generator
    struct MockGenerator<'ctx> {
        context: &'ctx Context,
        module: inkwell::module::Module<'ctx>,
    }
    
    impl<'ctx> MockGenerator<'ctx> {
        fn new(context: &'ctx Context, module: inkwell::module::Module<'ctx>) -> Self {
            Self { context, module }
        }
    }
    
    // Implement the necessary traits for testing
    // This is a simplified mock that doesn't need full functionality
    impl<'ctx> ImprovedTypeAssertion<'ctx> for MockGenerator<'ctx> {
        fn get_runtime_type_info(&mut self, _type_name: &str) -> Result<inkwell::values::BasicValueEnum<'ctx>, Error> {
            unimplemented!()
        }
        
        fn register_type_with_runtime(&mut self, _type_name: &str) -> Result<(), Error> {
            Ok(())
        }
        
        fn log_type_assertion(&mut self, _source_type: &str, _target_type: &str, _success: bool) -> Result<(), Error> {
            Ok(())
        }
    }
    
    impl<'ctx> InterfaceTypeAssertion<'ctx> for MockGenerator<'ctx> {
        fn context(&self) -> &'ctx Context {
            self.context
        }
        
        fn module(&self) -> &inkwell::module::Module<'ctx> {
            &self.module
        }
        
        fn builder(&self) -> &inkwell::builder::Builder<'ctx> {
            unimplemented!()
        }
        
        fn current_function(&self) -> Option<inkwell::values::FunctionValue<'ctx>> {
            None
        }
        
        fn check_instance_of(
            &mut self,
            _value: inkwell::values::BasicValueEnum<'ctx>,
            _type_name: &str
        ) -> Result<inkwell::values::BasicValueEnum<'ctx>, Error> {
            unimplemented!()
        }
        
        fn extract_interface_data_ptr(
            &mut self,
            _value: inkwell::values::BasicValueEnum<'ctx>
        ) -> Result<inkwell::values::PointerValue<'ctx>, Error> {
            unimplemented!()
        }
        
        fn extract_interface_type_id(
            &mut self,
            _value: inkwell::values::BasicValueEnum<'ctx>
        ) -> Result<inkwell::values::IntValue<'ctx>, Error> {
            unimplemented!()
        }
        
        fn get_runtime_type_id(
            &mut self,
            _value: inkwell::values::BasicValueEnum<'ctx>
        ) -> Result<u64, Error> {
            Ok(12345) // Mock implementation returns a fixed value
        }
        
        fn get_type_id(
            &mut self,
            _type_name: &str
        ) -> Result<u64, Error> {
            Ok(67890) // Different fixed value for testing
        }
        
        fn get_type_name_for_id(
            &mut self,
            _type_id: u64
        ) -> Result<String, Error> {
            Ok("MockSourceType".to_string())
        }
        
        fn cast_to_interface_type(
            &mut self,
            _value: inkwell::values::BasicValueEnum<'ctx>,
            _type_name: &str
        ) -> Result<inkwell::values::BasicValueEnum<'ctx>, Error> {
            unimplemented!()
        }
    }
    
    impl<'ctx> InterfaceTypeAssertionPathVisualization<'ctx> for MockGenerator<'ctx> {
        fn visualize_interface_hierarchy(
            &mut self,
            _type_name: &str,
            _depth: usize
        ) -> Result<String, Error> {
            Ok("Mock interface hierarchy visualization".to_string())
        }
        
        fn check_extension_relationship_enhanced(
            &mut self,
            _source_type: &str,
            _target_type: &str
        ) -> Result<bool, Error> {
            Ok(false) // Mock always returns false for simplicity
        }
        
        fn detect_reversed_inheritance_enhanced(
            &mut self,
            _source_type: &str,
            _target_type: &str
        ) -> Result<(bool, String), Error> {
            Ok((false, "".to_string()))
        }
        
        fn find_alternative_paths_enhanced(
            &mut self,
            _source_type: &str,
            _target_type: &str,
            _max_depth: usize
        ) -> Result<Vec<Rc<dyn crate::codegen::llvm::interface_type_assertion_path_visualization::InheritancePath>>, Error> {
            Ok(vec![])
        }
        
        fn generate_interface_hierarchy_dot_graph(&mut self) -> Result<String, Error> {
            Ok("digraph G { A -> B; }".to_string())
        }
    }
    
    // Create our mock generator
    let mut mock_generator = MockGenerator::new(&context, module);
    
    // Test generating an error
    let error = mock_generator.generate_type_assertion_error(
        "SourceType",
        "TargetType",
        "source.csd:42:10",
        Some("Additional error context".to_string()
    ).expect("Failed to generate error");
    
    // Verify the error has the expected fields
    assert_eq!(error.source_type, "SourceType");
    assert_eq!(error.target_type, "TargetType");
    assert_eq!(error.source_location, "source.csd:42:10");
    assert!(error.message.contains("Additional error context");
    assert!(error.message.contains("No inheritance path exists");
    assert!(error.message.contains("Mock interface hierarchy visualization");
    
    // Test recovery suggestion
    if let Some(hint) = error.recovery_hint {
        assert!(hint.contains("implement"));
    } else {
        panic!("Expected a recovery hint");
    }
    
    // Test string representation
    let error_string = error.to_string();
    assert!(error_string.contains("Type assertion error"));
    assert!(error_string.contains("SourceType is not a TargetType"));
    assert!(error_string.contains("Recovery hint"));
}

#[test]
fn test_suggest_recovery_options() {
    // Initialize tracing for this test
    common::tracing::setup();
    
    // Create a simple test context
    let context = Context::create();
    let module = context.create_module("test");
    
    // Simplified mock that has custom behavior for relationship checking
    struct RecoveryTestGenerator<'ctx> {
        context: &'ctx Context,
        module: inkwell::module::Module<'ctx>,
        // Toggle to test different scenarios
        reversed_relationship: bool,
    }
    
    impl<'ctx> RecoveryTestGenerator<'ctx> {
        fn new(context: &'ctx Context, module: inkwell::module::Module<'ctx>, reversed: bool) -> Self {
            Self { 
                context, 
                module,
                reversed_relationship: reversed,
            }
        }
    }
    
    // Stub implementations needed for the test
    impl<'ctx> ImprovedTypeAssertion<'ctx> for RecoveryTestGenerator<'ctx> {
        fn get_runtime_type_info(&mut self, _type_name: &str) -> Result<inkwell::values::BasicValueEnum<'ctx>, Error> {
            unimplemented!()
        }
        
        fn register_type_with_runtime(&mut self, _type_name: &str) -> Result<(), Error> {
            Ok(())
        }
        
        fn log_type_assertion(&mut self, _source_type: &str, _target_type: &str, _success: bool) -> Result<(), Error> {
            Ok(())
        }
    }
    
    impl<'ctx> InterfaceTypeAssertion<'ctx> for RecoveryTestGenerator<'ctx> {
        fn context(&self) -> &'ctx Context {
            self.context
        }
        
        fn module(&self) -> &inkwell::module::Module<'ctx> {
            &self.module
        }
        
        fn builder(&self) -> &inkwell::builder::Builder<'ctx> {
            unimplemented!()
        }
        
        fn current_function(&self) -> Option<inkwell::values::FunctionValue<'ctx>> {
            None
        }
        
        fn check_instance_of(
            &mut self,
            _value: inkwell::values::BasicValueEnum<'ctx>,
            _type_name: &str
        ) -> Result<inkwell::values::BasicValueEnum<'ctx>, Error> {
            unimplemented!()
        }
        
        fn extract_interface_data_ptr(
            &mut self,
            _value: inkwell::values::BasicValueEnum<'ctx>
        ) -> Result<inkwell::values::PointerValue<'ctx>, Error> {
            unimplemented!()
        }
        
        fn extract_interface_type_id(
            &mut self,
            _value: inkwell::values::BasicValueEnum<'ctx>
        ) -> Result<inkwell::values::IntValue<'ctx>, Error> {
            unimplemented!()
        }
        
        fn get_runtime_type_id(&mut self, _value: inkwell::values::BasicValueEnum<'ctx>) -> Result<u64, Error> {
            Ok(12345)
        }
        
        fn get_type_id(&mut self, _type_name: &str) -> Result<u64, Error> {
            Ok(67890)
        }
        
        fn get_type_name_for_id(&mut self, _type_id: u64) -> Result<String, Error> {
            Ok("MockType".to_string())
        }
        
        fn cast_to_interface_type(
            &mut self,
            _value: inkwell::values::BasicValueEnum<'ctx>,
            _type_name: &str
        ) -> Result<inkwell::values::BasicValueEnum<'ctx>, Error> {
            unimplemented!()
        }
    }
    
    impl<'ctx> InterfaceTypeAssertionPathVisualization<'ctx> for RecoveryTestGenerator<'ctx> {
        fn visualize_interface_hierarchy(&mut self, _type_name: &str, _depth: usize) -> Result<String, Error> {
            Ok("Mock interface hierarchy visualization".to_string())
        }
        
        fn check_extension_relationship_enhanced(&mut self, source_type: &str, target_type: &str) -> Result<bool, Error> {
            // Custom logic for testing different scenarios
            if source_type == "ChildType" && target_type == "ParentType" {
                Ok(true) // ChildType extends ParentType
            } else if source_type == "ParentType" && target_type == "ChildType" {
                // If testing reversed relationship, return true
                Ok(self.reversed_relationship)
            } else {
                Ok(false)
            }
        }
        
        fn detect_reversed_inheritance_enhanced(&mut self, _source_type: &str, _target_type: &str) -> Result<(bool, String), Error> {
            Ok((self.reversed_relationship, "Reversed relationship detected".to_string()))
        }
        
        fn find_alternative_paths_enhanced(
            &mut self,
            _source_type: &str,
            _target_type: &str,
            _max_depth: usize
        ) -> Result<Vec<Rc<dyn crate::codegen::llvm::interface_type_assertion_path_visualization::InheritancePath>>, Error> {
            Ok(vec![])
        }
        
        fn generate_interface_hierarchy_dot_graph(&mut self) -> Result<String, Error> {
            Ok("digraph G { A -> B; }".to_string())
        }
    }
    
    // Test with regular relationship
    {
        let mut generator = RecoveryTestGenerator::new(&context, module.clone(), false);
        
        let hint = generator.suggest_recovery_options("TypeA", "TypeB")
            .expect("Failed to get recovery options")
            .expect("Should have recovery hint");
        
        assert!(hint.contains("implement"));
        assert!(hint.contains("'TypeB' for the type 'TypeA'"));
    }
    
    // Test with reversed relationship
    {
        let mut generator = RecoveryTestGenerator::new(&context, module.clone(), true);
        
        let hint = generator.suggest_recovery_options("TypeA", "TypeB")
            .expect("Failed to get recovery options")
            .expect("Should have recovery hint");
        
        assert!(hint.contains("reversed"));
        assert!(hint.contains("Try asserting 'TypeB' as 'TypeA'"));
    }
}

// More extensive test using actual interface and type hierarchies would be added here
// These would test the full compilation and error propagation logic with realistic scenarios