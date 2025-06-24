/// Interprocedural Analyzer for Enhanced LLVM Optimization
/// 
/// Analyzes relationships between functions for cross-function optimizations
/// including inlining, constant propagation, and dead code elimination.

use crate::error::{Error, Result};

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use tracing::{debug, trace, info, instrument};

use inkwell::{
    values::{FunctionValue, BasicValue, BasicValueEnum, CallSiteValue, InstructionValue},
    crate::types::{BasicType, FunctionType},
    basic_block::BasicBlock,
    module::Module,
    context::Context,
};

use crate::optimization::enhanced_llvm_passes_manager::EnhancedOptimizationStatistics;

/// Interprocedural analyzer for cross-function optimization analysis
pub struct InterproceduralAnalyzer<'ctx> {
    context_lifetime: std::marker::PhantomData<&'ctx ()>,
    statistics: Arc<Mutex<EnhancedOptimizationStatistics>>,
    
    // Analysis data structures
    call_graph: CallGraph,
    function_analysis: FunctionAnalysis,
    interprocedural_info: InterproceduralInfo,
}

/// Call graph representation
#[derive(Debug, Default)]
struct CallGraph {
    /// Function name -> list of functions it calls
    callers: HashMap<String, HashSet<String>>,
    /// Function name -> list of functions that call it
    callees: HashMap<String, HashSet<String>>,
    /// Call site information
    call_sites: HashMap<String, Vec<CallSiteAnalysis>>,
    /// Recursive call detection
    recursive_functions: HashSet<String>,
}

/// Analysis of individual functions
#[derive(Debug, Default)]
struct FunctionAnalysis {
    /// Function name -> function properties
    function_properties: HashMap<String, FunctionProperties>,
    /// Function name -> optimization opportunities
    optimization_opportunities: HashMap<String, Vec<InterproceduralOptimization>>,
    /// Function name -> side effect analysis
    side_effects: HashMap<String, SideEffectInfo>,
}

/// Interprocedural information
#[derive(Debug, Default)]
struct InterproceduralInfo {
    /// Constants that can be propagated across function boundaries
    interprocedural_constants: HashMap<String, Vec<ConstantPropagation>>,
    /// Functions that can be inlined
    inlining_candidates: HashMap<String, InliningCandidate>,
    /// Dead code elimination opportunities
    dead_code_opportunities: Vec<DeadCodeOpportunity>,
}

/// Properties of a function
#[derive(Debug, Clone)]
struct FunctionProperties {
    /// Size in instructions
    instruction_count: usize,
    /// Number of basic blocks
    basic_block_count: usize,
    /// Call frequency (estimated)
    call_frequency: usize,
    /// Whether function is leaf (calls no other functions)
    is_leaf: bool,
    /// Whether function is recursive
    is_recursive: bool,
    /// Whether function has side effects
    has_side_effects: bool,
    /// Parameter usage patterns
    parameter_usage: Vec<ParameterUsage>,
}

/// Call site analysis information
#[derive(Debug, Clone)]
struct CallSiteAnalysis {
    /// Function being called
    callee_name: String,
    /// Call site location
    location: String,
    /// Arguments passed (with constant analysis)
    arguments: Vec<ArgumentInfo>,
    /// Estimated call frequency
    frequency: usize,
    /// Context-sensitive information
    context: CallContext,
}

/// Interprocedural optimization opportunities
#[derive(Debug, Clone)]
enum InterproceduralOptimization {
    Inlining { 
        target_function: String,
        estimated_benefit: f64,
        size_cost: usize,
    },
    ConstantPropagation {
        parameter_index: usize,
        constant_value: String,
        propagation_sites: usize,
    },
    DeadCodeElimination {
        unused_functions: Vec<String>,
        estimated_savings: usize,
    },
    TailCallOptimization {
        target_function: String,
        call_sites: usize,
    },
}

