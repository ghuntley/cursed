use crate::error::{CursedError, Result};
use inkwell::values::{FunctionValue, InstructionValue, BasicValueEnum, InstructionOpcode, BasicValue};
use inkwell::context::Context;
use inkwell::module::Module;
use std::collections::{HashSet, VecDeque};

/// Dead code elimination pass for CURSED
pub struct DeadCodeEliminationPass<'ctx> {
    context: &'ctx Context,
    aggressive: bool,
}

impl<'ctx> DeadCodeEliminationPass<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        Self { 
            context,
            aggressive: false,
        }
    }

    pub fn with_aggressive(mut self, aggressive: bool) -> Self {
        self.aggressive = aggressive;
        self
    }

    /// Eliminate dead code in a function
    pub fn eliminate_dead_code(&self, function: &FunctionValue<'ctx>) -> Result<bool> {
        let mut changed = false;
        
        // Collect all instructions
        let mut all_instructions = Vec::new();
        for block in function.get_basic_blocks() {
            for instruction in block.get_instructions() {
                all_instructions.push(instruction);
            }
        }
        
        // Find instructions that have no users and no side effects
        let mut dead_instructions = Vec::new();
        for instruction in &all_instructions {
            if self.is_dead_instruction(instruction) {
                dead_instructions.push(*instruction);
            }
        }
        
        // Remove dead instructions
        for instruction in dead_instructions {
            unsafe {
                instruction.erase_from_basic_block();
            }
            changed = true;
        }
        
        Ok(changed)
    }

    /// Check if an instruction is dead
    fn is_dead_instruction(&self, instruction: &InstructionValue<'ctx>) -> bool {
        // Don't eliminate instructions with side effects
        if self.has_side_effects(instruction) {
            return false;
        }
        
        // Check if instruction has any uses
        let has_uses = instruction.get_first_use().is_some();
        
        // If no uses and no side effects, it's dead
        !has_uses
    }
    
    /// Check if an instruction has side effects
    fn has_side_effects(&self, instruction: &InstructionValue<'ctx>) -> bool {
        match instruction.get_opcode() {
            // Instructions with side effects that should not be removed
            inkwell::values::InstructionOpcode::Store |
            inkwell::values::InstructionOpcode::Call |
            inkwell::values::InstructionOpcode::Invoke |
            inkwell::values::InstructionOpcode::Return |
            inkwell::values::InstructionOpcode::Br |
            inkwell::values::InstructionOpcode::Switch |
            inkwell::values::InstructionOpcode::IndirectBr |
            inkwell::values::InstructionOpcode::Resume |
            inkwell::values::InstructionOpcode::Unreachable |
            inkwell::values::InstructionOpcode::Fence |
            inkwell::values::InstructionOpcode::AtomicCmpXchg |
            inkwell::values::InstructionOpcode::AtomicRMW => true,
            
            // Loads can be eliminated if they don't escape
            inkwell::values::InstructionOpcode::Load => !self.aggressive,
            
            // Pure instructions can usually be eliminated
            inkwell::values::InstructionOpcode::Add |
            inkwell::values::InstructionOpcode::Sub |
            inkwell::values::InstructionOpcode::Mul |
            inkwell::values::InstructionOpcode::SDiv |
            inkwell::values::InstructionOpcode::UDiv |
            inkwell::values::InstructionOpcode::SRem |
            inkwell::values::InstructionOpcode::URem |
            inkwell::values::InstructionOpcode::And |
            inkwell::values::InstructionOpcode::Or |
            inkwell::values::InstructionOpcode::Xor |
            inkwell::values::InstructionOpcode::Shl |
            inkwell::values::InstructionOpcode::LShr |
            inkwell::values::InstructionOpcode::AShr |
            inkwell::values::InstructionOpcode::ICmp |
            inkwell::values::InstructionOpcode::FCmp |
            inkwell::values::InstructionOpcode::ZExt |
            inkwell::values::InstructionOpcode::SExt |
            inkwell::values::InstructionOpcode::Trunc |
            inkwell::values::InstructionOpcode::BitCast |
            inkwell::values::InstructionOpcode::GetElementPtr |
            inkwell::values::InstructionOpcode::Phi |
            inkwell::values::InstructionOpcode::Select => false,
            
            // Conservative: assume other instructions have side effects
            _ => true,
        }
    }

    pub fn run(&self, module: &Module<'ctx>) -> Result<DeadCodeResult> {
        let mut result = DeadCodeResult::default();
        
        for function in module.get_functions() {
            if function.get_first_basic_block().is_none() {
                continue; // Skip external functions
            }
            
            let function_result = self.run_on_function(function)?;
            result.merge(function_result);
        }
        
        Ok(result)
    }
    
    /// Run DCE on a single function
    fn run_on_function(&self, function: FunctionValue<'ctx>) -> Result<FunctionDceResult> {
        let mut result = FunctionDceResult::default();
        
        // Iteratively remove dead code until no more changes
        let mut iterations = 0;
        const MAX_ITERATIONS: u32 = 10;
        
        while iterations < MAX_ITERATIONS {
            iterations += 1;
            
            let changed = self.eliminate_dead_code(&function)?;
            if !changed {
                break;
            }
            
            result.instructions_eliminated += 1; // This is a rough estimate
        }
        
        result.iterations = iterations;
        
        Ok(result)
    }
}

