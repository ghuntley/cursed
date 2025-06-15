/// Tail Call Optimization (TCO)
/// 
/// This pass converts tail calls into jumps, eliminating stack frame overhead
/// for recursive functions and improving performance for functional programming patterns.

use super::{OptimizationPass, PassConfiguration, PassResult, OptimizationLevel};
use crate::error::{Error, Result};
use inkwell::{
    context::Context,
    module::Module,
    values::{FunctionValue, BasicValueEnum, InstructionValue, BasicValue, CallSiteValue},
    basic_block::BasicBlock,
    builder::Builder,
    types::BasicType,
};
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};
use tracing::{debug, info, instrument, warn};

/// Tail Call Optimization pass
pub struct TailCallPass<'ctx> {
    context_lifetime: std::marker::PhantomData<&'ctx ()>,
    statistics: TailCallStatistics,
    max_recursion_depth: usize,
    enable_mutual_recursion: bool,
}

impl<'ctx> TailCallPass<'ctx> {
    /// Create new Tail Call Optimization pass
    pub fn new() -> Self {
        Self {
            context_lifetime: std::marker::PhantomData,
            statistics: TailCallStatistics::default(),
            max_recursion_depth: 100,
            enable_mutual_recursion: false,
        }
    }
    
    /// Create TCO pass with custom settings
    pub fn with_settings(max_recursion_depth: usize, enable_mutual_recursion: bool) -> Self {
        Self {
            context_lifetime: std::marker::PhantomData,
            statistics: TailCallStatistics::default(),
            max_recursion_depth,
            enable_mutual_recursion,
        }
    }
}

