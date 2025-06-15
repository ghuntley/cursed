//! Enhanced Documentation Generation System for CURSED
//! 
//! A comprehensive documentation generation system that integrates with the CURSED
//! AST, parser, and type system to generate accurate API documentation with multiple
//! output formats and advanced features.

// Main implementation
pub mod main;

use crate::error::{Error, SourceLocation};
use crate::ast::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, instrument, error, warn};

// Sub-modules
pub mod generator;
pub mod comment_parser;

// Re-export from main implementation
pub use main::*;

// Re-export main components
pub use generator::{
    DocumentationGenerator, DocGeneratorConfig, OutputFormat, DocumentationResult,
    DocumentationItem, ItemKind, FunctionDoc, TypeDoc, ModuleDoc, ExampleDoc
};

pub use comment_parser::{
    CommentParser, ParsedComment, DocTag, CodeExample, CommentParsingError
};

/// Enhanced documentation configuration with advanced features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationConfig {
    /// Source directories to scan for documentation
    pub source_dirs: Vec<PathBuf>,
    /// Output directory for generated documentation
    pub output_dir: PathBuf,
    /// Output formats to generate
    pub output_formats: Vec<OutputFormat>,
    /// Project metadata
    pub project: ProjectMetadata,
    /// Documentation generation options
    pub options: DocOptions,
    /// Styling and theming options
    pub styling: StylingConfig,
}

/// Project metadata for documentation
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
    /// Include source code in documentation
    pub include_source: bool,
    /// Generate cross-references
    pub generate_cross_refs: bool,
    /// Generate search index
    pub generate_search_index: bool,
    /// Include examples from comments
    pub include_examples: bool,
    /// Maximum recursion depth for type documentation
    pub max_type_depth: usize,
    /// Include dependency documentation
    pub include_dependencies: bool,
}

/// Styling configuration for HTML output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StylingConfig {
    /// Custom CSS files
    pub custom_css: Vec<PathBuf>,
    /// Custom template directory
    pub template_dir: Option<PathBuf>,
    /// Theme name (light, dark, auto)
    pub theme: String,
    /// Color scheme override
    pub colors: Option<HashMap<String, String>>,
    /// Custom favicon
    pub favicon: Option<PathBuf>,
    /// Custom logo
    pub logo: Option<PathBuf>,
}

impl Default for DocumentationConfig {
    fn default() -> Self {
        Self {
            source_dirs: vec![PathBuf::from("src"), PathBuf::from("lib")],
            output_dir: PathBuf::from("docs"),
            output_formats: vec![OutputFormat::Html, OutputFormat::Markdown],
            project: ProjectMetadata {
                name: "CURSED Project".to_string(),
                version: "0.1.0".to_string(),
                description: None,
                authors: Vec::new(),
                homepage: None,
                repository: None,
                license: None,
            },
            options: DocOptions {
                include_private: false,
                include_source: true,
                generate_cross_refs: true,
                generate_search_index: true,
                include_examples: true,
                max_type_depth: 10,
                include_dependencies: false,
            },
            styling: StylingConfig {
                custom_css: Vec::new(),
                template_dir: None,
                theme: "auto".to_string(),
                colors: None,
                favicon: None,
                logo: None,
            },
        }
    }
}

/// Documentation extraction result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedDocumentation {
    /// Source file path
    pub source_file: PathBuf,
    /// Module documentation
    pub module_doc: Option<ModuleDoc>,
    /// Functions found in the module
    pub functions: Vec<FunctionDoc>,
    /// Types found in the module
    pub types: Vec<TypeDoc>,
    /// Constants found in the module
    pub constants: Vec<DocumentationItem>,
    /// Variables found in the module
    pub variables: Vec<DocumentationItem>,
    /// Submodules
    pub submodules: Vec<ModuleDoc>,
    /// Source code (if included)
    pub source_code: Option<String>,
    /// Extraction metadata
    pub metadata: ExtractionMetadata,
}

/// Metadata about the documentation extraction process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionMetadata {
    /// When the documentation was extracted
    pub extracted_at: chrono::DateTime<chrono::Utc>,
    /// Version of the documentation generator
    pub generator_version: String,
    /// Number of items extracted
    pub item_count: usize,
    /// Any warnings encountered during extraction
    pub warnings: Vec<String>,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
}

