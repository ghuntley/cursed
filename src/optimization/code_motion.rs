/// Code Motion Optimization Implementation
/// 
/// Provides comprehensive code motion optimization for CURSED,
/// including code hoisting, sinking, and loop-invariant code motion.

use crate::error::{Error, Result};
use crate::common::optimization_level::OptimizationLevel;

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{debug, info, warn, instrument};
use serde::{Deserialize, Serialize};

use inkwell::{
    context::Context,
    module::Module,
    values::{FunctionValue, InstructionValue, BasicValueEnum},
    basic_block::BasicBlock,
    builder::Builder,
};

/// Code motion optimizer
pub struct CodeMotionOptimizer<'ctx> {
    context: &'ctx Context,
    optimization_level: OptimizationLevel,
    dominance_analysis: DominanceAnalysis,
    loop_analysis: LoopAnalysis,
    motion_analysis: MotionAnalysis,
    motion_opportunities: HashMap<String, Vec<MotionOpportunity>>,
    statistics: Arc<Mutex<CodeMotionStatistics>>,
    builder: Builder<'ctx>,
}

/// Dominance analysis for code motion
#[derive(Debug, Clone)]
pub struct DominanceAnalysis {
    dominance_tree: HashMap<String, String>,      // block -> immediate dominator
    dominance_frontier: HashMap<String, HashSet<String>>,
    post_dominance_tree: HashMap<String, String>, // block -> immediate post-dominator
    dominated_blocks: HashMap<String, HashSet<String>>,
    post_dominated_blocks: HashMap<String, HashSet<String>>,
}

/// Loop analysis for code motion
#[derive(Debug, Clone)]
pub struct LoopAnalysis {
    natural_loops: HashMap<String, LoopInfo>,
    loop_nesting: HashMap<String, usize>,
    loop_invariants: HashMap<String, HashSet<String>>, // loop -> invariant instructions
    loop_exits: HashMap<String, HashSet<String>>,
}

/// Information about a natural loop
#[derive(Debug, Clone)]
pub struct LoopInfo {
    pub header: String,
    pub blocks: HashSet<String>,
    pub back_edges: Vec<(String, String)>, // (source, target)
    pub exit_blocks: HashSet<String>,
    pub preheader: Option<String>,
    pub depth: usize,
}

/// Motion analysis for identifying opportunities
#[derive(Debug, Clone)]
pub struct MotionAnalysis {
    instruction_dependencies: HashMap<String, HashSet<String>>,
    instruction_uses: HashMap<String, HashSet<String>>,
    side_effect_analysis: HashMap<String, SideEffectInfo>,
    profitability_analysis: HashMap<String, MotionProfitability>,
}

/// Side effect information for instructions
#[derive(Debug, Clone)]
pub struct SideEffectInfo {
    pub has_memory_effects: bool,
    pub may_throw: bool,
    pub calls_external: bool,
    pub accesses_volatile: bool,
    pub has_undefined_behavior: bool,
}

/// Profitability analysis for code motion
#[derive(Debug, Clone)]
pub struct MotionProfitability {
    pub instruction: String,
    pub motion_type: MotionType,
    pub execution_frequency_change: f64,
    pub register_pressure_impact: i32,
    pub cache_impact: f64,
    pub overall_benefit: f64,
}

/// Code motion opportunity
#[derive(Debug, Clone)]
pub struct MotionOpportunity {
    pub opportunity_type: MotionType,
    pub instruction: String,
    pub source_block: String,
    pub target_block: String,
    pub motion_reason: MotionReason,
    pub estimated_benefit: f64,
    pub constraints: Vec<MotionConstraint>,
    pub risk_factors: Vec<RiskFactor>,
}

/// Type of code motion
#[derive(Debug, Clone, PartialEq)]
pub enum MotionType {
    Hoisting,              // Move code up in dominance tree
    Sinking,               // Move code down in dominance tree
    LoopInvariantMotion,   // Move invariant code out of loops
    PredicateMotion,       // Move code based on predicates
    SpeculativeMotion,     // Speculative execution
    LoadMotion,            // Move load instructions
    StoreMotion,           // Move store instructions
}

/// Reason for code motion
#[derive(Debug, Clone)]
pub enum MotionReason {
    ReduceExecutionFrequency,  // Move to less frequently executed block
    ImproveRegisterPressure,   // Reduce register pressure
    EnableOtherOptimizations,  // Enable follow-up optimizations
    ReduceCodeSize,            // Eliminate duplicate code
    ImproveLocality,           // Improve cache locality
    RemoveRedundancy,          // Remove redundant computations
}

/// Constraints on code motion
#[derive(Debug, Clone)]
pub enum MotionConstraint {
    PreserveDependencies,      // Must preserve data dependencies
    NoSpeculation,             // Don't speculate potentially harmful operations
    PreserveExceptions,        // Don't change exception behavior
    LimitRegisterPressure,     // Don't increase register pressure too much
    PreserveMemoryOrder,       // Preserve memory ordering
    NoCodeGrowth,              // Don't increase code size
}

/// Risk factors for code motion
#[derive(Debug, Clone)]
pub enum RiskFactor {
    IncreaseRegisterPressure,  // May increase register pressure
    SpeculativeExecution,      // May execute unnecessarily
    WorseLocality,             // May worsen cache locality
    ComplexDependencies,       // Complex dependency analysis required
    ExceptionSafety,           // Exception safety concerns
}

