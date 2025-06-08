//! Parser implementation for type switch statements in CURSED.
//!
//! This module provides functions to parse type switch statements that allow
//! branching based on the runtime type of an interface value.

use crate::ast::control_flow::type_switch::{TypeSwitchStatement, TypeCase, DefaultTypeCase, TypePattern};
use crate::ast::traits::{Expression, Statement};
use crate::error::Error;
use crate::lexer::Token;
use crate::parser::{Parser, context::{ContextAwareParsing, ParsingContext}, precedence::Precedence};

impl<'a> Parser<'a> {
    /// Parse a type switch statement (vibe_check with type assertion) in CURSED
    ///
    /// This parses a type switch statement with the following forms:
    /// 
    /// Simple type switch:
    /// ```cursed
    /// vibe_check value.(type) {
    ///     case int:
    ///         // handle as int
    ///     case string, []byte:
    ///         // handle as string or byte slice
    ///     default:
    ///         // handle unknown type
    /// }
    /// ```
    ///
    /// Type switch with variable binding:
    /// ```cursed
    /// vibe_check v := value.(type) {
    ///     case int:
    ///         // v is bound as int
    ///     case string:
    ///         // v is bound as string
    ///     default:
    ///         // v remains interface type
    /// }
    /// ```
    ///
    /// # Returns
    ///
    /// A Result containing a boxed TypeSwitchStatement if successful, or an Error if parsing fails.
    pub(super) fn parse_type_switch_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        // Push type switch statement context
        self.push_context(ParsingContext::SwitchStatement);
        
        // Remember the current token for use in AST
        let token = self.current_token.clone();
        
        // Skip past 'vibe_check' token
        self.next_token()?;
        
        // Parse the expression and check for type assertion pattern
        let (expression, variable_name) = self.parse_type_switch_expression()?;
        
        // Handle LBrace if present, consume it
        if !self.current_token_is(Token::LBrace) {
            // Pop context before returning error
            self.pop_context();
            return Err(self.error(&format!(
                "Expected '{{' after type switch expression, got {:?}",
                self.current_token
            )));
        }
        
        // Move past opening brace
        self.next_token()?; // Skip past '{'
        
        // Push switch body context
        self.push_context(ParsingContext::SwitchBody);
        
        // Parse type cases
        let mut cases = Vec::new();
        let mut default_case = None;

        while !self.current_token_is(Token::RBrace) && !self.current_token_is(Token::Eof) {
            if self.current_token_is(Token::Case) {
                // Parse type case
                let case = self.parse_type_case()?;
                cases.push(case);
            } else if self.current_token_is(Token::Basic) {
                // Parse default case  
                let default = self.parse_default_type_case()?;
                default_case = Some(default);
            } else {
                // Pop contexts before returning error
                self.pop_context(); // Pop switch body
                self.pop_context(); // Pop switch statement
                return Err(self.error(&format!(
                    "Expected 'case' or 'basic' in type switch statement, got {:?}",
                    self.current_token
                )));
            }
        }

        // Expect closing brace
        if !self.current_token_is(Token::RBrace) {
            // Pop contexts before returning error
            self.pop_context(); // Pop switch body
            self.pop_context(); // Pop switch statement
            return Err(self.error(&format!(
                "Expected '}}' at end of type switch statement, got {:?}",
                self.current_token
            )));
        }
        
        self.next_token()?; // Advance past '}'
        
        // Pop switch body context
        self.pop_context();
        
        // Pop switch statement context
        self.pop_context();

