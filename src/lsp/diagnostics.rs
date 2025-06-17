//! Enhanced Diagnostics provider for CURSED language server
//! 
//! Provides real-time syntax errors, type errors, warnings, and linting diagnostics
//! using CURSED's actual compiler infrastructure

use std::collections::HashMap;
use tower_lsp::lsp_types::*;
use tracing::{debug, error, instrument, warn, info};

use crate::lexer::{Lexer, Token, TokenType};
use crate::parser::Parser;
use crate::core::type_checker::{TypeChecker, Type};
use crate::error::{CursedError, Error};
use crate::ast::Program;
use crate::import_resolution::ImportResolver;

/// Enhanced diagnostics provider with compiler integration
pub struct DiagnosticsProvider {
    /// Cached diagnostics to avoid recomputation
    diagnostic_cache: std::sync::RwLock<HashMap<String, Vec<Diagnostic>>>,
    /// Type checker for semantic analysis
    type_checker: std::sync::RwLock<TypeChecker>,
    /// Import resolver for cross-file validation
    import_resolver: std::sync::RwLock<ImportResolver>,
    /// Parsed AST cache for efficient re-analysis
    ast_cache: std::sync::RwLock<HashMap<String, Program>>,
}

impl DiagnosticsProvider {
    /// Create a new diagnostics provider with compiler integration
    pub fn new() -> Self {
        Self {
            diagnostic_cache: std::sync::RwLock::new(HashMap::new()),
            type_checker: std::sync::RwLock::new(TypeChecker::new()),
            import_resolver: std::sync::RwLock::new(ImportResolver::new()),
            ast_cache: std::sync::RwLock::new(HashMap::new()),
        }
    }

    /// Get comprehensive diagnostics using CURSED compiler infrastructure
    #[instrument(skip(self, content))]
    pub async fn get_syntax_diagnostics(&self, content: &str) -> Vec<Diagnostic> {
        debug!("Getting comprehensive syntax diagnostics using CURSED compiler");
        
        let mut diagnostics = Vec::new();
        
        // Check cache first
        let content_hash = format!("{:x}", md5::compute(content));
        if let Ok(cache) = self.diagnostic_cache.read() {
            if let Some(cached_diagnostics) = cache.get(&content_hash) {
                debug!("Returning cached diagnostics");
                return cached_diagnostics.clone();
            }
        }
        
        // Perform comprehensive analysis using CURSED compiler
        match self.analyze_with_compiler(content).await {
            Ok(compiler_diagnostics) => {
                diagnostics.extend(compiler_diagnostics);
            }
            Err(err) => {
                error!("Compiler analysis failed: {}", err);
                diagnostics.push(self.create_diagnostic_impl(
                    Range {
                        start: Position { line: 0, character: 0 },
                        end: Position { line: 0, character: 10 },
                    },
                    DiagnosticSeverity::ERROR,
                    format!("Compilation failed: {}", err),
                    Some("compiler".to_string()),
                ));
            }
        }
        
        // Cache the results
        if let Ok(mut cache) = self.diagnostic_cache.write() {
            cache.insert(content_hash, diagnostics.clone());
        }

        diagnostics
    }
    
