//! # Automatic Interface Implementation Registration
//!
//! This module integrates the type checker with the interface registry to automatically
//! register interface implementations found during type checking. This enables the
//! compiler to track which types implement which interfaces without requiring explicit
//! registration.

use crate::core::interface_registry::{InterfaceRegistry, GenericInterfaceImpl};
use crate::core::type_checker::{Type, TypeChecker};
use crate::error::Error;
use std::sync::{Arc, Mutex};
use tracing::{debug, error, info, instrument, trace, warn};

/// Trait for automatically registering interface implementations
/// during type checking
pub trait AutoInterfaceRegistration {
    /// Register that a type implements an interface
    fn register_interface_implementation(
        &mut self,
        implementing_type: &Type,
        interface_name: &str
    ) -> Result<(), Error>;
    
    /// Register a generic type that implements an interface
    fn register_generic_interface_implementation(
        &mut self,
        type_name: &str,
        type_params: &[String],
        interface_name: &str,
        constraints: Vec<(String, String)>
    ) -> Result<(), Error>;
    
    /// Check interface implementation during type checking and automatically
    /// register successful implementations
    fn check_and_register_interface_implementation(
        &mut self,
        implementing_type: &Type,
        interface: &Type
    ) -> Result<bool, Error>;
    
    /// Get the interface registry instance
    fn get_interface_registry(&self) -> Arc<Mutex<InterfaceRegistry>>;
}

impl AutoInterfaceRegistration for TypeChecker {
    /// Register a concrete type as implementing an interface
    #[instrument(skip(self), level = "debug")]
    fn register_interface_implementation(
        &mut self,
        implementing_type: &Type,
        interface_name: &str
    ) -> Result<(), Error> {
        debug!(
            "Registering {} as implementing interface {}",
            format!("{:?}", implementing_type),
            interface_name
        );
        
        // Get the registry instance
        let registry = self.get_interface_registry();
        let mut registry = registry.lock().unwrap();
        
        // Register the implementation
        registry.register_implementation(implementing_type.clone(), interface_name.to_string());
        
        debug!(
            "Successfully registered {} as implementing {}",
            format!("{:?}", implementing_type),
            interface_name
        );
        
        Ok(())
    }
    
    /// Register a generic type as implementing an interface
    #[instrument(skip(self), level = "debug")]
    fn register_generic_interface_implementation(
        &mut self,
        type_name: &str,
        type_params: &[String],
        interface_name: &str,
        constraints: Vec<(String, String)>
    ) -> Result<(), Error> {
        debug!(
            "Registering generic type {} with type parameters {:?} as implementing interface {}",
            type_name,
            type_params,
            interface_name
        );
        
        // Get the registry instance
        let registry = self.get_interface_registry();
        let mut registry = registry.lock().unwrap();
        
        // Convert type parameters to owned Strings
        let type_param_strings: Vec<String> = type_params.iter()
            .map(|s| s.clone())
            .collect();
        
        // Register the generic implementation
        registry.register_generic_implementation(
            type_name.to_string(),
            type_param_strings,
            interface_name.to_string(),
            constraints
        );
        
        debug!(
            "Successfully registered generic type {} as implementing {}",
            type_name,
            interface_name
        );
        
        Ok(())
    }
    
    /// Check if a type implements an interface and register it if so
    #[instrument(skip(self), level = "debug")]
    fn check_and_register_interface_implementation(
        &mut self,
        implementing_type: &Type,
        interface: &Type
    ) -> Result<bool, Error> {
        // Extract the interface name
        let (interface_name, _) = match interface {
            Type::Interface(name, type_args) => (name, type_args),
            _ => return Err(Error::from_str("Expected an interface type")),
        };
        
        // First check if the type implements the interface
        let implements = self.check_interface_implementation(implementing_type, interface)?;
        
        // If it implements the interface, register it
        if implements {
            match implementing_type {
                // For concrete structs
                Type::Struct(name, type_args) if type_args.is_empty() => {
                    self.register_interface_implementation(implementing_type, interface_name)?;
                },
                
                // For generic structs
                Type::Struct(name, _) => {
                    // Get the type parameters for this struct
                    // Get type parameters first - clone them to avoid borrowing issues
                    let type_params_opt = self.type_params_map.get(name).cloned();
                    if let Some(type_params) = type_params_opt {
                        // Extract constraints
                        // For this implementation, we assume no constraints
                        // A more complete implementation would extract constraints from the code
                        let constraints = Vec::new();
                        
                        self.register_generic_interface_implementation(
                            name, 
                            &type_params, 
                            interface_name,
                            constraints
                        )?;
                    } else {
                        // If no type parameters found, treat as concrete type
                        self.register_interface_implementation(implementing_type, interface_name)?;
                    }
                },
                
                // For all other types
                _ => {
                    self.register_interface_implementation(implementing_type, interface_name)?;
                }
            }
            
            debug!(
                "Successfully verified and registered {:?} as implementing {}",
                implementing_type,
                interface_name
            );
        }
        
        Ok(implements)
    }
    
