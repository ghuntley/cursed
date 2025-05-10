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
        
        // First evaluate all the expressions before borrowing self.builder
        // This eliminates borrow checker conflicts
        let (start_value, end_value, step_value) = match range_expr {
            RangeExpression::Range { end } => {
                // Basic form: for i := range end
                let end_val = self.compile_expression(end.as_ref())?
                    .into_int_value();
                let context = self.context;
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
                let context = self.context;
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
        
        // Get the current function or return an error if not in a function
        let func = self.current_function()
            .ok_or_else(|| Error::Compilation("No current function".to_string()))?;
            
        let context = self.context;
        // Use a mutable reference pattern where we modify and restore self.builder
        // For capturing current state
        let insert_block_before = self.builder.get_insert_block();
        
        // Create basic blocks for the loop
        let loop_entry = context.append_basic_block(func, "range.for.entry");
        let loop_body = context.append_basic_block(func, "range.for.body");
        let loop_increment = context.append_basic_block(func, "range.for.increment");
        let loop_exit = context.append_basic_block(func, "range.for.exit");

        // Allocate loop variable and initialize with start value
        let i_ptr = self.builder.build_alloca(context.i64_type(), iterator_name)?;
        self.builder.build_store(i_ptr, start_value)?;

        // Determine if we're counting up or down based on step sign
        let zero = context.i64_type().const_zero();
        let step_is_positive = self.builder.build_int_compare(
            IntPredicate::SGT,
            step_value,
            zero,
            "step.positive"
        )?;

        // Jump to loop entry
        self.builder.build_unconditional_branch(loop_entry)?;

        // Loop entry: check condition
        self.builder.position_at_end(loop_entry);
        let current_value = self.builder.build_load(context.i64_type(), i_ptr, "current")?
            .into_int_value();

        // Create the loop condition based on the step direction
        let up_condition = self.builder.build_int_compare(
            IntPredicate::SLT,
            current_value,
            end_value,
            "loop.condition.up"
        )?;
        
        let down_condition = self.builder.build_int_compare(
            IntPredicate::SGT,
            current_value,
            end_value,
            "loop.condition.down"
        )?;
        
        // Select the appropriate condition based on step direction
        // We need to ensure condition is an IntValue for the conditional branch
        let condition_value = self.builder.build_select(
            step_is_positive,
            up_condition,
            down_condition,
            "loop.condition"
        )?;

        // Cast to IntValue for conditional branch
        let condition = condition_value.into_int_value();
        self.builder.build_conditional_branch(condition, loop_body, loop_exit)?;

        // Loop body 
        self.builder.position_at_end(loop_body);

        // Create a new scope for the loop variable
        self.push_scope(super::variables::VariableScope::new());
        self.add_variable(iterator_name, i_ptr)?;
        
        // Set up loop context for break/continue
        let old_loop_exit = self.replace_loop_exit(Some(loop_exit));
        let old_loop_continue = self.replace_loop_continue(Some(loop_increment));
        
        // Compile the loop body statement
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
        let current_value = self.builder.build_load(context.i64_type(), i_ptr, "current.inc")?
            .into_int_value();
        let incremented = self.builder.build_int_add(
            current_value,
            step_value,
            "incremented"
        )?;
        self.builder.build_store(i_ptr, incremented)?;
        self.builder.build_unconditional_branch(loop_entry)?;

        // Loop exit block
        self.builder.position_at_end(loop_exit);

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
        
        // Evaluate container expressions first before borrowing self.builder
        // This eliminates borrow checker conflicts
        let container_value = self.compile_expression(container_expr.as_ref())?;
        
        // Get the current function or return an error if not in a function
        let func = self.current_function()
            .ok_or_else(|| Error::Compilation("No current function".to_string()))?;
            
        let context = self.context;
        // For capturing current state if needed
        let insert_block_before = self.builder.get_insert_block();

        // Create basic blocks for the loop
        let loop_setup = context.append_basic_block(func, "container.for.setup");
        let loop_entry = context.append_basic_block(func, "container.for.entry");
        let loop_body = context.append_basic_block(func, "container.for.body");
        let loop_increment = context.append_basic_block(func, "container.for.increment");
        let loop_exit = context.append_basic_block(func, "container.for.exit");

        // Get the container type and setup iteration
        self.builder.build_unconditional_branch(loop_setup)?;
        self.builder.position_at_end(loop_setup);

        // Create an index variable for iteration
        let i_ptr = self.builder.build_alloca(context.i32_type(), "index")?;
        self.builder.build_store(i_ptr, context.i32_type().const_zero())?;

        // Determine container length (using a helper method)
        let length_value = self.emit_container_length_fixed(container_value)?;

        // Allocate memory for the current element
        let element_type = self.determine_element_type_fixed(container_value)?;
        let value_ptr = self.builder.build_alloca(element_type, value_name)?;

        self.builder.build_unconditional_branch(loop_entry)?;

        // Loop entry: check if index < length
        self.builder.position_at_end(loop_entry);
        let current_index = self.builder.build_load(context.i32_type(), i_ptr, "current.index")?
            .into_int_value();
        let condition = self.builder.build_int_compare(
            IntPredicate::SLT,
            current_index,
            length_value,
            "loop.condition"
        )?;
        self.builder.build_conditional_branch(condition, loop_body, loop_exit)?;

        // Loop body: get current element and execute body
        self.builder.position_at_end(loop_body);

        // Get the current element from the container
        let current_element = self.emit_get_element_fixed(container_value, current_index)?;
        self.builder.build_store(value_ptr, current_element)?;

        // Create a new scope for the loop variable
        self.push_scope(super::variables::VariableScope::new());
        self.add_variable(value_name, value_ptr)?;
        
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
        
        // Evaluate map expression first before borrowing self.builder
        // This eliminates borrow checker conflicts
        let map_value = self.compile_expression(map_expr.as_ref())?;
        
        // Get the current function or return an error if not in a function
        let func = self.current_function()
            .ok_or_else(|| Error::Compilation("No current function".to_string()))?;
            
        let context = self.context;
        // For capturing current state if needed
        let insert_block_before = self.builder.get_insert_block();

        // Create basic blocks for the loop
        let loop_setup = context.append_basic_block(func, "map.for.setup");
        let loop_entry = context.append_basic_block(func, "map.for.entry");
        let loop_body = context.append_basic_block(func, "map.for.body");
        let loop_increment = context.append_basic_block(func, "map.for.increment");
        let loop_exit = context.append_basic_block(func, "map.for.exit");

        // Set up iteration
        self.builder.build_unconditional_branch(loop_setup)?;
        self.builder.position_at_end(loop_setup);

        // Create map iterator (using a helper method)
        let iterator_ptr = self.emit_map_iterator_create_fixed(map_value)?;
        
        // Determine key and value types
        let key_type = self.determine_map_key_type_fixed(map_value)?;
        let value_type = self.determine_map_value_type_fixed(map_value)?;
        
        // Allocate memory for current key and value
        let key_ptr = self.builder.build_alloca(key_type, key_name)?;
        let value_ptr = self.builder.build_alloca(value_type, value_name)?;

        self.builder.build_unconditional_branch(loop_entry)?;

        // Loop entry: check if iterator has next element
        self.builder.position_at_end(loop_entry);
        let has_next = self.emit_map_iterator_has_next_fixed(iterator_ptr)?;
        self.builder.build_conditional_branch(has_next, loop_body, loop_exit)?;

        // Loop body: get current key-value pair and execute body
        self.builder.position_at_end(loop_body);

        // Get current key-value pair (using a helper method)
        self.emit_map_iterator_get_current_fixed(iterator_ptr, key_ptr, value_ptr)?;

        // Create a new scope for the loop variables
        self.push_scope(super::variables::VariableScope::new());
        self.add_variable(key_name, key_ptr)?;
        self.add_variable(value_name, value_ptr)?;
        
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
        self.emit_map_iterator_next_fixed(iterator_ptr)?;
        self.builder.build_unconditional_branch(loop_entry)?;

        // Loop exit block
        self.builder.position_at_end(loop_exit);

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
    /// other containers, it accesses length fields or calls length methods.
    #[instrument(skip(self, container), level = "debug")]
    fn emit_container_length_fixed(&self, container: BasicValueEnum<'ctx>) -> Result<IntValue<'ctx>, Error> {
        if container.is_array_value() {
            // For array values, get length from the type
            let array_value = container.into_array_value();
            let length = array_value.get_type().len();
            debug!("Array length: {}", length);
            Ok(self.context.i32_type().const_int(length as u64, false))
        } else if container.is_pointer_value() {
            // For pointer types (slices, etc.), we need to handle different pointer types
            let ptr_value = container.into_pointer_value();
            let ptr_type = ptr_value.get_type();
            let element_type = ptr_type.get_pointee_type();
            
            if let Some(array_type) = element_type.into_array_type() {
                // Pointer to an array
                let length = array_type.len();
                debug!("Pointer to array length: {}", length);
                Ok(self.context.i32_type().const_int(length as u64, false))
            } else if element_type.is_struct_type() {
                // For slice-like types which are structs with a length field
                // First, try to load the struct value
                let struct_ptr = ptr_value;
                let struct_value = self.builder.build_load(element_type, struct_ptr, "container_struct")?;
                
                // Create or get the length function
                let module = self.module.clone().ok_or_else(|| {
                    Error::Compilation("Module not available".to_string())
                })?;
                
                // Try to find a length getter function
                let type_name = if let Some(struct_type) = element_type.into_struct_type() {
                    struct_type.get_name().to_str().unwrap_or("container").to_string()
                } else {
                    "container".to_string()
                };
                
                // Try different naming conventions for the length function
                let fn_names = vec![
                    format!("{}_length", type_name),
                    format!("{}_get_length", type_name),
                    "container_length".to_string(),
                ];
                
                // Find the first matching function or create a default one
                let length_fn = fn_names.iter()
                    .find_map(|name| module.get_function(name))
                    .unwrap_or_else(|| {
                        // Create a default length function
                        let fn_type = self.context.i32_type().fn_type(&[container.get_type().into()], false);
                        module.add_function("container_length", fn_type, None)
                    });
                
                // Call the length function
                let call = self.builder.build_call(length_fn, &[container.into()], "length_call")?;
                
                // Get the return value
                call.try_as_basic_value().left()
                    .and_then(|val| val.into_int_value().into())
                    .ok_or_else(|| Error::Compilation("Failed to get length return value".to_string()))
            } else {
                // For simple pointer to element types like strings
                // Use a runtime helper function to get length
                let module = self.module.clone().ok_or_else(|| {
                    Error::Compilation("Module not available".to_string())
                })?;
                
                // Get or create a generic container length function
                let fn_type = self.context.i32_type().fn_type(&[container.get_type().into()], false);
                let length_fn = module.get_function("container_length").unwrap_or_else(|| {
                    module.add_function("container_length", fn_type, None)
                });
                
                // Call the function
                let call = self.builder.build_call(length_fn, &[container.into()], "length_call")?;
                
                // Get the return value
                call.try_as_basic_value().left()
                    .and_then(|val| val.into_int_value().into())
                    .ok_or_else(|| Error::Compilation("Failed to get length return value".to_string()))
            }
        } else {
            Err(Error::CodeGenError(format!("Cannot determine length of container with type: {:?}", container.get_type())))
        }
    }
    
    /// Get an element from a container at the given index
    /// 
    /// This method retrieves elements from different container types:
    /// - Arrays: Direct indexing with proper LLVM IR generation
    /// - Pointers to arrays: Uses proper GEP instructions
    /// - Slices and other container types: Handles through appropriate pointer arithmetic
    #[instrument(skip(self, container, index), level = "debug")]
    fn emit_get_element_fixed(&self, container: BasicValueEnum<'ctx>, index: IntValue<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
        if container.is_array_value() {
            // For arrays, get element at index
            let array_value = container.into_array_value();
            let indices = &[self.context.i32_type().const_zero(), index];
            
            // First allocate space for the array to work with pointers
            let array_ptr = self.builder.build_alloca(array_value.get_type(), "array_temp")?;
            self.builder.build_store(array_ptr, array_value)?;
            
            // Then use GEP to get element pointer
            let element_ptr = unsafe {
                // Note: GEP operations are unsafe, but we've checked index bounds earlier
                self.builder.build_gep(array_value.get_type(), array_ptr, indices, "element_ptr")?
            };
            
            let element_type = array_value.get_type().get_element_type();
            debug!("Loading array element at index");
            Ok(self.builder.build_load(element_type, element_ptr, "element")?.into())
        } else if container.is_pointer_value() {
            let ptr_value = container.into_pointer_value();
            let ptr_type = ptr_value.get_type();
            let pointee_type = ptr_type.get_element_type();
            
            if pointee_type.is_array_type() {
                // Pointer to an array
                let array_type = pointee_type.into_array_type().unwrap();
                let element_type = array_type.get_element_type();
                
                // First, load the array
                let array_value = self.builder.build_load(array_type, ptr_value, "array_value")?;
                
                // Then, get the element
                let indices = &[self.context.i32_type().const_zero(), index];
                let element_ptr = unsafe {
                    // Create a temporary to work with the loaded array
                    let temp_array_ptr = self.builder.build_alloca(array_type, "temp_array")?;
                    self.builder.build_store(temp_array_ptr, array_value)?;
                    self.builder.build_gep(array_type, temp_array_ptr, indices, "element_ptr")?
                };
                
                debug!("Loading array element from pointer to array");
                Ok(self.builder.build_load(element_type, element_ptr, "element")?.into())
            } else if pointee_type.is_struct_type() {
                // This is likely a slice or similar container
                // For slices, we need to extract the data pointer and then index it
                
                // Load the struct
                let struct_value = self.builder.build_load(pointee_type, ptr_value, "struct_value")?;
                
                // For slice types, the first field is typically the data pointer
                // This approach assumes a specific memory layout for slice types
                let data_ptr_value = if let Some(struct_type) = pointee_type.into_struct_type() {
                    // Check if we can extract field information
                    if struct_type.get_field_types().len() > 0 {
                        let struct_ptr = ptr_value;
                        let indices = &[self.context.i32_type().const_zero(), self.context.i32_type().const_zero()];
                        
                        // Get pointer to the first field (data pointer)
                        let data_ptr_ptr = unsafe {
                            self.builder.build_gep(struct_type, struct_ptr, indices, "data_ptr_ptr")?
                        };
                        
                        // Load the data pointer itself
                        let data_ptr_type = struct_type.get_field_types()[0];
                        debug!("Loading data pointer from struct field");
                        self.builder.build_load(data_ptr_type, data_ptr_ptr, "data_ptr")?
                    } else {
                        // Fallback approach for unknown struct layouts
                        debug!("Using fallback for unknown struct layout");
                        let module = self.module.clone().ok_or_else(|| {
                            Error::Compilation("Module not available".to_string())
                        })?;
                        
                        // Try to find or create a get_element function for this container type
                        let type_name = struct_type.get_name().to_str().unwrap_or("container");
                        let fn_name = format!("{}_get_element", type_name);
                        let fn_type = pointee_type.fn_type(&[ptr_value.get_type().into(), index.get_type().into()], false);
                        
                        let get_elem_fn = module.get_function(&fn_name).unwrap_or_else(|| {
                            module.add_function(&fn_name, fn_type, None)
                        });
                        
                        // Call the function to get the element
                        debug!("Calling get_element function for struct type");
                        let call = self.builder.build_call(get_elem_fn, &[ptr_value.into(), index.into()], "get_elem_call")?;
                        return call.try_as_basic_value().left().ok_or_else(|| {
                            Error::Compilation("Failed to get element from container".to_string())
                        });
                    }
                } else {
                    // Should never happen since we already checked it's a struct type
                    return Err(Error::CodeGenError("Inconsistent struct type handling".to_string()));
                };
                
                // Now that we have the data pointer, index into it
                let indices = &[index];
                let element_ptr = unsafe {
                    // Extract element type from the data pointer value
                    let element_ptr_type = data_ptr_value.get_type();
                    self.builder.build_gep(element_ptr_type, data_ptr_value.into_pointer_value(), indices, "element_ptr")?
                };
                
                // Determine the element type from the data pointer
                let element_type = self.determine_element_type_fixed(data_ptr_value)?;
                debug!("Loading element from slice using data pointer");
                Ok(self.builder.build_load(element_type, element_ptr, "element")?.into())
            } else {
                // Direct pointer to elements (like a C array)
                let indices = &[index];
                let element_ptr = unsafe {
                    self.builder.build_gep(pointee_type, ptr_value, indices, "element_ptr")?
                };
                
                // Determine element type (should be what the pointer points to)
                debug!("Loading element from direct pointer indexing");
                Ok(self.builder.build_load(pointee_type, element_ptr, "element")?.into())
            }
        } else {
            Err(Error::CodeGenError(format!("Cannot get element from container with type: {:?}", container.get_type())))
        }
    }
    
    /// Determine the element type of a container
    /// 
    /// This method extracts the element type for different container types:
    /// - For arrays: returns the array's element type
    /// - For pointers to arrays: returns the array's element type
    /// - For pointers to slices: extracts the element type from slice structure
    /// - For other containers: attempts to determine element type from context
    #[instrument(skip(self, container), level = "debug")]
    fn determine_element_type_fixed(&self, container: BasicValueEnum<'ctx>) -> Result<BasicTypeEnum<'ctx>, Error> {
        if container.is_array_value() {
            // For arrays, get element type directly
            let array_value = container.into_array_value();
            let element_type = array_value.get_type().get_element_type();
            debug!("Array element type extracted");
            Ok(element_type)
        } else if container.is_pointer_value() {
            // For pointers to arrays or other containers
            let ptr_value = container.into_pointer_value();
            let ptr_type = ptr_value.get_type();
            let pointee_type = ptr_type.get_element_type();
            
            if let Some(array_type) = pointee_type.into_array_type() {
                // Pointer to an array - get the array's element type
                let element_type = array_type.get_element_type();
                debug!("Pointer to array element type extracted");
                Ok(element_type)
            } else if pointee_type.is_struct_type() {
                // This is likely a slice or other structured container
                // For slices, we need to extract the element type from the slice's definition
                // In a proper implementation, we would access type metadata or use reflection
                
                // Try to get struct type info
                if let Some(struct_type) = pointee_type.into_struct_type() {
                    let type_name = struct_type.get_name().to_str().unwrap_or("unknown");
                    debug!("Extracting element type from struct: {}", type_name);
                    
                    // For a slice type, the element type is usually available as part of the type information
                    // In a real implementation, we would have a type registry to look up this information
                    
                    // Check if the struct name contains type information
                    // Common pattern: Slice<T> or similar
                    if type_name.contains("Slice") || type_name.contains("Array") {
                        // Try to extract element type from field 0 (data pointer)
                        if struct_type.get_field_types().len() > 0 {
                            let data_field_type = struct_type.get_field_types()[0];
                            if data_field_type.is_pointer_type() {
                                // Element type should be what the data pointer points to
                                let element_type = data_field_type.into_pointer_type().get_element_type();
                                debug!("Extracted element type from slice structure");
                                return Ok(element_type);
                            }
                        }
                    }
                    
                    // Fallback: use i64 if we can't determine the actual type
                    debug!("Using fallback i64 type for struct container");
                    return Ok(self.context.i64_type().into());
                } else {
                    // Fallback for non-struct types
                    debug!("Using fallback i64 type for non-struct container");
                    return Ok(self.context.i64_type().into());
                }
            } else if pointee_type.is_pointer_type() {
                // This might be a string-like type (pointer to elements)
                let element_ptr_type = pointee_type.into_pointer_type();
                let element_type = element_ptr_type.get_element_type();
                debug!("Pointer to pointer element type extracted");
                Ok(element_type)
            } else {
                // Direct element type for other pointer types
                debug!("Direct pointee type used as element type");
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