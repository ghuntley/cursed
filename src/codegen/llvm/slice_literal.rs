//! LLVM code generation for slice literal expressions in the CURSED language.
//!
//! This module provides compilation of slice literals to LLVM IR, implementing
//! the slice struct format `{ptr, len, cap}` and handling memory allocation
//! for slice elements.

use crate::ast::expressions::slice_literal::SliceLiteral;
use crate::ast::Expression;
use crate::codegen::llvm::types::{convert_type, get_type_size};
use crate::codegen::llvm::pointer_type_extension::PointerTypeExtension;
use crate::core::type_checker::Type;
use crate::error_enhanced::CursedError;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicTypeEnum, BasicType, StructType};
use inkwell::values::{BasicValueEnum, PointerValue, StructValue};
use inkwell::AddressSpace;
use tracing::{debug, info, instrument, warn};

/// Trait for compiling slice literals to LLVM IR
pub trait SliceLiteralCompiler<'ctx> {
    /// Compile a slice literal expression to LLVM IR
    fn compile_slice_literal(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        slice_literal: &SliceLiteral,
        element_type: &Type,
    ) -> Result<BasicValueEnum<'ctx>, CursedError>;

    /// Create an empty slice of the given element type
    fn create_empty_slice(
        &self,
        context: &'ctx Context,
        builder: &Builder<'ctx>,
        element_type: &Type,
    ) -> Result<StructValue<'ctx>, CursedError>;

    /// Create a slice struct type for the given element type
    fn create_slice_struct_type(
        &self,
        context: &'ctx Context,
        element_type: &Type,
    ) -> Result<StructType<'ctx>, CursedError>;

    /// Allocate memory for slice elements
    fn allocate_slice_memory(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        element_type: &Type,
        element_count: usize,
    ) -> Result<PointerValue<'ctx>, CursedError>;

    /// Populate slice elements in allocated memory
    fn populate_slice_elements(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        element_ptr: PointerValue<'ctx>,
        elements: &[Box<dyn Expression>],
        element_type: &Type,
    ) -> Result<(), CursedError>;
}

/// Implementation of slice literal compilation
pub struct SliceLiteralCompilerImpl;

impl<'ctx> SliceLiteralCompiler<'ctx> for SliceLiteralCompilerImpl {
    #[instrument(skip(self, context, module, builder, slice_literal), level = "debug")]
    fn compile_slice_literal(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        slice_literal: &SliceLiteral,
        element_type: &Type,
    ) -> Result<BasicValueEnum<'ctx>, CursedError> {
        info!("Compiling slice literal with {} elements", slice_literal.len());
        
        // Handle empty slice
        if slice_literal.is_empty() {
            debug!("Creating empty slice");
            let empty_slice = self.create_empty_slice(context, builder, element_type)?;
            return Ok(empty_slice.into());
        }

        let element_count = slice_literal.len();
        debug!("Slice has {} elements", element_count);

        // Create the slice struct type
        let slice_struct_type = self.create_slice_struct_type(context, element_type)?;

        // Allocate memory for the slice elements
        let element_ptr = self.allocate_slice_memory(
            context,
            module,
            builder,
            element_type,
            element_count,
        )?;

        // Populate the allocated memory with elements
        self.populate_slice_elements(
            context,
            module,
            builder,
            element_ptr,
            &slice_literal.elements,
            element_type,
        )?;

        // Create the slice struct {ptr, len, cap}
        let len_value = context.i64_type().const_int(element_count as u64, false);
        let cap_value = context.i64_type().const_int(element_count as u64, false);

        // Build the slice struct
        let slice_struct = slice_struct_type.get_undef();
        let slice_with_ptr = builder
            .build_insert_value(slice_struct, element_ptr, 0, "slice_with_ptr")
            .map_err(|e| CursedError::codegen(format!("Failed to insert ptr: {}", e)))?
            .into_struct_value();

