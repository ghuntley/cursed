//! Improved Field Accessors Implementation Module for LLVM code generation
//!
//! This module provides enhanced field accessor generation with proper LLVM error 
//! handling and propagation using the Result<T, Error> type.

use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::FunctionValue;
use crate::ast::declarations::{FunctionStatement, SquadStatement, CollabStatement, GenericConstraint};
use crate::ast::traits::Node;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::core::type_checker::Type;
use crate::error::Error;
use crate::core::generic_instantiation::GenericInstantiator;
use std::collections::{HashMap, HashSet};
use tracing::{debug, info, warn, error, span, Level};

/// Trait for improved field accessor generation with proper error handling
pub trait ImprovedFieldAccessors<'ctx> {
    /// Generate field accessor methods (getters and setters) for a specialized struct
    /// with proper error propagation and LLVM Result handling
    fn generate_improved_field_accessors(
        &mut self,
        generic_struct: &SquadStatement,
        specialized_name: &str,
        type_args: &[Type],
    ) -> Result<(), Error>;
}

impl<'ctx> ImprovedFieldAccessors<'ctx> for LlvmCodeGenerator<'ctx> {
    #[tracing::instrument(skip(self, generic_struct), fields(struct_name = %generic_struct.name.value, specialized_name = %specialized_name), level = "debug")]
    fn generate_improved_field_accessors(
        &mut self,
        generic_struct: &SquadStatement,
        specialized_name: &str,
        type_args: &[Type],
    ) -> Result<(), Error> {
        // Create a GenericInstantiator to handle type parameter substitution
        let mut instantiator = GenericInstantiator::new();
        
        // Set up type parameter mappings
        for (i, type_param) in generic_struct.type_parameters.iter().enumerate() {
            instantiator.add_type_param(&type_param.value, type_args[i].clone());
        }
        
        // Get the specialized struct type
        let struct_type = self.context()
            .get_struct_type(specialized_name)
            .ok_or_else(|| Error::codegen(format!("Struct type not found: {}", specialized_name)))?;
        
        // Create getter and setter methods for each field
        for (i, field) in generic_struct.fields.iter().enumerate() {
            let field_name = &field.name.value;
            let field_index = i as u32;
            
            // Extract the field's type
            let field_type_expr = &field.type_name;
            let generic_field_type = Type::Named(field_type_expr.string());
            let concrete_field_type = instantiator.instantiate_type(&generic_field_type)?;
            
            // Generate getter function name: {struct_name}_get_{field_name}
            let getter_name = format!("{}_get_{}", specialized_name, field_name);
            
            // Generate setter function name: {struct_name}_set_{field_name}
            let setter_name = format!("{}_set_{}", specialized_name, field_name);
            
            // Get LLVM types
            let field_llvm_type = self.type_to_llvm_basic(&concrete_field_type)?;
            let struct_ptr_type = struct_type.ptr_type(inkwell::AddressSpace::default());
            
            // Create getter function
            let getter_fn_type = field_llvm_type.fn_type(&[struct_ptr_type.into()], false);
            let getter_fn = self.module().add_function(&getter_name, getter_fn_type, None);
            
            // Create getter function body
            let getter_entry = self.context().append_basic_block(getter_fn, "entry");
            self.builder().position_at_end(getter_entry);
            
            // Get function parameter (struct pointer)
            let struct_ptr = getter_fn.get_nth_param(0)
                .ok_or_else(|| Error::codegen(format!("Failed to get function parameter for {}", getter_name)))?;
            
            // Create a span with field information for better tracing
            let _span = tracing::info_span!("field_accessor", field_name = %field_name, field_index = field_index, accessor_type = "getter").entered();
            
            // Build GEP instruction to get the field pointer
            let pointer_type = struct_type.ptr_type(inkwell::AddressSpace::default());
            let field_ptr = unsafe {
                self.builder()
                    .build_struct_gep(
                        pointer_type, 
                        struct_ptr.into_pointer_value(), 
                        field_index, 
                        &format!("field_ptr_{}", field_name)
                    )
                    .map_err(|e| Error::codegen(format!("Failed to build field GEP for field '{}': {}", field_name, e)))?
            };
            
            // Get the correct element type for this field
            let elem_type = struct_type
                .get_field_type_at_index(field_index)
                .ok_or_else(|| Error::codegen(format!("Cannot get field type at index {} for field '{}'", field_index, field_name)))?;

            // Load the field value
            let field_value = self.builder()
                .build_load(
                    elem_type, 
                    field_ptr, 
                    &format!("field_value_{}", field_name)
                )
                .map_err(|e| Error::codegen(format!("Failed to build load for field '{}': {}", field_name, e)))?;
            
            // Return the field value
            self.builder()
                .build_return(Some(&field_value))
                .map_err(|e| Error::codegen(format!("Failed to build return for getter '{}': {}", getter_name, e)))?;
            
            // Create setter function
            let setter_fn_type = self.context()
                .void_type()
                .fn_type(&[struct_ptr_type.into(), field_llvm_type.into()], false);
                
            let setter_fn = self.module().add_function(&setter_name, setter_fn_type, None);
            
            // Create setter function body
            let setter_entry = self.context().append_basic_block(setter_fn, "entry");
            self.builder().position_at_end(setter_entry);
            
            // Create a span for setter tracing
            let _span = tracing::info_span!("field_accessor", field_name = %field_name, field_index = field_index, accessor_type = "setter").entered();
            
            // Get function parameters
            let struct_ptr = setter_fn.get_nth_param(0)
                .ok_or_else(|| Error::codegen(format!("Failed to get struct pointer parameter for {}", setter_name)))?;
            let value = setter_fn.get_nth_param(1)
                .ok_or_else(|| Error::codegen(format!("Failed to get value parameter for {}", setter_name)))?;
            
            // Build GEP instruction to get the field pointer
            let pointer_type = struct_type.ptr_type(inkwell::AddressSpace::default());
            let field_ptr = unsafe {
                self.builder()
                    .build_struct_gep(
                        pointer_type, 
                        struct_ptr.into_pointer_value(), 
                        field_index, 
                        &format!("field_ptr_{}", field_name)
                    )
                    .map_err(|e| Error::codegen(format!("Failed to build field GEP for setter '{}': {}", setter_name, e)))?
            };
            
            // Store the new value
            self.builder()
                .build_store(field_ptr, value)
                .map_err(|e| Error::codegen(format!("Failed to build store for field '{}': {}", field_name, e)))?;
            
            // Return void
            self.builder()
                .build_return(None)
                .map_err(|e| Error::codegen(format!("Failed to build return for setter '{}': {}", setter_name, e)))?;
            
            debug!("Generated accessor methods for field '{}' in struct '{}'", field_name, specialized_name);
        }
        
        info!("Successfully generated all field accessors for struct '{}'!", specialized_name);
        
        Ok(())
    }
}

// Extension function to register the improved field accessors
pub fn register_improved_field_accessors() {
    info!("Registering improved field accessors module");
    // The trait is automatically available to LlvmCodeGenerator
}