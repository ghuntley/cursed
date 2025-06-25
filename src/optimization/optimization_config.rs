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
    pub optimization_level: OptimizationLevel,
    
    /// Enable parallel compilation
    pub enable_parallel: bool,
    
    /// Enable incremental compilation
    pub enable_incremental: bool,
    
    /// Enable compilation caching
    pub enable_caching: bool,
    
    /// Enable profile-guided optimization
    pub enable_pgo: bool,
    
    /// Enable link-time optimization
    pub enable_lto: bool,
    
    /// Enable dead code elimination
    pub enable_dce: bool,
    
    /// Enable function inlining
    pub enable_inlining: bool,
    
    /// Enable loop optimizations
    pub enable_loop_optimizations: bool,
    
    /// Enable vectorization
    pub enable_vectorization: bool,
    
    /// Enable target-specific optimizations
    pub enable_target_specific: bool,
    
    /// Enable runtime performance profiling
    pub enable_profiling: bool,
    
    /// Maximum number of parallel compilation jobs
    pub max_parallel_jobs: Option<usize>,
    
    /// Cache directory for compilation artifacts
    pub cache_directory: Option<std::path::PathBuf>,
    
    /// Profile data file for PGO
    pub profile_data_file: Option<std::path::PathBuf>,
    
    /// Custom LLVM passes to apply
    pub custom_passes: Vec<String>,
    
    /// Debug information level
    pub debug_info_level: DebugInfoLevel,
    
    /// Optimization timeout per unit
    pub optimization_timeout_secs: Option<u64>,
}

/// Debug information levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DebugInfoLevel {
    /// No debug information
    None,
    /// Line tables only
    LineTablesOnly,
    /// Limited debug information
    Limited,
    /// Full debug information
    Full,
}

impl Hash for DebugInfoLevel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl fmt::Display for DebugInfoLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DebugInfoLevel::None => write!(f, "none"),
            DebugInfoLevel::LineTablesOnly => write!(f, "line-tables"),
            DebugInfoLevel::Limited => write!(f, "limited"),
            DebugInfoLevel::Full => write!(f, "full"),
        }
    }
}

impl FromStr for DebugInfoLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "none" | "0" => Ok(DebugInfoLevel::None),
            "line-tables" | "line_tables" | "linetables" | "1" => Ok(DebugInfoLevel::LineTablesOnly),
            "limited" | "2" => Ok(DebugInfoLevel::Limited),
            "full" | "3" => Ok(DebugInfoLevel::Full),
            _ => Err(format!(
                "Invalid debug info level '{}'. Valid values are: none, line-tables, limited, full",
                s
            )),
        }
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
            optimization_level: OptimizationLevel::O2,
            enable_parallel: true,
            enable_incremental: true,
            enable_caching: true,
            enable_pgo: false,
            enable_lto: false,
            enable_dce: true,
            enable_inlining: true,
            enable_loop_optimizations: true,
            enable_vectorization: true,
            enable_target_specific: true,
            enable_profiling: false,
            max_parallel_jobs: None,
            cache_directory: None,
            profile_data_file: None,
            custom_passes: Vec::new(),
            debug_info_level: DebugInfoLevel::Limited,
            optimization_timeout_secs: Some(300), // 5 minutes
        }
    }
}

impl OptimizationConfig {
    /// Create configuration for development builds
    pub fn development() -> Self {
        Self {
            optimization_level: OptimizationLevel::O0,
            enable_parallel: false,
            enable_incremental: true,
            enable_caching: true,
            enable_pgo: false,
            enable_lto: false,
            debug_info_level: DebugInfoLevel::Full,
            optimization_timeout_secs: Some(60),
            ..Default::default()
        }
    }

    /// Create configuration for release builds
    pub fn release() -> Self {
        Self {
            optimization_level: OptimizationLevel::O3,
            enable_parallel: true,
            enable_incremental: false,
            enable_caching: true,
            enable_pgo: true,
            enable_lto: true,
            debug_info_level: DebugInfoLevel::Limited,
            optimization_timeout_secs: Some(600), // 10 minutes
            ..Default::default()
        }
    }

    /// Create configuration for size-optimized builds
    pub fn size_optimized() -> Self {
        Self {
            optimization_level: OptimizationLevel::Os,
            enable_parallel: true,
            enable_incremental: true,
            enable_caching: true,
            enable_lto: true,
            enable_dce: true,
            enable_inlining: false, // Inlining can increase size
            enable_vectorization: false, // Vectorization can increase size
            debug_info_level: DebugInfoLevel::None,
            ..Default::default()
        }
    }

    /// Create configuration for fast compilation
    pub fn fast_compilation() -> Self {
        Self {
            optimization_level: OptimizationLevel::O1,
            enable_parallel: true,
            enable_incremental: true,
            enable_caching: true,
            enable_pgo: false,
            enable_lto: false,
            enable_loop_optimizations: false,
            enable_vectorization: false,
            debug_info_level: DebugInfoLevel::LineTablesOnly,
            optimization_timeout_secs: Some(30),
            ..Default::default()
        }
    }

    /// Get effective number of parallel jobs
    pub fn effective_parallel_jobs(&self) -> usize {
        if !self.enable_parallel {
            return 1;
        }

        self.max_parallel_jobs.unwrap_or_else(|| {
            std::cmp::min(
                num_cpus::get(),
                self.optimization_level.recommended_parallel_threads()
            )
        })
    }

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
        }

        Ok(())
    }

    /// Check if this configuration is suitable for production
    pub fn is_production_ready(&self) -> bool {
        matches!(
            self.optimization_level,
            OptimizationLevel::O2 | OptimizationLevel::O3 | OptimizationLevel::Os
        ) && self.debug_info_level != DebugInfoLevel::Full
    }

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
        }

        multiplier
    }
}

