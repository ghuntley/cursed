/// Build Profile Optimization Configurations
/// 
/// Provides predefined optimization configurations for different build scenarios
/// with appropriate optimization levels, features, and performance characteristics.

use crate::optimization::config::{OptimizationConfig, LlvmPassConfig};
use crate::common_types::optimization_level::OptimizationLevel;
use crate::optimization::optimization_levels::{LevelConfig, TargetSpecificSettings};
use std::collections::HashMap;
use std::path::PathBuf;

/// Build profile types for different use cases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildProfile {
    /// Development profile - fast compilation, minimal optimization
    /// Debug profile - debugging symbols, no optimization
    /// Release profile - balanced performance and compilation time
    /// Production profile - maximum runtime performance
    /// Size profile - minimize binary size
    /// Testing profile - optimized for test execution
impl BuildProfile {
    /// Get optimization configuration for this build profile
    pub fn to_optimization_config(&self) -> OptimizationConfig {
        match self {
        }
    }
    
    /// Development configuration - fast compilation, basic optimization
    fn development_config() -> OptimizationConfig {
        OptimizationConfig {
            parallel_workers: num_cpus::get().min(4), // Limit for faster startup
            cache_directory: Some(PathBuf::from(".cursed_cache/dev")),
            cache_max_size: 512, // 512MB for dev builds
            llvm_passes: LlvmPassConfig {
                function_passes: vec![
                module_passes: vec![
        }
    }
    
    /// Debug configuration - no optimization, full debug info
    fn debug_config() -> OptimizationConfig {
        OptimizationConfig {
            parallel_workers: 1, // Single-threaded for deterministic builds
            cache_directory: Some(PathBuf::from(".cursed_cache/debug")),
            cache_max_size: 256, // Smaller cache for debug builds
            llvm_passes: LlvmPassConfig {
                function_passes: vec![
                    "mem2reg".to_string(), // Only promote allocas for readability
                module_passes: vec![
            profile_output_dir: Some(PathBuf::from(".cursed_profiles/debug")),
            report_output_dir: Some(PathBuf::from(".cursed_reports/debug")),
        }
    }
    
    /// Release configuration - balanced performance
    fn release_config() -> OptimizationConfig {
        OptimizationConfig {
            cache_directory: Some(PathBuf::from(".cursed_cache/release")),
            cache_max_size: 1024, // 1GB for release builds
            llvm_passes: LlvmPassConfig {
                function_passes: vec![
                module_passes: vec![
        }
    }
    
    /// Production configuration - maximum performance
    fn production_config() -> OptimizationConfig {
        OptimizationConfig {
            cache_directory: Some(PathBuf::from(".cursed_cache/production")),
            cache_max_size: 2048, // 2GB for production builds
            llvm_passes: LlvmPassConfig {
                function_passes: vec![
                module_passes: vec![
            target_features: vec![
            profile_output_dir: Some(PathBuf::from(".cursed_profiles/production")),
            report_output_dir: Some(PathBuf::from(".cursed_reports/production")),
            custom_passes: vec![
            optimization_flags: {
                let mut flags = HashMap::new();
                flags.insert("enable-pgo".to_string(), "true".to_string());
                flags.insert("enable-lto".to_string(), "true".to_string());
                flags.insert("enable-auto-vectorization".to_string(), "true".to_string());
                flags
        }
    }
    
    /// Size configuration - minimize binary size
    fn size_config() -> OptimizationConfig {
        OptimizationConfig {
            cache_directory: Some(PathBuf::from(".cursed_cache/size")),
            cache_max_size: 512, // Smaller cache for size builds
            llvm_passes: LlvmPassConfig {
                function_passes: vec![
                module_passes: vec![
                enable_vectorization: false, // Disable to reduce code size
                enable_loop_unrolling: false, // Disable to reduce code size
                enable_inlining: true, // Enable for size reduction
            target_features: Vec::new(), // Generic target for size
            optimization_flags: {
                let mut flags = HashMap::new();
                flags.insert("optimize-for-size".to_string(), "true".to_string());
                flags.insert("enable-lto".to_string(), "true".to_string());
                flags
        }
    }
    
    /// Testing configuration - optimized for test execution
    fn testing_config() -> OptimizationConfig {
        OptimizationConfig {
            cache_directory: Some(PathBuf::from(".cursed_cache/test")),
            cache_max_size: 1024, // Large cache for test builds
            llvm_passes: LlvmPassConfig {
                function_passes: vec![
                module_passes: vec![
            profile_output_dir: Some(PathBuf::from(".cursed_profiles/test")),
            report_output_dir: Some(PathBuf::from(".cursed_reports/test")),
            optimization_flags: {
                let mut flags = HashMap::new();
                flags.insert("enable-coverage".to_string(), "true".to_string());
                flags.insert("enable-sanitizers".to_string(), "true".to_string());
                flags
        }
    }
    
    /// Get profile name as string
    pub fn as_str(&self) -> &'static str {
        match self {
        }
    }
    
    /// Parse profile from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
        }
    }
    
    /// Get description of the build profile
    pub fn description(&self) -> &'static str {
        match self {
        }
    }
    
    /// Get expected compilation time multiplier
    pub fn compilation_time_factor(&self) -> f64 {
        match self {
        }
    }
    
    /// Get expected runtime performance multiplier
    pub fn runtime_performance_factor(&self) -> f64 {
        match self {
        }
    }
/// Profile manager for handling build profiles
pub struct ProfileManager {
impl ProfileManager {
    /// Create a new profile manager
    pub fn new() -> Self {
        let mut profiles = HashMap::new();
        
        // Initialize all build profiles
        for profile in &[
        ] {
            profiles.insert(*profile, profile.to_optimization_config());
        Self {
        }
    }
    
    /// Get configuration for a build profile
    pub fn get_profile_config(&self, profile: BuildProfile) -> Option<&OptimizationConfig> {
        self.profiles.get(&profile)
    /// Get the default profile
    pub fn get_default_profile(&self) -> BuildProfile {
        self.default_profile
    /// Set the default profile
    pub fn set_default_profile(&mut self, profile: BuildProfile) {
        self.default_profile = profile;
    /// List all available profiles
    pub fn list_profiles(&self) -> Vec<BuildProfile> {
        self.profiles.keys().cloned().collect()
    /// Get profile summary
    pub fn get_profile_summary(&self, profile: BuildProfile) -> String {
        format!(
            match profile {
            profile.runtime_performance_factor()
        )
    /// Print all profiles summary
    pub fn print_profiles_summary(&self) {
        println!("📋 Available Build Profiles:");
        println!();
        
        for profile in self.list_profiles() {
            println!("🔧 {}", self.get_profile_summary(profile));
            println!();
        }
    }
impl Default for ProfileManager {
    fn default() -> Self {
        Self::new()
    }
}

