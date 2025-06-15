/// Statement parsing for the CURSED programming language

use super::Parser;
use crate::ast::*;
// Define TypeAliasStatement locally for now
use crate::ast::traits::{Node, Statement};
use std::any::Any;

// Import type switch functionality
use super::type_switch::{TypeSwitchStatement, TypeSwitchBinding, TypeLiteral};

#[derive(Debug, Clone)]
pub struct TypeAliasStatement {
    pub token: String,
    pub name: String,
    pub target_type: String,
}

impl Node for TypeAliasStatement {
    fn string(&self) -> String {
        format!("be_like {} {}", self.name, self.target_type)
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for TypeAliasStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}
use crate::ast::conditionals::{IfStatement, SwitchStatement, SwitchCase, ForStatement, RangeForStatement, WhileStatement};
use crate::ast::statements::{BreakStatement, ContinueStatement, ReturnStatement};
use crate::ast::declarations::{FunctionStatement, SquadStatement, CollabStatement, FieldStatement, TypeParameter, GenericConstraint, MethodDeclaration};
use crate::ast::identifiers::Identifier;
use crate::ast::fields::FieldDefinition;
use crate::ast::block::BlockStatement;
use crate::error::Error;
use crate::lexer::TokenType;

impl Parser {
    /// Parse any statement
    pub fn parse_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        match &self.current_token.token_type {
            TokenType::Sus | TokenType::Facts => self.parse_variable_declaration(),
            TokenType::Slay => {
                if self.peek_token_is(&TokenType::Async) {
                    self.parse_async_function()
                } else {
                    self.parse_function_declaration()
                }
            },
            TokenType::Squad => self.parse_struct_declaration(),
            TokenType::Collab => self.parse_interface_declaration(),
            TokenType::BeLike => self.parse_type_alias(),
            TokenType::Yolo => self.parse_return_statement(),
            TokenType::Lowkey => self.parse_if_statement(),
            TokenType::VibeCheck => {
                // Detect if this is a type switch or regular switch
                if self.is_type_switch() {
                    self.parse_type_switch_statement()
                } else {
                    self.parse_switch_statement()
                }
            },
            TokenType::Bestie => self.parse_for_statement(),
            TokenType::Periodt => self.parse_while_statement(),
            TokenType::Ghosted => self.parse_break_statement(),
            TokenType::Simp => self.parse_continue_statement(),
            TokenType::YeetError => self.parse_panic_statement(),
            TokenType::Catch => self.parse_recovery_statement(),
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
                
                // Parameter type is optional for now (simplified parsing)
                let param_type = if self.current_token_is(&TokenType::Identifier) {
                    let type_name = self.current_token.literal.clone();
                    self.advance_token()?;
                    type_name
                } else {
                    "".to_string() // Inferred type
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
            let ret_type_name = self.current_token.literal.clone();
            self.advance_token()?;
            // Convert to Expression - for now using a simple identifier expression
            Some(Box::new(Identifier::new(ret_type_name.clone(), ret_type_name)) as Box<dyn Expression>)
        } else {
            None
        };
        
        // Parse function body
        let body = self.parse_block_statement()?;
        
        // Convert body to BlockStatement
        let body_block = if let Some(block) = body.as_any().downcast_ref::<BlockStatement>() {
            block.clone()
        } else {
            return Err(Error::Parse("Expected block statement".to_string()));
        };
        
        Ok(Box::new(FunctionStatement {
            token: token.literal,
            name: Identifier::new(name.clone(), name),
            parameters,
            return_type,
            body: body_block,
            type_parameters: generic_params,
            generic_constraints: Vec::new(),
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
            
            let field_name = self.expect_token(TokenType::Identifier)?.literal.clone();
            let field_type = self.expect_token(TokenType::Identifier)?.literal.clone();
            
            fields.push(FieldStatement::new(
                field_name.clone(),
                Identifier::new(field_name.clone(), field_name),
                Identifier::new(field_type.clone(), field_type)
            ));
            
            self.skip_newlines();
        }
        
        self.expect_token(TokenType::RightBrace)?;
        
        Ok(Box::new(SquadStatement {
            token: token.literal,
            name: Identifier::new(name.clone(), name),
            fields,
            type_parameters: generic_params,
            generic_constraints: Vec::new(),
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
                let ret_type_name = self.current_token.literal.clone();
                self.advance_token()?;
                Some(Box::new(Identifier::new(ret_type_name.clone(), ret_type_name)) as Box<dyn Expression>)
            } else {
                None
            };
            
            methods.push(MethodDeclaration::new(
                Identifier::new(method_name.clone(), method_name),
                params,
                return_type,
            ));
            
            self.skip_newlines();
        }
        
        self.expect_token(TokenType::RightBrace)?;
        
        Ok(Box::new(CollabStatement {
            token: token.literal,
            name: Identifier::new(name.clone(), name),
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
            return_value: value,
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
        
        // Convert consequence to BlockStatement
        let consequence_block = if let Some(block) = consequence.as_any().downcast_ref::<BlockStatement>() {
            block.clone()
        } else {
            return Err(Error::Parse("Expected block statement".to_string()));
        };
        
        Ok(Box::new(IfStatement {
            token: token.literal,
            condition,
            consequence: consequence_block,
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
                
                // Convert statements to BlockStatement
                let block = BlockStatement::new(
                    "{".to_string(),
                    statements
                );
                cases.push(SwitchCase::new(case_values, block));
                
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
        
        // Convert default case to BlockStatement if present
        let default_block = if let Some(statements) = default_case {
            Some(BlockStatement::new("{".to_string(), statements))
        } else {
            None
        };
        
        Ok(Box::new(SwitchStatement {
            token: token.literal,
            value: Some(value),
            cases,
            default_case: default_block,
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
        
        // Convert body to BlockStatement
        let body_block = if let Some(block) = body.as_any().downcast_ref::<BlockStatement>() {
            block.clone()
        } else {
            return Err(Error::Parse("Expected block statement".to_string()));
        };
        
        Ok(Box::new(ForStatement::new(
            token.literal,
            init,
            condition,
            post,
            body_block,
        )))
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
        
        // Convert body to BlockStatement
        let body_block = if let Some(block) = body.as_any().downcast_ref::<BlockStatement>() {
            block.clone()
        } else {
            return Err(Error::Parse("Expected block statement".to_string()));
        };
        
        // Map to the correct structure
        let variable = value_var.unwrap_or_else(|| "item".to_string());
        
        Ok(Box::new(RangeForStatement {
            token: token.literal,
            variable,
            index_variable: key_var,
            iterable,
            body: body_block,
        }))
    }
    
    /// Parse while statement (periodt)
    fn parse_while_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.expect_token(TokenType::Periodt)?;
        let condition = self.parse_expression()?;
        let body = self.parse_block_statement()?;
        
        // Convert body to BlockStatement
        let body_block = if let Some(block) = body.as_any().downcast_ref::<BlockStatement>() {
            block.clone()
        } else {
            return Err(Error::Parse("Expected block statement".to_string()));
        };
        
        Ok(Box::new(WhileStatement {
            token: token.literal,
            condition,
            body: body_block,
        }))
    }
    
    /// Parse break statement (ghosted)
    fn parse_break_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.expect_token(TokenType::Ghosted)?;
        Ok(Box::new(BreakStatement { 
            token: token.literal,
            label: None 
        }))
    }
    
    /// Parse continue statement (simp)
    fn parse_continue_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.expect_token(TokenType::Simp)?;
        Ok(Box::new(ContinueStatement { 
            token: token.literal, 
            label: None 
        }))
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
            // Consume optional semicolon after statement
            if self.current_token_is(&TokenType::Semicolon) {
                self.advance_token()?;
            }
            self.skip_newlines();
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
                
                // Optional constraints (simplified for now)
                let constraints = if self.current_token_is(&TokenType::Identifier) {
                vec![self.current_token.literal.clone()]
                } else {
                Vec::new()
                };
                
                params.push(TypeParameter {
                token: name.clone(),
                name,
                constraints,
            });
                
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
    
    /// Parse panic statement (yeet_error message)
    fn parse_panic_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        use crate::ast::statements::PanicStatement;
        
        let token = self.current_token.clone();
        self.advance_token()?;
        
        let message = self.parse_expression()?;
        
        Ok(Box::new(PanicStatement::new(token.literal, message)))
    }
    
    /// Parse recovery statement (catch { ... })
    fn parse_recovery_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        use crate::ast::statements::RecoveryStatement;
        
        let token = self.current_token.clone();
        self.advance_token()?;
        
        // Parse protected block
        let protected_block = self.parse_block_statement()?;
        
        let mut recovery = RecoveryStatement::new(token.literal, protected_block);
        
        // Optional recovery block
        if self.current_token.literal == "recover" {
            self.advance_token()?;
            let recovery_block = self.parse_block_statement()?;
            recovery = recovery.with_recovery(recovery_block);
        }
        
        Ok(Box::new(recovery))
    }
}