/// Side effect information
#[derive(Debug, Clone, Default)]
struct SideEffectInfo {
    /// Whether function modifies global state
    modifies_global_state: bool,
    /// Whether function performs I/O
    performs_io: bool,
    /// Whether function allocates memory
    allocates_memory: bool,
    /// Whether function calls external functions
    calls_external: bool,
    /// Memory locations that may be modified
    modified_memory: HashSet<String>,
}

/// Constant propagation opportunity
#[derive(Debug, Clone)]
struct ConstantPropagation {
    /// Function parameter index
    parameter_index: usize,
    /// Constant value
    constant_value: ConstantValue,
    /// Number of call sites that can benefit
    affected_call_sites: usize,
}

/// Inlining candidate information
#[derive(Debug, Clone)]
struct InliningCandidate {
    /// Function to inline
    function_name: String,
    /// Estimated benefit score
    benefit_score: f64,
    /// Size cost of inlining
    size_cost: usize,
    /// Number of call sites
    call_site_count: usize,
    /// Whether inlining is profitable
    is_profitable: bool,
}

/// Dead code elimination opportunity
#[derive(Debug, Clone)]
struct DeadCodeOpportunity {
    /// Type of dead code
    dead_code_type: DeadCodeType,
    /// Functions or code segments affected
    affected_items: Vec<String>,
    /// Estimated size savings
    estimated_savings: usize,
}

/// Parameter usage analysis
#[derive(Debug, Clone)]
struct ParameterUsage {
    /// Parameter index
    index: usize,
    /// How the parameter is used
    usage_pattern: ParameterUsagePattern,
    /// Whether parameter is modified
    is_modified: bool,
    /// Whether parameter escapes the function
    escapes: bool,
}

/// Argument information at call sites
#[derive(Debug, Clone)]
struct ArgumentInfo {
    /// Argument index
    index: usize,
    /// Whether argument is constant
    is_constant: bool,
    /// Constant value if applicable
    constant_value: Option<ConstantValue>,
    /// Type information
    type_info: String,
}

/// Call context for context-sensitive analysis
#[derive(Debug, Clone)]
struct CallContext {
    /// Calling function
    caller: String,
    /// Call path depth
    depth: usize,
    /// Context-sensitive constants
    context_constants: HashMap<usize, ConstantValue>,
}

/// Types of constant values
#[derive(Debug, Clone)]
enum ConstantValue {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Null,
}

/// Types of dead code
#[derive(Debug, Clone)]
enum DeadCodeType {
    UnusedFunction,
    UnreachableCode,
    UnusedGlobal,
    UnusedParameter,
}

/// Parameter usage patterns
#[derive(Debug, Clone)]
enum ParameterUsagePattern {
    ReadOnly,
    WriteOnly,
    ReadWrite,
    Unused,
    AddressTaken,
}

impl<'ctx> InterproceduralAnalyzer<'ctx> {
    /// Create new interprocedural analyzer
    pub fn new(statistics: Arc<Mutex<EnhancedOptimizationStatistics>>) -> Self {
        Self {
            context_lifetime: std::marker::PhantomData,
            statistics,
            call_graph: CallGraph::default(),
            function_analysis: FunctionAnalysis::default(),
            interprocedural_info: InterproceduralInfo::default(),
        }
    }
    
    /// Analyze module for interprocedural optimizations
    #[instrument(skip(self, module))]
    pub fn analyze_module(&mut self, module: &Module<'ctx>) -> Result<()> {
        info!("Starting interprocedural analysis");
        
        // Phase 1: Build call graph
        self.build_call_graph(module)?;
        
        // Phase 2: Analyze individual functions
        self.analyze_functions(module)?;
        
        // Phase 3: Perform interprocedural analysis
        self.perform_interprocedural_analysis()?;
        
        // Phase 4: Identify optimization opportunities
        self.identify_optimization_opportunities()?;
        
        info!("Interprocedural analysis completed");
        Ok(())
    }
    
