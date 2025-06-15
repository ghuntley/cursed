//! AST Documentation Module
//!
//! Comprehensive documentation extraction and analysis for CURSED AST nodes.
//! This module provides the core infrastructure for extracting documentation
//! from AST nodes and building rich documentation metadata.

use crate::ast::traits::*;
use crate::ast::*;
use crate::error::{Error, SourceLocation};
use crate::lexer::{Token, TokenType};
use std::collections::{HashMap, BTreeMap, HashSet};
use std::path::{Path, PathBuf};
use std::fmt;
use serde::{Deserialize, Serialize};
use chrono;
use regex;

/// Main coordinator for extracting documentation from AST nodes
/// 
/// This struct orchestrates the extraction of documentation from parsed AST nodes,
/// building comprehensive metadata about functions, structs, interfaces, and other
/// language constructs. It maintains cross-references and generates documentation
/// suitable for multiple output formats.
#[derive(Debug)]
pub struct DocumentationExtractor {
    /// Configuration for extraction behavior
    config: ExtractionConfig,
    /// Cache of extracted documentation by module
    module_cache: HashMap<String, ModuleDocumentation>,
    /// Cross-reference mapping between documented elements
    cross_references: HashMap<String, Vec<CrossReference>>,
    /// Symbol table for resolving references
    symbol_table: HashMap<String, DocumentedSymbol>,
    /// Comment parser for extracting structured documentation
    comment_parser: CommentParser,
}

/// Configuration for documentation extraction
#[derive(Debug, Clone)]
pub struct ExtractionConfig {
    /// Include private items in documentation
    pub include_private: bool,
    /// Extract inline code examples
    pub extract_examples: bool,
    /// Generate cross-references
    pub generate_cross_refs: bool,
    /// Maximum depth for recursive type analysis
    pub max_depth: usize,
    /// Include source code snippets
    pub include_source: bool,
    /// Language-specific settings
    pub language_settings: LanguageSettings,
}

/// Language-specific documentation settings
#[derive(Debug, Clone)]
pub struct LanguageSettings {
    /// CURSED-specific keyword mappings
    pub keyword_mappings: HashMap<String, String>,
    /// Gen Z slang documentation style
    pub use_slang_docs: bool,
    /// Include type signatures
    pub include_signatures: bool,
}

impl Default for ExtractionConfig {
    fn default() -> Self {
        let mut keyword_mappings = HashMap::new();
        keyword_mappings.insert("slay".to_string(), "function".to_string());
        keyword_mappings.insert("sus".to_string(), "variable".to_string());
        keyword_mappings.insert("facts".to_string(), "constant".to_string());
        keyword_mappings.insert("squad".to_string(), "struct".to_string());
        keyword_mappings.insert("collab".to_string(), "interface".to_string());
        keyword_mappings.insert("yolo".to_string(), "yield".to_string());
        keyword_mappings.insert("stan".to_string(), "spawn_goroutine".to_string());

        Self {
            include_private: false,
            extract_examples: true,
            generate_cross_refs: true,
            max_depth: 10,
            include_source: true,
            language_settings: LanguageSettings {
                keyword_mappings,
                use_slang_docs: true,
                include_signatures: true,
            },
        }
    }
}

impl DocumentationExtractor {
    /// Create a new documentation extractor with default configuration
    pub fn new() -> Self {
        Self::with_config(ExtractionConfig::default())
    }

    /// Create a new documentation extractor with custom configuration
    pub fn with_config(config: ExtractionConfig) -> Self {
        Self {
            config,
            module_cache: HashMap::new(),
            cross_references: HashMap::new(),
            symbol_table: HashMap::new(),
            comment_parser: CommentParser::new(),
        }
    }

    /// Extract documentation from a complete program
    pub fn extract_program_documentation(&mut self, program: &Program, file_path: &Path) -> Result<ModuleDocumentation, Error> {
        let module_name = self.derive_module_name(file_path);
        
        // Extract package-level documentation
        let package_info = PackageInfo {
            name: program.package_name.clone(),
            description: None,
            version: None,
            authors: Vec::new(),
            dependencies: program.imports.iter().map(|imp| imp.path.clone()).collect(),
        };

        // Extract import documentation
        let imports = self.extract_import_docs(&program.imports)?;
        
        // Extract documentation from statements
        let mut items = Vec::new();
        let mut module_comments = Vec::new();
        
        for statement in &program.statements {
            match self.extract_statement_documentation(statement, &module_name) {
                Ok(Some(doc_element)) => items.push(doc_element),
                Ok(None) => {}, // Not a documentable statement
                Err(e) => return Err(e),
            }
        }

        // Build module documentation
        let module_doc = ModuleDocumentation {
            name: module_name.clone(),
            file_path: file_path.to_path_buf(),
            package_info: Some(package_info),
            imports,
            items,
            module_comments,
            metadata: DocumentationMetadata::new(&module_name),
            source_info: self.gather_source_info(file_path)?,
        };

        // Cache the module documentation
        self.module_cache.insert(module_name, module_doc.clone());
        
        // Update symbol table
        self.update_symbol_table(&module_doc);

        Ok(module_doc)
    }

