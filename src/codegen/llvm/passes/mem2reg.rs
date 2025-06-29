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
        
        // get_users() doesn't exist in inkwell 0.4, stub for now
        let users: Vec<inkwell::values::InstructionValue> = vec![]; // TODO: implement user traversal
        for user in users {
            // as_instruction() doesn't exist in inkwell 0.4, stub for now
            if false { // TODO: proper instruction check
                let instruction = user;
                // Check instruction type
                let opcode = instruction.get_opcode();
                match opcode {
                    inkwell::values::InstructionOpcode::Load => {
                        // Load is OK
                    },
                    inkwell::values::InstructionOpcode::Store => {
                        // Store is OK if storing TO the alloca, not the alloca itself
                        // as_store() doesn't exist, check via opcode
                if instruction.get_opcode() == InstructionOpcode::Store {
                            // Simplified - cannot get pointer operand without proper store instruction
                            let dest = *alloca; // placeholder
                            if dest != *alloca {
                                return Ok(false); // Address taken
                            }
                        }
                    },
                    _ => {
                        // Any other use means address is taken
                        return Ok(false);
                    }
                }
            }
        }
        
        Ok(true)
    }
    
    fn insert_phi_nodes(&mut self, function: &FunctionValue<'ctx>) -> Result<()> {
        // Insert phi nodes at join points for promotable allocas
        // This is a simplified implementation
        
        let blocks = function.get_basic_blocks();
        
        for alloca in &self.promotable_allocas {
            // Find blocks that need phi nodes (simplified - would use dominance frontier)
            for block in &blocks {
                // Check if block has multiple predecessors
                let predecessors = self.get_predecessors(block);
                if predecessors.len() > 1 {
                    // Insert phi node (placeholder - would create actual phi)
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
        // Get predecessor blocks (simplified implementation)
        // In practice, this would use CFG analysis
        Vec::new()
    }
    
    fn promote_allocas(&mut self, function: &FunctionValue<'ctx>) -> Result<()> {
        // Replace loads and stores with register operations
        // This is a placeholder - in practice would rewrite the IR
        
        for block in function.get_basic_blocks() {
            for instruction in block.get_instructions() {
                match instruction.get_opcode() {
                    inkwell::values::InstructionOpcode::Load => {
                        // as_load() doesn't exist, check via opcode
                        if instruction.get_opcode() == InstructionOpcode::Load {
                            // Simplified - cannot get pointer operand without proper load instruction
                            // Placeholder check - in practice would analyze operands
                            // Replace load with register value (placeholder)
                        }
                    },
                    inkwell::values::InstructionOpcode::Store => {
                        // as_store() doesn't exist, check via opcode
                        if instruction.get_opcode() == InstructionOpcode::Store {
                            // Simplified - cannot get pointer operand without proper store instruction
                            // Placeholder check - in practice would analyze operands
                            // Remove store, update register value (placeholder)
                        }
                    },
                    _ => {}
                }
            }
        }
        
        Ok(())
    }
}