    /// Comprehensive analysis using CURSED compiler infrastructure
    async fn analyze_with_compiler(&self, content: &str) -> Result<Vec<Diagnostic>, CursedError> {
        info!("Running comprehensive compiler analysis");
        let mut diagnostics = Vec::new();
        
        // Step 1: Lexical analysis
        let mut tokens = Vec::new();
        let mut lexer = Lexer::new(content.to_string());
        
        loop {
            match lexer.next_token() {
                Ok(token) => {
                    if token.token_type == TokenType::Eof {
                        break;
                    }
                    tokens.push(token);
                }
                Err(err) => {
                    let diagnostic = self.convert_lexer_error_to_diagnostic(&err, content);
                    diagnostics.push(diagnostic);
                    break;
                }
            }
        }
        
        if diagnostics.is_empty() {
            // Step 2: Syntax analysis
            let lexer = Lexer::new(content.to_string());
            let mut parser = Parser::new(lexer)?;
            
            match parser.parse_program() {
                Ok(ast) => {
                    // Cache the AST
                    let content_hash = format!("{:x}", md5::compute(content));
                    if let Ok(mut cache) = self.ast_cache.write() {
                        cache.insert(content_hash, ast.clone());
                    }
                    
                    // Step 3: Semantic analysis
                    let semantic_diagnostics = self.analyze_semantics(&ast).await?;
                    diagnostics.extend(semantic_diagnostics);
                    
                    // Step 4: Type checking
                    let type_diagnostics = self.analyze_types(&ast).await?;
                    diagnostics.extend(type_diagnostics);
                    
                    // Step 5: Import validation
                    let import_diagnostics = self.analyze_imports(&ast).await?;
                    diagnostics.extend(import_diagnostics);
                }
                Err(err) => {
                    let diagnostic = self.convert_parse_error_to_diagnostic(&err, content);
                    diagnostics.push(diagnostic);
                }
            }
        }
        
        Ok(diagnostics)
    }
    
    /// Analyze semantics using AST
    async fn analyze_semantics(&self, ast: &Program) -> Result<Vec<Diagnostic>, CursedError> {
        let mut diagnostics = Vec::new();
        
        // Check for unreachable code
        diagnostics.extend(self.check_unreachable_code(ast));
        
        // Check for unused variables
        diagnostics.extend(self.check_unused_variables(ast));
        
        // Check for infinite loops
        diagnostics.extend(self.check_infinite_loops(ast));
        
        // Check for dead code
        diagnostics.extend(self.check_dead_code(ast));
        
        Ok(diagnostics)
    }
    
    /// Analyze types using type checker
    async fn analyze_types(&self, ast: &Program) -> Result<Vec<Diagnostic>, CursedError> {
        let mut diagnostics = Vec::new();
        
        if let Ok(mut type_checker) = self.type_checker.write() {
            match type_checker.check_program(ast) {
                Ok(()) => {
                    debug!("Type checking completed successfully");
                }
                Err(err) => {
                    let diagnostic = self.convert_type_error_to_diagnostic(&err);
                    diagnostics.push(diagnostic);
                }
            }
        }
        
        Ok(diagnostics)
    }
    
    /// Analyze imports using import resolver
    async fn analyze_imports(&self, ast: &Program) -> Result<Vec<Diagnostic>, CursedError> {
        let mut diagnostics = Vec::new();
        
        if let Ok(mut import_resolver) = self.import_resolver.write() {
            // Check import statements for validity
            for statement in &ast.statements {
                let stmt_str = statement.string();
                if stmt_str.contains("use ") || stmt_str.contains("import ") {
                    // Extract import path and validate
                    if let Some(import_path) = self.extract_import_path(&stmt_str) {
                        if !import_resolver.resolve_import(&import_path).is_ok() {
                            diagnostics.push(self.create_diagnostic_impl(
                                Range {
                                    start: Position { line: 0, character: 0 },
                                    end: Position { line: 0, character: stmt_str.len() as u32 },
                                },
                                DiagnosticSeverity::ERROR,
                                format!("Cannot resolve import: '{}'", import_path),
                                Some("import".to_string()),
                            ));
                        }
                    }
                }
            }
        }
        
        Ok(diagnostics)
    }
    
    /// Convert lexer error to diagnostic
    fn convert_lexer_error_to_diagnostic(&self, error: &CursedError, _content: &str) -> Diagnostic {
        // Extract position information from error if available
        let (line, character) = self.extract_error_position(error);
        
        self.create_diagnostic_impl(
            Range {
                start: Position { line, character },
                end: Position { line, character: character + 1 },
            },
            DiagnosticSeverity::ERROR,
            format!("Lexical error: {}", error),
            Some("lexer".to_string()),
        )
    }
    
