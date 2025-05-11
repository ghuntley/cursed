//! Optimized Dynamic Dispatch for Interfaces
//! 
//! This module provides an optimized implementation of dynamic dispatch for interfaces
//! with inline caching and speculative dispatch for better performance. It builds on the
//! existing dynamic_dispatch.rs implementation but adds several optimizations:
//! 
//! 1. Method Call Caching - Avoiding repeated vtable lookups for the same method
//! 2. Inline Caching - Fast path for frequently called methods on known types
//! 3. Speculative Dispatch - Runtime type prediction with optimized dispatch paths
//! 4. Batched Lookup - More efficient handling of multiple lookups
//! 5. Enhanced Type Feedback - Runtime profiling for optimization decisions

use crate::error::Error;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::dynamic_dispatch::{InterfaceManager, InterfaceStructure, VTable, VTableImpl, TypeInfo};
#[cfg(feature = "enhanced_dynamic_dispatch")]
use crate::codegen::llvm::enhanced_dynamic_dispatch::EnhancedDynamicDispatch;
use crate::core::type_checker::Type as CursedType;
use inkwell::types::{BasicTypeEnum, FunctionType, PointerType, StructType};
use inkwell::values::{BasicValueEnum, BasicValue, FunctionValue, PointerValue, BasicMetadataValueEnum};
use inkwell::AddressSpace;
use inkwell::context::Context;
use inkwell::IntPredicate;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::{Arc, Mutex, RwLock};
use std::cell::RefCell;
use tracing::{debug, error, info, instrument, warn, span, Level};

/// Cache for method dispatch to avoid repeated vtable lookups
#[derive(Debug)]
pub struct MethodDispatchCache<'ctx> {
    /// Maps (interface_name, method_name) to cached method information
    method_cache: HashMap<(String, String), CachedMethod<'ctx>>,
    /// Maps interface_name to a set of concrete type names known to implement it
    implementing_types: HashMap<String, HashSet<String>>,
    /// Cache hit count for statistics
    cache_hits: usize,
    /// Cache miss count for statistics
    cache_misses: usize,
}

/// Information about a cached method for faster dispatch
#[derive(Debug)]
pub struct CachedMethod<'ctx> {
    /// Original method name
    pub method_name: String,
    /// Interface name this method belongs to
    pub interface_name: String,
    /// Method index in the vtable for faster lookup
    pub vtable_index: usize,
    /// LLVM function type for calling the method
    pub function_type: FunctionType<'ctx>,
    /// Concrete types that have been observed calling this method
    pub observed_types: HashSet<String>,
    /// Counter for calls to this method for optimization decisions
    pub call_count: usize,
    /// Whether this method has been promoted to speculative dispatch
    pub speculative: bool,
}

/// Enhanced interface dynamic dispatch trait with optimizations
pub trait OptimizedDynamicDispatch<'ctx>: EnhancedDynamicDispatch<'ctx> {
    /// Initialize the optimized dynamic dispatch system
    fn init_optimized_dynamic_dispatch(&mut self) -> Result<(), Error>;
    
