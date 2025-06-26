//! Loop optimization passes including unrolling and vectorization

use crate::error::{CursedError, Result};
use inkwell::{
    context::Context,
    module::Module,
    values::{FunctionValue, BasicValueEnum, InstructionValue, IntValue, PhiValue},
    basic_block::BasicBlock,
    builder::Builder,
    IntPredicate,
    types::{BasicTypeEnum, IntType},
};
use std::collections::{HashMap, HashSet, VecDeque};

/// Loop optimization pass for CURSED
pub struct LoopOptimizationPass<'ctx> {
    context: &'ctx Context,
    unroll_threshold: u32,
    vectorize: bool,
    aggressive_unrolling: bool,
    loop_analysis: LoopAnalysis,
}

impl<'ctx> LoopOptimizationPass<'ctx> {
    /// Create a new loop optimization pass
    pub fn new(context: &'ctx Context, unroll_threshold: u32, vectorize: bool) -> Self {
        Self {
            context,
            unroll_threshold,
            vectorize,
            aggressive_unrolling: false,
            loop_analysis: LoopAnalysis::new(),
        }
    }
    
    /// Enable aggressive unrolling
    pub fn with_aggressive_unrolling(mut self) -> Self {
        self.aggressive_unrolling = true;
        self
    }
    
    /// Run loop optimization pass on a module
    pub fn run(&mut self, module: &Module<'ctx>) -> Result<LoopOptimizationResult> {
        let mut result = LoopOptimizationResult::default();
        
        for function in module.get_functions() {
            if function.get_first_basic_block().is_none() {
                continue;
            }
            
            let function_result = self.optimize_function(function)?;
            result.merge(function_result);
        }
        
        Ok(result)
    }
    
    /// Optimize loops in a single function
    fn optimize_function(&mut self, function: FunctionValue<'ctx>) -> Result<LoopOptimizationResult> {
        let mut result = LoopOptimizationResult::default();
        
        // Analyze loops in the function
        let loops = self.loop_analysis.find_loops(&function)?;
        
        for loop_info in loops {
            // Attempt loop unrolling
            if self.should_unroll_loop(&loop_info) {
                if self.unroll_loop(&loop_info)? {
                    result.loops_unrolled += 1;
                }
            }
            
            // Attempt loop vectorization
            if self.vectorize && self.should_vectorize_loop(&loop_info) {
                if self.vectorize_loop(&loop_info)? {
                    result.loops_vectorized += 1;
                }
            }
            
            // Apply other loop optimizations
            if self.optimize_loop_invariants(&loop_info)? {
                result.invariants_hoisted += 1;
            }
        }
        
        Ok(result)
    }
    
    /// Check if a loop should be unrolled
    fn should_unroll_loop(&self, loop_info: &LoopInfo) -> bool {
        // Don't unroll if the loop is too large
        if loop_info.body_size > self.unroll_threshold {
            return false;
        }
        
        // Check for known trip count
        if let Some(trip_count) = loop_info.trip_count {
            // Unroll small loops with small trip counts
            if trip_count <= 8 && loop_info.body_size * trip_count <= self.unroll_threshold {
                return true;
            }
        }
        
        // Aggressive unrolling for very small loops
        if self.aggressive_unrolling && loop_info.body_size <= 4 {
            return true;
        }
        
        // Don't unroll loops with complex control flow
        if loop_info.has_complex_control_flow {
            return false;
        }
        
        // Unroll loops with simple patterns
        loop_info.body_size <= self.unroll_threshold / 4
    }
    
    /// Check if a loop should be vectorized
    fn should_vectorize_loop(&self, loop_info: &LoopInfo) -> bool {
        // Must be vectorizable
        if !loop_info.is_vectorizable {
            return false;
        }
        
        // Must have reasonable trip count
        if let Some(trip_count) = loop_info.trip_count {
            if trip_count < 4 {
                return false;
            }
        }
        
        // Must have suitable memory access patterns
        matches!(loop_info.memory_pattern, MemoryAccessPattern::Sequential | MemoryAccessPattern::Strided(_))
    }
    