    /// Extract documentation from individual statements
    pub fn extract_statement_documentation(&mut self, statement: &dyn Statement, module: &str) -> Result<Option<DocElement>, Error> {
        use crate::ast::declarations::{FunctionStatement, SquadStatement, CollabStatement};
        use crate::ast::statements::variable::VariableStatement;
        
        // Try to downcast to specific statement types for detailed extraction
        let any_stmt = statement.as_any();
        
        // Function declarations
        if let Some(func_stmt) = any_stmt.downcast_ref::<FunctionStatement>() {
            return Ok(Some(self.extract_function_documentation(func_stmt, module)?));
        }
        
        // Struct declarations
        if let Some(struct_stmt) = any_stmt.downcast_ref::<SquadStatement>() {
            return Ok(Some(self.extract_struct_documentation(struct_stmt, module)?));
        }
        
        // Interface declarations  
        if let Some(interface_stmt) = any_stmt.downcast_ref::<CollabStatement>() {
            return Ok(Some(self.extract_interface_documentation(interface_stmt, module)?));
        }
        
        // Variable declarations
        if let Some(var_stmt) = any_stmt.downcast_ref::<VariableStatement>() {
            return Ok(Some(self.extract_variable_documentation(var_stmt, module)?));
        }

        // For other statement types, return None (not documentable)
        Ok(None)
    }

    /// Extract function documentation
    fn extract_function_documentation(&mut self, func: &FunctionStatement, module: &str) -> Result<DocElement, Error> {
        let location = SourceLocation { line: 1, column: 1, file: None };
        
        // Extract function name and signature
        let func_name = &func.name.value;
        let signature = self.build_function_signature(func);
        
        // Extract parameter documentation
        let parameters = self.extract_parameter_docs(&func.parameters)?;
        
        // Extract return type information
        let return_type = func.return_type.as_ref().map(|rt| rt.string());
        
        // Parse documentation comments
        let (summary, description, tags, examples) = self.comment_parser.parse_documentation_for_location(&location)?;
        
        // Functions are typically public in CURSED
        let visibility = Visibility::Public;
        
        Ok(DocElement {
            name: func_name.clone(),
            element_type: ElementType::Function,
            visibility,
            module: module.to_string(),
            summary: if summary.is_empty() { 
                format!("Function {}", func_name) 
            } else { 
                summary 
            },
            description: if description.is_empty() { 
                Some(format!("CURSED function declaration using the 'slay' keyword"))
            } else { 
                Some(description) 
            },
            signature: Some(signature),
            parameters,
            return_type,
            type_info: None,
            examples,
            tags,
            location,
            source_code: if self.config.include_source { Some(func.string()) } else { None },
            metadata: ElementMetadata::from_function(func),
        })
    }

    /// Extract struct documentation
    fn extract_struct_documentation(&mut self, struct_stmt: &SquadStatement, module: &str) -> Result<DocElement, Error> {
        let location = SourceLocation { line: 1, column: 1, file: None };
        
        let struct_name = &struct_stmt.name.value;
        let signature = self.build_struct_signature(struct_stmt);
        
        // Extract field documentation
        let field_docs = self.extract_field_docs(&struct_stmt.fields)?;
        
        // Parse documentation comments
        let (summary, description, tags, examples) = self.comment_parser.parse_documentation_for_location(&location)?;
        
        // Build type information
        let type_info = Some(TypeInfo {
            base_type: "struct".to_string(),
            generic_params: struct_stmt.type_parameters.iter().map(|p| p.name.clone()).collect(),
            constraints: Vec::new(),
            fields: field_docs,
            methods: Vec::new(),
        });

        Ok(DocElement {
            name: struct_name.clone(),
            element_type: ElementType::Struct,
            visibility: Visibility::Public, // Structs are typically public in CURSED
            module: module.to_string(),
            summary: if summary.is_empty() { 
                format!("Struct {}", struct_name) 
            } else { 
                summary 
            },
            description: if description.is_empty() { 
                Some(format!("CURSED struct declaration using the 'squad' keyword"))
            } else { 
                Some(description) 
            },
            signature: Some(signature),
            parameters: Vec::new(),
            return_type: None,
            type_info,
            examples,
            tags,
            location,
            source_code: if self.config.include_source { Some(struct_stmt.string()) } else { None },
            metadata: ElementMetadata::from_struct(struct_stmt),
        })
    }

