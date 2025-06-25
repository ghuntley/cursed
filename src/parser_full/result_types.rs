// Parser support for Result and Option types
//
// This module provides parsing capabilities for Result<T, E> and Option<T> type syntax,
// pattern matching expressions, and error handling constructs.

use crate::parser::Parser;
use crate::lexer::{Token, TokenType};
use crate::ast::traits::Expression;
use crate::ast::identifiers::Identifier;
use crate::types::result::{ResultTypeExpression, OptionTypeExpression};
use crate::error::CursedError;

use std::any::Any;

/// Result and Option type pattern expressions
#[derive(Debug, Clone)]
pub enum ResultPattern {
    /// Ok(value) pattern
    Ok(Box<dyn Expression>),
    /// Err(error) pattern
    Err(Box<dyn Expression>),
    /// Some(value) pattern
    Some(Box<dyn Expression>),
    /// None pattern
    None,
    /// Wildcard pattern _
    Wildcard,
}

impl ResultPattern {
    pub fn string(&self) -> String {
        match self {
            ResultPattern::Ok(expr) => format!("Ok({})", expr.string()),
            ResultPattern::Err(expr) => format!("Err({})", expr.string()),
            ResultPattern::Some(expr) => format!("Some({})", expr.string()),
            ResultPattern::None => "None".to_string(),
            ResultPattern::Wildcard => "_".to_string(),
        }
    }
}

/// Match expression for Result/Option types
#[derive(Debug, Clone)]
pub struct ResultMatchExpression {
    pub token: String,
    pub value: Box<dyn Expression>,
    pub arms: Vec<MatchArm>,
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: ResultPattern,
    pub body: Box<dyn Expression>,
    pub guard: Option<Box<dyn Expression>>,
}

impl ResultMatchExpression {
    pub fn new(
        token: String,
        value: Box<dyn Expression>,
        arms: Vec<MatchArm>,
    ) -> Self {
        Self { token, value, arms }
    }
}

impl crate::ast::traits::Node for ResultMatchExpression {
    fn string(&self) -> String {
        let mut result = format!("match {} {{\n", self.value.string());
        
        for arm in &self.arms {
            result.push_str("  ");
            result.push_str(&arm.pattern.string());
            
            if let Some(guard) = &arm.guard {
                result.push_str(" if ");
                result.push_str(&guard.string());
            }
            
            result.push_str(" => ");
            result.push_str(&arm.body.string());
            result.push_str(",\n");
        }
        
        result.push('}');
        result
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for ResultMatchExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(ResultMatchExpression {
            token: self.token.clone(),
            value: self.value.clone_box(),
            arms: self.arms.iter().map(|arm| MatchArm {
                pattern: arm.pattern.clone(),
                body: arm.body.clone_box(),
                guard: arm.guard.as_ref().map(|g| g.clone_box()),
            }).collect(),
        })
    }
}

/// Try expression for error propagation (? operator)
#[derive(Debug, Clone)]
pub struct TryExpression {
    pub token: String,
    pub expression: Box<dyn Expression>,
}

impl TryExpression {
    pub fn new(token: String, expression: Box<dyn Expression>) -> Self {
        Self { token, expression }
    }
}

impl crate::ast::traits::Node for TryExpression {
    fn string(&self) -> String {
        format!("{}?", self.expression.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for TryExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(TryExpression {
            token: self.token.clone(),
            expression: self.expression.clone_box(),
        })
    }
}

/// Unwrap expression for explicit unwrapping
#[derive(Debug, Clone)]
pub struct UnwrapExpression {
    pub token: String,
    pub expression: Box<dyn Expression>,
    pub default_value: Option<Box<dyn Expression>>,
}

impl UnwrapExpression {
    pub fn new(token: String, expression: Box<dyn Expression>) -> Self {
        Self {
            token,
            expression,
            default_value: None,
        }
    }

