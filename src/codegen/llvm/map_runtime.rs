//! Runtime support functions for map operations
//!
//! This module provides runtime implementation helpers for hash table operations.
//! These functions can be linked into the final executable or provided as intrinsics.

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::types::{BasicType, FunctionType};
use inkwell::values::{FunctionValue, IntValue, PointerValue, BasicValueEnum};
use inkwell::{AddressSpace, IntPredicate};
use crate::core::type_checker::Type;
use crate::error_enhanced::CursedError;
use tracing::{debug, info, instrument};

/// Trait for providing map runtime function implementations
pub trait MapRuntimeProvider<'ctx> {
    /// Get or create hash function for a specific type
    fn get_hash_function(&self, context: &'ctx Context, module: &Module<'ctx>, key_type: &Type) -> Result<FunctionValue<'ctx>, CursedError>;
    
    /// Create runtime implementation for map operations
    fn create_runtime_functions(&self, context: &'ctx Context, module: &Module<'ctx>) -> Result<(), CursedError>;
    
    /// Generate simple hash function implementations
    fn implement_hash_functions(&self, context: &'ctx Context, module: &Module<'ctx>) -> Result<(), CursedError>;
}

/// Basic implementation of map runtime provider
pub struct BasicMapRuntime;

impl BasicMapRuntime {
    pub fn new() -> Self {
        Self
    }
}

impl<'ctx> MapRuntimeProvider<'ctx> for BasicMapRuntime {
    #[instrument(skip(self, context, module), level = "debug")]
    fn get_hash_function(&self, context: &'ctx Context, module: &Module<'ctx>, key_type: &Type) -> Result<FunctionValue<'ctx>, CursedError> {
        let function_name = match key_type {
            Type::Tea => "hash_string",    // Tea is the string type
            Type::Thicc => "hash_i64",     // Thicc is 64-bit int
            Type::Normie => "hash_i32",    // Normie is 32-bit int
            Type::Meal => "hash_f64",      // Meal is 64-bit float
            Type::Lit => "hash_bool",      // Lit is boolean
            _ => return Err(CursedError::codegen(format!("Unsupported key type for hashing: {:?}", key_type))),
        };

        // Get or create the hash function
        if let Some(func) = module.get_function(function_name) {
            return Ok(func);
        }

        debug!("Creating hash function for type: {:?}", key_type);

        // Create function signature
        let hash_type = context.i64_type();
        let key_llvm_type = crate::codegen::llvm::types::convert_type(context, key_type)
            .map_err(|e| CursedError::codegen(format!("Failed to convert key type: {}", e)))?;

        let fn_type = hash_type.fn_type(&[key_llvm_type.into()], false);
        let func = module.add_function(function_name, fn_type, None);

        // For basic implementation, we'll create simple hash functions
        self.implement_simple_hash_function(context, module, func, key_type)?;

        Ok(func)
    }

    fn create_runtime_functions(&self, context: &'ctx Context, module: &Module<'ctx>) -> Result<(), CursedError> {
        debug!("Creating map runtime functions");

        // Add malloc if not present
        if module.get_function("malloc").is_none() {
            let malloc_fn_type = context.i8_type()
                .ptr_type(AddressSpace::default())
                .fn_type(&[context.i64_type().into()], false);
            module.add_function("malloc", malloc_fn_type, None);
        }

        // Add free if not present
        if module.get_function("free").is_none() {
            let free_fn_type = context.void_type()
                .fn_type(&[context.i8_type().ptr_type(AddressSpace::default()).into()], false);
            module.add_function("free", free_fn_type, None);
        }

        // Add realloc if not present
        if module.get_function("realloc").is_none() {
            let realloc_fn_type = context.i8_type()
                .ptr_type(AddressSpace::default())
                .fn_type(&[
                    context.i8_type().ptr_type(AddressSpace::default()).into(),
                    context.i64_type().into(),
                ], false);
            module.add_function("realloc", realloc_fn_type, None);
        }

        // Add memcpy if not present
        if module.get_function("memcpy").is_none() {
            let memcpy_fn_type = context.void_type().fn_type(&[
                context.i8_type().ptr_type(AddressSpace::default()).into(),
                context.i8_type().ptr_type(AddressSpace::default()).into(),
                context.i64_type().into(),
            ], false);
            module.add_function("memcpy", memcpy_fn_type, None);
        }

        debug!("Map runtime functions created successfully");
        Ok(())
    }

    fn implement_hash_functions(&self, context: &'ctx Context, module: &Module<'ctx>) -> Result<(), CursedError> {
        debug!("Implementing hash functions");

        // Create hash functions for common types
        let common_types = vec![Type::Tea, Type::Thicc, Type::Normie, Type::Meal, Type::Lit];

        for key_type in common_types {
            self.get_hash_function(context, module, &key_type)?;
        }

        debug!("Hash functions implemented successfully");
        Ok(())
    }
}