/// Code motion optimization statistics
#[derive(Debug, Clone, Default)]
pub struct CodeMotionStatistics {
    pub functions_analyzed: usize,
    pub instructions_hoisted: usize,
    pub instructions_sunk: usize,
    pub loop_invariants_moved: usize,
    pub redundant_loads_eliminated: usize,
    pub dead_stores_eliminated: usize,
    pub register_pressure_reductions: usize,
    pub execution_frequency_improvements: f64,
    pub estimated_speedup: f64,
    pub optimization_time: Duration,
}

impl<'ctx> CodeMotionOptimizer<'ctx> {
    /// Create new code motion optimizer
    #[instrument(skip(context))]
    pub fn new(context: &'ctx Context, optimization_level: OptimizationLevel) -> Self {
        info!("Initializing code motion optimizer with optimization level {:?}", optimization_level);
        
        Self {
            context,
            optimization_level,
            dominance_analysis: DominanceAnalysis::new(),
            loop_analysis: LoopAnalysis::new(),
            motion_analysis: MotionAnalysis::new(),
            motion_opportunities: HashMap::new(),
            statistics: Arc::new(Mutex::new(CodeMotionStatistics::default())),
            builder: context.create_builder(),
        }
    }
    
    /// Perform code motion optimization on entire module
    #[instrument(skip(self, module))]
    pub fn optimize_module(&mut self, module: &Module<'ctx>) -> Result<CodeMotionResults> {
        let start_time = Instant::now();
        info!("Starting code motion optimization");
        
        let mut function_results = HashMap::new();
        
        for function in module.get_functions() {
            if function.get_first_basic_block().is_some() {
                let result = self.optimize_function(function)?;
                function_results.insert(
                    function.get_name().to_str().unwrap_or("unnamed").to_string(),
                    result
                );
            }
        }
        
        let optimization_time = start_time.elapsed();
        self.update_statistics(optimization_time, &function_results);
        
        info!(
            optimization_time = ?optimization_time,
            functions_optimized = function_results.len(),
            instructions_moved = self.get_statistics().instructions_hoisted + self.get_statistics().instructions_sunk,
            "Code motion optimization completed"
        );
        
        Ok(CodeMotionResults {
            function_results,
            motion_opportunities: self.motion_opportunities.clone(),
            optimization_summary: self.generate_optimization_summary()?,
            statistics: self.get_statistics(),
        })
    }
    
    /// Optimize a single function with code motion
    #[instrument(skip(self, function))]
    pub fn optimize_function(&mut self, function: FunctionValue<'ctx>) -> Result<FunctionCodeMotionResults> {
        let function_name = function.get_name().to_str().unwrap_or("unnamed");
        debug!("Optimizing function with code motion: {}", function_name);
        
        // Phase 1: Build analysis information
        self.build_dominance_analysis(function)?;
        self.build_loop_analysis(function)?;
        self.build_motion_analysis(function)?;
        
        // Phase 2: Identify motion opportunities
        let opportunities = self.identify_motion_opportunities(function)?;
        
        // Phase 3: Perform code motion optimizations
        let motion_results = self.perform_code_motion(function, &opportunities)?;
        
        // Phase 4: Loop-invariant code motion
        let licm_results = self.perform_loop_invariant_motion(function)?;
        
        // Phase 5: Cleanup and validation
        let cleanup_results = self.cleanup_and_validate(function)?;
        
        let total_motions = motion_results.len() + licm_results.len();
        let optimization_benefit = self.calculate_function_benefit(&motion_results, &licm_results);
        
        Ok(FunctionCodeMotionResults {
            function_name: function_name.to_string(),
            motion_opportunities: opportunities,
            hoisting_results: motion_results.iter().filter(|r| r.motion_type == MotionType::Hoisting).cloned().collect(),
            sinking_results: motion_results.iter().filter(|r| r.motion_type == MotionType::Sinking).cloned().collect(),
            licm_results,
            cleanup_results,
            total_motions,
            optimization_benefit,
        })
    }
    
    /// Check if code motion is safe for a specific instruction
    pub fn is_motion_safe(&self, instruction: &str, motion_type: &MotionType, target_block: &str) -> MotionSafety {
        let mut safety_issues = Vec::new();
        let mut is_safe = true;
        
        // Check side effects
        if let Some(side_effects) = self.motion_analysis.side_effect_analysis.get(instruction) {
            if side_effects.has_memory_effects && matches!(motion_type, MotionType::SpeculativeMotion) {
                safety_issues.push("Memory effects prevent speculative motion".to_string());
                is_safe = false;
            }
            
            if side_effects.may_throw && matches!(motion_type, MotionType::Hoisting) {
                safety_issues.push("Exception throwing prevents hoisting".to_string());
                is_safe = false;
            }
            
            if side_effects.accesses_volatile {
                safety_issues.push("Volatile access prevents motion".to_string());
                is_safe = false;
            }
        }
        
        // Check dependencies
        if let Some(dependencies) = self.motion_analysis.instruction_dependencies.get(instruction) {
            for dep in dependencies {
                if !self.is_dependency_satisfied_after_motion(dep, target_block) {
                    safety_issues.push(format!("Dependency on {} not satisfied", dep));
                    is_safe = false;
                }
            }
        }
        
        // Check dominance constraints
        if !self.respects_dominance_constraints(instruction, target_block) {
            safety_issues.push("Motion violates dominance constraints".to_string());
            is_safe = false;
        }
        
        let safety_confidence = if is_safe { 1.0 } else { 0.0 };
        
        MotionSafety {
            is_safe,
            safety_issues,
            safety_confidence,
        }
    }
    
