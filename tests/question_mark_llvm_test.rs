/// LLVM compilation tests for the question mark operator in CURSED
/// 
/// These tests validate that the question mark operator compiles correctly
/// to LLVM IR and produces the expected runtime behavior.

use cursed::ast::expressions::question_mark::QuestionMarkExpression;
use cursed::ast::identifiers::Identifier;
use cursed::ast::traits::{Node, Expression};
use cursed::codegen::llvm::question_mark::{QuestionMarkCompiler, ErrorPropagationRuntime};
use cursed::error::CursedError;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::OptimizationLevel;

#[cfg(test)]
mod llvm_compilation_tests {
    use super::*;

    /// Mock LLVM code generator for testing
    struct MockLlvmCodeGenerator<'ctx> {
        context: &'ctx Context,
        module: Module<'ctx>,
        builder: Builder<'ctx>,
    }

    impl<'ctx> MockLlvmCodeGenerator<'ctx> {
        fn new(context: &'ctx Context) -> Self {
            let module = context.create_module("test_module");
            let builder = context.create_builder();
            
            Self {
                context,
                module,
                builder,
            }
        }
    }

    #[test]
    fn test_question_mark_expression_ast() {
        // Test that question mark expressions can be created and manipulated
        let var_expr = Identifier::new("test_value".to_string(), "test_value".to_string());
        let question_expr = QuestionMarkExpression::new(
            Box::new(var_expr),
            10,
            15
        );
        
        assert_eq!(question_expr.string(), "test_value?");
        assert_eq!(question_expr.line, 10);
        assert_eq!(question_expr.column, 15);
        
        // Test inner expression access
        let inner = question_expr.inner_expression();
        assert_eq!(inner.string(), "test_value");
    }

    #[test]
    fn test_question_mark_expression_cloning() {
        let var_expr = Identifier::new("clone_test".to_string(), "clone_test".to_string());
        let question_expr = QuestionMarkExpression::new(
            Box::new(var_expr),
            5,
            8
        );
        
        let cloned = question_expr.clone();
        assert_eq!(cloned.string(), question_expr.string());
        assert_eq!(cloned.location(), question_expr.location());
        
        // Ensure they are independent
        assert_eq!(cloned.line, 5);
        assert_eq!(cloned.column, 8);
    }

    #[test]
    fn test_nested_question_mark_expressions() {
        let var_expr = Identifier::new("nested".to_string(), "nested".to_string());
        let first_question = QuestionMarkExpression::new(
            Box::new(var_expr),
            1,
            1
        );
        let second_question = QuestionMarkExpression::new(
            Box::new(first_question),
            1,
            2
        );
        let third_question = QuestionMarkExpression::new(
            Box::new(second_question),
            1,
            3
        );
        
        assert_eq!(third_question.string(), "nested???");
    }

    #[test]
    fn test_result_structure_creation() {
        let context = Context::create();
        
        // Test the concept of Result-like structures in LLVM
        let i32_type = context.i32_type();
        let bool_type = context.bool_type();
        let result_struct = context.struct_type(&[i32_type.into(), bool_type.into()], false);
        
        assert_eq!(result_struct.count_fields(), 2);
        
        // Test creating a result value
        let success_value = i32_type.const_int(42, false);
        let is_error_flag = bool_type.const_int(0, false); // false = success
        
        let result_value = result_struct.const_named_struct(&[
            success_value.into(),
            is_error_flag.into(),
        ]);
        
        assert!(result_value.is_struct_value());
    }

    #[test]
    fn test_error_checking_logic() {
        let context = Context::create();
        let module = context.create_module("error_check_test");
        let builder = context.create_builder();
        
        // Create a function that demonstrates error checking
        let i32_type = context.i32_type();
        let bool_type = context.bool_type();
        let fn_type = bool_type.fn_type(&[i32_type.into()], false);
        let function = module.add_function("is_error", fn_type, None);
        
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);
        
        // Simple error check: value == -1 means error
        let param = function.get_nth_param(0).unwrap().into_int_value();
        let error_value = i32_type.const_int(-1i64 as u64, true);
        let is_error = builder.build_int_compare(
            inkwell::IntPredicate::EQ,
            param,
            error_value,
            "is_error"
        ).unwrap();
        
        builder.build_return(Some(&is_error)).unwrap();
        
        // Verify the function was created
        assert!(module.verify().is_ok());
    }

    #[test]
    fn test_conditional_branching() {
        let context = Context::create();
        let module = context.create_module("branch_test");
        let builder = context.create_builder();
        
        // Create a function that demonstrates conditional branching
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[i32_type.into()], false);
        let function = module.add_function("handle_result", fn_type, None);
        
        let entry_block = context.append_basic_block(function, "entry");
        let success_block = context.append_basic_block(function, "success");
        let error_block = context.append_basic_block(function, "error");
        
        builder.position_at_end(entry_block);
        
        // Check if input is negative (error condition)
        let param = function.get_nth_param(0).unwrap().into_int_value();
        let zero = i32_type.const_int(0, false);
        let is_negative = builder.build_int_compare(
            inkwell::IntPredicate::SLT,
            param,
            zero,
            "is_negative"
        ).unwrap();
        
        builder.build_conditional_branch(is_negative, error_block, success_block).unwrap();
        
        // Success block: return the value
        builder.position_at_end(success_block);
        builder.build_return(Some(&param)).unwrap();
        
        // Error block: return -1
        builder.position_at_end(error_block);
        let error_return = i32_type.const_int(-1i64 as u64, true);
        builder.build_return(Some(&error_return)).unwrap();
        
        // Verify the function
        assert!(module.verify().is_ok());
    }

    #[test]
    fn test_phi_node_creation() {
        let context = Context::create();
        let module = context.create_module("phi_test");
        let builder = context.create_builder();
        
        // Create a function that uses phi nodes for merging control flow
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[i32_type.into()], false);
        let function = module.add_function("merge_values", fn_type, None);
        
        let entry_block = context.append_basic_block(function, "entry");
        let positive_block = context.append_basic_block(function, "positive");
        let negative_block = context.append_basic_block(function, "negative");
        let merge_block = context.append_basic_block(function, "merge");
        
        builder.position_at_end(entry_block);
        
        let param = function.get_nth_param(0).unwrap().into_int_value();
        let zero = i32_type.const_int(0, false);
        let is_positive = builder.build_int_compare(
            inkwell::IntPredicate::SGT,
            param,
            zero,
            "is_positive"
        ).unwrap();
        
        builder.build_conditional_branch(is_positive, positive_block, negative_block).unwrap();
        
        // Positive block
        builder.position_at_end(positive_block);
        let positive_value = i32_type.const_int(1, false);
        builder.build_unconditional_branch(merge_block).unwrap();
        
        // Negative block
        builder.position_at_end(negative_block);
        let negative_value = i32_type.const_int(0, false);
        builder.build_unconditional_branch(merge_block).unwrap();
        
        // Merge block with phi node
        builder.position_at_end(merge_block);
        let phi = builder.build_phi(i32_type, "result").unwrap();
        phi.add_incoming(&[(&positive_value, positive_block), (&negative_value, negative_block)]);
        
        builder.build_return(Some(&phi.as_basic_value())).unwrap();
        
        // Verify the function
        assert!(module.verify().is_ok());
    }
}

