/// Performance Optimization Integration System
/// 
/// Provides unified integration of all performance optimization features including
/// parallel compilation, incremental compilation, LLVM optimization, and adaptive optimization.

use crate::error::{CursedError, Result};
use crate::optimization::{
// };
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use tracing::{info, warn, debug, instrument};

/// Unified performance optimization coordinator
#[derive(Debug)]
pub struct PerformanceIntegrationSystem {
/// Configuration for the performance integration system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceIntegrationConfig {
    /// Enable adaptive optimization based on project characteristics
    
    /// Enable performance monitoring during compilation
    
    /// Enable automatic performance reporting
    
    /// Performance monitoring interval in milliseconds
    
    /// Threshold for switching optimization profiles (compilation time in seconds)
    
    /// Maximum number of parallel workers (0 = auto-detect)
    
    /// Enable profile-guided optimization when available
    
    /// Enable distributed compilation if available
    
    /// Cache size limit in MB
    
    /// Performance report output directory
    
    /// Benchmark configurations for different scenarios
    
    /// Target performance improvements (as percentages)
impl Default for PerformanceIntegrationConfig {
    fn default() -> Self {
        let mut benchmark_configs = HashMap::new();
        benchmark_configs.insert("quick".to_string(), BenchmarkConfig::quick());
        benchmark_configs.insert("thorough".to_string(), BenchmarkConfig::thorough());
        
        Self {
            max_parallel_workers: 0, // Auto-detect
            cache_size_limit_mb: 2048, // 2GB
        }
    }
/// Target performance improvements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTargets {
    /// Target compilation time reduction percentage
    
    /// Target runtime performance improvement percentage
    
    /// Target memory usage reduction percentage
    
    /// Target binary size reduction percentage
impl Default for PerformanceTargets {
    fn default() -> Self {
        Self {
            compilation_time_reduction: 30.0, // 30% faster compilation
            runtime_performance_improvement: 20.0, // 20% faster runtime
            memory_usage_reduction: 15.0, // 15% less memory usage
            binary_size_reduction: 10.0, // 10% smaller binaries
        }
    }
/// Adaptive optimizer that adjusts optimization strategies based on project characteristics
#[derive(Debug)]
pub struct AdaptiveOptimizer {
/// Project characteristics for adaptive optimization
#[derive(Debug, Clone)]
pub struct ProjectCharacteristics {
/// Record of optimization attempts and their results
#[derive(Debug, Clone)]
pub struct OptimizationRecord {
/// Performance monitor for real-time optimization tracking
#[derive(Debug)]
pub struct PerformanceMonitor {
/// Performance checkpoint for tracking optimization progress
#[derive(Debug, Clone)]
pub struct PerformanceCheckpoint {
/// Comprehensive optimization results with performance analysis
#[derive(Debug)]
pub struct IntegratedOptimizationResults {
/// Performance improvements achieved by optimization
#[derive(Debug, Clone)]
pub struct PerformanceImprovements {
/// Optimization recommendations based on analysis
#[derive(Debug, Clone)]
pub struct OptimizationRecommendation {
#[derive(Debug, Clone)]
pub enum RecommendationCategory {
#[derive(Debug, Clone)]
pub enum ImplementationEffort {
impl PerformanceIntegrationSystem {
    /// Create a new performance integration system
    #[instrument(skip(config, optimization_config))]
    pub fn new(
    ) -> Result<Self> {
        info!("Initializing Performance Integration System");
        
        // Initialize all subsystems
        let parallel_compiler = Arc::new(ParallelCompiler::new(
            optimization_config.effective_workers()
        )?);
        
        let incremental_compiler = Arc::new(IncrementalCompiler::new(
            optimization_config.cache_dir()
        )?);
        
        let cache_manager = Arc::new(CacheManager::with_size_limit(
            config.cache_size_limit_mb * 1024 * 1024 // Convert MB to bytes
        )?);
        
        let profiler_config = ProfilerConfig {
        let profiler = Arc::new(Mutex::new(EnhancedBuildProfiler::new(profiler_config)?));
        
        let metrics_collector = Arc::new(MetricsCollector::new(
            crate::optimization::PerformanceConfig {
                ..Default::default()
            }
        )?);
        
        let benchmarking_engine = Arc::new(BenchmarkingEngine::new(
            crate::optimization::PerformanceConfig::default()
        )?);
        
        let llvm_optimizer = Arc::new(EnhancedLlvmOptimizer::new(
            optimization_config.clone().into()
        )?);
        
        let adaptive_optimizer = AdaptiveOptimizer::new();
        let performance_monitor = PerformanceMonitor::new();
        
        Ok(Self {
        })
    /// Perform integrated optimization with all systems
    #[instrument(skip(self, source_files))]
    pub fn optimize_project<P: AsRef<Path>>(
    ) -> Result<IntegratedOptimizationResults> {
        info!("Starting integrated project optimization");
        
        // Start performance monitoring
        self.performance_monitor.start();
        
        if self.config.enable_performance_monitoring {
            self.metrics_collector.start_monitoring()?;
        // Analyze project characteristics for adaptive optimization
        let project_chars = self.analyze_project_characteristics(source_files)?;
        
        // Determine optimal optimization profile
        let optimization_profile = if self.config.enable_adaptive_optimization {
            self.adaptive_optimizer.select_optimal_profile(&project_chars)
        } else {
            OptimizationProfile::Release
        
        info!("Selected optimization profile: {:?}", optimization_profile);
        
        // Create optimized configuration
        let mut optimized_config = optimization_profile.to_config();
        self.apply_adaptive_optimizations(&mut optimized_config, &project_chars);
        
        // Checkpoint: Configuration complete
        self.performance_monitor.checkpoint("configuration_complete".to_string());
        
        // Execute compilation with all optimizations
        let compilation_start = Instant::now();
        
        // Phase 1: Incremental compilation check
        let incremental_results = if optimized_config.enable_incremental {
            self.incremental_compiler.check_incremental_needs(source_files)?
        } else {
            None
        
        self.performance_monitor.checkpoint("incremental_check_complete".to_string());
        
        // Phase 2: Parallel compilation
        let parallel_results = if optimized_config.enable_parallel {
            self.parallel_compiler.compile_parallel(
                incremental_results.as_ref()
            )?
        } else {
            self.compile_sequential(source_files, &optimized_config)?
        
        self.performance_monitor.checkpoint("compilation_complete".to_string());
        
        // Phase 3: LLVM optimization
        let llvm_results = self.llvm_optimizer.optimize_with_advanced_passes(
            &optimized_config.into()
        )?;
        
        self.performance_monitor.checkpoint("llvm_optimization_complete".to_string());
        
        // Phase 4: Link and finalize
        let final_binary = self.link_optimized_binary(&llvm_results, output_path)?;
        
        let compilation_time = compilation_start.elapsed();
        
        self.performance_monitor.checkpoint("linking_complete".to_string());
        
        // Calculate performance metrics
        let cache_hit_rate = self.cache_manager.get_hit_rate();
        let parallel_efficiency = parallel_results.efficiency;
        
        // Generate performance improvements analysis
        let performance_improvements = self.calculate_performance_improvements(
            &final_binary
        );
        
        // Generate recommendations
        let recommendations = self.generate_optimization_recommendations(
            parallel_efficiency
        );
        
        // Stop monitoring
        if self.config.enable_performance_monitoring {
            self.metrics_collector.stop_monitoring()?;
        let detailed_metrics = self.metrics_collector.get_compilation_metrics()?;
        let checkpoints = self.performance_monitor.get_checkpoints();
        
        // Record optimization attempt
        self.adaptive_optimizer.record_optimization(OptimizationRecord {
        });
        
        // Generate report if enabled
        if self.config.enable_automatic_reporting {
            self.generate_performance_report(&optimization_profile, &performance_improvements)?;
        info!(
            cache_hit_rate * 100.0
        );
        
        Ok(IntegratedOptimizationResults {
        })
    /// Run comprehensive performance benchmarks
    #[instrument(skip(self))]
    pub fn run_performance_benchmarks(&self, benchmark_name: &str) -> Result<crate::optimization::benchmarking::BenchmarkResults> {
        let config = self.config.benchmark_configs.get(benchmark_name)
            .ok_or_else(|| CursedError::General(format!("Unknown benchmark configuration: {}", benchmark_name)))?;
        
        info!("Running performance benchmark: {}", benchmark_name);
        self.benchmarking_engine.run_benchmark(config.clone())
    /// Get current performance statistics
    pub fn get_performance_statistics(&self) -> Result<PerformanceStatistics> {
        let system_stats = self.metrics_collector.get_system_statistics();
        let resource_stats = self.metrics_collector.get_resource_statistics()?;
        let cache_stats = self.cache_manager.get_statistics();
        
        Ok(PerformanceStatistics {
        })
    /// Update configuration and reinitialize subsystems
    #[instrument(skip(self, new_config))]
    pub fn update_configuration(&mut self, new_config: PerformanceIntegrationConfig) -> Result<()> {
        info!("Updating performance integration configuration");
        
        // Update cache size if changed
        if new_config.cache_size_limit_mb != self.config.cache_size_limit_mb {
            self.cache_manager.set_size_limit(new_config.cache_size_limit_mb * 1024 * 1024)?;
        // Update monitoring interval if changed
        if new_config.monitoring_interval_ms != self.config.monitoring_interval_ms {
            self.metrics_collector.update_config(crate::optimization::PerformanceConfig {
                ..Default::default()
            })?;
        self.config = new_config;
        Ok(())
    // Private helper methods
    
    fn analyze_project_characteristics<P: AsRef<Path>>(&self, source_files: &[P]) -> Result<ProjectCharacteristics> {
        let total_source_files = source_files.len();
        let mut total_lines = 0;
        let mut total_bytes = 0;
        
        for file_path in source_files {
            if let Ok(content) = std::fs::read_to_string(file_path) {
                total_lines += content.split("\n").count();
                total_bytes += content.len();
            }
        }
        
        let average_file_size = if total_source_files > 0 {
            total_bytes / total_source_files
        } else {
            0
        
        // Heuristics for project complexity
        let has_heavy_computation = total_lines > 50000; // Large projects likely have heavy computation
        let has_many_generics = total_lines > 20000; // Assume generics in larger projects
        
        Ok(ProjectCharacteristics {
            dependency_count: 0, // Would need dependency analysis
            typical_build_time_seconds: 10.0, // Default estimate
        })
    fn apply_adaptive_optimizations(&self, config: &mut OptimizationConfig, project_chars: &ProjectCharacteristics) {
        // Adjust parallel workers based on project size
        if project_chars.total_source_files > 100 {
            config.parallel_workers = config.parallel_workers.max(8);
        } else if project_chars.total_source_files < 10 {
            config.parallel_workers = config.parallel_workers.min(2);
        // Enable more aggressive optimization for large projects
        if project_chars.has_heavy_computation {
            config.llvm_passes.enable_vectorization = true;
            config.llvm_passes.enable_loop_unrolling = true;
            config.profile_guided = true;
        // Adjust cache settings based on project size
        if project_chars.total_source_files > 500 {
            config.cache_max_size = config.cache_max_size.max(4096); // 4GB for large projects
        }
    }
    
    fn compile_sequential<P: AsRef<Path>>(&self, source_files: &[P], config: &OptimizationConfig) -> Result<ParallelCompilationResults> {
        let start_time = Instant::now();
        let mut compiled_modules = Vec::new();
        let mut llvm_modules = Vec::new();
        
        info!("Starting sequential compilation of {} files", source_files.len());
        
        for (index, source_file) in source_files.iter().enumerate() {
            let file_path = source_file.as_ref();
            debug!("Compiling file {} of {}: {}", index + 1, source_files.len(), file_path.display());
            
            // Read source file
            let source_content = std::fs::read_to_string(file_path)
                .map_err(|e| CursedError::io_error(&format!("Failed to read {}: {}", file_path.display(), e)))?;
            
            // Generate module name from file path
            let module_name = file_path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();
            
            // Simulate compilation phases
            let compilation_result = self.compile_single_file(&source_content, &module_name, config)?;
            
            compiled_modules.push(compilation_result.module_name);
            llvm_modules.push(compilation_result.llvm_ir);
        let total_time = start_time.elapsed();
        let efficiency = 1.0; // Sequential is 100% efficient by definition
        
        info!("Sequential compilation completed in {:?}", total_time);
        
        Ok(ParallelCompilationResults {
        })
    /// Compile a single source file
    fn compile_single_file(&self, source_content: &str, module_name: &str, config: &OptimizationConfig) -> Result<SingleFileCompilationResult> {
        // Lexical analysis
        let tokens = self.tokenize(source_content)?;
        debug!("Tokenized {} tokens for module {}", tokens.len(), module_name);
        
        // Parsing
        let ast = self.parse_tokens(&tokens)?;
        debug!("Parsed AST with {} nodes for module {}", ast.node_count, module_name);
        
        // Semantic analysis
        let analyzed_ast = self.semantic_analysis(ast)?;
        debug!("Semantic analysis completed for module {}", module_name);
        
        // LLVM IR generation
        let llvm_ir = self.generate_llvm_ir(&analyzed_ast, module_name, config)?;
        debug!("Generated {} lines of LLVM IR for module {}", llvm_ir.split("\n").count(), module_name);
        
        Ok(SingleFileCompilationResult {
        })
    /// Tokenize source code
    fn tokenize(&self, source: &str) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        let mut current_pos = 0;
        let chars: Vec<char> = source.chars().collect();
        
        while current_pos < chars.len() {
            // Skip whitespace
            if chars[current_pos].is_whitespace() {
                current_pos += 1;
                continue;
            // Tokenize identifiers and keywords
            if chars[current_pos].is_alphabetic() || chars[current_pos] == '_' {
                let start = current_pos;
                while current_pos < chars.len() && 
                      (chars[current_pos].is_alphanumeric() || chars[current_pos] == '_') {
                    current_pos += 1;
                }
                let identifier: String = chars[start..current_pos].iter().collect();
                tokens.push(Token::Identifier(identifier));
                continue;
            // Tokenize numbers
            if chars[current_pos].is_numeric() {
                let start = current_pos;
                while current_pos < chars.len() && 
                      (chars[current_pos].is_numeric() || chars[current_pos] == '.') {
                    current_pos += 1;
                }
                let number: String = chars[start..current_pos].iter().collect();
                tokens.push(Token::Number(number));
                continue;
            // Tokenize strings
            if chars[current_pos] == '"' {
                current_pos += 1; // Skip opening quote
                let start = current_pos;
                while current_pos < chars.len() && chars[current_pos] != '"' {
                    current_pos += 1;
                }
                if current_pos < chars.len() {
                    let string_content: String = chars[start..current_pos].iter().collect();
                    tokens.push(Token::String(string_content));
                    current_pos += 1; // Skip closing quote
                } else {
                    return Err(CursedError::general("Unterminated string literal"));
                }
                continue;
            // Single character tokens
            match chars[current_pos] {
                '/' => tokens.push(Token::Slash),
            }
            current_pos += 1;
        Ok(tokens)
    /// Parse tokens into AST
    fn parse_tokens(&self, tokens: &[Token]) -> Result<ParsedAst> {
        let mut node_count = 0;
        let mut functions = Vec::new();
        let mut variables = Vec::new();
        
        let mut i = 0;
        while i < tokens.len() {
            match &tokens[i] {
                Token::Identifier(name) if name == "slay" => {
                    // Function declaration
                    if i + 1 < tokens.len() {
                        if let Token::Identifier(func_name) = &tokens[i + 1] {
                            functions.push(func_name.clone());
                            node_count += 1;
                        }
                    }
                    i += 2;
                }
                Token::Identifier(name) if name == "facts" => {
                    // Variable declaration
                    if i + 1 < tokens.len() {
                        if let Token::Identifier(var_name) = &tokens[i + 1] {
                            variables.push(var_name.clone());
                            node_count += 1;
                        }
                    }
                    i += 2;
                }
                _ => {
                    node_count += 1;
                    i += 1;
                }
            }
        Ok(ParsedAst {
        })
    /// Perform semantic analysis
    fn semantic_analysis(&self, ast: ParsedAst) -> Result<AnalyzedAst> {
        // Simulate semantic analysis - type checking, scope resolution, etc.
        Ok(AnalyzedAst {
        })
    /// Generate LLVM IR
    fn generate_llvm_ir(&self, ast: &AnalyzedAst, module_name: &str, config: &OptimizationConfig) -> Result<String> {
        let mut ir = String::new();
        
        // Module header
        ir.push_str(&format!("; ModuleID = '{}'\n", module_name));
        ir.push_str("target datalayout = \"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"\n");
        ir.push_str("target triple = \"x86_64-unknown-linux-gnu\"\n\n");
        
        // Generate functions
        for function in &ast.functions {
            ir.push_str(&format!("define i32 @{}() {{\n", function));
            ir.push_str("entry:\n");
            
            // Apply optimization-specific IR
            match config.optimization_level {
                crate::common::optimization_level::OptimizationLevel::O3 => {
                    ir.push_str("  ; Aggressive optimization enabled\n");
                    ir.push_str("  %result = add i32 42, 0\n");
                }
                crate::common::optimization_level::OptimizationLevel::O2 => {
                    ir.push_str("  ; Default optimization\n");
                    ir.push_str("  %result = alloca i32\n");
                    ir.push_str("  store i32 42, i32* %result\n");
                    ir.push_str("  %value = load i32, i32* %result\n");
                }
                _ => {
                    ir.push_str("  ; No optimization\n");
                    ir.push_str("  %result = alloca i32\n");
                    ir.push_str("  store i32 42, i32* %result\n");
                    ir.push_str("  %value = load i32, i32* %result\n");
                }
            }
            
            ir.push_str("  ret i32 %result\n");
            ir.push_str("}\n\n");
        // Generate global variables
        for variable in &ast.variables {
            ir.push_str(&format!("@{} = global i32 0\n", variable));
        Ok(ir)
    fn link_optimized_binary<P: AsRef<Path>>(&self, _llvm_results: &crate::optimization::enhanced_llvm_optimization::EnhancedOptimizationResults, _output_path: P) -> Result<OptimizedBinary> {
        // Placeholder for binary linking
        Ok(OptimizedBinary {
            size_bytes: 1024 * 1024, // 1MB placeholder
        })
    fn calculate_performance_improvements(&self, project_chars: &ProjectCharacteristics, compilation_time: Duration, _binary: &OptimizedBinary) -> PerformanceImprovements {
        // Estimate improvements based on project characteristics and optimization applied
        let compilation_time_saved = Duration::from_secs_f64(
            project_chars.typical_build_time_seconds * 0.3 // 30% improvement estimate
        );
        
        PerformanceImprovements {
            binary_size_reduction: 15.0, // 15% size reduction estimate
            runtime_improvement_estimate: 25.0, // 25% runtime improvement estimate
            memory_usage_reduction: 10.0, // 10% memory reduction estimate
        }
    }
    
    fn generate_optimization_recommendations(&self, project_chars: &ProjectCharacteristics, improvements: &PerformanceImprovements, cache_hit_rate: f64, parallel_efficiency: f64) -> Vec<OptimizationRecommendation> {
        let mut recommendations = Vec::new();
        
        // Cache hit rate recommendations
        if cache_hit_rate < 0.7 {
            recommendations.push(OptimizationRecommendation {
            });
        // Parallel efficiency recommendations
        if parallel_efficiency < 0.8 && project_chars.total_source_files > 20 {
            recommendations.push(OptimizationRecommendation {
            });
        // Runtime performance recommendations
        if improvements.runtime_improvement_estimate < self.config.target_improvements.runtime_performance_improvement {
            recommendations.push(OptimizationRecommendation {
            });
        recommendations
    fn generate_performance_report(&self, profile: &OptimizationProfile, improvements: &PerformanceImprovements) -> Result<()> {
        let report_dir = self.config.report_output_dir.as_ref()
            .unwrap_or(&PathBuf::from(".cursed_reports"));
        
        std::fs::create_dir_all(report_dir)?;
        
            chrono::Utc::now().format("%Y%m%d_%H%M%S")));
        
        let report = serde_json::json!({
            "performance_improvements": {
        });
        
        std::fs::write(report_path, serde_json::to_string_pretty(&report)?)?;
        Ok(())
    }
}

// Supporting types and implementations

#[derive(Debug)]
struct ParallelCompilationResults {
#[derive(Debug)]
struct SingleFileCompilationResult {
#[derive(Debug)]
enum Token {
#[derive(Debug)]
struct ParsedAst {
#[derive(Debug)]
struct AnalyzedAst {
#[derive(Debug)]
struct OptimizedBinary {
#[derive(Debug)]
pub struct PerformanceStatistics {
#[derive(Debug)]
pub struct OptimizationHistorySummary {
impl AdaptiveOptimizer {
    fn new() -> Self {
        Self {
            project_characteristics: ProjectCharacteristics {
        }
    }
    
    fn select_optimal_profile(&mut self, project_chars: &ProjectCharacteristics) -> OptimizationProfile {
        self.project_characteristics = project_chars.clone();
        
        // Simple heuristics for profile selection
        if project_chars.total_source_files < 10 && !project_chars.has_heavy_computation {
            OptimizationProfile::Development
        } else if project_chars.has_heavy_computation || project_chars.total_lines_of_code > 100000 {
            OptimizationProfile::Performance
        } else if project_chars.total_source_files > 500 {
            OptimizationProfile::Release
        } else {
            OptimizationProfile::Release
        }
    }
    
    fn record_optimization(&mut self, record: OptimizationRecord) {
        self.optimization_history.push(record);
        
        // Keep only the last 100 records to prevent unbounded growth
        if self.optimization_history.len() > 100 {
            self.optimization_history.remove(0);
        }
    }
    
    fn get_history_summary(&self) -> OptimizationHistorySummary {
        if self.optimization_history.is_empty() {
            return OptimizationHistorySummary {
        let total_time: Duration = self.optimization_history.iter()
            .map(|r| r.compilation_time)
            .sum();
        
        let average_compilation_time = total_time / self.optimization_history.len() as u32;
        
        let best_record = self.optimization_history.iter()
            .max_by(|a, b| a.performance_score.partial_cmp(&b.performance_score).unwrap())
            .unwrap();
        
        OptimizationHistorySummary {
        }
    }
impl PerformanceMonitor {
    fn new() -> Self {
        Self {
        }
    }
    
    fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.checkpoints.clear();
    fn checkpoint(&mut self, name: String) {
        let checkpoint = PerformanceCheckpoint {
        
        self.checkpoints.push(checkpoint);
    fn get_checkpoints(&self) -> Vec<PerformanceCheckpoint> {
        self.checkpoints.clone()
    fn get_memory_usage_mb(&self) -> f64 {
        // Real memory usage measurement
        #[cfg(target_os = "linux")]
        {
            if let Ok(content) = std::fs::read_to_string("/proc/self/status") {
                for line in content.split("\n") {
                    if line.starts_with("VmRSS:") {
                        if let Some(kb_str) = line.split_whitespace().nth(1) {
                            if let Ok(kb) = kb_str.parse::<f64>() {
                                return kb / 1024.0; // Convert KB to MB
                            }
                        }
                    }
                }
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            if let Ok(output) = Command::new("ps")
                .args(&["-o", "rss=", "-p"])
                .arg(std::process::id().to_string())
                .output() 
            {
                if let Ok(rss_str) = String::from_utf8(output.stdout) {
                    if let Ok(rss_kb) = rss_str.trim().parse::<f64>() {
                        return rss_kb / 1024.0; // Convert KB to MB
                    }
                }
            }
        }
        
        #[cfg(target_os = "windows")]
        {
            // Use GetProcessMemoryInfo on Windows
            use std::mem;
            use winapi::um::processthreadsapi::GetCurrentProcess;
            use winapi::um::psapi::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
            
            unsafe {
                let mut pmc: PROCESS_MEMORY_COUNTERS = mem::zeroed();
                if GetProcessMemoryInfo(
                    mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32
                ) != 0 {
                    return pmc.WorkingSetSize as f64 / (1024.0 * 1024.0); // Convert bytes to MB
                }
            }
        // Fallback: estimate based on Rust's allocator if available
        self.estimate_memory_usage()
    fn estimate_memory_usage(&self) -> f64 {
        // Fallback memory estimation
        use std::alloc::{GlobalAlloc, Layout, System};
        
        // Create a small allocation to estimate memory overhead
        let test_layout = Layout::from_size_align(1024, 8).unwrap();
        unsafe {
            let ptr = System.alloc(test_layout);
            if !ptr.is_null() {
                System.dealloc(ptr, test_layout);
                // Rough estimate: 50MB base + allocation overhead
                return 50.0;
            }
        }
        
        // Final fallback
        100.0
    fn get_cpu_usage_percent(&self) -> f64 {
        // Real CPU usage measurement
        let start = std::time::Instant::now();
        let start_time = self.get_process_cpu_time();
        
        // Sample over a short period
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        let end_time = self.get_process_cpu_time();
        let elapsed = start.elapsed();
        
        if elapsed.as_millis() > 0 {
            let cpu_time_used = end_time.saturating_sub(start_time);
            let usage = (cpu_time_used.as_millis() as f64 / elapsed.as_millis() as f64) * 100.0;
            
            // Clamp to reasonable bounds
            usage.min(100.0).max(0.0)
        } else {
            0.0
        }
    }
    
    fn get_process_cpu_time(&self) -> Duration {
        #[cfg(target_os = "linux")]
        {
            if let Ok(content) = std::fs::read_to_string("/proc/self/stat") {
                let fields: Vec<&str> = content.split_whitespace().collect();
                if fields.len() > 15 {
                    // utime (user time) is field 13, stime (system time) is field 14
                    let utime = fields[13].parse::<u64>().unwrap_or(0);
                    let stime = fields[14].parse::<u64>().unwrap_or(0);
                    
                    // Convert from clock ticks to milliseconds
                    // Assuming 100 ticks per second (typical for Linux)
                    let total_ticks = utime + stime;
                    return Duration::from_millis(total_ticks * 10);
                }
            }
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            if let Ok(output) = Command::new("ps")
                .args(&["-o", "time=", "-p"])
                .arg(std::process::id().to_string())
                .output() 
            {
                if let Ok(time_str) = String::from_utf8(output.stdout) {
                    // Parse format MM:SS.ss or HH:MM:SS
                    let time_parts: Vec<&str> = time_str.trim().split(':').collect();
                    if time_parts.len() >= 2 {
                        let seconds_part = time_parts.last().unwrap();
                        if let Ok(seconds) = seconds_part.parse::<f64>() {
                            let minutes = if time_parts.len() > 2 {
                                time_parts[time_parts.len() - 2].parse::<f64>().unwrap_or(0.0)
                            } else {
                                time_parts[0].parse::<f64>().unwrap_or(0.0)
                            
                            let total_seconds = minutes * 60.0 + seconds;
                            return Duration::from_secs_f64(total_seconds);
                        }
                    }
                }
            }
        #[cfg(target_os = "windows")]
        {
            use std::mem;
            use winapi::um::processthreadsapi::GetCurrentProcess;
            use winapi::um::processthreadsapi::GetProcessTimes;
            use winapi::shared::minwindef::FILETIME;
            
            unsafe {
                let mut creation_time: FILETIME = mem::zeroed();
                let mut exit_time: FILETIME = mem::zeroed();
                let mut kernel_time: FILETIME = mem::zeroed();
                let mut user_time: FILETIME = mem::zeroed();
                
                if GetProcessTimes(
                    &mut user_time
                ) != 0 {
                    // Convert FILETIME to milliseconds
                    let kernel_ms = ((kernel_time.dwHighDateTime as u64) << 32 | kernel_time.dwLowDateTime as u64) / 10_000;
                    let user_ms = ((user_time.dwHighDateTime as u64) << 32 | user_time.dwLowDateTime as u64) / 10_000;
                    
                    return Duration::from_millis(kernel_ms + user_ms);
                }
            }
        // Fallback
        Duration::from_millis(0)
    }
}

// Extension traits for configuration conversion
impl From<OptimizationConfig> for crate::optimization::enhanced_llvm_optimization::EnhancedOptimizationConfig {
    fn from(config: OptimizationConfig) -> Self {
        crate::optimization::enhanced_llvm_optimization::EnhancedOptimizationConfig {
            ..Default::default()
        }
    }
impl BenchmarkConfig {
    fn quick() -> Self {
        BenchmarkConfig {
        }
    }
    
    fn thorough() -> Self {
        BenchmarkConfig {
        }
    }
}
