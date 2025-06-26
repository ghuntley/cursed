//! Dead code elimination optimization pass

use crate::error::{CursedError, Result};
use inkwell::{
    context::Context,
    module::{Module, Linkage},
    values::{FunctionValue, BasicValueEnum, InstructionValue},
    basic_block::BasicBlock,
};
use std::collections::{HashSet, HashMap, VecDeque};

/// Dead code elimination pass for CURSED
pub struct DeadCodeEliminationPass<'ctx> {
    context: &'ctx Context,
    aggressive: bool,
    preserve_debug_info: bool,
}

impl<'ctx> DeadCodeEliminationPass<'ctx> {
    /// Create a new dead code elimination pass
    pub fn new(context: &'ctx Context, aggressive: bool) -> Self {
        Self {
            context,
            aggressive,
            preserve_debug_info: true,
        }
    }
    
    /// Enable or disable debug info preservation
    pub fn with_debug_preservation(mut self, preserve: bool) -> Self {
        self.preserve_debug_info = preserve;
        self
    }
    
    /// Run dead code elimination on a module
    pub fn run(&mut self, module: &Module<'ctx>) -> Result<DeadCodeResult> {
        let mut result = DeadCodeResult::default();
        
        // Global dead code elimination
        result.dead_globals = self.eliminate_dead_globals(module)?;
        
        // Function-level dead code elimination
        for function in module.get_functions() {
            if function.get_first_basic_block().is_none() {
                continue; // Skip external functions
            }
            
            let function_result = self.eliminate_dead_code_in_function(function)?;
            result.merge_function_result(function_result);
        }
        
        // Inter-procedural dead code elimination
        if self.aggressive {
            result.dead_functions = self.eliminate_dead_functions(module)?;
        }
        
        Ok(result)
    }
    
    /// Eliminate dead global variables
    fn eliminate_dead_globals(&self, module: &Module<'ctx>) -> Result<u32> {
        let mut removed_count = 0;
        let mut globals_to_remove = Vec::new();
        
        // Collect potentially dead globals
        for global in module.get_globals() {
            if self.is_global_dead(&global) {
                globals_to_remove.push(global);
            }
        }
        
        // Remove dead globals
        for global in globals_to_remove {
            if self.can_safely_remove_global(&global) {
                unsafe {
                    global.delete();
                }
                removed_count += 1;
            }
        }
        
        Ok(removed_count)
    }
    
    /// Check if a global variable is dead
    fn is_global_dead(&self, global: &inkwell::values::GlobalValue) -> bool {
        // A global is dead if it has no uses
        global.get_first_use().is_none()
    }
    
    /// Check if it's safe to remove a global
    fn can_safely_remove_global(&self, global: &inkwell::values::GlobalValue) -> bool {
        // Don't remove globals with external linkage unless aggressive
        if !self.aggressive {
            match global.get_linkage() {
                Linkage::External |
                Linkage::ExternalWeak => return false,
                _ => {}
            }
        }
        
        // Don't remove globals with special names (like constructors)
        if let Ok(name) = global.get_name().to_str() {
            if name.starts_with("llvm.") || name.contains("ctor") || name.contains("dtor") {
                return false;
            }
        }
        
        true
    }
    
    /// Eliminate dead code within a function
    fn eliminate_dead_code_in_function(&self, function: FunctionValue<'ctx>) -> Result<FunctionDeadCodeResult> {
        let mut result = FunctionDeadCodeResult::default();
        
        // Eliminate dead instructions
        result.dead_instructions = self.eliminate_dead_instructions(function)?;
        
        // Eliminate unreachable basic blocks
        result.dead_blocks = self.eliminate_unreachable_blocks(function)?;
        
        // Simplify control flow
        if self.aggressive {
            result.simplified_branches = self.simplify_control_flow(function)?;
        }
        
        Ok(result)
    }
    
    /// Eliminate dead instructions using mark-and-sweep
    fn eliminate_dead_instructions(&self, function: FunctionValue<'ctx>) -> Result<u32> {
        let mut analyzer = DeadCodeAnalyzer::new(function);
        analyzer.mark_live_instructions(self.preserve_debug_info);
        
        let dead_instructions = analyzer.find_dead_instructions();
        let mut removed_count = 0;
        
        // Remove dead instructions in reverse order to avoid invalidating references
        for instruction in dead_instructions.iter().rev() {
            if self.can_safely_remove_instruction(instruction) {
                unsafe {
                    instruction.delete();
                }
                removed_count += 1;
            }
        }
        
        Ok(removed_count)
    }
    
