//! Build Pipeline System
//! 
//! Comprehensive build pipeline that orchestrates all build phases including
//! formatting, linting, compilation, testing, documentation, and packaging.

use crate::build_system::{
    BuildConfig, BuildTarget, BuildProfile, BuildResult, BuildError, 
    BuildStatistics, IncrementalCache
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::task::JoinHandle;
use tracing::{debug, error, info, warn, instrument};

/// Build pipeline coordinator
#[derive(Debug)]
pub struct BuildPipeline {
    config: BuildConfig,
    work_dir: PathBuf,
    cache: Arc<Mutex<IncrementalCache>>,
    parallel_limit: usize,
}

/// Pipeline execution context
#[derive(Debug, Clone)]
pub struct PipelineContext {
    pub profile: String,
    pub targets: Vec<String>,
    pub work_dir: PathBuf,
    pub force_rebuild: bool,
    pub parallel: bool,
    pub verbose: bool,
}

/// Pipeline stage definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStage {
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub parallel: bool,
    pub dependencies: Vec<String>,
    pub timeout: Option<Duration>,
    pub retry_count: u32,
}

/// Pipeline execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineResult {
    pub success: bool,
    pub duration: Duration,
    pub stages: HashMap<String, StageResult>,
    pub statistics: PipelineStatistics,
    pub artifacts: HashMap<String, PathBuf>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

/// Individual stage result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageResult {
    pub name: String,
    pub success: bool,
    pub duration: Duration,
    pub output: Vec<String>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
    pub artifacts: Vec<PathBuf>,
    pub cache_hit: bool,
}

/// Pipeline execution statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStatistics {
    pub total_duration: Duration,
    pub stages_executed: usize,
    pub stages_cached: usize,
    pub stages_failed: usize,
    pub parallel_efficiency: f64,
    pub cache_hit_rate: f64,
    pub resource_usage: ResourceUsage,
}

/// Resource usage tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub peak_memory: usize,
    pub cpu_time: Duration,
    pub disk_reads: usize,
    pub disk_writes: usize,
}

impl BuildPipeline {
    /// Create new build pipeline
    pub fn new(config: BuildConfig, work_dir: PathBuf) -> Result<Self, BuildError> {
        let cache = Arc::new(Mutex::new(IncrementalCache::new(work_dir.join(".cursed-cache"))?));
        let parallel_limit = num_cpus::get();
        
        Ok(Self {
            config,
            work_dir,
            cache,
            parallel_limit,
        })
    }
    
    /// Execute complete build pipeline
    #[instrument(skip(self))]
    pub async fn execute(&mut self, context: PipelineContext) -> Result<PipelineResult, BuildError> {
        let start_time = Instant::now();
        info!("Starting build pipeline with profile: {}", context.profile);
        
        let stages = self.create_pipeline_stages(&context)?;
        let mut results = HashMap::new();
        let mut statistics = PipelineStatistics::default();
        let mut warnings = Vec::new();
        let mut errors = Vec::new();
        let mut artifacts = HashMap::new();
        
        // Execute stages in dependency order
        let execution_order = self.resolve_stage_dependencies(&stages)?;
        
        for stage_batch in execution_order {
            if context.parallel && stage_batch.len() > 1 {
                // Execute batch in parallel
                let batch_results = self.execute_parallel_batch(&stage_batch, &context).await?;
                for (name, result) in batch_results {
                    self.process_stage_result(&mut results, &mut warnings, &mut errors, 
                                             &mut artifacts, &mut statistics, name, result);
                }
            } else {
                // Execute sequentially
                for stage in stage_batch {
                    let result = self.execute_stage(&stage, &context).await?;
                    let should_continue = result.success || self.should_continue_on_failure(&stage);
                    let stage_name = stage.name.clone();
                    
                    self.process_stage_result(&mut results, &mut warnings, &mut errors, 
                                             &mut artifacts, &mut statistics, stage_name.clone(), result);
                    
                    // Stop on failure if not configured to continue
                    if !should_continue {
                        errors.push(format!("Pipeline stopped due to failure in stage: {}", stage_name));
                        break;
                    }
                }
            }
        }
        
        let total_duration = start_time.elapsed();
        statistics.total_duration = total_duration;
        
        let success = errors.is_empty() && !results.values().any(|r| !r.success);
        
        info!("Build pipeline completed in {:?}, success: {}", total_duration, success);
        
        Ok(PipelineResult {
            success,
            duration: total_duration,
            stages: results,
            statistics,
            artifacts,
            warnings,
            errors,
        })
    }
    
