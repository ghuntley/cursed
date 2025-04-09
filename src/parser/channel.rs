use crate::ast::{self, Expression};
use crate::ast::expressions::channel::ChannelExpression;
use crate::error::Error;
use crate::lexer::Token;
use super::parser::Parser;
use super::precedence::Precedence;

impl<'a> Parser<'a> {
    /// Parse a channel expression with optional capacity
    pub(super) fn parse_channel_expression(&mut self) -> Result<Box<dyn Expression>, Error> {
        let token = self.current_token.clone();
        self.next_token()?; // Advance past 'dm'
        
        // Parse the element type
        let element_type = self.parse_expression(Precedence::Lowest)?;
        
        // Check for optional capacity
        let capacity = if self.peek_token_is(Token::LBracket) {
            self.next_token()?; // Advance to '['
            self.next_token()?; // Advance past '['
            
            // Parse capacity expression
            let cap_expr = self.parse_expression(Precedence::Lowest)?;
            
            // Expect closing bracket
            if !self.peek_token_is(Token::RBracket) {
                return Err(self.error(&format!("Expected ']' after channel capacity, got {:?}", self.peek_token)));
            }
            self.next_token()?; // Advance to ']'
            self.next_token()?; // Advance past ']'
            
            Some(cap_expr)
        } else {
            None
        };
        
        // Create a channel expression
        let channel_expr = ChannelExpression {
            token: token.token_literal(),
            element_type,
            capacity,
        };
        
        Ok(Box::new(channel_expr))
    }
}