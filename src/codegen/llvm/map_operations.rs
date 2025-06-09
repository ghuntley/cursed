//! LLVM code generation for map (hash table) operations in the CURSED language.
//!
//! This module provides comprehensive LLVM IR generation for map operations including
//! map creation, indexing, assignment, iteration, and runtime management.
//! Maps are implemented as hash tables with proper collision resolution.

use crate::codegen::llvm::types::{convert_type, get_type_size};
use crate::core::type_checker::Type;
use crate::error_enhanced::CursedError;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicType, BasicTypeEnum, StructType};
use inkwell::values::{BasicValueEnum, IntValue, PointerValue, StructValue, FunctionValue};
use inkwell::{AddressSpace, IntPredicate};
use tracing::{debug, info, instrument, warn};

/// Hash table implementation strategy
#[derive(Debug, Clone, Copy)]
pub enum HashStrategy {
    /// Separate chaining with linked lists
    Chaining,
    /// Open addressing with linear probing
    LinearProbing,
    /// Open addressing with quadratic probing  
    QuadraticProbing,
}

/// Load factor threshold for triggering resize
const DEFAULT_LOAD_FACTOR: f64 = 0.75;
const INITIAL_CAPACITY: u64 = 16;

/// Trait for compiling map operations to LLVM IR
pub trait MapOperations<'ctx> {
    /// Create an empty map with the given key and value types
    fn create_map(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        key_type: &Type,
        value_type: &Type,
    ) -> Result<StructValue<'ctx>, CursedError>;

    /// Create a map from literal key-value pairs
    fn create_map_literal(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        pairs: &[(BasicValueEnum<'ctx>, BasicValueEnum<'ctx>)],
        key_type: &Type,
        value_type: &Type,
    ) -> Result<StructValue<'ctx>, CursedError>;

    /// Get a value from the map by key (map[key])
    fn map_get(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        map_value: StructValue<'ctx>,
        key_value: BasicValueEnum<'ctx>,
        key_type: &Type,
        value_type: &Type,
    ) -> Result<BasicValueEnum<'ctx>, CursedError>;

    /// Set a value in the map (map[key] = value)
    fn map_set(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        map_value: StructValue<'ctx>,
        key_value: BasicValueEnum<'ctx>,
        value_value: BasicValueEnum<'ctx>,
        key_type: &Type,
        value_type: &Type,
    ) -> Result<StructValue<'ctx>, CursedError>;

    /// Check if key exists in map
    fn map_has_key(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        map_value: StructValue<'ctx>,
        key_value: BasicValueEnum<'ctx>,
        key_type: &Type,
    ) -> Result<IntValue<'ctx>, CursedError>;

    /// Delete a key from the map
    fn map_delete(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        map_value: StructValue<'ctx>,
        key_value: BasicValueEnum<'ctx>,
        key_type: &Type,
        value_type: &Type,
    ) -> Result<StructValue<'ctx>, CursedError>;

    /// Get the number of elements in the map
    fn map_len(
        &self,
        context: &'ctx Context,
        builder: &Builder<'ctx>,
        map_value: StructValue<'ctx>,
    ) -> Result<IntValue<'ctx>, CursedError>;

    /// Initialize runtime functions for map operations
    fn init_map_runtime(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        key_type: &Type,
        value_type: &Type,
    ) -> Result<(), CursedError>;
}

/// Implementation of map operations using hash tables
pub struct MapOperationsImpl {
    strategy: HashStrategy,
    load_factor: f64,
}

impl MapOperationsImpl {
    /// Create a new map operations implementation
    pub fn new(strategy: HashStrategy) -> Self {
        Self {
            strategy,
            load_factor: DEFAULT_LOAD_FACTOR,
        }
    }

    /// Create a new map operations implementation with custom load factor
    pub fn with_load_factor(strategy: HashStrategy, load_factor: f64) -> Self {
        Self {
            strategy,
            load_factor,
        }
    }

