//! # Deep Nested Constraint Checker
//!
//! This module extends the constraint checking system to handle deeply nested generic types
//! with multiple constraints. It provides a more thorough constraint checking mechanism that
//! can handle complex cases like `Map<K, List<V>>` where K implements Comparable and V implements
//! Container.
//!
//! Features:
//! - Recursive traversal of nested generic types
//! - Multiple constraint verification at each level
//! - Path tracking for improved error messages
//! - Integration with async constraint checking for performance
//! - Integration with constraint recovery for better error handling

use crate::core::async_constraint_checker::{AsyncConstraintChecker, AsyncConstraintChecking};
use crate::core::constraint_recovery::{ConstraintRecovery, ConstraintFailureContext};
use crate::core::interface_registry::{InterfaceRegistry, GenericInterfaceImpl};
use crate::core::type_checker::Type;
use crate::error::Error;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use tracing::{debug, info, instrument, warn};

/// A path component in a nested generic type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypePathComponent {
    /// The type at this component
    pub type_: Type,
    /// The constraint (if any) at this component
    pub constraint: Option<String>,
    /// The nested position (e.g., parameter index)
    pub position: usize,
    /// The name of the generic parameter
    pub param_name: Option<String>,
}

/// A path through a nested generic type, used for error reporting
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypePath {
    /// The components of the path
    pub components: Vec<TypePathComponent>,
}

impl TypePath {
    /// Create a new empty path
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }
    
    /// Add a component to the path
    pub fn add_component(&mut self, component: TypePathComponent) {
        self.components.push(component);
    }
    
    /// Format the path as a string, showing the constraints
    pub fn format(&self) -> String {
        let mut result = String::new();
        
        for (i, component) in self.components.iter().enumerate() {
            if i > 0 {
                result.push_str(" -> ");
            }
            
            result.push_str(&format!("{:?}", component.type_));
            
            if let Some(constraint) = &component.constraint {
                result.push_str(&format!(" : {}", constraint));
            }
        }
        
        result
    }
}

/// A constraint node in the traversal of a nested generic type
#[derive(Debug, Clone)]
struct ConstraintNode {
    /// The type to check
    type_: Type,
    /// The path to this node
    path: TypePath,
    /// The constraints to check at this node
    constraints: Vec<String>,
    /// The parameter names at this level
    param_names: Vec<String>,
}

/// Result of a deep constraint check
#[derive(Debug, Clone)]
pub struct DeepConstraintResult {
    /// Whether all constraints were satisfied
    pub satisfied: bool,
    /// The path to the deepest failed constraint
    pub failure_path: Option<TypePath>,
    /// The recovery context for the failed constraint
    pub recovery_context: Option<ConstraintFailureContext>,
}

/// Trait for deep nested constraint checking
pub trait DeepNestedConstraintChecking {
    /// Check constraints in a deeply nested generic type
    ///
    /// # Arguments
    ///
    /// * `root_type` - The root generic type to check
    /// * `type_param_constraints` - A map from type parameter names to interface constraints
    ///
    /// # Returns
    ///
    /// A result indicating whether all constraints were satisfied and information about
    /// any failures
    fn check_deep_nested_constraints(
        &self,
        root_type: &Type,
        type_param_constraints: &HashMap<String, Vec<String>>,
    ) -> Result<DeepConstraintResult, Error>;
    
    /// Check all constraints in a generic type and its nested types
    ///
    /// # Arguments
    ///
    /// * `generic_type` - The generic type to check
    /// * `type_params` - The type parameters of the generic type
    /// * `type_args` - The concrete type arguments
    /// * `type_param_constraints` - A map from type parameter names to interface constraints
    ///
    /// # Returns
    ///
    /// A result indicating whether all constraints were satisfied and information about
    /// any failures
    fn check_generic_with_nested_constraints(
        &self,
        generic_type: &str,
        type_params: &[String],
        type_args: &[Type],
        type_param_constraints: &HashMap<String, Vec<String>>,
    ) -> Result<DeepConstraintResult, Error>;
}

