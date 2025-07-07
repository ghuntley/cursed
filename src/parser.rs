// Parser module for CURSED language
use crate::ast::{Program, Ast, Statement, FunctionStatement, Parameter, Expression, LetStatement, IfStatement, Type, Visibility, LetTarget, Literal, BinaryExpression};
use crate::lexer::{Lexer, Token, TokenKind};
use crate::error_types::{Error, Result};

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
    tokens: Vec<Token>,
    token_index: usize,
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
            tokens: Vec::new(),
            token_index: 0,
        })
    }

    pub fn from_tokens(tokens: Vec<Token>) -> Self {
        // Create a parser that works with a list of tokens
        let lexer = Lexer::new(String::new()); // Dummy lexer
        let current_token = tokens.first().cloned();
        Parser {
            lexer,
            current_token,
            tokens: tokens,
            token_index: 0,
        }
    }

    pub fn parse_program(&mut self) -> Result<Program> {
        let mut statements = Vec::new();
        let mut imports = Vec::new();
        let mut package = None;
        
        // Parse statements until we reach EOF
        while let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::Eof {
                break;
            }
            
            // Skip newlines
            if token.kind == TokenKind::Newline {
                self.next_token()?;
                continue;
            }
            
            // Check for package declaration first
            if token.kind == TokenKind::Vibe && package.is_none() {
                package = Some(self.parse_package_declaration()?);
                continue;
            }
            
            // Check for import statements
            if token.kind == TokenKind::Yeet {
                imports.push(self.parse_import_statement()?);
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
            imports,
            package,
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
        if !self.tokens.is_empty() {
            // Using tokens list (for testing)
            self.token_index += 1;
            self.current_token = if self.token_index < self.tokens.len() {
                Some(self.tokens[self.token_index].clone())
            } else {
                None
            };
        } else {
            // Using lexer (normal operation)
            self.current_token = match self.lexer.next_token() {
                Ok(token) => Some(token),
                Err(_) => None,
            };
        }
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
                let value = token.lexeme.clone();
                match value.as_str() {
                    "slay" => {
                        // Parse function declaration
                        return Ok(Some(Statement::Function(self.parse_function()?)));
                    }
                    _ => {
                        // Skip unknown identifiers for now
                        self.next_token()?;
                        return Ok(None);
                    }
                }
            }
            TokenKind::Slay => {
                // Parse function declaration
                return Ok(Some(Statement::Function(self.parse_function()?)));
            }
            TokenKind::Sus => {
                // Parse variable declaration
                return Ok(Some(Statement::Let(self.parse_let_statement()?)));
            }
            TokenKind::Lowkey => {
                // Parse if statement
                return Ok(Some(Statement::If(self.parse_if_statement()?)));
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
                let name = token.lexeme.clone();
                self.next_token()?;
                name
            }
            _ => return Err(Error::Parse("Expected function name".to_string())),
        };
        
        // Parse parameters
        let parameters = self.parse_parameters()?;
        
        // Parse return type (optional)
        let return_type = if let Some(token) = self.current_token.as_ref() {
            match token.kind {
                TokenKind::Normie => { self.next_token()?; Some(Type::Normie) },
                TokenKind::Smol => { self.next_token()?; Some(Type::Smol) },
                TokenKind::Mid => { self.next_token()?; Some(Type::Mid) },
                TokenKind::Thicc => { self.next_token()?; Some(Type::Thicc) },
                TokenKind::Snack => { self.next_token()?; Some(Type::Snack) },
                TokenKind::Meal => { self.next_token()?; Some(Type::Meal) },
                TokenKind::Tea => { self.next_token()?; Some(Type::Tea) },
                TokenKind::Lit => { self.next_token()?; Some(Type::Lit) },
                TokenKind::Sip => { self.next_token()?; Some(Type::Sip) },
                TokenKind::Byte => { self.next_token()?; Some(Type::Byte) },
                TokenKind::Rune => { self.next_token()?; Some(Type::Rune) },
                TokenKind::Extra => { self.next_token()?; Some(Type::Extra) },
                _ => None
            }
        } else {
            None
        };
        
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
            _ => return Err(Error::Parse("Expected '(' after function name".to_string())),
        }
        
        let mut parameters = Vec::new();
        
        // Parse parameter list
        while let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::RightParen {
                self.next_token()?;
                break;
            }
            
            if token.kind == TokenKind::Identifier {
                let param_name = token.lexeme.clone();
                self.next_token()?;
                
                // Parse parameter type if present
                let param_type = if let Some(token) = self.current_token.as_ref() {
                    // Check if next token is a type token
                    match token.kind {
                        TokenKind::Normie => { self.next_token()?; Some(Type::Normie) },
                        TokenKind::Smol => { self.next_token()?; Some(Type::Smol) },
                        TokenKind::Mid => { self.next_token()?; Some(Type::Mid) },
                        TokenKind::Thicc => { self.next_token()?; Some(Type::Thicc) },
                        TokenKind::Snack => { self.next_token()?; Some(Type::Snack) },
                        TokenKind::Meal => { self.next_token()?; Some(Type::Meal) },
                        TokenKind::Tea => { self.next_token()?; Some(Type::Tea) },
                        TokenKind::Lit => { self.next_token()?; Some(Type::Lit) },
                        TokenKind::Sip => { self.next_token()?; Some(Type::Sip) },
                        TokenKind::Byte => { self.next_token()?; Some(Type::Byte) },
                        TokenKind::Rune => { self.next_token()?; Some(Type::Rune) },
                        TokenKind::Extra => { self.next_token()?; Some(Type::Extra) },
                        _ => None
                    }
                } else {
                    None
                };
                
                parameters.push(Parameter {
                    name: param_name,
                    param_type,
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
            _ => return Err(Error::Parse("Expected '{' to start function body".to_string())),
        }
        
        let mut statements = Vec::new();
        
        // Parse statements until we reach '}'
        while let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::RightBrace {
                self.next_token()?;
                break;
            }
            
            // Skip newlines
            if token.kind == TokenKind::Newline {
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
                let name = token.lexeme.clone();
                self.next_token()?;
                name
            }
            _ => return Err(Error::Parse("Expected variable name".to_string())),
        };
        
        // Parse type (optional)
        let var_type = self.parse_type()?;
        
        // Parse equals sign and value
        let value = match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::Equal => {
                self.next_token()?;
                self.parse_expression()?
            }
            _ => Expression::Literal(Literal::Nil),
        };
        
        Ok(LetStatement {
            target: LetTarget::Single(name),
            value,
            var_type,
            visibility: Visibility::Private,
        })
    }
    
    fn parse_if_statement(&mut self) -> Result<IfStatement> {
        // Consume 'lowkey' keyword
        self.next_token()?;
        
        // Parse condition expression
        let condition = self.parse_expression()?;
        
        // Parse then branch
        let then_branch = self.parse_block()?;
        
        // Check for else branch (highkey)
        let else_branch = if let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::Highkey {
                self.next_token()?; // consume 'highkey'
                Some(self.parse_block()?)
            } else {
                None
            }
        } else {
            None
        };
        
        Ok(IfStatement {
            init: None,
            condition,
            then_branch,
            else_branch,
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
                        let type_name = token.lexeme.clone();
                        self.next_token()?;
                        let element_type = match type_name.as_str() {
                            "normie" => Type::Normie,
                            "tea" => Type::Tea,
                            "lit" => Type::Lit,
                            _ => Type::Custom(type_name),
                        };
                        return Ok(Some(Type::Array(Box::new(element_type), None)));
                    }
                }
            }
        }
        
        Ok(None)
    }
    
    fn parse_expression(&mut self) -> Result<Expression> {
        // Simplified expression parsing
        if let Some(token) = self.current_token.as_ref() {
            match token.kind {
                TokenKind::LeftBracket => {
                    // Parse array literal
                    self.next_token()?;
                    let mut elements = Vec::new();
                    
                    while let Some(token) = self.current_token.as_ref() {
                        if token.kind == TokenKind::RightBracket {
                            self.next_token()?;
                            break;
                        }
                        
                        if token.kind == TokenKind::Number {
                            elements.push(Expression::Literal(Literal::String(token.lexeme.clone())));
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
                TokenKind::Identifier => {
                    // Parse identifier, possibly part of binary expression
                    let name = token.lexeme.clone();
                    self.next_token()?;
                    
                    // Check for binary operators
                    if let Some(token) = self.current_token.as_ref() {
                        match token.kind {
                            TokenKind::Greater => {
                                self.next_token()?;
                                let right = self.parse_expression()?;
                                return Ok(Expression::Binary(BinaryExpression {
                                    left: Box::new(Expression::Identifier(name)),
                                    operator: ">".to_string(),
                                    right: Box::new(right),
                                }));
                            }
                            TokenKind::Less => {
                                self.next_token()?;
                                let right = self.parse_expression()?;
                                return Ok(Expression::Binary(BinaryExpression {
                                    left: Box::new(Expression::Identifier(name)),
                                    operator: "<".to_string(),
                                    right: Box::new(right),
                                }));
                            }
                            TokenKind::EqualEqual => {
                                self.next_token()?;
                                let right = self.parse_expression()?;
                                return Ok(Expression::Binary(BinaryExpression {
                                    left: Box::new(Expression::Identifier(name)),
                                    operator: "==".to_string(),
                                    right: Box::new(right),
                                }));
                            }
                            _ => {
                                return Ok(Expression::Identifier(name));
                            }
                        }
                    }
                    
                    return Ok(Expression::Identifier(name));
                }
                TokenKind::Number => {
                    // Parse number literal
                    let value = token.lexeme.clone();
                    self.next_token()?;
                    return Ok(Expression::Literal(Literal::Integer(value.parse().unwrap_or(0))));
                }
                TokenKind::String => {
                    // Parse string literal
                    let value = token.lexeme.clone();
                    self.next_token()?;
                    return Ok(Expression::Literal(Literal::String(value)));
                }
                TokenKind::Truth | TokenKind::Based => {
                    // Parse boolean literal
                    self.next_token()?;
                    return Ok(Expression::Literal(Literal::Boolean(true)));
                }
                _ => {
                    // Skip unknown tokens and return placeholder
                    self.next_token()?;
                }
            }
        }
        
        Ok(Expression::Literal(Literal::String("".to_string())))
    }

    fn parse_package_declaration(&mut self) -> Result<crate::ast::PackageDeclaration> {
        // Consume 'vibe' keyword
        self.next_token()?;
        
        // Parse package name
        let name = match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::Identifier => {
                let name = token.lexeme.clone();
                self.next_token()?;
                name
            }
            _ => return Err(Error::Parse("Expected package name after 'vibe'".to_string())),
        };
        
        Ok(crate::ast::PackageDeclaration {
            name,
            version: None,
        })
    }

    fn parse_import_statement(&mut self) -> Result<crate::ast::ImportStatement> {
        // Consume 'yeet' keyword
        self.next_token()?;
        
        // Parse import path (string literal)
        let path = match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::String => {
                let path = token.lexeme.clone().trim_matches('"').to_string();
                self.next_token()?;
                path
            }
            _ => return Err(Error::Parse("Expected string literal after 'yeet'".to_string())),
        };
        
        Ok(crate::ast::ImportStatement {
            path,
            alias: None,
            items: Vec::new(),
        })
    }
}

// Factory function for creating new parser
pub fn new_parser(source: &str) -> Result<Parser> {
    let lexer = Lexer::new(source.to_string());
    Parser::new(lexer)
}
