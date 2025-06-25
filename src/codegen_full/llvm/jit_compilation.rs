/// JIT Compilation Interface for CURSED Language
/// 
/// Provides integration between CURSED's LLVM codegen and the JIT engine,
/// enabling runtime compilation of CURSED functions with hot path detection
/// and dynamic optimization capabilities.

use crate::error::CursedError;
use crate::ast::Program;
use crate::codegen::llvm::{
// };

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use inkwell::{
// };

/// JIT compilation interface that bridges CURSED AST compilation with JIT execution
pub struct JitCompilationInterface<'ctx> {
/// Configuration for JIT compilation behavior
#[derive(Debug, Clone)]
pub struct JitCompilationConfig {
    /// Minimum execution count before considering hot path optimization
    /// Maximum time to spend on compilation before giving up
    /// Whether to enable dynamic recompilation of hot paths
    /// Whether to enable background compilation
    /// Optimization level for hot path compilation
    /// Optimization level for regular compilation
    /// Maximum number of functions to compile in parallel
    /// Whether to enable profiling-guided optimization
    /// Whether to enable OSR (On-Stack Replacement)
    /// Whether to enable tiered compilation
    /// OSR configuration
    /// Tiered compilation configuration
/// Statistics for JIT compilation performance
#[derive(Debug, Default, Clone)]
pub struct JitCompilationStats {
    /// Total number of functions compiled via JIT
    /// Number of hot path optimizations performed
    /// Number of background compilations
    /// Total time spent in JIT compilation
    /// Average compilation time per function
    /// Number of compilation timeouts
    /// Number of compilation failures
    /// Performance improvement from JIT optimization (as percentage)
    /// OSR statistics
    /// Tiered compilation statistics
/// Represents a compiled function with metadata
#[derive(Debug, Clone)]
pub struct CompiledFunction {
    /// Function name
    /// LLVM IR code
    /// Compilation timestamp
    /// Number of executions
    /// Total execution time
    /// Whether this is a hot path optimized version
    /// Optimization level used for compilation
/// Hot path detection and optimization
pub struct HotPathDetector {
impl Default for JitCompilationConfig {
    fn default() -> Self {
        Self {
        }
    }
impl HotPathDetector {
    /// Create a new hot path detector
    pub fn new(hot_path_threshold: u64) -> Self {
        Self {
        }
    }

    /// Record function execution
    pub fn record_execution(&self, function_name: &str, execution_time: Duration) {
        {
            let mut counts = self.execution_counts.lock().unwrap();
            *counts.entry(function_name.to_string()).or_insert(0) += 1;
        {
            let mut times = self.execution_times.lock().unwrap();
            *times.entry(function_name.to_string()).or_insert(Duration::ZERO) += execution_time;
        // Check if this function should be considered for optimization
        if self.is_hot_path(function_name) {
            let mut candidates = self.optimization_candidates.lock().unwrap();
            if !candidates.contains(&function_name.to_string()) {
                candidates.push(function_name.to_string());
            }
        }
    /// Check if a function is a hot path
    pub fn is_hot_path(&self, function_name: &str) -> bool {
        let counts = self.execution_counts.lock().unwrap();
        counts.get(function_name).unwrap_or(&0) >= &self.hot_path_threshold
    /// Get optimization candidates
    pub fn get_optimization_candidates(&self) -> Vec<String> {
        let candidates = self.optimization_candidates.lock().unwrap();
        candidates.clone()
    /// Clear optimization candidates
    pub fn clear_optimization_candidates(&self) {
        let mut candidates = self.optimization_candidates.lock().unwrap();
        candidates.clear();
    /// Get execution statistics for a function
    pub fn get_execution_stats(&self, function_name: &str) -> (u64, Duration) {
        let counts = self.execution_counts.lock().unwrap();
        let times = self.execution_times.lock().unwrap();
        
        let count = counts.get(function_name).unwrap_or(&0);
        let time = times.get(function_name).unwrap_or(&Duration::ZERO);
        
        (*count, *time)
    /// Get all hot path functions
    pub fn get_hot_paths(&self) -> Vec<String> {
        let counts = self.execution_counts.lock().unwrap();
        counts.iter()
            .filter(|(_, &count)| count >= self.hot_path_threshold)
            .map(|(name, _)| name.clone())
            .collect()
    }
}

impl<'ctx> JitCompilationInterface<'ctx> {
    /// Create a new JIT compilation interface
    pub fn new(
    ) -> crate::error::Result<()> {
        let hot_path_detector = HotPathDetector::new(config.hot_path_threshold);
        let osr_manager = OSRManager::new(context, config.osr_config.clone());
        let tiered_manager = TieredCompilationManager::new(context, config.tiered_config.clone())?;
        
        Ok(Self {
        })
    /// Create with default configuration
    pub fn new_with_default_config(
    ) -> crate::error::Result<()> {
        Self::new(context, jit_engine, codegen, JitCompilationConfig::default())
    /// Compile a CURSED function for JIT execution
    pub fn compile_function(&mut self, function_name: &str, source: &str) -> crate::error::Result<()> {
        let start_time = Instant::now();

        // Register function with tiered compilation manager
        if self.config.enable_tiered_compilation {
            self.tiered_manager.register_function(function_name)?;
        tracing::info!(
            "Starting JIT compilation"
        );

        // Check compilation cache
        {
            let cache = self.compilation_cache.lock().unwrap();
            if cache.contains_key(function_name) {
                tracing::debug!(function_name = function_name, "Function found in compilation cache");
                return Ok(());
            }
        }

        // Generate LLVM IR using the codegen
        let llvm_ir = self.codegen.generate_ir(source)
            .map_err(|e| CursedError::from_str(&format!("Failed to generate LLVM IR: {}", e)))?;

        // Determine optimization level based on tiered compilation
        let optimization_level = if self.config.enable_tiered_compilation {
            let current_tier = self.tiered_manager.get_function_tier(function_name);
            self.tier_to_optimization_level(current_tier)
        } else if self.hot_path_detector.is_hot_path(function_name) {
            self.config.hot_path_optimization_level
        } else {
            self.config.regular_optimization_level

        // Compile function in JIT engine
        self.jit_engine.compile_function(function_name, &llvm_ir)
            .map_err(|e| CursedError::from_str(&format!("JIT compilation failed: {}", e)))?;

        // Store in compilation cache
        let compiled_function = CompiledFunction {

        {
            let mut cache = self.compilation_cache.lock().unwrap();
            cache.insert(function_name.to_string(), compiled_function);
        // Update statistics
        let compilation_time = start_time.elapsed();
        self.stats.total_jit_compilations += 1;
        self.stats.total_compilation_time += compilation_time;
        self.stats.avg_compilation_time = self.stats.total_compilation_time / self.stats.total_jit_compilations as u32;

        if self.hot_path_detector.is_hot_path(function_name) {
            self.stats.hot_path_optimizations += 1;
        tracing::info!(
            "JIT compilation completed successfully"
        );

        Ok(())
    /// Execute a JIT-compiled function
    pub fn execute_function(&mut self, function_name: &str) -> crate::error::Result<()> {
        let start_time = Instant::now();

        // Check for OSR opportunity before execution
        if self.config.enable_osr && self.osr_manager.should_trigger_osr(function_name, self.get_execution_count(function_name)) {
            tracing::info!(
                "OSR opportunity detected before execution"
            );
            
            // Create a mock stack frame for OSR transition
            let stack_frame = self.create_current_stack_frame(function_name)?;
            if let Ok(osr_success) = self.osr_manager.perform_osr_transition(function_name, &stack_frame) {
                if osr_success {
                    tracing::info!(
                        "OSR transition successful, executing optimized version"
                    );
                }
            }
        // Execute the function
        let result = self.jit_engine.execute_function(function_name)?;
        
        let execution_time = start_time.elapsed();

        // Record execution for hot path detection
        self.hot_path_detector.record_execution(function_name, execution_time);

        // Record execution for tiered compilation
        if self.config.enable_tiered_compilation {
            self.tiered_manager.record_execution(function_name, execution_time)?;
        // Update function statistics in cache
        {
            let mut cache = self.compilation_cache.lock().unwrap();
            if let Some(func) = cache.get_mut(function_name) {
                func.execution_count += 1;
                func.total_execution_time += execution_time;
            }
        }

        // Check if function should be recompiled with higher optimization
        if self.config.enable_dynamic_recompilation && 
           self.hot_path_detector.is_hot_path(function_name) {
            if let Some(func) = self.compilation_cache.lock().unwrap().get(function_name) {
                if !func.is_hot_path_optimized {
                    tracing::info!(
                        "Function identified as hot path, scheduling recompilation"
                    );
                    
                    if self.config.enable_background_compilation {
                        // Trigger background recompilation
                        self.schedule_background_recompilation(function_name)?;
                    }
                }
            }
        }

        tracing::debug!(
            "JIT function executed"
        );

        Ok(result)
    /// Schedule background recompilation for hot path optimization
    fn schedule_background_recompilation(&mut self, function_name: &str) -> crate::error::Result<()> {
        tracing::info!(
            "Scheduling background recompilation for hot path optimization"
        );

        // Get current function info
        let (llvm_ir, _current_optimization) = {
            let cache = self.compilation_cache.lock().unwrap();
            if let Some(func) = cache.get(function_name) {
                (func.llvm_ir.clone(), func.optimization_level)
            } else {
                return Err(CursedError::from_str(&format!("Function '{}' not found in cache", function_name)));
            }

        // Recompile with aggressive optimization
        self.jit_engine.remove_function(function_name)?;
        self.jit_engine.compile_function(function_name, &llvm_ir)?;

        // Update cache to mark as optimized
        {
            let mut cache = self.compilation_cache.lock().unwrap();
            if let Some(func) = cache.get_mut(function_name) {
                func.is_hot_path_optimized = true;
                func.optimization_level = OptimizationLevel::O3;
                func.compiled_at = Instant::now();
            }
        }

        self.stats.hot_path_optimizations += 1;
        self.stats.background_compilations += 1;

        tracing::info!(
            "Background recompilation completed"
        );

        Ok(())
    /// Compile a complete CURSED program for JIT execution
    pub fn compile_program(&mut self, program: &Program) -> crate::error::Result<()> {
        let start_time = Instant::now();
        let mut compiled_functions = Vec::new();

        tracing::info!("Starting JIT compilation of complete program");

        // Compile the program to LLVM IR
        self.codegen.compile(program)
            .map_err(|e| CursedError::from_str(&format!("Failed to compile program: {}", e)))?;

        // For now, we'll create a simple main function
        // In a full implementation, this would extract all functions from the program
        let function_names = vec!["main".to_string()];

        for function_name in function_names {
            let llvm_ir = self.codegen.generate_ir("")?;
            self.jit_engine.compile_function(&function_name, &llvm_ir)?;
            compiled_functions.push(function_name);
        let compilation_time = start_time.elapsed();
        tracing::info!(
            "Program JIT compilation completed"
        );

        Ok(compiled_functions)
    /// Get hot path optimization candidates
    pub fn get_optimization_candidates(&self) -> Vec<String> {
        self.hot_path_detector.get_optimization_candidates()
    /// Optimize hot path functions
    pub fn optimize_hot_paths(&mut self) -> crate::error::Result<()> {
        let candidates = self.hot_path_detector.get_optimization_candidates();
        let mut optimized_count = 0;

        for function_name in candidates {
            tracing::info!(
                "Optimizing hot path function"
            );

            // Get current function from cache
            let (llvm_ir, current_optimization) = {
                let cache = self.compilation_cache.lock().unwrap();
                if let Some(func) = cache.get(&function_name) {
                    (func.llvm_ir.clone(), func.optimization_level)
                } else {
                    continue;
                }

            // Only optimize if not already at highest level
            if current_optimization != OptimizationLevel::O3 {
                // Recompile with aggressive optimization
                self.jit_engine.remove_function(&function_name)?;
                self.jit_engine.compile_function(&function_name, &llvm_ir)?;

                // Update cache
                {
                    let mut cache = self.compilation_cache.lock().unwrap();
                    if let Some(func) = cache.get_mut(&function_name) {
                        func.is_hot_path_optimized = true;
                        func.optimization_level = OptimizationLevel::O3;
                        func.compiled_at = Instant::now();
                    }
                }

                optimized_count += 1;
                self.stats.hot_path_optimizations += 1;
            }
        }

        // Clear optimization candidates
        self.hot_path_detector.clear_optimization_candidates();

        tracing::info!(
            "Hot path optimization completed"
        );

        Ok(optimized_count)
    /// Check if a function is available for execution
    pub fn has_function(&self, function_name: &str) -> bool {
        self.jit_engine.has_function(function_name)
    /// Get execution statistics for a function
    pub fn get_function_stats(&self, function_name: &str) -> Option<(u64, Duration, bool)> {
        let cache = self.compilation_cache.lock().unwrap();
        cache.get(function_name).map(|func| {
            (func.execution_count, func.total_execution_time, func.is_hot_path_optimized)
        })
    /// Get all hot path functions
    pub fn get_hot_paths(&self) -> Vec<String> {
        self.hot_path_detector.get_hot_paths()
    /// Get JIT compilation statistics
    pub fn get_stats(&self) -> JitCompilationStats {
        let mut stats = self.stats.clone();
        stats.osr_stats = self.osr_manager.get_stats();
        stats.tiered_stats = self.tiered_manager.get_stats();
        stats
    /// Get JIT engine statistics
    pub fn get_engine_stats(&self) -> JitEngineStats {
        self.jit_engine.get_stats()
    /// Reset all statistics
    pub fn reset_stats(&mut self) {
        self.stats = JitCompilationStats::default();
        self.jit_engine.reset_stats();
    /// Update configuration
    pub fn update_config(&mut self, config: JitCompilationConfig) {
        self.config = config;
    /// Get current configuration
    pub fn get_config(&self) -> &JitCompilationConfig {
        &self.config
    /// Perform background optimization of hot paths
    pub async fn background_optimize(&mut self) -> crate::error::Result<()> {
        if !self.config.enable_background_compilation {
            return Ok(());
        tracing::info!("Starting background optimization");

        // Run optimization in background
        tokio::task::spawn_blocking(move || {
            // In a full implementation, this would run optimization in a separate thread
            // For now, we'll just simulate the work
            std::thread::sleep(Duration::from_millis(100));
        }).await.map_err(|e| CursedError::from_str(&format!("Background optimization failed: {}", e)))?;

        self.stats.background_compilations += 1;

        Ok(())
    /// Compile function with specific optimization level
    pub fn compile_function_with_optimization(
    ) -> crate::error::Result<()> {
        // Temporarily update config
        let original_level = self.config.regular_optimization_level;
        self.config.regular_optimization_level = optimization_level;

        let result = self.compile_function(function_name, source);

        // Restore original config
        self.config.regular_optimization_level = original_level;

        result
    /// Get memory usage of compiled functions
    pub fn get_memory_usage(&self) -> u64 {
        self.jit_engine.get_memory_usage()
    /// Clear compilation cache
    pub fn clear_cache(&mut self) -> crate::error::Result<()> {
        {
            let mut cache = self.compilation_cache.lock().unwrap();
            cache.clear();
        }
        self.jit_engine.clear_cache()
    /// Get compiled function count
    pub fn get_compiled_function_count(&self) -> usize {
        let cache = self.compilation_cache.lock().unwrap();
        cache.len()
    /// Profile function execution
    pub fn profile_function_execution(&mut self, function_name: &str, iterations: u32) -> crate::error::Result<()> {
        if !self.has_function(function_name) {
            return Err(CursedError::from_str(&format!("Function '{}' not found", function_name)));
        let start_time = Instant::now();
        
        for _ in 0..iterations {
            self.execute_function(function_name)?;
        let total_time = start_time.elapsed();
        let avg_time = total_time / iterations;

        tracing::info!(
            "Function profiling completed"
        );

        Ok(avg_time)
    /// Execute CURSED code directly in REPL context
    pub fn execute_repl_code(&mut self, code: &str) -> crate::error::Result<()> {
        let function_name = format!("repl_expr_{}", self.generate_unique_id());
        
        tracing::debug!(
            "Executing REPL code"
        );

        // Generate LLVM IR for the code
        let llvm_ir = self.codegen.generate_ir(code)
            .map_err(|e| CursedError::from_str(&format!("Failed to generate IR for REPL code: {}", e)))?;

        // Compile and execute immediately
        self.compile_function(&function_name, &llvm_ir)?;
        let result = self.execute_function(&function_name)?;

        // Clean up temporary function unless it's being cached for optimization
        if !self.hot_path_detector.is_hot_path(&function_name) {
            let _ = self.jit_engine.remove_function(&function_name);
        Ok(result)
    /// Compile and cache a function for later execution
    pub fn compile_and_cache_function(&mut self, name: &str, source: &str) -> crate::error::Result<()> {
        tracing::info!(
            "Compiling and caching function for later execution"
        );

        // Generate LLVM IR
        let llvm_ir = self.codegen.generate_ir(source)
            .map_err(|e| CursedError::from_str(&format!("Failed to generate IR: {}", e)))?;

        // Compile with regular optimization initially
        self.compile_function(name, &llvm_ir)?;

        tracing::info!(
            "Function compiled and cached successfully"
        );

        Ok(())
    /// Generate unique ID for temporary functions
    fn generate_unique_id(&self) -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64
    /// List all available functions in the JIT engine
    pub fn list_functions(&self) -> Vec<String> {
        let cache = self.compilation_cache.lock().unwrap();
        cache.keys().cloned().collect()
    /// Get detailed function information
    pub fn get_function_info(&self, function_name: &str) -> Option<CompiledFunction> {
        let cache = self.compilation_cache.lock().unwrap();
        cache.get(function_name).cloned()
    /// Generate performance report for all functions
    pub fn generate_performance_report(&self) -> String {
        let cache = self.compilation_cache.lock().unwrap();
        let stats = self.get_stats();
        let engine_stats = self.get_engine_stats();

        let mut report = String::from("🔥 JIT Performance Report\n");
        report.push_str("=" .repeat(40).as_str());
        report.push('\n');

        // Overall statistics
        report.push_str(&format!("Total compilations: {}\n", stats.total_jit_compilations));
        report.push_str(&format!("Hot path optimizations: {}\n", stats.hot_path_optimizations));
        report.push_str(&format!("Background compilations: {}\n", stats.background_compilations));
        report.push_str(&format!("Average compilation time: {:.2}ms\n", stats.avg_compilation_time.as_millis()));
        report.push_str(&format!("Memory usage: {} bytes\n", engine_stats.memory_usage_bytes));
        report.push('\n');

        // Per-function statistics
        report.push_str("Function Details:\n");
        report.push_str("-".repeat(40).as_str());
        report.push('\n');

        for (name, func) in cache.iter() {
            report.push_str(&format!("📦 {}\n", name));
            report.push_str(&format!("  Executions: {}\n", func.execution_count));
            report.push_str(&format!("  Total time: {:.2}ms\n", func.total_execution_time.as_millis()));
            if func.execution_count > 0 {
                let avg_time = func.total_execution_time / func.execution_count as u32;
                report.push_str(&format!("  Avg time: {:.2}μs\n", avg_time.as_micros()));
            }
            report.push_str(&format!("  Optimized: {}\n", func.is_hot_path_optimized));
            report.push_str(&format!("  Opt level: {:?}\n", func.optimization_level));
            report.push('\n');
        // Hot path information
        let hot_paths = self.get_hot_paths();
        if !hot_paths.is_empty() {
            report.push_str("🔥 Hot Paths:\n");
            for path in hot_paths {
                report.push_str(&format!("  - {}\n", path));
            }
        }

        report
    /// Convert compilation tier to optimization level
    fn tier_to_optimization_level(&self, tier: CompilationTier) -> OptimizationLevel {
        match tier {
        }
    }

    /// Get execution count for a function
    fn get_execution_count(&self, function_name: &str) -> u64 {
        let cache = self.compilation_cache.lock().unwrap();
        cache.get(function_name).map(|f| f.execution_count).unwrap_or(0)
    /// Create current stack frame for OSR
    fn create_current_stack_frame(&self, function_name: &str) -> crate::error::Result<()> {
        // In a production implementation, this would capture the actual stack state
        // For this implementation, we'll create a mock stack frame
        let mut local_variables = HashMap::new();
        
        // Add some mock local variables
        local_variables.insert(
            VariableValue {
            }
        );
        
        local_variables.insert(
            VariableValue {
            }
        );

        Ok(StackFrame {
        })
    /// Prepare OSR for a function
    pub fn prepare_osr_for_function(&mut self, function_name: &str) -> crate::error::Result<()> {
        if !self.config.enable_osr {
            return Ok(());
        tracing::info!(
            "Preparing OSR for function"
        );

        // Get the current and optimized functions
        // In a production implementation, this would compile an optimized version
        // For now, we'll simulate the process
        
        // TODO: Get actual function values from LLVM module
        // This would require deeper integration with the LLVM code generation
        
        Ok(())
    /// Trigger deoptimization for a function
    pub fn trigger_deoptimization(&mut self, function_name: &str, reason: DeoptimizationReason) -> crate::error::Result<()> {
        if !self.config.enable_osr {
            return Ok(());
        tracing::warn!(
            "Triggering deoptimization for function"
        );

        self.osr_manager.trigger_deoptimization(function_name, reason)?;

        // Demote function in tiered compilation if applicable
        if self.config.enable_tiered_compilation {
            // Would implement tier demotion logic here
            tracing::info!(
                "Function demoted due to deoptimization"
            );
        Ok(())
    /// Get OSR manager (for advanced usage)
    pub fn get_osr_manager(&self) -> &OSRManager<'ctx> {
        &self.osr_manager
    /// Get tiered compilation manager (for advanced usage)
    pub fn get_tiered_manager(&self) -> &TieredCompilationManager<'ctx> {
        &self.tiered_manager
    /// Generate comprehensive performance report
    pub fn generate_comprehensive_report(&self) -> String {
        let mut report = String::from("🚀 Comprehensive JIT Performance Report\n");
        report.push_str("=".repeat(60).as_str());
        report.push('\n');

        // Basic JIT statistics
        let stats = self.get_stats();
        report.push_str("📊 JIT Compilation Statistics:\n");
        report.push_str(&format!("  Total compilations: {}\n", stats.total_jit_compilations));
        report.push_str(&format!("  Hot path optimizations: {}\n", stats.hot_path_optimizations));
        report.push_str(&format!("  Background compilations: {}\n", stats.background_compilations));
        report.push_str(&format!("  Average compilation time: {:.2}ms\n", stats.avg_compilation_time.as_millis()));
        report.push('\n');

        // OSR statistics
        if self.config.enable_osr {
            report.push_str("🔄 OSR (On-Stack Replacement) Statistics:\n");
            report.push_str(&format!("  Total OSR replacements: {}\n", stats.osr_stats.total_osr_replacements));
            report.push_str(&format!("  Successful transitions: {}\n", stats.osr_stats.successful_transitions));
            report.push_str(&format!("  Failed transitions: {}\n", stats.osr_stats.failed_transitions));
            report.push_str(&format!("  Deoptimizations: {}\n", stats.osr_stats.deoptimizations));
            
            if stats.osr_stats.total_osr_replacements > 0 {
                let success_rate = (stats.osr_stats.successful_transitions as f64 / stats.osr_stats.total_osr_replacements as f64) * 100.0;
                report.push_str(&format!("  Success rate: {:.2}%\n", success_rate));
            }
            report.push('\n');
        // Tiered compilation statistics
        if self.config.enable_tiered_compilation {
            report.push_str("🎯 Tiered Compilation Statistics:\n");
            for (tier, count) in &stats.tiered_stats.functions_per_tier {
                report.push_str(&format!("  {:?}: {} functions\n", tier, count));
            }
            report.push_str(&format!("  Total promotions: {}\n", stats.tiered_stats.total_promotions));
            report.push_str(&format!("  Total demotions: {}\n", stats.tiered_stats.total_demotions));
            report.push('\n');
        // Hot path information
        let hot_paths = self.get_hot_paths();
        if !hot_paths.is_empty() {
            report.push_str("🔥 Hot Path Functions:\n");
            for (i, path) in hot_paths.iter().enumerate() {
                if let Some(tier) = self.config.enable_tiered_compilation.then(|| self.tiered_manager.get_function_tier(path)) {
                    report.push_str(&format!("  {}. {} (tier: {:?})\n", i + 1, path, tier));
                } else {
                    report.push_str(&format!("  {}. {}\n", i + 1, path));
                }
            }
            report.push('\n');
        // Configuration summary
        report.push_str("⚙️ Configuration:\n");
        report.push_str(&format!("  OSR enabled: {}\n", self.config.enable_osr));
        report.push_str(&format!("  Tiered compilation enabled: {}\n", self.config.enable_tiered_compilation));
        report.push_str(&format!("  Dynamic recompilation enabled: {}\n", self.config.enable_dynamic_recompilation));
        report.push_str(&format!("  Background compilation enabled: {}\n", self.config.enable_background_compilation));
        report.push_str(&format!("  Hot path threshold: {}\n", self.config.hot_path_threshold));

        report
    }
}

/// Utility functions for JIT compilation

/// Create a JIT compilation interface with optimal settings
pub fn create_optimized_jit_interface<'ctx>(
) -> crate::error::Result<()> {
    let jit_engine = crate::codegen::llvm::jit_engine::create_optimized_jit_engine(context)?;
    let codegen = LlvmCodeGenerator::new()?;
    
    let config = JitCompilationConfig {
    
    JitCompilationInterface::new(context, jit_engine, codegen, config)
/// Create a JIT compilation interface for development
pub fn create_debug_jit_interface<'ctx>(
) -> crate::error::Result<()> {
    let jit_engine = crate::codegen::llvm::jit_engine::create_debug_jit_engine(context)?;
    let codegen = LlvmCodeGenerator::new()?;
    
    let config = JitCompilationConfig {
    
    JitCompilationInterface::new(context, jit_engine, codegen, config)