/// Implementation of deep nested constraint checking for InterfaceRegistry
impl DeepNestedConstraintChecking for InterfaceRegistry {
    #[instrument(skip(self, root_type, type_param_constraints), level = "debug")]
    fn check_deep_nested_constraints(
        &self,
        root_type: &Type,
        type_param_constraints: &HashMap<String, Vec<String>>,
    ) -> Result<DeepConstraintResult, Error> {
        // Create a shared registry for async checking
        let registry_arc = Arc::new(self.clone());
        let checker = AsyncConstraintChecker::new(registry_arc);
        
        // Create a queue for breadth-first traversal of the type hierarchy
        let mut queue = VecDeque::new();
        
        // Create an initial node for the root type
        // For the root node, we need to determine if it has any direct constraints
        let initial_constraints = match root_type {
            Type::Generic(name, type_args) => {
                // For generic types, we'll process the type arguments which may have constraints
                Vec::new()
            },
            Type::Struct(name, type_args) if !type_args.is_empty() => {
                // For parameterized structs, we'll process the type arguments
                Vec::new()
            },
            _ => Vec::new(),
        };
        
        let root_node = ConstraintNode {
            type_: root_type.clone(),
            path: TypePath::new(),
            constraints: initial_constraints,
            param_names: Vec::new(),
        };
        
        queue.push_back(root_node);
        
        // Track visited types to avoid infinite recursion
        let mut visited = HashSet::new();
        
        // Process nodes in the queue
        while let Some(node) = queue.pop_front() {
            // Skip if we've already visited this exact type
            if visited.contains(&node.type_) {
                continue;
            }
            
            // Mark as visited
            visited.insert(node.type_.clone());
            
            // Check direct constraints on this type
            if !node.constraints.is_empty() {
                let constraint_pairs: Vec<(Type, String)> = node.constraints.iter()
                    .map(|constraint| (node.type_.clone(), constraint.clone()))
                    .collect();
                
                // Check all constraints in parallel
                let results = checker.check_constraints_parallel(constraint_pairs);
                
                // Check if any constraints failed
                for (i, result) in results.iter().enumerate() {
                    match result {
                        Ok(true) => {
                            // Constraint satisfied
                            debug!("Constraint {:?} implements {} satisfied", 
                                  node.type_, node.constraints[i]);
                        },
                        Ok(false) => {
                            // Constraint failed
                            debug!("Constraint {:?} implements {} failed", 
                                  node.type_, node.constraints[i]);
                            
                            // Create a failure context
                            let failure_path = node.path.clone();
                            let recovery_context = self.create_recovery_context(
                                &node.type_, 
                                &node.constraints[i]
                            );
                            
                            return Ok(DeepConstraintResult {
                                satisfied: false,
                                failure_path: Some(failure_path),
                                recovery_context: Some(recovery_context),
                            });
                        },
                        Err(err) => {
                            // Error checking constraint
                            return Err(err.clone());
                        }
                    }
                }
            }
            
            // If this is a generic type, traverse its type arguments
            match &node.type_ {
                Type::Struct(name, type_args) if !type_args.is_empty() => {
                    // Try to find the generic implementation to get parameter names
                    let generic_implementations = self.get_generic_implementations_by_type(name);
                    let generic_impl = generic_implementations.first();
                    
                    if let Some(generic_impl) = generic_impl {
                        let param_names = &generic_impl.type_params;
                        
                        // For each type argument, create a child node
                        for (i, arg) in type_args.iter().enumerate() {
                            let mut child_path = node.path.clone();
                            let param_name = if i < param_names.len() {
                                Some(param_names[i].clone())
                            } else {
                                None
                            };
                            
                            // Check if this parameter has any constraints
                            let constraints = if let Some(name) = &param_name {
                                if let Some(constraints) = type_param_constraints.get(name) {
                                    constraints.clone()
                                } else {
                                    Vec::new()
                                }
                            } else {
                                Vec::new()
                            };
                            
                            // Add a component to the path
                            let constraint_str = if !constraints.is_empty() {
                                Some(constraints[0].clone())
                            } else {
                                None
                            };
                            
                            child_path.add_component(TypePathComponent {
                                type_: *arg.clone(),
                                constraint: constraint_str,
                                position: i,
                                param_name: param_name.clone(),
                            });
                            
                            // Enqueue the child node
                            let child_node = ConstraintNode {
                                type_: *arg.clone(),
                                path: child_path,
                                constraints,
                                param_names: Vec::new(), // Will be populated when processing this node
                            };
                            
                            queue.push_back(child_node);
                        }
                    }
                },
                Type::Generic(name, type_args) => {
                    // Similar to struct, but for explicit generic types
                    let generic_implementations = self.get_generic_implementations_by_type(name);
                    let generic_impl = generic_implementations.first();
                    
                    if let Some(generic_impl) = generic_impl {
                        let param_names = generic_impl.type_params.clone();
                        
                        // For each type argument, create a child node
                        for (i, arg) in type_args.iter().enumerate() {
                            let mut child_path = node.path.clone();
                            let param_name = if i < param_names.len() {
                                Some(param_names[i].clone())
                            } else {
                                None
                            };
                            
                            // Check if this parameter has any constraints
                            let constraints = if let Some(name) = &param_name {
                                if let Some(constraints) = type_param_constraints.get(name) {
                                    constraints.clone()
                                } else {
                                    Vec::new()
                                }
                            } else {
                                Vec::new()
                            };
                            
                            // Add a component to the path
                            let constraint_str = if !constraints.is_empty() {
                                Some(constraints[0].clone())
                            } else {
                                None
                            };
                            
                            child_path.add_component(TypePathComponent {
                                type_: *arg.clone(),
                                constraint: constraint_str,
                                position: i,
                                param_name: param_name.clone(),
                            });
                            
                            // Enqueue the child node - this will recursively check nested generics
                            let child_node = ConstraintNode {
                                type_: *arg.clone(),
                                path: child_path,
                                constraints,
                                param_names: param_names.clone(), // Pass parameter names for nested processing
                            };
                            
                            queue.push_back(child_node);
                        }
                    }
                },
                _ => {
                    // Non-generic types don't need traversal
                }
            }
        }
        
        // If we get here, all constraints were satisfied
        Ok(DeepConstraintResult {
            satisfied: true,
            failure_path: None,
            recovery_context: None,
        })
    }
    
