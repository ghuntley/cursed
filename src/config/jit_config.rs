/// JIT Configuration Module for CURSED Language
/// 
/// Provides comprehensive configuration options for JIT compilation,
/// including optimization levels, performance tuning, and runtime behavior.
/// Supports loading from files, environment variables, and command-line arguments.

use crate::error::CursedError;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;
use std::collections::HashMap;

/// Main JIT configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JitConfig {
    /// Core JIT engine settings
    /// Compilation behavior settings
    /// Runtime behavior settings
    /// Performance monitoring settings
    /// Memory management settings
    /// Optimization settings
    /// Debug and development settings
/// JIT engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JitEngineConfig {
    /// Optimization level: "none", "less", "default", "aggressive"
    /// Enable function caching
    /// Maximum cached functions
    /// Enable debug information in JIT code
    /// Target CPU for optimization (e.g., "native", "x86-64")
    /// Target features for optimization
    /// Enable ORC JIT v2 (if available)
    /// Enable lazy compilation
/// JIT compilation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JitCompilationConfig {
    /// Hot path execution threshold
    /// Compilation timeout in seconds
    /// Enable dynamic recompilation
    /// Enable background compilation
    /// Maximum parallel compilations
    /// Enable profiling-guided optimization
    /// Hot path optimization level
    /// Regular optimization level
    /// Enable inlining
    /// Inline threshold
/// JIT runtime configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JitRuntimeConfig {
    /// Enable goroutine integration
    /// Enable garbage collection integration
    /// Enable panic recovery
    /// Enable background optimization
    /// Optimization trigger threshold (%)
    /// Enable runtime profiling
    /// Performance monitoring interval in seconds
    /// Enable stack trace collection
/// JIT performance monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JitMonitoringConfig {
    /// Enable performance monitoring
    /// Monitoring sample rate (0.0 to 1.0)
    /// History size for performance data
    /// Enable performance reporting
    /// Report interval in seconds
    /// Performance thresholds
    /// Enable metrics collection
    /// Metrics output format: "json", "csv", "prometheus"
/// Performance threshold configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    /// Maximum acceptable execution time (ms)
    /// Maximum acceptable memory usage (MB)
    /// Maximum acceptable compilation time (ms)
    /// Performance degradation threshold (%)
/// JIT memory management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JitMemoryConfig {
    /// Maximum JIT memory usage in MB
    /// Enable memory pressure monitoring
    /// Memory pressure threshold (%)
    /// Enable automatic garbage collection triggers
    /// GC trigger threshold (%)
    /// Enable memory usage reporting
    /// Memory allocation tracking
/// JIT optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JitOptimizationConfig {
    /// Enable vectorization
    /// Enable loop optimization
    /// Enable constant folding
    /// Enable dead code elimination
    /// Enable common subexpression elimination
    /// Enable register allocation optimization
    /// Enable instruction scheduling
    /// Enable branch prediction optimization
    /// Optimization passes
    /// Custom optimization options
/// JIT debug and development configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JitDebugConfig {
    /// Enable debug mode
    /// Enable verbose logging
    /// Enable IR dumping
    /// IR dump directory
    /// Enable assembly dumping
    /// Assembly dump directory
    /// Enable timing information
    /// Enable compilation statistics
    /// Enable optimization statistics
