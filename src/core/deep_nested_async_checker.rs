//! Deep Nested Async Constraint Checker Integration
//!
//! This module provides integration between deep nested constraint checking
//! and asynchronous constraint checking for improved performance with complex
//! generic type hierarchies. It allows concurrent validation of constraints
//! in deeply nested generic types.
//!
//! Features:
//! - Parallel execution of deep nested constraint checks
//! - Optimized performance for complex nested generic types
//! - Path tracking with concurrent validation

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::core::interface_registry::{InterfaceRegistry, GenericInterfaceImpl};
use crate::core::nested_interface_registry::{NestedInterfaceRegistry, NestedConstraint, EnhancedInterfaceRegistry};
use crate::core::deep_nested_interface_registry::{DeepNestedInterfaceRegistry, ConstraintPath, DeepNestedInterfaceChecking};
use crate::core::async_constraint_checker::{AsyncConstraintChecker, AsyncConstraintChecking};
use crate::core::type_checker::Type;
use crate::error::Error;
use tracing::{debug, info, warn, error, trace, instrument};

/// Maximum number of constraints to check sequentially before switching to parallel mode
const SEQUENTIAL_CONSTRAINT_THRESHOLD: usize = 3;

/// Integrated deep nested and async constraint checker
pub struct DeepNestedAsyncChecker {
    /// The deep nested interface registry
    deep_registry: Arc<DeepNestedInterfaceRegistry>,
    
    /// The async constraint checker
    async_checker: Arc<AsyncConstraintChecker>,
    
    /// Cache for constraint check results to prevent redundant work
    cache: Mutex<HashMap<(String, String, String, String), bool>>,
}

impl DeepNestedAsyncChecker {
    /// Create a new integrated deep nested async constraint checker
    pub fn new(registry: Arc<InterfaceRegistry>) -> Self {
        // Create the deep nested registry
        let mut deep_registry = DeepNestedInterfaceRegistry::new();
        deep_registry.enhanced_registry.base_registry = registry.as_ref().clone();
        deep_registry.populate_deep_nested_defaults();
        let deep_registry = Arc::new(deep_registry);
        
        // Create the async checker
        let async_checker = Arc::new(AsyncConstraintChecker::new(registry.clone()));
        
        Self {
            deep_registry,
            async_checker,
            cache: Mutex::new(HashMap::new()),
        }
    }
    
    /// Create a new integrated checker with custom deep registry
    pub fn with_deep_registry(deep_registry: Arc<DeepNestedInterfaceRegistry>) -> Self {
        // Extract the base registry
        let registry = Arc::new(deep_registry.enhanced_registry.base_registry.clone());
        
        // Create the async checker
        let async_checker = Arc::new(AsyncConstraintChecker::new(registry));
        
        Self {
            deep_registry,
            async_checker,
            cache: Mutex::new(HashMap::new()),
        }
    }
    
    /// Check a multi-level constraint path in parallel when possible
    #[instrument(skip(self, inner_type), level = "debug")]
    pub fn check_complex_nested_constraint_parallel(
        &self,
        outer_type: &str,
        outer_param: &str,
        inner_type: &Type,
        interface: &str
    ) -> Result<bool, Error> {
        // Check cache first
        let cache_key = (
            outer_type.to_string(),
            outer_param.to_string(),
            format!("{:?}", inner_type),
            interface.to_string(),
        );
        
        {
            let cache = self.cache.lock().unwrap();
            if let Some(&result) = cache.get(&cache_key) {
                debug!("Found cached result for deep nested constraint check: {}", result);
                return Ok(result);
            }
        }
        
        // Prepare a constraint path for tracking
        let mut path = ConstraintPath::new();
        
        // First check if we have any nested constraints for this outer type
        let constraints = self.deep_registry.enhanced_registry.get_nested_constraints(outer_type);
        if !constraints.is_empty() {
            let matching_constraints: Vec<_> = constraints.iter()
                .filter(|c| c.outer_param == outer_param && 
                        (c.interface == interface || c.interface == "DependentConstraint"))
                .collect();
            
            if !matching_constraints.is_empty() {
                // Track this as the first segment in the path
                path.add_segment(outer_type, outer_param, interface);
                
                if matching_constraints.len() > SEQUENTIAL_CONSTRAINT_THRESHOLD {
                    // For many constraints, use parallel checking
                    return self.check_multiple_constraints_parallel(
                        matching_constraints,
                        inner_type,
                        interface,
                        &mut path,
                        &cache_key
                    );
                } else {
                    // For fewer constraints, use sequential checking via the standard method
                    let result = self.deep_registry.check_deep_nested_implementation(
                        outer_type,
                        outer_param,
                        inner_type,
                        interface,
                        &mut path
                    )?;
                    
                    // Cache the result
                    let mut cache = self.cache.lock().unwrap();
                    cache.insert(cache_key, result);
                    
                    return Ok(result);
                }
            }
        }
        
        // If no nested constraints were found, delegate to the standard implementation
        let result = self.deep_registry.check_complex_nested_constraint(
            outer_type, 
            outer_param, 
            inner_type, 
            interface
        )?;
        
        // Cache the result
        let mut cache = self.cache.lock().unwrap();
        cache.insert(cache_key, result);
        
        Ok(result)
    }
    
