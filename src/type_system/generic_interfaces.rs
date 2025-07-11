//! Generic interfaces for CURSED
//! Provides trait-like interfaces with generic type parameters and associated types
//! 
//! NOTE: This module is temporarily disabled due to missing Type variants
//! (Reference, SelfType, etc.) that need to be implemented first.

use crate::error::CursedError;
use crate::ast::{Type, FunctionStatement, Parameter, Statement};
use super::{TypeExpression, TypeEnvironment, GenericConstraint};
use super::advanced_constraints::AdvancedConstraint;
use super::advanced_constraints::AdvancedConstraintChecker;
use std::collections::{HashMap, HashSet};

/// Interface definition with generic parameters
#[derive(Debug, Clone)]
pub struct GenericInterface {
    /// Interface name
    pub name: String,
    /// Generic type parameters
    pub type_parameters: Vec<GenericTypeParameter>,
    /// Associated types
    pub associated_types: Vec<AssociatedType>,
    /// Method signatures
    pub methods: Vec<InterfaceMethod>,
    /// Superinterfaces (inheritance)
    pub superinterfaces: Vec<String>,
    /// Where clauses
    pub where_clauses: Vec<WhereClause>,
    /// Default implementations
    pub default_implementations: HashMap<String, FunctionStatement>,
}

/// Generic type parameter with bounds
#[derive(Debug, Clone)]
pub struct GenericTypeParameter {
    /// Parameter name
    pub name: String,
    /// Variance (covariant, contravariant, invariant)
    pub variance: Variance,
    /// Constraints on the parameter
    pub constraints: Vec<AdvancedConstraint>,
    /// Default type
    pub default_type: Option<TypeExpression>,
}

/// Associated type definition
#[derive(Debug, Clone)]
pub struct AssociatedType {
    /// Associated type name
    pub name: String,
    /// Constraints on the associated type
    pub constraints: Vec<AdvancedConstraint>,
    /// Default type
    pub default_type: Option<TypeExpression>,
}

/// Interface method signature
#[derive(Debug, Clone)]
pub struct InterfaceMethod {
    /// Method name
    pub name: String,
    /// Generic type parameters for the method
    pub type_parameters: Vec<GenericTypeParameter>,
    /// Method parameters
    pub parameters: Vec<Parameter>,
    /// Return type
    pub return_type: Option<Type>,
    /// Where clauses for the method
    pub where_clauses: Vec<WhereClause>,
    /// Whether this method has a default implementation
    pub has_default: bool,
}

/// Where clause for additional constraints
#[derive(Debug, Clone)]
pub struct WhereClause {
    /// Type being constrained
    pub type_expr: TypeExpression,
    /// Constraints on the type
    pub constraints: Vec<AdvancedConstraint>,
}

/// Type variance annotation
#[derive(Debug, Clone, PartialEq)]
pub enum Variance {
    /// Covariant (+T)
    Covariant,
    /// Contravariant (-T)
    Contravariant,
    /// Invariant (T)
    Invariant,
}

/// Interface implementation
#[derive(Debug, Clone)]
pub struct InterfaceImplementation {
    /// Interface being implemented
    pub interface_name: String,
    /// Type implementing the interface
    pub implementing_type: TypeExpression,
    /// Type parameter bindings
    pub type_bindings: HashMap<String, TypeExpression>,
    /// Associated type bindings
    pub associated_type_bindings: HashMap<String, TypeExpression>,
    /// Method implementations
    pub method_implementations: HashMap<String, FunctionStatement>,
    /// Where clauses for the implementation
    pub where_clauses: Vec<WhereClause>,
}

/// Generic interface checker and resolver
#[derive(Debug)]
pub struct GenericInterfaceChecker {
    /// Registered interfaces
    interfaces: HashMap<String, GenericInterface>,
    /// Interface implementations
    implementations: HashMap<String, Vec<InterfaceImplementation>>,
    /// Constraint checker
    constraint_checker: AdvancedConstraintChecker,
    /// Interface hierarchy
    hierarchy: InterfaceHierarchy,
}

/// Interface inheritance hierarchy
#[derive(Debug)]
pub struct InterfaceHierarchy {
    /// Parent-child relationships
    relationships: HashMap<String, Vec<String>>,
    /// Inheritance order cache
    order_cache: HashMap<String, Vec<String>>,
}

