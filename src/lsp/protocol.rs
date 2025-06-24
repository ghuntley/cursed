use crate::error::Error;
//! CURSED-specific LSP protocol extensions
//! 
//! Defines custom LSP methods and protocol extensions specific to CURSED

use serde::{Deserialize, Serialize};
use tower_lsp::lsp_crate::types::*;
use std::collections::HashMap;

/// CURSED-specific LSP method names
pub mod methods {
    pub const GET_AST_NODE: &str = "cursed/getAstNode";
    pub const GET_TYPE_INFO: &str = "cursed/getTypeInfo";
    pub const FORMAT_DOCUMENT: &str = "cursed/formatDocument";
    pub const RUN_LINTER: &str = "cursed/runLinter";
    pub const GET_GOROUTINE_INFO: &str = "cursed/getGoroutineInfo";
    pub const GET_CHANNEL_INFO: &str = "cursed/getChannelInfo";
    pub const VALIDATE_SYNTAX: &str = "cursed/validateSyntax";
    pub const GET_DOCUMENTATION: &str = "cursed/getDocumentation";
}

/// Request to get AST node information at a position
#[derive(Debug, Serialize, Deserialize)]
pub struct GetAstNodeParams {
    pub text_document: TextDocumentIdentifier,
    pub position: Position,
    pub include_children: Option<bool>,
    pub max_depth: Option<usize>,
}

/// Response for AST node information
#[derive(Debug, Serialize, Deserialize)]
pub struct AstNodeInfo {
    pub node_type: String,
    pub range: Range,
    pub text: String,
    pub children: Vec<AstNodeInfo>,
    pub properties: std::collections::HashMap<String, serde_json::Value>,
}

/// Request to get type information at a position
#[derive(Debug, Serialize, Deserialize)]
pub struct GetTypeInfoParams {
    pub text_document: TextDocumentIdentifier,
    pub position: Position,
    pub include_hierarchy: Option<bool>,
}

/// Response for type information
#[derive(Debug, Serialize, Deserialize)]
pub struct TypeInfo {
    pub type_name: String,
    pub base_type: Option<String>,
    pub nullable: bool,
    pub generic_params: Vec<String>,
    pub interfaces: Vec<String>,
    pub methods: Vec<MethodInfo>,
    pub fields: Vec<FieldInfo>,
    pub documentation: Option<String>,
}

/// Method information for types
#[derive(Debug, Serialize, Deserialize)]
pub struct MethodInfo {
    pub name: String,
    pub parameters: Vec<ParameterInfo>,
    pub return_type: Option<String>,
    pub is_async: bool,
    pub visibility: String,
    pub documentation: Option<String>,
}

/// Field information for types
#[derive(Debug, Serialize, Deserialize)]
pub struct FieldInfo {
    pub name: String,
    pub field_type: String,
    pub nullable: bool,
    pub visibility: String,
    pub documentation: Option<String>,
}

/// Parameter information
#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterInfo {
    pub name: String,
    pub param_type: String,
    pub optional: bool,
    pub default_value: Option<String>,
}

/// Request to format document with CURSED formatter
#[derive(Debug, Serialize, Deserialize)]
pub struct FormatDocumentParams {
    pub text_document: TextDocumentIdentifier,
    pub options: CursedFormattingOptions,
}

/// CURSED-specific formatting options
#[derive(Debug, Serialize, Deserialize)]
pub struct CursedFormattingOptions {
    pub indent_size: Option<usize>,
    pub use_tabs: Option<bool>,
    pub line_width: Option<usize>,
    pub brace_style: Option<String>, // "same-line", "next-line", "next-line-unindented"
    pub space_around_operators: Option<bool>,
    pub space_after_comma: Option<bool>,
    pub max_empty_lines: Option<usize>,
    pub enforce_cursed_style: Option<bool>, // Suggest CURSED slang keywords
}

/// Response for document formatting
#[derive(Debug, Serialize, Deserialize)]
pub struct FormatDocumentResponse {
    pub formatted_content: String,
    pub changes_made: Vec<FormattingChange>,
    pub warnings: Vec<String>,
}

/// Information about a formatting change
#[derive(Debug, Serialize, Deserialize)]
pub struct FormattingChange {
    pub range: Range,
    pub old_text: String,
    pub new_text: String,
    pub reason: String,
}

/// Request to run linter on document
#[derive(Debug, Serialize, Deserialize)]
pub struct RunLinterParams {
    pub text_document: TextDocumentIdentifier,
    pub options: LinterOptions,
}

/// Linter options
#[derive(Debug, Serialize, Deserialize)]
pub struct LinterOptions {
    pub check_style: Option<bool>,
    pub check_performance: Option<bool>,
    pub check_security: Option<bool>,
    pub check_best_practices: Option<bool>,
    pub severity_level: Option<String>, // "error", "warning", "info", "hint"
}

/// Response for linter execution
#[derive(Debug, Serialize, Deserialize)]
pub struct LinterResponse {
    pub diagnostics: Vec<Diagnostic>,
    pub summary: LintSummary,
    pub execution_time: u64, // milliseconds
}