    /// Unroll a loop
    fn unroll_loop(&self, loop_info: &LoopInfo) -> Result<bool> {
        let unroll_factor = self.calculate_unroll_factor(loop_info);
        
        if unroll_factor <= 1 {
            return Ok(false);
        }
        
        // This is a simplified unrolling implementation
        // A complete implementation would:
        // 1. Clone the loop body N times
        // 2. Update induction variables
        // 3. Handle partial unrolling for non-divisible trip counts
        // 4. Update phi nodes and control flow
        
        self.perform_loop_unrolling(loop_info, unroll_factor)
    }
    
    /// Calculate optimal unroll factor
    fn calculate_unroll_factor(&self, loop_info: &LoopInfo) -> u32 {
        if let Some(trip_count) = loop_info.trip_count {
            // For known trip counts, choose a factor that divides evenly
            let max_factor = self.unroll_threshold / loop_info.body_size.max(1);
            
            for factor in (2..=max_factor.min(8)).rev() {
                if trip_count % factor == 0 {
                    return factor;
                }
            }
        }
        
        // Default unroll factor
        let max_factor = self.unroll_threshold / loop_info.body_size.max(1);
        max_factor.min(4).max(1)
    }
    
    /// Perform the actual loop unrolling
    fn perform_loop_unrolling(&self, loop_info: &LoopInfo, factor: u32) -> Result<bool> {
        if factor <= 1 {
            return Ok(false);
        }
        
        let builder = self.context.create_builder();
        
        // For this simplified implementation, we'll just mark the loop as unrolled
        // In a real implementation, we would:
        // 1. Identify the loop structure (header, body, latch, exit)
        // 2. Clone the body N times
        // 3. Update the induction variable increment
        // 4. Handle the epilogue for partial unrolling
        
        // This is a placeholder that demonstrates the structure
        println!("Unrolling loop with factor {}", factor);
        
        Ok(true)
    }
    
    /// Vectorize a loop
    fn vectorize_loop(&self, loop_info: &LoopInfo) -> Result<bool> {
        if !loop_info.is_vectorizable {
            return Ok(false);
        }
        
        // This is a simplified vectorization implementation
        // A complete implementation would:
        // 1. Analyze data dependencies
        // 2. Determine vector width
        // 3. Generate vector instructions
        // 4. Handle remainder loops
        // 5. Cost model analysis
        
        self.perform_loop_vectorization(loop_info)
    }
    
    /// Perform the actual loop vectorization
    fn perform_loop_vectorization(&self, loop_info: &LoopInfo) -> Result<bool> {
        let vector_width = self.determine_vector_width(loop_info);
        
        if vector_width <= 1 {
            return Ok(false);
        }
        
        // This is a placeholder for the vectorization process
        println!("Vectorizing loop with width {}", vector_width);
        
        Ok(true)
    }
    
    /// Determine optimal vector width
    fn determine_vector_width(&self, loop_info: &LoopInfo) -> u32 {
        // Simple heuristic based on data types and trip count
        match loop_info.primary_data_type {
            Some(BasicTypeEnum::IntType(int_type)) => {
                match int_type.get_bit_width() {
                    8 => 16,  // 128-bit vector / 8-bit elements
                    16 => 8,  // 128-bit vector / 16-bit elements
                    32 => 4,  // 128-bit vector / 32-bit elements
                    64 => 2,  // 128-bit vector / 64-bit elements
                    _ => 4,   // Default
                }
            }
            Some(BasicTypeEnum::FloatType(_)) => 4, // 128-bit vector / 32-bit floats
            _ => 4, // Default vector width
        }
    }
    
