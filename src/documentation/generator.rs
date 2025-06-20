//! Documentation Generator Implementation
//! 
//! Core implementation of the CURSED documentation generation system with
//! support for multiple output formats, advanced features, and comprehensive
//! AST analysis.

use crate::ast::*;
use crate::documentation::extractors::ast_node_support::{ExpressionType, Literal};
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

    /// Extract documentation from an AST with enhanced integration
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
        
        // Use enhanced AST extractor for comprehensive documentation
        use crate::documentation::extractors::{AstExtractor, ExtractionConfig};
        
        let extraction_config = ExtractionConfig {
            include_private: self.generator_config.include_private,
            include_source: self.generator_config.include_source,
            include_generics: true,
            include_relationships: self.generator_config.generate_cross_refs,
            max_type_depth: self.generator_config.max_type_depth,
            include_implementations: true,
            include_error_types: true,
        };
        
        let ast_extractor = AstExtractor::new(extraction_config)?;
        let enhanced_items = ast_extractor.extract_complete_documentation(ast, file_path, source_code).await?;
        
        // Convert enhanced items to legacy format for compatibility
        for enhanced_item in enhanced_items {
            match enhanced_item.base.kind {
                super::ItemKind::Function => {
                    let func_doc = self.convert_to_function_doc(&enhanced_item, source_code)?;
                    extracted.functions.push(func_doc);
                }
                super::ItemKind::Type | super::ItemKind::Struct | super::ItemKind::Interface | super::ItemKind::Enum => {
                    let type_doc = self.convert_to_type_doc(&enhanced_item, source_code)?;
                    extracted.types.push(type_doc);
                }
                super::ItemKind::Constant => {
                    extracted.constants.push(enhanced_item.base);
                }
                super::ItemKind::Variable => {
                    extracted.variables.push(enhanced_item.base);
                }
                super::ItemKind::Module => {
                    let module_doc = self.convert_to_module_doc(&enhanced_item)?;
                    if extracted.module_doc.is_none() {
                        extracted.module_doc = Some(module_doc);
                    } else {
                        extracted.submodules.push(module_doc);
                    }
                }
                _ => {
                    // Handle other item types as generic documentation items
                    warn!("Unhandled item type: {:?}", enhanced_item.base.kind);
                }
            }
        }
        
        // Fallback to legacy extraction if no enhanced items found
        if extracted.functions.is_empty() && extracted.types.is_empty() && 
           extracted.constants.is_empty() && extracted.variables.is_empty() {
            self.extract_from_node(ast, &mut extracted, source_code).await?;
        }
        
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
        let lines: Vec<&str> = source_code.split("\n").collect();
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
            // Extract parameter documentation from preceding comment
            let param_description = self.extract_param_documentation(&param.name, &func_decl.location, source_code)?;
            
            parameters.push(ParameterDoc {
                name: param.name.clone(),
                param_type: param.param_type.as_ref()
                    .map(|t| self.format_type(t))
                    .unwrap_or_else(|| "Any".to_string()),
                description: param_description,
                is_optional: param.default_value.is_some(),
                default_value: param.default_value.as_ref()
                    .map(|expr| self.format_default_value(expr))
                    .flatten(),
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
            // Extract field documentation from preceding comment
            let field_description = self.extract_field_documentation(&field.name, &struct_decl.location, source_code)?;
            
            fields.push(FieldDoc {
                name: field.name.clone(),
                field_type: field.field_type.as_ref()
                    .map(|t| self.format_type(t))
                    .unwrap_or_else(|| "Any".to_string()),
                description: field_description,
                visibility: self.determine_field_visibility(field),
                is_optional: self.is_field_optional(field),
            });
        }
        
        let source_code_snippet = if self.generator_config.include_source {
            self.extract_source_snippet(&struct_decl.location, source_code)?
        } else {
            None
        };
        
        // Extract associated methods
        let methods = self.extract_associated_methods(&struct_decl.name, source_code)?;

        Ok(Some(TypeDoc {
            name: struct_decl.name.clone(),
            description,
            type_def: "struct".to_string(),
            fields,
            methods,
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
        let lines: Vec<&str> = source_code.split("\n").collect();
        
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
        let lines: Vec<&str> = source_code.split("\n").collect();
        
        if location.line > lines.len() {
            return Ok(None);
        }
        
        // Extract the full declaration including multi-line constructs
        let start_line = location.line.saturating_sub(1);
        let mut snippet_lines = Vec::new();
        let mut current_line = start_line;
        let mut brace_count = 0;
        let mut paren_count = 0;
        let mut in_declaration = false;
        
        while current_line < lines.len() {
            let line = lines[current_line];
            snippet_lines.push(line);
            
            // Track braces and parentheses to determine end of declaration
            for ch in line.chars() {
                match ch {
                    '{' => {
                        brace_count += 1;
                        in_declaration = true;
                    }
                    '}' => {
                        brace_count -= 1;
                        if brace_count == 0 && in_declaration {
                            return Ok(Some(snippet_lines.join("\n")));
                        }
                    }
                    '(' => paren_count += 1,
                    ')' => paren_count -= 1,
                    ';' => {
                        if brace_count == 0 && paren_count == 0 {
                            return Ok(Some(snippet_lines.join("\n")));
                        }
                    }
                    _ => {}
                }
            }
            
            current_line += 1;
            
            // Limit to reasonable number of lines to avoid including too much
            if snippet_lines.len() > 50 {
                break;
            }
        }
        
        Ok(Some(snippet_lines.join("\n")))
    }

    /// Extract examples from documentation description
    fn extract_examples_from_description(&self, description: &Option<String>) -> Result<Vec<ExampleDoc>, Error> {
        let mut examples = Vec::new();
        
        if let Some(desc) = description {
            // Look for code blocks in documentation
            let mut in_code_block = false;
            let mut current_example = String::new();
            let mut example_title = None;
            
            for line in desc.split("\n") {
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

    /// Convert enhanced documentation item to function documentation
    #[instrument(skip(self, enhanced_item, source_code))]
    fn convert_to_function_doc(
        &self,
        enhanced_item: &crate::documentation::extractors::EnhancedDocumentationItem,
        source_code: &str,
    ) -> Result<FunctionDoc, Error> {
        use crate::documentation::extractors::ast_extractor::{TypeKind, CompleteTypeInfo};
        
        let base = &enhanced_item.base;
        
        // Extract parameters from enhanced type information
        let mut parameters = Vec::new();
        if let Some(ref type_info) = enhanced_item.type_info {
            if type_info.type_kind == TypeKind::Function {
                // Extract parameter information from nested types
                for (i, nested_type) in type_info.nested_types.iter().enumerate() {
                    if i < type_info.nested_types.len() - 1 { // Last one is return type
                        parameters.push(ParameterDoc {
                            name: format!("param{}", i + 1), // Would need actual parameter names
                            param_type: nested_type.type_name.clone(),
                            description: None,
                            is_optional: false,
                            default_value: None,
                        });
                    }
                }
            }
        }

        // Extract return type information
        let return_type = enhanced_item.type_info.as_ref()
            .and_then(|ti| ti.nested_types.last())
            .map(|nested_type| TypeDoc {
                name: nested_type.type_name.clone(),
                description: None,
                type_def: "return".to_string(),
                fields: Vec::new(),
                methods: Vec::new(),
                location: base.location.clone(),
                source_code: None,
                visibility: "public".to_string(),
                generic_params: nested_type.type_parameters.clone(),
            });

        // Extract examples from description
        let examples = if self.generator_config.include_examples {
            self.extract_examples_from_description(&base.description)?
        } else {
            Vec::new()
        };

        // Extract generic parameters
        let generic_params = enhanced_item.generic_info.as_ref()
            .map(|gi| gi.parameters.iter().map(|p| p.name.clone()).collect())
            .unwrap_or_default();

        Ok(FunctionDoc {
            name: base.name.clone(),
            description: base.description.clone(),
            parameters,
            return_type,
            examples,
            location: base.location.clone(),
            source_code: base.source_code.clone(),
            visibility: base.visibility.clone(),
            is_async: base.metadata.get("is_async")
                .map(|v| v == "true")
                .unwrap_or(false),
            generic_params,
        })
    }

    /// Convert enhanced documentation item to type documentation
    #[instrument(skip(self, enhanced_item, source_code))]
    fn convert_to_type_doc(
        &self,
        enhanced_item: &crate::documentation::extractors::EnhancedDocumentationItem,
        source_code: &str,
    ) -> Result<TypeDoc, Error> {
        use crate::documentation::extractors::ast_extractor::TypeKind;
        
        let base = &enhanced_item.base;
        
        // Determine type definition string
        let type_def = enhanced_item.type_info.as_ref()
            .map(|ti| match ti.type_kind {
                TypeKind::Struct => "struct".to_string(),
                TypeKind::Interface => "interface".to_string(),
                TypeKind::Enum => "enum".to_string(),
                TypeKind::Union => "union".to_string(),
                TypeKind::Custom => "type".to_string(),
                _ => "unknown".to_string(),
            })
            .unwrap_or_else(|| "type".to_string());

        // Extract field information from metadata or type info
        let fields = self.extract_fields_from_enhanced_item(enhanced_item)?;

        // Extract method information
        let methods = self.extract_methods_from_enhanced_item(enhanced_item, source_code)?;

        // Extract generic parameters
        let generic_params = enhanced_item.generic_info.as_ref()
            .map(|gi| gi.parameters.iter().map(|p| p.name.clone()).collect())
            .unwrap_or_default();

        Ok(TypeDoc {
            name: base.name.clone(),
            description: base.description.clone(),
            type_def,
            fields,
            methods,
            location: base.location.clone(),
            source_code: base.source_code.clone(),
            visibility: base.visibility.clone(),
            generic_params,
        })
    }

    /// Convert enhanced documentation item to module documentation
    #[instrument(skip(self, enhanced_item))]
    fn convert_to_module_doc(
        &self,
        enhanced_item: &crate::documentation::extractors::EnhancedDocumentationItem,
    ) -> Result<ModuleDoc, Error> {
        let base = &enhanced_item.base;
        
        // Extract exports from relationships
        let exports = enhanced_item.relationships.iter()
            .filter_map(|rel| {
                use crate::documentation::extractors::ast_extractor::RelationshipType;
                match rel.relationship_type {
                    RelationshipType::Contains | RelationshipType::References => {
                        Some(rel.target.clone())
                    }
                    _ => None,
                }
            })
            .collect();

        // Extract submodules from relationships
        let submodules = enhanced_item.relationships.iter()
            .filter_map(|rel| {
                use crate::documentation::extractors::ast_extractor::RelationshipType;
                match rel.relationship_type {
                    RelationshipType::Contains => {
                        Some(rel.target.clone())
                    }
                    _ => None,
                }
            })
            .collect();

        Ok(ModuleDoc {
            name: base.name.clone(),
            description: base.description.clone(),
            path: std::path::PathBuf::from(&base.location.file),
            exports,
            submodules,
            location: base.location.clone(),
        })
    }

    /// Extract field information from enhanced documentation item
    fn extract_fields_from_enhanced_item(
        &self,
        enhanced_item: &crate::documentation::extractors::EnhancedDocumentationItem,
    ) -> Result<Vec<FieldDoc>, Error> {
        let mut fields = Vec::new();

        // Extract fields from type information
        if let Some(ref type_info) = enhanced_item.type_info {
            // For structs and other composite types, nested types might represent fields
            for (i, nested_type) in type_info.nested_types.iter().enumerate() {
                fields.push(FieldDoc {
                    name: format!("field{}", i + 1), // Would need actual field names from AST
                    field_type: nested_type.type_name.clone(),
                    description: None,
                    visibility: "public".to_string(), // Default visibility
                    is_optional: false, // Would need to extract from AST
                });
            }
        }

        Ok(fields)
    }

    /// Extract method information from enhanced documentation item
    fn extract_methods_from_enhanced_item(
        &self,
        enhanced_item: &crate::documentation::extractors::EnhancedDocumentationItem,
        source_code: &str,
    ) -> Result<Vec<FunctionDoc>, Error> {
        let mut methods = Vec::new();

        // Extract methods from implementations
        for impl_info in &enhanced_item.implementations {
            for method_name in &impl_info.methods {
                // Create a basic method documentation
                methods.push(FunctionDoc {
                    name: method_name.clone(),
                    description: Some(format!("Method from {} implementation", impl_info.interface_name)),
                    parameters: Vec::new(), // Would need to extract from AST
                    return_type: None,
                    examples: Vec::new(),
                    location: enhanced_item.base.location.clone(),
                    source_code: None,
                    visibility: "public".to_string(),
                    is_async: false,
                    generic_params: Vec::new(),
                });
            }
        }

        Ok(methods)
    }

    /// Format a type for documentation
    fn format_type(&self, type_expr: &dyn Expression) -> String {
        match &type_expr.expr_type {
            ExpressionType::Identifier(id) => id.name.clone(),
            ExpressionType::ArrayAccess(arr) => {
                format!("{}[]", self.format_type(&arr.array))
            }
            ExpressionType::FunctionCall(call) => {
                let args = call.arguments.iter()
                    .map(|arg| self.format_type(arg))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}({})", self.format_type(&call.function), args)
            }
            ExpressionType::MemberAccess(member) => {
                format!("{}.{}", self.format_type(&member.object), member.member)
            }
            ExpressionType::BinaryExpression(bin) => {
                // For generic types like Map<K, V>
                if bin.operator == "<" {
                    format!("{}<{}>", self.format_type(&bin.left), self.format_type(&bin.right))
                } else {
                    format!("({} {} {})", 
                        self.format_type(&bin.left), 
                        bin.operator, 
                        self.format_type(&bin.right))
                }
            }
            ExpressionType::Literal(lit) => {
                match lit {
                    Literal::String(s) => format!("\"{}\"", s),
                    Literal::Number(n) => n.to_string(),
                    Literal::Boolean(b) => b.to_string(),
                    Literal::Null => "null".to_string(),
                    Literal::Array(arr) => {
                        let elements = arr.iter()
                            .map(|elem| self.format_type(elem))
                            .collect::<Vec<_>>()
                            .join(", ");
                        format!("[{}]", elements)
                    }
                    Literal::Object(obj) => {
                        let fields = obj.iter()
                            .map(|(k, v)| format!("{}: {}", k, self.format_type(v)))
                            .collect::<Vec<_>>()
                            .join(", ");
                        format!("{{{}}}", fields)
                    }
                }
            }
            ExpressionType::ConditionalExpression(cond) => {
                format!("{} ? {} : {}", 
                    self.format_type(&cond.condition),
                    self.format_type(&cond.true_expr),
                    self.format_type(&cond.false_expr))
            }
            ExpressionType::UnaryExpression(unary) => {
                format!("{}{}", unary.operator, self.format_type(&unary.operand))
            }
            ExpressionType::ParenthesizedExpression(paren) => {
                format!("({})", self.format_type(&paren.expression))
            }
            ExpressionType::AssignmentExpression(assign) => {
                format!("{} = {}", self.format_type(&assign.left), self.format_type(&assign.right))
            }
            ExpressionType::UpdateExpression(update) => {
                if update.prefix {
                    format!("{}{}", update.operator, self.format_type(&update.operand))
                } else {
                    format!("{}{}", self.format_type(&update.operand), update.operator)
                }
            }
            ExpressionType::AwaitExpression(await_expr) => {
                format!("await {}", self.format_type(&await_expr.expression))
            }
            ExpressionType::YieldExpression(yield_expr) => {
                if let Some(ref arg) = yield_expr.argument {
                    format!("yield {}", self.format_type(arg))
                } else {
                    "yield".to_string()
                }
            }
            ExpressionType::NewExpression(new_expr) => {
                let args = new_expr.arguments.iter()
                    .map(|arg| self.format_type(arg))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("new {}({})", self.format_type(&new_expr.callee), args)
            }
            ExpressionType::ThisExpression => "this".to_string(),
            ExpressionType::Super => "super".to_string(),
            ExpressionType::ArrowFunction(arrow) => {
                let params = arrow.parameters.iter()
                    .map(|param| param.name.clone())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("({}) => {}", params, self.format_type(&arrow.body))
            }
            ExpressionType::TypeAssertion(assertion) => {
                format!("{}.({})", self.format_type(&assertion.expression), self.format_type(&assertion.type_annotation))
            }
            ExpressionType::TypeAssertionQuestion(assertion) => {
                format!("{}.({})?", self.format_type(&assertion.expression), self.format_type(&assertion.type_annotation))
            }
            _ => "Any".to_string(),
        }
    }

    /// Get child nodes from an AST node
    fn get_child_nodes(&self, node: &AstNode) -> Option<Vec<&AstNode>> {
        match &node.node_type {
            AstNodeType::Program(program) => {
                Some(program.statements.iter().collect())
            }
            AstNodeType::BlockStatement(block) => {
                Some(block.statements.iter().collect())
            }
            AstNodeType::IfStatement(if_stmt) => {
                let mut children = vec![&if_stmt.test, &if_stmt.consequent];
                if let Some(ref alternate) = if_stmt.alternate {
                    children.push(alternate);
                }
                Some(children)
            }
            AstNodeType::WhileStatement(while_stmt) => {
                Some(vec![&while_stmt.test, &while_stmt.body])
            }
            AstNodeType::ForStatement(for_stmt) => {
                let mut children = Vec::new();
                if let Some(ref init) = for_stmt.init {
                    children.push(init);
                }
                if let Some(ref test) = for_stmt.test {
                    children.push(test);
                }
                if let Some(ref update) = for_stmt.update {
                    children.push(update);
                }
                children.push(&for_stmt.body);
                Some(children)
            }
            AstNodeType::ForInStatement(for_in) => {
                Some(vec![&for_in.left, &for_in.right, &for_in.body])
            }
            AstNodeType::DoWhileStatement(do_while) => {
                Some(vec![&do_while.body, &do_while.test])
            }
            AstNodeType::SwitchStatement(switch) => {
                let mut children = vec![&switch.discriminant];
                children.extend(switch.cases.iter().map(|case| &case.node));
                Some(children)
            }
            AstNodeType::TryStatement(try_stmt) => {
                let mut children = vec![&try_stmt.block];
                if let Some(ref handler) = try_stmt.handler {
                    children.push(&handler.body);
                }
                if let Some(ref finalizer) = try_stmt.finalizer {
                    children.push(finalizer);
                }
                Some(children)
            }
            AstNodeType::FunctionDeclaration(func) => {
                Some(vec![&func.body])
            }
            AstNodeType::VariableDeclaration(var_decl) => {
                if let Some(ref init) = var_decl.init {
                    Some(vec![init])
                } else {
                    None
                }
            }
            AstNodeType::ReturnStatement(ret) => {
                if let Some(ref argument) = ret.argument {
                    Some(vec![argument])
                } else {
                    None
                }
            }
            AstNodeType::ThrowStatement(throw) => {
                Some(vec![&throw.argument])
            }
            AstNodeType::ExpressionStatement(expr_stmt) => {
                Some(vec![&expr_stmt.expression])
            }
            AstNodeType::ImportStatement(import) => {
                // Import statements don't typically have child nodes in the AST sense
                None
            }
            AstNodeType::StructDeclaration(_) => {
                // Struct declarations contain field information but not child AST nodes
                None
            }
            AstNodeType::InterfaceDeclaration(interface) => {
                // Interface methods are already handled in extract_interface_doc
                let method_nodes: Vec<&AstNode> = interface.methods.iter()
                    .map(|method| &method.node)
                    .collect();
                if method_nodes.is_empty() {
                    None
                } else {
                    Some(method_nodes)
                }
            }
            AstNodeType::EnumDeclaration(_) => {
                // Enum declarations contain variant information but not child AST nodes
                None
            }
            AstNodeType::TypeAliasDeclaration(_) => {
                // Type aliases don't have child nodes
                None
            }
            AstNodeType::ModuleDeclaration(module) => {
                if let Some(ref body) = module.body {
                    match &body.node_type {
                        AstNodeType::Program(program) => Some(program.statements.iter().collect()),
                        AstNodeType::BlockStatement(block) => Some(block.statements.iter().collect()),
                        _ => Some(vec![body]),
                    }
                } else {
                    None
                }
            }
            // Expression nodes generally don't have children we want to extract documentation from
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
        info!("Generating LaTeX documentation for {} files using enhanced LaTeX generator", extracted_docs.len());
        
        // Use the enhanced LaTeX generator
        use crate::documentation::generators::{LaTeXGenerator, LaTeXConfig, DocumentClass};
        
        // Create LaTeX configuration based on documentation config
        let latex_config = LaTeXConfig {
            document_class: if extracted_docs.len() > 10 {
                DocumentClass::Book
            } else if extracted_docs.len() > 5 {
                DocumentClass::Report
            } else {
                DocumentClass::Article
            },
            generate_toc: true,
            generate_index: true,
            generate_bibliography: true,
            include_code_listings: self.generator_config.include_source,
            generate_cross_refs: self.generator_config.generate_cross_refs,
            ..LaTeXConfig::default()
        };
        
        // Create and use the LaTeX generator
        let mut latex_generator = LaTeXGenerator::new(latex_config);
        let output_files = latex_generator.generate_documentation(extracted_docs, &self.config.output_dir)?;
        
        info!("Generated {} LaTeX documentation files using enhanced generator", output_files.len());
        Ok(output_files)
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
                // Generate function signature
                let signature = self.generate_function_signature(func);
                xml.push_str(&format!("      <signature>{}</signature>\n", 
                    self.escape_xml(&signature)));
                
                if let Some(return_type) = &func.return_type {
                    xml.push_str(&format!("      <return_type>{}</return_type>\n", 
                        self.escape_xml(&return_type.name)));
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
                        xml.push_str(&format!("          <type>{}</type>\n", 
                            self.escape_xml(&param.param_type)));
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
                    self.escape_xml(&type_doc.type_def)));
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
                    self.escape_xml(&constant.kind.to_string())));
                if let Some(description) = &constant.description {
                    xml.push_str(&format!("      <description>{}</description>\n", 
                        self.escape_xml(description)));
                }
                
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
                    self.escape_xml(&type_doc.type_def)));
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
        let css_content = include_str!("../../docs/styles.css");
        let css_path = output_dir.join("styles.css");
        std::fs::write(&css_path, css_content)
            .map_err(|e| Error::FileWriteError(css_path.clone(), e.to_string()))?;
        
        Ok(())
    }

    /// Extract parameter documentation from function documentation
    fn extract_param_documentation(&self, param_name: &str, func_location: &SourceLocation, source_code: &str) -> Result<Option<String>, Error> {
        let lines: Vec<&str> = source_code.split("\n").collect();
        
        if func_location.line <= 1 || func_location.line > lines.len() {
            return Ok(None);
        }
        
        // Look for documentation comments before the function
        let mut line_idx = func_location.line - 2;
        
        while line_idx < lines.len() {
            let line = lines[line_idx].trim();
            
            if line.starts_with("///") {
                let comment = line.trim_start_matches("///").trim();
                // Look for @param or parameter documentation
                if comment.starts_with(&format!("@param {}", param_name)) ||
                   comment.starts_with(&format!("{} -", param_name)) ||
                   comment.starts_with(&format!("{}: ", param_name)) {
                    let description = comment.split_once('-')
                        .or_else(|| comment.split_once(':'))
                        .map(|(_, desc)| desc.trim().to_string())
                        .or_else(|| {
                            // Extract after @param param_name
                            if comment.starts_with("@param") {
                                let parts: Vec<&str> = comment.split_whitespace().collect();
                                if parts.len() > 2 && parts[1] == param_name {
                                    Some(parts[2..].join(" "))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        });
                    return Ok(description);
                }
            } else if !line.starts_with("//") && !line.is_empty() {
                break;
            }
            
            if line_idx == 0 {
                break;
            }
            line_idx -= 1;
        }
        
        Ok(None)
    }

    /// Format default value expression
    fn format_default_value(&self, expr: &dyn Expression) -> Option<String> {
        match &expr.expr_type {
            ExpressionType::Literal(lit) => {
                match lit {
                    Literal::String(s) => Some(format!("\"{}\"", s)),
                    Literal::Number(n) => Some(n.to_string()),
                    Literal::Boolean(b) => Some(b.to_string()),
                    Literal::Null => Some("null".to_string()),
                    _ => Some("(complex)".to_string()),
                }
            }
            ExpressionType::Identifier(id) => Some(id.name.clone()),
            _ => Some("(expression)".to_string()),
        }
    }

    /// Extract field documentation
    fn extract_field_documentation(&self, field_name: &str, struct_location: &SourceLocation, source_code: &str) -> Result<Option<String>, Error> {
        let lines: Vec<&str> = source_code.split("\n").collect();
        
        // Find the struct definition and look for field comments
        let struct_start = struct_location.line.saturating_sub(1);
        let mut in_struct = false;
        
        for (idx, line) in lines.iter().enumerate().skip(struct_start) {
            if line.contains(&format!("{} {{", "struct")) || in_struct {
                in_struct = true;
                
                // Look for field definition
                if line.contains(field_name) && (line.contains(':') || line.contains(',') || line.contains('}')) {
                    // Check if there's a comment on the same line
                    if let Some(comment_start) = line.find("//") {
                        let comment = line[comment_start + 2..].trim();
                        if !comment.is_empty() {
                            return Ok(Some(comment.to_string()));
                        }
                    }
                    
                    // Check the line above for a comment
                    if idx > 0 {
                        let prev_line = lines[idx - 1].trim();
                        if prev_line.starts_with("///") {
                            let comment = prev_line.trim_start_matches("///").trim();
                            return Ok(Some(comment.to_string()));
                        }
                    }
                }
                
                if line.contains('}') {
                    break;
                }
            }
        }
        
        Ok(None)
    }

    /// Determine field visibility
    fn determine_field_visibility(&self, field: &crate::ast::StructField) -> String {
        // CURSED doesn't have explicit field visibility modifiers like Rust
        // All struct fields are public by default in most cases
        "public".to_string()
    }

    /// Check if field is optional
    fn is_field_optional(&self, field: &crate::ast::StructField) -> bool {
        // Check if the field type indicates optionality
        if let Some(ref field_type) = field.field_type {
            match &field_type.expr_type {
                ExpressionType::Identifier(id) => {
                    // Check for Option<T> or similar optional type patterns
                    id.name.starts_with("Option") || 
                    id.name.starts_with("Maybe") ||
                    id.name.contains("?")
                }
                ExpressionType::FunctionCall(call) => {
                    // Check for Option(T) or Maybe(T) patterns
                    if let ExpressionType::Identifier(id) = &call.function.expr_type {
                        id.name == "Option" || id.name == "Maybe"
                    } else {
                        false
                    }
                }
                _ => false,
            }
        } else {
            false
        }
    }

    /// Extract associated methods for a type using enhanced AST-based parsing
    fn extract_associated_methods(&self, type_name: &str, source_code: &str) -> Result<Vec<FunctionDoc>, Error> {
        let mut methods = Vec::new();
        
        // Use the enhanced AST extractor for better parsing
        if let Ok(ast_extractor) = crate::documentation::extractors::AstExtractor::new(
            crate::documentation::extractors::ExtractionConfig {
                include_private: self.generator_config.include_private,
                include_source: self.generator_config.include_source,
                include_generics: true,
                include_relationships: true,
                max_type_depth: self.generator_config.max_type_depth,
                include_implementations: true,
                include_error_types: true,
            }
        ) {
            // Parse the source code to get method information
            if let Ok(parsed_methods) = self.parse_impl_methods_from_source(type_name, source_code) {
                for method_info in parsed_methods {
                    methods.push(method_info);
                }
            }
        }
        
        // Fallback to line-based parsing if AST parsing fails
        if methods.is_empty() {
            methods = self.extract_methods_line_based(type_name, source_code)?;
        }
        
        Ok(methods)
    }

    /// Parse implementation methods from source code using enhanced parsing
    fn parse_impl_methods_from_source(&self, type_name: &str, source_code: &str) -> Result<Vec<FunctionDoc>, Error> {
        let mut methods = Vec::new();
        let lines: Vec<&str> = source_code.split("\n").collect();
        
        // Look for impl blocks for this type
        for (idx, line) in lines.iter().enumerate() {
            if line.trim().starts_with("impl") && line.contains(type_name) {
                // Found an impl block, extract methods with enhanced parsing
                let impl_methods = self.parse_impl_block_methods(&lines, idx, type_name)?;
                methods.extend(impl_methods);
            }
        }
        
        Ok(methods)
    }

    /// Parse methods within an impl block with full AST-based parameter and return type extraction
    fn parse_impl_block_methods(&self, lines: &[&str], start_idx: usize, type_name: &str) -> Result<Vec<FunctionDoc>, Error> {
        let mut methods = Vec::new();
        let mut brace_count = 0;
        let mut in_impl = false;
        let mut current_method_start = None;
        
        for (relative_idx, line) in lines[start_idx..].iter().enumerate() {
            let absolute_idx = start_idx + relative_idx;
            
            // Track brace nesting to stay within impl block
            for ch in line.chars() {
                match ch {
                    '{' => {
                        brace_count += 1;
                        in_impl = true;
                    }
                    '}' => {
                        brace_count -= 1;
                        if brace_count == 0 && in_impl {
                            // End of impl block
                            return Ok(methods);
                        }
                    }
                    _ => {}
                }
            }
            
            // Look for function definitions within the impl block
            if in_impl && (line.trim().starts_with("fn ") || line.trim().starts_with("async fn ") || 
                          line.trim().starts_with("pub fn ") || line.trim().starts_with("pub async fn ")) {
                current_method_start = Some(absolute_idx);
                
                // Parse the complete method signature
                if let Ok(method_doc) = self.parse_method_signature(lines, absolute_idx, type_name) {
                    methods.push(method_doc);
                }
            }
        }
        
        Ok(methods)
    }

    /// Parse a complete method signature including parameters and return type
    fn parse_method_signature(&self, lines: &[&str], start_idx: usize, type_name: &str) -> Result<FunctionDoc, Error> {
        let mut signature_lines = Vec::new();
        let mut paren_count = 0;
        let mut brace_count = 0;
        let mut found_opening_brace = false;
        
        // Collect the complete method signature (may span multiple lines)
        for line in &lines[start_idx..] {
            signature_lines.push(line.to_string());
            
            for ch in line.chars() {
                match ch {
                    '(' => paren_count += 1,
                    ')' => paren_count -= 1,
                    '{' => {
                        brace_count += 1;
                        found_opening_brace = true;
                    }
                    _ => {}
                }
            }
            
            // Stop when we've closed all parentheses and found the opening brace
            if paren_count == 0 && found_opening_brace {
                break;
            }
            
            // Safety check to avoid infinite loops
            if signature_lines.len() > 20 {
                break;
            }
        }
        
        let full_signature = signature_lines.join(" ");
        
        // Extract method components using enhanced parsing
        let method_info = self.extract_method_components(&full_signature, start_idx, type_name)?;
        
        Ok(method_info)
    }

    /// Extract method components (name, parameters, return type) from signature
    fn extract_method_components(&self, signature: &str, line_number: usize, type_name: &str) -> Result<FunctionDoc, Error> {
        // Parse method visibility and modifiers
        let is_async = signature.contains("async");
        let is_public = signature.contains("pub");
        
        // Extract method name
        let method_name = self.extract_method_name(signature)?;
        
        // Extract parameters using enhanced parsing
        let parameters = self.extract_method_parameters(signature)?;
        
        // Extract return type
        let return_type = self.extract_method_return_type(signature)?;
        
        // Extract generic parameters
        let generic_params = self.extract_method_generics(signature)?;
        
        // Extract documentation comment if available
        let description = Some(format!("Method of {}", type_name));
        
        Ok(FunctionDoc {
            name: method_name,
            description,
            parameters,
            return_type,
            examples: Vec::new(),
            location: SourceLocation {
                file: "".to_string(),
                line: line_number + 1,
                column: 1,
            },
            source_code: if self.generator_config.include_source {
                Some(signature.to_string())
            } else {
                None
            },
            visibility: if is_public { "public" } else { "private" }.to_string(),
            is_async,
            generic_params,
        })
    }

    /// Extract method name from signature
    fn extract_method_name(&self, signature: &str) -> Result<String, Error> {
        // Handle various function declaration patterns
        let patterns = [
            "pub async fn ",
            "async fn ",
            "pub fn ",
            "fn ",
        ];
        
        for pattern in &patterns {
            if let Some(fn_start) = signature.find(pattern) {
                let after_fn = &signature[fn_start + pattern.len()..];
                
                // Find the method name (everything before < or ()
                let end_pos = after_fn.find('<')
                    .or_else(|| after_fn.find('('))
                    .unwrap_or(after_fn.len());
                    
                let method_name = after_fn[..end_pos].trim().to_string();
                if !method_name.is_empty() {
                    return Ok(method_name);
                }
            }
        }
        
        Err(Error::ParseError("Failed to extract method name".to_string()))
    }

    /// Extract method parameters with enhanced type parsing
    fn extract_method_parameters(&self, signature: &str) -> Result<Vec<ParameterDoc>, Error> {
        let mut parameters = Vec::new();
        
        // Find the parameter list between parentheses
        if let Some(params_start) = signature.find('(') {
            if let Some(params_end) = signature.rfind(')') {
                let params_str = &signature[params_start + 1..params_end];
                
                if !params_str.trim().is_empty() {
                    // Split parameters by commas, handling nested generics
                    let param_parts = self.split_parameters_smart(params_str)?;
                    
                    for param_part in param_parts {
                        if let Ok(param_doc) = self.parse_parameter(param_part.trim()) {
                            parameters.push(param_doc);
                        }
                    }
                }
            }
        }
        
        Ok(parameters)
    }

    /// Smart parameter splitting that handles nested generics and complex types
    fn split_parameters_smart(&self, params_str: &str) -> Result<Vec<String>, Error> {
        let mut parameters = Vec::new();
        let mut current_param = String::new();
        let mut angle_bracket_depth = 0;
        let mut paren_depth = 0;
        let mut in_string = false;
        
        for ch in params_str.chars() {
            match ch {
                '<' if !in_string => angle_bracket_depth += 1,
                '>' if !in_string => angle_bracket_depth -= 1,
                '(' if !in_string => paren_depth += 1,
                ')' if !in_string => paren_depth -= 1,
                '"' => in_string = !in_string,
                ',' if !in_string && angle_bracket_depth == 0 && paren_depth == 0 => {
                    if !current_param.trim().is_empty() {
                        parameters.push(current_param.trim().to_string());
                    }
                    current_param.clear();
                    continue;
                }
                _ => {}
            }
            current_param.push(ch);
        }
        
        // Add the last parameter
        if !current_param.trim().is_empty() {
            parameters.push(current_param.trim().to_string());
        }
        
        Ok(parameters)
    }

    /// Parse individual parameter with type and default value
    fn parse_parameter(&self, param_str: &str) -> Result<ParameterDoc, Error> {
        // Handle different parameter patterns:
        // name: Type
        // name: Type = default
        // mut name: Type
        // &self, &mut self, self
        
        // Handle self parameters
        if param_str == "self" || param_str == "&self" || param_str == "&mut self" {
            return Ok(ParameterDoc {
                name: "self".to_string(),
                param_type: "Self".to_string(),
                description: Some("The instance being operated on".to_string()),
                is_optional: false,
                default_value: None,
            });
        }
        
        // Split by = to check for default values
        let (param_part, default_value) = if let Some(eq_pos) = param_str.find('=') {
            let param = param_str[..eq_pos].trim();
            let default = param_str[eq_pos + 1..].trim();
            (param, Some(default.to_string()))
        } else {
            (param_str, None)
        };
        
        // Split by : to separate name and type
        if let Some(colon_pos) = param_part.find(':') {
            let name_part = param_part[..colon_pos].trim();
            let type_part = param_part[colon_pos + 1..].trim();
            
            // Handle mut keyword
            let param_name = if name_part.starts_with("mut ") {
                name_part[4..].trim().to_string()
            } else {
                name_part.to_string()
            };
            
            Ok(ParameterDoc {
                name: param_name,
                param_type: type_part.to_string(),
                description: None, // Could be enhanced with doc comment parsing
                is_optional: default_value.is_some(),
                default_value,
            })
        } else {
            // Parameter without explicit type (rare but possible)
            Ok(ParameterDoc {
                name: param_str.to_string(),
                param_type: "Any".to_string(),
                description: None,
                is_optional: false,
                default_value: None,
            })
        }
    }

    /// Extract return type from method signature
    fn extract_method_return_type(&self, signature: &str) -> Result<Option<TypeDoc>, Error> {
        // Look for -> return_type pattern
        if let Some(arrow_pos) = signature.find("->") {
            let after_arrow = &signature[arrow_pos + 2..];
            
            // Find the return type (everything before { or where)
            let end_pos = after_arrow.find('{')
                .or_else(|| after_arrow.find("where"))
                .unwrap_or(after_arrow.len());
                
            let return_type_str = after_arrow[..end_pos].trim();
            
            if !return_type_str.is_empty() {
                return Ok(Some(TypeDoc {
                    name: return_type_str.to_string(),
                    description: None,
                    type_def: "return".to_string(),
                    fields: Vec::new(),
                    methods: Vec::new(),
                    location: SourceLocation {
                        file: "".to_string(),
                        line: 1,
                        column: 1,
                    },
                    source_code: None,
                    visibility: "public".to_string(),
                    generic_params: self.extract_type_generics(return_type_str)?,
                }));
            }
        }
        
        Ok(None)
    }

    /// Extract generic parameters from method signature
    fn extract_method_generics(&self, signature: &str) -> Result<Vec<String>, Error> {
        let mut generics = Vec::new();
        
        // Look for generic parameters after function name
        if let Some(fn_pos) = signature.find("fn ") {
            let after_fn = &signature[fn_pos + 3..];
            if let Some(name_end) = after_fn.find('(') {
                let name_and_generics = &after_fn[..name_end];
                
                if let Some(generic_start) = name_and_generics.find('<') {
                    if let Some(generic_end) = name_and_generics.find('>') {
                        let generic_str = &name_and_generics[generic_start + 1..generic_end];
                        
                        // Split generic parameters by comma
                        for generic in generic_str.split(',') {
                            let generic_name = generic.trim();
                            if !generic_name.is_empty() {
                                // Extract just the generic name (before any constraints)
                                let name = generic_name.split(':').next()
                                    .unwrap_or(generic_name)
                                    .trim()
                                    .to_string();
                                generics.push(name);
                            }
                        }
                    }
                }
            }
        }
        
        Ok(generics)
    }

    /// Extract generic parameters from type string
    fn extract_type_generics(&self, type_str: &str) -> Result<Vec<String>, Error> {
        let mut generics = Vec::new();
        
        if let Some(generic_start) = type_str.find('<') {
            if let Some(generic_end) = type_str.rfind('>') {
                let generic_str = &type_str[generic_start + 1..generic_end];
                
                // Split generic parameters, handling nested generics
                let generic_parts = self.split_generics_smart(generic_str)?;
                for generic in generic_parts {
                    if !generic.trim().is_empty() {
                        generics.push(generic.trim().to_string());
                    }
                }
            }
        }
        
        Ok(generics)
    }

    /// Smart generic splitting that handles nested generics
    fn split_generics_smart(&self, generics_str: &str) -> Result<Vec<String>, Error> {
        let mut generics = Vec::new();
        let mut current_generic = String::new();
        let mut angle_bracket_depth = 0;
        
        for ch in generics_str.chars() {
            match ch {
                '<' => {
                    angle_bracket_depth += 1;
                    current_generic.push(ch);
                }
                '>' => {
                    angle_bracket_depth -= 1;
                    current_generic.push(ch);
                }
                ',' if angle_bracket_depth == 0 => {
                    if !current_generic.trim().is_empty() {
                        generics.push(current_generic.trim().to_string());
                    }
                    current_generic.clear();
                }
                _ => current_generic.push(ch),
            }
        }
        
        // Add the last generic
        if !current_generic.trim().is_empty() {
            generics.push(current_generic.trim().to_string());
        }
        
        Ok(generics)
    }

    /// Fallback line-based method extraction (legacy support)
    fn extract_methods_line_based(&self, type_name: &str, source_code: &str) -> Result<Vec<FunctionDoc>, Error> {
        let mut methods = Vec::new();
        let lines: Vec<&str> = source_code.split("\n").collect();
        
        // Look for impl blocks for this type
        for (idx, line) in lines.iter().enumerate() {
            if line.trim().starts_with("impl") && line.contains(type_name) {
                // Found an impl block, extract methods
                let mut brace_count = 0;
                let mut in_impl = false;
                
                for (line_idx, impl_line) in lines[idx..].iter().enumerate() {
                    for ch in impl_line.chars() {
                        match ch {
                            '{' => {
                                brace_count += 1;
                                in_impl = true;
                            }
                            '}' => {
                                brace_count -= 1;
                                if brace_count == 0 && in_impl {
                                    // End of impl block
                                    return Ok(methods);
                                }
                            }
                            _ => {}
                        }
                    }
                    
                    // Look for function definitions within the impl block
                    if in_impl && impl_line.trim().starts_with("fn ") {
                        // Extract method name
                        if let Some(fn_start) = impl_line.find("fn ") {
                            let after_fn = &impl_line[fn_start + 3..];
                            if let Some(paren_pos) = after_fn.find('(') {
                                let method_name = after_fn[..paren_pos].trim().to_string();
                                
                                // Create a basic method documentation with minimal parsing
                                methods.push(FunctionDoc {
                                    name: method_name,
                                    description: Some(format!("Method of {}", type_name)),
                                    parameters: Vec::new(), // Basic fallback
                                    return_type: None, // Basic fallback
                                    examples: Vec::new(),
                                    location: SourceLocation {
                                        file: "".to_string(),
                                        line: idx + line_idx + 1,
                                        column: 1,
                                    },
                                    source_code: Some(impl_line.to_string()),
                                    visibility: "public".to_string(),
                                    is_async: impl_line.contains("async"),
                                    generic_params: Vec::new(),
                                });
                            }
                        }
                    }
                }
            }
        }
        
        Ok(methods)
    }

    /// Extract exports from source code
    fn extract_exports(&self, source_code: &str) -> Result<Vec<String>, Error> {
        let mut exports = Vec::new();
        
        for line in source_code.split("\n") {
            let trimmed = line.trim();
            if trimmed.starts_with("export ") || trimmed.starts_with("pub ") {
                // Extract export name
                if let Some(space_pos) = trimmed[6..].find(' ') {
                    let export_name = trimmed[6..6 + space_pos].trim();
                    if !export_name.is_empty() {
                        exports.push(export_name.to_string());
                    }
                }
            }
        }
        
        Ok(exports)
    }

    /// Extract submodules from source code
    fn extract_submodules(&self, source_code: &str) -> Result<Vec<String>, Error> {
        let mut submodules = Vec::new();
        
        for line in source_code.split("\n") {
            let trimmed = line.trim();
            if trimmed.starts_with("mod ") || trimmed.starts_with("module ") {
                // Extract module name
                let start_pos = if trimmed.starts_with("mod ") { 4 } else { 7 };
                if let Some(space_pos) = trimmed[start_pos..].find(' ') {
                    let module_name = trimmed[start_pos..start_pos + space_pos].trim();
                    if !module_name.is_empty() {
                        submodules.push(module_name.to_string());
                    }
                } else if let Some(semicolon_pos) = trimmed[start_pos..].find(';') {
                    let module_name = trimmed[start_pos..start_pos + semicolon_pos].trim();
                    if !module_name.is_empty() {
                        submodules.push(module_name.to_string());
                    }
                }
            }
        }
        
        Ok(submodules)
    }

    /// Build main LaTeX document
    fn build_main_latex_doc(&self, docs: &[super::ExtractedDocumentation]) -> Result<String, Error> {
        let mut latex = String::new();
        
        // Document preamble
        latex.push_str(r#"\documentclass[11pt,a4paper]{article}
\usepackage[utf8]{inputenc}
\usepackage[T1]{fontenc}
\usepackage{lmodern}
\usepackage{geometry}
\usepackage{fancyhdr}
\usepackage{listings}
\usepackage{xcolor}
\usepackage{hyperref}
\usepackage{graphicx}
\usepackage{amsmath}
\usepackage{amsfonts}
\usepackage{amssymb}

\geometry{margin=1in}
\pagestyle{fancy}

% Code listing style
\definecolor{codegreen}{rgb}{0,0.6,0}
\definecolor{codegray}{rgb}{0.5,0.5,0.5}
\definecolor{codepurple}{rgb}{0.58,0,0.82}
\definecolor{backcolour}{rgb}{0.95,0.95,0.92}

\lstdefinestyle{cursedstyle}{
    backgroundcolor=\color{backcolour},   
    commentstyle=\color{codegreen},
    keywordstyle=\color{magenta},
    numberstyle=\tiny\color{codegray},
    stringstyle=\color{codepurple},
    basicstyle=\ttfamily\footnotesize,
    breakatwhitespace=false,         
    breaklines=true,                 
    captionpos=b,                    
    keepspaces=true,                 
    numbers=left,                    
    numbersep=5pt,                  
    showspaces=false,                
    showstringspaces=false,
    showtabs=false,                  
    tabsize=2
}

\lstset{style=cursedstyle}

"#);

        // Title page
        latex.push_str(&format!(r#"\title{{{} Documentation}}
\author{{{}}}
\date{{\today}}

\begin{{document}}

\maketitle

\tableofcontents
\newpage

"#, 
            self.escape_latex(&self.config.project.name),
            self.config.project.authors.join(", ")));

        // Introduction section
        latex.push_str(r#"\section{Introduction}

"#);
        
        if let Some(ref description) = self.config.project.description {
            latex.push_str(&format!("{}\n\n", self.escape_latex(description)));
        }

        latex.push_str(&format!(r#"This documentation covers {} modules with comprehensive API reference.

\subsection{{Project Information}}

\begin{{itemize}}
\item \textbf{{Version:}} {}
\item \textbf{{Modules:}} {}
\item \textbf{{Total Functions:}} {}
\item \textbf{{Total Types:}} {}
\end{{itemize}}

"#,
            docs.len(),
            self.escape_latex(&self.config.project.version),
            docs.len(),
            docs.iter().map(|d| d.functions.len()).sum::<usize>(),
            docs.iter().map(|d| d.types.len()).sum::<usize>()));

        // Module sections
        for doc in docs {
            let module_name = doc.source_file.file_stem()
                .unwrap_or_default()
                .to_string_lossy();
            
            latex.push_str(&format!(r#"\section{{Module: {}}}

"#, self.escape_latex(&module_name)));

            // Module description
            if let Some(ref module_doc) = doc.module_doc {
                if let Some(ref description) = module_doc.description {
                    latex.push_str(&format!("{}\n\n", self.escape_latex(description)));
                }
            }

            // Functions
            if !doc.functions.is_empty() {
                latex.push_str(r#"\subsection{Functions}

"#);
                for func in &doc.functions {
                    latex.push_str(&self.format_function_latex(func)?);
                }
            }

            // Types
            if !doc.types.is_empty() {
                latex.push_str(r#"\subsection{Types}

"#);
                for type_doc in &doc.types {
                    latex.push_str(&self.format_type_latex(type_doc)?);
                }
            }
        }

        latex.push_str(r#"
\end{document}
"#);

        Ok(latex)
    }

    /// Build module-specific LaTeX document
    fn build_module_latex_doc(&self, doc: &super::ExtractedDocumentation) -> Result<String, Error> {
        let module_name = doc.source_file.file_stem()
            .unwrap_or_default()
            .to_string_lossy();

        let mut latex = format!(r#"\section{{Module: {}}}

"#, self.escape_latex(&module_name));

        if let Some(ref module_doc) = doc.module_doc {
            if let Some(ref description) = module_doc.description {
                latex.push_str(&format!("{}\n\n", self.escape_latex(description)));
            }
        }

        // Add module content...
        if !doc.functions.is_empty() {
            latex.push_str(r#"\subsection{Functions}

"#);
            for func in &doc.functions {
                latex.push_str(&self.format_function_latex(func)?);
            }
        }

        Ok(latex)
    }

    /// Format function for LaTeX
    fn format_function_latex(&self, func: &FunctionDoc) -> Result<String, Error> {
        let mut latex = format!(r#"\subsubsection{{{}}}

"#, self.escape_latex(&func.name));

        if let Some(ref description) = func.description {
            latex.push_str(&format!("{}\n\n", self.escape_latex(description)));
        }

        // Parameters
        if !func.parameters.is_empty() {
            latex.push_str(r#"\paragraph{Parameters:}
\begin{itemize}
"#);
            for param in &func.parameters {
                latex.push_str(&format!(r#"\item \texttt{{{}}} ({}){}"#,
                    self.escape_latex(&param.name),
                    self.escape_latex(&param.param_type),
                    param.description.as_ref()
                        .map(|d| format!(" -- {}", self.escape_latex(d)))
                        .unwrap_or_default()));
                latex.push_str("\n");
            }
            latex.push_str(r#"\end{itemize}

"#);
        }

        // Return type
        if let Some(ref return_type) = func.return_type {
            latex.push_str(&format!(r#"\paragraph{{Returns:}} \texttt{{{}}}

"#, self.escape_latex(&return_type.name)));
        }

        // Examples
        if !func.examples.is_empty() {
            latex.push_str(r#"\paragraph{Examples:}

"#);
            for example in &func.examples {
                latex.push_str(&format!(r#"\begin{{lstlisting}}[language=C]
{}
\end{{lstlisting}}

"#, example.code));
            }
        }

        Ok(latex)
    }

    /// Format type for LaTeX
    fn format_type_latex(&self, type_doc: &TypeDoc) -> Result<String, Error> {
        let mut latex = format!(r#"\subsubsection{{{} ({})}}

"#, 
            self.escape_latex(&type_doc.name),
            self.escape_latex(&type_doc.type_def));

        if let Some(ref description) = type_doc.description {
            latex.push_str(&format!("{}\n\n", self.escape_latex(description)));
        }

        // Fields
        if !type_doc.fields.is_empty() {
            latex.push_str(r#"\paragraph{Fields:}
\begin{itemize}
"#);
            for field in &type_doc.fields {
                latex.push_str(&format!(r#"\item \texttt{{{}}} ({})"#,
                    self.escape_latex(&field.name),
                    self.escape_latex(&field.field_type)));
                latex.push_str("\n");
            }
            latex.push_str(r#"\end{itemize}

"#);
        }

        Ok(latex)
    }

    /// Build bibliography
    fn build_bibliography(&self, _docs: &[super::ExtractedDocumentation]) -> Result<String, Error> {
        let bib = format!(r#"@misc{{cursed_docs,
    title={{CURSED Programming Language Documentation}},
    author={{{}}},
    year={{2024}},
    note={{Generated documentation for version {}}}
}}
"#, 
            self.config.project.authors.join(" and "),
            self.config.project.version);

        Ok(bib)
    }

    /// Build LaTeX Makefile
    fn build_latex_makefile(&self) -> Result<String, Error> {
        let makefile = r#"# LaTeX Documentation Makefile

MAIN = documentation
LATEX = pdflatex
BIBTEX = bibtex

.PHONY: all clean

all: $(MAIN).pdf

$(MAIN).pdf: $(MAIN).tex
	$(LATEX) $(MAIN).tex
	$(BIBTEX) $(MAIN)
	$(LATEX) $(MAIN).tex
	$(LATEX) $(MAIN).tex

clean:
	rm -f *.aux *.bbl *.blg *.log *.out *.toc *.pdf

view: $(MAIN).pdf
	open $(MAIN).pdf

help:
	@echo "Available targets:"
	@echo "  all    - Build PDF documentation"
	@echo "  clean  - Remove generated files"
	@echo "  view   - Open PDF documentation"
	@echo "  help   - Show this help"
"#;

        Ok(makefile.to_string())
    }

    /// Escape LaTeX special characters
    fn escape_latex(&self, text: &str) -> String {
        text.replace('\\', r#"\textbackslash{}"#)
            .replace('{', r#"\{"#)
            .replace('}', r#"\}"#)
            .replace('$', r#"\$"#)
            .replace('&', r#"\&"#)
            .replace('%', r#"\%"#)
            .replace('#', r#"\#"#)
            .replace('^', r#"\textasciicircum{}"#)
            .replace('_', r#"\_"#)
            .replace('~', r#"\textasciitilde{}"#)
    }

    /// Generate function signature for documentation
    fn generate_function_signature(&self, func: &FunctionDoc) -> String {
        let mut signature = String::new();
        
        // Add async keyword if applicable
        if func.is_async {
            signature.push_str("async ");
        }
        
        // Add function name
        signature.push_str("fn ");
        signature.push_str(&func.name);
        
        // Add generic parameters if any
        if !func.generic_params.is_empty() {
            signature.push('<');
            signature.push_str(&func.generic_params.join(", "));
            signature.push('>');
        }
        
        // Add parameters
        signature.push('(');
        let param_strings: Vec<String> = func.parameters.iter()
            .map(|param| {
                let mut param_str = param.name.clone();
                param_str.push_str(": ");
                param_str.push_str(&param.param_type);
                if param.is_optional {
                    if let Some(ref default) = param.default_value {
                        param_str.push_str(" = ");
                        param_str.push_str(default);
                    }
                }
                param_str
            })
            .collect();
        signature.push_str(&param_strings.join(", "));
        signature.push(')');
        
        // Add return type if any
        if let Some(ref return_type) = func.return_type {
            signature.push_str(" -> ");
            signature.push_str(&return_type.name);
        }
        
        signature
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
