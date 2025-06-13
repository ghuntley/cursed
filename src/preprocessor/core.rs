/// Core Preprocessor Implementation
/// 
/// This module implements the main Preprocessor struct that handles the processing
/// of tokens from the lexer, adding contextual information about generic syntax.

use crate::lexer::{Lexer, Token};
use crate::error::SourceLocation;
use crate::preprocessor::token_stream::{TokenStream, TokenWithContext, TokenMetadata};
use crate::preprocessor::error::{PreprocessorError, PreprocessorResult};
use std::collections::VecDeque;

/// Maximum nesting depth for generic types to prevent stack overflow
const MAX_GENERIC_NESTING_DEPTH: usize = 10;

/// Core preprocessor that enhances tokens with generic syntax context
#[derive(Debug)]
pub struct Preprocessor {
    /// The lexer providing raw tokens
    lexer: Lexer,
    /// Buffer for lookahead token analysis
    token_buffer: VecDeque<TokenWithContext>,
    /// The enhanced token stream being built
    token_stream: TokenStream,
    /// Current position in processing
    position: usize,
    /// Flag to track if preprocessor is initialized
    initialized: bool,
}

impl Preprocessor {
    /// Create a new preprocessor with the given lexer
    pub fn new(lexer: Lexer) -> Self {
        Self {
            lexer,
            token_buffer: VecDeque::new(),
            token_stream: TokenStream::new(),
            position: 0,
            initialized: true,
        }
    }

    /// Check if the preprocessor is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Get the current token stream
    pub fn token_stream(&self) -> &TokenStream {
        &self.token_stream
    }

    /// Process all tokens through the preprocessor
    pub fn process(&mut self) -> PreprocessorResult<TokenStream> {
        // Fill initial buffer for lookahead analysis
        self.fill_buffer()?;
        
        // Process tokens until buffer is empty
        while !self.token_buffer.is_empty() {
            self.process_buffer()?;
        }
        
        Ok(self.token_stream.clone())
    }

    /// Fill the token buffer with tokens from the lexer
    fn fill_buffer(&mut self) -> PreprocessorResult<()> {
        // Try to fill buffer with at least 10 tokens for lookahead
        while self.token_buffer.len() < 10 {
            match self.lexer.next_token() {
                Ok(token) => {
                    let location = self.lexer.current_location();
                    let token_with_context = TokenWithContext::new(token.clone(), location);
                    
                    // Stop on EOF
                    if matches!(token, Token::Eof) {
                        self.token_buffer.push_back(token_with_context);
                        break;
                    }
                    
                    self.token_buffer.push_back(token_with_context);
                }
                Err(err) => {
                    return Err(PreprocessorError::from(err));
                }
            }
        }
        Ok(())
    }

    /// Process the current token buffer
    fn process_buffer(&mut self) -> PreprocessorResult<()> {
        // Try to identify and process patterns
        if self.try_process_generic_type_declaration()? {
            return Ok(());
        }
        
        if self.try_process_generic_function_declaration()? {
            return Ok(());
        }
        
        if self.try_process_generic_function_call()? {
            return Ok(());
        }
        
        // If no pattern matched, process the first token normally
        if let Some(token) = self.token_buffer.pop_front() {
            self.token_stream.add_token(token.token, token.location);
            
            // Fill buffer to maintain lookahead capacity
            self.fill_buffer()?;
        }
        
        Ok(())
    }

    /// Try to process a generic type declaration pattern
    /// Pattern: identifier '[' type_params ']' 'squad'
    fn try_process_generic_type_declaration(&mut self) -> PreprocessorResult<bool> {
        if self.token_buffer.len() < 5 {
            return Ok(false);
        }
        
        // Check for pattern: identifier '[' ... ']' 'squad'
        if let (Some(id), Some(lbracket), Some(rbracket_pos), Some(squad)) = (
            self.token_buffer.get(0),
            self.token_buffer.get(1),
            self.find_matching_bracket(1)?,
            self.token_buffer.get(self.find_matching_bracket(1)? + 1),
        ) {
            if matches!(id.token, Token::Identifier(_)) &&
               matches!(lbracket.token, Token::LeftBracket) &&
               matches!(squad.token, Token::Squad) {
                
                return self.process_generic_type_declaration(rbracket_pos);
            }
        }
        
        Ok(false)
    }

