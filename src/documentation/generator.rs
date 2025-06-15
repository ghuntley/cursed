//! Documentation Generator Implementation
//! 
//! Core implementation of the CURSED documentation generation system with
//! support for multiple output formats, advanced features, and comprehensive
//! AST analysis.

use crate::ast::*;
use crate::error::{Error, SourceLocation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::{debug, info, instrument, warn};

/// Output formats supported by the documentation generator
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OutputFormat {
    Html,
    Markdown,
    Json,
    Xml,
    LaTeX,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Html => write!(f, "HTML"),
            OutputFormat::Markdown => write!(f, "Markdown"),
            OutputFormat::Json => write!(f, "JSON"),
            OutputFormat::Xml => write!(f, "XML"),
            OutputFormat::LaTeX => write!(f, "LaTeX"),
        }
    }
}

/// Result of documentation generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationResult {
    /// Number of files processed
    pub files_processed: usize,
    /// Number of items documented
    pub items_documented: usize,
    /// Generated output files
    pub output_files: Vec<PathBuf>,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Warnings encountered during generation
    pub warnings: Vec<String>,
}

/// Kind of documentation item
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ItemKind {
    Function,
    Type,
    Struct,
    Interface,
    Enum,
    Constant,
    Variable,
    Module,
    Namespace,
    Import,
}

/// Generic documentation item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationItem {
    /// Item name
    pub name: String,
    /// Item kind
    pub kind: ItemKind,
    /// Item description from comments
    pub description: Option<String>,
    /// Source location
    pub location: SourceLocation,
    /// Source code snippet
    pub source_code: Option<String>,
    /// Visibility (public, private)
    pub visibility: String,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Function documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDoc {
    /// Function name
    pub name: String,
    /// Function description
    pub description: Option<String>,
    /// Function parameters
    pub parameters: Vec<ParameterDoc>,
    /// Return type and description
    pub return_type: Option<TypeDoc>,
    /// Examples from documentation
    pub examples: Vec<ExampleDoc>,
    /// Source location
    pub location: SourceLocation,
    /// Source code
    pub source_code: Option<String>,
    /// Visibility
    pub visibility: String,
    /// Whether function is async
    pub is_async: bool,
    /// Generic parameters
    pub generic_params: Vec<String>,
}

/// Parameter documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDoc {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: String,
    /// Parameter description
    pub description: Option<String>,
    /// Whether parameter is optional
    pub is_optional: bool,
    /// Default value if any
    pub default_value: Option<String>,
}

/// Type documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeDoc {
    /// Type name
    pub name: String,
    /// Type description
    pub description: Option<String>,
    /// Type definition (struct, enum, interface, etc.)
    pub type_def: String,
    /// Fields or variants
    pub fields: Vec<FieldDoc>,
    /// Methods if any
    pub methods: Vec<FunctionDoc>,
    /// Source location
    pub location: SourceLocation,
    /// Source code
    pub source_code: Option<String>,
    /// Visibility
    pub visibility: String,
    /// Generic parameters
    pub generic_params: Vec<String>,
}

/// Field documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDoc {
    /// Field name
    pub name: String,
    /// Field type
    pub field_type: String,
    /// Field description
    pub description: Option<String>,
    /// Visibility
    pub visibility: String,
    /// Whether field is optional
    pub is_optional: bool,
}

/// Module documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDoc {
    /// Module name
    pub name: String,
    /// Module description
    pub description: Option<String>,
    /// Module path
    pub path: PathBuf,
    /// Exported items
    pub exports: Vec<String>,
    /// Sub-modules
    pub submodules: Vec<String>,
    /// Source location
    pub location: SourceLocation,
}

/// Code example documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleDoc {
    /// Example title
    pub title: Option<String>,
    /// Example code
    pub code: String,
    /// Example description
    pub description: Option<String>,
    /// Whether the example should be runnable
    pub is_runnable: bool,
    /// Expected output if any
    pub expected_output: Option<String>,
}

/// Configuration for the documentation generator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocGeneratorConfig {
    /// Include private items
    pub include_private: bool,
    /// Include source code
    pub include_source: bool,
    /// Generate cross-references
    pub generate_cross_refs: bool,
    /// Maximum type depth for recursive types
    pub max_type_depth: usize,
    /// Include examples
    pub include_examples: bool,
}

impl Default for DocGeneratorConfig {
    fn default() -> Self {
        Self {
            include_private: false,
            include_source: true,
            generate_cross_refs: true,
            max_type_depth: 10,
            include_examples: true,
        }
    }
}

/// Main documentation generator
pub struct DocumentationGenerator {
    config: super::DocumentationConfig,
    generator_config: DocGeneratorConfig,
    comment_parser: super::CommentParser,
}

impl DocumentationGenerator {
    /// Create a new documentation generator
    #[instrument(skip(config))]
    pub fn new(config: super::DocumentationConfig) -> Result<Self, Error> {
        info!("Initializing documentation generator");
        
        let generator_config = DocGeneratorConfig {
            include_private: config.options.include_private,
            include_source: config.options.include_source,
            generate_cross_refs: config.options.generate_cross_refs,
            max_type_depth: config.options.max_type_depth,
            include_examples: config.options.include_examples,
        };
        
        let comment_parser = super::CommentParser::new()?;
        
        Ok(Self {
            config,
            generator_config,
            comment_parser,
        })
    }

    /// Extract documentation from an AST
    #[instrument(skip(self, ast, source_code))]
    pub async fn extract_from_ast(
        &self,
        ast: &AstNode,
        file_path: &Path,
        source_code: &str,
    ) -> Result<super::ExtractedDocumentation, Error> {
        let start_time = std::time::Instant::now();
        
        debug!("Extracting documentation from AST for: {:?}", file_path);
        
        let mut extracted = super::ExtractedDocumentation {
            source_file: file_path.to_path_buf(),
            module_doc: None,
            functions: Vec::new(),
            types: Vec::new(),
            constants: Vec::new(),
            variables: Vec::new(),
            submodules: Vec::new(),
            source_code: if self.generator_config.include_source {
                Some(source_code.to_string())
            } else {
                None
            },
            metadata: super::ExtractionMetadata {
                extracted_at: chrono::Utc::now(),
                generator_version: env!("CARGO_PKG_VERSION").to_string(),
                item_count: 0,
                warnings: Vec::new(),
                processing_time_ms: 0,
            },
        };
        
        // Extract documentation based on AST node type
        self.extract_from_node(ast, &mut extracted, source_code).await?;
        
        let processing_time = start_time.elapsed();
        extracted.metadata.processing_time_ms = processing_time.as_millis() as u64;
        extracted.metadata.item_count = 
            extracted.functions.len() + 
            extracted.types.len() + 
            extracted.constants.len() + 
            extracted.variables.len();
        
        info!(
            "Extracted {} items from {} in {:?}",
            extracted.metadata.item_count,
            file_path.display(),
            processing_time
        );
        
        Ok(extracted)
    }

