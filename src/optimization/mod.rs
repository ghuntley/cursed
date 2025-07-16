// Optimization modules for CURSED
pub mod config;
pub mod optimization_manager;
pub mod real_llvm_passes;
pub mod simple_passes;
pub mod enhanced_llvm_passes_manager;
pub mod coordinator;
pub mod passes;
pub mod performance_monitor;
pub mod performance_system;
pub mod analysis;
pub mod cache;
pub mod types;
pub mod lto;
pub mod llvm_passes;
pub mod optimization_levels;
pub mod enhanced_llvm_optimization;
pub mod performance_analysis;
pub mod metrics;
pub mod compilation_speed;
pub mod pgo;
pub mod benchmarking;
pub mod incremental;
pub mod benchmarks;
pub mod parallel;
pub mod profiler;
pub mod comprehensive_performance_system;
pub mod distributed;
pub mod performance_integration;
pub mod build_integration;

// New comprehensive optimization modules
pub mod production_llvm_optimization;
pub mod enhanced_performance_monitor;
pub mod comprehensive_benchmarking;
pub mod advanced_llvm_passes;

// Complete optimization system modules (remaining 15%)
pub mod llvm_optimization_complete;

// Re-export key types
pub use config::{OptimizationConfig, OptimizationLevel, OptimizationProfile};
pub use real_llvm_passes::RealLlvmPassManager;
pub use enhanced_llvm_passes_manager::EnhancedLlvmPassManager;
pub use coordinator::{OptimizationCoordinator, CoordinatorConfiguration as CoordinatorConfig};
pub use performance_monitor::PerformanceMonitor;
pub use types::{OptimizationStats, OptimizationResult};
pub use optimization_manager::{
    PerformanceMetrics, AdvancedOptimizationManager, OptimizationManager
};
pub use performance_integration::{
    ImplementationEffort, IntegratedOptimizationResults, ProjectCharacteristics,
    PerformanceIntegrationSystem, PerformanceIntegrationConfig, PerformanceTargets
};
pub use build_integration::{BuildContext, create_build_optimizer_from_args_with_performance};
pub use benchmarking::BenchmarkResults;

// New comprehensive optimization exports
pub use production_llvm_optimization::{
    ProductionLlvmOptimizer, ComprehensiveOptimizationResult, ModuleMetrics,
    OptimizationStatistics, PassDependencyResolver, InliningDecision, LoopInfo
};
pub use enhanced_performance_monitor::{
    EnhancedPerformanceMonitor, PerformanceSummary, BaselineMetrics,
    PerformanceThresholds, BenchmarkConfig as EnhancedBenchmarkConfig, 
    BenchmarkResult as EnhancedBenchmarkResult, BaselineComparison
};
pub use comprehensive_benchmarking::{
    ComprehensiveBenchmarkingSystem, BenchmarkSuiteConfig, BenchmarkSuiteResult,
    BenchmarkStatistics, BenchmarkMeasurement, CrossBenchmarkAnalysis,
    RegressionAnalysis, RegressionSeverity as BenchmarkRegressionSeverity
};
pub use optimization_levels::{
    OptimizationLevelController, OptimizationLevelConfig, LevelRecommendation,
    BuildContext as OptBuildContext, BuildType, TargetPlatform, ValidationResult
};
pub use advanced_llvm_passes::{
    AdvancedLlvmPassManager, AdvancedOptimizationConfig, PgoManager, OptimizationResult as AdvancedOptimizationResult,
    OptimizationStats as AdvancedOptimizationStats, BenchmarkReport, LtoLevel, SizeOptLevel, PassPipeline
};

// Complete optimization system exports
pub use llvm_optimization_complete::{
    CompleteLlvmOptimizer, CompleteOptimizationResult, CompleteOptimizationStats,
    MemoryPressure, InliningConfiguration, RecommendationType
};

// Stub types that examples are trying to import
#[derive(Debug, Clone)]
pub struct BaselineComparator {
    config: BaselineComparisonConfig,
}

#[derive(Debug, Clone)]
pub struct BaselineComparisonConfig {
    pub tolerance: f64,
    pub min_samples: usize,
    pub confidence_level: f64,
    pub regression_threshold_percent: f64,
    pub improvement_threshold_percent: f64,
    pub min_confidence_level: f64,
    pub max_baseline_age_days: u32,
    pub use_statistical_testing: bool,
}

#[derive(Debug, Clone)]
pub struct BaselineMetadata {
    pub version: String,
    pub timestamp: std::time::SystemTime,
    pub environment: EnvironmentInfo,
    pub benchmark_results: Vec<BenchmarkResult>,
    pub metadata: BaselineInfo,
}

#[derive(Debug, Clone)]
pub struct BaselineInfo {
    pub version: String,
    pub timestamp: std::time::SystemTime,
    pub environment: EnvironmentInfo,
}

#[derive(Debug, Clone)]
pub struct EnvironmentInfo {
    pub os: String,
    pub arch: String,
    pub cpu_count: usize,
    pub memory_gb: f64,
    pub cpu_cores: usize,
    pub memory_mb: usize,
}