    /// Create pipeline stages based on configuration and context
    fn create_pipeline_stages(&self, context: &PipelineContext) -> Result<Vec<PipelineStage>, BuildError> {
        let mut stages = Vec::new();
        
        // 1. Dependency Resolution Stage
        stages.push(PipelineStage {
            name: "dependency_resolution".to_string(),
            description: "Resolve and fetch dependencies".to_string(),
            enabled: true,
            parallel: false,
            dependencies: vec![],
            timeout: Some(Duration::from_secs(300)),
            retry_count: 2,
        });
        
        // 2. Code Formatting Stage (if enabled)
        if self.config.tools.formatter.format_on_build {
            stages.push(PipelineStage {
                name: "format".to_string(),
                description: "Format source code".to_string(),
                enabled: true,
                parallel: true,
                dependencies: vec!["dependency_resolution".to_string()],
                timeout: Some(Duration::from_secs(60)),
                retry_count: 1,
            });
        }
        
        // 3. Code Linting Stage (if enabled)
        if self.config.tools.linter.lint_on_build {
            stages.push(PipelineStage {
                name: "lint".to_string(),
                description: "Lint source code".to_string(),
                enabled: true,
                parallel: true,
                dependencies: if self.config.tools.formatter.format_on_build {
                    vec!["format".to_string()]
                } else {
                    vec!["dependency_resolution".to_string()]
                },
                timeout: Some(Duration::from_secs(120)),
                retry_count: 1,
            });
        }
        
        // 4. Compilation Stage
        let lint_deps = if self.config.tools.linter.lint_on_build {
            vec!["lint".to_string()]
        } else if self.config.tools.formatter.format_on_build {
            vec!["format".to_string()]
        } else {
            vec!["dependency_resolution".to_string()]
        };
        
        stages.push(PipelineStage {
            name: "compile".to_string(),
            description: "Compile source code".to_string(),
            enabled: true,
            parallel: true,
            dependencies: lint_deps,
            timeout: Some(Duration::from_secs(600)),
            retry_count: 1,
        });
        
        // 5. Testing Stage
        stages.push(PipelineStage {
            name: "test".to_string(),
            description: "Run tests".to_string(),
            enabled: true,
            parallel: true,
            dependencies: vec!["compile".to_string()],
            timeout: Some(Duration::from_secs(300)),
            retry_count: 1,
        });
        
        // 6. Documentation Generation Stage (if enabled)
        if self.config.tools.docs.generate_on_build {
            stages.push(PipelineStage {
                name: "docs".to_string(),
                description: "Generate documentation".to_string(),
                enabled: true,
                parallel: false,
                dependencies: vec!["compile".to_string()],
                timeout: Some(Duration::from_secs(180)),
                retry_count: 1,
            });
        }
        
        // 7. Packaging Stage (for release builds)
        if context.profile == "release" {
            stages.push(PipelineStage {
                name: "package".to_string(),
                description: "Create distribution packages".to_string(),
                enabled: true,
                parallel: false,
                dependencies: if self.config.tools.docs.generate_on_build {
                    vec!["test".to_string(), "docs".to_string()]
                } else {
                    vec!["test".to_string()]
                },
                timeout: Some(Duration::from_secs(120)),
                retry_count: 1,
            });
        }
        
        Ok(stages)
    }
    
    /// Resolve stage dependencies into execution batches
    fn resolve_stage_dependencies(&self, stages: &[PipelineStage]) -> Result<Vec<Vec<PipelineStage>>, BuildError> {
        let mut batches = Vec::new();
        let mut completed = std::collections::HashSet::new();
        let stage_map: HashMap<String, &PipelineStage> = stages.iter()
            .map(|s| (s.name.clone(), s))
            .collect();
        
        while completed.len() < stages.len() {
            let mut current_batch = Vec::new();
            
            for stage in stages {
                if completed.contains(&stage.name) {
                    continue;
                }
                
                // Check if all dependencies are completed
                let deps_ready = stage.dependencies.iter()
                    .all(|dep| completed.contains(dep));
                
                if deps_ready {
                    current_batch.push(stage.clone());
                }
            }
            
            if current_batch.is_empty() {
                return Err(BuildError::ConfigError(
                    "Circular dependency detected in pipeline stages".to_string()
                ));
            }
            
            for stage in &current_batch {
                completed.insert(stage.name.clone());
            }
            
            batches.push(current_batch);
        }
        
        Ok(batches)
    }
    
