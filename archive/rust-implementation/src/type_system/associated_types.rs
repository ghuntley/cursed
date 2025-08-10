//! Associated type implementations for CURSED
//! 
//! Provides trait associated types, type projection and resolution,
//! and complex trait bounds handling.

use crate::error::CursedError;
use crate::type_system::{TypeExpression, TypeDefinition, GenericConstraint};
use std::collections::HashMap;

/// Associated type definition within a trait
#[derive(Debug, Clone)]
pub struct AssociatedType {
    /// Name of the associated type
    pub name: String,
    /// Trait that defines this associated type
    pub trait_ref: TraitRef,
    /// Bounds on the associated type
    pub bounds: Vec<TypeBound>,
    /// Default implementation if any
    pub default: Option<TypeExpression>,
}

/// Reference to a trait
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TraitRef {
    /// Name of the trait
    pub name: String,
    /// Type parameters for the trait
    pub type_parameters: Vec<TypeExpression>,
}

/// Bounds that can be placed on types
#[derive(Debug, Clone)]
pub enum TypeBound {
    /// Trait bound (T: Trait)
    Trait(TraitRef),
    /// Lifetime bound (T: 'a)
    Lifetime(String),
    /// Equality bound (T = SomeType)
    Equality(TypeExpression),
    /// Associated type bound (T::Item = U)
    AssociatedType(String, TypeExpression),
}

/// Type projection (T::AssocType)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeProjection {
    /// Base type that implements the trait
    pub base_type: TypeExpression,
    /// Trait containing the associated type
    pub trait_ref: TraitRef,
    /// Name of the associated type
    pub assoc_type_name: String,
}

/// Trait implementation with associated types
#[derive(Debug, Clone)]
pub struct TraitImplementation {
    /// The trait being implemented
    pub trait_ref: TraitRef,
    /// The type implementing the trait
    pub implementing_type: TypeExpression,
    /// Associated type implementations
    pub associated_types: HashMap<String, TypeExpression>,
    /// Method implementations
    pub methods: Vec<MethodImpl>,
    /// Where clauses
    pub where_clauses: Vec<WhereClause>,
}

/// Method implementation
#[derive(Debug, Clone)]
pub struct MethodImpl {
    /// Method name
    pub name: String,
    /// Method signature
    pub signature: MethodSignature,
    /// Implementation details (placeholder)
    pub implementation: String,
}

/// Method signature for trait methods
#[derive(Debug, Clone)]
pub struct MethodSignature {
    /// Method name
    pub name: String,
    /// Parameters including self
    pub parameters: Vec<Parameter>,
    /// Return type
    pub return_type: TypeExpression,
    /// Type parameters
    pub type_parameters: Vec<String>,
    /// Constraints on type parameters
    pub constraints: Vec<TypeBound>,
}

/// Parameter in method signature
#[derive(Debug, Clone)]
pub struct Parameter {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: TypeExpression,
    /// Whether this is a self parameter
    pub is_self: bool,
}

/// Where clause for additional constraints
#[derive(Debug, Clone)]
pub struct WhereClause {
    /// Type being constrained
    pub type_expr: TypeExpression,
    /// Bounds on the type
    pub bounds: Vec<TypeBound>,
}

/// Associated type resolution system
#[derive(Debug)]
pub struct AssociatedTypeResolver {
    /// Registry of trait definitions
    traits: HashMap<String, TraitDefinition>,
    /// Registry of trait implementations
    implementations: HashMap<String, Vec<TraitImplementation>>,
    /// Cache for resolved type projections
    projection_cache: HashMap<TypeProjection, TypeExpression>,
}

/// Complete trait definition
#[derive(Debug, Clone)]
pub struct TraitDefinition {
    /// Trait name
    pub name: String,
    /// Type parameters
    pub type_parameters: Vec<String>,
    /// Associated types defined in this trait
    pub associated_types: HashMap<String, AssociatedType>,
    /// Required methods
    pub methods: Vec<MethodSignature>,
    /// Super traits
    pub super_traits: Vec<TraitRef>,
    /// Default implementations
    pub default_methods: HashMap<String, String>,
}

