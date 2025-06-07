//! # Interface Type Registry
//!
//! This module provides runtime type information for interfaces and types,
//! enabling better debugging and error handling for type assertions.
//!
//! It maintains a registry of type IDs to type names, allowing runtime code
//! to look up type names from their IDs for more informative error messages.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use crate::core::interface_registry_extensions::{ThreadSafeInterfaceExtensionRegistry, InterfaceRegistryExtension};
use crate::core::interface_registry_visualization::InterfaceRegistryExtensionWithVisualization;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicTypeEnum, StructType};
use inkwell::values::{BasicValueEnum, PointerValue, IntValue, GlobalValue};
use inkwell::AddressSpace;

use tracing::{debug, error, info, instrument, span, trace, warn, Level};

use crate::error::Error;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::interface_registry::InterfaceTypeRegistry as InterfaceTypeRegistryTrait;
use crate::codegen::llvm::interface_path_finder_enhanced::InterfaceTypeRegistryExtensionChecking;
// This will be implemented in a future version when the runtime module is ready
// use crate::codegen::llvm::runtime::RuntimeFunction;

/// Interface type registry that maintains a mapping of type IDs to type names
/// for improved debugging and error reporting
#[derive(Debug)]
pub struct InterfaceTypeRegistry<'ctx> {
    /// Maps type IDs to corresponding type names
    type_id_to_name: HashMap<u64, String>,
    
    /// Global type name string array
    type_names_global: Option<GlobalValue<'ctx>>,
    
    /// Global type ID array
    type_ids_global: Option<GlobalValue<'ctx>>,
    
    /// Count of registered types
    type_count: usize,
    
    /// Next type ID to assign
    next_type_id: u64,
    
    /// Reference to the interface extension registry for checking relationships
    pub extension_registry: Option<std::sync::Arc<std::sync::RwLock<crate::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry>>>,
}

impl<'ctx> InterfaceTypeRegistry<'ctx> {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            type_id_to_name: HashMap::new(),
            type_names_global: None,
            type_ids_global: None,
            type_count: 0,
            next_type_id: 1, // Start with 1 so 0 can represent null or invalid
            extension_registry: None,
        }
    }
    
    /// Create a new registry with an extension registry
    pub fn with_extension_registry(extension_registry: std::sync::Arc<std::sync::RwLock<crate::core::interface_registry_extensions::ThreadSafeInterfaceExtensionRegistry>>) -> Self {
        Self {
            type_id_to_name: HashMap::new(),
            type_names_global: None,
            type_ids_global: None,
            type_count: 0,
            next_type_id: 1, // Start with 1 so 0 can represent null or invalid
            extension_registry: Some(extension_registry),
        }
    }
    
    /// Register a type with the registry
    pub fn register_type(&mut self, type_id: u64, type_name: String) {
        debug!("Registered type in registry: {} -> {}", type_id, type_name);
        self.type_id_to_name.insert(type_id, type_name);
        self.type_count = self.type_id_to_name.len();
    }
    
    /// Look up a type name by its ID
    pub fn get_type_name(&self, type_id: u64) -> Option<&String> {
        self.type_id_to_name.get(&type_id)
    }
    
    /// Get access to type_ids_global field
    pub fn type_ids_global(&self) -> Option<GlobalValue<'ctx>> {
        self.type_ids_global
    }
    
    /// Set type_ids_global field
    pub fn set_type_ids_global(&mut self, global: Option<GlobalValue<'ctx>>) {
        self.type_ids_global = global;
    }
    
    /// Get access to type_names_global field
    pub fn type_names_global(&self) -> Option<GlobalValue<'ctx>> {
        self.type_names_global
    }
    
    /// Set type_names_global field
    pub fn set_type_names_global(&mut self, global: Option<GlobalValue<'ctx>>) {
        self.type_names_global = global;
    }
    
    /// Get all registered types
    pub fn all_types(&self) -> Vec<(u64, String)> {
        self.type_id_to_name
            .iter()
            .map(|(&id, name)| (id, name.clone()))
            .collect()
    }
    
    /// Synchronize this registry with the extension registry
    /// 
    /// This method ensures that the type registry and extension registry
    /// are synchronized, making sure all types and extension relationships
    /// are properly reflected in both registries.
    #[instrument(skip(self), level = "debug")]
    pub fn synchronize_with_extension_registry(&mut self) -> Result<(), Error> {
        debug!("Synchronizing type registry with extension registry");
        
        if let Some(extension_registry) = &self.extension_registry {
            // Get the extension hierarchy from the registry
            let hierarchy = InterfaceRegistryExtension::get_extension_hierarchy(extension_registry).map_err(|e| {
                warn!("Error accessing extension registry: {}", e);
                Error::from_str("Error accessing interface registry data")
            })?;
            
            debug!("Retrieved extension hierarchy with {} interface relationships", hierarchy.len());
            
            // Ensure all interfaces are registered in the type registry
            let mut interface_names = HashSet::new();
            
            // Collect all interface names from hierarchy
            for (source, targets) in &hierarchy {
                interface_names.insert(source.clone());
                for target in targets {
                    interface_names.insert(target.clone());
                }
            }
            
            // Register all interfaces that aren't already registered
            for interface_name in interface_names {
                if !self.has_type(&interface_name) {
                    self.register_interface(&interface_name)?;
                }
            }
            
            // Now register all extension relationships
            for (source, targets) in &hierarchy {
                for target in targets {
                    self.register_interface_extension(source, target)?;
                }
            }
            
            debug!("Synchronization complete. Type registry now has {} types", self.type_count);
            
            Ok(())
        } else {
            warn!("No extension registry available for synchronization");
            Ok(())
        }
    }
    
    /// Get the count of registered types
    pub fn type_count(&self) -> usize {
        self.type_count
    }
}

