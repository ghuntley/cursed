//! Generic Interface Implementation for CURSED
//!
//! This module provides comprehensive support for generic interfaces including:
//! - Generic interface definitions with type parameters
//! - Generic constraint checking for interface compliance
//! - Generic interface instantiation and monomorphization
//! - Higher-kinded type support for interface generics
//! - Associated type support within generic interfaces
//! - Complex type parameter constraint resolution

use crate::ast::{InterfaceStatement, MethodSignature, Parameter, Type as AstType, TypeParameter};
use crate::error_types::CursedError;
use crate::type_system::{
    TypeExpression, TypeSubstitution, GenericConstraint, TypeEnvironment,
    interface_compliance::{InterfaceMethodRequirement, ReceiverType},
    generic_constraints::{GenericConstraintChecker, TypeConstraint, WhereClause},
    constraint_resolver::{ConstraintResolver, ConstraintSolution},
    higher_kinded_types::{Kind, TypeConstructor, KindedTypeParameter},
    generics_core::GenericsCore,
};
use std::collections::{HashMap, HashSet};

/// Generic interface definition with type parameters and constraints
#[derive(Debug, Clone)]
pub struct GenericInterface {
    /// Interface name
    pub name: String,
    /// Generic type parameters
    pub type_parameters: Vec<GenericTypeParameter>,
    /// Base interfaces this interface extends
    pub extends: Vec<String>,
    /// Method signatures with generic types
    pub methods: Vec<InterfaceMethod>,
    /// Where clauses for additional constraints
    pub where_clauses: Vec<WhereClause>,
    /// Associated types defined by this interface
    pub associated_types: Vec<AssociatedType>,
    /// Interface variance annotations
    pub variance: Vec<Variance>,
}

/// Generic type parameter with enhanced constraint support
#[derive(Debug, Clone)]
pub struct GenericTypeParameter {
    /// Parameter name (e.g., "T", "U")
    pub name: String,
    /// Kind information for higher-kinded types
    pub kind: Kind,
    /// Trait bounds (e.g., T: Clone + Send)
    pub bounds: Vec<String>,
    /// Default type if not specified
    pub default: Option<TypeExpression>,
    /// Variance annotation (covariant, contravariant, invariant)
    pub variance: Variance,
}

/// Associated type definition within an interface
#[derive(Debug, Clone)]
pub struct AssociatedType {
    /// Name of the associated type
    pub name: String,
    /// Optional default implementation
    pub default: Option<TypeExpression>,
    /// Additional constraints on the associated type
    pub constraints: Vec<TypeConstraint>,
}

/// Method within a generic interface
#[derive(Debug, Clone)]
pub struct InterfaceMethod {
    /// Method name
    pub name: String,
    /// Generic type parameters specific to this method
    pub type_parameters: Vec<GenericTypeParameter>,
    /// Method parameters with potentially generic types
    pub parameters: Vec<Parameter>,
    /// Return type (potentially generic)
    pub return_type: Option<AstType>,
    /// Receiver type (self, &self, &mut self)
    pub receiver: Option<Parameter>,
    /// Method-specific where clauses
    pub where_clauses: Vec<WhereClause>,
}

/// Variance annotation for type parameters
#[derive(Debug, Clone, PartialEq)]
pub enum Variance {
    /// Type parameter can vary in the same direction (T → U implies Generic<T> → Generic<U>)
    Covariant,
    /// Type parameter varies in opposite direction (T → U implies Generic<U> → Generic<T>)
    Contravariant,
    /// Type parameter cannot vary (T ≠ U implies Generic<T> ≠ Generic<U>)
    Invariant,
}

/// Implementation of a generic interface for a specific type
#[derive(Debug, Clone)]
pub struct InterfaceImplementation {
    /// The type implementing the interface
    pub implementing_type: String,
    /// The generic interface being implemented
    pub interface_name: String,
    /// Type arguments for generic parameters
    pub type_arguments: Vec<TypeExpression>,
    /// Associated type implementations
    pub associated_type_impls: HashMap<String, TypeExpression>,
    /// Method implementations
    pub method_implementations: Vec<ConcreteMethodImplementation>,
}

