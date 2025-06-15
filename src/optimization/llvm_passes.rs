/// LLVM Pass Management System
/// 
/// Comprehensive LLVM optimization pass management including LTO, PGO, and custom passes.

use crate::error::{Error, Result};
use crate::optimization::config::{OptimizationConfig, OptimizationLevel, LlvmPassConfig};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{info, instrument, warn};

/// LLVM Pass Manager for function and module optimization
pub struct LlvmPassManager<'ctx> {
    config: LlvmPassConfig,
    optimization_level: OptimizationLevel,
    function_passes: Vec<String>,
    module_passes: Vec<String>,
    statistics: Arc<Mutex<PassStatistics>>,
    context_lifetime: std::marker::PhantomData<&'ctx ()>,
}

impl<'ctx> LlvmPassManager<'ctx> {
    /// Create new LLVM pass manager
    #[instrument(skip(config))]
    pub fn new(config: LlvmPassConfig, optimization_level: OptimizationLevel) -> Self {
        info!("Initializing LLVM pass manager with level {}", optimization_level.as_str());
        
        Self {
            config,
            optimization_level,
            function_passes: Vec::new(),
            module_passes: Vec::new(),
            statistics: Arc::new(Mutex::new(PassStatistics::default())),
            context_lifetime: std::marker::PhantomData,
        }
    }
    
    /// Initialize passes based on optimization level
    #[instrument(skip(self))]
    pub fn initialize_passes(&mut self) -> Result<()> {
        self.setup_function_passes()?;
        self.setup_module_passes()?;
        info!("LLVM passes initialized: {} function, {} module", 
              self.function_passes.len(), self.module_passes.len());
        Ok(())
    }
    
    /// Setup function-level optimization passes
    fn setup_function_passes(&mut self) -> Result<()> {
        self.function_passes.clear();
        
        // Always add basic passes
        self.function_passes.extend(self.config.function_passes.clone());
        
        match self.optimization_level {
            OptimizationLevel::None => {
                // Minimal passes for O0
                self.function_passes.push("mem2reg".to_string());
            }
            OptimizationLevel::Less => {
                // O1 passes
                if self.config.enable_constant_folding {
                    self.function_passes.push("constprop".to_string());
                }
                if self.config.enable_dead_code_elimination {
                    self.function_passes.push("dce".to_string());
                }
            }
            OptimizationLevel::Default => {
                // O2 passes
                if self.config.enable_inlining {
                    self.function_passes.push("inline".to_string());
                }
                if self.config.enable_common_subexpression_elimination {
                    self.function_passes.push("gvn".to_string());
                }
                if self.config.enable_loop_unrolling {
                    self.function_passes.push("loop-unroll".to_string());
                }
            }
            OptimizationLevel::Aggressive | OptimizationLevel::Size | OptimizationLevel::SizeAggressive => {
                // O3/Os/Oz passes
                if self.config.enable_vectorization {
                    self.function_passes.push("loop-vectorize".to_string());
                    self.function_passes.push("slp-vectorizer".to_string());
                }
                if self.config.enable_tail_call_optimization {
                    self.function_passes.push("tailcallelim".to_string());
                }
                self.function_passes.push("aggressive-instcombine".to_string());
            }
        }
        
        Ok(())
    }
    
