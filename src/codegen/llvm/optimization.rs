/// LLVM Optimization Pass Manager Integration
/// 
/// Provides comprehensive optimization pass management for the CURSED compiler,
/// supporting various optimization levels and target-specific optimizations.

use crate::error::{Error, Result};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use inkwell::{
    context::Context,
    module::Module,
    passes::{PassManager, PassManagerBuilder},
    targets::{Target, TargetMachine, RelocMode, CodeModel, FileType},
    OptimizationLevel as InkwellOptLevel,
};

/// Optimization level configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationLevel {
    /// No optimization (-O0)
    None,
    /// Minimal optimization (-O1)
    Less,
    /// Standard optimization (-O2)
    Default,
    /// Aggressive optimization (-O3)
    Aggressive,
    /// Optimize for size (-Os)
    Size,
    /// Optimize aggressively for size (-Oz)
    SizeAggressive,
}

impl OptimizationLevel {
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "0" | "O0" => Ok(OptimizationLevel::None),
            "1" | "O1" => Ok(OptimizationLevel::Less),
            "2" | "O2" => Ok(OptimizationLevel::Default),
            "3" | "O3" => Ok(OptimizationLevel::Aggressive),
            "s" | "Os" => Ok(OptimizationLevel::Size),
            "z" | "Oz" => Ok(OptimizationLevel::SizeAggressive),
            _ => Err(Error::Other(format!("Invalid optimization level: {}", s))),
        }
    }
    
    pub fn to_inkwell_level(&self) -> InkwellOptLevel {
        match self {
            OptimizationLevel::None => InkwellOptLevel::None,
            OptimizationLevel::Less => InkwellOptLevel::Less,
            OptimizationLevel::Default => InkwellOptLevel::Default,
            OptimizationLevel::Aggressive => InkwellOptLevel::Aggressive,
            OptimizationLevel::Size => InkwellOptLevel::Default, // LLVM doesn't have size-specific levels
            OptimizationLevel::SizeAggressive => InkwellOptLevel::Aggressive,
        }
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            OptimizationLevel::None => "O0",
            OptimizationLevel::Less => "O1", 
            OptimizationLevel::Default => "O2",
            OptimizationLevel::Aggressive => "O3",
            OptimizationLevel::Size => "Os",
            OptimizationLevel::SizeAggressive => "Oz",
        }
    }
}

/// Optimization pass configuration
#[derive(Debug, Clone)]
pub struct OptimizationConfig {
    pub level: OptimizationLevel,
    pub target_cpu: Option<String>,
    pub target_features: Vec<String>,
    pub vectorize_loops: bool,
    pub vectorize_slp: bool,
    pub unroll_loops: bool,
    pub merge_functions: bool,
    pub inline_functions: bool,
    pub enable_lto: bool,
    pub custom_passes: Vec<String>,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            level: OptimizationLevel::Default,
            target_cpu: None,
            target_features: Vec::new(),
            vectorize_loops: true,
            vectorize_slp: true,
            unroll_loops: true,
            merge_functions: true,
            inline_functions: true,
            enable_lto: false,
            custom_passes: Vec::new(),
        }
    }
}

/// Optimization statistics
#[derive(Debug, Clone)]
pub struct OptimizationStats {
    pub passes_run: usize,
    pub optimization_time: Duration,
    pub code_size_before: usize,
    pub code_size_after: usize,
    pub functions_inlined: usize,
    pub loops_unrolled: usize,
    pub dead_code_eliminated: usize,
}

impl Default for OptimizationStats {
    fn default() -> Self {
        Self {
            passes_run: 0,
            optimization_time: Duration::from_secs(0),
            code_size_before: 0,
            code_size_after: 0,
            functions_inlined: 0,
            loops_unrolled: 0,
            dead_code_eliminated: 0,
        }
    }
}

/// Main optimization manager
pub struct OptimizationManager<'ctx> {
    context: &'ctx Context,
    config: OptimizationConfig,
    stats: Arc<Mutex<OptimizationStats>>,
    function_pass_manager: Option<PassManager<inkwell::values::FunctionValue<'ctx>>>,
    module_pass_manager: Option<PassManager<Module<'ctx>>>,
}

