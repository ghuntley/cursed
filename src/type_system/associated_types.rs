use crate::error::Error;
//! Associated Types Implementation for CURSED Language
//! 
//! This module provides support for associated types in interfaces,
//! enabling more expressive and flexible generic programming patterns.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use tracing::{debug, error, info, warn, instrument};

use crate::ast::crate::types::Type;
use crate::ast::traits::TypeParameter;
use crate::ast::declarations::GenericConstraint;
use crate::error::CursedError;
use crate::type_system::constraint_resolver::ConstraintResolver;

/// Represents an associated type definition within an interface
#[derive(Debug, Clone, PartialEq)]
pub struct AssociatedType {
    /// Name of the associated type
    pub name: String,
    /// Optional constraints on the associated type
    pub constraints: Vec<GenericConstraint>,
    /// Default type if provided
    pub default_type: Option<Type>,
    /// Documentation for the associated type
    pub documentation: Option<String>,
}

/// Represents an associated type projection (e.g., Iterator::Item)
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct AssociatedTypeProjection {
    /// The base type that implements the interface
    pub base_type: Type,
    /// The interface name containing the associated type
    pub interface_name: String,
    /// The associated type name
    pub associated_type_name: String,
}

/// Registry for managing associated type definitions and projections
#[derive(Debug)]
pub struct AssociatedTypeRegistry {
    /// Map from interface name to its associated types
    interface_associated_types: RwLock<HashMap<String, Vec<AssociatedType>>>,
    /// Cache of resolved projections
    projection_cache: RwLock<HashMap<AssociatedTypeProjection, Type>>,
    /// Constraint resolver for validation
    constraint_resolver: Arc<ConstraintResolver>,
}

impl AssociatedTypeRegistry {
    /// Create a new associated type registry
    #[instrument]
    pub fn new(constraint_resolver: Arc<ConstraintResolver>) -> Self {
        debug!("Creating new AssociatedTypeRegistry");
        Self {
            interface_associated_types: RwLock::new(HashMap::new()),
            projection_cache: RwLock::new(HashMap::new()),
            constraint_resolver,
        }
    }

    /// Register associated types for an interface
    #[instrument(skip(self))]
    pub fn register_interface_associated_types(
        &self,
        interface_name: &str,
        associated_types: Vec<AssociatedType>,
    ) -> Result<(), Error> {
        debug!("Registering {} associated types for interface {}", 
               associated_types.len(), interface_name);

        // Validate each associated type
        for assoc_type in &associated_types {
            self.validate_associated_type(assoc_type)?;
        }

        let mut registry = self.interface_associated_types.write()
            .map_err(|_| CursedError::system_error("Failed to acquire write lock"))?;
        
        registry.insert(interface_name.to_string(), associated_types);
        info!("Successfully registered associated types for interface {}", interface_name);
        Ok(())
    }

    /// Get associated types for an interface
    #[instrument(skip(self))]
    pub fn get_interface_associated_types(&self, interface_name: &str) -> Result<(), Error> {
        let registry = self.interface_associated_types.read()
            .map_err(|_| CursedError::system_error("Failed to acquire read lock"))?;
        
        Ok(registry.get(interface_name).cloned().unwrap_or_default())
    }

    /// Resolve an associated type projection
    #[instrument(skip(self))]
    pub fn resolve_projection(&self, projection: &AssociatedTypeProjection) -> Result<(), Error> {
        // Check cache first
        {
            let cache = self.projection_cache.read()
                .map_err(|_| CursedError::system_error("Failed to acquire read lock"))?;
            if let Some(resolved_type) = cache.get(projection) {
                debug!("Found cached projection for {}.{}", 
                       projection.interface_name, projection.associated_type_name);
                return Ok(resolved_type.clone());
            }
        }

        // Resolve the projection
        let resolved_type = self.resolve_projection_impl(projection)?;

        // Cache the result
        {
            let mut cache = self.projection_cache.write()
                .map_err(|_| CursedError::system_error("Failed to acquire write lock"))?;
            cache.insert(projection.clone(), resolved_type.clone());
        }

        info!("Resolved projection {}.{} to {:?}", 
              projection.interface_name, projection.associated_type_name, resolved_type);
        Ok(resolved_type)
    }

    /// Internal implementation for resolving projections
    #[instrument(skip(self))]
    fn resolve_projection_impl(&self, projection: &AssociatedTypeProjection) -> Result<(), Error> {
        // Get the associated type definition
        let associated_types = self.get_interface_associated_types(&projection.interface_name)?;
        let assoc_type = associated_types.iter()
            .find(|at| at.name == projection.associated_type_name)
            .ok_or_else(|| CursedError::type_error(format!(
                "Associated type {} not found in interface {}",
                projection.associated_type_name, projection.interface_name
            )))?;

        // For now, use the default type if available, otherwise create a placeholder
        if let Some(default_type) = &assoc_type.default_type {
            Ok(default_type.clone())
        } else {
            // Create a projection type that will be resolved later
            Ok(Type::AssociatedTypeProjection {
                base_type: Box::new(projection.base_type.clone()),
                interface_name: projection.interface_name.clone(),
                associated_type_name: projection.associated_type_name.clone(),
            })
        }
    }