impl Default for JitConfig {
    fn default() -> Self {
        Self {
        }
    }
impl Default for JitEngineConfig {
    fn default() -> Self {
        Self {
        }
    }
impl Default for JitCompilationConfig {
    fn default() -> Self {
        Self {
        }
    }
impl Default for JitRuntimeConfig {
    fn default() -> Self {
        Self {
        }
    }
impl Default for JitMonitoringConfig {
    fn default() -> Self {
        Self {
        }
    }
impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
        }
    }
impl Default for JitMemoryConfig {
    fn default() -> Self {
        Self {
        }
    }
impl Default for JitOptimizationConfig {
    fn default() -> Self {
        Self {
            optimization_passes: vec![
        }
    }
impl Default for JitDebugConfig {
    fn default() -> Self {
        Self {
        }
    }
impl JitConfig {
    /// Create a new JIT configuration with default values
    pub fn new() -> Self {
        Self::default()
    /// Load configuration from a TOML file
    pub fn from_toml_file<P: AsRef<Path>>(path: P) -> crate::error::Result<()> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| CursedError::from_str(&format!("Failed to read config file: {}", e)))?;
        
        toml::from_str(&content)
            .map_err(|e| CursedError::from_str(&format!("Failed to parse TOML config: {}", e)))
    /// Load configuration from a JSON file
    pub fn from_json_file<P: AsRef<Path>>(path: P) -> crate::error::Result<()> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| CursedError::from_str(&format!("Failed to read config file: {}", e)))?;
        
        serde_json::from_str(&content)
            .map_err(|e| CursedError::from_str(&format!("Failed to parse JSON config: {}", e)))
    /// Load configuration from environment variables
    pub fn from_env() -> crate::error::Result<()> {
        let mut config = Self::default();

        // Engine configuration
        if let Ok(level) = std::env::var("CURSED_JIT_OPTIMIZATION_LEVEL") {
            config.engine.optimization_level = level;
        }
        if let Ok(cache) = std::env::var("CURSED_JIT_ENABLE_CACHE") {
            config.engine.enable_function_cache = cache.parse().unwrap_or(true);
        }
        if let Ok(max_cache) = std::env::var("CURSED_JIT_MAX_CACHED_FUNCTIONS") {
            config.engine.max_cached_functions = max_cache.parse().unwrap_or(1000);
        }
        if let Ok(cpu) = std::env::var("CURSED_JIT_TARGET_CPU") {
            config.engine.target_cpu = Some(cpu);
        // Compilation configuration
        if let Ok(threshold) = std::env::var("CURSED_JIT_HOT_PATH_THRESHOLD") {
            config.compilation.hot_path_threshold = threshold.parse().unwrap_or(100);
        }
        if let Ok(timeout) = std::env::var("CURSED_JIT_COMPILATION_TIMEOUT") {
            config.compilation.compilation_timeout_secs = timeout.parse().unwrap_or(30);
        }
        if let Ok(recompile) = std::env::var("CURSED_JIT_ENABLE_RECOMPILATION") {
            config.compilation.enable_dynamic_recompilation = recompile.parse().unwrap_or(true);
        // Runtime configuration
        if let Ok(goroutines) = std::env::var("CURSED_JIT_ENABLE_GOROUTINES") {
            config.runtime.enable_goroutine_integration = goroutines.parse().unwrap_or(true);
        }
        if let Ok(gc) = std::env::var("CURSED_JIT_ENABLE_GC") {
            config.runtime.enable_gc_integration = gc.parse().unwrap_or(true);
        }
        if let Ok(panic) = std::env::var("CURSED_JIT_ENABLE_PANIC_RECOVERY") {
            config.runtime.enable_panic_recovery = panic.parse().unwrap_or(true);
        // Memory configuration
        if let Ok(memory) = std::env::var("CURSED_JIT_MAX_MEMORY_MB") {
            config.memory.max_jit_memory_mb = memory.parse().unwrap_or(100);
        // Debug configuration
        if let Ok(debug) = std::env::var("CURSED_JIT_DEBUG_MODE") {
            config.debug.debug_mode = debug.parse().unwrap_or(false);
        }
        if let Ok(verbose) = std::env::var("CURSED_JIT_VERBOSE") {
            config.debug.verbose_logging = verbose.parse().unwrap_or(false);
        Ok(config)
    /// Save configuration to a TOML file
    pub fn save_to_toml_file<P: AsRef<Path>>(&self, path: P) -> crate::error::Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| CursedError::from_str(&format!("Failed to serialize config to TOML: {}", e)))?;
        
        std::fs::write(path, content)
            .map_err(|e| CursedError::from_str(&format!("Failed to write config file: {}", e)))
    /// Save configuration to a JSON file
    pub fn save_to_json_file<P: AsRef<Path>>(&self, path: P) -> crate::error::Result<()> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| CursedError::from_str(&format!("Failed to serialize config to JSON: {}", e)))?;
        
        std::fs::write(path, content)
            .map_err(|e| CursedError::from_str(&format!("Failed to write config file: {}", e)))
    /// Merge with another configuration (other takes precedence)
    pub fn merge(&mut self, other: &Self) {
        // Engine config
        if other.engine.optimization_level != "default" {
            self.engine.optimization_level = other.engine.optimization_level.clone();
        }
        self.engine.enable_function_cache = other.engine.enable_function_cache;
        if other.engine.max_cached_functions != 1000 {
            self.engine.max_cached_functions = other.engine.max_cached_functions;
        }
        if other.engine.target_cpu.is_some() {
            self.engine.target_cpu = other.engine.target_cpu.clone();
        // Compilation config
        if other.compilation.hot_path_threshold != 100 {
            self.compilation.hot_path_threshold = other.compilation.hot_path_threshold;
        }
        if other.compilation.compilation_timeout_secs != 30 {
            self.compilation.compilation_timeout_secs = other.compilation.compilation_timeout_secs;
        }
        self.compilation.enable_dynamic_recompilation = other.compilation.enable_dynamic_recompilation;

        // Runtime config
        self.runtime.enable_goroutine_integration = other.runtime.enable_goroutine_integration;
        self.runtime.enable_gc_integration = other.runtime.enable_gc_integration;
        self.runtime.enable_panic_recovery = other.runtime.enable_panic_recovery;

        // Memory config
        if other.memory.max_jit_memory_mb != 100 {
            self.memory.max_jit_memory_mb = other.memory.max_jit_memory_mb;
        // Debug config
        self.debug.debug_mode = other.debug.debug_mode;
        self.debug.verbose_logging = other.debug.verbose_logging;
    /// Validate configuration values
    pub fn validate(&self) -> crate::error::Result<()> {
        // Validate optimization levels
        let valid_opt_levels = ["none", "less", "default", "aggressive"];
        if !valid_opt_levels.contains(&self.engine.optimization_level.as_str()) {
            return Err(CursedError::from_str(&format!(
                self.engine.optimization_level, valid_opt_levels
            )));
        // Validate thresholds
        if self.compilation.hot_path_threshold == 0 {
            return Err(CursedError::from_str("Hot path threshold must be greater than 0"));
        if self.compilation.compilation_timeout_secs == 0 {
            return Err(CursedError::from_str("Compilation timeout must be greater than 0"));
        if self.memory.max_jit_memory_mb == 0 {
            return Err(CursedError::from_str("Max JIT memory must be greater than 0"));
        if self.monitoring.sample_rate < 0.0 || self.monitoring.sample_rate > 1.0 {
            return Err(CursedError::from_str("Monitoring sample rate must be between 0.0 and 1.0"));
        // Validate directories
        if let Some(ref dir) = self.debug.ir_dump_dir {
            if !Path::new(dir).exists() {
                return Err(CursedError::from_str(&format!("IR dump directory does not exist: {}", dir)));
            }
        }

        if let Some(ref dir) = self.debug.asm_dump_dir {
            if !Path::new(dir).exists() {
                return Err(CursedError::from_str(&format!("Assembly dump directory does not exist: {}", dir)));
            }
        }

        Ok(())
    /// Create a development configuration
    pub fn development() -> Self {
        let mut config = Self::default();
        
        // Engine settings for development
        config.engine.optimization_level = "none".to_string();
        config.engine.enable_debug_info = true;
        config.engine.max_cached_functions = 100;
        
        // Compilation settings for development
        config.compilation.hot_path_threshold = 10;
        config.compilation.compilation_timeout_secs = 10;
        config.compilation.enable_dynamic_recompilation = false;
        config.compilation.enable_background_compilation = false;
        
        // Debug settings for development
        config.debug.debug_mode = true;
        config.debug.verbose_logging = true;
        config.debug.enable_timing = true;
        config.debug.enable_compilation_stats = true;
        
        // Monitoring settings for development
        config.monitoring.enabled = true;
        config.monitoring.sample_rate = 1.0;
        config.monitoring.enable_reporting = true;
        
        config
    /// Create a production configuration
    pub fn production() -> Self {
        let mut config = Self::default();
        
        // Engine settings for production
        config.engine.optimization_level = "aggressive".to_string();
        config.engine.enable_debug_info = false;
        config.engine.max_cached_functions = 5000;
        config.engine.target_cpu = Some("native".to_string());
        
        // Compilation settings for production
        config.compilation.hot_path_threshold = 50;
        config.compilation.enable_pgo = true;
        config.compilation.max_parallel_compilations = num_cpus::get();
        
        // Runtime settings for production
        config.runtime.enable_runtime_profiling = false; // Disable for performance
        
        // Memory settings for production
        config.memory.max_jit_memory_mb = 500;
        config.memory.enable_auto_gc_triggers = true;
        
        // Monitoring settings for production
        config.monitoring.sample_rate = 0.01; // 1% sampling
        config.monitoring.enable_metrics = true;
        
        // Debug settings for production
        config.debug.debug_mode = false;
        config.debug.verbose_logging = false;
        
        config
    /// Create a benchmarking configuration
    pub fn benchmarking() -> Self {
        let mut config = Self::production();
        
        // Maximize performance for benchmarking
        config.engine.optimization_level = "aggressive".to_string();
        config.compilation.enable_background_compilation = false; // Deterministic timing
        config.runtime.enable_runtime_profiling = false;
        config.monitoring.enabled = false; // No monitoring overhead
        
        // Enable detailed statistics
        config.debug.enable_timing = true;
        config.debug.enable_compilation_stats = true;
        config.debug.enable_optimization_stats = true;
        
        config
    /// Get configuration as environment variables map
    pub fn to_env_vars(&self) -> HashMap<String, String> {
        let mut vars = HashMap::new();
        
        vars.insert("CURSED_JIT_OPTIMIZATION_LEVEL".to_string(), self.engine.optimization_level.clone());
        vars.insert("CURSED_JIT_ENABLE_CACHE".to_string(), self.engine.enable_function_cache.to_string());
        vars.insert("CURSED_JIT_MAX_CACHED_FUNCTIONS".to_string(), self.engine.max_cached_functions.to_string());
        
        if let Some(ref cpu) = self.engine.target_cpu {
            vars.insert("CURSED_JIT_TARGET_CPU".to_string(), cpu.clone());
        vars.insert("CURSED_JIT_HOT_PATH_THRESHOLD".to_string(), self.compilation.hot_path_threshold.to_string());
        vars.insert("CURSED_JIT_COMPILATION_TIMEOUT".to_string(), self.compilation.compilation_timeout_secs.to_string());
        vars.insert("CURSED_JIT_ENABLE_RECOMPILATION".to_string(), self.compilation.enable_dynamic_recompilation.to_string());
        
        vars.insert("CURSED_JIT_ENABLE_GOROUTINES".to_string(), self.runtime.enable_goroutine_integration.to_string());
        vars.insert("CURSED_JIT_ENABLE_GC".to_string(), self.runtime.enable_gc_integration.to_string());
        vars.insert("CURSED_JIT_ENABLE_PANIC_RECOVERY".to_string(), self.runtime.enable_panic_recovery.to_string());
        
        vars.insert("CURSED_JIT_MAX_MEMORY_MB".to_string(), self.memory.max_jit_memory_mb.to_string());
        
        vars.insert("CURSED_JIT_DEBUG_MODE".to_string(), self.debug.debug_mode.to_string());
        vars.insert("CURSED_JIT_VERBOSE".to_string(), self.debug.verbose_logging.to_string());
        
        vars
    /// Get configuration summary as a string
    pub fn summary(&self) -> String {
        format!(
            "JIT Configuration Summary:\n\
             - Optimization Level: {}\n\
             - Function Cache: {} (max: {})\n\
             - Hot Path Threshold: {}\n\
             - Max Memory: {} MB\n\
             - Goroutine Integration: {}\n\
             - GC Integration: {}\n\
            self.debug.debug_mode
        )
    }
}

/// Convert optimization level string to inkwell OptimizationLevel
pub fn parse_optimization_level(level: &str) -> crate::error::Result<()> {
    match level.to_lowercase().as_str() {
    }
}

/// Convert Duration to seconds for serialization
pub fn duration_to_secs(duration: Duration) -> u64 {
    duration.as_secs()
/// Convert seconds to Duration for deserialization
pub fn secs_to_duration(secs: u64) -> Duration {
    Duration::from_secs(secs)
