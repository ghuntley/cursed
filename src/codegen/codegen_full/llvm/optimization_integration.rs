/// LLVM Optimization Integration
/// 
/// This module provides integration between the CURSED optimization system
/// and LLVM code generation.

use crate::error::{CursedError, Result};

use crate::optimization::{
    BenchmarkConfig, PerformanceMetrics
// };

use crate::common_types::optimization_level::OptimizationLevel; // Explicit import to resolve conflict
use crate::ast::*;

use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use tracing::{debug, info, instrument, warn};

/// LLVM optimization integration
#[derive(Debug)]
pub struct LlvmOptimizationIntegration {
    /// Main optimization manager
    /// Adaptive optimizer for runtime feedback
    /// Incremental compiler for fast development builds
    /// Benchmark suite for performance regression testing
    /// Performance profiler
    /// Current optimization state
/// Current state of optimization
#[derive(Debug, Clone)]
pub struct OptimizationState {
    /// Current optimization level
    /// Functions being optimized
    /// Optimization statistics
    /// Hot path information
/// Hot path information
#[derive(Debug, Clone)]
pub struct HotPath {
    /// Function name
    /// Execution count
    /// Average execution time
    /// Optimization level applied
/// Optimization statistics
#[derive(Debug, Clone, Default)]
pub struct OptimizationStats {
    /// Total optimizations applied
    /// Successful optimizations
    /// Total compilation time
    /// Time saved by optimizations
    /// Functions optimized
    /// Memory usage optimization savings
impl LlvmOptimizationIntegration {
    /// Create new optimization integration
    pub fn new(config: OptimizationConfig) -> Result<Self> {
        let optimization_manager = OptimizationManager::new(config.clone())?;
        
        let adaptive_optimizer = if config.enable_adaptive_optimization {
            Some(AdaptiveOptimizer::new(&config)?)
        } else {
            None
        
        let incremental_compiler = if config.enable_incremental_compilation {
            Some(crate::optimization::incremental::IncrementalCompiler::new(&config)?)
        } else {
            None
        
        let benchmark_suite = if config.enable_profiling {
            let benchmark_config = BenchmarkConfig::default();
            let benchmarks = crate::optimization::benchmarks::BenchmarkRunner::new(config.clone());
            Some(benchmarks.create_suite(benchmark_config)?)
        } else {
            None
        
        let profiler = if config.enable_profiling {
            Some(PerformanceProfiler::new(&config)?)
        } else {
            None
        
        Ok(Self {
            current_state: OptimizationState {
        })
    /// Optimize LLVM module
    #[instrument(skip(self, module))]
    pub fn optimize_module(&mut self, module: &str) -> Result<String> {
        let start_time = Instant::now();
        
        info!("Starting LLVM module optimization");
        
        // Apply LLVM optimizations
        let optimized_module = if let Some(llvm_optimizer) = self.optimization_manager.llvm_optimizer() {
            llvm_optimizer.optimize_module(module)?
        } else {
            module.to_string()
        
        // Update statistics
        let optimization_time = start_time.elapsed();
        self.current_state.stats.total_optimizations += 1;
        self.current_state.stats.total_compilation_time += optimization_time;
        
        info!("LLVM module optimization completed in {:?}", optimization_time);
        Ok(optimized_module)
    /// Optimize function with adaptive feedback
    #[instrument(skip(self))]
    pub fn optimize_function(&mut self, function_name: &str, function_ir: &str) -> Result<String> {
        let start_time = Instant::now();
        
        debug!("Optimizing function: {}", function_name);
        
        // Check if function is hot
        let is_hot = self.is_hot_function(function_name);
        
        // Apply appropriate optimization level
        let optimization_level = if is_hot {
            self.current_state.level.increase()
        } else {
            self.current_state.level
        
        // Optimize function
        let optimized_ir = self.apply_function_optimizations(function_name, function_ir, optimization_level)?;
        
        // Record optimization
        let optimization_time = start_time.elapsed();
        self.record_optimization_result(function_name, optimization_time, true, 0.1)?;
        
        debug!("Function {} optimized in {:?}", function_name, optimization_time);
        Ok(optimized_ir)
    /// Apply function-level optimizations
    fn apply_function_optimizations(
    ) -> Result<String> {
        if let Some(llvm_optimizer) = self.optimization_manager.llvm_optimizer() {
            // Apply optimizations based on level
            match level {
                OptimizationLevel::O1 => {
                    llvm_optimizer.apply_basic_optimizations(function_ir)
                OptimizationLevel::O2 => {
                    llvm_optimizer.apply_standard_optimizations(function_ir)
                OptimizationLevel::O3 => {
                    llvm_optimizer.apply_aggressive_optimizations(function_ir)
            }
        } else {
            Ok(function_ir.to_string())
        }
    }
    
    /// Check if function is hot (frequently executed)
    fn is_hot_function(&self, function_name: &str) -> bool {
        self.current_state.hot_paths.iter()
            .any(|hp| hp.function_name == function_name && hp.execution_count > 1000)
    /// Record optimization result for adaptive learning
    #[instrument(skip(self))]
    pub fn record_optimization_result(
    ) -> Result<()> {
        // Update statistics
        self.current_state.stats.total_optimizations += 1;
        if success {
            self.current_state.stats.successful_optimizations += 1;
            self.current_state.stats.time_saved += Duration::from_millis(
                (optimization_time.as_millis() as f64 * performance_improvement) as u64
            );
        // Record with adaptive optimizer
        if let Some(adaptive) = &self.adaptive_optimizer {
            // Calculate actual memory usage based on optimization context
            let estimated_memory_usage = self.calculate_optimization_memory_usage(function_name, optimization_time);
            
            let feedback = OptimizationFeedback {
            
            adaptive.record_execution(feedback)?;
            
            // Apply optimization feedback
            let strategy = if success {
                OptimizationStrategy::IncreaseLevel
            } else {
                OptimizationStrategy::DecreaseLevel
            
            adaptive.apply_optimization_result(
            )?;
               function_name, success, performance_improvement * 100.0);
        
        Ok(())
    /// Get optimization recommendations
    pub fn get_recommendations(&self) -> Result<Vec<OptimizationRecommendation>> {
        if let Some(adaptive) = &self.adaptive_optimizer {
            adaptive.get_recommendations()
        } else {
            Ok(Vec::new())
        }
    }
    
    /// Apply incremental compilation if available
    #[instrument(skip(self, compile_fn))]
    pub fn incremental_compile<F>(
    ) -> Result<IncrementalCompilationResult>
    where
    {
        if let Some(incremental) = &mut self.incremental_compiler {
            incremental.compile_directory(source_dir, compile_fn)
        } else {
            Err(CursedError::OptimizationError("Incremental compilation not enabled".to_string()))
        }
    }
    
    /// Run performance benchmarks
    pub fn run_benchmarks(&self) -> Result<BenchmarkSuiteResults> {
        if let Some(suite) = &self.benchmark_suite {
            suite.run_all()
        } else {
            Err(CursedError::OptimizationError("Benchmarking not enabled".to_string()))
        }
    }
    
    /// Start profiling session
    #[instrument(skip(self))]
    pub fn start_profiling(&mut self, session_name: &str) -> Result<()> {
        if let Some(profiler) = &mut self.profiler {
            profiler.start_session(session_name.to_string())?;
            info!("Started profiling session: {}", session_name);
        }
        Ok(())
    /// End profiling session
    #[instrument(skip(self))]
    pub fn end_profiling(&mut self) -> Result<PerformanceMetrics> {
        if let Some(profiler) = &mut self.profiler {
            let metrics = profiler.end_session()?;
            info!("Ended profiling session");
            Ok(metrics)
        } else {
            Err(CursedError::OptimizationError("Profiling not enabled".to_string()))
        }
    }
    
    /// Update hot path information
    pub fn update_hot_path(&mut self, function_name: String, execution_time: Duration) {
        if let Some(hot_path) = self.current_state.hot_paths
            .iter_mut()
            .find(|hp| hp.function_name == function_name) {
            
            hot_path.execution_count += 1;
            hot_path.average_time = Duration::from_nanos(
                (hot_path.average_time.as_nanos() as u64 + execution_time.as_nanos() as u64) / 2
            );
        } else {
            self.current_state.hot_paths.push(HotPath {
            });
        // Keep only top hot paths
        self.current_state.hot_paths.sort_by_key(|hp| std::cmp::Reverse(hp.execution_count));
        self.current_state.hot_paths.truncate(100);
    /// Get current optimization state
    pub fn get_state(&self) -> &OptimizationState {
        &self.current_state
    /// Get optimization statistics
    pub fn get_stats(&self) -> &OptimizationStats {
        &self.current_state.stats
    /// Set optimization level
    pub fn set_optimization_level(&mut self, level: OptimizationLevel) -> Result<()> {
        self.current_state.level = level;
        self.optimization_manager.set_optimization_level(level)?;
        info!("Set optimization level to: {:?}", level);
        Ok(())
    /// Perform adaptive optimization if needed
    pub fn adapt_if_needed(&self) -> Result<Option<AdaptationResult>> {
        if let Some(adaptive) = &self.adaptive_optimizer {
            adaptive.adapt_if_needed()
        } else {
            Ok(None)
        }
    }
    
    /// Calculate estimated memory usage for optimization process
    fn calculate_optimization_memory_usage(&self, function_name: &str, optimization_time: Duration) -> u64 {
        // Base memory usage for basic optimization structures
        let base_memory = 512; // bytes for basic optimization context
        
        // Scale by optimization time (more time usually means more memory for analysis)
        let time_factor = (optimization_time.as_millis() as f64 / 100.0).min(10.0); // Cap at 10x
        
        // Additional memory for hot functions (they need more detailed tracking)
        let hot_function_memory = if self.is_hot_function(function_name) {
            2048 // Additional memory for hot path analysis
        } else {
            256  // Basic memory for regular functions
        
        // Function name length can indicate complexity (longer names often mean more complex functions)
        let complexity_factor = (function_name.len() as f64 / 20.0).min(2.0); // Cap at 2x
        
        // Calculate total estimated memory usage
        let total_memory = base_memory as f64 
            + (hot_function_memory as f64 * time_factor) 
            + (128.0 * complexity_factor);
        
        total_memory as u64
    /// Print comprehensive optimization summary
    pub fn print_summary(&self) {
        println!("🚀 LLVM Optimization Integration Summary");
        println!("{}", "=".repeat(50));
        
        let stats = &self.current_state.stats;
        println!("📊 Optimization Statistics:");
        println!("  Total optimizations: {}", stats.total_optimizations);
        println!("  Successful optimizations: {}", stats.successful_optimizations);
                 if stats.total_optimizations > 0 {
                     stats.successful_optimizations as f64 / stats.total_optimizations as f64 * 100.0
                 } else {
                     0.0
                 });
        println!("  Total compilation time: {:?}", stats.total_compilation_time);
        println!("  Time saved: {:?}", stats.time_saved);
        println!("  Functions optimized: {}", stats.functions_optimized);
        
        println!("\n🔥 Hot Paths:");
        for (i, hot_path) in self.current_state.hot_paths.iter().take(10).enumerate() {
                     i + 1, hot_path.function_name, hot_path.execution_count, hot_path.average_time);
        println!("\n🎯 Current Optimization Level: {:?}", self.current_state.level);
        
        if let Some(adaptive) = &self.adaptive_optimizer {
            let summary = adaptive.get_summary();
            println!("\n🧠 Adaptive Optimization:");
            println!("  Total functions tracked: {}", summary.total_functions);
            println!("  Hot functions: {}", summary.hot_functions);
            println!("  Optimized functions: {}", summary.optimized_functions);
        // Print component summaries
        self.optimization_manager.print_comprehensive_summary();
        
        println!("{}", "=".repeat(50));
    }
}

impl OptimizationLevel {
    /// Convert from u32
    pub fn from_u32(level: u32) -> Self {
        match level {
        }
    }
    
    /// Increase optimization level
    pub fn increase(self) -> Self {
        match self {
        }
    }
    
    /// Decrease optimization level
    pub fn decrease(self) -> Self {
        match self {
        }
    }
