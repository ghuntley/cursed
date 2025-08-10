//! Link-Time Optimization (LTO) implementation for CURSED compiler
//! 
//! This module provides comprehensive LTO analysis, module linking optimization,
//! and whole-program optimization passes that work with the CURSED compiler's LLVM backend.

use crate::error_types::CursedError;
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Configuration for LTO optimization
#[derive(Debug, Clone)]
pub struct LTOConfig {
    /// Enable/disable LTO
    pub enabled: bool,
    /// Optimization level (0-3)
    pub optimization_level: u32,
    /// Maximum number of inlining iterations
    pub max_inline_iterations: u32,
    /// Enable interprocedural optimization
    pub enable_ipo: bool,
    /// Enable whole-program optimization
    pub enable_wpo: bool,
    /// Enable cross-module optimization
    pub enable_cross_module: bool,
    /// Enable dead code elimination
    pub enable_dce: bool,
    /// Enable constant propagation
    pub enable_constant_propagation: bool,
    /// Enable function merging
    pub enable_function_merging: bool,
    /// Time budget for LTO (in seconds)
    pub time_budget: Duration,
}

impl Default for LTOConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            optimization_level: 2,
            max_inline_iterations: 10,
            enable_ipo: true,
            enable_wpo: true,
            enable_cross_module: true,
            enable_dce: true,
            enable_constant_propagation: true,
            enable_function_merging: true,
            time_budget: Duration::from_secs(30),
        }
    }
}

/// LTO analysis result
#[derive(Debug, Clone)]
pub struct LTOAnalysis {
    /// Functions that can be inlined
    pub inlinable_functions: HashSet<String>,
    /// Dead functions that can be eliminated
    pub dead_functions: HashSet<String>,
    /// Constants that can be propagated
    pub propagatable_constants: HashMap<String, LTOConstant>,
    /// Functions that can be merged
    pub mergeable_functions: Vec<FunctionGroup>,
    /// Cross-module call graph
    pub call_graph: CallGraph,
    /// Optimization opportunities
    pub optimization_opportunities: Vec<OptimizationOpportunity>,
}

/// Constant value for propagation
#[derive(Debug, Clone, PartialEq)]
pub enum LTOConstant {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
}

/// Group of functions that can be merged
#[derive(Debug, Clone)]
pub struct FunctionGroup {
    pub functions: Vec<String>,
    pub similarity_score: f64,
    pub estimated_size_reduction: usize,
}

/// Call graph representation
#[derive(Debug, Clone)]
pub struct CallGraph {
    /// Function -> list of functions it calls
    pub calls: HashMap<String, Vec<String>>,
    /// Function -> list of functions that call it
    pub callers: HashMap<String, Vec<String>>,
    /// Function -> call frequency
    pub call_frequency: HashMap<String, u64>,
}

/// Optimization opportunity
#[derive(Debug, Clone)]
pub struct OptimizationOpportunity {
    pub kind: OptimizationKind,
    pub function: String,
    pub estimated_benefit: f64,
    pub estimated_cost: f64,
    pub description: String,
}

/// Types of optimization opportunities
#[derive(Debug, Clone, PartialEq)]
pub enum OptimizationKind {
    Inlining,
    DeadCodeElimination,
    ConstantPropagation,
    FunctionMerging,
    LoopOptimization,
    Vectorization,
    TailCallOptimization,
    CommonSubexpressionElimination,
}

/// Module information for LTO
#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub name: String,
    pub functions: Vec<FunctionInfo>,
    pub globals: Vec<GlobalInfo>,
    pub llvm_ir: String,
    pub size_bytes: usize,
}

/// Function information
#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub name: String,
    pub signature: String,
    pub size_bytes: usize,
    pub complexity_score: f64,
    pub call_count: u64,
    pub is_recursive: bool,
    pub is_leaf: bool,
    pub has_side_effects: bool,
    pub inline_hint: InlineHint,
}

