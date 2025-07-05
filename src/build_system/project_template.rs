//! Project Template and Configuration Management
//!
//! This module provides project template creation, configuration management,
//! and workspace initialization for CURSED projects.

use crate::error::{CursedError, Result};
use crate::build_system::build_pipeline::BuildConfig;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

/// Project configuration loaded from CursedPackage.toml
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

/// Package manifest structure (CursedPackage.toml)
#[derive(Debug, Clone)]
pub struct PackageManifest {
    /// Package metadata
    pub package: PackageMetadata,
    /// Dependencies
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    /// Development dependencies
    #[serde(default)]
    pub dev_dependencies: HashMap<String, String>,
    /// Build configuration
    #[serde(default)]
    pub build: BuildSettings,
    /// Library configuration
    #[serde(default)]
    pub lib: Option<LibraryConfig>,
    /// Binary configuration
    #[serde(default)]
    pub bin: Vec<BinaryConfig>,
}

/// Package metadata
#[derive(Debug, Clone)]
pub struct PackageMetadata {
    /// Package name
    pub name: String,
    /// Package version
    pub version: String,
    /// Package description
    pub description: Option<String>,
    /// Package authors
    pub authors: Option<Vec<String>>,
    /// License
    pub license: Option<String>,
    /// Repository URL
    pub repository: Option<String>,
    /// Keywords
    pub keywords: Option<Vec<String>>,
    /// Categories
    pub categories: Option<Vec<String>>,
    /// Edition
    #[serde(default = "default_edition")]
    pub edition: String,
}

/// Build settings from package manifest
#[derive(Debug, Clone)]
pub struct BuildSettings {
    /// Source directories
    #[serde(default = "default_source_dirs")]
    pub source_dirs: Vec<String>,
    /// Output directory
    #[serde(default = "default_output_dir")]
    pub output_dir: String,
    /// Optimization level
    #[serde(default = "default_optimization")]
    pub optimization: String,
    /// Debug information
    #[serde(default)]
    pub debug: bool,
    /// Incremental compilation
    #[serde(default = "default_incremental")]
    pub incremental: bool,
}

/// Library configuration
#[derive(Debug, Clone)]
pub struct LibraryConfig {
    /// Library name
    pub name: Option<String>,
    /// Library type
    #[serde(default = "default_lib_type")]
    pub lib_type: String,
    /// Main library file
    pub path: Option<String>,
}