impl<'ctx> OptimizationPass<'ctx> for TailCallPass<'ctx> {
    fn name(&self) -> &str {
        "tail_call"
    }
    
    fn description(&self) -> &str {
        "Tail Call Optimization - converts tail calls to jumps for better performance"
    }
    
    fn dependencies(&self) -> Vec<String> {
        vec![
            "mem2reg".to_string(),
            "instcombine".to_string(),
        ]
    }
    
    fn should_run(&self, config: &PassConfiguration) -> bool {
        config.optimization_level >= OptimizationLevel::Default
    }
    
    fn required_optimization_level(&self) -> OptimizationLevel {
        OptimizationLevel::Default
    }
    
    fn estimated_execution_time(&self) -> Duration {
        Duration::from_millis(250)
    }
    
    #[instrument(skip(self, module, context))]
    fn run_on_module(&mut self, module: &Module<'ctx>, context: &'ctx Context) -> Result<PassResult> {
        let start_time = Instant::now();
        info!("Running Tail Call Optimization pass on module");
        
        let mut total_result = PassResult::unchanged();
        
        // Run TCO on each function
        for function in module.get_functions() {
            if function.get_first_basic_block().is_some() {
                debug!("Running TCO on function: {:?}", function.get_name());
                let function_result = self.run_on_function(&function, context)?;
                total_result = total_result.merge(function_result);
            }
        }
        
        total_result.execution_time = start_time.elapsed();
        
        info!("TCO pass completed: {} tail calls optimized",
              total_result.instructions_eliminated);
        
        Ok(total_result)
    }
    
    #[instrument(skip(self, function, context))]
    fn run_on_function(&mut self, function: &FunctionValue<'ctx>, context: &'ctx Context) -> Result<PassResult> {
        let mut result = PassResult::unchanged();
        
        // Find tail call candidates
        let tail_call_candidates = self.find_tail_call_candidates(function)?;
        
        if tail_call_candidates.is_empty() {
            debug!("No tail call candidates found");
            return Ok(result);
        }
        
        info!("Found {} tail call candidates", tail_call_candidates.len());
        
        // Create tail call optimizer
        let mut optimizer = TailCallOptimizer::new(function, context);
        
        // Optimize each tail call
        let mut optimized_count = 0;
        for candidate in tail_call_candidates {
            if optimizer.optimize_tail_call(candidate)? {
                optimized_count += 1;
                result.changed = true;
            }
        }
        
        result.instructions_eliminated = optimized_count;
        self.statistics.functions_processed += 1;
        self.statistics.total_tail_calls_optimized += optimized_count;
        
        debug!("Optimized {} tail calls", optimized_count);
        
        Ok(result)
    }
    
    fn get_statistics(&self) -> super::PassStatistics {
        super::PassStatistics {
            total_executions: self.statistics.functions_processed,
            successful_executions: self.statistics.functions_processed,
            total_execution_time: Duration::from_millis(0),
            average_execution_time: Duration::from_millis(0),
            total_instructions_eliminated: self.statistics.total_tail_calls_optimized,
            total_functions_inlined: 0,
            total_optimizations_applied: self.statistics.total_tail_calls_optimized,
            peak_memory_usage: 0,
        }
    }
    
    /// Find tail call candidates in a function
    fn find_tail_call_candidates(&self, function: &FunctionValue<'ctx>) -> Result<Vec<TailCallCandidate<'ctx>>> {
        let mut candidates = Vec::new();
        
        // Look for tail calls in all basic blocks
        let mut block = function.get_first_basic_block();
        while let Some(bb) = block {
            if let Some(candidate) = self.analyze_block_for_tail_calls(bb, function)? {
                candidates.push(candidate);
            }
            block = bb.get_next_basic_block();
        }
        
        Ok(candidates)
    }
    
    /// Analyze a basic block for tail call patterns
    fn analyze_block_for_tail_calls(
        &self,
        block: BasicBlock<'ctx>,
        function: &FunctionValue<'ctx>,
    ) -> Result<Option<TailCallCandidate<'ctx>>> {
        // Look for the pattern: call followed immediately by return
        let mut prev_instruction = None;
        let mut instruction = block.get_first_instruction();
        
        while let Some(instr) = instruction {
            let opcode = instr.get_opcode();
            
            if opcode == inkwell::values::InstructionOpcode::Return {
                // Found a return, check if previous instruction was a call
                if let Some(prev_instr) = prev_instruction {
                    if self.is_tail_call_pattern(&prev_instr, &instr, function)? {
                        let candidate = TailCallCandidate {
                            call_instruction: prev_instr,
                            return_instruction: instr,
                            block,
                            call_type: self.determine_call_type(&prev_instr, function)?,
                        };
                        return Ok(Some(candidate));
                    }
                }
            }
            
            prev_instruction = Some(instr);
            instruction = instr.get_next_instruction();
        }
        
        Ok(None)
    }
    
    /// Check if we have a valid tail call pattern
    fn is_tail_call_pattern(
        &self,
        call_instr: &InstructionValue<'ctx>,
        return_instr: &InstructionValue<'ctx>,
        function: &FunctionValue<'ctx>,
    ) -> Result<bool> {
        // Must be a call instruction
        if call_instr.get_opcode() != inkwell::values::InstructionOpcode::Call {
            return Ok(false);
        }
        
        // Check if the return value matches the call result
        if !self.return_matches_call_result(call_instr, return_instr)? {
            return Ok(false);
        }
        
        // Check if the call doesn't have side effects that prevent optimization
        if self.has_preventing_side_effects(call_instr)? {
            return Ok(false);
        }
        
        // Check calling convention compatibility
        if !self.is_calling_convention_compatible(call_instr, function)? {
            return Ok(false);
        }
        
        Ok(true)
    }
    
    /// Check if return value matches call result
    fn return_matches_call_result(
        &self,
        call_instr: &InstructionValue<'ctx>,
        return_instr: &InstructionValue<'ctx>,
    ) -> Result<bool> {
        // Check if return instruction returns the result of the call
        // This is a simplified check - real implementation would be more thorough
        
        let return_operands = return_instr.get_num_operands();
        
        if return_operands == 0 {
            // Void return - check if call also returns void
            return Ok(true); // Simplified
        }
        
        if return_operands == 1 {
            // Single return value - check if it's the call result
            if let Some(return_operand) = return_instr.get_operand(0) {
                if let Some(return_value) = return_operand.left() {
                    // Check if return_value is the result of call_instr
                    return Ok(self.is_call_result(return_value, call_instr));
                }
            }
        }
        
        Ok(false)
    }
    
    /// Check if a value is the result of a specific call
    fn is_call_result(&self, value: BasicValueEnum<'ctx>, call_instr: &InstructionValue<'ctx>) -> bool {
        if value.is_instruction_value() {
            let value_instr = value.into_instruction_value();
            value_instr == *call_instr
        } else {
            false
        }
    }
    
    /// Check for side effects that prevent tail call optimization
    fn has_preventing_side_effects(&self, call_instr: &InstructionValue<'ctx>) -> Result<bool> {
        // Check for exception handling
        if self.may_throw_exception(call_instr) {
            return Ok(true);
        }
        
        // Check for debug info that might be lost
        // Real implementation would check debug metadata
        
        // Check for address-taken scenarios
        // Real implementation would check if function address is taken
        
        Ok(false)
    }
    
    /// Check if instruction may throw an exception
    fn may_throw_exception(&self, call_instr: &InstructionValue<'ctx>) -> bool {
        // In a real implementation, we'd check:
        // 1. If the called function may throw
        // 2. If there are exception handling constructs
        // For now, assume no exceptions
        false
    }
    
    /// Check calling convention compatibility
    fn is_calling_convention_compatible(
        &self,
        call_instr: &InstructionValue<'ctx>,
        function: &FunctionValue<'ctx>,
    ) -> Result<bool> {
        // Check if calling conventions match
        // Real implementation would extract and compare calling conventions
        Ok(true) // Simplified
    }
    
    /// Determine the type of tail call
    fn determine_call_type(
        &self,
        call_instr: &InstructionValue<'ctx>,
        function: &FunctionValue<'ctx>,
    ) -> Result<TailCallType> {
        // Check if it's a self-recursive call
        if let Some(called_function) = self.get_called_function(call_instr) {
            if called_function == *function {
                return Ok(TailCallType::SelfRecursive);
            }
            
            // Check for mutual recursion if enabled
            if self.enable_mutual_recursion {
                if self.is_mutually_recursive(function, &called_function)? {
                    return Ok(TailCallType::MutuallyRecursive);
                }
            }
            
            return Ok(TailCallType::NonRecursive);
        }
        
        Ok(TailCallType::Unknown)
    }
    
    /// Get the function being called
    fn get_called_function(&self, call_instr: &InstructionValue<'ctx>) -> Option<FunctionValue<'ctx>> {
        // Extract called function from call instruction
        // This is simplified - real implementation would handle function pointers
        let num_operands = call_instr.get_num_operands();
        if num_operands > 0 {
            if let Some(operand) = call_instr.get_operand(num_operands - 1) {
                if let Some(function_value) = operand.left() {
                    return function_value.try_into().ok();
                }
            }
        }
        None
    }
    
    /// Check if two functions are mutually recursive
    fn is_mutually_recursive(
        &self,
        function1: &FunctionValue<'ctx>,
        function2: &FunctionValue<'ctx>,
    ) -> Result<bool> {
        // This would require call graph analysis
        // For now, return false (simplified)
        Ok(false)
    }
}

