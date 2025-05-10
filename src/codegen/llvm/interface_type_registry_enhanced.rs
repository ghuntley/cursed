//! # Enhanced Interface Type Registry for Runtime Type Information
//! 
//! This module enhances the interface type registry to fully support runtime type information
//! during type assertions, significantly improving error reporting and debugging capabilities.
//! It fixes array initialization and ensures proper integration with the type assertion system.

use std::collections::HashMap;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicTypeEnum, StructType, ArrayType};
use inkwell::values::{BasicValueEnum, PointerValue, IntValue, GlobalValue, ArrayValue};
use inkwell::AddressSpace;

use tracing::{debug, error, info, instrument, span, trace, warn, Level};

use crate::error::Error;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::interface_type_registry::{InterfaceTypeRegistry, InterfaceTypeRegistryAccess};
use crate::codegen::llvm::interface_type_assertion_errors::TypeAssertionErrorHandler;
use crate::codegen::llvm::type_assertion::InterfaceTypeAssertion;

/// Trait for enhanced interface type registry functionality
pub trait EnhancedTypeRegistry<'ctx> {
    /// Initialize the type registry globals using correct LLVM operations
    fn initialize_type_registry_globals(&mut self) -> Result<(), Error>;
    
    /// Generate lookup code for finding a type name by ID with proper error handling
    fn lookup_type_name_enhanced(
        &mut self,
        type_id: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Get human-readable type information for an interface value
    fn get_interface_type_info(
        &mut self,
        interface_value: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Register a type with the enhanced registry including runtime information
    fn register_type_with_runtime_info(&mut self, type_id: u64, type_name: &str) -> Result<(), Error>;
    
    /// Log detailed information about a type assertion with human-readable type names
    fn log_type_assertion_with_info(
        &self,
        actual_type_id: BasicValueEnum<'ctx>,
        expected_type_name: &str,
        success: bool
    ) -> Result<(), Error>;
    
    /// Get both type ID and human-readable name for an interface value
    fn get_assertion_type_info(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        expected_type_name: &str
    ) -> Result<(BasicValueEnum<'ctx>, String), Error>;
}

impl<'ctx> EnhancedTypeRegistry<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn initialize_type_registry_globals(&mut self) -> Result<(), Error> {
        debug!("Initializing enhanced type registry globals with proper GEP operations");
        
        // Get all registered types
        let registry = self.interface_type_registry_mut();
        let types = registry.all_types();
        
        if types.is_empty() {
            debug!("No types registered, skipping global initialization");
            return Ok(());
        }
        
        let type_count = types.len();
        debug!("Initializing type registry with {} types", type_count);
        
        // Create an entry block to initialize our globals
        let current_fn = self.current_function()
            .ok_or_else(|| Error::codegen("No current function for type registry initialization"))?;
        
        let init_block = self.context().append_basic_block(current_fn, "type_registry_init");
        let old_block = self.builder().get_insert_block().ok_or_else(|| {
            Error::codegen("No current block for type registry initialization")
        })?;
        
        // Position at the new block for initialization
        self.builder().position_at_end(init_block);
        
        // --- Initialize the type ID array ---
        // 1. Create the array type for our type IDs
        let id_type = self.context().i64_type();
        let id_array_type = id_type.array_type(type_count as u32);
        
        // 2. Create a unique name for the global using a timestamp-based approach
        // This ensures we don't have naming conflicts when generating multiple globals
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        
        let id_global_name = format!("cursed_type_ids_{}", timestamp);
        let id_global = self.module().add_global(id_array_type, None, &id_global_name);
        
        // 3. Set to internal linkage so it's not visible outside the module
        id_global.set_linkage(inkwell::module::Linkage::Internal);
        
        // 4. Initialize with zeroes first to create the memory space
        id_global.set_initializer(&id_array_type.const_zero());
        
        // 5. Get a pointer to the global array for storing values
        let id_array_ptr = id_global.as_pointer_value();
        
        // --- Initialize the type name array ---
        // 1. Create the array type for our type name strings
        let i8_ptr_type = self.context().i8_type().ptr_type(AddressSpace::default());
        let str_array_type = i8_ptr_type.array_type(type_count as u32);
        
        // 2. Create a unique name for the string array global
        let str_global_name = format!("cursed_type_names_{}", timestamp);
        let str_global = self.module().add_global(str_array_type, None, &str_global_name);
        
        // 3. Set to internal linkage
        str_global.set_linkage(inkwell::module::Linkage::Internal);
        
        // 4. Initialize with zeroes first
        str_global.set_initializer(&str_array_type.const_zero());
        
        // 5. Get a pointer to the global array
        let str_array_ptr = str_global.as_pointer_value();
        
        // Generate string constants for each type name and store them in the arrays
        for (i, (id, name)) in types.iter().enumerate() {
            // 1. Store the type ID in the ID array using proper GEP operations
            let id_val = id_type.const_int(*id, false);
            
            // Use build_in_bounds_gep with the correct array type
            // First index 0 is for the pointer itself, second index i is for the array element
            let id_ptr = unsafe {
                self.builder().build_in_bounds_gep(
                    id_array_type,
                    id_array_ptr,
                    &[
                        self.context().i32_type().const_zero(),
                        self.context().i32_type().const_int(i as u64, false)
                    ],
                    &format!("type_id_{}_ptr", i)
                ).map_err(|e| Error::codegen(format!("Failed to get GEP for type ID: {}", e)))?
            };
            
            // Store the ID value in the array at index i
            self.builder().build_store(id_ptr, id_val)
                .map_err(|e| Error::codegen(format!("Failed to store type ID: {}", e)))?;
            
            // 2. Create a global string constant for each type name
            let name_with_null = format!("{}", name) + "\0";
            let name_bytes = name_with_null.as_bytes();
            let str_type = self.context().i8_type().array_type(name_bytes.len() as u32);
            
            // Create a unique name for each string global to avoid conflicts
            let str_global_name = format!("type_name_{}_{}", i, timestamp);
            let str_global = self.module().add_global(
                str_type, 
                None, 
                &str_global_name
            );
            str_global.set_linkage(inkwell::module::Linkage::Private);
            str_global.set_constant(true);
            
            // Initialize with the string content
            let str_val = self.context().const_string(name_bytes, false);
            str_global.set_initializer(&str_val);
            
            // 3. Cast the string global to i8* for storage in our array
            let str_ptr = self.builder().build_pointer_cast(
                str_global.as_pointer_value(),
                i8_ptr_type,
                &format!("type_name_{}_ptr", i)
            ).map_err(|e| Error::codegen(format!("Failed to cast type name pointer: {}", e)))?;
            
            // 4. Store the string pointer in the name array using proper GEP operations
            // Similar to the ID array, use GEP with the array type and indices
            let name_ptr = unsafe {
                self.builder().build_in_bounds_gep(
                    str_array_type,
                    str_array_ptr,
                    &[
                        self.context().i32_type().const_zero(),
                        self.context().i32_type().const_int(i as u64, false)
                    ],
                    &format!("str_ptr_{}_ptr", i)
                ).map_err(|e| Error::codegen(format!("Failed to get pointer to string ptr: {}", e)))?
            };
            
            // Store the string pointer in the array at index i
            self.builder().build_store(name_ptr, str_ptr)
                .map_err(|e| Error::codegen(format!("Failed to store string pointer: {}", e)))?;
            
            debug!("Initialized type registry entry {} with ID {} and name '{}'", i, id, name);
        }
        
        // Return to the original block
        self.builder().position_at_end(old_block);
        
        // Insert a branch to the initialization block and back
        self.builder().build_unconditional_branch(init_block)
            .map_err(|e| Error::codegen(format!("Failed to branch to initialization block: {}", e)))?;
        
        // Position at the end of the init block to add a return branch
        self.builder().position_at_end(init_block);
        self.builder().build_unconditional_branch(old_block)
            .map_err(|e| Error::codegen(format!("Failed to branch back from initialization block: {}", e)))?;
        
        // Continue at the original position
        self.builder().position_at_end(old_block);
        
        // Store the globals in the registry
        let registry = self.interface_type_registry_mut();
        registry.type_ids_global = Some(id_global);
        registry.type_names_global = Some(str_global);
        
        debug!("Successfully initialized type registry globals with {} types", type_count);
        Ok(())
    }
    
    #[instrument(skip(self, type_id), level = "debug")]
    fn lookup_type_name_enhanced(
        &mut self,
        type_id: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Enhanced type name lookup for ID: {:?}", type_id);
        
        // Ensure we've initialized the type registry globals
        let registry = self.interface_type_registry();
        if registry.type_ids_global.is_none() || registry.type_names_global.is_none() {
            self.initialize_type_registry_globals()?;
        }
        
        // Get the type count
        let type_count = registry.type_count();
        if type_count == 0 {
            debug!("No types in registry, returning unknown type");
            return self.create_unknown_type_string();
        }
        
        // Try to get the ID as a constant if available
        let const_id = if type_id.is_int_value() {
            type_id.into_int_value().get_zero_extended_constant()
        } else {
            None
        };
        
        // If we have a constant ID, we can do a compile-time lookup
        if let Some(id_val) = const_id {
            if let Some(type_name) = registry.get_type_name(id_val) {
                debug!("Compile-time lookup for ID {} found: {}", id_val, type_name);
                
                // Create a global string constant for the type name
                let name_with_null = format!("{}", type_name) + "\0";
                let name_bytes = name_with_null.as_bytes();
                
                let str_val = self.context().const_string(name_bytes, false);
                let str_type = self.context().i8_type().array_type(name_bytes.len() as u32);
                
                let str_global = self.module().add_global(
                    str_type,
                    None,
                    &format!("type_name_const_{}", id_val)
                );
                str_global.set_linkage(inkwell::module::Linkage::Private);
                str_global.set_constant(true);
                str_global.set_initializer(&str_val);
                
                let str_ptr = self.builder().build_pointer_cast(
                    str_global.as_pointer_value(),
                    self.context().i8_type().ptr_type(AddressSpace::default()),
                    &format!("type_name_ptr_{}", id_val)
                ).map_err(|e| Error::codegen(format!("Failed to cast type name pointer: {}", e)))?;
                
                return Ok(str_ptr.into());
            }
        }
        
        // Fall back to runtime lookup
        debug!("Using runtime lookup for type ID");
        
        // Get the globals from the registry
        let (ids_global, names_global) = match (registry.type_ids_global, registry.type_names_global) {
            (Some(ids), Some(names)) => (ids, names),
            _ => {
                debug!("Missing type registry globals, returning unknown type");
                return self.create_unknown_type_string();
            }
        };
        
        // Get the current function for creating basic blocks
        let current_fn = self.current_function()
            .ok_or_else(|| Error::codegen("No current function for type name lookup"))?;
        
        // Create basic blocks for the lookup
        let entry_block = self.builder().get_insert_block()
            .ok_or_else(|| Error::codegen("No current block for type lookup"))?;
        let lookup_block = self.context().append_basic_block(current_fn, "type_lookup");
        let not_found_block = self.context().append_basic_block(current_fn, "type_not_found");
        let continue_block = self.context().append_basic_block(current_fn, "type_lookup_continue");
        
        // Branch to the lookup block
        self.builder().build_unconditional_branch(lookup_block)
            .map_err(|e| Error::codegen(format!("Failed to branch to lookup block: {}", e)))?;
        
        // Position at the lookup block
        self.builder().position_at_end(lookup_block);
        
        // Get pointers to the global arrays
        let ids_ptr = ids_global.as_pointer_value();
        let names_ptr = names_global.as_pointer_value();
        
        // Create a helper function to look up the type
        let i8_ptr_type = self.context().i8_type().ptr_type(AddressSpace::default());
        let lookup_result = self.generate_array_lookup(
            ids_ptr,
            names_ptr,
            type_id,
            type_count as u32,
            "type_lookup_result"
        )?;
        
        // Branch to the continue block
        self.builder().build_unconditional_branch(continue_block)
            .map_err(|e| Error::codegen(format!("Failed to branch to continue block: {}", e)))?;
        
        // Not found block - return "Unknown Type"
        self.builder().position_at_end(not_found_block);
        let unknown_ptr = self.create_unknown_type_string()?;
        
        // Branch to continue block
        self.builder().build_unconditional_branch(continue_block)
            .map_err(|e| Error::codegen(format!("Failed to branch from not found: {}", e)))?;
        
        // Continue block - use phi node to select the appropriate string
        self.builder().position_at_end(continue_block);
        
        let phi = self.builder().build_phi(
            i8_ptr_type,
            "final_type_name"
        ).map_err(|e| Error::codegen(format!("Failed to build phi node: {}", e)))?;
        
        phi.add_incoming(&[(
            &lookup_result.into_pointer_value(),
            lookup_block
        ), (
            &unknown_ptr.into_pointer_value(),
            not_found_block
        )]);
        
        // Return to the original block
        self.builder().position_at_end(entry_block);
        
        debug!("Enhanced type name lookup completed");
        Ok(phi.as_basic_value())
    }
    
    #[instrument(skip(self, interface_value), level = "debug")]
    fn get_interface_type_info(
        &mut self,
        interface_value: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Getting interface type info");
        
        // Extract the type ID from the interface value
        let type_id = self.get_interface_type_id_safe(interface_value)?;
        
        // Look up the type name using the enhanced lookup
        let type_name_ptr = self.lookup_type_name_enhanced(type_id)?;
        
        debug!("Retrieved interface type info successfully");
        Ok(type_name_ptr)
    }
    
    #[instrument(skip(self), level = "debug")]
    fn register_type_with_runtime_info(&mut self, type_id: u64, type_name: &str) -> Result<(), Error> {
        debug!("Registering type with runtime info: {} -> {}", type_id, type_name);
        
        // Register the type in the registry
        self.interface_type_registry_mut().register_type(type_id, type_name.to_string());
        
        // Mark the registry globals as invalid so they'll be regenerated
        let registry = self.interface_type_registry_mut();
        registry.type_ids_global = None;
        registry.type_names_global = None;
        
        // Initialize the globals if we have a valid current function
        if self.current_function().is_some() {
            self.initialize_type_registry_globals()?;
        }
        
        debug!("Successfully registered type with runtime info");
        Ok(())
    }
    
    #[instrument(skip(self, actual_type_id), level = "debug")]
    fn log_type_assertion_with_info(
        &self,
        actual_type_id: BasicValueEnum<'ctx>,
        expected_type_name: &str,
        success: bool
    ) -> Result<(), Error> {
        debug!("Logging type assertion with enhanced info");
        
        // Get the debug level from environment variables
        let debug_level = std::env::var("CURSED_TYPE_DEBUG")
            .or_else(|_| std::env::var("CURSED_DEBUG"))
            .map(|val| {
                if val.is_empty() || val == "0" || val.to_lowercase() == "false" {
                    "none"
                } else {
                    val.as_str()
                }
            })
            .unwrap_or("none");
        
        // If debugging is disabled, just return
        if debug_level == "none" {
            return Ok(());
        }
        
        // Try to extract a constant type ID if available
        let type_id_val = if actual_type_id.is_int_value() {
            actual_type_id.into_int_value().get_zero_extended_constant()
                .unwrap_or(u64::MAX) // Use MAX as a sentinel for unknown
        } else {
            u64::MAX // Unknown type ID
        };
        
        // Get the type name from the registry
        let actual_type_name = match &self.interface_type_registry {
            Some(registry) => {
                registry.get_type_name(type_id_val)
                    .map(|name| name.clone())
                    .unwrap_or_else(|| String::from("Unknown Type"))
            },
            None => String::from("Unknown Type")
        };
        
        // Log the type assertion details with appropriate level based on success
        if success {
            if debug_level == "verbose" || debug_level == "standard" {
                info!(
                    "Type assertion SUCCESS: Value of type '{}' (ID: {}) asserted to type '{}'", 
                    actual_type_name, 
                    type_id_val,
                    expected_type_name
                );
            }
        } else {
            // Always log failures regardless of debug level
            warn!(
                "Type assertion FAILED: Cannot convert from '{}' (ID: {}) to '{}'. Types are incompatible.", 
                actual_type_name, 
                type_id_val,
                expected_type_name
            );
            
            if debug_level == "verbose" {
                // Additional debugging information for verbose mode
                let registry = self.interface_type_registry.as_ref().unwrap();
                let all_types = registry.all_types();
                let types_info = all_types.iter()
                    .map(|(id, name)| format!("{} -> {}", id, name))
                    .collect::<Vec<_>>()
                    .join(", ");
                
                debug!("Available types in registry: [{}]", types_info);
            }
        }
        
        debug!("Successfully logged type assertion information");
        Ok(())
    }
    
    #[instrument(skip(self, interface_value), level = "debug")]
    fn get_assertion_type_info(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        expected_type_name: &str
    ) -> Result<(BasicValueEnum<'ctx>, String), Error> {
        debug!("Getting type assertion info for expected type: {}", expected_type_name);
        
        // Get the actual type ID from the interface value
        let type_id = self.get_interface_type_id_safe(interface_value)?;
        
        // Try to extract a constant type ID if available
        let type_id_val = if type_id.is_int_value() {
            type_id.into_int_value().get_zero_extended_constant()
                .unwrap_or(u64::MAX) // Use MAX as a sentinel for unknown
        } else {
            u64::MAX // Unknown type ID
        };
        
        // Get the type name from the registry
        let type_name = match &self.interface_type_registry {
            Some(registry) => {
                registry.get_type_name(type_id_val)
                    .map(|name| name.clone())
                    .unwrap_or_else(|| String::from("Unknown Type"))
            },
            None => String::from("Unknown Type")
        };
        
        debug!("Retrieved type info: ID: {}, name: {}", type_id_val, type_name);
        Ok((type_id, type_name))
    }
}

