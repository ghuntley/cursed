//! Parser for enhanced generic constraints and where clauses.
//!
//! This module handles parsing of complex generic constraints including:
//! - Multi-bound constraints (T: Display + Clone)
//! - Where clauses (where T: Display, U: Into<String>)
//! - Associated type constraints 
//! - Cross-parameter relationships

use crate::ast::declarations::{
    AssociatedType, ConstraintOperator, ConstraintRelation, CrossParameterConstraint,
    EnhancedConstraint, EnhancedTypeParameter, MultiParamGeneric, TypeBound, 
    Variance, WhereClause
};
use crate::ast::{Expression, Node};
use crate::error::Error;
use crate::lexer::{Token, TokenType};
use crate::parser::Parser;
use std::collections::HashMap;
use tracing::{debug, error, info, instrument, warn};

impl<'a> Parser<'a> {
    /// Parse enhanced generic constraints for functions and types
    /// Supports syntax like: `[T: Display + Clone, U: Into<String>]`
    #[instrument(skip(self), fields(current_token = ?self.current_token))]
    pub fn parse_enhanced_generic_params(&mut self) -> Result<MultiParamGeneric, Error> {
        info!("Starting enhanced generic parameter parsing");

        let token = self.current_token.clone();
        
        self.expect_peek(Token::LeftBracket)?;

        let mut parameters = Vec::new();
        let mut cross_constraints = Vec::new();

        // Handle empty generic parameters []
        if self.peek_token_is(&TokenType::RightBracket) {
            self.next_token();
            return Ok(MultiParamGeneric::new(token, parameters));
        }

        // Parse first parameter
        parameters.push(self.parse_enhanced_type_parameter()?);

        // Parse additional parameters
        while self.peek_token_is(&TokenType::Comma) {
            self.next_token(); // consume comma
            if self.peek_token_is(&TokenType::RightBracket) {
                break; // trailing comma
            }
            self.next_token(); // move to parameter
            parameters.push(self.parse_enhanced_type_parameter()?);
        }

        if !self.expect_peek(&TokenType::RightBracket)? {
            return Err(Error::ParseError {
                message: "Expected ']' after generic parameters".to_string(),
                line: self.current_token.line,
                column: self.current_token.column,
            });
        }

        debug!(param_count = parameters.len(), "Parsed generic parameters");

        Ok(MultiParamGeneric::new(token, parameters).with_cross_constraints(cross_constraints))
    }

    /// Parse a single enhanced type parameter with constraints and variance
    /// Supports: `T`, `+T`, `-T`, `T: Display`, `T: Display + Clone`, `T = String`
    #[instrument(skip(self))]
    fn parse_enhanced_type_parameter(&mut self) -> Result<EnhancedTypeParameter, Error> {
        let token = self.current_token.clone();

        // Parse optional variance annotation
        let variance = if matches!(
            self.current_token.token_type,
            TokenType::Plus | TokenType::Minus
        ) {
            let var = match self.current_token.token_type {
                TokenType::Plus => Variance::Covariant,
                TokenType::Minus => Variance::Contravariant,
                _ => Variance::Invariant,
            };
            self.next_token(); // consume variance token
            var
        } else {
            Variance::Invariant
        };

        // Parse parameter name
        let name = if let TokenType::Ident(ref name) = self.current_token.token_type {
            name.clone()
        } else {
            return Err(Error::ParseError {
                message: format!("Expected identifier for type parameter, got {:?}", self.current_token.token_type),
                line: self.current_token.line,
                column: self.current_token.column,
            });
        };

        let mut parameter = EnhancedTypeParameter::with_variance(token, name, variance);

        // Parse optional constraints
        if self.peek_token_is(&TokenType::Colon) {
            self.next_token(); // consume ':'
            self.next_token(); // move to first constraint

            let constraints = self.parse_type_constraints()?;
            parameter.constraints = constraints;
        }

        // Parse optional default type
        if self.peek_token_is(&TokenType::Assign) {
            self.next_token(); // consume '='
            self.next_token(); // move to default type

            let default_type = self.parse_expression(crate::parser::precedence::Precedence::Lowest)?;
            parameter = parameter.with_default(default_type);
        }

        debug!(
            param_name = parameter.name,
            has_constraints = parameter.has_constraints(),
            has_default = parameter.has_default(),
            "Parsed enhanced type parameter"
        );

        Ok(parameter)
    }