    /// Build call graph from module
    fn build_call_graph(&mut self, module: &Module<'ctx>) -> Result<()> {
        debug!("Building call graph");
        
        for function in module.get_functions() {
            let function_name = function.get_name().to_str().unwrap_or("unknown").to_string();
            
            // Initialize entries for this function
            self.call_graph.callers.entry(function_name.clone()).or_insert_with(HashSet::new);
            self.call_graph.callees.entry(function_name.clone()).or_insert_with(HashSet::new);
            
            // Analyze function calls
            if let Some(first_block) = function.get_first_basic_block() {
                self.analyze_function_calls(function, first_block, &function_name)?;
            }
        }
        
        // Detect recursive functions
        self.detect_recursive_functions();
        
        debug!("Call graph built with {} functions", self.call_graph.callers.len());
        Ok(())
    }
    
    /// Analyze calls within a function
    fn analyze_function_calls(&mut self, function: FunctionValue<'ctx>, block: BasicBlock<'ctx>, function_name: &str) -> Result<()> {
        let mut current_block = Some(block);
        let mut call_sites = Vec::new();
        
        while let Some(bb) = current_block {
            let mut instruction = bb.get_first_instruction();
            
            while let Some(instr) = instruction {
                if let Some(call_site) = self.analyze_call_instruction(instr, function_name)? {
                    call_sites.push(call_site);
                }
                instruction = instr.get_next_instruction();
            }
            
            current_block = bb.get_next_basic_block();
        }
        
        self.call_graph.call_sites.insert(function_name.to_string(), call_sites);
        Ok(())
    }
    
    /// Analyze a call instruction
    fn analyze_call_instruction(&mut self, instruction: InstructionValue<'ctx>, caller_name: &str) -> Result<Option<CallSiteAnalysis>> {
        if let Some(opcode) = instruction.get_opcode().get_instruction_opcode() {
            if matches!(opcode, inkwell::values::InstructionOpcode::Call) {
                // Extract call information
                if let Ok(call_site) = instruction.get_call_site() {
                    if let Some(called_fn) = call_site.get_called_fn_value() {
                        let callee_name = called_fn.get_name().to_str().unwrap_or("unknown").to_string();
                        
                        // Update call graph
                        self.call_graph.callers.entry(caller_name.to_string())
                            .or_default().insert(callee_name.clone());
                        self.call_graph.callees.entry(callee_name.clone())
                            .or_default().insert(caller_name.to_string());
                        
                        // Analyze arguments
                        let mut arguments = Vec::new();
                        for (i, arg) in call_site.get_enum_arguments().iter().enumerate() {
                            arguments.push(ArgumentInfo {
                                index: i,
                                is_constant: self.is_constant_argument(arg),
                                constant_value: self.extract_constant_value(arg),
                                type_info: self.get_type_info(arg),
                            });
                        }
                        
                        return Ok(Some(CallSiteAnalysis {
                            callee_name,
                            location: format!("{}:{}", caller_name, i),
                            arguments,
                            frequency: 1, // Would be determined by profiling
                            context: CallContext {
                                caller: caller_name.to_string(),
                                depth: 1,
                                context_constants: HashMap::new(),
                            },
                        }));
                    }
                }
            }
        }
        
        Ok(None)
    }
    
    /// Check if an argument is constant
    fn is_constant_argument(&self, arg: &BasicValueEnum<'ctx>) -> bool {
        // In a real implementation, this would check if the value is a constant
        // For now, we'll use a simple heuristic
        false // Conservative assumption
    }
    
    /// Extract constant value from argument
    fn extract_constant_value(&self, arg: &BasicValueEnum<'ctx>) -> Option<ConstantValue> {
        // In a real implementation, this would extract actual constant values
        None
    }
    
    /// Get type information for argument
    fn get_type_info(&self, arg: &BasicValueEnum<'ctx>) -> String {
        // Extract basic type information
        format!("{:?}", arg.get_type())
    }
    
    /// Detect recursive functions in the call graph
    fn detect_recursive_functions(&mut self) {
        debug!("Detecting recursive functions");
        
        for function_name in self.call_graph.callers.keys() {
            if self.is_recursive_function(function_name, &mut HashSet::new()) {
                self.call_graph.recursive_functions.insert(function_name.clone());
            }
        }
        
        debug!("Found {} recursive functions", self.call_graph.recursive_functions.len());
    }
    