    pub fn with_default(
        token: String,
        expression: Box<dyn Expression>,
        default: Box<dyn Expression>,
    ) -> Self {
        Self {
            token,
            expression,
            default_value: Some(default),
        }
    }
}

impl crate::ast::traits::Node for UnwrapExpression {
    fn string(&self) -> String {
        if let Some(default) = &self.default_value {
            format!("{}.unwrap_or({})", self.expression.string(), default.string())
        } else {
            format!("{}.unwrap()", self.expression.string())
        }
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for UnwrapExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(UnwrapExpression {
            token: self.token.clone(),
            expression: self.expression.clone_box(),
            default_value: self.default_value.as_ref().map(|d| d.clone_box()),
        })
    }
}

/// Result constructor expressions
#[derive(Debug, Clone)]
pub enum ResultConstructor {
    Ok(Box<dyn Expression>),
    Err(Box<dyn Expression>),
    Some(Box<dyn Expression>),
    None,
}

impl ResultConstructor {
    pub fn string(&self) -> String {
        match self {
            ResultConstructor::Ok(expr) => format!("Ok({})", expr.string()),
            ResultConstructor::Err(expr) => format!("Err({})", expr.string()),
            ResultConstructor::Some(expr) => format!("Some({})", expr.string()),
            ResultConstructor::None => "None".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResultConstructorExpression {
    pub token: String,
    pub constructor: ResultConstructor,
}

impl ResultConstructorExpression {
    pub fn new(token: String, constructor: ResultConstructor) -> Self {
        Self { token, constructor }
    }
}

impl crate::ast::traits::Node for ResultConstructorExpression {
    fn string(&self) -> String {
        self.constructor.string()
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for ResultConstructorExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(ResultConstructorExpression {
            token: self.token.clone(),
            constructor: match &self.constructor {
                ResultConstructor::Ok(expr) => ResultConstructor::Ok(expr.clone_box()),
                ResultConstructor::Err(expr) => ResultConstructor::Err(expr.clone_box()),
                ResultConstructor::Some(expr) => ResultConstructor::Some(expr.clone_box()),
                ResultConstructor::None => ResultConstructor::None,
            },
        })
    }
}

/// Parser extension for Result/Option types
pub trait ResultTypeParser {
    /// Parse Result<T, E> type expression
    fn parse_result_type(&mut self) -> crate::error::Result<()>;

    /// Parse Option<T> type expression
    fn parse_option_type(&mut self) -> crate::error::Result<()>;

    /// Parse match expression for Result/Option
    fn parse_result_match(&mut self) -> crate::error::Result<()>;

    /// Parse try expression (? operator)
    fn parse_try_expression(&mut self, left: Box<dyn Expression>) -> crate::error::Result<()>;

    /// Parse unwrap expression
    fn parse_unwrap_expression(&mut self, left: Box<dyn Expression>) -> crate::error::Result<()>;

    /// Parse Result/Option constructor
    fn parse_result_constructor(&mut self) -> crate::error::Result<()>;

    /// Parse pattern for match arms
    fn parse_result_pattern(&mut self) -> crate::error::Result<()>;

    /// Parse match arm
    fn parse_match_arm(&mut self) -> crate::error::Result<()>;
}

impl ResultTypeParser for Parser {
    fn parse_result_type(&mut self) -> crate::error::Result<()> {
        let token = self.current_token.literal.clone();
        
        // Expect "Result"
        if !self.current_token_is(&TokenType::Identifier) || self.current_token.literal != "Result" {
            return Err(CursedError::parse_error_with_location(
                "Expected 'Result'".to_string(),
                self.current_token.location.line,
                self.current_token.location.column,
            ));
        }

        self.advance_token()?;

        // Expect "<"
        if !self.current_token_is(&TokenType::LessThan) {
            return Err(CursedError::parse_error_with_location(
                "Expected '<' after 'Result'".to_string(),
                self.current_token.location.line,
                self.current_token.location.column,
            ));
        }
        self.advance_token()?;

        // Parse ok type
        let ok_type = self.parse_expression()?;

        // Expect ","
        if !self.current_token_is(&TokenType::Comma) {
            return Err(CursedError::parse_error_with_location(
                "Expected ',' after ok type".to_string(),
                self.current_token.location.line,
                self.current_token.location.column,
            ));
        }
        self.advance_token()?;

        // Parse error type
        let err_type = self.parse_expression()?;

        // Expect ">"
        if !self.current_token_is(&TokenType::GreaterThan) {
            return Err(CursedError::parse_error_with_location(
                "Expected '>' after error type".to_string(),
                self.current_token.location.line,
                self.current_token.location.column,
            ));
        }

        Ok(ResultTypeExpression::new(token, ok_type, err_type))
    }

    fn parse_option_type(&mut self) -> crate::error::Result<()> {
        let token = self.current_token.literal.clone();
        
        // Expect "Option"
        if !self.current_token_is(&TokenType::Identifier) || self.current_token.literal != "Option" {
            return Err(CursedError::parse_error_with_location(
                "Expected 'Option'".to_string(),
                self.current_token.location.line,
                self.current_token.location.column,
            ));
        }

        self.lexer.next_token();

        // Expect "<"
        if self.expect_token(TokenType::LessThan).is_err() {
            return Err(CursedError::parse_error_with_location(
                "Expected '<' after 'Option'".to_string(),
                self.current_token.location.line,
                self.current_token.location.column,
            ));
        }

        // Parse inner type
        let inner_type = self.parse_expression()?;

        // Expect ">"
        if self.expect_token(TokenType::GreaterThan).is_err() {
            return Err(CursedError::parse_error_with_location(
                "Expected '>' after inner type".to_string(),
                self.current_token.location.line,
                self.current_token.location.column,
            ));
        }

        Ok(OptionTypeExpression::new(token, inner_type))
    }

    fn parse_result_match(&mut self) -> crate::error::Result<()> {
        let token = self.current_token.literal.clone();
        
        // Expect "match"
        if !self.current_token_is(&TokenType::Match) {
            return Err(CursedError::parse_error_with_location(
                "Expected 'match'".to_string(),
                self.current_token.location.line,
                self.current_token.location.column,
            ));
        }

        self.lexer.next_token();

        // Parse value to match
        let value = self.parse_expression()?;

        // Expect "{"
        if self.expect_token(TokenType::LeftBrace).is_err() {
            return Err(CursedError::parse_error_with_location(
                "Expected '{' after match value".to_string(),
                self.current_token.location.line,
                self.current_token.location.column,
            ));
        }

        // Parse match arms
        let mut arms = Vec::new();
        while !self.current_token_is(&TokenType::RightBrace) && !self.current_token_is(&TokenType::Eof) {
            let arm = self.parse_match_arm()?;
            arms.push(arm);

            // Optional comma
            if self.current_token_is(&TokenType::Comma) {
                self.lexer.next_token();
            }
        }

        // Expect "}"
        if self.expect_token(TokenType::RightBrace).is_err() {
            return Err(CursedError::parse_error_with_location(
                "Expected '}' after match arms".to_string(),
                self.current_token.location.line,
                self.current_token.location.column,
            ));
        }

        Ok(ResultMatchExpression::new(token, value, arms))
    }

    fn parse_try_expression(&mut self, left: Box<dyn Expression>) -> crate::error::Result<()> {
        let token = self.current_token.literal.clone();
        
        // Current token should be "?"
        if !self.current_token_is(&TokenType::Question) {
            return Err(CursedError::parse_error_with_location(
                "Expected '?' for try expression".to_string(),
                self.current_token.location.line,
                self.current_token.location.column,
            ));
        }

        self.lexer.next_token();

        Ok(TryExpression::new(token, left))
    }

    fn parse_unwrap_expression(&mut self, left: Box<dyn Expression>) -> crate::error::Result<()> {
        let token = self.current_token.literal.clone();
        
        // Expect ".unwrap"
        if !self.current_token_is(&TokenType::Dot) {
            return Err(CursedError::parse_error_with_location(
                "Expected '.' for method call".to_string(),
                self.current_token.location.line,
                self.current_token.location.column,
            ));
        }

        self.lexer.next_token();

        if !self.current_token_is(&TokenType::Identifier) {
            return Err(CursedError::parse_error_with_location(
                "Expected method name after '.'".to_string(),
                self.current_token.location.line,
                self.current_token.location.column,
            ));
        }

        let method_name = self.current_token.literal.clone();
        
        match method_name.as_str() {
            "unwrap" => {
                self.lexer.next_token();
                
                // Expect "()"
                if self.expect_token(TokenType::LeftParen).is_err() {
                    return Err(CursedError::parse_error_with_location(
                        "Expected '(' after 'unwrap'".to_string(),
                        self.current_token.location.line,
                        self.current_token.location.column,
                    ));
                }

                if self.expect_token(TokenType::RightParen).is_err() {
                    return Err(CursedError::parse_error_with_location(
                        "Expected ')' after 'unwrap('".to_string(),
                        self.current_token.location.line,
                        self.current_token.location.column,
                    ));
                }

                Ok(UnwrapExpression::new(token, left))
            }
            "unwrap_or" => {
                self.lexer.next_token();
                
                // Expect "("
                if self.expect_token(TokenType::LeftParen).is_err() {
                    return Err(CursedError::parse_error_with_location(
                        "Expected '(' after 'unwrap_or'".to_string(),
                        self.current_token.location.line,
                        self.current_token.location.column,
                    ));
                }

                // Parse default value
                let default_value = self.parse_expression()?;

                // Expect ")"
                if self.expect_token(TokenType::RightParen).is_err() {
                    return Err(CursedError::parse_error_with_location(
                        "Expected ')' after default value".to_string(),
                        self.current_token.location.line,
                        self.current_token.location.column,
                    ));
                }

                Ok(UnwrapExpression::with_default(token, left, default_value))
            }
            _ => Err(CursedError::parse_error_with_location(
                format!("Unknown method: {}", method_name),
                self.current_token.location.line,
                self.current_token.location.column,
            )),
        }
    }

    fn parse_result_constructor(&mut self) -> crate::error::Result<()> {
        let token = self.current_token.literal.clone();
        
        if !self.current_token_is(&TokenType::Identifier) {
            return Err(CursedError::parse_error_with_location(
                "Expected constructor name".to_string(),
                self.current_token.location.line,
                self.current_token.location.column,
            ));
        }

        let constructor_name = self.current_token.literal.clone();
        self.lexer.next_token();

        let constructor = match constructor_name.as_str() {
            "Ok" => {
                // Expect "("
                if self.expect_token(TokenType::LeftParen).is_err() {
                    return Err(CursedError::parse_error_with_location(
                        "Expected '(' after 'Ok'".to_string(),
                        self.current_token.location.line,
                        self.current_token.location.column,
                    ));
                }

                // Parse value
                let value = self.parse_expression()?;

                // Expect ")"
                if self.expect_token(TokenType::RightParen).is_err() {
                    return Err(CursedError::parse_error_with_location(
                        "Expected ')' after Ok value".to_string(),
                        self.current_token.location.line,
                        self.current_token.location.column,
                    ));
                }

                ResultConstructor::Ok(value)
            }
            "Err" => {
                // Expect "("
                if self.expect_token(TokenType::LeftParen).is_err() {
                    return Err(CursedError::parse_error_with_location(
                        "Expected '(' after 'Err'".to_string(),
                        self.current_token.location.line,
                        self.current_token.location.column,
                    ));
                }

                // Parse error
                let error = self.parse_expression()?;

                // Expect ")"
                if self.expect_token(TokenType::RightParen).is_err() {
                    return Err(CursedError::parse_error_with_location(
                        "Expected ')' after Err value".to_string(),
                        self.current_token.location.line,
                        self.current_token.location.column,
                    ));
                }

                ResultConstructor::Err(error)
            }
            "Some" => {
                // Expect "("
                if self.expect_token(TokenType::LeftParen).is_err() {
                    return Err(CursedError::parse_error_with_location(
                        "Expected '(' after 'Some'".to_string(),
                        self.current_token.location.line,
                        self.current_token.location.column,
                    ));
                }

                // Parse value
                let value = self.parse_expression()?;

                // Expect ")"
                if self.expect_token(TokenType::RightParen).is_err() {
                    return Err(CursedError::parse_error_with_location(
                        "Expected ')' after Some value".to_string(),
                        self.current_token.location.line,
                        self.current_token.location.column,
                    ));
                }

                ResultConstructor::Some(value)
            }
            "None" => {
                ResultConstructor::None
            }
            _ => {
                return Err(CursedError::parse_error_with_location(
                    format!("Unknown constructor: {}", constructor_name),
                    self.current_token.location.line,
                    self.current_token.location.column,
                ));
            }
        };

        Ok(ResultConstructorExpression::new(token, constructor))
    }

    fn parse_result_pattern(&mut self) -> crate::error::Result<()> {
        if !self.current_token_is(&TokenType::Identifier) {
            return Err(CursedError::parse_error_with_location(
                "Expected pattern".to_string(),
                self.current_token.location.line,
                self.current_token.location.column,
            ));
        }

        let pattern_name = self.current_token.literal.clone();
        self.lexer.next_token();

        match pattern_name.as_str() {
            "Ok" => {
                // Expect "("
                if self.expect_token(TokenType::LeftParen).is_err() {
                    return Err(CursedError::parse_error_with_location(
                        "Expected '(' after 'Ok'".to_string(),
                        self.current_token.location.line,
                        self.current_token.location.column,
                    ));
                }

                // Parse pattern
                let pattern = self.parse_expression()?;

                // Expect ")"
                if self.expect_token(TokenType::RightParen).is_err() {
                    return Err(CursedError::parse_error_with_location(
                        "Expected ')' after Ok pattern".to_string(),
                        self.current_token.location.line,
                        self.current_token.location.column,
                    ));
                }

                Ok(ResultPattern::Ok(pattern))
            }
            "Err" => {
                // Expect "("
                if self.expect_token(TokenType::LeftParen).is_err() {
                    return Err(CursedError::parse_error_with_location(
                        "Expected '(' after 'Err'".to_string(),
                        self.current_token.location.line,
                        self.current_token.location.column,
                    ));
                }

                // Parse pattern
                let pattern = self.parse_expression()?;

                // Expect ")"
                if self.expect_token(TokenType::RightParen).is_err() {
                    return Err(CursedError::parse_error_with_location(
                        "Expected ')' after Err pattern".to_string(),
                        self.current_token.location.line,
                        self.current_token.location.column,
                    ));
                }

                Ok(ResultPattern::Err(pattern))
            }
            "Some" => {
                // Expect "("
                if self.expect_token(TokenType::LeftParen).is_err() {
                    return Err(CursedError::parse_error_with_location(
                        "Expected '(' after 'Some'".to_string(),
                        self.current_token.location.line,
                        self.current_token.location.column,
                    ));
                }

                // Parse pattern
                let pattern = self.parse_expression()?;

                // Expect ")"
                if self.expect_token(TokenType::RightParen).is_err() {
                    return Err(CursedError::parse_error_with_location(
                        "Expected ')' after Some pattern".to_string(),
                        self.current_token.location.line,
                        self.current_token.location.column,
                    ));
                }

                Ok(ResultPattern::Some(pattern))
            }
            "None" => {
                Ok(ResultPattern::None)
            }
            "_" => {
                Ok(ResultPattern::Wildcard)
            }
            _ => {
                Err(CursedError::parse_error_with_location(
                    format!("Unknown pattern: {}", pattern_name),
                    self.current_token.location.line,
                    self.current_token.location.column,
                ))
            }
        }
    }

    fn parse_match_arm(&mut self) -> crate::error::Result<()> {
        // Parse pattern
        let pattern = self.parse_result_pattern()?;

        // Optional guard (if condition)
        let guard = if self.current_token_is(&TokenType::If) {
            self.lexer.next_token();
            Some(self.parse_expression()?)
        } else {
            None
        };

        // Expect "=>"
        if !self.current_token_is(&TokenType::Assign) {
            return Err(CursedError::parse_error_with_location(
                "Expected '=>' after pattern".to_string(),
                self.current_token.location.line,
                self.current_token.location.column,
            ));
        }

        self.lexer.next_token();

        // Parse body
        let body = self.parse_expression()?;

        Ok(MatchArm {
            pattern,
            body,
            guard,
        })
    }
}

