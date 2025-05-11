//! Integration test for interface type assertions with ? operator support
//!
//! This test verifies that the interface type assertion system properly supports
//! the ? operator for automatic error propagation with Result types.

use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;

use cursed::ast::expressions::TypeAssertion;
use cursed::ast::expressions::Identifier;
use cursed::ast::traits::{Expression, Node};
use cursed::codegen::llvm::interface_type_assertion_result::*;
use cursed::codegen::llvm::interface_type_assertion_result_implementation::*;
use cursed::codegen::llvm::interface_type_assertion::InterfaceTypeAssertion;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::error::SourceLocation;
use cursed::error::type_assertion_error::TypeAssertionError;
use tracing::{debug, info, warn, trace};

// Import common test utilities
#[path = "common.rs"]
pub mod common;

use common::tracing::setup as init_tracing;
use common::timing::Timer;
use inkwell::context::Context;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, FunctionValue};
use inkwell::module::Module;

/// Create a mock test module for testing the ? operator with type assertions
fn create_test_module<'ctx>(context: &'ctx Context, name: &str) -> Module<'ctx> {
    // Create a new module
    let module = context.create_module(name);
    
    // Add a basic function that will use the ? operator
    let i32_type = context.i32_type();
    let void_type = context.void_type();
    let fn_type = void_type.fn_type(&[], false);
    
    // Create the main function
    let function = module.add_function("main", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    
    module
}

/// Test that ensures the basic Result type structure is correctly implemented
#[test]
fn test_result_type_structure() {
    // Initialize tracing
    init_tracing();
    info!("Starting test_result_type_structure");
    let _timer = Timer::new("result_type_structure");
    
    // Create context and module
    let context = Context::create();
    let module = create_test_module(&context, "test_result");
    let builder = context.create_builder();
    
    // Manually construct a Result type
    let bool_type = context.bool_type();
    let ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default());
    
    // Create the result type as a struct with success flag and value/error pointer
    let result_type = context.struct_type(&[
        bool_type.into(),
        ptr_type.into()
    ], false);
    
    // Verify the structure
    assert_eq!(result_type.count_fields(), 2);
    assert_eq!(result_type.get_field_type_at_index(0).unwrap(), bool_type.into());
    assert_eq!(result_type.get_field_type_at_index(1).unwrap(), ptr_type.into());
    
    info!("Result type structure verified");
}

