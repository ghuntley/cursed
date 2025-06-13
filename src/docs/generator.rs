//! Documentation Generator
//! 
//! Core documentation generation functionality for the CURSED programming language.
//! Supports multiple output formats and comprehensive documentation extraction.

use crate::ast::*;
use crate::error::{Error, SourceLocation};
use crate::lexer::{Lexer, Token, TokenType};
use crate::parser::Parser;
use std::collections::{HashMap, BTreeMap};
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};

/// Configuration for documentation generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocGeneratorConfig {
    /// Output directory for generated documentation
    pub output_dir: PathBuf,
    /// Documentation format (html, markdown, json)
    pub format: DocFormat,
    /// Include source code examples
    pub include_examples: bool,
    /// Include private items
    pub include_private: bool,
    /// Generate cross-references
    pub generate_cross_refs: bool,
    /// Custom CSS for HTML output
    pub custom_css: Option<String>,
    /// Custom template directory
    pub template_dir: Option<PathBuf>,
    /// Project title
    pub title: String,
    /// Project description
    pub description: Option<String>,
    /// Project version
    pub version: Option<String>,
    /// Author information
    pub authors: Vec<String>,
    /// Base URL for linking
    pub base_url: Option<String>,
}

impl Default for DocGeneratorConfig {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("docs"),
            format: DocFormat::Html,
            include_examples: true,
            include_private: false,
            generate_cross_refs: true,
            custom_css: None,
            template_dir: None,
            title: "CURSED Documentation".to_string(),
            description: None,
            version: None,
            authors: Vec::new(),
            base_url: None,
        }
    }
}

/// Supported documentation formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocFormat {
    Html,
    Markdown,
    Json,
}

impl std::fmt::Display for DocFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocFormat::Html => write!(f, "html"),
            DocFormat::Markdown => write!(f, "markdown"),
            DocFormat::Json => write!(f, "json"),
        }
    }
}

impl std::str::FromStr for DocFormat {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "html" => Ok(DocFormat::Html),
            "markdown" | "md" => Ok(DocFormat::Markdown),
            "json" => Ok(DocFormat::Json),
            _ => Err(format!("Unsupported format: {}", s)),
        }
    }
}

/// Main documentation generator
pub struct DocumentationGenerator {
    config: DocGeneratorConfig,
    extracted_docs: Vec<ExtractedDocumentation>,
    cross_references: HashMap<String, Vec<CrossReference>>,
    search_index: Vec<SearchIndexEntry>,
}

impl DocumentationGenerator {
    /// Create a new documentation generator
    pub fn new(config: DocGeneratorConfig) -> Self {
        Self {
            config,
            extracted_docs: Vec::new(),
            cross_references: HashMap::new(),
            search_index: Vec::new(),
        }
    }

    /// Generate documentation from source files
    pub fn generate_from_files(&mut self, source_files: Vec<PathBuf>) -> Result<(), Error> {
        // Extract documentation from all source files
        for file_path in source_files {
            let documentation = self.extract_from_file(&file_path)?;
            self.extracted_docs.push(documentation);
        }

        // Build cross-references if enabled
        if self.config.generate_cross_refs {
            self.build_cross_references();
        }

        // Build search index
        self.build_search_index();

        // Generate output based on format
        match self.config.format {
            DocFormat::Html => self.generate_html_output(),
            DocFormat::Markdown => self.generate_markdown_output(),
            DocFormat::Json => self.generate_json_output(),
        }
    }

    /// Generate documentation from source directory
    pub fn generate_from_directory(&mut self, source_dir: &Path) -> Result<(), Error> {
        let source_files = self.find_cursed_files(source_dir)?;
        self.generate_from_files(source_files)
    }

    /// Extract documentation from a single file
    fn extract_from_file(&self, file_path: &Path) -> Result<ExtractedDocumentation, Error> {
        let source = fs::read_to_string(file_path)
            .map_err(|e| Error::Io(e))?;

        let extractor = DocumentationExtractor::new();
        extractor.extract_from_source(&source, file_path)
    }