    /// Check if an instruction can be safely removed
    fn can_safely_remove_instruction(&self, instruction: &InstructionValue) -> bool {
        match instruction.get_opcode() {
            // These instructions can always be removed if dead
            inkwell::values::InstructionOpcode::Add |
            inkwell::values::InstructionOpcode::Sub |
            inkwell::values::InstructionOpcode::Mul |
            inkwell::values::InstructionOpcode::SDiv |
            inkwell::values::InstructionOpcode::UDiv |
            inkwell::values::InstructionOpcode::And |
            inkwell::values::InstructionOpcode::Or |
            inkwell::values::InstructionOpcode::Xor |
            inkwell::values::InstructionOpcode::Shl |
            inkwell::values::InstructionOpcode::LShr |
            inkwell::values::InstructionOpcode::AShr |
            inkwell::values::InstructionOpcode::Load |
            inkwell::values::InstructionOpcode::GetElementPtr |
            inkwell::values::InstructionOpcode::BitCast |
            inkwell::values::InstructionOpcode::ZExt |
            inkwell::values::InstructionOpcode::SExt |
            inkwell::values::InstructionOpcode::Trunc |
            inkwell::values::InstructionOpcode::ICmp |
            inkwell::values::InstructionOpcode::FCmp => true,
            
            // These instructions have side effects and shouldn't be removed
            inkwell::values::InstructionOpcode::Store |
            inkwell::values::InstructionOpcode::Call |
            inkwell::values::InstructionOpcode::Invoke |
            inkwell::values::InstructionOpcode::Ret |
            inkwell::values::InstructionOpcode::Br |
            inkwell::values::InstructionOpcode::CondBr |
            inkwell::values::InstructionOpcode::Switch => false,
            
            // Debug instructions can be removed if not preserving debug info
            inkwell::values::InstructionOpcode::Call => {
                if !self.preserve_debug_info {
                    // Check if it's a debug intrinsic
                    if let Some(call_site) = instruction.as_call_site_value() {
                        if let Some(called_value) = call_site.try_as_basic_value().left() {
                            if let Some(function) = called_value.as_function_value() {
                                if let Ok(name) = function.get_name().to_str() {
                                    return name.starts_with("llvm.dbg.");
                                }
                            }
                        }
                    }
                }
                false
            }
            
            _ => false,
        }
    }
    
    /// Eliminate unreachable basic blocks
    fn eliminate_unreachable_blocks(&self, function: FunctionValue<'ctx>) -> Result<u32> {
        let reachable_blocks = self.find_reachable_blocks(function);
        let mut removed_count = 0;
        
        let all_blocks: Vec<_> = function.get_basic_blocks().collect();
        
        for block in all_blocks {
            if !reachable_blocks.contains(&block) {
                // Remove all instructions in the block first
                let instructions: Vec<_> = block.get_instructions().collect();
                for instruction in instructions {
                    unsafe {
                        instruction.delete();
                    }
                }
                
                // Remove the block
                unsafe {
                    block.delete();
                }
                removed_count += 1;
            }
        }
        
        Ok(removed_count)
    }
    
