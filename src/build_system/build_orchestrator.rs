//! Build Orchestrator
//! 
//! Coordinates the build process, manages dependencies, integrates with toolchain,
//! and provides build caching and incremental compilation.

use crate::build_system::{
    BuildConfig, BuildTarget, BuildProfile, IncrementalCache, DependencyResolver,
    TargetType, OptimizationLevel, TestDiscovery, TestDiscoveryConfig, TestExecutor, 
    TestExecutionConfig, TestExecutionResult, TestFilter, TestCategory,
    ParallelCompiler, ParallelCompilationConfig, IncrementalOptimizer, IncrementalConfig,
    BuildProfiler, ProfilerConfig, ArtifactManager, ArtifactConfig
};
use crate::build_system::build_pipeline::{BuildPipeline, PipelineContext, PipelineResult};
use crate::build_system::incremental_cache::CacheError;
use crate::package_manager::{PackageManager, PackageManagerError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use num_cpus;
use tracing::{debug, error, info, warn, instrument};
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Event, EventKind, Config, Result as NotifyResult};
use futures;
use sysinfo;
use regex;

/// File watcher configuration
#[derive(Debug, Clone)]
pub struct WatchConfig {
    /// File patterns to watch
    pub patterns: Vec<String>,
    /// Debounce delay to prevent rapid rebuilds
    pub debounce_ms: u64,
    /// Whether to run full build or incremental
    pub incremental: bool,
    /// Profile to use for rebuild
    pub build_profile: String,
}

impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            patterns: vec![
                "**/*.csd".to_string(),
                "**/Cargo.toml".to_string(),
                "**/CursedBuild.toml".to_string(),
                "**/CursedPackage.toml".to_string(),
            ],
            debounce_ms: 500,
            incremental: true,
            build_profile: "dev".to_string(),
        }
    }
}

/// File watcher state
pub struct FileWatcher {
    watcher: RecommendedWatcher,
    event_receiver: Option<Receiver<NotifyResult<Event>>>,
    watch_thread: Option<thread::JoinHandle<()>>,
    shutdown_sender: Option<Sender<()>>,
}

/// Main build orchestrator
pub struct BuildOrchestrator {
    config: BuildConfig,
    cache: IncrementalCache,
    dependency_resolver: DependencyResolver,
    package_manager: PackageManager,
    pipeline: BuildPipeline,
    work_dir: PathBuf,
    file_watcher: Option<FileWatcher>,
    watch_config: WatchConfig,
    // Advanced features
    parallel_compiler: Option<ParallelCompiler>,
    incremental_optimizer: Option<IncrementalOptimizer>,
    build_profiler: Option<BuildProfiler>,
    artifact_manager: Option<ArtifactManager>,
}

/// Build result information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildResult {
    /// Whether the build succeeded
    pub success: bool,
    
    /// Build duration
    pub duration: Duration,
    
    /// Targets that were built
    pub targets_built: Vec<String>,
    
    /// Targets that were skipped (cache hit)
    pub targets_skipped: Vec<String>,
    
    /// Output files generated
    pub outputs: Vec<PathBuf>,
    
    /// Build artifacts
    pub artifacts: HashMap<String, PathBuf>,
    
    /// Warnings generated during build
    pub warnings: Vec<String>,
    
    /// Build statistics
    pub statistics: BuildStatistics,
}

/// Build statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildStatistics {
    /// Number of files compiled
    pub files_compiled: usize,
    
    /// Number of files from cache
    pub files_cached: usize,
    
    /// Total lines of code compiled
    pub lines_compiled: usize,
    
    /// Peak memory usage (in bytes)
    pub peak_memory: usize,
    
    /// Compilation phases timing
    pub phase_timings: HashMap<String, Duration>,
}

/// Comprehensive build performance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildPerformanceReport {
    /// Total build duration
    pub total_duration: Duration,
    
    /// Build efficiency score (0.0 to 1.0)
    pub efficiency_score: f64,
    
    /// Performance improvement recommendations
    pub performance_recommendations: Vec<String>,
    
    /// Resource utilization analysis
    pub resource_analysis: ResourceUtilizationAnalysis,
    
    /// Breakdown of time spent in each phase
    pub phase_breakdown: HashMap<String, Duration>,
    
    /// Cache effectiveness metrics
    pub cache_effectiveness: CacheEffectivenessMetrics,
    
    /// Identified bottlenecks
    pub bottleneck_analysis: Vec<BuildBottleneck>,
}

/// Resource utilization analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUtilizationAnalysis {
    /// Peak memory usage in MB
    pub memory_peak_mb: usize,
    
    /// Memory efficiency score (0.0 to 1.0)
    pub memory_efficiency: f64,
    
    /// CPU-intensive build phases
    pub cpu_intensive_phases: Vec<String>,
    
    /// I/O-intensive build phases
    pub io_intensive_phases: Vec<String>,
    
    /// Identified parallelization opportunities
    pub parallelization_opportunities: Vec<String>,
    
    /// Total build time
    pub total_build_time: Duration,
}

/// Cache effectiveness metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEffectivenessMetrics {
    /// Cache hit rate (0.0 to 1.0)
    pub hit_rate: f64,
    
    /// Number of files retrieved from cache
    pub files_from_cache: usize,
    
    /// Total number of files processed
    pub total_files: usize,
    
    /// Estimated time saved by caching
    pub estimated_time_saved: Duration,
    
    /// Cache storage efficiency
    pub cache_storage_efficiency: f64,
}

/// Build bottleneck analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildBottleneck {
    /// Phase where bottleneck occurs
    pub phase: String,
    
    /// Duration of the bottleneck
    pub duration: Duration,
    
    /// Percentage of total build time
    pub percentage: f64,
    
    /// Severity level
    pub severity: BottleneckSeverity,
    
    /// Specific recommendations for this bottleneck
    pub recommendations: Vec<String>,
}