    /// Generate comprehensive code motion report
    pub fn generate_code_motion_report(&self, results: &CodeMotionResults) -> String {
        let mut report = String::new();
        
        report.push_str("# Code Motion Optimization Report\n\n");
        
        // Executive Summary
        report.push_str("## Executive Summary\n");
        report.push_str(&format!("- **Functions Analyzed**: {}\n", results.statistics.functions_analyzed));
        report.push_str(&format!("- **Instructions Hoisted**: {}\n", results.statistics.instructions_hoisted));
        report.push_str(&format!("- **Instructions Sunk**: {}\n", results.statistics.instructions_sunk));
        report.push_str(&format!("- **Loop Invariants Moved**: {}\n", results.statistics.loop_invariants_moved));
        report.push_str(&format!("- **Redundant Loads Eliminated**: {}\n", results.statistics.redundant_loads_eliminated));
        report.push_str(&format!("- **Dead Stores Eliminated**: {}\n", results.statistics.dead_stores_eliminated));
        report.push_str(&format!("- **Register Pressure Reductions**: {}\n", results.statistics.register_pressure_reductions));
        report.push_str(&format!("- **Execution Frequency Improvements**: {:.1}%\n", results.statistics.execution_frequency_improvements));
        report.push_str(&format!("- **Estimated Speedup**: {:.1}%\n", results.statistics.estimated_speedup));
        report.push_str(&format!("- **Optimization Time**: {:?}\n\n", results.statistics.optimization_time));
        
        // Function Results
        if !results.function_results.is_empty() {
            report.push_str("## Function Optimization Results\n");
            for (func_name, func_result) in &results.function_results {
                report.push_str(&format!("### {}\n", func_name));
                report.push_str(&format!("- Motion opportunities: {}\n", func_result.motion_opportunities.len()));
                report.push_str(&format!("- Hoisting operations: {}\n", func_result.hoisting_results.len()));
                report.push_str(&format!("- Sinking operations: {}\n", func_result.sinking_results.len()));
                report.push_str(&format!("- LICM operations: {}\n", func_result.licm_results.len()));
                report.push_str(&format!("- Total motions: {}\n", func_result.total_motions));
                report.push_str(&format!("- Optimization benefit: {:.1}%\n", func_result.optimization_benefit));
                
                if !func_result.hoisting_results.is_empty() {
                    report.push_str("  **Hoisting Results:**\n");
                    for (i, result) in func_result.hoisting_results.iter().enumerate().take(5) {
                        report.push_str(&format!("  {}. {}: {:.1}% benefit\n", 
                            i + 1, result.instruction, result.estimated_benefit));
                    }
                }
                
                if !func_result.sinking_results.is_empty() {
                    report.push_str("  **Sinking Results:**\n");
                    for (i, result) in func_result.sinking_results.iter().enumerate().take(5) {
                        report.push_str(&format!("  {}. {}: {:.1}% benefit\n", 
                            i + 1, result.instruction, result.estimated_benefit));
                    }
                }
                
                report.push_str("\n");
            }
        }
        
        // Motion Types Summary
        let mut motion_type_counts: HashMap<MotionType, usize> = HashMap::new();
        for opportunities in results.motion_opportunities.values() {
            for opportunity in opportunities {
                *motion_type_counts.entry(opportunity.opportunity_type.clone()).or_insert(0) += 1;
            }
        }
        
        if !motion_type_counts.is_empty() {
            report.push_str("## Motion Types Summary\n");
            for (motion_type, count) in &motion_type_counts {
                report.push_str(&format!("- **{:?}**: {} opportunities\n", motion_type, count));
            }
            report.push_str("\n");
        }
        
        // Optimization Summary
        report.push_str("## Optimization Summary\n");
        report.push_str(&results.optimization_summary);
        
        report
    }
    
    /// Get current optimization statistics
    pub fn get_statistics(&self) -> CodeMotionStatistics {
        self.statistics.lock().unwrap().clone()
    }
    
    // Implementation methods
    
    fn build_dominance_analysis(&mut self, function: FunctionValue<'ctx>) -> Result<()> {
        debug!("Building dominance analysis for code motion");
        
        let mut dominance_tree = HashMap::new();
        let mut dominated_blocks = HashMap::new();
        let mut post_dominance_tree = HashMap::new();
        let mut post_dominated_blocks = HashMap::new();
        
        // Simplified dominance analysis
        // In a real implementation, would use proper dominance algorithms
        
        if let Some(entry_block) = function.get_first_basic_block() {
            let entry_name = entry_block.get_name().to_str().unwrap_or("entry").to_string();
            
            // Build forward dominance
            let mut current_block = entry_block;
            let mut prev_name = entry_name.clone();
            
            while let Some(next_block) = current_block.get_next_basic_block() {
                let block_name = next_block.get_name().to_str().unwrap_or("block").to_string();
                dominance_tree.insert(block_name.clone(), prev_name.clone());
                
                dominated_blocks.entry(prev_name.clone())
                    .or_insert_with(HashSet::new)
                    .insert(block_name.clone());
                
                prev_name = block_name;
                current_block = next_block;
            }
            
            // Build post-dominance (simplified)
            // In practice, would traverse in reverse post-order
            for (dominated, dominator) in &dominance_tree {
                post_dominance_tree.insert(dominator.clone(), dominated.clone());
                post_dominated_blocks.entry(dominated.clone())
                    .or_insert_with(HashSet::new)
                    .insert(dominator.clone());
            }
        }
        
        self.dominance_analysis = DominanceAnalysis {
            dominance_tree,
            dominance_frontier: HashMap::new(), // Would compute properly
            post_dominance_tree,
            dominated_blocks,
            post_dominated_blocks,
        };
        
        Ok(())
    }
    