impl<'ctx> OptimizationManager<'ctx> {
    /// Create a new optimization manager
    pub fn new(context: &'ctx Context, config: OptimizationConfig) -> Self {
        Self {
            context,
            config,
            stats: Arc::new(Mutex::new(OptimizationStats::default())),
            function_pass_manager: None,
            module_pass_manager: None,
        }
    }
    
    /// Initialize optimization passes based on configuration
    pub fn initialize(&mut self, module: &Module<'ctx>) -> Result<()> {
        let start_time = Instant::now();
        
        // Initialize function pass manager
        let fpm = PassManager::create(module);
        self.setup_function_passes(&fpm)?;
        fpm.initialize();
        self.function_pass_manager = Some(fpm);
        
        // Initialize module pass manager
        let mpm = PassManager::create(());
        self.setup_module_passes(&mpm)?;
        self.module_pass_manager = Some(mpm);
        
        // Update stats
        let mut stats = self.stats.lock().unwrap();
        stats.optimization_time += start_time.elapsed();
        
        Ok(())
    }
    
    /// Setup function-level optimization passes
    fn setup_function_passes(&self, fpm: &PassManager<inkwell::values::FunctionValue<'ctx>>) -> Result<()> {
        let mut passes_added = 0;
        
        match self.config.level {
            OptimizationLevel::None => {
                // No optimization passes
            }
            OptimizationLevel::Less => {
                fpm.add_instruction_combining_pass();
                fpm.add_reassociate_pass();
                fpm.add_gvn_pass();
                fpm.add_cfg_simplification_pass();
                passes_added += 4;
            }
            OptimizationLevel::Default => {
                fpm.add_instruction_combining_pass();
                fpm.add_reassociate_pass();
                fpm.add_gvn_pass();
                fpm.add_cfg_simplification_pass();
                fpm.add_basic_alias_analysis_pass();
                fpm.add_promote_memory_to_register_pass();
                fpm.add_instruction_combining_pass();
                fpm.add_reassociate_pass();
                passes_added += 8;
            }
            OptimizationLevel::Aggressive | OptimizationLevel::Size | OptimizationLevel::SizeAggressive => {
                fpm.add_instruction_combining_pass();
                fpm.add_reassociate_pass();
                fpm.add_gvn_pass();
                fpm.add_cfg_simplification_pass();
                fpm.add_basic_alias_analysis_pass();
                fpm.add_promote_memory_to_register_pass();
                fpm.add_instruction_combining_pass();
                fpm.add_reassociate_pass();
                fpm.add_gvn_pass();
                fpm.add_cfg_simplification_pass();
                
                if self.config.unroll_loops {
                    fpm.add_loop_unroll_pass();
                }
                
                if self.config.vectorize_loops {
                    fpm.add_loop_vectorize_pass();
                }
                
                if self.config.vectorize_slp {
                    fpm.add_slp_vectorize_pass();
                }
                
                passes_added += 14;
            }
        }
        
        // Update stats
        let mut stats = self.stats.lock().unwrap();
        stats.passes_run += passes_added;
        
        Ok(())
    }
    
    /// Setup module-level optimization passes
    fn setup_module_passes(&self, mpm: &PassManager<Module<'ctx>>) -> Result<()> {
        let mut passes_added = 0;
        
        match self.config.level {
            OptimizationLevel::None => {
                // No optimization passes
            }
            OptimizationLevel::Less => {
                mpm.add_always_inliner_pass();
                mpm.add_strip_dead_prototypes_pass();
                passes_added += 2;
            }
            OptimizationLevel::Default => {
                mpm.add_always_inliner_pass();
                mpm.add_strip_dead_prototypes_pass();
                mpm.add_constant_merge_pass();
                mpm.add_dead_arg_elimination_pass();
                mpm.add_function_attrs_pass();
                mpm.add_global_dce_pass();
                passes_added += 6;
            }
            OptimizationLevel::Aggressive | OptimizationLevel::Size | OptimizationLevel::SizeAggressive => {
                mpm.add_always_inliner_pass();
                mpm.add_strip_dead_prototypes_pass();
                mpm.add_constant_merge_pass();
                mpm.add_dead_arg_elimination_pass();
                mpm.add_function_attrs_pass();
                mpm.add_global_dce_pass();
                mpm.add_global_optimizer_pass();
                mpm.add_prune_eh_pass();
                mpm.add_strip_dead_prototypes_pass();
                
                if self.config.merge_functions {
                    mpm.add_merge_functions_pass();
                }
                
                passes_added += 10;
            }
        }
        
        // Update stats
        let mut stats = self.stats.lock().unwrap();
        stats.passes_run += passes_added;
        
        Ok(())
    }
    