        Ok(Box::new(TypeSwitchStatement {
            token: token.token_literal(),
            expression,
            variable_name,
            cases,
            default_case,
        }))
    }
    
    /// Parse the expression part of a type switch, detecting variable binding
    ///
    /// Handles both:
    /// - `value.(type)` - simple type switch
    /// - `v := value.(type)` - type switch with variable binding
    fn parse_type_switch_expression(&mut self) -> Result<(Box<dyn Expression>, Option<String>), Error> {
        // Check if this is a variable binding (identifier := expression.(type))
        if let Token::Identifier(var_name) = &self.current_token {
            let potential_var = var_name.clone();
            
            // Look ahead to see if we have := pattern
            if self.peek_token_is(Token::Assign) {
                // This is variable binding: v := expr.(type)
                self.next_token()?; // Skip past identifier
                self.next_token()?; // Skip past :=
                
                // Parse the expression  
                let expr = self.parse_expression(Precedence::Lowest)?;
                
                // Verify it's a type assertion
                if !self.is_type_assertion(&expr) {
                    return Err(self.error(
                        "Expected type assertion expression (expr.(type)) in type switch"
                    ));
                }
                
                return Ok((expr, Some(potential_var)));
            }
        }
        
        // Simple type switch: expr.(type)
        let expr = self.parse_expression(Precedence::Lowest)?;
        
        // Verify it's a type assertion
        if !self.is_type_assertion(&expr) {
            return Err(self.error(
                "Expected type assertion expression (expr.(type)) in type switch"
            ));
        }
        
        Ok((expr, None))
    }
    
    /// Check if an expression is a type assertion (ends with .(type))
    fn is_type_assertion(&self, expr: &Box<dyn Expression>) -> bool {
        // For now, we'll check if the string representation contains .(type)
        // In a full implementation, we'd check the AST node type
        expr.string().contains(".(type)")
    }
    
    /// Parse a type case clause
    ///
    /// Format: `case type1, type2, type3: statements`
    fn parse_type_case(&mut self) -> Result<TypeCase, Error> {
        self.push_context(ParsingContext::CaseClause);
        
        // Skip past 'case' token
        self.next_token()?;
        
        // Parse the type list (comma-separated)
        let mut types = Vec::new();
        
        // Parse first type
        if let Token::Identifier(type_name) = &self.current_token {
            types.push(type_name.clone());
            self.next_token()?;
        } else {
            self.pop_context();
            return Err(self.error(&format!(
                "Expected type name in case clause, got {:?}",
                self.current_token
            )));
        }
        
        // Parse additional types separated by commas
        while self.current_token_is(Token::Comma) {
            self.next_token()?; // Skip comma
            
            if let Token::Identifier(type_name) = &self.current_token {
                types.push(type_name.clone());
                self.next_token()?;
            } else {
                self.pop_context();
                return Err(self.error(&format!(
                    "Expected type name after comma in case clause, got {:?}",
                    self.current_token
                )));
            }
        }
        
        // Expect colon
        if !self.current_token_is(Token::Colon) {
            self.pop_context();
            return Err(self.error(&format!(
                "Expected ':' after case types, got {:?}",
                self.current_token
            )));
        }
        
        self.next_token()?; // Skip colon
        
        // Parse case body statements
        let mut statements = Vec::new();
        while !self.current_token_is(Token::Case) &&
              !self.current_token_is(Token::Basic) &&
              !self.current_token_is(Token::RBrace) &&
              !self.current_token_is(Token::Eof) {
            statements.push(self.parse_statement()?);
            self.next_token()?;
        }
        
        self.pop_context();
        
        Ok(TypeCase {
            types,
            statements,
        })
    }
    
    /// Parse a default case clause
    ///
    /// Format: `basic: statements` or `default: statements`
    fn parse_default_type_case(&mut self) -> Result<DefaultTypeCase, Error> {
        self.push_context(ParsingContext::DefaultClause);
        
        // Skip past 'basic' token
        self.next_token()?;
        
        // Expect colon
        if !self.current_token_is(Token::Colon) {
            self.pop_context();
            return Err(self.error(&format!(
                "Expected ':' after 'basic', got {:?}",
                self.current_token
            )));
        }
        
        self.next_token()?; // Skip colon
        
        // Parse default case body statements
        let mut statements = Vec::new();
        while !self.current_token_is(Token::Case) &&
              !self.current_token_is(Token::Basic) &&
              !self.current_token_is(Token::RBrace) &&
              !self.current_token_is(Token::Eof) {
            statements.push(self.parse_statement()?);
            self.next_token()?;
        }
        
        self.pop_context();
        
        Ok(DefaultTypeCase {
            statements,
        })
    }
}