    /// Extract interface documentation
    fn extract_interface_documentation(&mut self, interface: &CollabStatement, module: &str) -> Result<DocElement, Error> {
        let location = SourceLocation { line: 1, column: 1, file: None };
        
        let interface_name = &interface.name.value;
        let signature = self.build_interface_signature(interface);
        
        // Extract method documentation
        let method_docs = self.extract_method_docs(&interface.methods)?;
        
        // Parse documentation comments
        let (summary, description, tags, examples) = self.comment_parser.parse_documentation_for_location(&location)?;
        
        // Build type information
        let type_info = Some(TypeInfo {
            base_type: "interface".to_string(),
            generic_params: interface.type_parameters.iter().map(|p| p.name.clone()).collect(),
            constraints: Vec::new(),
            fields: Vec::new(),
            methods: method_docs,
        });

        Ok(DocElement {
            name: interface_name.clone(),
            element_type: ElementType::Interface,
            visibility: Visibility::Public, // Interfaces are typically public
            module: module.to_string(),
            summary: if summary.is_empty() { 
                format!("Interface {}", interface_name) 
            } else { 
                summary 
            },
            description: if description.is_empty() { 
                Some(format!("CURSED interface declaration using the 'collab' keyword"))
            } else { 
                Some(description) 
            },
            signature: Some(signature),
            parameters: Vec::new(),
            return_type: None,
            type_info,
            examples,
            tags,
            location,
            source_code: if self.config.include_source { Some(interface.string()) } else { None },
            metadata: ElementMetadata::from_interface(interface),
        })
    }

    /// Extract variable documentation
    fn extract_variable_documentation(&mut self, var: &VariableStatement, module: &str) -> Result<DocElement, Error> {
        let location = SourceLocation { line: 1, column: 1, file: None };
        
        let var_name = &var.name;
        let keyword = if var.is_mutable { "sus" } else { "facts" };
        let signature = format!("{} {}", keyword, var_name);
        
        // Determine element type based on mutability
        let element_type = if var.is_mutable {
            ElementType::Variable
        } else {
            ElementType::Constant
        };
        
        // Parse documentation comments
        let (summary, description, tags, examples) = self.comment_parser.parse_documentation_for_location(&location)?;

        Ok(DocElement {
            name: var_name.clone(),
            element_type,
            visibility: Visibility::Private, // Variables are typically private unless exported
            module: module.to_string(),
            summary: if summary.is_empty() { 
                format!("{} {}", if var.is_mutable { "Variable" } else { "Constant" }, var_name) 
            } else { 
                summary 
            },
            description: if description.is_empty() { 
                Some(format!("CURSED {} declaration using the '{}' keyword", 
                           if var.is_mutable { "variable" } else { "constant" }, keyword))
            } else { 
                Some(description) 
            },
            signature: Some(signature),
            parameters: Vec::new(),
            return_type: var.var_type.clone(),
            type_info: None,
            examples,
            tags,
            location,
            source_code: if self.config.include_source { Some(var.string()) } else { None },
            metadata: ElementMetadata::from_variable(var),
        })
    }

    /// Build function signature string
    fn build_function_signature(&self, func: &FunctionStatement) -> String {
        use crate::ast::traits::Node;
        
        let mut sig = String::new();
        
        // Add function keyword (slay in CURSED)
        sig.push_str("slay ");
        
        // Add function name
        sig.push_str(&func.name.value);
        
        // Add generic parameters if present
        if !func.type_parameters.is_empty() {
            sig.push('<');
            let type_params: Vec<String> = func.type_parameters.iter().map(|p| p.name.clone()).collect();
            sig.push_str(&type_params.join(", "));
            sig.push('>');
        }
        
        // Add parameters
        sig.push('(');
        let param_strs: Vec<String> = func.parameters.iter().map(|p| p.string()).collect();
        sig.push_str(&param_strs.join(", "));
        sig.push(')');
        
        // Add return type if present
        if let Some(return_type) = &func.return_type {
            sig.push_str(" -> ");
            sig.push_str(&return_type.string());
        }
        
        sig
    }

    /// Build struct signature string
    fn build_struct_signature(&self, struct_stmt: &SquadStatement) -> String {
        let mut sig = String::new();
        
        sig.push_str("squad ");
        sig.push_str(&struct_stmt.name.value);
        
        // Add generic parameters if present
        if !struct_stmt.type_parameters.is_empty() {
            sig.push('<');
            let type_params: Vec<String> = struct_stmt.type_parameters.iter().map(|p| p.name.clone()).collect();
            sig.push_str(&type_params.join(", "));
            sig.push('>');
        }
        
        sig
    }

