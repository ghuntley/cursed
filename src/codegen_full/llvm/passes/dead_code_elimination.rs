
/// Dead Code Elimination Pass
/// 
/// Removes unused functions, unreachable basic blocks, and dead instructions
/// to reduce code size and improve performance.

use super::{OptimizationPass, PassConfiguration, PassResult, PassStatistics};
use crate::common_types::optimization_level::OptimizationLevel;
use crate::error::{CursedError, Result};
use inkwell::{
// };

use std::collections::{HashSet, HashMap, VecDeque};
use std::time::Instant;
use tracing::{debug, info, instrument, warn};

/// Dead code elimination optimization pass
pub struct DeadCodeEliminationPass {
impl DeadCodeEliminationPass {
    /// Create a new dead code elimination pass
    pub fn new(config: PassConfiguration) -> Self {
        let aggressive_mode = config.optimization_level >= OptimizationLevel::O3;
        
        Self {
        }
    }
    
    /// Eliminate dead functions from the module
    #[instrument(skip(self, module))]
    fn eliminate_dead_functions(&mut self, module: &Module) -> Result<usize> {
        let mut functions_to_remove = Vec::new();
        let mut live_functions = HashSet::new();
        
        // Find all exported functions and entry points
        for function in module.get_functions() {
            let function_name = function.get_name().to_str().unwrap_or("");
            
            // Keep main functions, exported functions, and externally visible functions
            if function_name == "main" || 
               function_name.starts_with("cursed_") ||
               function.get_linkage().is_external() ||
               !function.get_uses().is_empty() {
                live_functions.insert(function.get_name());
                debug!("Marking function as live: {}", function_name);
            }
        }
        
        // Mark functions called by live functions
        let mut worklist: VecDeque<_> = live_functions.iter().cloned().collect();
        while let Some(function_name) = worklist.pop_front() {
            if let Some(function) = module.get_function(&function_name.to_string()) {
                for basic_block in function.get_basic_blocks() {
                    for instruction in basic_block.get_instructions() {
                        if let Some(call_site) = self.get_call_target(&instruction) {
                            if live_functions.insert(call_site.clone()) {
                                worklist.push_back(call_site);
                                debug!("Marking called function as live: {}", call_site.to_str().unwrap_or(""));
                            }
                        }
                    }
                }
            }
        }
        
        // Collect dead functions
        for function in module.get_functions() {
            if !live_functions.contains(&function.get_name()) {
                let function_name = function.get_name().to_str().unwrap_or("<unnamed>");
                debug!("Found dead function: {}", function_name);
                functions_to_remove.push(function);
            }
        }
        
        // Remove dead functions
        let removed_count = functions_to_remove.len();
        for function in functions_to_remove {
            unsafe {
                function.delete();
            }
        }
        
        if removed_count > 0 {
            info!("Eliminated {} dead functions", removed_count);
        Ok(removed_count)
    /// Eliminate unreachable basic blocks within functions
    #[instrument(skip(self, function))]
    fn eliminate_unreachable_blocks(&mut self, function: &FunctionValue) -> Result<usize> {
        let mut reachable_blocks = HashSet::new();
        let mut worklist = VecDeque::new();
        
        // Start from the entry block
        if let Some(entry_block) = function.get_first_basic_block() {
            reachable_blocks.insert(entry_block.get_address());
            worklist.push_back(entry_block);
        // Mark all reachable blocks
        while let Some(current_block) = worklist.pop_front() {
            // Find successor blocks
            if let Some(terminator) = current_block.get_terminator() {
                for successor in self.get_successor_blocks(&terminator) {
                    if reachable_blocks.insert(successor.get_address()) {
                        worklist.push_back(successor);
                    }
                }
            }
        }
        
        // Collect unreachable blocks
        let mut unreachable_blocks = Vec::new();
        for basic_block in function.get_basic_blocks() {
            if !reachable_blocks.contains(&basic_block.get_address()) {
                unreachable_blocks.push(basic_block);
            }
        }
        
        // Remove unreachable blocks
        let removed_count = unreachable_blocks.len();
        for block in unreachable_blocks {
            let block_name = block.get_name().to_str().unwrap_or("<unnamed>");
            debug!("Removing unreachable block: {}", block_name);
            unsafe {
                block.remove_from_function();
            }
        }
        
        if removed_count > 0 {
                   function.get_name().to_str().unwrap_or("<unnamed>"));
        Ok(removed_count)
    /// Eliminate dead instructions within basic blocks
    #[instrument(skip(self, function))]
    fn eliminate_dead_instructions(&mut self, function: &FunctionValue) -> Result<usize> {
        let mut removed_count = 0;
        
        for basic_block in function.get_basic_blocks() {
            let mut instructions_to_remove = Vec::new();
            
            for instruction in basic_block.get_instructions() {
                if self.is_dead_instruction(&instruction) {
                    instructions_to_remove.push(instruction);
                }
            }
            
            // Remove dead instructions
            for instruction in instructions_to_remove {
                debug!("Removing dead instruction: {:?}", instruction.get_opcode());
                unsafe {
                    instruction.remove_from_basic_block();
                }
                removed_count += 1;
            }
        }
        
        if removed_count > 0 {
                   function.get_name().to_str().unwrap_or("<unnamed>"));
        Ok(removed_count)
    /// Check if an instruction is dead (has no uses and no side effects)
    fn is_dead_instruction(&self, instruction: &InstructionValue) -> bool {
        // Don't remove instructions with side effects
        if self.has_side_effects(instruction) {
            return false;
        // Check if the instruction has any uses
        instruction.get_uses().is_empty()
    /// Check if an instruction has side effects
    fn has_side_effects(&self, instruction: &InstructionValue) -> bool {
        use inkwell::values::InstructionOpcode;
        
        match instruction.get_opcode() {
            // Memory operations
            InstructionOpcode::Store |
            InstructionOpcode::AtomicRMW |
            InstructionOpcode::AtomicCmpXchg |
            
            // Function calls (may have side effects)
            InstructionOpcode::Call |
            
            // Control flow
            InstructionOpcode::Return |
            InstructionOpcode::Branch |
            InstructionOpcode::Switch |
            InstructionOpcode::IndirectBr |
            InstructionOpcode::Resume |
            
            // Exception handling
            InstructionOpcode::LandingPad |
            InstructionOpcode::CleanupRet |
            
            // Volatile loads
            InstructionOpcode::Load => {
                // In a real implementation, we'd check if the load is volatile
                false
            // Most arithmetic and logical operations don't have side effects
        }
    }
    
    /// Get the call target from a call instruction
    fn get_call_target(&self, instruction: &InstructionValue) -> Option<inkwell::values::StringValue> {
        use inkwell::values::InstructionOpcode;
        
        if instruction.get_opcode() == InstructionOpcode::Call {
            // Get the called function
            if let Some(called_value) = instruction.get_operand(instruction.get_num_operands() - 1) {
                if let BasicValueEnum::PointerValue(ptr) = called_value {
                    // In a real implementation, we'd extract the function name from the pointer
                    // For now, we'll return None to be safe
                }
            }
        None
    /// Get successor basic blocks from a terminator instruction
    fn get_successor_blocks(&self, terminator: &InstructionValue) -> Vec<BasicBlock> {
        use inkwell::values::InstructionOpcode;
        let mut successors = Vec::new();
        
        match terminator.get_opcode() {
            InstructionOpcode::Branch => {
                // Conditional or unconditional branch
                for i in 0..terminator.get_num_operands() {
                    if let Some(operand) = terminator.get_operand(i) {
                        if let BasicValueEnum::PointerValue(_ptr) = operand {
                            // In a real implementation, we'd extract the basic block from the pointer
                            // For now, we'll skip this to avoid unsafe operations
                        }
                    }
                }
            }
            InstructionOpcode::Switch => {
                // Switch instruction has multiple successors
                for i in 1..terminator.get_num_operands() {
                    if let Some(operand) = terminator.get_operand(i) {
                        if let BasicValueEnum::PointerValue(_ptr) = operand {
                            // In a real implementation, we'd extract the basic block from the pointer
                        }
                    }
                }
            }
            _ => {}
        }
        
        successors
    /// Perform aggressive dead code elimination optimizations
    #[instrument(skip(self, module))]
    fn aggressive_optimization(&mut self, module: &Module) -> Result<usize> {
        if !self.aggressive_mode {
            return Ok(0);
        debug!("Running aggressive dead code elimination");
        
        let mut total_eliminated = 0;
        
        // Multiple passes to catch code that becomes dead after other eliminations
        for pass_number in 0..3 {
            debug!("Aggressive DCE pass {}", pass_number + 1);
            
            let mut pass_eliminated = 0;
            
            // Eliminate dead functions
            pass_eliminated += self.eliminate_dead_functions(module)?;
            
            // Eliminate dead code in remaining functions
            for function in module.get_functions() {
                pass_eliminated += self.eliminate_unreachable_blocks(&function)?;
                pass_eliminated += self.eliminate_dead_instructions(&function)?;
            total_eliminated += pass_eliminated;
            
            // If no progress was made, we're done
            if pass_eliminated == 0 {
                debug!("No more dead code found, stopping aggressive passes");
                break;
            }
        }
        
        if total_eliminated > 0 {
            info!("Aggressive DCE eliminated {} additional items", total_eliminated);
        Ok(total_eliminated)
    }
}

impl<'ctx> OptimizationPass<'ctx> for DeadCodeEliminationPass {
    fn name(&self) -> &str {
        "dead_code_elimination"
    fn description(&self) -> &str {
        "Removes unused functions, unreachable basic blocks, and dead instructions"
    fn should_run(&self, config: &PassConfiguration) -> bool {
        config.enable_dead_code_elimination && 
        config.optimization_level >= OptimizationLevel::O1
    fn required_optimization_level(&self) -> OptimizationLevel {
        OptimizationLevel::O1
    #[instrument(skip(self, module, context))]
    fn run_on_module(&mut self, module: &Module<'ctx>, _context: &'ctx Context) -> Result<PassResult> {
        let start_time = Instant::now();
        
        info!("Running dead code elimination pass");
        
        let mut result = PassResult::unchanged();
        let mut total_eliminated = 0;
        
        // Eliminate dead functions
        let dead_functions = self.eliminate_dead_functions(module)?;
        total_eliminated += dead_functions;
        
        // Eliminate dead code within functions
        for function in module.get_functions() {
            let dead_blocks = self.eliminate_unreachable_blocks(&function)?;
            let dead_instructions = self.eliminate_dead_instructions(&function)?;
            
            total_eliminated += dead_blocks + dead_instructions;
        // Run aggressive optimizations if enabled
        if self.aggressive_mode {
            let aggressive_eliminated = self.aggressive_optimization(module)?;
            total_eliminated += aggressive_eliminated;
        // Update result
        if total_eliminated > 0 {
            result.changed = true;
            result.instructions_eliminated = total_eliminated;
        result.execution_time = start_time.elapsed();
        result.metrics.insert("dead_functions_eliminated".to_string(), dead_functions as f64);
        result.metrics.insert("total_eliminated".to_string(), total_eliminated as f64);
        
        // Update statistics
        self.statistics.update(&result);
        
              total_eliminated, result.execution_time);
        
        Ok(result)
    fn get_statistics(&self) -> PassStatistics {
        self.statistics.clone()
    fn reset(&mut self) {
        self.statistics = PassStatistics::default();
    }
}

/// Analyzer for identifying dead code patterns
pub struct DeadCodeAnalyzer {
impl DeadCodeAnalyzer {
    /// Create a new dead code analyzer
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// Analyze a module to identify dead code patterns
    pub fn analyze_module(&mut self, module: &Module) -> DeadCodeAnalysisResult {
        let mut result = DeadCodeAnalysisResult::default();
        
        // Build function call graph
        for function in module.get_functions() {
            let function_name = function.get_name().to_str().unwrap_or("").to_string();
            let mut called_functions = HashSet::new();
            
            for basic_block in function.get_basic_blocks() {
                for instruction in basic_block.get_instructions() {
                    if let Some(called_function) = self.extract_called_function(&instruction) {
                        called_functions.insert(called_function);
                    }
                }
            self.function_call_graph.insert(function_name.clone(), called_functions);
            
            // Analyze function for dead code
            let function_analysis = self.analyze_function(&function);
            result.function_analyses.insert(function_name, function_analysis);
        // Identify dead functions
        result.dead_functions = self.find_dead_functions();
        
        result
    /// Analyze a single function for dead code
    fn analyze_function(&mut self, function: &FunctionValue) -> FunctionDeadCodeAnalysis {
        let mut analysis = FunctionDeadCodeAnalysis::default();
        
        // Count instruction uses
        for basic_block in function.get_basic_blocks() {
            for instruction in basic_block.get_instructions() {
                let instruction_id = format!("{:p}", instruction.as_value_ref());
                analysis.total_instructions += 1;
                
                if instruction.get_uses().is_empty() && !self.has_side_effects_analyzer(&instruction) {
                    analysis.dead_instructions.push(instruction_id.clone());
                self.instruction_use_counts.insert(instruction_id, instruction.get_uses().count());
            }
        }
        
        analysis.dead_instruction_count = analysis.dead_instructions.len();
        analysis
    /// Find dead functions using call graph analysis
    fn find_dead_functions(&self) -> Vec<String> {
        let mut dead_functions = Vec::new();
        let mut live_functions = HashSet::new();
        
        // Start with entry points
        for (function_name, _) in &self.function_call_graph {
            if function_name == "main" || function_name.starts_with("cursed_") {
                live_functions.insert(function_name.clone());
            }
        }
        
        // Mark transitively called functions as live
        let mut changed = true;
        while changed {
            changed = false;
            let mut new_live_functions = live_functions.clone();
            
            for live_function in &live_functions {
                if let Some(called_functions) = self.function_call_graph.get(live_function) {
                    for called_function in called_functions {
                        if new_live_functions.insert(called_function.clone()) {
                            changed = true;
                        }
                    }
                }
            }
            
            live_functions = new_live_functions;
        // Find dead functions
        for function_name in self.function_call_graph.keys() {
            if !live_functions.contains(function_name) {
                dead_functions.push(function_name.clone());
            }
        }
        
        dead_functions
    /// Extract called function name from a call instruction
    fn extract_called_function(&self, _instruction: &InstructionValue) -> Option<String> {
        // In a real implementation, this would extract the function name
        // from call instructions. For now, we'll return None.
        None
    /// Check if instruction has side effects (analyzer version)
    fn has_side_effects_analyzer(&self, instruction: &InstructionValue) -> bool {
        use inkwell::values::InstructionOpcode;
        
            InstructionOpcode::Store |
            InstructionOpcode::Call |
            InstructionOpcode::Invoke |
            InstructionOpcode::Return |
            InstructionOpcode::Branch |
            InstructionOpcode::AtomicRMW |
            InstructionOpcode::AtomicCmpXchg |
            InstructionOpcode::Fence
        )
    }
}

/// Result of dead code analysis
#[derive(Debug, Default)]
pub struct DeadCodeAnalysisResult {
/// Dead code analysis for a single function
#[derive(Debug, Default)]
pub struct FunctionDeadCodeAnalysis {
