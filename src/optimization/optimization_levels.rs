/// CURSED Compiler Optimization Levels
/// 
/// Provides standardized optimization levels (-O0, -O1, -O2, -O3) with
/// comprehensive optimization pass configuration and performance tuning.

use crate::error::{Error, Result};
use std::time::Duration;
use tracing::{debug, info, instrument};

/// Standard optimization levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationLevel {
    /// No optimization (-O0) - fastest compilation, debugging friendly
    None,
    /// Basic optimization (-O1) - minimal optimizations, good for development
    Basic,
    /// Standard optimization (-O2) - balanced optimization, good for production
    Standard,
    /// Aggressive optimization (-O3) - maximum performance, slower compilation
    Aggressive,
}

impl Default for OptimizationLevel {
    fn default() -> Self {
        Self::Standard
    }
}

impl std::fmt::Display for OptimizationLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "O0"),
            Self::Basic => write!(f, "O1"),
            Self::Standard => write!(f, "O2"),
            Self::Aggressive => write!(f, "O3"),
        }
    }
}

impl std::str::FromStr for OptimizationLevel {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "0" | "o0" | "none" => Ok(Self::None),
            "1" | "o1" | "basic" => Ok(Self::Basic),
            "2" | "o2" | "standard" => Ok(Self::Standard),
            "3" | "o3" | "aggressive" => Ok(Self::Aggressive),
            _ => Err(Error::InvalidInput(format!("Invalid optimization level: {}", s))),
        }
    }
}

/// Optimization configuration for a specific level
#[derive(Debug, Clone)]
pub struct LevelConfig {
    /// Optimization level
    pub level: OptimizationLevel,
    /// Enable function inlining
    pub enable_inlining: bool,
    /// Maximum inline function size
    pub max_inline_size: usize,
    /// Enable dead code elimination
    pub enable_dead_code_elimination: bool,
    /// Enable constant folding
    pub enable_constant_folding: bool,
    /// Enable constant propagation
    pub enable_constant_propagation: bool,
    /// Enable common subexpression elimination
    pub enable_cse: bool,
    /// Enable loop optimizations
    pub enable_loop_optimization: bool,
    /// Maximum loop unroll count
    pub max_unroll_count: usize,
    /// Enable vectorization
    pub enable_vectorization: bool,
    /// Enable tail call optimization
    pub enable_tail_calls: bool,
    /// Enable interprocedural optimization
    pub enable_ipo: bool,
    /// Enable link-time optimization
    pub enable_lto: bool,
    /// Enable fast math optimizations
    pub enable_fast_math: bool,
    /// Enable memory optimization
    pub enable_memory_optimization: bool,
    /// Enable instruction scheduling
    pub enable_instruction_scheduling: bool,
    /// Enable register allocation optimization
    pub enable_register_optimization: bool,
    /// Enable branch prediction optimization
    pub enable_branch_optimization: bool,
    /// Enable profile-guided optimization
    pub enable_pgo: bool,
    /// Compilation timeout
    pub timeout: Duration,
    /// Maximum optimization iterations
    pub max_iterations: usize,
}

impl LevelConfig {
    /// Create configuration for optimization level
    pub fn for_level(level: OptimizationLevel) -> Self {
        match level {
            OptimizationLevel::None => Self::none_config(),
            OptimizationLevel::Basic => Self::basic_config(),
            OptimizationLevel::Standard => Self::standard_config(),
            OptimizationLevel::Aggressive => Self::aggressive_config(),
        }
    }

    /// No optimization configuration (-O0)
    fn none_config() -> Self {
        Self {
            level: OptimizationLevel::None,
            enable_inlining: false,
            max_inline_size: 0,
            enable_dead_code_elimination: false,
            enable_constant_folding: false,
            enable_constant_propagation: false,
            enable_cse: false,
            enable_loop_optimization: false,
            max_unroll_count: 0,
            enable_vectorization: false,
            enable_tail_calls: false,
            enable_ipo: false,
            enable_lto: false,
            enable_fast_math: false,
            enable_memory_optimization: false,
            enable_instruction_scheduling: false,
            enable_register_optimization: false,
            enable_branch_optimization: false,
            enable_pgo: false,
            timeout: Duration::from_secs(5),
            max_iterations: 1,
        }
    }