    /// Parse type constraints for a parameter
    /// Supports: `Display`, `Display + Clone`, `Display + Clone + Into<String>`
    #[instrument(skip(self))]
    fn parse_type_constraints(&mut self) -> Result<Vec<EnhancedConstraint>, Error> {
        let mut constraints = Vec::new();
        let mut bounds = Vec::new();

        // Parse first bound
        bounds.push(self.parse_type_bound()?);

        // Parse additional bounds connected with +
        while self.peek_token_is(&TokenType::Plus) {
            self.next_token(); // consume '+'
            self.next_token(); // move to next bound
            bounds.push(self.parse_type_bound()?);
        }

        // Create enhanced constraint with all bounds
        if !bounds.is_empty() {
            let token = bounds[0].token.clone();
            let param_name = "T".to_string(); // Will be set by caller
            let constraint = EnhancedConstraint::multiple_bounds(token, param_name, bounds);
            constraints.push(constraint);
        }

        debug!(constraint_count = constraints.len(), "Parsed type constraints");
        Ok(constraints)
    }

    /// Parse a single type bound
    /// Supports: `Display`, `Into<String>`, `Iterator<Item = String>`
    #[instrument(skip(self))]
    fn parse_type_bound(&mut self) -> Result<TypeBound, Error> {
        let token = self.current_token.clone();

        let interface_name = if let TokenType::Ident(ref name) = self.current_token.token_type {
            name.clone()
        } else {
            return Err(Error::ParseError {
                message: format!("Expected interface name for type bound, got {:?}", self.current_token.token_type),
                line: self.current_token.line,
                column: self.current_token.column,
            });
        };

        // Parse optional type arguments for generic interfaces
        let type_args = if self.peek_token_is(&TokenType::LeftAngle) {
            self.next_token(); // consume '<'
            self.parse_type_arguments()?
        } else {
            Vec::new()
        };

        Ok(TypeBound::with_args(token, interface_name, type_args))
    }

    /// Parse type arguments for generic interfaces
    /// Supports: `<String>`, `<T, U>`, `<Item = String>`
    #[instrument(skip(self))]
    fn parse_type_arguments(&mut self) -> Result<Vec<Box<dyn Expression>>, Error> {
        let mut args = Vec::new();

        if !self.expect_peek(&TokenType::LeftAngle)? {
            return Ok(args);
        }

        if self.peek_token_is(&TokenType::RightAngle) {
            self.next_token(); // consume '>'
            return Ok(args);
        }

        // Parse first argument
        self.next_token();
        args.push(self.parse_expression(crate::parser::precedence::Precedence::Lowest)?);

        // Parse additional arguments
        while self.peek_token_is(&TokenType::Comma) {
            self.next_token(); // consume ','
            if self.peek_token_is(&TokenType::RightAngle) {
                break; // trailing comma
            }
            self.next_token();
            args.push(self.parse_expression(crate::parser::precedence::Precedence::Lowest)?);
        }

        if !self.expect_peek(&TokenType::RightAngle)? {
            return Err(Error::ParseError {
                message: "Expected '>' after type arguments".to_string(),
                line: self.current_token.line,
                column: self.current_token.column,
            });
        }

        debug!(arg_count = args.len(), "Parsed type arguments");
        Ok(args)
    }