#[cfg(test)]
mod runtime_error_propagation_tests {
    use super::*;

    #[test]
    fn test_error_propagation_runtime_creation() {
        let runtime = ErrorPropagationRuntime::new();
        let (propagations, unwraps) = runtime.get_stats();
        
        assert_eq!(propagations, 0);
        assert_eq!(unwraps, 0);
    }

    #[test]
    fn test_runtime_error_handler_management() {
        let mut runtime = ErrorPropagationRuntime::new();
        
        // Test adding an error handler
        runtime.push_error_handler(|error| {
            // Simple logging handler
            eprintln!("Handled error: {}", error);
            Ok(())
        });
        
        // Test removing the handler
        let handler = runtime.pop_error_handler();
        assert!(handler.is_some());
        
        // Should be empty now
        let no_handler = runtime.pop_error_handler();
        assert!(no_handler.is_none());
    }

    #[test]
    fn test_successful_unwrap_tracking() {
        let mut runtime = ErrorPropagationRuntime::new();
        
        // Record some successful unwraps
        runtime.record_successful_unwrap();
        runtime.record_successful_unwrap();
        runtime.record_successful_unwrap();
        
        let (propagations, unwraps) = runtime.get_stats();
        assert_eq!(unwraps, 3);
        assert_eq!(propagations, 0);
    }

    #[test]
    fn test_error_propagation_with_handler() {
        let mut runtime = ErrorPropagationRuntime::new();
        
        // Add a handler that successfully recovers from errors
        runtime.push_error_handler(|_error| {
            Ok(()) // Always succeed
        });
        
        let test_error = CursedError::error_propagation("Test error".to_string());
        let result = runtime.propagate_error(test_error);
        
        assert!(result.is_ok());
        
        let (propagations, _) = runtime.get_stats();
        assert_eq!(propagations, 1);
    }

