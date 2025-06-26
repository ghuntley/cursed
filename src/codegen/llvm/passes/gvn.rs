//! Global Value Numbering optimization pass

use crate::error::{CursedError, Result};
use inkwell::{
    context::Context,
    module::Module,
    values::{FunctionValue, BasicValueEnum, InstructionValue},
    basic_block::BasicBlock,
};
use std::collections::{HashMap, HashSet};

/// Global Value Numbering pass for CURSED
pub struct GvnPass<'ctx> {
    context: &'ctx Context,
    load_pre: bool,      // Partial redundancy elimination for loads
    aggressive: bool,
    value_numbers: HashMap<ValueExpression, u32>,
    next_value_number: u32,
    redundant_instructions: HashSet<InstructionValue<'ctx>>,
}

impl<'ctx> GvnPass<'ctx> {
    /// Create a new GVN pass
    pub fn new(context: &'ctx Context, load_pre: bool, aggressive: bool) -> Self {
        Self {
            context,
            load_pre,
            aggressive,
            value_numbers: HashMap::new(),
            next_value_number: 0,
            redundant_instructions: HashSet::new(),
        }
    }
    
    /// Run GVN on a module
    pub fn run(&mut self, module: &Module<'ctx>) -> Result<GvnResult> {
        let mut result = GvnResult::default();
        
        for function in module.get_functions() {
            if function.get_first_basic_block().is_none() {
                continue; // Skip external functions
            }
            
            let function_result = self.run_on_function(function)?;
            result.merge(function_result);
        }
        
        Ok(result)
    }
    
    /// Run GVN on a single function
    fn run_on_function(&mut self, function: FunctionValue<'ctx>) -> Result<FunctionGvnResult> {
        let mut result = FunctionGvnResult::default();
        
        // Clear state for new function
        self.value_numbers.clear();
        self.next_value_number = 0;
        self.redundant_instructions.clear();
        
        // Build dominator tree (simplified)
        let dom_tree = self.build_dominator_tree(function)?;
        
        // Process blocks in dominator order
        let ordered_blocks = self.get_dominator_order(&dom_tree, function)?;
        
        for block in ordered_blocks {
            let block_result = self.process_basic_block(block)?;
            result.merge_block_result(block_result);
        }
        
        // Remove redundant instructions
        result.redundant_eliminated = self.eliminate_redundant_instructions()?;
        
        Ok(result)
    }
    
    /// Build a simplified dominator tree
    fn build_dominator_tree(&self, function: FunctionValue<'ctx>) -> Result<DominatorTree<'ctx>> {
        let mut dom_tree = DominatorTree::new();
        
        // For simplicity, we'll use a basic dominator analysis
        // A real implementation would use a more sophisticated algorithm
        
        let blocks: Vec<_> = function.get_basic_blocks().collect();
        
        // Entry block dominates itself
        if let Some(entry) = blocks.first() {
            dom_tree.set_immediate_dominator(*entry, None);
        }
        
        // For other blocks, find their immediate dominators
        for &block in &blocks[1..] {
            let idom = self.find_immediate_dominator(block, &blocks)?;
            dom_tree.set_immediate_dominator(block, idom);
        }
        
        Ok(dom_tree)
    }
    
    /// Find immediate dominator of a block (simplified)
    fn find_immediate_dominator(&self, block: BasicBlock<'ctx>, all_blocks: &[BasicBlock<'ctx>]) -> Result<Option<BasicBlock<'ctx>>> {
        // This is a very simplified approach
        // A real implementation would use the Lengauer-Tarjan algorithm
        
        let predecessors = self.get_predecessors(block);
        
        if predecessors.is_empty() {
            return Ok(None); // Entry block
        }
        
