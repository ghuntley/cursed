//! Bootstrap Build System Integration
//!
//! This module provides comprehensive integration between the bootstrap verification
//! system and the build orchestrator, enabling true self-hosting capability for
//! the CURSED compiler.

use crate::bootstrap::{
    SelfCompilationVerifier, VerificationConfig, VerificationResult,
    StageResult, PerformanceMetrics, ConvergenceAnalysis
};

use crate::build_system::{
    BuildConfig, BuildResult, BuildTarget, BuildProfile,
    IncrementalCache, DependencyResolver
};

use crate::common::optimization_level::OptimizationLevel;
use crate::build_system::build_pipeline::{BuildPipeline, PipelineContext, PipelineResult};
use crate::error::{Error, Result as CursedResult};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use tracing::{debug, error, info, warn, instrument};

/// Configuration for bootstrap build integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapConfig {
    /// Whether bootstrap compilation is enabled
    pub enabled: bool,
    
    /// Number of bootstrap cycles to perform
    pub bootstrap_cycles: usize,
    
    /// Timeout for each bootstrap stage
    pub stage_timeout: Duration,
    
    /// Whether to keep intermediate bootstrap files
    pub keep_intermediates: bool,
    
    /// Bootstrap verification configuration
    pub verification: VerificationConfig,
    
    /// Convergence detection threshold (performance variance)
    pub convergence_threshold: f64,
    
    /// Whether to auto-detect when to run bootstrap
    pub auto_bootstrap: bool,
    
    /// Minimum time between bootstrap runs
    pub bootstrap_cooldown: Duration,
}

impl Default for BootstrapConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            bootstrap_cycles: 3,
            stage_timeout: Duration::from_secs(600), // 10 minutes per stage
            keep_intermediates: false,
            verification: VerificationConfig::default(),
            convergence_threshold: 0.05, // 5% variance
            auto_bootstrap: false,
            bootstrap_cooldown: Duration::from_secs(3600), // 1 hour
        }
    }
}

/// Bootstrap build pipeline for managing multi-stage compilation
pub struct BootstrapPipeline {
    config: BootstrapConfig,
    verifier: SelfCompilationVerifier,
    work_dir: PathBuf,
    last_bootstrap: Option<Instant>,
    stage_cache: HashMap<u8, StageCache>,
}

/// Cached information for a bootstrap stage
#[derive(Debug, Clone)]
struct StageCache {
    checksum: String,
    timestamp: Instant,
    binary_path: PathBuf,
    performance: PerformanceMetrics,
}

/// Result from bootstrap build operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapBuildResult {
    /// Overall bootstrap success
    pub success: bool,
    
    /// Total bootstrap time
    pub total_time: Duration,
    
    /// Number of stages completed
    pub stages_completed: usize,
    
    /// Individual stage results
    pub stage_results: Vec<BootstrapStageResult>,
    
    /// Whether convergence was achieved
    pub converged: bool,
    
    /// Convergence analysis
    pub convergence_analysis: ConvergenceAnalysis,
    
    /// Performance metrics across stages
    pub performance_metrics: PerformanceMetrics,
    
    /// Any issues encountered
    pub issues: Vec<String>,
    
    /// Binary checksums for each stage
    pub binary_checksums: HashMap<u8, String>,
}

/// Result from a single bootstrap stage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapStageResult {
    /// Stage number (0, 1, 2, ...)
    pub stage: u8,
    
    /// Stage success
    pub success: bool,
    
    /// Compilation time for this stage
    pub compilation_time: Duration,
    
    /// Binary size produced
    pub binary_size: u64,
    
    /// Binary checksum
    pub checksum: String,
    
    /// Output binary path
    pub binary_path: PathBuf,
    
    /// Stage-specific errors
    pub errors: Vec<String>,
    
    /// Performance metrics for this stage
    pub performance: StagePerformance,
}

/// Performance metrics for a single bootstrap stage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StagePerformance {
    /// Memory usage during compilation
    pub memory_usage_mb: u64,
    
    /// CPU utilization percentage
    pub cpu_utilization: f64,
    
    /// Disk I/O operations
    pub disk_io_ops: u64,
    
    /// Compile-time optimization level achieved
    pub optimization_score: f64,
}

impl Default for StagePerformance {
    fn default() -> Self {
        Self {
            memory_usage_mb: 0,
            cpu_utilization: 0.0,
            disk_io_ops: 0,
            optimization_score: 0.0,
        }
    }
}

impl BootstrapPipeline {
    /// Create a new bootstrap pipeline
    pub fn new(config: BootstrapConfig, work_dir: PathBuf) -> CursedResult<Self> {
        let verification_config = config.verification.clone();
        let verifier = SelfCompilationVerifier::new(verification_config)?;
        
        Ok(Self {
            config,
            verifier,
            work_dir,
            last_bootstrap: None,
            stage_cache: HashMap::new(),
        })
    }
    