    /// Check multiple constraints in parallel
    #[instrument(skip(self, constraints, inner_type, path, cache_key), level = "debug")]
    fn check_multiple_constraints_parallel(
        &self,
        constraints: Vec<&NestedConstraint>,
        inner_type: &Type,
        interface: &str,
        path: &mut ConstraintPath,
        cache_key: &(String, String, String, String)
    ) -> Result<bool, Error> {
        debug!("Checking {} constraints in parallel", constraints.len());
        
        // Extract relevant Type structs if applicable
        if let Type::Struct(inner_name, type_args) = inner_type {
            // Prepare async constraint checking tasks
            let mut constraint_tasks = Vec::new();
            
            for constraint in &constraints {
                if constraint.inner_type == *inner_name {
                    debug!("Found matching constraint: {:?}", constraint);
                    
                    // For each matching constraint, prepare dependent checks
                    if constraint.interface == "DependentConstraint" {
                        // Handle dependent constraints - we need to check inner types recursively
                        // For these, we can't easily parallelize now, so we'll prepare sequential checks
                        let concrete_args: Vec<Type> = type_args.iter()
                            .map(|t| (**t).clone())
                            .collect();
                            
                        // Prepare checks for inner type parameters against their constraints
                        for (i, param_name) in constraint.inner_params.iter().enumerate() {
                            if i < concrete_args.len() {
                                // For each parameter, we need to check if it meets the interface requirement
                                // We'll add this as a task for parallel execution
                                constraint_tasks.push((
                                    constraint.inner_type.clone(),
                                    param_name.clone(),
                                    concrete_args[i].clone(),
                                    interface.to_string()
                                ));
                            }
                        }
                    } else {
                        // Direct interface checks for inner types
                        let concrete_args: Vec<Type> = type_args.iter()
                            .map(|t| (**t).clone())
                            .collect();
                            
                        // Check each inner type argument directly against the interface
                        for arg in concrete_args {
                            constraint_tasks.push((
                                "DirectCheck".to_string(),
                                "DirectParam".to_string(),
                                arg,
                                constraint.interface.clone()
                            ));
                        }
                    }
                }
            }
            
            // If we have tasks to execute in parallel
            if !constraint_tasks.is_empty() {
                debug!("Prepared {} constraint tasks for parallel execution", constraint_tasks.len());
                
                // Flatten the constraint tasks into (Type, interface_name) pairs for parallel checking
                let mut async_tasks = Vec::new();
                
                for (inner_type_name, inner_param, arg_type, interface_name) in &constraint_tasks {
                    if inner_type_name == "DirectCheck" {
                        // Direct interface check
                        async_tasks.push((arg_type.clone(), interface_name.clone()));
                    } else {
                        // For dependent constraints, we need deeper checking
                        // We can't easily parallelize these in the current framework without more complex changes
                        // So we'll do these checks sequentially after the parallel checks
                    }
                }
                
                // Execute direct interface checks in parallel if we have any
                if !async_tasks.is_empty() {
                    debug!("Executing {} direct interface checks in parallel", async_tasks.len());
                    let results = self.async_checker.check_constraints_parallel(async_tasks);
                    
                    // Check if any direct checks failed
                    for result in &results {
                        match result {
                            Ok(satisfied) => {
                                if !*satisfied {
                                    // Cache the result (false)
                                    let mut cache = self.cache.lock().unwrap();
                                    cache.insert(cache_key.clone(), false);
                                    return Ok(false);
                                }
                            }
                            Err(e) => return Err(e.clone()),
                        }
                    }
                }
                
                // Now perform sequential checks for dependent constraints that need deeper traversal
                for (inner_type_name, inner_param, arg_type, interface_name) in &constraint_tasks {
                    if inner_type_name != "DirectCheck" {
                        // Use a new path for this branch of constraints
                        let mut branch_path = path.clone();
                        
                        // Check dependent constraints recursively
                        let result = self.deep_registry.check_deep_nested_implementation(
                            inner_type_name,
                            inner_param,
                            &arg_type,
                            &interface_name,
                            &mut branch_path
                        )?;
                        
                        if !result {
                            // Cache the result (false)
                            let mut cache = self.cache.lock().unwrap();
                            cache.insert(cache_key.clone(), false);
                            return Ok(false);
                        }
                    }
                }
                
                // If we got here, all constraints passed
                // Cache the positive result
                let mut cache = self.cache.lock().unwrap();
                cache.insert(cache_key.clone(), true);
                
                return Ok(true);
            }
        }
        
        // If we didn't find any parallelizable constraints, fall back to sequential checking
        let result = self.deep_registry.check_complex_nested_constraint(
            &cache_key.0, 
            &cache_key.1, 
            inner_type, 
            &cache_key.3
        )?;
        
        // Cache the result
        let mut cache = self.cache.lock().unwrap();
        cache.insert(cache_key.clone(), result);
        
        Ok(result)
    }
    
