//! Interface Implementation Registry
//!
//! This module provides a central registry for tracking which types implement
//! which interfaces. It is used by the type checker and the monomorphization
//! system to determine if a type satisfies an interface constraint.
//!
//! The registry now supports both concrete and generic interface implementations.

use std::collections::{HashMap, HashSet};
use crate::core::type_checker::Type;
use crate::error::Error;
use std::sync::Arc;
use tracing::{debug, warn, error, info, trace, instrument};
use crate::core::async_constraint_checker::AsyncConstraintChecking;
use crate::core::constraint_recovery::{ConstraintRecovery, ConstraintRecoveryExtension, RecoveryStrategy};

/// Represents a generic interface implementation with type parameters
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GenericInterfaceImpl {
    /// The name of the generic type that implements the interface
    pub type_name: String,
    
    /// The type parameters for the generic type
    pub type_params: Vec<String>,
    
    /// The name of the interface being implemented
    pub interface_name: String,
    
    /// Optional constraints on the type parameters
    pub constraints: Vec<(String, String)>, // (type_param, interface_constraint)
}

/// A registry that tracks which types implement which interfaces.
#[derive(Debug, Default, Clone)]
pub struct InterfaceRegistry {
    /// Maps an interface name to a set of type names that implement it
    implementations: HashMap<String, HashSet<Type>>,
    
    /// Maps a type name to a set of interface names that it implements
    implementers: HashMap<Type, HashSet<String>>,
    
    /// Stores generic interface implementations where the implementing type is generic
    /// Maps interface name to a set of generic implementations
    generic_implementations: HashMap<String, Vec<GenericInterfaceImpl>>,
    
    /// Maps interface names to method signatures (method_name -> signature)
    interface_methods: HashMap<String, HashMap<String, String>>,
    
    /// Maps interface names to recovery strategies
    recovery_strategies: HashMap<String, crate::core::constraint_recovery::RecoveryStrategy>,
}