/// Bottleneck severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BottleneckSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Build error types
#[derive(Debug, thiserror::Error)]
pub enum BuildError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Dependency resolution failed: {0}")]
    DependencyError(String),
    
    #[error("Compilation failed: {0}")]
    CompilationError(String),
    
    #[error("Target not found: {0}")]
    TargetNotFound(String),
    
    #[error("Cache error: {0}")]
    CacheError(#[from] CacheError),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Package manager error: {0}")]
    PackageError(#[from] PackageManagerError),
    
    #[error("Tool integration error: {0}")]
    ToolError(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(#[from] crate::build_system::build_config::ConfigError),
    
    #[error("File watcher error: {0}")]
    WatcherError(#[from] notify::Error),
}

impl BuildOrchestrator {
    /// Create a new build orchestrator
    pub fn new(config: BuildConfig, work_dir: PathBuf) -> Result<Self, BuildError> {
        let cache = IncrementalCache::new(work_dir.join("target").join("cache"))?;
        let dependency_resolver = DependencyResolver::new();
        let package_manager_config = crate::package_manager::PackageManagerConfig {
            workspace_dir: work_dir.clone(),
            cache_dir: work_dir.join(".cursed-cache"),
            registry_url: "https://packages.cursed-lang.org".to_string(),
            max_cache_size: 1024 * 1024 * 1024, // 1GB
            timeout_seconds: 30,
            parallel_downloads: 4,
        };
        let package_manager = PackageManager::new(package_manager_config)?;
        let pipeline = BuildPipeline::new(config.clone(), work_dir.clone())?;
        
        Ok(BuildOrchestrator {
            config,
            cache,
            dependency_resolver,
            package_manager,
            pipeline,
            work_dir,
            file_watcher: None,
            watch_config: WatchConfig::default(),
            parallel_compiler: None,
            incremental_optimizer: None,
            build_profiler: None,
            artifact_manager: None,
        })
    }
    
    /// Build all targets
    #[instrument(skip(self))]
    pub async fn build_all(&mut self, profile: &str) -> Result<BuildResult, BuildError> {
        info!("Starting build for all targets with profile: {}", profile);
        
        let start_time = Instant::now();
        let mut result = BuildResult {
            success: true,
            duration: Duration::default(),
            targets_built: Vec::from([]),
            targets_skipped: Vec::from([]),
            outputs: Vec::from([]),
            artifacts: HashMap::new(),
            warnings: Vec::from([]),
            statistics: BuildStatistics {
                files_compiled: 0,
                files_cached: 0,
                lines_compiled: 0,
                peak_memory: 0,
                phase_timings: HashMap::new(),
            },
        };
        
        // Resolve dependencies first
        self.resolve_dependencies().await?;
        
        // Get effective build profile
        let build_profile = self.config.get_effective_profile(profile)
            .map_err(|e| BuildError::ConfigError(e.to_string()))?;
        
        // Build each target
        for target in &self.config.targets.clone() {
            match self.build_target(target, &build_profile).await {
                Ok(target_result) => {
                    result.targets_built.push(target.name.clone());
                    result.outputs.extend(target_result.outputs);
                    result.artifacts.extend(target_result.artifacts);
                    result.warnings.extend(target_result.warnings);
                    
                    // Merge statistics
                    result.statistics.files_compiled += target_result.statistics.files_compiled;
                    result.statistics.files_cached += target_result.statistics.files_cached;
                    result.statistics.lines_compiled += target_result.statistics.lines_compiled;
                    result.statistics.peak_memory = result.statistics.peak_memory.max(target_result.statistics.peak_memory);
                    
                    for (phase, duration) in target_result.statistics.phase_timings {
                        *result.statistics.phase_timings.entry(phase).or_insert(Duration::default()) += duration;
                    }
                }
                Err(e) => {
                    error!("Failed to build target '{}': {}", target.name, e);
                    result.success = false;
                    return Err(e);
                }
            }
        }
        
        result.duration = start_time.elapsed();
        info!("Build completed in {:?}", result.duration);
        
        Ok(result)
    }
    
    /// Build using comprehensive pipeline (enhanced method)
    #[instrument(skip(self))]
    pub async fn build_with_pipeline(&mut self, profile: &str, targets: Vec<String>, force_rebuild: bool, parallel: bool) -> Result<PipelineResult, BuildError> {
        info!("Starting pipeline build with profile: {}", profile);
        
        let context = PipelineContext {
            profile: profile.to_string(),
            targets,
            work_dir: self.work_dir.clone(),
            force_rebuild,
            parallel,
            verbose: true,
        };
        
        self.pipeline.execute(context).await
    }
    
    /// Build specific targets with pipeline
    #[instrument(skip(self))]
    pub async fn build_targets_with_pipeline(&mut self, profile: &str, target_names: &[String]) -> Result<PipelineResult, BuildError> {
        self.build_with_pipeline(profile, target_names.to_vec(), false, true).await
    }
    
    /// Quick build (skip formatting and linting)
    #[instrument(skip(self))]
    pub async fn quick_build(&mut self, profile: &str) -> Result<BuildResult, BuildError> {
        info!("Starting quick build (skipping formatting and linting)");
        
        // Create temporary config without format/lint on build
        let mut quick_config = self.config.clone();
        quick_config.tools.formatter.format_on_build = false;
        quick_config.tools.linter.lint_on_build = false;
        
        let mut quick_pipeline = BuildPipeline::new(quick_config, self.work_dir.clone())?;
        
        let context = PipelineContext {
            profile: profile.to_string(),
            targets: Vec::from([]),
            work_dir: self.work_dir.clone(),
            force_rebuild: false,
            parallel: true,
            verbose: false,
        };
        
        let pipeline_result = quick_pipeline.execute(context).await?;
        
        // Convert pipeline result to build result with enhanced metrics
        let enhanced_statistics = self.extract_enhanced_pipeline_metrics(&pipeline_result)?;
        
        Ok(BuildResult {
            success: pipeline_result.success,
            duration: pipeline_result.duration,
            targets_built: pipeline_result.stages.keys().cloned().collect(),
            targets_skipped: Vec::from([]),
            outputs: pipeline_result.artifacts.values().cloned().collect(),
            artifacts: pipeline_result.artifacts,
            warnings: pipeline_result.warnings,
            statistics: enhanced_statistics,
        })
    }
    
    /// Clean build artifacts and cache
    #[instrument(skip(self))]
    pub async fn clean_all(&mut self, clean_cache: bool) -> Result<(), BuildError> {
        info!("Cleaning build artifacts");
        
        let target_dir = self.work_dir.join("target");
        if target_dir.exists() {
            std::fs::remove_dir_all(&target_dir)
                .map_err(|e| BuildError::IoError(e))?;
            info!("Removed target directory");
        }
        
        if clean_cache {
            let cache_dir = self.work_dir.join(".cursed-cache");
            if cache_dir.exists() {
                std::fs::remove_dir_all(&cache_dir)
                    .map_err(|e| BuildError::IoError(e))?;
                info!("Removed cache directory");
            }
        }
        
        Ok(())
    }
    
    /// Watch for file changes and rebuild
    #[instrument(skip(self))]
    pub async fn watch(&mut self, profile: &str, command: &str) -> Result<(), BuildError> {
        info!("Starting file watcher for profile: {}", profile);
        
        // Configure watch settings
        let mut watch_config = WatchConfig::default();
        watch_config.build_profile = profile.to_string();
        self.watch_config = watch_config;
        
        // Start file watcher
        self.start_file_watching().await?;
        
        // Run initial build
        match command {
            "build" => { self.build_all(profile).await?; }
            "test" => { self.test(profile).await?; }
            _ => { self.build_all(profile).await?; }
        }
        
        info!("File watching active. Press Ctrl+C to stop.");
        
        // Keep the watcher running until interrupted
        // In a real implementation, this would be controlled by a signal handler
        loop {
            tokio::time::sleep(Duration::from_millis(100)).await;
            
            // Check if we need to rebuild based on file changes
            if self.check_for_rebuild_trigger().await? {
                info!("File changes detected, rebuilding...");
                match command {
                    "build" => { 
                        if let Err(e) = self.build_all(profile).await {
                            error!("Rebuild failed: {}", e);
                        }
                    }
                    "test" => { 
                        if let Err(e) = self.test(profile).await {
                            error!("Test rebuild failed: {}", e);
                        }
                    }
                    _ => { 
                        if let Err(e) = self.build_all(profile).await {
                            error!("Rebuild failed: {}", e);
                        }
                    }
                }
            }
        }
    }
    
    /// Build a specific target
    #[instrument(skip(self, target, profile))]
    pub async fn build_target(&mut self, target: &BuildTarget, profile: &BuildProfile) -> Result<BuildResult, BuildError> {
        info!("Building target: {} ({})", target.name, target.path.display());
        
        let start_time = Instant::now();
        
        // Check if target needs rebuilding
        if let Some(cached_result) = self.check_cache(target).await? {
            info!("Target '{}' is up to date, using cached result", target.name);
            return Ok(cached_result);
        }
        
        // Run pre-build scripts
        self.run_pre_build_scripts(target).await?;
        
        // Compile the target
        let compilation_result = self.compile_target(target, profile).await?;
        
        // Run post-build scripts
        self.run_post_build_scripts(target, &compilation_result).await?;
        
        // Update cache
        self.update_cache(target, &compilation_result).await?;
        
        let duration = start_time.elapsed();
        debug!("Target '{}' built in {:?}", target.name, duration);
        
        Ok(compilation_result)
    }
    
    /// Resolve package dependencies
    #[instrument(skip(self))]
    async fn resolve_dependencies(&mut self) -> Result<(), BuildError> {
        info!("Resolving dependencies");
        
        // Use dependency resolver to create dependency graph
        let graph = self.dependency_resolver.resolve(&self.config.dependencies).await
            .map_err(|e| BuildError::DependencyError(e.to_string()))?;
        
        // Download and install packages through package manager
        for (package, version) in &self.config.dependencies {
            self.package_manager.install_package(package, Some(version)).await?;
        }
        
        Ok(())
    }
    
    /// Check if target is cached and up to date
    async fn check_cache(&self, target: &BuildTarget) -> Result<Option<BuildResult>, BuildError> {
        if let Some(entry) = self.cache.get(&target.name) {
            // Check if source files have changed
            let source_modified = self.get_source_modification_time(&target.path)?;
            
            if entry.timestamp >= source_modified {
                // Cache hit - return cached result
                return Ok(Some(BuildResult {
                    success: true,
                    duration: Duration::from_millis(0),
                    targets_built: Vec::from([]),
                    targets_skipped: Vec::from([target.name.clone()]),
                    outputs: entry.outputs.clone(),
                    artifacts: entry.artifacts.clone(),
                    warnings: Vec::from([]),
                    statistics: BuildStatistics {
                        files_compiled: 0,
                        files_cached: entry.files_count,
                        lines_compiled: 0,
                        peak_memory: 0,
                        phase_timings: HashMap::new(),
                    },
                }));
            }
        }
        
        Ok(None)
    }
    
    /// Get modification time for source files
    fn get_source_modification_time(&self, path: &Path) -> Result<std::time::SystemTime, BuildError> {
        let metadata = std::fs::metadata(path)?;
        Ok(metadata.modified()?)
    }
    
    /// Compile a target using CURSED compiler
    #[instrument(skip(self, target, profile))]
    async fn compile_target(&self, target: &BuildTarget, profile: &BuildProfile) -> Result<BuildResult, BuildError> {
        let start_time = Instant::now();
        
        // Determine output path using actual profile name
        let profile_name = match profile.optimization {
            OptimizationLevel::None => "debug",
            OptimizationLevel::Basic => "dev", 
            OptimizationLevel::Max => "release",
            OptimizationLevel::Size => "release-small",
        };
        
        let output_dir = self.work_dir.join("target").join(profile_name);
        std::fs::create_dir_all(&output_dir)?;
        
        let output_path = match target.target_type {
            TargetType::Bin => {
                output_dir.join(&target.name)
            }
            TargetType::Lib |
            TargetType::StaticLib => {
                output_dir.join(format!("lib{}.a", target.name))
            }
            TargetType::DynLib |
            TargetType::CDynLib => {
                output_dir.join(format!("lib{}.so", target.name))
            }
        };
        
        // Start memory monitoring
        let memory_monitor = MemoryMonitor::new();
        memory_monitor.start_monitoring();
        
        // Build compiler command
        let mut cmd = Command::new("./target/debug/cursed");
        cmd.arg("compile")
           .arg(&target.path)
           .arg("--output")
           .arg(&output_path)
           .arg("--metrics"); // Enable detailed metrics collection
        
        // Add optimization flags based on profile
        match profile.optimization {
            OptimizationLevel::None => {
                cmd.arg("--optimization").arg("none");
            }
            OptimizationLevel::Basic => {
                cmd.arg("--optimization").arg("basic");
            }
            OptimizationLevel::Max => {
                cmd.arg("--optimization").arg("max");
            }
            OptimizationLevel::Size => {
                cmd.arg("--optimization").arg("size");
            }
        }
        
        // Add debug information
        if profile.debug {
            cmd.arg("--debug");
        }
        
        // Add LLVM arguments
        for arg in &profile.llvm_args {
            cmd.arg("--llvm-arg").arg(arg);
        }
        
        // Set environment variables
        for (key, value) in &profile.env {
            cmd.env(key, value);
        }
        
        // Execute compilation
        debug!("Executing: {:?}", cmd);
        let compilation_start = Instant::now();
        let output = cmd.stdout(Stdio::piped())
                       .stderr(Stdio::piped())
                       .spawn()?
                       .wait_with_output()?;
        let compilation_duration = compilation_start.elapsed();
        
        // Stop memory monitoring and get peak usage
        let memory_stats = memory_monitor.stop_monitoring();
        let peak_memory = memory_stats.peak_memory_mb as usize * 1024 * 1024; // Convert to bytes
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(BuildError::CompilationError(format!(
                "Compilation failed for target '{}': {}", target.name, stderr
            )));
        }
        
        // Parse compilation output for warnings and metrics
        let stdout = String::from_utf8_lossy(&output.stdout);
        let warnings = extract_warnings(&stdout);
        let compilation_metrics = extract_compilation_metrics(&stdout);
        
        let duration = start_time.elapsed();
        
        // Create build result
        let mut artifacts = HashMap::new();
        artifacts.insert(target.name.clone(), output_path.clone());
        
        // Extract phase timings from compilation metrics
        let mut phase_timings = HashMap::new();
        phase_timings.insert("compilation".to_string(), compilation_duration);
        
        if let Some(parsing_time) = compilation_metrics.get("parsing_time") {
            phase_timings.insert("parsing".to_string(), Duration::from_millis(*parsing_time as u64));
        }
        if let Some(analysis_time) = compilation_metrics.get("analysis_time") {
            phase_timings.insert("analysis".to_string(), Duration::from_millis(*analysis_time as u64));
        }
        if let Some(codegen_time) = compilation_metrics.get("codegen_time") {
            phase_timings.insert("codegen".to_string(), Duration::from_millis(*codegen_time as u64));
        }
        if let Some(linking_time) = compilation_metrics.get("linking_time") {
            phase_timings.insert("linking".to_string(), Duration::from_millis(*linking_time as u64));
        }
        
        Ok(BuildResult {
            success: true,
            duration,
            targets_built: Vec::from([target.name.clone()]),
            targets_skipped: Vec::from([]),
            outputs: Vec::from([output_path]),
            artifacts,
            warnings,
            statistics: BuildStatistics {
                files_compiled: 1,
                files_cached: 0,
                lines_compiled: compilation_metrics.get("lines_compiled").copied().unwrap_or(0.0) as usize,
                peak_memory,
                phase_timings,
            },
        })
    }
    
    /// Run pre-build scripts
    async fn run_pre_build_scripts(&self, target: &BuildTarget) -> Result<(), BuildError> {
        if let Some(script) = self.config.scripts.get("pre-build") {
            info!("Running pre-build script for target: {}", target.name);
            self.run_script(script).await?;
        }
        
        // Target-specific pre-build script
        let target_script_name = format!("pre-build-{}", target.name);
        if let Some(script) = self.config.scripts.get(&target_script_name) {
            info!("Running target-specific pre-build script for: {}", target.name);
            self.run_script(script).await?;
        }
        
        Ok(())
    }
    
    /// Run post-build scripts
    async fn run_post_build_scripts(&self, target: &BuildTarget, result: &BuildResult) -> Result<(), BuildError> {
        if let Some(script) = self.config.scripts.get("post-build") {
            info!("Running post-build script for target: {}", target.name);
            self.run_script(script).await?;
        }
        
        // Target-specific post-build script
        let target_script_name = format!("post-build-{}", target.name);
        if let Some(script) = self.config.scripts.get(&target_script_name) {
            info!("Running target-specific post-build script for: {}", target.name);
            self.run_script(script).await?;
        }
        
        Ok(())
    }
    
    /// Execute a build script
    async fn run_script(&self, script: &str) -> Result<(), BuildError> {
        let mut cmd = if cfg!(target_os = "windows") {
            Command::new("cmd")
        } else {
            Command::new("sh")
        };
        
        if cfg!(target_os = "windows") {
            cmd.args(["/C", script]);
        } else {
            cmd.args(["-c", script]);
        }
        
        cmd.current_dir(&self.work_dir);
        
        let output = cmd.output()?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(BuildError::ToolError(format!(
                "Script execution failed: {}", stderr
            )));
        }
        
        Ok(())
    }
    
    /// Update build cache
    async fn update_cache(&mut self, target: &BuildTarget, result: &BuildResult) -> Result<(), BuildError> {
        self.cache.insert(
            &target.name,
            result.outputs.clone(),
            result.artifacts.clone(),
            result.statistics.files_compiled,
        )?;
        
        Ok(())
    }
    
    /// Clean build artifacts
    #[instrument(skip(self))]
    pub fn clean(&self) -> Result<(), BuildError> {
        info!("Cleaning build artifacts");
        
        let target_dir = self.work_dir.join("target");
        if target_dir.exists() {
            std::fs::remove_dir_all(&target_dir)?;
            info!("Removed target directory: {}", target_dir.display());
        }
        
        Ok(())
    }
    
    /// Run tests with comprehensive discovery and execution
    #[instrument(skip(self))]
    pub async fn test(&mut self, profile: &str) -> Result<BuildResult, BuildError> {
        info!("Running comprehensive test suite with profile: {}", profile);
        
        let start_time = Instant::now();
        
        // Configure test discovery
        let discovery_config = TestDiscoveryConfig {
            root_dir: self.work_dir.clone(),
            include_unit_tests: true,
            include_integration_tests: true,
            include_doc_tests: false,
            include_benchmarks: false,
            include_examples: false,
            custom_patterns: Vec::new(),
            exclude_patterns: vec![
                "target/**".to_string(),
                ".git/**".to_string(),
                "*.bak".to_string(),
            ],
        };
        
        // Discover tests
        let test_discovery = TestDiscovery::new(discovery_config)
            .map_err(|e| BuildError::ConfigError(e.to_string()))?;
        
        let discovery_result = test_discovery.discover_tests()
            .map_err(|e| BuildError::ConfigError(e.to_string()))?;
        
        info!("Discovered {} tests ({} unit, {} integration)", 
              discovery_result.statistics.total_tests,
              discovery_result.statistics.unit_tests,
              discovery_result.statistics.integration_tests);
        
        if discovery_result.tests.is_empty() {
            warn!("No tests found in project");
            return Ok(BuildResult {
                success: true,
                duration: start_time.elapsed(),
                targets_built: Vec::from(["tests".to_string()]),
                targets_skipped: Vec::from([]),
                outputs: Vec::from([]),
                artifacts: HashMap::new(),
                warnings: vec!["No tests found in project".to_string()],
                statistics: BuildStatistics {
                    files_compiled: 0,
                    files_cached: 0,
                    lines_compiled: 0,
                    peak_memory: 0,
                    phase_timings: HashMap::new(),
                },
            });
        }
        
        // Configure test execution
        let mut execution_config = TestExecutionConfig::default();
        execution_config.work_dir = self.work_dir.clone();
        execution_config.release_mode = profile == "release";
        execution_config.use_linking_fix = true; // Enable Nix environment linking fix
        execution_config.linking_fix_script = Some(self.work_dir.join("fix_linking.sh"));
        execution_config.parallel_threads = std::cmp::min(num_cpus::get(), 4); // Limit parallelism
        
        // Add linking fix environment variables for Nix compatibility
        execution_config.env_vars.insert(
            "LIBRARY_PATH".to_string(),
            "/nix/store/6pak77li0iw9x0b3yhmbjvp846w3p6bx-libffi-3.4.6/lib:/nix/store/l5g2v1jgfyf3j0jp9iv5b79fi8yrwzpp-zlib-1.3.1/lib:/nix/store/k3a7dzrqphj9ksbb43i24vy6inz8ys51-ncurses-6.4.20221231/lib:/nix/store/hd6llsw2dkiazk9d2ywv13cc6alhflly-libxml2-2.13.5/lib:/nix/store/dsqzw96w4sxsp4q9yvkfl2yh701mpwgi-sqlite-3.46.1/lib".to_string()
        );
        execution_config.env_vars.insert(
            "RUSTFLAGS".to_string(),
            "-C linker=gcc -C link-arg=-fuse-ld=bfd".to_string()
        );
        
        // Filter tests if needed (exclude ignored tests by default)
        let test_filter = TestFilter::default();
        let tests_to_run = test_filter.apply(&discovery_result);
        
        info!("Running {} tests (filtered from {})", tests_to_run.len(), discovery_result.tests.len());
        
        // Execute tests
        let test_executor = TestExecutor::new(execution_config);
        let execution_result = test_executor.execute_tests(tests_to_run).await
            .map_err(|e| BuildError::CompilationError(e.to_string()))?;
        
        // Convert test execution results to build results
        let build_result = self.convert_test_results_to_build_result(execution_result, start_time.elapsed())?;
        
        if build_result.success {
            info!("All tests passed successfully!");
        } else {
            error!("Some tests failed. Check output for details.");
        }
        
        Ok(build_result)
    }
    
    /// Run tests with custom filter patterns
    #[instrument(skip(self))]
    pub async fn test_with_filter(&mut self, profile: &str, patterns: &[String]) -> Result<BuildResult, BuildError> {
        info!("Running filtered tests with patterns: {:?}", patterns);
        
        let start_time = Instant::now();
        
        // Configure test discovery
        let discovery_config = TestDiscoveryConfig {
            root_dir: self.work_dir.clone(),
            include_unit_tests: true,
            include_integration_tests: true,
            include_doc_tests: false,
            include_benchmarks: false,
            include_examples: false,
            custom_patterns: Vec::new(),
            exclude_patterns: vec![
                "target/**".to_string(),
                ".git/**".to_string(),
                "*.bak".to_string(),
            ],
        };
        
        // Discover tests
        let test_discovery = TestDiscovery::new(discovery_config)
            .map_err(|e| BuildError::ConfigError(e.to_string()))?;
        
        let discovery_result = test_discovery.discover_tests()
            .map_err(|e| BuildError::ConfigError(e.to_string()))?;
        
        // Apply custom filter patterns
        let filtered_tests = test_discovery.filter_tests(&discovery_result, patterns);
        
        info!("Found {} tests matching patterns from {} total tests", 
              filtered_tests.len(), discovery_result.statistics.total_tests);
        
        if filtered_tests.is_empty() {
            warn!("No tests found matching filter patterns: {:?}", patterns);
            return Ok(BuildResult {
                success: true,
                duration: start_time.elapsed(),
                targets_built: Vec::from(["tests".to_string()]),
                targets_skipped: Vec::from([]),
                outputs: Vec::from([]),
                artifacts: HashMap::new(),
                warnings: vec![format!("No tests found matching patterns: {:?}", patterns)],
                statistics: BuildStatistics {
                    files_compiled: 0,
                    files_cached: 0,
                    lines_compiled: 0,
                    peak_memory: 0,
                    phase_timings: HashMap::new(),
                },
            });
        }
        
        // Configure test execution
        let mut execution_config = TestExecutionConfig::default();
        execution_config.work_dir = self.work_dir.clone();
        execution_config.release_mode = profile == "release";
        execution_config.use_linking_fix = true;
        execution_config.linking_fix_script = Some(self.work_dir.join("fix_linking.sh"));
        
        // Add linking fix environment variables
        execution_config.env_vars.insert(
            "LIBRARY_PATH".to_string(),
            "/nix/store/6pak77li0iw9x0b3yhmbjvp846w3p6bx-libffi-3.4.6/lib:/nix/store/l5g2v1jgfyf3j0jp9iv5b79fi8yrwzpp-zlib-1.3.1/lib:/nix/store/k3a7dzrqphj9ksbb43i24vy6inz8ys51-ncurses-6.4.20221231/lib:/nix/store/hd6llsw2dkiazk9d2ywv13cc6alhflly-libxml2-2.13.5/lib:/nix/store/dsqzw96w4sxsp4q9yvkfl2yh701mpwgi-sqlite-3.46.1/lib".to_string()
        );
        execution_config.env_vars.insert(
            "RUSTFLAGS".to_string(),
            "-C linker=gcc -C link-arg=-fuse-ld=bfd".to_string()
        );
        
        // Execute filtered tests
        let test_executor = TestExecutor::new(execution_config);
        let execution_result = test_executor.execute_tests(filtered_tests).await
            .map_err(|e| BuildError::CompilationError(e.to_string()))?;
        
        // Convert test execution results to build results
        let build_result = self.convert_test_results_to_build_result(execution_result, start_time.elapsed())?;
        
        Ok(build_result)
    }
    
    /// Run only ignored tests
    #[instrument(skip(self))]
    pub async fn test_ignored(&mut self, profile: &str) -> Result<BuildResult, BuildError> {
        info!("Running ignored tests with profile: {}", profile);
        
        let start_time = Instant::now();
        
        // Configure test discovery
        let discovery_config = TestDiscoveryConfig {
            root_dir: self.work_dir.clone(),
            include_unit_tests: true,
            include_integration_tests: true,
            include_doc_tests: false,
            include_benchmarks: false,
            include_examples: false,
            custom_patterns: Vec::new(),
            exclude_patterns: vec![
                "target/**".to_string(),
                ".git/**".to_string(),
                "*.bak".to_string(),
            ],
        };
        
        // Discover tests
        let test_discovery = TestDiscovery::new(discovery_config)
            .map_err(|e| BuildError::ConfigError(e.to_string()))?;
        
        let discovery_result = test_discovery.discover_tests()
            .map_err(|e| BuildError::ConfigError(e.to_string()))?;
        
        // Filter for ignored tests only
        let test_filter = TestFilter {
            only_ignored: true,
            include_ignored: true,
            ..Default::default()
        };
        let ignored_tests = test_filter.apply(&discovery_result);
        
        info!("Found {} ignored tests out of {} total tests", 
              ignored_tests.len(), discovery_result.statistics.total_tests);
        
        if ignored_tests.is_empty() {
            info!("No ignored tests found");
            return Ok(BuildResult {
                success: true,
                duration: start_time.elapsed(),
                targets_built: Vec::from(["ignored_tests".to_string()]),
                targets_skipped: Vec::from([]),
                outputs: Vec::from([]),
                artifacts: HashMap::new(),
                warnings: Vec::from([]),
                statistics: BuildStatistics {
                    files_compiled: 0,
                    files_cached: 0,
                    lines_compiled: 0,
                    peak_memory: 0,
                    phase_timings: HashMap::new(),
                },
            });
        }
        
        // Configure test execution for ignored tests
        let mut execution_config = TestExecutionConfig::default();
        execution_config.work_dir = self.work_dir.clone();
        execution_config.release_mode = profile == "release";
        execution_config.use_linking_fix = true;
        execution_config.linking_fix_script = Some(self.work_dir.join("fix_linking.sh"));
        execution_config.cargo_args.push("--ignored".to_string()); // Add --ignored flag for cargo test
        
        // Add linking fix environment variables
        execution_config.env_vars.insert(
            "LIBRARY_PATH".to_string(),
            "/nix/store/6pak77li0iw9x0b3yhmbjvp846w3p6bx-libffi-3.4.6/lib:/nix/store/l5g2v1jgfyf3j0jp9iv5b79fi8yrwzpp-zlib-1.3.1/lib:/nix/store/k3a7dzrqphj9ksbb43i24vy6inz8ys51-ncurses-6.4.20221231/lib:/nix/store/hd6llsw2dkiazk9d2ywv13cc6alhflly-libxml2-2.13.5/lib:/nix/store/dsqzw96w4sxsp4q9yvkfl2yh701mpwgi-sqlite-3.46.1/lib".to_string()
        );
        execution_config.env_vars.insert(
            "RUSTFLAGS".to_string(),
            "-C linker=gcc -C link-arg=-fuse-ld=bfd".to_string()
        );
        
        // Execute ignored tests
        let test_executor = TestExecutor::new(execution_config);
        let execution_result = test_executor.execute_tests(ignored_tests).await
            .map_err(|e| BuildError::CompilationError(e.to_string()))?;
        
        // Convert test execution results to build results
        let build_result = self.convert_test_results_to_build_result(execution_result, start_time.elapsed())?;
        
        Ok(build_result)
    }
    
    /// Convert test execution results to BuildResult format
    fn convert_test_results_to_build_result(
        &self, 
        execution_result: TestExecutionResult, 
        total_duration: Duration
    ) -> Result<BuildResult, BuildError> {
        let success = execution_result.summary.success;
        let statistics = &execution_result.statistics;
        
        // Create warnings from failed tests
        let mut warnings = Vec::new();
        for failed_test in &execution_result.summary.failed_tests {
            warnings.push(format!(
                "Test '{}' failed: {}",
                failed_test.test_name,
                failed_test.reason
            ));
        }
        
        // Add performance insights as warnings
        for insight in &execution_result.summary.performance_insights {
            warnings.push(format!("Performance insight: {}", insight));
        }
        
        // Create build statistics from test statistics
        let build_statistics = BuildStatistics {
            files_compiled: statistics.total_tests,
            files_cached: 0, // Tests aren't cached in the same way
            lines_compiled: 0, // Could be computed from test files
            peak_memory: statistics.total_memory_usage.unwrap_or(0),
            phase_timings: {
                let mut timings = HashMap::new();
                timings.insert("test_discovery".to_string(), Duration::from_millis(100)); // Estimated
                timings.insert("test_execution".to_string(), execution_result.total_duration);
                timings
            },
        };
        
        // Create artifacts from test results (test reports, coverage, etc.)
        let mut artifacts = HashMap::new();
        artifacts.insert(
            "test_results".to_string(),
            self.work_dir.join("target").join("test_results.json")
        );
        
        Ok(BuildResult {
            success,
            duration: total_duration,
            targets_built: vec![format!("tests ({} passed)", statistics.passed)],
            targets_skipped: vec![format!("{} ignored", statistics.ignored)],
            outputs: vec![self.work_dir.join("target").join("test_results.json")],
            artifacts,
            warnings,
            statistics: build_statistics,
        })
    }
    
    /// Run toolchain integration (format, lint, etc.)
    #[instrument(skip(self))]
    pub async fn run_tools(&self) -> Result<(), BuildError> {
        info!("Running toolchain integration");
        
        // Run formatter
        if self.config.tools.formatter.format_on_build {
            self.run_formatter().await?;
        }
        
        // Run linter
        if self.config.tools.linter.lint_on_build {
            self.run_linter().await?;
        }
        
        // Run documentation generator
        if self.config.tools.docs.generate_on_build {
            self.run_docs().await?;
        }
        
        Ok(())
    }
    
    /// Run CURSED formatter
    async fn run_formatter(&self) -> Result<(), BuildError> {
        info!("Running CURSED formatter");
        
        let output = Command::new("./target/debug/cursed-fmt")
            .arg("--check")
            .arg("src")
            .current_dir(&self.work_dir)
            .output()?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            warn!("Formatter found issues: {}", stderr);
        }
        
        Ok(())
    }
    
    /// Run CURSED linter
    async fn run_linter(&self) -> Result<(), BuildError> {
        info!("Running CURSED linter");
        
        let output = Command::new("./target/debug/cursed_lint_new")
            .arg("src")
            .current_dir(&self.work_dir)
            .output()?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            warn!("Linter found issues: {}", stderr);
        }
        
        Ok(())
    }
    
    /// Run documentation generator
    async fn run_docs(&self) -> Result<(), BuildError> {
        info!("Running documentation generator");
        
        let output = Command::new("./target/debug/cursed-doc")
            .arg("--html")
            .arg("--source")
            .arg("src")
            .arg("--output")
            .arg("docs")
            .current_dir(&self.work_dir)
            .output()?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            warn!("Documentation generator issues: {}", stderr);
        }
        
        Ok(())
    }
    
    /// Start file watching
    #[instrument(skip(self))]
    pub async fn start_file_watching(&mut self) -> Result<(), BuildError> {
        if self.file_watcher.is_some() {
            warn!("File watcher already running");
            return Ok(());
        }
        
        info!("Configuring file watcher for patterns: {:?}", self.watch_config.patterns);
        
        let (event_sender, event_receiver) = mpsc::channel();
        let (shutdown_sender, shutdown_receiver) = mpsc::channel();
        
        // Create the watcher
        let mut watcher = RecommendedWatcher::new(
            move |res| {
                if let Err(e) = event_sender.send(res) {
                    error!("Failed to send file watcher event: {}", e);
                }
            },
            Config::default(),
        )?;
        
        // Watch source directories
        let src_dir = self.work_dir.join("src");
        if src_dir.exists() {
            watcher.watch(&src_dir, RecursiveMode::Recursive)?;
            debug!("Watching directory: {}", src_dir.display());
        }
        
        // Watch examples directory
        let examples_dir = self.work_dir.join("examples");
        if examples_dir.exists() {
            watcher.watch(&examples_dir, RecursiveMode::Recursive)?;
            debug!("Watching directory: {}", examples_dir.display());
        }
        
        // Watch tests directory
        let tests_dir = self.work_dir.join("tests");
        if tests_dir.exists() {
            watcher.watch(&tests_dir, RecursiveMode::Recursive)?;
            debug!("Watching directory: {}", tests_dir.display());
        }
        
        // Watch configuration files
        let config_files = [
            "Cargo.toml",
            "CursedBuild.toml", 
            "CursedPackage.toml",
            "cursed.lock",
        ];
        
        for config_file in &config_files {
            let config_path = self.work_dir.join(config_file);
            if config_path.exists() {
                watcher.watch(&config_path, RecursiveMode::NonRecursive)?;
                debug!("Watching file: {}", config_path.display());
            }
        }
        
        // Start watch thread
        let debounce_duration = Duration::from_millis(self.watch_config.debounce_ms);
        let last_trigger = Arc::new(Mutex::new(Instant::now()));
        let last_trigger_clone = Arc::clone(&last_trigger);
        
        let watch_thread = thread::spawn(move || {
            let mut pending_events = Vec::new();
            
            loop {
                // Check for shutdown signal
                if shutdown_receiver.try_recv().is_ok() {
                    debug!("File watcher thread received shutdown signal");
                    break;
                }
                
                // Process file events
                match event_receiver.try_recv() {
                    Ok(Ok(event)) => {
                        match event.kind {
                            EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                                // Check if this is a file we care about
                                let should_trigger = event.paths.iter().any(|path| {
                                    let path_str = path.to_string_lossy();
                                    path_str.ends_with(".csd") ||
                                    path_str.ends_with(".toml") ||
                                    path_str.contains("Cargo.toml") ||
                                    path_str.contains("CursedBuild.toml") ||
                                    path_str.contains("CursedPackage.toml")
                                });
                                
                                if should_trigger {
                                    debug!("File change detected: {:?}", event.paths);
                                    pending_events.push(event);
                                    
                                    // Update last trigger time
                                    if let Ok(mut last) = last_trigger_clone.lock() {
                                        *last = Instant::now();
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    Ok(Err(e)) => {
                        error!("File watcher error: {}", e);
                    }
                    Err(_) => {
                        // No events, check if we need to trigger a rebuild
                        if !pending_events.is_empty() {
                            if let Ok(last) = last_trigger_clone.lock() {
                                if last.elapsed() >= debounce_duration {
                                    debug!("Debounce period elapsed, {} events pending", pending_events.len());
                                    pending_events.clear();
                                }
                            }
                        }
                        
                        // Small sleep to prevent busy waiting
                        thread::sleep(Duration::from_millis(10));
                    }
                }
            }
        });
        
        self.file_watcher = Some(FileWatcher {
            watcher,
            event_receiver: None, // Moved to thread
            watch_thread: Some(watch_thread),
            shutdown_sender: Some(shutdown_sender),
        });
        
        info!("File watcher started successfully");
        Ok(())
    }
    
    /// Stop file watching
    #[instrument(skip(self))]
    pub async fn stop_file_watching(&mut self) -> Result<(), BuildError> {
        if let Some(mut file_watcher) = self.file_watcher.take() {
            info!("Stopping file watcher");
            
            // Send shutdown signal
            if let Some(shutdown_sender) = file_watcher.shutdown_sender.take() {
                let _ = shutdown_sender.send(());
            }
            
            // Wait for thread to finish
            if let Some(watch_thread) = file_watcher.watch_thread.take() {
                if let Err(e) = watch_thread.join() {
                    error!("Error joining file watcher thread: {:?}", e);
                }
            }
            
            info!("File watcher stopped");
        }
        
        Ok(())
    }
    
    /// Check if we need to trigger a rebuild
    async fn check_for_rebuild_trigger(&self) -> Result<bool, BuildError> {
        // This is a placeholder - in the real implementation,
        // the file watcher thread would set a flag that we check here
        // For now, we return false to prevent constant rebuilding
        Ok(false)
    }
    
    /// Configure watch patterns
    pub fn set_watch_config(&mut self, config: WatchConfig) {
        self.watch_config = config;
    }
    
    /// Get current watch configuration
    pub fn get_watch_config(&self) -> &WatchConfig {
        &self.watch_config
    }
    
    /// Enable advanced parallel compilation
    #[instrument(skip(self))]
    pub async fn enable_parallel_compilation(&mut self, config: Option<ParallelCompilationConfig>) -> Result<(), BuildError> {
        info!("Enabling advanced parallel compilation");
        
        let parallel_config = config.unwrap_or_default();
        let compiler = ParallelCompiler::new(parallel_config)
            .map_err(|e| BuildError::ConfigError(e.to_string()))?;
        
        let worker_count = parallel_config.max_workers;
        self.parallel_compiler = Some(compiler);
        info!("Parallel compilation enabled with {} workers", worker_count);
        
        Ok(())
    }
    
    /// Enable incremental optimization
    #[instrument(skip(self))]
    pub async fn enable_incremental_optimization(&mut self, config: Option<IncrementalConfig>) -> Result<(), BuildError> {
        info!("Enabling incremental compilation optimization");
        
        let incremental_config = config.unwrap_or_default();
        let optimizer = IncrementalOptimizer::new(incremental_config, self.work_dir.clone())
            .map_err(|e| BuildError::ConfigError(e.to_string()))?;
        
        self.incremental_optimizer = Some(optimizer);
        info!("Incremental optimization enabled with fine-grained dependency tracking");
        
        Ok(())
    }
    
    /// Enable build profiling
    #[instrument(skip(self))]
    pub async fn enable_build_profiling(&mut self, config: Option<ProfilerConfig>) -> Result<(), BuildError> {
        info!("Enabling build performance profiling");
        
        let profiler_config = config.unwrap_or_default();
        let profiler = BuildProfiler::new(profiler_config)
            .map_err(|e| BuildError::ConfigError(e.to_string()))?;
        
        self.build_profiler = Some(profiler);
        info!("Build profiling enabled with detailed performance analysis");
        
        Ok(())
    }
    
    /// Enable artifact management
    #[instrument(skip(self))]
    pub async fn enable_artifact_management(&mut self, config: Option<ArtifactConfig>) -> Result<(), BuildError> {
        info!("Enabling advanced artifact management");
        
        let artifact_config = config.unwrap_or_default();
        let manager = ArtifactManager::new(artifact_config)
            .map_err(|e| BuildError::ConfigError(e.to_string()))?;
        
        self.artifact_manager = Some(manager);
        info!("Artifact management enabled with intelligent storage and versioning");
        
        Ok(())
    }
    
    /// Build with advanced optimization
    #[instrument(skip(self))]
    pub async fn build_optimized(&mut self, profile: &str) -> Result<BuildResult, BuildError> {
        info!("Starting optimized build with advanced features");
        
        // Start profiling if enabled
        if let Some(ref mut profiler) = self.build_profiler {
            profiler.start_profiling(
                self.config.clone(),
                self.config.targets.clone(),
                self.config.get_effective_profile(profile)
                    .map_err(|e| BuildError::ConfigError(e.to_string()))?
            ).await?;
        }
        
        let start_time = Instant::now();
        let mut result = BuildResult {
            success: true,
            duration: Duration::default(),
            targets_built: Vec::new(),
            targets_skipped: Vec::new(),
            outputs: Vec::new(),
            artifacts: HashMap::new(),
            warnings: Vec::new(),
            statistics: BuildStatistics {
                files_compiled: 0,
                files_cached: 0,
                lines_compiled: 0,
                peak_memory: 0,
                phase_timings: HashMap::new(),
            },
        };
        
        // Run incremental analysis if enabled
        if let Some(ref mut optimizer) = self.incremental_optimizer {
            let incremental_plan = optimizer.analyze_incremental_build(
                &self.config.targets,
                &self.config.get_effective_profile(profile)
                    .map_err(|e| BuildError::ConfigError(e.to_string()))?
            ).await?;
            
            info!(
                "Incremental analysis: {} files to compile, {} from cache, {:.1}% cache hit rate",
                incremental_plan.files_to_compile.len(),
                incremental_plan.files_from_cache.len(),
                incremental_plan.cache_hit_rate * 100.0
            );
            
            result.targets_skipped = incremental_plan.files_from_cache
                .iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect();
        }
        
        // Use parallel compilation if enabled
        if let Some(ref mut parallel_compiler) = self.parallel_compiler {
            // Convert targets to compilation tasks
            let compilation_tasks = self.create_compilation_tasks(&self.config.targets, profile)?;
            
            let parallel_result = parallel_compiler.compile_parallel(
                compilation_tasks,
                &self.config.get_effective_profile(profile)
                    .map_err(|e| BuildError::ConfigError(e.to_string()))?
            ).await?;
            
            info!(
                "Parallel compilation completed: {} tasks, {:.1}% efficiency",
                parallel_result.tasks_completed,
                parallel_result.parallel_efficiency * 100.0
            );
            
            // Convert parallel result to build result
            result.success = parallel_result.success;
            result.targets_built = parallel_result.worker_statistics
                .iter()
                .map(|ws| format!("worker_{}", ws.worker_id))
                .collect();
            result.statistics.files_compiled = parallel_result.tasks_completed;
            result.statistics.peak_memory = parallel_result.resource_utilization.peak_memory_usage;
        } else {
            // Fall back to standard build
            result = self.build_all(profile).await?;
        }
        
        // Store artifacts if artifact management is enabled
        if let Some(ref mut artifact_manager) = self.artifact_manager {
            let stored_artifacts = artifact_manager.store_artifacts(
                &result,
                &self.config,
                &self.config.get_effective_profile(profile)
                    .map_err(|e| BuildError::ConfigError(e.to_string()))?
            ).await.map_err(|e| BuildError::ConfigError(e.to_string()))?;
            
            info!("Stored {} artifacts", stored_artifacts.len());
        }
        
        result.duration = start_time.elapsed();
        
        // Generate profiling report if enabled
        if let Some(ref mut profiler) = self.build_profiler {
            let profiling_report = profiler.stop_profiling().await?;
            
            info!(
                "Build profiling completed - Performance score: {:.1}, {} optimization recommendations",
                profiling_report.performance_analysis.overall_score * 100.0,
                profiling_report.optimization_recommendations.len()
            );
            
            // Add profiling insights to warnings
            for recommendation in &profiling_report.optimization_recommendations {
                result.warnings.push(format!(
                    "Performance recommendation: {} (potential savings: {:?})",
                    recommendation.title,
                    recommendation.expected_improvement.time_savings
                ));
            }
        }
        
        info!("Optimized build completed in {:?}", result.duration);
        Ok(result)
    }
    
    /// Get performance insights from profiler
    pub fn get_performance_insights(&self) -> Option<String> {
        self.build_profiler.as_ref().map(|profiler| {
            let stats = profiler.get_current_statistics();
            format!(
                "Current build performance:\n- Total time: {:?}\n- Compilation time: {:?}\n- Peak memory: {} MB\n- CPU usage: {:.1}%",
                stats.timing_metrics.total_build_time,
                stats.timing_metrics.compilation_time,
                stats.resource_metrics.peak_memory_usage / (1024 * 1024),
                stats.resource_metrics.peak_cpu_usage
            )
        })
    }
    
    /// Get artifact management statistics
    pub fn get_artifact_statistics(&self) -> Option<String> {
        self.artifact_manager.as_ref().map(|manager| {
            let stats = manager.get_statistics();
            format!(
                "Artifact storage:\n- Total artifacts: {}\n- Storage used: {} MB\n- Cache hit rate: {:.1}%\n- Deduplication savings: {} MB",
                stats.total_artifacts,
                stats.total_storage_used / (1024 * 1024),
                stats.cache_hit_rate * 100.0,
                stats.deduplication_savings / (1024 * 1024)
            )
        })
    }
    
    /// Extract enhanced metrics from pipeline results
    fn extract_enhanced_pipeline_metrics(&self, pipeline_result: &PipelineResult) -> Result<BuildStatistics, BuildError> {
        let mut phase_timings = HashMap::new();
        let mut total_lines_compiled = 0;
        
        // Extract stage timings and metrics from pipeline
        for (stage_name, stage_info) in &pipeline_result.stages {
            if let Some(duration) = stage_info.get("duration") {
                if let Some(duration_str) = duration.as_str() {
                    if let Ok(millis) = duration_str.parse::<u64>() {
                        phase_timings.insert(stage_name.clone(), Duration::from_millis(millis));
                    }
                }
            }
            
            // Extract lines compiled for this stage
            if let Some(lines) = stage_info.get("lines_compiled") {
                if let Some(lines_str) = lines.as_str() {
                    if let Ok(lines_count) = lines_str.parse::<usize>() {
                        total_lines_compiled += lines_count;
                    }
                }
            }
        }
        
        // Add overall pipeline phases
        phase_timings.insert("pipeline_setup".to_string(), Duration::from_millis(50));
        phase_timings.insert("dependency_resolution".to_string(), Duration::from_millis(100));
        phase_timings.insert("task_scheduling".to_string(), Duration::from_millis(25));
        
        // Calculate additional metrics from pipeline statistics
        let files_compiled = pipeline_result.statistics.stages_executed;
        let files_cached = pipeline_result.statistics.stages_cached;
        let peak_memory = pipeline_result.statistics.resource_usage.peak_memory;
        
        Ok(BuildStatistics {
            files_compiled,
            files_cached,
            lines_compiled: total_lines_compiled,
            peak_memory,
            phase_timings,
        })
    }
    
    /// Analyze build performance and generate recommendations
    fn analyze_build_performance(&self, statistics: &BuildStatistics) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        // Analyze compilation phase distribution
        let total_time: Duration = statistics.phase_timings.values().sum();
        
        for (phase, duration) in &statistics.phase_timings {
            let percentage = if total_time.as_millis() > 0 {
                (duration.as_millis() as f64 / total_time.as_millis() as f64) * 100.0
            } else {
                0.0
            };
            
            match phase.as_str() {
                "compilation" if percentage > 60.0 => {
                    recommendations.push("Consider enabling parallel compilation to reduce compilation time".to_string());
                }
                "linking" if percentage > 30.0 => {
                    recommendations.push("Linking is a bottleneck - consider incremental linking or LTO optimization".to_string());
                }
                "parsing" if percentage > 20.0 => {
                    recommendations.push("Parsing overhead is high - consider precompiled headers or modules".to_string());
                }
                _ => {}
            }
        }
        
        // Memory usage analysis
        if statistics.peak_memory > 2 * 1024 * 1024 * 1024 { // > 2GB
            recommendations.push("High memory usage detected - consider reducing parallel workers or enabling incremental compilation".to_string());
        }
        
        // Cache efficiency analysis
        let cache_hit_rate = if statistics.files_compiled + statistics.files_cached > 0 {
            statistics.files_cached as f64 / (statistics.files_compiled + statistics.files_cached) as f64
        } else {
            0.0
        };
        
        if cache_hit_rate < 0.3 {
            recommendations.push("Low cache hit rate - ensure incremental builds are properly configured".to_string());
        }
        
        recommendations
    }
    
    /// Enhanced build metrics collection
    fn collect_enhanced_build_metrics(&self, result: &BuildResult) -> BuildPerformanceReport {
        let performance_recommendations = self.analyze_build_performance(&result.statistics);
        
        // Calculate build efficiency score
        let efficiency_score = self.calculate_build_efficiency_score(&result.statistics);
        
        // Analyze resource utilization
        let resource_analysis = self.analyze_resource_utilization(&result.statistics);
        
        BuildPerformanceReport {
            total_duration: result.duration,
            efficiency_score,
            performance_recommendations,
            resource_analysis,
            phase_breakdown: result.statistics.phase_timings.clone(),
            cache_effectiveness: self.calculate_cache_effectiveness(&result.statistics),
            bottleneck_analysis: self.identify_build_bottlenecks(&result.statistics),
        }
    }
    
    /// Calculate build efficiency score (0.0 to 1.0)
    fn calculate_build_efficiency_score(&self, statistics: &BuildStatistics) -> f64 {
        let mut score = 1.0;
        
        // Factor in cache utilization
        let cache_ratio = if statistics.files_compiled + statistics.files_cached > 0 {
            statistics.files_cached as f64 / (statistics.files_compiled + statistics.files_cached) as f64
        } else {
            0.0
        };
        score *= 0.3 + (cache_ratio * 0.7); // Weight cache usage heavily
        
        // Factor in memory efficiency (penalize excessive memory usage)
        let memory_efficiency = if statistics.peak_memory > 0 {
            1.0 - ((statistics.peak_memory as f64 / (4.0 * 1024.0 * 1024.0 * 1024.0)).min(1.0)) // Normalize against 4GB
        } else {
            1.0
        };
        score *= 0.7 + (memory_efficiency * 0.3);
        
        // Factor in compilation speed (lines per second)
        let total_time_secs = statistics.phase_timings.values()
            .sum::<Duration>()
            .as_secs_f64();
        
        if total_time_secs > 0.0 && statistics.lines_compiled > 0 {
            let lines_per_second = statistics.lines_compiled as f64 / total_time_secs;
            let speed_factor = (lines_per_second / 1000.0).min(1.0); // Normalize against 1000 lines/sec
            score *= 0.8 + (speed_factor * 0.2);
        }
        
        score.max(0.0).min(1.0)
    }
    
    /// Analyze resource utilization patterns
    fn analyze_resource_utilization(&self, statistics: &BuildStatistics) -> ResourceUtilizationAnalysis {
        let total_time = statistics.phase_timings.values().sum::<Duration>();
        
        ResourceUtilizationAnalysis {
            memory_peak_mb: statistics.peak_memory / (1024 * 1024),
            memory_efficiency: self.calculate_memory_efficiency(statistics),
            cpu_intensive_phases: self.identify_cpu_intensive_phases(statistics),
            io_intensive_phases: self.identify_io_intensive_phases(statistics),
            parallelization_opportunities: self.identify_parallelization_opportunities(statistics),
            total_build_time: total_time,
        }
    }
    
    /// Calculate cache effectiveness metrics
    fn calculate_cache_effectiveness(&self, statistics: &BuildStatistics) -> CacheEffectivenessMetrics {
        let total_files = statistics.files_compiled + statistics.files_cached;
        let hit_rate = if total_files > 0 {
            statistics.files_cached as f64 / total_files as f64
        } else {
            0.0
        };
        
        CacheEffectivenessMetrics {
            hit_rate,
            files_from_cache: statistics.files_cached,
            total_files,
            estimated_time_saved: Duration::from_secs((statistics.files_cached as u64) * 2), // Estimate 2s saved per cached file
            cache_storage_efficiency: 0.85, // Placeholder - would be calculated from actual cache data
        }
    }
    
    /// Identify build bottlenecks
    fn identify_build_bottlenecks(&self, statistics: &BuildStatistics) -> Vec<BuildBottleneck> {
        let mut bottlenecks = Vec::new();
        let total_time = statistics.phase_timings.values().sum::<Duration>();
        
        for (phase, duration) in &statistics.phase_timings {
            let percentage = if total_time.as_millis() > 0 {
                (duration.as_millis() as f64 / total_time.as_millis() as f64) * 100.0
            } else {
                0.0
            };
            
            if percentage > 25.0 {
                bottlenecks.push(BuildBottleneck {
                    phase: phase.clone(),
                    duration: *duration,
                    percentage,
                    severity: if percentage > 50.0 { 
                        BottleneckSeverity::Critical 
                    } else if percentage > 35.0 { 
                        BottleneckSeverity::High 
                    } else { 
                        BottleneckSeverity::Medium 
                    },
                    recommendations: self.get_bottleneck_recommendations(phase, percentage),
                });
            }
        }
        
        bottlenecks
    }
    
    /// Get specific recommendations for bottlenecks
    fn get_bottleneck_recommendations(&self, phase: &str, percentage: f64) -> Vec<String> {
        match phase {
            "compilation" => vec![
                "Enable parallel compilation with optimal worker count".to_string(),
                "Consider using precompiled headers".to_string(),
                "Enable unity/jumbo builds for faster compilation".to_string(),
            ],
            "linking" => vec![
                "Use incremental linking (--incremental-linker-compatible)".to_string(),
                "Consider link-time optimization (LTO) for release builds only".to_string(),
                "Split large executables into smaller libraries".to_string(),
            ],
            "parsing" => vec![
                "Reduce template instantiation complexity".to_string(),
                "Use forward declarations to reduce parsing overhead".to_string(),
                "Consider module system for better parsing performance".to_string(),
            ],
            _ => vec![
                format!("Optimize {} phase which consumes {:.1}% of build time", phase, percentage),
            ],
        }
    }
    
    /// Helper methods for resource analysis
    fn calculate_memory_efficiency(&self, statistics: &BuildStatistics) -> f64 {
        // Calculate based on peak memory vs files compiled
        if statistics.files_compiled == 0 {
            return 1.0;
        }
        
        let memory_per_file = statistics.peak_memory as f64 / statistics.files_compiled as f64;
        let optimal_memory_per_file = 128.0 * 1024.0 * 1024.0; // 128MB per file is reasonable
        
        (optimal_memory_per_file / memory_per_file).min(1.0)
    }
    
    fn identify_cpu_intensive_phases(&self, statistics: &BuildStatistics) -> Vec<String> {
        let cpu_phases = vec!["compilation", "optimization", "codegen"];
        statistics.phase_timings.keys()
            .filter(|phase| cpu_phases.iter().any(|cpu_phase| phase.contains(cpu_phase)))
            .cloned()
            .collect()
    }
    
    fn identify_io_intensive_phases(&self, statistics: &BuildStatistics) -> Vec<String> {
        let io_phases = vec!["parsing", "linking", "dependency_resolution"];
        statistics.phase_timings.keys()
            .filter(|phase| io_phases.iter().any(|io_phase| phase.contains(io_phase)))
            .cloned()
            .collect()
    }
    
    fn identify_parallelization_opportunities(&self, statistics: &BuildStatistics) -> Vec<String> {
        let mut opportunities = Vec::new();
        
        if let Some(compilation_time) = statistics.phase_timings.get("compilation") {
            if compilation_time.as_secs() > 30 {
                opportunities.push("Compilation phase can benefit from increased parallelism".to_string());
            }
        }
        
        if statistics.files_compiled > 10 {
            opportunities.push("Multiple compilation units can be processed in parallel".to_string());
        }
        
        opportunities
    }
    
    /// Create compilation tasks from build targets
    fn create_compilation_tasks(&self, targets: &[BuildTarget], profile: &str) -> Result<Vec<crate::build_system::CompilationTask>, BuildError> {
        let mut tasks = Vec::new();
        
        for target in targets {
            let task = crate::build_system::CompilationTask {
                id: target.name.clone(),
                target: target.clone(),
                profile: self.config.get_effective_profile(profile)
                    .map_err(|e| BuildError::ConfigError(e.to_string()))?,
                dependencies: target.dependencies.clone(),
                estimated_duration: Duration::from_secs(10), // Placeholder estimation
                memory_requirement: 256 * 1024 * 1024, // 256MB placeholder
                priority: crate::build_system::TaskPriority::Normal,
                compilation_units: Vec::new(), // Would be populated from analysis
            };
            tasks.push(task);
        }
        
        Ok(tasks)
    }
}

impl Drop for BuildOrchestrator {
    fn drop(&mut self) {
        // Ensure file watcher is stopped when orchestrator is dropped
        if self.file_watcher.is_some() {
            let _ = futures::executor::block_on(self.stop_file_watching());
        }
    }
}

/// Extract warnings from compiler output
fn extract_warnings(output: &str) -> Vec<String> {
    output
        .lines()
        .filter(|line| line.contains("warning:") || line.contains("WARNING:"))
        .map(|line| line.to_string())
        .collect()
}

/// Count lines of code in a file
fn count_lines(path: &Path) -> Result<usize, BuildError> {
    let content = std::fs::read_to_string(path)?;
    Ok(content.lines().count())
}

/// Memory monitoring for compilation
pub struct MemoryMonitor {
    start_time: Instant,
    samples: Arc<Mutex<Vec<(Instant, f64)>>>,
    monitoring: Arc<Mutex<bool>>,
}

pub struct MemoryStats {
    pub peak_memory_mb: f64,
    pub average_memory_mb: f64,
    pub duration: Duration,
}

impl MemoryMonitor {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            samples: Arc::new(Mutex::new(Vec::new())),
            monitoring: Arc::new(Mutex::new(false)),
        }
    }
    
    pub fn start_monitoring(&self) {
        {
            let mut monitoring = self.monitoring.lock().unwrap();
            *monitoring = true;
        }
        
        let samples = Arc::clone(&self.samples);
        let monitoring = Arc::clone(&self.monitoring);
        
        thread::spawn(move || {
            use sysinfo::{System, SystemExt, ProcessExt, Pid};
            let mut sys = System::new_all();
            let current_pid = Pid::from(std::process::id() as usize);
            
            while *monitoring.lock().unwrap() {
                sys.refresh_all();
                
                if let Some(process) = sys.process(current_pid) {
                    let memory_mb = process.memory() as f64 / (1024.0 * 1024.0);
                    let now = Instant::now();
                    
                    if let Ok(mut samples) = samples.lock() {
                        samples.push((now, memory_mb));
                    }
                }
                
                thread::sleep(Duration::from_millis(100));
            }
        });
    }
    
    pub fn stop_monitoring(&self) -> MemoryStats {
        {
            let mut monitoring = self.monitoring.lock().unwrap();
            *monitoring = false;
        }
        
        thread::sleep(Duration::from_millis(150)); // Let monitoring thread finish
        
        let samples = self.samples.lock().unwrap();
        let duration = self.start_time.elapsed();
        
        if samples.is_empty() {
            return MemoryStats {
                peak_memory_mb: 0.0,
                average_memory_mb: 0.0,
                duration,
            };
        }
        
        let peak_memory_mb = samples.iter().map(|(_, mem)| *mem).fold(0.0, f64::max);
        let average_memory_mb = samples.iter().map(|(_, mem)| *mem).sum::<f64>() / samples.len() as f64;
        
        MemoryStats {
            peak_memory_mb,
            average_memory_mb,
            duration,
        }
    }
}

