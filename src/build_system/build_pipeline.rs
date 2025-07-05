//! Multi-file Build Pipeline for CURSED Projects
//!
//! This module implements the core build pipeline that orchestrates the compilation
//! of multi-file CURSED projects, handling dependencies, incremental builds, and
//! various output formats.

use crate::error::{CursedError, Result};
use crate::imports::resolver::{ImportResolver, ImportConfig, ResolvedImport};
use crate::lexer::Lexer;
use crate::parser::Parser;
// Note: TypeChecker and LLVMCodeGenerator would be used for actual compilation
// For now, we'll use placeholder implementations
use crate::optimization::OptimizationConfig;
use crate::ast::Program;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::fs;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};

/// Configuration for the build pipeline
#[derive(Debug, Clone)]
pub struct BuildConfig {
    /// Project root directory
    pub project_root: PathBuf,
    /// Source directories to compile
    pub source_dirs: Vec<PathBuf>,
    /// Output directory for build artifacts
    pub output_dir: PathBuf,
    /// Main entry point file
    pub main_file: Option<PathBuf>,
    /// Build mode (Debug, Release, Test)
    pub build_mode: BuildMode,
    /// Optimization configuration
    pub optimization: OptimizationConfig,
    /// Target architecture
    pub target: Option<String>,
    /// Number of parallel compilation jobs
    pub jobs: Option<usize>,
    /// Incremental compilation enabled
    pub incremental: bool,
    /// Generate debug information
    pub debug_info: bool,
    /// Additional compiler flags
    pub compiler_flags: Vec<String>,
    /// Import resolver configuration
    pub import_config: ImportConfig,
}

/// Build mode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildMode {
    Debug,
    Release,
    Test,
}

/// Build output format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Executable,
    Library,
    Object,
    LLVMIR,
    Assembly,
}

/// Compilation unit representing a single source file
#[derive(Debug, Clone)]
pub struct CompilationUnit {
    /// Source file path
    pub path: PathBuf,
    /// Source content
    pub source: String,
    /// Parsed AST
    pub program: Program,
    /// Last modification time
    pub modified: SystemTime,
    /// Dependencies (imports)
    pub dependencies: Vec<PathBuf>,
    /// Generated LLVM IR
    pub llvm_ir: Option<String>,
    /// Compiled to object file
    pub object_file: Option<PathBuf>,
}

/// Build artifact information
#[derive(Debug, Clone)]
pub struct BuildArtifact {
    /// Artifact type
    pub artifact_type: String,
    /// File path
    pub path: PathBuf,
    /// Creation time
    pub created: SystemTime,
    /// Source files that generated this artifact
    pub sources: Vec<PathBuf>,
}

/// Build result containing compilation information
#[derive(Debug, Clone)]
pub struct BuildResult {
    /// Success status
    pub success: bool,
    /// Compilation units processed
    pub units: Vec<CompilationUnit>,
    /// Generated artifacts
    pub artifacts: Vec<BuildArtifact>,
    /// Build warnings
    pub warnings: Vec<String>,
    /// Build errors
    pub errors: Vec<String>,
    /// Build duration
    pub duration: std::time::Duration,
}

/// Cache for incremental compilation
#[derive(Debug, Default)]
pub struct BuildCache {
    /// File modification times
    pub file_times: HashMap<PathBuf, SystemTime>,
    /// Dependency graph
    pub dependencies: HashMap<PathBuf, Vec<PathBuf>>,
    /// Compiled artifacts
    pub artifacts: HashMap<PathBuf, BuildArtifact>,
}

/// Main build pipeline implementation
pub struct BuildPipeline {
    /// Build configuration
    config: BuildConfig,
    /// Import resolver
    resolver: ImportResolver,
    /// Build cache
    cache: BuildCache,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            project_root: PathBuf::from("."),
            source_dirs: vec![PathBuf::from("src")],
            output_dir: PathBuf::from("target"),
            main_file: None,
            build_mode: BuildMode::Release,
            optimization: OptimizationConfig::default(),
            target: None,
            jobs: None,
            incremental: true,
            debug_info: false,
            compiler_flags: Vec::new(),
            import_config: ImportConfig::default(),
        }
    }
}

