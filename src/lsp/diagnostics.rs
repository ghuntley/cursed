//! Diagnostics provider for CURSED language server
//! 
//! Provides syntax errors, type errors, warnings, and linting diagnostics

use std::collections::HashMap;
use tower_lsp::lsp_types::*;
use tracing::{debug, error, instrument, warn};

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::error::CursedError;

/// Diagnostics provider for the LSP server
pub struct DiagnosticsProvider {
    /// Cached diagnostics to avoid recomputation
    diagnostic_cache: std::sync::RwLock<HashMap<String, Vec<Diagnostic>>>,
}

impl DiagnosticsProvider {
    /// Create a new diagnostics provider
    pub fn new() -> Self {
        Self {
            diagnostic_cache: std::sync::RwLock::new(HashMap::new()),
        }
    }

    /// Get syntax diagnostics from lexer and parser
    #[instrument(skip(self, content))]
    pub async fn get_syntax_diagnostics(&self, content: &str) -> Vec<Diagnostic> {
        debug!("Getting syntax diagnostics");
        
        let mut diagnostics = Vec::new();
        
        // Check lexer errors
        match self.analyze_lexer_errors(content) {
            Ok(lexer_diagnostics) => diagnostics.extend(lexer_diagnostics),
            Err(err) => {
                error!("Lexer analysis failed: {}", err);
                diagnostics.push(self.create_diagnostic(
                    Range {
                        start: Position { line: 0, character: 0 },
                        end: Position { line: 0, character: content.len() as u32 },
                    },
                    DiagnosticSeverity::ERROR,
                    "Lexer analysis failed".to_string(),
                    Some("lexer".to_string()),
                ));
            }
        }

        // Check parser errors
        match self.analyze_parser_errors(content) {
            Ok(parser_diagnostics) => diagnostics.extend(parser_diagnostics),
            Err(err) => {
                error!("Parser analysis failed: {}", err);
                diagnostics.push(self.create_diagnostic(
                    Range {
                        start: Position { line: 0, character: 0 },
                        end: Position { line: 0, character: content.len() as u32 },
                    },
                    DiagnosticSeverity::ERROR,
                    "Parser analysis failed".to_string(),
                    Some("parser".to_string()),
                ));
            }
        }

        diagnostics
    }

    /// Get semantic diagnostics (type checking, etc.)
    #[instrument(skip(self, content))]
    pub async fn get_semantic_diagnostics(&self, content: &str) -> Vec<Diagnostic> {
        debug!("Getting semantic diagnostics");
        
        let mut diagnostics = Vec::new();

        // Type checking diagnostics
        diagnostics.extend(self.check_type_errors(content));
        
        // Variable usage diagnostics
        diagnostics.extend(self.check_variable_usage(content));
        
        // Function call diagnostics
        diagnostics.extend(self.check_function_calls(content));
        
        // Import/module diagnostics
        diagnostics.extend(self.check_imports(content));

        diagnostics
    }

    /// Get linting diagnostics (style, best practices, etc.)
    #[instrument(skip(self, content))]
    pub async fn get_lint_diagnostics(&self, content: &str) -> Vec<Diagnostic> {
        debug!("Getting lint diagnostics");
        
        let mut diagnostics = Vec::new();

        // Style diagnostics
        diagnostics.extend(self.check_style_issues(content));
        
        // Best practice diagnostics
        diagnostics.extend(self.check_best_practices(content));
        
        // Performance diagnostics
        diagnostics.extend(self.check_performance_issues(content));
        
        // Security diagnostics
        diagnostics.extend(self.check_security_issues(content));

        diagnostics
    }

    /// Analyze lexer errors
    fn analyze_lexer_errors(&self, content: &str) -> Result<Vec<Diagnostic>, Box<dyn std::error::Error>> {
        let mut diagnostics = Vec::new();
        let mut lexer = Lexer::new(content.to_string());
        
        loop {
            match lexer.next_token() {
                Ok(token) => {
                    if token.token_type == crate::lexer::TokenType::Eof {
                        break;
                    }
                    // Check for specific CURSED keywords and validate them
                    if self.is_invalid_slang_usage(&token) {
                        diagnostics.push(self.create_diagnostic(
                            Range {
                                start: Position { 
                                    line: token.location.line as u32 - 1, 
                                    character: token.location.column as u32 - 1 
                                },
                                end: Position { 
                                    line: token.location.line as u32 - 1, 
                                    character: (token.location.column + 10) as u32 - 1 
                                },
                            },
                            DiagnosticSeverity::WARNING,
                            format!("Consider using proper CURSED slang: '{:?}'", token.token_type),
                            Some("cursed-style".to_string()),
                        ));
                    }
                }
                Err(err) => {
                    // Convert lexer error to diagnostic
                    let (line, column) = self.get_error_position(&err, content);
                    diagnostics.push(self.create_diagnostic(
                        Range {
                            start: Position { line, character: column },
                            end: Position { line, character: column + 1 },
                        },
                        DiagnosticSeverity::ERROR,
                        format!("Lexer error: {}", err),
                        Some("lexer".to_string()),
                    ));
                    break;
                }
            }
        }

        Ok(diagnostics)
    }

