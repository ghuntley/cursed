/// Optimization Level Configuration System
/// 
/// Provides detailed configuration settings for each optimization level,
/// including pass selection, performance tuning, and target-specific optimizations.

use crate::error::{Error, Result};
use crate::common::optimization_level::OptimizationLevel;
use crate::optimization::config::LlvmPassConfig;
use std::collections::HashMap;
use tracing::{info, debug};

/// Optimization level configuration with detailed settings
#[derive(Debug, Clone)]
pub struct LevelConfig {
    pub level: OptimizationLevel,
    pub description: String,
    pub compilation_speed: CompilationSpeed,
    pub runtime_performance: RuntimePerformance,
    pub code_size_priority: CodeSizePriority,
    pub llvm_passes: LlvmPassConfig,
    pub memory_optimization: MemoryOptimizationLevel,
    pub debug_info_level: DebugInfoLevel,
    pub vectorization_settings: VectorizationSettings,
    pub inlining_settings: InliningSettings,
    pub loop_optimization: LoopOptimizationSettings,
    pub target_specific: TargetSpecificSettings,
}

impl LevelConfig {
    /// Get configuration for optimization level
    pub fn for_level(level: OptimizationLevel) -> Self {
        match level {
            OptimizationLevel::O0 => Self::o0_config(),
            OptimizationLevel::O1 => Self::o1_config(),
            OptimizationLevel::O2 => Self::o2_config(),
            OptimizationLevel::O3 => Self::o3_config(),
            OptimizationLevel::Os => Self::os_config(),
            OptimizationLevel::Oz => Self::oz_config(),
        }
    }
    
    /// O0 configuration - No optimization
    fn o0_config() -> Self {
        Self {
            level: OptimizationLevel::O0,
            description: "No optimization - fastest compilation".to_string(),
            compilation_speed: CompilationSpeed::Fastest,
            runtime_performance: RuntimePerformance::Baseline,
            code_size_priority: CodeSizePriority::None,
            llvm_passes: LlvmPassConfig {
                function_passes: vec!["mem2reg".to_string()],
                module_passes: vec!["strip-dead-prototypes".to_string()],
                enable_vectorization: false,
                enable_loop_unrolling: false,
                enable_inlining: false,
                enable_constant_folding: false,
                enable_dead_code_elimination: false,
                enable_common_subexpression_elimination: false,
                enable_tail_call_optimization: false,
                enable_link_time_optimization: false,
            },
            memory_optimization: MemoryOptimizationLevel::O0,
            debug_info_level: DebugInfoLevel::Full,
            vectorization_settings: VectorizationSettings::disabled(),
            inlining_settings: InliningSettings::minimal(),
            loop_optimization: LoopOptimizationSettings::disabled(),
            target_specific: TargetSpecificSettings::optimized(),
        }
    }
    
