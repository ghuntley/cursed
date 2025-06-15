//! CURSED Documentation System
//! 
//! Complete documentation generation system for the CURSED programming language.
//! Provides unified interface for generating documentation in multiple formats.

use crate::docs::generator::{DocumentationGenerator, DocGeneratorConfig, DocFormat};
use crate::error::Error;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

/// Main documentation system
pub struct DocumentationSystem {
    config: DocumentationConfig,
}

/// Complete documentation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationConfig {
    /// Source directories to process
    pub source_dirs: Vec<PathBuf>,
    /// Output directory for documentation
    pub output_dir: PathBuf,
    /// Output formats to generate
    pub output_formats: Vec<OutputFormat>,
    /// Project metadata
    pub project: ProjectMetadata,
    /// Documentation options
    pub options: DocOptions,
    /// Styling configuration
    pub styling: StylingConfig,
}

/// Supported output formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    Html,
    Markdown,
    Json,
    Xml,
    LaTeX,
}

/// Project metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub authors: Vec<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub license: Option<String>,
}

/// Documentation generation options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocOptions {
    /// Include private items
    pub include_private: bool,
    /// Include source code
    pub include_source: bool,
    /// Generate cross-references
    pub generate_cross_refs: bool,
    /// Generate search index
    pub generate_search_index: bool,
    /// Include examples
    pub include_examples: bool,
    /// Maximum type recursion depth
    pub max_type_depth: usize,
    /// Include dependencies
    pub include_dependencies: bool,
}

/// Styling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StylingConfig {
    /// Custom CSS files
    pub custom_css: Vec<PathBuf>,
    /// Template directory
    pub template_dir: Option<PathBuf>,
    /// Theme name
    pub theme: String,
    /// Color overrides
    pub colors: Option<HashMap<String, String>>,
    /// Custom favicon
    pub favicon: Option<PathBuf>,
    /// Custom logo
    pub logo: Option<PathBuf>,
}

/// Documentation generation result
#[derive(Debug, Clone)]
pub struct DocumentationResult {
    /// Number of files processed
    pub files_processed: usize,
    /// Number of items documented
    pub items_documented: usize,
    /// Generated output files
    pub output_files: Vec<PathBuf>,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Warnings generated
    pub warnings: Vec<String>,
    /// Errors encountered (non-fatal)
    pub errors: Vec<String>,
}

impl Default for DocumentationConfig {
    fn default() -> Self {
        Self {
            source_dirs: vec![PathBuf::from(".")],
            output_dir: PathBuf::from("docs"),
            output_formats: vec![OutputFormat::Html],
            project: ProjectMetadata::default(),
            options: DocOptions::default(),
            styling: StylingConfig::default(),
        }
    }
}

impl Default for ProjectMetadata {
    fn default() -> Self {
        Self {
            name: "CURSED Project".to_string(),
            version: "0.1.0".to_string(),
            description: None,
            authors: Vec::new(),
            homepage: None,
            repository: None,
            license: None,
        }
    }
}

impl Default for DocOptions {
    fn default() -> Self {
        Self {
            include_private: false,
            include_source: true,
            generate_cross_refs: true,
            generate_search_index: true,
            include_examples: true,
            max_type_depth: 10,
            include_dependencies: false,
        }
    }
}

impl Default for StylingConfig {
    fn default() -> Self {
        Self {
            custom_css: Vec::new(),
            template_dir: None,
            theme: "auto".to_string(),
            colors: None,
            favicon: None,
            logo: None,
        }
    }
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Html => write!(f, "html"),
            OutputFormat::Markdown => write!(f, "markdown"),
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Xml => write!(f, "xml"),
            OutputFormat::LaTeX => write!(f, "latex"),
        }
    }
}

impl DocumentationSystem {
    /// Create a new documentation system
    pub fn new(config: DocumentationConfig) -> Result<Self, Error> {
        Ok(Self { config })
    }

