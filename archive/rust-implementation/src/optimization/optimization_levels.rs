//! Optimization level controls with fine-grained configuration
//! This module provides comprehensive optimization level management for CURSED

use crate::error::{CursedError, Result};
use crate::optimization::config::{OptimizationConfig, OptimizationLevel};
use crate::optimization::production_llvm_optimization::ProductionLlvmOptimizer;
use inkwell::context::Context;
use std::collections::HashMap;
use std::time::Duration;

/// Optimization level controller with advanced configuration
pub struct OptimizationLevelController<'ctx> {
    context: &'ctx Context,
    level_configs: HashMap<OptimizationLevel, OptimizationLevelConfig>,
    current_config: OptimizationConfig,
    performance_thresholds: PerformanceThresholds,
    adaptive_tuning: bool,
}

impl<'ctx> OptimizationLevelController<'ctx> {
    /// Create new optimization level controller
    pub fn new(context: &'ctx Context, config: OptimizationConfig) -> Result<Self> {
        let level_configs = Self::initialize_level_configs();
        let performance_thresholds = PerformanceThresholds::default();
        
        Ok(Self {
            context,
            level_configs,
            current_config: config,
            performance_thresholds,
            adaptive_tuning: false,
        })
    }
    
    /// Initialize optimization level configurations
    fn initialize_level_configs() -> HashMap<OptimizationLevel, OptimizationLevelConfig> {
        let mut configs = HashMap::new();
        
        // O0 - No optimizations
        configs.insert(
            OptimizationLevel::None,
            OptimizationLevelConfig {
                name: "O0".to_string(),
                description: "No optimizations - fastest compilation".to_string(),
                passes: vec![],
                inline_threshold: 0,
                unroll_threshold: 0,
                vectorize: false,
                lto: false,
                expected_compile_time_factor: 1.0,
                expected_performance_factor: 1.0,
                expected_size_factor: 1.0,
                parallelizable: false,
                memory_usage: MemoryUsage::Low,
                stability: StabilityLevel::Stable,
                recommended_for: vec![
                    "Development builds".to_string(),
                    "Debug builds".to_string(),
                    "Fast compilation".to_string(),
                ],
            }
        );
        
        // O1 - Basic optimizations
        configs.insert(
            OptimizationLevel::Less,
            OptimizationLevelConfig {
                name: "O1".to_string(),
                description: "Basic optimizations with reasonable compile time".to_string(),
                passes: vec![
                    "mem2reg".to_string(),
                    "simplifycfg".to_string(),
                    "basic-dce".to_string(),
                    "basic-constant-propagation".to_string(),
                ],
                inline_threshold: 50,
                unroll_threshold: 25,
                vectorize: false,
                lto: false,
                expected_compile_time_factor: 1.2,
                expected_performance_factor: 1.15,
                expected_size_factor: 0.95,
                parallelizable: true,
                memory_usage: MemoryUsage::Low,
                stability: StabilityLevel::Stable,
                recommended_for: vec![
                    "Development with basic optimization".to_string(),
                    "CI builds".to_string(),
                    "Incremental compilation".to_string(),
                ],
            }
        );
        
        // O2 - Standard optimizations
        configs.insert(
            OptimizationLevel::Default,
            OptimizationLevelConfig {
                name: "O2".to_string(),
                description: "Standard optimizations - good balance of compile time and performance".to_string(),
                passes: vec![
                    "mem2reg".to_string(),
                    "simplifycfg".to_string(),
                    "instcombine".to_string(),
                    "reassociate".to_string(),
                    "gvn".to_string(),
                    "sccp".to_string(),
                    "basic-inlining".to_string(),
                    "loop-optimizations".to_string(),
                    "advanced-constant-propagation".to_string(),
                    "enhanced-dce".to_string(),
                ],
                inline_threshold: 225,
                unroll_threshold: 150,
                vectorize: true,
                lto: false,
                expected_compile_time_factor: 1.5,
                expected_performance_factor: 1.3,
                expected_size_factor: 0.9,
                parallelizable: true,
                memory_usage: MemoryUsage::Medium,
                stability: StabilityLevel::Stable,
                recommended_for: vec![
                    "Production builds".to_string(),
                    "Release builds".to_string(),
                    "General purpose optimization".to_string(),
                ],
            }
        );
        
        // O3 - Aggressive optimizations
        configs.insert(
            OptimizationLevel::Aggressive,
            OptimizationLevelConfig {
                name: "O3".to_string(),
                description: "Aggressive optimizations - maximum performance".to_string(),
                passes: vec![
                    "mem2reg".to_string(),
                    "simplifycfg".to_string(),
                    "instcombine".to_string(),
                    "reassociate".to_string(),
                    "gvn".to_string(),
                    "sccp".to_string(),
                    "aggressive-inlining".to_string(),
                    "loop-optimizations".to_string(),
                    "aggressive-loop-optimizations".to_string(),
                    "advanced-constant-propagation".to_string(),
                    "enhanced-dce".to_string(),
                    "jump-threading".to_string(),
                    "tail-call-optimization".to_string(),
                    "sroa".to_string(),
                    "function-specialization".to_string(),
                    "ipo".to_string(),
                    "wpo".to_string(),
                ],
                inline_threshold: 275,
                unroll_threshold: 200,
                vectorize: true,
                lto: true,
                expected_compile_time_factor: 2.0,
                expected_performance_factor: 1.5,
                expected_size_factor: 1.1,
                parallelizable: true,
                memory_usage: MemoryUsage::High,
                stability: StabilityLevel::Stable,
                recommended_for: vec![
                    "Performance-critical applications".to_string(),
                    "Final production builds".to_string(),
                    "Benchmarking".to_string(),
                ],
            }
        );
        
        // Os - Size optimizations
        configs.insert(
            OptimizationLevel::Size,
            OptimizationLevelConfig {
                name: "Os".to_string(),
                description: "Optimize for size - smaller binaries".to_string(),
                passes: vec![
                    "mem2reg".to_string(),
                    "simplifycfg".to_string(),
                    "size-focused-dce".to_string(),
                    "size-focused-constant-propagation".to_string(),
                    "mergefunc".to_string(),
                    "deadargelim".to_string(),
                    "strip".to_string(),
                    "size-focused-inlining".to_string(),
                ],
                inline_threshold: 75,
                unroll_threshold: 50,
                vectorize: false,
                lto: true,
                expected_compile_time_factor: 1.3,
                expected_performance_factor: 1.1,
                expected_size_factor: 0.8,
                parallelizable: true,
                memory_usage: MemoryUsage::Low,
                stability: StabilityLevel::Stable,
                recommended_for: vec![
                    "Embedded systems".to_string(),
                    "WebAssembly targets".to_string(),
                    "Size-constrained environments".to_string(),
                ],
            }
        );
        
        // Oz - Aggressive size optimizations
        configs.insert(
            OptimizationLevel::SizeZ,
            OptimizationLevelConfig {
                name: "Oz".to_string(),
                description: "Aggressive size optimization - smallest binaries".to_string(),
                passes: vec![
                    "mem2reg".to_string(),
                    "simplifycfg".to_string(),
                    "size-focused-dce".to_string(),
                    "size-focused-constant-propagation".to_string(),
                    "mergefunc".to_string(),
                    "aggressive-mergefunc".to_string(),
                    "deadargelim".to_string(),
                    "aggressive-deadargelim".to_string(),
                    "strip".to_string(),
                    "string-deduplication".to_string(),
                    "constant-merging".to_string(),
                    "outline-functions".to_string(),
                ],
                inline_threshold: 25,
                unroll_threshold: 0,
                vectorize: false,
                lto: true,
                expected_compile_time_factor: 1.8,
                expected_performance_factor: 1.0,
                expected_size_factor: 0.7,
                parallelizable: true,
                memory_usage: MemoryUsage::Medium,
                stability: StabilityLevel::Stable,
                recommended_for: vec![
                    "Microcontrollers".to_string(),
                    "Extreme size constraints".to_string(),
                    "Embedded WebAssembly".to_string(),
                ],
            }
        );
        
        configs
    }
    
