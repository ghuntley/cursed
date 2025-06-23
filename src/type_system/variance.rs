//! Variance Analysis Implementation for CURSED Language
//!
//! This module provides variance analysis for generic types, enabling safe
//! subtyping relationships and covariance/contravariance checking.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use tracing::{debug, error, info, warn, instrument};

use crate::ast::crate::types::Type;
use crate::ast::traits::TypeParameter;
use crate::error::CursedError;

/// Represents the variance of a type parameter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Variance {
    /// Covariant: T' <: T implies F<T'> <: F<T>
    Covariant,
    /// Contravariant: T' <: T implies F<T> <: F<T'>
    Contravariant,
    /// Invariant: no subtyping relationship
    Invariant,
    /// Bivariant: both covariant and contravariant (rarely used)
    Bivariant,
}

impl Variance {
    /// Combine two variances (used for nested generic types)
    pub fn combine(self, other: Variance) -> Variance {
        match (self, other) {
            // Covariant rules
            (Variance::Covariant, Variance::Covariant) => Variance::Covariant,
            (Variance::Covariant, Variance::Contravariant) => Variance::Contravariant,
            (Variance::Covariant, Variance::Invariant) => Variance::Invariant,
            (Variance::Covariant, Variance::Bivariant) => Variance::Covariant,

            // Contravariant rules
            (Variance::Contravariant, Variance::Covariant) => Variance::Contravariant,
            (Variance::Contravariant, Variance::Contravariant) => Variance::Covariant,
            (Variance::Contravariant, Variance::Invariant) => Variance::Invariant,
            (Variance::Contravariant, Variance::Bivariant) => Variance::Contravariant,

            // Invariant rules
            (Variance::Invariant, _) => Variance::Invariant,

            // Bivariant rules
            (Variance::Bivariant, Variance::Covariant) => Variance::Covariant,
            (Variance::Bivariant, Variance::Contravariant) => Variance::Contravariant,
            (Variance::Bivariant, Variance::Invariant) => Variance::Invariant,
            (Variance::Bivariant, Variance::Bivariant) => Variance::Bivariant,
        }
    }

    /// Invert variance (used for function parameters)
    pub fn invert(self) -> Variance {
        match self {
            Variance::Covariant => Variance::Contravariant,
            Variance::Contravariant => Variance::Covariant,
            Variance::Invariant => Variance::Invariant,
            Variance::Bivariant => Variance::Bivariant,
        }
    }
}

/// Variance information for a type parameter
#[derive(Debug, Clone)]
pub struct TypeParameterVariance {
    /// Name of the type parameter
    pub parameter_name: String,
    /// Computed variance
    pub variance: Variance,
    /// Source locations where this variance was determined
    pub sources: Vec<VarianceSource>,
}

/// Source of variance information
#[derive(Debug, Clone)]
pub enum VarianceSource {
    /// Field access (usually covariant)
    FieldAccess { field_name: String },
    /// Function parameter (contravariant)
    FunctionParameter { function_name: String, param_index: usize },
    /// Function return type (covariant)
    FunctionReturn { function_name: String },
    /// Array element (covariant)
    ArrayElement,
    /// Interface method (depends on position)
    InterfaceMethod { method_name: String, is_return: bool },
    /// Explicit annotation
    Annotation,
}

/// Registry for managing variance analysis
#[derive(Debug)]
pub struct VarianceRegistry {
    /// Variance information for types and their parameters
    type_variances: RwLock<HashMap<String, Vec<TypeParameterVariance>>>,
    /// Subtyping relationships cache
    subtyping_cache: RwLock<HashMap<(Type, Type), bool>>,
    /// Safe variance relationships
    safe_variance_relationships: RwLock<HashSet<(String, String, Variance)>>,
}

