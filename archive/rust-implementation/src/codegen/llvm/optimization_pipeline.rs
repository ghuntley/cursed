//! Optimization Pipeline implementation for CURSED LLVM compilation

use crate::error::CursedError;
use super::optimization_passes::{PassRegistry, PassConfiguration, OptimizationPass, PassResult};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Main optimization pipeline
#[derive(Debug)]
pub struct OptimizationPipeline {
    stages: Vec<PipelineStage>,
    pass_registry: PassRegistry,
    statistics: PipelineStatistics,
    config: PipelineConfig,
}

/// Configuration for the optimization pipeline
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    pub max_total_time: Option<Duration>,
    pub fail_on_error: bool,
    pub parallel_execution: bool,
    pub optimization_level: u32,
}

/// A stage in the optimization pipeline
#[derive(Debug, Clone)]
pub struct PipelineStage {
    pub name: String,
    pub passes: Vec<String>,
    pub parallel: bool,
    pub required: bool,
}

/// Result of running the entire pipeline
#[derive(Debug)]
pub struct PipelineResult {
    pub success: bool,
    pub final_code: String,
    pub total_time: Duration,
    pub stage_results: Vec<StageResult>,
    pub overall_improvements: HashMap<String, f64>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

/// Result of running a single stage
#[derive(Debug)]
pub struct StageResult {
    pub stage_name: String,
    pub success: bool,
    pub execution_time: Duration,
    pub pass_results: Vec<PassResult>,
    pub code_before: String,
    pub code_after: String,
    pub improvements: HashMap<String, f64>,
}

/// Statistics for the optimization pipeline
#[derive(Debug, Default)]
pub struct PipelineStatistics {
    pub total_runs: u64,
    pub successful_runs: u64,
    pub failed_runs: u64,
    pub total_execution_time: Duration,
    pub average_improvement: f64,
    pub stage_statistics: HashMap<String, StageStatistics>,
}

/// Statistics for individual stages
#[derive(Debug, Default)]
pub struct StageStatistics {
    pub runs: u64,
    pub successes: u64,
    pub failures: u64,
    pub total_time: Duration,
    pub average_time: Duration,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            max_total_time: Some(Duration::from_secs(60)),
            fail_on_error: false,
            parallel_execution: false,
            optimization_level: 2,
        }
    }
}

impl OptimizationPipeline {
    /// Create a new optimization pipeline
    pub fn new() -> Self {
        Self {
            stages: vec![],
            pass_registry: PassRegistry::new(),
            statistics: PipelineStatistics::default(),
            config: PipelineConfig::default(),
        }
    }

    /// Create a new optimization pipeline with configuration
    pub fn with_config(config: PipelineConfig) -> Self {
        Self {
            stages: vec![],
            pass_registry: PassRegistry::new(),
            statistics: PipelineStatistics::default(),
            config,
        }
    }

    /// Add a stage to the pipeline
    pub fn add_stage(&mut self, stage: PipelineStage) -> Result<(), CursedError> {
        // Validate that all passes in the stage are registered
        for pass_name in &stage.passes {
            if !self.pass_registry.has_pass(pass_name) {
                return Err(CursedError::runtime_error(&format!(
                    "Pass '{}' not found in registry for stage '{}'", 
                    pass_name, stage.name
                )));
            }
        }
        
        self.stages.push(stage);
        Ok(())
    }

    /// Register an optimization pass
    pub fn register_pass(&mut self, pass: Box<dyn OptimizationPass>) -> Result<(), CursedError> {
        self.pass_registry.register_pass(pass)
    }

    /// Configure a pass
    pub fn configure_pass(&mut self, name: &str, config: PassConfiguration) -> Result<(), CursedError> {
        self.pass_registry.configure_pass(name, config)
    }