impl AssociatedTypeResolver {
    pub fn new() -> Self {
        let mut resolver = Self {
            traits: HashMap::new(),
            implementations: HashMap::new(),
            projection_cache: HashMap::new(),
        };
        
        resolver.register_builtin_traits();
        resolver
    }

    /// Register built-in traits with associated types
    fn register_builtin_traits(&mut self) {
        // Iterator trait
        let iterator_trait = TraitDefinition {
            name: "Iterator".to_string(),
            type_parameters: Vec::new(),
            associated_types: {
                let mut types = HashMap::new();
                types.insert("Item".to_string(), AssociatedType {
                    name: "Item".to_string(),
                    trait_ref: TraitRef {
                        name: "Iterator".to_string(),
                        type_parameters: Vec::new(),
                    },
                    bounds: Vec::new(),
                    default: None,
                });
                types
            },
            methods: vec![
                MethodSignature {
                    name: "next".to_string(),
                    parameters: vec![
                        Parameter {
                            name: "self".to_string(),
                            param_type: TypeExpression::named("Self"),
                            is_self: true,
                        }
                    ],
                    return_type: TypeExpression::generic("Option", vec![
                        TypeExpression::named("Self::Item")
                    ]),
                    type_parameters: Vec::new(),
                    constraints: Vec::new(),
                }
            ],
            super_traits: Vec::new(),
            default_methods: HashMap::new(),
        };
        self.register_trait(iterator_trait).expect("Failed to register Iterator");

        // From trait
        let from_trait = TraitDefinition {
            name: "From".to_string(),
            type_parameters: vec!["T".to_string()],
            associated_types: HashMap::new(),
            methods: vec![
                MethodSignature {
                    name: "from".to_string(),
                    parameters: vec![
                        Parameter {
                            name: "value".to_string(),
                            param_type: TypeExpression::named("T"),
                            is_self: false,
                        }
                    ],
                    return_type: TypeExpression::named("Self"),
                    type_parameters: Vec::new(),
                    constraints: Vec::new(),
                }
            ],
            super_traits: Vec::new(),
            default_methods: HashMap::new(),
        };
        self.register_trait(from_trait).expect("Failed to register From");

        // Into trait with associated type
        let into_trait = TraitDefinition {
            name: "Into".to_string(),
            type_parameters: vec!["T".to_string()],
            associated_types: HashMap::new(),
            methods: vec![
                MethodSignature {
                    name: "into".to_string(),
                    parameters: vec![
                        Parameter {
                            name: "self".to_string(),
                            param_type: TypeExpression::named("Self"),
                            is_self: true,
                        }
                    ],
                    return_type: TypeExpression::named("T"),
                    type_parameters: Vec::new(),
                    constraints: Vec::new(),
                }
            ],
            super_traits: Vec::new(),
            default_methods: HashMap::new(),
        };
        self.register_trait(into_trait).expect("Failed to register Into");
    }

    /// Register a new trait definition
    pub fn register_trait(&mut self, trait_def: TraitDefinition) -> Result<(), CursedError> {
        // Validate trait definition
        self.validate_trait(&trait_def)?;
        
        self.traits.insert(trait_def.name.clone(), trait_def);
        Ok(())
    }

    /// Validate a trait definition
    fn validate_trait(&self, trait_def: &TraitDefinition) -> Result<(), CursedError> {
        // Check for circular dependencies in super traits
        self.check_circular_dependencies(&trait_def.name, &trait_def.super_traits)?;
        
        // Validate associated types
        for assoc_type in trait_def.associated_types.values() {
            self.validate_associated_type(assoc_type)?;
        }
        
        Ok(())
    }

