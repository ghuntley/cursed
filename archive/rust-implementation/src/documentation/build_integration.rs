//! Build System Integration for Documentation Generation
//! 
//! Integrates documentation generation with the CURSED build system,
//! providing automatic documentation updates during compilation.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, SystemTime};
use crate::error::CursedError;
use crate::documentation::{DocumentationGenerator, DocConfig};
use crate::documentation::coverage_analyzer::{CoverageAnalyzer, CoverageReportConfig, ReportFormat};

/// Build integration configuration
#[derive(Debug, Clone)]
pub struct BuildIntegrationConfig {
    pub auto_generate: bool,
    pub generate_on_build: bool,
    pub generate_on_test: bool,
    pub check_coverage: bool,
    pub coverage_threshold: f64,
    pub fail_on_low_coverage: bool,
    pub watch_files: bool,
    pub output_formats: Vec<String>,
    pub build_hooks: BuildHooks,
}

/// Build hooks configuration
#[derive(Debug, Clone)]
pub struct BuildHooks {
    pub pre_build: Vec<String>,
    pub post_build: Vec<String>,
    pub pre_doc: Vec<String>,
    pub post_doc: Vec<String>,
    pub on_coverage_failure: Vec<String>,
}

/// Documentation build manager
pub struct DocBuildManager {
    config: BuildIntegrationConfig,
    doc_config: DocConfig,
    last_build_time: Option<SystemTime>,
    file_dependencies: HashMap<PathBuf, SystemTime>,
    build_cache: BuildCache,
}

/// Build cache for incremental documentation generation
#[derive(Debug, Clone)]
pub struct BuildCache {
    pub module_timestamps: HashMap<String, SystemTime>,
    pub generated_files: Vec<PathBuf>,
    pub coverage_data: Option<String>,
}