impl VarianceRegistry {
    /// Create a new variance registry
    #[instrument]
    pub fn new() -> Self {
        debug!("Creating new VarianceRegistry");
        let registry = Self {
            type_variances: RwLock::new(HashMap::new()),
            subtyping_cache: RwLock::new(HashMap::new()),
            safe_variance_relationships: RwLock::new(HashSet::new()),
        };

        // Register built-in type variances
        if let Err(e) = registry.register_builtin_variances() {
            error!("Failed to register builtin variances: {}", e);
        }

        registry
    }

    /// Analyze the variance of type parameters in a type definition
    #[instrument(skip(self))]
    pub fn analyze_type_variance(&self, type_name: &str, definition: &Type) -> Result<(), Error> {
        debug!("Analyzing variance for type: {}", type_name);

        let mut variance_analyzer = VarianceAnalyzer::new();
        let variances = variance_analyzer.analyze_type(definition)?;

        // Store the results
        {
            let mut type_variances = self.type_variances.write()
                .map_err(|_| CursedError::system_error("Failed to acquire write lock"))?;
            type_variances.insert(type_name.to_string(), variances.clone());
        }

        info!("Completed variance analysis for type {}: {} parameters", 
              type_name, variances.len());
        Ok(variances)
    }

    /// Get variance information for a type
    #[instrument(skip(self))]
    pub fn get_type_variance(&self, type_name: &str) -> Result<(), Error> {
        let type_variances = self.type_variances.read()
            .map_err(|_| CursedError::system_error("Failed to acquire read lock"))?;
        
        Ok(type_variances.get(type_name).cloned().unwrap_or_default())
    }

    /// Check if one type is a subtype of another (considering variance)
    #[instrument(skip(self))]
    pub fn is_subtype(&self, subtype: &Type, supertype: &Type) -> Result<(), Error> {
        // Check cache first
        let cache_key = (subtype.clone(), supertype.clone());
        {
            let cache = self.subtyping_cache.read()
                .map_err(|_| CursedError::system_error("Failed to acquire read lock"))?;
            if let Some(&result) = cache.get(&cache_key) {
                debug!("Found cached subtyping result: {:?} <: {:?} = {}", 
                       subtype, supertype, result);
                return Ok(result);
            }
        }

        // Compute subtyping relationship
        let result = self.compute_subtyping_relationship(subtype, supertype)?;

        // Cache the result
        {
            let mut cache = self.subtyping_cache.write()
                .map_err(|_| CursedError::system_error("Failed to acquire write lock"))?;
            cache.insert(cache_key, result);
        }

        debug!("Computed subtyping: {:?} <: {:?} = {}", subtype, supertype, result);
        Ok(result)
    }

    /// Internal implementation for computing subtyping relationships
    #[instrument(skip(self))]
    fn compute_subtyping_relationship(&self, subtype: &Type, supertype: &Type) -> Result<(), Error> {
        // Reflexivity: T <: T
        if subtype == supertype {
            return Ok(true);
        }

        match (subtype, supertype) {
            // Basic type relationships
            (Type::Integer, Type::Float) => Ok(true), // Int can be used as Float
            (Type::Nil, _) => Ok(true), // Nil is a subtype of all reference types
            
            // Generic type relationships
            (Type::Generic(sub_name), Type::Generic(super_name)) => {
                // For simplified Generic variant, check name equality
                Ok(sub_name == super_name)
            }

            // Function type relationships
            (Type::Function(sub_params, sub_return),
             Type::Function(super_params, super_return)) => {
                self.check_function_subtyping(sub_params, sub_return, super_params, super_return)
            }

            // Array type relationships
            (Type::Array(sub_elem), Type::Array(super_elem)) => {
                // Arrays are covariant in their element type
                self.is_subtype(sub_elem, super_elem)
            }

            // Tuple type relationships
            (Type::Tuple(sub_types), Type::Tuple(super_types)) => {
                if sub_types.len() == super_types.len() {
                    // Tuples are covariant in all their element types
                    for (sub_elem, super_elem) in sub_types.iter().zip(super_types.iter()) {
                        if !self.is_subtype(sub_elem, super_elem)? {
                            return Ok(false);
                        }
                    }
                    Ok(true)
                } else {
                    Ok(false)
                }
            }

            // Interface type relationships
            (Type::Interface(sub_interface), Type::Interface(super_interface)) => {
                // For now, interface subtyping is nominal (by name)
                Ok(sub_interface.name == super_interface.name)
            }

            // Channel type relationships
            (Type::Channel(sub_elem), Type::Channel(super_elem)) => {
                // Channels are invariant in their element type for safety
                Ok(sub_elem == super_elem)
            }

            // Default: no subtyping relationship
            _ => Ok(false),
        }
    }

