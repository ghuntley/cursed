use crate::error::Error;
// Generic Optimization System for CURSED Language
//
// This module provides advanced optimization techniques for generic code,
// including monomorphization, JIT compilation, and memory layout optimization.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use tracing::{debug, error, info, warn, instrument};

use crate::ast::types::Type;
use crate::ast::traits::TypeParameter;
use crate::error::CursedError;

/// Strategy for handling generic instantiation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InstantiationStrategy {
    /// Generate specialized code for each type combination (monomorphization)
    Monomorphization,
    /// Use dynamic dispatch with vtables
    DynamicDispatch,
    /// Use JIT compilation for hot paths
    JustInTimeCompilation,
    /// Hybrid approach based on usage patterns
    Adaptive,
}

/// Information about generic function usage
#[derive(Debug, Clone)]
pub struct GenericUsageInfo {
    /// Function or type name
    pub name: String,
    /// Type instantiations and their usage counts
    pub instantiations: HashMap<Vec<Type>, usize>,
    /// Total call count
    pub total_calls: usize,
    /// Average execution time per call (in nanoseconds)
    pub avg_execution_time: u64,
    /// Memory usage statistics
    pub memory_usage: MemoryUsageStats,
}

/// Memory usage statistics for generic instantiations
#[derive(Debug, Clone)]
pub struct MemoryUsageStats {
    /// Total allocated bytes
    pub total_allocated: usize,
    /// Peak memory usage
    pub peak_usage: usize,
    /// Average object size
    pub avg_object_size: usize,
    /// Fragmentation level (0.0 = no fragmentation, 1.0 = high fragmentation)
    pub fragmentation_level: f64,
}

/// Optimization decision for a generic function
#[derive(Debug, Clone)]
pub struct OptimizationDecision {
    /// Chosen strategy
    pub strategy: InstantiationStrategy,
    /// Reason for the decision
    pub reason: String,
    /// Expected performance improvement
    pub expected_improvement: f64,
    /// Memory overhead estimation
    pub memory_overhead: usize,
}

/// Main optimization registry and decision engine
#[derive(Debug)]
pub struct GenericOptimizer {
    /// Usage statistics for generic functions
    usage_stats: RwLock<HashMap<String, GenericUsageInfo>>,
    /// Optimization decisions cache
    decisions_cache: RwLock<HashMap<String, OptimizationDecision>>,
    /// Configuration parameters
    config: OptimizationConfig,
    /// JIT compilation state
    jit_state: RwLock<JitCompilationState>,
}

/// Configuration for the optimization system
#[derive(Debug, Clone)]
pub struct OptimizationConfig {
    /// Threshold for monomorphization (number of instantiations)
    pub monomorphization_threshold: usize,
    /// Threshold for JIT compilation (call count)
    pub jit_threshold: usize,
    /// Maximum code size increase allowed for monomorphization
    pub max_code_size_increase: f64,
    /// Memory usage threshold for switching strategies
    pub memory_usage_threshold: usize,
    /// Enable adaptive optimization
    pub enable_adaptive: bool,
    /// Profile collection interval (in milliseconds)
    pub profile_interval: u64,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            monomorphization_threshold: 10,
            jit_threshold: 1000,
            max_code_size_increase: 2.0,
            memory_usage_threshold: 1024 * 1024, // 1MB
            enable_adaptive: true,
            profile_interval: 100,
        }
    }
}

/// JIT compilation state
#[derive(Debug)]
struct JitCompilationState {
    /// Functions currently being compiled
    compiling: HashSet<String>,
    /// Compiled functions and their performance
    compiled: HashMap<String, JitCompiledFunction>,
    /// Compilation failures
    failures: HashMap<String, String>,
}

/// Information about JIT compiled functions
#[derive(Debug, Clone)]
struct JitCompiledFunction {
    /// Compilation timestamp
    pub compiled_at: std::time::SystemTime,
    /// Execution time improvement ratio
    pub speedup_ratio: f64,
    /// Memory usage after compilation
    pub memory_usage: usize,
}

impl GenericOptimizer {
    /// Create a new optimizer with default configuration
    #[instrument]
    pub fn new() -> Self {
        Self::with_config(OptimizationConfig::default())
    }

