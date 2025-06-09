//! Core types for the CURSED documentation system

use crate::docs::{DocComment, DocTag};
use std::collections::HashMap;

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
            if let DocTag::Example { code, .. } = tag {
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
                if let DocTag::Param { name, description } = tag {
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
                if let DocTag::Return { description } = tag {
                    Some(description.clone())
                } else {
                    None
                }
            })
        })
    }
}

/// Documentation validation result
#[derive(Debug, Clone, serde::Serialize)]
pub struct DocumentationValidationResult {
    /// Validation errors that must be fixed
    pub errors: Vec<String>,
    /// Warnings that should be addressed
    pub warnings: Vec<String>,
    /// Items missing documentation
    pub missing_documentation: Vec<String>,
    /// Total items checked
    pub total_items: usize,
    /// Items with documentation
    pub documented_items: usize,
}

impl DocumentationValidationResult {
    /// Create new validation result
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            missing_documentation: Vec::new(),
            total_items: 0,
            documented_items: 0,
        }
    }
    
    /// Check if validation has errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    
    /// Add an error
    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }
    
    /// Add a warning
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
    
    /// Add missing documentation
    pub fn add_missing_doc(&mut self, item: String) {
        self.missing_documentation.push(item);
    }
    
    /// Get validation summary
    pub fn summary(&self) -> String {
        let coverage = if self.total_items > 0 {
            (self.documented_items as f64 / self.total_items as f64) * 100.0
        } else {
            0.0
        };
        
        format!(
            "Documentation coverage: {:.1}% ({}/{} items), {} errors, {} warnings",
            coverage,
            self.documented_items,
            self.total_items,
            self.errors.len(),
            self.warnings.len()
        )
    }
}
