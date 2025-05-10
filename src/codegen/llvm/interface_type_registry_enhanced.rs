//! # Enhanced Interface Type Registry
//!
//! This module provides a comprehensive runtime type information system for interface type assertions.
//! It maintains a global registry of type names and IDs that can be accessed at runtime to provide
//! detailed error messages and debugging information.
//!
//! ## Key improvements:
//!
//! - Global arrays for type IDs and names properly initialized with LLVM pointer manipulation
//! - Full integration with error reporting system
//! - Human-readable type names in all error messages
//! - Enhanced diagnostics with both expected and actual type information
//! - Proper error handling and fallback mechanisms for type information
//! - Expanded structured logging

use inkwell::AddressSpace;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{ArrayType, BasicTypeEnum, PointerType, StructType};
use inkwell::values::{ArrayValue, BasicValueEnum, GlobalValue, PointerValue};
use inkwell::IntPredicate;

use crate::codegen::llvm::type_assertion::InterfaceTypeAssertion;

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

use crate::codegen::llvm::LlvmCodeGenerator;
use crate::error::Error;
use tracing::{debug, error, info, instrument, span, warn, Level};

/// Maximum number of types that can be registered in the global type registry
const MAX_TYPE_REGISTRY_SIZE: usize = 1024;

/// Type representing a registry ID
pub type TypeRegistryId = u64;

/// Enhanced registry for interface type information
pub trait EnhancedTypeRegistry<'ctx> {
    /// Register a new type with the registry, returning its unique ID
    fn register_type(&mut self, type_name: &str) -> Result<TypeRegistryId, Error>;
    
    /// Look up a type name from its ID
    fn lookup_type_name(&self, type_id: TypeRegistryId) -> Option<String>;
    
    /// Get the ID for a registered type
    fn get_type_id(&self, type_name: &str) -> Result<TypeRegistryId, Error>;
    
    /// Initialize the global type registry arrays in LLVM IR
    fn initialize_global_type_registry(&mut self) -> Result<(), Error>;
    
    /// Add a type to the global registry arrays
    fn add_type_to_global_registry(&mut self, type_id: TypeRegistryId, type_name: &str) -> Result<(), Error>;
    
    /// Get access to the global type name array
    fn get_global_type_names_array(&self) -> Result<PointerValue<'ctx>, Error>;
    
    /// Get a runtime type name string from a type ID
    fn get_runtime_type_name(&mut self, type_id: BasicValueEnum<'ctx>) -> Result<PointerValue<'ctx>, Error>;
    
    /// Create a debug message for type assertion failure
    fn create_type_assertion_error(
        &mut self,
        actual_type_id: BasicValueEnum<'ctx>,
        expected_type_name: &str
    ) -> Result<PointerValue<'ctx>, Error>;
}