    /// Create a new optimizer with custom configuration
    #[instrument]
    pub fn with_config(config: OptimizationConfig) -> Self {
        debug!("Creating GenericOptimizer with config: {:?}", config);
        Self {
            usage_stats: RwLock::new(HashMap::new()),
            decisions_cache: RwLock::new(HashMap::new()),
            config,
            jit_state: RwLock::new(JitCompilationState {
                compiling: HashSet::new(),
                compiled: HashMap::new(),
                failures: HashMap::new(),
            }),
        }
    }

    /// Record usage of a generic function
    #[instrument(skip(self))]
    pub fn record_usage(
        &self,
        function_name: &str,
        type_args: &[Type],
        execution_time: u64,
        memory_usage: usize,
    ) -> Result<(), Error> {
        let mut stats = self.usage_stats.write()
            .map_err(|_| CursedError::system_error("Failed to acquire write lock"))?;

        let usage_info = stats.entry(function_name.to_string()).or_insert_with(|| {
            GenericUsageInfo {
                name: function_name.to_string(),
                instantiations: HashMap::new(),
                total_calls: 0,
                avg_execution_time: 0,
                memory_usage: MemoryUsageStats {
                    total_allocated: 0,
                    peak_usage: 0,
                    avg_object_size: 0,
                    fragmentation_level: 0.0,
                },
            }
        });

        // Update instantiation count
        let instantiation_count = usage_info.instantiations
            .entry(type_args.to_vec())
            .or_insert(0);
        *instantiation_count += 1;

        // Update overall statistics
        usage_info.total_calls += 1;
        let total_time = usage_info.avg_execution_time as u64 * (usage_info.total_calls - 1) as u64;
        usage_info.avg_execution_time = ((total_time + execution_time) / usage_info.total_calls as u64) as u64;

        // Update memory statistics
        usage_info.memory_usage.total_allocated += memory_usage;
        usage_info.memory_usage.peak_usage = usage_info.memory_usage.peak_usage.max(memory_usage);
        usage_info.memory_usage.avg_object_size = 
            usage_info.memory_usage.total_allocated / usage_info.total_calls;

        debug!("Recorded usage for {}: {} calls, avg time: {}ns", 
               function_name, usage_info.total_calls, usage_info.avg_execution_time);

        Ok(())
    }

    /// Make optimization decision for a generic function
    #[instrument(skip(self))]
    pub fn make_optimization_decision(&self, function_name: &str) -> Result<(), Error> {
        // Check cache first
        {
            let cache = self.decisions_cache.read()
                .map_err(|_| CursedError::system_error("Failed to acquire read lock"))?;
            if let Some(decision) = cache.get(function_name) {
                debug!("Found cached optimization decision for {}", function_name);
                return Ok(decision.clone());
            }
        }

        // Analyze usage patterns and make decision
        let decision = self.analyze_and_decide(function_name)?;

        // Cache the decision
        {
            let mut cache = self.decisions_cache.write()
                .map_err(|_| CursedError::system_error("Failed to acquire write lock"))?;
            cache.insert(function_name.to_string(), decision.clone());
        }

        info!("Made optimization decision for {}: {:?}", function_name, decision.strategy);
        Ok(decision)
    }

    /// Internal analysis and decision logic
    #[instrument(skip(self))]
    fn analyze_and_decide(&self, function_name: &str) -> Result<(), Error> {
        let stats = self.usage_stats.read()
            .map_err(|_| CursedError::system_error("Failed to acquire read lock"))?;

        let usage_info = stats.get(function_name).ok_or_else(|| {
            CursedError::optimization_error(format!("No usage data for function: {}", function_name))
        })?;

        // Analyze different factors
        let instantiation_count = usage_info.instantiations.len();
        let total_calls = usage_info.total_calls;
        let avg_execution_time = usage_info.avg_execution_time;
        let memory_usage = usage_info.memory_usage.total_allocated;

        // Decision logic
        if self.config.enable_adaptive {
            self.make_adaptive_decision(usage_info)
        } else {
            self.make_static_decision(usage_info)
        }
    }