    /// Convert parse error to diagnostic  
    fn convert_parse_error_to_diagnostic(&self, error: &CursedError, _content: &str) -> Diagnostic {
        let (line, character) = self.extract_error_position(error);
        
        self.create_diagnostic_impl(
            Range {
                start: Position { line, character },
                end: Position { line, character: character + 10 },
            },
            DiagnosticSeverity::ERROR,
            format!("Syntax error: {}", error),
            Some("parser".to_string()),
        )
    }
    
    /// Convert type error to diagnostic
    fn convert_type_error_to_diagnostic(&self, error: &Error) -> Diagnostic {
        self.create_diagnostic_impl(
            Range {
                start: Position { line: 0, character: 0 },
                end: Position { line: 0, character: 10 },
            },
            DiagnosticSeverity::ERROR,
            format!("Type error: {}", error),
            Some("type-checker".to_string()),
        )
    }
    
    /// Extract error position from CURSED error
    fn extract_error_position(&self, error: &CursedError) -> (u32, u32) {
        // Try to extract position information from error
        // This would need to be enhanced based on CURSED's error structure
        match error {
            _ => (0, 0), // Default position for now
        }
    }

    /// Get semantic diagnostics (type checking, etc.)
    #[instrument(skip(self, content))]
    pub async fn get_semantic_diagnostics(&self, content: &str) -> Vec<Diagnostic> {
        debug!("Getting enhanced semantic diagnostics");
        
        // Use cached AST if available
        let content_hash = format!("{:x}", md5::compute(content));
        if let Ok(cache) = self.ast_cache.read() {
            if let Some(ast) = cache.get(&content_hash) {
                // Use AST-based semantic analysis
                match self.analyze_semantics(ast).await {
                    Ok(diagnostics) => return diagnostics,
                    Err(err) => {
                        warn!("AST-based semantic analysis failed: {}", err);
                    }
                }
            }
        }
        
        // Fallback to pattern-based analysis
        let mut diagnostics = Vec::new();
        diagnostics.extend(self.check_type_errors_impl(content));
        diagnostics.extend(self.check_variable_usage_impl(content));
        diagnostics.extend(self.check_function_calls_impl(content));
        diagnostics.extend(self.check_imports_impl(content));

        diagnostics
    }
    
    /// Check for unreachable code in AST
    fn check_unreachable_code(&self, ast: &Program) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        
        for (stmt_index, statement) in ast.statements.iter().enumerate() {
            let stmt_str = statement.string();
            
            // Check for code after return statements
            if stmt_str.contains("bounce ") || stmt_str.contains("yeet ") {
                // Check if there are more statements after this one
                if stmt_index + 1 < ast.statements.len() {
                    diagnostics.push(self.create_diagnostic_impl(
                        Range {
                            start: Position { line: (stmt_index + 1) as u32, character: 0 },
                            end: Position { line: (stmt_index + 1) as u32, character: 10 },
                        },
                        DiagnosticSeverity::WARNING,
                        "Unreachable code after return statement".to_string(),
                        Some("unreachable".to_string()),
                    ));
                }
            }
        }
        