    /// O1 configuration - Basic optimization
    fn o1_config() -> Self {
        Self {
            level: OptimizationLevel::O1,
            description: "Basic optimization - good compilation speed".to_string(),
            compilation_speed: CompilationSpeed::Fast,
            runtime_performance: RuntimePerformance::Basic,
            code_size_priority: CodeSizePriority::Low,
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
                enable_vectorization: true,
                enable_loop_unrolling: true,
                enable_inlining: true,
                enable_constant_folding: true,
                enable_dead_code_elimination: true,
                enable_common_subexpression_elimination: true,
                enable_tail_call_optimization: true,
                enable_link_time_optimization: false,
            },
            memory_optimization: MemoryOptimizationLevel::O1,
            debug_info_level: DebugInfoLevel::Standard,
            vectorization_settings: VectorizationSettings::conservative(),
            inlining_settings: InliningSettings::conservative(),
            loop_optimization: LoopOptimizationSettings::enhanced(),
            target_specific: TargetSpecificSettings::optimized(),
        }
    }
    
    /// O2 configuration - Standard optimization
    fn o2_config() -> Self {
        Self {
            level: OptimizationLevel::O2,
            description: "Standard optimization - balanced performance".to_string(),
            compilation_speed: CompilationSpeed::Moderate,
            runtime_performance: RuntimePerformance::Good,
            code_size_priority: CodeSizePriority::Low,
            llvm_passes: LlvmPassConfig {
                function_passes: vec![
                    "mem2reg".to_string(),
                    "instcombine".to_string(),
                    "simplifycfg".to_string(),
                    "reassociate".to_string(),
                    "gvn".to_string(),
                    "basic-aa".to_string(),
                    "sccp".to_string(),
                    "licm".to_string(),
                    "loop-rotate".to_string(),
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
                enable_link_time_optimization: false,
            },
            memory_optimization: MemoryOptimizationLevel::O2,
            debug_info_level: DebugInfoLevel::Standard,
            vectorization_settings: VectorizationSettings::standard(),
            inlining_settings: InliningSettings::standard(),
            loop_optimization: LoopOptimizationSettings::enhanced_standard(),
            target_specific: TargetSpecificSettings::optimized(),
        }
    }
    
    /// O3 configuration - Aggressive optimization
    fn o3_config() -> Self {
        Self {
            level: OptimizationLevel::O3,
            description: "Aggressive optimization - maximum performance".to_string(),
            compilation_speed: CompilationSpeed::Slow,
            runtime_performance: RuntimePerformance::Maximum,
            code_size_priority: CodeSizePriority::None,
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
                    "sccp".to_string(),
                    "loop-rotate".to_string(),
                    "loop-unroll".to_string(),
                    "loop-vectorize".to_string(),
                    "slp-vectorizer".to_string(),
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
            },
            memory_optimization: MemoryOptimizationLevel::O3,
            debug_info_level: DebugInfoLevel::Minimal,
            vectorization_settings: VectorizationSettings::aggressive(),
            inlining_settings: InliningSettings::aggressive(),
            loop_optimization: LoopOptimizationSettings::super_aggressive(),
            target_specific: TargetSpecificSettings::aggressive(),
        }
    }
    
    /// Os configuration - Optimize for size
    fn os_config() -> Self {
        Self {
            level: OptimizationLevel::Os,
            description: "Size optimization - minimize code size".to_string(),
            compilation_speed: CompilationSpeed::Moderate,
            runtime_performance: RuntimePerformance::Good,
            code_size_priority: CodeSizePriority::High,
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
                enable_vectorization: false,
                enable_loop_unrolling: false,
                enable_inlining: true,
                enable_constant_folding: true,
                enable_dead_code_elimination: true,
                enable_common_subexpression_elimination: true,
                enable_tail_call_optimization: true,
                enable_link_time_optimization: true,
            },
            memory_optimization: MemoryOptimizationLevel::Os,
            debug_info_level: DebugInfoLevel::Minimal,
            vectorization_settings: VectorizationSettings::size_optimized(),
            inlining_settings: InliningSettings::size_optimized(),
            loop_optimization: LoopOptimizationSettings::size_optimized(),
            target_specific: TargetSpecificSettings::size_optimized(),
        }
    }
    
    /// Oz configuration - Aggressively optimize for size
    fn oz_config() -> Self {
        Self {
            level: OptimizationLevel::OsAggressive,
            description: "Aggressive size optimization - minimize code size aggressively".to_string(),
            compilation_speed: CompilationSpeed::Slow,
            runtime_performance: RuntimePerformance::Basic,
            code_size_priority: CodeSizePriority::Maximum,
            llvm_passes: LlvmPassConfig {
                function_passes: vec![
                    "mem2reg".to_string(),
                    "instcombine".to_string(),
                    "simplifycfg".to_string(),
                    "gvn".to_string(),
                ],
                module_passes: vec![
                    "globalopt".to_string(),
                    "globaldce".to_string(),
                    "constmerge".to_string(),
                    "mergefunc".to_string(),
                    "strip".to_string(),
                    "strip-debug-declare".to_string(),
                ],
                enable_vectorization: false,
                enable_loop_unrolling: false,
                enable_inlining: true,
                enable_constant_folding: true,
                enable_dead_code_elimination: true,
                enable_common_subexpression_elimination: true,
                enable_tail_call_optimization: true,
                enable_link_time_optimization: true,
            },
            memory_optimization: MemoryOptimizationLevel::Os,
            debug_info_level: DebugInfoLevel::None,
            vectorization_settings: VectorizationSettings::disabled(),
            inlining_settings: InliningSettings::size_aggressive(),
            loop_optimization: LoopOptimizationSettings::size_optimized(),
            target_specific: TargetSpecificSettings::size_aggressive(),
        }
    }
    
    /// Get pass pipeline for this configuration
    pub fn get_pass_pipeline(&self) -> PassPipeline {
        PassPipeline {
            function_passes: self.llvm_passes.function_passes.clone(),
            module_passes: self.llvm_passes.module_passes.clone(),
            optimization_level: self.level,
            custom_passes: Vec::new(),
        }
    }
    
    /// Check if level enables specific optimization
    pub fn enables_optimization(&self, optimization: &str) -> bool {
        match optimization {
            "vectorization" => self.llvm_passes.enable_vectorization,
            "inlining" => self.llvm_passes.enable_inlining,
            "loop_unrolling" => self.llvm_passes.enable_loop_unrolling,
            "lto" => self.llvm_passes.enable_link_time_optimization,
            "dead_code_elimination" => self.llvm_passes.enable_dead_code_elimination,
            _ => false,
        }
    }
    
    /// Get compilation time estimate
    pub fn get_compilation_time_factor(&self) -> f64 {
        match self.compilation_speed {
            CompilationSpeed::Fastest => 1.0,
            CompilationSpeed::Fast => 1.5,
            CompilationSpeed::Moderate => 2.5,
            CompilationSpeed::Slow => 4.0,
        }
    }
    
    /// Get expected performance improvement
    pub fn get_performance_factor(&self) -> f64 {
        match self.runtime_performance {
            RuntimePerformance::Baseline => 1.0,
            RuntimePerformance::Basic => 1.2,
            RuntimePerformance::Good => 1.8,
            RuntimePerformance::Maximum => 3.0,
        }
    }
}