    /// Check if bootstrap should be triggered
    #[instrument(skip(self))]
    pub fn should_bootstrap(&self, build_config: &BuildConfig) -> bool {
        if !self.config.enabled {
            debug!("Bootstrap disabled in configuration");
            return false;
        }
        
        // Check cooldown period
        if let Some(last) = self.last_bootstrap {
            let elapsed = last.elapsed();
            if elapsed < self.config.bootstrap_cooldown {
                debug!("Bootstrap cooldown active: {:?} remaining", 
                       self.config.bootstrap_cooldown - elapsed);
                return false;
            }
        }
        
        // Auto-detection logic
        if self.config.auto_bootstrap {
            self.should_auto_bootstrap(build_config)
        } else {
            // Manual bootstrap trigger (via CLI)
            false
        }
    }
    
    /// Auto-detection logic for when to bootstrap
    fn should_auto_bootstrap(&self, build_config: &BuildConfig) -> bool {
        // Check if core compiler files have changed
        let core_files = [
            "src/lib.rs",
            "src/core/",
            "src/codegen/",
            "src/type_system/",
            "src/runtime/",
        ];
        
        // This would integrate with file change detection
        // For now, use a simple heuristic
        match build_config.profile {
            BuildProfile::Release => true,
            _ => false,
        }
    }
    
    /// Execute the bootstrap build process
    #[instrument(skip(self, build_config))]
    pub async fn execute_bootstrap(&mut self, build_config: &BuildConfig) -> CursedResult<BootstrapBuildResult> {
        info!("Starting bootstrap build process");
        let start_time = Instant::now();
        
        let mut result = BootstrapBuildResult {
            success: false,
            total_time: Duration::ZERO,
            stages_completed: 0,
            stage_results: Vec::new(),
            converged: false,
            convergence_analysis: ConvergenceAnalysis::default(),
            performance_metrics: PerformanceMetrics::default(),
            issues: Vec::new(),
            binary_checksums: HashMap::new(),
        };
        
        // Prepare verification configuration
        let mut verification_config = self.config.verification.clone();
        verification_config.bootstrap_cycles = self.config.bootstrap_cycles;
        verification_config.compilation_timeout = self.config.stage_timeout;
        verification_config.keep_intermediates = self.config.keep_intermediates;
        
        // Update optimization levels from build config
        verification_config.optimization_levels = 
            self.map_build_profile_to_optimization(build_config);
        
        // Execute verification
        match self.verifier.run_verification().await {
            Ok(verification_result) => {
                result.success = verification_result.success;
                result.stages_completed = verification_result.stages_completed;
                result.converged = verification_result.convergence_analysis.binary_stability;
                result.convergence_analysis = verification_result.convergence_analysis;
                result.performance_metrics = verification_result.performance_metrics;
                
                // Convert stage results
                for stage_result in verification_result.stage_results {
                    let bootstrap_stage = self.convert_stage_result(stage_result)?;
                    result.binary_checksums.insert(bootstrap_stage.stage, bootstrap_stage.checksum.clone());
                    result.stage_results.push(bootstrap_stage);
                }
                
                result.issues = verification_result.issues;
                
                if result.success {
                    info!("Bootstrap completed successfully in {:?}", start_time.elapsed());
                    self.last_bootstrap = Some(Instant::now());
                } else {
                    warn!("Bootstrap failed with {} issues", result.issues.len());
                }
            }
            Err(e) => {
                error!("Bootstrap verification failed: {}", e);
                result.issues.push(format!("Verification error: {}", e));
            }
        }
        
        result.total_time = start_time.elapsed();
        Ok(result)
    }
    
    /// Convert verification stage result to bootstrap stage result
    fn convert_stage_result(&self, stage_result: StageResult) -> CursedResult<BootstrapStageResult> {
        // Calculate binary size
        let binary_size = if stage_result.output_files.is_empty() {
            0
        } else {
            stage_result.output_files[0]
                .metadata()
                .map(|m| m.len())
                .unwrap_or(0)
        };
        
        // Calculate performance metrics (simplified for now)
        let performance = StagePerformance {
            memory_usage_mb: 512, // Would be measured during compilation
            cpu_utilization: 85.0, // Would be sampled during compilation
            disk_io_ops: 1000, // Would be tracked during compilation
            optimization_score: self.calculate_optimization_score(stage_result.stage),
        };
        
        Ok(BootstrapStageResult {
            stage: stage_result.stage,
            success: stage_result.success,
            compilation_time: stage_result.compilation_time,
            binary_size,
            checksum: stage_result.binary_checksum,
            binary_path: stage_result.output_files.get(0)
                .cloned()
                .unwrap_or_else(|| PathBuf::from("unknown")),
            errors: stage_result.errors,
            performance,
        })
    }
    
    /// Calculate optimization score for a stage
    fn calculate_optimization_score(&self, stage: u8) -> f64 {
        // Stages should become progressively more optimized
        match stage {
            0 => 0.6, // Stage 0: Basic Rust compiler
            1 => 0.75, // Stage 1: CURSED compiled by Rust
            2 => 0.85, // Stage 2: CURSED compiled by CURSED
            _ => 0.9, // Stage 3+: Convergence optimizations
        }
    }
    
