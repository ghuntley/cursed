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
            _ => {
                if visibility != crate::ast::Visibility::Private {
                    return Err(CursedError::parse_error("Visibility modifiers not allowed on expressions"));
                }
                Ok(Statement::Expression(self.parse_expression()?))
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

    pub fn parse_expression(&mut self) -> Result<Expression, CursedError> {
        self.parse_logical_or()
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
