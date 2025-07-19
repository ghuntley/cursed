//! Enhanced pass manager with improved ordering and configuration
//! 
//! This module provides an enhanced pass manager that integrates with
//! the existing inlining system and provides better optimization sequences.

use crate::error::{CursedError, Result};
use crate::codegen::llvm::passes::inlining::{InliningPass, InliningConfig, InliningResult};
use inkwell::{
    context::Context,
    module::Module,
    passes::PassManager,
    values::FunctionValue,
    OptimizationLevel as InkwellOptLevel,
};
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

/// Enhanced pass manager with intelligent pass ordering
pub struct EnhancedPassManager<'ctx> {
    context: &'ctx Context,
    function_pass_manager: Option<PassManager<FunctionValue<'ctx>>>,
    pass_execution_order: Vec<PassExecutionPlan>,
    pass_dependencies: HashMap<String, Vec<String>>,
    pass_statistics: HashMap<String, PassExecutionStats>,
    configuration: EnhancedPassConfiguration,
    inlining_integration: InliningIntegration<'ctx>,
}

/// Configuration for enhanced pass management
#[derive(Debug, Clone)]
pub struct EnhancedPassConfiguration {
    pub optimization_level: u32,
    pub enable_adaptive_ordering: bool,
    pub enable_dependency_analysis: bool,
    pub enable_parallel_execution: bool,
    pub pass_timeout: Option<Duration>,
    pub max_iterations: usize,
    pub convergence_threshold: f64,
    pub enable_inlining_integration: bool,
    pub inlining_phases: Vec<InliningPhase>,
}

/// Inlining phase configuration
#[derive(Debug, Clone)]
pub struct InliningPhase {
    pub phase_name: String,
    pub when_to_run: InliningTiming,
    pub config: InliningConfig,
    pub required_prerequisites: Vec<String>,
}

/// When to run inlining in the optimization pipeline
#[derive(Debug, Clone, PartialEq)]
pub enum InliningTiming {
    Early,        // Before most optimizations
    Mid,          // After basic optimizations, before advanced ones
    Late,         // After most optimizations
    Adaptive,     // Based on analysis results
}

/// Plan for executing a pass
#[derive(Debug, Clone)]
pub struct PassExecutionPlan {
    pub pass_name: String,
    pub execution_order: usize,
    pub dependencies: Vec<String>,
    pub estimated_benefit: f64,
    pub execution_cost: Duration,
    pub can_run_parallel: bool,
}

/// Statistics for pass execution
#[derive(Debug, Clone)]
pub struct PassExecutionStats {
    pub total_executions: usize,
    pub successful_executions: usize,
    pub total_time: Duration,
    pub average_time: Duration,
    pub improvement_score: f64,
    pub last_execution: Option<Instant>,
}

/// Inlining integration manager
pub struct InliningIntegration<'ctx> {
    context: &'ctx Context,
    inlining_phases: Vec<InliningPhase>,
    execution_results: Vec<InliningExecutionResult>,
    adaptive_thresholds: HashMap<String, u32>,
}

/// Result of inlining execution in a phase
#[derive(Debug, Clone)]
pub struct InliningExecutionResult {
    pub phase_name: String,
    pub timing: InliningTiming,
    pub result: InliningResult,
    pub execution_time: Duration,
    pub functions_before: usize,
    pub functions_after: usize,
}

impl Default for EnhancedPassConfiguration {
    fn default() -> Self {
        Self::for_optimization_level(2)
    }
}

impl EnhancedPassConfiguration {
    /// Create configuration for specific optimization level
    pub fn for_optimization_level(level: u32) -> Self {
        let mut config = Self {
            optimization_level: level,
            enable_adaptive_ordering: level >= 2,
            enable_dependency_analysis: level >= 1,
            enable_parallel_execution: level >= 2,
            pass_timeout: if level >= 3 {
                Some(Duration::from_secs(120))
            } else {
                Some(Duration::from_secs(60))
            },
            max_iterations: match level {
                0 => 1,
                1 => 2,
                2 => 3,
                3 => 5,
                _ => 3,
            },
            convergence_threshold: 0.01,
            enable_inlining_integration: level >= 1,
            inlining_phases: Vec::new(),
        };
        
        // Configure inlining phases based on optimization level
        config.inlining_phases = Self::create_inlining_phases_for_level(level);
        config
    }
    