    /// Get configuration for optimization level
    pub fn get_level_config(&self, level: &OptimizationLevel) -> Result<&OptimizationLevelConfig> {
        self.level_configs.get(level)
            .ok_or_else(|| CursedError::runtime_error(&format!("Unknown optimization level: {:?}", level)))
    }
    
    /// Create optimizer for specific level
    pub fn create_optimizer_for_level(&self, level: OptimizationLevel) -> Result<OptimizationConfig> {
        let level_config = self.get_level_config(&level)?;
        
        let mut config = self.current_config.clone();
        config.level = level;
        config.inline_threshold = level_config.inline_threshold;
        config.unroll_threshold = level_config.unroll_threshold;
        config.vectorize = level_config.vectorize;
        config.lto = level_config.lto;
        config.custom_passes = level_config.passes.clone();
        
        Ok(config)
    }
    
    /// Get all available optimization levels
    pub fn get_available_levels(&self) -> Vec<OptimizationLevel> {
        self.level_configs.keys().cloned().collect()
    }
    
    /// Get level recommendations based on context
    pub fn get_level_recommendations(&self, context: &BuildContext) -> Result<Vec<LevelRecommendation>> {
        let mut recommendations = Vec::new();
        
        for (level, config) in &self.level_configs {
            let mut score = 0.0;
            let mut reasons = Vec::new();
            
            // Score based on build context
            match context.build_type {
                BuildType::Debug => {
                    if *level == OptimizationLevel::None {
                        score += 10.0;
                        reasons.push("Fastest compilation for debug builds".to_string());
                    }
                }
                BuildType::Release => {
                    if *level == OptimizationLevel::Default || *level == OptimizationLevel::Aggressive {
                        score += 8.0;
                        reasons.push("Good performance for release builds".to_string());
                    }
                }
                BuildType::Size => {
                    if *level == OptimizationLevel::Size || *level == OptimizationLevel::SizeZ {
                        score += 10.0;
                        reasons.push("Optimized for binary size".to_string());
                    }
                }
            }
            
            // Score based on target platform
            match context.target_platform {
                TargetPlatform::Desktop => {
                    if *level == OptimizationLevel::Default || *level == OptimizationLevel::Aggressive {
                        score += 5.0;
                        reasons.push("Good for desktop applications".to_string());
                    }
                }
                TargetPlatform::Embedded => {
                    if *level == OptimizationLevel::Size || *level == OptimizationLevel::SizeZ {
                        score += 8.0;
                        reasons.push("Size-optimized for embedded systems".to_string());
                    }
                }
                TargetPlatform::WebAssembly => {
                    if *level == OptimizationLevel::Size {
                        score += 7.0;
                        reasons.push("Balanced size/performance for WebAssembly".to_string());
                    }
                }
                TargetPlatform::Server => {
                    if *level == OptimizationLevel::Aggressive {
                        score += 9.0;
                        reasons.push("Maximum performance for server applications".to_string());
                    }
                }
            }
            
            // Score based on compile time constraints
            if context.compile_time_budget < Duration::from_secs(30) {
                if *level == OptimizationLevel::None || *level == OptimizationLevel::Less {
                    score += 5.0;
                    reasons.push("Fast compilation within time budget".to_string());
                }
            } else if context.compile_time_budget > Duration::from_secs(300) {
                if *level == OptimizationLevel::Aggressive {
                    score += 3.0;
                    reasons.push("Sufficient time for aggressive optimization".to_string());
                }
            }
            
            // Score based on memory constraints
            if context.memory_budget < 1024 * 1024 * 1024 { // < 1GB
                if config.memory_usage == MemoryUsage::Low {
                    score += 3.0;
                    reasons.push("Low memory usage fits budget".to_string());
                }
            }
            
            if score > 0.0 {
                recommendations.push(LevelRecommendation {
                    level: level.clone(),
                    score,
                    reasons,
                    config: config.clone(),
                });
            }
        }
        
        // Sort by score
        recommendations.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        
        Ok(recommendations)
    }
    
