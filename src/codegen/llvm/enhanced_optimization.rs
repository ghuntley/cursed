//! Enhanced LLVM optimization integration for CURSED compiler
//! 
//! This module provides comprehensive optimization pipeline management
//! with improved inlining integration, better pass ordering, and
//! configuration-driven optimization for self-hosting performance.

use crate::error::{CursedError, Result};
use crate::codegen::llvm::optimization::{OptimizationConfig, OptimizationLevel, OptimizationStats};
use crate::codegen::llvm::passes::inlining::{InliningPass, InliningConfig};
use crate::codegen::llvm::passes::{PassConfiguration, PassResult, OptimizationLevel as PassOptLevel};
use inkwell::{
    context::Context,
    module::Module,
    passes::PassManager,
    values::FunctionValue,
    OptimizationLevel as InkwellOptLevel,
    targets::{TargetMachine, Target, RelocMode, CodeModel, InitializationConfig},
};
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};

/// Enhanced optimization manager with improved pass ordering and inlining integration
pub struct EnhancedOptimizationManager<'ctx> {
    context: &'ctx Context,
    config: EnhancedOptimizationConfig,
    stats: Arc<Mutex<OptimizationStats>>,
    pass_cache: HashMap<String, CachedPassResult>,
    optimization_pipeline: OptimizationPipeline,
    target_machine: Option<&'ctx TargetMachine>,
    performance_monitor: PerformanceMonitor,
}

/// Enhanced optimization configuration with comprehensive settings
#[derive(Debug, Clone)]
pub struct EnhancedOptimizationConfig {
    pub base_config: OptimizationConfig,
    
    // Pass ordering configuration
    pub early_passes: Vec<String>,
    pub inlining_passes: Vec<String>,
    pub mid_level_passes: Vec<String>,
    pub late_passes: Vec<String>,
    pub cleanup_passes: Vec<String>,
    
    // Inlining integration
    pub inline_before_optimization: bool,
    pub inline_after_optimization: bool,
    pub recursive_inlining_threshold: u32,
    pub cross_module_inlining: bool,
    
    // Self-hosting optimizations
    pub enable_self_hosting_optimizations: bool,
    pub bootstrap_optimization_level: OptimizationLevel,
    pub compilation_time_budget: Duration,
    pub memory_budget: usize,
    
    // Advanced configuration
    pub enable_profile_guided_optimization: bool,
    pub enable_interprocedural_optimization: bool,
    pub enable_whole_program_optimization: bool,
    pub enable_link_time_optimization: bool,
    
    // Performance tuning
    pub parallel_optimization_threads: Option<usize>,
    pub optimization_cache_size: usize,
    pub enable_fast_math: bool,
    pub vectorization_factor: Option<u32>,
    
    // Debug and profiling
    pub debug_optimization_pipeline: bool,
    pub collect_detailed_metrics: bool,
    pub optimization_timeout: Option<Duration>,
}

impl Default for EnhancedOptimizationConfig {
    fn default() -> Self {
        Self::for_self_hosting()
    }
}

impl EnhancedOptimizationConfig {
    /// Create configuration optimized for self-hosting compiler performance
    pub fn for_self_hosting() -> Self {
        Self {
            base_config: OptimizationConfig::release_config(),
            
            early_passes: vec![
                "mem2reg".to_string(),
                "sroa".to_string(),
                "early-cse".to_string(),
                "simplify-cfg".to_string(),
            ],
            
            inlining_passes: vec![
                "always-inline".to_string(),
                "inline".to_string(),
                "interface-inline".to_string(),
                "generics-inline".to_string(),
            ],
            
            mid_level_passes: vec![
                "gvn".to_string(),
                "sccp".to_string(),
                "instcombine".to_string(),
                "reassociate".to_string(),
                "licm".to_string(),
                "loop-unroll".to_string(),
            ],
            
            late_passes: vec![
                "dce".to_string(),
                "adce".to_string(),
                "tailcallelim".to_string(),
                "jump-threading".to_string(),
            ],
            
            cleanup_passes: vec![
                "strip-dead-prototypes".to_string(),
                "globaldce".to_string(),
                "constmerge".to_string(),
            ],
            
            inline_before_optimization: true,
            inline_after_optimization: false,
            recursive_inlining_threshold: 3,
            cross_module_inlining: true,
            
            enable_self_hosting_optimizations: true,
            bootstrap_optimization_level: OptimizationLevel::O3,
            compilation_time_budget: Duration::from_secs(300),
            memory_budget: 2_000_000_000, // 2GB
            
            enable_profile_guided_optimization: true,
            enable_interprocedural_optimization: true,
            enable_whole_program_optimization: true,
            enable_link_time_optimization: true,
            
            parallel_optimization_threads: Some(num_cpus::get()),
            optimization_cache_size: 10000,
            enable_fast_math: false, // Conservative for compiler correctness
            vectorization_factor: Some(4),
            
            debug_optimization_pipeline: false,
            collect_detailed_metrics: true,
            optimization_timeout: Some(Duration::from_secs(600)),
        }
    }
    