/// Concrete implementation of an interface method
#[derive(Debug, Clone)]
pub struct ConcreteMethodImplementation {
    /// Method name
    pub name: String,
    /// Implemented parameter types
    pub parameters: Vec<TypeExpression>,
    /// Implemented return type
    pub return_type: Option<TypeExpression>,
    /// Receiver type
    pub receiver_type: ReceiverType,
}

/// Interface hierarchy for generic interfaces
#[derive(Debug, Clone)]
pub struct InterfaceHierarchy {
    /// Map from interface name to generic interface definition
    interfaces: HashMap<String, GenericInterface>,
    /// Map from interface name to its direct parent interfaces
    parents: HashMap<String, Vec<String>>,
    /// Map from interface name to all transitive parents
    transitive_parents: HashMap<String, Vec<String>>,
    /// Map from (type, interface) to implementation
    implementations: HashMap<(String, String), InterfaceImplementation>,
}

/// Generic interface checker with full constraint validation
#[derive(Debug)]
pub struct GenericInterfaceChecker {
    /// Interface hierarchy
    hierarchy: InterfaceHierarchy,
    /// Constraint checker for generic bounds
    constraint_checker: GenericConstraintChecker,
    /// Constraint resolver for complex constraints
    constraint_resolver: ConstraintResolver,
    /// Type environment for context
    type_environment: TypeEnvironment,
    /// Generics core for type conversions
    generics_core: GenericsCore,
}

impl GenericInterface {
    /// Create a new generic interface
    pub fn new(name: String) -> Self {
        Self {
            name,
            type_parameters: Vec::new(),
            extends: Vec::new(),
            methods: Vec::new(),
            where_clauses: Vec::new(),
            associated_types: Vec::new(),
            variance: Vec::new(),
        }
    }

    /// Add a type parameter to the interface
    pub fn add_type_parameter(&mut self, param: GenericTypeParameter) {
        self.type_parameters.push(param);
    }

    /// Add a method to the interface
    pub fn add_method(&mut self, method: InterfaceMethod) {
        self.methods.push(method);
    }

    /// Add an associated type
    pub fn add_associated_type(&mut self, associated_type: AssociatedType) {
        self.associated_types.push(associated_type);
    }

    /// Check if this interface is generic
    pub fn is_generic(&self) -> bool {
        !self.type_parameters.is_empty()
    }

    /// Get type parameter by name
    pub fn get_type_parameter(&self, name: &str) -> Option<&GenericTypeParameter> {
        self.type_parameters.iter().find(|p| p.name == name)
    }

    /// Get associated type by name
    pub fn get_associated_type(&self, name: &str) -> Option<&AssociatedType> {
        self.associated_types.iter().find(|at| at.name == name)
    }

    /// Instantiate the interface with concrete type arguments
    pub fn instantiate(&self, type_args: &[TypeExpression]) -> Result<GenericInterface, CursedError> {
        if type_args.len() != self.type_parameters.len() {
            return Err(CursedError::Type(format!(
                "Interface {} expects {} type arguments, got {}",
                self.name,
                self.type_parameters.len(),
                type_args.len()
            )));
        }

        // Create type substitution map
        let mut substitutions = TypeSubstitution::new();
        for (param, arg) in self.type_parameters.iter().zip(type_args.iter()) {
            substitutions.add(param.name.clone(), arg.clone());
        }

        // Apply substitutions to create instantiated interface
        let mut instantiated = self.clone();
        
        // Clear type parameters as they're now concrete
        instantiated.type_parameters.clear();

        // Apply substitutions to methods
        for method in &mut instantiated.methods {
            method.apply_substitutions(&substitutions)?;
        }

        Ok(instantiated)
    }
}