    /// Generate documentation for all configured formats
    pub async fn generate_all(&mut self) -> Result<DocumentationResult, Error> {
        let start_time = Instant::now();
        let mut result = DocumentationResult {
            files_processed: 0,
            items_documented: 0,
            output_files: Vec::new(),
            processing_time_ms: 0,
            warnings: Vec::new(),
            errors: Vec::new(),
        };

        // Discover source files
        let source_files = self.discover_source_files()?;
        result.files_processed = source_files.len();

        // Generate documentation for each format
        for format in &self.config.output_formats {
            match self.generate_format(&source_files, format, &mut result).await {
                Ok(format_files) => {
                    result.output_files.extend(format_files);
                }
                Err(e) => {
                    result.errors.push(format!("Failed to generate {} documentation: {}", format, e));
                }
            }
        }

        result.processing_time_ms = start_time.elapsed().as_millis() as u64;
        Ok(result)
    }

    /// Generate documentation for a specific format
    async fn generate_format(
        &self,
        source_files: &[PathBuf],
        format: &OutputFormat,
        result: &mut DocumentationResult,
    ) -> Result<Vec<PathBuf>, Error> {
        let doc_config = self.build_doc_generator_config(format);
        let mut generator = DocumentationGenerator::new(doc_config);

        // Generate documentation
        generator.generate_from_files(source_files.to_vec())?;

        // TODO: Count documented items from generator results
        // For now, estimate based on files
        result.items_documented += source_files.len() * 5; // Rough estimate

        // Return list of generated files
        let output_dir = &self.config.output_dir;
        let mut generated_files = Vec::new();

        match format {
            OutputFormat::Html => {
                generated_files.push(output_dir.join("index.html"));
                for file in source_files {
                    if let Some(stem) = file.file_stem() {
                        generated_files.push(output_dir.join(format!("{}.html", stem.to_string_lossy())));
                    }
                }
            }
            OutputFormat::Markdown => {
                generated_files.push(output_dir.join("README.md"));
                for file in source_files {
                    if let Some(stem) = file.file_stem() {
                        generated_files.push(output_dir.join(format!("{}.md", stem.to_string_lossy())));
                    }
                }
            }
            OutputFormat::Json => {
                generated_files.push(output_dir.join("documentation.json"));
                generated_files.push(output_dir.join("api-index.json"));
                generated_files.push(output_dir.join("search-index.json"));
            }
            OutputFormat::Xml => {
                generated_files.push(output_dir.join("documentation.xml"));
                generated_files.push(output_dir.join("api_index.xml"));
                generated_files.push(output_dir.join("cursed_docs.dtd"));
                // Module files are generated dynamically based on source files
            }
            OutputFormat::LaTeX => {
                result.warnings.push("LaTeX format not yet implemented".to_string());
            }
        }

        Ok(generated_files)
    }

    /// Discover source files in configured directories
    fn discover_source_files(&self) -> Result<Vec<PathBuf>, Error> {
        let mut files = Vec::new();

        for source_dir in &self.config.source_dirs {
            self.scan_directory(source_dir, &mut files)?;
        }

        files.sort();
        Ok(files)
    }

    /// Recursively scan directory for CURSED source files
    fn scan_directory(&self, dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), Error> {
        if !dir.exists() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Source directory not found: {}", dir.display()),
            )));
        }

        for entry in std::fs::read_dir(dir).map_err(Error::Io)? {
            let entry = entry.map_err(Error::Io)?;
            let path = entry.path();

            if path.is_dir() {
                // Skip hidden directories and common ignore patterns
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if !name.starts_with('.') && name != "target" && name != "node_modules" {
                        self.scan_directory(&path, files)?;
                    }
                }
            } else if let Some(ext) = path.extension() {
                if ext == "csd" {
                    files.push(path);
                }
            }
        }

        Ok(())
    }

    /// Build DocGeneratorConfig for a specific format
    fn build_doc_generator_config(&self, format: &OutputFormat) -> DocGeneratorConfig {
        let doc_format = match format {
            OutputFormat::Html => DocFormat::Html,
            OutputFormat::Markdown => DocFormat::Markdown,
            OutputFormat::Json => DocFormat::Json,
            _ => DocFormat::Html, // Default fallback
        };

        DocGeneratorConfig {
            output_dir: self.config.output_dir.clone(),
            format: doc_format,
            include_examples: self.config.options.include_source,
            include_private: self.config.options.include_private,
            generate_cross_refs: self.config.options.generate_cross_refs,
            custom_css: self.config.styling.custom_css.first().map(|p| p.display().to_string()),
            template_dir: self.config.styling.template_dir.clone(),
            title: self.config.project.name.clone(),
            description: self.config.project.description.clone(),
            version: Some(self.config.project.version.clone()),
            authors: self.config.project.authors.clone(),
            base_url: self.config.project.homepage.clone(),
        }
    }
}

