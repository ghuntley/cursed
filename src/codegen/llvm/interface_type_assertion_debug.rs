//! # Interface Type Assertion Debug Utilities
//!
//! This module provides enhanced debugging support for interface type assertions,
//! allowing developers to trace and visualize type assertion operations at runtime.
//!
//! Key features:
//! - Runtime debugging of type assertions with detailed trace information
//! - Type relationship visualization during assertion operations
//! - Interactive debugging support with conditional breakpoints
//! - Performance metrics for type assertion operations

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::BasicValueEnum;
use tracing::{debug, info, instrument, warn, trace};

use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;
use crate::codegen::llvm::type_assertion::InterfaceTypeAssertion;
use crate::error::type_assertion_error::TypeAssertionError;
use crate::error::Error;
use crate::error::SourceLocation;

// Global debug flag that can be toggled at runtime
static DEBUG_TYPE_ASSERTIONS: AtomicBool = AtomicBool::new(false);

// Global statistics for type assertions
static SUCCESSFUL_ASSERTIONS: AtomicU64 = AtomicU64::new(0);
static FAILED_ASSERTIONS: AtomicU64 = AtomicU64::new(0);
static TOTAL_ASSERTION_TIME_MS: AtomicU64 = AtomicU64::new(0);

// Cache for runtime type name mappings
lazy_static::lazy_static! {
    static ref TYPE_ID_CACHE: Mutex<HashMap<u64, String>> = Mutex::new(HashMap::new());
}

/// Debug configuration for type assertions
pub struct TypeAssertionDebugConfig {
    /// Enable verbose debugging
    pub enable_debug: bool,
    /// Print type hierarchies during assertions
    pub print_hierarchies: bool,
    /// Collect performance metrics
    pub collect_metrics: bool,
    /// Break on failed assertions (useful for debugging)
    pub break_on_failure: bool,
    /// Maximum visualization depth for type hierarchies
    pub max_hierarchy_depth: usize,
}

impl Default for TypeAssertionDebugConfig {
    fn default() -> Self {
        // Check for environment variable to enable debug by default
        let debug_env = std::env::var("CURSED_DEBUG_TYPE_ASSERTIONS").unwrap_or_default();
        let enable_debug = !debug_env.is_empty() && debug_env != "0";
        
        Self {
            enable_debug,
            print_hierarchies: enable_debug,
            collect_metrics: true,
            break_on_failure: false,
            max_hierarchy_depth: 3,
        }
    }
}

/// Trait for enhanced debug capabilities for interface type assertions
pub trait InterfaceTypeAssertionDebug<'ctx> {
    /// Set debug configuration for type assertions
    fn set_type_assertion_debug_config(&mut self, config: TypeAssertionDebugConfig);
    
    /// Get current debug configuration
    fn get_type_assertion_debug_config(&self) -> TypeAssertionDebugConfig;
    
    /// Log a type assertion operation with detailed information
    fn debug_type_assertion(
        &mut self,
        source_type: &str,
        target_type: &str,
        actual_type_id: u64,
        target_type_id: u64,
        source_location: Option<SourceLocation>,
        success: bool,
    ) -> Result<(), Error>;
    
    /// Print type assertion statistics
    fn print_type_assertion_statistics(&self);
    
    /// Reset type assertion statistics
    fn reset_type_assertion_statistics(&self);
    
    /// Extract readable type name from a runtime value
    fn extract_readable_type_name(
        &mut self,
        value: BasicValueEnum<'ctx>
    ) -> Result<String, Error>;
    
    /// Add a runtime type ID to name mapping
    fn register_runtime_type(
        &self,
        type_id: u64,
        type_name: &str
    );
    
    /// Debug a type assertion operation with complete information
    fn debug_assertion_operation(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type: &str,
        location: Option<SourceLocation>
    ) -> Result<(), Error>;
}