    /// Make adaptive optimization decision based on runtime behavior
    #[instrument(skip(self))]
    fn make_adaptive_decision(&self, usage_info: &GenericUsageInfo) -> Result<(), Error> {
        let instantiation_count = usage_info.instantiations.len();
        let total_calls = usage_info.total_calls;
        let memory_usage = usage_info.memory_usage.total_allocated;

        // JIT compilation for hot paths
        if total_calls >= self.config.jit_threshold && 
           usage_info.avg_execution_time > 10_000 && // > 10μs
           instantiation_count <= 5 {
            return Ok(OptimizationDecision {
                strategy: InstantiationStrategy::JustInTimeCompilation,
                reason: format!("Hot path with {} calls, {} instantiations", total_calls, instantiation_count),
                expected_improvement: 3.0, // 3x speedup expected
                memory_overhead: memory_usage / 10, // 10% overhead
            });
        }

        // Monomorphization for frequently used functions with few instantiations
        if instantiation_count <= self.config.monomorphization_threshold && 
           total_calls >= 100 &&
           memory_usage <= self.config.memory_usage_threshold {
            let code_size_increase = instantiation_count as f64 * 1.5; // Rough estimate
            if code_size_increase <= self.config.max_code_size_increase {
                return Ok(OptimizationDecision {
                    strategy: InstantiationStrategy::Monomorphization,
                    reason: format!("Few instantiations ({}), frequent usage ({})", 
                                   instantiation_count, total_calls),
                    expected_improvement: 1.5, // 50% speedup expected
                    memory_overhead: (memory_usage as f64 * code_size_increase) as usize,
                });
            }
        }

        // Dynamic dispatch for many instantiations
        if instantiation_count > self.config.monomorphization_threshold {
            return Ok(OptimizationDecision {
                strategy: InstantiationStrategy::DynamicDispatch,
                reason: format!("Many instantiations ({}), prefer runtime dispatch", instantiation_count),
                expected_improvement: 1.0, // No performance improvement
                memory_overhead: memory_usage / 20, // 5% overhead for vtables
            });
        }

        // Default to adaptive strategy
        Ok(OptimizationDecision {
            strategy: InstantiationStrategy::Adaptive,
            reason: "Insufficient data for specific optimization".to_string(),
            expected_improvement: 1.2, // Modest improvement expected
            memory_overhead: memory_usage / 10,
        })
    }

    /// Make static optimization decision based on configuration
    #[instrument(skip(self))]
    fn make_static_decision(&self, usage_info: &GenericUsageInfo) -> Result<(), Error> {
        let instantiation_count = usage_info.instantiations.len();

        if instantiation_count <= self.config.monomorphization_threshold {
            Ok(OptimizationDecision {
                strategy: InstantiationStrategy::Monomorphization,
                reason: "Static: few instantiations".to_string(),
                expected_improvement: 1.3,
                memory_overhead: usage_info.memory_usage.total_allocated * instantiation_count,
            })
        } else {
            Ok(OptimizationDecision {
                strategy: InstantiationStrategy::DynamicDispatch,
                reason: "Static: many instantiations".to_string(),
                expected_improvement: 1.0,
                memory_overhead: usage_info.memory_usage.total_allocated / 10,
            })
        }
    }

