//! Generic type optimization and specialization for CURSED
//! 
//! Provides monomorphization, template instantiation efficiency,
//! and generic type specialization to improve compilation performance.

use crate::error::CursedError;
use crate::type_system::{TypeExpression, TypeDefinition, TypeSubstitution, InstantiatedType};
use std::collections::{HashMap, HashSet};

/// Generic optimization engine
#[derive(Debug)]
pub struct GenericOptimizer {
    /// Cache of monomorphized instances
    monomorphization_cache: HashMap<String, MonomorphizedInstance>,
    /// Specialization registry
    specializations: HashMap<String, Vec<Specialization>>,
    /// Template instantiation metrics
    instantiation_metrics: InstantiationMetrics,
    /// Optimization settings
    config: OptimizationConfig,
}

/// A monomorphized instance of a generic type
#[derive(Debug, Clone)]
pub struct MonomorphizedInstance {
    /// Original generic type
    pub base_type: TypeExpression,
    /// Concrete type arguments
    pub type_arguments: Vec<TypeExpression>,
    /// Generated specialized code identifier
    pub instance_id: String,
    /// Optimization level applied
    pub optimization_level: OptimizationLevel,
    /// Size estimation for code bloat analysis
    pub estimated_size: usize,
}

/// Type specialization for common instantiations
#[derive(Debug, Clone)]
pub struct Specialization {
    /// Pattern to match type arguments
    pub pattern: SpecializationPattern,
    /// Optimized implementation
    pub optimized_impl: String,
    /// Performance benefit estimation
    pub benefit_score: f64,
}

/// Pattern for specialization matching
#[derive(Debug, Clone)]
pub enum SpecializationPattern {
    /// Exact type match
    ExactMatch(Vec<TypeExpression>),
    /// Primitive type constraint
    PrimitiveOnly,
    /// Size-based constraint (for arrays/collections)
    SizeConstrained(usize),
    /// Custom predicate
    Custom(String),
}

/// Optimization levels for generic instantiation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OptimizationLevel {
    /// No optimization, direct instantiation
    None,
    /// Basic dead code elimination
    Basic,
    /// Aggressive inlining and specialization
    Aggressive,
    /// Maximum optimization with size trade-offs
    Maximum,
}

/// Metrics for tracking instantiation efficiency
#[derive(Debug, Default)]
pub struct InstantiationMetrics {
    /// Total number of instantiations
    pub total_instantiations: usize,
    /// Cache hit rate
    pub cache_hits: usize,
    /// Average instantiation time (microseconds)
    pub avg_instantiation_time: f64,
    /// Code size impact
    pub total_code_size: usize,
    /// Memory usage for templates
    pub template_memory_usage: usize,
}

/// Configuration for generic optimization
#[derive(Debug)]
pub struct OptimizationConfig {
    /// Maximum number of cached instances
    pub max_cache_size: usize,
    /// Enable aggressive specialization
    pub enable_specialization: bool,
    /// Code size threshold for instantiation
    pub size_threshold: usize,
    /// Enable template deduplication
    pub enable_deduplication: bool,
}

impl GenericOptimizer {
    pub fn new() -> Self {
        Self::with_config(OptimizationConfig::default())
    }

    pub fn with_config(config: OptimizationConfig) -> Self {
        Self {
            monomorphization_cache: HashMap::new(),
            specializations: HashMap::new(),
            instantiation_metrics: InstantiationMetrics::default(),
            config,
        }
    }

