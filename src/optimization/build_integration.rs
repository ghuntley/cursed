//! Build system integration for CURSED optimization with Performance Integration

use crate::error::{Result, CursedError};
use crate::optimization::{
    coordinator::{OptimizationCoordinator, OptimizationCoordinatorConfig, OptimizationCoordinatorResult},
    metrics::CompilationUnit,
    performance_integration::{
        PerformanceIntegrationSystem, PerformanceIntegrationConfig, 
        IntegratedOptimizationResults, PerformanceTargets,
    },
    config::{OptimizationConfig, OptimizationProfile, OptimizationLevel},
};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tracing::{info, debug, warn, error, instrument};
use serde::{Deserialize, Serialize};

/// Build context information
#[derive(Debug, Clone)]
pub struct BuildContext {
    pub project_root: PathBuf,
    pub source_files: Vec<PathBuf>,
    pub output_directory: PathBuf,
    pub target_triple: String,
    pub debug_mode: bool,
    pub release_mode: bool,
    pub verbose: bool,
}

/// Build optimization result with enhanced performance metrics
#[derive(Debug, Clone)]
pub struct BuildOptimizationResult {
    pub success: bool,
    pub compilation_time: Duration,
    pub optimization_time: Duration,
    pub total_time: Duration,
    pub files_compiled: usize,
    pub files_cached: usize,
    pub cache_hit_rate: f64,
    pub parallel_efficiency: f64,
    pub size_reduction_bytes: i64,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
    pub performance_summary: String,
    // Enhanced performance integration results
    pub optimization_profile_used: Option<OptimizationProfile>,
    pub performance_improvements: Option<crate::optimization::performance_integration::PerformanceImprovements>,
    pub optimization_recommendations: Vec<crate::optimization::performance_integration::OptimizationRecommendation>,
    pub adaptive_optimization_enabled: bool,
}

/// Enhanced build system integration for CURSED with performance optimization
pub struct BuildOptimizer {
    coordinator: OptimizationCoordinator,
    performance_system: Option<PerformanceIntegrationSystem>,
    context: BuildContext,
    compilation_cache: HashMap<PathBuf, CompilationUnit>,
    enable_performance_integration: bool,
}

impl BuildOptimizer {
    /// Create a new build optimizer
    #[instrument(skip(context))]
    pub fn new(context: BuildContext) -> Result<Self> {
        Self::new_with_performance_integration(context, true)
    }
    
    /// Create a new build optimizer with optional performance integration
    #[instrument(skip(context))]
    pub fn new_with_performance_integration(context: BuildContext, enable_performance: bool) -> Result<Self> {
        info!("Creating build optimizer for project: {:?}", context.project_root);
        
        // Create optimization configuration based on build context
        let config = Self::create_optimization_config(&context)?;
        
        // Create coordinator
        let coordinator = OptimizationCoordinator::new(config)?;
        
        // Create performance integration system if enabled
        let performance_system = if enable_performance {
            let perf_config = Self::create_performance_config(&context)?;
            let opt_config = Self::create_modern_optimization_config(&context)?;
            Some(PerformanceIntegrationSystem::new(perf_config, opt_config)?)
        } else {
            None
        };
        
        Ok(Self {
            coordinator,
            performance_system,
            context,
            compilation_cache: HashMap::new(),
            enable_performance_integration: enable_performance,
        })
    }
    
    /// Create optimization configuration from build context
    fn create_optimization_config(context: &BuildContext) -> Result<OptimizationCoordinatorConfig> {
        let mut config = OptimizationCoordinatorConfig::default();
        
        // Adjust settings based on build mode
        if context.debug_mode {
            // Debug mode: prioritize compilation speed
            config.llvm_config.optimization_level = "O0".to_string();
            config.enable_parallel = true;
            config.enable_incremental = true;
            config.enable_caching = true;
            config.parallel_config.max_parallel_jobs = Some(num_cpus::get());
        } else if context.release_mode {
            // Release mode: prioritize performance
            config.llvm_config.optimization_level = "O2".to_string();
            config.enable_parallel = true;
            config.enable_incremental = false; // Full optimization
            config.enable_caching = true;
            config.llvm_config.enable_vectorization = true;
            config.llvm_config.enable_loop_unrolling = true;
            config.llvm_config.enable_function_inlining = true;
        }
        
        // Enable profiling in verbose mode
        config.enable_profiling = context.verbose;
        config.enable_analysis = true;
        
        // Set cache directory
        config.cache_config.cache_directory = context.project_root.join(".cursed_cache");
        
        // Set parallel compilation based on project size
        let source_count = context.source_files.len();
        if source_count > 20 {
            config.parallel_config.max_parallel_jobs = Some(num_cpus::get());
        } else if source_count > 5 {
            config.parallel_config.max_parallel_jobs = Some((num_cpus::get() / 2).max(1));
        } else {
            config.enable_parallel = false;
        }
        
        Ok(config)
    }
    
