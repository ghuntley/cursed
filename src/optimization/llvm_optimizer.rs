//! LLVM optimization pass management and integration

use crate::error::{Result, CursedError};
use crate::optimization::{OptimizationConfig};
use crate::optimization::config::OptimizationLevel;
use crate::optimization::pass_manager::LlvmPassManager;
use crate::optimization::metrics::CompilationUnit;
use std::collections::HashMap;
use tracing::{info, debug, warn, error, instrument};

/// LLVM optimizer with pass management and optimization coordination
#[derive(Debug)]
pub struct LlvmOptimizer {
    config: OptimizationConfig,
    pass_manager: LlvmPassManager,
    optimization_cache: HashMap<String, OptimizationResult>,
    statistics: OptimizerStatistics,
}

/// Result of an optimization operation
#[derive(Debug, Clone)]
pub struct OptimizationResult {
    pub unit_name: String,
    pub optimization_level: OptimizationLevel,
    pub before_size_bytes: usize,
    pub after_size_bytes: usize,
    pub optimization_time_ms: u64,
    pub passes_applied: Vec<String>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

/// Statistics tracking for the optimizer
#[derive(Debug, Default)]
pub struct OptimizerStatistics {
    pub total_units_optimized: usize,
    pub total_optimization_time_ms: u64,
    pub total_size_reduction_bytes: i64,
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub optimization_failures: usize,
    pub average_optimization_time_ms: f64,
}

impl LlvmOptimizer {
    /// Create a new LLVM optimizer with the given configuration
    #[instrument]
    pub fn new(config: OptimizationConfig) -> Result<Self> {
        info!("Creating LLVM optimizer with level {:?}", config.optimization_level);
        
        // Validate configuration
        config.validate().map_err(|e| {
            CursedError::optimization_error(&format!("Invalid optimization config: {}", e))
        })?;

        let pass_manager = LlvmPassManager::new(&config)?;

        Ok(Self {
            config,
            pass_manager,
            optimization_cache: HashMap::new(),
            statistics: OptimizerStatistics::default(),
        })
    }

    /// Optimize a compilation unit
    #[instrument(skip(self, unit))]
    pub fn optimize_unit(&mut self, unit: &mut CompilationUnit) -> Result<OptimizationResult> {
        let start_time = std::time::Instant::now();
        
        debug!("Starting optimization for unit: {}", unit.name);

        // Check cache first if caching is enabled
        if self.config.enable_caching {
            if let Some(cached_result) = self.check_cache(&unit.name) {
                debug!("Using cached optimization result for {}", unit.name);
                self.statistics.cache_hits += 1;
                return Ok(cached_result);
            }
            self.statistics.cache_misses += 1;
        }

        let before_size = unit.estimated_size_bytes;
        let mut warnings = Vec::new();
        let mut errors = Vec::new();

        // Apply optimization passes
        let passes_applied = match self.apply_optimization_passes(unit) {
            Ok(passes) => passes,
            Err(e) => {
                error!("Optimization failed for unit {}: {}", unit.name, e);
                self.statistics.optimization_failures += 1;
                errors.push(e.to_string());
                Vec::new()
            }
        };

        // Apply target-specific optimizations if enabled
        if self.config.enable_target_specific {
            if let Err(e) = self.apply_target_optimizations(unit) {
                warn!("Target-specific optimization failed: {}", e);
                warnings.push(format!("Target optimization warning: {}", e));
            }
        }

        let after_size = unit.estimated_size_bytes;
        let optimization_time = start_time.elapsed();

        let result = OptimizationResult {
            unit_name: unit.name.clone(),
            optimization_level: self.config.optimization_level.clone(),
            before_size_bytes: before_size,
            after_size_bytes: after_size,
            optimization_time_ms: optimization_time.as_millis() as u64,
            passes_applied,
            warnings,
            errors,
        };

        // Update statistics
        self.update_statistics(&result);

        // Cache the result if caching is enabled
        if self.config.enable_caching {
            self.cache_result(result.clone());
        }

        info!(
            "Optimization completed for {}: {} -> {} bytes in {}ms",
            unit.name,
            before_size,
            after_size,
            optimization_time.as_millis()
        );

        Ok(result)
    }