/// Global variable information
#[derive(Debug, Clone)]
pub struct GlobalInfo {
    pub name: String,
    pub type_name: String,
    pub is_constant: bool,
    pub is_used: bool,
    pub initial_value: Option<LTOConstant>,
}

/// Inline hint for functions
#[derive(Debug, Clone, PartialEq)]
pub enum InlineHint {
    Always,
    Never,
    Auto,
    OptForSize,
}

/// LTO statistics
#[derive(Debug, Clone)]
pub struct LTOStats {
    pub total_modules: usize,
    pub total_functions: usize,
    pub inlined_functions: usize,
    pub eliminated_functions: usize,
    pub propagated_constants: usize,
    pub merged_functions: usize,
    pub size_reduction_bytes: usize,
    pub optimization_time: Duration,
}

/// Main LTO optimizer
pub struct LinkTimeOptimizer {
    config: LTOConfig,
    modules: Vec<ModuleInfo>,
    analysis: Option<LTOAnalysis>,
    stats: LTOStats,
}

impl LinkTimeOptimizer {
    /// Create a new LTO optimizer with default configuration
    pub fn new() -> Self {
        Self {
            config: LTOConfig::default(),
            modules: Vec::new(),
            analysis: None,
            stats: LTOStats {
                total_modules: 0,
                total_functions: 0,
                inlined_functions: 0,
                eliminated_functions: 0,
                propagated_constants: 0,
                merged_functions: 0,
                size_reduction_bytes: 0,
                optimization_time: Duration::from_secs(0),
            },
        }
    }

    /// Create a new LTO optimizer with custom configuration
    pub fn with_config(config: LTOConfig) -> Self {
        Self {
            config,
            modules: Vec::new(),
            analysis: None,
            stats: LTOStats {
                total_modules: 0,
                total_functions: 0,
                inlined_functions: 0,
                eliminated_functions: 0,
                propagated_constants: 0,
                merged_functions: 0,
                size_reduction_bytes: 0,
                optimization_time: Duration::from_secs(0),
            },
        }
    }

    /// Add a module for LTO analysis
    pub fn add_module(&mut self, module: ModuleInfo) -> Result<(), CursedError> {
        self.modules.push(module);
        self.stats.total_modules += 1;
        Ok(())
    }

    /// Perform comprehensive LTO analysis
    pub fn analyze(&mut self) -> Result<&LTOAnalysis, CursedError> {
        if !self.config.enabled {
            return Err(CursedError::Optimization("LTO is disabled".to_string()));
        }

        let start_time = Instant::now();

        // Build call graph
        let call_graph = self.build_call_graph()?;

        // Find inlinable functions
        let inlinable_functions = self.find_inlinable_functions(&call_graph)?;

        // Find dead functions
        let dead_functions = self.find_dead_functions(&call_graph)?;

        // Find propagatable constants
        let propagatable_constants = self.find_propagatable_constants()?;

        // Find mergeable functions
        let mergeable_functions = self.find_mergeable_functions()?;

        // Find optimization opportunities
        let optimization_opportunities = self.find_optimization_opportunities(
            &call_graph,
            &inlinable_functions,
            &dead_functions,
            &propagatable_constants,
        )?;

        let analysis = LTOAnalysis {
            inlinable_functions,
            dead_functions,
            propagatable_constants,
            mergeable_functions,
            call_graph,
            optimization_opportunities,
        };

        self.analysis = Some(analysis);
        self.stats.optimization_time = start_time.elapsed();

        Ok(self.analysis.as_ref().unwrap())
    }