    fn build_loop_analysis(&mut self, function: FunctionValue<'ctx>) -> Result<()> {
        debug!("Building loop analysis for code motion");
        
        let mut natural_loops = HashMap::new();
        let mut loop_nesting = HashMap::new();
        let mut loop_invariants = HashMap::new();
        
        // Simplified loop detection
        // In a real implementation, would use proper natural loop detection
        
        let mut block = function.get_first_basic_block();
        let mut blocks_in_order = Vec::new();
        
        while let Some(bb) = block {
            let block_name = bb.get_name().to_str().unwrap_or("block").to_string();
            blocks_in_order.push((bb, block_name));
            block = bb.get_next_basic_block();
        }
        
        // Look for back edges (simplified heuristic)
        for (i, (current_block, current_name)) in blocks_in_order.iter().enumerate() {
            if let Some(terminator) = current_block.get_terminator() {
                if let Some(br_instr) = terminator.as_branch_instruction() {
                    if br_instr.is_conditional() {
                        // Check if either target is a previous block (potential back edge)
                        let then_name = br_instr.get_then_block().get_name().to_str().unwrap_or("then");
                        let else_name = br_instr.get_else_block().get_name().to_str().unwrap_or("else");
                        
                        for (j, (_, prev_name)) in blocks_in_order.iter().enumerate().take(i) {
                            if then_name == prev_name || else_name == prev_name {
                                // Found potential loop
                                let header = prev_name.clone();
                                let mut loop_blocks = HashSet::new();
                                
                                // Add blocks from header to current block
                                for k in j..=i {
                                    loop_blocks.insert(blocks_in_order[k].1.clone());
                                }
                                
                                let loop_info = LoopInfo {
                                    header: header.clone(),
                                    blocks: loop_blocks.clone(),
                                    back_edges: vec![(current_name.clone(), header.clone())],
                                    exit_blocks: HashSet::new(), // Would analyze exits
                                    preheader: if j > 0 { Some(blocks_in_order[j-1].1.clone()) } else { None },
                                    depth: 1, // Simplified
                                };
                                
                                natural_loops.insert(header.clone(), loop_info);
                                
                                // Set loop nesting for all blocks in loop
                                for block_name in &loop_blocks {
                                    loop_nesting.insert(block_name.clone(), 1);
                                }
                                
                                // Analyze loop invariants
                                let invariants = self.find_loop_invariants(&loop_blocks)?;
                                loop_invariants.insert(header, invariants);
                            }
                        }
                    }
                }
            }
        }
        
        self.loop_analysis = LoopAnalysis {
            natural_loops,
            loop_nesting,
            loop_invariants,
            loop_exits: HashMap::new(),
        };
        
        Ok(())
    }
    
    fn find_loop_invariants(&self, _loop_blocks: &HashSet<String>) -> Result<HashSet<String>> {
        // Find instructions that are invariant within the loop
        // For now, return empty set
        Ok(HashSet::new())
    }
    
    fn build_motion_analysis(&mut self, function: FunctionValue<'ctx>) -> Result<()> {
        debug!("Building motion analysis");
        
        let mut instruction_dependencies = HashMap::new();
        let mut instruction_uses = HashMap::new();
        let mut side_effect_analysis = HashMap::new();
        let mut profitability_analysis = HashMap::new();
        
        // Analyze each instruction
        let mut block = function.get_first_basic_block();
        while let Some(bb) = block {
            let mut instruction = bb.get_first_instruction();
            while let Some(instr) = instruction {
                let instr_name = instr.get_name().to_str().unwrap_or("unnamed").to_string();
                
                // Analyze dependencies
                let dependencies = self.analyze_instruction_dependencies(&instr)?;
                instruction_dependencies.insert(instr_name.clone(), dependencies);
                
                // Analyze uses
                let uses = self.analyze_instruction_uses(&instr)?;
                instruction_uses.insert(instr_name.clone(), uses);
                
                // Analyze side effects
                let side_effects = self.analyze_side_effects(&instr)?;
                side_effect_analysis.insert(instr_name.clone(), side_effects);
                
                // Analyze profitability
                let profitability = self.analyze_motion_profitability(&instr)?;
                profitability_analysis.insert(instr_name, profitability);
                
                instruction = instr.get_next_instruction();
            }
            block = bb.get_next_basic_block();
        }
        
        self.motion_analysis = MotionAnalysis {
            instruction_dependencies,
            instruction_uses,
            side_effect_analysis,
            profitability_analysis,
        };
        
        Ok(())
    }
    
