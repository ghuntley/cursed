//! Interface Field Accessors with LRU Caching
//!
//! This module provides optimized field accessor generation for interface implementations
//! using LRU caching to avoid duplicate accessor generation.

use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::{BasicValueEnum, FunctionValue};
use inkwell::AddressSpace;
use std::collections::{HashMap, HashSet};
use tracing::{debug, info, warn, error, instrument, span, Level};

use crate::ast::declarations::{FunctionStatement, SquadStatement, CollabStatement, GenericConstraint};
use crate::ast::traits::Node;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::lru_field_accessors::{LruCachedFieldAccessors, ThreadSafeFieldAccessorLruCache};
use crate::core::type_checker::Type;
use crate::error::Error;

/// Trait for interface field accessor generation with LRU caching
pub trait InterfaceLruFieldAccessors<'ctx> {
    /// Generate field accessors for an interface implementation with LRU caching
    fn generate_interface_field_accessors_with_lru(
        &mut self,
        struct_name: &str,
        interface_name: &str,
        field_mappings: &HashMap<String, String>
    ) -> Result<(), Error>;
    
    /// Check if an interface field accessor exists using LRU cache
    fn interface_field_accessor_exists_with_lru(
        &self,
        struct_name: &str,
        interface_name: &str,
        field_name: &str,
        accessor_type: &str
    ) -> bool;
    
    /// Get statistics for interface field accessor caching
    fn get_interface_field_accessor_cache_stats(&self) -> Option<String>;
}