    /// Run the entire optimization pipeline
    pub fn run(&mut self, code: &str) -> Result<PipelineResult, CursedError> {
        let start_time = Instant::now();
        self.statistics.total_runs += 1;

        let mut current_code = code.to_string();
        let mut stage_results = Vec::new();
        let mut overall_improvements = HashMap::new();
        let mut warnings = Vec::new();
        let mut errors = Vec::new();
        let mut pipeline_success = true;

        // Clone stages to avoid borrowing issues
        let stages = self.stages.clone();
        for stage in &stages {
            let stage_start_time = Instant::now();
            let code_before = current_code.clone();

            match self.run_stage(stage, &current_code) {
                Ok(stage_result) => {
                    current_code = stage_result.code_after.clone();
                    
                    // Merge improvements
                    for (key, value) in &stage_result.improvements {
                        *overall_improvements.entry(key.clone()).or_insert(0.0) += value;
                    }
                    
                    // Update stage statistics
                    self.update_stage_statistics(&stage.name, true, stage_result.execution_time);
                    
                    stage_results.push(stage_result);
                }
                Err(e) => {
                    let error_msg = format!("Stage '{}' failed: {}", stage.name, e);
                    errors.push(error_msg.clone());
                    
                    self.update_stage_statistics(&stage.name, false, stage_start_time.elapsed());
                    
                    if stage.required || self.config.fail_on_error {
                        pipeline_success = false;
                        break;
                    }
                    
                    // Create a failed stage result
                    stage_results.push(StageResult {
                        stage_name: stage.name.clone(),
                        success: false,
                        execution_time: stage_start_time.elapsed(),
                        pass_results: vec![],
                        code_before,
                        code_after: current_code.clone(),
                        improvements: HashMap::new(),
                    });
                }
            }

            // Check timeout
            if let Some(max_time) = self.config.max_total_time {
                if start_time.elapsed() > max_time {
                    warnings.push("Pipeline execution timed out".to_string());
                    break;
                }
            }
        }

        let total_time = start_time.elapsed();
        
        if pipeline_success {
            self.statistics.successful_runs += 1;
        } else {
            self.statistics.failed_runs += 1;
        }
        
        self.statistics.total_execution_time += total_time;

        Ok(PipelineResult {
            success: pipeline_success,
            final_code: current_code,
            total_time,
            stage_results,
            overall_improvements,
            warnings,
            errors,
        })
    }

    /// Run a single stage
    fn run_stage(&mut self, stage: &PipelineStage, code: &str) -> Result<StageResult, CursedError> {
        let start_time = Instant::now();
        let mut current_code = code.to_string();
        let mut pass_results = Vec::new();
        let mut improvements = HashMap::new();

        for pass_name in &stage.passes {
            match self.pass_registry.run_pass(pass_name, &current_code) {
                Ok(result) => {
                    if result.success {
                        current_code = result.transformed_code.clone();
                    }
                    
                    // Merge improvements
                    for (key, value) in &result.improvements {
                        *improvements.entry(key.clone()).or_insert(0.0) += value;
                    }
                    
                    pass_results.push(result);
                }
                Err(e) => {
                    if stage.required {
                        return Err(e);
                    }
                    // Continue with other passes if stage is not required
                }
            }
        }

        Ok(StageResult {
            stage_name: stage.name.clone(),
            success: true,
            execution_time: start_time.elapsed(),
            pass_results,
            code_before: code.to_string(),
            code_after: current_code,
            improvements,
        })
    }

    /// Update statistics for a stage
    fn update_stage_statistics(&mut self, stage_name: &str, success: bool, execution_time: Duration) {
        let stats = self.statistics.stage_statistics
            .entry(stage_name.to_string())
            .or_insert_with(StageStatistics::default);
        
        stats.runs += 1;
        stats.total_time += execution_time;
        stats.average_time = stats.total_time / stats.runs as u32;
        
        if success {
            stats.successes += 1;
        } else {
            stats.failures += 1;
        }
    }

    /// Get pipeline statistics
    pub fn statistics(&self) -> &PipelineStatistics {
        &self.statistics
    }

    /// Reset pipeline statistics
    pub fn reset_statistics(&mut self) {
        self.statistics = PipelineStatistics::default();
    }

    /// Create a default optimization pipeline
    pub fn create_default() -> Result<Self, CursedError> {
        let mut pipeline = Self::new();
        
        // Add default stages
        pipeline.add_stage(PipelineStage {
            name: "analysis".to_string(),
            passes: vec!["dead_code_elimination".to_string()],
            parallel: false,
            required: false,
        })?;
        
        pipeline.add_stage(PipelineStage {
            name: "optimization".to_string(),
            passes: vec!["inlining".to_string()],
            parallel: false,
            required: false,
        })?;
        
        Ok(pipeline)
    }
}

impl Default for OptimizationPipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl PipelineStage {
    /// Create a new pipeline stage
    pub fn new(name: String, passes: Vec<String>) -> Self {
        Self {
            name,
            passes,
            parallel: false,
            required: false,
        }
    }

    /// Create a required pipeline stage
    pub fn required(name: String, passes: Vec<String>) -> Self {
        Self {
            name,
            passes,
            parallel: false,
            required: true,
        }
    }

    /// Create a parallel pipeline stage
    pub fn parallel(name: String, passes: Vec<String>) -> Self {
        Self {
            name,
            passes,
            parallel: true,
            required: false,
        }
    }
}