    /// Call an interface method with optimized dispatch
    fn call_interface_method_optimized(
        &mut self,
        interface_ptr: PointerValue<'ctx>,
        interface_name: &str,
        method_name: &str,
        args: &[BasicValueEnum<'ctx>],
    ) -> Result<Option<BasicValueEnum<'ctx>>, Error>;
    
    /// Generate inline cache for interface method calls
    fn generate_inline_cache(
        &mut self,
        interface_ptr: PointerValue<'ctx>,
        interface_name: &str,
        method_name: &str,
        expected_type: &str,
    ) -> Result<(), Error>;
    
    /// Generate speculative dispatch for a method call
    fn generate_speculative_dispatch(
        &mut self,
        interface_ptr: PointerValue<'ctx>,
        interface_name: &str,
        method_name: &str,
        args: &[BasicValueEnum<'ctx>],
        expected_types: &[&str],
    ) -> Result<Option<BasicValueEnum<'ctx>>, Error>;
    
    /// Get statistics about the method dispatch cache
    fn get_dispatch_statistics(&self) -> Result<HashMap<String, usize>, Error>;
    
    /// Perform batch registration of interface implementations
    fn batch_register_interface_implementations(
        &mut self,
        implementations: HashMap<(String, String), HashMap<String, FunctionValue<'ctx>>>,
    ) -> Result<(), Error>;
    
    /// Reset the performance statistics in the dispatch cache
    fn reset_dispatch_statistics(&mut self) -> Result<(), Error>;
}

impl<'ctx> OptimizedDynamicDispatch<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn init_optimized_dynamic_dispatch(&mut self) -> Result<(), Error> {
        debug!("Initializing optimized dynamic dispatch system");
        
        // Initialize the method dispatch cache if it doesn't exist
        if self.method_dispatch_cache.is_none() {
            self.method_dispatch_cache = Some(MethodDispatchCache {
                method_cache: HashMap::new(),
                implementing_types: HashMap::new(),
                cache_hits: 0,
                cache_misses: 0,
            });
            info!("Method dispatch cache initialized");
        }
        
        // Ensure the interface manager is initialized
        if self.interface_manager.is_none() {
            self.interface_manager = Some(InterfaceManager::new());
            info!("Interface manager initialized");
        }
        
        Ok(())
    }
    
