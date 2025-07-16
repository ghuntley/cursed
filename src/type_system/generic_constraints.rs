//! Generic constraints system for CURSED
//!
//! This module implements comprehensive constraint checking for generic types,
//! including trait bounds, where clauses, and associated types.

use crate::error_types::Error as CursedError;
use crate::type_system::{TypeExpression, TypeEnvironment};
use crate::ast::TypeParameter;
use std::collections::{HashMap, HashSet};

/// Comprehensive constraint checker for generic types
#[derive(Debug)]
pub struct GenericConstraintChecker {
    /// Type environment for constraint resolution
    type_env: TypeEnvironment,
    /// Cache of constraint satisfaction results
    constraint_cache: HashMap<String, ConstraintResult>,
    /// Interface/trait definitions for constraint checking
    interfaces: HashMap<String, InterfaceDefinition>,
}

/// Result of constraint checking
#[derive(Debug, Clone)]
pub struct ConstraintResult {
    pub is_satisfied: bool,
    pub violations: Vec<ConstraintViolation>,
    pub required_implementations: Vec<String>,
}

/// A constraint violation with detailed information
#[derive(Debug, Clone)]
pub struct ConstraintViolation {
    pub constraint_name: String,
    pub type_name: String,
    pub reason: ViolationReason,
    pub suggestion: Option<String>,
}

/// Reason for constraint violation
#[derive(Debug, Clone)]
pub enum ViolationReason {
    InterfaceNotImplemented(String),
    MissingMethod(String),
    TypeMismatch { expected: String, actual: String },
    AssociatedTypeMismatch { expected: String, actual: String },
    WhereClauseViolation(String),
    CircularConstraint,
}

/// Interface definition for constraint checking
#[derive(Debug, Clone)]
pub struct InterfaceDefinition {
    pub name: String,
    pub methods: Vec<InterfaceMethod>,
    pub associated_types: Vec<AssociatedType>,
    pub superinterfaces: Vec<String>,
}

/// Interface method signature
#[derive(Debug, Clone)]
pub struct InterfaceMethod {
    pub name: String,
    pub parameters: Vec<TypeExpression>,
    pub return_type: Option<TypeExpression>,
    pub is_static: bool,
}

/// Associated type definition
#[derive(Debug, Clone)]
pub struct AssociatedType {
    pub name: String,
    pub constraints: Vec<TypeConstraint>,
    pub default_type: Option<TypeExpression>,
}

/// Type constraint for generic parameters
#[derive(Debug, Clone)]
pub enum TypeConstraint {
    /// Interface constraint: T: Display
    Interface(String),
    /// Equality constraint: T = String
    Equality(TypeExpression),
    /// Subtype constraint: T <: Number
    Subtype(TypeExpression),
    /// Supertype constraint: T >: Integer
    Supertype(TypeExpression),
    /// Where clause constraint
    WhereClause(WhereClause),
    /// Lifetime constraint (for future use)
    Lifetime(String),
}

/// Where clause for additional constraints
#[derive(Debug, Clone)]
pub struct WhereClause {
    pub type_expr: TypeExpression,
    pub constraints: Vec<TypeConstraint>,
}

/// Type implementation for interfaces
#[derive(Debug, Clone)]
pub struct TypeImplementation {
    pub type_name: String,
    pub interface_name: String,
    pub methods: Vec<MethodImplementation>,
    pub associated_types: HashMap<String, TypeExpression>,
}

/// Method implementation
#[derive(Debug, Clone)]
pub struct MethodImplementation {
    pub name: String,
    pub parameters: Vec<TypeExpression>,
    pub return_type: Option<TypeExpression>,
    pub body: String, // Simplified for now
}

impl GenericConstraintChecker {
    /// Create a new constraint checker
    pub fn new() -> Self {
        Self {
            type_env: TypeEnvironment::new(),
            constraint_cache: HashMap::new(),
            interfaces: HashMap::new(),
        }
    }

    /// Add an interface definition
    pub fn add_interface(&mut self, interface: InterfaceDefinition) {
        self.interfaces.insert(interface.name.clone(), interface);
    }