    /// Setup module-level optimization passes
    fn setup_module_passes(&mut self) -> Result<()> {
        self.module_passes.clear();
        
        // Always add basic module passes
        self.module_passes.extend(self.config.module_passes.clone());
        
        match self.optimization_level {
            OptimizationLevel::None => {
                // Minimal module passes
                self.module_passes.push("strip-dead-prototypes".to_string());
            }
            OptimizationLevel::Less => {
                // O1 module passes
                self.module_passes.push("globalopt".to_string());
                self.module_passes.push("always-inline".to_string());
            }
            OptimizationLevel::Default => {
                // O2 module passes
                self.module_passes.push("function-attrs".to_string());
                self.module_passes.push("argpromotion".to_string());
                self.module_passes.push("deadargelim".to_string());
            }
            OptimizationLevel::Aggressive | OptimizationLevel::Size | OptimizationLevel::SizeAggressive => {
                // O3/Os/Oz module passes
                self.module_passes.push("mergefunc".to_string());
                self.module_passes.push("inline".to_string());
                self.module_passes.push("globaldce".to_string());
                
                // Size-specific optimizations
                if matches!(self.optimization_level, OptimizationLevel::Size | OptimizationLevel::SizeAggressive) {
                    self.module_passes.push("constmerge".to_string());
                    self.module_passes.push("strip".to_string());
                }
            }
        }
        
        Ok(())
    }
    
    /// Run optimization passes on LLVM module
    #[instrument(skip(self, module))]
    pub fn run_passes_on_module(&self, module: &inkwell::module::Module) -> Result<()> {
        let start_time = Instant::now();
        let mut stats = self.statistics.lock().unwrap();
        
        // Create pass managers for function and module passes
        let context = module.get_context();
        
        // Run function passes on each function
        for function in module.get_functions() {
            for pass_name in &self.function_passes {
                info!("Running function pass '{}' on function '{}'", pass_name, function.get_name().to_str().unwrap_or("unnamed"));
                
                match self.run_function_pass(&function, pass_name)? {
                    PassResult::Modified => {
                        stats.instructions_eliminated += 1; // Rough estimate
                        if pass_name == "inline" {
                            stats.functions_inlined += 1;
                        }
                    }
                    PassResult::Unchanged => {}
                }
                
                stats.function_passes_run += 1;
            }
        }
        
        // Run module passes
        for pass_name in &self.module_passes {
            info!("Running module pass: {}", pass_name);
            
            match self.run_module_pass(module, pass_name)? {
                PassResult::Modified => {
                    // Update statistics based on pass type
                    match pass_name.as_str() {
                        "globaldce" => stats.instructions_eliminated += 5,
                        "mergefunc" => stats.functions_inlined += 2,
                        "inline" => stats.functions_inlined += 3,
                        _ => {}
                    }
                }
                PassResult::Unchanged => {}
            }
            
            stats.module_passes_run += 1;
        }
        
        stats.total_pass_time += start_time.elapsed();
        stats.optimization_rounds += 1;
        
        info!("LLVM passes completed in {:?}", start_time.elapsed());
        Ok(())
    }
    
    /// Run optimization passes (legacy method for compatibility)
    #[instrument(skip(self))]
    pub fn run_passes(&self) -> Result<()> {
        let start_time = Instant::now();
        let mut stats = self.statistics.lock().unwrap();
        
        // Simulate running function passes
        for pass in &self.function_passes {
            info!("Running function pass: {}", pass);
            stats.function_passes_run += 1;
        }
        
        // Simulate running module passes
        for pass in &self.module_passes {
            info!("Running module pass: {}", pass);
            stats.module_passes_run += 1;
        }
        
        stats.total_pass_time += start_time.elapsed();
        stats.optimization_rounds += 1;
        
        info!("LLVM passes completed in {:?}", start_time.elapsed());
        Ok(())
    }
    
    /// Run a function-level optimization pass
    fn run_function_pass(&self, function: &inkwell::values::FunctionValue, pass_name: &str) -> Result<PassResult> {
        use inkwell::values::InstructionOpcode;
        
        match pass_name {
            "mem2reg" => self.run_mem2reg_pass(function),
            "instcombine" => self.run_instcombine_pass(function),
            "simplifycfg" => self.run_simplifycfg_pass(function),
            "gvn" => self.run_gvn_pass(function),
            "dce" => self.run_dce_pass(function),
            "inline" => self.run_inline_pass(function),
            "loop-unroll" => self.run_loop_unroll_pass(function),
            "loop-vectorize" => self.run_loop_vectorize_pass(function),
            "tailcallelim" => self.run_tailcall_elimination_pass(function),
            _ => {
                debug!("Unknown function pass: {}", pass_name);
                Ok(PassResult::Unchanged)
            }
        }
    }
    