impl<'ctx> EnhancedTypeRegistry<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn register_type(&mut self, type_name: &str) -> Result<TypeRegistryId, Error> {
        // Initialize the registry if needed
        if self.interface_type_registry.is_none() {
            debug!("Creating new interface type registry");
            self.interface_type_registry = Some(Arc::new(RwLock::new(HashMap::new())));
            
            // Pre-register any built-in types
            let registry = self.interface_type_registry.as_ref().unwrap();
            let mut registry = registry.write().map_err(|e| {
                Error::Compilation(format!("Failed to acquire write lock on interface registry: {}", e))
            })?;
            
            // Add basic types with reserved IDs
            registry.insert("Normie".to_string(), 1);  // int8
            registry.insert("Thiccie".to_string(), 2); // int16
            registry.insert("Lit".to_string(), 3);     // int32
            registry.insert("Thicc".to_string(), 4);   // int64
            registry.insert("Tea".to_string(), 5);     // string
            registry.insert("Mood".to_string(), 6);    // bool
            registry.insert("Snack".to_string(), 7);   // float32
            registry.insert("Meal".to_string(), 8);    // float64
            registry.insert("any".to_string(), 9);     // interface{}
            registry.insert("<null>".to_string(), 0);  // null/nil
        }
        
        // Check if the type is already registered
        let registry = self.interface_type_registry.as_ref().unwrap();
        let registry_read = registry.read().map_err(|e| {
            Error::Compilation(format!("Failed to acquire read lock on interface registry: {}", e))
        })?;
        
        if let Some(id) = registry_read.get(type_name) {
            debug!("Type {} already registered with ID {}", type_name, id);
            return Ok(*id);
        }
        
        // Type not found, we need to register it with a new ID
        std::mem::drop(registry_read); // Release the read lock
        
        let mut registry_write = registry.write().map_err(|e| {
            Error::Compilation(format!("Failed to acquire write lock on interface registry: {}", e))
        })?;
        
        // Generate a new unique ID
        let next_id = registry_write.len() as u64 + 10; // Start after reserved IDs
        
        // Check if we're about to exceed the maximum registry size
        if registry_write.len() >= MAX_TYPE_REGISTRY_SIZE {
            return Err(Error::Compilation(format!(
                "Type registry size exceeded maximum of {} types",
                MAX_TYPE_REGISTRY_SIZE
            )));
        }
        
        // Insert the new type
        registry_write.insert(type_name.to_string(), next_id);
        debug!("Registered new type {} with ID {}", type_name, next_id);
        
        Ok(next_id)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn lookup_type_name(&self, type_id: TypeRegistryId) -> Option<String> {
        if let Some(registry) = &self.interface_type_registry {
            let registry = match registry.read() {
                Ok(r) => r,
                Err(_) => return None,
            };
            
            // Find the type name by ID (linear search through the HashMap)
            for (name, id) in registry.iter() {
                if *id == type_id {
                    return Some(name.clone());
                }
            }
        }
        
        None
    }
    
    #[instrument(skip(self), level = "debug")]
    fn get_type_id(&self, type_name: &str) -> Result<TypeRegistryId, Error> {
        if let Some(registry) = &self.interface_type_registry {
            let registry = registry.read().map_err(|e| {
                Error::Compilation(format!("Failed to acquire read lock on interface registry: {}", e))
            })?;
            
            if let Some(id) = registry.get(type_name) {
                return Ok(*id);
            }
        }
        
        // Type not found
        Err(Error::Compilation(format!("Type {} not found in registry", type_name)))
    }
    
    #[instrument(skip(self), level = "debug")]
    fn initialize_global_type_registry(&mut self) -> Result<(), Error> {
        debug!("Initializing global type registry");
        
        let context = self.context();
        let module = self.module();
        
        // Create global arrays if they don't exist yet
        if self.global_type_names.is_none() {
            // Create type for string array - array of i8*
            let char_type = context.i8_type();
            let char_ptr_type = char_type.ptr_type(AddressSpace::default());
            let string_array_type = char_ptr_type.array_type(MAX_TYPE_REGISTRY_SIZE as u32);
            
            // Create global arrays for type names
            // The array will be initialized incrementally as types are registered
            let type_names_global = module.add_global(
                string_array_type,
                None,
                "__cursed_type_registry_names"
            );
            
            // Initialize the type names array with null pointers
            // For each position in the array, we'll store a null pointer that will be replaced
            // when the corresponding type is registered
            let null_ptr = char_ptr_type.const_null();
        
        // Create an array of null pointers as the initial value
        let mut null_ptrs = Vec::with_capacity(MAX_TYPE_REGISTRY_SIZE);
        for _ in 0..MAX_TYPE_REGISTRY_SIZE {
            null_ptrs.push(null_ptr);
        }
        
        // Create a constant array of null pointers
        // Using the array_type method to create an array of null pointers
        let null_array_type = char_ptr_type.array_type(MAX_TYPE_REGISTRY_SIZE as u32);
        
        // Create an empty struct initializer
        let null_array = null_array_type.const_array(&vec![null_ptr; MAX_TYPE_REGISTRY_SIZE]);
        
        // Set the initializer for the type names array
        type_names_global.set_initializer(&null_array);
            
            // Create global variable for the number of registered types
            let count_global = module.add_global(
                context.i32_type(),
                None,
                "__cursed_type_registry_count"
            );
            count_global.set_initializer(&context.i32_type().const_int(0, false));
            
            // Store these globals for later use
            self.global_type_names = Some(type_names_global);
            self.global_type_count = Some(count_global);
            
            debug!("Created global type registry arrays");
        }
        
        // Register the base types in the global registry
        if let Some(registry) = &self.interface_type_registry {
            let registry = registry.read().map_err(|e| {
                Error::Compilation(format!("Failed to acquire read lock on interface registry: {}", e))
            })?;
            
            // Add all registered types to the global arrays
            for (name, id) in registry.iter() {
                // Skip special/dummy types like <null>
                if name == "<null>" {
                    continue;
                }
                
                self.add_type_to_global_registry(*id, name)?;
            }
        }
        
        Ok(())
    }
    
    #[instrument(skip(self), level = "debug")]
    fn add_type_to_global_registry(&mut self, type_id: TypeRegistryId, type_name: &str) -> Result<(), Error> {
        // Get the global arrays
        let type_names_global = self.global_type_names.ok_or_else(|| {
            Error::Compilation("Global type registry not initialized".to_string())
        })?;
        
        let count_global = self.global_type_count.ok_or_else(|| {
            Error::Compilation("Global type registry count not initialized".to_string())
        })?;
        
        // Create a global string for the type name using our enhanced helper method
        let type_name_str = self.create_global_string_enhanced(
            type_name,
            &format!("type_name_{}", type_id)
        )?;
        
        // Get pointer to the array element at index type_id
        let type_id_ptr = unsafe {
            self.builder().build_gep(
                self.context().i8_type().ptr_type(AddressSpace::default()),
                type_names_global.as_pointer_value(),
                &[
                    self.context().i32_type().const_zero(),
                    self.context().i32_type().const_int(type_id, false)
                ],
                &format!("type_name_ptr_{}", type_id)
            ).map_err(|e| {
                Error::Compilation(format!("Failed to build GEP for type name array: {}", e))
            })?
        };
        
        // Store the type name pointer at the array element
        self.builder().build_store(
            type_id_ptr,
            type_name_str
        ).map_err(|e| {
            Error::Compilation(format!("Failed to store type name in registry array: {}", e))
        })?;
        
        // Increment the type count
        let current_count = self.builder().build_load(
            self.context().i32_type(),
            count_global.as_pointer_value(),
            "current_type_count"
        ).map_err(|e| {
            Error::Compilation(format!("Failed to load current type count: {}", e))
        })?;
        
        let incremented_count = self.builder().build_int_add(
            current_count.into_int_value(),
            self.context().i32_type().const_int(1, false),
            "incremented_type_count"
        ).map_err(|e| {
            Error::Compilation(format!("Failed to increment type count: {}", e))
        })?;
        
        self.builder().build_store(
            count_global.as_pointer_value(),
            incremented_count
        ).map_err(|e| {
            Error::Compilation(format!("Failed to store incremented type count: {}", e))
        })?;
        
        debug!("Added type {} (ID: {}) to global registry", type_name, type_id);
        Ok(())
    }
    
    #[instrument(skip(self), level = "debug")]
    fn get_global_type_names_array(&self) -> Result<PointerValue<'ctx>, Error> {
        let type_names_global = self.global_type_names.ok_or_else(|| {
            Error::Compilation("Global type registry not initialized".to_string())
        })?;
        
        Ok(type_names_global.as_pointer_value())
    }
    
    #[instrument(skip(self, type_id), level = "debug")]
    fn get_runtime_type_name(&mut self, type_id: BasicValueEnum<'ctx>) -> Result<PointerValue<'ctx>, Error> {
        debug!("Getting runtime type name for type ID: {:?}", type_id);
        
        // Ensure the registry is initialized
        if self.global_type_names.is_none() {
            debug!("Global type registry not initialized, initializing now");
            self.initialize_global_type_registry()?;
        }
        
        let type_names_array = self.get_global_type_names_array()?;
        debug!("Got global type names array: {:?}", type_names_array);
        
        // Get pointer to the type name string in the array
        let type_id_int = if type_id.is_int_value() {
            type_id.into_int_value()
        } else {
            error!("Expected type_id to be an integer value, got {:?}", type_id);
            return Err(Error::Compilation(format!(
                "Expected type_id to be an integer value, got {:?}",
                type_id
            )));
        };
        
        // Bounds check the type ID
        let max_id = self.context().i64_type().const_int(MAX_TYPE_REGISTRY_SIZE as u64, false);
        let is_valid_id = self.builder().build_int_compare(
            IntPredicate::ULT,
            type_id_int,
            max_id,
            "is_valid_type_id"
        ).map_err(|e| {
            Error::Compilation(format!("Failed to compare type ID with maximum: {}", e))
        })?;
        
        // Create basic blocks for valid and invalid ID paths
        let current_function = self.current_function().ok_or_else(|| {
            Error::Compilation("No current function when getting runtime type name".to_string())
        })?;
        
        let valid_id_block = self.context().append_basic_block(current_function, "valid_type_id");
        let invalid_id_block = self.context().append_basic_block(current_function, "invalid_type_id");
        let continue_block = self.context().append_basic_block(current_function, "continue_type_name");
        
        // Branch based on ID validity
        self.builder().build_conditional_branch(
            is_valid_id,
            valid_id_block,
            invalid_id_block
        ).map_err(|e| {
            Error::Compilation(format!("Failed to build branch for type ID validation: {}", e))
        })?;
        
        // Valid ID path - get the type name from the registry
        self.builder().position_at_end(valid_id_block);
        
        // Access the array element at index type_id
        let type_name_ptr = unsafe {
            self.builder().build_gep(
                self.context().i8_type().ptr_type(AddressSpace::default()),
                type_names_array,
                &[
                    self.context().i32_type().const_int(0, false),
                    self.builder().build_int_truncate(
                        type_id_int,
                        self.context().i32_type(),
                        "type_id_i32"
                    ).map_err(|e| {
                        Error::Compilation(format!("Failed to truncate type ID to i32: {}", e))
                    })?
                ],
                "type_name_ptr"
            ).map_err(|e| {
                Error::Compilation(format!("Failed to build GEP for type name array: {}", e))
            })?
        };
        
        // Load the type name pointer
        let type_name_loaded = self.builder().build_load(
            self.context().i8_type().ptr_type(AddressSpace::default()),
            type_name_ptr,
            "loaded_type_name"
        ).map_err(|e| {
            Error::Compilation(format!("Failed to load type name pointer: {}", e))
        })?;
        
        // Check if the loaded pointer is null
        let is_null = self.builder().build_is_null(
            type_name_loaded.into_pointer_value(),
            "is_null_type_name"
        ).map_err(|e| {
            Error::Compilation(format!("Failed to check if type name pointer is null: {}", e))
        })?;
        
        // Create blocks for null and non-null paths
        let null_block = self.context().append_basic_block(current_function, "null_type_name");
        let non_null_block = self.context().append_basic_block(current_function, "non_null_type_name");
        let merge_block = self.context().append_basic_block(current_function, "merge_type_name");
        
        // Branch based on null check
        self.builder().build_conditional_branch(
            is_null,
            null_block,
            non_null_block
        ).map_err(|e| {
            Error::Compilation(format!("Failed to build branch for null type name check: {}", e))
        })?;
        
        // Null path - use "<unknown>" string
        self.builder().position_at_end(null_block);
        let unknown_str = self.create_global_string_enhanced("<unknown>", "unknown_type_name")?;
        self.builder().build_unconditional_branch(merge_block).map_err(|e| {
            Error::Compilation(format!("Failed to build branch to merge block: {}", e))
        })?;
        
        // Non-null path - use the loaded type name
        self.builder().position_at_end(non_null_block);
        self.builder().build_unconditional_branch(merge_block).map_err(|e| {
            Error::Compilation(format!("Failed to build branch to merge block: {}", e))
        })?;
        
        // Merge path - select the appropriate string
        self.builder().position_at_end(merge_block);
        let valid_phi = self.builder().build_phi(
            self.context().i8_type().ptr_type(AddressSpace::default()),
            "selected_type_name"
        ).map_err(|e| {
            Error::Compilation(format!("Failed to build phi node for type name: {}", e))
        })?;
        
        valid_phi.add_incoming(&[(
            &unknown_str,
            null_block
        ), (
            &type_name_loaded.into_pointer_value(),
            non_null_block
        )]);
        
        self.builder().build_unconditional_branch(continue_block).map_err(|e| {
            Error::Compilation(format!("Failed to build branch to continue block: {}", e))
        })?;
        
        // Invalid ID path - use "<invalid>" string
        self.builder().position_at_end(invalid_id_block);
        let invalid_str = self.create_global_string_enhanced("<invalid>", "invalid_type_name")?;
        self.builder().build_unconditional_branch(continue_block).map_err(|e| {
            Error::Compilation(format!("Failed to build branch to continue block: {}", e))
        })?;
        
        // Continue path - select between valid and invalid paths
        self.builder().position_at_end(continue_block);
        let final_phi = self.builder().build_phi(
            self.context().i8_type().ptr_type(AddressSpace::default()),
            "final_type_name"
        ).map_err(|e| {
            Error::Compilation(format!("Failed to build final phi node for type name: {}", e))
        })?;
        
        final_phi.add_incoming(&[(
            &valid_phi.as_basic_value().into_pointer_value(),
            merge_block
        ), (
            &invalid_str,
            invalid_id_block
        )]);
        
        Ok(final_phi.as_basic_value().into_pointer_value())
    }
    
    #[instrument(skip(self, actual_type_id), level = "debug")]
    fn create_type_assertion_error(
        &mut self,
        actual_type_id: BasicValueEnum<'ctx>,
        expected_type_name: &str
    ) -> Result<PointerValue<'ctx>, Error> {
        debug!("Creating type assertion error message for expected type: {}", expected_type_name);
        
        // Get the actual type name from the registry
        let actual_type_name = self.get_runtime_type_name(actual_type_id)?;
        debug!("Got actual type name pointer: {:?}", actual_type_name);
        
        // Create a global string for the expected type with a unique name to avoid conflicts
        let unique_id = self.generate_unique_id();
        let expected_type_str = self.create_global_string_enhanced(
            expected_type_name, 
            &format!("expected_type_{}", unique_id)
        )?;
        debug!("Created expected type string: {:?}", expected_type_str);
        
        // Format: "Type assertion failed: cannot convert <actual_type> to <expected_type>"
        let prefix = "Type assertion failed: cannot convert ";
        let middle = " to ";
        
        // Allocate a buffer for the error message with ample size for both type names plus the message
        let buffer_size = self.context().i32_type().const_int(512, false); // Larger buffer to accommodate most type names
        let buffer = self.builder().build_array_alloca(
            self.context().i8_type(),
            buffer_size,
            "error_buffer"
        ).map_err(|e| {
            Error::Compilation(format!("Failed to allocate error message buffer: {}", e))
        })?;
        debug!("Allocated error buffer: {:?}", buffer);
        
        // Get runtime function for string formatting
        let sprintf_fn = match self.get_external_function("snprintf", self.context().i32_type(), &[
            self.context().i8_type().ptr_type(AddressSpace::default()).into(),
            self.context().i32_type().into(),
            self.context().i8_type().ptr_type(AddressSpace::default()).into()
        ], true) {
            Ok(f) => f,
            Err(_) => {
                // Fallback if snprintf is not available - create a static error message
                let static_err = self.create_global_string_enhanced(
                    &format!("{}{}{}{}", prefix, "<type>", middle, expected_type_name),
                    "static_error"
                )?;
                return Ok(static_err);
            }
        };
        
        // Create format string
        let format_str = self.create_global_string_enhanced("%s%s%s%s", "error_format")?;
        
        // Prefix string constant
        let prefix_str = self.create_global_string_enhanced(prefix, "error_prefix")?;
        
        // Middle string constant
        let middle_str = self.create_global_string_enhanced(middle, "error_middle")?;
        
        // Call sprintf to format the error message
        let _res = self.builder().build_call(
            sprintf_fn,
            &[
                buffer.into(),
                buffer_size.into(),
                format_str.into(),
                prefix_str.into(),
                actual_type_name.into(),
                middle_str.into(),
                expected_type_str.into()
            ],
            "sprintf_result"
        ).map_err(|e| {
            Error::Compilation(format!("Failed to call sprintf: {}", e))
        })?;
        
        // Return the buffer pointer
        Ok(buffer)
    }
}

impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Generate a unique ID for global variable names
    #[instrument(skip(self), level = "debug")]
    pub fn generate_unique_id(&self) -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
        let nanos = duration.as_nanos() as u64;
        let counter = self.unique_id_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        nanos ^ counter
    }

    /// Helper method to create a global string constant (enhanced version)
    #[instrument(skip(self), level = "debug")]
    pub fn create_global_string_enhanced(&self, string: &str, name: &str) -> Result<PointerValue<'ctx>, Error> {
        let string_const = self.context().const_string(string.as_bytes(), true);
        let global_string = self.module().add_global(
            string_const.get_type(),
            None,
            name
        );
        
        global_string.set_initializer(&string_const);
        global_string.set_constant(true);
        global_string.set_linkage(inkwell::module::Linkage::Private);
        global_string.set_unnamed_addr(true);
        
        Ok(global_string.as_pointer_value())
    }
}

/// Helper trait for runtime type information access
pub trait RuntimeTypeInfo<'ctx> {
    /// Get the registry entry for a type assertion in a way that can be used in error messages
    fn get_assertion_type_info(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type_name: &str
    ) -> Result<(BasicValueEnum<'ctx>, String), Error>;
    
    /// Log detailed information about a type assertion result
    fn log_type_assertion_with_info(
        &mut self,
        actual_type_id: BasicValueEnum<'ctx>,
        target_type_name: &str,
        success: bool
    ) -> Result<(), Error>;
    
    /// Create a user-friendly error message for a type assertion failure
    fn format_type_assertion_error(
        &mut self,
        actual_type_id: BasicValueEnum<'ctx>,
        expected_type_name: &str
    ) -> Result<String, Error>;
}