    /// Analyze parser errors
    fn analyze_parser_errors(&self, content: &str) -> Result<Vec<Diagnostic>, Box<dyn std::error::Error>> {
        let mut diagnostics = Vec::new();
        let lexer = Lexer::new(content.to_string());
        let mut parser = match Parser::new(lexer) {
            Ok(parser) => parser,
            Err(err) => {
                // Failed to create parser
                let diagnostic = Diagnostic::new_simple(
                    Range::new(Position::new(0, 0), Position::new(0, 0)),
                    format!("Failed to create parser: {}", err),
                );
                diagnostics.push(diagnostic);
                return Ok(diagnostics);
            }
        };
        
        match parser.parse_program() {
            Ok(_ast) => {
                // Parser succeeded, no syntax errors
            }
            Err(err) => {
                let (line, column) = self.get_error_position(&err, content);
                diagnostics.push(self.create_diagnostic(
                    Range {
                        start: Position { line, character: column },
                        end: Position { line, character: column + 10 }, // Approximate error span
                    },
                    DiagnosticSeverity::ERROR,
                    format!("Parse error: {}", err),
                    Some("parser".to_string()),
                ));
            }
        }

        Ok(diagnostics)
    }

    /// Check for type errors
    fn check_type_errors(&self, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        
        // Basic type checking patterns
        let lines: Vec<&str> = content.lines().collect();
        for (line_num, line) in lines.iter().enumerate() {
            // Check for type mismatches in variable assignments
            if line.contains("facts") && line.contains("=") {
                if let Some(diagnostic) = self.check_variable_type_assignment(line, line_num) {
                    diagnostics.push(diagnostic);
                }
            }
            
            // Check function return types
            if line.contains("slay") && line.contains("->") {
                if let Some(diagnostic) = self.check_function_return_type(line, line_num) {
                    diagnostics.push(diagnostic);
                }
            }
        }

        diagnostics
    }

    /// Check variable usage issues
    fn check_variable_usage(&self, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        for (line_num, line) in lines.iter().enumerate() {
            // Check for unused variables
            if line.contains("facts") && !self.is_variable_used(line, &lines) {
                if let Some(var_name) = self.extract_variable_name(line) {
                    diagnostics.push(self.create_diagnostic(
                        Range {
                            start: Position { line: line_num as u32, character: 0 },
                            end: Position { line: line_num as u32, character: line.len() as u32 },
                        },
                        DiagnosticSeverity::WARNING,
                        format!("Variable '{}' is declared but never used", var_name),
                        Some("unused-variable".to_string()),
                    ));
                }
            }
        }

        diagnostics
    }

    /// Check function call issues
    fn check_function_calls(&self, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        for (line_num, line) in lines.iter().enumerate() {
            // Check for undefined function calls
            if let Some(func_call) = self.extract_function_call(line) {
                if !self.is_function_defined(&func_call, &lines) {
                    diagnostics.push(self.create_diagnostic(
                        Range {
                            start: Position { line: line_num as u32, character: 0 },
                            end: Position { line: line_num as u32, character: line.len() as u32 },
                        },
                        DiagnosticSeverity::ERROR,
                        format!("Function '{}' is not defined", func_call),
                        Some("undefined-function".to_string()),
                    ));
                }
            }
        }

        diagnostics
    }

    /// Check import issues
    fn check_imports(&self, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        for (line_num, line) in lines.iter().enumerate() {
            if line.trim().starts_with("use") || line.trim().starts_with("import") {
                // Check for invalid import paths
                if let Some(import_path) = self.extract_import_path(line) {
                    if !self.is_valid_import_path(&import_path) {
                        diagnostics.push(self.create_diagnostic(
                            Range {
                                start: Position { line: line_num as u32, character: 0 },
                                end: Position { line: line_num as u32, character: line.len() as u32 },
                            },
                            DiagnosticSeverity::ERROR,
                            format!("Invalid import path: '{}'", import_path),
                            Some("invalid-import".to_string()),
                        ));
                    }
                }
            }
        }

        diagnostics
    }

