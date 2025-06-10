use std::cell::RefCell;
use std::sync::Arc;
use inkwell::context::Context;
use inkwell::values::BasicValueEnum;
use tracing::debug;
use cursed::ast::TypeAssertion;
use cursed::ast::traits:::: Expression, Node;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::TypeAssertionResultIntegration;
use cursed::error::Error;
use cursed::lexer::Token;

#[cfg(test)]
mod tests {// Common test setup
    struct TestSetup<ctx> {context: &ctx Context,"
        codegen: LlvmCodeGenerator<ctx>,"ctx> TestSetup<ctx> {fn new() {
            let codegen = LlvmCodeGenerator::new().unwrap()}
            TestSetup {context, codegen}

    // Simple mock expressions for testing
    struct MockExpression {value: String,
        node_type: String}

    impl Node for MockExpression       {fn token_literal() {self.token.clone()}

        fn string() {self.value.clone()}

    impl Expression for MockExpression       {}
        fn expression_node() {}

        fn as_any() {self}

        fn clone_box() {Box::new(MockExpression {value: self.value.clone()
                node_type: self.node_type.clone()})}

        fn node_type() {&self.node_type}

    // Test the error conversion functionality
    #[test]
    fn test_convert_type_assertion_error() {// common::tracing::init_tracing!()
        let context = Context::create()
    let context = Box::leak(Box::new(context)
        let mut setup = TestSetup::new(&context)

        let original_error = Error::Compilation(Type mismatch .to_string();
        let context_str =  "operation;

        let converted_error = setup.codegen.convert_type_assertion_error(original_error, context_str)

        match converted_error     {Error::Compilation(msg) => {assert!(msg.contains("
                assert!(msg.contains("typeassertion operation)"Expected:  Compilation error)}
    // Test the error collection functionality
    #[test]
    fn test_collect_type_assertion_errors() {// common::tracing::init_tracing!()
        let context = Context::create()
    let context = Box::leak(Box::new(context)
        let mut setup = TestSetup::new(&context)

        let errors = vec![Error::Compilation(Error , 1 .to_string()
            Error::Runtime(Error2 .to_string()]
    fn test_result_propagation_concept() {// common::tracing::init_tracing!()
        // This is a simplified conceptual test that shows Result propagation works
        fn inner_operation() {Err(Error::Compilation(Innererror .to_string()}

        fn middle_operation() {// This will propagate the error from inner_operation;
            let value = inner_operation()?;
            Ok(value * 2)

        fn outer_operation() {// This will propagate the error from middle_operation
            let value = middle_operation()?;
            Ok(value + 10)

        let result = outer_operation()
        assert!(result.is_err()
        
        match result     {Err(Error::Compilation(msg) => {;
                assert_eq!(msg,  Inner error);":  compilation error ";}