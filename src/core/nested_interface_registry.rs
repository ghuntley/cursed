//! Nested Interface Constraint Registry
//!
//! This module extends the interface registry system to properly handle
//! nested interface constraints in generic types.

use std::collections::{HashMap, HashSet};
use crate::core::interface_registry::{InterfaceRegistry, GenericInterfaceImpl};
use crate::core::type_checker::Type;
use crate::error::Error;
use tracing::{debug, warn, error, info, trace, instrument};

/// Represents a nested constraint relationship in generic types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NestedConstraint {
    /// The outer generic type name
    pub outer_type: String,
    
    /// The outer type parameter that has the constraint
    pub outer_param: String,
    
    /// The inner generic type that's used as argument
    pub inner_type: String,
    
    /// The inner type's parameters
    pub inner_params: Vec<String>,
    
    /// The interface that must be implemented
    pub interface: String,
}

/// Extension trait for InterfaceRegistry to support nested constraints
pub trait NestedInterfaceRegistry {
    /// Register a nested constraint relationship
    fn register_nested_constraint(&mut self, constraint: NestedConstraint);
    
    /// Check if a nested generic type satisfies constraints
    fn check_nested_implementation(
        &self,
        outer_type: &str,
        outer_param: &str,
        inner_type: &Type,
        interface: &str
    ) -> Result<bool, Error>;
    
    /// Get all nested constraints for a given type
    fn get_nested_constraints(&self, type_name: &str) -> Vec<NestedConstraint>;
}

/// Enhanced interface registry that supports nested constraints
#[derive(Debug, Default, Clone)]
pub struct EnhancedInterfaceRegistry {
    /// The base interface registry
    pub base_registry: InterfaceRegistry,
    
    /// Maps from outer type name to nested constraints
    nested_constraints: HashMap<String, Vec<NestedConstraint>>,
}

impl EnhancedInterfaceRegistry {
    /// Create a new empty enhanced interface registry
    pub fn new() -> Self {
        Self {
            base_registry: InterfaceRegistry::new(),
            nested_constraints: HashMap::new(),
        }
    }
    
    /// Create a new registry with default implementations
    pub fn new_with_defaults() -> Self {
        let mut registry = Self::new();
        registry.base_registry.populate_with_defaults();
        registry.populate_nested_defaults();
        registry
    }
    
    /// Populate with default nested constraint relationships
    fn populate_nested_defaults(&mut self) {
        // Register nested constraints for common container types
        
        // NestedList[T, Container[E]] where E must be Comparable
        self.register_nested_constraint(NestedConstraint {
            outer_type: "NestedList".to_string(),
            outer_param: "U".to_string(),
            inner_type: "Container".to_string(),
            inner_params: vec!["E".to_string()],
            interface: "Comparable".to_string(),
        });
        
        // KeyedContainer[K, V] where if V is a Container[E], E must implement Comparable
        self.register_nested_constraint(NestedConstraint {
            outer_type: "KeyedContainer".to_string(),
            outer_param: "V".to_string(),
            inner_type: "Container".to_string(),
            inner_params: vec!["E".to_string()],
            interface: "Comparable".to_string(),
        });
        
        debug!("Populated enhanced registry with default nested constraints");
    }
}

impl NestedInterfaceRegistry for EnhancedInterfaceRegistry {
    /// Register a nested constraint relationship
    #[instrument(level = "debug")]
    fn register_nested_constraint(&mut self, constraint: NestedConstraint) {
        self.nested_constraints
            .entry(constraint.outer_type.clone())
            .or_insert_with(Vec::new)
            .push(constraint.clone());
            
        debug!(
            outer_type = constraint.outer_type,
            outer_param = constraint.outer_param,
            inner_type = constraint.inner_type,
            interface = constraint.interface,
            "Registered nested constraint"
        );
    }
    
    /// Check if a nested generic type satisfies constraints
    /// 
    /// This performs deep constraint checking for nested generic types
    #[instrument(skip(self, inner_type), level = "debug")]
    fn check_nested_implementation(
        &self,
        outer_type: &str,
        outer_param: &str,
        inner_type: &Type,
        interface: &str
    ) -> Result<bool, Error> {
        // First check if we have any nested constraints for this outer type
        if let Some(constraints) = self.nested_constraints.get(outer_type) {
            // Find constraints that match our parameters
            for constraint in constraints {
                if constraint.outer_param == outer_param && constraint.interface == interface {
                    // This is a relevant constraint, check if inner_type matches
                    if let Type::Struct(inner_name, type_args) = inner_type {
                        if *inner_name == constraint.inner_type {
                            // We have a match - now check the nested constraint
                            debug!("Found matching nested constraint for {}", outer_type);
                            
                            // For each type parameter in the inner type, it must satisfy the interface
                            // Extract the inner type arguments as concrete types
                            let concrete_args: Vec<Type> = type_args.iter()
                                .map(|t| (**t).clone())
                                .collect();
                                
                            // Check each inner type argument against the interface
                            for arg in concrete_args {
                                if !self.base_registry.check_implementation(&arg, interface)? {
                                    debug!(
                                        "Inner type argument {:?} does not satisfy interface {}",
                                        arg, interface
                                    );
                                    return Ok(false);
                                }
                            }
                            
                            // All inner type arguments satisfy the constraint
                            debug!("All nested constraints satisfied");
                            return Ok(true);
                        }
                    }
                }
            }
        }
        
        // If we get here, either there were no nested constraints or none matched
        // Delegate to the base registry for regular constraint checking
        self.base_registry.check_implementation(inner_type, interface)
    }
    
