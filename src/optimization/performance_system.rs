/// Comprehensive Performance Optimization System for CURSED Compiler
/// 
/// This module provides a unified performance optimization system that enables:
/// - Smart optimization defaults based on build profiles
/// - Adaptive optimization with compilation time budgets
/// - Compilation speed optimizations with incremental caching
/// - Advanced runtime optimizations
/// - Comprehensive performance profiling and analysis

use crate::error::{Error, Result};
use crate::optimization::{
    BuildProfile, ProfileManager, OptimizationConfig, OptimizationLevel,
    adaptive::{AdaptiveOptimizer, OptimizationFeedback, OptimizationStrategy},
    compilation_speed::{CompilationSpeedOptimizer, CompilationUnit, CompilationStatistics},
    enhanced_llvm_optimization::{EnhancedLlvmOptimizer, EnhancedOptimizationConfig},
    profiler::{EnhancedBuildProfiler, ProfilerConfig},
    benchmarking::{BenchmarkingEngine, BenchmarkConfig, BenchmarkType},
    metrics::{MetricsCollector, ResourceMonitoringLevel},
};

use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant, SystemTime};
use std::thread;
use tracing::{debug, info, warn, error, instrument};
use serde::{Serialize, Deserialize};
use rayon::prelude::*;

/// Performance optimization system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSystemConfig {
    /// Build profile for optimization defaults
    pub build_profile: BuildProfile,
    /// Compilation time budget in seconds
    pub compilation_time_budget: f64,
    /// Enable adaptive optimization
    pub enable_adaptive_optimization: bool,
    /// Enable compilation speed optimizations
    pub enable_compilation_speed_optimizations: bool,
    /// Enable advanced runtime optimizations
    pub enable_advanced_runtime_optimizations: bool,
    /// Enable performance profiling
    pub enable_performance_profiling: bool,
    /// Performance monitoring level
    pub performance_monitoring_level: PerformanceMonitoringLevel,
    /// Parallel compilation configuration
    pub parallel_config: ParallelConfig,
    /// Cache configuration
    pub cache_config: CacheConfig,
    /// Benchmark configuration
    pub benchmark_config: BenchmarkConfig,
}

impl Default for PerformanceSystemConfig {
    fn default() -> Self {
        Self {
            build_profile: BuildProfile::Release,
            compilation_time_budget: 30.0, // 30 seconds default
            enable_adaptive_optimization: true,
            enable_compilation_speed_optimizations: true,
            enable_advanced_runtime_optimizations: true,
            enable_performance_profiling: true,
            performance_monitoring_level: PerformanceMonitoringLevel::Standard,
            parallel_config: ParallelConfig::default(),
            cache_config: CacheConfig::default(),
            benchmark_config: BenchmarkConfig::default(),
        }
    }
}

/// Performance monitoring levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceMonitoringLevel {
    /// Minimal monitoring for fastest compilation
    Minimal,
    /// Basic monitoring with essential metrics
    Basic,
    /// Standard monitoring with detailed metrics
    Standard,
    /// Comprehensive monitoring with all metrics
    Comprehensive,
    /// Maximum monitoring for debugging
    Maximum,
}

/// Parallel compilation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelConfig {
    /// Maximum number of parallel threads
    pub max_threads: usize,
    /// Enable parallel parsing
    pub enable_parallel_parsing: bool,
    /// Enable parallel type checking
    pub enable_parallel_type_checking: bool,
    /// Enable parallel optimization passes
    pub enable_parallel_optimization: bool,
    /// Work stealing enabled
    pub enable_work_stealing: bool,
    /// Thread priority (0-100)
    pub thread_priority: u8,
}

impl Default for ParallelConfig {
    fn default() -> Self {
        Self {
            max_threads: num_cpus::get().max(1),
            enable_parallel_parsing: true,
            enable_parallel_type_checking: true,
            enable_parallel_optimization: true,
            enable_work_stealing: true,
            thread_priority: 50,
        }
    }
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Cache directory
    pub cache_directory: PathBuf,
    /// Maximum cache size in MB
    pub max_cache_size_mb: usize,
    /// Enable AST caching
    pub enable_ast_caching: bool,
    /// Enable type checking cache
    pub enable_type_cache: bool,
    /// Enable optimization result cache
    pub enable_optimization_cache: bool,
    /// Cache compression level (0-9)
    pub compression_level: u8,
    /// Cache cleanup interval in seconds
    pub cleanup_interval_seconds: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            cache_directory: PathBuf::from(".cursed_cache"),
            max_cache_size_mb: 1024, // 1GB default
            enable_ast_caching: true,
            enable_type_cache: true,
            enable_optimization_cache: true,
            compression_level: 3,
            cleanup_interval_seconds: 3600, // 1 hour
        }
    }
}