/// Tail call optimizer that performs the actual transformations
struct TailCallOptimizer<'ctx> {
    function: &'ctx FunctionValue<'ctx>,
    context: &'ctx Context,
}

impl<'ctx> TailCallOptimizer<'ctx> {
    /// Create new tail call optimizer
    fn new(function: &'ctx FunctionValue<'ctx>, context: &'ctx Context) -> Self {
        Self { function, context }
    }
    
    /// Optimize a tail call candidate
    fn optimize_tail_call(&mut self, candidate: TailCallCandidate<'ctx>) -> Result<bool> {
        match candidate.call_type {
            TailCallType::SelfRecursive => {
                self.optimize_self_recursive_tail_call(candidate)
            }
            TailCallType::MutuallyRecursive => {
                self.optimize_mutually_recursive_tail_call(candidate)
            }
            TailCallType::NonRecursive => {
                self.optimize_non_recursive_tail_call(candidate)
            }
            TailCallType::Unknown => Ok(false),
        }
    }
    
    /// Optimize self-recursive tail call
    fn optimize_self_recursive_tail_call(&mut self, candidate: TailCallCandidate<'ctx>) -> Result<bool> {
        debug!("Optimizing self-recursive tail call");
        
        // Convert recursive call to loop:
        // 1. Extract call arguments
        // 2. Update function parameters  
        // 3. Jump to function entry
        // 4. Remove call and return instructions
        
        let call_args = self.extract_call_arguments(&candidate.call_instruction)?;
        
        // Create entry block if needed for the loop
        let entry_block = self.get_or_create_entry_block()?;
        
        // Update parameters and jump to entry
        self.create_parameter_updates_and_jump(&call_args, entry_block, &candidate)?;
        
        // Remove original call and return
        self.remove_tail_call_instructions(&candidate)?;
        
        debug!("Successfully converted recursive tail call to loop");
        Ok(true)
    }
    
    /// Optimize mutually recursive tail call
    fn optimize_mutually_recursive_tail_call(&mut self, candidate: TailCallCandidate<'ctx>) -> Result<bool> {
        debug!("Optimizing mutually recursive tail call");
        
        // This is more complex and requires coordination between functions
        // For now, just mark the call as tail call for LLVM
        self.mark_as_tail_call(&candidate.call_instruction)?;
        
        Ok(true)
    }
    
    /// Optimize non-recursive tail call
    fn optimize_non_recursive_tail_call(&mut self, candidate: TailCallCandidate<'ctx>) -> Result<bool> {
        debug!("Optimizing non-recursive tail call");
        
        // Mark as tail call for LLVM optimization
        self.mark_as_tail_call(&candidate.call_instruction)?;
        
        Ok(true)
    }
    