    /// Build interface signature string
    fn build_interface_signature(&self, interface: &CollabStatement) -> String {
        let mut sig = String::new();
        
        sig.push_str("collab ");
        sig.push_str(&interface.name.value);
        
        // Add generic parameters if present
        if !interface.type_parameters.is_empty() {
            sig.push('<');
            let type_params: Vec<String> = interface.type_parameters.iter().map(|p| p.name.clone()).collect();
            sig.push_str(&type_params.join(", "));
            sig.push('>');
        }
        
        sig
    }

    /// Extract parameter documentation
    fn extract_parameter_docs(&self, parameters: &[crate::ast::expressions::Parameter]) -> Result<Vec<ParameterDoc>, Error> {
        use crate::ast::traits::Node;
        
        let mut param_docs = Vec::new();
        
        for param in parameters {
            let param_doc = ParameterDoc {
                name: param.name.clone(),
                param_type: param.param_type.as_ref().map(|t| t.string()),
                description: format!("Parameter {}", param.name), // Would be extracted from comments
                default_value: param.default_value.as_ref().map(|v| v.string()),
                is_optional: param.default_value.is_some(),
            };
            param_docs.push(param_doc);
        }
        
        Ok(param_docs)
    }

    /// Extract field documentation
    fn extract_field_docs(&self, fields: &[crate::ast::declarations::FieldStatement]) -> Result<Vec<FieldDoc>, Error> {
        let mut field_docs = Vec::new();
        
        for field in fields {
            let field_doc = FieldDoc {
                name: field.name.value.clone(),
                field_type: field.type_name.value.clone(),
                description: format!("Field {}", field.name.value), // Would be extracted from comments
                is_public: true, // Fields in CURSED structs are typically public
                default_value: None, // Could be extracted if present
            };
            field_docs.push(field_doc);
        }
        
        Ok(field_docs)
    }

    /// Extract method documentation
    fn extract_method_docs(&self, methods: &[crate::ast::declarations::MethodDeclaration]) -> Result<Vec<MethodDoc>, Error> {
        use crate::ast::traits::Node;
        
        let mut method_docs = Vec::new();
        
        for method in methods {
            let method_doc = MethodDoc {
                name: method.name.value.clone(),
                signature: method.string(),
                description: format!("Method {}", method.name.value), // Would be extracted from comments
                parameters: self.extract_parameter_docs(&method.parameters)?,
                return_type: method.return_type.as_ref().map(|t| t.string()),
                is_static: false, // Would need to be determined from context
            };
            method_docs.push(method_doc);
        }
        
        Ok(method_docs)
    }

    /// Extract import documentation
    fn extract_import_docs(&self, imports: &[ImportStatement]) -> Result<Vec<ImportDoc>, Error> {
        let mut import_docs = Vec::new();
        
        for import in imports {
            let import_doc = ImportDoc {
                path: import.path.clone(),
                alias: import.alias.clone(),
                description: format!("Import from {}", import.path),
                is_public: false, // Imports are typically not re-exported
            };
            import_docs.push(import_doc);
        }
        
        Ok(import_docs)
    }

    /// Build cross-reference map
    pub fn build_cross_references(&mut self) -> Result<(), Error> {
        if !self.config.generate_cross_refs {
            return Ok(());
        }

        let mut references = HashMap::new();
        
        for (module_name, module_doc) in &self.module_cache {
            for item in &module_doc.items {
                let item_refs = self.find_references_for_item(item)?;
                if !item_refs.is_empty() {
                    references.insert(format!("{}::{}", module_name, item.name), item_refs);
                }
            }
        }
        
        self.cross_references = references;
        Ok(())
    }

    /// Find cross-references for a documentation item
    fn find_references_for_item(&self, item: &DocElement) -> Result<Vec<CrossReference>, Error> {
        let mut refs = Vec::new();
        
        // Search in signature
        if let Some(signature) = &item.signature {
            refs.extend(self.find_references_in_text(signature, &item.location)?);
        }
        
        // Search in description
        if let Some(description) = &item.description {
            refs.extend(self.find_references_in_text(description, &item.location)?);
        }
        
        // Search in examples
        for example in &item.examples {
            refs.extend(self.find_references_in_text(&example.code, &item.location)?);
        }
        
        Ok(refs)
    }

    /// Find references in text content
    fn find_references_in_text(&self, text: &str, location: &SourceLocation) -> Result<Vec<CrossReference>, Error> {
        let mut refs = Vec::new();
        
        // Simple implementation - look for known symbols
        for symbol_name in self.symbol_table.keys() {
            if text.contains(symbol_name) {
                refs.push(CrossReference {
                    target: symbol_name.clone(),
                    context: text.to_string(),
                    location: location.clone(),
                    reference_type: ReferenceType::Mention,
                });
            }
        }
        
        Ok(refs)
    }