    /// Create configuration for development builds (faster compilation)
    pub fn for_development() -> Self {
        let mut config = Self::for_self_hosting();
        config.base_config = OptimizationConfig::dev_config();
        config.bootstrap_optimization_level = OptimizationLevel::O1;
        config.compilation_time_budget = Duration::from_secs(60);
        config.enable_profile_guided_optimization = false;
        config.enable_interprocedural_optimization = false;
        config.enable_whole_program_optimization = false;
        config.parallel_optimization_threads = Some(2);
        config.optimization_cache_size = 1000;
        config.debug_optimization_pipeline = true;
        config
    }
    
    /// Create configuration for release builds (maximum performance)
    pub fn for_release() -> Self {
        let mut config = Self::for_self_hosting();
        config.bootstrap_optimization_level = OptimizationLevel::O3;
        config.compilation_time_budget = Duration::from_secs(1200);
        config.enable_fast_math = true;
        config.vectorization_factor = Some(8);
        config.recursive_inlining_threshold = 5;
        config
    }
}

/// Optimization pipeline with staged pass execution
#[derive(Debug)]
pub struct OptimizationPipeline {
    stages: Vec<OptimizationStage>,
    current_stage: usize,
    execution_history: Vec<StageExecutionResult>,
    adaptive_thresholds: HashMap<String, f64>,
}

/// A single optimization stage
#[derive(Debug, Clone)]
pub struct OptimizationStage {
    pub name: String,
    pub passes: Vec<String>,
    pub execution_mode: ExecutionMode,
    pub prerequisites: Vec<String>,
    pub metrics_targets: HashMap<String, f64>,
    pub time_budget: Option<Duration>,
}

/// Execution mode for optimization stages
#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionMode {
    Sequential,
    Parallel,
    Adaptive,
}

/// Result of executing an optimization stage
#[derive(Debug, Clone)]
pub struct StageExecutionResult {
    pub stage_name: String,
    pub execution_time: Duration,
    pub passes_executed: usize,
    pub improvements: HashMap<String, f64>,
    pub memory_usage: usize,
    pub success: bool,
}

/// Cached result of a pass execution for performance
#[derive(Debug, Clone)]
struct CachedPassResult {
    pub result: PassResult,
    pub module_hash: u64,
    pub config_hash: u64,
    pub timestamp: Instant,
}

/// Performance monitoring for optimization pipeline
#[derive(Debug)]
pub struct PerformanceMonitor {
    stage_times: HashMap<String, Duration>,
    pass_times: HashMap<String, Duration>,
    memory_usage: HashMap<String, usize>,
    improvement_metrics: HashMap<String, f64>,
    bottlenecks: Vec<PerformanceBottleneck>,
}

