//! Nested Generic Instantiation for Enhanced Monomorphization
//!
//! This module provides support for nested generic type instantiation,
//! allowing for proper handling of complex generic type hierarchies.

use crate::core::type_checker::Type;
use crate::error::Error;
use std::collections::HashMap;
use tracing::{debug, info, warn};

/// Extension trait for recursive type parameter substitution in complex nested generics
pub trait NestedGenericSubstitution {
    /// Performs type parameter substitution with support for nested generic types
    ///
    /// This handles cases where type parameters reference other type parameters,
    /// creating chains of substitutions that must be resolved recursively.
    ///
    /// # Arguments
    ///
    /// * `type_param_map` - A mapping from type parameter names to concrete types
    /// * `max_depth` - Maximum recursion depth to prevent infinite recursion
    ///
    /// # Returns
    ///
    /// The concrete type with all type parameters substituted
    fn substitute_nested_type_parameters(
        &self,
        type_param_map: &HashMap<String, Type>,
        max_depth: usize,
    ) -> Result<Type, Error>;
}

impl NestedGenericSubstitution for Type {
    fn substitute_nested_type_parameters(
        &self,
        type_param_map: &HashMap<String, Type>,
        max_depth: usize,
    ) -> Result<Type, Error> {
        // Check recursion depth to prevent stack overflow
        if max_depth == 0 {
            return Err(Error::from_str(
                "Maximum recursion depth reached during type parameter substitution"
            ));
        }
        
        // Substitute type parameters recursively
        match self {
            // For type parameters, substitute with the concrete type
            Type::TypeParam(param_name) => {
                if let Some(concrete_type) = type_param_map.get(param_name) {
                    // Recursively substitute type parameters in the concrete type
                    // This handles cases like T = List<U> where U is also a type parameter
                    concrete_type.substitute_nested_type_parameters(type_param_map, max_depth - 1)
                } else {
                    // Type parameter not found, return error
                    Err(Error::from_str(&format!("Unknown type parameter: {}", param_name)))
                }
            },
            
            // For named types, check if it's a type parameter
            Type::Named(name) => {
                if let Some(concrete_type) = type_param_map.get(name) {
                    // It's a type parameter, substitute it
                    concrete_type.substitute_nested_type_parameters(type_param_map, max_depth - 1)
                } else {
                    // Not a type parameter, keep as is
                    Ok(Type::Named(name.clone()))
                }
            },
            
            // Handle array types
            Type::Array(elem_type, size) => {
                let concrete_elem_type = elem_type.substitute_nested_type_parameters(type_param_map, max_depth - 1)?;
                Ok(Type::Array(Box::new(concrete_elem_type), *size))
            },
            
            // Handle slice types
            Type::Slice(elem_type) => {
                let concrete_elem_type = elem_type.substitute_nested_type_parameters(type_param_map, max_depth - 1)?;
                Ok(Type::Slice(Box::new(concrete_elem_type)))
            },
            
            // Handle map types
            Type::Map(key_type, value_type) => {
                let concrete_key_type = key_type.substitute_nested_type_parameters(type_param_map, max_depth - 1)?;
                let concrete_value_type = value_type.substitute_nested_type_parameters(type_param_map, max_depth - 1)?;
                Ok(Type::Map(Box::new(concrete_key_type), Box::new(concrete_value_type)))
            },
            
            // Handle pointer types
            Type::Pointer(target_type) => {
                let concrete_target_type = target_type.substitute_nested_type_parameters(type_param_map, max_depth - 1)?;
                Ok(Type::Pointer(Box::new(concrete_target_type)))
            },
            
            // Handle struct types - with special handling for generic structs
            Type::Struct(name, type_args) => {
                if type_args.is_empty() {
                    // Non-generic struct
                    Ok(Type::Struct(name.clone(), Vec::new()))
                } else {
                    // Generic struct - substitute type arguments
                    let mut concrete_type_args = Vec::new();
                    
                    for type_arg in type_args {
                        let concrete_type_arg = type_arg.substitute_nested_type_parameters(type_param_map, max_depth - 1)?;
                        concrete_type_args.push(Box::new(concrete_type_arg));
                    }
                    
                    Ok(Type::Struct(name.clone(), concrete_type_args))
                }
            },
            
            // Handle interface types
            Type::Interface(name, type_args) => {
                if type_args.is_empty() {
                    // Non-generic interface
                    Ok(Type::Interface(name.clone(), Vec::new()))
                } else {
                    // Generic interface - substitute type arguments
                    let mut concrete_type_args = Vec::new();
                    
                    for type_arg in type_args {
                        let concrete_type_arg = type_arg.substitute_nested_type_parameters(type_param_map, max_depth - 1)?;
                        concrete_type_args.push(Box::new(concrete_type_arg));
                    }
                    
                    Ok(Type::Interface(name.clone(), concrete_type_args))
                }
            },
            
            // Handle channel types
            Type::Channel(elem_type) => {
                let concrete_elem_type = elem_type.substitute_nested_type_parameters(type_param_map, max_depth - 1)?;
                Ok(Type::Channel(Box::new(concrete_elem_type)))
            },
            
            // Handle function types
            Type::Function(param_types, return_type) => {
                let mut concrete_param_types = Vec::new();
                
                for param_type in param_types {
                    let concrete_param_type = param_type.substitute_nested_type_parameters(type_param_map, max_depth - 1)?;
                    concrete_param_types.push(Box::new(concrete_param_type));
                }
                
                let concrete_return_type = if let Some(ret_type) = return_type {
                    Box::new(ret_type.substitute_nested_type_parameters(type_param_map, max_depth - 1)?)
                } else {
                    // Default return type for functions with no return
                    Box::new(Type::Unknown)
                };
                
                Ok(Type::Function(concrete_param_types, concrete_return_type))
            },
            
            // For primitive types, no substitution needed
            _ => Ok(self.clone()),
        }
    }
}

