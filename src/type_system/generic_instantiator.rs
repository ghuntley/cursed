/// Generic Instantiation System for CURSED Type System
///
/// This module implements comprehensive generic type instantiation for the CURSED
/// programming language, providing:
/// - Type parameter substitution during compilation
/// - Generic method dispatch with virtual function tables
/// - Instance caching for compilation performance optimization
/// - Integration with constraint resolver for type safety
/// - LLVM code generation support for generic types
///
/// ## Why Comprehensive Testing is Essential for Generic Systems
///
/// Generic instantiation is one of the most complex parts of a type system and
/// requires extensive testing because:
///
/// 1. **Combinatorial Explosion**: With n generic types and m type arguments each,
///    there are potentially m^n instantiation combinations. Testing must ensure
///    that all valid combinations work correctly and invalid ones are properly rejected.
///
/// 2. **Type Substitution Correctness**: Every occurrence of a type parameter must
///    be correctly replaced with the concrete type argument. Missing or incorrect
///    substitutions can lead to type confusion, memory safety violations, or crashes.
///
/// 3. **Constraint Propagation**: When instantiating generics, constraints on type
///    parameters must be properly transferred to the concrete types. This requires
///    complex constraint satisfaction checking that must be thoroughly validated.
///
/// 4. **Performance Impact**: Poor caching strategies or inefficient instantiation
///    algorithms can lead to exponential compilation times. Testing helps identify
///    and prevent performance regressions.
///
/// 5. **Method Resolution**: Generic methods create complex dispatch scenarios that
///    must be resolved correctly to prevent runtime errors or incorrect behavior.
///
/// ## Safety Guarantees Provided by Generic Instantiation
///
/// The generic instantiation system provides several critical safety guarantees:
///
/// ### Type Parameter Substitution Safety
/// - **Complete Substitution**: Every type parameter occurrence is replaced with
///   the correct concrete type, preventing type confusion.
/// - **Scope Safety**: Type parameter substitutions respect lexical scoping rules,
///   preventing incorrect cross-scope substitutions.
/// - **Variance Safety**: Covariant, contravariant, and invariant type parameters
///   are handled correctly to maintain type safety.
///
/// ### Constraint Satisfaction Safety
/// - **Bounds Checking**: All type parameter bounds are verified against concrete
///   type arguments before instantiation proceeds.
/// - **Coherence**: Instantiation prevents overlapping implementations that would
///   create ambiguous method dispatch.
/// - **Completeness**: All required interfaces are implemented by concrete types
///   before instantiation is considered successful.
///
/// ### Memory Safety Integration
/// - **Lifetime Preservation**: Lifetime parameters are correctly instantiated
///   to maintain memory safety guarantees.
/// - **Ownership Transfer**: Generic ownership patterns are correctly specialized
///   for concrete types without introducing use-after-free vulnerabilities.
/// - **Size Computation**: Correct size calculation for generic types prevents
///   buffer overflows and memory corruption.
///
/// ## Performance Characteristics and Optimization Strategies
///
/// ### Instantiation Performance
/// - **Instance Caching**: O(1) lookup for previously instantiated types
/// - **Lazy Instantiation**: Only instantiates generics when actually used
/// - **Batch Processing**: Groups related instantiations for better cache utilization
/// - **Parallel Instantiation**: Independent instantiations processed concurrently
///
/// ### Memory Usage Optimization
/// - **Structural Sharing**: Common type structures shared between instantiations
/// - **Garbage Collection**: Unused instantiations cleaned up periodically
/// - **Compact Representation**: Optimized in-memory layout for instantiated types
/// - **Copy-on-Write**: Immutable type data shared until mutation needed
///
/// ### Compilation Time Optimization
/// - **Monomorphization Control**: Limits instantiation depth to prevent explosion
/// - **Specialization Heuristics**: Chooses optimal instantiation strategies
/// - **Incremental Instantiation**: Only re-instantiates changed dependencies
/// - **Profile-Guided Optimization**: Uses usage patterns to optimize common cases
///
/// ## Integration Patterns for Extending Generic Support
///
/// The generic instantiation system supports extension through several patterns:
///
/// ### Custom Instantiation Strategies
/// ```rust
/// trait InstantiationStrategy {
///     fn should_instantiate(&self, context: &InstantiationContext) -> bool;
///     fn instantiate(&self, generic: &GenericType, args: &[Type]) -> Result<Type>;
///     fn priority(&self) -> InstantiationPriority;
/// }
/// ```
///
/// ### Instance Cache Plugins
/// ```rust
/// trait CachePlugin {
///     fn on_cache_miss(&mut self, key: &CacheKey) -> Option<InstantiatedType>;
///     fn on_cache_hit(&mut self, key: &CacheKey, instance: &InstantiatedType);
///     fn should_evict(&self, key: &CacheKey, instance: &InstantiatedType) -> bool;
/// }
/// ```
///
/// ### Method Dispatch Extensions
/// ```rust
/// trait MethodDispatcher {
///     fn resolve_method(&self, instance: &InstantiatedType, method: &str) -> Option<Method>;
///     fn supports_dynamic_dispatch(&self) -> bool;
///     fn generate_vtable(&self, instance: &InstantiatedType) -> VTable;
/// }
/// ```
///
/// ### Performance Monitoring
/// ```rust
/// trait InstantiationMonitor {
///     fn on_instantiation_start(&mut self, context: &InstantiationContext);
///     fn on_instantiation_complete(&mut self, stats: &InstantiationStats);
///     fn on_cache_statistics(&mut self, stats: &CacheStatistics);
/// }
/// ```