/// Optimization level settings container
#[derive(Debug, Clone)]
pub struct OptimizationSettings {
    pub level_configs: HashMap<OptimizationLevel, LevelConfig>,
    pub default_level: OptimizationLevel,
}

impl OptimizationSettings {
    /// Create new optimization settings
    pub fn new() -> Self {
        let mut level_configs = HashMap::new();
        
        // Initialize all optimization levels
        for level in &[
            OptimizationLevel::O0,
            OptimizationLevel::O1,
            OptimizationLevel::O2,
            OptimizationLevel::O3,
            OptimizationLevel::Os,
            OptimizationLevel::OsAggressive,
        ] {
            level_configs.insert(*level, LevelConfig::for_level(*level));
        }
        
        Self {
            level_configs,
            default_level: OptimizationLevel::O2,
        }
    }
    
    /// Get configuration for level
    pub fn get_config(&self, level: OptimizationLevel) -> Result<&LevelConfig> {
        self.level_configs.get(&level)
            .ok_or_else(|| Error::General(format!("No configuration for optimization level: {:?}", level)))
    }
    
    /// Get all available levels
    pub fn get_available_levels(&self) -> Vec<OptimizationLevel> {
        self.level_configs.keys().cloned().collect()
    }
    
    /// Get optimization comparison
    pub fn compare_levels(&self, level1: OptimizationLevel, level2: OptimizationLevel) -> Result<LevelComparison> {
        let config1 = self.get_config(level1)?;
        let config2 = self.get_config(level2)?;
        
        Ok(LevelComparison {
            level1,
            level2,
            compilation_time_ratio: config2.get_compilation_time_factor() / config1.get_compilation_time_factor(),
            performance_ratio: config2.get_performance_factor() / config1.get_performance_factor(),
            code_size_difference: Self::compare_code_size_priority(config1.code_size_priority, config2.code_size_priority),
        })
    }
    
