//! Interface Field Accessors Integration Module
//!
//! This module integrates the improved field accessors with the interface
//! implementation and monomorphization systems to provide proper error
//! handling and propagation throughout the field accessor generation pipeline.

use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::FunctionValue;
use crate::ast::declarations::{FunctionStatement, SquadStatement, CollabStatement, GenericConstraint};
use crate::ast::traits::Node;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::improved_field_accessors::ImprovedFieldAccessors;
use crate::codegen::llvm::lru_field_accessors::LruCachedFieldAccessors;
use crate::codegen::llvm::integrated_monomorphization::IntegratedMonomorphization;
use crate::codegen::llvm::monomorphization::MonomorphizationManagerExtension;
use crate::core::type_checker::Type;
use crate::error::Error;
use crate::core::generic_instantiation::GenericInstantiator;
use crate::codegen::llvm::interface_type_assertion_common::MutableInterfaceRegistry;
use std::collections::{HashMap, HashSet};
use tracing::{debug, info, warn, error, span, Level};

/// Trait for integrating interface-specific field accessors with proper error handling
pub trait InterfaceFieldAccessors<'ctx> {
    /// Generate field accessor methods for struct fields that are accessed through interfaces
    /// with proper error propagation and Result handling
    fn generate_interface_field_accessors(
        &mut self,
        struct_stmt: &SquadStatement,
        interface_name: &str,
        type_args: &[Type],
    ) -> Result<(), Error>;
    
    /// Install field accessors for a specialized struct during monomorphization
    fn install_field_accessors_for_specialized_struct(
        &mut self,
        generic_struct: &SquadStatement,
        specialized_name: &str, 
        type_args: &[Type]
    ) -> Result<(), Error>;
}

impl<'ctx> InterfaceFieldAccessors<'ctx> for LlvmCodeGenerator<'ctx> {
    #[tracing::instrument(skip(self, struct_stmt), fields(struct_name = %struct_stmt.name.value, interface_name = %interface_name), level = "debug")]
    fn generate_interface_field_accessors(
        &mut self,
        struct_stmt: &SquadStatement,
        interface_name: &str,
        type_args: &[Type],
    ) -> Result<(), Error> {
        debug!("Generating interface field accessors for struct {} accessed through {}", 
            struct_stmt.name.value, interface_name);
        
        // Create a span for the entire operation
        let _span = tracing::info_span!("interface_field_accessors", 
            struct_name = %struct_stmt.name.value, 
            interface_name = %interface_name
        ).entered();
        
        // Generate the specialized struct name if type arguments are provided
        let specialized_name = if !type_args.is_empty() {
            let mut name = struct_stmt.name.value.clone();
            name.push('<');
            for (i, ty) in type_args.iter().enumerate() {
                if i > 0 {
                    name.push_str(", ");
                }
                name.push_str(&ty.to_string());
            }
            name.push('>');
            name
        } else {
            struct_stmt.name.value.clone()
        };
        
        // Use the LRU cached field accessors implementation for better performance
        self.generate_lru_cached_field_accessors(struct_stmt, &specialized_name, type_args)?;
        
        // Register the field accessors with the interface registry
        self.register_field_accessors_with_interface(
            &specialized_name, 
            interface_name
        )?;
        
        info!("Successfully generated interface field accessors for struct {} through interface {}", 
            specialized_name, interface_name);
        
        Ok(())
    }
    
    #[tracing::instrument(skip(self, generic_struct), fields(struct_name = %generic_struct.name.value, specialized_name = %specialized_name), level = "debug")]
    fn install_field_accessors_for_specialized_struct(
        &mut self,
        generic_struct: &SquadStatement,
        specialized_name: &str,
        type_args: &[Type]
    ) -> Result<(), Error> {
        debug!("Installing field accessors for specialized struct {}", specialized_name);
        
        // Create a span for this operation
        let _span = tracing::info_span!("install_field_accessors", 
            specialized_name = %specialized_name,
            struct_name = %generic_struct.name.value,
            type_args_count = type_args.len()
        ).entered();
        
        // Create a monomorphization key for this specialized struct
        let key = format!("field_accessors_{}", specialized_name);
        
        // Setup the monomorphization manager if not already initialized
        if self.monomorphization_manager.is_none() {
            debug!("Initializing monomorphization manager");
            let type_checker = std::sync::Arc::new(std::sync::RwLock::new(crate::core::type_checker::TypeChecker::new()));
            self.setup_monomorphization_manager(type_checker);
        }
        
        // Check if we've already generated accessors for this struct
        let is_specialized = {
            let manager = self.monomorphization_manager.as_ref().unwrap();
            manager.is_specialized(&key)
        };
        
        if is_specialized {
            debug!("Field accessors for {} already exist - skipping generation", specialized_name);
            return Ok(());
        }
        
        // Generate the field accessors using the LRU cached implementation
        let result = self.generate_lru_cached_field_accessors(generic_struct, specialized_name, type_args);
        
        // Register that we've generated accessors for this struct
        if let Some(manager) = self.monomorphization_manager.as_mut() {
            manager.register_specialized(&key);
        }
        
        // Propagate any errors that occurred during accessor generation
        result.map_err(|e| {
            error!("Failed to install field accessors for {}: {}", specialized_name, e);
            Error::codegen(format!(
                "Failed to install field accessors for specialized struct {}: {}",
                specialized_name, e
            ))
        })
    }
}

// Extension methods for LlvmCodeGenerator to support field accessor registration
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Initialize the interface manager if not already initialized
    fn initialize_interface_manager(&mut self) {
        if self.interface_manager.is_none() {
            self.interface_manager = Some(crate::codegen::llvm::dynamic_dispatch::InterfaceManager::new());
        }
    }
    /// Register field accessors with the interface registry to ensure proper interface
    /// method dispatch
    pub fn register_field_accessors_with_interface(
        &mut self,
        struct_name: &str,
        interface_name: &str
    ) -> Result<(), Error> {
        debug!("Registering field accessors for {} with interface {}", struct_name, interface_name);
        
        // Check if we have an interface manager initialized
        if self.interface_manager.is_none() {
            debug!("Interface manager not initialized, creating default instance");
            self.initialize_interface_manager();
        }
        
        // Get the interface manager
        let interface_manager = self.interface_manager.as_mut().unwrap();
        
        // Register the struct as implementing the interface
        interface_manager.register_implementation(struct_name, interface_name)
            .map_err(|e| Error::codegen(format!(
                "Failed to register field accessors with interface: {}", e
            )))?;
        
        debug!("Successfully registered field accessors for {} with interface {}", struct_name, interface_name);
        Ok(())
    }
}

// Register the module functionality
pub fn register_interface_field_accessors() {
    info!("Registering interface field accessors integration module");
    // The trait is automatically available to LlvmCodeGenerator
}