/// Trait for accessing and manipulating the interface type registry
pub trait InterfaceTypeRegistryAccess<'ctx> {
    /// Get a reference to the interface type registry
    fn interface_type_registry(&self) -> &InterfaceTypeRegistry<'ctx>;
    
    /// Get a mutable reference to the interface type registry
    fn interface_type_registry_mut(&mut self) -> &mut InterfaceTypeRegistry<'ctx>;
    
    /// Register a type with the interface type registry
    fn register_type_in_registry(&mut self, type_id: u64, type_name: &str);
    
    /// Create global data structures for the type registry
    fn create_type_registry_globals(&mut self) -> Result<(), Error>;
    
    /// Generate code to look up a type name at runtime
    fn generate_type_name_lookup(
        &mut self,
        type_id: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error>;
}

impl<'ctx> InterfaceTypeRegistryAccess<'ctx> for LlvmCodeGenerator<'ctx> {
    fn interface_type_registry(&self) -> &InterfaceTypeRegistry<'ctx> {
        self.interface_type_registry.as_ref().expect("Interface type registry not initialized")
    }
    
    fn interface_type_registry_mut(&mut self) -> &mut InterfaceTypeRegistry<'ctx> {
        self.interface_type_registry.as_mut().expect("Interface type registry not initialized")
    }
    
    #[instrument(skip(self), level = "debug")]
    fn register_type_in_registry(&mut self, type_id: u64, type_name: &str) {
        debug!("Registering type in registry: {} -> {}", type_id, type_name);
        self.interface_type_registry_mut().register_type(type_id, type_name.to_string());
    }
    
    #[instrument(skip(self), level = "debug")]
    fn create_type_registry_globals(&mut self) -> Result<(), Error> {
        debug!("Creating globals for type registry");
        
        // Get all registered types
        let types = self.interface_type_registry().all_types();
        
        if types.is_empty() {
            // If no types are registered, don't create the globals
            debug!("No types registered, skipping global creation");
            return Ok(());
        }
        
        // Create the type ID array
        let id_type = self.context().i64_type();
        let id_array_type = id_type.array_type(types.len() as u32);
        
        let id_global = self.module().add_global(id_array_type, None, "cursed_type_ids");
        id_global.set_linkage(inkwell::module::Linkage::Internal);
        
        // Create ID array initializer with actual type IDs
        let id_values: Vec<_> = types.iter()
            .map(|(id, _)| id_type.const_int(*id, false))
            .collect();
        
        // Use individual values rather than attempting to create a const_array directly
        // since the API requires ArrayValue which we don't have
        let id_array_ptr = id_global.as_pointer_value();
        for (i, val) in id_values.iter().enumerate() {
            // Create entry block alloca for the value
            let val_ptr = unsafe {
                self.builder().build_in_bounds_gep(
                    id_array_type,
                    id_array_ptr,
                    &[self.context().i32_type().const_zero(), self.context().i32_type().const_int(i as u64, false)],
                    &format!("type_id_{}_ptr", i)
                ).map_err(|e| Error::codegen(format!("Failed to get GEP for type ID: {}", e)))?
            };
            // Store the value
            self.builder().build_store(val_ptr, *val)
                .map_err(|e| Error::codegen(format!("Failed to store type ID: {}", e)))?;
        }
        // Use zero initializer since we manually populate later
        id_global.set_initializer(&id_array_type.const_zero());
        
        // Create an array of string pointers for the type names
        let i8_ptr_type = self.context().i8_type().ptr_type(AddressSpace::default());
        let str_array_type = i8_ptr_type.array_type(types.len() as u32);
        
        // Create global string constants for each type name
        let mut string_globals = Vec::with_capacity(types.len());
        
        for (i, (_, name)) in types.iter().enumerate() {
            // Create a null-terminated string constant
            let name_with_null = format!("{}", name) + "\0";
            let str_type = self.context().i8_type().array_type(name_with_null.len() as u32);
            
            // Add the global string constant
            let str_global = self.module().add_global(
                str_type, 
                None, 
                &format!("type_name_{}", i)
            );
            str_global.set_linkage(inkwell::module::Linkage::Private);
            str_global.set_constant(true);
            
            // Set the initializer with the string content
            let str_val = self.context().const_string(name_with_null.as_bytes(), false);
            str_global.set_initializer(&str_val);
            
            // Get a pointer to the string and store it for later
            let str_ptr = self.builder().build_pointer_cast(
                str_global.as_pointer_value(),
                i8_ptr_type,
                &format!("type_name_{}_ptr", i)
            ).map_err(|e| Error::codegen(format!("Failed to cast type name pointer: {}", e)))?;
            
            string_globals.push(str_ptr);
        }
        
        // Create the array of string pointers
        let str_global = self.module().add_global(str_array_type, None, "cursed_type_names");
        str_global.set_linkage(inkwell::module::Linkage::Internal);
        
        // Create the initializer for the string pointer array
        // Initialize with zeroes first, then store values directly in a similar manner as the ID array
        str_global.set_initializer(&str_array_type.const_zero());
        
        // Get pointer to global array
        let str_array_ptr = str_global.as_pointer_value();
        
        // Store each string pointer in the array
        for (i, str_ptr) in string_globals.into_iter().enumerate() {
            // Calculate index for this element
            let index = self.context().i32_type().const_int(i as u64, false);
            
            // Get pointer to array element
            let elem_ptr = unsafe {
                self.builder().build_in_bounds_gep(
                    str_array_type,
                    str_array_ptr,
                    &[self.context().i32_type().const_zero(), index],
                    &format!("str_ptr_{}_ptr", i)
                ).map_err(|e| Error::codegen(format!("Failed to get pointer to string ptr: {}", e)))?
            };
            
            // Store the string pointer
            self.builder().build_store(elem_ptr, str_ptr)
                .map_err(|e| Error::codegen(format!("Failed to store string pointer: {}", e)))?;
        }
        
        // Store the globals in the registry
        self.interface_type_registry_mut().type_ids_global = Some(id_global);
        self.interface_type_registry_mut().type_names_global = Some(str_global);
        
        debug!("Created type registry globals with {} types", types.len());
        Ok(())
    }
    
    #[instrument(skip(self, type_id), level = "debug")]
    fn generate_type_name_lookup(
        &mut self,
        type_id: BasicValueEnum<'ctx>
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        debug!("Generating type name lookup for type ID: {:?}", type_id);
        
        // Ensure we've created the type registry globals
        // If they don't exist yet, create them
        if self.interface_type_registry().type_ids_global.is_none() {
            self.create_type_registry_globals()?;
        }
        
        // Get the globals from the registry
        let type_ids_global = match self.interface_type_registry().type_ids_global {
            Some(global) => global,
            None => {
                // If we still don't have the globals, fall back to a placeholder
                let unknown_type_str = self.context().const_string("Unknown Type".as_bytes(), true);
                let unknown_global = self.module().add_global(
                    unknown_type_str.get_type(), 
                    None, 
                    "str_unknown_type"
                );
                unknown_global.set_linkage(inkwell::module::Linkage::Private);
                unknown_global.set_initializer(&unknown_type_str);
                
                return Ok(unknown_global.as_pointer_value().into());
            }
        };
        
        let type_names_global = match self.interface_type_registry().type_names_global {
            Some(global) => global,
            None => {
                // If we don't have the names global, fall back to a placeholder
                debug!("No type names global found, using placeholder");
                let unknown_type_str = self.context().const_string("Unknown Type".as_bytes(), true);
                let unknown_global = self.module().add_global(
                    unknown_type_str.get_type(), 
                    None, 
                    "str_unknown_type"
                );
                unknown_global.set_linkage(inkwell::module::Linkage::Private);
                unknown_global.set_initializer(&unknown_type_str);
                
                return Ok(unknown_global.as_pointer_value().into());
            }
        };
        
        // Get the number of types in the registry
        let type_count = self.interface_type_registry().type_count();
        if type_count == 0 {
            // If there are no types, return the unknown type
            let unknown_type_str = self.context().const_string("Unknown Type".as_bytes(), true);
            let unknown_global = self.module().add_global(
                unknown_type_str.get_type(), 
                None, 
                "str_unknown_type"
            );
            unknown_global.set_linkage(inkwell::module::Linkage::Private);
            unknown_global.set_initializer(&unknown_type_str);
            
            return Ok(unknown_global.as_pointer_value().into());
        }
        
        // Get the current function for creating basic blocks
        let current_fn = self.current_function()
            .ok_or_else(|| Error::codegen("No current function for type name lookup"))?;
        
        // Create basic blocks for the lookup loop
        let start_block = self.context().append_basic_block(current_fn, "type_lookup_start");
        let loop_block = self.context().append_basic_block(current_fn, "type_lookup_loop");
        let found_block = self.context().append_basic_block(current_fn, "type_lookup_found");
        let not_found_block = self.context().append_basic_block(current_fn, "type_lookup_not_found");
        let continue_block = self.context().append_basic_block(current_fn, "type_lookup_continue");
        
        // Branch to the start of the lookup
        self.builder().build_unconditional_branch(start_block)
            .map_err(|e| Error::codegen(format!("Failed to branch to lookup start: {}", e)))?;
        
        // Start block - initialize loop variables
        self.builder().position_at_end(start_block);
        
        // Cast the globals to pointer types for easier GEP operations
        let ids_ptr = type_ids_global.as_pointer_value();
        let names_ptr = type_names_global.as_pointer_value();
        
        // Create a loop index variable
        let i32_type = self.context().i32_type();
        let i64_type = self.context().i64_type();
        let index_ptr = self.create_entry_block_alloca(i32_type, "type_lookup_index");
        
        // Initialize index to 0
        self.builder().build_store(index_ptr, i32_type.const_int(0, false))
            .map_err(|e| Error::codegen(format!("Failed to initialize index: {}", e)))?;
        
        // Branch to the loop
        self.builder().build_unconditional_branch(loop_block)
            .map_err(|e| Error::codegen(format!("Failed to branch to loop: {}", e)))?;
        
        // Loop block - check if we've reached the end of the array
        self.builder().position_at_end(loop_block);
        
        // Load the current index
        let index = self.builder().build_load(i32_type, index_ptr, "index")
            .map_err(|e| Error::codegen(format!("Failed to load index: {}", e)))?;
        
        // Check if we're at the end of the array
        let end_check = self.builder().build_int_compare(
            inkwell::IntPredicate::UGE,
            index.into_int_value(),
            i32_type.const_int(type_count as u64, false),
            "end_check"
        ).map_err(|e| Error::codegen(format!("Failed to check end condition: {}", e)))?;
        
        // If we've reached the end, go to not found block
        self.builder().build_conditional_branch(end_check, not_found_block, loop_block)
            .map_err(|e| Error::codegen(format!("Failed to branch on end check: {}", e)))?;
        
        // Get pointer to current ID in the array
        let id_ptr = unsafe {
            self.builder().build_in_bounds_gep(
                i64_type.array_type(type_count as u32),
                ids_ptr,
                &[i32_type.const_int(0, false), index.into_int_value()],
                "id_ptr"
            ).map_err(|e| Error::codegen(format!("Failed to get ID pointer: {}", e)))?
        };
        
        // Load the ID
        let current_id = self.builder().build_load(i64_type, id_ptr, "current_id")
            .map_err(|e| Error::codegen(format!("Failed to load ID: {}", e)))?;
        
        // Compare with the target ID
        let id_match = self.builder().build_int_compare(
            inkwell::IntPredicate::EQ,
            current_id.into_int_value(),
            type_id.into_int_value(),
            "id_match"
        ).map_err(|e| Error::codegen(format!("Failed to compare IDs: {}", e)))?;
        
        // If they match, go to found block, otherwise increment index and continue
        self.builder().build_conditional_branch(id_match, found_block, loop_block)
            .map_err(|e| Error::codegen(format!("Failed to branch on ID match: {}", e)))?;
        
        // Found block - get the corresponding type name
        self.builder().position_at_end(found_block);
        
        // Get pointer to the type name
        let i8_ptr_type = self.context().i8_type().ptr_type(AddressSpace::default());
        let name_ptr_ptr = unsafe {
            self.builder().build_in_bounds_gep(
                i8_ptr_type.array_type(type_count as u32),
                names_ptr,
                &[i32_type.const_int(0, false), index.into_int_value()],
                "name_ptr_ptr"
            ).map_err(|e| Error::codegen(format!("Failed to get name pointer: {}", e)))?
        };
        
        // Load the name pointer
        let name_ptr = self.builder().build_load(i8_ptr_type, name_ptr_ptr, "name_ptr")
            .map_err(|e| Error::codegen(format!("Failed to load name pointer: {}", e)))?;
        
        // Branch to continue block
        self.builder().build_unconditional_branch(continue_block)
            .map_err(|e| Error::codegen(format!("Failed to branch to continue: {}", e)))?;
        
        // Not found block - return "Unknown Type"
        self.builder().position_at_end(not_found_block);
        
        // Create an "Unknown Type" string
        let unknown_type_str = self.context().const_string("Unknown Type".as_bytes(), true);
        let unknown_global = self.module().add_global(
            unknown_type_str.get_type(), 
            None, 
            "str_unknown_type"
        );
        unknown_global.set_linkage(inkwell::module::Linkage::Private);
        unknown_global.set_initializer(&unknown_type_str);
        
        let unknown_ptr = unknown_global.as_pointer_value();
        
        // Branch to continue block
        self.builder().build_unconditional_branch(continue_block)
            .map_err(|e| Error::codegen(format!("Failed to branch from not found: {}", e)))?;
        
        // Continue block - use phi node to select the appropriate string
        self.builder().position_at_end(continue_block);
        
        let phi = self.builder().build_phi(
            i8_ptr_type,
            "type_name_result"
        ).map_err(|e| Error::codegen(format!("Failed to build phi node: {}", e)))?;
        
        phi.add_incoming(&[(
            &name_ptr.into_pointer_value(),
            found_block
        ), (
            &unknown_ptr,
            not_found_block
        )]);
        
        // Return the result
        Ok(phi.as_basic_value())
    }
}

