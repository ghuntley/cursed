/// Type Switch Parser for CURSED language
/// Handles parsing of type switch statements with `vibe_check expr.(type)` syntax

use crate::parser::Parser;
use crate::ast::conditionals::{SwitchStatement, SwitchCase};
use crate::ast::expressions::{TypeAssertion, TypeAssertionQuestion};
use crate::ast::traits::{Statement, Expression};
use crate::ast::block::BlockStatement;
use crate::lexer::TokenType;
use crate::error::Error;

impl Parser {
    /// Parse type switch statement (vibe_check expr.(type))
    /// 
    /// Type switch syntax:
    /// ```cursed
    /// vibe_check expr.(type) {
    ///     mood Type1:
    ///         // statements
    ///     mood Type2, Type3:
    ///         // statements
    ///     basic:
    ///         // default case
    /// }
    /// ```
    /// 
    /// Or with variable binding:
    /// ```cursed
    /// vibe_check var := expr.(type) {
    ///     mood Type1:
    ///         // var is available as Type1
    /// }
    /// ```
    pub fn parse_type_switch_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.expect_token(TokenType::VibeCheck)?;
        
        // Parse the expression which should be a type assertion
        let switch_expr = self.parse_type_switch_expression()?;
        
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
                
                // Parse type case labels (mood Type1, Type2:)
                let mut case_types = vec![self.parse_type_case_label()?];
                
                // Handle multiple types in one case
                while self.current_token_is(&TokenType::Comma) {
                    self.advance_token()?;
                    case_types.push(self.parse_type_case_label()?);
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
                
                let block = BlockStatement::new(
                    "{".to_string(),
                    statements
                );
                cases.push(SwitchCase::new(case_types, block));
                
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
                return Err(Error::Parse("Expected 'mood' or 'basic' in type switch statement".to_string()));
            }
        }
        
        self.expect_token(TokenType::RightBrace)?;
        
        // Convert default case to BlockStatement if present
        let default_block = if let Some(statements) = default_case {
            Some(BlockStatement::new("{".to_string(), statements))
        } else {
            None
        };
        
        Ok(Box::new(TypeSwitchStatement {
            token: token.literal,
            switch_expression: switch_expr,
            cases,
            default_case: default_block,
        }))
    }
    
    /// Parse type switch expression (handles both simple expr.(type) and var := expr.(type))
    fn parse_type_switch_expression(&mut self) -> Result<Box<dyn Expression>, Error> {
        // Check for variable binding syntax (var := expr.(type))
        if self.current_token_is(&TokenType::Identifier) && self.peek_token_is(&TokenType::ShortVarDecl) {
            // Parse variable binding
            let var_name = self.current_token.literal.clone();
            self.advance_token()?; // consume identifier
            self.advance_token()?; // consume :=
            
            // Parse the type assertion expression
            let type_assertion_expr = self.parse_type_assertion_expression()?;
            
            // Create a binding expression that combines the variable and type assertion
            Ok(Box::new(TypeSwitchBinding::new(var_name, type_assertion_expr)))
        } else {
            // Parse simple type assertion
            self.parse_type_assertion_expression()
        }
    }
    
    /// Parse type assertion expression (expr.(type) or expr.(type)?)
    fn parse_type_assertion_expression(&mut self) -> Result<Box<dyn Expression>, Error> {
        let expr = self.parse_primary_expression()?;
        
        // Check for .(type) syntax
        if self.current_token_is(&TokenType::Dot) {
            self.advance_token()?; // consume .
            self.expect_token(TokenType::LeftParen)?;
            
            // Parse the type inside parentheses
            let type_text = if self.current_token_is(&TokenType::Identifier) {
                self.current_token.literal.clone()
            } else {
                return Err(Error::Parse("Expected type name in type assertion".to_string()));
            };
            self.advance_token()?;
            
            self.expect_token(TokenType::RightParen)?;
            
            // Check for question mark (error propagation)
            if self.current_token_is(&TokenType::Question) {
                self.advance_token()?;
                Ok(Box::new(TypeAssertionQuestion::new(
                    expr.string(),
                    type_text
                )))
            } else {
                Ok(Box::new(TypeAssertion::new(
                    expr.string(),
                    type_text
                )))
            }
        } else {
            Err(Error::Parse("Expected type assertion syntax '.(type)' in type switch".to_string()))
        }
    }
    
    /// Parse type case label (just the type name in mood clauses)
    fn parse_type_case_label(&mut self) -> Result<Box<dyn Expression>, Error> {
        if self.current_token_is(&TokenType::Identifier) {
            let type_name = self.current_token.literal.clone();
            self.advance_token()?;
            Ok(Box::new(TypeLiteral::new(type_name)))
        } else {
            Err(Error::Parse("Expected type name in type switch case".to_string()))
        }
    }
    
    /// Detect if this is a type switch vs regular switch
    /// Type switches have the pattern: vibe_check (expr|var :=) expr.(type)
    pub fn is_type_switch(&mut self) -> bool {
        // Save current position
        let saved_pos = self.current_position;
        let saved_token = self.current_token.clone();
        let saved_peek = self.peek_token.clone();
        
        // Skip vibe_check token
        if !self.current_token_is(&TokenType::VibeCheck) {
            return false;
        }
        let _ = self.advance_token();
        
        // Check for variable binding (identifier :=)
        if self.current_token_is(&TokenType::Identifier) && self.peek_token_is(&TokenType::ShortVarDecl) {
            let _ = self.advance_token(); // skip identifier
            let _ = self.advance_token(); // skip :=
        }
        
        // Look for type assertion pattern: expr.(type)
        let mut has_type_assertion = false;
        let mut paren_depth = 0;
        
        // Parse through the expression looking for .(
        while !self.current_token_is(&TokenType::LeftBrace) && 
              !self.current_token_is(&TokenType::Eof) {
            
            if self.current_token_is(&TokenType::LeftParen) {
                paren_depth += 1;
            } else if self.current_token_is(&TokenType::RightParen) {
                paren_depth -= 1;
            } else if self.current_token_is(&TokenType::Dot) && paren_depth == 0 {
                // Found a dot, check if next is opening paren
                let _ = self.advance_token();
                if self.current_token_is(&TokenType::LeftParen) {
                    has_type_assertion = true;
                    break;
                }
                continue;
            }
            
            let _ = self.advance_token();
        }
        
        // Restore position
        self.current_position = saved_pos;
        self.current_token = saved_token;
        self.peek_token = saved_peek;
        
        has_type_assertion
    }
}