    /// Trigger JIT compilation for a function
    #[instrument(skip(self))]
    pub fn trigger_jit_compilation(&self, function_name: &str, type_args: &[Type]) -> Result<(), Error> {
        let mut jit_state = self.jit_state.write()
            .map_err(|_| CursedError::system_error("Failed to acquire write lock"))?;

        let key = format!("{}_{:?}", function_name, type_args);

        // Check if already compiling or compiled
        if jit_state.compiling.contains(&key) {
            debug!("JIT compilation already in progress for {}", key);
            return Ok(());
        }

        if jit_state.compiled.contains_key(&key) {
            debug!("Function {} already JIT compiled", key);
            return Ok(());
        }

        // Start compilation
        jit_state.compiling.insert(key.clone());
        info!("Starting JIT compilation for {}", key);

        // In a real implementation, this would trigger the actual JIT compilation
        // For now, we simulate it
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(100)); // Simulate compilation time
            // Mark compilation as complete (this would be done by the actual JIT compiler)
        });

        Ok(())
    }

    /// Complete JIT compilation (called by the JIT compiler)
    #[instrument(skip(self))]
    pub fn complete_jit_compilation(
        &self,
        function_name: &str,
        type_args: &[Type],
        speedup_ratio: f64,
        memory_usage: usize,
    ) -> Result<(), Error> {
        let mut jit_state = self.jit_state.write()
            .map_err(|_| CursedError::system_error("Failed to acquire write lock"))?;

        let key = format!("{}_{:?}", function_name, type_args);

        // Remove from compiling set
        jit_state.compiling.remove(&key);

        // Add to compiled functions
        jit_state.compiled.insert(key.clone(), JitCompiledFunction {
            compiled_at: std::time::SystemTime::now(),
            speedup_ratio,
            memory_usage,
        });

        info!("Completed JIT compilation for {} with {}x speedup", key, speedup_ratio);
        Ok(())
    }

    /// Mark JIT compilation as failed
    #[instrument(skip(self))]
    pub fn mark_jit_compilation_failed(&self, function_name: &str, type_args: &[Type], error: &str) -> Result<(), Error> {
        let mut jit_state = self.jit_state.write()
            .map_err(|_| CursedError::system_error("Failed to acquire write lock"))?;

        let key = format!("{}_{:?}", function_name, type_args);

        // Remove from compiling set
        jit_state.compiling.remove(&key);

        // Record failure
        jit_state.failures.insert(key.clone(), error.to_string());

        warn!("JIT compilation failed for {}: {}", key, error);
        Ok(())
    }

    /// Get optimization statistics
    #[instrument(skip(self))]
    pub fn get_statistics(&self) -> Result<(), Error> {
        let usage_stats = self.usage_stats.read()
            .map_err(|_| CursedError::system_error("Failed to acquire read lock"))?;
        let decisions_cache = self.decisions_cache.read()
            .map_err(|_| CursedError::system_error("Failed to acquire read lock"))?;
        let jit_state = self.jit_state.read()
            .map_err(|_| CursedError::system_error("Failed to acquire read lock"))?;

        let total_functions = usage_stats.len();
        let total_calls: usize = usage_stats.values().map(|info| info.total_calls).sum();
        let total_instantiations: usize = usage_stats.values()
            .map(|info| info.instantiations.len())
            .sum();

        let strategy_counts = decisions_cache.values().fold(HashMap::new(), |mut acc, decision| {
            *acc.entry(decision.strategy.clone()).or_insert(0) += 1;
            acc
        });

        Ok(OptimizationStatistics {
            total_functions,
            total_calls,
            total_instantiations,
            strategy_distribution: strategy_counts,
            jit_compiled_functions: jit_state.compiled.len(),
            jit_compilation_failures: jit_state.failures.len(),
            average_speedup: jit_state.compiled.values()
                .map(|f| f.speedup_ratio)
                .fold(0.0, |acc, x| acc + x) / jit_state.compiled.len().max(1) as f64,
        })
    }

    /// Clear all optimization data (useful for testing)
    #[instrument(skip(self))]
    pub fn clear_all_data(&self) -> Result<(), Error> {
        {
            let mut usage_stats = self.usage_stats.write()
                .map_err(|_| CursedError::system_error("Failed to acquire write lock"))?;
            usage_stats.clear();
        }
        {
            let mut decisions_cache = self.decisions_cache.write()
                .map_err(|_| CursedError::system_error("Failed to acquire write lock"))?;
            decisions_cache.clear();
        }
        {
            let mut jit_state = self.jit_state.write()
                .map_err(|_| CursedError::system_error("Failed to acquire write lock"))?;
            jit_state.compiling.clear();
            jit_state.compiled.clear();
            jit_state.failures.clear();
        }
        debug!("Cleared all optimization data");
        Ok(())
    }

    /// Optimize generic instantiations in the type environment
    #[instrument(skip(self, _type_environment))]
    pub fn optimize_instantiations(&self, _type_environment: &mut crate::type_system::TypeEnvironment) -> Result<(), Error> {
        // For now, this is a placeholder that analyzes existing usage data
        // and could trigger optimizations based on patterns
        
        let usage_stats = self.usage_stats.read()
            .map_err(|_| CursedError::system_error("Failed to acquire read lock"))?;
        
        // Count total functions that could benefit from optimization
        let mut optimization_candidates = 0;
        
        for (function_name, usage_info) in usage_stats.iter() {
            if usage_info.total_calls > 100 && usage_info.instantiations.len() < 5 {
                optimization_candidates += 1;
                debug!("Function {} is a candidate for monomorphization", function_name);
            } else if usage_info.instantiations.len() > 10 {
                debug!("Function {} should use dynamic dispatch", function_name);
            }
        }
        
        info!("Found {} optimization candidates", optimization_candidates);
        Ok(())
    }
}

