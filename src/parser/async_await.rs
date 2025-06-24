/// Parser for async/await syntax in the CURSED programming language
/// 
/// This module handles parsing of:
/// - Async function declarations: slay async function_name() -> ReturnType { body }
/// - Await expressions: await expression

use crate::parser::Parser;
use crate::lexer::{TokenType, Token};
use crate::ast::{
use crate::error::Error;
    declarations::async_function::{AsyncFunctionStatement, AsyncFunctionDeclaration},
    expressions::await_expression::{AwaitExpression, AwaitAssignment, AsyncBlockExpression, SourceLocation},
    identifiers::Identifier,
    expressions::Parameter,
    block::BlockStatement,
    traits::{Statement, Expression},
};
use crate::error::{Error, ParseError};

impl Parser {
    /// Parse an async function declaration
    /// Expected syntax: slay async identifier(parameters) -> return_type { body }
    pub fn parse_async_function(&mut self) -> Result<(), Error> {
        // Expect 'slay' token
        self.expect_token(TokenType::Slay)?;
        let function_token = self.previous_token().literal.clone();

        // Expect 'async' token
        self.expect_token(TokenType::Async)?;

        // Parse function name
        let name_token = self.expect_token(TokenType::Identifier)?;
        let name = Identifier::new(name_token.literal.clone(), name_token.literal);

        // Parse parameters
        self.expect_token(TokenType::LeftParen)?;
        let parameters = self.parse_function_parameters()?;
        self.expect_token(TokenType::RightParen)?;

        // Parse optional return type
        let return_type = if self.current_token_is(TokenType::Arrow) {
            self.advance(); // consume '->'
            Some(self.parse_expression()?)
        } else {
            None
        };

        // Parse function body
        let body = self.parse_block_statement()?;

        Ok(Box::new(AsyncFunctionStatement::new(
            function_token,
            name,
            parameters,
            return_type,
            body,
        )))
    }

    /// Parse an await expression
    /// Expected syntax: await expression
    pub fn parse_await_expression(&mut self) -> Result<(), Error> {
        let await_token = self.expect_token(TokenType::Await)?;
        
        // Get source location for better error reporting
        let location = SourceLocation {
            line: await_token.location.line,
            column: await_token.location.column,
            file: await_token.location.file.clone(),
        };

        // Parse the expression being awaited
        let expression = self.parse_expression()?;

        // Validate that we're awaiting a valid expression
        self.validate_await_expression(&expression)?;

        Ok(Box::new(AwaitExpression::with_location(
            await_token.literal,
            expression,
            location,
        )))
    }

    /// Parse an await assignment statement
    /// Expected syntax: facts variable = await expression
    pub fn parse_await_assignment(&mut self) -> Result<(), Error> {
        // Parse variable declaration
        let var_token = self.expect_token(TokenType::Facts)?;
        let var_name_token = self.expect_token(TokenType::Identifier)?;
        let var_name = var_name_token.literal;

        // Optional type annotation
        let var_type = if self.current_token_is(TokenType::Identifier) {
            let type_token = self.advance().literal;
            Some(type_token)
        } else {
            None
        };

        // Expect assignment
        self.expect_token(TokenType::Assign)?;

        // Parse await expression
        let await_expr = if self.current_token_is(TokenType::Await) {
            let await_token = self.advance();
            let expression = self.parse_expression()?;
            
            let location = SourceLocation {
                line: await_token.location.line,
                column: await_token.location.column,
                file: await_token.location.file.clone(),
            };

            AwaitExpression::with_location(await_token.literal, expression, location)
        } else {
            return Err(Error::ParseError(ParseError {
                message: "Expected 'await' expression in async assignment".to_string(),
                location: self.current_token().location.clone(),
            }));
        };

        Ok(Box::new(AwaitAssignment::new(
            var_token.literal,
            var_name,
            await_expr,
            var_type,
        )))
    }

    /// Parse an async block with multiple await expressions
    pub fn parse_async_block(&mut self) -> Result<(), Error> {
        let async_token = self.expect_token(TokenType::Async)?;
        self.expect_token(TokenType::LeftBrace)?;

        let mut async_block = AsyncBlockExpression::new(async_token.literal);

        // Parse statements and await expressions until we hit the closing brace
        while !self.current_token_is(TokenType::RightBrace) && !self.current_token_is(TokenType::Eof) {
            if self.current_token_is(TokenType::Await) {
                // Parse await expression
                let await_expr = self.parse_await_expression()?;
                if let Some(await_expr) = await_expr.as_any().downcast_ref::<AwaitExpression>() {
                    async_block.add_await(await_expr.clone());
                }
            } else {
                // Parse regular statement/expression
                let stmt = self.parse_expression()?;
                async_block.add_statement(stmt);
            }

            // Optional semicolon
            if self.current_token_is(TokenType::Semicolon) {
                self.advance();
            }
        }

        self.expect_token(TokenType::RightBrace)?;
        Ok(Box::new(async_block))
    }

    /// Validate that an expression is valid for awaiting
    fn validate_await_expression(&self, expression: &Box<dyn Expression>) -> Result<(), Error> {
        let expr_str = expression.string();
        
        // Basic validation - ensure it's not an obviously invalid expression
        if expr_str.is_empty() {
            return Err(Error::ParseError(ParseError {
                message: "Cannot await empty expression".to_string(),
                location: self.current_token().location.clone(),
            }));
        }

        // Could add more sophisticated validation here based on type system
        // For now, we allow any non-empty expression
        Ok(())
    }