    /// Update symbol table with module documentation
    fn update_symbol_table(&mut self, module_doc: &ModuleDocumentation) {
        for item in &module_doc.items {
            let symbol = DocumentedSymbol {
                name: item.name.clone(),
                element_type: item.element_type.clone(),
                module: item.module.clone(),
                location: item.location.clone(),
                signature: item.signature.clone(),
            };
            
            let fully_qualified_name = format!("{}::{}", module_doc.name, item.name);
            self.symbol_table.insert(fully_qualified_name, symbol);
        }
    }

    /// Infer expression type for variables (simplified)
    fn infer_expression_type(&self, _expr: &dyn Expression) -> String {
        // Simplified type inference - would need full type system integration
        "unknown".to_string()
    }

    /// Derive module name from file path
    fn derive_module_name(&self, file_path: &Path) -> String {
        file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string()
    }

    /// Gather source file information
    fn gather_source_info(&self, file_path: &Path) -> Result<SourceInfo, Error> {
        use std::fs;
        
        let metadata = fs::metadata(file_path).map_err(Error::Io)?;
        let file_size = metadata.len();
        
        let source = fs::read_to_string(file_path).map_err(Error::Io)?;
        let line_count = source.lines().count();
        
        let last_modified = metadata.modified().ok();
        
        Ok(SourceInfo {
            file_size,
            line_count,
            last_modified,
            encoding: "UTF-8".to_string(),
        })
    }

    /// Get extracted documentation for a module
    pub fn get_module_documentation(&self, module_name: &str) -> Option<&ModuleDocumentation> {
        self.module_cache.get(module_name)
    }

    /// Get all extracted modules
    pub fn get_all_modules(&self) -> Vec<&ModuleDocumentation> {
        self.module_cache.values().collect()
    }

    /// Get cross-references for an item
    pub fn get_cross_references(&self, item_name: &str) -> Option<&Vec<CrossReference>> {
        self.cross_references.get(item_name)
    }

    /// Get symbol by fully qualified name
    pub fn get_symbol(&self, name: &str) -> Option<&DocumentedSymbol> {
        self.symbol_table.get(name)
    }

    /// Export documentation as structured data
    pub fn export_documentation(&self) -> ExportedDocumentation {
        ExportedDocumentation {
            modules: self.module_cache.values().cloned().collect(),
            cross_references: self.cross_references.clone(),
            symbol_table: self.symbol_table.clone(),
            metadata: ExportMetadata {
                generator_version: env!("CARGO_PKG_VERSION").to_string(),
                generated_at: chrono::Utc::now(),
                total_modules: self.module_cache.len(),
                total_items: self.module_cache.values().map(|m| m.items.len()).sum(),
            },
        }
    }
}

/// Represents different types of documentable elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocElement {
    /// Name of the documented element
    pub name: String,
    /// Type of element (function, struct, etc.)
    pub element_type: ElementType,
    /// Visibility level
    pub visibility: Visibility,
    /// Module containing this element
    pub module: String,
    /// Brief summary/description
    pub summary: String,
    /// Detailed description
    pub description: Option<String>,
    /// Type signature or declaration
    pub signature: Option<String>,
    /// Parameter documentation
    pub parameters: Vec<ParameterDoc>,
    /// Return type information
    pub return_type: Option<String>,
    /// Type information for complex types
    pub type_info: Option<TypeInfo>,
    /// Code examples
    pub examples: Vec<CodeExample>,
    /// Documentation tags (author, since, deprecated, etc.)
    pub tags: HashMap<String, Vec<String>>,
    /// Source location
    pub location: SourceLocation,
    /// Source code snippet
    pub source_code: Option<String>,
    /// Additional metadata
    pub metadata: ElementMetadata,
}

/// Types of documentable elements
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ElementType {
    Function,
    Struct,
    Interface,
    Variable,
    Constant,
    Type,
    Module,
    Macro,
    Other,
}

impl fmt::Display for ElementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ElementType::Function => write!(f, "function"),
            ElementType::Struct => write!(f, "struct"),
            ElementType::Interface => write!(f, "interface"),
            ElementType::Variable => write!(f, "variable"),
            ElementType::Constant => write!(f, "constant"),
            ElementType::Type => write!(f, "type"),
            ElementType::Module => write!(f, "module"),
            ElementType::Macro => write!(f, "macro"),
            ElementType::Other => write!(f, "other"),
        }
    }
}

/// Item visibility levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private,
    Protected,
    Internal,
}

/// Rich metadata about documented items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationMetadata {
    /// Module this documentation belongs to
    pub module: String,
    /// Timestamp when extracted
    pub extracted_at: chrono::DateTime<chrono::Utc>,
    /// Version of documentation format
    pub format_version: String,
    /// Language-specific metadata
    pub language_metadata: LanguageMetadata,
    /// Statistics about the documentation
    pub statistics: DocumentationStatistics,
}