/// Statistics about the optimization system
#[derive(Debug, Clone)]
pub struct OptimizationStatistics {
    pub total_functions: usize,
    pub total_calls: usize,
    pub total_instantiations: usize,
    pub strategy_distribution: HashMap<InstantiationStrategy, usize>,
    pub jit_compiled_functions: usize,
    pub jit_compilation_failures: usize,
    pub average_speedup: f64,
}

/// Trait for optimizing generic code
pub trait GenericCodeOptimizer {
    /// Optimize a generic function based on usage patterns
    fn optimize_function(&self, function_name: &str, usage_info: &GenericUsageInfo) -> Result<(), Error>;
    
    /// Generate monomorphized code
    fn generate_monomorphized_code(&self, function_name: &str, type_args: &[Type]) -> Result<(), Error>;
    
    /// Generate dynamic dispatch code
    fn generate_dynamic_dispatch_code(&self, function_name: &str) -> Result<(), Error>;
}

impl GenericCodeOptimizer for GenericOptimizer {
    #[instrument(skip(self))]
    fn optimize_function(&self, function_name: &str, usage_info: &GenericUsageInfo) -> Result<(), Error> {
        if self.config.enable_adaptive {
            self.make_adaptive_decision(usage_info)
        } else {
            self.make_static_decision(usage_info)
        }
    }

    #[instrument(skip(self))]
    fn generate_monomorphized_code(&self, function_name: &str, type_args: &[Type]) -> Result<(), Error> {
        // This would generate actual specialized code in a real implementation
        let specialized_name = format!("{}_{:?}", function_name, type_args);
        Ok(format!("// Monomorphized version: {}", specialized_name))
    }

    #[instrument(skip(self))]
    fn generate_dynamic_dispatch_code(&self, function_name: &str) -> Result<(), Error> {
        // This would generate vtable-based dispatch code in a real implementation
        Ok(format!("// Dynamic dispatch version: {}", function_name))
    }
}

/// Memory layout optimizer for generic types
#[derive(Debug)]
pub struct MemoryLayoutOptimizer {
    /// Cache of optimized layouts
    layout_cache: RwLock<HashMap<Vec<Type>, MemoryLayout>>,
}

/// Optimized memory layout information
#[derive(Debug, Clone)]
pub struct MemoryLayout {
    /// Total size in bytes
    pub size: usize,
    /// Alignment requirement
    pub alignment: usize,
    /// Field offsets
    pub field_offsets: Vec<usize>,
    /// Padding bytes
    pub padding: usize,
}

impl MemoryLayoutOptimizer {
    /// Create a new memory layout optimizer
    #[instrument]
    pub fn new() -> Self {
        Self {
            layout_cache: RwLock::new(HashMap::new()),
        }
    }

    /// Optimize memory layout for a generic type
    #[instrument(skip(self))]
    pub fn optimize_layout(&self, type_args: &[Type]) -> Result<(), Error> {
        // Check cache first
        {
            let cache = self.layout_cache.read()
                .map_err(|_| CursedError::system_error("Failed to acquire read lock"))?;
            if let Some(layout) = cache.get(type_args) {
                debug!("Found cached memory layout");
                return Ok(layout.clone());
            }
        }

        // Compute optimal layout
        let layout = self.compute_optimal_layout(type_args)?;

        // Cache the result
        {
            let mut cache = self.layout_cache.write()
                .map_err(|_| CursedError::system_error("Failed to acquire write lock"))?;
            cache.insert(type_args.to_vec(), layout.clone());
        }

        Ok(layout)
    }

    /// Compute the optimal memory layout
    #[instrument(skip(self))]
    fn compute_optimal_layout(&self, type_args: &[Type]) -> Result<(), Error> {
        let mut total_size = 0;
        let mut max_alignment = 1;
        let mut field_offsets = Vec::new();
        let mut current_offset = 0;

        for type_arg in type_args {
            let (size, alignment) = self.get_type_size_and_alignment(type_arg)?;
            
            // Align current offset
            let aligned_offset = (current_offset + alignment - 1) & !(alignment - 1);
            field_offsets.push(aligned_offset);
            
            current_offset = aligned_offset + size;
            max_alignment = max_alignment.max(alignment);
        }

        // Final alignment
        total_size = (current_offset + max_alignment - 1) & !(max_alignment - 1);
        let padding = total_size - current_offset;

        Ok(MemoryLayout {
            size: total_size,
            alignment: max_alignment,
            field_offsets,
            padding,
        })
    }

