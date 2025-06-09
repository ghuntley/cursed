//! Integration layer for separate compilation with the main CURSED compiler
//!
//! This module provides integration between the separate compilation system
//! and the main CURSED compilation pipeline, allowing the compiler to
//! automatically detect when separate compilation should be used.

use crate::codegen::llvm::{PackageCompilationConfig, PackageCompilationPipeline, compile_and_link_packages_pipeline};
use crate::error::Error;
use std::path::{Path, PathBuf};
use tracing::{debug, info, instrument, warn};

/// Options for separate compilation
#[derive(Debug, Clone)]
pub struct SeparateCompilationOptions {
    /// Whether to use separate compilation
    pub enabled: bool,
    /// Output directory for intermediate files
    pub output_dir: PathBuf,
    /// Whether to emit LLVM IR files
    pub emit_ir: bool,
    /// Whether to emit object files
    pub emit_object: bool,
    /// Optimization level
    pub optimization_level: inkwell::OptimizationLevel,
    /// Target triple for cross-compilation
    pub target_triple: Option<String>,
}

impl Default for SeparateCompilationOptions {
    fn default() -> Self {
        Self {
            enabled: false,
            output_dir: PathBuf::from("./build"),
            emit_ir: true,
            emit_object: true,
            optimization_level: inkwell::OptimizationLevel::Default,
            target_triple: None,
        }
    }
}

/// Determines if separate compilation should be used based on input
#[instrument(level = "debug")]
pub fn should_use_separate_compilation(input_paths: &[PathBuf]) -> bool {
    debug!(path_count = input_paths.len(), "Checking if separate compilation should be used");

    // Use separate compilation if:
    // 1. Multiple files are provided
    // 2. Any file contains package declarations other than "main"
    // 3. Any file contains import statements

    if input_paths.len() > 1 {
        debug!("Multiple files detected, using separate compilation");
        return true;
    }

    // Check single file for package/import indicators
    if let Some(file_path) = input_paths.first() {
        if let Ok(content) = std::fs::read_to_string(file_path) {
            // Look for non-main package declarations
            if content.contains("vibe ") && !content.contains("vibe main") {
                debug!("Non-main package detected, using separate compilation");
                return true;
            }

            // Look for import statements
            if content.contains("yeet ") {
                debug!("Import statements detected, using separate compilation");
                return true;
            }
        }
    }

    debug!("Using single-file compilation");
    false
}

/// Compile using separate compilation pipeline
#[instrument(skip(input_paths, options), fields(file_count = input_paths.len()), level = "info")]
pub fn compile_with_separate_compilation(
    input_paths: &[PathBuf],
    output_path: &Path,
    options: SeparateCompilationOptions,
) -> Result<(), Error> {
    info!("Starting separate compilation");

    let config = PackageCompilationConfig {
        optimization_level: options.optimization_level,
        target_triple: options.target_triple,
        output_dir: options.output_dir,
        debug_info: true,
        emit_ir: options.emit_ir,
        emit_object: options.emit_object,
    };

    compile_and_link_packages_pipeline(input_paths, output_path, config)
}

/// Auto-detect compilation mode and compile accordingly
#[instrument(skip(input_paths, options), fields(file_count = input_paths.len()), level = "info")]
pub fn auto_compile(
    input_paths: &[PathBuf],
    output_path: &Path,
    mut options: SeparateCompilationOptions,
) -> Result<(), Error> {
    info!("Auto-detecting compilation mode");

    // Auto-enable separate compilation if needed
    if !options.enabled && should_use_separate_compilation(input_paths) {
        info!("Auto-enabling separate compilation");
        options.enabled = true;
    }

    if options.enabled {
        compile_with_separate_compilation(input_paths, output_path, options)
    } else {
        // Fall back to single-file compilation
        info!("Using single-file compilation");
        if input_paths.len() != 1 {
            return Err(Error::from_str("Single-file compilation requires exactly one input file"));
        }

        // Use the existing compilation pipeline
        let input = std::fs::read_to_string(&input_paths[0])
            .map_err(|e| Error::from_str(&format!("Failed to read input file: {}", e)))?;
        
        crate::run_program(&input, false, input_paths[0].clone())
    }
}

