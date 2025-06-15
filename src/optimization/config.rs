/// Optimization Configuration System
/// 
/// Provides comprehensive configuration for all optimization features including
/// LLVM passes, parallel compilation, caching, and performance monitoring.

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use toml;

/// Main optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationConfig {
    // General settings
    pub optimization_level: OptimizationLevel,
    pub debug_mode: bool,
    pub profile_guided: bool,
    
    // Parallel compilation
    pub parallel_workers: usize,
    pub enable_parallel: bool,
    
    // Incremental compilation
    pub enable_incremental: bool,
    pub dependency_tracking: bool,
    pub cache_directory: Option<PathBuf>,
    pub cache_max_size: usize, // MB
    
    // LLVM optimization settings
    pub llvm_passes: LlvmPassConfig,
    pub target_cpu: Option<String>,
    pub target_features: Vec<String>,
    
    // Performance monitoring
    pub enable_profiling: bool,
    pub profile_output_dir: Option<PathBuf>,
    pub profile_data_dir: Option<PathBuf>,
    pub benchmark_iterations: usize,
    
    // Analysis and reporting
    pub generate_reports: bool,
    pub report_output_dir: Option<PathBuf>,
    pub verbose_optimization: bool,
    
    // Custom optimization settings
    pub custom_passes: Vec<String>,
    pub optimization_flags: HashMap<String, String>,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            optimization_level: OptimizationLevel::Aggressive,
            debug_mode: false,
            profile_guided: false,
            parallel_workers: num_cpus::get(),
            enable_parallel: true,
            enable_incremental: true,
            dependency_tracking: true,
            cache_directory: None,
            cache_max_size: 1024, // 1GB default
            llvm_passes: LlvmPassConfig::default(),
            target_cpu: None,
            target_features: Vec::new(),
            enable_profiling: true,
            profile_output_dir: None,
            profile_data_dir: None,
            benchmark_iterations: 3,
            generate_reports: false,
            report_output_dir: None,
            verbose_optimization: false,
            custom_passes: Vec::new(),
            optimization_flags: HashMap::new(),
        }
    }
}

/// Optimization levels for CURSED compiler
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptimizationLevel {
    /// No optimization (-O0)
    None,
    /// Minimal optimization (-O1)
    Less,
    /// Standard optimization (-O2)
    Default,
    /// Aggressive optimization (-O3)
    Aggressive,
    /// Optimize for size (-Os)
    Size,
    /// Optimize aggressively for size (-Oz)
    SizeAggressive,
}

impl OptimizationLevel {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "0" | "o0" | "none" => Ok(OptimizationLevel::None),
            "1" | "o1" | "less" => Ok(OptimizationLevel::Less),
            "2" | "o2" | "default" => Ok(OptimizationLevel::Default),
            "3" | "o3" | "aggressive" => Ok(OptimizationLevel::Aggressive),
            "s" | "os" | "size" => Ok(OptimizationLevel::Size),
            "z" | "oz" | "size-aggressive" => Ok(OptimizationLevel::SizeAggressive),
            _ => Err(Error::Other(format!("Invalid optimization level: {}", s))),
        }
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            OptimizationLevel::None => "O0",
            OptimizationLevel::Less => "O1",
            OptimizationLevel::Default => "O2",
            OptimizationLevel::Aggressive => "O3",
            OptimizationLevel::Size => "Os",
            OptimizationLevel::SizeAggressive => "Oz",
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            OptimizationLevel::None => "No optimization (fastest compilation)",
            OptimizationLevel::Less => "Minimal optimization (good for development)",
            OptimizationLevel::Default => "Standard optimization (balanced)",
            OptimizationLevel::Aggressive => "Aggressive optimization (best performance)",
            OptimizationLevel::Size => "Optimize for size",
            OptimizationLevel::SizeAggressive => "Aggressively optimize for size",
        }
    }
}