impl DocumentationMetadata {
    pub fn new(module: &str) -> Self {
        Self {
            module: module.to_string(),
            extracted_at: chrono::Utc::now(),
            format_version: "1.0".to_string(),
            language_metadata: LanguageMetadata::default(),
            statistics: DocumentationStatistics::default(),
        }
    }
}

/// Language-specific metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageMetadata {
    /// CURSED language version
    pub language_version: String,
    /// Compiler version used
    pub compiler_version: String,
    /// Target platform
    pub target_platform: String,
    /// Feature flags enabled
    pub features: Vec<String>,
}

impl Default for LanguageMetadata {
    fn default() -> Self {
        Self {
            language_version: "0.1.0".to_string(),
            compiler_version: env!("CARGO_PKG_VERSION").to_string(),
            target_platform: std::env::consts::OS.to_string(),
            features: Vec::new(),
        }
    }
}

/// Documentation statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentationStatistics {
    /// Total number of documented items
    pub total_items: usize,
    /// Number of functions
    pub function_count: usize,
    /// Number of structs
    pub struct_count: usize,
    /// Number of interfaces
    pub interface_count: usize,
    /// Number of variables
    pub variable_count: usize,
    /// Number of constants
    pub constant_count: usize,
    /// Coverage percentage
    pub coverage_percentage: f64,
}

/// Cross-reference between documented elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossReference {
    /// Target of the reference
    pub target: String,
    /// Context where reference appears
    pub context: String,
    /// Location of the reference
    pub location: SourceLocation,
    /// Type of reference
    pub reference_type: ReferenceType,
}

/// Types of cross-references
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReferenceType {
    /// Direct usage/call
    Usage,
    /// Type annotation
    TypeReference,
    /// Import statement
    Import,
    /// Inheritance/implementation
    Inheritance,
    /// Documentation mention
    Mention,
}

/// Parameter documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDoc {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: Option<String>,
    /// Description of parameter
    pub description: String,
    /// Default value if any
    pub default_value: Option<String>,
    /// Whether parameter is optional
    pub is_optional: bool,
}

/// Field documentation for structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDoc {
    /// Field name
    pub name: String,
    /// Field type
    pub field_type: String,
    /// Field description
    pub description: String,
    /// Whether field is public
    pub is_public: bool,
    /// Default value if any
    pub default_value: Option<String>,
}

/// Method documentation for interfaces
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodDoc {
    /// Method name
    pub name: String,
    /// Method signature
    pub signature: String,
    /// Method description
    pub description: String,
    /// Parameter documentation
    pub parameters: Vec<ParameterDoc>,
    /// Return type
    pub return_type: Option<String>,
    /// Whether method is static
    pub is_static: bool,
}

/// Import documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportDoc {
    /// Import path
    pub path: String,
    /// Import alias
    pub alias: Option<String>,
    /// Import description
    pub description: String,
    /// Whether import is re-exported
    pub is_public: bool,
}

/// Type information for complex types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeInfo {
    /// Base type name
    pub base_type: String,
    /// Generic parameters
    pub generic_params: Vec<String>,
    /// Type constraints
    pub constraints: Vec<String>,
    /// Fields for struct types
    pub fields: Vec<FieldDoc>,
    /// Methods for interface types
    pub methods: Vec<MethodDoc>,
}

/// Code example with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExample {
    /// Example title
    pub title: Option<String>,
    /// Example description
    pub description: Option<String>,
    /// Code content
    pub code: String,
    /// Programming language
    pub language: String,
    /// Expected output
    pub output: Option<String>,
    /// Whether example can be executed
    pub is_runnable: bool,
}

/// Additional metadata for elements
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ElementMetadata {
    /// When element was first added
    pub since_version: Option<String>,
    /// If element is deprecated
    pub is_deprecated: bool,
    /// Deprecation message
    pub deprecation_message: Option<String>,
    /// Stability level
    pub stability: StabilityLevel,
    /// Performance characteristics
    pub performance_notes: Vec<String>,
    /// Security considerations
    pub security_notes: Vec<String>,
}

impl ElementMetadata {
    pub fn from_function(func: &FunctionStatement) -> Self {
        Self {
            stability: StabilityLevel::Stable,
            ..Default::default()
        }
    }

    pub fn from_struct(struct_stmt: &SquadStatement) -> Self {
        Self {
            stability: StabilityLevel::Stable,
            ..Default::default()
        }
    }

    pub fn from_interface(interface: &CollabStatement) -> Self {
        Self {
            stability: StabilityLevel::Stable,
            ..Default::default()
        }
    }