    /// Create inlining phases for optimization level
    fn create_inlining_phases_for_level(level: u32) -> Vec<InliningPhase> {
        let mut phases = Vec::new();
        
        match level {
            0 => {
                // No inlining at O0
            }
            1 => {
                // Basic inlining only
                phases.push(InliningPhase {
                    phase_name: "basic_inlining".to_string(),
                    when_to_run: InliningTiming::Early,
                    config: InliningConfig::for_optimization_level(1),
                    required_prerequisites: vec!["mem2reg".to_string()],
                });
            }
            2 => {
                // Standard inlining with two phases
                phases.push(InliningPhase {
                    phase_name: "early_inlining".to_string(),
                    when_to_run: InliningTiming::Early,
                    config: {
                        let mut config = InliningConfig::for_optimization_level(2);
                        config.inline_threshold = 200; // Conservative early threshold
                        config
                    },
                    required_prerequisites: vec!["mem2reg".to_string(), "sroa".to_string()],
                });
                
                phases.push(InliningPhase {
                    phase_name: "mid_inlining".to_string(),
                    when_to_run: InliningTiming::Mid,
                    config: InliningConfig::for_optimization_level(2),
                    required_prerequisites: vec!["instcombine".to_string(), "gvn".to_string()],
                });
            }
            3 => {
                // Aggressive inlining with three phases
                phases.push(InliningPhase {
                    phase_name: "early_aggressive_inlining".to_string(),
                    when_to_run: InliningTiming::Early,
                    config: {
                        let mut config = InliningConfig::for_optimization_level(3);
                        config.inline_threshold = 300;
                        config.aggressive_inlining = false; // Conservative in early phase
                        config
                    },
                    required_prerequisites: vec!["mem2reg".to_string(), "sroa".to_string()],
                });
                
                phases.push(InliningPhase {
                    phase_name: "mid_aggressive_inlining".to_string(),
                    when_to_run: InliningTiming::Mid,
                    config: {
                        let mut config = InliningConfig::for_optimization_level(3);
                        config.aggressive_inlining = true;
                        config
                    },
                    required_prerequisites: vec!["instcombine".to_string(), "gvn".to_string(), "licm".to_string()],
                });
                
                phases.push(InliningPhase {
                    phase_name: "late_cleanup_inlining".to_string(),
                    when_to_run: InliningTiming::Late,
                    config: {
                        let mut config = InliningConfig::for_optimization_level(3);
                        config.inline_threshold = 150; // Smaller threshold for cleanup
                        config.aggressive_inlining = false;
                        config
                    },
                    required_prerequisites: vec!["loop-unroll".to_string()],
                });
            }
            _ => {
                // Use level 3 configuration for higher levels
                phases = Self::create_inlining_phases_for_level(3);
            }
        }
        
        phases
    }
    
    /// Create configuration for self-hosting compiler optimization
    pub fn for_self_hosting() -> Self {
        let mut config = Self::for_optimization_level(3);
        config.enable_adaptive_ordering = true;
        config.enable_parallel_execution = true;
        config.max_iterations = 4;
        config.pass_timeout = Some(Duration::from_secs(180));
        
        // Enhanced inlining for self-hosting
        for phase in &mut config.inlining_phases {
            phase.config.enable_interface_inlining = true;
            phase.config.enable_generics_inlining = true;
            phase.config.enable_cross_module_inlining = true;
            phase.config.performance_mode = true;
        }
        
        config
    }
}

impl<'ctx> EnhancedPassManager<'ctx> {
    /// Create a new enhanced pass manager
    pub fn new(context: &'ctx Context, configuration: EnhancedPassConfiguration) -> Self {
        Self {
            context,
            function_pass_manager: None,
            pass_execution_order: Vec::new(),
            pass_dependencies: HashMap::new(),
            pass_statistics: HashMap::new(),
            inlining_integration: InliningIntegration::new(context, configuration.inlining_phases.clone()),
            configuration,
        }
    }
    
    /// Initialize the pass manager with a module
    pub fn initialize(&mut self, module: &Module<'ctx>) -> Result<()> {
        // Create function pass manager
        self.function_pass_manager = Some(PassManager::create(module));
        
        // Build pass execution plan
        self.build_execution_plan()?;
        
        // Initialize inlining integration
        self.inlining_integration.initialize(module)?;
        
        // Initialize pass manager
        if let Some(ref fpm) = self.function_pass_manager {
            if !fpm.initialize() {
                return Err(CursedError::from("Failed to initialize function pass manager".to_string()));
            }
        }
        
        Ok(())
    }
    