    /// Get the shared interface registry instance
    fn get_interface_registry(&self) -> Arc<Mutex<InterfaceRegistry>> {
        // Return the registry that's now stored in the TypeChecker
        self.interface_registry.clone()
    }
}

// Extension trait to add caching to the interface registry
pub trait CachedInterfaceRegistry {
    /// Check if a type implements an interface with caching
    fn check_implementation_cached(
        &mut self,
        type_: &Type,
        interface_name: &str
    ) -> Result<bool, Error>;
    
    /// Clear the cache
    fn clear_cache(&mut self);
}

// Implement cache wrapper around the registry
impl CachedInterfaceRegistry for InterfaceRegistry {
    fn check_implementation_cached(
        &mut self,
        type_: &Type,
        interface_name: &str
    ) -> Result<bool, Error> {
        // In a real implementation, we would check a cache first
        // before performing the actual check
        // For simplicity, we'll just delegate to the regular check
        debug!("Checking if {:?} implements {} (cached)", type_, interface_name);
        
        // For now just forward to the regular implementation
        self.check_implementation(type_, interface_name)
    }
    
    fn clear_cache(&mut self) {
        // Clear the implementation cache
        debug!("Clearing interface implementation cache");
        // In a real implementation, this would clear the actual cache
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    
    #[path = "../../tests/common.rs"]
    mod common;
    
    #[test]
    fn test_auto_registration_concrete_type() {
        // Set up tracing for the test
        common::tracing::setup();
        
        // Create a type checker with automatic registration
        let mut type_checker = TypeChecker::new();
        
        // Add a struct and interface to the type checker
        let mut fields = std::collections::HashMap::new();
        fields.insert("name".to_string(), Type::Tea);
        fields.insert("age".to_string(), Type::Normie);
        type_checker.register_struct("Person", fields, Vec::new());
        
        // Register methods for the struct
        let method_signatures = vec![
            ("getName".to_string(), Vec::new(), Some(Type::Tea)),
            ("getAge".to_string(), Vec::new(), Some(Type::Normie)),
        ];
        type_checker.struct_methods_map.insert("Person".to_string(), method_signatures);
        
        // Register an interface
        let interface_methods = vec![
            ("getName".to_string(), Vec::new(), Some(Type::Tea)),
        ];
        type_checker.interface_map.insert("Named".to_string(), interface_methods);
        
        // Check and register implementation
        let person_type = Type::Struct("Person".to_string(), Vec::new());
        let named_interface = Type::Interface("Named".to_string(), Vec::new());
        
        // This should check and register automatically
        let result = type_checker.check_and_register_interface_implementation(
            &person_type,
            &named_interface
        ).unwrap();
        
        assert!(result, "Person should implement Named interface");
        
        // Verify that Person is registered as implementing Named
        let registry = type_checker.get_interface_registry();
        let registry = registry.lock().unwrap();
        
        assert!(registry.check_implementation(&person_type, "Named").unwrap());
    }
    
    #[test]
    fn test_auto_registration_generic_type() {
        // Set up tracing for the test
        common::tracing::setup();
        
        // Create a type checker with automatic registration
        let mut type_checker = TypeChecker::new();
        
        // Register a generic struct
        let mut fields = std::collections::HashMap::new();
        fields.insert("items".to_string(), Type::Unknown);
        fields.insert("size".to_string(), Type::Normie);
        type_checker.register_struct("Stack", fields, vec!["T".to_string()]);
        
        // Register methods for the struct
        let method_signatures = vec![
            ("push".to_string(), vec![Type::TypeParam("T".to_string())], None),
            ("pop".to_string(), Vec::new(), Some(Type::TypeParam("T".to_string()))),
            ("isEmpty".to_string(), Vec::new(), Some(Type::Lit)),
        ];
        type_checker.struct_methods_map.insert("Stack".to_string(), method_signatures);
        
        // Register an interface
        let interface_methods = vec![
            ("isEmpty".to_string(), Vec::new(), Some(Type::Lit)),
        ];
        type_checker.interface_map.insert("Container".to_string(), interface_methods);
        
        // Create a generic Stack type
        let stack_type = Type::Struct("Stack".to_string(), Vec::new());
        let container_interface = Type::Interface("Container".to_string(), Vec::new());
        
        // This should check and register automatically
        let result = type_checker.check_and_register_interface_implementation(
            &stack_type,
            &container_interface
        ).unwrap();
        
        assert!(result, "Stack should implement Container interface");
        
        // Verify that Stack is registered as implementing Container
        let registry = type_checker.get_interface_registry();
        let registry = registry.lock().unwrap();
        
        // Check a concrete instantiation of Stack
        let stack_string = Type::Struct(
            "Stack".to_string(), 
            vec![Box::new(Type::Tea)]
        );
        
        assert!(registry.check_implementation(&stack_string, "Container").unwrap());
    }
}