/// Cross-reference information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossReference {
    /// The item being referenced
    pub target: String,
    /// Type of reference (function_call, type_usage, etc.)
    pub reference_type: String,
    /// Source location of the reference
    pub location: SourceLocation,
    /// Context around the reference
    pub context: Option<String>,
}

/// Search index entry for generated documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchIndexEntry {
    /// Unique ID for the item
    pub id: String,
    /// Item title/name
    pub title: String,
    /// Item type (function, type, module, etc.)
    pub item_type: String,
    /// Brief description
    pub description: String,
    /// Keywords for searching
    pub keywords: Vec<String>,
    /// URL/path to the documentation
    pub url: String,
    /// Module path
    pub module_path: String,
}

/// Main documentation coordinator
pub struct DocumentationSystem {
    config: DocumentationConfig,
    generator: DocumentationGenerator,
    comment_parser: CommentParser,
    extracted_docs: Vec<ExtractedDocumentation>,
    cross_references: HashMap<String, Vec<CrossReference>>,
    search_index: Vec<SearchIndexEntry>,
}

impl DocumentationSystem {
    /// Create a new documentation system with the given configuration
    #[instrument(skip(config))]
    pub fn new(config: DocumentationConfig) -> Result<Self, Error> {
        info!("Initializing documentation system");
        debug!("Output directory: {:?}", config.output_dir);
        debug!("Source directories: {:?}", config.source_dirs);
        
        let generator = DocumentationGenerator::new(config.clone())?;
        let comment_parser = CommentParser::new()?;
        
        Ok(Self {
            config,
            generator,
            comment_parser,
            extracted_docs: Vec::new(),
            cross_references: HashMap::new(),
            search_index: Vec::new(),
        })
    }

    /// Generate documentation for all configured source directories
    #[instrument(skip(self))]
    pub async fn generate_all(&mut self) -> Result<DocumentationResult, Error> {
        info!("Starting documentation generation for all sources");
        
        let start_time = std::time::Instant::now();
        let mut total_files = 0;
        let mut total_items = 0;
        
        // Extract documentation from all source directories
        for source_dir in &self.config.source_dirs {
            info!("Processing source directory: {:?}", source_dir);
            
            if !source_dir.exists() {
                warn!("Source directory does not exist: {:?}", source_dir);
                continue;
            }
            
            let (files_processed, items_extracted) = self.process_directory(source_dir).await?;
            total_files += files_processed;
            total_items += items_extracted;
        }
        
        // Generate cross-references
        if self.config.options.generate_cross_refs {
            info!("Generating cross-references");
            self.generate_cross_references().await?;
        }
        
        // Generate search index
        if self.config.options.generate_search_index {
            info!("Generating search index");
            self.generate_search_index().await?;
        }
        
        // Generate output in all requested formats
        let mut results = Vec::new();
        for format in &self.config.output_formats {
            info!("Generating {} documentation", format);
            let result = self.generator.generate_output(
                &self.extracted_docs,
                &self.cross_references,
                &self.search_index,
                format.clone(),
            ).await?;
            results.push(result);
        }
        
        let processing_time = start_time.elapsed();
        info!(
            "Documentation generation completed in {:?}: {} files, {} items",
            processing_time, total_files, total_items
        );
        
        Ok(DocumentationResult {
            files_processed: total_files,
            items_documented: total_items,
            output_files: results.into_iter().flatten().collect(),
            processing_time_ms: processing_time.as_millis() as u64,
            warnings: Vec::new(),
        })
    }

    /// Process a single directory for documentation
    #[instrument(skip(self))]
    async fn process_directory(&mut self, dir: &Path) -> Result<(usize, usize), Error> {
        let mut files_processed = 0;
        let mut items_extracted = 0;
        
        // Find all CURSED source files
        let source_files = self.find_source_files(dir)?;
        
        for file_path in source_files {
            debug!("Processing file: {:?}", file_path);
            
            match self.process_file(&file_path).await {
                Ok(extracted) => {
                    items_extracted += extracted.metadata.item_count;
                    self.extracted_docs.push(extracted);
                    files_processed += 1;
                }
                Err(e) => {
                    error!("Failed to process file {:?}: {}", file_path, e);
                    // Continue processing other files
                }
            }
        }
        
        Ok((files_processed, items_extracted))
    }

