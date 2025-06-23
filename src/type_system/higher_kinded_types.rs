//! Higher-Kinded Types Implementation for CURSED Language
//!
//! This module provides support for type constructors and higher-order generic functions,
//! enabling advanced generic programming patterns like functors and monads.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use tracing::{debug, error, info, warn, instrument};

use crate::ast::crate::types::Type;
use crate::ast::traits::TypeParameter;
use crate::error::CursedError;

/// Represents the kind of a type (e.g., *, * -> *, * -> * -> *)
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Kind {
    /// A concrete type (*)
    Star,
    /// A type constructor that takes one type parameter (* -> *)
    Arrow(Box<Kind>, Box<Kind>),
    /// A higher-order type constructor
    HigherOrder(Vec<Kind>, Box<Kind>),
}

impl Kind {
    /// Create a type constructor kind (* -> *)
    pub fn type_constructor() -> Self {
        Kind::Arrow(Box::new(Kind::Star), Box::new(Kind::Star))
    }

    /// Create a higher-order type constructor kind (* -> * -> *)
    pub fn binary_type_constructor() -> Self {
        Kind::Arrow(
            Box::new(Kind::Star),
            Box::new(Kind::Arrow(Box::new(Kind::Star), Box::new(Kind::Star)))
        )
    }

    /// Get the arity (number of type parameters) of this kind
    pub fn arity(&self) -> usize {
        match self {
            Kind::Star => 0,
            Kind::Arrow(_, result) => 1 + result.arity(),
            Kind::HigherOrder(params, _) => params.len(),
        }
    }

    /// Check if this kind can be applied to another kind
    pub fn can_apply_to(&self, arg_kind: &Kind) -> bool {
        match self {
            Kind::Arrow(param_kind, _) => param_kind.as_ref() == arg_kind,
            Kind::HigherOrder(param_kinds, _) => {
                param_kinds.first().map_or(false, |first| first == arg_kind)
            }
            Kind::Star => false,
        }
    }

    /// Apply this kind to an argument kind, returning the result kind
    pub fn apply(&self, arg_kind: &Kind) -> Result<(), Error> {
        match self {
            Kind::Arrow(param_kind, result_kind) => {
                if param_kind.as_ref() == arg_kind {
                    Ok(result_kind.as_ref().clone())
                } else {
                    Err(CursedError::type_error(format!(
                        "Cannot apply kind {:?} to argument kind {:?}",
                        self, arg_kind
                    )))
                }
            }
            Kind::HigherOrder(param_kinds, result_kind) => {
                if let Some(first_param) = param_kinds.first() {
                    if first_param == arg_kind {
                        let remaining_params = param_kinds[1..].to_vec();
                        if remaining_params.is_empty() {
                            Ok(result_kind.as_ref().clone())
                        } else {
                            Ok(Kind::HigherOrder(remaining_params, result_kind.clone()))
                        }
                    } else {
                        Err(CursedError::type_error(format!(
                            "Cannot apply higher-order kind {:?} to argument kind {:?}",
                            self, arg_kind
                        )))
                    }
                } else {
                    Err(CursedError::type_error("Cannot apply kind with no parameters".to_string()))
                }
            }
            Kind::Star => Err(CursedError::type_error("Cannot apply concrete type".to_string())),
        }
    }
}

/// Represents a type constructor (e.g., Option, List, Map)
#[derive(Debug, Clone, PartialEq)]
pub struct TypeConstructor {
    /// Name of the type constructor
    pub name: String,
    /// Kind of the type constructor
    pub kind: Kind,
    /// Type parameters and their kinds
    pub type_parameters: Vec<(String, Kind)>,
    /// Documentation for the type constructor
    pub documentation: Option<String>,
}

/// Represents a higher-kinded type parameter (e.g., F in forall F<_>)
#[derive(Debug, Clone, PartialEq)]
pub struct HigherKindedTypeParameter {
    /// Name of the type parameter
    pub name: String,
    /// Kind of the type parameter
    pub kind: Kind,
    /// Constraints on the type parameter
    pub constraints: Vec<HigherKindedConstraint>,
}

