/// JIT Configuration Module for CURSED Language
/// 
/// Provides comprehensive configuration options for JIT compilation,
/// including optimization levels, performance tuning, and runtime behavior.
/// Supports loading from files, environment variables, and command-line arguments.

use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;
use std::collections::HashMap;

/// Main JIT configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JitConfig {
    /// Core JIT engine settings
    pub engine: JitEngineConfig,
    /// Compilation behavior settings
    pub compilation: JitCompilationConfig,
    /// Runtime behavior settings
    pub runtime: JitRuntimeConfig,
    /// Performance monitoring settings
    pub monitoring: JitMonitoringConfig,
    /// Memory management settings
    pub memory: JitMemoryConfig,
    /// Optimization settings
    pub optimization: JitOptimizationConfig,
    /// Debug and development settings
    pub debug: JitDebugConfig,
}

/// JIT engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JitEngineConfig {
    /// Optimization level: "none", "less", "default", "aggressive"
    pub optimization_level: String,
    /// Enable function caching
    pub enable_function_cache: bool,
    /// Maximum cached functions
    pub max_cached_functions: usize,
    /// Enable debug information in JIT code
    pub enable_debug_info: bool,
    /// Target CPU for optimization (e.g., "native", "x86-64")
    pub target_cpu: Option<String>,
    /// Target features for optimization
    pub target_features: Vec<String>,
    /// Enable ORC JIT v2 (if available)
    pub enable_orc_v2: bool,
    /// Enable lazy compilation
    pub enable_lazy_compilation: bool,
}

/// JIT compilation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JitCompilationConfig {
    /// Hot path execution threshold
    pub hot_path_threshold: u64,
    /// Compilation timeout in seconds
    pub compilation_timeout_secs: u64,
    /// Enable dynamic recompilation
    pub enable_dynamic_recompilation: bool,
    /// Enable background compilation
    pub enable_background_compilation: bool,
    /// Maximum parallel compilations
    pub max_parallel_compilations: usize,
    /// Enable profiling-guided optimization
    pub enable_pgo: bool,
    /// Hot path optimization level
    pub hot_path_optimization_level: String,
    /// Regular optimization level
    pub regular_optimization_level: String,
    /// Enable inlining
    pub enable_inlining: bool,
    /// Inline threshold
    pub inline_threshold: u32,
}

/// JIT runtime configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JitRuntimeConfig {
    /// Enable goroutine integration
    pub enable_goroutine_integration: bool,
    /// Enable garbage collection integration
    pub enable_gc_integration: bool,
    /// Enable panic recovery
    pub enable_panic_recovery: bool,
    /// Enable background optimization
    pub enable_background_optimization: bool,
    /// Optimization trigger threshold (%)
    pub optimization_trigger_threshold: f64,
    /// Enable runtime profiling
    pub enable_runtime_profiling: bool,
    /// Performance monitoring interval in seconds
    pub monitoring_interval_secs: u64,
    /// Enable stack trace collection
    pub enable_stack_traces: bool,
}

/// JIT performance monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JitMonitoringConfig {
    /// Enable performance monitoring
    pub enabled: bool,
    /// Monitoring sample rate (0.0 to 1.0)
    pub sample_rate: f64,
    /// History size for performance data
    pub history_size: usize,
    /// Enable performance reporting
    pub enable_reporting: bool,
    /// Report interval in seconds
    pub report_interval_secs: u64,
    /// Performance thresholds
    pub thresholds: PerformanceThresholds,
    /// Enable metrics collection
    pub enable_metrics: bool,
    /// Metrics output format: "json", "csv", "prometheus"
    pub metrics_format: String,
}

/// Performance threshold configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    /// Maximum acceptable execution time (ms)
    pub max_execution_time_ms: u64,
    /// Maximum acceptable memory usage (MB)
    pub max_memory_usage_mb: usize,
    /// Maximum acceptable compilation time (ms)
    pub max_compilation_time_ms: u64,
    /// Performance degradation threshold (%)
    pub performance_degradation_threshold: f64,
}