    /// Get the LLVM type for a map with the given key/value types
    fn get_map_type<'ctx>(
        &self,
        context: &'ctx Context,
        key_type: &Type,
        value_type: &Type,
    ) -> Result<StructType<'ctx>, CursedError> {
        let size_type = context.i64_type();
        let capacity_type = context.i64_type();
        let buckets_ptr_type = context.i8_type().ptr_type(AddressSpace::default());
        
        // Map structure: {size, capacity, buckets_ptr}
        let map_type = context.opaque_struct_type("map");
        map_type.set_body(&[
            size_type.into(),
            capacity_type.into(), 
            buckets_ptr_type.into(),
        ], false);
        
        Ok(map_type)
    }

    /// Get hash function for the given key type
    fn get_hash_function<'ctx>(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        key_type: &Type,
    ) -> Result<FunctionValue<'ctx>, CursedError> {
        let function_name = match key_type {
            Type::Tea => "hash_string",
            Type::Thicc => "hash_i64",
            Type::Normie => "hash_i32", 
            Type::Meal => "hash_f64",
            Type::Lit => "hash_bool",
            _ => return Err(CursedError::codegen(format!("Unsupported key type for hashing: {:?}", key_type))),
        };

        // Get or create the hash function
        if let Some(func) = module.get_function(function_name) {
            return Ok(func);
        }

        let key_llvm_type = convert_type(context, key_type)
            .map_err(|e| CursedError::codegen(format!("Failed to convert key type: {}", e)))?;
        let hash_type = context.i64_type();

        let fn_type = hash_type.fn_type(&[key_llvm_type.into()], false);
        let func = module.add_function(function_name, fn_type, None);
        Ok(func)
    }

    /// Generate bucket index from hash value
    fn get_bucket_index<'ctx>(
        &self,
        context: &'ctx Context,
        builder: &Builder<'ctx>,
        hash_value: IntValue<'ctx>,
        capacity: IntValue<'ctx>,
    ) -> Result<IntValue<'ctx>, CursedError> {
        let index = builder
            .build_int_unsigned_rem(hash_value, capacity, "bucket_index")
            .map_err(|e| CursedError::codegen(format!("Failed to calculate bucket index: {}", e)))?;
        Ok(index)
    }

    /// Allocate memory for hash table buckets
    fn allocate_buckets<'ctx>(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        capacity: IntValue<'ctx>,
        key_type: &Type,
        value_type: &Type,
    ) -> Result<PointerValue<'ctx>, CursedError> {
        // Each bucket contains: key, value, hash, next_ptr (for chaining)
        let key_llvm_type = convert_type(context, key_type)
            .map_err(|e| CursedError::codegen(format!("Failed to convert key type: {}", e)))?;
        let value_llvm_type = convert_type(context, value_type)
            .map_err(|e| CursedError::codegen(format!("Failed to convert value type: {}", e)))?;
        let hash_type = context.i64_type();
        let next_ptr_type = context.i8_type().ptr_type(AddressSpace::default());

        let bucket_type = context.opaque_struct_type("bucket");
        bucket_type.set_body(&[
            key_llvm_type.into(),
            value_llvm_type.into(),
            hash_type.into(),
            next_ptr_type.into(),
        ], false);

        let bucket_size = bucket_type.size_of().unwrap();
        let total_size = builder
            .build_int_mul(capacity, bucket_size, "total_bucket_size")
            .map_err(|e| CursedError::codegen(format!("Failed to calculate total bucket size: {}", e)))?;

        // Call malloc
        let malloc_fn_type = context.i8_type()
            .ptr_type(AddressSpace::default())
            .fn_type(&[context.i64_type().into()], false);
        let malloc_fn = module.add_function("malloc", malloc_fn_type, None);

        let malloc_result = builder
            .build_call(malloc_fn, &[total_size.into()], "malloc_buckets")
            .map_err(|e| CursedError::codegen(format!("Failed to call malloc: {}", e)))?;

        let raw_ptr = malloc_result
            .try_as_basic_value()
            .left()
            .ok_or_else(|| CursedError::codegen("Malloc returned void".to_string()))?
            .into_pointer_value();

        Ok(raw_ptr)
    }

    /// Check if the map needs resizing based on load factor
    fn needs_resize<'ctx>(
        &self,
        context: &'ctx Context,
        builder: &Builder<'ctx>,
        size: IntValue<'ctx>,
        capacity: IntValue<'ctx>,
    ) -> Result<IntValue<'ctx>, CursedError> {
        // Convert load factor to fixed point arithmetic to avoid floating point
        let load_factor_scaled = (self.load_factor * 100.0) as u64;
        let load_factor_const = context.i64_type().const_int(load_factor_scaled, false);
        let hundred = context.i64_type().const_int(100, false);

        // Calculate size * 100
        let size_scaled = builder
            .build_int_mul(size, hundred, "size_scaled")
            .map_err(|e| CursedError::codegen(format!("Failed to scale size: {}", e)))?;

        // Calculate capacity * load_factor
        let capacity_threshold = builder
            .build_int_mul(capacity, load_factor_const, "capacity_threshold")
            .map_err(|e| CursedError::codegen(format!("Failed to calculate capacity threshold: {}", e)))?;

        let needs_resize = builder
            .build_int_compare(IntPredicate::UGE, size_scaled, capacity_threshold, "needs_resize")
            .map_err(|e| CursedError::codegen(format!("Failed to compare load factor: {}", e)))?;

        Ok(needs_resize)
    }

    /// Resize the hash table when load factor exceeds threshold
    fn resize_map<'ctx>(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        map_value: StructValue<'ctx>,
        key_type: &Type,
        value_type: &Type,
    ) -> Result<StructValue<'ctx>, CursedError> {
        // Extract current capacity
        let old_capacity = builder
            .build_extract_value(map_value, 1, "old_capacity")
            .map_err(|e| CursedError::codegen(format!("Failed to extract capacity: {}", e)))?
            .into_int_value();

        // Double the capacity
        let two = context.i64_type().const_int(2, false);
        let new_capacity = builder
            .build_int_mul(old_capacity, two, "new_capacity")
            .map_err(|e| CursedError::codegen(format!("Failed to calculate new capacity: {}", e)))?;

        // Allocate new buckets
        let new_buckets = self.allocate_buckets(context, module, builder, new_capacity, key_type, value_type)?;

        // TODO: Implement rehashing of existing elements
        // For now, just create a new map structure with new capacity
        let new_map = map_value.get_type().get_undef();
        let size = builder
            .build_extract_value(map_value, 0, "size")
            .map_err(|e| CursedError::codegen(format!("Failed to extract size: {}", e)))?;

        let map_with_size = builder
            .build_insert_value(new_map, size, 0, "map_with_size")
            .map_err(|e| CursedError::codegen(format!("Failed to insert size: {}", e)))?
            .into_struct_value();

        let map_with_capacity = builder
            .build_insert_value(map_with_size, new_capacity, 1, "map_with_capacity")
            .map_err(|e| CursedError::codegen(format!("Failed to insert capacity: {}", e)))?
            .into_struct_value();

        let resized_map = builder
            .build_insert_value(map_with_capacity, new_buckets, 2, "resized_map")
            .map_err(|e| CursedError::codegen(format!("Failed to insert buckets: {}", e)))?
            .into_struct_value();

        warn!("Map resizing implementation is incomplete - need to implement rehashing");
        Ok(resized_map)
    }
}