    /// Enable adaptive tuning based on performance metrics
    pub fn enable_adaptive_tuning(&mut self, enable: bool) {
        self.adaptive_tuning = enable;
    }
    
    /// Update performance thresholds
    pub fn update_performance_thresholds(&mut self, thresholds: PerformanceThresholds) {
        self.performance_thresholds = thresholds;
    }
    
    /// Suggest optimal level based on performance history
    pub fn suggest_optimal_level(&self, performance_history: &[PerformanceMetric]) -> Result<OptimizationLevel> {
        if performance_history.is_empty() {
            return Ok(OptimizationLevel::Default);
        }
        
        let mut best_level = OptimizationLevel::Default;
        let mut best_score = 0.0;
        
        for (level, _config) in &self.level_configs {
            let score = self.calculate_level_score(level, performance_history)?;
            if score > best_score {
                best_score = score;
                best_level = level.clone();
            }
        }
        
        Ok(best_level)
    }
    
    /// Calculate score for optimization level based on performance history
    fn calculate_level_score(&self, level: &OptimizationLevel, history: &[PerformanceMetric]) -> Result<f64> {
        let config = self.get_level_config(level)?;
        let mut score = 0.0;
        
        for metric in history {
            // Score based on compile time vs target
            let compile_time_score = if metric.compile_time <= self.performance_thresholds.max_compile_time {
                1.0
            } else {
                0.5
            };
            
            // Score based on performance improvement
            let performance_score = metric.performance_improvement / self.performance_thresholds.min_performance_improvement;
            
            // Score based on size reduction
            let size_score = metric.size_reduction / self.performance_thresholds.min_size_reduction;
            
            // Weighted score
            let weighted_score = 
                (compile_time_score * 0.3) +
                (performance_score * 0.5) +
                (size_score * 0.2);
            
            score += weighted_score;
        }
        
        // Average score
        Ok(score / history.len() as f64)
    }
    