    /// Optimize a generic type instantiation
    pub fn optimize_instantiation(&mut self, 
                                 base_type: &TypeExpression, 
                                 type_args: &[TypeExpression]) -> Result<MonomorphizedInstance, CursedError> {
        let cache_key = self.generate_cache_key(base_type, type_args);
        
        // Check cache first
        if let Some(cached) = self.monomorphization_cache.get(&cache_key) {
            self.instantiation_metrics.cache_hits += 1;
            return Ok(cached.clone());
        }

        let start_time = std::time::Instant::now();

        // Check for available specializations
        let instance = if let Some(specialization) = self.find_specialization(base_type, type_args)? {
            self.create_specialized_instance(base_type, type_args, specialization)?
        } else {
            self.create_generic_instance(base_type, type_args)?
        };

        // Update metrics
        self.instantiation_metrics.total_instantiations += 1;
        let elapsed = start_time.elapsed().as_micros() as f64;
        self.update_avg_time(elapsed);

        // Cache if under size threshold
        if instance.estimated_size <= self.config.size_threshold {
            self.cache_instance(cache_key, instance.clone());
        }

        Ok(instance)
    }

    /// Find the best specialization for given type arguments
    fn find_specialization(&self, 
                          base_type: &TypeExpression, 
                          type_args: &[TypeExpression]) -> Result<Option<&Specialization>, CursedError> {
        
        if let Some(type_name) = &base_type.name {
            if let Some(specializations) = self.specializations.get(type_name) {
                for specialization in specializations {
                    if self.matches_pattern(&specialization.pattern, type_args)? {
                        return Ok(Some(specialization));
                    }
                }
            }
        }
        
        Ok(None)
    }

    /// Check if type arguments match a specialization pattern
    fn matches_pattern(&self, pattern: &SpecializationPattern, type_args: &[TypeExpression]) -> Result<bool, CursedError> {
        match pattern {
            SpecializationPattern::ExactMatch(expected) => {
                Ok(type_args.len() == expected.len() && 
                   type_args.iter().zip(expected.iter()).all(|(a, e)| a == e))
            }
            SpecializationPattern::PrimitiveOnly => {
                Ok(type_args.iter().all(|arg| self.is_primitive_type(arg)))
            }
            SpecializationPattern::SizeConstrained(max_size) => {
                Ok(self.estimate_type_size(type_args) <= *max_size)
            }
            SpecializationPattern::Custom(_predicate) => {
                // For now, return false for custom predicates
                // In a full implementation, this would evaluate the predicate
                Ok(false)
            }
        }
    }

    /// Create a specialized instance
    fn create_specialized_instance(&self, 
                                  base_type: &TypeExpression,
                                  type_args: &[TypeExpression],
                                  specialization: &Specialization) -> Result<MonomorphizedInstance, CursedError> {
        Ok(MonomorphizedInstance {
            base_type: base_type.clone(),
            type_arguments: type_args.to_vec(),
            instance_id: format!("specialized_{}", self.generate_instance_id(base_type, type_args)),
            optimization_level: OptimizationLevel::Aggressive,
            estimated_size: self.estimate_specialized_size(base_type, type_args, specialization),
        })
    }

    /// Create a generic instance without specialization
    fn create_generic_instance(&self, 
                              base_type: &TypeExpression,
                              type_args: &[TypeExpression]) -> Result<MonomorphizedInstance, CursedError> {
        Ok(MonomorphizedInstance {
            base_type: base_type.clone(),
            type_arguments: type_args.to_vec(),
            instance_id: self.generate_instance_id(base_type, type_args),
            optimization_level: self.config.optimization_level(),
            estimated_size: self.estimate_generic_size(base_type, type_args),
        })
    }

    /// Add a specialization for a generic type
    pub fn add_specialization(&mut self, 
                             type_name: String, 
                             specialization: Specialization) -> Result<(), CursedError> {
        self.specializations
            .entry(type_name)
            .or_insert_with(Vec::new)
            .push(specialization);
        Ok(())
    }

    /// Perform template deduplication to reduce code bloat
    pub fn deduplicate_templates(&mut self) -> Result<usize, CursedError> {
        if !self.config.enable_deduplication {
            return Ok(0);
        }

        let mut deduplication_map: HashMap<String, Vec<String>> = HashMap::new();
        let mut removed_count = 0;

        // Group instances by their generated code signature
        for (key, instance) in &self.monomorphization_cache {
            let signature = self.compute_code_signature(instance);
            deduplication_map.entry(signature).or_insert_with(Vec::new).push(key.clone());
        }

        // Remove duplicates, keeping the first instance of each signature
        for (_, keys) in deduplication_map {
            if keys.len() > 1 {
                for key in keys.iter().skip(1) {
                    self.monomorphization_cache.remove(key);
                    removed_count += 1;
                }
            }
        }

        Ok(removed_count)
    }