    /// Create performance integration configuration from build context
    fn create_performance_config(context: &BuildContext) -> Result<PerformanceIntegrationConfig> {
        let mut config = PerformanceIntegrationConfig::default();
        
        // Adjust based on build mode
        if context.debug_mode {
            config.enable_adaptive_optimization = true;
            config.enable_performance_monitoring = context.verbose;
            config.enable_automatic_reporting = false;
            config.target_improvements = PerformanceTargets {
                compilation_time_reduction: 50.0, // Prioritize fast compilation
                runtime_performance_improvement: 10.0,
                memory_usage_reduction: 10.0,
                binary_size_reduction: 5.0,
            };
        } else if context.release_mode {
            config.enable_adaptive_optimization = true;
            config.enable_performance_monitoring = true;
            config.enable_automatic_reporting = context.verbose;
            config.enable_pgo = true;
            config.target_improvements = PerformanceTargets {
                compilation_time_reduction: 20.0,
                runtime_performance_improvement: 40.0, // Prioritize runtime performance
                memory_usage_reduction: 25.0,
                binary_size_reduction: 20.0,
            };
        }
        
        // Set output directories
        if context.verbose {
            config.report_output_dir = Some(context.project_root.join(".cursed_reports"));
        }
        
        // Adjust based on project size
        let source_count = context.source_files.len();
        if source_count > 100 {
            config.max_parallel_workers = num_cpus::get();
            config.cache_size_limit_mb = 4096; // 4GB for large projects
            config.optimization_threshold_seconds = 60.0;
        } else if source_count > 20 {
            config.max_parallel_workers = num_cpus::get() / 2;
            config.cache_size_limit_mb = 2048; // 2GB for medium projects
            config.optimization_threshold_seconds = 30.0;
        } else {
            config.max_parallel_workers = 2;
            config.cache_size_limit_mb = 1024; // 1GB for small projects
            config.optimization_threshold_seconds = 15.0;
        }
        
        Ok(config)
    }
    
    /// Create modern optimization configuration for performance integration
    fn create_modern_optimization_config(context: &BuildContext) -> Result<OptimizationConfig> {
        let mut config = OptimizationConfig::default();
        
        // Set optimization level based on build mode
        if context.debug_mode {
            config.optimization_level = OptimizationLevel::Less;
            config.debug_mode = true;
        } else if context.release_mode {
            config.optimization_level = OptimizationLevel::Aggressive;
            config.debug_mode = false;
            config.profile_guided = true;
        } else {
            config.optimization_level = OptimizationLevel::Default;
        }
        
        // Configure parallel compilation
        let source_count = context.source_files.len();
        config.enable_parallel = source_count > 5;
        config.parallel_workers = if source_count > 50 {
            num_cpus::get()
        } else if source_count > 10 {
            (num_cpus::get() / 2).max(1)
        } else {
            2
        };
        
        // Configure incremental compilation
        config.enable_incremental = true;
        config.dependency_tracking = true;
        config.cache_directory = Some(context.project_root.join(".cursed_cache"));
        
        // Configure profiling and reporting
        config.enable_profiling = context.verbose;
        config.generate_reports = context.verbose;
        config.verbose_optimization = context.verbose;
        
        if context.verbose {
            config.profile_output_dir = Some(context.project_root.join(".cursed_profiles"));
            config.report_output_dir = Some(context.project_root.join(".cursed_reports"));
        }
        
        // Set target-specific optimizations
        config.target_cpu = Some("native".to_string());
        
        Ok(config)
    }
    
