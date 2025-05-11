//! Error recovery strategies for range clause iteration
//!
//! This module provides error recovery mechanisms for range-based iteration,
//! particularly focusing on map iteration which may fail due to various reasons
//! such as invalid map types, incompatible key/value types, or runtime errors.
//!
//! The implementation provides graceful fallbacks, helpful error messages,
//! and debugging context to improve developer experience when dealing with
//! range clause errors.

use crate::ast::Expression;
use crate::ast::statements::BlockStatement;
use crate::ast::expressions::RangeExpression;
use crate::codegen::llvm::context::LlvmCodeGenerator;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::codegen::llvm::statement::StatementCompilation;
use crate::codegen::llvm::range_clause_fixed::{RangeClauseCompilationEnhanced};
use crate::error::Error;
use inkwell::values::{BasicValueEnum, IntValue, PointerValue};
use tracing::{debug, info, instrument, warn, error};

/// Trait for implementing error recovery strategies for range clause iteration
///
/// This trait provides methods to handle various error cases that can occur during
/// range clause compilation and execution, with a focus on map iteration which
/// is one of the more complex cases.
pub trait RangeClauseErrorRecovery<'ctx> {
    /// Creates a fallback map iterator when the original map expression cannot be evaluated
    /// 
    /// This method provides a graceful fallback when the map expression cannot be compiled
    /// or is invalid, allowing the code to continue execution with an empty iterator.
    fn create_fallback_map_iterator(&mut self) -> Result<PointerValue<'ctx>, Error>;
    
    /// Recovers from type errors in map key-value iteration
    ///
    /// When the key or value types don't match what's expected, this method provides
    /// appropriate type conversion where possible, or suitable default values.
    fn recover_from_map_key_value_type_error(
        &mut self,
        key_name: &str,
        value_name: &str,
        map_expr: &Box<dyn Expression>,
    ) -> Result<(PointerValue<'ctx>, PointerValue<'ctx>), Error>;
    
    /// Recovers from container type errors by providing a fallback container
    ///
    /// This is used when an expression that was expected to be a container (array, slice, etc.)
    /// has an incompatible type. The method creates a suitable empty container to allow
    /// the code to continue execution without crashing.
    fn recover_from_container_type_error(
        &mut self,
        container_expr: &Box<dyn Expression>,
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Provides detailed diagnostic information for range clause errors
    ///
    /// This method helps developers identify and fix issues by providing specific
    /// information about what went wrong and potential fixes.
    fn provide_range_error_diagnostics(
        &self,
        error: &Error,
        context: &str,
    ) -> String;
    
    /// Handles iterator advancement failures by providing a replacement iterator
    ///
    /// This method is used when an iterator fails to advance to the next element,
    /// allowing the iteration to continue in a controlled way.
    fn handle_iterator_advancement_failure(
        &mut self,
        iterator_ptr: PointerValue<'ctx>,
    ) -> Result<PointerValue<'ctx>, Error>;
}

impl<'ctx> RangeClauseErrorRecovery<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn create_fallback_map_iterator(&mut self) -> Result<PointerValue<'ctx>, Error> {
        debug!("Creating fallback map iterator for recovery");
        
        // Create an empty map object
        let map_type = self.context.struct_type(
            &[
                self.context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(), // data pointer
                self.context.i32_type().into(), // capacity
                self.context.i32_type().into(), // size
            ],
            false
        );
        
        // Allocate space for the empty map
        let map_ptr = self.builder.build_alloca(map_type, "empty_map_fallback")?;
        
        // Initialize the struct fields to zero/null values
        let null_ptr = self.context.i8_type().ptr_type(inkwell::AddressSpace::default())
            .const_null();
        let zero = self.context.i32_type().const_zero();
        
        // Get pointers to each field
        let data_ptr_ptr = unsafe {
            self.builder.build_struct_gep(
                map_type,
                map_ptr,
                0,
                "map_data_ptr_ptr"
            )?
        };
        
        let capacity_ptr = unsafe {
            self.builder.build_struct_gep(
                map_type,
                map_ptr,
                1,
                "map_capacity_ptr"
            )?
        };
        
        let size_ptr = unsafe {
            self.builder.build_struct_gep(
                map_type,
                map_ptr,
                2,
                "map_size_ptr"
            )?
        };
        
