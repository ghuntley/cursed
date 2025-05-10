//! Deep Nested Interface Constraint Registry
//!
//! This module enhances the interface registry system with deep constraint checking
//! capabilities for complex nested generic hierarchies. It builds on the existing
//! nested_interface_registry.rs implementation to provide more comprehensive constraint
//! checking for deeply nested types.

use std::collections::{HashMap, HashSet, VecDeque};
use crate::core::interface_registry::{InterfaceRegistry, GenericInterfaceImpl};
use crate::core::nested_interface_registry::{NestedInterfaceRegistry, NestedConstraint, EnhancedInterfaceRegistry};
use crate::core::type_checker::Type;
use crate::error::Error;
use crate::core::constraint_recovery::{ConstraintRecovery, ConstraintFailureContext};
use tracing::{debug, warn, error, info, trace, instrument};

/// Maximum depth for nested constraint checking to prevent infinite recursion
const MAX_NESTED_DEPTH: usize = 10;

/// Represents a path in the nested type hierarchy
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstraintPath {
    /// The types in the hierarchy, from outermost to innermost
    pub type_path: Vec<String>,
    
    /// The parameters in the hierarchy, from outermost to innermost
    pub param_path: Vec<String>,
    
    /// The interface being checked at each level
    pub interface_path: Vec<String>,
}

impl ConstraintPath {
    /// Create a new empty constraint path
    pub fn new() -> Self {
        Self {
            type_path: Vec::new(),
            param_path: Vec::new(),
            interface_path: Vec::new(),
        }
    }
    
    /// Add a segment to the path
    pub fn add_segment(&mut self, type_name: &str, param_name: &str, interface_name: &str) {
        self.type_path.push(type_name.to_string());
        self.param_path.push(param_name.to_string());
        self.interface_path.push(interface_name.to_string());
    }
    
    /// Get the current depth of the path
    pub fn depth(&self) -> usize {
        self.type_path.len()
    }
    
    /// Format the path as a string
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        
        for i in 0..self.depth() {
            if i > 0 {
                result.push_str(" -> ");
            }
            result.push_str(&format!("{}<{}>: {}", 
                self.type_path[i], 
                self.param_path[i], 
                self.interface_path[i]));
        }
        
        result
    }
}

/// Enhanced interface registry with deep constraint checking capabilities
#[derive(Debug, Clone)]
pub struct DeepNestedInterfaceRegistry {
    /// The underlying enhanced interface registry
    pub enhanced_registry: EnhancedInterfaceRegistry,
    
    /// Cache of deep constraint check results
    deep_constraint_cache: HashMap<(String, String, String, String), bool>,
}

impl DeepNestedInterfaceRegistry {
    /// Create a new empty deep nested interface registry
    pub fn new() -> Self {
        Self {
            enhanced_registry: EnhancedInterfaceRegistry::new(),
            deep_constraint_cache: HashMap::new(),
        }
    }
    
    /// Create a new registry with default implementations
    pub fn new_with_defaults() -> Self {
        let mut registry = Self::new();
        registry.enhanced_registry = EnhancedInterfaceRegistry::new_with_defaults();
        registry.populate_deep_nested_defaults();
        registry
    }
    
    /// Populate with default deep nested constraint relationships
    fn populate_deep_nested_defaults(&mut self) {
        // Register deep nested constraints for common container types
        
        // NestedMap[K, V] where if V is a Container[E], E must be Comparable
        self.register_deep_nested_constraint(
            "NestedMap",
            "V",
            "Container",
            "E",
            "Comparable"
        );
        
        // Triple[A, B, C] where if C is a Pair[X, Y], Y must be Numeric
        self.register_deep_nested_constraint(
            "Triple",
            "C",
            "Pair",
            "Y",
            "Numeric"
        );
        
        // MultiContainer[T] where if T is a Container[List[E]], E must be Comparable
        self.register_deep_multi_level_constraint(
            "MultiContainer",
            "T",
            vec!["Container", "List"],
            vec!["U", "E"],
            "Comparable"
        );
        
        debug!("Populated deep nested registry with default constraints");
    }
    