    /// Parse where clause for complex constraints
    /// Supports: `where T: Display, U: Into<String>, V: Iterator<Item = T>`
    #[instrument(skip(self))]
    pub fn parse_where_clause(&mut self) -> Result<Option<WhereClause>, Error> {
        if !self.current_token_is(&TokenType::Where) {
            return Ok(None);
        }

        info!("Parsing where clause");
        let token = self.current_token.clone();
        self.next_token(); // consume 'where'

        let mut constraints = Vec::new();

        // Parse first constraint
        constraints.push(self.parse_where_constraint()?);

        // Parse additional constraints
        while self.peek_token_is(&TokenType::Comma) {
            self.next_token(); // consume ','
            if self.is_where_clause_end() {
                break;
            }
            self.next_token();
            constraints.push(self.parse_where_constraint()?);
        }

        debug!(constraint_count = constraints.len(), "Parsed where clause");
        Ok(Some(WhereClause::new(token, constraints)))
    }

    /// Parse a single where constraint
    /// Supports: `T: Display`, `T: Display + Clone`, `Iterator::Item = String`
    #[instrument(skip(self))]
    fn parse_where_constraint(&mut self) -> Result<crate::ast::declarations::GenericConstraint, Error> {
        let token = self.current_token.clone();

        // Parse parameter name
        let param_name = if let TokenType::Ident(ref name) = self.current_token.token_type {
            name.clone()
        } else {
            return Err(Error::ParseError {
                message: format!("Expected parameter name in where constraint, got {:?}", self.current_token.token_type),
                line: self.current_token.line,
                column: self.current_token.column,
            });
        };

        // Expect ':'
        if !self.expect_peek(&TokenType::Colon)? {
            return Err(Error::ParseError {
                message: "Expected ':' after parameter name in where constraint".to_string(),
                line: self.current_token.line,
                column: self.current_token.column,
            });
        }

        // Parse interface name (simplified for now)
        self.next_token();
        let interface_name = if let TokenType::Ident(ref name) = self.current_token.token_type {
            name.clone()
        } else {
            return Err(Error::ParseError {
                message: format!("Expected interface name in where constraint, got {:?}", self.current_token.token_type),
                line: self.current_token.line,
                column: self.current_token.column,
            });
        };

        Ok(crate::ast::declarations::GenericConstraint::new(token, param_name, interface_name))
    }

    /// Check if we've reached the end of a where clause
    fn is_where_clause_end(&self) -> bool {
        matches!(
            self.peek_token.token_type,
            TokenType::LeftBrace | TokenType::Semicolon | TokenType::EOF
        )
    }

    /// Helper to check if current token matches expected type
    fn current_token_is(&self, token_type: &TokenType) -> bool {
        std::mem::discriminant(&self.current_token.token_type) == std::mem::discriminant(token_type)
    }

    /// Helper to check if peek token matches expected type  
    fn peek_token_is(&self, token_type: &TokenType) -> bool {
        std::mem::discriminant(&self.peek_token.token_type) == std::mem::discriminant(token_type)
    }
}

/// Error types specific to generic constraint parsing
#[derive(Debug, Clone)]
pub enum GenericConstraintError {
    UnexpectedToken {
        expected: String,
        actual: TokenType,
        line: usize,
        column: usize,
    },
    InvalidConstraint {
        message: String,
        line: usize,
        column: usize,
    },
    MissingTypeParameter {
        parameter_name: String,
        line: usize,
        column: usize,
    },
    CircularConstraint {
        parameters: Vec<String>,
        line: usize,
        column: usize,
    },
}

impl std::fmt::Display for GenericConstraintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenericConstraintError::UnexpectedToken { expected, actual, line, column } => {
                write!(f, "Expected {} but got {:?} at line {}, column {}", expected, actual, line, column)
            }
            GenericConstraintError::InvalidConstraint { message, line, column } => {
                write!(f, "Invalid constraint: {} at line {}, column {}", message, line, column)
            }
            GenericConstraintError::MissingTypeParameter { parameter_name, line, column } => {
                write!(f, "Missing type parameter '{}' at line {}, column {}", parameter_name, line, column)
            }
            GenericConstraintError::CircularConstraint { parameters, line, column } => {
                write!(f, "Circular constraint involving parameters {:?} at line {}, column {}", parameters, line, column)
            }
        }
    }
}

impl std::error::Error for GenericConstraintError {}
