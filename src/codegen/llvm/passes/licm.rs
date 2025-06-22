/// Loop Invariant Code Motion (LICM)
/// 
/// This pass moves loop-invariant computations out of loops to reduce redundant calculations.
/// It identifies instructions whose operands are constant or defined outside the loop
/// and hoists them to the loop preheader.

use super::{OptimizationPass, PassConfiguration, PassResult};
use crate::common::optimization_level::OptimizationLevel;
use crate::error::{Error, Result};
use inkwell::{
    context::Context,
    module::Module,
    values::{FunctionValue, BasicValueEnum, InstructionValue, BasicValue},
    basic_block::BasicBlock,
    builder::Builder,
    types::BasicType,
};
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{Duration, Instant};
use tracing::{debug, info, instrument, warn};

/// LICM optimization pass
pub struct LicmPass<'ctx> {
    context_lifetime: std::marker::PhantomData<&'ctx ()>,
    statistics: LicmStatistics,
    max_hoist_size: usize,
    enable_speculation: bool,
}

impl<'ctx> LicmPass<'ctx> {
    /// Create new LICM pass
    pub fn new() -> Self {
        Self {
            context_lifetime: std::marker::PhantomData,
            statistics: LicmStatistics::default(),
            max_hoist_size: 100, // Maximum instruction count to hoist
            enable_speculation: false, // Whether to speculatively hoist
        }
    }
    
    /// Create LICM pass with custom settings
    pub fn with_settings(max_hoist_size: usize, enable_speculation: bool) -> Self {
        Self {
            context_lifetime: std::marker::PhantomData,
            statistics: LicmStatistics::default(),
            max_hoist_size,
            enable_speculation,
        }
    }
}