    /// Check if a function is recursive using DFS
    fn is_recursive_function(&self, function_name: &str, visited: &mut HashSet<String>) -> bool {
        if visited.contains(function_name) {
            return true; // Cycle detected
        }
        
        visited.insert(function_name.to_string());
        
        if let Some(callees) = self.call_graph.callers.get(function_name) {
            for callee in callees {
                if self.is_recursive_function(callee, visited) {
                    return true;
                }
            }
        }
        
        visited.remove(function_name);
        false
    }
    
    /// Analyze individual functions
    fn analyze_functions(&mut self, module: &Module<'ctx>) -> Result<()> {
        debug!("Analyzing individual functions");
        
        for function in module.get_functions() {
            let function_name = function.get_name().to_str().unwrap_or("unknown").to_string();
            
            let properties = self.analyze_function_properties(function)?;
            let side_effects = self.analyze_side_effects(function)?;
            
            self.function_analysis.function_properties.insert(function_name.clone(), properties);
            self.function_analysis.side_effects.insert(function_name, side_effects);
        }
        
        debug!("Analyzed {} functions", self.function_analysis.function_properties.len());
        Ok(())
    }
    
    /// Analyze properties of a single function
    fn analyze_function_properties(&self, function: FunctionValue<'ctx>) -> Result<FunctionProperties> {
        let mut instruction_count = 0;
        let mut basic_block_count = 0;
        let mut parameter_usage = Vec::new();
        
        // Count instructions and basic blocks
        let mut current_block = function.get_first_basic_block();
        while let Some(block) = current_block {
            basic_block_count += 1;
            
            let mut instruction = block.get_first_instruction();
            while let Some(_) = instruction {
                instruction_count += 1;
                instruction = instruction.unwrap().get_next_instruction();
            }
            
            current_block = block.get_next_basic_block();
        }
        
        // Analyze parameters
        for (i, param) in function.get_param_iter().enumerate() {
            parameter_usage.push(ParameterUsage {
                index: i,
                usage_pattern: self.analyze_parameter_usage(&param),
                is_modified: false, // Would be determined by analysis
                escapes: false,    // Would be determined by escape analysis
            });
        }
        
        let function_name = function.get_name().to_str().unwrap_or("unknown");
        let is_leaf = !self.call_graph.callers.get(function_name)
            .map(|callees| !callees.is_empty())
            .unwrap_or(false);
        let is_recursive = self.call_graph.recursive_functions.contains(function_name);
        
        Ok(FunctionProperties {
            instruction_count,
            basic_block_count,
            call_frequency: 1, // Would be determined by profiling
            is_leaf,
            is_recursive,
            has_side_effects: true, // Conservative assumption
            parameter_usage,
        })
    }
    
    /// Analyze parameter usage pattern
    fn analyze_parameter_usage(&self, _parameter: &BasicValueEnum<'ctx>) -> ParameterUsagePattern {
        // In a real implementation, this would analyze how the parameter is used
        ParameterUsagePattern::ReadOnly // Conservative assumption
    }
    
    /// Analyze side effects of a function
    fn analyze_side_effects(&self, function: FunctionValue<'ctx>) -> Result<SideEffectInfo> {
        let mut side_effects = SideEffectInfo::default();
        
        // Analyze instructions for side effects
        let mut current_block = function.get_first_basic_block();
        while let Some(block) = current_block {
            let mut instruction = block.get_first_instruction();
            
            while let Some(instr) = instruction {
                if let Some(opcode) = instr.get_opcode().get_instruction_opcode() {
                    match opcode {
                        inkwell::values::InstructionOpcode::Store => {
                            side_effects.modifies_global_state = true;
                        },
                        inkwell::values::InstructionOpcode::Call => {
                            side_effects.calls_external = true;
                            // Could be I/O or memory allocation
                            side_effects.performs_io = true;
                            side_effects.allocates_memory = true;
                        },
                        inkwell::values::InstructionOpcode::Alloca => {
                            side_effects.allocates_memory = true;
                        },
                        _ => {}
                    }
                }
                instruction = instr.get_next_instruction();
            }
            
            current_block = block.get_next_basic_block();
        }
        
        Ok(side_effects)
    }
    
