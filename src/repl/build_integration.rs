use crate::error_types::Error;
// Build System Integration for CURSED REPL
// 
// Provides integration with the CURSED build system, allowing
// REPL users to build, test, format, and lint their projects
// without leaving the interactive environment.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::fs;

use crate::repl::ReplResult;
use crate::error::Error;

/// Project information structure
#[derive(Debug, Clone)]
pub struct ProjectInfo {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub root_directory: PathBuf,
    pub source_directories: Vec<PathBuf>,
    pub build_file: Option<PathBuf>,
    pub config_file: Option<PathBuf>,
}

/// Build target information
#[derive(Debug, Clone)]
pub struct BuildTarget {
    pub name: String,
    pub target_type: String,
    pub source_files: Vec<PathBuf>,
    pub dependencies: Vec<String>,
}

/// Test information
#[derive(Debug, Clone)]
pub struct TestInfo {
    pub name: String,
    pub file_path: PathBuf,
    pub test_functions: Vec<String>,
}

/// Build system integration for CURSED REPL
pub struct BuildIntegration {
    working_directory: Option<PathBuf>,
    project_info: Option<ProjectInfo>,
    build_targets: HashMap<String, BuildTarget>,
    test_files: Vec<TestInfo>,
    last_build_time: Option<std::time::SystemTime>,
}

impl BuildIntegration {
    /// Create a new build integration instance
    pub fn new() -> Self {
        Self {
            working_directory: None,
            project_info: None,
            build_targets: HashMap::new(),
            test_files: Vec::new(),
            last_build_time: None,
        }
    }

    /// Set the working directory and scan for project files
    pub fn set_working_directory(&mut self, dir: PathBuf) -> ReplResult<()> {
        if !dir.exists() {
            return Err(Error::repl_error(format!("Directory does not exist: {}", dir.display())));
        }

        self.working_directory = Some(dir);
        self.scan_project(&self.working_directory.clone().unwrap())?;
        Ok(())
    }

    /// Scan the project directory for build files and structure
    pub fn scan_project(&mut self, dir: &Path) -> ReplResult<()> {
        // Look for CURSED project files
        let cursed_build = dir.join("CursedBuild.toml");
        let cursed_package = dir.join("CursedPackage.toml");
        let makefile = dir.join("Makefile");

        if cursed_build.exists() {
            self.load_cursed_build_file(&cursed_build)?;
        } else if cursed_package.exists() {
            self.load_cursed_package_file(&cursed_package)?;
        } else if makefile.exists() {
            self.load_makefile(&makefile)?;
        } else {
            // Create a default project info
            self.create_default_project_info(dir)?;
        }

        // Scan for source files and tests
        self.scan_source_files(dir)?;
        self.scan_test_files(dir)?;

        Ok(())
    }

    /// Build the project or a specific target
    pub fn build_project(&mut self, target: Option<&str>) -> ReplResult<String> {
        let working_dir = self.working_directory.as_ref()
            .ok_or_else(|| Error::repl_error("No working directory set".to_string()))?;

        let mut result = String::new();

        // Determine build command based on project structure
        let build_command = if working_dir.join("CursedBuild.toml").exists() {
            self.build_with_cursed_build(target)?
        } else if working_dir.join("Makefile").exists() {
            self.build_with_make(target)?
        } else {
            self.build_with_cursed_compiler(target)?
        };

        // Execute build command
        let output = Command::new("sh")
            .arg("-c")
            .arg(&build_command)
            .current_dir(working_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| Error::repl_error(format!("Failed to execute build: {}", e)))?;

        if output.status.success() {
            result.push_str("✅ Build successful!\n");
            self.last_build_time = Some(std::time::SystemTime::now());
        } else {
            result.push_str("❌ Build failed!\n");
        }

        result.push_str(&String::from_utf8_lossy(&output.stdout));
        if !output.stderr.is_empty() {
            result.push_str("\nErrors:\n");
            result.push_str(&String::from_utf8_lossy(&output.stderr));
        }

        Ok(result)
    }

    /// Run project tests
    pub fn run_tests(&self, pattern: Option<&str>) -> ReplResult<String> {
        let working_dir = self.working_directory.as_ref()
            .ok_or_else(|| Error::repl_error("No working directory set".to_string()))?;

        let mut result = String::new();

        // Determine test command
        let test_command = if working_dir.join("CursedBuild.toml").exists() {
            format!("cursed-build test {}", pattern.unwrap_or(""))
        } else if working_dir.join("Makefile").exists() {
            "make test".to_string()
        } else {
            format!("cursed test {}", pattern.unwrap_or(""))
        };

        // Execute test command
        let output = Command::new("sh")
            .arg("-c")
            .arg(&test_command)
            .current_dir(working_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| Error::repl_error(format!("Failed to execute tests: {}", e)))?;

        if output.status.success() {
            result.push_str("✅ Tests passed!\n");
        } else {
            result.push_str("❌ Some tests failed!\n");
        }

        result.push_str(&String::from_utf8_lossy(&output.stdout));
        if !output.stderr.is_empty() {
            result.push_str("\nErrors:\n");
            result.push_str(&String::from_utf8_lossy(&output.stderr));
        }

        Ok(result)
    }