/// Build result information
#[derive(Debug)]
pub struct BuildResult {
    pub success: bool,
    pub documentation_generated: bool,
    pub coverage_passed: bool,
    pub coverage_percentage: f64,
    pub build_time: Duration,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl DocBuildManager {
    /// Create a new documentation build manager
    pub fn new(config: BuildIntegrationConfig, doc_config: DocConfig) -> Self {
        Self {
            config,
            doc_config,
            last_build_time: None,
            file_dependencies: HashMap::new(),
            build_cache: BuildCache::new(),
        }
    }

    /// Initialize build integration
    pub fn initialize(&mut self) -> Result<(), CursedError> {
        println!("Initializing documentation build integration...");

        // Create output directories
        self.create_output_directories()?;

        // Scan for file dependencies
        self.scan_file_dependencies()?;

        // Execute pre-build hooks
        self.execute_hooks(&self.config.build_hooks.pre_build)?;

        // Generate initial documentation if auto-generate is enabled
        if self.config.auto_generate {
            self.generate_documentation(false)?;
        }

        println!("Documentation build integration initialized");
        Ok(())
    }

    /// Integrate with build process
    pub fn integrate_with_build(&mut self, build_command: &str) -> Result<BuildResult, CursedError> {
        let start_time = SystemTime::now();
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        println!("Integrating documentation with build process...");

        // Check if documentation needs to be regenerated
        let needs_doc_update = self.needs_documentation_update()?;
        
        // Execute pre-build hooks
        if let Err(e) = self.execute_hooks(&self.config.build_hooks.pre_build) {
            errors.push(format!("Pre-build hook failed: {}", e));
        }

        // Generate documentation if needed and configured
        let mut documentation_generated = false;
        if (self.config.generate_on_build && needs_doc_update) || self.config.auto_generate {
            match self.generate_documentation(true) {
                Ok(_) => {
                    documentation_generated = true;
                    println!("Documentation generated successfully");
                }
                Err(e) => {
                    errors.push(format!("Documentation generation failed: {}", e));
                }
            }
        }

        // Execute the main build command
        let build_success = match self.execute_build_command(build_command) {
            Ok(_) => true,
            Err(e) => {
                errors.push(format!("Build command failed: {}", e));
                false
            }
        };

        // Check documentation coverage if enabled
        let mut coverage_passed = true;
        let mut coverage_percentage = 0.0;
        
        if self.config.check_coverage && documentation_generated {
            match self.check_documentation_coverage() {
                Ok((passed, percentage)) => {
                    coverage_passed = passed;
                    coverage_percentage = percentage;
                    
                    if !passed {
                        let message = format!("Documentation coverage {:.1}% below threshold {:.1}%", 
                            percentage, self.config.coverage_threshold);
                        
                        if self.config.fail_on_low_coverage {
                            errors.push(message);
                        } else {
                            warnings.push(message);
                        }
                        
                        // Execute coverage failure hooks
                        if let Err(e) = self.execute_hooks(&self.config.build_hooks.on_coverage_failure) {
                            warnings.push(format!("Coverage failure hook failed: {}", e));
                        }
                    }
                }
                Err(e) => {
                    warnings.push(format!("Coverage check failed: {}", e));
                }
            }
        }

        // Execute post-build hooks
        if let Err(e) = self.execute_hooks(&self.config.build_hooks.post_build) {
            warnings.push(format!("Post-build hook failed: {}", e));
        }

        // Update build cache
        self.update_build_cache()?;

        let build_time = start_time.elapsed().unwrap_or_default();
        let success = build_success && errors.is_empty() && 
                     (!self.config.fail_on_low_coverage || coverage_passed);

        Ok(BuildResult {
            success,
            documentation_generated,
            coverage_passed,
            coverage_percentage,
            build_time,
            errors,
            warnings,
        })
    }

    /// Generate documentation
    pub fn generate_documentation(&mut self, incremental: bool) -> Result<(), CursedError> {
        println!("Generating documentation...");

        // Execute pre-doc hooks
        self.execute_hooks(&self.config.build_hooks.pre_doc)?;

        // Skip generation if incremental and no changes detected
        if incremental && !self.needs_documentation_update()? {
            println!("No changes detected, skipping documentation generation");
            return Ok(());
        }

        // Generate documentation
        let mut generator = DocumentationGenerator::new(None)?;
        generator.generate()?;

        // Execute post-doc hooks
        self.execute_hooks(&self.config.build_hooks.post_doc)?;

        // Update last build time
        self.last_build_time = Some(SystemTime::now());

        println!("Documentation generation completed");
        Ok(())
    }

    /// Check if documentation needs to be updated
    fn needs_documentation_update(&self) -> Result<bool, CursedError> {
        // If no previous build, always update
        let last_build = match self.last_build_time {
            Some(time) => time,
            None => return Ok(true),
        };

        // Check if any source files have been modified since last build
        for source_dir in &self.doc_config.input.source_dirs {
            if self.directory_modified_since(&PathBuf::from(source_dir), last_build)? {
                return Ok(true);
            }
        }

        // Check if config file has been modified
        if Path::new(".cursed-doc.toml").exists() {
            if let Ok(metadata) = fs::metadata(".cursed-doc.toml") {
                if let Ok(modified) = metadata.modified() {
                    if modified > last_build {
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }

    /// Check if directory has been modified since given time
    fn directory_modified_since(&self, dir: &Path, since: SystemTime) -> Result<bool, CursedError> {
        if !dir.exists() {
            return Ok(false);
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                // Check if this file matches our include patterns
                if self.should_check_file(&path) {
                    if let Ok(metadata) = entry.metadata() {
                        if let Ok(modified) = metadata.modified() {
                            if modified > since {
                                return Ok(true);
                            }
                        }
                    }
                }
            } else if path.is_dir() {
                // Recursively check subdirectories
                if !self.should_exclude_directory(&path) {
                    if self.directory_modified_since(&path, since)? {
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }

    /// Check if file should be monitored for changes
    fn should_check_file(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        
        // Check include patterns
        for pattern in &self.doc_config.input.include_patterns {
            if glob::Pattern::new(pattern).unwrap().matches(&path_str) {
                return true;
            }
        }
        
        false
    }

    /// Check if directory should be excluded from monitoring
    fn should_exclude_directory(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        
        // Check exclude patterns
        for pattern in &self.doc_config.input.exclude_patterns {
            if glob::Pattern::new(pattern).unwrap().matches(&path_str) {
                return true;
            }
        }
        
        false
    }

    /// Execute build command
    fn execute_build_command(&self, command: &str) -> Result<(), CursedError> {
        println!("Executing build command: {}", command);

        let mut cmd = if cfg!(target_os = "windows") {
            let mut c = Command::new("cmd");
            c.args(&["/C", command]);
            c
        } else {
            let mut c = Command::new("sh");
            c.args(&["-c", command]);
            c
        };

        let output = cmd
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .map_err(|e| CursedError::IoError(format!("Failed to execute build command: {}", e)))?;

        if !output.status.success() {
            return Err(CursedError::IoError(format!(
                "Build command failed with exit code: {:?}",
                output.status.code()
            )));
        }

        Ok(())
    }

    /// Check documentation coverage
    fn check_documentation_coverage(&self) -> Result<(bool, f64), CursedError> {
        println!("Checking documentation coverage...");

        // Load documentation data
        let mut generator = DocumentationGenerator::new(None)?;
        generator.generate()?;

        // Analyze coverage
        let mut analyzer = CoverageAnalyzer::new();
        analyzer.analyze_documentation(&generator.documentation)?;

        let coverage_percentage = analyzer.global_coverage.overall_percentage;
        let passed = coverage_percentage >= self.config.coverage_threshold;

        // Generate coverage report
        let report_config = CoverageReportConfig {
            include_missing_items: true,
            include_quality_metrics: true,
            include_suggestions: true,
            format: ReportFormat::Console,
            output_file: Some("docs/coverage-report.html".to_string()),
        };

        if let Err(e) = analyzer.save_report(&report_config) {
            eprintln!("Warning: Failed to save coverage report: {}", e);
        }

        println!("Documentation coverage: {:.1}% (threshold: {:.1}%)", 
            coverage_percentage, self.config.coverage_threshold);

        Ok((passed, coverage_percentage))
    }

    /// Execute build hooks
    fn execute_hooks(&self, hooks: &[String]) -> Result<(), CursedError> {
        for hook in hooks {
            if !hook.trim().is_empty() {
                println!("Executing hook: {}", hook);
                
                let mut cmd = if cfg!(target_os = "windows") {
                    let mut c = Command::new("cmd");
                    c.args(&["/C", hook]);
                    c
                } else {
                    let mut c = Command::new("sh");
                    c.args(&["-c", hook]);
                    c
                };

                let output = cmd
                    .output()
                    .map_err(|e| CursedError::IoError(format!("Failed to execute hook: {}", e)))?;

                if !output.status.success() {
                    return Err(CursedError::IoError(format!(
                        "Hook failed: {} (exit code: {:?})",
                        hook,
                        output.status.code()
                    )));
                }
            }
        }
        Ok(())
    }

    /// Create necessary output directories
    fn create_output_directories(&self) -> Result<(), CursedError> {
        let output_dir = Path::new(&self.doc_config.output.output_dir);
        fs::create_dir_all(output_dir)
            .map_err(|e| CursedError::IoError(format!("Failed to create output directory: {}", e)))?;

        // Create subdirectories for different formats
        for format in &self.config.output_formats {
            let format_dir = output_dir.join(format);
            fs::create_dir_all(&format_dir)
                .map_err(|e| CursedError::IoError(format!("Failed to create {} directory: {}", format, e)))?;
        }

        Ok(())
    }

    /// Scan for file dependencies
    fn scan_file_dependencies(&mut self) -> Result<(), CursedError> {
        self.file_dependencies.clear();

        let source_dirs = self.doc_config.input.source_dirs.clone();
        for source_dir in &source_dirs {
            let dir_path = PathBuf::from(source_dir);
            if dir_path.exists() {
                self.scan_directory_dependencies(&dir_path)?;
            }
        }

        Ok(())
    }

    /// Scan directory for file dependencies
    fn scan_directory_dependencies(&mut self, dir: &Path) -> Result<(), CursedError> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && self.should_check_file(&path) {
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        self.file_dependencies.insert(path, modified);
                    }
                }
            } else if path.is_dir() && !self.should_exclude_directory(&path) {
                self.scan_directory_dependencies(&path)?;
            }
        }

        Ok(())
    }

    /// Update build cache
    fn update_build_cache(&mut self) -> Result<(), CursedError> {
        // Update module timestamps
        self.build_cache.module_timestamps.clear();
        for source_dir in &self.doc_config.input.source_dirs {
            // Implementation would scan modules and update timestamps
        }

        // Update generated files list
        self.build_cache.generated_files.clear();
        let output_dir_path = self.doc_config.output.output_dir.clone();
        let output_dir = Path::new(&output_dir_path);
        if output_dir.exists() {
            self.scan_generated_files(output_dir)?;
        }

        Ok(())
    }

    /// Scan generated files for cache
    fn scan_generated_files(&mut self, dir: &Path) -> Result<(), CursedError> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                self.build_cache.generated_files.push(path);
            } else if path.is_dir() {
                self.scan_generated_files(&path)?;
            }
        }

        Ok(())
    }

    /// Clean generated documentation files
    pub fn clean(&mut self) -> Result<(), CursedError> {
        println!("Cleaning generated documentation files...");

        let output_dir = Path::new(&self.doc_config.output.output_dir);
        if output_dir.exists() {
            fs::remove_dir_all(output_dir)
                .map_err(|e| CursedError::IoError(format!("Failed to clean output directory: {}", e)))?;
        }

        // Clear caches
        self.build_cache = BuildCache::new();
        self.file_dependencies.clear();
        self.last_build_time = None;

        println!("Documentation files cleaned");
        Ok(())
    }

    /// Watch for file changes and auto-regenerate documentation
    pub fn start_watch_mode(&mut self) -> Result<(), CursedError> {
        if !self.config.watch_files {
            return Ok(());
        }

        println!("Starting documentation watch mode...");
        println!("Watching for changes in source files...");

        loop {
            std::thread::sleep(Duration::from_millis(1000));

            if self.needs_documentation_update()? {
                println!("Changes detected, regenerating documentation...");
                
                match self.generate_documentation(true) {
                    Ok(_) => println!("Documentation updated successfully"),
                    Err(e) => eprintln!("Failed to update documentation: {}", e),
                }
            }
        }
    }
}

impl BuildCache {
    fn new() -> Self {
        Self {
            module_timestamps: HashMap::new(),
            generated_files: Vec::new(),
            coverage_data: None,
        }
    }
}

impl Default for BuildIntegrationConfig {
    fn default() -> Self {
        Self {
            auto_generate: false,
            generate_on_build: true,
            generate_on_test: false,
            check_coverage: false,
            coverage_threshold: 70.0,
            fail_on_low_coverage: false,
            watch_files: false,
            output_formats: vec!["html".to_string()],
            build_hooks: BuildHooks::default(),
        }
    }
}

impl Default for BuildHooks {
    fn default() -> Self {
        Self {
            pre_build: Vec::new(),
            post_build: Vec::new(),
            pre_doc: Vec::new(),
            post_doc: Vec::new(),
            on_coverage_failure: Vec::new(),
        }
    }
}

/// CLI interface for build integration
pub struct BuildIntegrationCli;

impl BuildIntegrationCli {
    /// Initialize build integration
    pub fn init() -> Result<(), CursedError> {
        println!("Initializing CURSED documentation build integration...");

        // Create default configuration file
        let config_content = r#"
[build_integration]
auto_generate = false
generate_on_build = true
generate_on_test = false
check_coverage = true
coverage_threshold = 70.0
fail_on_low_coverage = false
watch_files = false
output_formats = ["html", "json"]

[build_integration.hooks]
pre_build = []
post_build = ["echo 'Build completed with documentation'"]
pre_doc = ["echo 'Starting documentation generation'"]
post_doc = ["echo 'Documentation generation completed'"]
on_coverage_failure = ["echo 'Documentation coverage below threshold'"]
"#;

        fs::write(".cursed-build-docs.toml", config_content)
            .map_err(|e| CursedError::IoError(format!("Failed to write config file: {}", e)))?;

        println!("Build integration configuration created: .cursed-build-docs.toml");
        println!("Edit the configuration file to customize build integration settings.");

        Ok(())
    }

