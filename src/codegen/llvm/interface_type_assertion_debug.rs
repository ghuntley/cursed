//! Enhanced Runtime Type Debugging for Interface Type Assertions
//!
//! This module provides additional debugging capabilities for interface type
//! assertions, including detailed error messages, type visualization, and
//! runtime type inspection. These features help developers diagnose and fix
//! type-related issues in interface assertions.
//!
//! The debugging system integrates with the path visualization capabilities
//! to provide comprehensive information about type hierarchies and inheritance
//! relationships when type assertions fail.

use inkwell::values::BasicValueEnum;
use inkwell::IntPredicate;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::error::Error;
use crate::error::SourceLocation;
use crate::error::type_assertion_error::TypeAssertionError;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;

/// Configuration for the type assertion debugging system
/// Controls the level of detail and when debugging information is displayed
#[derive(Debug, Clone)]
pub struct TypeAssertionDebugConfig {
    /// Whether to print detailed type information for all assertions
    pub print_all_assertions: bool,
    
    /// Whether to print only failed assertions
    pub print_failed_assertions: bool,
    
    /// Whether to include type hierarchy information
    pub include_hierarchy: bool,
    
    /// Whether to include type path visualization
    pub include_path_visualization: bool,
    
    /// Whether to generate runtime debug information
    pub runtime_debug: bool,
}

impl Default for TypeAssertionDebugConfig {
    fn default() -> Self {
        // Read debug level from environment
        let debug_env = env::var("CURSED_DEBUG").unwrap_or_else(|_| "0".to_string());
        let debug_level = debug_env.parse::<u8>().unwrap_or(0);
        
        match debug_level {
            0 => Self {
                print_all_assertions: false,
                print_failed_assertions: false,
                include_hierarchy: false,
                include_path_visualization: false,
                runtime_debug: false,
            },
            1 => Self {
                print_all_assertions: false,
                print_failed_assertions: true,
                include_hierarchy: false,
                include_path_visualization: true,
                runtime_debug: false,
            },
            _ => Self {
                print_all_assertions: true,
                print_failed_assertions: true,
                include_hierarchy: true,
                include_path_visualization: true,
                runtime_debug: true,
            },
        }
    }
}

// Global flag to track if debugging is registered
static DEBUG_REGISTERED: AtomicBool = AtomicBool::new(false);

/// Register the type assertion debug system
pub fn register_type_assertion_debug() {
    // Only register once
    if DEBUG_REGISTERED.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
        println!("Registered enhanced type assertion debugging system");
        
        // Initialize from environment if specified
        if let Ok(debug_value) = env::var("CURSED_DEBUG") {
            if let Ok(level) = debug_value.parse::<u8>() {
                if level > 0 {
                    println!("Type assertion debugging enabled at level {}", level);
                }
            }
        }
    }
}

/// A trait that enhances the debugging capabilities for interface type assertions
pub trait InterfaceTypeAssertionDebug<'ctx> {
    /// Get the current debug config or create a default one
    fn get_debug_config(&self) -> TypeAssertionDebugConfig;
    
    /// Generate detailed debug information about a type assertion
    fn debug_type_assertion(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type: &str,
        source_location: Option<SourceLocation>,
    ) -> Result<String, Error>;
    
    /// Extract and debug the type ID from an interface value
    fn debug_type_id_extraction(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
    ) -> Result<String, Error>;
    
    /// Verify a type assertion with detailed error reporting
    fn verify_type_assertion_with_debug(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type: &str,
        source_location: Option<SourceLocation>,
    ) -> Result<BasicValueEnum<'ctx>, Error>;
    
    /// Trace the type hierarchy or interface inheritance chain
    fn trace_type_hierarchy(
        &self,
        type_name: &str,
    ) -> Result<String, Error>;
}