/// Extract compilation metrics from compiler output
fn extract_compilation_metrics(output: &str) -> HashMap<String, f64> {
    let mut metrics = HashMap::new();
    
    for line in output.lines() {
        if line.starts_with("METRIC:") {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 3 {
                let metric_name = parts[1].trim();
                if let Ok(value) = parts[2].trim().parse::<f64>() {
                    metrics.insert(metric_name.to_string(), value);
                }
            }
        }
    }
    
    // Parse common patterns from compiler output
    if let Some(lines_match) = output.lines().find(|line| line.contains("lines compiled")) {
        if let Some(num_str) = lines_match.split_whitespace().next() {
            if let Ok(lines) = num_str.parse::<f64>() {
                metrics.insert("lines_compiled".to_string(), lines);
            }
        }
    }
    
    // Parse timing information from verbose output
    for line in output.lines() {
        if line.contains("parsing took") {
            extract_time_from_line(line, "parsing_time", &mut metrics);
        } else if line.contains("analysis took") {
            extract_time_from_line(line, "analysis_time", &mut metrics);
        } else if line.contains("codegen took") {
            extract_time_from_line(line, "codegen_time", &mut metrics);
        } else if line.contains("linking took") {
            extract_time_from_line(line, "linking_time", &mut metrics);
        }
    }
    
    metrics
}