    /// Check if the current context allows async/await syntax
    pub fn is_in_async_context(&self) -> bool {
        // This would be implemented with proper context tracking
        // For now, we allow async/await everywhere for simplicity
        true
    }

    /// Parse function parameters for async functions
    fn parse_function_parameters(&mut self) -> Result<(), Error> {
        let mut parameters = Vec::new();

        if self.current_token_is(TokenType::RightParen) {
            return Ok(parameters);
        }

        loop {
            // Parse parameter name
            let name_token = self.expect_token(TokenType::Identifier)?;
            
            // Parse optional parameter type
            let param_type = if self.current_token_is(TokenType::Identifier) {
                Some(Box::new(Identifier::new(
                    self.advance().literal.clone(),
                    self.previous_token().literal.clone(),
                )) as Box<dyn Expression>)
            } else {
                None
            };

            parameters.push(Parameter::new(
                name_token.literal.clone(),
                name_token.literal,
                param_type,
            ));

            if self.current_token_is(TokenType::Comma) {
                self.advance(); // consume comma
            } else {
                break;
            }
        }

        Ok(parameters)
    }

    /// Helper method to get the previous token
    fn previous_token(&self) -> &Token {
        // This assumes we have access to the previous token
        // Implementation depends on Parser structure
        &self.current_token
    }

    /// Helper method to expect a specific token type
    fn expect_token(&mut self, expected: TokenType) -> Result<(), Error> {
        if self.current_token_is(expected.clone()) {
            Ok(self.advance())
        } else {
            Err(Error::ParseError(ParseError {
                message: format!("Expected {:?}, got {:?}", expected, self.current_token().token_type),
                location: self.current_token().location.clone(),
            }))
        }
    }

    /// Helper method to check if current token matches expected type
    fn current_token_is(&self, token_type: TokenType) -> bool {
        self.current_token().token_type == token_type
    }

    /// Helper method to get current token
    fn current_token(&self) -> &Token {
        &self.current_token
    }

    /// Helper method to advance to next token
    fn advance(&mut self) -> Token {
        let current = self.current_token.clone();
        self.next_token();
        current
    }
}

/// Async function parsing utilities
pub struct AsyncParser;

impl AsyncParser {
    /// Check if a token sequence represents an async function declaration
    pub fn is_async_function_declaration(tokens: &[Token]) -> bool {
        tokens.len() >= 3 &&
        tokens[0].token_type == TokenType::Slay &&
        tokens[1].token_type == TokenType::Async &&
        tokens[2].token_type == TokenType::Identifier
    }

    /// Check if a token represents the start of an await expression
    pub fn is_await_expression(token: &Token) -> bool {
        token.token_type == TokenType::Await
    }

    /// Extract async function name from token sequence
    pub fn extract_async_function_name(tokens: &[Token]) -> Option<String> {
        if Self::is_async_function_declaration(tokens) {
            Some(tokens[2].literal.clone())
        } else {
            None
        }
    }

    /// Validate async function signature
    pub fn validate_async_function_signature(
        name: &str,
        parameters: &[Parameter],
        return_type: &Option<Box<dyn Expression>>,
    ) -> Result<(), Error> {
        // Validate function name
        if name.is_empty() {
            return Err(Error::ParseError(ParseError {
                message: "Async function name cannot be empty".to_string(),
                location: crate::error::SourceLocation::default(),
            }));
        }

        // Validate parameters (basic check)
        for param in parameters {
            if param.to_string().is_empty() {
                return Err(Error::ParseError(ParseError {
                    message: "Parameter name cannot be empty".to_string(),
                    location: crate::error::SourceLocation::default(),
                }));
            }
        }

        // Additional validation could be added here
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_async_function_detection() {
        let mut lexer = Lexer::new("slay async test_func() {}".to_string());
        let mut tokens = Vec::new();
        
        loop {
            match lexer.next_token() {
                Ok(token) => {
                    let is_eof = token.token_type == TokenType::Eof;
                    tokens.push(token);
                    if is_eof { break; }
                }
                Err(_) => break,
            }
        }

        assert!(AsyncParser::is_async_function_declaration(&tokens));
        assert_eq!(AsyncParser::extract_async_function_name(&tokens), Some("test_func".to_string()));
    }

    #[test]
    fn test_await_expression_detection() {
        let token = Token {
            token_type: TokenType::Await,
            literal: "await".to_string(),
            location: crate::error::SourceLocation::default(),
        };

        assert!(AsyncParser::is_await_expression(&token));
    }

    #[test]
    fn test_async_function_signature_validation() {
        let result = AsyncParser::validate_async_function_signature(
            "test_func",
            &[],
            &None,
        );
        assert!(result.is_ok());

        let result = AsyncParser::validate_async_function_signature(
            "",
            &[],
            &None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_non_async_function_detection() {
        let mut lexer = Lexer::new("slay test_func() {}".to_string());
        let mut tokens = Vec::new();
        
        loop {
            match lexer.next_token() {
                Ok(token) => {
                    let is_eof = token.token_type == TokenType::Eof;
                    tokens.push(token);
                    if is_eof { break; }
                }
                Err(_) => break,
            }
        }

        assert!(!AsyncParser::is_async_function_declaration(&tokens));
    }
}
