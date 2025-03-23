use crate::ast::{
    BooleanLiteral, Expression, FloatLiteral, Identifier, ImportStatement, IntegerLiteral,
    Node, PackageStatement, Program, Statement, StringLiteral, ExpressionStatement
};
use crate::error::{Error, ErrorReporter, SourceLocation};
use crate::lexer::{Lexer, Token};
use std::mem;

/// Precedence levels for parsing expressions
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
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
    lexer: Lexer<'a>,
    current_token: Option<Token>,
    peek_token: Option<Token>,
    errors: Vec<Error>,
}

impl<'a> Parser<'a> {
    /// Create a new parser
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut p = Parser {
            lexer,
            errors: Vec::new(),
            current_token: None,
            peek_token: None,
        };
        
        // Read the first two tokens to initialize current_token and peek_token
        p.next_token();
        p.next_token();
        
        p
    }
    
    /// Get the errors that occurred during parsing
    pub fn errors(&self) -> &[Error] {
        &self.errors
    }
    
    /// Advance to the next token
    fn next_token(&mut self) {
        self.current_token = self.peek_token.take();
        match self.lexer.next_token() {
            Ok(token) => self.peek_token = Some(token),
            Err(err) => {
                self.errors.push(err);
                self.peek_token = Some(Token::Illegal("error".to_string()));
            }
        }
    }
    
    /// Check if the current token is of the expected type
    fn current_token_is(&self, token_type: &Token) -> bool {
        match (&self.current_token, token_type) {
            (Some(Token::Eof), Token::Eof) => true,
            (Some(current), expected) => std::mem::discriminant(current) == std::mem::discriminant(expected),
            _ => false,
        }
    }
    
    /// Check if the peek token is of the expected type
    fn peek_token_is(&self, token_type: &Token) -> bool {
        match (&self.peek_token, token_type) {
            (Some(Token::Eof), Token::Eof) => true,
            (Some(peek), expected) => std::mem::discriminant(peek) == std::mem::discriminant(expected),
            _ => false,
        }
    }
    
    /// Expect the peek token to be of a certain type, and advance if it is
    fn expect_peek(&mut self, token_type: Token) -> Result<(), Error> {
        if self.peek_token_is(&token_type) {
            self.next_token();
            Ok(())
        } else {
            let location = SourceLocation::new(0, 0); // TODO: get actual location
            let message = format!(
                "Expected next token to be {:?}, got {:?} instead",
                token_type, self.peek_token
            );
            Err(ErrorReporter::parser_error(location, &message))
        }
    }
    
    /// Parse a CURSED program
    pub fn parse_program(&mut self) -> Result<Program, Error> {
        let mut program = Program::default();
        
        while !self.current_token_is(&Token::Eof) {
            match self.parse_statement() {
                Ok(stmt) => program.statements.push(stmt),
                Err(err) => {
                    self.errors.push(err);
                    self.next_token(); // Skip the problematic token and continue
                }
            }
            
            self.next_token();
        }
        
        if !self.errors.is_empty() {
            // Return the first error for now
            return Err(self.errors[0].clone());
        }
        
        Ok(program)
    }
    
    /// Parse a statement
    fn parse_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        match &self.current_token {
            Some(Token::Vibe) => self.parse_package_statement(),
            Some(Token::Yeet) => self.parse_import_statement(),
            Some(_) => self.parse_expression_statement(),
            None => {
                let location = SourceLocation::new(0, 0); // TODO: get actual location
                Err(ErrorReporter::parser_error(location, "Unexpected end of input"))
            }
        }
    }
    
    /// Parse a package statement (vibe)
    fn parse_package_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token_literal = "vibe".to_string();
        
        // Expect an identifier after 'vibe'
        self.next_token();
        
        let name = match &self.current_token {
            Some(Token::Identifier(ident)) => Identifier {
                token: "identifier".to_string(),
                value: ident.clone(),
            },
            _ => {
                let location = SourceLocation::new(0, 0); // TODO: get actual location
                return Err(ErrorReporter::parser_error(
                    location,
                    "Expected identifier after 'vibe'",
                ));
            }
        };
        
        // Expect a semicolon after the package name
        self.expect_peek(Token::Semicolon)?;
        
        Ok(Box::new(PackageStatement {
            token: token_literal,
            name,
        }))
    }
    
    /// Parse an import statement (yeet)
    fn parse_import_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token_literal = "yeet".to_string();
        let mut alias = None;
        
        // Check if there's an import alias
        self.next_token();
        if let Some(Token::Identifier(ident)) = &self.current_token {
            alias = Some(Identifier {
                token: "identifier".to_string(),
                value: ident.clone(),
            });
            self.next_token();
        }
        
        // Expect a string literal for import path
        let path = match &self.current_token {
            Some(Token::String(s)) => StringLiteral {
                token: "string".to_string(),
                value: s.clone(),
            },
            _ => {
                let location = SourceLocation::new(0, 0); // TODO: get actual location
                return Err(ErrorReporter::parser_error(
                    location,
                    "Expected string literal for import path",
                ));
            }
        };
        
        // Expect a semicolon after the import path
        self.expect_peek(Token::Semicolon)?;
        
        Ok(Box::new(ImportStatement {
            token: token_literal,
            path,
            alias,
        }))
    }
    
    /// Parse an expression statement
    fn parse_expression_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = match &self.current_token {
            Some(token) => format!("{:?}", token),
            None => "".to_string(),
        };
        
        let expr = self.parse_expression(Precedence::Lowest)?;
        
        // Optional semicolon after expressions
        if self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }
        
        Ok(Box::new(ExpressionStatement {
            token,
            expression: Some(expr),
        }))
    }
    
    /// Parse an expression
    fn parse_expression(&mut self, precedence: Precedence) -> Result<Box<dyn Expression>, Error> {
        // Parse prefix expressions
        let mut left_expr = match &self.current_token {
            Some(Token::Identifier(ident)) => {
                let identifier = Identifier {
                    token: "identifier".to_string(),
                    value: ident.clone(),
                };
                Box::new(identifier) as Box<dyn Expression>
            },
            Some(Token::Int(i)) => {
                let int_literal = IntegerLiteral {
                    token: "int".to_string(),
                    value: *i,
                };
                Box::new(int_literal) as Box<dyn Expression>
            },
            Some(Token::Float(f)) => {
                let float_literal = FloatLiteral {
                    token: "float".to_string(),
                    value: *f,
                };
                Box::new(float_literal) as Box<dyn Expression>
            },
            Some(Token::String(s)) => {
                let string_literal = StringLiteral {
                    token: "string".to_string(),
                    value: s.clone(),
                };
                Box::new(string_literal) as Box<dyn Expression>
            },
            Some(Token::Based) => {
                let bool_literal = BooleanLiteral {
                    token: "based".to_string(),
                    value: true,
                };
                Box::new(bool_literal) as Box<dyn Expression>
            },
            Some(Token::Cap) => {
                let bool_literal = BooleanLiteral {
                    token: "cap".to_string(),
                    value: false,
                };
                Box::new(bool_literal) as Box<dyn Expression>
            },
            Some(Token::Bang) | Some(Token::Minus) => {
                self.parse_prefix_expression()?
            },
            Some(Token::LParen) => {
                self.parse_grouped_expression()?
            },
            _ => {
                let location = SourceLocation::new(0, 0); // TODO: get actual location
                let message = format!("No prefix parse function for token: {:?}", self.current_token);
                return Err(ErrorReporter::parser_error(location, &message));
            }
        };
        
        // Parse infix expressions while the next token has higher precedence
        while !self.peek_token_is(&Token::Semicolon) && precedence < self.peek_precedence() {
            match &self.peek_token {
                Some(Token::Plus) | Some(Token::Minus) | Some(Token::Asterisk) | Some(Token::Slash) |
                Some(Token::Eq) | Some(Token::NotEq) | Some(Token::Lt) | Some(Token::Gt) | 
                Some(Token::LtEq) | Some(Token::GtEq) => {
                    self.next_token();
                    left_expr = self.parse_infix_expression(left_expr)?;
                },
                Some(Token::LParen) => {
                    self.next_token();
                    left_expr = self.parse_call_expression(left_expr)?;
                },
                Some(Token::LBracket) => {
                    self.next_token();
                    left_expr = self.parse_index_expression(left_expr)?;
                },
                _ => break,
            }
        }
        
        Ok(left_expr)
    }
    
    /// Parse a prefix expression (e.g., !x, -5)
    fn parse_prefix_expression(&mut self) -> Result<Box<dyn Expression>, Error> {
        // Store the operator token
        let operator = match &self.current_token {
            Some(Token::Bang) => "!".to_string(),
            Some(Token::Minus) => "-".to_string(),
            _ => {
                let location = SourceLocation::new(0, 0); // TODO: get actual location
                return Err(ErrorReporter::parser_error(
                    location,
                    "Expected prefix operator (! or -)",
                ));
            }
        };
        
        // Move to the expression
        self.next_token();
        
        // Parse the right expression with prefix precedence
        let right = self.parse_expression(Precedence::Prefix)?;
        
        // Create a struct to represent the prefix expression
        struct PrefixExpression {
            token: String,
            operator: String,
            right: Box<dyn Expression>,
        }
        
        // Implement necessary traits for PrefixExpression
        impl Node for PrefixExpression {
            fn token_literal(&self) -> String {
                self.token.clone()
            }
            
            fn string(&self) -> String {
                format!("({}{})", self.operator, self.right.string())
            }
        }
        
        impl Expression for PrefixExpression {
            fn expression_node(&self) {}
            
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        
        // Return the prefix expression
        Ok(Box::new(PrefixExpression {
            token: operator.clone(),
            operator,
            right,
        }))
    }
    
    /// Parse a grouped expression (e.g., (x + y))
    fn parse_grouped_expression(&mut self) -> Result<Box<dyn Expression>, Error> {
        self.next_token(); // Skip opening parenthesis
        
        let expr = self.parse_expression(Precedence::Lowest)?;
        
        self.expect_peek(Token::RParen)?;
        
        Ok(expr)
    }
    
    /// Parse an infix expression (e.g., x + y)
    fn parse_infix_expression(&mut self, left: Box<dyn Expression>) -> Result<Box<dyn Expression>, Error> {
        // Store the operator token
        let operator = match &self.current_token {
            Some(Token::Plus) => "+".to_string(),
            Some(Token::Minus) => "-".to_string(),
            Some(Token::Asterisk) => "*".to_string(),
            Some(Token::Slash) => "/".to_string(),
            Some(Token::Eq) => "==".to_string(),
            Some(Token::NotEq) => "!=".to_string(),
            Some(Token::Lt) => "<".to_string(),
            Some(Token::Gt) => ">".to_string(),
            Some(Token::LtEq) => "<=".to_string(),
            Some(Token::GtEq) => ">=".to_string(),
            _ => {
                let location = SourceLocation::new(0, 0); // TODO: get actual location
                return Err(ErrorReporter::parser_error(
                    location,
                    "Expected infix operator (+, -, *, /, ==, !=, <, >, <=, >=)",
                ));
            }
        };
        
        // Get the precedence of the current operator
        let precedence = self.current_precedence();
        
        // Move to the right expression
        self.next_token();
        
        // Parse the right expression with the current precedence
        let right = self.parse_expression(precedence)?;
        
        // Create a struct to represent the infix expression
        struct InfixExpression {
            token: String,
            left: Box<dyn Expression>,
            operator: String,
            right: Box<dyn Expression>,
        }
        
        // Implement necessary traits for InfixExpression
        impl Node for InfixExpression {
            fn token_literal(&self) -> String {
                self.token.clone()
            }
            
            fn string(&self) -> String {
                format!("({} {} {})", self.left.string(), self.operator, self.right.string())
            }
        }
        
        impl Expression for InfixExpression {
            fn expression_node(&self) {}
            
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        
        // Return the infix expression
        Ok(Box::new(InfixExpression {
            token: operator.clone(),
            left,
            operator,
            right,
        }))
    }
    
    /// Parse a call expression (e.g., add(1, 2))
    fn parse_call_expression(&mut self, function: Box<dyn Expression>) -> Result<Box<dyn Expression>, Error> {
        // Create a struct to represent the call expression
        struct CallExpression {
            token: String,
            function: Box<dyn Expression>,
            arguments: Vec<Box<dyn Expression>>,
        }
        
        // Implement necessary traits for CallExpression
        impl Node for CallExpression {
            fn token_literal(&self) -> String {
                self.token.clone()
            }
            
            fn string(&self) -> String {
                let args: Vec<String> = self.arguments.iter()
                    .map(|arg| arg.string())
                    .collect();
                format!("{}({})", self.function.string(), args.join(", "))
            }
        }
        
        impl Expression for CallExpression {
            fn expression_node(&self) {}
            
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        
        // Parse the arguments
        let arguments = self.parse_expression_list(Token::RParen)?;
        
        // Return the call expression
        Ok(Box::new(CallExpression {
            token: "call".to_string(), // Using a placeholder
            function,
            arguments,
        }))
    }
    
    /// Parse a list of expressions separated by commas
    fn parse_expression_list(&mut self, end_token: Token) -> Result<Vec<Box<dyn Expression>>, Error> {
        let mut expressions = Vec::new();
        
        // Handle empty list
        if self.peek_token_is(&end_token) {
            self.next_token();
            return Ok(expressions);
        }
        
        // Parse the first expression
        self.next_token();
        expressions.push(self.parse_expression(Precedence::Lowest)?);
        
        // Parse subsequent expressions
        while self.peek_token_is(&Token::Comma) {
            self.next_token(); // Skip the comma
            self.next_token(); // Move to the next expression
            expressions.push(self.parse_expression(Precedence::Lowest)?);
        }
        
        // Expect the end token
        self.expect_peek(end_token)?;
        
        Ok(expressions)
    }
    
    /// Parse an index expression (e.g., array[1])
    fn parse_index_expression(&mut self, left: Box<dyn Expression>) -> Result<Box<dyn Expression>, Error> {
        // Create a struct to represent the index expression
        struct IndexExpression {
            token: String,
            left: Box<dyn Expression>,
            index: Box<dyn Expression>,
        }
        
        // Implement necessary traits for IndexExpression
        impl Node for IndexExpression {
            fn token_literal(&self) -> String {
                self.token.clone()
            }
            
            fn string(&self) -> String {
                format!("({}[{}])", self.left.string(), self.index.string())
            }
        }
        
        impl Expression for IndexExpression {
            fn expression_node(&self) {}
            
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        
        // Move past the '[' token
        self.next_token();
        
        // Parse the index expression
        let index = self.parse_expression(Precedence::Lowest)?;
        
        // Expect the closing ']'
        self.expect_peek(Token::RBracket)?;
        
        // Return the index expression
        Ok(Box::new(IndexExpression {
            token: "index".to_string(), // Using a placeholder
            left,
            index,
        }))
    }
    
    /// Get the precedence of the peek token
    fn peek_precedence(&self) -> Precedence {
        match &self.peek_token {
            Some(Token::Eq) | Some(Token::NotEq) => Precedence::Equals,
            Some(Token::Lt) | Some(Token::Gt) | Some(Token::LtEq) | Some(Token::GtEq) => Precedence::LessGreater,
            Some(Token::Plus) | Some(Token::Minus) => Precedence::Sum,
            Some(Token::Asterisk) | Some(Token::Slash) => Precedence::Product,
            Some(Token::LParen) => Precedence::Call,
            Some(Token::LBracket) => Precedence::Index,
            _ => Precedence::Lowest,
        }
    }
    
    /// Get the precedence of the current token
    fn current_precedence(&self) -> Precedence {
        match &self.current_token {
            Some(Token::Eq) | Some(Token::NotEq) => Precedence::Equals,
            Some(Token::Lt) | Some(Token::Gt) | Some(Token::LtEq) | Some(Token::GtEq) => Precedence::LessGreater,
            Some(Token::Plus) | Some(Token::Minus) => Precedence::Sum,
            Some(Token::Asterisk) | Some(Token::Slash) => Precedence::Product,
            Some(Token::LParen) => Precedence::Call,
            Some(Token::LBracket) => Precedence::Index,
            _ => Precedence::Lowest,
        }
    }
} 