/// Constraints that can be applied to higher-kinded type parameters
#[derive(Debug, Clone, PartialEq)]
pub enum HigherKindedConstraint {
    /// Must implement a specific type class (e.g., Functor, Monad)
    TypeClass(String),
    /// Must have a specific kind signature
    KindConstraint(Kind),
    /// Must be applicable to certain types
    Applicability(Vec<Kind>),
}

/// Registry for managing higher-kinded types and type constructors
#[derive(Debug)]
pub struct HigherKindedTypeRegistry {
    /// Map from type constructor name to its definition
    type_constructors: RwLock<HashMap<String, TypeConstructor>>,
    /// Map from type to its kind
    kind_cache: RwLock<HashMap<Type, Kind>>,
    /// Registry of type classes and their instances
    type_classes: RwLock<HashMap<String, TypeClass>>,
}

/// Represents a type class (e.g., Functor, Monad)
#[derive(Debug, Clone)]
pub struct TypeClass {
    pub name: String,
    pub kind_signature: Kind,
    pub required_methods: Vec<String>,
    pub instances: HashSet<String>, // Type constructor names that implement this class
}

impl HigherKindedTypeRegistry {
    /// Create a new higher-kinded type registry
    #[instrument]
    pub fn new() -> Self {
        debug!("Creating new HigherKindedTypeRegistry");
        let mut registry = Self {
            type_constructors: RwLock::new(HashMap::new()),
            kind_cache: RwLock::new(HashMap::new()),
            type_classes: RwLock::new(HashMap::new()),
        };

        // Register common type constructors
        if let Err(e) = registry.register_builtin_type_constructors() {
            error!("Failed to register builtin type constructors: {}", e);
        }

        // Register common type classes
        if let Err(e) = registry.register_builtin_type_classes() {
            error!("Failed to register builtin type classes: {}", e);
        }

        registry
    }

    /// Register a type constructor
    #[instrument(skip(self))]
    pub fn register_type_constructor(&self, constructor: TypeConstructor) -> Result<(), Error> {
        debug!("Registering type constructor: {}", constructor.name);
        
        // Validate the type constructor
        self.validate_type_constructor(&constructor)?;

        let mut constructors = self.type_constructors.write()
            .map_err(|_| CursedError::system_error("Failed to acquire write lock"))?;
        
        constructors.insert(constructor.name.clone(), constructor);
        info!("Successfully registered type constructor");
        Ok(())
    }

    /// Get a type constructor by name
    #[instrument(skip(self))]
    pub fn get_type_constructor(&self, name: &str) -> Result<(), Error> {
        let constructors = self.type_constructors.read()
            .map_err(|_| CursedError::system_error("Failed to acquire read lock"))?;
        
        Ok(constructors.get(name).cloned())
    }

    /// Infer the kind of a type
    #[instrument(skip(self))]
    pub fn infer_kind(&self, type_ref: &Type) -> Result<(), Error> {
        // Check cache first
        {
            let cache = self.kind_cache.read()
                .map_err(|_| CursedError::system_error("Failed to acquire read lock"))?;
            if let Some(kind) = cache.get(type_ref) {
                debug!("Found cached kind for type: {:?}", type_ref);
                return Ok(kind.clone());
            }
        }

        // Infer the kind
        let kind = self.infer_kind_impl(type_ref)?;

        // Cache the result
        {
            let mut cache = self.kind_cache.write()
                .map_err(|_| CursedError::system_error("Failed to acquire write lock"))?;
            cache.insert(type_ref.clone(), kind.clone());
        }

        debug!("Inferred kind {:?} for type {:?}", kind, type_ref);
        Ok(kind)
    }

