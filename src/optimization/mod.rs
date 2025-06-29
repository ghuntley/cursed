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

// Re-export key types
pub use config::{OptimizationConfig, OptimizationLevel};
pub use real_llvm_passes::RealLlvmPassManager;
pub use enhanced_llvm_passes_manager::EnhancedLlvmPassManager;
pub use coordinator::{OptimizationCoordinator, CoordinatorConfiguration as CoordinatorConfig};
pub use performance_monitor::PerformanceMonitor;
pub use types::{OptimizationStats, OptimizationResult};
pub use optimization_manager::{
    PerformanceMetrics, AdvancedOptimizationManager, OptimizationManager
};