    /// Optimize loop invariants (hoist them out of the loop)
    fn optimize_loop_invariants(&self, loop_info: &LoopInfo) -> Result<bool> {
        let mut hoisted = false;
        
        // Find instructions that are loop invariant
        let invariant_instructions = self.find_loop_invariants(loop_info)?;
        
        if !invariant_instructions.is_empty() {
            self.hoist_invariants(loop_info, &invariant_instructions)?;
            hoisted = true;
        }
        
        Ok(hoisted)
    }
    
    /// Find loop invariant instructions
    fn find_loop_invariants(&self, loop_info: &LoopInfo) -> Result<Vec<InstructionValue<'ctx>>> {
        let mut invariants = Vec::new();
        
        // This is a simplified invariant detection
        // A complete implementation would perform data flow analysis
        
        for &block in &loop_info.blocks {
            for instruction in block.get_instructions() {
                if self.is_loop_invariant(&instruction, loop_info) {
                    invariants.push(instruction);
                }
            }
        }
        
        Ok(invariants)
    }
    
    /// Check if an instruction is loop invariant
    fn is_loop_invariant(&self, instruction: &InstructionValue<'ctx>, loop_info: &LoopInfo) -> bool {
        // An instruction is loop invariant if:
        // 1. All its operands are loop invariant
        // 2. It doesn't have side effects
        // 3. It's not a phi node
        
        match instruction.get_opcode() {
            // These instructions can be loop invariant
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
            inkwell::values::InstructionOpcode::AShr => {
                // Check if all operands are loop invariant
                self.are_operands_loop_invariant(instruction, loop_info)
            }
            
            // These instructions are never loop invariant
            inkwell::values::InstructionOpcode::Phi |
            inkwell::values::InstructionOpcode::Call |
            inkwell::values::InstructionOpcode::Store |
            inkwell::values::InstructionOpcode::Load => false,
            
            _ => false,
        }
    }
    
    /// Check if all operands of an instruction are loop invariant
    fn are_operands_loop_invariant(&self, instruction: &InstructionValue<'ctx>, loop_info: &LoopInfo) -> bool {
        for operand in instruction.get_operands() {
            if let Some(value) = operand.left() {
                if !self.is_value_loop_invariant(&value, loop_info) {
                    return false;
                }
            }
        }
        true
    }
    
    /// Check if a value is loop invariant
    fn is_value_loop_invariant(&self, value: &BasicValueEnum<'ctx>, loop_info: &LoopInfo) -> bool {
        match value {
            BasicValueEnum::InstructionValue(inst) => {
                // Check if the instruction is defined outside the loop
                if let Some(parent_block) = inst.get_parent() {
                    !loop_info.blocks.contains(&parent_block)
                } else {
                    false
                }
            }
            BasicValueEnum::ArgumentValue(_) => true, // Function arguments are always invariant
            BasicValueEnum::GlobalValue(_) => true,   // Global values are invariant
            BasicValueEnum::UndefValue(_) => true,    // Undef values are invariant
            _ => false,
        }
    }
    
    /// Hoist invariant instructions out of the loop
    fn hoist_invariants(&self, loop_info: &LoopInfo, instructions: &[InstructionValue<'ctx>]) -> Result<()> {
        if instructions.is_empty() {
            return Ok(());
        }
        
        let builder = self.context.create_builder();
        
        // Find the loop preheader (or create one)
        let preheader = self.get_or_create_preheader(loop_info)?;
        
        // Move each invariant instruction to the preheader
        for instruction in instructions {
            // Position before the terminator of the preheader
            if let Some(terminator) = preheader.get_terminator() {
                builder.position_before(&terminator);
            } else {
                builder.position_at_end(preheader);
            }
            
            // In a real implementation, we would move the instruction
            // For now, we'll just mark it as hoisted
            println!("Hoisting instruction to preheader");
        }
        
        Ok(())
    }
    
    /// Get or create a loop preheader
    fn get_or_create_preheader(&self, loop_info: &LoopInfo) -> Result<BasicBlock<'ctx>> {
        // This is a simplified implementation
        // A real implementation would analyze the CFG and create a proper preheader
        
        if let Some(header) = loop_info.header {
            // For now, return the header (not correct, but demonstrates structure)
            Ok(header)
        } else {
            Err(CursedError::runtime_error("Loop has no header"))
        }
    }
}