    /// Compare code size priorities
    fn compare_code_size_priority(priority1: CodeSizePriority, priority2: CodeSizePriority) -> i8 {
        let score1 = match priority1 {
            CodeSizePriority::None => 0,
            CodeSizePriority::Low => 1,
            CodeSizePriority::High => 2,
            CodeSizePriority::Maximum => 3,
        };
        
        let score2 = match priority2 {
            CodeSizePriority::None => 0,
            CodeSizePriority::Low => 1,
            CodeSizePriority::High => 2,
            CodeSizePriority::Maximum => 3,
        };
        
        score2 - score1
    }
}

impl Default for OptimizationSettings {
    fn default() -> Self {
        Self::new()
    }
}

/// Compilation speed priority
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompilationSpeed {
    Fastest,
    Fast,
    Moderate,
    Slow,
}

/// Runtime performance priority
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RuntimePerformance {
    Baseline,
    Basic,
    Good,
    Maximum,
}

/// Code size optimization priority
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CodeSizePriority {
    None,
    Low,
    High,
    Maximum,
}

/// Memory optimization level
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemoryOptimizationLevel {
    None,
    Basic,
    Standard,
    Aggressive,
    Size,
}

/// Debug information level
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DebugInfoLevel {
    None,
    Minimal,
    Standard,
    Full,
}

/// Vectorization settings
#[derive(Debug, Clone)]
pub struct VectorizationSettings {
    pub enable_loop_vectorization: bool,
    pub enable_slp_vectorization: bool,
    pub vectorization_threshold: usize,
    pub max_vector_width: usize,
}

impl VectorizationSettings {
    pub fn disabled() -> Self {
        Self {
            enable_loop_vectorization: false,
            enable_slp_vectorization: false,
            vectorization_threshold: 0,
            max_vector_width: 1,
        }
    }
    
    pub fn conservative() -> Self {
        Self {
            enable_loop_vectorization: true,
            enable_slp_vectorization: false,
            vectorization_threshold: 8,
            max_vector_width: 8,
        }
    }
    
    pub fn standard() -> Self {
        Self {
            enable_loop_vectorization: true,
            enable_slp_vectorization: true,
            vectorization_threshold: 4,
            max_vector_width: 16,
        }
    }
    
    pub fn aggressive() -> Self {
        Self {
            enable_loop_vectorization: true,
            enable_slp_vectorization: true,
            vectorization_threshold: 2,
            max_vector_width: 32,
        }
    }
    
    pub fn size_optimized() -> Self {
        Self {
            enable_loop_vectorization: false,
            enable_slp_vectorization: false,
            vectorization_threshold: 8,
            max_vector_width: 8,
        }
    }
}

/// Inlining settings
#[derive(Debug, Clone)]
pub struct InliningSettings {
    pub enable_always_inline: bool,
    pub inline_threshold: usize,
    pub max_inline_depth: usize,
    pub size_threshold: usize,
}

impl InliningSettings {
    pub fn minimal() -> Self {
        Self {
            enable_always_inline: false,
            inline_threshold: 50,
            max_inline_depth: 2,
            size_threshold: 100,
        }
    }
    
    pub fn conservative() -> Self {
        Self {
            enable_always_inline: true,
            inline_threshold: 100,
            max_inline_depth: 3,
            size_threshold: 200,
        }
    }
    
    pub fn standard() -> Self {
        Self {
            enable_always_inline: true,
            inline_threshold: 225,
            max_inline_depth: 4,
            size_threshold: 500,
        }
    }
    
    pub fn aggressive() -> Self {
        Self {
            enable_always_inline: true,
            inline_threshold: 500,
            max_inline_depth: 8,
            size_threshold: 1000,
        }
    }
    
    pub fn size_optimized() -> Self {
        Self {
            enable_always_inline: true,
            inline_threshold: 25,
            max_inline_depth: 2,
            size_threshold: 50,
        }
    }
    
    pub fn size_aggressive() -> Self {
        Self {
            enable_always_inline: false,
            inline_threshold: 10,
            max_inline_depth: 1,
            size_threshold: 25,
        }
    }
}

/// Loop optimization settings
#[derive(Debug, Clone)]
pub struct LoopOptimizationSettings {
    pub enable_loop_unrolling: bool,
    pub unroll_threshold: usize,
    pub enable_loop_interchange: bool,
    pub enable_loop_rotation: bool,
}