    /// Run the complete optimization pipeline with inlining integration
    pub fn run_optimization_pipeline(&mut self, module: &Module<'ctx>) -> Result<OptimizationPipelineResult> {
        let start_time = Instant::now();
        let mut result = OptimizationPipelineResult::new();
        
        // Count functions before optimization
        result.functions_before = module.get_functions().count();
        
        // Phase 1: Early passes with early inlining
        self.run_phase_with_inlining(module, InliningTiming::Early, &mut result)?;
        
        // Phase 2: Mid-level passes with mid inlining
        self.run_phase_with_inlining(module, InliningTiming::Mid, &mut result)?;
        
        // Phase 3: Late passes with late inlining
        self.run_phase_with_inlining(module, InliningTiming::Late, &mut result)?;
        
        // Phase 4: Adaptive inlining if enabled
        if self.configuration.enable_adaptive_ordering {
            self.run_adaptive_inlining(module, &mut result)?;
        }
        
        // Count functions after optimization
        result.functions_after = module.get_functions().count();
        result.total_time = start_time.elapsed();
        
        // Update statistics
        self.update_pipeline_statistics(&result);
        
        Ok(result)
    }
    
    /// Run optimization phase with inlining integration
    fn run_phase_with_inlining(
        &mut self,
        module: &Module<'ctx>,
        timing: InliningTiming,
        result: &mut OptimizationPipelineResult
    ) -> Result<()> {
        // Run pre-inlining passes for this phase
        self.run_passes_for_phase(module, &timing, true)?;
        
        // Run inlining for this phase
        if self.configuration.enable_inlining_integration {
            let inlining_result = self.inlining_integration.run_phase(module, timing.clone())?;
            result.inlining_results.push(inlining_result);
        }
        
        // Run post-inlining passes for this phase
        self.run_passes_for_phase(module, &timing, false)?;
        
        Ok(())
    }
    
    /// Run passes for a specific phase
    fn run_passes_for_phase(
        &mut self,
        module: &Module<'ctx>,
        timing: &InliningTiming,
        pre_inlining: bool
    ) -> Result<()> {
        let pass_names = self.get_passes_for_timing(timing, pre_inlining);
        
        if let Some(ref fpm) = self.function_pass_manager {
            let mut statistics_updates = Vec::new();
            
            for function in module.get_functions() {
                for pass_name in &pass_names {
                    let pass_start = Instant::now();
                    
                    // Run the pass (simplified - in practice would need actual pass objects)
                    fpm.run_on(&function);
                    
                    // Collect statistics updates
                    let execution_time = pass_start.elapsed();
                    statistics_updates.push((pass_name.clone(), execution_time, true));
                }
            }
            
            // Update statistics after releasing the borrow
            for (pass_name, execution_time, success) in statistics_updates {
                self.update_pass_statistics(&pass_name, execution_time, success);
            }
        }
        
        Ok(())
    }
    
    /// Get passes to run for specific timing and phase
    fn get_passes_for_timing(&self, timing: &InliningTiming, pre_inlining: bool) -> Vec<String> {
        match (timing, pre_inlining) {
            (InliningTiming::Early, true) => vec![
                "mem2reg".to_string(),
                "sroa".to_string(),
                "early-cse".to_string(),
            ],
            (InliningTiming::Early, false) => vec![
                "instcombine".to_string(),
                "simplify-cfg".to_string(),
            ],
            (InliningTiming::Mid, true) => vec![
                "gvn".to_string(),
                "sccp".to_string(),
                "licm".to_string(),
            ],
            (InliningTiming::Mid, false) => vec![
                "instcombine".to_string(),
                "reassociate".to_string(),
                "loop-simplify".to_string(),
            ],
            (InliningTiming::Late, true) => vec![
                "loop-unroll".to_string(),
                "jump-threading".to_string(),
            ],
            (InliningTiming::Late, false) => vec![
                "dce".to_string(),
                "adce".to_string(),
                "tailcallelim".to_string(),
            ],
            (InliningTiming::Adaptive, _) => vec![
                "instcombine".to_string(),
                "simplify-cfg".to_string(),
            ],
        }
    }
    