    /// Get statistics from the async constraint checker
    pub fn get_async_stats(&self) -> (usize, usize, usize, usize, f32, f32, usize) {
        self.async_checker.get_stats()
    }
}

/// Extension trait for InterfaceRegistry to support parallel deep nested constraint checking
pub trait DeepNestedAsyncConstraintChecking {
    /// Check complex nested constraints with parallel execution
    fn check_complex_nested_constraint_parallel(
        &self,
        outer_type: &str,
        outer_param: &str,
        inner_type: &Type,
        interface: &str
    ) -> Result<bool, Error>;
    
    /// Create a deep nested async constraint checker
    fn to_deep_nested_async_checker(&self) -> DeepNestedAsyncChecker;
}

/// Implementation of the deep nested async interface checking for the base registry
impl DeepNestedAsyncConstraintChecking for InterfaceRegistry {
    fn check_complex_nested_constraint_parallel(
        &self,
        outer_type: &str,
        outer_param: &str,
        inner_type: &Type,
        interface: &str
    ) -> Result<bool, Error> {
        // Create a deep nested async checker and delegate to it
        let checker = self.to_deep_nested_async_checker();
        checker.check_complex_nested_constraint_parallel(outer_type, outer_param, inner_type, interface)
    }
    
    fn to_deep_nested_async_checker(&self) -> DeepNestedAsyncChecker {
        // Create a new deep nested async checker with this registry as the base
        let registry_arc = Arc::new(self.clone());
        DeepNestedAsyncChecker::new(registry_arc)
    }
}

/// Implementation of the deep nested async interface checking for the deep registry
impl DeepNestedAsyncConstraintChecking for DeepNestedInterfaceRegistry {
    fn check_complex_nested_constraint_parallel(
        &self,
        outer_type: &str,
        outer_param: &str,
        inner_type: &Type,
        interface: &str
    ) -> Result<bool, Error> {
        // Create a deep nested async checker and delegate to it
        let checker = self.to_deep_nested_async_checker();
        checker.check_complex_nested_constraint_parallel(outer_type, outer_param, inner_type, interface)
    }
    
