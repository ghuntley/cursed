//! Build Orchestrator for Multi-file CURSED Projects
//!
//! This module provides high-level coordination of the build process,
//! managing multiple build pipelines, parallel compilation, and
//! workspace-level operations.

use crate::error::{CursedError, Result};
use crate::build_system::build_pipeline::{BuildPipeline, BuildConfig, BuildResult, BuildMode};
use crate::build_system::project_template_simple::{ProjectTemplate, ProjectConfig};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use futures::future::select_all;

/// Workspace configuration for multi-project builds
#[derive(Debug, Clone)]
pub struct WorkspaceConfig {
    /// Root directory of the workspace
    pub root: PathBuf,
    /// Projects in the workspace
    pub projects: Vec<ProjectConfig>,
    /// Global build settings
    pub global_config: BuildConfig,
    /// Parallel job limit
    pub max_jobs: usize,
    /// Workspace-level dependencies
    pub dependencies: HashMap<String, String>,
}

/// Build orchestrator managing multiple projects
pub struct BuildOrchestrator {
    /// Workspace configuration
    workspace: WorkspaceConfig,
    /// Active build pipelines
    pipelines: HashMap<String, Arc<RwLock<BuildPipeline>>>,
    /// Build results cache
    results_cache: HashMap<String, BuildResult>,
}

/// Build strategy for project compilation
#[derive(Debug, Clone, Copy)]
pub enum BuildStrategy {
    /// Sequential compilation
    Sequential,
    /// Parallel compilation within dependencies
    Parallel,
    /// Distributed compilation
    Distributed,
}

/// Build target specification
#[derive(Debug, Clone)]
pub struct BuildTarget {
    /// Target name
    pub name: String,
    /// Project path
    pub project_path: PathBuf,
    /// Build configuration
    pub config: BuildConfig,
    /// Dependencies on other targets
    pub dependencies: Vec<String>,
}

impl BuildOrchestrator {
    /// Create a new build orchestrator
    pub fn new(workspace: WorkspaceConfig) -> Self {
        Self {
            workspace,
            pipelines: HashMap::new(),
            results_cache: HashMap::new(),
        }
    }
    
    /// Helper constructor that accepts BuildConfig and converts it to WorkspaceConfig
    pub fn from_build_config(config: BuildConfig, work_dir: PathBuf) -> Result<Self> {
        let workspace_config = WorkspaceConfig {
            root: work_dir,
            projects: vec![],
            global_config: config,
            max_jobs: 4,
            dependencies: HashMap::new(),
        };
        
        Ok(Self::new(workspace_config))
    }

    /// Initialize orchestrator from workspace directory
    pub async fn from_workspace<P: AsRef<Path>>(workspace_root: P) -> Result<Self> {
        let workspace_root = workspace_root.as_ref().to_path_buf();
        let workspace_config = Self::discover_workspace(&workspace_root).await?;
        
        let mut orchestrator = Self::new(workspace_config);
        orchestrator.initialize_pipelines().await?;
        
        Ok(orchestrator)
    }

    /// Discover workspace configuration
    async fn discover_workspace(root: &Path) -> Result<WorkspaceConfig> {
        let mut projects = Vec::new();
        let mut global_config = BuildConfig::default();
        global_config.project_root = root.to_path_buf();

        // Look for workspace configuration file
        let workspace_file = root.join("CursedWorkspace.toml");
        if workspace_file.exists() {
            let content = std::fs::read_to_string(workspace_file)?;
            let workspace_toml: toml::Value = toml::from_str(&content).map_err(|e| CursedError::General(format!("Failed to parse workspace config: {}", e)))?;
            
            if let Some(members) = workspace_toml.get("members") {
                if let Some(member_array) = members.as_array() {
                    for member in member_array {
                        if let Some(path_str) = member.as_str() {
                            let project_path = root.join(path_str);
                            if let Ok(project_config) = ProjectTemplate::load_project_config(&project_path) {
                                projects.push(project_config);
                            }
                        }
                    }
                }
            }
        } else {
            // Single project workspace
            if let Ok(project_config) = ProjectTemplate::load_project_config(root) {
                projects.push(project_config);
            }
        }

        // Default to current directory if no projects found
        if projects.is_empty() {
            projects.push(ProjectConfig {
                name: "main".to_string(),
                version: "0.1.0".to_string(),
                root: root.to_path_buf(),
                source_dirs: vec![PathBuf::from("src")],
                main_file: None,
                dependencies: HashMap::new(),
                build_config: BuildConfig::default(),
            });
        }

        Ok(WorkspaceConfig {
            root: root.to_path_buf(),
            projects,
            global_config,
            max_jobs: num_cpus::get(),
            dependencies: HashMap::new(),
        })
    }

    /// Initialize build pipelines for all projects
    async fn initialize_pipelines(&mut self) -> Result<()> {
        for project in &self.workspace.projects {
            let mut config = project.build_config.clone();
            config.project_root = project.root.clone();
            config.source_dirs = project.source_dirs.clone();
            config.main_file = project.main_file.clone();

            let pipeline = BuildPipeline::new(config)?;
            self.pipelines.insert(
                project.name.clone(),
                Arc::new(RwLock::new(pipeline))
            );
        }
        
        Ok(())
    }

