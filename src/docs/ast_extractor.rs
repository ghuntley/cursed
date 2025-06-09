//! AST documentation extractor for CURSED source files
//!
//! Extracts documentation from AST nodes including function declarations,
//! struct definitions, interface definitions, and handles package-level documentation.

use crate::ast::*;
use crate::ast::statements::{FactsStatement, LetStatement};
use crate::docs::{DocError, DocResult, DocComment, CommentParser};
use crate::lexer::{Lexer, Token, TokenType};
use crate::parser::Parser;
use std::collections::HashMap;
use tracing::{debug, instrument, warn};

/// Helper trait for joining parameter lists  
trait VecParameterJoinExt {
    fn join(&self, separator: &str) -> String;
}

impl VecParameterJoinExt for Vec<Parameter> {
    fn join(&self, separator: &str) -> String {
        self.iter()
            .map(|p| format!("{} {}", p.name.string(), p.param_type.string()))
            .collect::<Vec<_>>()
            .join(separator)
    }
}

/// Type of documentation item
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ItemType {
    Function,
    Squad,      // struct
    Collab,     // interface
    Variable,
    Constant,
    TypeAlias,
    Module,
    Package,
}

impl std::fmt::Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemType::Function => write!(f, "function"),
            ItemType::Squad => write!(f, "squad"),
            ItemType::Collab => write!(f, "collab"),
            ItemType::Variable => write!(f, "variable"),
            ItemType::Constant => write!(f, "constant"),
            ItemType::TypeAlias => write!(f, "type"),
            ItemType::Module => write!(f, "module"),
            ItemType::Package => write!(f, "package"),
        }
    }
}

/// Extracted documentation item
#[derive(Debug, Clone)]
pub struct DocumentationItem {
    /// Item name
    pub name: String,
    /// Item type
    pub item_type: ItemType,
    /// Associated documentation comment
    pub doc_comment: Option<DocComment>,
    /// Function signature (for functions)
    pub signature: Option<String>,
    /// Type information
    pub type_info: Option<String>,
    /// Source location
    pub line: usize,
    /// Visibility (public/private)
    pub visibility: String,
    /// Generic parameters
    pub generics: Vec<String>,
    /// Function parameters (for functions)
    pub parameters: Vec<ParameterInfo>,
    /// Return type (for functions)
    pub return_type: Option<String>,
    /// Fields (for structs/interfaces)
    pub fields: Vec<FieldInfo>,
    /// Methods (for structs/interfaces)
    pub methods: Vec<DocumentationItem>,
    /// Examples from documentation
    pub examples: Vec<String>,
}

/// Parameter information
#[derive(Debug, Clone)]
pub struct ParameterInfo {
    pub name: String,
    pub param_type: String,
    pub description: Option<String>,
}

/// Field information
#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub name: String,
    pub field_type: String,
    pub description: Option<String>,
    pub visibility: String,
}

impl DocumentationItem {
    /// Create a new documentation item
    pub fn new(name: String, item_type: ItemType, line: usize) -> Self {
        Self {
            name,
            item_type,
            doc_comment: None,
            signature: None,
            type_info: None,
            line,
            visibility: "public".to_string(),
            generics: Vec::new(),
            parameters: Vec::new(),
            return_type: None,
            fields: Vec::new(),
            methods: Vec::new(),
            examples: Vec::new(),
        }
    }

    /// Set documentation comment
    pub fn with_doc_comment(mut self, doc_comment: DocComment) -> Self {
        // Extract examples from doc comment
        for tag in &doc_comment.tags {
            if let crate::docs::DocTag::Example { code, .. } = tag {
                self.examples.push(code.clone());
            }
        }
        
        self.doc_comment = Some(doc_comment);
        self
    }

    /// Set signature
    pub fn with_signature(mut self, signature: String) -> Self {
        self.signature = Some(signature);
        self
    }

    /// Set visibility
    pub fn with_visibility(mut self, visibility: String) -> Self {
        self.visibility = visibility;
        self
    }

    /// Add generic parameter
    pub fn add_generic(mut self, generic: String) -> Self {
        self.generics.push(generic);
        self
    }