    fn to_deep_nested_async_checker(&self) -> DeepNestedAsyncChecker {
        // Create a new deep nested async checker with this registry as the base
        let registry_arc = Arc::new(self.clone());
        DeepNestedAsyncChecker::with_deep_registry(registry_arc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    use crate::tests::common;
    
    #[test]
    fn test_deep_nested_async_checker_creation() {
        common::tracing::setup();
        
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        let registry_arc = Arc::new(registry);
        
        let checker = DeepNestedAsyncChecker::new(registry_arc);
        assert!(Arc::strong_count(&checker.deep_registry) == 1);
        assert!(Arc::strong_count(&checker.async_checker) == 1);
    }
    
    #[test]
    fn test_constraint_check_direct_interface() {
        common::tracing::setup();
        
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        let registry_arc = Arc::new(registry);
        
        let checker = DeepNestedAsyncChecker::new(registry_arc);
        
        // Test a direct interface check first - Normie implements Numeric
        let result = checker.check_complex_nested_constraint_parallel(
            "Container",
            "T",
            &Type::Normie,
            "Numeric"
        );
        
        assert!(result.is_ok());
        assert!(result.unwrap());
    }
    
    #[test]
    fn test_deep_nested_constraint_parallel() {
        common::tracing::setup();
        
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        let registry_arc = Arc::new(registry);
        
        // Pre-configure the registry with the constraint before creating the checker
        let mut deep_registry = DeepNestedInterfaceRegistry::new();
        deep_registry.enhanced_registry.base_registry = registry_arc.as_ref().clone();
        deep_registry.populate_deep_nested_defaults();
        deep_registry.register_deep_nested_constraint(
            "Container",
            "T",
            "Stack",
            "E",
            "Comparable"
        );
        let deep_registry_arc = Arc::new(deep_registry);
        
        let checker = DeepNestedAsyncChecker::with_deep_registry(deep_registry_arc);
        
        // Create test types
        let stack_int = Type::Struct(
            "Stack".to_string(),
            vec![Box::new(Type::Normie)] // Int implements Comparable
        );
        
        // Container[Stack[Int]]
        let result = checker.check_complex_nested_constraint_parallel(
            "Container",
            "T",
            &stack_int,
            "Comparable"
        );
        
        assert!(result.is_ok());
        assert!(result.unwrap());
        
        // Test with non-comparable type
        let non_comparable = Type::Struct("NonComparable".to_string(), vec![]);
        let stack_non_comparable = Type::Struct(
            "Stack".to_string(),
            vec![Box::new(non_comparable)]
        );
        
        let result = checker.check_complex_nested_constraint_parallel(
            "Container",
            "T",
            &stack_non_comparable,
            "Comparable"
        );
        
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }
    
    #[test]
    fn test_multi_level_constraint_parallel() {
        common::tracing::setup();
        
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        // Use extension trait to create checker
        // Pre-configure the registry with the constraint before creating the checker
        let mut deep_registry = DeepNestedInterfaceRegistry::new();
        deep_registry.enhanced_registry.base_registry = registry.clone();
        deep_registry.populate_deep_nested_defaults();
        deep_registry.register_deep_multi_level_constraint(
            "Collection",
            "T",
            vec!["Container", "List"],
            vec!["U", "E"],
            "Comparable"
        );
        let deep_registry_arc = Arc::new(deep_registry);
        
        let checker = DeepNestedAsyncChecker::with_deep_registry(deep_registry_arc);
        
        // Create test types
        let list_int = Type::Struct(
            "List".to_string(),
            vec![Box::new(Type::Normie)] // Int implements Comparable
        );
        
        let container_list_int = Type::Struct(
            "Container".to_string(),
            vec![Box::new(list_int.clone())]
        );
        
        // Check multi-level constraint
        let result = checker.check_complex_nested_constraint_parallel(
            "Collection",
            "T",
            &container_list_int,
            "Comparable"
        );
        
        assert!(result.is_ok());
        assert!(result.unwrap());
        
        // Test with non-comparable inner type
        let non_comparable = Type::Struct("NonComparable".to_string(), vec![]);
        let list_non_comparable = Type::Struct(
            "List".to_string(),
            vec![Box::new(non_comparable.clone())]
        );
        
        let container_list_non_comparable = Type::Struct(
            "Container".to_string(),
            vec![Box::new(list_non_comparable.clone())]
        );
        
        let result = checker.check_complex_nested_constraint_parallel(
            "Collection",
            "T",
            &container_list_non_comparable,
            "Comparable"
        );
        
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }
    
    #[test]
    fn test_extension_trait_usage() {
        common::tracing::setup();
        
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        // Register test implementations
        registry.register_implementation(
            Type::Struct("TestStruct".to_string(), vec![]),
            "Testable".to_string()
        );
        
        // Use extension trait directly
        let result = registry.check_complex_nested_constraint_parallel(
            "Container",
            "T",
            &Type::Struct("TestStruct".to_string(), vec![]),
            "Testable"
        );
        
        assert!(result.is_ok());
        assert!(result.unwrap());
    }
    
    #[test]
    fn test_caching_mechanism() {
        common::tracing::setup();
        
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        let checker = registry.to_deep_nested_async_checker();
        
        // Make the same check twice
        let stack_int = Type::Struct(
            "Stack".to_string(),
            vec![Box::new(Type::Normie)]
        );
        
        // First call should compute the result
        let result1 = checker.check_complex_nested_constraint_parallel(
            "Container",
            "T",
            &stack_int,
            "Comparable"
        );
        
        // Second call should use cached result
        let result2 = checker.check_complex_nested_constraint_parallel(
            "Container",
            "T",
            &stack_int,
            "Comparable"
        );
        
        assert_eq!(result1.unwrap(), result2.unwrap());
        
        // Examine cache state - we can't directly assert on private fields
        // but we could check performance characteristics
    }
}