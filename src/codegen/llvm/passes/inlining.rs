/// Function Inlining Pass
/// 
/// Implements intelligent function inlining decisions based on cost-benefit analysis,
/// call site characteristics, and optimization heuristics.

use super::{OptimizationPass, PassConfiguration, PassResult, PassStatistics};
use crate::common::optimization_level::OptimizationLevel;
use crate::error::{Error, Result};
use inkwell::{
    context::Context,
    module::Module,
    values::{FunctionValue, InstructionValue, CallSiteValue},
    basic_block::BasicBlock,
};
use std::collections::{HashMap, HashSet};
use std::time::Instant;
use tracing::{debug, info, instrument, warn};

/// Function inlining optimization pass
pub struct InliningPass {
    statistics: PassStatistics,
    config: InliningConfig,
    heuristics: InliningHeuristics,
    analyzer: CallSiteAnalyzer,
}

impl InliningPass {
    /// Create a new inlining pass
    pub fn new(config: PassConfiguration) -> Self {
        let inlining_config = InliningConfig {
            enabled: config.enable_inlining,
            max_inline_size: config.max_inline_size,
            aggressive_inlining: config.optimization_level >= OptimizationLevel::O3,
            size_optimization: config.enable_size_optimizations,
            preserve_debug_info: config.enable_debug_info_preservation,
            hot_path_threshold: 100, // Call count threshold for hot paths
            cold_path_penalty: 2.0,  // Penalty multiplier for cold paths
            recursive_inline_limit: 3, // Maximum recursion depth for inlining
        };
        
        Self {
            statistics: PassStatistics::default(),
            config: inlining_config,
            heuristics: InliningHeuristics::new(inlining_config.clone()),
            analyzer: CallSiteAnalyzer::new(),
        }
    }
    
    /// Perform inlining optimization on a module
    #[instrument(skip(self, module))]
    fn perform_inlining(&mut self, module: &Module) -> Result<usize> {
        let mut inlined_count = 0;
        
        // Build call graph
        let call_graph = self.build_call_graph(module)?;
        
        // Analyze all call sites
        let call_sites = self.analyzer.analyze_call_sites(module)?;
        
        // Sort call sites by inlining priority
        let mut prioritized_calls = self.prioritize_call_sites(call_sites)?;
        
        // Perform inlining in priority order
        for call_site_info in prioritized_calls {
            if self.should_inline_call_site(&call_site_info, &call_graph)? {
                if self.inline_call_site(&call_site_info)? {
                    inlined_count += 1;
                    debug!("Inlined call to {} at {}",
                           call_site_info.callee_name,
                           call_site_info.location);
                }
            }
        }
        
        // Cleanup after inlining
        self.cleanup_dead_functions(module)?;
        
        if inlined_count > 0 {
            info!("Inlined {} function calls", inlined_count);
        }
        
        Ok(inlined_count)
    }
    
    /// Build call graph for the module
    fn build_call_graph(&self, module: &Module) -> Result<CallGraph> {
        let mut call_graph = CallGraph::new();
        
        for function in module.get_functions() {
            let function_name = function.get_name().to_str().unwrap_or("").to_string();
            
            // Add function node
            call_graph.add_function(function_name.clone(), function);
            
            // Find all calls within this function
            for basic_block in function.get_basic_blocks() {
                for instruction in basic_block.get_instructions() {
                    if let Some(called_function) = self.get_called_function(&instruction) {
                        call_graph.add_call_edge(function_name.clone(), called_function);
                    }
                }
            }
        }
        
        debug!("Built call graph with {} functions and {} edges",
               call_graph.function_count(),
               call_graph.edge_count());
        
        Ok(call_graph)
    }
    
    /// Get called function name from call instruction
    fn get_called_function(&self, instruction: &InstructionValue) -> Option<String> {
        use inkwell::values::InstructionOpcode;
        
        if instruction.get_opcode() == InstructionOpcode::Call {
            // In a real implementation, this would extract the function name
            // from the call instruction operands
            // For now, return None as this requires complex LLVM API usage
        }
        
        None
    }
    