    /// Add a type implementation for an interface
    pub fn add_implementation(&mut self, implementation: TypeImplementation) {
        // Store implementation in type environment
        self.type_env.add_type_implementation(implementation);
    }

    /// Check if a type satisfies all constraints
    pub fn check_constraints(
        &mut self,
        type_expr: &TypeExpression,
        constraints: &[TypeConstraint],
    ) -> Result<ConstraintResult, CursedError> {
        let cache_key = format!("{:?}:{:?}", type_expr, constraints);
        
        // Check cache first
        if let Some(cached_result) = self.constraint_cache.get(&cache_key) {
            return Ok(cached_result.clone());
        }

        let mut violations = Vec::new();
        let mut required_implementations = Vec::new();

        // Check each constraint
        for constraint in constraints {
            match self.check_single_constraint(type_expr, constraint)? {
                ConstraintResult { is_satisfied: false, violations: mut v, required_implementations: mut r } => {
                    violations.append(&mut v);
                    required_implementations.append(&mut r);
                }
                ConstraintResult { required_implementations: mut r, .. } => {
                    required_implementations.append(&mut r);
                }
            }
        }

        let result = ConstraintResult {
            is_satisfied: violations.is_empty(),
            violations,
            required_implementations,
        };

        // Cache result
        self.constraint_cache.insert(cache_key, result.clone());
        Ok(result)
    }

    /// Check a single constraint
    fn check_single_constraint(
        &mut self,
        type_expr: &TypeExpression,
        constraint: &TypeConstraint,
    ) -> Result<ConstraintResult, CursedError> {
        match constraint {
            TypeConstraint::Interface(interface_name) => {
                self.check_interface_constraint(type_expr, interface_name)
            }
            TypeConstraint::Equality(expected_type) => {
                self.check_equality_constraint(type_expr, expected_type)
            }
            TypeConstraint::Subtype(supertype) => {
                self.check_subtype_constraint(type_expr, supertype)
            }
            TypeConstraint::Supertype(subtype) => {
                self.check_supertype_constraint(type_expr, subtype)
            }
            TypeConstraint::WhereClause(where_clause) => {
                self.check_where_clause(type_expr, where_clause)
            }
            TypeConstraint::Lifetime(_) => {
                // Lifetime constraints not implemented yet
                Ok(ConstraintResult {
                    is_satisfied: true,
                    violations: vec![],
                    required_implementations: vec![],
                })
            }
        }
    }

    /// Check interface constraint (T: Display)
    fn check_interface_constraint(
        &self,
        type_expr: &TypeExpression,
        interface_name: &str,
    ) -> Result<ConstraintResult, CursedError> {
        let interface = self.interfaces.get(interface_name)
            .ok_or_else(|| CursedError::InterfaceNotFound(interface_name.to_string()))?;

        // Check if type implements the interface
        if self.type_env.type_implements_interface(type_expr, interface_name) {
            Ok(ConstraintResult {
                is_satisfied: true,
                violations: vec![],
                required_implementations: vec![],
            })
        } else {
            // Check what methods are missing
            let missing_methods = self.find_missing_methods(type_expr, interface)?;
            
            let violations = missing_methods.into_iter()
                .map(|method| ConstraintViolation {
                    constraint_name: interface_name.to_string(),
                    type_name: format!("{:?}", type_expr),
                    reason: ViolationReason::MissingMethod(method.clone()),
                    suggestion: Some(format!("Implement method '{}' for type '{:?}'", method, type_expr)),
                })
                .collect();

            Ok(ConstraintResult {
                is_satisfied: false,
                violations,
                required_implementations: vec![interface_name.to_string()],
            })
        }
    }

    /// Check equality constraint (T = String)
    fn check_equality_constraint(
        &self,
        type_expr: &TypeExpression,
        expected_type: &TypeExpression,
    ) -> Result<ConstraintResult, CursedError> {
        let is_equal = self.type_env.types_equal(type_expr, expected_type);
        
        if is_equal {
            Ok(ConstraintResult {
                is_satisfied: true,
                violations: vec![],
                required_implementations: vec![],
            })
        } else {
            Ok(ConstraintResult {
                is_satisfied: false,
                violations: vec![ConstraintViolation {
                    constraint_name: "equality".to_string(),
                    type_name: format!("{:?}", type_expr),
                    reason: ViolationReason::TypeMismatch {
                        expected: format!("{:?}", expected_type),
                        actual: format!("{:?}", type_expr),
                    },
                    suggestion: None,
                }],
                required_implementations: vec![],
            })
        }
    }

