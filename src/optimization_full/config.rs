/// Optimization Configuration System
/// 
/// Provides comprehensive configuration for all optimization features including
/// LLVM passes, parallel compilation, caching, and performance monitoring.

use crate::error::{CursedError, Result};
use crate::common_types::optimization_level::OptimizationLevel;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use toml;

/// Main optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationConfig {
    // General settings
    
    // Parallel compilation
    
    // Incremental compilation
    pub cache_max_size: usize, // MB
    
    // LLVM optimization settings
    
    // Performance monitoring
    
    // Analysis and reporting
    
    // Custom optimization settings
impl Default for OptimizationConfig {
    fn default() -> Self {
        let cpu_count = num_cpus::get();
        
        Self {
            optimization_level: OptimizationLevel::O2, // Balanced default for better dev experience
            parallel_workers: cpu_count.max(2), // Ensure at least 2 workers for parallel benefits
            enable_parallel: cpu_count > 1, // Enable parallel only if beneficial
            cache_max_size: 2048, // 2GB default for better caching
        }
    }
// OptimizationLevel is imported at the top of the file

/// LLVM pass configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlvmPassConfig {
impl Default for LlvmPassConfig {
    fn default() -> Self {
        Self {
            function_passes: vec![
            module_passes: vec![
            enable_numa_optimization: false, // Disabled by default as not all systems are NUMA
        }
    }
impl LlvmPassConfig {
    /// Enhanced default configuration with more aggressive optimizations
    pub fn enhanced_default() -> Self {
        Self {
            function_passes: vec![
                "sroa".to_string(),          // Scalar replacement of aggregates
                "mem2reg".to_string(),       // Promote memory to register
                "licm".to_string(),          // Loop invariant code motion
                "indvars".to_string(),       // Canonicalize induction variables
                "loop-unroll".to_string(),   // Loop unrolling
                "early-cse".to_string(),     // Early common subexpression elimination
            module_passes: vec![
                "deadargelim".to_string(),   // Dead argument elimination
                "function-attrs".to_string(), // Function attribute inference
                "inline".to_string(),        // Function inlining
                "argpromotion".to_string(),  // Argument promotion
                "sccp".to_string(),          // Sparse conditional constant propagation
            enable_link_time_optimization: false, // LTO can be expensive, disabled by default
            enable_prefetch_insertion: false, // Conservative for enhanced default
        }
    }
    
    /// Lightweight configuration for debug builds
    pub fn debug_friendly() -> Self {
        Self {
            function_passes: vec![
            module_passes: vec![
            enable_interprocedural_analysis: false, // Disabled for debug builds
        }
    }
    
    /// Aggressive configuration for release builds
    pub fn aggressive_release() -> Self {
        Self {
            function_passes: vec![
            module_passes: vec![
            enable_prefetch_insertion: true, // Aggressive mode enables all optimizations
            enable_numa_optimization: true, // Enable for aggressive release builds
        }
    }
/// Predefined optimization profiles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationProfile {
impl OptimizationProfile {
    pub fn to_config(&self) -> OptimizationConfig {
        match self {
        }
    }
    
    fn development_config() -> OptimizationConfig {
        OptimizationConfig {
            llvm_passes: LlvmPassConfig {
                ..Default::default()
            ..Default::default()
        }
    }
    
    fn release_config() -> OptimizationConfig {
        OptimizationConfig {
            llvm_passes: LlvmPassConfig {
                ..Default::default()
            ..Default::default()
        }
    }
    
    fn debug_config() -> OptimizationConfig {
        OptimizationConfig {
            llvm_passes: LlvmPassConfig {
                ..Default::default()
            ..Default::default()
        }
    }
    
    fn size_config() -> OptimizationConfig {
        OptimizationConfig {
            llvm_passes: LlvmPassConfig {
                ..Default::default()
            ..Default::default()
        }
    }
    
    fn performance_config() -> OptimizationConfig {
        OptimizationConfig {
            llvm_passes: LlvmPassConfig {
                ..Default::default()
            ..Default::default()
        }
    }
impl OptimizationConfig {
    /// Load configuration from TOML file
    pub fn from_file(path: &PathBuf) -> Result<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| CursedError::General(format!("Failed to read config file: {}", e)))?;
        
        let config: OptimizationConfig = toml::from_str(&content)
            .map_err(|e| CursedError::General(format!("Failed to parse config file: {}", e)))?;
        
        Ok(config)
    /// Save configuration to TOML file
    pub fn to_file(&self, path: &PathBuf) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| CursedError::General(format!("Failed to serialize config: {}", e)))?;
        
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| CursedError::General(format!("Failed to create config directory: {}", e)))?;
        fs::write(path, content)
            .map_err(|e| CursedError::General(format!("Failed to write config file: {}", e)))?;
        
        Ok(())
    /// Create configuration from command line arguments
    pub fn from_args(args: &OptimizationArgs) -> Result<Self> {
        let mut config = if let Some(ref profile) = args.profile {
            profile.to_config()
        } else {
            OptimizationConfig::default()
        
        // Override with command line arguments
        if let Some(ref level) = args.optimization_level {
            config.optimization_level = OptimizationLevel::from_str(level)?;
        if let Some(workers) = args.parallel_workers {
            config.parallel_workers = workers;
        if let Some(parallel) = args.enable_parallel {
            config.enable_parallel = parallel;
        if let Some(incremental) = args.enable_incremental {
            config.enable_incremental = incremental;
        if let Some(ref cache_dir) = args.cache_directory {
            config.cache_directory = Some(cache_dir.clone());
        if let Some(profiling) = args.enable_profiling {
            config.enable_profiling = profiling;
        if let Some(verbose) = args.verbose {
            config.verbose_optimization = verbose;
        if let Some(ref target_cpu) = args.target_cpu {
            config.target_cpu = Some(target_cpu.clone());
        config.target_features.extend(args.target_features.clone());
        
        Ok(config)
    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        if self.parallel_workers == 0 {
            return Err(CursedError::General("Parallel workers must be greater than 0".to_string()));
        if self.cache_max_size < 10 {
            return Err(CursedError::General("Cache max size must be at least 10 MB".to_string()));
        if self.benchmark_iterations == 0 {
            return Err(CursedError::General("Benchmark iterations must be greater than 0".to_string()));
        // Validate cache directory if specified
        if let Some(ref cache_dir) = self.cache_directory {
            if let Some(parent) = cache_dir.parent() {
                if !parent.exists() {
                    return Err(CursedError::General(format!(
                        parent.display()
                    )));
                }
            }
        Ok(())
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
        !matches!(self.optimization_level, OptimizationLevel::O0)
    /// Get cache directory with fallback
    pub fn cache_dir(&self) -> PathBuf {
        self.cache_directory.clone()
            .unwrap_or_else(|| PathBuf::from(".cursed_cache"))
    /// Get profile output directory with fallback
    pub fn profile_dir(&self) -> PathBuf {
        self.profile_output_dir.clone()
            .unwrap_or_else(|| PathBuf::from(".cursed_profiles"))
    /// Get report output directory with fallback
    pub fn report_dir(&self) -> PathBuf {
        self.report_output_dir.clone()
            .unwrap_or_else(|| PathBuf::from(".cursed_reports"))
    /// Detect target CPU for optimizations
    fn detect_target_cpu() -> Option<String> {
        // Try to detect the native CPU for better optimization
        if let Ok(cpu_info) = std::env::var("CURSED_TARGET_CPU") {
            Some(cpu_info)
        } else {
            // Use native detection on supported platforms
            #[cfg(target_arch = "x86_64")]
            {
                Some("x86-64".to_string())
            }
            #[cfg(target_arch = "aarch64")]
            {
                Some("generic".to_string())
            }
            #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
            {
                None
            }
        }
    /// Detect target features for optimizations
    fn detect_target_features() -> Vec<String> {
        let mut features = Vec::new();
        
        // Check environment variable first
        if let Ok(features_str) = std::env::var("CURSED_TARGET_FEATURES") {
            return features_str.split(',').map(|s| s.trim().to_string()).collect();
        // Detect common features based on architecture
        #[cfg(target_arch = "x86_64")]
        {
            features.extend_from_slice(&[
            ]);
            
            // Check for additional features based on CPUID (simplified)
            if std::env::var("CURSED_ENABLE_AVX").is_ok() {
                features.push("avx".to_string());
                features.push("avx2".to_string());
            }
        }
        
        #[cfg(target_arch = "aarch64")]
        {
            features.extend_from_slice(&[
            ]);
        features
    /// Create optimized configuration for the current environment
    pub fn for_environment() -> Self {
        let mut config = Self::default();
        
        // Adjust based on available resources
        let cpu_count = num_cpus::get();
        config.parallel_workers = match cpu_count {
            5..=8 => cpu_count - 1, // Leave one core for OS
            _ => cpu_count / 2,     // Don't overwhelm on high-core systems
        
        // Adjust cache size based on available memory (simplified)
        if let Ok(memory_info) = std::env::var("CURSED_MEMORY_HINT") {
            if let Ok(memory_gb) = memory_info.parse::<usize>() {
                config.cache_max_size = match memory_gb {
                    0..=4 => 512,   // 512MB cache for low memory
                    5..=8 => 1024,  // 1GB cache for medium memory
                    9..=16 => 2048, // 2GB cache for high memory
                    _ => 4096,      // 4GB cache for very high memory
            }
        }
        
        // Enable LTO only for release builds with sufficient resources
        if cpu_count >= 4 && !config.debug_mode {
            config.llvm_passes.enable_link_time_optimization = true;
        config
    /// Create configuration optimized for fast development cycles
    pub fn for_development() -> Self {
        let mut config = OptimizationProfile::Development.to_config();
        
        // Use enhanced LLVM passes but keep them lightweight
        config.llvm_passes = LlvmPassConfig::debug_friendly();
        
        // Optimize for fast incremental builds
        config.enable_incremental = true;
        config.dependency_tracking = true;
        config.enable_parallel = true;
        config.parallel_workers = num_cpus::get().min(4); // Cap workers for development
        
        // Smaller cache for faster iteration
        config.cache_max_size = 512;
        
        config
    /// Create configuration optimized for production builds
    pub fn for_production() -> Self {
        let mut config = OptimizationProfile::Performance.to_config();
        
        // Use aggressive LLVM passes for maximum performance
        config.llvm_passes = LlvmPassConfig::aggressive_release();
        
        // Enable all optimizations
        config.enable_parallel = true;
        config.parallel_workers = num_cpus::get();
        config.enable_incremental = true; // Still beneficial for large projects
        config.profile_guided = true;
        
        // Large cache for complex optimizations
        config.cache_max_size = 4096;
        
        // Enable comprehensive profiling and reporting
        config.enable_profiling = true;
        config.generate_reports = true;
        config.benchmark_iterations = 5;
        
        config
    }
}

/// Command line arguments for optimization
#[derive(Debug, Clone)]
pub struct OptimizationArgs {
impl Default for OptimizationArgs {
    fn default() -> Self {
        Self {
        }
    }

/// Parallel compilation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelCompilationConfig {
/// Incremental compilation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncrementalCompilationConfig {
/// Link-time optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LtoConfig {
/// Debug information configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugInfoConfig {
/// Caching configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachingConfig {
/// Runtime optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeOptimizationConfig {
/// JIT optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JitOptimizationConfig {
/// Profile-guided optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PgoConfig {
/// Load balancing strategy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LoadBalancingStrategy {
/// Dependency granularity level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DependencyGranularity {
/// Change detection strategy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChangeDetectionStrategy {
/// Cache invalidation strategy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CacheInvalidationStrategy {
/// LTO mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LtoMode {
/// Debug info level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DebugInfoLevel {
/// Cache eviction strategy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CacheEvictionStrategy {
/// Instrumentation mode for PGO
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InstrumentationMode {
/// Collection mode for PGO
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CollectionMode {
impl Default for BuildOptimizationConfig {
    fn default() -> Self {
        Self {
        }
    }
impl Default for ParallelCompilationConfig {
    fn default() -> Self {
        Self {
        }
    }
impl Default for IncrementalCompilationConfig {
    fn default() -> Self {
        Self {
        }
    }
impl Default for LtoConfig {
    fn default() -> Self {
        Self {
        }
    }
impl Default for DebugInfoConfig {
    fn default() -> Self {
        Self {
        }
    }
impl Default for CachingConfig {
    fn default() -> Self {
        Self {
            max_cache_size: 1024, // 1GB
        }
    }
impl Default for RuntimeOptimizationConfig {
    fn default() -> Self {
        Self {
        }
    }
impl Default for JitOptimizationConfig {
    fn default() -> Self {
        Self {
        }
    }
impl Default for PgoConfig {
    fn default() -> Self {
        Self {
        }
    }
}