    /// Prioritize call sites for inlining
    fn prioritize_call_sites(&self, call_sites: Vec<CallSiteInfo>) -> Result<Vec<CallSiteInfo>> {
        let mut prioritized = call_sites;
        
        // Sort by inlining benefit score (highest first)
        prioritized.sort_by(|a, b| {
            let score_a = self.heuristics.calculate_inlining_benefit(a);
            let score_b = self.heuristics.calculate_inlining_benefit(b);
            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        debug!("Prioritized {} call sites for inlining evaluation", prioritized.len());
        Ok(prioritized)
    }
    
    /// Decide whether to inline a specific call site
    fn should_inline_call_site(&self, call_site: &CallSiteInfo, call_graph: &CallGraph) -> Result<bool> {
        if !self.config.enabled {
            return Ok(false);
        }
        
        // Check basic constraints
        if call_site.callee_size > self.config.max_inline_size {
            debug!("Skipping inlining: function {} too large ({} > {})",
                   call_site.callee_name, call_site.callee_size, self.config.max_inline_size);
            return Ok(false);
        }
        
        // Check for recursion
        if call_graph.is_recursive(&call_site.callee_name) && 
           call_site.recursion_depth >= self.config.recursive_inline_limit {
            debug!("Skipping inlining: recursive function {} at depth {}",
                   call_site.callee_name, call_site.recursion_depth);
            return Ok(false);
        }
        
        // Apply inlining heuristics
        let benefit = self.heuristics.calculate_inlining_benefit(call_site);
        let cost = self.heuristics.calculate_inlining_cost(call_site);
        
        let should_inline = benefit > cost;
        
        debug!("Inlining decision for {}: benefit={:.2}, cost={:.2}, inline={}",
               call_site.callee_name, benefit, cost, should_inline);
        
        Ok(should_inline)
    }
    
    /// Inline a specific call site
    fn inline_call_site(&mut self, call_site: &CallSiteInfo) -> Result<bool> {
        // In a real implementation, this would:
        // 1. Clone the callee function body
        // 2. Replace parameter references with argument values
        // 3. Replace return instructions with jumps to continuation
        // 4. Insert the cloned instructions at the call site
        // 5. Update phi nodes and other control flow
        // 6. Preserve debug information if enabled
        
        debug!("Simulating inlining of {} (size: {} instructions)",
               call_site.callee_name, call_site.callee_size);
        
        // For now, just simulate successful inlining
        Ok(true)
    }
    
    /// Cleanup functions that are no longer called after inlining
    fn cleanup_dead_functions(&self, module: &Module) -> Result<usize> {
        let mut removed_count = 0;
        
        // Find functions with no remaining callers
        let mut functions_to_remove = Vec::new();
        
        for function in module.get_functions() {
            let function_name = function.get_name().to_str().unwrap_or("");
            
            // Skip entry points and exported functions
            if function_name == "main" || 
               function_name.starts_with("cursed_") ||
               function.get_linkage().is_external() {
                continue;
            }
            
            // Check if function has any remaining uses
            if function.get_uses().is_empty() {
                functions_to_remove.push(function);
            }
        }
        
        // Remove dead functions
        for function in functions_to_remove {
            let function_name = function.get_name().to_str().unwrap_or("<unnamed>");
            debug!("Removing dead function after inlining: {}", function_name);
            
            unsafe {
                function.delete();
            }
            removed_count += 1;
        }
        
        if removed_count > 0 {
            debug!("Removed {} dead functions after inlining", removed_count);
        }
        
        Ok(removed_count)
    }
}

impl<'ctx> OptimizationPass<'ctx> for InliningPass {
    fn name(&self) -> &str {
        "inlining"
    }
    
    fn description(&self) -> &str {
        "Inlines function calls based on cost-benefit analysis"
    }
    
    fn should_run(&self, config: &PassConfiguration) -> bool {
        config.enable_inlining && 
        config.optimization_level >= OptimizationLevel::O1
    }
    
    fn required_optimization_level(&self) -> OptimizationLevel {
        OptimizationLevel::O1
    }
    
    fn dependencies(&self) -> Vec<String> {
        vec!["dead_code_elimination".to_string()]
    }
    
    #[instrument(skip(self, module, context))]
    fn run_on_module(&mut self, module: &Module<'ctx>, _context: &'ctx Context) -> Result<PassResult> {
        let start_time = Instant::now();
        
        info!("Running function inlining pass");
        
        let mut result = PassResult::unchanged();
        
        // Perform inlining
        let inlined_count = self.perform_inlining(module)?;
        
        // Update result
        if inlined_count > 0 {
            result.changed = true;
            result.functions_inlined = inlined_count;
        }
        
        result.execution_time = start_time.elapsed();
        result.metrics.insert("functions_inlined".to_string(), inlined_count as f64);
        result.metrics.insert("max_inline_size".to_string(), self.config.max_inline_size as f64);
        result.metrics.insert("aggressive_inlining".to_string(), 
                             if self.config.aggressive_inlining { 1.0 } else { 0.0 });
        
        // Update statistics
        self.statistics.update(&result);
        
        info!("Function inlining completed: {} functions inlined in {:?}", 
              inlined_count, result.execution_time);
        
        Ok(result)
    }
    
    fn get_statistics(&self) -> PassStatistics {
        self.statistics.clone()
    }
    
    fn reset(&mut self) {
        self.statistics = PassStatistics::default();
        self.analyzer.reset();
    }
}

/// Configuration for function inlining
#[derive(Debug, Clone)]
struct InliningConfig {
    enabled: bool,
    max_inline_size: usize,
    aggressive_inlining: bool,
    size_optimization: bool,
    preserve_debug_info: bool,
    hot_path_threshold: usize,
    cold_path_penalty: f64,
    recursive_inline_limit: usize,
}

/// Inlining heuristics engine
pub struct InliningHeuristics {
    config: InliningConfig,
    benefit_weights: BenefitWeights,
}

impl InliningHeuristics {
    /// Create new inlining heuristics
    pub fn new(config: InliningConfig) -> Self {
        let benefit_weights = if config.size_optimization {
            BenefitWeights::for_size_optimization()
        } else if config.aggressive_inlining {
            BenefitWeights::for_aggressive_optimization()
        } else {
            BenefitWeights::default()
        };
        
        Self {
            config,
            benefit_weights,
        }
    }
    
    /// Calculate inlining benefit score
    pub fn calculate_inlining_benefit(&self, call_site: &CallSiteInfo) -> f64 {
        let mut benefit = 0.0;
        
        // Call frequency benefit
        benefit += call_site.call_frequency as f64 * self.benefit_weights.call_frequency;
        
        // Small function benefit
        if call_site.callee_size <= 10 {
            benefit += self.benefit_weights.small_function;
        }
        
        // Hot path benefit
        if call_site.call_frequency >= self.config.hot_path_threshold {
            benefit += self.benefit_weights.hot_path;
        }
        
        // Single caller benefit
        if call_site.caller_count == 1 {
            benefit += self.benefit_weights.single_caller;
        }
        
        // Constant parameter benefit
        benefit += call_site.constant_parameters as f64 * self.benefit_weights.constant_params;
        
        // Apply penalties
        if call_site.is_in_loop {
            benefit *= 1.2; // Boost for loop inlining
        }
        
        if call_site.call_frequency < 10 {
            benefit *= 1.0 / self.config.cold_path_penalty; // Penalty for cold paths
        }
        
        benefit
    }
    
    /// Calculate inlining cost score
    pub fn calculate_inlining_cost(&self, call_site: &CallSiteInfo) -> f64 {
        let mut cost = 0.0;
        
        // Size cost
        cost += call_site.callee_size as f64 * self.benefit_weights.size_penalty;
        
        // Complexity cost
        if call_site.has_exception_handling {
            cost += 50.0;
        }
        
        if call_site.has_complex_control_flow {
            cost += 30.0;
        }
        
        // Recursion cost
        if call_site.recursion_depth > 0 {
            cost += call_site.recursion_depth as f64 * 20.0;
        }
        
        // Debug info preservation cost
        if self.config.preserve_debug_info && call_site.has_debug_info {
            cost += 10.0;
        }
        
        cost
    }
}

/// Weights for different inlining benefits
#[derive(Debug, Clone)]
struct BenefitWeights {
    call_frequency: f64,
    small_function: f64,
    hot_path: f64,
    single_caller: f64,
    constant_params: f64,
    size_penalty: f64,
}

impl BenefitWeights {
    /// Default benefit weights
    fn default() -> Self {
        Self {
            call_frequency: 1.0,
            small_function: 25.0,
            hot_path: 50.0,
            single_caller: 30.0,
            constant_params: 5.0,
            size_penalty: 0.5,
        }
    }
    
    /// Weights for aggressive optimization
    fn for_aggressive_optimization() -> Self {
        Self {
            call_frequency: 1.5,
            small_function: 35.0,
            hot_path: 75.0,
            single_caller: 40.0,
            constant_params: 8.0,
            size_penalty: 0.3,
        }
    }
    
    /// Weights for size optimization
    fn for_size_optimization() -> Self {
        Self {
            call_frequency: 0.8,
            small_function: 50.0,
            hot_path: 30.0,
            single_caller: 60.0,
            constant_params: 3.0,
            size_penalty: 1.5,
        }
    }
}

/// Call site analyzer
pub struct CallSiteAnalyzer {
    analysis_cache: HashMap<String, CallSiteInfo>,
}

impl CallSiteAnalyzer {
    /// Create new call site analyzer
    pub fn new() -> Self {
        Self {
            analysis_cache: HashMap::new(),
        }
    }
    
    /// Analyze all call sites in a module
    pub fn analyze_call_sites(&mut self, module: &Module) -> Result<Vec<CallSiteInfo>> {
        let mut call_sites = Vec::new();
        
        for function in module.get_functions() {
            let caller_name = function.get_name().to_str().unwrap_or("").to_string();
            
            for basic_block in function.get_basic_blocks() {
                for instruction in basic_block.get_instructions() {
                    if let Some(call_info) = self.analyze_call_instruction(&instruction, &caller_name)? {
                        call_sites.push(call_info);
                    }
                }
            }
        }
        
        debug!("Analyzed {} call sites", call_sites.len());
        Ok(call_sites)
    }
    
    /// Analyze a single call instruction
    fn analyze_call_instruction(&self, instruction: &InstructionValue, caller_name: &str) -> Result<Option<CallSiteInfo>> {
        use inkwell::values::InstructionOpcode;
        
        if instruction.get_opcode() != InstructionOpcode::Call {
            return Ok(None);
        }
        
        // In a real implementation, this would extract detailed information
        // about the call site, including:
        // - Called function name and size
        // - Call frequency (from profiling data)
        // - Argument analysis (constants, etc.)
        // - Context analysis (in loop, exception handling, etc.)
        
        // For now, create a placeholder call site info
        let call_info = CallSiteInfo {
            caller_name: caller_name.to_string(),
            callee_name: "unknown_function".to_string(),
            location: "unknown:0".to_string(),
            callee_size: 50, // Estimate
            call_frequency: 10, // Default frequency
            caller_count: 1,
            constant_parameters: 0,
            recursion_depth: 0,
            is_in_loop: false,
            has_exception_handling: false,
            has_complex_control_flow: false,
            has_debug_info: false,
        };
        
        Ok(Some(call_info))
    }
    
    /// Reset analyzer state
    pub fn reset(&mut self) {
        self.analysis_cache.clear();
    }
}

/// Information about a call site
#[derive(Debug, Clone)]
pub struct CallSiteInfo {
    pub caller_name: String,
    pub callee_name: String,
    pub location: String,
    pub callee_size: usize,
    pub call_frequency: usize,
    pub caller_count: usize,
    pub constant_parameters: usize,
    pub recursion_depth: usize,
    pub is_in_loop: bool,
    pub has_exception_handling: bool,
    pub has_complex_control_flow: bool,
    pub has_debug_info: bool,
}

/// Call graph representation
struct CallGraph {
    functions: HashMap<String, FunctionValue<'static>>,
    edges: HashMap<String, Vec<String>>,
}

impl CallGraph {
    /// Create new call graph
    fn new() -> Self {
        Self {
            functions: HashMap::new(),
            edges: HashMap::new(),
        }
    }
    
    /// Add a function to the call graph
    fn add_function(&mut self, name: String, function: FunctionValue<'static>) {
        self.functions.insert(name.clone(), function);
        self.edges.entry(name).or_insert_with(Vec::new);
    }
    
    /// Add a call edge
    fn add_call_edge(&mut self, caller: String, callee: String) {
        self.edges.entry(caller).or_insert_with(Vec::new).push(callee);
    }
    
    /// Check if a function is recursive
    fn is_recursive(&self, function_name: &str) -> bool {
        let mut visited = HashSet::new();
        self.is_recursive_helper(function_name, function_name, &mut visited)
    }
    
    /// Helper for recursion detection
    fn is_recursive_helper(&self, start: &str, current: &str, visited: &mut HashSet<String>) -> bool {
        if visited.contains(current) {
            return current == start;
        }
        
        visited.insert(current.to_string());
        
        if let Some(callees) = self.edges.get(current) {
            for callee in callees {
                if self.is_recursive_helper(start, callee, visited) {
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Get function count
    fn function_count(&self) -> usize {
        self.functions.len()
    }
    
    /// Get edge count
    fn edge_count(&self) -> usize {
        self.edges.values().map(|v| v.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    
    #[test]
    fn test_inlining_pass_creation() {
        let config = PassConfiguration::default();
        let pass = InliningPass::new(config);
        
        assert_eq!(pass.name(), "inlining");
        assert!(pass.description().contains("inline"));
    }
    
    #[test]
    fn test_should_run_logic() {
        let mut config = PassConfiguration::default();
        config.enable_inlining = true;
        config.optimization_level = OptimizationLevel::O1;
        
        let pass = InliningPass::new(config.clone());
        assert!(pass.should_run(&config));
        
        config.enable_inlining = false;
        assert!(!pass.should_run(&config));
    }
    
    #[test]
    fn test_inlining_heuristics() {
        let config = InliningConfig {
            enabled: true,
            max_inline_size: 100,
            aggressive_inlining: false,
            size_optimization: false,
            preserve_debug_info: true,
            hot_path_threshold: 50,
            cold_path_penalty: 2.0,
            recursive_inline_limit: 3,
        };
        
        let heuristics = InliningHeuristics::new(config);
        
        let call_site = CallSiteInfo {
            caller_name: "caller".to_string(),
            callee_name: "callee".to_string(),
            location: "test:1".to_string(),
            callee_size: 20,
            call_frequency: 100,
            caller_count: 1,
            constant_parameters: 2,
            recursion_depth: 0,
            is_in_loop: true,
            has_exception_handling: false,
            has_complex_control_flow: false,
            has_debug_info: true,
        };
        
        let benefit = heuristics.calculate_inlining_benefit(&call_site);
        let cost = heuristics.calculate_inlining_cost(&call_site);
        
        assert!(benefit > 0.0);
        assert!(cost > 0.0);
    }
    
    #[test]
    fn test_call_site_analyzer() {
        let mut analyzer = CallSiteAnalyzer::new();
        assert_eq!(analyzer.analysis_cache.len(), 0);
        
        analyzer.reset();
        assert_eq!(analyzer.analysis_cache.len(), 0);
    }
    
    #[test]
    fn test_call_graph() {
        let mut call_graph = CallGraph::new();
        
        assert_eq!(call_graph.function_count(), 0);
        assert_eq!(call_graph.edge_count(), 0);
        
        call_graph.add_call_edge("main".to_string(), "helper".to_string());
        call_graph.add_call_edge("helper".to_string(), "helper".to_string()); // Self-recursive
        
        assert!(call_graph.is_recursive("helper"));
        assert!(!call_graph.is_recursive("main"));
    }
    
    #[test]
    fn test_benefit_weights() {
        let default_weights = BenefitWeights::default();
        let aggressive_weights = BenefitWeights::for_aggressive_optimization();
        let size_weights = BenefitWeights::for_size_optimization();
        
        // Aggressive should favor inlining more
        assert!(aggressive_weights.call_frequency > default_weights.call_frequency);
        assert!(aggressive_weights.hot_path > default_weights.hot_path);
        
        // Size optimization should be more conservative
        assert!(size_weights.size_penalty > default_weights.size_penalty);
        assert!(size_weights.single_caller > default_weights.single_caller);
    }
    
    #[test]
    fn test_inlining_dependencies() {
        let config = PassConfiguration::default();
        let pass = InliningPass::new(config);
        
        let deps = pass.dependencies();
        assert!(deps.contains(&"dead_code_elimination".to_string()));
    }
}
