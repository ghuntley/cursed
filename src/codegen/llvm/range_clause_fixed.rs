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
use inkwell::basic_block::BasicBlock;
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::{BasicValue, BasicValueEnum, IntValue, PointerValue};
use inkwell::IntPredicate;
use tracing::{debug, info, instrument};
use inkwell::AddressSpace;
use crate::error::Error;
use std::convert::From;

/// Enhanced trait for implementing range-based iteration in for loops with proper error handling
///
/// This trait provides methods to compile different types of range clauses:
/// - Numeric ranges with optional start, end, and step values
/// - Container iteration (arrays, slices)
/// - Key-value iteration for maps
/// 
/// Improved version with consistent error handling and proper LLVM builder operations
pub trait RangeClauseCompilationEnhanced<'ctx> {
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

impl<'ctx> RangeClauseCompilationEnhanced<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self, body, range_expr), fields(iterator = iterator_name))]
    fn compile_range_for_loop(
        &mut self,
        iterator_name: &str,
        range_expr: &RangeExpression,
        body: &BlockStatement,
    ) -> Result<(), Error> {
        debug!("Compiling range for loop with iterator: {}", iterator_name);
        
        // Get the current function or return an error if not in a function
        let func = self.current_function()
            .ok_or_else(|| Error::Compilation("No current function".to_string()))?;
            
        let context = self.context;
        let builder = &self.builder;

        // Create basic blocks for the loop
        let loop_entry = context.append_basic_block(func, "range.for.entry");
        let loop_body = context.append_basic_block(func, "range.for.body");
        let loop_increment = context.append_basic_block(func, "range.for.increment");
        let loop_exit = context.append_basic_block(func, "range.for.exit");

        // Compile range expression components based on the type of range
        let (start_value, end_value, step_value) = match range_expr {
            RangeExpression::Range { end } => {
                // Basic form: for i := range end
                let end_val = self.compile_expression(end.as_ref())?
                    .into_int_value();
                let start_val = context.i64_type().const_int(0, false); // Default start: 0
                let step_val = context.i64_type().const_int(1, false);  // Default step: 1
                (start_val, end_val, step_val)
            },
            RangeExpression::RangeFromTo { start, end } => {
                // Two-argument form: for i := range start, end
                let start_val = self.compile_expression(start.as_ref())?
                    .into_int_value();
                let end_val = self.compile_expression(end.as_ref())?
                    .into_int_value();
                let step_val = context.i64_type().const_int(1, false);  // Default step: 1
                (start_val, end_val, step_val)
            },
            RangeExpression::RangeFromToStep { start, end, step } => {
                // Complete form: for i := range start, end, step
                let start_val = self.compile_expression(start.as_ref())?
                    .into_int_value();
                let end_val = self.compile_expression(end.as_ref())?
                    .into_int_value();
                let step_val = self.compile_expression(step.as_ref())?
                    .into_int_value();
                (start_val, end_val, step_val)
            },
        };

        // Allocate loop variable and initialize with start value
        let i_ptr = builder.build_alloca(context.i64_type(), iterator_name)?;
        builder.build_store(i_ptr, start_value)?;

        // Determine if we're counting up or down based on step sign
        let zero = context.i64_type().const_zero();
        let step_is_positive = builder.build_int_compare(
            IntPredicate::SGT,
            step_value,
            zero,
            "step.positive"
        )?;

        // Jump to loop entry
        builder.build_unconditional_branch(loop_entry)?;

        // Loop entry: check condition
        builder.position_at_end(loop_entry);
        let current_value = builder.build_load(context.i64_type(), i_ptr, "current")?
            .into_int_value();

        // Create the loop condition based on the step direction
        let up_condition = builder.build_int_compare(
            IntPredicate::SLT,
            current_value,
            end_value,
            "loop.condition.up"
        )?;
        
        let down_condition = builder.build_int_compare(
            IntPredicate::SGT,
            current_value,
            end_value,
            "loop.condition.down"
        )?;
        
        // Select the appropriate condition based on step direction
        // We need to ensure condition is an IntValue for the conditional branch
        let condition_value = builder.build_select(
            step_is_positive,
            up_condition,
            down_condition,
            "loop.condition"
        )?;

        // Cast to IntValue for conditional branch
        let condition = condition_value.into_int_value();
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
            builder.build_unconditional_branch(loop_increment)?;
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
        // Now incremented is a Result<IntValue> - extract it before passing to build_store
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
        
        // Get the current function or return an error if not in a function
        let func = self.current_function()
            .ok_or_else(|| Error::Compilation("No current function".to_string()))?;
            
        let context = self.context;
        let builder = &self.builder;

        // Evaluate the container expression
        let container_value = self.compile_expression(container_expr.as_ref())?;

        // Create basic blocks for the loop
        let loop_setup = context.append_basic_block(func, "container.for.setup");
        let loop_entry = context.append_basic_block(func, "container.for.entry");
        let loop_body = context.append_basic_block(func, "container.for.body");
        let loop_increment = context.append_basic_block(func, "container.for.increment");
        let loop_exit = context.append_basic_block(func, "container.for.exit");

        // Get the container type and setup iteration
        builder.build_unconditional_branch(loop_setup)?;
        builder.position_at_end(loop_setup);

        // Create an index variable for iteration
        let i_ptr = builder.build_alloca(context.i32_type(), "index")?;
        builder.build_store(i_ptr, context.i32_type().const_zero())?;

        // Determine container length (using a helper method)
        let length_value = self.emit_container_length_fixed(container_value)?;

        // Allocate memory for the current element
        let element_type = self.determine_element_type_fixed(container_value)?;
        let value_ptr = builder.build_alloca(element_type, value_name)?;

        builder.build_unconditional_branch(loop_entry)?;

        // Loop entry: check if index < length
        builder.position_at_end(loop_entry);
        let current_index = builder.build_load(context.i32_type(), i_ptr, "current.index")?
            .into_int_value();
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
        let current_element = self.emit_get_element_fixed(container_value, current_index)?;
        builder.build_store(value_ptr, current_element)?;

        // Push a new scope for the loop body
        self.push_scope(super::variables::VariableScope::new());

        // Add element variable to current scope
        self.add_variable(value_name, value_ptr)?;

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
            builder.build_unconditional_branch(loop_increment)?;
        }

        // Loop increment
        builder.position_at_end(loop_increment);
        let current_index = builder.build_load(context.i32_type(), i_ptr, "current.index.inc")?
            .into_int_value();
        let incremented = builder.build_int_add(
            current_index,
            context.i32_type().const_int(1, false),
            "incremented.index"
        )?;
        // Now incremented is a Result<IntValue> - extract it before passing to build_store
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
        
        // Get the current function or return an error if not in a function
        let func = self.current_function()
            .ok_or_else(|| Error::Compilation("No current function".to_string()))?;
            
        let context = self.context;
        let builder = &self.builder;

        // Evaluate the map expression
        let map_value = self.compile_expression(map_expr.as_ref())?;

        // Create basic blocks for the loop
        let loop_setup = context.append_basic_block(func, "map.for.setup");
        let loop_entry = context.append_basic_block(func, "map.for.entry");
        let loop_body = context.append_basic_block(func, "map.for.body");
        let loop_increment = context.append_basic_block(func, "map.for.increment");
        let loop_exit = context.append_basic_block(func, "map.for.exit");

        // Set up iteration
        builder.build_unconditional_branch(loop_setup)?;
        builder.position_at_end(loop_setup);

        // Create map iterator (using a helper method)
        let iterator_ptr = self.emit_map_iterator_create_fixed(map_value)?;
        
        // Determine key and value types
        let key_type = self.determine_map_key_type_fixed(map_value)?;
        let value_type = self.determine_map_value_type_fixed(map_value)?;
        
        // Allocate memory for current key and value
        let key_ptr = builder.build_alloca(key_type, key_name)?;
        let value_ptr = builder.build_alloca(value_type, value_name)?;

        builder.build_unconditional_branch(loop_entry)?;

        // Loop entry: check if iterator has next element
        builder.position_at_end(loop_entry);
        let has_next = self.emit_map_iterator_has_next_fixed(iterator_ptr)?;
        builder.build_conditional_branch(has_next, loop_body, loop_exit)?;

        // Loop body: get current key-value pair and execute body
        builder.position_at_end(loop_body);

        // Get current key-value pair (using a helper method)
        self.emit_map_iterator_get_current_fixed(iterator_ptr, key_ptr, value_ptr)?;

        // Push a new scope for the loop body
        self.push_scope(super::variables::VariableScope::new());

        // Add key and value variables to current scope
        self.add_variable(key_name, key_ptr)?;
        self.add_variable(value_name, value_ptr)?;

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
            builder.build_unconditional_branch(loop_increment)?;
        }

        // Loop increment: advance the iterator
        builder.position_at_end(loop_increment);
        self.emit_map_iterator_next_fixed(iterator_ptr)?;
        builder.build_unconditional_branch(loop_entry)?;

        // Loop exit
        builder.position_at_end(loop_exit);

        Ok(())
    }
}