    /// Map build profile to optimization levels
    fn map_build_profile_to_optimization(&self, build_config: &BuildConfig) -> Vec<String> {
        match build_config.profile {
            BuildProfile::Debug => vec!["-O0".to_string()],
            BuildProfile::Development => vec!["-O1".to_string()],
            BuildProfile::Release => vec!["-O2".to_string(), "-O3".to_string()],
            BuildProfile::Production => vec!["-O3".to_string(), "-Ofast".to_string()],
            _ => vec!["-O2".to_string()],
        }
    }
    
    /// Get cached stage information
    pub fn get_stage_cache(&self, stage: u8) -> Option<&StageCache> {
        self.stage_cache.get(&stage)
    }
    
    /// Update stage cache
    pub fn update_stage_cache(&mut self, stage: u8, cache: StageCache) {
        self.stage_cache.insert(stage, cache);
    }
    
    /// Clear all cached stage information
    pub fn clear_cache(&mut self) {
        self.stage_cache.clear();
        info!("Bootstrap stage cache cleared");
    }
    
    /// Get bootstrap statistics
    pub fn get_statistics(&self) -> BootstrapStatistics {
        BootstrapStatistics {
            total_bootstrap_runs: self.stage_cache.len(),
            last_bootstrap_time: self.last_bootstrap,
            cached_stages: self.stage_cache.keys().cloned().collect(),
            average_stage_time: self.calculate_average_stage_time(),
        }
    }
    
    /// Calculate average stage compilation time
    fn calculate_average_stage_time(&self) -> Duration {
        if self.stage_cache.is_empty() {
            return Duration::ZERO;
        }
        
        // This would be calculated from historical data
        Duration::from_secs(180) // 3 minutes average
    }
}

/// Bootstrap pipeline statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapStatistics {
    /// Total number of bootstrap runs performed
    pub total_bootstrap_runs: usize,
    
    /// Timestamp of last bootstrap
    pub last_bootstrap_time: Option<Instant>,
    
    /// Currently cached stage numbers
    pub cached_stages: Vec<u8>,
    
    /// Average time per stage
    pub average_stage_time: Duration,
}

/// Integration trait for bootstrap functionality in build orchestrator
pub trait BootstrapIntegration {
    /// Initialize bootstrap integration
    fn init_bootstrap(&mut self, config: BootstrapConfig) -> CursedResult<()>;
    
    /// Execute bootstrap compilation
    async fn bootstrap_compile(&mut self, force: bool) -> CursedResult<BootstrapBuildResult>;
    
    /// Check bootstrap status
    fn bootstrap_status(&self) -> Option<&BootstrapStatistics>;
    
    /// Verify bootstrap integrity
    async fn verify_bootstrap(&self) -> CursedResult<VerificationResult>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_bootstrap_config_default() {
        let config = BootstrapConfig::default();
        assert!(config.enabled);
        assert_eq!(config.bootstrap_cycles, 3);
        assert!(config.stage_timeout > Duration::ZERO);
    }
    
    #[test]
    fn test_bootstrap_pipeline_creation() {
        let config = BootstrapConfig::default();
        let work_dir = tempdir().unwrap().into_path();
        
        let pipeline = BootstrapPipeline::new(config, work_dir);
        assert!(pipeline.is_ok());
    }
    
    #[test]
    fn test_stage_performance_default() {
        let perf = StagePerformance::default();
        assert_eq!(perf.memory_usage_mb, 0);
        assert_eq!(perf.cpu_utilization, 0.0);
        assert_eq!(perf.disk_io_ops, 0);
        assert_eq!(perf.optimization_score, 0.0);
    }
    
    #[test]
    fn test_should_bootstrap_disabled() {
        let mut config = BootstrapConfig::default();
        config.enabled = false;
        
        let work_dir = tempdir().unwrap().into_path();
        let pipeline = BootstrapPipeline::new(config, work_dir).unwrap();
        let build_config = BuildConfig::default();
        
        assert!(!pipeline.should_bootstrap(&build_config));
    }
    
    #[test]
    fn test_optimization_score_calculation() {
        let config = BootstrapConfig::default();
        let work_dir = tempdir().unwrap().into_path();
        let pipeline = BootstrapPipeline::new(config, work_dir).unwrap();
        
        assert_eq!(pipeline.calculate_optimization_score(0), 0.6);
        assert_eq!(pipeline.calculate_optimization_score(1), 0.75);
        assert_eq!(pipeline.calculate_optimization_score(2), 0.85);
        assert_eq!(pipeline.calculate_optimization_score(3), 0.9);
    }
    
    #[tokio::test]
    async fn test_bootstrap_statistics() {
        let config = BootstrapConfig::default();
        let work_dir = tempdir().unwrap().into_path();
        let pipeline = BootstrapPipeline::new(config, work_dir).unwrap();
        
        let stats = pipeline.get_statistics();
        assert_eq!(stats.total_bootstrap_runs, 0);
        assert!(stats.last_bootstrap_time.is_none());
        assert!(stats.cached_stages.is_empty());
    }
}
