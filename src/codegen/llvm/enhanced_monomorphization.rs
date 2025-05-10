//! Enhanced monomorphization module for LLVM code generation
//!
//! This module provides improved type parameter substitution, constraint checking,
//! and generic struct implementations.

use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::FunctionValue;
use crate::ast::declarations::{FunctionStatement, SquadStatement, CollabStatement, GenericConstraint};
use crate::ast::traits::Node;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::core::type_checker::Type;
use crate::error::Error;
use crate::core::generic_instantiation::GenericInstantiator;
use std::collections::{HashMap, HashSet};

/// Trait for enhanced monomorphization capabilities
pub trait EnhancedMonomorphization<'ctx> {
    /// Check if a type satisfies a constraint
    fn check_constraint(
        &self,
        concrete_type: &Type,
        interface_name: &str,
    ) -> Result<bool, Error>;
    
    /// Check if a set of concrete types satisfy the constraints of a generic function
    fn validate_constraints(
        &self,
        generic_function: &FunctionStatement,
        type_args: &[Type],
    ) -> Result<(), Error>;
    
    /// Generate a specialized struct with proper field types
    fn generate_specialized_struct(
        &mut self,
        generic_struct: &SquadStatement,
        specialized_name: &str,
        type_args: &[Type],
    ) -> Result<(), Error>;
    
    /// Generate accessor methods for a generic struct
    fn generate_field_accessors(
        &mut self,
        generic_struct: &SquadStatement,
        specialized_name: &str,
        type_args: &[Type],
    ) -> Result<(), Error>;
}