// Helper methods for LlvmCodeGenerator to handle container and map iteration
// Implementation helpers for the fixed range clause trait
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Get the length of a container
    /// Get container length (array size, slice length, etc.)
    /// 
    /// This method handles different container types and extracts their length
    /// appropriately. For arrays, it uses the type's length property, while for
    /// other containers, it would access length fields or call length methods.
    fn emit_container_length_fixed(&self, container: BasicValueEnum<'ctx>) -> Result<IntValue<'ctx>, Error> {
        if container.is_array_value() {
            // For array values, get length from the type
            let array_value = container.into_array_value();
            let length = array_value.get_type().len();
            Ok(self.context.i32_type().const_int(length as u64, false))
        } else if container.is_pointer_value() {
            // For pointer types (slices, etc.), get from fields or call a helper
            let ptr_value = container.into_pointer_value();
            // Get the element type from pointer's pointee type
            let ptr_type = ptr_value.get_type();
            let element_ty = ptr_type.get_pointed_type(); // This gets the type the pointer points to
            
            if element_ty.is_array_type() {
                // Pointer to an array - get array length
                let array_ty = element_ty.into_array_type();
                let length = array_ty.len();
                Ok(self.context.i32_type().const_int(length as u64, false))
            } else {
                // For non-array containers, call a container.len() method
                // This would need to be implemented for each container type
                Err(Error::CodeGenError("Cannot determine length of container - method not implemented".to_string()))
            }
        } else {
            Err(Error::CodeGenError(format!("Cannot determine length of container with type: {:?}", container.get_type())))
        }
    }
    
    /// Get an element from a container at the given index
    fn emit_get_element_fixed(&self, container: BasicValueEnum<'ctx>, index: IntValue<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        if container.is_array_value() {
            // For arrays, get element at index
            let array_value = container.into_array_value();
            let indices = &[self.context.i32_type().const_zero(), index];
            let element_ptr = unsafe {
                // Note: GEP operations are unsafe, so we ensure index is in bounds before using
                // Convert array to pointer first for GEP operation
                let array_ptr = unsafe { self.builder.build_alloca(array_value.get_type(), "array_temp") }?;
                unsafe { self.builder.build_store(array_ptr, array_value)? };
                self.builder.build_gep(array_value.get_type(), array_ptr, indices, "element_ptr")?
            };
            
            let element_type = array_value.get_type().get_element_type();
            Ok(self.builder.build_load(element_type, element_ptr, "element")?.into())
        } else if container.is_pointer_value() {
            // For pointers to arrays, calculate the element pointer
            let ptr_value = container.into_pointer_value();
            let indices = &[index];
            let element_ptr = unsafe {
                // Note: GEP operations are unsafe, so we ensure index is in bounds before using
                // Get the element type that pointer points to
                let pointee_type = ptr_value.get_type().get_pointed_type();
                self.builder.build_gep(pointee_type, ptr_value, indices, "element_ptr")?
            };
            
            let element_type = self.determine_element_type_fixed(container)?;
            Ok(self.builder.build_load(element_type, element_ptr, "element")?.into())
        } else {
            Err(Error::CodeGenError(format!("Cannot get element from container with type: {:?}", container.get_type())))
        }
    }
    
    /// Determine the element type of a container
    fn determine_element_type_fixed(&self, container: BasicValueEnum<'ctx>) -> Result<BasicTypeEnum<'ctx>, Error> {
        if container.is_array_value() {
            // For arrays, get element type directly
            let array_value = container.into_array_value();
            Ok(array_value.get_type().get_element_type())
        } else if container.is_pointer_value() {
            // For pointers to arrays or other containers
            let ptr_type = container.into_pointer_value().get_type();
            // Use pointer's pointee type to identify the container type
            let pointee_type = ptr_type.get_pointed_type();
            
            if pointee_type.is_array_type() {
                // Pointer to array - element type is array element type
                let array_type = pointee_type.into_array_type();
                Ok(array_type.get_element_type())
            } else if pointee_type.is_struct_type() {
                // For container structs like slices or strings
                // Would need to retrieve element type from container metadata
                Err(Error::CodeGenError("Container element type resolution for structs not implemented".to_string()))
            } else {
                // For pointer to a non-container type, the element type is what it points to
                Ok(pointee_type)
            }
        } else {
            Err(Error::CodeGenError(format!("Cannot determine element type for container: {:?}", container.get_type())))
        }
    }
    
    /// Create an iterator for a map
    fn emit_map_iterator_create_fixed(&self, map_value: BasicValueEnum<'ctx>) -> Result<PointerValue<'ctx>, Error> {
        // This would call into runtime library functions for map iteration
        // A simplified implementation that returns a pointer to a map iterator object
        if !map_value.is_pointer_value() {
            return Err(Error::CodeGenError("Expected map to be a pointer type".to_string()));
        }
        
        // For an actual implementation, we would get the module directly
        // let module = &self.module;
        // Then we would look up or create the map iterator creation function
        // Then call it and return the result
        
        // For now, return an error since this isn't implemented yet
        Err(Error::CodeGenError("Map iterator creation not implemented".to_string()))
    }
    
    /// Check if a map iterator has more elements
    fn emit_map_iterator_has_next_fixed(&self, iterator_ptr: PointerValue<'ctx>) -> Result<IntValue<'ctx>, Error> {
        // This would call into runtime library functions to check if iterator has more elements
        // A simplified implementation that returns a boolean (int1) value
        
        // For an actual implementation, we would:
        // 1. Get a reference to the module
        // let module = &self.module;
        // 2. Look up or create a function for has_next
        // 3. Call that function with the iterator pointer
        // 4. Return a boolean value
        
        // For now, return an error since this isn't implemented yet
        Err(Error::CodeGenError("Map iterator has_next check not implemented".to_string()))
    }
    
    /// Get current key-value pair from map iterator and advance
    fn emit_map_iterator_get_current_fixed(
        &self,
        iterator_ptr: PointerValue<'ctx>,
        key_ptr: PointerValue<'ctx>,
        value_ptr: PointerValue<'ctx>
    ) -> Result<(), Error> {
        // This would call into runtime library functions to get current key-value pair
        // For actual implementation, would store current key/value into the provided pointers
        Err(Error::CodeGenError("Map iterator get_current not implemented".to_string()))
    }
    
    /// Advance a map iterator to the next element
    fn emit_map_iterator_next_fixed(&self, iterator_ptr: PointerValue<'ctx>) -> Result<(), Error> {
        // This would call into runtime library functions to advance the iterator
        Err(Error::CodeGenError("Map iterator advancement not implemented".to_string()))
    }
    
    /// Determine the key type for a map
    fn determine_map_key_type_fixed(&self, map_value: BasicValueEnum<'ctx>) -> Result<BasicTypeEnum<'ctx>, Error> {
        // This would need to extract key type information from the map type
        // For actual implementation, would look at map type and return appropriate key type
        if !map_value.is_pointer_value() {
            return Err(Error::CodeGenError("Expected map to be a pointer type".to_string()));
        }
        
        // For now, default to i8* (string type) as common map key type
        Ok(self.context.i8_type().ptr_type(AddressSpace::default()).into())
    }
    
    /// Determine the value type for a map
    fn determine_map_value_type_fixed(&self, map_value: BasicValueEnum<'ctx>) -> Result<BasicTypeEnum<'ctx>, Error> {
        // This would need to extract value type information from the map type
        // For actual implementation, would look at map type and return appropriate value type
        if !map_value.is_pointer_value() {
            return Err(Error::CodeGenError("Expected map to be a pointer type".to_string()));
        }
        
        // For now, default to i64 (int type) as common map value type
        Ok(self.context.i64_type().into())
    }
}