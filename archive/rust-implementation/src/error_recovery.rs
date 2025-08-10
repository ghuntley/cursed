//! Robust Error Recovery System for CURSED Compiler
//! 
//! This module provides comprehensive error recovery mechanisms for:
//! - Parser error recovery and continuation
//! - Semantic error accumulation and reporting
//! - Source location tracking and context generation
//! - Error suggestions and fix recommendations
//! - Graceful degradation for LLVM codegen

use crate::error_types::{Error, Result};
use crate::lexer::{Token, TokenKind};
use std::collections::HashMap;
use std::fmt;

/// Source location information for precise error reporting
#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
    pub file: Option<String>,
}

impl SourceLocation {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self { line, column, offset, file: None }
    }
    
    pub fn with_file(mut self, file: String) -> Self {
        self.file = Some(file);
        self
    }
}

/// Enhanced error with recovery information
#[derive(Debug, Clone)]
pub struct RecoveryError {
    pub error: Error,
    pub location: SourceLocation,
    pub context: ErrorContext,
    pub suggestions: Vec<ErrorSuggestion>,
    pub severity: ErrorSeverity,
    pub related_errors: Vec<Box<RecoveryError>>,
}

/// Error context with source code snippet
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub source_line: String,
    pub highlight_start: usize,
    pub highlight_end: usize,
    pub surrounding_lines: Vec<(usize, String)>,
}

/// Error suggestion for fixing issues
#[derive(Debug, Clone)]
pub struct ErrorSuggestion {
    pub message: String,
    pub replacement: Option<String>,
    pub location: Option<SourceLocation>,
    pub suggestion_type: SuggestionType,
}

#[derive(Debug, Clone)]
pub enum SuggestionType {
    AddToken(TokenKind),
    RemoveToken,
    ReplaceToken(String),
    AddImport(String),
    TypeHint(String),
    SyntaxFix(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorSeverity {
    Fatal,     // Cannot continue compilation
    Error,     // Error but can recover
    Warning,   // Warning but compilation continues
    Note,      // Informational note
    Help,      // Help message
}

/// Error recovery strategies
#[derive(Debug, Clone)]
pub enum RecoveryStrategy {
    /// Skip to next statement/expression
    SkipToNext,
    /// Insert missing token
    InsertToken(TokenKind),
    /// Replace current token
    ReplaceToken(TokenKind),
    /// Backtrack and try alternative parsing
    Backtrack,
    /// Continue with default/placeholder value
    UseDefault,
    /// Abort current scope but continue outer scope
    AbortScope,
}

/// Error recovery manager
pub struct ErrorRecoveryManager {
    pub errors: Vec<RecoveryError>,
    pub recovery_points: Vec<RecoveryPoint>,
    pub max_errors: usize,
    pub source_cache: HashMap<String, Vec<String>>,
    pub suppress_follow_up: bool,
}

/// Recovery point for backtracking
#[derive(Debug, Clone)]
pub struct RecoveryPoint {
    pub location: SourceLocation,
    pub token_index: usize,
    pub parser_state: ParserState,
}

#[derive(Debug, Clone)]
pub struct ParserState {
    pub scope_depth: usize,
    pub in_function: bool,
    pub in_loop: bool,
    pub expected_tokens: Vec<TokenKind>,
}

impl ErrorRecoveryManager {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            recovery_points: Vec::new(),
            max_errors: 100,
            source_cache: HashMap::new(),
            suppress_follow_up: false,
        }
    }
    
    /// Add error with recovery information
    pub fn add_error(&mut self, error: Error, location: SourceLocation, context: ErrorContext) -> RecoveryError {
        let suggestions = self.generate_suggestions(&error, &location, &context);
        let severity = self.determine_severity(&error);
        
        let recovery_error = RecoveryError {
            error,
            location,
            context,
            suggestions,
            severity,
            related_errors: Vec::new(),
        };
        
        self.errors.push(recovery_error.clone());
        recovery_error
    }
    
