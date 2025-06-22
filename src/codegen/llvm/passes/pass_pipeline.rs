/// Optimization Pipeline Builder and Executor
/// 
/// Provides automated pipeline construction and execution for optimization passes
/// based on optimization levels, target configurations, and performance constraints.

use super::{OptimizationPass, PassConfiguration, PassResult, PassRegistry};
use crate::optimization::config::OptimizationLevel;
use crate::error::{Error, Result};
use inkwell::{context::Context, module::Module};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{debug, info, instrument, warn, error};

/// Optimization pipeline that manages pass execution
pub struct OptimizationPipeline<'ctx> {
    stages: Vec<OptimizationStage>,
    registry: Arc<Mutex<PassRegistry<'ctx>>>,
    config: PassConfiguration,
    statistics: PipelineStatistics,
    execution_context: PassExecutionContext,
}

impl<'ctx> OptimizationPipeline<'ctx> {
    /// Create a new optimization pipeline
    pub fn new(
        registry: Arc<Mutex<PassRegistry<'ctx>>>,
        config: PassConfiguration,
    ) -> Self {
        let execution_context = PassExecutionContext::new(&config);
        
        Self {
            stages: Vec::new(),
            registry,
            config,
            statistics: PipelineStatistics::default(),
            execution_context,
        }
    }
    
    /// Add an optimization stage to the pipeline
    pub fn add_stage(&mut self, stage: OptimizationStage) {
        info!("Adding optimization stage: {:?}", stage.name);
        self.stages.push(stage);
    }
    
    /// Build pipeline from optimization level
    pub fn build_from_level(&mut self, level: OptimizationLevel) -> Result<()> {
        self.stages.clear();
        
        match level {
            OptimizationLevel::O0 => {
                // No optimization stages
                debug!("Building pipeline for O0 - no optimizations");
            }
            OptimizationLevel::O1 => {
                self.build_basic_pipeline()?;
            }
            OptimizationLevel::O2 => {
                self.build_default_pipeline()?;
            }
            OptimizationLevel::O3 => {
                self.build_aggressive_pipeline()?;
            }
            OptimizationLevel::Os | OptimizationLevel::Oz => {
                self.build_size_pipeline()?;
            }
        }
        
        info!("Built pipeline with {} stages for optimization level {}", 
              self.stages.len(), level.as_str());
        
        Ok(())
    }
    
    /// Execute the optimization pipeline on a module
    #[instrument(skip(self, module, context))]
    pub fn execute(&mut self, module: &Module<'ctx>, context: &'ctx Context) -> Result<PipelineResult> {
        let start_time = Instant::now();
        info!("Executing optimization pipeline with {} stages", self.stages.len());
        
        let mut pipeline_result = PipelineResult::default();
        let mut registry = self.registry.lock().unwrap();
        
        // Execute each stage
        for (stage_index, stage) in self.stages.iter().enumerate() {
            if !self.execution_context.has_time_for_pass(stage.estimated_time) {
                warn!("Skipping stage {} due to time budget constraints", stage.name);
                pipeline_result.skipped_stages.push(stage.name.clone());
                continue;
            }
            
            info!("Executing stage {}: {}", stage_index + 1, stage.name);
            let stage_start = Instant::now();
            
            let stage_result = self.execute_stage(stage, &mut registry, module, context)?;
            let stage_time = stage_start.elapsed();
            
            // Update execution context
            self.execution_context.update_after_pass(stage_time);
            
            // Merge results
            pipeline_result.stages_executed += 1;
            pipeline_result.total_passes_run += stage_result.passes_executed;
            pipeline_result.total_optimizations += stage_result.total_optimizations;
            pipeline_result.execution_time += stage_time;
            pipeline_result.stage_results.push(stage_result);
            
            // Check if we should continue based on results
            if stage_result.errors > 0 && !stage.continue_on_error {
                warn!("Stopping pipeline due to errors in stage: {}", stage.name);
                break;
            }
            
            info!("Stage {} completed in {:?}", stage.name, stage_time);
        }
        
        pipeline_result.total_time = start_time.elapsed();
        pipeline_result.success = pipeline_result.stage_results.iter().all(|r| r.errors == 0);
        
        // Update statistics
        self.statistics.update(&pipeline_result);
        
        info!("Pipeline execution completed: {} stages, {} passes, {:?}", 
              pipeline_result.stages_executed,
              pipeline_result.total_passes_run,
              pipeline_result.total_time);
        
        Ok(pipeline_result)
    }
    