    /// Basic optimization configuration (-O1)
    fn basic_config() -> Self {
        Self {
            level: OptimizationLevel::Basic,
            enable_inlining: true,
            max_inline_size: 50,
            enable_dead_code_elimination: true,
            enable_constant_folding: true,
            enable_constant_propagation: true,
            enable_cse: true,
            enable_loop_optimization: false,
            max_unroll_count: 0,
            enable_vectorization: false,
            enable_tail_calls: true,
            enable_ipo: false,
            enable_lto: false,
            enable_fast_math: false,
            enable_memory_optimization: true,
            enable_instruction_scheduling: false,
            enable_register_optimization: true,
            enable_branch_optimization: false,
            enable_pgo: false,
            timeout: Duration::from_secs(15),
            max_iterations: 3,
        }
    }

    /// Standard optimization configuration (-O2)
    fn standard_config() -> Self {
        Self {
            level: OptimizationLevel::Standard,
            enable_inlining: true,
            max_inline_size: 250,
            enable_dead_code_elimination: true,
            enable_constant_folding: true,
            enable_constant_propagation: true,
            enable_cse: true,
            enable_loop_optimization: true,
            max_unroll_count: 4,
            enable_vectorization: true,
            enable_tail_calls: true,
            enable_ipo: true,
            enable_lto: false,
            enable_fast_math: false,
            enable_memory_optimization: true,
            enable_instruction_scheduling: true,
            enable_register_optimization: true,
            enable_branch_optimization: true,
            enable_pgo: false,
            timeout: Duration::from_secs(60),
            max_iterations: 5,
        }
    }

    /// Aggressive optimization configuration (-O3)
    fn aggressive_config() -> Self {
        Self {
            level: OptimizationLevel::Aggressive,
            enable_inlining: true,
            max_inline_size: 1000,
            enable_dead_code_elimination: true,
            enable_constant_folding: true,
            enable_constant_propagation: true,
            enable_cse: true,
            enable_loop_optimization: true,
            max_unroll_count: 16,
            enable_vectorization: true,
            enable_tail_calls: true,
            enable_ipo: true,
            enable_lto: true,
            enable_fast_math: true,
            enable_memory_optimization: true,
            enable_instruction_scheduling: true,
            enable_register_optimization: true,
            enable_branch_optimization: true,
            enable_pgo: false,
            timeout: Duration::from_secs(300),
            max_iterations: 10,
        }
    }

    /// Create a custom configuration with specific overrides
    pub fn custom(level: OptimizationLevel) -> LevelConfigBuilder {
        LevelConfigBuilder::new(level)
    }
}

/// Builder for custom optimization configurations
pub struct LevelConfigBuilder {
    config: LevelConfig,
}

impl LevelConfigBuilder {
    pub fn new(level: OptimizationLevel) -> Self {
        Self {
            config: LevelConfig::for_level(level),
        }
    }

    pub fn enable_inlining(mut self, enable: bool) -> Self {
        self.config.enable_inlining = enable;
        self
    }

    pub fn max_inline_size(mut self, size: usize) -> Self {
        self.config.max_inline_size = size;
        self
    }

    pub fn enable_loop_optimization(mut self, enable: bool) -> Self {
        self.config.enable_loop_optimization = enable;
        self
    }

    pub fn max_unroll_count(mut self, count: usize) -> Self {
        self.config.max_unroll_count = count;
        self
    }

    pub fn enable_vectorization(mut self, enable: bool) -> Self {
        self.config.enable_vectorization = enable;
        self
    }

    pub fn enable_lto(mut self, enable: bool) -> Self {
        self.config.enable_lto = enable;
        self
    }

    pub fn enable_fast_math(mut self, enable: bool) -> Self {
        self.config.enable_fast_math = enable;
        self
    }

    pub fn enable_pgo(mut self, enable: bool) -> Self {
        self.config.enable_pgo = enable;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    pub fn max_iterations(mut self, iterations: usize) -> Self {
        self.config.max_iterations = iterations;
        self
    }

    pub fn build(self) -> LevelConfig {
        self.config
    }
}

/// Optimization level manager
pub struct OptimizationLevelManager {
    current_level: OptimizationLevel,
    current_config: LevelConfig,
    custom_overrides: Option<LevelConfig>,
}

impl OptimizationLevelManager {
    /// Create a new optimization level manager
    pub fn new(level: OptimizationLevel) -> Self {
        let config = LevelConfig::for_level(level);
        Self {
            current_level: level,
            current_config: config,
            custom_overrides: None,
        }
    }

    /// Set optimization level
    #[instrument(skip(self))]
    pub fn set_level(&mut self, level: OptimizationLevel) {
        debug!("Setting optimization level to {}", level);
        self.current_level = level;
        self.current_config = LevelConfig::for_level(level);
        self.custom_overrides = None;
    }

