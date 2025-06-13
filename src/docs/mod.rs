/// Documentation generation for CURSED
//! 
//! Comprehensive documentation generation system supporting HTML, Markdown, and JSON output formats.
//! Features include cross-reference generation, search functionality, and comprehensive AST extraction.

// External dependencies
use serde::{Deserialize, Serialize};

// Re-export main components
pub mod generator;
pub mod html_generator;
pub mod markdown_generator;
pub mod json_generator;
pub mod comment_parser;
pub mod cli;

// Re-export public API
pub use generator::{
    DocumentationGenerator, DocGeneratorConfig, DocFormat, ExtractedDocumentation,
    DocumentationItem, ItemKind, Visibility, Parameter, Example, CrossReference,
    SearchIndexEntry, SourceInfo, DocumentationExtractor
};

pub use comment_parser::{CommentParser, ParsedDocumentation};
pub use cli::{add_doc_commands, handle_doc_command, generate_sample_config};

use crate::error::{Error, SourceLocation};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Legacy documentation generator for backward compatibility
pub struct DocumentationGenerator {
    inner: generator::DocumentationGenerator,
}

impl DocumentationGenerator {
    pub fn new() -> Self {
        let config = generator::DocGeneratorConfig::default();
        Self {
            inner: generator::DocumentationGenerator::new(config),
        }
    }
    
    /// Generate documentation from source directory
    pub fn generate_docs(&mut self, source_dir: &str, output_dir: &str) -> Result<(), Error> {
        let source_path = Path::new(source_dir);
        let mut config = generator::DocGeneratorConfig::default();
        config.output_dir = PathBuf::from(output_dir);
        
        self.inner = generator::DocumentationGenerator::new(config);
        self.inner.generate_from_directory(source_path)
    }

    /// Generate documentation with custom configuration
    pub fn generate_with_config(&mut self, config: generator::DocGeneratorConfig) -> Result<(), Error> {
        self.inner = generator::DocumentationGenerator::new(config.clone());
        
        // Use current directory as default source
        let source_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        self.inner.generate_from_directory(&source_dir)
    }

    /// Generate documentation from specific files
    pub fn generate_from_files(&mut self, files: Vec<PathBuf>, output_dir: &str) -> Result<(), Error> {
        let mut config = generator::DocGeneratorConfig::default();
        config.output_dir = PathBuf::from(output_dir);
        
        self.inner = generator::DocumentationGenerator::new(config);
        self.inner.generate_from_files(files)
    }
}

impl Default for DocumentationGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Legacy code example structure for backward compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExample {
    pub language: String,
    pub code: String,
}

impl From<CodeExample> for generator::Example {
    fn from(example: CodeExample) -> Self {
        Self {
            title: None,
            description: None,
            code: example.code,
            language: example.language,
            output: None,
        }
    }
}

impl From<generator::Example> for CodeExample {
    fn from(example: generator::Example) -> Self {
        Self {
            language: example.language,
            code: example.code,
        }
    }
}

/// Legacy documentation comment structure for backward compatibility
#[derive(Debug, Clone)]
pub struct DocumentationComment {
    pub summary: String,
    pub description: String,
    pub tags: HashMap<String, Vec<String>>,
    pub examples: Vec<CodeExample>,
    pub raw_content: String,
    pub location: SourceLocation,
}

impl DocumentationComment {
    pub fn new(location: SourceLocation) -> Self {
        Self {
            summary: String::new(),
            description: String::new(),
            tags: HashMap::new(),
            examples: Vec::new(),
            raw_content: String::new(),
            location,
        }
    }
    
    /// Parse content using the new comment parser
    pub fn parse_content(&mut self) {
        if let Ok(parser) = comment_parser::CommentParser::new() {
            if let Ok(parsed) = parser.parse_doc_content(&self.raw_content) {
                self.summary = parsed.summary;
                self.description = parsed.description;
                self.tags = parsed.tags;
                self.examples = parsed.examples.into_iter().map(|e| e.into()).collect();
            }
        } else {
            // Fallback to simple parsing
            if let Some(first_line) = self.raw_content.lines().next() {
                self.summary = first_line.trim_start_matches("///").trim().to_string();
            }
        }
    }
    