    pub fn from_variable(var: &VariableStatement) -> Self {
        Self {
            stability: StabilityLevel::Stable,
            ..Default::default()
        }
    }
}

/// Stability levels for API elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StabilityLevel {
    Experimental,
    Unstable,
    Stable,
    Deprecated,
}

impl Default for StabilityLevel {
    fn default() -> Self {
        StabilityLevel::Stable
    }
}

/// Module-level documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDocumentation {
    /// Module name
    pub name: String,
    /// Source file path
    pub file_path: PathBuf,
    /// Package information
    pub package_info: Option<PackageInfo>,
    /// Import documentation
    pub imports: Vec<ImportDoc>,
    /// Documented items in module
    pub items: Vec<DocElement>,
    /// Module-level comments
    pub module_comments: Vec<String>,
    /// Module metadata
    pub metadata: DocumentationMetadata,
    /// Source file information
    pub source_info: SourceInfo,
}

/// Package information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    /// Package name
    pub name: Option<String>,
    /// Package description
    pub description: Option<String>,
    /// Package version
    pub version: Option<String>,
    /// Package authors
    pub authors: Vec<String>,
    /// Package dependencies
    pub dependencies: Vec<String>,
}

/// Source file information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceInfo {
    /// File size in bytes
    pub file_size: u64,
    /// Number of lines
    pub line_count: usize,
    /// Last modification time
    pub last_modified: Option<std::time::SystemTime>,
    /// File encoding
    pub encoding: String,
}

/// Documented symbol for cross-referencing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentedSymbol {
    /// Symbol name
    pub name: String,
    /// Element type
    pub element_type: ElementType,
    /// Module containing symbol
    pub module: String,
    /// Source location
    pub location: SourceLocation,
    /// Symbol signature
    pub signature: Option<String>,
}

/// Comment parser for extracting structured documentation
#[derive(Debug)]
pub struct CommentParser {
    /// Patterns for detecting documentation tags
    tag_patterns: HashMap<String, regex::Regex>,
}

impl CommentParser {
    pub fn new() -> Self {
        let mut tag_patterns = HashMap::new();
        
        // Common documentation tags
        tag_patterns.insert("param".to_string(), regex::Regex::new(r"@param\s+(\w+)\s+(.+)").unwrap());
        tag_patterns.insert("return".to_string(), regex::Regex::new(r"@return\s+(.+)").unwrap());
        tag_patterns.insert("example".to_string(), regex::Regex::new(r"@example\s*\n((?:.|\n)*?)(?=@|\z)").unwrap());
        tag_patterns.insert("since".to_string(), regex::Regex::new(r"@since\s+(.+)").unwrap());
        tag_patterns.insert("deprecated".to_string(), regex::Regex::new(r"@deprecated\s*(.*)").unwrap());
        tag_patterns.insert("author".to_string(), regex::Regex::new(r"@author\s+(.+)").unwrap());
        
        Self {
            tag_patterns,
        }
    }

    /// Parse documentation for a specific location
    pub fn parse_documentation_for_location(&self, _location: &SourceLocation) -> Result<(String, String, HashMap<String, Vec<String>>, Vec<CodeExample>), Error> {
        // Simplified implementation - in practice would extract comments from token stream
        Ok((
            String::new(),      // summary
            String::new(),      // description
            HashMap::new(),     // tags
            Vec::new(),         // examples
        ))
    }

    /// Parse documentation content
    pub fn parse_documentation_content(&self, content: &str) -> Result<(String, String, HashMap<String, Vec<String>>, Vec<CodeExample>), Error> {
        let mut summary = String::new();
        let mut description = String::new();
        let mut tags = HashMap::new();
        let mut examples = Vec::new();

        // Extract summary (first line)
        if let Some(first_line) = content.lines().next() {
            summary = first_line.trim().to_string();
        }

        // Extract description (everything before first @tag)
        let desc_end = content.find('@').unwrap_or(content.len());
        description = content[..desc_end].trim().to_string();

        // Parse tags
        for (tag_name, pattern) in &self.tag_patterns {
            for captures in pattern.captures_iter(content) {
                let tag_value = captures.get(1).map_or("", |m| m.as_str()).to_string();
                tags.entry(tag_name.clone()).or_insert_with(Vec::new).push(tag_value);
            }
        }

        // Extract examples
        if let Some(example_pattern) = self.tag_patterns.get("example") {
            for captures in example_pattern.captures_iter(content) {
                let code = captures.get(1).map_or("", |m| m.as_str()).to_string();
                examples.push(CodeExample {
                    title: None,
                    description: None,
                    code,
                    language: "cursed".to_string(),
                    output: None,
                    is_runnable: true,
                });
            }
        }

        Ok((summary, description, tags, examples))
    }
}