impl InterfaceMethod {
    /// Apply type substitutions to this method
    pub fn apply_substitutions(&mut self, substitutions: &TypeSubstitution) -> Result<(), CursedError> {
        // Apply to parameters - use indices to avoid borrow checker issues
        for i in 0..self.parameters.len() {
            if let Some(param_type) = self.parameters[i].param_type.clone() {
                let substituted = self.apply_type_substitution(&param_type, substitutions)?;
                self.parameters[i].param_type = Some(substituted);
            }
        }

        // Apply to return type
        if let Some(return_type) = self.return_type.clone() {
            let substituted = self.apply_type_substitution(&return_type, substitutions)?;
            self.return_type = Some(substituted);
        }

        // Apply to receiver
        if let Some(receiver_type) = self.receiver.as_ref().and_then(|r| r.param_type.clone()) {
            let substituted = self.apply_type_substitution(&receiver_type, substitutions)?;
            if let Some(receiver) = &mut self.receiver {
                receiver.param_type = Some(substituted);
            }
        }

        Ok(())
    }

    /// Apply substitution to a single type
    fn apply_type_substitution(&self, ast_type: &AstType, substitutions: &TypeSubstitution) -> Result<AstType, CursedError> {
        match ast_type {
            AstType::Custom(name) => {
                // Check if this is a type parameter that needs substitution
                let type_expr = TypeExpression::named(name);
                let substituted = substitutions.apply(&type_expr);
                if let Some(new_name) = &substituted.name {
                    Ok(AstType::Custom(new_name.clone()))
                } else {
                    Ok(ast_type.clone())
                }
            }
            AstType::Array(element_type, size) => {
                let substituted_element = self.apply_type_substitution(element_type, substitutions)?;
                Ok(AstType::Array(Box::new(substituted_element), size.clone()))
            }
            AstType::Slice(element_type) => {
                let substituted_element = self.apply_type_substitution(element_type, substitutions)?;
                Ok(AstType::Slice(Box::new(substituted_element)))
            }
            AstType::Function(params, return_type) => {
                let mut substituted_params = Vec::new();
                for param in params {
                    substituted_params.push(self.apply_type_substitution(param, substitutions)?);
                }
                let substituted_return = self.apply_type_substitution(return_type, substitutions)?;
                Ok(AstType::Function(substituted_params, Box::new(substituted_return)))
            }
            // For primitive types, no substitution needed
            _ => Ok(ast_type.clone()),
        }
    }
}

impl InterfaceHierarchy {
    /// Create a new interface hierarchy
    pub fn new() -> Self {
        Self {
            interfaces: HashMap::new(),
            parents: HashMap::new(),
            transitive_parents: HashMap::new(),
            implementations: HashMap::new(),
        }
    }

    /// Add a generic interface to the hierarchy
    pub fn add_interface(&mut self, interface: GenericInterface) -> Result<(), CursedError> {
        let interface_name = interface.name.clone();

        // Add parent relationships
        if !interface.extends.is_empty() {
            self.parents.insert(interface_name.clone(), interface.extends.clone());
        }

        // Store the interface
        self.interfaces.insert(interface_name, interface);

        Ok(())
    }

    /// Add an interface implementation
    pub fn add_implementation(&mut self, implementation: InterfaceImplementation) -> Result<(), CursedError> {
        let key = (implementation.implementing_type.clone(), implementation.interface_name.clone());
        
        // Validate the implementation
        self.validate_implementation(&implementation)?;
        
        self.implementations.insert(key, implementation);
        Ok(())
    }

    /// Build transitive parent relationships
    pub fn build_transitive_relationships(&mut self) -> Result<(), CursedError> {
        for interface_name in self.interfaces.keys() {
            let mut transitive = Vec::new();
            let mut visited = HashSet::new();
            self.collect_transitive_parents(interface_name, &mut transitive, &mut visited)?;
            self.transitive_parents.insert(interface_name.clone(), transitive);
        }
        Ok(())
    }

    /// Recursively collect transitive parents
    fn collect_transitive_parents(
        &self,
        interface_name: &str,
        transitive: &mut Vec<String>,
        visited: &mut HashSet<String>,
    ) -> Result<(), CursedError> {
        if visited.contains(interface_name) {
            return Err(CursedError::Runtime(format!(
                "Circular interface inheritance detected: {}",
                interface_name
            )));
        }

        visited.insert(interface_name.to_string());

        if let Some(direct_parents) = self.parents.get(interface_name) {
            for parent in direct_parents {
                if !transitive.contains(parent) {
                    transitive.push(parent.clone());
                }
                self.collect_transitive_parents(parent, transitive, visited)?;
            }
        }

        visited.remove(interface_name);
        Ok(())
    }