        // Store the initial values
        self.builder.build_store(data_ptr_ptr, null_ptr)?;
        self.builder.build_store(capacity_ptr, zero)?;
        self.builder.build_store(size_ptr, zero)?;
        
        // Now create a map iterator using this empty map
        // We'll use the normal runtime function, but with our empty map
        self.ensure_runtime_container_functions()?;
        
        let module = self.module()
            .ok_or_else(|| Error::Compilation("Module not available".to_string()))?;
        
        // Get or create the map iterator create function
        let fn_name = "map_iterator_create";
        let fn_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::default())
            .fn_type(&[map_ptr.get_type().into()], false);
        
        let create_iterator_fn = module.get_function(fn_name).unwrap_or_else(|| {
            let function = module.add_function(fn_name, fn_type, None);
            function.set_linkage(inkwell::module::Linkage::External);
            function
        });
        
        // Call the function
        let call = self.builder.build_call(
            create_iterator_fn, 
            &[map_ptr.into()], 
            "create_empty_iterator_call"
        )?;
        
        // Get the returned iterator pointer
        let iterator_ptr = call.try_as_basic_value().left()
            .ok_or_else(|| Error::Compilation("Failed to get iterator pointer".to_string()))?;
        
        if !iterator_ptr.is_pointer_value() {
            return Err(Error::Compilation("Iterator creation failed".to_string()));
        }
        
        Ok(iterator_ptr.into_pointer_value())
    }
    
    #[instrument(skip(self, map_expr), fields(key = key_name, value = value_name), level = "debug")]
    fn recover_from_map_key_value_type_error(
        &mut self,
        key_name: &str,
        value_name: &str,
        map_expr: &Box<dyn Expression>,
    ) -> Result<(PointerValue<'ctx>, PointerValue<'ctx>), Error> {
        debug!("Recovering from map key-value type error");
        
        // Log the error first
        warn!("Map expression has type errors, providing default key-value variables");
        
        // Create default key and value variables with appropriate common types
        // For key, we'll use string type (tea) as it's the most common map key type
        // For value, we'll use int64 (lit) as it's a commonly used value type
        
        // Allocate space for the default key (string/tea)
        let key_ptr = self.builder.build_alloca(
            self.context.i8_type().ptr_type(inkwell::AddressSpace::default()),
            key_name
        )?;
        
        // Allocate space for the default value (int64/lit)
        let value_ptr = self.builder.build_alloca(
            self.context.i64_type(),
            value_name
        )?;
        
        // Initialize with empty string and zero
        let null_string = self.context.i8_type().ptr_type(inkwell::AddressSpace::default())
            .const_null();
        let zero = self.context.i64_type().const_zero();
        
        self.builder.build_store(key_ptr, null_string)?;
        self.builder.build_store(value_ptr, zero)?;
        
        // Return the key and value pointers
        Ok((key_ptr, value_ptr))
    }
    
    #[instrument(skip(self, container_expr), level = "debug")]
    fn recover_from_container_type_error(
        &mut self,
        container_expr: &Box<dyn Expression>,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Recovering from container type error");
        
        // Log the error first
        warn!("Container expression has type errors, providing empty array fallback");
        
        // Create an empty array as a fallback
        // We'll use a 0-length array of i64 values
        let empty_array_type = self.context.i64_type().array_type(0);
        let empty_array = empty_array_type.const_array(&[]);
        
        // Create a pointer to this empty array for easier handling
        let array_ptr = self.builder.build_alloca(empty_array_type, "empty_array_fallback")?;
        self.builder.build_store(array_ptr, empty_array)?;
        
        Ok(array_ptr.into())
    }
    
    #[instrument(skip(self, error), level = "debug")]
    fn provide_range_error_diagnostics(
        &self,
        error: &Error,
        context: &str,
    ) -> String {
        // Extract error details and provide useful diagnostic information
        match error {
            Error::Compilation(msg) => {
                format!("Range clause error ({}): {}", context, msg)
            },
            Error::CodeGenError(msg) => {
                format!("Range code generation error ({}): {}", context, msg)
            },
            Error::Parsing(msg) => {
                format!("Range parsing error ({}): {}", context, msg)
            },
            _ => {
                format!("Range error in {}: {:?}", context, error)
            }
        }
    }
    
    #[instrument(skip(self, iterator_ptr), level = "debug")]
    fn handle_iterator_advancement_failure(
        &mut self,
        iterator_ptr: PointerValue<'ctx>,
    ) -> Result<PointerValue<'ctx>, Error> {
        debug!("Handling iterator advancement failure");
        
        // Log the issue
        warn!("Iterator advancement failed, creating replacement iterator");
        
        // The typical approach here would be to:
        // 1. Create a new empty iterator (via create_fallback_map_iterator)
        // 2. Replace the failed iterator with this new one
        
        // However, since we already have an iterator pointer, we'll try to reset it
        // to a safe state that will report no more elements available
        
        // Get the module
        let module = self.module()
            .ok_or_else(|| Error::Compilation("Module not available".to_string()))?;
        
        // Define or get a function to reset an iterator to safe state
        let fn_name = "reset_iterator_to_end";
        let fn_type = self.context.void_type()
            .fn_type(&[iterator_ptr.get_type().into()], false);
        
        let reset_fn = module.get_function(fn_name).unwrap_or_else(|| {
            let function = module.add_function(fn_name, fn_type, None);
            function.set_linkage(inkwell::module::Linkage::External);
            function
        });
        
        // Call the function to reset the iterator
        self.builder.build_call(
            reset_fn,
            &[iterator_ptr.into()],
            "reset_iterator_call"
        )?;
        
        // Return the original iterator pointer, now in a safe state
        Ok(iterator_ptr)
    }
}