    /// Run adaptive inlining based on analysis results
    fn run_adaptive_inlining(
        &mut self,
        module: &Module<'ctx>,
        result: &mut OptimizationPipelineResult
    ) -> Result<()> {
        // Analyze the current state of the module
        let analysis = self.analyze_module_for_adaptive_inlining(module)?;
        
        // Decide whether to run additional inlining
        if analysis.should_inline_more {
            let adaptive_result = self.inlining_integration.run_phase(module, InliningTiming::Adaptive)?;
            result.inlining_results.push(adaptive_result);
        }
        
        Ok(())
    }
    
    /// Analyze module for adaptive inlining decisions
    fn analyze_module_for_adaptive_inlining(&self, module: &Module<'ctx>) -> Result<AdaptiveInliningAnalysis> {
        let mut function_count = 0;
        let mut small_function_count = 0;
        let mut call_site_count = 0;
        
        for function in module.get_functions() {
            function_count += 1;
            
            let mut instruction_count = 0;
            for basic_block in function.get_basic_blocks() {
                for instruction in basic_block.get_instructions() {
                    instruction_count += 1;
                    
                    // Count call sites (simplified check)
                    if instruction.get_opcode() == inkwell::values::InstructionOpcode::Call {
                        call_site_count += 1;
                    }
                }
            }
            
            // Consider functions with < 20 instructions as small
            if instruction_count < 20 {
                small_function_count += 1;
            }
        }
        
        // Decide whether to inline more based on heuristics
        let should_inline_more = 
            small_function_count as f64 / function_count as f64 > 0.3 && // Many small functions
            call_site_count > 10; // Reasonable number of call sites
        
        Ok(AdaptiveInliningAnalysis {
            function_count,
            small_function_count,
            call_site_count,
            should_inline_more,
        })
    }
    
    /// Build execution plan for passes
    fn build_execution_plan(&mut self) -> Result<()> {
        // Build dependency graph
        self.build_pass_dependencies();
        
        // Create execution order
        self.pass_execution_order = self.calculate_optimal_pass_order()?;
        
        Ok(())
    }
    
    /// Build pass dependencies
    fn build_pass_dependencies(&mut self) {
        // Define pass dependencies (simplified)
        self.pass_dependencies.insert("instcombine".to_string(), vec!["mem2reg".to_string()]);
        self.pass_dependencies.insert("gvn".to_string(), vec!["mem2reg".to_string(), "sroa".to_string()]);
        self.pass_dependencies.insert("licm".to_string(), vec!["loop-simplify".to_string()]);
        self.pass_dependencies.insert("loop-unroll".to_string(), vec!["licm".to_string()]);
    }
    
    /// Calculate optimal pass execution order
    fn calculate_optimal_pass_order(&self) -> Result<Vec<PassExecutionPlan>> {
        let mut execution_plans = Vec::new();
        let mut order = 0;
        
        // Build execution plans (simplified topological sort)
        let base_passes = vec![
            "mem2reg", "sroa", "early-cse", "instcombine", "gvn", 
            "sccp", "licm", "loop-unroll", "dce", "simplify-cfg"
        ];
        
        for pass_name in base_passes {
            execution_plans.push(PassExecutionPlan {
                pass_name: pass_name.to_string(),
                execution_order: order,
                dependencies: self.pass_dependencies.get(pass_name).cloned().unwrap_or_default(),
                estimated_benefit: 1.0, // Would be calculated based on analysis
                execution_cost: Duration::from_millis(100), // Estimated
                can_run_parallel: false, // Conservative
            });
            order += 1;
        }
        
        Ok(execution_plans)
    }
    
    /// Update pass execution statistics
    fn update_pass_statistics(&mut self, pass_name: &str, execution_time: Duration, success: bool) {
        let stats = self.pass_statistics.entry(pass_name.to_string()).or_insert_with(|| {
            PassExecutionStats {
                total_executions: 0,
                successful_executions: 0,
                total_time: Duration::default(),
                average_time: Duration::default(),
                improvement_score: 0.0,
                last_execution: None,
            }
        });
        
        stats.total_executions += 1;
        if success {
            stats.successful_executions += 1;
        }
        stats.total_time += execution_time;
        stats.average_time = stats.total_time / stats.total_executions as u32;
        stats.last_execution = Some(Instant::now());
    }
    
