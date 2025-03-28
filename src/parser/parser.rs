use crate::ast::{self, Program, Statement, Expression};
use crate::error::Error;
use crate::lexer::{Lexer, Token};

/// Precedence levels for expression parsing
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Precedence {
    Lowest,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // myFunction(X)
    Index,       // array[index]
}

/// Parser for the CURSED language
pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: Token,
    peek_token: Token,
    errors: Vec<Error>,
}

impl<'a> Parser<'a> {
    /// Create a new parser from a lexer
    pub fn new(lexer: &'a mut Lexer<'a>) -> Result<Self, Error> {
        let mut parser = Parser {
            lexer,
            current_token: Token::Eof,
            peek_token: Token::Eof,
            errors: Vec::new(),
        };
        
        // Read two tokens to initialize current_token and peek_token
        parser.next_token()?;
        parser.next_token()?;
        
        Ok(parser)
    }
    
    /// Parse a complete program
    pub fn parse_program(&mut self) -> Result<Program, Error> {
        let mut program = Program {
            statements: Vec::new(),
        };
        
        // While not EOF, parse statements
        while self.current_token != Token::Eof {
            match self.parse_statement() {
                Ok(stmt) => program.statements.push(stmt),
                Err(e) => self.errors.push(e),
            }
            
            // Advance to the next token
            match self.next_token() {
                Ok(_) => {},
                Err(e) => self.errors.push(e),
            }
        }
        
        if !self.errors.is_empty() {
            // If we had parsing errors, return the first one
            return Err(self.errors.remove(0));
        }
        
        Ok(program)
    }
    