    /// Find all reachable basic blocks using BFS
    fn find_reachable_blocks(&self, function: FunctionValue<'ctx>) -> HashSet<BasicBlock<'ctx>> {
        let mut reachable = HashSet::new();
        let mut worklist = VecDeque::new();
        
        // Start from the entry block
        if let Some(entry_block) = function.get_first_basic_block() {
            worklist.push_back(entry_block);
            reachable.insert(entry_block);
        }
        
        while let Some(block) = worklist.pop_front() {
            // Find successors of this block
            if let Some(terminator) = block.get_terminator() {
                for successor in self.get_instruction_successors(&terminator) {
                    if !reachable.contains(&successor) {
                        reachable.insert(successor);
                        worklist.push_back(successor);
                    }
                }
            }
        }
        
        reachable
    }
    
    /// Get successor blocks from a terminator instruction
    fn get_instruction_successors(&self, instruction: &InstructionValue<'ctx>) -> Vec<BasicBlock<'ctx>> {
        let mut successors = Vec::new();
        
        match instruction.get_opcode() {
            inkwell::values::InstructionOpcode::Br => {
                // Unconditional branch
                if let Some(target) = instruction.get_operand(0) {
                    if let Some(target_block) = target.left().and_then(|v| v.as_basic_block()) {
                        successors.push(target_block);
                    }
                }
            }
            inkwell::values::InstructionOpcode::CondBr => {
                // Conditional branch
                if let Some(true_target) = instruction.get_operand(2) {
                    if let Some(true_block) = true_target.left().and_then(|v| v.as_basic_block()) {
                        successors.push(true_block);
                    }
                }
                if let Some(false_target) = instruction.get_operand(1) {
                    if let Some(false_block) = false_target.left().and_then(|v| v.as_basic_block()) {
                        successors.push(false_block);
                    }
                }
            }
            inkwell::values::InstructionOpcode::Switch => {
                // Switch instruction
                for i in (1..instruction.get_num_operands()).step_by(2) {
                    if let Some(target) = instruction.get_operand(i) {
                        if let Some(target_block) = target.left().and_then(|v| v.as_basic_block()) {
                            successors.push(target_block);
                        }
                    }
                }
            }
            _ => {}
        }
        
        successors
    }
    
    /// Simplify control flow by removing unnecessary branches
    fn simplify_control_flow(&self, function: FunctionValue<'ctx>) -> Result<u32> {
        let mut simplified_count = 0;
        
        for block in function.get_basic_blocks() {
            if let Some(terminator) = block.get_terminator() {
                if self.simplify_branch(&terminator)? {
                    simplified_count += 1;
                }
            }
        }
        
        Ok(simplified_count)
    }
    
    /// Simplify a branch instruction
    fn simplify_branch(&self, instruction: &InstructionValue<'ctx>) -> Result<bool> {
        match instruction.get_opcode() {
            inkwell::values::InstructionOpcode::CondBr => {
                // Check for branches with constant conditions
                if let Some(condition) = instruction.get_operand(0) {
                    if let Some(const_val) = condition.left() {
                        if let Some(int_val) = const_val.as_int_value() {
                            // Convert conditional branch to unconditional
                            let builder = self.context.create_builder();
                            builder.position_before(instruction);
                            
                            if int_val.get_zero_extended_constant().unwrap_or(0) != 0 {
                                // Branch to true target
                                if let Some(true_target) = instruction.get_operand(2) {
                                    if let Some(true_block) = true_target.left().and_then(|v| v.as_basic_block()) {
                                        builder.build_unconditional_branch(true_block).unwrap();
                                        unsafe {
                                            instruction.delete();
                                        }
                                        return Ok(true);
                                    }
                                }
                            } else {
                                // Branch to false target
                                if let Some(false_target) = instruction.get_operand(1) {
                                    if let Some(false_block) = false_target.left().and_then(|v| v.as_basic_block()) {
                                        builder.build_unconditional_branch(false_block).unwrap();
                                        unsafe {
                                            instruction.delete();
                                        }
                                        return Ok(true);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        
        Ok(false)
    }
    
    /// Eliminate dead functions
    fn eliminate_dead_functions(&self, module: &Module<'ctx>) -> Result<u32> {
        let mut removed_count = 0;
        let mut functions_to_remove = Vec::new();
        
        // Find dead functions
        for function in module.get_functions() {
            if self.is_function_dead(&function) {
                functions_to_remove.push(function);
            }
        }
        
        // Remove dead functions
        for function in functions_to_remove {
            unsafe {
                function.delete();
            }
            removed_count += 1;
        }
        
        Ok(removed_count)
    }
    
    /// Check if a function is dead
    fn is_function_dead(&self, function: &FunctionValue) -> bool {
        // Don't remove main function
        if let Ok(name) = function.get_name().to_str() {
            if name == "main" {
                return false;
            }
        }
        
        // Don't remove external functions unless aggressive
        if function.get_first_basic_block().is_none() && !self.aggressive {
            return false;
        }
        
        // Function is dead if it has no uses
        function.get_first_use().is_none()
    }
}

/// Analyzer for finding dead code within a function
pub struct DeadCodeAnalyzer<'ctx> {
    function: FunctionValue<'ctx>,
    live_instructions: HashSet<InstructionValue<'ctx>>,
    worklist: VecDeque<InstructionValue<'ctx>>,
}

impl<'ctx> DeadCodeAnalyzer<'ctx> {
    pub fn new(function: FunctionValue<'ctx>) -> Self {
        Self {
            function,
            live_instructions: HashSet::new(),
            worklist: VecDeque::new(),
        }
    }
    
    /// Mark all live instructions using a worklist algorithm
    pub fn mark_live_instructions(&mut self, preserve_debug: bool) {
        // Start with instructions that must be kept alive
        self.mark_critical_instructions(preserve_debug);
        
        // Propagate liveness backwards
        while let Some(instruction) = self.worklist.pop_front() {
            self.mark_instruction_operands(&instruction);
        }
    }
    
    /// Mark instructions that are critical and must be kept
    fn mark_critical_instructions(&mut self, preserve_debug: bool) {
        for block in self.function.get_basic_blocks() {
            for instruction in block.get_instructions() {
                if self.is_critical_instruction(&instruction, preserve_debug) {
                    self.mark_live(&instruction);
                }
            }
        }
    }
    
    /// Check if an instruction is critical (has side effects or is control flow)
    fn is_critical_instruction(&self, instruction: &InstructionValue, preserve_debug: bool) -> bool {
        match instruction.get_opcode() {
            // Control flow instructions are always critical
            inkwell::values::InstructionOpcode::Ret |
            inkwell::values::InstructionOpcode::Br |
            inkwell::values::InstructionOpcode::CondBr |
            inkwell::values::InstructionOpcode::Switch |
            inkwell::values::InstructionOpcode::IndirectBr => true,
            
            // Instructions with side effects are critical
            inkwell::values::InstructionOpcode::Store |
            inkwell::values::InstructionOpcode::Call |
            inkwell::values::InstructionOpcode::Invoke => true,
            
            // Debug instructions are critical if preserving debug info
            inkwell::values::InstructionOpcode::Call if preserve_debug => {
                if let Some(call_site) = instruction.as_call_site_value() {
                    if let Some(called_value) = call_site.try_as_basic_value().left() {
                        if let Some(function) = called_value.as_function_value() {
                            if let Ok(name) = function.get_name().to_str() {
                                return name.starts_with("llvm.dbg.");
                            }
                        }
                    }
                }
                false
            }
            
            _ => false,
        }
    }
    
    /// Mark an instruction as live and add it to the worklist
    fn mark_live(&mut self, instruction: &InstructionValue<'ctx>) {
        if self.live_instructions.insert(*instruction) {
            self.worklist.push_back(*instruction);
        }
    }
    
    /// Mark all operands of an instruction as live
    fn mark_instruction_operands(&mut self, instruction: &InstructionValue<'ctx>) {
        for operand in instruction.get_operands() {
            if let Some(value) = operand.left() {
                if let Some(inst_value) = value.as_instruction_value() {
                    self.mark_live(&inst_value);
                }
            }
        }
    }
    
    /// Find all dead instructions
    pub fn find_dead_instructions(&self) -> Vec<InstructionValue<'ctx>> {
        let mut dead_instructions = Vec::new();
        
        for block in self.function.get_basic_blocks() {
            for instruction in block.get_instructions() {
                if !self.live_instructions.contains(&instruction) {
                    dead_instructions.push(instruction);
                }
            }
        }
        
        dead_instructions
    }
}

/// Result of dead code elimination
#[derive(Debug, Default)]
pub struct DeadCodeResult {
    pub dead_instructions: u32,
    pub dead_blocks: u32,
    pub dead_functions: u32,
    pub dead_globals: u32,
    pub simplified_branches: u32,
}

impl DeadCodeResult {
    fn merge_function_result(&mut self, result: FunctionDeadCodeResult) {
        self.dead_instructions += result.dead_instructions;
        self.dead_blocks += result.dead_blocks;
        self.simplified_branches += result.simplified_branches;
    }
    
    pub fn total_eliminated(&self) -> u32 {
        self.dead_instructions + self.dead_blocks + self.dead_functions + self.dead_globals
    }
}

/// Result of dead code elimination within a function
#[derive(Debug, Default)]
struct FunctionDeadCodeResult {
    pub dead_instructions: u32,
    pub dead_blocks: u32,
    pub simplified_branches: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    
    #[test]
    fn test_dead_code_analyzer() {
        let context = Context::create();
        let module = context.create_module("test");
        let function_type = context.void_type().fn_type(&[], false);
        let function = module.add_function("test_func", function_type, None);
        
        let analyzer = DeadCodeAnalyzer::new(function);
        assert!(analyzer.live_instructions.is_empty());
    }
    
    #[test]
    fn test_dead_code_result() {
        let mut result = DeadCodeResult::default();
        result.dead_instructions = 5;
        result.dead_blocks = 2;
        
        assert_eq!(result.total_eliminated(), 7);
    }
    
    #[test]
    fn test_dead_code_elimination_pass() {
        let context = Context::create();
        let pass = DeadCodeEliminationPass::new(&context, true);
        assert!(pass.aggressive);
        assert!(pass.preserve_debug_info);
    }
}