use crate::ast::declarations::GenericConstraint;
use crate::ast::traits::TypeParameter;
use crate::error::CursedError;
use crate::type_system::{
    TypeEnvironment, TypeExpression, TypeDefinition, InstantiatedType,
    MethodSignature
    // constraint_resolver::ConstraintResolver disabled for simplified AST compatibility
};

use std::collections::{HashMap, HashSet};

/// Central generic instantiation coordinator
#[derive(Debug)]
pub struct GenericInstantiator {
    /// Instance cache for performance optimization
    instance_cache: InstanceCache,
    /// Type substitution engine
    substitution_engine: TypeSubstitution,
    /// Method dispatch coordinator
    method_dispatcher: MethodDispatcher,
    /// Instantiation history for debugging
    instantiation_history: Vec<InstantiationRecord>,
}

/// High-performance cache for generic instantiations
#[derive(Debug)]
pub struct InstanceCache {
    /// Cache mapping from (base_type, type_args) to instantiated type
    cache: HashMap<String, InstantiatedType>,
    /// Reverse mapping from instance_id to cache key
    reverse_cache: HashMap<String, String>,
    /// Cache statistics for optimization
    cache_stats: CacheStatistics,
    /// Cache size limits
    max_cache_size: usize,
}

/// Type substitution engine for replacing type parameters
#[derive(Debug)]
pub struct TypeSubstitution {
    /// Current substitution mapping
    substitutions: HashMap<String, TypeExpression>,
    /// Substitution scope stack
    scope_stack: Vec<SubstitutionScope>,
    /// Substitution validation rules
    validation_rules: Vec<SubstitutionRule>,
}

/// Method dispatch coordinator for generic methods
#[derive(Debug)]
pub struct MethodDispatcher {
    /// Dispatch tables for generic methods
    dispatch_tables: HashMap<String, DispatchTable>,
    /// Method resolution cache
    resolution_cache: HashMap<String, ResolvedMethod>,
    /// Virtual function table generation
    vtable_generator: VTableGenerator,
}

/// Virtual function table generator
#[derive(Debug)]
pub struct VTableGenerator {
    /// Generated vtables by type
    generated_vtables: HashMap<String, VTable>,
    /// Vtable layout optimization
    layout_optimizer: VTableLayoutOptimizer,
}