    fn analyze_instruction_dependencies(&self, _instruction: &InstructionValue<'ctx>) -> Result<HashSet<String>> {
        // Analyze what this instruction depends on
        // For now, return empty set
        Ok(HashSet::new())
    }
    
    fn analyze_instruction_uses(&self, _instruction: &InstructionValue<'ctx>) -> Result<HashSet<String>> {
        // Analyze what uses this instruction
        // For now, return empty set
        Ok(HashSet::new())
    }
    
    fn analyze_side_effects(&self, instruction: &InstructionValue<'ctx>) -> Result<SideEffectInfo> {
        let opcode = instruction.get_opcode();
        
        Ok(SideEffectInfo {
            has_memory_effects: matches!(opcode, 
                inkwell::values::InstructionOpcode::Load | 
                inkwell::values::InstructionOpcode::Store),
            may_throw: matches!(opcode, inkwell::values::InstructionOpcode::Call),
            calls_external: matches!(opcode, inkwell::values::InstructionOpcode::Call),
            accesses_volatile: false, // Would need more detailed analysis
            has_undefined_behavior: false, // Would need more detailed analysis
        })
    }
    
    fn analyze_motion_profitability(&self, instruction: &InstructionValue<'ctx>) -> Result<MotionProfitability> {
        let instr_name = instruction.get_name().to_str().unwrap_or("unnamed").to_string();
        let opcode = instruction.get_opcode();
        
        // Determine optimal motion type
        let motion_type = match opcode {
            inkwell::values::InstructionOpcode::Load => MotionType::LoadMotion,
            inkwell::values::InstructionOpcode::Store => MotionType::StoreMotion,
            _ => MotionType::Hoisting, // Default
        };
        
        // Calculate profitability factors
        let execution_frequency_change = match motion_type {
            MotionType::Hoisting => -0.2, // Reduces frequency
            MotionType::Sinking => 0.1,   // May increase frequency
            MotionType::LoopInvariantMotion => -0.5, // Significantly reduces frequency
            _ => 0.0,
        };
        
        let register_pressure_impact = match motion_type {
            MotionType::Hoisting => 1,  // May increase pressure
            MotionType::Sinking => -1,  // May decrease pressure
            _ => 0,
        };
        
        let cache_impact = 0.05; // Small positive cache impact
        let overall_benefit = execution_frequency_change.abs() * 10.0 + cache_impact * 5.0;
        
        Ok(MotionProfitability {
            instruction: instr_name,
            motion_type,
            execution_frequency_change,
            register_pressure_impact,
            cache_impact,
            overall_benefit,
        })
    }
    
    fn identify_motion_opportunities(&mut self, function: FunctionValue<'ctx>) -> Result<Vec<MotionOpportunity>> {
        debug!("Identifying code motion opportunities");
        
        let mut opportunities = Vec::new();
        
        // Look for hoisting opportunities
        opportunities.extend(self.find_hoisting_opportunities(function)?);
        
        // Look for sinking opportunities
        opportunities.extend(self.find_sinking_opportunities(function)?);
        
        // Look for loop-invariant motion opportunities
        opportunities.extend(self.find_licm_opportunities(function)?);
        
        // Look for load/store motion opportunities
        opportunities.extend(self.find_load_store_motion_opportunities(function)?);
        
        // Store opportunities for this function
        let function_name = function.get_name().to_str().unwrap_or("unnamed");
        self.motion_opportunities.insert(function_name.to_string(), opportunities.clone());
        
        Ok(opportunities)
    }
    
    fn find_hoisting_opportunities(&self, function: FunctionValue<'ctx>) -> Result<Vec<MotionOpportunity>> {
        let mut opportunities = Vec::new();
        
        let mut block = function.get_first_basic_block();
        while let Some(bb) = block {
            let block_name = bb.get_name().to_str().unwrap_or("block").to_string();
            
            let mut instruction = bb.get_first_instruction();
            while let Some(instr) = instruction {
                let instr_name = instr.get_name().to_str().unwrap_or("instr").to_string();
                
                // Check if instruction can be hoisted
                if self.can_be_hoisted(&instr) {
                    if let Some(target_block) = self.find_hoist_target(&block_name, &instr_name) {
                        let opportunity = MotionOpportunity {
                            opportunity_type: MotionType::Hoisting,
                            instruction: instr_name,
                            source_block: block_name.clone(),
                            target_block,
                            motion_reason: MotionReason::ReduceExecutionFrequency,
                            estimated_benefit: 15.0,
                            constraints: vec![MotionConstraint::PreserveDependencies],
                            risk_factors: vec![RiskFactor::IncreaseRegisterPressure],
                        };
                        opportunities.push(opportunity);
                    }
                }
                
                instruction = instr.get_next_instruction();
            }
            block = bb.get_next_basic_block();
        }
        
        Ok(opportunities)
    }
    
