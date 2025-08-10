//! Parser Error Recovery Implementation for CURSED
//! 
//! This module extends the main parser with comprehensive error recovery capabilities

use crate::ast::{Statement, Expression, Program};
use crate::lexer::{Token, TokenKind};
use crate::error_types::{Error, Result};
use crate::error_recovery::{
    ErrorRecoveryManager, SourceLocation, ErrorContext, RecoveryStrategy, 
    ParserState, ParserErrorRecovery, RecoveryError
};
use crate::parser_main::Parser;

impl ParserErrorRecovery for Parser {
    /// Attempt to recover from a parsing error using the specified strategy
    fn recover_from_error(&mut self, error: Error, strategy: RecoveryStrategy) -> Result<bool> {
        let location = SourceLocation::new(self.current_line, self.current_column, self.token_index)
            .with_file(self.filename.clone().unwrap_or_else(|| "input".to_string()));
        
        let context = self.error_recovery.generate_context(&location, self.filename.as_deref());
        self.error_recovery.add_error(error.clone(), location, context);
        
        match strategy {
            RecoveryStrategy::SkipToNext => {
                self.synchronize_to_statement();
                Ok(true)
            }
            RecoveryStrategy::InsertToken(token_kind) => {
                // Simulate insertion by continuing with the expected token type
                eprintln!("Recovery: Inserting missing token {:?}", token_kind);
                Ok(true)
            }
            RecoveryStrategy::ReplaceToken(token_kind) => {
                // Skip current token and continue
                self.advance_token();
                eprintln!("Recovery: Replacing token with {:?}", token_kind);
                Ok(true)
            }
            RecoveryStrategy::Backtrack => {
                if let Some(recovery_point) = self.error_recovery.restore_recovery_point() {
                    self.token_index = recovery_point.token_index;
                    self.current_token = if self.token_index < self.tokens.len() {
                        Some(self.tokens[self.token_index].clone())
                    } else {
                        None
                    };
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            RecoveryStrategy::UseDefault => {
                // Continue with a default/placeholder value
                eprintln!("Recovery: Using default value");
                Ok(true)
            }
            RecoveryStrategy::AbortScope => {
                // Skip to end of current scope
                self.skip_to_scope_end();
                Ok(true)
            }
        }
    }
    
    /// Try multiple recovery strategies in order of preference
    fn try_recovery_strategies(&mut self, error: Error) -> Result<bool> {
        let strategies = vec![
            RecoveryStrategy::InsertToken(TokenKind::Semicolon),
            RecoveryStrategy::SkipToNext,
            RecoveryStrategy::UseDefault,
            RecoveryStrategy::AbortScope,
        ];
        
        for strategy in strategies {
            if self.recover_from_error(error.clone(), strategy)? {
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    /// Synchronize parser to next statement boundary
    fn synchronize_to_statement(&mut self) {
        while let Some(token) = self.current_token.as_ref() {
            match token.kind {
                TokenKind::Eof => break,
                TokenKind::Semicolon | TokenKind::Newline => {
                    self.advance_token();
                    break;
                }
                // Statement keywords
                TokenKind::Sus | TokenKind::Slay | TokenKind::Facts | 
                TokenKind::Lowkey | TokenKind::Bestie | TokenKind::While |
                TokenKind::Return | TokenKind::Defer | TokenKind::Yikes => {
                    break;
                }
                // Scope delimiters
                TokenKind::LeftBrace | TokenKind::RightBrace => {
                    if token.kind == TokenKind::RightBrace {
                        self.advance_token();
                    }
                    break;
                }
                _ => {
                    self.advance_token();
                }
            }
        }
    }
    
    /// Synchronize parser to next expression boundary
    fn synchronize_to_expression(&mut self) {
        while let Some(token) = self.current_token.as_ref() {
            match token.kind {
                TokenKind::Eof => break,
                TokenKind::Semicolon | TokenKind::Newline | TokenKind::Comma => {
                    break;
                }
                TokenKind::RightParen | TokenKind::RightBrace | TokenKind::RightBracket => {
                    break;
                }
                _ => {
                    self.advance_token();
                }
            }
        }
    }
}

impl Parser {
    /// Enhanced parse_program with error recovery
    pub fn parse_program_with_recovery(&mut self) -> Result<Program> {
        let mut statements = Vec::new();
        let mut imports = Vec::new();
        let mut package = None;
        
        while let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::Eof {
                break;
            }
            
            // Skip newlines and semicolons
            if token.kind == TokenKind::Newline || token.kind == TokenKind::Semicolon {
                if let Err(_) = self.next_token() {
                    break;
                }
                continue;
            }
            
            // Try to parse with error recovery
            match self.parse_statement_with_recovery() {
                Ok(Some(statement)) => {
                    statements.push(statement);
                }
                Ok(None) => {
                    // Successfully recovered, continue
                    continue;
                }
                Err(error) => {
                    // Attempt error recovery
                    if !self.try_recovery_strategies(error)? {
                        // If recovery fails, check if we should continue
                        if !self.error_recovery.should_continue() {
                            break;
                        }
                        self.synchronize_to_statement();
                    }
                }
            }
            
            // Prevent infinite loops
            if self.current_token.is_none() {
                break;
            }
        }
        
        Ok(Program {
            statements,
            imports,
            package,
        })
    }
    
    /// Enhanced parse_statement with error recovery
    pub fn parse_statement_with_recovery(&mut self) -> Result<Option<Statement>> {
        // Create recovery point
        let state = ParserState {
            scope_depth: self.scope_depth,
            in_function: self.in_function,
            in_loop: self.in_loop,
            expected_tokens: vec![],
        };
        
        let location = SourceLocation::new(self.current_line, self.current_column, self.token_index)
            .with_file(self.filename.clone().unwrap_or_else(|| "input".to_string()));
        
        self.error_recovery.create_recovery_point(location, self.token_index, state);
        
        // Try to parse statement normally first
        match self.parse_statement() {
            Ok(stmt) => Ok(stmt),
            Err(error) => {
                // Attempt recovery strategies
                if self.try_recovery_strategies(error)? {
                    // Recovery successful, return None to indicate we should continue
                    Ok(None)
                } else {
                    // Recovery failed, propagate error
                    Err(Error::Parse("Failed to recover from parse error".to_string()))
                }
            }
        }
    }
    
    /// Enhanced parse_expression with error recovery
    pub fn parse_expression_with_recovery(&mut self) -> Result<Expression> {
        match self.parse_expression() {
            Ok(expr) => Ok(expr),
            Err(error) => {
                // Attempt recovery for expressions
                if self.try_recovery_strategies(error)? {
                    // Return a placeholder expression
                    Ok(Expression::Literal(crate::ast::Literal::String("__RECOVERY_PLACEHOLDER__".to_string())))
                } else {
                    Err(Error::Parse("Failed to recover from expression parse error".to_string()))
                }
            }
        }
    }
    
    /// Skip to end of current scope (matching braces)
    fn skip_to_scope_end(&mut self) {
        let mut brace_depth = 0;
        let mut paren_depth = 0;
        let mut bracket_depth = 0;
        
        while let Some(token) = self.current_token.as_ref() {
            match token.kind {
                TokenKind::Eof => break,
                TokenKind::LeftBrace => brace_depth += 1,
                TokenKind::RightBrace => {
                    if brace_depth > 0 {
                        brace_depth -= 1;
                    } else {
                        // End of current scope
                        self.advance_token();
                        break;
                    }
                }
                TokenKind::LeftParen => paren_depth += 1,
                TokenKind::RightParen => {
                    if paren_depth > 0 {
                        paren_depth -= 1;
                    }
                }
                TokenKind::LeftBracket => bracket_depth += 1,
                TokenKind::RightBracket => {
                    if bracket_depth > 0 {
                        bracket_depth -= 1;
                    }
                }
                _ => {}
            }
            
            self.advance_token();
            
            // If all depths are zero and we're at statement boundary, stop
            if brace_depth == 0 && paren_depth == 0 && bracket_depth == 0 {
                if let Some(current) = self.current_token.as_ref() {
                    if current.kind == TokenKind::Semicolon || current.kind == TokenKind::Newline {
                        break;
                    }
                }
            }
        }
    }
    
    /// Enhanced consume_token with error recovery
    pub fn consume_token_with_recovery(&mut self, expected: TokenKind) -> Result<()> {
        if let Some(token) = self.current_token.as_ref() {
            if token.kind == expected {
                self.next_token()?;
                Ok(())
            } else {
                let error = Error::Parse(format!("Expected {:?}, found {:?}", expected, token.kind));
                
                // Try recovery by inserting the missing token
                if self.recover_from_error(error, RecoveryStrategy::InsertToken(expected))? {
                    Ok(())
                } else {
                    Err(Error::Parse(format!("Failed to recover: Expected {:?}, found {:?}", expected, token.kind)))
                }
            }
        } else {
            let error = Error::Parse(format!("Expected {:?}, found end of input", expected));
            
            // Try recovery by inserting the missing token
            if self.recover_from_error(error, RecoveryStrategy::InsertToken(expected))? {
                Ok(())
            } else {
                Err(Error::Parse("Unexpected end of input".to_string()))
            }
        }
    }
    
    /// Generate comprehensive error report
    pub fn generate_error_report(&self) -> String {
        self.error_recovery.generate_report()
    }
    
    /// Check if compilation should continue despite errors
    pub fn should_continue_compilation(&self) -> bool {
        self.error_recovery.should_continue()
    }
    
    /// Get all recovery errors
    pub fn get_recovery_errors(&self) -> &Vec<RecoveryError> {
        &self.error_recovery.errors
    }
    
    /// Update line and column tracking
    pub fn update_location(&mut self, line: usize, column: usize) {
        self.current_line = line;
        self.current_column = column;
    }
    
    /// Set source text for error context
    pub fn set_source_context(&mut self, source: String, filename: Option<String>) {
        self.source_text = source.clone();
        self.filename = filename.clone();
        
        // Cache source for error reporting
        if let Some(ref file) = filename {
            self.error_recovery.cache_source(file, &source);
        } else {
            self.error_recovery.cache_source("input", &source);
        }
    }
}

/// Helper macro for error recovery in parsing
#[macro_export]
macro_rules! parse_with_recovery {
    ($parser:expr, $parse_fn:expr, $recovery_strategy:expr) => {
        match $parse_fn {
            Ok(result) => Ok(result),
            Err(error) => {
                if $parser.recover_from_error(error, $recovery_strategy)? {
                    // Return appropriate placeholder/default value
                    $parse_fn
                } else {
                    Err(Error::Parse("Recovery failed".to_string()))
                }
            }
        }
    };
}

/// Helper function to create enhanced parser with error recovery
pub fn create_parser_with_recovery(source: String, filename: Option<String>) -> Result<Parser> {
    use crate::lexer::Lexer;
    
    let lexer = Lexer::new(source.clone());
    let mut parser = Parser::new(lexer)?;
    parser.set_source_context(source, filename);
    Ok(parser)
}
