// Documentation Generator
// 
// Core documentation generation functionality for the CURSED programming language.
// Supports multiple output formats and comprehensive documentation extraction.

use crate::ast::*;
use crate::error::{CursedError, SourceLocation};
use crate::lexer::{Lexer, Token, TokenType};
use crate::parser::Parser;
use crate::package_manager::Package;

use std::collections::{HashMap, BTreeMap};
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};

// For enhanced cross-reference detection
extern crate regex;

/// Configuration for documentation generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocGeneratorConfig {
    /// Output directory for generated documentation
    /// Documentation format (html, markdown, json)
    /// Include source code examples
    /// Include private items
    /// Generate cross-references
    /// Custom CSS for HTML output
    /// Custom template directory
    /// Project title
    /// Project description
    /// Project version
    /// Author information
    /// Base URL for linking
impl Default for DocGeneratorConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Supported documentation formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocFormat {
impl std::fmt::Display for DocFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
        }
    }
impl std::str::FromStr for DocFormat {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
        }
    }
/// Main documentation generator
pub struct DocumentationGenerator {
impl DocumentationGenerator {
    /// Create a new documentation generator
    pub fn new(config: DocGeneratorConfig) -> Self {
        Self {
        }
    }

    /// Generate documentation from source files
    pub fn generate_from_files(&mut self, source_files: Vec<PathBuf>) -> crate::error::Result<()> {
        // Extract documentation from all source files
        for file_path in source_files {
            let documentation = self.extract_from_file(&file_path)?;
            self.extracted_docs.push(documentation);
        // Build cross-references if enabled
        if self.config.generate_cross_refs {
            self.build_cross_references();
        // Build search index
        self.build_search_index();

        // Generate output based on format
        match self.config.format {
        }
    }

    /// Generate documentation from source directory
    pub fn generate_from_directory(&mut self, source_dir: &Path) -> crate::error::Result<()> {
        let source_files = self.find_cursed_files(source_dir)?;
        self.generate_from_files(source_files)
    /// Extract documentation from a single file
    fn extract_from_file(&self, file_path: &Path) -> crate::error::Result<()> {
        let source = fs::read_to_string(file_path)
            .map_err(|e| CursedError::Io(e))?;

        let extractor = DocumentationExtractor::new();
        extractor.extract_from_source(&source, file_path)
    /// Find all CURSED source files in directory
    fn find_cursed_files(&self, dir: &Path) -> crate::error::Result<()> {
        let mut files = Vec::new();
        
        fn walk_dir(dir: &Path, files: &mut Vec<PathBuf>) -> crate::error::Result<()> {
            for entry in fs::read_dir(dir).map_err(CursedError::Io)? {
                let entry = entry.map_err(CursedError::Io)?;
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
        walk_dir(dir, &mut files)?;
        Ok(files)
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
        self.cross_references = refs;
    /// Find references in a documentation item using enhanced semantic analysis
    fn find_references_in_item(
        symbols: &HashMap<String, DocumentationItem>
    ) -> Vec<CrossReference> {
        let mut references = Vec::new();
        
        // Enhanced text analysis for references
        references.extend(self.find_references_in_text(&item.description, &item.name, &item.location, symbols, "description"));
        references.extend(self.find_references_in_text(&item.summary, &item.name, &item.location, symbols, "summary"));
        
        // Search in type signatures with better parsing
        if let Some(ref signature) = item.signature {
            references.extend(self.find_references_in_signature(signature, &item.name, &item.location, symbols));
        // Search in examples
        for (i, example) in item.examples.iter().enumerate() {
            references.extend(self.find_references_in_text(&example.code, &item.name, &item.location, symbols, &format!("example_{}", i)));
        // Search in parameter types
        for param in &item.parameters {
            if let Some(ref type_name) = param.type_name {
                references.extend(self.find_references_in_type(type_name, &item.name, &item.location, symbols));
            }
        }
        
        // Search in return type
        if let Some(ref return_type) = item.return_type {
            references.extend(self.find_references_in_type(return_type, &item.name, &item.location, symbols));
        // Search in documentation tags
        for (tag_name, tag_values) in &item.tags {
            for tag_value in tag_values {
                references.extend(self.find_references_in_text(tag_value, &item.name, &item.location, symbols, &format!("tag_{}", tag_name)));
            }
        }
        
        references
    /// Find references in text with context-aware parsing
    fn find_references_in_text(
        context_type: &str
    ) -> Vec<CrossReference> {
        let mut references = Vec::new();
        
        // Split by various delimiters while preserving word boundaries
        let words: Vec<&str> = text
            .split(&[' ', '\t', '\n', '.', ',', '(', ')', '[', ']', '{', '}', '<', '>', ':', ';', '!', '?'])
            .filter(|w| !w.is_empty())
            .collect();
        
        for word in words {
            let clean_word = word.trim();
            
            // Skip very short words, common words, and keywords
            if clean_word.len() < 2 || self.is_common_word(clean_word) || self.is_cursed_keyword(clean_word) {
                continue;
            // Check for exact matches first
            if symbols.contains_key(clean_word) {
                references.push(CrossReference {
                });
            // Check for partial matches (e.g., for generic types like Vec<String>)
            for symbol_name in symbols.keys() {
                if clean_word.contains(symbol_name) && symbol_name.len() > 3 {
                    references.push(CrossReference {
                    });
                }
            }
        references
    /// Find references in type signatures with better parsing
    fn find_references_in_signature(
        symbols: &HashMap<String, DocumentationItem>
    ) -> Vec<CrossReference> {
        let mut references = Vec::new();
        
        // Parse common type patterns
        let type_patterns = vec![
            r":\s*(\w+)",           // parameter types
            r"->\s*(\w+)",          // return types  
            r"<(\w+)>",             // generic types
            r"Vec<(\w+)>",          // vector types
            r"Option<(\w+)>",       // option types
            r"Result<(\w+),\s*(\w+)>", // result types
        ];
        
        for pattern in type_patterns {
            if let Ok(regex) = regex::Regex::new(pattern) {
                for captures in regex.captures_iter(signature) {
                    for i in 1..captures.len() {
                        if let Some(type_name) = captures.get(i) {
                            let type_str = type_name.as_str();
                            if symbols.contains_key(type_str) {
                                references.push(CrossReference {
                                });
                            }
                        }
                    }
                }
            }
        }
        
        // Fallback to simple word-based parsing
        if references.is_empty() {
            references.extend(self.find_references_in_text(signature, source_name, location, symbols, "signature"));
        references
    /// Find references in type names
    fn find_references_in_type(
        symbols: &HashMap<String, DocumentationItem>
    ) -> Vec<CrossReference> {
        let mut references = Vec::new();
        
        // Extract base type from complex types
        let base_types = self.extract_base_types(type_name);
        
        for base_type in base_types {
            if symbols.contains_key(&base_type) {
                references.push(CrossReference {
                });
            }
        }
        
        references
    /// Extract base types from complex type expressions
    fn extract_base_types(&self, type_expr: &str) -> Vec<String> {
        let mut types = Vec::new();
        
        // Remove common type decorators
        let clean_expr = type_expr
            .replace("Vec<", "")
            .replace("Option<", "")
            .replace("Result<", "")
            .replace("&", "")
            .replace("mut ", "")
            .replace(">", "")
            .replace(",", " ");
        
        // Split by whitespace and collect valid type names
        for word in clean_expr.split_whitespace() {
            let trimmed = word.trim();
            if !trimmed.is_empty() && !self.is_primitive_type(trimmed) {
                types.push(trimmed.to_string());
            }
        }
        
        types
    /// Check if a word is a common English word that shouldn't be linked
    fn is_common_word(&self, word: &str) -> bool {
            "a" | "an" | "and" | "or" | "but" | "in" | "on" | "at" | "to" | "for" | "of" | "with" | 
            "by" | "from" | "up" | "about" | "into" | "through" | "during" | "before" | "after" |
            "above" | "below" | "between" | "among" | "through" | "during" | "if" | "unless" |
            "while" | "when" | "where" | "how" | "why" | "what" | "which" | "who" | "whom" |
            "this" | "that" | "these" | "those" | "the" | "is" | "are" | "was" | "were" | "be" |
            "been" | "being" | "have" | "has" | "had" | "do" | "does" | "did" | "will" | "would" |
            "could" | "should" | "may" | "might" | "must" | "can" | "cannot" | "not" | "no"
        )
    /// Check if a word is a CURSED keyword
    fn is_cursed_keyword(&self, word: &str) -> bool {
            "slay" | "yolo" | "sus" | "facts" | "periodt" | "lowkey" | "highkey" | "bestie" | 
            "flex" | "cap" | "nocap" | "fr" | "ong" | "bet" | "say" | "less" | "stan" | "vibe" |
            "yeet" | "squad" | "collab" | "mood" | "basic" | "salty" | "tea" | "spill" | "ghost" |
            "clap" | "back" | "left" | "right" | "up" | "down" | "fire" | "lit" | "fam" | "sis" |
            "bro" | "bestie" | "queen" | "king" | "icon" | "legend" | "main" | "character"
        )
    /// Check if a type is a primitive type
    fn is_primitive_type(&self, type_name: &str) -> bool {
            "i8" | "i16" | "i32" | "i64" | "i128" | "isize" |
            "u8" | "u16" | "u32" | "u64" | "u128" | "usize" |
            "f32" | "f64" | "bool" | "char" | "str" | "String" |
            "int" | "float" | "string" | "boolean" | "void" | "()"
        )
    /// Build search index for generated documentation
    fn build_search_index(&mut self) {
        let mut index = Vec::new();
        
        for doc in &self.extracted_docs {
            for item in &doc.items {
                index.push(SearchIndexEntry {
                });
            }
        }
        
        // Sort by name for better search performance
        index.sort_by(|a, b| a.name.cmp(&b.name));
        
        self.search_index = index;
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
    /// Generate HTML documentation output
    fn generate_html_output(&self) -> crate::error::Result<()> {
        fs::create_dir_all(&self.config.output_dir).map_err(CursedError::Io)?;
        
        let html_generator = HtmlGenerator::new(&self.config);
        
        // Generate main index page
        html_generator.generate_index_page(&self.extracted_docs, &self.config.output_dir)?;
        
        // Generate module pages
        for doc in &self.extracted_docs {
            html_generator.generate_module_page(doc, &self.config.output_dir)?;
        // Generate search index
        html_generator.generate_search_index(&self.search_index, &self.config.output_dir)?;
        
        // Copy static assets
        html_generator.copy_static_assets(&self.config.output_dir)?;
        
        Ok(())
    /// Generate Markdown documentation output
    fn generate_markdown_output(&self) -> crate::error::Result<()> {
        fs::create_dir_all(&self.config.output_dir).map_err(CursedError::Io)?;
        
        let markdown_generator = MarkdownGenerator::new(&self.config);
        
        // Generate main README
        markdown_generator.generate_readme(&self.extracted_docs, &self.config.output_dir)?;
        
        // Generate module documentation
        for doc in &self.extracted_docs {
            markdown_generator.generate_module_doc(doc, &self.config.output_dir)?;
        Ok(())
    /// Generate JSON documentation output
    fn generate_json_output(&self) -> crate::error::Result<()> {
        fs::create_dir_all(&self.config.output_dir).map_err(CursedError::Io)?;
        
        let json_generator = JsonGenerator::new(&self.config);
        
        // Generate comprehensive JSON documentation
        json_generator.generate_documentation(&self.extracted_docs, &self.config.output_dir)?;
        
        // Generate search index
        json_generator.generate_search_index(&self.search_index, &self.config.output_dir)?;
        
        Ok(())
    /// Generate HTML documentation for a package (async version for publisher)
    pub async fn generate_html_docs(&self, package: &Package, output_dir: &Path) -> crate::error::Result<()> {
        // Create output directory
        fs::create_dir_all(output_dir).map_err(CursedError::Io)?;
        
        // Create HTML generator with package-specific config
        let mut html_config = self.config.clone();
        html_config.output_dir = output_dir.to_path_buf();
        let html_generator = HtmlGenerator::new(&html_config);
        
        // Generate package documentation
        html_generator.generate_package_docs(package, &self.extracted_docs, output_dir)?;
        
        // Generate module documentation
        for doc in &self.extracted_docs {
            html_generator.generate_module_doc(doc, output_dir)?;
        // Generate index and navigation
        html_generator.generate_index(&self.extracted_docs, output_dir)?;
        html_generator.generate_navigation(&self.extracted_docs, output_dir)?;
        
        Ok(())
    /// Generate XML documentation output
    fn generate_xml_output(&self) -> crate::error::Result<()> {
        fs::create_dir_all(&self.config.output_dir).map_err(CursedError::Io)?;
        
        let xml_generator = XmlGenerator::new(&self.config);
        
        // Generate comprehensive XML documentation
        xml_generator.generate_documentation(&self.extracted_docs, &self.config.output_dir)?;
        
        // Generate search index
        xml_generator.generate_search_index(&self.search_index, &self.config.output_dir)?;
        
        Ok(())
    }
}

/// Documentation extracted from source code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedDocumentation {
/// Individual documentation item (function, struct, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationItem {
/// Kind of documentation item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemKind {
impl std::fmt::Display for ItemKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
        }
    }
/// Item visibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Visibility {
/// Function parameter documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
/// Code example
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Example {
/// Cross-reference between documentation items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossReference {
/// Search index entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchIndexEntry {
/// Source file information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceInfo {
/// Documentation extractor
pub struct DocumentationExtractor {
impl DocumentationExtractor {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn with_config(config: DocGeneratorConfig) -> Self {
        Self { config }
    }

    /// Extract documentation from source code
    pub fn extract_from_source(&self, source: &str, file_path: &Path) -> crate::error::Result<()> {
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
            if let Some(item) = self.extract_from_statement_with_docs(statement, &module_name, source)? {
                items.push(item);
            }
        }

        // Extract module-level documentation
        if let Ok(module_docs) = self.extract_module_level_docs(source, &module_name) {
            items.extend(module_docs);
        // Gather source file information
        let source_info = self.gather_source_info(source, file_path)?;

        Ok(ExtractedDocumentation {
        })
    /// Derive module name from file path
    fn derive_module_name(&self, file_path: &Path) -> String {
        file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string()
    /// Extract documentation from a statement with documentation comments
    fn extract_from_statement_with_docs(&self, statement: &Box<dyn Statement>, module: &str, source: &str) -> crate::error::Result<()> {
        use crate::ast::declarations::{FunctionStatement, SquadStatement, CollabStatement};
        use crate::ast::statements::variable::VariableStatement;
        use crate::ast::traits::Locatable;
        
        let any_stmt = statement.as_any();
        
        // Get location if available
        let location = if let Some(locatable) = any_stmt.downcast_ref::<dyn Locatable>() {
            locatable.location().unwrap_or(SourceLocation { line: 1, column: 1, file: None })
        } else {
            SourceLocation { line: 1, column: 1, file: None }
        
        // Function declarations (slay keyword)
        if let Some(func_stmt) = any_stmt.downcast_ref::<FunctionStatement>() {
            return Ok(Some(self.extract_function_documentation_with_docs(func_stmt, module, &location, source)?));
        // Struct declarations (squad keyword)
        if let Some(struct_stmt) = any_stmt.downcast_ref::<SquadStatement>() {
            return Ok(Some(self.extract_struct_documentation_with_docs(struct_stmt, module, &location, source)?));
        // Interface declarations (collab keyword)
        if let Some(interface_stmt) = any_stmt.downcast_ref::<CollabStatement>() {
            return Ok(Some(self.extract_interface_documentation_with_docs(interface_stmt, module, &location, source)?));
        // Variable declarations (sus/facts keywords)
        if let Some(var_stmt) = any_stmt.downcast_ref::<VariableStatement>() {
            return Ok(Some(self.extract_variable_documentation_with_docs(var_stmt, module, &location, source)?));
        // Package declarations (vibe keyword)
        if let Some(pkg_stmt) = any_stmt.downcast_ref::<PackageStatement>() {
            return Ok(Some(self.extract_package_documentation(pkg_stmt, module, &location, source)?));
        // Import statements (yeet keyword)
        if let Some(import_stmt) = any_stmt.downcast_ref::<ImportStatement>() {
            return Ok(Some(self.extract_import_documentation(import_stmt, module, &location, source)?));
        // Type aliases and constant declarations
        if let Some(item) = self.extract_type_alias_documentation(any_stmt, module, &location, source)? {
            return Ok(Some(item));
        if let Some(item) = self.extract_constant_documentation(any_stmt, module, &location, source)? {
            return Ok(Some(item));
        // For other statement types, return None (not documentable)
        Ok(None)
    /// Extract documentation from a statement (legacy method)
    fn extract_from_statement(&self, statement: &Box<dyn Statement>, module: &str) -> crate::error::Result<()> {
        // Fallback to old method without source context
        self.extract_from_statement_with_docs(statement, module, "")
    /// Extract documentation from function declaration with doc comments
    fn extract_function_documentation_with_docs(&self, func: &crate::ast::declarations::FunctionStatement, module: &str, location: &SourceLocation, source: &str) -> crate::error::Result<()> {
        use crate::ast::traits::Node;
        
        let func_name = func.name.value.clone();
        let signature = self.build_function_signature(func);
        
        // Extract parameter documentation
        let mut parameters = self.extract_function_parameters(&func.parameters)?;
        
        // Extract generic parameters if present
        if !func.type_parameters.is_empty() {
            let mut generic_params = self.extract_generic_parameters(&func.type_parameters)?;
            parameters.append(&mut generic_params);
        // Extract return type
        let return_type = func.return_type.as_ref().map(|rt| rt.string());
        
        // Extract documentation comments
        let (summary, description, tags, examples) = self.extract_doc_comments(source, location)?;
        
        // Use extracted documentation or fallback to auto-generated
        let final_summary = if summary.is_empty() {
            format!("Function {}", func_name)
        } else {
            summary
        
        let final_description = if description.is_empty() {
            format!("CURSED function declaration using the 'slay' keyword")
        } else {
            description
        
        // Extract source code if configured
        let source_code = if self.config.include_examples {
            Some(func.string())
        } else {
            None
        
        Ok(DocumentationItem {
            visibility: Visibility::Public, // Functions are typically public in CURSED
        })
    /// Extract documentation from function declaration (legacy method)
    fn extract_function_documentation(&self, func: &crate::ast::declarations::FunctionStatement, module: &str, location: &SourceLocation) -> crate::error::Result<()> {
        self.extract_function_documentation_with_docs(func, module, location, "")
    /// Extract documentation from struct declaration with doc comments
    fn extract_struct_documentation_with_docs(&self, struct_stmt: &crate::ast::declarations::SquadStatement, module: &str, location: &SourceLocation, source: &str) -> crate::error::Result<()> {
        use crate::ast::traits::Node;
        
        let struct_name = struct_stmt.name.value.clone();
        let signature = self.build_struct_signature(struct_stmt);
        
        // Extract field documentation
        let mut parameters = self.extract_struct_fields(&struct_stmt.fields)?;
        
        // Extract generic parameters if present
        if !struct_stmt.type_parameters.is_empty() {
            let mut generic_params = self.extract_generic_parameters(&struct_stmt.type_parameters)?;
            parameters.append(&mut generic_params);
        // Extract associated methods if available
        let mut associated_methods = self.extract_associated_methods(struct_stmt)?;
        parameters.append(&mut associated_methods);
        
        // Extract documentation comments
        let (summary, description, tags, examples) = self.extract_doc_comments(source, location)?;
        
        // Use extracted documentation or fallback to auto-generated
        let final_summary = if summary.is_empty() {
            format!("Struct {}", struct_name)
        } else {
            summary
        
        let final_description = if description.is_empty() {
            format!("CURSED struct declaration using the 'squad' keyword")
        } else {
            description
        
        // Extract source code if configured
        let source_code = if self.config.include_examples {
            Some(struct_stmt.string())
        } else {
            None
        
        Ok(DocumentationItem {
        })
    /// Extract documentation from struct declaration (legacy method)
    fn extract_struct_documentation(&self, struct_stmt: &crate::ast::declarations::SquadStatement, module: &str, location: &SourceLocation) -> crate::error::Result<()> {
        self.extract_struct_documentation_with_docs(struct_stmt, module, location, "")
    /// Extract documentation from interface declaration with doc comments
    fn extract_interface_documentation_with_docs(&self, interface_stmt: &crate::ast::declarations::CollabStatement, module: &str, location: &SourceLocation, source: &str) -> crate::error::Result<()> {
        use crate::ast::traits::Node;
        
        let interface_name = interface_stmt.name.value.clone();
        let signature = self.build_interface_signature(interface_stmt);
        
        // Extract method documentation as parameters
        let parameters = self.extract_interface_methods(&interface_stmt.methods)?;
        
        // Extract documentation comments
        let (summary, description, tags, examples) = self.extract_doc_comments(source, location)?;
        
        // Use extracted documentation or fallback to auto-generated
        let final_summary = if summary.is_empty() {
            format!("Interface {}", interface_name)
        } else {
            summary
        
        let final_description = if description.is_empty() {
            format!("CURSED interface declaration using the 'collab' keyword")
        } else {
            description
        
        // Extract source code if configured
        let source_code = if self.config.include_examples {
            Some(interface_stmt.string())
        } else {
            None
        
        Ok(DocumentationItem {
        })
    /// Extract documentation from interface declaration (legacy method)
    fn extract_interface_documentation(&self, interface_stmt: &crate::ast::declarations::CollabStatement, module: &str, location: &SourceLocation) -> crate::error::Result<()> {
        self.extract_interface_documentation_with_docs(interface_stmt, module, location, "")
    /// Extract documentation from variable declaration with doc comments
    fn extract_variable_documentation_with_docs(&self, var_stmt: &crate::ast::statements::variable::VariableStatement, module: &str, location: &SourceLocation, source: &str) -> crate::error::Result<()> {
        use crate::ast::traits::Node;
        
        let var_name = var_stmt.name.clone();
        let signature = self.build_variable_signature(var_stmt);
        
        // Determine if variable or constant based on mutability
        let kind = if var_stmt.is_mutable {
            ItemKind::Variable
        } else {
            ItemKind::Constant
        
        // Extract documentation comments
        let (summary, description, tags, examples) = self.extract_doc_comments(source, location)?;
        
        let keyword = if var_stmt.is_mutable { "sus" } else { "facts" };
        
        // Use extracted documentation or fallback to auto-generated
        let final_summary = if summary.is_empty() {
            format!("{} {}", if var_stmt.is_mutable { "Variable" } else { "Constant" }, var_name)
        } else {
            summary
        
        let final_description = if description.is_empty() {
                   if var_stmt.is_mutable { "variable" } else { "constant" }, keyword)
        } else {
            description
        
        // Extract source code if configured
        let source_code = if self.config.include_examples {
            Some(var_stmt.string())
        } else {
            None
        
        Ok(DocumentationItem {
            visibility: Visibility::Private, // Variables are typically private unless exported
        })
    /// Extract documentation from variable declaration (legacy method)
    fn extract_variable_documentation(&self, var_stmt: &crate::ast::statements::variable::VariableStatement, module: &str, location: &SourceLocation) -> crate::error::Result<()> {
        self.extract_variable_documentation_with_docs(var_stmt, module, location, "")
    /// Extract documentation from package declaration
    fn extract_package_documentation(&self, pkg_stmt: &PackageStatement, module: &str, location: &SourceLocation, source: &str) -> crate::error::Result<()> {
        use crate::ast::traits::Node;
        
        // Extract documentation comments
        let (summary, description, tags, examples) = self.extract_doc_comments(source, location)?;
        
        let final_summary = if summary.is_empty() {
            format!("Package {}", pkg_stmt.name)
        } else {
            summary
        
        let final_description = if description.is_empty() {
            format!("CURSED package declaration using the 'vibe' keyword")
        } else {
            description
        
        Ok(DocumentationItem {
        })
    /// Extract documentation from import declaration
    fn extract_import_documentation(&self, import_stmt: &ImportStatement, module: &str, location: &SourceLocation, source: &str) -> crate::error::Result<()> {
        use crate::ast::traits::Node;
        
        // Extract documentation comments
        let (summary, description, tags, examples) = self.extract_doc_comments(source, location)?;
        
        let import_name = import_stmt.alias.as_ref().unwrap_or(&import_stmt.path);
        
        let final_summary = if summary.is_empty() {
            format!("Import {}", import_name)
        } else {
            summary
        
        let final_description = if description.is_empty() {
            format!("CURSED import declaration using the 'yeet' keyword for path '{}'", import_stmt.path)
        } else {
            description
        
        Ok(DocumentationItem {
        })
    /// Extract documentation from type alias declarations
    fn extract_type_alias_documentation(&self, any_stmt: &dyn std::any::Any, module: &str, location: &SourceLocation, source: &str) -> crate::error::Result<()> {
        // In a full implementation, this would check for type alias statements
        // For now, we'll create a placeholder that can be extended when type alias AST nodes are available
        if let Some(_type_alias) = any_stmt.downcast_ref::<()>() { // Placeholder type - replace with actual type alias AST node
            let (summary, description, tags, examples) = self.extract_doc_comments(source, location)?;
            
            Ok(Some(DocumentationItem {
            }))
        } else {
            Ok(None)
        }
    }

    /// Extract documentation from constant declarations
    fn extract_constant_documentation(&self, any_stmt: &dyn std::any::Any, module: &str, location: &SourceLocation, source: &str) -> crate::error::Result<()> {
        // In a full implementation, this would check for constant declaration statements
        // For now, we'll create a placeholder that can be extended when constant AST nodes are available
        if let Some(_const_stmt) = any_stmt.downcast_ref::<()>() { // Placeholder type - replace with actual constant AST node
            let (summary, description, tags, examples) = self.extract_doc_comments(source, location)?;
            
            Ok(Some(DocumentationItem {
            }))
        } else {
            Ok(None)
        }
    }

    /// Extract module-level documentation
    fn extract_module_level_docs(&self, source: &str, module_name: &str) -> crate::error::Result<()> {
        use crate::docs::comment_parser::CommentParser;
        
        let parser = CommentParser::new()?;
        let all_docs = parser.extract_all_documentation(source)?;
        
        let mut module_items = Vec::new();
        
        // Look for module-level documentation at the top of the file
        for (location, parsed_doc) in all_docs {
            if location.line <= 10 && !parsed_doc.summary.is_empty() {
                // This appears to be module-level documentation
                module_items.push(DocumentationItem {
                });
            }
        }
        
        Ok(module_items)
    /// Extract documentation from constant declaration (simplified - legacy method)
    fn extract_constant_docs(&self, _const_stmt: &dyn std::any::Any, module: &str) -> crate::error::Result<()> {
        let location = SourceLocation { line: 1, column: 1, file: None };
        
        Ok(DocumentationItem {
        })
    /// Extract documentation comments from source at a specific location
    fn extract_doc_comments(&self, source: &str, location: &SourceLocation) -> crate::error::Result<()> {
        use crate::docs::comment_parser::CommentParser;
        
        let parser = CommentParser::new()?;
        let parsed_docs = parser.parse_item_documentation(source, location)?;
        
        Ok((
        ))
    /// Gather source file information
    fn gather_source_info(&self, source: &str, file_path: &Path) -> crate::error::Result<()> {
        let file_size = source.len() as u64;
        let line_count = source.split("\n").count();
        
        let last_modified = fs::metadata(file_path)
            .ok()
            .and_then(|meta| meta.modified().ok());

        Ok(SourceInfo {
        })
    /// Build function signature string
    fn build_function_signature(&self, func: &crate::ast::declarations::FunctionStatement) -> String {
        use crate::ast::traits::Node;
        
        let mut sig = String::new();
        sig.push_str("slay ");
        sig.push_str(&func.name.value);
        
        // Add type parameters if present
        if !func.type_parameters.is_empty() {
            sig.push('<');
            let type_params: Vec<String> = func.type_parameters.iter()
                .map(|p| p.string())
                .collect();
            sig.push_str(&type_params.join(", "));
            sig.push('>');
        // Add parameters
        sig.push('(');
        let param_strs: Vec<String> = func.parameters.iter()
            .map(|p| p.string())
            .collect();
        sig.push_str(&param_strs.join(", "));
        sig.push(')');
        
        // Add return type if present
        if let Some(return_type) = &func.return_type {
            sig.push_str(" -> ");
            sig.push_str(&return_type.string());
        sig
    /// Build struct signature string
    fn build_struct_signature(&self, struct_stmt: &crate::ast::declarations::SquadStatement) -> String {
        let mut sig = String::new();
        sig.push_str("squad ");
        sig.push_str(&struct_stmt.name.value);
        
        // Add type parameters if present
        if !struct_stmt.type_parameters.is_empty() {
            sig.push('<');
            let type_params: Vec<String> = struct_stmt.type_parameters.iter()
                .map(|p| p.string())
                .collect();
            sig.push_str(&type_params.join(", "));
            sig.push('>');
        sig
    /// Build interface signature string
    fn build_interface_signature(&self, interface_stmt: &crate::ast::declarations::CollabStatement) -> String {
        let mut sig = String::new();
        sig.push_str("collab ");
        sig.push_str(&interface_stmt.name.value);
        
        // Add type parameters if present
        if !interface_stmt.type_parameters.is_empty() {
            sig.push('<');
            let type_params: Vec<String> = interface_stmt.type_parameters.iter()
                .map(|p| p.string())
                .collect();
            sig.push_str(&type_params.join(", "));
            sig.push('>');
        sig
    /// Build variable signature string
    fn build_variable_signature(&self, var_stmt: &crate::ast::statements::variable::VariableStatement) -> String {
        use crate::ast::traits::Node;
        
        let keyword = if var_stmt.is_mutable { "sus" } else { "facts" };
        let mut sig = format!("{} {}", keyword, var_stmt.name);
        
        if let Some(var_type) = &var_stmt.var_type {
            sig.push_str(&format!(" {}", var_type));
        sig
    /// Extract function parameters with enhanced optional parameter detection
    fn extract_function_parameters(&self, parameters: &[crate::ast::expressions::Parameter]) -> crate::error::Result<()> {
        use crate::ast::traits::Node;
        
        let mut param_docs = Vec::new();
        
        for param in parameters {
            // Detect if parameter is optional
            let is_optional = param.default_value.is_some() || 
                param.param_type.as_ref().map_or(false, |t| self.is_optional_type(&t.string()));
            
            // Enhanced description with optional/required status
            let description = if is_optional && param.default_value.is_some() {
                format!("Optional parameter {} with default value", param.name)
            } else if is_optional {
                format!("Optional parameter {}", param.name)
            } else {
                format!("Required parameter {}", param.name)
            
            let param_doc = Parameter {
            param_docs.push(param_doc);
        Ok(param_docs)
    /// Detect if a type is optional (ends with ? or is Option<T>)
    fn is_optional_type(&self, type_str: &str) -> bool {
        type_str.ends_with('?') || type_str.starts_with("Option<")
    /// Extract struct fields with visibility detection and default values
    fn extract_struct_fields(&self, fields: &[crate::ast::declarations::FieldStatement]) -> crate::error::Result<()> {
        use crate::ast::traits::Node;
        
        let mut field_docs = Vec::new();
        
        for field in fields {
            // Detect field visibility (public fields typically start with uppercase in CURSED)
            let is_public = field.name.value.chars().next().map_or(false, |c| c.is_uppercase());
            let visibility = if is_public { "public" } else { "private" };
            
            // Enhanced field description with visibility
                field.type_name.value
            );
            
            // Extract default value if present (from field initialization)
            let default_value = field.default_value.as_ref().map(|v| v.string());
            
            let field_doc = Parameter {
            field_docs.push(field_doc);
        Ok(field_docs)
    /// Extract interface methods as parameters
    fn extract_interface_methods(&self, methods: &[crate::ast::declarations::MethodDeclaration]) -> crate::error::Result<()> {
        use crate::ast::traits::Node;
        
        let mut method_docs = Vec::new();
        
        for method in methods {
                method.parameters.iter()
                    .map(|p| p.string())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            
            let method_doc = Parameter {
            method_docs.push(method_doc);
        Ok(method_docs)
    /// Extract generic parameters from type parameter list
    fn extract_generic_parameters(&self, type_params: &[crate::ast::types::TypeParameter]) -> crate::error::Result<()> {
        use crate::ast::traits::Node;
        
        let mut generic_docs = Vec::new();
        
        for type_param in type_params {
            // Extract constraints if present
            let constraints = if !type_param.constraints.is_empty() {
                let constraint_names: Vec<String> = type_param.constraints.iter()
                    .map(|c| c.string())
                    .collect();
                Some(format!("where {}", constraint_names.join(" + ")))
            } else {
                None
            
            let description = if let Some(constraints) = &constraints {
                format!("Generic type parameter {} with constraints: {}", type_param.name.value, constraints)
            } else {
                format!("Generic type parameter {}", type_param.name.value)
            
            let generic_doc = Parameter {
            generic_docs.push(generic_doc);
        Ok(generic_docs)
    /// Extract submodules from module statements
    fn extract_submodules(&self, statements: &[Box<dyn Statement>]) -> crate::error::Result<()> {
        let mut submodules = Vec::new();
        let location = SourceLocation { line: 1, column: 1, file: None };
        
        for statement in statements {
            // Check for module declarations (when available in AST)
            // For now, we look for import statements that might indicate submodules
            // Check for import-like statements (simplified for now)
            let stmt_str = statement.to_string();
            if stmt_str.contains("import") || stmt_str.contains("yeet") {
                // If import is relative, it might be a submodule
                if stmt_str.contains("./") || stmt_str.contains("../") {
                    let module_name = "submodule".to_string(); // Simplified for now
                    let submodule = DocumentationItem {
                    submodules.push(submodule);
                }
            }
        Ok(submodules)
    /// Extract constants from constant declarations
    fn extract_constants(&self, statements: &[Box<dyn Statement>]) -> crate::error::Result<()> {
        let mut constants = Vec::new();
        let location = SourceLocation { line: 1, column: 1, file: None };
        
        for statement in statements {
            // Look for variable statements that are constants (facts keyword)
            if let Some(var_stmt) = statement.as_any().downcast_ref::<crate::ast::statements::variable::VariableStatement>() {
                if !var_stmt.is_mutable {
                    let constant = DocumentationItem {
                        source_code: if self.config.include_examples {
                            Some(format!("facts {} = <value>;", var_stmt.name))
                        } else {
                            None
                    constants.push(constant);
                }
            }
        Ok(constants)
    /// Extract interface implementations
    fn extract_interface_implementations(&self, struct_stmt: &crate::ast::declarations::SquadStatement) -> crate::error::Result<()> {
        let mut implementations = Vec::new();
        
        // Check if the struct has interface implementations (when available in AST)
        // For now, we'll look for interface-like field patterns or method blocks
        
        // Check field names for interface patterns (fields that end with interface-like suffixes)
        for field in &struct_stmt.fields {
            let field_name = &field.name.value;
            let type_name = &field.type_name.value;
            
            // If field type looks like an interface (capitalized, might have Interface suffix)
            if type_name.chars().next().map_or(false, |c| c.is_uppercase()) &&
               (type_name.ends_with("Interface") || type_name.ends_with("Trait")) {
                implementations.push(format!("implements {}", type_name));
            // Check for common interface patterns
            if field_name.starts_with("impl_") || field_name.contains("interface") {
                implementations.push(format!("implements {} via field {}", type_name, field_name));
            }
        }
        
        // Look for interface implementation comments or annotations in field descriptions
        // This would be enhanced when proper interface implementation syntax is added to AST
        
        Ok(implementations)
    /// Extract associated methods for structs
    fn extract_associated_methods(&self, struct_stmt: &crate::ast::declarations::SquadStatement) -> crate::error::Result<()> {
        let mut methods = Vec::new();
        
        // For now, we'll create placeholder methods based on common patterns
        // This will be enhanced when method implementation blocks are available in AST
        
        let struct_name = &struct_stmt.name.value;
        
        // Generate common constructor method
        let constructor = Parameter {
        methods.push(constructor);
        
        // Generate getter methods for each field
        for field in &struct_stmt.fields {
            let field_name = &field.name.value;
            let field_type = &field.type_name.value;
            
            // Only create getters for public fields (assuming capitalized names are public)
            if field_name.chars().next().map_or(false, |c| c.is_lowercase()) {
                let getter = Parameter {
                methods.push(getter);
                
                // Create setter for mutable fields (basic heuristic)
                if !field_type.starts_with("const") {
                    let setter = Parameter {
                    methods.push(setter);
                }
            }
        // Generate common utility methods
        if !struct_stmt.fields.is_empty() {
            let clone_method = Parameter {
            methods.push(clone_method);
            
            let debug_method = Parameter {
            methods.push(debug_method);
        Ok(methods)
    /// Extract documentation comments from source location
    fn extract_documentation_comments(&self, source: &str, location: &SourceLocation) -> crate::error::Result<()> {
        let lines = source.split("\n").collect::<Vec<_>>();
        let mut summary = String::new();
        let mut description = String::new();
        let mut tags = HashMap::new();
        let mut examples = Vec::new();
        
        // Start looking for comments before the location
        let start_line = if location.line > 10 { location.line - 10 } else { 1 };
        let end_line = location.line;
        
        let mut doc_lines = Vec::new();
        let mut in_doc_block = false;
        
        // Scan for documentation comments (/// style)
        for line_num in start_line..=end_line {
            if let Some(line) = lines.get((line_num - 1) as usize) {
                let trimmed = line.trim();
                
                if trimmed.starts_with("///") {
                    in_doc_block = true;
                    let content = trimmed.trim_start_matches("///").trim();
                    if !content.is_empty() {
                        doc_lines.push(content.to_string());
                    }
                } else if trimmed.starts_with("//!") {
                    in_doc_block = true;
                    let content = trimmed.trim_start_matches("//!").trim();
                    if !content.is_empty() {
                        doc_lines.push(content.to_string());
                    }
                } else if in_doc_block && (trimmed.is_empty() || trimmed.starts_with("//")) {
                    // Continue doc block through empty lines or regular comments
                    if trimmed.starts_with("//") && !trimmed.starts_with("///") {
                        let content = trimmed.trim_start_matches("//").trim();
                        if !content.is_empty() {
                            doc_lines.push(content.to_string());
                        }
                    }
                } else if in_doc_block && !trimmed.starts_with("//") {
                    // End of doc block
                    break;
                }
            }
        // Parse the documentation lines
        let mut current_example: Option<Example> = None;
        let mut in_example_code = false;
        let mut example_code = String::new();
        
        for (i, line) in doc_lines.iter().enumerate() {
            if line.starts_with('@') {
                // Handle tags
                if let Some(space_pos) = line.find(' ') {
                    let tag_name = &line[1..space_pos];
                    let tag_value = &line[space_pos + 1..];
                    
                    match tag_name {
                        "param" | "parameter" => {
                            tags.entry("parameters".to_string())
                                .or_insert_with(Vec::new)
                                .push(tag_value.to_string());
                        }
                        "return" | "returns" => {
                            tags.entry("returns".to_string())
                                .or_insert_with(Vec::new)
                                .push(tag_value.to_string());
                        }
                        "example" => {
                            // Start a new example
                            if let Some(example) = current_example.take() {
                                examples.push(example);
                            }
                            current_example = Some(Example {
                            });
                            in_example_code = false;
                        }
                        "code" => {
                            in_example_code = true;
                            example_code.clear();
                        }
                        "output" => {
                            if let Some(ref mut example) = current_example {
                                example.output = Some(tag_value.to_string());
                            }
                        }
                        _ => {
                            tags.entry(tag_name.to_string())
                                .or_insert_with(Vec::new)
                                .push(tag_value.to_string());
                        }
                    }
                }
            } else if in_example_code {
                // Accumulate example code
                example_code.push_str(line);
                example_code.push('\n');
            } else {
                // Regular documentation text
                if i == 0 && summary.is_empty() {
                    summary = line.clone();
                } else {
                    if !description.is_empty() {
                        description.push('\n');
                    }
                    description.push_str(line);
                }
            }
        // Finalize current example
        if let Some(mut example) = current_example {
            if !example_code.is_empty() {
                example.code = example_code.trim().to_string();
            }
            examples.push(example);
        // Use fallbacks if no documentation found
        if summary.is_empty() {
            summary = "Auto-generated summary".to_string();
        }
        if description.is_empty() {
            description = "Auto-generated description".to_string();
        Ok((summary, description, tags, examples))
    }
}

// Re-export generators from the docs module
use crate::docs::html_generator::HtmlGenerator;
use crate::docs::markdown_generator::MarkdownGenerator;
use crate::docs::json_generator::JsonGenerator;
use crate::docs::xml_generator::XmlGenerator;