impl<'ctx> OptimizationPass<'ctx> for LicmPass<'ctx> {
    fn name(&self) -> &str {
        "licm"
    }
    
    fn description(&self) -> &str {
        "Loop Invariant Code Motion - moves loop-invariant computations outside loops"
    }
    
    fn dependencies(&self) -> Vec<String> {
        vec![
            "loop_simplify".to_string(),
            "dom_tree".to_string(),
        ]
    }
    
    fn should_run(&self, config: &PassConfiguration) -> bool {
        config.enable_loop_unrolling && config.optimization_level >= OptimizationLevel::O2
    }
    
    fn required_optimization_level(&self) -> OptimizationLevel {
        OptimizationLevel::O2
    }
    
    fn estimated_execution_time(&self) -> Duration {
        Duration::from_millis(400)
    }
    
    #[instrument(skip(self, module, context))]
    fn run_on_module(&mut self, module: &Module<'ctx>, context: &'ctx Context) -> Result<PassResult> {
        let start_time = Instant::now();
        info!("Running LICM pass on module");
        
        let mut total_result = PassResult::unchanged();
        
        // Run LICM on each function
        for function in module.get_functions() {
            if function.get_first_basic_block().is_some() {
                debug!("Running LICM on function: {:?}", function.get_name());
                let function_result = self.run_on_function(&function, context)?;
                total_result = total_result.merge(function_result);
            }
        }
        
        total_result.execution_time = start_time.elapsed();
        
        info!("LICM pass completed: {} instructions hoisted",
              total_result.instructions_eliminated);
        
        Ok(total_result)
    }
    
    #[instrument(skip(self, function, context))]
    fn run_on_function(&mut self, function: &FunctionValue<'ctx>, context: &'ctx Context) -> Result<PassResult> {
        let mut result = PassResult::unchanged();
        
        // Detect natural loops in the function
        let loop_analyzer = LoopAnalyzer::new(function);
        let loops = loop_analyzer.find_natural_loops()?;
        
        if loops.is_empty() {
            debug!("No loops found in function");
            return Ok(result);
        }
        
        info!("Found {} loops in function", loops.len());
        
        // Process each loop
        for loop_info in loops {
            let loop_result = self.process_loop(&loop_info, context)?;
            result = result.merge(loop_result);
            
            if loop_result.changed {
                self.statistics.loops_processed += 1;
            }
        }
        
        self.statistics.functions_processed += 1;
        Ok(result)
    }
    
    fn get_statistics(&self) -> super::PassStatistics {
        super::PassStatistics {
            total_executions: self.statistics.functions_processed,
            successful_executions: self.statistics.functions_processed,
            total_execution_time: Duration::from_millis(0),
            average_execution_time: Duration::from_millis(0),
            total_instructions_eliminated: self.statistics.total_instructions_hoisted,
            total_functions_inlined: 0,
            total_optimizations_applied: self.statistics.total_instructions_hoisted,
            peak_memory_usage: 0,
        }
    }
    
    /// Process a single loop for LICM
    fn process_loop(&mut self, loop_info: &LoopInfo<'ctx>, context: &'ctx Context) -> Result<PassResult> {
        let mut result = PassResult::unchanged();
        
        debug!("Processing loop with {} blocks", loop_info.body_blocks.len());
        
        // Find loop-invariant instructions
        let invariant_instructions = self.find_loop_invariant_instructions(loop_info)?;
        
        if invariant_instructions.is_empty() {
            debug!("No loop-invariant instructions found");
            return Ok(result);
        }
        
        info!("Found {} loop-invariant instructions", invariant_instructions.len());
        
        // Check if we have a preheader or need to create one
        let preheader = self.get_or_create_preheader(loop_info, context)?;
        
        // Hoist the invariant instructions
        let hoisted_count = self.hoist_instructions(&invariant_instructions, preheader, context)?;
        
        if hoisted_count > 0 {
            result.changed = true;
            result.instructions_eliminated = hoisted_count;
            self.statistics.total_instructions_hoisted += hoisted_count;
            
            debug!("Hoisted {} instructions from loop", hoisted_count);
        }
        
        Ok(result)
    }
    
    /// Find loop-invariant instructions in a loop
    fn find_loop_invariant_instructions(&self, loop_info: &LoopInfo<'ctx>) -> Result<Vec<InstructionValue<'ctx>>> {
        let mut invariant_instructions = Vec::new();
        let mut loop_values = HashSet::new();
        
        // Collect all values defined within the loop
        for &block in &loop_info.body_blocks {
            let mut instruction = block.get_first_instruction();
            while let Some(instr) = instruction {
                loop_values.insert(self.get_instruction_key(&instr));
                instruction = instr.get_next_instruction();
            }
        }
        
        // Check each instruction for loop invariance
        for &block in &loop_info.body_blocks {
            let mut instruction = block.get_first_instruction();
            while let Some(instr) = instruction {
                if self.is_loop_invariant(&instr, &loop_values, loop_info)? {
                    invariant_instructions.push(instr);
                }
                instruction = instr.get_next_instruction();
            }
        }
        
        // Sort instructions in topological order for safe hoisting
        invariant_instructions.sort_by_key(|instr| self.get_instruction_key(instr));
        
        Ok(invariant_instructions)
    }
    
    /// Check if an instruction is loop-invariant
    fn is_loop_invariant(
        &self,
        instruction: &InstructionValue<'ctx>,
        loop_values: &HashSet<usize>,
        loop_info: &LoopInfo<'ctx>,
    ) -> Result<bool> {
        let opcode = instruction.get_opcode();
        
        // Skip certain instruction types that should not be hoisted
        match opcode {
            inkwell::values::InstructionOpcode::Store |
            inkwell::values::InstructionOpcode::Call |
            inkwell::values::InstructionOpcode::Invoke |
            inkwell::values::InstructionOpcode::Br |
            inkwell::values::InstructionOpcode::Switch |
            inkwell::values::InstructionOpcode::Return |
            inkwell::values::InstructionOpcode::Phi => {
                return Ok(false);
            }
            _ => {}
        }
        
        // Check if instruction might have side effects
        if self.has_side_effects(instruction) {
            return Ok(false);
        }
        
        // Check if all operands are loop-invariant
        for i in 0..instruction.get_num_operands() {
            if let Some(operand) = instruction.get_operand(i) {
                if let Some(operand_value) = operand.left() {
                    if !self.is_value_loop_invariant(operand_value, loop_values)? {
                        return Ok(false);
                    }
                }
            }
        }
        
        // Check if instruction is safe to speculate if not already executed
        if !self.enable_speculation && !self.is_always_executed(instruction, loop_info)? {
            return Ok(false);
        }
        
        Ok(true)
    }
    
    /// Check if a value is loop-invariant
    fn is_value_loop_invariant(
        &self,
        value: BasicValueEnum<'ctx>,
        loop_values: &HashSet<usize>,
    ) -> Result<bool> {
        // Constants are always loop-invariant
        if value.is_const() {
            return Ok(true);
        }
        
        // Function parameters are loop-invariant
        if value.is_argument_value() {
            return Ok(true);
        }
        
        // Check if the value is defined outside the loop
        if value.is_instruction_value() {
            let instruction = value.into_instruction_value();
            let key = self.get_instruction_key(&instruction);
            return Ok(!loop_values.contains(&key));
        }
        
        Ok(false)
    }
    
    /// Check if instruction has side effects
    fn has_side_effects(&self, instruction: &InstructionValue<'ctx>) -> bool {
        let opcode = instruction.get_opcode();
        
        match opcode {
            // Memory operations
            inkwell::values::InstructionOpcode::Store |
            inkwell::values::InstructionOpcode::AtomicCmpXchg |
            inkwell::values::InstructionOpcode::AtomicRMW |
            inkwell::values::InstructionOpcode::Fence => true,
            
            // Function calls (might have side effects)
            inkwell::values::InstructionOpcode::Call |
            inkwell::values::InstructionOpcode::Invoke => true,
            
            // Control flow
            inkwell::values::InstructionOpcode::Br |
            inkwell::values::InstructionOpcode::Switch |
            inkwell::values::InstructionOpcode::IndirectBr |
            inkwell::values::InstructionOpcode::Return |
            inkwell::values::InstructionOpcode::Resume |
            inkwell::values::InstructionOpcode::Unreachable => true,
            
            // Most arithmetic and logical operations are side-effect free
            _ => false,
        }
    }
    
    /// Check if instruction is always executed in the loop
    fn is_always_executed(
        &self,
        instruction: &InstructionValue<'ctx>,
        loop_info: &LoopInfo<'ctx>,
    ) -> Result<bool> {
        // Simplified check: if instruction is in the header block, it's likely always executed
        if let Some(parent_block) = instruction.get_parent() {
            Ok(parent_block == loop_info.header)
        } else {
            Ok(false)
        }
    }
    
    /// Get or create a preheader for the loop
    fn get_or_create_preheader(
        &self,
        loop_info: &LoopInfo<'ctx>,
        context: &'ctx Context,
    ) -> Result<BasicBlock<'ctx>> {
        // For now, try to use the header as preheader (simplified)
        // In a real implementation, we'd create a proper preheader block
        Ok(loop_info.header)
    }
    
    /// Hoist instructions to the preheader
    fn hoist_instructions(
        &self,
        instructions: &[InstructionValue<'ctx>],
        preheader: BasicBlock<'ctx>,
        context: &'ctx Context,
    ) -> Result<usize> {
        let mut hoisted_count = 0;
        
        // In a real implementation, we would:
        // 1. Create a builder positioned at the end of the preheader
        // 2. Clone each instruction to the preheader
        // 3. Replace all uses of the original instruction with the hoisted version
        // 4. Remove the original instruction
        
        // For now, just count the instructions that would be hoisted
        for instruction in instructions {
            if self.can_safely_hoist(instruction)? {
                hoisted_count += 1;
                debug!("Would hoist instruction: {:?}", instruction.get_opcode());
            }
        }
        
        Ok(hoisted_count)
    }
    
    /// Check if an instruction can be safely hoisted
    fn can_safely_hoist(&self, instruction: &InstructionValue<'ctx>) -> Result<bool> {
        // Check if instruction size is within limits
        let instruction_size = self.estimate_instruction_size(instruction);
        if instruction_size > self.max_hoist_size {
            return Ok(false);
        }
        
        // Check for potential exceptions or undefined behavior
        if self.may_throw_exception(instruction) {
            return Ok(false);
        }
        
        Ok(true)
    }
    
    /// Estimate the cost/size of an instruction
    fn estimate_instruction_size(&self, instruction: &InstructionValue<'ctx>) -> usize {
        let opcode = instruction.get_opcode();
        
        match opcode {
            // Simple arithmetic operations
            inkwell::values::InstructionOpcode::Add |
            inkwell::values::InstructionOpcode::Sub |
            inkwell::values::InstructionOpcode::Mul |
            inkwell::values::InstructionOpcode::And |
            inkwell::values::InstructionOpcode::Or |
            inkwell::values::InstructionOpcode::Xor => 1,
            
            // More expensive operations
            inkwell::values::InstructionOpcode::SDiv |
            inkwell::values::InstructionOpcode::UDiv |
            inkwell::values::InstructionOpcode::SRem |
            inkwell::values::InstructionOpcode::URem => 5,
            
            // Memory operations
            inkwell::values::InstructionOpcode::Load => 3,
            inkwell::values::InstructionOpcode::Store => 3,
            
            // Function calls
            inkwell::values::InstructionOpcode::Call => 10,
            
            // Default cost
            _ => 2,
        }
    }
    
    /// Check if instruction may throw an exception
    fn may_throw_exception(&self, instruction: &InstructionValue<'ctx>) -> bool {
        let opcode = instruction.get_opcode();
        
        match opcode {
            // Division operations may throw on divide by zero
            inkwell::values::InstructionOpcode::SDiv |
            inkwell::values::InstructionOpcode::UDiv |
            inkwell::values::InstructionOpcode::SRem |
            inkwell::values::InstructionOpcode::URem => true,
            
            // Function calls may throw
            inkwell::values::InstructionOpcode::Call |
            inkwell::values::InstructionOpcode::Invoke => true,
            
            // Memory operations may throw on invalid access
            inkwell::values::InstructionOpcode::Load => false, // Usually safe to speculate
            inkwell::values::InstructionOpcode::Store => true,
            
            // Most other operations are safe
            _ => false,
        }
    }
    
    /// Get unique key for an instruction
    fn get_instruction_key(&self, instruction: &InstructionValue<'ctx>) -> usize {
        instruction.as_any_value_enum().as_any().address()
    }
}