impl GenericInterfaceChecker {
    pub fn new() -> Self {
        let mut checker = Self {
            interfaces: HashMap::new(),
            implementations: HashMap::new(),
            constraint_checker: AdvancedConstraintChecker::new(),
            hierarchy: InterfaceHierarchy::new(),
        };
        
        checker.register_builtin_interfaces();
        checker
    }
    
    /// Register built-in interfaces
    fn register_builtin_interfaces(&mut self) {
        // Clone interface
        self.register_interface(GenericInterface {
            name: "Clone".to_string(),
            type_parameters: vec![],
            associated_types: vec![],
            methods: vec![
                InterfaceMethod {
                    name: "clone".to_string(),
                    type_parameters: vec![],
                    parameters: vec![Parameter {
                        name: "self".to_string(),
                        param_type: Some(Type::Custom("Self".to_string())),
                    }],
                    return_type: Some(Type::Custom("Self".to_string())),
                    where_clauses: vec![],
                    has_default: false,
                }
            ],
            superinterfaces: vec![],
            where_clauses: vec![],
            default_implementations: HashMap::new(),
        }).unwrap();
        
        // Iterator interface
        self.register_interface(GenericInterface {
            name: "Iterator".to_string(),
            type_parameters: vec![],
            associated_types: vec![
                AssociatedType {
                    name: "Item".to_string(),
                    constraints: vec![],
                    default_type: None,
                }
            ],
            methods: vec![
                InterfaceMethod {
                    name: "next".to_string(),
                    type_parameters: vec![],
                    parameters: vec![Parameter {
                        name: "self".to_string(),
                        param_type: Some(Type::Custom("Self".to_string())),
                    }],
                    return_type: Some(Type::Custom("Option".to_string())),
                    where_clauses: vec![],
                    has_default: false,
                }
            ],
            superinterfaces: vec![],
            where_clauses: vec![],
            default_implementations: HashMap::new(),
        }).unwrap();
        
        // IntoIterator interface
        self.register_interface(GenericInterface {
            name: "IntoIterator".to_string(),
            type_parameters: vec![],
            associated_types: vec![
                AssociatedType {
                    name: "Item".to_string(),
                    constraints: vec![],
                    default_type: None,
                },
                AssociatedType {
                    name: "IntoIter".to_string(),
                    constraints: vec![AdvancedConstraint::TraitBound("IntoIter".to_string(), "Iterator".to_string())],
                    default_type: None,
                }
            ],
            methods: vec![
                InterfaceMethod {
                    name: "into_iter".to_string(),
                    type_parameters: vec![],
                    parameters: vec![Parameter {
                        name: "self".to_string(),
                        param_type: Some(Type::Custom("Self".to_string())),
                    }],
                    return_type: Some(Type::Custom("IntoIter".to_string())),
                    where_clauses: vec![],
                    has_default: false,
                }
            ],
            superinterfaces: vec![],
            where_clauses: vec![],
            default_implementations: HashMap::new(),
        }).unwrap();
        
        // From interface
        self.register_interface(GenericInterface {
            name: "From".to_string(),
            type_parameters: vec![
                GenericTypeParameter {
                    name: "T".to_string(),
                    variance: Variance::Invariant,
                    constraints: vec![],
                    default_type: None,
                }
            ],
            associated_types: vec![],
            methods: vec![
                InterfaceMethod {
                    name: "from".to_string(),
                    type_parameters: vec![],
                    parameters: vec![Parameter {
                        name: "value".to_string(),
                        param_type: Some(Type::Custom("T".to_string())),
                    }],
                    return_type: Some(Type::Custom("Self".to_string())),
                    where_clauses: vec![],
                    has_default: false,
                }
            ],
            superinterfaces: vec![],
            where_clauses: vec![],
            default_implementations: HashMap::new(),
        }).unwrap();
        
        // Functor interface (higher-kinded)
        self.register_interface(GenericInterface {
            name: "Functor".to_string(),
            type_parameters: vec![
                GenericTypeParameter {
                    name: "F".to_string(),
                    variance: Variance::Covariant,
                    constraints: vec![],
                    default_type: None,
                }
            ],
            associated_types: vec![],
            methods: vec![
                InterfaceMethod {
                    name: "map".to_string(),
                    type_parameters: vec![
                        GenericTypeParameter {
                            name: "A".to_string(),
                            variance: Variance::Invariant,
                            constraints: vec![],
                            default_type: None,
                        },
                        GenericTypeParameter {
                            name: "B".to_string(),
                            variance: Variance::Invariant,
                            constraints: vec![],
                            default_type: None,
                        }
                    ],
                    parameters: vec![
                        Parameter {
                            name: "self".to_string(),
                            param_type: Some(Type::Custom("F".to_string())),
                        },
                        Parameter {
                            name: "f".to_string(),
                            param_type: Some(Type::Function(
                                vec![Type::Custom("A".to_string())],
                                Box::new(Type::Custom("B".to_string()))
                            )),
                        }
                    ],
                    return_type: Some(Type::Custom("F".to_string())),
                    where_clauses: vec![],
                    has_default: false,
                }
            ],
            superinterfaces: vec![],
            where_clauses: vec![],
            default_implementations: HashMap::new(),
        }).unwrap();
    }
    
