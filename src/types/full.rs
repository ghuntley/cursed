//! Full type system implementation for CURSED
//! 
//! Provides complete type representation, advanced type operations,
//! and integration with CURSED semantics.

use crate::error::CursedError;
use crate::type_system::{
    TypeExpression, TypeDefinition, TypeSystem, TypeEnvironment,
    variance::{VarianceAnalyzer, Variance},
    generic_optimization::{GenericOptimizer, MonomorphizedInstance},
    higher_kinded_types::{HigherKindedTypeSystem, TypeConstructor, Kind},
    associated_types::{AssociatedTypeResolver, TraitImplementation, TypeProjection},
};
use std::collections::HashMap;

/// Complete type system integrating all advanced features
#[derive(Debug)]
pub struct FullTypeSystem {
    /// Core type system
    pub core: TypeSystem,
    /// Variance analysis
    pub variance_analyzer: VarianceAnalyzer,
    /// Generic optimization
    pub generic_optimizer: GenericOptimizer,
    /// Higher-kinded types
    pub hkt_system: HigherKindedTypeSystem,
    /// Associated types
    pub assoc_type_resolver: AssociatedTypeResolver,
    /// Advanced type cache
    type_cache: HashMap<String, CachedTypeInfo>,
    /// Type relationship graph
    relationship_graph: TypeRelationshipGraph,
}

/// Cached information about a type
#[derive(Debug, Clone)]
struct CachedTypeInfo {
    /// Type definition
    pub definition: TypeDefinition,
    /// Computed variance
    pub variance: Option<Vec<Variance>>,
    /// Available specializations
    pub specializations: Vec<String>,
    /// Trait implementations
    pub trait_implementations: Vec<String>,
}

/// Graph of type relationships for advanced analysis
#[derive(Debug, Default)]
struct TypeRelationshipGraph {
    /// Subtyping relationships
    subtyping: HashMap<String, Vec<String>>,
    /// Trait implementation relationships
    trait_impls: HashMap<String, Vec<String>>,
    /// Type constructor relationships
    constructors: HashMap<String, Vec<String>>,
}

/// Advanced type operations
impl FullTypeSystem {
    pub fn new() -> Self {
        Self {
            core: TypeSystem::new(),
            variance_analyzer: VarianceAnalyzer::new(),
            generic_optimizer: GenericOptimizer::new(),
            hkt_system: HigherKindedTypeSystem::new(),
            assoc_type_resolver: AssociatedTypeResolver::new(),
            type_cache: HashMap::new(),
            relationship_graph: TypeRelationshipGraph::default(),
        }
    }

    /// Perform comprehensive type checking with all advanced features
    pub fn comprehensive_type_check(&mut self, expr: &crate::ast::Expression) -> Result<TypeExpression, CursedError> {
        // Start with core type checking
        let base_type = self.core.check_expression(expr)
            .map_err(|e| CursedError::type_error(&e))?;

        // Apply advanced type analysis
        self.enhance_type_information(&base_type)
    }

    /// Enhance basic type information with advanced analysis
    fn enhance_type_information(&mut self, base_type: &TypeExpression) -> Result<TypeExpression, CursedError> {
        let mut enhanced_type = base_type.clone();

        // Apply variance analysis if this is a generic type
        if !base_type.parameters.is_empty() {
            enhanced_type = self.apply_variance_analysis(&enhanced_type)?;
        }

        // Apply generic optimization
        if self.is_generic_instantiation(&enhanced_type) {
            enhanced_type = self.apply_generic_optimization(&enhanced_type)?;
        }

        // Apply higher-kinded type analysis
        if self.has_higher_kinded_aspects(&enhanced_type) {
            enhanced_type = self.apply_hkt_analysis(&enhanced_type)?;
        }

        // Resolve associated types
        enhanced_type = self.resolve_associated_types(&enhanced_type)?;

        Ok(enhanced_type)
    }

    /// Apply variance analysis to a type
    fn apply_variance_analysis(&mut self, type_expr: &TypeExpression) -> Result<TypeExpression, CursedError> {
        if let Some(type_name) = &type_expr.name {
            // Get or compute variance for this type
            if let Some(type_def) = self.core.environment.type_definitions.get(type_name) {
                let variances = self.variance_analyzer.compute_variance(type_def)?;
                
                // Store variance information in cache
                if let Some(cached_info) = self.type_cache.get_mut(type_name) {
                    cached_info.variance = Some(variances);
                }
            }
        }
        
        Ok(type_expr.clone())
    }