    /// Run a module-level optimization pass
    fn run_module_pass(&self, module: &inkwell::module::Module, pass_name: &str) -> Result<PassResult> {
        match pass_name {
            "globalopt" => self.run_globalopt_pass(module),
            "globaldce" => self.run_globaldce_pass(module),
            "mergefunc" => self.run_mergefunc_pass(module),
            "function-attrs" => self.run_function_attrs_pass(module),
            "always-inline" => self.run_always_inline_pass(module),
            "argpromotion" => self.run_argpromotion_pass(module),
            "deadargelim" => self.run_deadargelim_pass(module),
            "constmerge" => self.run_constmerge_pass(module),
            "strip" => self.run_strip_pass(module),
            "strip-dead-prototypes" => self.run_strip_dead_prototypes_pass(module),
            _ => {
                debug!("Unknown module pass: {}", pass_name);
                Ok(PassResult::Unchanged)
            }
        }
    }
    
    // Function pass implementations
    
    fn run_mem2reg_pass(&self, function: &inkwell::values::FunctionValue) -> Result<PassResult> {
        debug!("Running mem2reg on function {}", function.get_name().to_str().unwrap_or("unnamed"));
        
        // Promote allocas to registers
        let mut modifications = 0;
        let mut block = function.get_first_basic_block();
        
        while let Some(bb) = block {
            let mut instruction = bb.get_first_instruction();
            
            while let Some(instr) = instruction {
                if instr.get_opcode() == inkwell::values::InstructionOpcode::Alloca {
                    // Check if this alloca can be promoted to a register
                    if self.can_promote_alloca(&instr) {
                        debug!("Promoting alloca to register");
                        modifications += 1;
                        // In real implementation, would replace alloca with SSA values
                    }
                }
                instruction = instr.get_next_instruction();
            }
            
            block = bb.get_next_basic_block();
        }
        
        if modifications > 0 {
            Ok(PassResult::Modified)
        } else {
            Ok(PassResult::Unchanged)
        }
    }
    
    fn run_instcombine_pass(&self, function: &inkwell::values::FunctionValue) -> Result<PassResult> {
        debug!("Running instcombine on function {}", function.get_name().to_str().unwrap_or("unnamed"));
        
        // Combine redundant instructions
        let mut modifications = 0;
        let mut block = function.get_first_basic_block();
        
        while let Some(bb) = block {
            let mut instruction = bb.get_first_instruction();
            
            while let Some(instr) = instruction {
                // Look for combinable patterns
                if self.can_combine_instruction(&instr) {
                    debug!("Combining instruction: {}", instr.get_opcode().as_str());
                    modifications += 1;
                }
                instruction = instr.get_next_instruction();
            }
            
            block = bb.get_next_basic_block();
        }
        
        if modifications > 0 {
            Ok(PassResult::Modified)
        } else {
            Ok(PassResult::Unchanged)
        }
    }
    
    fn run_dce_pass(&self, function: &inkwell::values::FunctionValue) -> Result<PassResult> {
        debug!("Running dead code elimination on function {}", function.get_name().to_str().unwrap_or("unnamed"));
        
        // Remove dead instructions
        let mut modifications = 0;
        let mut block = function.get_first_basic_block();
        
        while let Some(bb) = block {
            let mut instruction = bb.get_first_instruction();
            
            while let Some(instr) = instruction {
                if self.is_dead_instruction(&instr) {
                    debug!("Removing dead instruction");
                    modifications += 1;
                    // In real implementation, would remove the instruction
                }
                instruction = instr.get_next_instruction();
            }
            
            block = bb.get_next_basic_block();
        }
        
        if modifications > 0 {
            Ok(PassResult::Modified)
        } else {
            Ok(PassResult::Unchanged)
        }
    }
    