/// Linter summary statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct LintSummary {
    pub errors_count: usize,
    pub warnings_count: usize,
    pub info_count: usize,
    pub hints_count: usize,
    pub files_checked: usize,
    pub rules_applied: Vec<String>,
}

/// Request to get goroutine information
#[derive(Debug, Serialize, Deserialize)]
pub struct GetGoroutineInfoParams {
    pub text_document: TextDocumentIdentifier,
    pub position: Position,
}

/// Response for goroutine information
#[derive(Debug, Serialize, Deserialize)]
pub struct GoroutineInfo {
    pub is_async_context: bool,
    pub spawn_locations: Vec<Location>,
    pub channel_usage: Vec<ChannelUsage>,
    pub sync_primitives: Vec<SyncPrimitive>,
    pub potential_deadlocks: Vec<DeadlockWarning>,
}

/// Channel usage information
#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelUsage {
    pub channel_name: String,
    pub channel_type: String,
    pub send_locations: Vec<Location>,
    pub receive_locations: Vec<Location>,
    pub close_locations: Vec<Location>,
    pub is_buffered: bool,
    pub buffer_size: Option<usize>,
}

/// Synchronization primitive information
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncPrimitive {
    pub primitive_type: String, // "mutex", "rwlock", "waitgroup", etc.
    pub location: Location,
    pub usage_pattern: String,
}

/// Deadlock warning information
#[derive(Debug, Serialize, Deserialize)]
pub struct DeadlockWarning {
    pub message: String,
    pub locations: Vec<Location>,
    pub severity: String,
    pub suggestions: Vec<String>,
}

/// Request to get channel information
#[derive(Debug, Serialize, Deserialize)]
pub struct GetChannelInfoParams {
    pub text_document: TextDocumentIdentifier,
    pub position: Position,
}

/// Response for channel information
#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelInfo {
    pub channel_name: String,
    pub element_type: String,
    pub is_buffered: bool,
    pub buffer_size: Option<usize>,
    pub creation_location: Option<Location>,
    pub send_operations: Vec<ChannelOperation>,
    pub receive_operations: Vec<ChannelOperation>,
    pub close_operations: Vec<Location>,
    pub goroutines_using: Vec<String>,
}

/// Channel operation information
#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelOperation {
    pub location: Location,
    pub operation_type: String, // "send", "receive", "select_send", "select_receive"
    pub is_blocking: bool,
    pub timeout: Option<String>,
}

/// Request to validate syntax
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateSyntaxParams {
    pub text_document: TextDocumentIdentifier,
    pub include_semantic_analysis: Option<bool>,
}

/// Response for syntax validation
#[derive(Debug, Serialize, Deserialize)]
pub struct SyntaxValidationResponse {
    pub is_valid: bool,
    pub errors: Vec<SyntaxError>,
    pub warnings: Vec<SyntaxWarning>,
    pub parse_tree: Option<serde_json::Value>,
}

/// Syntax error information
#[derive(Debug, Serialize, Deserialize)]
pub struct SyntaxError {
    pub range: Range,
    pub message: String,
    pub error_code: Option<String>,
    pub suggestions: Vec<String>,
}

/// Syntax warning information
#[derive(Debug, Serialize, Deserialize)]
pub struct SyntaxWarning {
    pub range: Range,
    pub message: String,
    pub warning_code: Option<String>,
    pub suggestion: Option<String>,
}

/// Request to get documentation
#[derive(Debug, Serialize, Deserialize)]
pub struct GetDocumentationParams {
    pub text_document: TextDocumentIdentifier,
    pub position: Position,
    pub include_examples: Option<bool>,
}

/// Response for documentation
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentationResponse {
    pub symbol_name: String,
    pub symbol_type: String,
    pub documentation: String,
    pub examples: Vec<DocumentationExample>,
    pub related_symbols: Vec<String>,
    pub source_location: Option<Location>,
}

/// Documentation example
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentationExample {
    pub title: String,
    pub code: String,
    pub description: Option<String>,
}

/// CURSED language configuration for LSP clients
#[derive(Debug, Serialize, Deserialize)]
pub struct CursedLanguageConfig {
    pub file_extensions: Vec<String>,
    pub comment_patterns: CommentPatterns,
    pub bracket_pairs: Vec<BracketPair>,
    pub indentation_rules: IndentationRules,
    pub folding_rules: FoldingRules,
}

/// Comment patterns for the language
#[derive(Debug, Serialize, Deserialize)]
pub struct CommentPatterns {
    pub line_comment: String,
    pub block_comment_start: String,
    pub block_comment_end: String,
}

/// Bracket pair configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct BracketPair {
    pub open: String,
    pub close: String,
    pub auto_close: bool,
}

/// Indentation rules
#[derive(Debug, Serialize, Deserialize)]
pub struct IndentationRules {
    pub increase_indent_pattern: String,
    pub decrease_indent_pattern: String,
    pub ignore_pattern: Option<String>,
}