    /// Add parameter
    pub fn add_parameter(mut self, param: ParameterInfo) -> Self {
        self.parameters.push(param);
        self
    }

    /// Set return type
    pub fn with_return_type(mut self, return_type: String) -> Self {
        self.return_type = Some(return_type);
        self
    }

    /// Add field
    pub fn add_field(mut self, field: FieldInfo) -> Self {
        self.fields.push(field);
        self
    }

    /// Add method
    pub fn add_method(mut self, method: DocumentationItem) -> Self {
        self.methods.push(method);
        self
    }

    /// Get main description from doc comment
    pub fn description(&self) -> Option<&str> {
        self.doc_comment.as_ref().map(|doc| doc.description.as_str())
    }

    /// Check if this item is deprecated
    pub fn is_deprecated(&self) -> bool {
        self.doc_comment.as_ref().map_or(false, |doc| doc.is_deprecated())
    }

    /// Get parameter descriptions from doc comment
    pub fn parameter_descriptions(&self) -> HashMap<String, String> {
        let mut descriptions = HashMap::new();
        
        if let Some(doc) = &self.doc_comment {
            for tag in doc.get_params() {
                if let crate::docs::DocTag::Param { name, description } = tag {
                    descriptions.insert(name.clone(), description.clone());
                }
            }
        }
        
        descriptions
    }

    /// Get return description from doc comment
    pub fn return_description(&self) -> Option<String> {
        self.doc_comment.as_ref().and_then(|doc| {
            doc.get_return().and_then(|tag| {
                if let crate::docs::DocTag::Return { description } = tag {
                    Some(description.clone())
                } else {
                    None
                }
            })
        })
    }
}

/// AST documentation extractor
pub struct AstExtractor {
    /// Current module name
    current_module: Option<String>,
    /// Extracted items
    items: Vec<DocumentationItem>,
    /// Comment parser for extracting documentation
    comment_parser: CommentParser,
    /// Source file path for line number tracking
    source_path: Option<String>,
}

impl AstExtractor {
    /// Create a new AST extractor
    pub fn new() -> Self {
        Self {
            current_module: None,
            items: Vec::new(),
            comment_parser: CommentParser::new().expect("Failed to create comment parser"),
            source_path: None,
        }
    }

    /// Create a new AST extractor with source file path
    pub fn with_source_path(source_path: String) -> Self {
        Self {
            current_module: None,
            items: Vec::new(),
            comment_parser: CommentParser::new().expect("Failed to create comment parser"),
            source_path: Some(source_path),
        }
    }

    /// Extract documentation from CURSED source file
    #[instrument(skip(self, source))]
    pub fn extract_from_source(&mut self, source: &str, file_path: Option<String>) -> DocResult<Vec<DocumentationItem>> {
        debug!("Extracting documentation from source file");
        
        self.source_path = file_path;
        self.items.clear();

        // Parse source to get both AST and comments
        let mut lexer = Lexer::new(source);
        
        // Extract comments from source directly
        let comments = self.comment_parser.parse_comments(source)?;
        debug!("Found {} documentation comments", comments.len());

        // Parse AST
        let mut parser = Parser::new(&mut lexer).map_err(|e| {
            DocError::ParseError(format!("Failed to create parser: {}", e))
        })?;
        let program = parser.parse_program().map_err(|e| {
            DocError::ParseError(format!("Failed to parse program: {}", e))
        })?;

        // Extract documentation items from AST
        for statement in &program.statements {
            self.extract_from_statement(statement)?;
        }

        // Associate comments with extracted items
        self.associate_comments(&comments)?;

        debug!("Extracted {} documentation items", self.items.len());
        Ok(self.items.clone())
    }

    /// Extract documentation from a program AST
    #[instrument(skip(self, program))]
    pub fn extract_from_program(&mut self, program: &Program) -> DocResult<Vec<DocumentationItem>> {
        debug!("Extracting documentation from program AST");
        
        self.items.clear();
        
        for statement in &program.statements {
            self.extract_from_statement(statement)?;
        }

        debug!("Extracted {} documentation items", self.items.len());
        Ok(self.items.clone())
    }

    /// Extract comments from source - now handled by parse_comments method

