//! Function inlining optimization pass

use crate::error::{CursedError, Result};
use inkwell::{
    context::Context,
    module::Module,
    values::{FunctionValue, BasicValueEnum, CallSiteValue},
    basic_block::BasicBlock,
    builder::Builder,
    IntPredicate,
};
use std::collections::{HashMap, HashSet};

/// Function inlining pass for CURSED
pub struct InliningPass<'ctx> {
    context: &'ctx Context,
    inline_threshold: u32,
    size_threshold: u32,
    recursive_inline_limit: u32,
    call_graph: CallGraph,
    inlining_decisions: HashMap<String, InliningDecision>,
}

impl<'ctx> InliningPass<'ctx> {
    /// Create a new inlining pass
    pub fn new(context: &'ctx Context, inline_threshold: u32) -> Self {
        Self {
            context,
            inline_threshold,
            size_threshold: inline_threshold / 2,
            recursive_inline_limit: 3,
            call_graph: CallGraph::new(),
            inlining_decisions: HashMap::new(),
        }
    }
    
    /// Run inlining pass on a module
    pub fn run(&mut self, module: &Module<'ctx>) -> Result<InliningResult> {
        let mut result = InliningResult::default();
        
        // Build call graph
        self.build_call_graph(module)?;
        
        // Analyze functions for inlining decisions
        self.analyze_functions(module)?;
        
        // Perform inlining based on decisions
        let functions_to_inline = self.get_functions_to_inline();
        
        for (caller_name, inlined_calls) in functions_to_inline {
            if let Some(caller) = module.get_function(&caller_name) {
                let inline_count = self.inline_calls_in_function(caller, &inlined_calls)?;
                result.functions_inlined += inline_count;
                result.total_calls_inlined += inlined_calls.len() as u32;
            }
        }
        
        // Clean up unused functions
        result.functions_removed = self.remove_unused_functions(module)?;
        
        Ok(result)
    }
    
