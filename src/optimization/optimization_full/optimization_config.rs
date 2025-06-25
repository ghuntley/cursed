// Optimization configuration and level management

use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

// Use the canonical OptimizationLevel from common module
pub use crate::common_types::optimization_level::OptimizationLevel;

/// Comprehensive optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationConfig {
    /// Primary optimization level
    
    /// Enable parallel compilation
    
    /// Enable incremental compilation
    
    /// Enable compilation caching
    
    /// Enable profile-guided optimization
    
    /// Enable link-time optimization
    
    /// Enable dead code elimination
    
    /// Enable function inlining
    
    /// Enable loop optimizations
    
    /// Enable vectorization
    
    /// Enable target-specific optimizations
    
    /// Enable runtime performance profiling
    
    /// Maximum number of parallel compilation jobs
    
    /// Cache directory for compilation artifacts
    
    /// Profile data file for PGO
    
    /// Custom LLVM passes to apply
    
    /// Debug information level
    
    /// Optimization timeout per unit
/// Debug information levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DebugInfoLevel {
    /// No debug information
    /// Line tables only
    /// Limited debug information
    /// Full debug information
impl Hash for DebugInfoLevel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl fmt::Display for DebugInfoLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        }
    }
impl FromStr for DebugInfoLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            _ => Err(format!(
                s
        }
    }
impl Default for DebugInfoLevel {
    fn default() -> Self {
        Self::Limited
    }
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            optimization_timeout_secs: Some(300), // 5 minutes
        }
    }
impl OptimizationConfig {
    /// Create configuration for development builds
    pub fn development() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Create configuration for release builds
    pub fn release() -> Self {
        Self {
            optimization_timeout_secs: Some(600), // 10 minutes
            ..Default::default()
        }
    }

    /// Create configuration for size-optimized builds
    pub fn size_optimized() -> Self {
        Self {
            enable_inlining: false, // Inlining can increase size
            enable_vectorization: false, // Vectorization can increase size
            ..Default::default()
        }
    }

    /// Create configuration for fast compilation
    pub fn fast_compilation() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Get effective number of parallel jobs
    pub fn effective_parallel_jobs(&self) -> usize {
        if !self.enable_parallel {
            return 1;
        self.max_parallel_jobs.unwrap_or_else(|| {
            std::cmp::min(
                self.optimization_level.recommended_parallel_threads()
            )
        })
    /// Validate configuration settings
    pub fn validate(&self) -> Result<(), String> {
        if let Some(jobs) = self.max_parallel_jobs {
            if jobs == 0 {
                return Err("max_parallel_jobs must be greater than 0".to_string());
            }
            if jobs > 64 {
                return Err("max_parallel_jobs should not exceed 64".to_string());
            }
        }

        if let Some(timeout) = self.optimization_timeout_secs {
            if timeout == 0 {
                return Err("optimization_timeout_secs must be greater than 0".to_string());
            }
            if timeout > 3600 {
                return Err("optimization_timeout_secs should not exceed 1 hour".to_string());
            }
        }

        if self.enable_pgo && self.profile_data_file.is_none() {
            return Err("profile_data_file required when PGO is enabled".to_string());
        Ok(())
    /// Check if this configuration is suitable for production
    pub fn is_production_ready(&self) -> bool {
        matches!(
            OptimizationLevel::O2 | OptimizationLevel::O3 | OptimizationLevel::Os
        ) && self.debug_info_level != DebugInfoLevel::Full
    /// Get estimated compilation time multiplier
    pub fn compilation_time_multiplier(&self) -> f64 {
        let mut multiplier = self.optimization_level.compilation_time_multiplier();

        if self.enable_lto {
            multiplier *= 1.8;
        }
        if self.enable_pgo {
            multiplier *= 1.3;
        }
        if self.enable_parallel && self.effective_parallel_jobs() > 1 {
            multiplier /= (self.effective_parallel_jobs() as f64).sqrt();
        multiplier
    }
}

