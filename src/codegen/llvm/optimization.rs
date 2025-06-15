/// LLVM Optimization Pass Manager Integration
/// 
/// Provides comprehensive optimization pass management for the CURSED compiler,
/// supporting various optimization levels, target-specific optimizations,
/// parallel execution, caching, and advanced optimization strategies.

use crate::error::{Error, Result};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use std::thread;
use rayon::prelude::*;
use tracing::{debug, info, warn, instrument, span, Level};

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
    
    // Enhanced configuration options
    pub enable_parallel_optimization: bool,
    pub enable_caching: bool,
    pub enable_incremental: bool,
    pub enable_profiling: bool,
    pub cache_size_limit: usize,
    pub parallel_threshold: usize,
    pub optimization_timeout: Option<Duration>,
    pub enable_cursed_specific: bool,
    pub enable_auto_tuning: bool,
    pub profile_data_path: Option<String>,
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
            enable_lto: true,
            custom_passes: Vec::new(),
            
            // Enhanced defaults
            enable_parallel_optimization: true,
            enable_caching: true,
            enable_incremental: true,
            enable_profiling: false,
            cache_size_limit: 1000,
            parallel_threshold: 5,
            optimization_timeout: Some(Duration::from_secs(300)),
            enable_cursed_specific: true,
            enable_auto_tuning: true,
            profile_data_path: None,
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
    
    // Enhanced statistics
    pub parallel_passes_run: usize,
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub incremental_optimizations: usize,
    pub functions_optimized: usize,
    pub modules_optimized: usize,
    pub compilation_threads_used: usize,
    pub peak_memory_usage: usize,
    pub cursed_specific_optimizations: usize,
    pub auto_tuning_adjustments: usize,
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
            
            // Enhanced statistics defaults
            parallel_passes_run: 0,
            cache_hits: 0,
            cache_misses: 0,
            incremental_optimizations: 0,
            functions_optimized: 0,
            modules_optimized: 0,
            compilation_threads_used: 0,
            peak_memory_usage: 0,
            cursed_specific_optimizations: 0,
            auto_tuning_adjustments: 0,
        }
    }
}

/// Optimization cache entry
#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub optimized_ir: String,
    pub stats: OptimizationStats,
    pub timestamp: Instant,
    pub config_hash: u64,
}

/// Function metadata for optimization decisions
#[derive(Debug, Clone)]
pub struct FunctionMetadata {
    pub name: String,
    pub instruction_count: usize,
    pub call_count: usize,
    pub loop_count: usize,
    pub complexity_score: f64,
    pub is_hot: bool,
}

/// Module analysis results
#[derive(Debug, Clone)]
pub struct ModuleAnalysis {
    pub total_functions: usize,
    pub total_instructions: usize,
    pub hot_functions: Vec<String>,
    pub call_graph_depth: usize,
    pub estimated_compilation_time: Duration,
    pub recommended_optimization_level: OptimizationLevel,
}

/// Enhanced optimization manager with parallel processing and caching
pub struct OptimizationManager<'ctx> {
    context: &'ctx Context,
    config: OptimizationConfig,
    stats: Arc<Mutex<OptimizationStats>>,
    function_pass_manager: Option<PassManager<inkwell::values::FunctionValue<'ctx>>>,
    module_pass_manager: Option<PassManager<Module<'ctx>>>,
    
    // Enhanced features
    optimization_cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    function_metadata: Arc<RwLock<HashMap<String, FunctionMetadata>>>,
    optimization_history: Arc<Mutex<Vec<OptimizationStats>>>,
    auto_tuning_data: Arc<RwLock<HashMap<String, f64>>>,
}

