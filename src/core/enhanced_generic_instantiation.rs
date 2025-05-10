//! Enhanced Generic Type Instantiation Module
//!
//! This module provides improved type parameter substitution for generic types,
//! with special handling for nested generics and complex type hierarchies.

use crate::ast::declarations::{FunctionStatement, SquadStatement};
use crate::ast::expressions::Expression;
use crate::ast::statements::Statement;
use crate::ast::traits::Node;
use crate::core::type_checker::Type;
use crate::error::Error;
use std::collections::HashMap;
use tracing::{debug, info, warn, error, span, Level};

/// An enhanced generic instantiator that provides improved handling of 
/// nested generic types and complex type hierarchies.
#[derive(Debug, Default)]
pub struct EnhancedGenericInstantiator {
    /// Maps type parameter names to concrete types
    type_param_map: HashMap<String, Type>,
    /// Tracks recursion depth to prevent infinite recursion
    recursion_depth: usize,
    /// Maximum allowed recursion depth
    max_recursion_depth: usize,
}

impl EnhancedGenericInstantiator {
    /// Create a new enhanced generic instantiator with default settings
    pub fn new() -> Self {
        EnhancedGenericInstantiator {
            type_param_map: HashMap::new(),
            recursion_depth: 0,
            max_recursion_depth: 32, // Reasonable default to prevent stack overflow
        }
    }

    /// Create a new enhanced generic instantiator with custom recursion depth
    pub fn new_with_max_depth(max_depth: usize) -> Self {
        EnhancedGenericInstantiator {
            type_param_map: HashMap::new(),
            recursion_depth: 0,
            max_recursion_depth: max_depth,
        }
    }

    /// Add a type parameter mapping
    pub fn add_type_param(&mut self, param_name: &str, concrete_type: Type) {
        debug!("Adding type parameter mapping: {} -> {:?}", param_name, concrete_type);
        self.type_param_map.insert(param_name.to_string(), concrete_type);
    }

    /// Check if a type parameter is mapped
    pub fn has_type_param(&self, param_name: &str) -> bool {
        self.type_param_map.contains_key(param_name)
    }

    /// Get the concrete type for a type parameter
    pub fn get_type_param(&self, param_name: &str) -> Option<&Type> {
        self.type_param_map.get(param_name)
    }

    /// Instantiate a type by substituting type parameters with concrete types
    #[tracing::instrument(skip(self), level = "debug")]
    pub fn instantiate_type(&mut self, typ: &Type) -> Result<Type, Error> {
        // Check for recursion depth to prevent stack overflow
        if self.recursion_depth >= self.max_recursion_depth {
            return Err(Error::from_str(
                &format!("Maximum recursion depth ({}) reached during type instantiation", 
                        self.max_recursion_depth)
            ));
        }
        
        // Increment recursion depth
        self.recursion_depth += 1;
        
        // Use a defer-like pattern to ensure recursion depth is decremented
        struct RecursionGuard<'a> {
            instantiator: &'a mut EnhancedGenericInstantiator,
        }
        
        impl<'a> Drop for RecursionGuard<'a> {
            fn drop(&mut self) {
                self.instantiator.recursion_depth -= 1;
            }
        }
        
        let _guard = RecursionGuard { instantiator: self };
        