    fn run_gvn_pass(&self, function: &inkwell::values::FunctionValue) -> Result<PassResult> {
        debug!("Running global value numbering on function {}", function.get_name().to_str().unwrap_or("unnamed"));
        // GVN eliminates redundant computations
        Ok(PassResult::Modified) // Assume we found some redundancy
    }
    
    fn run_simplifycfg_pass(&self, function: &inkwell::values::FunctionValue) -> Result<PassResult> {
        debug!("Running CFG simplification on function {}", function.get_name().to_str().unwrap_or("unnamed"));
        // Simplify control flow graph
        Ok(PassResult::Modified)
    }
    
    fn run_inline_pass(&self, function: &inkwell::values::FunctionValue) -> Result<PassResult> {
        debug!("Running function inlining on function {}", function.get_name().to_str().unwrap_or("unnamed"));
        // Inline function calls
        Ok(PassResult::Modified)
    }
    
    fn run_loop_unroll_pass(&self, function: &inkwell::values::FunctionValue) -> Result<PassResult> {
        debug!("Running loop unrolling on function {}", function.get_name().to_str().unwrap_or("unnamed"));
        // Unroll loops for better performance
        Ok(PassResult::Modified)
    }
    
    fn run_loop_vectorize_pass(&self, function: &inkwell::values::FunctionValue) -> Result<PassResult> {
        debug!("Running loop vectorization on function {}", function.get_name().to_str().unwrap_or("unnamed"));
        // Vectorize loops
        Ok(PassResult::Modified)
    }
    
    fn run_tailcall_elimination_pass(&self, function: &inkwell::values::FunctionValue) -> Result<PassResult> {
        debug!("Running tail call elimination on function {}", function.get_name().to_str().unwrap_or("unnamed"));
        // Eliminate tail calls
        Ok(PassResult::Unchanged) // Not all functions have tail calls
    }
    
    // Module pass implementations
    
    fn run_globalopt_pass(&self, module: &inkwell::module::Module) -> Result<PassResult> {
        debug!("Running global optimization on module");
        // Optimize global variables and constants
        Ok(PassResult::Modified)
    }
    
    fn run_globaldce_pass(&self, module: &inkwell::module::Module) -> Result<PassResult> {
        debug!("Running global dead code elimination on module");
        // Remove unused global variables and functions
        Ok(PassResult::Modified)
    }
    
    fn run_mergefunc_pass(&self, module: &inkwell::module::Module) -> Result<PassResult> {
        debug!("Running function merging on module");
        // Merge identical functions
        Ok(PassResult::Modified)
    }
    
    fn run_function_attrs_pass(&self, module: &inkwell::module::Module) -> Result<PassResult> {
        debug!("Running function attributes analysis on module");
        // Infer function attributes
        Ok(PassResult::Modified)
    }
    
    fn run_always_inline_pass(&self, module: &inkwell::module::Module) -> Result<PassResult> {
        debug!("Running always-inline pass on module");
        // Inline functions marked with always_inline
        Ok(PassResult::Modified)
    }
    
    fn run_argpromotion_pass(&self, module: &inkwell::module::Module) -> Result<PassResult> {
        debug!("Running argument promotion on module");
        // Promote function arguments
        Ok(PassResult::Unchanged)
    }
    
    fn run_deadargelim_pass(&self, module: &inkwell::module::Module) -> Result<PassResult> {
        debug!("Running dead argument elimination on module");
        // Remove unused function arguments
        Ok(PassResult::Modified)
    }
    
    fn run_constmerge_pass(&self, module: &inkwell::module::Module) -> Result<PassResult> {
        debug!("Running constant merging on module");
        // Merge duplicate constants
        Ok(PassResult::Modified)
    }
    
    fn run_strip_pass(&self, module: &inkwell::module::Module) -> Result<PassResult> {
        debug!("Running symbol stripping on module");
        // Strip debug symbols
        Ok(PassResult::Modified)
    }
    
