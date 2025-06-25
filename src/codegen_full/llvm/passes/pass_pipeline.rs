
/// Optimization Pipeline Builder and Executor
/// 
/// Provides automated pipeline construction and execution for optimization passes
/// based on optimization levels, target configurations, and performance constraints.

use super::{OptimizationPass, PassConfiguration, PassResult, PassRegistry};
use crate::common_types::optimization_level::OptimizationLevel;
use crate::error::{CursedError, Result};
use inkwell::{context::Context, module::Module};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{debug, info, instrument, warn, error};

/// Optimization pipeline that manages pass execution
pub struct OptimizationPipeline<'ctx> {
impl<'ctx> OptimizationPipeline<'ctx> {
    /// Create a new optimization pipeline
    pub fn new(
    ) -> Self {
        let execution_context = PassExecutionContext::new(&config);
        
        Self {
        }
    }
    
    /// Add an optimization stage to the pipeline
    pub fn add_stage(&mut self, stage: OptimizationStage) {
        info!("Adding optimization stage: {:?}", stage.name);
        self.stages.push(stage);
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
        
              self.stages.len(), level.as_str());
        
        Ok(())
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
            info!("Stage {} completed in {:?}", stage.name, stage_time);
        pipeline_result.total_time = start_time.elapsed();
        pipeline_result.success = pipeline_result.stage_results.iter().all(|r| r.errors == 0);
        
        // Update statistics
        self.statistics.update(&pipeline_result);
        