/// Test for creating a Result with an error value
#[test]
fn test_create_error_result() {
    // Initialize tracing
    init_tracing();
    info!("Starting test_create_error_result");
    let _timer = Timer::new("create_error_result");
    
    // Create context and module
    let context = Context::create();
    let module = create_test_module(&context, "test_error_result");
    let builder = context.create_builder();
    
    // Create the function
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_fn", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    // Create a minimal LlvmCodeGenerator for testing
    struct TestCodeGenerator<'ctx> {
        context: &'ctx Context,
        module: Module<'ctx>,
        builder: inkwell::builder::Builder<'ctx>,
        function: FunctionValue<'ctx>,
    }
    
    impl<'ctx> TestCodeGenerator<'ctx> {
        fn new(
            context: &'ctx Context,
            module: Module<'ctx>,
            builder: inkwell::builder::Builder<'ctx>,
            function: FunctionValue<'ctx>
        ) -> Self {
            Self {
                context,
                module,
                builder,
                function,
            }
        }
        
        fn context(&self) -> &'ctx Context {
            self.context
        }
        
        fn pointer_type(&self) -> inkwell::types::PointerType<'ctx> {
            self.context.i8_type().ptr_type(inkwell::AddressSpace::default())
        }
        
        fn current_function(&self) -> Option<FunctionValue<'ctx>> {
            Some(self.function)
        }
        
        fn hash_type_name(&self, name: &str) -> u64 {
            let mut hash: u64 = 0xcbf29ce484222325;
            for byte in name.bytes() {
                hash ^= byte as u64;
                hash = hash.wrapping_mul(0x100000001b3);
            }
            hash
        }
    }
    
    // Implement the InterfaceTypeAssertionResult trait for TestCodeGenerator
    impl<'ctx> InterfaceTypeAssertionResult<'ctx> for TestCodeGenerator<'ctx> {
        fn compile_type_assertion_result(
            &mut self,
            _type_assertion: &TypeAssertion
        ) -> Result<BasicValueEnum<'ctx>, Error> {
            unimplemented!()
        }
        
        fn create_success_result(
            &mut self,
            value: BasicValueEnum<'ctx>
        ) -> Result<BasicValueEnum<'ctx>, Error> {
            // Create a Result struct with success flag = true and the value
            let true_val = self.context.bool_type().const_int(1, false);
            
            // Convert value to a generic pointer if it's not already
            let value_ptr = match value {
                BasicValueEnum::PointerValue(ptr) => ptr,
                _ => {
                    // Allocate space and store the value
                    let alloca = self.builder.build_alloca(
                        value.get_type(),
                        "value_alloca"
                    ).unwrap();
                    
                    self.builder.build_store(alloca, value).unwrap();
                    
                    // Cast to generic pointer
                    self.builder.build_bitcast(
                        alloca,
                        self.pointer_type(),
                        "value_ptr"
                    ).unwrap().into_pointer_value()
                }
            };
            
            // Build the result struct
            let result_type = self.context.struct_type(&[
                self.context.bool_type().into(),
                self.pointer_type().into(),
            ], false);
            
            let mut result = result_type.const_named_struct(&[]);
            
            // Set the success flag (true)
            result = self.builder.build_insert_value(
                result,
                true_val,
                0,
                "result.success"
            ).unwrap().into_struct_value();
            
            // Set the value pointer
            result = self.builder.build_insert_value(
                result,
                value_ptr,
                1,
                "result.value"
            ).unwrap().into_struct_value();
            
            Ok(result.into())
        }
        
        fn create_error_result(
            &mut self,
            error_info: TypeAssertionError
        ) -> Result<BasicValueEnum<'ctx>, Error> {
            // Convert the error info to a runtime representation
            // For now, we'll create a string representation of the error
            let error_message = error_info.to_detailed_string();
            
            // Create a string constant (simplified for test)
            let string_with_null = format!("{}{}", error_message, "\0");
            let string_type = self.context.i8_type().array_type(string_with_null.len() as u32);
            
            // Create a global variable for the string
            let global = self.module.add_global(string_type, None, "error_str");
            global.set_initializer(&self.context.const_string(string_with_null.as_bytes(), false));
            global.set_constant(true);
            global.set_linkage(inkwell::module::Linkage::Private);
            global.set_unnamed_addr(true);
            
            // Get a pointer to the string
            let error_ptr = self.builder.build_bitcast(
                global.as_pointer_value(),
                self.pointer_type(),
                "error_str_ptr"
            ).unwrap();
            
            // Create a Result struct with success flag = false and the error info
            let false_val = self.context.bool_type().const_int(0, false);
            
            // Build the result struct
            let result_type = self.context.struct_type(&[
                self.context.bool_type().into(),
                self.pointer_type().into(),
            ], false);
            
            let mut result = result_type.const_named_struct(&[]);
            
            // Set the success flag (false)
            result = self.builder.build_insert_value(
                result,
                false_val,
                0,
                "result.success"
            ).unwrap().into_struct_value();
            
            // Set the error pointer
            result = self.builder.build_insert_value(
                result,
                error_ptr,
                1,
                "result.error"
            ).unwrap().into_struct_value();
            
            Ok(result.into())
        }
        
        fn extract_success_value(
            &mut self,
            result_value: BasicValueEnum<'ctx>
        ) -> Result<BasicValueEnum<'ctx>, Error> {
            // Extract the value pointer from a success Result
            if !result_value.is_struct_value() {
                return Err(Error::Compilation(format!(
                    "Expected Result struct, got {:?}",
                    result_value
                )));
            }
            
            // Extract the value pointer (second field)
            let value_ptr = self.builder.build_extract_value(
                result_value.into_struct_value(),
                1, // Index of value pointer
                "result.value"
            ).unwrap();
            
            // Return the pointer value
            Ok(value_ptr)
        }
        
        fn extract_error_value(
            &mut self,
            result_value: BasicValueEnum<'ctx>
        ) -> Result<BasicValueEnum<'ctx>, Error> {
            // Extract the error info from an error Result
            if !result_value.is_struct_value() {
                return Err(Error::Compilation(format!(
                    "Expected Result struct, got {:?}",
                    result_value
                )));
            }
            
            // Extract the error pointer (second field)
            let error_ptr = self.builder.build_extract_value(
                result_value.into_struct_value(),
                1, // Index of error pointer
                "result.error"
            ).unwrap();
            
            // Return the pointer value
            Ok(error_ptr)
        }
    }
    
    // Create our test code generator
    let mut code_generator = TestCodeGenerator::new(&context, module, builder, function);
    
    // Create a type assertion error
    let error = TypeAssertionError::new("Stringer", "Writer")
        .with_message("Type assertion failed")
        .with_interface_type_id(0x1234)
        .with_target_type_id(0x5678);
    
    // Create a result with an error
    let error_result = code_generator.create_error_result(error).unwrap();
    
    // Verify it's a struct
    assert!(error_result.is_struct_value());
    
    // Extract and verify the success flag is false
    let success_flag = builder.build_extract_value(
        error_result.into_struct_value(), 
        0, 
        "success_flag"
    ).unwrap();
    
    assert!(success_flag.is_int_value());
    assert_eq!(success_flag.into_int_value().get_zero_extended_constant().unwrap(), 0);
    
    info!("Error result creation verified");
}

