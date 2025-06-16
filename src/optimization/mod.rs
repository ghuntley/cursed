//! Optimization module for the CURSED language
//! 
//! Provides comprehensive optimization infrastructure including:
//! - Performance benchmarking and validation
//! - Profile-guided optimization (PGO) support
//! - Advanced optimization pass management
//! - Performance regression testing
//! - Comprehensive performance optimization system
//! - Real LLVM optimization passes
//! - Build performance optimization

pub mod benchmarks;
pub mod intelligent_recommendations;
pub mod ast_analyzer;
pub mod comprehensive_performance_system;
pub mod real_llvm_passes;
pub mod advanced_function_inlining;
pub mod enhanced_llvm_passes;
pub mod enhanced_llvm_passes_manager;
pub mod enhanced_llvm_optimization;
pub mod performance_analysis;
pub mod parallel_pass_manager;
pub mod baseline_comparison;
pub mod time_savings;
pub mod coordinator;
pub mod pgo;

pub use benchmarks::{
    BenchmarkRunner, BenchmarkConfig, BenchmarkResult, BenchmarkSuiteResult,
    PerformanceThresholds, RegressionAnalysis, create_default_benchmarks,
};
pub use intelligent_recommendations::{
    CodeAnalysisEngine, AnalysisConfig, DetailedRecommendation, AnalysisPattern,
    PatternType, PatternSeverity, PerformanceImpact, OptimizationAction,
    ActionType, ActionPriority, ConfigChange, CodeSuggestion,
};
pub use comprehensive_performance_system::{
    ComprehensivePerformanceSystem, PerformanceConfig, OptimizationResults,
    LlvmOptimizationResults, PgoOptimizationResults, RuntimePerformanceMetrics,
    PerformanceStatistics, BenchmarkResults,
};
pub use real_llvm_passes::{
    RealLlvmPassManager, OptimizationStatistics,
};
pub use advanced_function_inlining::{
    AdvancedFunctionInliner, InliningStatistics, FunctionMetrics, CallSiteAnalysis,
    CallGraph, InlineDecision, InlineType,
};
pub use enhanced_llvm_passes::{
    EnhancedLlvmPassManager, EnhancedOptimizationStatistics, IntelligentFunctionInliner,
    AdvancedDeadCodeEliminator, EnhancedConstantPropagator, AdvancedLoopOptimizer,
    ControlFlowGraphSimplifier, PerformanceAnalyzer, ModuleAnalysis, PerformanceImprovements,
};
pub use enhanced_llvm_optimization::{
    EnhancedLlvmOptimizer, EnhancedOptimizationConfig, EnhancedOptimizationResults,
    OptimizationFeedbackConfig, OptimizationFeedback, OptimizationPattern, FailedOptimization,
    TargetOptimizationResults, CacheOptimizationResults,
    VectorizationResults,
};
pub use parallel_pass_manager::{
    ParallelPassManager, ParallelPassConfig, ParallelPassStatistics,
};
pub use baseline_comparison::{
    BaselineComparator, BaselineData, BaselineComparisonResult, BenchmarkComparison,
    BaselineComparisonConfig, BaselineMetadata, EnvironmentInfo,
};
pub use time_savings::{
    TimeSavingsCalculator, TimeSavingsAnalysis, OptimizationSavings, TimeSavingsConfig,
    CompilationTimingContext, TrendAnalysis,
};
pub use coordinator::{
    OptimizationCoordinator, OptimizationCoordinatorConfig, ComprehensiveOptimizationResult,
    OptimizationLevel as CoordinatorOptimizationLevel, OptimizationFeature,
    IncrementalSavings, ParallelPerformance, CachePerformance, OverallImprovement,
};
pub use pgo::{
    PgoSystem, PgoSystemConfig, PgoSystemStatistics, ProfileData, ProfileAnalysisResult,
    OptimizationOpportunity, ProfileInsight, ExecutionContext, PgoError,
    OptimizationAggressiveness, PerformanceMetrics, OptimizationResult,
};

