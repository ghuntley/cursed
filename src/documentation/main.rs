// CURSED Documentation System
// 
// Complete documentation generation system for the CURSED programming language.
// Provides unified interface for generating documentation in multiple formats.

use crate::docs::generator::{DocumentationGenerator, DocGeneratorConfig, DocFormat};
use crate::error::CursedError;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

/// Main documentation system
pub struct DocumentationSystem {
/// Complete documentation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationConfig {
    /// Source directories to process
    /// Output directory for documentation
    /// Output formats to generate
    /// Project metadata
    /// Documentation options
    /// Styling configuration
/// Supported output formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
/// Project metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
/// Documentation generation options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocOptions {
    /// Include private items
    /// Include source code
    /// Generate cross-references
    /// Generate search index
    /// Include examples
    /// Maximum type recursion depth
    /// Include dependencies
/// Styling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StylingConfig {
    /// Custom CSS files
    /// Template directory
    /// Theme name
    /// Color overrides
    /// Custom favicon
    /// Custom logo
/// Documentation generation result
#[derive(Debug, Clone)]
pub struct DocumentationResult {
    /// Number of files processed
    /// Number of items documented
    /// Generated output files
    /// Processing time in milliseconds
    /// Warnings generated
    /// Errors encountered (non-fatal)
impl Default for DocumentationConfig {
    fn default() -> Self {
        Self {
        }
    }
impl Default for ProjectMetadata {
    fn default() -> Self {
        Self {
        }
    }
impl Default for DocOptions {
    fn default() -> Self {
        Self {
        }
    }
impl Default for StylingConfig {
    fn default() -> Self {
        Self {
        }
    }
impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
        }
    }
impl DocumentationSystem {
    /// Create a new documentation system
    pub fn new(config: DocumentationConfig) -> crate::error::Result<()> {
        Ok(Self { config })
    /// Generate documentation for all configured formats
    pub async fn generate_all(&mut self) -> crate::error::Result<()> {
        let start_time = Instant::now();
        let mut result = DocumentationResult {

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
        result.processing_time_ms = start_time.elapsed().as_millis() as u64;
        Ok(result)
    /// Generate documentation for a specific format
    async fn generate_format(
    ) -> crate::error::Result<()> {
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
    /// Discover source files in configured directories
    fn discover_source_files(&self) -> crate::error::Result<()> {
        let mut files = Vec::new();

        for source_dir in &self.config.source_dirs {
            self.scan_directory(source_dir, &mut files)?;
        files.sort();
        Ok(files)
    /// Recursively scan directory for CURSED source files
    fn scan_directory(&self, dir: &Path, files: &mut Vec<PathBuf>) -> crate::error::Result<()> {
        if !dir.exists() {
            return Err(CursedError::Io(std::io::Error::new(
            )));
        for entry in std::fs::read_dir(dir).map_err(CursedError::Io)? {
            let entry = entry.map_err(CursedError::Io)?;
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
        Ok(())
    /// Build DocGeneratorConfig for a specific format
    fn build_doc_generator_config(&self, format: &OutputFormat) -> DocGeneratorConfig {
        let doc_format = match format {
            _ => DocFormat::Html, // Default fallback

        DocGeneratorConfig {
        }
    }
/// Load configuration from file
pub fn load_config(path: &Path) -> crate::error::Result<()> {
    let content = std::fs::read_to_string(path).map_err(CursedError::Io)?;
    
    if path.extension().map_or(false, |ext| ext == "json") {
        serde_json::from_str(&content)
            .map_err(|e| CursedError::ConfigurationError(format!("Invalid JSON config: {}", e)))
    } else {
        // Default to TOML
        toml::from_str(&content)
            .map_err(|e| CursedError::ConfigurationError(format!("Invalid TOML config: {}", e)))
    }
}

/// Save configuration to file
pub fn save_config(config: &DocumentationConfig, path: &Path) -> crate::error::Result<()> {
    let content = if path.extension().map_or(false, |ext| ext == "json") {
        serde_json::to_string_pretty(config)
            .map_err(|e| CursedError::ConfigurationError(format!("Failed to serialize config: {}", e)))?
    } else {
        // Default to TOML
        toml::to_string_pretty(config)
            .map_err(|e| CursedError::ConfigurationError(format!("Failed to serialize config: {}", e)))?

    std::fs::write(path, content).map_err(CursedError::Io)?;
    Ok(())
/// Create a default configuration file
pub fn create_default_config(path: &Path) -> crate::error::Result<()> {
    let config = DocumentationConfig::default();
    save_config(&config, path)
