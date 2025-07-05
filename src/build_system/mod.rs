// Build system module with comprehensive multi-file project support
pub mod analytics;
pub mod advanced_cache;
pub mod memory_optimizer;
pub mod incremental_cache;

// Core build system components
pub mod build_pipeline;
pub mod build_orchestrator;
pub mod project_template_simple;

// Re-export main components
pub use build_pipeline::{BuildPipeline, BuildConfig, BuildResult, BuildMode};
pub use build_orchestrator::{BuildOrchestrator, WorkspaceConfig, BuildStrategy};
pub use project_template_simple::{ProjectTemplate, ProjectConfig};
