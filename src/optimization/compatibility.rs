// Optimization compatibility layer to prevent E0659 conflicts
// 
// This module provides type aliases and re-exports to ensure consistent
// naming across optimization modules and prevent ambiguous imports.

/// Core optimization types with explicit naming
pub use crate::optimization::config::OptimizationConfig as CoreOptimizationConfig;
pub use crate::common_types::optimization_level::OptimizationLevel as CoreOptimizationLevel;

/// Performance analysis types
pub use crate::optimization::performance_analysis::{
    PerformanceAnalyzer as CorePerformanceAnalyzer,
    PerformanceAnalysisEngine,
    ComprehensivePerformanceAnalysis
};

/// Benchmark types
pub use crate::optimization::benchmarks::{
    BenchmarkResult as CoreBenchmarkResult,
    BenchmarkSuiteResult,
    BenchmarkRunner
};

/// Adaptive optimization types
pub use crate::optimization::adaptive::{
    AdaptiveOptimizer,
    AdaptationResult as CoreAdaptationResult,
};
// AdaptiveStrategy comes from optimization_result module, not adaptive
pub use crate::optimization::optimization_result::{
    AdaptiveStrategy as CoreAdaptiveStrategy
};

/// LLVM optimization types
pub use crate::optimization::real_llvm_passes::{
    RealLlvmOptimizer as LlvmOptimizer,
    OptimizationResults as LlvmOptimizationResults,
    PerformanceImprovements as LlvmPerformanceImprovements
};

/// Type aliases for backward compatibility and conflict resolution
pub type DefaultOptimizationConfig = CoreOptimizationConfig;
pub type DefaultOptimizationLevel = CoreOptimizationLevel;
pub type DefaultPerformanceAnalyzer = CorePerformanceAnalyzer;
pub type DefaultBenchmarkResult = CoreBenchmarkResult;
pub type DefaultAdaptationResult = CoreAdaptationResult;
pub type DefaultAdaptiveStrategy = CoreAdaptiveStrategy;

/// Re-export commonly used optimization types for convenience
pub use crate::optimization::{
    OptimizationManager,
    LocalOptimizationCoordinator,
    OptimizationRecommendation
};