    /// Get all nested constraints for a given type
    fn get_nested_constraints(&self, type_name: &str) -> Vec<NestedConstraint> {
        self.nested_constraints
            .get(type_name)
            .cloned()
            .unwrap_or_default()
    }
}

/// Extension trait for the standard InterfaceRegistry to handle nested constraints
impl NestedInterfaceRegistry for InterfaceRegistry {
    /// Register a nested constraint relationship
    /// 
    /// Note: This is a stub implementation that logs a warning since the base
    /// InterfaceRegistry doesn't support nested constraints directly.
    fn register_nested_constraint(&mut self, constraint: NestedConstraint) {
        warn!(
            "Base InterfaceRegistry doesn't support nested constraints. Use EnhancedInterfaceRegistry instead."
        );
    }
    
    /// Check if a nested generic type satisfies constraints
    /// 
    /// This delegates to standard constraint checking since the base registry
    /// doesn't understand nested constraints.
    fn check_nested_implementation(
        &self,
        _outer_type: &str,
        _outer_param: &str,
        inner_type: &Type,
        interface: &str
    ) -> Result<bool, Error> {
        // Just check if the inner type implements the interface directly
        self.check_implementation(inner_type, interface)
    }
    
    /// Get all nested constraints for a given type
    fn get_nested_constraints(&self, _type_name: &str) -> Vec<NestedConstraint> {
        // Base registry doesn't support nested constraints
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_nested_constraint_registration() {
        let mut registry = EnhancedInterfaceRegistry::new();
        
        // Register a nested constraint
        let constraint = NestedConstraint {
            outer_type: "Container".to_string(),
            outer_param: "T".to_string(),
            inner_type: "List".to_string(),
            inner_params: vec!["E".to_string()],
            interface: "Comparable".to_string(),
        };
        
        registry.register_nested_constraint(constraint.clone());
        
        // Verify it was registered
        let constraints = registry.get_nested_constraints("Container");
        assert_eq!(constraints.len(), 1);
        assert_eq!(constraints[0], constraint);
    }
    
    #[test]
    fn test_nested_constraint_checking() {
        let mut registry = EnhancedInterfaceRegistry::new_with_defaults();
        
        // Register a nested constraint for a Container of Lists
        let constraint = NestedConstraint {
            outer_type: "Container".to_string(),
            outer_param: "T".to_string(),
            inner_type: "List".to_string(),
            inner_params: vec!["E".to_string()],
            interface: "Comparable".to_string(),
        };
        
        registry.register_nested_constraint(constraint);
        
        // Create test types
        let list_of_int = Type::Struct(
            "List".to_string(),
            vec![Box::new(Type::Normie)]
        );
        
        let list_of_non_comparable = Type::Struct(
            "List".to_string(),
            vec![Box::new(Type::Struct("NonComparable".to_string(), vec![]))]
        );
        
        // Int implements Comparable, so this should pass
        assert!(registry
            .check_nested_implementation("Container", "T", &list_of_int, "Comparable")
            .unwrap());
            
        // NonComparable doesn't implement Comparable, so this should fail
        assert!(!registry
            .check_nested_implementation("Container", "T", &list_of_non_comparable, "Comparable")
            .unwrap());
    }
    
    #[test]
    fn test_deep_nested_constraints() {
        let mut registry = EnhancedInterfaceRegistry::new_with_defaults();
        
        // Register constraints for three-level nesting
        // TripleContainer[A, B, C] requires that if A is a Container[E],
        // then E must be Comparable
        let constraint = NestedConstraint {
            outer_type: "TripleContainer".to_string(),
            outer_param: "A".to_string(),
            inner_type: "Container".to_string(),
            inner_params: vec!["E".to_string()],
            interface: "Comparable".to_string(),
        };
        
        registry.register_nested_constraint(constraint);
        
        // Create test types
        let container_of_int = Type::Struct(
            "Container".to_string(),
            vec![Box::new(Type::Normie)]
        );
        
        let container_of_non_comparable = Type::Struct(
            "Container".to_string(),
            vec![Box::new(Type::Struct("NonComparable".to_string(), vec![]))]
        );
        
        // Container of Int should satisfy the constraint
        assert!(registry
            .check_nested_implementation("TripleContainer", "A", &container_of_int, "Comparable")
            .unwrap());
            
        // Container of NonComparable should not satisfy the constraint
        assert!(!registry
            .check_nested_implementation("TripleContainer", "A", &container_of_non_comparable, "Comparable")
            .unwrap());
    }
}