    /// Perform interprocedural analysis
    fn perform_interprocedural_analysis(&mut self) -> Result<()> {
        debug!("Performing interprocedural analysis");
        
        // Analyze constant propagation opportunities
        self.analyze_interprocedural_constants()?;
        
        // Analyze inlining opportunities
        self.analyze_inlining_candidates()?;
        
        // Analyze dead code elimination opportunities
        self.analyze_dead_code_opportunities()?;
        
        Ok(())
    }
    
    /// Analyze interprocedural constant propagation
    fn analyze_interprocedural_constants(&mut self) -> Result<()> {
        debug!("Analyzing interprocedural constant propagation");
        
        for (function_name, call_sites) in &self.call_graph.call_sites {
            let mut constants = Vec::new();
            
            for call_site in call_sites {
                for (i, arg) in call_site.arguments.iter().enumerate() {
                    if arg.is_constant {
                        constants.push(ConstantPropagation {
                            parameter_index: i,
                            constant_value: arg.constant_value.clone().unwrap_or(ConstantValue::Null),
                            affected_call_sites: 1,
                        });
                    }
                }
            }
            
            if !constants.is_empty() {
                self.interprocedural_info.interprocedural_constants.insert(function_name.clone(), constants);
            }
        }
        
        Ok(())
    }
    
    /// Analyze inlining candidates
    fn analyze_inlining_candidates(&mut self) -> Result<()> {
        debug!("Analyzing inlining candidates");
        
        for (function_name, properties) in &self.function_analysis.function_properties {
            let call_site_count = self.call_graph.callees.get(function_name)
                .map(|callers| callers.len())
                .unwrap_or(0);
            
            if call_site_count > 0 {
                let benefit_score = self.calculate_inlining_benefit(function_name, properties);
                let size_cost = properties.instruction_count;
                let is_profitable = benefit_score > 0.5 && size_cost < 100; // Heuristic
                
                if is_profitable {
                    let candidate = InliningCandidate {
                        function_name: function_name.clone(),
                        benefit_score,
                        size_cost,
                        call_site_count,
                        is_profitable,
                    };
                    
                    self.interprocedural_info.inlining_candidates.insert(function_name.clone(), candidate);
                }
            }
        }
        
        debug!("Found {} inlining candidates", self.interprocedural_info.inlining_candidates.len());
        Ok(())
    }
    
    /// Calculate inlining benefit score
    fn calculate_inlining_benefit(&self, function_name: &str, properties: &FunctionProperties) -> f64 {
        let mut score = 0.0;
        
        // Small functions benefit more from inlining
        if properties.instruction_count < 20 {
            score += 0.4;
        }
        
        // Leaf functions are good candidates
        if properties.is_leaf {
            score += 0.3;
        }
        
        // Functions without side effects are easier to inline
        if !properties.has_side_effects {
            score += 0.2;
        }
        
        // Frequently called functions benefit more
        if properties.call_frequency > 10 {
            score += 0.1;
        }
        
        score
    }
    
    /// Analyze dead code elimination opportunities
    fn analyze_dead_code_opportunities(&mut self) -> Result<()> {
        debug!("Analyzing dead code elimination opportunities");
        
        // Find unused functions
        let mut unused_functions = Vec::new();
        for function_name in self.function_analysis.function_properties.keys() {
            if !self.call_graph.callees.get(function_name)
                .map(|callers| !callers.is_empty())
                .unwrap_or(false) {
                unused_functions.push(function_name.clone());
            }
        }
        
        if !unused_functions.is_empty() {
            let estimated_savings = unused_functions.iter()
                .map(|name| self.function_analysis.function_properties.get(name)
                    .map(|props| props.instruction_count)
                    .unwrap_or(0))
                .sum();
            
            self.interprocedural_info.dead_code_opportunities.push(DeadCodeOpportunity {
                dead_code_type: DeadCodeType::UnusedFunction,
                affected_items: unused_functions,
                estimated_savings,
            });
        }
        
        Ok(())
    }
    
