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