    /// Check subtype constraint (T <: Number)
    fn check_subtype_constraint(
        &self,
        type_expr: &TypeExpression,
        supertype: &TypeExpression,
    ) -> Result<ConstraintResult, CursedError> {
        let is_subtype = self.type_env.is_subtype(type_expr, supertype);
        
        if is_subtype {
            Ok(ConstraintResult {
                is_satisfied: true,
                violations: vec![],
                required_implementations: vec![],
            })
        } else {
            Ok(ConstraintResult {
                is_satisfied: false,
                violations: vec![ConstraintViolation {
                    constraint_name: "subtype".to_string(),
                    type_name: format!("{:?}", type_expr),
                    reason: ViolationReason::TypeMismatch {
                        expected: format!("{:?} <: {:?}", type_expr, supertype),
                        actual: format!("{:?}", type_expr),
                    },
                    suggestion: None,
                }],
                required_implementations: vec![],
            })
        }
    }

    /// Check supertype constraint (T >: Integer)
    fn check_supertype_constraint(
        &self,
        type_expr: &TypeExpression,
        subtype: &TypeExpression,
    ) -> Result<ConstraintResult, CursedError> {
        let is_supertype = self.type_env.is_subtype(subtype, type_expr);
        
        if is_supertype {
            Ok(ConstraintResult {
                is_satisfied: true,
                violations: vec![],
                required_implementations: vec![],
            })
        } else {
            Ok(ConstraintResult {
                is_satisfied: false,
                violations: vec![ConstraintViolation {
                    constraint_name: "supertype".to_string(),
                    type_name: format!("{:?}", type_expr),
                    reason: ViolationReason::TypeMismatch {
                        expected: format!("{:?} >: {:?}", type_expr, subtype),
                        actual: format!("{:?}", type_expr),
                    },
                    suggestion: None,
                }],
                required_implementations: vec![],
            })
        }
    }

    /// Check where clause constraint - public wrapper for interface validation
    pub fn check_where_clause_public(
        &mut self,
        where_clause: &WhereClause,
        _type_env: &TypeEnvironment,
    ) -> Result<bool, CursedError> {
        // Simplified public version
        let result = self.check_constraints(&where_clause.type_expr, &where_clause.constraints)?;
        Ok(result.is_satisfied)
    }

    /// Check where clause constraint
    fn check_where_clause(
        &mut self,
        _type_expr: &TypeExpression,
        where_clause: &WhereClause,
    ) -> Result<ConstraintResult, CursedError> {
        // Recursively check constraints in where clause
        self.check_constraints(&where_clause.type_expr, &where_clause.constraints)
    }

    /// Find missing methods for interface implementation
    fn find_missing_methods(
        &self,
        type_expr: &TypeExpression,
        interface: &InterfaceDefinition,
    ) -> Result<Vec<String>, CursedError> {
        let mut missing = Vec::new();
        
        for method in &interface.methods {
            if !self.type_env.type_has_method(type_expr, &method.name) {
                missing.push(method.name.clone());
            }
        }

        Ok(missing)
    }

    /// Check multiple types against constraints (batch processing)
    pub fn check_multiple_constraints(
        &mut self,
        type_constraints: &[(TypeExpression, Vec<TypeConstraint>)],
    ) -> Result<Vec<ConstraintResult>, CursedError> {
        let mut results = Vec::new();
        
        for (type_expr, constraints) in type_constraints {
            let result = self.check_constraints(type_expr, constraints)?;
            results.push(result);
        }

        Ok(results)
    }

    /// Get all interfaces
    pub fn get_interfaces(&self) -> &HashMap<String, InterfaceDefinition> {
        &self.interfaces
    }