    /// Format a file or the entire project
    pub fn format_file(&self, file_path: &str) -> ReplResult<String> {
        let working_dir = self.working_directory.as_ref()
            .ok_or_else(|| Error::repl_error("No working directory set".to_string()))?;

        let format_command = if file_path.is_empty() {
            "cursed format .".to_string()
        } else {
            format!("cursed format {}", file_path)
        };

        let output = Command::new("sh")
            .arg("-c")
            .arg(&format_command)
            .current_dir(working_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| Error::repl_error(format!("Failed to format file: {}", e)))?;

        let mut result = String::new();
        
        if output.status.success() {
            result.push_str("✅ Formatting completed!\n");
        } else {
            result.push_str("❌ Formatting failed!\n");
        }

        result.push_str(&String::from_utf8_lossy(&output.stdout));
        if !output.stderr.is_empty() {
            result.push_str("\nErrors:\n");
            result.push_str(&String::from_utf8_lossy(&output.stderr));
        }

        Ok(result)
    }

    /// Lint a file or the entire project
    pub fn lint_file(&self, file_path: &str) -> ReplResult<String> {
        let working_dir = self.working_directory.as_ref()
            .ok_or_else(|| Error::repl_error("No working directory set".to_string()))?;

        let lint_command = if file_path.is_empty() {
            "cursed-lint .".to_string()
        } else {
            format!("cursed-lint {}", file_path)
        };

        let output = Command::new("sh")
            .arg("-c")
            .arg(&lint_command)
            .current_dir(working_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| Error::repl_error(format!("Failed to lint file: {}", e)))?;

        let mut result = String::new();
        
        if output.status.success() {
            result.push_str("✅ No linting issues found!\n");
        } else {
            result.push_str("🔍 Linting issues found:\n");
        }

        result.push_str(&String::from_utf8_lossy(&output.stdout));
        if !output.stderr.is_empty() {
            result.push_str("\nErrors:\n");
            result.push_str(&String::from_utf8_lossy(&output.stderr));
        }

        Ok(result)
    }

    /// Get project information
    pub fn get_project_info(&self) -> ReplResult<String> {
        match &self.project_info {
            Some(info) => {
                let mut result = String::new();
                result.push_str(&format!("Name: {}\n", info.name));
                result.push_str(&format!("Version: {}\n", info.version));
                if let Some(desc) = &info.description {
                    result.push_str(&format!("Description: {}\n", desc));
                }
                result.push_str(&format!("Root: {}\n", info.root_directory.display()));
                result.push_str(&format!("Source dirs: {}\n", info.source_directories.len()));
                Ok(result)
            }
            None => Ok("No project information available".to_string()),
        }
    }

    /// Get workspace information
    pub fn get_workspace_info(&self) -> ReplResult<String> {
        let mut result = String::new();
        
        if let Some(dir) = &self.working_directory {
            result.push_str(&format!("Working Directory: {}\n", dir.display()));
        }
        
        result.push_str(&format!("Build Targets: {}\n", self.build_targets.len()));
        for target in self.build_targets.keys() {
            result.push_str(&format!("  - {}\n", target));
        }
        
        result.push_str(&format!("Test Files: {}\n", self.test_files.len()));
        for test in &self.test_files {
            result.push_str(&format!("  - {} ({} tests)\n", test.name, test.test_functions.len()));
        }

        if let Some(build_time) = self.last_build_time {
            result.push_str(&format!("Last Build: {:?}\n", build_time));
        } else {
            result.push_str("Last Build: Never\n");
        }

        Ok(result)
    }

