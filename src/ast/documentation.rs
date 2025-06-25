// AST Documentation Module
//
// Comprehensive documentation extraction and analysis for CURSED AST nodes.
// This module provides the core infrastructure for extracting documentation
// from AST nodes and building rich documentation metadata.

use crate::ast::traits::*;
use crate::ast::*;
use crate::error::{CursedError, SourceLocation};
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
    /// Cache of extracted documentation by module
    /// Cross-reference mapping between documented elements
    /// Symbol table for resolving references
    /// Comment parser for extracting structured documentation
/// Configuration for documentation extraction
#[derive(Debug, Clone)]
pub struct ExtractionConfig {
    /// Include private items in documentation
    /// Extract inline code examples
    /// Generate cross-references
    /// Maximum depth for recursive type analysis
    /// Include source code snippets
    /// Language-specific settings
/// Language-specific documentation settings
#[derive(Debug, Clone)]
pub struct LanguageSettings {
    /// CURSED-specific keyword mappings
    /// Gen Z slang documentation style
    /// Include type signatures
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
            language_settings: LanguageSettings {
        }
    }
impl DocumentationExtractor {
    /// Create a new documentation extractor with default configuration
    pub fn new() -> Self {
        Self::with_config(ExtractionConfig::default())
    /// Create a new documentation extractor with custom configuration
    pub fn with_config(config: ExtractionConfig) -> Self {
        Self {
        }
    }

    /// Extract documentation from a complete program
    pub fn extract_program_documentation(&mut self, program: &Program, file_path: &Path) -> crate::error::Result<()> {
        let module_name = self.derive_module_name(file_path);
        
        // Extract package-level documentation
        let package_info = PackageInfo {

        // Extract import documentation
        let imports = self.extract_import_docs(&program.imports)?;
        
        // Extract documentation from statements
        let mut items = Vec::new();
        let mut module_comments = Vec::new();
        
        for statement in &program.statements {
            match self.extract_statement_documentation(statement, &module_name) {
                Ok(None) => {}, // Not a documentable statement
            }
        }

        // Build module documentation
        let module_doc = ModuleDocumentation {

        // Cache the module documentation
        self.module_cache.insert(module_name, module_doc.clone());
        
        // Update symbol table
        self.update_symbol_table(&module_doc);

        Ok(module_doc)
    /// Extract documentation from individual statements
    pub fn extract_statement_documentation(&mut self, statement: &dyn Statement, module: &str) -> crate::error::Result<()> {
        use crate::ast::declarations::{FunctionStatement, SquadStatement, CollabStatement};
        use crate::ast::VariableStatement;
        
        // Try to downcast to specific statement types for detailed extraction
        let any_stmt = statement.as_any();
        
        // Function declarations
        if let Some(func_stmt) = any_stmt.downcast_ref::<FunctionStatement>() {
            return Ok(Some(self.extract_function_documentation(func_stmt, module)?));
        // Struct declarations
        if let Some(struct_stmt) = any_stmt.downcast_ref::<SquadStatement>() {
            return Ok(Some(self.extract_struct_documentation(struct_stmt, module)?));
        // Interface declarations  
        if let Some(interface_stmt) = any_stmt.downcast_ref::<CollabStatement>() {
            return Ok(Some(self.extract_interface_documentation(interface_stmt, module)?));
        // Variable declarations
        if let Some(var_stmt) = any_stmt.downcast_ref::<VariableStatement>() {
            return Ok(Some(self.extract_variable_documentation(var_stmt, module)?));
        // For other statement types, return None (not documentable)
        Ok(None)
    /// Extract function documentation
    fn extract_function_documentation(&mut self, func: &FunctionStatement, module: &str) -> crate::error::Result<()> {
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
            summary: if summary.is_empty() { 
                format!("Function {}", func_name) 
            } else { 
                summary 
            description: if description.is_empty() { 
                Some(format!("CURSED function declaration using the 'slay' keyword"))
            } else { 
                Some(description) 
        })
    /// Extract struct documentation
    fn extract_struct_documentation(&mut self, struct_stmt: &SquadStatement, module: &str) -> crate::error::Result<()> {
        let location = SourceLocation { line: 1, column: 1, file: None };
        
        let struct_name = &struct_stmt.name.value;
        let signature = self.build_struct_signature(struct_stmt);
        
        // Extract field documentation
        let field_docs = self.extract_field_docs(&struct_stmt.fields)?;
        
        // Parse documentation comments
        let (summary, description, tags, examples) = self.comment_parser.parse_documentation_for_location(&location)?;
        
        // Build type information
        let type_info = Some(TypeInfo {
        });

        Ok(DocElement {
            visibility: Visibility::Public, // Structs are typically public in CURSED
            summary: if summary.is_empty() { 
                format!("Struct {}", struct_name) 
            } else { 
                summary 
            description: if description.is_empty() { 
                Some(format!("CURSED struct declaration using the 'squad' keyword"))
            } else { 
                Some(description) 
        })
    /// Extract interface documentation
    fn extract_interface_documentation(&mut self, interface: &CollabStatement, module: &str) -> crate::error::Result<()> {
        let location = SourceLocation { line: 1, column: 1, file: None };
        
        let interface_name = &interface.name.value;
        let signature = self.build_interface_signature(interface);
        
        // Extract method documentation
        let method_docs = self.extract_method_docs(&interface.methods)?;
        
        // Parse documentation comments
        let (summary, description, tags, examples) = self.comment_parser.parse_documentation_for_location(&location)?;
        
        // Build type information
        let type_info = Some(TypeInfo {
        });

        Ok(DocElement {
            visibility: Visibility::Public, // Interfaces are typically public
            summary: if summary.is_empty() { 
                format!("Interface {}", interface_name) 
            } else { 
                summary 
            description: if description.is_empty() { 
                Some(format!("CURSED interface declaration using the 'collab' keyword"))
            } else { 
                Some(description) 
        })
    /// Extract variable documentation
    fn extract_variable_documentation(&mut self, var: &VariableStatement, module: &str) -> crate::error::Result<()> {
        let location = SourceLocation { line: 1, column: 1, file: None };
        
        let var_name = &var.name;
        let keyword = if var.is_mutable { "sus" } else { "facts" };
        let signature = format!("{} {}", keyword, var_name);
        
        // Determine element type based on mutability
        let element_type = if var.is_mutable {
            ElementType::Variable
        } else {
            ElementType::Constant
        
        // Parse documentation comments
        let (summary, description, tags, examples) = self.comment_parser.parse_documentation_for_location(&location)?;

        Ok(DocElement {
            visibility: Visibility::Private, // Variables are typically private unless exported
            summary: if summary.is_empty() { 
                format!("{} {}", if var.is_mutable { "Variable" } else { "Constant" }, var_name) 
            } else { 
                summary 
            description: if description.is_empty() { 
                           if var.is_mutable { "variable" } else { "constant" }, keyword))
            } else { 
                Some(description) 
        })
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
        // Add parameters
        sig.push('(');
        let param_strs: Vec<String> = func.parameters.iter().map(|p| p.string()).collect();
        sig.push_str(&param_strs.join(", "));
        sig.push(')');
        
        // Add return type if present
        if let Some(return_type) = &func.return_type {
            sig.push_str(" -> ");
            sig.push_str(&return_type.string());
        sig
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
        sig
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
        sig
    /// Extract parameter documentation
    fn extract_parameter_docs(&self, parameters: &[crate::ast::expressions::Parameter]) -> crate::error::Result<()> {
        use crate::ast::traits::Node;
        
        let mut param_docs = Vec::new();
        
        for param in parameters {
            let param_doc = ParameterDoc {
                description: format!("Parameter {}", param.name), // Would be extracted from comments
            param_docs.push(param_doc);
        Ok(param_docs)
    /// Extract field documentation
    fn extract_field_docs(&self, fields: &[crate::ast::declarations::FieldStatement]) -> crate::error::Result<()> {
        let mut field_docs = Vec::new();
        
        for field in fields {
            let field_doc = FieldDoc {
                description: format!("Field {}", field.name.value), // Would be extracted from comments
                is_public: true, // Fields in CURSED structs are typically public
                default_value: None, // Could be extracted if present
            field_docs.push(field_doc);
        Ok(field_docs)
    /// Extract method documentation
    fn extract_method_docs(&self, methods: &[crate::ast::declarations::MethodDeclaration]) -> crate::error::Result<()> {
        use crate::ast::traits::Node;
        
        let mut method_docs = Vec::new();
        
        for method in methods {
            let method_doc = MethodDoc {
                description: format!("Method {}", method.name.value), // Would be extracted from comments
                is_static: false, // Would need to be determined from context
            method_docs.push(method_doc);
        Ok(method_docs)
    /// Extract import documentation
    fn extract_import_docs(&self, imports: &[ImportStatement]) -> crate::error::Result<()> {
        let mut import_docs = Vec::new();
        
        for import in imports {
            let import_doc = ImportDoc {
                is_public: false, // Imports are typically not re-exported
            import_docs.push(import_doc);
        Ok(import_docs)
    /// Build cross-reference map
    pub fn build_cross_references(&mut self) -> crate::error::Result<()> {
        if !self.config.generate_cross_refs {
            return Ok(());
        let mut references = HashMap::new();
        
        for (module_name, module_doc) in &self.module_cache {
            for item in &module_doc.items {
                let item_refs = self.find_references_for_item(item)?;
                if !item_refs.is_empty() {
                    references.insert(format!("{}::{}", module_name, item.name), item_refs);
                }
            }
        self.cross_references = references;
        Ok(())
    /// Find cross-references for a documentation item
    fn find_references_for_item(&self, item: &DocElement) -> crate::error::Result<()> {
        let mut refs = Vec::new();
        
        // Search in signature
        if let Some(signature) = &item.signature {
            refs.extend(self.find_references_in_text(signature, &item.location)?);
        // Search in description
        if let Some(description) = &item.description {
            refs.extend(self.find_references_in_text(description, &item.location)?);
        // Search in examples
        for example in &item.examples {
            refs.extend(self.find_references_in_text(&example.code, &item.location)?);
        Ok(refs)
    /// Find references in text content
    fn find_references_in_text(&self, text: &str, location: &SourceLocation) -> crate::error::Result<()> {
        let mut refs = Vec::new();
        
        // Simple implementation - look for known symbols
        for symbol_name in self.symbol_table.keys() {
            if text.contains(symbol_name) {
                refs.push(CrossReference {
                });
            }
        }
        
        Ok(refs)
    /// Update symbol table with module documentation
    fn update_symbol_table(&mut self, module_doc: &ModuleDocumentation) {
        for item in &module_doc.items {
            let symbol = DocumentedSymbol {
            
            let fully_qualified_name = format!("{}::{}", module_doc.name, item.name);
            self.symbol_table.insert(fully_qualified_name, symbol);
        }
    }

    /// Infer expression type for variables (simplified)
    fn infer_expression_type(&self, _expr: &dyn Expression) -> String {
        // Simplified type inference - would need full type system integration
        "unknown".to_string()
    /// Derive module name from file path
    fn derive_module_name(&self, file_path: &Path) -> String {
        file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string()
    /// Gather source file information
    fn gather_source_info(&self, file_path: &Path) -> crate::error::Result<()> {
        use std::fs;
        
        let metadata = fs::metadata(file_path).map_err(CursedError::Io)?;
        let file_size = metadata.len();
        
        let source = fs::read_to_string(file_path).map_err(CursedError::Io)?;
        let line_count = source.split("\n").count();
        
        let last_modified = metadata.modified().ok();
        
        Ok(SourceInfo {
        })
    /// Get extracted documentation for a module
    pub fn get_module_documentation(&self, module_name: &str) -> Option<&ModuleDocumentation> {
        self.module_cache.get(module_name)
    /// Get all extracted modules
    pub fn get_all_modules(&self) -> Vec<&ModuleDocumentation> {
        self.module_cache.values().collect()
    /// Get cross-references for an item
    pub fn get_cross_references(&self, item_name: &str) -> Option<&Vec<CrossReference>> {
        self.cross_references.get(item_name)
    /// Get symbol by fully qualified name
    pub fn get_symbol(&self, name: &str) -> Option<&DocumentedSymbol> {
        self.symbol_table.get(name)
    /// Export documentation as structured data
    pub fn export_documentation(&self) -> ExportedDocumentation {
        ExportedDocumentation {
            metadata: ExportMetadata {
        }
    }
/// Represents different types of documentable elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocElement {
    /// Name of the documented element
    /// Type of element (function, struct, etc.)
    /// Visibility level
    /// Module containing this element
    /// Brief summary/description
    /// Detailed description
    /// Type signature or declaration
    /// Parameter documentation
    /// Return type information
    /// Type information for complex types
    /// Code examples
    /// Documentation tags (author, since, deprecated, etc.)
    /// Source location
    /// Source code snippet
    /// Additional metadata
/// Types of documentable elements
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ElementType {
impl fmt::Display for ElementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        }
    }
/// Item visibility levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Visibility {
/// Rich metadata about documented items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationMetadata {
    /// Module this documentation belongs to
    /// Timestamp when extracted
    /// Version of documentation format
    /// Language-specific metadata
    /// Statistics about the documentation
impl DocumentationMetadata {
    pub fn new(module: &str) -> Self {
        Self {
        }
    }
/// Language-specific metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageMetadata {
    /// CURSED language version
    /// Compiler version used
    /// Target platform
    /// Feature flags enabled
impl Default for LanguageMetadata {
    fn default() -> Self {
        Self {
        }
    }
/// Documentation statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentationStatistics {
    /// Total number of documented items
    /// Number of functions
    /// Number of structs
    /// Number of interfaces
    /// Number of variables
    /// Number of constants
    /// Coverage percentage
/// Cross-reference between documented elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossReference {
    /// Target of the reference
    /// Context where reference appears
    /// Location of the reference
    /// Type of reference
/// Types of cross-references
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReferenceType {
    /// Direct usage/call
    /// Type annotation
    /// Import statement
    /// Inheritance/implementation
    /// Documentation mention
/// Parameter documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDoc {
    /// Parameter name
    /// Parameter type
    /// Description of parameter
    /// Default value if any
    /// Whether parameter is optional
/// Field documentation for structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDoc {
    /// Field name
    /// Field type
    /// Field description
    /// Whether field is public
    /// Default value if any
/// Method documentation for interfaces
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodDoc {
    /// Method name
    /// Method signature
    /// Method description
    /// Parameter documentation
    /// Return type
    /// Whether method is static
/// Import documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportDoc {
    /// Import path
    /// Import alias
    /// Import description
    /// Whether import is re-exported
/// Type information for complex types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeInfo {
    /// Base type name
    /// Generic parameters
    /// Type constraints
    /// Fields for struct types
    /// Methods for interface types
/// Code example with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExample {
    /// Example title
    /// Example description
    /// Code content
    /// Programming language
    /// Expected output
    /// Whether example can be executed
/// Additional metadata for elements
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ElementMetadata {
    /// When element was first added
    /// If element is deprecated
    /// Deprecation message
    /// Stability level
    /// Performance characteristics
    /// Security considerations
impl ElementMetadata {
    pub fn from_function(func: &FunctionStatement) -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn from_struct(struct_stmt: &SquadStatement) -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn from_interface(interface: &CollabStatement) -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn from_variable(var: &VariableStatement) -> Self {
        Self {
            ..Default::default()
        }
    }
/// Stability levels for API elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StabilityLevel {
impl Default for StabilityLevel {
    fn default() -> Self {
        StabilityLevel::Stable
    }
}

/// Module-level documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDocumentation {
    /// Module name
    /// Source file path
    /// Package information
    /// Import documentation
    /// Documented items in module
    /// Module-level comments
    /// Module metadata
    /// Source file information
/// Package information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    /// Package name
    /// Package description
    /// Package version
    /// Package authors
    /// Package dependencies
/// Source file information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceInfo {
    /// File size in bytes
    /// Number of lines
    /// Last modification time
    /// File encoding
/// Documented symbol for cross-referencing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentedSymbol {
    /// Symbol name
    /// Element type
    /// Module containing symbol
    /// Source location
    /// Symbol signature
/// Comment parser for extracting structured documentation
#[derive(Debug)]
pub struct CommentParser {
    /// Patterns for detecting documentation tags
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
        }
    }

    /// Parse documentation for a specific location
    pub fn parse_documentation_for_location(&self, _location: &SourceLocation) -> crate::error::Result<()> {
        // Simplified implementation - in practice would extract comments from token stream
        Ok((
            String::new(),      // summary
            String::new(),      // description
            HashMap::new(),     // tags
            Vec::new(),         // examples
        ))
    /// Parse documentation content
    pub fn parse_documentation_content(&self, content: &str) -> crate::error::Result<()> {
        let mut summary = String::new();
        let mut description = String::new();
        let mut tags = HashMap::new();
        let mut examples = Vec::new();

        // Extract summary (first line)
        if let Some(first_line) = content.split("\n").next() {
            summary = first_line.trim().to_string();
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
    /// Cross-reference mapping
    /// Symbol table
    /// Export metadata
/// Export metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportMetadata {
    /// Generator version
    /// Generation timestamp
    /// Total modules processed
    /// Total items documented
impl Default for DocumentationExtractor {
    fn default() -> Self {
        Self::new()
    }
}