    /// Execute a single optimization stage
    #[instrument(skip(self, stage, registry, module, context))]
    fn execute_stage(
        &mut self,
        stage: &OptimizationStage,
        registry: &mut PassRegistry<'ctx>,
        module: &Module<'ctx>,
        context: &'ctx Context,
    ) -> Result<StageResult> {
        let mut stage_result = StageResult::default();
        let stage_start = Instant::now();
        
        debug!("Executing stage: {} with {} passes", stage.name, stage.passes.len());
        
        // Execute passes in the stage
        for pass_name in &stage.passes {
            if let Some(pass) = registry.get_pass_mut(pass_name) {
                if !pass.should_run(&self.config) {
                    debug!("Skipping pass {} - should not run", pass_name);
                    continue;
                }
                
                info!("Running pass: {}", pass_name);
                let pass_start = Instant::now();
                
                match pass.run_on_module(module, context) {
                    Ok(pass_result) => {
                        let pass_time = pass_start.elapsed();
                        
                        stage_result.passes_executed += 1;
                        if pass_result.changed {
                            stage_result.passes_with_changes += 1;
                            stage_result.total_optimizations += pass_result.instructions_eliminated +
                                                               pass_result.functions_inlined +
                                                               pass_result.loops_unrolled +
                                                               pass_result.constants_folded;
                        }
                        
                        stage_result.pass_results.insert(pass_name.clone(), pass_result);
                        
                        debug!("Pass {} completed in {:?}", pass_name, pass_time);
                    }
                    Err(e) => {
                        error!("Pass {} failed: {}", pass_name, e);
                        stage_result.errors += 1;
                        stage_result.error_messages.push(format!("Pass {}: {}", pass_name, e));
                        
                        if !stage.continue_on_error {
                            return Err(e);
                        }
                    }
                }
            } else {
                warn!("Pass not found in registry: {}", pass_name);
                stage_result.errors += 1;
                stage_result.error_messages.push(format!("Pass not found: {}", pass_name));
            }
        }
        
        stage_result.execution_time = stage_start.elapsed();
        
        debug!("Stage {} completed: {} passes executed, {} with changes, {} errors",
               stage.name, stage_result.passes_executed, stage_result.passes_with_changes, stage_result.errors);
        
        Ok(stage_result)
    }
    
    /// Build basic optimization pipeline (O1)
    fn build_basic_pipeline(&mut self) -> Result<()> {
        debug!("Building basic optimization pipeline");
        
        // Memory promotion stage
        self.add_stage(OptimizationStage {
            name: "memory_promotion".to_string(),
            passes: vec![
                "mem2reg".to_string(),
                "sroa".to_string(),
            ],
            parallel: false,
            continue_on_error: false,
            estimated_time: Duration::from_millis(250),
        });
        
        // Early cleanup stage
        self.add_stage(OptimizationStage {
            name: "early_cleanup".to_string(),
            passes: vec![
                "dead_code_elimination".to_string(),
                "constant_propagation".to_string(),
            ],
            parallel: false,
            continue_on_error: false,
            estimated_time: Duration::from_millis(200),
        });
        
        Ok(())
    }
    