    /// Create custom optimization level
    pub fn create_custom_level(&mut self, name: String, passes: Vec<String>, config: CustomLevelConfig) -> Result<OptimizationLevel> {
        let mut pass_map = HashMap::new();
        for pass in passes {
            pass_map.insert(pass, true);
        }
        
        let custom_level = OptimizationLevel::Custom(pass_map);
        
        let level_config = OptimizationLevelConfig {
            name,
            description: config.description,
            passes: config.passes,
            inline_threshold: config.inline_threshold,
            unroll_threshold: config.unroll_threshold,
            vectorize: config.vectorize,
            lto: config.lto,
            expected_compile_time_factor: config.expected_compile_time_factor,
            expected_performance_factor: config.expected_performance_factor,
            expected_size_factor: config.expected_size_factor,
            parallelizable: config.parallelizable,
            memory_usage: config.memory_usage,
            stability: config.stability,
            recommended_for: config.recommended_for,
        };
        
        self.level_configs.insert(custom_level.clone(), level_config);
        
        Ok(custom_level)
    }
    
    /// Validate optimization level configuration
    pub fn validate_level_config(&self, level: &OptimizationLevel) -> Result<ValidationResult> {
        let config = self.get_level_config(level)?;
        let mut warnings = Vec::new();
        let mut errors = Vec::new();
        
        // Validate thresholds
        if config.inline_threshold > 1000 {
            warnings.push("High inline threshold may increase compile time significantly".to_string());
        }
        
        if config.unroll_threshold > 500 {
            warnings.push("High unroll threshold may increase code size significantly".to_string());
        }
        
        // Validate pass compatibility
        if config.passes.contains(&"vectorize".to_string()) && !config.vectorize {
            errors.push("Vectorize pass enabled but vectorization disabled".to_string());
        }
        
        if config.lto && config.expected_compile_time_factor < 1.5 {
            warnings.push("LTO enabled but compile time factor seems optimistic".to_string());
        }
        
        // Validate memory usage vs expected factors
        if config.memory_usage == MemoryUsage::Low && config.expected_performance_factor > 1.3 {
            warnings.push("Low memory usage with high performance expectations may not be realistic".to_string());
        }
        
        Ok(ValidationResult {
            valid: errors.is_empty(),
            warnings,
            errors,
        })
    }
    
    /// Get optimization level comparison
    pub fn compare_levels(&self, levels: &[OptimizationLevel]) -> Result<LevelComparison> {
        let mut comparison = LevelComparison {
            levels: Vec::new(),
            comparison_matrix: HashMap::new(),
        };
        
        for level in levels {
            let config = self.get_level_config(level)?;
            comparison.levels.push(LevelComparisonEntry {
                level: level.clone(),
                config: config.clone(),
            });
        }
        
        // Generate comparison matrix
        for (i, level1) in levels.iter().enumerate() {
            for (j, level2) in levels.iter().enumerate() {
                if i != j {
                    let comparison_result = self.compare_two_levels(level1, level2)?;
                    comparison.comparison_matrix.insert(
                        (level1.clone(), level2.clone()),
                        comparison_result
                    );
                }
            }
        }
        
        Ok(comparison)
    }
    
    /// Compare two optimization levels
    fn compare_two_levels(&self, level1: &OptimizationLevel, level2: &OptimizationLevel) -> Result<TwoLevelComparison> {
        let config1 = self.get_level_config(level1)?;
        let config2 = self.get_level_config(level2)?;
        
        Ok(TwoLevelComparison {
            compile_time_ratio: config1.expected_compile_time_factor / config2.expected_compile_time_factor,
            performance_ratio: config1.expected_performance_factor / config2.expected_performance_factor,
            size_ratio: config1.expected_size_factor / config2.expected_size_factor,
            better_for_compile_time: config1.expected_compile_time_factor < config2.expected_compile_time_factor,
            better_for_performance: config1.expected_performance_factor > config2.expected_performance_factor,
            better_for_size: config1.expected_size_factor < config2.expected_size_factor,
            pass_count_difference: config1.passes.len() as i32 - config2.passes.len() as i32,
        })
    }
}