    /// Apply generic optimization to a type
    fn apply_generic_optimization(&mut self, type_expr: &TypeExpression) -> Result<TypeExpression, CursedError> {
        if let Some(base_name) = &type_expr.name {
            if !type_expr.parameters.is_empty() {
                // Create a base type for optimization
                let base_type = TypeExpression {
                    kind: type_expr.kind.clone(),
                    name: Some(base_name.clone()),
                    parameters: Vec::new(),
                    return_type: None,
                };
                
                // Optimize the instantiation
                let optimized = self.generic_optimizer.optimize_instantiation(&base_type, &type_expr.parameters)?;
                
                // Return the optimized type expression
                return Ok(TypeExpression {
                    kind: type_expr.kind.clone(),
                    name: Some(format!("{}_{}", base_name, optimized.instance_id)),
                    parameters: optimized.type_arguments,
                    return_type: type_expr.return_type.clone(),
                });
            }
        }
        
        Ok(type_expr.clone())
    }

    /// Apply higher-kinded type analysis
    fn apply_hkt_analysis(&mut self, type_expr: &TypeExpression) -> Result<TypeExpression, CursedError> {
        // Infer the kind of this type
        let kind = self.hkt_system.infer_kind(type_expr)?;
        
        // If it's a higher-kinded type, validate its usage
        if let Kind::TypeConstructor(_, _) = kind {
            // Validate that it's being used correctly
            self.validate_hkt_usage(type_expr)?;
        }
        
        Ok(type_expr.clone())
    }

    /// Validate higher-kinded type usage
    fn validate_hkt_usage(&self, type_expr: &TypeExpression) -> Result<(), CursedError> {
        // Check if the type constructor exists and is being applied correctly
        if let Some(name) = &type_expr.name {
            if let Some(constructor) = self.hkt_system.get_constructor(name) {
                if type_expr.parameters.len() != constructor.parameters.len() {
                    return Err(CursedError::type_error(&format!(
                        "Type constructor {} expects {} parameters, got {}",
                        name, constructor.parameters.len(), type_expr.parameters.len()
                    )));
                }
            }
        }
        Ok(())
    }

    /// Resolve associated types in a type expression
    fn resolve_associated_types(&mut self, type_expr: &TypeExpression) -> Result<TypeExpression, CursedError> {
        // Look for type projections (T::AssocType patterns)
        if let Some(name) = &type_expr.name {
            if name.contains("::") {
                let parts: Vec<&str> = name.split("::").collect();
                if parts.len() == 2 {
                    // This looks like a type projection
                    let projection = self.create_type_projection(parts[0], parts[1])?;
                    let resolved = self.assoc_type_resolver.resolve_projection(&projection)?;
                    return Ok(resolved);
                }
            }
        }

        // Recursively resolve in parameters
        let mut resolved_params = Vec::new();
        for param in &type_expr.parameters {
            resolved_params.push(self.resolve_associated_types(param)?);
        }

        let mut result = type_expr.clone();
        result.parameters = resolved_params;
        Ok(result)
    }

    /// Create a type projection from string parts
    fn create_type_projection(&self, base_type: &str, assoc_type: &str) -> Result<TypeProjection, CursedError> {
        // This is a simplified implementation
        // In practice, we'd need more context to determine the correct trait
        Ok(TypeProjection {
            base_type: TypeExpression::named(base_type),
            trait_ref: crate::type_system::associated_types::TraitRef {
                name: "Iterator".to_string(), // Default assumption
                type_parameters: Vec::new(),
            },
            assoc_type_name: assoc_type.to_string(),
        })
    }

    /// Check if a type is a generic instantiation
    fn is_generic_instantiation(&self, type_expr: &TypeExpression) -> bool {
        !type_expr.parameters.is_empty()
    }

    /// Check if a type has higher-kinded aspects
    fn has_higher_kinded_aspects(&self, type_expr: &TypeExpression) -> bool {
        if let Some(name) = &type_expr.name {
            // Check if it's a known type constructor
            self.hkt_system.get_constructor(name).is_some()
        } else {
            false
        }
    }

    /// Register a new type with full analysis
    pub fn register_type(&mut self, type_def: TypeDefinition) -> Result<(), CursedError> {
        let type_name = type_def.name.clone();
        
        // Register with core system
        self.core.environment.add_type_definition(type_def.clone());
        
        // Compute and cache variance
        let variance = self.variance_analyzer.compute_variance(&type_def)?;
        
        // Cache type information
        let cached_info = CachedTypeInfo {
            definition: type_def.clone(),
            variance: Some(variance),
            specializations: Vec::new(),
            trait_implementations: Vec::new(),
        };
        self.type_cache.insert(type_name.clone(), cached_info);
        
        // Update relationship graph
        self.update_relationship_graph(&type_def);
        
        Ok(())
    }