        let slice_with_len = builder
            .build_insert_value(slice_with_ptr, len_value, 1, "slice_with_len")
            .map_err(|e| CursedError::codegen(format!("Failed to insert len: {}", e)))?
            .into_struct_value();

        let slice_complete = builder
            .build_insert_value(slice_with_len, cap_value, 2, "slice_complete")
            .map_err(|e| CursedError::codegen(format!("Failed to insert cap: {}", e)))?
            .into_struct_value();

        debug!("Slice literal compilation completed successfully");
        Ok(slice_complete.into())
    }

    #[instrument(skip(self, context, builder), level = "debug")]
    fn create_empty_slice(
        &self,
        context: &'ctx Context,
        builder: &Builder<'ctx>,
        element_type: &Type,
    ) -> Result<StructValue<'ctx>, CursedError> {
        debug!("Creating empty slice for type: {:?}", element_type);

        let slice_struct_type = self.create_slice_struct_type(context, element_type)?;

        // For empty slice: ptr = null, len = 0, cap = 0
        let element_llvm_type = convert_type(context, element_type)
            .map_err(|e| CursedError::codegen(format!("Failed to convert element type: {}", e)))?;
        
        let null_ptr = element_llvm_type
            .ptr_type(AddressSpace::default())
            .const_null();
        let zero_len = context.i64_type().const_int(0, false);
        let zero_cap = context.i64_type().const_int(0, false);

        // Build the empty slice struct
        let empty_slice = slice_struct_type.get_undef();
        let slice_with_ptr = builder
            .build_insert_value(empty_slice, null_ptr, 0, "empty_slice_ptr")
            .map_err(|e| CursedError::codegen(format!("Failed to insert null ptr: {}", e)))?
            .into_struct_value();

        let slice_with_len = builder
            .build_insert_value(slice_with_ptr, zero_len, 1, "empty_slice_len")
            .map_err(|e| CursedError::codegen(format!("Failed to insert zero len: {}", e)))?
            .into_struct_value();

        let empty_slice_complete = builder
            .build_insert_value(slice_with_len, zero_cap, 2, "empty_slice_cap")
            .map_err(|e| CursedError::codegen(format!("Failed to insert zero cap: {}", e)))?
            .into_struct_value();

        debug!("Empty slice created successfully");
        Ok(empty_slice_complete)
    }

    #[instrument(skip(self, context), level = "debug")]
    fn create_slice_struct_type(
        &self,
        context: &'ctx Context,
        element_type: &Type,
    ) -> Result<StructType<'ctx>, CursedError> {
        debug!("Creating slice struct type for element type: {:?}", element_type);

        let element_llvm_type = convert_type(context, element_type)
            .map_err(|e| CursedError::codegen(format!("Failed to convert element type: {}", e)))?;

        let ptr_type = element_llvm_type.ptr_type(AddressSpace::default());
        let len_type = context.i64_type();
        let cap_type = context.i64_type();

        let slice_struct = context.opaque_struct_type("cursed_slice");
        slice_struct.set_body(&[ptr_type.into(), len_type.into(), cap_type.into()], false);

        debug!("Slice struct type created: {{ptr, len, cap}}");
        Ok(slice_struct)
    }

    #[instrument(skip(self, context, module, builder), level = "debug")]
    fn allocate_slice_memory(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        element_type: &Type,
        element_count: usize,
    ) -> Result<PointerValue<'ctx>, CursedError> {
        debug!("Allocating memory for {} elements of type: {:?}", element_count, element_type);

        let element_llvm_type = convert_type(context, element_type)
            .map_err(|e| CursedError::codegen(format!("Failed to convert element type: {}", e)))?;

        let element_size = get_type_size(element_type);
        let total_size = element_size * element_count;
        debug!("Total memory needed: {} bytes", total_size);

        // Get or create malloc function
        let malloc_fn_type = context.i8_type()
            .ptr_type(AddressSpace::default())
            .fn_type(&[context.i64_type().into()], false);

        let malloc_fn = module.add_function("malloc", malloc_fn_type, None);

        // Call malloc to allocate memory
        let size_value = context.i64_type().const_int(total_size as u64, false);
        let malloc_result = builder
            .build_call(malloc_fn, &[size_value.into()], "malloc_slice")
            .map_err(|e| CursedError::codegen(format!("Failed to call malloc: {}", e)))?;

        // Cast the i8* result to the appropriate element type pointer
        let raw_ptr = malloc_result
            .try_as_basic_value()
            .left()
            .ok_or_else(|| CursedError::codegen("Malloc returned void".to_string()))?
            .into_pointer_value();

        let typed_ptr = builder
            .build_pointer_cast(
                raw_ptr,
                element_llvm_type.ptr_type(AddressSpace::default()),
                "typed_slice_ptr",
            )
            .map_err(|e| CursedError::codegen(format!("Failed to cast pointer: {}", e)))?;

        debug!("Memory allocated successfully for slice elements");
        Ok(typed_ptr)
    }

    #[instrument(skip(self, context, module, builder, element_ptr, elements), level = "debug")]
    fn populate_slice_elements(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        element_ptr: PointerValue<'ctx>,
        elements: &[Box<dyn Expression>],
        element_type: &Type,
    ) -> Result<(), CursedError> {
        debug!("Populating {} slice elements", elements.len());

        for (index, element_expr) in elements.iter().enumerate() {
            debug!("Populating element {} at index {}", element_expr.string(), index);

            // TODO: This would need to call the expression compiler
            // For now, we'll create a placeholder that would be replaced
            // with actual expression compilation in integration
            warn!("Element expression compilation not implemented yet: {}", element_expr.string());

            // Calculate the GEP for this element
            let index_value = context.i32_type().const_int(index as u64, false);
            let element_gep = unsafe {
                builder
                    .build_gep(
                        element_ptr.get_type().get_element_type(),
                        element_ptr,
                        &[index_value],
                        &format!("element_{}_ptr", index),
                    )
                    .map_err(|e| CursedError::codegen(format!("Failed to build GEP for element {}: {}", index, e)))?
            };

            // TODO: Compile the element expression and store it
            // This would require integration with the main expression compiler
            debug!("Element {} GEP calculated, compilation placeholder", index);
        }

        debug!("Slice elements population completed");
        Ok(())
    }
}