    /// Try to process a generic function declaration pattern
    /// Pattern: 'slay' identifier '[' type_params ']' '('
    fn try_process_generic_function_declaration(&mut self) -> PreprocessorResult<bool> {
        if self.token_buffer.len() < 6 {
            return Ok(false);
        }
        
        // Check for pattern: 'slay' identifier '[' ... ']' '('
        if let (Some(slay), Some(id), Some(lbracket), Some(rbracket_pos), Some(lparen)) = (
            self.token_buffer.get(0),
            self.token_buffer.get(1),
            self.token_buffer.get(2),
            self.find_matching_bracket(2).ok(),
            self.token_buffer.get(self.find_matching_bracket(2).unwrap_or(0) + 1),
        ) {
            if matches!(slay.token, Token::Slay) &&
               matches!(id.token, Token::Identifier(_)) &&
               matches!(lbracket.token, Token::LeftBracket) &&
               matches!(lparen.token, Token::LeftParen) {
                
                return self.process_generic_function_declaration(rbracket_pos);
            }
        }
        
        Ok(false)
    }

    /// Try to process a generic function call pattern
    /// Pattern: identifier '[' type_args ']' '('
    fn try_process_generic_function_call(&mut self) -> PreprocessorResult<bool> {
        if self.token_buffer.len() < 5 {
            return Ok(false);
        }
        
        // Check for pattern: identifier '[' ... ']' '('
        if let (Some(id), Some(lbracket), Some(rbracket_pos), Some(lparen)) = (
            self.token_buffer.get(0),
            self.token_buffer.get(1),
            self.find_matching_bracket(1).ok(),
            self.token_buffer.get(self.find_matching_bracket(1).unwrap_or(0) + 1),
        ) {
            if matches!(id.token, Token::Identifier(_)) &&
               matches!(lbracket.token, Token::LeftBracket) &&
               matches!(lparen.token, Token::LeftParen) {
                
                return self.process_generic_function_call(rbracket_pos);
            }
        }
        
        Ok(false)
    }

    /// Process a generic type declaration
    fn process_generic_type_declaration(&mut self, rbracket_pos: usize) -> PreprocessorResult<bool> {
        // Add identifier with metadata
        if let Some(token) = self.token_buffer.pop_front() {
            self.token_stream.add_token_with_metadata(
                token.token,
                token.location,
                TokenMetadata::GenericType
            );
        }
        
        // Add left bracket with metadata
        if let Some(token) = self.token_buffer.pop_front() {
            self.token_stream.add_token_with_metadata(
                token.token,
                token.location,
                TokenMetadata::GenericType
            );
        }
        
        // Process type parameters with nesting detection
        let mut processed = 2; // Already processed identifier and left bracket
        let mut nesting_depth = 0;
        
        while processed < rbracket_pos {
            if let Some(token) = self.token_buffer.pop_front() {
                let metadata = match token.token {
                    Token::LeftBracket => {
                        nesting_depth += 1;
                        if nesting_depth > MAX_GENERIC_NESTING_DEPTH {
                            return Err(PreprocessorError::nested_generic_too_deep(
                                token.location,
                                nesting_depth,
                                MAX_GENERIC_NESTING_DEPTH
                            ));
                        }
                        if nesting_depth > 1 {
                            TokenMetadata::NestedGenericType
                        } else {
                            TokenMetadata::GenericType
                        }
                    }
                    Token::RightBracket => {
                        nesting_depth = nesting_depth.saturating_sub(1);
                        if nesting_depth > 0 {
                            TokenMetadata::NestedGenericType
                        } else {
                            TokenMetadata::GenericType
                        }
                    }
                    _ => {
                        if nesting_depth > 1 {
                            TokenMetadata::NestedGenericType
                        } else {
                            TokenMetadata::GenericType
                        }
                    }
                };
                
                self.token_stream.add_token_with_metadata(
                    token.token,
                    token.location,
                    metadata
                );
                processed += 1;
            }
        }
        
        // Add right bracket with metadata
        if let Some(token) = self.token_buffer.pop_front() {
            self.token_stream.add_token_with_metadata(
                token.token,
                token.location,
                TokenMetadata::GenericType
            );
        }
        
        // Add 'squad' with metadata
        if let Some(token) = self.token_buffer.pop_front() {
            self.token_stream.add_token_with_metadata(
                token.token,
                token.location,
                TokenMetadata::GenericType
            );
        }
        
        self.fill_buffer()?;
        Ok(true)
    }