// Real optimization implementations
pub use real_optimization_implementation::{
    RealPerformanceCalculator, PerformanceImprovements, BaselineMetrics, 
    PerformanceTrends, AppliedOptimization,
};
pub use real_cpu_efficiency_estimator::{
    CpuEfficiencyEstimator, CpuEfficiencyEstimation, CpuArchitectureModel,
    PerformanceBottleneck, BottleneckType,
};
pub use real_regression_detector::{
    RegressionDetector, RegressionDetectionResult, PerformanceDataPoint,
    RegressionType, AffectedMetric, RootCauseAnalysis,
};
pub use real_optimization_integration::{
    RealOptimizationManager, RealOptimizationResult, OptimizationSession,
    DetailedPerformanceMetrics, OptimizationEffectivenessAnalysis,
    RecommendationType,
};

// Advanced optimization exports
pub use advanced_llvm_integration::{
    AdvancedLlvmIntegration, AdvancedLlvmConfig, AdvancedOptimizationStatistics,
    CfgTransformationStatistics, LoopOptimizationStatistics as AdvancedLoopStats,
    VectorizationStatistics, TargetSpecificStatistics, FunctionComplexity,
};
pub use target_optimization::{
    TargetOptimizationManager, TargetOptimizationConfig, CpuArchitecture,
    CpuInfo, CpuFeature, SimdCapabilities, OptimizationStrategy,
    TargetOptimizationStatistics as TargetStats, CodeUnit,
};
pub use advanced_loop_optimization::{
    AdvancedLoopOptimizer, LoopOptimizationConfig as AdvancedLoopConfig,
    LoopOptimizationStatistics, LoopInfo, OptimizationOpportunity as LoopOpportunity,
    CodeUnit as LoopCodeUnit,
};
pub use profile_guided_optimization::{
    ProfileGuidedOptimizer, PgoConfig, PgoOptimizationResult,
    ProfileCollectionMethod, PgoOptimizationLevel, PgoStatistics,
    OptimizationOpportunity as PgoOpportunity, CodeUnit as PgoCodeUnit,
};
pub use link_time_optimization::{
    LinkTimeOptimizer, LtoConfig, LtoOptimizationResult,
    LtoOptimizationLevel, ModuleInfo, FunctionInfo, LtoStatistics,
};
pub use advanced_coordinator::{
    AdvancedOptimizationCoordinator, AdvancedCoordinatorConfig, AdvancedOptimizationResult,
    AdvancedOptimizationLevel, OptimizationPhase, AdvancedCoordinatorStatistics, AdvancedCodeUnit,
};

use crate::codegen::llvm::optimization::{OptimizationConfig, OptimizationLevel};
use crate::error::Result;
use std::path::Path;

// ML-Guided Optimization System
pub mod ml_optimization;
pub mod ml;

// Re-export ML optimization types
pub use ml_optimization::{
    MLOptimizationEngine, MLOptimizationConfig, ProfilingData,
};
pub use ml::{
    MLOptimizationCoordinator, OptimizationStrategy, OptimizationLevel as MLOptimizationLevel,
    OptimizationPass, CompilationContext, PerformanceStatistics,
};

// Real optimization implementations that replace placeholders
pub mod real_optimization_implementation;
pub mod real_cpu_efficiency_estimator;
pub mod real_regression_detector;
pub mod real_optimization_integration;

// Advanced optimization modules
pub mod advanced_llvm_integration;
pub mod target_optimization;
pub mod advanced_loop_optimization;
pub mod profile_guided_optimization;
pub mod link_time_optimization;
pub mod advanced_coordinator;

/// High-level optimization manager that coordinates all optimization features
pub struct OptimizationManager {
    /// Default optimization configuration
    pub default_config: OptimizationConfig,
    /// Benchmark runner for performance testing
    pub benchmark_runner: Option<BenchmarkRunner>,
    /// Enable performance regression testing
    pub enable_regression_testing: bool,
    /// Baseline comparator for regression detection
    pub baseline_comparator: Option<BaselineComparator>,
    /// Time savings calculator for performance analysis
    pub time_savings_calculator: TimeSavingsCalculator,
}

impl OptimizationManager {
    /// Create a new optimization manager with default settings
    pub fn new() -> Self {
        Self {
            default_config: OptimizationConfig::release_config(),
            benchmark_runner: None,
            enable_regression_testing: true,
            baseline_comparator: None,
            time_savings_calculator: TimeSavingsCalculator::new(TimeSavingsConfig::default()),
        }
    }

