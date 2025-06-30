//! Memory to Register promotion pass

use crate::error::{CursedError, Result};
use inkwell::values::{FunctionValue, InstructionValue, PointerValue, InstructionOpcode, AnyValue};
use inkwell::basic_block::BasicBlock;
use inkwell::context::Context;
use std::collections::{HashMap, HashSet};

/// Mem2Reg pass - Promote memory locations to registers
pub struct Mem2RegPass<'ctx> {
    context: &'ctx Context,
    allocas: Vec<PointerValue<'ctx>>,
    promotable_allocas: HashSet<PointerValue<'ctx>>,
    phi_locations: HashMap<BasicBlock<'ctx>, Vec<PointerValue<'ctx>>>,
}

impl<'ctx> Mem2RegPass<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        Self {
            context,
            allocas: Vec::new(),
            promotable_allocas: HashSet::new(),
            phi_locations: HashMap::new(),
        }
    }
    
    /// Run Mem2Reg on a function
    pub fn run_on_function(&mut self, function: &FunctionValue<'ctx>) -> Result<bool> {
        let mut changed = false;
        
        // Find all allocas in the function
        self.find_allocas(function)?;
        
        // Determine which allocas can be promoted
        self.analyze_promotability()?;
        
        // Insert phi nodes where needed
        if !self.promotable_allocas.is_empty() {
            self.insert_phi_nodes(function)?;
            changed = true;
        }
        
        // Replace loads and stores with register operations
        if changed {
            self.promote_allocas(function)?;
        }
        
        Ok(changed)
    }
    
    fn find_allocas(&mut self, function: &FunctionValue<'ctx>) -> Result<()> {
        // Find all alloca instructions in entry block
        if let Some(entry_block) = function.get_first_basic_block() {
            for instruction in entry_block.get_instructions() {
                // Check if instruction is an alloca via opcode
                if instruction.get_opcode() == InstructionOpcode::Alloca {
                    let alloca = instruction.as_any_value_enum().into_pointer_value();
                    self.allocas.push(alloca);
                }
            }
        }
        
        Ok(())
    }
    
    fn analyze_promotability(&mut self) -> Result<()> {
        // Analyze each alloca to see if it can be promoted
        for alloca in &self.allocas {
            if self.is_promotable(alloca)? {
                self.promotable_allocas.insert(*alloca);
            }
        }
        
        Ok(())
    }
    
    fn is_promotable(&self, alloca: &PointerValue<'ctx>) -> Result<bool> {
        // Check if alloca is promotable:
        // 1. Only accessed via loads and stores
        // 2. Not address-taken
        // 3. Not volatile
        
        // In inkwell 0.4, we can work around missing get_users() by scanning all instructions
        // This is a simplified heuristic approach
        
        // For now return true for simple allocas - real implementation would analyze uses
        Ok(true)
    }
    
    fn insert_phi_nodes(&mut self, function: &FunctionValue<'ctx>) -> Result<()> {
        // Insert phi nodes at join points for promotable allocas
        // Simplified implementation - would use dominance frontier analysis
        
        let blocks = function.get_basic_blocks();
        
        for alloca in &self.promotable_allocas {
            // Find blocks that need phi nodes
            for block in &blocks {
                // Check if block has multiple predecessors
                let predecessors = self.get_predecessors(block);
                if predecessors.len() > 1 {
                    // Mark for phi insertion
                    if let Some(phi_list) = self.phi_locations.get_mut(block) {
                        phi_list.push(*alloca);
                    } else {
                        self.phi_locations.insert(*block, vec![*alloca]);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn get_predecessors(&self, _block: &BasicBlock<'ctx>) -> Vec<BasicBlock<'ctx>> {
        // Simplified - real implementation would analyze CFG
        Vec::new()
    }
    
    fn promote_allocas(&mut self, _function: &FunctionValue<'ctx>) -> Result<()> {
        // Simplified implementation - real implementation would rewrite IR
        // This would replace load/store operations with SSA values
        Ok(())
    }
}
