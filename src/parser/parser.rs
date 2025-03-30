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
    Dot,         // object.property
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
            Token::Sus => self.parse_sus_statement(),
            Token::Facts => self.parse_facts_statement(),
            Token::Yolo => self.parse_return_statement(),
            Token::Lowkey => self.parse_if_statement(),
            Token::Periodt => self.parse_while_statement(),
            Token::Bestie => self.parse_for_statement(),
            Token::VibeCheck => self.parse_switch_statement(),
            Token::BeLike => self.parse_type_statement(),
            Token::Slay => {
                // Check if this is a method declaration (look ahead for colon)
                // First save the current position
                let current_token = self.current_token.clone();
                let peek_token = self.peek_token.clone();
                
                // Look ahead to see if this is a method declaration
                let is_method_declaration = {
                    // Move past 'slay'
                    if !self.expect_peek_identifier() {
                        return self.parse_expression_statement();
                    }
                    
                    // Move past type name
                    self.next_token()?;
                    
                    // Check if next token is colon
                    let is_method = self.current_token == Token::Colon;
                    
                    // Restore the tokens
                    self.current_token = current_token;
                    self.peek_token = peek_token;
                    
                    is_method
                };
                
                if is_method_declaration {
                    self.parse_method_declaration()
                } else {
                    self.parse_expression_statement()
                }
            },
            _ => self.parse_expression_statement(),
        }
    }
    
    /// Parse a package statement
    pub fn parse_package_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        // Store the 'vibe' token
        let token = self.current_token.token_literal();
        
        // Next token should be the package name (identifier)
        if !self.expect_peek(&Token::Identifier("".to_string())) {
            return Err(Error::from_str(
                &format!("Expected identifier after 'vibe', got {:?}", self.peek_token)
            ));
        }
        
        // Get the package name
        let name = match &self.current_token {
            Token::Identifier(val) => ast::Identifier {
                token: self.current_token.token_literal(),
                value: val.clone(),
            },
            _ => unreachable!(), // We already checked it's an identifier
        };
        
        // Expect a semicolon
        if !self.expect_peek(&Token::Semicolon) {
            return Err(Error::from_str(
                &format!("Expected ';' after package name, got {:?}", self.peek_token)
            ));
        }
        
        // Create and return the package statement
        Ok(Box::new(ast::PackageStatement {
            token,
            name,
        }))
    }
    
    /// Parse an import statement
    pub fn parse_import_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        // Store the 'yeet' token
        let token = self.current_token.token_literal();
        
        // Handle optional alias first
        let mut alias = None;
        
        // Next token could be an identifier (alias) or a string (path)
        self.next_token()?;
        
        // Check if we have an alias
        if let Token::Identifier(name) = &self.current_token {
            alias = Some(ast::Identifier {
                token: self.current_token.token_literal(),
                value: name.clone(),
            });
            
            // Move to the next token, which should be the path
            self.next_token()?;
        }
        
        // Next token must be a string literal (path)
        if let Token::String(path_value) = &self.current_token {
            let path = ast::StringLiteral {
                token: self.current_token.token_literal(),
                value: path_value.clone(),
            };
            
            // Expect a semicolon
            if !self.expect_peek(&Token::Semicolon) {
                return Err(Error::from_str(
                    &format!("Expected ';' after import path, got {:?}", self.peek_token)
                ));
            }
            
            // Create and return the import statement
            Ok(Box::new(ast::ImportStatement {
                token,
                path,
                alias,
            }))
        } else {
            Err(Error::from_str(
                &format!("Expected string literal for import path, got {:?}", self.current_token)
            ))
        }
    }
    
    /// Parse a let statement (now 'sus')
    pub fn parse_sus_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.current_token.token_literal();
        
        // Expect the identifier name
        if !self.expect_peek_identifier() {
            return Err(Error::from_str(
                &format!("Expected identifier after 'sus', got {:?}", self.peek_token)
            ));
        }
        
        let name = match &self.current_token {
            Token::Identifier(val) => ast::Identifier {
                token: self.current_token.token_literal(),
                value: val.clone(),
            },
            _ => unreachable!(), // Already checked
        };
        
        // Check for optional type annotation (smol, mid, normie, thicc)
        let mut type_annotation = None;
        if matches!(self.peek_token, Token::Smol | Token::Mid | Token::Normie | Token::Thicc) {
            self.next_token()?; // Consume the type token
            type_annotation = Some(self.current_token.clone());
        }
        
        // Expect the assignment operator
        if !self.expect_peek(&Token::Assign) {
            return Err(Error::from_str(
                &format!("Expected '=' after identifier in sus statement, got {:?}", self.peek_token)
            ));
        }
        
        // Move past the '=' token
        self.next_token()?;
        
        // Parse the expression value
        let value = self.parse_expression(Precedence::Lowest)?;
        
        // Optionally consume a semicolon
        if self.peek_token == Token::Semicolon {
            self.next_token()?;
        }
        
        // Create and return the let statement with type annotation
        let let_stmt = ast::LetStatement {
            token,
            name,
            value: Some(value),
            type_annotation, // Include the type annotation
        };
        
        Ok(Box::new(let_stmt))
    }
    
    /// Parse a facts statement (constant declaration)
    pub fn parse_facts_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.current_token.token_literal();
        
        // Expect the identifier name
        if !self.expect_peek_identifier() {
            return Err(Error::from_str(
                &format!("Expected identifier after 'facts', got {:?}", self.peek_token)
            ));
        }
        
        let name = match &self.current_token {
            Token::Identifier(val) => ast::Identifier {
                token: self.current_token.token_literal(),
                value: val.clone(),
            },
            _ => unreachable!(), // Already checked
        };
        
        // Expect the assignment operator
        if !self.expect_peek(&Token::Assign) {
            return Err(Error::from_str(
                &format!("Expected '=' after identifier in facts statement, got {:?}", self.peek_token)
            ));
        }
        
        // Move past the '=' token
        self.next_token()?;
        
        // Parse the expression value
        let value = self.parse_expression(Precedence::Lowest)?;
        
        // Optionally consume a semicolon
        if self.peek_token == Token::Semicolon {
            self.next_token()?;
        }
        
        // Create and return the facts statement
        Ok(Box::new(ast::FactsStatement {
            token,
            name,
            value,
        }))
    }
    
    /// Parse a return statement
    pub fn parse_return_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.current_token.token_literal();
        
        // Move past the 'yolo' token
        self.next_token()?;
        
        let return_value = if self.current_token == Token::Semicolon {
            // No return value provided
            None
        } else {
            // Parse the expression for the return value
            Some(self.parse_expression(Precedence::Lowest)?)
        };
        
        // Optionally consume a semicolon
        if self.current_token == Token::Semicolon {
            self.next_token()?;
        }
        
        // Create and return the ReturnStatement
        Ok(Box::new(ast::ReturnStatement {
            token,
            return_value,
        }))
    }
    
    /// Parse an if statement
    /// 
    /// The CURSED language allows for optional parentheses around the condition.
    /// Both `lowkey expression { ... }` and `lowkey (expression) { ... }` are valid.
    pub fn parse_if_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.current_token.token_literal(); // 'lowkey' token
        
        // Move past 'lowkey'
        self.next_token()?;
        
        // Check if there's an opening parenthesis (optional according to the grammar)
        let has_parens = self.current_token == Token::LParen;
        
        // If there's an opening parenthesis, consume it
        if has_parens {
            // Move past '('
            self.next_token()?;
        }
        
        // Parse the condition expression
        let condition = self.parse_expression(Precedence::Lowest)?;
        
        // If we had an opening parenthesis, expect a closing one
        if has_parens {
            if self.current_token != Token::RParen {
                return Err(Error::from_str(
                    &format!("Expected next token to be RParen, got {:?}", self.current_token)
                ));
            }
            // Move past ')'
            self.next_token()?;
        }
        
        // Expect an opening brace for the consequence block
        if self.current_token != Token::LBrace {
            return Err(Error::from_str(
                &format!("Expected '{{' after condition, got {:?}", self.current_token)
            ));
        }
        
        // Parse the consequence block
        let consequence = self.parse_block_statement()?;
        
        // Check for an optional 'highkey' (else) block
        let mut alternative = None;
        if self.peek_token == Token::Highkey {
            // Move past 'highkey'
            self.next_token()?;
            
            // Expect an opening brace for the alternative block
            if !self.expect_peek(&Token::LBrace) {
                return Err(Error::from_str(
                    &format!("Expected '{{' after highkey, got {:?}", self.peek_token)
                ));
            }
            
            // Parse the alternative block
            alternative = Some(self.parse_block_statement()?);
        }
        
        Ok(Box::new(ast::IfStatement {
            token,
            condition,
            consequence,
            alternative,
        }))
    }
    
    /// Parse a for statement
    pub fn parse_for_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.current_token.token_literal(); // 'bestie' token
        
        // Initialize optional parts
        let mut _init: Option<Box<dyn Statement>> = None;
        let mut _condition: Option<Box<dyn Expression>> = None;
        let mut _post: Option<Box<dyn Statement>> = None;

        // Look ahead to determine the form of the for loop
        // The structure depends on whether the next token after 'bestie' is '{' or something else
        self.next_token()?; // Consume 'bestie'

        // Case 1: Infinite loop - bestie { body }
        if self.current_token == Token::LBrace {
            let body = self.parse_block_statement()?;
            return Ok(Box::new(ast::ForStatement {
                token,
                init: None,
                condition: None,
                post: None,
                body,
            }));
        }

        // Case 2 or 3: Condition-only or C-style
        // We need to check for semicolons to distinguish
        
        // Try parsing the first part (could be init statement or condition expression)
        // Determine if the first part looks like an init statement (sus or facts)
        let first_part_is_init_stmt = matches!(self.current_token, Token::Sus | Token::Facts);

        if first_part_is_init_stmt {
            // C-style loop with init statement
            _init = Some(self.parse_statement()?);

            // Expect semicolon after init
            if !self.expect_peek(&Token::Semicolon) {
                return Err(Error::from_str(
                    &format!("Expected ';' after for loop initializer, got {:?}", self.peek_token)
                ));
            }
            
            // Parse condition (optional)
            if self.peek_token != Token::Semicolon {
                self.next_token()?;
                _condition = Some(self.parse_expression(Precedence::Lowest)?);
            } else {
                // If it's a semicolon, there's no condition
                _condition = None;
            }
            
            // Expect semicolon after condition
            if !self.expect_peek(&Token::Semicolon) {
                return Err(Error::from_str(
                    &format!("Expected ';' after for loop condition, got {:?}", self.peek_token)
                ));
            }
            
            // Parse post statement (optional)
            if self.peek_token != Token::LBrace {
                self.next_token()?;
                // Treat post as an expression statement for simplicity
                // Need to ensure parse_statement can handle expressions correctly here
                _post = Some(self.parse_expression_statement()?);
            } else {
                // If next is LBrace, there's no post statement
                _post = None;
            }
            
            // Expect opening brace for body
            if !self.expect_peek(&Token::LBrace) {
                return Err(Error::from_str(
                    &format!("Expected '{{' to start for loop body, got {:?}", self.peek_token)
                ));
            }
            let body = self.parse_block_statement()?;

            return Ok(Box::new(ast::ForStatement {
                token,
                init: _init,
                condition: _condition,
                post: _post,
                body,
            }));

        } else {
            // Either Condition-only or C-style loop without init
            // The first part MUST be the condition
            _condition = Some(self.parse_expression(Precedence::Lowest)?);
            
            if self.current_token == Token::LBrace {
                // Condition-only loop: bestie condition { body }
                 let body = self.parse_block_statement()?;
                 return Ok(Box::new(ast::ForStatement {
                     token,
                     init: None, // No init in this form
                     condition: _condition,
                     post: None, // No post in this form
                     body,
                 }));
            } else if self.current_token == Token::Semicolon {
                 // C-style loop without init: bestie ; condition ; post { body }
                 // We've already parsed the condition. We need to adjust.
                 // Let's restart the parsing for this specific sub-case for clarity.
                 
                 // Reset condition, we will parse it again after the first semicolon.
                 _condition = None;
                 
                 // We already consumed the first semicolon implicitly by checking self.current_token
                 
                 // Parse condition (optional) - after the first semicolon
                 if self.peek_token != Token::Semicolon {
                    self.next_token()?;
                    _condition = Some(self.parse_expression(Precedence::Lowest)?);
                 }
                 
                 // Expect semicolon after condition
                 if !self.expect_peek(&Token::Semicolon) {
                     return Err(Error::from_str(
                         &format!("Expected ';' after for loop condition, got {:?}", self.peek_token)
                     ));
                 }
                 
                 // Parse post statement (optional)
                 if self.peek_token != Token::LBrace {
                     self.next_token()?;
                     _post = Some(self.parse_expression_statement()?);
                 } else {
                     _post = None;
                 }
                 
                 // Expect opening brace for body
                 if !self.expect_peek(&Token::LBrace) {
                     return Err(Error::from_str(
                         &format!("Expected '{{' to start for loop body, got {:?}", self.peek_token)
                     ));
                 }
                 let body = self.parse_block_statement()?;

                 return Ok(Box::new(ast::ForStatement {
                     token,
                     init: None, // No init in this form
                     condition: _condition,
                     post: _post,
                     body,
                 }));
            } else {
                 // Syntax error if it's not '{' or ';' after the first part
                 return Err(Error::from_str(
                     &format!("Expected '{{' or ';' after for loop clause, got {:?}", self.current_token)
                 ));
            }
        }
    }
    
    /// Parse a switch statement
    pub fn parse_switch_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.current_token.token_literal(); // 'vibe_check' token

        // Move past 'vibe_check'
        self.next_token()?;

        // Parse the value expression being switched on
        let value = self.parse_expression(Precedence::Lowest)?;

        // Expect opening brace '{'
        if !self.expect_peek(&Token::LBrace) {
            return Err(Error::from_str(
                &format!("Expected '{{' after switch value, got {:?}", self.peek_token)
            ));
        }

        let mut cases = Vec::new();
        let mut default: Option<ast::BlockStatement> = None;

        // Move past '{'
        self.next_token()?;

        // Parse case statements until closing brace '}'
        while self.current_token != Token::RBrace && self.current_token != Token::Eof {
            match &self.current_token {
                Token::Mood => {
                    cases.push(self.parse_case_statement()?);
                }
                Token::Basic => {
                    if default.is_some() {
                        return Err(Error::from_str("Only one 'basic' (default) case allowed in vibe_check statement"));
                    }
                    self.next_token()?; // Consume 'basic'
                    if !self.expect_peek(&Token::Colon) {
                        return Err(Error::from_str(
                            &format!("Expected ':' after 'basic', got {:?}", self.peek_token)
                        ));
                    }
                    default = Some(self.parse_block_statement()?);
                }
                _ => {
                    return Err(Error::from_str(
                        &format!("Expected 'mood' or 'basic' inside vibe_check, got {:?}", self.current_token)
                    ));
                }
            }
        }

        // Ensure we have a closing brace
        if self.current_token != Token::RBrace {
            return Err(Error::from_str(
                "Expected '}' to close vibe_check statement"
            ));
        }

        Ok(Box::new(ast::SwitchStatement {
            token,
            value,
            cases,
            default,
        }))
    }

    /// Parse a case statement (mood) within a switch
    fn parse_case_statement(&mut self) -> Result<ast::CaseStatement, Error> {
        let token = self.current_token.token_literal(); // 'mood' token

        // Move past 'mood'
        self.next_token()?;

        let mut expressions = Vec::new();

        // Parse first case expression
        expressions.push(self.parse_expression(Precedence::Lowest)?);

        // Parse additional comma-separated expressions
        while self.current_token == Token::Comma {
            self.next_token()?; // Consume ','
            expressions.push(self.parse_expression(Precedence::Lowest)?);
        }

        // Expect colon ':' after expressions
        if !self.expect_peek(&Token::Colon) {
            return Err(Error::from_str(
                &format!("Expected ':' after mood expressions, got {:?}", self.peek_token)
            ));
        }

        // Parse the block statement for the case body
        let body = self.parse_block_statement()?;

        Ok(ast::CaseStatement {
            token,
            expressions,
            body,
        })
    }
    
    /// Parse a type statement
    pub fn parse_type_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        // Store the 'be_like' token
        let token = self.current_token.token_literal();
        
        // Next token should be the type name (identifier)
        if !self.expect_peek_identifier() {
            return Err(Error::from_str(
                &format!("Expected identifier after 'be_like', got {:?}", self.peek_token)
            ));
        }
        
        // Get the type name
        let name = match &self.current_token {
            Token::Identifier(val) => ast::Identifier {
                token: self.current_token.token_literal(),
                value: val.clone(),
            },
            _ => unreachable!(), // We already checked it's an identifier
        };
        
        // Next token determines if this is a struct or interface declaration
        self.next_token()?;
        match &self.current_token {
            Token::Squad => self.parse_squad_statement(token, name),
            Token::Collab => self.parse_collab_statement(token, name),
            _ => Err(Error::from_str(
                &format!("Expected 'squad' or 'collab' after type name, got {:?}", self.current_token)
            ))
        }
    }
    
    /// Parse a struct (squad) declaration
    fn parse_squad_statement(&mut self, token: String, name: ast::Identifier) -> Result<Box<dyn Statement>, Error> {
        // Next token should be opening brace '{'
        if !self.expect_peek(&Token::LBrace) {
            return Err(Error::from_str(
                &format!("Expected '{{' after 'squad', got {:?}", self.peek_token)
            ));
        }
        
        // Parse the struct fields
        let mut fields = Vec::new();
        
        // Move past the '{'
        self.next_token()?;
        
        // Parse fields until closing brace or EOF
        while self.current_token != Token::RBrace && self.current_token != Token::Eof {
            // Parse field name
            if let Token::Identifier(field_name) = &self.current_token {
                let field_token = self.current_token.token_literal();
                let field_id = ast::Identifier {
                    token: field_token.clone(),
                    value: field_name.clone(),
                };
                
                // Next token should be the field type
                self.next_token()?;
                
                // Get the type name - can be an identifier or certain special tokens
                let type_id = if let Token::Identifier(type_name) = &self.current_token {
                    ast::Identifier {
                        token: self.current_token.token_literal(),
                        value: type_name.clone(),
                    }
                } else if let Some(token_value) = self.token_to_type_name() {
                    ast::Identifier {
                        token: self.current_token.token_literal(),
                        value: token_value,
                    }
                } else {
                    return Err(Error::from_str(
                        &format!("Expected field type, got {:?}", self.current_token)
                    ));
                };
                
                // Create field statement
                let field = ast::FieldStatement {
                    token: field_token,
                    name: field_id,
                    type_name: type_id,
                };
                
                fields.push(field);
                
                // Field may be followed by semicolon or newline
                if self.peek_token == Token::Semicolon {
                    self.next_token()?;
                }
                
                // Move to the next field
                self.next_token()?;
            } else {
                return Err(Error::from_str(
                    &format!("Expected field name, got {:?}", self.current_token)
                ));
            }
        }
        
        // Ensure we have a closing brace
        if self.current_token != Token::RBrace {
            return Err(Error::from_str(
                "Expected '}' to close type declaration"
            ));
        }
        
        // Create and return the squad statement
        Ok(Box::new(ast::SquadStatement {
            token,
            name,
            fields,
        }))
    }
    
    /// Parse an interface (collab) declaration
    fn parse_collab_statement(&mut self, token: String, name: ast::Identifier) -> Result<Box<dyn Statement>, Error> {
        // Next token should be opening brace '{'
        if !self.expect_peek(&Token::LBrace) {
            return Err(Error::from_str(
                &format!("Expected '{{' after 'collab', got {:?}", self.peek_token)
            ));
        }
        
        // Parse the interface methods
        let mut methods = Vec::new();
        
        // Move past the '{'
        self.next_token()?;
        
        // Parse methods until closing brace or EOF
        while self.current_token != Token::RBrace && self.current_token != Token::Eof {
            // Parse method name
            if let Token::Identifier(method_name) = &self.current_token {
                let method_token = self.current_token.token_literal();
                let method_id = ast::Identifier {
                    token: method_token.clone(),
                    value: method_name.clone(),
                };
                
                // Next token should be the opening paren for parameters
                if !self.expect_peek(&Token::LParen) {
                    return Err(Error::from_str(
                        &format!("Expected '(' after method name, got {:?}", self.peek_token)
                    ));
                }
                
                // Parse parameters
                let parameters = self.parse_parameters()?;
                
                // Handle optional return type
                let mut return_type = None;
                
                // Check if next token is a return type
                if let Token::Identifier(_) = &self.current_token {
                    // It's a return type
                    if let Token::Identifier(type_name) = &self.current_token {
                        return_type = Some(ast::Identifier {
                            token: self.current_token.token_literal(),
                            value: type_name.clone(),
                        });
                    } else if let Some(token_value) = self.token_to_type_name() {
                        return_type = Some(ast::Identifier {
                            token: self.current_token.token_literal(),
                            value: token_value,
                        });
                    }
                    
                    // Move to the next token
                    self.next_token()?;
                }
                
                // Create method signature
                let method = ast::MethodSignature {
                    token: method_token,
                    name: method_id,
                    parameters,
                    return_type,
                };
                
                methods.push(method);
                
                // Method may be followed by semicolon
                if self.current_token == Token::Semicolon {
                    self.next_token()?;
                }
            } else {
                return Err(Error::from_str(
                    &format!("Expected method name, got {:?}", self.current_token)
                ));
            }
        }
        
        // Ensure we have a closing brace
        if self.current_token != Token::RBrace {
            return Err(Error::from_str(
                "Expected '}' to close interface declaration"
            ));
        }
        
        // Create and return the collab statement
        Ok(Box::new(ast::CollabStatement {
            token,
            name,
            methods,
        }))
    }
    
    /// Parse parameters for a method
    fn parse_parameters(&mut self) -> Result<Vec<ast::ParameterStatement>, Error> {
        let mut parameters = Vec::new();
        
        // Move past the '('
        self.next_token()?;
        
        // Empty parameter list case
        if self.current_token == Token::RParen {
            self.next_token()?;
            return Ok(parameters);
        }
        
        // Parse first parameter
        parameters.push(self.parse_parameter()?);
        
        // Parse remaining parameters
        while self.current_token == Token::Comma {
            self.next_token()?;
            parameters.push(self.parse_parameter()?);
        }
        
        // Expect closing paren
        if self.current_token != Token::RParen {
            return Err(Error::from_str(
                &format!("Expected ')' after parameters, got {:?}", self.current_token)
            ));
        }
        
        // Move past the ')'
        self.next_token()?;
        
        Ok(parameters)
    }
    
    /// Parse a single parameter
    fn parse_parameter(&mut self) -> Result<ast::ParameterStatement, Error> {
        // Parse parameter name
        if let Token::Identifier(param_name) = &self.current_token {
            let param_token = self.current_token.token_literal();
            let param_id = ast::Identifier {
                token: param_token.clone(),
                value: param_name.clone(),
            };
            
            // Next token should be the parameter type
            self.next_token()?;
            
            // Get the type name
            let type_id = if let Token::Identifier(type_name) = &self.current_token {
                ast::Identifier {
                    token: self.current_token.token_literal(),
                    value: type_name.clone(),
                }
            } else if let Some(token_value) = self.token_to_type_name() {
                ast::Identifier {
                    token: self.current_token.token_literal(),
                    value: token_value,
                }
            } else {
                return Err(Error::from_str(
                    &format!("Expected parameter type, got {:?}", self.current_token)
                ));
            };
            
            // Move to the next token
            self.next_token()?;
            
            // Create parameter statement
            let param = ast::ParameterStatement {
                token: param_token,
                name: param_id,
                type_name: type_id,
            };
            
            Ok(param)
        } else {
            Err(Error::from_str(
                &format!("Expected parameter name, got {:?}", self.current_token)
            ))
        }
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
                
                let _curr_token = self.current_token.clone();
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
            Token::Float(value) => {
                let literal = ast::FloatLiteral {
                    token: self.current_token.token_literal(),
                    value: *value,
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
            Token::String(value) => {
                let literal = ast::StringLiteral {
                    token: self.current_token.token_literal(),
                    value: value.clone(),
                };
                
                self.next_token()?;
                Box::new(literal)
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
            Token::Crew => {
                self.parse_array_literal()?
            },
            Token::Tea => {
                self.parse_hash_literal()?
            },
            Token::Stan => {
                self.parse_function_literal()?
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
                Token::Dot => {
                    // Parse property access expression
                    left_expr = self.parse_property_access_expression(left_expr)?;
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
    
    /// Parse a property access expression
    fn parse_property_access_expression(&mut self, object: Box<dyn Expression>) -> Result<Box<dyn Expression>, Error> {
        let token_literal = self.current_token.token_literal(); // Store the literal (".")
        
        // Move past the '.'
        self.next_token()?;
        
        // Parse the property name
        if !self.expect_peek_identifier() {
            return Err(Error::from_str(
                &format!("Expected identifier after '.', got {:?}", self.peek_token)
            ));
        }
        
        let property_name = match &self.current_token {
            Token::Identifier(name) => ast::Identifier {
                token: self.current_token.token_literal(),
                value: name.clone(),
            },
            _ => unreachable!(), // We already checked it's an identifier
        };
        
        // Create the property expression (don't advance token yet)
        Ok(Box::new(ast::PropertyAccessExpression {
            token: token_literal, // Use the stored string literal
            object,
            property: property_name,
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
            Token::Dot => Precedence::Dot,
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
    
    /// Helper to check if the next token is an identifier (any identifier)
    fn expect_peek_identifier(&mut self) -> bool {
        if let Token::Identifier(_) = &self.peek_token {
            match self.next_token() {
                Ok(_) => true,
                Err(_) => false,
            }
        } else {
            self.errors.push(Error::from_str(
                &format!("Expected next token to be an identifier, got {:?}", self.peek_token)
            ));
            false
        }
    }
    
    fn peek_error(&mut self, token: &Token) {
        self.errors.push(Error::from_str(
            &format!("Expected next token to be {:?}, got {:?}", token, self.peek_token)
        ));
    }
    
    /// Helper function to convert certain tokens to type names
    fn token_to_type_name(&self) -> Option<String> {
        match &self.current_token {
            Token::Tea => Some("tea".to_string()),
            Token::Squad => Some("squad".to_string()),
            Token::Collab => Some("collab".to_string()),
            Token::Dm => Some("dm".to_string()),
            Token::Based => Some("lit".to_string()),  // true -> lit (boolean)
            // Add other token types that should be valid as type names
            _ => None,
        }
    }
    
    /// Parse a method declaration
    pub fn parse_method_declaration(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.current_token.token_literal();
        
        // Next token should be the type name (the receiver)
        if !self.expect_peek_identifier() {
            return Err(Error::from_str(
                &format!("Expected identifier after 'slay', got {:?}", self.peek_token)
            ));
        }
        
        // Parse the receiver type
        let receiver_type = match &self.current_token {
            Token::Identifier(type_name) => ast::Identifier {
                token: self.current_token.token_literal(),
                value: type_name.clone(),
            },
            _ => {
                return Err(Error::from_str(
                    &format!("Expected identifier for receiver type, got {:?}", self.current_token)
                ));
            }
        };
        
        // Next token should be colon
        if !self.expect_peek(&Token::Colon) {
            return Err(Error::from_str(
                &format!("Expected ':' after receiver type, got {:?}", self.peek_token)
            ));
        }
        
        // Next token should be the method name
        if !self.expect_peek_identifier() {
            return Err(Error::from_str(
                &format!("Expected method name after ':', got {:?}", self.peek_token)
            ));
        }
        
        // Parse the method name
        let method_name = match &self.current_token {
            Token::Identifier(name) => ast::Identifier {
                token: self.current_token.token_literal(),
                value: name.clone(),
            },
            _ => {
                return Err(Error::from_str(
                    &format!("Expected identifier for method name, got {:?}", self.current_token)
                ));
            }
        };
        
        // Next token should be opening parenthesis
        if !self.expect_peek(&Token::LParen) {
            return Err(Error::from_str(
                &format!("Expected '(' after method name, got {:?}", self.peek_token)
            ));
        }
        
        // Parse parameters
        let parameters = self.parse_parameters()?;
        
        // Optional return type
        let mut return_type = None;
        
        // Check if there's a return type
        if let Token::Identifier(_) = &self.current_token {
            // It's a return type
            if let Token::Identifier(type_name) = &self.current_token {
                return_type = Some(ast::Identifier {
                    token: self.current_token.token_literal(),
                    value: type_name.clone(),
                });
            } else if let Some(token_value) = self.token_to_type_name() {
                return_type = Some(ast::Identifier {
                    token: self.current_token.token_literal(),
                    value: token_value,
                });
            }
            
            // Move to the next token
            self.next_token()?;
        }
        
        // Next token should be opening brace for the body
        if !self.expect_peek(&Token::LBrace) {
            return Err(Error::from_str(
                &format!("Expected '{{' to start method body, got {:?}", self.peek_token)
            ));
        }
        
        // Parse method body
        let body = self.parse_block_statement()?;
        
        // Create and return method declaration
        Ok(Box::new(ast::MethodDeclaration {
            token,
            receiver_type,
            name: method_name,
            parameters,
            return_type,
            body,
        }))
    }
    
    /// Parse an array literal (crew [...] in CURSED)
    fn parse_array_literal(&mut self) -> Result<Box<dyn Expression>, Error> {
        let token = self.current_token.clone(); // Token::Crew

        if !self.expect_peek(&Token::LBracket) {
            return Err(Error::from_str(&format!(
                "Expected '[' after crew, got {:?}",
                self.peek_token
            )));
        }

        let elements = self.parse_expression_list(&Token::RBracket)?;

        Ok(Box::new(ast::ArrayLiteral { token, elements }))
    }

    /// Parse a hash literal (tea {...} in CURSED)
    fn parse_hash_literal(&mut self) -> Result<Box<dyn Expression>, Error> {
        let token = self.current_token.clone(); // Token::Tea
        let mut pairs = Vec::new();

        if !self.expect_peek(&Token::LBrace) {
            return Err(Error::from_str(&format!(
                "Expected '{{' after tea, got {:?}",
                self.peek_token
            )));
        }

        // Move past '{'
        self.next_token()?;

        // Parse key-value pairs
        while self.current_token != Token::RBrace {
            // Parse key
            let key = self.parse_expression(Precedence::Lowest)?;

            // Expect colon
            if !self.expect_peek(&Token::Colon) {
                return Err(Error::from_str(&format!(
                    "Expected ':' after hash key, got {:?}",
                    self.peek_token
                )));
            }

            // Move past ':'
            self.next_token()?;

            // Parse value
            let value = self.parse_expression(Precedence::Lowest)?;

            pairs.push((key, value));

            // Expect comma or closing brace
            if self.current_token != Token::RBrace && !self.expect_peek(&Token::Comma) {
                 return Err(Error::from_str(&format!(
                     "Expected ',' or '}}' after hash pair, got {:?}",
                     self.peek_token
                 )));
            }
            
            // If it was a comma, consume it and continue
            if self.current_token == Token::Comma {
                 self.next_token()?; // Consume ','
                 // Handle trailing comma before '}'
                 if self.current_token == Token::RBrace {
                     break;
                 }
            }
        }

        // Expect closing brace
        if self.current_token != Token::RBrace {
            return Err(Error::from_str(&format!(
                "Expected '}}' to close hash literal, got {:?}",
                self.current_token
            )));
        }

        // Move past '}'
        self.next_token()?;

        Ok(Box::new(ast::HashLiteral { token, pairs }))
    }

    /// Parse function parameters (identifiers only for now)
    fn parse_function_parameters(&mut self) -> Result<Vec<ast::Identifier>, Error> {
        let mut identifiers = Vec::new();

        if !self.expect_peek(&Token::LParen) {
            return Err(Error::from_str(&format!(
                "Expected '(' for function parameters, got {:?}",
                self.peek_token
            )));
        }
        // Move past '('
        self.next_token()?;

        if self.current_token == Token::RParen {
            // No parameters
            self.next_token()?; // Consume ')'
            return Ok(identifiers);
        }

        // Parse first parameter
        match &self.current_token {
            Token::Identifier(name) => {
                identifiers.push(ast::Identifier {
                    token: self.current_token.token_literal(),
                    value: name.clone(),
                });
                self.next_token()?;
            }
            _ => return Err(Error::from_str(&format!(
                "Expected identifier as function parameter, got {:?}",
                self.current_token
            ))),
        }

        // Parse remaining parameters
        while self.current_token == Token::Comma {
            self.next_token()?; // Consume ','
            match &self.current_token {
                Token::Identifier(name) => {
                    identifiers.push(ast::Identifier {
                        token: self.current_token.token_literal(),
                        value: name.clone(),
                    });
                    self.next_token()?;
                }
                _ => return Err(Error::from_str(&format!(
                    "Expected identifier after comma in parameters, got {:?}",
                    self.current_token
                ))),
            }
        }

        // Expect closing parenthesis
        if self.current_token != Token::RParen {
            return Err(Error::from_str(&format!(
                "Expected ')' after function parameters, got {:?}",
                self.current_token
            )));
        }
        self.next_token()?; // Consume ')'

        Ok(identifiers)
    }


    /// Parse a function literal (stan (...) { ... } in CURSED)
    fn parse_function_literal(&mut self) -> Result<Box<dyn Expression>, Error> {
        let token = self.current_token.clone(); // Token::Stan

        let parameters = self.parse_function_parameters()?;

        // Expect opening brace for body
        if !self.expect_peek(&Token::LBrace) {
            return Err(Error::from_str(&format!(
                "Expected '{{' for function body, got {:?}",
                self.peek_token
            )));
        }

        let body = self.parse_block_statement()?;

        // For now, is_variadic is always false
        let is_variadic = false;

        Ok(Box::new(ast::FunctionLiteral {
            token,
            parameters,
            body,
            is_variadic,
        }))
    }

    /// Parse an identifier expression
    fn parse_identifier(&self) -> Result<Box<dyn Expression>, Error> {
        if let Token::Identifier(ref ident) = self.current_token {
            Ok(Box::new(ast::Identifier {
                token: self.current_token.token_literal(),
                value: ident.clone(),
            }))
        } else {
            Err(Error::from_str(&format!("Expected identifier, got {:?}", self.current_token)))
        }
    }
    
    /// Parse an integer literal expression
    fn parse_integer_literal(&self) -> Result<Box<dyn Expression>, Error> {
        if let Token::Int(value) = self.current_token {
            Ok(Box::new(ast::IntegerLiteral {
                token: self.current_token.token_literal(),
                value,
            }))
        } else {
            Err(Error::from_str(&format!("Expected integer literal, got {:?}", self.current_token)))
        }
    }
    
    /// Parse a float literal expression
    fn parse_float_literal(&self) -> Result<Box<dyn Expression>, Error> {
        if let Token::Float(value) = self.current_token {
            Ok(Box::new(ast::FloatLiteral {
                token: self.current_token.token_literal(),
                value,
            }))
        } else {
            Err(Error::from_str(&format!("Expected float literal, got {:?}", self.current_token)))
        }
    }
    
    /// Parse a boolean literal expression
    fn parse_boolean_literal(&self) -> Result<Box<dyn Expression>, Error> {
        if let Token::Based = self.current_token {
            Ok(Box::new(ast::BooleanLiteral {
                token: self.current_token.token_literal(),
                value: true,
            }))
        } else if let Token::Cap = self.current_token {
            Ok(Box::new(ast::BooleanLiteral {
                token: self.current_token.token_literal(),
                value: false,
            }))
        } else {
            Err(Error::from_str(&format!("Expected boolean literal (based/cap), got {:?}", self.current_token)))
        }
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
            "periodt based { x = x + 1; }",
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