    /// Extract documentation from a specific AST node
    #[instrument(skip(self, node, extracted, source_code))]
    async fn extract_from_node(
        &self,
        node: &AstNode,
        extracted: &mut super::ExtractedDocumentation,
        source_code: &str,
    ) -> Result<(), Error> {
        match &node.node_type {
            AstNodeType::Program(program) => {
                // Extract module-level documentation
                if let Some(module_doc) = self.extract_module_doc(&extracted.source_file, source_code)? {
                    extracted.module_doc = Some(module_doc);
                }
                
                // Process all statements in the program
                for statement in &program.statements {
                    self.extract_from_node(statement, extracted, source_code).await?;
                }
            }
            
            AstNodeType::FunctionDeclaration(func_decl) => {
                if let Some(func_doc) = self.extract_function_doc(func_decl, source_code)? {
                    extracted.functions.push(func_doc);
                }
            }
            
            AstNodeType::StructDeclaration(struct_decl) => {
                if let Some(type_doc) = self.extract_struct_doc(struct_decl, source_code)? {
                    extracted.types.push(type_doc);
                }
            }
            
            AstNodeType::InterfaceDeclaration(interface_decl) => {
                if let Some(type_doc) = self.extract_interface_doc(interface_decl, source_code)? {
                    extracted.types.push(type_doc);
                }
            }
            
            AstNodeType::VariableDeclaration(var_decl) => {
                if let Some(var_doc) = self.extract_variable_doc(var_decl, source_code)? {
                    if var_decl.is_const {
                        extracted.constants.push(var_doc);
                    } else {
                        extracted.variables.push(var_doc);
                    }
                }
            }
            
            // Handle other node types as needed
            _ => {
                // For composite nodes, recursively process children
                if let Some(children) = self.get_child_nodes(node) {
                    for child in children {
                        self.extract_from_node(child, extracted, source_code).await?;
                    }
                }
            }
        }
        
        Ok(())
    }

    /// Extract module documentation
    #[instrument(skip(self, source_code))]
    fn extract_module_doc(&self, file_path: &Path, source_code: &str) -> Result<Option<ModuleDoc>, Error> {
        // Look for module-level documentation at the beginning of the file
        let lines: Vec<&str> = source_code.lines().collect();
        let mut description = None;
        
        // Find documentation comments at the start of the file
        for line in &lines {
            let trimmed = line.trim();
            if trimmed.starts_with("//!") {
                let comment = trimmed.trim_start_matches("//!").trim();
                if description.is_none() {
                    description = Some(comment.to_string());
                } else {
                    description = Some(format!("{}\n{}", description.unwrap(), comment));
                }
            } else if !trimmed.is_empty() && !trimmed.starts_with("//") {
                break; // Stop at first non-comment, non-empty line
            }
        }
        
        if description.is_some() {
            // Extract exports and submodules from the source code
            let exports = self.extract_exports(source_code)?;
            let submodules = self.extract_submodules(source_code)?;
            
            debug!(
                "Extracted {} exports and {} submodules from {:?}",
                exports.len(),
                submodules.len(),
                file_path
            );
            
            Ok(Some(ModuleDoc {
                name: file_path.file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
                description,
                path: file_path.to_path_buf(),
                exports,
                submodules,
                location: SourceLocation {
                    file: file_path.to_string_lossy().to_string(),
                    line: 1,
                    column: 1,
                },
            }))
        } else {
            Ok(None)
        }
    }

    /// Extract function documentation
    fn extract_function_doc(&self, func_decl: &FunctionDeclaration, source_code: &str) -> Result<Option<FunctionDoc>, Error> {
        // Check visibility - skip private functions if not including them
        if !self.generator_config.include_private && !func_decl.is_public {
            return Ok(None);
        }
        
        // Extract documentation comment above the function
        let description = self.extract_doc_comment_before(&func_decl.location, source_code)?;
        
        // Extract parameters
        let mut parameters = Vec::new();
        for param in &func_decl.parameters {
            parameters.push(ParameterDoc {
                name: param.name.clone(),
                param_type: param.param_type.as_ref()
                    .map(|t| self.format_type(t))
                    .unwrap_or_else(|| "Any".to_string()),
                description: None, // TODO: Extract from param documentation
                is_optional: param.default_value.is_some(),
                default_value: param.default_value.as_ref()
                    .map(|_| "TODO".to_string()), // TODO: Format default value
            });
        }
        
        // Extract return type
        let return_type = func_decl.return_type.as_ref().map(|rt| TypeDoc {
            name: self.format_type(rt),
            description: None,
            type_def: "return".to_string(),
            fields: Vec::new(),
            methods: Vec::new(),
            location: func_decl.location.clone(),
            source_code: None,
            visibility: "public".to_string(),
            generic_params: Vec::new(),
        });
        
        // Extract source code
        let source_code_snippet = if self.generator_config.include_source {
            self.extract_source_snippet(&func_decl.location, source_code)?
        } else {
            None
        };
        
        // Extract examples from documentation
        let examples = if self.generator_config.include_examples {
            self.extract_examples_from_description(&description)?
        } else {
            Vec::new()
        };
        
        Ok(Some(FunctionDoc {
            name: func_decl.name.clone(),
            description,
            parameters,
            return_type,
            examples,
            location: func_decl.location.clone(),
            source_code: source_code_snippet,
            visibility: if func_decl.is_public { "public" } else { "private" }.to_string(),
            is_async: func_decl.is_async,
            generic_params: func_decl.generic_params.clone().unwrap_or_default(),
        }))
    }

    /// Extract struct documentation
    fn extract_struct_doc(&self, struct_decl: &StructDeclaration, source_code: &str) -> Result<Option<TypeDoc>, Error> {
        // Check visibility
        if !self.generator_config.include_private && !struct_decl.is_public {
            return Ok(None);
        }
        
        let description = self.extract_doc_comment_before(&struct_decl.location, source_code)?;
        
        // Extract fields
        let mut fields = Vec::new();
        for field in &struct_decl.fields {
            fields.push(FieldDoc {
                name: field.name.clone(),
                field_type: field.field_type.as_ref()
                    .map(|t| self.format_type(t))
                    .unwrap_or_else(|| "Any".to_string()),
                description: None, // TODO: Extract field documentation
                visibility: "public".to_string(), // TODO: Handle field visibility
                is_optional: false, // TODO: Handle optional fields
            });
        }
        
        let source_code_snippet = if self.generator_config.include_source {
            self.extract_source_snippet(&struct_decl.location, source_code)?
        } else {
            None
        };
        
        Ok(Some(TypeDoc {
            name: struct_decl.name.clone(),
            description,
            type_def: "struct".to_string(),
            fields,
            methods: Vec::new(), // TODO: Extract methods
            location: struct_decl.location.clone(),
            source_code: source_code_snippet,
            visibility: if struct_decl.is_public { "public" } else { "private" }.to_string(),
            generic_params: struct_decl.generic_params.clone().unwrap_or_default(),
        }))
    }