    /// Internal implementation for kind inference
    #[instrument(skip(self))]
    fn infer_kind_impl(&self, type_ref: &Type) -> Result<(), Error> {
        match type_ref {
            // Basic types have kind *
            Type::Integer | Type::Float | Type::String | Type::Boolean | Type::Character => {
                Ok(Kind::Star)
            }

            // Generic types depend on their constructor
            Type::Generic(name) => {
                if let Some(constructor) = self.get_type_constructor(name)? {
                    // For simplified Generic variant, assume it's a fully applied type
                    Ok(Kind::Star)
                } else {
                    // Unknown type constructor, assume it's a type parameter with kind *
                    Ok(Kind::Star)
                }
            }

            // Function types have kind *
            Type::Function { .. } => Ok(Kind::Star),

            // Array types have kind *
            Type::Array(_) => Ok(Kind::Star),

            // Tuple types have kind *
            Type::Tuple(_) => Ok(Kind::Star),

            // Interface types have kind *
            Type::Interface(_) => Ok(Kind::Star),

            // Channel types have kind *
            Type::Channel(_) => Ok(Kind::Star),

            // Associated type projections have kind *
            Type::AssociatedTypeProjection { .. } => Ok(Kind::Star),

            // Nil type has kind *
            Type::Nil => Ok(Kind::Star),

            // Any type has kind *
            Type::Any => Ok(Kind::Star),

            // Struct types have kind *
            Type::Struct(_) => Ok(Kind::Star),

            // Primitive types have kind *
            Type::Primitive(_) => Ok(Kind::Star),

            // Map types have kind *
            Type::Map(_, _) => Ok(Kind::Star),

            // Type parameters have kind *
            Type::Parameter(_) => Ok(Kind::Star),

            // Type constructors have their defined kinds
            Type::Constructor { name, arity: _ } => {
                if let Some(constructor) = self.get_type_constructor(name)? {
                    Ok(constructor.kind.clone())
                } else {
                    Ok(Kind::Star)
                }
            }

            // Type applications have kind *
            Type::Application { .. } => Ok(Kind::Star),
        }
    }

    /// Compute the kind of a partially applied type constructor
    #[instrument(skip(self))]
    fn compute_partial_application_kind(&self, constructor_kind: &Kind, applied_args: usize) -> Result<(), Error> {
        let mut current_kind = constructor_kind.clone();
        
        for _ in 0..applied_args {
            match current_kind {
                Kind::Arrow(_, result) => {
                    current_kind = *result;
                }
                Kind::HigherOrder(params, result) => {
                    if params.len() > 1 {
                        current_kind = Kind::HigherOrder(params[1..].to_vec(), result);
                    } else {
                        current_kind = *result;
                    }
                }
                Kind::Star => {
                    return Err(CursedError::type_error("Cannot apply arguments to concrete type".to_string()));
                }
            }
        }

        Ok(current_kind)
    }

    /// Check if a type constructor implements a type class
    #[instrument(skip(self))]
    pub fn implements_type_class(&self, constructor_name: &str, class_name: &str) -> Result<(), Error> {
        let type_classes = self.type_classes.read()
            .map_err(|_| CursedError::system_error("Failed to acquire read lock"))?;
        
        if let Some(type_class) = type_classes.get(class_name) {
            Ok(type_class.instances.contains(constructor_name))
        } else {
            Ok(false)
        }
    }

    /// Register a type class instance
    #[instrument(skip(self))]
    pub fn register_type_class_instance(&self, constructor_name: &str, class_name: &str) -> Result<(), Error> {
        let mut type_classes = self.type_classes.write()
            .map_err(|_| CursedError::system_error("Failed to acquire write lock"))?;
        
        if let Some(type_class) = type_classes.get_mut(class_name) {
            type_class.instances.insert(constructor_name.to_string());
            info!("Registered {} as instance of {}", constructor_name, class_name);
            Ok(())
        } else {
            Err(CursedError::type_error(format!("Type class {} not found", class_name)))
        }
    }

    /// Validate a type constructor definition
    #[instrument(skip(self))]
    fn validate_type_constructor(&self, constructor: &TypeConstructor) -> Result<(), Error> {
        // Validate name
        if constructor.name.is_empty() {
            return Err(CursedError::type_error("Type constructor name cannot be empty".to_string()));
        }

        // Validate that the kind matches the number of type parameters
        let expected_arity = constructor.type_parameters.len();
        let actual_arity = constructor.kind.arity();
        
        if expected_arity != actual_arity {
            return Err(CursedError::type_error(format!(
                "Type constructor {} has {} type parameters but kind has arity {}",
                constructor.name, expected_arity, actual_arity
            )));
        }

        Ok(())
    }

