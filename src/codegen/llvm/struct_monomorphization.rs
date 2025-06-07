//! Struct monomorphization for LLVM code generation
//!
//! This module handles the specialization of generic struct types in LLVM code generation.
//! It creates concrete implementations of generic structs with specific type parameters.

use inkwell::types::StructType;
use crate::ast::declarations::SquadStatement;
use crate::core::type_checker::Type;
use crate::error::Error;
use super::context::LlvmCodeGenerator;

/// Trait for struct monomorphization functionality
pub trait StructMonomorphization<'ctx> {
    /// Generate a specialized struct type with concrete type arguments
    fn generate_specialized_struct(
        &mut self,
        generic_struct: &SquadStatement,
        specialized_name: &str,
        type_args: &[Type],
    ) -> Result<StructType<'ctx>, Error>;
    
    /// Register GC metadata for a specialized struct type
    fn register_struct_gc_metadata(
        &mut self,
        type_name: &str,
        traceable_fields: Vec<(usize, String)>,
    ) -> Result<(), Error>;
}

impl<'ctx> StructMonomorphization<'ctx> for LlvmCodeGenerator<'ctx> {
    fn generate_specialized_struct(
        &mut self,
        generic_struct: &SquadStatement,
        specialized_name: &str,
        type_args: &[Type],
    ) -> Result<StructType<'ctx>, Error> {
        // Create an opaque struct type initially
        let struct_type = self.context().opaque_struct_type(specialized_name);
        
        // Build a map between type parameter names and concrete types
        let mut type_parameter_map = std::collections::HashMap::new();
        
        // Ensure we have the right number of type arguments
        if generic_struct.type_parameters.len() != type_args.len() {
            return Err(Error::new(
                "GNRC-001",
                format!(
                    "Type argument count mismatch for {}: expected {}, got {}",
                    generic_struct.name.value,
                    generic_struct.type_parameters.len(),
                    type_args.len()
                ),
                None,
            ));
        }
        
        // Map each type parameter to its concrete type
        for (i, param) in generic_struct.type_parameters.iter().enumerate() {
            type_parameter_map.insert(param.value.clone(), type_args[i].clone());
        }
        
        // Create a list of field types by substituting type parameters
        let mut field_types = Vec::with_capacity(generic_struct.fields.len());
        let mut traceable_fields = Vec::new();
        
        // For each field in the generic struct
        for (i, field) in generic_struct.fields.iter().enumerate() {
            let field_type_name = field.type_name.value.clone();
            
            // Check if this field's type is a type parameter that needs substitution
            let concrete_type = if let Some(concrete_type) = type_parameter_map.get(&field_type_name) {
                // This field has a generic type parameter that needs to be substituted
                concrete_type.clone()
            } else {
                // This field has a concrete type
                // Convert to our Type enum format
                match field_type_name.as_str() {
                    "normie" => Type::Normie,
                    "thicc" => Type::Thicc,
                    "snack" => Type::Snack,
                    "meal" => Type::Meal,
                    "tea" => Type::Tea,
                    "lit" => Type::Lit,
                    "byte" => Type::Byte,
                    "rune" => Type::Rune,
                    _ => Type::Struct(field_type_name, Vec::new()), // Non-generic struct type
                }
            };
            
            // Convert our Type enum to LLVM type
            let llvm_field_type = match concrete_type {
                Type::Normie => self.context().i32_type().into(),
                Type::Thicc => self.context().i64_type().into(),
                Type::Snack => self.context().f32_type().into(),
                Type::Meal => self.context().f64_type().into(),
                Type::Lit => self.context().bool_type().into(),
                Type::Tea => self.context().i8_type().ptr_type(inkwell::AddressSpace::default()).into(),
                Type::Byte => self.context().i8_type().into(),
                Type::Rune => self.context().i32_type().into(),
                Type::Struct(struct_name, nested_type_args) => {
                    // Handle nested generic struct types if needed
                    if nested_type_args.is_empty() {
                        // Non-generic struct reference
                        if let Some(nested_struct_type) = self.get_struct_type(&self.current_package_name, &struct_name) {
                            nested_struct_type.ptr_type(inkwell::AddressSpace::default()).into()
                        } else {
                            return Err(Error::new(
                                "GNRC-002",
                                format!("Unknown struct type: {}", struct_name),
                                None,
                            ));
                        }
                    } else {
                        // Nested generic struct that needs specialization
                        return self.handle_nested_generic_struct(&struct_name, nested_type_args);
                    }
                },
                Type::Named(name) => {
                    // Handle named type - look for a struct with this name
                    if let Some(nested_struct_type) = self.get_struct_type(&self.current_package_name, &name) {
                        nested_struct_type.ptr_type(inkwell::AddressSpace::default()).into()
                    } else {
                        // For named types not found, default to a pointer to an opaque type
                        return Err(Error::new(
                            "GNRC-005",
                            format!("Unknown named type: {}", name),
                            None,
                        ));
                    }
                },
                _ => return Err(Error::new(
                    "GNRC-006",
                    format!("Unsupported field type: {:?}", concrete_type),
                    None,
                )),
            };
            
            field_types.push(llvm_field_type);
            
            // Track which fields need GC tracing (pointers, reference types)
            match concrete_type {
                Type::Tea | Type::Struct(_, _) => {
                    traceable_fields.push((i, field.name.value.clone()));
                },
                _ => {}, // Primitive types don't need GC tracing
            }
        }
        
        // Set the body of the struct with the concrete field types
        struct_type.set_body(&field_types, false);
        
        // Use our helper method to register the struct type in the registry
        let package_name = self.current_package_name.clone();
        self.register_struct_type_in_registry(&package_name, specialized_name, struct_type);
        
        // Register GC metadata for traceable fields
        if !traceable_fields.is_empty() {
            self.register_struct_gc_metadata(specialized_name, traceable_fields)?;
        }
        
        Ok(struct_type)
    }

    fn register_struct_gc_metadata(
        &mut self,
        type_name: &str,
        traceable_fields: Vec<(usize, String)>,
    ) -> Result<(), Error> {
        // Use the core implementation from context.rs
        self.register_gc_metadata(type_name, traceable_fields)
    }
}