    /// Extract interface documentation
    fn extract_interface_doc(&self, interface_decl: &InterfaceDeclaration, source_code: &str) -> Result<Option<TypeDoc>, Error> {
        // Check visibility
        if !self.generator_config.include_private && !interface_decl.is_public {
            return Ok(None);
        }
        
        let description = self.extract_doc_comment_before(&interface_decl.location, source_code)?;
        
        // Extract methods from interface
        let mut methods = Vec::new();
        for method in &interface_decl.methods {
            if let Some(method_doc) = self.extract_function_doc(method, source_code)? {
                methods.push(method_doc);
            }
        }
        
        let source_code_snippet = if self.generator_config.include_source {
            self.extract_source_snippet(&interface_decl.location, source_code)?
        } else {
            None
        };
        
        Ok(Some(TypeDoc {
            name: interface_decl.name.clone(),
            description,
            type_def: "interface".to_string(),
            fields: Vec::new(),
            methods,
            location: interface_decl.location.clone(),
            source_code: source_code_snippet,
            visibility: if interface_decl.is_public { "public" } else { "private" }.to_string(),
            generic_params: interface_decl.generic_params.clone().unwrap_or_default(),
        }))
    }

    /// Extract variable documentation
    fn extract_variable_doc(&self, var_decl: &VariableDeclaration, source_code: &str) -> Result<Option<DocumentationItem>, Error> {
        // Check visibility
        if !self.generator_config.include_private && !var_decl.is_public {
            return Ok(None);
        }
        
        let description = self.extract_doc_comment_before(&var_decl.location, source_code)?;
        
        let source_code_snippet = if self.generator_config.include_source {
            self.extract_source_snippet(&var_decl.location, source_code)?
        } else {
            None
        };
        
        let mut metadata = HashMap::new();
        if let Some(ref var_type) = var_decl.var_type {
            metadata.insert("type".to_string(), self.format_type(var_type));
        }
        if var_decl.is_mutable {
            metadata.insert("mutable".to_string(), "true".to_string());
        }
        
        Ok(Some(DocumentationItem {
            name: var_decl.name.clone(),
            kind: if var_decl.is_const { ItemKind::Constant } else { ItemKind::Variable },
            description,
            location: var_decl.location.clone(),
            source_code: source_code_snippet,
            visibility: if var_decl.is_public { "public" } else { "private" }.to_string(),
            metadata,
        }))
    }

    /// Extract documentation comment before a location
    fn extract_doc_comment_before(&self, location: &SourceLocation, source_code: &str) -> Result<Option<String>, Error> {
        let lines: Vec<&str> = source_code.lines().collect();
        
        if location.line <= 1 || location.line > lines.len() {
            return Ok(None);
        }
        
        let mut comment_lines = Vec::new();
        let mut line_idx = location.line - 2; // Start from line before the declaration
        
        // Look backwards for documentation comments
        loop {
            if line_idx >= lines.len() {
                break;
            }
            
            let line = lines[line_idx].trim();
            
            if line.starts_with("///") {
                // Documentation comment
                let comment = line.trim_start_matches("///").trim();
                comment_lines.insert(0, comment.to_string());
            } else if line.starts_with("//") || line.is_empty() {
                // Regular comment or empty line - skip
            } else {
                // Non-comment line - stop looking
                break;
            }
            
            if line_idx == 0 {
                break;
            }
            line_idx -= 1;
        }
        
        if comment_lines.is_empty() {
            Ok(None)
        } else {
            Ok(Some(comment_lines.join("\n")))
        }
    }

    /// Extract source code snippet for a location
    fn extract_source_snippet(&self, location: &SourceLocation, source_code: &str) -> Result<Option<String>, Error> {
        let lines: Vec<&str> = source_code.lines().collect();
        
        if location.line > lines.len() {
            return Ok(None);
        }
        
        // Extract the line where the declaration starts
        // TODO: This is a simplified implementation - ideally we'd extract
        // the full declaration including multi-line constructs
        let start_line = location.line.saturating_sub(1);
        let snippet = lines.get(start_line)
            .map(|line| line.to_string());
        
        Ok(snippet)
    }

    /// Extract examples from documentation description
    fn extract_examples_from_description(&self, description: &Option<String>) -> Result<Vec<ExampleDoc>, Error> {
        let mut examples = Vec::new();
        
        if let Some(desc) = description {
            // Look for code blocks in documentation
            let mut in_code_block = false;
            let mut current_example = String::new();
            let mut example_title = None;
            
            for line in desc.lines() {
                let trimmed = line.trim();
                
                if trimmed.starts_with("```cursed") || trimmed.starts_with("```csd") {
                    in_code_block = true;
                    current_example.clear();
                    // Extract title from the line if present
                    if let Some(title_part) = trimmed.strip_prefix("```cursed").or_else(|| trimmed.strip_prefix("```csd")) {
                        let title = title_part.trim();
                        if !title.is_empty() {
                            example_title = Some(title.to_string());
                        }
                    }
                } else if trimmed == "```" && in_code_block {
                    in_code_block = false;
                    if !current_example.trim().is_empty() {
                        examples.push(ExampleDoc {
                            title: example_title.take(),
                            code: current_example.trim().to_string(),
                            description: None,
                            is_runnable: true,
                            expected_output: None,
                        });
                    }
                    current_example.clear();
                } else if in_code_block {
                    current_example.push_str(line);
                    current_example.push('\n');
                }
            }
        }
        
        Ok(examples)
    }

    /// Format a type for documentation
    fn format_type(&self, type_expr: &Expression) -> String {
        // This is a simplified implementation
        // TODO: Implement proper type formatting based on expression type
        match &type_expr.expr_type {
            ExpressionType::Identifier(id) => id.name.clone(),
            _ => "Any".to_string(),
        }
    }

    /// Get child nodes from an AST node
    fn get_child_nodes(&self, node: &AstNode) -> Option<Vec<&AstNode>> {
        // This is a simplified implementation
        // TODO: Implement proper child node extraction for all AST node types
        match &node.node_type {
            AstNodeType::Program(program) => {
                Some(program.statements.iter().collect())
            }
            _ => None,
        }
    }