    /// Build default optimization pipeline (O2)
    fn build_default_pipeline(&mut self) -> Result<()> {
        debug!("Building default optimization pipeline");
        
        // Memory promotion stage
        self.add_stage(OptimizationStage {
            name: "memory_promotion".to_string(),
            passes: vec![
                "mem2reg".to_string(),
                "sroa".to_string(),
            ],
            parallel: false,
            continue_on_error: false,
            estimated_time: Duration::from_millis(250),
        });
        
        // Early optimization stage
        self.add_stage(OptimizationStage {
            name: "early_optimization".to_string(),
            passes: vec![
                "sccp".to_string(),
                "dead_code_elimination".to_string(),
                "gvn".to_string(),
            ],
            parallel: false,
            continue_on_error: false,
            estimated_time: Duration::from_millis(400),
        });
        
        // Loop optimization stage
        self.add_stage(OptimizationStage {
            name: "loop_optimization".to_string(),
            passes: vec![
                "licm".to_string(),
                "loop_optimization".to_string(),
            ],
            parallel: false,
            continue_on_error: true,
            estimated_time: Duration::from_millis(500),
        });
        
        // Function optimization stage
        self.add_stage(OptimizationStage {
            name: "function_optimization".to_string(),
            passes: vec![
                "inlining".to_string(),
                "tail_call".to_string(),
                "jump_threading".to_string(),
                "dead_code_elimination".to_string(), // Run again after inlining
            ],
            parallel: false,
            continue_on_error: false,
            estimated_time: Duration::from_millis(600),
        });
        
        Ok(())
    }
    
    /// Build aggressive optimization pipeline (O3)
    fn build_aggressive_pipeline(&mut self) -> Result<()> {
        debug!("Building aggressive optimization pipeline");
        
        // Memory promotion stage
        self.add_stage(OptimizationStage {
            name: "memory_promotion".to_string(),
            passes: vec![
                "mem2reg".to_string(),
                "sroa".to_string(),
            ],
            parallel: false,
            continue_on_error: false,
            estimated_time: Duration::from_millis(250),
        });
        
        // Early optimization stage
        self.add_stage(OptimizationStage {
            name: "early_optimization".to_string(),
            passes: vec![
                "sccp".to_string(),
                "dead_code_elimination".to_string(),
                "gvn".to_string(),
            ],
            parallel: false,
            continue_on_error: false,
            estimated_time: Duration::from_millis(400),
        });
        
        // Aggressive loop optimization
        self.add_stage(OptimizationStage {
            name: "aggressive_loop_optimization".to_string(),
            passes: vec![
                "licm".to_string(),
                "loop_optimization".to_string(),
                "sccp".to_string(), // Run SCCP again after loop opts
            ],
            parallel: false,
            continue_on_error: true,
            estimated_time: Duration::from_millis(700),
        });
        
        // Aggressive function optimization
        self.add_stage(OptimizationStage {
            name: "aggressive_function_optimization".to_string(),
            passes: vec![
                "inlining".to_string(),
                "tail_call".to_string(),
                "gvn".to_string(), // Run GVN again after inlining
                "dead_code_elimination".to_string(),
            ],
            parallel: false,
            continue_on_error: true,
            estimated_time: Duration::from_millis(900),
        });
        
        // Final optimization stage
        self.add_stage(OptimizationStage {
            name: "final_optimization".to_string(),
            passes: vec![
                "jump_threading".to_string(),
                "sccp".to_string(), // Final constant propagation
                "dead_code_elimination".to_string(), // Final cleanup
            ],
            parallel: false,
            continue_on_error: true,
            estimated_time: Duration::from_millis(500),
        });
        
        Ok(())
    }
    
    /// Build size optimization pipeline (Os/Oz)
    fn build_size_pipeline(&mut self) -> Result<()> {
        debug!("Building size optimization pipeline");
        
        // Size-focused optimization stage
        self.add_stage(OptimizationStage {
            name: "size_optimization".to_string(),
            passes: vec![
                "dead_code_elimination".to_string(),
                "constant_propagation".to_string(),
                "memory_optimization".to_string(),
            ],
            parallel: false,
            continue_on_error: false,
            estimated_time: Duration::from_millis(400),
        });
        
        Ok(())
    }
    
    /// Get pipeline statistics
    pub fn get_statistics(&self) -> &PipelineStatistics {
        &self.statistics
    }
    