impl<'ctx> EnhancedMonomorphization<'ctx> for LlvmCodeGenerator<'ctx> {
    fn check_constraint(
        &self,
        concrete_type: &Type,
        interface_name: &str,
    ) -> Result<bool, Error> {
        // Use the interface registry for consistency
        // This ensures that constraint checking behaves the same way in all parts of the system
        tracing::debug!("Enhanced monomorphization using interface registry");
        
        // Get the interface registry
        let registry = crate::core::interface_registry::InterfaceRegistry::new_with_defaults();
        
        // Check the constraint directly with the registry
        let result = registry.check_implementation(concrete_type, interface_name);
        
        // Log the result for debugging
        match &result {
            Ok(true) => tracing::debug!(concrete_type = ?concrete_type, interface = interface_name, "Type implements interface"),
            Ok(false) => tracing::warn!(concrete_type = ?concrete_type, interface = interface_name, "Type does not implement interface"),
            Err(e) => tracing::error!(concrete_type = ?concrete_type, interface = interface_name, error = ?e, "Error checking interface implementation"),
        }
        
        // Return the result or convert Ok(false) to an Err for consistency with other constraint checkers
        match result {
            Ok(true) => Ok(true),
            Ok(false) => Err(Error::from_str(&format!(
                "Type '{:?}' does not implement interface '{}': implementation not found in registry",
                concrete_type, interface_name
            ))),
            Err(e) => Err(e),
        }
    }
    
    fn validate_constraints(
        &self,
        generic_function: &FunctionStatement,
        type_args: &[Type],
    ) -> Result<(), Error> {
        // Check if we have the right number of type arguments
        if generic_function.type_parameters.len() != type_args.len() {
            return Err(Error::from_str(&format!(
                "Wrong number of type arguments for {}: expected {}, got {}",
                generic_function.name.value,
                generic_function.type_parameters.len(),
                type_args.len()
            )));
        }
        
        // Create a map of type parameter names to concrete types
        let mut type_map = HashMap::new();
        for (i, param) in generic_function.type_parameters.iter().enumerate() {
            type_map.insert(param.value.clone(), type_args[i].clone());
        }
        
        // Check each constraint
        for constraint in &generic_function.generic_constraints {
            let param_name = &constraint.type_parameter.value;
            let interface_name = &constraint.trait_name.value;
            
            // Get the concrete type for this parameter
            if let Some(concrete_type) = type_map.get(param_name) {
                // Check if the concrete type satisfies the constraint
                self.check_constraint(concrete_type, interface_name)?;
            } else {
                return Err(Error::from_str(&format!(
                    "Unknown type parameter: {}",
                    param_name
                )));
            }
        }
        
        Ok(())
    }
    
    fn generate_specialized_struct(
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
        
        // Define the LLVM struct type
        let struct_type = self.context().opaque_struct_type(specialized_name);
        
        // Process each field to determine its concrete type
        let mut field_types = Vec::new();
        for field in &generic_struct.fields {
            // Get the field's type expression
            let field_type_expr = &field.type_name;
            
            // Extract the type from the expression
            let generic_field_type = Type::Named(field_type_expr.string());
            
            // Apply type parameter substitution to get the concrete field type
            let concrete_field_type = instantiator.instantiate_type(&generic_field_type)?;
            
            // Convert the concrete type to an LLVM type
            let llvm_field_type = self.type_to_llvm_basic(&concrete_field_type)?;
            
            // Add to the list of field types
            field_types.push(llvm_field_type);
        }
        
        // Set the struct body with the field types
        struct_type.set_body(&field_types, false);
        
        // Register the struct type in the module
        self.register_struct_type(specialized_name, struct_type);
        
        Ok(())
    }
    
    fn generate_field_accessors(
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
        let struct_type = self.lookup_specialized_struct_type(specialized_name)
            .ok_or_else(|| Error::from_str(&format!("Struct type not found: {}", specialized_name)))?;
        
        // Create getter and setter methods for each field
        for (i, field) in generic_struct.fields.iter().enumerate() {
            let field_name = &field.name.value;
            
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
                .ok_or_else(|| Error::from_str("Failed to get function parameter"))?;
            
            // Build GEP instruction to get the field pointer
            let pointer_type = struct_type.ptr_type(inkwell::AddressSpace::default());
            let field_ptr = unsafe {
                self.builder().build_struct_gep(pointer_type, struct_ptr.into_pointer_value(), i as u32, "field_ptr")
                    .map_err(|e| Error::from_str(&format!("Failed to build GEP: {}", e)))?
            };
            
            // Create a type reference for the element
            let elem_type = struct_type.get_field_type_at_index(i as u32).unwrap();

            // Load the field value
            let field_value = self.builder().build_load(elem_type, field_ptr, "field_value")
                .map_err(|e| Error::from_str(&format!("Failed to build load: {}", e)))?;
            
            // Return the field value
            self.builder().build_return(Some(&field_value))
                .map_err(|e| Error::from_str(&format!("Failed to build return: {}", e)))?;
            
            // Create setter function
            let setter_fn_type = self.context().void_type().fn_type(
                &[struct_ptr_type.into(), field_llvm_type.into()],
                false,
            );
            let setter_fn = self.module().add_function(&setter_name, setter_fn_type, None);
            
            // Create setter function body
            let setter_entry = self.context().append_basic_block(setter_fn, "entry");
            self.builder().position_at_end(setter_entry);
            
            // Get function parameters
            let struct_ptr = setter_fn.get_nth_param(0)
                .ok_or_else(|| Error::from_str("Failed to get struct pointer parameter"))?;
            let value = setter_fn.get_nth_param(1)
                .ok_or_else(|| Error::from_str("Failed to get value parameter"))?;
            
            // Build GEP instruction to get the field pointer
            let pointer_type = struct_type.ptr_type(inkwell::AddressSpace::default());
            let field_ptr = unsafe {
                self.builder().build_struct_gep(pointer_type, struct_ptr.into_pointer_value(), i as u32, "field_ptr")
                    .map_err(|e| Error::from_str(&format!("Failed to build GEP: {}", e)))?
            };
            
            // Store the new value
            self.builder().build_store(field_ptr, value)
                .map_err(|e| Error::from_str(&format!("Failed to build store: {}", e)))?;
            
            // Return void
            self.builder().build_return(None)
                .map_err(|e| Error::from_str(&format!("Failed to build return: {}", e)))?;
        }
        
        Ok(())
    }
}