    /// Set custom configuration
    #[instrument(skip(self, config))]
    pub fn set_custom_config(&mut self, config: LevelConfig) {
        debug!("Setting custom optimization configuration for level {}", config.level);
        self.current_level = config.level;
        self.current_config = config.clone();
        self.custom_overrides = Some(config);
    }

    /// Get current optimization level
    pub fn current_level(&self) -> OptimizationLevel {
        self.current_level
    }

    /// Get current configuration
    pub fn current_config(&self) -> &LevelConfig {
        &self.current_config
    }

    /// Check if using custom configuration
    pub fn is_custom(&self) -> bool {
        self.custom_overrides.is_some()
    }

    /// Get LLVM optimization level equivalent
    pub fn llvm_optimization_level(&self) -> inkwell::OptimizationLevel {
        match self.current_level {
            OptimizationLevel::None => inkwell::OptimizationLevel::None,
            OptimizationLevel::Basic => inkwell::OptimizationLevel::Less,
            OptimizationLevel::Standard => inkwell::OptimizationLevel::Default,
            OptimizationLevel::Aggressive => inkwell::OptimizationLevel::Aggressive,
        }
    }

    /// Print optimization configuration summary
    pub fn print_summary(&self) {
        let config = &self.current_config;
        println!("🎯 Optimization Configuration Summary:");
        println!("   Level: {} ({})", config.level, 
                if self.is_custom() { "custom" } else { "standard" });
        println!("   Function inlining: {} (max size: {})", 
                config.enable_inlining, config.max_inline_size);
        println!("   Dead code elimination: {}", config.enable_dead_code_elimination);
        println!("   Constant folding/propagation: {} / {}", 
                config.enable_constant_folding, config.enable_constant_propagation);
        println!("   Common subexpression elimination: {}", config.enable_cse);
        println!("   Loop optimization: {} (max unroll: {})", 
                config.enable_loop_optimization, config.max_unroll_count);
        println!("   Vectorization: {}", config.enable_vectorization);
        println!("   Tail call optimization: {}", config.enable_tail_calls);
        println!("   Interprocedural optimization: {}", config.enable_ipo);
        println!("   Link-time optimization: {}", config.enable_lto);
        println!("   Fast math: {}", config.enable_fast_math);
        println!("   Memory optimization: {}", config.enable_memory_optimization);
        println!("   Profile-guided optimization: {}", config.enable_pgo);
        println!("   Timeout: {:?}", config.timeout);
        println!("   Max iterations: {}", config.max_iterations);
    }
}

impl Default for OptimizationLevelManager {
    fn default() -> Self {
        Self::new(OptimizationLevel::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_optimization_level_parsing() {
        assert_eq!(OptimizationLevel::from_str("0").unwrap(), OptimizationLevel::None);
        assert_eq!(OptimizationLevel::from_str("O1").unwrap(), OptimizationLevel::Basic);
        assert_eq!(OptimizationLevel::from_str("o2").unwrap(), OptimizationLevel::Standard);
        assert_eq!(OptimizationLevel::from_str("3").unwrap(), OptimizationLevel::Aggressive);
        
        assert!(OptimizationLevel::from_str("invalid").is_err());
    }

    #[test]
    fn test_level_configs() {
        let o0 = LevelConfig::for_level(OptimizationLevel::None);
        assert!(!o0.enable_inlining);
        assert!(!o0.enable_loop_optimization);
        assert_eq!(o0.max_inline_size, 0);

        let o3 = LevelConfig::for_level(OptimizationLevel::Aggressive);
        assert!(o3.enable_inlining);
        assert!(o3.enable_loop_optimization);
        assert!(o3.enable_lto);
        assert!(o3.enable_fast_math);
        assert!(o3.max_inline_size > 0);
    }

    #[test]
    fn test_custom_builder() {
        let config = LevelConfig::custom(OptimizationLevel::Basic)
            .enable_lto(true)
            .max_inline_size(500)
            .enable_fast_math(true)
            .build();

        assert_eq!(config.level, OptimizationLevel::Basic);
        assert!(config.enable_lto);
        assert_eq!(config.max_inline_size, 500);
        assert!(config.enable_fast_math);
    }

    #[test]
    fn test_optimization_manager() {
        let mut manager = OptimizationLevelManager::new(OptimizationLevel::Standard);
        assert_eq!(manager.current_level(), OptimizationLevel::Standard);
        assert!(!manager.is_custom());

        let custom_config = LevelConfig::custom(OptimizationLevel::Aggressive)
            .enable_lto(false)
            .build();

        manager.set_custom_config(custom_config);
        assert_eq!(manager.current_level(), OptimizationLevel::Aggressive);
        assert!(manager.is_custom());
    }
}