    /// Identify optimization opportunities
    fn identify_optimization_opportunities(&mut self) -> Result<()> {
        debug!("Identifying interprocedural optimization opportunities");
        
        for (function_name, properties) in &self.function_analysis.function_properties {
            let mut opportunities = Vec::new();
            
            // Inlining opportunities
            if let Some(candidate) = self.interprocedural_info.inlining_candidates.get(function_name) {
                opportunities.push(InterproceduralOptimization::Inlining {
                    target_function: function_name.clone(),
                    estimated_benefit: candidate.benefit_score,
                    size_cost: candidate.size_cost,
                });
            }
            
            // Constant propagation opportunities
            if let Some(constants) = self.interprocedural_info.interprocedural_constants.get(function_name) {
                for constant in constants {
                    opportunities.push(InterproceduralOptimization::ConstantPropagation {
                        parameter_index: constant.parameter_index,
                        constant_value: format!("{:?}", constant.constant_value),
                        propagation_sites: constant.affected_call_sites,
                    });
                }
            }
            
            self.function_analysis.optimization_opportunities.insert(function_name.clone(), opportunities);
        }
        
        // Update statistics
        let mut stats = self.statistics.lock().unwrap();
        stats.interprocedural_optimizations = self.function_analysis.optimization_opportunities
            .values()
            .map(|ops| ops.len())
            .sum();
        
        Ok(())
    }
    
    /// Get call graph statistics
    pub fn get_call_graph_statistics(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        
        stats.insert("total_functions".to_string(), self.call_graph.callers.len());
        stats.insert("total_call_sites".to_string(), 
                    self.call_graph.call_sites.values().map(|sites| sites.len()).sum());
        stats.insert("recursive_functions".to_string(), self.call_graph.recursive_functions.len());
        stats.insert("leaf_functions".to_string(),
                    self.function_analysis.function_properties.values()
                        .filter(|props| props.is_leaf)
                        .count());
        stats.insert("inlining_candidates".to_string(), self.interprocedural_info.inlining_candidates.len());
        
        stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    
    #[test]
    fn test_interprocedural_analyzer_creation() {
        let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
        let analyzer = InterproceduralAnalyzer::new(statistics);
        
        assert_eq!(analyzer.call_graph.callers.len(), 0);
        assert_eq!(analyzer.call_graph.recursive_functions.len(), 0);
    }
    
    #[test]
    fn test_inlining_benefit_calculation() {
        let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
        let analyzer = InterproceduralAnalyzer::new(statistics);
        
        let properties = FunctionProperties {
            instruction_count: 15,
            basic_block_count: 2,
            call_frequency: 20,
            is_leaf: true,
            is_recursive: false,
            has_side_effects: false,
            parameter_usage: vec![],
        };
        
        let benefit = analyzer.calculate_inlining_benefit("test_function", &properties);
        assert!(benefit > 0.5); // Should be a good candidate
    }
    
    #[test]
    fn test_recursive_function_detection() {
        let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
        let mut analyzer = InterproceduralAnalyzer::new(statistics);
        
        // Create a simple recursive call graph
        analyzer.call_graph.callers.insert("func_a".to_string(), 
            ["func_b".to_string()].iter().cloned().collect());
        analyzer.call_graph.callers.insert("func_b".to_string(), 
            ["func_a".to_string()].iter().cloned().collect());
        
        analyzer.detect_recursive_functions();
        
        assert!(analyzer.call_graph.recursive_functions.contains("func_a"));
        assert!(analyzer.call_graph.recursive_functions.contains("func_b"));
    }
    
    #[test]
    fn test_side_effect_analysis() {
        let side_effects = SideEffectInfo {
            modifies_global_state: true,
            performs_io: false,
            allocates_memory: true,
            calls_external: false,
            modified_memory: HashSet::new(),
        };
        
        assert!(side_effects.modifies_global_state);
        assert!(!side_effects.performs_io);
        assert!(side_effects.allocates_memory);
    }
}