impl InterfaceRegistry {
    /// Create a new empty interface registry
    pub fn new() -> Self {
        Self {
            implementations: HashMap::new(),
            implementers: HashMap::new(),
            generic_implementations: HashMap::new(),
            interface_methods: HashMap::new(),
            recovery_strategies: HashMap::new(),
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
    
    /// Get mutable access to interface methods
    pub fn interface_methods_mut(&mut self) -> &mut HashMap<String, HashMap<String, String>> {
        &mut self.interface_methods
    }
    
    /// Get immutable access to interface methods
    pub fn interface_methods(&self) -> &HashMap<String, HashMap<String, String>> {
        &self.interface_methods
    }
    
    /// Get mutable access to recovery strategies
    pub fn recovery_strategies_mut(&mut self) -> &mut HashMap<String, crate::core::constraint_recovery::RecoveryStrategy> {
        &mut self.recovery_strategies
    }
    
    /// Get immutable access to recovery strategies
    pub fn recovery_strategies(&self) -> &HashMap<String, crate::core::constraint_recovery::RecoveryStrategy> {
        &self.recovery_strategies
    }
    
    /// Get immutable access to implementations
    pub fn implementations(&self) -> &HashMap<String, HashSet<Type>> {
        &self.implementations
    }

    /// Register a generic type that implements an interface
    /// 
    /// # Arguments
    /// 
    /// * `type_name` - The name of the generic type (e.g., "Stack")
    /// * `type_params` - The type parameters for the generic type (e.g., ["T"])
    /// * `interface_name` - The name of the interface being implemented
    /// * `constraints` - Optional constraints on the type parameters
    pub fn register_generic_implementation(
        &mut self,
        type_name: String,
        type_params: Vec<String>,
        interface_name: String,
        constraints: Vec<(String, String)>
    ) {
        // Create the generic implementation
        let generic_impl = GenericInterfaceImpl {
            type_name: type_name.clone(),
            type_params: type_params.clone(),
            interface_name: interface_name.clone(),
            constraints: constraints.clone(),
        };
        
        // Add to the generic implementations map
        self.generic_implementations
            .entry(interface_name.clone())
            .or_insert_with(Vec::new)
            .push(generic_impl);
            
        debug!(
            type_name = type_name, 
            type_params = ?type_params,
            interface = interface_name,
            constraints = ?constraints,
            "Registered generic interface implementation"
        );
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
                // First check directly in the registry for concrete types
                if let Some(implementers) = self.implementations.get(interface_name) {
                    if implementers.contains(type_) {
                        return Ok(true);
                    }
                }
                
                // If it's not a direct match, check if it's a generic type
                if let Type::Struct(struct_name, type_args) = type_ {
                    // Check if there's a generic implementation for this type
                    if let Some(generic_impls) = self.generic_implementations.get(interface_name) {
                        for impl_ in generic_impls {
                            if impl_.type_name == *struct_name {
                                // This is a potential match - we need to verify the constraints
                                // Convert Vec<Box<Type>> to Vec<Type> for checking
                                let concrete_args: Vec<Type> = type_args.iter()
                                    .map(|t| (**t).clone())
                                    .collect();
                                
                                if self.check_generic_constraints(&concrete_args, &impl_.type_params, &impl_.constraints)? {
                                    return Ok(true);
                                }
                            }
                        }
                    }
                    false
                } else {
                    // Not a struct type or not found in registry
                    debug!(interface = interface_name, "Interface not found in registry for type: {:?}", type_);
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
    
    /// Check if a set of concrete type arguments satisfies the constraints of a generic implementation
    ///
    /// # Arguments
    ///
    /// * `type_args` - The concrete type arguments
    /// * `type_params` - The type parameters of the generic type
    /// * `constraints` - The constraints on the type parameters
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - If the constraints are satisfied
    /// * `Ok(false)` - If the constraints are not satisfied
    /// * `Err` - If there was an error during the check
    #[instrument(skip(self, type_args, type_params, constraints), level = "debug")]
    fn check_generic_constraints(
        &self,
        type_args: &[Type],
        type_params: &[String],
        constraints: &[(String, String)]
    ) -> Result<bool, Error> {
        // Check if we have the right number of type arguments
        if type_args.len() != type_params.len() {
            debug!(
                "Wrong number of type arguments: expected {}, got {}",
                type_params.len(),
                type_args.len()
            );
            return Ok(false);
        }
        
        // Create a mapping from type parameter names to concrete types
        let mut type_map = HashMap::new();
        for (i, param_name) in type_params.iter().enumerate() {
            type_map.insert(param_name.clone(), type_args[i].clone());
        }
        
        // For small numbers of constraints, just check sequentially
        if constraints.len() <= 2 {
            // Check each constraint sequentially
            for (param_name, interface_name) in constraints {
                if let Some(concrete_type) = type_map.get(param_name) {
                    // Check if the concrete type satisfies the constraint
                    if !self.check_implementation(&concrete_type, interface_name)? {
                        debug!(
                            "Type parameter {} with concrete type {:?} does not implement required interface {}",
                            param_name, concrete_type, interface_name
                        );
                        return Ok(false);
                    }
                } else {
                    // This should never happen if the type parameter lists match
                    debug!("Type parameter {} not found in mapping", param_name);
                    return Ok(false);
                }
            }
            
            // All constraints satisfied
            debug!("All generic constraints satisfied sequentially");
            Ok(true)
        } else {
            // For larger numbers of constraints, use parallel checking
            // Create constraint pairs for parallel checking
            let mut constraint_pairs = Vec::with_capacity(constraints.len());
            
            for (param_name, interface_name) in constraints {
                if let Some(concrete_type) = type_map.get(param_name) {
                    constraint_pairs.push((concrete_type.clone(), interface_name.clone()));
                } else {
                    // This should never happen if the type parameter lists match
                    debug!("Type parameter {} not found in mapping", param_name);
                    return Ok(false);
                }
            }
            
            // Use self's implementation of AsyncConstraintChecking trait
            let results = self.check_constraints_parallel(constraint_pairs);
            
            // All constraints must be satisfied
            for result in results {
                match result {
                    Ok(satisfied) => {
                        if !satisfied {
                            debug!("One of the constraints is not satisfied");
                            return Ok(false);
                        }
                    }
                    Err(err) => {
                        // Propagate any errors
                        return Err(err);
                    }
                }
            }
            
            // All constraints satisfied
            debug!("All generic constraints satisfied in parallel");
            Ok(true)
        }
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
    
    /// Get all generic types that implement an interface
    pub fn get_generic_interface_implementers(&self, interface_name: &str) -> Vec<GenericInterfaceImpl> {
        self.generic_implementations
            .get(interface_name)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Test if a generic type with specific type arguments implements an interface
    ///
    /// # Arguments
    ///
    /// * `type_name` - The name of the generic type
    /// * `type_args` - The concrete type arguments
    /// * `interface_name` - The name of the interface to check
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - If the type implements the interface
    /// * `Ok(false)` - If the type does not implement the interface
    /// * `Err` - If there was an error during the check
    pub fn test_generic_implementation(
        &self,
        type_name: &str,
        type_args: &[Type],
        interface_name: &str
    ) -> Result<bool, Error> {
        // Check if there are any generic implementations for this interface
        if let Some(generic_impls) = self.generic_implementations.get(interface_name) {
            // Look for a matching implementation
            for impl_ in generic_impls {
                if impl_.type_name == type_name {
                    // Check type parameter constraints
                    return self.check_generic_constraints(type_args, &impl_.type_params, &impl_.constraints);
                }
            }
        }
        
        // No matching implementation found
        debug!(
            "No generic implementation found for {} with interface {}",
            type_name, interface_name
        );
        Ok(false)
    }
    
    /// Populate the registry with known interface implementations
    pub fn populate_with_defaults(&mut self) {
        // Populate interface methods
        self.populate_default_interface_methods();
        
        // Populate recovery strategies
        self.populate_default_recovery_strategies();
        
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
        
        // Register generic interface implementations
        
        // GenericStack[T] implements Container
        self.register_generic_implementation(
            "GenericStack".to_string(),
            vec!["T".to_string()],
            "Container".to_string(),
            vec![] // No constraints on T
        );
        
        // GenericStack[T] implements Stack
        self.register_generic_implementation(
            "GenericStack".to_string(),
            vec!["T".to_string()],
            "Stack".to_string(),
            vec![] // No constraints on T
        );
        
        // Pair[T, U] implements Container when T implements Comparable
        self.register_generic_implementation(
            "Pair".to_string(),
            vec!["T".to_string(), "U".to_string()],
            "Container".to_string(),
            vec![("T".to_string(), "Comparable".to_string())] // T must be Comparable
        );
        
        // SortedList[T] implements List and Container when T implements Comparable
        self.register_generic_implementation(
            "SortedList".to_string(),
            vec!["T".to_string()],
            "List".to_string(),
            vec![("T".to_string(), "Comparable".to_string())] // T must be Comparable
        );
        self.register_generic_implementation(
            "SortedList".to_string(),
            vec!["T".to_string()],
            "Container".to_string(),
            vec![("T".to_string(), "Comparable".to_string())] // T must be Comparable
        );
        
        // Dictionary[K, V] implements Map when K implements Comparable
        self.register_generic_implementation(
            "Dictionary".to_string(),
            vec!["K".to_string(), "V".to_string()],
            "Map".to_string(),
            vec![("K".to_string(), "Comparable".to_string())] // K must be Comparable
        );
        
        debug!("Populated interface registry with default implementations");
    }
    
    /// Populate the registry with default interface method signatures
    fn populate_default_interface_methods(&mut self) {
        // Comparable interface
        let mut comparable_methods = HashMap::new();
        comparable_methods.insert("Compare".to_string(), "a Self, b Self".to_string());
        comparable_methods.insert("Equals".to_string(), "a Self, b Self".to_string());
        self.interface_methods.insert("Comparable".to_string(), comparable_methods);
        
        // Numeric interface
        let mut numeric_methods = HashMap::new();
        numeric_methods.insert("Add".to_string(), "a Self, b Self".to_string());
        numeric_methods.insert("Subtract".to_string(), "a Self, b Self".to_string());
        numeric_methods.insert("Multiply".to_string(), "a Self, b Self".to_string());
        numeric_methods.insert("Divide".to_string(), "a Self, b Self".to_string());
        self.interface_methods.insert("Numeric".to_string(), numeric_methods);
        
        // Container interface
        let mut container_methods = HashMap::new();
        container_methods.insert("Size".to_string(), "self Self".to_string());
        container_methods.insert("IsEmpty".to_string(), "self Self".to_string());
        self.interface_methods.insert("Container".to_string(), container_methods);
        
        // List interface
        let mut list_methods = HashMap::new();
        list_methods.insert("Get".to_string(), "self Self, index Normie".to_string());
        list_methods.insert("Set".to_string(), "self Self, index Normie, value T".to_string());
        list_methods.insert("Append".to_string(), "self Self, value T".to_string());
        self.interface_methods.insert("List".to_string(), list_methods);
        
        debug!("Populated default interface method signatures");
    }
    
    /// Populate the registry with default recovery strategies
    fn populate_default_recovery_strategies(&mut self) {
        // Set default recovery strategies for known interfaces
        self.recovery_strategies.insert("Comparable".to_string(), RecoveryStrategy::GenerateStub);
        self.recovery_strategies.insert("Numeric".to_string(), RecoveryStrategy::GenerateStub);
        self.recovery_strategies.insert("Container".to_string(), RecoveryStrategy::GeneratePlaceholder);
        self.recovery_strategies.insert("List".to_string(), RecoveryStrategy::SuggestAlternatives);
        
        debug!("Populated default recovery strategies");
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
    
    #[test]
    fn test_generic_interface_implementations() {
        // Create a new registry and populate it with defaults
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        // Test GenericStack[T] implements Container with any type
        assert!(registry
            .test_generic_implementation(
                "GenericStack",
                &[Type::Tea],
                "Container"
            )
            .unwrap());
        
        assert!(registry
            .test_generic_implementation(
                "GenericStack",
                &[Type::Normie],
                "Container"
            )
            .unwrap());
        
        // Test SortedList[T] implements List only when T implements Comparable
        assert!(registry
            .test_generic_implementation(
                "SortedList",
                &[Type::Tea],  // String implements Comparable
                "List"
            )
            .unwrap());
            
        assert!(registry
            .test_generic_implementation(
                "SortedList",
                &[Type::Normie],  // Int implements Comparable
                "List"
            )
            .unwrap());
            
        // Create a type that doesn't implement Comparable
        let non_comparable_type = Type::Struct("NonComparable".to_string(), vec![]);
        registry.register_implementation(non_comparable_type.clone(), "Container".to_string());
        
        // SortedList[NonComparable] should not implement List since NonComparable 
        // doesn't implement Comparable
        assert!(!registry
            .test_generic_implementation(
                "SortedList",
                &[non_comparable_type],
                "List"
            )
            .unwrap());
    }
    
    #[test]
    fn test_direct_check_for_generic_types() {
        // Create a new registry and populate it with defaults
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        // Create a struct type with concrete type arguments
        let generic_stack_tea = Type::Struct(
            "GenericStack".to_string(),
            vec![Type::Tea]
        );
        
        // Direct check should work for generic types with concrete type args
        assert!(registry
            .check_implementation(&generic_stack_tea, "Container")
            .unwrap());
            
        // Create a SortedList with a comparable type (Tea/String)
        let sorted_list_tea = Type::Struct(
            "SortedList".to_string(),
            vec![Type::Tea]
        );
        
        // Check should succeed as Tea implements Comparable
        assert!(registry
            .check_implementation(&sorted_list_tea, "List")
            .unwrap());
            
        // Create a SortedList with a non-comparable type
        let non_comparable = Type::Struct("NonComparable".to_string(), vec![]);
        registry.register_implementation(non_comparable.clone(), "Container".to_string());
        
        let sorted_list_non_comparable = Type::Struct(
            "SortedList".to_string(),
            vec![non_comparable]
        );
        
        // Check should fail as NonComparable doesn't implement Comparable
        assert!(!registry
            .check_implementation(&sorted_list_non_comparable, "List")
            .unwrap());
    }
    
    #[test]
    fn test_dictionary_map_implementation() {
        // Create a new registry and populate it with defaults
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        // Dictionary[String, Int] implements Map because String implements Comparable
        let dict_string_int = Type::Struct(
            "Dictionary".to_string(),
            vec![Type::Tea, Type::Normie]
        );
        
        assert!(registry
            .check_implementation(&dict_string_int, "Map")
            .unwrap());
            
        // Dictionary[NonComparable, Int] should not implement Map
        let non_comparable = Type::Struct("NonComparable".to_string(), vec![]);
        registry.register_implementation(non_comparable.clone(), "Container".to_string());
        
        let dict_non_comparable_int = Type::Struct(
            "Dictionary".to_string(),
            vec![non_comparable, Type::Normie]
        );
        
        assert!(!registry
            .check_implementation(&dict_non_comparable_int, "Map")
            .unwrap());
    }
}