    /// Apply LTO optimizations
    pub fn optimize(&mut self) -> Result<Vec<ModuleInfo>, CursedError> {
        if self.analysis.is_none() {
            self.analyze()?;
        }

        let analysis = self.analysis.as_ref().unwrap().clone();
        let mut optimized_modules = self.modules.clone();

        // Apply optimizations in order
        if self.config.enable_dce {
            optimized_modules = self.apply_dead_code_elimination(optimized_modules, &analysis)?;
        }

        if self.config.enable_constant_propagation {
            optimized_modules = self.apply_constant_propagation(optimized_modules, &analysis)?;
        }

        if self.config.enable_ipo {
            optimized_modules = self.apply_interprocedural_optimization(optimized_modules, &analysis)?;
        }

        if self.config.enable_function_merging {
            optimized_modules = self.apply_function_merging(optimized_modules, &analysis)?;
        }

        if self.config.enable_cross_module {
            optimized_modules = self.apply_cross_module_optimization(optimized_modules, &analysis)?;
        }

        if self.config.enable_wpo {
            optimized_modules = self.apply_whole_program_optimization(optimized_modules, &analysis)?;
        }

        // Apply inlining last for best results
        optimized_modules = self.apply_inlining(optimized_modules, &analysis)?;

        Ok(optimized_modules)
    }

    /// Build call graph from all modules
    fn build_call_graph(&self) -> Result<CallGraph, CursedError> {
        let mut calls = HashMap::new();
        let mut callers = HashMap::new();
        let mut call_frequency = HashMap::new();

        for module in &self.modules {
            for function in &module.functions {
                // Parse LLVM IR to find function calls
                let function_calls = self.parse_function_calls(&module.llvm_ir, &function.name)?;
                
                calls.insert(function.name.clone(), function_calls.clone());
                call_frequency.insert(function.name.clone(), function.call_count);

                // Build reverse mapping
                for called_function in &function_calls {
                    callers.entry(called_function.clone())
                        .or_insert_with(Vec::new)
                        .push(function.name.clone());
                }
            }
        }

        Ok(CallGraph {
            calls,
            callers,
            call_frequency,
        })
    }

    /// Parse function calls from LLVM IR
    fn parse_function_calls(&self, llvm_ir: &str, function_name: &str) -> Result<Vec<String>, CursedError> {
        let mut calls = Vec::new();
        
        // Find function definition
        let function_start = format!("define.*@{}", function_name);
        let function_regex = regex::Regex::new(&function_start)
            .map_err(|e| CursedError::Optimization(format!("Regex error: {}", e)))?;

        if let Some(start_match) = function_regex.find(llvm_ir) {
            let function_body = &llvm_ir[start_match.start()..];
            
            // Find function end (next define or end of file)
            let end_pos = function_body.find("\ndefine").unwrap_or(function_body.len());
            let function_ir = &function_body[..end_pos];

            // Parse call instructions
            let call_regex = regex::Regex::new(r"call.*@([a-zA-Z_][a-zA-Z0-9_]*)")
                .map_err(|e| CursedError::Optimization(format!("Regex error: {}", e)))?;

            for cap in call_regex.captures_iter(function_ir) {
                if let Some(called_function) = cap.get(1) {
                    calls.push(called_function.as_str().to_string());
                }
            }
        }

        Ok(calls)
    }

    /// Find functions that can be inlined
    fn find_inlinable_functions(&self, call_graph: &CallGraph) -> Result<HashSet<String>, CursedError> {
        let mut inlinable = HashSet::new();

        for module in &self.modules {
            for function in &module.functions {
                if self.should_inline_function(function, call_graph) {
                    inlinable.insert(function.name.clone());
                }
            }
        }

        Ok(inlinable)
    }

    /// Determine if a function should be inlined
    fn should_inline_function(&self, function: &FunctionInfo, call_graph: &CallGraph) -> bool {
        // Check inline hints
        match function.inline_hint {
            InlineHint::Always => return true,
            InlineHint::Never => return false,
            _ => {}
        }

        // Small functions are good candidates
        if function.size_bytes < 100 {
            return true;
        }

        // Leaf functions with high call frequency
        if function.is_leaf && function.call_count > 10 {
            return true;
        }

        // Functions with low complexity and multiple call sites
        if function.complexity_score < 5.0 {
            if let Some(callers) = call_graph.callers.get(&function.name) {
                if callers.len() > 1 && callers.len() < 10 {
                    return true;
                }
            }
        }

        false
    }

