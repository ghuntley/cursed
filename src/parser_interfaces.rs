//! Enhanced interface parsing for CURSED language
//! Supports generic interfaces, constraints, and method definitions

use crate::ast::{InterfaceStatement, MethodSignature, Parameter, Type, Visibility, InterfaceComposition};
use crate::lexer::{Token, TokenKind};
use crate::error_types::{Error, Result};
use crate::parser::Parser;

impl Parser {
    /// Parse an interface statement (collab keyword)
    pub fn parse_interface_statement(&mut self) -> Result<InterfaceStatement> {
        // Consume 'collab' keyword
        self.next_token()?;
        
        // Parse interface name
        let name = match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::Identifier => {
                let name = token.lexeme.clone();
                self.next_token()?;
                name
            }
            _ => return Err(Error::ParseError("Expected interface name".to_string())),
        };
        
        // Parse generic type parameters <T, U>
        let mut type_parameters = Vec::new();
        if self.current_token.as_ref().map(|t| &t.kind) == Some(&TokenKind::Less) {
            self.next_token()?; // consume '<'
            
            // Parse first type parameter
            if self.current_token.as_ref().map(|t| &t.kind) != Some(&TokenKind::Greater) {
                if let Some(token) = self.current_token.as_ref() {
                    if token.kind == TokenKind::Identifier {
                        type_parameters.push(token.lexeme.clone());
                        self.next_token()?;
                    }
                }
                
                // Parse additional type parameters
                while self.current_token.as_ref().map(|t| &t.kind) == Some(&TokenKind::Comma) {
                    self.next_token()?; // consume ','
                    if let Some(token) = self.current_token.as_ref() {
                        if token.kind == TokenKind::Identifier {
                            type_parameters.push(token.lexeme.clone());
                            self.next_token()?;
                        }
                    }
                }
            }
            
            if self.current_token.as_ref().map(|t| &t.kind) == Some(&TokenKind::Greater) {
                self.next_token()?; // consume '>'
            }
        }
        
        // Parse interface inheritance and composition
        let mut extends = Vec::new();
        let mut compositions = Vec::new();
        
        // Parse extends clause
        if self.current_token.as_ref().map(|t| &t.kind) == Some(&TokenKind::Extends) ||
           self.current_token.as_ref().map(|t| &t.kind) == Some(&TokenKind::Colon) {
            self.next_token()?; // consume 'extends' or ':'
            
            // Parse first parent interface
            if let Some(token) = self.current_token.as_ref() {
                if token.kind == TokenKind::Identifier {
                    extends.push(token.lexeme.clone());
                    self.next_token()?;
                }
            }
            
            // Parse additional parent interfaces (comma-separated)
            while self.current_token.as_ref().map(|t| &t.kind) == Some(&TokenKind::Comma) {
                self.next_token()?; // consume ','
                if let Some(token) = self.current_token.as_ref() {
                    if token.kind == TokenKind::Identifier {
                        extends.push(token.lexeme.clone());
                        self.next_token()?;
                    }
                }
            }
        }
        
        // Parse composition clause (with keyword)
        if self.current_token.as_ref().map(|t| &t.kind) == Some(&TokenKind::With) {
            self.next_token()?; // consume 'with'
            
            // Parse composition list
            loop {
                let composition = self.parse_interface_composition()?;
                compositions.push(composition);
                
                if self.current_token.as_ref().map(|t| &t.kind) == Some(&TokenKind::Comma) {
                    self.next_token()?; // consume ','
                } else {
                    break;
                }
            }
        }
        
        // Expect opening brace
        if self.current_token.as_ref().map(|t| &t.kind) != Some(&TokenKind::LeftBrace) {
            return Err(Error::ParseError("Expected '{' after interface name".to_string()));
        }
        self.next_token()?; // consume '{'
        
        // Parse method signatures
        let mut methods = Vec::new();
        while self.current_token.as_ref().map(|t| &t.kind) != Some(&TokenKind::RightBrace) {
            // Skip newlines
            if self.current_token.as_ref().map(|t| &t.kind) == Some(&TokenKind::Newline) {
                self.next_token()?;
                continue;
            }
            
            // Parse method signature
            methods.push(self.parse_method_signature()?);
        }
        
        // Expect closing brace
        if self.current_token.as_ref().map(|t| &t.kind) == Some(&TokenKind::RightBrace) {
            self.next_token()?; // consume '}'
        }
        
