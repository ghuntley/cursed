//! Higher-kinded type support for CURSED
//! 
//! Provides type constructors, generic containers and functors,
//! and advanced generic programming patterns.

use crate::error::CursedError;
use crate::type_system::{TypeExpression, TypeDefinition, TypeParameter, GenericConstraint};
use std::collections::HashMap;

/// Kind system for higher-kinded types
#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    /// Base kind * (concrete types)
    Type,
    /// Function kind * -> * (type constructor taking one type)
    TypeConstructor(Box<Kind>, Box<Kind>),
    /// Higher-order kind (* -> *) -> * (taking type constructor)
    HigherOrder(Box<Kind>, Box<Kind>),
    /// Polymorphic kind variable
    Variable(String),
}

/// Type constructor representation
#[derive(Debug, Clone)]
pub struct TypeConstructor {
    /// Name of the type constructor
    pub name: String,
    /// Kind of the constructor
    pub kind: Kind,
    /// Type parameters with their kinds
    pub parameters: Vec<KindedTypeParameter>,
    /// Constructor constraints
    pub constraints: Vec<HigherKindedConstraint>,
    /// Whether this is a built-in constructor
    pub is_builtin: bool,
}

/// Type parameter with kind annotation
#[derive(Debug, Clone)]
pub struct KindedTypeParameter {
    /// Parameter name
    pub name: String,
    /// Kind of the parameter
    pub kind: Kind,
    /// Additional constraints on the parameter
    pub bounds: Vec<HigherKindedConstraint>,
}

/// Constraints for higher-kinded types
#[derive(Debug, Clone)]
pub enum HigherKindedConstraint {
    /// Functor constraint F: * -> *
    Functor(String),
    /// Applicative constraint F: * -> *
    Applicative(String),
    /// Monad constraint M: * -> *
    Monad(String),
    /// Custom higher-kinded constraint
    Custom(String, Kind),
}

/// Higher-kinded type system manager
#[derive(Debug)]
pub struct HigherKindedTypeSystem {
    /// Registry of type constructors
    constructors: HashMap<String, TypeConstructor>,
    /// Kind inference context
    kind_context: KindInferenceContext,
    /// Type constructor instances
    instances: HashMap<String, Vec<TypeConstructorInstance>>,
}

/// Context for kind inference
#[derive(Debug, Default)]
struct KindInferenceContext {
    /// Kind variable assignments
    kind_vars: HashMap<String, Kind>,
    /// Next kind variable ID
    next_var_id: usize,
}

/// An instantiated type constructor
#[derive(Debug, Clone)]
pub struct TypeConstructorInstance {
    /// Base constructor
    pub constructor: String,
    /// Applied type arguments
    pub type_args: Vec<TypeExpression>,
    /// Resulting concrete type
    pub instantiated_type: TypeExpression,
    /// Kind of the result
    pub result_kind: Kind,
}

impl HigherKindedTypeSystem {
    pub fn new() -> Self {
        let mut system = Self {
            constructors: HashMap::new(),
            kind_context: KindInferenceContext::default(),
            instances: HashMap::new(),
        };
        
        system.register_builtin_constructors();
        system
    }

    /// Register built-in type constructors
    fn register_builtin_constructors(&mut self) {
        // Array: * -> *
        self.register_constructor(TypeConstructor {
            name: "Array".to_string(),
            kind: Kind::TypeConstructor(Box::new(Kind::Type), Box::new(Kind::Type)),
            parameters: vec![
                KindedTypeParameter {
                    name: "T".to_string(),
                    kind: Kind::Type,
                    bounds: Vec::new(),
                }
            ],
            constraints: Vec::new(),
            is_builtin: true,
        }).expect("Failed to register Array constructor");

        // Option: * -> *
        self.register_constructor(TypeConstructor {
            name: "Option".to_string(),
            kind: Kind::TypeConstructor(Box::new(Kind::Type), Box::new(Kind::Type)),
            parameters: vec![
                KindedTypeParameter {
                    name: "T".to_string(),
                    kind: Kind::Type,
                    bounds: Vec::new(),
                }
            ],
            constraints: vec![HigherKindedConstraint::Functor("Option".to_string())],
            is_builtin: true,
        }).expect("Failed to register Option constructor");

        // Result: * -> * -> *
        self.register_constructor(TypeConstructor {
            name: "Result".to_string(),
            kind: Kind::TypeConstructor(
                Box::new(Kind::Type),
                Box::new(Kind::TypeConstructor(Box::new(Kind::Type), Box::new(Kind::Type)))
            ),
            parameters: vec![
                KindedTypeParameter {
                    name: "T".to_string(),
                    kind: Kind::Type,
                    bounds: Vec::new(),
                },
                KindedTypeParameter {
                    name: "E".to_string(),
                    kind: Kind::Type,
                    bounds: Vec::new(),
                }
            ],
            constraints: vec![HigherKindedConstraint::Functor("Result".to_string())],
            is_builtin: true,
        }).expect("Failed to register Result constructor");
    }

