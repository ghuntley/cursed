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

pub mod config;
pub mod metrics;
pub mod compilation_speed;
// pub mod pgo;  // Using existing pgo directory instead
pub mod performance_system;
pub mod baseline_storage;
pub mod benchmarks;
pub mod intelligent_recommendations;
pub mod regression_analyzer;
pub mod ast_analyzer;
pub mod comprehensive_performance_system;
pub mod real_llvm_passes;
pub mod advanced_function_inlining;
pub mod enhanced_llvm_passes;
pub mod enhanced_llvm_passes_manager;
pub mod enhanced_llvm_optimization;
pub mod interprocedural_analysis;
pub mod memory_layout_optimization;
pub mod performance_analysis;
pub mod parallel_pass_manager;
pub mod baseline_comparison;
pub mod time_savings;
pub mod coordinator;
pub mod pgo;
pub mod performance_optimization_system;
pub mod build_profiles;
pub mod benchmarking_types;
pub mod enablement_system;
pub mod configuration_manager;
pub mod dependency_analyzer;
pub mod benchmarking;
pub mod profiling;
pub mod compiler_passes;

// Missing critical modules causing E0433 errors
pub mod llvm_advanced;
pub mod incremental;
pub mod optimization_manager;
pub mod cache_manager;
pub mod adaptive;
pub mod memory_optimization;
pub mod build_optimization;
pub mod parallel_compilation;
pub mod profiler;
pub mod runtime_optimizations;
pub mod real_performance_analyzer;
pub mod real_compilation_profiler;
// pub mod analysis; // Moved to alias section below
pub mod enhanced_benchmarking;

pub use baseline_storage::{
    BaselineStorage, BaselineStorageConfig, PerformanceBaseline, BaselineType,
    BaselineBenchmark, TimeMetrics,
};
pub use benchmarks::{
    BenchmarkRunner, BenchmarkConfig, BenchmarkResult, BenchmarkSuiteResult,
    PerformanceThresholds, RegressionAnalysis, create_default_benchmarks,
};
pub use regression_analyzer::{
    RegressionAnalyzer, RegressionAnalysisConfig, DetailedRegressionAnalysis,
    RegressionRecommendation, EffortLevel, ImpactLevel,
};
pub use intelligent_recommendations::{
    CodeAnalysisEngine, AnalysisConfig, DetailedRecommendation, AnalysisPattern,
    PatternType, PatternSeverity, PerformanceImpact, OptimizationAction,
    ActionType, ActionPriority, ConfigChange, CodeSuggestion,
};
pub use comprehensive_performance_system::{
    ComprehensivePerformanceSystem, PerformanceConfig, OptimizationResults,
    LlvmOptimizationResults, PgoOptimizationResults, RuntimePerformanceMetrics,
    PerformanceStatistics as SystemPerformanceStatistics, BenchmarkResults,
};
pub use real_llvm_passes::{
    RealLlvmOptimizer, OptimizationResults as RealOptimizationResults, PerformanceImprovements, ModuleMetrics,
    OptimizationStatistics, IntelligentInliner, AdvancedDeadCodeEliminator as RealAdvancedDeadCodeEliminator,
    EnhancedLoopOptimizer, RealConstantPropagator,
};
pub use advanced_function_inlining::{
    AdvancedFunctionInliner, InliningStatistics, FunctionMetrics, CallSiteAnalysis,
    CallGraph, InlineDecision, InlineType,
};
pub use enhanced_llvm_passes_manager::{
    EnhancedLlvmPassManager, EnhancedOptimizationStatistics, IntelligentFunctionInliner,
    AdvancedDeadCodeEliminator, EnhancedConstantPropagator, AdvancedLoopOptimizer as EnhancedAdvancedLoopOptimizer,
    ControlFlowGraphSimplifier, PerformanceAnalyzer, ModuleAnalysis, PerformanceImprovements as EnhancedPerformanceImprovements,
};
pub use enhanced_llvm_optimization::{
    EnhancedLlvmOptimizationSystem, EnhancedOptimizationResults, ModuleCharacteristics,
    ComprehensivePerformanceImprovements, PerformanceResult, PerformanceMonitoringResults,
    RegressionAnalysis as EnhancedRegressionAnalysis,
};
pub use interprocedural_analysis::{
    InterproceduralAnalyzer, CallGraph as InterproceduralCallGraph, FunctionInfo, CallSite, CallType,
    InferredAttributes, MemoryEffects, ReturnDependency, OptimizationOpportunity,
    OpportunityType, InterproceduralResults, OptimizationResults as InterproceduralOptResults,
    PerformanceImprovements as InterproceduralPerfImprovements, InterproceduralStatistics,
};
pub use memory_layout_optimization::{
    MemoryLayoutOptimizer, StructLayoutAnalyzer, StackLayoutOptimizer, AlignmentOptimizer,
    NumaOptimizer, MemoryOptimizationResults, StructOptimizationResults, StackOptimizationResults,
    AlignmentOptimizationResults, NumaOptimizationResults, StructLayoutOptimization, LayoutMetrics,
    StackAnalysis, StackAllocation, StackOptimizationPlan, AlignmentOptimization, AlignmentTarget,
    NumaAnalysis, NumaAllocationPattern, NumaAccessPattern, NumaOptimization, NumaPolicy,
    MemoryOptimizationStatistics,
};