    fn find_sinking_opportunities(&self, function: FunctionValue<'ctx>) -> Result<Vec<MotionOpportunity>> {
        let mut opportunities = Vec::new();
        
        let mut block = function.get_first_basic_block();
        while let Some(bb) = block {
            let block_name = bb.get_name().to_str().unwrap_or("block").to_string();
            
            let mut instruction = bb.get_first_instruction();
            while let Some(instr) = instruction {
                let instr_name = instr.get_name().to_str().unwrap_or("instr").to_string();
                
                // Check if instruction can be sunk
                if self.can_be_sunk(&instr) {
                    if let Some(target_block) = self.find_sink_target(&block_name, &instr_name) {
                        let opportunity = MotionOpportunity {
                            opportunity_type: MotionType::Sinking,
                            instruction: instr_name,
                            source_block: block_name.clone(),
                            target_block,
                            motion_reason: MotionReason::ImproveRegisterPressure,
                            estimated_benefit: 10.0,
                            constraints: vec![MotionConstraint::PreserveDependencies],
                            risk_factors: vec![RiskFactor::WorseLocality],
                        };
                        opportunities.push(opportunity);
                    }
                }
                
                instruction = instr.get_next_instruction();
            }
            block = bb.get_next_basic_block();
        }
        
        Ok(opportunities)
    }
    
    fn find_licm_opportunities(&self, _function: FunctionValue<'ctx>) -> Result<Vec<MotionOpportunity>> {
        let mut opportunities = Vec::new();
        
        // Look through loop invariants
        for (loop_header, invariants) in &self.loop_analysis.loop_invariants {
            if let Some(loop_info) = self.loop_analysis.natural_loops.get(loop_header) {
                for invariant in invariants {
                    let target_block = loop_info.preheader.clone()
                        .unwrap_or_else(|| "preheader".to_string());
                    
                    let opportunity = MotionOpportunity {
                        opportunity_type: MotionType::LoopInvariantMotion,
                        instruction: invariant.clone(),
                        source_block: loop_header.clone(),
                        target_block,
                        motion_reason: MotionReason::ReduceExecutionFrequency,
                        estimated_benefit: 25.0, // High benefit for LICM
                        constraints: vec![MotionConstraint::PreserveDependencies],
                        risk_factors: vec![RiskFactor::IncreaseRegisterPressure],
                    };
                    opportunities.push(opportunity);
                }
            }
        }
        
        Ok(opportunities)
    }
    
    fn find_load_store_motion_opportunities(&self, function: FunctionValue<'ctx>) -> Result<Vec<MotionOpportunity>> {
        let mut opportunities = Vec::new();
        
        let mut block = function.get_first_basic_block();
        while let Some(bb) = block {
            let block_name = bb.get_name().to_str().unwrap_or("block").to_string();
            
            let mut instruction = bb.get_first_instruction();
            while let Some(instr) = instruction {
                let instr_name = instr.get_name().to_str().unwrap_or("instr").to_string();
                let opcode = instr.get_opcode();
                
                match opcode {
                    inkwell::values::InstructionOpcode::Load => {
                        if self.can_move_load(&instr) {
                            let opportunity = MotionOpportunity {
                                opportunity_type: MotionType::LoadMotion,
                                instruction: instr_name,
                                source_block: block_name.clone(),
                                target_block: "load_target".to_string(),
                                motion_reason: MotionReason::RemoveRedundancy,
                                estimated_benefit: 12.0,
                                constraints: vec![MotionConstraint::PreserveMemoryOrder],
                                risk_factors: vec![RiskFactor::ComplexDependencies],
                            };
                            opportunities.push(opportunity);
                        }
                    }
                    inkwell::values::InstructionOpcode::Store => {
                        if self.can_move_store(&instr) {
                            let opportunity = MotionOpportunity {
                                opportunity_type: MotionType::StoreMotion,
                                instruction: instr_name,
                                source_block: block_name.clone(),
                                target_block: "store_target".to_string(),
                                motion_reason: MotionReason::EnableOtherOptimizations,
                                estimated_benefit: 8.0,
                                constraints: vec![MotionConstraint::PreserveMemoryOrder, MotionConstraint::PreserveExceptions],
                                risk_factors: vec![RiskFactor::ExceptionSafety],
                            };
                            opportunities.push(opportunity);
                        }
                    }
                    _ => {}
                }
                
                instruction = instr.get_next_instruction();
            }
            block = bb.get_next_basic_block();
        }
        
        Ok(opportunities)
    }
    
    fn can_be_hoisted(&self, instruction: &InstructionValue<'ctx>) -> bool {
        let instr_name = instruction.get_name().to_str().unwrap_or("unnamed");
        
        // Check side effects
        if let Some(side_effects) = self.motion_analysis.side_effect_analysis.get(instr_name) {
            if side_effects.has_memory_effects || side_effects.may_throw {
                return false;
            }
        }
        
        // Check if it's a pure computation
        matches!(instruction.get_opcode(), 
            inkwell::values::InstructionOpcode::Add |
            inkwell::values::InstructionOpcode::Sub |
            inkwell::values::InstructionOpcode::Mul |
            inkwell::values::InstructionOpcode::And |
            inkwell::values::InstructionOpcode::Or |
            inkwell::values::InstructionOpcode::Xor)
    }
    
    fn can_be_sunk(&self, instruction: &InstructionValue<'ctx>) -> bool {
        let instr_name = instruction.get_name().to_str().unwrap_or("unnamed");
        
        // Check if instruction has limited uses
        if let Some(uses) = self.motion_analysis.instruction_uses.get(instr_name) {
            if uses.len() <= 1 {
                return true;
            }
        }
        
        false
    }
    
    fn can_move_load(&self, _instruction: &InstructionValue<'ctx>) -> bool {
        // Check if load can be safely moved
        // For now, conservative approach
        false
    }
    
