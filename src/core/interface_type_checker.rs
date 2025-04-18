//! Interface type checking and verification
//!
//! This module provides a comprehensive type checking framework
//! for interfaces in the CURSED language, verifying that types
//! correctly implement interfaces and managing the relationship
//! between interfaces and their implementing types.

use crate::core::type_checker::Type;
use crate::error::Error;
use std::collections::HashMap;

/// Interface definition with methods and type parameters
#[derive(Clone, Debug)]
pub struct InterfaceDefinition {
    /// Name of the interface
    pub name: String,
    /// Methods defined by the interface with parameter types and return type
    pub methods: Vec<(String, Vec<Type>, Option<Type>)>,
    /// Type parameters for generic interfaces
    pub type_parameters: Vec<String>,
}

/// Extension trait for TypeChecker to add interface-specific functionality
pub trait InterfaceTypeChecking {
    /// Register an interface with the type checker
    fn register_interface(
        &mut self,
        name: &str,
        methods: Vec<(String, Vec<Type>, Option<Type>)>,
        type_params: Vec<String>,
    );
    
    /// Check if a type implements an interface
    fn check_interface_implementation(
        &self,
        implementing_type: &Type,
        interface_type: &Type,
    ) -> Result<bool, Error>;
    
    /// Get all interfaces implemented by a type
    fn get_implemented_interfaces(
        &self,
        implementing_type: &Type,
    ) -> Result<Vec<Type>, Error>;
    
    /// Verify a type assertion is valid (can convert from one type to another)
    fn verify_type_assertion(
        &self,
        from_type: &Type,
        to_type: &Type,
    ) -> Result<bool, Error>;
    
    /// Get interface method signatures
    fn get_interface_methods(
        &self,
        interface_name: &str,
    ) -> Option<Vec<(String, Vec<Type>, Option<Type>)>>;
}