// Re-export performance analysis types with namespace qualification
pub use performance_analysis::{
    PerformanceAnalysisEngine, ComprehensivePerformanceAnalysis, BenchmarkComparisonResults,
    TrendAnalysisResults, BottleneckAnalysisResults, RegressionAnalysisResults,
    OverallPerformanceAssessment, PerformanceAnalysisStatistics,
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
    OptimizationCoordinator, CoordinatedOptimizationResults, RealCacheStatistics,
    RealTimeSavings, OptimizationStrategy, CoordinatorStatistics, CacheBenefits,
    ParallelBenefits, IncrementalBenefits, CoordinationMetadata,
};
pub use pgo::{
    PgoSystem, PgoSystemConfig, PgoSystemStatistics, ProfileData, ProfileAnalysisResult,
    OptimizationOpportunity as PgoOptimizationOpportunity, ProfileInsight, ExecutionContext, PgoError,
    OptimizationAggressiveness, PerformanceMetrics, OptimizationResult,
    InstrumentationMode, CollectionMode,
};
pub use performance_optimization_system::{
    PerformanceOptimizationSystem as ProductionPerformanceOptimizationSystem, SmartCompilationResults,
};
pub use build_profiles::{
    BuildProfile, ProfileManager,
};
pub use enablement_system::{
    OptimizationEnablementSystem, OptimizationEnablementConfig, OptimizationProfile,
    PerformanceMonitoringConfig, PerformanceReportFormat, OptimizationResults as EnablementOptimizationResults,
    PerformanceStatistics as EnablementSystemPerformanceStatistics, LlvmImprovements, PgoImprovements, AdaptiveImprovements,
    TimeSavingsAnalysis as EnablementTimeSavingsAnalysis,
};
pub use configuration_manager::{
    OptimizationConfigManager, ManagedOptimizationConfig, GlobalOptimizationSettings,
    TargetOptimizationConfig,
};