    /// Process a generic function declaration
    fn process_generic_function_declaration(&mut self, rbracket_pos: usize) -> PreprocessorResult<bool> {
        // Add 'slay' with metadata
        if let Some(token) = self.token_buffer.pop_front() {
            self.token_stream.add_token_with_metadata(
                token.token,
                token.location,
                TokenMetadata::GenericFunction
            );
        }
        
        // Add identifier with metadata
        if let Some(token) = self.token_buffer.pop_front() {
            self.token_stream.add_token_with_metadata(
                token.token,
                token.location,
                TokenMetadata::GenericFunction
            );
        }
        
        // Add left bracket with metadata
        if let Some(token) = self.token_buffer.pop_front() {
            self.token_stream.add_token_with_metadata(
                token.token,
                token.location,
                TokenMetadata::GenericFunction
            );
        }
        
        // Process type parameters
        let mut processed = 3; // Already processed 'slay', identifier, and left bracket
        
        while processed < rbracket_pos {
            if let Some(token) = self.token_buffer.pop_front() {
                self.token_stream.add_token_with_metadata(
                    token.token,
                    token.location,
                    TokenMetadata::GenericFunction
                );
                processed += 1;
            }
        }
        
        // Add right bracket with metadata
        if let Some(token) = self.token_buffer.pop_front() {
            self.token_stream.add_token_with_metadata(
                token.token,
                token.location,
                TokenMetadata::GenericFunction
            );
        }
        
        self.fill_buffer()?;
        Ok(true)
    }

    /// Process a generic function call
    fn process_generic_function_call(&mut self, rbracket_pos: usize) -> PreprocessorResult<bool> {
        // Add identifier with metadata
        if let Some(token) = self.token_buffer.pop_front() {
            self.token_stream.add_token_with_metadata(
                token.token,
                token.location,
                TokenMetadata::GenericFunctionCall
            );
        }
        
        // Add left bracket with metadata
        if let Some(token) = self.token_buffer.pop_front() {
            self.token_stream.add_token_with_metadata(
                token.token,
                token.location,
                TokenMetadata::GenericFunctionCall
            );
        }
        
        // Process type arguments
        let mut processed = 2; // Already processed identifier and left bracket
        
        while processed < rbracket_pos {
            if let Some(token) = self.token_buffer.pop_front() {
                self.token_stream.add_token_with_metadata(
                    token.token,
                    token.location,
                    TokenMetadata::GenericFunctionCall
                );
                processed += 1;
            }
        }
        
        // Add right bracket with metadata
        if let Some(token) = self.token_buffer.pop_front() {
            self.token_stream.add_token_with_metadata(
                token.token,
                token.location,
                TokenMetadata::GenericFunctionCall
            );
        }
        
        self.fill_buffer()?;
        Ok(true)
    }

    /// Find the matching right bracket for a left bracket at the given position
    fn find_matching_bracket(&self, start_pos: usize) -> PreprocessorResult<usize> {
        let mut depth = 0;
        
        for (i, token) in self.token_buffer.iter().enumerate().skip(start_pos) {
            match token.token {
                Token::LeftBracket => depth += 1,
                Token::RightBracket => {
                    depth -= 1;
                    if depth == 0 {
                        return Ok(i);
                    }
                }
                Token::Eof => {
                    return Err(PreprocessorError::unclosed_type_parameters(
                        token.location.clone(),
                        "Reached end of file while looking for closing bracket".to_string()
                    ));
                }
                _ => {}
            }
        }
        
        Err(PreprocessorError::unclosed_type_parameters(
            self.token_buffer.get(start_pos)
                .map(|t| t.location.clone())
                .unwrap_or_else(|| SourceLocation::new(0, 0, 0, None)),
            "No matching closing bracket found".to_string()
        ))
    }

    /// Get the current position in processing
    pub fn position(&self) -> usize {
        self.position
    }

