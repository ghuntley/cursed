/// Expression parsing for the CURSED programming language
/// Uses Pratt parsing for proper operator precedence

use super::{Parser, Precedence};
use crate::ast::*;
use crate::error::Error;
use crate::lexer::TokenType;

impl Parser {
    /// Parse expression with precedence climbing
    pub fn parse_expression(&mut self) -> Result<Box<dyn Expression>, Error> {
        self.parse_expression_with_precedence(Precedence::Lowest)
    }
    
    /// Parse expression with given precedence level
    fn parse_expression_with_precedence(&mut self, precedence: Precedence) -> Result<Box<dyn Expression>, Error> {
        let mut left = self.parse_prefix_expression()?;
        
        while !self.peek_token_is(&TokenType::Semicolon) && 
              !self.peek_token_is(&TokenType::Newline) &&
              !self.peek_token_is(&TokenType::Eof) &&
              precedence < self.peek_precedence() {
            
            match &self.peek_token.token_type {
                TokenType::Plus | TokenType::Minus |
                TokenType::Multiply | TokenType::Divide | TokenType::Modulo |
                TokenType::Equal | TokenType::NotEqual |
                TokenType::LessThan | TokenType::LessThanEqual |
                TokenType::GreaterThan | TokenType::GreaterThanEqual |
                TokenType::LogicalAnd | TokenType::LogicalOr |
                TokenType::BitwiseAnd | TokenType::BitwiseOr | TokenType::BitwiseXor |
                TokenType::LeftShift | TokenType::RightShift => {
                    self.advance_token()?;
                    left = self.parse_infix_expression(left)?;
                }
                TokenType::LeftParen => {
                    self.advance_token()?;
                    left = self.parse_call_expression(left)?;
                }
                TokenType::LeftBracket => {
                    self.advance_token()?;
                    left = self.parse_index_expression(left)?;
                }
                TokenType::Dot => {
                    self.advance_token()?;
                    left = self.parse_dot_expression(left)?;
                }
                _ => return Ok(left),
            }
        }
        
        Ok(left)
    }
    
    /// Parse prefix expressions (unary operators, primary expressions)
    fn parse_prefix_expression(&mut self) -> Result<Box<dyn Expression>, Error> {
        match &self.current_token.token_type {
            TokenType::Identifier => self.parse_identifier(),
            TokenType::Integer => self.parse_integer_literal(),
            TokenType::Float => self.parse_float_literal(),
            TokenType::String => self.parse_string_literal(),
            TokenType::Boolean => self.parse_boolean_literal(),
            TokenType::NoCap => self.parse_nil_literal(),
            TokenType::Not | TokenType::Minus | TokenType::BitwiseNot => self.parse_unary_expression(),
            TokenType::LeftParen => self.parse_grouped_expression(),
            TokenType::LeftBracket => self.parse_array_literal(),
            TokenType::LeftBrace => self.parse_hash_literal(),
            TokenType::Slay => self.parse_function_literal(),
            _ => Err(Error::Parse(format!(
                "No prefix parse function for {:?} found at line {} column {}",
                self.current_token.token_type,
                self.current_token.location.line,
                self.current_token.location.column
            ))),
        }
    }
    
    /// Parse infix expressions (binary operators)
    fn parse_infix_expression(&mut self, left: Box<dyn Expression>) -> Result<Box<dyn Expression>, Error> {
        let operator = self.current_token.literal.clone();
        let precedence = self.current_precedence();
        
        self.advance_token()?;
        let right = self.parse_expression_with_precedence(precedence)?;
        
        Ok(Box::new(BinaryExpression::new(
            operator.clone(),
            left,
            operator,
            right,
        )))
    }
    
    /// Parse call expressions function(args...)
    fn parse_call_expression(&mut self, function: Box<dyn Expression>) -> Result<Box<dyn Expression>, Error> {
        let token = self.current_token.literal.clone();
        let arguments = self.parse_call_arguments()?;
        
        Ok(Box::new(CallExpression::new(
            token,
            function,
            arguments,
        )))
    }
    