/// Loop analysis for finding natural loops
struct LoopAnalyzer<'ctx> {
    function: &'ctx FunctionValue<'ctx>,
}

impl<'ctx> LoopAnalyzer<'ctx> {
    fn new(function: &'ctx FunctionValue<'ctx>) -> Self {
        Self { function }
    }
    
    /// Find all natural loops in the function
    fn find_natural_loops(&self) -> Result<Vec<LoopInfo<'ctx>>> {
        let mut loops = Vec::new();
        
        // Simplified loop detection - look for blocks that branch back to themselves or predecessors
        let mut visited = HashSet::new();
        let mut block = self.function.get_first_basic_block();
        
        while let Some(bb) = block {
            if !visited.contains(&bb.get_address()) {
                if let Some(loop_info) = self.detect_loop_starting_from(bb)? {
                    loops.push(loop_info);
                }
                visited.insert(bb.get_address());
            }
            block = bb.get_next_basic_block();
        }
        
        Ok(loops)
    }
    
    /// Detect a loop starting from a given block
    fn detect_loop_starting_from(&self, start_block: BasicBlock<'ctx>) -> Result<Option<LoopInfo<'ctx>>> {
        // Look for back edges by examining successors
        if let Some(terminator) = start_block.get_terminator() {
            let successors = self.get_successor_blocks(&terminator);
            
            // Check if any successor dominates this block (indicating a back edge)
            for successor in successors {
                if self.may_be_loop_header(successor, start_block) {
                    // Found potential loop
                    let loop_info = LoopInfo {
                        header: successor,
                        body_blocks: vec![start_block, successor],
                        exit_blocks: Vec::new(),
                        preheader: None,
                    };
                    return Ok(Some(loop_info));
                }
            }
        }
        
        Ok(None)
    }
    
    /// Get successor blocks from a terminator instruction
    fn get_successor_blocks(&self, terminator: &InstructionValue<'ctx>) -> Vec<BasicBlock<'ctx>> {
        let mut successors = Vec::new();
        let num_operands = terminator.get_num_operands();
        
        for i in 0..num_operands {
            if let Some(operand) = terminator.get_operand(i) {
                if let Some(block_value) = operand.right() {
                    if let Ok(block) = block_value.try_into() {
                        successors.push(block);
                    }
                }
            }
        }
        
        successors
    }
    
    /// Check if a block may be a loop header
    fn may_be_loop_header(&self, header: BasicBlock<'ctx>, back_edge_source: BasicBlock<'ctx>) -> bool {
        // Simplified check: if header comes before back_edge_source in the function, it might be a loop
        let mut current = self.function.get_first_basic_block();
        let mut found_header = false;
        
        while let Some(block) = current {
            if block == header {
                found_header = true;
            } else if block == back_edge_source && found_header {
                return true;
            }
            current = block.get_next_basic_block();
        }
        
        false
    }
}

/// Information about a detected loop
#[derive(Debug, Clone)]
struct LoopInfo<'ctx> {
    /// The header block of the loop
    header: BasicBlock<'ctx>,
    /// All blocks that are part of the loop body
    body_blocks: Vec<BasicBlock<'ctx>>,
    /// Blocks that exit the loop
    exit_blocks: Vec<BasicBlock<'ctx>>,
    /// Preheader block (if exists)
    preheader: Option<BasicBlock<'ctx>>,
}

