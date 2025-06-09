//! LLVM code generation for slice operations in the CURSED language.
//!
//! This module provides compilation of slice operations including append,
//! subslicing, copying, and bounds checking to LLVM IR.

use crate::codegen::llvm::types::{convert_type, get_type_size};
use crate::core::type_checker::Type;
use crate::error_enhanced::CursedError;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::BasicType;
use inkwell::values::{BasicValueEnum, IntValue, PointerValue, StructValue};
use inkwell::{AddressSpace, IntPredicate};
use tracing::{debug, info, instrument, warn};

/// Trait for compiling slice operations to LLVM IR
pub trait SliceOperations<'ctx> {
    /// Append an element to a slice, potentially reallocating if needed
    fn slice_append(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        slice_value: StructValue<'ctx>,
        element_value: BasicValueEnum<'ctx>,
        element_type: &Type,
    ) -> Result<StructValue<'ctx>, CursedError>;

    /// Create a subslice from slice[start:end]
    fn slice_subslice(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        slice_value: StructValue<'ctx>,
        start_index: IntValue<'ctx>,
        end_index: IntValue<'ctx>,
        element_type: &Type,
    ) -> Result<StructValue<'ctx>, CursedError>;

    /// Copy a slice to a new slice
    fn slice_copy(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        slice_value: StructValue<'ctx>,
        element_type: &Type,
    ) -> Result<StructValue<'ctx>, CursedError>;

    /// Get the length of a slice
    fn slice_len(
        &self,
        context: &'ctx Context,
        builder: &Builder<'ctx>,
        slice_value: StructValue<'ctx>,
    ) -> Result<IntValue<'ctx>, CursedError>;

    /// Get the capacity of a slice
    fn slice_cap(
        &self,
        context: &'ctx Context,
        builder: &Builder<'ctx>,
        slice_value: StructValue<'ctx>,
    ) -> Result<IntValue<'ctx>, CursedError>;

    /// Perform bounds checking for slice access
    fn check_slice_bounds(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        slice_value: StructValue<'ctx>,
        index: IntValue<'ctx>,
    ) -> Result<(), CursedError>;

    /// Access a slice element with bounds checking
    fn slice_index(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        slice_value: StructValue<'ctx>,
        index: IntValue<'ctx>,
        element_type: &Type,
    ) -> Result<BasicValueEnum<'ctx>, CursedError>;
}

/// Implementation of slice operations
pub struct SliceOperationsImpl;

impl<'ctx> SliceOperations<'ctx> for SliceOperationsImpl {
    #[instrument(skip(self, context, module, builder), level = "debug")]
    fn slice_append(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        slice_value: StructValue<'ctx>,
        element_value: BasicValueEnum<'ctx>,
        element_type: &Type,
    ) -> Result<StructValue<'ctx>, CursedError> {
        info!("Appending element to slice");

        // Extract slice components
        let ptr = builder
            .build_extract_value(slice_value, 0, "slice_ptr")
            .map_err(|e| CursedError::codegen(format!("Failed to extract ptr: {}", e)))?
            .into_pointer_value();

        let len = builder
            .build_extract_value(slice_value, 1, "slice_len")
            .map_err(|e| CursedError::codegen(format!("Failed to extract len: {}", e)))?
            .into_int_value();

        let cap = builder
            .build_extract_value(slice_value, 2, "slice_cap")
            .map_err(|e| CursedError::codegen(format!("Failed to extract cap: {}", e)))?
            .into_int_value();

        // Check if we need to grow the slice
        let needs_realloc = builder
            .build_int_compare(IntPredicate::EQ, len, cap, "needs_realloc")
            .map_err(|e| CursedError::codegen(format!("Failed to compare len and cap: {}", e)))?;

        // Create blocks for reallocation and no reallocation paths
        let function = builder
            .get_insert_block()
            .ok_or_else(|| CursedError::codegen("No insert block".to_string()))?
            .get_parent()
            .ok_or_else(|| CursedError::codegen("No parent function".to_string()))?;

        let realloc_block = context.append_basic_block(function, "realloc");
        let no_realloc_block = context.append_basic_block(function, "no_realloc");
        let append_block = context.append_basic_block(function, "append");

        builder
            .build_conditional_branch(needs_realloc, realloc_block, no_realloc_block)
            .map_err(|e| CursedError::codegen(format!("Failed to build conditional branch: {}", e)))?;