/// Configuration for a specific optimization level
#[derive(Debug, Clone)]
pub struct OptimizationLevelConfig {
    pub name: String,
    pub description: String,
    pub passes: Vec<String>,
    pub inline_threshold: u32,
    pub unroll_threshold: u32,
    pub vectorize: bool,
    pub lto: bool,
    pub expected_compile_time_factor: f64,
    pub expected_performance_factor: f64,
    pub expected_size_factor: f64,
    pub parallelizable: bool,
    pub memory_usage: MemoryUsage,
    pub stability: StabilityLevel,
    pub recommended_for: Vec<String>,
}

/// Memory usage category
#[derive(Debug, Clone, PartialEq)]
pub enum MemoryUsage {
    Low,
    Medium,
    High,
}

/// Stability level
#[derive(Debug, Clone, PartialEq)]
pub enum StabilityLevel {
    Stable,
    Beta,
    Experimental,
}

/// Build context for recommendations
#[derive(Debug, Clone)]
pub struct BuildContext {
    pub build_type: BuildType,
    pub target_platform: TargetPlatform,
    pub compile_time_budget: Duration,
    pub memory_budget: usize,
    pub performance_requirements: PerformanceRequirements,
    pub size_requirements: SizeRequirements,
}

/// Build type
#[derive(Debug, Clone, PartialEq)]
pub enum BuildType {
    Debug,
    Release,
    Size,
}

/// Target platform
#[derive(Debug, Clone, PartialEq)]
pub enum TargetPlatform {
    Desktop,
    Embedded,
    WebAssembly,
    Server,
}

/// Performance requirements
#[derive(Debug, Clone)]
pub struct PerformanceRequirements {
    pub min_improvement: f64,
    pub max_regression: f64,
    pub critical_paths: Vec<String>,
}

/// Size requirements
#[derive(Debug, Clone)]
pub struct SizeRequirements {
    pub max_size: usize,
    pub max_growth: f64,
    pub minimize_size: bool,
}

/// Level recommendation
#[derive(Debug, Clone)]
pub struct LevelRecommendation {
    pub level: OptimizationLevel,
    pub score: f64,
    pub reasons: Vec<String>,
    pub config: OptimizationLevelConfig,
}

/// Performance thresholds
#[derive(Debug, Clone)]
pub struct PerformanceThresholds {
    pub max_compile_time: Duration,
    pub min_performance_improvement: f64,
    pub min_size_reduction: f64,
    pub max_memory_usage: usize,
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            max_compile_time: Duration::from_secs(300),
            min_performance_improvement: 0.1,
            min_size_reduction: 0.05,
            max_memory_usage: 2 * 1024 * 1024 * 1024, // 2GB
        }
    }
}

/// Performance metric
#[derive(Debug, Clone)]
pub struct PerformanceMetric {
    pub level: OptimizationLevel,
    pub compile_time: Duration,
    pub performance_improvement: f64,
    pub size_reduction: f64,
    pub memory_usage: usize,
}

/// Custom level configuration
#[derive(Debug, Clone)]
pub struct CustomLevelConfig {
    pub description: String,
    pub passes: Vec<String>,
    pub inline_threshold: u32,
    pub unroll_threshold: u32,
    pub vectorize: bool,
    pub lto: bool,
    pub expected_compile_time_factor: f64,
    pub expected_performance_factor: f64,
    pub expected_size_factor: f64,
    pub parallelizable: bool,
    pub memory_usage: MemoryUsage,
    pub stability: StabilityLevel,
    pub recommended_for: Vec<String>,
}