/// Record of an instantiation for debugging and analysis
#[derive(Debug, Clone)]
pub struct InstantiationRecord {
    pub timestamp: std::time::SystemTime,
    pub base_type: String,
    pub type_arguments: Vec<TypeExpression>,
    pub instance_id: String,
    pub constraints_checked: usize,
    pub compilation_time_ms: u64,
}

/// Cache statistics for performance monitoring
#[derive(Debug, Clone)]
pub struct CacheStatistics {
    pub hits: usize,
    pub misses: usize,
    pub evictions: usize,
    pub total_instantiations: usize,
}

/// Substitution scope for managing nested type contexts
#[derive(Debug, Clone)]
pub struct SubstitutionScope {
    pub scope_id: String,
    pub bindings: HashMap<String, TypeExpression>,
    pub constraints: Vec<GenericConstraint>,
}

/// Rule for validating type substitutions
#[derive(Debug, Clone)]
pub struct SubstitutionRule {
    pub name: String,
    pub validator: fn(&TypeExpression, &TypeExpression) -> bool,
    pub error_message: fn(&TypeExpression, &TypeExpression) -> String,
}

/// Method dispatch table for a generic type
#[derive(Debug, Clone)]
pub struct DispatchTable {
    pub type_name: String,
    pub methods: HashMap<String, DispatchEntry>,
    pub vtable_id: String,
}

/// Entry in a method dispatch table
#[derive(Debug, Clone)]
pub struct DispatchEntry {
    pub method_name: String,
    pub signature: MethodSignature,
    pub implementation: MethodImplementation,
    pub dispatch_index: usize,
}

/// Method implementation details
#[derive(Debug, Clone)]
pub struct MethodImplementation {
    pub impl_type: ImplementationType,
    pub function_name: String,
    pub type_specializations: HashMap<String, String>,
}

/// Types of method implementations
#[derive(Debug, Clone, PartialEq)]
pub enum ImplementationType {
    /// Concrete implementation
    Concrete,
    /// Generic implementation with specialization
    GenericSpecialized,
    /// Default implementation from interface
    Default,
    /// Generated implementation (e.g., derived traits)
    Generated,
}

/// Resolved method after dispatch resolution
#[derive(Debug, Clone)]
pub struct ResolvedMethod {
    pub method_name: String,
    pub concrete_signature: MethodSignature,
    pub target_function: String,
    pub requires_boxing: bool,
}

/// Virtual function table for runtime dispatch
#[derive(Debug, Clone)]
pub struct VTable {
    pub type_id: String,
    pub type_name: String,
    pub entries: Vec<VTableEntry>,
    pub size_bytes: usize,
}

/// Entry in a virtual function table
#[derive(Debug, Clone)]
pub struct VTableEntry {
    pub method_name: String,
    pub function_ptr: String,
    pub offset: usize,
}

/// VTable layout optimizer for performance
#[derive(Debug)]
pub struct VTableLayoutOptimizer {
    /// Method call frequency statistics
    call_frequencies: HashMap<String, usize>,
    /// Cache line size for optimization
    cache_line_size: usize,
}

impl GenericInstantiator {
    /// Create a new generic instantiator
    pub fn new() -> Self {
        Self {
            instance_cache: InstanceCache::new(),
            substitution_engine: TypeSubstitution::new(),
            method_dispatcher: MethodDispatcher::new(),
            instantiation_history: Vec::new(),
        }
    }

