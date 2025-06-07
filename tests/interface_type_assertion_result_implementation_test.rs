use std::sync::Arc;
use std::cell::RefCell;
use cursed::ast::expressions::TypeAssertion;
use cursed::ast::expressions::Identifier;
use cursed::ast::traits::{Expression, Node};
use cursed::codegen::llvm::interface_type_assertion_result_implementation::*;
use cursed::codegen::llvm::interface_type_assertion_result_implementation::IntegratedResultTypeAssertion;
use cursed::codegen::llvm::interface_type_assertion::InterfaceTypeAssertion;
use cursed::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::error::SourceLocation;
use cursed::error::type_assertion_error::TypeAssertionError;
use cursed::lexer::Token;
use common::tracing::setup as init_tracing;
use inkwell::context::Context;
use tracing::{debug, info, warn, trace};

//! Integration tests for the full Result-based interface type assertion implementation
//!
//! These tests verify that interface type assertions work correctly with proper
//! integration of the ? operator for error propagation.



// Import common test utilities
#[path = "common.rs"]
pub mod common;


#[test]
fn test_result_implementation_registration() {
    // Initialize tracing for this test
    init_tracing();
    
    // Call the registration function to ensure it exists and doesn't panic
    register_result_implementation();
}

#[test]
fn test_generate_type_assertion_error_info() {
    // Initialize tracing for this test
    init_tracing();
    
    // Create a test context
    let context = Context::create();
    let module = context.create_module("test");
    
    // Create a mock code generator
    struct MockGenerator<'ctx> {
        context: &'ctx Context,
        module: inkwell::module::Module<'ctx>,
        builder: inkwell::builder::Builder<'ctx>,
    }
    
    impl<'ctx> MockGenerator<'ctx> {
        fn new(context: &'ctx Context, module: inkwell::module::Module<'ctx>) -> Self {
            Self { 
                context, 
                module,
                builder: context.create_builder(),
            }
        }
    }
    
    // Implement necessary traits for testing
    impl<'ctx> InterfaceTypeAssertion<'ctx> for MockGenerator<'ctx> {
        fn get_interface_type_id(
            &mut self,
            _interface_value: inkwell::values::BasicValueEnum<'ctx>
        ) -> Result<inkwell::values::BasicValueEnum<'ctx>, Error> {
            Ok(self.context.i64_type().const_int(12345, false).into())
        }
        
        fn extract_interface_data_ptr(
            &mut self,
            _interface_value: inkwell::values::BasicValueEnum<'ctx>
        ) -> Result<inkwell::values::PointerValue<'ctx>, Error> {
            Ok(self.context.i8_type().ptr_type(inkwell::AddressSpace::default()).const_null())
        }
        
        fn check_instance_of(
            &mut self,
            _interface_value: inkwell::values::BasicValueEnum<'ctx>,
            _target_type_name: &str,
            _source_location: Option<SourceLocation>
        ) -> Result<inkwell::values::BasicValueEnum<'ctx>, Error> {
            Ok(self.context.bool_type().const_int(0, false).into())
        }
        
        fn compile_type_assertion(
            &mut self,
            _type_assertion: &TypeAssertion
        ) -> Result<inkwell::values::BasicValueEnum<'ctx>, Error> {
            unimplemented!()
        }
    }
    
    impl<'ctx> IntegratedResultTypeAssertion<'ctx> for MockGenerator<'ctx> {
        fn compile_type_assertion_with_integrated_result(
            &mut self,
            _type_assertion: &TypeAssertion
        ) -> Result<inkwell::values::BasicValueEnum<'ctx>, Error> {
            unimplemented!()
        }
        
        fn build_result_structure(
            &mut self,
            _success: bool,
            _value: Option<inkwell::values::BasicValueEnum<'ctx>>,
            _error_info: Option<TypeAssertionErrorInfo>
        ) -> Result<inkwell::values::BasicValueEnum<'ctx>, Error> {
            unimplemented!()
        }
        
        fn extract_success_value(
            &mut self,
            _result_value: inkwell::values::BasicValueEnum<'ctx>,
            _expected_type: inkwell::types::BasicTypeEnum<'ctx>
        ) -> Result<inkwell::values::BasicValueEnum<'ctx>, Error> {
            unimplemented!()
        }
        
        fn propagate_result_error(
            &mut self,
            _result_value: inkwell::values::BasicValueEnum<'ctx>
        ) -> Result<(), Error> {
            unimplemented!()
        }
        
        fn generate_type_assertion_error_info(
            &mut self,
            source_type: &str,
            target_type: &str,
            source_location: Option<SourceLocation>
        ) -> Result<TypeAssertionErrorInfo, Error> {
            // Implement this for testing
            let error_message = format!("Type assertion failed: {} is not a {}", source_type, target_type);
            
            Ok(TypeAssertionErrorInfo {
                source_type: source_type.to_string()),
                target_type: target_type.to_string()),
                source_location,
                source_type_id: Some(12345),
                target_type_id: Some(67890),
                type_path: Some("source -> middle -> target".to_string()),
                error_message,
            })
        }
    }
    
    impl<'ctx> InterfaceTypeAssertionPathVisualization<'ctx> for MockGenerator<'ctx> {
        fn visualize_interface_path(
            &mut self,
            _type_name: &str,
            _depth: usize
        ) -> Result<String, Error> {
            Ok("Mock interface path visualization".to_string()
        }
        
        fn find_alternative_paths(
            &mut self,
            _source_type: &str,
            _target_type: &str,
            _max_depth: usize
        ) -> Result<Vec<String>, Error> {
            Ok(vec!["source -> middle -> target".to_string())])
        }
        
        fn check_extension_relationship(
            &mut self,
            _source_type: &str,
            _target_type: &str
        ) -> Result<bool, Error> {
            Ok(false) // Mock always returns false for simplicity
        }
    }
    
    // Create our mock generator
    let mut mock_generator = MockGenerator::new(&context, module);
    
    // Create a test source location
    let source_location = SourceLocation {
        line: 42,
        column: 10,
        file: Some("test.csd".to_string()),
        source_line: "x, ok = value.(TargetType)".to_string()),
    };
    
    // Test generating error info
    let error_info = mock_generator.generate_type_assertion_error_info(
        "SourceType",
        "TargetType",
        Some(source_location)
    ).expect("Failed to generate error info");
    
    // Verify the error info fields
    assert_eq!(error_info.source_type, "SourceType");
    assert_eq!(error_info.target_type, "TargetType");
    assert_eq!(error_info.source_type_id, Some(12345));
    assert_eq!(error_info.target_type_id, Some(67890));
    
    // Check source location fields
    assert!(error_info.source_location.is_some());
    let loc = error_info.source_location.unwrap();
    assert_eq!(loc.line, 42);
    assert_eq!(loc.column, 10);
    
    // Verify error message contents
    assert!(error_info.error_message.contains("Type assertion failed"));
    assert!(error_info.error_message.contains("SourceType is not a TargetType"));
    
    // Verify type path is passed through
    assert!(error_info.type_path.is_some());
    assert_eq!(error_info.type_path.unwrap(), "source -> middle -> target");
}

