//! Compiler pass management and orchestration
//! 
//! Provides infrastructure for managing and executing compiler optimization passes
//! in the correct order with proper dependency handling.

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use crate::error::Result;

/// Manages the execution of compiler optimization passes
#[derive(Debug, Clone)]
pub struct CompilerPassManager {
    passes: Vec<CompilerPass>,
    pass_graph: HashMap<String, Vec<String>>, // Dependencies
    execution_order: Vec<String>,
    statistics: PassExecutionStatistics,
}

/// Individual compiler pass definition
#[derive(Debug, Clone)]
pub struct CompilerPass {
    pub name: String,
    pub description: String,
    pub pass_type: PassType,
    pub optimization_level: OptimizationLevel,
    pub dependencies: Vec<String>,
    pub estimated_duration: Duration,
    pub effectiveness: f64, // 0.0 to 1.0
}

/// Type of optimization pass
#[derive(Debug, Clone, PartialEq)]
pub enum PassType {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}

// Import canonical OptimizationLevel from optimization_config
pub use crate::optimization::config::OptimizationLevel;

/// Statistics for pass execution
#[derive(Debug, Clone)]
pub struct PassExecutionStatistics {
    pub total_passes: usize,
    pub successful_passes: usize,
    pub failed_passes: usize,
    pub total_execution_time: Duration,
    pub pass_timings: HashMap<String, Duration>,
    pub effectiveness_scores: HashMap<String, f64>,
}

/// Result of executing a compiler pass
#[derive(Debug, Clone)]
pub struct PassExecutionResult {
    pub pass_name: String,
    pub success: bool,
    pub execution_time: Duration,
    pub effectiveness_score: f64,
    pub transformations_applied: usize,
    pub error_message: Option<String>,
}

/// Configuration for pass execution
#[derive(Debug, Clone)]
pub struct PassExecutionConfig {
    pub enable_parallel_execution: bool,
    pub max_parallel_passes: usize,
    pub timeout_per_pass: Duration,
    pub continue_on_failure: bool,
    pub collect_detailed_stats: bool,
}

/// Dead code eliminator pass
#[derive(Debug, Clone)]
pub struct DeadCodeEliminator {
    pub aggressive_mode: bool,
    pub preserve_debug_info: bool,
}

/// Constant propagator pass
#[derive(Debug, Clone)]
pub struct ConstantPropagator {
    pub fold_arithmetic: bool,
    pub propagate_globals: bool,
}

/// Loop optimizer pass
#[derive(Debug, Clone)]
pub struct LoopOptimizer {
    pub unroll_small_loops: bool,
    pub vectorize_loops: bool,
    pub max_unroll_count: usize,
}

/// Inlining decision information
#[derive(Debug, Clone)]
pub struct InliningDecision {
    pub function_name: String,
    pub should_inline: bool,
    pub cost_benefit_ratio: f64,
    pub size_increase: usize,
}

/// Register allocator pass
#[derive(Debug, Clone)]
pub struct RegisterAllocator {
    pub algorithm: RegisterAllocationAlgorithm,
    pub spill_optimization: bool,
}

/// Register allocation algorithm variants
#[derive(Debug, Clone)]
pub enum RegisterAllocationAlgorithm {
    LinearScan,
    GraphColoring,
    Greedy,
}

impl CompilerPassManager {
    /// Creates a new compiler pass manager
    pub fn new() -> Self {
        Self {
            passes: Vec::new(),
            pass_graph: HashMap::new(),
            execution_order: Vec::new(),
            statistics: PassExecutionStatistics::new(),
        }
    }

    /// Adds a compiler pass to the manager
    pub fn add_pass(&mut self, pass: CompilerPass) {
        self.pass_graph.insert(pass.name.clone(), pass.dependencies.clone());
        self.passes.push(pass);
    }

    /// Computes the execution order based on dependencies
    pub fn compute_execution_order(&mut self) -> Result<()> {
        self.execution_order = self.topological_sort()?;
        Ok(())
    }

    /// Executes all passes in the computed order
    pub fn execute_all_passes(&mut self, config: &PassExecutionConfig) -> Result<Vec<PassExecutionResult>> {
        if self.execution_order.is_empty() {
            self.compute_execution_order()?;
        }

        let mut results = Vec::new();
        let start_time = Instant::now();

        for pass_name in &self.execution_order.clone() {
            if let Some(pass) = self.find_pass(pass_name) {
                let result = self.execute_pass(&pass, config)?;
                
                if !result.success && !config.continue_on_failure {
                    return Err(crate::error::Error::General(
                        format!("Pass '{}' failed: {}", pass_name, 
                               result.error_message.unwrap_or_else(|| "Unknown error".to_string()))
                    ));
                }

                self.update_statistics(&result);
                results.push(result);
            }
        }

        self.statistics.total_execution_time = start_time.elapsed();
        Ok(results)
    }

