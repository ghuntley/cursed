/// LLVM Optimization Pass Registry and Management
/// 
/// Provides a comprehensive pass registry system for managing individual
/// optimization passes, their dependencies, execution order, and performance tracking.

use crate::error::{CursedError, Result};

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use tracing::{debug, info, warn, instrument, span, Level};

use inkwell::{
// };

/// Pass execution time categories for performance analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PassTimeCategory {
    VeryFast,   // < 1ms
    Fast,       // 1-10ms
    Medium,     // 10-100ms
    Slow,       // 100ms-1s
    VerySlow,   // > 1s
impl PassTimeCategory {
    pub fn from_duration(duration: Duration) -> Self {
        let millis = duration.as_millis();
        match millis {
        }
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
        }
    }
/// Individual optimization pass metadata
#[derive(Debug, Clone)]
pub struct OptimizationPass {
impl OptimizationPass {
    /// Create a new optimization pass
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            target_optimization_levels: vec![
        }
    }
    
    /// Add a dependency to this pass
    pub fn depends_on(mut self, pass_name: &str) -> Self {
        self.dependencies.push(pass_name.to_string());
        self
    /// Add a conflict with another pass
    pub fn conflicts_with(mut self, pass_name: &str) -> Self {
        self.conflicts.push(pass_name.to_string());
        self
    /// Set target optimization levels
    pub fn for_levels(mut self, levels: Vec<super::optimization::OptimizationLevel>) -> Self {
        self.target_optimization_levels = levels;
        self
    /// Mark as analysis pass
    pub fn analysis_pass(mut self) -> Self {
        self.is_analysis_pass = true;
        self.is_transformation_pass = false;
        self
    /// Mark as CURSED-specific pass
    pub fn cursed_specific(mut self) -> Self {
        self.cursed_specific = true;
        self
    /// Set estimated performance improvement
    pub fn estimated_improvement(mut self, improvement: f64) -> Self {
        self.estimated_improvement = improvement;
        self
    /// Set estimated compile time cost
    pub fn compile_time_cost(mut self, category: PassTimeCategory) -> Self {
        self.estimated_compile_time_cost = category;
        self
    }
}

/// Pass execution result
#[derive(Debug, Clone)]
pub struct PassResult {
/// Pass execution configuration
#[derive(Debug, Clone)]
pub struct PassConfiguration {
impl Default for PassConfiguration {
    fn default() -> Self {
        Self {
        }
    }
/// Pass registry for managing optimization passes
pub struct PassRegistry {
impl PassRegistry {
    /// Create a new pass registry
    #[instrument]
    pub fn new() -> Self {
        let registry = Self {
        
        registry.register_default_passes();
        registry
    /// Register default LLVM optimization passes
    fn register_default_passes(&self) {
        // Analysis passes
        self.register_pass(
            OptimizationPass::new(
                "Basic alias analysis for memory dependencies"
            ).analysis_pass()
            .compile_time_cost(PassTimeCategory::Fast)
        );
        
        self.register_pass(
            OptimizationPass::new(
                "Dominator tree construction"
            ).analysis_pass()
            .compile_time_cost(PassTimeCategory::Fast)
        );
        
        // Transformation passes
        self.register_pass(
            OptimizationPass::new(
                "Combine redundant instructions"
            ).estimated_improvement(1.2)
            .compile_time_cost(PassTimeCategory::Fast)
        );
        
        self.register_pass(
            OptimizationPass::new(
                "Reassociate expressions"
            ).depends_on("instruction-combining")
            .estimated_improvement(1.1)
            .compile_time_cost(PassTimeCategory::Fast)
        );
        
        self.register_pass(
            OptimizationPass::new(
                "Global value numbering"
            ).depends_on("basic-alias-analysis")
            .estimated_improvement(1.3)
            .compile_time_cost(PassTimeCategory::Medium)
        );
        
        self.register_pass(
            OptimizationPass::new(
                "Control flow graph simplification"
            ).estimated_improvement(1.2)
            .compile_time_cost(PassTimeCategory::Fast)
        );
        
        self.register_pass(
            OptimizationPass::new(
                "Promote memory to register (mem2reg)"
            ).depends_on("domtree")
            .estimated_improvement(1.4)
            .compile_time_cost(PassTimeCategory::Medium)
        );
        
        self.register_pass(
            OptimizationPass::new(
                "Unroll loops for better performance"
            ).for_levels(vec![
            ])
            .estimated_improvement(1.5)
            .compile_time_cost(PassTimeCategory::Slow)
        );
        
        self.register_pass(
            OptimizationPass::new(
                "Vectorize loops using SIMD instructions"
            ).for_levels(vec![
            ])
            .estimated_improvement(2.0)
            .compile_time_cost(PassTimeCategory::Slow)
        );
        
        self.register_pass(
            OptimizationPass::new(
                "Superword-level parallelism vectorization"
            ).for_levels(vec![
            ])
            .estimated_improvement(1.6)
            .compile_time_cost(PassTimeCategory::Medium)
        );
        
        // CURSED-specific passes
        self.register_pass(
            OptimizationPass::new(
                "Optimize goroutine stack allocations and context switches"
            ).cursed_specific()
            .estimated_improvement(1.3)
            .compile_time_cost(PassTimeCategory::Medium)
        );
        
        self.register_pass(
            OptimizationPass::new(
                "Optimize channel send/receive operations"
            ).cursed_specific()
            .estimated_improvement(1.4)
            .compile_time_cost(PassTimeCategory::Medium)
        );
        
        self.register_pass(
            OptimizationPass::new(
                "Optimize garbage collection allocations"
            ).cursed_specific()
            .estimated_improvement(1.2)
            .compile_time_cost(PassTimeCategory::Fast)
        );
        
        info!("Registered {} default optimization passes", self.get_pass_count());
    /// Register a new optimization pass
    #[instrument(skip(self, pass))]
    pub fn register_pass(&self, pass: OptimizationPass) {
        let pass_name = pass.name.clone();
        
        if let Ok(mut passes) = self.passes.write() {
            // Update dependency graph
            if let Ok(mut deps) = self.dependency_graph.write() {
                deps.insert(pass_name.clone(), pass.dependencies.iter().cloned().collect());
            passes.insert(pass_name.clone(), pass);
            debug!("Registered optimization pass: {}", pass_name);
        }
    }
    
    /// Get a pass by name
    pub fn get_pass(&self, name: &str) -> Option<OptimizationPass> {
        self.passes.read().ok()?.get(name).cloned()
    /// Get all registered passes
    pub fn get_all_passes(&self) -> Vec<OptimizationPass> {
        if let Ok(passes) = self.passes.read() {
            passes.values().cloned().collect()
        } else {
            Vec::new()
        }
    }
    
    /// Get pass count
    pub fn get_pass_count(&self) -> usize {
        self.passes.read().map(|p| p.len()).unwrap_or(0)
    /// Select passes for given optimization level and configuration
    #[instrument(skip(self, config))]
    pub fn select_passes(&self, config: &PassConfiguration) -> Result<Vec<String>> {
        let _span = span!(Level::DEBUG, "select_passes").entered();
        
        let passes = self.passes.read()
            .map_err(|_| CursedError::General("Failed to read pass registry".to_string()))?;
        
        let mut selected_passes = Vec::new();
        
        for (name, pass) in passes.iter() {
            // Skip if explicitly disabled
            if config.disabled_passes.contains(name) {
                continue;
            // Include if explicitly enabled
            if config.enabled_passes.contains(name) {
                selected_passes.push(name.clone());
                continue;
            // Check optimization level compatibility
            if !pass.target_optimization_levels.contains(&config.optimization_level) {
                continue;
            // Skip expensive passes if disabled
            if !config.enable_expensive_passes 
                && matches!(pass.estimated_compile_time_cost, PassTimeCategory::Slow | PassTimeCategory::VerySlow) {
                continue;
            // Skip CURSED passes if disabled
            if !config.enable_cursed_passes && pass.cursed_specific {
                continue;
            selected_passes.push(name.clone());
        // Sort passes by dependencies
        let ordered_passes = self.topological_sort(&selected_passes)?;
        
               ordered_passes.len(), config.optimization_level);
        
        Ok(ordered_passes)
    /// Topologically sort passes based on dependencies
    fn topological_sort(&self, selected_passes: &[String]) -> Result<Vec<String>> {
        let deps = self.dependency_graph.read()
            .map_err(|_| CursedError::General("Failed to read dependency graph".to_string()))?;
        
        let mut in_degree = HashMap::new();
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        
        // Initialize graph and in-degree count
        for pass_name in selected_passes {
            in_degree.insert(pass_name.clone(), 0);
            graph.insert(pass_name.clone(), Vec::new());
        // Build graph and calculate in-degrees
        for pass_name in selected_passes {
            if let Some(dependencies) = deps.get(pass_name) {
                for dep in dependencies {
                    if selected_passes.contains(dep) {
                        graph.get_mut(dep).unwrap().push(pass_name.clone());
                        *in_degree.get_mut(pass_name).unwrap() += 1;
                    }
                }
            }
        }
        
        // Kahn's algorithm for topological sorting
        let mut queue = VecDeque::new();
        let mut result = Vec::new();
        
        // Add all nodes with in-degree 0
        for (node, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(node.clone());
            }
        }
        
        while let Some(current) = queue.pop_front() {
            result.push(current.clone());
            
            // Process all neighbors
            if let Some(neighbors) = graph.get(&current) {
                for neighbor in neighbors {
                    let degree = in_degree.get_mut(neighbor).unwrap();
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }
        
        // Check for cycles
        if result.len() != selected_passes.len() {
            return Err(CursedError::General("Circular dependency detected in pass ordering".to_string()));
        Ok(result)
    /// Execute a pass and record results
    #[instrument(skip(self, pass_manager, function))]
    pub fn execute_function_pass(
        config: &PassConfiguration
    ) -> PassResult {
        let start_time = Instant::now();
        let _span = span!(Level::DEBUG, "execute_function_pass", pass = pass_name).entered();
        
        // Get detailed initial metrics
        let initial_metrics = self.collect_function_metrics(function);
        
        // Pre-execution validation
        if !function.verify(false) {
                  function.get_name().to_str().unwrap_or("unknown"), pass_name);
        // Execute the pass with comprehensive monitoring
        let success = if config.enable_pass_timing {
            // Execute with timeout monitoring
            let timeout = config.max_pass_execution_time;
            let execution_start = Instant::now();
            
            // Actual pass execution
            let result = pass_manager.run_on(function);
            
            let actual_time = execution_start.elapsed();
            if actual_time > timeout {
                warn!("Pass {} exceeded timeout of {:?} (took {:?})", pass_name, timeout, actual_time);
                false
            } else {
                debug!("Pass {} completed in {:?}", pass_name, actual_time);
                result
            }
        } else {
            pass_manager.run_on(function)
        
        let execution_time = start_time.elapsed();
        
        // Post-execution validation
        let post_execution_valid = function.verify(false);
        if !post_execution_valid {
                  function.get_name().to_str().unwrap_or("unknown"), pass_name);
        // Get detailed final metrics
        let final_metrics = self.collect_function_metrics(function);
        
        // Calculate comprehensive changes
        let instructions_added = final_metrics.instruction_count as i32 - initial_metrics.instruction_count as i32;
        let instructions_removed = -instructions_added.min(0);
        let changes_made = instructions_added != 0 || 
                          final_metrics.basic_block_count != initial_metrics.basic_block_count;
        
        // Calculate realistic performance impact based on actual changes
        let estimated_performance_impact = if let Some(pass) = self.get_pass(pass_name) {
            if changes_made {
                // Adjust estimated improvement based on actual instruction reduction
                let instruction_reduction_factor = if initial_metrics.instruction_count > 0 {
                    instructions_removed as f64 / initial_metrics.instruction_count as f64
                } else {
                    0.0
                
                // Scale base improvement by actual reduction achieved
                pass.estimated_improvement * (1.0 + instruction_reduction_factor * 0.5)
            } else {
                1.0 // No changes, no improvement
            }
        } else {
            1.0
        
        let result = PassResult {
            error_message: if success && post_execution_valid { 
                None 
            } else { 
                           if success { "succeeded but" } else { "" }))
        
        // Record detailed result
        self.record_pass_result(&result);
        
        debug!(
            "Function pass execution complete"
        );
        
        result
    /// Collect comprehensive function metrics
    fn collect_function_metrics(&self, function: &FunctionValue) -> FunctionMetrics {
        let mut instruction_count = 0;
        let mut basic_block_count = 0;
        let mut call_count = 0;
        let mut load_store_count = 0;
        let mut branch_count = 0;
        
        for basic_block in function.get_basic_blocks() {
            basic_block_count += 1;
            
            for instruction in basic_block.get_instructions() {
                instruction_count += 1;
                
                match instruction.get_opcode() {
                    inkwell::values::InstructionOpcode::Load | 
                    inkwell::values::InstructionOpcode::Br | 
                    _ => {}
                }
            }
        }
        
        FunctionMetrics {
        }
    }
    
    /// Record pass execution result
    fn record_pass_result(&self, result: &PassResult) {
        // Add to execution history
        if let Ok(mut history) = self.execution_history.lock() {
            history.push(result.clone());
            
            // Keep only last 1000 results
            if history.len() > 1000 {
                history.remove(0);
            }
        }
        
        // Add to pass-specific statistics
        if let Ok(mut stats) = self.pass_statistics.write() {
            let entry = stats.entry(result.pass_name.clone()).or_insert_with(Vec::new);
            entry.push(result.clone());
            
            // Keep only last 100 results per pass
            if entry.len() > 100 {
                entry.remove(0);
            }
        }
    /// Get pass execution statistics
    pub fn get_pass_statistics(&self, pass_name: &str) -> Option<Vec<PassResult>> {
        self.pass_statistics.read().ok()?.get(pass_name).cloned()
    /// Get overall execution statistics
    pub fn get_overall_statistics(&self) -> HashMap<String, (usize, Duration, f64)> {
        let mut stats = HashMap::new();
        
        if let Ok(pass_stats) = self.pass_statistics.read() {
            for (pass_name, results) in pass_stats.iter() {
                let total_executions = results.len();
                let total_time: Duration = results.iter().map(|r| r.execution_time).sum();
                let success_rate = results.iter()
                    .filter(|r| r.success)
                    .count() as f64 / total_executions as f64;
                
                stats.insert(pass_name.clone(), (total_executions, total_time, success_rate));
            }
        }
        
        stats
    /// Clear all statistics
    pub fn clear_statistics(&self) {
        if let Ok(mut history) = self.execution_history.lock() {
            history.clear();
        }
        if let Ok(mut stats) = self.pass_statistics.write() {
            stats.clear();
        }
        info!("Pass statistics cleared");
    /// Print pass registry summary
    #[instrument(skip(self))]
    pub fn print_summary(&self) {
        let total_passes = self.get_pass_count();
        let cursed_passes = self.get_all_passes().iter()
            .filter(|p| p.cursed_specific)
            .count();
        
        println!("📋 Pass Registry Summary:");
        println!("   Total passes: {}", total_passes);
        println!("   CURSED-specific passes: {}", cursed_passes);
        println!("   Standard LLVM passes: {}", total_passes - cursed_passes);
        
        let stats = self.get_overall_statistics();
        if !stats.is_empty() {
            println!("   Execution statistics available for {} passes", stats.len());
            
            // Show top 5 most executed passes
            let mut sorted_stats: Vec<_> = stats.iter().collect();
            sorted_stats.sort_by(|a, b| b.1.0.cmp(&a.1.0));
            
            println!("   Most executed passes:");
            for (pass_name, (count, total_time, success_rate)) in sorted_stats.iter().take(5) {
                         pass_name, count, total_time, success_rate * 100.0);
            }
        }
    }
}

/// Function metrics for detailed analysis
#[derive(Debug, Clone)]
pub struct FunctionMetrics {
impl Default for PassRegistry {
    fn default() -> Self {
        Self::new()
    }
}