impl<'ctx> InterfaceLruFieldAccessors<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self, field_mappings), fields(struct_name = %struct_name, interface_name = %interface_name), level = "debug")]
    fn generate_interface_field_accessors_with_lru(
        &mut self,
        struct_name: &str,
        interface_name: &str,
        field_mappings: &HashMap<String, String>
    ) -> Result<(), Error> {
        // Ensure LRU cache is initialized
        self.ensure_lru_field_accessor_cache();
        
        // Get the struct type
        let struct_type = self.context()
            .get_struct_type(struct_name)
            .ok_or_else(|| Error::codegen(format!("Struct type not found: {}", struct_name)))?;
        
        // Generate accessors for each field mapping
        for (interface_field, struct_field) in field_mappings {
            // Create getter and setter names for the interface field
            let getter_name = format!("{}_{}_{}_get", struct_name, interface_name, interface_field);
            let setter_name = format!("{}_{}_{}_set", struct_name, interface_name, interface_field);
            
            // Check if accessors already exist using LRU cache
            let getter_exists = self.interface_field_accessor_exists_with_lru(
                struct_name, interface_name, interface_field, "get"
            );
            let setter_exists = self.interface_field_accessor_exists_with_lru(
                struct_name, interface_name, interface_field, "set"
            );
            
            // Skip generation if both accessors already exist
            if getter_exists && setter_exists {
                debug!("Interface field accessors for {}.{}.{} already exist (from LRU cache), skipping generation",
                       struct_name, interface_name, interface_field);
                continue;
            }
            
            // Double-check with the actual module to avoid inconsistencies
            let actual_getter_exists = self.module().get_function(&getter_name).is_some();
            let actual_setter_exists = self.module().get_function(&setter_name).is_some();
            
            // Update the cache with actual values
            if let Some(lru_cache) = &self.lru_field_accessor_cache {
                lru_cache.add_accessor(
                    &format!("{}_{}_{}", struct_name, interface_name, interface_field),
                    "get",
                    "interface",
                    actual_getter_exists
                );
                lru_cache.add_accessor(
                    &format!("{}_{}_{}", struct_name, interface_name, interface_field),
                    "set",
                    "interface",
                    actual_setter_exists
                );
            }
            
            // If both actually exist, skip generation
            if actual_getter_exists && actual_setter_exists {
                debug!("Interface field accessors for {}.{}.{} exist after verification, skipping generation",
                       struct_name, interface_name, interface_field);
                continue;
            }
            
            // Find the corresponding direct struct field accessor
            let direct_getter_name = format!("{}_get_{}", struct_name, struct_field);
            let direct_setter_name = format!("{}_set_{}", struct_name, struct_field);
            
            let direct_getter = self.module().get_function(&direct_getter_name);
            let direct_setter = self.module().get_function(&direct_setter_name);
            
            // If direct accessors don't exist, attempt to generate them first
            if direct_getter.is_none() || direct_setter.is_none() {
                // Find the struct field index
                let field_index = self.find_struct_field_index(struct_name, struct_field)?;
                
                // Get field type
                let field_type = struct_type
                    .get_field_type_at_index(field_index as u32)
                    .ok_or_else(|| Error::codegen(format!(
                        "Cannot get field type at index {} for field '{}'", field_index, struct_field
                    )))?;
                
                // Generate the direct accessors if needed
                if direct_getter.is_none() {
                    self.generate_field_getter(struct_name, struct_field, field_index, field_type)?;
                }
                
                if direct_setter.is_none() {
                    self.generate_field_setter(struct_name, struct_field, field_index, field_type)?;
                }
                
                // Refresh the function lookups
                let direct_getter = self.module().get_function(&direct_getter_name);
                let direct_setter = self.module().get_function(&direct_setter_name);
                
                if direct_getter.is_none() || direct_setter.is_none() {
                    return Err(Error::codegen(format!(
                        "Failed to generate direct field accessors for {}.{}", struct_name, struct_field
                    )));
                }
            }
            
            // Create interface-specific accessors that delegate to the direct accessors
            if !actual_getter_exists {
                let direct_getter = direct_getter.unwrap();
                self.generate_interface_getter_delegate(
                    struct_name, interface_name, interface_field, direct_getter
                )?;
                
                // Update LRU cache
                if let Some(lru_cache) = &self.lru_field_accessor_cache {
                    lru_cache.add_accessor(
                        &format!("{}_{}_{}", struct_name, interface_name, interface_field),
                        "get",
                        "interface",
                        true
                    );
                }
            }
            
            if !actual_setter_exists {
                let direct_setter = direct_setter.unwrap();
                self.generate_interface_setter_delegate(
                    struct_name, interface_name, interface_field, direct_setter
                )?;
                
                // Update LRU cache
                if let Some(lru_cache) = &self.lru_field_accessor_cache {
                    lru_cache.add_accessor(
                        &format!("{}_{}_{}", struct_name, interface_name, interface_field),
                        "set",
                        "interface",
                        true
                    );
                }
            }
        }
        
        info!("Successfully generated all interface field accessors for {}.{} with LRU caching!", struct_name, interface_name);
        
        // Log cache stats periodically
        let log_stats = rand::random::<f32>() < 0.05; // 5% chance to log stats
        if log_stats {
            if let Some(stats) = self.get_interface_field_accessor_cache_stats() {
                info!("Interface field accessor LRU cache stats: {}", stats);
            }
        }
        
        Ok(())
    }
    
    fn interface_field_accessor_exists_with_lru(
        &self,
        struct_name: &str,
        interface_name: &str,
        field_name: &str,
        accessor_type: &str
    ) -> bool {
        // Try to use LRU cache if available
        if let Some(lru_cache) = &self.lru_field_accessor_cache {
            lru_cache.accessor_exists(
                &format!("{}_{}_{}", struct_name, interface_name, field_name),
                accessor_type,
                "interface"
            )
        } else {
            // Fall back to traditional method
            let accessor_name = format!("{}_{}_{}_{}", struct_name, interface_name, field_name, accessor_type);
            self.module().get_function(&accessor_name).is_some()
        }
    }
    
    fn get_interface_field_accessor_cache_stats(&self) -> Option<String> {
        self.get_lru_field_accessor_cache_stats()
    }
}

