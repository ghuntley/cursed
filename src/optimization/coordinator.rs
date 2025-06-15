//! Optimization coordinator that integrates all optimization systems

use crate::error::{Result, CursedError};
use crate::optimization::{
    llvm_optimizer::{LlvmOptimizer, OptimizationResult},
    cache_manager::{CacheManager, CacheConfig},
    incremental::{IncrementalCompiler, IncrementalConfig, IncrementalBuildPlan},
    parallel_compilation::{ParallelCompiler, ParallelCompilationConfig, ParallelCompilationResult},
    profiler::{EnhancedBuildProfiler, ProfilerConfig, ProfileSession, ProfileReport},
    metrics::{MetricsCollector, CompilationUnit, ResourceStatistics},
    benchmarking::{BenchmarkingEngine, BenchmarkConfig, BenchmarkResults},
    analysis::{PerformanceAnalyzer, AnalysisConfig, PerformanceReport},
    dependency_analyzer::{DependencyAnalyzer, DependencyGraph},
    PerformanceConfig,
};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::{info, debug, warn, error, instrument};
use serde::{Deserialize, Serialize};

/// Comprehensive optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationCoordinatorConfig {
    pub enable_llvm_optimization: bool,
    pub enable_caching: bool,
    pub enable_incremental: bool,
    pub enable_parallel: bool,
    pub enable_profiling: bool,
    pub enable_benchmarking: bool,
    pub enable_analysis: bool,
    
    pub llvm_config: LlvmOptimizerConfig,
    pub cache_config: CacheConfig,
    pub incremental_config: IncrementalConfig,
    pub parallel_config: ParallelCompilationConfig,
    pub profiler_config: ProfilerConfig,
    pub analysis_config: AnalysisConfig,
    
    pub optimization_timeout: Option<Duration>,
    pub max_cache_size_mb: u64,
    pub benchmark_frequency: BenchmarkFrequency,
}

/// LLVM optimizer configuration subset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlvmOptimizerConfig {
    pub optimization_level: String,
    pub enable_vectorization: bool,
    pub enable_loop_unrolling: bool,
    pub enable_function_inlining: bool,
    pub target_cpu: Option<String>,
    pub target_features: Vec<String>,
}

/// Frequency for automatic benchmarking
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BenchmarkFrequency {
    Never,
    OnceDaily,
    Weekly,
    AfterMajorChanges,
    Always,
}

impl Default for OptimizationCoordinatorConfig {
    fn default() -> Self {
        Self {
            enable_llvm_optimization: true,
            enable_caching: true,
            enable_incremental: true,
            enable_parallel: true,
            enable_profiling: true,
            enable_benchmarking: false,
            enable_analysis: true,
            
            llvm_config: LlvmOptimizerConfig::default(),
            cache_config: CacheConfig::default(),
            incremental_config: IncrementalConfig::default(),
            parallel_config: ParallelCompilationConfig::default(),
            profiler_config: ProfilerConfig::default(),
            analysis_config: AnalysisConfig::default(),
            
            optimization_timeout: Some(Duration::from_secs(600)), // 10 minutes
            max_cache_size_mb: 2048, // 2GB
            benchmark_frequency: BenchmarkFrequency::Weekly,
        }
    }
}

impl Default for LlvmOptimizerConfig {
    fn default() -> Self {
        Self {
            optimization_level: "O2".to_string(),
            enable_vectorization: true,
            enable_loop_unrolling: true,
            enable_function_inlining: true,
            target_cpu: None,
            target_features: Vec::new(),
        }
    }
}

/// Comprehensive optimization results
#[derive(Debug, Clone)]
pub struct OptimizationCoordinatorResult {
    pub compilation_successful: bool,
    pub total_time: Duration,
    pub units_compiled: usize,
    pub units_from_cache: usize,
    pub units_from_incremental: usize,
    pub optimization_savings: Duration,
    pub cache_hit_rate: f64,
    pub parallel_efficiency: f64,
    pub profile_report: Option<ProfileReport>,
    pub performance_analysis: Option<PerformanceReport>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

/// Central optimization coordinator
pub struct OptimizationCoordinator {
    config: OptimizationCoordinatorConfig,
    
