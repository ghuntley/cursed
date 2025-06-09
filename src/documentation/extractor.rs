//! Documentation extractor for CURSED AST nodes
//!
//! This module provides the DocumentationExtractor that walks through CURSED AST
//! and extracts documentation comments, parameter information, return types,
//! and other documentation-relevant metadata.

use crate::ast::{
    Node, Statement, Expression, Program,
    FunctionStatement, SquadStatement, CollabStatement, 
    Parameter, TypeParameter, Field
};
use regex;
use crate::error::SourceLocation;
use crate::documentation::{DocumentationError, DocumentationResult};
use std::collections::HashMap;
use tracing::{instrument, debug, warn};

/// Represents a documented item extracted from the AST
#[derive(Debug, Clone)]
pub struct DocumentationItem {
    /// Name of the documented item
    pub name: String,
    /// Type of the item (function, struct, interface, etc.)
    pub item_type: ItemType,
    /// Raw documentation text
    pub documentation: Option<String>,
    /// Source location for error reporting
    pub location: SourceLocation,
    /// Parameters for functions/methods
    pub parameters: Vec<ParameterInfo>,
    /// Return type for functions
    pub return_type: Option<String>,
    /// Fields for structs/interfaces
    pub fields: Vec<FieldInfo>,
    /// Type parameters for generic items
    pub type_parameters: Vec<String>,
    /// Cross-references found in documentation
    pub references: Vec<String>,
}

/// Type of documented item
#[derive(Debug, Clone, PartialEq)]
pub enum ItemType {
    Function,
    Struct,
    Interface,
    Method,
    Field,
    Parameter,
    Constant,
    Variable,
    Module,
}

/// Parameter information for documentation
#[derive(Debug, Clone)]
pub struct ParameterInfo {
    pub name: String,
    pub type_name: String,
    pub description: Option<String>,
}

/// Field information for documentation
#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub name: String,
    pub type_name: String,
    pub description: Option<String>,
}

/// Result of documentation extraction
#[derive(Debug, Clone)]
pub struct ExtractionResult {
    /// All extracted documentation items
    pub items: Vec<DocumentationItem>,
    /// Symbol table for cross-reference resolution
    pub symbols: HashMap<String, DocumentationItem>,
    /// Extraction statistics
    pub stats: ExtractionStats,
}

/// Statistics about the extraction process
#[derive(Debug, Clone)]
pub struct ExtractionStats {
    pub total_functions: usize,
    pub documented_functions: usize,
    pub total_types: usize,
    pub documented_types: usize,
    pub total_fields: usize,
    pub documented_fields: usize,
}

/// Documentation extractor that walks AST and extracts documentation
pub struct DocumentationExtractor {
    /// Current module path for namespacing
    module_path: Vec<String>,
    /// Symbol table being built
    symbols: HashMap<String, DocumentationItem>,
    /// Statistics tracking
    stats: ExtractionStats,
}

impl DocumentationExtractor {
    /// Create a new documentation extractor
    pub fn new() -> Self {
        Self {
            module_path: Vec::new(),
            symbols: HashMap::new(),
            stats: ExtractionStats {
                total_functions: 0,
                documented_functions: 0,
                total_types: 0,
                documented_types: 0,
                total_fields: 0,
                documented_fields: 0,
            },
        }
    }

    /// Extract documentation from a CURSED program
    #[instrument(skip(self, program))]
    pub fn extract_documentation(&mut self, program: &Program) -> DocumentationResult<ExtractionResult> {
        debug!("Starting documentation extraction");
        
        let mut items = Vec::new();
        
        // Extract from all statements in the program
        for statement in &program.statements {
            if let Some(item) = self.extract_from_statement(statement.as_ref())? {
                items.push(item);
            }
        }

        debug!(
            "Extraction complete: {} items, {} functions, {} types",
            items.len(),
            self.stats.total_functions,
            self.stats.total_types
        );

        Ok(ExtractionResult {
            items,
            symbols: self.symbols.clone(),
            stats: self.stats.clone(),
        })
    }