/// LLVM pass configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlvmPassConfig {
    pub function_passes: Vec<String>,
    pub module_passes: Vec<String>,
    pub enable_vectorization: bool,
    pub enable_loop_unrolling: bool,
    pub enable_inlining: bool,
    pub enable_constant_folding: bool,
    pub enable_dead_code_elimination: bool,
    pub enable_common_subexpression_elimination: bool,
    pub enable_tail_call_optimization: bool,
    pub enable_link_time_optimization: bool,
    pub enable_memory_optimization: bool,
}

impl Default for LlvmPassConfig {
    fn default() -> Self {
        Self {
            function_passes: vec![
                "instcombine".to_string(),
                "reassociate".to_string(),
                "gvn".to_string(),
                "simplifycfg".to_string(),
            ],
            module_passes: vec![
                "globalopt".to_string(),
                "globaldce".to_string(),
                "constmerge".to_string(),
            ],
            enable_vectorization: true,
            enable_loop_unrolling: true,
            enable_inlining: true,
            enable_constant_folding: true,
            enable_dead_code_elimination: true,
            enable_common_subexpression_elimination: true,
            enable_tail_call_optimization: true,
            enable_link_time_optimization: true,
            enable_memory_optimization: true,
        }
    }
}

/// Predefined optimization profiles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationProfile {
    Development,
    Release,
    Debug,
    Size,
    Performance,
}

impl OptimizationProfile {
    pub fn to_config(&self) -> OptimizationConfig {
        match self {
            OptimizationProfile::Development => Self::development_config(),
            OptimizationProfile::Release => Self::release_config(),
            OptimizationProfile::Debug => Self::debug_config(),
            OptimizationProfile::Size => Self::size_config(),
            OptimizationProfile::Performance => Self::performance_config(),
        }
    }
    
    fn development_config() -> OptimizationConfig {
        OptimizationConfig {
            optimization_level: OptimizationLevel::Less,
            debug_mode: true,
            enable_parallel: true,
            enable_incremental: true,
            enable_profiling: true,
            llvm_passes: LlvmPassConfig {
                enable_vectorization: false,
                enable_loop_unrolling: false,
                enable_inlining: false,
                enable_link_time_optimization: false,
                ..Default::default()
            },
            ..Default::default()
        }
    }
    
    fn release_config() -> OptimizationConfig {
        OptimizationConfig {
            optimization_level: OptimizationLevel::Aggressive,
            debug_mode: false,
            enable_parallel: true,
            enable_incremental: true,
            enable_profiling: true,
            llvm_passes: LlvmPassConfig {
                enable_vectorization: true,
                enable_loop_unrolling: true,
                enable_inlining: true,
                enable_link_time_optimization: true,
                ..Default::default()
            },
            ..Default::default()
        }
    }
    
    fn debug_config() -> OptimizationConfig {
        OptimizationConfig {
            optimization_level: OptimizationLevel::None,
            debug_mode: true,
            enable_parallel: false,
            enable_incremental: true,
            enable_profiling: true,
            verbose_optimization: true,
            generate_reports: true,
            llvm_passes: LlvmPassConfig {
                enable_vectorization: false,
                enable_loop_unrolling: false,
                enable_inlining: false,
                enable_link_time_optimization: false,
                ..Default::default()
            },
            ..Default::default()
        }
    }
    
    fn size_config() -> OptimizationConfig {
        OptimizationConfig {
            optimization_level: OptimizationLevel::SizeAggressive,
            debug_mode: false,
            enable_parallel: true,
            enable_incremental: true,
            llvm_passes: LlvmPassConfig {
                enable_vectorization: false,
                enable_loop_unrolling: false,
                enable_inlining: true,
                enable_link_time_optimization: true,
                ..Default::default()
            },
            ..Default::default()
        }
    }
    
