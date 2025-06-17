/// Documentation generation for CURSED
/// 
/// Comprehensive documentation generation system supporting HTML, Markdown, and JSON output formats.
/// Features include cross-reference generation, search functionality, and comprehensive AST extraction.

// External dependencies
use serde::{Deserialize, Serialize};

// Re-export main components
pub mod generator;
pub mod html_generator;
pub mod markdown_generator; 
pub mod json_generator;
pub mod xml_generator;
pub mod comment_parser;
pub mod api_extractor;
pub mod examples;
pub mod cli;
pub mod publisher;
pub mod server;
pub mod registry;
pub mod testing;

// Enhanced documentation system modules
pub mod coverage_analyzer;
pub mod advanced_examples;
pub mod enhanced_output;
pub mod cross_reference;
pub mod quality_system;

// Re-export main generator
pub use generator::{DocumentationGenerator, DocGeneratorConfig, DocFormat};

// Re-export public API
pub use generator::{
    ExtractedDocumentation,
    DocumentationItem, ItemKind, Visibility, Parameter, Example, CrossReference,
    SearchIndexEntry, SourceInfo, DocumentationExtractor
};

pub use comment_parser::{CommentParser, ParsedDocumentation};
pub use api_extractor::ApiExtractor;
pub use examples::ExampleGenerator;
pub use cli::DocsCommand;
pub use publisher::{DocumentationPublisher, PublishConfig, PublishTarget, PublicationMetadata};
pub use server::{DocumentationServer, ServerConfig, SearchQuery, SearchResponse};
pub use registry::{DocumentationRegistry, RegistryConfig, PackageDocumentation, RegistrySearchQuery};
pub use testing::{DocumentationTester, TestingConfig, TestResults, TestIssue};

// Enhanced system exports
pub use coverage_analyzer::{CoverageAnalyzer, CoverageConfig, CoverageReport, CoverageStatistics};
pub use advanced_examples::{AdvancedExampleGenerator, ExampleConfig, ExamplesDatabase, ExtractedExample};
pub use enhanced_output::{EnhancedOutputGenerator, OutputConfig, GenerationResults, HtmlGenerationResult};
pub use cross_reference::{CrossReferenceAnalyzer, CrossReferenceConfig, CrossReferenceResult, DependencyGraph};
pub use quality_system::{DocumentationQualityAnalyzer, QualityConfig, QualityAnalysisResult, QualityRating};

use crate::error::{Error, SourceLocation};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

// DocumentationGenerator is re-exported from generator module above

// Legacy implementations removed - use generator::DocumentationGenerator directly

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
            if let Some(first_line) = self.raw_content.split("\n").next() {
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
            for line in self.raw_content.split("\n") {
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
            let lines: Vec<&str> = self.raw_content.split("\n").collect();
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

/// Generate XML documentation for a single file
pub fn generate_xml_docs(source_file: &Path, output_dir: &Path) -> Result<(), Error> {
    let mut config = generator::DocGeneratorConfig::default();
    config.output_dir = output_dir.to_path_buf();
    config.format = generator::DocFormat::Xml;
    
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