// Extension methods for the LlvmCodeGenerator
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Register a struct type for later lookup
    fn register_struct_type(&mut self, name: &str, struct_type: inkwell::types::StructType<'ctx>) {
        // In a real implementation, this would store the struct type in a map for later lookup
        // For now, this is just a placeholder
    }
    
    /// Look up a previously defined struct type
    fn lookup_specialized_struct_type(&self, name: &str) -> Option<inkwell::types::StructType<'ctx>> {
        // In a real implementation, this would retrieve the struct type from a map
        // For now, just get it from the module
        self.context().get_struct_type(name)
    }
    
    /// Convert a Type to an LLVM BasicTypeEnum
    pub fn type_to_llvm_basic(&self, typ: &Type) -> Result<BasicTypeEnum<'ctx>, Error> {
        match typ {
            Type::Normie => Ok(self.context().i32_type().into()),
            Type::Smol => Ok(self.context().i8_type().into()),
            Type::Mid => Ok(self.context().i16_type().into()),
            Type::Thicc => Ok(self.context().i64_type().into()),
            Type::Snack => Ok(self.context().f32_type().into()),
            Type::Meal => Ok(self.context().f64_type().into()),
            Type::Lit => Ok(self.context().bool_type().into()),
            Type::Tea => {
                // String is represented as a pointer to an array of characters
                let char_type = self.context().i8_type();
                Ok(char_type.ptr_type(inkwell::AddressSpace::default()).into())
            },
            Type::Byte => Ok(self.context().i8_type().into()),
            Type::Rune | Type::Sip => Ok(self.context().i32_type().into()),
            Type::Struct(name, _) => {
                // Look up the struct type in the module
                if let Some(struct_type) = self.context().get_struct_type(name) {
                    Ok(struct_type.into())
                } else {
                    Err(Error::from_str(&format!("Unknown struct type: {}", name)))
                }
            },
            Type::Array(elem_type, size) => {
                // Convert the element type to LLVM type
                let elem_llvm_type = self.type_to_llvm_basic(elem_type)?;
                
                // Create an array type
                match elem_llvm_type {
                    BasicTypeEnum::IntType(int_type) => {
                        Ok(int_type.array_type(*size as u32).into())
                    },
                    BasicTypeEnum::FloatType(float_type) => {
                        Ok(float_type.array_type(*size as u32).into())
                    },
                    BasicTypeEnum::PointerType(ptr_type) => {
                        Ok(ptr_type.array_type(*size as u32).into())
                    },
                    BasicTypeEnum::StructType(struct_type) => {
                        Ok(struct_type.array_type(*size as u32).into())
                    },
                    BasicTypeEnum::ArrayType(array_type) => {
                        // This is a nested array, which might not be directly representable
                        // For simplicity, we'll make it a pointer to the element array type
                        Ok(array_type.ptr_type(inkwell::AddressSpace::default()).into())
                    },
                    BasicTypeEnum::VectorType(vector_type) => {
                        Ok(vector_type.array_type(*size as u32).into())
                    },
                }
            },
            Type::Slice(elem_type) => {
                // A slice is represented as a struct with a pointer to the data and a length
                let elem_llvm_type = self.type_to_llvm_basic(elem_type)?;
                let ptr_type = match elem_llvm_type {
                    BasicTypeEnum::IntType(t) => t.ptr_type(inkwell::AddressSpace::default()),
                    BasicTypeEnum::FloatType(t) => t.ptr_type(inkwell::AddressSpace::default()),
                    BasicTypeEnum::PointerType(t) => t.ptr_type(inkwell::AddressSpace::default()),
                    BasicTypeEnum::StructType(t) => t.ptr_type(inkwell::AddressSpace::default()),
                    BasicTypeEnum::ArrayType(t) => t.ptr_type(inkwell::AddressSpace::default()),
                    BasicTypeEnum::VectorType(t) => t.ptr_type(inkwell::AddressSpace::default()),
                };
                
                // Create a struct type for the slice
                let slice_struct_type = self.context().struct_type(
                    &[ptr_type.into(), self.context().i64_type().into()],
                    false,
                );
                
                Ok(slice_struct_type.into())
            },
            Type::Pointer(target_type) => {
                // Convert the target type to LLVM type
                let target_llvm_type = self.type_to_llvm_basic(target_type)?;
                
                // Create a pointer type
                match target_llvm_type {
                    BasicTypeEnum::IntType(t) => Ok(t.ptr_type(inkwell::AddressSpace::default()).into()),
                    BasicTypeEnum::FloatType(t) => Ok(t.ptr_type(inkwell::AddressSpace::default()).into()),
                    BasicTypeEnum::PointerType(t) => Ok(t.ptr_type(inkwell::AddressSpace::default()).into()),
                    BasicTypeEnum::StructType(t) => Ok(t.ptr_type(inkwell::AddressSpace::default()).into()),
                    BasicTypeEnum::ArrayType(t) => Ok(t.ptr_type(inkwell::AddressSpace::default()).into()),
                    BasicTypeEnum::VectorType(t) => Ok(t.ptr_type(inkwell::AddressSpace::default()).into()),
                }
            },
            _ => Err(Error::from_str(&format!("Unsupported type: {:?}", typ))),
        }
    }
}