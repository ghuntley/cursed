/// Build Profile Optimization Configurations
/// 
/// Provides predefined optimization configurations for different build scenarios
/// with appropriate optimization levels, features, and performance characteristics.

use crate::optimization::config::{OptimizationConfig, LlvmPassConfig};
use crate::common::optimization_level::OptimizationLevel;
use crate::optimization::optimization_levels::{LevelConfig, TargetSpecificSettings};
use std::collections::HashMap;
use std::path::PathBuf;

/// Build profile types for different use cases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildProfile {
    /// Development profile - fast compilation, minimal optimization
    Development,
    /// Debug profile - debugging symbols, no optimization
    Debug,
    /// Release profile - balanced performance and compilation time
    Release,
    /// Production profile - maximum runtime performance
    Production,
    /// Size profile - minimize binary size
    Size,
    /// Testing profile - optimized for test execution
    Testing,
}

impl BuildProfile {
    /// Get optimization configuration for this build profile
    pub fn to_optimization_config(&self) -> OptimizationConfig {
        match self {
            BuildProfile::Development => Self::development_config(),
            BuildProfile::Debug => Self::debug_config(),
            BuildProfile::Release => Self::release_config(),
            BuildProfile::Production => Self::production_config(),
            BuildProfile::Size => Self::size_config(),
            BuildProfile::Testing => Self::testing_config(),
        }
    }
    
    /// Development configuration - fast compilation, basic optimization
    fn development_config() -> OptimizationConfig {
        OptimizationConfig {
            optimization_level: OptimizationLevel::O1,
            debug_mode: true,
            profile_guided: false,
            parallel_workers: num_cpus::get().min(4), // Limit for faster startup
            enable_parallel: true,
            enable_incremental: true,
            dependency_tracking: true,
            cache_directory: Some(PathBuf::from(".cursed_cache/dev")),
            cache_max_size: 512, // 512MB for dev builds
            llvm_passes: LlvmPassConfig {
                function_passes: vec![
                    "mem2reg".to_string(),
                    "instcombine".to_string(),
                    "simplifycfg".to_string(),
                ],
                module_passes: vec![
                    "globalopt".to_string(),
                    "strip-dead-prototypes".to_string(),
                ],
                enable_vectorization: false,
                enable_loop_unrolling: false,
                enable_inlining: true,
                enable_constant_folding: true,
                enable_dead_code_elimination: true,
                enable_common_subexpression_elimination: false,
                enable_tail_call_optimization: false,
                enable_link_time_optimization: false,
                enable_memory_optimization: true,
            },
            target_cpu: None,
            target_features: Vec::new(),
            enable_profiling: false,
            profile_output_dir: None,
            profile_data_dir: None,
            benchmark_iterations: 1,
            generate_reports: false,
            report_output_dir: None,
            verbose_optimization: false,
            custom_passes: Vec::new(),
            optimization_flags: HashMap::new(),
        }
    }
    
    /// Debug configuration - no optimization, full debug info
    fn debug_config() -> OptimizationConfig {
        OptimizationConfig {
            optimization_level: OptimizationLevel::O0,
            debug_mode: true,
            profile_guided: false,
            parallel_workers: 1, // Single-threaded for deterministic builds
            enable_parallel: false,
            enable_incremental: true,
            dependency_tracking: true,
            cache_directory: Some(PathBuf::from(".cursed_cache/debug")),
            cache_max_size: 256, // Smaller cache for debug builds
            llvm_passes: LlvmPassConfig {
                function_passes: vec![
                    "mem2reg".to_string(), // Only promote allocas for readability
                ],
                module_passes: vec![
                    "strip-dead-prototypes".to_string(),
                ],
                enable_vectorization: false,
                enable_loop_unrolling: false,
                enable_inlining: false,
                enable_constant_folding: false,
                enable_dead_code_elimination: false,
                enable_common_subexpression_elimination: false,
                enable_tail_call_optimization: false,
                enable_link_time_optimization: false,
                enable_memory_optimization: false,
            },
            target_cpu: None,
            target_features: Vec::new(),
            enable_profiling: true,
            profile_output_dir: Some(PathBuf::from(".cursed_profiles/debug")),
            profile_data_dir: None,
            benchmark_iterations: 1,
            generate_reports: true,
            report_output_dir: Some(PathBuf::from(".cursed_reports/debug")),
            verbose_optimization: true,
            custom_passes: Vec::new(),
            optimization_flags: HashMap::new(),
        }
    }
    
