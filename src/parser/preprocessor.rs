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
        nested_types: Vec<GenericTypeInfo>,
    },
}

/// Information about a generic type parameter, potentially nested
#[derive(Debug, Clone)]
pub struct GenericTypeInfo {
    /// The name of the type parameter
    pub name: String,
    /// Optional nested type parameters if this is a generic type
    pub nested_params: Option<Vec<GenericTypeInfo>>,
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
    
    /// Gets info about nested generic types in the token stream
    pub fn get_nested_generic_info(&self) -> Vec<(String, Vec<String>)> {
        let mut result = Vec::new();
        
        for token_with_context in &self.tokens {
            if let Some(TokenMetadata::NestedGenericType { outer_type, nested_types }) = &token_with_context.metadata {
                // Extract the names of nested types (flattened for simplicity)
                let nested_names = nested_types.iter()
                    .map(|info| info.name.clone())
                    .collect::<Vec<String>>();
                    
                result.push((outer_type.clone(), nested_names));
            }
        }
        
        result
    }
    
    /// Gets detailed info about nested generic types including their full structure
    pub fn get_detailed_nested_generic_info(&self) -> Vec<(String, Vec<GenericTypeInfo>)> {
        let mut result = Vec::new();
        
        for token_with_context in &self.tokens {
            if let Some(TokenMetadata::NestedGenericType { outer_type, nested_types }) = &token_with_context.metadata {
                result.push((outer_type.clone(), nested_types.clone()));
            }
        }
        
        result
    }
    
    /// Extracts the full generic parameter structure for a given type name
    pub fn get_generic_type_structure(&self, type_name: &str) -> Option<Vec<GenericTypeInfo>> {
        for token_with_context in &self.tokens {
            if let Some(TokenMetadata::NestedGenericType { outer_type, nested_types }) = &token_with_context.metadata {
                if outer_type == type_name {
                    return Some(nested_types.clone());
                }
            } else if let Some(TokenMetadata::GenericType { name, type_params }) = &token_with_context.metadata {
                if name == type_name {
                    // Convert simple type params to GenericTypeInfo
                    return Some(type_params.iter()
                        .map(|param| GenericTypeInfo {
                            name: param.clone(),
                            nested_params: None,
                        })
                        .collect());
                }
            }
        }
        None
    }