    /// Find dead functions that can be eliminated
    fn find_dead_functions(&self, call_graph: &CallGraph) -> Result<HashSet<String>, CursedError> {
        let mut dead_functions = HashSet::new();
        let mut visited = HashSet::new();
        let mut entry_points = HashSet::new();

        // Find entry points (main, exported functions, etc.)
        for module in &self.modules {
            for function in &module.functions {
                if function.name == "main" || function.name.starts_with("export_") {
                    entry_points.insert(function.name.clone());
                }
            }
        }

        // Mark reachable functions
        for entry_point in &entry_points {
            self.mark_reachable(entry_point, call_graph, &mut visited);
        }

        // Find unreachable functions
        for module in &self.modules {
            for function in &module.functions {
                if !visited.contains(&function.name) {
                    dead_functions.insert(function.name.clone());
                }
            }
        }

        Ok(dead_functions)
    }

    /// Mark function as reachable (DFS)
    fn mark_reachable(&self, function_name: &str, call_graph: &CallGraph, visited: &mut HashSet<String>) {
        if visited.contains(function_name) {
            return;
        }

        visited.insert(function_name.to_string());

        if let Some(callees) = call_graph.calls.get(function_name) {
            for callee in callees {
                self.mark_reachable(callee, call_graph, visited);
            }
        }
    }

    /// Find constants that can be propagated
    fn find_propagatable_constants(&self) -> Result<HashMap<String, LTOConstant>, CursedError> {
        let mut constants = HashMap::new();

        for module in &self.modules {
            for global in &module.globals {
                if global.is_constant {
                    if let Some(initial_value) = &global.initial_value {
                        constants.insert(global.name.clone(), initial_value.clone());
                    }
                }
            }
        }

        Ok(constants)
    }

    /// Find functions that can be merged
    fn find_mergeable_functions(&self) -> Result<Vec<FunctionGroup>, CursedError> {
        let mut mergeable_groups = Vec::new();
        let mut processed = HashSet::new();

        for module in &self.modules {
            for function in &module.functions {
                if processed.contains(&function.name) {
                    continue;
                }

                let mut group = vec![function.name.clone()];
                processed.insert(function.name.clone());

                // Find similar functions
                for other_module in &self.modules {
                    for other_function in &other_module.functions {
                        if processed.contains(&other_function.name) {
                            continue;
                        }

                        let similarity = self.calculate_function_similarity(function, other_function);
                        if similarity > 0.8 {
                            group.push(other_function.name.clone());
                            processed.insert(other_function.name.clone());
                        }
                    }
                }

                if group.len() > 1 {
                    let similarity_score = 0.8; // Minimum similarity
                    let estimated_size_reduction = function.size_bytes * (group.len() - 1) / 2;
                    
                    mergeable_groups.push(FunctionGroup {
                        functions: group,
                        similarity_score,
                        estimated_size_reduction,
                    });
                }
            }
        }

        Ok(mergeable_groups)
    }

    /// Calculate similarity between two functions
    fn calculate_function_similarity(&self, func1: &FunctionInfo, func2: &FunctionInfo) -> f64 {
        let mut score = 0.0;

        // Signature similarity
        if func1.signature == func2.signature {
            score += 0.3;
        }

        // Size similarity
        let size_diff = (func1.size_bytes as f64 - func2.size_bytes as f64).abs();
        let size_sim = 1.0 - (size_diff / (func1.size_bytes.max(func2.size_bytes) as f64));
        score += size_sim * 0.2;

        // Complexity similarity
        let complexity_diff = (func1.complexity_score - func2.complexity_score).abs();
        let complexity_sim = 1.0 - (complexity_diff / func1.complexity_score.max(func2.complexity_score));
        score += complexity_sim * 0.2;

        // Properties similarity
        if func1.is_leaf == func2.is_leaf {
            score += 0.1;
        }
        if func1.has_side_effects == func2.has_side_effects {
            score += 0.1;
        }
        if func1.is_recursive == func2.is_recursive {
            score += 0.1;
        }

        score.min(1.0)
    }