    /// Register built-in type constructors
    #[instrument(skip(self))]
    fn register_builtin_type_constructors(&self) -> Result<(), Error> {
        // Option<T>
        let option_constructor = TypeConstructor {
            name: "Option".to_string(),
            kind: Kind::type_constructor(),
            type_parameters: vec![("T".to_string(), Kind::Star)],
            documentation: Some("Optional value type".to_string()),
        };
        self.register_type_constructor(option_constructor)?;

        // List<T>
        let list_constructor = TypeConstructor {
            name: "List".to_string(),
            kind: Kind::type_constructor(),
            type_parameters: vec![("T".to_string(), Kind::Star)],
            documentation: Some("Dynamic list type".to_string()),
        };
        self.register_type_constructor(list_constructor)?;

        // Map<K, V>
        let map_constructor = TypeConstructor {
            name: "Map".to_string(),
            kind: Kind::binary_type_constructor(),
            type_parameters: vec![
                ("K".to_string(), Kind::Star),
                ("V".to_string(), Kind::Star),
            ],
            documentation: Some("Key-value map type".to_string()),
        };
        self.register_type_constructor(map_constructor)?;

        // Result<T, E>
        let result_constructor = TypeConstructor {
            name: "Result".to_string(),
            kind: Kind::binary_type_constructor(),
            type_parameters: vec![
                ("T".to_string(), Kind::Star),
                ("E".to_string(), Kind::Star),
            ],
            documentation: Some("Result type for error handling".to_string()),
        };
        self.register_type_constructor(result_constructor)?;

        info!("Registered built-in type constructors");
        Ok(())
    }

    /// Register built-in type classes
    #[instrument(skip(self))]
    fn register_builtin_type_classes(&self) -> Result<(), Error> {
        let mut type_classes = self.type_classes.write()
            .map_err(|_| CursedError::system_error("Failed to acquire write lock"))?;

        // Functor type class
        let functor = TypeClass {
            name: "Functor".to_string(),
            kind_signature: Kind::type_constructor(),
            required_methods: vec!["map".to_string()],
            instances: HashSet::new(),
        };
        type_classes.insert("Functor".to_string(), functor);

        // Monad type class
        let monad = TypeClass {
            name: "Monad".to_string(),
            kind_signature: Kind::type_constructor(),
            required_methods: vec!["bind".to_string(), "return".to_string()],
            instances: HashSet::new(),
        };
        type_classes.insert("Monad".to_string(), monad);

        // Applicative type class
        let applicative = TypeClass {
            name: "Applicative".to_string(),
            kind_signature: Kind::type_constructor(),
            required_methods: vec!["apply".to_string(), "pure".to_string()],
            instances: HashSet::new(),
        };
        type_classes.insert("Applicative".to_string(), applicative);

        info!("Registered built-in type classes");
        Ok(())
    }

    /// Get statistics about the registry
    #[instrument(skip(self))]
    pub fn get_statistics(&self) -> Result<(), Error> {
        let constructors = self.type_constructors.read()
            .map_err(|_| CursedError::system_error("Failed to acquire read lock"))?;
        let kind_cache = self.kind_cache.read()
            .map_err(|_| CursedError::system_error("Failed to acquire read lock"))?;
        let type_classes = self.type_classes.read()
            .map_err(|_| CursedError::system_error("Failed to acquire read lock"))?;

        let total_instances: usize = type_classes.values()
            .map(|tc| tc.instances.len())
            .sum();

        Ok(HigherKindedTypeStatistics {
            total_type_constructors: constructors.len(),
            cached_kinds: kind_cache.len(),
            total_type_classes: type_classes.len(),
            total_type_class_instances: total_instances,
        })
    }
}