#[derive(Debug, Clone)]
pub struct TimeSavingsConfig {
    pub baseline_time: std::time::Duration,
    pub optimization_time: std::time::Duration,
    pub threshold: f64,
    pub baseline_compile_time_per_unit: std::time::Duration,
    pub cache_lookup_time: std::time::Duration,
    pub incremental_analysis_time: std::time::Duration,
    pub parallel_scheduling_overhead: std::time::Duration,
    pub include_confidence_intervals: bool,
}

#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    pub iterations: usize,
    pub warmup_iterations: usize,
    pub timeout: std::time::Duration,
}

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub duration: std::time::Duration,
    pub throughput: f64,
    pub memory_usage: usize,
    pub cpu_usage: f64,
}

#[derive(Debug, Clone)]
pub enum RegressionSeverity {
    Critical,
    Major,
    Minor,
    Warning,
}

impl BaselineComparator {
    pub fn new(config: BaselineComparisonConfig) -> Self {
        Self { config }
    }
}

impl Default for BaselineComparisonConfig {
    fn default() -> Self {
        Self {
            tolerance: 0.05,
            min_samples: 10,
            confidence_level: 0.95,
            regression_threshold_percent: 5.0,
            improvement_threshold_percent: 5.0,
            min_confidence_level: 0.8,
            max_baseline_age_days: 30,
            use_statistical_testing: true,
        }
    }
}

impl Default for TimeSavingsConfig {
    fn default() -> Self {
        Self {
            baseline_time: std::time::Duration::from_secs(10),
            optimization_time: std::time::Duration::from_secs(8),
            threshold: 0.1,
            baseline_compile_time_per_unit: std::time::Duration::from_secs(3),
            cache_lookup_time: std::time::Duration::from_millis(50),
            incremental_analysis_time: std::time::Duration::from_millis(100),
            parallel_scheduling_overhead: std::time::Duration::from_millis(200),
            include_confidence_intervals: false,
        }
    }
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            iterations: 100,
            warmup_iterations: 10,
            timeout: std::time::Duration::from_secs(60),
        }
    }
}

// Test module
#[cfg(test)]
pub mod tests;

// Additional types for optimization examples
#[derive(Debug, Clone)]
pub struct TimingContext {
    pub optimization_timings: std::collections::HashMap<String, std::time::Duration>,
    pub cache_timings: CacheTimings,
    pub parallel_metrics: ParallelMetrics,
}

#[derive(Debug, Clone)]
pub struct CacheTimings {
    pub total_lookup_time: std::time::Duration,
    pub cache_hits: usize,
    pub cache_misses: usize,
}

#[derive(Debug, Clone)]
pub struct ParallelMetrics {
    pub worker_threads: usize,
    pub thread_utilizations: Vec<f64>,
    pub work_stealing_events: usize,
    pub synchronization_overhead: std::time::Duration,
}

#[derive(Debug, Clone)]
pub struct TimeSavingsCalculator {
    pub config: TimeSavingsConfig,
}

#[derive(Debug, Clone)]
pub struct TimeSavingsResult {
    pub total_time_saved: std::time::Duration,
    pub cache_savings: std::time::Duration,
    pub incremental_savings: std::time::Duration,
    pub parallel_savings: std::time::Duration,
    pub llvm_optimization_savings: std::time::Duration,
    pub efficiency_improvement_percent: f64,
    pub throughput_improvement: f64,
    pub savings_breakdown: std::collections::HashMap<String, OptimizationSavings>,
}

#[derive(Debug, Clone)]
pub struct OptimizationSavings {
    pub optimization_name: String,
    pub time_saved: std::time::Duration,
    pub units_affected: usize,
    pub avg_savings_per_unit: std::time::Duration,
    pub confidence_level: f64,
    pub metadata: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct TimeSavingsTrend {
    pub average_efficiency_ratio: f64,
    pub average_parallel_efficiency: f64,
    pub measurement_count: usize,
    pub trend_direction: TrendDirection,
}

#[derive(Debug, Clone)]
pub enum TrendDirection {
    Improving,
    Stable,
    Declining,
}

#[derive(Debug, Clone)]
pub struct OptimizationRecommendation {
    pub category: RecommendationCategory,
    pub priority: RecommendationPriority,
    pub description: String,
    pub suggested_config: Option<String>,
}

#[derive(Debug, Clone)]
pub enum RecommendationCategory {
    Performance,
    Memory,
    Size,
    Compilation,
}

#[derive(Debug, Clone)]
pub enum RecommendationPriority {
    High,
    Medium,
    Low,
}

impl TimingContext {
    pub fn new() -> Self {
        Self {
            optimization_timings: std::collections::HashMap::new(),
            cache_timings: CacheTimings {
                total_lookup_time: std::time::Duration::from_millis(0),
                cache_hits: 0,
                cache_misses: 0,
            },
            parallel_metrics: ParallelMetrics {
                worker_threads: 1,
                thread_utilizations: vec![],
                work_stealing_events: 0,
                synchronization_overhead: std::time::Duration::from_millis(0),
            },
        }
    }
}

impl TimeSavingsCalculator {
    pub fn new(config: TimeSavingsConfig) -> Self {
        Self { config }
    }
    
    pub fn record_optimization_timing(&mut self, context: &mut TimingContext, optimization_name: &str, duration: std::time::Duration) {
        context.optimization_timings.insert(optimization_name.to_string(), duration);
    }
}
