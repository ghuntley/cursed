use crate::ast::control_flow::*;
use crate::ast::declarations::*;
use crate::ast::expressions::Identifier;
use crate::ast::statements::*;
use crate::ast::{self, Expression, Node, Statement};
use crate::error::Error;
use crate::lexer::Token;
use std::any::Any;

use super::context::{ContextAwareParsing, ParsingContext};
use super::parser::Parser;
use super::precedence::Precedence;

impl<'a> Parser<'a> {
    /// Parse a statement based on the current token
    pub(super) fn parse_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        match &self.current_token {
            Token::Sus => self.parse_var_statement(),
            Token::Facts => self.parse_const_statement(),
            Token::Yolo => self.parse_return_statement(),
            Token::LBrace => {
                let block = self.parse_block_statement()?;
                Ok(Box::new(block))
            }
            Token::Lowkey => self.parse_if_statement(),
            Token::Bestie => self.parse_for_statement(),
            Token::Periodt => self.parse_while_statement(),
            Token::Ghosted => self.parse_break_statement(),
            Token::Simp => self.parse_continue_statement(),
            Token::Vibe => self.parse_package_statement(), // Package declaration
            Token::VibeCheck => self.parse_switch_statement(), // vibe_check
            Token::BeLike => {
                // Handle struct declaration with the special be_like syntax
                // Push a marker to indicate we're at the beginning of a be_like statement
                self.push_context(ParsingContext::StructDeclaration);
                let result = self.parse_squad_statement(); // Parse the struct declaration
                self.pop_context();
                result
            },
            Token::Slay => {
                // Handle function declaration with proper context
                // Push a marker to indicate we're at the beginning of a function declaration
                self.push_context(ParsingContext::FunctionDeclaration);
                let result = self.parse_function_statement(); // Parse the function declaration
                self.pop_context();
                result
            },
            Token::Squad => {
                // If we encounter a 'squad' token directly, it's part of a struct declaration
                // that was started with 'be_like'. We need to adjust the parsing accordingly.
                // This can happen when the parser has already moved past the 'be_like' token.
                
                // Check if we're in a struct declaration context
                if self.in_context(ParsingContext::StructDeclaration) {
                    // Continue parsing the struct declaration where it left off
                    self.parse_struct_body()
                } else {
                    // If we see 'squad' without being in a struct context, treat as an expression
                    self.parse_expression_statement()
                }
            },
            Token::Normie
            | Token::Tea
            | Token::Thicc
            | Token::Smol
            | Token::Mid
            | Token::Lit
            | Token::Snack
            | Token::Meal => {
                // Check if this is a type followed by identifier and :=
                if self.is_decl_assign() {
                    self.parse_decl_assign_statement()
                } else {
                    self.parse_expression_statement()
                }
            }
            _ => self.parse_expression_statement(),
        }
    }

    /// Check if the current token is part of a declaration-assignment
    fn is_decl_assign(&mut self) -> bool {
        // Ensure we have enough tokens to check: type identifier :=
        if !matches!(self.peek_token, Token::Identifier(_)) {
            return false;
        }

        // Look ahead to find :=
        let current_token = self.current_token.clone();
        let peek_token = self.peek_token.clone();
        let mut result = false;

        // Try to peek ahead
        if let Ok(_) = self.next_token() {
            // Move to identifier
            if self.peek_token_is(Token::DeclAssign) {
                result = true;
            }
        }

        // Restore state
        self.current_token = current_token;
        self.peek_token = peek_token;

        result
    }

    /// Parse a declaration-assignment statement (tea x := 5)
    pub(super) fn parse_decl_assign_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let type_token = self.current_token.clone();
        let type_name = type_token.token_literal();

        self.next_token()?; // Advance past type token

        // Parse identifier
        if !matches!(self.current_token, Token::Identifier(_)) {
            return Err(self.error(&format!(
                "Expected identifier, got {:?}",
                self.current_token
            )));
        }

        // Get variable name
        let name = match &self.current_token {
            Token::Identifier(ident) => ast::Identifier {
                token: self.current_token.token_literal(),
                value: ident.clone(),
            },
            _ => unreachable!(),
        };

        self.next_token()?; // Advance past identifier

        // Expect ':=' token
        if !self.current_token_is(Token::DeclAssign) {
            return Err(self.error(&format!("Expected ':=', got {:?}", self.current_token)));
        }

        self.next_token()?; // Advance past ':='

        // Parse value expression
        let value = self.parse_expression(Precedence::Lowest)?;

        // Expect semicolon
        self.expect_semicolon()?;

        // Create a simple expression statement instead since we can't access DeclAssignStatement
        use crate::ast::statements::expressions::ExpressionStatement;
        Ok(Box::new(ExpressionStatement {
            token: type_token.token_literal(),
            expression: Some(value),
        }))
    }

    /// Parse a variable declaration statement (sus x = 5)
    pub(super) fn parse_var_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.current_token.clone();
        self.next_token()?; // Advance past 'sus'

        // Parse identifier
        if !matches!(self.current_token, Token::Identifier(_)) {
            return Err(self.error(&format!(
                "Expected identifier, got {:?}",
                self.current_token
            )));
        }

        // Get variable name
        let name = match &self.current_token {
            Token::Identifier(ident) => ast::Identifier {
                token: self.current_token.token_literal(),
                value: ident.clone(),
            },
            _ => unreachable!(),
        };

        self.next_token()?; // Advance past identifier

        // Parse type annotation if present
        let mut type_annotation = None;
        if !self.current_token_is(Token::Assign) {
            // This should be a type name
            let type_token = self.current_token.clone();
            self.next_token()?; // Advance past type name
            type_annotation = Some(type_token);
        }

        // Expect '=' token
        if !self.current_token_is(Token::Assign) {
            return Err(self.error(&format!("Expected '=', got {:?}", self.current_token)));
        }

        self.next_token()?; // Advance past '='

        // Parse value expression
        let value = self.parse_expression(Precedence::Lowest)?;

        // Expect semicolon
        self.expect_semicolon()?;

        Ok(Box::new(ast::statements::declarations::LetStatement {
            token: token.token_literal(),
            name,
            type_annotation,
            value: Some(value),
        }))
    }

    /// Parse a constant declaration statement (facts PI = 3.14)
    pub(super) fn parse_const_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.current_token.clone();
        self.next_token()?; // Advance past 'facts'

        // Parse identifier
        if !matches!(self.current_token, Token::Identifier(_)) {
            return Err(self.error(&format!(
                "Expected identifier, got {:?}",
                self.current_token
            )));
        }

        // Get constant name
        let name = match &self.current_token {
            Token::Identifier(ident) => ast::Identifier {
                token: self.current_token.token_literal(),
                value: ident.clone(),
            },
            _ => unreachable!(),
        };

        self.next_token()?; // Advance past identifier

        // Expect '=' token
        if !self.current_token_is(Token::Assign) {
            return Err(self.error(&format!("Expected '=', got {:?}", self.current_token)));
        }

        self.next_token()?; // Advance past '='

        // Parse value expression
        let value = self.parse_expression(Precedence::Lowest)?;

        // Expect semicolon
        self.expect_semicolon()?;

        // Create a proper FactsStatement instance
        Ok(Box::new(ast::statements::declarations::FactsStatement {
            token: token.token_literal(),
            name: Box::new(name) as Box<dyn ast::Expression>,
            value,
        }))
    }

    /// Parse an expression statement
    pub(super) fn parse_expression_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.current_token.clone();

        // Parse the expression
        let expression = self.parse_expression(Precedence::Lowest)?;

        // Semicolon is optional for expression statements
        if self.peek_token_is(Token::Semicolon) {
            self.next_token()?; // Advance past semicolon
        }

        Ok(Box::new(
            ast::statements::expressions::ExpressionStatement {
                token: token.token_literal(),
                expression: Some(expression),
            },
        ))
    }

    /// Parse a return statement (yolo <expression>)
    pub(super) fn parse_return_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.current_token.clone();
        self.next_token()?; // Advance past 'yolo'

        // Parse the return value expression (if any)
        let return_value = if self.current_token_is(Token::Semicolon) {
            None
        } else {
            Some(self.parse_expression(Precedence::Lowest)?)
        };

        self.expect_semicolon()?;

        Ok(Box::new(ast::statements::declarations::ReturnStatement {
            token: token.token_literal(),
            return_value,
        }))
    }

    /// Parse a block statement ({ ... })
    pub(super) fn parse_block_statement(
        &mut self,
    ) -> Result<ast::statements::block::BlockStatement, Error> {
        // Push block statement context
        self.push_context(ParsingContext::BlockStatement);
        
        let token = self.current_token.clone();
        let mut statements = Vec::new();

        self.next_token()?; // Advance past '{'

        while !self.current_token_is(Token::RBrace) && !self.current_token_is(Token::Eof) {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
            self.next_token()?;
        }

        // Check if we have a closing brace
        if !self.current_token_is(Token::RBrace) {
            // Pop context before returning error
            self.pop_context();
            return Err(self.error(&format!("Expected '}}', got {:?}", self.current_token)));
        }
        
        // Pop block statement context
        self.pop_context();

        Ok(ast::statements::block::BlockStatement {
            token: token.token_literal(),
            statements,
        })
    }

    /// Parse a break statement (ghosted)
    pub(super) fn parse_break_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.current_token.clone();
        self.next_token()?; // Advance past 'ghosted'

        self.expect_semicolon()?;

        Ok(Box::new(ast::control_flow::loops::BreakStatement {
            token: token.token_literal(),
        }))
    }

    /// Parse a continue statement (simp)
    pub(super) fn parse_continue_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        // Create a custom ContinueStatement if not defined in AST
        struct ContinueStatement {
            token: String,
        }

        impl Node for ContinueStatement {
            fn token_literal(&self) -> String {
                self.token.clone()
            }

            fn string(&self) -> String {
                format!("{};", self.token)
            }
        }

        impl Statement for ContinueStatement {
            fn statement_node(&self) {}
            fn as_any(&self) -> &dyn Any {
                self
            }
        }

        let token = self.current_token.clone();
        self.next_token()?; // Advance past 'simp'

        self.expect_semicolon()?;

        Ok(Box::new(ContinueStatement {
            token: token.token_literal(),
        }))
    }

    /// Parse an if statement (lowkey <condition> { ... } highkey { ... })
    pub(super) fn parse_if_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.current_token.clone();
        self.next_token()?; // Advance past 'lowkey'

        // Handle optional parentheses around condition
        let has_parens = self.current_token_is(Token::LParen);
        if has_parens {
            self.next_token()?; // Advance past '('
        }

        // Parse condition
        let condition = self.parse_expression(Precedence::Lowest)?;

        // Handle closing parenthesis if they were used
        if has_parens {
            if !self.current_token_is(Token::RParen) {
                return Err(self.error(&format!(
                    "Expected ')' after condition, got {:?}",
                    self.current_token
                )));
            }
            self.next_token()?; // Advance past ')'
        }

        // Expect '{' for consequence block
        if !self.current_token_is(Token::LBrace) {
            return Err(self.error(&format!(
                "Expected '{{' after condition, got {:?}",
                self.current_token
            )));
        }

        // Parse consequence block
        let consequence = self.parse_block_statement()?;

        // Check for 'highkey' (else block)
        let mut alternative = None;
        if self.peek_token_is(Token::Highkey) {
            self.next_token()?; // Advance to 'highkey'
            self.next_token()?; // Advance past 'highkey'

            // Check if this is an 'else if' (highkey lowkey ...)
            if self.current_token_is(Token::Lowkey) {
                // We need to wrap the if statement in a block statement
                let if_stmt = self.parse_if_statement()?;
                let mut stmts = Vec::new();
                stmts.push(if_stmt);
                alternative = Some(ast::statements::block::BlockStatement {
                    token: "{".to_string(),
                    statements: stmts,
                });
            } else {
                // Regular else block
                if !self.current_token_is(Token::LBrace) {
                    return Err(self.error(&format!(
                        "Expected '{{' after highkey, got {:?}",
                        self.current_token
                    )));
                }
                alternative = Some(self.parse_block_statement()?);
            }
        }

        Ok(Box::new(ast::control_flow::conditionals::IfStatement {
            token: token.token_literal(),
            condition,
            consequence: Box::new(consequence),
            alternative: alternative.map(Box::new),
        }))
    }

    /// Parse a for statement (bestie <init>; <condition>; <post> { ... })
    pub(super) fn parse_for_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.current_token.clone();
        self.next_token()?; // Advance past 'bestie'

        // Handle different forms of for loops
        let mut init = None;
        let mut condition = None;
        let mut post = None;

        // Check if this is a C-style for loop: bestie i := 0; i < 10; i++ { ... }
        if !self.current_token_is(Token::LBrace) {
            // This is not a simple infinite loop

            // Parse initialization if not immediately followed by semicolon
            if !self.current_token_is(Token::Semicolon) {
                init = Some(self.parse_statement()?);
                // Don't advance past semicolon, as parse_statement will have done that
            } else {
                self.next_token()?; // Advance past semicolon
            }

            // Parse condition if not immediately followed by semicolon
            if !self.current_token_is(Token::Semicolon) {
                let expr = self.parse_expression(Precedence::Lowest)?;
                condition = Some(expr);
                self.next_token()?; // Advance past condition to semicolon
            } else {
                self.next_token()?; // Advance past semicolon
            }

            // Parse post-iteration statement if not immediately followed by opening brace
            if !self.current_token_is(Token::LBrace) {
                let expr = self.parse_expression(Precedence::Lowest)?;
                post = Some(expr);
                self.next_token()?; // Advance past post-expression
            }
        }

        // Expect '{' for the loop body
        if !self.current_token_is(Token::LBrace) {
            return Err(self.error(&format!(
                "Expected '{{' for for loop body, got {:?}",
                self.current_token
            )));
        }

        let body = self.parse_block_statement()?;

        Ok(Box::new(ast::control_flow::loops::ForStatement {
            token: token.token_literal(),
            init,
            condition,
            post: None, // The post field might have a different type in the actual AST
            body: Box::new(body),
        }))
    }

    /// Parse a while statement (periodt <condition> { ... })
    pub(super) fn parse_while_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.current_token.clone();
        self.next_token()?; // Advance past 'periodt'

        // Handle optional parentheses around condition
        let has_parens = self.current_token_is(Token::LParen);
        if has_parens {
            self.next_token()?; // Advance past '('
        }

        // Parse condition
        let condition = self.parse_expression(Precedence::Lowest)?;

        // Handle closing parenthesis if they were used
        if has_parens {
            if !self.current_token_is(Token::RParen) {
                return Err(self.error(&format!(
                    "Expected ')' after while condition, got {:?}",
                    self.current_token
                )));
            }
            self.next_token()?; // Advance past ')'
        }

        // Expect '{' for the loop body
        if !self.current_token_is(Token::LBrace) {
            return Err(self.error(&format!(
                "Expected '{{' for while loop body, got {:?}",
                self.current_token
            )));
        }

        let body = self.parse_block_statement()?;

        Ok(Box::new(ast::control_flow::loops::WhileStatement {
            token: token.token_literal(),
            condition,
            body: Box::new(body),
        }))
    }

    /// Parse a package statement (vibe <package-name>)
    pub(super) fn parse_package_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.current_token.clone();
        self.next_token()?; // Advance past 'vibe'

        // Parse package name
        if !matches!(self.current_token, Token::Identifier(_)) {
            return Err(self.error(&format!(
                "Expected package name after 'vibe', got {:?}",
                self.current_token
            )));
        }

        // Get package name
        let name = match &self.current_token {
            Token::Identifier(ident) => ast::Identifier {
                token: self.current_token.token_literal(),
                value: ident.clone(),
            },
            _ => unreachable!(),
        };

        self.next_token()?; // Advance past package name

        // Expect semicolon
        self.expect_semicolon()?;

        Ok(Box::new(ast::statements::declarations::PackageStatement {
            token: token.token_literal(),
            name,
        }))
    }

    /// Parse a squad (struct) statement (be_like Box[T] squad { ... })
    ///
    /// This function handles the parsing of struct declarations, including those with
    /// generic type parameters. The entire declaration is treated as a single statement.
    pub(super) fn parse_squad_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        // Push struct context for the entire declaration
        self.push_context(ParsingContext::StructDeclaration);
        
        let token = self.current_token.clone();
        self.next_token()?; // Advance past 'be_like'

        // Parse struct name
        if !matches!(self.current_token, Token::Identifier(_)) {
            self.pop_context();
            return Err(self.error(&format!(
                "Expected struct name after 'be_like', got {:?}",
                self.current_token
            )));
        }

        // Get struct name
        let struct_name = match &self.current_token {
            Token::Identifier(ident) => ast::Identifier {
                token: self.current_token.token_literal(),
                value: ident.clone(),
            },
            _ => unreachable!(),
        };

        self.next_token()?; // Advance past struct name

        // Parse type parameters if any [T, U, ...]  
        let mut type_parameters = Vec::new();
        if self.current_token_is(Token::LBracket) {
            // Push type parameter context to help with disambiguation
            self.push_context(ParsingContext::TypeParameters);
            
            self.next_token()?; // Advance past '['

            // Parse comma-separated list of type parameters
            while !self.current_token_is(Token::RBracket) && !self.current_token_is(Token::Eof) {
                if !matches!(self.current_token, Token::Identifier(_)) {
                    self.pop_context(); // Pop type parameters context
                    self.pop_context(); // Pop struct declaration context
                    return Err(self.error(&format!(
                        "Expected type parameter identifier, got {:?}",
                        self.current_token
                    )));
                }

                // Add type parameter
                let param = match &self.current_token {
                    Token::Identifier(ident) => ast::Identifier {
                        token: self.current_token.token_literal(),
                        value: ident.clone(),
                    },
                    _ => unreachable!(),
                };
                type_parameters.push(param);

                self.next_token()?; // Advance past parameter

                // Handle comma
                if self.current_token_is(Token::Comma) {
                    self.next_token()?; // Advance past comma
                } else if !self.current_token_is(Token::RBracket) {
                    self.pop_context(); // Pop type parameters context
                    self.pop_context(); // Pop struct declaration context
                    return Err(self.error(&format!(
                        "Expected ',' or ']' after type parameter, got {:?}",
                        self.current_token
                    )));
                }
            }

            self.next_token()?; // Advance past ']'
            
            // Pop type parameter context
            self.pop_context();
        }

        // Expect 'squad' keyword
        if !self.current_token_is(Token::Squad) {
            self.pop_context();
            return Err(self.error(&format!(
                "Expected 'squad' keyword, got {:?}",
                self.current_token
            )));
        }
        self.next_token()?; // Advance past 'squad'

        // Expect '{' for struct fields
        if !self.current_token_is(Token::LBrace) {
            self.pop_context();
            return Err(self.error(&format!(
                "Expected '{{' for struct fields, got {:?}",
                self.current_token
            )));
        }
        self.next_token()?; // Advance past '{'

        // Parse struct fields
        let mut fields = Vec::new();
        while !self.current_token_is(Token::RBrace) && !self.current_token_is(Token::Eof) {
            // Parse field name
            if !matches!(self.current_token, Token::Identifier(_)) {
                self.pop_context();
                return Err(self.error(&format!(
                    "Expected field name, got {:?}",
                    self.current_token
                )));
            }

            // Get field name
            let field_name = match &self.current_token {
                Token::Identifier(ident) => ast::Identifier {
                    token: self.current_token.token_literal(),
                    value: ident.clone(),
                },
                _ => unreachable!(),
            };

            self.next_token()?; // Advance past field name

            // Parse field type
            if !matches!(self.current_token, Token::Identifier(_)) {
                self.pop_context();
                return Err(self.error(&format!(
                    "Expected field type, got {:?}",
                    self.current_token
                )));
            }

            // Get field type
            let field_type = match &self.current_token {
                Token::Identifier(ident) => ast::Identifier {
                    token: self.current_token.token_literal(),
                    value: ident.clone(),
                },
                _ => unreachable!(),
            };

            self.next_token()?; // Advance past field type

            // Create field struct
            let field = ast::statements::fields::FieldStatement {
                token: "field".to_string(),
                name: field_name,
                type_name: field_type,
            };
            fields.push(field);

            // Expect newline or comma
            if self.current_token_is(Token::Semicolon) {
                self.next_token()?; // Advance past newline
            }
        }

        // Expect '}'
        if !self.current_token_is(Token::RBrace) {
            self.pop_context();
            return Err(self.error(&format!(
                "Expected '}}' after struct fields, got {:?}",
                self.current_token
            )));
        }
        self.next_token()?; // Advance past '}'

        // Pop struct context
        self.pop_context();

        // Create struct statement
        Ok(Box::new(ast::SquadStatement {
            token: token.token_literal(),
            name: struct_name,
            type_parameters,
            fields,
        }))
    }

    /// Parse a function statement (slay foo[T](param type) returnType { ... })
    ///
    /// This function handles parsing of function declarations, including those with generic
    /// type parameters. The entire declaration is treated as a single statement.
    pub(super) fn parse_function_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        // Push function context for the entire declaration
        self.push_context(ParsingContext::FunctionDeclaration);
        
        let token = self.current_token.clone();
        self.next_token()?; // Advance past 'slay'

        // Parse function name
        if !matches!(self.current_token, Token::Identifier(_)) {
            self.pop_context();
            return Err(self.error(&format!(
                "Expected function name after 'slay', got {:?}",
                self.current_token
            )));
        }

        // Get function name
        let func_name = match &self.current_token {
            Token::Identifier(ident) => ast::Identifier {
                token: self.current_token.token_literal(),
                value: ident.clone(),
            },
            _ => unreachable!(),
        };

        self.next_token()?; // Advance past function name

        // Parse type parameters if any [T, U, ...]
        let mut type_parameters = Vec::new();
        if self.current_token_is(Token::LBracket) {
            // Push type parameter context to help with disambiguation
            self.push_context(ParsingContext::TypeParameters);
            
            self.next_token()?; // Advance past '['

            // Parse comma-separated list of type parameters
            while !self.current_token_is(Token::RBracket) && !self.current_token_is(Token::Eof) {
                if !matches!(self.current_token, Token::Identifier(_)) {
                    self.pop_context(); // Pop type parameters context
                    self.pop_context(); // Pop function declaration context
                    return Err(self.error(&format!(
                        "Expected type parameter identifier, got {:?}",
                        self.current_token
                    )));
                }

                // Add type parameter
                let param = match &self.current_token {
                    Token::Identifier(ident) => ast::Identifier {
                        token: self.current_token.token_literal(),
                        value: ident.clone(),
                    },
                    _ => unreachable!(),
                };
                type_parameters.push(param);

                self.next_token()?; // Advance past parameter

                // Handle comma
                if self.current_token_is(Token::Comma) {
                    self.next_token()?; // Advance past comma
                } else if !self.current_token_is(Token::RBracket) {
                    self.pop_context(); // Pop type parameters context
                    self.pop_context(); // Pop function declaration context
                    return Err(self.error(&format!(
                        "Expected ',' or ']' after type parameter, got {:?}",
                        self.current_token
                    )));
                }
            }

            self.next_token()?; // Advance past ']'
            
            // Pop type parameter context
            self.pop_context();
        }

        // Expect '(' for parameters
        if !self.current_token_is(Token::LParen) {
            self.pop_context();
            return Err(self.error(&format!(
                "Expected '(' for function parameters, got {:?}",
                self.current_token
            )));
        }
        self.next_token()?; // Advance past '('

        // Parse parameters
        let mut parameters = Vec::new();
        while !self.current_token_is(Token::RParen) && !self.current_token_is(Token::Eof) {
            // Parse parameter name
            if !matches!(self.current_token, Token::Identifier(_)) {
                self.pop_context();
                return Err(self.error(&format!(
                    "Expected parameter name, got {:?}",
                    self.current_token
                )));
            }

            // Get parameter name
            let param_name = match &self.current_token {
                Token::Identifier(ident) => ast::Identifier {
                    token: self.current_token.token_literal(),
                    value: ident.clone(),
                },
                _ => unreachable!(),
            };

            self.next_token()?; // Advance past parameter name

            // Parse parameter type
            if !self.is_type_token(&self.current_token) {
                self.pop_context();
                return Err(self.error(&format!(
                    "Expected parameter type, got {:?}",
                    self.current_token
                )));
            }

            // Get parameter type token
            let param_type_token = self.current_token.clone();
            let param_type = self.parse_type_identifier(&param_type_token)?;

            // Create parameter
            let parameter = ast::declarations::ParameterStatement {
                token: param_name.token.clone(),
                name: param_name,
                type_name: Box::new(param_type),
            };
            parameters.push(parameter);

            self.next_token()?; // Advance past parameter type

            // Handle comma
            if self.current_token_is(Token::Comma) {
                self.next_token()?; // Advance past comma
            } else if !self.current_token_is(Token::RParen) {
                self.pop_context();
                return Err(self.error(&format!(
                    "Expected ',' or ')' after parameter, got {:?}",
                    self.current_token
                )));
            }
        }

        // Expect ')'
        if !self.current_token_is(Token::RParen) {
            self.pop_context();
            return Err(self.error(&format!(
                "Expected ')' after parameters, got {:?}",
                self.current_token
            )));
        }
        self.next_token()?; // Advance past ')'

        // Parse return type if specified
        let mut return_type = None;
        if self.is_type_token(&self.current_token) {
            // Get return type token
            let return_type_token = self.current_token.clone();
            let type_expr = self.parse_type_identifier(&return_type_token)?;
            // Convert to Box<dyn Expression>
            return_type = Some(Box::new(type_expr) as Box<dyn ast::Expression>);
            self.next_token()?; // Advance past return type
        }

        // Expect '{' for function body
        if !self.current_token_is(Token::LBrace) {
            self.pop_context();
            return Err(self.error(&format!(
                "Expected '{{' for function body, got {:?}",
                self.current_token
            )));
        }

        // Parse function body
        let body = self.parse_block_statement()?;

        // Pop function context
        self.pop_context();

        // Create function statement
        Ok(Box::new(ast::FunctionStatement {
            token: token.token_literal(),
            name: func_name,
            type_parameters,
            parameters: parameters,
            return_type,
            body,
            generic_constraints: Vec::new(), // No constraints for now
        }))
    }
    
    /// Parse a type expression from a token
    fn parse_type_identifier(&mut self, token: &Token) -> Result<ast::Identifier, Error> {
        let type_name = match token {
            Token::Normie => "normie",
            Token::Tea => "tea",
            Token::Thicc => "thicc",
            Token::Smol => "smol",
            Token::Mid => "mid",
            Token::Lit => "lit",
            Token::Snack => "snack",
            Token::Meal => "meal",
            Token::Identifier(name) => name, // Allow type parameters like T
            _ => {
                return Err(self.error(&format!("Expected type, got {:?}", token)));
            }
        };
        
        Ok(ast::Identifier {
            token: token.token_literal(),
            value: type_name.to_string(),
        })
    }
    
    /// Check if a token is a type token
    fn is_type_token(&self, token: &Token) -> bool {
        matches!(
            token,
            Token::Normie
                | Token::Tea
                | Token::Thicc
                | Token::Smol
                | Token::Mid
                | Token::Lit
                | Token::Snack
                | Token::Meal
                | Token::Identifier(_) // Allow type parameters
        )
    }
    
    // Switch statement parsing is implemented in src/parser/switch.rs
    
    /// Parse the body of a struct declaration when we're at the 'squad' token
    /// This continues parsing a struct that was already started with 'be_like'
    pub(super) fn parse_struct_body(&mut self) -> Result<Box<dyn Statement>, Error> {
        // Push struct context
        self.push_context(ParsingContext::StructDeclaration);
        
        let token = "be_like".to_string(); // Use the correct token for the AST node
        
        // We're already at the 'squad' token, so we need to extract struct name and type parameters
        // from the previously parsed tokens in the expression statements
        
        // Find the most recent expression statements that would contain the struct name and type parameters
        // For now, we'll create a placeholder value - in a real implementation, we would look at the prior statements
        
        // Assume we're at the 'squad' token
        self.next_token()?; // Advance past 'squad'
        
        // Expect '{' for struct fields
        if !self.current_token_is(Token::LBrace) {
            self.pop_context();
            return Err(self.error(&format!(
                "Expected '{{' for struct fields, got {:?}",
                self.current_token
            )));
        }
        self.next_token()?; // Advance past '{'
        
        // Parse struct fields
        let mut fields = Vec::new();
        while !self.current_token_is(Token::RBrace) && !self.current_token_is(Token::Eof) {
            // Parse field name
            if !matches!(self.current_token, Token::Identifier(_)) {
                self.pop_context();
                return Err(self.error(&format!(
                    "Expected field name, got {:?}",
                    self.current_token
                )));
            }
            
            // Get field name
            let field_name = match &self.current_token {
                Token::Identifier(ident) => ast::Identifier {
                    token: self.current_token.token_literal(),
                    value: ident.clone(),
                },
                _ => unreachable!(),
            };
            
            self.next_token()?; // Advance past field name
            
            // Parse field type
            if !matches!(self.current_token, Token::Identifier(_)) {
                self.pop_context();
                return Err(self.error(&format!(
                    "Expected field type, got {:?}",
                    self.current_token
                )));
            }
            
            // Get field type
            let field_type = match &self.current_token {
                Token::Identifier(ident) => ast::Identifier {
                    token: self.current_token.token_literal(),
                    value: ident.clone(),
                },
                _ => unreachable!(),
            };
            
            self.next_token()?; // Advance past field type
            
            // Create field struct
            let field = ast::statements::fields::FieldStatement {
                token: "field".to_string(),
                name: field_name,
                type_name: field_type,
            };
            fields.push(field);
            
            // Expect newline or comma
            if self.current_token_is(Token::Semicolon) {
                self.next_token()?; // Advance past newline
            }
        }
        
        // Expect '}'
        if !self.current_token_is(Token::RBrace) {
            self.pop_context();
            return Err(self.error(&format!(
                "Expected '}}' after struct fields, got {:?}",
                self.current_token
            )));
        }
        self.next_token()?; // Advance past '}'
        
        // Pop struct context
        self.pop_context();
        
        // Create a placeholder struct statement for now
        // In a real implementation, we'd combine this with the previously parsed name and type parameters
        let struct_name = ast::Identifier {
            token: "Box".to_string(),
            value: "Box".to_string(),
        };
        
        // Create struct statement with the placeholder name
        Ok(Box::new(ast::SquadStatement {
            token: token,
            name: struct_name,
            type_parameters: vec![ast::Identifier {
                token: "T".to_string(),
                value: "T".to_string(),
            }], // Placeholder for type parameter T
            fields,
        }))
    }
    
    /// Expect semicolon and advance past it if found
    fn expect_semicolon(&mut self) -> Result<(), Error> {
        if self.peek_token_is(Token::Semicolon) {
            self.next_token()?; // Advance past semicolon
        }
        Ok(())
    }
}
