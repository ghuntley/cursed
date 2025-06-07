//! Implementation of range clauses for iteration in loops
//!
//! This module provides functionality for:
//! - Numeric range-based loops (for i := range 10 {...})
//! - Single-value container iteration (for elem := range array {...})
//! - Key-value map iteration (for key, value := range map {...})
//!
//! The implementation uses a standardized approach following the module
//! architecture, where functionality is added to LlvmCodeGenerator through
//! trait implementation.

use crate::ast::{BlockStatement, Expression, RangeExpression};
use crate::codegen::llvm::context::LlvmCodeGenerator;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::codegen::llvm::variables::VariableHandling;
use crate::codegen::llvm::statement::StatementCompilation;
use crate::codegen::llvm::pointer_type_extension::PointerTypeExtension;
use inkwell::basic_block::BasicBlock;
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::{BasicValue, BasicValueEnum, IntValue, PointerValue};
use inkwell::IntPredicate;
use tracing::{debug, info, instrument};
use inkwell::AddressSpace;
use crate::error::Error;
use std::convert::From;

/// Trait for implementing range-based iteration in for loops
///
/// This trait provides methods to compile different types of range clauses:
/// - Numeric ranges with optional start, end, and step values
/// - Container iteration (arrays, slices)
/// - Key-value iteration for maps
pub trait RangeClauseCompilation<'ctx> {
    /// Compiles a range-based for loop with numeric bounds
    ///
    /// Supports various range clause forms:
    /// - `for i := range end` (implicit start=0, step=1)
    /// - `for i := range start, end` (implicit step=1)
    /// - `for i := range start, end, step` (fully specified)
    ///
    /// Handles positive and negative steps, and special cases like empty ranges.
    fn compile_range_for_loop(
        &mut self,
        iterator_name: &str,
        range_expr: &RangeExpression,
        body: &BlockStatement,
    ) -> Result<(), Error>;

    /// Compiles container iteration for arrays, slices, and similar sequential containers
    ///
    /// Usage: `for elem := range container { ... }`
    fn compile_container_for_loop(
        &mut self,
        value_name: &str,
        container_expr: &Box<dyn Expression>,
        body: &BlockStatement,
    ) -> Result<(), Error>;

    /// Compiles key-value map iteration
    ///
    /// Usage: `for key, value := range map { ... }`
    fn compile_map_for_loop(
        &mut self,
        key_name: &str,
        value_name: &str,
        map_expr: &Box<dyn Expression>,
        body: &BlockStatement,
    ) -> Result<(), Error>;
}

