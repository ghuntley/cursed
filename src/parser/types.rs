//! Type parsing for the CURSED programming language
//!
//! This module implements the parsing of type expressions, including
//! primitive types, composite types, user-defined types, and generic types.
//! It supports all the type constructs in the CURSED language specification.

use crate::ast::{self, Expression};
use crate::core::type_checker::Type;
use crate::error::Error;
use crate::lexer::Token;

use super::parser::Parser;

impl<'a> Parser<'a> {
    /// Parses a type expression from the token stream
    ///
    /// This method handles all the possible types in CURSED, including:
    /// - Primitive types (lit, smol, mid, normie, thicc, snack, meal, tea, sip)
    /// - Arrays and slices ([n]T and []T)
    /// - Maps (tea[K]V)
    /// - Structs (squad { fields })
    /// - Interfaces (collab { methods })
    /// - Pointers (@T)
    /// - Channels (dm<T>)
    /// - Generic types (Stack[T])
    /// - Named types (user-defined types)
    ///
    /// # Returns
    ///
    /// Result<Type, Error> - The parsed type or an error
    pub fn parse_type(&mut self) -> Result<Type, Error> {
        match &self.current_token {
            Token::Lit => {
                let _ = self.next_token()?;
                Ok(Type::Lit)
            }
            Token::Smol => {
                let _ = self.next_token()?;
                Ok(Type::Smol)
            }
            Token::Mid => {
                let _ = self.next_token()?;
                Ok(Type::Mid)
            }
            Token::Normie => {
                let _ = self.next_token()?;
                Ok(Type::Normie)
            }
            Token::Thicc => {
                let _ = self.next_token()?;
                Ok(Type::Thicc)
            }
            Token::Snack => {
                let _ = self.next_token()?;
                Ok(Type::Snack)
            }
            Token::Meal => {
                let _ = self.next_token()?;
                Ok(Type::Meal)
            }
            Token::Tea => {
                // First check if it's followed by a left bracket (peek ahead)
                let is_map = matches!(self.peek_token(), Token::LBracket);

                if is_map {
                    // It's a map type: tea[K]V
                    let _ = self.next_token()?; // consume Tea
                    self.parse_map_type()
                } else {
                    // It's the tea type (string)
                    let _ = self.next_token()?;
                    Ok(Type::Tea)
                }
            }
            Token::Sip => {
                let _ = self.next_token()?;
                Ok(Type::Sip)
            }
            Token::Byte(_) => {
                let _ = self.next_token()?;
                Ok(Type::Byte)
            }
            Token::Rune(_) => {
                let _ = self.next_token()?;
                Ok(Type::Rune)
            }
            Token::Identifier(id) if id == "extra" => {
                let _ = self.next_token()?;
                Ok(Type::new_basic("extra"))
            }
            Token::Identifier(id) => {
                // User-defined type or type parameter
                let type_name = id.clone();
                let _ = self.next_token()?;

                // Check if it's a generic type with type arguments
                if self.current_token == Token::LBracket {
                    self.parse_generic_type(type_name)
                } else {
                    Ok(Type::Named(type_name))
                }
            }
            Token::Squad => {
                // Anonymous struct type
                self.parse_anonymous_struct_type()
            }
            Token::Collab => {
                // Anonymous interface type
                self.parse_anonymous_interface_type()
            }
            Token::LBracket => {
                // Array or slice type
                self.parse_array_or_slice_type()
            }

            Token::At => {
                // Pointer type: @T
                self.parse_pointer_type()
            }
            Token::Dm => {
                // Channel type: dm<T>
                self.parse_channel_type_for_type()
            }
            _ => Err(Error::from_str(&format!(
                "Expected type, got {:?}",
                self.current_token
            ))),
        }
    }

    /// Parse a generic type like Stack[T] or Pair[A, B]
    fn parse_generic_type(&mut self, type_name: String) -> Result<Type, Error> {
        // We've already seen the type name and are at the '['
        let _ = self.next_token()?; // Skip '[''

        let mut type_args = Vec::new();

        // Parse type arguments
        if self.current_token != Token::RBracket {
            loop {
                let type_arg = self.parse_type()?;
                type_args.push(type_arg);

                if self.current_token == Token::Comma {
                    let _ = self.next_token()?; // Skip ','
                } else {
                    break;
                }
            }
        }

        // Expect closing bracket
        if self.current_token != Token::RBracket {
            return Err(Error::from_str(&format!(
                "Expected ']', got {:?}",
                self.current_token
            )));
        }

        let _ = self.next_token()?; // Skip ']'

        Ok(Type::new_generic(&type_name, type_args))
    }

    /// Parse an anonymous struct type
    fn parse_anonymous_struct_type(&mut self) -> Result<Type, Error> {
        // We've already seen the 'squad' token
        let _ = self.next_token()?; // Skip 'squad'

        // Expect '{'
        if self.current_token != Token::LBrace {
            return Err(Error::from_str(&format!(
                "Expected '{{', got {:?}",
                self.current_token
            )));
        }

        let _ = self.next_token()?; // Skip '{'

        let mut fields = Vec::new();

        // Parse fields
        while self.current_token != Token::RBrace {
            let field_name = match &self.current_token {
                Token::Identifier(id) => id.clone(),
                _ => {
                    return Err(Error::from_str(&format!(
                        "Expected field name, got {:?}",
                        self.current_token
                    )));
                }
            };

            let _ = self.next_token()?; // Skip field name

            let field_type = self.parse_type()?;
            fields.push((field_name, field_type));

            // Optional semicolon
            if self.current_token == Token::Semicolon {
                let _ = self.next_token()?; // Skip ';'
            }
        }

        let _ = self.next_token()?; // Skip '}'

        Ok(Type::new_struct("anonymous", fields))
    }

