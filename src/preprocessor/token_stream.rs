/// Token Stream and Context Types
/// 
/// This module defines the enhanced token stream used by the preprocessor
/// to provide contextual information about generic syntax patterns.

use crate::lexer::Token;
use crate::error::SourceLocation;
use crate::preprocessor::error::{PreprocessorError, PreprocessorResult};
use crate::error::Error;

/// Enhanced token with contextual information
#[derive(Debug, Clone, PartialEq)]
pub struct TokenWithContext {
    /// The original token from the lexer
    pub token: Token,
    /// Source location information
    pub location: SourceLocation,
    /// Optional metadata about generic syntax context
    pub metadata: Option<TokenMetadata>,
}

impl TokenWithContext {
    /// Create a new token with context
    pub fn new(token: Token, location: SourceLocation) -> Self {
        Self {
            token,
            location,
            metadata: None,
        }
    }

    /// Create a new token with context and metadata
    pub fn with_metadata(token: Token, location: SourceLocation, metadata: TokenMetadata) -> Self {
        Self {
            token,
            location,
            metadata: Some(metadata),
        }
    }

    /// Check if this token has metadata
    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }

    /// Get the metadata type if present
    pub fn metadata_type(&self) -> Option<&TokenMetadata> {
        self.metadata.as_ref()
    }

    /// Check if this token is part of a generic type declaration
    pub fn is_generic_type(&self) -> bool {
        matches!(self.metadata, Some(TokenMetadata::GenericType))
    }

    /// Check if this token is part of a generic function declaration
    pub fn is_generic_function(&self) -> bool {
        matches!(self.metadata, Some(TokenMetadata::GenericFunction))
    }

    /// Check if this token is part of a generic function call
    pub fn is_generic_function_call(&self) -> bool {
        matches!(self.metadata, Some(TokenMetadata::GenericFunctionCall))
    }

    /// Check if this token is part of a nested generic type
    pub fn is_nested_generic_type(&self) -> bool {
        matches!(self.metadata, Some(TokenMetadata::NestedGenericType))
    }
}

/// Metadata about token context in generic syntax
#[derive(Debug, Clone, PartialEq)]
pub enum TokenMetadata {
    /// Token is part of a generic type declaration (e.g., Box[T])
    GenericType,
    /// Token is part of a generic function declaration (e.g., slay foo[T])
    GenericFunction,
    /// Token is part of a generic function call (e.g., foo[normie])
    GenericFunctionCall,
    /// Token is part of a nested generic type (e.g., Pair[K, V[T]])
    NestedGenericType,
}

impl TokenMetadata {
    /// Get a human-readable description of the metadata type
    pub fn description(&self) -> &'static str {
        match self {
            Self::GenericType => "generic type declaration",
            Self::GenericFunction => "generic function declaration",
            Self::GenericFunctionCall => "generic function call",
            Self::NestedGenericType => "nested generic type",
        }
    }
}

/// Enhanced token stream with contextual information
#[derive(Debug, Clone)]
pub struct TokenStream {
    /// Collection of tokens with context
    pub tokens: Vec<TokenWithContext>,
    /// Current position in the token stream
    pub position: usize,
}