impl LoopOptimizationSettings {
    pub fn disabled() -> Self {
        Self {
            enable_loop_unrolling: false,
            unroll_threshold: 0,
            enable_loop_interchange: false,
            enable_loop_rotation: false,
        }
    }
    
    pub fn basic() -> Self {
        Self {
            enable_loop_unrolling: false,
            unroll_threshold: 0,
            enable_loop_interchange: false,
            enable_loop_rotation: true,
        }
    }
    
    pub fn enhanced() -> Self {
        Self {
            enable_loop_unrolling: true,
            unroll_threshold: 75,
            enable_loop_interchange: false,
            enable_loop_rotation: true,
        }
    }
    
    pub fn standard() -> Self {
        Self {
            enable_loop_unrolling: true,
            unroll_threshold: 150,
            enable_loop_interchange: true,
            enable_loop_rotation: true,
        }
    }
    
    pub fn enhanced_standard() -> Self {
        Self {
            enable_loop_unrolling: true,
            unroll_threshold: 200,
            enable_loop_interchange: true,
            enable_loop_rotation: true,
        }
    }
    
    pub fn aggressive() -> Self {
        Self {
            enable_loop_unrolling: true,
            unroll_threshold: 300,
            enable_loop_interchange: true,
            enable_loop_rotation: true,
        }
    }
    
    pub fn super_aggressive() -> Self {
        Self {
            enable_loop_unrolling: true,
            unroll_threshold: 500,
            enable_loop_interchange: true,
            enable_loop_rotation: true,
        }
    }
    
    pub fn size_optimized() -> Self {
        Self {
            enable_loop_unrolling: false,
            unroll_threshold: 0,
            enable_loop_interchange: false,
            enable_loop_rotation: false,
        }
    }
}

/// Target-specific optimization settings
#[derive(Debug, Clone)]
pub struct TargetSpecificSettings {
    pub enable_cpu_specific: bool,
    pub enable_feature_detection: bool,
    pub target_cpu: Option<String>,
    pub target_features: Vec<String>,
}

impl TargetSpecificSettings {
    pub fn generic() -> Self {
        Self {
            enable_cpu_specific: false,
            enable_feature_detection: false,
            target_cpu: None,
            target_features: Vec::new(),
        }
    }
    
    pub fn optimized() -> Self {
        Self {
            enable_cpu_specific: true,
            enable_feature_detection: true,
            target_cpu: Some("native".to_string()),
            target_features: vec!["sse4.2".to_string()],
        }
    }
    
    pub fn aggressive() -> Self {
        Self {
            enable_cpu_specific: true,
            enable_feature_detection: true,
            target_cpu: Some("native".to_string()),
            target_features: vec!["sse4.2".to_string(), "avx".to_string(), "avx2".to_string()],
        }
    }
    
    pub fn size_optimized() -> Self {
        Self {
            enable_cpu_specific: false,
            enable_feature_detection: false,
            target_cpu: Some("generic".to_string()),
            target_features: Vec::new(),
        }
    }
    
    pub fn size_aggressive() -> Self {
        Self {
            enable_cpu_specific: false,
            enable_feature_detection: false,
            target_cpu: Some("generic".to_string()),
            target_features: Vec::new(),
        }
    }
}

/// Pass pipeline configuration
#[derive(Debug, Clone)]
pub struct PassPipeline {
    pub function_passes: Vec<String>,
    pub module_passes: Vec<String>,
    pub optimization_level: OptimizationLevel,
    pub custom_passes: Vec<String>,
}

impl PassPipeline {
    /// Get total number of passes
    pub fn total_passes(&self) -> usize {
        self.function_passes.len() + self.module_passes.len() + self.custom_passes.len()
    }
    
    /// Get pass names as string
    pub fn get_pass_names(&self) -> String {
        let mut passes = Vec::new();
        passes.extend(self.function_passes.iter().cloned());
        passes.extend(self.module_passes.iter().cloned());
        passes.extend(self.custom_passes.iter().cloned());
        passes.join(",")
    }
}