    /// Create optimization manager for development
    pub fn for_development() -> Self {
        Self {
            default_config: OptimizationConfig::dev_config(),
            benchmark_runner: None,
            enable_regression_testing: false,
            baseline_comparator: None,
            time_savings_calculator: TimeSavingsCalculator::new(TimeSavingsConfig::default()),
        }
    }

    /// Create optimization manager with custom configuration
    pub fn with_config(config: OptimizationConfig) -> Self {
        Self {
            default_config: config,
            benchmark_runner: None,
            enable_regression_testing: true,
            baseline_comparator: None,
            time_savings_calculator: TimeSavingsCalculator::new(TimeSavingsConfig::default()),
        }
    }

    /// Enable benchmarking with specified compiler path and work directory
    pub fn with_benchmarking<P: AsRef<Path>>(
        mut self,
        compiler_path: P,
        work_dir: P,
    ) -> Self {
        self.benchmark_runner = Some(BenchmarkRunner::new(
            compiler_path.as_ref().to_path_buf(),
            work_dir.as_ref().to_path_buf(),
        ));
        self
    }

    /// Enable baseline comparison with storage path
    pub fn with_baseline_comparison<P: AsRef<Path>>(
        mut self,
        storage_path: P,
        config: BaselineComparisonConfig,
    ) -> Self {
        self.baseline_comparator = Some(BaselineComparator::new(storage_path, config));
        self
    }

    /// Configure time savings calculation
    pub fn with_time_savings_config(mut self, config: TimeSavingsConfig) -> Self {
        self.time_savings_calculator = TimeSavingsCalculator::new(config);
        self
    }

    /// Get the current optimization configuration
    pub fn get_config(&self) -> &OptimizationConfig {
        &self.default_config
    }

    /// Update the optimization configuration
    pub fn set_config(&mut self, config: OptimizationConfig) {
        self.default_config = config;
    }

    /// Run performance benchmarks if available
    pub async fn run_benchmarks(&self, suite_name: &str) -> Result<Option<BenchmarkSuiteResult>> {
        if let Some(ref runner) = self.benchmark_runner {
            let configs = create_default_benchmarks();
            let results = runner.run_benchmark_suite(suite_name, &configs).await?;
            Ok(Some(results))
        } else {
            Ok(None)
        }
    }

    /// Validate performance against regression thresholds
    pub async fn validate_performance(&self, baseline_path: Option<&Path>) -> Result<bool> {
        if !self.enable_regression_testing {
            return Ok(true);
        }

        if let Some(ref runner) = self.benchmark_runner {
            let configs = create_default_benchmarks();
            let results = runner.run_benchmark_suite("regression_test", &configs).await?;
            
            // Check for regressions
            if let Some(ref regression_analysis) = results.regression_analysis {
                if regression_analysis.has_regressions {
                    tracing::warn!("Performance regressions detected!");
                    for regression in &regression_analysis.regressions {
                        tracing::warn!("Regression in {}: {}", 
                                     regression.benchmark_name, 
                                     regression.description);
                    }
                    return Ok(false);
                }
            }

            // Compare against baseline if provided
            if let Some(baseline_path) = baseline_path {
                if baseline_path.exists() {
                    let baseline = runner.load_baseline(baseline_path)?;
                    // Perform real baseline comparison
                    if let Some(ref comparator) = self.baseline_comparator {
                        let comparison_result = comparator.compare_against_baseline(&results, &baseline)?;
                        
                        if comparison_result.has_regressions {
                            tracing::error!("Baseline comparison detected regressions!");
                            tracing::error!("{}", comparison_result.summary);
                            return Ok(false);
                        } else if comparison_result.has_improvements {
                            tracing::info!("Baseline comparison detected improvements!");
                            tracing::info!("{}", comparison_result.summary);
                        } else {
                            tracing::info!("Performance is stable compared to baseline");
                        }
                    }
                }
            }

            Ok(true)
        } else {
            Ok(true) // No benchmarking configured, assume OK
        }
    }