    /// Register a new interface
    pub fn register_interface(&mut self, interface: GenericInterface) -> Result<(), CursedError> {
        // Validate interface definition
        self.validate_interface(&interface)?;
        
        // Update hierarchy
        for superinterface in &interface.superinterfaces {
            self.hierarchy.add_relationship(superinterface.clone(), interface.name.clone());
        }
        
        self.interfaces.insert(interface.name.clone(), interface);
        Ok(())
    }
    
    /// Validate interface definition
    fn validate_interface(&self, interface: &GenericInterface) -> Result<(), CursedError> {
        // Check for duplicate type parameters
        let mut seen_params = HashSet::new();
        for param in &interface.type_parameters {
            if seen_params.contains(&param.name) {
                return Err(CursedError::type_error(&format!(
                    "Duplicate type parameter '{}' in interface '{}'",
                    param.name, interface.name
                )));
            }
            seen_params.insert(param.name.clone());
        }
        
        // Check for duplicate associated types
        let mut seen_assoc = HashSet::new();
        for assoc in &interface.associated_types {
            if seen_assoc.contains(&assoc.name) {
                return Err(CursedError::type_error(&format!(
                    "Duplicate associated type '{}' in interface '{}'",
                    assoc.name, interface.name
                )));
            }
            seen_assoc.insert(assoc.name.clone());
        }
        
        // Check for duplicate methods
        let mut seen_methods = HashSet::new();
        for method in &interface.methods {
            if seen_methods.contains(&method.name) {
                return Err(CursedError::type_error(&format!(
                    "Duplicate method '{}' in interface '{}'",
                    method.name, interface.name
                )));
            }
            seen_methods.insert(method.name.clone());
        }
        
        // Validate superinterfaces exist
        for superinterface in &interface.superinterfaces {
            if !self.interfaces.contains_key(superinterface) {
                return Err(CursedError::type_error(&format!(
                    "Unknown superinterface '{}' in interface '{}'",
                    superinterface, interface.name
                )));
            }
        }
        
        Ok(())
    }
    
    /// Register interface implementation
    pub fn register_implementation(&mut self, implementation: InterfaceImplementation) -> Result<(), CursedError> {
        // Validate implementation
        self.validate_implementation(&implementation)?;
        
        let interface_name = implementation.interface_name.clone();
        self.implementations.entry(interface_name)
            .or_insert_with(Vec::new)
            .push(implementation);
        
        Ok(())
    }
    