        // For simplicity, just return the first predecessor
        Ok(predecessors.first().copied())
    }
    
    /// Get predecessor blocks
    fn get_predecessors(&self, block: BasicBlock<'ctx>) -> Vec<BasicBlock<'ctx>> {
        let mut predecessors = Vec::new();
        
        // This would require analyzing the CFG
        // For now, return empty vector
        
        predecessors
    }
    
    /// Get blocks in dominator order
    fn get_dominator_order(&self, dom_tree: &DominatorTree<'ctx>, function: FunctionValue<'ctx>) -> Result<Vec<BasicBlock<'ctx>>> {
        let mut ordered_blocks = Vec::new();
        let blocks: Vec<_> = function.get_basic_blocks().collect();
        
        // For simplicity, just return blocks in original order
        // A real implementation would do a dominator tree traversal
        ordered_blocks.extend(blocks);
        
        Ok(ordered_blocks)
    }
    
    /// Process a basic block for GVN
    fn process_basic_block(&mut self, block: BasicBlock<'ctx>) -> Result<BlockGvnResult> {
        let mut result = BlockGvnResult::default();
        let mut expressions_in_block = HashMap::new();
        
        for instruction in block.get_instructions() {
            let process_result = self.process_instruction(instruction, &mut expressions_in_block)?;
            result.merge_instruction_result(process_result);
        }
        
        Ok(result)
    }
    
    /// Process a single instruction
    fn process_instruction(
        &mut self,
        instruction: InstructionValue<'ctx>,
        expressions_in_block: &mut HashMap<ValueExpression, BasicValueEnum<'ctx>>,
    ) -> Result<InstructionGvnResult> {
        let mut result = InstructionGvnResult::default();
        
        match instruction.get_opcode() {
            // Handle loads specially for load PRE
            inkwell::values::InstructionOpcode::Load => {
                if self.load_pre {
                    result = self.process_load_instruction(instruction, expressions_in_block)?;
                } else {
                    result = self.process_regular_instruction(instruction, expressions_in_block)?;
                }
            }
            
            // Handle arithmetic and logical operations
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
            inkwell::values::InstructionOpcode::ICmp |
            inkwell::values::InstructionOpcode::FCmp => {
                result = self.process_regular_instruction(instruction, expressions_in_block)?;
            }
            
            // Handle GEP instructions
            inkwell::values::InstructionOpcode::GetElementPtr => {
                result = self.process_gep_instruction(instruction, expressions_in_block)?;
            }
            
            // Skip instructions with side effects
            inkwell::values::InstructionOpcode::Store |
            inkwell::values::InstructionOpcode::Call |
            inkwell::values::InstructionOpcode::Invoke => {
                // These instructions can't be eliminated
            }
            
            _ => {
                // Try to process as regular instruction
                result = self.process_regular_instruction(instruction, expressions_in_block)?;
            }
        }
        
        Ok(result)
    }
    
    /// Process a load instruction
    fn process_load_instruction(
        &mut self,
        instruction: InstructionValue<'ctx>,
        expressions_in_block: &mut HashMap<ValueExpression, BasicValueEnum<'ctx>>,
    ) -> Result<InstructionGvnResult> {
        let mut result = InstructionGvnResult::default();
        
        // Create expression for the load
        let expr = self.create_load_expression(&instruction)?;
        
        // Check if we've seen this load before
        if let Some(&existing_value) = expressions_in_block.get(&expr) {
            // We can replace this load with the existing value
            instruction.replace_all_uses_with(&existing_value);
            self.redundant_instructions.insert(instruction);
            result.redundant_loads += 1;
        } else {
            // Record this load
            let value = instruction.as_basic_value_enum();
            expressions_in_block.insert(expr, value);
            result.loads_numbered += 1;
        }
        
        Ok(result)
    }
    
    /// Process a regular instruction
    fn process_regular_instruction(
        &mut self,
        instruction: InstructionValue<'ctx>,
        expressions_in_block: &mut HashMap<ValueExpression, BasicValueEnum<'ctx>>,
    ) -> Result<InstructionGvnResult> {
        let mut result = InstructionGvnResult::default();
        
        // Create expression for the instruction
        if let Some(expr) = self.create_instruction_expression(&instruction)? {
            // Check if we've seen this expression before
            if let Some(&existing_value) = expressions_in_block.get(&expr) {
                // We can replace this instruction with the existing value
                instruction.replace_all_uses_with(&existing_value);
                self.redundant_instructions.insert(instruction);
                result.redundant_expressions += 1;
            } else {
                // Record this expression
                let value = instruction.as_basic_value_enum();
                expressions_in_block.insert(expr, value);
                result.expressions_numbered += 1;
            }
        }
        
        Ok(result)
    }
    
    /// Process a GEP instruction
    fn process_gep_instruction(
        &mut self,
        instruction: InstructionValue<'ctx>,
        expressions_in_block: &mut HashMap<ValueExpression, BasicValueEnum<'ctx>>,
    ) -> Result<InstructionGvnResult> {
        let mut result = InstructionGvnResult::default();
        
        // GEP instructions can be complex, so we'll treat them specially
        let expr = self.create_gep_expression(&instruction)?;
        
        if let Some(&existing_value) = expressions_in_block.get(&expr) {
            instruction.replace_all_uses_with(&existing_value);
            self.redundant_instructions.insert(instruction);
            result.redundant_expressions += 1;
        } else {
            let value = instruction.as_basic_value_enum();
            expressions_in_block.insert(expr, value);
            result.expressions_numbered += 1;
        }
        
        Ok(result)
    }
    
    /// Create expression for a load instruction
    fn create_load_expression(&mut self, instruction: &InstructionValue<'ctx>) -> Result<ValueExpression> {
        if let Some(pointer_operand) = instruction.get_operand(0) {
            if let Some(pointer_value) = pointer_operand.left() {
                let pointer_vn = self.get_or_create_value_number(&pointer_value);
                
                return Ok(ValueExpression::Load {
                    pointer: pointer_vn,
                });
            }
        }
        
        Err(CursedError::runtime_error("Invalid load instruction"))
    }
    
    /// Create expression for a regular instruction
    fn create_instruction_expression(&mut self, instruction: &InstructionValue<'ctx>) -> Result<Option<ValueExpression>> {
        match instruction.get_opcode() {
            inkwell::values::InstructionOpcode::Add |
            inkwell::values::InstructionOpcode::Sub |
            inkwell::values::InstructionOpcode::Mul |
            inkwell::values::InstructionOpcode::SDiv |
            inkwell::values::InstructionOpcode::UDiv => {
                self.create_binary_expression(instruction)
            }
            
            inkwell::values::InstructionOpcode::And |
            inkwell::values::InstructionOpcode::Or |
            inkwell::values::InstructionOpcode::Xor => {
                self.create_binary_expression(instruction)
            }
            
            inkwell::values::InstructionOpcode::ICmp |
            inkwell::values::InstructionOpcode::FCmp => {
                self.create_comparison_expression(instruction)
            }
            
            _ => Ok(None),
        }
    }
    
    /// Create expression for a binary operation
    fn create_binary_expression(&mut self, instruction: &InstructionValue<'ctx>) -> Result<Option<ValueExpression>> {
        if instruction.get_num_operands() >= 2 {
            if let (Some(lhs_operand), Some(rhs_operand)) = (instruction.get_operand(0), instruction.get_operand(1)) {
                if let (Some(lhs_value), Some(rhs_value)) = (lhs_operand.left(), rhs_operand.left()) {
                    let lhs_vn = self.get_or_create_value_number(&lhs_value);
                    let rhs_vn = self.get_or_create_value_number(&rhs_value);
                    
                    let opcode = instruction.get_opcode();
                    
                    return Ok(Some(ValueExpression::Binary {
                        opcode: BinaryOpcode::from_llvm_opcode(opcode),
                        lhs: lhs_vn,
                        rhs: rhs_vn,
                    }));
                }
            }
        }
        
        Ok(None)
    }
    
    /// Create expression for a comparison
    fn create_comparison_expression(&mut self, instruction: &InstructionValue<'ctx>) -> Result<Option<ValueExpression>> {
        if instruction.get_num_operands() >= 3 {
            // ICmp and FCmp have predicate as first operand
            if let (Some(lhs_operand), Some(rhs_operand)) = (instruction.get_operand(1), instruction.get_operand(2)) {
                if let (Some(lhs_value), Some(rhs_value)) = (lhs_operand.left(), rhs_operand.left()) {
                    let lhs_vn = self.get_or_create_value_number(&lhs_value);
                    let rhs_vn = self.get_or_create_value_number(&rhs_value);
                    
                    // For simplicity, we'll ignore the predicate
                    return Ok(Some(ValueExpression::Compare {
                        lhs: lhs_vn,
                        rhs: rhs_vn,
                    }));
                }
            }
        }
        
        Ok(None)
    }
    
    /// Create expression for a GEP instruction
    fn create_gep_expression(&mut self, instruction: &InstructionValue<'ctx>) -> Result<ValueExpression> {
        let mut indices = Vec::new();
        
        // Get base pointer
        if let Some(base_operand) = instruction.get_operand(0) {
            if let Some(base_value) = base_operand.left() {
                let base_vn = self.get_or_create_value_number(&base_value);
                
                // Get indices
                for i in 1..instruction.get_num_operands() {
                    if let Some(index_operand) = instruction.get_operand(i) {
                        if let Some(index_value) = index_operand.left() {
                            let index_vn = self.get_or_create_value_number(&index_value);
                            indices.push(index_vn);
                        }
                    }
                }
                
                return Ok(ValueExpression::GetElementPtr {
                    base: base_vn,
                    indices,
                });
            }
        }
        
        Err(CursedError::runtime_error("Invalid GEP instruction"))
    }
    
    /// Get or create value number for a value
    fn get_or_create_value_number(&mut self, value: &BasicValueEnum<'ctx>) -> u32 {
        // For constants, we can use a special numbering scheme
        if value.is_const() {
            return self.get_constant_value_number(value);
        }
        
        // For instructions, create an expression and get its number
        if let Some(instruction) = value.as_instruction_value() {
            if let Ok(Some(expr)) = self.create_instruction_expression(&instruction) {
                return self.get_or_create_expression_number(expr);
            }
        }
        
        // Fallback: create a unique number
        let vn = self.next_value_number;
        self.next_value_number += 1;
        vn
    }
    
    /// Get value number for a constant
    fn get_constant_value_number(&mut self, value: &BasicValueEnum<'ctx>) -> u32 {
        // For constants, we can use a hash-based approach
        // This is simplified - a real implementation would handle different constant types
        
        if let Some(int_value) = value.as_int_value() {
            if let Some(const_val) = int_value.get_zero_extended_constant() {
                // Use the constant value as the base for numbering
                return (const_val as u32) % 1000000; // Avoid overflow
            }
        }
        
        // Fallback
        let vn = self.next_value_number;
        self.next_value_number += 1;
        vn
    }
    
    /// Get or create value number for an expression
    fn get_or_create_expression_number(&mut self, expr: ValueExpression) -> u32 {
        if let Some(&vn) = self.value_numbers.get(&expr) {
            vn
        } else {
            let vn = self.next_value_number;
            self.next_value_number += 1;
            self.value_numbers.insert(expr, vn);
            vn
        }
    }
    
    /// Eliminate redundant instructions
    fn eliminate_redundant_instructions(&mut self) -> Result<u32> {
        let mut eliminated_count = 0;
        
        for instruction in &self.redundant_instructions {
            unsafe {
                instruction.delete();
            }
            eliminated_count += 1;
        }
        
        self.redundant_instructions.clear();
        
        Ok(eliminated_count)
    }
}