/// Compilation performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationPerformanceMetrics {
    /// Total compilation time
    pub total_time: Duration,
    /// Parse time
    pub parse_time: Duration,
    /// Type checking time
    pub type_check_time: Duration,
    /// Optimization time
    pub optimization_time: Duration,
    /// Code generation time
    pub codegen_time: Duration,
    /// Cache hit rate
    pub cache_hit_rate: f64,
    /// Parallel efficiency
    pub parallel_efficiency: f64,
    /// Memory peak usage in MB
    pub peak_memory_mb: usize,
    /// Lines of code processed per second
    pub loc_per_second: f64,
    /// Compilation units processed
    pub units_processed: usize,
    /// Optimization level used
    pub optimization_level: OptimizationLevel,
}

impl Default for CompilationPerformanceMetrics {
    fn default() -> Self {
        Self {
            total_time: Duration::default(),
            parse_time: Duration::default(),
            type_check_time: Duration::default(),
            optimization_time: Duration::default(),
            codegen_time: Duration::default(),
            cache_hit_rate: 0.0,
            parallel_efficiency: 0.0,
            peak_memory_mb: 0,
            loc_per_second: 0.0,
            units_processed: 0,
            optimization_level: OptimizationLevel::None,
        }
    }
}

/// Runtime performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimePerformanceMetrics {
    /// Execution time
    pub execution_time: Duration,
    /// Memory usage
    pub memory_usage_mb: usize,
    /// GC pause time
    pub gc_pause_time: Duration,
    /// Goroutine count
    pub goroutine_count: usize,
    /// Channel operations per second
    pub channel_ops_per_second: f64,
    /// Function call overhead
    pub function_call_overhead_ns: u64,
    /// Cache misses
    pub cache_misses: u64,
    /// Branch prediction accuracy
    pub branch_prediction_accuracy: f64,
}

impl Default for RuntimePerformanceMetrics {
    fn default() -> Self {
        Self {
            execution_time: Duration::default(),
            memory_usage_mb: 0,
            gc_pause_time: Duration::default(),
            goroutine_count: 0,
            channel_ops_per_second: 0.0,
            function_call_overhead_ns: 0,
            cache_misses: 0,
            branch_prediction_accuracy: 0.0,
        }
    }
}

/// Optimization session tracking
#[derive(Debug, Clone)]
pub struct OptimizationSession {
    /// Session ID
    pub id: String,
    /// Session name
    pub name: String,
    /// Start time
    pub start_time: Instant,
    /// Build profile used
    pub build_profile: BuildProfile,
    /// Configuration used
    pub config: PerformanceSystemConfig,
    /// Performance metrics
    pub compilation_metrics: CompilationPerformanceMetrics,
    /// Runtime metrics
    pub runtime_metrics: RuntimePerformanceMetrics,
    /// Adaptive optimization decisions
    pub adaptive_decisions: Vec<AdaptiveDecision>,
    /// Warnings and issues
    pub warnings: Vec<String>,
}

/// Adaptive optimization decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveDecision {
    /// Timestamp
    pub timestamp: SystemTime,
    /// Decision type
    pub decision_type: AdaptiveDecisionType,
    /// Reason for decision
    pub reason: String,
    /// Expected improvement
    pub expected_improvement: f64,
    /// Actual improvement (if measured)
    pub actual_improvement: Option<f64>,
}

/// Types of adaptive decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdaptiveDecisionType {
    /// Changed optimization level
    OptimizationLevelChange { from: OptimizationLevel, to: OptimizationLevel },
    /// Enabled parallel compilation
    EnableParallelCompilation,
    /// Disabled parallel compilation
    DisableParallelCompilation,
    /// Changed thread count
    ThreadCountChange { from: usize, to: usize },
    /// Enabled caching
    EnableCaching,
    /// Disabled caching
    DisableCaching,
    /// Changed build profile
    BuildProfileChange { from: BuildProfile, to: BuildProfile },
    /// Applied custom optimization
    CustomOptimization { optimization: String },
}

