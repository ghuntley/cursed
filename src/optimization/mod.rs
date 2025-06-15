//! Performance Optimization System for CURSED
//! 
//! This module provides comprehensive performance optimization capabilities including:
//! - LLVM optimization pass management
//! - Compilation speed improvements through caching and parallelization
//! - Performance analysis and metrics collection
//! - Runtime optimizations for goroutines and GC
//! - Automated benchmarking and profiling

use crate::error::Result;
use std::time::Duration;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

// Core modules
pub mod llvm_optimizer;
pub mod optimization_config;
pub mod pass_manager;
pub mod profile_guided;
pub mod incremental;
pub mod cache_manager;
pub mod parallel_compilation;
pub mod dependency_analyzer;
pub mod profiler;
pub mod metrics;
pub mod benchmarking;
pub mod analysis;
pub mod coordinator;
pub mod build_integration;
pub mod cli_integration;
pub mod jit_optimization;
pub mod build_profiles;
pub mod optimization_levels;
pub mod config;

// PGO modules
pub mod pgo;

// CURSED-specific optimization integration
pub mod cursed_integration;

// Distributed compilation system
pub mod distributed;

// Additional exports for analysis module
pub use analysis::PerformanceAnalysis;

// Re-export main types
pub use optimization_config::{OptimizationConfig, OptimizationLevel};
pub use profiler::{EnhancedBuildProfiler, ProfilerConfig, ReportFormat};
pub use benchmarking::{BenchmarkConfig, BenchmarkType, BenchmarkTestData, ComplexityLevel};
pub use metrics::{ResourceMonitoringLevel, CompilationUnit};
pub use build_profiles::{BuildProfile, ProfileManager};
pub use config::{OptimizationProfile, LlvmPassConfig};
pub use distributed::{DistributedCompilationSystem, DistributedConfig, DistributedStats};

// Enhanced LLVM optimization system
pub mod enhanced_llvm_optimization;
pub use enhanced_llvm_optimization::{
    EnhancedLlvmOptimizer, EnhancedOptimizationConfig, EnhancedOptimizationResults,
    PerformanceImprovements, OptimizationFeedback, TargetOptimizationResults,
};

// Performance Integration System
pub mod performance_integration;
pub use performance_integration::{
    PerformanceIntegrationSystem, PerformanceIntegrationConfig, PerformanceTargets,
    IntegratedOptimizationResults, AdaptiveOptimizer, PerformanceMonitor,
    ProjectCharacteristics, OptimizationRecord, OptimizationRecommendation,
    RecommendationCategory, ImplementationEffort, PerformanceStatistics,
};

// Machine Learning driven optimization system
pub mod ml_optimization;
pub use ml_optimization::{
    MLOptimizationEngine, MLOptimizationConfig, FeatureVector, OptimizationDecision,
    TrainingSample, PerformanceMetrics, CursedOptType, LoopOptType, RegAllocStrategy,
};

/// Main performance optimization system coordinator
#[derive(Debug)]
pub struct PerformanceOptimizationSystem {
    performance_config: PerformanceConfig,
    optimization_config: OptimizationConfig,
    llvm_optimizer: llvm_optimizer::LlvmOptimizer,
    cache_manager: cache_manager::CacheManager,
    profiler: profiler::EnhancedBuildProfiler,
    metrics_collector: metrics::MetricsCollector,
    benchmarking_engine: benchmarking::BenchmarkingEngine,
}

/// Configuration for performance optimization system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub enable_realtime_monitoring: bool,
    pub enable_benchmarking: bool,
    pub enable_prediction: bool,
    pub monitoring_interval_ms: u64,
    pub max_benchmark_iterations: usize,
    pub max_performance_entries: usize,
    pub resource_monitoring_level: ResourceMonitoringLevel,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enable_realtime_monitoring: true,
            enable_benchmarking: true,
            enable_prediction: false,
            monitoring_interval_ms: 100,
            max_benchmark_iterations: 10,
            max_performance_entries: 10000,
            resource_monitoring_level: ResourceMonitoringLevel::Basic,
        }
    }
}

/// Optimization session for tracking related operations
#[derive(Debug, Clone)]
pub struct OptimizationSession {
    pub id: String,
    pub name: String,
    pub created_at: std::time::Instant,
}

/// Results from optimization with tracking
#[derive(Debug)]
pub struct OptimizationResults {
    pub session_id: String,
    pub unit_results: Vec<OptimizationUnitResult>,
    pub total_time: Duration,
    pub performance_analysis: Option<analysis::PerformanceAnalysis>,
}

/// Results for a single compilation unit optimization
#[derive(Debug)]
pub struct OptimizationUnitResult {
    pub unit_name: String,
    pub optimization_time: Duration,
    pub before_size: usize,
    pub after_size: usize,
    pub optimization_level: OptimizationLevel,
}

