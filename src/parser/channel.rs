use super::parser::Parser;
use super::precedence::Precedence;
use crate::ast::expressions::channel::ChannelExpression;
use crate::ast::{self, Expression};
use crate::error::Error;
use crate::lexer::Token;

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
                return Err(self.error(&format!(
                    "Expected ']' after channel capacity, got {:?}",
                    self.peek_token
                )));
            }
            self.next_token()?; // Advance to ']'
            self.next_token()?; // Advance past ']'

            Some(cap_expr)
        } else {
            None
        };

        // Use the helper function to create a channel expression
        use crate::parser::channel_helpers::create_channel_expression;
        
        let channel_expr = create_channel_expression(
            token.clone(), // Pass the token directly
            element_type, // Pass the element type directly
            capacity,
        );

        Ok(Box::new(channel_expr))
    }
}