    /// Register a deep nested constraint relationship
    #[instrument(level = "debug")]
    pub fn register_deep_nested_constraint(
        &mut self,
        outer_type: &str,
        outer_param: &str,
        inner_type: &str,
        inner_param: &str,
        interface: &str
    ) {
        // Create a nested constraint for the first level
        let constraint = NestedConstraint {
            outer_type: outer_type.to_string(),
            outer_param: outer_param.to_string(),
            inner_type: inner_type.to_string(),
            inner_params: vec![inner_param.to_string()],
            interface: interface.to_string(),
        };
        
        // Register with the enhanced registry
        self.enhanced_registry.register_nested_constraint(constraint);
        
        debug!(
            outer_type = outer_type,
            outer_param = outer_param,
            inner_type = inner_type,
            inner_param = inner_param,
            interface = interface,
            "Registered deep nested constraint"
        );
    }
    
    /// Register a multi-level nested constraint relationship
    #[instrument(level = "debug")]
    pub fn register_deep_multi_level_constraint(
        &mut self,
        outer_type: &str,
        outer_param: &str,
        inner_types: Vec<&str>,
        inner_params: Vec<&str>,
        interface: &str
    ) {
        // Validate input
        if inner_types.len() != inner_params.len() {
            warn!("inner_types and inner_params must have the same length");
            return;
        }
        
        // Store the constraint in a more complex format that supports multi-level nesting
        // For now, we'll just record it as a series of simple nested constraints
        
        if inner_types.len() >= 1 {
            // Register the first level constraint
            self.register_deep_nested_constraint(
                outer_type,
                outer_param,
                inner_types[0],
                inner_params[0],
                if inner_types.len() > 1 { "DependentConstraint" } else { interface }
            );
        }
        
        // Register intermediate constraints
        for i in 1..inner_types.len() {
            let prev_type = inner_types[i-1];
            let prev_param = inner_params[i-1];
            let curr_type = inner_types[i];
            let curr_param = inner_params[i];
            
            self.register_deep_nested_constraint(
                prev_type,
                prev_param,
                curr_type,
                curr_param,
                if i == inner_types.len() - 1 { interface } else { "DependentConstraint" }
            );
        }
        
        debug!(
            outer_type = outer_type,
            outer_param = outer_param,
            inner_types = ?inner_types,
            inner_params = ?inner_params,
            interface = interface,
            "Registered multi-level nested constraint"
        );
    }
    