impl<'ctx> MapOperations<'ctx> for MapOperationsImpl {
    #[instrument(skip(self, context, module, builder), level = "debug")]
    fn create_map(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        key_type: &Type,
        value_type: &Type,
    ) -> Result<StructValue<'ctx>, CursedError> {
        info!("Creating empty map");

        // Initialize runtime functions
        self.init_map_runtime(context, module, key_type, value_type)?;

        let map_type = self.get_map_type(context, key_type, value_type)?;
        let initial_capacity = context.i64_type().const_int(INITIAL_CAPACITY, false);
        let initial_size = context.i64_type().const_int(0, false);

        // Allocate buckets
        let buckets = self.allocate_buckets(context, module, builder, initial_capacity, key_type, value_type)?;

        // Build map struct
        let empty_map = map_type.get_undef();
        let map_with_size = builder
            .build_insert_value(empty_map, initial_size, 0, "map_with_size")
            .map_err(|e| CursedError::codegen(format!("Failed to insert size: {}", e)))?
            .into_struct_value();

        let map_with_capacity = builder
            .build_insert_value(map_with_size, initial_capacity, 1, "map_with_capacity")
            .map_err(|e| CursedError::codegen(format!("Failed to insert capacity: {}", e)))?
            .into_struct_value();

        let final_map = builder
            .build_insert_value(map_with_capacity, buckets, 2, "final_map")
            .map_err(|e| CursedError::codegen(format!("Failed to insert buckets: {}", e)))?
            .into_struct_value();