impl BuildPipeline {
    /// Create a new build pipeline with the given configuration
    pub fn new(config: BuildConfig) -> Result<Self> {
        let mut import_config = config.import_config.clone();
        
        // Configure import resolver with project-specific settings
        import_config.search_paths = config.source_dirs.clone();
        import_config.search_paths.push(config.project_root.clone());
        
        let resolver = ImportResolver::with_config(import_config)?;
        let cache = Self::load_cache(&config.output_dir)?;
        
        Ok(Self {
            config,
            resolver,
            cache,
        })
    }

    /// Build the entire project
    pub async fn build(&mut self) -> Result<BuildResult> {
        let start_time = std::time::Instant::now();
        
        println!("🔨 Building CURSED project...");
        
        // Discover source files
        let source_files = self.discover_source_files()?;
        println!("📁 Found {} source files", source_files.len());
        
        // Create compilation units
        let mut units = self.create_compilation_units(source_files)?;
        println!("📦 Created {} compilation units", units.len());
        
        // Resolve dependencies and create build order
        let build_order = self.resolve_build_order(&mut units).await?;
        println!("🔗 Resolved build order: {} files", build_order.len());
        
        // Perform incremental compilation check
        let units_to_compile = if self.config.incremental {
            self.filter_incremental_units(&units, &build_order)?
        } else {
            build_order
        };
        
        println!("⚡ Compiling {} files (incremental: {})", 
                 units_to_compile.len(), 
                 self.config.incremental);
        
        // Compile units in dependency order
        let mut compiled_units = Vec::new();
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        
        for unit_path in units_to_compile {
            let unit = units.iter_mut().find(|u| u.path == unit_path).unwrap();
            
            match self.compile_unit(unit).await {
                Ok(warns) => {
                    warnings.extend(warns);
                    compiled_units.push(unit.clone());
                    println!("✅ Compiled: {}", unit.path.display());
                }
                Err(e) => {
                    errors.push(format!("Error compiling {}: {}", unit.path.display(), e));
                    println!("❌ Failed: {}", unit.path.display());
                }
            }
        }
        
        // Link if main file is specified
        let mut artifacts = Vec::new();
        
        if let Some(main_file) = &self.config.main_file {
            if errors.is_empty() {
                match self.link_executable(&compiled_units, main_file).await {
                    Ok(artifact) => {
                        artifacts.push(artifact);
                        println!("🔗 Linked executable successfully");
                    }
                    Err(e) => {
                        errors.push(format!("Linking failed: {}", e));
                        println!("❌ Linking failed");
                    }
                }
            }
        }
        
        // Update cache
        self.update_cache(&units)?;
        
        // Save cache
        self.save_cache()?;
        
        let duration = start_time.elapsed();
        let success = errors.is_empty();
        
        if success {
            println!("🎉 Build completed successfully in {:?}", duration);
        } else {
            println!("💥 Build failed with {} errors", errors.len());
        }
        
        Ok(BuildResult {
            success,
            units,
            artifacts,
            warnings,
            errors,
            duration,
        })
    }

    /// Discover all source files in the project
    fn discover_source_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        
        for source_dir in &self.config.source_dirs {
            let search_path = if source_dir.is_absolute() {
                source_dir.clone()
            } else {
                self.config.project_root.join(source_dir)
            };
            
            if search_path.exists() {
                self.discover_files_recursive(&search_path, &mut files)?;
            }
        }
        