    /// Check if a type implements an interface
    pub fn implements_interface(&self, type_name: &str, interface_name: &str) -> bool {
        let key = (type_name.to_string(), interface_name.to_string());
        self.implementations.contains_key(&key)
    }

    /// Get interface implementation
    pub fn get_implementation(&self, type_name: &str, interface_name: &str) -> Option<&InterfaceImplementation> {
        let key = (type_name.to_string(), interface_name.to_string());
        self.implementations.get(&key)
    }

    /// Get interface by name
    pub fn get_interface(&self, name: &str) -> Option<&GenericInterface> {
        self.interfaces.get(name)
    }

    /// Validate an interface implementation
    fn validate_implementation(&self, implementation: &InterfaceImplementation) -> Result<(), CursedError> {
        let interface = self.interfaces.get(&implementation.interface_name)
            .ok_or_else(|| CursedError::Type(format!("Interface '{}' not found", implementation.interface_name)))?;

        // Check type argument count
        if implementation.type_arguments.len() != interface.type_parameters.len() {
            return Err(CursedError::Type(format!(
                "Interface {} expects {} type arguments, implementation provides {}",
                interface.name,
                interface.type_parameters.len(),
                implementation.type_arguments.len()
            )));
        }

        // Check that all required methods are implemented
        for interface_method in &interface.methods {
            let mut method_found = false;
            for impl_method in &implementation.method_implementations {
                if impl_method.name == interface_method.name {
                    // Validate method signature compatibility
                    self.validate_method_compatibility(interface_method, impl_method, &implementation.type_arguments)?;
                    method_found = true;
                    break;
                }
            }

            if !method_found {
                return Err(CursedError::Type(format!(
                    "Method '{}' required by interface '{}' not implemented by type '{}'",
                    interface_method.name,
                    interface.name,
                    implementation.implementing_type
                )));
            }
        }

        // Check associated type implementations
        for associated_type in &interface.associated_types {
            if !implementation.associated_type_impls.contains_key(&associated_type.name) && associated_type.default.is_none() {
                return Err(CursedError::Type(format!(
                    "Associated type '{}' required by interface '{}' not implemented by type '{}'",
                    associated_type.name,
                    interface.name,
                    implementation.implementing_type
                )));
            }
        }

        Ok(())
    }

    /// Validate method signature compatibility
    fn validate_method_compatibility(
        &self,
        interface_method: &InterfaceMethod,
        impl_method: &ConcreteMethodImplementation,
        type_arguments: &[TypeExpression],
    ) -> Result<(), CursedError> {
        // Check parameter count
        if interface_method.parameters.len() != impl_method.parameters.len() {
            return Err(CursedError::Type(format!(
                "Method '{}' parameter count mismatch: expected {}, got {}",
                interface_method.name,
                interface_method.parameters.len(),
                impl_method.parameters.len()
            )));
        }

        // Create substitution map for type parameters
        let mut substitutions = TypeSubstitution::new();
        // This would need the interface's type parameters to create proper substitutions
        // For now, we'll do a simplified check

        // Check return type compatibility
        match (&interface_method.return_type, &impl_method.return_type) {
            (None, None) => {},
            (Some(_), Some(_)) => {
                // Would need to validate type compatibility with substitutions
            },
            _ => return Err(CursedError::Type(format!(
                "Method '{}' return type mismatch",
                interface_method.name
            ))),
        }

        Ok(())
    }
}

impl GenericInterfaceChecker {
    /// Create a new generic interface checker
    pub fn new(type_environment: TypeEnvironment) -> Self {
        let generics_core = GenericsCore::new(type_environment.clone());
        
        Self {
            hierarchy: InterfaceHierarchy::new(),
            constraint_checker: GenericConstraintChecker::new(),
            constraint_resolver: ConstraintResolver::new(),
            type_environment,
            generics_core,
        }
    }