    /// Generate output in the specified format
    #[instrument(skip(self, extracted_docs, cross_references, search_index))]
    pub async fn generate_output(
        &self,
        extracted_docs: &[super::ExtractedDocumentation],
        cross_references: &HashMap<String, Vec<super::CrossReference>>,
        search_index: &[super::SearchIndexEntry],
        format: OutputFormat,
    ) -> Result<Vec<PathBuf>, Error> {
        info!("Generating {} documentation", format);
        
        match format {
            OutputFormat::Html => self.generate_html_output(extracted_docs, cross_references, search_index).await,
            OutputFormat::Markdown => self.generate_markdown_output(extracted_docs).await,
            OutputFormat::Json => self.generate_json_output(extracted_docs, cross_references, search_index).await,
            OutputFormat::Xml => self.generate_xml_output(extracted_docs).await,
            OutputFormat::LaTeX => self.generate_latex_output(extracted_docs).await,
        }
    }

    /// Generate HTML documentation
    async fn generate_html_output(
        &self,
        extracted_docs: &[super::ExtractedDocumentation],
        cross_references: &HashMap<String, Vec<super::CrossReference>>,
        search_index: &[super::SearchIndexEntry],
    ) -> Result<Vec<PathBuf>, Error> {
        let output_dir = &self.config.output_dir;
        std::fs::create_dir_all(output_dir)
            .map_err(|e| Error::FileWriteError(output_dir.clone(), e.to_string()))?;
        
        let mut output_files = Vec::new();
        
        // Generate index.html
        let index_content = self.generate_html_index(extracted_docs, search_index)?;
        let index_path = output_dir.join("index.html");
        std::fs::write(&index_path, index_content)
            .map_err(|e| Error::FileWriteError(index_path.clone(), e.to_string()))?;
        output_files.push(index_path);
        
        // Generate page for each module
        for doc in extracted_docs {
            let module_name = doc.source_file.file_stem()
                .unwrap_or_default()
                .to_string_lossy();
            let page_content = self.generate_html_module_page(doc, cross_references)?;
            let page_path = output_dir.join(format!("{}.html", module_name));
            std::fs::write(&page_path, page_content)
                .map_err(|e| Error::FileWriteError(page_path.clone(), e.to_string()))?;
            output_files.push(page_path);
        }
        
        // Generate search index file
        if self.config.options.generate_search_index {
            let search_content = serde_json::to_string_pretty(search_index)
                .map_err(|e| Error::SerializationError(e.to_string()))?;
            let search_path = output_dir.join("search-index.json");
            std::fs::write(&search_path, search_content)
                .map_err(|e| Error::FileWriteError(search_path.clone(), e.to_string()))?;
            output_files.push(search_path);
        }
        
        // Copy static files (CSS, JS, etc.)
        self.copy_static_files(output_dir)?;
        
        Ok(output_files)
    }

    /// Generate HTML index page
    fn generate_html_index(
        &self,
        extracted_docs: &[super::ExtractedDocumentation],
        search_index: &[super::SearchIndexEntry],
    ) -> Result<String, Error> {
        let project = &self.config.project;
        
        let mut content = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - Documentation</title>
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <header>
        <h1>{}</h1>
        <p class="version">Version: {}</p>
    </header>
    <main>
        <section class="overview">
            <h2>Overview</h2>
            <p>{}</p>
        </section>
        <section class="modules">
            <h2>Modules</h2>
            <ul>
"#,
            project.name,
            project.name,
            project.version,
            project.description.as_deref().unwrap_or("CURSED project documentation")
        );
        
        for doc in extracted_docs {
            let module_name = doc.source_file.file_stem()
                .unwrap_or_default()
                .to_string_lossy();
            let description = doc.module_doc.as_ref()
                .and_then(|m| m.description.as_ref())
                .unwrap_or(&"No description available".to_string());
            
            content.push_str(&format!(
                r#"                <li>
                    <a href="{}.html">{}</a>
                    <p>{}</p>
                </li>
"#,
                module_name, module_name, description
            ));
        }
        
        content.push_str(r#"            </ul>
        </section>
    </main>
    <footer>
        <p>Generated by CURSED Documentation System</p>
    </footer>
</body>
</html>"#);
        
        Ok(content)
    }

    /// Generate HTML module page
    fn generate_html_module_page(
        &self,
        doc: &super::ExtractedDocumentation,
        cross_references: &HashMap<String, Vec<super::CrossReference>>,
    ) -> Result<String, Error> {
        let module_name = doc.source_file.file_stem()
            .unwrap_or_default()
            .to_string_lossy();
        
        let mut content = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - {}</title>
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <header>
        <nav><a href="index.html">← Back to Index</a></nav>
        <h1>Module: {}</h1>
    </header>
    <main>
"#,
            module_name, self.config.project.name, module_name
        );
        
        // Module description
        if let Some(ref module_doc) = doc.module_doc {
            if let Some(ref description) = module_doc.description {
                content.push_str(&format!(
                    r#"        <section class="module-description">
            <h2>Description</h2>
            <p>{}</p>
        </section>
"#,
                    html_escape(description)
                ));
            }
        }
        