// Critical missing exports causing E0433 errors  
pub use llvm_advanced::{
    AdvancedOptimizationManager, AdvancedOptimizationConfig, OptimizationStatistics as AdvancedOptimizationStatistics,
    OptimizationPipeline, FunctionInliner, LoopOptimizer, DeadCodeEliminator, ConstantPropagator,
    CommonSubexpressionEliminator, TailCallOptimizer, MemoryOptimizer, LoopInfo,
};
pub use incremental::{
    IncrementalCompiler, IncrementalConfig, IncrementalResult, CompilationUnit as IncrementalCompilationUnit,
};
pub use optimization_manager::{
    OptimizationManagerEngine, OptimizationSession, OptimizationTaskConfig,
};
pub use cache_manager::{
    CacheManager, CacheConfig, CacheStatistics, CacheEntry,
};
pub use adaptive::{
    AdaptiveOptimizer, AdaptiveConfig, AdaptiveResults, AdaptiveStrategy,
};
pub use memory_optimization::{
    MemoryOptimizer as ModuleMemoryOptimizer, MemoryOptimizationConfig, MemoryOptimizationResults as ModuleMemoryOptimizationResults,
};
pub use build_optimization::{
    BuildOptimizer, BuildOptimizationConfig, BuildOptimizationResults,
};
pub use parallel_compilation::{
    ParallelCompiler, ParallelCompilationConfig, ParallelCompilationResults,
};
pub use profiler::{
    OptimizationProfiler, ProfilerConfig, ProfilerResults,
};
pub use runtime_optimizations::{
    RuntimeOptimizer, RuntimeOptimizationConfig, RuntimeOptimizationResults,
};
pub use real_performance_analyzer::{
    PerformanceAnalyzer as RealPerformanceAnalyzer, AnalyzerConfig, BottleneckSeverity, AnalysisResult,
};
pub use real_compilation_profiler::{
    CompilationProfiler, ProfileResult, ProfilingConfig,
};
pub use enhanced_benchmarking::{
    EnhancedBenchmarkResult, BenchmarkMetrics,
};
pub use dependency_analyzer::{
    DependencyAnalyzer, CompilationUnit as DependencyCompilationUnit, DependencyAnalysisResult,
};
pub use benchmarking::{
    BenchmarkingSuite, BenchmarkingConfig, Benchmark, BenchmarkResult as BenchmarkingBenchmarkResult, BenchmarkCategory, PerformanceComparison,
};
pub use profiling::{
    ProfilingSystem, ProfilingConfig, ProfilingSession, Profile, ProfileStatistics, Hotspot, CompilationPhaseProfile,
};
pub use compiler_passes::{
    CompilerPassManager, CompilerPass, PassType, PassExecutionResult, PassExecutionConfig, PassExecutionStatistics,
};

// Create analysis and utils modules as aliases
pub mod analysis {
    pub use super::real_performance_analyzer::*;
}

pub mod utils {
    pub use super::intelligent_recommendations::*;
}

// Real optimization implementations
pub use real_optimization_implementation::{
    RealPerformanceCalculator, PerformanceImprovements as RealPerformanceImprovements, BaselineMetrics, 
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
    AdvancedLlvmIntegration, AdvancedLlvmConfig, AdvancedOptimizationStatistics as LlvmAdvancedOptimizationStatistics,
    CfgTransformationStatistics, LoopOptimizationStatistics as AdvancedLoopStats,
    VectorizationStatistics, TargetSpecificStatistics, FunctionComplexity,
};
pub use target_optimization::{
    TargetOptimizationManager, TargetOptimizationConfig as TargetConfig, CpuArchitecture,
    CpuInfo, CpuFeature, SimdCapabilities, OptimizationStrategy as TargetOptimizationStrategy,
    TargetOptimizationStatistics as TargetStats, CodeUnit,
};
pub use advanced_loop_optimization::{
    AdvancedLoopOptimizer, LoopOptimizationConfig as AdvancedLoopConfig,
    LoopOptimizationStatistics, LoopInfo as AdvancedLoopInfo, OptimizationOpportunity as LoopOpportunity,
    CodeUnit as LoopCodeUnit,
};
pub use profile_guided_optimization::{
    ProfileGuidedOptimizer, PgoConfig, PgoOptimizationResult,
    ProfileCollectionMethod, PgoOptimizationLevel, PgoStatistics,
    OptimizationOpportunity as PgoOpportunity, CodeUnit as PgoCodeUnit,
};
pub use link_time_optimization::{
    LinkTimeOptimizer, LtoConfig, LtoOptimizationResult,
    LtoOptimizationLevel, ModuleInfo, FunctionInfo as LtoFunctionInfo, LtoStatistics,
};
pub use advanced_coordinator::{
    AdvancedOptimizationCoordinator, AdvancedCoordinatorConfig, AdvancedOptimizationResult,
    AdvancedOptimizationLevel, OptimizationPhase, AdvancedCoordinatorStatistics, AdvancedCodeUnit,
};