    /// Release configuration - balanced performance
    fn release_config() -> OptimizationConfig {
        OptimizationConfig {
            optimization_level: OptimizationLevel::O2,
            debug_mode: false,
            profile_guided: false,
            parallel_workers: num_cpus::get(),
            enable_parallel: true,
            enable_incremental: true,
            dependency_tracking: true,
            cache_directory: Some(PathBuf::from(".cursed_cache/release")),
            cache_max_size: 1024, // 1GB for release builds
            llvm_passes: LlvmPassConfig {
                function_passes: vec![
                    "mem2reg".to_string(),
                    "instcombine".to_string(),
                    "simplifycfg".to_string(),
                    "reassociate".to_string(),
                    "gvn".to_string(),
                    "basic-aa".to_string(),
                ],
                module_passes: vec![
                    "globalopt".to_string(),
                    "globaldce".to_string(),
                    "function-attrs".to_string(),
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
            },
            target_cpu: Some("native".to_string()),
            target_features: vec!["sse4.2".to_string()],
            enable_profiling: false,
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
    
    /// Production configuration - maximum performance
    fn production_config() -> OptimizationConfig {
        OptimizationConfig {
            optimization_level: OptimizationLevel::O3,
            debug_mode: false,
            profile_guided: true,
            parallel_workers: num_cpus::get(),
            enable_parallel: true,
            enable_incremental: true,
            dependency_tracking: true,
            cache_directory: Some(PathBuf::from(".cursed_cache/production")),
            cache_max_size: 2048, // 2GB for production builds
            llvm_passes: LlvmPassConfig {
                function_passes: vec![
                    "mem2reg".to_string(),
                    "instcombine".to_string(),
                    "simplifycfg".to_string(),
                    "reassociate".to_string(),
                    "gvn".to_string(),
                    "basic-aa".to_string(),
                    "aggressive-instcombine".to_string(),
                    "licm".to_string(),
                    "indvars".to_string(),
                ],
                module_passes: vec![
                    "globalopt".to_string(),
                    "globaldce".to_string(),
                    "function-attrs".to_string(),
                    "constmerge".to_string(),
                    "inline".to_string(),
                    "argpromotion".to_string(),
                    "deadargelim".to_string(),
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
            },
            target_cpu: Some("native".to_string()),
            target_features: vec![
                "sse4.2".to_string(),
                "avx".to_string(),
                "avx2".to_string(),
            ],
            enable_profiling: true,
            profile_output_dir: Some(PathBuf::from(".cursed_profiles/production")),
            profile_data_dir: Some(PathBuf::from(".cursed_pgo_data")),
            benchmark_iterations: 5,
            generate_reports: true,
            report_output_dir: Some(PathBuf::from(".cursed_reports/production")),
            verbose_optimization: false,
            custom_passes: vec![
                "cursed-goroutine-opt".to_string(),
                "cursed-channel-opt".to_string(),
                "cursed-gc-opt".to_string(),
            ],
            optimization_flags: {
                let mut flags = HashMap::new();
                flags.insert("enable-pgo".to_string(), "true".to_string());
                flags.insert("enable-lto".to_string(), "true".to_string());
                flags.insert("enable-auto-vectorization".to_string(), "true".to_string());
                flags
            },
        }
    }
    
    /// Size configuration - minimize binary size
    fn size_config() -> OptimizationConfig {
        OptimizationConfig {
            optimization_level: OptimizationLevel::Os,
            debug_mode: false,
            profile_guided: false,
            parallel_workers: num_cpus::get(),
            enable_parallel: true,
            enable_incremental: true,
            dependency_tracking: true,
            cache_directory: Some(PathBuf::from(".cursed_cache/size")),
            cache_max_size: 512, // Smaller cache for size builds
            llvm_passes: LlvmPassConfig {
                function_passes: vec![
                    "mem2reg".to_string(),
                    "instcombine".to_string(),
                    "simplifycfg".to_string(),
                    "reassociate".to_string(),
                    "gvn".to_string(),
                ],
                module_passes: vec![
                    "globalopt".to_string(),
                    "globaldce".to_string(),
                    "constmerge".to_string(),
                    "mergefunc".to_string(),
                    "strip".to_string(),
                ],
                enable_vectorization: false, // Disable to reduce code size
                enable_loop_unrolling: false, // Disable to reduce code size
                enable_inlining: true, // Enable for size reduction
                enable_constant_folding: true,
                enable_dead_code_elimination: true,
                enable_common_subexpression_elimination: true,
                enable_tail_call_optimization: true,
                enable_link_time_optimization: true,
                enable_memory_optimization: true,
            },
            target_cpu: Some("generic".to_string()),
            target_features: Vec::new(), // Generic target for size
            enable_profiling: false,
            profile_output_dir: None,
            profile_data_dir: None,
            benchmark_iterations: 3,
            generate_reports: false,
            report_output_dir: None,
            verbose_optimization: false,
            custom_passes: Vec::new(),
            optimization_flags: {
                let mut flags = HashMap::new();
                flags.insert("optimize-for-size".to_string(), "true".to_string());
                flags.insert("enable-lto".to_string(), "true".to_string());
                flags
            },
        }
    }
    
    /// Testing configuration - optimized for test execution
    fn testing_config() -> OptimizationConfig {
        OptimizationConfig {
            optimization_level: OptimizationLevel::O1,
            debug_mode: true,
            profile_guided: false,
            parallel_workers: num_cpus::get(),
            enable_parallel: true,
            enable_incremental: true,
            dependency_tracking: true,
            cache_directory: Some(PathBuf::from(".cursed_cache/test")),
            cache_max_size: 1024, // Large cache for test builds
            llvm_passes: LlvmPassConfig {
                function_passes: vec![
                    "mem2reg".to_string(),
                    "instcombine".to_string(),
                    "simplifycfg".to_string(),
                    "reassociate".to_string(),
                ],
                module_passes: vec![
                    "globalopt".to_string(),
                    "strip-dead-prototypes".to_string(),
                ],
                enable_vectorization: false,
                enable_loop_unrolling: false,
                enable_inlining: true,
                enable_constant_folding: true,
                enable_dead_code_elimination: true,
                enable_common_subexpression_elimination: true,
                enable_tail_call_optimization: false,
                enable_link_time_optimization: false,
                enable_memory_optimization: true,
            },
            target_cpu: None,
            target_features: Vec::new(),
            enable_profiling: true,
            profile_output_dir: Some(PathBuf::from(".cursed_profiles/test")),
            profile_data_dir: None,
            benchmark_iterations: 1,
            generate_reports: true,
            report_output_dir: Some(PathBuf::from(".cursed_reports/test")),
            verbose_optimization: true,
            custom_passes: Vec::new(),
            optimization_flags: {
                let mut flags = HashMap::new();
                flags.insert("enable-coverage".to_string(), "true".to_string());
                flags.insert("enable-sanitizers".to_string(), "true".to_string());
                flags
            },
        }
    }
    
    /// Get profile name as string
    pub fn as_str(&self) -> &'static str {
        match self {
            BuildProfile::Development => "development",
            BuildProfile::Debug => "debug",
            BuildProfile::Release => "release",
            BuildProfile::Production => "production",
            BuildProfile::Size => "size",
            BuildProfile::Testing => "testing",
        }
    }
    
    /// Parse profile from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "dev" | "development" => Ok(BuildProfile::Development),
            "debug" | "dbg" => Ok(BuildProfile::Debug),
            "release" | "rel" => Ok(BuildProfile::Release),
            "production" | "prod" => Ok(BuildProfile::Production),
            "size" | "small" => Ok(BuildProfile::Size),
            "test" | "testing" => Ok(BuildProfile::Testing),
            _ => Err(format!("Unknown build profile: {}", s)),
        }
    }
    
    /// Get description of the build profile
    pub fn description(&self) -> &'static str {
        match self {
            BuildProfile::Development => "Fast compilation with basic optimizations for development",
            BuildProfile::Debug => "No optimization with full debug information",
            BuildProfile::Release => "Balanced performance and compilation time for releases",
            BuildProfile::Production => "Maximum runtime performance with aggressive optimizations",
            BuildProfile::Size => "Minimize binary size with size-focused optimizations",
            BuildProfile::Testing => "Optimized for test execution with coverage and debugging",
        }
    }
    
