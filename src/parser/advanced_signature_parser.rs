//! Advanced function signature parsing for CURSED
//!
//! This module extends the parser with comprehensive support for:
//! - Variadic parameters (...syntax)
//! - Complex generic bounds and where clauses
//! - Tuple types in parameters and returns
//! - Function pointer types
//! - Enhanced array/slice type annotations
//! - Documentation generation improvements

use crate::error::CursedError;
use crate::error_types::Error;
use crate::lexer::{Token, TokenKind};
use crate::ast::{
    Type, Parameter, TypeParameter, WhereClause, TypeConstraint, 
    Expression, Visibility, FunctionStatement
};

/// Enhanced parameter with additional features
#[derive(Debug, Clone)]
pub struct AdvancedParameter {
    pub name: String,
    pub param_type: Option<Type>,
    pub is_mutable: bool,
    pub is_variadic: bool,
    pub default_value: Option<Expression>,
    pub documentation: Option<String>,
}

/// Enhanced function signature with advanced features
#[derive(Debug, Clone)]
pub struct AdvancedFunctionSignature {
    pub name: String,
    pub type_parameters: Vec<AdvancedTypeParameter>,
    pub parameters: Vec<AdvancedParameter>,
    pub return_type: Option<Type>,
    pub where_clauses: Vec<WhereClause>,
    pub visibility: Visibility,
    pub is_async: bool,
    pub is_unsafe: bool,
    pub documentation: Option<String>,
}

/// Advanced type parameter with complex bounds
#[derive(Debug, Clone)]
pub struct AdvancedTypeParameter {
    pub name: String,
    pub bounds: Vec<TypeBound>,
    pub default_type: Option<Type>,
    pub variance: AdvancedTypeVariance,
}

/// Type bounds for generic parameters
#[derive(Debug, Clone)]
pub enum TypeBound {
    Trait(String),
    Lifetime(String),
    Sized,
    Send,
    Sync,
    Copy,
    Clone,
    Debug,
    Display,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Custom(String),
}

/// Type variance for generic parameters
#[derive(Debug, Clone)]
pub enum AdvancedTypeVariance {
    Invariant,
    Covariant,
    Contravariant,
}

/// Enhanced function type for function pointers
#[derive(Debug, Clone)]
pub struct FunctionType {
    pub parameters: Vec<Type>,
    pub return_type: Box<Type>,
    pub calling_convention: CallingConvention,
    pub is_unsafe: bool,
}

/// Calling conventions for function pointers
#[derive(Debug, Clone)]
pub enum CallingConvention {
    Default,
    C,
    Stdcall,
    Fastcall,
    Vectorcall,
    Rust,
}