    /// Update pipeline statistics
    fn update_pipeline_statistics(&mut self, result: &OptimizationPipelineResult) {
        // Update adaptive thresholds based on results
        for inlining_result in &result.inlining_results {
            if inlining_result.result.functions_inlined > 0 {
                let threshold = self.inlining_integration.adaptive_thresholds
                    .entry(inlining_result.phase_name.clone())
                    .or_insert(275);
                
                // Increase threshold if inlining was successful
                *threshold = (*threshold as f64 * 1.1) as u32;
            }
        }
    }
    
    /// Get pass execution statistics
    pub fn get_pass_statistics(&self) -> &HashMap<String, PassExecutionStats> {
        &self.pass_statistics
    }
    
    /// Get inlining integration results
    pub fn get_inlining_results(&self) -> &[InliningExecutionResult] {
        &self.inlining_integration.execution_results
    }
}

/// Result of running the optimization pipeline
#[derive(Debug)]
pub struct OptimizationPipelineResult {
    pub functions_before: usize,
    pub functions_after: usize,
    pub total_time: Duration,
    pub inlining_results: Vec<InliningExecutionResult>,
    pub pass_execution_times: HashMap<String, Duration>,
    pub overall_improvement: f64,
}

impl OptimizationPipelineResult {
    fn new() -> Self {
        Self {
            functions_before: 0,
            functions_after: 0,
            total_time: Duration::default(),
            inlining_results: Vec::new(),
            pass_execution_times: HashMap::new(),
            overall_improvement: 0.0,
        }
    }
}

/// Analysis result for adaptive inlining
#[derive(Debug)]
struct AdaptiveInliningAnalysis {
    function_count: usize,
    small_function_count: usize,
    call_site_count: usize,
    should_inline_more: bool,
}

impl<'ctx> InliningIntegration<'ctx> {
    fn new(context: &'ctx Context, inlining_phases: Vec<InliningPhase>) -> Self {
        Self {
            context,
            inlining_phases,
            execution_results: Vec::new(),
            adaptive_thresholds: HashMap::new(),
        }
    }
    
    fn initialize(&mut self, _module: &Module<'ctx>) -> Result<()> {
        // Initialize adaptive thresholds
        for phase in &self.inlining_phases {
            self.adaptive_thresholds.insert(
                phase.phase_name.clone(),
                phase.config.inline_threshold
            );
        }
        Ok(())
    }
    
    fn run_phase(&mut self, module: &Module<'ctx>, timing: InliningTiming) -> Result<InliningExecutionResult> {
        let start_time = Instant::now();
        let functions_before = module.get_functions().count();
        
        // Find the inlining phase for this timing
        let phase = self.inlining_phases.iter()
            .find(|p| p.when_to_run == timing)
            .ok_or_else(|| CursedError::from(format!("No inlining phase found for timing {:?}", timing)))?;
        
        // Create inlining pass with adaptive threshold
        let mut config = phase.config.clone();
        if let Some(&adaptive_threshold) = self.adaptive_thresholds.get(&phase.phase_name) {
            config.inline_threshold = adaptive_threshold;
        }
        
        let mut inlining_pass = InliningPass::with_config(self.context, config);
        let inlining_result = inlining_pass.run(module)?;
        
        let execution_time = start_time.elapsed();
        let functions_after = module.get_functions().count();
        
        let result = InliningExecutionResult {
            phase_name: phase.phase_name.clone(),
            timing,
            result: inlining_result,
            execution_time,
            functions_before,
            functions_after,
        };
        
        self.execution_results.push(result.clone());
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    
    #[test]
    fn test_enhanced_pass_configuration_creation() {
        let config = EnhancedPassConfiguration::for_optimization_level(3);
        assert!(config.enable_adaptive_ordering);
        assert!(config.enable_inlining_integration);
        assert_eq!(config.optimization_level, 3);
    }
    
    #[test]
    fn test_inlining_phases_creation() {
        let phases = EnhancedPassConfiguration::create_inlining_phases_for_level(3);
        assert_eq!(phases.len(), 3);
        
        let early_phase = phases.iter().find(|p| p.when_to_run == InliningTiming::Early);
        assert!(early_phase.is_some());
    }
    
    #[test]
    fn test_pass_execution_plan_creation() {
        let context = Context::create();
        let config = EnhancedPassConfiguration::for_optimization_level(2);
        let mut manager = EnhancedPassManager::new(&context, config);
        
        assert!(manager.build_execution_plan().is_ok());
        assert!(!manager.pass_execution_order.is_empty());
    }
}