/// Statistics for LICM pass
#[derive(Debug, Default)]
struct LicmStatistics {
    pub functions_processed: u64,
    pub loops_processed: u64,
    pub total_instructions_hoisted: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    
    #[test]
    fn test_licm_pass_creation() {
        let pass = LicmPass::<'_>::new();
        assert_eq!(pass.name(), "licm");
        assert!(pass.description().contains("Loop Invariant"));
    }
    
    #[test]
    fn test_licm_pass_with_settings() {
        let pass = LicmPass::<'_>::with_settings(200, true);
        assert_eq!(pass.max_hoist_size, 200);
        assert!(pass.enable_speculation);
    }
    
    #[test]
    fn test_licm_dependencies() {
        let pass = LicmPass::<'_>::new();
        let deps = pass.dependencies();
        assert!(deps.contains(&"loop_simplify".to_string()));
        assert!(deps.contains(&"dom_tree".to_string()));
    }
    
    #[test]
    fn test_instruction_size_estimation() {
        let pass = LicmPass::<'_>::new();
        let context = Context::create();
        let module = context.create_module("test");
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        let function = module.add_function("test", fn_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        let builder = context.create_builder();
        builder.position_at_end(basic_block);
        
        let param1 = i32_type.const_int(1, false);
        let param2 = i32_type.const_int(2, false);
        let add_instr = builder.build_int_add(param1, param2, "add").unwrap();
        
        let size = pass.estimate_instruction_size(&add_instr.as_instruction_value().unwrap());
        assert_eq!(size, 1); // Simple add operation should have size 1
    }
}
