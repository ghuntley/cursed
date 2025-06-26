//! Token stream management for preprocessing

use crate::error::CursedError;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct TokenStream {
    tokens: Vec<TokenWithContext>,
    position: usize,
}

#[derive(Debug, Clone)]
pub struct TokenWithContext {
    pub token: String,
    pub metadata: TokenMetadata,
}

#[derive(Debug, Clone)]
pub struct TokenMetadata {
    pub line: usize,
    pub column: usize,
    pub file: String,
    pub timestamp: Instant,
}

impl TokenStream {
    pub fn new(tokens: Vec<crate::lexer::Token>) -> Self {
        let token_contexts = tokens.into_iter().map(|token| {
            TokenWithContext {
                token: format!("{:?}", token), // Convert token to string representation
                metadata: TokenMetadata::new(0, 0, "".to_string()),
            }
        }).collect();
        
        Self {
            tokens: token_contexts,
            position: 0,
        }
    }
    
    pub fn empty() -> Self {
        Self {
            tokens: Vec::new(),
            position: 0,
        }
    }

    pub fn add_token(&mut self, token: String, metadata: TokenMetadata) {
        self.tokens.push(TokenWithContext { token, metadata });
    }

    pub fn next_token(&mut self) -> Option<&TokenWithContext> {
        if self.position < self.tokens.len() {
            let token = &self.tokens[self.position];
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    pub fn peek_token(&self) -> Option<&TokenWithContext> {
        if self.position < self.tokens.len() {
            Some(&self.tokens[self.position])
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.position = 0;
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }
}

impl TokenMetadata {
    pub fn new(line: usize, column: usize, file: String) -> Self {
        Self {
            line,
            column,
            file,
            timestamp: Instant::now(),
        }
    }
}

impl Default for TokenStream {
    fn default() -> Self {
        Self::empty()
    }
}