impl TokenStream {
    /// Create a new empty token stream
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            position: 0,
        }
    }

    /// Create a token stream with initial capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            tokens: Vec::with_capacity(capacity),
            position: 0,
        }
    }

    /// Add a token without metadata
    pub fn add_token(&mut self, token: Token, location: SourceLocation) {
        self.tokens.push(TokenWithContext::new(token, location));
    }

    /// Add a token with metadata
    pub fn add_token_with_metadata(&mut self, token: Token, location: SourceLocation, metadata: TokenMetadata) {
        self.tokens.push(TokenWithContext::with_metadata(token, location, metadata));
    }

    /// Get the current token
    pub fn current_token(&self) -> Option<&TokenWithContext> {
        self.tokens.get(self.position)
    }

    /// Advance to the next token
    pub fn advance(&mut self) -> Option<&TokenWithContext> {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
        self.current_token()
    }

    /// Peek at the next token without advancing
    pub fn peek(&self) -> Option<&TokenWithContext> {
        self.tokens.get(self.position + 1)
    }

    /// Peek at a token at the given offset from current position
    pub fn peek_at(&self, offset: usize) -> Option<&TokenWithContext> {
        self.tokens.get(self.position + offset)
    }

    /// Reset position to the beginning
    pub fn reset(&mut self) {
        self.position = 0;
    }

    /// Check if we're at the end of the stream
    pub fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }

    /// Get the number of tokens in the stream
    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    /// Check if the stream is empty
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    /// Get all tokens as a slice
    pub fn tokens(&self) -> &[TokenWithContext] {
        &self.tokens
    }

    /// Get tokens with specific metadata type
    pub fn tokens_with_metadata(&self, metadata_type: &TokenMetadata) -> Vec<&TokenWithContext> {
        self.tokens
            .iter()
            .filter(|token| token.metadata.as_ref() == Some(metadata_type))
            .collect()
    }

    /// Pattern detection methods as specified in the preprocessor spec

    /// Check if the token stream contains a generic type declaration pattern
    pub fn contains_generic_type_declaration(&self) -> bool {
        // Look for pattern: identifier '[' type_params ']' 'squad'
        for i in 0..self.tokens.len().saturating_sub(4) {
            if let (Some(id), Some(lbracket), Some(_param), Some(rbracket), Some(squad)) = (
                self.tokens.get(i),
                self.tokens.get(i + 1),
                self.tokens.get(i + 2),
                self.tokens.get(i + 3),
                self.tokens.get(i + 4),
            ) {
                if matches!(id.token, Token::Identifier(_)) &&
                   matches!(lbracket.token, Token::LeftBracket) &&
                   matches!(rbracket.token, Token::RightBracket) &&
                   matches!(squad.token, Token::Squad) {
                    return true;
                }
            }
        }
        false
    }

    /// Check if the token stream contains a generic function declaration pattern
    pub fn contains_generic_function_declaration(&self) -> bool {
        // Look for pattern: 'slay' identifier '[' type_params ']' '('
        for i in 0..self.tokens.len().saturating_sub(5) {
            if let (Some(slay), Some(id), Some(lbracket), Some(_param), Some(rbracket), Some(lparen)) = (
                self.tokens.get(i),
                self.tokens.get(i + 1),
                self.tokens.get(i + 2),
                self.tokens.get(i + 3),
                self.tokens.get(i + 4),
                self.tokens.get(i + 5),
            ) {
                if matches!(slay.token, Token::Slay) &&
                   matches!(id.token, Token::Identifier(_)) &&
                   matches!(lbracket.token, Token::LeftBracket) &&
                   matches!(rbracket.token, Token::RightBracket) &&
                   matches!(lparen.token, Token::LeftParen) {
                    return true;
                }
            }
        }
        false
    }

    /// Check if the token stream contains a generic function call pattern
    pub fn contains_generic_function_call(&self) -> bool {
        // Look for pattern: identifier '[' type_args ']' '('
        for i in 0..self.tokens.len().saturating_sub(4) {
            if let (Some(id), Some(lbracket), Some(_arg), Some(rbracket), Some(lparen)) = (
                self.tokens.get(i),
                self.tokens.get(i + 1),
                self.tokens.get(i + 2),
                self.tokens.get(i + 3),
                self.tokens.get(i + 4),
            ) {
                if matches!(id.token, Token::Identifier(_)) &&
                   matches!(lbracket.token, Token::LeftBracket) &&
                   matches!(rbracket.token, Token::RightBracket) &&
                   matches!(lparen.token, Token::LeftParen) {
                    return true;
                }
            }
        }
        false
    }

    /// Check if the token stream contains nested generic types
    pub fn contains_nested_generic_type(&self) -> bool {
        let mut bracket_depth = 0;
        for token in &self.tokens {
            match token.token {
                Token::LeftBracket => {
                    bracket_depth += 1;
                    if bracket_depth > 1 {
                        return true;
                    }
                }
                Token::RightBracket => {
                    bracket_depth = bracket_depth.saturating_sub(1);
                }
                _ => {}
            }
        }
        false
    }

    /// Check if the token stream has separate bracket pairs (not nested)
    pub fn contains_separate_brackets(&self) -> bool {
        let mut bracket_pairs = 0;
        let mut bracket_depth = 0;
        
        for token in &self.tokens {
            match token.token {
                Token::LeftBracket => {
                    if bracket_depth == 0 {
                        bracket_pairs += 1;
                    }
                    bracket_depth += 1;
                }
                Token::RightBracket => {
                    bracket_depth = bracket_depth.saturating_sub(1);
                }
                _ => {}
            }
        }
        
        bracket_pairs > 1
    }

    /// Get statistics about the token stream
    pub fn statistics(&self) -> TokenStreamStatistics {
        let mut stats = TokenStreamStatistics::default();
        
        for token in &self.tokens {
            stats.total_tokens += 1;
            
            match &token.metadata {
                Some(TokenMetadata::GenericType) => stats.generic_type_tokens += 1,
                Some(TokenMetadata::GenericFunction) => stats.generic_function_tokens += 1,
                Some(TokenMetadata::GenericFunctionCall) => stats.generic_function_call_tokens += 1,
                Some(TokenMetadata::NestedGenericType) => stats.nested_generic_tokens += 1,
                None => stats.plain_tokens += 1,
            }
            
            match token.token {
                Token::LeftBracket => stats.left_brackets += 1,
                Token::RightBracket => stats.right_brackets += 1,
                _ => {}
            }
        }
        
        stats
    }
}

