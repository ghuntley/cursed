//! Parser support for struct field type inference in the CURSED language.
//!
//! This module extends the parser with the ability to handle struct declarations
//! where field types can be omitted and inferred from their initializer expressions.
//!
//! Key features:
//! - Parsing struct declarations with optional field types
//! - Tracking fields that need type inference
//! - Integration with the type system for type propagation

use crate::ast::Identifier;
use crate::ast::statements::fields::FieldStatement;
use crate::error::Error;
use crate::lexer::Token;
use crate::lexer::TokenType;
use super::parser::Parser;
use super::context::ParsingContext;

/// Extension trait for struct field type inference in the parser
pub trait StructFieldTypeInference {
    /// Parse a struct field with optional type annotation
    /// 
    /// If the type is omitted, it will be represented as an empty string in the AST,
    /// which will signal to the type checker that the type should be inferred from
    /// the initializer.
    fn parse_struct_field_with_inference(&mut self) -> Result<FieldStatement, Error>;

    /// Check if the current token represents a valid field type
    fn is_valid_field_type(&self) -> bool;
}

impl<'a> StructFieldTypeInference for Parser<'a> {
    fn parse_struct_field_with_inference(&mut self) -> Result<FieldStatement, Error> {
        // Parse field name
        if !matches!(self.current_token, Token::Identifier(_)) {
            return Err(self.error(&format!(
                "Expected field name, got {:?}",
                self.current_token
            )));
        }

        // Get field name
        let field_name = match &self.current_token {
            Token::Identifier(ident) => Identifier {
                token: self.current_token.token_literal(),
                value: ident.clone(),
            },
            _ => unreachable!(),
        };

        self.next_token()?; // Advance past field name

        // Check if there's a field type
        let field_type = if self.is_valid_field_type() {
            // Get field type
            let typ = match &self.current_token {
                Token::Identifier(ident) => Identifier {
                    token: self.current_token.token_literal(),
                    value: ident.clone(),
                },
                _ => unreachable!(),
            };

            self.next_token()?; // Advance past field type
            typ
        } else {
            // Create a placeholder type representing inference
            Identifier {
                token: "".to_string(),
                value: "".to_string(),
            }
        };

        // Create field struct
        Ok(FieldStatement {
            token: "field".to_string(),
            name: field_name,
            type_name: field_type,
        })
    }

    fn is_valid_field_type(&self) -> bool {
        match &self.current_token {
            // Basic type tokens
            Token::Lit | Token::Smol | Token::Mid | Token::Normie | 
            Token::Thicc | Token::Snack | Token::Meal | Token::Tea | 
            Token::Sip | Token::Byte(_) | Token::Rune(_) => true,
            
            // User-defined types
            Token::Identifier(_) => true,
            
            // Composite type starts
            Token::Squad | Token::Collab | Token::LBracket | Token::At | Token::Dm => true,
            
            // Not a type
            _ => false,
        }
    }
}