    /// Checks if the token stream contains separate brackets (which would indicate failure to combine tokens)
    pub fn contains_separate_brackets(&self) -> bool {
        let mut in_brackets = false;
        
        for token_with_context in &self.tokens {
            match token_with_context.token {
                Token::LBracket => {
                    // If we find a left bracket, it should be part of a generic token's metadata
                    if token_with_context.metadata.is_none() {
                        // This is a standalone bracket
                        return true;
                    }
                    
                    // Generic tokens with metadata should not have raw bracket tokens
                    if let Some(metadata) = &token_with_context.metadata {
                        match metadata {
                            TokenMetadata::GenericType { .. } |
                            TokenMetadata::GenericFunction { .. } |
                            TokenMetadata::GenericFunctionCall { .. } |
                            TokenMetadata::NestedGenericType { .. } => {
                                // If this token has generic metadata but is still a bracket token, that's bad
                                return true;
                            }
                        }
                    }
                    
                    in_brackets = true;
                },
                Token::RBracket => {
                    // If we find a right bracket that isn't part of a generic token, return true
                    if token_with_context.metadata.is_none() {
                        return true;
                    }
                    
                    // Generic tokens with metadata should not have raw bracket tokens
                    if let Some(metadata) = &token_with_context.metadata {
                        match metadata {
                            TokenMetadata::GenericType { .. } |
                            TokenMetadata::GenericFunction { .. } |
                            TokenMetadata::GenericFunctionCall { .. } |
                            TokenMetadata::NestedGenericType { .. } => {
                                // If this token has generic metadata but is still a bracket token, that's bad
                                return true;
                            }
                        }
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
    
    /// Parse a potentially nested generic type parameter
    /// For example: "T", "Map[K, V]", "Pair[K, List[T]]", etc.
    #[tracing::instrument(skip(self), level = "debug")]
    fn parse_generic_param(&self, param_str: &str) -> Result<GenericTypeInfo, Error> {
        tracing::debug!(param = param_str, "Parsing generic parameter");
        
        // Trim the parameter string
        let param_str = param_str.trim();
        
        // Check if this has nested generic parameters [T, U, ...]
        if let Some(bracket_idx) = param_str.find('[') {
            // Find the matching closing bracket
            // We need to count brackets to handle nesting correctly
            let mut bracket_depth = 0;
            let mut end_bracket_idx = None;
            
            for (i, c) in param_str.chars().enumerate() {
                match c {
                    '[' => bracket_depth += 1,
                    ']' => {
                        bracket_depth -= 1;
                        if bracket_depth == 0 {
                            end_bracket_idx = Some(i);
                            break;
                        }
                    }
                    _ => {}
                }
            }
            
            if let Some(end_idx) = end_bracket_idx {
                // Extract the base type name and the type parameters
                let base_name = &param_str[0..bracket_idx];
                let params_str = &param_str[bracket_idx+1..end_idx];
                
                tracing::debug!(base = base_name, params = params_str, "Found nested generic type");
                
                // Parse the nested parameters
                let nested_params = self.parse_param_list(params_str)?;
                
                return Ok(GenericTypeInfo {
                    name: base_name.trim().to_string(),
                    nested_params: Some(nested_params),
                });
            } else {
                return Err(Error::from_str(&format!(
                    "Malformed generic parameter: unclosed bracket in {}",
                    param_str
                )));
            }
        }
        
        // If there are no nested parameters, just return the name
        tracing::debug!(name = param_str, "Simple type parameter");
        Ok(GenericTypeInfo {
            name: param_str.trim().to_string(),
            nested_params: None,
        })
    }
    
    /// Parse a comma-separated list of type parameters, handling nested generics
    #[tracing::instrument(skip(self), level = "debug")]
    fn parse_param_list(&self, params_str: &str) -> Result<Vec<GenericTypeInfo>, Error> {
        tracing::debug!(params = params_str, "Parsing parameter list");
        
        let mut result = Vec::new();
        let mut current_param = String::new();
        let mut bracket_depth = 0;
        let mut angle_bracket_depth = 0; // For handling angle brackets in complex types
        
        // Process character by character to handle nested commas
        for c in params_str.chars() {
            match c {
                '[' => {
                    bracket_depth += 1;
                    current_param.push(c);
                },
                ']' => {
                    if bracket_depth > 0 {
                        bracket_depth -= 1;
                        current_param.push(c);
                    } else {
                        // Unmatched closing bracket - this is an error
                        return Err(Error::from_str(&format!(
                            "Unmatched closing bracket in parameter list: {}",
                            params_str
                        )));
                    }
                },
                '<' => {
                    angle_bracket_depth += 1;
                    current_param.push(c);
                },
                '>' => {
                    if angle_bracket_depth > 0 {
                        angle_bracket_depth -= 1;
                        current_param.push(c);
                    } else {
                        // Unmatched closing angle bracket - this is an error
                        return Err(Error::from_str(&format!(
                            "Unmatched closing angle bracket in parameter list: {}",
                            params_str
                        )));
                    }
                },
                ',' => {
                    if bracket_depth == 0 && angle_bracket_depth == 0 {
                        // This comma separates top-level parameters
                        if !current_param.trim().is_empty() {
                            tracing::debug!(param = current_param, "Found parameter");
                            let param_info = self.parse_generic_param(&current_param)?;
                            result.push(param_info);
                            current_param = String::new();
                        }
                    } else {
                        // This comma is within nested parameters, keep it
                        current_param.push(c);
                    }
                },
                // Handle whitespace more intelligently
                ' ' | '\t' | '\n' | '\r' => {
                    // Skip consecutive whitespace in parameter lists
                    if !current_param.is_empty() && !current_param.ends_with(|ch: char| ch.is_whitespace()) {
                        current_param.push(' '); // Normalize all whitespace to a single space
                    }
                },
                _ => current_param.push(c),
            }
        }
        
        // Ensure all brackets are closed
        if bracket_depth > 0 {
            return Err(Error::from_str(&format!(
                "Unclosed square brackets in parameter list: {}",
                params_str
            )));
        }
        
        if angle_bracket_depth > 0 {
            return Err(Error::from_str(&format!(
                "Unclosed angle brackets in parameter list: {}",
                params_str
            )));
        }
        
        // Handle the last parameter if there is one
        if !current_param.trim().is_empty() {
            tracing::debug!(param = current_param, "Found final parameter");
            let param_info = self.parse_generic_param(&current_param)?;
            result.push(param_info);
        }
        
        tracing::debug!(count = result.len(), "Parsed parameter list");
        Ok(result)
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
    #[tracing::instrument(skip(self), fields(buffer_size = self.token_buffer.len()), level = "debug")]
    fn process_generic_type_declaration(&mut self) -> Result<(), Error> {
        tracing::debug!("Processing potential generic type declaration");
        
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
                tracing::debug!(type_name = type_name, "Found potential generic type declaration");
                
                // Found potential generic type declaration
                let mut bracket_depth = 1;
                let mut end_index = 3;
                let mut param_str = String::new();
                
                // Process everything inside the brackets to build a parameter string
                while end_index < self.token_buffer.len() {
                    match &self.token_buffer[end_index].0 {
                        Token::LBracket => {
                            bracket_depth += 1;
                            param_str.push('[');
                            tracing::trace!("Added '[', new depth: {}", bracket_depth);
                        },
                        Token::RBracket => {
                            bracket_depth -= 1;
                            tracing::trace!("Found ']', new depth: {}", bracket_depth);
                            if bracket_depth == 0 {
                                // We've reached the closing bracket
                                tracing::debug!("Found closing bracket at index {}", end_index);
                                break;
                            }
                            param_str.push(']');
                        },
                        Token::Identifier(param) => {
                            // Add the parameter name to our string
                            tracing::trace!(param = param, "Adding parameter to string");
                            param_str.push_str(param);
                        },
                        Token::Comma => {
                            tracing::trace!("Adding comma to parameter string");
                            param_str.push(',');
                        },
                        Token::Less => {
                            // Handle angle brackets for additional types like channels
                            tracing::trace!("Adding '<' to parameter string");
                            param_str.push('<');
                        },
                        Token::Greater => {
                            // Handle angle brackets for additional types like channels
                            tracing::trace!("Adding '>' to parameter string");
                            param_str.push('>');
                        },
                        Token::At => {
                            // Handle pointer types
                            tracing::trace!("Adding '@' to parameter string");
                            param_str.push('@');
                        },
                        token => {
                            // For other token types, add more detailed handling
                            tracing::trace!(token = ?token, "Unknown token in parameter list");
                        }
                    }
                    end_index += 1;
                }
                
                // Check if we found the closing bracket
                if end_index < self.token_buffer.len() && bracket_depth == 0 {
                    // Look for 'squad' keyword after closing bracket
                    if end_index + 1 < self.token_buffer.len() && 
                       matches!(self.token_buffer[end_index + 1].0, Token::Squad) {
                        tracing::debug!(param_str = param_str, "Found complete generic type declaration with squad");
                        
                        // Parse the parameter string to handle nested generics
                        let type_param_info = match self.parse_param_list(&param_str) {
                            Ok(params) => params,
                            Err(e) => {
                                tracing::error!(error = ?e, "Error parsing generic type parameters");
                                return Err(Error::new(
                                    "Preprocessor",
                                    &format!("Error parsing generic type parameters: {}", e),
                                    Some(bracket_loc.clone())
                                ));
                            }
                        };
                        
                        // Extract simple parameter names for metadata
                        let type_params = type_param_info.iter()
                            .map(|info| info.name.clone())
                            .collect::<Vec<String>>();
                        
                        tracing::debug!(
                            type_name = type_name,
                            param_count = type_params.len(),
                            params = ?type_params,
                            "Parsed generic type parameters"
                        );
                        
                        // Check if we have any nested parameters
                        let has_nested = type_param_info.iter().any(|info| info.nested_params.is_some());
                        
                        // Create appropriate metadata based on whether we have nested parameters
                        let metadata = if has_nested {
                            tracing::debug!("Creating nested generic type metadata");
                            TokenMetadata::NestedGenericType {
                                outer_type: type_name.clone(),
                                nested_types: type_param_info,
                            }
                        } else {
                            tracing::debug!("Creating simple generic type metadata");
                            TokenMetadata::GenericType {
                                name: type_name.clone(),
                                type_params,
                            }
                        };
                        
                        // Add the tokens to the stream with the metadata
                        let (be_like_token, be_like_loc) = self.token_buffer.remove(0);
                        
                        // Check if buffer is empty after removing first token
                        if self.token_buffer.is_empty() {
                            tracing::error!("Unexpected end of token buffer during generic type processing");
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
                            tracing::warn!("Buffer smaller than expected, clearing remaining tokens");
                            self.token_buffer.clear();
                        } else {
                            // Remove tokens one by one
                            tracing::debug!(count = end_index, "Removing processed tokens");
                            for _ in 0..end_index {
                                if !self.token_buffer.is_empty() {
                                    self.token_buffer.remove(0);
                                }
                            }
                        }
                        
                        // Now add the tokens to the stream in the right order
                        tracing::debug!("Adding tokens to stream with metadata");
                        self.token_stream.add_token(be_like_token, be_like_loc);
                        self.token_stream.add_token_with_metadata(
                            identifier_token, 
                            identifier_loc.clone(),
                            metadata
                        );
                        
                        // Add the squad token directly instead of waiting for it
                        if !self.token_buffer.is_empty() && matches!(self.token_buffer[0].0, Token::Squad) {
                            tracing::debug!("Adding squad token");
                            let squad_token = self.token_buffer.remove(0).0;
                            let squad_loc = if !self.token_buffer.is_empty() {
                                self.token_buffer[0].1.clone()
                            } else {
                                identifier_loc_clone
                            };
                            self.token_stream.add_token(squad_token, squad_loc);
                        }
                        
                        tracing::debug!("Successfully processed generic type declaration");
                        return Ok(());
                    }
                } else if bracket_depth > 0 {
                    // Malformed generic syntax - unclosed bracket
                    tracing::error!("Unclosed type parameter bracket in generic type declaration");
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
    #[tracing::instrument(skip(self), fields(buffer_size = self.token_buffer.len()), level = "debug")]
    fn process_generic_function_declaration(&mut self) -> Result<(), Error> {
        tracing::debug!("Processing potential generic function declaration");
        
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
                tracing::debug!(func_name = func_name, "Found potential generic function declaration");
                
                // Found potential generic function declaration
                let mut bracket_depth = 1;
                let mut end_index = 3;
                let mut param_str = String::new();
                
                // Process everything inside the brackets to build a parameter string
                while end_index < self.token_buffer.len() {
                    match &self.token_buffer[end_index].0 {
                        Token::LBracket => {
                            bracket_depth += 1;
                            param_str.push('[');
                            tracing::trace!("Added '[', new depth: {}", bracket_depth);
                        },
                        Token::RBracket => {
                            bracket_depth -= 1;
                            tracing::trace!("Found ']', new depth: {}", bracket_depth);
                            if bracket_depth == 0 {
                                // We've reached the closing bracket
                                tracing::debug!("Found closing bracket at index {}", end_index);
                                break;
                            }
                            param_str.push(']');
                        },
                        Token::Identifier(param) => {
                            // Add the parameter name to our string
                            tracing::trace!(param = param, "Adding parameter to string");
                            param_str.push_str(param);
                        },
                        Token::Comma => {
                            tracing::trace!("Adding comma to parameter string");
                            param_str.push(',');
                        },
                        Token::Less => {
                            // Handle angle brackets for additional types like channels
                            tracing::trace!("Adding '<' to parameter string");
                            param_str.push('<');
                        },
                        Token::Greater => {
                            // Handle angle brackets for additional types like channels
                            tracing::trace!("Adding '>' to parameter string");
                            param_str.push('>');
                        },
                        Token::At => {
                            // Handle pointer types
                            tracing::trace!("Adding '@' to parameter string");
                            param_str.push('@');
                        },
                        token => {
                            // For other token types, add more detailed handling
                            tracing::trace!(token = ?token, "Unknown token in parameter list");
                        }
                    }
                    end_index += 1;
                }
                
                // Check if we found the closing bracket
                if end_index < self.token_buffer.len() && bracket_depth == 0 {
                    // Look for parameter list after closing bracket
                    if end_index + 1 < self.token_buffer.len() && 
                       matches!(self.token_buffer[end_index + 1].0, Token::LParen) {
                        tracing::debug!(param_str = param_str, "Found complete generic function declaration with params");
                        
                        // Parse the parameter string to handle nested generics
                        let type_param_info = match self.parse_param_list(&param_str) {
                            Ok(params) => params,
                            Err(e) => {
                                tracing::error!(error = ?e, "Error parsing generic function parameters");
                                return Err(Error::new(
                                    "Preprocessor",
                                    &format!("Error parsing generic function parameters: {}", e),
                                    Some(bracket_loc.clone())
                                ));
                            }
                        };
                        
                        // Extract simple parameter names for metadata
                        let type_params = type_param_info.iter()
                            .map(|info| info.name.clone())
                            .collect::<Vec<String>>();
                        
                        tracing::debug!(
                            func_name = func_name,
                            param_count = type_params.len(),
                            params = ?type_params,
                            has_nested = type_param_info.iter().any(|info| info.nested_params.is_some()),
                            "Parsed generic function parameters"
                        );
                        
                        // Create metadata for the generic function
                        // We now handle nested generics in functions too
                        let metadata = TokenMetadata::GenericFunction {
                            name: func_name.clone(),
                            type_params,
                        };
                        
                        // Add the tokens to the stream with the metadata
                        let (slay_token, slay_loc) = self.token_buffer.remove(0);
                        
                        // Check if buffer is empty after removing first token
                        if self.token_buffer.is_empty() {
                            tracing::error!("Unexpected end of token buffer during generic function processing");
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
                            tracing::warn!("Buffer smaller than expected, clearing remaining tokens");
                            self.token_buffer.clear();
                        } else {
                            // Remove tokens one by one
                            tracing::debug!(count = end_index, "Removing processed tokens");
                            for _ in 0..end_index {
                                if !self.token_buffer.is_empty() {
                                    self.token_buffer.remove(0);
                                }
                            }
                        }
                        
                        // Now add the tokens to the stream in the right order
                        tracing::debug!("Adding tokens to stream with metadata");
                        self.token_stream.add_token(slay_token, slay_loc);
                        self.token_stream.add_token_with_metadata(
                            identifier_token, 
                            identifier_loc.clone(),
                            metadata
                        );
                        
                        tracing::debug!("Successfully processed generic function declaration");
                        return Ok(());
                    }
                } else if bracket_depth > 0 {
                    // Malformed generic syntax - unclosed bracket
                    tracing::error!("Unclosed type parameter bracket in generic function declaration");
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
    #[tracing::instrument(skip(self), fields(buffer_size = self.token_buffer.len()), level = "debug")]
    fn process_generic_function_call(&mut self) -> Result<(), Error> {
        tracing::debug!("Processing potential generic function call");
        
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
                tracing::debug!(func_name = func_name, "Found potential generic function call");
                
                // Found potential generic function call
                let mut bracket_depth = 1;
                let mut end_index = 2;
                let mut param_str = String::new();
                
                // Process everything inside the brackets to build a parameter string
                while end_index < self.token_buffer.len() {
                    match &self.token_buffer[end_index].0 {
                        Token::LBracket => {
                            bracket_depth += 1;
                            param_str.push('[');
                            tracing::trace!("Added '[', new depth: {}", bracket_depth);
                        },
                        Token::RBracket => {
                            bracket_depth -= 1;
                            tracing::trace!("Found ']', new depth: {}", bracket_depth);
                            if bracket_depth == 0 {
                                // We've reached the closing bracket
                                tracing::debug!("Found closing bracket at index {}", end_index);
                                break;
                            }
                            param_str.push(']');
                        },
                        Token::Identifier(arg) => {
                            // Add the parameter name to our string
                            tracing::trace!(arg = arg, "Adding type argument to string");
                            param_str.push_str(arg);
                        },
                        Token::Comma => {
                            tracing::trace!("Adding comma to parameter string");
                            param_str.push(',');
                        },
                        Token::Less => {
                            // Handle angle brackets for additional types like channels
                            tracing::trace!("Adding '<' to parameter string");
                            param_str.push('<');
                        },
                        Token::Greater => {
                            // Handle angle brackets for additional types like channels
                            tracing::trace!("Adding '>' to parameter string");
                            param_str.push('>');
                        },
                        Token::At => {
                            // Handle pointer types
                            tracing::trace!("Adding '@' to parameter string");
                            param_str.push('@');
                        },
                        token => {
                            // For other token types, add more detailed handling
                            tracing::trace!(token = ?token, "Unknown token in type argument list");
                        }
                    }
                    end_index += 1;
                }
                
                // Check if we found the closing bracket
                if end_index < self.token_buffer.len() && bracket_depth == 0 {
                    // Look for parameter list after closing bracket
                    if end_index + 1 < self.token_buffer.len() && 
                       matches!(self.token_buffer[end_index + 1].0, Token::LParen) {
                        tracing::debug!(param_str = param_str, "Found complete generic function call with args");
                        
                        // Parse the parameter string to handle nested generics
                        let type_param_info = match self.parse_param_list(&param_str) {
                            Ok(params) => params,
                            Err(e) => {
                                tracing::error!(error = ?e, "Error parsing generic function call parameters");
                                return Err(Error::new(
                                    "Preprocessor",
                                    &format!("Error parsing generic function call parameters: {}", e),
                                    Some(bracket_loc.clone())
                                ));
                            }
                        };
                        
                        // Extract simple parameter names for metadata
                        let type_args = type_param_info.iter()
                            .map(|info| info.name.clone())
                            .collect::<Vec<String>>();
                        
                        tracing::debug!(
                            func_name = func_name,
                            arg_count = type_args.len(),
                            args = ?type_args,
                            has_nested = type_param_info.iter().any(|info| info.nested_params.is_some()),
                            "Parsed generic function call type arguments"
                        );
                        
                        // Create metadata for the generic function call
                        let metadata = TokenMetadata::GenericFunctionCall {
                            name: func_name.clone(),
                            type_args,
                        };
                        
                        // Add the tokens to the stream with the metadata
                        let (identifier_token, identifier_loc) = self.token_buffer.remove(0);
                        
                        // Check if buffer is empty after removing first token
                        if self.token_buffer.is_empty() {
                            tracing::error!("Unexpected end of token buffer during generic function call processing");
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
                            tracing::warn!("Buffer smaller than expected, clearing remaining tokens");
                            self.token_buffer.clear();
                        } else {
                            // Remove tokens one by one
                            tracing::debug!(count = end_index, "Removing processed tokens");
                            for _ in 0..end_index {
                                if !self.token_buffer.is_empty() {
                                    self.token_buffer.remove(0);
                                }
                            }
                        }
                        
                        // Add the token with metadata
                        tracing::debug!("Adding token to stream with metadata");
                        self.token_stream.add_token_with_metadata(
                            identifier_token, 
                            identifier_loc.clone(),
                            metadata
                        );
                        
                        tracing::debug!("Successfully processed generic function call");
                        return Ok(());
                    }
                } else if bracket_depth > 0 {
                    // Malformed generic syntax - unclosed bracket
                    tracing::error!("Unclosed type argument bracket in generic function call");
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