    fn performance_config() -> OptimizationConfig {
        OptimizationConfig {
            optimization_level: OptimizationLevel::Aggressive,
            debug_mode: false,
            profile_guided: true,
            enable_parallel: true,
            enable_incremental: true,
            enable_profiling: true,
            benchmark_iterations: 5,
            llvm_passes: LlvmPassConfig {
                enable_vectorization: true,
                enable_loop_unrolling: true,
                enable_inlining: true,
                enable_link_time_optimization: true,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl OptimizationConfig {
    /// Load configuration from TOML file
    pub fn from_file(path: &PathBuf) -> Result<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| Error::Other(format!("Failed to read config file: {}", e)))?;
        
        let config: OptimizationConfig = toml::from_str(&content)
            .map_err(|e| Error::Other(format!("Failed to parse config file: {}", e)))?;
        
        Ok(config)
    }
    
    /// Save configuration to TOML file
    pub fn to_file(&self, path: &PathBuf) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| Error::Other(format!("Failed to serialize config: {}", e)))?;
        
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| Error::Other(format!("Failed to create config directory: {}", e)))?;
        }
        
        fs::write(path, content)
            .map_err(|e| Error::Other(format!("Failed to write config file: {}", e)))?;
        
        Ok(())
    }
    
    /// Create configuration from command line arguments
    pub fn from_args(args: &OptimizationArgs) -> Result<Self> {
        let mut config = if let Some(ref profile) = args.profile {
            profile.to_config()
        } else {
            OptimizationConfig::default()
        };
        
        // Override with command line arguments
        if let Some(ref level) = args.optimization_level {
            config.optimization_level = OptimizationLevel::from_str(level)?;
        }
        
        if let Some(workers) = args.parallel_workers {
            config.parallel_workers = workers;
        }
        
        if let Some(parallel) = args.enable_parallel {
            config.enable_parallel = parallel;
        }
        
        if let Some(incremental) = args.enable_incremental {
            config.enable_incremental = incremental;
        }
        
        if let Some(ref cache_dir) = args.cache_directory {
            config.cache_directory = Some(cache_dir.clone());
        }
        
        if let Some(profiling) = args.enable_profiling {
            config.enable_profiling = profiling;
        }
        
        if let Some(verbose) = args.verbose {
            config.verbose_optimization = verbose;
        }
        
        if let Some(ref target_cpu) = args.target_cpu {
            config.target_cpu = Some(target_cpu.clone());
        }
        
        config.target_features.extend(args.target_features.clone());
        
        Ok(config)
    }
    
    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        if self.parallel_workers == 0 {
            return Err(Error::Other("Parallel workers must be greater than 0".to_string()));
        }
        
        if self.cache_max_size < 10 {
            return Err(Error::Other("Cache max size must be at least 10 MB".to_string()));
        }
        
        if self.benchmark_iterations == 0 {
            return Err(Error::Other("Benchmark iterations must be greater than 0".to_string()));
        }
        
        // Validate cache directory if specified
        if let Some(ref cache_dir) = self.cache_directory {
            if let Some(parent) = cache_dir.parent() {
                if !parent.exists() {
                    return Err(Error::Other(format!(
                        "Cache directory parent does not exist: {}", 
                        parent.display()
                    )));
                }
            }
        }
        
        Ok(())
    }
    
    /// Get effective number of workers
    pub fn effective_workers(&self) -> usize {
        if self.enable_parallel {
            self.parallel_workers.max(1)
        } else {
            1
        }
    }
    
    /// Check if optimization is enabled
    pub fn is_optimized(&self) -> bool {
        !matches!(self.optimization_level, OptimizationLevel::None)
    }
    
    /// Get cache directory with fallback
    pub fn cache_dir(&self) -> PathBuf {
        self.cache_directory.clone()
            .unwrap_or_else(|| PathBuf::from(".cursed_cache"))
    }
    
    /// Get profile output directory with fallback
    pub fn profile_dir(&self) -> PathBuf {
        self.profile_output_dir.clone()
            .unwrap_or_else(|| PathBuf::from(".cursed_profiles"))
    }
    
    /// Get report output directory with fallback
    pub fn report_dir(&self) -> PathBuf {
        self.report_output_dir.clone()
            .unwrap_or_else(|| PathBuf::from(".cursed_reports"))
    }
}

