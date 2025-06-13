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

// Advanced Build System Optimizations
pub mod dependency_optimizer;
pub mod advanced_cache;
pub mod distributed_compilation;
pub mod analytics;
pub mod memory_optimizer;

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