/// Load configuration from file
pub fn load_config(path: &Path) -> Result<DocumentationConfig, Error> {
    let content = std::fs::read_to_string(path).map_err(Error::Io)?;
    
    if path.extension().map_or(false, |ext| ext == "json") {
        serde_json::from_str(&content)
            .map_err(|e| Error::ConfigurationError(format!("Invalid JSON config: {}", e)))
    } else {
        // Default to TOML
        toml::from_str(&content)
            .map_err(|e| Error::ConfigurationError(format!("Invalid TOML config: {}", e)))
    }
}

/// Save configuration to file
pub fn save_config(config: &DocumentationConfig, path: &Path) -> Result<(), Error> {
    let content = if path.extension().map_or(false, |ext| ext == "json") {
        serde_json::to_string_pretty(config)
            .map_err(|e| Error::ConfigurationError(format!("Failed to serialize config: {}", e)))?
    } else {
        // Default to TOML
        toml::to_string_pretty(config)
            .map_err(|e| Error::ConfigurationError(format!("Failed to serialize config: {}", e)))?
    };

    std::fs::write(path, content).map_err(Error::Io)?;
    Ok(())
}

/// Create a default configuration file
pub fn create_default_config(path: &Path) -> Result<(), Error> {
    let config = DocumentationConfig::default();
    save_config(&config, path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = DocumentationConfig::default();
        
        assert_eq!(config.source_dirs, vec![PathBuf::from(".")]);
        assert_eq!(config.output_dir, PathBuf::from("docs"));
        assert_eq!(config.output_formats.len(), 1);
        assert_eq!(config.project.name, "CURSED Project");
        assert!(config.options.include_source);
        assert!(config.options.generate_cross_refs);
    }

    #[test]
    fn test_config_serialization() {
        let config = DocumentationConfig::default();
        
        // Test TOML serialization
        let toml_str = toml::to_string_pretty(&config).unwrap();
        let deserialized: DocumentationConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(config.project.name, deserialized.project.name);
        
        // Test JSON serialization
        let json_str = serde_json::to_string_pretty(&config).unwrap();
        let deserialized: DocumentationConfig = serde_json::from_str(&json_str).unwrap();
        assert_eq!(config.project.name, deserialized.project.name);
    }

    #[tokio::test]
    async fn test_documentation_system() {
        let temp_dir = TempDir::new().unwrap();
        let config = DocumentationConfig {
            source_dirs: vec![temp_dir.path().to_path_buf()],
            output_dir: temp_dir.path().join("docs"),
            ..Default::default()
        };

        let mut system = DocumentationSystem::new(config).unwrap();
        
        // Should handle empty directory gracefully
        let result = system.generate_all().await.unwrap();
        assert_eq!(result.files_processed, 0);
    }

    #[test]
    fn test_output_format_display() {
        assert_eq!(OutputFormat::Html.to_string(), "html");
        assert_eq!(OutputFormat::Markdown.to_string(), "markdown");
        assert_eq!(OutputFormat::Json.to_string(), "json");
        assert_eq!(OutputFormat::Xml.to_string(), "xml");
        assert_eq!(OutputFormat::LaTeX.to_string(), "latex");
    }
}