    /// Process a single CURSED source file
    #[instrument(skip(self))]
    async fn process_file(&mut self, file_path: &Path) -> Result<ExtractedDocumentation, Error> {
        let start_time = std::time::Instant::now();
        
        // Read source code
        let source_code = std::fs::read_to_string(file_path)
            .map_err(|e| Error::FileReadError(file_path.to_path_buf(), e.to_string()))?;
        
        // Parse the source code to get AST
        let mut lexer = crate::lexer::Lexer::new(&source_code);
        let tokens = lexer.tokenize()?;
        
        let mut parser = crate::parser::Parser::new(tokens);
        let ast = parser.parse()?;
        
        // Extract documentation using the generator
        let extracted = self.generator.extract_from_ast(&ast, file_path, &source_code).await?;
        
        let processing_time = start_time.elapsed();
        info!(
            "Processed {} in {:?}: {} items extracted",
            file_path.display(),
            processing_time,
            extracted.metadata.item_count
        );
        
        Ok(extracted)
    }

    /// Find all CURSED source files in a directory
    fn find_source_files(&self, dir: &Path) -> Result<Vec<PathBuf>, Error> {
        let mut source_files = Vec::new();
        
        if dir.is_file() {
            if let Some(extension) = dir.extension() {
                if extension == "csd" || extension == "cursed" {
                    source_files.push(dir.to_path_buf());
                }
            }
            return Ok(source_files);
        }
        
        fn collect_recursively(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), Error> {
            let entries = std::fs::read_dir(dir)
                .map_err(|e| Error::FileReadError(dir.to_path_buf(), e.to_string()))?;
            
            for entry in entries {
                let entry = entry.map_err(|e| Error::FileReadError(dir.to_path_buf(), e.to_string()))?;
                let path = entry.path();
                
                if path.is_dir() {
                    // Skip hidden directories and common ignore patterns
                    if let Some(dir_name) = path.file_name() {
                        let dir_name = dir_name.to_string_lossy();
                        if dir_name.starts_with('.') || 
                           dir_name == "target" || 
                           dir_name == "node_modules" ||
                           dir_name == "build" {
                            continue;
                        }
                    }
                    collect_recursively(&path, files)?;
                } else if let Some(extension) = path.extension() {
                    if extension == "csd" || extension == "cursed" {
                        files.push(path);
                    }
                }
            }
            
            Ok(())
        }
        
        collect_recursively(dir, &mut source_files)?;
        source_files.sort();
        
        debug!("Found {} source files in {:?}", source_files.len(), dir);
        Ok(source_files)
    }

    /// Generate cross-references between documentation items
    #[instrument(skip(self))]
    async fn generate_cross_references(&mut self) -> Result<(), Error> {
        info!("Generating cross-references");
        
        // Build a mapping of all documented items
        let mut item_map: HashMap<String, (PathBuf, String)> = HashMap::new();
        
        for doc in &self.extracted_docs {
            // Add functions
            for func in &doc.functions {
                let key = format!("{}::{}", doc.source_file.display(), func.name);
                item_map.insert(func.name.clone(), (doc.source_file.clone(), "function".to_string()));
                item_map.insert(key, (doc.source_file.clone(), "function".to_string()));
            }
            
            // Add types
            for type_doc in &doc.types {
                let key = format!("{}::{}", doc.source_file.display(), type_doc.name);
                item_map.insert(type_doc.name.clone(), (doc.source_file.clone(), "type".to_string()));
                item_map.insert(key, (doc.source_file.clone(), "type".to_string()));
            }
        }
        
        // Find references in source code and comments
        for doc in &self.extracted_docs {
            let mut references = Vec::new();
            
            // Scan function bodies and comments for references
            for func in &doc.functions {
                if let Some(ref body) = func.source_code {
                    for (name, (target_file, ref_type)) in &item_map {
                        if body.contains(name) && name != &func.name {
                            references.push(CrossReference {
                                target: name.clone(),
                                reference_type: ref_type.clone(),
                                location: func.location.clone(),
                                context: Some(format!("Function: {}", func.name)),
                            });
                        }
                    }
                }
            }
            
            let file_key = doc.source_file.to_string_lossy().to_string();
            self.cross_references.insert(file_key, references);
        }
        
        Ok(())
    }