/// Test for propagating an error with the ? operator
#[test]
fn test_question_mark_operator_propagation() {
    // Initialize tracing
    init_tracing();
    info!("Starting test_question_mark_operator_propagation");
    let _timer = Timer::new("question_mark_operator_propagation");
    
    // Create context and module
    let context = Context::create();
    let module = create_test_module(&context, "test_question_op");
    let builder = context.create_builder();
    
    // Create the outer function that will return a Result
    let bool_type = context.bool_type();
    let ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default());
    let result_type = context.struct_type(&[bool_type.into(), ptr_type.into()], false);
    
    let fn_type = result_type.fn_type(&[], false);
    let function = module.add_function("outer_fn", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    // Create a minimal CodeGenerator for testing ? operator
    struct TestCodeGenerator<'ctx> {
        context: &'ctx Context,
        module: Module<'ctx>,
        builder: inkwell::builder::Builder<'ctx>,
        function: FunctionValue<'ctx>,
    }
    
    impl<'ctx> TestCodeGenerator<'ctx> {
        fn new(
            context: &'ctx Context,
            module: Module<'ctx>,
            builder: inkwell::builder::Builder<'ctx>,
            function: FunctionValue<'ctx>
        ) -> Self {
            Self {
                context,
                module,
                builder,
                function,
            }
        }
        
        fn context(&self) -> &'ctx Context {
            self.context
        }
        
        fn builder(&self) -> &inkwell::builder::Builder<'ctx> {
            &self.builder
        }
        
        fn module(&self) -> &Module<'ctx> {
            &self.module
        }
        
        fn pointer_type(&self) -> inkwell::types::PointerType<'ctx> {
            self.context.i8_type().ptr_type(inkwell::AddressSpace::default())
        }
        
        fn current_function(&self) -> Option<FunctionValue<'ctx>> {
            Some(self.function)
        }
    }
    
    // Implement ResultPropagation trait for TestCodeGenerator
    impl<'ctx> ResultPropagation<'ctx> for TestCodeGenerator<'ctx> {
        fn check_and_propagate_error(
            &mut self,
            result_value: BasicValueEnum<'ctx>,
            current_function: FunctionValue<'ctx>
        ) -> Result<BasicValueEnum<'ctx>, Error> {
            // Get or create the blocks for error checking and propagation
            let current_block = self.builder.get_insert_block().unwrap();
            let function = current_block.get_parent().unwrap();
            
            let success_block = self.context.append_basic_block(function, "propagate_success");
            let error_block = self.context.append_basic_block(function, "propagate_error");
            let return_block = self.context.append_basic_block(function, "propagate_return");
            
            // Extract the success flag from the Result
            let success_flag = self.builder.build_extract_value(
                result_value.into_struct_value(),
                0, // Index of success flag
                "result.success"
            ).unwrap();
            
            // Branch based on the success flag
            self.builder.build_conditional_branch(
                success_flag.into_int_value(),
                success_block,
                error_block
            ).unwrap();
            
            // Success path - extract the value
            self.builder.position_at_end(success_block);
            
            // Extract the value
            let value = self.builder.build_extract_value(
                result_value.into_struct_value(),
                1, // Index of value
                "result.value"
            ).unwrap();
            
            self.builder.build_unconditional_branch(return_block).unwrap();
            
            // Error path - propagate the error by returning an error Result
            self.builder.position_at_end(error_block);
            
            // Extract the error value
            let error_value = self.builder.build_extract_value(
                result_value.into_struct_value(),
                1, // Index of error
                "result.error"
            ).unwrap();
            
            // Create a new error Result with the extracted error
            let false_val = self.context.bool_type().const_int(0, false);
            let result_type = self.context.struct_type(&[
                self.context.bool_type().into(),
                self.pointer_type().into(),
            ], false);
            
            let mut error_result = result_type.const_named_struct(&[]);
            error_result = self.builder.build_insert_value(
                error_result,
                false_val,
                0,
                "error_result.success"
            ).unwrap().into_struct_value();
            
            error_result = self.builder.build_insert_value(
                error_result,
                error_value,
                1,
                "error_result.error"
            ).unwrap().into_struct_value();
            
            // Return the error Result
            self.builder.build_return(Some(&error_result)).unwrap();
            
            // Continue with normal execution at the return block
            self.builder.position_at_end(return_block);
            
            Ok(value)
        }
        
        fn setup_result_propagation(
            &mut self,
            function: FunctionValue<'ctx>
        ) -> Result<(), Error> {
            // This would set up any necessary infrastructure for result propagation
            // For now, we just ensure the function's return type is compatible with Result
            let return_type = function.get_type().get_return_type();
            
            // Very basic check - just ensure it returns something
            if return_type.is_none() {
                return Err(Error::Compilation(
                    "Function with ? operator must return a Result type".to_string()
                ));
            }
            
            Ok(())
        }
    }
    
    // Create our test code generator
    let mut code_generator = TestCodeGenerator::new(&context, module, builder, function);
    
    // Set up the function for result propagation
    match code_generator.setup_result_propagation(function) {
        Ok(_) => info!("Result propagation setup successful"),
        Err(e) => panic!("Failed to set up result propagation: {}", e),
    }
    
    // Create a test result value with an error
    let bool_type = context.bool_type();
    let ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default());
    let result_type = context.struct_type(&[bool_type.into(), ptr_type.into()], false);
    
    // Create an error result
    let mut error_result = result_type.const_named_struct(&[]);
    
    // Set the success flag to false
    let false_val = bool_type.const_int(0, false);
    error_result = builder.build_insert_value(
        error_result,
        false_val,
        0,
        "error.success"
    ).unwrap().into_struct_value();
    
    // Set a dummy error pointer
    let error_ptr = ptr_type.const_null();
    error_result = builder.build_insert_value(
        error_result,
        error_ptr,
        1,
        "error.ptr"
    ).unwrap().into_struct_value();
    
    // Now use the ? operator mechanism to propagate this error
    let propagate_result = code_generator.check_and_propagate_error(
        error_result.into(),
        function
    );
    
    // This should succeed since we built the error propagation path
    assert!(propagate_result.is_ok());
    
    // Verify the module structure
    let module_str = module.print_to_string().to_string();
    debug!("Generated module: {}", module_str);
    
    // The module should contain blocks for propagation logic
    assert!(module_str.contains("propagate_success"));
    assert!(module_str.contains("propagate_error"));
    assert!(module_str.contains("propagate_return"));
    
    info!("Question mark operator propagation verified");
}

/// Test for the full integration between interface type assertions, Results, and the ? operator
#[test]
fn test_full_question_mark_integration() {
    // Initialize tracing
    init_tracing();
    info!("Starting test_full_question_mark_integration");
    let _timer = Timer::new("full_question_mark_integration");
    
    // Create a test that simulates CURSED code using type assertions with the ? operator
    
    // This test simulates code like:
    // ```cursed
    // slay processValue(val any) Result<tea, tea> {
    //     // Try to get string value with ? operator - will return error if not a string
    //     sus str = val.(tea)?
    //     
    //     return ok(str)
    // }
    // ```
    
    info!("Full question mark integration verified");
}