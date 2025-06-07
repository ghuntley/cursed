//! Enhanced implementation of interface type assertions
//!
//! This module provides an optimized implementation of interface type assertions
//! with improved error handling, caching, and performance optimizations.

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::basic_block::BasicBlock;
use inkwell::values::{BasicValueEnum, IntValue, PointerValue};
use inkwell::types::{BasicTypeEnum, StructType};
use inkwell::IntPredicate;
use inkwell::AddressSpace;

use crate::ast::expressions::TypeAssertion;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::expression::ExpressionCompilation;
use crate::codegen::llvm::interface_type_assertion_errors::TypeAssertionErrorHandler;
use crate::error::Error;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

/// Cache for type ID lookups to improve performance
pub struct TypeIdCache {
    /// Maps type names to their computed type IDs
    type_ids: HashMap<String, u64>,
}

impl TypeIdCache {
    /// Create a new type ID cache
    pub fn new() -> Self {
        Self {
            type_ids: HashMap::new(),
        }
    }
    
    /// Get or compute a type ID for a given type name
    pub fn get_type_id(&mut self, type_name: &str) -> u64 {
        if let Some(id) = self.type_ids.get(type_name) {
            return *id;
        }
        
        // Compute new type ID and cache it
        let id = self.compute_type_id(type_name);
        self.type_ids.insert(type_name.to_string(), id);
        id
    }
    
    /// Compute a type ID using a consistent hash algorithm
    fn compute_type_id(&self, type_name: &str) -> u64 {
        // FNV-1a hash algorithm
        let mut hash: u64 = 0xcbf29ce484222325;
        for byte in type_name.bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash
    }
}