    /// Reset pipeline statistics
    pub fn reset_statistics(&mut self) {
        self.statistics = PipelineStatistics::default();
    }
    
    /// Get current execution context
    pub fn get_execution_context(&self) -> &PassExecutionContext {
        &self.execution_context
    }
    
    /// Estimate total pipeline execution time
    pub fn estimate_execution_time(&self) -> Duration {
        self.stages.iter().map(|s| s.estimated_time).sum()
    }
    
    /// Check if pipeline can complete within time budget
    pub fn can_complete_in_budget(&self) -> bool {
        self.estimate_execution_time() <= self.config.time_budget
    }
}

/// Builder for optimization pipelines
pub struct PipelineBuilder<'ctx> {
    registry: Arc<Mutex<PassRegistry<'ctx>>>,
    config: PassConfiguration,
    stages: Vec<OptimizationStage>,
}

impl<'ctx> PipelineBuilder<'ctx> {
    /// Create a new pipeline builder
    pub fn new(registry: Arc<Mutex<PassRegistry<'ctx>>>, config: PassConfiguration) -> Self {
        Self {
            registry,
            config,
            stages: Vec::new(),
        }
    }
    
    /// Add a stage to the pipeline
    pub fn add_stage(mut self, stage: OptimizationStage) -> Self {
        self.stages.push(stage);
        self
    }
    
    /// Add a simple pass stage
    pub fn add_pass_stage(mut self, name: &str, passes: Vec<String>) -> Self {
        self.stages.push(OptimizationStage {
            name: name.to_string(),
            passes,
            parallel: false,
            continue_on_error: false,
            estimated_time: Duration::from_millis(200),
        });
        self
    }
    
    /// Add stages based on optimization level
    pub fn with_optimization_level(mut self, level: OptimizationLevel) -> Self {
        match level {
            OptimizationLevel::O0 => {},
            OptimizationLevel::O1 => {
                self = self.add_pass_stage("basic_optimization", vec![
                    "dead_code_elimination".to_string(),
                    "constant_propagation".to_string(),
                ]);
            }
            OptimizationLevel::O2 => {
                self = self.add_pass_stage("default_optimization", vec![
                    "dead_code_elimination".to_string(),
                    "constant_propagation".to_string(),
                    "loop_optimization".to_string(),
                    "inlining".to_string(),
                ]);
            }
            OptimizationLevel::O3 => {
                self = self.add_pass_stage("aggressive_optimization", vec![
                    "dead_code_elimination".to_string(),
                    "constant_propagation".to_string(),
                    "loop_optimization".to_string(),
                    "inlining".to_string(),
                    "memory_optimization".to_string(),
                    "instruction_combining".to_string(),
                    "branch_optimization".to_string(),
                ]);
            }
            OptimizationLevel::Os | OptimizationLevel::Oz => {
                self = self.add_pass_stage("size_optimization", vec![
                    "dead_code_elimination".to_string(),
                    "constant_propagation".to_string(),
                    "memory_optimization".to_string(),
                ]);
            }
        }
        self
    }
    
    /// Build the pipeline
    pub fn build(self) -> OptimizationPipeline<'ctx> {
        let mut pipeline = OptimizationPipeline::new(self.registry, self.config);
        
        for stage in self.stages {
            pipeline.add_stage(stage);
        }
        
        pipeline
    }
}

/// An optimization stage containing related passes
#[derive(Debug, Clone)]
pub struct OptimizationStage {
    pub name: String,
    pub passes: Vec<String>,
    pub parallel: bool,
    pub continue_on_error: bool,
    pub estimated_time: Duration,
}

impl OptimizationStage {
    /// Create a new optimization stage
    pub fn new(name: String, passes: Vec<String>) -> Self {
        Self {
            name,
            passes,
            parallel: false,
            continue_on_error: false,
            estimated_time: Duration::from_millis(200),
        }
    }
    
    /// Set whether passes can run in parallel
    pub fn with_parallel(mut self, parallel: bool) -> Self {
        self.parallel = parallel;
        self
    }
    