/// Validation result
#[derive(Debug)]
pub struct ValidationResult {
    pub valid: bool,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

/// Level comparison
#[derive(Debug)]
pub struct LevelComparison {
    pub levels: Vec<LevelComparisonEntry>,
    pub comparison_matrix: HashMap<(OptimizationLevel, OptimizationLevel), TwoLevelComparison>,
}

/// Level comparison entry
#[derive(Debug)]
pub struct LevelComparisonEntry {
    pub level: OptimizationLevel,
    pub config: OptimizationLevelConfig,
}

/// Two level comparison
#[derive(Debug)]
pub struct TwoLevelComparison {
    pub compile_time_ratio: f64,
    pub performance_ratio: f64,
    pub size_ratio: f64,
    pub better_for_compile_time: bool,
    pub better_for_performance: bool,
    pub better_for_size: bool,
    pub pass_count_difference: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    
    #[test]
    fn test_level_controller_creation() {
        let context = Context::create();
        let config = OptimizationConfig::default();
        
        let controller = OptimizationLevelController::new(&context, config);
        assert!(controller.is_ok());
    }
    
    #[test]
    fn test_level_configurations() {
        let context = Context::create();
        let config = OptimizationConfig::default();
        let controller = OptimizationLevelController::new(&context, config).unwrap();
        
        let levels = controller.get_available_levels();
        assert!(!levels.is_empty());
        
        for level in levels {
            let level_config = controller.get_level_config(&level);
            assert!(level_config.is_ok());
        }
    }
    
    #[test]
    fn test_level_recommendations() {
        let context = Context::create();
        let config = OptimizationConfig::default();
        let controller = OptimizationLevelController::new(&context, config).unwrap();
        
        let build_context = BuildContext {
            build_type: BuildType::Release,
            target_platform: TargetPlatform::Desktop,
            compile_time_budget: Duration::from_secs(60),
            memory_budget: 2 * 1024 * 1024 * 1024,
            performance_requirements: PerformanceRequirements {
                min_improvement: 0.2,
                max_regression: 0.0,
                critical_paths: vec!["main".to_string()],
            },
            size_requirements: SizeRequirements {
                max_size: 10 * 1024 * 1024,
                max_growth: 0.1,
                minimize_size: false,
            },
        };
        
        let recommendations = controller.get_level_recommendations(&build_context);
        assert!(recommendations.is_ok());
        
        let recs = recommendations.unwrap();
        assert!(!recs.is_empty());
    }
    
    #[test]
    fn test_optimizer_creation_for_level() {
        let context = Context::create();
        let config = OptimizationConfig::default();
        let controller = OptimizationLevelController::new(&context, config).unwrap();
        
        let optimizer_config = controller.create_optimizer_for_level(OptimizationLevel::Default);
        assert!(optimizer_config.is_ok());
        
        let config = optimizer_config.unwrap();
        assert_eq!(config.level, OptimizationLevel::Default);
    }
    
    #[test]
    fn test_level_validation() {
        let context = Context::create();
        let config = OptimizationConfig::default();
        let controller = OptimizationLevelController::new(&context, config).unwrap();
        
        let validation = controller.validate_level_config(&OptimizationLevel::Default);
        assert!(validation.is_ok());
        
        let result = validation.unwrap();
        assert!(result.valid);
    }
    
    #[test]
    fn test_level_comparison() {
        let context = Context::create();
        let config = OptimizationConfig::default();
        let controller = OptimizationLevelController::new(&context, config).unwrap();
        
        let levels = vec![OptimizationLevel::None, OptimizationLevel::Default, OptimizationLevel::Aggressive];
        let comparison = controller.compare_levels(&levels);
        assert!(comparison.is_ok());
        
        let comp = comparison.unwrap();
        assert_eq!(comp.levels.len(), 3);
        assert!(!comp.comparison_matrix.is_empty());
    }
    
    #[test]
    fn test_custom_level_creation() {
        let context = Context::create();
        let config = OptimizationConfig::default();
        let mut controller = OptimizationLevelController::new(&context, config).unwrap();
        
        let custom_config = CustomLevelConfig {
            description: "Custom test level".to_string(),
            passes: vec!["mem2reg".to_string(), "instcombine".to_string()],
            inline_threshold: 100,
            unroll_threshold: 50,
            vectorize: true,
            lto: false,
            expected_compile_time_factor: 1.2,
            expected_performance_factor: 1.1,
            expected_size_factor: 0.95,
            parallelizable: true,
            memory_usage: MemoryUsage::Medium,
            stability: StabilityLevel::Beta,
            recommended_for: vec!["Testing".to_string()],
        };
        
        let custom_level = controller.create_custom_level(
            "custom-test".to_string(),
            vec!["mem2reg".to_string(), "instcombine".to_string()],
            custom_config
        );
        
        assert!(custom_level.is_ok());
        
        let level = custom_level.unwrap();
        let level_config = controller.get_level_config(&level);
        assert!(level_config.is_ok());
    }
}