/// Enhanced file compilation that supports both single and separate compilation
#[instrument(level = "info")]
pub fn compile_files(
    input_paths: &[PathBuf],
    output_path: Option<&Path>,
    options: SeparateCompilationOptions,
) -> Result<(), Error> {
    info!(input_count = input_paths.len(), "Compiling files");

    if input_paths.is_empty() {
        return Err(Error::from_str("No input files provided"));
    }

    // Determine output path
    let default_output = PathBuf::from("./cursed_program");
    let output = output_path.unwrap_or(&default_output);

    auto_compile(input_paths, output, options)
}

/// Extract package information from source files for dependency analysis
#[instrument(skip(input_paths), fields(file_count = input_paths.len()), level = "debug")]
pub fn analyze_package_structure(input_paths: &[PathBuf]) -> Result<Vec<PackageInfo>, Error> {
    debug!("Analyzing package structure");

    let mut packages = Vec::new();

    for path in input_paths {
        let content = std::fs::read_to_string(path)
            .map_err(|e| Error::from_str(&format!("Failed to read {}: {}", path.display(), e)))?;

        let package_info = extract_package_info(&content, path.clone())?;
        packages.push(package_info);
    }

    debug!(package_count = packages.len(), "Package analysis completed");
    Ok(packages)
}

/// Information about a package extracted from source
#[derive(Debug, Clone)]
pub struct PackageInfo {
    /// Package name
    pub name: String,
    /// Source file path
    pub path: PathBuf,
    /// Dependencies (imported packages)
    pub dependencies: Vec<String>,
    /// Whether this is the main package
    pub is_main: bool,
}

/// Extract package information from source content
fn extract_package_info(content: &str, path: PathBuf) -> Result<PackageInfo, Error> {
    let mut package_name = "main".to_string();
    let mut dependencies = Vec::new();
    let mut is_main = true;

    // Parse package declaration
    for line in content.lines() {
        let line = line.trim();
        
        // Look for package declaration
        if line.starts_with("vibe ") {
            if let Some(name) = line.strip_prefix("vibe ") {
                if let Some(clean_name) = name.strip_suffix(";") {
                    package_name = clean_name.trim().to_string();
                    is_main = package_name == "main";
                }
            }
        }
        
        // Look for import statements
        if line.starts_with("yeet ") {
            if let Some(start) = line.find('"') {
                if let Some(end) = line[start + 1..].find('"') {
                    let import_name = &line[start + 1..start + 1 + end];
                    dependencies.push(import_name.to_string());
                }
            }
        }
    }

    Ok(PackageInfo {
        name: package_name,
        path,
        dependencies,
        is_main,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_should_use_separate_compilation() {
        let temp_dir = TempDir::new().unwrap();

        // Single file with main package - should use single compilation
        let main_file = temp_dir.path().join("main.csd");
        fs::write(&main_file, "vibe main;\nslay main() {}").unwrap();
        assert!(!should_use_separate_compilation(&[main_file.clone()]));

        // Single file with non-main package - should use separate compilation
        let pkg_file = temp_dir.path().join("package.csd");
        fs::write(&pkg_file, "vibe mypackage;\nslay func() {}").unwrap();
        assert!(should_use_separate_compilation(&[pkg_file.clone()]));

        // Single file with imports - should use separate compilation
        let import_file = temp_dir.path().join("imports.csd");
        fs::write(&import_file, "vibe main;\nyeet \"fmt\";\nslay main() {}").unwrap();
        assert!(should_use_separate_compilation(&[import_file.clone()]));

        // Multiple files - should use separate compilation
        assert!(should_use_separate_compilation(&[main_file, pkg_file]));
    }

    #[test]
    fn test_extract_package_info() {
        let content = r#"
vibe testpkg;

yeet "fmt"
yeet "strings"

slay main() {
    vibez.spill("Hello!")
}
"#;

        let path = PathBuf::from("test.csd");
        let info = extract_package_info(content, path.clone()).unwrap();

        assert_eq!(info.name, "testpkg");
        assert_eq!(info.path, path);
        assert_eq!(info.dependencies, vec!["fmt", "strings"]);
        assert!(!info.is_main);
    }

    #[test]
    fn test_main_package_detection() {
        let content = r#"
vibe main;

slay main() {
    vibez.spill("Hello!")
}
"#;

        let path = PathBuf::from("main.csd");
        let info = extract_package_info(content, path).unwrap();

        assert_eq!(info.name, "main");
        assert!(info.is_main);
        assert!(info.dependencies.is_empty());
    }
}