    // Component optimizers
    llvm_optimizer: Option<LlvmOptimizer>,
    cache_manager: Option<CacheManager>,
    incremental_compiler: Option<IncrementalCompiler>,
    parallel_compiler: Option<ParallelCompiler>,
    profiler: Option<EnhancedBuildProfiler>,
    metrics_collector: Option<MetricsCollector>,
    benchmarking_engine: Option<BenchmarkingEngine>,
    performance_analyzer: Option<PerformanceAnalyzer>,
    dependency_analyzer: DependencyAnalyzer,
    
    // State tracking
    last_benchmark_time: Option<Instant>,
    compilation_history: Vec<OptimizationCoordinatorResult>,
}

impl OptimizationCoordinator {
    /// Create a new optimization coordinator
    #[instrument]
    pub fn new(config: OptimizationCoordinatorConfig) -> Result<Self> {
        info!("Creating optimization coordinator");
        
        // Initialize dependency analyzer (always needed)
        let dependency_analyzer = DependencyAnalyzer::new()?;
        
        // Initialize components based on configuration
        let llvm_optimizer = if config.enable_llvm_optimization {
            // Note: This would need the actual LLVM optimizer creation
            None // Placeholder - would need LLVM context
        } else {
            None
        };
        
        let cache_manager = if config.enable_caching {
            Some(CacheManager::new_with_config(config.cache_config.clone())?)
        } else {
            None
        };
        
        let incremental_compiler = if config.enable_incremental {
            Some(IncrementalCompiler::new(config.incremental_config.clone())?)
        } else {
            None
        };
        
        let parallel_compiler = if config.enable_parallel {
            Some(ParallelCompiler::new(config.parallel_config.clone())?)
        } else {
            None
        };
        
        let profiler = if config.enable_profiling {
            Some(EnhancedBuildProfiler::new(config.profiler_config.clone())?)
        } else {
            None
        };
        
        let metrics_collector = if config.enable_analysis || config.enable_profiling {
            // Create performance config based on our settings
            let perf_config = crate::optimization::PerformanceConfig {
                enable_benchmarking: config.enable_benchmarking,
                enable_profiling: config.enable_profiling,
                resource_monitoring_level: crate::optimization::metrics::ResourceMonitoringLevel::Detailed,
                monitoring_interval_ms: 100,
                ..Default::default()
            };
            Some(MetricsCollector::new(perf_config)?)
        } else {
            None
        };
        
        let benchmarking_engine = if config.enable_benchmarking {
            let perf_config = crate::optimization::PerformanceConfig {
                enable_benchmarking: true,
                ..Default::default()
            };
            Some(BenchmarkingEngine::new(perf_config)?)
        } else {
            None
        };
        
        let performance_analyzer = if config.enable_analysis {
            Some(PerformanceAnalyzer::new(config.analysis_config.clone())?)
        } else {
            None
        };
        
        Ok(Self {
            config,
            llvm_optimizer,
            cache_manager,
            incremental_compiler,
            parallel_compiler,
            profiler,
            metrics_collector,
            benchmarking_engine,
            performance_analyzer,
            dependency_analyzer,
            last_benchmark_time: None,
            compilation_history: Vec::new(),
        })
    }
    