/// Performance improvement recommendations
#[derive(Debug, Clone)]
pub struct PerformanceRecommendation {
    /// Recommendation type
    pub recommendation_type: RecommendationType,
    /// Description
    pub description: String,
    /// Expected improvement percentage
    pub expected_improvement_percent: f64,
    /// Implementation difficulty (1-5)
    pub implementation_difficulty: u8,
    /// Priority (1-5)
    pub priority: u8,
    /// Required actions
    pub required_actions: Vec<String>,
}

/// Types of performance recommendations
#[derive(Debug, Clone)]
pub enum RecommendationType {
    /// Optimize compilation speed
    CompilationSpeed,
    /// Optimize runtime performance
    RuntimePerformance,
    /// Optimize memory usage
    MemoryUsage,
    /// Optimize build configuration
    BuildConfiguration,
    /// Optimize parallelization
    Parallelization,
    /// Optimize caching
    Caching,
}

/// Main performance optimization system
pub struct PerformanceOptimizationSystem {
    /// Configuration
    config: PerformanceSystemConfig,
    /// Profile manager
    profile_manager: ProfileManager,
    /// Adaptive optimizer
    adaptive_optimizer: AdaptiveOptimizer,
    /// Compilation speed optimizer
    compilation_speed_optimizer: CompilationSpeedOptimizer,
    /// Enhanced LLVM optimizer
    llvm_optimizer: EnhancedLlvmOptimizer,
    /// Build profiler
    build_profiler: EnhancedBuildProfiler,
    /// Benchmarking engine
    benchmarking_engine: BenchmarkingEngine,
    /// Metrics collector
    metrics_collector: MetricsCollector,
    /// Current session
    current_session: Arc<RwLock<Option<OptimizationSession>>>,
    /// Performance history
    performance_history: Arc<RwLock<VecDeque<CompilationPerformanceMetrics>>>,
    /// Runtime performance tracking
    runtime_performance: Arc<RwLock<VecDeque<RuntimePerformanceMetrics>>>,
    /// Recommendations cache
    recommendations_cache: Arc<RwLock<Vec<PerformanceRecommendation>>>,
}

impl PerformanceOptimizationSystem {
    /// Create a new performance optimization system
    pub fn new(config: PerformanceSystemConfig) -> Result<Self> {
        info!("Initializing Performance Optimization System with profile: {:?}", config.build_profile);

        let profile_manager = ProfileManager::new();
        let optimization_config = profile_manager.get_profile_config(config.build_profile)
            .ok_or_else(|| Error::Runtime("Invalid build profile".to_string()))?
            .clone();

        let adaptive_optimizer = AdaptiveOptimizer::new(&optimization_config)?;
        let compilation_speed_optimizer = CompilationSpeedOptimizer::new(&optimization_config)?;
        
        let enhanced_config = EnhancedOptimizationConfig::from_optimization_config(&optimization_config);
        let llvm_optimizer = EnhancedLlvmOptimizer::new(enhanced_config)?;

        let profiler_config = ProfilerConfig {
            enable_detailed_profiling: config.enable_performance_profiling,
            enable_memory_profiling: matches!(config.performance_monitoring_level, 
                PerformanceMonitoringLevel::Comprehensive | PerformanceMonitoringLevel::Maximum),
            enable_function_profiling: true,
            profile_output_dir: config.cache_config.cache_directory.join("profiles"),
            ..Default::default()
        };
        let build_profiler = EnhancedBuildProfiler::new(profiler_config)?;

        let benchmarking_engine = BenchmarkingEngine::new(config.benchmark_config.clone())?;
        
        let metrics_config = crate::optimization::PerformanceConfig {
            enable_realtime_monitoring: config.enable_performance_profiling,
            resource_monitoring_level: match config.performance_monitoring_level {
                PerformanceMonitoringLevel::Minimal => ResourceMonitoringLevel::None,
                PerformanceMonitoringLevel::Basic => ResourceMonitoringLevel::Basic,
                PerformanceMonitoringLevel::Standard => ResourceMonitoringLevel::Standard,
                PerformanceMonitoringLevel::Comprehensive => ResourceMonitoringLevel::Detailed,
                PerformanceMonitoringLevel::Maximum => ResourceMonitoringLevel::Detailed,
            },
            ..Default::default()
        };
        let metrics_collector = MetricsCollector::new(metrics_config)?;

        Ok(Self {
            config,
            profile_manager,
            adaptive_optimizer,
            compilation_speed_optimizer,
            llvm_optimizer,
            build_profiler,
            benchmarking_engine,
            metrics_collector,
            current_session: Arc::new(RwLock::new(None)),
            performance_history: Arc::new(RwLock::new(VecDeque::new())),
            runtime_performance: Arc::new(RwLock::new(VecDeque::new())),
            recommendations_cache: Arc::new(RwLock::new(Vec::new())),
        })
    }