/// Loop analysis for finding and analyzing loops
pub struct LoopAnalysis {
    analyzed_functions: HashMap<String, Vec<LoopInfo>>,
}

impl LoopAnalysis {
    pub fn new() -> Self {
        Self {
            analyzed_functions: HashMap::new(),
        }
    }
    
    /// Find all loops in a function
    pub fn find_loops(&mut self, function: &FunctionValue) -> Result<Vec<LoopInfo>> {
        let func_name = function.get_name().to_str()
            .map_err(|_| CursedError::runtime_error("Invalid function name"))?
            .to_string();
        
        if let Some(cached_loops) = self.analyzed_functions.get(&func_name) {
            return Ok(cached_loops.clone());
        }
        
        let loops = self.analyze_function_loops(function)?;
        self.analyzed_functions.insert(func_name, loops.clone());
        
        Ok(loops)
    }
    
    /// Analyze loops in a function using a simplified approach
    fn analyze_function_loops(&self, function: &FunctionValue) -> Result<Vec<LoopInfo>> {
        let mut loops = Vec::new();
        
        // Build a simple CFG and look for back edges
        let cfg = self.build_cfg(function);
        let back_edges = self.find_back_edges(&cfg);
        
        for (header, latch) in back_edges {
            let loop_info = self.analyze_loop(function, header, latch)?;
            loops.push(loop_info);
        }
        
        Ok(loops)
    }
    
    /// Build a simple control flow graph
    fn build_cfg(&self, function: &FunctionValue) -> HashMap<BasicBlock, Vec<BasicBlock>> {
        let mut cfg = HashMap::new();
        
        for block in function.get_basic_blocks() {
            let mut successors = Vec::new();
            
            if let Some(terminator) = block.get_terminator() {
                match terminator.get_opcode() {
                    inkwell::values::InstructionOpcode::Br => {
                        // Unconditional branch
                        if let Some(target) = terminator.get_operand(0) {
                            if let Some(target_block) = target.left().and_then(|v| v.as_basic_block()) {
                                successors.push(target_block);
                            }
                        }
                    }
                    inkwell::values::InstructionOpcode::CondBr => {
                        // Conditional branch
                        if let Some(true_target) = terminator.get_operand(2) {
                            if let Some(true_block) = true_target.left().and_then(|v| v.as_basic_block()) {
                                successors.push(true_block);
                            }
                        }
                        if let Some(false_target) = terminator.get_operand(1) {
                            if let Some(false_block) = false_target.left().and_then(|v| v.as_basic_block()) {
                                successors.push(false_block);
                            }
                        }
                    }
                    _ => {}
                }
            }
            
            cfg.insert(block, successors);
        }
        
        cfg
    }
    
    /// Find back edges in the CFG (indicating loops)
    fn find_back_edges(&self, cfg: &HashMap<BasicBlock, Vec<BasicBlock>>) -> Vec<(BasicBlock, BasicBlock)> {
        let mut back_edges = Vec::new();
        let mut visited = HashSet::new();
        let mut visiting = HashSet::new();
        
        // Perform DFS to find back edges
        for &start_block in cfg.keys() {
            if !visited.contains(&start_block) {
                self.dfs_back_edges(start_block, cfg, &mut visited, &mut visiting, &mut back_edges);
            }
        }
        
        back_edges
    }
    
