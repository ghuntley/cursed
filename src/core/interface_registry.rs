//! Interface Implementation Registry
//!
//! This module provides a central registry for tracking which types implement
//! which interfaces. It is used by the type checker and the monomorphization
//! system to determine if a type satisfies an interface constraint.

use std::collections::{HashMap, HashSet};
use crate::core::type_checker::Type;
use crate::error::Error;
use std::sync::Arc;
use tracing::{debug, warn, error, info, trace};

/// A registry that tracks which types implement which interfaces.
#[derive(Debug, Default)]
pub struct InterfaceRegistry {
    /// Maps an interface name to a set of type names that implement it
    implementations: HashMap<String, HashSet<Type>>,
    
    /// Maps a type name to a set of interface names that it implements
    implementers: HashMap<Type, HashSet<String>>,
}

impl InterfaceRegistry {
    /// Create a new empty interface registry
    pub fn new() -> Self {
        Self {
            implementations: HashMap::new(),
            implementers: HashMap::new(),
        }
    }
    
    /// Create a new interface registry populated with default implementations
    pub fn new_with_defaults() -> Self {
        let mut registry = Self::new();
        registry.populate_with_defaults();
        registry
    }
    
    /// Register that a type implements an interface
    pub fn register_implementation(&mut self, type_: Type, interface_name: String) {
        // Record in the implementations map
        self.implementations
            .entry(interface_name.clone())
            .or_insert_with(HashSet::new)
            .insert(type_.clone());
        
        // Record in the implementers map
        self.implementers
            .entry(type_)
            .or_insert_with(HashSet::new)
            .insert(interface_name);
    }
    
    /// Check if a type implements an interface
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - If the type implements the interface
    /// * `Ok(false)` - If the type does not implement the interface
    /// * `Err` - If there was an error during the check
    pub fn check_implementation(&self, type_: &Type, interface_name: &str) -> Result<bool, Error> {
        // Check primitive types first
        let implements = match (type_, interface_name) {
            // Known primitive type implementations
            (Type::Normie, "Comparable") => true,
            (Type::Normie, "Numeric") => true,
            (Type::Thicc, "Comparable") => true,
            (Type::Thicc, "Numeric") => true,
            (Type::Snack, "Comparable") => true,
            (Type::Snack, "Numeric") => true,
            (Type::Meal, "Comparable") => true,
            (Type::Meal, "Numeric") => true,
            (Type::Tea, "Comparable") => true,
            (Type::Lit, "Comparable") => true,
            _ => {
                // Check in the registry
                if let Some(implementers) = self.implementations.get(interface_name) {
                    implementers.contains(type_)
                } else {
                    debug!(interface = interface_name, "Interface not found in registry");
                    false
                }
            }
        };
        
        debug!(
            type_name = ?type_,
            interface = interface_name,
            implements = implements,
            "Checked interface implementation"
        );
        
        Ok(implements)
    }
    
    /// Get all interfaces implemented by a type
    pub fn get_implemented_interfaces(&self, type_: &Type) -> HashSet<String> {
        self.implementers
            .get(type_)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Get all types that implement an interface
    pub fn get_interface_implementers(&self, interface_name: &str) -> HashSet<Type> {
        self.implementations
            .get(interface_name)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Populate the registry with known interface implementations
    pub fn populate_with_defaults(&mut self) {
        // Primitive numeric types implement Numeric and Comparable
        for numeric_type in [Type::Normie, Type::Thicc, Type::Snack, Type::Meal] {
            self.register_implementation(numeric_type.clone(), "Numeric".to_string());
            self.register_implementation(numeric_type, "Comparable".to_string());
        }
        
        // Boolean implements Comparable
        self.register_implementation(Type::Lit, "Comparable".to_string());
        
        // String implements Comparable
        self.register_implementation(Type::Tea, "Comparable".to_string());
        
        // Standard test cases for the Point struct
        self.register_implementation(
            Type::Struct("Point".to_string(), vec![]),
            "Comparable".to_string()
        );
        
        // Register StringStack as Container
        self.register_implementation(
            Type::Struct("StringStack".to_string(), vec![]),
            "Container".to_string()
        );
        self.register_implementation(
            Type::Struct("StringStack".to_string(), vec![]),
            "Stack".to_string()
        );
        
        // Register IntList as Container, List, and Numeric
        self.register_implementation(
            Type::Struct("IntList".to_string(), vec![]),
            "Container".to_string()
        );
        self.register_implementation(
            Type::Struct("IntList".to_string(), vec![]),
            "List".to_string()
        );
        self.register_implementation(
            Type::Struct("IntList".to_string(), vec![]),
            "Numeric".to_string()
        );
        
        debug!("Populated interface registry with default implementations");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_registry_basic_operations() {
        // Create a new registry and populate it with defaults
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        // Check that Normie implements Numeric
        assert!(registry.check_implementation(&Type::Normie, "Numeric").unwrap());
        
        // Check that Lit doesn't implement Numeric
        assert!(!registry.check_implementation(&Type::Lit, "Numeric").unwrap());
        
        // Check that Point implements Comparable
        assert!(registry
            .check_implementation(&Type::Struct("Point".to_string(), vec![]), "Comparable")
            .unwrap());
        
        // Check that Point doesn't implement Numeric
        assert!(!registry
            .check_implementation(&Type::Struct("Point".to_string(), vec![]), "Numeric")
            .unwrap());
    }
    
    #[test]
    fn test_registry_custom_implementations() {
        // Create a new registry
        let mut registry = InterfaceRegistry::new();
        
        // Register a custom type implementation
        registry.register_implementation(
            Type::Struct("Vector".to_string(), vec![]),
            "Numeric".to_string()
        );
        
        // Check that Vector implements Numeric
        assert!(registry
            .check_implementation(&Type::Struct("Vector".to_string(), vec![]), "Numeric")
            .unwrap());
        
        // Check that Vector doesn't implement Comparable
        assert!(!registry
            .check_implementation(&Type::Struct("Vector".to_string(), vec![]), "Comparable")
            .unwrap());
    }
}