/// Helper function to create a type parameter map from parallel lists of parameters and arguments
pub fn create_type_param_map(param_names: &[String], type_args: &[Type]) -> HashMap<String, Type> {
    let mut map = HashMap::new();
    
    for (i, param_name) in param_names.iter().enumerate() {
        if i < type_args.len() {
            map.insert(param_name.clone(), type_args[i].clone());
            debug!("Added type parameter mapping: {} -> {:?}", param_name, type_args[i]);
        }
    }
    
    map
}

/// Substitute type parameters in a type with support for nested generics
///
/// This is a convenience function that creates a type parameter map and then
/// performs substitution with a reasonable default recursion depth.
pub fn substitute_type_parameters(typ: &Type, param_names: &[String], type_args: &[Type]) -> Result<Type, Error> {
    let type_param_map = create_type_param_map(param_names, type_args);
    typ.substitute_nested_type_parameters(&type_param_map, 32) // 32 is a reasonable depth limit
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_substitution() {
        let mut type_param_map = HashMap::new();
        type_param_map.insert("T".to_string(), Type::Normie);
        
        let type_param = Type::TypeParam("T".to_string());
        let result = type_param.substitute_nested_type_parameters(&type_param_map, 32).unwrap();
        
        assert_eq!(result, Type::Normie);
    }
    
    #[test]
    fn test_nested_substitution() {
        let mut type_param_map = HashMap::new();
        type_param_map.insert("T".to_string(), Type::Slice(Box::new(Type::TypeParam("U".to_string()))));
        type_param_map.insert("U".to_string(), Type::Normie);
        
        let type_param = Type::TypeParam("T".to_string());
        let result = type_param.substitute_nested_type_parameters(&type_param_map, 32).unwrap();
        
        assert_eq!(result, Type::Slice(Box::new(Type::Normie)));
    }
    
    #[test]
    fn test_deeply_nested_substitution() {
        let mut type_param_map = HashMap::new();
        type_param_map.insert("T".to_string(), 
            Type::Struct(
                "List".to_string(), 
                vec![Box::new(Type::TypeParam("U".to_string()))]
            )
        );
        type_param_map.insert("U".to_string(), 
            Type::Map(
                Box::new(Type::Tea), 
                Box::new(Type::TypeParam("V".to_string()))
            )
        );
        type_param_map.insert("V".to_string(), Type::Normie);
        
        let type_param = Type::TypeParam("T".to_string());
        let result = type_param.substitute_nested_type_parameters(&type_param_map, 32).unwrap();
        
        let expected = Type::Struct(
            "List".to_string(),
            vec![Box::new(Type::Map(
                Box::new(Type::Tea),
                Box::new(Type::Normie)
            ))]
        );
        
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_recursive_substitution_with_depth_limit() {
        // Create a recursive type parameter reference: T -> U, U -> T
        let mut type_param_map = HashMap::new();
        type_param_map.insert("T".to_string(), Type::TypeParam("U".to_string()));
        type_param_map.insert("U".to_string(), Type::TypeParam("T".to_string()));
        
        let type_param = Type::TypeParam("T".to_string());
        
        // This should fail due to recursion depth limit
        let result = type_param.substitute_nested_type_parameters(&type_param_map, 10);
        assert!(result.is_err());
    }
}