    /// Generate optimization recommendations based on code analysis
    pub fn generate_recommendations(&self, source_code: &str) -> Vec<OptimizationRecommendation> {
        // Use the intelligent recommendations system
        match self.generate_intelligent_recommendations(source_code) {
            Ok(detailed_recommendations) => {
                // Convert detailed recommendations to basic format for backward compatibility
                detailed_recommendations.into_iter().map(|detailed| {
                    OptimizationRecommendation {
                        category: detailed.category,
                        priority: detailed.priority,
                        description: detailed.description,
                        suggested_config: detailed.suggested_config,
                    }
                }).collect()
            }
            Err(e) => {
                tracing::warn!("Failed to generate intelligent recommendations: {}", e);
                // Fallback to basic recommendations
                vec![
                    OptimizationRecommendation {
                        category: OptimizationCategory::Performance,
                        priority: RecommendationPriority::High,
                        description: "Enable aggressive optimization for production builds".to_string(),
                        suggested_config: OptimizationConfig::release_config(),
                    },
                    OptimizationRecommendation {
                        category: OptimizationCategory::CompileTime,
                        priority: RecommendationPriority::Medium,
                        description: "Use development optimization for faster iteration".to_string(),
                        suggested_config: OptimizationConfig::dev_config(),
                    },
                ]
            }
        }
    }

    /// Generate detailed intelligent recommendations with comprehensive analysis
    pub fn generate_intelligent_recommendations(&self, source_code: &str) -> Result<Vec<intelligent_recommendations::DetailedRecommendation>> {
        let mut analysis_engine = intelligent_recommendations::CodeAnalysisEngine::new();
        analysis_engine.analyze_code(source_code)
    }

    /// Generate recommendations with custom analysis configuration
    pub fn generate_recommendations_with_config(
        &self, 
        source_code: &str, 
        analysis_config: intelligent_recommendations::AnalysisConfig
    ) -> Result<Vec<intelligent_recommendations::DetailedRecommendation>> {
        let mut analysis_engine = intelligent_recommendations::CodeAnalysisEngine::with_config(analysis_config);
        analysis_engine.analyze_code(source_code)
    }

    /// Create a baseline from current benchmark results
    pub fn create_baseline(
        &self,
        version: String,
        commit_hash: Option<String>,
        notes: Option<String>,
    ) -> Result<Option<BaselineData>> {
        if let (Some(ref runner), Some(ref comparator)) = (&self.benchmark_runner, &self.baseline_comparator) {
            let configs = create_default_benchmarks();
            let results = runner.run_benchmark_suite("baseline_creation", &configs)?;
            
            let metadata = BaselineMetadata {
                commit_hash,
                environment: BaselineComparator::get_current_environment(),
                compiler_config: format!("{:?}", self.default_config),
                notes,
            };
            
            let baseline = comparator.create_baseline(&results, version, metadata)?;
            Ok(Some(baseline))
        } else {
            Ok(None)
        }
    }

    /// Get time savings analysis for recent compilation
    pub fn get_time_savings_analysis(&self) -> Option<TrendAnalysis> {
        self.time_savings_calculator.get_trend_analysis()
    }

    /// Start timing measurement for compilation
    pub fn start_timing_measurement(&mut self) -> CompilationTimingContext {
        self.time_savings_calculator.start_measurement()
    }

    /// Calculate time savings from compilation timing
    pub fn calculate_time_savings(
        &mut self,
        context: &CompilationTimingContext,
        units_compiled: usize,
        units_from_cache: usize,
        units_from_incremental: usize,
        parallel_efficiency: f64,
    ) -> Result<TimeSavingsAnalysis> {
        self.time_savings_calculator.calculate_time_savings(
            context,
            units_compiled,
            units_from_cache,
            units_from_incremental,
            parallel_efficiency,
        )
    }
}

impl Default for OptimizationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Optimization recommendation generated by analysis
#[derive(Debug, Clone)]
pub struct OptimizationRecommendation {
    /// Category of optimization
    pub category: OptimizationCategory,
    /// Priority level
    pub priority: RecommendationPriority,
    /// Human-readable description
    pub description: String,
    /// Suggested configuration to apply
    pub suggested_config: OptimizationConfig,
}

/// Category of optimization recommendation
#[derive(Debug, Clone)]
pub enum OptimizationCategory {
    Performance,
    CompileTime,
    BinarySize,
    MemoryUsage,
    Debugging,
}

/// Priority level for recommendations
#[derive(Debug, Clone)]
pub enum RecommendationPriority {
    Critical,
    High,
    Medium,
    Low,
}