    /// Extract documentation from a statement
    #[instrument(skip(self, statement))]
    fn extract_from_statement(&mut self, statement: &dyn Statement) -> DocumentationResult<Option<DocumentationItem>> {
        // Try to cast to specific statement types
        let any_statement = statement.as_any();
        
        if let Some(function) = any_statement.downcast_ref::<FunctionStatement>() {
            return Ok(Some(self.extract_from_function_impl(function)?));
        }
        
        if let Some(squad) = any_statement.downcast_ref::<SquadStatement>() {
            return Ok(Some(self.extract_from_struct_impl(squad)?));
        }
        
        if let Some(collab) = any_statement.downcast_ref::<CollabStatement>() {
            return Ok(Some(self.extract_from_interface_impl(collab)?));
        }

        debug!("No documentation extraction for statement type: {}", statement.string());
        Ok(None)
    }

    /// Public wrapper for extract_from_function for testing
    pub fn extract_from_function(&mut self, function: &FunctionStatement) -> DocumentationResult<DocumentationItem> {
        self.extract_from_function_impl(function)
    }

    /// Public wrapper for extract_from_struct for testing
    pub fn extract_from_struct(&mut self, squad: &SquadStatement) -> DocumentationResult<DocumentationItem> {
        self.extract_from_struct_impl(squad)
    }

    /// Public wrapper for extract_from_interface for testing
    pub fn extract_from_interface(&mut self, collab: &CollabStatement) -> DocumentationResult<DocumentationItem> {
        self.extract_from_interface_impl(collab)
    }

    /// Extract documentation from a function declaration
    #[instrument(skip(self, function))]
    fn extract_from_function_impl(&mut self, function: &FunctionStatement) -> DocumentationResult<DocumentationItem> {
        self.stats.total_functions += 1;
        
        let location = SourceLocation::new(0, 0); // TODO: Extract actual location from AST
        let name = function.name.value.clone();
        
        // Extract parameter information
        let parameters = function.parameters.iter()
            .map(|param| self.extract_parameter_info(param))
            .collect::<Result<Vec<_>, _>>()?;

        // Extract return type
        let return_type = function.return_type.as_ref()
            .map(|rt| rt.string());

        // Extract type parameters
        let type_parameters = function.type_parameters.iter()
            .map(|tp| tp.name.clone())
            .collect();

        // Extract documentation from comments (TODO: implement comment extraction)
        let documentation = self.extract_doc_comments(&name, &location)?;
        
        if documentation.is_some() {
            self.stats.documented_functions += 1;
        }

        // Find cross-references in documentation
        let references = documentation.as_ref()
            .map(|doc| self.extract_references(doc))
            .unwrap_or_default();

        let item = DocumentationItem {
            name: name.clone(),
            item_type: ItemType::Function,
            documentation,
            location,
            parameters,
            return_type,
            fields: Vec::new(),
            type_parameters,
            references,
        };

        // Add to symbol table
        let full_name = self.get_full_name(&name);
        self.symbols.insert(full_name, item.clone());

        debug!("Extracted documentation for function: {}", name);
        Ok(item)
    }

    /// Extract documentation from a struct declaration
    #[instrument(skip(self, squad))]
    fn extract_from_struct_impl(&mut self, squad: &SquadStatement) -> DocumentationResult<DocumentationItem> {
        self.stats.total_types += 1;
        
        let location = SourceLocation::new(0, 0); // TODO: Extract actual location
        let name = squad.name.value.clone();
        
        // Extract field information from FieldStatements
        let mut fields = Vec::new();
        for field_stmt in &squad.fields {
            self.stats.total_fields += 1;
            fields.push(FieldInfo {
                name: field_stmt.name.value.clone(),
                type_name: field_stmt.type_name.string(),
                description: None, // TODO: Extract field documentation
            });
        }

        // Extract type parameters
        let type_parameters = squad.type_parameters.iter()
            .map(|tp| tp.name.clone())
            .collect();

        let documentation = self.extract_doc_comments(&name, &location)?;
        
        if documentation.is_some() {
            self.stats.documented_types += 1;
        }

        let references = documentation.as_ref()
            .map(|doc| self.extract_references(doc))
            .unwrap_or_default();

        let item = DocumentationItem {
            name: name.clone(),
            item_type: ItemType::Struct,
            documentation,
            location,
            parameters: Vec::new(),
            return_type: None,
            fields,
            type_parameters,
            references,
        };

        let full_name = self.get_full_name(&name);
        self.symbols.insert(full_name, item.clone());

        debug!("Extracted documentation for struct: {}", name);
        Ok(item)
    }