    /// Get size and alignment for a type
    #[instrument(skip(self))]
    fn get_type_size_and_alignment(&self, type_ref: &Type) -> Result<(), Error> {
        match type_ref {
            Type::Integer => Ok((8, 8)),
            Type::Float => Ok((8, 8)),
            Type::Boolean => Ok((1, 1)),
            Type::Character => Ok((4, 4)), // UTF-32
            Type::String => Ok((24, 8)), // Pointer + length + capacity
            Type::Array(_) => Ok((24, 8)), // Similar to string
            Type::Tuple(types) => {
                let mut total_size = 0;
                let mut max_alignment = 1;
                for t in types {
                    let (size, alignment) = self.get_type_size_and_alignment(t)?;
                    total_size += size;
                    max_alignment = max_alignment.max(alignment);
                }
                Ok((total_size, max_alignment))
            }
            Type::Generic(_) => Ok((8, 8)), // Pointer to generic data
            _ => Ok((8, 8)), // Default pointer size
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimizer_creation() {
        let optimizer = GenericOptimizer::new();
        let stats = optimizer.get_statistics().unwrap();
        assert_eq!(stats.total_functions, 0);
    }

    #[test]
    fn test_usage_recording() {
        let optimizer = GenericOptimizer::new();
        let type_args = vec![Type::Integer, Type::String];
        
        optimizer.record_usage("test_function", &type_args, 1000, 128).unwrap();
        
        let stats = optimizer.get_statistics().unwrap();
        assert_eq!(stats.total_functions, 1);
        assert_eq!(stats.total_calls, 1);
    }

    #[test]
    fn test_optimization_decision() {
        let optimizer = GenericOptimizer::new();
        let type_args = vec![Type::Integer];
        
        // Record enough usage to trigger optimization
        for _ in 0..200 {
            optimizer.record_usage("hot_function", &type_args, 500, 64).unwrap();
        }
        
        let decision = optimizer.make_optimization_decision("hot_function").unwrap();
        assert!(matches!(decision.strategy, InstantiationStrategy::Monomorphization));
    }

    #[test]
    fn test_jit_compilation_workflow() {
        let optimizer = GenericOptimizer::new();
        let type_args = vec![Type::Integer];
        
        optimizer.trigger_jit_compilation("jit_function", &type_args).unwrap();
        optimizer.complete_jit_compilation("jit_function", &type_args, 2.5, 1024).unwrap();
        
        let stats = optimizer.get_statistics().unwrap();
        assert_eq!(stats.jit_compiled_functions, 1);
        assert!(stats.average_speedup > 2.0);
    }

    #[test]
    fn test_memory_layout_optimization() {
        let optimizer = MemoryLayoutOptimizer::new();
        let type_args = vec![Type::Integer, Type::Boolean, Type::Float];
        
        let layout = optimizer.optimize_layout(&type_args).unwrap();
        assert!(layout.size > 0);
        assert!(layout.alignment > 0);
        assert_eq!(layout.field_offsets.len(), 3);
    }

    #[test]
    fn test_adaptive_vs_static_decisions() {
        let config_adaptive = OptimizationConfig {
            enable_adaptive: true,
            ..OptimizationConfig::default()
        };
        let config_static = OptimizationConfig {
            enable_adaptive: false,
            ..OptimizationConfig::default()
        };

        let optimizer_adaptive = GenericOptimizer::with_config(config_adaptive);
        let optimizer_static = GenericOptimizer::with_config(config_static);

        let type_args = vec![Type::Integer];
        
        // Record usage for both optimizers
        for _ in 0..100 {
            optimizer_adaptive.record_usage("test_func", &type_args, 1000, 128).unwrap();
            optimizer_static.record_usage("test_func", &type_args, 1000, 128).unwrap();
        }

        let decision_adaptive = optimizer_adaptive.make_optimization_decision("test_func").unwrap();
        let decision_static = optimizer_static.make_optimization_decision("test_func").unwrap();

        // Both should choose monomorphization for this pattern, but reasons might differ
        assert!(matches!(decision_adaptive.strategy, InstantiationStrategy::Monomorphization));
        assert!(matches!(decision_static.strategy, InstantiationStrategy::Monomorphization));
    }
}