    /// Set whether to continue on errors
    pub fn with_continue_on_error(mut self, continue_on_error: bool) -> Self {
        self.continue_on_error = continue_on_error;
        self
    }
    
    /// Set estimated execution time
    pub fn with_estimated_time(mut self, time: Duration) -> Self {
        self.estimated_time = time;
        self
    }
}

/// Result of pipeline execution
#[derive(Debug, Default)]
pub struct PipelineResult {
    pub success: bool,
    pub stages_executed: usize,
    pub total_passes_run: usize,
    pub total_optimizations: usize,
    pub execution_time: Duration,
    pub total_time: Duration,
    pub stage_results: Vec<StageResult>,
    pub skipped_stages: Vec<String>,
}

impl PipelineResult {
    /// Calculate optimization effectiveness
    pub fn effectiveness_score(&self) -> f64 {
        if self.execution_time.as_millis() == 0 {
            0.0
        } else {
            self.total_optimizations as f64 / self.execution_time.as_millis() as f64
        }
    }
    
    /// Get total errors across all stages
    pub fn total_errors(&self) -> usize {
        self.stage_results.iter().map(|r| r.errors).sum()
    }
    
    /// Get passes that made changes
    pub fn passes_with_changes(&self) -> usize {
        self.stage_results.iter().map(|r| r.passes_with_changes).sum()
    }
}

/// Result of executing a single stage
#[derive(Debug, Default)]
pub struct StageResult {
    pub passes_executed: usize,
    pub passes_with_changes: usize,
    pub total_optimizations: usize,
    pub execution_time: Duration,
    pub errors: usize,
    pub error_messages: Vec<String>,
    pub pass_results: HashMap<String, PassResult>,
}

/// Pipeline execution statistics
#[derive(Debug, Default)]
pub struct PipelineStatistics {
    pub total_executions: u64,
    pub successful_executions: u64,
    pub total_execution_time: Duration,
    pub average_execution_time: Duration,
    pub total_stages_executed: u64,
    pub total_passes_executed: u64,
    pub total_optimizations_applied: u64,
}

/// Execution context for passes
#[derive(Debug)]
pub struct PassExecutionContext {
    pub current_pass: Option<String>,
    pub passes_executed: usize,
    pub remaining_time: Duration,
    pub start_time: Instant,
    pub time_budget: Duration,
}

impl PassExecutionContext {
    pub fn new(config: &PassConfiguration) -> Self {
        Self {
            current_pass: None,
            passes_executed: 0,
            remaining_time: config.time_budget,
            start_time: Instant::now(),
            time_budget: config.time_budget,
        }
    }
    
    pub fn has_time_for_pass(&self, estimated_time: Duration) -> bool {
        self.remaining_time >= estimated_time
    }
    
    pub fn set_current_pass(&mut self, pass_name: String) {
        self.current_pass = Some(pass_name);
    }
    
    pub fn update_after_pass(&mut self, actual_time: Duration) {
        self.passes_executed += 1;
        self.remaining_time = self.remaining_time.saturating_sub(actual_time);
    }
}

impl PipelineStatistics {
    /// Update statistics with pipeline result
    pub fn update(&mut self, result: &PipelineResult) {
        self.total_executions += 1;
        if result.success {
            self.successful_executions += 1;
        }
        
        self.total_execution_time += result.total_time;
        self.average_execution_time = self.total_execution_time / self.total_executions as u32;
        self.total_stages_executed += result.stages_executed as u64;
        self.total_passes_executed += result.total_passes_run as u64;
        self.total_optimizations_applied += result.total_optimizations as u64;
    }
    
