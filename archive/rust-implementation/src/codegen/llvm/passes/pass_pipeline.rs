//! Optimization pass pipeline management

use crate::error::{CursedError, Result};
use inkwell::{
    context::Context,
    module::Module,
    passes::PassManager,
    values::FunctionValue,
};
// use super::{mem2reg::Mem2RegPass, sroa::SroaPass}; // Disabled
use std::collections::HashMap;

/// Optimization pipeline that manages pass execution
pub struct OptimizationPipeline<'ctx> {
    context: &'ctx Context,
    function_passes: Vec<Box<dyn FunctionPass<'ctx> + 'ctx>>,
    module_passes: Vec<Box<dyn ModulePass<'ctx> + 'ctx>>,
    statistics: PipelineStatistics,
}

/// Function-level optimization pass trait
pub trait FunctionPass<'ctx> {
    fn name(&self) -> &str;
    fn run_on_function(&mut self, function: &FunctionValue<'ctx>) -> Result<bool>;
}

/// Module-level optimization pass trait
pub trait ModulePass<'ctx> {
    fn name(&self) -> &str;
    fn run_on_module(&mut self, module: &Module<'ctx>) -> Result<bool>;
}

/// Pipeline execution statistics
#[derive(Debug, Clone, Default)]
pub struct PipelineStatistics {
    pub passes_run: usize,
    pub functions_optimized: usize,
    pub modules_optimized: usize,
    pub total_changes: usize,
}

impl<'ctx> OptimizationPipeline<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        Self {
            context,
            function_passes: Vec::new(),
            module_passes: Vec::new(),
            statistics: PipelineStatistics::default(),
        }
    }
    
    /*
    pub fn add_mem2reg_pass(&mut self) {
        let pass = Mem2RegFunctionPass::new(self.context);
        self.function_passes.push(Box::new(pass));
    }
    
    pub fn add_sroa_pass(&mut self) {
        let pass = SroaFunctionPass::new(self.context);
        self.function_passes.push(Box::new(pass));
    }
    */
    
    pub fn run_on_module(&mut self, module: &Module<'ctx>) -> Result<bool> {
        let mut changed = false;
        
        // Run function passes on all functions
        for function in module.get_functions() {
            for pass in &mut self.function_passes {
                if pass.run_on_function(&function)? {
                    changed = true;
                    self.statistics.total_changes += 1;
                }
                self.statistics.passes_run += 1;
            }
            self.statistics.functions_optimized += 1;
        }
        
        // Run module passes
        for pass in &mut self.module_passes {
            if pass.run_on_module(module)? {
                changed = true;
                self.statistics.total_changes += 1;
            }  
            self.statistics.passes_run += 1;
        }
        
        self.statistics.modules_optimized += 1;
        Ok(changed)
    }
    
    pub fn get_statistics(&self) -> &PipelineStatistics {
        &self.statistics
    }
}

/*
/// Wrapper for Mem2Reg pass to implement FunctionPass trait
struct Mem2RegFunctionPass<'ctx> {
    pass: Mem2RegPass<'ctx>,
}

impl<'ctx> Mem2RegFunctionPass<'ctx> {
    fn new(context: &'ctx Context) -> Self {
        Self {
            pass: Mem2RegPass::new(context),
        }
    }
}

impl<'ctx> FunctionPass<'ctx> for Mem2RegFunctionPass<'ctx> {
    fn name(&self) -> &str {
        "mem2reg"
    }
    
    fn run_on_function(&mut self, function: &FunctionValue<'ctx>) -> Result<bool> {
        self.pass.run_on_function(function)
    }
}
*/

/*
/// Wrapper for SROA pass to implement FunctionPass trait
struct SroaFunctionPass<'ctx> {
    pass: SroaPass<'ctx>,
}

impl<'ctx> SroaFunctionPass<'ctx> {
    fn new(context: &'ctx Context) -> Self {
        Self {
            pass: SroaPass::new(context),
        }
    }
}

impl<'ctx> FunctionPass<'ctx> for SroaFunctionPass<'ctx> {
    fn name(&self) -> &str {
        "sroa"
    }
    
    fn run_on_function(&mut self, function: &FunctionValue<'ctx>) -> Result<bool> {
        self.pass.run_on_function(function)
    }
}
*/

/// Pipeline builder for configuring optimization sequences
pub struct PipelineBuilder<'ctx> {
    context: &'ctx Context,
    pipeline: OptimizationPipeline<'ctx>,
}

impl<'ctx> PipelineBuilder<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        Self {
            context,
            pipeline: OptimizationPipeline::new(context),
        }
    }
    
    pub fn with_basic_optimizations(mut self) -> Self {
        // self.pipeline.add_mem2reg_pass(); // Disabled
        self
    }
    
    pub fn with_advanced_optimizations(mut self) -> Self {
        // self.pipeline.add_mem2reg_pass(); // Disabled
        // self.pipeline.add_sroa_pass(); // Disabled
        self
    }
    
    pub fn build(self) -> OptimizationPipeline<'ctx> {
        self.pipeline
    }
}