/// JIT memory management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JitMemoryConfig {
    /// Maximum JIT memory usage in MB
    pub max_jit_memory_mb: usize,
    /// Enable memory pressure monitoring
    pub enable_memory_pressure_monitoring: bool,
    /// Memory pressure threshold (%)
    pub memory_pressure_threshold: f64,
    /// Enable automatic garbage collection triggers
    pub enable_auto_gc_triggers: bool,
    /// GC trigger threshold (%)
    pub gc_trigger_threshold: f64,
    /// Enable memory usage reporting
    pub enable_memory_reporting: bool,
    /// Memory allocation tracking
    pub enable_allocation_tracking: bool,
}

/// JIT optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JitOptimizationConfig {
    /// Enable vectorization
    pub enable_vectorization: bool,
    /// Enable loop optimization
    pub enable_loop_optimization: bool,
    /// Enable constant folding
    pub enable_constant_folding: bool,
    /// Enable dead code elimination
    pub enable_dead_code_elimination: bool,
    /// Enable common subexpression elimination
    pub enable_cse: bool,
    /// Enable register allocation optimization
    pub enable_register_allocation: bool,
    /// Enable instruction scheduling
    pub enable_instruction_scheduling: bool,
    /// Enable branch prediction optimization
    pub enable_branch_prediction: bool,
    /// Optimization passes
    pub optimization_passes: Vec<String>,
    /// Custom optimization options
    pub custom_options: HashMap<String, String>,
}

/// JIT debug and development configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JitDebugConfig {
    /// Enable debug mode
    pub debug_mode: bool,
    /// Enable verbose logging
    pub verbose_logging: bool,
    /// Enable IR dumping
    pub enable_ir_dump: bool,
    /// IR dump directory
    pub ir_dump_dir: Option<String>,
    /// Enable assembly dumping
    pub enable_asm_dump: bool,
    /// Assembly dump directory
    pub asm_dump_dir: Option<String>,
    /// Enable timing information
    pub enable_timing: bool,
    /// Enable compilation statistics
    pub enable_compilation_stats: bool,
    /// Enable optimization statistics
    pub enable_optimization_stats: bool,
}

impl Default for JitConfig {
    fn default() -> Self {
        Self {
            engine: JitEngineConfig::default(),
            compilation: JitCompilationConfig::default(),
            runtime: JitRuntimeConfig::default(),
            monitoring: JitMonitoringConfig::default(),
            memory: JitMemoryConfig::default(),
            optimization: JitOptimizationConfig::default(),
            debug: JitDebugConfig::default(),
        }
    }
}

impl Default for JitEngineConfig {
    fn default() -> Self {
        Self {
            optimization_level: "default".to_string(),
            enable_function_cache: true,
            max_cached_functions: 1000,
            enable_debug_info: false,
            target_cpu: None,
            target_features: Vec::new(),
            enable_orc_v2: true,
            enable_lazy_compilation: true,
        }
    }
}

impl Default for JitCompilationConfig {
    fn default() -> Self {
        Self {
            hot_path_threshold: 100,
            compilation_timeout_secs: 30,
            enable_dynamic_recompilation: true,
            enable_background_compilation: true,
            max_parallel_compilations: num_cpus::get(),
            enable_pgo: false,
            hot_path_optimization_level: "aggressive".to_string(),
            regular_optimization_level: "default".to_string(),
            enable_inlining: true,
            inline_threshold: 275,
        }
    }
}

impl Default for JitRuntimeConfig {
    fn default() -> Self {
        Self {
            enable_goroutine_integration: true,
            enable_gc_integration: true,
            enable_panic_recovery: true,
            enable_background_optimization: true,
            optimization_trigger_threshold: 10.0,
            enable_runtime_profiling: true,
            monitoring_interval_secs: 30,
            enable_stack_traces: false,
        }
    }
}

impl Default for JitMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            sample_rate: 1.0,
            history_size: 1000,
            enable_reporting: false,
            report_interval_secs: 300,
            thresholds: PerformanceThresholds::default(),
            enable_metrics: false,
            metrics_format: "json".to_string(),
        }
    }
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            max_execution_time_ms: 1000,
            max_memory_usage_mb: 100,
            max_compilation_time_ms: 5000,
            performance_degradation_threshold: 25.0,
        }
    }
}

impl Default for JitMemoryConfig {
    fn default() -> Self {
        Self {
            max_jit_memory_mb: 100,
            enable_memory_pressure_monitoring: true,
            memory_pressure_threshold: 80.0,
            enable_auto_gc_triggers: true,
            gc_trigger_threshold: 75.0,
            enable_memory_reporting: false,
            enable_allocation_tracking: true,
        }
    }
}