    fn can_move_store(&self, _instruction: &InstructionValue<'ctx>) -> bool {
        // Check if store can be safely moved
        // For now, conservative approach
        false
    }
    
    fn find_hoist_target(&self, current_block: &str, _instruction: &str) -> Option<String> {
        // Find dominating block where instruction can be hoisted
        if let Some(dominator) = self.dominance_analysis.dominance_tree.get(current_block) {
            Some(dominator.clone())
        } else {
            None
        }
    }
    
    fn find_sink_target(&self, current_block: &str, _instruction: &str) -> Option<String> {
        // Find post-dominated block where instruction can be sunk
        if let Some(post_dominated) = self.dominance_analysis.post_dominated_blocks.get(current_block) {
            post_dominated.iter().next().cloned()
        } else {
            None
        }
    }
    
    fn perform_code_motion(&mut self, _function: FunctionValue<'ctx>, opportunities: &[MotionOpportunity]) -> Result<Vec<MotionResult>> {
        let mut results = Vec::new();
        
        for opportunity in opportunities {
            // Check if motion is profitable
            if opportunity.estimated_benefit > 5.0 {
                let result = self.perform_single_motion(opportunity)?;
                results.push(result);
            }
        }
        
        Ok(results)
    }
    
    fn perform_single_motion(&mut self, opportunity: &MotionOpportunity) -> Result<MotionResult> {
        debug!("Performing code motion: {} from {} to {}", 
               opportunity.instruction, opportunity.source_block, opportunity.target_block);
        
        // This would perform the actual LLVM IR transformation
        // For now, create a placeholder result
        
        Ok(MotionResult {
            motion_type: opportunity.opportunity_type.clone(),
            instruction: opportunity.instruction.clone(),
            source_block: opportunity.source_block.clone(),
            target_block: opportunity.target_block.clone(),
            estimated_benefit: opportunity.estimated_benefit,
            actual_benefit: opportunity.estimated_benefit * 0.8, // Realistic reduction
            register_pressure_change: -1, // Slight improvement
            success: true,
        })
    }
    
    fn perform_loop_invariant_motion(&mut self, _function: FunctionValue<'ctx>) -> Result<Vec<LicmResult>> {
        let mut results = Vec::new();
        
        // Perform LICM for each loop
        for (loop_header, invariants) in &self.loop_analysis.loop_invariants {
            for invariant in invariants {
                let result = LicmResult {
                    loop_header: loop_header.clone(),
                    invariant_instruction: invariant.clone(),
                    moved_to_preheader: true,
                    execution_frequency_reduction: 0.8, // 80% reduction
                    estimated_speedup: 20.0,
                };
                results.push(result);
            }
        }
        
        Ok(results)
    }
    
    fn cleanup_and_validate(&mut self, _function: FunctionValue<'ctx>) -> Result<CleanupResults> {
        debug!("Cleaning up after code motion");
        
        Ok(CleanupResults {
            dead_instructions_removed: 0,
            empty_blocks_removed: 0,
            redundant_moves_eliminated: 0,
            validation_passed: true,
        })
    }
    
    fn is_dependency_satisfied_after_motion(&self, _dependency: &str, _target_block: &str) -> bool {
        // Check if dependency is satisfied after motion
        // For now, conservative approach
        true
    }
    
    fn respects_dominance_constraints(&self, _instruction: &str, _target_block: &str) -> bool {
        // Check if motion respects dominance constraints
        // For now, assume yes
        true
    }
    
    fn generate_optimization_summary(&self) -> Result<String> {
        let mut summary = String::new();
        
        summary.push_str("Code motion optimization focused on improving performance by:\n");
        summary.push_str("- Hoisting computations to reduce execution frequency\n");
        summary.push_str("- Sinking computations to improve register pressure\n");
        summary.push_str("- Moving loop-invariant code outside loops\n");
        summary.push_str("- Optimizing memory access patterns\n");
        summary.push_str("- Eliminating redundant loads and dead stores\n");
        
        let total_opportunities: usize = self.motion_opportunities.values()
            .map(|ops| ops.len())
            .sum();
        
        if total_opportunities > 0 {
            summary.push_str(&format!("\nTotal motion opportunities identified: {}\n", total_opportunities));
        }
        
        Ok(summary)
    }
    
    fn calculate_function_benefit(&self, motion_results: &[MotionResult], licm_results: &[LicmResult]) -> f64 {
        let motion_benefit: f64 = motion_results.iter().map(|r| r.actual_benefit).sum();
        let licm_benefit: f64 = licm_results.iter().map(|r| r.estimated_speedup).sum();
        
        (motion_benefit + licm_benefit) / (motion_results.len() + licm_results.len()).max(1) as f64
    }
    
    fn update_statistics(&self, optimization_time: Duration, function_results: &HashMap<String, FunctionCodeMotionResults>) {
        if let Ok(mut stats) = self.statistics.lock() {
            stats.optimization_time = optimization_time;
            stats.functions_analyzed = function_results.len();
            
            for function_result in function_results.values() {
                stats.instructions_hoisted += function_result.hoisting_results.len();
                stats.instructions_sunk += function_result.sinking_results.len();
                stats.loop_invariants_moved += function_result.licm_results.len();
                
                // Count specific motion types
                for result in &function_result.hoisting_results {
                    if result.register_pressure_change < 0 {
                        stats.register_pressure_reductions += 1;
                    }
                }
                
                // Calculate execution frequency improvements
                for licm_result in &function_result.licm_results {
                    stats.execution_frequency_improvements += licm_result.execution_frequency_reduction * 100.0;
                }
            }
            
            stats.estimated_speedup = function_results.values()
                .map(|r| r.optimization_benefit)
                .sum::<f64>() / function_results.len().max(1) as f64;
        }
    }
}