/// Value expression for GVN
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValueExpression {
    Binary {
        opcode: BinaryOpcode,
        lhs: u32,
        rhs: u32,
    },
    Compare {
        lhs: u32,
        rhs: u32,
    },
    Load {
        pointer: u32,
    },
    GetElementPtr {
        base: u32,
        indices: Vec<u32>,
    },
}

/// Binary operation opcodes for GVN
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOpcode {
    Add,
    Sub,
    Mul,
    SDiv,
    UDiv,
    And,
    Or,
    Xor,
    Shl,
    LShr,
    AShr,
}

impl BinaryOpcode {
    fn from_llvm_opcode(opcode: inkwell::values::InstructionOpcode) -> Self {
        match opcode {
            inkwell::values::InstructionOpcode::Add => BinaryOpcode::Add,
            inkwell::values::InstructionOpcode::Sub => BinaryOpcode::Sub,
            inkwell::values::InstructionOpcode::Mul => BinaryOpcode::Mul,
            inkwell::values::InstructionOpcode::SDiv => BinaryOpcode::SDiv,
            inkwell::values::InstructionOpcode::UDiv => BinaryOpcode::UDiv,
            inkwell::values::InstructionOpcode::And => BinaryOpcode::And,
            inkwell::values::InstructionOpcode::Or => BinaryOpcode::Or,
            inkwell::values::InstructionOpcode::Xor => BinaryOpcode::Xor,
            inkwell::values::InstructionOpcode::Shl => BinaryOpcode::Shl,
            inkwell::values::InstructionOpcode::LShr => BinaryOpcode::LShr,
            inkwell::values::InstructionOpcode::AShr => BinaryOpcode::AShr,
            _ => BinaryOpcode::Add, // Fallback
        }
    }
}

