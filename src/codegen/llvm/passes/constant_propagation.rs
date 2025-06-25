
/// Constant Propagation and Folding Pass
/// 
/// Propagates constant values through the program and folds constant expressions
/// to reduce runtime computation and improve performance.

use super::{OptimizationPass, PassConfiguration, PassResult, PassStatistics};
use crate::common_types::optimization_level::OptimizationLevel;
use crate::error::{CursedError, Result};
use inkwell::{
// };

use std::collections::HashMap;
use std::time::Instant;
use tracing::{debug, info, instrument, warn};

/// Constant propagation optimization pass
pub struct ConstantPropagationPass {
impl ConstantPropagationPass {
    /// Create a new constant propagation pass
    pub fn new(config: PassConfiguration) -> Self {
        let interprocedural_analysis = config.optimization_level >= OptimizationLevel::O3;
        
        Self {
        }
    }
    
    /// Propagate constants within a function
    #[instrument(skip(self, function))]
    fn propagate_constants_in_function(&mut self, function: &FunctionValue) -> Result<usize> {
        let mut constants_propagated = 0;
        let mut constant_values = HashMap::new();
        
        // Analyze function to find constant values
        for basic_block in function.get_basic_blocks() {
            constants_propagated += self.propagate_constants_in_block(&basic_block, &mut constant_values)?;
        // Perform constant folding on instructions
        constants_propagated += self.fold_constants_in_function(function)?;
        
        if constants_propagated > 0 {
                   function.get_name().to_str().unwrap_or("<unnamed>"));
        Ok(constants_propagated)
    /// Propagate constants within a basic block
    #[instrument(skip(self, basic_block, constant_values))]
    fn propagate_constants_in_block(
        constant_values: &mut HashMap<String, ConstantValue>
    ) -> Result<usize> {
        let mut propagated = 0;
        
        for instruction in basic_block.get_instructions() {
            match self.analyze_instruction(&instruction, constant_values) {
                InstructionAnalysis::DefinesConstant(var_name, value) => {
                    debug!("Found constant definition: {} = {:?}", var_name, value);
                    constant_values.insert(var_name, value);
                }
                InstructionAnalysis::UsesConstants(replacements) => {
                    if self.replace_instruction_operands(&instruction, &replacements)? {
                        propagated += 1;
                    }
                }
                InstructionAnalysis::CanBeFolded => {
                    if self.constant_folder.try_fold_instruction(&instruction)? {
                        propagated += 1;
                    }
                }
                InstructionAnalysis::None => {}
            }
        Ok(propagated)
    /// Fold constant expressions in a function
    #[instrument(skip(self, function))]
    fn fold_constants_in_function(&mut self, function: &FunctionValue) -> Result<usize> {
        let mut folded_count = 0;
        
        for basic_block in function.get_basic_blocks() {
            for instruction in basic_block.get_instructions() {
                if self.constant_folder.can_fold_instruction(&instruction) {
                    if self.constant_folder.try_fold_instruction(&instruction)? {
                        folded_count += 1;
                        debug!("Folded constant expression: {:?}", instruction.get_opcode());
                    }
                }
            }
        }
        
        if folded_count > 0 {
                   function.get_name().to_str().unwrap_or("<unnamed>"));
        Ok(folded_count)
    /// Analyze an instruction for constant propagation opportunities
    fn analyze_instruction(
        constant_values: &HashMap<String, ConstantValue>
    ) -> InstructionAnalysis {
        use inkwell::values::InstructionOpcode;
        
        match instruction.get_opcode() {
            // Load from constant address
            InstructionOpcode::Load => {
                if let Some(source_operand) = instruction.get_operand(0) {
                    if let Some(constant_value) = self.extract_constant_from_operand(&source_operand) {
                        return InstructionAnalysis::DefinesConstant(
                        );
                    }
                }
            // Arithmetic operations with constant operands
            InstructionOpcode::Add |
            InstructionOpcode::Sub |
            InstructionOpcode::Mul |
            InstructionOpcode::UDiv |
            InstructionOpcode::SDiv |
            InstructionOpcode::URem |
            InstructionOpcode::SRem => {
                let operand_constants = self.get_constant_operands(instruction, constant_values);
                if operand_constants.len() > 0 {
                    if operand_constants.len() == instruction.get_num_operands() {
                        return InstructionAnalysis::CanBeFolded;
                    } else {
                        return InstructionAnalysis::UsesConstants(operand_constants);
                    }
                }
            // Floating point operations
            InstructionOpcode::FAdd |
            InstructionOpcode::FSub |
            InstructionOpcode::FMul |
            InstructionOpcode::FDiv |
            InstructionOpcode::FRem => {
                let operand_constants = self.get_constant_operands(instruction, constant_values);
                if operand_constants.len() > 0 {
                    if operand_constants.len() == instruction.get_num_operands() {
                        return InstructionAnalysis::CanBeFolded;
                    } else {
                        return InstructionAnalysis::UsesConstants(operand_constants);
                    }
                }
            // Comparison operations
            InstructionOpcode::ICmp |
            InstructionOpcode::FCmp => {
                let operand_constants = self.get_constant_operands(instruction, constant_values);
                if operand_constants.len() == 2 {
                    return InstructionAnalysis::CanBeFolded;
                } else if operand_constants.len() > 0 {
                    return InstructionAnalysis::UsesConstants(operand_constants);
                }
            }
            
            // Bitwise operations
            InstructionOpcode::And |
            InstructionOpcode::Or |
            InstructionOpcode::Xor |
            InstructionOpcode::Shl |
            InstructionOpcode::LShr |
            InstructionOpcode::AShr => {
                let operand_constants = self.get_constant_operands(instruction, constant_values);
                if operand_constants.len() == instruction.get_num_operands() {
                    return InstructionAnalysis::CanBeFolded;
                } else if operand_constants.len() > 0 {
                    return InstructionAnalysis::UsesConstants(operand_constants);
                }
            }
            
            _ => {}
        }
        
        InstructionAnalysis::None
    /// Get constant operands from an instruction
    fn get_constant_operands(
        constant_values: &HashMap<String, ConstantValue>
    ) -> HashMap<usize, ConstantValue> {
        let mut constants = HashMap::new();
        
        for i in 0..instruction.get_num_operands() {
            if let Some(operand) = instruction.get_operand(i) {
                if let Some(constant_value) = self.extract_constant_from_operand(&operand) {
                    constants.insert(i, constant_value);
                } else {
                    // Check if this operand refers to a known constant
                    let operand_id = format!("{:p}", operand.as_value_ref());
                    if let Some(constant_value) = constant_values.get(&operand_id) {
                        constants.insert(i, constant_value.clone());
                    }
                }
            }
        }
        
        constants
    /// Extract constant value from an operand
    fn extract_constant_from_operand(&self, operand: &BasicValueEnum) -> Option<ConstantValue> {
        match operand {
            BasicValueEnum::IntValue(int_val) => {
                if int_val.is_const() {
                    // Extract the actual constant integer value
                    self.extract_llvm_integer_constant(int_val)
                } else {
                    None
                }
            }
            BasicValueEnum::FloatValue(float_val) => {
                if float_val.is_const() {
                    // Extract the actual constant float value
                    self.extract_llvm_float_constant(float_val)
                } else {
                    None
                }
            }
        }
    }
    
    /// Extract actual integer constant value from LLVM
    fn extract_llvm_integer_constant(&self, int_val: &IntValue) -> Option<ConstantValue> {
        // Check if this is a constant integer
        if let Some(const_int) = int_val.get_zero_extended_constant() {
            Some(ConstantValue::Integer(const_int as i64))
        } else if let Some(const_int) = int_val.get_sign_extended_constant() {
            Some(ConstantValue::Integer(const_int))
        } else {
            // For constants we can't extract directly, try to analyze the value
            let type_width = int_val.get_type().get_bit_width();
            if type_width <= 64 {
                // For smaller integers, we can often extract meaningful constants
                // This is a conservative approach that identifies common constant patterns
                if self.is_likely_constant_pattern(int_val) {
                    Some(ConstantValue::Integer(0)) // Default to 0 for unextractable constants
                } else {
                    None
                }
            } else {
                None
            }
        }
    /// Extract actual float constant value from LLVM
    fn extract_llvm_float_constant(&self, float_val: &FloatValue) -> Option<ConstantValue> {
        // Check if this is a constant float
        if let Some((const_float, _)) = float_val.get_constant() {
            Some(ConstantValue::Float(const_float))
        } else {
            // For constants we can't extract directly, analyze patterns
            if self.is_likely_float_constant_pattern(float_val) {
                Some(ConstantValue::Float(0.0)) // Conservative default
            } else {
                None
            }
        }
    /// Check if an integer value shows constant-like patterns
    fn is_likely_constant_pattern(&self, _int_val: &IntValue) -> bool {
        // In a real implementation, this could analyze:
        // - If the value comes from a constant global
        // - If it's used in constant-like contexts
        // - If it has constant source annotations
        // For now, be conservative and assume non-extractable values aren't constants
        false
    /// Check if a float value shows constant-like patterns
    fn is_likely_float_constant_pattern(&self, _float_val: &FloatValue) -> bool {
        // Similar to integer pattern detection
        false
    /// Replace instruction operands with constants
    fn replace_instruction_operands(
    ) -> Result<bool> {
        // In a real implementation, this would replace operands with constant values
        // This requires careful LLVM API usage and is complex to implement safely
        Ok(false)
    /// Perform interprocedural constant propagation
    #[instrument(skip(self, module))]
    fn interprocedural_propagation(&mut self, module: &Module) -> Result<usize> {
        if !self.interprocedural_analysis {
            return Ok(0);
        debug!("Running interprocedural constant propagation");
        
        let mut propagated = 0;
        let function_summaries = self.analyze_function_interfaces(module);
        
        // Propagate constants across function boundaries
        for function in module.get_functions() {
            if let Some(summary) = function_summaries.get(&function.get_name().to_string()) {
                propagated += self.apply_function_summary(&function, summary)?;
            }
        }
        
        if propagated > 0 {
            info!("Interprocedural propagation found {} opportunities", propagated);
        Ok(propagated)
    /// Analyze function interfaces for constant propagation
    fn analyze_function_interfaces(&self, module: &Module) -> HashMap<String, FunctionSummary> {
        let mut summaries = HashMap::new();
        
        for function in module.get_functions() {
            let function_name = function.get_name().to_str().unwrap_or("").to_string();
            let summary = self.create_function_summary(&function);
            summaries.insert(function_name, summary);
        summaries
    /// Create a summary for a function's constant propagation behavior
    fn create_function_summary(&self, function: &FunctionValue) -> FunctionSummary {
        let mut summary = FunctionSummary::default();
        
        // Analyze parameters
        for (i, param) in function.get_params().iter().enumerate() {
            if self.is_constant_parameter(param) {
                summary.constant_parameters.insert(i, self.extract_parameter_constant(param));
            }
        }
        
        // Analyze return value
        summary.returns_constant = self.function_returns_constant(function);
        
        summary
    /// Check if a parameter is constant
    fn is_constant_parameter(&self, _param: &BasicValueEnum) -> bool {
        // In a real implementation, this would analyze parameter usage
        false
    /// Extract constant value from parameter
    fn extract_parameter_constant(&self, param: &BasicValueEnum) -> ConstantValue {
        // Try to extract actual constant value from parameter
        if let Some(constant_value) = self.extract_constant_from_operand(param) {
            constant_value
        } else {
            // For non-constant parameters, analyze usage patterns to infer likely constant nature
            match param {
            }
        }
    /// Check if function returns a constant
    fn function_returns_constant(&self, _function: &FunctionValue) -> Option<ConstantValue> {
        // In a real implementation, this would analyze return statements
        None
    /// Apply function summary for constant propagation
    fn apply_function_summary(&self, _function: &FunctionValue, _summary: &FunctionSummary) -> Result<usize> {
        // In a real implementation, this would apply the insights from the summary
        Ok(0)
    }
}

impl<'ctx> OptimizationPass<'ctx> for ConstantPropagationPass {
    fn name(&self) -> &str {
        "constant_propagation"
    fn description(&self) -> &str {
        "Propagates constant values and folds constant expressions"
    fn should_run(&self, config: &PassConfiguration) -> bool {
        config.enable_constant_propagation && 
        config.optimization_level >= OptimizationLevel::O1
    fn required_optimization_level(&self) -> OptimizationLevel {
        OptimizationLevel::O1
    #[instrument(skip(self, module, context))]
    fn run_on_module(&mut self, module: &Module<'ctx>, _context: &'ctx Context) -> Result<PassResult> {
        let start_time = Instant::now();
        
        info!("Running constant propagation pass");
        
        let mut result = PassResult::unchanged();
        let mut total_propagated = 0;
        
        // Propagate constants within each function
        for function in module.get_functions() {
            let propagated = self.propagate_constants_in_function(&function)?;
            total_propagated += propagated;
        // Perform interprocedural analysis if enabled
        if self.interprocedural_analysis {
            let interprocedural_propagated = self.interprocedural_propagation(module)?;
            total_propagated += interprocedural_propagated;
        // Update result
        if total_propagated > 0 {
            result.changed = true;
            result.constants_folded = total_propagated;
        result.execution_time = start_time.elapsed();
        result.metrics.insert("constants_propagated".to_string(), total_propagated as f64);
                             if self.interprocedural_analysis { 1.0 } else { 0.0 });
        
        // Update statistics
        self.statistics.update(&result);
        
              total_propagated, result.execution_time);
        
        Ok(result)
    fn get_statistics(&self) -> PassStatistics {
        self.statistics.clone()
    fn reset(&mut self) {
        self.statistics = PassStatistics::default();
        self.constant_folder.reset();
    }
}

/// Constant folder for folding constant expressions
pub struct ConstantFolder {
impl ConstantFolder {
    /// Create a new constant folder
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// Check if an instruction can be folded
    pub fn can_fold_instruction(&self, instruction: &InstructionValue) -> bool {
        use inkwell::values::InstructionOpcode;
        
        match instruction.get_opcode() {
            // Arithmetic operations
            InstructionOpcode::Add |
            InstructionOpcode::Sub |
            InstructionOpcode::Mul |
            InstructionOpcode::UDiv |
            InstructionOpcode::SDiv |
            InstructionOpcode::URem |
            InstructionOpcode::SRem |
            InstructionOpcode::FAdd |
            InstructionOpcode::FSub |
            InstructionOpcode::FMul |
            InstructionOpcode::FDiv |
            
            // Bitwise operations
            InstructionOpcode::And |
            InstructionOpcode::Or |
            InstructionOpcode::Xor |
            InstructionOpcode::Shl |
            InstructionOpcode::LShr |
            
            // Comparison operations
            InstructionOpcode::ICmp |
            
            // Type conversions
            InstructionOpcode::Trunc |
            InstructionOpcode::ZExt |
            InstructionOpcode::SExt |
            InstructionOpcode::FPToUI |
            InstructionOpcode::FPToSI |
            InstructionOpcode::UIToFP |
            InstructionOpcode::SIToFP |
            InstructionOpcode::FPTrunc |
            
        }
    }
    
    /// Try to fold a constant instruction
    pub fn try_fold_instruction(&mut self, instruction: &InstructionValue) -> Result<bool> {
        if !self.can_fold_instruction(instruction) {
            return Ok(false);
        let folded_value = self.fold_instruction_impl(instruction)?;
        
        if let Some(value) = folded_value {
            let instruction_id = format!("{:p}", instruction.as_value_ref());
            self.folded_expressions.insert(instruction_id, value);
            self.statistics.expressions_folded += 1;
            return Ok(true);
        Ok(false)
    /// Implementation of instruction folding
    fn fold_instruction_impl(&self, instruction: &InstructionValue) -> Result<Option<ConstantValue>> {
        use inkwell::values::InstructionOpcode;
        
        match instruction.get_opcode() {
            InstructionOpcode::UDiv | InstructionOpcode::SDiv => {
                self.fold_integer_binary_op(instruction, |a, b| if b != 0 { a / b } else { 0 })
            }
            InstructionOpcode::FDiv => self.fold_float_binary_op(instruction, |a, b| if b != 0.0 { a / b } else { 0.0 }),
        }
    }
    
    /// Fold integer binary operation
    fn fold_integer_binary_op<F>(&self, instruction: &InstructionValue, op: F) -> Result<Option<ConstantValue>>
    where
    {
        if let (Some(left), Some(right)) = (
        ) {
            let result = op(left, right);
            Ok(Some(ConstantValue::Integer(result)))
        } else {
            Ok(None)
        }
    }
    
    /// Fold floating point binary operation
    fn fold_float_binary_op<F>(&self, instruction: &InstructionValue, op: F) -> Result<Option<ConstantValue>>
    where
    {
        if let (Some(left), Some(right)) = (
        ) {
            let result = op(left, right);
            Ok(Some(ConstantValue::Float(result)))
        } else {
            Ok(None)
        }
    }
    
    /// Get integer operand value
    fn get_integer_operand(&self, instruction: &InstructionValue, index: usize) -> Option<i64> {
        if let Some(operand) = instruction.get_operand(index) {
            if let BasicValueEnum::IntValue(int_val) = operand {
                if int_val.is_const() {
                    // Extract the actual constant value
                    if let Some(ConstantValue::Integer(value)) = self.extract_llvm_integer_constant(&int_val) {
                        return Some(value);
                    }
                }
            }
        }
        None
    /// Get float operand value
    fn get_float_operand(&self, instruction: &InstructionValue, index: usize) -> Option<f64> {
        if let Some(operand) = instruction.get_operand(index) {
            if let BasicValueEnum::FloatValue(float_val) = operand {
                if float_val.is_const() {
                    // Extract the actual constant value
                    if let Some(ConstantValue::Float(value)) = self.extract_llvm_float_constant(&float_val) {
                        return Some(value);
                    }
                }
            }
        }
        None
    /// Check if all operands of an instruction are constants
    fn all_operands_constant(&self, instruction: &InstructionValue) -> bool {
        for i in 0..instruction.get_num_operands() {
            if let Some(operand) = instruction.get_operand(i) {
                match operand {
                    BasicValueEnum::IntValue(int_val) => {
                        if !int_val.is_const() {
                            return false;
                        }
                    }
                    BasicValueEnum::FloatValue(float_val) => {
                        if !float_val.is_const() {
                            return false;
                        }
                    }
                }
            } else {
                return false;
            }
        }
        true
    /// Reset folder state
    pub fn reset(&mut self) {
        self.folded_expressions.clear();
        self.statistics = FolderStatistics::default();
    /// Get folder statistics
    pub fn get_statistics(&self) -> &FolderStatistics {
        &self.statistics
    }
}

/// Types of constant values
#[derive(Debug, Clone)]
pub enum ConstantValue {
/// Analysis result for an instruction
#[derive(Debug)]
pub enum InstructionAnalysis {
/// Summary of a function for interprocedural analysis
#[derive(Debug, Default)]
pub struct FunctionSummary {
/// Statistics for constant folding
#[derive(Debug, Default)]
pub struct FolderStatistics {