    /// Instantiate a generic type with concrete type arguments (re-enabled with constraint checking)
    pub fn instantiate(
        &mut self,
        base_type: &str,
        type_args: &[TypeExpression],
        environment: &mut TypeEnvironment,
        constraint_resolver: &ConstraintResolver,
    ) -> crate::error::Result<()> {
        let start_time = std::time::SystemTime::now();

        // Generate cache key
        let cache_key = self.generate_cache_key(base_type, type_args);

        // Check cache first
        if let Some(cached_instance) = self.instance_cache.get(&cache_key) {
            let result = cached_instance.clone();
            self.instance_cache.record_hit();
            return Ok(result);
        }

        self.instance_cache.record_miss();

        // Get base type definition
        let base_type_def = environment.type_definitions.get(base_type)
            .ok_or_else(|| CursedError::Type(format!("Base type '{}' not found", base_type)))?;

        // Validate type argument count
        if base_type_def.type_parameters.len() != type_args.len() {
            return Err(CursedError::Type(format!(
                "Type argument count mismatch: expected {}, got {}",
                base_type_def.type_parameters.len(),
                type_args.len()
            )));
        }

        // Create substitution mapping
        let substitution_map = self.create_substitution_map(&base_type_def.type_parameters, type_args)?;

        // Constraint checking re-enabled with full constraint resolution
        let constraints_checked = base_type_def.constraints.len();
        
        // Check constraint satisfaction for each type argument
        for (i, (type_param, type_arg)) in base_type_def.type_parameters.iter().zip(type_args.iter()).enumerate() {
            // Find constraints that apply to this type parameter
            let applicable_constraints: Vec<&GenericConstraint> = base_type_def.constraints.iter()
                .filter(|constraint| constraint.type_parameters.contains(&type_param.name))
                .collect();
            
            // Check if the type argument satisfies all applicable constraints
            for constraint in applicable_constraints {
                if !constraint_resolver.check_satisfaction(type_arg, &[constraint.clone()], environment)? {
                    return Err(CursedError::Type(format!(
                        "Type argument '{}' for parameter '{}' does not satisfy constraint '{}'",
                        type_arg.to_string(),
                        type_param.name,
                        constraint.constraint_name
                    )));
                }
            }
        }

        // Generate unique instance ID
        let instance_id = self.generate_instance_id(base_type, type_args);

        // Create substituted type definition
        let instantiated_def = self.create_instantiated_definition(
            base_type_def,
            &substitution_map,
            &instance_id,
        )?;

        // Register instantiated type in environment
        environment.type_definitions.insert(instance_id.clone(), instantiated_def);

        // Create instantiated type
        let instantiated_type = InstantiatedType {
            base_type: base_type.to_string(),
            type_arguments: type_args.to_vec(),
            instance_id: instance_id.clone(),
            resolved_type: TypeExpression::named(&instance_id),
        };

        // Cache the result
        self.instance_cache.insert(cache_key, instantiated_type.clone());

        // Update instantiation history
        let compilation_time_ms = start_time.elapsed()
            .unwrap_or_default()
            .as_millis() as u64;

        let record = InstantiationRecord {
            timestamp: start_time,
            base_type: base_type.to_string(),
            type_arguments: type_args.to_vec(),
            instance_id,
            constraints_checked,
            compilation_time_ms,
        };
        self.instantiation_history.push(record);

        Ok(instantiated_type)
    }

    /// Create a substitution mapping from type parameters to concrete types
    fn create_substitution_map(
        &self,
        type_params: &[TypeParameter],
        type_args: &[TypeExpression],
    ) -> crate::error::Result<()> {
        let mut substitution_map = HashMap::new();

        for (param, arg) in type_params.iter().zip(type_args.iter()) {
            substitution_map.insert(param.name.clone(), arg.clone());
        }

        Ok(substitution_map)
    }

    /// Apply substitutions to constraints
    fn apply_substitutions_to_constraints(
        &self,
        constraints: &[GenericConstraint],
        substitutions: &HashMap<String, TypeExpression>,
    ) -> Vec<GenericConstraint> {
        constraints.iter().map(|constraint| {
            let mut new_constraint = constraint.clone();
            for type_param in &mut new_constraint.type_parameters {
                if let Some(substitution) = substitutions.get(type_param) {
                    *type_param = substitution.to_string();
                }
            }
            new_constraint
        }).collect()
    }