        Ok(InterfaceStatement {
            name,
            type_parameters,
            extends,
            compositions,
            methods,
            visibility: Visibility::Public,
        })
    }
    
    /// Parse a method signature within an interface
    fn parse_method_signature(&mut self) -> Result<MethodSignature> {
        // Expect 'slay' keyword
        if self.current_token.as_ref().map(|t| &t.kind) != Some(&TokenKind::Slay) {
            return Err(Error::ParseError("Expected 'slay' keyword for method".to_string()));
        }
        self.next_token()?; // consume 'slay'
        
        // Parse method name
        let name = match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::Identifier => {
                let name = token.lexeme.clone();
                self.next_token()?;
                name
            }
            _ => return Err(Error::ParseError("Expected method name".to_string())),
        };
        
        // Parse parameters
        let parameters = self.parse_method_parameters()?;
        
        // Parse return type (optional)
        let return_type = if let Some(token) = self.current_token.as_ref() {
            match token.kind {
                TokenKind::Normie => { self.next_token()?; Some(Type::Normie) },
                TokenKind::Tea => { self.next_token()?; Some(Type::Tea) },
                TokenKind::Lit => { self.next_token()?; Some(Type::Lit) },
                TokenKind::Sip => { self.next_token()?; Some(Type::Sip) },
                TokenKind::Smol => { self.next_token()?; Some(Type::Smol) },
                TokenKind::Mid => { self.next_token()?; Some(Type::Mid) },
                TokenKind::Thicc => { self.next_token()?; Some(Type::Thicc) },
                TokenKind::Snack => { self.next_token()?; Some(Type::Snack) },
                TokenKind::Meal => { self.next_token()?; Some(Type::Meal) },
                TokenKind::Byte => { self.next_token()?; Some(Type::Byte) },
                TokenKind::Rune => { self.next_token()?; Some(Type::Rune) },
                TokenKind::Extra => { self.next_token()?; Some(Type::Extra) },
                TokenKind::Identifier(name) => {
                    let type_name = name.clone();
                    self.next_token()?;
                    Some(Type::Custom(type_name))
                },
                _ => None
            }
        } else {
            None
        };
        
        // Parse default implementation (optional)
        let mut has_default = false;
        if self.current_token.as_ref().map(|t| &t.kind) == Some(&TokenKind::LeftBrace) {
            // Skip default implementation body for now
            has_default = true;
            self.skip_block()?;
        }
        
        Ok(MethodSignature {
            name,
            parameters,
            return_type,
        })
    }
    
    /// Parse method parameters
    fn parse_method_parameters(&mut self) -> Result<Vec<Parameter>> {
        // Expect '('
        if self.current_token.as_ref().map(|t| &t.kind) != Some(&TokenKind::LeftParen) {
            return Err(Error::ParseError("Expected '(' for method parameters".to_string()));
        }
        self.next_token()?; // consume '('
        
        let mut parameters = Vec::new();
        
        // Parse parameters
        while self.current_token.as_ref().map(|t| &t.kind) != Some(&TokenKind::RightParen) {
            // Skip newlines
            if self.current_token.as_ref().map(|t| &t.kind) == Some(&TokenKind::Newline) {
                self.next_token()?;
                continue;
            }
            
            // Parse parameter name
            let name = match self.current_token.as_ref() {
                Some(token) if token.kind == TokenKind::Identifier => {
                    let name = token.lexeme.clone();
                    self.next_token()?;
                    name
                }
                _ => return Err(Error::ParseError("Expected parameter name".to_string())),
            };
            
            // Parse parameter type
            let param_type = self.parse_type()?;
            
            parameters.push(Parameter {
                name,
                param_type,
            });
            
            // Check for comma
            if self.current_token.as_ref().map(|t| &t.kind) == Some(&TokenKind::Comma) {
                self.next_token()?; // consume ','
            } else {
                break;
            }
        }
        
        // Expect ')'
        if self.current_token.as_ref().map(|t| &t.kind) == Some(&TokenKind::RightParen) {
            self.next_token()?; // consume ')'
        }
        
        Ok(parameters)
    }
    
    /// Parse interface composition
    fn parse_interface_composition(&mut self) -> Result<InterfaceComposition> {
        // Parse composed interface name
        let composed_interface = match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::Identifier => {
                let name = token.lexeme.clone();
                self.next_token()?;
                name
            }
            _ => return Err(Error::ParseError("Expected interface name in composition".to_string())),
        };
        
        let mut alias = None;
        let mut excluded_methods = Vec::new();
        let mut method_renames = std::collections::HashMap::new();
        
        // Parse composition modifiers
        while self.current_token.as_ref().map(|t| &t.kind) != Some(&TokenKind::Comma) &&
              self.current_token.as_ref().map(|t| &t.kind) != Some(&TokenKind::LeftBrace) {
            
            match self.current_token.as_ref().map(|t| &t.kind) {
                Some(TokenKind::As) => {
                    // Parse alias: "with SomeInterface as Alias"
                    self.next_token()?; // consume 'as'
                    if let Some(token) = self.current_token.as_ref() {
                        if token.kind == TokenKind::Identifier {
                            alias = Some(token.lexeme.clone());
                            self.next_token()?;
                        }
                    }
                },
                Some(TokenKind::Except) => {
                    // Parse exclusions: "with SomeInterface except method1, method2"
                    self.next_token()?; // consume 'except'
                    loop {
                        if let Some(token) = self.current_token.as_ref() {
                            if token.kind == TokenKind::Identifier {
                                excluded_methods.push(token.lexeme.clone());
                                self.next_token()?;
                            }
                        }
                        
                        if self.current_token.as_ref().map(|t| &t.kind) == Some(&TokenKind::Comma) {
                            self.next_token()?; // consume ','
                        } else {
                            break;
                        }
                    }
                },
                Some(TokenKind::Rename) => {
                    // Parse renames: "with SomeInterface rename oldMethod -> newMethod"
                    self.next_token()?; // consume 'rename'
                    loop {
                        let old_name = match self.current_token.as_ref() {
                            Some(token) if token.kind == TokenKind::Identifier => {
                                let name = token.lexeme.clone();
                                self.next_token()?;
                                name
                            }
                            _ => break,
                        };
                        
                        if self.current_token.as_ref().map(|t| &t.kind) == Some(&TokenKind::Arrow) {
                            self.next_token()?; // consume '->'
                            if let Some(token) = self.current_token.as_ref() {
                                if token.kind == TokenKind::Identifier {
                                    let new_name = token.lexeme.clone();
                                    self.next_token()?;
                                    method_renames.insert(old_name, new_name);
                                }
                            }
                        }
                        
                        if self.current_token.as_ref().map(|t| &t.kind) == Some(&TokenKind::Comma) {
                            self.next_token()?; // consume ','
                        } else {
                            break;
                        }
                    }
                },
                _ => break,
            }
        }
        
        Ok(InterfaceComposition {
            composed_interface,
            alias,
            excluded_methods,
            method_renames,
        })
    }

    /// Skip a block (for default implementations)
    fn skip_block(&mut self) -> Result<()> {
        if self.current_token.as_ref().map(|t| &t.kind) == Some(&TokenKind::LeftBrace) {
            self.next_token()?; // consume '{'
            
            let mut brace_count = 1;
            while brace_count > 0 && self.current_token.is_some() {
                match self.current_token.as_ref().unwrap().kind {
                    TokenKind::LeftBrace => brace_count += 1,
                    TokenKind::RightBrace => brace_count -= 1,
                    _ => {}
                }
                self.next_token()?;
            }
        }
        
        Ok(())
    }
    
    /// Parse interface implementation (impl keyword)
    pub fn parse_impl_statement(&mut self) -> Result<()> {
        // Consume 'impl' keyword
        self.next_token()?;
        
        // Parse interface name or generic parameters
        let interface_name = match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::Identifier => {
                let name = token.lexeme.clone();
                self.next_token()?;
                name
            }
            _ => return Err(Error::ParseError("Expected interface name after 'impl'".to_string())),
        };
        
        // Parse 'for' keyword
        if self.current_token.as_ref().map(|t| &t.kind) != Some(&TokenKind::For) {
            return Err(Error::ParseError("Expected 'for' keyword after interface name".to_string()));
        }
        self.next_token()?; // consume 'for'
        
        // Parse type name
        let type_name = match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::Identifier => {
                let name = token.lexeme.clone();
                self.next_token()?;
                name
            }
            _ => return Err(Error::ParseError("Expected type name after 'for'".to_string())),
        };
        
        // Parse implementation body
        if self.current_token.as_ref().map(|t| &t.kind) == Some(&TokenKind::LeftBrace) {
            self.skip_block()?;
        }
        
        // TODO: Store implementation info
        
        Ok(())
    }
}