    /// Calculate success rate
    pub fn success_rate(&self) -> f64 {
        if self.total_executions == 0 {
            0.0
        } else {
            self.successful_executions as f64 / self.total_executions as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::llvm::passes::{PassRegistry, PassConfiguration};
    use std::sync::{Arc, Mutex};
    
    #[test]
    fn test_pipeline_creation() {
        let config = PassConfiguration::default();
        let registry = Arc::new(Mutex::new(PassRegistry::new(config.clone())));
        let pipeline = OptimizationPipeline::new(registry, config);
        
        assert_eq!(pipeline.stages.len(), 0);
        assert_eq!(pipeline.statistics.total_executions, 0);
    }
    
    #[test]
    fn test_optimization_stage_creation() {
        let stage = OptimizationStage::new(
            "test_stage".to_string(),
            vec!["pass1".to_string(), "pass2".to_string()],
        );
        
        assert_eq!(stage.name, "test_stage");
        assert_eq!(stage.passes.len(), 2);
        assert!(!stage.parallel);
        assert!(!stage.continue_on_error);
    }
    
    #[test]
    fn test_optimization_stage_builder() {
        let stage = OptimizationStage::new(
            "test".to_string(),
            vec!["pass1".to_string()],
        )
        .with_parallel(true)
        .with_continue_on_error(true)
        .with_estimated_time(Duration::from_millis(500));
        
        assert!(stage.parallel);
        assert!(stage.continue_on_error);
        assert_eq!(stage.estimated_time, Duration::from_millis(500));
    }
    
    #[test]
    fn test_pipeline_builder() {
        let config = PassConfiguration::default();
        let registry = Arc::new(Mutex::new(PassRegistry::new(config.clone())));
        
        let pipeline = PipelineBuilder::new(registry, config)
            .add_pass_stage("stage1", vec!["pass1".to_string()])
            .add_pass_stage("stage2", vec!["pass2".to_string()])
            .build();
        
        assert_eq!(pipeline.stages.len(), 2);
        assert_eq!(pipeline.stages[0].name, "stage1");
        assert_eq!(pipeline.stages[1].name, "stage2");
    }
    
    #[test]
    fn test_pipeline_builder_with_optimization_level() {
        let config = PassConfiguration::default();
        let registry = Arc::new(Mutex::new(PassRegistry::new(config.clone())));
        
        let pipeline = PipelineBuilder::new(registry, config)
            .with_optimization_level(OptimizationLevel::O2)
            .build();
        
        assert_eq!(pipeline.stages.len(), 1);
        assert!(!pipeline.stages[0].passes.is_empty());
    }
    
    #[test]
    fn test_pipeline_result() {
        let mut result = PipelineResult::default();
        result.total_optimizations = 100;
        result.execution_time = Duration::from_millis(100);
        
        assert_eq!(result.effectiveness_score(), 1.0);
        assert_eq!(result.total_errors(), 0);
    }
    
    #[test]
    fn test_stage_result() {
        let mut stage_result = StageResult::default();
        stage_result.errors = 2;
        stage_result.error_messages = vec!["error1".to_string(), "error2".to_string()];
        
        assert_eq!(stage_result.errors, 2);
        assert_eq!(stage_result.error_messages.len(), 2);
    }
    
    #[test]
    fn test_pipeline_statistics() {
        let mut stats = PipelineStatistics::default();
        
        let mut result = PipelineResult::default();
        result.success = true;
        result.stages_executed = 3;
        result.total_passes_run = 6;
        result.total_time = Duration::from_millis(100);
        
        stats.update(&result);
        
        assert_eq!(stats.total_executions, 1);
        assert_eq!(stats.successful_executions, 1);
        assert_eq!(stats.success_rate(), 1.0);
        assert_eq!(stats.total_stages_executed, 3);
        assert_eq!(stats.total_passes_executed, 6);
    }
    
    #[test]
    fn test_execution_context() {
        let config = PassConfiguration::default();
        let mut context = PassExecutionContext::new(&config);
        
        let pass_time = Duration::from_millis(100);
        assert!(context.has_time_for_pass(pass_time));
        
        context.set_current_pass("test_pass".to_string());
        assert_eq!(context.current_pass, Some("test_pass".to_string()));
        
        context.update_after_pass(pass_time);
        assert_eq!(context.passes_executed, 1);
        assert!(context.remaining_time < config.time_budget);
    }
}