        // Reallocation path
        builder.position_at_end(realloc_block);
        let new_cap = self.grow_slice_capacity(context, builder, cap)?;
        let new_ptr = self.reallocate_slice(context, module, builder, ptr, len, new_cap, element_type)?;
        builder
            .build_unconditional_branch(append_block)
            .map_err(|e| CursedError::codegen(format!("Failed to build unconditional branch: {}", e)))?;

        // No reallocation path
        builder.position_at_end(no_realloc_block);
        builder
            .build_unconditional_branch(append_block)
            .map_err(|e| CursedError::codegen(format!("Failed to build unconditional branch: {}", e)))?;

        // Append block - merge both paths
        builder.position_at_end(append_block);

        // PHI nodes for ptr and cap
        let final_ptr = builder
            .build_phi(ptr.get_type(), "final_ptr")
            .map_err(|e| CursedError::codegen(format!("Failed to build ptr phi: {}", e)))?;
        final_ptr.add_incoming(&[(&ptr, no_realloc_block), (&new_ptr, realloc_block)]);

        let final_cap = builder
            .build_phi(cap.get_type(), "final_cap")
            .map_err(|e| CursedError::codegen(format!("Failed to build cap phi: {}", e)))?;
        final_cap.add_incoming(&[(&cap, no_realloc_block), (&new_cap, realloc_block)]);

        // Store the new element at index len
        let element_gep = unsafe {
            builder.build_gep(
                element_value.get_type().into_struct_type(),
                final_ptr.as_basic_value().into_pointer_value(),
                &[len],
                "element_ptr",
            )
            .map_err(|e| CursedError::codegen(format!("Failed to build GEP: {}", e)))?
        };

        builder
            .build_store(element_gep, element_value)
            .map_err(|e| CursedError::codegen(format!("Failed to store element: {}", e)))?;

        // Increment length
        let one = context.i64_type().const_int(1, false);
        let new_len = builder
            .build_int_add(len, one, "new_len")
            .map_err(|e| CursedError::codegen(format!("Failed to add to len: {}", e)))?;

        // Build new slice struct
        let new_slice = slice_value.get_type().get_undef();
        let slice_with_ptr = builder
            .build_insert_value(new_slice, final_ptr.as_basic_value(), 0, "slice_with_ptr")
            .map_err(|e| CursedError::codegen(format!("Failed to insert ptr: {}", e)))?
            .into_struct_value();

        let slice_with_len = builder
            .build_insert_value(slice_with_ptr, new_len, 1, "slice_with_len")
            .map_err(|e| CursedError::codegen(format!("Failed to insert len: {}", e)))?
            .into_struct_value();

        let final_slice = builder
            .build_insert_value(slice_with_len, final_cap.as_basic_value(), 2, "final_slice")
            .map_err(|e| CursedError::codegen(format!("Failed to insert cap: {}", e)))?
            .into_struct_value();

