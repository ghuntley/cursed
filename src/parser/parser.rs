use crate::ast::{self, Program, Statement, Expression};
use crate::error::{Error, SourceLocation};
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
    /// Parse a stan expression (goroutine)
    fn parse_stan_expression(&mut self) -> Result<Box<dyn Expression>, Error> {
        let token = self.current_token.clone(); // Token::Stan
        self.next_token()?; // Move past 'stan'
        
        // Parse the expression to be executed as a goroutine
        // This will typically be a function call
        let expression = self.parse_expression(Precedence::Lowest)?;
        
        Ok(Box::new(ast::StanExpression {
            token: token.token_literal(),
            expression,
        }))
    }
    /// Helper method to provide debug information about current parser state
    fn parser_state_debug(&self) -> String {
        let mut info = String::new();
        info.push_str(&format!("Parser state:\n"));
        info.push_str(&format!("  Position: line {}, column {}\n", self.lexer.line, self.lexer.column));
        info.push_str(&format!("  Current token: {:?}\n", self.current_token));
        info.push_str(&format!("  Next token: {:?}\n", self.peek_token));
        
        // Add token history if available
        // (in a more complete implementation we could keep a token history buffer)
        
        info
    }
    
    /// Helper method to get a snippet of source code around the current position with line number
    fn get_source_snippet_with_line(&self) -> (String, usize) {
        // Get the current position in the input
        let pos = self.lexer.position;
        let input = self.lexer.input;
        let line = self.lexer.line;
        
        // Find the start of the current line
        let mut line_start = pos;
        while line_start > 0 && input.as_bytes().get(line_start - 1).map_or(false, |&c| c != b'\n') {
            line_start -= 1;
        }
        
        // Find the end of the current line
        let mut line_end = pos;
        while line_end < input.len() && input.as_bytes().get(line_end).map_or(false, |&c| c != b'\n') {
            line_end += 1;
        }
        
        // Extract the line
        if line_start < line_end {
            let line_content = &input[line_start..line_end];
            
            // Calculate column for the caret
            let caret_pos = self.lexer.column - 1; // Convert to 0-based
            
            // Add the caret pointing at the error position
            let mut result = line_content.to_string();
            result.push_str("\n     |");
            result.push_str(&" ".repeat(caret_pos));
            result.push_str("^"); // Red caret would go here
            
            (result, line)
        } else {
            (String::new(), line)
        }
    }
    
    /// Helper method to get a snippet of source code around the current position
    fn get_source_snippet(&self) -> String {
        // Get the current position in the input
        let pos = self.lexer.position;
        let input = self.lexer.input;
        
        // Get a snippet of context (30 chars before and after if possible)
        let start = pos.saturating_sub(30);
        let end = (pos + 30).min(input.len());
        
        if start < end {
            // Add markers around the current position
            let before = &input[start..pos];
            let after = if pos < input.len() {
                &input[pos..end]
            } else {
                ""
            };
            
            // Highlight the position with markers
            format!("{}{}{}{}{}", before, "[31m", "^", "[0m", after)
        } else {
            // Return empty string if we can't get a good snippet
            String::new()
        }
    }
    
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
                Err(e) => {
                    // Enhanced error reporting
                    let (snippet, line) = self.get_source_snippet_with_line();
                    println!("Parser error at line {}: {}", line, e);
                    if !snippet.is_empty() {
                        println!("{}", snippet);
                    }
                    println!("Current token: {:?}, Next token: {:?}", self.current_token, self.peek_token);
                    self.errors.push(e);
                }
            }
            
            // Advance to the next token
            match self.next_token() {
                Ok(_) => {},
                Err(e) => {
                    println!("Error advancing token: {}", e);
                    self.errors.push(e);
                }
            }
        }
        
        if !self.errors.is_empty() {
            // If we had parsing errors, print them in Rust compiler style
            if self.errors.len() > 1 {
                println!("error: aborting due to {} previous errors\n", self.errors.len());
            }
            
            // Display debugging information about the parser state
            println!("Parser state debug information:");
            println!("{}", self.parser_state_debug());
            
            // Display a summary similar to Rust compiler
            println!("error: could not parse CURSED code successfully");
            println!("note: check the error details above for more information");
            
            // Return the first error
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
            Token::Ghosted => self.parse_break_statement(),
            Token::Later => self.parse_later_statement(),
            Token::Slay => {
                // Check if this is a method declaration (look ahead for colon)
                // First save the current position
                let current_token = self.current_token.clone();
                let peek_token = self.peek_token.clone();
                
                // Look ahead to see if this is a method declaration
                let is_method_declaration = {
                    // Move past 'slay'
                    if !self.expect_peek_any_identifier() {
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
                    // This is a regular function declaration or a function expression
                    // Determine if this is a function declaration
                    let function_token = self.current_token.token_literal();
                    
                    // Move past 'slay'
                    self.next_token()?;
                    
                    // Check if we have an identifier followed by params or generics
                    if let Token::Identifier(name) = &self.current_token {
                        // This is a function declaration
                        let func_name = ast::Identifier {
                            token: self.current_token.token_literal(),
                            value: name.clone(),
                        };
                        
                        // Move past the function name
                        self.next_token()?;
                        
                        // Check for optional type parameters (e.g. [T] or [A, B])
                        // In CURSED we use the syntax: slay function_name[T](x T) T
                        let mut type_parameters = Vec::new();
                        
                        // Check for a left bracket immediately after the function name
                        if self.current_token == Token::LBracket {
                            // Parse the type parameters
                            type_parameters = self.parse_type_parameters()?
                        }
                        
                        // The next token after function name or type parameters should be '('
                        if self.current_token != Token::LParen {
                            return Err(Error::from_str(
                                &format!("Expected '(' after function name or type parameters, got {:?}", self.current_token)
                            ));
                        }
                        
                        // Parse parameter names with their types
                        // For a typed parameter function like: slay identity[T](x T) T
                        let parameters = self.parse_typed_function_parameters()?.iter()
                            .map(|param| param.name.clone())
                            .collect();
                        
                        // Check for optional return type
                        let mut return_type = None;
                        
                        // Check if we're at the opening brace or need to handle return type
                        if self.current_token != Token::LBrace && !self.current_token.token_literal().is_empty() {
                            if let Token::Identifier(type_name) = &self.current_token {
                                return_type = Some(ast::Identifier {
                                    token: self.current_token.token_literal(),
                                    value: type_name.clone(),
                                });
                                
                                // Move past the return type
                                self.next_token()?;
                            } else if let Some(token_value) = self.token_to_type_name() {
                                return_type = Some(ast::Identifier {
                                    token: self.current_token.token_literal(),
                                    value: token_value,
                                });
                                
                                // Move past the return type
                                self.next_token()?;
                            }
                        }
                        
                        // Expect an opening brace for the function body
                        if self.current_token != Token::LBrace {
                            // Check if we're looking at a valid type token - then it must be a return type
                            if let Token::Identifier(_) = &self.current_token {
                                // It's a valid identifier - likely a return type
                                // Don't return error, return types are handled above
                            } else if self.token_to_type_name().is_none() {
                                // Not a valid type and not an opening brace - syntax error
                                return Err(Error::from_str(
                                    &format!("Expected '{{' or return type after function parameters, got {:?}", self.current_token)
                                ));
                            }
                        }
                        
                        // Parse body
                        let body = self.parse_block_statement()?;
                        
                        // Create expression statement with function literal
                        let func_expr = ast::FunctionLiteral {
                            token: Token::Slay, // Use Slay token
                            type_parameters,
                            parameters, 
                            body,
                            is_variadic: false,
                            return_type,
                        };
                        
                        // Create assignment expression
                        let assign_expr = ast::AssignmentExpression {
                            token: "=".to_string(),
                            name: func_name,
                            value: Box::new(func_expr),
                        };
                        
                        return Ok(Box::new(ast::ExpressionStatement {
                            token: function_token,
                            expression: Some(Box::new(assign_expr)),
                        }));
                    } else {
                        // Not a function declaration, treat as expression
                        self.parse_expression_statement()
                    }
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
        if !self.expect_peek_any_identifier() {
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
        
        // Expect a semicolon (optional in some contexts since statements can be separated by newlines)
        if self.peek_token == Token::Semicolon {
            self.next_token()?; // Consume the semicolon
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
        
        // Check for optional type annotation (smol, mid, normie, thicc, byte, rune, etc.)
        let mut type_annotation = None;
        if matches!(self.peek_token, Token::Smol | Token::Mid | Token::Normie | Token::Thicc) {
            self.next_token()?; // Consume the type token
            type_annotation = Some(self.current_token.clone());
        } else if let Token::Identifier(type_name) = &self.peek_token.clone() {
            // Check if the identifier is a known type name (byte, rune, etc.)
            let is_type = type_name == "byte" || type_name == "rune";
            if is_type {
                let type_name_str = type_name.clone();  // Clone to avoid borrowing issues
                self.next_token()?; // Consume the type token
                type_annotation = Some(self.current_token.clone());
                println!("DEBUG: Found type annotation: {} for variable {}", type_name_str, name.value);
            }
        }
        
        // Expect the assignment operator
        if !self.expect_peek(&Token::Assign) {
            return Err(Error::from_str(
                &format!("Expected '=' after identifier in sus statement, got {:?}. Current token: {:?}, current position: variable '{}', type annotation: {:?}", 
                    self.peek_token, self.current_token, name.value, type_annotation)
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
    
    /// Parse a break statement
    pub fn parse_break_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.current_token.token_literal();
        
        // Move past the 'ghosted' token
        self.next_token()?;
        
        // Optionally consume a semicolon, but don't advance past it as that will be done
        // by the calling function
        
        // Create and return the BreakStatement
        Ok(Box::new(ast::BreakStatement {
            token,
        }))
    }
    
    /// Parse a later statement (defer)
    pub fn parse_later_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.current_token.token_literal();
        
        // Move past the 'later' token
        self.next_token()?;
        
        // Parse the expression to be deferred
        let expression = self.parse_expression(Precedence::Lowest)?;
        
        // Optionally consume a semicolon
        if self.peek_token == Token::Semicolon {
            self.next_token()?;
        }
        
        // Create and return the LaterStatement
        Ok(Box::new(ast::LaterStatement {
            token,
            expression,
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
            // Get previous token for better error context
            let previous_token = format!("{:?}", self.peek_token);
            
            return Err(Error::from_str(
                &format!("Expected '}}' to close vibe_check statement, got {:?}, previous token was {}", 
                    self.current_token, previous_token)
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
        if !self.expect_peek_any_identifier() {
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
        
        // Check for optional type parameters [T] or [A, B, C]
        let mut type_parameters = Vec::new();
        
        // Next token should be '[' for type parameters or 'squad'/'collab' otherwise
        self.next_token()?;
        
        // If current token is '[', parse type parameters
        if self.current_token == Token::LBracket {
            type_parameters = self.parse_type_parameters()?;
        }
        
        // Now the current token should be 'squad' or 'collab'
        match &self.current_token {
            Token::Squad => self.parse_squad_statement(token, name, type_parameters),
            Token::Collab => self.parse_collab_statement(token, name, type_parameters),
            _ => Err(Error::from_str(
                &format!("Expected 'squad' or 'collab' after type name, got {:?}", self.current_token)
            ))
        }
    }
    
    /// Parse a struct (squad) declaration
    fn parse_squad_statement(&mut self, token: String, name: ast::Identifier, type_parameters: Vec<ast::Identifier>) -> Result<Box<dyn Statement>, Error> {
        // Store squad token for error reporting
        let squad_token = self.current_token.token_literal();
        
        // Note: type_parameters is now passed as an argument instead of parsed here
        
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
            // Get previous token for better error context
            let previous_token = format!("{:?}", self.peek_token);
            
            return Err(Error::from_str(
                &format!("Expected '}}' to close struct (squad) declaration, got {:?}, previous token was {}", 
                    self.current_token, previous_token)
            ));
        }
        
        // Create and return the squad statement
        Ok(Box::new(ast::SquadStatement {
            token,
            name,
            type_parameters,
            fields,
        }))
    }
    
    /// Parse an interface (collab) declaration
    fn parse_collab_statement(&mut self, token: String, name: ast::Identifier, type_parameters: Vec<ast::Identifier>) -> Result<Box<dyn Statement>, Error> {
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
            // Get previous token for better error context
            let previous_token = format!("{:?}", self.peek_token);
            
            return Err(Error::from_str(
                &format!("Expected '}}' to close interface declaration, got {:?}, previous token was {}", 
                    self.current_token, previous_token)
            ));
        }
        
        // Create and return the collab statement
        Ok(Box::new(ast::CollabStatement {
            token,
            name,
            type_parameters,
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
        
        // Check for optional parentheses
        let has_parens = self.current_token == Token::LParen;
        
        // If there's an opening parenthesis, consume it
        if has_parens {
            // Move past '('
            self.next_token()?;
        }
        
        // Parse the condition
        let condition = self.parse_expression(Precedence::Lowest)?;
        
        // If we had an opening parenthesis, expect and consume a closing one
        if has_parens {
            if self.current_token != Token::RParen {
                return Err(Error::from_str(
                    &format!("Expected ')' after condition in periodt statement, got {:?}", self.current_token)
                ));
            }
            self.next_token()?; // Consume the closing parenthesis
        }
        
        // Check if the current token is already a '{'
        if self.current_token == Token::LBrace {
            // Parse the body
            let body = self.parse_block_statement()?;
            
            return Ok(Box::new(ast::WhileStatement {
                token,
                condition,
                body,
            }));
        }
        
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
            // Include token info in error message for better debugging
            // Get previous token for better error context
            let previous_token = format!("{:?}", self.peek_token);
            
            return Err(Error::from_str(
                &format!("Expected '}}' to close block statement, got {:?}, previous token was {}", 
                    self.current_token, previous_token)
            ));
        }
        
        Ok(ast::BlockStatement {
            token,
            statements,
        })
    }
    
    /// Parse an expression with the given precedence
    /// This handles all expressions, including identifiers, literals, and complex expressions
    /// It also handles generic function calls like `identity[T](x)` by detecting the `[T]` syntax
    fn parse_expression(&mut self, precedence: Precedence) -> Result<Box<dyn Expression>, Error> {
        // Get current location for error reporting
        let location = SourceLocation::new(self.lexer.line, self.lexer.column);
        
        // We could log parsing attempts when a special debug flag is set
        // But for normal operation, we'll keep it quiet
        // println!("DEBUG: Parsing expression with token: {:?} at line {}, column {}", 
        //          self.current_token, self.lexer.line, self.lexer.column);
        
        // First try to find a prefix parsing function
        let mut left_expr: Box<dyn Expression> = match &self.current_token {
            Token::Arrow => {
                // Receive expression (<-ch)
                self.parse_receive_expression()?
            },
            Token::Dm => {
                // Channel type (dm<T>)
                self.parse_channel_expression()?
            },
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
                
                // Check for generic type arguments
                if self.current_token == Token::LBracket {
                    // This might be generic type arguments for a function call
                    // e.g., identity[T](x)
                    
                    // Clone the identifier before we move past it
                    let ident_clone = identifier.clone();
                    
                    // Try to parse the generic type arguments
                    let result = self.parse_type_arguments();
                    
                    if let Ok(type_args) = result {
                        // Successfully parsed type arguments
                        // Check if we're followed by a function call - '('
                        if self.current_token == Token::LParen {
                            // This is a generic function call
                            let token = self.current_token.clone(); // LParen token
                            let arguments = self.parse_expression_list(&Token::RParen)?;
                            
                            return Ok(Box::new(ast::GenericCallExpression {
                                token,
                                function: Box::new(ident_clone),
                                type_arguments: type_args,
                                arguments,
                            }));
                        }
                    }
                    
                    // If we get here, we either couldn't parse type arguments or
                    // there was no opening parenthesis for arguments afterward.
                    // Since we've already consumed tokens, we'll just return the identifier
                    return Ok(Box::new(ident_clone));
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
            Token::Byte(value) => {
                let literal = ast::ByteLiteral {
                    token: self.current_token.token_literal(),
                    value: *value,
                };
                
                self.next_token()?;
                Box::new(literal)
            },
            Token::Rune(value) => {
                let literal = ast::RuneLiteral {
                    token: self.current_token.token_literal(),
                    value: *value,
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
            Token::BeLike => {
                self.parse_be_like_expression()?                
            },
            Token::Stan => {
                self.parse_stan_expression()?
            },
            _ => {
                // Get a snippet of the input around the error location with line number
                let (snippet, line_num) = self.get_source_snippet_with_line();
                let source_context = if !snippet.is_empty() {
                    format!("\n  --> {}:{}\n     |\n{:4} | {}", 
                           "input", self.lexer.line, self.lexer.line, snippet)
                } else {
                    String::new()
                };
                
                // Create a user-friendly error message in Rust compiler style
                // Store previous token for error message
                let previous_token = format!("{:?}", self.peek_token);
                
                // More detailed error message based on the token type
                let expression_tips = match &self.current_token {
                    Token::Assign => "Assignment is not allowed in this context. Did you mean to use ':=' for declaration?",
                    Token::Semicolon => "Unexpected semicolon. Did you forget an expression?",
                    Token::RBrace => "Unexpected closing brace. Did you forget to finish an expression?",
                    Token::RParen => "Unexpected closing parenthesis. Check your parentheses matching.",
                    Token::RBracket => "Unexpected closing bracket. Check your bracket matching.",
                    Token::Eof => "Unexpected end of file. Your expression or statement is incomplete.",
                    _ => "This token cannot start an expression in CURSED. Did you mean to use a different expression or statement?"
                };
                
                let message = format!(
                    "error[E0003]: unexpected token {:?}, previous token was {}{}\n\nSource context: {}\n\nhelp: {}", 
                    self.current_token,
                    previous_token,
                    source_context,
                    format!("Line: {}, Column: {}", self.lexer.line, self.lexer.column),
                    expression_tips
                );
                
                // Print the error for immediate debugging
                println!("{}\n", message);
                
                return Err(Error::Parser {
                    location: SourceLocation::new(self.lexer.line, self.lexer.column),
                    message,
                });
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
                Token::Arrow => {
                    // Parse send expression (ch <- value)
                    left_expr = self.parse_send_expression(left_expr)?;
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
    
    /// Parse a call expression or type conversion
    fn parse_call_expression(&mut self, function: Box<dyn Expression>) -> Result<Box<dyn Expression>, Error> {
        let token = self.current_token.clone();
        
        // Parse the arguments
        let mut arguments = self.parse_expression_list(&Token::RParen)?;
        
        // Check if this is a type conversion
        if let Some(ident) = function.as_any().downcast_ref::<ast::Identifier>() {
            // Check if the identifier is a type name (smol, mid, normie, thicc, snack, meal)
            let is_type_name = matches!(ident.value.as_str(), 
                  "smol" | "mid" | "normie" | "thicc" | "snack" | "meal" | 
                  "byte" | "rune" | "tea" | "lit");
            
            // If this is a type name and we have exactly one argument, this is a type conversion
            if is_type_name && arguments.len() == 1 {
                return Ok(Box::new(ast::TypeConversionExpression {
                    token: ident.token.clone(),
                    type_name: ident.value.clone(),
                    expression: arguments.remove(0),
                }));
            }
        }
        
        // Otherwise, this is a regular function call
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
            Token::Arrow => Precedence::Sum, // Same precedence as + and -
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
                Err(e) => {
                    // Log the error for better debugging
                    println!("PARSER ERROR: Failed to advance token: {}", e);
                    false
                },
            }
        } else {
            // Get location from the lexer if possible
            let location = SourceLocation::new(self.lexer.line, self.lexer.column);
            
            // Get a snippet of source code around the current position with line number
            let (snippet, line_num) = self.get_source_snippet_with_line();
            let source_context = if !snippet.is_empty() {
                format!("\n  --> {}:{}\n     |\n{:4} | {}", 
                       "input", self.lexer.line, self.lexer.line, snippet)
            } else {
                String::new()
            };
            
            // Create detailed error message
            let context = format!("Current token: {:?}, Next token: {:?}, at line {}, column {}", 
                                  self.current_token, 
                                  self.peek_token, 
                                  self.lexer.line, 
                                  self.lexer.column);
            // Add syntax guidance based on the expected token
            let syntax_tip = match token {
                Token::LParen => "Function calls and conditions require parentheses: main() or lowkey (x < 5)",
                Token::RParen => "Missing closing parenthesis: main() or lowkey (x < 5)",
                Token::LBrace => "Block statements require braces: slay main() { ... }",
                Token::RBrace => "Missing closing brace for a block",
                Token::Semicolon => "Statements must end with semicolons: vibe main; or sus x = 5;",
                _ => "Make sure your syntax follows the CURSED language specification"
            };
            
            let message = format!("error[E0001]: expected token {:?}, found {:?}{}\n\nContext: {}\n\nhelp: {}", 
                                 token, 
                                 self.peek_token,
                                 source_context,
                                 context,
                                 syntax_tip);
            
            // Print error for immediate debugging
            println!("{}\n", message);
            
            self.errors.push(Error::Parser {
                location,
                message
            });
            false
        }
    }
    
    /// Helper to check if the next token is an identifier (any identifier)
    fn expect_peek_identifier(&mut self) -> bool {
        if let Token::Identifier(_) = &self.peek_token {
            match self.next_token() {
                Ok(_) => true,
                Err(e) => {
                    // Log the error for better debugging
                    println!("PARSER ERROR: Failed to advance token: {}", e);
                    false
                },
            }
        } else {
            // Get location from the lexer if possible
            let location = SourceLocation::new(self.lexer.line, self.lexer.column);
            
            // Get a snippet of source code around the current position with line number
            let (snippet, line_num) = self.get_source_snippet_with_line();
            let source_context = if !snippet.is_empty() {
                format!("\n  --> {}:{}\n     |\n{:4} | {}", 
                       "input", self.lexer.line, self.lexer.line, snippet)
            } else {
                String::new()
            };
            
            // Create detailed error message in a Rust-like style
            let context = format!("Current token: {:?}, Peek token: {:?}, at line {}, column {}", 
                                  self.current_token, 
                                  self.peek_token, 
                                  self.lexer.line, 
                                  self.lexer.column);
                                  
            // Add specific guidance for identifiers
            let identifier_tips = match &self.peek_token {
                Token::Int(_) => "Identifiers cannot start with numbers. Try adding a letter prefix.",
                Token::String(_) => "Strings cannot be used as identifiers. Try removing the quotes.",
                Token::LParen => "Function parameters need names. Example: slay main(x, y) { ... }",
                Token::Semicolon => "Missing identifier before semicolon. Example: sus name = value;",
                _ => "Variable or field names must be identifiers"
            };
            
            let message = format!("error[E0002]: expected identifier, found {:?}{}\n\nContext: {}\n\nhelp: {}", 
                                self.peek_token,
                                source_context,
                                context,
                                identifier_tips);
            
            // Print error for immediate debugging
            println!("{}\n", message);
            
            self.errors.push(Error::Parser {
                location,
                message
            });
            false
        }
    }
    
    /// Parse typed function parameters, which include parameter name and type
    /// For example: "x normie, y tea"
    fn parse_typed_function_parameters(&mut self) -> Result<Vec<ast::ParameterStatement>, Error> {
        let mut parameters = Vec::new();
        
        // The current token should already be '('
        
        // Move past '('
        self.next_token()?;

        if self.current_token == Token::RParen {
            // No parameters
            self.next_token()?; // Consume ')'
            return Ok(parameters);
        }

        // Parse first parameter
        if let Token::Identifier(param_name) = &self.current_token.clone() {
            let name = ast::Identifier {
                token: self.current_token.token_literal(),
                value: param_name.clone(),
            };
            
            // Move past parameter name
            self.next_token()?;
            
            // Expect parameter type
            let type_name = if let Token::Identifier(type_name) = &self.current_token {
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
            
            // Create parameter statement
            parameters.push(ast::ParameterStatement {
                token: name.token.clone(),
                name,
                type_name,
            });
            
            // Move past type name
            self.next_token()?;
        }

        // Parse remaining parameters
        while self.current_token == Token::Comma {
            self.next_token()?; // Consume ','
            
            // Parse parameter name
            if let Token::Identifier(param_name) = &self.current_token.clone() {
                let name = ast::Identifier {
                    token: self.current_token.token_literal(),
                    value: param_name.clone(),
                };
                
                // Move past parameter name
                self.next_token()?;
                
                // Expect parameter type
                let type_name = if let Token::Identifier(type_name) = &self.current_token {
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
                
                // Create parameter statement
                parameters.push(ast::ParameterStatement {
                    token: name.token.clone(),
                    name,
                    type_name,
                });
                
                // Move past type name
                self.next_token()?;
            } else {
                return Err(Error::from_str(
                    &format!("Expected parameter name after comma, got {:?}", self.current_token)
                ));
            }
        }

        // Expect closing parenthesis
        if self.current_token != Token::RParen {
            return Err(Error::from_str(
                &format!("Expected ')' after parameters, got {:?}", self.current_token)
            ));
        }
        self.next_token()?; // Consume ')'

        Ok(parameters)
    }

    /// Helper to check if the next token is any valid identifier without checking specific values
    fn expect_peek_any_identifier(&mut self) -> bool {
        if let Token::Identifier(_) = &self.peek_token {
            match self.next_token() {
                Ok(_) => true,
                Err(e) => {
                    // Log the error for better debugging
                    println!("PARSER ERROR: Failed to advance token: {}", e);
                    false
                },
            }
        } else {
            // Get location from the lexer if possible
            let location = SourceLocation::new(self.lexer.line, self.lexer.column);
            
            // Get a snippet of source code around the current position with line number
            let (snippet, line_num) = self.get_source_snippet_with_line();
            let source_context = if !snippet.is_empty() {
                format!("\n  --> {}:{}\n     |\n{:4} | {}", 
                       "input", self.lexer.line, self.lexer.line, snippet)
            } else {
                String::new()
            };
            
            // Create detailed error message in a Rust-like style
            let message = format!("error[E0002]: expected identifier, found {:?}{}\n\nhelp: Variable or field names must be identifiers", 
                                self.peek_token,
                                source_context);
            
            // Print error for immediate debugging
            println!("{}", message);
            
            self.errors.push(Error::Parser {
                location,
                message
            });
            false
        }
    }
    
    fn peek_error(&mut self, token: &Token) {
        // Add more context to the error message
        let context = format!("Current token: {:?}, Next token: {:?}", self.current_token, self.peek_token);
        let message = format!("Expected next token to be {:?}, got {:?}", token, self.peek_token);
        
        self.errors.push(Error::Parser {
            location: SourceLocation::default(),
            message: format!("{} - Context: {}", message, context)
        });
    }
    
    /// Parse a channel expression (dm<T> or dm<T>[capacity])
    fn parse_channel_expression(&mut self) -> Result<Box<dyn Expression>, Error> {
        let token = self.current_token.token_literal();
        
        // Expect element type after 'dm'
        self.next_token()?;
        
        // Parse the element type
        let element_type = if let Some(token_value) = self.token_to_type_name() {
            // Create an identifier for the type
            let type_ident = ast::Identifier {
                token: self.current_token.token_literal(),
                value: token_value,
            };
            Box::new(type_ident) as Box<dyn Expression>
        } else {
            // If it's not a built-in type, parse it as a regular expression
            self.parse_expression(Precedence::Lowest)?
        };
        
        // Move past the element type
        if self.current_token != Token::Eof {
            self.next_token()?;
        }
        
        // Check for optional capacity specification with []  
        let capacity = if self.current_token == Token::LBracket {
            // Move past '['
            self.next_token()?;
            
            // Parse the capacity expression
            let cap_expr = self.parse_expression(Precedence::Lowest)?;
            
            // Expect closing ']'
            if !self.expect_peek(&Token::RBracket) {
                return Err(Error::Parser {
                    location: SourceLocation::default(),
                    message: format!("Expected closing ']' after channel capacity")
                });
            }
            
            Some(cap_expr)
        } else {
            None
        };
        
        Ok(Box::new(ast::ChannelExpression {
            token,
            element_type,
            capacity,
        }))
    }

    /// Parse a send expression (ch <- value)
    fn parse_send_expression(&mut self, channel: Box<dyn Expression>) -> Result<Box<dyn Expression>, Error> {
        let token = self.current_token.token_literal(); // Should be '<-'
        
        // Move past the '<-' token
        self.next_token()?;
        
        // Parse the value to send
        let value = self.parse_expression(Precedence::Lowest)?;
        
        Ok(Box::new(ast::SendExpression {
            token,
            channel,
            value,
        }))
    }

    /// Parse a receive expression (<-ch)
    fn parse_receive_expression(&mut self) -> Result<Box<dyn Expression>, Error> {
        let token = self.current_token.token_literal(); // Should be '<-'
        
        // Move past the '<-' token
        self.next_token()?;
        
        // Parse the channel expression
        let channel = self.parse_expression(Precedence::Lowest)?;
        
        Ok(Box::new(ast::ReceiveExpression {
            token,
            channel,
        }))
    }

    /// Helper function to convert certain tokens to type names
    fn token_to_type_name(&self) -> Option<String> {
        match &self.current_token {
            // String type
            Token::Tea => Some("tea".to_string()),
            // Struct
            Token::Squad => Some("squad".to_string()),
            // Interface
            Token::Collab => Some("collab".to_string()),
            // Channels
            Token::Dm => Some("dm".to_string()),
            // Boolean (lit type)
            Token::Based => Some("lit".to_string()),  // true -> lit (boolean)
            // Integer types
            Token::Smol => Some("smol".to_string()), // int8
            Token::Mid => Some("mid".to_string()),   // int16  
            Token::Normie => Some("normie".to_string()), // int32
            Token::Thicc => Some("thicc".to_string()), // int64
            // Float types (identified as snack/meal in CURSED)
            Token::Identifier(s) if s == "snack" => Some("snack".to_string()), // float32
            Token::Identifier(s) if s == "meal" => Some("meal".to_string()),  // float64
            // Byte and Rune types
            Token::Identifier(s) if s == "byte" => Some("byte".to_string()),
            Token::Identifier(s) if s == "rune" => Some("rune".to_string()),
            // Lit (boolean) type identifier
            Token::Identifier(s) if s == "lit" => Some("lit".to_string()),
            // Array or slice types
            Token::LBracket => {
                // Try to get the original text for this token
                let start_pos = self.lexer.position - 1; // Adjust for the current position
                let mut type_str = "[".to_string();
                if start_pos < self.lexer.input.len() {
                    let bracket_index = start_pos;
                    let mut lookahead = bracket_index + 1;
                    
                    // Look ahead to check if it's a slice [] or array [N]
                    while lookahead < self.lexer.input.len() {
                        let ch = self.lexer.input.chars().nth(lookahead);
                        if let Some(c) = ch {
                            if c == ']' {
                                type_str.push(c);
                                // It could be a slice or an array, return it as is
                                return Some(type_str);
                            } else {
                                type_str.push(c);
                            }
                        }
                        lookahead += 1;
                    }
                }
                None // Couldn't determine the type
            },
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
    
    /// Parse a struct instantiation expression (be_like Name[T] { field: value })
    fn parse_be_like_expression(&mut self) -> Result<Box<dyn Expression>, Error> {
        let token = self.current_token.token_literal();
        
        // Move past 'be_like'
        self.next_token()?;
        
        // Expect the struct name as an identifier
        if let Token::Identifier(name) = &self.current_token {
            let struct_name = ast::Identifier {
                token: self.current_token.token_literal(),
                value: name.clone(),
            };
            
            // Move past the struct name
            self.next_token()?;
            
            // Check for optional type arguments [T] or [A, B, C]
            let mut type_arguments = Vec::new();
            
            // If current token is '[', parse type arguments
            if self.current_token == Token::LBracket {
                type_arguments = self.parse_type_arguments()?;
            }
            
            // Expect opening brace '{' for struct fields
            if self.current_token != Token::LBrace {
                return Err(Error::from_str(
                    &format!("Expected '{{' after struct name, got {:?}", self.current_token)
                ));
            }
            
            // Parse struct fields as key-value pairs
            let mut fields = Vec::new();
            
            // Move past '{'
            self.next_token()?;
            
            // Empty struct case
            if self.current_token == Token::RBrace {
                self.next_token()?; // Move past '}'
                
                return Ok(Box::new(ast::BeLikeExpression {
                    token: token.to_string(),
                    struct_name,
                    type_arguments,
                    fields,
                }));
            }
            
            // Parse fields until closing brace
            loop {
                // Parse field name
                if let Token::Identifier(field_name) = &self.current_token {
                    let name = field_name.clone();
                    
                    // Expect colon after field name
                    if !self.expect_peek(&Token::Colon) {
                        return Err(Error::from_str(
                            &format!("Expected ':' after field name, got {:?}", self.peek_token)
                        ));
                    }
                    
                    // Move past the colon
                    self.next_token()?;
                    
                    // Parse the field value
                    let value = self.parse_expression(Precedence::Lowest)?;
                    
                    // Add the field
                    fields.push((name, value));
                    
                    // If next token is comma, continue parsing fields
                    if self.current_token == Token::Comma {
                        self.next_token()?;
                        continue;
                    }
                    
                    // If next token is closing brace, we're done
                    if self.current_token == Token::RBrace {
                        break;
                    }
                    
                    // Otherwise, expect a comma
                    return Err(Error::from_str(
                        &format!("Expected ',' or '}}' after field, got {:?}", self.current_token)
                    ));
                } else {
                    return Err(Error::from_str(
                        &format!("Expected field name, got {:?}", self.current_token)
                    ));
                }
            }
            
            // Move past the '}}'
            self.next_token()?;
            
            Ok(Box::new(ast::BeLikeExpression {
                token: token.to_string(),
                struct_name,
                type_arguments,
                fields,
            }))
        } else {
            Err(Error::from_str(
                &format!("Expected struct name after 'be_like', got {:?}", self.current_token)
            ))
        }
    }

    /// Parse function parameters, including type annotations
    /// This method parses parameters in the form "name type" or just "name"
    /// For example: "x normie, y tea"
    /// It also handles complex types like function types: "f slay(X) Y"
    /// For generic function declarations with type parameters like: identity[T](x T) T
    fn parse_function_parameters(&mut self) -> Result<Vec<ast::Identifier>, Error> {
        let mut identifiers = Vec::new();

        // The current token should already be '('
        
        // Move past '('
        self.next_token()?;

        if self.current_token == Token::RParen {
            // No parameters
            self.next_token()?; // Consume ')'
            return Ok(identifiers);
        }

        // Parse first parameter
        if let Token::Identifier(name) = &self.current_token.clone() {
            let param_name = ast::Identifier {
                token: self.current_token.token_literal(),
                value: name.clone(),
            };
            identifiers.push(param_name);
            
            // Move past the parameter name
            self.next_token()?;
            
            // Skip the type annotation if present
            // We're only collecting parameter names, not full parameter declarations
            if self.current_token != Token::Comma && self.current_token != Token::RParen {
                // Handle complex type cases like "f slay(A) B" (function types)
                if self.current_token == Token::Slay {
                    // Skip until we reach a comma or closing paren
                    let mut depth = 0;
                    
                    while self.current_token != Token::Comma && 
                          self.current_token != Token::RParen &&
                          self.current_token != Token::Eof {
                        
                        // Track nested parentheses to handle function types properly
                        if self.current_token == Token::LParen {
                            depth += 1;
                        } else if self.current_token == Token::RParen {
                            depth -= 1;
                            if depth < 0 && !matches!(self.peek_token, Token::Identifier(_)) {
                                // We've reached the end of the parameter list
                                break;
                            }
                        }
                        
                        self.next_token()?;
                    }
                } else {
                    // Skip just a single token for simple types
                    self.next_token()?;
                }
            }
        } else {
            return Err(Error::from_str(&format!(
                "Expected identifier as function parameter, got {:?}",
                self.current_token
            )));
        }

        // Parse remaining parameters
        while self.current_token == Token::Comma {
            self.next_token()?; // Consume ','
            
            // Parse parameter name
            if let Token::Identifier(name) = &self.current_token.clone() {
                let param_name = ast::Identifier {
                    token: self.current_token.token_literal(),
                    value: name.clone(),
                };
                identifiers.push(param_name);
                
                // Move past the parameter name
                self.next_token()?;
                
                // Skip the type annotation if present
                if self.current_token != Token::Comma && self.current_token != Token::RParen {
                    // Handle complex type cases like "f slay(A) B" (function types)
                    if self.current_token == Token::Slay {
                        // Skip until we reach a comma or closing paren
                        let mut depth = 0;
                        
                        while self.current_token != Token::Comma && 
                              self.current_token != Token::RParen &&
                              self.current_token != Token::Eof {
                            
                            // Track nested parentheses to handle function types properly
                            if self.current_token == Token::LParen {
                                depth += 1;
                            } else if self.current_token == Token::RParen {
                                depth -= 1;
                                if depth < 0 && !matches!(self.peek_token, Token::Identifier(_)) {
                                    // We've reached the end of the parameter list
                                    break;
                                }
                            }
                            
                            self.next_token()?;
                        }
                    } else {
                        // Skip just a single token for simple types
                        self.next_token()?;
                    }
                }
            } else {
                return Err(Error::from_str(&format!(
                    "Expected identifier after comma in parameters, got {:?}",
                    self.current_token
                )));
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

    /// Parse type parameters for generic functions/structs ([T], [A, B], etc.)
    /// This method also supports type constraint parameters in the future.
    fn parse_type_parameters(&mut self) -> Result<Vec<ast::Identifier>, Error> {
        let mut type_parameters = Vec::new();
        
        // The current token should be '['
        if self.current_token != Token::LBracket {
            return Err(Error::from_str(
                &format!("Expected '[' to start type parameters, got {:?}", self.current_token)
            ));
        }
        
        // Move past '['
        self.next_token()?;
        
        // Empty type parameters case
        if self.current_token == Token::RBracket {
            self.next_token()?; // Consume ']'
            return Ok(type_parameters);
        }
        
        // Parse first type parameter
        if let Token::Identifier(param_name) = &self.current_token {
            let param_name_value = param_name.clone();
            type_parameters.push(ast::Identifier {
                token: self.current_token.token_literal(),
                value: param_name_value,
            });
            self.next_token()?;
            
            // Skip any constraint that might be present (for future compatibility)
            // For example: [T: Comparable]
            if self.current_token == Token::Colon {
                // Consume ':'
                self.next_token()?;
                
                // Skip the constraint (can be an identifier or more complex constraint)
                // We're just preparing for this syntax, not fully implementing it yet
                if let Token::Identifier(_) = &self.current_token {
                    // Simple constraint like [T: Comparable]
                    self.next_token()?;
                }
            }
        } else {
            return Err(Error::from_str(
                &format!("Expected identifier as type parameter, got {:?}", self.current_token)
            ));
        }
        
        // Parse remaining type parameters
        while self.current_token == Token::Comma {
            self.next_token()?; // Consume ','
            
            if let Token::Identifier(param_name) = &self.current_token {
                let param_name_value = param_name.clone();
                type_parameters.push(ast::Identifier {
                    token: self.current_token.token_literal(),
                    value: param_name_value,
                });
                self.next_token()?;
                
                // Skip any constraint that might be present (for future compatibility)
                if self.current_token == Token::Colon {
                    // Consume ':'
                    self.next_token()?;
                    
                    // Skip the constraint (only single identifier constraints for now)
                    if let Token::Identifier(_) = &self.current_token {
                        self.next_token()?;
                    }
                }
            } else {
                return Err(Error::from_str(
                    &format!("Expected identifier after comma in type parameters, got {:?}", self.current_token)
                ));
            }
        }
        
        // Expect closing bracket
        if self.current_token != Token::RBracket {
            return Err(Error::from_str(
                &format!("Expected ']' after type parameters, got {:?}", self.current_token)
            ));
        }
        
        // Move past ']'
        self.next_token()?;
        
        Ok(type_parameters)
    }

    /// Parse type arguments for generic types ([normie], [tea, normie], etc.)
    fn parse_type_arguments(&mut self) -> Result<Vec<Box<dyn Expression>>, Error> {
        let mut type_args = Vec::new();
        
        // The current token should be '['
        if self.current_token != Token::LBracket {
            return Err(Error::from_str(
                &format!("Expected '[' to start type arguments, got {:?}", self.current_token)
            ));
        }
        
        // Move past '['
        self.next_token()?;
        
        // Empty type arguments case
        if self.current_token == Token::RBracket {
            self.next_token()?; // Consume ']'
            return Ok(type_args);
        }
        
        // Handle the case where we have a type name like 'normie'
        if let Some(token_value) = self.token_to_type_name() {
            // Create an identifier for the type
            let type_ident = ast::Identifier {
                token: self.current_token.token_literal(),
                value: token_value,
            };
            
            // Add it to the type arguments
            type_args.push(Box::new(type_ident));
            
            // Move past the type name
            self.next_token()?;
        } else {
            // Parse first type argument (can be an identifier or a type expression)
            type_args.push(self.parse_expression(Precedence::Lowest)?);
        }
        
        // Parse remaining type arguments
        while self.current_token == Token::Comma {
            self.next_token()?; // Consume ','
            
            // Handle built-in type names like 'normie'
            if let Some(token_value) = self.token_to_type_name() {
                // Create an identifier for the type
                let type_ident = ast::Identifier {
                    token: self.current_token.token_literal(),
                    value: token_value,
                };
                
                // Add it to the type arguments
                type_args.push(Box::new(type_ident));
                
                // Move past the type name
                self.next_token()?;
            } else {
                // Parse the expression as normal
                type_args.push(self.parse_expression(Precedence::Lowest)?);
            }
        }
        
        // Expect closing bracket
        if self.current_token != Token::RBracket {
            return Err(Error::from_str(
                &format!("Expected ']' after type arguments, got {:?}", self.current_token)
            ));
        }
        
        // Move past ']'
        self.next_token()?;
        
        Ok(type_args)
    }

    /// Parse a function literal (stan (...) { ... } in CURSED) or generic function (slay name[T](...) { ... })
    fn parse_function_literal(&mut self) -> Result<Box<dyn Expression>, Error> {
        let token = self.current_token.clone(); // Token::Stan or Token::Slay
        
        // Check for optional type parameters [T] or [A, B, C]
        let mut type_parameters = Vec::new();
        
        // If next token is '[', we have type parameters
        if self.peek_token == Token::LBracket {
            self.next_token()?; // Move to '['  
            type_parameters = self.parse_type_parameters()?;
        }

        // Parse function parameters
        let parameters = self.parse_function_parameters()?;
        
        // Check for optional return type
        let mut return_type = None;
        
        // Check if the current token is a type identifier before the opening brace
        if self.current_token != Token::LBrace {
            // Check for complex return types like "slay(A) B"
            if self.current_token == Token::Slay {
                // Capture the full return type as a string
                let start_pos = self.lexer.position;
                let mut depth = 0;
                let mut has_encountered_lparen = false;
                
                // Parse until we hit either the opening brace or EOF
                while self.current_token != Token::LBrace && self.current_token != Token::Eof {
                    if self.current_token == Token::LParen {
                        has_encountered_lparen = true;
                        depth += 1;
                    } else if self.current_token == Token::RParen {
                        depth -= 1;
                    }
                    
                    // If we've balanced all parentheses and we've seen at least one opening paren,
                    // and we've encountered an identifier after closing the last paren, we're done
                    if depth == 0 && has_encountered_lparen && !matches!(self.peek_token, Token::Identifier(_)) {
                        self.next_token()?; // Move past the last token of the return type
                        break;
                    }
                    
                    self.next_token()?;
                }
                
                // Create a string from the parsed tokens for the return type
                let end_pos = self.lexer.position;
                let type_text = if start_pos < end_pos && start_pos < self.lexer.input.len() {
                    self.lexer.input[start_pos..end_pos].trim().to_string()
                } else {
                    "slay(?) ?".to_string() // Fallback if we can't get the text
                };
                
                return_type = Some(ast::Identifier {
                    token: "return_type".to_string(),
                    value: type_text,
                });
            } else if let Token::Identifier(type_name) = &self.current_token {
                return_type = Some(ast::Identifier {
                    token: self.current_token.token_literal(),
                    value: type_name.clone(),
                });
                
                // Move past the return type
                self.next_token()?;
            } else if let Some(token_value) = self.token_to_type_name() {
                return_type = Some(ast::Identifier {
                    token: self.current_token.token_literal(),
                    value: token_value,
                });
                
                // Move past the return type
                self.next_token()?;
            }
        }
        
        // Expect opening brace for body
        if self.current_token != Token::LBrace {
            return Err(Error::from_str(&format!(
                "Expected '{{' for function body, got {:?}",
                self.current_token
            )));
        }

        let body = self.parse_block_statement()?;

        // For now, is_variadic is always false
        let is_variadic = false;

        Ok(Box::new(ast::FunctionLiteral {
            token,
            type_parameters,
            parameters,
            body,
            is_variadic,
            return_type,
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
    
    #[test]
    fn test_channel_expressions() -> Result<(), Error> {
        // Test channel type declaration
        let input = "sus ch = dm smol";
        let program = test_parser_with_input(input)?;
        
        // Verify we have one statement
        assert_eq!(program.statements.len(), 1);
        
        // Check that it's a LetStatement with a ChannelExpression value
        let stmt = program.statements[0].as_any().downcast_ref::<ast::LetStatement>().unwrap();
        let expr = stmt.value.as_ref().unwrap().as_any().downcast_ref::<ast::ChannelExpression>();
        assert!(expr.is_some(), "Expression is not a ChannelExpression");
        
        // Test send operation
        let input = "ch <- 42";
        let program = test_parser_with_input(input)?;
        
        // Verify we have one statement
        assert_eq!(program.statements.len(), 1);
        
        // Check that it's an ExpressionStatement with a SendExpression
        let stmt = program.statements[0].as_any().downcast_ref::<ast::ExpressionStatement>().unwrap();
        let expr = stmt.expression.as_ref().unwrap().as_any().downcast_ref::<ast::SendExpression>();
        assert!(expr.is_some(), "Expression is not a SendExpression");
        
        // Test receive operation
        let input = "sus value = <-ch";
        let program = test_parser_with_input(input)?;
        
        // Verify we have one statement
        assert_eq!(program.statements.len(), 1);
        
        // Check that it's a LetStatement with a ReceiveExpression value
        let stmt = program.statements[0].as_any().downcast_ref::<ast::LetStatement>().unwrap();
        let expr = stmt.value.as_ref().unwrap().as_any().downcast_ref::<ast::ReceiveExpression>();
        assert!(expr.is_some(), "Expression is not a ReceiveExpression");
        
        Ok(())
    }
    
    #[test]
    fn test_parse_later_statement() -> Result<(), Error> {
        let input = "later close_file(resource);";
        let program = test_parser_with_input(input)?;
        
        // Verify we have exactly one statement
        assert_eq!(program.statements.len(), 1, "Program should have 1 statement");
        
        // Check that the statement is a later statement
        let stmt = program.statements[0].as_any().downcast_ref::<ast::LaterStatement>();
        assert!(stmt.is_some(), "Statement is not a LaterStatement");
        
        // Check the token and expression
        let stmt = stmt.unwrap();
        assert_eq!(stmt.token, "later", "Expected 'later' token");
        
        // Verify the expression is a call expression
        assert!(stmt.expression.is_call_expression(), "Expression is not a call expression");
        
        // Test more complex later statements
        let inputs = vec![
            "later resource.close();",
            "later println(\"Cleaning up...\");"
        ];
        
        for input in inputs {
            let result = test_parser_with_input(input);
            assert!(result.is_ok(), "Failed to parse: {}", input);
        }
        
        Ok(())
    }
    
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
    fn test_parse_break_statement() -> Result<(), Error> {
        let input = "ghosted;";
        let program = test_parser_with_input(input)?;
        
        // Verify we have exactly one statement
        assert_eq!(program.statements.len(), 1, "Program should have 1 statement");
        
        // Check that the statement is a break statement
        let stmt = program.statements[0].as_any().downcast_ref::<ast::BreakStatement>();
        assert!(stmt.is_some(), "Statement is not a BreakStatement");
        
        // Check the token is correct
        let stmt = stmt.unwrap();
        assert_eq!(stmt.token, "ghosted", "Expected 'ghosted' token");
        
        Ok(())
    }
    
    #[test]
    fn test_parse_byte_literals() -> Result<(), Error> {
        let input = "b'a';";
        let program = test_parser_with_input(input)?;
        
        // Verify we have exactly one statement (expression statement)
        assert_eq!(program.statements.len(), 1, "Program should have 1 statement");
        
        // Check that it's an expression statement containing a byte literal
        let stmt = program.statements[0].as_any().downcast_ref::<ast::ExpressionStatement>().unwrap();
        let expr = stmt.expression.as_ref().unwrap().as_any().downcast_ref::<ast::ByteLiteral>();
        assert!(expr.is_some(), "Expression is not a ByteLiteral");
        
        // Check the value is correct
        let byte_literal = expr.unwrap();
        assert_eq!(byte_literal.value, b'a', "Expected byte value 'a'");
        
        Ok(())
    }
    
    #[test]
    fn test_parse_rune_literals() -> Result<(), Error> {
        let input = "'X';";
        let program = test_parser_with_input(input)?;
        
        // Verify we have exactly one statement (expression statement)
        assert_eq!(program.statements.len(), 1, "Program should have 1 statement");
        
        // Check that it's an expression statement containing a rune literal
        let stmt = program.statements[0].as_any().downcast_ref::<ast::ExpressionStatement>().unwrap();
        let expr = stmt.expression.as_ref().unwrap().as_any().downcast_ref::<ast::RuneLiteral>();
        assert!(expr.is_some(), "Expression is not a RuneLiteral");
        
        // Check the value is correct
        let rune_literal = expr.unwrap();
        assert_eq!(rune_literal.value, 'X', "Expected rune value 'X'");
        
        // Test unicode characters
        let input = "'🙂';";
        let program = test_parser_with_input(input)?;
        
        let stmt = program.statements[0].as_any().downcast_ref::<ast::ExpressionStatement>().unwrap();
        let expr = stmt.expression.as_ref().unwrap().as_any().downcast_ref::<ast::RuneLiteral>();
        assert!(expr.is_some(), "Expression is not a RuneLiteral");
        
        let rune_literal = expr.unwrap();
        assert_eq!(rune_literal.value, '🙂', "Expected rune value '🙂'");
        
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