/// Identified performance bottleneck
#[derive(Debug, Clone)]
pub struct PerformanceBottleneck {
    pub stage: String,
    pub pass: String,
    pub issue: String,
    pub recommendation: String,
    pub severity: BottleneckSeverity,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BottleneckSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl<'ctx> EnhancedOptimizationManager<'ctx> {
    /// Create a new enhanced optimization manager
    pub fn new(context: &'ctx Context, config: EnhancedOptimizationConfig) -> Self {
        let optimization_pipeline = OptimizationPipeline::new(&config);
        
        Self {
            context,
            config,
            stats: Arc::new(Mutex::new(OptimizationStats::default())),
            pass_cache: HashMap::new(),
            optimization_pipeline,
            target_machine: None,
            performance_monitor: PerformanceMonitor::new(),
        }
    }
    
    /// Set target machine for optimization
    pub fn set_target_machine(&mut self, target_machine: &'ctx TargetMachine) {
        self.target_machine = Some(target_machine);
    }
    
    /// Run comprehensive optimization pipeline
    pub fn optimize_module(&mut self, module: &Module<'ctx>) -> Result<EnhancedOptimizationResult> {
        let start_time = Instant::now();
        let mut result = EnhancedOptimizationResult::new();
        
        // Pre-optimization analysis
        let complexity = self.analyze_module_complexity(module)?;
        result.initial_complexity = Some(complexity);
        
        // Apply optimization timeout if configured
        let timeout_start = Instant::now();
        
        // Stage 1: Early optimization passes
        if let Err(e) = self.run_early_passes(module) {
            result.add_warning(format!("Early passes failed: {}", e));
        }
        
        // Stage 2: Inlining optimization (integrated)
        if self.config.inline_before_optimization {
            if let Err(e) = self.run_enhanced_inlining(module) {
                result.add_error(format!("Inlining failed: {}", e));
            }
        }
        
        // Stage 3: Mid-level optimization passes
        if let Err(e) = self.run_mid_level_passes(module) {
            result.add_warning(format!("Mid-level passes failed: {}", e));
        }
        
        // Stage 4: Late optimization passes
        if let Err(e) = self.run_late_passes(module) {
            result.add_warning(format!("Late passes failed: {}", e));
        }
        
        // Stage 5: Cleanup passes
        if let Err(e) = self.run_cleanup_passes(module) {
            result.add_warning(format!("Cleanup passes failed: {}", e));
        }
        
        // Post-optimization inlining if configured
        if self.config.inline_after_optimization {
            if let Err(e) = self.run_enhanced_inlining(module) {
                result.add_warning(format!("Post-optimization inlining failed: {}", e));
            }
        }
        
        // Check timeout
        if let Some(timeout) = self.config.optimization_timeout {
            if timeout_start.elapsed() > timeout {
                result.add_warning("Optimization pipeline timed out".to_string());
            }
        }
        
        // Post-optimization analysis
        let final_complexity = self.analyze_module_complexity(module)?;
        result.final_complexity = Some(final_complexity);
        result.total_time = start_time.elapsed();
        
        // Update performance monitor
        self.performance_monitor.record_optimization(&result);
        
        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.modules_optimized += 1;
            stats.optimization_time += result.total_time;
        }
        
        Ok(result)
    }
    
    /// Run early optimization passes
    fn run_early_passes(&mut self, module: &Module<'ctx>) -> Result<()> {
        let stage_start = Instant::now();
        
        // Create function pass manager for early passes
        let fpm = PassManager::create(module);
        
        // Add early passes based on configuration
        for pass_name in &self.config.early_passes {
            self.add_pass_to_manager(&fpm, pass_name)?;
        }
        
        fpm.initialize();
        
        // Run passes on all functions
        for function in module.get_functions() {
            fpm.run_on(&function);
        }
        
        self.performance_monitor.record_stage_time("early_passes", stage_start.elapsed());
        Ok(())
    }
    
    /// Run enhanced inlining with improved integration
    fn run_enhanced_inlining(&mut self, module: &Module<'ctx>) -> Result<()> {
        let stage_start = Instant::now();
        
        // Configure inlining based on optimization level
        let mut inlining_config = InliningConfig::for_optimization_level(
            match self.config.base_config.level {
                OptimizationLevel::O0 => 0,
                OptimizationLevel::O1 => 1,
                OptimizationLevel::O2 | OptimizationLevel::Default => 2,
                OptimizationLevel::O3 => 3,
                OptimizationLevel::Os | OptimizationLevel::Oz => 2,
            }
        );
        
        // Apply enhanced configuration
        if self.config.enable_self_hosting_optimizations {
            inlining_config.aggressive_inlining = true;
            inlining_config.inline_threshold = inlining_config.inline_threshold.max(400);
            inlining_config.enable_interface_inlining = true;
            inlining_config.enable_generics_inlining = true;
        }
        
        if self.config.cross_module_inlining {
            inlining_config.enable_cross_module_inlining = true;
        }
        
        // Set recursive inlining threshold
        inlining_config.recursive_inline_limit = self.config.recursive_inlining_threshold;
        
        // Run multiple inlining passes for better integration
        for pass_name in &self.config.inlining_passes {
            let mut inlining_pass = InliningPass::with_config(self.context, inlining_config.clone());
            let inlining_result = inlining_pass.run(module)?;
            
            // Log inlining results for debugging
            if self.config.debug_optimization_pipeline {
                eprintln!("Inlining pass '{}': {} functions inlined, {} calls inlined", 
                         pass_name, inlining_result.functions_inlined, inlining_result.total_calls_inlined);
            }
        }
        
        self.performance_monitor.record_stage_time("inlining", stage_start.elapsed());
        Ok(())
    }
    
    /// Run mid-level optimization passes
    fn run_mid_level_passes(&mut self, module: &Module<'ctx>) -> Result<()> {
        let stage_start = Instant::now();
        
        let fpm = PassManager::create(module);
        
        for pass_name in &self.config.mid_level_passes {
            self.add_pass_to_manager(&fpm, pass_name)?;
        }
        
        fpm.initialize();
        
        // Run passes on all functions
        for function in module.get_functions() {
            fpm.run_on(&function);
        }
        
        self.performance_monitor.record_stage_time("mid_level_passes", stage_start.elapsed());
        Ok(())
    }
    