    /// Check a deep nested constraint with path tracking
    /// 
    /// This performs deep constraint checking for nested generic types with
    /// detailed path information for error reporting.
    #[instrument(skip(self, inner_type, path), level = "debug")]
    pub fn check_deep_nested_implementation(
        &self,
        outer_type: &str,
        outer_param: &str,
        inner_type: &Type,
        interface: &str,
        path: &mut ConstraintPath
    ) -> Result<bool, Error> {
        // Check for excessive recursion
        if path.depth() >= MAX_NESTED_DEPTH {
            warn!("Maximum nested constraint depth exceeded: {}", path.depth());
            return Ok(false);
        }
        
        // Add the current segment to the path
        path.add_segment(outer_type, outer_param, interface);
        debug!("Checking constraint at path: {}", path.to_string());
        
        // Check cache for already evaluated constraints
        let cache_key = (
            outer_type.to_string(),
            outer_param.to_string(),
            format!("{:?}", inner_type),
            interface.to_string()
        );
        
        if let Some(&result) = self.deep_constraint_cache.get(&cache_key) {
            debug!("Found cached result for deep constraint check: {}", result);
            return Ok(result);
        }
        
        // First check if we have any nested constraints for this outer type
        if let Some(constraints) = self.enhanced_registry.get_nested_constraints(outer_type) {
            // Find constraints that match our parameters
            for constraint in constraints {
                if constraint.outer_param == outer_param && 
                  (constraint.interface == interface || constraint.interface == "DependentConstraint") {
                    // This is a relevant constraint, check if inner_type matches
                    if let Type::Struct(inner_name, type_args) = inner_type {
                        if *inner_name == constraint.inner_type {
                            // We have a match - now check the nested constraint
                            debug!("Found matching nested constraint for {}", outer_type);
                            
                            // Extract the inner type arguments as concrete types
                            let concrete_args: Vec<Type> = type_args.iter()
                                .map(|t| (**t).clone())
                                .collect();
                                
                            // For DependentConstraint, we need to follow the chain
                            if constraint.interface == "DependentConstraint" {
                                debug!("Following dependent constraint chain");
                                
                                // For each param in the inner type, check if it has further constraints
                                for (i, param_name) in constraint.inner_params.iter().enumerate() {
                                    if i < concrete_args.len() {
                                        // Recursively check this parameter against its constraints
                                        if !self.check_deep_nested_implementation(
                                            &constraint.inner_type,
                                            param_name,
                                            &concrete_args[i],
                                            interface,
                                            path
                                        )? {
                                            debug!("Deep constraint failed at intermediate level");
                                            return Ok(false);
                                        }
                                    }
                                }
                                
                                // All dependent constraints passed
                                debug!("All dependent constraints satisfied in chain");
                                return Ok(true);
                            } else {
                                // Direct constraint check against the interface
                                // Check each inner type argument against the interface
                                for (i, arg) in concrete_args.iter().enumerate() {
                                    if i < constraint.inner_params.len() {
                                        if !self.enhanced_registry.base_registry.check_implementation(arg, interface)? {
                                            debug!(
                                                "Inner type argument {:?} does not satisfy interface {}",
                                                arg, interface
                                            );
                                            return Ok(false);
                                        }
                                    }
                                }
                                
                                // All inner type arguments satisfy the constraint
                                debug!("All nested constraints satisfied");
                                
                                // Cache the result
                                let mut registry = self.clone();
                                registry.deep_constraint_cache.insert(cache_key, true);
                                
                                return Ok(true);
                            }
                        }
                    }
                }
            }
        }
        
        // If we get here, either there were no nested constraints or none matched
        // Delegate to the enhanced registry for regular constraint checking
        let result = self.enhanced_registry.check_nested_implementation(
            outer_type,
            outer_param,
            inner_type,
            interface
        )?;
        
        // Cache the result
        let mut registry = self.clone();
        registry.deep_constraint_cache.insert(cache_key, result);
        
        Ok(result)
    }
    
    /// Create a comprehensive error with detailed path information
    pub fn create_deep_constraint_error(
        &self,
        failed_type: &Type,
        interface: &str,
        path: &ConstraintPath
    ) -> Error {
        // Get the basic error from the constraint recovery system
        let basic_context = self.enhanced_registry.base_registry.create_recovery_context(failed_type, interface);
        
        // Enhance the error message with the constraint path
        let mut enhanced_message = format!(
            "Deep nested constraint failure:\n{}\n\n{}",
            path.to_string(),
            basic_context.to_error_message()
        );
        
        // Add recommendations specific to deep nested constraints
        enhanced_message.push_str("\n\nFor nested container types, ensure that:\n");
        enhanced_message.push_str("1. The innermost type satisfies the required interface\n");
        enhanced_message.push_str("2. The nesting pattern matches the expected constraint path\n");
        enhanced_message.push_str("3. All intermediate container types are correctly parameterized\n");
        
        Error::new("CNST03", &enhanced_message, None)
    }
    
    /// Check a complex deep nested constraint with multi-level nesting
    /// 
    /// This is a convenience method that initializes a fresh path and performs the check
    #[instrument(skip(self, inner_type), level = "debug")]
    pub fn check_complex_nested_constraint(
        &self,
        outer_type: &str,
        outer_param: &str,
        inner_type: &Type,
        interface: &str
    ) -> Result<bool, Error> {
        let mut path = ConstraintPath::new();
        self.check_deep_nested_implementation(outer_type, outer_param, inner_type, interface, &mut path)
    }
}

/// Extension trait for InterfaceRegistry to support deep nested constraints
pub trait DeepNestedInterfaceChecking {
    /// Check a complex nested constraint with multi-level nesting
    fn check_complex_nested_constraint(
        &self,
        outer_type: &str,
        outer_param: &str,
        inner_type: &Type,
        interface: &str
    ) -> Result<bool, Error>;
    
    /// Create an enhanced registry capable of deep nested constraint checking
    fn to_deep_nested_registry(&self) -> DeepNestedInterfaceRegistry;
}