/// Initialize LlvmCodeGenerator with the interface type registry
pub fn add_interface_type_registry_to<'ctx>() {
    debug!("Interface type registry loaded");
    // This function is called when the module is loaded, but doesn't need to do anything
    // since the LlvmCodeGenerator already initializes the registry in its new() method
}

/// Extension methods for type registration and management
impl<'ctx> InterfaceTypeRegistry<'ctx> {
    /// Register an interface with the registry and return its type ID
    #[instrument(skip(self), level = "debug")]
    pub fn register_interface(&mut self, interface_name: &str) -> Result<u64, Error> {
        debug!("Registering interface: {}", interface_name);
        
        // Check if the interface is already registered
        let type_id = self.hash_type_name(interface_name);
        if self.type_id_to_name.contains_key(&type_id) {
            debug!("Interface {} already registered with ID {}", interface_name, type_id);
            return Ok(type_id);
        }
        
        // Register the type
        self.register_type(type_id, interface_name.to_string());
        self.type_count += 1;
        
        debug!("Registered new interface {} with ID {}", interface_name, type_id);
        Ok(type_id)
    }
    
    /// Register that one interface extends another
    #[instrument(skip(self), level = "debug")]
    pub fn register_interface_extension(&mut self, source: &str, target: &str) -> Result<(), Error> {
        debug!("Registering that {} extends {}", source, target);
        
        // Make sure both interfaces are registered
        let source_id = self.register_interface(source)?;
        let target_id = self.register_interface(target)?;
        
        // If we have an extension registry, register the extension there too for consistency
        if let Some(registry) = &self.extension_registry {
            // Register the extension in the external registry
            let mut reg = registry.write().map_err(|_| Error::Compilation("Failed to acquire write lock".to_string()))?;
            reg.register_extension(source, target).map_err(|e| {
                warn!("Error registering extension in external registry: {}", e);
                Error::from_str("Error updating extension registry")
            })?;
        }
        
        debug!("Registered extension: {} (ID: {}) extends {} (ID: {})", 
               source, source_id, target, target_id);
               
        Ok(())
    }
    