// Comprehensive optimization system exports
pub use comprehensive_optimization_enablement::{
    ComprehensiveOptimizationSystem, ComprehensiveOptimizationConfig, 
    AdaptiveOptimizationLevel, PerformanceMonitor, AdaptiveOptimizationEngine,
    OptimizationResults as ComprehensiveOptimizationResults,
    PerformanceStatistics as ComprehensiveEnablementPerformanceStatistics,
};
pub use cli_optimization_interface::{
    OptimizationCLI,
};

use crate::codegen::llvm::optimization::{OptimizationConfig, OptimizationLevel};
use crate::error::Result;
use std::path::Path;

// Re-export core optimization types for CLI and external usage
pub use crate::codegen::llvm::optimization::{OptimizationConfig, OptimizationLevel};

// Additional optimization types for CLI compatibility
pub type OptimizationEngine = OptimizationManager;
pub type OptimizationPass = String; // Simplified pass representation for CLI

// ML-Guided Optimization System
pub mod ml_optimization;
pub mod ml;

// Re-export ML optimization types
pub use ml_optimization::{
    MLOptimizationEngine, MLOptimizationConfig, ProfilingData,
};
pub use ml::{
    MLOptimizationCoordinator, OptimizationStrategy as MLOptimizationStrategy, OptimizationLevel as MLOptimizationLevel,
    OptimizationPass as MLOptimizationPass, CompilationContext, PerformanceStatistics as MLPerformanceStatistics,
};

// Advanced optimization passes
pub mod alias_analysis;
pub mod sroa;
pub mod gvn;
pub mod tail_call_optimization;
pub mod jump_threading;
pub mod code_motion;

// Re-export advanced optimization types
pub use alias_analysis::{
    AdvancedAliasAnalyzer, AliasAnalysisResults, FunctionAliasAnalysis, AliasSet, AliasType,
    PointerAnalysis, EscapeAnalysis, AliasOptimizationOpportunity, AliasAnalysisStatistics,
};
pub use sroa::{
    SroaOptimizer, SroaOptimizationResults, FunctionSroaResults, AllocationSite, 
    PromotionEligibility, ScalarReplacement, SroaStatistics,
};
pub use gvn::{
    GvnOptimizer, GvnOptimizationResults, FunctionGvnResults, ValueNumbering, Expression,
    GvnOptimization, PhiSimplification, LoadForwardingOpportunity, GvnStatistics,
};
pub use tail_call_optimization::{
    TailCallOptimizer as TailCallOptimizationOptimizer, TailCallOptimizationResults, FunctionTailCallResults, TailCallCandidate,
    TailCallEligibility, TailCallOptimization, TailCallStatistics,
};
pub use jump_threading::{
    JumpThreadingOptimizer, JumpThreadingResults, FunctionJumpThreadingResults, 
    ThreadingOpportunity, ThreadingResult, ThreadingProfitability, JumpThreadingStatistics,
};
pub use code_motion::{
    CodeMotionOptimizer, CodeMotionResults, FunctionCodeMotionResults, MotionOpportunity,
    MotionResult, LicmResult, MotionSafety, CodeMotionStatistics,
};

// Real optimization implementations that replace placeholders
pub mod real_optimization_implementation;
pub mod real_cpu_efficiency_estimator;
pub mod real_regression_detector;
pub mod real_optimization_integration;

// Comprehensive optimization system with all features enabled
pub mod comprehensive_optimization_enablement;
pub mod cli_optimization_interface;

// Advanced optimization modules
pub mod advanced_llvm_integration;
pub mod target_optimization;
pub mod advanced_loop_optimization;
pub mod profile_guided_optimization;
pub mod link_time_optimization;
pub mod advanced_coordinator;
pub mod lto;
pub mod llvm_passes;
pub mod optimization_levels;

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

// New module exports
pub use metrics::{
    CompilationUnit, CompilationStatistics, SystemStatistics, ResourceStatistics,
    MetricsCollector, MetricsSummary
};
pub use compilation_speed::{
    CompilationSpeedOptimizer, CompilationResult, ResourceMonitor
};
// PGO types already exported above in lines 120-124
pub use performance_system::{
    PerformanceSystem, PerformanceSystemConfig, PerformanceStatus,
    PerformanceOptimizationLevel, CompilationStatus
};