    /// Optimize a complete build with optional performance integration
    #[instrument(skip(self))]
    pub fn optimize_build(&mut self) -> Result<BuildOptimizationResult> {
        let start_time = Instant::now();
        info!("Starting optimized build for {} source files", self.context.source_files.len());
        
        // Use performance integration system if available
        if let Some(ref mut performance_system) = self.performance_system {
            self.optimize_build_with_performance_integration(performance_system, start_time)
        } else {
            self.optimize_build_legacy(start_time)
        }
    }
    
    /// Optimize build using the enhanced performance integration system
    #[instrument(skip(self, performance_system))]
    fn optimize_build_with_performance_integration(
        &mut self,
        performance_system: &mut PerformanceIntegrationSystem,
        start_time: Instant,
    ) -> Result<BuildOptimizationResult> {
        info!("Using performance integration system for optimization");
        
        // Run integrated optimization
        let output_path = self.context.output_directory.join("optimized_output");
        let integration_result = performance_system.optimize_project(&self.context.source_files, &output_path)?;
        
        // Generate build artifacts using traditional method as fallback
        let mut compilation_units = self.discover_compilation_units()?;
        let artifact_result = self.generate_build_artifacts(&compilation_units)?;
        
        let total_time = start_time.elapsed();
        
        // Collect warnings and errors
        let mut warnings = Vec::new();
        let mut errors = Vec::new();
        warnings.extend(artifact_result.warnings);
        errors.extend(artifact_result.errors);
        
        // Enhanced performance summary
        let performance_summary = format!(
            "Performance Integration: {} compilation, {:.1}% parallel efficiency, {:.1}% cache hit rate. {:.1}% runtime improvement estimated.",
            format_duration(integration_result.compilation_time),
            integration_result.parallel_efficiency * 100.0,
            integration_result.cache_hit_rate * 100.0,
            integration_result.performance_improvements.runtime_improvement_estimate
        );
        
        let result = BuildOptimizationResult {
            success: artifact_result.success,
            compilation_time: integration_result.compilation_time,
            optimization_time: total_time - integration_result.compilation_time,
            total_time,
            files_compiled: self.context.source_files.len(),
            files_cached: ((integration_result.cache_hit_rate * self.context.source_files.len() as f64) as usize),
            cache_hit_rate: integration_result.cache_hit_rate,
            parallel_efficiency: integration_result.parallel_efficiency,
            size_reduction_bytes: (integration_result.performance_improvements.binary_size_reduction * 1024.0) as i64,
            warnings,
            errors,
            performance_summary,
            // Enhanced fields
            optimization_profile_used: Some(integration_result.optimization_profile),
            performance_improvements: Some(integration_result.performance_improvements),
            optimization_recommendations: integration_result.recommendations,
            adaptive_optimization_enabled: self.enable_performance_integration,
        };
        
        info!(
            success = result.success,
            total_time = ?total_time,
            files_compiled = result.files_compiled,
            cache_hit_rate = result.cache_hit_rate,
            parallel_efficiency = result.parallel_efficiency,
            optimization_profile = ?result.optimization_profile_used,
            "Enhanced build optimization completed"
        );
        
        Ok(result)
    }
    