    /// Perform comprehensive optimization on compilation units
    #[instrument(skip(self, units))]
    pub fn optimize_compilation(&mut self, units: &mut [CompilationUnit]) -> Result<OptimizationCoordinatorResult> {
        let start_time = Instant::now();
        info!("Starting comprehensive optimization for {} units", units.len());
        
        let mut warnings = Vec::new();
        let mut errors = Vec::new();
        
        // Start profiling session if enabled
        let profile_session = if let Some(ref mut profiler) = self.profiler {
            let session_name = format!("compilation_{}", chrono::Utc::now().format("%Y%m%d_%H%M%S"));
            Some(profiler.start_build_session(session_name)?)
        } else {
            None
        };
        
        // Start metrics collection if enabled
        if let Some(ref metrics) = self.metrics_collector {
            metrics.start_monitoring()?;
        }
        
        // Phase 1: Dependency Analysis
        debug!("Phase 1: Analyzing dependencies");
        let dependency_graph = self.dependency_analyzer.analyze_dependencies(units)?;
        
        // Phase 2: Incremental Compilation Analysis
        let incremental_plan = if let Some(ref mut incremental) = self.incremental_compiler {
            debug!("Phase 2: Creating incremental build plan");
            Some(incremental.analyze_changes(units)?)
        } else {
            None
        };
        
        // Phase 3: Compilation Strategy Decision
        let compilation_strategy = self.decide_compilation_strategy(units, &dependency_graph, incremental_plan.as_ref())?;
        
        // Phase 4: Execute Compilation
        let compilation_result = self.execute_compilation_strategy(
            units, 
            &dependency_graph, 
            incremental_plan, 
            compilation_strategy,
            profile_session.as_ref()
        )?;
        
        // Phase 5: Post-compilation Analysis
        let performance_analysis = if let Some(ref mut analyzer) = self.performance_analyzer {
            debug!("Phase 5: Performing performance analysis");
            
            // Add metrics to analyzer
            if let Some(ref metrics) = self.metrics_collector {
                let system_stats = metrics.get_system_statistics();
                analyzer.add_system_metrics(system_stats);
                
                if let Ok(resource_stats) = metrics.get_resource_statistics() {
                    analyzer.add_resource_metrics(resource_stats);
                }
            }
            
            Some(analyzer.generate_comprehensive_report()?)
        } else {
            None
        };
        
        // Phase 6: Finalize Profiling
        let profile_report = if let (Some(ref mut profiler), Some(session)) = (&mut self.profiler, profile_session) {
            debug!("Phase 6: Finalizing profile report");
            Some(profiler.end_build_session(session)?)
        } else {
            None
        };
        
        // Stop metrics collection
        if let Some(ref metrics) = self.metrics_collector {
            metrics.stop_monitoring()?;
        }
        
        // Phase 7: Benchmarking (if needed)
        if self.should_run_benchmark() {
            if let Err(e) = self.run_automatic_benchmark(units) {
                warnings.push(format!("Automatic benchmarking failed: {}", e));
            }
        }
        
        let total_time = start_time.elapsed();
        
        // Create comprehensive result
        let result = OptimizationCoordinatorResult {
            compilation_successful: compilation_result.successful_units.len() == units.len(),
            total_time,
            units_compiled: compilation_result.successful_units.len(),
            units_from_cache: 0, // TODO: Track this
            units_from_incremental: 0, // TODO: Track this
            optimization_savings: compilation_result.time_saved,
            cache_hit_rate: 0.0, // TODO: Calculate this
            parallel_efficiency: compilation_result.parallel_efficiency,
            profile_report,
            performance_analysis,
            warnings,
            errors,
        };
        
        // Store result in history
        self.compilation_history.push(result.clone());
        
        // Keep only recent history
        if self.compilation_history.len() > 100 {
            self.compilation_history.drain(0..50);
        }
        
        info!(
            total_time = ?total_time,
            units_compiled = result.units_compiled,
            parallel_efficiency = result.parallel_efficiency,
            "Comprehensive optimization completed"
        );
        
        Ok(result)
    }
    
    /// Decide on the optimal compilation strategy
    fn decide_compilation_strategy(
        &self,
        units: &[CompilationUnit],
        dependency_graph: &DependencyGraph,
        incremental_plan: Option<&IncrementalBuildPlan>,
    ) -> Result<CompilationStrategy> {
        let unit_count = units.len();
        let has_dependencies = !dependency_graph.get_dependencies().is_empty();
        let incremental_savings = incremental_plan
            .map(|p| p.estimated_time_savings)
            .unwrap_or(Duration::from_secs(0));
        
        // Decide on parallel vs sequential
        let use_parallel = self.config.enable_parallel 
            && unit_count >= self.config.parallel_config.effective_worker_count()
            && (incremental_plan.map(|p| p.units_to_compile.len()).unwrap_or(unit_count) >= 2);
        
        // Decide on incremental compilation
        let use_incremental = self.config.enable_incremental
            && incremental_plan.is_some()
            && incremental_savings > Duration::from_secs(5);
        
        // Decide on caching
        let use_caching = self.config.enable_caching;
        
        // Decide on LLVM optimization level
        let optimization_level = if unit_count > 50 {
            "O1".to_string() // Faster compilation for large projects
        } else {
            self.config.llvm_config.optimization_level.clone()
        };
        
        debug!(
            use_parallel = use_parallel,
            use_incremental = use_incremental,
            use_caching = use_caching,
            optimization_level = %optimization_level,
            "Selected compilation strategy"
        );
        
        Ok(CompilationStrategy {
            use_parallel,
            use_incremental,
            use_caching,
            optimization_level,
            expected_time_savings: incremental_savings,
        })
    }
    