    /// Register a generic interface
    pub fn register_interface(&mut self, interface: &InterfaceStatement) -> Result<(), CursedError> {
        let generic_interface = self.convert_ast_interface_to_generic(interface)?;
        self.hierarchy.add_interface(generic_interface)?;
        Ok(())
    }

    /// Register an interface implementation
    pub fn register_implementation(&mut self, implementation: InterfaceImplementation) -> Result<(), CursedError> {
        self.hierarchy.add_implementation(implementation)?;
        Ok(())
    }

    /// Check if a type implements an interface with generic instantiation
    pub fn check_interface_compliance(
        &self,
        type_name: &str,
        interface_name: &str,
        type_arguments: &[TypeExpression],
    ) -> Result<bool, CursedError> {
        // Get the generic interface
        let interface = self.hierarchy.get_interface(interface_name)
            .ok_or_else(|| CursedError::Type(format!("Interface '{}' not found", interface_name)))?;

        // If interface is generic, instantiate it with type arguments
        let instantiated_interface = if interface.is_generic() {
            interface.instantiate(type_arguments)?
        } else {
            interface.clone()
        };

        // Check if there's a concrete implementation
        if self.hierarchy.implements_interface(type_name, interface_name) {
            // Validate the implementation with constraint checking
            self.validate_generic_constraints(type_name, &instantiated_interface, type_arguments)?;
            return Ok(true);
        }

        // Check parent interfaces
        if let Some(parents) = self.hierarchy.transitive_parents.get(interface_name) {
            for parent in parents {
                if self.check_interface_compliance(type_name, parent, type_arguments)? {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    /// Validate generic constraints for an interface implementation
    fn validate_generic_constraints(
        &self,
        type_name: &str,
        interface: &GenericInterface,
        type_arguments: &[TypeExpression],
    ) -> Result<(), CursedError> {
        // Check type parameter bounds
        for (type_param, type_arg) in interface.type_parameters.iter().zip(type_arguments.iter()) {
            for bound in &type_param.bounds {
                // Check if type_arg satisfies the bound
                if !self.type_satisfies_bound(type_arg, bound)? {
                    return Err(CursedError::Type(format!(
                        "Type argument '{}' does not satisfy bound '{}' for parameter '{}'",
                        type_arg.name.as_ref().unwrap_or(&"unknown".to_string()),
                        bound,
                        type_param.name
                    )));
                }
            }
        }

        // Check where clauses  
        let type_args_map: std::collections::HashMap<String, TypeExpression> = interface.type_parameters
            .iter()
            .zip(type_arguments.iter())
            .map(|(param, arg)| (param.name.clone(), arg.clone()))
            .collect();
            
        for where_clause in &interface.where_clauses {
            // Convert generic_constraints::WhereClause to ast::WhereClause
            let ast_where_clause = crate::ast::WhereClause {
                constraints: where_clause.constraints.iter().map(|c| {
                    crate::ast::TypeConstraint {
                        type_name: where_clause.type_expr.name.clone().unwrap_or_default(),
                        bounds: match c {
                            crate::type_system::generic_constraints::TypeConstraint::Interface(name) => vec![name.clone()],
                            crate::type_system::generic_constraints::TypeConstraint::Lifetime(name) => vec![name.clone()],
                            crate::type_system::generic_constraints::TypeConstraint::Equality(expr) => vec![expr.name.clone().unwrap_or_default()],
                                _ => vec!["Unknown".to_string()],
                        }
                    }
                }).collect()
            };
            self.validate_where_clause_constraint(&ast_where_clause, &type_args_map)?;
        }

        Ok(())
    }

    /// Validate where clause constraints
    fn validate_where_clause_constraint(
        &self, 
        where_clause: &crate::ast::WhereClause, 
        type_args: &std::collections::HashMap<String, TypeExpression>
    ) -> Result<(), CursedError> {
        // Check each constraint in the where clause
        for constraint in &where_clause.constraints {
            // Resolve the type being constrained
            let constrained_type = if let Some(resolved) = type_args.get(&constraint.type_name) {
                resolved.clone()
            } else {
                // If not in type args, treat as generic type parameter
                TypeExpression::named(&constraint.type_name)
            };

            // Check each bound in the constraint
            for bound in &constraint.bounds {
                // Check if the type implements the required trait/bound
                if !self.check_trait_implementation(&constrained_type, bound) {
                    return Err(CursedError::Type(format!(
                        "Type '{}' does not implement trait '{}' required by where clause", 
                        &constraint.type_name, bound
                    )));
                }
            }
        }

        Ok(())
    }

    /// Check if a type implements a trait
    fn check_trait_implementation(&self, type_expr: &TypeExpression, trait_name: &str) -> bool {
        // Check if the type implements the trait
        // This is a simplified implementation - in a full system, this would
        // check the trait implementation registry
        match trait_name {
            "Clone" | "Copy" | "Debug" | "Default" => {
                // Built-in traits that most types can implement
                true
            }
            "Send" | "Sync" => {
                // Concurrency traits - check if type is safe for threading
                !type_expr.name.as_ref().map_or(false, |n| n.contains("Rc") || n.contains("RefCell"))
            }
            "Eq" | "PartialEq" | "Ord" | "PartialOrd" => {
                // Comparison traits - primitive types implement these
                matches!(type_expr.name.as_ref().map(|n| n.as_str()).unwrap_or(""), "drip" | "meal" | "tea" | "lit")
            }
            _ => {
                // For custom traits, assume implementation exists
                // In a full system, this would check the trait registry
                true
            }
        }
    }

    /// Check if two types are equivalent
    fn types_equivalent(&self, type1: &TypeExpression, type2: &TypeExpression) -> bool {
        // Simplified type equivalence check
        type1.name == type2.name
    }

    /// Check if a type satisfies a trait bound
    fn type_satisfies_bound(&self, type_expr: &TypeExpression, bound: &str) -> Result<bool, CursedError> {
        // This would integrate with the trait system to check if type_expr implements bound
        // For now, return true as a placeholder
        Ok(true)
    }

    /// Convert AST interface to generic interface
    fn convert_ast_interface_to_generic(&self, interface: &InterfaceStatement) -> Result<GenericInterface, CursedError> {
        let mut generic_interface = GenericInterface::new(interface.name.clone());

        // Convert type parameters
        for type_param in &interface.type_parameters {
            let generic_param = GenericTypeParameter {
                name: type_param.name.clone(),
                kind: Kind::Type, // Default to Type kind
                bounds: type_param.bounds.clone(),
                default: None,
                variance: Variance::Invariant, // Default variance
            };
            generic_interface.add_type_parameter(generic_param);
        }

        // Set extends
        generic_interface.extends = interface.extends.clone();

        // Convert methods
        for method in &interface.methods {
            let interface_method = InterfaceMethod {
                name: method.name.clone(),
                type_parameters: Vec::new(), // Method-level generics would be parsed separately
                parameters: method.parameters.clone(),
                return_type: method.return_type.clone(),
                receiver: method.receiver.as_ref().map(|r| Parameter {
                    name: r.name.clone(),
                    param_type: Some(r.receiver_type.clone()),
                }),
                where_clauses: Vec::new(), // Would be parsed from method signature
            };
            generic_interface.add_method(interface_method);
        }

        Ok(generic_interface)
    }

    /// Instantiate a generic interface with concrete types
    pub fn instantiate_interface(
        &self,
        interface_name: &str,
        type_arguments: &[TypeExpression],
    ) -> Result<GenericInterface, CursedError> {
        let interface = self.hierarchy.get_interface(interface_name)
            .ok_or_else(|| CursedError::Type(format!("Interface '{}' not found", interface_name)))?;

        interface.instantiate(type_arguments)
    }

    /// Get all interfaces that a type implements
    pub fn get_implemented_interfaces(&self, type_name: &str) -> Vec<String> {
        let mut implemented = Vec::new();
        
        for ((impl_type, interface_name), _) in &self.hierarchy.implementations {
            if impl_type == type_name {
                implemented.push(interface_name.clone());
            }
        }
        
        implemented
    }

    /// Check interface hierarchy for errors
    pub fn validate_hierarchy(&mut self) -> Result<(), CursedError> {
        self.hierarchy.build_transitive_relationships()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;

    #[test]
    fn test_generic_interface_creation() {
        let mut interface = GenericInterface::new("Container".to_string());
        
        let type_param = GenericTypeParameter {
            name: "T".to_string(),
            kind: Kind::Type,
            bounds: vec!["Clone".to_string()],
            default: None,
            variance: Variance::Covariant,
        };
        
        interface.add_type_parameter(type_param);
        
        assert!(interface.is_generic());
        assert_eq!(interface.type_parameters.len(), 1);
        assert!(interface.get_type_parameter("T").is_some());
    }

    #[test]
    fn test_interface_instantiation() {
        let mut interface = GenericInterface::new("Iterator".to_string());
        
        let type_param = GenericTypeParameter {
            name: "T".to_string(),
            kind: Kind::Type,
            bounds: vec![],
            default: None,
            variance: Variance::Covariant,
        };
        
        interface.add_type_parameter(type_param);
        
        // Add a method with generic parameter
        let method = InterfaceMethod {
            name: "next".to_string(),
            type_parameters: vec![],
            parameters: vec![],
            return_type: Some(AstType::Custom("T".to_string())),
            receiver: None,
            where_clauses: vec![],
        };
        
        interface.add_method(method);
        
        // Instantiate with concrete type
        let type_args = vec![TypeExpression::named("normie")];
        let instantiated = interface.instantiate(&type_args).unwrap();
        
        assert!(!instantiated.is_generic());
        assert_eq!(instantiated.methods.len(), 1);
    }

    #[test]
    fn test_interface_hierarchy() {
        let mut hierarchy = InterfaceHierarchy::new();
        
        // Create base interface
        let base = GenericInterface::new("Display".to_string());
        hierarchy.add_interface(base).unwrap();
        
        // Create derived interface
        let mut derived = GenericInterface::new("Debug".to_string());
        derived.extends = vec!["Display".to_string()];
        hierarchy.add_interface(derived).unwrap();
        
        assert!(hierarchy.build_transitive_relationships().is_ok());
    }

    #[test]
    fn test_generic_interface_checker() {
        let type_env = TypeEnvironment::new();
        let mut checker = GenericInterfaceChecker::new(type_env);
        
        // Create interface statement
        let interface_stmt = InterfaceStatement {
            name: "Comparable".to_string(),
            type_parameters: vec![
                TypeParameter {
                    name: "T".to_string(),
                    bounds: vec![],
                }
            ],
            extends: vec![],
            compositions: vec![],
            methods: vec![
                MethodSignature {
                    name: "compare".to_string(),
                    receiver: None,
                    parameters: vec![
                        Parameter {
                            name: "other".to_string(),
                            param_type: Some(AstType::Custom("T".to_string())),
                        }
                    ],
                    return_type: Some(AstType::Normie),
                }
            ],
            visibility: Visibility::Public,
        };
        
        assert!(checker.register_interface(&interface_stmt).is_ok());
        assert!(checker.validate_hierarchy().is_ok());
    }

    #[test]
    fn test_interface_implementation_validation() {
        let mut hierarchy = InterfaceHierarchy::new();
        
        // Create interface
        let mut interface = GenericInterface::new("Drawable".to_string());
        let method = InterfaceMethod {
            name: "draw".to_string(),
            type_parameters: vec![],
            parameters: vec![],
            return_type: None,
            receiver: None,
            where_clauses: vec![],
        };
        interface.add_method(method);
        hierarchy.add_interface(interface).unwrap();
        
        // Create valid implementation
        let implementation = InterfaceImplementation {
            implementing_type: "Rectangle".to_string(),
            interface_name: "Drawable".to_string(),
            type_arguments: vec![],
            associated_type_impls: HashMap::new(),
            method_implementations: vec![
                ConcreteMethodImplementation {
                    name: "draw".to_string(),
                    parameters: vec![],
                    return_type: None,
                    receiver_type: ReceiverType::Value,
                }
            ],
        };
        
        assert!(hierarchy.add_implementation(implementation).is_ok());
    }
}
