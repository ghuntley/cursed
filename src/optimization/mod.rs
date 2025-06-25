//! Optimization modules

pub mod config;
pub mod real_llvm_passes; 
pub mod enhanced_llvm_passes_manager;
pub mod coordinator;
pub mod llvm_passes;
pub mod optimization_levels;
pub mod lto;
pub mod dependency_analyzer;
pub mod baseline_storage;
pub mod regression_analyzer;
pub mod analysis;
pub mod benchmarking;
pub mod advanced_optimization_manager;

// Re-export key types
pub use benchmarking::{BenchmarkResults, BenchmarkStatistics, BenchmarkRunner};
pub use advanced_optimization_manager::{AdvancedOptimizationManager, OptimizationResult};
pub use config::{OptimizationConfig, OptimizationPass};