    /// Run late optimization passes
    fn run_late_passes(&mut self, module: &Module<'ctx>) -> Result<()> {
        let stage_start = Instant::now();
        
        let fpm = PassManager::create(module);
        
        for pass_name in &self.config.late_passes {
            self.add_pass_to_manager(&fpm, pass_name)?;
        }
        
        fpm.initialize();
        
        for function in module.get_functions() {
            fpm.run_on(&function);
        }
        
        self.performance_monitor.record_stage_time("late_passes", stage_start.elapsed());
        Ok(())
    }
    
    /// Run cleanup passes
    fn run_cleanup_passes(&mut self, module: &Module<'ctx>) -> Result<()> {
        let stage_start = Instant::now();
        
        // Cleanup passes are typically module-level
        // Note: With inkwell 0.4, we have limited direct access to module passes
        // This is a placeholder for when more comprehensive pass management is available
        
        self.performance_monitor.record_stage_time("cleanup_passes", stage_start.elapsed());
        Ok(())
    }
    
    /// Add a pass to the pass manager (placeholder for future expansion)
    fn add_pass_to_manager(&self, _fpm: &PassManager<FunctionValue>, pass_name: &str) -> Result<()> {
        // Note: inkwell 0.4 has limited pass management APIs
        // This is a placeholder for when more passes become available
        match pass_name {
            "mem2reg" | "sroa" | "early-cse" | "simplify-cfg" |
            "gvn" | "sccp" | "instcombine" | "reassociate" |
            "licm" | "loop-unroll" | "dce" | "adce" |
            "tailcallelim" | "jump-threading" => {
                // These would be added when inkwell supports them
                Ok(())
            }
            _ => {
                Err(CursedError::from(format!("Unknown pass: {}", pass_name)))
            }
        }
    }
    
    /// Analyze module complexity for optimization planning
    fn analyze_module_complexity(&self, module: &Module<'ctx>) -> Result<ModuleComplexity> {
        let mut function_count = 0;
        let mut instruction_count = 0;
        let mut basic_block_count = 0;
        
        for function in module.get_functions() {
            function_count += 1;
            for basic_block in function.get_basic_blocks() {
                basic_block_count += 1;
                for _instruction in basic_block.get_instructions() {
                    instruction_count += 1;
                }
            }
        }
        
        let estimated_optimization_time = Duration::from_millis(
            (function_count * 10 + instruction_count / 100 + basic_block_count * 5) as u64
        );
        
        Ok(ModuleComplexity {
            function_count,
            instruction_count,
            basic_block_count,
            estimated_optimization_time,
        })
    }
    
    /// Get performance monitoring data
    pub fn get_performance_monitor(&self) -> &PerformanceMonitor {
        &self.performance_monitor
    }
    
    /// Get optimization statistics
    pub fn get_stats(&self) -> Arc<Mutex<OptimizationStats>> {
        Arc::clone(&self.stats)
    }
}

/// Result of enhanced optimization
#[derive(Debug)]
pub struct EnhancedOptimizationResult {
    pub success: bool,
    pub total_time: Duration,
    pub stages_completed: usize,
    pub initial_complexity: Option<ModuleComplexity>,
    pub final_complexity: Option<ModuleComplexity>,
    pub performance_improvement: f64,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
    pub detailed_metrics: HashMap<String, f64>,
}

impl EnhancedOptimizationResult {
    fn new() -> Self {
        Self {
            success: true,
            total_time: Duration::default(),
            stages_completed: 0,
            initial_complexity: None,
            final_complexity: None,
            performance_improvement: 0.0,
            warnings: Vec::new(),
            errors: Vec::new(),
            detailed_metrics: HashMap::new(),
        }
    }
    
    fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
    
    fn add_error(&mut self, error: String) {
        self.errors.push(error);
        self.success = false;
    }
}

/// Module complexity metrics
#[derive(Debug, Clone)]
pub struct ModuleComplexity {
    pub function_count: usize,
    pub instruction_count: usize,
    pub basic_block_count: usize,
    pub estimated_optimization_time: Duration,
}

impl ModuleComplexity {
    /// Calculate complexity reduction percentage
    pub fn improvement_over(&self, other: &ModuleComplexity) -> f64 {
        let initial_score = other.function_count + other.instruction_count + other.basic_block_count;
        let final_score = self.function_count + self.instruction_count + self.basic_block_count;
        
        if initial_score == 0 {
            0.0
        } else {
            ((initial_score as f64 - final_score as f64) / initial_score as f64) * 100.0
        }
    }
}