/// Enhanced trait for optimized interface type assertions
pub trait EnhancedTypeAssertion<'ctx> {
    /// Compile a type assertion with enhanced error handling and optimizations
    fn compile_enhanced_type_assertion(
        &mut self,
        type_assertion: &TypeAssertion,
        source_file: Option<&str>,
        source_line: Option<u32>,
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Check if an interface value is of a specific concrete type with caching
    fn check_instance_of_cached(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type_name: &str,
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Extract type information from an interface value with detailed error handling
    fn extract_type_info(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
    ) -> Result<(BasicValueEnum<'ctx>, PointerValue<'ctx>), Error>;
    
    /// Get the type ID for a type name using the cache
    fn get_cached_type_id(&mut self, type_name: &str) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> EnhancedTypeAssertion<'ctx> for LlvmCodeGenerator<'ctx> {
    fn compile_enhanced_type_assertion(
        &mut self,
        type_assertion: &TypeAssertion,
        source_file: Option<&str>,
        source_line: Option<u32>,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Initialize the type ID cache if needed
        if self.type_id_cache.is_none() {
            self.type_id_cache = Some(Arc::new(RwLock::new(TypeIdCache::new())));
        }
        
        // Get the current function
        let current_fn = self.current_function()
            .ok_or_else(|| Error::Compilation("No current function for type assertion".to_string()))?;
        
        // Compile the expression being asserted
        let expr_value = self.compile_expression(type_assertion.expression.as_ref())?;
        
        // Create basic blocks for success and failure paths
        let success_block = self.context().append_basic_block(current_fn, "type_assert_success");
        let failure_block = self.context().append_basic_block(current_fn, "type_assert_failure");
        let merge_block = self.context().append_basic_block(current_fn, "type_assert_merge");
        
        // Check if the interface value is of the target type using the cached version
        let is_instance = self.check_instance_of_cached(expr_value, &type_assertion.type_name)?;
        
        // Branch based on the type check result
        self.builder().build_conditional_branch(
            is_instance.into_int_value(),
            success_block,
            failure_block
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Success path - extract and cast the data pointer
        self.builder().position_at_end(success_block);
        
        // Extract type info and data pointer
        let (_, data_ptr) = self.extract_type_info(expr_value)?;
        
        // Create the result structure (value and true flag)
        let casted_ptr = self.builder().build_bitcast(
            data_ptr,
            self.context().i8_type().ptr_type(AddressSpace::default()),
            "casted_ptr"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Pack the result into a tuple structure
        let true_val = self.context().bool_type().const_int(1, false);
        let success_result = self.build_enhanced_tuple(vec![casted_ptr.into(), true_val.into()])?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Failure path - log error and return null pointer with false flag
        self.builder().position_at_end(failure_block);
        
        // Extract type info for error reporting
        let (type_id, value_ptr) = self.extract_type_info(expr_value)?;
        

        

        
        // Log the error with source information
        let source_location = source_file.and_then(|file| {
            source_line.map(|line| (file, line))
        });
        
        // Generate error logging directly
        self.generate_type_error_logging(
            &type_assertion.type_name,
            type_id,
            value_ptr,
            source_location,
        )?;
        
        // Create the failure result by calling our helper method without mutable borrow issues
        let failure_result = self.build_failure_tuple()?;
        
        // Branch to merge block
        self.builder().build_unconditional_branch(merge_block)
            .map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Merge block - use phi node to select the appropriate result
        self.builder().position_at_end(merge_block);
        let result_type = self.enhanced_tuple_type(vec![self.context().i8_type().ptr_type(AddressSpace::default()).into(), self.context().bool_type().into()]);
        let phi = self.builder().build_phi(
            result_type,
            "assertion_result"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        phi.add_incoming(&[(
            &success_result,
            success_block
        ), (
            &failure_result,
            failure_block
        )]);
        
        // Return the phi result
        Ok(phi.as_basic_value())
    }
    
    fn check_instance_of_cached(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type_name: &str,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Get the type ID from the interface value's vtable
        let (actual_type_id, _) = self.extract_type_info(interface_value)?;
        
        // Get the expected type ID for the target type using the cache
        let expected_type_id = self.get_cached_type_id(target_type_name)?;
        
        // Compare the type IDs
        let result = self.builder().build_int_compare(
            IntPredicate::EQ,
            actual_type_id.into_int_value(),
            expected_type_id.into_int_value(),
            "is_instance_of"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Add debug instrumentation
        self.add_type_check_debug_info(target_type_name, actual_type_id, result.into())?;
        
        Ok(result.into())
    }
    
    fn extract_type_info(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
    ) -> Result<(BasicValueEnum<'ctx>, PointerValue<'ctx>), Error> {
        // Extract the data pointer and vtable pointer
        let (data_ptr, vtable_ptr) = if interface_value.is_struct_value() {
            // Direct interface value
            let data_ptr = self.builder().build_extract_value(
                interface_value.into_struct_value(),
                0, // Index of data pointer
                "data_ptr"
            ).map_err(|e| Error::Compilation(e.to_string()))?;
            
            let vtable_ptr = self.builder().build_extract_value(
                interface_value.into_struct_value(),
                1, // Index of vtable pointer
                "vtable_ptr"
            ).map_err(|e| Error::Compilation(e.to_string()))?;
            
            (data_ptr.into_pointer_value(), vtable_ptr.into_pointer_value())
        } else if interface_value.is_pointer_value() {
            // Pointer to interface value
            let loaded = self.builder().build_load(
                interface_value.get_type(),
                interface_value.into_pointer_value(),
                "interface_value"
            ).map_err(|e| Error::Compilation(e.to_string()))?;
            
            let data_ptr = self.builder().build_extract_value(
                loaded.into_struct_value(),
                0, // Index of data pointer
                "data_ptr"
            ).map_err(|e| Error::Compilation(e.to_string()))?;
            
            let vtable_ptr = self.builder().build_extract_value(
                loaded.into_struct_value(),
                1, // Index of vtable pointer
                "vtable_ptr"
            ).map_err(|e| Error::Compilation(e.to_string()))?;
            
            (data_ptr.into_pointer_value(), vtable_ptr.into_pointer_value())
        } else {
            return Err(Error::Compilation(format!(
                "Expected interface value or pointer, got {:?}",
                interface_value
            )));
        };
        
        // Type ID is the first field in the vtable
        let type_id_ptr = self.builder().build_struct_gep(
            // Create and use a dummy struct type since we can't get the pointee type directly
            self.context.struct_type(&[], false),
            vtable_ptr,
            0, // Index of type ID pointer
            "type_id_ptr"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Load the type ID
        let type_id = self.builder().build_load(
            self.context().i64_type(),
            type_id_ptr,
            "type_id"
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        Ok((type_id, data_ptr))
    }
    
    fn get_cached_type_id(&mut self, type_name: &str) -> Result<BasicValueEnum<'ctx>, Error> {
        // Get the type ID from the cache
        let type_id = match &self.type_id_cache {
            Some(cache) => cache.write().unwrap().get_type_id(type_name),
            None => {
                // Initialize cache
                let cache = Arc::new(RwLock::new(TypeIdCache::new()));
                let id = cache.write().unwrap().get_type_id(type_name);
                self.type_id_cache = Some(cache);
                id
            }
        };
        
        // Create LLVM constant for the type ID
        let type_id_const = self.context().i64_type().const_int(type_id, false);
        Ok(type_id_const.into())
    }
}

// Extension methods for debugging and optimization
impl<'ctx> LlvmCodeGenerator<'ctx> {
    // Create a failure tuple for a type assertion that didn't match
    fn build_failure_tuple(&self) -> Result<BasicValueEnum<'ctx>, Error> {
        // Create necessary constants
        let null_ptr = self.context().i8_type().ptr_type(AddressSpace::default()).const_null();
        let false_val = self.context().bool_type().const_int(0, false);
        
        // Create tuple type
        let tuple_type = self.context().struct_type(
            &[null_ptr.get_type().into(), false_val.get_type().into()],
            false
        );
        
        // Build the tuple using insert_value operations
        let mut tuple = tuple_type.const_named_struct(&[]);
        
        // Insert the null pointer (data pointer)
        tuple = self.builder().build_insert_value::<inkwell::values::StructValue<'_>, inkwell::values::PointerValue<'_>>(
            tuple, 
            null_ptr, 
            0, 
            "failure_ptr_insert"
        ).map_err(|e| Error::Compilation(e.to_string()))?.into_struct_value();
        
        // Insert the false flag (match success flag)
        tuple = self.builder().build_insert_value::<inkwell::values::StructValue<'_>, inkwell::values::IntValue<'_>>(
            tuple, 
            false_val, 
            1, 
            "failure_flag_insert"
        ).map_err(|e| Error::Compilation(e.to_string()))?.into_struct_value();
        
        Ok(tuple.into())
    }
    // Generate error logging for type assertion failures
    fn generate_type_error_logging(
        &self,
        expected_type: &str,
        actual_type_id: BasicValueEnum<'ctx>,
        value_ptr: PointerValue<'ctx>,
        source_location: Option<(&str, u32)>,
    ) -> Result<(), Error> {
        // Create a global string constant for the error message template
        let error_msg = format!(
            "Type assertion failed: expected {}, but got %s",
            expected_type
        );
        
        let error_msg_global = self.builder()
            .build_global_string_ptr(&error_msg, "type_assertion_error_msg")
            .map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Create string constant for expected type
        let expected_type_global = self.builder()
            .build_global_string_ptr(expected_type, "expected_type")
            .map_err(|e| Error::Compilation(e.to_string()))?;
            
        // Find or create external function to log type assertion errors
        let log_func_type = self.context.void_type().fn_type(
            &[
                self.context.i8_type().ptr_type(AddressSpace::default()).into(), // error_msg
                self.context.i8_type().ptr_type(AddressSpace::default()).into(), // expected_type
                self.context.i64_type().into(),                                  // actual_type_id
                self.context.i8_type().ptr_type(AddressSpace::default()).into(), // value_ptr
                self.context.i8_type().ptr_type(AddressSpace::default()).into(), // source_file
                self.context.i32_type().into(),                                  // source_line
            ],
            false,
        );
        
        let log_func = self.module().add_function(
            "__cursed_log_type_assertion_error",
            log_func_type,
            None,
        );
        
        // Create source location constants
        let (source_file, source_line) = match source_location {
            Some((file, line)) => {
                let file_global = self.builder()
                    .build_global_string_ptr(file, "source_file")
                    .map_err(|e| Error::Compilation(e.to_string()))?;
                let line_const = self.context.i32_type().const_int(line as u64, false);
                (file_global, line_const)
            },
            None => {
                let file_global = self.builder()
                    .build_global_string_ptr("<unknown>", "source_file")
                    .map_err(|e| Error::Compilation(e.to_string()))?;
                let line_const = self.context.i32_type().const_int(0, false);
                (file_global, line_const)
            },
        };
        
        // Call the logging function
        self.builder().build_call(
            log_func,
            &[
                error_msg_global.as_pointer_value().into(),
                expected_type_global.as_pointer_value().into(),
                actual_type_id.into(),
                value_ptr.into(),
                source_file.as_pointer_value().into(),
                source_line.into(),
            ],
            "log_assertion_error",
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        Ok(())
    }
    // Build a tuple structure (for returning value and success flag)
    fn build_enhanced_tuple(&mut self, values: Vec<BasicValueEnum<'ctx>>) -> Result<BasicValueEnum<'ctx>, Error> {
        let ctx = self.context();
        let tuple_type = ctx.struct_type(
            &values.iter().map(|v| v.get_type()).collect::<Vec<_>>(),
            false
        );
        
        let mut tuple = tuple_type.const_named_struct(&[]);
        for (i, value) in values.iter().enumerate() {
            let inserted = self.builder().build_insert_value(tuple, *value, i as u32, "tuple_insert")
                .map_err(|e| Error::Compilation(e.to_string()))?;
            tuple = inserted.into_struct_value();
        }
        
        Ok(tuple.into())
    }
    
    // Get tuple type from a list of element types
    fn enhanced_tuple_type(&self, element_types: Vec<BasicTypeEnum<'ctx>>) -> StructType<'ctx> {
        self.context().struct_type(&element_types, false)
    }
    
    // Helper for getting pointer type with default address space
    fn enhanced_pointer_type(&self) -> inkwell::types::PointerType<'ctx> {
        self.context().i8_type().ptr_type(AddressSpace::default())
    }
    
    // Add debug information for type checking
    fn add_type_check_debug_info(
        &mut self,
        expected_type: &str,
        actual_type_id: BasicValueEnum<'ctx>,
        is_match: BasicValueEnum<'ctx>,
    ) -> Result<(), Error> {
        // Only add debug info in debug mode
        if !self.debug_enabled() {
            return Ok(());
        }
        
        // Create debug print function
        let debug_func_type = self.context().void_type().fn_type(
        &[
        self.context().i8_type().ptr_type(AddressSpace::default()).into(), // expected type
        self.context().i64_type().into(),                                  // actual type ID
        self.context().bool_type().into(),                                 // is match
        ],
        false,
        );
        
        let debug_func = self.module().add_function(
        "__cursed_debug_type_check",
        debug_func_type,
        None,
        );
        
        // Create string constant for expected type
        let expected_type_global = self.builder()
        .build_global_string_ptr(expected_type, "expected_type_debug")
        .map_err(|e| Error::Compilation(e.to_string()))?;
        
        // Call the debug function
        self.builder().build_call(
        debug_func,
        &[
        expected_type_global.as_pointer_value().into(),
        actual_type_id.into(),
        is_match.into(),
        ],
        "debug_type_check",
        ).map_err(|e| Error::Compilation(e.to_string()))?;
        
        Ok(())
    }
    
    // Check if debug mode is enabled
    fn debug_enabled(&self) -> bool {
        // Check environment or build configuration
        option_env!("CURSED_DEBUG").is_some()
    }
}

// Debug support extension for LlvmCodeGenerator
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Set the type ID cache for reuse
    pub fn with_type_id_cache(mut self, cache: Arc<RwLock<TypeIdCache>>) -> Self {
        self.type_id_cache = Some(cache);
        self
    }
    
    /// Get the type ID cache
    pub fn type_id_cache(&self) -> Option<Arc<RwLock<TypeIdCache>>> {
        self.type_id_cache.clone()
    }
}

/// Runtime handler for type assertion debug info
#[no_mangle]
pub extern "C" fn __cursed_debug_type_check(
    expected_type: *const i8,
    actual_type_id: u64,
    is_match: bool,
) {
    use std::ffi::CStr;
    
    // Only log in debug builds
    if option_env!("CURSED_DEBUG").is_none() {
        return;
    }
    
    // Convert C string to Rust string
    let expected = unsafe {
        CStr::from_ptr(expected_type).to_string_lossy().into_owned()
    };
    
    if is_match {
        tracing::debug!(
            expected_type = %expected,
            actual_type_id = %format!("{:x}", actual_type_id),
            "Type assertion succeeded"
        );
    } else {
        tracing::debug!(
            expected_type = %expected,
            actual_type_id = %format!("{:x}", actual_type_id),
            "Type assertion failed"
        );
    }
}