    /// Parse a statement
    fn parse_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        match &self.current_token {
            Token::Vibe => self.parse_package_statement(),
            Token::Yeet => self.parse_import_statement(),
            Token::Sus => self.parse_let_statement(),
            Token::Yolo => self.parse_return_statement(),
            Token::Lowkey => self.parse_if_statement(),
            Token::Periodt => self.parse_while_statement(),
            Token::Bestie => self.parse_for_statement(),
            Token::VibeCheck => self.parse_switch_statement(),
            Token::BeLike => self.parse_type_statement(),
            _ => self.parse_expression_statement(),
        }
    }
    
    /// Parse a package statement
    pub fn parse_package_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        // For now, just return a not implemented error
        Err(Error::from_str("Package statement parsing not implemented yet"))
    }
    
    /// Parse an import statement
    pub fn parse_import_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        // For now, just return a not implemented error
        Err(Error::from_str("Import statement parsing not implemented yet"))
    }
    
    /// Parse a let statement
    pub fn parse_let_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        // For now, just return a not implemented error
        Err(Error::from_str("Let statement parsing not implemented yet"))
    }
    
    /// Parse a return statement
    pub fn parse_return_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        // For now, just return a not implemented error
        Err(Error::from_str("Return statement parsing not implemented yet"))
    }
    
    /// Parse an if statement
    pub fn parse_if_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        // For now, just return a not implemented error
        Err(Error::from_str("If statement parsing not implemented yet"))
    }
    
    /// Parse a for statement
    pub fn parse_for_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        // For now, just return a not implemented error
        Err(Error::from_str("For statement parsing not implemented yet"))
    }
    
    /// Parse a switch statement
    pub fn parse_switch_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        // For now, just return a not implemented error
        Err(Error::from_str("Switch statement parsing not implemented yet"))
    }
    
    /// Parse a type statement
    pub fn parse_type_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        // For now, just return a not implemented error
        Err(Error::from_str("Type statement parsing not implemented yet"))
    }
    
    /// Parse an expression statement
    fn parse_expression_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.current_token.token_literal();
        
        // Parse the expression
        let expr = self.parse_expression(Precedence::Lowest)?;
        
        // Optionally consume a semicolon
        if self.peek_token == Token::Semicolon {
            self.next_token()?;
        }
        
        Ok(Box::new(ast::ExpressionStatement {
            token,
            expression: Some(expr),
        }))
    }
    
    /// Parse a while statement (periodt)
    pub fn parse_while_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        // Store the 'periodt' token
        let token = self.current_token.token_literal();
        
        // Move to the next token after 'periodt'
        self.next_token()?;
        
        // Parse the condition
        let condition = self.parse_expression(Precedence::Lowest)?;
        
        // Check if the current token is already a '{'
        if self.current_token == Token::LBrace {
            // Parse the body
            let body = self.parse_block_statement()?;
            
            Ok(Box::new(ast::WhileStatement {
                token,
                condition,
                body,
            }))
        } else {
            // Otherwise, expect the next token to be a '{'
            if !self.expect_peek(&Token::LBrace) {
                return Err(Error::from_str(
                    &format!("Expected '{{' after condition in periodt statement, got {:?}", self.peek_token)
                ));
            }
            
            // Parse the body
            let body = self.parse_block_statement()?;
            
            Ok(Box::new(ast::WhileStatement {
                token,
                condition,
                body,
            }))
        }
    }
    
    /// Parse a block statement
    pub fn parse_block_statement(&mut self) -> Result<ast::BlockStatement, Error> {
        let token = self.current_token.token_literal();
        let mut statements = Vec::new();
        
        // Move past the '{'
        self.next_token()?;
        
        // Keep parsing statements until we hit a closing brace or EOF
        while self.current_token != Token::RBrace && self.current_token != Token::Eof {
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => self.errors.push(e),
            }
            
            // Advance to the next token
            match self.next_token() {
                Ok(_) => {},
                Err(e) => self.errors.push(e),
            }
        }
        
        // Check if we exited because of EOF (which would be an error)
        if self.current_token != Token::RBrace {
            return Err(Error::from_str(
                "Expected '}' to close block statement, got EOF"
            ));
        }
        
        Ok(ast::BlockStatement {
            token,
            statements,
        })
    }
    
    /// Parse an expression with the given precedence
    fn parse_expression(&mut self, precedence: Precedence) -> Result<Box<dyn Expression>, Error> {
        // First try to find a prefix parsing function
        let mut left_expr: Box<dyn Expression> = match &self.current_token {
            Token::Identifier(name) => {
                let identifier = ast::Identifier {
                    token: self.current_token.token_literal(),
                    value: name.clone(),
                };
                
                let curr_token = self.current_token.clone();
                self.next_token()?;
                
                // Check for assignment expressions
                if self.current_token == Token::Assign {
                    // Move past the '='
                    self.next_token()?;
                    
                    // Parse the right side of the assignment
                    let value = self.parse_expression(Precedence::Lowest)?;
                    
                    return Ok(Box::new(ast::AssignmentExpression {
                        token: "=".to_string(),
                        name: identifier,
                        value,
                    }));
                }
                
                Box::new(identifier)
            },
            Token::Int(value) => {
                let literal = ast::IntegerLiteral {
                    token: self.current_token.token_literal(),
                    value: *value,
                };
                
                self.next_token()?;
                Box::new(literal)
            },
            Token::String(value) => {
                let literal = ast::StringLiteral {
                    token: self.current_token.token_literal(),
                    value: value.clone(),
                };
                
                self.next_token()?;
                Box::new(literal)
            },
            Token::Based => {
                let literal = ast::BooleanLiteral {
                    token: self.current_token.token_literal(),
                    value: true,
                };
                
                self.next_token()?;
                Box::new(literal)
            },
            Token::Cap => {
                let literal = ast::BooleanLiteral {
                    token: self.current_token.token_literal(),
                    value: false,
                };
                
                self.next_token()?;
                Box::new(literal)
            },
            Token::LParen => {
                // Group expression
                self.next_token()?;
                let expr = self.parse_expression(Precedence::Lowest)?;
                
                // Check for the closing parenthesis
                if self.current_token != Token::RParen {
                    return Err(Error::from_str(&format!("Expected ')' after grouped expression, got {:?}", self.current_token)));
                }
                
                // Move past the right parenthesis
                self.next_token()?;
                
                expr
            },
            Token::Bang | Token::Minus => {
                let operator = self.current_token.token_literal();
                let token = self.current_token.clone();
                
                // Move past the prefix operator
                self.next_token()?;
                
                // Parse the right expression
                let right = self.parse_expression(Precedence::Prefix)?;
                
                Box::new(ast::PrefixExpression {
                    token,
                    operator,
                    right,
                })
            },
            _ => {
                return Err(Error::from_str(
                    &format!("No prefix parse function for {:?}", self.current_token)
                ));
            }
        };
        
        // Now check for infix expressions and keep building as long as the precedence allows
        while self.current_token != Token::Semicolon && 
              self.current_token != Token::RBrace && 
              self.current_token != Token::RParen &&
              precedence < self.get_precedence(&self.current_token) {
            
            match &self.current_token {
                Token::Plus | Token::Minus | Token::Asterisk | Token::Slash |
                Token::Eq | Token::NotEq | Token::Lt | Token::Gt | Token::LtEq | Token::GtEq => {
                    left_expr = self.parse_infix_expression(left_expr)?;
                },
                Token::LParen => {
                    // Parse function call
                    left_expr = self.parse_call_expression(left_expr)?;
                },
                Token::LBracket => {
                    // Parse index expression
                    left_expr = self.parse_index_expression(left_expr)?;
                },
                _ => {
                    // No infix parser for this token, return the expression as is
                    return Ok(left_expr);
                }
            }
        }
        
        Ok(left_expr)
    }
    
    /// Parse an infix expression
    fn parse_infix_expression(&mut self, left: Box<dyn Expression>) -> Result<Box<dyn Expression>, Error> {
        let operator = self.current_token.token_literal();
        let token = self.current_token.clone();
        let precedence = self.get_precedence(&self.current_token);
        
        // Move past the operator
        self.next_token()?;
        
        // Parse the right expression
        let right = self.parse_expression(precedence)?;
        
        Ok(Box::new(ast::InfixExpression {
            token,
            left,
            operator,
            right,
        }))
    }
    
    /// Parse a call expression
    fn parse_call_expression(&mut self, function: Box<dyn Expression>) -> Result<Box<dyn Expression>, Error> {
        let token = self.current_token.clone();
        
        // Parse the arguments
        let arguments = self.parse_expression_list(&Token::RParen)?;
        
        Ok(Box::new(ast::CallExpression {
            token,
            function,
            arguments,
        }))
    }
    
    /// Parse a list of expressions
    fn parse_expression_list(&mut self, end: &Token) -> Result<Vec<Box<dyn Expression>>, Error> {
        let mut args = Vec::new();
        
        // Move past the opening token
        self.next_token()?;
        
        // Empty list case
        if &self.current_token == end {
            self.next_token()?;
            return Ok(args);
        }
        
        // Parse the first expression
        args.push(self.parse_expression(Precedence::Lowest)?);
        
        // Parse the remaining expressions
        while self.current_token == Token::Comma {
            self.next_token()?;
            args.push(self.parse_expression(Precedence::Lowest)?);
        }
        
        // Check for the end token
        if &self.current_token != end {
            return Err(Error::from_str(
                &format!("Expected {:?} after expression list, got {:?}", end, self.current_token)
            ));
        }
        
        // Move past the end token
        self.next_token()?;
        
        Ok(args)
    }
    
    /// Parse an index expression (array[index])
    fn parse_index_expression(&mut self, array: Box<dyn Expression>) -> Result<Box<dyn Expression>, Error> {
        let token = self.current_token.clone();
        
        // Move past the '['
        self.next_token()?;
        
        // Parse the index expression
        let index = self.parse_expression(Precedence::Lowest)?;
        
        // Expect the closing ']'
        if !self.expect_peek(&Token::RBracket) {
            return Err(Error::from_str(
                &format!("Expected ']' after index, got {:?}", self.peek_token)
            ));
        }
        
        Ok(Box::new(ast::IndexExpression {
            token,
            left: array,
            index,
        }))
    }
    
    /// Get the precedence of a token
    fn get_precedence(&self, token: &Token) -> Precedence {
        match token {
            Token::Eq | Token::NotEq => Precedence::Equals,
            Token::Lt | Token::Gt | Token::LtEq | Token::GtEq => Precedence::LessGreater,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Asterisk | Token::Slash => Precedence::Product,
            Token::LParen => Precedence::Call,
            Token::LBracket => Precedence::Index,
            _ => Precedence::Lowest,
        }
    }
    
    /// Advance to the next token
    fn next_token(&mut self) -> Result<(), Error> {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token()?;
        Ok(())
    }
    
    /// Get the parser errors
    pub fn errors(&self) -> &[Error] {
        &self.errors
    }
    
    /// Helper to check if the next token is as expected
    fn expect_peek(&mut self, token: &Token) -> bool {
        if &self.peek_token == token {
            match self.next_token() {
                Ok(_) => true,
                Err(_) => false,
            }
        } else {
            self.peek_error(token);
            false
        }
    }
    
    fn peek_error(&mut self, token: &Token) {
        self.errors.push(Error::from_str(
            &format!("Expected next token to be {:?}, got {:?}", token, self.peek_token)
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    /// Test the parser with the given input string
    fn test_parser_with_input(input: &str) -> Result<Program, Error> {
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer)?;
        parser.parse_program()
    }
    
    #[test]
    fn test_parser_initialization() -> Result<(), Error> {
        let mut lexer = Lexer::new("5;");
        let parser = Parser::new(&mut lexer)?;
        
        // Check that the parser is properly initialized with tokens
        assert!(parser.current_token != Token::Eof);
        assert_eq!(parser.errors().len(), 0);
        
        Ok(())
    }
    
    #[test]
    fn test_parse_empty_program() -> Result<(), Error> {
        let program = test_parser_with_input("")?;
        
        // Empty program should have no statements
        assert_eq!(program.statements.len(), 0);
        
        Ok(())
    }
    
    #[test]
    fn test_token_advancement() -> Result<(), Error> {
        let mut lexer = Lexer::new("5; 10;");
        let mut parser = Parser::new(&mut lexer)?;
        
        // Check initial tokens
        assert!(matches!(parser.current_token, Token::Int(5)));
        assert!(matches!(parser.peek_token, Token::Semicolon));
        
        // Advance and check
        parser.next_token()?;
        assert!(matches!(parser.current_token, Token::Semicolon));
        assert!(matches!(parser.peek_token, Token::Int(10)));
        
        // Advance again
        parser.next_token()?;
        assert!(matches!(parser.current_token, Token::Int(10)));
        assert!(matches!(parser.peek_token, Token::Semicolon));
        
        Ok(())
    }
    
    #[test]
    fn test_parse_periodt_statements() -> Result<(), Error> {
        // Test with different while statement (periodt) formats
        let inputs = vec![
            "periodt x < 10 { x = x + 1; }",
            "periodt (x < 10) { x = x + 1; }",
            "periodt true { x = x + 1; }",
            "periodt 1 < 2 { print(\"hello\"); }"
        ];
        
        for input in inputs {
            let program = test_parser_with_input(input)?;
            
            // Verify we have exactly one statement
            assert_eq!(program.statements.len(), 1, "Failed to parse: {}", input);
            
            // Verify it's a while statement (periodt)
            let while_stmt = program.statements[0].as_any().downcast_ref::<ast::WhileStatement>();
            assert!(while_stmt.is_some(), "Not a while statement: {}", input);
            
            // Verify it has a condition and body
            let while_stmt = while_stmt.unwrap();
            assert!(while_stmt.condition.token_literal().len() > 0, "Missing condition in: {}", input);
            assert!(while_stmt.body.statements.len() > 0, "Empty body in: {}", input);
        }
        
        Ok(())
    }
    
    #[test]
    fn test_parse_parenthesized_periodt() -> Result<(), Error> {
        let input = "periodt (x < 10) { x = x + 1; }";
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer)?;
        
        // Start parsing
        let token = parser.current_token.clone();
        assert_eq!(token, Token::Periodt);
        
        // Move to next token after periodt
        parser.next_token()?;
        assert_eq!(parser.current_token, Token::LParen);
        
        // Parse the condition
        let condition = parser.parse_expression(Precedence::Lowest)?;
        
        // After parsing the condition, the current token should be LBrace
        assert_eq!(parser.current_token, Token::LBrace, "Current token after parsing condition should be LBrace but is {:?}", parser.current_token);
        
        Ok(())
    }
} 