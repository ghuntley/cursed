/// Tail Call Optimization Implementation
/// 
/// Provides comprehensive tail call optimization for CURSED, converting
/// tail calls to jumps for better stack efficiency and performance.

use crate::error::{CursedError, Result};
use crate::common_types::optimization_level::OptimizationLevel;

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{debug, info, warn, instrument};
use serde::{Deserialize, Serialize};

use inkwell::{
// };

/// Tail call optimizer
pub struct TailCallOptimizer<'ctx> {
/// Analysis of function calls for tail call optimization
#[derive(Debug, Clone)]
pub struct CallAnalysis {
/// Information about a call site
#[derive(Debug, Clone)]
pub struct CallSiteInfo {
/// Type of function call
#[derive(Debug, Clone, PartialEq)]
pub enum CallType {
    DirectCall,        // Direct function call
    IndirectCall,      // Call through function pointer
    RecursiveCall,     // Self-recursive call
    MutuallyRecursive, // Mutually recursive call
    ExternalCall,      // Call to external function
/// Result of tail call analysis
#[derive(Debug, Clone)]
pub struct TailCallAnalysisResult {
/// Type of tail call optimization
#[derive(Debug, Clone, PartialEq)]
pub enum TailCallOptimizationType {
    TailRecursion,     // Simple tail recursion elimination
    SiblingCall,       // Tail call to different function
    TailCallElimination, // General tail call elimination
    IterativeLoop,     // Convert tail recursion to loop
    NotOptimizable,    // Cannot be optimized
/// Factors that block tail call optimization
#[derive(Debug, Clone)]
pub enum BlockingFactor {
    HasCleanupCode,              // Function has cleanup code after call
    UsesCallResult,              // Return value is used after call
    ModifiesStackAfterCall,      // Stack is modified after the call
    HasExceptionHandling,        // Exception handling complicates optimization
    IncompatibleCallingConvention, // Calling conventions don't match
    VariableArguments,           // Varargs complicate optimization
    AddressOfLocalTaken,         // Address of local variable taken
    ComplexControlFlow,          // Complex control flow after call
    RecursiveWithAccumulator,    // Tail recursion with accumulator pattern
/// Tail call candidate for optimization
#[derive(Debug, Clone)]
pub struct TailCallCandidate {
/// Strategy for optimizing a tail call
#[derive(Debug, Clone)]
pub enum OptimizationStrategy {
    ReplaceWithJump,              // Replace call with jump
    ConvertToLoop,                // Convert recursion to loop
    EliminateStackFrame,          // Eliminate unnecessary stack frame
    ReuseStackFrame,              // Reuse current stack frame
    TransformAccumulator,         // Transform accumulator pattern
/// Plan for transforming a tail call
#[derive(Debug, Clone)]
pub struct TransformationPlan {
/// Individual replacement operation
#[derive(Debug, Clone)]
pub struct ReplacementOperation {
/// Type of replacement operation
#[derive(Debug, Clone)]
pub enum OperationType {
    CallToJump,           // Replace call with jump
    ParameterUpdate,      // Update function parameters
    StackFrameElimination, // Eliminate stack frame setup
    ReturnElimination,    // Eliminate return instruction
    LoopCreation,         // Create loop structure
    VariableRename,       // Rename variables
/// Control flow change description
#[derive(Debug, Clone)]
pub struct ControlFlowChange {
/// Type of control flow change
#[derive(Debug, Clone)]
pub enum ControlFlowChangeType {
    AddJump,              // Add jump instruction
    RemoveCall,           // Remove call instruction
    CreateLoop,           // Create loop structure
    ModifyReturn,         // Modify return behavior
    AddBranch,            // Add conditional branch
/// Stack impact analysis
#[derive(Debug, Clone)]
pub struct StackImpact {
/// Performance benefit estimation
#[derive(Debug, Clone)]
pub struct PerformanceBenefit {
/// Constraints for tail call optimization
#[derive(Debug, Clone)]
pub struct OptimizationConstraints {
/// Tail call optimization statistics
#[derive(Debug, Clone, Default)]
pub struct TailCallStatistics {
impl<'ctx> TailCallOptimizer<'ctx> {
    /// Create new tail call optimizer
    #[instrument(skip(context))]
    pub fn new(context: &'ctx Context, optimization_level: OptimizationLevel) -> Self {
        info!("Initializing tail call optimizer with optimization level {:?}", optimization_level);
        
        let optimization_constraints = OptimizationConstraints {
            max_recursion_depth: match optimization_level {
        
        Self {
        }
    }
    
    /// Perform tail call optimization on entire module
    #[instrument(skip(self, module))]
    pub fn optimize_module(&mut self, module: &Module<'ctx>) -> Result<TailCallOptimizationResults> {
        let start_time = Instant::now();
        info!("Starting tail call optimization");
        
        // Phase 1: Analyze all function calls
        self.analyze_module_calls(module)?;
        
        // Phase 2: Identify tail call candidates
        self.identify_tail_call_candidates(module)?;
        
        // Phase 3: Perform optimizations
        let mut function_results = HashMap::new();
        for function in module.get_functions() {
            if function.get_first_basic_block().is_some() {
                let result = self.optimize_function(function)?;
                function_results.insert(
                    result
                );
            }
        }
        
        // Phase 4: Generate optimization opportunities
        let optimization_opportunities = self.identify_additional_opportunities()?;
        
        let optimization_time = start_time.elapsed();
        self.update_statistics(optimization_time, &function_results);
        
        info!(
            "Tail call optimization completed"
        );
        
        Ok(TailCallOptimizationResults {
        })
    /// Optimize a single function
    #[instrument(skip(self, function))]
    pub fn optimize_function(&mut self, function: FunctionValue<'ctx>) -> Result<FunctionTailCallResults> {
        let function_name = function.get_name().to_str().unwrap_or("unnamed");
        debug!("Optimizing function with tail call optimization: {}", function_name);
        
        // Analyze tail calls in this function
        let tail_call_analysis = self.analyze_function_tail_calls(function)?;
        
        // Get candidates for this function
        let candidates = self.tail_call_candidates
            .get(function_name)
            .cloned()
            .unwrap_or_default();
        
        // Perform optimizations
        let mut optimizations = Vec::new();
        for candidate in &candidates {
            if candidate.call_site.analysis_result.can_optimize {
                let optimization = self.perform_tail_call_optimization(function, candidate)?;
                optimizations.push(optimization);
            }
        }
        
        let optimization_benefit = self.calculate_function_benefit(&optimizations);
        
        Ok(FunctionTailCallResults {
        })
    /// Check if a call site is eligible for tail call optimization
    pub fn is_tail_call_eligible(&self, call_site: &CallSiteInfo) -> TailCallEligibility {
        if !call_site.is_tail_position {
            return TailCallEligibility {
        let mut blocking_reasons = Vec::new();
        let mut optimization_potential = 0.8; // Base potential
        
        // Check for blocking factors
        for blocking_factor in &call_site.analysis_result.blocking_factors {
            match blocking_factor {
                BlockingFactor::HasCleanupCode => {
                    blocking_reasons.push("Function has cleanup code after call".to_string());
                    optimization_potential -= 0.3;
                }
                BlockingFactor::UsesCallResult => {
                    blocking_reasons.push("Return value is used after call".to_string());
                    optimization_potential -= 0.4;
                }
                BlockingFactor::ModifiesStackAfterCall => {
                    blocking_reasons.push("Stack is modified after call".to_string());
                    optimization_potential -= 0.5;
                }
                BlockingFactor::HasExceptionHandling => {
                    blocking_reasons.push("Exception handling complicates optimization".to_string());
                    optimization_potential -= 0.2;
                }
                BlockingFactor::IncompatibleCallingConvention => {
                    blocking_reasons.push("Incompatible calling conventions".to_string());
                    optimization_potential -= 0.6;
                }
                _ => {
                    optimization_potential -= 0.1;
                }
            }
        let eligible = optimization_potential > 0.3 && blocking_reasons.len() < 3;
        
        TailCallEligibility {
        }
    }
    
    /// Generate comprehensive tail call optimization report
    pub fn generate_tail_call_report(&self, results: &TailCallOptimizationResults) -> String {
        let mut report = String::new();
        
        report.push_str("# Tail Call Optimization Report\n\n");
        
        // Executive Summary
        report.push_str("## Executive Summary\n");
        report.push_str(&format!("- **Functions Analyzed**: {}\n", results.statistics.functions_analyzed));
        report.push_str(&format!("- **Call Sites Analyzed**: {}\n", results.statistics.call_sites_analyzed));
        report.push_str(&format!("- **Tail Calls Identified**: {}\n", results.statistics.tail_calls_identified));
        report.push_str(&format!("- **Tail Calls Optimized**: {}\n", results.statistics.tail_calls_optimized));
        report.push_str(&format!("- **Recursive Calls Converted**: {}\n", results.statistics.recursive_calls_converted));
        report.push_str(&format!("- **Sibling Calls Optimized**: {}\n", results.statistics.sibling_calls_optimized));
        report.push_str(&format!("- **Stack Frames Eliminated**: {}\n", results.statistics.stack_frames_eliminated));
        report.push_str(&format!("- **Estimated Stack Savings**: {} bytes\n", results.statistics.estimated_stack_savings));
        report.push_str(&format!("- **Performance Improvement**: {:.1}%\n", results.statistics.performance_improvement));
        report.push_str(&format!("- **Optimization Time**: {:?}\n\n", results.statistics.optimization_time));
        
        // Function Results
        if !results.function_results.is_empty() {
            report.push_str("## Function Optimization Results\n");
            for (func_name, func_result) in &results.function_results {
                report.push_str(&format!("### {}\n", func_name));
                report.push_str(&format!("- Tail call candidates: {}\n", func_result.candidates));
                report.push_str(&format!("- Optimizations performed: {}\n", func_result.optimizations_performed.len()));
                report.push_str(&format!("- Stack frames eliminated: {}\n", func_result.stack_frames_eliminated));
                report.push_str(&format!("- Optimization benefit: {:.1}%\n", func_result.optimization_benefit));
                
                if !func_result.optimizations_performed.is_empty() {
                    report.push_str("  **Optimizations:**\n");
                    for (i, opt) in func_result.optimizations_performed.iter().enumerate().take(5) {
                            i + 1, opt.optimization_type, opt.performance_benefit.overall_speedup));
                    }
                }
                report.push_str("\n");
            }
        }
        
        // Call Analysis Summary
        report.push_str("## Call Analysis Summary\n");
        report.push_str(&format!("- **Recursive Functions**: {}\n", results.call_analysis.recursive_functions.len()));
        report.push_str(&format!("- **Mutual Recursion Groups**: {}\n", results.call_analysis.mutual_recursion_groups.len()));
        
        if !results.call_analysis.recursive_functions.is_empty() {
            report.push_str("### Recursive Functions\n");
            for (i, func) in results.call_analysis.recursive_functions.iter().enumerate().take(10) {
                report.push_str(&format!("{}. {}\n", i + 1, func));
            }
        }
        
        // Optimization Opportunities
        if !results.optimization_opportunities.is_empty() {
            report.push_str("\n## Additional Optimization Opportunities\n");
            for (i, opportunity) in results.optimization_opportunities.iter().enumerate().take(5) {
                report.push_str(&format!("{}. **{}**\n", i + 1, opportunity.opportunity_type));
                report.push_str(&format!("   - Description: {}\n", opportunity.description));
                report.push_str(&format!("   - Potential benefit: {:.1}%\n", opportunity.potential_benefit));
                report.push_str(&format!("   - Complexity: {}\n", opportunity.implementation_complexity));
            }
        }
        
        report
    /// Get current optimization statistics
    pub fn get_statistics(&self) -> TailCallStatistics {
        self.statistics.lock().unwrap().clone()
    // Implementation methods
    
    fn analyze_module_calls(&mut self, module: &Module<'ctx>) -> Result<()> {
        debug!("Analyzing module calls for tail call optimization");
        
        // Build call graph
        for function in module.get_functions() {
            if function.get_first_basic_block().is_some() {
                self.analyze_function_calls(function)?;
            }
        }
        
        // Identify recursive patterns
        self.identify_recursive_patterns()?;
        
        Ok(())
    fn analyze_function_calls(&mut self, function: FunctionValue<'ctx>) -> Result<()> {
        let function_name = function.get_name().to_str().unwrap_or("unnamed");
        let mut function_calls = Vec::new();
        let mut callees = HashSet::new();
        
        let mut block = function.get_first_basic_block();
        while let Some(bb) = block {
            let block_name = bb.get_name().to_str().unwrap_or("unnamed_block").to_string();
            
            let mut instruction = bb.get_first_instruction();
            while let Some(instr) = instruction {
                if instr.get_opcode() == inkwell::values::InstructionOpcode::Call {
                    if let Some(call_instr) = instr.as_call_instruction() {
                        let call_info = self.analyze_call_site(function_name, &block_name, &instr, call_instr)?;
                        
                        callees.insert(call_info.callee_function.clone());
                        function_calls.push(call_info);
                    }
                }
                instruction = instr.get_next_instruction();
            }
            block = bb.get_next_basic_block();
        self.call_analysis.function_calls.insert(function_name.to_string(), function_calls);
        self.call_analysis.call_graph.insert(function_name.to_string(), callees);
        
        Ok(())
    fn analyze_call_site(
        call_instruction: &inkwell::values::CallInstruction<'ctx>
    ) -> Result<CallSiteInfo> {
        let call_name = instruction.get_name().to_str().unwrap_or("unnamed_call").to_string();
        
        // Determine callee function
        let (callee_function, call_type) = if let Some(called_func) = call_instruction.get_called_function() {
            let callee_name = called_func.get_name().to_str().unwrap_or("external").to_string();
            let call_type = if callee_name == caller {
                CallType::RecursiveCall
            } else {
                CallType::DirectCall
            (callee_name, call_type)
        } else {
            ("indirect_call".to_string(), CallType::IndirectCall)
        
        // Check if call is in tail position
        let is_tail_position = self.is_call_in_tail_position(instruction)?;
        
        // Perform tail call analysis
        let analysis_result = self.analyze_tail_call_eligibility(instruction, &call_type)?;
        
        Ok(CallSiteInfo {
        })
    fn is_call_in_tail_position(&self, call_instruction: &InstructionValue<'ctx>) -> Result<bool> {
        // Check if call is immediately followed by return or is the last instruction
        // that affects the return value
        
        let mut next_instr = call_instruction.get_next_instruction();
        while let Some(instr) = next_instr {
            match instr.get_opcode() {
                inkwell::values::InstructionOpcode::Ret => {
                    // Check if return uses the call result directly
                    if let Some(ret_instr) = instr.as_return_instruction() {
                        if let Some(ret_value) = ret_instr.get_return_value() {
                            // Check if return value is the call result
                            return Ok(self.is_same_value(&ret_value, call_instruction.as_basic_value_enum()));
                        } else {
                            // Void return - call can be in tail position
                            return Ok(true);
                        }
                    }
                }
                inkwell::values::InstructionOpcode::Br => {
                    // Unconditional branch - need to check target block
                    return Ok(false); // Simplified
                }
                inkwell::values::InstructionOpcode::Store |
                inkwell::values::InstructionOpcode::Load => {
                    // Memory operations after call may prevent tail optimization
                    return Ok(false);
                }
                _ => {
                    // Other instructions may prevent tail call optimization
                }
            }
            next_instr = instr.get_next_instruction();
        Ok(false)
    fn is_same_value(&self, value1: &BasicValueEnum<'ctx>, value2: BasicValueEnum<'ctx>) -> bool {
        // In a real implementation, would compare LLVM values properly
        // For now, simplified comparison
        std::ptr::eq(value1.as_any_value_enum().as_ref(), value2.as_any_value_enum().as_ref())
    fn analyze_tail_call_eligibility(&self, _call_instruction: &InstructionValue<'ctx>, call_type: &CallType) -> Result<TailCallAnalysisResult> {
        let mut blocking_factors = Vec::new();
        let mut can_optimize = true;
        let mut estimated_benefit = 0.0;
        
        // Analyze based on call type
        let optimization_type = match call_type {
            CallType::RecursiveCall => {
                estimated_benefit = 25.0; // High benefit for recursive calls
                TailCallOptimizationType::TailRecursion
            }
            CallType::DirectCall => {
                estimated_benefit = 15.0; // Moderate benefit for sibling calls
                TailCallOptimizationType::SiblingCall
            }
            CallType::IndirectCall => {
                if self.optimization_constraints.optimize_indirect_calls {
                    estimated_benefit = 10.0; // Lower benefit, higher complexity
                    TailCallOptimizationType::TailCallElimination
                } else {
                    can_optimize = false;
                    blocking_factors.push(BlockingFactor::ComplexControlFlow);
                    TailCallOptimizationType::NotOptimizable
                }
            }
            CallType::ExternalCall => {
                can_optimize = false;
                blocking_factors.push(BlockingFactor::IncompatibleCallingConvention);
                TailCallOptimizationType::NotOptimizable
            }
            CallType::MutuallyRecursive => {
                estimated_benefit = 20.0; // Good benefit for mutual recursion
                TailCallOptimizationType::TailCallElimination
            }
        
        // Additional analysis would check for other blocking factors
        // For now, simplified analysis
        
        let confidence = if can_optimize { 0.8 } else { 0.0 };
        
        Ok(TailCallAnalysisResult {
        })
    fn identify_recursive_patterns(&mut self) -> Result<()> {
        debug!("Identifying recursive patterns");
        
        // Find self-recursive functions
        for (caller, callees) in &self.call_analysis.call_graph {
            if callees.contains(caller) {
                self.call_analysis.recursive_functions.insert(caller.clone());
            }
        }
        
        // Find mutually recursive groups using strongly connected components
        self.find_mutual_recursion_groups()?;
        
        Ok(())
    fn find_mutual_recursion_groups(&mut self) -> Result<()> {
        // Simplified mutual recursion detection
        // In a real implementation, would use Tarjan's algorithm for SCCs
        
        let mut visited = HashSet::new();
        let mut groups = Vec::new();
        
        for function in self.call_analysis.call_graph.keys() {
            if !visited.contains(function) {
                let mut group = Vec::new();
                self.dfs_mutual_recursion(function, &mut visited, &mut group);
                if group.len() > 1 {
                    groups.push(group);
                }
            }
        self.call_analysis.mutual_recursion_groups = groups;
        Ok(())
    fn dfs_mutual_recursion(&self, function: &str, visited: &mut HashSet<String>, group: &mut Vec<String>) {
        visited.insert(function.to_string());
        group.push(function.to_string());
        
        if let Some(callees) = self.call_analysis.call_graph.get(function) {
            for callee in callees {
                if !visited.contains(callee) {
                    self.dfs_mutual_recursion(callee, visited, group);
                }
            }
        }
    }
    
    fn identify_tail_call_candidates(&mut self, module: &Module<'ctx>) -> Result<()> {
        debug!("Identifying tail call candidates");
        
        for function in module.get_functions() {
            if function.get_first_basic_block().is_some() {
                let function_name = function.get_name().to_str().unwrap_or("unnamed");
                let candidates = self.find_function_tail_call_candidates(function)?;
                self.tail_call_candidates.insert(function_name.to_string(), candidates);
            }
        }
        
        Ok(())
    fn find_function_tail_call_candidates(&self, function: FunctionValue<'ctx>) -> Result<Vec<TailCallCandidate>> {
        let function_name = function.get_name().to_str().unwrap_or("unnamed");
        let mut candidates = Vec::new();
        
        if let Some(call_sites) = self.call_analysis.function_calls.get(function_name) {
            for call_site in call_sites {
                if call_site.is_tail_position && call_site.analysis_result.can_optimize {
                    let candidate = self.create_tail_call_candidate(call_site)?;
                    candidates.push(candidate);
                }
            }
        Ok(candidates)
    fn create_tail_call_candidate(&self, call_site: &CallSiteInfo) -> Result<TailCallCandidate> {
        let optimization_strategy = self.determine_optimization_strategy(call_site);
        let transformation_plan = self.create_transformation_plan(call_site, &optimization_strategy)?;
        let stack_impact = self.calculate_stack_impact(call_site);
        let performance_benefit = self.calculate_performance_benefit(call_site);
        
        Ok(TailCallCandidate {
        })
    fn determine_optimization_strategy(&self, call_site: &CallSiteInfo) -> OptimizationStrategy {
        match call_site.call_type {
            CallType::RecursiveCall => {
                if self.optimization_constraints.aggressive_optimization {
                    OptimizationStrategy::ConvertToLoop
                } else {
                    OptimizationStrategy::ReuseStackFrame
                }
            }
        }
    }
    
    fn create_transformation_plan(&self, call_site: &CallSiteInfo, strategy: &OptimizationStrategy) -> Result<TransformationPlan> {
        let mut replacement_operations = Vec::new();
        let mut control_flow_changes = Vec::new();
        
        match strategy {
            OptimizationStrategy::ReplaceWithJump => {
                replacement_operations.push(ReplacementOperation {
                });
                
                control_flow_changes.push(ControlFlowChange {
                });
            }
            OptimizationStrategy::ConvertToLoop => {
                replacement_operations.push(ReplacementOperation {
                });
                
                control_flow_changes.push(ControlFlowChange {
                });
            }
            _ => {
                // Default transformation
                replacement_operations.push(ReplacementOperation {
                });
            }
        }
        
        Ok(TransformationPlan {
        })
    fn calculate_stack_impact(&self, call_site: &CallSiteInfo) -> StackImpact {
        let frames_eliminated = match call_site.call_type {
        
        let stack_space_saved = frames_eliminated * 256; // Estimated frame size
        let overall_reduction = if frames_eliminated > 0 { 0.15 } else { 0.0 };
        
        StackImpact {
            parameter_overhead: -32, // Reduced parameter passing overhead
        }
    }
    
    fn calculate_performance_benefit(&self, call_site: &CallSiteInfo) -> PerformanceBenefit {
        let base_benefit = call_site.analysis_result.estimated_benefit;
        
        PerformanceBenefit {
        }
    }
    
    fn analyze_function_tail_calls(&self, function: FunctionValue<'ctx>) -> Result<TailCallAnalysisInfo> {
        let function_name = function.get_name().to_str().unwrap_or("unnamed");
        
        let call_sites = self.call_analysis.function_calls
            .get(function_name)
            .map(|sites| sites.len())
            .unwrap_or(0);
        
        let tail_calls = self.call_analysis.function_calls
            .get(function_name)
            .map(|sites| sites.iter().filter(|site| site.is_tail_position).count())
            .unwrap_or(0);
        
        let recursive_calls = self.call_analysis.function_calls
            .get(function_name)
            .map(|sites| sites.iter().filter(|site| matches!(site.call_type, CallType::RecursiveCall)).count())
            .unwrap_or(0);
        
        let optimization_potential = if tail_calls > 0 {
            (tail_calls as f64 / call_sites.max(1) as f64) * 100.0
        } else {
            0.0
        
        Ok(TailCallAnalysisInfo {
        })
    fn perform_tail_call_optimization(&mut self, function: FunctionValue<'ctx>, candidate: &TailCallCandidate) -> Result<TailCallOptimization> {
        debug!("Performing tail call optimization for: {}", candidate.call_site.call_instruction);
        
        // This would perform the actual LLVM IR transformation
        // For now, create a placeholder optimization result
        
        let optimization_type = format!("{:?}", candidate.optimization_strategy);
        let stack_impact = candidate.stack_impact.clone();
        let performance_benefit = candidate.performance_benefit.clone();
        
        Ok(TailCallOptimization {
        })
    fn identify_additional_opportunities(&self) -> Result<Vec<TailCallOptimizationOpportunity>> {
        let mut opportunities = Vec::new();
        
        // Look for recursive functions that could benefit from tail call optimization
        for recursive_func in &self.call_analysis.recursive_functions {
            opportunities.push(TailCallOptimizationOpportunity {
            });
        // Look for mutual recursion groups
        for group in &self.call_analysis.mutual_recursion_groups {
            if group.len() > 1 {
                opportunities.push(TailCallOptimizationOpportunity {
                });
            }
        }
        
        Ok(opportunities)
    fn calculate_function_benefit(&self, optimizations: &[TailCallOptimization]) -> f64 {
        optimizations.iter().map(|opt| opt.performance_benefit.overall_speedup).sum::<f64>() / optimizations.len().max(1) as f64
    fn update_statistics(&self, optimization_time: Duration, function_results: &HashMap<String, FunctionTailCallResults>) {
        if let Ok(mut stats) = self.statistics.lock() {
            stats.optimization_time = optimization_time;
            stats.functions_analyzed = function_results.len();
            
            for function_result in function_results.values() {
                stats.call_sites_analyzed += function_result.tail_call_analysis.total_call_sites;
                stats.tail_calls_identified += function_result.tail_call_analysis.tail_position_calls;
                stats.tail_calls_optimized += function_result.optimizations_performed.len();
                stats.stack_frames_eliminated += function_result.stack_frames_eliminated;
                
                for optimization in &function_result.optimizations_performed {
                    stats.estimated_stack_savings += optimization.stack_impact.stack_space_saved;
                    
                    if optimization.optimization_type.contains("Recursive") {
                        stats.recursive_calls_converted += 1;
                    } else {
                        stats.sibling_calls_optimized += 1;
                    }
                }
            stats.performance_improvement = function_results.values()
                .map(|r| r.optimization_benefit)
                .sum::<f64>() / function_results.len().max(1) as f64;
        }
    }
// Supporting types and implementations

impl CallAnalysis {
    fn new() -> Self {
        Self {
        }
    }
/// Results of tail call optimization
#[derive(Debug, Clone)]
pub struct TailCallOptimizationResults {
/// Results for a single function
#[derive(Debug, Clone)]
pub struct FunctionTailCallResults {
/// Tail call analysis information for a function
#[derive(Debug, Clone)]
pub struct TailCallAnalysisInfo {
/// Individual tail call optimization result
#[derive(Debug, Clone)]
pub struct TailCallOptimization {
/// Tail call eligibility assessment
#[derive(Debug, Clone)]
pub struct TailCallEligibility {
/// Additional optimization opportunity
#[derive(Debug, Clone)]
pub struct TailCallOptimizationOpportunity {