    /// Legacy optimization build method
    #[instrument(skip(self))]
    fn optimize_build_legacy(&mut self, start_time: Instant) -> Result<BuildOptimizationResult> {
        info!("Using legacy optimization system");
        
        // Phase 1: Discover and analyze compilation units
        debug!("Phase 1: Discovering compilation units");
        let mut compilation_units = self.discover_compilation_units()?;
        
        // Phase 2: Run optimization coordinator
        debug!("Phase 2: Running optimization coordinator");
        let optimization_start = Instant::now();
        let optimization_result = self.coordinator.optimize_compilation(&mut compilation_units)?;
        let optimization_time = optimization_start.elapsed();
        
        // Phase 3: Generate final build artifacts
        debug!("Phase 3: Generating build artifacts");
        let artifact_result = self.generate_build_artifacts(&compilation_units)?;
        
        let total_time = start_time.elapsed();
        let compilation_time = total_time - optimization_time;
        
        // Calculate statistics
        let cache_hit_rate = optimization_result.cache_hit_rate;
        let parallel_efficiency = optimization_result.parallel_efficiency;
        let size_reduction = self.calculate_size_reduction(&compilation_units);
        
        // Generate performance summary
        let performance_summary = self.generate_performance_summary(&optimization_result);
        
        // Collect warnings and errors
        let mut warnings = optimization_result.warnings;
        let mut errors = optimization_result.errors;
        warnings.extend(artifact_result.warnings);
        errors.extend(artifact_result.errors);
        
        let result = BuildOptimizationResult {
            success: optimization_result.compilation_successful && artifact_result.success,
            compilation_time,
            optimization_time,
            total_time,
            files_compiled: optimization_result.units_compiled,
            files_cached: optimization_result.units_from_cache,
            cache_hit_rate,
            parallel_efficiency,
            size_reduction_bytes: size_reduction,
            warnings,
            errors,
            performance_summary,
            // Enhanced fields (empty for legacy)
            optimization_profile_used: None,
            performance_improvements: None,
            optimization_recommendations: Vec::new(),
            adaptive_optimization_enabled: false,
        };
        
        info!(
            success = result.success,
            total_time = ?total_time,
            files_compiled = result.files_compiled,
            cache_hit_rate = result.cache_hit_rate,
            "Legacy build optimization completed"
        );
        
        Ok(result)
    }
    
    /// Discover compilation units from source files
    fn discover_compilation_units(&mut self) -> Result<Vec<CompilationUnit>> {
        let mut units = Vec::new();
        
        for source_file in &self.context.source_files {
            if !source_file.exists() {
                warn!("Source file does not exist: {:?}", source_file);
                continue;
            }
            
            // Check cache first
            if let Some(cached_unit) = self.compilation_cache.get(source_file) {
                units.push(cached_unit.clone());
                continue;
            }
            
            // Create compilation unit
            let mut unit = CompilationUnit::new(
                source_file.file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
            );
            
            unit.add_source_file(source_file.to_string_lossy().to_string());
            
            // Analyze dependencies
            let dependencies = self.analyze_file_dependencies(source_file)?;
            for dep in dependencies {
                unit.add_dependency(dep);
            }
            
            // Estimate size
            if let Ok(metadata) = std::fs::metadata(source_file) {
                unit.estimated_size_bytes = metadata.len() as usize;
            }
            
            // Cache the unit
            self.compilation_cache.insert(source_file.clone(), unit.clone());
            units.push(unit);
        }
        
        debug!("Discovered {} compilation units", units.len());
        Ok(units)
    }
    
    /// Analyze dependencies for a source file
    fn analyze_file_dependencies(&self, source_file: &Path) -> Result<Vec<String>> {
        let content = std::fs::read_to_string(source_file).map_err(|e| {
            CursedError::optimization_error(&format!("Failed to read source file {:?}: {}", source_file, e))
        })?;
        
        let mut dependencies = Vec::new();
        
        // Simple import analysis for CURSED files
        for line in content.lines() {
            let trimmed = line.trim();
            
            // Look for import statements
            if trimmed.starts_with("import") {
                if let Some(module_name) = self.extract_import_module(trimmed) {
                    dependencies.push(module_name);
                }
            }
            
            // Look for use statements
            if trimmed.starts_with("use") {
                if let Some(module_name) = self.extract_use_module(trimmed) {
                    dependencies.push(module_name);
                }
            }
        }
        
        Ok(dependencies)
    }
    
    /// Extract module name from import statement
    fn extract_import_module(&self, line: &str) -> Option<String> {
        // import "module::path" or import module::path
        if let Some(start) = line.find('"') {
            if let Some(end) = line.rfind('"') {
                if start < end {
                    let module_path = &line[start + 1..end];
                    return Some(module_path.replace("::", "_"));
                }
            }
        }
        None
    }
    
