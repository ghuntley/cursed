use std::cell::RefCell;
use std::sync::Arc;
use inkwell::context::Context;
use inkwell::values::BasicValueEnum;
use tracing::debug;
use cursed::ast::TypeAssertion;
use cursed::ast::traits::{Expression, Node};
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::TypeAssertionResultIntegration;
use cursed::error::Error;
use cursed::lexer::Token;

#[cfg(test)]
mod tests {


    // Common test setup
    struct TestSetup<'ctx> {
        context: &'ctx Context,
        codegen: LlvmCodeGenerator<'ctx>,
    }

    impl<'ctx> TestSetup<'ctx> {
        fn new(context: &'ctx Context) -> Self {
            let codegen = LlvmCodeGenerator::new(context);
            TestSetup { context, codegen }
        }
    }

    // Simple mock expressions for testing
    struct MockExpression {
        token: String,
        value: String,
        node_type: String,
    }

    impl Node for MockExpression {
        fn token_literal(&self) -> String {
            self.token.clone()
        }

        fn string(&self) -> String {
            self.value.clone()
        }
    }

    impl Expression for MockExpression {
        fn expression_node(&self) {}

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }

        fn clone_box(&self) -> Box<dyn Expression> {
            Box::new(MockExpression {
                token: self.token.clone(),
                value: self.value.clone(),
                node_type: self.node_type.clone(),
            })
        }

        fn node_type(&self) -> &str {
            &self.node_type
        }
    }

    // Test the error conversion functionality
    #[test]
    fn test_convert_type_assertion_error() {
    // init_tracing!();
        let context = Context::create();
        let mut setup = TestSetup::new(&context);

        let original_error = Error::Compilation("Type mismatch".to_string());
        let context_str = "type assertion operation";

        let converted_error = setup.codegen.convert_type_assertion_error(original_error, context_str);

        match converted_error {
            Error::Compilation(msg) => {
                assert!(msg.contains("Type mismatch"));
                assert!(msg.contains("type assertion operation"));
            },
            _ => panic!("Expected Compilation error")
        }
    }

    // Test the error collection functionality
    #[test]
    fn test_collect_type_assertion_errors() {
    // init_tracing!();
        let context = Context::create();
        let mut setup = TestSetup::new(&context);

        let errors = vec![Error::Compilation("Error 1".to_string()),
            Error::Runtime("Error 2".to_string())];

        let combined_error = setup.codegen.collect_type_assertion_errors(errors);

        match combined_error {
            Error::Compilation(msg) => {
                assert!(msg.contains("Multiple type assertion errors"));
                assert!(msg.contains("Error 1"));
                assert!(msg.contains("Error 2"));
            },
            _ => panic!("Expected Compilation error")
        }
    }

    // Test the error report creation
    #[test]
    fn test_create_type_assertion_error_report() {
    // init_tracing!();
        let context = Context::create();
        let setup = TestSetup::new(&context);

        let interface_type = "Greeter";
        let target_type = "Person";
        let source_location = "test.csd:42";
        let errors = vec![Error::Compilation("Type mismatch".to_string())];

        let report = setup.codegen.create_type_assertion_error_report(
            interface_type,
            target_type,
            source_location,
            &errors
        );

        assert!(report.contains("Type Assertion Error Report");
        assert!(report.contains("test.csd:42"));
        assert!(report.contains("Greeter -> Person"));
        assert!(report.contains("Type mismatch"));
    }

    // This test would verify the full ? operator integration if we could set up the complete test environment
    // However, this would require much more extensive test scaffolding including function setup
    #[test]
    fn test_result_propagation_concept() {
    // init_tracing!();
        // This is a simplified conceptual test that shows Result propagation works
        fn inner_operation() -> Result<i32, Error> {
            Err(Error::Compilation("Inner error".to_string()))
        }

        fn middle_operation() -> Result<i32, Error> {
            // This will propagate the error from inner_operation
            let value = inner_operation()?;
            Ok(value * 2)
        }

        fn outer_operation() -> Result<i32, Error> {
            // This will propagate the error from middle_operation
            let value = middle_operation()?;
            Ok(value + 10)
        }

        let result = outer_operation();
        assert!(result.is_err());
        
        match result {
            Err(Error::Compilation(msg)) => {
                assert_eq!(msg, "Inner error");
            },
            _ => panic!("Expected compilation error")
        }
    }
}