    /// Generate optimization report
    pub fn generate_report(&self) -> OptimizationReport {
        let cache_hit_rate = if self.instantiation_metrics.total_instantiations > 0 {
            self.instantiation_metrics.cache_hits as f64 / self.instantiation_metrics.total_instantiations as f64
        } else {
            0.0
        };

        OptimizationReport {
            total_instantiations: self.instantiation_metrics.total_instantiations,
            cache_hit_rate,
            avg_instantiation_time: self.instantiation_metrics.avg_instantiation_time,
            total_specializations: self.specializations.values().map(|v| v.len()).sum(),
            memory_usage: self.instantiation_metrics.template_memory_usage,
            code_size_impact: self.instantiation_metrics.total_code_size,
        }
    }

    // Helper methods
    
    fn generate_cache_key(&self, base_type: &TypeExpression, type_args: &[TypeExpression]) -> String {
        format!("{}_{}", 
                base_type.name.as_ref().unwrap_or(&"unknown".to_string()),
                type_args.iter()
                    .map(|t| t.name.as_deref().unwrap_or("unknown"))
                    .collect::<Vec<_>>()
                    .join("_"))
    }

    fn generate_instance_id(&self, base_type: &TypeExpression, type_args: &[TypeExpression]) -> String {
        format!("inst_{}_{}", 
                base_type.name.as_ref().unwrap_or(&"unknown".to_string()),
                self.instantiation_metrics.total_instantiations)
    }

    fn is_primitive_type(&self, type_expr: &TypeExpression) -> bool {
        matches!(type_expr.name.as_deref(), 
                Some("int") | Some("float") | Some("bool") | Some("string"))
    }

    fn estimate_type_size(&self, type_args: &[TypeExpression]) -> usize {
        type_args.iter().map(|_| 8).sum() // Simplified size estimation
    }

    fn estimate_specialized_size(&self, _base_type: &TypeExpression, type_args: &[TypeExpression], _spec: &Specialization) -> usize {
        // Specialized code is typically smaller due to optimizations
        self.estimate_generic_size(_base_type, type_args) * 70 / 100
    }

    fn estimate_generic_size(&self, _base_type: &TypeExpression, type_args: &[TypeExpression]) -> usize {
        // Base size + parameter overhead
        100 + type_args.len() * 50
    }

    fn compute_code_signature(&self, instance: &MonomorphizedInstance) -> String {
        // Simplified signature computation
        format!("{}_{:?}", instance.base_type.name.as_ref().unwrap_or(&"unknown".to_string()), 
                instance.optimization_level)
    }

    fn cache_instance(&mut self, key: String, instance: MonomorphizedInstance) {
        if self.monomorphization_cache.len() >= self.config.max_cache_size {
            // Simple LRU eviction (remove first entry)
            if let Some(first_key) = self.monomorphization_cache.keys().next().cloned() {
                self.monomorphization_cache.remove(&first_key);
            }
        }
        self.monomorphization_cache.insert(key, instance);
    }

    fn update_avg_time(&mut self, elapsed: f64) {
        let total = self.instantiation_metrics.total_instantiations as f64;
        self.instantiation_metrics.avg_instantiation_time = 
            (self.instantiation_metrics.avg_instantiation_time * (total - 1.0) + elapsed) / total;
    }
}

/// Report on optimization performance
#[derive(Debug)]
pub struct OptimizationReport {
    pub total_instantiations: usize,
    pub cache_hit_rate: f64,
    pub avg_instantiation_time: f64,
    pub total_specializations: usize,
    pub memory_usage: usize,
    pub code_size_impact: usize,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            max_cache_size: 1000,
            enable_specialization: true,
            size_threshold: 10000,
            enable_deduplication: true,
        }
    }
}