        // Functions
        if !doc.functions.is_empty() {
            content.push_str(r#"        <section class="functions">
            <h2>Functions</h2>
"#);
            
            for func in &doc.functions {
                content.push_str(&self.format_function_html(func)?);
            }
            
            content.push_str("        </section>\n");
        }
        
        // Types
        if !doc.types.is_empty() {
            content.push_str(r#"        <section class="types">
            <h2>Types</h2>
"#);
            
            for type_doc in &doc.types {
                content.push_str(&self.format_type_html(type_doc)?);
            }
            
            content.push_str("        </section>\n");
        }
        
        content.push_str(r#"    </main>
    <footer>
        <p>Generated by CURSED Documentation System</p>
    </footer>
</body>
</html>"#);
        
        Ok(content)
    }

    /// Format function documentation as HTML
    fn format_function_html(&self, func: &FunctionDoc) -> Result<String, Error> {
        let mut html = format!(
            r#"            <div class="function" id="{}">
                <h3>{}</h3>
"#,
            func.name, func.name
        );
        
        if let Some(ref description) = func.description {
            html.push_str(&format!(
                "                <p class=\"description\">{}</p>\n",
                html_escape(description)
            ));
        }
        
        // Parameters
        if !func.parameters.is_empty() {
            html.push_str("                <h4>Parameters</h4>\n");
            html.push_str("                <ul class=\"parameters\">\n");
            
            for param in &func.parameters {
                html.push_str(&format!(
                    "                    <li><code>{}</code>: {} {}</li>\n",
                    html_escape(&param.name),
                    html_escape(&param.param_type),
                    param.description.as_ref()
                        .map(|d| format!("- {}", html_escape(d)))
                        .unwrap_or_default()
                ));
            }
            
            html.push_str("                </ul>\n");
        }
        
        // Return type
        if let Some(ref return_type) = func.return_type {
            html.push_str(&format!(
                "                <h4>Returns</h4>\n                <p><code>{}</code></p>\n",
                html_escape(&return_type.name)
            ));
        }
        
        // Examples
        if !func.examples.is_empty() {
            html.push_str("                <h4>Examples</h4>\n");
            for example in &func.examples {
                html.push_str(&format!(
                    "                <pre><code class=\"cursed\">{}</code></pre>\n",
                    html_escape(&example.code)
                ));
            }
        }
        
        // Source code
        if let Some(ref source) = func.source_code {
            html.push_str(&format!(
                "                <details class=\"source\">\n                    <summary>Source Code</summary>\n                    <pre><code class=\"cursed\">{}</code></pre>\n                </details>\n",
                html_escape(source)
            ));
        }
        
        html.push_str("            </div>\n");
        Ok(html)
    }

    /// Format type documentation as HTML
    fn format_type_html(&self, type_doc: &TypeDoc) -> Result<String, Error> {
        let mut html = format!(
            r#"            <div class="type" id="{}">
                <h3>{} ({})</h3>
"#,
            type_doc.name, type_doc.name, type_doc.type_def
        );
        
        if let Some(ref description) = type_doc.description {
            html.push_str(&format!(
                "                <p class=\"description\">{}</p>\n",
                html_escape(description)
            ));
        }
        
        // Fields
        if !type_doc.fields.is_empty() {
            html.push_str("                <h4>Fields</h4>\n");
            html.push_str("                <ul class=\"fields\">\n");
            
            for field in &type_doc.fields {
                html.push_str(&format!(
                    "                    <li><code>{}</code>: {}</li>\n",
                    html_escape(&field.name),
                    html_escape(&field.field_type)
                ));
            }
            
            html.push_str("                </ul>\n");
        }
        
        // Methods
        if !type_doc.methods.is_empty() {
            html.push_str("                <h4>Methods</h4>\n");
            for method in &type_doc.methods {
                html.push_str(&self.format_function_html(method)?);
            }
        }
        
        html.push_str("            </div>\n");
        Ok(html)
    }

    /// Generate Markdown documentation
    async fn generate_markdown_output(
        &self,
        extracted_docs: &[super::ExtractedDocumentation],
    ) -> Result<Vec<PathBuf>, Error> {
        let output_dir = &self.config.output_dir;
        std::fs::create_dir_all(output_dir)
            .map_err(|e| Error::FileWriteError(output_dir.clone(), e.to_string()))?;
        
        let mut output_files = Vec::new();
        
        // Generate README.md
        let readme_content = self.generate_markdown_index(extracted_docs)?;
        let readme_path = output_dir.join("README.md");
        std::fs::write(&readme_path, readme_content)
            .map_err(|e| Error::FileWriteError(readme_path.clone(), e.to_string()))?;
        output_files.push(readme_path);
        
        // Generate page for each module
        for doc in extracted_docs {
            let module_name = doc.source_file.file_stem()
                .unwrap_or_default()
                .to_string_lossy();
            let page_content = self.generate_markdown_module_page(doc)?;
            let page_path = output_dir.join(format!("{}.md", module_name));
            std::fs::write(&page_path, page_content)
                .map_err(|e| Error::FileWriteError(page_path.clone(), e.to_string()))?;
            output_files.push(page_path);
        }
        
        Ok(output_files)
    }

    /// Generate Markdown index
    fn generate_markdown_index(&self, extracted_docs: &[super::ExtractedDocumentation]) -> Result<String, Error> {
        let project = &self.config.project;
        
        let mut content = format!(
            "# {}\n\nVersion: {}\n\n{}\n\n## Modules\n\n",
            project.name,
            project.version,
            project.description.as_deref().unwrap_or("CURSED project documentation")
        );
        
        for doc in extracted_docs {
            let module_name = doc.source_file.file_stem()
                .unwrap_or_default()
                .to_string_lossy();
            let description = doc.module_doc.as_ref()
                .and_then(|m| m.description.as_ref())
                .unwrap_or(&"No description available".to_string());
            
            content.push_str(&format!(
                "- [{}]({}.md) - {}\n",
                module_name, module_name, description
            ));
        }
        
        content.push_str("\n---\n\n*Generated by CURSED Documentation System*\n");
        
        Ok(content)
    }

    /// Generate Markdown module page
    fn generate_markdown_module_page(&self, doc: &super::ExtractedDocumentation) -> Result<String, Error> {
        let module_name = doc.source_file.file_stem()
            .unwrap_or_default()
            .to_string_lossy();
        
        let mut content = format!("# Module: {}\n\n", module_name);
        
        // Module description
        if let Some(ref module_doc) = doc.module_doc {
            if let Some(ref description) = module_doc.description {
                content.push_str(&format!("{}\n\n", description));
            }
        }
        
        // Functions
        if !doc.functions.is_empty() {
            content.push_str("## Functions\n\n");
            
            for func in &doc.functions {
                content.push_str(&self.format_function_markdown(func)?);
                content.push_str("\n");
            }
        }
        
        // Types
        if !doc.types.is_empty() {
            content.push_str("## Types\n\n");
            
            for type_doc in &doc.types {
                content.push_str(&self.format_type_markdown(type_doc)?);
                content.push_str("\n");
            }
        }
        
        content.push_str("---\n\n*Generated by CURSED Documentation System*\n");
        
        Ok(content)
    }

    /// Format function documentation as Markdown
    fn format_function_markdown(&self, func: &FunctionDoc) -> Result<String, Error> {
        let mut markdown = format!("### {}\n\n", func.name);
        
        if let Some(ref description) = func.description {
            markdown.push_str(&format!("{}\n\n", description));
        }
        
        // Parameters
        if !func.parameters.is_empty() {
            markdown.push_str("**Parameters:**\n\n");
            for param in &func.parameters {
                markdown.push_str(&format!(
                    "- `{}`: {}{}\n",
                    param.name,
                    param.param_type,
                    param.description.as_ref()
                        .map(|d| format!(" - {}", d))
                        .unwrap_or_default()
                ));
            }
            markdown.push_str("\n");
        }
        
        // Return type
        if let Some(ref return_type) = func.return_type {
            markdown.push_str(&format!("**Returns:** `{}`\n\n", return_type.name));
        }
        
        // Examples
        if !func.examples.is_empty() {
            markdown.push_str("**Examples:**\n\n");
            for example in &func.examples {
                markdown.push_str(&format!("```cursed\n{}\n```\n\n", example.code));
            }
        }
        
        Ok(markdown)
    }

    /// Format type documentation as Markdown
    fn format_type_markdown(&self, type_doc: &TypeDoc) -> Result<String, Error> {
        let mut markdown = format!("### {} ({})\n\n", type_doc.name, type_doc.type_def);
        
        if let Some(ref description) = type_doc.description {
            markdown.push_str(&format!("{}\n\n", description));
        }
        
        // Fields
        if !type_doc.fields.is_empty() {
            markdown.push_str("**Fields:**\n\n");
            for field in &type_doc.fields {
                markdown.push_str(&format!("- `{}`: {}\n", field.name, field.field_type));
            }
            markdown.push_str("\n");
        }
        
        // Methods
        if !type_doc.methods.is_empty() {
            markdown.push_str("**Methods:**\n\n");
            for method in &type_doc.methods {
                markdown.push_str(&self.format_function_markdown(method)?);
            }
        }
        
        Ok(markdown)
    }

    /// Generate JSON documentation
    async fn generate_json_output(
        &self,
        extracted_docs: &[super::ExtractedDocumentation],
        cross_references: &HashMap<String, Vec<super::CrossReference>>,
        search_index: &[super::SearchIndexEntry],
    ) -> Result<Vec<PathBuf>, Error> {
        let output_dir = &self.config.output_dir;
        std::fs::create_dir_all(output_dir)
            .map_err(|e| Error::FileWriteError(output_dir.clone(), e.to_string()))?;
        
        let mut output_files = Vec::new();
        
        // Create comprehensive JSON documentation
        let json_doc = serde_json::json!({
            "project": self.config.project,
            "modules": extracted_docs,
            "cross_references": cross_references,
            "search_index": search_index,
            "generated_at": chrono::Utc::now(),
            "generator_version": env!("CARGO_PKG_VERSION")
        });
        
        let json_content = serde_json::to_string_pretty(&json_doc)
            .map_err(|e| Error::SerializationError(e.to_string()))?;
        
        let json_path = output_dir.join("documentation.json");
        std::fs::write(&json_path, json_content)
            .map_err(|e| Error::FileWriteError(json_path.clone(), e.to_string()))?;
        output_files.push(json_path);
        
        Ok(output_files)
    }

    /// Generate XML documentation
    async fn generate_xml_output(&self, extracted_docs: &[super::ExtractedDocumentation]) -> Result<Vec<PathBuf>, Error> {
        use std::io::Write;
        
        info!("Generating XML documentation for {} files", extracted_docs.len());
        
        let mut output_files = Vec::new();
        
        // Generate main documentation file
        let main_doc_xml = self.build_main_xml_doc(extracted_docs)?;
        let main_path = self.config.output_dir.join("documentation.xml");
        std::fs::write(&main_path, main_doc_xml)
            .map_err(|e| Error::FileWriteError(main_path.clone(), e.to_string()))?;
        output_files.push(main_path);
        
        // Generate individual module files
        for doc in extracted_docs {
            let module_xml = self.build_module_xml_doc(doc)?;
            let module_filename = format!("{}.xml", 
                doc.source_file.file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .replace(' ', "_"));
            let module_path = self.config.output_dir.join(module_filename);
            std::fs::write(&module_path, module_xml)
                .map_err(|e| Error::FileWriteError(module_path.clone(), e.to_string()))?;
            output_files.push(module_path);
        }
        
        // Generate API index
        let api_index_xml = self.build_api_index_xml(extracted_docs)?;
        let api_path = self.config.output_dir.join("api_index.xml");
        std::fs::write(&api_path, api_index_xml)
            .map_err(|e| Error::FileWriteError(api_path.clone(), e.to_string()))?;
        output_files.push(api_path);
        
        // Generate DTD file for validation
        let dtd_content = self.build_dtd_content()?;
        let dtd_path = self.config.output_dir.join("cursed_docs.dtd");
        std::fs::write(&dtd_path, dtd_content)
            .map_err(|e| Error::FileWriteError(dtd_path.clone(), e.to_string()))?;
        output_files.push(dtd_path);
        
        info!("Generated {} XML documentation files", output_files.len());
        Ok(output_files)
    }

    /// Generate LaTeX documentation
    async fn generate_latex_output(&self, extracted_docs: &[super::ExtractedDocumentation]) -> Result<Vec<PathBuf>, Error> {
        // TODO: Implement LaTeX generation
        warn!("LaTeX output format not yet implemented");
        Ok(Vec::new())
    }

    /// Build main XML documentation structure
    fn build_main_xml_doc(&self, docs: &[super::ExtractedDocumentation]) -> Result<String, Error> {
        let mut xml = String::new();
        
        // XML declaration with DTD reference
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<!DOCTYPE documentation SYSTEM \"cursed_docs.dtd\">\n\n");
        
        // Root documentation element
        xml.push_str("<documentation>\n");
        xml.push_str("  <metadata>\n");
        xml.push_str(&format!("    <project_name>{}</project_name>\n", 
            self.escape_xml(&self.config.project.name)));
        xml.push_str(&format!("    <version>{}</version>\n", 
            self.escape_xml(&self.config.project.version)));
        
        if let Some(description) = &self.config.project.description {
            xml.push_str(&format!("    <description>{}</description>\n", 
                self.escape_xml(description)));
        }
        
        if !self.config.project.authors.is_empty() {
            xml.push_str("    <authors>\n");
            for author in &self.config.project.authors {
                xml.push_str(&format!("      <author>{}</author>\n", self.escape_xml(author)));
            }
            xml.push_str("    </authors>\n");
        }
        
        if let Some(homepage) = &self.config.project.homepage {
            xml.push_str(&format!("    <homepage>{}</homepage>\n", self.escape_xml(homepage)));
        }
        
        if let Some(repository) = &self.config.project.repository {
            xml.push_str(&format!("    <repository>{}</repository>\n", self.escape_xml(repository)));
        }
        
        xml.push_str(&format!("    <generated_at>{}</generated_at>\n", 
            chrono::Utc::now().to_rfc3339()));
        xml.push_str("    <generator>CURSED Enhanced Documentation System</generator>\n");
        xml.push_str("  </metadata>\n\n");

        // Project summary
        xml.push_str("  <summary>\n");
        xml.push_str(&format!("    <total_files>{}</total_files>\n", docs.len()));
        let total_functions: usize = docs.iter().map(|d| d.functions.len()).sum();
        let total_types: usize = docs.iter().map(|d| d.types.len()).sum();
        let total_constants: usize = docs.iter().map(|d| d.constants.len()).sum();
        xml.push_str(&format!("    <total_functions>{}</total_functions>\n", total_functions));
        xml.push_str(&format!("    <total_types>{}</total_types>\n", total_types));
        xml.push_str(&format!("    <total_constants>{}</total_constants>\n", total_constants));
        xml.push_str("  </summary>\n\n");

        // Files overview
        xml.push_str("  <files>\n");
        for doc in docs {
            xml.push_str("    <file>\n");
            xml.push_str(&format!("      <path>{}</path>\n", 
                self.escape_xml(&doc.source_file.display().to_string())));
            xml.push_str(&format!("      <function_count>{}</function_count>\n", doc.functions.len()));
            xml.push_str(&format!("      <type_count>{}</type_count>\n", doc.types.len()));
            xml.push_str(&format!("      <constant_count>{}</constant_count>\n", doc.constants.len()));
            xml.push_str(&format!("      <item_count>{}</item_count>\n", 
                doc.metadata.item_count));
            
            if let Some(module_doc) = &doc.module_doc {
                xml.push_str(&format!("      <module_name>{}</module_name>\n", 
                    self.escape_xml(&module_doc.name)));
                if let Some(description) = &module_doc.description {
                    xml.push_str(&format!("      <module_description>{}</module_description>\n", 
                        self.escape_xml(description)));
                }
            }
            
            xml.push_str(&format!("      <reference>file://{}.xml</reference>\n", 
                doc.source_file.file_stem().unwrap_or_default().to_string_lossy()));
            xml.push_str("    </file>\n");
        }
        xml.push_str("  </files>\n");
        xml.push_str("</documentation>\n");
        
        Ok(xml)
    }

    /// Build module-specific XML documentation
    fn build_module_xml_doc(&self, doc: &super::ExtractedDocumentation) -> Result<String, Error> {
        let mut xml = String::new();
        
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<!DOCTYPE module SYSTEM \"cursed_docs.dtd\">\n\n");
        
        xml.push_str("<module>\n");
        xml.push_str("  <header>\n");
        xml.push_str(&format!("    <source_file>{}</source_file>\n", 
            self.escape_xml(&doc.source_file.display().to_string())));
        xml.push_str(&format!("    <generated_at>{}</generated_at>\n", 
            chrono::Utc::now().to_rfc3339()));
        xml.push_str(&format!("    <extraction_time>{}</extraction_time>\n", 
            doc.metadata.extracted_at.to_rfc3339()));
        xml.push_str(&format!("    <item_count>{}</item_count>\n", 
            doc.metadata.item_count));
        xml.push_str("  </header>\n\n");

        // Module documentation if available
        if let Some(module_doc) = &doc.module_doc {
            xml.push_str("  <module_info>\n");
            xml.push_str(&format!("    <name>{}</name>\n", self.escape_xml(&module_doc.name)));
            if let Some(description) = &module_doc.description {
                xml.push_str(&format!("    <description>{}</description>\n", 
                    self.escape_xml(description)));
            }
            xml.push_str("  </module_info>\n\n");
        }

        // Functions section
        if !doc.functions.is_empty() {
            xml.push_str("  <functions>\n");
            for func in &doc.functions {
                xml.push_str("    <function>\n");
                xml.push_str(&format!("      <name>{}</name>\n", self.escape_xml(&func.name)));
                if let Some(description) = &func.description {
                    xml.push_str(&format!("      <description>{}</description>\n", 
                        self.escape_xml(description)));
                }
                if let Some(signature) = &func.signature {
                    xml.push_str(&format!("      <signature>{}</signature>\n", 
                        self.escape_xml(signature)));
                }
                if let Some(return_type) = &func.return_type {
                    xml.push_str(&format!("      <return_type>{}</return_type>\n", 
                        self.escape_xml(return_type)));
                }
                
                // Function location
                xml.push_str("      <location>\n");
                xml.push_str(&format!("        <line>{}</line>\n", func.location.line));
                xml.push_str(&format!("        <column>{}</column>\n", func.location.column));
                xml.push_str("      </location>\n");
                
                // Function parameters
                if !func.parameters.is_empty() {
                    xml.push_str("      <parameters>\n");
                    for param in &func.parameters {
                        xml.push_str("        <parameter>\n");
                        xml.push_str(&format!("          <name>{}</name>\n", 
                            self.escape_xml(&param.name)));
                        if let Some(param_type) = &param.param_type {
                            xml.push_str(&format!("          <type>{}</type>\n", 
                                self.escape_xml(param_type)));
                        }
                        if let Some(description) = &param.description {
                            xml.push_str(&format!("          <description>{}</description>\n", 
                                self.escape_xml(description)));
                        }
                        xml.push_str("        </parameter>\n");
                    }
                    xml.push_str("      </parameters>\n");
                }
                
                // Source code if available
                if self.config.options.include_source {
                    if let Some(source) = &func.source_code {
                        xml.push_str("      <source_code>\n");
                        xml.push_str(&format!("        <![CDATA[{}]]>\n", source));
                        xml.push_str("      </source_code>\n");
                    }
                }
                
                xml.push_str("    </function>\n");
            }
            xml.push_str("  </functions>\n\n");
        }

        // Types section
        if !doc.types.is_empty() {
            xml.push_str("  <types>\n");
            for type_doc in &doc.types {
                xml.push_str("    <type>\n");
                xml.push_str(&format!("      <name>{}</name>\n", self.escape_xml(&type_doc.name)));
                xml.push_str(&format!("      <kind>{}</kind>\n", 
                    self.escape_xml(&type_doc.kind.to_string())));
                if let Some(description) = &type_doc.description {
                    xml.push_str(&format!("      <description>{}</description>\n", 
                        self.escape_xml(description)));
                }
                
                // Type location
                xml.push_str("      <location>\n");
                xml.push_str(&format!("        <line>{}</line>\n", type_doc.location.line));
                xml.push_str(&format!("        <column>{}</column>\n", type_doc.location.column));
                xml.push_str("      </location>\n");
                
                xml.push_str("    </type>\n");
            }
            xml.push_str("  </types>\n\n");
        }

        // Constants section
        if !doc.constants.is_empty() {
            xml.push_str("  <constants>\n");
            for constant in &doc.constants {
                xml.push_str("    <constant>\n");
                xml.push_str(&format!("      <name>{}</name>\n", self.escape_xml(&constant.name)));
                xml.push_str(&format!("      <kind>{}</kind>\n", 
                    constant.kind.to_string()));
                xml.push_str(&format!("      <summary>{}</summary>\n", 
                    self.escape_xml(&constant.summary)));
                
                // Constant location
                xml.push_str("      <location>\n");
                xml.push_str(&format!("        <line>{}</line>\n", constant.location.line));
                xml.push_str(&format!("        <column>{}</column>\n", constant.location.column));
                xml.push_str("      </location>\n");
                
                xml.push_str("    </constant>\n");
            }
            xml.push_str("  </constants>\n\n");
        }

        xml.push_str("</module>\n");
        
        Ok(xml)
    }