    /// Build all projects in the workspace
    pub async fn build_workspace(&mut self, strategy: BuildStrategy) -> Result<HashMap<String, BuildResult>> {
        let start_time = Instant::now();
        
        println!("🏗️  Building workspace with {} projects", self.workspace.projects.len());
        println!("📊 Strategy: {:?}, Max jobs: {}", strategy, self.workspace.max_jobs);
        
        let results = match strategy {
            BuildStrategy::Sequential => self.build_sequential().await?,
            BuildStrategy::Parallel => self.build_parallel().await?,
            BuildStrategy::Distributed => self.build_distributed().await?,
        };
        
        let duration = start_time.elapsed();
        let success_count = results.values().filter(|r| r.success).count();
        let total_count = results.len();
        
        println!("🎯 Workspace build completed in {:?}", duration);
        println!("📈 Success rate: {}/{} projects", success_count, total_count);
        
        Ok(results)
    }

    /// Build projects sequentially
    async fn build_sequential(&mut self) -> Result<HashMap<String, BuildResult>> {
        let mut results = HashMap::new();
        let build_order = self.resolve_build_order()?;
        
        for project_name in build_order {
            println!("🔨 Building project: {}", project_name);
            
            if let Some(pipeline) = self.pipelines.get(&project_name) {
                let mut pipeline = pipeline.write().await;
                let result = pipeline.build().await?;
                results.insert(project_name.clone(), result);
            }
        }
        
        Ok(results)
    }

    /// Build projects in parallel where possible
    async fn build_parallel(&mut self) -> Result<HashMap<String, BuildResult>> {
        let mut results = HashMap::new();
        let build_order = self.resolve_build_order()?;
        let mut handles: Vec<JoinHandle<Result<(String, BuildResult)>>> = Vec::new();
        let mut active_jobs = 0;
        let mut completed = std::collections::HashSet::new();
        
        for project_name in build_order {
            // Wait for dependencies to complete
            let project = self.workspace.projects.iter()
                .find(|p| p.name == project_name)
                .unwrap();
            
            let dependencies_ready = project.dependencies.keys()
                .all(|dep| completed.contains(dep));
            
            if dependencies_ready && active_jobs < self.workspace.max_jobs {
                // Start build job
                if let Some(pipeline) = self.pipelines.get(&project_name) {
                    let pipeline = pipeline.clone();
                    let name = project_name.clone();
                    
                    let handle = tokio::spawn(async move {
                        let mut pipeline = pipeline.write().await;
                        let result = pipeline.build().await?;
                        Ok((name, result))
                    });
                    
                    handles.push(handle);
                    active_jobs += 1;
                }
            } else {
                // Wait for some jobs to complete
                if !handles.is_empty() {
                    let (result, _index, remaining) = select_all(handles).await;
                    handles = remaining;
                    active_jobs -= 1;
                    
                    match result {
                        Ok(Ok((name, build_result))) => {
                            completed.insert(name.clone());
                            results.insert(name, build_result);
                        }
                        Ok(Err(e)) => return Err(e),
                        Err(e) => return Err(CursedError::CompilerError(format!("Build job failed: {}", e))),
                    }
                }
            }
        }
        
        // Wait for remaining jobs
        while !handles.is_empty() {
            let (result, _index, remaining) = select_all(handles).await;
            handles = remaining;
            
            match result {
                Ok(Ok((name, build_result))) => {
                    completed.insert(name.clone());
                    results.insert(name, build_result);
                }
                Ok(Err(e)) => return Err(e),
                Err(e) => return Err(CursedError::CompilerError(format!("Build job failed: {}", e))),
            }
        }
        
        Ok(results)
    }

    /// Build projects in distributed mode (placeholder)
    async fn build_distributed(&mut self) -> Result<HashMap<String, BuildResult>> {
        println!("⚠️  Distributed build not yet implemented, falling back to parallel");
        self.build_parallel().await
    }

    /// Resolve build order based on project dependencies
    fn resolve_build_order(&self) -> Result<Vec<String>> {
        let mut order = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut visiting = std::collections::HashSet::new();
        
        for project in &self.workspace.projects {
            if !visited.contains(&project.name) {
                self.visit_project(&project.name, &mut visited, &mut visiting, &mut order)?;
            }
        }
        
        Ok(order)
    }

    /// Visit a project during topological sort
    fn visit_project(
        &self,
        name: &str,
        visited: &mut std::collections::HashSet<String>,
        visiting: &mut std::collections::HashSet<String>,
        order: &mut Vec<String>,
    ) -> Result<()> {
        if visiting.contains(name) {
            return Err(CursedError::CompilerError(format!("Circular dependency detected: {}", name)));
        }
        
        if visited.contains(name) {
            return Ok(());
        }
        
        visiting.insert(name.to_string());
        
        if let Some(project) = self.workspace.projects.iter().find(|p| p.name == name) {
            for dep in project.dependencies.keys() {
                if self.workspace.projects.iter().any(|p| p.name == *dep) {
                    self.visit_project(dep, visited, visiting, order)?;
                }
            }
        }
        
        visiting.remove(name);
        visited.insert(name.to_string());
        order.push(name.to_string());
        
        Ok(())
    }