/// Simplified dominator tree
pub struct DominatorTree<'ctx> {
    immediate_dominators: HashMap<BasicBlock<'ctx>, Option<BasicBlock<'ctx>>>,
}

impl<'ctx> DominatorTree<'ctx> {
    pub fn new() -> Self {
        Self {
            immediate_dominators: HashMap::new(),
        }
    }
    
    pub fn set_immediate_dominator(&mut self, block: BasicBlock<'ctx>, idom: Option<BasicBlock<'ctx>>) {
        self.immediate_dominators.insert(block, idom);
    }
    
    pub fn get_immediate_dominator(&self, block: BasicBlock<'ctx>) -> Option<BasicBlock<'ctx>> {
        self.immediate_dominators.get(&block).copied().flatten()
    }
    
    pub fn dominates(&self, dom: BasicBlock<'ctx>, block: BasicBlock<'ctx>) -> bool {
        if dom == block {
            return true;
        }
        
        let mut current = block;
        while let Some(idom) = self.get_immediate_dominator(current) {
            if idom == dom {
                return true;
            }
            current = idom;
        }
        
        false
    }
}

/// Result of GVN pass
#[derive(Debug, Default)]
pub struct GvnResult {
    pub redundant_expressions: u32,
    pub redundant_loads: u32,
    pub expressions_numbered: u32,
    pub loads_numbered: u32,
    pub redundant_eliminated: u32,
}

impl GvnResult {
    fn merge(&mut self, other: FunctionGvnResult) {
        self.redundant_expressions += other.redundant_expressions;
        self.redundant_loads += other.redundant_loads;
        self.expressions_numbered += other.expressions_numbered;
        self.loads_numbered += other.loads_numbered;
        self.redundant_eliminated += other.redundant_eliminated;
    }
    