impl PerformanceOptimizationSystem {
    /// Create a new performance optimization system
    pub fn new(
        performance_config: PerformanceConfig,
        optimization_config: OptimizationConfig,
    ) -> Result<Self> {
        let llvm_optimizer = llvm_optimizer::LlvmOptimizer::new(optimization_config.clone())?;
        let cache_manager = cache_manager::CacheManager::new()?;
        
        let profiler_config = profiler::ProfilerConfig::default();
        let profiler = profiler::EnhancedBuildProfiler::new(profiler_config)?;
        
        let metrics_collector = metrics::MetricsCollector::new(performance_config.clone())?;
        let benchmarking_engine = benchmarking::BenchmarkingEngine::new(performance_config.clone())?;

        Ok(Self {
            performance_config,
            optimization_config,
            llvm_optimizer,
            cache_manager,
            profiler,
            metrics_collector,
            benchmarking_engine,
        })
    }

    /// Start real-time monitoring
    pub fn start_monitoring(&self) -> Result<()> {
        if self.performance_config.enable_realtime_monitoring {
            self.metrics_collector.start_monitoring()
        } else {
            Ok(())
        }
    }

    /// Stop real-time monitoring
    pub fn stop_monitoring(&self) -> Result<()> {
        self.metrics_collector.stop_monitoring()
    }

    /// Create a new optimization session
    pub fn create_session(&self, name: String) -> OptimizationSession {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        
        OptimizationSession {
            id: format!("{}_{}", name, timestamp),
            name,
            created_at: std::time::Instant::now(),
        }
    }

    /// Optimize compilation units with tracking
    pub fn optimize_with_tracking(
        &self,
        units: &mut [CompilationUnit],
        session: &OptimizationSession,
    ) -> Result<OptimizationResults> {
        let start_time = std::time::Instant::now();
        let mut unit_results = Vec::new();

        for unit in units.iter_mut() {
            let unit_start = std::time::Instant::now();
            let before_size = unit.estimated_size_bytes;

            // Perform optimization
            self.llvm_optimizer.optimize_unit(unit)?;
            
            let after_size = unit.estimated_size_bytes;
            let optimization_time = unit_start.elapsed();

            unit_results.push(OptimizationUnitResult {
                unit_name: unit.name.clone(),
                optimization_time,
                before_size,
                after_size,
                optimization_level: self.optimization_config.optimization_level.clone(),
            });
        }

        let total_time = start_time.elapsed();
        
        // Generate performance analysis if enabled
        let performance_analysis = if self.performance_config.enable_prediction {
            Some(self.generate_performance_analysis(&unit_results)?)
        } else {
            None
        };

        Ok(OptimizationResults {
            session_id: session.id.clone(),
            unit_results,
            total_time,
            performance_analysis,
        })
    }

    /// Run a benchmark
    pub fn run_benchmark(&self, config: BenchmarkConfig) -> Result<benchmarking::BenchmarkResults> {
        self.benchmarking_engine.run_benchmark(config)
    }

    /// Get system statistics
    pub fn get_system_statistics(&self) -> metrics::SystemStatistics {
        self.metrics_collector.get_system_statistics()
    }

    /// Get resource statistics
    pub fn get_resource_statistics(&self) -> Result<metrics::ResourceStatistics> {
        self.metrics_collector.get_resource_statistics()
    }

    /// Get performance analysis for a time period
    pub fn get_performance_analysis(&self, duration: Duration) -> Result<analysis::PerformanceAnalysis> {
        self.metrics_collector.get_performance_analysis(duration)
    }

    /// Update system configuration
    pub fn update_config(&mut self, new_config: PerformanceConfig) -> Result<()> {
        self.performance_config = new_config.clone();
        self.metrics_collector.update_config(new_config)?;
        Ok(())
    }

    /// Generate performance analysis from optimization results
    fn generate_performance_analysis(
        &self,
        unit_results: &[OptimizationUnitResult],
    ) -> Result<analysis::PerformanceAnalysis> {
        let total_optimization_time: Duration = unit_results.iter()
            .map(|r| r.optimization_time)
            .sum();

        let total_size_reduction: i64 = unit_results.iter()
            .map(|r| r.before_size as i64 - r.after_size as i64)
            .sum();

        let optimization_efficiency = if total_optimization_time.as_millis() > 0 {
            total_size_reduction as f64 / total_optimization_time.as_millis() as f64
        } else {
            0.0
        };

        Ok(analysis::PerformanceAnalysis {
            units_optimized: unit_results.len(),
            total_optimization_time,
            total_size_reduction,
            optimization_efficiency,
            recommendations: Vec::new(),
        })
    }
}