    /// Get expected compilation time multiplier
    pub fn compilation_time_factor(&self) -> f64 {
        match self {
            BuildProfile::Development => 1.0,
            BuildProfile::Debug => 0.8,
            BuildProfile::Release => 2.0,
            BuildProfile::Production => 4.0,
            BuildProfile::Size => 3.0,
            BuildProfile::Testing => 1.5,
        }
    }
    
    /// Get expected runtime performance multiplier
    pub fn runtime_performance_factor(&self) -> f64 {
        match self {
            BuildProfile::Development => 1.1,
            BuildProfile::Debug => 1.0,
            BuildProfile::Release => 1.8,
            BuildProfile::Production => 3.0,
            BuildProfile::Size => 1.5,
            BuildProfile::Testing => 1.2,
        }
    }
}

/// Profile manager for handling build profiles
pub struct ProfileManager {
    profiles: HashMap<BuildProfile, OptimizationConfig>,
    default_profile: BuildProfile,
}

impl ProfileManager {
    /// Create a new profile manager
    pub fn new() -> Self {
        let mut profiles = HashMap::new();
        
        // Initialize all build profiles
        for profile in &[
            BuildProfile::Development,
            BuildProfile::Debug,
            BuildProfile::Release,
            BuildProfile::Production,
            BuildProfile::Size,
            BuildProfile::Testing,
        ] {
            profiles.insert(*profile, profile.to_optimization_config());
        }
        
        Self {
            profiles,
            default_profile: BuildProfile::Release,
        }
    }
    