    /// Build call graph for the module
    fn build_call_graph(&mut self, module: &Module<'ctx>) -> Result<()> {
        self.call_graph.clear();
        
        for function in module.get_functions() {
            let func_name = function.get_name().to_str()
                .map_err(|_| CursedError::runtime_error("Invalid function name"))?
                .to_string();
            
            self.call_graph.add_function(func_name.clone());
            
            // Skip external functions
            if function.get_first_basic_block().is_none() {
                continue;
            }
            
            // Find all function calls
            for basic_block in function.get_basic_blocks() {
                for instruction in basic_block.get_instructions() {
                    if let Some(call_site) = instruction.as_call_site_value() {
                        if let Some(called_func) = call_site.try_as_basic_value().left() {
                            if let Some(called_function) = called_func.as_function_value() {
                                let called_name = called_function.get_name().to_str()
                                    .map_err(|_| CursedError::runtime_error("Invalid called function name"))?
                                    .to_string();
                                
                                self.call_graph.add_call(func_name.clone(), called_name);
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Analyze functions and make inlining decisions
    fn analyze_functions(&mut self, module: &Module<'ctx>) -> Result<()> {
        for function in module.get_functions() {
            let func_name = function.get_name().to_str()
                .map_err(|_| CursedError::runtime_error("Invalid function name"))?
                .to_string();
            
            let analyzer = CallSiteAnalyzer::new(&function);
            let heuristics = InliningHeuristics::new(self.inline_threshold, self.size_threshold);
            
            let decision = heuristics.should_inline(&analyzer, &self.call_graph, &func_name)?;
            self.inlining_decisions.insert(func_name, decision);
        }
        
        Ok(())
    }
    
    /// Get functions and their calls to inline
    fn get_functions_to_inline(&self) -> HashMap<String, Vec<String>> {
        let mut result = HashMap::new();
        
        for (caller, decision) in &self.inlining_decisions {
            match decision {
                InliningDecision::Inline(targets) => {
                    if !targets.is_empty() {
                        result.insert(caller.clone(), targets.clone());
                    }
                }
                _ => {}
            }
        }
        
        result
    }
    
    /// Inline calls in a specific function
    fn inline_calls_in_function(&self, function: FunctionValue<'ctx>, targets: &[String]) -> Result<u32> {
        let mut inlined_count = 0;
        
        // Find all call sites in the function
        let mut call_sites = Vec::new();
        
        for basic_block in function.get_basic_blocks() {
            for instruction in basic_block.get_instructions() {
                if let Some(call_site) = instruction.as_call_site_value() {
                    if let Some(called_value) = call_site.try_as_basic_value().left() {
                        if let Some(called_func) = called_value.as_function_value() {
                            let called_name = called_func.get_name().to_str()
                                .map_err(|_| CursedError::runtime_error("Invalid called function name"))?;
                            
                            if targets.contains(&called_name.to_string()) {
                                call_sites.push((call_site, called_func));
                            }
                        }
                    }
                }
            }
        }
        
        // Perform inlining for each call site
        for (call_site, called_func) in call_sites {
            if self.inline_call_site(call_site, called_func)? {
                inlined_count += 1;
            }
        }
        
        Ok(inlined_count)
    }
    
    /// Inline a specific call site
    fn inline_call_site(&self, call_site: CallSiteValue<'ctx>, called_func: FunctionValue<'ctx>) -> Result<bool> {
        // This is a simplified inlining implementation
        // In a real implementation, we would:
        // 1. Clone the called function's body
        // 2. Replace parameters with arguments
        // 3. Handle return values
        // 4. Update phi nodes and control flow
        // 5. Handle debug information
        
        // For now, we'll just mark it as inlined and remove the call
        // This is not a complete implementation but demonstrates the structure
        
        let builder = self.context.create_builder();
        
        // Position builder after the call site
        if let Some(next_instruction) = call_site.as_instruction_value().get_next_instruction() {
            builder.position_before(&next_instruction);
        } else {
            // If it's the last instruction, position at the end of the block
            let parent_block = call_site.as_instruction_value().get_parent()
                .ok_or_else(|| CursedError::runtime_error("Call site has no parent block"))?;
            builder.position_at_end(parent_block);
        }
        
        // For simple functions, we can attempt basic inlining
        if self.can_simple_inline(&called_func) {
            self.perform_simple_inline(&builder, call_site, called_func)?;
            return Ok(true);
        }
        
        Ok(false)
    }
    
    /// Check if a function can be simply inlined
    fn can_simple_inline(&self, function: &FunctionValue<'ctx>) -> bool {
        // Only inline very simple functions for this implementation
        let basic_blocks: Vec<_> = function.get_basic_blocks().collect();
        
        // Must have exactly one basic block
        if basic_blocks.len() != 1 {
            return false;
        }
        
        let block = &basic_blocks[0];
        let instructions: Vec<_> = block.get_instructions().collect();
        
        // Must have very few instructions (just return, or simple computation + return)
        if instructions.len() > 3 {
            return false;
        }
        
        // Check that it's just simple operations
        for instruction in &instructions {
            match instruction.get_opcode() {
                inkwell::values::InstructionOpcode::Ret |
                inkwell::values::InstructionOpcode::Add |
                inkwell::values::InstructionOpcode::Sub |
                inkwell::values::InstructionOpcode::Mul |
                inkwell::values::InstructionOpcode::SDiv |
                inkwell::values::InstructionOpcode::Load |
                inkwell::values::InstructionOpcode::Store => {
                    // These are safe to inline
                }
                _ => return false,
            }
        }
        
        true
    }
    
    /// Perform simple inlining
    fn perform_simple_inline(&self, builder: &Builder<'ctx>, call_site: CallSiteValue<'ctx>, called_func: FunctionValue<'ctx>) -> Result<()> {
        // This is a very basic implementation that handles only simple cases
        let basic_block = called_func.get_first_basic_block()
            .ok_or_else(|| CursedError::runtime_error("Function has no basic blocks"))?;
        
        let instructions: Vec<_> = basic_block.get_instructions().collect();
        let args: Vec<_> = call_site.as_instruction_value().get_operands();
        
        // Map function parameters to call arguments
        let mut value_map = HashMap::new();
        for (i, param) in called_func.get_params().iter().enumerate() {
            if i < args.len() {
                if let Some(arg_value) = args[i].left() {
                    value_map.insert(param.as_basic_value_enum(), arg_value);
                }
            }
        }
        
        // Clone and execute the instructions
        let mut return_value = None;
        
        for instruction in &instructions {
            match instruction.get_opcode() {
                inkwell::values::InstructionOpcode::Ret => {
                    // Handle return instruction
                    if instruction.get_num_operands() > 0 {
                        if let Some(ret_val) = instruction.get_operand(0) {
                            if let Some(mapped_val) = value_map.get(&ret_val.left().unwrap()) {
                                return_value = Some(*mapped_val);
                            } else {
                                return_value = ret_val.left();
                            }
                        }
                    }
                }
                _ => {
                    // For other instructions, we'd need to recreate them with mapped values
                    // This is a complex process that would require handling each instruction type
                }
            }
        }
        
        // Replace the call with the return value if any
        if let Some(ret_val) = return_value {
            call_site.as_instruction_value().replace_all_uses_with(&ret_val);
        }
        
        // Remove the call instruction
        unsafe {
            call_site.as_instruction_value().delete();
        }
        
        Ok(())
    }
    
    /// Remove unused functions after inlining
    fn remove_unused_functions(&self, module: &Module<'ctx>) -> Result<u32> {
        let mut removed_count = 0;
        let mut functions_to_remove = Vec::new();
        
        // Collect functions that are no longer used
        for function in module.get_functions() {
            let func_name = function.get_name().to_str()
                .map_err(|_| CursedError::runtime_error("Invalid function name"))?;
            
            // Skip main function and external functions
            if func_name == "main" || function.get_first_basic_block().is_none() {
                continue;
            }
            
            // Check if function is still referenced
            if !self.call_graph.is_function_referenced(func_name) {
                functions_to_remove.push(function);
            }
        }
        
        // Remove unused functions
        for function in functions_to_remove {
            unsafe {
                function.delete();
            }
            removed_count += 1;
        }
        
        Ok(removed_count)
    }
}

/// Call site analyzer for inlining decisions
pub struct CallSiteAnalyzer<'ctx> {
    function: &'ctx FunctionValue<'ctx>,
    size: u32,
    call_count: u32,
    complexity_score: f64,
}

impl<'ctx> CallSiteAnalyzer<'ctx> {
    pub fn new(function: &'ctx FunctionValue<'ctx>) -> Self {
        let (size, call_count) = Self::analyze_function(function);
        let complexity_score = Self::calculate_complexity(function);
        
        Self {
            function,
            size,
            call_count,
            complexity_score,
        }
    }
    
    fn analyze_function(function: &FunctionValue<'ctx>) -> (u32, u32) {
        let mut size = 0;
        let mut call_count = 0;
        
        for basic_block in function.get_basic_blocks() {
            for instruction in basic_block.get_instructions() {
                size += 1;
                if instruction.get_opcode() == inkwell::values::InstructionOpcode::Call {
                    call_count += 1;
                }
            }
        }
        
        (size, call_count)
    }
    
    fn calculate_complexity(function: &FunctionValue<'ctx>) -> f64 {
        let basic_block_count = function.count_basic_blocks() as f64;
        let mut instruction_count = 0.0;
        let mut branch_count = 0.0;
        
        for basic_block in function.get_basic_blocks() {
            for instruction in basic_block.get_instructions() {
                instruction_count += 1.0;
                match instruction.get_opcode() {
                    inkwell::values::InstructionOpcode::Br |
                    inkwell::values::InstructionOpcode::CondBr |
                    inkwell::values::InstructionOpcode::Switch => {
                        branch_count += 1.0;
                    }
                    _ => {}
                }
            }
        }
        
        // Simple complexity metric
        basic_block_count * 2.0 + instruction_count + branch_count * 3.0
    }
    
    pub fn get_size(&self) -> u32 {
        self.size
    }
    
    pub fn get_call_count(&self) -> u32 {
        self.call_count
    }
    
    pub fn get_complexity(&self) -> f64 {
        self.complexity_score
    }
    
    pub fn is_leaf_function(&self) -> bool {
        self.call_count == 0
    }
    
    pub fn is_small_function(&self, threshold: u32) -> bool {
        self.size <= threshold
    }
}

/// Inlining heuristics for making decisions
pub struct InliningHeuristics {
    inline_threshold: u32,
    size_threshold: u32,
    complexity_threshold: f64,
}

impl InliningHeuristics {
    pub fn new(inline_threshold: u32, size_threshold: u32) -> Self {
        Self {
            inline_threshold,
            size_threshold,
            complexity_threshold: 100.0,
        }
    }
    
    pub fn should_inline(&self, analyzer: &CallSiteAnalyzer, call_graph: &CallGraph, func_name: &str) -> Result<InliningDecision> {
        // Don't inline recursive functions
        if call_graph.is_recursive(func_name) {
            return Ok(InliningDecision::NoInline(InliningReason::Recursive));
        }
        
        // Don't inline large functions
        if analyzer.get_size() > self.inline_threshold {
            return Ok(InliningDecision::NoInline(InliningReason::TooLarge));
        }
        
        // Don't inline complex functions
        if analyzer.get_complexity() > self.complexity_threshold {
            return Ok(InliningDecision::NoInline(InliningReason::TooComplex));
        }
        
        // Inline small leaf functions
        if analyzer.is_leaf_function() && analyzer.is_small_function(self.size_threshold) {
            let callees = call_graph.get_callees(func_name);
            return Ok(InliningDecision::Inline(callees));
        }
        
        // Inline small functions with few calls
        if analyzer.get_size() <= self.size_threshold && analyzer.get_call_count() <= 2 {
            let callees = call_graph.get_callees(func_name);
            return Ok(InliningDecision::Inline(callees));
        }
        
        Ok(InliningDecision::NoInline(InliningReason::Heuristic))
    }
}

/// Call graph representation
pub struct CallGraph {
    functions: HashSet<String>,
    calls: HashMap<String, Vec<String>>,
    reverse_calls: HashMap<String, Vec<String>>,
}

impl CallGraph {
    pub fn new() -> Self {
        Self {
            functions: HashSet::new(),
            calls: HashMap::new(),
            reverse_calls: HashMap::new(),
        }
    }
    
    pub fn clear(&mut self) {
        self.functions.clear();
        self.calls.clear();
        self.reverse_calls.clear();
    }
    
    pub fn add_function(&mut self, name: String) {
        self.functions.insert(name);
    }
    
    pub fn add_call(&mut self, caller: String, callee: String) {
        self.calls.entry(caller.clone()).or_insert_with(Vec::new).push(callee.clone());
        self.reverse_calls.entry(callee).or_insert_with(Vec::new).push(caller);
    }
    
    pub fn get_callees(&self, function: &str) -> Vec<String> {
        self.calls.get(function).cloned().unwrap_or_default()
    }
    
    pub fn is_recursive(&self, function: &str) -> bool {
        self.is_recursive_helper(function, &mut HashSet::new())
    }
    
    fn is_recursive_helper(&self, function: &str, visited: &mut HashSet<String>) -> bool {
        if visited.contains(function) {
            return true;
        }
        
        visited.insert(function.to_string());
        
        if let Some(callees) = self.calls.get(function) {
            for callee in callees {
                if self.is_recursive_helper(callee, visited) {
                    return true;
                }
            }
        }
        
        visited.remove(function);
        false
    }
    
    pub fn is_function_referenced(&self, function: &str) -> bool {
        self.reverse_calls.get(function).map_or(false, |callers| !callers.is_empty())
    }
}

/// Inlining decision
#[derive(Debug, Clone)]
pub enum InliningDecision {
    Inline(Vec<String>),
    NoInline(InliningReason),
}

/// Reasons for not inlining
#[derive(Debug, Clone)]
pub enum InliningReason {
    TooLarge,
    TooComplex,
    Recursive,
    External,
    Heuristic,
}

/// Result of inlining pass
#[derive(Debug, Default)]
pub struct InliningResult {
    pub functions_inlined: u32,
    pub total_calls_inlined: u32,
    pub functions_removed: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    
    #[test]
    fn test_call_graph() {
        let mut graph = CallGraph::new();
        graph.add_function("main".to_string());
        graph.add_function("helper".to_string());
        graph.add_call("main".to_string(), "helper".to_string());
        
        assert_eq!(graph.get_callees("main"), vec!["helper"]);
        assert!(graph.is_function_referenced("helper"));
        assert!(!graph.is_recursive("main"));
    }
    
    #[test]
    fn test_inlining_heuristics() {
        let heuristics = InliningHeuristics::new(100, 50);
        assert_eq!(heuristics.inline_threshold, 100);
        assert_eq!(heuristics.size_threshold, 50);
    }
}