impl OptimizationPipeline {
    fn new(config: &EnhancedOptimizationConfig) -> Self {
        let mut pipeline = Self {
            stages: Vec::new(),
            current_stage: 0,
            execution_history: Vec::new(),
            adaptive_thresholds: HashMap::new(),
        };
        
        // Build stages from configuration
        pipeline.stages.push(OptimizationStage {
            name: "early".to_string(),
            passes: config.early_passes.clone(),
            execution_mode: ExecutionMode::Sequential,
            prerequisites: Vec::new(),
            metrics_targets: HashMap::new(),
            time_budget: Some(Duration::from_secs(30)),
        });
        
        pipeline.stages.push(OptimizationStage {
            name: "inlining".to_string(),
            passes: config.inlining_passes.clone(),
            execution_mode: ExecutionMode::Adaptive,
            prerequisites: vec!["early".to_string()],
            metrics_targets: HashMap::new(),
            time_budget: Some(Duration::from_secs(60)),
        });
        
        pipeline.stages.push(OptimizationStage {
            name: "mid_level".to_string(),
            passes: config.mid_level_passes.clone(),
            execution_mode: ExecutionMode::Parallel,
            prerequisites: vec!["inlining".to_string()],
            metrics_targets: HashMap::new(),
            time_budget: Some(Duration::from_secs(90)),
        });
        
        pipeline.stages.push(OptimizationStage {
            name: "late".to_string(),
            passes: config.late_passes.clone(),
            execution_mode: ExecutionMode::Sequential,
            prerequisites: vec!["mid_level".to_string()],
            metrics_targets: HashMap::new(),
            time_budget: Some(Duration::from_secs(45)),
        });
        
        pipeline.stages.push(OptimizationStage {
            name: "cleanup".to_string(),
            passes: config.cleanup_passes.clone(),
            execution_mode: ExecutionMode::Sequential,
            prerequisites: vec!["late".to_string()],
            metrics_targets: HashMap::new(),
            time_budget: Some(Duration::from_secs(15)),
        });
        
        pipeline
    }
}

impl PerformanceMonitor {
    fn new() -> Self {
        Self {
            stage_times: HashMap::new(),
            pass_times: HashMap::new(),
            memory_usage: HashMap::new(),
            improvement_metrics: HashMap::new(),
            bottlenecks: Vec::new(),
        }
    }
    
    fn record_stage_time(&mut self, stage: &str, time: Duration) {
        self.stage_times.insert(stage.to_string(), time);
    }
    
    fn record_optimization(&mut self, result: &EnhancedOptimizationResult) {
        // Record overall performance improvement
        self.improvement_metrics.insert("total_improvement".to_string(), result.performance_improvement);
        
        // Analyze for bottlenecks
        for (stage, time) in &self.stage_times {
            if *time > Duration::from_secs(30) {
                self.bottlenecks.push(PerformanceBottleneck {
                    stage: stage.clone(),
                    pass: "unknown".to_string(),
                    issue: "Stage taking too long".to_string(),
                    recommendation: "Consider reducing pass complexity or splitting stage".to_string(),
                    severity: if *time > Duration::from_secs(60) { 
                        BottleneckSeverity::High 
                    } else { 
                        BottleneckSeverity::Medium 
                    },
                });
            }
        }
    }
    
    /// Get identified performance bottlenecks
    pub fn get_bottlenecks(&self) -> &[PerformanceBottleneck] {
        &self.bottlenecks
    }
    
    /// Get stage execution times
    pub fn get_stage_times(&self) -> &HashMap<String, Duration> {
        &self.stage_times
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    
    #[test]
    fn test_enhanced_optimization_config_creation() {
        let config = EnhancedOptimizationConfig::for_self_hosting();
        assert!(config.enable_self_hosting_optimizations);
        assert!(config.cross_module_inlining);
        assert_eq!(config.bootstrap_optimization_level, OptimizationLevel::O3);
    }
    
    #[test]
    fn test_module_complexity_improvement_calculation() {
        let initial = ModuleComplexity {
            function_count: 100,
            instruction_count: 1000,
            basic_block_count: 500,
            estimated_optimization_time: Duration::from_secs(10),
        };
        
        let final_complexity = ModuleComplexity {
            function_count: 90,
            instruction_count: 800,
            basic_block_count: 400,
            estimated_optimization_time: Duration::from_secs(8),
        };
        
        let improvement = final_complexity.improvement_over(&initial);
        assert!(improvement > 0.0);
        assert!(improvement < 100.0);
    }
}