impl BasicMapRuntime {
    /// Implement a simple hash function for a specific type
    fn implement_simple_hash_function<'ctx>(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        func: FunctionValue<'ctx>,
        key_type: &Type,
    ) -> Result<(), CursedError> {
        let builder = context.create_builder();
        let entry_block = context.append_basic_block(func, "entry");
        builder.position_at_end(entry_block);

        let param = func.get_nth_param(0)
            .ok_or_else(|| CursedError::codegen("Hash function must have one parameter".to_string()))?;

        let hash_value = match key_type {
            Type::Tea => {
                // Simple string hash using FNV-1a algorithm
                // param is now a struct value, not a pointer
                self.implement_string_hash_struct(context, &builder, param)?
            }
            Type::Thicc => {
                // Simple integer hash
                self.implement_int_hash(context, &builder, param.into_int_value())?
            }
            Type::Normie => {
                // Cast to i64 and hash
                let extended = builder
                    .build_int_z_extend(param.into_int_value(), context.i64_type(), "ext")
                    .map_err(|e| CursedError::codegen(format!("Failed to extend i32: {}", e)))?;
                self.implement_int_hash(context, &builder, extended)?
            }
            Type::Meal => {
                // Cast float bits to int and hash
                let bits = builder
                    .build_bitcast(param, context.i64_type(), "float_bits")
                    .map_err(|e| CursedError::codegen(format!("Failed to cast float to bits: {}", e)))?
                    .into_int_value();
                self.implement_int_hash(context, &builder, bits)?
            }
            Type::Lit => {
                // Simple boolean hash
                let extended = builder
                    .build_int_z_extend(param.into_int_value(), context.i64_type(), "bool_ext")
                    .map_err(|e| CursedError::codegen(format!("Failed to extend bool: {}", e)))?;
                extended
            }
            _ => return Err(CursedError::codegen(format!("Unsupported hash type: {:?}", key_type))),
        };

        builder
            .build_return(Some(&hash_value))
            .map_err(|e| CursedError::codegen(format!("Failed to build return: {}", e)))?;

        Ok(())
    }

    /// Implement FNV-1a string hash for struct values
    fn implement_string_hash_struct<'ctx>(
        &self,
        context: &'ctx Context,
        builder: &Builder<'ctx>,
        str_struct: BasicValueEnum<'ctx>,
    ) -> Result<IntValue<'ctx>, CursedError> {
        // FNV-1a constants
        let fnv_offset_basis = context.i64_type().const_int(14695981039346656037u64, false);
        let fnv_prime = context.i64_type().const_int(1099511628211u64, false);

        // Extract length and data pointer from the string struct
        let struct_val = str_struct.into_struct_value();
        
        // Get length (first field)
        let length = builder
            .build_extract_value(struct_val, 0, "len")
            .map_err(|e| CursedError::codegen(format!("Failed to extract length: {}", e)))?
            .into_int_value();
        
        // Get data pointer (second field) 
        let data_ptr = builder
            .build_extract_value(struct_val, 1, "data")
            .map_err(|e| CursedError::codegen(format!("Failed to extract data pointer: {}", e)))?
            .into_pointer_value();

        // Simple hash combining length and pointer address
        let ptr_int = builder
            .build_ptr_to_int(data_ptr, context.i64_type(), "ptr_int")
            .map_err(|e| CursedError::codegen(format!("Failed to convert pointer to int: {}", e)))?;

        // Convert length to i64 and XOR with pointer
        let length_ext = builder
            .build_int_z_extend(length, context.i64_type(), "len_ext")
            .map_err(|e| CursedError::codegen(format!("Failed to extend length: {}", e)))?;

        let combined = builder
            .build_xor(ptr_int, length_ext, "ptr_len_xor")
            .map_err(|e| CursedError::codegen(format!("Failed to XOR pointer and length: {}", e)))?;

        let result = builder
            .build_xor(fnv_offset_basis, combined, "simple_hash")
            .map_err(|e| CursedError::codegen(format!("Failed to XOR hash: {}", e)))?;

        Ok(result)
    }

    /// Implement FNV-1a string hash
    fn implement_string_hash<'ctx>(
        &self,
        context: &'ctx Context,
        builder: &Builder<'ctx>,
        str_ptr: PointerValue<'ctx>,
    ) -> Result<IntValue<'ctx>, CursedError> {
        // FNV-1a constants
        let fnv_offset_basis = context.i64_type().const_int(14695981039346656037u64, false);
        let fnv_prime = context.i64_type().const_int(1099511628211u64, false);

        // Get string length (assuming it's stored as {len, ptr} struct)
        // For simplicity, we'll assume null-terminated strings for now
        let hash = builder
            .build_alloca(context.i64_type(), "hash")
            .map_err(|e| CursedError::codegen(format!("Failed to alloca hash: {}", e)))?;

        builder
            .build_store(hash, fnv_offset_basis)
            .map_err(|e| CursedError::codegen(format!("Failed to store initial hash: {}", e)))?;

        // Simple implementation: hash the pointer address as a placeholder
        // TODO: Implement proper string traversal and character hashing
        let ptr_int = builder
            .build_ptr_to_int(str_ptr, context.i64_type(), "ptr_int")
            .map_err(|e| CursedError::codegen(format!("Failed to convert pointer to int: {}", e)))?;

        let result = builder
            .build_xor(fnv_offset_basis, ptr_int, "simple_hash")
            .map_err(|e| CursedError::codegen(format!("Failed to XOR hash: {}", e)))?;

        Ok(result)
    }

    /// Implement simple integer hash
    fn implement_int_hash<'ctx>(
        &self,
        context: &'ctx Context,
        builder: &Builder<'ctx>,
        int_val: IntValue<'ctx>,
    ) -> Result<IntValue<'ctx>, CursedError> {
        // Simple multiplicative hash
        let multiplier = context.i64_type().const_int(11400714819323198485u64, false);
        
        let result = builder
            .build_int_mul(int_val, multiplier, "int_hash")
            .map_err(|e| CursedError::codegen(format!("Failed to multiply for hash: {}", e)))?;

        Ok(result)
    }
}

/// Create a basic map runtime provider
pub fn create_map_runtime() -> impl MapRuntimeProvider<'static> {
    BasicMapRuntime::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;

    #[test]
    fn test_runtime_creation() {
        let _runtime = create_map_runtime();
    }

    #[test]
    fn test_hash_function_creation() {
        let context = Context::create();
        let module = context.create_module("test");
        let runtime = BasicMapRuntime::new();

        let key_type = Type::Tea;
        let result = runtime.get_hash_function(&context, &module, &key_type);
        assert!(result.is_ok());

        let func = result.unwrap();
        assert_eq!(func.get_name().to_str().unwrap(), "hash_string");
    }

    #[test]
    fn test_runtime_functions_creation() {
        let context = Context::create();
        let module = context.create_module("test");
        let runtime = BasicMapRuntime::new();

        let result = runtime.create_runtime_functions(&context, &module);
        assert!(result.is_ok());

        // Verify functions exist
        assert!(module.get_function("malloc").is_some());
        assert!(module.get_function("free").is_some());
        assert!(module.get_function("realloc").is_some());
        assert!(module.get_function("memcpy").is_some());
    }
}