impl Default for JitOptimizationConfig {
    fn default() -> Self {
        Self {
            enable_vectorization: true,
            enable_loop_optimization: true,
            enable_constant_folding: true,
            enable_dead_code_elimination: true,
            enable_cse: true,
            enable_register_allocation: true,
            enable_instruction_scheduling: true,
            enable_branch_prediction: true,
            optimization_passes: vec![
                "mem2reg".to_string(),
                "instcombine".to_string(),
                "reassociate".to_string(),
                "gvn".to_string(),
                "simplifycfg".to_string(),
            ],
            custom_options: HashMap::new(),
        }
    }
}

impl Default for JitDebugConfig {
    fn default() -> Self {
        Self {
            debug_mode: cfg!(debug_assertions),
            verbose_logging: false,
            enable_ir_dump: false,
            ir_dump_dir: None,
            enable_asm_dump: false,
            asm_dump_dir: None,
            enable_timing: false,
            enable_compilation_stats: false,
            enable_optimization_stats: false,
        }
    }
}

impl JitConfig {
    /// Create a new JIT configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Load configuration from a TOML file
    pub fn from_toml_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| Error::from_str(&format!("Failed to read config file: {}", e)))?;
        
        toml::from_str(&content)
            .map_err(|e| Error::from_str(&format!("Failed to parse TOML config: {}", e)))
    }

    /// Load configuration from a JSON file
    pub fn from_json_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| Error::from_str(&format!("Failed to read config file: {}", e)))?;
        
        serde_json::from_str(&content)
            .map_err(|e| Error::from_str(&format!("Failed to parse JSON config: {}", e)))
    }

    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, Error> {
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
        }

        // Compilation configuration
        if let Ok(threshold) = std::env::var("CURSED_JIT_HOT_PATH_THRESHOLD") {
            config.compilation.hot_path_threshold = threshold.parse().unwrap_or(100);
        }
        if let Ok(timeout) = std::env::var("CURSED_JIT_COMPILATION_TIMEOUT") {
            config.compilation.compilation_timeout_secs = timeout.parse().unwrap_or(30);
        }
        if let Ok(recompile) = std::env::var("CURSED_JIT_ENABLE_RECOMPILATION") {
            config.compilation.enable_dynamic_recompilation = recompile.parse().unwrap_or(true);
        }

        // Runtime configuration
        if let Ok(goroutines) = std::env::var("CURSED_JIT_ENABLE_GOROUTINES") {
            config.runtime.enable_goroutine_integration = goroutines.parse().unwrap_or(true);
        }
        if let Ok(gc) = std::env::var("CURSED_JIT_ENABLE_GC") {
            config.runtime.enable_gc_integration = gc.parse().unwrap_or(true);
        }
        if let Ok(panic) = std::env::var("CURSED_JIT_ENABLE_PANIC_RECOVERY") {
            config.runtime.enable_panic_recovery = panic.parse().unwrap_or(true);
        }

        // Memory configuration
        if let Ok(memory) = std::env::var("CURSED_JIT_MAX_MEMORY_MB") {
            config.memory.max_jit_memory_mb = memory.parse().unwrap_or(100);
        }

        // Debug configuration
        if let Ok(debug) = std::env::var("CURSED_JIT_DEBUG_MODE") {
            config.debug.debug_mode = debug.parse().unwrap_or(false);
        }
        if let Ok(verbose) = std::env::var("CURSED_JIT_VERBOSE") {
            config.debug.verbose_logging = verbose.parse().unwrap_or(false);
        }

        Ok(config)
    }

    /// Save configuration to a TOML file
    pub fn save_to_toml_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| Error::from_str(&format!("Failed to serialize config to TOML: {}", e)))?;
        
        std::fs::write(path, content)
            .map_err(|e| Error::from_str(&format!("Failed to write config file: {}", e)))
    }

    /// Save configuration to a JSON file
    pub fn save_to_json_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| Error::from_str(&format!("Failed to serialize config to JSON: {}", e)))?;
        
        std::fs::write(path, content)
            .map_err(|e| Error::from_str(&format!("Failed to write config file: {}", e)))
    }

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
        }

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
        }

        // Debug config
        self.debug.debug_mode = other.debug.debug_mode;
        self.debug.verbose_logging = other.debug.verbose_logging;
    }

    /// Validate configuration values
    pub fn validate(&self) -> Result<(), Error> {
        // Validate optimization levels
        let valid_opt_levels = ["none", "less", "default", "aggressive"];
        if !valid_opt_levels.contains(&self.engine.optimization_level.as_str()) {
            return Err(Error::from_str(&format!(
                "Invalid optimization level: {}. Valid values: {:?}",
                self.engine.optimization_level, valid_opt_levels
            )));
        }

        // Validate thresholds
        if self.compilation.hot_path_threshold == 0 {
            return Err(Error::from_str("Hot path threshold must be greater than 0"));
        }

        if self.compilation.compilation_timeout_secs == 0 {
            return Err(Error::from_str("Compilation timeout must be greater than 0"));
        }

        if self.memory.max_jit_memory_mb == 0 {
            return Err(Error::from_str("Max JIT memory must be greater than 0"));
        }

        if self.monitoring.sample_rate < 0.0 || self.monitoring.sample_rate > 1.0 {
            return Err(Error::from_str("Monitoring sample rate must be between 0.0 and 1.0"));
        }

        // Validate directories
        if let Some(ref dir) = self.debug.ir_dump_dir {
            if !Path::new(dir).exists() {
                return Err(Error::from_str(&format!("IR dump directory does not exist: {}", dir)));
            }
        }

        if let Some(ref dir) = self.debug.asm_dump_dir {
            if !Path::new(dir).exists() {
                return Err(Error::from_str(&format!("Assembly dump directory does not exist: {}", dir)));
            }
        }

        Ok(())
    }

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
    }

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
    }

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
    }

    /// Get configuration as environment variables map
    pub fn to_env_vars(&self) -> HashMap<String, String> {
        let mut vars = HashMap::new();
        
        vars.insert("CURSED_JIT_OPTIMIZATION_LEVEL".to_string(), self.engine.optimization_level.clone());
        vars.insert("CURSED_JIT_ENABLE_CACHE".to_string(), self.engine.enable_function_cache.to_string());
        vars.insert("CURSED_JIT_MAX_CACHED_FUNCTIONS".to_string(), self.engine.max_cached_functions.to_string());
        
        if let Some(ref cpu) = self.engine.target_cpu {
            vars.insert("CURSED_JIT_TARGET_CPU".to_string(), cpu.clone());
        }
        
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
    }

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
             - Debug Mode: {}",
            self.engine.optimization_level,
            self.engine.enable_function_cache,
            self.engine.max_cached_functions,
            self.compilation.hot_path_threshold,
            self.memory.max_jit_memory_mb,
            self.runtime.enable_goroutine_integration,
            self.runtime.enable_gc_integration,
            self.debug.debug_mode
        )
    }
}

