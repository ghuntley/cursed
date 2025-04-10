use crate::ast::{Expression, ReferenceExpression};
use crate::error::Error;
use crate::lexer::Token;

use super::parser::Parser;
use super::precedence::Precedence;

impl<'a> Parser<'a> {
    /// Parse a reference expression (&expr)
    pub(super) fn parse_reference_expression(&mut self) -> Result<Box<dyn Expression>, Error> {
        let token = self.current_token.clone();
        println!("DEBUG: Parsing reference expression, current token: {:?}", &self.current_token);
        self.next_token()?; // Advance past '&'
        println!("DEBUG: After & token, current token: {:?}", &self.current_token);
        
        // Parse the target expression
        let target = self.parse_expression(Precedence::Prefix)?;
        println!("DEBUG: Parsed reference target expression: {}", target.string());
        
        // Create the reference expression
        let reference_expr = ReferenceExpression {
            token,
            value: target,
        };
        
        Ok(Box::new(reference_expr))
    }
}