    /// Run optimization passes on a module
    pub fn optimize_module(&self, module: &Module<'ctx>) -> Result<()> {
        let start_time = Instant::now();
        
        // Get code size before optimization
        let code_before = module.print_to_string().to_string();
        let size_before = code_before.len();
        
        // Run function passes on all functions
        if let Some(ref fpm) = self.function_pass_manager {
            for function in module.get_functions() {
                fpm.run_on(&function);
            }
        }
        
        // Run module passes
        if let Some(ref mpm) = self.module_pass_manager {
            mpm.run_on(module);
        }
        
        // Get code size after optimization
        let code_after = module.print_to_string().to_string();
        let size_after = code_after.len();
        
        // Update stats
        let mut stats = self.stats.lock().unwrap();
        stats.optimization_time += start_time.elapsed();
        stats.code_size_before = size_before;
        stats.code_size_after = size_after;
        
        Ok(())
    }
    
    /// Get optimization statistics
    pub fn get_stats(&self) -> OptimizationStats {
        self.stats.lock().unwrap().clone()
    }
    
    /// Get optimization configuration
    pub fn get_config(&self) -> &OptimizationConfig {
        &self.config
    }
    
    /// Update optimization configuration
    pub fn update_config(&mut self, config: OptimizationConfig) {
        self.config = config;
    }
    
    /// Create a target machine for code generation
    pub fn create_target_machine(&self, target_triple: &str) -> Result<TargetMachine> {
        Target::initialize_native(&inkwell::targets::InitializationConfig::default())
            .map_err(|e| Error::Other(format!("Failed to initialize target: {}", e)))?;
        
        let target = Target::from_triple(target_triple)
            .map_err(|e| Error::Other(format!("Failed to create target from triple: {}", e)))?;
        
        let cpu = self.config.target_cpu.as_deref().unwrap_or("generic");
        let features = self.config.target_features.join(",");
        
        target.create_target_machine(
            target_triple,
            cpu,
            &features,
            self.config.level.to_inkwell_level(),
            RelocMode::Default,
            CodeModel::Default,
        ).ok_or_else(|| Error::Other("Failed to create target machine".to_string()))
    }
    
    /// Generate optimized object code
    pub fn generate_object_code(&self, module: &Module<'ctx>, target_triple: &str) -> Result<Vec<u8>> {
        let target_machine = self.create_target_machine(target_triple)?;
        
        target_machine.write_to_memory_buffer(module, FileType::Object)
            .map_err(|e| Error::Other(format!("Failed to generate object code: {}", e)))
            .map(|buffer| buffer.as_slice().to_vec())
    }
    
    /// Generate optimized assembly code
    pub fn generate_assembly(&self, module: &Module<'ctx>, target_triple: &str) -> Result<String> {
        let target_machine = self.create_target_machine(target_triple)?;
        
        target_machine.write_to_memory_buffer(module, FileType::Assembly)
            .map_err(|e| Error::Other(format!("Failed to generate assembly: {}", e)))
            .and_then(|buffer| {
                String::from_utf8(buffer.as_slice().to_vec())
                    .map_err(|e| Error::Other(format!("Invalid UTF-8 in assembly: {}", e)))
            })
    }
    