    /// Execute multiple stages in parallel
    async fn execute_parallel_batch(
        &self,
        stages: &[PipelineStage],
        context: &PipelineContext,
    ) -> Result<HashMap<String, StageResult>, BuildError> {
        let mut handles: Vec<JoinHandle<Result<(String, StageResult), BuildError>>> = Vec::new();
        
        for stage in stages {
            let stage_clone = stage.clone();
            let context_clone = context.clone();
            let self_clone = self.clone(); // This would require Clone implementation
            
            let handle = tokio::spawn(async move {
                let result = self_clone.execute_stage(&stage_clone, &context_clone).await?;
                Ok((stage_clone.name, result))
            });
            
            handles.push(handle);
        }
        
        let mut results = HashMap::new();
        
        for handle in handles {
            match handle.await {
                Ok(Ok((name, result))) => {
                    results.insert(name, result);
                }
                Ok(Err(e)) => return Err(e),
                Err(e) => return Err(BuildError::ToolError(format!("Async execution error: {}", e))),
            }
        }
        
        Ok(results)
    }
    
    /// Execute individual pipeline stage
    #[instrument(skip(self))]
    pub async fn execute_stage(&self, stage: &PipelineStage, context: &PipelineContext) -> Result<StageResult, BuildError> {
        let start_time = Instant::now();
        info!("Executing stage: {}", stage.name);
        
        // Check cache first
        let cache_key = self.generate_cache_key(stage, context);
        let cache_hit = if let Ok(cache) = self.cache.lock() {
            cache.get(&cache_key).is_some()
        } else {
            false
        };
        
        if cache_hit && !context.force_rebuild {
            debug!("Cache hit for stage: {}", stage.name);
            return Ok(StageResult {
                name: stage.name.clone(),
                success: true,
                duration: Duration::from_millis(0),
                output: vec!["Cache hit".to_string()],
                warnings: vec![],
                errors: vec![],
                artifacts: vec![],
                cache_hit: true,
            });
        }
        
        let mut result = match stage.name.as_str() {
            "dependency_resolution" => self.execute_dependency_resolution(context).await?,
            "format" => self.execute_formatting(context).await?,
            "lint" => self.execute_linting(context).await?,
            "compile" => self.execute_compilation(context).await?,
            "test" => self.execute_testing(context).await?,
            "docs" => self.execute_documentation(context).await?,
            "package" => self.execute_packaging(context).await?,
            _ => return Err(BuildError::ConfigError(format!("Unknown stage: {}", stage.name))),
        };
        
        result.duration = start_time.elapsed();
        result.cache_hit = false;
        
        // Update cache on success
        if result.success {
            if let Ok(mut cache) = self.cache.lock() {
                // Convert Vec<PathBuf> to HashMap<String, PathBuf> for artifacts
                let artifacts_map: HashMap<String, PathBuf> = result.artifacts.iter()
                    .enumerate()
                    .map(|(i, path)| (format!("artifact_{}", i), path.clone()))
                    .collect();
                let _ = cache.insert(&cache_key, vec![], artifacts_map, 1);
            }
        }
        
        info!("Stage '{}' completed in {:?}, success: {}", 
              stage.name, result.duration, result.success);
        
        Ok(result)
    }
    