/// Implementation of the deep nested interface checking for the base registry
impl DeepNestedInterfaceChecking for InterfaceRegistry {
    fn check_complex_nested_constraint(
        &self,
        outer_type: &str,
        outer_param: &str,
        inner_type: &Type,
        interface: &str
    ) -> Result<bool, Error> {
        // Create a deep nested registry and delegate to it
        let registry = self.to_deep_nested_registry();
        registry.check_complex_nested_constraint(outer_type, outer_param, inner_type, interface)
    }
    
    fn to_deep_nested_registry(&self) -> DeepNestedInterfaceRegistry {
        // Create a new deep nested registry with this registry as the base
        let mut enhanced = EnhancedInterfaceRegistry::new();
        enhanced.base_registry = self.clone();
        
        let mut deep = DeepNestedInterfaceRegistry::new();
        deep.enhanced_registry = enhanced;
        
        deep
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[path = "../../tests/common.rs"]
    mod common;
    
    #[test]
    fn test_deep_nested_constraint_registration() {
        common::tracing::setup();
        
        let mut registry = DeepNestedInterfaceRegistry::new();
        
        // Register a deep nested constraint
        registry.register_deep_nested_constraint(
            "OuterContainer",
            "T",
            "InnerContainer",
            "E",
            "Comparable"
        );
        
        // Verify it was registered via the enhanced registry
        let constraints = registry.enhanced_registry.get_nested_constraints("OuterContainer");
        assert_eq!(constraints.len(), 1);
        assert_eq!(constraints[0].outer_type, "OuterContainer");
        assert_eq!(constraints[0].inner_type, "InnerContainer");
        assert_eq!(constraints[0].interface, "Comparable");
    }
    
    #[test]
    fn test_multi_level_constraint_registration() {
        common::tracing::setup();
        
        let mut registry = DeepNestedInterfaceRegistry::new();
        
        // Register a multi-level constraint
        registry.register_deep_multi_level_constraint(
            "Triple",
            "T",
            vec!["Pair", "Box"],
            vec!["U", "V"],
            "Numeric"
        );
        
        // Verify first level was registered
        let constraints = registry.enhanced_registry.get_nested_constraints("Triple");
        assert_eq!(constraints.len(), 1);
        assert_eq!(constraints[0].outer_type, "Triple");
        assert_eq!(constraints[0].inner_type, "Pair");
        assert_eq!(constraints[0].interface, "DependentConstraint");
        
        // Verify second level was registered
        let constraints = registry.enhanced_registry.get_nested_constraints("Pair");
        assert_eq!(constraints.len(), 1);
        assert_eq!(constraints[0].outer_type, "Pair");
        assert_eq!(constraints[0].inner_type, "Box");
        assert_eq!(constraints[0].interface, "Numeric");
    }
    
    #[test]
    fn test_constraint_path_tracking() {
        common::tracing::setup();
        
        // Create a path and add segments
        let mut path = ConstraintPath::new();
        path.add_segment("Triple", "T", "Comparable");
        path.add_segment("Pair", "U", "DependentConstraint");
        path.add_segment("Box", "V", "Numeric");
        
        // Verify path properties
        assert_eq!(path.depth(), 3);
        assert_eq!(path.type_path, vec!["Triple", "Pair", "Box"]);
        assert_eq!(path.param_path, vec!["T", "U", "V"]);
        
        // Verify string representation
        let path_str = path.to_string();
        assert!(path_str.contains("Triple<T>"));
        assert!(path_str.contains("Pair<U>"));
        assert!(path_str.contains("Box<V>"));
    }
    
    #[test]
    fn test_deep_constraint_checking() {
        common::tracing::setup();
        
        let mut registry = DeepNestedInterfaceRegistry::new_with_defaults();
        
        // Register deep nested constraints
        registry.register_deep_nested_constraint(
            "OuterContainer",
            "T",
            "InnerContainer",
            "E",
            "Comparable"
        );
        
        // Create test types
        // InnerContainer[Int] - where Int implements Comparable
        let inner_container_int = Type::Struct(
            "InnerContainer".to_string(),
            vec![Type::Normie]
        );
        
        // OuterContainer[InnerContainer[Int]]
        let outer_container = Type::Struct(
            "OuterContainer".to_string(),
            vec![inner_container_int.clone()]
        );
        
        // Check the constraint - should pass because Int implements Comparable
        let result = registry.check_complex_nested_constraint(
            "OuterContainer",
            "T",
            &inner_container_int,
            "Comparable"
        );
        assert!(result.unwrap());
        
        // InnerContainer[NonComparable]
        let non_comparable = Type::Struct("NonComparable".to_string(), vec![]);
        let inner_container_non_comparable = Type::Struct(
            "InnerContainer".to_string(),
            vec![non_comparable]
        );
        
        // Check the constraint - should fail because NonComparable doesn't implement Comparable
        let result = registry.check_complex_nested_constraint(
            "OuterContainer",
            "T",
            &inner_container_non_comparable,
            "Comparable"
        );
        assert!(!result.unwrap());
    }
    
    #[test]
    fn test_multi_level_constraint_checking() {
        common::tracing::setup();
        
        let mut registry = DeepNestedInterfaceRegistry::new_with_defaults();
        
        // Register a three-level nested constraint
        registry.register_deep_multi_level_constraint(
            "Triple",
            "T",
            vec!["Pair", "Box"],
            vec!["U", "V"],
            "Numeric"
        );
        
        // Create test types
        // Box[Int] - where Int implements Numeric
        let box_int = Type::Struct(
            "Box".to_string(),
            vec![Type::Normie]
        );
        
        // Pair[String, Box[Int]]
        let pair = Type::Struct(
            "Pair".to_string(),
            vec![Type::Tea, box_int.clone()]
        );
        
        // Check the first step in the chain
        let mut path = ConstraintPath::new();
        let result = registry.check_deep_nested_implementation(
            "Triple",
            "T",
            &pair,
            "Numeric",
            &mut path
        );
        assert!(result.unwrap());
        
        // Box[NonNumeric]
        let non_numeric = Type::Struct("NonNumeric".to_string(), vec![]);
        let box_non_numeric = Type::Struct(
            "Box".to_string(),
            vec![non_numeric]
        );
        
        // Pair[String, Box[NonNumeric]]
        let pair_with_non_numeric = Type::Struct(
            "Pair".to_string(),
            vec![Type::Tea, box_non_numeric]
        );
        
        // Check the constraint - should fail because NonNumeric doesn't implement Numeric
        let mut path = ConstraintPath::new();
        let result = registry.check_deep_nested_implementation(
            "Triple",
            "T",
            &pair_with_non_numeric,
            "Numeric",
            &mut path
        );
        assert!(!result.unwrap());
        
        // Verify path has all three segments
        assert_eq!(path.depth(), 3);
    }
    
    #[test]
    fn test_error_creation_with_path() {
        common::tracing::setup();
        
        let registry = DeepNestedInterfaceRegistry::new_with_defaults();
        
        // Create a constraint path
        let mut path = ConstraintPath::new();
        path.add_segment("Triple", "T", "Comparable");
        path.add_segment("Pair", "U", "DependentConstraint");
        path.add_segment("Box", "V", "Numeric");
        
        // Create a type that doesn't implement Numeric
        let non_numeric = Type::Struct("NonNumeric".to_string(), vec![]);
        
        // Create an error with the path
        let error = registry.create_deep_constraint_error(
            &non_numeric,
            "Numeric",
            &path
        );
        
        // Verify error contains path information
        let message = error.message();
        assert!(message.contains("Deep nested constraint failure"));
        assert!(message.contains("Triple<T>"));
        assert!(message.contains("Pair<U>"));
        assert!(message.contains("Box<V>"));
        assert!(message.contains("NonNumeric"));
        assert!(message.contains("Numeric"));
    }
    
    #[test]
    fn test_extension_trait() {
        common::tracing::setup();
        
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        // Convert to deep nested registry
        let deep_registry = registry.to_deep_nested_registry();
        
        // Register a constraint
        let mut deep_registry = deep_registry.clone();
        deep_registry.register_deep_nested_constraint(
            "Container",
            "T",
            "List",
            "E",
            "Comparable"
        );
        
        // Check a constraint directly through the extension trait
        let list_of_int = Type::Struct(
            "List".to_string(),
            vec![Type::Normie]
        );
        
        let result = registry.check_complex_nested_constraint(
            "Container",
            "T",
            &list_of_int,
            "Comparable"
        );
        
        // Should pass due to default registry population
        assert!(result.unwrap());
    }
}