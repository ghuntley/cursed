//! Tail call optimization pass

use crate::error::{CursedError, Result};
use inkwell::values::{FunctionValue, InstructionValue, InstructionOpcode};
use inkwell::context::Context;
use std::collections::HashSet;

/// Tail call optimization pass
pub struct TailCallPass<'ctx> {
    context: &'ctx Context,
    tail_calls: Vec<InstructionValue<'ctx>>,
    optimizable_calls: HashSet<InstructionValue<'ctx>>,
}

impl<'ctx> TailCallPass<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        Self {
            context,
            tail_calls: Vec::new(),
            optimizable_calls: HashSet::new(),
        }
    }
    
    /// Run tail call optimization on a function
    pub fn run_on_function(&mut self, function: &FunctionValue<'ctx>) -> Result<bool> {
        let mut changed = false;
        
        // Find potential tail calls
        self.find_tail_calls(function)?;
        
        // Analyze which calls can be optimized
        self.analyze_tail_calls()?;
        
        // Apply tail call optimization
        if !self.optimizable_calls.is_empty() {
            self.optimize_tail_calls()?;
            changed = true;
        }
        
        Ok(changed)
    }
    
    fn find_tail_calls(&mut self, function: &FunctionValue<'ctx>) -> Result<()> {
        // Find call instructions that are immediately followed by return
        for block in function.get_basic_blocks() {
            let instructions: Vec<_> = block.get_instructions().collect();
            
            for i in 0..instructions.len() {
                let instruction = &instructions[i];
                
                // Check if this is a call instruction
                if let inkwell::values::InstructionOpcode::Call = instruction.get_opcode() {
                    // Check if the next instruction is a return
                    if self.is_tail_call(&instructions, i) {
                        self.tail_calls.push(*instruction);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn is_tail_call(&self, instructions: &[InstructionValue<'ctx>], call_index: usize) -> bool {
        // Check if call is immediately followed by return (possibly with some cleanup)
        
        for i in (call_index + 1)..instructions.len() {
            let instruction = &instructions[i];
            
            match instruction.get_opcode() {
                inkwell::values::InstructionOpcode::Return => {
                    // Found return - this is a tail call
                    return true;
                },
                inkwell::values::InstructionOpcode::Store => {
                    // Store might be cleanup, continue checking
                    continue;
                },
                inkwell::values::InstructionOpcode::Load => {
                    // Load might be for return value, continue checking
                    continue;
                },
                _ => {
                    // Other instructions mean this is not a tail call
                    return false;
                }
            }
        }
        
        false
    }
    
    fn analyze_tail_calls(&mut self) -> Result<()> {
        // Analyze each tail call to see if it can be optimized
        for tail_call in &self.tail_calls {
            if self.can_optimize_tail_call(tail_call)? {
                self.optimizable_calls.insert(*tail_call);
            }
        }
        
        Ok(())
    }
    
    fn can_optimize_tail_call(&self, call_instruction: &InstructionValue<'ctx>) -> Result<bool> {
        // Check if tail call can be optimized:
        // 1. Return type matches function return type
        // 2. No exception handling in between
        // 3. No stack frame dependencies
        
        // Simplified check - in practice would be more thorough
        // as_call_site() doesn't exist in inkwell 0.4, check opcode instead
        if call_instruction.get_opcode() == InstructionOpcode::Call {
            // Check if we're calling the same function (self-recursion is most optimizable)
            // Simplified - cannot get called function without proper call site
            // For self-recursion, tail call optimization is usually beneficial
            return Ok(true);
        }
        
        // For other calls, be conservative
        Ok(false)
    }
    
    fn optimize_tail_calls(&mut self) -> Result<()> {
        // Apply tail call optimization to optimizable calls
        for call_instruction in &self.optimizable_calls {
            self.apply_tail_call_optimization(call_instruction)?;
        }
        
        Ok(())
    }
    
    fn apply_tail_call_optimization(&self, call_instruction: &InstructionValue<'ctx>) -> Result<()> {
        // Apply tail call optimization
        // This is a placeholder - in practice would:
        // 1. Mark the call as a tail call in LLVM IR
        // 2. Potentially transform to a jump for self-recursion
        // 3. Ensure proper calling convention
        
        // as_call_site() doesn't exist in inkwell 0.4, check opcode instead
        if call_instruction.get_opcode() == InstructionOpcode::Call {
            // In practice, we'd set the tail call attribute
            // call_site.set_tail_call(true);
            // This is architecture and ABI dependent
        }
        
        Ok(())
    }
}