    /// Validate an associated type definition
    #[instrument(skip(self))]
    fn validate_associated_type(&self, assoc_type: &AssociatedType) -> Result<(), Error> {
        // Validate name
        if assoc_type.name.is_empty() {
            return Err(CursedError::type_error("Associated type name cannot be empty".to_string()));
        }

        // Validate constraints
        for constraint in &assoc_type.constraints {
            self.validate_constraint(constraint)?;
        }

        // Validate default type if present
        if let Some(default_type) = &assoc_type.default_type {
            self.validate_default_type(assoc_type, default_type)?;
        }

        Ok(())
    }

    /// Validate a constraint on an associated type
    #[instrument(skip(self))]
    fn validate_constraint(&self, constraint: &GenericConstraint) -> Result<(), Error> {
        // Use the constraint resolver to validate the constraint
        self.constraint_resolver.validate_constraint(constraint, &crate::type_system::TypeEnvironment::new())
            .map_err(|e| CursedError::type_error(format!("Constraint validation failed: {:?}", e)))?;
        
        // Validate that constraint name is not empty
        if constraint.constraint_name.is_empty() {
            return Err(CursedError::type_error("Constraint name cannot be empty".to_string()));
        }
        
        // Validate that type parameters are not empty
        if constraint.type_parameters.is_empty() {
            return Err(CursedError::type_error("Constraint must have at least one type parameter".to_string()));
        }
        
        debug!("Successfully validated constraint: {}", constraint.constraint_name);
        Ok(())
    }

    /// Validate that a default type satisfies the associated type's constraints
    #[instrument(skip(self))]
    fn validate_default_type(&self, assoc_type: &AssociatedType, default_type: &Type) -> Result<(), Error> {
        // Check that the default type satisfies all constraints
        for constraint in &assoc_type.constraints {
            // Verify that the default type implements the required constraint
            if !self.type_implements_interface(default_type, &constraint.constraint_name)? {
                return Err(CursedError::type_error(format!(
                    "Default type {:?} does not implement required constraint {}",
                    default_type, constraint.constraint_name
                )));
            }
        }
        Ok(())
    }

