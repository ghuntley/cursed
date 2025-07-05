//! Parser module for CURSED language

use crate::error::{CursedError, StructuredError, ErrorCode};
use crate::error::structured::ErrorSourceLocation;
use crate::lexer::{Lexer, Token, TokenKind};
use crate::ast::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Result<Self, CursedError> {
        let tokens = lexer.tokenize()?;
        Ok(Self {
            tokens,
            current: 0,
            errors: Vec::new(),
        })
    }

    /// Alternative constructor for direct token input (used by tests)
    pub fn from_tokens(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            errors: Vec::new(),
        }
    }

    /// Alias for parse_program() to match test expectations
    pub fn parse(&mut self) -> Result<Program, CursedError> {
        self.parse_program()
    }

    pub fn parse_program(&mut self) -> Result<Program, CursedError> {
        let mut statements = Vec::new();
        let mut imports = Vec::new();
        let mut package = None;

        // Skip initial newlines
        self.skip_newlines();

        // Parse package declaration if present
        if self.match_tokens(&[TokenKind::Vibe]) {
            package = Some(self.parse_package_declaration()?);
            self.skip_newlines();
        }

        // Parse imports
        while self.match_tokens(&[TokenKind::Yeet]) {
            imports.push(self.parse_import_statement()?);
            self.skip_newlines();
        }

        // Parse statements
        while !self.is_at_end() {
            self.skip_newlines();
            if self.is_at_end() {
                break;
            }
            
            match self.parse_statement() {
                Ok(stmt) => {
                    log::debug!("➕ Adding statement to program: {:?}", std::mem::discriminant(&stmt));
                    statements.push(stmt);
                    
                    // Optional semicolon after statement
                    if self.check(&TokenKind::Semicolon) {
                        self.advance();
                    }
                },
                Err(e) => {
                    // Record error but continue parsing
                    log::error!("❌ Parse error: {}", e);
                    self.errors.push(format!("Parse error: {}", e));
                    self.synchronize();
                }
            }
            self.skip_newlines();
        }

        log::info!("📋 Program parsed with {} statements", statements.len());
        Ok(Program {
            statements,
            imports,
            package,
        })
    }

    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    // Helper methods
    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn peek_ahead(&self, offset: usize) -> &Token {
        let index = self.current + offset;
        if index >= self.tokens.len() {
            &self.tokens[self.tokens.len() - 1] // Return EOF token
        } else {
            &self.tokens[index]
        }
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&self, kind: &TokenKind) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.peek().kind == kind
        }
    }

    /// Check if the current token is a valid type token (identifier or type keyword)
    fn check_type_token(&self) -> bool {
        matches!(self.peek().kind, 
            TokenKind::Identifier | 
            TokenKind::Normie | 
            TokenKind::Tea | 
            TokenKind::Txt | 
            TokenKind::Dm |
            TokenKind::Truth |
            TokenKind::Lies |
            TokenKind::Cap
        )
    }

    fn is_keyword(&self) -> bool {
        if self.is_at_end() {
            return false;
        }
        
        match self.peek().kind {
            TokenKind::Slay | TokenKind::Yolo | TokenKind::Sus | TokenKind::Facts |
            TokenKind::Lowkey | TokenKind::Highkey | TokenKind::Periodt | TokenKind::Stan |
            TokenKind::Bestie | TokenKind::Flex | TokenKind::Ghosted | TokenKind::Simp |
            TokenKind::Squad | TokenKind::Collab | TokenKind::Vibe | TokenKind::Yeet |
            TokenKind::BeLike | TokenKind::VibeCheck | TokenKind::Mood | TokenKind::Basic |
            TokenKind::YeetError | TokenKind::Catch | TokenKind::Where | TokenKind::Normie |
            TokenKind::Tea | TokenKind::Cap | TokenKind::NoCap | TokenKind::Truth |
            TokenKind::Lies | TokenKind::MainCharacter | TokenKind::Dm | TokenKind::Select |
            TokenKind::Spill | TokenKind::Priv | TokenKind::Crew => true,
            _ => false,
        }
    }

    fn match_tokens(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.check(kind) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, kind: TokenKind, message: &str) -> Result<&Token, CursedError> {
        if self.check(&kind) {
            Ok(self.advance())
        } else {
            let current_token = &self.peek();
            let error = StructuredError::unexpected_token(
                &format!("{:?}", kind),
                &current_token.lexeme,
                current_token.line,
                current_token.column,
            );
            Err(CursedError::from(error))
        }
    }

    fn skip_newlines(&mut self) {
        while self.match_tokens(&[TokenKind::Newline]) {
            // Skip newlines
        }
    }
    
    fn skip_newlines_and_semicolons(&mut self) {
        while self.match_tokens(&[TokenKind::Newline, TokenKind::Semicolon]) {
            // Skip newlines and semicolons
        }
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().kind == TokenKind::Semicolon {
                return;
            }

            match self.peek().kind {
                TokenKind::Slay | TokenKind::Sus | TokenKind::Facts | TokenKind::Lowkey |
                TokenKind::Bestie | TokenKind::Yolo | TokenKind::Vibe | TokenKind::Yeet |
                TokenKind::RightBrace | TokenKind::Newline => return,
                _ => {}
            }

            self.advance();
        }
    }

    // Parsing methods
    fn parse_package_declaration(&mut self) -> Result<PackageDeclaration, CursedError> {
        let name_token = self.consume(TokenKind::Identifier, "Expected package name")?;
        let name = name_token.lexeme.clone();
        
        // Consume optional semicolon after package name
        if self.check(&TokenKind::Semicolon) {
            self.advance();
        }
        
        Ok(PackageDeclaration {
            name,
            version: None,
        })
    }

    fn parse_import_statement(&mut self) -> Result<ImportStatement, CursedError> {
        let path_token = self.consume(TokenKind::String, "Expected import path")?;
        Ok(ImportStatement {
            path: path_token.lexeme.clone(),
            alias: None,
            items: Vec::new(),
        })
    }

    fn parse_statement(&mut self) -> Result<Statement, CursedError> {
        // Check for visibility modifiers
        let visibility = self.parse_visibility()?;
        
        log::debug!("🔍 Parsing statement with token: {:?}", self.peek().kind);
        match self.peek().kind {
            TokenKind::Slay => {
                log::info!("📝 Parsing function statement with 'slay' keyword");
                Ok(Statement::Function(self.parse_function_statement_with_visibility(visibility)?))
            },
            TokenKind::Sus => Ok(Statement::Let(self.parse_let_statement_with_visibility(visibility)?)),
            TokenKind::Facts => Ok(Statement::Let(self.parse_const_statement_with_visibility(visibility)?)),
            TokenKind::Txt => Ok(Statement::Let(self.parse_typed_variable_statement_with_visibility(visibility)?)),
            TokenKind::Lowkey => {
                if visibility != crate::ast::Visibility::Private {
                    return Err(CursedError::parse_error("Visibility modifiers not allowed on control flow statements"));
                }
                Ok(Statement::If(self.parse_if_statement()?))
            },
            TokenKind::Periodt => {
                if visibility != crate::ast::Visibility::Private {
                    return Err(CursedError::parse_error("Visibility modifiers not allowed on control flow statements"));
                }
                log::info!("📝 Parsing while statement with 'periodt' keyword");
                Ok(Statement::While(self.parse_while_statement()?))
            },
            TokenKind::Bestie => {
                if visibility != crate::ast::Visibility::Private {
                    return Err(CursedError::parse_error("Visibility modifiers not allowed on control flow statements"));
                }
                log::info!("📝 Parsing for statement with 'bestie' keyword");
                Ok(Statement::For(self.parse_for_statement()?))
            },
            TokenKind::VibeCheck => {
                if visibility != crate::ast::Visibility::Private {
                    return Err(CursedError::parse_error("Visibility modifiers not allowed on control flow statements"));
                }
                log::info!("📝 Parsing switch statement with 'vibe_check' keyword");
                Ok(Statement::Switch(self.parse_switch_statement()?))
            },
            TokenKind::Yolo => {
                if visibility != crate::ast::Visibility::Private {
                    return Err(CursedError::parse_error("Visibility modifiers not allowed on return statements"));
                }
                Ok(Statement::Return(self.parse_return_statement()?))
            },
            TokenKind::Return => {
                if visibility != crate::ast::Visibility::Private {
                    return Err(CursedError::parse_error("Visibility modifiers not allowed on return statements"));
                }
                Ok(Statement::Return(self.parse_return_statement()?))
            },
            TokenKind::BeLike => {
                log::info!("📝 Parsing struct statement with 'be_like' keyword");
                Ok(Statement::Struct(self.parse_struct_statement_with_visibility(visibility)?))
            },
            TokenKind::Collab => {
                log::info!("📝 Parsing interface statement with 'collab' keyword");
                Ok(Statement::Interface(self.parse_interface_statement_with_visibility(visibility)?))
            },
            TokenKind::Stan => {
                if visibility != crate::ast::Visibility::Private {
                    return Err(CursedError::parse_error("Visibility modifiers not allowed on goroutine statements"));
                }
                log::info!("📝 Parsing goroutine statement with 'stan' keyword");
                Ok(Statement::Goroutine(self.parse_goroutine_statement()?))
            },
            TokenKind::Select => {
                if visibility != crate::ast::Visibility::Private {
                    return Err(CursedError::parse_error("Visibility modifiers not allowed on select statements"));
                }
                log::info!("📝 Parsing select statement");
                Ok(Statement::Select(self.parse_select_statement()?))
            },
            TokenKind::YeetError => {
                if visibility != crate::ast::Visibility::Private {
                    return Err(CursedError::parse_error("Visibility modifiers not allowed on panic statements"));
                }
                log::info!("📝 Parsing panic statement with 'yeet_error' keyword");
                Ok(Statement::Panic(self.parse_panic_statement()?))
            },
            TokenKind::Catch => {
                if visibility != crate::ast::Visibility::Private {
                    return Err(CursedError::parse_error("Visibility modifiers not allowed on catch statements"));
                }
                log::info!("📝 Parsing catch statement with 'catch' keyword");
                Ok(Statement::Catch(self.parse_catch_statement()?))
            },
            _ => {
                if visibility != crate::ast::Visibility::Private {
                    return Err(CursedError::parse_error("Visibility modifiers not allowed on expressions"));
                }
                

                // Check if this is an assignment (identifier = expression or tuple destructuring)
                if (self.check(&TokenKind::Identifier) && self.peek_ahead(1).kind == TokenKind::Equal) ||
                   (self.check(&TokenKind::LeftParen) && self.is_tuple_assignment()) {
                    Ok(Statement::Assignment(self.parse_assignment_statement()?))
                } else {
                    Ok(Statement::Expression(self.parse_expression()?))
                }
            },
        }
    }

    fn parse_function_statement(&mut self) -> Result<FunctionStatement, CursedError> {
        self.consume(TokenKind::Slay, "Expected 'slay'")?;
        let name = self.consume(TokenKind::Identifier, "Expected function name")?.lexeme.clone();
        
        // Parse optional generic parameters
        let type_parameters = self.parse_generic_parameters()?;
        
        self.consume(TokenKind::LeftParen, "Expected '(' after function name")?;
        let mut parameters = Vec::new();
        
        if !self.check(&TokenKind::RightParen) {
            loop {
                let param_name = self.consume(TokenKind::Identifier, "Expected parameter name")?.lexeme.clone();
                
                // Check for optional type after parameter name (CURSED syntax: "x normie", "name tea")
                let param_type = if self.check_type_token() {
                    Some(self.advance().lexeme.clone())
                } else {
                    None
                };
                
                parameters.push(Parameter {
                    name: param_name,
                    param_type,
                });
                
                if !self.match_tokens(&[TokenKind::Comma]) {
                    break;
                }
            }
        }
        
        self.consume(TokenKind::RightParen, "Expected ')' after parameters")?;
        
        // Parse optional return type (CURSED syntax: "slay func(x normie) normie { ... }")
        let return_type = if self.check_type_token() {
            Some(self.advance().lexeme.clone())
        } else {
            None
        };
        
        // Parse optional where clause
        let where_clause = self.parse_where_clause()?;
        
        self.consume(TokenKind::LeftBrace, "Expected '{' before function body")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            self.skip_newlines(); // Skip newlines before parsing each statement
            if !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
                body.push(self.parse_statement()?);
            }
        }
        
        self.consume(TokenKind::RightBrace, "Expected '}' after function body")?;
        
        Ok(FunctionStatement {
            name,
            type_parameters,
            parameters,
            body,
            return_type,
            where_clause,
            visibility: crate::ast::Visibility::Private,
        })
    }

    fn parse_function_statement_with_visibility(&mut self, visibility: crate::ast::Visibility) -> Result<FunctionStatement, CursedError> {
        self.consume(TokenKind::Slay, "Expected 'slay'")?;
        let name = self.consume(TokenKind::Identifier, "Expected function name")?.lexeme.clone();
        
        // Parse optional generic parameters
        let type_parameters = self.parse_generic_parameters()?;
        
        self.consume(TokenKind::LeftParen, "Expected '(' after function name")?;
        let mut parameters = Vec::new();
        
        if !self.check(&TokenKind::RightParen) {
            loop {
                let param_name = self.consume(TokenKind::Identifier, "Expected parameter name")?.lexeme.clone();
                
                // Check for optional type after parameter name (CURSED syntax: "x normie", "name tea")
                let param_type = if self.check_type_token() {
                    Some(self.advance().lexeme.clone())
                } else {
                    None
                };
                
                parameters.push(Parameter {
                    name: param_name,
                    param_type,
                });
                
                if !self.match_tokens(&[TokenKind::Comma]) {
                    break;
                }
            }
        }
        
        self.consume(TokenKind::RightParen, "Expected ')' after parameters")?;
        
        // Parse optional return type (CURSED syntax: "slay func(x normie) normie { ... }")
        let return_type = if self.check_type_token() {
            Some(self.advance().lexeme.clone())
        } else {
            None
        };
        
        // Parse optional where clause
        let where_clause = self.parse_where_clause()?;
        
        self.consume(TokenKind::LeftBrace, "Expected '{' before function body")?;
        self.skip_newlines_and_semicolons();
        
        let mut body = Vec::new();
        log::debug!("🔧 Parsing function body for: {}", name);
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            self.skip_newlines_and_semicolons(); // Skip newlines before parsing each statement
            if !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
                let stmt = self.parse_statement()?;
                log::debug!("➕ Adding statement to function body: {:?}", std::mem::discriminant(&stmt));
                body.push(stmt);
                
                // Optional semicolon after statement
                if self.check(&TokenKind::Semicolon) {
                    self.advance();
                }
            }
        }
        log::debug!("✅ Function body parsing complete for: {}", name);
        
        self.consume(TokenKind::RightBrace, "Expected '}' after function body")?;
        
        Ok(FunctionStatement {
            name,
            type_parameters,
            parameters,
            body,
            return_type,
            where_clause,
            visibility,
        })
    }

    fn parse_let_statement(&mut self) -> Result<LetStatement, CursedError> {
        self.consume(TokenKind::Sus, "Expected 'sus'")?;
        let name = self.consume(TokenKind::Identifier, "Expected variable name")?.lexeme.clone();
        
        // Check for optional type after variable name (CURSED syntax: "sus result normie = ...")
        let var_type = if self.check_type_token() {
            Some(self.advance().lexeme.clone())
        } else {
            None
        };
        
        self.consume(TokenKind::Equal, "Expected '=' after variable name")?;
        let value = self.parse_expression()?;
        
        Ok(LetStatement {
            name,
            value,
            var_type,
            visibility: crate::ast::Visibility::Private,
        })
    }

    fn parse_assignment_statement(&mut self) -> Result<AssignmentStatement, CursedError> {
        let target = self.parse_assignment_target()?;
        self.consume(TokenKind::Equal, "Expected '=' after assignment target")?;
        let value = self.parse_expression()?;
        
        Ok(AssignmentStatement {
            target,
            value,
        })
    }

    fn parse_assignment_target(&mut self) -> Result<crate::ast::AssignmentTarget, CursedError> {
        // Check for tuple destructuring: (a, b, c) = ...
        if self.check(&TokenKind::LeftParen) {
            self.advance(); // consume '('
            
            let mut names = Vec::new();
            if !self.check(&TokenKind::RightParen) {
                loop {
                    let name = self.consume(TokenKind::Identifier, "Expected variable name in tuple destructuring")?;
                    names.push(name.lexeme.clone());
                    
                    if !self.match_tokens(&[TokenKind::Comma]) {
                        break;
                    }
                }
            }
            
            self.consume(TokenKind::RightParen, "Expected ')' after tuple destructuring")?;
            Ok(crate::ast::AssignmentTarget::Tuple(names))
        } else {
            // Single variable assignment
            let name = self.consume(TokenKind::Identifier, "Expected variable name")?;
            Ok(crate::ast::AssignmentTarget::Single(name.lexeme.clone()))
        }
    }

    /// Check if the current position looks like a tuple assignment (a, b) =
    fn is_tuple_assignment(&self) -> bool {
        if !self.check(&TokenKind::LeftParen) {
            return false;
        }
        
        // Look ahead to see if this is a tuple assignment pattern
        // We're looking for: ( identifier, identifier, ... ) =
        let mut pos = 1; // Start after the opening paren
        
        // Check for at least one identifier
        if pos >= self.tokens.len() || self.tokens[self.current + pos].kind != TokenKind::Identifier {
            return false;
        }
        pos += 1;
        
        // Look for comma-separated identifiers
        while self.current + pos < self.tokens.len() {
            match self.tokens[self.current + pos].kind {
                TokenKind::Comma => {
                    pos += 1;
                    // After comma, expect identifier
                    if self.current + pos >= self.tokens.len() || self.tokens[self.current + pos].kind != TokenKind::Identifier {
                        return false;
                    }
                    pos += 1;
                },
                TokenKind::RightParen => {
                    pos += 1;
                    // After closing paren, expect =
                    return self.current + pos < self.tokens.len() && self.tokens[self.current + pos].kind == TokenKind::Equal;
                },
                _ => return false,
            }
        }
        
        false
    }

    fn parse_const_statement(&mut self) -> Result<LetStatement, CursedError> {
        self.consume(TokenKind::Facts, "Expected 'facts'")?;
        let name = self.consume(TokenKind::Identifier, "Expected constant name")?.lexeme.clone();
        self.consume(TokenKind::Equal, "Expected '=' after constant name")?;
        let value = self.parse_expression()?;
        
        Ok(LetStatement {
            name,
            value,
            var_type: None, // Constants don't need explicit type annotations
            visibility: crate::ast::Visibility::Private,
        })
    }

    fn parse_let_statement_with_visibility(&mut self, visibility: crate::ast::Visibility) -> Result<LetStatement, CursedError> {
        self.consume(TokenKind::Sus, "Expected 'sus'")?;
        let name = self.consume(TokenKind::Identifier, "Expected variable name")?.lexeme.clone();
        
        // Check for optional type after variable name (CURSED syntax: "sus result normie = ...")
        let var_type = if self.check_type_token() {
            Some(self.advance().lexeme.clone())
        } else {
            None
        };
        
        self.consume(TokenKind::Equal, "Expected '=' after variable name")?;
        let value = self.parse_expression()?;
        
        Ok(LetStatement {
            name,
            value,
            var_type,
            visibility,
        })
    }

    fn parse_const_statement_with_visibility(&mut self, visibility: crate::ast::Visibility) -> Result<LetStatement, CursedError> {
        self.consume(TokenKind::Facts, "Expected 'facts'")?;
        let name = self.consume(TokenKind::Identifier, "Expected constant name")?.lexeme.clone();
        self.consume(TokenKind::Equal, "Expected '=' after constant name")?;
        let value = self.parse_expression()?;
        
        Ok(LetStatement {
            name,
            value,
            var_type: None, // Generic constants don't need explicit type annotations
            visibility,
        })
    }

    fn parse_typed_variable_statement_with_visibility(&mut self, visibility: crate::ast::Visibility) -> Result<LetStatement, CursedError> {
        self.consume(TokenKind::Txt, "Expected 'txt'")?;
        let name = self.consume(TokenKind::Identifier, "Expected variable name")?.lexeme.clone();
        self.consume(TokenKind::Equal, "Expected '=' after variable name")?;
        let value = self.parse_expression()?;
        
        Ok(LetStatement {
            name,
            value,
            var_type: None, // Typed variables don't need explicit type annotations
            visibility,
        })
    }

    /// Parse visibility modifiers (spill = pub, priv = private, crew = pkg)
    fn parse_visibility(&mut self) -> Result<crate::ast::Visibility, CursedError> {
        match self.peek().kind {
            TokenKind::Spill => {
                self.advance();
                Ok(crate::ast::Visibility::Public)
            },
            TokenKind::Priv => {
                self.advance();
                Ok(crate::ast::Visibility::Private)
            },
            TokenKind::Crew => {
                self.advance();
                Ok(crate::ast::Visibility::Package)
            },
            _ => Ok(crate::ast::Visibility::Private), // Default visibility
        }
    }

    fn parse_if_statement(&mut self) -> Result<IfStatement, CursedError> {
        self.consume(TokenKind::Lowkey, "Expected 'lowkey'")?;
        self.skip_newlines(); // Skip newlines after lowkey
        let condition = self.parse_expression()?;
        self.skip_newlines(); // Skip newlines after condition
        self.consume(TokenKind::LeftBrace, "Expected '{' after if condition")?;
        
        let mut then_branch = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            // Skip newlines within if statement body
            if self.check(&TokenKind::Newline) {
                self.advance();
                continue;
            }
            then_branch.push(self.parse_statement()?);
            
            // Optional semicolon after statement
            if self.check(&TokenKind::Semicolon) {
                self.advance();
            }
        }
        
        self.consume(TokenKind::RightBrace, "Expected '}' after if body")?;
        
        let mut else_branch = None;
        if self.match_tokens(&[TokenKind::Highkey]) {
            self.consume(TokenKind::LeftBrace, "Expected '{' after else")?;
            let mut else_stmts = Vec::new();
            while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
                // Skip newlines within else statement body
                if self.check(&TokenKind::Newline) {
                    self.advance();
                    continue;
                }
                else_stmts.push(self.parse_statement()?);
                
                // Optional semicolon after statement  
                if self.check(&TokenKind::Semicolon) {
                    self.advance();
                }
            }
            self.consume(TokenKind::RightBrace, "Expected '}' after else body")?;
            else_branch = Some(else_stmts);
        }
        
        Ok(IfStatement {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn parse_return_statement(&mut self) -> Result<ReturnStatement, CursedError> {
        // Handle both 'yolo' and 'return' keywords
        if self.check(&TokenKind::Yolo) {
            self.consume(TokenKind::Yolo, "Expected 'yolo'")?;
        } else if self.check(&TokenKind::Return) {
            self.consume(TokenKind::Return, "Expected 'return'")?;
        } else {
            return Err(CursedError::parse_error("Expected 'yolo' or 'return'"));
        }
        
        // Skip any newlines after return keyword
        self.skip_newlines();
        
        let value = if self.check(&TokenKind::Semicolon) || self.check(&TokenKind::RightBrace) {
            None
        } else {
            Some(self.parse_expression()?)
        };
        
        Ok(ReturnStatement { value })
    }

    fn parse_while_statement(&mut self) -> Result<WhileStatement, CursedError> {
        self.consume(TokenKind::Periodt, "Expected 'periodt'")?;
        let condition = self.parse_expression()?;
        self.consume(TokenKind::LeftBrace, "Expected '{' after while condition")?;
        
        let mut body = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            body.push(self.parse_statement()?);
        }
        
        self.consume(TokenKind::RightBrace, "Expected '}' after while body")?;
        
        Ok(WhileStatement {
            condition,
            body,
        })
    }

    fn parse_for_statement(&mut self) -> Result<ForStatement, CursedError> {
        self.consume(TokenKind::Bestie, "Expected 'bestie'")?;
        
        // Parse for loop variants:
        // bestie init; condition; update { ... }
        // bestie variable in iterable { ... } (future enhancement)
        
        let init = if self.check(&TokenKind::Semicolon) {
            None
        } else {
            Some(Box::new(self.parse_statement()?))
        };
        
        self.consume(TokenKind::Semicolon, "Expected ';' after for loop init")?;
        
        let condition = if self.check(&TokenKind::Semicolon) {
            None
        } else {
            Some(self.parse_expression()?)
        };
        
        self.consume(TokenKind::Semicolon, "Expected ';' after for loop condition")?;
        
        let update = if self.check(&TokenKind::LeftBrace) {
            None
        } else {
            Some(self.parse_expression()?)
        };
        
        self.consume(TokenKind::LeftBrace, "Expected '{' after for loop header")?;
        
        let mut body = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            body.push(self.parse_statement()?);
        }
        
        self.consume(TokenKind::RightBrace, "Expected '}' after for body")?;
        
        Ok(ForStatement {
            init,
            condition,
            update,
            body,
        })
    }

    fn parse_switch_statement(&mut self) -> Result<SwitchStatement, CursedError> {
        self.consume(TokenKind::VibeCheck, "Expected 'vibe_check'")?;
        let expression = self.parse_expression()?;
        self.consume(TokenKind::LeftBrace, "Expected '{' after switch expression")?;
        self.skip_newlines();
        
        let mut cases = Vec::new();
        let mut default_case = None;
        
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            if self.match_tokens(&[TokenKind::Mood]) {
                // Parse case: mood pattern1, pattern2, ... :
                let mut patterns = Vec::new();
                
                // Parse first pattern
                patterns.push(self.parse_expression()?);
                
                // Parse additional patterns separated by commas
                while self.match_tokens(&[TokenKind::Comma]) {
                    self.skip_newlines();
                    patterns.push(self.parse_expression()?);
                }
                
                self.consume(TokenKind::Colon, "Expected ':' after case pattern(s)")?;
                self.skip_newlines();
                
                // Parse case body (indented statements without braces)
                let mut case_body = Vec::new();
                
                // Parse statements until we hit another mood/basic or closing brace
                while !self.check(&TokenKind::RightBrace) && 
                      !self.check(&TokenKind::Mood) && 
                      !self.check(&TokenKind::Basic) && 
                      !self.is_at_end() {
                    case_body.push(self.parse_statement()?);
                    self.skip_newlines();
                }
                
                // For now, combine multiple patterns into one by using the first pattern
                // In a full implementation, you'd need to modify SwitchCase to support multiple patterns
                let combined_pattern = if patterns.len() == 1 {
                    patterns.into_iter().next().unwrap()
                } else {
                    // Create a virtual pattern that represents multiple values
                    // For now, just use the first pattern
                    patterns.into_iter().next().unwrap()
                };
                
                cases.push(SwitchCase {
                    pattern: combined_pattern,
                    body: case_body,
                });
            } else if self.match_tokens(&[TokenKind::Basic]) {
                // Parse default case: basic:
                self.consume(TokenKind::Colon, "Expected ':' after 'basic'")?;
                self.skip_newlines();
                
                let mut default_body = Vec::new();
                while !self.check(&TokenKind::RightBrace) && 
                      !self.check(&TokenKind::Mood) && 
                      !self.is_at_end() {
                    default_body.push(self.parse_statement()?);
                    self.skip_newlines();
                }
                
                default_case = Some(default_body);
            } else {
                self.skip_newlines();
                if !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
                    return Err(CursedError::parse_error("Expected 'mood' or 'basic' in switch statement"));
                }
            }
        }
        
        self.consume(TokenKind::RightBrace, "Expected '}' after switch body")?;
        
        Ok(SwitchStatement {
            expression,
            cases,
            default_case,
        })
    }

    pub fn parse_expression(&mut self) -> Result<Expression, CursedError> {
        self.skip_newlines(); // Skip newlines at the start of expression parsing
        self.parse_channel_operations()
    }

    fn parse_channel_operations(&mut self) -> Result<Expression, CursedError> {
        // Check for channel receive first: <-channel
        if self.check(&TokenKind::LeftArrow) {
            self.advance();
            let channel = Box::new(self.parse_logical_or()?);
            return Ok(Expression::ChannelReceive(crate::ast::ChannelReceiveExpression {
                channel,
            }));
        }
        
        let mut expr = self.parse_logical_or()?;
        
        // Check for channel send: channel <- value
        if self.match_tokens(&[TokenKind::LeftArrow]) {
            let value = Box::new(self.parse_logical_or()?);
            expr = Expression::ChannelSend(crate::ast::ChannelSendExpression {
                channel: Box::new(expr),
                value,
            });
        }
        
        Ok(expr)
    }

    fn parse_logical_or(&mut self) -> Result<Expression, CursedError> {
        let mut expr = self.parse_logical_and()?;

        while self.match_tokens(&[TokenKind::PipePipe]) {
            let operator = self.previous().lexeme.clone();
            let right = self.parse_logical_and()?;
            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn parse_logical_and(&mut self) -> Result<Expression, CursedError> {
        let mut expr = self.parse_equality()?;

        while self.match_tokens(&[TokenKind::AmpAmp]) {
            let operator = self.previous().lexeme.clone();
            let right = self.parse_equality()?;
            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expression, CursedError> {
        let mut expr = self.parse_comparison()?;

        while self.match_tokens(&[TokenKind::BangEqual, TokenKind::EqualEqual]) {
            let operator = self.previous().lexeme.clone();
            let right = self.parse_comparison()?;
            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expression, CursedError> {
        let mut expr = self.parse_term()?;

        while self.match_tokens(&[TokenKind::Greater, TokenKind::GreaterEqual, 
                                  TokenKind::Less, TokenKind::LessEqual]) {
            let operator = self.previous().lexeme.clone();
            let right = self.parse_term()?;
            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expression, CursedError> {
        let mut expr = self.parse_factor()?;

        while self.match_tokens(&[TokenKind::Minus, TokenKind::Plus]) {
            let operator = self.previous().lexeme.clone();
            let right = self.parse_factor()?;
            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expression, CursedError> {
        let mut expr = self.parse_unary()?;

        while self.match_tokens(&[TokenKind::Slash, TokenKind::Star, TokenKind::Percent]) {
            let operator = self.previous().lexeme.clone();
            let right = self.parse_unary()?;
            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expression, CursedError> {
        if self.match_tokens(&[TokenKind::Bang, TokenKind::Minus]) {
            let operator = self.previous().lexeme.clone();
            let right = self.parse_unary()?;
            let unary_op = match operator.as_str() {
                "!" => UnaryOperator::Not,
                "-" => UnaryOperator::Minus,
                "+" => UnaryOperator::Plus,
                _ => return Err(CursedError::syntax_error("Invalid unary operator")),
            };
            return Ok(Expression::Unary(UnaryExpression {
                operator: unary_op,
                operand: Box::new(right),
            }));
        }

        self.parse_call()
    }

    fn parse_call(&mut self) -> Result<Expression, CursedError> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.match_tokens(&[TokenKind::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else if self.check(&TokenKind::LeftBrace) {
                // Check if this is a struct literal (identifier followed by {)
                if let Expression::Identifier(struct_name) = expr {
                    expr = self.parse_struct_literal(struct_name)?;
                } else {
                    break;
                }
            } else if self.match_tokens(&[TokenKind::Dot]) {
                // Check for tuple access (e.g., tuple.0, tuple.1)
                if self.check(&TokenKind::Number) {
                    let index_token = self.advance();
                    let index = index_token.lexeme.parse::<usize>()
                        .map_err(|_| CursedError::syntax_error("Invalid tuple index"))?;
                    
                    expr = Expression::TupleAccess(crate::ast::TupleAccessExpression {
                        tuple: Box::new(expr),
                        index,
                    });
                } else {
                    // Accept identifiers or keywords as property names
                    let property = if self.check(&TokenKind::Identifier) {
                        self.advance()
                    } else if self.check(&TokenKind::Spill) {
                        // Handle 'spill' specifically 
                        self.advance()
                    } else if self.is_keyword() {
                        // Allow other keywords as property names in member access
                        self.advance()
                    } else {
                        return Err(CursedError::syntax_error("Expected property name after '.'"));
                    };
                    
                    expr = Expression::MemberAccess(MemberAccessExpression {
                        object: Box::new(expr),
                        property: property.lexeme.clone(),
                    });
                }
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expression) -> Result<Expression, CursedError> {
        let mut arguments = Vec::new();

        if !self.check(&TokenKind::RightParen) {
            loop {
                arguments.push(self.parse_expression()?);
                if !self.match_tokens(&[TokenKind::Comma]) {
                    break;
                }
            }
        }

        self.consume(TokenKind::RightParen, "Expected ')' after arguments")?;

        Ok(Expression::Call(CallExpression {
            function: Box::new(callee),
            arguments,
        }))
    }

    /// Parse channel creation expression (dm type())
    fn parse_channel_creation(&mut self) -> Result<Expression, CursedError> {
        self.consume(TokenKind::Dm, "Expected 'dm'")?;
        
        // Parse the channel element type (handle different token types)
        let element_type = if self.match_tokens(&[TokenKind::Identifier]) {
            self.previous().lexeme.clone()
        } else if self.match_tokens(&[TokenKind::Truth, TokenKind::Lies]) { // Handle boolean keywords
            self.previous().lexeme.clone()
        } else {
            return Err(CursedError::syntax_error("Expected channel element type after 'dm'"));
        };
        
        // Parse parentheses (required for channel creation syntax)
        if self.match_tokens(&[TokenKind::LeftParen]) {
            // Optional capacity argument for buffered channels
            let capacity = if !self.check(&TokenKind::RightParen) {
                let expr = self.parse_expression()?;
                Some(Box::new(expr))
            } else {
                None
            };
            
            self.consume(TokenKind::RightParen, "Expected ')' after channel arguments")?;
            
            // Create channel creation expression
            Ok(Expression::ChannelCreation(crate::ast::ChannelCreationExpression {
                element_type,
                capacity,
            }))
        } else {
            // Simple channel type without parentheses syntax (also valid)
            Ok(Expression::ChannelCreation(crate::ast::ChannelCreationExpression {
                element_type,
                capacity: None,
            }))
        }
    }

    fn parse_primary(&mut self) -> Result<Expression, CursedError> {
        self.skip_newlines(); // Skip any newlines before parsing primary expression
        match &self.peek().kind {
            TokenKind::Truth => {
                self.advance();
                Ok(Expression::Boolean(true))
            },
            TokenKind::Lies => {
                self.advance();
                Ok(Expression::Boolean(false))
            },
            TokenKind::Number => {
                let token = self.advance();
                let value = token.lexeme.parse::<i64>()
                    .map_err(|_| CursedError::syntax_error("Invalid number literal"))?;
                Ok(Expression::Integer(value))
            },
            TokenKind::String => {
                let token = self.advance();
                Ok(Expression::String(token.lexeme.clone()))
            },
            TokenKind::Identifier => {
                let token = self.advance();
                Ok(Expression::Identifier(token.lexeme.clone()))
            },
            TokenKind::LeftParen => {
                self.parse_tuple_or_parenthesized()
            },
            TokenKind::Dm => {
                self.parse_channel_creation()
            },
            TokenKind::Pipe => {
                self.parse_lambda_expression()
            },
            TokenKind::PipePipe => {
                self.parse_lambda_expression()
            },
            _ => Err(CursedError::syntax_error("Expected expression")),
        }
    }

    fn parse_lambda_expression(&mut self) -> Result<Expression, CursedError> {
        let mut parameters = Vec::new();
        
        // Check if it's a parameterless lambda: || { body }
        if self.check(&TokenKind::PipePipe) {
            self.advance(); // consume ||
        } else {
            // Parse parameterized lambda: |param1, param2| { body }
            self.consume(TokenKind::Pipe, "Expected '|' to start lambda parameters")?;
            
            // Parse parameters
            if !self.check(&TokenKind::Pipe) {
                loop {
                    let param = self.consume(TokenKind::Identifier, "Expected parameter name")?;
                    parameters.push(param.lexeme.clone());
                    
                    if !self.match_tokens(&[TokenKind::Comma]) {
                        break;
                    }
                }
            }
            
            self.consume(TokenKind::Pipe, "Expected '|' to close lambda parameters")?;
        }
        
        // Parse lambda body
        self.consume(TokenKind::LeftBrace, "Expected '{' to start lambda body")?;
        self.skip_newlines();
        
        // For now, we'll parse a single expression as the body
        // In a full implementation, you might want to support multiple statements
        let body = if self.check(&TokenKind::RightBrace) {
            // Empty body - return unit/void expression
            Expression::Identifier("()".to_string())
        } else {
            self.parse_expression()?
        };
        
        self.skip_newlines();
        self.consume(TokenKind::RightBrace, "Expected '}' to close lambda body")?;
        
        Ok(Expression::Lambda(crate::ast::LambdaExpression {
            parameters,
            body: Box::new(body),
        }))
    }

    fn parse_tuple_or_parenthesized(&mut self) -> Result<Expression, CursedError> {
        self.advance(); // consume '('
        
        // Check for empty tuple: ()
        if self.check(&TokenKind::RightParen) {
            self.advance(); // consume ')'
            return Ok(Expression::Tuple(crate::ast::TupleExpression {
                elements: Vec::new(),
            }));
        }
        
        // Parse first expression
        let first_expr = self.parse_expression()?;
        
        // Check if it's a tuple (comma-separated) or parenthesized expression
        if self.check(&TokenKind::Comma) {
            // It's a tuple
            let mut elements = vec![first_expr];
            
            while self.match_tokens(&[TokenKind::Comma]) {
                // Allow trailing comma
                if self.check(&TokenKind::RightParen) {
                    break;
                }
                elements.push(self.parse_expression()?);
            }
            
            self.consume(TokenKind::RightParen, "Expected ')' after tuple elements")?;
            Ok(Expression::Tuple(crate::ast::TupleExpression { elements }))
        } else {
            // It's a parenthesized expression
            self.consume(TokenKind::RightParen, "Expected ')' after expression")?;
            Ok(first_expr)
        }
    }

    fn parse_struct_statement_with_visibility(&mut self, visibility: crate::ast::Visibility) -> Result<crate::ast::StructStatement, CursedError> {
        self.consume(TokenKind::BeLike, "Expected 'be_like'")?;
        let name = self.consume(TokenKind::Identifier, "Expected struct name")?.lexeme.clone();
        self.consume(TokenKind::Squad, "Expected 'squad' after struct name")?;
        
        self.consume(TokenKind::LeftBrace, "Expected '{' after 'squad'")?;
        let mut fields = Vec::new();
        
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            // Skip newlines in field declarations
            self.skip_newlines();
            if self.check(&TokenKind::RightBrace) {
                break;
            }
            
            let field = self.parse_struct_field()?;
            fields.push(field);
            
            // Optional semicolon after field
            if self.match_tokens(&[TokenKind::Semicolon]) {
                // Consume semicolon
            }
            
            // Skip newlines after field
            self.skip_newlines();
        }
        
        self.consume(TokenKind::RightBrace, "Expected '}' after struct fields")?;
        
        Ok(crate::ast::StructStatement {
            name,
            fields,
            visibility,
        })
    }

    fn parse_struct_field(&mut self) -> Result<crate::ast::StructField, CursedError> {
        let name = self.consume(TokenKind::Identifier, "Expected field name")?.lexeme.clone();
        
        // CURSED syntax: field_name field_type (e.g., "name tea", "age normie")
        let field_type = if self.check_type_token() {
            Some(self.advance().lexeme.clone())
        } else {
            None
        };
        
        Ok(crate::ast::StructField {
            name,
            field_type,
            visibility: crate::ast::Visibility::Private, // Default to private
        })
    }

    fn parse_struct_literal(&mut self, struct_name: String) -> Result<Expression, CursedError> {
        use crate::ast::{StructLiteralExpression, StructFieldAssignment};
        
        self.consume(TokenKind::LeftBrace, "Expected '{' for struct literal")?;
        let mut fields = Vec::new();
        
        // Handle empty struct literal
        if self.check(&TokenKind::RightBrace) {
            self.advance();
            return Ok(Expression::StructLiteral(StructLiteralExpression {
                struct_name,
                fields,
            }));
        }
        
        loop {
            // Skip newlines in field assignments
            self.skip_newlines();
            
            if self.check(&TokenKind::RightBrace) {
                break;
            }
            
            // Parse field assignment: field_name: value
            let field_name = self.consume(TokenKind::Identifier, "Expected field name")?.lexeme.clone();
            self.consume(TokenKind::Colon, "Expected ':' after field name")?;
            let value = self.parse_expression()?;
            
            fields.push(StructFieldAssignment { field_name, value });
            
            // Check for comma separator or end
            if self.match_tokens(&[TokenKind::Comma]) {
                continue;
            } else if self.check(&TokenKind::RightBrace) {
                break;
            } else {
                return Err(CursedError::syntax_error("Expected ',' or '}' in struct literal"));
            }
        }
        
        self.consume(TokenKind::RightBrace, "Expected '}' to close struct literal")?;
        
        Ok(Expression::StructLiteral(StructLiteralExpression {
            struct_name,
            fields,
        }))
    }

    fn parse_interface_statement_with_visibility(&mut self, visibility: crate::ast::Visibility) -> Result<crate::ast::InterfaceStatement, CursedError> {
        self.consume(TokenKind::Collab, "Expected 'collab'")?;
        let name = self.consume(TokenKind::Identifier, "Expected interface name")?.lexeme.clone();
        
        self.consume(TokenKind::LeftBrace, "Expected '{' after interface name")?;
        let mut methods = Vec::new();
        
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            let method = self.parse_method_signature()?;
            methods.push(method);
            
            // Optional semicolon after method signature
            if self.match_tokens(&[TokenKind::Semicolon]) {
                // Consume semicolon
            }
        }
        
        self.consume(TokenKind::RightBrace, "Expected '}' after interface methods")?;
        
        Ok(crate::ast::InterfaceStatement {
            name,
            methods,
            visibility,
        })
    }

    fn parse_method_signature(&mut self) -> Result<crate::ast::MethodSignature, CursedError> {
        // Expect 'slay' keyword for method declaration
        self.consume(TokenKind::Slay, "Expected 'slay' for method declaration")?;
        let name = self.consume(TokenKind::Identifier, "Expected method name")?.lexeme.clone();
        
        self.consume(TokenKind::LeftParen, "Expected '(' after method name")?;
        let mut parameters = Vec::new();
        
        if !self.check(&TokenKind::RightParen) {
            loop {
                let param = self.parse_parameter()?;
                parameters.push(param);
                
                if !self.match_tokens(&[TokenKind::Comma]) {
                    break;
                }
            }
        }
        
        self.consume(TokenKind::RightParen, "Expected ')' after parameters")?;
        
        // Optional return type (using space syntax like in test)
        let return_type = if self.check(&TokenKind::Identifier) {
            let type_name = self.advance().lexeme.clone();
            Some(type_name)
        } else {
            None
        };
        
        Ok(crate::ast::MethodSignature {
            name,
            parameters,
            return_type,
        })
    }

    fn parse_parameter(&mut self) -> Result<crate::ast::Parameter, CursedError> {
        let name = self.consume(TokenKind::Identifier, "Expected parameter name")?.lexeme.clone();
        
        // CURSED syntax: parameter name followed by type (e.g., "x lit", "name tea")
        // Also support colon syntax for compatibility (e.g., "x: lit", "name: tea")
        let param_type = if self.match_tokens(&[TokenKind::Colon]) {
            // Colon syntax: name: type
            let type_name = self.consume(TokenKind::Identifier, "Expected parameter type after colon")?.lexeme.clone();
            Some(type_name)
        } else if self.check(&TokenKind::Identifier) {
            // Space syntax: name type
            let type_name = self.advance().lexeme.clone();
            Some(type_name)
        } else {
            None
        };
        
        Ok(crate::ast::Parameter {
            name,
            param_type,
        })
    }

    fn parse_goroutine_statement(&mut self) -> Result<crate::ast::GoroutineStatement, CursedError> {
        log::debug!("🚀 Parsing goroutine statement");
        self.consume(TokenKind::Stan, "Expected 'stan'")?;
        
        // Parse the expression that represents the function call to run in the goroutine
        let expression = self.parse_expression()?;
        
        log::debug!("✅ Successfully parsed goroutine statement");
        Ok(crate::ast::GoroutineStatement {
            expression,
        })
    }

    fn parse_select_statement(&mut self) -> Result<crate::ast::SelectStatement, CursedError> {
        log::debug!("📺 Parsing select statement");
        self.consume(TokenKind::Select, "Expected 'select'")?;
        self.consume(TokenKind::LeftBrace, "Expected '{' after 'select'")?;
        
        let mut cases = Vec::new();
        let mut default_case = None;
        
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            if self.match_tokens(&[TokenKind::Mood]) {
                // Parse case: mood <-channel { statements } or mood channel <- value { statements }
                let operation = self.parse_channel_operation()?;
                self.consume(TokenKind::LeftBrace, "Expected '{' after channel operation")?;
                
                let mut case_body = Vec::new();
                while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
                    case_body.push(self.parse_statement()?);
                }
                
                self.consume(TokenKind::RightBrace, "Expected '}' after case body")?;
                
                cases.push(crate::ast::SelectCase {
                    operation: Box::new(operation),
                    body: case_body,
                });
            } else if self.match_tokens(&[TokenKind::Basic]) {
                // Parse default case: basic { statements }
                self.consume(TokenKind::LeftBrace, "Expected '{' after 'basic'")?;
                
                let mut default_body = Vec::new();
                while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
                    default_body.push(self.parse_statement()?);
                }
                
                self.consume(TokenKind::RightBrace, "Expected '}' after default case body")?;
                default_case = Some(default_body);
            } else {
                return Err(CursedError::parse_error("Expected 'mood' or 'basic' in select statement"));
            }
        }
        
        self.consume(TokenKind::RightBrace, "Expected '}' after select body")?;
        
        log::debug!("✅ Successfully parsed select statement with {} cases", cases.len());
        Ok(crate::ast::SelectStatement {
            cases,
            default_case,
        })
    }

    fn parse_channel_operation(&mut self) -> Result<crate::ast::Expression, CursedError> {
        log::debug!("📡 Parsing channel operation");
        
        if self.check(&TokenKind::LeftArrow) {
            // Receive: <-channel
            self.advance();
            let channel = Box::new(self.parse_primary()?);
            
            log::debug!("✅ Parsed channel receive operation");
            Ok(crate::ast::Expression::ChannelReceive(crate::ast::ChannelReceiveExpression {
                channel,
            }))
        } else {
            // Send: channel <- value
            let channel = Box::new(self.parse_primary()?);
            self.consume(TokenKind::LeftArrow, "Expected '<-' for channel send")?;
            let value = Box::new(self.parse_logical_or()?);
            
            log::debug!("✅ Parsed channel send operation");
            Ok(crate::ast::Expression::ChannelSend(crate::ast::ChannelSendExpression {
                channel,
                value,
            }))
        }
    }

    fn parse_panic_statement(&mut self) -> Result<crate::ast::PanicStatement, CursedError> {
        log::debug!("💀 Parsing panic statement");
        self.consume(TokenKind::YeetError, "Expected 'yeet_error'")?;
        
        // Parse the message expression
        let message = Box::new(self.parse_expression()?);
        
        log::debug!("✅ Successfully parsed panic statement");
        Ok(crate::ast::PanicStatement {
            message,
        })
    }

    fn parse_catch_statement(&mut self) -> Result<crate::ast::CatchStatement, CursedError> {
        log::debug!("🛡️ Parsing catch statement");
        self.consume(TokenKind::Catch, "Expected 'catch'")?;
        self.consume(TokenKind::LeftBrace, "Expected '{' after 'catch'")?;
        
        // Parse the protected block
        let mut protected_block = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            protected_block.push(self.parse_statement()?);
        }
        
        self.consume(TokenKind::RightBrace, "Expected '}' after catch body")?;
        
        // Optional recovery block
        let mut recovery_block = None;
        let mut error_variable = None;
        
        // Check for optional 'recover' block syntax: catch { ... } recover (error) { ... }
        if self.check(&TokenKind::Identifier) && self.peek().lexeme == "recover" {
            self.advance(); // consume 'recover'
            
            // Optional error variable: recover (error) { ... }
            if self.check(&TokenKind::LeftParen) {
                self.consume(TokenKind::LeftParen, "Expected '(' after 'recover'")?;
                let error_var = self.consume(TokenKind::Identifier, "Expected error variable name")?.lexeme.clone();
                error_variable = Some(error_var);
                self.consume(TokenKind::RightParen, "Expected ')' after error variable")?;
            }
            
            self.consume(TokenKind::LeftBrace, "Expected '{' after 'recover'")?;
            
            let mut recovery_statements = Vec::new();
            while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
                recovery_statements.push(self.parse_statement()?);
            }
            
            self.consume(TokenKind::RightBrace, "Expected '}' after recovery body")?;
            recovery_block = Some(recovery_statements);
        }
        
        log::debug!("✅ Successfully parsed catch statement");
        Ok(crate::ast::CatchStatement {
            protected_block,
            recovery_block,
            error_variable,
        })
    }

    /// Parse generic type parameters: <T, U: Clone, V: Debug + Send>
    fn parse_generic_parameters(&mut self) -> Result<Vec<crate::ast::TypeParameter>, CursedError> {
        let mut type_parameters = Vec::new();
        
        // Check if we have generic parameters (accept both Less and LeftAngle)
        if !self.check(&TokenKind::LeftAngle) && !self.check(&TokenKind::Less) {
            return Ok(type_parameters);
        }
        
        // Accept either LeftAngle or Less for generic parameters
        if self.check(&TokenKind::LeftAngle) {
            self.consume(TokenKind::LeftAngle, "Expected '<'")?;
        } else {
            self.consume(TokenKind::Less, "Expected '<'")?;
        }
        
        if !self.check(&TokenKind::RightAngle) && !self.check(&TokenKind::Greater) {
            loop {
                let name = self.consume(TokenKind::Identifier, "Expected type parameter name")?.lexeme.clone();
                let mut bounds = Vec::new();
                
                // Parse bounds: T: Clone + Debug
                if self.match_tokens(&[TokenKind::Colon]) {
                    loop {
                        let bound = self.consume(TokenKind::Identifier, "Expected trait bound")?.lexeme.clone();
                        bounds.push(bound);
                        
                        if !self.match_tokens(&[TokenKind::Plus]) {
                            break;
                        }
                    }
                }
                
                type_parameters.push(crate::ast::TypeParameter { name, bounds });
                
                if !self.match_tokens(&[TokenKind::Comma]) {
                    break;
                }
            }
        }
        
        // Accept either RightAngle or Greater for closing generic parameters
        if self.check(&TokenKind::RightAngle) {
            self.consume(TokenKind::RightAngle, "Expected '>'")?;
        } else {
            self.consume(TokenKind::Greater, "Expected '>'")?;
        }
        Ok(type_parameters)
    }

    /// Parse where clause: where T: Clone, U: Debug + Send
    fn parse_where_clause(&mut self) -> Result<Option<crate::ast::WhereClause>, CursedError> {
        if !self.check(&TokenKind::Where) {
            return Ok(None);
        }
        
        self.consume(TokenKind::Where, "Expected 'where'")?;
        let mut constraints = Vec::new();
        
        loop {
            let type_name = self.consume(TokenKind::Identifier, "Expected type name in where clause")?.lexeme.clone();
            self.consume(TokenKind::Colon, "Expected ':' after type name in where clause")?;
            
            let mut bounds = Vec::new();
            loop {
                let bound = self.consume(TokenKind::Identifier, "Expected trait bound in where clause")?.lexeme.clone();
                bounds.push(bound);
                
                if !self.match_tokens(&[TokenKind::Plus]) {
                    break;
                }
            }
            
            constraints.push(crate::ast::TypeConstraint { type_name, bounds });
            
            if !self.match_tokens(&[TokenKind::Comma]) {
                break;
            }
        }
        
        Ok(Some(crate::ast::WhereClause { constraints }))
    }
}

#[cfg(test)]
mod generic_tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_simple_generic_function() {
        let source = r#"
slay identity<T>(value) {
    yolo value
}
"#;
        
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let program = parser.parse().unwrap();
        
        assert_eq!(program.statements.len(), 1);
        
        if let Statement::Function(func) = &program.statements[0] {
            assert_eq!(func.name, "identity");
            assert_eq!(func.type_parameters.len(), 1);
            assert_eq!(func.type_parameters[0].name, "T");
            assert_eq!(func.type_parameters[0].bounds.len(), 0);
            assert!(func.where_clause.is_none());
        } else {
            panic!("Expected function statement");
        }
    }

    #[test]
    fn test_generic_function_with_bounds() {
        let source = r#"
slay compare<T: Clone + Debug>(a, b) {
    yolo true
}
"#;
        
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let program = parser.parse().unwrap();
        
        assert_eq!(program.statements.len(), 1);
        
        if let Statement::Function(func) = &program.statements[0] {
            assert_eq!(func.name, "compare");
            assert_eq!(func.type_parameters.len(), 1);
            assert_eq!(func.type_parameters[0].name, "T");
            assert_eq!(func.type_parameters[0].bounds.len(), 2);
            assert_eq!(func.type_parameters[0].bounds[0], "Clone");
            assert_eq!(func.type_parameters[0].bounds[1], "Debug");
            assert!(func.where_clause.is_none());
        } else {
            panic!("Expected function statement");
        }
    }



    #[test]
    fn test_generic_function_with_where_clause() {
        let source = r#"
slay complex<T, U>(x, y) where T: Clone, U: Debug + Send {
    yolo x
}
"#;
        
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let program = parser.parse().unwrap();
        
        assert_eq!(program.statements.len(), 1);
        
        if let Statement::Function(func) = &program.statements[0] {
            assert_eq!(func.name, "complex");
            assert_eq!(func.type_parameters.len(), 2);
            assert_eq!(func.type_parameters[0].name, "T");
            assert_eq!(func.type_parameters[1].name, "U");
            
            // Check where clause
            assert!(func.where_clause.is_some());
            let where_clause = func.where_clause.as_ref().unwrap();
            assert_eq!(where_clause.constraints.len(), 2);
            
            assert_eq!(where_clause.constraints[0].type_name, "T");
            assert_eq!(where_clause.constraints[0].bounds.len(), 1);
            assert_eq!(where_clause.constraints[0].bounds[0], "Clone");
            
            assert_eq!(where_clause.constraints[1].type_name, "U");
            assert_eq!(where_clause.constraints[1].bounds.len(), 2);
            assert_eq!(where_clause.constraints[1].bounds[0], "Debug");
            assert_eq!(where_clause.constraints[1].bounds[1], "Send");
        } else {
            panic!("Expected function statement");
        }
    }

    #[test]
    fn test_non_generic_function_still_works() {
        let source = r#"
slay simple(x) {
    yolo x
}
"#;
        
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let program = parser.parse().unwrap();
        
        assert_eq!(program.statements.len(), 1);
        
        if let Statement::Function(func) = &program.statements[0] {
            assert_eq!(func.name, "simple");
            assert_eq!(func.type_parameters.len(), 0);
            assert!(func.where_clause.is_none());
        } else {
            panic!("Expected function statement");
        }
    }
}

/// Create a new parser from source code
pub fn new_parser(source: &str) -> Result<Parser, CursedError> {
    let lexer = Lexer::new(source.to_string());
    Parser::new(lexer)
}

pub struct MinimalImplementation;

impl MinimalImplementation {
    pub fn new() -> Self {
        Self
    }
}

pub fn get_minimal_result() -> Result<String, CursedError> {
    Ok("CURSED advanced features enabled".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_basic_cursed_parsing() {
        println!("🧪 Testing basic CURSED parsing...");
        
        let simple_program = r#"
vibe main
sus x = 42
"#;
        
        let mut lexer = Lexer::new(simple_program.to_string());
        let tokens = lexer.tokenize().expect("Should tokenize successfully");
        
        println!("📋 Tokens created: {}", tokens.len());
        for (i, token) in tokens.iter().enumerate() {
            println!("  {}: {:?} = '{}'", i, token.kind, token.lexeme);
        }
        
        let mut parser = Parser::from_tokens(tokens);
        let result = parser.parse();
        
        match result {
            Ok(program) => {
                println!("✅ Basic parsing successful!");
                println!("📦 Package: {:?}", program.package);
                println!("📤 Imports: {}", program.imports.len());
                println!("📋 Statements: {}", program.statements.len());
                
                // Verify package declaration
                assert!(program.package.is_some(), "Should have package declaration");
                assert_eq!(program.package.unwrap().name, "main");
                
                // Should have at least one statement
                assert!(program.statements.len() > 0, "Should have at least one statement");
                
                println!("🎉 Basic CURSED parsing test passed!");
            },
            Err(e) => {
                println!("❌ Parsing failed: {}", e);
                let errors = parser.errors();
                if !errors.is_empty() {
                    println!("Parser errors:");
                    for error in errors {
                        println!("  - {}", error);
                    }
                }
                panic!("Basic parsing should succeed");
            }
        }
    }

    #[test]
    fn test_function_parsing_cursed() {
        println!("🧪 Testing CURSED function parsing...");
        
        let function_program = r#"
slay testFunc() {
    sus x = 42
    yolo x
}
"#;
        
        let mut lexer = Lexer::new(function_program.to_string());
        let tokens = lexer.tokenize().expect("Should tokenize successfully");
        
        let mut parser = Parser::from_tokens(tokens);
        let result = parser.parse();
        
        match result {
            Ok(program) => {
                println!("✅ Function parsing successful!");
                println!("📋 Statements: {}", program.statements.len());
                
                // Should have one function statement
                assert_eq!(program.statements.len(), 1, "Should have exactly one statement");
                
                match &program.statements[0] {
                    Statement::Function(func) => {
                        assert_eq!(func.name, "testFunc", "Function name should be 'testFunc'");
                        assert_eq!(func.parameters.len(), 0, "Function should have no parameters");
                        assert!(func.body.len() > 0, "Function should have body statements");
                        
                        println!("📝 Function '{}' parsed with {} body statements", func.name, func.body.len());
                        println!("🎉 Function parsing test passed!");
                    },
                    _ => panic!("Expected function statement, got {:?}", std::mem::discriminant(&program.statements[0])),
                }
            },
            Err(e) => {
                println!("❌ Function parsing failed: {}", e);
                let errors = parser.errors();
                if !errors.is_empty() {
                    println!("Parser errors:");
                    for error in errors {
                        println!("  - {}", error);
                    }
                }
                panic!("Function parsing should succeed");
            }
        }
    }

    #[test]
    fn test_lambda_expression_parsing() {
        println!("🧪 Testing lambda expression parsing...");
        
        // Test parameterless lambda
        let tokens = vec![
            Token { kind: TokenKind::PipePipe, lexeme: "||".to_string(), line: 1, column: 1 },
            Token { kind: TokenKind::LeftBrace, lexeme: "{".to_string(), line: 1, column: 3 },
            Token { kind: TokenKind::Number, lexeme: "42".to_string(), line: 1, column: 5 },
            Token { kind: TokenKind::RightBrace, lexeme: "}".to_string(), line: 1, column: 7 },
            Token { kind: TokenKind::Eof, lexeme: "".to_string(), line: 1, column: 8 },
        ];
        
        let mut parser = Parser::from_tokens(tokens);
        match parser.parse_expression() {
            Ok(expr) => {
                if let Expression::Lambda(lambda) = expr {
                    assert_eq!(lambda.parameters.len(), 0, "Parameterless lambda should have no parameters");
                    println!("✅ Parameterless lambda parsed successfully");
                } else {
                    panic!("Expected lambda expression, got {:?}", std::mem::discriminant(&expr));
                }
            },
            Err(e) => {
                println!("❌ Parameterless lambda parsing failed: {}", e);
                panic!("Parameterless lambda parsing should succeed");
            }
        }
        
        // Test lambda with parameters
        let tokens = vec![
            Token { kind: TokenKind::Pipe, lexeme: "|".to_string(), line: 1, column: 1 },
            Token { kind: TokenKind::Identifier, lexeme: "x".to_string(), line: 1, column: 2 },
            Token { kind: TokenKind::Comma, lexeme: ",".to_string(), line: 1, column: 3 },
            Token { kind: TokenKind::Identifier, lexeme: "y".to_string(), line: 1, column: 5 },
            Token { kind: TokenKind::Pipe, lexeme: "|".to_string(), line: 1, column: 6 },
            Token { kind: TokenKind::LeftBrace, lexeme: "{".to_string(), line: 1, column: 8 },
            Token { kind: TokenKind::Identifier, lexeme: "x".to_string(), line: 1, column: 10 },
            Token { kind: TokenKind::Plus, lexeme: "+".to_string(), line: 1, column: 12 },
            Token { kind: TokenKind::Identifier, lexeme: "y".to_string(), line: 1, column: 14 },
            Token { kind: TokenKind::RightBrace, lexeme: "}".to_string(), line: 1, column: 15 },
            Token { kind: TokenKind::Eof, lexeme: "".to_string(), line: 1, column: 16 },
        ];
        
        let mut parser = Parser::from_tokens(tokens);
        match parser.parse_expression() {
            Ok(expr) => {
                if let Expression::Lambda(lambda) = expr {
                    assert_eq!(lambda.parameters.len(), 2, "Lambda should have 2 parameters");
                    assert_eq!(lambda.parameters[0], "x", "First parameter should be 'x'");
                    assert_eq!(lambda.parameters[1], "y", "Second parameter should be 'y'");
                    println!("✅ Lambda with parameters parsed successfully");
                } else {
                    panic!("Expected lambda expression, got {:?}", std::mem::discriminant(&expr));
                }
            },
            Err(e) => {
                println!("❌ Lambda with parameters parsing failed: {}", e);
                panic!("Lambda with parameters parsing should succeed");
            }
        }
        
        println!("🎉 Lambda expression parsing tests passed!");
    }
}