    /// Generate helpful suggestions based on error type
    fn generate_suggestions(&self, error: &Error, location: &SourceLocation, context: &ErrorContext) -> Vec<ErrorSuggestion> {
        let mut suggestions = Vec::new();
        
        match error {
            Error::Parse(msg) => {
                if msg.contains("Expected") {
                    if msg.contains("Expected identifier") {
                        suggestions.push(ErrorSuggestion {
                            message: "Add a valid identifier here".to_string(),
                            replacement: Some("variable_name".to_string()),
                            location: Some(location.clone()),
                            suggestion_type: SuggestionType::SyntaxFix("identifier".to_string()),
                        });
                    } else if msg.contains("Expected ')'") {
                        suggestions.push(ErrorSuggestion {
                            message: "Add missing closing parenthesis".to_string(),
                            replacement: Some(")".to_string()),
                            location: Some(location.clone()),
                            suggestion_type: SuggestionType::AddToken(TokenKind::RightParen),
                        });
                    } else if msg.contains("Expected ';'") {
                        suggestions.push(ErrorSuggestion {
                            message: "Add semicolon to terminate statement".to_string(),
                            replacement: Some(";".to_string()),
                            location: Some(location.clone()),
                            suggestion_type: SuggestionType::AddToken(TokenKind::Semicolon),
                        });
                    }
                }
                
                // Suggest common fixes for syntax errors
                if context.source_line.contains("vibez.spill") && !context.source_line.contains("(") {
                    suggestions.push(ErrorSuggestion {
                        message: "Function calls require parentheses".to_string(),
                        replacement: Some("vibez.spill(\"message\")".to_string()),
                        location: Some(location.clone()),
                        suggestion_type: SuggestionType::SyntaxFix("function_call".to_string()),
                    });
                }
            }
            
            Error::Type(msg) => {
                if msg.contains("type mismatch") {
                    suggestions.push(ErrorSuggestion {
                        message: "Consider using type conversion or checking variable types".to_string(),
                        replacement: None,
                        location: Some(location.clone()),
                        suggestion_type: SuggestionType::TypeHint("type conversion".to_string()),
                    });
                }
                
                if msg.contains("undefined variable") {
                    suggestions.push(ErrorSuggestion {
                        message: "Declare the variable before using it".to_string(),
                        replacement: Some("sus variable_name type_name = value".to_string()),
                        location: Some(location.clone()),
                        suggestion_type: SuggestionType::SyntaxFix("variable_declaration".to_string()),
                    });
                }
            }
            
            Error::Import(msg) => {
                if msg.contains("module not found") {
                    suggestions.push(ErrorSuggestion {
                        message: "Check if the module exists in stdlib/ or add the correct import path".to_string(),
                        replacement: Some("yeet \"module_name\"".to_string()),
                        location: Some(location.clone()),
                        suggestion_type: SuggestionType::AddImport("module_name".to_string()),
                    });
                }
            }
            
            _ => {
                // Generic helpful suggestions
                suggestions.push(ErrorSuggestion {
                    message: "Check the CURSED language documentation for syntax reference".to_string(),
                    replacement: None,
                    location: None,
                    suggestion_type: SuggestionType::SyntaxFix("documentation".to_string()),
                });
            }
        }
        
        suggestions
    }
    
    /// Determine error severity based on error type
    fn determine_severity(&self, error: &Error) -> ErrorSeverity {
        match error {
            Error::Parse(_) => ErrorSeverity::Error,
            Error::Type(_) => ErrorSeverity::Error,
            Error::Runtime(_) => ErrorSeverity::Fatal,
            Error::Compile(_) => ErrorSeverity::Error,
            Error::Import(_) => ErrorSeverity::Error,
            Error::Lexer(_) => ErrorSeverity::Error,
            Error::Io(_) => ErrorSeverity::Fatal,
            Error::Memory(_) => ErrorSeverity::Fatal,
            _ => ErrorSeverity::Error,
        }
    }
    
    /// Check if we should continue compilation
    pub fn should_continue(&self) -> bool {
        let fatal_errors = self.errors.iter()
            .filter(|e| e.severity == ErrorSeverity::Fatal)
            .count();
        
        let total_errors = self.errors.iter()
            .filter(|e| e.severity == ErrorSeverity::Error || e.severity == ErrorSeverity::Fatal)
            .count();
        
        fatal_errors == 0 && total_errors < self.max_errors
    }
    
    /// Generate comprehensive error report
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        
        let error_count = self.errors.iter()
            .filter(|e| e.severity == ErrorSeverity::Error || e.severity == ErrorSeverity::Fatal)
            .count();
        let warning_count = self.errors.iter()
            .filter(|e| e.severity == ErrorSeverity::Warning)
            .count();
        
        report.push_str(&format!("Compilation Results: {} error(s), {} warning(s)\n\n", error_count, warning_count));
        