    /// Get configuration for a build profile
    pub fn get_profile_config(&self, profile: BuildProfile) -> Option<&OptimizationConfig> {
        self.profiles.get(&profile)
    }
    
    /// Get the default profile
    pub fn get_default_profile(&self) -> BuildProfile {
        self.default_profile
    }
    
    /// Set the default profile
    pub fn set_default_profile(&mut self, profile: BuildProfile) {
        self.default_profile = profile;
    }
    
    /// List all available profiles
    pub fn list_profiles(&self) -> Vec<BuildProfile> {
        self.profiles.keys().cloned().collect()
    }
    
    /// Get profile summary
    pub fn get_profile_summary(&self, profile: BuildProfile) -> String {
        format!(
            "{} ({}): {}\n  Compilation time: {:.1}x, Runtime performance: {:.1}x",
            profile.as_str(),
            profile.description(),
            match profile {
                BuildProfile::Development => "O1 + Parallel + Incremental",
                BuildProfile::Debug => "O0 + Debug Info + Profiling",
                BuildProfile::Release => "O2 + LTO + Target-specific",
                BuildProfile::Production => "O3 + PGO + LTO + All features",
                BuildProfile::Size => "Os + LTO + Size optimization",
                BuildProfile::Testing => "O1 + Coverage + Sanitizers",
            },
            profile.compilation_time_factor(),
            profile.runtime_performance_factor()
        )
    }
    
    /// Print all profiles summary
    pub fn print_profiles_summary(&self) {
        println!("📋 Available Build Profiles:");
        println!();
        
        for profile in self.list_profiles() {
            println!("🔧 {}", self.get_profile_summary(profile));
            println!();
        }
    }
}

impl Default for ProfileManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_profile_creation() {
        let dev_config = BuildProfile::Development.to_optimization_config();
        assert_eq!(dev_config.optimization_level, OptimizationLevel::O1);
        assert!(dev_config.debug_mode);
        assert!(dev_config.enable_incremental);
        
        let prod_config = BuildProfile::Production.to_optimization_config();
        assert_eq!(prod_config.optimization_level, OptimizationLevel::O3);
        assert!(!prod_config.debug_mode);
        assert!(prod_config.profile_guided);
        assert!(prod_config.llvm_passes.enable_link_time_optimization);
    }
    
    #[test]
    fn test_profile_parsing() {
        assert_eq!(BuildProfile::from_str("dev").unwrap(), BuildProfile::Development);
        assert_eq!(BuildProfile::from_str("release").unwrap(), BuildProfile::Release);
        assert_eq!(BuildProfile::from_str("production").unwrap(), BuildProfile::Production);
        assert!(BuildProfile::from_str("invalid").is_err());
    }
    
    #[test]
    fn test_profile_manager() {
        let manager = ProfileManager::new();
        assert_eq!(manager.get_default_profile(), BuildProfile::Release);
        
        let dev_config = manager.get_profile_config(BuildProfile::Development).unwrap();
        assert_eq!(dev_config.optimization_level, OptimizationLevel::O1);
        
        let profiles = manager.list_profiles();
        assert_eq!(profiles.len(), 6);
    }
    
    #[test]
    fn test_performance_factors() {
        let dev = BuildProfile::Development;
        let prod = BuildProfile::Production;
        
        assert!(prod.compilation_time_factor() > dev.compilation_time_factor());
        assert!(prod.runtime_performance_factor() > dev.runtime_performance_factor());
    }
    
    #[test]
    fn test_size_profile_optimizations() {
        let size_config = BuildProfile::Size.to_optimization_config();
        assert_eq!(size_config.optimization_level, OptimizationLevel::Os);
        assert!(!size_config.llvm_passes.enable_vectorization); // Disabled for size
        assert!(!size_config.llvm_passes.enable_loop_unrolling); // Disabled for size
        assert!(size_config.llvm_passes.enable_link_time_optimization); // Enabled for size reduction
    }
    
    #[test]
    fn test_production_profile_features() {
        let prod_config = BuildProfile::Production.to_optimization_config();
        assert!(prod_config.profile_guided);
        assert!(prod_config.enable_profiling);
        assert!(!prod_config.custom_passes.is_empty());
        assert!(prod_config.optimization_flags.contains_key("enable-pgo"));
        assert_eq!(prod_config.target_cpu, Some("native".to_string()));
        assert!(!prod_config.target_features.is_empty());
    }
}
