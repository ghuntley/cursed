//! Type inference utilities for the CURSED programming language
//!
//! This module provides functionality for inferring types from expressions,
//! particularly for struct fields when explicit type annotations are omitted.
//! It extends the type system to support more flexible struct definitions.

use crate::ast::Expression;
use crate::core::type_checker::{Type, TypeChecker};
use crate::error::Error;

/// Trait for type inference extensions to the TypeChecker
pub trait TypeInference {
    /// Infer the type of an expression
    fn infer_type(&mut self, expr: &dyn Expression) -> Result<Type, Error>;
    
    /// Infer struct field types based on initializer expressions
    fn infer_struct_field_types(
        &mut self,
        struct_name: &str,
        fields: &[(String, Option<Type>, Option<Box<dyn Expression>>)]
    ) -> Result<Vec<(String, Type)>, Error>;
    
    /// Register inferred field types for a struct
    fn register_inferred_field_types(
        &mut self,
        struct_name: &str,
        field_types: Vec<(String, Type)>
    ) -> Result<(), Error>;
}

impl TypeInference for TypeChecker {
    fn infer_type(&mut self, expr: &dyn Expression) -> Result<Type, Error> {
        // This is already implemented as get_expression_type in the TypeChecker
        // We're just creating a more intuitive API for type inference
        self.get_expression_type(expr)
    }
    
    fn infer_struct_field_types(
        &mut self,
        struct_name: &str,
        fields: &[(String, Option<Type>, Option<Box<dyn Expression>>)]
    ) -> Result<Vec<(String, Type)>, Error> {
        let mut inferred_types = Vec::new();
        
        for (field_name, explicit_type, initializer) in fields {
            let field_type = if let Some(typ) = explicit_type {
                // Use the explicit type if provided
                typ.clone()
            } else if let Some(init_expr) = initializer {
                // Infer type from initializer expression
                self.infer_type(init_expr.as_ref())?
            } else {
                // Cannot infer type without either explicit type or initializer
                return Err(Error::from_str(&format!(
                    "Cannot infer type for field '{}' in struct '{}': \
                     no type annotation or initializer provided",
                    field_name, struct_name
                )));
            };
            
            inferred_types.push((field_name.clone(), field_type));
        }
        
        Ok(inferred_types)
    }
    
    fn register_inferred_field_types(
        &mut self,
        struct_name: &str,
        field_types: Vec<(String, Type)>
    ) -> Result<(), Error> {
        // Update the struct type definition in the type system
        if let Some(struct_type) = self.get_type(struct_name) {
            if let Type::Struct(name, _) = struct_type {
                // Create a new struct type with the inferred field types
                let new_struct_type = Type::new_struct(&name, field_types);
                
                // Register the updated struct type
                self.register_type(struct_name.to_string(), new_struct_type);
                return Ok(());
            }
        }
        
        Err(Error::from_str(&format!(
            "Cannot register inferred field types: struct '{}' not found",
            struct_name
        )))
    }
}