impl<'ctx> RangeClauseCompilation<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self, body, range_expr), fields(iterator = iterator_name))]
    fn compile_range_for_loop(
        &mut self,
        iterator_name: &str,
        range_expr: &RangeExpression,
        body: &BlockStatement,
    ) -> Result<(), Error> {
        debug!("Compiling range for loop with iterator: {}", iterator_name);
        let func = self.current_function().ok_or(Error::Compilation("No current function".to_string()))?;
        let context = self.context;
        let builder = &self.builder;

        // Create basic blocks for the loop
        let loop_entry = context.append_basic_block(func, "range.for.entry");
        let loop_body = context.append_basic_block(func, "range.for.body");
        let loop_increment = context.append_basic_block(func, "range.for.increment");
        let loop_exit = context.append_basic_block(func, "range.for.exit");

        // Compile range expression components
        let (start_value, end_value, step_value) = match range_expr {
            RangeExpression::Range { end } => {
                // Basic form: for i := range end
                let end_val = self.compile_expression(end.as_ref())
                    .map_err(|e| Error::Compilation(format!("Failed to compile range end expression: {}", e)))?
                    .into_int_value();
                let start_val = context.i64_type().const_int(0, false); // Default start: 0
                let step_val = context.i64_type().const_int(1, false);  // Default step: 1
                (start_val, end_val, step_val)
            },
            RangeExpression::RangeFromTo { start, end } => {
                // Two-argument form: for i := range start, end
                let start_val = self.compile_expression(start.as_ref())
                    .map_err(|e| Error::Compilation(format!("Failed to compile range start expression: {}", e)))?
                    .into_int_value();
                let end_val = self.compile_expression(end.as_ref())
                    .map_err(|e| Error::Compilation(format!("Failed to compile range end expression: {}", e)))?
                    .into_int_value();
                let step_val = context.i64_type().const_int(1, false);  // Default step: 1
                (start_val, end_val, step_val)
            },
            RangeExpression::RangeFromToStep { start, end, step } => {
                // Complete form: for i := range start, end, step
                let start_val = self.compile_expression(start.as_ref())
                    .map_err(|e| Error::Compilation(format!("Failed to compile range start expression: {}", e)))?
                    .into_int_value();
                let end_val = self.compile_expression(end.as_ref())
                    .map_err(|e| Error::Compilation(format!("Failed to compile range end expression: {}", e)))?
                    .into_int_value();
                let step_val = self.compile_expression(step.as_ref())
                    .map_err(|e| Error::Compilation(format!("Failed to compile range step expression: {}", e)))?
                    .into_int_value();
                (start_val, end_val, step_val)
            },
        };

        // Allocate loop variable and initialize with start value
        let i_ptr = builder.build_alloca(context.i64_type(), iterator_name)?;
        builder.build_store(i_ptr, start_value)?;

        // Determine if we're counting up or down based on step sign
        let zero = context.i64_type().const_zero();
        let step_positive = builder.build_int_compare(
            IntPredicate::SGT,
            step_value,
            zero,
            "step.positive"
        );

        // Jump to loop entry
        builder.build_unconditional_branch(loop_entry);

        // Loop entry: check condition
        builder.position_at_end(loop_entry);
        let current_value = builder.build_load(context.i64_type(), i_ptr, "current")?
            .into_int_value();

        // Different comparison based on step direction
        // Create comparison based on step direction
        let step_gt_zero = self.builder.build_int_compare(
        IntPredicate::SGT,
        step_value,
        zero,
        "step.is.positive"
        )?;
        
        let condition = if step_gt_zero.is_constant_int() && 
        step_gt_zero.get_sign_extended_constant().unwrap_or(0) > 0 {
            // If step > 0, condition is current < end
            self.builder.build_int_compare(
                IntPredicate::SLT,
                current_value,
                end_value,
                "loop.condition.up"
            )?
        } else {
            // If step <= 0, condition is current > end
            self.builder.build_int_compare(
                IntPredicate::SGT,
                current_value,
                end_value,
                "loop.condition.down"
            )?
        };

        builder.build_conditional_branch(condition, loop_body, loop_exit)?;

        // Loop body
        builder.position_at_end(loop_body);

        // Push a new scope for the loop body
        self.push_scope(super::variables::VariableScope::new());

        // Add variable to current scope
        self.add_variable(iterator_name, i_ptr)?;

        // Track current loop blocks for break/continue using loop_context methods
        let old_loop_exit = self.replace_loop_exit(Some(loop_exit));
        let old_loop_continue = self.replace_loop_continue(Some(loop_increment));

        // Compile loop body
        self.compile_statement(body)?;

        // Restore previous loop blocks
        self.replace_loop_exit(old_loop_exit);
        self.replace_loop_continue(old_loop_continue);

        // Pop the loop body scope
        self.pop_scope();

        // Jump to increment if no explicit branch was added
        if builder.get_insert_block().unwrap().get_terminator().is_none() {
            builder.build_unconditional_branch(loop_increment);
        }

        // Loop increment
        builder.position_at_end(loop_increment);
        let current_value = builder.build_load(context.i64_type(), i_ptr, "current.inc")?
            .into_int_value();
        let incremented = builder.build_int_add(
            current_value,
            step_value,
            "incremented"
        )?;
        builder.build_store(i_ptr, incremented)?;
        builder.build_unconditional_branch(loop_entry)?;

        // Loop exit
        builder.position_at_end(loop_exit);

        Ok(())
    }

    #[instrument(skip(self, body, container_expr), fields(value = value_name))]
    fn compile_container_for_loop(
        &mut self,
        value_name: &str,
        container_expr: &Box<dyn Expression>,
        body: &BlockStatement,
    ) -> Result<(), Error> {
        debug!("Compiling container for loop with value: {}", value_name);
        let func = self.current_function().ok_or(Error::Compilation("No current function".to_string()))?;
        let context = self.context;
        let builder = &self.builder;

        // Evaluate the container expression
        let container_value = self.compile_expression(container_expr.as_ref())
            .map_err(|e| Error::Compilation(format!("Failed to compile container expression: {}", e)))?;

        // Create basic blocks for the loop
        let loop_setup = context.append_basic_block(func, "container.for.setup");
        let loop_entry = context.append_basic_block(func, "container.for.entry");
        let loop_body = context.append_basic_block(func, "container.for.body");
        let loop_increment = context.append_basic_block(func, "container.for.increment");
        let loop_exit = context.append_basic_block(func, "container.for.exit");

        // Get the container type and setup iteration
        builder.build_unconditional_branch(loop_setup);
        builder.position_at_end(loop_setup);

        // Create an index variable for iteration
        let i_ptr = builder.build_alloca(context.i32_type(), "index")?;
        builder.build_store(i_ptr, context.i32_type().const_zero())?;

        // Determine length based on container type (array, slice, etc.)
        // Assuming we have access to a length method or property
        let length_value = if container_value.is_array_value() {
            // For arrays, we know the length at compile time
            let array_value = container_value.into_array_value();
            let length = array_value.get_type().len();
            context.i32_type().const_int(length as u64, false)
        } else {
            // For other containers, call a length() method or access a length property
            // This is a simplified implementation - real implementation would use
            // reflection or type-specific access methods
            let length_result = self.emit_container_length_call(container_value)?;
            length_result.into_int_value()
        };

        // Allocate space for the current element value
        let value_ptr = builder.build_alloca(self.determine_element_type(container_value)?, value_name);

        builder.build_unconditional_branch(loop_entry);

        // Loop entry: check if index < length
        builder.position_at_end(loop_entry);
        let current_index_value = builder.build_load(context.i32_type(), i_ptr, "current.index")?;
        let current_index = current_index_value.into_int_value();
        let condition = builder.build_int_compare(
            IntPredicate::SLT,
            current_index,
            length_value,
            "loop.condition"
        )?;
        builder.build_conditional_branch(condition, loop_body, loop_exit)?;

        // Loop body: get current element and execute body
        builder.position_at_end(loop_body);

        // Get the current element from the container
        let current_element = self.emit_container_get_element(container_value, current_index)?;
        let unpacked_value_ptr = value_ptr?;
        builder.build_store(unpacked_value_ptr, current_element)?;

        // Push a new scope for the loop body
        self.push_scope(super::variables::VariableScope::new());

        // Add element variable to current scope - unwrap the value_ptr Result
        let unpacked_value_ptr = value_ptr?;
        self.add_variable(value_name, unpacked_value_ptr)?;

        // Track current loop blocks for break/continue
        let old_loop_exit = self.replace_loop_exit(Some(loop_exit));
        let old_loop_continue = self.replace_loop_continue(Some(loop_increment));

        // Compile loop body
        self.compile_statement(body)?;

        // Restore previous loop blocks
        self.replace_loop_exit(old_loop_exit);
        self.replace_loop_continue(old_loop_continue);

        // Pop the loop body scope
        self.pop_scope();

        // Jump to increment if no explicit branch was added
        if builder.get_insert_block().unwrap().get_terminator().is_none() {
            builder.build_unconditional_branch(loop_increment);
        }

        // Loop increment
        builder.position_at_end(loop_increment);
        let current_index_value = builder.build_load(context.i32_type(), i_ptr, "current.index.inc")?;
        let current_index = current_index_value.into_int_value();
        let incremented = builder.build_int_add(
            current_index,
            context.i32_type().const_int(1, false),
            "incremented.index"
        )?;
        builder.build_store(i_ptr, incremented)?;
        builder.build_unconditional_branch(loop_entry)?;

        // Loop exit
        builder.position_at_end(loop_exit);

        Ok(())
    }

    #[instrument(skip(self, body, map_expr), fields(key = key_name, value = value_name))]
    fn compile_map_for_loop(
        &mut self,
        key_name: &str,
        value_name: &str,
        map_expr: &Box<dyn Expression>,
        body: &BlockStatement,
    ) -> Result<(), Error> {
        debug!("Compiling map for loop with key: {} and value: {}", key_name, value_name);
        let func = self.current_function().ok_or(Error::Compilation("No current function".to_string()))?;
        let context = self.context;
        let builder = self.builder;

        // Evaluate the map expression
        let map_value = self.compile_expression(map_expr.as_ref())
            .map_err(|e| Error::Compilation(format!("Failed to compile map expression: {}", e)))?;

        // Create basic blocks for the loop
        let loop_setup = context.append_basic_block(func, "map.for.setup");
        let loop_entry = context.append_basic_block(func, "map.for.entry");
        let loop_body = context.append_basic_block(func, "map.for.body");
        let loop_increment = context.append_basic_block(func, "map.for.increment");
        let loop_exit = context.append_basic_block(func, "map.for.exit");

        // Set up iteration
        builder.build_unconditional_branch(loop_setup);
        builder.position_at_end(loop_setup);

        // Create an iterator for the map
        // This would involve calling a map iterator creation function
        let iterator_value = self.emit_map_iterator_create(map_value)?;
        let iterator_ptr = builder.build_alloca(iterator_value.get_type(), "map.iterator")?;
        builder.build_store(iterator_ptr, iterator_value)?;

        // Allocate variables for key and value
        let key_type = self.determine_map_key_type(map_value)?;
        let value_type = self.determine_map_value_type(map_value)?;
        
        let key_ptr = builder.build_alloca(key_type, key_name);
        let value_ptr = builder.build_alloca(value_type, value_name);

        builder.build_unconditional_branch(loop_entry);

        // Loop entry: check if iterator has more elements
        builder.position_at_end(loop_entry);
        let iterator = builder.build_load(
            iterator_value.get_type(),
            iterator_ptr,
            "current.iterator"
        )?;
        let has_next = self.emit_iterator_has_next(iterator)?;
        builder.build_conditional_branch(has_next, loop_body, loop_exit)?;

        // Loop body: get current key-value pair and execute body
        builder.position_at_end(loop_body);

        // Get current key-value pair
        let (current_key, current_value) = self.emit_iterator_get_key_value(iterator)?;
        let unpacked_key_ptr = key_ptr?;
        let unpacked_value_ptr = value_ptr?;
        builder.build_store(unpacked_key_ptr, current_key)?;
        builder.build_store(unpacked_value_ptr, current_value)?;

        // Push a new scope for the loop body
        self.push_scope(super::variables::VariableScope::new());

        // Add key and value variables to current scope
        self.add_variable(key_name, unpacked_key_ptr)?;
        self.add_variable(value_name, unpacked_value_ptr)?;

        // Track current loop blocks for break/continue
        let old_loop_exit = self.replace_loop_exit(Some(loop_exit));
        let old_loop_continue = self.replace_loop_continue(Some(loop_increment));

        // Compile loop body
        self.compile_statement(body)?;

        // Restore previous loop blocks
        self.replace_loop_exit(old_loop_exit);
        self.replace_loop_continue(old_loop_continue);

        // Pop the loop body scope
        self.pop_scope();

        // Jump to increment if no explicit branch was added
        if builder.get_insert_block().unwrap().get_terminator().is_none() {
            builder.build_unconditional_branch(loop_increment);
        }

        // Loop increment: advance the iterator
        builder.position_at_end(loop_increment);
        let iterator = builder.build_load(
            iterator_value.get_type(),
            iterator_ptr,
            "current.iterator.inc"
        )?;
        let advanced_iterator = self.emit_iterator_next(iterator)?;
        builder.build_store(iterator_ptr, advanced_iterator)?;
        builder.build_unconditional_branch(loop_entry)?;

        // Loop exit
        builder.position_at_end(loop_exit);

        Ok(())
    }
}