    /// Find all CURSED source files in directory
    fn find_cursed_files(&self, dir: &Path) -> Result<Vec<PathBuf>, Error> {
        let mut files = Vec::new();
        
        fn walk_dir(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), Error> {
            for entry in fs::read_dir(dir).map_err(Error::Io)? {
                let entry = entry.map_err(Error::Io)?;
                let path = entry.path();
                
                if path.is_dir() {
                    walk_dir(&path, files)?;
                } else if let Some(ext) = path.extension() {
                    if ext == "csd" {
                        files.push(path);
                    }
                }
            }
            Ok(())
        }
        
        walk_dir(dir, &mut files)?;
        Ok(files)
    }

    /// Build cross-references between documentation items
    fn build_cross_references(&mut self) {
        let mut refs = HashMap::new();
        
        // Build symbol table
        let mut symbols = HashMap::new();
        for doc in &self.extracted_docs {
            for item in &doc.items {
                symbols.insert(item.name.clone(), item.clone());
            }
        }
        
        // Find references in each documentation item
        for doc in &self.extracted_docs {
            for item in &doc.items {
                let item_refs = self.find_references_in_item(item, &symbols);
                if !item_refs.is_empty() {
                    refs.insert(item.name.clone(), item_refs);
                }
            }
        }
        
        self.cross_references = refs;
    }

    /// Find references in a documentation item
    fn find_references_in_item(
        &self,
        item: &DocumentationItem,
        symbols: &HashMap<String, DocumentationItem>
    ) -> Vec<CrossReference> {
        let mut references = Vec::new();
        
        // Search in description
        for word in item.description.split_whitespace() {
            let clean_word = word.trim_matches(&['.', ',', '(', ')', '[', ']', '{', '}'][..]);
            if symbols.contains_key(clean_word) {
                references.push(CrossReference {
                    target: clean_word.to_string(),
                    context: format!("Referenced in {}", item.name),
                    location: item.location.clone(),
                });
            }
        }
        
        // Search in type signatures
        if let Some(ref signature) = item.signature {
            for word in signature.split_whitespace() {
                let clean_word = word.trim_matches(&['.', ',', '(', ')', '[', ']', '{', '}'][..]);
                if symbols.contains_key(clean_word) {
                    references.push(CrossReference {
                        target: clean_word.to_string(),
                        context: format!("Type reference in {}", item.name),
                        location: item.location.clone(),
                    });
                }
            }
        }
        
        references
    }

    /// Build search index for generated documentation
    fn build_search_index(&mut self) {
        let mut index = Vec::new();
        
        for doc in &self.extracted_docs {
            for item in &doc.items {
                index.push(SearchIndexEntry {
                    name: item.name.clone(),
                    kind: item.kind.clone(),
                    description: item.summary.clone(),
                    module: doc.module_name.clone(),
                    url: self.generate_item_url(item),
                    keywords: self.extract_keywords(item),
                });
            }
        }
        
        // Sort by name for better search performance
        index.sort_by(|a, b| a.name.cmp(&b.name));
        
        self.search_index = index;
    }

    /// Generate URL for a documentation item
    fn generate_item_url(&self, item: &DocumentationItem) -> String {
        match self.config.format {
            DocFormat::Html => {
                format!("{}.html#{}", item.module.replace("::", "_"), item.name.to_lowercase())
            }
            DocFormat::Markdown => {
                format!("{}.md#{}", item.module.replace("::", "_"), item.name.to_lowercase())
            }
            DocFormat::Json => {
                format!("{}#{}", item.module.replace("::", "_"), item.name)
            }
        }
    }