/// Command line arguments for optimization
#[derive(Debug, Clone)]
pub struct OptimizationArgs {
    pub profile: Option<OptimizationProfile>,
    pub optimization_level: Option<String>,
    pub parallel_workers: Option<usize>,
    pub enable_parallel: Option<bool>,
    pub enable_incremental: Option<bool>,
    pub cache_directory: Option<PathBuf>,
    pub enable_profiling: Option<bool>,
    pub verbose: Option<bool>,
    pub target_cpu: Option<String>,
    pub target_features: Vec<String>,
}

impl Default for OptimizationArgs {
    fn default() -> Self {
        Self {
            profile: None,
            optimization_level: None,
            parallel_workers: None,
            enable_parallel: None,
            enable_incremental: None,
            cache_directory: None,
            enable_profiling: None,
            verbose: None,
            target_cpu: None,
            target_features: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_optimization_level_conversion() {
        assert_eq!(OptimizationLevel::from_str("O0").unwrap(), OptimizationLevel::None);
        assert_eq!(OptimizationLevel::from_str("o2").unwrap(), OptimizationLevel::Default);
        assert_eq!(OptimizationLevel::from_str("aggressive").unwrap(), OptimizationLevel::Aggressive);
        assert_eq!(OptimizationLevel::from_str("size").unwrap(), OptimizationLevel::Size);
        
        assert!(OptimizationLevel::from_str("invalid").is_err());
    }
    
    #[test]
    fn test_optimization_profiles() {
        let dev_config = OptimizationProfile::Development.to_config();
        assert_eq!(dev_config.optimization_level, OptimizationLevel::Less);
        assert!(dev_config.debug_mode);
        
        let release_config = OptimizationProfile::Release.to_config();
        assert_eq!(release_config.optimization_level, OptimizationLevel::Aggressive);
        assert!(!release_config.debug_mode);
        assert!(release_config.llvm_passes.enable_link_time_optimization);
    }
    
    #[test]
    fn test_config_file_io() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("optimization.toml");
        
        let config = OptimizationConfig {
            optimization_level: OptimizationLevel::Aggressive,
            parallel_workers: 8,
            enable_profiling: true,
            ..Default::default()
        };
        
        // Save config
        config.to_file(&config_path).unwrap();
        assert!(config_path.exists());
        
        // Load config
        let loaded_config = OptimizationConfig::from_file(&config_path).unwrap();
        assert_eq!(loaded_config.optimization_level, OptimizationLevel::Aggressive);
        assert_eq!(loaded_config.parallel_workers, 8);
        assert!(loaded_config.enable_profiling);
    }
    
    #[test]
    fn test_config_validation() {
        let mut config = OptimizationConfig::default();
        assert!(config.validate().is_ok());
        
        config.parallel_workers = 0;
        assert!(config.validate().is_err());
        
        config.parallel_workers = 4;
        config.cache_max_size = 5;
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_config_from_args() {
        let args = OptimizationArgs {
            optimization_level: Some("O3".to_string()),
            parallel_workers: Some(6),
            enable_profiling: Some(true),
            target_cpu: Some("native".to_string()),
            target_features: vec!["sse4.2".to_string(), "avx".to_string()],
            ..Default::default()
        };
        
        let config = OptimizationConfig::from_args(&args).unwrap();
        assert_eq!(config.optimization_level, OptimizationLevel::Aggressive);
        assert_eq!(config.parallel_workers, 6);
        assert!(config.enable_profiling);
        assert_eq!(config.target_cpu, Some("native".to_string()));
        assert_eq!(config.target_features, vec!["sse4.2", "avx"]);
    }
    
    #[test]
    fn test_effective_workers() {
        let mut config = OptimizationConfig::default();
        config.parallel_workers = 4;
        config.enable_parallel = true;
        assert_eq!(config.effective_workers(), 4);
        
        config.enable_parallel = false;
        assert_eq!(config.effective_workers(), 1);
        
        config.enable_parallel = true;
        config.parallel_workers = 0;
        assert_eq!(config.effective_workers(), 1);
    }
}