    /// Print optimization summary
    pub fn print_summary(&self) {
        let stats = self.get_stats();
        
        println!("🔧 Optimization Summary:");
        println!("   Level: {}", self.config.level.as_str());
        println!("   Passes run: {}", stats.passes_run);
        println!("   Time: {:?}", stats.optimization_time);
        println!("   Code size: {} -> {} bytes ({:.1}% reduction)", 
                 stats.code_size_before, 
                 stats.code_size_after,
                 if stats.code_size_before > 0 {
                     100.0 * (stats.code_size_before - stats.code_size_after) as f64 / stats.code_size_before as f64
                 } else {
                     0.0
                 });
        
        if stats.functions_inlined > 0 {
            println!("   Functions inlined: {}", stats.functions_inlined);
        }
        if stats.loops_unrolled > 0 {
            println!("   Loops unrolled: {}", stats.loops_unrolled);
        }
        if stats.dead_code_eliminated > 0 {
            println!("   Dead code eliminated: {}", stats.dead_code_eliminated);
        }
    }
}

/// Optimization utilities
pub mod utils {
    use super::*;
    
    /// Create optimization configuration from command line arguments
    pub fn create_config_from_args(
        opt_level: Option<&str>,
        target_cpu: Option<&str>,
        features: &[String],
        enable_lto: bool,
    ) -> Result<OptimizationConfig> {
        let level = if let Some(level_str) = opt_level {
            OptimizationLevel::from_str(level_str)?
        } else {
            OptimizationLevel::Default
        };
        
        Ok(OptimizationConfig {
            level,
            target_cpu: target_cpu.map(|s| s.to_string()),
            target_features: features.to_vec(),
            enable_lto,
            ..Default::default()
        })
    }
    
    /// Get default optimization configuration for development
    pub fn dev_config() -> OptimizationConfig {
        OptimizationConfig {
            level: OptimizationLevel::None,
            vectorize_loops: false,
            vectorize_slp: false,
            unroll_loops: false,
            merge_functions: false,
            inline_functions: false,
            enable_lto: false,
            ..Default::default()
        }
    }
    
    /// Get default optimization configuration for release
    pub fn release_config() -> OptimizationConfig {
        OptimizationConfig {
            level: OptimizationLevel::Aggressive,
            vectorize_loops: true,
            vectorize_slp: true,
            unroll_loops: true,
            merge_functions: true,
            inline_functions: true,
            enable_lto: true,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    
    #[test]
    fn test_optimization_level_conversion() {
        assert_eq!(OptimizationLevel::from_str("O0").unwrap(), OptimizationLevel::None);
        assert_eq!(OptimizationLevel::from_str("O1").unwrap(), OptimizationLevel::Less);
        assert_eq!(OptimizationLevel::from_str("O2").unwrap(), OptimizationLevel::Default);
        assert_eq!(OptimizationLevel::from_str("O3").unwrap(), OptimizationLevel::Aggressive);
        assert_eq!(OptimizationLevel::from_str("Os").unwrap(), OptimizationLevel::Size);
        assert_eq!(OptimizationLevel::from_str("Oz").unwrap(), OptimizationLevel::SizeAggressive);
    }
    
    #[test]
    fn test_optimization_config_creation() {
        let config = utils::create_config_from_args(
            Some("O2"),
            Some("native"),
            &["sse4.2".to_string(), "avx".to_string()],
            true,
        ).unwrap();
        
        assert_eq!(config.level, OptimizationLevel::Default);
        assert_eq!(config.target_cpu, Some("native".to_string()));
        assert_eq!(config.target_features, vec!["sse4.2", "avx"]);
        assert!(config.enable_lto);
    }
    
    #[test]
    fn test_optimization_manager_creation() {
        let context = Context::create();
        let config = OptimizationConfig::default();
        let manager = OptimizationManager::new(&context, config);
        
        assert_eq!(manager.get_config().level, OptimizationLevel::Default);
    }
    
    #[test]
    fn test_optimization_stats_default() {
        let stats = OptimizationStats::default();
        assert_eq!(stats.passes_run, 0);
        assert_eq!(stats.code_size_before, 0);
        assert_eq!(stats.code_size_after, 0);
    }
    
    #[test]
    fn test_dev_and_release_configs() {
        let dev_config = utils::dev_config();
        let release_config = utils::release_config();
        
        assert_eq!(dev_config.level, OptimizationLevel::None);
        assert_eq!(release_config.level, OptimizationLevel::Aggressive);
        assert!(!dev_config.enable_lto);
        assert!(release_config.enable_lto);
    }
}