    /// Extract module name from use statement  
    fn extract_use_module(&self, line: &str) -> Option<String> {
        // use module::path;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let module_path = parts[1].trim_end_matches(';');
            return Some(module_path.replace("::", "_"));
        }
        None
    }
    
    /// Generate build artifacts
    fn generate_build_artifacts(&self, units: &[CompilationUnit]) -> Result<ArtifactResult> {
        debug!("Generating build artifacts for {} units", units.len());
        
        let mut warnings = Vec::new();
        let mut errors = Vec::new();
        let mut success = true;
        
        // Create output directory if it doesn't exist
        if !self.context.output_directory.exists() {
            std::fs::create_dir_all(&self.context.output_directory).map_err(|e| {
                CursedError::optimization_error(&format!("Failed to create output directory: {}", e))
            })?;
        }
        
        // Generate artifacts for each unit
        for unit in units {
            match self.generate_unit_artifact(unit) {
                Ok(artifact_warnings) => {
                    warnings.extend(artifact_warnings);
                }
                Err(e) => {
                    errors.push(format!("Failed to generate artifact for {}: {}", unit.name, e));
                    success = false;
                }
            }
        }
        
        // Generate final executable or library
        if success {
            match self.link_final_artifact(units) {
                Ok(link_warnings) => {
                    warnings.extend(link_warnings);
                }
                Err(e) => {
                    errors.push(format!("Failed to link final artifact: {}", e));
                    success = false;
                }
            }
        }
        
        Ok(ArtifactResult {
            success,
            warnings,
            errors,
        })
    }
    
    /// Generate artifact for a single compilation unit
    fn generate_unit_artifact(&self, unit: &CompilationUnit) -> Result<Vec<String>> {
        let mut warnings = Vec::new();
        
        // Generate object file name
        let object_name = format!("{}.o", unit.name);
        let object_path = self.context.output_directory.join(object_name);
        
        // Simulate artifact generation
        debug!("Generating artifact for unit: {}", unit.name);
        
        // Create a placeholder object file
        std::fs::write(&object_path, format!("// Object file for {}\n", unit.name)).map_err(|e| {
            CursedError::optimization_error(&format!("Failed to write object file: {}", e))
        })?;
        
        // Check for potential issues
        if unit.estimated_size_bytes > 1_000_000 {
            warnings.push(format!("Unit {} is very large ({} bytes)", unit.name, unit.estimated_size_bytes));
        }
        
        Ok(warnings)
    }
    
    /// Link final artifact
    fn link_final_artifact(&self, units: &[CompilationUnit]) -> Result<Vec<String>> {
        let mut warnings = Vec::new();
        
        // Determine output name
        let output_name = if self.context.debug_mode {
            "debug_output"
        } else {
            "release_output"
        };
        
        let output_path = self.context.output_directory.join(output_name);
        
        debug!("Linking final artifact: {:?}", output_path);
        
        // Simulate linking
        let link_script = units.iter()
            .map(|unit| format!("link {}.o", unit.name))
            .collect::<Vec<_>>()
            .join("\n");
        
        std::fs::write(&output_path, format!("// Linked executable\n{}\n", link_script)).map_err(|e| {
            CursedError::optimization_error(&format!("Failed to write final artifact: {}", e))
        })?;
        
        if units.len() > 100 {
            warnings.push("Large number of compilation units may impact link time".to_string());
        }
        
        Ok(warnings)
    }
    
    /// Calculate size reduction from optimizations
    fn calculate_size_reduction(&self, units: &[CompilationUnit]) -> i64 {
        // Simulate size reduction calculation
        let original_size: usize = units.iter().map(|u| u.estimated_size_bytes).sum();
        let optimized_size = (original_size as f64 * 0.85) as usize; // Assume 15% reduction
        (original_size - optimized_size) as i64
    }
    
    /// Generate performance summary
    fn generate_performance_summary(&self, result: &OptimizationCoordinatorResult) -> String {
        format!(
            "Optimization Summary: {} units compiled in {:.2?} with {:.1}% parallel efficiency. Cache hit rate: {:.1}%.",
            result.units_compiled,
            result.total_time,
            result.parallel_efficiency * 100.0,
            result.cache_hit_rate * 100.0
        )
    }
    
    /// Clean build cache and temporary files
    pub fn clean(&mut self) -> Result<()> {
        info!("Cleaning build cache and temporary files");
        
        // Clear compilation cache
        self.compilation_cache.clear();
        
        // Remove cache directory
        let cache_dir = self.context.project_root.join(".cursed_cache");
        if cache_dir.exists() {
            std::fs::remove_dir_all(&cache_dir).map_err(|e| {
                CursedError::optimization_error(&format!("Failed to remove cache directory: {}", e))
            })?;
        }
        
        // Remove output directory
        if self.context.output_directory.exists() {
            std::fs::remove_dir_all(&self.context.output_directory).map_err(|e| {
                CursedError::optimization_error(&format!("Failed to remove output directory: {}", e))
            })?;
        }
        
        info!("Build cache and temporary files cleaned");
        Ok(())
    }
    
    /// Get optimization statistics
    pub fn get_statistics(&self) -> OptimizationStatistics {
        let coordinator_stats = self.coordinator.get_statistics();
        
        OptimizationStatistics {
            total_compilations: coordinator_stats.total_compilations,
            successful_compilations: coordinator_stats.successful_compilations,
            average_compilation_time: coordinator_stats.average_compilation_time,
            cache_enabled: coordinator_stats.cache_enabled,
            incremental_enabled: coordinator_stats.incremental_enabled,
            parallel_enabled: coordinator_stats.parallel_enabled,
            cached_units: self.compilation_cache.len(),
        }
    }
}

