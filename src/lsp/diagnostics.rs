//! Diagnostics Provider for CURSED Language Server
//! Provides real-time error detection and reporting

use tower_lsp::lsp_types::*;
use crate::error::CursedError;
use crate::lexer::{Lexer, LexError};
use crate::parser::{Parser, ParseError};
use crate::type_system::{TypeChecker, TypeError};
use std::collections::HashMap;

/// CURSED diagnostics provider
pub struct CursedDiagnosticsProvider {
    /// Cache of document diagnostics
    diagnostics_cache: HashMap<Url, Vec<Diagnostic>>,
}

impl CursedDiagnosticsProvider {
    pub fn new() -> Self {
        Self {
            diagnostics_cache: HashMap::new(),
        }
    }

    /// Analyze document and return diagnostics
    pub fn analyze_document(&mut self, uri: &Url, text: &str, version: Option<i32>) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        // Lexical analysis
        let mut lexer = Lexer::new(text);
        let tokens = match lexer.tokenize() {
            Ok(tokens) => tokens,
            Err(errors) => {
                for error in &errors {
                    diagnostics.push(self.lex_error_to_diagnostic(error, text));
                }
                // Return early if lexing failed
                self.diagnostics_cache.insert(uri.clone(), diagnostics.clone());
                return diagnostics;
            }
        };

        // Syntax analysis
        let mut parser = Parser::new(tokens);
        let ast = match parser.parse() {
            Ok(program) => program,
            Err(errors) => {
                for error in &errors {
                    diagnostics.push(self.parse_error_to_diagnostic(error, text));
                }
                // Return early if parsing failed
                self.diagnostics_cache.insert(uri.clone(), diagnostics.clone());
                return diagnostics;
            }
        };

        // Semantic analysis
        let mut type_checker = TypeChecker::new();
        if let Err(errors) = type_checker.check_program(&ast) {
            for error in &errors {
                diagnostics.push(self.type_error_to_diagnostic(error, text));
            }
        }

        // CURSED-specific lint checks
        diagnostics.extend(self.cursed_lint_checks(text));