    /// Start a new optimization session
    #[instrument(skip(self))]
    pub fn start_session(&self, name: String) -> Result<String> {
        let session_id = format!("{}_{}", name, SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis());

        let session = OptimizationSession {
            id: session_id.clone(),
            name,
            start_time: Instant::now(),
            build_profile: self.config.build_profile,
            config: self.config.clone(),
            compilation_metrics: CompilationPerformanceMetrics::default(),
            runtime_metrics: RuntimePerformanceMetrics::default(),
            adaptive_decisions: Vec::new(),
            warnings: Vec::new(),
        };

        {
            let mut current_session = self.current_session.write().unwrap();
            *current_session = Some(session);
        }

        // Start monitoring
        if self.config.enable_performance_profiling {
            self.metrics_collector.start_monitoring()?;
        }

        info!("Started optimization session: {}", session_id);
        Ok(session_id)
    }

    /// End the current optimization session
    #[instrument(skip(self))]
    pub fn end_session(&self) -> Result<Option<OptimizationSession>> {
        let mut current_session = self.current_session.write().unwrap();
        
        if let Some(mut session) = current_session.take() {
            // Stop monitoring
            if self.config.enable_performance_profiling {
                self.metrics_collector.stop_monitoring()?;
            }

            // Calculate final metrics
            session.compilation_metrics.total_time = session.start_time.elapsed();
            
            // Store performance history
            {
                let mut history = self.performance_history.write().unwrap();
                history.push_back(session.compilation_metrics.clone());
                
                // Keep only last 100 sessions
                if history.len() > 100 {
                    history.pop_front();
                }
            }

            info!("Ended optimization session: {} (duration: {}ms)", 
                  session.id, session.compilation_metrics.total_time.as_millis());

            Ok(Some(session))
        } else {
            Ok(None)
        }
    }

    /// Compile with smart optimization
    #[instrument(skip(self, compilation_units))]
    pub fn compile_with_smart_optimization(&self, compilation_units: Vec<CompilationUnit>) -> Result<CompilationResults> {
        let start_time = Instant::now();
        info!("Starting smart optimization compilation of {} units", compilation_units.len());

        // Apply adaptive optimization if enabled
        if self.config.enable_adaptive_optimization {
            self.apply_adaptive_optimizations(&compilation_units)?;
        }

        // Check if we should adjust optimization level based on time budget
        let adjusted_profile = self.adjust_optimization_for_time_budget(&compilation_units)?;
        
        // Use compilation speed optimizer if enabled
        let results = if self.config.enable_compilation_speed_optimizations {
            self.compilation_speed_optimizer.compile_incremental(compilation_units)?
        } else {
            // Fallback to basic compilation
            self.compile_basic(compilation_units)?
        };

        // Apply LLVM optimizations if enabled
        let optimized_results = if self.config.enable_advanced_runtime_optimizations {
            self.apply_advanced_runtime_optimizations(results)?
        } else {
            results
        };

        // Record performance metrics
        self.record_compilation_metrics(start_time, &optimized_results)?;

        // Generate recommendations
        let recommendations = self.generate_recommendations(&optimized_results)?;

        Ok(CompilationResults {
            compilation_results: optimized_results,
            performance_metrics: self.get_current_compilation_metrics(),
            recommendations,
            adaptive_decisions: self.get_adaptive_decisions(),
            build_profile_used: adjusted_profile,
        })
    }