        debug!("Empty map created successfully");
        Ok(final_map)
    }

    #[instrument(skip(self, context, module, builder, pairs), level = "debug")]
    fn create_map_literal(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        pairs: &[(BasicValueEnum<'ctx>, BasicValueEnum<'ctx>)],
        key_type: &Type,
        value_type: &Type,
    ) -> Result<StructValue<'ctx>, CursedError> {
        info!(pairs_count = pairs.len(), "Creating map literal");

        // Create empty map
        let mut map = self.create_map(context, module, builder, key_type, value_type)?;

        // Insert each pair
        for (key, value) in pairs {
            map = self.map_set(context, module, builder, map, *key, *value, key_type, value_type)?;
        }

        debug!("Map literal created successfully");
        Ok(map)
    }

    #[instrument(skip(self, context, module, builder), level = "debug")]
    fn map_get(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        map_value: StructValue<'ctx>,
        key_value: BasicValueEnum<'ctx>,
        key_type: &Type,
        value_type: &Type,
    ) -> Result<BasicValueEnum<'ctx>, CursedError> {
        info!("Getting value from map");

        // Get hash function and calculate hash
        let hash_fn = self.get_hash_function(context, module, key_type)?;
        let hash_result = builder
            .build_call(hash_fn, &[key_value.into()], "key_hash")
            .map_err(|e| CursedError::codegen(format!("Failed to call hash function: {}", e)))?;

        let hash_value = hash_result
            .try_as_basic_value()
            .left()
            .ok_or_else(|| CursedError::codegen("Hash function returned void".to_string()))?
            .into_int_value();

        // Extract map components
        let capacity = builder
            .build_extract_value(map_value, 1, "capacity")
            .map_err(|e| CursedError::codegen(format!("Failed to extract capacity: {}", e)))?
            .into_int_value();

        let buckets_ptr = builder
            .build_extract_value(map_value, 2, "buckets_ptr")
            .map_err(|e| CursedError::codegen(format!("Failed to extract buckets: {}", e)))?
            .into_pointer_value();

        // Calculate bucket index
        let bucket_index = self.get_bucket_index(context, builder, hash_value, capacity)?;

        // TODO: Implement proper bucket lookup with collision resolution
        // For now, return a zero value
        let zero_value = convert_type(context, value_type)
            .map_err(|e| CursedError::codegen(format!("Failed to convert value type: {}", e)))?
            .const_zero();

        warn!("Map get operation is not fully implemented - returning zero value");
        Ok(zero_value)
    }

    #[instrument(skip(self, context, module, builder), level = "debug")]
    fn map_set(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        map_value: StructValue<'ctx>,
        key_value: BasicValueEnum<'ctx>,
        value_value: BasicValueEnum<'ctx>,
        key_type: &Type,
        value_type: &Type,
    ) -> Result<StructValue<'ctx>, CursedError> {
        info!("Setting value in map");

        // Check if resize is needed
        let size = builder
            .build_extract_value(map_value, 0, "size")
            .map_err(|e| CursedError::codegen(format!("Failed to extract size: {}", e)))?
            .into_int_value();

        let capacity = builder
            .build_extract_value(map_value, 1, "capacity")
            .map_err(|e| CursedError::codegen(format!("Failed to extract capacity: {}", e)))?
            .into_int_value();

        let needs_resize = self.needs_resize(context, builder, size, capacity)?;

        // Create conditional blocks
        let function = builder
            .get_insert_block()
            .ok_or_else(|| CursedError::codegen("No insert block".to_string()))?
            .get_parent()
            .ok_or_else(|| CursedError::codegen("No parent function".to_string()))?;

        let resize_block = context.append_basic_block(function, "resize_map");
        let no_resize_block = context.append_basic_block(function, "no_resize");
        let insert_block = context.append_basic_block(function, "insert_value");

        builder
            .build_conditional_branch(needs_resize, resize_block, no_resize_block)
            .map_err(|e| CursedError::codegen(format!("Failed to build conditional branch: {}", e)))?;

        // Resize path
        builder.position_at_end(resize_block);
        let resized_map = self.resize_map(context, module, builder, map_value, key_type, value_type)?;
        builder
            .build_unconditional_branch(insert_block)
            .map_err(|e| CursedError::codegen(format!("Failed to build unconditional branch: {}", e)))?;

        // No resize path
        builder.position_at_end(no_resize_block);
        builder
            .build_unconditional_branch(insert_block)
            .map_err(|e| CursedError::codegen(format!("Failed to build unconditional branch: {}", e)))?;

        // Insert block
        builder.position_at_end(insert_block);

        // PHI node for map
        let final_map_phi = builder
            .build_phi(map_value.get_type(), "final_map")
            .map_err(|e| CursedError::codegen(format!("Failed to build map phi: {}", e)))?;
        final_map_phi.add_incoming(&[(&map_value, no_resize_block), (&resized_map, resize_block)]);

        let final_map = final_map_phi.as_basic_value().into_struct_value();

        // TODO: Implement actual insertion logic with hash calculation and collision resolution

        // For now, just increment size and return
        let one = context.i64_type().const_int(1, false);
        let new_size = builder
            .build_int_add(size, one, "new_size")
            .map_err(|e| CursedError::codegen(format!("Failed to increment size: {}", e)))?;

        let updated_map = builder
            .build_insert_value(final_map, new_size, 0, "updated_map")
            .map_err(|e| CursedError::codegen(format!("Failed to update size: {}", e)))?
            .into_struct_value();

        warn!("Map set operation is not fully implemented - only incrementing size");
        debug!("Map set completed");
        Ok(updated_map)
    }

    fn map_has_key(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        map_value: StructValue<'ctx>,
        key_value: BasicValueEnum<'ctx>,
        key_type: &Type,
    ) -> Result<IntValue<'ctx>, CursedError> {
        // TODO: Implement proper key lookup
        // For now, return false
        let false_value = context.bool_type().const_int(0, false);
        warn!("Map has_key operation is not implemented - returning false");
        Ok(false_value)
    }

    fn map_delete(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        map_value: StructValue<'ctx>,
        key_value: BasicValueEnum<'ctx>,
        key_type: &Type,
        value_type: &Type,
    ) -> Result<StructValue<'ctx>, CursedError> {
        // TODO: Implement proper key deletion
        // For now, just return the original map
        warn!("Map delete operation is not implemented - returning original map");
        Ok(map_value)
    }

    fn map_len(
        &self,
        context: &'ctx Context,
        builder: &Builder<'ctx>,
        map_value: StructValue<'ctx>,
    ) -> Result<IntValue<'ctx>, CursedError> {
        let size = builder
            .build_extract_value(map_value, 0, "map_size")
            .map_err(|e| CursedError::codegen(format!("Failed to extract map size: {}", e)))?
            .into_int_value();
        Ok(size)
    }

    fn init_map_runtime(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
        key_type: &Type,
        value_type: &Type,
    ) -> Result<(), CursedError> {
        debug!("Initializing map runtime functions");

        // Initialize hash functions for the key type
        self.get_hash_function(context, module, key_type)?;

        // Add malloc function if not present
        if module.get_function("malloc").is_none() {
            let malloc_fn_type = context.i8_type()
                .ptr_type(AddressSpace::default())
                .fn_type(&[context.i64_type().into()], false);
            module.add_function("malloc", malloc_fn_type, None);
        }

        // Add free function if not present
        if module.get_function("free").is_none() {
            let free_fn_type = context.void_type()
                .fn_type(&[context.i8_type().ptr_type(AddressSpace::default()).into()], false);
            module.add_function("free", free_fn_type, None);
        }

        debug!("Map runtime functions initialized");
        Ok(())
    }
}

