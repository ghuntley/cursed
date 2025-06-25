use crate::error::CursedError;
// CURSED-specific LSP protocol extensions
// 
// Defines custom LSP methods and protocol extensions specific to CURSED

use serde::{Deserialize, Serialize};
use tower_lsp::lsp_types::*;
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
/// Request to get AST node information at a position
#[derive(Debug, Serialize, Deserialize)]
pub struct GetAstNodeParams {
/// Response for AST node information
#[derive(Debug, Serialize, Deserialize)]
pub struct AstNodeInfo {
/// Request to get type information at a position
#[derive(Debug, Serialize, Deserialize)]
pub struct GetTypeInfoParams {
/// Response for type information
#[derive(Debug, Serialize, Deserialize)]
pub struct TypeInfo {
/// Method information for types
#[derive(Debug, Serialize, Deserialize)]
pub struct MethodInfo {
/// Field information for types
#[derive(Debug, Serialize, Deserialize)]
pub struct FieldInfo {
/// Parameter information
#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterInfo {
/// Request to format document with CURSED formatter
#[derive(Debug, Serialize, Deserialize)]
pub struct FormatDocumentParams {
/// CURSED-specific formatting options
#[derive(Debug, Serialize, Deserialize)]
pub struct CursedFormattingOptions {
    pub brace_style: Option<String>, // "same-line", "next-line", "next-line-unindented"
    pub enforce_cursed_style: Option<bool>, // Suggest CURSED slang keywords
/// Response for document formatting
#[derive(Debug, Serialize, Deserialize)]
pub struct FormatDocumentResponse {
/// Information about a formatting change
#[derive(Debug, Serialize, Deserialize)]
pub struct FormattingChange {
/// Request to run linter on document
#[derive(Debug, Serialize, Deserialize)]
pub struct RunLinterParams {
/// Linter options
#[derive(Debug, Serialize, Deserialize)]
pub struct LinterOptions {
    pub severity_level: Option<String>, // "error", "warning", "info", "hint"
/// Response for linter execution
#[derive(Debug, Serialize, Deserialize)]
pub struct LinterResponse {
    pub execution_time: u64, // milliseconds
/// Linter summary statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct LintSummary {
/// Request to get goroutine information
#[derive(Debug, Serialize, Deserialize)]
pub struct GetGoroutineInfoParams {
/// Response for goroutine information
#[derive(Debug, Serialize, Deserialize)]
pub struct GoroutineInfo {
/// Channel usage information
#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelUsage {
/// Synchronization primitive information
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncPrimitive {
    pub primitive_type: String, // "mutex", "rwlock", "waitgroup", etc.
/// Deadlock warning information
#[derive(Debug, Serialize, Deserialize)]
pub struct DeadlockWarning {
/// Request to get channel information
#[derive(Debug, Serialize, Deserialize)]
pub struct GetChannelInfoParams {
/// Response for channel information
#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelInfo {
/// Channel operation information
#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelOperation {
    pub operation_type: String, // "send", "receive", "select_send", "select_receive"
/// Request to validate syntax
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateSyntaxParams {
/// Response for syntax validation
#[derive(Debug, Serialize, Deserialize)]
pub struct SyntaxValidationResponse {
/// Syntax error information
#[derive(Debug, Serialize, Deserialize)]
pub struct SyntaxError {
/// Syntax warning information
#[derive(Debug, Serialize, Deserialize)]
pub struct SyntaxWarning {
/// Request to get documentation
#[derive(Debug, Serialize, Deserialize)]
pub struct GetDocumentationParams {
/// Response for documentation
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentationResponse {
/// Documentation example
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentationExample {
/// CURSED language configuration for LSP clients
#[derive(Debug, Serialize, Deserialize)]
pub struct CursedLanguageConfig {
/// Comment patterns for the language
#[derive(Debug, Serialize, Deserialize)]
pub struct CommentPatterns {
/// Bracket pair configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct BracketPair {
/// Indentation rules
#[derive(Debug, Serialize, Deserialize)]
pub struct IndentationRules {
/// Folding rules
#[derive(Debug, Serialize, Deserialize)]
pub struct FoldingRules {
/// Folding marker
#[derive(Debug, Serialize, Deserialize)]
pub struct FoldingMarker {
    pub kind: String, // "comment", "region", "imports"
impl Default for CursedLanguageConfig {
    fn default() -> Self {
        Self {
            comment_patterns: CommentPatterns {
                line_comment: "//".to_string(),
                block_comment_start: "/*".to_string(),
                block_comment_end: "*/".to_string(),
            bracket_pairs: vec![
                BracketPair {
                BracketPair {
                BracketPair {
                BracketPair {
            indentation_rules: IndentationRules {
                ignore_pattern: Some(r"^\s*(//.*|/\*.*\*/)$".to_string()),
            folding_rules: FoldingRules {
                markers: vec![
                    FoldingMarker {
                    FoldingMarker {
                    FoldingMarker {
                    FoldingMarker {
        }
    }
/// Utility functions for working with CURSED LSP protocol

impl CursedFormattingOptions {
    /// Convert to standard LSP formatting options
    pub fn to_formatting_options(&self) -> FormattingOptions {
        let mut properties = std::collections::HashMap::new();
        
        if let Some(line_width) = self.line_width {
            properties.insert("lineWidth".to_string(), serde_json::Value::Number(line_width.into()));
        if let Some(brace_style) = &self.brace_style {
            properties.insert("braceStyle".to_string(), serde_json::Value::String(brace_style.clone()));
        if let Some(space_around_operators) = self.space_around_operators {
            properties.insert("spaceAroundOperators".to_string(), serde_json::Value::Bool(space_around_operators));
        if let Some(space_after_comma) = self.space_after_comma {
            properties.insert("spaceAfterComma".to_string(), serde_json::Value::Bool(space_after_comma));
        if let Some(max_empty_lines) = self.max_empty_lines {
            properties.insert("maxEmptyLines".to_string(), serde_json::Value::Number(max_empty_lines.into()));
        if let Some(enforce_cursed_style) = self.enforce_cursed_style {
            properties.insert("enforceCursedStyle".to_string(), serde_json::Value::Bool(enforce_cursed_style));
        FormattingOptions {
            properties: HashMap::new(), // Empty properties map
        }
    }
impl Default for CursedFormattingOptions {
    fn default() -> Self {
        Self {
        }
    }
impl Default for LinterOptions {
    fn default() -> Self {
        Self {
        }
    }
}
