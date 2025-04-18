//! Interface-specific type checking functionality
//!
//! This module provides specialized type checking functionality for working with
//! interfaces, including type conversion checks, interface implementation verification,
//! and method resolution for interface types.

use crate::core::type_checker::{Type, TypeChecker};
use crate::error::Error;
use std::collections::HashMap;

/// Extension trait for TypeChecker to add interface-specific functionality
pub trait InterfaceTypeChecker {
    /// Convert a value to an interface type if compatible
    fn convert_to_interface(
        &self,
        value_type: &Type,
        target_interface: &Type,
    ) -> Result<Type, Error>;
}

impl InterfaceTypeChecker for TypeChecker {
    
    fn convert_to_interface(&self, value_type: &Type, target_interface: &Type) -> Result<Type, Error> {
        // Check if the conversion is possible
        if !self.can_assign_to_interface(value_type, target_interface)? {
            return Err(Error::from_str(&format!(
                "Cannot convert {} to interface {}",
                value_type.to_string(),
                target_interface.to_string()
            )));
        }
        
        // The result of the conversion is the target interface type
        Ok(target_interface.clone())
    }
}

/// Interface method call resolver
pub struct InterfaceMethodResolver {
    /// Cache of resolved methods for interface types
    method_cache: HashMap<(String, String), (Vec<Type>, Option<Type>)>,
}

impl InterfaceMethodResolver {
    /// Create a new interface method resolver
    pub fn new() -> Self {
        Self {
            method_cache: HashMap::new(),
        }
    }
    
    /// Resolve a method call on an interface type
    pub fn resolve_method(
        &mut self,
        type_checker: &TypeChecker,
        interface_type: &Type,
        method_name: &str,
    ) -> Result<(Vec<Type>, Option<Type>), Error> {
        // Extract interface name for cache key
        let interface_name = match interface_type {
            Type::Interface(name, _) => name.clone(),
            _ => return Err(Error::from_str("Expected an interface type")),
        };
        
        // Check cache first
        let cache_key = (interface_name.clone(), method_name.to_string());
        if let Some(cached_result) = self.method_cache.get(&cache_key) {
            return Ok(cached_result.clone());
        }
        
        // Resolve the method
        let method_result = type_checker.resolve_interface_method(interface_type, method_name)?;
        
        if let Some(method_sig) = method_result {
            // Cache the result
            self.method_cache.insert(cache_key, method_sig.clone());
            return Ok(method_sig);
        }
        
        Err(Error::from_str(&format!(
            "Method '{}' not found on interface {}",
            method_name,
            interface_name
        )))
    }
    
    /// Clear the method cache
    pub fn clear_cache(&mut self) {
        self.method_cache.clear();
    }
}