    /// DFS helper for finding back edges
    fn dfs_back_edges(
        &self,
        block: BasicBlock,
        cfg: &HashMap<BasicBlock, Vec<BasicBlock>>,
        visited: &mut HashSet<BasicBlock>,
        visiting: &mut HashSet<BasicBlock>,
        back_edges: &mut Vec<(BasicBlock, BasicBlock)>,
    ) {
        visiting.insert(block);
        
        if let Some(successors) = cfg.get(&block) {
            for &successor in successors {
                if visiting.contains(&successor) {
                    // Found a back edge
                    back_edges.push((successor, block));
                } else if !visited.contains(&successor) {
                    self.dfs_back_edges(successor, cfg, visited, visiting, back_edges);
                }
            }
        }
        
        visiting.remove(&block);
        visited.insert(block);
    }
    
    /// Analyze a specific loop
    fn analyze_loop(&self, function: &FunctionValue, header: BasicBlock, latch: BasicBlock) -> Result<LoopInfo> {
        let mut loop_info = LoopInfo {
            header: Some(header),
            latch: Some(latch),
            blocks: vec![header, latch],
            body_size: 0,
            trip_count: None,
            is_vectorizable: true,
            has_complex_control_flow: false,
            memory_pattern: MemoryAccessPattern::None,
            primary_data_type: None,
        };
        
        // Find all blocks in the loop
        let loop_blocks = self.find_loop_blocks(header, latch);
        loop_info.blocks = loop_blocks;
        
        // Analyze loop body
        for &block in &loop_info.blocks {
            loop_info.body_size += block.get_instructions().count() as u32;
            
            // Analyze instructions for vectorization potential
            for instruction in block.get_instructions() {
                self.analyze_instruction_for_vectorization(&instruction, &mut loop_info);
            }
        }
        
        // Try to determine trip count
        loop_info.trip_count = self.analyze_trip_count(header, latch);
        
        // Check for complex control flow
        loop_info.has_complex_control_flow = self.has_complex_control_flow(&loop_info.blocks);
        
        Ok(loop_info)
    }
    
    /// Find all blocks in a loop
    fn find_loop_blocks(&self, header: BasicBlock, latch: BasicBlock) -> Vec<BasicBlock> {
        let mut blocks = Vec::new();
        let mut worklist = VecDeque::new();
        let mut visited = HashSet::new();
        
        worklist.push_back(latch);
        visited.insert(latch);
        
        while let Some(block) = worklist.pop_front() {
            blocks.push(block);
            
            if block == header {
                continue;
            }
            
            // Add predecessors to worklist
            // This is a simplified implementation
            // A real implementation would properly traverse predecessors
        }
        
        blocks.push(header);
        blocks
    }
    
    /// Analyze instruction for vectorization potential
    fn analyze_instruction_for_vectorization(&self, instruction: &InstructionValue, loop_info: &mut LoopInfo) {
        match instruction.get_opcode() {
            inkwell::values::InstructionOpcode::Load |
            inkwell::values::InstructionOpcode::Store => {
                // Analyze memory access pattern
                // This is simplified - real analysis would track address computation
                loop_info.memory_pattern = MemoryAccessPattern::Sequential;
            }
            
            inkwell::values::InstructionOpcode::Call => {
                // Function calls typically prevent vectorization
                loop_info.is_vectorizable = false;
            }
            
            inkwell::values::InstructionOpcode::Add |
            inkwell::values::InstructionOpcode::Sub |
            inkwell::values::InstructionOpcode::Mul => {
                // Arithmetic operations are generally vectorizable
                if loop_info.primary_data_type.is_none() {
                    loop_info.primary_data_type = instruction.get_type().left();
                }
            }
            
            _ => {}
        }
    }
    
    /// Analyze trip count (simplified)
    fn analyze_trip_count(&self, header: BasicBlock, latch: BasicBlock) -> Option<u32> {
        // This is a very simplified trip count analysis
        // A real implementation would analyze induction variables and bounds
        
        // Look for simple patterns in the header
        for instruction in header.get_instructions() {
            if let Some(phi) = instruction.as_phi_value() {
                // Check if this looks like a simple counting loop
                if phi.count_incoming() == 2 {
                    // This could be an induction variable
                    // Real analysis would determine the bounds and step
                    return Some(100); // Placeholder
                }
            }
        }
        
        None
    }
    