    /// Extract keywords from documentation item
    fn extract_keywords(&self, item: &DocumentationItem) -> Vec<String> {
        let mut keywords = vec![item.name.clone(), item.kind.to_string()];
        
        // Add words from summary
        keywords.extend(
            item.summary
                .split_whitespace()
                .filter(|w| w.len() > 3)
                .map(|w| w.to_lowercase())
        );
        
        // Add module components
        keywords.extend(
            item.module
                .split("::")
                .map(|w| w.to_lowercase())
        );
        
        // Remove duplicates and sort
        keywords.sort();
        keywords.dedup();
        
        keywords
    }

    /// Generate HTML documentation output
    fn generate_html_output(&self) -> Result<(), Error> {
        fs::create_dir_all(&self.config.output_dir).map_err(Error::Io)?;
        
        let html_generator = HtmlGenerator::new(&self.config);
        
        // Generate main index page
        html_generator.generate_index_page(&self.extracted_docs, &self.config.output_dir)?;
        
        // Generate module pages
        for doc in &self.extracted_docs {
            html_generator.generate_module_page(doc, &self.config.output_dir)?;
        }
        
        // Generate search index
        html_generator.generate_search_index(&self.search_index, &self.config.output_dir)?;
        
        // Copy static assets
        html_generator.copy_static_assets(&self.config.output_dir)?;
        
        Ok(())
    }

    /// Generate Markdown documentation output
    fn generate_markdown_output(&self) -> Result<(), Error> {
        fs::create_dir_all(&self.config.output_dir).map_err(Error::Io)?;
        
        let markdown_generator = MarkdownGenerator::new(&self.config);
        
        // Generate main README
        markdown_generator.generate_readme(&self.extracted_docs, &self.config.output_dir)?;
        
        // Generate module documentation
        for doc in &self.extracted_docs {
            markdown_generator.generate_module_doc(doc, &self.config.output_dir)?;
        }
        
        Ok(())
    }

    /// Generate JSON documentation output
    fn generate_json_output(&self) -> Result<(), Error> {
        fs::create_dir_all(&self.config.output_dir).map_err(Error::Io)?;
        
        let json_generator = JsonGenerator::new(&self.config);
        
        // Generate comprehensive JSON documentation
        json_generator.generate_documentation(&self.extracted_docs, &self.config.output_dir)?;
        
        // Generate search index
        json_generator.generate_search_index(&self.search_index, &self.config.output_dir)?;
        
        Ok(())
    }
}

/// Documentation extracted from source code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedDocumentation {
    pub file_path: PathBuf,
    pub module_name: String,
    pub package_name: Option<String>,
    pub imports: Vec<String>,
    pub items: Vec<DocumentationItem>,
    pub source_info: SourceInfo,
}

/// Individual documentation item (function, struct, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationItem {
    pub name: String,
    pub kind: ItemKind,
    pub visibility: Visibility,
    pub module: String,
    pub summary: String,
    pub description: String,
    pub signature: Option<String>,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<String>,
    pub examples: Vec<Example>,
    pub tags: HashMap<String, Vec<String>>,
    pub location: SourceLocation,
    pub source_code: Option<String>,
}

/// Kind of documentation item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemKind {
    Function,
    Struct,
    Interface,
    Variable,
    Constant,
    Type,
    Module,
}

impl std::fmt::Display for ItemKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemKind::Function => write!(f, "function"),
            ItemKind::Struct => write!(f, "struct"),
            ItemKind::Interface => write!(f, "interface"),
            ItemKind::Variable => write!(f, "variable"),
            ItemKind::Constant => write!(f, "constant"),
            ItemKind::Type => write!(f, "type"),
            ItemKind::Module => write!(f, "module"),
        }
    }
}

/// Item visibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private,
}

/// Function parameter documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub type_name: Option<String>,
    pub description: String,
    pub default_value: Option<String>,
}

/// Code example
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Example {
    pub title: Option<String>,
    pub description: Option<String>,
    pub code: String,
    pub language: String,
    pub output: Option<String>,
}

