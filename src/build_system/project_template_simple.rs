//! Simplified Project Template and Configuration Management
//!
//! This module provides basic project template creation and configuration management
//! for CURSED projects without complex serialization dependencies.

use crate::error::{CursedError, Result};
use crate::build_system::build_pipeline::BuildConfig;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Simple project configuration
#[derive(Debug, Clone)]
pub struct ProjectConfig {
    /// Project name
    pub name: String,
    /// Project version
    pub version: String,
    /// Project root directory
    pub root: PathBuf,
    /// Source directories
    pub source_dirs: Vec<PathBuf>,
    /// Main entry point
    pub main_file: Option<PathBuf>,
    /// Project dependencies
    pub dependencies: HashMap<String, String>,
    /// Build configuration
    pub build_config: BuildConfig,
}

/// Project template for creating new projects
pub struct ProjectTemplate {
    /// Template name
    pub name: String,
    /// Template description
    pub description: String,
    /// Files to create
    pub files: HashMap<String, String>,
}

impl ProjectTemplate {
    /// Create a new project template
    pub fn new(name: String) -> Self {
        let (description, files) = match name.as_str() {
            "bin" => Self::binary_template(),
            "lib" => Self::library_template(),
            _ => Self::default_template(),
        };
        
        Self {
            name,
            description,
            files,
        }
    }

    /// Create a binary project template
    fn binary_template() -> (String, HashMap<String, String>) {
        let mut files = HashMap::new();
        
        // CursedPackage.toml
        files.insert("CursedPackage.toml".to_string(), r#"[package]
name = "{{name}}"
version = "0.1.0"
description = "A CURSED binary project"
authors = ["Your Name <your.email@example.com>"]
license = "MIT"
edition = "2024"

[build]
source_dirs = ["src"]
output_dir = "target"

[[bin]]
name = "{{name}}"
path = "src/main.csd"
"#.to_string());

        // src/main.csd
        files.insert("src/main.csd".to_string(), r#"// CURSED Binary Project
// This is the main entry point for your application

func main() {
    print("Hello, CURSED world! 🔥")
    print("This is a binary project created from template")
}
"#.to_string());

        // README.md
        files.insert("README.md".to_string(), r#"# {{name}}

A CURSED binary project.

## Building

```bash
cursed build
```

## Running

```bash
cursed run
```
"#.to_string());

        ("A CURSED binary project template".to_string(), files)
    }

    /// Create a library project template
    fn library_template() -> (String, HashMap<String, String>) {
        let mut files = HashMap::new();
        
        // CursedPackage.toml
        files.insert("CursedPackage.toml".to_string(), r#"[package]
name = "{{name}}"
version = "0.1.0"
description = "A CURSED library project"
authors = ["Your Name <your.email@example.com>"]
license = "MIT"
edition = "2024"

[build]
source_dirs = ["src"]
output_dir = "target"

[lib]
name = "{{name}}"
path = "src/lib.csd"
"#.to_string());

        // src/lib.csd
        files.insert("src/lib.csd".to_string(), r#"// CURSED Library Project
// This is the main library module

/// A sample function that demonstrates library functionality
pub func greet(name: string) -> string {
    return "Hello, " + name + "! This is a CURSED library 🔥"
}

/// A sample constant
pub const LIBRARY_VERSION: string = "0.1.0"
"#.to_string());

        ("A CURSED library project template".to_string(), files)
    }

    /// Create a default project template
    fn default_template() -> (String, HashMap<String, String>) {
        Self::binary_template()
    }

    /// Create a project from this template
    pub fn create_project<P: AsRef<Path>>(&self, project_path: P, project_name: &str) -> Result<ProjectConfig> {
        let project_path = project_path.as_ref();
        
        // Create project directory
        fs::create_dir_all(project_path)?;
        
        // Create files from template
        for (file_path, content) in &self.files {
            let full_path = project_path.join(file_path);
            
            // Create parent directories
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            // Replace template variables
            let processed_content = content.replace("{{name}}", project_name);
            
            // Write file
            fs::write(&full_path, processed_content)?;
        }
        
        println!("✅ Created project '{}' from template '{}'", project_name, self.name);
        
        // Create a simple project configuration
        let config = ProjectConfig {
            name: project_name.to_string(),
            version: "0.1.0".to_string(),
            root: project_path.to_path_buf(),
            source_dirs: vec![project_path.join("src")],
            main_file: Some(project_path.join("src/main.csd")),
            dependencies: HashMap::new(),
            build_config: BuildConfig::default(),
        };
        
        Ok(config)
    }

    /// Load project configuration from directory
    pub fn load_project_config<P: AsRef<Path>>(project_path: P) -> Result<ProjectConfig> {
        let project_path = project_path.as_ref();
        
        // Simple implementation that doesn't require complex parsing
        let mut config = ProjectConfig {
            name: project_path.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            version: "0.1.0".to_string(),
            root: project_path.to_path_buf(),
            source_dirs: vec![project_path.join("src")],
            main_file: None,
            dependencies: HashMap::new(),
            build_config: BuildConfig::default(),
        };
        
        // Look for main files
        let main_candidates = vec![
            project_path.join("src/main.csd"),
            project_path.join("src/lib.csd"),
            project_path.join("main.csd"),
        ];
        
        for candidate in main_candidates {
            if candidate.exists() {
                config.main_file = Some(candidate);
                break;
            }
        }
        
        // Check if CursedPackage.toml exists (simple check, no parsing)
        let manifest_path = project_path.join("CursedPackage.toml");
        if manifest_path.exists() {
            // Extract name from manifest (simple regex-like approach)
            if let Ok(content) = fs::read_to_string(&manifest_path) {
                for line in content.lines() {
                    if line.trim().starts_with("name = ") {
                        if let Some(name_part) = line.split('=').nth(1) {
                            let name = name_part.trim().trim_matches('"').trim_matches('\'');
                            if !name.is_empty() {
                                config.name = name.to_string();
                            }
                        }
                    }
                }
            }
        }
        
        Ok(config)
    }

    /// Initialize a new project in the current directory
    pub fn init_project<P: AsRef<Path>>(project_path: P, project_name: &str, template_name: &str) -> Result<ProjectConfig> {
        let template = Self::new(template_name.to_string());
        template.create_project(project_path, project_name)
    }

    /// List available templates
    pub fn list_templates() -> Vec<String> {
        vec![
            "bin".to_string(),
            "lib".to_string(),
        ]
    }

    /// Get template description
    pub fn get_template_description(template_name: &str) -> Option<String> {
        match template_name {
            "bin" => Some("A CURSED binary project template".to_string()),
            "lib" => Some("A CURSED library project template".to_string()),
            _ => None,
        }
    }
}

impl ProjectConfig {
    /// Validate project configuration
    pub fn validate(&self) -> Result<()> {
        // Check if root directory exists
        if !self.root.exists() {
            return Err(CursedError::General(format!(
                "Project root does not exist: {}",
                self.root.display()
            )));
        }
        
        // Check if at least one source directory exists
        let mut has_source = false;
        for source_dir in &self.source_dirs {
            if source_dir.exists() {
                has_source = true;
                break;
            }
        }
        
        if !has_source {
            return Err(CursedError::General(
                "No source directories found".to_string()
            ));
        }
        
        Ok(())
    }
}