// Helper methods as extension trait
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Helper function to emit code for getting container length
    fn emit_container_length_call(&mut self, container: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        // Get the container length based on its type
        let builder = self.builder;
        
        // Check the type of container
        if container.is_array_value() {
            // For arrays, use the length from the type
            let array_value = container.into_array_value();
            let length = array_value.get_type().len();
            Ok(self.context.i32_type().const_int(length as u64, false).into())
        } else if container.is_pointer_value() {
            // For pointers to containers (slices, strings, etc.), call the appropriate
            // length method or access the length field
            let ptr_type = container.into_pointer_value().get_type();
            let element_type = ptr_type.get_element_type();
            
            if element_type.is_struct_type() {
                // For containers like slices or strings which are structs with length field
                // Call the length method or access the length field
                // This assumes the container has a length() method
                
                // Create function signature for length method
                let container_type = element_type.into_struct_type();
                let fn_type = self.context.i32_type().fn_type(&[container.get_type().into()], false);
                
                // Get or create the length function
                let length_fn_name = format!("_{}_length", container_type.get_name().to_str().unwrap_or("container"));
                let module = self.module.as_ref().unwrap();
                let length_fn = module.get_function(&length_fn_name).unwrap_or_else(|| {
                    module.add_function(&length_fn_name, fn_type, None)
                });
                
                // Call the length function
                let call = builder.build_call(length_fn.into(), &[container.into()], "length_call");
                
                // Return the result
                call.try_as_basic_value().left().ok_or_else(|| Error::Compilation("Failed to get length return value".to_string()))
            } else if let Some(array_type) = element_type.into_array_type_enum() {
                // For pointers to arrays, use the array length
                let length = array_type.len();
                Ok(self.context.i32_type().const_int(length as u64, false).into())
            } else {
                // For other container types, try to find and call a length method
                // or access a length field
                
                // This is a simplified approach - in a real implementation, you would
                // need to handle different container types differently
                
                // For demonstration purposes, we'll use a default approach of calling a length method
                let fn_type = self.context.i32_type().fn_type(&[container.get_type().into()], false);
                let module = self.module.as_ref().unwrap();
                let length_fn = module.get_function("container_length").unwrap_or_else(|| {
                    module.add_function("container_length", fn_type, None)
                });
                
                // Call the length function
                let call = builder.build_call(length_fn.into(), &[container.into()], "length_call");
                
                // Return the result
                call.try_as_basic_value().left().ok_or_else(|| Error::Compilation("Failed to get length return value".to_string()))
            }
        } else {
            // For other types, return an error
            Err(Error::Compilation(format!("Cannot get length of container with type: {:?}", container.get_type())))
        }
    }

    /// Helper function to emit code for getting element at index
    fn emit_container_get_element(
        &mut self,
        container: BasicValueEnum<'ctx>,
        index: IntValue<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        let builder = self.builder;
        
        // Handle different container types
        if container.is_array_value() {
            // For arrays, use GEP to access the element at the index
            let array_value = container.into_array_value();
            let array_type = array_value.get_type();
            let array_pointer = unsafe { 
                // Array pointers aren't directly accessible in inkwell, need to use a workaround
                let ptr_val = self.builder.build_alloca(array_type, "array_temp")?;
                self.builder.build_store(ptr_val, array_value)?;
                ptr_val
            };
            
            let element_ptr = unsafe {
                builder.build_in_bounds_gep(array_type, array_pointer, &[self.context.i32_type().const_zero(), index], "array_element_ptr")
            }?;
            
            // Load the element value
            let element_type = array_type.get_element_type();
            let element = builder.build_load(element_type, element_ptr, "array_element")?;
            Ok(element)
        } else if container.is_pointer_value() {
            // For pointers to containers (slices, arrays, etc.)
            let ptr_value = container.into_pointer_value();
            // For pointers in newer LLVM, we can't directly access element_type
            // Using determine_element_type instead
            let element_type = self.determine_element_type(container.into())?;
            
            // Get a pointer to the element at the index
            let element_ptr = unsafe {
                // For newer LLVM, use a different approach for GEP
                let ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
                builder.build_in_bounds_gep(ptr_type, ptr_value, &[index], "element_ptr")
            }?;
            
            // Load the element value
            let element = builder.build_load(element_type, element_ptr, "element")?;
            Ok(element)
        } else {
            // For other container types, we need to handle them specifically
            // or return an error
            Err(Error::Compilation(format!("Cannot get element from container of type: {:?}", container.get_type())))
        }
    }

    /// Helper function to determine container element type
    fn determine_element_type(&self, container: BasicValueEnum<'ctx>) -> Result<BasicTypeEnum<'ctx>, Error> {
        // Determine the element type based on the container type
        if container.is_array_value() {
            // For arrays, get the element type directly from the array type
            let array_value = container.into_array_value();
            Ok(array_value.get_type().get_element_type())
        } else if container.is_pointer_value() {
            // For pointers, check what they point to
            let ptr_value = container.into_pointer_value();
            let pointed_type = ptr_value.get_type().get_element_type();
            
            if pointed_type.is_array_type() {
                // Pointer to array - element type is the array element type
                Ok(pointed_type.into_array_type().get_element_type())
            } else if pointed_type.is_struct_type() {
                // For struct types (like slices), we need to determine the element type
                // based on the struct definition
                // This is a simplified approach - in a real implementation
                // you would need to handle different slice/container types differently
                
                // For slices, typically the element type is stored in the struct
                // or can be determined from its name/metadata
                let struct_type = pointed_type.into_struct_type();
                let type_name = struct_type.get_name().to_str().unwrap_or("");
                
                // Check if it's a slice type (simplified approach)
                if type_name.contains("slice") || type_name.contains("array") {
                    // For slices, we'll assume the element type is stored in the struct
                    // In a real implementation, you would extract this from the type system
                    
                    // For demonstration, we'll return a default type
                    // In practice, you would need to lookup the actual element type
                    Ok(self.context.i32_type().into())
                } else {
                    // For other struct types, we'll return an error
                    Err(Error::Compilation(format!("Cannot determine element type for struct: {}", type_name)))
                }
            } else {
                // For other pointer types, just return the pointed type as the element type
                Ok(pointed_type.into())
            }
        } else {
            // For other types, return an error
            Err(Error::Compilation(format!("Cannot determine element type for container: {:?}", container.get_type())))
        }
    }

    /// Helper function to create a map iterator
    fn emit_map_iterator_create(&mut self, map: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        let builder = self.builder;
        let context = self.context;
        
        // Validate map type
        if !map.is_pointer_value() {
            return Err(Error::Compilation("Expected map to be a pointer type".to_string()));
        }
        
        // For maps, we need to create an iterator object
        // This typically involves calling a runtime function that creates and initializes
        // an iterator from the map
        
        // First, create the function signature for the map_iterator_create function
        let map_ptr_type = map.into_pointer_value().get_type();
        let iter_ptr_type = context.i8_type().ptr_type(AddressSpace::default());
        let fn_type = iter_ptr_type.fn_type(&[map_ptr_type.into()], false);
        
        // Get or create the function
        let module = self.module.as_ref().unwrap();
        let create_fn = module.get_function("map_iterator_create").unwrap_or_else(|| {
            module.add_function("map_iterator_create", fn_type, None)
        });
        
        // Call the function to create the iterator
        let call = builder.build_call(create_fn.into(), &[map.into()], "create_map_iterator");
        
        // Return the result (the iterator pointer)
        call.try_as_basic_value().left().ok_or_else(|| Error::Compilation("Failed to create map iterator".to_string()))
    }

    /// Helper function to determine map key type
    fn determine_map_key_type(&self, map: BasicValueEnum<'ctx>) -> Result<BasicTypeEnum<'ctx>, Error> {
        // Validate map type
        if !map.is_pointer_value() {
            return Err(Error::Compilation("Expected map to be a pointer type".to_string()));
        }
        
        // Get the map type
        let ptr_type = map.into_pointer_value().get_type();
        let pointed_type = ptr_type.get_element_type();
        
        // Check if it's a struct type (maps are typically implemented as structs)
        if pointed_type.is_struct_type() {
            let struct_type = pointed_type.into_struct_type();
            let type_name = struct_type.get_name().to_str().unwrap_or("");
            
            // Check if it's a map type (based on naming convention)
            if type_name.contains("map") || type_name.contains("hash") {
                // For maps, the key type should be stored in the type system
                // In a real implementation, you would extract this from the type metadata
                
                // For demonstration, we'll return a string type (common for maps)
                // In practice, you'd look up the actual key type from the type system
                return Ok(self.context.i8_type().ptr_type(AddressSpace::default()).into());
            }
        }
        
        // If we can't determine the key type, return a default
        // In a real implementation, this would be an error case
        Ok(self.context.i8_type().ptr_type(AddressSpace::default()).into())
    }

    /// Helper function to determine map value type
    fn determine_map_value_type(&self, map: BasicValueEnum<'ctx>) -> Result<BasicTypeEnum<'ctx>, Error> {
        // Validate map type
        if !map.is_pointer_value() {
            return Err(Error::Compilation("Expected map to be a pointer type".to_string()));
        }
        
        // Get the map type
        let ptr_type = map.into_pointer_value().get_type();
        let pointed_type = ptr_type.get_element_type();
        
        // Check if it's a struct type (maps are typically implemented as structs)
        if pointed_type.is_struct_type() {
            let struct_type = pointed_type.into_struct_type();
            let type_name = struct_type.get_name().to_str().unwrap_or("");
            
            // Check if it's a map type (based on naming convention)
            if type_name.contains("map") || type_name.contains("hash") {
                // For maps, the value type should be stored in the type system
                // In a real implementation, you would extract this from the type metadata
                
                // For demonstration, we'll return an integer type (common for maps)
                // In practice, you'd look up the actual value type from the type system
                return Ok(self.context.i32_type().into());
            }
        }
        
        // If we can't determine the value type, return a default
        // In a real implementation, this would be an error case
        Ok(self.context.i32_type().into())
    }

    /// Helper function to check if map iterator has next element
    fn emit_iterator_has_next(&mut self, iterator: BasicValueEnum<'ctx>) -> Result<IntValue<'ctx>, Error> {
        let builder = self.builder;
        
        // Validate iterator type
        if !iterator.is_pointer_value() {
            return Err(Error::Compilation("Expected iterator to be a pointer type".to_string()));
        }
        
        // Create function signature for iterator_has_next
        let iter_ptr_type = iterator.get_type();
        let bool_type = self.context.bool_type();
        let fn_type = bool_type.fn_type(&[iter_ptr_type.into()], false);
        
        // Get or create the function
        let module = self.module.as_ref().unwrap();
        let has_next_fn = module.get_function("map_iterator_has_next").unwrap_or_else(|| {
            module.add_function("map_iterator_has_next", fn_type, None)
        });
        
        // Call the function to check if iterator has next
        let call = builder.build_call(has_next_fn.into(), &[iterator.into()], "iterator_has_next");
        
        // Get the result as a boolean value
        call.try_as_basic_value().left()
            .ok_or_else(|| Error::Compilation("Failed to get has_next result".to_string()))
            .map(|val| val.into_int_value())
    }

    /// Helper function to get key-value pair from iterator
    fn emit_iterator_get_key_value(
        &mut self,
        iterator: BasicValueEnum<'ctx>
    ) -> Result<(BasicValueEnum<'ctx>, BasicValueEnum<'ctx>), Error> {
        let builder = self.builder;
        
        // Validate iterator type
        if !iterator.is_pointer_value() {
            return Err(Error::Compilation("Expected iterator to be a pointer type".to_string()));
        }
        
        // Determine key and value types - in a real implementation
        // these would be determined from the iterator's type information
        let key_type = self.context.i8_type().ptr_type(AddressSpace::default());
        let value_type = self.context.i32_type();
        
        // Create memory for the key and value to be stored
        let key_ptr = builder.build_alloca(key_type, "key_ptr");
        let value_ptr = builder.build_alloca(value_type, "value_ptr");
        
        // Create function signature for get_key_value
        let iter_ptr_type = iterator.get_type();
        let void_type = self.context.void_type();
        let fn_type = void_type.fn_type(&[
            iter_ptr_type.into(),
            key_ptr.get_type().into(),
            value_ptr.get_type().into(),
        ], false);
        
        // Get or create the function
        let module = self.module.as_ref().unwrap();
        let get_kv_fn = module.get_function("map_iterator_get_key_value").unwrap_or_else(|| {
            module.add_function("map_iterator_get_key_value", fn_type, None)
        });
        
        // Call the function to get the key-value pair
        let unpacked_key_ptr = key_ptr?;
        let unpacked_value_ptr = value_ptr?;
        
        builder.build_call(
            get_kv_fn.into(),
            &[iterator.into(), unpacked_key_ptr.into(), unpacked_value_ptr.into()],
            "get_key_value"
        )?;
        
        // Load the key and value from the memory locations
        let key = builder.build_load(key_type, unpacked_key_ptr, "current_key")?;
        let value = builder.build_load(value_type, unpacked_value_ptr, "current_value")?;
        
        Ok((key, value))
    }

    /// Helper function to advance map iterator
    fn emit_iterator_next(&mut self, iterator: BasicValueEnum<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        let builder = self.builder;
        
        // Validate iterator type
        if !iterator.is_pointer_value() {
            return Err(Error::Compilation("Expected iterator to be a pointer type".to_string()));
        }
        
        // Create function signature for iterator_next
        let iter_ptr_type = iterator.get_type();
        let fn_type = iter_ptr_type.fn_type(&[iter_ptr_type.into()], false);
        
        // Get or create the function
        let module = self.module.as_ref().unwrap();
        let next_fn = module.get_function("map_iterator_next").unwrap_or_else(|| {
            module.add_function("map_iterator_next", fn_type, None)
        });
        
        // Call the function to advance the iterator
        let call = builder.build_call(next_fn.into(), &[iterator.into()], "iterator_next");
        
        // Return the updated iterator
        call.try_as_basic_value().left()
            .ok_or_else(|| Error::Compilation("Failed to advance iterator".to_string()))
    }
}