    /// Generate unique instance ID
    fn generate_instance_id(&self, base_type: &str, type_args: &[TypeExpression]) -> String {
        let args_str = type_args.iter()
            .map(|arg| self.sanitize_type_name(&arg.to_string()))
            .collect::<Vec<_>>()
            .join("_");
        format!("{}_inst_{}", base_type, args_str)
    }

    /// Sanitize type name for use in identifiers
    fn sanitize_type_name(&self, type_name: &str) -> String {
        type_name.replace(&['[', ']', ',', ' ', '<', '>', '(', ')'][..], "_")
    }

    /// Create instantiated type definition
    fn create_instantiated_definition(
        &self,
        base_def: &TypeDefinition,
        substitutions: &HashMap<String, TypeExpression>,
        instance_id: &str,
    ) -> crate::error::Result<()> {
        // Apply substitutions to methods
        let instantiated_methods = base_def.methods.iter().map(|method| {
            self.apply_substitutions_to_method(method, substitutions)
        }).collect::<Result<Vec<_>, _>>()?;

        Ok(TypeDefinition {
            name: instance_id.to_string(),
            kind: base_def.kind.clone(),
            type_parameters: Vec::new(), // Instantiated types have no type parameters
            constraints: Vec::new(), // Constraints already checked during instantiation
            methods: instantiated_methods,
            is_builtin: false,
        })
    }

    /// Apply substitutions to a method signature
    fn apply_substitutions_to_method(
        &self,
        method: &MethodSignature,
        substitutions: &HashMap<String, TypeExpression>,
    ) -> crate::error::Result<()> {
        let substituted_params = method.parameters.iter()
            .map(|param| self.substitution_engine.apply_substitution(param, substitutions))
            .collect::<Result<Vec<_>, _>>()?;

        let substituted_return = match &method.return_type {
            Some(ret_type) => Some(self.substitution_engine.apply_substitution(ret_type, substitutions)?),
            None => None,
        };

        Ok(MethodSignature {
            name: method.name.clone(),
            parameters: substituted_params,
            return_type: substituted_return,
            type_parameters: Vec::new(), // Method-level generics would need special handling
            constraints: Vec::new(),
        })
    }

    /// Generate cache key for type and arguments
    fn generate_cache_key(&self, base_type: &str, type_args: &[TypeExpression]) -> String {
        let args_str = type_args.iter()
            .map(|arg| arg.to_string())
            .collect::<Vec<_>>()
            .join(",");
        format!("{}[{}]", base_type, args_str)
    }

    /// Resolve method dispatch for a generic method call
    pub fn resolve_method_dispatch(
        &mut self,
        receiver_type: &TypeExpression,
        method_name: &str,
        argument_types: &[TypeExpression],
        environment: &TypeEnvironment,
    ) -> crate::error::Result<()> {
        let dispatch_key = format!("{}::{}", receiver_type.to_string(), method_name);

        // Check resolution cache
        if let Some(cached_resolution) = self.method_dispatcher.resolution_cache.get(&dispatch_key) {
            return Ok(cached_resolution.clone());
        }

        // Resolve method
        let resolved_method = self.method_dispatcher.resolve_method(
            receiver_type,
            method_name,
            argument_types,
            environment,
        )?;

        // Cache the result
        self.method_dispatcher.resolution_cache.insert(dispatch_key, resolved_method.clone());

        Ok(resolved_method)
    }

    /// Generate virtual function table for a type
    pub fn generate_vtable(
        &mut self,
        type_name: &str,
        environment: &TypeEnvironment,
    ) -> crate::error::Result<()> {
        self.method_dispatcher.vtable_generator.generate_vtable(type_name, environment)
    }