        debug!("Slice append completed successfully");
        Ok(final_slice)
    }

    #[instrument(skip(self, context, module, builder), level = "debug")]
    fn slice_subslice(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        slice_value: StructValue<'ctx>,
        start_index: IntValue<'ctx>,
        end_index: IntValue<'ctx>,
        element_type: &Type,
    ) -> Result<StructValue<'ctx>, CursedError> {
        info!("Creating subslice");

        // Bounds checking
        self.check_subslice_bounds(context, module, builder, slice_value, start_index, end_index)?;

        // Extract slice components
        let ptr = builder
            .build_extract_value(slice_value, 0, "slice_ptr")
            .map_err(|e| CursedError::codegen(format!("Failed to extract ptr: {}", e)))?
            .into_pointer_value();

        // Calculate new pointer (ptr + start_index)
        let new_ptr = unsafe {
            builder.build_gep(
                convert_type(context, element_type)
                    .map_err(|e| CursedError::codegen(format!("Failed to convert element type: {}", e)))?,
                ptr,
                &[start_index],
                "subslice_ptr",
            )
            .map_err(|e| CursedError::codegen(format!("Failed to build GEP for subslice: {}", e)))?
        };

        // Calculate new length (end_index - start_index)
        let new_len = builder
            .build_int_sub(end_index, start_index, "subslice_len")
            .map_err(|e| CursedError::codegen(format!("Failed to calculate subslice length: {}", e)))?;

        // New capacity is the same as length for subslices
        let new_cap = new_len;

        // Build subslice struct
        let subslice = slice_value.get_type().get_undef();
        let slice_with_ptr = builder
            .build_insert_value(subslice, new_ptr, 0, "subslice_with_ptr")
            .map_err(|e| CursedError::codegen(format!("Failed to insert ptr: {}", e)))?
            .into_struct_value();

        let slice_with_len = builder
            .build_insert_value(slice_with_ptr, new_len, 1, "subslice_with_len")
            .map_err(|e| CursedError::codegen(format!("Failed to insert len: {}", e)))?
            .into_struct_value();

        let final_subslice = builder
            .build_insert_value(slice_with_len, new_cap, 2, "final_subslice")
            .map_err(|e| CursedError::codegen(format!("Failed to insert cap: {}", e)))?
            .into_struct_value();

        debug!("Subslice created successfully");
        Ok(final_subslice)
    }

    #[instrument(skip(self, context, module, builder), level = "debug")]
    fn slice_copy(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        slice_value: StructValue<'ctx>,
        element_type: &Type,
    ) -> Result<StructValue<'ctx>, CursedError> {
        info!("Copying slice");

        // Extract slice components
        let len = builder
            .build_extract_value(slice_value, 1, "slice_len")
            .map_err(|e| CursedError::codegen(format!("Failed to extract len: {}", e)))?
            .into_int_value();

        // Allocate new memory
        let new_ptr = self.allocate_slice_memory(context, module, builder, element_type, len)?;

        // Copy memory from old slice to new slice
        self.copy_slice_memory(context, module, builder, slice_value, new_ptr, len, element_type)?;

        // Build new slice struct with copied data
        let new_slice = slice_value.get_type().get_undef();
        let slice_with_ptr = builder
            .build_insert_value(new_slice, new_ptr, 0, "copied_slice_ptr")
            .map_err(|e| CursedError::codegen(format!("Failed to insert ptr: {}", e)))?
            .into_struct_value();

        let slice_with_len = builder
            .build_insert_value(slice_with_ptr, len, 1, "copied_slice_len")
            .map_err(|e| CursedError::codegen(format!("Failed to insert len: {}", e)))?
            .into_struct_value();

        let copied_slice = builder
            .build_insert_value(slice_with_len, len, 2, "copied_slice")
            .map_err(|e| CursedError::codegen(format!("Failed to insert cap: {}", e)))?
            .into_struct_value();

        debug!("Slice copy completed successfully");
        Ok(copied_slice)
    }

    fn slice_len(
        &self,
        context: &'ctx Context,
        builder: &Builder<'ctx>,
        slice_value: StructValue<'ctx>,
    ) -> Result<IntValue<'ctx>, CursedError> {
        let len = builder
            .build_extract_value(slice_value, 1, "slice_len")
            .map_err(|e| CursedError::codegen(format!("Failed to extract len: {}", e)))?
            .into_int_value();
        Ok(len)
    }

    fn slice_cap(
        &self,
        context: &'ctx Context,
        builder: &Builder<'ctx>,
        slice_value: StructValue<'ctx>,
    ) -> Result<IntValue<'ctx>, CursedError> {
        let cap = builder
            .build_extract_value(slice_value, 2, "slice_cap")
            .map_err(|e| CursedError::codegen(format!("Failed to extract cap: {}", e)))?
            .into_int_value();
        Ok(cap)
    }

    #[instrument(skip(self, context, module, builder), level = "debug")]
    fn check_slice_bounds(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        slice_value: StructValue<'ctx>,
        index: IntValue<'ctx>,
    ) -> Result<(), CursedError> {
        debug!("Checking slice bounds");

        let len = self.slice_len(context, builder, slice_value)?;

        // Check if index < len
        let is_valid = builder
            .build_int_compare(IntPredicate::ULT, index, len, "bounds_check")
            .map_err(|e| CursedError::codegen(format!("Failed to compare index and length: {}", e)))?;

        // Create panic block for out of bounds
        let function = builder
            .get_insert_block()
            .ok_or_else(|| CursedError::codegen("No insert block".to_string()))?
            .get_parent()
            .ok_or_else(|| CursedError::codegen("No parent function".to_string()))?;

        let panic_block = context.append_basic_block(function, "bounds_panic");
        let continue_block = context.append_basic_block(function, "bounds_ok");

        builder
            .build_conditional_branch(is_valid, continue_block, panic_block)
            .map_err(|e| CursedError::codegen(format!("Failed to build conditional branch: {}", e)))?;

        // Panic block - call panic function
        builder.position_at_end(panic_block);
        self.emit_panic(context, module, builder, "slice index out of bounds")?;

        // Continue block
        builder.position_at_end(continue_block);

        debug!("Bounds check completed");
        Ok(())
    }

    #[instrument(skip(self, context, module, builder), level = "debug")]
    fn slice_index(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        slice_value: StructValue<'ctx>,
        index: IntValue<'ctx>,
        element_type: &Type,
    ) -> Result<BasicValueEnum<'ctx>, CursedError> {
        debug!("Accessing slice element at index");

        // Bounds checking
        self.check_slice_bounds(context, module, builder, slice_value, index)?;

        // Extract pointer
        let ptr = builder
            .build_extract_value(slice_value, 0, "slice_ptr")
            .map_err(|e| CursedError::codegen(format!("Failed to extract ptr: {}", e)))?
            .into_pointer_value();

        // Get element pointer
        let element_gep = unsafe {
            builder.build_gep(
                convert_type(context, element_type)
                    .map_err(|e| CursedError::codegen(format!("Failed to convert element type: {}", e)))?,
                ptr,
                &[index],
                "element_ptr",
            )
            .map_err(|e| CursedError::codegen(format!("Failed to build GEP: {}", e)))?
        };

        // Load the element
        let element_llvm_type = convert_type(context, element_type)
            .map_err(|e| CursedError::codegen(format!("Failed to convert element type: {}", e)))?;
        let element_value = builder
            .build_load(element_llvm_type, element_gep, "element_value")
            .map_err(|e| CursedError::codegen(format!("Failed to load element: {}", e)))?;

        debug!("Slice element access completed");
        Ok(element_value)
    }
}