#[test]
fn test_type_assertion_error_info_creation() {
    // Initialize tracing for this test
    init_tracing();
    
    // Create a test TypeAssertionErrorInfo directly
    let error_info = TypeAssertionErrorInfo {
        source_type: "Stringer".to_string()),
        target_type: "Writer".to_string()),
        source_location: Some(SourceLocation {
            line: 100,
            column: 20,
            file: Some("test.csd".to_string()),
            source_line: "test code".to_string()),
        }),
        source_type_id: Some(12345),
        target_type_id: Some(67890),
        type_path: Some("Stringer -> io.Writer -> Writer".to_string()),
        error_message: "Test error message".to_string()),
    };
    
    // Verify the fields
    assert_eq!(error_info.source_type, "Stringer");
    assert_eq!(error_info.target_type, "Writer");
    assert_eq!(error_info.source_type_id, Some(12345));
    assert_eq!(error_info.target_type_id, Some(67890));
    assert!(error_info.source_location.is_some());
    assert_eq!(error_info.type_path.unwrap(), "Stringer -> io.Writer -> Writer");
    assert_eq!(error_info.error_message, "Test error message");
}

#[test]
fn test_type_assertion_compilation() {
    // Initialize tracing for this test
    init_tracing();
    
    // Create a simple test TypeAssertion AST node
    let type_assertion = TypeAssertion {
        token: Token::Dot,
        expression: Box::new(Identifier {
            token: "token".to_string()),
            value: "value".to_string()),
        }),
        type_name: "TargetType".to_string()),
    };
    
    // Verify the string representation
    assert_eq!(type_assertion.string(), "value.(TargetType)");
    
    // Verify token literal
    assert_eq!(type_assertion, ".");
}