    /// Apply optimization passes based on configuration
    #[instrument(skip(self, unit))]
    fn apply_optimization_passes(&mut self, unit: &mut CompilationUnit) -> Result<Vec<String>> {
        let mut applied_passes = Vec::new();

        // Apply standard optimization passes based on level
        match self.config.optimization_level {
            OptimizationLevel::O0 => {
                // No optimization passes for -O0
                debug!("Skipping optimization passes for O0 level");
            }
            OptimizationLevel::O1 => {
                applied_passes.extend(self.apply_basic_passes(unit)?);
            }
            OptimizationLevel::O2 => {
                applied_passes.extend(self.apply_basic_passes(unit)?);
                applied_passes.extend(self.apply_standard_passes(unit)?);
            }
            OptimizationLevel::O3 => {
                applied_passes.extend(self.apply_basic_passes(unit)?);
                applied_passes.extend(self.apply_standard_passes(unit)?);
                applied_passes.extend(self.apply_aggressive_passes(unit)?);
            }
            OptimizationLevel::Os => {
                applied_passes.extend(self.apply_size_passes(unit)?);
            }
            OptimizationLevel::Fast => {
                applied_passes.extend(self.apply_fast_passes(unit)?);
            }
        }

        // Apply custom passes if specified
        for custom_pass in &self.config.custom_passes {
            if let Err(e) = self.pass_manager.apply_custom_pass(unit, custom_pass) {
                warn!("Failed to apply custom pass {}: {}", custom_pass, e);
            } else {
                applied_passes.push(custom_pass.clone());
            }
        }

        Ok(applied_passes)
    }

    /// Apply basic optimization passes (O1 level)
    fn apply_basic_passes(&mut self, unit: &mut CompilationUnit) -> Result<Vec<String>> {
        let mut passes = Vec::new();

        if self.config.enable_dce {
            self.pass_manager.apply_dead_code_elimination(unit)?;
            passes.push("dead-code-elimination".to_string());
        }

        // Basic constant folding and propagation
        self.pass_manager.apply_constant_folding(unit)?;
        passes.push("constant-folding".to_string());

        // Basic block merging
        self.pass_manager.apply_block_merging(unit)?;
        passes.push("block-merging".to_string());

        Ok(passes)
    }

    /// Apply standard optimization passes (O2 level)
    fn apply_standard_passes(&mut self, unit: &mut CompilationUnit) -> Result<Vec<String>> {
        let mut passes = Vec::new();

        if self.config.enable_inlining {
            self.pass_manager.apply_function_inlining(unit)?;
            passes.push("function-inlining".to_string());
        }

        if self.config.enable_loop_optimizations {
            self.pass_manager.apply_loop_optimizations(unit)?;
            passes.push("loop-optimizations".to_string());
        }

        if self.config.enable_vectorization {
            self.pass_manager.apply_vectorization(unit)?;
            passes.push("vectorization".to_string());
        }

        // Global value numbering
        self.pass_manager.apply_global_value_numbering(unit)?;
        passes.push("global-value-numbering".to_string());

        // Instruction combining
        self.pass_manager.apply_instruction_combining(unit)?;
        passes.push("instruction-combining".to_string());

        Ok(passes)
    }

    /// Apply aggressive optimization passes (O3 level)
    fn apply_aggressive_passes(&mut self, unit: &mut CompilationUnit) -> Result<Vec<String>> {
        let mut passes = Vec::new();

        // Aggressive inlining
        self.pass_manager.apply_aggressive_inlining(unit)?;
        passes.push("aggressive-inlining".to_string());

        // Loop unrolling
        self.pass_manager.apply_loop_unrolling(unit)?;
        passes.push("loop-unrolling".to_string());

        // Tail call optimization
        self.pass_manager.apply_tail_call_optimization(unit)?;
        passes.push("tail-call-optimization".to_string());

        // Interprocedural optimizations
        self.pass_manager.apply_interprocedural_optimizations(unit)?;
        passes.push("interprocedural-optimizations".to_string());

        Ok(passes)
    }