    /// Check style issues
    fn check_style_issues(&self, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        for (line_num, line) in lines.iter().enumerate() {
            // Check for proper CURSED slang usage
            if line.contains("function") && !line.contains("slay") {
                diagnostics.push(self.create_diagnostic(
                    Range {
                        start: Position { line: line_num as u32, character: 0 },
                        end: Position { line: line_num as u32, character: line.len() as u32 },
                    },
                    DiagnosticSeverity::INFORMATION,
                    "Consider using 'slay' instead of 'function' for proper CURSED style".to_string(),
                    Some("cursed-style".to_string()),
                ));
            }
            
            // Check for variable declaration style
            if line.contains("var") && !line.contains("facts") && !line.contains("sus") {
                diagnostics.push(self.create_diagnostic(
                    Range {
                        start: Position { line: line_num as u32, character: 0 },
                        end: Position { line: line_num as u32, character: line.len() as u32 },
                    },
                    DiagnosticSeverity::INFORMATION,
                    "Consider using 'facts' or 'sus' instead of 'var' for proper CURSED style".to_string(),
                    Some("cursed-style".to_string()),
                ));
            }
        }

        diagnostics
    }

    /// Check best practice issues
    fn check_best_practices(&self, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        for (line_num, line) in lines.iter().enumerate() {
            // Check for magic numbers
            if self.contains_magic_number(line) {
                diagnostics.push(self.create_diagnostic(
                    Range {
                        start: Position { line: line_num as u32, character: 0 },
                        end: Position { line: line_num as u32, character: line.len() as u32 },
                    },
                    DiagnosticSeverity::HINT,
                    "Consider using a named constant instead of a magic number".to_string(),
                    Some("best-practice".to_string()),
                ));
            }
            
            // Check for long lines
            if line.len() > 120 {
                diagnostics.push(self.create_diagnostic(
                    Range {
                        start: Position { line: line_num as u32, character: 120 },
                        end: Position { line: line_num as u32, character: line.len() as u32 },
                    },
                    DiagnosticSeverity::HINT,
                    "Line is too long (>120 characters)".to_string(),
                    Some("line-length".to_string()),
                ));
            }
        }

        diagnostics
    }

    /// Check performance issues
    fn check_performance_issues(&self, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        for (line_num, line) in lines.iter().enumerate() {
            // Check for inefficient string concatenation in loops
            if line.contains("for") || line.contains("while") {
                // Look ahead for string concatenation
                for (offset, next_line) in lines.iter().enumerate().skip(line_num + 1).take(10) {
                    if next_line.contains("+") && next_line.contains("\"") {
                        diagnostics.push(self.create_diagnostic(
                            Range {
                                start: Position { line: (line_num + offset) as u32, character: 0 },
                                end: Position { line: (line_num + offset) as u32, character: next_line.len() as u32 },
                            },
                            DiagnosticSeverity::HINT,
                            "Consider using a string builder for better performance in loops".to_string(),
                            Some("performance".to_string()),
                        ));
                        break;
                    }
                }
            }
        }

        diagnostics
    }

    /// Check security issues
    fn check_security_issues(&self, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        for (line_num, line) in lines.iter().enumerate() {
            // Check for potential security issues
            if line.contains("eval") || line.contains("exec") {
                diagnostics.push(self.create_diagnostic(
                    Range {
                        start: Position { line: line_num as u32, character: 0 },
                        end: Position { line: line_num as u32, character: line.len() as u32 },
                    },
                    DiagnosticSeverity::WARNING,
                    "Use of eval/exec functions can be a security risk".to_string(),
                    Some("security".to_string()),
                ));
            }
        }

        diagnostics
    }

    /// Create a diagnostic
    fn create_diagnostic(
        &self,
        range: Range,
        severity: DiagnosticSeverity,
        message: String,
        source: Option<String>,
    ) -> Diagnostic {
        Diagnostic {
            range,
            severity: Some(severity),
            code: None,
            code_description: None,
            source,
            message,
            related_information: None,
            tags: None,
            data: None,
        }
    }

    /// Helper functions for analysis

    fn is_invalid_slang_usage(&self, token: &crate::lexer::Token) -> bool {
        // Check if token should use CURSED slang but doesn't
        matches!(token.token_type, 
                 crate::lexer::TokenType::Slay | 
                 crate::lexer::TokenType::Sus |
                 crate::lexer::TokenType::Facts)
    }

    fn get_error_position(&self, error: &CursedError, _content: &str) -> (u32, u32) {
        // Extract position from error if available, otherwise default to (0, 0)
        (0, 0)
    }