/// AST node for type switch statements
#[derive(Debug, Clone)]
pub struct TypeSwitchStatement {
    pub token: String,
    pub switch_expression: Box<dyn Expression>,
    pub cases: Vec<SwitchCase>,
    pub default_case: Option<BlockStatement>,
}

impl crate::ast::traits::Node for TypeSwitchStatement {
    fn string(&self) -> String {
        let mut result = format!("vibe_check {} {{\n", self.switch_expression.string());
        
        for case in &self.cases {
            let values: Vec<String> = case.values.iter()
                .map(|v| v.string())
                .collect();
            result.push_str(&format!("  mood {}:\n", values.join(", ")));
            
            for stmt in &case.statements {
                result.push_str(&format!("    {}\n", stmt.string()));
            }
        }
        
        if let Some(ref default) = self.default_case {
            result.push_str("  basic:\n");
            for stmt in &default.statements {
                result.push_str(&format!("    {}\n", stmt.string()));
            }
        }
        
        result.push('}');
        result
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for TypeSwitchStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

/// AST node for type switch variable binding (var := expr.(type))
#[derive(Debug, Clone)]
pub struct TypeSwitchBinding {
    pub variable_name: String,
    pub type_assertion: Box<dyn Expression>,
}

impl TypeSwitchBinding {
    pub fn new(variable_name: String, type_assertion: Box<dyn Expression>) -> Self {
        Self {
            variable_name,
            type_assertion,
        }
    }
}

impl crate::ast::traits::Node for TypeSwitchBinding {
    fn string(&self) -> String {
        format!("{} := {}", self.variable_name, self.type_assertion.string())
    }

    fn token_literal(&self) -> String {
        self.variable_name.clone()
    }
}

impl Expression for TypeSwitchBinding {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

/// AST node for type literals in type switch cases
#[derive(Debug, Clone)]
pub struct TypeLiteral {
    pub type_name: String,
}

impl TypeLiteral {
    pub fn new(type_name: String) -> Self {
        Self { type_name }
    }
}

impl crate::ast::traits::Node for TypeLiteral {
    fn string(&self) -> String {
        self.type_name.clone()
    }

    fn token_literal(&self) -> String {
        self.type_name.clone()
    }
}

impl Expression for TypeLiteral {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}