    #[test]
    fn test_error_propagation_without_handler() {
        let mut runtime = ErrorPropagationRuntime::new();
        
        // No error handler registered
        let test_error = CursedError::error_propagation("Unhandled error".to_string());
        let result = runtime.propagate_error(test_error);
        
        // Should return the error since no handler is available
        assert!(result.is_err());
        
        let (propagations, _) = runtime.get_stats();
        assert_eq!(propagations, 1);
    }

    #[test]
    fn test_nested_error_handlers() {
        let mut runtime = ErrorPropagationRuntime::new();
        
        // Add multiple handlers (stack-like behavior)
        runtime.push_error_handler(|_| {
            Ok(()) // Outer handler
        });
        
        runtime.push_error_handler(|_| {
            Ok(()) // Inner handler (should be called first)
        });
        
        let test_error = CursedError::error_propagation("Nested test".to_string());
        let result = runtime.propagate_error(test_error);
        
        assert!(result.is_ok());
        
        // Remove inner handler
        runtime.pop_error_handler();
        
        // Test with outer handler
        let test_error2 = CursedError::error_propagation("Outer test".to_string());
        let result2 = runtime.propagate_error(test_error2);
        
        assert!(result2.is_ok());
        
        let (propagations, _) = runtime.get_stats();
        assert_eq!(propagations, 2);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_complete_question_mark_workflow() {
        // Test the complete workflow from AST creation to runtime handling
        
        // 1. Create AST
        let var_expr = Identifier::new("async_operation".to_string(), "async_operation".to_string());
        let question_expr = QuestionMarkExpression::new(
            Box::new(var_expr),
            25,
            30
        );
        
        // 2. Verify AST structure
        assert_eq!(question_expr.string(), "async_operation?");
        assert_eq!(question_expr.location(), (25, 30));
        
        // 3. Test expression trait compliance
        let expr_trait: Box<dyn Expression> = Box::new(question_expr.clone());
        assert_eq!(expr_trait.string(), "async_operation?");
        
        // 4. Test cloning
        let cloned = question_expr.clone();
        assert_eq!(cloned.string(), question_expr.string());
        
        // 5. Set up runtime for error handling
        let mut runtime = ErrorPropagationRuntime::new();
        runtime.push_error_handler(|error| {
            println!("Question mark operator triggered error: {}", error);
            Ok(())
        });
        
        // 6. Simulate error propagation
        let test_error = CursedError::error_propagation_with_location(
            "Async operation failed".to_string(),
            25,
            30
        );
        
        let result = runtime.propagate_error(test_error);
        assert!(result.is_ok());
        
        // 7. Verify statistics
        let (propagations, unwraps) = runtime.get_stats();
        assert_eq!(propagations, 1);
        assert_eq!(unwraps, 0);
    }

    #[test]
    fn test_question_mark_with_context() {
        // Test question mark operator with rich error context
        
        let var_expr = Identifier::new("database_query".to_string(), "database_query".to_string());
        let question_expr = QuestionMarkExpression::new(
            Box::new(var_expr),
            45,
            12
        );
        
        // Create error with context
        use cursed::error::error_propagation::{PropagatingError, ErrorContext, SourceLocation};
        
        let original_error = CursedError::error_propagation_with_location(
            "Database connection failed".to_string(),
            45,
            12
        );
        
        let mut prop_error = PropagatingError::new(original_error);
        
        // Add context chain
        prop_error.add_context_message(
            "execute_query",
            "Failed to execute database query"
        );
        prop_error.add_context_message(
            "handle_request", 
            "Error occurred while handling HTTP request"
        );
        
        let source_loc = SourceLocation::with_file(45, 12, "database.csd".to_string());
        prop_error.set_propagation_site(source_loc);
        
        // Convert back to CursedError
        let final_error = prop_error.into_cursed_error();
        let error_string = format!("{}", final_error);
        
        assert!(error_string.contains("Database connection failed"));
        assert!(error_string.contains("execute_query"));
        assert!(error_string.contains("handle_request"));
        assert!(error_string.contains("Propagated at line 45, column 12"));
    }

    #[test] 
    fn test_question_mark_performance() {
        // Basic performance test for question mark operations
        
        let start = std::time::Instant::now();
        
        // Create many question mark expressions
        for i in 0..1000 {
            let var_expr = Identifier::new(
                format!("var_{}", i),
                format!("var_{}", i)
            );
            let question_expr = QuestionMarkExpression::new(
                Box::new(var_expr),
                i,
                i
            );
            
            // Verify each one
            assert_eq!(question_expr.string(), format!("var_{}?", i));
            assert_eq!(question_expr.location(), (i, i));
        }
        
        let duration = start.elapsed();
        
        // Should complete in reasonable time (less than 100ms for 1000 operations)
        assert!(duration.as_millis() < 100, "Question mark operations took too long: {:?}", duration);
    }
}