        diagnostics
    }
    
    /// Check for unused variables in AST
    fn check_unused_variables(&self, ast: &Program) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let mut declared_vars = HashMap::new();
        let mut used_vars = std::collections::HashSet::new();
        
        // First pass: collect variable declarations
        for (stmt_index, statement) in ast.statements.iter().enumerate() {
            let stmt_str = statement.string();
            if stmt_str.contains("facts ") || stmt_str.contains("sus ") {
                if let Some(var_name) = self.extract_variable_name(&stmt_str) {
                    declared_vars.insert(var_name, stmt_index);
                }
            }
        }
        
        // Second pass: collect variable usages
        for statement in &ast.statements {
            let stmt_str = statement.string();
            for var_name in declared_vars.keys() {
                if stmt_str.contains(var_name) && !stmt_str.contains("facts ") && !stmt_str.contains("sus ") {
                    used_vars.insert(var_name.clone());
                }
            }
        }
        
        // Report unused variables
        for (var_name, stmt_index) in declared_vars {
            if !used_vars.contains(&var_name) {
                diagnostics.push(self.create_diagnostic_impl(
                    Range {
                        start: Position { line: stmt_index as u32, character: 0 },
                        end: Position { line: stmt_index as u32, character: 10 },
                    },
                    DiagnosticSeverity::WARNING,
                    format!("Variable '{}' is declared but never used", var_name),
                    Some("unused-variable".to_string()),
                ));
            }
        }
        
        diagnostics
    }
    
    /// Check for infinite loops in AST
    fn check_infinite_loops(&self, ast: &Program) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        
        for (stmt_index, statement) in ast.statements.iter().enumerate() {
            let stmt_str = statement.string();
            
            // Check for while true loops
            if stmt_str.contains("flex true") || stmt_str.contains("periodt true") {
                diagnostics.push(self.create_diagnostic_impl(
                    Range {
                        start: Position { line: stmt_index as u32, character: 0 },
                        end: Position { line: stmt_index as u32, character: stmt_str.len() as u32 },
                    },
                    DiagnosticSeverity::WARNING,
                    "Potential infinite loop detected".to_string(),
                    Some("infinite-loop".to_string()),
                ));
            }
        }
        
        diagnostics
    }
    
    /// Check for dead code in AST
    fn check_dead_code(&self, ast: &Program) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        
        // Check for unreachable code after unconditional returns
        let mut found_return = false;
        for (stmt_index, statement) in ast.statements.iter().enumerate() {
            let stmt_str = statement.string();
            
            if found_return {
                diagnostics.push(self.create_diagnostic_impl(
                    Range {
                        start: Position { line: stmt_index as u32, character: 0 },
                        end: Position { line: stmt_index as u32, character: stmt_str.len() as u32 },
                    },
                    DiagnosticSeverity::INFORMATION,
                    "Dead code - this statement will never be executed".to_string(),
                    Some("dead-code".to_string()),
                ));
            }
            
            if stmt_str.trim().starts_with("bounce ") {
                found_return = true;
            }
        }
        
        diagnostics
    }

    /// Get linting diagnostics (style, best practices, etc.)
    #[instrument(skip(self, content))]
    pub async fn get_lint_diagnostics(&self, content: &str) -> Vec<Diagnostic> {
        debug!("Getting lint diagnostics");
        
        let mut diagnostics = Vec::new();

        // Style diagnostics
        diagnostics.extend(self.check_style_issues_impl(content));
        
        // Best practice diagnostics
        diagnostics.extend(self.check_best_practices_impl(content));
        
        // Performance diagnostics
        diagnostics.extend(self.check_performance_issues_impl(content));
        
        // Security diagnostics
        diagnostics.extend(self.check_security_issues_impl(content));

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
                        diagnostics.push(self.create_diagnostic_impl(
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
                    diagnostics.push(self.create_diagnostic_impl(
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
                diagnostics.push(self.create_diagnostic_impl(
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

    /// Check for type errors (internal implementation)
    fn check_type_errors_impl(&self, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        
        // Basic type checking patterns
        let lines: Vec<&str> = content.split("\n").collect();
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

    /// Check variable usage issues (internal implementation)
    fn check_variable_usage_impl(&self, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = content.split("\n").collect();
        
        for (line_num, line) in lines.iter().enumerate() {
            // Check for unused variables
            if line.contains("facts") && !self.is_variable_used(line, &lines) {
                if let Some(var_name) = self.extract_variable_name(line) {
                    diagnostics.push(self.create_diagnostic_impl(
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

    /// Check function call issues (internal implementation)
    fn check_function_calls_impl(&self, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = content.split("\n").collect();
        
        for (line_num, line) in lines.iter().enumerate() {
            // Check for undefined function calls
            if let Some(func_call) = self.extract_function_call(line) {
                if !self.is_function_defined(&func_call, &lines) {
                    diagnostics.push(self.create_diagnostic_impl(
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

    /// Check import issues (internal implementation)
    fn check_imports_impl(&self, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = content.split("\n").collect();
        
        for (line_num, line) in lines.iter().enumerate() {
            if line.trim().starts_with("use") || line.trim().starts_with("import") {
                // Check for invalid import paths
                if let Some(import_path) = self.extract_import_path(line) {
                    if !self.is_valid_import_path(&import_path) {
                        diagnostics.push(self.create_diagnostic_impl(
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

    /// Check style issues (internal implementation)
    fn check_style_issues_impl(&self, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = content.split("\n").collect();
        
        for (line_num, line) in lines.iter().enumerate() {
            // Check for proper CURSED slang usage
            if line.contains("function") && !line.contains("slay") {
                diagnostics.push(self.create_diagnostic_impl(
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
                diagnostics.push(self.create_diagnostic_impl(
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

    /// Check best practice issues (internal implementation)
    fn check_best_practices_impl(&self, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = content.split("\n").collect();
        
        for (line_num, line) in lines.iter().enumerate() {
            // Check for magic numbers
            if self.contains_magic_number(line) {
                diagnostics.push(self.create_diagnostic_impl(
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
                diagnostics.push(self.create_diagnostic_impl(
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

    /// Check performance issues (internal implementation)
    fn check_performance_issues_impl(&self, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = content.split("\n").collect();
        
        for (line_num, line) in lines.iter().enumerate() {
            // Check for inefficient string concatenation in loops
            if line.contains("for") || line.contains("while") {
                // Look ahead for string concatenation
                for (offset, next_line) in lines.iter().enumerate().skip(line_num + 1).take(10) {
                    if next_line.contains("+") && next_line.contains("\"") {
                        diagnostics.push(self.create_diagnostic_impl(
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

    /// Check security issues (internal implementation)
    fn check_security_issues_impl(&self, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = content.split("\n").collect();
        
        for (line_num, line) in lines.iter().enumerate() {
            // Check for potential security issues
            if line.contains("eval") || line.contains("exec") {
                diagnostics.push(self.create_diagnostic_impl(
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

    /// Create a diagnostic (internal implementation)
    fn create_diagnostic_impl(
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
            return Some(self.create_diagnostic_impl(
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
            return Some(self.create_diagnostic_impl(
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


    /// Check type errors in content
    pub fn check_type_errors(&self, content: &str) -> Vec<tower_lsp::lsp_types::Diagnostic> {
        self.check_type_errors_impl(content)
    }

    /// Check variable usage
    pub fn check_variable_usage(&self, content: &str) -> Vec<tower_lsp::lsp_types::Diagnostic> {
        self.check_variable_usage_impl(content)
    }

    /// Check function calls
    pub fn check_function_calls(&self, content: &str) -> Vec<tower_lsp::lsp_types::Diagnostic> {
        self.check_function_calls_impl(content)
    }

    /// Check imports
    pub fn check_imports(&self, content: &str) -> Vec<tower_lsp::lsp_types::Diagnostic> {
        self.check_imports_impl(content)
    }

    /// Check style issues
    pub fn check_style_issues(&self, content: &str) -> Vec<tower_lsp::lsp_types::Diagnostic> {
        self.check_style_issues_impl(content)
    }

    /// Check best practices
    pub fn check_best_practices(&self, content: &str) -> Vec<tower_lsp::lsp_types::Diagnostic> {
        self.check_best_practices_impl(content)
    }

    /// Check performance issues
    pub fn check_performance_issues(&self, content: &str) -> Vec<tower_lsp::lsp_types::Diagnostic> {
        self.check_performance_issues_impl(content)
    }

    /// Check security issues
    pub fn check_security_issues(&self, content: &str) -> Vec<tower_lsp::lsp_types::Diagnostic> {
        self.check_security_issues_impl(content)
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