// Extension methods that don't need to be part of the trait
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Get information about a generic struct type
    pub fn get_generic_struct_info(&self, name: &str) -> Option<&SquadStatement> {
        // This would normally look up the struct in a symbol table
        // For now, return None to indicate no struct was found
        None
    }
    
    /// Register a struct type in the type registry
    /// This helper method handles the common case of registering a struct type
    /// in the struct_types map for later retrieval
    pub fn register_struct_type_in_registry(&mut self, package_name: &str, struct_name: &str, struct_type: StructType<'ctx>) {
        // Get or create the package entry in the struct_types map
        let pkg_structs = self.struct_types
            .entry(package_name.to_string())
            .or_insert_with(std::collections::HashMap::new);
            
        // Insert the struct type into the package entry
        pkg_structs.insert(struct_name.to_string(), struct_type);
    }

    fn handle_nested_generic_struct(&mut self, struct_name: &str, nested_type_args: &[Box<Type>]) -> Result<BasicTypeEnum<'ctx>, Error> {
        // First, find the generic struct definition and clone it immediately
        let generic_nested_struct = {
            if let Some(generic_nested_struct) = self.get_generic_struct_info(struct_name) {
                generic_nested_struct.clone()
            } else {
                return Err(Error::new(
                    "GNRC-004",
                    format!("Unknown generic struct type: {}", struct_name),
                    None,
                ));
            }
        };
        
        // Recursively specialize the nested struct
        let nested_type_args_vec = nested_type_args.iter().map(|b| (**b).clone()).collect::<Vec<_>>();
        
        // Get specialized name
        let specialized_nested_name = {
            self.mono_manager
                .get_specialized_function_name(struct_name, &nested_type_args_vec)
                .ok_or_else(|| {
                    Error::new(
                        "GNRC-003",
                        format!(
                            "Failed to generate specialized name for {} with {:?}",
                            struct_name, nested_type_args
                        ),
                        None,
                    )
                })?
        };
        
        // Check if we've already created this specialized struct
        let package_name = self.current_package_name.clone();
        let nested_struct_type = if let Some(existing) = self.get_struct_type(&package_name, &specialized_nested_name) {
            existing
        } else {
            // Generate the specialized struct recursively
            self.generate_specialized_struct(
                &generic_nested_struct,
                &specialized_nested_name,
                &nested_type_args_vec,
            )?
        };
        
        Ok(nested_struct_type.ptr_type(inkwell::AddressSpace::default()).into())
    }
}
