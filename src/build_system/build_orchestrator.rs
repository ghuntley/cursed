//! Build Orchestrator
//! 
//! Coordinates the build process, manages dependencies, integrates with toolchain,
//! and provides build caching and incremental compilation.

use crate::build_system::{
    BuildConfig, BuildTarget, BuildProfile, IncrementalCache, DependencyResolver,
    TargetType, OptimizationLevel
};
use crate::build_system::build_pipeline::{BuildPipeline, PipelineContext, PipelineResult};
use crate::build_system::incremental_cache::CacheError;
use crate::package_manager::{PackageManager, PackageManagerError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use tracing::{debug, error, info, warn, instrument};

/// Main build orchestrator
#[derive(Debug)]
pub struct BuildOrchestrator {
    config: BuildConfig,
    cache: IncrementalCache,
    dependency_resolver: DependencyResolver,
    package_manager: PackageManager,
    pipeline: BuildPipeline,
    work_dir: PathBuf,
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
        
        // Convert pipeline result to build result
        Ok(BuildResult {
            success: pipeline_result.success,
            duration: pipeline_result.duration,
            targets_built: pipeline_result.stages.keys().cloned().collect(),
            targets_skipped: Vec::from([]),
            outputs: pipeline_result.artifacts.values().cloned().collect(),
            artifacts: pipeline_result.artifacts,
            warnings: pipeline_result.warnings,
            statistics: BuildStatistics {
                files_compiled: pipeline_result.statistics.stages_executed,
                files_cached: pipeline_result.statistics.stages_cached,
                lines_compiled: 0, // TODO: Extract from pipeline
                peak_memory: pipeline_result.statistics.resource_usage.peak_memory,
                phase_timings: HashMap::new(), // TODO: Extract from pipeline
            },
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
        
        // TODO: Implement file watching with notify crate
        // For now, just return an error indicating it's not implemented
        Err(BuildError::ToolError("File watching not yet implemented".to_string()))
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
        
        // Determine output path
        let output_dir = self.work_dir.join("target").join("debug"); // TODO: Use profile name
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
        
        // Build compiler command
        let mut cmd = Command::new("./target/debug/cursed");
        cmd.arg("compile")
           .arg(&target.path)
           .arg("--output")
           .arg(&output_path);
        
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
        let output = cmd.stdout(Stdio::piped())
                       .stderr(Stdio::piped())
                       .spawn()?
                       .wait_with_output()?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(BuildError::CompilationError(format!(
                "Compilation failed for target '{}': {}", target.name, stderr
            )));
        }
        
        // Parse compilation output for warnings
        let stdout = String::from_utf8_lossy(&output.stdout);
        let warnings = extract_warnings(&stdout);
        
        let duration = start_time.elapsed();
        
        // Create build result
        let mut artifacts = HashMap::new();
        artifacts.insert(target.name.clone(), output_path.clone());
        
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
                lines_compiled: count_lines(&target.path)?,
                peak_memory: 0, // TODO: Implement memory monitoring
                phase_timings: {
                    let mut timings = HashMap::new();
                    timings.insert("compilation".to_string(), duration);
                    timings
                },
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
    
    /// Run tests
    #[instrument(skip(self))]
    pub async fn test(&mut self, profile: &str) -> Result<BuildResult, BuildError> {
        info!("Running tests with profile: {}", profile);
        
        // Build first if needed
        self.build_all(profile).await?;
        
        // Run test executable if it exists
        let test_executable = self.work_dir.join("target").join("debug").join("test");
        if test_executable.exists() {
            let output = Command::new(&test_executable)
                .output()?;
            
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(BuildError::CompilationError(format!(
                    "Tests failed: {}", stderr
                )));
            }
        }
        
        // TODO: Implement proper test discovery and execution
        Ok(BuildResult {
            success: true,
            duration: Duration::from_millis(0),
            targets_built: Vec::from(["tests".to_string()]),
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