    /// Parse index expressions array[index]
    fn parse_index_expression(&mut self, left: Box<dyn Expression>) -> Result<Box<dyn Expression>, Error> {
        let token = self.current_token.literal.clone();
        
        self.advance_token()?;
        let index = self.parse_expression()?;
        self.expect_token(TokenType::RightBracket)?;
        
        Ok(Box::new(IndexExpression::new(
            token,
            left,
            index,
        )))
    }
    
    /// Parse dot expressions obj.field
    fn parse_dot_expression(&mut self, left: Box<dyn Expression>) -> Result<Box<dyn Expression>, Error> {
        let token = self.current_token.literal.clone();
        
        self.advance_token()?;
        let property = self.expect_token(TokenType::Identifier)?.literal;
        
        Ok(Box::new(DotExpression::new(
            token,
            left,
            Box::new(Identifier::new(property.clone(), property)),
        )))
    }
    
    /// Parse identifier
    fn parse_identifier(&mut self) -> Result<Box<dyn Expression>, Error> {
        let token = self.current_token.clone();
        let value = token.literal.clone();
        self.advance_token()?;
        
        Ok(Box::new(Identifier::new(value.clone(), value)))
    }
    
    /// Parse integer literal
    fn parse_integer_literal(&mut self) -> Result<Box<dyn Expression>, Error> {
        let token = self.current_token.clone();
        
        let value = token.literal.parse::<i64>()
            .map_err(|_| Error::Parse(format!("Could not parse {} as integer", token.literal)))?;
        
        self.advance_token()?;
        
        Ok(Box::new(IntegerLiteral::new(token.literal, value)))
    }
    
    /// Parse float literal
    fn parse_float_literal(&mut self) -> Result<Box<dyn Expression>, Error> {
        let token = self.current_token.clone();
        
        let value = token.literal.parse::<f64>()
            .map_err(|_| Error::Parse(format!("Could not parse {} as float", token.literal)))?;
        
        self.advance_token()?;
        
        Ok(Box::new(FloatLiteral::new(token.literal, value)))
    }
    
    /// Parse string literal
    fn parse_string_literal(&mut self) -> Result<Box<dyn Expression>, Error> {
        let token = self.current_token.clone();
        let value = token.literal.clone();
        self.advance_token()?;
        
        Ok(Box::new(StringLiteral::new(token.literal, value)))
    }
    
    /// Parse boolean literal
    fn parse_boolean_literal(&mut self) -> Result<Box<dyn Expression>, Error> {
        let token = self.current_token.clone();
        let value = token.literal == "true";
        self.advance_token()?;
        
        Ok(Box::new(BooleanLiteral::new(token.literal, value)))
    }
    
    /// Parse nil literal (no_cap)
    fn parse_nil_literal(&mut self) -> Result<Box<dyn Expression>, Error> {
        let token = self.current_token.clone();
        self.advance_token()?;
        
        Ok(Box::new(NilLiteral::new()))
    }
    
    /// Parse unary expressions (!x, -x, ~x)
    fn parse_unary_expression(&mut self) -> Result<Box<dyn Expression>, Error> {
        let token = self.current_token.clone();
        let operator = token.literal.clone();
        
        self.advance_token()?;
        let operand = self.parse_expression_with_precedence(Precedence::Prefix)?;
        
        Ok(Box::new(UnaryExpression::new(
            token.literal,
            operator,
            operand,
        )))
    }
    
    /// Parse grouped expressions (expr)
    fn parse_grouped_expression(&mut self) -> Result<Box<dyn Expression>, Error> {
        self.advance_token()?;
        let expression = self.parse_expression()?;
        self.expect_token(TokenType::RightParen)?;
        
        Ok(Box::new(ParenthesizedExpression::new(
            "(".to_string(),
            expression,
        )))
    }
    
    /// Parse array literals [1, 2, 3]
    fn parse_array_literal(&mut self) -> Result<Box<dyn Expression>, Error> {
        let token = self.current_token.literal.clone();
        let elements = self.parse_expression_list(TokenType::RightBracket)?;
        
        Ok(Box::new(ArrayLiteral::new(token, elements)))
    }
    