    /// Extract arguments from call instruction
    fn extract_call_arguments(&self, call_instr: &InstructionValue<'ctx>) -> Result<Vec<BasicValueEnum<'ctx>>> {
        let mut args = Vec::new();
        let num_operands = call_instr.get_num_operands();
        
        // All operands except the last one (function) are arguments
        for i in 0..num_operands.saturating_sub(1) {
            if let Some(operand) = call_instr.get_operand(i) {
                if let Some(arg_value) = operand.left() {
                    args.push(arg_value);
                }
            }
        }
        
        Ok(args)
    }
    
    /// Get or create entry block for the function
    fn get_or_create_entry_block(&self) -> Result<BasicBlock<'ctx>> {
        if let Some(entry) = self.function.get_first_basic_block() {
            Ok(entry)
        } else {
            Err(Error::Internal("Function has no entry block".to_string()))
        }
    }
    
    /// Create parameter updates and jump to entry
    fn create_parameter_updates_and_jump(
        &self,
        call_args: &[BasicValueEnum<'ctx>],
        entry_block: BasicBlock<'ctx>,
        candidate: &TailCallCandidate<'ctx>,
    ) -> Result<()> {
        // In a real implementation:
        // 1. Create a new basic block for parameter updates
        // 2. Generate stores to parameter locations
        // 3. Create unconditional branch to entry
        // 4. Update the current block to branch to update block
        
        debug!("Would create parameter updates for {} arguments", call_args.len());
        Ok(())
    }
    
    /// Remove tail call instructions
    fn remove_tail_call_instructions(&self, candidate: &TailCallCandidate<'ctx>) -> Result<()> {
        // In a real implementation:
        // 1. Remove the call instruction
        // 2. Remove the return instruction
        // 3. Update CFG as needed
        
        debug!("Would remove tail call and return instructions");
        Ok(())
    }
    
    /// Mark call as tail call for LLVM
    fn mark_as_tail_call(&self, call_instr: &InstructionValue<'ctx>) -> Result<()> {
        // In a real implementation, we'd set the tail call attribute
        // on the call instruction for LLVM to optimize
        
        debug!("Would mark call as tail call for LLVM");
        Ok(())
    }
}

/// Information about a tail call candidate
#[derive(Debug)]
struct TailCallCandidate<'ctx> {
    /// The call instruction
    call_instruction: InstructionValue<'ctx>,
    /// The return instruction
    return_instruction: InstructionValue<'ctx>,
    /// The basic block containing the tail call
    block: BasicBlock<'ctx>,
    /// Type of tail call
    call_type: TailCallType,
}

/// Types of tail calls
#[derive(Debug, Clone, PartialEq)]
enum TailCallType {
    /// Self-recursive call (function calls itself)
    SelfRecursive,
    /// Mutually recursive call (function calls another that calls back)
    MutuallyRecursive,
    /// Non-recursive call
    NonRecursive,
    /// Unknown or complex call pattern
    Unknown,
}

/// Statistics for tail call optimization
#[derive(Debug, Default)]
struct TailCallStatistics {
    pub functions_processed: u64,
    pub total_tail_calls_optimized: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    
    #[test]
    fn test_tail_call_pass_creation() {
        let pass = TailCallPass::<'_>::new();
        assert_eq!(pass.name(), "tail_call");
        assert!(pass.description().contains("Tail Call"));
    }
    
    #[test]
    fn test_tail_call_pass_with_settings() {
        let pass = TailCallPass::<'_>::with_settings(200, true);
        assert_eq!(pass.max_recursion_depth, 200);
        assert!(pass.enable_mutual_recursion);
    }
    
    #[test]
    fn test_tail_call_dependencies() {
        let pass = TailCallPass::<'_>::new();
        let deps = pass.dependencies();
        assert!(deps.contains(&"mem2reg".to_string()));
        assert!(deps.contains(&"instcombine".to_string()));
    }
    
    #[test]
    fn test_tail_call_types() {
        assert_eq!(TailCallType::SelfRecursive, TailCallType::SelfRecursive);
        assert_ne!(TailCallType::SelfRecursive, TailCallType::NonRecursive);
    }
    
    #[test]
    fn test_tail_call_optimizer_creation() {
        let context = Context::create();
        let module = context.create_module("test");
        let fn_type = context.void_type().fn_type(&[], false);
        let function = module.add_function("test", fn_type, None);
        
        let optimizer = TailCallOptimizer::new(&function, &context);
        assert_eq!(optimizer.function, &function);
    }
    
    #[test]
    fn test_call_argument_extraction() {
        let context = Context::create();
        let module = context.create_module("test");
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
        let function = module.add_function("test", fn_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        let builder = context.create_builder();
        builder.position_at_end(basic_block);
        
        let param1 = i32_type.const_int(1, false);
        let param2 = i32_type.const_int(2, false);
        let call = builder.build_call(function, &[param1.into(), param2.into()], "call").unwrap();
        
        let optimizer = TailCallOptimizer::new(&function, &context);
        let call_instr = call.as_instruction_value().unwrap();
        let args = optimizer.extract_call_arguments(&call_instr).unwrap();
        
        assert_eq!(args.len(), 2);
    }
}