    /// Executes a specific pass
    pub fn execute_pass(&mut self, pass: &CompilerPass, config: &PassExecutionConfig) -> Result<PassExecutionResult> {
        let start_time = Instant::now();
        
        // Simulate pass execution
        let (success, transformations_applied, error_message) = self.simulate_pass_execution(pass);
        
        let execution_time = start_time.elapsed();
        let effectiveness_score = if success {
            pass.effectiveness * (transformations_applied as f64 / 10.0).min(1.0)
        } else {
            0.0
        };

        Ok(PassExecutionResult {
            pass_name: pass.name.clone(),
            success,
            execution_time,
            effectiveness_score,
            transformations_applied,
            error_message,
        })
    }

    fn simulate_pass_execution(&self, pass: &CompilerPass) -> (bool, usize, Option<String>) {
        // Simulate different pass behaviors based on type and name
        match pass.pass_type {
            PassType::Analysis => (true, 0, None), // Analysis passes don't transform
            PassType::Transformation => {
                let transformations = match pass.name.as_str() {
                    name if name.contains("inline") => 15,
                    name if name.contains("dead_code") => 25,
                    name if name.contains("constant") => 10,
                    name if name.contains("loop") => 8,
                    _ => 5,
                };
                (true, transformations, None)
            },
            PassType::Verification => (true, 0, None),
            PassType::Cleanup => (true, 3, None),
        }
    }

    fn find_pass(&self, name: &str) -> Option<CompilerPass> {
        self.passes.iter().find(|p| p.name == name).cloned()
    }

    fn topological_sort(&self) -> Result<Vec<String>> {
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut result = Vec::new();
        let mut queue = VecDeque::new();

        // Initialize in-degree for all passes
        for pass in &self.passes {
            in_degree.insert(pass.name.clone(), 0);
        }

        // Calculate in-degrees
        for (pass, deps) in &self.pass_graph {
            for dep in deps {
                *in_degree.entry(dep.clone()).or_insert(0) += 1;
            }
        }

        // Add passes with no dependencies to queue
        for (pass, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(pass.clone());
            }
        }

        // Process queue
        while let Some(pass) = queue.pop_front() {
            result.push(pass.clone());
            
            if let Some(deps) = self.pass_graph.get(&pass) {
                for dep in deps {
                    if let Some(degree) = in_degree.get_mut(dep) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(dep.clone());
                        }
                    }
                }
            }
        }

        if result.len() != self.passes.len() {
            return Err(crate::error::Error::General("Circular dependency detected in compiler passes".to_string()));
        }

        Ok(result)
    }

    fn update_statistics(&mut self, result: &PassExecutionResult) {
        self.statistics.total_passes += 1;
        
        if result.success {
            self.statistics.successful_passes += 1;
        } else {
            self.statistics.failed_passes += 1;
        }

        self.statistics.pass_timings.insert(result.pass_name.clone(), result.execution_time);
        self.statistics.effectiveness_scores.insert(result.pass_name.clone(), result.effectiveness_score);
    }

    /// Gets execution statistics
    pub fn get_statistics(&self) -> &PassExecutionStatistics {
        &self.statistics
    }

    /// Resets statistics
    pub fn reset_statistics(&mut self) {
        self.statistics = PassExecutionStatistics::new();
    }

    /// Gets the computed execution order
    pub fn get_execution_order(&self) -> &[String] {
        &self.execution_order
    }

    /// Gets all registered passes
    pub fn get_passes(&self) -> &[CompilerPass] {
        &self.passes
    }
}

impl PassExecutionStatistics {
    fn new() -> Self {
        Self {
            total_passes: 0,
            successful_passes: 0,
            failed_passes: 0,
            total_execution_time: Duration::new(0, 0),
            pass_timings: HashMap::new(),
            effectiveness_scores: HashMap::new(),
        }
    }

    /// Gets the success rate as a percentage
    pub fn success_rate(&self) -> f64 {
        if self.total_passes == 0 {
            0.0
        } else {
            (self.successful_passes as f64 / self.total_passes as f64) * 100.0
        }
    }