impl OptimizationConfig {
    fn optimization_level(&self) -> OptimizationLevel {
        if self.enable_specialization {
            OptimizationLevel::Aggressive
        } else {
            OptimizationLevel::Basic
        }
    }
}

/// Utility functions for generic optimization
pub mod optimization_utils {
    use super::*;

    /// Create common specializations for built-in types
    pub fn create_builtin_specializations() -> Vec<(String, Specialization)> {
        vec![
            // Array<int> specialization
            ("Array".to_string(), Specialization {
                pattern: SpecializationPattern::ExactMatch(vec![TypeExpression::named("int")]),
                optimized_impl: "optimized_int_array".to_string(),
                benefit_score: 0.8,
            }),
            // Map<string, T> specialization
            ("Map".to_string(), Specialization {
                pattern: SpecializationPattern::Custom("string_key".to_string()),
                optimized_impl: "optimized_string_map".to_string(),
                benefit_score: 0.7,
            }),
        ]
    }

    /// Estimate optimization benefit for a type instantiation
    pub fn estimate_benefit(base_type: &TypeExpression, type_args: &[TypeExpression]) -> f64 {
        let mut benefit: f64 = 0.0;
        
        // Benefit increases with primitive types
        for arg in type_args {
            if matches!(arg.name.as_deref(), Some("int") | Some("float") | Some("bool")) {
                benefit += 0.2;
            }
        }
        
        // Benefit increases with common patterns
        if let Some(name) = &base_type.name {
            match name.as_str() {
                "Array" | "List" => benefit += 0.3,
                "Map" | "HashMap" => benefit += 0.4,
                _ => {}
            }
        }
        
        benefit.min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_optimization() {
        let mut optimizer = GenericOptimizer::new();
        
        let base_type = TypeExpression::named("Array");
        let type_args = vec![TypeExpression::named("int")];
        
        let instance = optimizer.optimize_instantiation(&base_type, &type_args).unwrap();
        assert_eq!(instance.base_type.name, Some("Array".to_string()));
        assert_eq!(instance.type_arguments.len(), 1);
    }

    #[test]
    fn test_specialization_matching() {
        let optimizer = GenericOptimizer::new();
        
        let pattern = SpecializationPattern::PrimitiveOnly;
        let type_args = vec![TypeExpression::named("int"), TypeExpression::named("bool")];
        
        assert!(optimizer.matches_pattern(&pattern, &type_args).unwrap());
    }

    #[test] 
    fn test_cache_functionality() {
        let mut optimizer = GenericOptimizer::new();
        
        let base_type = TypeExpression::named("List");
        let type_args = vec![TypeExpression::named("string")];
        
        // First instantiation
        let _instance1 = optimizer.optimize_instantiation(&base_type, &type_args).unwrap();
        assert_eq!(optimizer.instantiation_metrics.total_instantiations, 1);
        assert_eq!(optimizer.instantiation_metrics.cache_hits, 0);
        
        // Second instantiation should hit cache
        let _instance2 = optimizer.optimize_instantiation(&base_type, &type_args).unwrap();
        assert_eq!(optimizer.instantiation_metrics.total_instantiations, 1);
        assert_eq!(optimizer.instantiation_metrics.cache_hits, 1);
    }

    #[test]
    fn test_optimization_report() {
        let mut optimizer = GenericOptimizer::new();
        
        let base_type = TypeExpression::named("Vector");
        let type_args = vec![TypeExpression::named("float")];
        
        let _instance = optimizer.optimize_instantiation(&base_type, &type_args).unwrap();
        
        let report = optimizer.generate_report();
        assert_eq!(report.total_instantiations, 1);
        assert!(report.cache_hit_rate >= 0.0 && report.cache_hit_rate <= 1.0);
    }
}
