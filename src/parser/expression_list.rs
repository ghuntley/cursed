use crate::ast::Expression;
use crate::error::Error;
use crate::lexer::Token;

use super::parser::Parser;
use super::precedence::Precedence;

impl<'a> Parser<'a> {
    /// Parse a list of expressions separated by commas, ended by the given end_token
    pub(super) fn parse_expression_list(
        &mut self,
        end_token: Token,
    ) -> Result<Vec<Box<dyn Expression>>, Error> {
        let mut expressions = Vec::new();

        // Handle empty list
        if self.peek_token_is(end_token.clone()) {
            self.next_token()?;
            return Ok(expressions);
        }

        self.next_token()?; // Skip past opening token
        expressions.push(self.parse_expression(Precedence::Lowest)?); // First expression

        // Parse remaining expressions in the list
        while self.peek_token_is(Token::Comma) {
            self.next_token()?; // Skip past comma
            self.next_token()?; // Move to next expression
            expressions.push(self.parse_expression(Precedence::Lowest)?);
        }

        // Expect end token
        if !self.peek_token_is(end_token.clone()) {
            return Err(self.error(&format!("Expected {:?} after expression list", end_token)));
        }
        self.next_token()?; // Skip past end token

        Ok(expressions)
    }
}