    fn check_variable_type_assignment(&self, line: &str, line_num: usize) -> Option<Diagnostic> {
        // Basic type checking - this would be more sophisticated in a real implementation
        if line.contains("= \"") && line.contains(": int") {
            return Some(self.create_diagnostic(
                Range {
                    start: Position { line: line_num as u32, character: 0 },
                    end: Position { line: line_num as u32, character: line.len() as u32 },
                },
                DiagnosticSeverity::ERROR,
                "Type mismatch: cannot assign string to int variable".to_string(),
                Some("type-error".to_string()),
            ));
        }
        None
    }

    fn check_function_return_type(&self, line: &str, line_num: usize) -> Option<Diagnostic> {
        // Check if function return type matches actual return
        if line.contains("-> string") && line.contains("return 42") {
            return Some(self.create_diagnostic(
                Range {
                    start: Position { line: line_num as u32, character: 0 },
                    end: Position { line: line_num as u32, character: line.len() as u32 },
                },
                DiagnosticSeverity::ERROR,
                "Return type mismatch: expected string, got number".to_string(),
                Some("type-error".to_string()),
            ));
        }
        None
    }

    fn is_variable_used(&self, declaration_line: &str, all_lines: &[&str]) -> bool {
        if let Some(var_name) = self.extract_variable_name(declaration_line) {
            all_lines.iter().any(|line| line != &declaration_line && line.contains(&var_name))
        } else {
            true // Assume used if we can't extract name
        }
    }

    fn extract_variable_name(&self, line: &str) -> Option<String> {
        // Extract variable name from declaration line
        if let Some(facts_pos) = line.find("facts") {
            let after_facts = &line[facts_pos + 5..];
            if let Some(equals_pos) = after_facts.find('=') {
                let var_part = &after_facts[..equals_pos].trim();
                return Some(var_part.to_string());
            }
        }
        None
    }

    fn extract_function_call(&self, line: &str) -> Option<String> {
        // Extract function name from function call
        if let Some(paren_pos) = line.find('(') {
            let before_paren = &line[..paren_pos];
            if let Some(space_pos) = before_paren.rfind(' ') {
                return Some(before_paren[space_pos + 1..].to_string());
            } else {
                return Some(before_paren.to_string());
            }
        }
        None
    }

    fn is_function_defined(&self, func_name: &str, all_lines: &[&str]) -> bool {
        // Check if function is defined anywhere
        all_lines.iter().any(|line| {
            line.contains("slay") && line.contains(func_name) && line.contains('(')
        }) || self.is_builtin_function(func_name)
    }

    fn is_builtin_function(&self, func_name: &str) -> bool {
        // List of built-in CURSED functions
        matches!(func_name, "print" | "println" | "len" | "str" | "int" | "float")
    }

    fn extract_import_path(&self, line: &str) -> Option<String> {
        // Extract import path from import statement
        if let Some(quote_start) = line.find('"') {
            if let Some(quote_end) = line[quote_start + 1..].find('"') {
                return Some(line[quote_start + 1..quote_start + 1 + quote_end].to_string());
            }
        }
        None
    }

    fn is_valid_import_path(&self, _path: &str) -> bool {
        // Basic import path validation
        // In a real implementation, this would check if the module exists
        true
    }

    fn contains_magic_number(&self, line: &str) -> bool {
        // Check for magic numbers (not 0, 1, or obvious constants)
        let numbers = regex::Regex::new(r"\b\d+\b").unwrap();
        for number_match in numbers.find_iter(line) {
            let number = number_match.as_str();
            if !matches!(number, "0" | "1" | "2" | "10" | "100" | "1000") {
                return true;
            }
        }
        false
    }
}

impl Default for DiagnosticsProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_syntax_diagnostics() {
        let provider = DiagnosticsProvider::new();
        let content = "slay main() {\n    facts x = 42\n    // Missing closing brace";
        
        let diagnostics = provider.get_syntax_diagnostics(content).await;
        // Should have at least one diagnostic for syntax error
        assert!(!diagnostics.is_empty());
    }

    #[tokio::test]
    async fn test_style_diagnostics() {
        let provider = DiagnosticsProvider::new();
        let content = "function main() {\n    var x = 42\n}"; // Using non-CURSED style
        
        let diagnostics = provider.get_lint_diagnostics(content).await;
        // Should suggest using CURSED slang
        assert!(!diagnostics.is_empty());
        assert!(diagnostics.iter().any(|d| d.source == Some("cursed-style".to_string())));
    }

    #[tokio::test]
    async fn test_unused_variable_detection() {
        let provider = DiagnosticsProvider::new();
        let content = "slay main() {\n    facts unused_var = 42\n    print(\"hello\")\n}";
        
        let diagnostics = provider.get_semantic_diagnostics(content).await;
        // Should detect unused variable
        assert!(diagnostics.iter().any(|d| d.message.contains("never used")));
    }
}