    /// Extract documentation from an interface declaration
    #[instrument(skip(self, collab))]
    fn extract_from_interface_impl(&mut self, collab: &CollabStatement) -> DocumentationResult<DocumentationItem> {
        self.stats.total_types += 1;
        
        let location = SourceLocation::new(0, 0); // TODO: Extract actual location
        let name = collab.name.value.clone();
        
        // Extract method signatures as fields for interfaces
        let fields = collab.methods.iter()
            .map(|method| FieldInfo {
                name: method.name.value.clone(),
                type_name: format!("fn({})", 
                    method.parameters.iter()
                        .map(|p| format!("{}: {}", p.name.value, p.param_type.string()))
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                description: None, // TODO: Extract method documentation
            })
            .collect();

        let documentation = self.extract_doc_comments(&name, &location)?;
        
        if documentation.is_some() {
            self.stats.documented_types += 1;
        }

        let references = documentation.as_ref()
            .map(|doc| self.extract_references(doc))
            .unwrap_or_default();

        let item = DocumentationItem {
            name: name.clone(),
            item_type: ItemType::Interface,
            documentation,
            location,
            parameters: Vec::new(),
            return_type: None,
            fields,
            type_parameters: Vec::new(),
            references,
        };

        let full_name = self.get_full_name(&name);
        self.symbols.insert(full_name, item.clone());

        debug!("Extracted documentation for interface: {}", name);
        Ok(item)
    }

    /// Extract parameter information
    fn extract_parameter_info(&mut self, parameter: &Parameter) -> DocumentationResult<ParameterInfo> {
        Ok(ParameterInfo {
            name: parameter.name.value.clone(),
            type_name: parameter.param_type.string(),
            description: None, // TODO: Extract parameter documentation from @param tags
        })
    }

    /// Extract field information  
    fn extract_field_info(&mut self, field: &Field) -> DocumentationResult<FieldInfo> {
        self.stats.total_fields += 1;
        
        Ok(FieldInfo {
            name: field.name.value.clone(),
            type_name: field.type_name.value.clone(),
            description: None, // TODO: Extract field documentation
        })
    }

    /// Extract documentation comments for an item
    /// TODO: This needs to be implemented to actually parse comments from source
    fn extract_doc_comments(&self, _name: &str, _location: &SourceLocation) -> DocumentationResult<Option<String>> {
        // Placeholder - in a real implementation this would:
        // 1. Look for /** */ or /// comments above the item
        // 2. Parse @param, @return, @example tags
        // 3. Extract and clean up the documentation text
        Ok(None)
    }

    /// Extract cross-references from documentation text
    fn extract_references(&self, documentation: &str) -> Vec<String> {
        let mut references = Vec::new();
        
        // Look for references in the format [Type] or [function_name]
        let re = regex::Regex::new(r"\[([A-Za-z_][A-Za-z0-9_]*)\]").unwrap();
        for cap in re.captures_iter(documentation) {
            if let Some(reference) = cap.get(1) {
                references.push(reference.as_str().to_string());
            }
        }
        
        references
    }

    /// Get the full qualified name for a symbol
    fn get_full_name(&self, name: &str) -> String {
        if self.module_path.is_empty() {
            name.to_string()
        } else {
            format!("{}::{}", self.module_path.join("::"), name)
        }
    }

    /// Validate parameter documentation against function parameters
    pub fn validate_parameters(&self, item: &DocumentationItem) -> Vec<String> {
        let mut issues = Vec::new();
        
        // Check if all parameters are documented
        for param in &item.parameters {
            if param.description.is_none() {
                issues.push(format!("Parameter '{}' is not documented", param.name));
            }
        }
        
        // TODO: Check @param tags in documentation match actual parameters
        
        issues
    }

    /// Resolve cross-references to actual symbols
    pub fn resolve_references(&self, item: &DocumentationItem) -> HashMap<String, bool> {
        let mut resolved = HashMap::new();
        
        for reference in &item.references {
            let exists = self.symbols.contains_key(reference) || 
                        self.symbols.iter().any(|(_, item)| item.name == *reference);
            resolved.insert(reference.clone(), exists);
        }
        
        resolved
    }
}

impl Default for DocumentationExtractor {
    fn default() -> Self {
        Self::new()
    }
}