    /// Check subtyping for generic types considering variance
    #[instrument(skip(self))]
    fn check_generic_subtyping(&self, type_name: &str, sub_args: &[Type], super_args: &[Type]) -> Result<(), Error> {
        let variances = self.get_type_variance(type_name)?;
        
        if variances.len() != sub_args.len() {
            // If we don't have variance information, assume invariant
            for (sub_arg, super_arg) in sub_args.iter().zip(super_args.iter()) {
                if sub_arg != super_arg {
                    return Ok(false);
                }
            }
            return Ok(true);
        }

        for ((sub_arg, super_arg), variance_info) in sub_args.iter().zip(super_args.iter()).zip(variances.iter()) {
            let valid = match variance_info.variance {
                Variance::Covariant => self.is_subtype(sub_arg, super_arg)?,
                Variance::Contravariant => self.is_subtype(super_arg, sub_arg)?,
                Variance::Invariant => sub_arg == super_arg,
                Variance::Bivariant => true, // Both directions are acceptable
            };

            if !valid {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Check subtyping for function types
    #[instrument(skip(self))]
    fn check_function_subtyping(
        &self,
        sub_params: &[Type],
        sub_return: &Type,
        super_params: &[Type],
        super_return: &Type,
    ) -> Result<(), Error> {
        // Functions are contravariant in parameters and covariant in return type
        if sub_params.len() != super_params.len() {
            return Ok(false);
        }

        // Check parameter types (contravariant)
        for (sub_param, super_param) in sub_params.iter().zip(super_params.iter()) {
            if !self.is_subtype(super_param, sub_param)? {
                return Ok(false);
            }
        }

        // Check return type (covariant)
        self.is_subtype(sub_return, super_return)
    }

    /// Register safe variance relationships
    #[instrument(skip(self))]
    pub fn register_safe_variance(&self, type_name: &str, param_name: &str, variance: Variance) -> Result<(), Error> {
        let mut relationships = self.safe_variance_relationships.write()
            .map_err(|_| CursedError::system_error("Failed to acquire write lock"))?;
        
        relationships.insert((type_name.to_string(), param_name.to_string(), variance));
        debug!("Registered safe variance: {}::{} = {:?}", type_name, param_name, variance);
        Ok(())
    }

    /// Check if a variance relationship is safe
    #[instrument(skip(self))]
    pub fn is_variance_safe(&self, type_name: &str, param_name: &str, variance: Variance) -> Result<(), Error> {
        let relationships = self.safe_variance_relationships.read()
            .map_err(|_| CursedError::system_error("Failed to acquire read lock"))?;
        
        Ok(relationships.contains(&(type_name.to_string(), param_name.to_string(), variance)))
    }

    /// Register built-in type variances
    #[instrument(skip(self))]
    fn register_builtin_variances(&self) -> Result<(), Error> {
        // Array<T> is covariant in T
        self.register_safe_variance("Array", "T", Variance::Covariant)?;

        // Option<T> is covariant in T
        self.register_safe_variance("Option", "T", Variance::Covariant)?;

        // List<T> is covariant in T
        self.register_safe_variance("List", "T", Variance::Covariant)?;

        // Result<T, E> is covariant in both T and E
        self.register_safe_variance("Result", "T", Variance::Covariant)?;
        self.register_safe_variance("Result", "E", Variance::Covariant)?;

        // Map<K, V> is covariant in V, invariant in K (for simplicity)
        self.register_safe_variance("Map", "K", Variance::Invariant)?;
        self.register_safe_variance("Map", "V", Variance::Covariant)?;

        // Channel<T> is invariant in T (for safety)
        self.register_safe_variance("Channel", "T", Variance::Invariant)?;

        info!("Registered built-in variance relationships");
        Ok(())
    }

    /// Clear all caches
    #[instrument(skip(self))]
    pub fn clear_caches(&self) -> Result<(), Error> {
        {
            let mut cache = self.subtyping_cache.write()
                .map_err(|_| CursedError::system_error("Failed to acquire write lock"))?;
            cache.clear();
        }
        debug!("Cleared variance caches");
        Ok(())
    }

    /// Get statistics about the variance registry
    #[instrument(skip(self))]
    pub fn get_statistics(&self) -> Result<(), Error> {
        let type_variances = self.type_variances.read()
            .map_err(|_| CursedError::system_error("Failed to acquire read lock"))?;
        let subtyping_cache = self.subtyping_cache.read()
            .map_err(|_| CursedError::system_error("Failed to acquire read lock"))?;
        let safe_relationships = self.safe_variance_relationships.read()
            .map_err(|_| CursedError::system_error("Failed to acquire read lock"))?;

        let total_analyzed_params: usize = type_variances.values()
            .map(|params| params.len())
            .sum();

        Ok(VarianceStatistics {
            analyzed_types: type_variances.len(),
            total_analyzed_parameters: total_analyzed_params,
            cached_subtyping_checks: subtyping_cache.len(),
            safe_variance_relationships: safe_relationships.len(),
        })
    }
}

/// Statistics about variance analysis
#[derive(Debug, Clone)]
pub struct VarianceStatistics {
    pub analyzed_types: usize,
    pub total_analyzed_parameters: usize,
    pub cached_subtyping_checks: usize,
    pub safe_variance_relationships: usize,
}

/// Variance analyzer for computing type parameter variances
struct VarianceAnalyzer {
    parameter_variances: HashMap<String, Variance>,
    variance_sources: HashMap<String, Vec<VarianceSource>>,
}

impl VarianceAnalyzer {
    fn new() -> Self {
        Self {
            parameter_variances: HashMap::new(),
            variance_sources: HashMap::new(),
        }
    }

    #[instrument(skip(self))]
    fn analyze_type(&mut self, type_ref: &Type) -> Result<(), Error> {
        self.visit_type(type_ref, Variance::Covariant)?;
        
        let mut result = Vec::new();
        for (param_name, variance) in &self.parameter_variances {
            let sources = self.variance_sources.get(param_name).cloned().unwrap_or_default();
            result.push(TypeParameterVariance {
                parameter_name: param_name.clone(),
                variance: *variance,
                sources,
            });
        }

        Ok(result)
    }

    #[instrument(skip(self))]
    fn visit_type(&mut self, type_ref: &Type, context_variance: Variance) -> Result<(), Error> {
        match type_ref {
            Type::Generic(name) => {
                // This is a type parameter usage
                self.record_parameter_usage(name, context_variance, VarianceSource::Annotation);
            }

            Type::Function(params, return_type) => {
                // Function parameters are contravariant
                for param in params {
                    self.visit_type(param, context_variance.invert())?;
                }
                // Return type is covariant
                self.visit_type(return_type, context_variance)?;
            }

            Type::Array(element_type) => {
                // Arrays are covariant in their element type
                self.visit_type(element_type, context_variance)?;
            }

            Type::Tuple(element_types) => {
                // Tuples are covariant in all their element types
                for element_type in element_types {
                    self.visit_type(element_type, context_variance)?;
                }
            }

            Type::Channel(element_type) => {
                // Channels are invariant for safety
                self.visit_type(element_type, Variance::Invariant)?;
            }

            Type::Interface { .. } => {
                // Interface types don't directly contain type parameters we care about
                // TODO: Handle interface methods and their type parameters
            }

            Type::AssociatedTypeProjection { base_type, .. } => {
                // Visit the base type with current variance
                self.visit_type(base_type, context_variance)?;
            }

            // Concrete types don't affect variance
            Type::Integer | Type::Float | Type::String | Type::Boolean | 
            Type::Character | Type::Nil | Type::Any => {}

            // Struct types don't affect variance (usually)
            Type::Struct(_) => {}

            // Primitive types don't affect variance
            Type::Primitive(_) => {}

            // Map types - visit both key and value types
            Type::Map(key_type, value_type) => {
                self.visit_type(key_type, context_variance)?;
                self.visit_type(value_type, context_variance)?;
            }

            // Type parameters - record usage
            Type::Parameter(name) => {
                self.record_parameter_usage(name, context_variance, VarianceSource::Annotation);
            }

            // Type constructors and applications
            Type::Constructor { .. } => {}
            Type::Application { constructor, arguments } => {
                self.visit_type(constructor, context_variance)?;
                for arg in arguments {
                    self.visit_type(arg, context_variance)?;
                }
            }
        }

        Ok(())
    }

    fn record_parameter_usage(&mut self, param_name: &str, variance: Variance, source: VarianceSource) {
        let current_variance = self.parameter_variances.get(param_name).copied().unwrap_or(variance);
        let combined_variance = current_variance.combine(variance);
        
        self.parameter_variances.insert(param_name.to_string(), combined_variance);
        
        self.variance_sources.entry(param_name.to_string())
            .or_insert_with(Vec::new)
            .push(source);
    }
}

/// Trait for working with variance in the type system
pub trait VarianceHandler {
    /// Check if a variance assignment is safe for a given usage pattern
    fn is_variance_assignment_safe(&self, type_name: &str, param_name: &str, variance: Variance) -> Result<(), Error>;
    
    /// Get the most restrictive variance for a type parameter
    fn compute_safe_variance(&self, type_name: &str, param_name: &str) -> Result<(), Error>;
    
    /// Validate variance annotations against actual usage
    fn validate_variance_annotations(&self, type_name: &str, annotations: &[(String, Variance)]) -> Result<(), Error>;
}

impl VarianceHandler for VarianceRegistry {
    #[instrument(skip(self))]
    fn is_variance_assignment_safe(&self, type_name: &str, param_name: &str, variance: Variance) -> Result<(), Error> {
        let type_variances = self.get_type_variance(type_name)?;
        
        if let Some(param_variance) = type_variances.iter().find(|pv| pv.parameter_name == param_name) {
            // Check if the requested variance is compatible with the computed variance
            match (param_variance.variance, variance) {
                // Exact match is always safe
                (computed, requested) if computed == requested => Ok(true),
                
                // More restrictive variance is generally safe
                (Variance::Bivariant, _) => Ok(true),
                (Variance::Covariant, Variance::Invariant) => Ok(true),
                (Variance::Contravariant, Variance::Invariant) => Ok(true),
                
                // Less restrictive variance is unsafe
                _ => Ok(false),
            }
        } else {
            // No variance information available, assume invariant is safe
            Ok(variance == Variance::Invariant)
        }
    }

    #[instrument(skip(self))]
    fn compute_safe_variance(&self, type_name: &str, param_name: &str) -> Result<(), Error> {
        let type_variances = self.get_type_variance(type_name)?;
        
        if let Some(param_variance) = type_variances.iter().find(|pv| pv.parameter_name == param_name) {
            Ok(param_variance.variance)
        } else {
            // Default to invariant for safety
            Ok(Variance::Invariant)
        }
    }

    #[instrument(skip(self))]
    fn validate_variance_annotations(&self, type_name: &str, annotations: &[(String, Variance)]) -> Result<(), Error> {
        let mut errors = Vec::new();
        
        for (param_name, requested_variance) in annotations {
            if !self.is_variance_assignment_safe(type_name, param_name, *requested_variance)? {
                errors.push(format!(
                    "Unsafe variance annotation for parameter '{}': requested {:?} but computed variance is different",
                    param_name, requested_variance
                ));
            }
        }
        
        Ok(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variance_combination() {
        assert_eq!(Variance::Covariant.combine(Variance::Covariant), Variance::Covariant);
        assert_eq!(Variance::Covariant.combine(Variance::Contravariant), Variance::Contravariant);
        assert_eq!(Variance::Contravariant.combine(Variance::Contravariant), Variance::Covariant);
        assert_eq!(Variance::Invariant.combine(Variance::Covariant), Variance::Invariant);
    }

    #[test]
    fn test_variance_inversion() {
        assert_eq!(Variance::Covariant.invert(), Variance::Contravariant);
        assert_eq!(Variance::Contravariant.invert(), Variance::Covariant);
        assert_eq!(Variance::Invariant.invert(), Variance::Invariant);
        assert_eq!(Variance::Bivariant.invert(), Variance::Bivariant);
    }

    #[test]
    fn test_basic_subtyping() {
        let registry = VarianceRegistry::new();
        
        // Test reflexivity
        assert!(registry.is_subtype(&Type::Integer, &Type::Integer).unwrap());
        
        // Test basic subtyping
        assert!(registry.is_subtype(&Type::Integer, &Type::Float).unwrap());
        assert!(registry.is_subtype(&Type::Nil, &Type::String).unwrap());
    }

    #[test]
    fn test_array_covariance() {
        let registry = VarianceRegistry::new();
        
        let int_array = Type::Array(Box::new(Type::Integer));
        let float_array = Type::Array(Box::new(Type::Float));
        
        // Arrays should be covariant: Array<Int> <: Array<Float>
        assert!(registry.is_subtype(&int_array, &float_array).unwrap());
    }

    #[test]
    fn test_function_contravariance() {
        let registry = VarianceRegistry::new();
        
        let func1 = Type::Function(vec![Type::Float], Box::new(Type::Integer));
        let func2 = Type::Function(vec![Type::Integer], Box::new(Type::Float));
        
        // Function types: (Float) -> Int <: (Int) -> Float
        assert!(registry.is_subtype(&func1, &func2).unwrap());
    }

    #[test]
    fn test_variance_registry() {
        let registry = VarianceRegistry::new();
        
        // Test safe variance registration
        let result = registry.register_safe_variance("MyType", "T", Variance::Covariant);
        assert!(result.is_ok());
        
        // Test variance safety check
        let is_safe = registry.is_variance_safe("MyType", "T", Variance::Covariant).unwrap();
        assert!(is_safe);
        
        let is_not_safe = registry.is_variance_safe("MyType", "T", Variance::Contravariant).unwrap();
        assert!(!is_not_safe);
    }

    #[test]
    fn test_variance_analyzer() {
        let mut analyzer = VarianceAnalyzer::new();
        
        // Test analyzing a simple generic type
        let array_type = Type::Array(Box::new(Type::Generic("T".to_string())));
        
        let variances = analyzer.analyze_type(&array_type).unwrap();
        assert_eq!(variances.len(), 1);
        assert_eq!(variances[0].parameter_name, "T");
        assert_eq!(variances[0].variance, Variance::Covariant);
    }

    #[test]
    fn test_statistics() {
        let registry = VarianceRegistry::new();
        
        // Register some variance information
        registry.register_safe_variance("Type1", "T", Variance::Covariant).unwrap();
        registry.register_safe_variance("Type2", "U", Variance::Contravariant).unwrap();
        
        let stats = registry.get_statistics().unwrap();
        assert!(stats.safe_variance_relationships >= 2);
    }
}