impl<'ctx> InterfaceTypeAssertionDebug<'ctx> for LlvmCodeGenerator<'ctx> {    
    /// Get the current debug config or create a default one
    fn get_debug_config(&self) -> TypeAssertionDebugConfig {
        self.type_assertion_debug_config.clone().unwrap_or_default()
    }
    fn debug_type_assertion(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type: &str,
        source_location: Option<SourceLocation>,
    ) -> Result<String, Error> {
        // First, get the runtime type information
        let (actual_type_id, actual_type_name) = self.get_runtime_type_id(interface_value, source_location.clone())?;
        
        // Get the target type ID and name
        let target_type_id = match &self.interface_type_registry {
            Some(registry) => registry.get_type_id(target_type).unwrap_or_else(|_| self.hash_type_name(target_type)),
            None => self.hash_type_name(target_type),
        };
        
        // Check compatibility
        let compatible = self.is_type_compatible(actual_type_id, target_type_id);
        
        // Build the debug report
        let mut debug_info = String::new();
        debug_info.push_str(&format!("Interface Type Assertion Debug Report\n"));
        debug_info.push_str(&format!("======================================\n\n"));
        
        // Basic assertion information
        debug_info.push_str(&format!("Assertion: {} is a {}\n", actual_type_name, target_type));
        debug_info.push_str(&format!("Result: {}\n\n", if compatible { "COMPATIBLE" } else { "INCOMPATIBLE" }));
        
        // Type IDs
        debug_info.push_str(&format!("Type Information:\n"));
        debug_info.push_str(&format!("- Actual type: {} (ID: 0x{:x})\n", actual_type_name, actual_type_id));
        debug_info.push_str(&format!("- Target type: {} (ID: 0x{:x})\n\n", target_type, target_type_id));
        
        // Location if available
        if let Some(loc) = &source_location {
            debug_info.push_str(&format!("Source Location:\n"));
            debug_info.push_str(&format!("- File: {}\n", loc.file.clone().unwrap_or_else(|| "<unknown>".to_string())));
            debug_info.push_str(&format!("- Line: {}\n", loc.line));
            debug_info.push_str(&format!("- Column: {}\n", loc.column));
            if !loc.source_line.is_empty() {
                debug_info.push_str(&format!("- Code: {}\n", loc.source_line));
            }
            debug_info.push_str("\n");
        }
        
        // Registry information if available
        if let Some(registry) = &self.interface_type_registry {
            debug_info.push_str(&format!("Registry Information:\n"));
            debug_info.push_str(&format!("- Registry initialized: {}\n", "Yes"));
            
            // Add visualization
            let visualization = self.visualize_type_path(&actual_type_name, target_type)?;
            debug_info.push_str(&format!("\nType Path Visualization:\n{}", visualization));
        } else {
            debug_info.push_str(&format!("Registry Information:\n"));
            debug_info.push_str(&format!("- Registry initialized: {}\n", "No"));
        }
        
        Ok(debug_info)
    }
    
    fn debug_type_id_extraction(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
    ) -> Result<String, Error> {
        // Extract and get type ID
        let type_id = self.get_interface_type_id(interface_value)?;
        
        // Get constant if possible
        let const_id = match type_id.as_int_value().get_zero_extended_constant() {
            Some(id) => id,
            None => return Ok(format!("Type ID Extraction Debug:\n- Type ID: <dynamic at runtime>\n")),
        };
        
        // Try to get type name from registry
        let type_name = if let Some(registry) = &self.interface_type_registry {
            match registry.get_type_name(const_id) {
                Ok(name) => name,
                Err(_) => format!("unknown(0x{:x})", const_id),
            }
        } else {
            format!("unknown(0x{:x})", const_id)
        };
        
        // Build debug info
        let mut debug_info = String::new();
        debug_info.push_str(&format!("Type ID Extraction Debug:\n"));
        debug_info.push_str(&format!("- Type ID: 0x{:x}\n", const_id));
        debug_info.push_str(&format!("- Type Name: {}\n", type_name));
        
        // Add vtable information if possible
        debug_info.push_str(&format!("- From: Interface Value Vtable (first field)\n"));
        
        Ok(debug_info)
    }
    
    fn verify_type_assertion_with_debug(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type: &str,
        source_location: Option<SourceLocation>,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        // Get configuration
        let config = self.get_debug_config();
        
        // Get type information
        let (actual_type_id, actual_type_name) = self.get_runtime_type_id(interface_value, source_location.clone())?;
        
        // Get target type ID
        let target_type_id = match &self.interface_type_registry {
            Some(registry) => registry.get_type_id(target_type).unwrap_or_else(|_| self.hash_type_name(target_type)),
            None => self.hash_type_name(target_type),
        };
        
        // Check compatibility
        let compatible = self.is_type_compatible(actual_type_id, target_type_id);
        
        // Print debug info for all assertions if enabled
        if config.print_all_assertions {
            let debug_info = self.debug_type_assertion(interface_value, target_type, source_location.clone())?;
            println!("Type Assertion Debug ({}): \n{}", 
                if compatible { "SUCCESS" } else { "FAILURE" }, 
                debug_info);
        }
        
        // Generate runtime debug if enabled
        if config.runtime_debug && self.module().get_function("printf").is_some() {
            // Generate runtime debugging code
            let _ = self.generate_type_assertion_debug_print(interface_value, target_type);
        }
        
        if compatible {
            // Return true for compatible types
            let true_value = self.context().bool_type().const_int(1, false);
            return Ok(true_value.into());
        }
        
        // Print failed assertions if enabled
        if config.print_failed_assertions && !config.print_all_assertions {
            let debug_info = self.debug_type_assertion(interface_value, target_type, source_location.clone())?;
            println!("Type Assertion Failed: \n{}", debug_info);
        }
        
        // Generate full debug information for incompatible types
        let mut debug_info = self.debug_type_assertion(interface_value, target_type, source_location.clone())?;
        
        // Add hierarchy information if enabled
        if config.include_hierarchy {
            if let Ok(hierarchy) = self.trace_type_hierarchy(&actual_type_name) {
                debug_info.push_str("\n\nType Hierarchy Information:\n");
                debug_info.push_str(&hierarchy);
            }
        }
        
        // Create detailed error with debug information
        let detailed_error = TypeAssertionError::new("interface", target_type)
            .with_message(format!("Type assertion failed with detailed debugging:\n\n{}", debug_info))
            .with_actual_type(actual_type_name, Some(actual_type_id))
            .with_target_type_id(target_type_id);
            
        if let Some(loc) = source_location {
            return Err(Error::TypeAssertion(
                detailed_error.with_location(loc)
            ));
        }
        
        // Return false for incompatible types
        let false_value = self.context().bool_type().const_int(0, false);
        Ok(false_value.into())
    }
    
    fn trace_type_hierarchy(
        &self,
        type_name: &str,
    ) -> Result<String, Error> {
        // Check if registry is available
        if self.interface_type_registry.is_none() {
            return Ok(format!("Type Hierarchy Trace for {}:\n- Registry not available\n", type_name));
        }
        
        let registry = self.interface_type_registry.as_ref().unwrap();
        
        // Get type ID
        let type_id = match registry.get_type_id(type_name) {
            Ok(id) => id,
            Err(_) => return Ok(format!("Type Hierarchy Trace for {}:\n- Type not found in registry\n", type_name)),
        };
        
        // Get inheritance map
        let inheritance_map = match registry.get_inheritance_map() {
            Some(map) => map,
            None => return Ok(format!("Type Hierarchy Trace for {}:\n- No inheritance information available\n", type_name)),
        };
        
        // Build hierarchy trace
        let mut result = format!("Type Hierarchy Trace for {}:\n", type_name);
        
        // Check for implemented interfaces (parent types)
        result.push_str("\nImplemented Interfaces:\n");
        let mut found_implementations = false;
        
        // Find all interfaces this type implements
        for (interface_id, implementers) in inheritance_map.iter() {
            if implementers.contains(&type_id) {
                let interface_name = registry.get_type_name(*interface_id)
                    .unwrap_or_else(|_| format!("unknown(0x{:x})", interface_id));
                result.push_str(&format!("- {}\n", interface_name));
                found_implementations = true;
            }
        }
        
        if !found_implementations {
            result.push_str("- None\n");
        }
        
        // Check for implementations (child types)
        result.push_str("\nImplementing Types:\n");
        let mut found_implementers = false;
        
        // Find all types that implement this interface
        if let Some(implementers) = inheritance_map.get(&type_id) {
            for implementer_id in implementers {
                let implementer_name = registry.get_type_name(*implementer_id)
                    .unwrap_or_else(|_| format!("unknown(0x{:x})", implementer_id));
                result.push_str(&format!("- {}\n", implementer_name));
                found_implementers = true;
            }
        }
        
        if !found_implementers {
            result.push_str("- None\n");
        }
        
        Ok(result)
    }
}

