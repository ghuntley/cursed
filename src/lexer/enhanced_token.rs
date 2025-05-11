//! Enhanced token implementation with source location information
//!
//! This module extends the basic Token structure with source location information,
//! including line and column numbers. This is especially useful for error reporting
//! and for providing better diagnostics for the compiler.

use crate::error::SourceLocation;
use crate::lexer::token::Token;
use crate::lexer::TokenType;

/// Extends Token with source location information
#[derive(Debug, Clone, PartialEq)]
pub struct EnhancedToken {
    /// The actual token data
    pub token: Token,
    /// Line number (1-based)
    pub line: usize,
    /// Column number (1-based)
    pub column: usize,
    /// Source file path
    pub file: Option<String>,
    /// Source line content
    pub source_line: Option<String>,
}

impl EnhancedToken {
    /// Create a new enhanced token with source location information
    pub fn new(
        token_type: TokenType,
        literal: &str,
        line: usize,
        column: usize,
        file: Option<String>,
        source_line: Option<String>,
    ) -> Self {
        Self {
            token: Token::new(token_type, literal),
            line,
            column,
            file,
            source_line,
        }
    }
    
    /// Create a new enhanced token from an existing token and location info
    pub fn from_token(
        token: Token,
        line: usize,
        column: usize,
        file: Option<String>,
        source_line: Option<String>,
    ) -> Self {
        Self {
            token,
            line,
            column,
            file,
            source_line,
        }
    }
    
    /// Convert to a source location for error reporting
    pub fn to_source_location(&self) -> SourceLocation {
        SourceLocation {
            line: self.line,
            column: self.column,
            file: self.file.clone(),
            source_line: self.source_line.clone().unwrap_or_default(),
        }
    }
    
    /// Create a string representation with location information
    pub fn to_string_with_location(&self) -> String {
        let token_str = self.token.token_literal();
        format!("{}@{}:{}", token_str, self.line, self.column)
    }
}

/// Extension trait for Token to convert to EnhancedToken
pub trait TokenEnhancement {
    /// Convert to an enhanced token with location information
    fn with_location(
        self,
        line: usize,
        column: usize,
        file: Option<String>,
        source_line: Option<String>,
    ) -> EnhancedToken;
    
    /// Convert to an enhanced token with just line and column
    fn with_position(self, line: usize, column: usize) -> EnhancedToken;
}

impl TokenEnhancement for Token {
    fn with_location(
        self,
        line: usize,
        column: usize,
        file: Option<String>,
        source_line: Option<String>,
    ) -> EnhancedToken {
        EnhancedToken::from_token(self, line, column, file, source_line)
    }
    
    fn with_position(self, line: usize, column: usize) -> EnhancedToken {
        EnhancedToken::from_token(self, line, column, None, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_enhanced_token_creation() {
        let token = Token::new(TokenType::Identifier, "test_var");
        let enhanced = token.with_position(10, 5);
        
        assert_eq!(enhanced.line, 10);
        assert_eq!(enhanced.column, 5);
        
        if let Token::Identifier(name) = enhanced.token {
            assert_eq!(name, "test_var");
        } else {
            panic!("Wrong token type!");
        }
    }
    
    #[test]
    fn test_enhanced_token_to_source_location() {
        let token = Token::new(TokenType::Identifier, "test_var");
        let enhanced = token.with_location(
            10, 5, 
            Some("test.csd".to_string()),
            Some("    let test_var = 42;".to_string())
        );
        
        let location = enhanced.to_source_location();
        assert_eq!(location.line, 10);
        assert_eq!(location.column, 5);
        assert_eq!(location.file.unwrap(), "test.csd");
        assert_eq!(location.source_line, "    let test_var = 42;");
    }
    
    #[test]
    fn test_to_string_with_location() {
        let token = Token::new(TokenType::Identifier, "test_var");
        let enhanced = token.with_position(10, 5);
        
        assert_eq!(enhanced.to_string_with_location(), "test_var@10:5");
    }
}