    #[instrument(skip(self, generic_type, type_params, type_args, type_param_constraints), level = "debug")]
    fn check_generic_with_nested_constraints(
        &self,
        generic_type: &str,
        type_params: &[String],
        type_args: &[Type],
        type_param_constraints: &HashMap<String, Vec<String>>,
    ) -> Result<DeepConstraintResult, Error> {
        // Check if we have the right number of type arguments
        if type_args.len() != type_params.len() {
            debug!(
                "Wrong number of type arguments: expected {}, got {}",
                type_params.len(),
                type_args.len()
            );
            return Ok(DeepConstraintResult {
                satisfied: false,
                failure_path: None,
                recovery_context: None,
            });
        }
        
        // Check each type argument against its constraints directly
        for (i, type_arg) in type_args.iter().enumerate() {
            if i < type_params.len() {
                let param_name = &type_params[i];
                if let Some(constraints) = type_param_constraints.get(param_name) {
                    // Check each constraint for this parameter
                    for constraint in constraints {
                        let implementation_result = self.check_implementation(type_arg, constraint);
                        match implementation_result {
                            Ok(true) => {
                                debug!("Type {:?} satisfies constraint {}", type_arg, constraint);
                            },
                            Ok(false) => {
                                debug!("Type {:?} does not satisfy constraint {}", type_arg, constraint);
                                
                                // Create failure context
                                let mut path = TypePath::new();
                                path.add_component(TypePathComponent {
                                    type_: type_arg.clone(),
                                    constraint: Some(constraint.clone()),
                                    position: i,
                                    param_name: Some(param_name.clone()),
                                });
                                
                                let recovery_context = self.create_recovery_context(type_arg, constraint);
                                
                                return Ok(DeepConstraintResult {
                                    satisfied: false,
                                    failure_path: Some(path),
                                    recovery_context: Some(recovery_context),
                                });
                            },
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                }
            }
        }
        
        // Create a root generic type for checking nested constraints
        let root_type = Type::Generic(generic_type.to_string(), type_args.into_iter().map(|t| Box::new(t.clone())).collect());
        
        // Check all nested constraints recursively
        self.check_deep_nested_constraints(&root_type, type_param_constraints)
    }
}

/// Extension of InterfaceRegistry for deep nested constraint checking
impl InterfaceRegistry {
    /// Check if a generic type with nested type arguments satisfies all constraints
    ///
    /// # Arguments
    ///
    /// * `generic_type` - The generic type name
    /// * `type_args` - The concrete type arguments
    /// * `constraints` - A map from type parameter names to interface constraints
    ///
    /// # Returns
    ///
    /// `true` if all constraints are satisfied, `false` otherwise
    pub fn check_nested_generic_constraints(
        &self,
        generic_type: &str,
        type_args: &[Type],
        constraints: &HashMap<String, Vec<String>>,
    ) -> Result<bool, Error> {
        // Find the generic implementation to get the parameter names
        let generic_implementations = self.get_generic_implementations_by_type(generic_type);
        let generic_impl = generic_implementations.first().cloned();
        
        if let Some(generic_impl) = generic_impl {
            // Check the constraints at all levels
            let result = self.check_generic_with_nested_constraints(
                generic_type,
                &generic_impl.type_params,
                type_args,
                constraints
            )?;
            
            Ok(result.satisfied)
        } else {
            // Generic type not found
            debug!("Generic type {} not found in registry", generic_type);
            Ok(false)
        }
    }
    
    /// Check if a generic type with nested type arguments satisfies all constraints,
    /// with detailed error information
    ///
    /// # Arguments
    ///
    /// * `generic_type` - The generic type name
    /// * `type_args` - The concrete type arguments
    /// * `constraints` - A map from type parameter names to interface constraints
    ///
    /// # Returns
    ///
    /// A detailed result with constraint failure information
    pub fn check_nested_generic_constraints_with_details(
        &self,
        generic_type: &str,
        type_args: &[Type],
        constraints: &HashMap<String, Vec<String>>,
    ) -> Result<DeepConstraintResult, Error> {
        // Find the generic implementation to get the parameter names
        let generic_implementations = self.get_generic_implementations_by_type(generic_type);
        let generic_impl = generic_implementations.first().cloned();
        
        if let Some(generic_impl) = generic_impl {
            // Check the constraints at all levels
            self.check_generic_with_nested_constraints(
                generic_type,
                &generic_impl.type_params,
                type_args,
                constraints
            )
        } else {
            // Generic type not found
            debug!("Generic type {} not found in registry", generic_type);
            Ok(DeepConstraintResult {
                satisfied: false,
                failure_path: None,
                recovery_context: None,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::type_checker::Type;
    
    use crate::tests::common;
    
    #[test]
    fn test_deep_nested_constraints_basic() {
        common::tracing::setup();
        
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        // Register a Map[K, V] generic type with K: Comparable constraint
        let map_constraints = vec![("K".to_string(), "Comparable".to_string())];
        registry.register_generic_implementation(
            "Map".to_string(),
            vec!["K".to_string(), "V".to_string()],
            "Container".to_string(),
            map_constraints
        );
        
        // Create a constraint map
        let mut constraint_map = HashMap::new();
        constraint_map.insert("K".to_string(), vec!["Comparable".to_string()]);
        
        // Check with valid arguments
        let type_args = vec![Type::Tea, Type::Normie]; // String and Int
        let result = registry.check_nested_generic_constraints(
            "Map",
            &type_args,
            &constraint_map
        );
        
        assert_eq!(result, Ok(true));
        
        // Check with invalid arguments
        let non_comparable = Type::Struct("NonComparable".to_string(), vec![]);
        let type_args = vec![non_comparable, Type::Normie];
        let result = registry.check_nested_generic_constraints(
            "Map",
            &type_args,
            &constraint_map
        );
        
        assert_eq!(result, Ok(false));
    }
    
    #[test]
    fn test_deep_nested_constraints_with_recovery() {
        common::tracing::setup();
        
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        // Register a Map[K, V] generic type with K: Comparable constraint
        let map_constraints = vec![("K".to_string(), "Comparable".to_string())];
        registry.register_generic_implementation(
            "Map".to_string(),
            vec!["K".to_string(), "V".to_string()],
            "Container".to_string(),
            map_constraints
        );
        
        // Create a constraint map
        let mut constraint_map = HashMap::new();
        constraint_map.insert("K".to_string(), vec!["Comparable".to_string()]);
        
        // Check with invalid arguments and get detailed result
        let non_comparable = Type::Struct("NonComparable".to_string(), vec![]);
        let type_args = vec![non_comparable.clone(), Type::Normie];
        let result = registry.check_nested_generic_constraints_with_details(
            "Map",
            &type_args,
            &constraint_map
        );
        
        // Verify the result has recovery information
        assert!(result.is_ok());
        let details = result.unwrap();
        assert_eq!(details.satisfied, false);
        assert!(details.failure_path.is_some());
        assert!(details.recovery_context.is_some());
        
        // Verify the recovery context has the right information
        let recovery = details.recovery_context.unwrap();
        assert_eq!(recovery.failed_type, non_comparable);
        assert_eq!(recovery.interface_name, "Comparable");
        assert!(!recovery.missing_methods.is_empty());
    }
    
    #[test]
    fn test_deeply_nested_generic_constraints() {
        common::tracing::setup();
        
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        // Register nested generics: Map[K, List[V]] with K: Comparable, V: Container
        registry.register_generic_implementation(
            "Map".to_string(),
            vec!["K".to_string(), "V".to_string()],
            "Container".to_string(),
            vec![("K".to_string(), "Comparable".to_string())]
        );
        
        registry.register_generic_implementation(
            "List".to_string(),
            vec!["T".to_string()],
            "Container".to_string(),
            vec![("T".to_string(), "Container".to_string())]
        );
        
        // Create a constraint map
        let mut constraint_map = HashMap::new();
        constraint_map.insert("K".to_string(), vec!["Comparable".to_string()]);
        constraint_map.insert("T".to_string(), vec!["Container".to_string()]);
        
        // Create a List[V] as the second argument
        let list_type = Type::Generic(
            "List".to_string(),
            vec![Box::new(Type::Struct("Array".to_string(), vec![]))] // Array implements Container
        );
        
        // Check with valid arguments
        let type_args = vec![Type::Tea, list_type]; // String and List[Array]
        let result = registry.check_nested_generic_constraints(
            "Map",
            &type_args,
            &constraint_map
        );
        
        assert_eq!(result, Ok(true));
        
        // Check with invalid nested argument
        let bad_list_type = Type::Generic(
            "List".to_string(),
            vec![Box::new(Type::Lit)] // Lit doesn't implement Container
        );
        
        let type_args = vec![Type::Tea, bad_list_type];
        let result = registry.check_nested_generic_constraints_with_details(
            "Map",
            &type_args,
            &constraint_map
        );
        
        // Verify the result correctly identifies the nested failure
        assert!(result.is_ok());
        let details = result.unwrap();
        assert_eq!(details.satisfied, false);
        
        // The failure path should include the nested List generic
        if let Some(path) = details.failure_path {
            let path_str = path.format();
            assert!(path_str.contains("List"));
            assert!(path_str.contains("Lit"));
        } else {
            panic!("Expected failure path");
        }
    }
}