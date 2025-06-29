//! Optimization configuration and level management

use crate::error::CursedError;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct OptimizationConfig {
    pub level: OptimizationLevel,
    pub target_features: Vec<String>,
    pub inline_threshold: u32,
    pub unroll_threshold: u32,
    pub vectorize: bool,
    pub parallel_codegen: bool,
    pub lto: bool,
    pub debug_info: bool,
    pub custom_passes: Vec<String>,
    pub pass_manager_config: PassManagerConfig,
    
    // Compatibility fields for examples/tests
    pub optimization_level: OptimizationLevel,
    pub enable_profiling: bool,
    pub profile_data_dir: Option<std::path::PathBuf>,
    pub profile_guided: bool,
    pub parallel_workers: u32,
    pub enable_parallel: bool,
    pub enable_incremental: bool,
    pub cache_directory: Option<std::path::PathBuf>,
    pub generate_reports: bool,
    pub verbose_optimization: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OptimizationLevel {
    None,        // -O0: No optimizations
    Less,        // -O1: Basic optimizations
    Default,     // -O2: Standard optimizations
    Aggressive,  // -O3: Aggressive optimizations
    Size,        // -Os: Optimize for size
    SizeZ,       // -Oz: Optimize aggressively for size
    Custom(HashMap<String, bool>),  // Custom optimization settings
}

impl OptimizationLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            OptimizationLevel::None => "O0",
            OptimizationLevel::Less => "O1",
            OptimizationLevel::Default => "O2", 
            OptimizationLevel::Aggressive => "O3",
            OptimizationLevel::Size => "Os",
            OptimizationLevel::SizeZ => "Oz",
            OptimizationLevel::Custom(_) => "Custom",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PassManagerConfig {
    pub enable_function_passes: bool,
    pub enable_module_passes: bool,
    pub enable_loop_passes: bool,
    pub enable_cgsc_passes: bool,
    pub verification_level: VerificationLevel,
    pub pass_timing: bool,
}

#[derive(Debug, Clone)]
pub enum VerificationLevel {
    None,
    Basic,
    Full,
    Debug,
}

impl OptimizationConfig {
    pub fn new(level: OptimizationLevel) -> Self {
        Self {
            level: level.clone(),
            target_features: Vec::new(),
            inline_threshold: 225,
            unroll_threshold: 150,
            vectorize: true,
            parallel_codegen: true,
            lto: false,
            debug_info: false,
            custom_passes: Vec::new(),
            pass_manager_config: PassManagerConfig::default(),
            optimization_level: level,
            enable_profiling: false,
            profile_data_dir: None,
            profile_guided: false,
            parallel_workers: 4,
            enable_parallel: true,
            enable_incremental: false,
            cache_directory: None,
            generate_reports: false,
            verbose_optimization: false,
        }
    }

    pub fn debug() -> Self {
        Self {
            level: OptimizationLevel::None,
            target_features: Vec::new(),
            inline_threshold: 0,
            unroll_threshold: 0,
            vectorize: false,
            parallel_codegen: false,
            lto: false,
            debug_info: true,
            custom_passes: Vec::new(),
            pass_manager_config: PassManagerConfig::debug(),
            optimization_level: OptimizationLevel::None,
            enable_profiling: false,
            profile_data_dir: None,
            profile_guided: false,
            parallel_workers: 1,
            enable_parallel: false,
            enable_incremental: false,
            cache_directory: None,
            generate_reports: true,
            verbose_optimization: true,
        }
    }

    pub fn release() -> Self {
        Self {
            level: OptimizationLevel::Aggressive,
            target_features: vec!["sse2".to_string(), "sse3".to_string()],
            inline_threshold: 275,
            unroll_threshold: 200,
            vectorize: true,
            parallel_codegen: true,
            lto: true,
            debug_info: false,
            custom_passes: vec![
                "mem2reg".to_string(),
                "instcombine".to_string(),
                "reassociate".to_string(),
                "gvn".to_string(),
                "simplifycfg".to_string(),
            ],
            pass_manager_config: PassManagerConfig::release(),
            optimization_level: OptimizationLevel::Aggressive,
            enable_profiling: true,
            profile_data_dir: None,
            profile_guided: false,
            parallel_workers: 8,
            enable_parallel: true,
            enable_incremental: true,
            cache_directory: None,
            generate_reports: false,
            verbose_optimization: false,
        }
    }

    pub fn size_optimized() -> Self {
        Self {
            level: OptimizationLevel::Size,
            target_features: Vec::new(),
            inline_threshold: 75,
            unroll_threshold: 50,
            vectorize: false,
            parallel_codegen: false,
            lto: true,
            debug_info: false,
            custom_passes: vec![
                "mem2reg".to_string(),
                "simplifycfg".to_string(),
                "deadargelim".to_string(),
            ],
            pass_manager_config: PassManagerConfig::default(),
            optimization_level: OptimizationLevel::Size,
            enable_profiling: false,
            profile_data_dir: None,
            profile_guided: false,
            parallel_workers: 1,
            enable_parallel: false,
            enable_incremental: false,
            cache_directory: None,
            generate_reports: false,
            verbose_optimization: false,
        }
    }

    pub fn add_target_feature(&mut self, feature: String) {
        if !self.target_features.contains(&feature) {
            self.target_features.push(feature);
        }
    }

    pub fn add_custom_pass(&mut self, pass_name: String) {
        if !self.custom_passes.contains(&pass_name) {
            self.custom_passes.push(pass_name);
        }
    }

    pub fn set_inline_threshold(&mut self, threshold: u32) {
        self.inline_threshold = threshold;
    }

    pub fn set_unroll_threshold(&mut self, threshold: u32) {
        self.unroll_threshold = threshold;
    }

    pub fn enable_lto(&mut self) {
        self.lto = true;
    }

    pub fn disable_lto(&mut self) {
        self.lto = false;
    }

    pub fn enable_debug_info(&mut self) {
        self.debug_info = true;
    }

    pub fn disable_debug_info(&mut self) {
        self.debug_info = false;
    }

    pub fn get_optimization_flags(&self) -> Vec<String> {
        let mut flags = Vec::new();

        match self.level {
            OptimizationLevel::None => flags.push("-O0".to_string()),
            OptimizationLevel::Less => flags.push("-O1".to_string()),
            OptimizationLevel::Default => flags.push("-O2".to_string()),
            OptimizationLevel::Aggressive => flags.push("-O3".to_string()),
            OptimizationLevel::Size => flags.push("-Os".to_string()),
            OptimizationLevel::SizeZ => flags.push("-Oz".to_string()),
            OptimizationLevel::Custom(_) => flags.push("-O2".to_string()), // Default fallback
        }

        if self.lto {
            flags.push("-flto".to_string());
        }

        if self.debug_info {
            flags.push("-g".to_string());
        }

        if self.vectorize {
            flags.push("-vectorize-loops".to_string());
            flags.push("-vectorize-slp".to_string());
        }

        for feature in &self.target_features {
            flags.push(format!("-mattr=+{}", feature));
        }

        flags
    }

    pub fn should_inline(&self, function_size: u32) -> bool {
        match self.level {
            OptimizationLevel::None => false,
            OptimizationLevel::Size | OptimizationLevel::SizeZ => function_size <= (self.inline_threshold / 3),
            _ => function_size <= self.inline_threshold,
        }
    }

    pub fn should_unroll(&self, loop_size: u32) -> bool {
        match self.level {
            OptimizationLevel::None => false,
            OptimizationLevel::Size | OptimizationLevel::SizeZ => false,
            _ => loop_size <= self.unroll_threshold,
        }
    }

    pub fn validate(&self) -> Result<(), CursedError> {
        if self.inline_threshold > 10000 {
            return Err(CursedError::runtime_error("Inline threshold too high (max 10000)"));
        }

        if self.unroll_threshold > 1000 {
            return Err(CursedError::runtime_error("Unroll threshold too high (max 1000)"));
        }

        // Validate target features
        for feature in &self.target_features {
            if feature.is_empty() {
                return Err(CursedError::runtime_error("Empty target feature not allowed"));
            }
        }

        // Validate custom passes
        for pass in &self.custom_passes {
            if pass.is_empty() {
                return Err(CursedError::runtime_error("Empty pass name not allowed"));
            }
        }

        Ok(())
    }

    /// Get effective number of workers for parallel optimization
    pub fn effective_workers(&self) -> u32 {
        if self.enable_parallel {
            self.parallel_workers
        } else {
            1
        }
    }

    /// Get cache directory path
    pub fn cache_dir(&self) -> std::path::PathBuf {
        self.cache_directory.clone()
            .unwrap_or_else(|| std::env::temp_dir().join("cursed_cache"))
    }

    /// Create configuration from command line arguments
    pub fn from_args(args: &[String]) -> Result<Self, CursedError> {
        let mut config = Self::new(OptimizationLevel::Default);
        
        for arg in args {
            match arg.as_str() {
                "--debug" | "-g" => config = Self::debug(),
                "--release" | "-O3" => config = Self::release(),
                "--size" | "-Os" => config = Self::size_optimized(),
                _ => {} // Ignore unknown args
            }
        }
        
        Ok(config)
    }
}

impl PassManagerConfig {
    pub fn new() -> Self {
        Self {
            enable_function_passes: true,
            enable_module_passes: true,
            enable_loop_passes: true,
            enable_cgsc_passes: true,
            verification_level: VerificationLevel::Basic,
            pass_timing: false,
        }
    }

    pub fn debug() -> Self {
        Self {
            enable_function_passes: true,
            enable_module_passes: true,
            enable_loop_passes: false,
            enable_cgsc_passes: false,
            verification_level: VerificationLevel::Full,
            pass_timing: true,
        }
    }

    pub fn release() -> Self {
        Self {
            enable_function_passes: true,
            enable_module_passes: true,
            enable_loop_passes: true,
            enable_cgsc_passes: true,
            verification_level: VerificationLevel::Basic,
            pass_timing: false,
        }
    }

    pub fn minimal() -> Self {
        Self {
            enable_function_passes: true,
            enable_module_passes: false,
            enable_loop_passes: false,
            enable_cgsc_passes: false,
            verification_level: VerificationLevel::None,
            pass_timing: false,
        }
    }
}

impl Default for PassManagerConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self::new(OptimizationLevel::Default)
    }
}