impl SliceOperationsImpl {
    /// Grow slice capacity (typically double it)
    fn grow_slice_capacity<'ctx>(
        &self,
        context: &'ctx Context,
        builder: &Builder<'ctx>,
        current_cap: IntValue<'ctx>,
    ) -> Result<IntValue<'ctx>, CursedError> {
        let two = context.i64_type().const_int(2, false);
        let new_cap = builder
            .build_int_mul(current_cap, two, "new_cap")
            .map_err(|e| CursedError::codegen(format!("Failed to multiply capacity: {}", e)))?;
        Ok(new_cap)
    }

    /// Reallocate slice memory to new capacity
    fn reallocate_slice<'ctx>(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        old_ptr: PointerValue<'ctx>,
        len: IntValue<'ctx>,
        new_cap: IntValue<'ctx>,
        element_type: &Type,
    ) -> Result<PointerValue<'ctx>, CursedError> {
        let element_size = get_type_size(element_type) as u64;
        let element_size_value = context.i64_type().const_int(element_size, false);

        // Calculate new total size
        let new_total_size = builder
            .build_int_mul(new_cap, element_size_value, "new_total_size")
            .map_err(|e| CursedError::codegen(format!("Failed to calculate new total size: {}", e)))?;

        // Get realloc function
        let realloc_fn_type = context.i8_type()
            .ptr_type(AddressSpace::default())
            .fn_type(&[
                context.i8_type().ptr_type(AddressSpace::default()).into(),
                context.i64_type().into(),
            ], false);

        let realloc_fn = module.add_function("realloc", realloc_fn_type, None);

        // Cast old pointer to i8*
        let old_ptr_i8 = builder
            .build_pointer_cast(
                old_ptr,
                context.i8_type().ptr_type(AddressSpace::default()),
                "old_ptr_i8",
            )
            .map_err(|e| CursedError::codegen(format!("Failed to cast old pointer: {}", e)))?;

        // Call realloc
        let realloc_result = builder
            .build_call(realloc_fn, &[old_ptr_i8.into(), new_total_size.into()], "realloc_result")
            .map_err(|e| CursedError::codegen(format!("Failed to call realloc: {}", e)))?;

        // Cast result back to element type pointer
        let raw_ptr = realloc_result
            .try_as_basic_value()
            .left()
            .ok_or_else(|| CursedError::codegen("Realloc returned void".to_string()))?
            .into_pointer_value();

        let element_llvm_type = convert_type(context, element_type)
            .map_err(|e| CursedError::codegen(format!("Failed to convert element type: {}", e)))?;

        let typed_ptr = builder
            .build_pointer_cast(
                raw_ptr,
                element_llvm_type.ptr_type(AddressSpace::default()),
                "typed_ptr",
            )
            .map_err(|e| CursedError::codegen(format!("Failed to cast new pointer: {}", e)))?;

        Ok(typed_ptr)
    }

    /// Check bounds for subslice operation
    fn check_subslice_bounds<'ctx>(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        slice_value: StructValue<'ctx>,
        start: IntValue<'ctx>,
        end: IntValue<'ctx>,
    ) -> Result<(), CursedError> {
        let len = builder
            .build_extract_value(slice_value, 1, "slice_len")
            .map_err(|e| CursedError::codegen(format!("Failed to extract len: {}", e)))?
            .into_int_value();

        // Check start <= end
        let start_le_end = builder
            .build_int_compare(IntPredicate::ULE, start, end, "start_le_end")
            .map_err(|e| CursedError::codegen(format!("Failed to compare start and end: {}", e)))?;

        // Check end <= len
        let end_le_len = builder
            .build_int_compare(IntPredicate::ULE, end, len, "end_le_len")
            .map_err(|e| CursedError::codegen(format!("Failed to compare end and len: {}", e)))?;

        // Combined check
        let bounds_ok = builder
            .build_and(start_le_end, end_le_len, "bounds_ok")
            .map_err(|e| CursedError::codegen(format!("Failed to combine bounds checks: {}", e)))?;

        // Create panic block for invalid bounds
        let function = builder
            .get_insert_block()
            .ok_or_else(|| CursedError::codegen("No insert block".to_string()))?
            .get_parent()
            .ok_or_else(|| CursedError::codegen("No parent function".to_string()))?;

        let panic_block = context.append_basic_block(function, "subslice_bounds_panic");
        let continue_block = context.append_basic_block(function, "subslice_bounds_ok");

        builder
            .build_conditional_branch(bounds_ok, continue_block, panic_block)
            .map_err(|e| CursedError::codegen(format!("Failed to build conditional branch: {}", e)))?;

        // Panic block
        builder.position_at_end(panic_block);
        self.emit_panic(context, module, builder, "subslice bounds out of range")?;

        // Continue block
        builder.position_at_end(continue_block);

        Ok(())
    }

    /// Allocate memory for slice with given length
    fn allocate_slice_memory<'ctx>(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        element_type: &Type,
        len: IntValue<'ctx>,
    ) -> Result<PointerValue<'ctx>, CursedError> {
        let element_size = get_type_size(element_type) as u64;
        let element_size_value = context.i64_type().const_int(element_size, false);

        let total_size = builder
            .build_int_mul(len, element_size_value, "total_size")
            .map_err(|e| CursedError::codegen(format!("Failed to calculate total size: {}", e)))?;

        // Get malloc function
        let malloc_fn_type = context.i8_type()
            .ptr_type(AddressSpace::default())
            .fn_type(&[context.i64_type().into()], false);

        let malloc_fn = module.add_function("malloc", malloc_fn_type, None);

        // Call malloc
        let malloc_result = builder
            .build_call(malloc_fn, &[total_size.into()], "malloc_slice")
            .map_err(|e| CursedError::codegen(format!("Failed to call malloc: {}", e)))?;

        // Cast to appropriate type
        let raw_ptr = malloc_result
            .try_as_basic_value()
            .left()
            .ok_or_else(|| CursedError::codegen("Malloc returned void".to_string()))?
            .into_pointer_value();

        let element_llvm_type = convert_type(context, element_type)
            .map_err(|e| CursedError::codegen(format!("Failed to convert element type: {}", e)))?;

        let typed_ptr = builder
            .build_pointer_cast(
                raw_ptr,
                element_llvm_type.ptr_type(AddressSpace::default()),
                "typed_slice_ptr",
            )
            .map_err(|e| CursedError::codegen(format!("Failed to cast pointer: {}", e)))?;

        Ok(typed_ptr)
    }

    /// Copy memory from source slice to destination pointer
    fn copy_slice_memory<'ctx>(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        src_slice: StructValue<'ctx>,
        dst_ptr: PointerValue<'ctx>,
        len: IntValue<'ctx>,
        element_type: &Type,
    ) -> Result<(), CursedError> {
        let src_ptr = builder
            .build_extract_value(src_slice, 0, "src_ptr")
            .map_err(|e| CursedError::codegen(format!("Failed to extract src ptr: {}", e)))?
            .into_pointer_value();

        let element_size = get_type_size(element_type) as u64;
        let element_size_value = context.i64_type().const_int(element_size, false);

        let total_size = builder
            .build_int_mul(len, element_size_value, "copy_size")
            .map_err(|e| CursedError::codegen(format!("Failed to calculate copy size: {}", e)))?;

        // Get memcpy function
        let memcpy_fn_type = context.void_type().fn_type(&[
            context.i8_type().ptr_type(AddressSpace::default()).into(),
            context.i8_type().ptr_type(AddressSpace::default()).into(),
            context.i64_type().into(),
        ], false);

        let memcpy_fn = module.add_function("memcpy", memcpy_fn_type, None);

        // Cast pointers to i8*
        let dst_i8 = builder
            .build_pointer_cast(
                dst_ptr,
                context.i8_type().ptr_type(AddressSpace::default()),
                "dst_i8",
            )
            .map_err(|e| CursedError::codegen(format!("Failed to cast dst pointer: {}", e)))?;

        let src_i8 = builder
            .build_pointer_cast(
                src_ptr,
                context.i8_type().ptr_type(AddressSpace::default()),
                "src_i8",
            )
            .map_err(|e| CursedError::codegen(format!("Failed to cast src pointer: {}", e)))?;

        // Call memcpy
        builder
            .build_call(memcpy_fn, &[dst_i8.into(), src_i8.into(), total_size.into()], "memcpy")
            .map_err(|e| CursedError::codegen(format!("Failed to call memcpy: {}", e)))?;

        Ok(())
    }

    /// Emit a panic with the given message
    fn emit_panic<'ctx>(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        message: &str,
    ) -> Result<(), CursedError> {
        // Get or create panic function
        let panic_fn_type = context.void_type().fn_type(&[
            context.i8_type().ptr_type(AddressSpace::default()).into(),
        ], false);

        let panic_fn = module.add_function("panic", panic_fn_type, None);

        // Create string constant for message
        let message_str = builder.build_global_string_ptr(message, "panic_msg")
            .map_err(|e| CursedError::codegen(format!("Failed to create global string: {}", e)))?;
        
        // Call panic function
        builder
            .build_call(panic_fn, &[message_str.as_pointer_value().into()], "panic_call")
            .map_err(|e| CursedError::codegen(format!("Failed to call panic: {}", e)))?;

        // Unreachable after panic
        builder
            .build_unreachable()
            .map_err(|e| CursedError::codegen(format!("Failed to build unreachable: {}", e)))?;

        Ok(())
    }
}