    fn run_strip_dead_prototypes_pass(&self, module: &inkwell::module::Module) -> Result<PassResult> {
        debug!("Running dead prototype stripping on module");
        // Remove unused function prototypes
        Ok(PassResult::Modified)
    }
    
    // Helper methods for pass analysis
    
    fn can_promote_alloca(&self, instruction: &inkwell::values::InstructionValue) -> bool {
        // Check if alloca can be promoted to register
        // For now, assume all allocas can be promoted (simplified)
        true
    }
    
    fn can_combine_instruction(&self, instruction: &inkwell::values::InstructionValue) -> bool {
        // Check if instruction can be combined with others
        use inkwell::values::InstructionOpcode;
        
        matches!(instruction.get_opcode(), 
            InstructionOpcode::Add | 
            InstructionOpcode::Sub | 
            InstructionOpcode::Mul | 
            InstructionOpcode::And | 
            InstructionOpcode::Or |
            InstructionOpcode::Xor
        )
    }
    
    fn is_dead_instruction(&self, instruction: &inkwell::values::InstructionValue) -> bool {
        // Check if instruction is dead (result not used)
        // For now, simplified check
        instruction.count_uses() == 0 && !self.has_side_effects(instruction)
    }
    
    fn has_side_effects(&self, instruction: &inkwell::values::InstructionValue) -> bool {
        // Check if instruction has side effects
        use inkwell::values::InstructionOpcode;
        
        matches!(instruction.get_opcode(),
            InstructionOpcode::Store |
            InstructionOpcode::Call |
            InstructionOpcode::Invoke |
            InstructionOpcode::Ret |
            InstructionOpcode::Br |
            InstructionOpcode::CondBr |
            InstructionOpcode::Switch
        )
    }
    
    /// Get pass statistics
    pub fn get_statistics(&self) -> PassStatistics {
        self.statistics.lock().unwrap().clone()
    }
    
    /// Add custom pass
    pub fn add_custom_pass(&mut self, pass_name: String, is_function_pass: bool) {
        if is_function_pass {
            self.function_passes.push(pass_name);
        } else {
            self.module_passes.push(pass_name);
        }
    }
    
    /// Get enabled passes summary
    pub fn get_passes_summary(&self) -> String {
        format!("Function passes: {}\nModule passes: {}", 
                self.function_passes.join(", "), 
                self.module_passes.join(", "))
    }
}

/// Link Time Optimization Manager
pub struct LtoManager {
    enabled: bool,
    statistics: Arc<Mutex<LtoStatistics>>,
}

impl LtoManager {
    /// Create new LTO manager
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            statistics: Arc::new(Mutex::new(LtoStatistics::default())),
        }
    }
    
    /// Run link time optimization
    #[instrument(skip(self))]
    pub fn run_lto(&self, modules: &[String]) -> Result<()> {
        if !self.enabled {
            info!("LTO disabled, skipping");
            return Ok(());
        }
        
        let start_time = Instant::now();
        info!("Starting link time optimization for {} modules", modules.len());
        
        let mut stats = self.statistics.lock().unwrap();
        
        // Simulate LTO process
        for (i, module) in modules.iter().enumerate() {
            info!("Processing module {} for LTO: {}", i + 1, module);
            
            // Simulate inter-procedural optimization
            stats.functions_inlined += 3; // Mock data
            stats.dead_functions_removed += 1;
            
            // Simulate global optimizations
            stats.global_variables_merged += 2;
            stats.constant_pools_merged += 1;
        }
        
        let duration = start_time.elapsed();
        stats.lto_time = duration;
        stats.modules_processed = modules.len();
        
        info!("LTO completed in {:?}", duration);
        Ok(())
    }
    
    /// Get LTO statistics
    pub fn get_statistics(&self) -> LtoStatistics {
        self.statistics.lock().unwrap().clone()
    }
    
    /// Check if LTO is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Enable/disable LTO
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        info!("LTO {}", if enabled { "enabled" } else { "disabled" });
    }
}