/// Cross-reference between documentation items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossReference {
    pub target: String,
    pub context: String,
    pub location: SourceLocation,
}

/// Search index entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchIndexEntry {
    pub name: String,
    pub kind: ItemKind,
    pub description: String,
    pub module: String,
    pub url: String,
    pub keywords: Vec<String>,
}

/// Source file information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceInfo {
    pub file_size: u64,
    pub line_count: usize,
    pub last_modified: Option<std::time::SystemTime>,
    pub encoding: String,
}

/// Documentation extractor
pub struct DocumentationExtractor {
    // Future: Add configuration for extraction behavior
}

impl DocumentationExtractor {
    pub fn new() -> Self {
        Self {}
    }

    /// Extract documentation from source code
    pub fn extract_from_source(&self, source: &str, file_path: &Path) -> Result<ExtractedDocumentation, Error> {
        // Parse the source code
        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer)?;
        let program = parser.parse_program()?;

        // Extract module information
        let module_name = self.derive_module_name(file_path);
        let package_name = program.package_name.clone();

        // Extract imports
        let imports = program.imports.iter()
            .map(|imp| imp.path.clone())
            .collect();

        // Extract documentation items
        let mut items = Vec::new();
        for statement in &program.statements {
            if let Some(item) = self.extract_from_statement(statement, &module_name)? {
                items.push(item);
            }
        }

        // Gather source file information
        let source_info = self.gather_source_info(source, file_path)?;