    /// Check for circular dependencies in trait hierarchy
    fn check_circular_dependencies(&self, trait_name: &str, super_traits: &[TraitRef]) -> Result<(), CursedError> {
        let mut visited = std::collections::HashSet::new();
        self.check_circular_helper(trait_name, super_traits, &mut visited)
    }

    fn check_circular_helper(&self, 
                            current: &str, 
                            super_traits: &[TraitRef], 
                            visited: &mut std::collections::HashSet<String>) -> Result<(), CursedError> {
        if visited.contains(current) {
            return Err(CursedError::type_error(&format!("Circular trait dependency detected involving {}", current)));
        }
        
        visited.insert(current.to_string());
        
        for super_trait in super_traits {
            if let Some(super_def) = self.traits.get(&super_trait.name) {
                self.check_circular_helper(&super_trait.name, &super_def.super_traits, visited)?;
            }
        }
        
        visited.remove(current);
        Ok(())
    }

    /// Validate an associated type definition
    fn validate_associated_type(&self, assoc_type: &AssociatedType) -> Result<(), CursedError> {
        // Validate bounds
        for bound in &assoc_type.bounds {
            self.validate_type_bound(bound)?;
        }
        
        Ok(())
    }

    /// Validate a type bound
    fn validate_type_bound(&self, bound: &TypeBound) -> Result<(), CursedError> {
        match bound {
            TypeBound::Trait(trait_ref) => {
                if !self.traits.contains_key(&trait_ref.name) {
                    return Err(CursedError::type_error(&format!("Unknown trait: {}", trait_ref.name)));
                }
            }
            TypeBound::AssociatedType(name, _type_expr) => {
                // Would validate the associated type exists and type_expr is valid
            }
            _ => {} // Other bounds are assumed valid for now
        }
        Ok(())
    }

    /// Register a trait implementation
    pub fn register_implementation(&mut self, impl_def: TraitImplementation) -> Result<(), CursedError> {
        // Validate the implementation
        self.validate_implementation(&impl_def)?;
        
        // Store by implementing type name
        let type_name = impl_def.implementing_type.name.clone()
            .unwrap_or_else(|| "unknown".to_string());
        
        self.implementations.entry(type_name)
            .or_insert_with(Vec::new)
            .push(impl_def);
        
        Ok(())
    }

    /// Validate a trait implementation
    fn validate_implementation(&self, impl_def: &TraitImplementation) -> Result<(), CursedError> {
        // Check that the trait exists
        let trait_def = self.traits.get(&impl_def.trait_ref.name)
            .ok_or_else(|| CursedError::type_error(&format!("Unknown trait: {}", impl_def.trait_ref.name)))?;
        
        // Check that all required associated types are implemented
        for (assoc_name, _assoc_def) in &trait_def.associated_types {
            if !impl_def.associated_types.contains_key(assoc_name) {
                return Err(CursedError::type_error(&format!(
                    "Missing associated type {} in implementation of {} for {}",
                    assoc_name, impl_def.trait_ref.name, 
                    impl_def.implementing_type.name.as_deref().unwrap_or("unknown")
                )));
            }
        }
        
        // Check that all required methods are implemented
        for method in &trait_def.methods {
            if !impl_def.methods.iter().any(|m| m.name == method.name) &&
               !trait_def.default_methods.contains_key(&method.name) {
                return Err(CursedError::type_error(&format!(
                    "Missing method {} in implementation of {} for {}",
                    method.name, impl_def.trait_ref.name,
                    impl_def.implementing_type.name.as_deref().unwrap_or("unknown")
                )));
            }
        }
        
        Ok(())
    }

    /// Resolve a type projection (T::AssocType)
    pub fn resolve_projection(&mut self, projection: &TypeProjection) -> Result<TypeExpression, CursedError> {
        // Check cache first
        if let Some(cached) = self.projection_cache.get(projection) {
            return Ok(cached.clone());
        }

        let resolved = self.resolve_projection_impl(projection)?;
        
        // Cache the result
        self.projection_cache.insert(projection.clone(), resolved.clone());
        
        Ok(resolved)
    }