        Ok(files)
    }

    /// Recursively discover .csd files in a directory
    fn discover_files_recursive(&self, dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_dir() {
                    self.discover_files_recursive(&path, files)?;
                } else if path.extension().map_or(false, |ext| ext == "csd") {
                    files.push(path);
                }
            }
        }
        
        Ok(())
    }

    /// Create compilation units from source files
    fn create_compilation_units(&mut self, files: Vec<PathBuf>) -> Result<Vec<CompilationUnit>> {
        let mut units = Vec::new();
        
        for file in files {
            let source = fs::read_to_string(&file)?;
            let modified = fs::metadata(&file)?.modified()?;
            
            // Parse the source
            let mut lexer = Lexer::new(source.clone());
            let tokens = lexer.tokenize()?;
            let mut parser = Parser::from_tokens(tokens);
            let program = parser.parse_program()?;
            
            // Extract dependencies
            let dependencies = program.imports.iter()
                .map(|import| self.resolve_import_path(&import.path, &file))
                .collect::<Result<Vec<_>>>()?;
            
            units.push(CompilationUnit {
                path: file,
                source,
                program,
                modified,
                dependencies,
                llvm_ir: None,
                object_file: None,
            });
        }
        
        Ok(units)
    }

    /// Resolve import path to actual file path
    fn resolve_import_path(&self, import_path: &str, current_file: &Path) -> Result<PathBuf> {
        // Try relative to current file
        if let Some(parent) = current_file.parent() {
            let relative_path = parent.join(import_path);
            if relative_path.exists() {
                return Ok(relative_path);
            }
            
            // Try with .csd extension
            let with_ext = relative_path.with_extension("csd");
            if with_ext.exists() {
                return Ok(with_ext);
            }
        }
        
        // Try in source directories
        for source_dir in &self.config.source_dirs {
            let search_path = if source_dir.is_absolute() {
                source_dir.clone()
            } else {
                self.config.project_root.join(source_dir)
            };
            
            let candidate = search_path.join(import_path);
            if candidate.exists() {
                return Ok(candidate);
            }
            
            let with_ext = candidate.with_extension("csd");
            if with_ext.exists() {
                return Ok(with_ext);
            }
        }
        
        Err(CursedError::ImportError(format!("Could not resolve import: {}", import_path)))
    }

    /// Resolve build order based on dependencies
    async fn resolve_build_order(&mut self, units: &mut Vec<CompilationUnit>) -> Result<Vec<PathBuf>> {
        let mut order = Vec::new();
        let mut visited = HashSet::new();
        let mut visiting = HashSet::new();
        
        // Create a map for quick lookup
        let unit_map: HashMap<PathBuf, usize> = units.iter()
            .enumerate()
            .map(|(i, unit)| (unit.path.clone(), i))
            .collect();
        
        // Topological sort with cycle detection
        for unit in units.iter() {
            if !visited.contains(&unit.path) {
                self.visit_unit(&unit.path, &unit_map, units, &mut visited, &mut visiting, &mut order)?;
            }
        }
        
        Ok(order)
    }

    /// Visit a unit during topological sort
    fn visit_unit(
        &self,
        path: &PathBuf,
        unit_map: &HashMap<PathBuf, usize>,
        units: &[CompilationUnit],
        visited: &mut HashSet<PathBuf>,
        visiting: &mut HashSet<PathBuf>,
        order: &mut Vec<PathBuf>,
    ) -> Result<()> {
        if visiting.contains(path) {
            return Err(CursedError::ImportError(format!("Circular dependency detected: {}", path.display())));
        }
        
        if visited.contains(path) {
            return Ok(());
        }
        
        visiting.insert(path.clone());
        
        if let Some(&unit_idx) = unit_map.get(path) {
            let unit = &units[unit_idx];
            
            for dep in &unit.dependencies {
                if unit_map.contains_key(dep) {
                    self.visit_unit(dep, unit_map, units, visited, visiting, order)?;
                }
            }
        }
        
        visiting.remove(path);
        visited.insert(path.clone());
        order.push(path.clone());
        
        Ok(())
    }

    /// Filter units for incremental compilation
    fn filter_incremental_units(&self, units: &[CompilationUnit], build_order: &[PathBuf]) -> Result<Vec<PathBuf>> {
        let mut units_to_compile = Vec::new();
        
        for path in build_order {
            let unit = units.iter().find(|u| u.path == *path).unwrap();
            
            // Check if unit needs recompilation
            if self.unit_needs_recompilation(unit)? {
                units_to_compile.push(path.clone());
            }
        }
        
        Ok(units_to_compile)
    }

    /// Check if a unit needs recompilation
    fn unit_needs_recompilation(&self, unit: &CompilationUnit) -> Result<bool> {
        // Check if file is newer than cached time
        if let Some(cached_time) = self.cache.file_times.get(&unit.path) {
            if unit.modified > *cached_time {
                return Ok(true);
            }
        } else {
            return Ok(true);
        }
        
        // Check if any dependency is newer
        for dep in &unit.dependencies {
            if let Ok(dep_metadata) = fs::metadata(dep) {
                if let Ok(dep_modified) = dep_metadata.modified() {
                    if let Some(cached_time) = self.cache.file_times.get(&unit.path) {
                        if dep_modified > *cached_time {
                            return Ok(true);
                        }
                    }
                }
            }
        }
        
        Ok(false)
    }

    /// Compile a single unit
    async fn compile_unit(&mut self, unit: &mut CompilationUnit) -> Result<Vec<String>> {
        let mut warnings = Vec::new();
        
        // Resolve imports
        let _resolved_imports = self.resolver.resolve_imports(&unit.program.imports).await?;
        
        // For now, generate placeholder IR (actual compilation would use TypeChecker and LLVMCodeGenerator)
        let placeholder_ir = format!(
            "; Generated IR for {}\n; Source: {}\n\ndefine i32 @main() {{\n  ret i32 0\n}}\n",
            unit.path.display(),
            unit.source.len()
        );
        unit.llvm_ir = Some(placeholder_ir);
        
        // Compile to object file if needed
        if self.config.build_mode != BuildMode::Test {
            let object_path = self.get_object_path(&unit.path);
            
            // For now, just save the IR (actual object compilation would use LLVM)
            let ir_path = object_path.with_extension("ll");
            fs::write(&ir_path, unit.llvm_ir.as_ref().unwrap())?;
            
            unit.object_file = Some(object_path);
        }
        
        Ok(warnings)
    }

    /// Get object file path for a source file
    fn get_object_path(&self, source_path: &Path) -> PathBuf {
        let relative_path = source_path.strip_prefix(&self.config.project_root)
            .unwrap_or(source_path);
        
        self.config.output_dir
            .join(relative_path)
            .with_extension("o")
    }

    /// Link executable from compiled units
    async fn link_executable(&self, units: &[CompilationUnit], main_file: &Path) -> Result<BuildArtifact> {
        let executable_path = self.config.output_dir.join(
            main_file.file_stem().unwrap_or_default()
        );
        
        // Find main unit
        let main_unit = units.iter()
            .find(|u| u.path == *main_file)
            .ok_or_else(|| CursedError::CompilerError("Main file not found in compilation units".to_string()))?;
        
        // Create a simple executable by combining IR (placeholder implementation)
        let mut combined_ir = String::new();
        
        for unit in units {
            if let Some(ir) = &unit.llvm_ir {
                combined_ir.push_str(ir);
                combined_ir.push('\n');
            }
        }
        
        // Save combined IR
        let ir_path = executable_path.with_extension("ll");
        fs::write(&ir_path, combined_ir)?;
        
        // Create build artifact
        let artifact = BuildArtifact {
            artifact_type: "executable".to_string(),
            path: executable_path,
            created: SystemTime::now(),
            sources: units.iter().map(|u| u.path.clone()).collect(),
        };
        
        Ok(artifact)
    }

    /// Load build cache
    fn load_cache(_output_dir: &Path) -> Result<BuildCache> {
        // For now, just return a default cache
        // Real implementation would load from a cache file
        Ok(BuildCache::default())
    }

    /// Save build cache
    fn save_cache(&self) -> Result<()> {
        // For now, just create the output directory
        // Real implementation would save cache to file
        fs::create_dir_all(&self.config.output_dir)?;
        Ok(())
    }

    /// Update cache with compilation results
    fn update_cache(&mut self, units: &[CompilationUnit]) -> Result<()> {
        for unit in units {
            self.cache.file_times.insert(unit.path.clone(), unit.modified);
            self.cache.dependencies.insert(unit.path.clone(), unit.dependencies.clone());
        }
        
        Ok(())
    }

    /// Clean build artifacts
    pub fn clean(&self) -> Result<()> {
        if self.config.output_dir.exists() {
            fs::remove_dir_all(&self.config.output_dir)?;
        }
        
        println!("🧹 Cleaned build artifacts");
        Ok(())
    }
}

/// Builder for BuildConfig
pub struct BuildConfigBuilder {
    config: BuildConfig,
}

impl BuildConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: BuildConfig::default(),
        }
    }

    pub fn project_root<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.config.project_root = path.as_ref().to_path_buf();
        self
    }

    pub fn source_dir<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.config.source_dirs.push(path.as_ref().to_path_buf());
        self
    }

    pub fn output_dir<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.config.output_dir = path.as_ref().to_path_buf();
        self
    }

    pub fn main_file<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.config.main_file = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn build_mode(mut self, mode: BuildMode) -> Self {
        self.config.build_mode = mode;
        self
    }

    pub fn incremental(mut self, enabled: bool) -> Self {
        self.config.incremental = enabled;
        self
    }

    pub fn jobs(mut self, count: usize) -> Self {
        self.config.jobs = Some(count);
        self
    }

    pub fn build(self) -> BuildConfig {
        self.config
    }
}

impl Default for BuildConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