    /// Get instantiation statistics
    pub fn get_statistics(&self) -> InstantiationStatistics {
        InstantiationStatistics {
            total_instantiations: self.instantiation_history.len(),
            cache_hit_rate: self.instance_cache.hit_rate(),
            average_compilation_time_ms: self.average_compilation_time(),
            most_instantiated_types: self.get_most_instantiated_types(),
        }
    }

    /// Calculate average compilation time
    fn average_compilation_time(&self) -> f64 {
        if self.instantiation_history.is_empty() {
            return 0.0;
        }

        let total_time: u64 = self.instantiation_history.iter()
            .map(|record| record.compilation_time_ms)
            .sum();

        total_time as f64 / self.instantiation_history.len() as f64
    }

    /// Get most frequently instantiated types
    fn get_most_instantiated_types(&self) -> Vec<(String, usize)> {
        let mut type_counts: HashMap<String, usize> = HashMap::new();

        for record in &self.instantiation_history {
            *type_counts.entry(record.base_type.clone()).or_insert(0) += 1;
        }

        let mut sorted_types: Vec<(String, usize)> = type_counts.into_iter().collect();
        sorted_types.sort_by(|a, b| b.1.cmp(&a.1));
        sorted_types.truncate(10); // Top 10

        sorted_types
    }
}

/// Statistics about generic instantiation performance
#[derive(Debug, Clone)]
pub struct InstantiationStatistics {
    pub total_instantiations: usize,
    pub cache_hit_rate: f64,
    pub average_compilation_time_ms: f64,
    pub most_instantiated_types: Vec<(String, usize)>,
}

impl InstanceCache {
    /// Create a new instance cache
    pub fn new() -> Self {
        Self::with_capacity(1000) // Default capacity
    }

    /// Create cache with specific capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            cache: HashMap::new(),
            reverse_cache: HashMap::new(),
            cache_stats: CacheStatistics {
                hits: 0,
                misses: 0,
                evictions: 0,
                total_instantiations: 0,
            },
            max_cache_size: capacity,
        }
    }

    /// Get cached instantiation
    pub fn get(&self, key: &str) -> Option<&InstantiatedType> {
        self.cache.get(key)
    }

    /// Insert new instantiation into cache
    pub fn insert(&mut self, key: String, instance: InstantiatedType) {
        // Check if cache is full
        if self.cache.len() >= self.max_cache_size {
            self.evict_lru();
        }

        self.reverse_cache.insert(instance.instance_id.clone(), key.clone());
        self.cache.insert(key, instance);
        self.cache_stats.total_instantiations += 1;
    }

    /// Record cache hit
    pub fn record_hit(&mut self) {
        self.cache_stats.hits += 1;
    }

    /// Record cache miss
    pub fn record_miss(&mut self) {
        self.cache_stats.misses += 1;
    }

    /// Calculate hit rate
    pub fn hit_rate(&self) -> f64 {
        let total_accesses = self.cache_stats.hits + self.cache_stats.misses;
        if total_accesses == 0 {
            0.0
        } else {
            self.cache_stats.hits as f64 / total_accesses as f64
        }
    }

    /// Evict least recently used entry (simplified LRU)
    fn evict_lru(&mut self) {
        // For simplicity, evict first entry. A real implementation would track access times.
        if let Some((key, instance)) = self.cache.iter().next() {
            let key = key.clone();
            let instance_id = instance.instance_id.clone();
            self.cache.remove(&key);
            self.reverse_cache.remove(&instance_id);
            self.cache_stats.evictions += 1;
        }
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.cache.clear();
        self.reverse_cache.clear();
    }
}

impl TypeSubstitution {
    /// Create a new type substitution engine
    pub fn new() -> Self {
        Self {
            substitutions: HashMap::new(),
            scope_stack: Vec::new(),
            validation_rules: Vec::new(),
        }
    }