    /// Internal implementation of projection resolution
    fn resolve_projection_impl(&self, projection: &TypeProjection) -> Result<TypeExpression, CursedError> {
        // Find the trait implementation for the base type
        let base_type_name = projection.base_type.name.as_ref()
            .ok_or_else(|| CursedError::type_error("Cannot resolve projection for anonymous type"))?;
        
        let implementations = self.implementations.get(base_type_name)
            .ok_or_else(|| CursedError::type_error(&format!("No implementations found for type {}", base_type_name)))?;
        
        // Find the specific trait implementation
        for impl_def in implementations {
            if impl_def.trait_ref.name == projection.trait_ref.name {
                // Check if this implementation provides the associated type
                if let Some(assoc_type) = impl_def.associated_types.get(&projection.assoc_type_name) {
                    return Ok(assoc_type.clone());
                }
            }
        }
        
        Err(CursedError::type_error(&format!(
            "No implementation of {}::{} found for type {}",
            projection.trait_ref.name, projection.assoc_type_name, base_type_name
        )))
    }

    /// Check if a type implements a trait
    pub fn check_trait_implementation(&self, 
                                    implementing_type: &TypeExpression, 
                                    trait_ref: &TraitRef) -> Result<bool, CursedError> {
        let type_name = implementing_type.name.as_ref()
            .ok_or_else(|| CursedError::type_error("Cannot check trait implementation for anonymous type"))?;
        
        if let Some(implementations) = self.implementations.get(type_name) {
            for impl_def in implementations {
                if impl_def.trait_ref.name == trait_ref.name {
                    // Check type parameter compatibility
                    if self.type_parameters_compatible(&impl_def.trait_ref.type_parameters, &trait_ref.type_parameters)? {
                        return Ok(true);
                    }
                }
            }
        }
        
        Ok(false)
    }

    /// Check if type parameters are compatible
    fn type_parameters_compatible(&self, impl_params: &[TypeExpression], required_params: &[TypeExpression]) -> Result<bool, CursedError> {
        if impl_params.len() != required_params.len() {
            return Ok(false);
        }
        
        for (impl_param, required_param) in impl_params.iter().zip(required_params.iter()) {
            if impl_param != required_param {
                return Ok(false);
            }
        }
        
        Ok(true)
    }

    /// Get all implementations for a type
    pub fn get_implementations(&self, type_name: &str) -> Option<&Vec<TraitImplementation>> {
        self.implementations.get(type_name)
    }

    /// Get a trait definition
    pub fn get_trait(&self, trait_name: &str) -> Option<&TraitDefinition> {
        self.traits.get(trait_name)
    }

    /// Clear the projection cache
    pub fn clear_cache(&mut self) {
        self.projection_cache.clear();
    }
}

/// Utility functions for associated types
pub mod assoc_type_utils {
    use super::*;

    /// Create a simple trait with one associated type
    pub fn create_simple_trait(name: &str, assoc_type: &str) -> TraitDefinition {
        let mut associated_types = HashMap::new();
        associated_types.insert(assoc_type.to_string(), AssociatedType {
            name: assoc_type.to_string(),
            trait_ref: TraitRef {
                name: name.to_string(),
                type_parameters: Vec::new(),
            },
            bounds: Vec::new(),
            default: None,
        });

        TraitDefinition {
            name: name.to_string(),
            type_parameters: Vec::new(),
            associated_types,
            methods: Vec::new(),
            super_traits: Vec::new(),
            default_methods: HashMap::new(),
        }
    }