impl From<u8> for OptimizationLevel {
    fn from(level: u8) -> Self {
        match level {
            0 => OptimizationLevel::None,
            1 => OptimizationLevel::Less,
            2 => OptimizationLevel::Default,
            3 => OptimizationLevel::Aggressive,
            _ => OptimizationLevel::Default,
        }
    }
}

impl OptimizationConfig {
    /// Convert to LLVM optimization config
    pub fn to_llvm_config(&self) -> crate::codegen::llvm::optimization::OptimizationConfig {
        let level = match self.level {
            OptimizationLevel::None => 0,
            OptimizationLevel::Less => 1,
            OptimizationLevel::Default => 2,
            OptimizationLevel::Aggressive => 3,
            OptimizationLevel::Size => 2,
            OptimizationLevel::SizeZ => 2,
            OptimizationLevel::Custom(_) => 2,
        };

        crate::codegen::llvm::optimization::OptimizationConfig {
            level,
            enable_inlining: self.inline_threshold > 0,
            enable_vectorization: self.vectorize,
        }
    }
}

impl From<&str> for OptimizationLevel {
    fn from(level: &str) -> Self {
        match level.to_lowercase().as_str() {
            "none" | "0" | "o0" => OptimizationLevel::None,
            "less" | "1" | "o1" => OptimizationLevel::Less,
            "default" | "2" | "o2" => OptimizationLevel::Default,
            "aggressive" | "3" | "o3" => OptimizationLevel::Aggressive,
            "size" | "s" | "os" => OptimizationLevel::Size,
            "sizez" | "z" | "oz" => OptimizationLevel::SizeZ,
            _ => OptimizationLevel::Default,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimization_config_creation() {
        let config = OptimizationConfig::debug();
        assert_eq!(config.level, OptimizationLevel::None);
        assert!(config.debug_info);
        assert!(!config.lto);

        let config = OptimizationConfig::release();
        assert_eq!(config.level, OptimizationLevel::Aggressive);
        assert!(!config.debug_info);
        assert!(config.lto);
    }

    #[test]
    fn test_optimization_level_conversion() {
        assert_eq!(OptimizationLevel::from(0u8), OptimizationLevel::None);
        assert_eq!(OptimizationLevel::from(3u8), OptimizationLevel::Aggressive);
        assert_eq!(OptimizationLevel::from("O2"), OptimizationLevel::Default);
        assert_eq!(OptimizationLevel::from("size"), OptimizationLevel::Size);
    }

    #[test]
    fn test_should_inline() {
        let config = OptimizationConfig::debug();
        assert!(!config.should_inline(100));

        let config = OptimizationConfig::release();
        assert!(config.should_inline(100));
        assert!(!config.should_inline(1000));
    }

    #[test]
    fn test_config_validation() {
        let mut config = OptimizationConfig::default();
        assert!(config.validate().is_ok());

        config.inline_threshold = 20000;
        assert!(config.validate().is_err());
    }
}