    /// Load CURSED build file
    fn load_cursed_build_file(&mut self, file_path: &Path) -> ReplResult<()> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| Error::repl_error(format!("Failed to read build file: {}", e)))?;

        // Parse TOML content (simplified parsing)
        let mut name = "unknown".to_string();
        let mut version = "0.1.0".to_string();
        let mut description = None;

        for line in content.split("\n") {
            if let Some(name_val) = line.strip_prefix("name = ") {
                name = name_val.trim_matches('"').to_string();
            } else if let Some(version_val) = line.strip_prefix("version = ") {
                version = version_val.trim_matches('"').to_string();
            } else if let Some(desc_val) = line.strip_prefix("description = ") {
                description = Some(desc_val.trim_matches('"').to_string());
            }
        }

        self.project_info = Some(ProjectInfo {
            name,
            version,
            description,
            root_directory: file_path.parent().unwrap().to_path_buf(),
            source_directories: Vec::from([]),
            build_file: Some(file_path.to_path_buf()),
            config_file: None,
        });

        Ok(())
    }

    /// Load CURSED package file
    fn load_cursed_package_file(&mut self, file_path: &Path) -> ReplResult<()> {
        // Similar to build file but for package management
        self.load_cursed_build_file(file_path)
    }

    /// Load Makefile
    fn load_makefile(&mut self, file_path: &Path) -> ReplResult<()> {
        self.project_info = Some(ProjectInfo {
            name: file_path.parent().unwrap().file_name().unwrap().to_string_lossy().to_string(),
            version: "unknown".to_string(),
            description: Some("Makefile-based project".to_string()),
            root_directory: file_path.parent().unwrap().to_path_buf(),
            source_directories: Vec::from([]),
            build_file: Some(file_path.to_path_buf()),
            config_file: None,
        });

        Ok(())
    }

    /// Create default project info when no build files are found
    fn create_default_project_info(&mut self, dir: &Path) -> ReplResult<()> {
        self.project_info = Some(ProjectInfo {
            name: dir.file_name().unwrap_or_default().to_string_lossy().to_string(),
            version: "unknown".to_string(),
            description: None,
            root_directory: dir.to_path_buf(),
            source_directories: Vec::from([]),
            build_file: None,
            config_file: None,
        });

        Ok(())
    }

    /// Scan for source files
    fn scan_source_files(&mut self, dir: &Path) -> ReplResult<()> {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension() == Some(std::ffi::OsStr::new("csd")) {
                    // Add to build targets
                    let name = path.file_stem().unwrap().to_string_lossy().to_string();
                    let target = BuildTarget {
                        name: name.clone(),
                        target_type: "executable".to_string(),
                        source_files: Vec::from([path]),
                        dependencies: Vec::from([]),
                    };
                    self.build_targets.insert(name, target);
                }
            }
        }

        Ok(())
    }

    /// Scan for test files
    fn scan_test_files(&mut self, dir: &Path) -> ReplResult<()> {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && 
                   path.extension() == Some(std::ffi::OsStr::new("csd")) &&
                   path.file_name().unwrap().to_string_lossy().contains("test") {
                    
                    let test_info = TestInfo {
                        name: path.file_stem().unwrap().to_string_lossy().to_string(),
                        file_path: path,
                        test_functions: Vec::from([]), // Would be populated by parsing the file
                    };
                    self.test_files.push(test_info);
                }
            }
        }

        Ok(())
    }

    /// Build with CURSED build system
    fn build_with_cursed_build(&self, target: Option<&str>) -> ReplResult<String> {
        if let Some(target) = target {
            Ok(format!("cursed-build build {}", target))
        } else {
            Ok("cursed-build build".to_string())
        }
    }

    /// Build with Make
    fn build_with_make(&self, target: Option<&str>) -> ReplResult<String> {
        if let Some(target) = target {
            Ok(format!("make {}", target))
        } else {
            Ok("make".to_string())
        }
    }

    /// Build with CURSED compiler directly
    fn build_with_cursed_compiler(&self, target: Option<&str>) -> ReplResult<String> {
        if let Some(target) = target {
            if let Some(build_target) = self.build_targets.get(target) {
                if let Some(source_file) = build_target.source_files.first() {
                    return Ok(format!("cursed build {}", source_file.display()));
                }
            }
            Ok(format!("cursed build {}.csd", target))
        } else {
            // Build all source files
            Ok("cursed build *.csd".to_string())
        }
    }
}

impl Default for BuildIntegration {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_build_integration_creation() {
        let integration = BuildIntegration::new();
        assert!(integration.working_directory.is_none());
        assert!(integration.project_info.is_none());
    }

    #[test]
    fn test_scan_project_with_cursed_build() {
        let temp_dir = TempDir::new().unwrap();
        let build_file = temp_dir.path().join("CursedBuild.toml");
        
        fs::write(&build_file, r#"
name = "test_project"
version = "1.0.0"
description = "Test project"
"#).unwrap();

        let mut integration = BuildIntegration::new();
        assert!(integration.scan_project(temp_dir.path()).is_ok());
        
        let project_info = integration.project_info.unwrap();
        assert_eq!(project_info.name, "test_project");
        assert_eq!(project_info.version, "1.0.0");
    }

    #[test]
    fn test_scan_source_files() {
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.path().join("main.csd");
        
        fs::write(&source_file, "slay main() { println(\"Hello, world!\"); }").unwrap();

        let mut integration = BuildIntegration::new();
        assert!(integration.scan_project(temp_dir.path()).is_ok());
        
        assert_eq!(integration.build_targets.len(), 1);
        assert!(integration.build_targets.contains_key("main"));
    }

    #[test]
    fn test_project_info() {
        let temp_dir = TempDir::new().unwrap();
        
        let mut integration = BuildIntegration::new();
        integration.set_working_directory(temp_dir.path().to_path_buf()).unwrap();
        
        let info = integration.get_project_info().unwrap();
        assert!(!info.is_empty());
    }
}