    /// Create a trait implementation
    pub fn create_implementation(trait_name: &str, 
                               impl_type: &str, 
                               assoc_types: Vec<(&str, TypeExpression)>) -> TraitImplementation {
        let mut associated_types = HashMap::new();
        for (name, type_expr) in assoc_types {
            associated_types.insert(name.to_string(), type_expr);
        }

        // Create method implementations for common traits
        let methods = match trait_name {
            "Iterator" => vec![
                MethodImpl {
                    name: "next".to_string(),
                    signature: MethodSignature {
                        name: "next".to_string(),
                        parameters: vec![Parameter {
                            name: "self".to_string(),
                            param_type: TypeExpression::named("Self"),
                            is_self: true,
                        }],
                        return_type: TypeExpression::generic("Option", vec![
                            TypeExpression::named("Self::Item")
                        ]),
                        type_parameters: Vec::new(),
                        constraints: Vec::new(),
                    },
                    implementation: "placeholder_implementation".to_string(),
                }
            ],
            _ => Vec::new(),
        };

        TraitImplementation {
            trait_ref: TraitRef {
                name: trait_name.to_string(),
                type_parameters: Vec::new(),
            },
            implementing_type: TypeExpression::named(impl_type),
            associated_types,
            methods,
            where_clauses: Vec::new(),
        }
    }

    /// Format a type projection for display
    pub fn format_projection(projection: &TypeProjection) -> String {
        format!("{}::{}", 
                projection.base_type.name.as_deref().unwrap_or("unknown"),
                projection.assoc_type_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_traits() {
        let resolver = AssociatedTypeResolver::new();
        
        assert!(resolver.get_trait("Iterator").is_some());
        assert!(resolver.get_trait("From").is_some());
        assert!(resolver.get_trait("Into").is_some());
    }

    #[test]
    fn test_trait_registration() {
        let mut resolver = AssociatedTypeResolver::new();
        
        let trait_def = assoc_type_utils::create_simple_trait("Collect", "Output");
        resolver.register_trait(trait_def).unwrap();
        
        assert!(resolver.get_trait("Collect").is_some());
    }

    #[test]
    fn test_implementation_registration() {
        let mut resolver = AssociatedTypeResolver::new();
        
        let impl_def = assoc_type_utils::create_implementation(
            "Iterator",
            "Array",
            vec![("Item", TypeExpression::named("int"))]
        );
        
        resolver.register_implementation(impl_def).unwrap();
        
        let implementations = resolver.get_implementations("Array");
        assert!(implementations.is_some());
        assert_eq!(implementations.unwrap().len(), 1);
    }

    #[test]
    fn test_projection_resolution() {
        let mut resolver = AssociatedTypeResolver::new();
        
        // Register an implementation
        let impl_def = assoc_type_utils::create_implementation(
            "Iterator",
            "Array",
            vec![("Item", TypeExpression::named("int"))]
        );
        resolver.register_implementation(impl_def).unwrap();
        
        // Test projection resolution
        let projection = TypeProjection {
            base_type: TypeExpression::named("Array"),
            trait_ref: TraitRef {
                name: "Iterator".to_string(),
                type_parameters: Vec::new(),
            },
            assoc_type_name: "Item".to_string(),
        };
        
        let resolved = resolver.resolve_projection(&projection).unwrap();
        assert_eq!(resolved.name, Some("int".to_string()));
    }

    #[test]
    fn test_trait_implementation_check() {
        let mut resolver = AssociatedTypeResolver::new();
        
        let impl_def = assoc_type_utils::create_implementation(
            "Iterator",
            "Array",
            vec![("Item", TypeExpression::named("int"))]
        );
        resolver.register_implementation(impl_def).unwrap();
        
        let array_type = TypeExpression::named("Array");
        let iterator_trait = TraitRef {
            name: "Iterator".to_string(),
            type_parameters: Vec::new(),
        };
        
        let implements = resolver.check_trait_implementation(&array_type, &iterator_trait).unwrap();
        assert!(implements);
    }
}