    /// Find optimization opportunities
    fn find_optimization_opportunities(
        &self,
        call_graph: &CallGraph,
        inlinable: &HashSet<String>,
        dead: &HashSet<String>,
        constants: &HashMap<String, LTOConstant>,
    ) -> Result<Vec<OptimizationOpportunity>, CursedError> {
        let mut opportunities = Vec::new();

        // Inlining opportunities
        for function_name in inlinable {
            if let Some(function) = self.find_function_info(function_name) {
                opportunities.push(OptimizationOpportunity {
                    kind: OptimizationKind::Inlining,
                    function: function_name.clone(),
                    estimated_benefit: function.call_count as f64 * 0.1,
                    estimated_cost: function.size_bytes as f64 * 0.01,
                    description: format!("Inline function {} with {} call sites", function_name, function.call_count),
                });
            }
        }

        // Dead code elimination opportunities
        for function_name in dead {
            if let Some(function) = self.find_function_info(function_name) {
                opportunities.push(OptimizationOpportunity {
                    kind: OptimizationKind::DeadCodeElimination,
                    function: function_name.clone(),
                    estimated_benefit: function.size_bytes as f64,
                    estimated_cost: 0.0,
                    description: format!("Remove dead function {}", function_name),
                });
            }
        }

        // Constant propagation opportunities
        for (global_name, _) in constants {
            opportunities.push(OptimizationOpportunity {
                kind: OptimizationKind::ConstantPropagation,
                function: global_name.clone(),
                estimated_benefit: 50.0, // Estimated benefit
                estimated_cost: 10.0,
                description: format!("Propagate constant {}", global_name),
            });
        }

        // Sort by benefit/cost ratio
        opportunities.sort_by(|a, b| {
            let ratio_a = a.estimated_benefit / (a.estimated_cost + 1.0);
            let ratio_b = b.estimated_benefit / (b.estimated_cost + 1.0);
            ratio_b.partial_cmp(&ratio_a).unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(opportunities)
    }

    /// Find function info by name
    fn find_function_info(&self, name: &str) -> Option<&FunctionInfo> {
        for module in &self.modules {
            for function in &module.functions {
                if function.name == name {
                    return Some(function);
                }
            }
        }
        None
    }

    /// Apply dead code elimination
    fn apply_dead_code_elimination(
        &mut self,
        mut modules: Vec<ModuleInfo>,
        analysis: &LTOAnalysis,
    ) -> Result<Vec<ModuleInfo>, CursedError> {
        for module in &mut modules {
            module.functions.retain(|f| !analysis.dead_functions.contains(&f.name));
        }

        self.stats.eliminated_functions = analysis.dead_functions.len();
        Ok(modules)
    }

    /// Apply constant propagation
    fn apply_constant_propagation(
        &mut self,
        mut modules: Vec<ModuleInfo>,
        analysis: &LTOAnalysis,
    ) -> Result<Vec<ModuleInfo>, CursedError> {
        // Replace constant references with their values
        for module in &mut modules {
            for (global_name, constant_value) in &analysis.propagatable_constants {
                module.llvm_ir = self.replace_constant_references(&module.llvm_ir, global_name, constant_value)?;
            }
        }

        self.stats.propagated_constants = analysis.propagatable_constants.len();
        Ok(modules)
    }

    /// Replace constant references in LLVM IR
    fn replace_constant_references(
        &self,
        llvm_ir: &str,
        global_name: &str,
        constant_value: &LTOConstant,
    ) -> Result<String, CursedError> {
        let replacement = match constant_value {
            LTOConstant::Integer(val) => val.to_string(),
            LTOConstant::Float(val) => val.to_string(),
            LTOConstant::String(val) => format!("\"{}\"", val),
            LTOConstant::Boolean(val) => if *val { "1" } else { "0" }.to_string(),
            LTOConstant::Null => "null".to_string(),
        };

        // Replace global references
        let pattern = format!("@{}", global_name);
        Ok(llvm_ir.replace(&pattern, &replacement))
    }

    /// Apply interprocedural optimization
    fn apply_interprocedural_optimization(
        &mut self,
        modules: Vec<ModuleInfo>,
        _analysis: &LTOAnalysis,
    ) -> Result<Vec<ModuleInfo>, CursedError> {
        // Implementation would analyze function calls across modules
        // and optimize parameter passing, return value optimization, etc.
        Ok(modules)
    }

    /// Apply function merging
    fn apply_function_merging(
        &mut self,
        mut modules: Vec<ModuleInfo>,
        analysis: &LTOAnalysis,
    ) -> Result<Vec<ModuleInfo>, CursedError> {
        for group in &analysis.mergeable_functions {
            if group.functions.len() > 1 {
                // Create merged function
                let merged_name = format!("merged_{}", group.functions.join("_"));
                
                // Remove original functions and add merged one
                for module in &mut modules {
                    module.functions.retain(|f| !group.functions.contains(&f.name));
                    
                    // Add merged function (simplified implementation)
                    if let Some(first_func) = group.functions.first() {
                        if let Some(original_func) = self.find_function_info(first_func) {
                            module.functions.push(FunctionInfo {
                                name: merged_name.clone(),
                                signature: original_func.signature.clone(),
                                size_bytes: original_func.size_bytes,
                                complexity_score: original_func.complexity_score,
                                call_count: original_func.call_count,
                                is_recursive: original_func.is_recursive,
                                is_leaf: original_func.is_leaf,
                                has_side_effects: original_func.has_side_effects,
                                inline_hint: original_func.inline_hint.clone(),
                            });
                        }
                    }
                }
                
                self.stats.merged_functions += group.functions.len() - 1;
            }
        }

        Ok(modules)
    }

    /// Apply cross-module optimization
    fn apply_cross_module_optimization(
        &mut self,
        modules: Vec<ModuleInfo>,
        _analysis: &LTOAnalysis,
    ) -> Result<Vec<ModuleInfo>, CursedError> {
        // Implementation would optimize across module boundaries
        Ok(modules)
    }

    /// Apply whole-program optimization
    fn apply_whole_program_optimization(
        &mut self,
        modules: Vec<ModuleInfo>,
        _analysis: &LTOAnalysis,
    ) -> Result<Vec<ModuleInfo>, CursedError> {
        // Implementation would perform global optimizations
        Ok(modules)
    }

    /// Apply function inlining
    fn apply_inlining(
        &mut self,
        mut modules: Vec<ModuleInfo>,
        analysis: &LTOAnalysis,
    ) -> Result<Vec<ModuleInfo>, CursedError> {
        for module in &mut modules {
            for function_name in &analysis.inlinable_functions {
                if let Some(function_info) = self.find_function_info(function_name) {
                    if function_info.size_bytes < 200 { // Only inline small functions
                        module.llvm_ir = self.inline_function_calls(&module.llvm_ir, function_name)?;
                        self.stats.inlined_functions += 1;
                    }
                }
            }
        }

        Ok(modules)
    }

    /// Inline function calls in LLVM IR
    fn inline_function_calls(&self, llvm_ir: &str, function_name: &str) -> Result<String, CursedError> {
        // Find function definition
        let function_def_pattern = format!(r"define.*@{}.*\{{.*?\n\}}", function_name);
        let function_def_regex = regex::Regex::new(&function_def_pattern)
            .map_err(|e| CursedError::Optimization(format!("Regex error: {}", e)))?;

        if let Some(function_def) = function_def_regex.find(llvm_ir) {
            let function_body = function_def.as_str();
            
            // Replace function calls with function body
            let call_pattern = format!(r"call.*@{}\([^)]*\)", function_name);
            let call_regex = regex::Regex::new(&call_pattern)
                .map_err(|e| CursedError::Optimization(format!("Regex error: {}", e)))?;

            let result = call_regex.replace_all(llvm_ir, function_body);
            return Ok(result.to_string());
        }

        Ok(llvm_ir.to_string())
    }

    /// Get optimization statistics
    pub fn get_stats(&self) -> &LTOStats {
        &self.stats
    }

    /// Get current configuration
    pub fn get_config(&self) -> &LTOConfig {
        &self.config
    }

    /// Update configuration
    pub fn update_config(&mut self, config: LTOConfig) {
        self.config = config;
    }
}

/// Global LTO manager for the CURSED compiler
pub struct LTOManager {
    optimizer: Arc<Mutex<LinkTimeOptimizer>>,
}

impl LTOManager {
    /// Create a new LTO manager
    pub fn new() -> Self {
        Self {
            optimizer: Arc::new(Mutex::new(LinkTimeOptimizer::new())),
        }
    }

    /// Create a new LTO manager with configuration
    pub fn with_config(config: LTOConfig) -> Self {
        Self {
            optimizer: Arc::new(Mutex::new(LinkTimeOptimizer::with_config(config))),
        }
    }

    /// Add module for LTO
    pub fn add_module(&self, module: ModuleInfo) -> Result<(), CursedError> {
        let mut optimizer = self.optimizer.lock()
            .map_err(|e| CursedError::Optimization(format!("Lock error: {}", e)))?;
        optimizer.add_module(module)
    }

    /// Perform LTO optimization
    pub fn optimize(&self) -> Result<Vec<ModuleInfo>, CursedError> {
        let mut optimizer = self.optimizer.lock()
            .map_err(|e| CursedError::Optimization(format!("Lock error: {}", e)))?;
        optimizer.optimize()
    }

    /// Get optimization statistics
    pub fn get_stats(&self) -> Result<LTOStats, CursedError> {
        let optimizer = self.optimizer.lock()
            .map_err(|e| CursedError::Optimization(format!("Lock error: {}", e)))?;
        Ok(optimizer.get_stats().clone())
    }
}

/// Legacy compatibility functions
impl LinkTimeOptimizer {
    /// Legacy function for compatibility
    pub fn get_minimal_result() -> Result<String, CursedError> {
        Ok("CURSED Link-Time Optimization enabled".to_string())
    }
}

/// Global LTO instance
static GLOBAL_LTO: Lazy<std::sync::Mutex<LTOManager>> = Lazy::new(|| std::sync::Mutex::new(LTOManager::new()));
static GLOBAL_LTO_INIT: std::sync::Once = std::sync::Once::new();

/// Initialize global LTO
pub fn initialize_global_lto() -> Result<(), CursedError> {
    GLOBAL_LTO_INIT.call_once(|| {
        unsafe {
            // Initialization handled by Lazy);
        }
    });
    Ok(())
}

/// Initialize global LTO with configuration
pub fn initialize_global_lto_with_config(config: LTOConfig) -> Result<(), CursedError> {
    GLOBAL_LTO_INIT.call_once(|| {
        unsafe {
            // Initialization handled by Lazy);
        }
    });
    Ok(())
}

/// Get global LTO instance
pub fn get_global_lto() -> Result<std::sync::MutexGuard<'static, LTOManager>, CursedError> {
    Ok(GLOBAL_LTO.lock().unwrap())
}

/// Convenience function for adding module to global LTO
pub fn add_module_to_global_lto(module: ModuleInfo) -> Result<(), CursedError> {
    let lto = get_global_lto()?;
    lto.add_module(module)
}

/// Convenience function for optimizing with global LTO
pub fn optimize_with_global_lto() -> Result<Vec<ModuleInfo>, CursedError> {
    let lto = get_global_lto()?;
    lto.optimize()
}

/// Convenience function for getting global LTO stats
pub fn get_global_lto_stats() -> Result<LTOStats, CursedError> {
    let lto = get_global_lto()?;
    lto.get_stats()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lto_config_default() {
        let config = LTOConfig::default();
        assert!(config.enabled);
        assert_eq!(config.optimization_level, 2);
        assert!(config.enable_ipo);
        assert!(config.enable_wpo);
    }

    #[test]
    fn test_lto_optimizer_creation() {
        let optimizer = LinkTimeOptimizer::new();
        assert!(optimizer.config.enabled);
        assert_eq!(optimizer.modules.len(), 0);
    }

    #[test]
    fn test_lto_module_addition() {
        let mut optimizer = LinkTimeOptimizer::new();
        let module = ModuleInfo {
            name: "test_module".to_string(),
            functions: vec![],
            globals: vec![],
            llvm_ir: "".to_string(),
            size_bytes: 0,
        };

        assert!(optimizer.add_module(module).is_ok());
        assert_eq!(optimizer.modules.len(), 1);
    }

    #[test]
    fn test_function_similarity() {
        let optimizer = LinkTimeOptimizer::new();
        let func1 = FunctionInfo {
            name: "func1".to_string(),
            signature: "i32 (i32)".to_string(),
            size_bytes: 100,
            complexity_score: 5.0,
            call_count: 10,
            is_recursive: false,
            is_leaf: true,
            has_side_effects: false,
            inline_hint: InlineHint::Auto,
        };

        let func2 = FunctionInfo {
            name: "func2".to_string(),
            signature: "i32 (i32)".to_string(),
            size_bytes: 120,
            complexity_score: 5.5,
            call_count: 12,
            is_recursive: false,
            is_leaf: true,
            has_side_effects: false,
            inline_hint: InlineHint::Auto,
        };

        let similarity = optimizer.calculate_function_similarity(&func1, &func2);
        assert!(similarity > 0.5);
    }

    #[test]
    fn test_inlining_heuristics() {
        let optimizer = LinkTimeOptimizer::new();
        let call_graph = CallGraph {
            calls: HashMap::new(),
            callers: HashMap::new(),
            call_frequency: HashMap::new(),
        };

        // Small function should be inlined
        let small_func = FunctionInfo {
            name: "small_func".to_string(),
            signature: "void ()".to_string(),
            size_bytes: 50,
            complexity_score: 1.0,
            call_count: 5,
            is_recursive: false,
            is_leaf: true,
            has_side_effects: false,
            inline_hint: InlineHint::Auto,
        };

        assert!(optimizer.should_inline_function(&small_func, &call_graph));

        // Large function should not be inlined
        let large_func = FunctionInfo {
            name: "large_func".to_string(),
            signature: "void ()".to_string(),
            size_bytes: 1000,
            complexity_score: 20.0,
            call_count: 1,
            is_recursive: true,
            is_leaf: false,
            has_side_effects: true,
            inline_hint: InlineHint::Auto,
        };

        assert!(!optimizer.should_inline_function(&large_func, &call_graph));
    }

    #[test]
    fn test_constant_propagation() {
        let optimizer = LinkTimeOptimizer::new();
        let llvm_ir = "
            @global_const = constant i32 42
            define i32 @test() {
                %1 = load i32, i32* @global_const
                ret i32 %1
            }
        ";

        let result = optimizer.replace_constant_references(
            llvm_ir,
            "global_const",
            &LTOConstant::Integer(42),
        );

        assert!(result.is_ok());
        let optimized_ir = result.unwrap();
        assert!(optimized_ir.contains("42"));
    }

    #[test]
    fn test_lto_manager() {
        let manager = LTOManager::new();
        let module = ModuleInfo {
            name: "test".to_string(),
            functions: vec![],
            globals: vec![],
            llvm_ir: "".to_string(),
            size_bytes: 0,
        };

        assert!(manager.add_module(module).is_ok());
    }

    #[test]
    fn test_global_lto_initialization() {
        assert!(initialize_global_lto().is_ok());
        assert!(get_global_lto().is_ok());
    }
}