    /// Validate interface implementation
    fn validate_implementation(&self, implementation: &InterfaceImplementation) -> Result<(), CursedError> {
        let interface = self.interfaces.get(&implementation.interface_name)
            .ok_or_else(|| CursedError::type_error(&format!(
                "Unknown interface '{}'", implementation.interface_name
            )))?;
        
        // Check all required methods are implemented
        for method in &interface.methods {
            if !method.has_default && !implementation.method_implementations.contains_key(&method.name) {
                return Err(CursedError::type_error(&format!(
                    "Missing implementation for method '{}' in interface '{}'",
                    method.name, interface.name
                )));
            }
        }
        
        // Check all associated types are bound
        for assoc_type in &interface.associated_types {
            if assoc_type.default_type.is_none() && !implementation.associated_type_bindings.contains_key(&assoc_type.name) {
                return Err(CursedError::type_error(&format!(
                    "Missing binding for associated type '{}' in interface '{}'",
                    assoc_type.name, interface.name
                )));
            }
        }
        
        // Validate constraints
        let mut type_bindings = implementation.type_bindings.clone();
        type_bindings.extend(implementation.associated_type_bindings.clone());
        
        self.constraint_checker.check_constraints(&type_bindings)?;
        
        Ok(())
    }
    
    /// Check if a type implements an interface
    pub fn implements_interface(&self, type_expr: &TypeExpression, interface_name: &str) -> Result<bool, CursedError> {
        if let Some(implementations) = self.implementations.get(interface_name) {
            for impl_item in implementations {
                if self.type_matches(&impl_item.implementing_type, type_expr) {
                    return Ok(true);
                }
            }
        }
        
        Ok(false)
    }
    
    /// Find interface implementation for a type
    pub fn find_implementation(&self, 
                              type_expr: &TypeExpression, 
                              interface_name: &str) -> Option<&InterfaceImplementation> {
        if let Some(implementations) = self.implementations.get(interface_name) {
            for impl_item in implementations {
                if self.type_matches(&impl_item.implementing_type, type_expr) {
                    return Some(impl_item);
                }
            }
        }
        
        None
    }
    
    /// Check if two types match (with generics)
    fn type_matches(&self, pattern: &TypeExpression, concrete: &TypeExpression) -> bool {
        // Simple matching for now - would need full unification in practice
        pattern.name == concrete.name && 
        pattern.parameters.len() == concrete.parameters.len() &&
        pattern.parameters.iter().zip(concrete.parameters.iter())
            .all(|(p, c)| self.type_matches(p, c))
    }
    
    /// Get interface definition
    pub fn get_interface(&self, name: &str) -> Option<&GenericInterface> {
        self.interfaces.get(name)
    }
    
    /// Get all implementations of an interface
    pub fn get_implementations(&self, interface_name: &str) -> Option<&Vec<InterfaceImplementation>> {
        self.implementations.get(interface_name)
    }
    
    /// Check interface hierarchy
    pub fn is_subinterface(&self, child: &str, parent: &str) -> bool {
        self.hierarchy.is_descendant(child, parent)
    }
    
    /// Get all superinterfaces of an interface
    pub fn get_superinterfaces(&self, interface_name: &str) -> Vec<String> {
        self.hierarchy.get_ancestors(interface_name)
    }
}

impl InterfaceHierarchy {
    pub fn new() -> Self {
        Self {
            relationships: HashMap::new(),
            order_cache: HashMap::new(),
        }
    }
    
    /// Add parent-child relationship
    pub fn add_relationship(&mut self, parent: String, child: String) {
        self.relationships.entry(parent)
            .or_insert_with(Vec::new)
            .push(child);
        
        // Clear cache
        self.order_cache.clear();
    }
    