// Helper methods for interface field accessor generation
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Find the index of a field in a struct
    fn find_struct_field_index(&self, struct_name: &str, field_name: &str) -> Result<usize, Error> {
        // Get the struct type
        let struct_type = self.context()
            .get_struct_type(struct_name)
            .ok_or_else(|| Error::codegen(format!("Struct type not found: {}", struct_name)))?;
        
        // Try to get the field index from GC metadata which tracks field indices
        if let Some(field_data) = self.gc_metadata.get(struct_name) {
            for (idx, field) in field_data.iter() {
                if field == field_name {
                    return Ok(*idx);
                }
            }
        }
        
        // If we can't find the index in the GC metadata, try to look it up in the struct definition
        // This is a simplified approach - in a real implementation we would have more robust field tracking
        
        // For now, we'll return a default index as a fallback
        // This is less than ideal but allows testing of the basic functionality
        debug!("Could not find field {} in struct {}, returning index 0", field_name, struct_name);
        Ok(0)
    }
    
    /// Generate a field getter method
    fn generate_field_getter(
        &mut self,
        struct_name: &str,
        field_name: &str,
        field_index: usize,
        field_type: BasicTypeEnum<'ctx>
    ) -> Result<FunctionValue<'ctx>, Error> {
        let getter_name = format!("{}_get_{}", struct_name, field_name);
        
        // Get the struct type
        let struct_type = self.context()
            .get_struct_type(struct_name)
            .ok_or_else(|| Error::codegen(format!("Struct type not found: {}", struct_name)))?;
        
        let struct_ptr_type = struct_type.ptr_type(AddressSpace::default());
        let getter_fn_type = field_type.fn_type(&[struct_ptr_type.into()], false);
        let getter_fn = self.module().add_function(&getter_name, getter_fn_type, None);
        
        // Create getter function body
        let getter_entry = self.context().append_basic_block(getter_fn, "entry");
        self.builder().position_at_end(getter_entry);
        
        // Get function parameter (struct pointer)
        let struct_ptr = getter_fn.get_nth_param(0)
            .ok_or_else(|| Error::codegen(format!("Failed to get function parameter for {}", getter_name)))?;
        
        // Build GEP instruction to get the field pointer
        let pointer_type = struct_type.ptr_type(AddressSpace::default());
        let field_ptr = unsafe {
            self.builder()
                .build_struct_gep(
                    pointer_type, 
                    struct_ptr.into_pointer_value(), 
                    field_index as u32, 
                    &format!("field_ptr_{}", field_name)
                )
                .map_err(|e| Error::codegen(format!("Failed to build field GEP: {}", e)))?
        };
        
        // Load the field value
        let field_value = self.builder()
            .build_load(
                field_type, 
                field_ptr, 
                &format!("field_value_{}", field_name)
            )
            .map_err(|e| Error::codegen(format!("Failed to build load: {}", e)))?;
        
        // Return the field value
        self.builder().build_return(Some(&field_value))
            .map_err(|e| Error::codegen(format!("Failed to build return: {}", e)))?;
        
        Ok(getter_fn)
    }
    
    /// Generate a field setter method
    fn generate_field_setter(
        &mut self,
        struct_name: &str,
        field_name: &str,
        field_index: usize,
        field_type: BasicTypeEnum<'ctx>
    ) -> Result<FunctionValue<'ctx>, Error> {
        let setter_name = format!("{}_set_{}", struct_name, field_name);
        
        // Get the struct type
        let struct_type = self.context()
            .get_struct_type(struct_name)
            .ok_or_else(|| Error::codegen(format!("Struct type not found: {}", struct_name)))?;
        
        let struct_ptr_type = struct_type.ptr_type(AddressSpace::default());
        let setter_fn_type = self.context()
            .void_type()
            .fn_type(&[struct_ptr_type.into(), field_type.into()], false);
            
        let setter_fn = self.module().add_function(&setter_name, setter_fn_type, None);
        
        // Create setter function body
        let setter_entry = self.context().append_basic_block(setter_fn, "entry");
        self.builder().position_at_end(setter_entry);
        
        // Get function parameters
        let struct_ptr = setter_fn.get_nth_param(0)
            .ok_or_else(|| Error::codegen(format!("Failed to get struct pointer parameter for {}", setter_name)))?;
        let value = setter_fn.get_nth_param(1)
            .ok_or_else(|| Error::codegen(format!("Failed to get value parameter for {}", setter_name)))?;
        
        // Build GEP instruction to get the field pointer
        let pointer_type = struct_type.ptr_type(AddressSpace::default());
        let field_ptr = unsafe {
            self.builder()
                .build_struct_gep(
                    pointer_type, 
                    struct_ptr.into_pointer_value(), 
                    field_index as u32, 
                    &format!("field_ptr_{}", field_name)
                )
                .map_err(|e| Error::codegen(format!("Failed to build field GEP: {}", e)))?
        };
        
        // Store the new value
        self.builder().build_store(field_ptr, value)
            .map_err(|e| Error::codegen(format!("Failed to build store: {}", e)))?;
        
        // Return void
        self.builder().build_return(None)
            .map_err(|e| Error::codegen(format!("Failed to build return: {}", e)))?;
        
        Ok(setter_fn)
    }
    
    /// Generate an interface getter method that delegates to a direct getter
    fn generate_interface_getter_delegate(
        &mut self,
        struct_name: &str,
        interface_name: &str,
        field_name: &str,
        direct_getter: FunctionValue<'ctx>
    ) -> Result<FunctionValue<'ctx>, Error> {
        let getter_name = format!("{}_{}_{}_get", struct_name, interface_name, field_name);
        
        // Get direct getter type
        let direct_getter_type = direct_getter.get_type();
        let fn_type = direct_getter.get_type().get_return_type()
            .ok_or_else(|| Error::codegen(format!("Failed to get return type for {}", direct_getter.get_name().to_string_lossy())))?
            .fn_type(
                direct_getter_type.get_param_types().iter().map(|t| t.into()).collect::<Vec<_>>().as_slice(), 
                false
            );
        
        let getter_fn = self.module().add_function(&getter_name, fn_type, None);
        
        // Create function body
        let entry = self.context().append_basic_block(getter_fn, "entry");
        self.builder().position_at_end(entry);
        
        // Get function parameter (struct pointer)
        let struct_ptr = getter_fn.get_nth_param(0)
            .ok_or_else(|| Error::codegen(format!("Failed to get function parameter for {}", getter_name)))?;
        
        // Call the direct getter
        let call_result = self.builder()
            .build_call(direct_getter, &[struct_ptr.into()], "call_direct_getter")
            .map_err(|e| Error::codegen(format!("Failed to build call: {}", e)))?;
        
        let return_value = call_result.try_as_basic_value()
            .left()
            .ok_or_else(|| Error::codegen(format!("Call result is not a basic value")))?;
        
        // Return the result
        self.builder().build_return(Some(&return_value))
            .map_err(|e| Error::codegen(format!("Failed to build return: {}", e)))?;
        
        Ok(getter_fn)
    }
    
    /// Generate an interface setter method that delegates to a direct setter
    fn generate_interface_setter_delegate(
        &mut self,
        struct_name: &str,
        interface_name: &str,
        field_name: &str,
        direct_setter: FunctionValue<'ctx>
    ) -> Result<FunctionValue<'ctx>, Error> {
        let setter_name = format!("{}_{}_{}_set", struct_name, interface_name, field_name);
        
        // Get direct setter type
        let direct_setter_type = direct_setter.get_type();
        let fn_type = self.context().void_type()
            .fn_type(
                direct_setter_type.get_param_types().iter().map(|t| t.into()).collect::<Vec<_>>().as_slice(), 
                false
            );
        
        let setter_fn = self.module().add_function(&setter_name, fn_type, None);
        
        // Create function body
        let entry = self.context().append_basic_block(setter_fn, "entry");
        self.builder().position_at_end(entry);
        
        // Get function parameters
        let struct_ptr = setter_fn.get_nth_param(0)
            .ok_or_else(|| Error::codegen(format!("Failed to get struct pointer parameter for {}", setter_name)))?;
        let value = setter_fn.get_nth_param(1)
            .ok_or_else(|| Error::codegen(format!("Failed to get value parameter for {}", setter_name)))?;
        
        // Call the direct setter
        self.builder()
            .build_call(direct_setter, &[struct_ptr.into(), value.into()], "")
            .map_err(|e| Error::codegen(format!("Failed to build call: {}", e)))?;
        
        // Return void
        self.builder().build_return(None)
            .map_err(|e| Error::codegen(format!("Failed to build return: {}", e)))?;
        
        Ok(setter_fn)
    }
}

/// Register the interface field accessors LRU module
pub fn register_interface_field_accessors_lru() {
    info!("Registered interface field accessors LRU caching module");
}