    /// Parse hash literals {key: value, ...}
    fn parse_hash_literal(&mut self) -> Result<Box<dyn Expression>, Error> {
        let token = self.current_token.literal.clone();
        let mut pairs = Vec::new();
        
        while !self.peek_token_is(&TokenType::RightBrace) && !self.peek_token_is(&TokenType::Eof) {
            self.advance_token()?;
            let key = self.parse_expression()?;
            
            self.expect_token(TokenType::Colon)?;
            let value = self.parse_expression()?;
            
            pairs.push((key, value));
            
            if !self.peek_token_is(&TokenType::RightBrace) {
                self.expect_token(TokenType::Comma)?;
            }
        }
        
        self.expect_token(TokenType::RightBrace)?;
        
        Ok(Box::new(HashLiteral::new(token, pairs)))
    }
    
    /// Parse function literals slay(params) { body }
    fn parse_function_literal(&mut self) -> Result<Box<dyn Expression>, Error> {
        let token = self.current_token.literal.clone();
        
        self.expect_token(TokenType::LeftParen)?;
        let parameters = self.parse_function_parameters()?;
        self.expect_token(TokenType::RightParen)?;
        
        // Optional return type
        let return_type = if self.current_token_is(&TokenType::Identifier) {
            let ret_type = Box::new(Identifier::new(
                self.current_token.literal.clone(),
                self.current_token.literal.clone(),
            ));
            self.advance_token()?;
            Some(ret_type)
        } else {
            None
        };
        
        let body = if let Ok(block_stmt) = self.parse_block_statement() {
            if let Some(block) = block_stmt.as_any().downcast_ref::<BlockStatement>() {
                block.clone()
            } else {
                return Err(Error::Parse("Expected block statement".to_string()));
            }
        } else {
            return Err(Error::Parse("Expected function body".to_string()));
        };
        
        Ok(Box::new(FunctionLiteral::new(
            token,
            parameters,
            body,
            return_type.map(|id| id as Box<dyn Expression>),
        )))
    }
    
    /// Parse function parameters for function literals
    fn parse_function_parameters(&mut self) -> Result<Vec<Parameter>, Error> {
        let mut parameters = Vec::new();
        
        if self.peek_token_is(&TokenType::RightParen) {
            self.advance_token()?;
            return Ok(parameters);
        }
        
        self.advance_token()?;
        
        let name = self.current_token.literal.clone();
        let param_type = if self.peek_token_is(&TokenType::Identifier) {
            self.advance_token()?;
            self.current_token.literal.clone()
        } else {
            "".to_string() // Inferred type
        };
        
        parameters.push(Parameter::new(name, param_type));
        
        while self.peek_token_is(&TokenType::Comma) {
            self.advance_token()?;
            self.advance_token()?;
            
            let name = self.current_token.literal.clone();
            let param_type = if self.peek_token_is(&TokenType::Identifier) {
                self.advance_token()?;
                self.current_token.literal.clone()
            } else {
                "".to_string()
            };
            
            parameters.push(Parameter::new(name, param_type));
        }
        
        self.advance_token()?;
        Ok(parameters)
    }
    
    /// Parse call arguments
    fn parse_call_arguments(&mut self) -> Result<Vec<Box<dyn Expression>>, Error> {
        self.parse_expression_list(TokenType::RightParen)
    }
    
    /// Parse expression list separated by commas
    fn parse_expression_list(&mut self, end_token: TokenType) -> Result<Vec<Box<dyn Expression>>, Error> {
        let mut args = Vec::new();
        
        if self.peek_token_is(&end_token) {
            self.advance_token()?;
            return Ok(args);
        }
        
        self.advance_token()?;
        args.push(self.parse_expression()?);
        
        while self.peek_token_is(&TokenType::Comma) {
            self.advance_token()?;
            self.advance_token()?;
            args.push(self.parse_expression()?);
        }
        
        self.expect_token(end_token)?;
        Ok(args)
    }
}