    /// Get the size of the current token buffer
    pub fn buffer_size(&self) -> usize {
        self.token_buffer.len()
    }

    /// Get statistics about processing
    pub fn statistics(&self) -> PreprocessorStatistics {
        PreprocessorStatistics {
            tokens_processed: self.token_stream.len(),
            buffer_size: self.token_buffer.len(),
            position: self.position,
            stream_stats: self.token_stream.statistics(),
        }
    }
}

/// Statistics about preprocessor processing
#[derive(Debug, Clone)]
pub struct PreprocessorStatistics {
    pub tokens_processed: usize,
    pub buffer_size: usize,
    pub position: usize,
    pub stream_stats: crate::preprocessor::token_stream::TokenStreamStatistics,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preprocessor_creation() {
        let source = "be_like Box[T] squad { value normie }";
        let lexer = Lexer::new(source.to_string());
        let preprocessor = Preprocessor::new(lexer);
        
        assert!(preprocessor.is_initialized());
        assert_eq!(preprocessor.position(), 0);
        assert_eq!(preprocessor.buffer_size(), 0);
    }

    #[test]
    fn test_process_generic_type() {
        let source = "be_like Box[T] squad { value normie }";
        let lexer = Lexer::new(source.to_string());
        let mut preprocessor = Preprocessor::new(lexer);
        
        let result = preprocessor.process();
        assert!(result.is_ok());
        
        let stream = result.unwrap();
        assert!(stream.contains_generic_type_declaration());
        
        let generic_tokens = stream.tokens_with_metadata(&TokenMetadata::GenericType);
        assert!(!generic_tokens.is_empty());
    }

    #[test]
    fn test_process_generic_function() {
        let source = "slay foo[T](x normie) T { x }";
        let lexer = Lexer::new(source.to_string());
        let mut preprocessor = Preprocessor::new(lexer);
        
        let result = preprocessor.process();
        assert!(result.is_ok());
        
        let stream = result.unwrap();
        assert!(stream.contains_generic_function_declaration());
        
        let generic_tokens = stream.tokens_with_metadata(&TokenMetadata::GenericFunction);
        assert!(!generic_tokens.is_empty());
    }

    #[test]
    fn test_process_generic_function_call() {
        let source = "facts result = foo[normie](42)";
        let lexer = Lexer::new(source.to_string());
        let mut preprocessor = Preprocessor::new(lexer);
        
        let result = preprocessor.process();
        assert!(result.is_ok());
        
        let stream = result.unwrap();
        assert!(stream.contains_generic_function_call());
        
        let generic_tokens = stream.tokens_with_metadata(&TokenMetadata::GenericFunctionCall);
        assert!(!generic_tokens.is_empty());
    }

    #[test]
    fn test_nested_generics() {
        let source = "be_like Pair[K, V[T]] squad { k normie, v normie }";
        let lexer = Lexer::new(source.to_string());
        let mut preprocessor = Preprocessor::new(lexer);
        
        let result = preprocessor.process();
        assert!(result.is_ok());
        
        let stream = result.unwrap();
        assert!(stream.contains_nested_generic_type());
        
        let nested_tokens = stream.tokens_with_metadata(&TokenMetadata::NestedGenericType);
        assert!(!nested_tokens.is_empty());
    }

    #[test]
    fn test_unclosed_brackets_error() {
        let source = "be_like Box[T squad { value normie }"; // Missing closing bracket
        let lexer = Lexer::new(source.to_string());
        let mut preprocessor = Preprocessor::new(lexer);
        
        let result = preprocessor.process();
        assert!(result.is_err());
        
        if let Err(PreprocessorError::UnclosedTypeParameters { .. }) = result {
            // Expected error type
        } else {
            panic!("Expected UnclosedTypeParameters error");
        }
    }

    #[test]
    fn test_statistics() {
        let source = "slay foo[T](x normie) T { x }";
        let lexer = Lexer::new(source.to_string());
        let mut preprocessor = Preprocessor::new(lexer);
        
        preprocessor.process().unwrap();
        let stats = preprocessor.statistics();
        
        assert!(stats.tokens_processed > 0);
        assert!(stats.stream_stats.generic_function_tokens > 0);
    }
}
