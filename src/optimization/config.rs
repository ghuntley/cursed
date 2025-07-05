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
    
    // Additional fields for package manager and workspace integration
    pub workspace_dir: String,
    pub max_cache_size: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OptimizationLevel {
    None,        // -O0: No optimizations
    Less,        // -O1: Basic optimizations
    Default,     // -O2: Standard optimizations
    Aggressive,  // -O3: Aggressive optimizations
    Size,        // -Os: Optimize for size
    SizeZ,       // -Oz: Optimize aggressively for size
    SizeAggressive, // Additional size optimization level
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
            OptimizationLevel::SizeAggressive => "Oz", // Same as SizeZ
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
            workspace_dir: std::env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| ".".to_string()),
            max_cache_size: 1024 * 1024 * 1024, // 1GB default
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
            workspace_dir: std::env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| ".".to_string()),
            max_cache_size: 512 * 1024 * 1024, // 512MB for debug
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
            workspace_dir: std::env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| ".".to_string()),
            max_cache_size: 2 * 1024 * 1024 * 1024, // 2GB for release
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
            workspace_dir: std::env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| ".".to_string()),
            max_cache_size: 256 * 1024 * 1024, // 256MB for size optimization
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
            OptimizationLevel::SizeAggressive => flags.push("-Oz".to_string()),
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

    /// Set workspace directory
    pub fn set_workspace_dir(&mut self, dir: String) {
        self.workspace_dir = dir;
    }

    /// Set maximum cache size
    pub fn set_max_cache_size(&mut self, size: usize) {
        self.max_cache_size = size;
    }

    /// Get workspace directory as PathBuf
    pub fn workspace_path(&self) -> std::path::PathBuf {
        std::path::PathBuf::from(&self.workspace_dir)
    }

    /// Check if cache is enabled
    pub fn is_cache_enabled(&self) -> bool {
        self.max_cache_size > 0
    }

    /// Get cache size in MB
    pub fn cache_size_mb(&self) -> usize {
        self.max_cache_size / (1024 * 1024)
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
    /// Alias for release() method for compatibility
    pub fn release_config() -> Self {
        Self::release()
    }

    /// Create configuration optimized for development
    pub fn for_development() -> Self {
        Self {
            level: OptimizationLevel::Less,
            target_features: Vec::new(),
            inline_threshold: 50,
            unroll_threshold: 25,
            vectorize: false,
            parallel_codegen: false,
            lto: false,
            debug_info: true,
            custom_passes: vec!["mem2reg".to_string()],
            pass_manager_config: PassManagerConfig::debug(),
            optimization_level: OptimizationLevel::Less,
            enable_profiling: false,
            profile_data_dir: None,
            profile_guided: false,
            parallel_workers: 2,
            enable_parallel: false,
            enable_incremental: true,
            cache_directory: None,
            generate_reports: true,
            verbose_optimization: true,
            workspace_dir: std::env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| ".".to_string()),
            max_cache_size: 512 * 1024 * 1024, // 512MB for development
        }
    }

    /// Create configuration optimized for production
    pub fn for_production() -> Self {
        Self {
            level: OptimizationLevel::Aggressive,
            target_features: vec!["sse2".to_string(), "sse3".to_string(), "sse4.1".to_string()],
            inline_threshold: 300,
            unroll_threshold: 250,
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
                "loop-unroll".to_string(),
                "sroa".to_string(),
            ],
            pass_manager_config: PassManagerConfig::release(),
            optimization_level: OptimizationLevel::Aggressive,
            enable_profiling: true,
            profile_data_dir: None,
            profile_guided: true,
            parallel_workers: 16,
            enable_parallel: true,
            enable_incremental: true,
            cache_directory: None,
            generate_reports: false,
            verbose_optimization: false,
            workspace_dir: std::env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| ".".to_string()),
            max_cache_size: 4 * 1024 * 1024 * 1024, // 4GB for production
        }
    }

    /// Create configuration based on environment
    pub fn for_environment() -> Self {
        match std::env::var("CURSED_ENV").unwrap_or_else(|_| "development".to_string()).as_str() {
            "production" | "prod" => Self::for_production(),
            "release" => Self::release(),
            "debug" => Self::debug(),
            "size" => Self::size_optimized(),
            _ => Self::for_development(),
        }
    }

    /// Convert to LLVM optimization config
    pub fn to_llvm_config(&self) -> crate::codegen::llvm::optimization::OptimizationConfig {
        let level = match self.level {
            OptimizationLevel::None => 0,
            OptimizationLevel::Less => 1,
            OptimizationLevel::Default => 2,
            OptimizationLevel::Aggressive => 3,
            OptimizationLevel::Size => 2,
            OptimizationLevel::SizeZ => 2,
            OptimizationLevel::SizeAggressive => 2,
            OptimizationLevel::Custom(_) => 2,
        };

        // Convert to LLVM optimization config
        let llvm_level = match self.level {
            OptimizationLevel::None => crate::codegen::llvm::optimization::OptimizationLevel::O0,
            OptimizationLevel::Less => crate::codegen::llvm::optimization::OptimizationLevel::O1,
            OptimizationLevel::Default => crate::codegen::llvm::optimization::OptimizationLevel::O2,
            OptimizationLevel::Aggressive => crate::codegen::llvm::optimization::OptimizationLevel::O3,
            OptimizationLevel::Size => crate::codegen::llvm::optimization::OptimizationLevel::Os,
            OptimizationLevel::SizeZ => crate::codegen::llvm::optimization::OptimizationLevel::Oz,
            OptimizationLevel::SizeAggressive => crate::codegen::llvm::optimization::OptimizationLevel::Oz,
            _ => crate::codegen::llvm::optimization::OptimizationLevel::O2, // Default for other levels
        };
        
        crate::codegen::llvm::optimization::OptimizationConfig {
            level: llvm_level,
            target_cpu: None,
            target_features: Vec::new(),
            vectorize_loops: self.vectorize,
            vectorize_slp: self.vectorize,
            unroll_loops: self.level == OptimizationLevel::Aggressive,
            merge_functions: self.level == OptimizationLevel::Aggressive,
            inline_functions: self.inline_threshold > 0,
            enable_lto: self.level == OptimizationLevel::Aggressive,
            custom_passes: Vec::new(),
            enable_parallel_optimization: true,
            enable_caching: true,
            enable_incremental: false,
            enable_profiling: false,
            cache_size_limit: 1000,
            parallel_threshold: 4,
            optimization_timeout: None,
            enable_cursed_specific: false,
            enable_auto_tuning: false,
            profile_data_path: None,
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

/// Optimization profile combining multiple configuration aspects
#[derive(Debug, Clone)]
pub struct OptimizationProfile {
    pub name: String,
    pub config: OptimizationConfig,
    pub description: String,
    pub use_cases: Vec<String>,
    pub estimated_build_time_factor: f64,
    pub estimated_performance_gain: f64,
}

impl OptimizationProfile {
    /// Create a new optimization profile
    pub fn new(name: String, config: OptimizationConfig) -> Self {
        Self {
            name: name.clone(),
            config,
            description: format!("Optimization profile: {}", name),
            use_cases: Vec::new(),
            estimated_build_time_factor: 1.0,
            estimated_performance_gain: 1.0,
        }
    }

    /// Create development profile
    pub fn development() -> Self {
        Self {
            name: "Development".to_string(),
            config: OptimizationConfig::for_development(),
            description: "Fast compilation with basic optimizations".to_string(),
            use_cases: vec![
                "Development builds".to_string(),
                "Debug builds".to_string(),
                "Rapid iteration".to_string(),
            ],
            estimated_build_time_factor: 0.8,
            estimated_performance_gain: 1.1,
        }
    }

    /// Create production profile
    pub fn production() -> Self {
        Self {
            name: "Production".to_string(),
            config: OptimizationConfig::for_production(),
            description: "Maximum performance optimizations".to_string(),
            use_cases: vec![
                "Production releases".to_string(),
                "Performance-critical applications".to_string(),
                "Final builds".to_string(),
            ],
            estimated_build_time_factor: 1.8,
            estimated_performance_gain: 1.6,
        }
    }

    /// Create size-optimized profile
    pub fn size_optimized() -> Self {
        Self {
            name: "Size".to_string(),
            config: OptimizationConfig::size_optimized(),
            description: "Optimize for binary size".to_string(),
            use_cases: vec![
                "Embedded systems".to_string(),
                "WebAssembly targets".to_string(),
                "Size-constrained environments".to_string(),
            ],
            estimated_build_time_factor: 1.2,
            estimated_performance_gain: 1.0,
        }
    }

    /// Create balanced profile
    pub fn balanced() -> Self {
        Self {
            name: "Balanced".to_string(),
            config: OptimizationConfig::new(OptimizationLevel::Default),
            description: "Balanced compilation time and performance".to_string(),
            use_cases: vec![
                "General purpose builds".to_string(),
                "CI/CD pipelines".to_string(),
                "Default builds".to_string(),
            ],
            estimated_build_time_factor: 1.0,
            estimated_performance_gain: 1.3,
        }
    }

    /// Get all available profiles
    pub fn all_profiles() -> Vec<Self> {
        vec![
            Self::development(),
            Self::balanced(),
            Self::production(),
            Self::size_optimized(),
        ]
    }

    /// Find profile by name
    pub fn by_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "development" | "dev" => Some(Self::development()),
            "production" | "prod" => Some(Self::production()),
            "size" | "min" => Some(Self::size_optimized()),
            "balanced" | "default" => Some(Self::balanced()),
            _ => None,
        }
    }

    /// Add use case to profile
    pub fn add_use_case(&mut self, use_case: String) {
        if !self.use_cases.contains(&use_case) {
            self.use_cases.push(use_case);
        }
    }

    /// Set estimated factors
    pub fn set_estimates(&mut self, build_time_factor: f64, performance_gain: f64) {
        self.estimated_build_time_factor = build_time_factor;
        self.estimated_performance_gain = performance_gain;
    }

    /// Check if profile is suitable for use case
    pub fn is_suitable_for(&self, use_case: &str) -> bool {
        self.use_cases.iter().any(|uc| uc.to_lowercase().contains(&use_case.to_lowercase()))
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