/// Result from artifact generation
#[derive(Debug, Clone)]
struct ArtifactResult {
    success: bool,
    warnings: Vec<String>,
    errors: Vec<String>,
}

/// Build optimization statistics
#[derive(Debug, Clone)]
pub struct OptimizationStatistics {
    pub total_compilations: usize,
    pub successful_compilations: usize,
    pub average_compilation_time: Duration,
    pub cache_enabled: bool,
    pub incremental_enabled: bool,
    pub parallel_enabled: bool,
    pub cached_units: usize,
}

/// Create a build optimizer from CLI arguments
pub fn create_build_optimizer_from_args(
    project_root: PathBuf,
    source_files: Vec<PathBuf>,
    output_dir: Option<PathBuf>,
    target: Option<String>,
    debug: bool,
    release: bool,
    verbose: bool,
) -> Result<BuildOptimizer> {
    create_build_optimizer_from_args_with_performance(
        project_root, source_files, output_dir, target, debug, release, verbose, true
    )
}

/// Create a build optimizer from CLI arguments with performance integration option
pub fn create_build_optimizer_from_args_with_performance(
    project_root: PathBuf,
    source_files: Vec<PathBuf>,
    output_dir: Option<PathBuf>,
    target: Option<String>,
    debug: bool,
    release: bool,
    verbose: bool,
    enable_performance: bool,
) -> Result<BuildOptimizer> {
    let context = BuildContext {
        project_root: project_root.clone(),
        source_files,
        output_directory: output_dir.unwrap_or_else(|| project_root.join("target")),
        target_triple: target.unwrap_or_else(|| "native".to_string()),
        debug_mode: debug,
        release_mode: release,
        verbose,
    };
    
    BuildOptimizer::new_with_performance_integration(context, enable_performance)
}

/// Format duration for display
fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();
    
    if secs > 0 {
        format!("{}.{:03}s", secs, millis)
    } else {
        format!("{}ms", millis)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_build_optimizer_creation() {
        let temp_dir = env::temp_dir();
        let context = BuildContext {
            project_root: temp_dir.clone(),
            source_files: vec![],
            output_directory: temp_dir.join("output"),
            target_triple: "native".to_string(),
            debug_mode: true,
            release_mode: false,
            verbose: false,
        };
        
        let optimizer = BuildOptimizer::new(context);
        assert!(optimizer.is_ok());
    }

    #[test]
    fn test_optimization_config_creation() {
        let context = BuildContext {
            project_root: PathBuf::from("/tmp"),
            source_files: vec![],
            output_directory: PathBuf::from("/tmp/output"),
            target_triple: "native".to_string(),
            debug_mode: true,
            release_mode: false,
            verbose: true,
        };
        
        let config = BuildOptimizer::create_optimization_config(&context);
        assert!(config.is_ok());
        
        let config = config.unwrap();
        assert_eq!(config.llvm_config.optimization_level, "O0");
        assert!(config.enable_profiling);
    }
}
