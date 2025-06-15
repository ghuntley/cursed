//! CURSED Build System
//! 
//! Complete build system integration for the CURSED programming language.
//! Provides project configuration, build orchestration, and toolchain integration.

pub mod build_config;
pub mod build_orchestrator;
pub mod build_pipeline;
pub mod project_template;
pub mod incremental_cache;
pub mod dependency_resolver;
pub mod package_integration;
pub mod file_watcher;
pub mod test_discovery;
pub mod test_executor;

// Advanced Build System Optimizations
pub mod dependency_optimizer;
pub mod advanced_cache;
pub mod distributed_compilation;
pub mod analytics;
pub mod memory_optimizer;
pub mod performance_tracker;

// New Advanced Build System Features
pub mod parallel_compilation;
pub mod incremental_optimization;
pub mod build_profiler;
pub mod artifact_manager;
pub mod lto_integration;

// Re-export main types
pub use build_config::{
    BuildConfig, BuildTarget, BuildProfile, ProjectMetadata, ProjectType,
    TargetType, OptimizationLevel, PanicStrategy, ToolConfigurations,
    FormatterConfig, LinterConfig, DocsConfig, PackageManagerConfig, 
    CompilerConfig, CrossTargetConfig
};
pub use build_orchestrator::{BuildOrchestrator, BuildResult, BuildError, BuildStatistics};
pub use build_pipeline::{
    BuildPipeline, PipelineContext, PipelineResult, PipelineStage, 
    StageResult, PipelineStatistics, ResourceUsage
};
pub use project_template::{ProjectTemplate, TemplateManager, TemplateContext, TemplateCategory};
pub use incremental_cache::{IncrementalCache, CacheEntry, CacheManager};
pub use dependency_resolver::{DependencyResolver, DependencyGraph, VersionConstraintResolver};
pub use package_integration::{
    PackageIntegration, PackageIntegrationConfig, CompilationContext, 
    IntegratedBuildResult, PackageAwareCompiler
};
pub use file_watcher::{
    FileWatcher, WatchConfig, FileWatchEvent, WatchedPath, DebounceManager,
    EventFilter, WatchStatistics, FileWatcherBuilder
};
pub use test_discovery::{
    TestDiscovery, TestDiscoveryConfig, TestDiscoveryResult, TestFunction, 
    TestCategory, TestFilter, TestDiscoveryStatistics
};
pub use test_executor::{
    TestExecutor, TestExecutionConfig, TestExecutionResult, TestResult, 
    TestStatus, TestMetrics, TestExecutionStatistics, TestExecutionSummary,
    TestBatch, TestOutputParser
};

// Re-export advanced build system components
pub use dependency_optimizer::{
    DependencyOptimizer, DependencyOptimizerConfig, AnalysisResult, OptimizationStats
};
pub use advanced_cache::{
    AdvancedCache, AdvancedCacheConfig, CacheEntry, CacheData, CacheStatistics
};
pub use distributed_compilation::{
    DistributedCompilationSystem, DistributedCompilationConfig, CompilationTask, CompilationResult
};
pub use analytics::{
    BuildAnalytics, BuildAnalyticsConfig, BuildReport, BuildMetrics, BottleneckAnalysis
};
pub use memory_optimizer::{
    MemoryOptimizer, MemoryOptimizerConfig, MemoryAwareTask, MemoryStats
};
pub use performance_tracker::{
    BuildPerformanceTracker, PerformanceConfig, BuildPerformanceReport, PerformanceStatistics
};

// Re-export new advanced features
pub use parallel_compilation::{
    ParallelCompiler, ParallelCompilationConfig, ParallelCompilationResult,
    CompilationTask, TaskPriority, SchedulingStrategy, WorkerStatistics
};
pub use incremental_optimization::{
    IncrementalOptimizer, IncrementalConfig, IncrementalBuildPlan,
    DependencyTracker, ChangeDetector, InvalidationEngine, IncrementalStatistics
};
pub use build_profiler::{
    BuildProfiler, ProfilerConfig, ProfilingReport, ProfilingMetrics,
    PerformanceAnalysis, BottleneckAnalysis as ProfilerBottleneckAnalysis,
    OptimizationRecommendation
};
pub use artifact_manager::{
    ArtifactManager, ArtifactConfig, BuildArtifact, ArtifactType,
    VersionManager, CleanupManager, DistributionManager, ArtifactStatistics
};
pub use lto_integration::{
    LtoBuildIntegration, LtoBuildConfig, LtoOptimizationResult, CompilationArtifact,
    LtoBuildStatistics, LtoBuildConfigFactory
};