    /// Parse an anonymous interface type
    fn parse_anonymous_interface_type(&mut self) -> Result<Type, Error> {
        // We've already seen the 'collab' token
        let _ = self.next_token()?; // Skip 'collab'

        // Expect '{'
        if self.current_token != Token::LBrace {
            return Err(Error::from_str(&format!(
                "Expected '{{', got {:?}",
                self.current_token
            )));
        }

        let _ = self.next_token()?; // Skip '{'

        let mut methods = Vec::new();

        // Parse method signatures
        while self.current_token != Token::RBrace {
            let method_name = match &self.current_token {
                Token::Identifier(id) => id.clone(),
                _ => {
                    return Err(Error::from_str(&format!(
                        "Expected method name, got {:?}",
                        self.current_token
                    )));
                }
            };

            let _ = self.next_token()?; // Skip method name

            // Parse method parameters
            if self.current_token != Token::LParen {
                return Err(Error::from_str(&format!(
                    "Expected '(', got {:?}",
                    self.current_token
                )));
            }

            let _ = self.next_token()?; // Skip '('

            let mut params = Vec::new();

            // Parse parameters
            if self.current_token != Token::RParen {
                loop {
                    let param_name = match &self.current_token {
                        Token::Identifier(id) => id.clone(),
                        _ => {
                            return Err(Error::from_str(&format!(
                                "Expected parameter name, got {:?}",
                                self.current_token
                            )));
                        }
                    };

                    let _ = self.next_token()?; // Skip parameter name

                    let param_type = self.parse_type()?;
                    params.push((param_name, param_type));

                    if self.current_token == Token::Comma {
                        let _ = self.next_token()?; // Skip ','
                    } else {
                        break;
                    }
                }
            }

            // Expect ')'
            if self.current_token != Token::RParen {
                return Err(Error::from_str(&format!(
                    "Expected ')', got {:?}",
                    self.current_token
                )));
            }

            let _ = self.next_token()?; // Skip ')'

            // Parse return type if any
            let return_type =
                if self.current_token != Token::Semicolon && self.current_token != Token::RBrace {
                    Some(self.parse_type()?)
                } else {
                    None
                };

            methods.push((method_name, params, return_type));

            // Optional semicolon
            if self.current_token == Token::Semicolon {
                let _ = self.next_token()?; // Skip ';'
            }
        }

        let _ = self.next_token()?; // Skip '}'

        Ok(Type::new_interface("anonymous", methods))
    }

    /// Parse an array or slice type
    fn parse_array_or_slice_type(&mut self) -> Result<Type, Error> {
        // We've already seen the '['
        let _ = self.next_token()?; // Skip '['

        // Check if it's a slice (empty brackets)
        if self.current_token == Token::RBracket {
            let _ = self.next_token()?; // Skip ']'

            // Parse the element type
            let element_type = self.parse_type()?;

            return Ok(Type::new_slice(element_type));
        }

        // It's an array, parse the size
        let size = match &self.current_token {
            Token::Int(n) => *n as usize,
            _ => {
                return Err(Error::from_str(&format!(
                    "Expected array size, got {:?}",
                    self.current_token
                )));
            }
        };

        let _ = self.next_token()?; // Skip size

        // Expect ']'
        if self.current_token != Token::RBracket {
            return Err(Error::from_str(&format!(
                "Expected ']', got {:?}",
                self.current_token
            )));
        }

        let _ = self.next_token()?; // Skip ']'

        // Parse the element type
        let element_type = self.parse_type()?;

        Ok(Type::new_array(element_type, size))
    }

    /// Parse a map type
    fn parse_map_type(&mut self) -> Result<Type, Error> {
        // We've already seen the 'tea'
        let _ = self.next_token()?; // Skip 'tea'

        // Expect '['
        if self.current_token != Token::LBracket {
            return Err(Error::from_str(&format!(
                "Expected '[', got {:?}",
                self.current_token
            )));
        }

        let _ = self.next_token()?; // Skip '['

        // Parse key type
        let key_type = self.parse_type()?;

        // Expect ']'
        if self.current_token != Token::RBracket {
            return Err(Error::from_str(&format!(
                "Expected ']', got {:?}",
                self.current_token
            )));
        }

        let _ = self.next_token()?; // Skip ']'

        // Parse value type
        let value_type = self.parse_type()?;

        Ok(Type::new_map(key_type, value_type))
    }

    /// Parse a pointer type
    fn parse_pointer_type(&mut self) -> Result<Type, Error> {
        // We've already seen the '@'
        let _ = self.next_token()?; // Skip '@'

        // Parse the target type
        let target_type = self.parse_type()?;

        Ok(Type::new_pointer(target_type))
    }

    /// Parse a channel type for type declarations
    fn parse_channel_type_for_type(&mut self) -> Result<Type, Error> {
        // We've already seen the 'dm'
        self.next_token()?; // Skip 'dm'

        // Expect '<'
        if self.current_token != Token::Lt {
            return Err(Error::from_str(&format!(
                "Expected '<', got {:?}",
                self.current_token
            )));
        }

        self.next_token()?; // Skip '<'

        // Parse the element type
        let element_type = self.parse_type()?;

        // Expect '>'
        if self.current_token != Token::Gt {
            return Err(Error::from_str(&format!(
                "Expected '>', got {:?}",
                self.current_token
            )));
        }

        self.next_token()?; // Skip '>'

        Ok(Type::new_channel(element_type))
    }
}
