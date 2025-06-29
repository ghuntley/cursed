//! Scalar Replacement of Aggregates pass

use crate::error::{CursedError, Result};
use inkwell::values::{FunctionValue, InstructionValue, PointerValue, AggregateValue, InstructionOpcode, AnyValue};
use inkwell::context::Context;
use std::collections::{HashMap, HashSet};

/// SROA pass - Scalar Replacement of Aggregates
pub struct SroaPass<'ctx> {
    context: &'ctx Context,
    aggregates: Vec<PointerValue<'ctx>>,
    replaceable_aggregates: HashSet<PointerValue<'ctx>>,
    scalar_replacements: HashMap<PointerValue<'ctx>, Vec<PointerValue<'ctx>>>,
}

impl<'ctx> SroaPass<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        Self {
            context,
            aggregates: Vec::new(),
            replaceable_aggregates: HashSet::new(),
            scalar_replacements: HashMap::new(),
        }
    }
    
    /// Run SROA on a function
    pub fn run_on_function(&mut self, function: &FunctionValue<'ctx>) -> Result<bool> {
        let mut changed = false;
        
        // Find aggregate allocas
        self.find_aggregates(function)?;
        
        // Analyze which aggregates can be replaced
        self.analyze_replaceability()?;
        
        // Replace aggregates with scalars
        if !self.replaceable_aggregates.is_empty() {
            self.replace_aggregates(function)?;
            changed = true;
        }
        
        Ok(changed)
    }
    
    fn find_aggregates(&mut self, function: &FunctionValue<'ctx>) -> Result<()> {
        // Find alloca instructions that allocate aggregate types
        if let Some(entry_block) = function.get_first_basic_block() {
            for instruction in entry_block.get_instructions() {
                // Check if instruction is an alloca via opcode
                if instruction.get_opcode() == InstructionOpcode::Alloca {
                    let alloca = instruction.as_any_value_enum().into_pointer_value();
                    // get_element_type() doesn't exist on PointerType in inkwell 0.4
                    // Skip checking type for now
                    let alloca_type = alloca.get_type(); // simplified
                    
                    // Check if it's an aggregate type (struct, array)
                    // Note: PointerType doesn't have is_struct_type/is_array_type in inkwell 0.4
                    // Simplified check - assume all allocas are candidates for now
                    if true { // TODO: proper type checking
                        self.aggregates.push(alloca);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn analyze_replaceability(&mut self) -> Result<()> {
        // Analyze each aggregate to see if it can be replaced
        for aggregate in &self.aggregates {
            if self.can_replace_aggregate(aggregate)? {
                self.replaceable_aggregates.insert(*aggregate);
            }
        }
        
        Ok(())
    }
    
    fn can_replace_aggregate(&self, aggregate: &PointerValue<'ctx>) -> Result<bool> {
        // Check if aggregate is only accessed via GEP instructions
        // and the GEP indices are constants
        
        // get_users() doesn't exist in inkwell 0.4, stub for now
        let users: Vec<inkwell::values::InstructionValue> = vec![]; // TODO: implement user traversal
        for user in users {
            // as_instruction() doesn't exist in inkwell 0.4, stub for now
            if false { // TODO: proper instruction check
                let instruction = user;
                match instruction.get_opcode() {
                    inkwell::values::InstructionOpcode::GetElementPtr => {
                        // GEP is OK if indices are constant
                        if !self.has_constant_indices(&instruction) {
                            return Ok(false);
                        }
                    },
                    inkwell::values::InstructionOpcode::Load => {
                        // Direct load of aggregate is OK
                    },
                    inkwell::values::InstructionOpcode::Store => {
                        // Direct store to aggregate is OK
                    },
                    _ => {
                        // Other uses prevent replacement
                        return Ok(false);
                    }
                }
            }
        }
        
        Ok(true)
    }
    
    fn has_constant_indices(&self, _instruction: &InstructionValue<'ctx>) -> bool {
        // Check if GEP instruction has all constant indices
        // Simplified - in practice would check all operands
        true
    }
    
    fn replace_aggregates(&mut self, function: &FunctionValue<'ctx>) -> Result<()> {
        // Replace each aggregate with individual scalar allocas
        let aggregates: Vec<_> = self.replaceable_aggregates.iter().cloned().collect();
        for aggregate in aggregates {
            self.create_scalar_replacements(&aggregate)?;
        }
        
        // Rewrite uses of aggregates to use scalars
        self.rewrite_aggregate_uses(function)?;
        
        Ok(())
    }
    
    fn create_scalar_replacements(&mut self, aggregate: &PointerValue<'ctx>) -> Result<()> {
        // Create individual allocas for each field/element
        // This is a placeholder - in practice would analyze the type structure
        
        // get_element_type() doesn't exist on PointerType in inkwell 0.4
        // For now, return without creating replacements
        
        let replacements = Vec::new();
        self.scalar_replacements.insert(*aggregate, replacements);
        Ok(())
    }
    
    fn rewrite_aggregate_uses(&self, function: &FunctionValue<'ctx>) -> Result<()> {
        // Rewrite all uses of aggregates to use scalar replacements
        // This is a placeholder - in practice would rewrite the IR
        
        for block in function.get_basic_blocks() {
            for instruction in block.get_instructions() {
                if let inkwell::values::InstructionOpcode::GetElementPtr = instruction.get_opcode() {
                    // Rewrite GEP to access scalar replacement
                }
            }
        }
        
        Ok(())
    }
}