/// Convert optimization level string to inkwell OptimizationLevel
pub fn parse_optimization_level(level: &str) -> Result<inkwell::OptimizationLevel, Error> {
    match level.to_lowercase().as_str() {
        "none" => Ok(inkwell::OptimizationLevel::None),
        "less" => Ok(inkwell::OptimizationLevel::Less),
        "default" => Ok(inkwell::OptimizationLevel::Default),
        "aggressive" => Ok(inkwell::OptimizationLevel::Aggressive),
        _ => Err(Error::from_str(&format!("Invalid optimization level: {}", level))),
    }
}

/// Convert Duration to seconds for serialization
pub fn duration_to_secs(duration: Duration) -> u64 {
    duration.as_secs()
}

/// Convert seconds to Duration for deserialization
pub fn secs_to_duration(secs: u64) -> Duration {
    Duration::from_secs(secs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_default_config() {
        let config = JitConfig::default();
        assert_eq!(config.engine.optimization_level, "default");
        assert!(config.engine.enable_function_cache);
        assert_eq!(config.compilation.hot_path_threshold, 100);
        assert!(config.runtime.enable_goroutine_integration);
    }

    #[test]
    fn test_config_validation() {
        let mut config = JitConfig::default();
        assert!(config.validate().is_ok());
        
        // Test invalid optimization level
        config.engine.optimization_level = "invalid".to_string();
        assert!(config.validate().is_err());
        
        // Test invalid threshold
        config.engine.optimization_level = "default".to_string();
        config.compilation.hot_path_threshold = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_development_config() {
        let config = JitConfig::development();
        assert_eq!(config.engine.optimization_level, "none");
        assert!(config.debug.debug_mode);
        assert!(config.debug.verbose_logging);
        assert_eq!(config.compilation.hot_path_threshold, 10);
    }

    #[test]
    fn test_production_config() {
        let config = JitConfig::production();
        assert_eq!(config.engine.optimization_level, "aggressive");
        assert!(!config.debug.debug_mode);
        assert!(!config.debug.verbose_logging);
        assert_eq!(config.memory.max_jit_memory_mb, 500);
    }

    #[test]
    fn test_benchmarking_config() {
        let config = JitConfig::benchmarking();
        assert_eq!(config.engine.optimization_level, "aggressive");
        assert!(!config.compilation.enable_background_compilation);
        assert!(!config.monitoring.enabled);
        assert!(config.debug.enable_timing);
    }

    #[test]
    fn test_config_serialization() {
        let config = JitConfig::default();
        
        // Test TOML serialization
        let toml_str = toml::to_string(&config).unwrap();
        let deserialized: JitConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(config.engine.optimization_level, deserialized.engine.optimization_level);
        
        // Test JSON serialization
        let json_str = serde_json::to_string(&config).unwrap();
        let deserialized: JitConfig = serde_json::from_str(&json_str).unwrap();
        assert_eq!(config.compilation.hot_path_threshold, deserialized.compilation.hot_path_threshold);
    }

    #[test]
    fn test_config_file_operations() {
        let config = JitConfig::development();
        let temp_dir = tempdir().unwrap();
        
        // Test TOML file operations
        let toml_path = temp_dir.path().join("config.toml");
        config.save_to_toml_file(&toml_path).unwrap();
        let loaded_config = JitConfig::from_toml_file(&toml_path).unwrap();
        assert_eq!(config.engine.optimization_level, loaded_config.engine.optimization_level);
        
        // Test JSON file operations
        let json_path = temp_dir.path().join("config.json");
        config.save_to_json_file(&json_path).unwrap();
        let loaded_config = JitConfig::from_json_file(&json_path).unwrap();
        assert_eq!(config.debug.debug_mode, loaded_config.debug.debug_mode);
    }

    #[test]
    fn test_config_merging() {
        let mut base_config = JitConfig::default();
        let override_config = JitConfig::development();
        
        let original_opt_level = base_config.engine.optimization_level.clone();
        base_config.merge(&override_config);
        
        // Should use development config values
        assert_eq!(base_config.engine.optimization_level, "none");
        assert!(base_config.debug.debug_mode);
        assert_ne!(base_config.engine.optimization_level, original_opt_level);
    }

    #[test]
    fn test_env_var_loading() {
        std::env::set_var("CURSED_JIT_OPTIMIZATION_LEVEL", "aggressive");
        std::env::set_var("CURSED_JIT_ENABLE_CACHE", "false");
        std::env::set_var("CURSED_JIT_HOT_PATH_THRESHOLD", "50");
        std::env::set_var("CURSED_JIT_DEBUG_MODE", "true");
        
        let config = JitConfig::from_env().unwrap();
        assert_eq!(config.engine.optimization_level, "aggressive");
        assert!(!config.engine.enable_function_cache);
        assert_eq!(config.compilation.hot_path_threshold, 50);
        assert!(config.debug.debug_mode);
        
        // Clean up
        std::env::remove_var("CURSED_JIT_OPTIMIZATION_LEVEL");
        std::env::remove_var("CURSED_JIT_ENABLE_CACHE");
        std::env::remove_var("CURSED_JIT_HOT_PATH_THRESHOLD");
        std::env::remove_var("CURSED_JIT_DEBUG_MODE");
    }

    #[test]
    fn test_optimization_level_parsing() {
        assert!(parse_optimization_level("none").is_ok());
        assert!(parse_optimization_level("AGGRESSIVE").is_ok());
        assert!(parse_optimization_level("invalid").is_err());
    }

    #[test]
    fn test_config_summary() {
        let config = JitConfig::development();
        let summary = config.summary();
        assert!(summary.contains("Optimization Level: none"));
        assert!(summary.contains("Debug Mode: true"));
        assert!(summary.contains("Hot Path Threshold: 10"));
    }
}