    /// Execute the chosen compilation strategy
    fn execute_compilation_strategy(
        &mut self,
        units: &mut [CompilationUnit],
        dependency_graph: &DependencyGraph,
        incremental_plan: Option<IncrementalBuildPlan>,
        strategy: CompilationStrategy,
        profile_session: Option<&ProfileSession>,
    ) -> Result<CompilationExecutionResult> {
        let start_time = Instant::now();
        
        // Profile each unit if profiling is enabled
        if let (Some(ref mut profiler), Some(session)) = (&mut self.profiler, profile_session) {
            for unit in units.iter() {
                if let Err(e) = profiler.profile_compilation_unit(unit, session) {
                    warn!("Failed to profile unit {}: {}", unit.name, e);
                }
            }
        }
        
        // Execute compilation based on strategy
        let result = if strategy.use_parallel {
            // Parallel compilation
            if let Some(ref mut parallel_compiler) = self.parallel_compiler {
                debug!("Executing parallel compilation strategy");
                parallel_compiler.compile_parallel(units, Some(dependency_graph))?
            } else {
                return Err(CursedError::optimization_error("Parallel compilation not available"));
            }
        } else {
            // Sequential compilation
            debug!("Executing sequential compilation strategy");
            self.compile_sequential(units)?
        };
        
        Ok(CompilationExecutionResult {
            successful_units: result.successful_units,
            failed_units: result.failed_units,
            total_time: result.total_time,
            parallel_efficiency: result.parallel_efficiency,
            time_saved: Duration::from_secs(0), // TODO: Calculate actual savings
        })
    }
    
    /// Sequential compilation fallback
    fn compile_sequential(&self, units: &mut [CompilationUnit]) -> Result<ParallelCompilationResult> {
        let start_time = Instant::now();
        let mut successful_units = Vec::new();
        let mut failed_units = Vec::new();
        
        for unit in units.iter_mut() {
            // Simulate compilation
            unit.start_compilation();
            std::thread::sleep(Duration::from_millis(50)); // Simulate work
            
            // Most compilations succeed
            if rand::random::<f64>() > 0.05 {
                successful_units.push(unit.name.clone());
            } else {
                failed_units.push(unit.name.clone());
            }
        }
        
        Ok(ParallelCompilationResult {
            successful_units,
            failed_units,
            total_time: start_time.elapsed(),
            parallel_efficiency: 1.0, // Sequential is 100% efficient by definition
            jobs_per_second: units.len() as f64 / start_time.elapsed().as_secs_f64(),
            statistics: Default::default(),
        })
    }
    
    /// Check if automatic benchmarking should be run
    fn should_run_benchmark(&self) -> bool {
        match self.config.benchmark_frequency {
            BenchmarkFrequency::Never => false,
            BenchmarkFrequency::Always => true,
            BenchmarkFrequency::OnceDaily => {
                self.last_benchmark_time.map_or(true, |last| {
                    last.elapsed() > Duration::from_secs(24 * 3600)
                })
            }
            BenchmarkFrequency::Weekly => {
                self.last_benchmark_time.map_or(true, |last| {
                    last.elapsed() > Duration::from_secs(7 * 24 * 3600)
                })
            }
            BenchmarkFrequency::AfterMajorChanges => {
                // This would need more sophisticated change detection
                false
            }
        }
    }
    
    /// Run automatic benchmark
    fn run_automatic_benchmark(&mut self, units: &[CompilationUnit]) -> Result<()> {
        if let Some(ref mut benchmarking_engine) = self.benchmarking_engine {
            debug!("Running automatic benchmark");
            
            let benchmark_config = BenchmarkConfig {
                name: format!("auto_benchmark_{}", chrono::Utc::now().format("%Y%m%d_%H%M%S")),
                benchmark_type: crate::optimization::benchmarking::BenchmarkType::CompilationSpeed,
                iterations: 3,
                warmup_iterations: 1,
                test_data: crate::optimization::benchmarking::BenchmarkTestData {
                    unit_count: units.len(),
                    complexity_level: crate::optimization::benchmarking::ComplexityLevel::Medium,
                    data_size_mb: (units.len() as f64 * 0.1).max(1.0),
                },
            };
            
            let result = benchmarking_engine.run_benchmark(benchmark_config)?;
            
            // Add results to performance analyzer if available
            if let Some(ref mut analyzer) = self.performance_analyzer {
                analyzer.add_benchmark_results(result.name.clone(), &result);
            }
            
            self.last_benchmark_time = Some(Instant::now());
            info!("Automatic benchmark completed: {:.2}ms average", result.statistics.mean_time_ms);
        }
        
        Ok(())
    }
    