    /// Parse tags from content
    pub fn parse_tags(&mut self) {
        if let Ok(parser) = comment_parser::CommentParser::new() {
            if let Ok(parsed) = parser.parse_doc_content(&self.raw_content) {
                self.tags = parsed.tags;
            }
        } else {
            // Fallback to simple tag parsing
            for line in self.raw_content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("@") {
                    if let Some((tag_name, content)) = trimmed[1..].split_once(' ') {
                        self.tags.entry(tag_name.to_string())
                            .or_insert_with(Vec::new)
                            .push(content.to_string());
                    }
                }
            }
        }
    }
    
    pub fn get_examples(&self) -> &[CodeExample] {
        &self.examples
    }
    
    /// Parse examples from content
    pub fn parse_examples(&mut self) {
        if let Ok(parser) = comment_parser::CommentParser::new() {
            if let Ok(parsed) = parser.parse_doc_content(&self.raw_content) {
                self.examples = parsed.examples.into_iter().map(|e| e.into()).collect();
            }
        } else {
            // Fallback to simple example parsing
            let lines: Vec<&str> = self.raw_content.lines().collect();
            let mut in_code_block = false;
            let mut current_example = String::new();
            let mut current_language = String::new();
            
            for line in lines {
                if line.trim().starts_with("```") {
                    if in_code_block {
                        // End of code block
                        if !current_example.trim().is_empty() {
                            self.examples.push(CodeExample {
                                language: current_language.clone(),
                                code: current_example.trim().to_string(),
                            });
                        }
                        current_example.clear();
                        current_language.clear();
                        in_code_block = false;
                    } else {
                        // Start of code block
                        in_code_block = true;
                        // Extract language from ```language
                        current_language = line.trim().strip_prefix("```").unwrap_or("").to_string();
                    }
                } else if in_code_block {
                    current_example.push_str(line);
                    current_example.push('\n');
                }
            }
        }
    }
    
    pub fn validate(&self) -> Result<(), String> {
        if self.summary.is_empty() {
            return Err("Summary cannot be empty".to_string());
        }
        Ok(())
    }
    
    pub fn get_parameters(&self) -> Vec<String> {
        self.tags.get("param").cloned().unwrap_or_default()
    }
    
    pub fn get_return_documentation(&self) -> Option<String> {
        self.tags.get("return").and_then(|v| v.first()).cloned()
    }
}

/// Convert parsed documentation to legacy format
impl From<comment_parser::ParsedDocumentation> for DocumentationComment {
    fn from(parsed: comment_parser::ParsedDocumentation) -> Self {
        Self {
            summary: parsed.summary,
            description: parsed.description,
            tags: parsed.tags,
            examples: parsed.examples.into_iter().map(|e| e.into()).collect(),
            raw_content: String::new(),
            location: SourceLocation { line: 0, column: 0, file: None },
        }
    }
}

/// Convenience functions for documentation generation

/// Generate HTML documentation for a single file
pub fn generate_html_docs(source_file: &Path, output_dir: &Path) -> Result<(), Error> {
    let mut config = generator::DocGeneratorConfig::default();
    config.output_dir = output_dir.to_path_buf();
    config.format = generator::DocFormat::Html;
    
    let mut generator = generator::DocumentationGenerator::new(config);
    generator.generate_from_files(vec![source_file.to_path_buf()])
}

/// Generate Markdown documentation for a single file
pub fn generate_markdown_docs(source_file: &Path, output_dir: &Path) -> Result<(), Error> {
    let mut config = generator::DocGeneratorConfig::default();
    config.output_dir = output_dir.to_path_buf();
    config.format = generator::DocFormat::Markdown;
    
    let mut generator = generator::DocumentationGenerator::new(config);
    generator.generate_from_files(vec![source_file.to_path_buf()])
}

/// Generate JSON documentation for a single file
pub fn generate_json_docs(source_file: &Path, output_dir: &Path) -> Result<(), Error> {
    let mut config = generator::DocGeneratorConfig::default();
    config.output_dir = output_dir.to_path_buf();
    config.format = generator::DocFormat::Json;
    
    let mut generator = generator::DocumentationGenerator::new(config);
    generator.generate_from_files(vec![source_file.to_path_buf()])
}

/// Extract documentation from CURSED source code
pub fn extract_documentation(source: &str, file_path: &Path) -> Result<generator::ExtractedDocumentation, Error> {
    let extractor = generator::DocumentationExtractor::new();
    extractor.extract_from_source(source, file_path)
}

/// Parse documentation comments from source
pub fn parse_doc_comments(source: &str, location: &SourceLocation) -> Result<comment_parser::ParsedDocumentation, Error> {
    let parser = comment_parser::CommentParser::new()?;
    parser.parse_item_documentation(source, location)
}