    /// Update the type relationship graph
    fn update_relationship_graph(&mut self, type_def: &TypeDefinition) {
        let type_name = &type_def.name;
        
        // Add constructor relationships
        if !type_def.type_parameters.is_empty() {
            self.relationship_graph.constructors
                .entry(type_name.clone())
                .or_insert_with(Vec::new)
                .extend(type_def.type_parameters.iter().cloned());
        }
    }

    /// Register a trait implementation with full integration
    pub fn register_trait_implementation(&mut self, impl_def: TraitImplementation) -> Result<(), CursedError> {
        let implementing_type = impl_def.implementing_type.name.clone()
            .unwrap_or_else(|| "unknown".to_string());
        
        // Register with associated type resolver
        self.assoc_type_resolver.register_implementation(impl_def.clone())?;
        
        // Update cache
        if let Some(cached_info) = self.type_cache.get_mut(&implementing_type) {
            cached_info.trait_implementations.push(impl_def.trait_ref.name.clone());
        }
        
        // Update relationship graph
        self.relationship_graph.trait_impls
            .entry(implementing_type)
            .or_insert_with(Vec::new)
            .push(impl_def.trait_ref.name);
        
        Ok(())
    }

    /// Perform subtyping check with variance
    pub fn is_subtype(&mut self, sub_type: &TypeExpression, super_type: &TypeExpression) -> Result<bool, CursedError> {
        // Basic equality check first
        if sub_type == super_type {
            return Ok(true);
        }

        // Check variance-based subtyping
        if let (Some(sub_name), Some(super_name)) = (&sub_type.name, &super_type.name) {
            if sub_name == super_name {
                // Same type constructor, check parameters with variance
                if let Some(variances) = self.variance_analyzer.get_variance(sub_name) {
                    return self.variance_analyzer.check_subtyping(super_type, sub_type, variances);
                }
            }
        }

        // Check trait-based subtyping
        self.check_trait_subtyping(sub_type, super_type)
    }

    /// Check subtyping based on trait implementations
    fn check_trait_subtyping(&self, _sub_type: &TypeExpression, _super_type: &TypeExpression) -> Result<bool, CursedError> {
        // Simplified implementation
        // In practice, this would check if sub_type implements traits that super_type requires
        Ok(false)
    }

    /// Get optimization report for the type system
    pub fn get_optimization_report(&self) -> TypeSystemReport {
        let generic_report = self.generic_optimizer.generate_report();
        
        TypeSystemReport {
            total_types: self.type_cache.len(),
            cached_variances: self.type_cache.values()
                .filter(|info| info.variance.is_some())
                .count(),
            generic_instantiations: generic_report.total_instantiations,
            cache_hit_rate: generic_report.cache_hit_rate,
            type_constructors: self.hkt_system.get_instances("Array")
                .map(|instances| instances.len())
                .unwrap_or(0),
            trait_implementations: self.relationship_graph.trait_impls.values()
                .map(|v| v.len())
                .sum(),
        }
    }

    /// Clear all caches for memory management
    pub fn clear_caches(&mut self) {
        self.variance_analyzer.clear_cache();
        self.assoc_type_resolver.clear_cache();
        self.type_cache.clear();
    }
}

/// Report on type system performance and statistics
#[derive(Debug)]
pub struct TypeSystemReport {
    pub total_types: usize,
    pub cached_variances: usize,
    pub generic_instantiations: usize,
    pub cache_hit_rate: f64,
    pub type_constructors: usize,
    pub trait_implementations: usize,
}

/// Utility functions for the full type system
pub mod full_type_utils {
    use super::*;
    use crate::type_system::{TypeKind, MethodSignature};