    /// Gets the average effectiveness score
    pub fn average_effectiveness(&self) -> f64 {
        if self.effectiveness_scores.is_empty() {
            0.0
        } else {
            self.effectiveness_scores.values().sum::<f64>() / self.effectiveness_scores.len() as f64
        }
    }

    /// Gets the total time spent on successful passes
    pub fn successful_pass_time(&self) -> Duration {
        self.pass_timings.values().sum()
    }
}

impl Default for PassExecutionConfig {
    fn default() -> Self {
        Self {
            enable_parallel_execution: false, // Sequential by default for deterministic results
            max_parallel_passes: 4,
            timeout_per_pass: Duration::from_secs(60),
            continue_on_failure: false,
            collect_detailed_stats: true,
        }
    }
}

/// Creates a standard set of compiler passes
pub fn create_standard_passes() -> Vec<CompilerPass> {
    vec![
        CompilerPass {
            name: "dead_code_elimination".to_string(),
            description: "Removes dead code and unused variables".to_string(),
            pass_type: PassType::Transformation,
            optimization_level: OptimizationLevel::Aggressive,
            dependencies: vec!["control_flow_analysis".to_string()],
            estimated_duration: Duration::from_millis(200),
            effectiveness: 0.8,
        },
        CompilerPass {
            name: "constant_propagation".to_string(),
            description: "Propagates constant values".to_string(),
            pass_type: PassType::Transformation,
            optimization_level: OptimizationLevel::Aggressive,
            dependencies: vec![],
            estimated_duration: Duration::from_millis(150),
            effectiveness: 0.7,
        },
        CompilerPass {
            name: "function_inlining".to_string(),
            description: "Inlines small functions".to_string(),
            pass_type: PassType::Transformation,
            optimization_level: OptimizationLevel::Default,
            dependencies: vec!["call_graph_analysis".to_string()],
            estimated_duration: Duration::from_millis(300),
            effectiveness: 0.6,
        },
        CompilerPass {
            name: "loop_optimization".to_string(),
            description: "Optimizes loop structures".to_string(),
            pass_type: PassType::Transformation,
            optimization_level: OptimizationLevel::Default,
            dependencies: vec!["loop_analysis".to_string()],
            estimated_duration: Duration::from_millis(400),
            effectiveness: 0.75,
        },
        CompilerPass {
            name: "control_flow_analysis".to_string(),
            description: "Analyzes control flow patterns".to_string(),
            pass_type: PassType::Analysis,
            optimization_level: OptimizationLevel::None,
            dependencies: vec![],
            estimated_duration: Duration::from_millis(100),
            effectiveness: 1.0,
        },
        CompilerPass {
            name: "call_graph_analysis".to_string(),
            description: "Builds call graph for function analysis".to_string(),
            pass_type: PassType::Analysis,
            optimization_level: OptimizationLevel::None,
            dependencies: vec![],
            estimated_duration: Duration::from_millis(120),
            effectiveness: 1.0,
        },
        CompilerPass {
            name: "loop_analysis".to_string(),
            description: "Analyzes loop structures and nesting".to_string(),
            pass_type: PassType::Analysis,
            optimization_level: OptimizationLevel::None,
            dependencies: vec!["control_flow_analysis".to_string()],
            estimated_duration: Duration::from_millis(80),
            effectiveness: 1.0,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pass_manager_creation() {
        let mut manager = CompilerPassManager::new();
        let passes = create_standard_passes();
        
        for pass in passes {
            manager.add_pass(pass);
        }
        
        assert_eq!(manager.get_passes().len(), 7);
    }

    #[test]
    fn test_execution_order_computation() {
        let mut manager = CompilerPassManager::new();
        let passes = create_standard_passes();
        
        for pass in passes {
            manager.add_pass(pass);
        }
        
        manager.compute_execution_order().unwrap();
        let order = manager.get_execution_order();
        
        // Analysis passes should come before transformation passes
        let analysis_pos = order.iter().position(|p| p == "control_flow_analysis").unwrap();
        let transform_pos = order.iter().position(|p| p == "dead_code_elimination").unwrap();
        assert!(analysis_pos < transform_pos);
    }

    #[test]
    fn test_pass_execution() {
        let mut manager = CompilerPassManager::new();
        let config = PassExecutionConfig::default();
        let passes = create_standard_passes();
        
        for pass in passes {
            manager.add_pass(pass);
        }
        
        let results = manager.execute_all_passes(&config).unwrap();
        assert_eq!(results.len(), 7);
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_passes, 7);
        assert_eq!(stats.successful_passes, 7);
        assert_eq!(stats.failed_passes, 0);
    }
}