        for (index, error) in self.errors.iter().enumerate() {
            report.push_str(&format!("{}. {}: {}\n", 
                index + 1, 
                self.severity_to_string(&error.severity),
                error.error));
            
            // Add location information
            if let Some(ref file) = error.location.file {
                report.push_str(&format!("   at {}:{}:{}\n", file, error.location.line, error.location.column));
            } else {
                report.push_str(&format!("   at line {}, column {}\n", error.location.line, error.location.column));
            }
            
            // Add source context
            report.push_str(&format!("   | {}\n", error.context.source_line));
            report.push_str("   | ");
            for i in 0..error.context.highlight_start {
                if i < error.context.source_line.len() && error.context.source_line.chars().nth(i) == Some('\t') {
                    report.push('\t');
                } else {
                    report.push(' ');
                }
            }
            for _ in error.context.highlight_start..error.context.highlight_end {
                report.push('^');
            }
            report.push('\n');
            
            // Add suggestions
            for suggestion in &error.suggestions {
                report.push_str(&format!("   help: {}\n", suggestion.message));
                if let Some(ref replacement) = suggestion.replacement {
                    report.push_str(&format!("         suggestion: {}\n", replacement));
                }
            }
            
            report.push('\n');
        }
        
        if error_count == 0 && warning_count == 0 {
            report.push_str("✅ Compilation successful with no errors or warnings!\n");
        } else if error_count == 0 {
            report.push_str("✅ Compilation successful with warnings.\n");
        } else {
            report.push_str("❌ Compilation failed. Please fix the errors above.\n");
        }
        
        report
    }
    
    fn severity_to_string(&self, severity: &ErrorSeverity) -> &'static str {
        match severity {
            ErrorSeverity::Fatal => "fatal error",
            ErrorSeverity::Error => "error",
            ErrorSeverity::Warning => "warning",
            ErrorSeverity::Note => "note",
            ErrorSeverity::Help => "help",
        }
    }
    
    /// Create recovery point for backtracking
    pub fn create_recovery_point(&mut self, location: SourceLocation, token_index: usize, state: ParserState) {
        self.recovery_points.push(RecoveryPoint {
            location,
            token_index,
            parser_state: state,
        });
    }
    
    /// Restore to last recovery point
    pub fn restore_recovery_point(&mut self) -> Option<RecoveryPoint> {
        self.recovery_points.pop()
    }
    
    /// Cache source file for context generation
    pub fn cache_source(&mut self, filename: &str, content: &str) {
        let lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
        self.source_cache.insert(filename.to_string(), lines);
    }
    
    /// Generate error context with source lines
    pub fn generate_context(&self, location: &SourceLocation, filename: Option<&str>) -> ErrorContext {
        let default_context = ErrorContext {
            source_line: "".to_string(),
            highlight_start: 0,
            highlight_end: 0,
            surrounding_lines: Vec::new(),
        };
        
        let source_lines = match filename {
            Some(file) => self.source_cache.get(file),
            None => self.source_cache.values().next(),
        };
        
        let lines = match source_lines {
            Some(lines) => lines,
            None => return default_context,
        };
        
        if location.line == 0 || location.line > lines.len() {
            return default_context;
        }
        
        let source_line = lines[location.line - 1].clone();
        let highlight_start = location.column.saturating_sub(1);
        let highlight_end = (highlight_start + 1).min(source_line.len());
        
        // Get surrounding lines for context
        let mut surrounding_lines = Vec::new();
        let start_line = location.line.saturating_sub(3);
        let end_line = (location.line + 2).min(lines.len());
        
        for line_num in start_line..end_line {
            if line_num != location.line && line_num > 0 && line_num <= lines.len() {
                surrounding_lines.push((line_num, lines[line_num - 1].clone()));
            }
        }
        
        ErrorContext {
            source_line,
            highlight_start,
            highlight_end,
            surrounding_lines,
        }
    }
}

impl fmt::Display for RecoveryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at line {}, column {}: {}", 
               match self.severity {
                   ErrorSeverity::Fatal => "fatal error",
                   ErrorSeverity::Error => "error",
                   ErrorSeverity::Warning => "warning",
                   ErrorSeverity::Note => "note",
                   ErrorSeverity::Help => "help",
               },
               self.location.line,
               self.location.column,
               self.error)
    }
}

impl Default for ErrorRecoveryManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Parser error recovery trait
pub trait ParserErrorRecovery {
    fn recover_from_error(&mut self, error: Error, strategy: RecoveryStrategy) -> Result<bool>;
    fn try_recovery_strategies(&mut self, error: Error) -> Result<bool>;
    fn synchronize_to_statement(&mut self);
    fn synchronize_to_expression(&mut self);
}

/// Semantic error recovery trait  
pub trait SemanticErrorRecovery {
    fn accumulate_error(&mut self, error: Error, location: SourceLocation);
    fn can_continue_analysis(&self) -> bool;
    fn generate_placeholder_type(&self) -> String;
    fn skip_erroneous_declaration(&mut self) -> bool;
}

/// Codegen error recovery trait
pub trait CodegenErrorRecovery {
    fn graceful_codegen_failure(&mut self, error: Error) -> String;
    fn generate_error_placeholder(&self, context: &str) -> String;
    fn fallback_to_interpretation(&self) -> bool;
}