    /// Check if the registry contains a type with the given name
    pub fn has_type(&self, type_name: &str) -> bool {
        let type_id = self.hash_type_name(type_name);
        self.type_id_to_name.contains_key(&type_id)
    }
    
    /// Get the type ID for a given type name
    pub fn get_type_id(&self, type_name: &str) -> Result<u64, Error> {
        let type_id = self.hash_type_name(type_name);
        if self.type_id_to_name.contains_key(&type_id) {
            Ok(type_id)
        } else {
            Err(Error::from_str(&format!("Type {} not found in registry", type_name)))
        }
    }
    
    /// Generate a hash for the type name
    pub fn hash_type_name(&self, type_name: &str) -> u64 {
        // FNV-1a hash algorithm for more consistent hashing
        let mut hash: u64 = 0xcbf29ce484222325;
        for byte in type_name.bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash
    }

    /// Check if source interface extends target interface (alias for extends)
    pub fn check_interface_extends(&self, source: &str, target: &str) -> Result<bool, Error> {
        if let Some(registry) = &self.extension_registry {
            InterfaceRegistryExtension::extends(registry, source, target)
        } else {
            Ok(false)
        }
    }
}

/// Implementation of the InterfaceTypeRegistry trait for the concrete struct
impl<'ctx> InterfaceTypeRegistryTrait for InterfaceTypeRegistry<'ctx> {
    fn register_interface(&mut self, name: &str) -> Result<(), Error> {
        // Register as a type with a generated ID
        let type_id = self.next_type_id;
        self.next_type_id += 1;
        self.register_type(type_id, name.to_string());
        Ok(())
    }
    
