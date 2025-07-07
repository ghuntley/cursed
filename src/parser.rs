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

        // Parse statements (and any additional imports)
        while !self.is_at_end() {
            self.skip_newlines();
            if self.is_at_end() {
                break;
            }
            
            // Check for imports that can appear anywhere
            if self.match_tokens(&[TokenKind::Yeet]) {
                match self.parse_import_statement() {
                    Ok(import) => {
                        log::debug!("➕ Adding import to program: {:?}", import);
                        imports.push(import);
                    },
                    Err(e) => {
                        log::error!("❌ Import parse error: {}", e);
                        self.errors.push(format!("Import parse error: {}", e));
                        self.synchronize();
                    }
                }
                self.skip_newlines();
                continue;
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
            TokenKind::Sip |
            TokenKind::Smol |
            TokenKind::Mid |
            TokenKind::Thicc |
            TokenKind::Snack |
            TokenKind::Meal |
            TokenKind::Byte |
            TokenKind::Rune |
            TokenKind::Extra |
            TokenKind::Lit |
            TokenKind::Dm |
            TokenKind::Truth |
            TokenKind::Lies |
            TokenKind::Cap |
            TokenKind::LeftBracket |  // For array/slice types
            TokenKind::At             // For pointer types
        )
    }

    /// Parse a type expression
    fn parse_type(&mut self) -> Result<crate::ast::Type, CursedError> {
        match self.peek().kind {
            TokenKind::At => {
                // Pointer type: @T
                self.advance(); // consume '@'
                let inner_type = self.parse_type()?;
                Ok(crate::ast::Type::Pointer(Box::new(inner_type)))
            }
            TokenKind::LeftBracket => {
                self.advance(); // consume '['
                
                // Check if it's a slice type []T or array type [N]T
                if self.check(&TokenKind::RightBracket) {
                    // Slice type: []T
                    self.advance(); // consume ']'
                    let element_type = self.parse_type()?;
                    Ok(crate::ast::Type::Slice(Box::new(element_type)))
                } else {
                    // Array type: [N]T - parse the size expression
                    let size_expr = self.parse_expression()?;
                    self.consume(TokenKind::RightBracket, "Expected ']' after array size")?;
                    let element_type = self.parse_type()?;
                    Ok(crate::ast::Type::Array(Box::new(element_type), Some(Box::new(size_expr))))
                }
            }
            TokenKind::Dm => {
                self.advance(); // consume 'dm'
                let element_type = self.parse_type()?;
                Ok(crate::ast::Type::Dm(Box::new(element_type)))
            }
            TokenKind::LeftParen => {
                // Tuple type: (T1, T2, ...)
                self.advance(); // consume '('
                let mut types = Vec::new();
                
                if !self.check(&TokenKind::RightParen) {
                    loop {
                        types.push(self.parse_type()?);
                        if !self.check(&TokenKind::Comma) {
                            break;
                        }
                        self.advance(); // consume ','
                    }
                }
                
                self.consume(TokenKind::RightParen, "Expected ')' after tuple type")?;
                Ok(crate::ast::Type::Tuple(types))
            }
            // Basic types
            TokenKind::Normie => {
                self.advance();
                Ok(crate::ast::Type::Normie)
            }
            TokenKind::Tea => {
                self.advance();
                Ok(crate::ast::Type::Tea)
            }
            TokenKind::Sip => {
                self.advance();
                Ok(crate::ast::Type::Sip)
            }
            TokenKind::Smol => {
                self.advance();
                Ok(crate::ast::Type::Smol)
            }
            TokenKind::Mid => {
                self.advance();
                Ok(crate::ast::Type::Mid)
            }
            TokenKind::Thicc => {
                self.advance();
                Ok(crate::ast::Type::Thicc)
            }
            TokenKind::Snack => {
                self.advance();
                Ok(crate::ast::Type::Snack)
            }
            TokenKind::Meal => {
                self.advance();
                Ok(crate::ast::Type::Meal)
            }
            TokenKind::Byte => {
                self.advance();
                Ok(crate::ast::Type::Byte)
            }
            TokenKind::Rune => {
                self.advance();
                Ok(crate::ast::Type::Rune)
            }
            TokenKind::Extra => {
                self.advance();
                Ok(crate::ast::Type::Extra)
            }
            TokenKind::Lit => {
                self.advance();
                Ok(crate::ast::Type::Lit)
            }
            TokenKind::Truth => {
                self.advance();
                Ok(crate::ast::Type::Lit)
            }
            TokenKind::Lies => {
                self.advance();
                Ok(crate::ast::Type::Lit)
            }
            TokenKind::Cap => {
                self.advance();
                Ok(crate::ast::Type::Lit)
            }
            _ => {
                // Custom type (identifier)
                let token = self.advance();
                Ok(crate::ast::Type::Custom(token.lexeme.clone()))
            }
        }
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
            TokenKind::Tea | TokenKind::Sip | TokenKind::Smol | TokenKind::Mid | TokenKind::Thicc |
            TokenKind::Snack | TokenKind::Meal | TokenKind::Byte | TokenKind::Rune | TokenKind::Extra |
            TokenKind::Lit | TokenKind::Cap | TokenKind::NoCap | TokenKind::Truth |
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
    
    fn is_sus_function_declaration(&self) -> bool {
        // Look ahead to see if this is a function declaration (sus identifier() ...) 
        // or a variable declaration (sus identifier type = ...)
        if self.current + 1 < self.tokens.len() && self.tokens[self.current].kind == TokenKind::Sus {
            if self.current + 2 < self.tokens.len() && self.tokens[self.current + 1].kind == TokenKind::Identifier {
                // Check if the next token after the identifier is a left parenthesis (function)
                return self.tokens[self.current + 2].kind == TokenKind::LeftParen;
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

    /// Enhanced synchronization with error recovery suggestions
    fn synchronize_with_suggestions(&mut self, error_context: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        let current_token = self.peek();
        
        // Analyze current context for recovery suggestions
        match current_token.kind {
            TokenKind::Identifier if error_context.contains("expected") => {
                let name = &current_token.lexeme;
                suggestions.push(format!("Did you mean to declare a variable? Try: sus {} = value", name));
                suggestions.push(format!("Or define a function? Try: slay {}() {{ ... }}", name));
            }
            TokenKind::LeftBrace => {
                if error_context.contains("function") {
                    suggestions.push("This looks like a function body. Make sure the function signature is correct".to_string());
                } else {
                    suggestions.push("This looks like a block. Make sure it's properly structured".to_string());
                }
            }
            TokenKind::RightBrace => {
                suggestions.push("Unmatched closing brace. Check for missing opening brace".to_string());
            }
            _ => {}
        }
        
        // Perform standard synchronization
        self.synchronize();
        
        suggestions
    }

    /// Expression-level error recovery
    fn recover_expression(&mut self) -> Result<Expression, CursedError> {
        let token = self.peek();
        
        match token.kind {
            TokenKind::Identifier => {
                // Try to create a variable reference as fallback
                let name = token.lexeme.clone();
                self.advance();
                Ok(Expression::Variable(name))
            }
            TokenKind::Number => {
                let value = token.lexeme.parse::<i64>().unwrap_or(0);
                self.advance();
                Ok(Expression::Literal(Literal::Integer(value)))
            }
            TokenKind::String => {
                let lexeme = token.lexeme.clone();
                self.advance();
                Ok(Expression::Literal(Literal::String(lexeme)))
            }
            TokenKind::Truth => {
                self.advance();
                Ok(Expression::Literal(Literal::Boolean(true)))
            }
            TokenKind::Cap => {
                self.advance();
                Ok(Expression::Literal(Literal::Boolean(false)))
            }
            _ => {
                // Create a placeholder expression and skip problematic token
                self.advance();
                Ok(Expression::Literal(Literal::Integer(0))) // Safe fallback
            }
        }
    }

    /// Statement-level error recovery
    fn recover_statement(&mut self) -> Result<Statement, CursedError> {
        // Skip to next statement boundary
        self.synchronize();
        
        // Return a no-op statement as fallback
        Ok(Statement::Expression(Expression::Literal(Literal::Integer(0))))
    }

    /// Enhanced error reporting with context
    fn report_error_with_context(&mut self, error: CursedError, context: &str) -> StructuredError {
        let token = self.peek();
        let line = token.line;
        let column = token.column;
        
        let structured_error = match &error {
            CursedError::SyntaxError(msg) => {
                StructuredError::new(ErrorCode::E0001, msg.clone())
                    .with_location(ErrorSourceLocation {
                        file: "".to_string(),
                        line,
                        column,
                        length: token.lexeme.len(),
                        source_line: None,
                    })
            }
            CursedError::TypeError(msg) => {
                StructuredError::new(ErrorCode::E0100, msg.clone())
                    .with_location(ErrorSourceLocation {
                        file: "".to_string(),
                        line,
                        column,
                        length: token.lexeme.len(),
                        source_line: None,
                    })
            }
            _ => {
                StructuredError::new(ErrorCode::E0001, error.to_string())
                    .with_location(ErrorSourceLocation {
                        file: "".to_string(),
                        line,
                        column,
                        length: token.lexeme.len(),
                        source_line: None,
                    })
            }
        };
        
        // Generate context-specific suggestions
        let suggestions = self.generate_error_suggestions(&error, context);
        let final_error = structured_error.with_suggestions(suggestions);
        
        // Store for later reporting
        self.errors.push(final_error.to_string());
        
        final_error
    }

    /// Generate context-specific error suggestions
    fn generate_error_suggestions(&mut self, error: &CursedError, context: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        let current_token = self.peek();
        
        match error {
            CursedError::SyntaxError(msg) => {
                if msg.contains("expected") {
                    if msg.contains("')'") {
                        suggestions.push("Check for missing closing parenthesis".to_string());
                        suggestions.push("Make sure all function calls are properly closed".to_string());
                    } else if msg.contains("';'") {
                        suggestions.push("Add a semicolon at the end of the statement".to_string());
                    } else if msg.contains("'{'") {
                        suggestions.push("Add opening brace for block statement".to_string());
                    } else if msg.contains("'}'") {
                        suggestions.push("Add closing brace to end the block".to_string());
                    }
                }
                
                // Token-specific suggestions
                match current_token.kind {
                    TokenKind::Identifier => {
                        let name = &current_token.lexeme;
                        suggestions.push(format!("If '{}' is a variable, make sure it's declared with 'sus'", name));
                        suggestions.push(format!("If '{}' is a function, make sure it's defined with 'slay'", name));
                    }
                    TokenKind::LeftParen => {
                        suggestions.push("This looks like a function call. Make sure the function exists".to_string());
                    }
                    TokenKind::LeftBrace => {
                        suggestions.push("This looks like a block. Make sure it's in the right context".to_string());
                    }
                    _ => {}
                }
            }
            CursedError::TypeError(msg) => {
                suggestions.push("Check the types of values in this expression".to_string());
                suggestions.push("Consider using type assertions with .() syntax".to_string());
                
                if msg.contains("mismatch") {
                    suggestions.push("Use explicit type conversion if needed".to_string());
                }
            }
            _ => {}
        }
        
        // Add context-specific suggestions
        if context.contains("function") {
            suggestions.push("Make sure function parameters and return types are correct".to_string());
        } else if context.contains("variable") {
            suggestions.push("Check variable declaration syntax: sus name type = value".to_string());
        }
        
        suggestions
    }

    /// Check for common typos and suggest corrections
    fn suggest_corrections(&self, token: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        // Common CURSED keyword corrections
        let keywords = [
            ("sus", vec!["sus", "let", "var"]),
            ("slay", vec!["slay", "fn", "func", "function"]),
            ("facts", vec!["facts", "struct"]),
            ("lowkey", vec!["lowkey", "for"]),
            ("bestie", vec!["bestie", "if"]),
            ("yolo", vec!["yolo", "while"]),
            ("vibez", vec!["vibez", "console", "print"]),
            ("based", vec!["based", "true"]),
            ("cap", vec!["cap", "false", "nil", "null"]),
            ("damn", vec!["damn", "return"]),
            ("ghosted", vec!["ghosted", "break"]),
            ("simp", vec!["simp", "continue"]),
        ];
        
        for (correct, alternatives) in &keywords {
            if alternatives.contains(&token) && token != *correct {
                suggestions.push(format!("Did you mean '{}'?", correct));
            }
        }
        
        // Levenshtein distance for typos
        for (correct, _) in &keywords {
            if Self::levenshtein_distance(token, correct) <= 2 && token != *correct {
                suggestions.push(format!("Did you mean '{}'?", correct));
            }
        }
        
        suggestions
    }

    /// Calculate Levenshtein distance for typo detection
    fn levenshtein_distance(a: &str, b: &str) -> usize {
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();
        let a_len = a_chars.len();
        let b_len = b_chars.len();
        
        if a_len == 0 { return b_len; }
        if b_len == 0 { return a_len; }
        
        let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];
        
        for i in 0..=a_len {
            matrix[i][0] = i;
        }
        for j in 0..=b_len {
            matrix[0][j] = j;
        }
        
        for i in 1..=a_len {
            for j in 1..=b_len {
                let cost = if a_chars[i-1] == b_chars[j-1] { 0 } else { 1 };
                matrix[i][j] = std::cmp::min(
                    std::cmp::min(matrix[i-1][j] + 1, matrix[i][j-1] + 1),
                    matrix[i-1][j-1] + cost
                );
            }
        }
        
        matrix[a_len][b_len]
    }

    /// Enhanced error handling with recovery for missing tokens
    fn handle_missing_token(&mut self, expected: TokenKind, context: &str) -> Result<(), CursedError> {
        let error_msg = format!("Expected {:?}, found {:?}", expected, self.peek().kind);
        
        // Try to recover by inserting the missing token conceptually
        match expected {
            TokenKind::Semicolon => {
                // Often optional in CURSED, so just continue
                return Ok(());
            }
            TokenKind::RightParen => {
                // Critical for parsing, but try to continue
                let suggestions = vec![
                    "Add closing parenthesis ')'".to_string(),
                    "Check for nested parentheses".to_string(),
                ];
                self.errors.push(format!("{} - {}", error_msg, suggestions.join(", ")));
                return Ok(());
            }
            TokenKind::RightBrace => {
                // Critical for block structure
                let suggestions = vec![
                    "Add closing brace '}'".to_string(),
                    "Check block structure".to_string(),
                ];
                self.errors.push(format!("{} - {}", error_msg, suggestions.join(", ")));
                return Ok(());
            }
            _ => {
                return Err(CursedError::SyntaxError(error_msg));
            }
        }
    }

    /// Recovery method for errors within blocks that doesn't escape to top-level
    fn recover_within_block(&mut self) {
        self.advance();

        while !self.is_at_end() {
            // Stop at statement boundaries within the current block
            if self.previous().kind == TokenKind::Semicolon || 
               self.previous().kind == TokenKind::Newline {
                return;
            }

            // Stop at the start of new statements but don't leave the block
            match self.peek().kind {
                TokenKind::Sus | TokenKind::Facts | TokenKind::Lowkey |
                TokenKind::Bestie | TokenKind::Yolo | TokenKind::Newline => return,
                // Don't stop at RightBrace since we want to stay within the current block context
                TokenKind::RightBrace => return,
                _ => {}
            }

            self.advance();
        }
    }

    /// Look ahead to determine if { starts a struct literal
    /// Real struct literals look like: { field: value, field2: value2 }
    /// vs blocks that are just: { statements... }
    fn looks_like_struct_literal(&self) -> bool {
        if !self.check(&TokenKind::LeftBrace) {
            return false;
        }
        
        // Look ahead past the opening brace
        let mut offset = 1;
        
        // Skip newlines
        while self.peek_ahead(offset).kind == TokenKind::Newline {
            offset += 1;
        }
        
        // Check if we see identifier : pattern (struct field assignment)
        if self.peek_ahead(offset).kind == TokenKind::Identifier &&
           self.peek_ahead(offset + 1).kind == TokenKind::Colon {
            return true;
        }
        
        // Empty braces could be empty struct literal
        if self.peek_ahead(offset).kind == TokenKind::RightBrace {
            return true;
        }
        
        // Otherwise, probably not a struct literal
        false
    }

    // Parsing methods
    fn parse_package_declaration(&mut self) -> Result<PackageDeclaration, CursedError> {
        let name_token = self.consume(TokenKind::Identifier, "Expected package name")?;
        let name = name_token.lexeme.clone();
        
        // Check for optional version string
        let version = if self.check(&TokenKind::String) {
            let version_token = self.advance();
            Some(version_token.lexeme.clone())
        } else {
            None
        };
        
        // Consume optional semicolon after package declaration
        if self.check(&TokenKind::Semicolon) {
            self.advance();
        }
        
        Ok(PackageDeclaration {
            name,
            version,
        })
    }

    fn parse_import_statement(&mut self) -> Result<ImportStatement, CursedError> {
        // Handle grouped imports: yeet ( "path1"; "path2"; "path3" )
        if self.check(&TokenKind::LeftParen) {
            self.advance(); // consume '('
            let mut paths = Vec::new();
            
            self.skip_newlines(); // Skip newlines after opening paren
            
            while !self.check(&TokenKind::RightParen) && !self.is_at_end() {
                let path_token = self.consume(TokenKind::String, "Expected import path")?;
                paths.push(path_token.lexeme.clone());
                
                // Check for semicolon or right paren
                if self.check(&TokenKind::Semicolon) {
                    self.advance(); // consume semicolon
                    self.skip_newlines(); // Skip newlines after semicolon
                } else {
                    self.skip_newlines(); // Skip newlines before checking for right paren
                    if !self.check(&TokenKind::RightParen) {
                        return Err(CursedError::parse_error("Expected ';' between grouped imports or ')' to close"));
                    }
                }
            }
            
            self.consume(TokenKind::RightParen, "Expected ')' after grouped imports")?;
            
            // Return a grouped import statement
            return Ok(ImportStatement {
                path: "".to_string(), // Empty path for grouped imports
                alias: None,
                items: paths, // Store the paths in the items field
            });
        }
        
        // Handle selective imports: yeet {Symbol1, Symbol2} from "path"
        if self.check(&TokenKind::LeftBrace) {
            self.advance(); // consume '{'
            let mut items = Vec::new();
            
            while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
                let item = self.consume(TokenKind::Identifier, "Expected identifier")?;
                items.push(item.lexeme.clone());
                
                if !self.check(&TokenKind::RightBrace) {
                    self.consume(TokenKind::Comma, "Expected ',' between imports")?;
                }
            }
            
            self.consume(TokenKind::RightBrace, "Expected '}'")?;
            self.consume(TokenKind::Identifier, "Expected 'from'")?; // consume 'from'
            let path_token = self.consume(TokenKind::String, "Expected import path")?;
            
            return Ok(ImportStatement {
                path: path_token.lexeme.clone(),
                alias: None,
                items,
            });
        }
        
        // Handle wildcard imports: yeet * from "path"
        if self.check(&TokenKind::Star) {
            self.advance(); // consume '*'
            self.consume(TokenKind::Identifier, "Expected 'from'")?; // consume 'from'
            let path_token = self.consume(TokenKind::String, "Expected import path")?;
            
            return Ok(ImportStatement {
                path: path_token.lexeme.clone(),
                alias: Some("*".to_string()),
                items: vec!["*".to_string()],
            });
        }
        
        // Handle aliased imports: yeet alias "path" or basic imports: yeet "path"
        if self.check(&TokenKind::Identifier) {
            let alias_token = self.advance(); // consume identifier
            let alias = alias_token.lexeme.clone();
            let path_token = self.consume(TokenKind::String, "Expected import path")?;
            
            return Ok(ImportStatement {
                path: path_token.lexeme.clone(),
                alias: Some(alias),
                items: Vec::new(),
            });
        }
        
        // Basic import: yeet "path"
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
            TokenKind::Sus => {
                // Look ahead to determine if this is a function declaration or variable declaration
                if self.is_sus_function_declaration() {
                    log::info!("📝 Parsing function statement with 'sus' keyword");
                    Ok(Statement::Function(self.parse_function_statement_with_visibility(visibility)?))
                } else {
                    log::info!("📝 Parsing variable statement with 'sus' keyword");
                    Ok(Statement::Let(self.parse_let_statement_with_visibility(visibility)?))
                }
            },
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
                self.parse_for_statement()
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
            TokenKind::Later => {
                if visibility != crate::ast::Visibility::Private {
                    return Err(CursedError::parse_error("Visibility modifiers not allowed on defer statements"));
                }
                log::info!("📝 Parsing defer statement with 'later' keyword");
                Ok(Statement::Defer(self.parse_defer_statement()?))
            },
            TokenKind::Ghosted => {
                if visibility != crate::ast::Visibility::Private {
                    return Err(CursedError::parse_error("Visibility modifiers not allowed on break statements"));
                }
                log::info!("📝 Parsing break statement with 'ghosted' keyword");
                Ok(Statement::Break(self.parse_break_statement()?))
            },
            TokenKind::Simp => {
                if visibility != crate::ast::Visibility::Private {
                    return Err(CursedError::parse_error("Visibility modifiers not allowed on continue statements"));
                }
                log::info!("📝 Parsing continue statement with 'simp' keyword");
                Ok(Statement::Continue(self.parse_continue_statement()?))
            },
            TokenKind::PlusPlus => {
                if visibility != crate::ast::Visibility::Private {
                    return Err(CursedError::parse_error("Visibility modifiers not allowed on increment statements"));
                }
                log::info!("📝 Parsing prefix increment statement");
                Ok(Statement::Increment(self.parse_prefix_increment_statement()?))
            },
            TokenKind::MinusMinus => {
                if visibility != crate::ast::Visibility::Private {
                    return Err(CursedError::parse_error("Visibility modifiers not allowed on decrement statements"));
                }
                log::info!("📝 Parsing prefix decrement statement");
                Ok(Statement::Decrement(self.parse_prefix_decrement_statement()?))
            },
            _ => {
                if visibility != crate::ast::Visibility::Private {
                    return Err(CursedError::parse_error("Visibility modifiers not allowed on expressions"));
                }
                

                // Check if this is a short variable declaration (identifier := expression or tuple destructuring)
                if (self.check(&TokenKind::Identifier) && self.peek_ahead(1).kind == TokenKind::ColonEqual) ||
                   (self.check(&TokenKind::LeftParen) && self.is_tuple_short_declaration()) {
                    Ok(Statement::ShortDeclaration(self.parse_short_declaration_statement()?))
                } else if (self.check(&TokenKind::Identifier) && self.peek_ahead(1).kind == TokenKind::Equal) ||
                          (self.check(&TokenKind::LeftParen) && self.is_tuple_assignment()) {
                    Ok(Statement::Assignment(self.parse_assignment_statement()?))
                } else if self.check(&TokenKind::Identifier) && self.peek_ahead(1).kind == TokenKind::PlusPlus {
                    // Postfix increment (variable++)
                    Ok(Statement::Increment(self.parse_postfix_increment_statement()?))
                } else if self.check(&TokenKind::Identifier) && self.peek_ahead(1).kind == TokenKind::MinusMinus {
                    // Postfix decrement (variable--)
                    Ok(Statement::Decrement(self.parse_postfix_decrement_statement()?))
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
                let param = self.parse_parameter()?;
                parameters.push(param);
                
                if !self.match_tokens(&[TokenKind::Comma]) {
                    break;
                }
            }
        }
        
        self.consume(TokenKind::RightParen, "Expected ')' after parameters")?;
        
        // Parse optional return type (CURSED syntax: "slay func(x normie) normie { ... }")
        let return_type = if self.check_type_token() {
            Some(self.parse_type()?)
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
        // Accept both 'slay' and 'sus' for function declarations
        if self.check(&TokenKind::Slay) {
            self.advance(); // consume 'slay'
        } else if self.check(&TokenKind::Sus) {
            self.advance(); // consume 'sus'
        } else {
            return Err(CursedError::parse_error("Expected 'slay' or 'sus' for function declaration"));
        }
        let name = self.consume(TokenKind::Identifier, "Expected function name")?.lexeme.clone();
        
        // Parse optional generic parameters
        let type_parameters = self.parse_generic_parameters()?;
        
        self.consume(TokenKind::LeftParen, "Expected '(' after function name")?;
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
        
        // Parse optional return type (CURSED syntax: "slay func(x normie) normie { ... }")
        let return_type = if self.check_type_token() {
            Some(self.parse_type()?)
        } else {
            None
        };
        
        // Parse optional where clause
        let where_clause = self.parse_where_clause()?;
        
        self.consume(TokenKind::LeftBrace, "Expected '{' before function body")?;
        self.skip_newlines_and_semicolons();
        
        let mut body = Vec::new();
        
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            self.skip_newlines_and_semicolons(); // Skip newlines before parsing each statement
            if !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
                match self.parse_statement() {
                    Ok(stmt) => {
                        body.push(stmt);
                        
                        // Optional semicolon after statement
                        if self.check(&TokenKind::Semicolon) {
                            self.advance();
                        }
                    },
                    Err(e) => {
                        // Record error but continue parsing function body
                        log::error!("❌ Parse error in function body: {}", e);
                        self.errors.push(format!("Parse error in function body: {}", e));
                        self.recover_within_block();
                    }
                }
            }
        }
        
        self.consume(TokenKind::RightBrace, "Expected '}' after function body")?;
        
        let func_stmt = FunctionStatement {
            name: name.clone(),
            type_parameters,
            parameters,
            body,
            return_type,
            where_clause,
            visibility,
        };
        
        Ok(func_stmt)
    }

    fn parse_let_statement(&mut self) -> Result<LetStatement, CursedError> {
        self.consume(TokenKind::Sus, "Expected 'sus'")?;
        let name = self.consume(TokenKind::Identifier, "Expected variable name")?.lexeme.clone();
        
        // Check for optional type after variable name (CURSED syntax: "sus result normie = ...")
        let var_type = if self.check_type_token() {
            Some(self.parse_type()?)
        } else {
            None
        };
        
        self.consume(TokenKind::Equal, "Expected '=' after variable name")?;
        let value = self.parse_expression()?;
        
        Ok(LetStatement {
            target: crate::ast::LetTarget::Single(name),
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

    /// Parse a short variable declaration (identifier := expression)
    fn parse_short_declaration_statement(&mut self) -> Result<crate::ast::ShortDeclarationStatement, CursedError> {
        let target = self.parse_short_declaration_target()?;
        self.consume(TokenKind::ColonEqual, "Expected ':=' in short variable declaration")?;
        let value = self.parse_expression()?;
        
        Ok(crate::ast::ShortDeclarationStatement {
            target,
            value,
        })
    }

    fn parse_short_declaration_target(&mut self) -> Result<crate::ast::ShortDeclarationTarget, CursedError> {
        // Check for tuple destructuring: (a, b, c) := ...
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
            Ok(crate::ast::ShortDeclarationTarget::Tuple(names))
        } else {
            // Single variable declaration
            let name = self.consume(TokenKind::Identifier, "Expected variable name")?;
            Ok(crate::ast::ShortDeclarationTarget::Single(name.lexeme.clone()))
        }
    }

    /// Check if the current position looks like a tuple short declaration (a, b) :=
    fn is_tuple_short_declaration(&self) -> bool {
        if !self.check(&TokenKind::LeftParen) {
            return false;
        }
        
        // Look ahead to see if this is a tuple short declaration pattern
        // We're looking for: ( identifier, identifier, ... ) :=
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
                    // After closing paren, expect :=
                    return self.current + pos < self.tokens.len() && self.tokens[self.current + pos].kind == TokenKind::ColonEqual;
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
            target: crate::ast::LetTarget::Single(name),
            value,
            var_type: None, // Constants don't need explicit type annotations
            visibility: crate::ast::Visibility::Private,
        })
    }

    fn parse_let_statement_with_visibility(&mut self, visibility: crate::ast::Visibility) -> Result<LetStatement, CursedError> {
        self.consume(TokenKind::Sus, "Expected 'sus'")?;
        
        // Check for tuple destructuring syntax: sus (x, y, z) = ...
        let target = if self.check(&TokenKind::LeftParen) {
            self.advance(); // consume '('
            let mut names = Vec::new();
            
            // Parse variable names separated by commas
            loop {
                let name = self.consume(TokenKind::Identifier, "Expected variable name in tuple destructuring")?.lexeme.clone();
                names.push(name);
                
                if !self.check(&TokenKind::Comma) {
                    break;
                }
                self.advance(); // consume ','
            }
            
            self.consume(TokenKind::RightParen, "Expected ')' after tuple destructuring")?;
            crate::ast::LetTarget::Tuple(names)
        } else {
            let name = self.consume(TokenKind::Identifier, "Expected variable name")?.lexeme.clone();
            crate::ast::LetTarget::Single(name)
        };
        
        // Check for optional type after variable name (CURSED syntax: "sus result normie = ...")
        let var_type = if self.check_type_token() {
            Some(self.parse_type()?)
        } else {
            None
        };
        
        self.consume(TokenKind::Equal, "Expected '=' after variable name")?;
        let value = self.parse_expression()?;
        
        Ok(LetStatement {
            target,
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
            target: crate::ast::LetTarget::Single(name),
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
            target: crate::ast::LetTarget::Single(name),
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
        
        // Try to parse optional simple statement prefix
        let (init, condition) = self.parse_if_statement_parts()?;
        
        self.skip_newlines(); // Skip newlines after condition
        self.consume(TokenKind::LeftBrace, "Expected '{' after if condition")?;
        
        let mut then_branch = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            // Skip newlines within if statement body
            if self.check(&TokenKind::Newline) {
                self.advance();
                continue;
            }
            match self.parse_statement() {
                Ok(stmt) => {
                    then_branch.push(stmt);
                    
                    // Optional semicolon after statement
                    if self.check(&TokenKind::Semicolon) {
                        self.advance();
                    }
                },
                Err(e) => {
                    // Record error but continue parsing if body
                    log::error!("❌ Parse error in if body: {}", e);
                    self.errors.push(format!("Parse error in if body: {}", e));
                    // Instead of full synchronize, just skip to next statement boundary within this block
                    self.recover_within_block();
                }
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
                match self.parse_statement() {
                    Ok(stmt) => {
                        else_stmts.push(stmt);
                        
                        // Optional semicolon after statement  
                        if self.check(&TokenKind::Semicolon) {
                            self.advance();
                        }
                    },
                    Err(e) => {
                        // Record error but continue parsing else body
                        log::error!("❌ Parse error in else body: {}", e);
                        self.errors.push(format!("Parse error in else body: {}", e));
                        self.recover_within_block();
                    }
                }
            }
            self.consume(TokenKind::RightBrace, "Expected '}' after else body")?;
            else_branch = Some(else_stmts);
        }
        
        Ok(IfStatement {
            init,
            condition,
            then_branch,
            else_branch,
        })
    }

    /// Parse if statement parts: optional simple statement followed by semicolon, then condition
    fn parse_if_statement_parts(&mut self) -> Result<(Option<Box<Statement>>, Expression), CursedError> {
        // Check if this looks like a simple statement followed by semicolon
        if self.is_simple_statement_prefix() {
            // Parse the simple statement
            let simple_stmt = self.parse_simple_statement()?;
            
            // Expect semicolon
            self.consume(TokenKind::Semicolon, "Expected ';' after simple statement in if")?;
            self.skip_newlines();
            
            // Parse the condition
            let condition = self.parse_expression()?;
            
            Ok((Some(Box::new(simple_stmt)), condition))
        } else {
            // No simple statement prefix, just parse the condition
            let condition = self.parse_expression()?;
            Ok((None, condition))
        }
    }

    /// Check if current position looks like a simple statement prefix for if/switch
    fn is_simple_statement_prefix(&self) -> bool {
        // Look ahead to see if this pattern matches: SimpleStmt ';'
        // We need to look for patterns like:
        // - identifier := expression ;
        // - identifier = expression ;
        // - identifier++ ;
        // - identifier-- ;
        // - (a, b) := expression ;
        // - (a, b) = expression ;
        
        if self.check(&TokenKind::Identifier) {
            match self.peek_ahead(1).kind {
                TokenKind::ColonEqual => {
                    // Check for semicolon after a reasonable expression
                    self.find_semicolon_after_expression(2)
                }
                TokenKind::Equal => {
                    // Check for semicolon after a reasonable expression
                    self.find_semicolon_after_expression(2)
                }
                TokenKind::PlusPlus | TokenKind::MinusMinus => {
                    // Increment/decrement followed by semicolon
                    self.peek_ahead(2).kind == TokenKind::Semicolon
                }
                _ => false,
            }
        } else if self.check(&TokenKind::LeftParen) {
            // Tuple assignment/declaration
            self.is_tuple_statement_prefix()
        } else {
            false
        }
    }

    /// Check if current position looks like a tuple statement prefix
    fn is_tuple_statement_prefix(&self) -> bool {
        // Look for pattern: ( identifier, ... ) := or = followed by semicolon
        let mut pos = 1; // Skip initial '('
        
        // Skip identifiers and commas
        while pos < 10 { // Reasonable lookahead limit
            let token = self.peek_ahead(pos);
            match token.kind {
                TokenKind::Identifier => pos += 1,
                TokenKind::Comma => pos += 1,
                TokenKind::RightParen => {
                    pos += 1;
                    break;
                }
                _ => return false,
            }
        }
        
        // Check for := or = after closing paren
        let op_token = self.peek_ahead(pos);
        if op_token.kind == TokenKind::ColonEqual || op_token.kind == TokenKind::Equal {
            pos += 1;
            // Look for semicolon after expression
            self.find_semicolon_after_expression(pos)
        } else {
            false
        }
    }

    /// Look ahead to find semicolon after an expression starting at given position
    fn find_semicolon_after_expression(&self, start_pos: usize) -> bool {
        let mut pos = start_pos;
        let mut paren_depth = 0;
        let mut brace_depth = 0;
        
        // Simple heuristic: look for semicolon while tracking parentheses and braces
        while pos < 20 { // Reasonable lookahead limit
            let token = self.peek_ahead(pos);
            match token.kind {
                TokenKind::Semicolon if paren_depth == 0 && brace_depth == 0 => return true,
                TokenKind::LeftParen => paren_depth += 1,
                TokenKind::RightParen => paren_depth -= 1,
                TokenKind::LeftBrace => brace_depth += 1,
                TokenKind::RightBrace => brace_depth -= 1,
                TokenKind::Newline | TokenKind::Eof => return false,
                _ => {}
            }
            pos += 1;
        }
        false
    }

    /// Parse a simple statement (for use in if/switch prefixes)
    fn parse_simple_statement(&mut self) -> Result<Statement, CursedError> {
        // Parse the same simple statements as in regular statement parsing
        if (self.check(&TokenKind::Identifier) && self.peek_ahead(1).kind == TokenKind::ColonEqual) ||
           (self.check(&TokenKind::LeftParen) && self.is_tuple_short_declaration()) {
            Ok(Statement::ShortDeclaration(self.parse_short_declaration_statement()?))
        } else if (self.check(&TokenKind::Identifier) && self.peek_ahead(1).kind == TokenKind::Equal) ||
                  (self.check(&TokenKind::LeftParen) && self.is_tuple_assignment()) {
            Ok(Statement::Assignment(self.parse_assignment_statement()?))
        } else if self.check(&TokenKind::Identifier) && self.peek_ahead(1).kind == TokenKind::PlusPlus {
            Ok(Statement::Increment(self.parse_postfix_increment_statement()?))
        } else if self.check(&TokenKind::Identifier) && self.peek_ahead(1).kind == TokenKind::MinusMinus {
            Ok(Statement::Decrement(self.parse_postfix_decrement_statement()?))
        } else {
            // For any other case, try to parse as expression statement
            Ok(Statement::Expression(self.parse_expression()?))
        }
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
        log::debug!("🔍 After consuming periodt, current token: {:?}", self.peek());
        self.consume(TokenKind::LeftParen, "Expected '(' after 'periodt'")?;
        log::debug!("🔍 After consuming left paren, current token: {:?}", self.peek());
        let condition = self.parse_expression()?;
        log::debug!("🔍 After parsing condition, current token: {:?}", self.peek());
        self.consume(TokenKind::RightParen, "Expected ')' after while condition")?;
        log::debug!("🔍 After consuming right paren, current token: {:?}", self.peek());
        self.consume(TokenKind::LeftBrace, "Expected '{' after while condition")?;
        
        let mut body = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            // Skip newlines in the body
            if self.check(&TokenKind::Newline) {
                self.advance();
                continue;
            }
            body.push(self.parse_statement()?);
        }
        
        self.consume(TokenKind::RightBrace, "Expected '}' after while body")?;
        
        Ok(WhileStatement {
            condition,
            body,
        })
    }

    fn parse_for_statement(&mut self) -> Result<Statement, CursedError> {
        self.consume(TokenKind::Bestie, "Expected 'bestie'")?;
        
        // Check if this is a for-in loop by looking ahead without consuming tokens
        // Pattern: bestie variable in iterable { ... }
        let checkpoint = self.current;
        
        // Check for for-in pattern: identifier followed by 'in'
        if self.check(&TokenKind::Identifier) && self.peek_ahead(1).kind == TokenKind::In {
            // This is a for-in loop
            let variable = self.advance().lexeme.clone();
            self.consume(TokenKind::In, "Expected 'in' for for-in loop")?;
            let iterable = self.parse_expression()?;
            
            self.consume(TokenKind::LeftBrace, "Expected '{' after for-in header")?;
            
            let mut body = Vec::new();
            while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
                self.skip_newlines(); // Skip newlines before parsing each statement
                if !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
                    body.push(self.parse_statement()?);
                }
            }
            
            self.consume(TokenKind::RightBrace, "Expected '}' after for-in body")?;
            
            return Ok(Statement::ForIn(ForInStatement {
                variable,
                iterable,
                body,
            }));
        }
        
        // This is a traditional C-style for loop
        // Current position is at the first token after 'bestie'
        
        // Parse for loop variants:
        // bestie init; condition; update { ... }
        
        let init = if self.check(&TokenKind::Semicolon) {
            None
        } else {
            // For C-style for loops, we need to handle the initialization properly
            // The initialization can be a short declaration or regular statement
            Some(Box::new(self.parse_for_init_statement()?))
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
            Some(self.parse_for_update_clause()?)
        };
        
        self.consume(TokenKind::LeftBrace, "Expected '{' after for loop header")?;
        
        let mut body = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            self.skip_newlines(); // Skip newlines before parsing each statement
            if !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
                body.push(self.parse_statement()?);
            }
        }
        
        self.consume(TokenKind::RightBrace, "Expected '}' after for body")?;
        
        Ok(Statement::For(ForStatement {
            init,
            condition,
            update,
            body,
        }))
    }

    fn parse_for_init_statement(&mut self) -> Result<Statement, CursedError> {
        // In a for loop, the initialization can be:
        // - A short variable declaration: i := 0
        // - A regular assignment: i = 0
        // - An expression statement
        
        // Check if this is a short declaration pattern: identifier := expression
        if self.check(&TokenKind::Identifier) && self.peek_ahead(1).kind == TokenKind::ColonEqual {
            Ok(Statement::ShortDeclaration(self.parse_short_declaration_statement()?))
        } else if self.check(&TokenKind::Identifier) && self.peek_ahead(1).kind == TokenKind::Equal {
            Ok(Statement::Assignment(self.parse_assignment_statement()?))
        } else {
            Ok(Statement::Expression(self.parse_expression()?))
        }
    }

    fn parse_for_update_clause(&mut self) -> Result<Expression, CursedError> {
        // Handle increment/decrement operators in for loop update clause
        if self.check(&TokenKind::PlusPlus) {
            // Prefix increment: ++i
            self.advance();
            let variable = self.consume(TokenKind::Identifier, "Expected variable name after '++'")?;
            return Ok(Expression::Increment(crate::ast::IncrementExpression {
                variable: variable.lexeme.clone(),
                is_prefix: true,
            }));
        }
        
        if self.check(&TokenKind::MinusMinus) {
            // Prefix decrement: --i
            self.advance();
            let variable = self.consume(TokenKind::Identifier, "Expected variable name after '--'")?;
            return Ok(Expression::Decrement(crate::ast::DecrementExpression {
                variable: variable.lexeme.clone(),
                is_prefix: true,
            }));
        }
        
        // Check for postfix increment/decrement or assignment
        let checkpoint = self.current;
        if self.check(&TokenKind::Identifier) {
            let variable = self.advance().lexeme.clone();
            if self.check(&TokenKind::PlusPlus) {
                // Postfix increment: i++
                self.advance();
                return Ok(Expression::Increment(crate::ast::IncrementExpression {
                    variable,
                    is_prefix: false,
                }));
            } else if self.check(&TokenKind::MinusMinus) {
                // Postfix decrement: i--
                self.advance();
                return Ok(Expression::Decrement(crate::ast::DecrementExpression {
                    variable,
                    is_prefix: false,
                }));
            } else if self.check(&TokenKind::Equal) {
                // Assignment: i = expression
                self.advance();
                let value = self.parse_expression()?;
                return Ok(Expression::Binary(crate::ast::BinaryExpression {
                    left: Box::new(Expression::Identifier(variable)),
                    operator: "=".to_string(),
                    right: Box::new(value),
                }));
            }
        }
        
        // Reset and parse as normal expression
        self.current = checkpoint;
        self.parse_expression()
    }

    fn parse_switch_statement(&mut self) -> Result<SwitchStatement, CursedError> {
        self.consume(TokenKind::VibeCheck, "Expected 'vibe_check'")?;
        
        // Try to parse optional simple statement prefix
        let (init, expression) = self.parse_switch_statement_parts()?;
        
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
            init,
            expression,
            cases,
            default_case,
        })
    }

    /// Parse switch statement parts: optional simple statement followed by semicolon, then expression
    fn parse_switch_statement_parts(&mut self) -> Result<(Option<Box<Statement>>, Expression), CursedError> {
        // Check if this looks like a simple statement followed by semicolon
        if self.is_simple_statement_prefix() {
            // Parse the simple statement
            let simple_stmt = self.parse_simple_statement()?;
            
            // Expect semicolon
            self.consume(TokenKind::Semicolon, "Expected ';' after simple statement in switch")?;
            self.skip_newlines();
            
            // Parse the expression
            let expression = self.parse_expression()?;
            
            Ok((Some(Box::new(simple_stmt)), expression))
        } else {
            // No simple statement prefix, just parse the expression
            let expression = self.parse_expression()?;
            Ok((None, expression))
        }
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
        if self.match_tokens(&[TokenKind::Bang, TokenKind::Minus, TokenKind::At, TokenKind::Star]) {
            let operator = self.previous().lexeme.clone();
            let right = self.parse_unary()?;
            let unary_op = match operator.as_str() {
                "!" => UnaryOperator::Not,
                "-" => UnaryOperator::Minus,
                "+" => UnaryOperator::Plus,
                "@" => UnaryOperator::AddressOf,
                "*" => UnaryOperator::Dereference,
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
                if let Expression::Identifier(ref struct_name) = expr {
                    // Look ahead to see if this is actually a struct literal
                    // A real struct literal should have field assignments: identifier { field: value, ... }
                    if self.looks_like_struct_literal() {
                        expr = self.parse_struct_literal(struct_name.clone())?;
                    } else {
                        break;
                    }
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
                } else if self.check(&TokenKind::LeftParen) {
                    // Type assertion: value.(type)
                    self.advance(); // consume '('
                    let target_type = self.parse_type()?;
                    
                    // Check for safe type assertion: value.(type)?
                    let is_safe = if self.check(&TokenKind::Question) {
                        self.advance(); // consume '?'
                        true
                    } else {
                        false
                    };
                    
                    self.consume(TokenKind::RightParen, "Expected ')' after type assertion")?;
                    
                    expr = Expression::TypeAssertion(crate::ast::TypeAssertionExpression {
                        value: Box::new(expr),
                        target_type,
                        is_safe,
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
            } else if self.match_tokens(&[TokenKind::LeftBracket]) {
                // Array indexing or slice access: array[index] or array[start:end]
                let mut start = None;
                let mut end = None;
                
                // Check for slice expressions
                if self.check(&TokenKind::Colon) {
                    // Slice starting from beginning: array[:end]
                    self.advance(); // consume ':'
                    if !self.check(&TokenKind::RightBracket) {
                        end = Some(Box::new(self.parse_expression()?));
                    }
                } else if !self.check(&TokenKind::RightBracket) {
                    // Parse first expression
                    let first_expr = self.parse_expression()?;
                    
                    if self.check(&TokenKind::Colon) {
                        // Slice expression: array[start:end] or array[start:]
                        self.advance(); // consume ':'
                        start = Some(Box::new(first_expr));
                        
                        if !self.check(&TokenKind::RightBracket) {
                            end = Some(Box::new(self.parse_expression()?));
                        }
                    } else {
                        // Regular array indexing: array[index]
                        self.consume(TokenKind::RightBracket, "Expected ']' after array index")?;
                        
                        expr = Expression::ArrayAccess(crate::ast::ArrayAccessExpression {
                            array: Box::new(expr),
                            index: Box::new(first_expr),
                        });
                        continue;
                    }
                }
                
                // If we reach here, it's a slice expression
                self.consume(TokenKind::RightBracket, "Expected ']' after slice expression")?;
                
                expr = Expression::SliceAccess(crate::ast::SliceAccessExpression {
                    array: Box::new(expr),
                    start,
                    end,
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
        
        // Parse the channel element type using the new type parser
        let element_type = self.parse_type()?;
        
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
            Ok(Expression::ChannelCreation(Box::new(crate::ast::ChannelCreationExpression {
                element_type: Box::new(element_type),
                capacity,
            })))
        } else {
            // Simple channel type without parentheses syntax (also valid)
            Ok(Expression::ChannelCreation(Box::new(crate::ast::ChannelCreationExpression {
                element_type: Box::new(element_type),
                capacity: None,
            })))
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
            TokenKind::Sus => {
                // Handle 'sus' as boolean literal false in expression context
                self.advance();
                Ok(Expression::Boolean(false))
            },
            TokenKind::Cap => {
                // Handle 'cringe' as nil literal
                self.advance();
                Ok(Expression::Literal(crate::ast::Literal::Nil))
            },
            TokenKind::Number => {
                let token = self.advance();
                // Try parsing as integer first, then as float
                if let Ok(int_value) = token.lexeme.parse::<i64>() {
                    Ok(Expression::Integer(int_value))
                } else if let Ok(float_value) = token.lexeme.parse::<f64>() {
                    Ok(Expression::Float(float_value))
                } else {
                    Err(CursedError::syntax_error("Invalid number literal"))
                }
            },
            TokenKind::String => {
                let token = self.advance();
                Ok(Expression::String(token.lexeme.clone()))
            },
            TokenKind::Character => {
                let token = self.advance();
                // Parse the first character from the lexeme string
                if let Some(c) = token.lexeme.chars().next() {
                    Ok(Expression::Character(c))
                } else {
                    Err(CursedError::syntax_error("Invalid character literal"))
                }
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
            TokenKind::LeftBracket => {
                // Could be array literal [1, 2, 3] or composite literal [5]int{1, 2, 3, 4, 5}
                self.parse_array_literal_or_composite()
            },
            TokenKind::LeftBrace => {
                self.parse_map_literal()
            },
            // Handle composite literals starting with type names
            TokenKind::Normie | TokenKind::Tea | TokenKind::Lit | TokenKind::Sip |
            TokenKind::Smol | TokenKind::Mid | TokenKind::Thicc | TokenKind::Snack |
            TokenKind::Meal | TokenKind::Byte | TokenKind::Rune | TokenKind::Extra => {
                // Check if this is actually a composite literal (type{...}) or a type conversion/function call (type(...))
                if self.peek_ahead(1).kind == TokenKind::LeftBrace {
                    self.parse_composite_literal()
                } else {
                    // Treat as identifier (type conversion or function call)
                    let token = self.advance();
                    Ok(Expression::Identifier(token.lexeme.clone()))
                }
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
            Some(self.parse_type()?)
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

    fn parse_array_literal(&mut self) -> Result<Expression, CursedError> {
        self.consume(TokenKind::LeftBracket, "Expected '[' for array literal")?;
        let mut elements = Vec::new();
        
        // Handle empty array literal []
        if self.check(&TokenKind::RightBracket) {
            self.advance();
            return Ok(Expression::Array(elements));
        }
        
        // Parse array elements
        loop {
            self.skip_newlines(); // Allow newlines between elements
            
            let element = self.parse_expression()?;
            elements.push(element);
            
            self.skip_newlines(); // Allow newlines after elements
            
            if self.match_tokens(&[TokenKind::Comma]) {
                // Continue parsing next element
                continue;
            } else if self.check(&TokenKind::RightBracket) {
                break;
            } else {
                return Err(CursedError::syntax_error("Expected ',' or ']' in array literal"));
            }
        }
        
        self.consume(TokenKind::RightBracket, "Expected ']' to close array literal")?;
        
        Ok(Expression::Array(elements))
    }

    fn parse_map_literal(&mut self) -> Result<Expression, CursedError> {
        self.consume(TokenKind::LeftBrace, "Expected '{' for map literal")?;
        let mut pairs = Vec::new();
        
        // Handle empty map literal {}
        if self.check(&TokenKind::RightBrace) {
            self.advance();
            return Ok(Expression::Map(pairs));
        }
        
        // Parse map key-value pairs
        loop {
            self.skip_newlines(); // Allow newlines between pairs
            
            // Parse key
            let key = self.parse_expression()?;
            
            // Expect colon separator
            self.consume(TokenKind::Colon, "Expected ':' after map key")?;
            
            // Parse value
            let value = self.parse_expression()?;
            pairs.push((key, value));
            
            self.skip_newlines(); // Allow newlines after pairs
            
            if self.match_tokens(&[TokenKind::Comma]) {
                // Continue parsing next pair
                continue;
            } else if self.check(&TokenKind::RightBrace) {
                break;
            } else {
                return Err(CursedError::syntax_error("Expected ',' or '}' in map literal"));
            }
        }
        
        self.consume(TokenKind::RightBrace, "Expected '}' to close map literal")?;
        
        Ok(Expression::Map(pairs))
    }

    fn parse_array_literal_or_composite(&mut self) -> Result<Expression, CursedError> {
        // Look ahead to determine if this is a composite literal
        let mut lookahead = 1;
        let mut bracket_depth = 1; // We start inside brackets since we've already seen '['
        let mut found_type = false;
        
        // Look ahead to see if we have a pattern like [size]type{ or []type{
        while self.current + lookahead < self.tokens.len() {
            let token = &self.tokens[self.current + lookahead];
            match token.kind {
                TokenKind::LeftBracket => bracket_depth += 1,
                TokenKind::RightBracket => {
                    bracket_depth -= 1;
                    if bracket_depth == 0 {
                        // Check if next token is a type followed by a left brace
                        if self.current + lookahead + 1 < self.tokens.len() {
                            let next_token = &self.tokens[self.current + lookahead + 1];
                            if matches!(next_token.kind, 
                                TokenKind::Normie | TokenKind::Tea | TokenKind::Lit | TokenKind::Sip |
                                TokenKind::Smol | TokenKind::Mid | TokenKind::Thicc | TokenKind::Snack |
                                TokenKind::Meal | TokenKind::Byte | TokenKind::Rune | TokenKind::Extra) {
                                // Now check if there's a left brace after the type
                                if self.current + lookahead + 2 < self.tokens.len() {
                                    let brace_token = &self.tokens[self.current + lookahead + 2];
                                    if brace_token.kind == TokenKind::LeftBrace {
                                        found_type = true;
                                        break;
                                    }
                                }
                            }
                        }
                        break;
                    }
                }
                _ => {}
            }
            lookahead += 1;
        }
        
        if found_type {
            // Parse as composite literal: [size]type{...} or []type{...}
            let type_spec = self.parse_type()?;
            self.consume(TokenKind::LeftBrace, "Expected '{' after type in composite literal")?;
            let elements = self.parse_composite_elements()?;
            self.consume(TokenKind::RightBrace, "Expected '}' to close composite literal")?;
            
            Ok(Expression::CompositeLiteral(crate::ast::CompositeLiteralExpression {
                type_spec,
                elements,
            }))
        } else {
            // Parse as regular array literal: [1, 2, 3]
            self.parse_array_literal()
        }
    }

    fn parse_composite_literal(&mut self) -> Result<Expression, CursedError> {
        // Parse type specification
        let type_spec = self.parse_type()?;
        
        // Expect opening brace
        self.consume(TokenKind::LeftBrace, "Expected '{' after type in composite literal")?;
        
        // Parse elements
        let elements = self.parse_composite_elements()?;
        
        // Expect closing brace
        self.consume(TokenKind::RightBrace, "Expected '}' to close composite literal")?;
        
        Ok(Expression::CompositeLiteral(crate::ast::CompositeLiteralExpression {
            type_spec,
            elements,
        }))
    }

    fn parse_composite_elements(&mut self) -> Result<Vec<Expression>, CursedError> {
        let mut elements = Vec::new();
        
        // Handle empty initialization {}
        if self.check(&TokenKind::RightBrace) {
            return Ok(elements);
        }
        
        // Parse elements
        loop {
            self.skip_newlines(); // Allow newlines between elements
            
            let element = self.parse_expression()?;
            elements.push(element);
            
            self.skip_newlines(); // Allow newlines after elements
            
            if self.match_tokens(&[TokenKind::Comma]) {
                // Continue parsing next element
                continue;
            } else if self.check(&TokenKind::RightBrace) {
                break;
            } else {
                return Err(CursedError::syntax_error("Expected ',' or '}' in composite literal"));
            }
        }
        
        Ok(elements)
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
        let return_type = if self.check_type_token() {
            Some(self.parse_type()?)
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
        
        // CURSED syntax: parameter name followed by type (e.g., "x normie", "name tea")
        // Also support colon syntax for compatibility (e.g., "x: normie", "name: tea")
        let param_type = if self.match_tokens(&[TokenKind::Colon]) {
            // Colon syntax: name: type
            if self.check_type_token() {
                Some(self.parse_type()?)
            } else {
                return Err(CursedError::parse_error("Expected parameter type after colon"));
            }
        } else if self.check_type_token() {
            // Space syntax: name type
            Some(self.parse_type()?)
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

    fn parse_defer_statement(&mut self) -> Result<crate::ast::DeferStatement, CursedError> {
        log::debug!("⏰ Parsing defer statement");
        self.consume(TokenKind::Later, "Expected 'later'")?;
        
        // Parse the deferred expression
        let expression = Box::new(self.parse_expression()?);
        
        log::debug!("✅ Successfully parsed defer statement");
        Ok(crate::ast::DeferStatement {
            expression,
        })
    }

    fn parse_break_statement(&mut self) -> Result<crate::ast::BreakStatement, CursedError> {
        log::debug!("💨 Parsing break statement");
        self.consume(TokenKind::Ghosted, "Expected 'ghosted' for break statement")?;
        
        // Check for optional label
        let label = if self.check(&TokenKind::Identifier) {
            Some(self.advance().lexeme.clone())
        } else {
            None
        };
        
        log::debug!("✅ Successfully parsed break statement");
        Ok(crate::ast::BreakStatement { label })
    }

    fn parse_continue_statement(&mut self) -> Result<crate::ast::ContinueStatement, CursedError> {
        log::debug!("🔄 Parsing continue statement");
        self.consume(TokenKind::Simp, "Expected 'simp' for continue statement")?;
        
        // Check for optional label
        let label = if self.check(&TokenKind::Identifier) {
            Some(self.advance().lexeme.clone())
        } else {
            None
        };
        
        log::debug!("✅ Successfully parsed continue statement");
        Ok(crate::ast::ContinueStatement { label })
    }

    fn parse_prefix_increment_statement(&mut self) -> Result<crate::ast::IncrementStatement, CursedError> {
        log::debug!("⬆️ Parsing prefix increment statement");
        self.consume(TokenKind::PlusPlus, "Expected '++' for prefix increment")?;
        let variable = self.consume(TokenKind::Identifier, "Expected variable name after '++'")?;
        
        log::debug!("✅ Successfully parsed prefix increment statement");
        Ok(crate::ast::IncrementStatement::prefix(variable.lexeme.clone()))
    }

    fn parse_prefix_decrement_statement(&mut self) -> Result<crate::ast::DecrementStatement, CursedError> {
        log::debug!("⬇️ Parsing prefix decrement statement");
        self.consume(TokenKind::MinusMinus, "Expected '--' for prefix decrement")?;
        let variable = self.consume(TokenKind::Identifier, "Expected variable name after '--'")?;
        
        log::debug!("✅ Successfully parsed prefix decrement statement");
        Ok(crate::ast::DecrementStatement::prefix(variable.lexeme.clone()))
    }

    fn parse_postfix_increment_statement(&mut self) -> Result<crate::ast::IncrementStatement, CursedError> {
        log::debug!("⬆️ Parsing postfix increment statement");
        let variable = self.consume(TokenKind::Identifier, "Expected variable name")?;
        let variable_name = variable.lexeme.clone();
        self.consume(TokenKind::PlusPlus, "Expected '++' after variable name")?;
        
        log::debug!("✅ Successfully parsed postfix increment statement");
        Ok(crate::ast::IncrementStatement::postfix(variable_name))
    }

    fn parse_postfix_decrement_statement(&mut self) -> Result<crate::ast::DecrementStatement, CursedError> {
        log::debug!("⬇️ Parsing postfix decrement statement");
        let variable = self.consume(TokenKind::Identifier, "Expected variable name")?;
        let variable_name = variable.lexeme.clone();
        self.consume(TokenKind::MinusMinus, "Expected '--' after variable name")?;
        
        log::debug!("✅ Successfully parsed postfix decrement statement");
        Ok(crate::ast::DecrementStatement::postfix(variable_name))
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
