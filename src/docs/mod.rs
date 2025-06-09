//! Documentation generation system for the CURSED programming language
//!
//! This module provides comprehensive documentation generation capabilities
//! that parse CURSED source files, extract documentation comments, and
//! generate professional HTML documentation with navigation, cross-references,
//! and search functionality.

pub mod ast_extractor;
pub mod comment_parser;
pub mod config;
pub mod doc_generator;
pub mod doc_generator_simplified;
pub mod html_renderer;
pub mod markdown_generator;
pub mod package_docs;
pub mod server;
pub mod templates;
pub mod type_resolver;
pub mod types;

// Public API exports
pub use ast_extractor::{AstExtractor, DocumentationItem as AstDocumentationItem};
pub use doc_generator::{DocumentationGenerator, DocConfig, DocumentationGenerationResult};
pub use types::{DocumentationItem, ItemType, ParameterInfo, FieldInfo, DocumentationValidationResult};
pub use comment_parser::{CommentParser, DocComment, DocTag};
pub use package_docs::{PackageDocumentation, ModuleInfo};
pub use templates::{HtmlTemplate, TemplateEngine};
pub use type_resolver::{TypeResolver, ResolvedType, TypeHierarchy};
pub use markdown_generator::{MarkdownGenerator, MarkdownConfig, MarkdownFormat, MarkdownOutput};

/// Documentation generation error type
#[derive(Debug, Clone)]
pub enum DocError {
    ParseError(String),
    IoError(String),
    TemplateError(String),
    AstError(String),
}

impl std::fmt::Display for DocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            DocError::IoError(msg) => write!(f, "IO error: {}", msg),
            DocError::TemplateError(msg) => write!(f, "Template error: {}", msg),
            DocError::AstError(msg) => write!(f, "AST error: {}", msg),
        }
    }
}

impl std::error::Error for DocError {}

/// Result type for documentation operations
pub type DocResult<T> = Result<T, DocError>;