    /// Get current optimization statistics
    pub fn get_statistics(&self) -> OptimizationCoordinatorStatistics {
        let total_compilations = self.compilation_history.len();
        let successful_compilations = self.compilation_history.iter()
            .filter(|r| r.compilation_successful)
            .count();
        
        let average_time = if !self.compilation_history.is_empty() {
            let total_time: Duration = self.compilation_history.iter()
                .map(|r| r.total_time)
                .sum();
            total_time / total_compilations as u32
        } else {
            Duration::from_secs(0)
        };
        
        let average_parallel_efficiency = if !self.compilation_history.is_empty() {
            self.compilation_history.iter()
                .map(|r| r.parallel_efficiency)
                .sum::<f64>() / total_compilations as f64
        } else {
            0.0
        };
        
        OptimizationCoordinatorStatistics {
            total_compilations,
            successful_compilations,
            average_compilation_time: average_time,
            average_parallel_efficiency,
            cache_enabled: self.config.enable_caching,
            incremental_enabled: self.config.enable_incremental,
            parallel_enabled: self.config.enable_parallel,
        }
    }
    
    /// Update coordinator configuration
    pub fn update_config(&mut self, new_config: OptimizationCoordinatorConfig) -> Result<()> {
        info!("Updating optimization coordinator configuration");
        
        // Update component configurations
        if let Some(ref mut cache_manager) = self.cache_manager {
            cache_manager.update_config(new_config.cache_config.clone())?;
        }
        
        if let Some(ref mut incremental_compiler) = self.incremental_compiler {
            incremental_compiler.update_config(new_config.incremental_config.clone())?;
        }
        
        if let Some(ref mut parallel_compiler) = self.parallel_compiler {
            parallel_compiler.update_config(new_config.parallel_config.clone())?;
        }
        
        self.config = new_config;
        Ok(())
    }
}

/// Compilation strategy decisions
#[derive(Debug, Clone)]
struct CompilationStrategy {
    use_parallel: bool,
    use_incremental: bool,
    use_caching: bool,
    optimization_level: String,
    expected_time_savings: Duration,
}

/// Compilation execution result
#[derive(Debug, Clone)]
struct CompilationExecutionResult {
    successful_units: Vec<String>,
    failed_units: Vec<String>,
    total_time: Duration,
    parallel_efficiency: f64,
    time_saved: Duration,
}

/// Statistics for the optimization coordinator
#[derive(Debug, Clone)]
pub struct OptimizationCoordinatorStatistics {
    pub total_compilations: usize,
    pub successful_compilations: usize,
    pub average_compilation_time: Duration,
    pub average_parallel_efficiency: f64,
    pub cache_enabled: bool,
    pub incremental_enabled: bool,
    pub parallel_enabled: bool,
}

// Simple random number generation for simulation
mod rand {
    use std::cell::Cell;
    
    thread_local! {
        static RNG_STATE: Cell<u64> = Cell::new(1);
    }
    
    pub fn random<T>() -> T 
    where 
        T: From<u64>
    {
        RNG_STATE.with(|state| {
            let current = state.get();
            let next = current.wrapping_mul(1103515245).wrapping_add(12345);
            state.set(next);
            T::from(next)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinator_creation() {
        let config = OptimizationCoordinatorConfig::default();
        let coordinator = OptimizationCoordinator::new(config);
        assert!(coordinator.is_ok());
    }

    #[test]
    fn test_strategy_decision() {
        let config = OptimizationCoordinatorConfig::default();
        let coordinator = OptimizationCoordinator::new(config).unwrap();
        
        let units = vec![
            CompilationUnit::new("unit1".to_string()),
            CompilationUnit::new("unit2".to_string()),
        ];
        
        let dependency_graph = DependencyGraph {
            dependencies: HashMap::new(),
            reverse_dependencies: HashMap::new(),
            units: HashMap::new(),
        };
        
        let strategy = coordinator.decide_compilation_strategy(&units, &dependency_graph, None);
        assert!(strategy.is_ok());
    }
}
