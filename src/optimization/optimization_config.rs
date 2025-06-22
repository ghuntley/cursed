//! Optimization configuration and level management

use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

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

impl Hash for OptimizationLevel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl fmt::Display for OptimizationLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptimizationLevel::O0 => write!(f, "none"),
            OptimizationLevel::O1 => write!(f, "basic"),
            OptimizationLevel::O2 => write!(f, "default"),
            OptimizationLevel::O3 => write!(f, "aggressive"),
            OptimizationLevel::Os => write!(f, "size"),
            OptimizationLevel::Fast => write!(f, "fast"),
        }
    }
}

impl FromStr for OptimizationLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "none" | "0" | "o0" => Ok(OptimizationLevel::O0),
            "basic" | "1" | "o1" => Ok(OptimizationLevel::O1),
            "default" | "2" | "o2" => Ok(OptimizationLevel::O2),
            "aggressive" | "3" | "o3" => Ok(OptimizationLevel::O3),
            "size" | "s" | "os" => Ok(OptimizationLevel::Os),
            "fast" | "ofast" => Ok(OptimizationLevel::Fast),
            _ => Err(format!(
                "Invalid optimization level '{}'. Valid values are: none, basic, default, aggressive, size, fast",
                s
            )),
        }
    }
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
            OptimizationLevel::O0 => 0,
            OptimizationLevel::O1 => 1,
            OptimizationLevel::O2 => 2,
            OptimizationLevel::O3 => 3,
            OptimizationLevel::Os => 2,  // -Os maps to O2 with size focus
            OptimizationLevel::Fast => 3,  // -Ofast maps to O3 with fast math
        }
    }

    /// Check if this optimization level focuses on size
    pub fn optimizes_for_size(&self) -> bool {
        matches!(self, OptimizationLevel::Os)
    }

    /// Check if this optimization level enables fast math
    pub fn enables_fast_math(&self) -> bool {
        matches!(self, OptimizationLevel::Fast)
    }

    /// Get recommended parallel compilation threshold for this level
    pub fn parallel_threshold(&self) -> usize {
        match self {
            OptimizationLevel::O0 => 1,      // No parallel for debug builds
            OptimizationLevel::O1 => 4,     // Light parallelization
            OptimizationLevel::O2 => 8,   // Moderate parallelization
            OptimizationLevel::O3 => 16, // Heavy parallelization
            OptimizationLevel::Os => 4,      // Conservative for size
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
            OptimizationLevel::O2 | OptimizationLevel::O3 | OptimizationLevel::Os
        ) && self.debug_info_level != DebugInfoLevel::Full
    }

    /// Get estimated compilation time multiplier
    pub fn compilation_time_multiplier(&self) -> f64 {
        let base_multiplier = match self.optimization_level {
            OptimizationLevel::O0 => 1.0,
            OptimizationLevel::O1 => 1.5,
            OptimizationLevel::O2 => 2.5,
            OptimizationLevel::O3 => 4.0,
            OptimizationLevel::Os => 3.0,
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
        assert_eq!(OptimizationLevel::O0.to_llvm_level(), 0);
        assert_eq!(OptimizationLevel::O1.to_llvm_level(), 1);
        assert_eq!(OptimizationLevel::O2.to_llvm_level(), 2);
        assert_eq!(OptimizationLevel::O3.to_llvm_level(), 3);
    }

    #[test]
    fn test_optimization_config_presets() {
        let dev_config = OptimizationConfig::development();
        assert_eq!(dev_config.optimization_level, OptimizationLevel::O0);
        assert_eq!(dev_config.debug_info_level, DebugInfoLevel::Full);

        let release_config = OptimizationConfig::release();
        assert_eq!(release_config.optimization_level, OptimizationLevel::O3);
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

    #[test]
    fn test_optimization_level_display() {
        assert_eq!(OptimizationLevel::O0.to_string(), "none");
        assert_eq!(OptimizationLevel::O1.to_string(), "basic");
        assert_eq!(OptimizationLevel::O2.to_string(), "default");
        assert_eq!(OptimizationLevel::O3.to_string(), "aggressive");
        assert_eq!(OptimizationLevel::Os.to_string(), "size");
        assert_eq!(OptimizationLevel::Fast.to_string(), "fast");
    }

    #[test]
    fn test_optimization_level_from_str() {
        // Test lowercase variations
        assert_eq!("none".parse::<OptimizationLevel>().unwrap(), OptimizationLevel::O0);
        assert_eq!("basic".parse::<OptimizationLevel>().unwrap(), OptimizationLevel::O1);
        assert_eq!("default".parse::<OptimizationLevel>().unwrap(), OptimizationLevel::O2);
        assert_eq!("aggressive".parse::<OptimizationLevel>().unwrap(), OptimizationLevel::O3);
        assert_eq!("size".parse::<OptimizationLevel>().unwrap(), OptimizationLevel::Os);
        assert_eq!("fast".parse::<OptimizationLevel>().unwrap(), OptimizationLevel::Fast);

        // Test numeric variations
        assert_eq!("0".parse::<OptimizationLevel>().unwrap(), OptimizationLevel::O0);
        assert_eq!("1".parse::<OptimizationLevel>().unwrap(), OptimizationLevel::O1);
        assert_eq!("2".parse::<OptimizationLevel>().unwrap(), OptimizationLevel::O2);
        assert_eq!("3".parse::<OptimizationLevel>().unwrap(), OptimizationLevel::O3);

        // Test compiler flag variations
        assert_eq!("o0".parse::<OptimizationLevel>().unwrap(), OptimizationLevel::O0);
        assert_eq!("o1".parse::<OptimizationLevel>().unwrap(), OptimizationLevel::O1);
        assert_eq!("o2".parse::<OptimizationLevel>().unwrap(), OptimizationLevel::O2);
        assert_eq!("o3".parse::<OptimizationLevel>().unwrap(), OptimizationLevel::O3);
        assert_eq!("os".parse::<OptimizationLevel>().unwrap(), OptimizationLevel::Os);
        assert_eq!("ofast".parse::<OptimizationLevel>().unwrap(), OptimizationLevel::Fast);

        // Test short variations
        assert_eq!("s".parse::<OptimizationLevel>().unwrap(), OptimizationLevel::Os);

        // Test case insensitivity
        assert_eq!("NONE".parse::<OptimizationLevel>().unwrap(), OptimizationLevel::O0);
        assert_eq!("Basic".parse::<OptimizationLevel>().unwrap(), OptimizationLevel::O1);
        assert_eq!("DEFAULT".parse::<OptimizationLevel>().unwrap(), OptimizationLevel::O2);

        // Test invalid values
        assert!("invalid".parse::<OptimizationLevel>().is_err());
        assert!("4".parse::<OptimizationLevel>().is_err());
        assert!("".parse::<OptimizationLevel>().is_err());
    }

    #[test]
    fn test_optimization_level_hash() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        map.insert(OptimizationLevel::O0, "none");
        map.insert(OptimizationLevel::O1, "basic");
        map.insert(OptimizationLevel::O2, "default");
        map.insert(OptimizationLevel::O3, "aggressive");
        map.insert(OptimizationLevel::Os, "size");
        map.insert(OptimizationLevel::Fast, "fast");

        // Test that all values are stored and retrievable
        assert_eq!(map.get(&OptimizationLevel::O0), Some(&"none"));
        assert_eq!(map.get(&OptimizationLevel::O1), Some(&"basic"));
        assert_eq!(map.get(&OptimizationLevel::O2), Some(&"default"));
        assert_eq!(map.get(&OptimizationLevel::O3), Some(&"aggressive"));
        assert_eq!(map.get(&OptimizationLevel::Os), Some(&"size"));
        assert_eq!(map.get(&OptimizationLevel::Fast), Some(&"fast"));

        // Test that we have exactly 6 entries
        assert_eq!(map.len(), 6);
    }

    #[test]
    fn test_optimization_level_serialization() {
        // Test JSON serialization/deserialization
        let level = OptimizationLevel::O3;
        let json = serde_json::to_string(&level).unwrap();
        let deserialized: OptimizationLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(level, deserialized);

        // Test all variants
        let levels = vec![
            OptimizationLevel::O0,
            OptimizationLevel::O1,
            OptimizationLevel::O2,
            OptimizationLevel::O3,
            OptimizationLevel::Os,
            OptimizationLevel::Fast,
        ];

        for level in levels {
            let json = serde_json::to_string(&level).unwrap();
            let deserialized: OptimizationLevel = serde_json::from_str(&json).unwrap();
            assert_eq!(level, deserialized);
        }
    }

    #[test]
    fn test_optimization_level_round_trip() {
        // Test that Display and FromStr are consistent
        let levels = vec![
            OptimizationLevel::O0,
            OptimizationLevel::O1,
            OptimizationLevel::O2,
            OptimizationLevel::O3,
            OptimizationLevel::Os,
            OptimizationLevel::Fast,
        ];

        for level in levels {
            let string_repr = level.to_string();
            let parsed_level = string_repr.parse::<OptimizationLevel>().unwrap();
            assert_eq!(level, parsed_level);
        }
    }

    #[test]
    fn test_optimization_level_default() {
        assert_eq!(OptimizationLevel::default(), OptimizationLevel::O2);
    }

    #[test]
    fn test_optimization_level_clone_and_debug() {
        let level = OptimizationLevel::O3;
        let cloned = level.clone();
        assert_eq!(level, cloned);
        
        // Test that Debug implementation works
        let debug_string = format!("{:?}", level);
        assert!(debug_string.contains("Aggressive"));
    }

    #[test]
    fn test_optimization_level_equality() {
        assert_eq!(OptimizationLevel::O0, OptimizationLevel::O0);
        assert_ne!(OptimizationLevel::O0, OptimizationLevel::O1);
        
        // Test with cloned values
        let level1 = OptimizationLevel::O2;
        let level2 = level1.clone();
        assert_eq!(level1, level2);
    }

    #[test]
    fn test_debug_info_level_display() {
        assert_eq!(DebugInfoLevel::None.to_string(), "none");
        assert_eq!(DebugInfoLevel::LineTablesOnly.to_string(), "line-tables");
        assert_eq!(DebugInfoLevel::Limited.to_string(), "limited");
        assert_eq!(DebugInfoLevel::Full.to_string(), "full");
    }

    #[test]
    fn test_debug_info_level_from_str() {
        // Test standard names
        assert_eq!("none".parse::<DebugInfoLevel>().unwrap(), DebugInfoLevel::None);
        assert_eq!("line-tables".parse::<DebugInfoLevel>().unwrap(), DebugInfoLevel::LineTablesOnly);
        assert_eq!("limited".parse::<DebugInfoLevel>().unwrap(), DebugInfoLevel::Limited);
        assert_eq!("full".parse::<DebugInfoLevel>().unwrap(), DebugInfoLevel::Full);

        // Test variations
        assert_eq!("line_tables".parse::<DebugInfoLevel>().unwrap(), DebugInfoLevel::LineTablesOnly);
        assert_eq!("linetables".parse::<DebugInfoLevel>().unwrap(), DebugInfoLevel::LineTablesOnly);

        // Test numeric variations
        assert_eq!("0".parse::<DebugInfoLevel>().unwrap(), DebugInfoLevel::None);
        assert_eq!("1".parse::<DebugInfoLevel>().unwrap(), DebugInfoLevel::LineTablesOnly);
        assert_eq!("2".parse::<DebugInfoLevel>().unwrap(), DebugInfoLevel::Limited);
        assert_eq!("3".parse::<DebugInfoLevel>().unwrap(), DebugInfoLevel::Full);

        // Test case insensitivity
        assert_eq!("NONE".parse::<DebugInfoLevel>().unwrap(), DebugInfoLevel::None);
        assert_eq!("Full".parse::<DebugInfoLevel>().unwrap(), DebugInfoLevel::Full);

        // Test invalid values
        assert!("invalid".parse::<DebugInfoLevel>().is_err());
        assert!("4".parse::<DebugInfoLevel>().is_err());
    }

    #[test]
    fn test_debug_info_level_hash() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        map.insert(DebugInfoLevel::None, "none");
        map.insert(DebugInfoLevel::LineTablesOnly, "line-tables");
        map.insert(DebugInfoLevel::Limited, "limited");
        map.insert(DebugInfoLevel::Full, "full");

        assert_eq!(map.len(), 4);
        assert_eq!(map.get(&DebugInfoLevel::None), Some(&"none"));
        assert_eq!(map.get(&DebugInfoLevel::Full), Some(&"full"));
    }

    #[test]
    fn test_debug_info_level_default() {
        assert_eq!(DebugInfoLevel::default(), DebugInfoLevel::Limited);
    }

    #[test]
    fn test_debug_info_level_round_trip() {
        let levels = vec![
            DebugInfoLevel::None,
            DebugInfoLevel::LineTablesOnly,
            DebugInfoLevel::Limited,
            DebugInfoLevel::Full,
        ];

        for level in levels {
            let string_repr = level.to_string();
            let parsed_level = string_repr.parse::<DebugInfoLevel>().unwrap();
            assert_eq!(level, parsed_level);
        }
    }
}