    #[instrument(skip(self, interface_ptr, args), fields(interface_name = %interface_name, method_name = %method_name), level = "debug")]
    fn call_interface_method_optimized(
        &mut self,
        interface_ptr: PointerValue<'ctx>,
        interface_name: &str,
        method_name: &str,
        args: &[BasicValueEnum<'ctx>],
    ) -> Result<Option<BasicValueEnum<'ctx>>, Error> {
        // Create a span for tracing this method call
        let _span = span!(Level::DEBUG, "optimized_dispatch", method = %method_name).entered();
        debug!("Calling interface method with optimized dispatch");
        
        // Make sure the optimized dispatch system is initialized
        if self.method_dispatch_cache.is_none() || self.interface_manager.is_none() {
            self.init_optimized_dynamic_dispatch()?;
        }
        
        // 1. Check if we have a cached method entry
        let cache_key = (interface_name.to_string(), method_name.to_string());
        let mut method_info = None;
        let mut cache_hit = false;
        
        // Use a block to ensure we release the reference to method_dispatch_cache before the next mutable borrow
        {
            if let Some(cache) = &mut self.method_dispatch_cache {
                if let Some(cached_method) = cache.method_cache.get_mut(&cache_key) {
                    method_info = Some(cached_method);
                    cached_method.call_count += 1;
                    cache_hit = true;
                    cache.cache_hits += 1;
                } else {
                    cache.cache_misses += 1;
                }
            }
        }
        
        // If we don't have a cached entry, create one
        if method_info.is_none() {
            debug!("Cache miss for method {}::{}", interface_name, method_name);
            
            // Get interface and vtable info to create a cache entry
            let interface_manager = self.interface_manager.as_ref().unwrap();
            
            let interface = match interface_manager.get_interface(interface_name) {
                Some(interface) => interface,
                None => return Err(Error::from_str(&format!(
                    "Unknown interface: {}", 
                    interface_name
                ))),
            };

            let vtable = match interface_manager.get_vtable(interface_name) {
                Some(vtable) => vtable,
                None => return Err(Error::from_str(&format!(
                    "No vtable found for interface: {}", 
                    interface_name
                ))),
            };

            // Get the method index in the vtable
            let method_index = match vtable.method_indices.get(method_name) {
                Some(index) => *index,
                None => return Err(Error::from_str(&format!(
                    "Interface '{}' does not have method: {}", 
                    interface_name, 
                    method_name
                ))),
            };

            // Get the method's signature information
            let method_signature = match vtable.method_signatures.get(method_index) {
                Some(signature) => signature,
                None => return Err(Error::from_str(&format!(
                    "Method signature not found for '{}' in interface: {}", 
                    method_name, 
                    interface_name
                ))),
            };
            
            // Create a new cache entry
            let new_cache_entry = CachedMethod {
                method_name: method_name.to_string(),
                interface_name: interface_name.to_string(),
                vtable_index: method_index,
                function_type: method_signature.function_type,
                observed_types: HashSet::new(),
                call_count: 1,
                speculative: false,
            };
            
            // Update the cache
            if let Some(cache) = &mut self.method_dispatch_cache {
                cache.method_cache.insert(cache_key, new_cache_entry);
            }
            
            // Proceed with the standard dynamic dispatch since we don't have optimization info yet
            return self.call_interface_method_enhanced(interface_ptr, interface_name, method_name, args);
        }
        
        // 2. For frequently called methods, try speculative dispatch
        if let Some(ref cached_method) = method_info {
            if cached_method.call_count > 10 && cached_method.observed_types.len() <= 3 {
                debug!("Using speculative dispatch for frequently called method");
                
                // Convert the observed types to str slices for the speculative dispatch function
                let expected_types: Vec<&str> = if let Some(cache) = &self.method_dispatch_cache {
                    if let Some(cached) = cache.method_cache.get(&cache_key) {
                        cached.observed_types.iter().map(|s| s.as_str()).collect()
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                };
                
                if !expected_types.is_empty() {
                    // Try speculative dispatch with the observed types
                    return self.generate_speculative_dispatch(
                        interface_ptr,
                        interface_name,
                        method_name,
                        args,
                        &expected_types,
                    );
                }
            }
        }
        
        // 3. Fall back to enhanced dynamic dispatch if speculative dispatch isn't appropriate yet
        let result = self.call_interface_method_enhanced(interface_ptr, interface_name, method_name, args)?;
        
        // 4. Update observed types for future optimization
        self.update_type_observations(interface_ptr, interface_name, method_name)?;
        
        debug!("Completed optimized dispatch call");
        Ok(result)
    }
    
    #[instrument(skip(self, interface_ptr), fields(interface_name = %interface_name, method_name = %method_name, expected_type = %expected_type), level = "debug")]
    fn generate_inline_cache(
        &mut self,
        interface_ptr: PointerValue<'ctx>,
        interface_name: &str,
        method_name: &str,
        expected_type: &str,
    ) -> Result<(), Error> {
        debug!("Generating inline cache for method {} with expected type {}", method_name, expected_type);
        
        // This function adds an inline cache check for the expected type
        // by generating a conditional that checks if the vtable matches the expected type
        // Not fully implemented in this version, as it requires more context to generate the inline cache efficiently
        
        // Basic implementation to update our knowledge of implementing types
        if let Some(cache) = &mut self.method_dispatch_cache {
            let types = cache.implementing_types.entry(interface_name.to_string())
                .or_insert_with(HashSet::new);
            types.insert(expected_type.to_string());
            
            debug!("Updated implementing types for {}", interface_name);
        }
        
        Ok(())
    }
    
    #[instrument(skip(self, interface_ptr, args), fields(interface_name = %interface_name, method_name = %method_name), level = "debug")]
    fn generate_speculative_dispatch(
        &mut self,
        interface_ptr: PointerValue<'ctx>,
        interface_name: &str,
        method_name: &str,
        args: &[BasicValueEnum<'ctx>],
        expected_types: &[&str],
    ) -> Result<Option<BasicValueEnum<'ctx>>, Error> {
        debug!("Generating speculative dispatch for {} expected types", expected_types.len());
        
        // Make sure we have a valid interface pointer
        if self.check_interface_null(interface_ptr, &format!("speculative dispatch for {}", method_name))? {
            return Err(Error::from_str(&format!(
                "Cannot call method '{}' on null interface value of type {}",
                method_name, interface_name
            )));
        }
        
        // Get the interface manager
        let interface_manager = match &self.interface_manager {
            Some(manager) => manager,
            None => return Err(Error::from_str("Interface manager not initialized")),
        };
        
        // Extract the vtable pointer for type checking
        let vtable_ptr = self.extract_vtable_pointer(interface_ptr)?;
        
        // For each expected type, we'll generate a fast path
        let current_function = self.builder().get_insert_block().unwrap().get_parent().unwrap();
        
        // Create blocks for the dispatch paths
        let mut type_blocks = Vec::new();
        for _ in expected_types {
            let block = self.context().append_basic_block(current_function, "type_check");
            type_blocks.push(block);
        }
        
        // Create a fallback block for the slow path
        let fallback_block = self.context().append_basic_block(current_function, "fallback");
        
        // Create a result block
        let result_block = self.context().append_basic_block(current_function, "result");
        
        // Prepare a PHI node for the result
        self.builder().position_at_end(result_block);
        // We don't know the result type yet, so we'll fix this after generating the calls
        let result_phi_placeholder = self.builder().build_phi(
            self.context().i32_type(), 
            "result_phi"
        ).unwrap();
        
        // Generate the type checks and dispatch fast paths
        self.builder().position_at_end(self.builder().get_insert_block().unwrap());
        
        // Get the cache key for the method
        let cache_key = (interface_name.to_string(), method_name.to_string());
        
        // We need to track incoming values for the PHI node
        let mut incoming_values = Vec::new();
        let mut incoming_blocks = Vec::new();
        
        // For each expected type, generate a fast path
        for (i, &type_name) in expected_types.iter().enumerate() {
            // Look up the vtable for this type
            if let Some(vtable_impl) = interface_manager.get_vtable_impl(interface_name, type_name) {
                // Generated code will check if the interface's vtable matches this type's vtable
                let type_vtable_ptr = vtable_impl.vtable_global;
                
                // Compare the vtable pointers
                let vtable_match = self.builder().build_int_compare(
                    IntPredicate::EQ,
                    vtable_ptr,
                    type_vtable_ptr,
                    &format!("vtable_match_{}", type_name)
                ).unwrap();
                
                // Branch to the fast path if the vtable matches
                if i < expected_types.len() - 1 {
                    // If not the last type, branch to next type check on failure
                    self.builder().build_conditional_branch(
                        vtable_match,
                        type_blocks[i],
                        type_blocks[i + 1]
                    ).unwrap();
                } else {
                    // If last type, branch to fallback on failure
                    self.builder().build_conditional_branch(
                        vtable_match,
                        type_blocks[i],
                        fallback_block
                    ).unwrap();
                }
                
                // Position at the fast path block
                self.builder().position_at_end(type_blocks[i]);
                
                // Generate direct method call logic using direct dispatch
                // for the concrete type we know
                let type_struct = CursedType::Struct(type_name.to_string(), Vec::new());
                
                // Extract the data pointer
                let data_ptr = self.extract_data_pointer(interface_ptr)?;
                
                // Cast to the concrete type
                // This would be more complex in practice and involve proper struct type determination
                // For now, we just use the i8* pointer directly
                
                // Look up the method implementation
                if let Some(vtable) = interface_manager.get_vtable(interface_name) {
                    // Get the method index
                    if let Some(&method_index) = vtable.method_indices.get(method_name) {
                        // Get the method signature
                        if let Some(method_signature) = vtable.method_signatures.get(method_index) {
                            // Get the function pointer from the vtable
                            let typed_vtable_ptr = self.builder()
                                .build_bitcast(
                                    type_vtable_ptr,
                                    vtable.vtable_type.ptr_type(AddressSpace::default()),
                                    "typed_vtable_ptr"
                                )
                                .expect("Failed to cast vtable pointer")
                                .into_pointer_value();
                            
                            let fn_ptr_ptr = unsafe {
                                self.builder().build_struct_gep(
                                    vtable.vtable_type,
                                    typed_vtable_ptr, 
                                    method_index as u32, 
                                    "fn_ptr_ptr"
                                )
                            }.map_err(|e| Error::from_str(&format!("Failed to get function pointer: {}", e)))?;
                            
                            // Get the method signature function type pointer for correct loading
                            let fn_ptr_type = method_signature.function_type.ptr_type(AddressSpace::default());
                            
                            let fn_ptr = self.builder()
                                .build_load(fn_ptr_type, fn_ptr_ptr, "fn_ptr")
                                .map_err(|e| Error::from_str(&format!("Failed to load function pointer: {}", e)))?;
                                
                            let fn_ptr = fn_ptr.into_pointer_value();
                            
                            // Call the function through the function pointer
                            // Create a new array of arguments with the data pointer as the first argument (self pointer)
                            let mut real_args = vec![data_ptr.into()];
                            real_args.extend_from_slice(args);
                            
                            // Convert BasicValueEnum to BasicMetadataValueEnum for the arguments
                            let metadata_args: Vec<_> = real_args.iter().map(|arg| {
                                (*arg).into()
                            }).collect();
                            
                            // Call the function
                            let call_site = self.builder().build_indirect_call(
                                method_signature.function_type,
                                fn_ptr,
                                &metadata_args,
                                &format!("fast_call_{}", type_name)
                            ).map_err(|e| Error::from_str(&format!("Failed to call interface method: {}", e)))?;
                            
                            // Get the result if any
                            let result_value = call_site.try_as_basic_value().left();
                            
                            // Remember this result for the PHI node
                            if let Some(value) = result_value {
                                incoming_values.push(value);
                                incoming_blocks.push(self.builder().get_insert_block().unwrap());
                            }
                            
                            // Branch to the result block
                            self.builder().build_unconditional_branch(result_block).unwrap();
                        }
                    }
                }
            }
            
            // Reset builder position to the next block or the fallback
            if i < expected_types.len() - 1 {
                self.builder().position_at_end(type_blocks[i + 1]);
            } else {
                self.builder().position_at_end(fallback_block);
            }
        }
        
        // Fallback path with regular dynamic dispatch
        self.builder().position_at_end(fallback_block);
        
        // Call the regular dynamic dispatch method
        let fallback_result = self.call_interface_method_enhanced(
            interface_ptr,
            interface_name,
            method_name,
            args
        )?;
        
        // Add the fallback result to the PHI node if it exists
        if let Some(value) = fallback_result {
            incoming_values.push(value);
            incoming_blocks.push(self.builder().get_insert_block().unwrap());
        }
        
        // Branch to the result block
        self.builder().build_unconditional_branch(result_block).unwrap();
        
        // Position at the result block to fix up the PHI node
        self.builder().position_at_end(result_block);
        
        // If we have results, create a proper PHI node with the correct type
        let result = if !incoming_values.is_empty() {
            // Determine the actual result type from the first value
            let first_value = incoming_values[0];
            let phi_type = first_value.get_type();
            
            // Create a new PHI node with the correct type
            let phi = self.builder().build_phi(phi_type, "result_phi").unwrap();
            
            // Add incoming values
            for (value, block) in incoming_values.iter().zip(incoming_blocks.iter()) {
                phi.add_incoming(&[(&value, &block)]);
            }
            
            Some(phi.as_basic_value())
        } else {
            None
        };
        
        debug!("Completed speculative dispatch generation");
        Ok(result)
    }
    
    fn get_dispatch_statistics(&self) -> Result<HashMap<String, usize>, Error> {
        let mut stats = HashMap::new();
        
        if let Some(cache) = &self.method_dispatch_cache {
            stats.insert("cache_hits".to_string(), cache.cache_hits);
            stats.insert("cache_misses".to_string(), cache.cache_misses);
            stats.insert("cached_methods".to_string(), cache.method_cache.len());
            
            // Count speculative methods
            let speculative_count = cache.method_cache.values()
                .filter(|m| m.speculative)
                .count();
            stats.insert("speculative_methods".to_string(), speculative_count);
            
            // Total call count across all methods
            let total_calls = cache.method_cache.values()
                .map(|m| m.call_count)
                .sum();
            stats.insert("total_calls".to_string(), total_calls);
        } else {
            stats.insert("cache_hits".to_string(), 0);
            stats.insert("cache_misses".to_string(), 0);
            stats.insert("cached_methods".to_string(), 0);
            stats.insert("speculative_methods".to_string(), 0);
            stats.insert("total_calls".to_string(), 0);
        }
        
        Ok(stats)
    }
    
    fn batch_register_interface_implementations(
        &mut self,
        implementations: HashMap<(String, String), HashMap<String, FunctionValue<'ctx>>>,
    ) -> Result<(), Error> {
        debug!("Batch registering {} interface implementations", implementations.len());
        
        let interface_manager = match &mut self.interface_manager {
            Some(manager) => manager,
            None => {
                self.interface_manager = Some(InterfaceManager::new());
                self.interface_manager.as_mut().unwrap()
            }
        };
        
        // Process all implementations
        for ((struct_name, interface_name), methods) in implementations {
            debug!("Registering {} as implementing {} with {} methods", 
                  struct_name, interface_name, methods.len());
            
            // Create the CursedType for the struct
            let struct_type = CursedType::Struct(struct_name.clone(), Vec::new());
            
            // Register the implementation with the interface manager
            interface_manager.create_vtable_for_implementation(
                self.context(),
                self.module(),
                &interface_name,
                &struct_type,
                methods,
            )?;
            
            // Update our cache of implementing types
            if let Some(cache) = &mut self.method_dispatch_cache {
                let types = cache.implementing_types.entry(interface_name.clone())
                    .or_insert_with(HashSet::new);
                types.insert(struct_name.clone());
            }
        }
        
        debug!("Batch registration complete");
        Ok(())
    }
    
    fn reset_dispatch_statistics(&mut self) -> Result<(), Error> {
        if let Some(cache) = &mut self.method_dispatch_cache {
            cache.cache_hits = 0;
            cache.cache_misses = 0;
            
            // Reset individual method call counts but preserve type observations
            for method in cache.method_cache.values_mut() {
                method.call_count = 0;
            }
            
            debug!("Reset dispatch statistics");
        }
        
        Ok(())
    }
}

/// Helper trait to add the method dispatch cache to LlvmCodeGenerator
pub trait OptimizedDynamicDispatchExtensions<'ctx> {
    /// Update the observed types for a method call
    fn update_type_observations(
        &mut self,
        interface_ptr: PointerValue<'ctx>,
        interface_name: &str,
        method_name: &str,
    ) -> Result<(), Error>;
}