    pub fn total_optimizations(&self) -> u32 {
        self.redundant_expressions + self.redundant_loads
    }
}

/// Result of GVN on a function
#[derive(Debug, Default)]
struct FunctionGvnResult {
    pub redundant_expressions: u32,
    pub redundant_loads: u32,
    pub expressions_numbered: u32,
    pub loads_numbered: u32,
    pub redundant_eliminated: u32,
}

impl FunctionGvnResult {
    fn merge_block_result(&mut self, other: BlockGvnResult) {
        self.redundant_expressions += other.redundant_expressions;
        self.redundant_loads += other.redundant_loads;
        self.expressions_numbered += other.expressions_numbered;
        self.loads_numbered += other.loads_numbered;
    }
}

/// Result of GVN on a basic block
#[derive(Debug, Default)]
struct BlockGvnResult {
    pub redundant_expressions: u32,
    pub redundant_loads: u32,
    pub expressions_numbered: u32,
    pub loads_numbered: u32,
}

impl BlockGvnResult {
    fn merge_instruction_result(&mut self, other: InstructionGvnResult) {
        self.redundant_expressions += other.redundant_expressions;
        self.redundant_loads += other.redundant_loads;
        self.expressions_numbered += other.expressions_numbered;
        self.loads_numbered += other.loads_numbered;
    }
}

/// Result of GVN on an instruction
#[derive(Debug, Default)]
struct InstructionGvnResult {
    pub redundant_expressions: u32,
    pub redundant_loads: u32,
    pub expressions_numbered: u32,
    pub loads_numbered: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    
    #[test]
    fn test_gvn_pass() {
        let context = Context::create();
        let pass = GvnPass::new(&context, true, false);
        assert!(pass.load_pre);
        assert!(!pass.aggressive);
    }
    
    #[test]
    fn test_value_expression() {
        let expr1 = ValueExpression::Binary {
            opcode: BinaryOpcode::Add,
            lhs: 1,
            rhs: 2,
        };
        
        let expr2 = ValueExpression::Binary {
            opcode: BinaryOpcode::Add,
            lhs: 1,
            rhs: 2,
        };
        
        assert_eq!(expr1, expr2);
    }
    
    #[test]
    fn test_dominator_tree() {
        let mut dom_tree = DominatorTree::new();
        assert!(dom_tree.immediate_dominators.is_empty());
    }
    
    #[test]
    fn test_gvn_result() {
        let mut result = GvnResult::default();
        result.redundant_expressions = 5;
        result.redundant_loads = 3;
        
        assert_eq!(result.total_optimizations(), 8);
    }
}