/// Create a new slice operations instance
pub fn create_slice_operations() -> impl SliceOperations<'static> {
    SliceOperationsImpl
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::type_checker::Type;
    use inkwell::context::Context;

    #[test]
    fn test_slice_operations_creation() {
        let _ops = create_slice_operations();
        // Test that we can create the operations instance
    }

    #[test]
    fn test_slice_len_extraction() {
        let context = Context::create();
        let module = context.create_module("test");
        let builder = context.create_builder();
        let ops = SliceOperationsImpl;

        // Create a function to have a basic block
        let fn_type = context.void_type().fn_type(&[], false);
        let function = module.add_function("test_fn", fn_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);

        // Create a mock slice struct
        let slice_type = context.opaque_struct_type("slice");
        let ptr_type = context.i32_type().ptr_type(AddressSpace::default());
        let len_type = context.i64_type();
        let cap_type = context.i64_type();
        slice_type.set_body(&[ptr_type.into(), len_type.into(), cap_type.into()], false);

        let mock_slice = slice_type.get_undef();
        let len_value = context.i64_type().const_int(42, false);
        let slice_with_len = builder
            .build_insert_value(mock_slice, len_value, 1, "slice_with_len")
            .unwrap()
            .into_struct_value();

        let result = ops.slice_len(&context, &builder, slice_with_len);
        assert!(result.is_ok());
    }
}