/// Folding rules
#[derive(Debug, Serialize, Deserialize)]
pub struct FoldingRules {
    pub markers: Vec<FoldingMarker>,
    pub auto_fold_imports: bool,
    pub auto_fold_comments: bool,
}

/// Folding marker
#[derive(Debug, Serialize, Deserialize)]
pub struct FoldingMarker {
    pub start_pattern: String,
    pub end_pattern: String,
    pub kind: String, // "comment", "region", "imports"
}

impl Default for CursedLanguageConfig {
    fn default() -> Self {
        Self {
            file_extensions: Vec::from(["csd".to_string()]),
            comment_patterns: CommentPatterns {
                line_comment: "//".to_string(),
                block_comment_start: "/*".to_string(),
                block_comment_end: "*/".to_string(),
            },
            bracket_pairs: vec![
                BracketPair {
                    open: "{".to_string(),
                    close: "}".to_string(),
                    auto_close: true,
                },
                BracketPair {
                    open: "(".to_string(),
                    close: ")".to_string(),
                    auto_close: true,
                },
                BracketPair {
                    open: "[".to_string(),
                    close: "]".to_string(),
                    auto_close: true,
                },
                BracketPair {
                    open: "\"".to_string(),
                    close: "\"".to_string(),
                    auto_close: true,
                },
            ],
            indentation_rules: IndentationRules {
                increase_indent_pattern: r".*[\{\[\(]\s*$".to_string(),
                decrease_indent_pattern: r"^\s*[\}\]\)].*$".to_string(),
                ignore_pattern: Some(r"^\s*(//.*|/\*.*\*/)$".to_string()),
            },
            folding_rules: FoldingRules {
                markers: vec![
                    FoldingMarker {
                        start_pattern: r"^\s*slay\s+\w+.*\{\s*$".to_string(),
                        end_pattern: r"^\s*\}\s*$".to_string(),
                        kind: "region".to_string(),
                    },
                    FoldingMarker {
                        start_pattern: r"^\s*squad\s+\w+.*\{\s*$".to_string(),
                        end_pattern: r"^\s*\}\s*$".to_string(),
                        kind: "region".to_string(),
                    },
                    FoldingMarker {
                        start_pattern: r"^\s*collab\s+\w+.*\{\s*$".to_string(),
                        end_pattern: r"^\s*\}\s*$".to_string(),
                        kind: "region".to_string(),
                    },
                    FoldingMarker {
                        start_pattern: r"^\s*use\s+".to_string(),
                        end_pattern: r"^(?!\s*use\s+).*$".to_string(),
                        kind: "imports".to_string(),
                    },
                ],
                auto_fold_imports: false,
                auto_fold_comments: false,
            },
        }
    }
}

/// Utility functions for working with CURSED LSP protocol

impl CursedFormattingOptions {
    /// Convert to standard LSP formatting options
    pub fn to_formatting_options(&self) -> FormattingOptions {
        let mut properties = std::collections::HashMap::new();
        
        if let Some(line_width) = self.line_width {
            properties.insert("lineWidth".to_string(), serde_json::Value::Number(line_width.into()));
        }
        
        if let Some(brace_style) = &self.brace_style {
            properties.insert("braceStyle".to_string(), serde_json::Value::String(brace_style.clone()));
        }
        
        if let Some(space_around_operators) = self.space_around_operators {
            properties.insert("spaceAroundOperators".to_string(), serde_json::Value::Bool(space_around_operators));
        }
        
        if let Some(space_after_comma) = self.space_after_comma {
            properties.insert("spaceAfterComma".to_string(), serde_json::Value::Bool(space_after_comma));
        }
        
        if let Some(max_empty_lines) = self.max_empty_lines {
            properties.insert("maxEmptyLines".to_string(), serde_json::Value::Number(max_empty_lines.into()));
        }
        
        if let Some(enforce_cursed_style) = self.enforce_cursed_style {
            properties.insert("enforceCursedStyle".to_string(), serde_json::Value::Bool(enforce_cursed_style));
        }

        FormattingOptions {
            tab_size: self.indent_size.unwrap_or(4) as u32,
            insert_spaces: !self.use_tabs.unwrap_or(false),
            properties: HashMap::new(), // Empty properties map
            trim_trailing_whitespace: Some(true),
            insert_final_newline: Some(true),
            trim_final_newlines: Some(true),
        }
    }
}

impl Default for CursedFormattingOptions {
    fn default() -> Self {
        Self {
            indent_size: Some(4),
            use_tabs: Some(false),
            line_width: Some(120),
            brace_style: Some("same-line".to_string()),
            space_around_operators: Some(true),
            space_after_comma: Some(true),
            max_empty_lines: Some(2),
            enforce_cursed_style: Some(true),
        }
    }
}

impl Default for LinterOptions {
    fn default() -> Self {
        Self {
            check_style: Some(true),
            check_performance: Some(true),
            check_security: Some(true),
            check_best_practices: Some(true),
            severity_level: Some("warning".to_string()),
        }
    }
}