    /// Build API index XML
    fn build_api_index_xml(&self, docs: &[super::ExtractedDocumentation]) -> Result<String, Error> {
        let mut xml = String::new();
        
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<!DOCTYPE api_index SYSTEM \"cursed_docs.dtd\">\n\n");
        
        xml.push_str("<api_index>\n");
        xml.push_str("  <metadata>\n");
        xml.push_str(&format!("    <generated_at>{}</generated_at>\n", 
            chrono::Utc::now().to_rfc3339()));
        xml.push_str("    <description>API index for all documented items</description>\n");
        xml.push_str("  </metadata>\n\n");

        // Function index
        xml.push_str("  <function_index>\n");
        for doc in docs {
            for func in &doc.functions {
                xml.push_str("    <function_ref>\n");
                xml.push_str(&format!("      <name>{}</name>\n", self.escape_xml(&func.name)));
                xml.push_str(&format!("      <file>{}</file>\n", 
                    self.escape_xml(&doc.source_file.display().to_string())));
                if let Some(description) = &func.description {
                    xml.push_str(&format!("      <description>{}</description>\n", 
                        self.escape_xml(description)));
                }
                xml.push_str(&format!("      <file_ref>{}.xml#{}</file_ref>\n", 
                    doc.source_file.file_stem().unwrap_or_default().to_string_lossy(),
                    func.name));
                xml.push_str("    </function_ref>\n");
            }
        }
        xml.push_str("  </function_index>\n\n");