    /// Run build with documentation integration
    pub fn build_with_docs(build_command: &str) -> Result<(), CursedError> {
        let config = Self::load_config()?;
        let doc_config = crate::documentation::DocumentationGenerator::load_config(None)?;
        
        let mut manager = DocBuildManager::new(config, doc_config);
        manager.initialize()?;
        
        let result = manager.integrate_with_build(build_command)?;
        
        // Print build summary
        println!("\n=== Build Summary ===");
        println!("Success: {}", result.success);
        println!("Documentation generated: {}", result.documentation_generated);
        println!("Coverage passed: {}", result.coverage_passed);
        println!("Coverage: {:.1}%", result.coverage_percentage);
        println!("Build time: {:.2}s", result.build_time.as_secs_f64());
        
        if !result.errors.is_empty() {
            println!("\nErrors:");
            for error in &result.errors {
                println!("  - {}", error);
            }
        }
        
        if !result.warnings.is_empty() {
            println!("\nWarnings:");
            for warning in &result.warnings {
                println!("  - {}", warning);
            }
        }

        if !result.success {
            return Err(CursedError::IoError("Build failed".to_string()));
        }

        Ok(())
    }

    /// Load build integration configuration
    fn load_config() -> Result<BuildIntegrationConfig, CursedError> {
        let config_path = ".cursed-build-docs.toml";
        
        if !Path::new(config_path).exists() {
            return Ok(BuildIntegrationConfig::default());
        }

        let content = fs::read_to_string(config_path)
            .map_err(|e| CursedError::IoError(format!("Failed to read config file: {}", e)))?;

        // Parse TOML config (simplified - would use proper TOML parsing in real implementation)
        Ok(BuildIntegrationConfig::default())
    }
}