    /// Build a specific project
    pub async fn build_project(&mut self, project_name: &str) -> Result<BuildResult> {
        if let Some(pipeline) = self.pipelines.get(project_name) {
            let mut pipeline = pipeline.write().await;
            let result = pipeline.build().await?;
            self.results_cache.insert(project_name.to_string(), result.clone());
            Ok(result)
        } else {
            Err(CursedError::CompilerError(format!("Project not found: {}", project_name)))
        }
    }

    /// Clean all projects
    pub async fn clean_workspace(&self) -> Result<()> {
        println!("🧹 Cleaning workspace...");
        
        for (name, pipeline) in &self.pipelines {
            println!("🧹 Cleaning project: {}", name);
            let pipeline = pipeline.read().await;
            pipeline.clean()?;
        }
        
        println!("✅ Workspace cleaned");
        Ok(())
    }

    /// Get build status for all projects
    pub fn get_build_status(&self) -> HashMap<String, bool> {
        self.results_cache.iter()
            .map(|(name, result)| (name.clone(), result.success))
            .collect()
    }

    /// Create a new project in the workspace
    pub async fn create_project(&mut self, name: &str, template: &str) -> Result<()> {
        let project_path = self.workspace.root.join(name);
        
        // Create project from template
        let project_template = ProjectTemplate::new(template.to_string());
        let project_config = project_template.create_project(&project_path, name)?;
        
        // Add to workspace
        self.workspace.projects.push(project_config.clone());
        
        // Initialize pipeline
        let mut config = project_config.build_config.clone();
        config.project_root = project_config.root.clone();
        config.source_dirs = project_config.source_dirs.clone();
        config.main_file = project_config.main_file.clone();
        
        let pipeline = BuildPipeline::new(config)?;
        self.pipelines.insert(name.to_string(), Arc::new(RwLock::new(pipeline)));
        
        println!("✅ Created project: {}", name);
        Ok(())
    }

    /// Add dependency between projects
    pub async fn add_project_dependency(&mut self, project: &str, dependency: &str) -> Result<()> {
        if let Some(proj) = self.workspace.projects.iter_mut().find(|p| p.name == project) {
            proj.dependencies.insert(dependency.to_string(), "*".to_string());
            println!("✅ Added dependency: {} -> {}", project, dependency);
            Ok(())
        } else {
            Err(CursedError::CompilerError(format!("Project not found: {}", project)))
        }
    }

    /// Run tests for all projects
    pub async fn test_workspace(&mut self) -> Result<HashMap<String, BuildResult>> {
        println!("🧪 Running tests for workspace...");
        
        let mut results = HashMap::new();
        
        for project in &self.workspace.projects {
            if let Some(pipeline) = self.pipelines.get(&project.name) {
                let mut pipeline = pipeline.write().await;
                
                // Build in test mode
                // TODO: Implement proper test mode
                let result = pipeline.build().await?;
                results.insert(project.name.clone(), result);
            }
        }
        
        Ok(results)
    }

    /// Get workspace statistics
    pub fn get_workspace_stats(&self) -> WorkspaceStats {
        let project_count = self.workspace.projects.len();
        let successful_builds = self.results_cache.values().filter(|r| r.success).count();
        let failed_builds = self.results_cache.values().filter(|r| !r.success).count();
        
        let total_warnings = self.results_cache.values()
            .map(|r| r.warnings.len())
            .sum();
        
        let total_errors = self.results_cache.values()
            .map(|r| r.errors.len())
            .sum();
        
        WorkspaceStats {
            project_count,
            successful_builds,
            failed_builds,
            total_warnings,
            total_errors,
        }
    }
}

/// Workspace build statistics
#[derive(Debug, Clone)]
pub struct WorkspaceStats {
    pub project_count: usize,
    pub successful_builds: usize,
    pub failed_builds: usize,
    pub total_warnings: usize,
    pub total_errors: usize,
}

impl std::fmt::Display for WorkspaceStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Workspace Stats: {} projects, {} successful, {} failed, {} warnings, {} errors",
               self.project_count, self.successful_builds, self.failed_builds, 
               self.total_warnings, self.total_errors)
    }
}

/// Workspace configuration builder
pub struct WorkspaceConfigBuilder {
    config: WorkspaceConfig,
}

impl WorkspaceConfigBuilder {
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        Self {
            config: WorkspaceConfig {
                root: root.as_ref().to_path_buf(),
                projects: Vec::new(),
                global_config: BuildConfig::default(),
                max_jobs: num_cpus::get(),
                dependencies: HashMap::new(),
            },
        }
    }

    pub fn add_project(mut self, project: ProjectConfig) -> Self {
        self.config.projects.push(project);
        self
    }

    pub fn max_jobs(mut self, jobs: usize) -> Self {
        self.config.max_jobs = jobs;
        self
    }

    pub fn build(self) -> WorkspaceConfig {
        self.config
    }
}