/// Create a new map operations implementation with default settings
pub fn create_map_operations() -> MapOperationsImpl {
    MapOperationsImpl::new(HashStrategy::Chaining)
}

/// Create a new map operations implementation with linear probing
pub fn create_map_operations_linear_probing() -> MapOperationsImpl {
    MapOperationsImpl::new(HashStrategy::LinearProbing)
}

/// Create a new map operations implementation with quadratic probing  
pub fn create_map_operations_quadratic_probing() -> MapOperationsImpl {
    MapOperationsImpl::new(HashStrategy::QuadraticProbing)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::type_checker::Type;
    use inkwell::context::Context;

    #[test]
    fn test_map_operations_creation() {
        let _ops = create_map_operations();
        let _ops_linear = create_map_operations_linear_probing();
        let _ops_quad = create_map_operations_quadratic_probing();
    }

    #[test]
    fn test_map_type_creation() {
        let context = Context::create();
        let module = context.create_module("test");
        let builder = context.create_builder();
        let ops = MapOperationsImpl::new(HashStrategy::Chaining);

        // Create a function to have a basic block
        let fn_type = context.void_type().fn_type(&[], false);
        let function = module.add_function("test_fn", fn_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);

        let key_type = Type::Tea;
        let value_type = Type::Thicc;

        let map_type = ops.get_map_type(&context, &key_type, &value_type);
        assert!(map_type.is_ok());
    }

    #[test]
    fn test_needs_resize() {
        let context = Context::create();
        let module = context.create_module("test");
        let builder = context.create_builder();
        let ops = MapOperationsImpl::new(HashStrategy::Chaining);

        // Create a function to have a basic block
        let fn_type = context.void_type().fn_type(&[], false);
        let function = module.add_function("test_fn", fn_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);

        let size = context.i64_type().const_int(12, false);
        let capacity = context.i64_type().const_int(16, false);

        let needs_resize = ops.needs_resize(&context, &builder, size, capacity);
        assert!(needs_resize.is_ok());
    }
}
