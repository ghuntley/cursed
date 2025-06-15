//! Optimization configuration and level management

use serde::{Deserialize, Serialize};

/// Optimization levels corresponding to standard compiler optimization levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OptimizationLevel {
    /// No optimization (equivalent to -O0)
    None,
    /// Basic optimization (equivalent to -O1)
    Basic,
    /// Default optimization (equivalent to -O2)
    Default,
    /// Aggressive optimization (equivalent to -O3)
    Aggressive,
    /// Size optimization (equivalent to -Os)
    Size,
    /// Fast optimization (equivalent to -Ofast)
    Fast,
}

impl Default for OptimizationLevel {
    fn default() -> Self {
        Self::Default
    }
}

impl OptimizationLevel {
    /// Get the LLVM optimization level equivalent
    pub fn to_llvm_level(&self) -> u32 {
        match self {
            OptimizationLevel::None => 0,
            OptimizationLevel::Basic => 1,
            OptimizationLevel::Default => 2,
            OptimizationLevel::Aggressive => 3,
            OptimizationLevel::Size => 2,  // -Os maps to O2 with size focus
            OptimizationLevel::Fast => 3,  // -Ofast maps to O3 with fast math
        }
    }

    /// Check if this optimization level focuses on size
    pub fn optimizes_for_size(&self) -> bool {
        matches!(self, OptimizationLevel::Size)
    }

    /// Check if this optimization level enables fast math
    pub fn enables_fast_math(&self) -> bool {
        matches!(self, OptimizationLevel::Fast)
    }

    /// Get recommended parallel compilation threshold for this level
    pub fn parallel_threshold(&self) -> usize {
        match self {
            OptimizationLevel::None => 1,      // No parallel for debug builds
            OptimizationLevel::Basic => 4,     // Light parallelization
            OptimizationLevel::Default => 8,   // Moderate parallelization
            OptimizationLevel::Aggressive => 16, // Heavy parallelization
            OptimizationLevel::Size => 4,      // Conservative for size
            OptimizationLevel::Fast => 12,     // Aggressive but not maximum
        }
    }
}

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

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            optimization_level: OptimizationLevel::Default,
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
            optimization_level: OptimizationLevel::None,
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
            optimization_level: OptimizationLevel::Aggressive,
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
            optimization_level: OptimizationLevel::Size,
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
            optimization_level: OptimizationLevel::Basic,
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
                self.optimization_level.parallel_threshold()
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
            OptimizationLevel::Default | OptimizationLevel::Aggressive | OptimizationLevel::Size
        ) && self.debug_info_level != DebugInfoLevel::Full
    }

    /// Get estimated compilation time multiplier
    pub fn compilation_time_multiplier(&self) -> f64 {
        let base_multiplier = match self.optimization_level {
            OptimizationLevel::None => 1.0,
            OptimizationLevel::Basic => 1.5,
            OptimizationLevel::Default => 2.5,
            OptimizationLevel::Aggressive => 4.0,
            OptimizationLevel::Size => 3.0,
            OptimizationLevel::Fast => 3.5,
        };

        let mut multiplier = base_multiplier;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimization_level_llvm_mapping() {
        assert_eq!(OptimizationLevel::None.to_llvm_level(), 0);
        assert_eq!(OptimizationLevel::Basic.to_llvm_level(), 1);
        assert_eq!(OptimizationLevel::Default.to_llvm_level(), 2);
        assert_eq!(OptimizationLevel::Aggressive.to_llvm_level(), 3);
    }

    #[test]
    fn test_optimization_config_presets() {
        let dev_config = OptimizationConfig::development();
        assert_eq!(dev_config.optimization_level, OptimizationLevel::None);
        assert_eq!(dev_config.debug_info_level, DebugInfoLevel::Full);

        let release_config = OptimizationConfig::release();
        assert_eq!(release_config.optimization_level, OptimizationLevel::Aggressive);
        assert!(release_config.enable_lto);
    }

    #[test]
    fn test_config_validation() {
        let mut config = OptimizationConfig::default();
        assert!(config.validate().is_ok());

        config.max_parallel_jobs = Some(0);
        assert!(config.validate().is_err());

        config.max_parallel_jobs = Some(8);
        config.enable_pgo = true;
        assert!(config.validate().is_err()); // Missing profile data file
    }
}