impl<'ctx> OptimizedDynamicDispatchExtensions<'ctx> for LlvmCodeGenerator<'ctx> {
    fn update_type_observations(
        &mut self,
        interface_ptr: PointerValue<'ctx>,
        interface_name: &str,
        method_name: &str,
    ) -> Result<(), Error> {
        // Extract the vtable pointer
        let vtable_ptr = self.extract_vtable_pointer(interface_ptr)?;
        
        // Get the interface manager
        let interface_manager = match &self.interface_manager {
            Some(manager) => manager,
            None => return Ok(()), // Skip if interface manager isn't initialized
        };
        
        // Try to determine the concrete type from the vtable pointer
        let mut concrete_type = None;
        
        // Check all registered vtable implementations
        let implementing_types = if let Some(cache) = &self.method_dispatch_cache {
            cache.implementing_types.get(interface_name).cloned().unwrap_or_default()
        } else {
            HashSet::new()
        };
        
        for type_name in implementing_types {
            if let Some(vtable_impl) = interface_manager.get_vtable_impl(interface_name, &type_name) {
                // Compare vtable pointers
                let result = self.compare_vtable_pointers(vtable_ptr, vtable_impl.vtable_global)?;
                
                // Check if the comparison is true
                if result.is_int_value() {
                    let int_val = result.into_int_value();
                    if int_val.get_zero_extended_value() != 0 {
                        concrete_type = Some(type_name.clone());
                        break;
                    }
                }
            }
        }
        
        // Update the observed types for this method
        if let Some(type_name) = concrete_type {
            let cache_key = (interface_name.to_string(), method_name.to_string());
            
            if let Some(cache) = &mut self.method_dispatch_cache {
                if let Some(method_info) = cache.method_cache.get_mut(&cache_key) {
                    method_info.observed_types.insert(type_name);
                    debug!("Updated observed types for method {}::{}", interface_name, method_name);
                }
            }
        }
        
        Ok(())
    }
}

/// Trait to add the method dispatch cache field to LlvmCodeGenerator
pub trait WithMethodDispatchCache<'ctx> {
    /// Get a reference to the method dispatch cache
    fn method_dispatch_cache(&self) -> Option<&MethodDispatchCache<'ctx>>;
    
    /// Get a mutable reference to the method dispatch cache
    fn method_dispatch_cache_mut(&mut self) -> Option<&mut MethodDispatchCache<'ctx>>;
}

// Helper function to register the optimized dynamic dispatch module
pub fn register_optimized_dynamic_dispatch() {
    debug!("Registering optimized dynamic dispatch module");
}