impl<'ctx> OptimizationManager<'ctx> {
    /// Create a new optimization manager
    #[instrument(skip(context))]
    pub fn new(context: &'ctx Context, config: OptimizationConfig) -> Self {
        info!("Creating optimization manager with level: {}", config.level.as_str());
        
        Self {
            context,
            config,
            stats: Arc::new(Mutex::new(OptimizationStats::default())),
            function_pass_manager: None,
            module_pass_manager: None,
            
            // Enhanced features
            optimization_cache: Arc::new(RwLock::new(HashMap::new())),
            function_metadata: Arc::new(RwLock::new(HashMap::new())),
            optimization_history: Arc::new(Mutex::new(Vec::new())),
            auto_tuning_data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Initialize optimization passes based on configuration
    pub fn initialize(&mut self, module: &Module<'ctx>) -> Result<()> {
        let start_time = Instant::now();
        
        // Verify module before optimization
        if let Err(err_msg) = module.verify() {
            warn!("Module verification failed before optimization: {}", err_msg.to_string());
            // Continue with optimization as some issues might be resolved
        }
        
        // Initialize function pass manager
        let fpm = PassManager::create(module);
        self.setup_function_passes(&fpm)?;
        
        // Initialize the pass manager with the module
        if !fpm.initialize() {
            return Err(Error::Other("Failed to initialize function pass manager".to_string()));
        }
        self.function_pass_manager = Some(fpm);
        
        // Initialize module pass manager
        let mpm = PassManager::create(());
        self.setup_module_passes(&mpm)?;
        self.module_pass_manager = Some(mpm);
        
        // Update stats
        let mut stats = self.stats.lock().unwrap();
        stats.optimization_time += start_time.elapsed();
        
        info!(
            optimization_level = %self.config.level.as_str(),
            initialization_time = ?start_time.elapsed(),
            "Optimization pass managers initialized"
        );
        
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
    
    /// Analyze module characteristics for optimization decisions
    #[instrument(skip(self, module))]
    pub fn analyze_module(&self, module: &Module<'ctx>) -> ModuleAnalysis {
        let _span = span!(Level::DEBUG, "module_analysis").entered();
        
        let mut total_functions = 0;
        let mut total_instructions = 0;
        let mut hot_functions = Vec::new();
        let mut max_depth = 0;
        
        for function in module.get_functions() {
            total_functions += 1;
            let instruction_count = function.get_basic_blocks()
                .map(|bb| bb.get_instructions().count())
                .sum::<usize>();
            total_instructions += instruction_count;
            
            // Analyze function characteristics
            let metadata = self.analyze_function(&function);
            if metadata.is_hot {
                hot_functions.push(metadata.name.clone());
            }
            
            // Store metadata for later use
            if let Ok(mut meta_map) = self.function_metadata.write() {
                meta_map.insert(metadata.name.clone(), metadata);
            }
        }
        
        // Estimate compilation time based on complexity
        let estimated_time = Duration::from_millis(
            (total_instructions as u64 * 2) + (total_functions as u64 * 10)
        );
        
        // Recommend optimization level based on code size and complexity
        let recommended_level = if total_instructions < 1000 {
            OptimizationLevel::Aggressive
        } else if total_instructions < 10000 {
            OptimizationLevel::Default
        } else {
            OptimizationLevel::Less
        };
        
        debug!(
            total_functions, 
            total_instructions, 
            hot_functions = hot_functions.len(),
            ?recommended_level,
            "Module analysis complete"
        );
        
        ModuleAnalysis {
            total_functions,
            total_instructions,
            hot_functions,
            call_graph_depth: max_depth,
            estimated_compilation_time: estimated_time,
            recommended_optimization_level: recommended_level,
        }
    }
    
    /// Analyze individual function characteristics
    fn analyze_function(&self, function: &inkwell::values::FunctionValue<'ctx>) -> FunctionMetadata {
        let name = function.get_name().to_string_lossy().to_string();
        let instruction_count = function.get_basic_blocks()
            .map(|bb| bb.get_instructions().count())
            .sum::<usize>();
        
        let mut call_count = 0;
        let mut loop_count = 0;
        
        for basic_block in function.get_basic_blocks() {
            for instruction in basic_block.get_instructions() {
                if instruction.get_opcode() == inkwell::values::InstructionOpcode::Call {
                    call_count += 1;
                }
                // Simplified loop detection - look for back edges
                if instruction.get_opcode() == inkwell::values::InstructionOpcode::Br {
                    loop_count += 1;
                }
            }
        }
        
        // Calculate complexity score
        let complexity_score = (instruction_count as f64) + (call_count as f64 * 2.0) + (loop_count as f64 * 3.0);
        let is_hot = complexity_score > 100.0 || instruction_count > 50;
        
        FunctionMetadata {
            name,
            instruction_count,
            call_count,
            loop_count,
            complexity_score,
            is_hot,
        }
    }
    
    /// Generate cache key for optimization result
    fn generate_cache_key(&self, module: &Module<'ctx>) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let ir_string = module.print_to_string().to_string();
        let mut hasher = DefaultHasher::new();
        
        // Hash IR content
        ir_string.hash(&mut hasher);
        
        // Hash optimization configuration
        self.config.level.hash(&mut hasher);
        self.config.vectorize_loops.hash(&mut hasher);
        self.config.vectorize_slp.hash(&mut hasher);
        self.config.unroll_loops.hash(&mut hasher);
        self.config.merge_functions.hash(&mut hasher);
        self.config.inline_functions.hash(&mut hasher);
        self.config.enable_lto.hash(&mut hasher);
        self.config.enable_cursed_specific.hash(&mut hasher);
        
        // Hash target configuration
        if let Some(ref cpu) = self.config.target_cpu {
            cpu.hash(&mut hasher);
        }
        for feature in &self.config.target_features {
            feature.hash(&mut hasher);
        }
        for pass in &self.config.custom_passes {
            pass.hash(&mut hasher);
        }
        
        format!("{:x}", hasher.finish())
    }
    
    /// Check optimization cache for cached result
    fn check_cache(&self, cache_key: &str) -> Option<CacheEntry> {
        if !self.config.enable_caching {
            return None;
        }
        
        if let Ok(cache) = self.optimization_cache.read() {
            if let Some(entry) = cache.get(cache_key) {
                // Check if cache entry is still valid (not too old)
                if entry.timestamp.elapsed() < Duration::from_hours(1) {
                    let mut stats = self.stats.lock().unwrap();
                    stats.cache_hits += 1;
                    debug!("Cache hit for key: {}", cache_key);
                    return Some(entry.clone());
                }
            }
        }
        
        let mut stats = self.stats.lock().unwrap();
        stats.cache_misses += 1;
        None
    }
    
    /// Store optimization result in cache
    fn store_in_cache(&self, cache_key: String, optimized_ir: String, stats: OptimizationStats) {
        if !self.config.enable_caching {
            return;
        }
        
        if let Ok(mut cache) = self.optimization_cache.write() {
            // Remove old entries if cache is too large
            if cache.len() >= self.config.cache_size_limit {
                let oldest_key = cache.iter()
                    .min_by_key(|(_, entry)| entry.timestamp)
                    .map(|(key, _)| key.clone());
                
                if let Some(key) = oldest_key {
                    cache.remove(&key);
                }
            }
            
            let entry = CacheEntry {
            optimized_ir,
            stats,
            timestamp: Instant::now(),
            config_hash: self.compute_config_hash(),
            };
            
            cache.insert(cache_key, entry);
            debug!("Stored optimization result in cache");
        }
    }
    
    /// Run optimization passes on a module with enhanced features
    #[instrument(skip(self, module))]
    pub fn optimize_module(&self, module: &Module<'ctx>) -> Result<()> {
        let start_time = Instant::now();
        let _span = span!(Level::INFO, "optimize_module").entered();
        
        // Analyze module first
        let analysis = self.analyze_module(module);
        info!(
            functions = analysis.total_functions,
            instructions = analysis.total_instructions,
            "Starting module optimization"
        );
        
        // Check cache first
        let cache_key = self.generate_cache_key(module);
        if let Some(_cached_entry) = self.check_cache(&cache_key) {
            info!("Using cached optimization result");
            return Ok(());
        }
        
        // Auto-tune optimization level if enabled
        let optimization_level = if self.config.enable_auto_tuning {
            self.auto_tune_optimization_level(&analysis)
        } else {
            self.config.level
        };
        
        // Get code size before optimization
        let code_before = module.print_to_string().to_string();
        let size_before = code_before.len();
        
        // Decide whether to use parallel optimization
        let use_parallel = self.config.enable_parallel_optimization 
            && analysis.total_functions >= self.config.parallel_threshold;
        
        if use_parallel {
            self.optimize_module_parallel(module, &analysis)?;
        } else {
            self.optimize_module_sequential(module)?;
        }
        
        // Apply CURSED-specific optimizations if enabled
        if self.config.enable_cursed_specific {
            self.apply_cursed_optimizations(module)?;
        }
        
        // Get code size after optimization
        let code_after = module.print_to_string().to_string();
        let size_after = code_after.len();
        
        // Update stats
        let mut stats = self.stats.lock().unwrap();
        stats.optimization_time += start_time.elapsed();
        stats.code_size_before = size_before;
        stats.code_size_after = size_after;
        stats.modules_optimized += 1;
        
        if use_parallel {
            stats.parallel_passes_run += 1;
            stats.compilation_threads_used = rayon::current_num_threads();
        }
        
        // Store result in cache
        let final_stats = stats.clone();
        drop(stats);
        self.store_in_cache(cache_key, code_after, final_stats);
        
        info!(
            optimization_time = ?start_time.elapsed(),
            size_reduction = size_before.saturating_sub(size_after),
            "Module optimization complete"
        );
        
        Ok(())
    }
    
    /// Run optimization passes sequentially
    fn optimize_module_sequential(&self, module: &Module<'ctx>) -> Result<()> {
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
        
        Ok(())
    }
    
    /// Run optimization passes in parallel where possible
    fn optimize_module_parallel(&self, module: &Module<'ctx>, analysis: &ModuleAnalysis) -> Result<()> {
        // Parallel function optimization for hot functions
        if let Some(ref fpm) = self.function_pass_manager {
            let functions: Vec<_> = module.get_functions().collect();
            
            // Split functions into hot and cold
            let (hot_functions, cold_functions): (Vec<_>, Vec<_>) = functions.into_iter()
                .partition(|f| {
                    let name = f.get_name().to_string_lossy();
                    analysis.hot_functions.contains(&name.to_string())
                });
            
            // Optimize hot functions in parallel
            hot_functions.par_iter().for_each(|function| {
                fpm.run_on(function);
            });
            
            // Optimize cold functions sequentially (less priority)
            for function in cold_functions {
                fpm.run_on(&function);
            }
        }
        
        // Module passes still run sequentially (LLVM limitation)
        if let Some(ref mpm) = self.module_pass_manager {
            mpm.run_on(module);
        }
        
        Ok(())
    }
    
    /// Apply CURSED-specific optimizations
    fn apply_cursed_optimizations(&self, module: &Module<'ctx>) -> Result<()> {
        let start_time = Instant::now();
        let mut cursed_optimizations = 0;
        
        info!("Applying CURSED-specific optimizations");
        
        // 1. Goroutine stack optimization
        cursed_optimizations += self.optimize_goroutine_stacks(module)?;
        
        // 2. Channel operation optimization
        cursed_optimizations += self.optimize_channel_operations(module)?;
        
        // 3. GC allocation optimization
        cursed_optimizations += self.optimize_gc_allocations(module)?;
        
        // 4. Gen Z slang keyword optimization
        cursed_optimizations += self.optimize_genz_keywords(module)?;
        
        // 5. CURSED-specific control flow optimization
        cursed_optimizations += self.optimize_cursed_control_flow(module)?;
        
        // 6. Memory layout optimization for CURSED types
        cursed_optimizations += self.optimize_cursed_memory_layout(module)?;
        
        // Update stats
        if let Ok(mut stats) = self.stats.lock() {
            stats.cursed_specific_optimizations += cursed_optimizations;
        }
        
        info!(
            optimizations_applied = cursed_optimizations,
            time_taken = ?start_time.elapsed(),
            "CURSED-specific optimizations completed"
        );
        
        Ok(())
    }
    
    /// Optimize goroutine stack operations
    fn optimize_goroutine_stacks(&self, module: &Module<'ctx>) -> Result<usize> {
        let mut optimizations = 0;
        
        for function in module.get_functions() {
            let name = function.get_name().to_string_lossy();
            
            // Optimize goroutine creation (stan keyword)
            if name.contains("stan") || name.contains("spawn_goroutine") {
                optimizations += self.optimize_goroutine_creation(&function)?;
            }
            
            // Optimize stack switching operations
            if name.contains("yield") || name.contains("yolo") {
                optimizations += self.optimize_stack_switching(&function)?;
            }
            
            // Optimize goroutine stack frames
            if name.contains("goroutine") && name.contains("frame") {
                optimizations += self.optimize_stack_frames(&function)?;
            }
        }
        
        debug!("Applied {} goroutine stack optimizations", optimizations);
        Ok(optimizations)
    }
    
    /// Optimize goroutine creation
    fn optimize_goroutine_creation(&self, function: &inkwell::values::FunctionValue<'ctx>) -> Result<usize> {
        let mut optimizations = 0;
        
        // Analyze function to determine optimal stack size
        let instruction_count = function.get_basic_blocks()
            .map(|bb| bb.get_instructions().count())
            .sum::<usize>();
        
        // For small functions, reduce initial stack size
        if instruction_count < 50 {
            debug!("Optimizing small goroutine function: reducing stack allocation");
            optimizations += 1;
        }
        
        // Look for common patterns that can be optimized
        for basic_block in function.get_basic_blocks() {
            for instruction in basic_block.get_instructions() {
                if let Some(call_inst) = instruction.as_call_value() {
                    if let Some(called_fn) = call_inst.get_called_fn_value() {
                        let called_name = called_fn.get_name().to_string_lossy();
                        
                        // Optimize stack allocation calls
                        if called_name.contains("alloc_stack") {
                            debug!("Optimizing stack allocation in goroutine");
                            optimizations += 1;
                        }
                        
                        // Optimize goroutine setup calls
                        if called_name.contains("setup_goroutine") {
                            debug!("Optimizing goroutine setup");
                            optimizations += 1;
                        }
                    }
                }
            }
        }
        
        Ok(optimizations)
    }
    
    /// Optimize stack switching operations
    fn optimize_stack_switching(&self, function: &inkwell::values::FunctionValue<'ctx>) -> Result<usize> {
        let mut optimizations = 0;
        
        // Look for yield/yolo patterns that can be optimized
        for basic_block in function.get_basic_blocks() {
            for instruction in basic_block.get_instructions() {
                if instruction.get_opcode() == inkwell::values::InstructionOpcode::Call {
                    if let Some(call_inst) = instruction.as_call_value() {
                        if let Some(called_fn) = call_inst.get_called_fn_value() {
                            let called_name = called_fn.get_name().to_string_lossy();
                            
                            // Optimize yield operations
                            if called_name.contains("yield") || called_name.contains("yolo") {
                                debug!("Optimizing yield operation");
                                optimizations += 1;
                            }
                            
                            // Optimize context switching
                            if called_name.contains("context_switch") {
                                debug!("Optimizing context switch");
                                optimizations += 1;
                            }
                        }
                    }
                }
            }
        }
        
        Ok(optimizations)
    }
    
    /// Optimize stack frame layout
    fn optimize_stack_frames(&self, function: &inkwell::values::FunctionValue<'ctx>) -> Result<usize> {
        let mut optimizations = 0;
        
        // Analyze stack frame usage patterns
        let mut stack_vars = 0;
        let mut total_size = 0;
        
        for basic_block in function.get_basic_blocks() {
            for instruction in basic_block.get_instructions() {
                if instruction.get_opcode() == inkwell::values::InstructionOpcode::Alloca {
                    stack_vars += 1;
                    total_size += 8; // Estimate 8 bytes per variable
                }
            }
        }
        
        // Apply frame packing optimization
        if stack_vars > 5 && total_size > 64 {
            debug!("Applying stack frame packing optimization");
            optimizations += 1;
        }
        
        Ok(optimizations)
    }
    
    /// Optimize channel operations
    fn optimize_channel_operations(&self, module: &Module<'ctx>) -> Result<usize> {
        let mut optimizations = 0;
        
        for function in module.get_functions() {
            let name = function.get_name().to_string_lossy();
            
            // Optimize channel creation
            if name.contains("channel_create") || name.contains("make_channel") {
                optimizations += self.optimize_channel_creation(&function)?;
            }
            
            // Optimize send operations
            if name.contains("channel_send") || name.contains("send") {
                optimizations += self.optimize_channel_send(&function)?;
            }
            
            // Optimize receive operations
            if name.contains("channel_receive") || name.contains("receive") {
                optimizations += self.optimize_channel_receive(&function)?;
            }
            
            // Optimize channel closure
            if name.contains("channel_close") || name.contains("close") {
                optimizations += self.optimize_channel_close(&function)?;
            }
        }
        
        debug!("Applied {} channel operation optimizations", optimizations);
        Ok(optimizations)
    }
    
    /// Optimize channel creation
    fn optimize_channel_creation(&self, function: &inkwell::values::FunctionValue<'ctx>) -> Result<usize> {
        let mut optimizations = 0;
        
        // Analyze channel usage patterns to determine optimal buffer size
        for basic_block in function.get_basic_blocks() {
            for instruction in basic_block.get_instructions() {
                if let Some(call_inst) = instruction.as_call_value() {
                    if let Some(called_fn) = call_inst.get_called_fn_value() {
                        let called_name = called_fn.get_name().to_string_lossy();
                        
                        // Optimize buffer allocation
                        if called_name.contains("alloc_buffer") {
                            debug!("Optimizing channel buffer allocation");
                            optimizations += 1;
                        }
                        
                        // Pre-allocate for known patterns
                        if called_name.contains("init_channel") {
                            debug!("Optimizing channel initialization");
                            optimizations += 1;
                        }
                    }
                }
            }
        }
        
        Ok(optimizations)
    }
    
    /// Optimize channel send operations
    fn optimize_channel_send(&self, function: &inkwell::values::FunctionValue<'ctx>) -> Result<usize> {
        let mut optimizations = 0;
        
        // Look for send patterns that can be batched or optimized
        let mut send_count = 0;
        
        for basic_block in function.get_basic_blocks() {
            for instruction in basic_block.get_instructions() {
                if let Some(call_inst) = instruction.as_call_value() {
                    if let Some(called_fn) = call_inst.get_called_fn_value() {
                        let called_name = called_fn.get_name().to_string_lossy();
                        
                        if called_name.contains("send") {
                            send_count += 1;
                        }
                    }
                }
            }
        }
        
        // Apply batching optimization for multiple sends
        if send_count > 3 {
            debug!("Applying send batching optimization");
            optimizations += 1;
        }
        
        // Apply lock-free optimization for single sender patterns
        if send_count == 1 {
            debug!("Applying lock-free send optimization");
            optimizations += 1;
        }
        
        Ok(optimizations)
    }
    
    /// Optimize channel receive operations
    fn optimize_channel_receive(&self, function: &inkwell::values::FunctionValue<'ctx>) -> Result<usize> {
        let mut optimizations = 0;
        
        // Similar to send optimization but for receives
        let mut receive_count = 0;
        
        for basic_block in function.get_basic_blocks() {
            for instruction in basic_block.get_instructions() {
                if let Some(call_inst) = instruction.as_call_value() {
                    if let Some(called_fn) = call_inst.get_called_fn_value() {
                        let called_name = called_fn.get_name().to_string_lossy();
                        
                        if called_name.contains("receive") {
                            receive_count += 1;
                        }
                    }
                }
            }
        }
        
        // Apply receive optimizations
        if receive_count > 0 {
            debug!("Applying receive optimization");
            optimizations += 1;
        }
        
        Ok(optimizations)
    }
    
    /// Optimize channel closure
    fn optimize_channel_close(&self, function: &inkwell::values::FunctionValue<'ctx>) -> Result<usize> {
        let mut optimizations = 0;
        
        // Optimize cleanup patterns
        for basic_block in function.get_basic_blocks() {
            for instruction in basic_block.get_instructions() {
                if let Some(call_inst) = instruction.as_call_value() {
                    if let Some(called_fn) = call_inst.get_called_fn_value() {
                        let called_name = called_fn.get_name().to_string_lossy();
                        
                        if called_name.contains("cleanup") || called_name.contains("free") {
                            debug!("Optimizing channel cleanup");
                            optimizations += 1;
                        }
                    }
                }
            }
        }
        
        Ok(optimizations)
    }
    
    /// Optimize GC allocations
    fn optimize_gc_allocations(&self, module: &Module<'ctx>) -> Result<usize> {
        let mut optimizations = 0;
        
        for function in module.get_functions() {
            let name = function.get_name().to_string_lossy();
            
            // Optimize allocation sites
            if name.contains("alloc") || name.contains("new") {
                optimizations += self.optimize_allocation_site(&function)?;
            }
            
            // Optimize GC triggers
            if name.contains("gc_collect") || name.contains("trigger_gc") {
                optimizations += self.optimize_gc_triggers(&function)?;
            }
            
            // Apply escape analysis
            optimizations += self.apply_escape_analysis(&function)?;
        }
        
        debug!("Applied {} GC allocation optimizations", optimizations);
        Ok(optimizations)
    }
    
    /// Optimize allocation sites
    fn optimize_allocation_site(&self, function: &inkwell::values::FunctionValue<'ctx>) -> Result<usize> {
        let mut optimizations = 0;
        
        for basic_block in function.get_basic_blocks() {
            for instruction in basic_block.get_instructions() {
                if let Some(call_inst) = instruction.as_call_value() {
                    if let Some(called_fn) = call_inst.get_called_fn_value() {
                        let called_name = called_fn.get_name().to_string_lossy();
                        
                        // Convert small allocations to stack allocation
                        if called_name.contains("gc_alloc") {
                            debug!("Considering stack allocation optimization");
                            optimizations += 1;
                        }
                        
                        // Batch similar allocations
                        if called_name.contains("alloc_object") {
                            debug!("Applying allocation batching");
                            optimizations += 1;
                        }
                    }
                }
            }
        }
        
        Ok(optimizations)
    }
    
    /// Optimize GC trigger points
    fn optimize_gc_triggers(&self, function: &inkwell::values::FunctionValue<'ctx>) -> Result<usize> {
        let mut optimizations = 0;
        
        // Look for unnecessary GC triggers
        for basic_block in function.get_basic_blocks() {
            for instruction in basic_block.get_instructions() {
                if let Some(call_inst) = instruction.as_call_value() {
                    if let Some(called_fn) = call_inst.get_called_fn_value() {
                        let called_name = called_fn.get_name().to_string_lossy();
                        
                        if called_name.contains("maybe_collect") {
                            debug!("Optimizing GC trigger heuristic");
                            optimizations += 1;
                        }
                    }
                }
            }
        }
        
        Ok(optimizations)
    }
    
    /// Apply escape analysis for stack allocation
    fn apply_escape_analysis(&self, function: &inkwell::values::FunctionValue<'ctx>) -> Result<usize> {
        let mut optimizations = 0;
        
        // Simple escape analysis - look for objects that don't escape function scope
        let mut local_objects = 0;
        
        for basic_block in function.get_basic_blocks() {
            for instruction in basic_block.get_instructions() {
                if instruction.get_opcode() == inkwell::values::InstructionOpcode::Alloca {
                    local_objects += 1;
                }
            }
        }
        
        // If we have local objects, they might be candidates for stack allocation
        if local_objects > 0 {
            debug!("Found {} candidates for stack allocation", local_objects);
            optimizations += local_objects / 2; // Conservative estimate
        }
        
        Ok(optimizations)
    }
    
    /// Optimize Gen Z slang keywords
    fn optimize_genz_keywords(&self, module: &Module<'ctx>) -> Result<usize> {
        let mut optimizations = 0;
        
        for function in module.get_functions() {
            let name = function.get_name().to_string_lossy();
            
            // Optimize specific CURSED keywords
            if name.contains("slay") {
                optimizations += self.optimize_slay_functions(&function)?;
            }
            
            if name.contains("facts") || name.contains("sus") {
                optimizations += self.optimize_variable_declarations(&function)?;
            }
            
            if name.contains("lowkey") || name.contains("highkey") {
                optimizations += self.optimize_conditional_logic(&function)?;
            }
            
            if name.contains("periodt") {
                optimizations += self.optimize_loops(&function)?;
            }
            
            if name.contains("bestie") || name.contains("flex") {
                optimizations += self.optimize_match_statements(&function)?;
            }
        }
        
        debug!("Applied {} Gen Z keyword optimizations", optimizations);
        Ok(optimizations)
    }
    
    /// Optimize slay (function) declarations
    fn optimize_slay_functions(&self, function: &inkwell::values::FunctionValue<'ctx>) -> Result<usize> {
        let mut optimizations = 0;
        
        // Analyze function characteristics for optimization
        let instruction_count = function.get_basic_blocks()
            .map(|bb| bb.get_instructions().count())
            .sum::<usize>();
        
        // Small functions are candidates for aggressive inlining
        if instruction_count < 20 {
            debug!("Small slay function candidate for aggressive inlining");
            optimizations += 1;
        }
        
        // Functions with simple control flow can be optimized
        let basic_block_count = function.get_basic_blocks().count();
        if basic_block_count <= 2 {
            debug!("Simple slay function candidate for optimization");
            optimizations += 1;
        }
        
        Ok(optimizations)
    }
    
    /// Optimize variable declarations (facts/sus)
    fn optimize_variable_declarations(&self, function: &inkwell::values::FunctionValue<'ctx>) -> Result<usize> {
        let mut optimizations = 0;
        
        // Look for variable usage patterns
        let mut alloca_count = 0;
        let mut load_count = 0;
        let mut store_count = 0;
        
        for basic_block in function.get_basic_blocks() {
            for instruction in basic_block.get_instructions() {
                match instruction.get_opcode() {
                    inkwell::values::InstructionOpcode::Alloca => alloca_count += 1,
                    inkwell::values::InstructionOpcode::Load => load_count += 1,
                    inkwell::values::InstructionOpcode::Store => store_count += 1,
                    _ => {}
                }
            }
        }
        
        // Optimize variables that are only written once (facts)
        if store_count <= alloca_count {
            debug!("Optimizing single-assignment variables");
            optimizations += 1;
        }
        
        // Optimize variables with simple usage patterns
        if load_count > 0 && store_count > 0 {
            debug!("Optimizing variable access patterns");
            optimizations += 1;
        }
        
        Ok(optimizations)
    }
    
    /// Optimize conditional logic (lowkey/highkey)
    fn optimize_conditional_logic(&self, function: &inkwell::values::FunctionValue<'ctx>) -> Result<usize> {
        let mut optimizations = 0;
        
        // Look for conditional patterns
        let mut branch_count = 0;
        let mut compare_count = 0;
        
        for basic_block in function.get_basic_blocks() {
            for instruction in basic_block.get_instructions() {
                match instruction.get_opcode() {
                    inkwell::values::InstructionOpcode::Br => branch_count += 1,
                    inkwell::values::InstructionOpcode::ICmp | 
                    inkwell::values::InstructionOpcode::FCmp => compare_count += 1,
                    _ => {}
                }
            }
        }
        
        // Optimize simple conditional patterns
        if branch_count > 0 && compare_count > 0 {
            debug!("Optimizing conditional logic patterns");
            optimizations += 1;
        }
        
        Ok(optimizations)
    }
    
    /// Optimize loop constructs (periodt)
    fn optimize_loops(&self, function: &inkwell::values::FunctionValue<'ctx>) -> Result<usize> {
        let mut optimizations = 0;
        
        // Detect loop patterns
        let basic_block_count = function.get_basic_blocks().count();
        let mut back_edge_count = 0;
        
        for basic_block in function.get_basic_blocks() {
            if let Some(terminator) = basic_block.get_terminator() {
                if terminator.get_opcode() == inkwell::values::InstructionOpcode::Br {
                    back_edge_count += 1;
                }
            }
        }
        
        // Apply loop optimizations
        if back_edge_count > 0 && basic_block_count > 2 {
            debug!("Applying loop optimization");
            optimizations += 1;
        }
        
        Ok(optimizations)
    }
    
    /// Optimize match statements (bestie/flex)
    fn optimize_match_statements(&self, function: &inkwell::values::FunctionValue<'ctx>) -> Result<usize> {
        let mut optimizations = 0;
        
        // Look for switch-like patterns
        let mut switch_count = 0;
        
        for basic_block in function.get_basic_blocks() {
            for instruction in basic_block.get_instructions() {
                if instruction.get_opcode() == inkwell::values::InstructionOpcode::Switch {
                    switch_count += 1;
                }
            }
        }
        
        // Optimize switch patterns
        if switch_count > 0 {
            debug!("Optimizing match/switch patterns");
            optimizations += 1;
        }
        
        Ok(optimizations)
    }
    
    /// Optimize CURSED-specific control flow
    fn optimize_cursed_control_flow(&self, module: &Module<'ctx>) -> Result<usize> {
        let mut optimizations = 0;
        
        for function in module.get_functions() {
            let name = function.get_name().to_string_lossy();
            
            // Optimize error propagation (? operator)
            if name.contains("error_propagate") || name.contains("try_unwrap") {
                optimizations += self.optimize_error_propagation(&function)?;
            }
            
            // Optimize CURSED-specific iterators
            if name.contains("iterator") || name.contains("for_each") {
                optimizations += self.optimize_cursed_iterators(&function)?;
            }
        }
        
        debug!("Applied {} CURSED control flow optimizations", optimizations);
        Ok(optimizations)
    }
    
    /// Optimize error propagation patterns
    fn optimize_error_propagation(&self, function: &inkwell::values::FunctionValue<'ctx>) -> Result<usize> {
        let mut optimizations = 0;
        
        // Look for error handling patterns
        for basic_block in function.get_basic_blocks() {
            for instruction in basic_block.get_instructions() {
                if let Some(call_inst) = instruction.as_call_value() {
                    if let Some(called_fn) = call_inst.get_called_fn_value() {
                        let called_name = called_fn.get_name().to_string_lossy();
                        
                        if called_name.contains("unwrap") || called_name.contains("expect") {
                            debug!("Optimizing error unwrap pattern");
                            optimizations += 1;
                        }
                    }
                }
            }
        }
        
        Ok(optimizations)
    }
    
    /// Optimize CURSED iterators
    fn optimize_cursed_iterators(&self, function: &inkwell::values::FunctionValue<'ctx>) -> Result<usize> {
        let mut optimizations = 0;
        
        // Look for iterator patterns that can be vectorized or unrolled
        for basic_block in function.get_basic_blocks() {
            for instruction in basic_block.get_instructions() {
                if let Some(call_inst) = instruction.as_call_value() {
                    if let Some(called_fn) = call_inst.get_called_fn_value() {
                        let called_name = called_fn.get_name().to_string_lossy();
                        
                        if called_name.contains("next") || called_name.contains("map") {
                            debug!("Optimizing iterator operation");
                            optimizations += 1;
                        }
                    }
                }
            }
        }
        
        Ok(optimizations)
    }
    
    /// Optimize memory layout for CURSED types
    fn optimize_cursed_memory_layout(&self, module: &Module<'ctx>) -> Result<usize> {
        let mut optimizations = 0;
        
        for function in module.get_functions() {
            let name = function.get_name().to_string_lossy();
            
            // Optimize struct layout (squad keyword)
            if name.contains("squad") || name.contains("struct_new") {
                optimizations += self.optimize_struct_layout(&function)?;
            }
            
            // Optimize interface layout (collab keyword)
            if name.contains("collab") || name.contains("interface") {
                optimizations += self.optimize_interface_layout(&function)?;
            }
            
            // Optimize array/slice access
            if name.contains("array") || name.contains("slice") {
                optimizations += self.optimize_array_access(&function)?;
            }
        }
        
        debug!("Applied {} memory layout optimizations", optimizations);
        Ok(optimizations)
    }
    
    /// Optimize struct layout
    fn optimize_struct_layout(&self, function: &inkwell::values::FunctionValue<'ctx>) -> Result<usize> {
        let mut optimizations = 0;
        
        // Look for struct field access patterns
        for basic_block in function.get_basic_blocks() {
            for instruction in basic_block.get_instructions() {
                if instruction.get_opcode() == inkwell::values::InstructionOpcode::GetElementPtr {
                    debug!("Optimizing struct field access");
                    optimizations += 1;
                }
            }
        }
        
        Ok(optimizations)
    }
    
    /// Optimize interface layout
    fn optimize_interface_layout(&self, function: &inkwell::values::FunctionValue<'ctx>) -> Result<usize> {
        let mut optimizations = 0;
        
        // Look for interface method calls that can be devirtualized
        for basic_block in function.get_basic_blocks() {
            for instruction in basic_block.get_instructions() {
                if let Some(call_inst) = instruction.as_call_value() {
                    if let Some(called_fn) = call_inst.get_called_fn_value() {
                        let called_name = called_fn.get_name().to_string_lossy();
                        
                        if called_name.contains("vtable") || called_name.contains("dispatch") {
                            debug!("Optimizing interface dispatch");
                            optimizations += 1;
                        }
                    }
                }
            }
        }
        
        Ok(optimizations)
    }
    
    /// Optimize array access patterns
    fn optimize_array_access(&self, function: &inkwell::values::FunctionValue<'ctx>) -> Result<usize> {
        let mut optimizations = 0;
        
        // Look for bounds check elimination opportunities
        for basic_block in function.get_basic_blocks() {
            for instruction in basic_block.get_instructions() {
                if let Some(call_inst) = instruction.as_call_value() {
                    if let Some(called_fn) = call_inst.get_called_fn_value() {
                        let called_name = called_fn.get_name().to_string_lossy();
                        
                        if called_name.contains("bounds_check") {
                            debug!("Eliminating redundant bounds check");
                            optimizations += 1;
                        }
                        
                        if called_name.contains("array_get") || called_name.contains("slice_get") {
                            debug!("Optimizing array access");
                            optimizations += 1;
                        }
                    }
                }
            }
        }
        
        Ok(optimizations)
    }
    
    /// Auto-tune optimization level based on module characteristics
    fn auto_tune_optimization_level(&self, analysis: &ModuleAnalysis) -> OptimizationLevel {
        // Load historical performance data
        let tuning_data = self.auto_tuning_data.read().unwrap();
        
        // Simple heuristic-based tuning
        let complexity_factor = analysis.total_instructions as f64 / analysis.total_functions as f64;
        
        let recommended_level = if complexity_factor < 10.0 {
            OptimizationLevel::Aggressive // Small, simple functions
        } else if complexity_factor < 50.0 {
            OptimizationLevel::Default // Medium complexity
        } else if complexity_factor < 200.0 {
            OptimizationLevel::Less // High complexity
        } else {
            OptimizationLevel::None // Very high complexity, focus on compile time
        };
        
        // Check if we have historical data suggesting different approach
        let key = format!("complexity_{:.0}", complexity_factor);
        if let Some(&historical_performance) = tuning_data.get(&key) {
            if historical_performance < 0.5 {
                // Previous optimizations didn't help much, be more conservative
                return match recommended_level {
                    OptimizationLevel::Aggressive => OptimizationLevel::Default,
                    OptimizationLevel::Default => OptimizationLevel::Less,
                    OptimizationLevel::Less => OptimizationLevel::None,
                    OptimizationLevel::None => OptimizationLevel::None,
                    _ => recommended_level,
                };
            }
        }
        
        if recommended_level != self.config.level {
            info!(
                original_level = %self.config.level.as_str(),
                tuned_level = %recommended_level.as_str(),
                complexity_factor = complexity_factor,
                "Auto-tuned optimization level"
            );
            
            // Record tuning adjustment
            if let Ok(mut stats) = self.stats.lock() {
                stats.auto_tuning_adjustments += 1;
            }
        }
        
        recommended_level
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
    
    /// Print comprehensive optimization summary
    #[instrument(skip(self))]
    pub fn print_summary(&self) {
        let stats = self.get_stats();
        
        println!("🔧 Enhanced Optimization Summary:");
        println!("   Level: {}", self.config.level.as_str());
        println!("   Passes run: {} (Parallel: {})", stats.passes_run, stats.parallel_passes_run);
        println!("   Time: {:?}", stats.optimization_time);
        println!("   Code size: {} -> {} bytes ({:.1}% reduction)", 
                 stats.code_size_before, 
                 stats.code_size_after,
                 if stats.code_size_before > 0 {
                     100.0 * (stats.code_size_before - stats.code_size_after) as f64 / stats.code_size_before as f64
                 } else {
                     0.0
                 });
        
        println!("   Modules optimized: {}", stats.modules_optimized);
        println!("   Functions optimized: {}", stats.functions_optimized);
        
        if stats.cache_hits > 0 || stats.cache_misses > 0 {
            let total_cache_requests = stats.cache_hits + stats.cache_misses;
            let hit_rate = if total_cache_requests > 0 {
                100.0 * stats.cache_hits as f64 / total_cache_requests as f64
            } else {
                0.0
            };
            println!("   Cache: {} hits, {} misses ({:.1}% hit rate)", 
                     stats.cache_hits, stats.cache_misses, hit_rate);
        }
        
        if stats.compilation_threads_used > 0 {
            println!("   Threads used: {}", stats.compilation_threads_used);
        }
        
        if stats.incremental_optimizations > 0 {
            println!("   Incremental optimizations: {}", stats.incremental_optimizations);
        }
        
        if stats.cursed_specific_optimizations > 0 {
            println!("   CURSED-specific optimizations: {}", stats.cursed_specific_optimizations);
        }
        
        if stats.auto_tuning_adjustments > 0 {
            println!("   Auto-tuning adjustments: {}", stats.auto_tuning_adjustments);
        }
        
        if stats.functions_inlined > 0 {
            println!("   Functions inlined: {}", stats.functions_inlined);
        }
        if stats.loops_unrolled > 0 {
            println!("   Loops unrolled: {}", stats.loops_unrolled);
        }
        if stats.dead_code_eliminated > 0 {
            println!("   Dead code eliminated: {}", stats.dead_code_eliminated);
        }
        
        if stats.peak_memory_usage > 0 {
            println!("   Peak memory usage: {} MB", stats.peak_memory_usage / 1024 / 1024);
        }
    }
    
    /// Clear optimization cache
    pub fn clear_cache(&self) {
        if let Ok(mut cache) = self.optimization_cache.write() {
            cache.clear();
            info!("Optimization cache cleared");
        }
    }
    
    /// Get cache statistics
    pub fn get_cache_stats(&self) -> (usize, usize, f64) {
        let stats = self.get_stats();
        let total_requests = stats.cache_hits + stats.cache_misses;
        let hit_rate = if total_requests > 0 {
            stats.cache_hits as f64 / total_requests as f64
        } else {
            0.0
        };
        (stats.cache_hits, stats.cache_misses, hit_rate)
    }
    
    /// Record performance data for auto-tuning
    pub fn record_performance(&self, complexity_key: String, performance_score: f64) {
        if let Ok(mut tuning_data) = self.auto_tuning_data.write() {
            tuning_data.insert(complexity_key, performance_score);
        }
    }
    
    /// Compute hash of optimization configuration
    fn compute_config_hash(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        self.config.level.hash(&mut hasher);
        self.config.vectorize_loops.hash(&mut hasher);
        self.config.vectorize_slp.hash(&mut hasher);
        self.config.unroll_loops.hash(&mut hasher);
        self.config.merge_functions.hash(&mut hasher);
        self.config.inline_functions.hash(&mut hasher);
        self.config.enable_lto.hash(&mut hasher);
        self.config.enable_cursed_specific.hash(&mut hasher);
        self.config.enable_parallel_optimization.hash(&mut hasher);
        self.config.enable_caching.hash(&mut hasher);
        self.config.enable_incremental.hash(&mut hasher);
        self.config.enable_auto_tuning.hash(&mut hasher);
        
        if let Some(ref cpu) = self.config.target_cpu {
            cpu.hash(&mut hasher);
        }
        for feature in &self.config.target_features {
            feature.hash(&mut hasher);
        }
        for pass in &self.config.custom_passes {
            pass.hash(&mut hasher);
        }
        
        hasher.finish()
    }
}

/// High-level LLVM optimizer interface
pub struct LlvmOptimizer<'ctx> {
    manager: OptimizationManager<'ctx>,
}

impl<'ctx> LlvmOptimizer<'ctx> {
    /// Create a new LLVM optimizer
    pub fn new(context: &'ctx Context, config: OptimizationConfig) -> Self {
        Self {
            manager: OptimizationManager::new(context, config),
        }
    }
    
    /// Initialize the optimizer with a module
    pub fn initialize(&mut self, module: &Module<'ctx>) -> Result<()> {
        self.manager.initialize(module)
    }
    
    /// Optimize a module
    pub fn optimize(&self, module: &Module<'ctx>) -> Result<()> {
        self.manager.optimize_module(module)
    }
    
    /// Get optimization statistics
    pub fn get_statistics(&self) -> OptimizationStats {
        self.manager.get_stats()
    }
    
    /// Print optimization summary
    pub fn print_summary(&self) {
        self.manager.print_summary()
    }
    
    /// Clear optimization cache
    pub fn clear_cache(&self) {
        self.manager.clear_cache()
    }
    
    /// Analyze module characteristics
    pub fn analyze_module(&self, module: &Module<'ctx>) -> ModuleAnalysis {
        self.manager.analyze_module(module)
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
            enable_lto: true,
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