    /// Execute dependency resolution stage
    async fn execute_dependency_resolution(&self, context: &PipelineContext) -> Result<StageResult, BuildError> {
        let mut output = Vec::new();
        let mut warnings = Vec::new();
        let mut errors = Vec::new();
        
        output.push("Resolving dependencies...".to_string());
        
        // Check if package manifest exists
        let package_manifest = context.work_dir.join("CursedPackage.toml");
        if package_manifest.exists() {
            // Use package manager to install dependencies
            let mut cmd = Command::new("./target/debug/cursed-pkg");
            cmd.arg("install")
                .current_dir(&context.work_dir)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());
            
            match cmd.output() {
                Ok(output_result) => {
                    let stdout = String::from_utf8_lossy(&output_result.stdout);
                    let stderr = String::from_utf8_lossy(&output_result.stderr);
                    
                    output.push(stdout.to_string());
                    if !stderr.as_ref().is_empty() {
                        warnings.push(stderr.to_string());
                    }
                    
                    if !output_result.status.success() {
                        errors.push("Package installation failed".to_string());
                    }
                }
                Err(e) => {
                    errors.push(format!("Failed to run package manager: {}", e));
                }
            }
        } else {
            output.push("No package manifest found, skipping dependency resolution".to_string());
        }
        
        Ok(StageResult {
            name: "dependency_resolution".to_string(),
            success: errors.is_empty(),
            duration: Duration::default(), // Will be set by caller
            output,
            warnings,
            errors,
            artifacts: vec![],
            cache_hit: false,
        })
    }
    
    /// Execute code formatting stage
    async fn execute_formatting(&self, context: &PipelineContext) -> Result<StageResult, BuildError> {
        let mut output = Vec::new();
        let mut warnings = Vec::new();
        let mut errors = Vec::new();
        
        output.push("Formatting source code...".to_string());
        
        let mut cmd = Command::new("./target/debug/cursed-fmt");
        cmd.arg("-w")
            .arg(".")
            .current_dir(&context.work_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        
        match cmd.output() {
            Ok(output_result) => {
                let stdout = String::from_utf8_lossy(&output_result.stdout);
                let stderr = String::from_utf8_lossy(&output_result.stderr);
                
                output.push(stdout.to_string());
                if !stderr.as_ref().is_empty() {
                    warnings.push(stderr.to_string());
                }
                
                if !output_result.status.success() {
                    errors.push("Code formatting failed".to_string());
                }
            }
            Err(e) => {
                errors.push(format!("Failed to run formatter: {}", e));
            }
        }
        
        Ok(StageResult {
            name: "format".to_string(),
            success: errors.is_empty(),
            duration: Duration::default(),
            output,
            warnings,
            errors,
            artifacts: vec![],
            cache_hit: false,
        })
    }
    
    /// Execute code linting stage
    async fn execute_linting(&self, context: &PipelineContext) -> Result<StageResult, BuildError> {
        let mut output = Vec::new();
        let mut warnings = Vec::new();
        let mut errors = Vec::new();
        
        output.push("Linting source code...".to_string());
        
        let mut cmd = Command::new("./target/debug/cursed_lint_new");
        cmd.arg(".")
            .current_dir(&context.work_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        
        if self.config.tools.linter.auto_fix {
            cmd.arg("--fix");
        }
        
        match cmd.output() {
            Ok(output_result) => {
                let stdout = String::from_utf8_lossy(&output_result.stdout);
                let stderr = String::from_utf8_lossy(&output_result.stderr);
                
                output.push(stdout.to_string());
                if !stderr.as_ref().is_empty() {
                    warnings.push(stderr.to_string());
                }
                
                // Linting warnings don't fail the build unless configured to do so
                if !output_result.status.success() && self.config.tools.linter.severity == "error" {
                    errors.push("Code linting found errors".to_string());
                }
            }
            Err(e) => {
                errors.push(format!("Failed to run linter: {}", e));
            }
        }
        
        Ok(StageResult {
            name: "lint".to_string(),
            success: errors.is_empty(),
            duration: Duration::default(),
            output,
            warnings,
            errors,
            artifacts: vec![],
            cache_hit: false,
        })
    }
    
    /// Execute compilation stage
    async fn execute_compilation(&self, context: &PipelineContext) -> Result<StageResult, BuildError> {
        let mut output = Vec::new();
        let mut warnings = Vec::new();
        let mut errors = Vec::new();
        let mut artifacts = Vec::new();
        
        output.push("Compiling source code...".to_string());
        
        // Build specific targets or all targets
        let targets_to_build = if context.targets.is_empty() {
            (&self.config.targets).iter().map(|t| t.name.clone()).collect()
        } else {
            context.targets.clone()
        };
        
        for target_name in targets_to_build {
            let target = (&self.config.targets).iter()
                .find(|t| t.name == target_name)
                .ok_or_else(|| BuildError::TargetNotFound(target_name.clone()))?;
            
            output.push(format!("Building target: {}", target.name));
            
            let mut cmd = Command::new("./target/debug/cursed");
            cmd.arg(&target.path)
                .arg("-o")
                .arg(format!("target/{}/{}", context.profile, target.name))
                .current_dir(&context.work_dir)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());
            
            // Add profile-specific flags
            let profile = self.config.get_effective_profile(&context.profile)?;
            match profile.optimization {
                crate::build_system::OptimizationLevel::None => cmd.arg("-O0"),
                crate::build_system::OptimizationLevel::Basic => cmd.arg("-O1"),
                crate::build_system::OptimizationLevel::Max => cmd.arg("-O3"),
                crate::build_system::OptimizationLevel::Size => cmd.arg("-Os"),
            };
            
            if profile.debug {
                cmd.arg("-g");
            }
            
            match cmd.output() {
                Ok(output_result) => {
                    let stdout = String::from_utf8_lossy(&output_result.stdout);
                    let stderr = String::from_utf8_lossy(&output_result.stderr);
                    
                    output.push(stdout.to_string());
                    if !stderr.as_ref().is_empty() {
                        warnings.push(stderr.to_string());
                    }
                    
                    if output_result.status.success() {
                        let artifact_path = context.work_dir.join("target").join(&context.profile).join(&target.name);
                        artifacts.push(artifact_path);
                    } else {
                        errors.push(format!("Compilation failed for target: {}", target.name));
                    }
                }
                Err(e) => {
                    errors.push(format!("Failed to run compiler: {}", e));
                }
            }
        }
        
        Ok(StageResult {
            name: "compile".to_string(),
            success: errors.is_empty(),
            duration: Duration::default(),
            output,
            warnings,
            errors,
            artifacts,
            cache_hit: false,
        })
    }
    
    /// Execute testing stage
    async fn execute_testing(&self, context: &PipelineContext) -> Result<StageResult, BuildError> {
        let mut output = Vec::new();
        let mut warnings = Vec::new();
        let mut errors = Vec::new();
        
        output.push("Running tests...".to_string());
        
        let mut cmd = Command::new("cargo");
        cmd.arg("test")
            .current_dir(&context.work_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        
        if context.profile == "release" {
            cmd.arg("--release");
        }
        
        match cmd.output() {
            Ok(output_result) => {
                let stdout = String::from_utf8_lossy(&output_result.stdout);
                let stderr = String::from_utf8_lossy(&output_result.stderr);
                
                output.push(stdout.to_string());
                if !stderr.as_ref().is_empty() {
                    warnings.push(stderr.to_string());
                }
                
                if !output_result.status.success() {
                    errors.push("Tests failed".to_string());
                }
            }
            Err(e) => {
                errors.push(format!("Failed to run tests: {}", e));
            }
        }
        
        Ok(StageResult {
            name: "test".to_string(),
            success: errors.is_empty(),
            duration: Duration::default(),
            output,
            warnings,
            errors,
            artifacts: vec![],
            cache_hit: false,
        })
    }
    
    /// Execute documentation generation stage
    async fn execute_documentation(&self, context: &PipelineContext) -> Result<StageResult, BuildError> {
        let mut output = Vec::new();
        let mut warnings = Vec::new();
        let mut errors = Vec::new();
        let mut artifacts = Vec::new();
        
        output.push("Generating documentation...".to_string());
        
        let mut cmd = Command::new("./target/debug/cursed-doc");
        cmd.arg("--html")
            .arg("--source")
            .arg("src")
            .arg("--output")
            .arg(&self.config.tools.docs.output_dir)
            .current_dir(&context.work_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        
        match cmd.output() {
            Ok(output_result) => {
                let stdout = String::from_utf8_lossy(&output_result.stdout);
                let stderr = String::from_utf8_lossy(&output_result.stderr);
                
                output.push(stdout.to_string());
                if !stderr.as_ref().is_empty() {
                    warnings.push(stderr.to_string());
                }
                
                if output_result.status.success() {
                    artifacts.push(context.work_dir.join(&self.config.tools.docs.output_dir));
                } else {
                    errors.push("Documentation generation failed".to_string());
                }
            }
            Err(e) => {
                errors.push(format!("Failed to run documentation generator: {}", e));
            }
        }
        
        Ok(StageResult {
            name: "docs".to_string(),
            success: errors.is_empty(),
            duration: Duration::default(),
            output,
            warnings,
            errors,
            artifacts,
            cache_hit: false,
        })
    }
    
    /// Execute packaging stage
    async fn execute_packaging(&self, context: &PipelineContext) -> Result<StageResult, BuildError> {
        let mut output = Vec::new();
        let mut warnings = Vec::new();
        let mut errors = Vec::new();
        let mut artifacts = Vec::new();
        
        output.push("Creating distribution packages...".to_string());
        
        // Create tar.gz package
        let package_name = format!("{}-{}", self.config.project.name, self.config.project.version);
        let package_path = context.work_dir.join("target").join("package").join(format!("{}.tar.gz", package_name));
        
        // Ensure package directory exists
        if let Some(parent) = package_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| BuildError::IoError(e))?;
        }
        
        let mut cmd = Command::new("tar");
        cmd.arg("czf")
            .arg(&package_path)
            .arg("--exclude")
            .arg("target")
            .arg("--exclude")
            .arg(".git")
            .arg(".")
            .current_dir(&context.work_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        
        match cmd.output() {
            Ok(output_result) => {
                if output_result.status.success() {
                    output.push(format!("Package created: {}", package_path.display()));
                    artifacts.push(package_path);
                } else {
                    let stderr = String::from_utf8_lossy(&output_result.stderr);
                    errors.push(format!("Package creation failed: {}", stderr));
                }
            }
            Err(e) => {
                errors.push(format!("Failed to create package: {}", e));
            }
        }
        
        Ok(StageResult {
            name: "package".to_string(),
            success: errors.is_empty(),
            duration: Duration::default(),
            output,
            warnings,
            errors,
            artifacts,
            cache_hit: false,
        })
    }
    
    /// Generate cache key for stage
    pub fn generate_cache_key(&self, stage: &PipelineStage, context: &PipelineContext) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        stage.name.hash(&mut hasher);
        context.profile.hash(&mut hasher);
        context.targets.hash(&mut hasher);
        
        format!("stage_{}_{:x}", stage.name, hasher.finish())
    }
    
    /// Check if pipeline should continue on stage failure
    fn should_continue_on_failure(&self, stage: &PipelineStage) -> bool {
        // Don't continue on compilation or test failures
        matches!(stage.name.as_str(), "format" | "lint" | "docs")
    }
    
    /// Process stage result and update aggregate statistics
    fn process_stage_result(
        &self,
        results: &mut HashMap<String, StageResult>,
        warnings: &mut Vec<String>,
        errors: &mut Vec<String>,
        artifacts: &mut HashMap<String, PathBuf>,
        statistics: &mut PipelineStatistics,
        name: String,
        result: StageResult,
    ) {
        // Update statistics
        statistics.stages_executed += 1;
        if result.cache_hit {
            statistics.stages_cached += 1;
        }
        if !result.success {
            statistics.stages_failed += 1;
        }
        
        // Collect warnings and errors
        warnings.extend(result.warnings.clone());
        errors.extend(result.errors.clone());
        
        // Collect artifacts
        for (i, artifact) in result.artifacts.iter().enumerate() {
            artifacts.insert(format!("{}_{}", name, i), artifact.clone());
        }
        
        results.insert(name, result);
    }
}

impl Clone for BuildPipeline {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            work_dir: self.work_dir.clone(),
            cache: Arc::clone(&self.cache),
            parallel_limit: self.parallel_limit,
        }
    }
}

impl Default for PipelineStatistics {
    fn default() -> Self {
        Self {
            total_duration: Duration::default(),
            stages_executed: 0,
            stages_cached: 0,
            stages_failed: 0,
            parallel_efficiency: 0.0,
            cache_hit_rate: 0.0,
            resource_usage: ResourceUsage::default(),
        }
    }
}

impl Default for ResourceUsage {
    fn default() -> Self {
        Self {
            peak_memory: 0,
            cpu_time: Duration::default(),
            disk_reads: 0,
            disk_writes: 0,
        }
    }
}
