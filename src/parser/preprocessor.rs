//! Token preprocessor for the CURSED language
//!
//! This module implements a token preprocessor that handles complex syntax
//! patterns like generics by combining multiple tokens into meaningful units
//! before they reach the parser.

use crate::error::{Error, ErrorReporter, SourceLocation};
use crate::lexer::{Lexer, Token};

/// A stream of preprocessed tokens with metadata
#[derive(Debug, Clone)]
pub struct TokenStream {
    /// The tokens in the stream
    pub tokens: Vec<TokenWithContext>,
    /// The current position in the stream
    pub position: usize,
}

/// A token with additional context information
#[derive(Debug, Clone)]
pub struct TokenWithContext {
    /// The token
    pub token: Token,
    /// The source location of the token
    pub location: SourceLocation,
    /// Optional metadata for the token
    pub metadata: Option<TokenMetadata>,
}

/// Metadata that can be attached to tokens
#[derive(Debug, Clone)]
pub enum TokenMetadata {
    /// Metadata for a generic type declaration
    GenericType {
        /// The name of the type
        name: String,
        /// The type parameters
        type_params: Vec<String>,
    },
    /// Metadata for a generic function declaration
    GenericFunction {
        /// The name of the function
        name: String,
        /// The type parameters
        type_params: Vec<String>,
    },
    /// Metadata for a generic function call
    GenericFunctionCall {
        /// The name of the function
        name: String,
        /// The type arguments
        type_args: Vec<String>,
    },
    /// Metadata for a nested generic type
    NestedGenericType {
        /// The outer type
        outer_type: String,
        /// Information about the nested type
        nested_types: Vec<String>,
    },
}

impl TokenStream {
    /// Creates a new token stream
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            position: 0,
        }
    }

    /// Adds a token to the stream
    pub fn add_token(&mut self, token: Token, location: SourceLocation) {
        self.tokens.push(TokenWithContext {
            token,
            location,
            metadata: None,
        });
    }

    /// Adds a token with metadata to the stream
    pub fn add_token_with_metadata(&mut self, token: Token, location: SourceLocation, metadata: TokenMetadata) {
        self.tokens.push(TokenWithContext {
            token,
            location,
            metadata: Some(metadata),
        });
    }

    /// Checks if the token stream contains a generic type declaration
    pub fn contains_generic_type_declaration(&self, name: &str, type_params: &[&str]) -> bool {
        for token_with_context in &self.tokens {
            if let Some(TokenMetadata::GenericType { name: ref n, type_params: ref tp }) = token_with_context.metadata {
                if n == name && tp.len() == type_params.len() && tp.iter().zip(type_params).all(|(a, b)| a == b) {
                    return true;
                }
            }
        }
        false
    }

    /// Checks if the token stream contains a generic function declaration
    pub fn contains_generic_function_declaration(&self, name: &str, type_params: &[&str]) -> bool {
        for token_with_context in &self.tokens {
            if let Some(TokenMetadata::GenericFunction { name: ref n, type_params: ref tp }) = token_with_context.metadata {
                if n == name && tp.len() == type_params.len() && tp.iter().zip(type_params).all(|(a, b)| a == b) {
                    return true;
                }
            }
        }
        false
    }

    /// Checks if the token stream contains a generic function call
    pub fn contains_generic_function_call(&self, name: &str, type_args: &[&str]) -> bool {
        for token_with_context in &self.tokens {
            if let Some(TokenMetadata::GenericFunctionCall { name: ref n, type_args: ref ta }) = token_with_context.metadata {
                if n == name && ta.len() == type_args.len() && ta.iter().zip(type_args).all(|(a, b)| a == b) {
                    return true;
                }
            }
        }
        false
    }

    /// Checks if the token stream contains a nested generic type
    pub fn contains_nested_generic_type(&self) -> bool {
        for token_with_context in &self.tokens {
            if let Some(TokenMetadata::NestedGenericType { .. }) = token_with_context.metadata {
                return true;
            }
        }
        false
    }

    /// Checks if the token stream contains separate brackets (which would indicate failure to combine tokens)
    pub fn contains_separate_brackets(&self) -> bool {
        let mut in_brackets = false;
        
        for token_with_context in &self.tokens {
            match token_with_context.token {
                Token::LBracket => {
                    // If we find a left bracket that isn't part of a generic token, return true
                    if token_with_context.metadata.is_none() {
                        return true;
                    }
                    in_brackets = true;
                },
                Token::RBracket => {
                    // If we find a right bracket that isn't part of a generic token, return true
                    if token_with_context.metadata.is_none() {
                        return true;
                    }
                    in_brackets = false;
                },
                _ => {}
            }
        }
        
        // If we're still in brackets at the end, that's a problem too
        in_brackets
    }
}