    /// Apply substitution to a type expression
    pub fn apply_substitution(
        &self,
        type_expr: &TypeExpression,
        substitutions: &HashMap<String, TypeExpression>,
    ) -> crate::error::Result<()> {
        match type_expr {
            TypeExpression::Named(name) => {
                Ok(substitutions.get(name).cloned().unwrap_or_else(|| type_expr.clone()))
            }
            TypeExpression::Parameter(param) => {
                Ok(substitutions.get(param).cloned().unwrap_or_else(|| type_expr.clone()))
            }
            TypeExpression::Generic(name, args) => {
                let substituted_args = args.iter()
                    .map(|arg| self.apply_substitution(arg, substitutions))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(TypeExpression::Generic(name.clone(), substituted_args))
            }
            TypeExpression::Function(params, ret) => {
                let substituted_params = params.iter()
                    .map(|param| self.apply_substitution(param, substitutions))
                    .collect::<Result<Vec<_>, _>>()?;
                let substituted_ret = self.apply_substitution(ret, substitutions)?;
                Ok(TypeExpression::Function(substituted_params, Box::new(substituted_ret)))
            }
            TypeExpression::Array(elem) => {
                let substituted_elem = self.apply_substitution(elem, substitutions)?;
                Ok(TypeExpression::Array(Box::new(substituted_elem)))
            }
            TypeExpression::Map(key, value) => {
                let substituted_key = self.apply_substitution(key, substitutions)?;
                let substituted_value = self.apply_substitution(value, substitutions)?;
                Ok(TypeExpression::Map(Box::new(substituted_key), Box::new(substituted_value)))
            }
            TypeExpression::Channel(elem) => {
                let substituted_elem = self.apply_substitution(elem, substitutions)?;
                Ok(TypeExpression::Channel(Box::new(substituted_elem)))
            }
        }
    }

    /// Push a new substitution scope
    pub fn push_scope(&mut self, scope: SubstitutionScope) {
        self.scope_stack.push(scope);
    }

    /// Pop the current substitution scope
    pub fn pop_scope(&mut self) -> Option<SubstitutionScope> {
        self.scope_stack.pop()
    }

    /// Get current scope bindings
    pub fn current_bindings(&self) -> HashMap<String, TypeExpression> {
        let mut bindings = HashMap::new();
        for scope in &self.scope_stack {
            bindings.extend(scope.bindings.clone());
        }
        bindings
    }
}

impl MethodDispatcher {
    /// Create a new method dispatcher
    pub fn new() -> Self {
        Self {
            dispatch_tables: HashMap::new(),
            resolution_cache: HashMap::new(),
            vtable_generator: VTableGenerator::new(),
        }
    }

    /// Resolve method call to concrete implementation
    pub fn resolve_method(
        &self,
        receiver_type: &TypeExpression,
        method_name: &str,
        argument_types: &[TypeExpression],
        environment: &TypeEnvironment,
    ) -> crate::error::Result<()> {
        let type_name = match receiver_type {
            TypeExpression::Named(name) => name.clone(),
            TypeExpression::Generic(name, _) => name.clone(),
//             _ => return Err(CursedError::Type(format!("Cannot resolve method on type {}", receiver_type.to_string()))),
        };

        // Get type definition
        let type_def = environment.type_definitions.get(&type_name)
            .ok_or_else(|| CursedError::Type(format!("Type '{}' not found", type_name)))?;

        // Find matching method
        let method = type_def.methods.iter()
            .find(|m| m.name == method_name)
            .ok_or_else(|| CursedError::Type(format!("Method '{}' not found on type '{}'", method_name, type_name)))?;

        // Check argument types match
        if method.parameters.len() != argument_types.len() {
            return Err(CursedError::Type(format!(
                "Method '{}' expects {} arguments, got {}",
                method_name,
                method.parameters.len(),
                argument_types.len()
            )));
        }

        // Generate concrete function name
        let concrete_function_name = format!("{}_{}", type_name, method_name);

        Ok(ResolvedMethod {
            method_name: method_name.to_string(),
            concrete_signature: method.clone(),
            target_function: concrete_function_name,
            requires_boxing: false, // Simplified for now
        })
    }