        // Type index
        xml.push_str("  <type_index>\n");
        for doc in docs {
            for type_doc in &doc.types {
                xml.push_str("    <type_ref>\n");
                xml.push_str(&format!("      <name>{}</name>\n", self.escape_xml(&type_doc.name)));
                xml.push_str(&format!("      <kind>{}</kind>\n", 
                    self.escape_xml(&type_doc.kind.to_string())));
                xml.push_str(&format!("      <file>{}</file>\n", 
                    self.escape_xml(&doc.source_file.display().to_string())));
                xml.push_str(&format!("      <file_ref>{}.xml#{}</file_ref>\n", 
                    doc.source_file.file_stem().unwrap_or_default().to_string_lossy(),
                    type_doc.name));
                xml.push_str("    </type_ref>\n");
            }
        }
        xml.push_str("  </type_index>\n");
        
        xml.push_str("</api_index>\n");
        
        Ok(xml)
    }

    /// Build DTD content for XML validation
    fn build_dtd_content(&self) -> Result<String, Error> {
        let dtd = r#"<!-- CURSED Documentation DTD -->
<!ELEMENT documentation (metadata, summary, files)>
<!ELEMENT metadata (project_name, version, description?, authors?, homepage?, repository?, generated_at, generator)>
<!ELEMENT project_name (#PCDATA)>
<!ELEMENT version (#PCDATA)>
<!ELEMENT description (#PCDATA)>
<!ELEMENT authors (author+)>
<!ELEMENT author (#PCDATA)>
<!ELEMENT homepage (#PCDATA)>
<!ELEMENT repository (#PCDATA)>
<!ELEMENT generated_at (#PCDATA)>
<!ELEMENT generator (#PCDATA)>

<!ELEMENT summary (total_files, total_functions, total_types, total_constants)>
<!ELEMENT total_files (#PCDATA)>
<!ELEMENT total_functions (#PCDATA)>
<!ELEMENT total_types (#PCDATA)>
<!ELEMENT total_constants (#PCDATA)>

<!ELEMENT files (file*)>
<!ELEMENT file (path, function_count, type_count, constant_count, item_count, module_name?, module_description?, reference)>
<!ELEMENT path (#PCDATA)>
<!ELEMENT function_count (#PCDATA)>
<!ELEMENT type_count (#PCDATA)>
<!ELEMENT constant_count (#PCDATA)>
<!ELEMENT item_count (#PCDATA)>
<!ELEMENT module_name (#PCDATA)>
<!ELEMENT module_description (#PCDATA)>
<!ELEMENT reference (#PCDATA)>

<!ELEMENT module (header, module_info?, functions?, types?, constants?)>
<!ELEMENT header (source_file, generated_at, extraction_time, item_count)>
<!ELEMENT source_file (#PCDATA)>
<!ELEMENT extraction_time (#PCDATA)>

<!ELEMENT module_info (name, description?)>
<!ELEMENT name (#PCDATA)>

<!ELEMENT functions (function*)>
<!ELEMENT function (name, description?, signature?, return_type?, location, parameters?, source_code?)>
<!ELEMENT signature (#PCDATA)>
<!ELEMENT return_type (#PCDATA)>

<!ELEMENT location (line, column)>
<!ELEMENT line (#PCDATA)>
<!ELEMENT column (#PCDATA)>

<!ELEMENT parameters (parameter*)>
<!ELEMENT parameter (name, type?, description?)>
<!ELEMENT type (#PCDATA)>

<!ELEMENT source_code (#PCDATA)>

<!ELEMENT types (type*)>
<!ELEMENT type (name, kind, description?, location)>
<!ELEMENT kind (#PCDATA)>

<!ELEMENT constants (constant*)>
<!ELEMENT constant (name, kind, summary, location)>
<!ELEMENT summary (#PCDATA)>

<!ELEMENT api_index (metadata, function_index, type_index)>
<!ELEMENT function_index (function_ref*)>
<!ELEMENT function_ref (name, file, description?, file_ref)>
<!ELEMENT file (#PCDATA)>
<!ELEMENT file_ref (#PCDATA)>

<!ELEMENT type_index (type_ref*)>
<!ELEMENT type_ref (name, kind, file, file_ref)>
"#;
        
        Ok(dtd.to_string())
    }

    /// Escape XML special characters
    fn escape_xml(&self, text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;")
    }

    /// Copy static files (CSS, JS, etc.) for HTML output
    fn copy_static_files(&self, output_dir: &Path) -> Result<(), Error> {
        // Create basic CSS file
        let css_content = include_str!("../../../docs/styles.css");
        let css_path = output_dir.join("styles.css");
        std::fs::write(&css_path, css_content)
            .map_err(|e| Error::FileWriteError(css_path.clone(), e.to_string()))?;
        
        Ok(())
    }
}

/// HTML escape utility
fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}