/// The preprocessor for the CURSED language
///
/// The preprocessor takes a token stream from the lexer and processes it
/// to handle complex syntax patterns like generics before the parser sees it.
pub struct Preprocessor<'a> {
    /// The lexer that provides tokens
    lexer: &'a mut Lexer<'a>,
    /// Buffer for tokens being processed
    token_buffer: Vec<(Token, SourceLocation)>,
    /// The processed token stream
    token_stream: TokenStream,
}

impl<'a> Preprocessor<'a> {
    /// Creates a new preprocessor with the given lexer
    pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
        Self {
            lexer,
            token_buffer: Vec::new(),
            token_stream: TokenStream::new(),
        }
    }

    /// Processes the token stream and returns the preprocessed tokens
    #[tracing::instrument(skip(self), level = "debug")]
    pub fn process(&mut self) -> Result<TokenStream, Error> {
        tracing::debug!("Starting token preprocessing");
        // Main processing loop - lex tokens and combine as needed
        loop {
            let token_result = self.lexer.next_token();
            
            match token_result {
                Ok(token) => {
                    let location = self.lexer.location();
                    
                    // Check for end of file
                    if token == Token::Eof {
                        self.token_stream.add_token(token, location);
                        break;
                    }
                    
                    // Buffer the token and its location
                    self.token_buffer.push((token, location));
                    
                    // Try to process the buffer
                    self.process_buffer()?;
                },
                Err(e) => return Err(e),
            }
        }
        
        // Process any remaining tokens in the buffer
        while !self.token_buffer.is_empty() {
            let (token, location) = self.token_buffer.remove(0);
            self.token_stream.add_token(token, location);
        }
        
        Ok(self.token_stream.clone())
    }

    /// Processes the token buffer to identify and handle generic syntax
    #[tracing::instrument(skip(self), fields(buffer_size = self.token_buffer.len()), level = "trace")]
    fn process_buffer(&mut self) -> Result<(), Error> {
        // Need at least 3 tokens to detect a generic pattern (identifier, LBracket, ...)
        if self.token_buffer.len() < 3 {
            return Ok(());
        }
        
        // Check for generic type declaration pattern: be_like TypeName[TypeParam] squad
        self.process_generic_type_declaration()?;
        
        // Check for generic function declaration pattern: slay funcName[TypeParam](args) returnType
        self.process_generic_function_declaration()?;
        
        // Check for generic function call pattern: funcName[TypeArg](args)
        self.process_generic_function_call()?;
        
        // If we couldn't match any pattern and the buffer is getting large, start flushing
        if self.token_buffer.len() > 10 {
            let (token, location) = self.token_buffer.remove(0);
            tracing::trace!(token = ?token, line = location.line, column = location.column, "Flushing token from buffer");
            self.token_stream.add_token(token, location);
        }
        
        Ok(())
    }

    /// Processes a generic type declaration pattern
    fn process_generic_type_declaration(&mut self) -> Result<(), Error> {
        // Pattern: be_like TypeName[TypeParam] squad
        if self.token_buffer.len() >= 5 {
            if let (
                (Token::BeLike, _),
                (Token::Identifier(ref type_name), _),
                (Token::LBracket, bracket_loc),
                ..,
            ) = (
                &self.token_buffer[0],
                &self.token_buffer[1],
                &self.token_buffer[2],
                // The rest of the tokens
            ) {
                // Found potential generic type declaration
                let mut type_params = Vec::new();
                let mut bracket_depth = 1;
                let mut end_index = 3;
                
                // Process everything inside the brackets
                while end_index < self.token_buffer.len() {
                    match &self.token_buffer[end_index].0 {
                        Token::LBracket => bracket_depth += 1,
                        Token::RBracket => {
                            bracket_depth -= 1;
                            if bracket_depth == 0 {
                                // We've reached the closing bracket
                                break;
                            }
                        },
                        Token::Identifier(param) => {
                            if bracket_depth == 1 && end_index > 0 {
                                // This is a top-level type parameter
                                type_params.push(param.clone());
                            }
                        },
                        Token::Comma => {
                            // Just skip commas between type parameters
                        },
                        _ => {
                            // Other tokens inside brackets might be part of nested generic types
                        }
                    }
                    end_index += 1;
                }
                
                // Check if we found the closing bracket
                if end_index < self.token_buffer.len() && bracket_depth == 0 {
                    // Look for 'squad' keyword after closing bracket
                    if end_index + 1 < self.token_buffer.len() && 
                       matches!(self.token_buffer[end_index + 1].0, Token::Squad) {
                        
                        // Create metadata for the generic type
                        let metadata = TokenMetadata::GenericType {
                            name: type_name.clone(),
                            type_params,
                        };
                        
                        // Add the tokens to the stream with the metadata
                        let (be_like_token, be_like_loc) = self.token_buffer.remove(0);
                        
                        // Check if buffer is empty after removing first token
                        if self.token_buffer.is_empty() {
                            return Err(Error::new(
                                "Preprocessor",
                                "Unexpected end of token buffer during generic type processing",
                                Some(be_like_loc.clone())
                            ));
                        }
                        
                        let (identifier_token, identifier_loc) = self.token_buffer.remove(0);
                        let identifier_loc_clone = identifier_loc.clone(); // Clone early for later use
                        
                        // Remove the processed tokens (from LBracket to RBracket)
                        // First check if we have enough tokens
                        if self.token_buffer.len() < end_index {
                            // Not enough tokens in buffer, just clear the buffer
                            self.token_buffer.clear();
                        } else {
                            // Remove tokens one by one
                            for _ in 0..end_index {
                                if !self.token_buffer.is_empty() {
                                    self.token_buffer.remove(0);
                                }
                            }
                        }
                        
                        // Now add the tokens to the stream in the right order
                        self.token_stream.add_token(be_like_token, be_like_loc);
                        self.token_stream.add_token_with_metadata(
                            identifier_token, 
                            identifier_loc.clone(),
                            metadata
                        );
                        
                        // Add the squad token directly instead of waiting for it
                        if !self.token_buffer.is_empty() && matches!(self.token_buffer[0].0, Token::Squad) {
                            let squad_token = self.token_buffer.remove(0).0;
                            let squad_loc = if !self.token_buffer.is_empty() {
                                self.token_buffer[0].1.clone()
                            } else {
                                identifier_loc_clone
                            };
                            self.token_stream.add_token(squad_token, squad_loc);
                        }
                        
                        return Ok(());
                    }
                } else if bracket_depth > 0 {
                    // Malformed generic syntax - unclosed bracket
                    return Err(Error::new(
                        "Preprocessor",
                        &format!("Unclosed type parameter bracket in generic type declaration for '{}'", type_name),
                        Some(bracket_loc.clone())
                    ));
                }
            }
        }
        
        Ok(())
    }

    /// Processes a generic function declaration pattern
    fn process_generic_function_declaration(&mut self) -> Result<(), Error> {
        // Pattern: slay funcName[TypeParam](args) returnType
        if self.token_buffer.len() >= 5 {
            if let (
                (Token::Slay, _),
                (Token::Identifier(ref func_name), _),
                (Token::LBracket, bracket_loc),
                ..,
            ) = (
                &self.token_buffer[0],
                &self.token_buffer[1],
                &self.token_buffer[2],
                // The rest of the tokens
            ) {
                // Found potential generic function declaration
                let mut type_params = Vec::new();
                let mut bracket_depth = 1;
                let mut end_index = 3;
                
                // Process everything inside the brackets
                while end_index < self.token_buffer.len() {
                    match &self.token_buffer[end_index].0 {
                        Token::LBracket => bracket_depth += 1,
                        Token::RBracket => {
                            bracket_depth -= 1;
                            if bracket_depth == 0 {
                                // We've reached the closing bracket
                                break;
                            }
                        },
                        Token::Identifier(param) => {
                            if bracket_depth == 1 && end_index > 0 {
                                // This is a top-level type parameter
                                type_params.push(param.clone());
                            }
                        },
                        Token::Comma => {
                            // Just skip commas between type parameters
                        },
                        _ => {
                            // Other tokens inside brackets might be part of nested generic types
                        }
                    }
                    end_index += 1;
                }
                
                // Check if we found the closing bracket
                if end_index < self.token_buffer.len() && bracket_depth == 0 {
                    // Look for parameter list after closing bracket
                    if end_index + 1 < self.token_buffer.len() && 
                       matches!(self.token_buffer[end_index + 1].0, Token::LParen) {
                        
                        // Create metadata for the generic function
                        let metadata = TokenMetadata::GenericFunction {
                            name: func_name.clone(),
                            type_params,
                        };
                        
                        // Add the tokens to the stream with the metadata
                        let (slay_token, slay_loc) = self.token_buffer.remove(0);
                        
                        // Check if buffer is empty after removing first token
                        if self.token_buffer.is_empty() {
                            return Err(Error::new(
                                "Preprocessor",
                                "Unexpected end of token buffer during generic function processing",
                                Some(slay_loc.clone())
                            ));
                        }
                        
                        let (identifier_token, identifier_loc) = self.token_buffer.remove(0);
                        
                        // Remove the processed tokens (from LBracket to RBracket)
                        // First check if we have enough tokens
                        if self.token_buffer.len() < end_index {
                            // Not enough tokens in buffer, just clear the buffer
                            self.token_buffer.clear();
                        } else {
                            // Remove tokens one by one
                            for _ in 0..end_index {
                                if !self.token_buffer.is_empty() {
                                    self.token_buffer.remove(0);
                                }
                            }
                        }
                        
                        // Now add the tokens to the stream in the right order
                        self.token_stream.add_token(slay_token, slay_loc);
                        self.token_stream.add_token_with_metadata(
                            identifier_token, 
                            identifier_loc.clone(),
                            metadata
                        );
                        
                        return Ok(());
                    }
                } else if bracket_depth > 0 {
                    // Malformed generic syntax - unclosed bracket
                    return Err(Error::new(
                        "Preprocessor",
                        &format!("Unclosed type parameter bracket in generic function declaration for '{}'", func_name),
                        Some(bracket_loc.clone())
                    ));
                }
            }
        }
        
        Ok(())
    }

    /// Processes a generic function call pattern
    fn process_generic_function_call(&mut self) -> Result<(), Error> {
        // Pattern: funcName[TypeArg](args)
        if self.token_buffer.len() >= 4 {
            if let (
                (Token::Identifier(ref func_name), _),
                (Token::LBracket, bracket_loc),
                ..,
            ) = (
                &self.token_buffer[0],
                &self.token_buffer[1],
                // The rest of the tokens
            ) {
                // Found potential generic function call
                let mut type_args = Vec::new();
                let mut bracket_depth = 1;
                let mut end_index = 2;
                
                // Process everything inside the brackets
                while end_index < self.token_buffer.len() {
                    match &self.token_buffer[end_index].0 {
                        Token::LBracket => bracket_depth += 1,
                        Token::RBracket => {
                            bracket_depth -= 1;
                            if bracket_depth == 0 {
                                // We've reached the closing bracket
                                break;
                            }
                        },
                        Token::Identifier(arg) => {
                            if bracket_depth == 1 && end_index > 0 {
                                // This is a top-level type argument
                                type_args.push(arg.clone());
                            }
                        },
                        Token::Comma => {
                            // Just skip commas between type arguments
                        },
                        _ => {
                            // Other tokens inside brackets might be part of nested generic types
                        }
                    }
                    end_index += 1;
                }
                
                // Check if we found the closing bracket
                if end_index < self.token_buffer.len() && bracket_depth == 0 {
                    // Look for parameter list after closing bracket
                    if end_index + 1 < self.token_buffer.len() && 
                       matches!(self.token_buffer[end_index + 1].0, Token::LParen) {
                        
                        // Create metadata for the generic function call
                        let metadata = TokenMetadata::GenericFunctionCall {
                            name: func_name.clone(),
                            type_args,
                        };
                        
                        // Add the tokens to the stream with the metadata
                        let (identifier_token, identifier_loc) = self.token_buffer.remove(0);
                        
                        // Check if buffer is empty after removing first token
                        if self.token_buffer.is_empty() {
                            return Err(Error::new(
                                "Preprocessor",
                                "Unexpected end of token buffer during generic function call processing",
                                Some(identifier_loc.clone())
                            ));
                        }
                        
                        // Remove the processed tokens (from LBracket to RBracket)
                        // First check if we have enough tokens
                        if self.token_buffer.len() < end_index {
                            // Not enough tokens in buffer, just clear the buffer
                            self.token_buffer.clear();
                        } else {
                            // Remove tokens one by one
                            for _ in 0..end_index {
                                if !self.token_buffer.is_empty() {
                                    self.token_buffer.remove(0);
                                }
                            }
                        }
                        
                        // Add the token with metadata
                        self.token_stream.add_token_with_metadata(
                            identifier_token, 
                            identifier_loc.clone(),
                            metadata
                        );
                        
                        return Ok(());
                    }
                } else if bracket_depth > 0 {
                    // Malformed generic syntax - unclosed bracket
                    return Err(Error::new(
                        "Preprocessor",
                        &format!("Unclosed type argument bracket in generic function call to '{}'", func_name),
                        Some(bracket_loc.clone())
                    ));
                }
            }
        }
        
        Ok(())
    }
}