        Ok(ExtractedDocumentation {
            file_path: file_path.to_path_buf(),
            module_name,
            package_name,
            imports,
            items,
            source_info,
        })
    }

    /// Derive module name from file path
    fn derive_module_name(&self, file_path: &Path) -> String {
        file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string()
    }

    /// Extract documentation from a statement
    fn extract_from_statement(&self, statement: &Statement, module: &str) -> Result<Option<DocumentationItem>, Error> {
        // For now, return empty documentation until we can properly match AST types
        // This is a simplified implementation that would need to be enhanced
        // based on the actual AST structure in the CURSED codebase
        
        // Create a default documentation item
        let location = SourceLocation { line: 1, column: 1, file: None };
        let item = DocumentationItem {
            name: "placeholder".to_string(),
            kind: ItemKind::Function,
            visibility: Visibility::Public,
            module: module.to_string(),
            summary: "Placeholder documentation".to_string(),
            description: "This is a placeholder until proper AST integration is completed".to_string(),
            signature: None,
            parameters: Vec::new(),
            return_type: None,
            examples: Vec::new(),
            tags: HashMap::new(),
            location,
            source_code: None,
        };
        
        Ok(Some(item))
    }

    /// Extract documentation from function declaration (simplified)
    fn extract_function_docs(&self, _func: &dyn std::any::Any, module: &str) -> Result<DocumentationItem, Error> {
        // Simplified implementation - would need proper AST integration
        let location = SourceLocation { line: 1, column: 1, file: None };
        
        Ok(DocumentationItem {
            name: "example_function".to_string(),
            kind: ItemKind::Function,
            visibility: Visibility::Public,
            module: module.to_string(),
            summary: "Example function documentation".to_string(),
            description: "This is placeholder documentation for a function".to_string(),
            signature: Some("slay example_function() -> void".to_string()),
            parameters: Vec::new(),
            return_type: Some("void".to_string()),
            examples: Vec::new(),
            tags: HashMap::new(),
            location,
            source_code: None,
        })
    }

    /// Extract documentation from struct declaration (simplified)
    fn extract_struct_docs(&self, _struct_stmt: &dyn std::any::Any, module: &str) -> Result<DocumentationItem, Error> {
        let location = SourceLocation { line: 1, column: 1, file: None };
        
        Ok(DocumentationItem {
            name: "example_struct".to_string(),
            kind: ItemKind::Struct,
            visibility: Visibility::Public,
            module: module.to_string(),
            summary: "Example struct documentation".to_string(),
            description: "This is placeholder documentation for a struct".to_string(),
            signature: Some("squad example_struct".to_string()),
            parameters: Vec::new(),
            return_type: None,
            examples: Vec::new(),
            tags: HashMap::new(),
            location,
            source_code: None,
        })
    }

    /// Extract documentation from interface declaration (simplified)
    fn extract_interface_docs(&self, _interface_stmt: &dyn std::any::Any, module: &str) -> Result<DocumentationItem, Error> {
        let location = SourceLocation { line: 1, column: 1, file: None };
        
        Ok(DocumentationItem {
            name: "example_interface".to_string(),
            kind: ItemKind::Interface,
            visibility: Visibility::Public,
            module: module.to_string(),
            summary: "Example interface documentation".to_string(),
            description: "This is placeholder documentation for an interface".to_string(),
            signature: Some("collab example_interface".to_string()),
            parameters: Vec::new(),
            return_type: None,
            examples: Vec::new(),
            tags: HashMap::new(),
            location,
            source_code: None,
        })
    }

    /// Extract documentation from variable declaration (simplified)
    fn extract_variable_docs(&self, _var: &dyn std::any::Any, module: &str) -> Result<DocumentationItem, Error> {
        let location = SourceLocation { line: 1, column: 1, file: None };
        
        Ok(DocumentationItem {
            name: "example_variable".to_string(),
            kind: ItemKind::Variable,
            visibility: Visibility::Private,
            module: module.to_string(),
            summary: "Example variable documentation".to_string(),
            description: "This is placeholder documentation for a variable".to_string(),
            signature: Some("sus example_variable".to_string()),
            parameters: Vec::new(),
            return_type: Some("i32".to_string()),
            examples: Vec::new(),
            tags: HashMap::new(),
            location,
            source_code: None,
        })
    }

    /// Extract documentation from constant declaration (simplified)
    fn extract_constant_docs(&self, _const_stmt: &dyn std::any::Any, module: &str) -> Result<DocumentationItem, Error> {
        let location = SourceLocation { line: 1, column: 1, file: None };
        
        Ok(DocumentationItem {
            name: "example_constant".to_string(),
            kind: ItemKind::Constant,
            visibility: Visibility::Public,
            module: module.to_string(),
            summary: "Example constant documentation".to_string(),
            description: "This is placeholder documentation for a constant".to_string(),
            signature: Some("facts example_constant".to_string()),
            parameters: Vec::new(),
            return_type: Some("string".to_string()),
            examples: Vec::new(),
            tags: HashMap::new(),
            location,
            source_code: None,
        })
    }

    /// Extract documentation comments (simplified implementation)
    fn extract_doc_comments(&self, _location: &SourceLocation) -> Result<(String, String, HashMap<String, Vec<String>>, Vec<Example>), Error> {
        // In a real implementation, this would parse doc comments from the token stream
        // For now, return empty values
        Ok((
            String::new(),
            String::new(),
            HashMap::new(),
            Vec::new(),
        ))
    }

    /// Gather source file information
    fn gather_source_info(&self, source: &str, file_path: &Path) -> Result<SourceInfo, Error> {
        let file_size = source.len() as u64;
        let line_count = source.lines().count();
        
        let last_modified = fs::metadata(file_path)
            .ok()
            .and_then(|meta| meta.modified().ok());

        Ok(SourceInfo {
            file_size,
            line_count,
            last_modified,
            encoding: "UTF-8".to_string(),
        })
    }
}

/// HTML documentation generator
mod html_generator;
/// Markdown documentation generator  
mod markdown_generator;
/// JSON documentation generator
mod json_generator;

// Re-export generators
use html_generator::HtmlGenerator;
use markdown_generator::MarkdownGenerator;
use json_generator::JsonGenerator;