/// Optimization level comparison
#[derive(Debug, Clone)]
pub struct LevelComparison {
    pub level1: OptimizationLevel,
    pub level2: OptimizationLevel,
    pub compilation_time_ratio: f64,
    pub performance_ratio: f64,
    pub code_size_difference: i8,
}

impl LevelComparison {
    /// Get human-readable comparison
    pub fn get_summary(&self) -> String {
        format!(
            "{:?} vs {:?}: {:.1}x compilation time, {:.1}x performance, {} code size",
            self.level1,
            self.level2,
            self.compilation_time_ratio,
            self.performance_ratio,
            match self.code_size_difference {
                x if x > 0 => format!("+{} size priority", x),
                x if x < 0 => format!("{} size priority", x),
                _ => "same size priority".to_string(),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_level_config_creation() {
        let config = LevelConfig::for_level(OptimizationLevel::O2);
        assert_eq!(config.level, OptimizationLevel::O2);
        assert!(config.enables_optimization("vectorization"));
    }
    
    #[test]
    fn test_optimization_settings() {
        let settings = OptimizationSettings::new();
        let config = settings.get_config(OptimizationLevel::O3).unwrap();
        assert_eq!(config.level, OptimizationLevel::O3);
        assert_eq!(config.compilation_speed, CompilationSpeed::Slow);
        assert_eq!(config.runtime_performance, RuntimePerformance::Maximum);
    }
    
    #[test]
    fn test_level_comparison() {
        let settings = OptimizationSettings::new();
        let comparison = settings.compare_levels(
            OptimizationLevel::O0,
            OptimizationLevel::O3
        ).unwrap();
        
        assert!(comparison.compilation_time_ratio > 1.0);
        assert!(comparison.performance_ratio > 1.0);
    }
    
    #[test]
    fn test_vectorization_settings() {
        let aggressive = VectorizationSettings::aggressive();
        let disabled = VectorizationSettings::disabled();
        
        assert!(aggressive.enable_loop_vectorization);
        assert!(!disabled.enable_loop_vectorization);
        assert!(aggressive.max_vector_width > disabled.max_vector_width);
    }
    
    #[test]
    fn test_inlining_settings() {
        let aggressive = InliningSettings::aggressive();
        let minimal = InliningSettings::minimal();
        
        assert!(aggressive.inline_threshold > minimal.inline_threshold);
        assert!(aggressive.max_inline_depth > minimal.max_inline_depth);
    }
    
    #[test]
    fn test_pass_pipeline() {
        let config = LevelConfig::for_level(OptimizationLevel::O2);
        let pipeline = config.get_pass_pipeline();
        
        assert!(!pipeline.function_passes.is_empty());
        assert!(!pipeline.module_passes.is_empty());
        assert!(pipeline.total_passes() > 0);
    }
    
    #[test]
    fn test_o0_vs_o3_settings() {
        let o0 = LevelConfig::for_level(OptimizationLevel::O0);
        let o3 = LevelConfig::for_level(OptimizationLevel::O3);
        
        // O0 should be faster to compile but slower runtime
        assert!(o0.get_compilation_time_factor() < o3.get_compilation_time_factor());
        assert!(o0.get_performance_factor() < o3.get_performance_factor());
        
        // O3 should enable more optimizations
        assert!(!o0.enables_optimization("vectorization"));
        assert!(o3.enables_optimization("vectorization"));
        assert!(!o0.enables_optimization("lto"));
        assert!(o3.enables_optimization("lto"));
    }
    
    #[test]
    fn test_size_optimization_settings() {
        let os = LevelConfig::for_level(OptimizationLevel::Os);
        let oz = LevelConfig::for_level(OptimizationLevel::OsAggressive);
        
        assert_eq!(os.code_size_priority, CodeSizePriority::High);
        assert_eq!(oz.code_size_priority, CodeSizePriority::Maximum);
        
        // Size optimizations should disable vectorization
        assert!(!os.vectorization_settings.enable_loop_vectorization);
        assert!(!oz.vectorization_settings.enable_loop_vectorization);
        
        // But should still enable inlining for size reduction
        assert!(os.enables_optimization("inlining"));
        assert!(oz.enables_optimization("inlining"));
    }
}