    /// Register a new type constructor
    pub fn register_constructor(&mut self, constructor: TypeConstructor) -> Result<(), CursedError> {
        // Validate the constructor's kind
        self.validate_constructor_kind(&constructor)?;
        
        self.constructors.insert(constructor.name.clone(), constructor);
        Ok(())
    }

    /// Validate that a type constructor's kind is well-formed
    fn validate_constructor_kind(&self, constructor: &TypeConstructor) -> Result<(), CursedError> {
        // Check that parameter kinds are consistent with constructor kind
        let expected_arity = self.count_kind_arity(&constructor.kind);
        let actual_arity = constructor.parameters.len();
        
        if expected_arity != actual_arity {
            return Err(CursedError::type_error(&format!(
                "Kind arity mismatch for constructor {}: expected {}, got {}",
                constructor.name, expected_arity, actual_arity
            )));
        }
        
        Ok(())
    }

    /// Count the arity of a kind (number of type parameters)
    fn count_kind_arity(&self, kind: &Kind) -> usize {
        match kind {
            Kind::Type => 0,
            Kind::Variable(_) => 0,
            Kind::TypeConstructor(_, result) => 1 + self.count_kind_arity(result),
            Kind::HigherOrder(_, result) => 1 + self.count_kind_arity(result),
        }
    }

    /// Apply a type constructor to type arguments
    pub fn apply_constructor(&mut self, 
                           constructor_name: &str, 
                           type_args: Vec<TypeExpression>) -> Result<TypeExpression, CursedError> {
        
        let constructor = self.constructors.get(constructor_name)
            .ok_or_else(|| CursedError::type_error(&format!("Unknown type constructor: {}", constructor_name)))?
            .clone();

        // Check argument count
        if type_args.len() != constructor.parameters.len() {
            return Err(CursedError::type_error(&format!(
                "Constructor {} expects {} arguments, got {}",
                constructor_name, constructor.parameters.len(), type_args.len()
            )));
        }

        // Perform kind checking on arguments
        for (i, (arg, param)) in type_args.iter().zip(constructor.parameters.iter()).enumerate() {
            let arg_kind = self.infer_kind(arg)?;
            if !self.kind_matches(&arg_kind, &param.kind) {
                return Err(CursedError::type_error(&format!(
                    "Kind mismatch for argument {} of {}: expected {:?}, got {:?}",
                    i, constructor_name, param.kind, arg_kind
                )));
            }
        }

        // Create the instantiated type
        let instantiated_type = TypeExpression::generic(constructor_name, type_args.clone());
        
        // Record the instance
        let instance = TypeConstructorInstance {
            constructor: constructor_name.to_string(),
            type_args: type_args.clone(),
            instantiated_type: instantiated_type.clone(),
            result_kind: Kind::Type, // Fully applied constructors result in concrete types
        };
        
        self.instances.entry(constructor_name.to_string())
            .or_insert_with(Vec::new)
            .push(instance);

        Ok(instantiated_type)
    }

    /// Infer the kind of a type expression
    pub fn infer_kind(&mut self, type_expr: &TypeExpression) -> Result<Kind, CursedError> {
        match &type_expr.name {
            Some(name) => {
                // Check if it's a known constructor
                if let Some(constructor) = self.constructors.get(name) {
                    let constructor_kind = constructor.kind.clone();
                    if type_expr.parameters.is_empty() {
                        // Unapplied constructor
                        Ok(constructor_kind)
                    } else {
                        // Partially or fully applied constructor
                        self.infer_applied_kind(&constructor_kind, &type_expr.parameters)
                    }
                } else {
                    // Assume it's a concrete type
                    Ok(Kind::Type)
                }
            }
            None => {
                // Anonymous type (e.g., function type)
                Ok(Kind::Type)
            }
        }
    }