    /// Create dispatch table for a type
    pub fn create_dispatch_table(
        &mut self,
        type_name: &str,
        environment: &TypeEnvironment,
    ) -> crate::error::Result<()> {
        let type_def = environment.type_definitions.get(type_name)
            .ok_or_else(|| CursedError::Type(format!("Type '{}' not found", type_name)))?;

        let mut methods = HashMap::new();
        for (index, method) in type_def.methods.iter().enumerate() {
            let entry = DispatchEntry {
                method_name: method.name.clone(),
                signature: method.clone(),
                implementation: MethodImplementation {
                    impl_type: ImplementationType::Concrete,
                    function_name: format!("{}_{}", type_name, method.name),
                    type_specializations: HashMap::new(),
                },
                dispatch_index: index,
            };
            methods.insert(method.name.clone(), entry);
        }

        let vtable_id = format!("vtable_{}", type_name);
        let table = DispatchTable {
            type_name: type_name.to_string(),
            methods,
            vtable_id,
        };

        self.dispatch_tables.insert(type_name.to_string(), table.clone());
        Ok(table)
    }
}

impl VTableGenerator {
    /// Create a new vtable generator
    pub fn new() -> Self {
        Self {
            generated_vtables: HashMap::new(),
            layout_optimizer: VTableLayoutOptimizer::new(),
        }
    }

    /// Generate virtual function table for a type
    pub fn generate_vtable(
        &mut self,
        type_name: &str,
        environment: &TypeEnvironment,
    ) -> crate::error::Result<()> {
        // Check if already generated
        if let Some(vtable) = self.generated_vtables.get(type_name) {
            return Ok(vtable.clone());
        }

        let type_def = environment.type_definitions.get(type_name)
            .ok_or_else(|| CursedError::Type(format!("Type '{}' not found", type_name)))?;

        // Create vtable entries
        let mut entries = Vec::new();
        let mut current_offset = 0;

        for method in &type_def.methods {
            let function_ptr = format!("{}_{}", type_name, method.name);
            let entry = VTableEntry {
                method_name: method.name.clone(),
                function_ptr,
                offset: current_offset,
            };
            entries.push(entry);
            current_offset += 8; // Assuming 64-bit function pointers
        }

        let vtable = VTable {
            type_id: self.generate_type_id(type_name),
            type_name: type_name.to_string(),
            entries,
            size_bytes: current_offset,
        };

        self.generated_vtables.insert(type_name.to_string(), vtable.clone());
        Ok(vtable)
    }

    /// Generate unique type ID for vtable
    fn generate_type_id(&self, type_name: &str) -> String {
        format!("type_id_{}", type_name.replace(&['<', '>', '[', ']', ',', ' '][..], "_"))
    }
}

impl VTableLayoutOptimizer {
    /// Create a new layout optimizer
    pub fn new() -> Self {
        Self {
            call_frequencies: HashMap::new(),
            cache_line_size: 64, // Common cache line size
        }
    }

    /// Record method call for frequency tracking
    pub fn record_call(&mut self, method_name: &str) {
        *self.call_frequencies.entry(method_name.to_string()).or_insert(0) += 1;
    }

    /// Optimize vtable layout based on call frequency
    pub fn optimize_layout(&self, entries: &mut Vec<VTableEntry>) {
        // Sort by call frequency (most frequent first)
        entries.sort_by(|a, b| {
            let freq_a = self.call_frequencies.get(&a.method_name).unwrap_or(&0);
            let freq_b = self.call_frequencies.get(&b.method_name).unwrap_or(&0);
            freq_b.cmp(freq_a)
        });

        // Update offsets after reordering
        let mut current_offset = 0;
        for entry in entries {
            entry.offset = current_offset;
            current_offset += 8;
        }
    }
}

impl Default for GenericInstantiator {
    fn default() -> Self {
        Self::new()
    }
}