    fn register_extension(&mut self, source: &str, target: &str) -> Result<(), Error> {
        // This would be handled by the extension registry if available
        if let Some(registry) = &self.extension_registry {
            let mut reg = registry.write().map_err(|_| Error::Compilation("Failed to acquire write lock".to_string()))?;
            reg.register_extension(source, target)?;
        }
        Ok(())
    }
    
    fn extends(&self, source: &str, target: &str) -> Result<bool, Error> {
        if let Some(registry) = &self.extension_registry {
            InterfaceRegistryExtension::extends(registry, source, target)
        } else {
            Ok(false)
        }
    }
    
    fn find_path(&self, source: &str, target: &str) -> Result<Option<Vec<String>>, Error> {
        if let Some(registry) = &self.extension_registry {
            let reg = registry.read().map_err(|_| Error::Compilation("Failed to acquire read lock".to_string()))?;
            // Use find_longest_path as a fallback since find_path is not in the trait
            reg.find_longest_path(source, target)
        } else {
            Ok(None)
        }
    }
    
    fn get_all_interfaces(&self) -> Result<HashSet<String>, Error> {
        let mut interfaces = HashSet::new();
        for (_, name) in &self.type_id_to_name {
            interfaces.insert(name.clone());
        }
        Ok(interfaces)
    }
    
