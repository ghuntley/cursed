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

use crate::ast::Expression;
use crate::ast::statements::BlockStatement;
use crate::ast::expressions::RangeExpression;
use crate::codegen::llvm::context::LlvmCodeGenerator;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::codegen::llvm::variables::VariableHandling;
use crate::codegen::llvm::statement::StatementCompilation;
use crate::codegen::llvm::pointer_type_extension::{PointerTypeExtension, BasicTypeEnumExtension};
use crate::codegen::llvm::range_clause_fixed_extension::RangeClauseFixedMethodsExtension;
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
        self.add_variable_with_type(iterator_name, i_ptr, self.context().i32_type().into())?;
        
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
        
        // Ensure all required runtime functions are available
        self.ensure_runtime_container_functions()?;
        
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
        
        // Ensure all required runtime functions are available
        self.ensure_runtime_container_functions()?;
        
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
    /// Helper method to safely get the module reference for the range clause implementation
    #[inline]
    fn get_module_ref(&self) -> Result<&inkwell::module::Module<'ctx>, Error> {
        Ok(self.module())
    }
    
    /// Helper method to get element type from pointer type
    #[inline]
    fn get_pointee_type(&self, ptr_type: inkwell::types::PointerType<'ctx>) -> inkwell::types::BasicTypeEnum<'ctx> {
        // Get the element type of the pointer using the proper LLVM API
        use inkwell::types::BasicTypeEnum;
        match ptr_type.get_element_type() {
            BasicTypeEnum::ArrayType(t) => t.into(),
            BasicTypeEnum::FloatType(t) => t.into(),
            BasicTypeEnum::IntType(t) => t.into(),
            BasicTypeEnum::PointerType(t) => t.into(),
            BasicTypeEnum::StructType(t) => t.into(),
            BasicTypeEnum::VectorType(t) => t.into(),
        }
    }
    /// Get the length of a container
    /// Get container length (array size, slice length, etc.)
    /// 
    /// This method handles different container types and extracts their length
    /// appropriately. For arrays, it uses the type's length property, while for
    /// other containers, it accesses length fields or calls length methods.
    #[instrument(skip(self, container), level = "debug")]
    pub fn emit_container_length_fixed(&self, container: BasicValueEnum<'ctx>) -> Result<IntValue<'ctx>, Error> {
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
            let element_type = self.get_pointee_type(ptr_type);
            
            if element_type.is_array_type() {
                // Pointer to an array
                if let BasicTypeEnum::ArrayType(array_type) = element_type {
                    let length = array_type.len();
                    debug!("Pointer to array length: {}", length);
                    Ok(self.context.i32_type().const_int(length as u64, false))
                } else {
                    debug!("Failed to convert element type to array type");
                    Err(Error::CodeGenError("Expected array type".to_string()))
                }
            } else if element_type.is_struct_type() {
                // For slice-like types which are structs with a length field
                // Struct types typically have length as field 1 and data pointer as field 0
                if let BasicTypeEnum::StructType(struct_type) = element_type {
                    // Get a pointer to the length field (assuming it's at index 1)
                    let indices = &[
                        self.context.i32_type().const_zero(),
                        self.context.i32_type().const_int(1, false)
                    ];
                
                    // Create a GEP instruction to get the length field pointer
                    let length_field_ptr = unsafe {
                        self.builder.build_gep(struct_type, ptr_value, indices, "length_field_ptr")?
                    };
                    
                    // Load the length value
                    let type_name = struct_type.get_name().map(|n| n.to_str().unwrap_or("unnamed")).unwrap_or("unnamed");
                    debug!("Loading length field from struct type: {}", type_name);
                    
                    // Determine length field type - default to i32 if we can't determine it
                    let length_field_type = if struct_type.get_field_types().len() > 1 {
                        struct_type.get_field_types()[1]
                    } else {
                        self.context.i32_type().into()
                    };
                    
                    let length_value = self.builder.build_load(length_field_type, length_field_ptr, "container_length")?;
                    
                    // Convert to i32 if needed
                    if length_value.is_int_value() {
                        let length_int = length_value.into_int_value();
                        if length_int.get_type().get_bit_width() != 32 {
                            // Convert to i32
                            let i32_length = self.builder.build_int_cast(length_int, self.context.i32_type(), "length_cast")?;
                            Ok(i32_length)
                        } else {
                            Ok(length_int)
                        }
                    } else {
                        // Fall back to a default length if we couldn't load it properly
                        debug!("Falling back to default length for struct container");
                        Ok(self.context.i32_type().const_int(0, false))
                    }
                } else {
                    debug!("Failed to convert element type to struct type");
                    Err(Error::CodeGenError("Expected struct type".to_string()))
                }
            } else if element_type.is_pointer_type() {
                // This is likely a string, which is a pointer to char
                // For string types, we need to call a string length function
                
                // Get the module reference safely
                let module = self.get_module_ref()?;
                
                // Use the string_length function from our runtime
                let fn_name = "string_length";
                let length_fn = module.get_function(fn_name).unwrap_or_else(|| {
                    // Create a function for string length
                    let fn_type = self.context.i32_type().fn_type(&[ptr_value.get_type().into()], false);
                    let function = module.add_function(fn_name, fn_type, None);
                    // This ensures the function has external linkage
                    function.set_linkage(inkwell::module::Linkage::External);
                    function
                });
                
                // Call the string length function
                let call = self.builder.build_call(length_fn, &[ptr_value.into()], "strlen_call")?;
                
                // Get the return value
                call.try_as_basic_value().left()
                    .and_then(|val| val.into_int_value().into())
                    .ok_or_else(|| {
                        debug!("Failed to get string length return value");
                        Error::Compilation("Failed to get string length return value".to_string())
                    })
            } else {
                // For direct pointer to element types (like C arrays)
                // We need to determine the length from context or metadata
                // For now, we'll use a special runtime helper function
                
                // Get the module reference safely
                let module = self.get_module_ref()?;
                
                // Get or create a generic container length function 
                let fn_name = "container_length";
                let fn_type = self.context.i32_type().fn_type(&[ptr_value.get_type().into()], false);
                let length_fn = module.get_function(fn_name).unwrap_or_else(|| {
                    let function = module.add_function(fn_name, fn_type, None);
                    // This ensures the function has external linkage
                    function.set_linkage(inkwell::module::Linkage::External);
                    function
                });
                
                // Call the function
                let call = self.builder.build_call(length_fn, &[ptr_value.into()], "container_length_call")?;
                
                // Get the return value
                call.try_as_basic_value().left()
                    .and_then(|val| val.into_int_value().into())
                    .ok_or_else(|| {
                        debug!("Failed to get container length return value");
                        Error::Compilation("Failed to get container length return value".to_string())
                    })
            }
        } else {
            // For other container types like maps or custom containers
            // We need to call a length method or access a length property
            
            // Get the module reference safely
            let module = self.get_module_ref()?;
            
            // Try to find a container length function that takes this type
            let fn_names = vec![
                "container_length",
                "get_container_length",
                "map_length",
                "collection_length"
            ];
            
            // Find or create a length function
            let fn_type = self.context.i32_type().fn_type(&[container.get_type().into()], false);
            let length_fn = module.get_function("container_length").unwrap_or_else(|| {
                let function = module.add_function("container_length", fn_type, None);
                // This ensures the function has external linkage
                function.set_linkage(inkwell::module::Linkage::External);
                function
            });
            
            // Call the function
            let call = self.builder.build_call(length_fn, &[container.into()], "length_call")?;
            
            // Get the return value
            call.try_as_basic_value().left()
                .and_then(|val| val.into_int_value().into())
                .ok_or_else(|| {
                    debug!("Failed to get container length return value for non-standard container");
                    Error::Compilation("Failed to get container length for unsupported container type".to_string())
                })
        }
    }
    
    /// Get an element from a container at the given index
    /// 
    /// This method retrieves elements from different container types:
    /// - Arrays: Direct indexing with proper LLVM IR generation
    /// - Pointers to arrays: Uses proper GEP instructions
    /// - Slices and other container types: Handles through appropriate pointer arithmetic
    #[instrument(skip(self, container, index), level = "debug")]
    pub fn emit_get_element_fixed(&self, container: BasicValueEnum<'ctx>, index: IntValue<'ctx>) -> Result<BasicValueEnum<'ctx>, Error> {
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
                if let BasicTypeEnum::ArrayType(array_type) = pointee_type {
                    let element_type = array_type.get_element_type();
                    
                    // We have two options:
                    // 1. Load the array and then index into it
                    // 2. Use GEP directly on the array pointer
                    
                    // Option 2 is more efficient, so we'll use that
                    let indices = &[self.context.i32_type().const_zero(), index];
                    let element_ptr = unsafe {
                    self.builder.build_gep(array_type, ptr_value, indices, "element_ptr")?
                    };
                    
                    debug!("Loading array element from pointer to array using GEP");
                    Ok(self.builder.build_load(element_type, element_ptr, "element")?.into())
                } else {
                    debug!("Failed to convert pointee type to array type");
                    Err(Error::CodeGenError("Expected array type".to_string()))
                }
            } else if pointee_type.is_struct_type() {
                // This is likely a slice or similar container
                // For slices, we need to extract the data pointer and then index it
                
                if let BasicTypeEnum::StructType(struct_type) = pointee_type {
                    let type_name = struct_type.get_name().map(|s| s.to_str()).unwrap_or(Ok("unknown")).unwrap_or("unknown");
                    debug!("Accessing element from struct container: {}", type_name);
                    
                    // For slice types, the first field is typically the data pointer
                    if struct_type.get_field_types().len() > 0 {
                        // Get pointer to the data pointer field (field 0)
                        let indices = &[self.context.i32_type().const_zero(), self.context.i32_type().const_zero()];
                        let data_ptr_ptr = unsafe {
                            self.builder.build_gep(struct_type, ptr_value, indices, "data_ptr_ptr")?
                        };
                        
                        // Load the data pointer itself
                        let data_ptr_type = struct_type.get_field_types()[0];
                        if !data_ptr_type.is_pointer_type() {
                            return Err(Error::CodeGenError(format!("First field of struct {} is not a pointer type", type_name)));
                        }
                        
                        debug!("Loading data pointer from struct field");
                        let data_ptr = self.builder.build_load(data_ptr_type, data_ptr_ptr, "data_ptr")?;
                        
                        if !data_ptr.is_pointer_value() {
                            return Err(Error::CodeGenError(format!("Loaded value is not a pointer")));
                        }
                        
                        // Get element type from the data pointer
                        let data_ptr_value = data_ptr.into_pointer_value();
                        let element_type = data_ptr_type.into_pointer_type().get_element_type();
                        
                        // Index into the data pointer
                        let indices = &[index];
                        let element_ptr = unsafe {
                            self.builder.build_gep(element_type, data_ptr_value, indices, "element_ptr")?
                        };
                        
                        debug!("Loading element from slice data pointer");
                        Ok(self.builder.build_load(element_type, element_ptr, "element")?.into())
                    } else {
                        // Fallback approach for unknown struct layouts
                        debug!("Using fallback for container with unknown layout");
                        
                        // Try to use a runtime helper function
                        let module = self.get_module_ref()?;
                        
                        // Use our runtime container_get_element function
                        let fn_name = "container_get_element";
                        
                        // Create the function signature
                        let fn_type = self.context.i64_type().fn_type(&[ptr_value.get_type().into(), index.get_type().into()], false);
                        
                        // Get or declare the function
                        let get_elem_fn = module.get_function(fn_name).unwrap_or_else(|| {
                        let function = module.add_function(fn_name, fn_type, None);
                        // This ensures the function has external linkage
                        function.set_linkage(inkwell::module::Linkage::External);
                        function
                        });
                        
                        // Call the function to get the element
                        debug!("Calling get_element function for container");
                        let call = self.builder.build_call(get_elem_fn, &[ptr_value.into(), index.into()], "get_elem_call")?;
                        
                        // Get the return value
                        call.try_as_basic_value().left().ok_or_else(|| {
                            Error::Compilation("Failed to get element from container using helper function".to_string())
                        })
                        }
            } else {
                debug!("Failed to convert pointee type to struct type");
                Err(Error::CodeGenError("Expected struct type for container".to_string()))
            }
            } else if pointee_type.is_pointer_type() {
                // This could be a string-like type (pointer to elements)
                // or another container with a pointer to its data
                let element_ptr_type = pointee_type.into_pointer_type();
                
                // First, load the pointer
                let data_ptr = self.builder.build_load(element_ptr_type, ptr_value, "string_data_ptr")?;
                
                if !data_ptr.is_pointer_value() {
                    return Err(Error::CodeGenError("Expected a pointer value after loading".to_string()));
                }
                
                // Then index into it
                let inner_ptr = data_ptr.into_pointer_value();
                let element_type = element_ptr_type.get_element_type();
                
                let indices = &[index];
                let element_ptr = unsafe {
                    self.builder.build_gep(element_type, inner_ptr, indices, "string_element_ptr")?
                };
                
                debug!("Loading element from string/pointer-to-pointer");
                Ok(self.builder.build_load(element_type, element_ptr, "string_element")?.into())
            } else {
                // Direct pointer to elements (like a C-style array)
                // This is the simplest case - just index into the pointer
                let indices = &[index];
                let element_ptr = unsafe {
                    self.builder.build_gep(pointee_type, ptr_value, indices, "element_ptr")?
                };
                
                debug!("Loading element from direct pointer indexing");
                Ok(self.builder.build_load(pointee_type, element_ptr, "element")?.into())
            }
        } else if container.is_struct_value() {
            // Direct struct value (not a pointer to a struct)
            let struct_val = container.into_struct_value();
            let struct_type = struct_val.get_type();
            let type_name = struct_type.get_name().map_or("unknown", |n| n.to_str().unwrap_or("unknown"));
            
            debug!("Accessing element from direct struct value: {}", type_name);
            
            // For a proper implementation, we would need to copy the struct to a local variable
            // and then access its fields. For now, we'll use a runtime helper function.
            
            let module = self.get_module_ref()?;
            
            // Use our runtime container_get_element function
            let fn_name = "container_get_element";
            
            // Create the function signature
            let fn_type = self.context.i64_type().fn_type(
                &[struct_val.get_type().into(), index.get_type().into()], 
                false
            );
            
            // Get or declare the function
            let get_elem_fn = module.get_function(fn_name).unwrap_or_else(|| {
                let function = module.add_function(fn_name, fn_type, None);
                // This ensures the function has external linkage
                function.set_linkage(inkwell::module::Linkage::External);
                function
            });
            
            // We can't pass a struct value directly to a function in many cases,
            // so we'll need to create a temporary variable and pass its address
            let temp_ptr = self.builder.build_alloca(struct_type, "temp_struct")?;
            self.builder.build_store(temp_ptr, struct_val)?;
            
            // Call the function to get the element
            debug!("Calling get_element function for direct struct");
            let call = self.builder.build_call(
                get_elem_fn, 
                &[temp_ptr.into(), index.into()], 
                "struct_get_elem_call"
            )?;
            
            // Get the return value
            call.try_as_basic_value().left().ok_or_else(|| {
                Error::Compilation("Failed to get element from direct struct value".to_string())
            })
        } else {
            // For other container types or unsupported types
            debug!("Unsupported container type: {:?}", container.get_type());
            Err(Error::CodeGenError(format!(
                "Cannot get element from container with type: {:?}", 
                container.get_type()
            )))
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
    pub fn determine_element_type_fixed(&self, container: BasicValueEnum<'ctx>) -> Result<BasicTypeEnum<'ctx>, Error> {
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
            let pointee_type = self.get_pointee_type(ptr_type);
            
            if pointee_type.is_array_type() {
                // Pointer to an array - get the array's element type
                if let BasicTypeEnum::ArrayType(array_type) = pointee_type {
                    let element_type = array_type.get_element_type();
                    debug!("Pointer to array element type extracted");
                    Ok(element_type)
                } else {
                    debug!("Failed to convert pointee type to array type");
                    Err(Error::CodeGenError("Expected array type".to_string()))
                }
            } else if pointee_type.is_struct_type() {
                // This is likely a slice or other structured container (e.g., a Slice<T>)
                if let BasicTypeEnum::StructType(struct_type) = pointee_type {
                    let type_name = struct_type.get_name().map_or("unknown", |n| n.to_str().unwrap_or("unknown"));
                    debug!("Determining element type from struct container: {}", type_name);
                    
                    // We have multiple approaches to determine the element type:
                    
                    // 1. Field-based approach: For slice-like types, field 0 is typically a data pointer
                    if struct_type.get_field_types().len() > 0 {
                        let data_field_type = struct_type.get_field_types()[0];
                        if data_field_type.is_pointer_type() {
                            // Get the element type from the data pointer
                            let data_ptr_type = data_field_type.into_pointer_type();
                            let element_type = data_ptr_type.get_element_type();
                            debug!("Extracted element type from data pointer field");
                            return Ok(element_type);
                        }
                    }
                
                // 2. Name-based approach: Parse element type from the struct name
                // Common patterns: Slice<T>, Array<T>, Vector<T>, etc.
                if type_name.contains('<') && type_name.contains('>') {
                    // Try to extract the type parameter from the name
                    let start_idx = type_name.find('<').unwrap() + 1;
                    let end_idx = type_name.rfind('>').unwrap();
                    if start_idx < end_idx {
                        let type_param = &type_name[start_idx..end_idx];
                        debug!("Extracted type parameter from name: {}", type_param);
                        
                        // Map the type name to a BasicType
                        // This is a simplified approach - in a real implementation,
                        // we would have a type registry to look up these types
                        match type_param {
                            "i8" | "char" | "rune" | "sip" => {
                                debug!("Using i8 type for element");
                                return Ok(self.context.i8_type().into());
                            },
                            "i32" | "int" => {
                                debug!("Using i32 type for element");
                                return Ok(self.context.i32_type().into());
                            },
                            "i64" | "lit" => {
                                debug!("Using i64 type for element");
                                return Ok(self.context.i64_type().into());
                            },
                            "f32" => {
                                debug!("Using f32 type for element");
                                return Ok(self.context.f32_type().into());
                            },
                            "f64" | "normie" => {
                                debug!("Using f64 type for element");
                                return Ok(self.context.f64_type().into());
                            },
                            "string" | "tea" => {
                                debug!("Using i8* type for string element");
                                return Ok(self.context.i8_type().ptr_type(AddressSpace::default()).into());
                            },
                            _ => {
                                // For other types, try to look up the type in the module
                                // This would require a type registry in a real implementation
                                debug!("Unknown type parameter: {}, using i64 fallback", type_param);
                            }
                        }
                    }
                }
                
                    // 3. Runtime type query: Use a runtime function to determine element type
                    // In a real implementation, this would call into a reflection system
                    debug!("Using fallback i64 type for struct container");
                    return Ok(self.context.i64_type().into());
                } else {
                    debug!("Failed to convert pointee type to struct type");
                    return Err(Error::CodeGenError("Expected struct type".to_string()));
                }
            } else if pointee_type.is_pointer_type() {
                // This might be a string-like type (pointer to elements)
                // String in CURSED is typically a pointer to runes (i8)
                let element_ptr_type = pointee_type.into_pointer_type();
                let element_type = element_ptr_type.get_element_type();
                debug!("String or pointer-to-pointer element type extracted");
                Ok(element_type)
            } else {
                // Direct pointer to elements (like a C-style array)
                debug!("Direct pointer to elements type: {:?}", pointee_type);
                Ok(pointee_type)
            }
        } else if container.is_struct_value() {
            // Direct struct container (not a pointer to a struct)
            let struct_val = container.into_struct_value();
            let struct_type = struct_val.get_type();
            
            // Try to get element type from field 0 (assuming data pointer pattern)
            if struct_type.get_field_types().len() > 0 {
                let first_field_type = struct_type.get_field_types()[0];
                if first_field_type.is_pointer_type() {
                    let ptr_type = first_field_type.into_pointer_type();
                    let element_type = ptr_type.get_element_type();
                    debug!("Element type extracted from direct struct's first field");
                    return Ok(element_type);
                }
            }
            
            // Fallback to i64 if we can't determine the type
            debug!("Using fallback i64 type for direct struct container");
            return Ok(self.context.i64_type().into());
        } else {
            // For unsupported container types
            debug!("Unsupported container type: {:?}", container.get_type());
            Err(Error::CodeGenError(format!("Cannot determine element type for container: {:?}", container.get_type())))
        }
    }
    
    /// Create an iterator for a map
    pub fn emit_map_iterator_create_fixed(&self, map_value: BasicValueEnum<'ctx>) -> Result<PointerValue<'ctx>, Error> {
        debug!("Creating map iterator");
        if !map_value.is_pointer_value() {
            return Err(Error::CodeGenError("Expected map to be a pointer type".to_string()));
        }
        
        let map_ptr = map_value.into_pointer_value();
        
        // Get the module reference safely
        let module = self.get_module_ref()?;
        
        // Use our runtime map_iterator_create function
        let fn_name = "map_iterator_create";
        let fn_type = self.context.i8_type().ptr_type(AddressSpace::default())
            .fn_type(&[map_ptr.get_type().into()], false);
            
        // Get or declare the function
        let create_fn = module.get_function(fn_name).unwrap_or_else(|| {
            let function = module.add_function(fn_name, fn_type, None);
            // This ensures the function has external linkage
            function.set_linkage(inkwell::module::Linkage::External);
            function
        });
        
        // Call the function
        let call = self.builder.build_call(create_fn, &[map_ptr.into()], "map_iterator_create_call")?;
        
        // Get the return value as a pointer
        let iterator_ptr = call.try_as_basic_value().left()
            .ok_or_else(|| Error::Compilation("Failed to get map iterator".to_string()))?;
            
        if !iterator_ptr.is_pointer_value() {
            return Err(Error::Compilation("Map iterator is not a pointer".to_string()));
        }
        
        Ok(iterator_ptr.into_pointer_value())
    }
    
    /// Check if a map iterator has more elements
    pub fn emit_map_iterator_has_next_fixed(&self, iterator_ptr: PointerValue<'ctx>) -> Result<IntValue<'ctx>, Error> {
        debug!("Checking if map iterator has next element");
        
        // Get the module reference safely
        let module = self.get_module_ref()?;
        
        // Use our runtime map_iterator_has_next function
        let fn_name = "map_iterator_has_next";
        let fn_type = self.context.i32_type()
            .fn_type(&[iterator_ptr.get_type().into()], false);
            
        // Get or declare the function
        let has_next_fn = module.get_function(fn_name).unwrap_or_else(|| {
            let function = module.add_function(fn_name, fn_type, None);
            // This ensures the function has external linkage
            function.set_linkage(inkwell::module::Linkage::External);
            function
        });
        
        // Call the function
        let call = self.builder.build_call(has_next_fn, &[iterator_ptr.into()], "map_iterator_has_next_call")?;
        
        // Get the return value as an integer (boolean)
        let has_next = call.try_as_basic_value().left()
            .ok_or_else(|| Error::Compilation("Failed to get map iterator has_next result".to_string()))?;
            
        if !has_next.is_int_value() {
            return Err(Error::Compilation("Map iterator has_next result is not an integer".to_string()));
        }
        
        // Convert the result to a boolean value for the conditional branch
        let has_next_int = has_next.into_int_value();
        let bool_value = self.builder.build_int_compare(
            IntPredicate::NE,
            has_next_int,
            self.context.i32_type().const_zero(),
            "has_next_bool"
        )?;
        
        Ok(bool_value)
    }
    
    /// Get current key-value pair from map iterator and advance
    pub fn emit_map_iterator_get_current_fixed(
        &self,
        iterator_ptr: PointerValue<'ctx>,
        key_ptr: PointerValue<'ctx>,
        value_ptr: PointerValue<'ctx>
    ) -> Result<(), Error> {
        debug!("Getting current key-value pair from map iterator");
        
        // Get the module reference safely
        let module = self.get_module_ref()?;
        
        // Use our runtime map_iterator_get_current function
        let fn_name = "map_iterator_get_current";
        let fn_type = self.context.i32_type().fn_type(&[
            iterator_ptr.get_type().into(),
            key_ptr.get_type().into(),
            value_ptr.get_type().into()
        ], false);
        
        // Get or declare the function
        let get_current_fn = module.get_function(fn_name).unwrap_or_else(|| {
            let function = module.add_function(fn_name, fn_type, None);
            // This ensures the function has external linkage
            function.set_linkage(inkwell::module::Linkage::External);
            function
        });
        
        // Call the function
        let call = self.builder.build_call(
            get_current_fn, 
            &[iterator_ptr.into(), key_ptr.into(), value_ptr.into()], 
            "map_iterator_get_current_call"
        )?;
        
        // Check return value (non-zero indicates success)
        let result = call.try_as_basic_value().left()
            .ok_or_else(|| Error::Compilation("Failed to get map iterator current result".to_string()))?;
            
        if !result.is_int_value() {
            return Err(Error::Compilation("Map iterator get_current result is not an integer".to_string()));
        }
        
        let result_int = result.into_int_value();
        let success = self.builder.build_int_compare(
            IntPredicate::NE,
            result_int,
            self.context.i32_type().const_zero(),
            "get_current_success"
        )?;
        
        // We could check if it succeeded and return an error if not,
        // but for now we'll just return Ok and let the calling code handle any issues
        Ok(())
    }
    
    /// Advance a map iterator to the next element
    pub fn emit_map_iterator_next_fixed(&self, iterator_ptr: PointerValue<'ctx>) -> Result<(), Error> {
        debug!("Advancing map iterator to next element");
        
        // Get the module reference safely
        let module = self.get_module_ref()?;
        
        // Use our runtime map_iterator_next function
        let fn_name = "map_iterator_next";
        let fn_type = self.context.void_type().fn_type(&[iterator_ptr.get_type().into()], false);
        
        // Get or declare the function
        let next_fn = module.get_function(fn_name).unwrap_or_else(|| {
            let function = module.add_function(fn_name, fn_type, None);
            // This ensures the function has external linkage
            function.set_linkage(inkwell::module::Linkage::External);
            function
        });
        
        // Call the function (returns void)
        self.builder.build_call(next_fn, &[iterator_ptr.into()], "map_iterator_next_call")?;
        
        Ok(())
    }
    
    /// Determine the key type for a map
    pub fn determine_map_key_type_fixed(&self, map_value: BasicValueEnum<'ctx>) -> Result<BasicTypeEnum<'ctx>, Error> {
        // Use the improved map iteration enhancements functionality
        use crate::codegen::llvm::map_iteration_improvements::MapIterationEnhancements;
        self.determine_map_key_type(map_value)
    }
    
    /// Determine the value type for a map
    pub fn determine_map_value_type_fixed(&self, map_value: BasicValueEnum<'ctx>) -> Result<BasicTypeEnum<'ctx>, Error> {
        // Use the improved map iteration enhancements functionality
        use crate::codegen::llvm::map_iteration_improvements::MapIterationEnhancements;
        self.determine_map_value_type(map_value)
    }

    /// Helper method to add missing instrument annotation to methods that need it
    /// and add more robust error handling for container operations
    #[instrument(skip(self), level = "debug")]
    pub fn ensure_runtime_container_functions(&self) -> Result<(), Error> {
        // Get the module reference safely
        let module = self.get_module_ref()?;
        
        // Ensure our runtime container support functions are available
        let container_function_names = vec![
            "container_length",
            "container_get_element",
            "string_length",
            "map_iterator_create",
            "map_iterator_has_next",
            "map_iterator_next",
            "map_iterator_get_current"
        ];
        
        for fn_name in container_function_names {
            // Check if the function exists, and if not, generate a placeholder
            if module.get_function(fn_name).is_none() {
                debug!("Creating placeholder for runtime function: {}", fn_name);
                
                // Create appropriate function type based on name
                let fn_type = match fn_name {
                    "container_length" | "string_length" => {
                        // These take a pointer and return an i32
                        let ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
                        self.context.i32_type().fn_type(&[ptr_type.into()], false)
                    },
                    "container_get_element" => {
                        // Takes a pointer and an index, returns a pointer
                        let ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
                        let ptr_ret_type = self.context.i8_type().ptr_type(AddressSpace::default());
                        ptr_ret_type.fn_type(&[ptr_type.into(), self.context.i32_type().into()], false)
                    },
                    "map_iterator_create" => {
                        // Takes a map pointer and returns an iterator pointer
                        let ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
                        let ptr_ret_type = self.context.i8_type().ptr_type(AddressSpace::default());
                        ptr_ret_type.fn_type(&[ptr_type.into()], false)
                    },
                    "map_iterator_has_next" => {
                        // Takes an iterator pointer and returns a boolean (i32)
                        let ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
                        self.context.i32_type().fn_type(&[ptr_type.into()], false)
                    },
                    "map_iterator_next" => {
                        // Takes an iterator pointer and returns void
                        let ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
                        self.context.void_type().fn_type(&[ptr_type.into()], false)
                    },
                    "map_iterator_get_current" => {
                        // Takes iterator pointer and two output pointers, returns status i32
                        let ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
                        self.context.i32_type().fn_type(&[ptr_type.into(), ptr_type.into(), ptr_type.into()], false)
                    },
                    _ => {
                        // Default to a simple void function for unknown names
                        self.context.void_type().fn_type(&[], false)
                    }
                };
                
                // Add the function to the module with external linkage
                let function = module.add_function(fn_name, fn_type, None);
                function.set_linkage(inkwell::module::Linkage::External);
            }
        }
        
        Ok(())
    }
}