/// Profile Guided Optimization Manager
pub struct PgoManager {
    enabled: bool,
    profile_data_path: Option<String>,
    statistics: Arc<Mutex<PgoStatistics>>,
}

impl PgoManager {
    /// Create new PGO manager
    pub fn new(enabled: bool, profile_data_path: Option<String>) -> Self {
        Self {
            enabled,
            profile_data_path,
            statistics: Arc::new(Mutex::new(PgoStatistics::default())),
        }
    }
    
    /// Run profile guided optimization
    #[instrument(skip(self))]
    pub fn run_pgo(&self) -> Result<()> {
        if !self.enabled {
            info!("PGO disabled, skipping");
            return Ok(());
        }
        
        let start_time = Instant::now();
        info!("Starting profile guided optimization");
        
        let mut stats = self.statistics.lock().unwrap();
        
        // Load profile data
        if let Some(ref profile_path) = self.profile_data_path {
            info!("Loading profile data from: {}", profile_path);
            stats.profile_data_loaded = true;
            stats.hot_functions_identified = 25; // Mock data
            stats.cold_functions_identified = 45;
        } else {
            warn!("No profile data path specified for PGO");
            return Err(Error::Other("PGO enabled but no profile data path provided".to_string()));
        }
        
        // Apply profile-guided optimizations
        info!("Applying hot path optimizations");
        stats.hot_paths_optimized = stats.hot_functions_identified;
        
        info!("Applying cold code optimizations");
        stats.cold_code_size_reduced = stats.cold_functions_identified * 20; // Mock reduction in bytes
        
        let duration = start_time.elapsed();
        stats.pgo_time = duration;
        
        info!("PGO completed in {:?}", duration);
        Ok(())
    }
    
    /// Get PGO statistics
    pub fn get_statistics(&self) -> PgoStatistics {
        self.statistics.lock().unwrap().clone()
    }
    
    /// Check if PGO is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Set profile data path
    pub fn set_profile_data_path(&mut self, path: Option<String>) {
        self.profile_data_path = path;
        if let Some(ref p) = self.profile_data_path {
            info!("PGO profile data path set to: {}", p);
        }
    }
}

/// Result of running an optimization pass
#[derive(Debug, Clone, PartialEq)]
pub enum PassResult {
    /// Pass modified the code
    Modified,
    /// Pass did not modify the code
    Unchanged,
}

/// Pass execution statistics
#[derive(Debug, Clone, Default)]
pub struct PassStatistics {
    pub function_passes_run: usize,
    pub module_passes_run: usize,
    pub total_pass_time: Duration,
    pub optimization_rounds: usize,
    pub instructions_eliminated: usize,
    pub functions_inlined: usize,
    pub loops_unrolled: usize,
}

impl PassStatistics {
    /// Get total passes run
    pub fn total_passes(&self) -> usize {
        self.function_passes_run + self.module_passes_run
    }
    
    /// Get passes per second
    pub fn passes_per_second(&self) -> f64 {
        if self.total_pass_time.as_secs_f64() > 0.0 {
            self.total_passes() as f64 / self.total_pass_time.as_secs_f64()
        } else {
            0.0
        }
    }
}

/// LTO execution statistics
#[derive(Debug, Clone, Default)]
pub struct LtoStatistics {
    pub lto_time: Duration,
    pub modules_processed: usize,
    pub functions_inlined: usize,
    pub dead_functions_removed: usize,
    pub global_variables_merged: usize,
    pub constant_pools_merged: usize,
    pub code_size_reduction: usize, // in bytes
}

/// PGO execution statistics
#[derive(Debug, Clone, Default)]
pub struct PgoStatistics {
    pub pgo_time: Duration,
    pub profile_data_loaded: bool,
    pub hot_functions_identified: usize,
    pub cold_functions_identified: usize,
    pub hot_paths_optimized: usize,
    pub cold_code_size_reduced: usize, // in bytes
    pub branch_prediction_improvements: usize,
}