    /// Generate search index for documentation
    #[instrument(skip(self))]
    async fn generate_search_index(&mut self) -> Result<(), Error> {
        info!("Generating search index");
        
        let mut index = Vec::new();
        
        for doc in &self.extracted_docs {
            let module_path = doc.source_file.to_string_lossy().to_string();
            
            // Index functions
            for func in &doc.functions {
                index.push(SearchIndexEntry {
                    id: format!("{}::{}", module_path, func.name),
                    title: func.name.clone(),
                    item_type: "function".to_string(),
                    description: func.description.clone().unwrap_or_default(),
                    keywords: self.generate_keywords(&func.name, &func.description),
                    url: format!("{}.html#{}", module_path, func.name),
                    module_path: module_path.clone(),
                });
            }
            
            // Index types
            for type_doc in &doc.types {
                index.push(SearchIndexEntry {
                    id: format!("{}::{}", module_path, type_doc.name),
                    title: type_doc.name.clone(),
                    item_type: "type".to_string(),
                    description: type_doc.description.clone().unwrap_or_default(),
                    keywords: self.generate_keywords(&type_doc.name, &type_doc.description),
                    url: format!("{}.html#{}", module_path, type_doc.name),
                    module_path: module_path.clone(),
                });
            }
        }
        
        self.search_index = index;
        info!("Generated search index with {} entries", self.search_index.len());
        
        Ok(())
    }

    /// Generate search keywords for an item
    fn generate_keywords(&self, name: &str, description: &Option<String>) -> Vec<String> {
        let mut keywords = Vec::new();
        
        // Add the name itself
        keywords.push(name.to_lowercase());
        
        // Add words from the name (split on underscores, camelCase, etc.)
        let name_words: Vec<String> = name
            .split(|c: char| !c.is_alphanumeric())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_lowercase())
            .collect();
        keywords.extend(name_words);
        
        // Add words from description
        if let Some(desc) = description {
            let desc_words: Vec<String> = desc
                .split_whitespace()
                .filter(|s| s.len() > 2) // Skip very short words
                .map(|s| s.to_lowercase().trim_matches(|c: char| !c.is_alphanumeric()).to_string())
                .filter(|s| !s.is_empty())
                .collect();
            keywords.extend(desc_words);
        }
        
        // Remove duplicates and sort
        keywords.sort();
        keywords.dedup();
        
        keywords
    }

    /// Get the current configuration
    pub fn config(&self) -> &DocumentationConfig {
        &self.config
    }

    /// Get extracted documentation
    pub fn extracted_docs(&self) -> &[ExtractedDocumentation] {
        &self.extracted_docs
    }

    /// Get generated cross-references
    pub fn cross_references(&self) -> &HashMap<String, Vec<CrossReference>> {
        &self.cross_references
    }

    /// Get search index
    pub fn search_index(&self) -> &[SearchIndexEntry] {
        &self.search_index
    }
}

/// Load configuration from file
pub fn load_config(config_path: &Path) -> Result<DocumentationConfig, Error> {
    let config_str = std::fs::read_to_string(config_path)
        .map_err(|e| Error::FileReadError(config_path.to_path_buf(), e.to_string()))?;
    
    // Try to parse as TOML first, then JSON
    if config_path.extension().map_or(false, |ext| ext == "toml") {
        toml::from_str(&config_str)
            .map_err(|e| Error::ConfigurationError(format!("Invalid TOML configuration: {}", e)))
    } else {
        serde_json::from_str(&config_str)
            .map_err(|e| Error::ConfigurationError(format!("Invalid JSON configuration: {}", e)))
    }
}

/// Save configuration to file
pub fn save_config(config: &DocumentationConfig, config_path: &Path) -> Result<(), Error> {
    let config_str = if config_path.extension().map_or(false, |ext| ext == "toml") {
        toml::to_string_pretty(config)
            .map_err(|e| Error::ConfigurationError(format!("Failed to serialize TOML: {}", e)))?
    } else {
        serde_json::to_string_pretty(config)
            .map_err(|e| Error::ConfigurationError(format!("Failed to serialize JSON: {}", e)))?
    };
    
    std::fs::write(config_path, config_str)
        .map_err(|e| Error::FileWriteError(config_path.to_path_buf(), e.to_string()))?;
    
    Ok(())
}

/// Create a default configuration file
pub fn create_default_config(config_path: &Path) -> Result<(), Error> {
    let config = DocumentationConfig::default();
    save_config(&config, config_path)
}