impl Default for TokenStream {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about a token stream
#[derive(Debug, Clone, Default)]
pub struct TokenStreamStatistics {
    pub total_tokens: usize,
    pub plain_tokens: usize,
    pub generic_type_tokens: usize,
    pub generic_function_tokens: usize,
    pub generic_function_call_tokens: usize,
    pub nested_generic_tokens: usize,
    pub left_brackets: usize,
    pub right_brackets: usize,
}

impl TokenStreamStatistics {
    /// Check if brackets are balanced
    pub fn brackets_balanced(&self) -> bool {
        self.left_brackets == self.right_brackets
    }

    /// Get the percentage of tokens with metadata
    pub fn metadata_percentage(&self) -> f64 {
        if self.total_tokens == 0 {
            0.0
        } else {
            (self.total_tokens - self.plain_tokens) as f64 / self.total_tokens as f64 * 100.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_with_context() {
        let token = Token::Identifier("Box".to_string());
        let location = SourceLocation::new(1, 5, 8, Some("test.csd".to_string()));
        let context = TokenWithContext::new(token.clone(), location.clone());
        
        assert_eq!(context.token, token);
        assert_eq!(context.location, location);
        assert!(!context.has_metadata());
    }

    #[test]
    fn test_token_with_metadata() {
        let token = Token::Identifier("T".to_string());
        let location = SourceLocation::new(1, 10, 11, Some("test.csd".to_string()));
        let metadata = TokenMetadata::GenericType;
        let context = TokenWithContext::with_metadata(token.clone(), location.clone(), metadata.clone());
        
        assert_eq!(context.token, token);
        assert_eq!(context.location, location);
        assert!(context.has_metadata());
        assert_eq!(context.metadata_type(), Some(&metadata));
        assert!(context.is_generic_type());
    }

    #[test]
    fn test_token_stream_operations() {
        let mut stream = TokenStream::new();
        let location = SourceLocation::new(1, 1, 5, Some("test.csd".to_string()));
        
        stream.add_token(Token::Identifier("Box".to_string()), location.clone());
        stream.add_token(Token::LeftBracket, location.clone());
        stream.add_token_with_metadata(
            Token::Identifier("T".to_string()),
            location.clone(),
            TokenMetadata::GenericType
        );
        stream.add_token(Token::RightBracket, location.clone());
        
        assert_eq!(stream.len(), 4);
        assert!(!stream.is_empty());
        
        let current = stream.current_token().unwrap();
        assert!(matches!(current.token, Token::Identifier(_)));
        
        stream.advance();
        let next = stream.current_token().unwrap();
        assert!(matches!(next.token, Token::LeftBracket));
    }

    #[test]
    fn test_pattern_detection() {
        let mut stream = TokenStream::new();
        let location = SourceLocation::new(1, 1, 1, Some("test.csd".to_string()));
        
        // Add tokens for: Box[T] squad
        stream.add_token(Token::Identifier("Box".to_string()), location.clone());
        stream.add_token(Token::LeftBracket, location.clone());
        stream.add_token(Token::Identifier("T".to_string()), location.clone());
        stream.add_token(Token::RightBracket, location.clone());
        stream.add_token(Token::Squad, location.clone());
        
        assert!(stream.contains_generic_type_declaration());
        assert!(!stream.contains_generic_function_declaration());
        assert!(!stream.contains_nested_generic_type());
    }

    #[test]
    fn test_statistics() {
        let mut stream = TokenStream::new();
        let location = SourceLocation::new(1, 1, 1, Some("test.csd".to_string()));
        
        stream.add_token(Token::Identifier("foo".to_string()), location.clone());
        stream.add_token(Token::LeftBracket, location.clone());
        stream.add_token_with_metadata(
            Token::Identifier("T".to_string()),
            location.clone(),
            TokenMetadata::GenericType
        );
        stream.add_token(Token::RightBracket, location.clone());
        
        let stats = stream.statistics();
        assert_eq!(stats.total_tokens, 4);
        assert_eq!(stats.generic_type_tokens, 1);
        assert_eq!(stats.left_brackets, 1);
        assert_eq!(stats.right_brackets, 1);
        assert!(stats.brackets_balanced());
    }
}