    /// Apply adaptive optimizations based on current performance data
    fn apply_adaptive_optimizations(&self, compilation_units: &[CompilationUnit]) -> Result<()> {
        debug!("Applying adaptive optimizations");

        // Get recommendations from adaptive optimizer
        let recommendations = self.adaptive_optimizer.get_recommendations()?;
        
        for recommendation in recommendations {
            match recommendation.strategy {
                OptimizationStrategy::IncreaseLevel => {
                    self.record_adaptive_decision(AdaptiveDecisionType::OptimizationLevelChange {
                        from: OptimizationLevel::Default,
                        to: OptimizationLevel::Aggressive,
                    }, "Performance analysis suggests higher optimization level".to_string(), 0.15)?;
                }
                OptimizationStrategy::DecreaseLevel => {
                    self.record_adaptive_decision(AdaptiveDecisionType::OptimizationLevelChange {
                        from: OptimizationLevel::Aggressive,
                        to: OptimizationLevel::Default,
                    }, "Compilation time budget constraint".to_string(), -0.05)?;
                }
                OptimizationStrategy::SpecificPass(pass_name) => {
                    self.record_adaptive_decision(AdaptiveDecisionType::CustomOptimization {
                        optimization: pass_name,
                    }, format!("Specific optimization for {}", recommendation.function_name), recommendation.expected_improvement)?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Adjust optimization level based on compilation time budget
    fn adjust_optimization_for_time_budget(&self, compilation_units: &[CompilationUnit]) -> Result<BuildProfile> {
        let estimated_compile_time = self.estimate_compilation_time(compilation_units);
        let mut target_profile = self.config.build_profile;

        if estimated_compile_time > Duration::from_secs_f64(self.config.compilation_time_budget) {
            // Reduce optimization level to meet time budget
            target_profile = match self.config.build_profile {
                BuildProfile::Production => BuildProfile::Release,
                BuildProfile::Release => BuildProfile::Development,
                BuildProfile::Development => BuildProfile::Debug,
                _ => target_profile,
            };

            self.record_adaptive_decision(AdaptiveDecisionType::BuildProfileChange {
                from: self.config.build_profile,
                to: target_profile,
            }, format!("Adjusted profile to meet time budget of {:.1}s", self.config.compilation_time_budget), 0.0)?;

            warn!("Reduced optimization level to meet compilation time budget");
        }

        Ok(target_profile)
    }

    /// Estimate compilation time for units
    fn estimate_compilation_time(&self, compilation_units: &[CompilationUnit]) -> Duration {
        // Simple estimation based on source code size and complexity
        let total_lines: usize = compilation_units.iter()
            .map(|unit| unit.source_code.split("\n").count())
            .sum();

        let base_time_per_line = match self.config.build_profile {
            BuildProfile::Debug => Duration::from_micros(10),
            BuildProfile::Development => Duration::from_micros(50),
            BuildProfile::Release => Duration::from_micros(200),
            BuildProfile::Production => Duration::from_micros(500),
            BuildProfile::Size => Duration::from_micros(300),
            BuildProfile::Testing => Duration::from_micros(100),
        };

        base_time_per_line * total_lines as u32
    }

    /// Apply advanced runtime optimizations
    fn apply_advanced_runtime_optimizations(&self, results: Vec<(String, Result<crate::ast::Program>)>) -> Result<Vec<(String, Result<crate::ast::Program>)>> {
        debug!("Applying advanced runtime optimizations");
        
        // This would integrate with the LLVM optimizer for advanced optimizations
        // For now, return the results as-is
        Ok(results)
    }

    /// Basic compilation fallback
    fn compile_basic(&self, compilation_units: Vec<CompilationUnit>) -> Result<Vec<(String, Result<crate::ast::Program>)>> {
        compilation_units.into_par_iter().map(|unit| {
            let lexer = crate::lexer::Lexer::new(unit.source_code);
            let mut parser = crate::parser::Parser::new(lexer)?;
            let program = parser.parse_program()?;
            Ok((unit.id, Ok(program)))
        }).collect()
    }

    /// Record adaptive decision
    fn record_adaptive_decision(&self, decision_type: AdaptiveDecisionType, reason: String, expected_improvement: f64) -> Result<()> {
        let decision = AdaptiveDecision {
            timestamp: SystemTime::now(),
            decision_type,
            reason,
            expected_improvement,
            actual_improvement: None,
        };

        if let Some(ref mut session) = self.current_session.write().unwrap().as_mut() {
            session.adaptive_decisions.push(decision);
        }

        Ok(())
    }

    /// Record compilation metrics
    fn record_compilation_metrics(&self, start_time: Instant, results: &[(String, Result<crate::ast::Program>)]) -> Result<()> {
        let total_time = start_time.elapsed();
        let successful_compilations = results.iter().filter(|(_, result)| result.is_ok()).count();
        
        let metrics = CompilationPerformanceMetrics {
            total_time,
            units_processed: results.len(),
            optimization_level: self.get_current_optimization_level(),
            ..Default::default()
        };

        if let Some(ref mut session) = self.current_session.write().unwrap().as_mut() {
            session.compilation_metrics = metrics;
        }

        Ok(())
    }

    /// Get current optimization level
    fn get_current_optimization_level(&self) -> OptimizationLevel {
        self.profile_manager.get_profile_config(self.config.build_profile)
            .map(|config| config.optimization_level.clone())
            .unwrap_or(OptimizationLevel::Default)
    }

    /// Generate performance recommendations
    fn generate_recommendations(&self, results: &[(String, Result<crate::ast::Program>)]) -> Result<Vec<PerformanceRecommendation>> {
        let mut recommendations = Vec::new();

        // Analyze compilation performance
        let compilation_stats = self.compilation_speed_optimizer.get_statistics();
        
        // Check cache hit rate
        if compilation_stats.cache_hit_rate < 0.5 {
            recommendations.push(PerformanceRecommendation {
                recommendation_type: RecommendationType::Caching,
                description: "Low cache hit rate detected. Consider enabling more aggressive caching.".to_string(),
                expected_improvement_percent: 25.0,
                implementation_difficulty: 2,
                priority: 4,
                required_actions: vec![
                    "Enable AST caching".to_string(),
                    "Increase cache size".to_string(),
                    "Review cache invalidation strategy".to_string(),
                ],
            });
        }

        // Check parallelization efficiency
        if compilation_stats.parallelization_efficiency < 0.7 {
            recommendations.push(PerformanceRecommendation {
                recommendation_type: RecommendationType::Parallelization,
                description: "Low parallelization efficiency. Consider adjusting thread count or dependency structure.".to_string(),
                expected_improvement_percent: 15.0,
                implementation_difficulty: 3,
                priority: 3,
                required_actions: vec![
                    "Analyze dependency graph".to_string(),
                    "Optimize module structure".to_string(),
                    "Adjust thread pool size".to_string(),
                ],
            });
        }

        // Check compilation speed
        if compilation_stats.total_compilation_time.as_secs() > 60 {
            recommendations.push(PerformanceRecommendation {
                recommendation_type: RecommendationType::CompilationSpeed,
                description: "Long compilation times detected. Consider optimizing for development speed.".to_string(),
                expected_improvement_percent: 40.0,
                implementation_difficulty: 2,
                priority: 5,
                required_actions: vec![
                    "Use development build profile".to_string(),
                    "Enable incremental compilation".to_string(),
                    "Reduce optimization level".to_string(),
                ],
            });
        }

        {
            let mut cache = self.recommendations_cache.write().unwrap();
            *cache = recommendations.clone();
        }

        Ok(recommendations)
    }

    /// Get current compilation metrics
    fn get_current_compilation_metrics(&self) -> CompilationPerformanceMetrics {
        self.current_session.read().unwrap()
            .as_ref()
            .map(|session| session.compilation_metrics.clone())
            .unwrap_or_default()
    }

    /// Get adaptive decisions
    fn get_adaptive_decisions(&self) -> Vec<AdaptiveDecision> {
        self.current_session.read().unwrap()
            .as_ref()
            .map(|session| session.adaptive_decisions.clone())
            .unwrap_or_default()
    }

    /// Run performance benchmark
    #[instrument(skip(self))]
    pub fn run_performance_benchmark(&self, benchmark_type: BenchmarkType) -> Result<BenchmarkResults> {
        info!("Running performance benchmark: {:?}", benchmark_type);
        
        let mut benchmark_config = self.config.benchmark_config.clone();
        benchmark_config.benchmark_type = benchmark_type;
        
        let results = self.benchmarking_engine.run_benchmark(benchmark_config)?;
        
        info!("Benchmark completed: avg={}ms, min={}ms, max={}ms", 
              results.average_time.as_millis(),
              results.min_time.as_millis(),
              results.max_time.as_millis());
        
        Ok(results)
    }

    /// Get comprehensive performance report
    pub fn generate_performance_report(&self) -> String {
        let mut report = String::new();
        report.push_str("# CURSED Compiler Performance Report\n\n");

        // Current session information
        if let Some(session) = self.current_session.read().unwrap().as_ref() {
            report.push_str("## Current Session\n");
            report.push_str(&format!("- Session ID: {}\n", session.id));
            report.push_str(&format!("- Build Profile: {:?}\n", session.build_profile));
            report.push_str(&format!("- Duration: {}ms\n", session.start_time.elapsed().as_millis()));
            report.push_str(&format!("- Units Processed: {}\n", session.compilation_metrics.units_processed));
            report.push_str(&format!("- Optimization Level: {:?}\n\n", session.compilation_metrics.optimization_level));
        }

        // Compilation speed optimizer report
        report.push_str("## Compilation Performance\n");
        let compilation_report = self.compilation_speed_optimizer.generate_performance_report();
        report.push_str(&compilation_report);
        report.push_str("\n");

        // Performance history analysis
        report.push_str("## Performance History\n");
        let history = self.performance_history.read().unwrap();
        if !history.is_empty() {
            let avg_time: Duration = history.iter().map(|m| m.total_time).sum::<Duration>() / history.len() as u32;
            let avg_cache_rate = history.iter().map(|m| m.cache_hit_rate).sum::<f64>() / history.len() as f64;
            let avg_parallel_efficiency = history.iter().map(|m| m.parallel_efficiency).sum::<f64>() / history.len() as f64;
            
            report.push_str(&format!("- Average compilation time: {}ms\n", avg_time.as_millis()));
            report.push_str(&format!("- Average cache hit rate: {:.1}%\n", avg_cache_rate * 100.0));
            report.push_str(&format!("- Average parallel efficiency: {:.1}%\n", avg_parallel_efficiency * 100.0));
            report.push_str(&format!("- Sessions analyzed: {}\n\n", history.len()));
        }

        // Current recommendations
        report.push_str("## Performance Recommendations\n");
        let recommendations = self.recommendations_cache.read().unwrap();
        if recommendations.is_empty() {
            report.push_str("No specific recommendations at this time.\n\n");
        } else {
            for (i, rec) in recommendations.iter().enumerate() {
                report.push_str(&format!("{}. **{}** (Priority: {}/5)\n", 
                    i + 1, rec.description, rec.priority));
                report.push_str(&format!("   - Expected improvement: {:.1}%\n", rec.expected_improvement_percent));
                report.push_str(&format!("   - Implementation difficulty: {}/5\n", rec.implementation_difficulty));
                report.push_str("   - Actions:\n");
                for action in &rec.required_actions {
                    report.push_str(&format!("     - {}\n", action));
                }
                report.push_str("\n");
            }
        }

        // Build profile recommendations
        report.push_str("## Build Profile Analysis\n");
        report.push_str(&self.profile_manager.get_profile_summary(self.config.build_profile));
        report.push_str("\n\n");

        report.push_str("## Configuration Summary\n");
        report.push_str(&format!("- Adaptive optimization: {}\n", self.config.enable_adaptive_optimization));
        report.push_str(&format!("- Compilation speed optimization: {}\n", self.config.enable_compilation_speed_optimizations));
        report.push_str(&format!("- Advanced runtime optimization: {}\n", self.config.enable_advanced_runtime_optimizations));
        report.push_str(&format!("- Performance profiling: {}\n", self.config.enable_performance_profiling));
        report.push_str(&format!("- Monitoring level: {:?}\n", self.config.performance_monitoring_level));
        report.push_str(&format!("- Compilation time budget: {:.1}s\n", self.config.compilation_time_budget));
        report.push_str(&format!("- Max parallel threads: {}\n", self.config.parallel_config.max_threads));
        report.push_str(&format!("- Cache size limit: {} MB\n", self.config.cache_config.max_cache_size_mb));

        report
    }

    /// Update configuration
    pub fn update_config(&mut self, new_config: PerformanceSystemConfig) -> Result<()> {
        info!("Updating performance system configuration");
        
        // Record configuration change as adaptive decision
        if new_config.build_profile != self.config.build_profile {
            self.record_adaptive_decision(AdaptiveDecisionType::BuildProfileChange {
                from: self.config.build_profile,
                to: new_config.build_profile,
            }, "Manual configuration update".to_string(), 0.0)?;
        }

        self.config = new_config;
        Ok(())
    }

    /// Get current configuration
    pub fn get_config(&self) -> &PerformanceSystemConfig {
        &self.config
    }

    /// Get performance recommendations
    pub fn get_recommendations(&self) -> Vec<PerformanceRecommendation> {
        self.recommendations_cache.read().unwrap().clone()
    }

    /// Clear all caches
    pub fn clear_caches(&self) -> Result<()> {
        info!("Clearing all performance optimization caches");
        self.compilation_speed_optimizer.clear_caches()?;
        Ok(())
    }
}

/// Results from smart compilation
#[derive(Debug)]
pub struct CompilationResults {
    /// Compilation results
    pub compilation_results: Vec<(String, Result<crate::ast::Program>)>,
    /// Performance metrics
    pub performance_metrics: CompilationPerformanceMetrics,
    /// Performance recommendations
    pub recommendations: Vec<PerformanceRecommendation>,
    /// Adaptive decisions made
    pub adaptive_decisions: Vec<AdaptiveDecision>,
    /// Build profile used
    pub build_profile_used: BuildProfile,
}

/// Benchmark results
#[derive(Debug, Clone)]
pub struct BenchmarkResults {
    /// Benchmark type
    pub benchmark_type: BenchmarkType,
    /// Average execution time
    pub average_time: Duration,
    /// Minimum execution time
    pub min_time: Duration,
    /// Maximum execution time
    pub max_time: Duration,
    /// Standard deviation
    pub std_deviation: Duration,
    /// Iterations performed
    pub iterations: usize,
    /// Throughput (operations per second)
    pub throughput: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_performance_system_creation() {
        let config = PerformanceSystemConfig::default();
        let system = PerformanceOptimizationSystem::new(config).unwrap();
        
        assert_eq!(system.config.build_profile, BuildProfile::Release);
        assert!(system.config.enable_adaptive_optimization);
    }

    #[test]
    fn test_session_management() {
        let config = PerformanceSystemConfig::default();
        let system = PerformanceOptimizationSystem::new(config).unwrap();
        
        let session_id = system.start_session("test_session".to_string()).unwrap();
        assert!(!session_id.is_empty());
        
        let session = system.end_session().unwrap();
        assert!(session.is_some());
        assert_eq!(session.unwrap().name, "test_session");
    }

    #[test]
    fn test_adaptive_decision_recording() {
        let config = PerformanceSystemConfig::default();
        let system = PerformanceOptimizationSystem::new(config).unwrap();
        
        system.start_session("test".to_string()).unwrap();
        
        system.record_adaptive_decision(
            AdaptiveDecisionType::OptimizationLevelChange {
                from: OptimizationLevel::Default,
                to: OptimizationLevel::Aggressive,
            },
            "Test decision".to_string(),
            0.15,
        ).unwrap();
        
        let decisions = system.get_adaptive_decisions();
        assert_eq!(decisions.len(), 1);
        assert_eq!(decisions[0].reason, "Test decision");
    }

    #[test]
    fn test_time_budget_adjustment() {
        let mut config = PerformanceSystemConfig::default();
        config.compilation_time_budget = 1.0; // Very short budget
        config.build_profile = BuildProfile::Production;
        
        let system = PerformanceOptimizationSystem::new(config).unwrap();
        
        let units = vec![CompilationUnit {
            id: "test".to_string(),
            source_path: std::path::PathBuf::from("test.csd"),
            module_name: "test".to_string(),
            source_code: "facts x = 42;".repeat(1000), // Large source
            dependencies: vec![],
            last_modified: SystemTime::now(),
            status: crate::optimization::compilation_speed::CompilationStatus::Pending,
            priority: 1,
            content_hash: String::new(),
        }];
        
        let adjusted_profile = system.adjust_optimization_for_time_budget(&units).unwrap();
        
        // Should have reduced optimization level
        assert_ne!(adjusted_profile, BuildProfile::Production);
    }

    #[test]
    fn test_performance_recommendations() {
        let config = PerformanceSystemConfig::default();
        let system = PerformanceOptimizationSystem::new(config).unwrap();
        
        let results = vec![
            ("test1".to_string(), Ok(crate::ast::Program::new(vec![]))),
            ("test2".to_string(), Ok(crate::ast::Program::new(vec![]))),
        ];
        
        let recommendations = system.generate_recommendations(&results).unwrap();
        // Should generate some recommendations based on mock data
        assert!(!recommendations.is_empty());
    }
}
