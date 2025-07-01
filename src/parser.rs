//! Parser module for CURSED language

use crate::error::CursedError;
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
            Err(CursedError::syntax_error(message))
        }
    }

    fn skip_newlines(&mut self) {
        while self.match_tokens(&[TokenKind::Newline]) {
            // Skip newlines
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
        let name = self.consume(TokenKind::Identifier, "Expected package name")?;
        Ok(PackageDeclaration {
            name: name.lexeme.clone(),
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
            TokenKind::Squad => {
                log::info!("📝 Parsing struct statement with 'squad' keyword");
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
                
                // Check if this is an assignment (identifier = expression)
                if self.check(&TokenKind::Identifier) && self.peek_ahead(1).kind == TokenKind::Equal {
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
        
        self.consume(TokenKind::LeftParen, "Expected '(' after function name")?;
        let mut parameters = Vec::new();
        
        if !self.check(&TokenKind::RightParen) {
            loop {
                let param = self.consume(TokenKind::Identifier, "Expected parameter name")?.lexeme.clone();
                parameters.push(param);
                
                if !self.match_tokens(&[TokenKind::Comma]) {
                    break;
                }
            }
        }
        
        self.consume(TokenKind::RightParen, "Expected ')' after parameters")?;
        self.consume(TokenKind::LeftBrace, "Expected '{' before function body")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            body.push(self.parse_statement()?);
            self.skip_newlines();
        }
        
        self.consume(TokenKind::RightBrace, "Expected '}' after function body")?;
        
        Ok(FunctionStatement {
            name,
            parameters,
            body,
            return_type: None, // TODO: Add proper type parsing
            visibility: crate::ast::Visibility::Private,
        })
    }

    fn parse_function_statement_with_visibility(&mut self, visibility: crate::ast::Visibility) -> Result<FunctionStatement, CursedError> {
        self.consume(TokenKind::Slay, "Expected 'slay'")?;
        let name = self.consume(TokenKind::Identifier, "Expected function name")?.lexeme.clone();
        
        self.consume(TokenKind::LeftParen, "Expected '(' after function name")?;
        let mut parameters = Vec::new();
        
        if !self.check(&TokenKind::RightParen) {
            loop {
                let param = self.consume(TokenKind::Identifier, "Expected parameter name")?.lexeme.clone();
                parameters.push(param);
                
                if !self.match_tokens(&[TokenKind::Comma]) {
                    break;
                }
            }
        }
        
        self.consume(TokenKind::RightParen, "Expected ')' after parameters")?;
        self.consume(TokenKind::LeftBrace, "Expected '{' before function body")?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        log::debug!("🔧 Parsing function body for: {}", name);
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            let stmt = self.parse_statement()?;
            log::debug!("➕ Adding statement to function body: {:?}", std::mem::discriminant(&stmt));
            body.push(stmt);
            self.skip_newlines();
        }
        log::debug!("✅ Function body parsing complete for: {}", name);
        
        self.consume(TokenKind::RightBrace, "Expected '}' after function body")?;
        
        Ok(FunctionStatement {
            name,
            parameters,
            body,
            return_type: None, // TODO: Add proper type parsing
            visibility,
        })
    }

    fn parse_let_statement(&mut self) -> Result<LetStatement, CursedError> {
        self.consume(TokenKind::Sus, "Expected 'sus'")?;
        let name = self.consume(TokenKind::Identifier, "Expected variable name")?.lexeme.clone();
        self.consume(TokenKind::Equal, "Expected '=' after variable name")?;
        let value = self.parse_expression()?;
        
        Ok(LetStatement {
            name,
            value,
            visibility: crate::ast::Visibility::Private,
        })
    }

    fn parse_assignment_statement(&mut self) -> Result<AssignmentStatement, CursedError> {
        let name = self.consume(TokenKind::Identifier, "Expected variable name")?.lexeme.clone();
        self.consume(TokenKind::Equal, "Expected '=' after variable name")?;
        let value = self.parse_expression()?;
        
        Ok(AssignmentStatement {
            name,
            value,
        })
    }

    fn parse_const_statement(&mut self) -> Result<LetStatement, CursedError> {
        self.consume(TokenKind::Facts, "Expected 'facts'")?;
        let name = self.consume(TokenKind::Identifier, "Expected constant name")?.lexeme.clone();
        self.consume(TokenKind::Equal, "Expected '=' after constant name")?;
        let value = self.parse_expression()?;
        
        Ok(LetStatement {
            name,
            value,
            visibility: crate::ast::Visibility::Private,
        })
    }

    fn parse_let_statement_with_visibility(&mut self, visibility: crate::ast::Visibility) -> Result<LetStatement, CursedError> {
        self.consume(TokenKind::Sus, "Expected 'sus'")?;
        let name = self.consume(TokenKind::Identifier, "Expected variable name")?.lexeme.clone();
        self.consume(TokenKind::Equal, "Expected '=' after variable name")?;
        let value = self.parse_expression()?;
        
        Ok(LetStatement {
            name,
            value,
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
        let condition = self.parse_expression()?;
        self.consume(TokenKind::LeftBrace, "Expected '{' after if condition")?;
        
        let mut then_branch = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            then_branch.push(self.parse_statement()?);
        }
        
        self.consume(TokenKind::RightBrace, "Expected '}' after if body")?;
        
        let mut else_branch = None;
        if self.match_tokens(&[TokenKind::Highkey]) {
            self.consume(TokenKind::LeftBrace, "Expected '{' after else")?;
            let mut else_stmts = Vec::new();
            while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
                else_stmts.push(self.parse_statement()?);
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
        self.consume(TokenKind::Yolo, "Expected 'yolo'")?;
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
        
        let mut cases = Vec::new();
        let mut default_case = None;
        
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            if self.match_tokens(&[TokenKind::Mood]) {
                // Parse case: mood pattern { statements }
                let pattern = self.parse_expression()?;
                self.consume(TokenKind::LeftBrace, "Expected '{' after case pattern")?;
                
                let mut case_body = Vec::new();
                while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
                    case_body.push(self.parse_statement()?);
                }
                
                self.consume(TokenKind::RightBrace, "Expected '}' after case body")?;
                
                cases.push(SwitchCase {
                    pattern,
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
                return Err(CursedError::parse_error("Expected 'mood' or 'basic' in switch statement"));
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
            } else if self.match_tokens(&[TokenKind::Dot]) {
                let property = self.consume(TokenKind::Identifier, "Expected property name after '.'")?;
                expr = Expression::MemberAccess(MemberAccessExpression {
                    object: Box::new(expr),
                    property: property.lexeme.clone(),
                });
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
        } else if self.match_tokens(&[TokenKind::Boolean]) { // Handle 'lit' and other keywords that might be tokenized as Boolean
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
        match &self.peek().kind {
            TokenKind::Boolean => {
                let token = self.advance();
                let value = match token.lexeme.as_str() {
                    "true" | "based" => true,
                    "false" | "lies" => false,
                    _ => false,
                };
                Ok(Expression::Boolean(value))
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
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(TokenKind::RightParen, "Expected ')' after expression")?;
                Ok(expr)
            },
            TokenKind::Dm => {
                self.parse_channel_creation()
            },
            _ => Err(CursedError::syntax_error("Expected expression")),
        }
    }

    fn parse_struct_statement_with_visibility(&mut self, visibility: crate::ast::Visibility) -> Result<crate::ast::StructStatement, CursedError> {
        self.consume(TokenKind::Squad, "Expected 'squad'")?;
        let name = self.consume(TokenKind::Identifier, "Expected struct name")?.lexeme.clone();
        
        self.consume(TokenKind::LeftBrace, "Expected '{' after struct name")?;
        let mut fields = Vec::new();
        
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            let field = self.parse_struct_field()?;
            fields.push(field);
            
            // Optional comma
            if self.match_tokens(&[TokenKind::Comma]) {
                // Consume comma
            }
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
        
        // Optional type annotation
        let field_type = if self.match_tokens(&[TokenKind::Colon]) {
            let type_name = self.consume(TokenKind::Identifier, "Expected type name")?.lexeme.clone();
            Some(type_name)
        } else {
            None
        };
        
        Ok(crate::ast::StructField {
            name,
            field_type,
            visibility: crate::ast::Visibility::Private, // Default to private
        })
    }

    fn parse_interface_statement_with_visibility(&mut self, visibility: crate::ast::Visibility) -> Result<crate::ast::InterfaceStatement, CursedError> {
        self.consume(TokenKind::Collab, "Expected 'collab'")?;
        let name = self.consume(TokenKind::Identifier, "Expected interface name")?.lexeme.clone();
        
        self.consume(TokenKind::LeftBrace, "Expected '{' after interface name")?;
        let mut methods = Vec::new();
        
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            let method = self.parse_method_signature()?;
            methods.push(method);
        }
        
        self.consume(TokenKind::RightBrace, "Expected '}' after interface methods")?;
        
        Ok(crate::ast::InterfaceStatement {
            name,
            methods,
            visibility,
        })
    }

    fn parse_method_signature(&mut self) -> Result<crate::ast::MethodSignature, CursedError> {
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
        
        // Optional return type (using colon for now)
        let return_type = if self.match_tokens(&[TokenKind::Colon]) {
            let type_name = self.consume(TokenKind::Identifier, "Expected return type")?.lexeme.clone();
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
        
        // Optional type annotation
        let param_type = if self.match_tokens(&[TokenKind::Colon]) {
            let type_name = self.consume(TokenKind::Identifier, "Expected parameter type")?.lexeme.clone();
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