/// Advanced parser for function signatures
pub struct AdvancedSignatureParser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> AdvancedSignatureParser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, current: 0 }
    }

    /// Parse a complete function signature with all advanced features
    pub fn parse_advanced_function_signature(&mut self) -> Result<AdvancedFunctionSignature, CursedError> {
        // Parse optional documentation comment
        let documentation = self.parse_documentation()?;

        // Parse visibility modifier
        let visibility = self.parse_visibility()?;

        // Parse optional 'async' keyword
        let is_async = self.consume_if_matches(&TokenKind::Async);

        // Parse optional 'unsafe' keyword
        let is_unsafe = self.consume_if_matches(&TokenKind::Unsafe);

        // Parse 'slay' keyword
        self.expect_token(TokenKind::Slay)?;

        // Parse function name
        let name = self.parse_identifier()?;

        // Parse generic type parameters with bounds
        let type_parameters = self.parse_enhanced_type_parameters()?;

        // Parse function parameters (including variadic)
        self.expect_token(TokenKind::LeftParen)?;
        let parameters = self.parse_advanced_parameters()?;
        self.expect_token(TokenKind::RightParen)?;

        // Parse return type with tuple support
        let return_type = self.parse_return_type()?;

        // Parse where clauses
        let where_clauses = self.parse_where_clauses()?;

        Ok(AdvancedFunctionSignature {
            name,
            type_parameters,
            parameters,
            return_type,
            where_clauses,
            visibility,
            is_async,
            is_unsafe,
            documentation,
        })
    }

    /// Parse documentation comments preceding the function
    fn parse_documentation(&mut self) -> Result<Option<String>, CursedError> {
        let mut doc_lines = Vec::new();

        while let Some(token) = self.current_token() {
            match &token.kind {
                TokenKind::Comment(text) if text.starts_with("///") => {
                    // Doc comment
                    doc_lines.push(text[3..].trim().to_string());
                    self.advance();
                }
                TokenKind::Comment(text) if text.starts_with("//!") => {
                    // Module-level doc comment
                    doc_lines.push(text[3..].trim().to_string());
                    self.advance();
                }
                _ => break,
            }
        }

        if doc_lines.is_empty() {
            Ok(None)
        } else {
            Ok(Some(doc_lines.join("\n")))
        }
    }

    /// Parse enhanced type parameters with complex bounds
    fn parse_enhanced_type_parameters(&mut self) -> Result<Vec<AdvancedTypeParameter>, CursedError> {
        if !self.current_token_is(&TokenKind::Less) {
            return Ok(Vec::new());
        }

        self.advance(); // consume '<'
        let mut type_parameters = Vec::new();

        while !self.current_token_is(&TokenKind::Greater) && !self.is_at_end() {
            let type_param = self.parse_enhanced_type_parameter()?;
            type_parameters.push(type_param);

            if self.current_token_is(&TokenKind::Comma) {
                self.advance();
            } else if !self.current_token_is(&TokenKind::Greater) {
                return Err(CursedError::Parse("Expected ',' or '>' in type parameter list".to_string()));
            }
        }

        self.expect_token(TokenKind::Greater)?;
        Ok(type_parameters)
    }

    /// Parse a single enhanced type parameter
    fn parse_enhanced_type_parameter(&mut self) -> Result<AdvancedTypeParameter, CursedError> {
        let name = self.parse_identifier()?;

        // Parse optional bounds
        let mut bounds = Vec::new();
        if self.current_token_is(&TokenKind::Colon) {
            self.advance(); // consume ':'
            bounds = self.parse_type_bounds()?;
        }

        // Parse optional default type
        let default_type = if self.current_token_is(&TokenKind::Assign) {
            self.advance(); // consume '='
            Some(self.parse_type()?)
        } else {
            None
        };

        Ok(AdvancedTypeParameter {
            name,
            bounds,
            default_type,
            variance: AdvancedTypeVariance::Invariant, // Default variance
        })
    }

    /// Parse type bounds for generic parameters
    fn parse_type_bounds(&mut self) -> Result<Vec<TypeBound>, CursedError> {
        let mut bounds = Vec::new();

        loop {
            if let Some(token) = self.current_token() {
                match &token.kind {
                    TokenKind::Identifier => {
                        let bound_name = token.lexeme.clone();
                        self.advance();

                        // Map common trait names to bounds
                        let bound = match bound_name.as_str() {
                            "Sized" => TypeBound::Sized,
                            "Send" => TypeBound::Send,
                            "Sync" => TypeBound::Sync,
                            "Copy" => TypeBound::Copy,
                            "Clone" => TypeBound::Clone,
                            "Debug" => TypeBound::Debug,
                            "Display" => TypeBound::Display,
                            "Default" => TypeBound::Default,
                            "PartialEq" => TypeBound::PartialEq,
                            "Eq" => TypeBound::Eq,
                            "PartialOrd" => TypeBound::PartialOrd,
                            "Ord" => TypeBound::Ord,
                            "Hash" => TypeBound::Hash,
                            _ => TypeBound::Custom(bound_name),
                        };
                        bounds.push(bound);
                    }
                    _ => break,
                }

                if self.current_token_is(&TokenKind::Plus) {
                    self.advance(); // consume '+'
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(bounds)
    }

    /// Parse advanced function parameters with variadic support
    fn parse_advanced_parameters(&mut self) -> Result<Vec<AdvancedParameter>, CursedError> {
        let mut parameters = Vec::new();

        while !self.current_token_is(&TokenKind::RightParen) && !self.is_at_end() {
            let param = self.parse_advanced_parameter()?;
            parameters.push(param);

            if self.current_token_is(&TokenKind::Comma) {
                self.advance();
            } else if !self.current_token_is(&TokenKind::RightParen) {
                return Err(CursedError::Parse("Expected ',' or ')' in parameter list".to_string()));
            }
        }

        Ok(parameters)
    }

    /// Parse a single advanced parameter
    fn parse_advanced_parameter(&mut self) -> Result<AdvancedParameter, CursedError> {
        // Check for variadic parameter (...)
        if self.current_token_matches_sequence(&[TokenKind::Dot, TokenKind::Dot, TokenKind::Dot]) {
            self.advance(); // consume first '.'
            self.advance(); // consume second '.'
            self.advance(); // consume third '.'

            let name = self.parse_identifier()?;
            let param_type = Some(self.parse_type()?);

            return Ok(AdvancedParameter {
                name,
                param_type,
                is_mutable: false,
                is_variadic: true,
                default_value: None,
                documentation: None,
            });
        }

        // Parse optional mutability
        let is_mutable = self.consume_if_matches(&TokenKind::Mut);

        // Parse parameter name
        let name = self.parse_identifier()?;

        // Parse parameter type
        let param_type = Some(self.parse_type()?);

        // Parse optional default value
        let default_value = if self.current_token_is(&TokenKind::Assign) {
            self.advance(); // consume '='
            Some(self.parse_expression()?)
        } else {
            None
        };

        Ok(AdvancedParameter {
            name,
            param_type,
            is_mutable,
            is_variadic: false,
            default_value,
            documentation: None,
        })
    }

    /// Parse return type with tuple support
    fn parse_return_type(&mut self) -> Result<Option<Type>, CursedError> {
        if !self.current_token_is(&TokenKind::Arrow) {
            return Ok(None);
        }

        self.advance(); // consume '->'
        Ok(Some(self.parse_type()?))
    }

    /// Parse enhanced type annotations
    fn parse_type(&mut self) -> Result<Type, CursedError> {
        // Handle tuple types
        if self.current_token_is(&TokenKind::LeftParen) {
            return self.parse_tuple_type();
        }

        // Handle function pointer types
        if self.current_token_is(&TokenKind::Fn) {
            return self.parse_function_pointer_type();
        }

        // Handle array/slice types with enhanced syntax
        if self.current_token_is(&TokenKind::LeftBracket) {
            return self.parse_array_or_slice_type();
        }

        // Handle pointer types
        if self.current_token_is(&TokenKind::Star) {
            self.advance(); // consume '*'
            let inner_type = self.parse_type()?;
            return Ok(Type::Pointer(Box::new(inner_type)));
        }

        // Parse basic types
        self.parse_basic_type()
    }

    /// Parse tuple types (T, U, V)
    fn parse_tuple_type(&mut self) -> Result<Type, CursedError> {
        self.expect_token(TokenKind::LeftParen)?;

        let mut types = Vec::new();
        while !self.current_token_is(&TokenKind::RightParen) && !self.is_at_end() {
            types.push(self.parse_type()?);

            if self.current_token_is(&TokenKind::Comma) {
                self.advance();
            } else if !self.current_token_is(&TokenKind::RightParen) {
                return Err(CursedError::Parse("Expected ',' or ')' in tuple type".to_string()));
            }
        }

        self.expect_token(TokenKind::RightParen)?;
        Ok(Type::Tuple(types))
    }

    /// Parse function pointer types
    fn parse_function_pointer_type(&mut self) -> Result<Type, CursedError> {
        self.expect_token(TokenKind::Fn)?;
        self.expect_token(TokenKind::LeftParen)?;

        let mut param_types = Vec::new();
        while !self.current_token_is(&TokenKind::RightParen) && !self.is_at_end() {
            param_types.push(self.parse_type()?);

            if self.current_token_is(&TokenKind::Comma) {
                self.advance();
            } else if !self.current_token_is(&TokenKind::RightParen) {
                return Err(CursedError::Parse("Expected ',' or ')' in function type".to_string()));
            }
        }

        self.expect_token(TokenKind::RightParen)?;

        let return_type = if self.current_token_is(&TokenKind::Arrow) {
            self.advance(); // consume '->'
            Box::new(self.parse_type()?)
        } else {
            Box::new(Type::Void)
        };

        Ok(Type::Function(param_types, return_type))
    }

    /// Parse enhanced array/slice types
    fn parse_array_or_slice_type(&mut self) -> Result<Type, CursedError> {
        self.expect_token(TokenKind::LeftBracket)?;

        // Check if this is a slice []T or array [N]T
        if self.current_token_is(&TokenKind::RightBracket) {
            // Slice type []T
            self.advance(); // consume ']'
            let element_type = self.parse_type()?;
            Ok(Type::Slice(Box::new(element_type)))
        } else {
            // Array type [N]T with size expression
            let size_expr = self.parse_expression()?;
            self.expect_token(TokenKind::RightBracket)?;
            let element_type = self.parse_type()?;
            Ok(Type::Array(Box::new(element_type), Some(Box::new(size_expr))))
        }
    }

    /// Parse basic CURSED types
    fn parse_basic_type(&mut self) -> Result<Type, CursedError> {
        if let Some(token) = self.current_token() {
            let type_result = match &token.kind {
                TokenKind::Normie => Ok(Type::Normie),
                TokenKind::Smol => Ok(Type::Smol),
                TokenKind::Mid => Ok(Type::Mid),
                TokenKind::Thicc => Ok(Type::Thicc),
                TokenKind::Snack => Ok(Type::Snack),
                TokenKind::Meal => Ok(Type::Meal),
                TokenKind::Tea => Ok(Type::Tea),
                TokenKind::Lit => Ok(Type::Lit),
                TokenKind::Sip => Ok(Type::Sip),
                TokenKind::Byte => Ok(Type::Byte),
                TokenKind::Rune => Ok(Type::Rune),
                TokenKind::Extra => Ok(Type::Extra),
                TokenKind::Identifier => Ok(Type::Custom(token.lexeme.clone())),
                _ => Err(CursedError::Parse(format!("Expected type, found {:?}", token.kind))),
            };

            if type_result.is_ok() {
                self.advance();
            }

            type_result
        } else {
            Err(CursedError::Parse("Expected type".to_string()))
        }
    }

    /// Parse where clauses for complex constraints
    fn parse_where_clauses(&mut self) -> Result<Vec<WhereClause>, CursedError> {
        let mut where_clauses = Vec::new();

        while self.current_token_is_identifier("where") {
            self.advance(); // consume 'where'
            let constraints = self.parse_type_constraints()?;
            where_clauses.push(WhereClause { constraints });

            // Allow multiple where clauses separated by commas
            if self.current_token_is(&TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }

        Ok(where_clauses)
    }

    /// Parse type constraints for where clauses
    fn parse_type_constraints(&mut self) -> Result<Vec<TypeConstraint>, CursedError> {
        let mut constraints = Vec::new();

        while !self.is_at_end() && !self.current_token_is(&TokenKind::LeftBrace) {
            let type_name = self.parse_identifier()?;
            self.expect_token(TokenKind::Colon)?;
            let bounds = self.parse_type_bounds()?
                .into_iter()
                .map(|bound| match bound {
                    TypeBound::Custom(name) => name,
                    TypeBound::Clone => "Clone".to_string(),
                    TypeBound::Debug => "Debug".to_string(),
                    TypeBound::Display => "Display".to_string(),
                    TypeBound::Default => "Default".to_string(),
                    TypeBound::PartialEq => "PartialEq".to_string(),
                    TypeBound::Eq => "Eq".to_string(),
                    TypeBound::PartialOrd => "PartialOrd".to_string(),
                    TypeBound::Ord => "Ord".to_string(),
                    TypeBound::Hash => "Hash".to_string(),
                    TypeBound::Send => "Send".to_string(),
                    TypeBound::Sync => "Sync".to_string(),
                    TypeBound::Copy => "Copy".to_string(),
                    TypeBound::Sized => "Sized".to_string(),
                    TypeBound::Trait(name) => name,
                    TypeBound::Lifetime(name) => name,
                })
                .collect();

            constraints.push(TypeConstraint { type_name, bounds });

            if self.current_token_is(&TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }

        Ok(constraints)
    }

    /// Parse visibility modifier
    fn parse_visibility(&mut self) -> Result<Visibility, CursedError> {
        if let Some(token) = self.current_token() {
            match &token.kind {
                TokenKind::Public => {
                    self.advance();
                    Ok(Visibility::Public)
                }
                TokenKind::Private => {
                    self.advance();
                    Ok(Visibility::Private)
                }
                _ => Ok(Visibility::Public), // Default visibility
            }
        } else {
            Ok(Visibility::Public)
        }
    }

    // Helper methods for token management
    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || 
        self.current_token().map(|t| &t.kind) == Some(&TokenKind::Eof)
    }

    fn current_token_is(&self, kind: &TokenKind) -> bool {
        self.current_token().map(|t| &t.kind) == Some(kind)
    }

    fn current_token_is_identifier(&self, expected: &str) -> bool {
        if let Some(token) = self.current_token() {
            token.kind == TokenKind::Identifier && token.lexeme == expected
        } else {
            false
        }
    }

    fn consume_if_matches(&mut self, kind: &TokenKind) -> bool {
        if self.current_token_is(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn current_token_matches_sequence(&self, sequence: &[TokenKind]) -> bool {
        for (i, expected_kind) in sequence.iter().enumerate() {
            if let Some(token) = self.tokens.get(self.current + i) {
                if &token.kind != expected_kind {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    fn expect_token(&mut self, expected: TokenKind) -> Result<(), CursedError> {
        if let Some(token) = self.current_token() {
            if token.kind == expected {
                self.advance();
                Ok(())
            } else {
                Err(CursedError::Parse(format!("Expected {:?}, found {:?}", expected, token.kind)))
            }
        } else {
            Err(CursedError::Parse(format!("Expected {:?}, found EOF", expected)))
        }
    }

    fn parse_identifier(&mut self) -> Result<String, CursedError> {
        if let Some(token) = self.current_token() {
            if token.kind == TokenKind::Identifier {
                let name = token.lexeme.clone();
                self.advance();
                Ok(name)
            } else {
                Err(CursedError::Parse(format!("Expected identifier, found {:?}", token.kind)))
            }
        } else {
            Err(CursedError::Parse("Expected identifier, found EOF".to_string()))
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, CursedError> {
        // Simplified expression parsing for array sizes
        if let Some(token) = self.current_token() {
            match &token.kind {
                TokenKind::IntegerLiteral => {
                    let value = token.lexeme.parse::<i64>()
                        .map_err(|_| CursedError::Parse("Invalid integer literal".to_string()))?;
                    self.advance();
                    Ok(Expression::Integer(value))
                }
                TokenKind::Identifier => {
                    let name = token.lexeme.clone();
                    self.advance();
                    Ok(Expression::Identifier(name))
                }
                _ => Err(CursedError::Parse("Expected expression".to_string())),
            }
        } else {
            Err(CursedError::Parse("Expected expression, found EOF".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn create_tokens(input: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(input.to_string());
        let mut tokens = Vec::new();
        
        loop {
            match lexer.next_token() {
                Ok(token) => {
                    let is_eof = token.kind == TokenKind::Eof;
                    tokens.push(token);
                    if is_eof { break; }
                }
                Err(_) => break,
            }
        }
        
        tokens
    }

    #[test]
    fn test_parse_variadic_function() {
        let input = "slay printf(format tea, ...args normie) {  }";
        let tokens = create_tokens(input);
        let mut parser = AdvancedSignatureParser::new(&tokens);
        
        let result = parser.parse_advanced_function_signature();
        assert!(result.is_ok());
        
        let signature = result.unwrap();
        assert_eq!(signature.name, "printf");
        assert_eq!(signature.parameters.len(), 2);
        assert!(!signature.parameters[0].is_variadic);
        assert!(signature.parameters[1].is_variadic);
    }

    #[test]
    fn test_parse_tuple_return_type() {
        let input = "slay get_coords() -> (normie, normie) {  }";
        let tokens = create_tokens(input);
        let mut parser = AdvancedSignatureParser::new(&tokens);
        
        let result = parser.parse_advanced_function_signature();
        assert!(result.is_ok());
        
        let signature = result.unwrap();
        assert!(signature.return_type.is_some());
        if let Some(Type::Tuple(types)) = signature.return_type {
            assert_eq!(types.len(), 2);
        } else {
            panic!("Expected tuple return type");
        }
    }

    #[test]
    fn test_parse_function_pointer_type() {
        let input = "slay callback(func fn(normie) -> lit) {  }";
        let tokens = create_tokens(input);
        let mut parser = AdvancedSignatureParser::new(&tokens);
        
        let result = parser.parse_advanced_function_signature();
        assert!(result.is_ok());
        
        let signature = result.unwrap();
        assert_eq!(signature.parameters.len(), 1);
        if let Some(Type::Function(params, ret)) = &signature.parameters[0].param_type {
            assert_eq!(params.len(), 1);
            assert!(matches!(**ret, Type::Lit));
        } else {
            panic!("Expected function pointer parameter type");
        }
    }

    #[test]
    fn test_parse_generic_with_bounds() {
        let input = "slay sort<T: Clone + Debug>(items [T]) where T: Ord {  }";
        let tokens = create_tokens(input);
        let mut parser = AdvancedSignatureParser::new(&tokens);
        
        let result = parser.parse_advanced_function_signature();
        assert!(result.is_ok());
        
        let signature = result.unwrap();
        assert_eq!(signature.type_parameters.len(), 1);
        assert_eq!(signature.type_parameters[0].bounds.len(), 2);
        assert!(!signature.where_clauses.is_empty());
    }

    #[test]
    fn test_parse_complex_array_type() {
        let input = "slay process_matrix(matrix [[normie; 10]; 20]) {  }";
        let tokens = create_tokens(input);
        let mut parser = AdvancedSignatureParser::new(&tokens);
        
        let result = parser.parse_advanced_function_signature();
        assert!(result.is_ok());
        
        let signature = result.unwrap();
        assert_eq!(signature.parameters.len(), 1);
        // Test nested array type parsing
        assert!(signature.parameters[0].param_type.is_some());
    }
}