    /// Extract documentation from a statement
    fn extract_from_statement(&mut self, statement: &Box<dyn Statement>) -> DocResult<()> {
        // Try to downcast to specific statement types
        if let Some(func) = statement.as_any().downcast_ref::<FunctionStatement>() {
            self.extract_function_doc(func)?;
        } else if let Some(squad) = statement.as_any().downcast_ref::<SquadStatement>() {
            self.extract_squad_doc(squad)?;
        } else if let Some(collab) = statement.as_any().downcast_ref::<CollabStatement>() {
            self.extract_collab_doc(collab)?;
        }
        // Add more CURSED-specific statement types
        else if let Some(facts_stmt) = statement.as_any().downcast_ref::<FactsStatement>() {
            self.extract_constant_doc(facts_stmt)?;
        } else if let Some(sus_stmt) = statement.as_any().downcast_ref::<LetStatement>() {
            self.extract_variable_doc(sus_stmt)?;
        }
        
        Ok(())
    }

    /// Extract documentation from function statement
    fn extract_function_doc(&mut self, func: &FunctionStatement) -> DocResult<()> {
        let mut item = DocumentationItem::new(
            func.name.string(),
            ItemType::Function,
            0, // Line number would come from token position in real implementation
        );

        // Generate signature
        let signature = self.generate_function_signature(func);
        item = item.with_signature(signature);

        // Extract generic parameters
        for type_param in &func.type_parameters {
            item = item.add_generic(type_param.string());
        }

        // Extract parameters
        for param in &func.parameters {
            let param_info = ParameterInfo {
                name: param.name.string(),
                param_type: param.param_type.string(),
                description: None, // Will be filled from doc comment
            };
            item = item.add_parameter(param_info);
        }

        // Extract return type
        if let Some(return_type) = &func.return_type {
            item = item.with_return_type(return_type.string());
        }

        self.items.push(item);
        Ok(())
    }

    /// Extract documentation from squad (struct) statement
    fn extract_squad_doc(&mut self, squad: &SquadStatement) -> DocResult<()> {
        let mut item = DocumentationItem::new(
            squad.name.string(),
            ItemType::Squad,
            0,
        );

        // Extract generic parameters
        for type_param in &squad.type_parameters {
            item = item.add_generic(type_param.string());
        }

        // Extract fields
        for field in &squad.fields {
            let field_info = FieldInfo {
                name: field.name.string(),
                field_type: field.type_name.string(),
                description: None,
                visibility: "public".to_string(), // Default visibility
            };
            item = item.add_field(field_info);
        }

        self.items.push(item);
        Ok(())
    }

    /// Extract documentation from collab (interface) statement
    fn extract_collab_doc(&mut self, collab: &CollabStatement) -> DocResult<()> {
        let mut item = DocumentationItem::new(
            collab.name.string(),
            ItemType::Collab,
            0,
        );

        // Extract generic parameters
        for type_param in &collab.type_parameters {
            item = item.add_generic(type_param.string());
        }

        // Extract method signatures
        for method in &collab.methods {
            let method_item = DocumentationItem::new(
                method.name.string(),
                ItemType::Function,
                0,
            ).with_signature(format!(
                "{}({}): {}",
                method.name.string(),
                method.parameters.join(", "),
                method.return_type.as_ref().map(|rt| rt.string()).unwrap_or_else(|| "void".to_string())
            ));
            
            item = item.add_method(method_item);
        }

        self.items.push(item);
        Ok(())
    }

    /// Extract documentation from constant (facts) statement
    fn extract_constant_doc(&mut self, constant: &FactsStatement) -> DocResult<()> {
        let item = DocumentationItem::new(
            constant.name.string(), // Extract actual name from statement
            ItemType::Constant,
            0,
        ).with_signature(format!("facts {} = {}", constant.name.string(), constant.value.string()));
        
        self.items.push(item);
        Ok(())
    }

    /// Extract documentation from variable (sus) statement  
    fn extract_variable_doc(&mut self, variable: &LetStatement) -> DocResult<()> {
        let item = DocumentationItem::new(
            variable.name.string(), // Extract actual name from statement
            ItemType::Variable,
            0,
        ).with_signature(format!("sus {} = {}", variable.name.string(), 
            variable.value.as_ref().map(|v| v.string()).unwrap_or_else(|| "undefined".to_string())));
        
        self.items.push(item);
        Ok(())
    }