    /// Check if a type implements an interface (proper implementation)
    #[instrument(skip(self))]
    fn type_implements_interface(&self, type_ref: &Type, interface_name: &str) -> Result<(), Error> {
        match type_ref {
            Type::Interface(interface_type) => {
                // Check if this interface is the same or extends the required interface
                Ok(interface_type.name == interface_name)
            }
            Type::Struct(struct_type) => {
                // Check if struct implements the required interface
                // This would require lookup in type environment for impl blocks
                // For now, check against common interfaces
                match interface_name {
                    "Clone" | "Debug" | "Display" | "PartialEq" | "Eq" => {
                        // Common derivable traits - assume implemented
                        Ok(true)
                    }
                    "Iterator" => {
                        // Check if struct has iterator methods
                        Ok(struct_type.fields.iter().any(|field| field == "next" || field == "item"))
                    }
                    "Collection" => {
                        // Check if struct has collection methods
                        Ok(struct_type.fields.iter().any(|field| field == "len" || field == "get"))
                    }
                    _ => {
                        warn!("Unknown interface '{}' for struct '{}'", interface_name, struct_type.name);
                        Ok(false)
                    }
                }
            }
            Type::Primitive(primitive_name) => {
                // Primitive types implement certain interfaces
                match (primitive_name.as_str(), interface_name) {
                    ("normie" | "facts" | "tea" | "based", "Clone") => Ok(true),
                    ("normie" | "facts" | "tea" | "based", "Debug") => Ok(true),
                    ("normie" | "facts", "PartialEq") => Ok(true),
                    ("normie" | "facts", "Eq") => Ok(true),
                    ("tea", "Display") => Ok(true),
                    _ => Ok(false)
                }
            }
            Type::Array(element_type) => {
                // Array implements interface if element type implements it
                match interface_name {
                    "Clone" | "Debug" => {
                        self.type_implements_interface(element_type, interface_name)
                    }
                    "Iterator" => Ok(true), // Arrays are always iterable
                    "Collection" => Ok(true), // Arrays are collections
                    _ => Ok(false)
                }
            }
            Type::Map(key_type, value_type) => {
                // Map implements interface if both key and value types implement it
                match interface_name {
                    "Clone" | "Debug" => {
                        let key_impl = self.type_implements_interface(key_type, interface_name)?;
                        let value_impl = self.type_implements_interface(value_type, interface_name)?;
                        Ok(key_impl && value_impl)
                    }
                    "Collection" => Ok(true), // Maps are collections
                    _ => Ok(false)
                }
            }
            Type::Channel(element_type) => {
                // Channel implements interface based on element type
                match interface_name {
                    "Clone" => Ok(false), // Channels are not cloneable
                    "Debug" => self.type_implements_interface(element_type, interface_name),
                    _ => Ok(false)
                }
            }
            Type::Function(params, return_type) => {
                // Functions implement limited interfaces
                match interface_name {
                    "Clone" => Ok(true), // Function pointers are cloneable
                    "Debug" => Ok(true), // Functions can be debugged
                    _ => Ok(false)
                }
            }
            Type::Generic(type_name) => {
                // Generic types depend on their bounds - for now assume false
                warn!("Cannot determine interface implementation for unresolved generic type '{}'", type_name);
                Ok(false)
            }
            Type::Integer | Type::String | Type::Boolean | Type::Float | Type::Character => {
                // Built-in types have known interface implementations
                match interface_name {
                    "Clone" | "Debug" | "PartialEq" | "Eq" => Ok(true),
                    "Display" => Ok(true),
                    _ => Ok(false)
                }
            }
            Type::Nil => {
                // Nil type implements limited interfaces
                match interface_name {
                    "Clone" | "Debug" | "PartialEq" | "Eq" => Ok(true),
                    _ => Ok(false)
                }
            }
            Type::Any => {
                // Any type can implement any interface
                Ok(true)
            }
            Type::AssociatedTypeProjection { .. } => {
                // Associated type projections need resolution first
                warn!("Cannot check interface implementation for unresolved associated type projection");
                Ok(false)
            }
            Type::Parameter(_) => {
                // Type parameters depend on their bounds
                warn!("Cannot check interface implementation for unresolved type parameter");
                Ok(false)
            }
            Type::Constructor { .. } | Type::Application { .. } => {
                // Higher-kinded types need special handling
                warn!("Cannot check interface implementation for higher-kinded type");
                Ok(false)
            }
            Type::Tuple(types) => {
                // Tuples implement interface if all element types implement it
                match interface_name {
                    "Clone" | "Debug" | "PartialEq" | "Eq" => {
                        for t in types {
                            if !self.type_implements_interface(t, interface_name)? {
                                return Ok(false);
                            }
                        }
                        Ok(true)
                    }
                    _ => Ok(false)
                }
            }
        }
    }

    /// Get all projections that depend on a given type
    #[instrument(skip(self))]
    pub fn get_dependent_projections(&self, base_type: &Type) -> Result<(), Error> {
        let cache = self.projection_cache.read()
            .map_err(|_| CursedError::system_error("Failed to acquire read lock"))?;
        
        let projections: Vec<AssociatedTypeProjection> = cache.keys()
            .filter(|proj| &proj.base_type == base_type)
            .cloned()
            .collect();

        Ok(projections)
    }

    /// Clear the projection cache
    #[instrument(skip(self))]
    pub fn clear_cache(&self) -> Result<(), Error> {
        let mut cache = self.projection_cache.write()
            .map_err(|_| CursedError::system_error("Failed to acquire write lock"))?;
        cache.clear();
        debug!("Cleared associated type projection cache");
        Ok(())
    }

    /// Get statistics about the registry
    #[instrument(skip(self))]
    pub fn get_statistics(&self) -> Result<(), Error> {
        let interface_registry = self.interface_associated_types.read()
            .map_err(|_| CursedError::system_error("Failed to acquire read lock"))?;
        let projection_cache = self.projection_cache.read()
            .map_err(|_| CursedError::system_error("Failed to acquire read lock"))?;

        let total_associated_types: usize = interface_registry.values()
            .map(|types| types.len())
            .sum();

        Ok(AssociatedTypeStatistics {
            total_interfaces: interface_registry.len(),
            total_associated_types,
            cached_projections: projection_cache.len(),
        })
    }
}

/// Statistics about the associated type registry
#[derive(Debug, Clone)]
pub struct AssociatedTypeStatistics {
    pub total_interfaces: usize,
    pub total_associated_types: usize,
    pub cached_projections: usize,
}

/// Trait for working with associated types in the type system
pub trait AssociatedTypeHandler {
    /// Create an associated type projection
    fn create_projection(&self, base_type: Type, interface_name: String, associated_type_name: String) -> AssociatedTypeProjection;
    
    /// Resolve multiple projections at once
    fn resolve_projections(&self, projections: &[AssociatedTypeProjection]) -> Result<(), Error>;
    