    /// Check if child is a descendant of parent
    pub fn is_descendant(&self, child: &str, parent: &str) -> bool {
        if child == parent {
            return true;
        }
        
        if let Some(children) = self.relationships.get(parent) {
            for child_name in children {
                if self.is_descendant(child, child_name) {
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Get all ancestors of an interface
    pub fn get_ancestors(&self, interface_name: &str) -> Vec<String> {
        let mut ancestors = Vec::new();
        let mut visited = HashSet::new();
        
        self.collect_ancestors(interface_name, &mut ancestors, &mut visited);
        ancestors
    }
    
    fn collect_ancestors(&self, interface_name: &str, ancestors: &mut Vec<String>, visited: &mut HashSet<String>) {
        if visited.contains(interface_name) {
            return;
        }
        
        visited.insert(interface_name.to_string());
        
        for (parent, children) in &self.relationships {
            if children.contains(&interface_name.to_string()) {
                ancestors.push(parent.clone());
                self.collect_ancestors(parent, ancestors, visited);
            }
        }
    }
}

/// Utility functions for generic interfaces
pub mod interface_utils {
    use super::*;
    
    /// Create a simple interface with no generics
    pub fn create_simple_interface(name: &str, methods: Vec<InterfaceMethod>) -> GenericInterface {
        GenericInterface {
            name: name.to_string(),
            type_parameters: vec![],
            associated_types: vec![],
            methods,
            superinterfaces: vec![],
            where_clauses: vec![],
            default_implementations: HashMap::new(),
        }
    }
    
    /// Create a generic interface with type parameters
    pub fn create_generic_interface(name: &str, 
                                   type_params: Vec<GenericTypeParameter>,
                                   methods: Vec<InterfaceMethod>) -> GenericInterface {
        GenericInterface {
            name: name.to_string(),
            type_parameters: type_params,
            associated_types: vec![],
            methods,
            superinterfaces: vec![],
            where_clauses: vec![],
            default_implementations: HashMap::new(),
        }
    }
    
    /// Create a method signature
    pub fn create_method(name: &str, 
                        params: Vec<Parameter>,
                        return_type: Option<Type>) -> InterfaceMethod {
        InterfaceMethod {
            name: name.to_string(),
            type_parameters: vec![],
            parameters: params,
            return_type,
            where_clauses: vec![],
            has_default: false,
        }
    }
    
    /// Create a type parameter
    pub fn create_type_parameter(name: &str, 
                                constraints: Vec<AdvancedConstraint>) -> GenericTypeParameter {
        GenericTypeParameter {
            name: name.to_string(),
            variance: Variance::Invariant,
            constraints,
            default_type: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Parameter;

    #[test]
    fn test_interface_registration() {
        let mut checker = GenericInterfaceChecker::new();
        
        let interface = GenericInterface {
            name: "TestInterface".to_string(),
            type_parameters: vec![],
            associated_types: vec![],
            methods: vec![],
            superinterfaces: vec![],
            where_clauses: vec![],
            default_implementations: HashMap::new(),
        };
        
        assert!(checker.register_interface(interface).is_ok());
        assert!(checker.get_interface("TestInterface").is_some());
    }

    #[test]
    fn test_builtin_interfaces() {
        let checker = GenericInterfaceChecker::new();
        
        assert!(checker.get_interface("Clone").is_some());
        assert!(checker.get_interface("Iterator").is_some());
        assert!(checker.get_interface("IntoIterator").is_some());
        assert!(checker.get_interface("From").is_some());
        assert!(checker.get_interface("Functor").is_some());
    }

    #[test]
    fn test_interface_hierarchy() {
        let mut hierarchy = InterfaceHierarchy::new();
        
        hierarchy.add_relationship("Parent".to_string(), "Child".to_string());
        hierarchy.add_relationship("Child".to_string(), "Grandchild".to_string());
        
        assert!(hierarchy.is_descendant("Grandchild", "Parent"));
        assert!(hierarchy.is_descendant("Child", "Parent"));
        assert!(!hierarchy.is_descendant("Parent", "Child"));
    }

    #[test]
    fn test_interface_validation() {
        let mut checker = GenericInterfaceChecker::new();
        
        // Test duplicate type parameters
        let invalid_interface = GenericInterface {
            name: "InvalidInterface".to_string(),
            type_parameters: vec![
                GenericTypeParameter {
                    name: "T".to_string(),
                    variance: Variance::Invariant,
                    constraints: vec![],
                    default_type: None,
                },
                GenericTypeParameter {
                    name: "T".to_string(),
                    variance: Variance::Invariant,
                    constraints: vec![],
                    default_type: None,
                },
            ],
            associated_types: vec![],
            methods: vec![],
            superinterfaces: vec![],
            where_clauses: vec![],
            default_implementations: HashMap::new(),
        };
        
        assert!(checker.validate_interface(&invalid_interface).is_err());
    }

    #[test]
    fn test_interface_utilities() {
        let interface = interface_utils::create_simple_interface("TestInterface", vec![]);
        assert_eq!(interface.name, "TestInterface");
        assert_eq!(interface.type_parameters.len(), 0);
        
        let type_param = interface_utils::create_type_parameter("T", vec![]);
        assert_eq!(type_param.name, "T");
        assert_eq!(type_param.variance, Variance::Invariant);
    }
}