    /// Infer the kind of an applied type constructor
    fn infer_applied_kind(&mut self, constructor_kind: &Kind, args: &[TypeExpression]) -> Result<Kind, CursedError> {
        if args.is_empty() {
            return Ok(constructor_kind.clone());
        }

        match constructor_kind {
            Kind::TypeConstructor(param_kind, result_kind) => {
                if args.len() == 1 {
                    // Check first argument
                    let arg_kind = self.infer_kind(&args[0])?;
                    if self.kind_matches(&arg_kind, param_kind) {
                        Ok((**result_kind).clone())
                    } else {
                        Err(CursedError::type_error(&format!(
                            "Kind mismatch: expected {:?}, got {:?}", param_kind, arg_kind
                        )))
                    }
                } else {
                    // Multiple arguments - recurse
                    let first_result = self.infer_applied_kind(constructor_kind, &args[0..1])?;
                    self.infer_applied_kind(&first_result, &args[1..])
                }
            }
            Kind::HigherOrder(param_kind, result_kind) => {
                if args.len() >= 1 {
                    let arg_kind = self.infer_kind(&args[0])?;
                    if self.kind_matches(&arg_kind, param_kind) {
                        if args.len() == 1 {
                            Ok((**result_kind).clone())
                        } else {
                            self.infer_applied_kind(result_kind, &args[1..])
                        }
                    } else {
                        Err(CursedError::type_error(&format!(
                            "Higher-order kind mismatch: expected {:?}, got {:?}", param_kind, arg_kind
                        )))
                    }
                } else {
                    Ok(constructor_kind.clone())
                }
            }
            _ => Err(CursedError::type_error("Cannot apply non-constructor kind")),
        }
    }

    /// Check if two kinds match (with unification)
    fn kind_matches(&mut self, k1: &Kind, k2: &Kind) -> bool {
        match (k1, k2) {
            (Kind::Type, Kind::Type) => true,
            (Kind::Variable(v1), Kind::Variable(v2)) => v1 == v2,
            (Kind::Variable(v), k) | (k, Kind::Variable(v)) => {
                // Unify kind variable
                self.kind_context.kind_vars.insert(v.clone(), k.clone());
                true
            }
            (Kind::TypeConstructor(p1, r1), Kind::TypeConstructor(p2, r2)) => {
                self.kind_matches(p1, p2) && self.kind_matches(r1, r2)
            }
            (Kind::HigherOrder(p1, r1), Kind::HigherOrder(p2, r2)) => {
                self.kind_matches(p1, p2) && self.kind_matches(r1, r2)
            }
            _ => false,
        }
    }

    /// Check if a type satisfies a higher-kinded constraint
    pub fn check_constraint(&self, type_name: &str, constraint: &HigherKindedConstraint) -> Result<bool, CursedError> {
        match constraint {
            HigherKindedConstraint::Functor(expected) => {
                // Check if type_name implements Functor
                Ok(self.has_functor_instance(type_name))
            }
            HigherKindedConstraint::Applicative(expected) => {
                // Check if type_name implements Applicative (requires Functor)
                Ok(self.has_functor_instance(type_name) && self.has_applicative_instance(type_name))
            }
            HigherKindedConstraint::Monad(expected) => {
                // Check if type_name implements Monad (requires Applicative)
                Ok(self.has_functor_instance(type_name) && 
                   self.has_applicative_instance(type_name) && 
                   self.has_monad_instance(type_name))
            }
            HigherKindedConstraint::Custom(name, _kind) => {
                // Custom constraint checking would be implemented here
                Ok(false)
            }
        }
    }

    /// Get all instances of a type constructor
    pub fn get_instances(&self, constructor_name: &str) -> Option<&Vec<TypeConstructorInstance>> {
        self.instances.get(constructor_name)
    }

    /// Get a registered type constructor
    pub fn get_constructor(&self, name: &str) -> Option<&TypeConstructor> {
        self.constructors.get(name)
    }

    // Helper methods for constraint checking
    fn has_functor_instance(&self, type_name: &str) -> bool {
        // In a full implementation, this would check for Functor trait implementations
        matches!(type_name, "Option" | "Result" | "Array" | "List")
    }

    fn has_applicative_instance(&self, type_name: &str) -> bool {
        // Check for Applicative trait implementations
        matches!(type_name, "Option" | "Result")
    }