    /// Apply size-focused optimization passes
    fn apply_size_passes(&mut self, unit: &mut CompilationUnit) -> Result<Vec<String>> {
        let mut passes = Vec::new();

        // Dead code elimination is crucial for size
        if self.config.enable_dce {
            self.pass_manager.apply_dead_code_elimination(unit)?;
            passes.push("dead-code-elimination".to_string());
        }

        // Conservative inlining for size
        self.pass_manager.apply_size_optimized_inlining(unit)?;
        passes.push("size-optimized-inlining".to_string());

        // Code deduplication
        self.pass_manager.apply_code_deduplication(unit)?;
        passes.push("code-deduplication".to_string());

        Ok(passes)
    }

    /// Apply fast optimization passes (Ofast level)
    fn apply_fast_passes(&mut self, unit: &mut CompilationUnit) -> Result<Vec<String>> {
        let mut passes = self.apply_standard_passes(unit)?;

        // Fast math optimizations
        self.pass_manager.apply_fast_math_optimizations(unit)?;
        passes.push("fast-math-optimizations".to_string());

        // Unsafe optimizations that may break strict compliance
        self.pass_manager.apply_unsafe_optimizations(unit)?;
        passes.push("unsafe-optimizations".to_string());

        Ok(passes)
    }

    /// Apply target-specific optimizations
    fn apply_target_optimizations(&mut self, unit: &mut CompilationUnit) -> Result<()> {
        // Apply CPU-specific optimizations
        self.pass_manager.apply_cpu_specific_optimizations(unit)?;
        
        // Apply architecture-specific optimizations
        self.pass_manager.apply_architecture_optimizations(unit)?;
        
        Ok(())
    }

    /// Check optimization cache for existing result
    fn check_cache(&self, unit_name: &str) -> Option<OptimizationResult> {
        self.optimization_cache.get(unit_name).cloned()
    }

    /// Cache an optimization result
    fn cache_result(&mut self, result: OptimizationResult) {
        self.optimization_cache.insert(result.unit_name.clone(), result);
    }

    /// Update optimizer statistics
    fn update_statistics(&mut self, result: &OptimizationResult) {
        self.statistics.total_units_optimized += 1;
        self.statistics.total_optimization_time_ms += result.optimization_time_ms;
        self.statistics.total_size_reduction_bytes += 
            result.before_size_bytes as i64 - result.after_size_bytes as i64;

        // Update average optimization time
        self.statistics.average_optimization_time_ms = 
            self.statistics.total_optimization_time_ms as f64 / 
            self.statistics.total_units_optimized as f64;
    }

    /// Get optimizer statistics
    pub fn get_statistics(&self) -> &OptimizerStatistics {
        &self.statistics
    }

    /// Clear optimization cache
    pub fn clear_cache(&mut self) {
        info!("Clearing optimization cache");
        self.optimization_cache.clear();
    }

    /// Update optimizer configuration
    #[instrument(skip(self))]
    pub fn update_config(&mut self, new_config: OptimizationConfig) -> Result<()> {
        info!("Updating optimizer configuration");
        
        new_config.validate().map_err(|e| {
            CursedError::optimization_error(&format!("Invalid optimization config: {}", e))
        })?;

        self.config = new_config;
        self.pass_manager.update_config(&self.config)?;
        
        // Clear cache when configuration changes
        self.clear_cache();
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::optimization::metrics::CompilationUnit;

    #[test]
    fn test_llvm_optimizer_creation() {
        let config = OptimizationConfig::default();
        let optimizer = LlvmOptimizer::new(config);
        assert!(optimizer.is_ok());
    }

    #[test]
    fn test_optimization_levels() {
        let levels = vec![
            OptimizationLevel::O0,
            OptimizationLevel::O1,
            OptimizationLevel::O2,
            OptimizationLevel::O3,
        ];

        for level in levels {
            let mut config = OptimizationConfig::default();
            config.optimization_level = level.clone();
            
            let optimizer = LlvmOptimizer::new(config);
            assert!(optimizer.is_ok(), "Failed to create optimizer for level {:?}", level);
        }
    }

    #[test]
    fn test_optimizer_statistics_initialization() {
        let config = OptimizationConfig::default();
        let optimizer = LlvmOptimizer::new(config).unwrap();
        
        let stats = optimizer.get_statistics();
        assert_eq!(stats.total_units_optimized, 0);
        assert_eq!(stats.total_optimization_time_ms, 0);
        assert_eq!(stats.cache_hits, 0);
    }
}
