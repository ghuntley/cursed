//! Associated Types Implementation for CURSED Language
//! 
//! This module provides support for associated types in interfaces,
//! enabling more expressive and flexible generic programming patterns.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use tracing::{debug, error, info, warn, instrument};

use crate::ast::types::{Type, TypeParameter, GenericConstraint};
use crate::error::CursedError;
use crate::type_system::constraint_system::{ConstraintSystem, TypeConstraint};

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
    /// Constraint system for validation
    constraint_system: Arc<ConstraintSystem>,
}

impl AssociatedTypeRegistry {
    /// Create a new associated type registry
    #[instrument]
    pub fn new(constraint_system: Arc<ConstraintSystem>) -> Self {
        debug!("Creating new AssociatedTypeRegistry");
        Self {
            interface_associated_types: RwLock::new(HashMap::new()),
            projection_cache: RwLock::new(HashMap::new()),
            constraint_system,
        }
    }

    /// Register associated types for an interface
    #[instrument(skip(self))]
    pub fn register_interface_associated_types(
        &self,
        interface_name: &str,
        associated_types: Vec<AssociatedType>,
    ) -> Result<(), CursedError> {
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
    pub fn get_interface_associated_types(&self, interface_name: &str) -> Result<Vec<AssociatedType>, CursedError> {
        let registry = self.interface_associated_types.read()
            .map_err(|_| CursedError::system_error("Failed to acquire read lock"))?;
        
        Ok(registry.get(interface_name).cloned().unwrap_or_default())
    }

    /// Resolve an associated type projection
    #[instrument(skip(self))]
    pub fn resolve_projection(&self, projection: &AssociatedTypeProjection) -> Result<Type, CursedError> {
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
    fn resolve_projection_impl(&self, projection: &AssociatedTypeProjection) -> Result<Type, CursedError> {
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
    fn validate_associated_type(&self, assoc_type: &AssociatedType) -> Result<(), CursedError> {
        // Validate name
        if assoc_type.name.is_empty() {
            return Err(CursedError::type_error("Associated type name cannot be empty"));
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
    fn validate_constraint(&self, _constraint: &GenericConstraint) -> Result<(), CursedError> {
        // TODO: Implement constraint validation using constraint system
        Ok(())
    }

    /// Validate that a default type satisfies the associated type's constraints
    #[instrument(skip(self))]
    fn validate_default_type(&self, assoc_type: &AssociatedType, default_type: &Type) -> Result<(), CursedError> {
        // Check that the default type satisfies all constraints
        for constraint in &assoc_type.constraints {
            match constraint {
                GenericConstraint::InterfaceConstraint { interface_name, .. } => {
                    // Verify that the default type implements the required interface
                    if !self.type_implements_interface(default_type, interface_name)? {
                        return Err(CursedError::type_error(format!(
                            "Default type {:?} does not implement required interface {}",
                            default_type, interface_name
                        )));
                    }
                }
                GenericConstraint::TypeConstraint { .. } => {
                    // Validate type constraints
                    // TODO: Implement specific type constraint validation
                }
            }
        }
        Ok(())
    }

    /// Check if a type implements an interface (simplified implementation)
    #[instrument(skip(self))]
    fn type_implements_interface(&self, _type_ref: &Type, _interface_name: &str) -> Result<bool, CursedError> {
        // TODO: Implement proper interface implementation checking
        // For now, assume basic types implement common interfaces
        Ok(true)
    }

    /// Get all projections that depend on a given type
    #[instrument(skip(self))]
    pub fn get_dependent_projections(&self, base_type: &Type) -> Result<Vec<AssociatedTypeProjection>, CursedError> {
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
    pub fn clear_cache(&self) -> Result<(), CursedError> {
        let mut cache = self.projection_cache.write()
            .map_err(|_| CursedError::system_error("Failed to acquire write lock"))?;
        cache.clear();
        debug!("Cleared associated type projection cache");
        Ok(())
    }

    /// Get statistics about the registry
    #[instrument(skip(self))]
    pub fn get_statistics(&self) -> Result<AssociatedTypeStatistics, CursedError> {
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
    fn resolve_projections(&self, projections: &[AssociatedTypeProjection]) -> Result<Vec<Type>, CursedError>;
    
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
    fn resolve_projections(&self, projections: &[AssociatedTypeProjection]) -> Result<Vec<Type>, CursedError> {
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
    use crate::type_system::constraint_system::ConstraintSystem;

    fn create_test_registry() -> AssociatedTypeRegistry {
        let constraint_system = Arc::new(ConstraintSystem::new());
        AssociatedTypeRegistry::new(constraint_system)
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