/// Create a new slice literal compiler instance
pub fn create_slice_literal_compiler() -> impl SliceLiteralCompiler<'static> {
    SliceLiteralCompilerImpl
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::type_checker::Type;
    use inkwell::context::Context;

    #[test]
    fn test_create_slice_struct_type() {
        let context = Context::create();
        let compiler = SliceLiteralCompilerImpl;
        
        let result = compiler.create_slice_struct_type(&context, &Type::Normie);
        assert!(result.is_ok());
        
        let slice_type = result.unwrap();
        assert_eq!(slice_type.count_fields(), 3); // ptr, len, cap
    }

    #[test]
    fn test_empty_slice_creation() {
        let context = Context::create();
        let module = context.create_module("test");
        let builder = context.create_builder();
        let compiler = SliceLiteralCompilerImpl;

        // Create a function to have a basic block for the builder
        let fn_type = context.void_type().fn_type(&[], false);
        let function = module.add_function("test_fn", fn_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);

        let result = compiler.create_empty_slice(&context, &builder, &Type::Normie);
        assert!(result.is_ok());
    }

    #[test]
    fn test_slice_memory_allocation() {
        let context = Context::create();
        let module = context.create_module("test");
        let builder = context.create_builder();
        let compiler = SliceLiteralCompilerImpl;

        // Create a function to have a basic block for the builder
        let fn_type = context.void_type().fn_type(&[], false);
        let function = module.add_function("test_fn", fn_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);

        let result = compiler.allocate_slice_memory(&context, &module, &builder, &Type::Normie, 5);
        assert!(result.is_ok());
    }
}