    /// Create a comprehensive type system with common types pre-registered
    pub fn create_with_stdlib() -> Result<FullTypeSystem, CursedError> {
        let mut system = FullTypeSystem::new();
        
        // Register common collection types
        let array_type = TypeDefinition {
            name: "Array".to_string(),
            kind: TypeKind::Struct,
            type_parameters: vec!["T".to_string()],
            constraints: Vec::new(),
            methods: vec![
                MethodSignature {
                    name: "push".to_string(),
                    parameters: vec![TypeExpression::named("T")],
                    return_type: Some(TypeExpression::named("void")),
                    type_parameters: Vec::new(),
                    constraints: Vec::new(),
                },
                MethodSignature {
                    name: "get".to_string(),
                    parameters: vec![TypeExpression::named("int")],
                    return_type: Some(TypeExpression::generic("Option", vec![TypeExpression::named("T")])),
                    type_parameters: Vec::new(),
                    constraints: Vec::new(),
                }
            ],
            is_builtin: true,
        };
        system.register_type(array_type)?;

        // Register Option type
        let option_type = TypeDefinition {
            name: "Option".to_string(),
            kind: TypeKind::Enum,
            type_parameters: vec!["T".to_string()],
            constraints: Vec::new(),
            methods: vec![
                MethodSignature {
                    name: "is_some".to_string(),
                    parameters: Vec::new(),
                    return_type: Some(TypeExpression::named("bool")),
                    type_parameters: Vec::new(),
                    constraints: Vec::new(),
                },
                MethodSignature {
                    name: "unwrap".to_string(),
                    parameters: Vec::new(),
                    return_type: Some(TypeExpression::named("T")),
                    type_parameters: Vec::new(),
                    constraints: Vec::new(),
                }
            ],
            is_builtin: true,
        };
        system.register_type(option_type)?;

        // Register Result type
        let result_type = TypeDefinition {
            name: "Result".to_string(),
            kind: TypeKind::Enum,
            type_parameters: vec!["T".to_string(), "E".to_string()],
            constraints: Vec::new(),
            methods: vec![
                MethodSignature {
                    name: "is_ok".to_string(),
                    parameters: Vec::new(),
                    return_type: Some(TypeExpression::named("bool")),
                    type_parameters: Vec::new(),
                    constraints: Vec::new(),
                },
                MethodSignature {
                    name: "unwrap".to_string(),
                    parameters: Vec::new(),
                    return_type: Some(TypeExpression::named("T")),
                    type_parameters: Vec::new(),
                    constraints: Vec::new(),
                }
            ],
            is_builtin: true,
        };
        system.register_type(result_type)?;

        Ok(system)
    }

    /// Create a type definition with common patterns
    pub fn create_generic_type(name: &str, type_params: Vec<&str>) -> TypeDefinition {
        TypeDefinition {
            name: name.to_string(),
            kind: TypeKind::Struct,
            type_parameters: type_params.into_iter().map(|s| s.to_string()).collect(),
            constraints: Vec::new(),
            methods: Vec::new(),
            is_builtin: false,
        }
    }

    /// Format a type system report
    pub fn format_report(report: &TypeSystemReport) -> String {
        format!(
            "Type System Report:\n\
             - Total types: {}\n\
             - Cached variances: {}\n\
             - Generic instantiations: {}\n\
             - Cache hit rate: {:.2}%\n\
             - Type constructors: {}\n\
             - Trait implementations: {}",
            report.total_types,
            report.cached_variances,
            report.generic_instantiations,
            report.cache_hit_rate * 100.0,
            report.type_constructors,
            report.trait_implementations
        )
    }
}

impl Default for FullTypeSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Expression;

    #[test]
    fn test_full_type_system_creation() {
        let system = FullTypeSystem::new();
        assert!(system.type_cache.is_empty());
    }

    #[test]
    fn test_comprehensive_type_checking() {
        let mut system = FullTypeSystem::new();
        
        let expr = Expression::Integer(42);
        let result = system.comprehensive_type_check(&expr);
        assert!(result.is_ok());
        
        let type_expr = result.unwrap();
        assert_eq!(type_expr.name, Some("normie".to_string()));
    }

    #[test]
    fn test_type_registration() {
        let mut system = FullTypeSystem::new();
        
        let type_def = full_type_utils::create_generic_type("List", vec!["T"]);
        let result = system.register_type(type_def);
        assert!(result.is_ok());
        
        assert!(system.type_cache.contains_key("List"));
    }

    #[test]
    fn test_stdlib_creation() {
        let result = full_type_utils::create_with_stdlib();
        assert!(result.is_ok());
        
        let system = result.unwrap();
        assert!(system.type_cache.contains_key("Array"));
        assert!(system.type_cache.contains_key("Option"));
        assert!(system.type_cache.contains_key("Result"));
    }

    #[test]
    fn test_subtyping_check() {
        let mut system = FullTypeSystem::new();
        
        let int_type = TypeExpression::named("int");
        let float_type = TypeExpression::named("float");
        
        let is_subtype = system.is_subtype(&int_type, &int_type).unwrap();
        assert!(is_subtype); // Same type is subtype of itself
        
        let is_not_subtype = system.is_subtype(&int_type, &float_type).unwrap();
        assert!(!is_not_subtype); // Different types without relation
    }

    #[test]
    fn test_optimization_report() {
        let system = FullTypeSystem::new();
        let report = system.get_optimization_report();
        
        assert_eq!(report.total_types, 0); // No types registered yet
        assert_eq!(report.cached_variances, 0);
    }

    #[test]
    fn test_cache_clearing() {
        let mut system = FullTypeSystem::new();
        
        // Add some data to cache
        let type_def = full_type_utils::create_generic_type("Test", vec!["T"]);
        system.register_type(type_def).unwrap();
        
        assert!(!system.type_cache.is_empty());
        
        system.clear_caches();
        assert!(system.type_cache.is_empty());
    }
}