/// Dead code analyzer - real implementation
pub struct DeadCodeAnalyzer<'ctx> {
    context: &'ctx Context,
    live_instructions: std::collections::HashSet<InstructionValue<'ctx>>,
    work_list: Vec<InstructionValue<'ctx>>,
}

impl<'ctx> DeadCodeAnalyzer<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        Self { 
            context,
            live_instructions: std::collections::HashSet::new(),
            work_list: Vec::new(),
        }
    }
    
    /// Analyze function and identify dead code
    pub fn analyze_function(&mut self, function: FunctionValue<'ctx>) -> crate::error::Result<DeadCodeResult> {
        self.live_instructions.clear();
        self.work_list.clear();
        
        let mut result = DeadCodeResult::default();
        
        // Mark critical instructions as live (those with side effects)
        self.mark_critical_instructions(function);
        
        // Propagate liveness backwards
        while let Some(inst) = self.work_list.pop() {
            self.mark_operands_live(inst);
        }
        
        // Remove dead instructions
        let eliminated = self.remove_dead_instructions(function)?;
        result.total_eliminated = eliminated;
        result.functions_processed = 1;
        result.iterations_total = 1;
        
        Ok(result)
    }
    
    /// Mark instructions with side effects as critical
    fn mark_critical_instructions(&mut self, function: FunctionValue<'ctx>) {
        for basic_block in function.get_basic_blocks() {
            for instruction in basic_block.get_instructions() {
                if self.is_critical_instruction(&instruction) {
                    self.mark_live(instruction);
                }
            }
        }
    }
    
    /// Check if instruction has side effects and cannot be eliminated
    fn is_critical_instruction(&self, instruction: &InstructionValue<'ctx>) -> bool {
        match instruction.get_opcode() {
            // Function calls might have side effects
            InstructionOpcode::Call => true,
            // Store operations have side effects
            InstructionOpcode::Store => true,
            // Return statements are critical
            InstructionOpcode::Return => true,
            // Branch instructions are critical for control flow
            InstructionOpcode::Br => true,
            // Switch statements
            InstructionOpcode::Switch => true,
            // Unreachable blocks
            InstructionOpcode::Unreachable => true,
            // Most other instructions can potentially be eliminated
            _ => false,
        }
    }
    
    /// Mark instruction and its operands as live
    fn mark_live(&mut self, instruction: InstructionValue<'ctx>) {
        if self.live_instructions.insert(instruction) {
            self.work_list.push(instruction);
        }
    }
    
    /// Mark all operands of an instruction as live
    fn mark_operands_live(&mut self, instruction: InstructionValue<'ctx>) {
        for i in 0..instruction.get_num_operands() {
            if let Some(operand) = instruction.get_operand(i) {
                if let Some(inst_operand) = operand.left().and_then(|v| v.as_instruction_value()) {
                    self.mark_live(inst_operand);
                }
            }
        }
    }
    
    /// Remove instructions that are not marked as live
    fn remove_dead_instructions(&self, function: FunctionValue<'ctx>) -> crate::error::Result<u32> {
        let mut eliminated = 0;
        
        for basic_block in function.get_basic_blocks() {
            let mut instructions_to_remove = Vec::new();
            
            for instruction in basic_block.get_instructions() {
                if !self.live_instructions.contains(&instruction) && !self.is_critical_instruction(&instruction) {
                    instructions_to_remove.push(instruction);
                }
            }
            
            // Remove dead instructions
            for instruction in instructions_to_remove {
                unsafe {
                    instruction.erase_from_basic_block();
                }
                eliminated += 1;
            }
        }
        
        Ok(eliminated)
    }
}

/// Dead code result type
#[derive(Debug, Clone, Default)]
pub struct DeadCodeResult {
    pub total_eliminated: u32,
    pub functions_processed: u32,
    pub iterations_total: u32,
}

impl DeadCodeResult {
    pub fn total_eliminated(&self) -> u32 {
        self.total_eliminated
    }
    
    pub fn merge(&mut self, other: FunctionDceResult) {
        self.total_eliminated += other.instructions_eliminated;
        self.functions_processed += 1;
        self.iterations_total += other.iterations;
    }
}

/// Result of DCE on a single function
#[derive(Debug, Default)]
pub struct FunctionDceResult {
    pub instructions_eliminated: u32,
    pub iterations: u32,
}