/// Binary configuration
#[derive(Debug, Clone)]
pub struct BinaryConfig {
    /// Binary name
    pub name: String,
    /// Binary source path
    pub path: Option<String>,
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

impl Default for BuildSettings {
    fn default() -> Self {
        Self {
            source_dirs: default_source_dirs(),
            output_dir: default_output_dir(),
            optimization: default_optimization(),
            debug: false,
            incremental: default_incremental(),
        }
    }
}

fn default_edition() -> String {
    "2024".to_string()
}

fn default_source_dirs() -> Vec<String> {
    vec!["src".to_string()]
}

fn default_output_dir() -> String {
    "target".to_string()
}

fn default_optimization() -> String {
    "2".to_string()
}

fn default_incremental() -> bool {
    true
}

fn default_lib_type() -> String {
    "lib".to_string()
}

impl ProjectTemplate {
    /// Create a new project template
    pub fn new(name: String) -> Self {
        let (description, files) = match name.as_str() {
            "bin" => Self::binary_template(),
            "lib" => Self::library_template(),
            "workspace" => Self::workspace_template(),
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
optimization = "2"
debug = false
incremental = true

[[bin]]
name = "{{name}}"
path = "src/main.csd"
"#.to_string());

        // src/main.csd
        files.insert("src/main.csd".to_string(), r#"// CURSED Binary Project
// This is the main entry point for your application

import "std::io"

func main() {
    io::print("Hello, CURSED world! 🔥")
    io::println("This is a binary project created from template")
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

## Testing

```bash
cursed test
```
"#.to_string());

        // .gitignore
        files.insert(".gitignore".to_string(), r#"# Build artifacts
target/
*.o
*.ll
*.s

# IDE files
.vscode/
.idea/
*.swp
*.swo

# OS files
.DS_Store
Thumbs.db

# CURSED specific
.cursed/
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
optimization = "2"
debug = false
incremental = true

[lib]
name = "{{name}}"
path = "src/lib.csd"
"#.to_string());

        // src/lib.csd
        files.insert("src/lib.csd".to_string(), r#"// CURSED Library Project
// This is the main library module

import "std::io"

/// A sample function that demonstrates library functionality
pub func greet(name: string) -> string {
    return "Hello, " + name + "! This is a CURSED library 🔥"
}

/// A sample constant
pub const LIBRARY_VERSION: string = "0.1.0"

/// A sample type
pub struct LibraryConfig {
    pub name: string,
    pub version: string,
    pub debug: bool,
}

impl LibraryConfig {
    pub func new(name: string) -> LibraryConfig {
        return LibraryConfig {
            name: name,
            version: LIBRARY_VERSION,
            debug: false,
        }
    }
    
    pub func enable_debug(&mut self) {
        self.debug = true
    }
}
"#.to_string());

        // src/utils.csd
        files.insert("src/utils.csd".to_string(), r#"// Utility functions for the library

import "std::math"

/// Calculate the square of a number
pub func square(x: i32) -> i32 {
    return x * x
}

/// Check if a number is even
pub func is_even(x: i32) -> bool {
    return x % 2 == 0
}

/// Generate a range of numbers
pub func range(start: i32, end: i32) -> []i32 {
    let mut result: []i32 = []
    
    for i in start..end {
        result.push(i)
    }
    
    return result
}
"#.to_string());

        // examples/basic.csd
        files.insert("examples/basic.csd".to_string(), r#"// Basic example of using the library

import "./src/lib.csd"
import "std::io"

func main() {
    let config = LibraryConfig::new("example")
    config.enable_debug()
    
    io::println(greet("World"))
    io::println("Library version: " + LIBRARY_VERSION)
    io::println("Config debug: " + config.debug.to_string())
}
"#.to_string());

        // README.md
        files.insert("README.md".to_string(), r#"# {{name}}

A CURSED library project.

## Building

```bash
cursed build
```

## Running Examples

```bash
cursed run examples/basic.csd
```

## Testing

```bash
cursed test
```

## Using in Other Projects

Add this to your `CursedPackage.toml`:

```toml
[dependencies]
{{name}} = "0.1.0"
```

Then import in your code:

```cursed
import "{{name}}"
```
"#.to_string());

        // .gitignore
        files.insert(".gitignore".to_string(), r#"# Build artifacts
target/
*.o
*.ll
*.s

# IDE files
.vscode/
.idea/
*.swp
*.swo

# OS files
.DS_Store
Thumbs.db

# CURSED specific
.cursed/
"#.to_string());

        ("A CURSED library project template".to_string(), files)
    }

    /// Create a workspace template
    fn workspace_template() -> (String, HashMap<String, String>) {
        let mut files = HashMap::new();
        
        // CursedWorkspace.toml
        files.insert("CursedWorkspace.toml".to_string(), r#"[workspace]
members = [
    "app",
    "lib",
]

[workspace.dependencies]
common = { path = "lib" }
"#.to_string());

        // app/CursedPackage.toml
        files.insert("app/CursedPackage.toml".to_string(), r#"[package]
name = "{{name}}-app"
version = "0.1.0"
description = "Application for {{name}} workspace"
authors = ["Your Name <your.email@example.com>"]
license = "MIT"
edition = "2024"

[dependencies]
{{name}}-lib = { path = "../lib" }

[[bin]]
name = "{{name}}-app"
path = "src/main.csd"
"#.to_string());

        // app/src/main.csd
        files.insert("app/src/main.csd".to_string(), r#"// Workspace Application

import "{{name}}-lib"
import "std::io"

func main() {
    io::println("🔥 Welcome to {{name}} workspace!")
    
    let result = {{name}}_lib::process_data("Hello, CURSED!")
    io::println("Result: " + result)
}
"#.to_string());

        // lib/CursedPackage.toml
        files.insert("lib/CursedPackage.toml".to_string(), r#"[package]
name = "{{name}}-lib"
version = "0.1.0"
description = "Library for {{name}} workspace"
authors = ["Your Name <your.email@example.com>"]
license = "MIT"
edition = "2024"

[lib]
name = "{{name}}-lib"
path = "src/lib.csd"
"#.to_string());

        // lib/src/lib.csd
        files.insert("lib/src/lib.csd".to_string(), r#"// Workspace Library

import "std::string"

/// Process data with the library
pub func process_data(input: string) -> string {
    return "Processed: " + input.to_uppercase()
}

/// Library configuration
pub struct Config {
    pub name: string,
    pub version: string,
}

impl Config {
    pub func new() -> Config {
        return Config {
            name: "{{name}}-lib",
            version: "0.1.0",
        }
    }
}
"#.to_string());

        // README.md
        files.insert("README.md".to_string(), r#"# {{name}} Workspace

A CURSED workspace with multiple projects.

## Structure

- `app/` - Main application
- `lib/` - Shared library

## Building

```bash
cursed build
```

## Running

```bash
cursed run app
```

## Testing

```bash
cursed test
```
"#.to_string());

        ("A CURSED workspace template".to_string(), files)
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
        
        // Load the created project configuration
        Self::load_project_config(project_path)
    }

    /// Load project configuration from directory
    pub fn load_project_config<P: AsRef<Path>>(project_path: P) -> Result<ProjectConfig> {
        let project_path = project_path.as_ref();
        let manifest_path = project_path.join("CursedPackage.toml");
        
        if !manifest_path.exists() {
            return Err(CursedError::General(format!(
                "No CursedPackage.toml found in {}",
                project_path.display()
            )));
        }
        
        let manifest_content = fs::read_to_string(&manifest_path)?;
        let manifest: PackageManifest = toml::from_str(&manifest_content).map_err(|e| CursedError::General(format!("Failed to parse manifest: {}", e)))?;
        
        // Convert to ProjectConfig
        let mut build_config = BuildConfig::default();
        build_config.project_root = project_path.to_path_buf();
        
        // Configure source directories
        build_config.source_dirs = manifest.build.source_dirs.iter()
            .map(|dir| project_path.join(dir))
            .collect();
        
        // Configure output directory
        build_config.output_dir = project_path.join(&manifest.build.output_dir);
        
        // Configure main file
        let main_file = if let Some(bin) = manifest.bin.first() {
            bin.path.as_ref().map(|p| project_path.join(p))
        } else if let Some(lib) = &manifest.lib {
            lib.path.as_ref().map(|p| project_path.join(p))
        } else {
            None
        };
        
        let config = ProjectConfig {
            name: manifest.package.name.clone(),
            version: manifest.package.version.clone(),
            root: project_path.to_path_buf(),
            source_dirs: build_config.source_dirs.clone(),
            main_file,
            dependencies: manifest.dependencies.clone(),
            build_config,
        };
        
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
            "workspace".to_string(),
        ]
    }

    /// Get template description
    pub fn get_template_description(template_name: &str) -> Option<String> {
        match template_name {
            "bin" => Some("A CURSED binary project template".to_string()),
            "lib" => Some("A CURSED library project template".to_string()),
            "workspace" => Some("A CURSED workspace template".to_string()),
            _ => None,
        }
    }
}

/// Project configuration utilities
impl ProjectConfig {
    /// Save configuration to file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let manifest = PackageManifest {
            package: PackageMetadata {
                name: self.name.clone(),
                version: self.version.clone(),
                description: None,
                authors: None,
                license: None,
                repository: None,
                keywords: None,
                categories: None,
                edition: default_edition(),
            },
            dependencies: self.dependencies.clone(),
            dev_dependencies: HashMap::new(),
            build: BuildSettings {
                source_dirs: self.source_dirs.iter()
                    .map(|p| p.to_string_lossy().to_string())
                    .collect(),
                output_dir: self.build_config.output_dir.to_string_lossy().to_string(),
                optimization: "2".to_string(),
                debug: self.build_config.debug_info,
                incremental: self.build_config.incremental,
            },
            lib: None,
            bin: vec![],
        };
        
        let manifest_content = toml::to_string_pretty(&manifest).map_err(|e| CursedError::General(format!("Failed to serialize manifest: {}", e)))?;
        fs::write(path.as_ref().join("CursedPackage.toml"), manifest_content)?;
        
        Ok(())
    }

    /// Validate project configuration
    pub fn validate(&self) -> Result<()> {
        // Check if root directory exists
        if !self.root.exists() {
            return Err(CursedError::General(format!(
                "Project root does not exist: {}",
                self.root.display()
            )));
        }
        
        // Check if source directories exist
        for source_dir in &self.source_dirs {
            if !source_dir.exists() {
                return Err(CursedError::General(format!(
                    "Source directory does not exist: {}",
                    source_dir.display()
                )));
            }
        }
        
        // Check if main file exists
        if let Some(main_file) = &self.main_file {
            if !main_file.exists() {
                return Err(CursedError::General(format!(
                    "Main file does not exist: {}",
                    main_file.display()
                )));
            }
        }
        
        Ok(())
    }
}
