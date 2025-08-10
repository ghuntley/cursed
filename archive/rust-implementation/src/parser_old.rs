// Parser module for CURSED language
use crate::ast::{Program, Ast, Statement, FunctionStatement, Parameter, Expression, LetStatement, Type, Visibility, LetTarget, Literal};
use crate::lexer::{Lexer, Token, TokenKind};
use crate::error_types::{Error, Result};

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Result<Self> {
        let current_token = match lexer.next_token() {
            Ok(token) => Some(token),
            Err(_) => None,
        };
        Ok(Parser {
            lexer,
            current_token,
        })
    }

    pub fn from_tokens(tokens: Vec<Token>) -> Self {
        // Create a basic lexer from tokens - simplified implementation
        let mut lexer = Lexer::new(String::new());
        let current_token = tokens.first().cloned();
        Parser {
            lexer,
            current_token,
        }
    }

    pub fn parse_program(&mut self) -> Result<Program> {
        let mut statements = Vec::new();
        
        // Parse statements until we reach EOF
        while let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::EOF {
                break;
            }
            
            // Skip whitespace and comments
            if token.kind == TokenKind::Whitespace || token.kind == TokenKind::Comment {
                self.next_token()?;
                continue;
            }
            
            // Try to parse a statement
            if let Some(statement) = self.parse_statement()? {
                statements.push(statement);
            }
            
            // If we didn't advance, break to avoid infinite loop
            if self.current_token.is_none() {
                break;
            }
        }
        
        Ok(Program {
            statements,
            imports: vec![],
            package: None,
        })
    }

    pub fn parse(&mut self) -> Result<Ast> {
        // Basic implementation
        Ok(Ast::Program(self.parse_program()?))
    }

    pub fn errors(&self) -> Vec<Error> {
        // Return empty errors for now
        vec![]
    }

    fn next_token(&mut self) -> Result<()> {
        self.current_token = match self.lexer.next_token() {
            Ok(token) => Some(token),
            Err(_) => None,
        };
        Ok(())
    }

    fn peek_token(&self) -> Option<&Token> {
        self.current_token.as_ref()
    }
    
    fn parse_statement(&mut self) -> Result<Option<Statement>> {
        let token = match self.current_token.as_ref() {
            Some(token) => token,
            None => return Ok(None),
        };
        
        match token.kind {
            TokenKind::Identifier => {
                let value = token.value.clone();
                match value.as_str() {
                    "slay" => {
                        // Parse function declaration
                        return Ok(Some(Statement::Function(self.parse_function()?)));
                    }
                    "sus" => {
                        // Parse variable declaration
                        return Ok(Some(Statement::Let(self.parse_let_statement()?)));
                    }
                    _ => {
                        // Skip unknown identifiers for now
                        self.next_token()?;
                        return Ok(None);
                    }
                }
            }
            _ => {
                // Skip unknown tokens for now
                self.next_token()?;
                return Ok(None);
            }
        }
    }
    
    fn parse_function(&mut self) -> Result<FunctionStatement> {
        // Consume 'slay' keyword
        self.next_token()?;
        
        // Parse function name
        let name = match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::Identifier => {
                let name = token.value.clone();
                self.next_token()?;
                name
            }
            _ => return Err(Error::ParseError("Expected function name".to_string())),
        };
        
        // Parse parameters
        let parameters = self.parse_parameters()?;
        
        // Parse return type (optional)
        let return_type = None; // Simplified for now
        
        // Parse function body
        let body = self.parse_block()?;
        
        Ok(FunctionStatement {
            name,
            type_parameters: vec![],
            parameters,
            body,
            return_type,
            where_clause: None,
            visibility: Visibility::Public,
        })
    }
    
    fn parse_parameters(&mut self) -> Result<Vec<Parameter>> {
        // Expect '('
        match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::LeftParen => {
                self.next_token()?;
            }
            _ => return Err(Error::ParseError("Expected '(' after function name".to_string())),
        }
        
        let mut parameters = Vec::new();
        
        // Parse parameter list
        while let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::RightParen {
                self.next_token()?;
                break;
            }
            
            if token.kind == TokenKind::Identifier {
                let param_name = token.value.clone();
                self.next_token()?;
                
                // Simplified: assume no type for now
                parameters.push(Parameter {
                    name: param_name,
                    param_type: None,
                });
                
                // Skip comma if present
                if let Some(token) = self.current_token.as_ref() {
                    if token.kind == TokenKind::Comma {
                        self.next_token()?;
                    }
                }
            } else {
                self.next_token()?;
            }
        }
        
        Ok(parameters)
    }
    
    fn parse_block(&mut self) -> Result<Vec<Statement>> {
        // Expect '{'
        match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::LeftBrace => {
                self.next_token()?;
            }
            _ => return Err(Error::ParseError("Expected '{' to start function body".to_string())),
        }
        
        let mut statements = Vec::new();
        
        // Parse statements until we reach '}'
        while let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::RightBrace {
                self.next_token()?;
                break;
            }
            
            // Skip whitespace and comments
            if token.kind == TokenKind::Whitespace || token.kind == TokenKind::Comment {
                self.next_token()?;
                continue;
            }
            
            // Try to parse a statement
            if let Some(statement) = self.parse_statement()? {
                statements.push(statement);
            }
            
            // If we didn't advance, break to avoid infinite loop
            if self.current_token.is_none() {
                break;
            }
        }
        
        Ok(statements)
    }
    
    fn parse_let_statement(&mut self) -> Result<LetStatement> {
        // Consume 'sus' keyword
        self.next_token()?;
        
        // Parse variable name
        let name = match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::Identifier => {
                let name = token.value.clone();
                self.next_token()?;
                name
            }
            _ => return Err(Error::ParseError("Expected variable name".to_string())),
        };
        
        // Parse type (optional)
        let var_type = self.parse_type()?;
        
        // Parse equals sign and value
        let value = match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::Equals => {
                self.next_token()?;
                Some(self.parse_expression()?)
            }
            _ => None,
        };
        
        Ok(LetStatement {
            name,
            value,
            var_type,
            mutable: false,
        })
    }
    
    fn parse_type(&mut self) -> Result<Option<Type>> {
        // Simplified type parsing for arrays
        if let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::LeftBracket {
                self.next_token()?;
                
                // Parse array size (simplified - skip for now)
                while let Some(token) = self.current_token.as_ref() {
                    if token.kind == TokenKind::RightBracket {
                        self.next_token()?;
                        break;
                    }
                    self.next_token()?;
                }
                
                // Parse element type
                if let Some(token) = self.current_token.as_ref() {
                    if token.kind == TokenKind::Identifier {
                        let type_name = token.value.clone();
                        self.next_token()?;
                        return Ok(Some(Type::Array(Box::new(Type::Primitive(type_name)))));
                    }
                }
            }
        }
        
        Ok(None)
    }
    
    fn parse_expression(&mut self) -> Result<Expression> {
        // Simplified expression parsing
        if let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::LeftBracket {
                // Parse array literal
                self.next_token()?;
                let mut elements = Vec::new();
                
                while let Some(token) = self.current_token.as_ref() {
                    if token.kind == TokenKind::RightBracket {
                        self.next_token()?;
                        break;
                    }
                    
                    if token.kind == TokenKind::Number {
                        elements.push(Expression::Literal(token.value.clone()));
                        self.next_token()?;
                    } else {
                        self.next_token()?;
                    }
                    
                    // Skip comma if present
                    if let Some(token) = self.current_token.as_ref() {
                        if token.kind == TokenKind::Comma {
                            self.next_token()?;
                        }
                    }
                }
                
                return Ok(Expression::Array(elements));
            }
        }
        
        Ok(Expression::Literal("".to_string()))
    }
}

// Factory function for creating new parser
pub fn new_parser(source: &str) -> Result<Parser> {
    let lexer = Lexer::new(source.to_string());
    Parser::new(lexer)
}