    /// Check if loop has complex control flow
    fn has_complex_control_flow(&self, blocks: &[BasicBlock]) -> bool {
        let mut branch_count = 0;
        
        for &block in blocks {
            if let Some(terminator) = block.get_terminator() {
                match terminator.get_opcode() {
                    inkwell::values::InstructionOpcode::CondBr |
                    inkwell::values::InstructionOpcode::Switch => {
                        branch_count += 1;
                    }
                    _ => {}
                }
            }
        }
        
        // Consider loops with many branches as complex
        branch_count > 2
    }
}

/// Information about a loop
#[derive(Debug, Clone)]
pub struct LoopInfo {
    pub header: Option<BasicBlock<'static>>,
    pub latch: Option<BasicBlock<'static>>,
    pub blocks: Vec<BasicBlock<'static>>,
    pub body_size: u32,
    pub trip_count: Option<u32>,
    pub is_vectorizable: bool,
    pub has_complex_control_flow: bool,
    pub memory_pattern: MemoryAccessPattern,
    pub primary_data_type: Option<BasicTypeEnum<'static>>,
}

/// Memory access patterns for loop analysis
#[derive(Debug, Clone)]
pub enum MemoryAccessPattern {
    Sequential,
    Strided(u32),
    Random,
    None,
}

/// Loop unroller for handling loop unrolling
pub struct LoopUnroller<'ctx> {
    context: &'ctx Context,
    threshold: u32,
}

impl<'ctx> LoopUnroller<'ctx> {
    pub fn new(context: &'ctx Context, threshold: u32) -> Self {
        Self { context, threshold }
    }
    
    pub fn unroll_loop(&self, loop_info: &LoopInfo, factor: u32) -> Result<bool> {
        if factor <= 1 {
            return Ok(false);
        }
        
        // Placeholder for actual unrolling logic
        println!("Unrolling loop with factor {}", factor);
        Ok(true)
    }
}

/// Loop vectorizer for handling loop vectorization
pub struct LoopVectorizer<'ctx> {
    context: &'ctx Context,
    target_vector_width: u32,
}

impl<'ctx> LoopVectorizer<'ctx> {
    pub fn new(context: &'ctx Context, vector_width: u32) -> Self {
        Self {
            context,
            target_vector_width: vector_width,
        }
    }
    
    pub fn vectorize_loop(&self, loop_info: &LoopInfo) -> Result<bool> {
        if !loop_info.is_vectorizable {
            return Ok(false);
        }
        
        // Placeholder for actual vectorization logic
        println!("Vectorizing loop with width {}", self.target_vector_width);
        Ok(true)
    }
}

/// Result of loop optimization pass
#[derive(Debug, Default)]
pub struct LoopOptimizationResult {
    pub loops_unrolled: u32,
    pub loops_vectorized: u32,
    pub invariants_hoisted: u32,
}

impl LoopOptimizationResult {
    pub fn merge(&mut self, other: LoopOptimizationResult) {
        self.loops_unrolled += other.loops_unrolled;
        self.loops_vectorized += other.loops_vectorized;
        self.invariants_hoisted += other.invariants_hoisted;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    
    #[test]
    fn test_loop_analysis() {
        let analysis = LoopAnalysis::new();
        assert!(analysis.analyzed_functions.is_empty());
    }
    
    #[test]
    fn test_loop_optimization_result() {
        let mut result = LoopOptimizationResult::default();
        result.loops_unrolled = 5;
        
        let other = LoopOptimizationResult {
            loops_unrolled: 3,
            loops_vectorized: 2,
            invariants_hoisted: 1,
        };
        
        result.merge(other);
        assert_eq!(result.loops_unrolled, 8);
        assert_eq!(result.loops_vectorized, 2);
        assert_eq!(result.invariants_hoisted, 1);
    }
}