    fn interface_exists(&self, name: &str) -> Result<bool, Error> {
        Ok(self.type_id_to_name.values().any(|n| n == name))
    }
    
    fn get_type_name(&self, type_id: u64) -> Result<String, Error> {
        self.type_id_to_name.get(&type_id)
            .cloned()
            .ok_or_else(|| Error::Compilation(format!("Type name not found for ID {}", type_id)))
    }
    
    fn lookup_type_id(&self, type_name: &str) -> Result<u64, Error> {
        for (id, name) in &self.type_id_to_name {
            if name == type_name {
                return Ok(*id);
            }
        }
        Err(Error::Compilation(format!("Type ID not found for name {}", type_name)))
    }
    
    fn get_inheritance_map(&self) -> Option<HashMap<String, HashSet<String>>> {
        if let Some(registry) = &self.extension_registry {
            if let Ok(reg) = registry.read() {
                // Convert from HashMap<String, Vec<String>> to HashMap<String, HashSet<String>>
                if let Ok(hierarchy) = reg.get_extension_hierarchy() {
                    let mut inheritance_map = HashMap::new();
                    for (key, values) in hierarchy {
                        let hash_set: HashSet<String> = values.into_iter().collect();
                        inheritance_map.insert(key, hash_set);
                    }
                    return Some(inheritance_map);
                }
            }
        }
        None
    }
    
    fn all_types(&self) -> Vec<(u64, String)> {
        self.type_id_to_name
            .iter()
            .map(|(id, name)| (*id, name.clone()))
            .collect()
    }
    
    fn is_interface(&self, type_id: u32) -> Result<bool, Error> {
        // For now, assume all registered types could be interfaces
        Ok(self.type_id_to_name.contains_key(&(type_id as u64)))
    }
}