/// Statistics about the higher-kinded type registry
#[derive(Debug, Clone)]
pub struct HigherKindedTypeStatistics {
    pub total_type_constructors: usize,
    pub cached_kinds: usize,
    pub total_type_classes: usize,
    pub total_type_class_instances: usize,
}

/// Trait for working with higher-kinded types
pub trait HigherKindedTypeHandler {
    /// Apply a type constructor to type arguments
    fn apply_type_constructor(&self, constructor_name: &str, type_args: &[Type]) -> Result<(), Error>;
    
    /// Check if a type is a higher-kinded type
    fn is_higher_kinded(&self, type_ref: &Type) -> Result<(), Error>;
    
    /// Get the type constructor name from a generic type
    fn get_constructor_name(&self, type_ref: &Type) -> Option<String>;
}

impl HigherKindedTypeHandler for HigherKindedTypeRegistry {
    #[instrument(skip(self))]
    fn apply_type_constructor(&self, constructor_name: &str, type_args: &[Type]) -> Result<(), Error> {
        if let Some(constructor) = self.get_type_constructor(constructor_name)? {
            if type_args.len() == constructor.type_parameters.len() {
                // For simplified Generic variant, just use the constructor name
                Ok(Type::Generic(constructor_name.to_string()))
            } else {
                Err(CursedError::type_error(format!(
                    "Type constructor {} expects {} arguments, got {}",
                    constructor_name, constructor.type_parameters.len(), type_args.len()
                )))
            }
        } else {
            Err(CursedError::type_error(format!(
                "Unknown type constructor: {}", constructor_name
            )))
        }
    }

    #[instrument(skip(self))]
    fn is_higher_kinded(&self, type_ref: &Type) -> Result<(), Error> {
        let kind = self.infer_kind(type_ref)?;
        Ok(matches!(kind, Kind::Arrow(_, _) | Kind::HigherOrder(_, _)))
    }

    #[instrument(skip(self))]
    fn get_constructor_name(&self, type_ref: &Type) -> Option<String> {
        match type_ref {
            Type::Generic(name) => Some(name.clone()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kind_creation() {
        let star = Kind::Star;
        assert_eq!(star.arity(), 0);

        let type_constructor = Kind::type_constructor();
        assert_eq!(type_constructor.arity(), 1);

        let binary_constructor = Kind::binary_type_constructor();
        assert_eq!(binary_constructor.arity(), 2);
    }

    #[test]
    fn test_kind_application() {
        let type_constructor = Kind::type_constructor();
        let star = Kind::Star;

        let result = type_constructor.apply(&star);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Kind::Star);
    }

    #[test]
    fn test_type_constructor_registration() {
        let registry = HigherKindedTypeRegistry::new();
        
        let option_constructor = TypeConstructor {
            name: "MyOption".to_string(),
            kind: Kind::type_constructor(),
            type_parameters: vec![("T".to_string(), Kind::Star)],
            documentation: None,
        };

        let result = registry.register_type_constructor(option_constructor);
        assert!(result.is_ok());

        let retrieved = registry.get_type_constructor("MyOption").unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "MyOption");
    }

    #[test]
    fn test_kind_inference() {
        let registry = HigherKindedTypeRegistry::new();

        // Basic types should have kind *
        let int_kind = registry.infer_kind(&Type::Integer).unwrap();
        assert_eq!(int_kind, Kind::Star);

        // Test with a built-in type constructor
        let option_type = Type::Generic("Option".to_string());
        let option_kind = registry.infer_kind(&option_type).unwrap();
        assert_eq!(option_kind, Kind::Star);
    }

    #[test]
    fn test_type_class_registration() {
        let registry = HigherKindedTypeRegistry::new();

        let result = registry.register_type_class_instance("Option", "Functor");
        assert!(result.is_ok());

        let implements = registry.implements_type_class("Option", "Functor").unwrap();
        assert!(implements);
    }

    #[test]
    fn test_statistics() {
        let registry = HigherKindedTypeRegistry::new();
        let stats = registry.get_statistics().unwrap();

        // Should have built-in type constructors and type classes
        assert!(stats.total_type_constructors > 0);
        assert!(stats.total_type_classes > 0);
    }
}