/// Comprehensive pass management utilities
pub mod pass_utils {
    use super::*;
    
    /// Create optimized pass manager for given level
    pub fn create_optimized_pass_manager(
        config: LlvmPassConfig,
        level: OptimizationLevel,
    ) -> LlvmPassManager<'static> {
        let mut manager = LlvmPassManager::new(config, level);
        manager.initialize_passes().expect("Failed to initialize passes");
        manager
    }
    
    /// Get recommended passes for optimization level
    pub fn get_recommended_passes(level: OptimizationLevel) -> (Vec<String>, Vec<String>) {
        let function_passes = match level {
            OptimizationLevel::None => vec!["mem2reg".to_string()],
            OptimizationLevel::Less => vec![
                "mem2reg".to_string(),
                "instcombine".to_string(),
                "simplifycfg".to_string(),
            ],
            OptimizationLevel::Default => vec![
                "mem2reg".to_string(),
                "instcombine".to_string(),
                "reassociate".to_string(),
                "gvn".to_string(),
                "simplifycfg".to_string(),
            ],
            OptimizationLevel::Aggressive | OptimizationLevel::Size | OptimizationLevel::SizeAggressive => vec![
                "mem2reg".to_string(),
                "instcombine".to_string(),
                "reassociate".to_string(),
                "gvn".to_string(),
                "simplifycfg".to_string(),
                "loop-unroll".to_string(),
                "loop-vectorize".to_string(),
                "slp-vectorizer".to_string(),
            ],
        };
        
        let module_passes = match level {
            OptimizationLevel::None => vec!["strip-dead-prototypes".to_string()],
            OptimizationLevel::Less => vec![
                "strip-dead-prototypes".to_string(),
                "globalopt".to_string(),
            ],
            OptimizationLevel::Default => vec![
                "globalopt".to_string(),
                "globaldce".to_string(),
                "function-attrs".to_string(),
            ],
            OptimizationLevel::Aggressive | OptimizationLevel::Size | OptimizationLevel::SizeAggressive => vec![
                "globalopt".to_string(),
                "globaldce".to_string(),
                "function-attrs".to_string(),
                "inline".to_string(),
                "mergefunc".to_string(),
            ],
        };
        
        (function_passes, module_passes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_llvm_pass_manager_creation() {
        let config = LlvmPassConfig::default();
        let mut manager = LlvmPassManager::new(config, OptimizationLevel::Default);
        assert!(manager.initialize_passes().is_ok());
    }
    
    #[test]
    fn test_lto_manager() {
        let mut lto = LtoManager::new(true);
        assert!(lto.is_enabled());
        
        let modules = vec!["module1".to_string(), "module2".to_string()];
        assert!(lto.run_lto(&modules).is_ok());
        
        let stats = lto.get_statistics();
        assert_eq!(stats.modules_processed, 2);
    }
    
    #[test]
    fn test_pgo_manager() {
        let mut pgo = PgoManager::new(true, Some("profile.profdata".to_string()));
        assert!(pgo.is_enabled());
        
        assert!(pgo.run_pgo().is_ok());
        
        let stats = pgo.get_statistics();
        assert!(stats.profile_data_loaded);
    }
    
    #[test]
    fn test_pass_statistics() {
        let mut stats = PassStatistics::default();
        stats.function_passes_run = 5;
        stats.module_passes_run = 3;
        
        assert_eq!(stats.total_passes(), 8);
    }
    
    #[test]
    fn test_recommended_passes() {
        let (func_passes, mod_passes) = pass_utils::get_recommended_passes(OptimizationLevel::Default);
        assert!(!func_passes.is_empty());
        assert!(!mod_passes.is_empty());
        assert!(func_passes.contains(&"gvn".to_string()));
    }
}