/// Update the type assertion debug configuration
pub fn update_type_assertion_debug_config(level: u8) -> TypeAssertionDebugConfig {
    let config = match level {
        0 => TypeAssertionDebugConfig {
            print_all_assertions: false,
            print_failed_assertions: false,
            include_hierarchy: false,
            include_path_visualization: false,
            runtime_debug: false,
        },
        1 => TypeAssertionDebugConfig {
            print_all_assertions: false,
            print_failed_assertions: true,
            include_hierarchy: false,
            include_path_visualization: true,
            runtime_debug: false,
        },
        _ => TypeAssertionDebugConfig {
            print_all_assertions: true,
            print_failed_assertions: true,
            include_hierarchy: true,
            include_path_visualization: true,
            runtime_debug: true,
        },
    };
    
    println!("Updated type assertion debug level to {}", level);
    config
}

// Additional debugging utilities for LLVM code generator
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Set the debug configuration for type assertions
    pub fn set_type_assertion_debug_config(&mut self, config: TypeAssertionDebugConfig) {
        self.type_assertion_debug_config = Some(config);
    }
    
    /// Set the debug level for type assertions
    pub fn set_type_assertion_debug_level(&mut self, level: u8) {
        let config = update_type_assertion_debug_config(level);
        self.set_type_assertion_debug_config(config);
    }

    /// Generate LLVM code to print type assertion debug information at runtime
    pub fn generate_type_assertion_debug_print(
        &mut self,
        interface_value: BasicValueEnum<'ctx>,
        target_type: &str,
    ) -> Result<(), Error> {
        // Get or create printf function declaration
        let printf_fn = self.get_or_create_printf()?;
        
        // Format string for debug output
        let format_str = "Type assertion debug: %s is%s a %s (IDs: 0x%lx, 0x%lx)\n\0";
        let format_global = self.create_string_global(format_str)?;
        
        // Extract type information
        let (actual_type_id, actual_type_name) = self.get_runtime_type_id(interface_value, None)?;
        let actual_type_global = self.create_string_global(&format!("{}{}", actual_type_name, "\0"))?;
        
        // Get target type ID
        let target_type_id = match &self.interface_type_registry {
            Some(registry) => registry.get_type_id(target_type).unwrap_or_else(|_| self.hash_type_name(target_type)),
            None => self.hash_type_name(target_type),
        };
        let target_type_global = self.create_string_global(&format!("{}{}", target_type, "\0"))?;
        
        // Check if compatible
        let compatible = self.is_type_compatible(actual_type_id, target_type_id);
        let compatible_str = if compatible { "" } else { " not" };
        let compatible_global = self.create_string_global(&format!("{}{}", compatible_str, "\0"))?;
        
        // Create type ID constants
        let actual_id_const = self.context().i64_type().const_int(actual_type_id, false);
        let target_id_const = self.context().i64_type().const_int(target_type_id, false);
        
        // Create printf call arguments
        let printf_args = vec![
            self.builder().build_pointer_cast(
                format_global,
                self.context().i8_type().ptr_type(inkwell::AddressSpace::default()),
                "format_ptr"
            ).unwrap().into(),
            self.builder().build_pointer_cast(
                actual_type_global,
                self.context().i8_type().ptr_type(inkwell::AddressSpace::default()),
                "actual_type_ptr"
            ).unwrap().into(),
            self.builder().build_pointer_cast(
                compatible_global,
                self.context().i8_type().ptr_type(inkwell::AddressSpace::default()),
                "compatible_ptr"
            ).unwrap().into(),
            self.builder().build_pointer_cast(
                target_type_global,
                self.context().i8_type().ptr_type(inkwell::AddressSpace::default()),
                "target_type_ptr"
            ).unwrap().into(),
            actual_id_const.into(),
            target_id_const.into(),
        ];
        
        // Call printf
        self.builder().build_call(printf_fn, &printf_args, "debug_printf")
            .map_err(|e| Error::Compilation(e.to_string()))?;
        
        Ok(())
    }
    
    // Helper function to create global string constant
    fn create_string_global(&self, content: &str) -> Result<inkwell::values::PointerValue<'ctx>, Error> {
        let string_type = self.context().i8_type().array_type(content.len() as u32);
        let global = self.module().add_global(string_type, None, "str_const");
        
        global.set_initializer(&self.context().const_string(content.as_bytes(), false));
        global.set_constant(true);
        global.set_linkage(inkwell::module::Linkage::Private);
        
        Ok(global.as_pointer_value())
    }
    
    // Get or create printf function declaration
    fn get_or_create_printf(&self) -> Result<inkwell::values::FunctionValue<'ctx>, Error> {
        let printf_type = self.context().i32_type().fn_type(
            &[self.context().i8_type().ptr_type(inkwell::AddressSpace::default()).into()],
            true
        );
        
        if let Some(printf_fn) = self.module().get_function("printf") {
            return Ok(printf_fn);
        }
        
        let printf_fn = self.module().add_function("printf", printf_type, None);
        Ok(printf_fn)
}

/// Enable improved debugging for interface type assertions with a specific level
/// - level 0: No debugging
/// - level 1: Basic debugging (failed assertions only)
/// - level 2: Full debugging (all assertions with visualization)
#[no_mangle]
pub extern "C" fn enable_interface_type_assertion_debugging(level: u8) {
    // Set environment variable for future instances
    std::env::set_var("CURSED_DEBUG", level.to_string());
    
    // Print confirmation message
    println!("Enabled interface type assertion debugging at level {}", level);
    
    // Register the debug system
    register_type_assertion_debug();
}
}