    /// Clear constraint cache
    pub fn clear_cache(&mut self) {
        self.constraint_cache.clear();
    }

    /// Get constraint cache statistics
    pub fn get_cache_stats(&self) -> (usize, usize) {
        (self.constraint_cache.len(), self.constraint_cache.capacity())
    }
}

/// Extension methods for TypeEnvironment
impl TypeEnvironment {
    /// Check if a type implements an interface
    pub fn type_implements_interface(&self, _type_expr: &TypeExpression, _interface_name: &str) -> bool {
        // Implementation would check stored implementations
        false
    }

    /// Check if two types are equal
    pub fn types_equal(&self, _type1: &TypeExpression, _type2: &TypeExpression) -> bool {
        // Implementation would check type equality
        false
    }

    /// Check if type1 is a subtype of type2
    pub fn is_subtype(&self, _type1: &TypeExpression, _type2: &TypeExpression) -> bool {
        // Implementation would check subtype relationship
        false
    }

    /// Check if a type has a method
    pub fn type_has_method(&self, _type_expr: &TypeExpression, _method_name: &str) -> bool {
        // Implementation would check method existence
        false
    }

    /// Add a type implementation
    pub fn add_type_implementation(&mut self, _implementation: TypeImplementation) {
        // Implementation would store the implementation
    }
}

/// Built-in interface definitions
pub fn create_builtin_interfaces() -> Vec<InterfaceDefinition> {
    vec![
        // Display interface
        InterfaceDefinition {
            name: "Display".to_string(),
            methods: vec![
                InterfaceMethod {
                    name: "display".to_string(),
                    parameters: vec![],
                    return_type: Some(TypeExpression::named("tea")),
                    is_static: false,
                },
            ],
            associated_types: vec![],
            superinterfaces: vec![],
        },
        // Comparable interface
        InterfaceDefinition {
            name: "Comparable".to_string(),
            methods: vec![
                InterfaceMethod {
                    name: "compare".to_string(),
                    parameters: vec![TypeExpression::named("Self")],
                    return_type: Some(TypeExpression::named("normie")),
                    is_static: false,
                },
            ],
            associated_types: vec![],
            superinterfaces: vec![],
        },
        // Copyable interface
        InterfaceDefinition {
            name: "Copyable".to_string(),
            methods: vec![
                InterfaceMethod {
                    name: "copy".to_string(),
                    parameters: vec![],
                    return_type: Some(TypeExpression::named("Self")),
                    is_static: false,
                },
            ],
            associated_types: vec![],
            superinterfaces: vec![],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constraint_checker_creation() {
        let checker = GenericConstraintChecker::new();
        assert!(checker.interfaces.is_empty());
        assert!(checker.constraint_cache.is_empty());
    }

    #[test]
    fn test_interface_addition() {
        let mut checker = GenericConstraintChecker::new();
        let interface = InterfaceDefinition {
            name: "Test".to_string(),
            methods: vec![],
            associated_types: vec![],
            superinterfaces: vec![],
        };

        checker.add_interface(interface);
        assert!(checker.interfaces.contains_key("Test"));
    }

    #[test]
    fn test_builtin_interfaces() {
        let interfaces = create_builtin_interfaces();
        assert!(interfaces.len() >= 3);
        assert!(interfaces.iter().any(|i| i.name == "Display"));
        assert!(interfaces.iter().any(|i| i.name == "Comparable"));
        assert!(interfaces.iter().any(|i| i.name == "Copyable"));
    }

    #[test]
    fn test_constraint_result() {
        let result = ConstraintResult {
            is_satisfied: true,
            violations: vec![],
            required_implementations: vec![],
        };
        assert!(result.is_satisfied);
        assert!(result.violations.is_empty());
    }

    #[test]
    fn test_constraint_violation() {
        let violation = ConstraintViolation {
            constraint_name: "Display".to_string(),
            type_name: "CustomType".to_string(),
            reason: ViolationReason::MissingMethod("display".to_string()),
            suggestion: Some("Implement display method".to_string()),
        };
        assert_eq!(violation.constraint_name, "Display");
        assert!(violation.suggestion.is_some());
    }
}