impl<'ctx> RuntimeTypeInfo<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self, interface_value), level = "debug")]
    fn get_assertion_type_info(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type_name: &str
    ) -> Result<(BasicValueEnum<'ctx>, String), Error> {
        // Get the type ID from the interface value
        // Use safe version of get_interface_type_id
        let actual_type_id = match self.get_interface_type_id_safe(interface_value) {
            Ok(id) => id,
            Err(e) => {
                warn!("Failed to get interface type ID: {}", e);
                // Return a default/fallback type ID
                (self.context().i64_type().const_int(u64::MAX, false), "<unknown>".to_string())
            }
        };
        
        // Look up the type name if the ID is a constant
        let type_name = if let Some(id_const) = actual_type_id.into_int_value().get_zero_extended_constant() {
            // Try to get the name from the registry
            match self.lookup_type_name(id_const) {
                Some(name) => name,
                None => format!("<unknown:{:x}>", id_const)
            }
        } else {
            "<dynamic>".to_string()
        };
        
        Ok((actual_type_id, type_name))
    }
    
    #[instrument(skip(self, actual_type_id), level = "debug")]
    fn log_type_assertion_with_info(
        &mut self,
        actual_type_id: BasicValueEnum<'ctx>,
        target_type_name: &str,
        success: bool
    ) -> Result<(), Error> {
        // Determine if debug logging is enabled
        let debug_level = std::env::var("CURSED_TYPE_DEBUG")
            .or_else(|_| std::env::var("CURSED_DEBUG"))
            .map(|val| {
                if val.is_empty() || val == "0" || val.to_lowercase() == "false" {
                    0 // Disabled
                } else {
                    val.parse::<u32>().unwrap_or(1) // Parse level or default to 1
                }
            })
            .unwrap_or(0); // Default to disabled
            
        if debug_level == 0 {
            return Ok(());
        }
        
        // Get the actual type name from the ID
        let type_name = if let Some(id_const) = actual_type_id.into_int_value().get_zero_extended_constant() {
            match self.lookup_type_name(id_const) {
                Some(name) => name,
                None => format!("<unknown:{:x}>", id_const)
            }
        } else {
            "<dynamic>".to_string()
        };
        
        // Log the assertion result at the appropriate level
        match (success, debug_level) {
            (true, 1) => {
                debug!("Type assertion SUCCESS: {} to {}", type_name, target_type_name);
            },
            (true, _) => {
                // More verbose success logging for higher debug levels
                info!(
                    "Type assertion SUCCESS: value of type '{}' (ID: {:?}) converted to '{}'.",
                    type_name,
                    actual_type_id,
                    target_type_name
                );
            },
            (false, _) => {
                // Always log failures regardless of verbosity level (if debugging is enabled)
                warn!(
                    "Type assertion FAILED: value of type '{}' (ID: {:?}) cannot be converted to '{}'.",
                    type_name,
                    actual_type_id,
                    target_type_name
                );
            }
        }
        
        Ok(())
    }
    
    #[instrument(skip(self, actual_type_id), level = "debug")]
    fn format_type_assertion_error(
        &mut self,
        actual_type_id: BasicValueEnum<'ctx>,
        expected_type_name: &str
    ) -> Result<String, Error> {
        // Get the actual type name from the ID
        let type_name = if let Some(id_const) = actual_type_id.into_int_value().get_zero_extended_constant() {
            match self.lookup_type_name(id_const) {
                Some(name) => name,
                None => format!("<unknown:{:x}>", id_const)
            }
        } else {
            "<dynamic>".to_string()
        };
        
        // Format a user-friendly error message
        let message = format!(
            "Type assertion failed: cannot convert {} to {}",
            type_name,
            expected_type_name
        );
        
        Ok(message)
    }
}