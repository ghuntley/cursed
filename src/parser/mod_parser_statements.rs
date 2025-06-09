/// Statement parsing for the CURSED programming language

use super::Parser;
use crate::ast::*;
use crate::error::Error;
use crate::lexer::TokenType;

impl Parser {
    /// Parse any statement
    pub fn parse_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        match &self.current_token.token_type {
            TokenType::Sus | TokenType::Facts => self.parse_variable_declaration(),
            TokenType::Slay => self.parse_function_declaration(),
            TokenType::Squad => self.parse_struct_declaration(),
            TokenType::Collab => self.parse_interface_declaration(),
            TokenType::BeLike => self.parse_type_alias(),
            TokenType::Yolo => self.parse_return_statement(),
            TokenType::Lowkey => self.parse_if_statement(),
            TokenType::VibeCheck => self.parse_switch_statement(),
            TokenType::Bestie => self.parse_for_statement(),
            TokenType::Periodt => self.parse_while_statement(),
            TokenType::Ghosted => self.parse_break_statement(),
            TokenType::Simp => self.parse_continue_statement(),
            TokenType::LeftBrace => self.parse_block_statement(),
            _ => self.parse_expression_statement(),
        }
    }
    
    /// Parse variable declaration (sus/facts)
    fn parse_variable_declaration(&mut self) -> Result<Box<dyn Statement>, Error> {
        let is_mutable = self.current_token_is(&TokenType::Sus);
        let token = self.current_token.clone();
        self.advance_token()?;
        
        let name = self.expect_token(TokenType::Identifier)?.literal;
        
        // Optional type annotation
        let var_type = if self.current_token_is(&TokenType::Identifier) {
            // Handle type names like 'tea', 'normie', etc.
            let type_name = self.current_token.literal.clone();
            self.advance_token()?;
            Some(type_name)
        } else {
            None
        };
        
        // Optional assignment
        let value = if self.current_token_is(&TokenType::Assign) {
            self.advance_token()?;
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        Ok(Box::new(VariableStatement {
            token: token.literal,
            name,
            var_type,
            value,
            is_mutable,
        }))
    }
    
    /// Parse function declaration (slay)
    fn parse_function_declaration(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.expect_token(TokenType::Slay)?;
        let name = self.expect_token(TokenType::Identifier)?.literal;
        
        // Parse generic parameters if present
        let generic_params = if self.current_token_is(&TokenType::LeftBracket) {
            self.parse_generic_parameters()?
        } else {
            Vec::new()
        };
        
        // Parse function parameters
        self.expect_token(TokenType::LeftParen)?;
        let mut parameters = Vec::new();
        
        if !self.current_token_is(&TokenType::RightParen) {
            loop {
                let param_name = self.expect_token(TokenType::Identifier)?.literal;
                
                // Parameter type is required
                let param_type = if self.current_token_is(&TokenType::Identifier) {
                    let type_name = self.current_token.literal.clone();
                    self.advance_token()?;
                    type_name
                } else {
                    return Err(Error::Parse("Expected parameter type".to_string()));
                };
                
                parameters.push(Parameter::new(param_name, param_type));
                
                if self.current_token_is(&TokenType::Comma) {
                    self.advance_token()?;
                } else {
                    break;
                }
            }
        }
        
        self.expect_token(TokenType::RightParen)?;
        
        // Parse return type if present
        let return_type = if self.current_token_is(&TokenType::Identifier) {
            let ret_type = self.current_token.literal.clone();
            self.advance_token()?;
            Some(ret_type)
        } else {
            None
        };
        
        // Parse function body
        let body = self.parse_block_statement()?;
        
        Ok(Box::new(FunctionStatement {
            token: token.literal,
            name: Identifier::new(name.clone(), name),
            parameters,
            return_type,
            body: *body.as_any().downcast_ref::<BlockStatement>()
                .ok_or_else(|| Error::Parse("Expected block statement".to_string()))?
                .clone(),
            type_parameters: generic_params,
        }))
    }
    
    /// Parse struct declaration (squad)
    fn parse_struct_declaration(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.expect_token(TokenType::Squad)?;
        let name = self.expect_token(TokenType::Identifier)?.literal;
        
        // Parse generic parameters if present
        let generic_params = if self.current_token_is(&TokenType::LeftBracket) {
            self.parse_generic_parameters()?
        } else {
            Vec::new()
        };
        
        self.expect_token(TokenType::LeftBrace)?;
        let mut fields = Vec::new();
        
        while !self.current_token_is(&TokenType::RightBrace) && !self.current_token_is(&TokenType::Eof) {
            self.skip_newlines();
            if self.current_token_is(&TokenType::RightBrace) {
                break;
            }
            
            let field_name = self.expect_token(TokenType::Identifier)?.literal;
            let field_type = self.expect_token(TokenType::Identifier)?.literal;
            
            fields.push(FieldDefinition::new(field_name, field_type));
            
            self.skip_newlines();
        }
        
        self.expect_token(TokenType::RightBrace)?;
        
        Ok(Box::new(SquadStatement {
            token: token.literal,
            name,
            fields,
            type_parameters: generic_params,
        }))
    }
    
    /// Parse interface declaration (collab)
    fn parse_interface_declaration(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.expect_token(TokenType::Collab)?;
        let name = self.expect_token(TokenType::Identifier)?.literal;
        
        // Parse generic parameters if present
        let generic_params = if self.current_token_is(&TokenType::LeftBracket) {
            self.parse_generic_parameters()?
        } else {
            Vec::new()
        };
        
        self.expect_token(TokenType::LeftBrace)?;
        let mut methods = Vec::new();
        
        while !self.current_token_is(&TokenType::RightBrace) && !self.current_token_is(&TokenType::Eof) {
            self.skip_newlines();
            if self.current_token_is(&TokenType::RightBrace) {
                break;
            }
            
            // Parse method signature
            let method_name = self.expect_token(TokenType::Identifier)?.literal;
            self.expect_token(TokenType::LeftParen)?;
            
            let mut params = Vec::new();
            if !self.current_token_is(&TokenType::RightParen) {
                loop {
                    let param_name = self.expect_token(TokenType::Identifier)?.literal;
                    let param_type = self.expect_token(TokenType::Identifier)?.literal;
                    params.push(Parameter::new(param_name, param_type));
                    
                    if self.current_token_is(&TokenType::Comma) {
                        self.advance_token()?;
                    } else {
                        break;
                    }
                }
            }
            
            self.expect_token(TokenType::RightParen)?;
            
            let return_type = if self.current_token_is(&TokenType::Identifier) {
                let ret_type = self.current_token.literal.clone();
                self.advance_token()?;
                Some(ret_type)
            } else {
                None
            };
            
            methods.push(FunctionDeclaration {
                name: method_name,
                parameters: params,
                return_type,
                type_parameters: Vec::new(),
            });
            
            self.skip_newlines();
        }
        
        self.expect_token(TokenType::RightBrace)?;
        
        Ok(Box::new(CollabStatement {
            token: token.literal,
            name,
            methods,
            type_parameters: generic_params,
        }))
    }
    
    /// Parse type alias (be_like)
    fn parse_type_alias(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.expect_token(TokenType::BeLike)?;
        let name = self.expect_token(TokenType::Identifier)?.literal;
        let target_type = self.expect_token(TokenType::Identifier)?.literal;
        
        Ok(Box::new(TypeAliasStatement {
            token: token.literal,
            name,
            target_type,
        }))
    }
    
    /// Parse return statement (yolo)
    fn parse_return_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.expect_token(TokenType::Yolo)?;
        
        let value = if !self.current_token_is(&TokenType::Newline) && 
                       !self.current_token_is(&TokenType::Eof) &&
                       !self.current_token_is(&TokenType::RightBrace) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        Ok(Box::new(ReturnStatement {
            token: token.literal,
            return_expression: value,
        }))
    }
    
    /// Parse if statement (lowkey/highkey)
    fn parse_if_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.expect_token(TokenType::Lowkey)?;
        
        // Optional parentheses around condition
        let has_parens = self.current_token_is(&TokenType::LeftParen);
        if has_parens {
            self.advance_token()?;
        }
        
        let condition = self.parse_expression()?;
        
        if has_parens {
            self.expect_token(TokenType::RightParen)?;
        }
        
        let consequence = self.parse_block_statement()?;
        
        let alternative = if self.current_token_is(&TokenType::Highkey) {
            self.advance_token()?;
            
            // highkey lowkey (else if) or highkey { (else)
            if self.current_token_is(&TokenType::Lowkey) {
                Some(self.parse_if_statement()?)
            } else {
                Some(self.parse_block_statement()?)
            }
        } else {
            None
        };
        
        Ok(Box::new(ParserIfStatement {
            token: token.literal,
            condition,
            consequence: *consequence.as_any().downcast_ref::<BlockStatement>()
                .ok_or_else(|| Error::Parse("Expected block statement".to_string()))?
                .clone(),
            alternative,
        }))
    }
    
    /// Parse switch statement (vibe_check)
    fn parse_switch_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.expect_token(TokenType::VibeCheck)?;
        
        let value = self.parse_expression()?;
        self.expect_token(TokenType::LeftBrace)?;
        
        let mut cases = Vec::new();
        let mut default_case = None;
        
        while !self.current_token_is(&TokenType::RightBrace) && !self.current_token_is(&TokenType::Eof) {
            self.skip_newlines();
            if self.current_token_is(&TokenType::RightBrace) {
                break;
            }
            
            if self.current_token_is(&TokenType::Mood) {
                self.advance_token()?;
                let mut case_values = vec![self.parse_expression()?];
                
                // Handle multiple case values
                while self.current_token_is(&TokenType::Comma) {
                    self.advance_token()?;
                    case_values.push(self.parse_expression()?);
                }
                
                self.expect_token(TokenType::Colon)?;
                let mut statements = Vec::new();
                
                while !self.current_token_is(&TokenType::Mood) && 
                      !self.current_token_is(&TokenType::Basic) &&
                      !self.current_token_is(&TokenType::RightBrace) &&
                      !self.current_token_is(&TokenType::Eof) {
                    statements.push(self.parse_statement()?);
                    self.skip_newlines();
                }
                
                cases.push(ParserSwitchCase { values: case_values, body: statements });
                
            } else if self.current_token_is(&TokenType::Basic) {
                self.advance_token()?;
                self.expect_token(TokenType::Colon)?;
                
                let mut statements = Vec::new();
                while !self.current_token_is(&TokenType::RightBrace) && !self.current_token_is(&TokenType::Eof) {
                    statements.push(self.parse_statement()?);
                    self.skip_newlines();
                }
                
                default_case = Some(statements);
                break;
            } else {
                return Err(Error::Parse("Expected 'mood' or 'basic' in switch statement".to_string()));
            }
        }
        
        self.expect_token(TokenType::RightBrace)?;
        
        Ok(Box::new(ParserSwitchStatement {
            token: token.literal,
            value,
            cases,
            default_case: default_case,
        }))
    }
    
    /// Parse for statement (bestie)
    fn parse_for_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.expect_token(TokenType::Bestie)?;
        
        // Check for range-based for loop (bestie x := flex items)
        if self.peek_token_is(&TokenType::ShortVarDecl) || 
           (self.peek_token_is(&TokenType::Comma) && self.current_token_is(&TokenType::Identifier)) {
            return self.parse_range_for_statement(token);
        }
        
        // Traditional for loop: bestie init; condition; post { ... }
        let init = if self.current_token_is(&TokenType::Semicolon) {
            None
        } else {
            Some(self.parse_statement()?)
        };
        
        if !self.current_token_is(&TokenType::Semicolon) {
            self.expect_token(TokenType::Semicolon)?;
        } else {
            self.advance_token()?;
        }
        
        let condition = if self.current_token_is(&TokenType::Semicolon) {
            None
        } else {
            Some(self.parse_expression()?)
        };
        
        self.expect_token(TokenType::Semicolon)?;
        
        let post = if self.current_token_is(&TokenType::LeftBrace) {
            None
        } else {
            Some(self.parse_statement()?)
        };
        
        let body = self.parse_block_statement()?;
        
        Ok(Box::new(ParserForStatement {
            token: token.literal,
            init,
            condition,
            post,
            body: *body.as_any().downcast_ref::<BlockStatement>()
                .ok_or_else(|| Error::Parse("Expected block statement".to_string()))?
                .clone(),
        }))
    }
    
    /// Parse range-based for statement (bestie x := flex items)
    fn parse_range_for_statement(&mut self, token: crate::lexer::Token) -> Result<Box<dyn Statement>, Error> {
        let key_var = if self.current_token_is(&TokenType::Identifier) {
            let var = self.current_token.literal.clone();
            self.advance_token()?;
            Some(var)
        } else {
            None
        };
        
        let value_var = if self.current_token_is(&TokenType::Comma) {
            self.advance_token()?;
            let var = self.expect_token(TokenType::Identifier)?.literal;
            Some(var)
        } else if key_var.is_some() {
            key_var.clone()
        } else {
            None
        };
        
        self.expect_token(TokenType::ShortVarDecl)?;
        self.expect_token(TokenType::Flex)?;
        
        let iterable = self.parse_expression()?;
        let body = self.parse_block_statement()?;
        
        Ok(Box::new(ParserRangeForStatement {
            token: token.literal,
            key_var,
            value_var,
            iterable,
            body: *body.as_any().downcast_ref::<BlockStatement>()
                .ok_or_else(|| Error::Parse("Expected block statement".to_string()))?
                .clone(),
        }))
    }
    
    /// Parse while statement (periodt)
    fn parse_while_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.expect_token(TokenType::Periodt)?;
        let condition = self.parse_expression()?;
        let body = self.parse_block_statement()?;
        
        Ok(Box::new(ParserWhileStatement {
            token: token.literal,
            condition,
            body: *body.as_any().downcast_ref::<BlockStatement>()
                .ok_or_else(|| Error::Parse("Expected block statement".to_string()))?
                .clone(),
        }))
    }
    
    /// Parse break statement (ghosted)
    fn parse_break_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.expect_token(TokenType::Ghosted)?;
        Ok(Box::new(ParserBreakStatement { token: token.literal, label: None }))
    }
    
    /// Parse continue statement (simp)
    fn parse_continue_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.expect_token(TokenType::Simp)?;
        Ok(Box::new(ParserContinueStatement { token: token.literal, label: None }))
    }
    
    /// Parse block statement
    pub fn parse_block_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.expect_token(TokenType::LeftBrace)?;
        let mut statements = Vec::new();
        
        while !self.current_token_is(&TokenType::RightBrace) && !self.current_token_is(&TokenType::Eof) {
            self.skip_newlines();
            if self.current_token_is(&TokenType::RightBrace) {
                break;
            }
            statements.push(self.parse_statement()?);
        }
        
        self.expect_token(TokenType::RightBrace)?;
        
        Ok(Box::new(BlockStatement {
            token: token.literal,
            statements,
        }))
    }
    
    /// Parse expression statement
    fn parse_expression_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let expression = self.parse_expression()?;
        
        Ok(Box::new(ExpressionStatement {
            token: expression.token_literal(),
            expression,
        }))
    }
    
    /// Parse generic parameters [T, U, ...]
    fn parse_generic_parameters(&mut self) -> Result<Vec<TypeParameter>, Error> {
        self.expect_token(TokenType::LeftBracket)?;
        let mut params = Vec::new();
        
        if !self.current_token_is(&TokenType::RightBracket) {
            loop {
                let name = self.expect_token(TokenType::Identifier)?.literal;
                
                // Optional constraint
                let constraint = if self.current_token_is(&TokenType::Identifier) {
                    Some(self.current_token.literal.clone())
                } else {
                    None
                };
                
                params.push(TypeParameter { name, constraint });
                
                if self.current_token_is(&TokenType::Comma) {
                    self.advance_token()?;
                } else {
                    break;
                }
            }
        }
        
        self.expect_token(TokenType::RightBracket)?;
        Ok(params)
    }
}