    /// Check if a type has associated types
    fn has_associated_types(&self, interface_name: &str) -> bool;
}

impl AssociatedTypeHandler for AssociatedTypeRegistry {
    #[instrument(skip(self))]
    fn create_projection(&self, base_type: Type, interface_name: String, associated_type_name: String) -> AssociatedTypeProjection {
        AssociatedTypeProjection {
            base_type,
            interface_name,
            associated_type_name,
        }
    }

    #[instrument(skip(self))]
    fn resolve_projections(&self, projections: &[AssociatedTypeProjection]) -> Result<(), Error> {
        let mut results = Vec::with_capacity(projections.len());
        for projection in projections {
            results.push(self.resolve_projection(projection)?);
        }
        Ok(results)
    }

    #[instrument(skip(self))]
    fn has_associated_types(&self, interface_name: &str) -> bool {
        if let Ok(registry) = self.interface_associated_types.read() {
            registry.contains_key(interface_name)
        } else {
            false
        }
    }
}

/// Helper functions for working with associated types
pub mod utils {
    use super::*;

    /// Create a standard Iterator interface with Item associated type
    pub fn create_iterator_interface() -> (String, Vec<AssociatedType>) {
        let item_type = AssociatedType {
            name: "Item".to_string(),
            constraints: vec![],
            default_type: None,
            documentation: Some("The type of items yielded by the iterator".to_string()),
        };

        ("Iterator".to_string(), vec![item_type])
    }

    /// Create a standard Collection interface with Item and Index associated types
    pub fn create_collection_interface() -> (String, Vec<AssociatedType>) {
        let item_type = AssociatedType {
            name: "Item".to_string(),
            constraints: vec![],
            default_type: None,
            documentation: Some("The type of items stored in the collection".to_string()),
        };

        let index_type = AssociatedType {
            name: "Index".to_string(),
            constraints: vec![],
            default_type: Some(Type::Integer),
            documentation: Some("The type used for indexing into the collection".to_string()),
        };

        ("Collection".to_string(), vec![item_type, index_type])
    }

    /// Extract associated type name from a projection string (e.g., "Iterator::Item" -> "Item")
    pub fn extract_associated_type_name(projection_str: &str) -> Option<(&str, &str)> {
        projection_str.split_once("::")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::type_system::constraint_resolver::ConstraintResolver;

    fn create_test_registry() -> AssociatedTypeRegistry {
        let constraint_resolver = Arc::new(ConstraintResolver::new());
        AssociatedTypeRegistry::new(constraint_resolver)
    }

    #[test]
    fn test_associated_type_creation() {
        let assoc_type = AssociatedType {
            name: "Item".to_string(),
            constraints: vec![],
            default_type: Some(Type::Integer),
            documentation: Some("Test associated type".to_string()),
        };

        assert_eq!(assoc_type.name, "Item");
        assert!(assoc_type.constraints.is_empty());
        assert_eq!(assoc_type.default_type, Some(Type::Integer));
    }

    #[test]
    fn test_registry_interface_registration() {
        let registry = create_test_registry();
        let (interface_name, associated_types) = utils::create_iterator_interface();

        let result = registry.register_interface_associated_types(&interface_name, associated_types);
        assert!(result.is_ok());

        let retrieved_types = registry.get_interface_associated_types(&interface_name).unwrap();
        assert_eq!(retrieved_types.len(), 1);
        assert_eq!(retrieved_types[0].name, "Item");
    }

    #[test]
    fn test_projection_creation() {
        let registry = create_test_registry();
        let projection = registry.create_projection(
            Type::Integer,
            "Iterator".to_string(),
            "Item".to_string(),
        );

        assert_eq!(projection.base_type, Type::Integer);
        assert_eq!(projection.interface_name, "Iterator");
        assert_eq!(projection.associated_type_name, "Item");
    }

    #[test]
    fn test_statistics() {
        let registry = create_test_registry();
        let (interface_name, associated_types) = utils::create_collection_interface();
        
        registry.register_interface_associated_types(&interface_name, associated_types).unwrap();
        
        let stats = registry.get_statistics().unwrap();
        assert_eq!(stats.total_interfaces, 1);
        assert_eq!(stats.total_associated_types, 2);
        assert_eq!(stats.cached_projections, 0);
    }

    #[test]
    fn test_projection_string_parsing() {
        let result = utils::extract_associated_type_name("Iterator::Item");
        assert_eq!(result, Some(("Iterator", "Item")));

        let result = utils::extract_associated_type_name("Collection::Index");
        assert_eq!(result, Some(("Collection", "Index")));

        let result = utils::extract_associated_type_name("InvalidFormat");
        assert_eq!(result, None);
    }
}