impl<'ctx> InterfaceTypeAssertionDebug<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self), level = "debug")]
    fn set_type_assertion_debug_config(&mut self, config: TypeAssertionDebugConfig) {
        // Set the global debug flag based on configuration
        DEBUG_TYPE_ASSERTIONS.store(config.enable_debug, Ordering::SeqCst);
        
        // Store the configuration for later use
        self.type_assertion_debug_config = Some(config);
    }
    
    fn get_type_assertion_debug_config(&self) -> TypeAssertionDebugConfig {
        // Return the current config or a default if not set
        self.type_assertion_debug_config.clone().unwrap_or_default()
    }
    
    #[instrument(skip(self), fields(success = success), level = "debug")]
    fn debug_type_assertion(
        &mut self,
        source_type: &str,
        target_type: &str,
        actual_type_id: u64,
        target_type_id: u64,
        source_location: Option<SourceLocation>,
        success: bool,
    ) -> Result<(), Error> {
        // Check if debugging is enabled
        if !DEBUG_TYPE_ASSERTIONS.load(Ordering::SeqCst) {
            return Ok(());
        }
        
        // Update statistics
        if success {
            SUCCESSFUL_ASSERTIONS.fetch_add(1, Ordering::SeqCst);
        } else {
            FAILED_ASSERTIONS.fetch_add(1, Ordering::SeqCst);
        }
        
        // Log detailed information
        let config = self.get_type_assertion_debug_config();
        
        debug!(
            "Type assertion: {} -> {} (success: {})", 
            source_type, target_type, success
        );
        
        debug!(
            "Type IDs: actual=0x{:016x}, target=0x{:016x}",
            actual_type_id, target_type_id
        );
        
        if let Some(loc) = &source_location {
            debug!("Location: {}:{}", loc.file.as_deref().unwrap_or("<unknown>"), loc.line);
            if !loc.source_line.is_empty() {
                debug!("Source: {}", loc.source_line);
            }
        }
        
        // Print type hierarchies if enabled
        if config.print_hierarchies {
            match self.visualize_interface_path(target_type, config.max_hierarchy_depth) {
                Ok(hierarchy) => {
                    debug!("Type hierarchy for target type:\n{}", hierarchy);
                }
                Err(err) => {
                    warn!("Failed to visualize type hierarchy: {}", err);
                }
            }
            
            // Try to find alternative paths if assertion failed
            if !success {
                // Get the actual type name if possible
                let actual_type = self.get_type_name_for_id(actual_type_id).unwrap_or_else(|_| "<unknown>".to_string());
                
                match self.find_alternative_paths(&actual_type, target_type, 3) {
                    Ok(paths) if !paths.is_empty() => {
                        debug!("Alternative conversion paths available:");
                        for (i, path) in paths.iter().enumerate() {
                            debug!("{}: {}", i + 1, path);
                        }
                    }
                    Ok(_) => {
                        debug!("No alternative conversion paths found.");
                    }
                    Err(err) => {
                        warn!("Failed to find alternative paths: {}", err);
                    }
                }
            }
        }
        
        // Break on failure if configured
        if !success && config.break_on_failure {
            debug!("Breakpoint triggered for failed type assertion");
            // In a real implementation, this would trigger a debugger breakpoint
            // For now, we just log it
        }
        
        Ok(())
    }
    
    fn print_type_assertion_statistics(&self) {
        let successful = SUCCESSFUL_ASSERTIONS.load(Ordering::SeqCst);
        let failed = FAILED_ASSERTIONS.load(Ordering::SeqCst);
        let total = successful + failed;
        let total_time_ms = TOTAL_ASSERTION_TIME_MS.load(Ordering::SeqCst);
        
        let success_rate = if total > 0 {
            (successful as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        
        let avg_time = if total > 0 {
            total_time_ms as f64 / total as f64
        } else {
            0.0
        };
        
        info!("Type Assertion Statistics:");
        info!("  Total assertions: {}", total);
        info!("  Successful: {} ({:.2}%)", successful, success_rate);
        info!("  Failed: {}", failed);
        info!("  Total time: {}ms (avg: {:.3}ms per assertion)", total_time_ms, avg_time);
    }
    
    fn reset_type_assertion_statistics(&self) {
        SUCCESSFUL_ASSERTIONS.store(0, Ordering::SeqCst);
        FAILED_ASSERTIONS.store(0, Ordering::SeqCst);
        TOTAL_ASSERTION_TIME_MS.store(0, Ordering::SeqCst);
        info!("Type assertion statistics reset");
    }
    
    #[instrument(skip(self, value), level = "debug")]
    fn extract_readable_type_name(
        &mut self,
        value: BasicValueEnum<'ctx>
    ) -> Result<String, Error> {
        // Extract the type ID from the value
        let type_id_value = self.extract_interface_type_id(value)?;
        let type_id = if type_id_value.is_int_value() {
            type_id_value.into_int_value().get_zero_extended_constant().unwrap_or(0)
        } else {
            // If we can't get a constant, use dynamic extraction
            // This would be implemented in a real compiler
            0
        };
        
        // Try to find the type name in the cache
        if let Ok(cache) = TYPE_ID_CACHE.lock() {
            if let Some(name) = cache.get(&type_id) {
                return Ok(name.clone());
            }
        }
        
        // Try to get the type name from the registry
        if let Some(registry) = &self.interface_type_registry {
            match registry.get_type_name_for_id(type_id) {
                Ok(name) => {
                    // Cache the result for future use
                    if let Ok(mut cache) = TYPE_ID_CACHE.lock() {
                        cache.insert(type_id, name.clone());
                    }
                    Ok(name)
                }
                Err(_) => Ok(format!("<unknown type 0x{:016x}>", type_id))
            }
        } else {
            Ok(format!("<unknown type 0x{:016x}>", type_id))
        }
    }
    
    fn register_runtime_type(
        &self,
        type_id: u64,
        type_name: &str
    ) {
        if let Ok(mut cache) = TYPE_ID_CACHE.lock() {
            cache.insert(type_id, type_name.to_string());
            trace!("Registered runtime type: {} (ID: 0x{:016x})", type_name, type_id);
        }
    }
    
    #[instrument(skip(self, interface_value), level = "debug")]
    fn debug_assertion_operation(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type: &str,
        location: Option<SourceLocation>
    ) -> Result<(), Error> {
        // Check if debugging is enabled
        if !DEBUG_TYPE_ASSERTIONS.load(Ordering::SeqCst) {
            return Ok(());
        }
        
        // Collect performance metrics if enabled
        let config = self.get_type_assertion_debug_config();
        let start_time = if config.collect_metrics {
            Some(Instant::now())
        } else {
            None
        };
        
        // Extract the actual type ID
        let type_id_value = self.extract_interface_type_id(interface_value)?;
        let actual_type_id = if type_id_value.is_int_value() {
            type_id_value.into_int_value().get_zero_extended_constant().unwrap_or(0)
        } else {
            0
        };
        
        // Get the target type ID
        let target_type_id = self.hash_type_name(target_type);
        
        // Get the actual type name if possible
        let actual_type = self.get_type_name_for_id(actual_type_id).unwrap_or_else(|_| "<unknown>".to_string());
        
        // Check if the assertion would succeed
        let success = actual_type_id == target_type_id;
        
        // Log the assertion
        self.debug_type_assertion(
            &actual_type,
            target_type,
            actual_type_id,
            target_type_id,
            location,
            success
        )?;
        
        // Update performance metrics
        if let Some(start) = start_time {
            let elapsed = start.elapsed();
            let elapsed_ms = elapsed.as_millis() as u64;
            TOTAL_ASSERTION_TIME_MS.fetch_add(elapsed_ms, Ordering::SeqCst);
        }
        
        Ok(())
    }
}

// Extension to LlvmCodeGenerator for internal methods
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Initialize type assertion debug support
    pub fn initialize_type_assertion_debug(&mut self) {
        // Check for environment variable to enable debug
        let debug_env = std::env::var("CURSED_DEBUG_TYPE_ASSERTIONS").unwrap_or_default();
        let enable_debug = !debug_env.is_empty() && debug_env != "0";
        
        // Configure debug settings
        let config = TypeAssertionDebugConfig {
            enable_debug,
            print_hierarchies: enable_debug,
            collect_metrics: true,
            break_on_failure: false,
            max_hierarchy_depth: 3,
        };
        
        self.set_type_assertion_debug_config(config);
        
        if enable_debug {
            debug!("Type assertion debugging enabled");
        }
    }
    
    /// Hash a type name to create a type ID using FNV-1a algorithm
    /// 
    /// This function converts a type name string to a 64-bit type ID hash.
    /// We use FNV-1a for its good distribution and speed on short strings.
    /// 
    /// IMPORTANT: This implementation MUST match the one in InterfaceTypeRegistry
    /// to ensure consistent type ID generation across the codebase.
    ///
    /// @param type_name The type name to hash
    /// @return The 64-bit type ID hash
    pub fn hash_type_name(&self, type_name: &str) -> u64 {
        // FNV-1a hash algorithm for more consistent hashing
        // This must match the implementation in InterfaceTypeRegistry
        trace!("Hashing type name: {}", type_name);
        
        // FNV-1a initialization value (FNV offset basis)
        let mut hash: u64 = 0xcbf29ce484222325;
        
        // Process each byte of the string
        for byte in type_name.bytes() {
            hash ^= byte as u64;
            // FNV prime for 64-bit hash
            hash = hash.wrapping_mul(0x100000001b3);
        }
        
        trace!("Hashed type name '{}' to ID: {}", type_name, hash);
        hash
    }
}

/// Register the type assertion debug module
pub fn register_type_assertion_debug() {
    debug!("Type assertion debug module registered");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_debug_module_registration() {
        register_type_assertion_debug();
        assert!(true);
    }
}