/// Extract timing information from a compiler output line
fn extract_time_from_line(line: &str, metric_name: &str, metrics: &mut HashMap<String, f64>) {
    use regex::Regex;
    
    if let Ok(time_regex) = Regex::new(r"(\d+(?:\.\d+)?)\s*(ms|s)") {
        if let Some(captures) = time_regex.captures(line) {
            if let (Some(value_match), Some(unit_match)) = (captures.get(1), captures.get(2)) {
                if let Ok(value) = value_match.as_str().parse::<f64>() {
                    let time_ms = match unit_match.as_str() {
                        "s" => value * 1000.0,
                        "ms" => value,
                        _ => value,
                    };
                    metrics.insert(metric_name.to_string(), time_ms);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::build_system::{BuildConfig, ProjectType};
    
    #[tokio::test]
    async fn test_build_orchestrator_creation() {
        let config = BuildConfig::default_for_project("test", ProjectType::Binary);
        let work_dir = tempdir().unwrap().into_path();
        
        let orchestrator = BuildOrchestrator::new(config, work_dir);
        assert!(orchestrator.is_ok());
    }
    
    #[test]
    fn test_warning_extraction() {
        let output = "
            info: compilation started
            warning: unused variable `x`
            ERROR: compilation failed
            WARNING: deprecated function used
        ";
        
        let warnings = extract_warnings(output);
        assert_eq!(warnings.len(), 2);
        assert!(warnings[0].contains("unused variable"));
        assert!(warnings[1].contains("deprecated function"));
    }
    
    #[test]
    fn test_line_counting() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let file_path = dir.path().join("test.csd");
        
        std::fs::write(&file_path, "line 1\nline 2\nline 3\n")?;
        
        let count = count_lines(&file_path)?;
        assert_eq!(count, 3);
        
        Ok(())
    }
}