/// Exported documentation structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportedDocumentation {
    /// All module documentation
    pub modules: Vec<ModuleDocumentation>,
    /// Cross-reference mapping
    pub cross_references: HashMap<String, Vec<CrossReference>>,
    /// Symbol table
    pub symbol_table: HashMap<String, DocumentedSymbol>,
    /// Export metadata
    pub metadata: ExportMetadata,
}

/// Export metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportMetadata {
    /// Generator version
    pub generator_version: String,
    /// Generation timestamp
    pub generated_at: chrono::DateTime<chrono::Utc>,
    /// Total modules processed
    pub total_modules: usize,
    /// Total items documented
    pub total_items: usize,
}

impl Default for DocumentationExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_documentation_extractor_creation() {
        let extractor = DocumentationExtractor::new();
        assert_eq!(extractor.module_cache.len(), 0);
        assert_eq!(extractor.cross_references.len(), 0);
        assert_eq!(extractor.symbol_table.len(), 0);
    }

    #[test]
    fn test_extraction_config_default() {
        let config = ExtractionConfig::default();
        assert!(!config.include_private);
        assert!(config.extract_examples);
        assert!(config.generate_cross_refs);
        assert_eq!(config.max_depth, 10);
        assert!(config.include_source);
    }

    #[test]
    fn test_comment_parser_creation() {
        let parser = CommentParser::new();
        assert!(parser.tag_patterns.contains_key("param"));
        assert!(parser.tag_patterns.contains_key("return"));
        assert!(parser.tag_patterns.contains_key("example"));
    }

    #[test]
    fn test_doc_element_creation() {
        let location = SourceLocation { line: 1, column: 1, file: None };
        let element = DocElement {
            name: "test_function".to_string(),
            element_type: ElementType::Function,
            visibility: Visibility::Public,
            module: "test_module".to_string(),
            summary: "Test function".to_string(),
            description: Some("A test function for documentation".to_string()),
            signature: Some("slay test_function() -> void".to_string()),
            parameters: Vec::new(),
            return_type: Some("void".to_string()),
            type_info: None,
            examples: Vec::new(),
            tags: HashMap::new(),
            location,
            source_code: None,
            metadata: ElementMetadata::default(),
        };

        assert_eq!(element.name, "test_function");
        assert_eq!(element.element_type, ElementType::Function);
        assert_eq!(element.visibility, Visibility::Public);
    }

    #[test]
    fn test_element_type_display() {
        assert_eq!(ElementType::Function.to_string(), "function");
        assert_eq!(ElementType::Struct.to_string(), "struct");
        assert_eq!(ElementType::Interface.to_string(), "interface");
        assert_eq!(ElementType::Variable.to_string(), "variable");
        assert_eq!(ElementType::Constant.to_string(), "constant");
    }

    #[test]
    fn test_documentation_metadata_creation() {
        let metadata = DocumentationMetadata::new("test_module");
        assert_eq!(metadata.module, "test_module");
        assert_eq!(metadata.format_version, "1.0");
    }

    #[test]
    fn test_cross_reference_creation() {
        let location = SourceLocation { line: 1, column: 1, file: None };
        let cross_ref = CrossReference {
            target: "test_function".to_string(),
            context: "function call".to_string(),
            location,
            reference_type: ReferenceType::Usage,
        };

        assert_eq!(cross_ref.target, "test_function");
        assert_eq!(cross_ref.context, "function call");
    }

    #[test]
    fn test_parameter_doc_creation() {
        let param_doc = ParameterDoc {
            name: "param1".to_string(),
            param_type: Some("i32".to_string()),
            description: "First parameter".to_string(),
            default_value: None,
            is_optional: false,
        };

        assert_eq!(param_doc.name, "param1");
        assert_eq!(param_doc.param_type, Some("i32".to_string()));
        assert!(!param_doc.is_optional);
    }

    #[test]
    fn test_code_example_creation() {
        let example = CodeExample {
            title: Some("Basic Usage".to_string()),
            description: Some("Shows how to use the function".to_string()),
            code: "test_function()".to_string(),
            language: "cursed".to_string(),
            output: Some("Success".to_string()),
            is_runnable: true,
        };

        assert_eq!(example.title, Some("Basic Usage".to_string()));
        assert_eq!(example.language, "cursed");
        assert!(example.is_runnable);
    }

    #[test]
    fn test_stability_level_default() {
        let metadata = ElementMetadata::default();
        matches!(metadata.stability, StabilityLevel::Stable);
    }

    #[test]
    fn test_language_metadata_default() {
        let lang_meta = LanguageMetadata::default();
        assert_eq!(lang_meta.language_version, "0.1.0");
        assert_eq!(lang_meta.target_platform, std::env::consts::OS);
    }
}