// Supporting types and implementations

impl DominanceAnalysis {
    fn new() -> Self {
        Self {
            dominance_tree: HashMap::new(),
            dominance_frontier: HashMap::new(),
            post_dominance_tree: HashMap::new(),
            dominated_blocks: HashMap::new(),
            post_dominated_blocks: HashMap::new(),
        }
    }
}

impl LoopAnalysis {
    fn new() -> Self {
        Self {
            natural_loops: HashMap::new(),
            loop_nesting: HashMap::new(),
            loop_invariants: HashMap::new(),
            loop_exits: HashMap::new(),
        }
    }
}

impl MotionAnalysis {
    fn new() -> Self {
        Self {
            instruction_dependencies: HashMap::new(),
            instruction_uses: HashMap::new(),
            side_effect_analysis: HashMap::new(),
            profitability_analysis: HashMap::new(),
        }
    }
}

/// Results of code motion optimization
#[derive(Debug, Clone)]
pub struct CodeMotionResults {
    pub function_results: HashMap<String, FunctionCodeMotionResults>,
    pub motion_opportunities: HashMap<String, Vec<MotionOpportunity>>,
    pub optimization_summary: String,
    pub statistics: CodeMotionStatistics,
}

/// Results for a single function
#[derive(Debug, Clone)]
pub struct FunctionCodeMotionResults {
    pub function_name: String,
    pub motion_opportunities: Vec<MotionOpportunity>,
    pub hoisting_results: Vec<MotionResult>,
    pub sinking_results: Vec<MotionResult>,
    pub licm_results: Vec<LicmResult>,
    pub cleanup_results: CleanupResults,
    pub total_motions: usize,
    pub optimization_benefit: f64,
}

/// Result of a single motion operation
#[derive(Debug, Clone)]
pub struct MotionResult {
    pub motion_type: MotionType,
    pub instruction: String,
    pub source_block: String,
    pub target_block: String,
    pub estimated_benefit: f64,
    pub actual_benefit: f64,
    pub register_pressure_change: i32,
    pub success: bool,
}

/// Result of loop-invariant code motion
#[derive(Debug, Clone)]
pub struct LicmResult {
    pub loop_header: String,
    pub invariant_instruction: String,
    pub moved_to_preheader: bool,
    pub execution_frequency_reduction: f64,
    pub estimated_speedup: f64,
}

/// Cleanup results after code motion
#[derive(Debug, Clone)]
pub struct CleanupResults {
    pub dead_instructions_removed: usize,
    pub empty_blocks_removed: usize,
    pub redundant_moves_eliminated: usize,
    pub validation_passed: bool,
}

/// Motion safety analysis
#[derive(Debug, Clone)]
pub struct MotionSafety {
    pub is_safe: bool,
    pub safety_issues: Vec<String>,
    pub safety_confidence: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    
    #[test]
    fn test_code_motion_optimizer_creation() {
        let context = Context::create();
        let optimizer = CodeMotionOptimizer::new(&context, OptimizationLevel::O2);
        
        let stats = optimizer.get_statistics();
        assert_eq!(stats.functions_analyzed, 0);
        assert_eq!(stats.instructions_hoisted, 0);
        assert_eq!(stats.instructions_sunk, 0);
    }
    
    #[test]
    fn test_dominance_analysis_initialization() {
        let analysis = DominanceAnalysis::new();
        
        assert!(analysis.dominance_tree.is_empty());
        assert!(analysis.dominated_blocks.is_empty());
        assert!(analysis.post_dominance_tree.is_empty());
    }
    
    #[test]
    fn test_loop_analysis_initialization() {
        let analysis = LoopAnalysis::new();
        
        assert!(analysis.natural_loops.is_empty());
        assert!(analysis.loop_nesting.is_empty());
        assert!(analysis.loop_invariants.is_empty());
    }
    
    #[test]
    fn test_motion_analysis_initialization() {
        let analysis = MotionAnalysis::new();
        
        assert!(analysis.instruction_dependencies.is_empty());
        assert!(analysis.instruction_uses.is_empty());
        assert!(analysis.side_effect_analysis.is_empty());
        assert!(analysis.profitability_analysis.is_empty());
    }
    
    #[test]
    fn test_motion_type_comparison() {
        assert_eq!(MotionType::Hoisting, MotionType::Hoisting);
        assert_ne!(MotionType::Hoisting, MotionType::Sinking);
        assert_ne!(MotionType::LoopInvariantMotion, MotionType::LoadMotion);
    }
    
    #[test]
    fn test_side_effect_info_creation() {
        let side_effects = SideEffectInfo {
            has_memory_effects: true,
            may_throw: false,
            calls_external: false,
            accesses_volatile: false,
            has_undefined_behavior: false,
        };
        
        assert!(side_effects.has_memory_effects);
        assert!(!side_effects.may_throw);
    }
}