    fn has_monad_instance(&self, type_name: &str) -> bool {
        // Check for Monad trait implementations
        matches!(type_name, "Option" | "Result")
    }
}

/// Utility functions for higher-kinded types
pub mod hkt_utils {
    use super::*;

    /// Create a simple type constructor (* -> *)
    pub fn make_unary_constructor(name: &str) -> TypeConstructor {
        TypeConstructor {
            name: name.to_string(),
            kind: Kind::TypeConstructor(Box::new(Kind::Type), Box::new(Kind::Type)),
            parameters: vec![
                KindedTypeParameter {
                    name: "T".to_string(),
                    kind: Kind::Type,
                    bounds: Vec::new(),
                }
            ],
            constraints: Vec::new(),
            is_builtin: false,
        }
    }

    /// Create a binary type constructor (* -> * -> *)
    pub fn make_binary_constructor(name: &str) -> TypeConstructor {
        TypeConstructor {
            name: name.to_string(),
            kind: Kind::TypeConstructor(
                Box::new(Kind::Type),
                Box::new(Kind::TypeConstructor(Box::new(Kind::Type), Box::new(Kind::Type)))
            ),
            parameters: vec![
                KindedTypeParameter {
                    name: "T".to_string(),
                    kind: Kind::Type,
                    bounds: Vec::new(),
                },
                KindedTypeParameter {
                    name: "U".to_string(),
                    kind: Kind::Type,
                    bounds: Vec::new(),
                }
            ],
            constraints: Vec::new(),
            is_builtin: false,
        }
    }

    /// Format a kind for display
    pub fn format_kind(kind: &Kind) -> String {
        match kind {
            Kind::Type => "*".to_string(),
            Kind::Variable(v) => v.clone(),
            Kind::TypeConstructor(param, result) => {
                format!("{} -> {}", format_kind(param), format_kind(result))
            }
            Kind::HigherOrder(param, result) => {
                format!("({}) -> {}", format_kind(param), format_kind(result))
            }
        }
    }

    /// Check if a kind is higher-order
    pub fn is_higher_order(kind: &Kind) -> bool {
        matches!(kind, Kind::HigherOrder(_, _))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_constructors() {
        let system = HigherKindedTypeSystem::new();
        
        assert!(system.get_constructor("Array").is_some());
        assert!(system.get_constructor("Option").is_some());
        assert!(system.get_constructor("Result").is_some());
    }

    #[test]
    fn test_constructor_application() {
        let mut system = HigherKindedTypeSystem::new();
        
        let int_type = TypeExpression::named("int");
        let array_int = system.apply_constructor("Array", vec![int_type.clone()]).unwrap();
        
        assert_eq!(array_int.name, Some("Array".to_string()));
        assert_eq!(array_int.parameters.len(), 1);
        assert_eq!(array_int.parameters[0].name, Some("int".to_string()));
    }

    #[test]
    fn test_kind_inference() {
        let mut system = HigherKindedTypeSystem::new();
        
        let int_type = TypeExpression::named("int");
        let kind = system.infer_kind(&int_type).unwrap();
        assert_eq!(kind, Kind::Type);
        
        let array_constructor = TypeExpression::named("Array");
        let array_kind = system.infer_kind(&array_constructor).unwrap();
        assert!(matches!(array_kind, Kind::TypeConstructor(_, _)));
    }

    #[test]
    fn test_constraint_checking() {
        let system = HigherKindedTypeSystem::new();
        
        let functor_constraint = HigherKindedConstraint::Functor("Option".to_string());
        assert!(system.check_constraint("Option", &functor_constraint).unwrap());
        
        let monad_constraint = HigherKindedConstraint::Monad("Option".to_string());
        assert!(system.check_constraint("Option", &monad_constraint).unwrap());
    }

    #[test]
    fn test_kind_utilities() {
        let unary = hkt_utils::make_unary_constructor("List");
        assert_eq!(unary.parameters.len(), 1);
        assert!(matches!(unary.kind, Kind::TypeConstructor(_, _)));
        
        let binary = hkt_utils::make_binary_constructor("Map");
        assert_eq!(binary.parameters.len(), 2);
        
        let kind_str = hkt_utils::format_kind(&Kind::TypeConstructor(
            Box::new(Kind::Type), 
            Box::new(Kind::Type)
        ));
        assert_eq!(kind_str, "* -> *");
    }
}
