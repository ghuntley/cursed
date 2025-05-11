use crate::ast::expressions::{TypeAssertion, TypeAssertionQuestion};
use crate::ast::traits::Expression;
use crate::error::{Error, ErrorReporter, SourceLocation};
use crate::lexer::token::Token;
use crate::parser::Parser;

impl<'a> Parser<'a> {
    /// Parse a type assertion expression of the form `expr.(Type)` or `expr.(Type)?`
    /// Returns a TypeAssertion or TypeAssertionQuestion AST node
    pub fn parse_type_assertion(
        &mut self,
        left: Box<dyn Expression>
    ) -> Result<Box<dyn Expression>, Error> {
        // Store the current token for error reporting
        let token = self.current_token.clone();
        
        // Expect and consume the opening parenthesis
        self.expect_peek(Token::LParen)?;
        
        // Advance to the token after the opening parenthesis
        let _ = self.next_token();
        
        // Expect and parse the type name
        match &self.current_token {
            Token::Identifier(name) => {
                let type_name = name.clone();
                
                // Expect and consume the closing parenthesis
                self.expect_peek(Token::RParen)?;
                
                // Check if the next token is a question mark for error propagation
                let peek_is_question = match &self.peek_token {
                    Token::Question => true,
                    _ => false,
                };
                
                if peek_is_question {
                    // Consume the question mark token
                    self.next_token()?;
                    
                    // Create and return the TypeAssertionQuestion node with error propagation
                    Ok(Box::new(TypeAssertionQuestion {
                        token: token.token_literal(),
                        expression: left,
                        type_name,
                    }))
                } else {
                    // Create and return the regular TypeAssertion node
                    Ok(Box::new(TypeAssertion {
                        token: token.token_literal(),
                        expression: left,
                        type_name,
                    }))
                }
            },
            _ => {
                Err(Error::Parser { location: SourceLocation { line: 0, column: 0, file: Some("".to_string()), source_line: "".to_string() }, message: format!("Expected type name, got '{:?}'", self.current_token) })
            }
        }
    }
}