              pipeline_result.total_time);
        
        Ok(pipeline_result)
    /// Execute a single optimization stage
    #[instrument(skip(self, stage, registry, module, context))]
    fn execute_stage(
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
        
               stage.name, stage_result.passes_executed, stage_result.passes_with_changes, stage_result.errors);
        
        Ok(stage_result)
    /// Build basic optimization pipeline (O1)
    fn build_basic_pipeline(&mut self) -> Result<()> {
        debug!("Building basic optimization pipeline");
        
        // Memory promotion stage
        self.add_stage(OptimizationStage {
            passes: vec![
        });
        
        // Early cleanup stage
        self.add_stage(OptimizationStage {
            passes: vec![
        });
        
        Ok(())
    /// Build default optimization pipeline (O2)
    fn build_default_pipeline(&mut self) -> Result<()> {
        debug!("Building default optimization pipeline");
        
        // Memory promotion stage
        self.add_stage(OptimizationStage {
            passes: vec![
        });
        
        // Early optimization stage
        self.add_stage(OptimizationStage {
            passes: vec![
        });
        
        // Loop optimization stage
        self.add_stage(OptimizationStage {
            passes: vec![
        });
        
        // Function optimization stage
        self.add_stage(OptimizationStage {
            passes: vec![
                "dead_code_elimination".to_string(), // Run again after inlining
        });
        
        Ok(())
    /// Build aggressive optimization pipeline (O3)
    fn build_aggressive_pipeline(&mut self) -> Result<()> {
        debug!("Building aggressive optimization pipeline");
        
        // Memory promotion stage
        self.add_stage(OptimizationStage {
            passes: vec![
        });
        
        // Early optimization stage
        self.add_stage(OptimizationStage {
            passes: vec![
        });
        
        // Aggressive loop optimization
        self.add_stage(OptimizationStage {
            passes: vec![
                "sccp".to_string(), // Run SCCP again after loop opts
        });
        
        // Aggressive function optimization
        self.add_stage(OptimizationStage {
            passes: vec![
                "gvn".to_string(), // Run GVN again after inlining
        });
        
        // Final optimization stage
        self.add_stage(OptimizationStage {
            passes: vec![
                "sccp".to_string(), // Final constant propagation
                "dead_code_elimination".to_string(), // Final cleanup
        });
        
        Ok(())
    /// Build size optimization pipeline (Os/Oz)
    fn build_size_pipeline(&mut self) -> Result<()> {
        debug!("Building size optimization pipeline");
        
        // Size-focused optimization stage
        self.add_stage(OptimizationStage {
            passes: vec![
        });
        
        Ok(())
    /// Get pipeline statistics
    pub fn get_statistics(&self) -> &PipelineStatistics {
        &self.statistics
    /// Reset pipeline statistics
    pub fn reset_statistics(&mut self) {
        self.statistics = PipelineStatistics::default();
    /// Get current execution context
    pub fn get_execution_context(&self) -> &PassExecutionContext {
        &self.execution_context
    /// Estimate total pipeline execution time
    pub fn estimate_execution_time(&self) -> Duration {
        self.stages.iter().map(|s| s.estimated_time).sum()
    /// Check if pipeline can complete within time budget
    pub fn can_complete_in_budget(&self) -> bool {
        self.estimate_execution_time() <= self.config.time_budget
    }
}

/// Builder for optimization pipelines
pub struct PipelineBuilder<'ctx> {
impl<'ctx> PipelineBuilder<'ctx> {
    /// Create a new pipeline builder
    pub fn new(registry: Arc<Mutex<PassRegistry<'ctx>>>, config: PassConfiguration) -> Self {
        Self {
        }
    }
    
    /// Add a stage to the pipeline
    pub fn add_stage(mut self, stage: OptimizationStage) -> Self {
        self.stages.push(stage);
        self
    /// Add a simple pass stage
    pub fn add_pass_stage(mut self, name: &str, passes: Vec<String>) -> Self {
        self.stages.push(OptimizationStage {
        });
        self
    /// Add stages based on optimization level
    pub fn with_optimization_level(mut self, level: OptimizationLevel) -> Self {
        match level {
            OptimizationLevel::O1 => {
                self = self.add_pass_stage("basic_optimization", vec![
                ]);
            }
            OptimizationLevel::O2 => {
                self = self.add_pass_stage("default_optimization", vec![
                ]);
            }
            OptimizationLevel::O3 => {
                self = self.add_pass_stage("aggressive_optimization", vec![
                ]);
            }
            OptimizationLevel::Os | OptimizationLevel::Oz => {
                self = self.add_pass_stage("size_optimization", vec![
                ]);
            }
        }
        self
    /// Build the pipeline
    pub fn build(self) -> OptimizationPipeline<'ctx> {
        let mut pipeline = OptimizationPipeline::new(self.registry, self.config);
        
        for stage in self.stages {
            pipeline.add_stage(stage);
        pipeline
    }
}

/// An optimization stage containing related passes
#[derive(Debug, Clone)]
pub struct OptimizationStage {
impl OptimizationStage {
    /// Create a new optimization stage
    pub fn new(name: String, passes: Vec<String>) -> Self {
        Self {
        }
    }
    
    /// Set whether passes can run in parallel
    pub fn with_parallel(mut self, parallel: bool) -> Self {
        self.parallel = parallel;
        self
    /// Set whether to continue on errors
    pub fn with_continue_on_error(mut self, continue_on_error: bool) -> Self {
        self.continue_on_error = continue_on_error;
        self
    /// Set estimated execution time
    pub fn with_estimated_time(mut self, time: Duration) -> Self {
        self.estimated_time = time;
        self
    }
}

/// Result of pipeline execution
#[derive(Debug, Default)]
pub struct PipelineResult {
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
    /// Get passes that made changes
    pub fn passes_with_changes(&self) -> usize {
        self.stage_results.iter().map(|r| r.passes_with_changes).sum()
    }
}

/// Result of executing a single stage
#[derive(Debug, Default)]
pub struct StageResult {
/// Pipeline execution statistics
#[derive(Debug, Default)]
pub struct PipelineStatistics {
/// Execution context for passes
#[derive(Debug)]
pub struct PassExecutionContext {
impl PassExecutionContext {
    pub fn new(config: &PassConfiguration) -> Self {
        Self {
        }
    }
    
    pub fn has_time_for_pass(&self, estimated_time: Duration) -> bool {
        self.remaining_time >= estimated_time
    pub fn set_current_pass(&mut self, pass_name: String) {
        self.current_pass = Some(pass_name);
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
        self.total_execution_time += result.total_time;
        self.average_execution_time = self.total_execution_time / self.total_executions as u32;
        self.total_stages_executed += result.stages_executed as u64;
        self.total_passes_executed += result.total_passes_run as u64;
        self.total_optimizations_applied += result.total_optimizations as u64;
    /// Calculate success rate
    pub fn success_rate(&self) -> f64 {
        if self.total_executions == 0 {
            0.0
        } else {
            self.successful_executions as f64 / self.total_executions as f64
        }
    }