        self.diagnostics_cache.insert(uri.clone(), diagnostics.clone());
        diagnostics
    }

    /// Convert lexical error to LSP diagnostic
    fn lex_error_to_diagnostic(&self, error: &LexError, text: &str) -> Diagnostic {
        let (line, character, end_character) = self.get_error_position(error.position, text);
        
        Diagnostic {
            range: Range {
                start: Position { line, character },
                end: Position { line, character: end_character },
            },
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(NumberOrString::String("CURSED_LEX_ERROR".to_string())),
            code_description: None,
            source: Some("cursed-lsp".to_string()),
            message: format!("Lexical error: {}", error.message),
            related_information: None,
            tags: None,
            data: None,
        }
    }

    /// Convert parse error to LSP diagnostic
    fn parse_error_to_diagnostic(&self, error: &ParseError, text: &str) -> Diagnostic {
        let (line, character, end_character) = self.get_error_position(error.position, text);
        
        Diagnostic {
            range: Range {
                start: Position { line, character },
                end: Position { line, character: end_character },
            },
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(NumberOrString::String("CURSED_PARSE_ERROR".to_string())),
            code_description: None,
            source: Some("cursed-lsp".to_string()),
            message: format!("Syntax error: {}", error.message),
            related_information: None,
            tags: None,
            data: None,
        }
    }

    /// Convert type error to LSP diagnostic
    fn type_error_to_diagnostic(&self, error: &TypeError, text: &str) -> Diagnostic {
        let (line, character, end_character) = self.get_error_position(error.position, text);
        
        let severity = match error.severity {
            crate::type_system::ErrorSeverity::Error => DiagnosticSeverity::ERROR,
            crate::type_system::ErrorSeverity::Warning => DiagnosticSeverity::WARNING,
            crate::type_system::ErrorSeverity::Info => DiagnosticSeverity::INFORMATION,
        };

        Diagnostic {
            range: Range {
                start: Position { line, character },
                end: Position { line, character: end_character },
            },
            severity: Some(severity),
            code: Some(NumberOrString::String("CURSED_TYPE_ERROR".to_string())),
            code_description: None,
            source: Some("cursed-lsp".to_string()),
            message: format!("Type error: {}", error.message),
            related_information: None,
            tags: None,
            data: None,
        }
    }

    /// CURSED-specific lint checks
    fn cursed_lint_checks(&self, text: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = text.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            // Check for deprecated keywords
            if line.contains("if ") || line.contains("else ") {
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position { 
                            line: line_num as u32, 
                            character: line.find("if ").unwrap_or(line.find("else ").unwrap_or(0)) as u32,
                        },
                        end: Position { 
                            line: line_num as u32, 
                            character: (line.find("if ").unwrap_or(line.find("else ").unwrap_or(0)) + 2) as u32,
                        },
                    },
                    severity: Some(DiagnosticSeverity::WARNING),
                    code: Some(NumberOrString::String("CURSED_DEPRECATED_KEYWORD".to_string())),
                    source: Some("cursed-lsp".to_string()),
                    message: "Use 'lowkey' instead of 'if' and 'otherwise' instead of 'else' in CURSED".to_string(),
                    related_information: None,
                    tags: Some(vec![DiagnosticTag::DEPRECATED]),
                    data: None,
                    code_description: None,
                });
            }

            // Check for missing semicolons
            let trimmed = line.trim();
            if !trimmed.is_empty() && 
               !trimmed.ends_with('{') && 
               !trimmed.ends_with('}') && 
               !trimmed.ends_with(';') &&
               !trimmed.starts_with("//") &&
               !trimmed.contains("lowkey") &&
               !trimmed.contains("otherwise") {
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position { 
                            line: line_num as u32, 
                            character: line.len() as u32,
                        },
                        end: Position { 
                            line: line_num as u32, 
                            character: line.len() as u32,
                        },
                    },
                    severity: Some(DiagnosticSeverity::INFORMATION),
                    code: Some(NumberOrString::String("CURSED_MISSING_SEMICOLON".to_string())),
                    source: Some("cursed-lsp".to_string()),
                    message: "Consider adding a semicolon".to_string(),
                    related_information: None,
                    tags: None,
                    data: None,
                    code_description: None,
                });
            }

            // Check for CURSED style guide violations
            if line.contains("function") {
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position { 
                            line: line_num as u32, 
                            character: line.find("function").unwrap() as u32,
                        },
                        end: Position { 
                            line: line_num as u32, 
                            character: (line.find("function").unwrap() + 8) as u32,
                        },
                    },
                    severity: Some(DiagnosticSeverity::WARNING),
                    code: Some(NumberOrString::String("CURSED_STYLE_VIOLATION".to_string())),
                    source: Some("cursed-lsp".to_string()),
                    message: "Use 'slay' instead of 'function' in CURSED".to_string(),
                    related_information: None,
                    tags: None,
                    data: None,
                    code_description: None,
                });
            }

            // Check for uninitialized variables
            if line.contains("sus ") && !line.contains("=") {
                if let Some(var_start) = line.find("sus ") {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position { 
                                line: line_num as u32, 
                                character: var_start as u32,
                            },
                            end: Position { 
                                line: line_num as u32, 
                                character: line.len() as u32,
                            },
                        },
                        severity: Some(DiagnosticSeverity::WARNING),
                        code: Some(NumberOrString::String("CURSED_UNINITIALIZED_VAR".to_string())),
                        source: Some("cursed-lsp".to_string()),
                        message: "Variable declared without initialization".to_string(),
                        related_information: None,
                        tags: None,
                        data: None,
                        code_description: None,
                    });
                }
            }
        }

        diagnostics
    }

    /// Get position information from byte offset
    fn get_error_position(&self, position: usize, text: &str) -> (u32, u32, u32) {
        let mut line = 0;
        let mut character = 0;
        let mut current_pos = 0;

        for ch in text.chars() {
            if current_pos >= position {
                break;
            }
            
            if ch == '\n' {
                line += 1;
                character = 0;
            } else {
                character += 1;
            }
            
            current_pos += ch.len_utf8();
        }

        // Estimate end character (for highlighting)
        let end_character = character + 5; // Highlight a few characters
        
        (line, character, end_character)
    }

    /// Get cached diagnostics for a document
    pub fn get_diagnostics(&self, uri: &Url) -> Option<&Vec<Diagnostic>> {
        self.diagnostics_cache.get(uri)
    }

    /// Clear diagnostics for a document
    pub fn clear_diagnostics(&mut self, uri: &Url) {
        self.diagnostics_cache.remove(uri);
    }

    /// Clear all diagnostics
    pub fn clear_all_diagnostics(&mut self) {
        self.diagnostics_cache.clear();
    }

    /// Check if document has errors
    pub fn has_errors(&self, uri: &Url) -> bool {
        if let Some(diagnostics) = self.diagnostics_cache.get(uri) {
            diagnostics.iter().any(|d| d.severity == Some(DiagnosticSeverity::ERROR))
        } else {
            false
        }
    }

    /// Get error count for document
    pub fn get_error_count(&self, uri: &Url) -> usize {
        if let Some(diagnostics) = self.diagnostics_cache.get(uri) {
            diagnostics.iter()
                .filter(|d| d.severity == Some(DiagnosticSeverity::ERROR))
                .count()
        } else {
            0
        }
    }

    /// Get warning count for document
    pub fn get_warning_count(&self, uri: &Url) -> usize {
        if let Some(diagnostics) = self.diagnostics_cache.get(uri) {
            diagnostics.iter()
                .filter(|d| d.severity == Some(DiagnosticSeverity::WARNING))
                .count()
        } else {
            0
        }
    }
}