    /// Generate function signature string
    fn generate_function_signature(&self, func: &FunctionStatement) -> String {
        let mut signature = String::new();
        
        // Add function keyword (slay in CURSED)
        signature.push_str("slay ");
        
        // Add name
        signature.push_str(&func.name.string());
        
        // Add type parameters if any
        if !func.type_parameters.is_empty() {
            signature.push('[');
            let type_params: Vec<String> = func.type_parameters
                .iter()
                .map(|tp| tp.string())
                .collect();
            signature.push_str(&type_params.join(", "));
            signature.push(']');
        }
        
        // Add parameters
        signature.push('(');
        let params: Vec<String> = func.parameters
            .iter()
            .map(|p| format!("{} {}", p.name.string(), p.param_type.string()))
            .collect();
        signature.push_str(&params.join(", "));
        signature.push(')');
        
        // Add return type if any
        if let Some(return_type) = &func.return_type {
            signature.push_str(" -> ");
            signature.push_str(&return_type.string());
        }
        
        signature
    }

    /// Associate documentation comments with extracted items
    pub fn associate_comments(&mut self, comments: &[DocComment]) -> DocResult<()> {
        let items_to_update: Vec<(usize, DocComment)> = self.items
            .iter()
            .enumerate()
            .filter_map(|(idx, item)| {
                Self::find_preceding_comment_static(comments, item.line)
                    .map(|comment| (idx, comment.clone()))
            })
            .collect();
            
        for (idx, comment) in items_to_update {
            let item = &mut self.items[idx];
            
            // Update parameter descriptions
            let param_descriptions = item.parameter_descriptions();
            for param in &mut item.parameters {
                if let Some(desc) = param_descriptions.get(&param.name) {
                    param.description = Some(desc.clone());
                }
            }
            
            item.doc_comment = Some(comment);
        }
        
        Ok(())
    }

    /// Find documentation comment preceding a given line (static version)
    fn find_preceding_comment_static(comments: &[DocComment], target_line: usize) -> Option<&DocComment> {
        comments
            .iter()
            .filter(|comment| comment.line < target_line)
            .max_by_key(|comment| comment.line)
    }

    /// Set current module name
    pub fn set_module(&mut self, module_name: String) {
        self.current_module = Some(module_name);
    }

    /// Get extracted items
    pub fn items(&self) -> &[DocumentationItem] {
        &self.items
    }

    /// Clear extracted items
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Extract package-level documentation
    pub fn extract_package_doc(&mut self, package_comment: &str) -> DocResult<()> {
        if let Ok(comments) = self.comment_parser.parse_comments(package_comment) {
            if let Some(doc_comment) = comments.first() {
                let item = DocumentationItem::new(
                    self.current_module.clone().unwrap_or_else(|| "package".to_string()),
                    ItemType::Package,
                    1,
                ).with_doc_comment(doc_comment.clone());
                
                self.items.push(item);
            }
        }
        Ok(())
    }

    /// Get documentation for a specific item by name
    pub fn get_item_doc(&self, name: &str) -> Option<&DocumentationItem> {
        self.items.iter().find(|item| item.name == name)
    }

    /// Get all items of a specific type
    pub fn get_items_by_type(&self, item_type: ItemType) -> Vec<&DocumentationItem> {
        self.items.iter().filter(|item| item.item_type == item_type).collect()
    }

    /// Get summary statistics
    pub fn get_stats(&self) -> HashMap<ItemType, usize> {
        let mut stats = HashMap::new();
        for item in &self.items {
            *stats.entry(item.item_type.clone()).or_insert(0) += 1;
        }
        stats
    }
}

impl Default for AstExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_extractor_creation() {
        let extractor = AstExtractor::new();
        assert_eq!(extractor.items.len(), 0);
    }

    #[test] 
    fn test_item_creation() {
        let item = DocumentationItem::new(
            "test_function".to_string(),
            ItemType::Function,
            42
        );
        assert_eq!(item.name, "test_function");
        assert_eq!(item.item_type, ItemType::Function);
        assert_eq!(item.line, 42);
    }
}