// Helper methods for the enhanced type registry
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Get the type ID from an interface value safely using our enhanced registry
    #[instrument(skip(self, interface_value), level = "debug")]
    fn get_interface_type_id_safe(
        &mut self,
        interface_value: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Getting type ID safely from interface value");
        
        // Delegate to the error handler implementation for robust error handling
        let type_id = crate::codegen::llvm::interface_type_assertion_errors::TypeAssertionErrorHandler::get_interface_type_id_safe(
            self, interface_value
        )?;
        
        debug!("Retrieved type ID safely");
        Ok(type_id)
    }
    /// Create an "Unknown Type" string global
    fn create_unknown_type_string(&mut self) -> Result<BasicValueEnum<'ctx>, Error> {
        let unknown_type_str = self.context().const_string("Unknown Type".as_bytes(), true);
        let unknown_global = self.module().add_global(
            unknown_type_str.get_type(), 
            None, 
            "str_unknown_type"
        );
        unknown_global.set_linkage(inkwell::module::Linkage::Private);
        unknown_global.set_initializer(&unknown_type_str);
        
        Ok(unknown_global.as_pointer_value().into())
    }
    
    /// Generate code to look up a value in an array with enhanced GetElementPtr operations
    fn generate_array_lookup(
        &mut self,
        keys_array: PointerValue<'ctx>,
        values_array: PointerValue<'ctx>,
        target_key: BasicValueEnum<'ctx>,
        array_length: u32,
        result_name: &str
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Get the current function
        let current_fn = self.current_function()
            .ok_or_else(|| Error::codegen("No current function for array lookup"))?;
        
        // Create basic blocks for the loop
        let entry_block = self.builder().get_insert_block()
            .ok_or_else(|| Error::codegen("No current block for array lookup"))?;
        let loop_header = self.context().append_basic_block(current_fn, "array_lookup_header");
        let loop_body = self.context().append_basic_block(current_fn, "array_lookup_body");
        let loop_exit = self.context().append_basic_block(current_fn, "array_lookup_exit");
        let not_found = self.context().append_basic_block(current_fn, "array_lookup_not_found");
        
        // Create an alloca for the loop index
        let i32_type = self.context().i32_type();
        let i64_type = self.context().i64_type();
        let index_ptr = self.create_entry_block_alloca(i32_type, "lookup_index");
        
        // Create an alloca for the result
        let i8_ptr_type = self.context().i8_type().ptr_type(AddressSpace::default());
        let result_ptr = self.create_entry_block_alloca(i8_ptr_type, "lookup_result");
        
        // Initialize index to 0
        self.builder().build_store(index_ptr, i32_type.const_zero())
            .map_err(|e| Error::codegen(format!("Failed to initialize index: {}", e)))?;
        
        // Branch to loop header
        self.builder().build_unconditional_branch(loop_header)
            .map_err(|e| Error::codegen(format!("Failed to branch to loop header: {}", e)))?;
        
        // Loop header - check if we've reached the end
        self.builder().position_at_end(loop_header);
        let current_index = self.builder().build_load(i32_type, index_ptr, "current_index")
            .map_err(|e| Error::codegen(format!("Failed to load index: {}", e)))?;
        
        // Check if index < array_length
        let continue_loop = self.builder().build_int_compare(
            inkwell::IntPredicate::ULT,
            current_index.into_int_value(),
            i32_type.const_int(array_length as u64, false),
            "continue_loop"
        ).map_err(|e| Error::codegen(format!("Failed to compare index: {}", e)))?;
        
        // Branch based on the condition
        self.builder().build_conditional_branch(continue_loop, loop_body, not_found)
            .map_err(|e| Error::codegen(format!("Failed to branch in loop header: {}", e)))?;
        
        // Loop body - check if the current key matches
        self.builder().position_at_end(loop_body);
        
        // Get the current key
        let key_ptr = unsafe {
            self.builder().build_in_bounds_gep(
                i64_type.array_type(array_length),
                keys_array,
                &[i32_type.const_zero(), current_index.into_int_value()],
                "key_ptr"
            ).map_err(|e| Error::codegen(format!("Failed to get key pointer: {}", e)))?
        };
        
        let current_key = self.builder().build_load(i64_type, key_ptr, "current_key")
            .map_err(|e| Error::codegen(format!("Failed to load key: {}", e)))?;
        
        // Compare with target key
        let key_match = self.builder().build_int_compare(
            inkwell::IntPredicate::EQ,
            current_key.into_int_value(),
            target_key.into_int_value(),
            "key_match"
        ).map_err(|e| Error::codegen(format!("Failed to compare keys: {}", e)))?;
        
        // Create blocks for matching and non-matching cases
        let key_match_block = self.context().append_basic_block(current_fn, "key_match");
        let key_no_match_block = self.context().append_basic_block(current_fn, "key_no_match");
        
        // Branch based on key comparison
        self.builder().build_conditional_branch(key_match, key_match_block, key_no_match_block)
            .map_err(|e| Error::codegen(format!("Failed to branch on key match: {}", e)))?;
        
        // Key match - get the corresponding value
        self.builder().position_at_end(key_match_block);
        
        let value_ptr = unsafe {
            self.builder().build_in_bounds_gep(
                i8_ptr_type.array_type(array_length),
                values_array,
                &[i32_type.const_zero(), current_index.into_int_value()],
                "value_ptr"
            ).map_err(|e| Error::codegen(format!("Failed to get value pointer: {}", e)))?
        };
        
        let value = self.builder().build_load(i8_ptr_type, value_ptr, "value")
            .map_err(|e| Error::codegen(format!("Failed to load value: {}", e)))?;
        
        // Store the result and exit the loop
        self.builder().build_store(result_ptr, value)
            .map_err(|e| Error::codegen(format!("Failed to store result: {}", e)))?;
        
        self.builder().build_unconditional_branch(loop_exit)
            .map_err(|e| Error::codegen(format!("Failed to branch to loop exit: {}", e)))?;
        
        // Key no match - increment index and continue
        self.builder().position_at_end(key_no_match_block);
        
        let next_index = self.builder().build_int_add(
            current_index.into_int_value(),
            i32_type.const_int(1, false),
            "next_index"
        ).map_err(|e| Error::codegen(format!("Failed to increment index: {}", e)))?;
        
        self.builder().build_store(index_ptr, next_index)
            .map_err(|e| Error::codegen(format!("Failed to store next index: {}", e)))?;
        
        self.builder().build_unconditional_branch(loop_header)
            .map_err(|e| Error::codegen(format!("Failed to branch back to loop header: {}", e)))?;
        
        // Not found - return "Unknown Type"
        self.builder().position_at_end(not_found);
        
        let unknown_ptr = self.create_unknown_type_string()?;
        
        self.builder().build_store(result_ptr, unknown_ptr.into_pointer_value())
            .map_err(|e| Error::codegen(format!("Failed to store unknown result: {}", e)))?;
        
        self.builder().build_unconditional_branch(loop_exit)
            .map_err(|e| Error::codegen(format!("Failed to branch to loop exit from not found: {}", e)))?;
        
        // Loop exit - load and return the result
        self.builder().position_at_end(loop_exit);
        
        let result = self.builder().build_load(i8_ptr_type, result_ptr, result_name)
            .map_err(|e| Error::codegen(format!("Failed to load result: {}", e)))?;
        
        // Return to the original block
        self.builder().position_at_end(entry_block);
        
        Ok(result)
    }
}

/// Register the enhanced type registry implementation
pub fn register_enhanced_type_registry() {
    debug!("Registering enhanced type registry implementation");
    // This function is called during LlvmCodeGenerator initialization
}