        // Perform type substitution based on the type
        match typ {
            // For type parameters, substitute with the concrete type
            Type::TypeParam(param_name) => {
                if let Some(concrete_type) = self.type_param_map.get(param_name) {
                    // Handle recursively generic types by also instantiating the concrete type
                    // This is important for cases like T = List<U> when U is also a type parameter
                    self.instantiate_type(concrete_type)
                } else {
                    // Type parameter not found, return error
                    Err(Error::from_str(&format!("Unknown type parameter: {}", param_name)))
                }
            },
            
            // For named types, check if it's a type parameter
            Type::Named(name) => {
                if let Some(concrete_type) = self.type_param_map.get(name) {
                    // It's a type parameter, substitute it
                    Ok(concrete_type.clone())
                } else {
                    // Not a type parameter, keep as is
                    Ok(Type::Named(name.clone()))
                }
            },
            
            // Handle array types
            Type::Array(elem_type, size) => {
                let concrete_elem_type = self.instantiate_type(elem_type)?;
                Ok(Type::Array(Box::new(concrete_elem_type), *size))
            },
            
            // Handle slice types
            Type::Slice(elem_type) => {
                let concrete_elem_type = self.instantiate_type(elem_type)?;
                Ok(Type::Slice(Box::new(concrete_elem_type)))
            },
            
            // Handle map types
            Type::Map(key_type, value_type) => {
                let concrete_key_type = self.instantiate_type(key_type)?;
                let concrete_value_type = self.instantiate_type(value_type)?;
                Ok(Type::Map(Box::new(concrete_key_type), Box::new(concrete_value_type)))
            },
            
            // Handle pointer types
            Type::Pointer(target_type) => {
                let concrete_target_type = self.instantiate_type(target_type)?;
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
                        let concrete_type_arg = self.instantiate_type(type_arg)?;
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
                        let concrete_type_arg = self.instantiate_type(type_arg)?;
                        concrete_type_args.push(Box::new(concrete_type_arg));
                    }
                    
                    Ok(Type::Interface(name.clone(), concrete_type_args))
                }
            },
            
            // Handle channel types
            Type::Channel(elem_type) => {
                let concrete_elem_type = self.instantiate_type(elem_type)?;
                Ok(Type::Channel(Box::new(concrete_elem_type)))
            },
            
            // Handle function types
            Type::Function(param_types, return_type) => {
                let mut concrete_param_types = Vec::new();
                
                for param_type in param_types {
                    let concrete_param_type = self.instantiate_type(param_type)?;
                    concrete_param_types.push(concrete_param_type);
                }
                
                let concrete_return_type = if let Some(ret_type) = return_type {
                    Some(Box::new(self.instantiate_type(ret_type)?))
                } else {
                    None
                };
                
                Ok(Type::Function(concrete_param_types, concrete_return_type))
            },
            
            // For primitive types, no substitution needed
            _ => Ok(typ.clone()),
        }
    }
    
    /// Monomorphize a generic function statement into a concrete function statement
    pub fn monomorphize_function(
        &mut self,
        generic_function: &FunctionStatement,
        type_args: &[Type],
    ) -> Result<FunctionStatement, Error> {
        // Create specialized function by cloning the original
        let mut specialized_function = generic_function.clone();
        
        // Clear type parameters since this is now a concrete function
        specialized_function.type_parameters.clear();
        specialized_function.generic_constraints.clear();
        
        // Monomorphize parameter types
        for param in &mut specialized_function.parameters {
            let original_type_name = param.type_name.string();
            let original_type = Type::Named(original_type_name.clone());
            
            // Attempt to substitute the type
            match self.instantiate_type(&original_type) {
                Ok(concrete_type) => {
                    // Convert the concrete type back to a string representation
                    let concrete_type_str = concrete_type.to_string();
                    
                    // Update the parameter type AST node
                    // In a real implementation, we would create a proper AST node for the concrete type
                    // Here we're using a simple replacement
                    let new_type = crate::ast::expressions::Identifier {
                        token: concrete_type_str.clone(),
                        value: concrete_type_str,
                    };
                    
                    param.type_name = Box::new(new_type);
                },
                Err(e) => {
                    // Failed to substitute the type, propagate the error
                    return Err(Error::from_str(&format!(
                        "Failed to substitute type for parameter {}: {}",
                        param.name.value, e
                    )));
                }
            }
        }
        
        // Monomorphize return type if present
        if let Some(return_type) = &mut specialized_function.return_type {
            let original_return_type_name = return_type.string();
            let original_return_type = Type::Named(original_return_type_name.clone());
            
            // Attempt to substitute the return type
            match self.instantiate_type(&original_return_type) {
                Ok(concrete_return_type) => {
                    // Convert the concrete type back to a string representation
                    let concrete_type_str = concrete_return_type.to_string();
                    
                    // Update the return type AST node
                    let new_type = crate::ast::expressions::Identifier {
                        token: concrete_type_str.clone(),
                        value: concrete_type_str,
                    };
                    
                    *return_type = Box::new(new_type);
                },
                Err(e) => {
                    // Failed to substitute the return type, propagate the error
                    return Err(Error::from_str(&format!(
                        "Failed to substitute return type: {}",
                        e
                    )));
                }
            }
        }
        
        // For a complete implementation, we would also monomorphize the function body
        // by traversing the AST and substituting type references
        
        Ok(specialized_function)
    }
    
    /// Monomorphize a generic struct statement into a concrete struct statement
    pub fn monomorphize_struct(
        &mut self,
        generic_struct: &SquadStatement,
        type_args: &[Type],
    ) -> Result<SquadStatement, Error> {
        // Create specialized struct by cloning the original
        let mut specialized_struct = generic_struct.clone();
        
        // Clear type parameters since this is now a concrete struct
        specialized_struct.type_parameters.clear();
        
        // Monomorphize field types
        for field in &mut specialized_struct.fields {
            let original_type_name = field.type_name.string();
            let original_type = Type::Named(original_type_name.clone());
            
            // Attempt to substitute the type
            match self.instantiate_type(&original_type) {
                Ok(concrete_type) => {
                    // Convert the concrete type back to a string representation
                    let concrete_type_str = concrete_type.to_string();
                    
                    // Update the field type AST node
                    let new_type = crate::ast::expressions::Identifier {
                        token: concrete_type_str.clone(),
                        value: concrete_type_str,
                    };
                    
                    field.type_name = Box::new(new_type);
                },
                Err(e) => {
                    // Failed to substitute the type, propagate the error
                    return Err(Error::from_str(&format!(
                        "Failed to substitute type for field {}: {}",
                        field.name.value, e
                    )));
                }
            }
        }
        
        Ok(specialized_struct)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_instantiate_simple_type() {
        let mut instantiator = EnhancedGenericInstantiator::new();
        instantiator.add_type_param("T", Type::Normie);
        
        let result = instantiator.instantiate_type(&Type::TypeParam("T".to_string())).unwrap();
        assert_eq!(result, Type::Normie);
    }
    
    #[test]
    fn test_instantiate_array_type() {
        let mut instantiator = EnhancedGenericInstantiator::new();
        instantiator.add_type_param("T", Type::Normie);
        
        let result = instantiator.instantiate_type(&Type::Array(Box::new(Type::TypeParam("T".to_string())), 10)).unwrap();
        assert_eq!(result, Type::Array(Box::new(Type::Normie), 10));
    }
    
    #[test]
    fn test_instantiate_nested_generic() {
        let mut instantiator = EnhancedGenericInstantiator::new();
        instantiator.add_type_param("T", Type::Slice(Box::new(Type::TypeParam("U".to_string()))));
        instantiator.add_type_param("U", Type::Normie);
        
        let result = instantiator.instantiate_type(&Type::TypeParam("T".to_string())).unwrap();
        assert_eq!(result, Type::Slice(Box::new(Type::Normie)));
    }
    
    #[test]
    fn test_instantiate_struct_type() {
        let mut instantiator = EnhancedGenericInstantiator::new();
        instantiator.add_type_param("T", Type::Normie);
        instantiator.add_type_param("U", Type::Tea);
        
        let generic_struct = Type::Struct(
            "Container".to_string(),
            vec![Box::new(Type::TypeParam("T".to_string())), Box::new(Type::TypeParam("U".to_string()))]
        );
        
        let result = instantiator.instantiate_type(&generic_struct).unwrap();
        
        let expected = Type::Struct(
            "Container".to_string(),
            vec![Box::new(Type::Normie), Box::new(Type::Tea)]
        );
        
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_instantiate_deeply_nested_types() {
        let mut instantiator = EnhancedGenericInstantiator::new();
        instantiator.add_type_param("T", Type::Struct(
            "List".to_string(),
            vec![Box::new(Type::TypeParam("U".to_string()))]
        ));
        instantiator.add_type_param("U", Type::Map(
            Box::new(Type::Tea),
            Box::new(Type::TypeParam("V".to_string()))
        ));
        instantiator.add_type_param("V", Type::Normie);
        
        // T = List<U>, U = Map<Tea, V>, V = Normie
        // So T = List<Map<Tea, Normie>>
        let result = instantiator.instantiate_type(&Type::TypeParam("T".to_string())).unwrap();
        
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
    fn test_recursion_depth_limit() {
        let mut instantiator = EnhancedGenericInstantiator::new_with_max_depth(5);
        
        // Set up cyclic type parameters
        instantiator.add_type_param("T", Type::TypeParam("U".to_string()));
        instantiator.add_type_param("U", Type::TypeParam("V".to_string()));
        instantiator.add_type_param("V", Type::TypeParam("W".to_string()));
        instantiator.add_type_param("W", Type::TypeParam("X".to_string()));
        instantiator.add_type_param("X", Type::TypeParam("Y".to_string()));
        instantiator.add_type_param("Y", Type::TypeParam("T".to_string())); // Cycle back to T
        
        // Should fail due to recursion limit
        let result = instantiator.instantiate_type(&Type::TypeParam("T".to_string()));
        assert!(result.is_err());
    }
}