/// Extended trait to add error recovery to the range clause compilation
///
/// This trait extends the existing RangeClauseCompilationEnhanced trait with
/// methods that apply error recovery strategies during range clause compilation.
pub trait RangeClauseCompilationWithRecovery<'ctx>: RangeClauseErrorRecovery<'ctx> {
    /// Compiles a map iteration for loop with error recovery
    ///
    /// This method enhances the standard map for loop compilation with error
    /// recovery mechanisms to handle invalid maps, type errors, and runtime failures.
    fn compile_map_for_loop_with_recovery(
        &mut self,
        key_name: &str,
        value_name: &str,
        map_expr: &Box<dyn Expression>,
        body: &BlockStatement,
    ) -> Result<(), Error>;
    
    /// Compiles a container iteration for loop with error recovery
    ///
    /// This method adds error recovery to container iteration, handling
    /// invalid containers, type mismatches, and runtime errors.
    fn compile_container_for_loop_with_recovery(
        &mut self,
        value_name: &str,
        container_expr: &Box<dyn Expression>,
        body: &BlockStatement,
    ) -> Result<(), Error>;
}

impl<'ctx> RangeClauseCompilationWithRecovery<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self, body, map_expr), fields(key = key_name, value = value_name))]
    fn compile_map_for_loop_with_recovery(
        &mut self,
        key_name: &str,
        value_name: &str,
        map_expr: &Box<dyn Expression>,
        body: &BlockStatement,
    ) -> Result<(), Error> {
        info!("Compiling map for loop with recovery for key: {} and value: {}", key_name, value_name);
        
        // Ensure all required runtime functions are available
        self.ensure_runtime_container_functions()?;
        
        // Try to evaluate the map expression with error handling
        let map_value = match self.compile_expression(map_expr.as_ref()) {
            Ok(value) => value,
            Err(err) => {
                // Log the error
                warn!(
                    error = %err,
                    "Map expression compilation failed, using empty map fallback"
                );
                
                // Create a compile-time warning about the issue
                let diag = self.provide_range_error_diagnostics(&err, "map iteration");
                error!(diagnostic = %diag, "Map iteration error diagnostics");
                
                // Return a default BasicValueEnum that represents an empty map
                // We'll use a basic struct type with null pointer and zero size
                let empty_map_type = self.context.struct_type(
                    &[
                        self.context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(), // data pointer
                        self.context.i32_type().into(), // capacity
                        self.context.i32_type().into(), // size
                    ],
                    false
                );
                
                // Allocate and zero-initialize
                let map_ptr = self.builder.build_alloca(empty_map_type, "empty_map_fallback")?;
                let null_ptr = self.context.i8_type().ptr_type(inkwell::AddressSpace::default())
                    .const_null();
                let zero = self.context.i32_type().const_zero();
                
                // Initialize fields
                let data_ptr_ptr = unsafe {
                    self.builder.build_struct_gep(
                        empty_map_type,
                        map_ptr,
                        0,
                        "map_data_ptr_ptr"
                    )?
                };
                
                let capacity_ptr = unsafe {
                    self.builder.build_struct_gep(
                        empty_map_type,
                        map_ptr,
                        1,
                        "map_capacity_ptr"
                    )?
                };
                
                let size_ptr = unsafe {
                    self.builder.build_struct_gep(
                        empty_map_type,
                        map_ptr,
                        2,
                        "map_size_ptr"
                    )?
                };
                
                self.builder.build_store(data_ptr_ptr, null_ptr)?;
                self.builder.build_store(capacity_ptr, zero)?;
                self.builder.build_store(size_ptr, zero)?;
                
                self.builder.build_load(empty_map_type, map_ptr, "empty_map_value")?.into()
            }
        };
        
        // Get the current function or return an error if not in a function
        let func = self.current_function()
            .ok_or_else(|| Error::Compilation("No current function".to_string()))?;
            
        let context = self.context;
        
        // Create basic blocks for the loop
        let loop_setup = context.append_basic_block(func, "map.for.setup");
        let loop_entry = context.append_basic_block(func, "map.for.entry");
        let loop_body = context.append_basic_block(func, "map.for.body");
        let loop_increment = context.append_basic_block(func, "map.for.increment");
        let loop_exit = context.append_basic_block(func, "map.for.exit");
        let error_handler = context.append_basic_block(func, "map.for.error");
        
        // Set up iteration
        self.builder.build_unconditional_branch(loop_setup)?;
        self.builder.position_at_end(loop_setup);
        
        // Try to create a map iterator with error handling
        let iterator_ptr = match self.emit_map_iterator_create_fixed(map_value) {
            Ok(ptr) => ptr,
            Err(err) => {
                // Log the error
                warn!(
                    error = %err,
                    "Map iterator creation failed, using fallback"
                );
                
                // Jump to error handler
                self.builder.build_unconditional_branch(error_handler)?;
                self.builder.position_at_end(error_handler);
                
                // Create a fallback iterator
                let fallback_ptr = self.create_fallback_map_iterator()?;
                
                // Jump to loop entry to continue execution
                self.builder.build_unconditional_branch(loop_entry)?;
                self.builder.position_at_end(loop_entry);
                
                fallback_ptr
            }
        };
        
        // If we didn't jump to the error handler, continue with normal flow
        if self.builder.get_insert_block().unwrap().get_name().to_str().unwrap() == "map.for.setup" {
            self.builder.build_unconditional_branch(loop_entry)?;
        }
        
        // Loop entry: check if iterator has next element
        self.builder.position_at_end(loop_entry);
        let has_next = match self.emit_map_iterator_has_next_fixed(iterator_ptr) {
            Ok(val) => val,
            Err(err) => {
                // Log the error
                warn!(
                    error = %err,
                    "Map iterator has_next check failed, assuming no more elements"
                );
                
                // Assume no more elements
                self.context.bool_type().const_int(0, false)
            }
        };
        self.builder.build_conditional_branch(has_next, loop_body, loop_exit)?;
        
        // Loop body: get current key-value pair and execute body
        self.builder.position_at_end(loop_body);
        
        // Try to determine key and value types with fallback
        let (key_type, value_type) = match (self.determine_map_key_type_fixed(map_value), 
                                           self.determine_map_value_type_fixed(map_value)) {
            (Ok(kt), Ok(vt)) => (kt, vt),
            _ => {
                // Fallback to common types
                warn!("Could not determine map key/value types, using fallback types");
                (
                    self.context.i8_type().ptr_type(inkwell::AddressSpace::default()), // string/tea for key
                    self.context.i64_type() // int64/lit for value
                )
            }
        };
        
        // Allocate memory for current key and value
        let key_ptr = self.builder.build_alloca(key_type, key_name)?;
        let value_ptr = self.builder.build_alloca(value_type, value_name)?;
        
        // Try to get current key-value pair with error handling
        match self.emit_map_iterator_get_current_fixed(iterator_ptr, key_ptr, value_ptr) {
            Ok(_) => {}, // Successfully retrieved key-value pair
            Err(err) => {
                // Log the error
                warn!(
                    error = %err,
                    "Failed to get current key-value pair from map iterator"
                );
                
                // Initialize with default values
                let null_key = key_type.const_zero();
                let zero_value = value_type.const_zero();
                
                self.builder.build_store(key_ptr, null_key)?;
                self.builder.build_store(value_ptr, zero_value)?;
            }
        };
        
        // Create a new scope for the loop variables
        self.push_scope(super::variables::VariableScope::new());
        // Create placeholder types for the key and value variables
        let dummy_type = crate::core::type_checker::Type::Thicc; // 64-bit integer
        self.add_variable(key_name, key_ptr, &dummy_type)?;
        self.add_variable(value_name, value_ptr, &dummy_type)?;
        
        // Set up loop context for break/continue
        let old_loop_exit = self.replace_loop_exit(Some(loop_exit));
        let old_loop_continue = self.replace_loop_continue(Some(loop_increment));
        
        // Compile the loop body
        self.compile_statement(body)?;
        
        // Restore previous loop context
        self.replace_loop_exit(old_loop_exit);
        self.replace_loop_continue(old_loop_continue);
        
        // Pop the scope when done with the loop body
        self.pop_scope();
        
        // Check if we need to jump to increment
        let current_block = self.builder.get_insert_block();
        if let Some(block) = current_block {
            if block.get_terminator().is_none() {
                self.builder.build_unconditional_branch(loop_increment)?;
            }
        }
        
        // Loop increment: advance the iterator
        self.builder.position_at_end(loop_increment);
        match self.emit_map_iterator_next_fixed(iterator_ptr) {
            Ok(_) => {}, // Successfully advanced iterator
            Err(err) => {
                // Log the error
                warn!(
                    error = %err,
                    "Failed to advance map iterator, handling failure"
                );
                
                // Try to handle the advancement failure
                let _ = self.handle_iterator_advancement_failure(iterator_ptr);
            }
        };
        self.builder.build_unconditional_branch(loop_entry)?;
        
        // Loop exit block
        self.builder.position_at_end(loop_exit);
        
        // Position builder at error handler if we haven't generated code for it yet
        if self.builder.get_insert_block().unwrap().get_name().to_str().unwrap() != "map.for.error" {
            self.builder.position_at_end(error_handler);
            // Create a fallback iterator (just to have valid code in this block)
            let _ = self.create_fallback_map_iterator()?;
            self.builder.build_unconditional_branch(loop_exit)?;
            
            // Return to the exit block
            self.builder.position_at_end(loop_exit);
        }
        
        Ok(())
    }
    
    #[instrument(skip(self, body, container_expr), fields(value = value_name))]
    fn compile_container_for_loop_with_recovery(
        &mut self,
        value_name: &str,
        container_expr: &Box<dyn Expression>,
        body: &BlockStatement,
    ) -> Result<(), Error> {
        info!("Compiling container for loop with recovery for value: {}", value_name);
        
        // Ensure all required runtime functions are available
        self.ensure_runtime_container_functions()?;
        
        // Try to evaluate container expressions with error handling
        let container_value = match self.compile_expression(container_expr.as_ref()) {
            Ok(value) => value,
            Err(err) => {
                // Log the error
                warn!(
                    error = %err,
                    "Container expression compilation failed, using empty array fallback"
                );
                
                // Create a compile-time warning about the issue
                let diag = self.provide_range_error_diagnostics(&err, "container iteration");
                error!(diagnostic = %diag, "Container iteration error diagnostics");
                
                // Recover by creating an empty array fallback
                self.recover_from_container_type_error(container_expr)?
            }
        };
        
        // Get the current function or return an error if not in a function
        let func = self.current_function()
            .ok_or_else(|| Error::Compilation("No current function".to_string()))?;
            
        let context = self.context;
        
        // Create basic blocks for the loop
        let loop_setup = context.append_basic_block(func, "container.for.setup");
        let loop_entry = context.append_basic_block(func, "container.for.entry");
        let loop_body = context.append_basic_block(func, "container.for.body");
        let loop_increment = context.append_basic_block(func, "container.for.increment");
        let loop_exit = context.append_basic_block(func, "container.for.exit");
        let error_handler = context.append_basic_block(func, "container.for.error");
        
        // Set up iteration
        self.builder.build_unconditional_branch(loop_setup)?;
        self.builder.position_at_end(loop_setup);
        
        // Create an index variable for iteration
        let i_ptr = self.builder.build_alloca(context.i32_type(), "index")?;
        self.builder.build_store(i_ptr, context.i32_type().const_zero())?;
        
        // Try to determine container length with error handling
        let length_value = match self.emit_container_length_fixed(container_value) {
            Ok(len) => len,
            Err(err) => {
                // Log the error
                warn!(
                    error = %err,
                    "Container length determination failed, assuming length 0"
                );
                
                // Jump to error handler
                self.builder.build_unconditional_branch(error_handler)?;
                self.builder.position_at_end(error_handler);
                
                // Create a diagnostic message
                let diag = self.provide_range_error_diagnostics(&err, "container length");
                error!(diagnostic = %diag, "Container length error diagnostics");
                
                // Return zero length
                let zero_length = context.i32_type().const_zero();
                
                // Jump to loop entry
                self.builder.build_unconditional_branch(loop_entry)?;
                self.builder.position_at_end(loop_entry);
                
                zero_length
            }
        };
        
        // Try to determine element type with fallback
        let element_type = match self.determine_element_type_fixed(container_value) {
            Ok(ty) => ty,
            Err(err) => {
                // Log the error
                warn!(
                    error = %err,
                    "Could not determine container element type, using i64 fallback"
                );
                
                // Use i64 (lit) as a fallback
                context.i64_type().into()
            }
        };
        
        // Allocate memory for the current element
        let value_ptr = self.builder.build_alloca(element_type, value_name)?;
        
        // If we didn't jump to the error handler, continue with normal flow
        if self.builder.get_insert_block().unwrap().get_name().to_str().unwrap() == "container.for.setup" {
            self.builder.build_unconditional_branch(loop_entry)?;
        }
        
        // Loop entry: check if index < length
        self.builder.position_at_end(loop_entry);
        let current_index = self.builder.build_load(context.i32_type(), i_ptr, "current.index")?
            .into_int_value();
        let condition = self.builder.build_int_compare(
            inkwell::IntPredicate::SLT,
            current_index,
            length_value,
            "loop.condition"
        )?;
        self.builder.build_conditional_branch(condition, loop_body, loop_exit)?;
        
        // Loop body: get current element and execute body
        self.builder.position_at_end(loop_body);
        
        // Get the current element from the container with error handling
        match self.emit_get_element_fixed(container_value, current_index) {
            Ok(element) => {
                // Store the element in the value variable
                self.builder.build_store(value_ptr, element)?
            },
            Err(err) => {
                // Log the error
                warn!(
                    error = %err,
                    index = %current_index.to_string(),
                    "Failed to get element from container, using zero value"
                );
                
                // Store a zero/default value instead
                let zero_element = element_type.const_zero();
                self.builder.build_store(value_ptr, zero_element)?
            }
        };
        
        // Create a new scope for the loop variable
        self.push_scope(super::variables::VariableScope::new());
        // Create a placeholder type for the value variable
        let dummy_type = crate::core::type_checker::Type::Thicc; // 64-bit integer
        self.add_variable(value_name, value_ptr, &dummy_type)?;
        
        // Set up loop context for break/continue
        let old_loop_exit = self.replace_loop_exit(Some(loop_exit));
        let old_loop_continue = self.replace_loop_continue(Some(loop_increment));
        
        // Compile the loop body
        self.compile_statement(body)?;
        
        // Restore previous loop context
        self.replace_loop_exit(old_loop_exit);
        self.replace_loop_continue(old_loop_continue);
        
        // Pop the scope when done with the loop body
        self.pop_scope();
        
        // Check if we need to jump to increment
        let current_block = self.builder.get_insert_block();
        if let Some(block) = current_block {
            if block.get_terminator().is_none() {
                self.builder.build_unconditional_branch(loop_increment)?;
            }
        }
        
        // Loop increment block
        self.builder.position_at_end(loop_increment);
        let current_index = self.builder.build_load(context.i32_type(), i_ptr, "current.index.inc")?
            .into_int_value();
        let incremented = self.builder.build_int_add(
            current_index,
            context.i32_type().const_int(1, false),
            "incremented.index"
        )?;
        self.builder.build_store(i_ptr, incremented)?;
        self.builder.build_unconditional_branch(loop_entry)?;
        
        // Loop exit block
        self.builder.position_at_end(loop_exit);
        
        // Position builder at error handler if we haven't generated code for it yet
        if self.builder.get_insert_block().unwrap().get_name().to_str().unwrap() != "container.for.error" {
            self.builder.position_at_end(error_handler);
            // Just branch to the exit (this code is already handled above)
            self.builder.build_unconditional_branch(loop_exit)?;
            
            // Return to the exit block
            self.builder.position_at_end(loop_exit);
        }
        
        Ok(())
    }
}