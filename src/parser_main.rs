// Parser module for CURSED language
use crate::ast::{Program, Ast, Statement, FunctionStatement, Parameter, Expression, LetStatement, IfStatement, ForStatement, WhileStatement, Type, Visibility, LetTarget, Literal, BinaryExpression, IncrementExpression, DecrementExpression, TupleExpression, TupleAccessExpression, MemberAccessExpression, CallExpression, AssignmentStatement, AssignmentTarget, DeferStatement, SelectStatement, SelectCase, PatternSwitchStatement, PatternSwitchCase, PatternExpression, FieldPattern, YikesStatement, FamStatement, ShookExpression, ErrorValueExpression, PanicExpression, RecoverExpression, InterfaceStatement, MethodSignature, MethodReceiver, TypeParameter, StructStatement, StructField, StructLiteralExpression, StructFieldAssignment, ConstDecl, ConstSpec, GoroutineStatement, ImportParseResult, TypeAliasStatement, ReturnStatement, BreakStatement, ContinueStatement, MatchExpression, MatchArm, MatchPattern, TypeSwitchExpression, TypeSwitchArm, TypePattern};
use crate::lexer::{Lexer, Token, TokenKind};
use crate::error_types::{Error, Result};
use crate::error_recovery::{ErrorRecoveryManager, SourceLocation, ErrorContext, RecoveryStrategy, ParserState, ParserErrorRecovery};

pub struct Parser {
    lexer: Lexer,
    pub current_token: Option<Token>,
    pub tokens: Vec<Token>,
    pub token_index: usize,
    error_count: usize,
    pub error_recovery: ErrorRecoveryManager,
    pub source_text: String,
    pub filename: Option<String>,
    pub current_line: usize,
    pub current_column: usize,
    pub scope_depth: usize,
    pub in_function: bool,
    pub in_loop: bool,
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
            error_count: 0,
            error_recovery: ErrorRecoveryManager::new(),
            source_text: String::new(),
            filename: None,
            current_line: 1,
            current_column: 1,
            scope_depth: 0,
            in_function: false,
            in_loop: false,
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
            error_count: 0,
            error_recovery: ErrorRecoveryManager::new(),
            source_text: String::new(),
            filename: None,
            current_line: 1,
            current_column: 1,
            scope_depth: 0,
            in_function: false,
            in_loop: false,
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
            
            // Skip newlines and semicolons
            if token.kind == TokenKind::Newline || token.kind == TokenKind::Semicolon {
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
                let import_statements = self.parse_import_statement()?;
                if let Some(single_import) = import_statements.single {
                    imports.push(single_import);
                } else if let Some(group_imports) = import_statements.group {
                    imports.extend(group_imports);
                }
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

    /// Enhanced error recovery - try to find and consume a specific token
    fn recover_to_token(&mut self, target: TokenKind) {
        while let Some(token) = self.current_token.as_ref() {
            if token.kind == target {
                self.advance_token();
                break;
            }
            if token.kind == TokenKind::Eof {
                break;
            }
            self.advance_token();
        }
    }

    pub fn errors(&self) -> Vec<Error> {
        // Return empty errors for now
        vec![]
    }

    pub fn next_token(&mut self) -> Result<()> {
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
    
    fn consume_token(&mut self, expected: TokenKind) -> Result<()> {
        if let Some(token) = self.current_token.as_ref() {
            if token.kind == expected {
                self.next_token()?;
                Ok(())
            } else {
                Err(Error::Parse(format!("Expected {:?}, found {:?}", expected, token.kind)))
            }
        } else {
            Err(Error::Parse("Unexpected end of input".to_string()))
        }
    }
    
    pub fn advance_token(&mut self) {
        let _ = self.next_token();
    }

    fn peek_token(&self) -> Option<&Token> {
        self.current_token.as_ref()
    }
    
    pub fn parse_statement(&mut self) -> Result<Option<Statement>> {
        let token = match self.current_token.as_ref() {
            Some(token) => token,
            None => return Ok(None),
        };
        
        match token.kind {
            TokenKind::Truth => {
                let value = token.lexeme.clone();
                match value.as_str() {
                    "slay" => {
                        // Parse function declaration
                        return Ok(Some(Statement::Function(self.parse_function()?)));
                    }
                    _ => {
                        // Check if this is a short variable declaration or assignment
                        if self.peek_token().map(|t| t.kind == TokenKind::ColonEqual || t.kind == TokenKind::Equal).unwrap_or(false) {
                            // Parse as assignment or short declaration
                            if let Ok(stmt) = self.parse_assignment_or_short_declaration() {
                                return Ok(Some(stmt));
                            }
                        }
                        // Try to parse as expression statement or assignment
                        if let Ok(expr) = self.parse_expression() {
                            return Ok(Some(Statement::Expression(expr)));
                        }
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
            TokenKind::Facts => {
                // Parse constants declaration
                return Ok(Some(Statement::Const(self.parse_facts_statement()?)));
            }
            TokenKind::Lowkey => {
                // Parse if statement
                return Ok(Some(Statement::If(self.parse_if_statement()?)));
            }
            TokenKind::Bestie => {
                // Parse for loop
                return Ok(Some(Statement::For(self.parse_for_statement()?)));
            }
            TokenKind::Flex => {
                // Parse flex range-based loop
                return Ok(Some(Statement::For(self.parse_flex_statement()?)));
            }
            TokenKind::Periodt => {
                // Parse while loop
                return Ok(Some(Statement::While(self.parse_while_statement()?)));
            }
            TokenKind::Later => {
                // Parse defer statement
                return Ok(Some(Statement::Defer(self.parse_defer_statement()?)));
            }
            TokenKind::Select => {
                // Parse select statement
                return Ok(Some(Statement::Select(self.parse_select_statement()?)));
            }
            TokenKind::VibeCheck => {
                // Always try pattern switch parsing for now since that's what we're implementing
                return Ok(Some(Statement::PatternSwitch(self.parse_pattern_switch_vibe_check()?)));
            }
            TokenKind::Yikes => {
                // Parse error handling statement
                return Ok(Some(Statement::Yikes(self.parse_yikes_statement()?)));
            }
            TokenKind::Fam => {
                // Parse panic recovery block
                return Ok(Some(Statement::Fam(self.parse_fam_statement()?)));
            }
            TokenKind::Squad => {
                // Parse struct declaration
                return Ok(Some(Statement::Struct(self.parse_struct_statement()?)));
            }
            TokenKind::Collab => {
                // Parse interface declaration
                return Ok(Some(Statement::Interface(self.parse_interface_statement()?)));
            }
            TokenKind::BeLike => {
                // Parse type alias declaration
                return Ok(Some(Statement::TypeAlias(self.parse_type_alias_statement()?)));
            }
            TokenKind::Stan => {
                // Parse goroutine statement
                return Ok(Some(Statement::Goroutine(self.parse_stan_statement()?)));
            }
            TokenKind::Ready => {
                // Parse ready select statement
                return Ok(Some(Statement::Select(self.parse_ready_statement()?)));
            }
            TokenKind::Yolo => {
                // Parse return statement
                return Ok(Some(Statement::Return(self.parse_yolo_statement()?)));
            }
            TokenKind::Ghosted => {
                // Parse break statement
                return Ok(Some(Statement::Break(self.parse_ghosted_statement()?)));
            }
            TokenKind::Simp => {
                // Parse continue statement
                return Ok(Some(Statement::Continue(self.parse_simp_statement()?)));
            }

            TokenKind::LeftParen => {
                // Check if this is tuple destructuring assignment
                if self.is_tuple_destructuring_assignment() {
                    // Try to parse as assignment or short declaration
                    if let Ok(stmt) = self.parse_assignment_or_short_declaration() {
                        return Ok(Some(stmt));
                    }
                }
                // Otherwise, try to parse as expression statement
                if let Ok(expr) = self.parse_expression() {
                    return Ok(Some(Statement::Expression(expr)));
                }
                // Skip unknown tokens
                self.next_token()?;
                return Ok(None);
            }
            _ => {
                // Try to parse as expression statement first
                if let Ok(expr) = self.parse_expression() {
                    return Ok(Some(Statement::Expression(expr)));
                }
                // Try to parse as assignment or short declaration (handles tuple destructuring)
                if let Ok(stmt) = self.parse_assignment_or_short_declaration() {
                    return Ok(Some(stmt));
                }
                // Skip unknown tokens

                self.next_token()?;
                return Ok(None);
            }
        }
    }
    
    fn parse_function(&mut self) -> Result<FunctionStatement> {
        // Try parsing with the advanced signature parser first
        if let Ok(advanced_signature) = self.try_parse_advanced_signature() {
            return Ok(self.convert_advanced_signature_to_function_statement(advanced_signature));
        }

        // Fall back to legacy parsing
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
        
        // Parse type parameters (generics) <T, U>
        let mut type_parameters = Vec::new();
        if self.current_token.as_ref().map(|t| &t.kind) == Some(&TokenKind::Less) {
            self.next_token()?; // consume '<'
            
            // Parse first type parameter
            if self.current_token.as_ref().map(|t| &t.kind) != Some(&TokenKind::Greater) {
                if let Some(token) = self.current_token.as_ref() {
                    if token.kind == TokenKind::Identifier {
                        type_parameters.push(TypeParameter {
                            name: token.lexeme.clone(),
                            bounds: Vec::new(),
                        });
                        self.next_token()?;
                    }
                }
                
                // Parse additional type parameters
                while self.current_token.as_ref().map(|t| &t.kind) == Some(&TokenKind::Comma) {
                    self.next_token()?; // consume ','
                    if let Some(token) = self.current_token.as_ref() {
                        if token.kind == TokenKind::Identifier {
                            type_parameters.push(TypeParameter {
                                name: token.lexeme.clone(),
                                bounds: Vec::new(),
                            });
                            self.next_token()?;
                        }
                    }
                }
            }
            
            if self.current_token.as_ref().map(|t| &t.kind) == Some(&TokenKind::Greater) {
                self.next_token()?; // consume '>'
            }
        }
        
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
            type_parameters,
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

    /// Try to parse function signature using the advanced parser
    fn try_parse_advanced_signature(&mut self) -> std::result::Result<crate::parser::AdvancedFunctionSignature, crate::error::CursedError> {
        use crate::parser::AdvancedSignatureParser;
        
        // Collect remaining tokens for the advanced parser
        let mut tokens = Vec::new();
        let mut current_token_index = self.token_index;
        
        while current_token_index < self.tokens.len() {
            tokens.push(self.tokens[current_token_index].clone());
            current_token_index += 1;
        }
        
        let mut advanced_parser = AdvancedSignatureParser::new(&tokens);
        advanced_parser.parse_advanced_function_signature()
    }

    /// Convert advanced signature to legacy FunctionStatement
    fn convert_advanced_signature_to_function_statement(
        &mut self, 
        signature: crate::parser::AdvancedFunctionSignature
    ) -> FunctionStatement {
        use crate::ast::{TypeParameter, WhereClause, TypeConstraint};
        
        // Convert enhanced type parameters to basic type parameters
        let type_parameters = signature.type_parameters.into_iter()
            .map(|etp| TypeParameter {
                name: etp.name,
                bounds: etp.bounds.into_iter()
                    .map(|bound| match bound {
                        crate::parser::TypeBound::Clone => "Clone".to_string(),
                        crate::parser::TypeBound::Debug => "Debug".to_string(),
                        crate::parser::TypeBound::Display => "Display".to_string(),
                        crate::parser::TypeBound::Custom(name) => name,
                        _ => "Unknown".to_string(),
                    })
                    .collect(),
            })
            .collect();

        // Convert advanced parameters to basic parameters
        let parameters = signature.parameters.into_iter()
            .map(|ap| Parameter {
                name: ap.name,
                param_type: ap.param_type,
            })
            .collect();

        // Convert where clauses
        let where_clause = if signature.where_clauses.is_empty() {
            None
        } else {
            Some(WhereClause {
                constraints: signature.where_clauses.into_iter()
                    .flat_map(|wc| wc.constraints)
                    .collect(),
            })
        };

        // Advance token index to skip parsed tokens
        // In a real implementation, we'd need to track how many tokens were consumed
        
        FunctionStatement {
            name: signature.name,
            type_parameters,
            parameters,
            body: Vec::new(), // Will be parsed separately
            return_type: signature.return_type,
            where_clause,
            visibility: signature.visibility,
        }
    }
    
    fn parse_parameter(&mut self) -> Result<Parameter> {
        // Parse parameter name
        let name = match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::Truth => {
                let name = token.lexeme.clone();
                self.next_token()?;
                name
            }
            _ => return Err(Error::Parse("Expected parameter name".to_string())),
        };
        
        // Parse parameter type if present
        let param_type = if let Some(token) = self.current_token.as_ref() {
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
        
        Ok(Parameter {
            name,
            param_type,
        })
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
            let old_token = self.current_token.clone();
            if let Some(statement) = self.parse_statement()? {
                statements.push(statement);
            }
            
            // If we didn't advance, break to avoid infinite loop
            if self.current_token.is_none() {
                break;
            }
            
            // If the token didn't change, advance manually to avoid infinite loop
            if self.current_token == old_token {
                self.next_token()?;
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

    fn parse_facts_statement(&mut self) -> Result<ConstDecl> {
        // Consume 'facts' keyword
        self.next_token()?;
        
        let mut specs = Vec::new();
        
        // Check if we have grouped constants: facts ( ... )
        if let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::LeftParen {
                self.next_token()?; // consume '('
                
                // Parse multiple constant specs
                while let Some(token) = self.current_token.as_ref() {
                    if token.kind == TokenKind::RightParen {
                        break;
                    }
                    
                    // Skip newlines and semicolons
                    if token.kind == TokenKind::Newline || token.kind == TokenKind::Semicolon {
                        self.next_token()?;
                        continue;
                    }
                    
                    specs.push(self.parse_const_spec()?);
                }
                
                // Consume ')'
                if let Some(token) = self.current_token.as_ref() {
                    if token.kind == TokenKind::RightParen {
                        self.next_token()?;
                    } else {
                        return Err(Error::Parse("Expected ')' after constants block".to_string()));
                    }
                }
            } else {
                // Single constant spec
                specs.push(self.parse_const_spec()?);
            }
        }
        
        Ok(ConstDecl { specs })
    }
    
    fn parse_const_spec(&mut self) -> Result<ConstSpec> {
        // Parse identifier list
        let mut names = Vec::new();
        
        // Parse first identifier
        if let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::Identifier {
                names.push(token.lexeme.clone());
                self.next_token()?;
            } else {
                return Err(Error::Parse("Expected constant name".to_string()));
            }
        }
        
        // Parse additional identifiers separated by commas
        while let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::Comma {
                self.next_token()?; // consume ','
                
                if let Some(token) = self.current_token.as_ref() {
                    if token.kind == TokenKind::Identifier {
                        names.push(token.lexeme.clone());
                        self.next_token()?;
                    } else {
                        return Err(Error::Parse("Expected constant name after comma".to_string()));
                    }
                }
            } else {
                break;
            }
        }
        
        // Parse optional type
        let const_type = self.parse_type()?;
        
        // Parse '=' and expression list
        let mut values = Vec::new();
        
        if let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::Equal {
                self.next_token()?; // consume '='
                
                // Parse first expression
                values.push(self.parse_expression()?);
                
                // Parse additional expressions separated by commas
                while let Some(token) = self.current_token.as_ref() {
                    if token.kind == TokenKind::Comma {
                        self.next_token()?; // consume ','
                        values.push(self.parse_expression()?);
                    } else {
                        break;
                    }
                }
            } else {
                return Err(Error::Parse("Expected '=' in constant declaration".to_string()));
            }
        }
        
        Ok(ConstSpec {
            names,
            const_type,
            values,
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
        // Parse types - both arrays and basic types
        if let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::LeftBracket {
                // Array type parsing
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
                            "sip" => Type::Sip,
                            "drip" => Type::Float,
                            // TestResult type system
                            "TestResult" => Type::TestResult,
                            "TestStatus" => Type::TestStatus,
                            "TestSuite" => Type::TestSuite,
                            "TestReport" => Type::TestReport,
                            _ => Type::Custom(type_name),
                        };
                        return Ok(Some(Type::Array(Box::new(element_type), None)));
                    }
                }
            } else if token.kind == TokenKind::Dm {
                // Channel type parsing (dm<type>)
                self.next_token()?; // consume 'dm'
                
                // Expect '<'
                if let Some(token) = self.current_token.as_ref() {
                    if token.kind == TokenKind::Less {
                        self.next_token()?; // consume '<'
                        
                        // Parse channel element type
                        if let Some(element_type) = self.parse_type()? {
                            // Expect '>'
                            if let Some(token) = self.current_token.as_ref() {
                                if token.kind == TokenKind::Greater {
                                    self.next_token()?; // consume '>'
                                    
                                    // Check for optional buffer size [N]
                                    if let Some(token) = self.current_token.as_ref() {
                                        if token.kind == TokenKind::LeftBracket {
                                            self.next_token()?; // consume '['
                                            
                                            // Parse buffer size expression
                                            let _buffer_size = self.parse_expression()?;
                                            
                                            // Expect ']'
                                            if let Some(token) = self.current_token.as_ref() {
                                                if token.kind == TokenKind::RightBracket {
                                                    self.next_token()?; // consume ']'
                                                }
                                            }
                                            
                                            // For now, return basic channel type (buffered channels handled later)
                                            return Ok(Some(Type::Dm(Box::new(element_type))));
                                        }
                                    }
                                    
                                    return Ok(Some(Type::Dm(Box::new(element_type))));
                                } else {
                                    return Err(Error::Parse("Expected '>' after channel element type".to_string()));
                                }
                            } else {
                                return Err(Error::Parse("Expected '>' after channel element type".to_string()));
                            }
                        } else {
                            return Err(Error::Parse("Expected channel element type after 'dm<'".to_string()));
                        }
                    } else {
                        return Err(Error::Parse("Expected '<' after 'dm' for channel type".to_string()));
                    }
                } else {
                    return Err(Error::Parse("Expected '<' after 'dm' for channel type".to_string()));
                }
            } else if token.kind == TokenKind::Identifier || token.kind == TokenKind::Normie || token.kind == TokenKind::Tea || token.kind == TokenKind::Lit || token.kind == TokenKind::Sip || token.kind == TokenKind::Smol || token.kind == TokenKind::Mid || token.kind == TokenKind::Thicc || token.kind == TokenKind::Snack || token.kind == TokenKind::Meal || token.kind == TokenKind::Byte || token.kind == TokenKind::Rune || token.kind == TokenKind::Extra {
                // Basic type parsing
                let type_name = token.lexeme.clone();
                self.next_token()?;
                let basic_type = match type_name.as_str() {
                    "normie" => Type::Normie,
                    "tea" => Type::Tea,
                    "lit" => Type::Lit,
                    "sip" => Type::Sip,
                    "smol" => Type::Smol,
                    "mid" => Type::Mid,
                    "thicc" => Type::Thicc,
                    "snack" => Type::Snack,
                    "meal" => Type::Meal,
                    "byte" => Type::Byte,
                    "rune" => Type::Rune,
                    "extra" => Type::Extra,
                    "drip" => Type::Float,  // Legacy support
                    // TestResult type system
                    "TestResult" => Type::TestResult,
                    "TestStatus" => Type::TestStatus,
                    "TestSuite" => Type::TestSuite,
                    "TestReport" => Type::TestReport,
                    _ => Type::Custom(type_name.clone()),
                };
                
                // Check for generic type parameters <T, U>
                if self.current_token.as_ref().map(|t| &t.kind) == Some(&TokenKind::Less) {
                    self.next_token()?; // consume '<'
                    
                    let mut type_args = Vec::new();
                    
                    // Parse first type argument
                    if self.current_token.as_ref().map(|t| &t.kind) != Some(&TokenKind::Greater) {
                        if let Some(arg_type) = self.parse_type()? {
                            type_args.push(arg_type);
                        }
                        
                        // Parse additional type arguments
                        while self.current_token.as_ref().map(|t| &t.kind) == Some(&TokenKind::Comma) {
                            self.next_token()?; // consume ','
                            if let Some(arg_type) = self.parse_type()? {
                                type_args.push(arg_type);
                            }
                        }
                    }
                    
                    if self.current_token.as_ref().map(|t| &t.kind) == Some(&TokenKind::Greater) {
                        self.next_token()?; // consume '>'
                    }
                    
                    return Ok(Some(Type::Generic(type_name, type_args)));
                }
                
                return Ok(Some(basic_type));
            }
        }
        
        Ok(None)
    }
    
    fn parse_expression(&mut self) -> Result<Expression> {
        // Parse a primary expression first
        let mut left = self.parse_primary_expression()?;
        
        // Then check for binary operators and chain them
        while let Some(token) = self.current_token.as_ref() {
            match token.kind {
                TokenKind::Plus => {
                    self.next_token()?;
                    let right = self.parse_primary_expression()?;
                    left = Expression::Binary(BinaryExpression {
                        left: Box::new(left),
                        operator: "+".to_string(),
                        right: Box::new(right),
                    });
                }
                TokenKind::Minus => {
                    self.next_token()?;
                    let right = self.parse_primary_expression()?;
                    left = Expression::Binary(BinaryExpression {
                        left: Box::new(left),
                        operator: "-".to_string(),
                        right: Box::new(right),
                    });
                }
                TokenKind::Star => {
                    self.next_token()?;
                    let right = self.parse_primary_expression()?;
                    left = Expression::Binary(BinaryExpression {
                        left: Box::new(left),
                        operator: "*".to_string(),
                        right: Box::new(right),
                    });
                }
                TokenKind::Slash => {
                    self.next_token()?;
                    let right = self.parse_primary_expression()?;
                    left = Expression::Binary(BinaryExpression {
                        left: Box::new(left),
                        operator: "/".to_string(),
                        right: Box::new(right),
                    });
                }
                TokenKind::Greater => {
                    self.next_token()?;
                    let right = self.parse_primary_expression()?;
                    left = Expression::Binary(BinaryExpression {
                        left: Box::new(left),
                        operator: ">".to_string(),
                        right: Box::new(right),
                    });
                }
                TokenKind::Less => {
                    self.next_token()?;
                    let right = self.parse_primary_expression()?;
                    left = Expression::Binary(BinaryExpression {
                        left: Box::new(left),
                        operator: "<".to_string(),
                        right: Box::new(right),
                    });
                }
                TokenKind::EqualEqual => {
                    self.next_token()?;
                    let right = self.parse_primary_expression()?;
                    left = Expression::Binary(BinaryExpression {
                        left: Box::new(left),
                        operator: "==".to_string(),
                        right: Box::new(right),
                    });
                }
                TokenKind::LeftArrow => {
                    // Channel send expression (channel <- value)
                    self.next_token()?; // consume '<-'
                    let right = self.parse_primary_expression()?;
                    left = Expression::ChannelSend(crate::ast::ChannelSendExpression {
                        channel: Box::new(left),
                        value: Box::new(right),
                    });
                }
                TokenKind::Shook => {
                    // Parse error propagation operator
                    self.next_token()?;
                    left = Expression::Shook(ShookExpression::new(Box::new(left)));
                }
                _ => break,
            }
        }
        
        Ok(left)
    }
    
    fn parse_primary_expression(&mut self) -> Result<Expression> {
        // Parse primary expressions (literals, identifiers, etc.)
        if let Some(token) = self.current_token.as_ref() {
            match token.kind {
                TokenKind::LeftArrow => {
                    // Channel receive expression (<-channel)
                    self.next_token()?; // consume '<-'
                    let channel_expr = self.parse_primary_expression()?;
                    return Ok(Expression::ChannelReceive(crate::ast::ChannelReceiveExpression {
                        channel: Box::new(channel_expr),
                    }));
                }
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
                        } else if token.kind == TokenKind::Comma {
                            // Skip comma and continue
                            self.next_token()?;
                            continue;
                        } else {
                            // Skip unknown tokens for now
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
                TokenKind::Match => {
                    // Parse match expression
                    return self.parse_match_expression();
                }
                TokenKind::TypeCheck => {
                    // Parse type switch expression
                    return self.parse_type_switch_expression();
                }
                TokenKind::Truth | TokenKind::Based => {
                    // Parse boolean literal
                    self.next_token()?;
                    return Ok(Expression::Literal(Literal::Boolean(true)));
                }
                TokenKind::Lies => {
                    // Parse boolean literal for false (cap)
                    self.next_token()?;
                    return Ok(Expression::Literal(Literal::Boolean(false)));
                }
                TokenKind::Identifier => {
                    // Parse identifier, possibly with postfix operations
                    let name = token.lexeme.clone();
                    self.next_token()?;
                    
                    // Handle postfix operations in a loop to allow chaining
                    let mut expr = Expression::Identifier(name);
                    
                    loop {
                        if let Some(token) = self.current_token.as_ref() {
                            match token.kind {
                                TokenKind::PlusPlus => {
                                    // Postfix increment
                                    self.next_token()?;
                                    expr = Expression::Increment(IncrementExpression {
                                        variable: match expr {
                                            Expression::Identifier(ref name) => name.clone(),
                                            _ => return Err(Error::Parse("Invalid target for increment".to_string())),
                                        },
                                        is_prefix: false,
                                    });
                                }
                                TokenKind::MinusMinus => {
                                    // Postfix decrement
                                    self.next_token()?;
                                    expr = Expression::Decrement(DecrementExpression {
                                        variable: match expr {
                                            Expression::Identifier(ref name) => name.clone(),
                                            _ => return Err(Error::Parse("Invalid target for decrement".to_string())),
                                        },
                                        is_prefix: false,
                                    });
                                }
                                TokenKind::Dot => {
                                    // Handle both tuple access (e.g., tuple.0, tuple.1) and member access (e.g., vibez.spill)
                                    self.next_token()?;
                                    if let Some(token) = self.current_token.as_ref() {
                                        match token.kind {
                                            TokenKind::Number => {
                                                // Tuple access with numeric index
                                                let index: usize = token.lexeme.parse().unwrap_or(0);
                                                self.next_token()?;
                                                expr = Expression::TupleAccess(TupleAccessExpression {
                                                    tuple: Box::new(expr),
                                                    index,
                                                });
                                            }
                                            TokenKind::Identifier | TokenKind::Spill => {
                                            // Member access with identifier or spill keyword
                                            let property_name = token.lexeme.clone();
                                            self.next_token()?;
                                            expr = Expression::MemberAccess(MemberAccessExpression {
                                            object: Box::new(expr),
                                            property: property_name,
                                            });
                                            }
                                            _ => {
                                                return Err(Error::Parse("Expected number or identifier after '.' for member access".to_string()));
                                            }
                                        }
                                    } else {
                                        return Err(Error::Parse("Expected number or identifier after '.' for member access".to_string()));
                                    }
                                }
                                TokenKind::LeftParen => {
                                    // Function call - parse arguments
                                    self.next_token()?; // consume '('
                                    let mut arguments = Vec::new();
                                    
                                    // Parse arguments
                                    if let Some(token) = self.current_token.as_ref() {
                                    if token.kind != TokenKind::RightParen {
                                    loop {
                                    arguments.push(self.parse_expression()?);
                                    
                                    if let Some(token) = self.current_token.as_ref() {
                                    match token.kind {
                                    TokenKind::Comma => {
                                    self.next_token()?; // consume ','
                                    }
                                    TokenKind::RightParen => {
                                    break;
                                    }
                                    _ => {
                                    return Err(Error::Parse("Expected ',' or ')' in function call".to_string()));
                                    }
                                    }
                                    } else {
                                    return Err(Error::Parse("Unexpected end of input in function call".to_string()));
                                    }
                                    }
                                    }
                                    }
                                    
                                    // Consume ')'
                                    if let Some(token) = self.current_token.as_ref() {
                                        if token.kind == TokenKind::RightParen {
                                            self.next_token()?;
                                        } else {
                                            return Err(Error::Parse("Expected ')' to close function call".to_string()));
                                        }
                                    } else {
                                        return Err(Error::Parse("Expected ')' to close function call".to_string()));
                                    }
                                    
                                    // Special handling for "make" function calls for channels
                                    if let Expression::Identifier(ref func_name) = expr {
                                        if func_name == "make" && !arguments.is_empty() {
                                            // Check if first argument is a channel type (dm<Type>)
                                            // For now, we'll handle this as a regular function call
                                             // and let the runtime handle channel creation
                                             expr = Expression::Call(CallExpression {
                                                 function: Box::new(expr),
                                                 arguments,
                                             });
                                         } else {
                                             expr = Expression::Call(CallExpression {
                                                 function: Box::new(expr),
                                                 arguments,
                                             });
                                         }
                                     } else {
                                         expr = Expression::Call(CallExpression {
                                             function: Box::new(expr),
                                             arguments,
                                         });
                                     }
                                     }
                                    TokenKind::LeftBrace => {
                                    // Check if this is actually a struct literal context
                                    // A struct literal should have the pattern: identifier { field: value }
                                    // If we can't parse it as a struct literal, we should not consume the token
                                    
                                    let struct_name = match &expr {
                                    Expression::Identifier(name) => name.clone(),
                                    _ => {
                                    // If the previous expression isn't an identifier, this can't be a struct literal
                                    // Return the current expression without consuming the '{'
                                    return Ok(expr);
                                    }
                                    };
                                    
                                    // Look ahead to see if this looks like a struct literal
                                    // We need to be more careful here to avoid consuming tokens incorrectly
                                    let saved_index = self.token_index;
                                    let saved_token = self.current_token.clone();
                                    
                                    self.next_token()?; // consume '{'
                                    
                                    // Check if the next token could be the start of a struct field
                                    let is_struct_literal = if let Some(token) = self.current_token.as_ref() {
                                    token.kind == TokenKind::Identifier || token.kind == TokenKind::RightBrace
                                    } else {
                                         false
                                     };
                                     
                                     if !is_struct_literal {
                                         // This doesn't look like a struct literal, backtrack
                                         self.token_index = saved_index;
                                         self.current_token = saved_token;
                                         return Ok(expr);
                                     }
                                     
                                     let mut fields = Vec::new();
                                     
                                     // Parse struct fields
                                     while let Some(token) = self.current_token.as_ref() {
                                         if token.kind == TokenKind::RightBrace {
                                             break;
                                         }
                                         
                                         // Parse field name
                                         let field_name = match token.kind {
                                             TokenKind::Truth => {
                                                 let name = token.lexeme.clone();
                                                 self.next_token()?;
                                                 name
                                             }
                                             _ => return Err(Error::Parse("Expected field name in struct literal".to_string())),
                                         };
                                        
                                        // Expect ':'
                                        if let Some(token) = self.current_token.as_ref() {
                                            if token.kind == TokenKind::Colon {
                                                self.next_token()?;
                                            } else {
                                                return Err(Error::Parse("Expected ':' after field name in struct literal".to_string()));
                                            }
                                        } else {
                                            return Err(Error::Parse("Expected ':' after field name in struct literal".to_string()));
                                        }
                                        
                                        // Parse field value
                                        let field_value = self.parse_expression()?;
                                        
                                        fields.push(crate::ast::StructFieldAssignment {
                                            field_name,
                                            value: field_value,
                                        });
                                        
                                        // Check for comma or end
                                        if let Some(token) = self.current_token.as_ref() {
                                            if token.kind == TokenKind::Comma {
                                                self.next_token()?;
                                            } else if token.kind == TokenKind::RightBrace {
                                                break;
                                            } else {
                                                return Err(Error::Parse("Expected ',' or '}' in struct literal".to_string()));
                                            }
                                        }
                                    }
                                    
                                    // Consume '}'
                                    if let Some(token) = self.current_token.as_ref() {
                                        if token.kind == TokenKind::RightBrace {
                                            self.next_token()?;
                                        } else {
                                            return Err(Error::Parse("Expected '}' to close struct literal".to_string()));
                                        }
                                    } else {
                                        return Err(Error::Parse("Expected '}' to close struct literal".to_string()));
                                    }
                                    
                                    expr = Expression::StructLiteral(crate::ast::StructLiteralExpression {
                                        struct_name,
                                        fields,
                                    });
                                }
                                _ => {
                                    // No more postfix operations
                                    break;
                                }
                            }
                        } else {
                            // No more tokens
                            break;
                        }
                    }
                    
                    return Ok(expr);
                }
                TokenKind::Number => {
                    // Parse number literal
                    let value = token.lexeme.clone();
                    self.next_token()?;
                    // Check if it's a float (contains a decimal point)
                    if value.contains('.') {
                        return Ok(Expression::Literal(Literal::Float(value.parse().unwrap_or(0.0))));
                    } else {
                        return Ok(Expression::Literal(Literal::Integer(value.parse().unwrap_or(0))));
                    }
                }
                TokenKind::String => {
                    // Parse string literal
                    let value = token.lexeme.clone();
                    self.next_token()?;
                    return Ok(Expression::Literal(Literal::String(value)));
                }
                TokenKind::Character => {
                    // Parse character literal
                    let value = token.lexeme.clone();
                    self.next_token()?;
                    // The lexer should have already extracted the character value
                    // Just use the first character if available, otherwise use null character
                    let char_value = value.chars().next().unwrap_or('\0');
                    return Ok(Expression::Character(char_value));
                }

                TokenKind::Yikes => {
                    // Parse error value expression
                    self.next_token()?;
                    return Ok(self.parse_yikes_expression()?);
                }
                TokenKind::Panic => {
                    // Parse panic expression
                    self.next_token()?;
                    return Ok(self.parse_panic_expression()?);
                }
                TokenKind::Recover => {
                    // Parse recover expression
                    self.next_token()?;
                    return Ok(self.parse_recover_expression()?);
                }
                TokenKind::LeftParen => {
                    // Parse tuple literal
                    self.next_token()?;
                    let mut elements = Vec::new();
                    
                    // Handle empty tuple
                    if let Some(token) = self.current_token.as_ref() {
                        if token.kind == TokenKind::RightParen {
                            self.next_token()?;
                            return Ok(Expression::Tuple(TupleExpression { elements }));
                        }
                    }
                    
                    // Parse tuple elements
                    loop {
                        let element = self.parse_expression()?;
                        elements.push(element);
                        
                        if let Some(token) = self.current_token.as_ref() {
                            match token.kind {
                                TokenKind::Comma => {
                                    self.next_token()?;
                                    // Check if we have a trailing comma
                                    if let Some(token) = self.current_token.as_ref() {
                                        if token.kind == TokenKind::RightParen {
                                            self.next_token()?;
                                            break;
                                        }
                                    }
                                }
                                TokenKind::RightParen => {
                                    self.next_token()?;
                                    break;
                                }
                                _ => {
                                    return Err(Error::Parse("Expected ',' or ')' in tuple".to_string()));
                                }
                            }
                        } else {
                            return Err(Error::Parse("Unexpected end of input in tuple".to_string()));
                        }
                    }
                    
                    return Ok(Expression::Tuple(TupleExpression { elements }));
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
            Some(token) if token.kind == TokenKind::Truth => {
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

    fn parse_import_statement(&mut self) -> Result<ImportParseResult> {
        // Consume 'yeet' keyword
        self.next_token()?;
        
        // Check if this is a grouped import (starts with '(')
        if let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::LeftParen {
                // Parse grouped imports: yeet ( "path1"; "path2"; ... )
                self.next_token()?; // consume '('
                let mut imports = Vec::new();
                
                while let Some(token) = self.current_token.as_ref() {
                    if token.kind == TokenKind::RightParen {
                        break;
                    }
                    
                    // Skip semicolons and newlines
                    if token.kind == TokenKind::Semicolon || token.kind == TokenKind::Newline {
                        self.next_token()?;
                        continue;
                    }
                    
                    // Parse import path (string literal)
                    let path = match token.kind {
                        TokenKind::String => {
                            let path = token.lexeme.clone().trim_matches('"').to_string();
                            self.next_token()?;
                            path
                        }
                        _ => return Err(Error::Parse("Expected string literal in grouped import".to_string())),
                    };
                    
                    imports.push(crate::ast::ImportStatement {
                        path,
                        alias: None,
                        items: Vec::new(),
                    });
                }
                
                // Consume ')'
                if let Some(token) = self.current_token.as_ref() {
                    if token.kind == TokenKind::RightParen {
                        self.next_token()?;
                    } else {
                        return Err(Error::Parse("Expected ')' after grouped imports".to_string()));
                    }
                }
                
                Ok(ImportParseResult {
                    single: None,
                    group: Some(imports),
                })
            } else {
                // Parse single import
                let path = match token.kind {
                    TokenKind::String => {
                        let path = token.lexeme.clone().trim_matches('"').to_string();
                        self.next_token()?;
                        path
                    }
                    _ => return Err(Error::Parse("Expected string literal after 'yeet'".to_string())),
                };
                
                Ok(ImportParseResult {
                    single: Some(crate::ast::ImportStatement {
                        path,
                        alias: None,
                        items: Vec::new(),
                    }),
                    group: None,
                })
            }
        } else {
            Err(Error::Parse("Expected import path after 'yeet'".to_string()))
        }
    }

    fn parse_for_statement(&mut self) -> Result<ForStatement> {
        // Consume 'bestie' keyword
        self.next_token()?;
        
        // Check if this is a range-for loop (bestie var := flex ...)
        if let Some(token) = self.current_token.as_ref() {
            // Look ahead to see if this is a range-for pattern
            if token.kind == TokenKind::Identifier {
                // Check for various range-for patterns:
                // 1. bestie i, v := flex collection
                // 2. bestie _, v := flex collection  
                // 3. bestie v := flex collection
                let mut lookahead_pos = 1;
                let mut is_range_for = false;
                
                // Look for comma (multi-variable assignment)
                if self.peek_token().map(|t| &t.kind) == Some(&TokenKind::Comma) {
                    lookahead_pos += 2; // Skip comma and next identifier
                }
                
                // Look for := or = followed by flex
                if self.peek_token().map(|t| &t.kind) == Some(&TokenKind::ColonEqual) ||
                   self.peek_token().map(|t| &t.kind) == Some(&TokenKind::Equal) {
                    lookahead_pos += 1;
                    if self.peek_token().map(|t| &t.kind) == Some(&TokenKind::Flex) {
                        is_range_for = true;
                    }
                }
                
                if is_range_for {
                    return self.parse_range_for_statement();
                }
            }
        }
        
        // Check if it's a while-style for loop (no semicolons)
        // Look ahead to see if there are semicolons before the opening brace
        let mut semicolon_count = 0;
        let mut lexer_clone = self.lexer.clone();
        let mut peek_token = lexer_clone.next_token();
        
        while let Ok(token) = peek_token {
            if token.kind == TokenKind::LeftBrace {
                break;
            }
            if token.kind == TokenKind::Semicolon {
                semicolon_count += 1;
            }
            if token.kind == TokenKind::Eof {
                break;
            }
            peek_token = lexer_clone.next_token();
        }
        
        if semicolon_count == 0 {
            // While-style for loop: bestie condition { ... }
            let condition = if let Some(token) = self.current_token.as_ref() {
                if token.kind != TokenKind::LeftBrace {
                    Some(self.parse_expression()?)
                } else {
                    None // Infinite loop
                }
            } else {
                None
            };
            
            let body = self.parse_block()?;
            
            return Ok(ForStatement {
                init: None,
                condition,
                update: None,
                body,
            });
        }
        
        // C-style for loop: bestie init; condition; update { ... }
        
        // Parse init statement (optional)
        let init = if let Some(token) = self.current_token.as_ref() {
            if token.kind != TokenKind::Semicolon {
                // Try to parse as assignment or short declaration
                if let Ok(stmt) = self.parse_assignment_or_short_declaration() {
                    Some(Box::new(stmt))
                } else {
                    // Otherwise try to parse as statement
                    let stmt = self.parse_statement()?.unwrap_or_else(|| {
                        // If no statement parsed, create a simple expression statement
                        Statement::Expression(self.parse_expression().unwrap_or(Expression::Identifier("".to_string())))
                    });
                    Some(Box::new(stmt))
                }
            } else {
                None
            }
        } else {
            None
        };
        
        // Expect ';'
        match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::Semicolon => {
                self.next_token()?;
            }
            _ => return Err(Error::Parse("Expected ';' after for loop init".to_string())),
        }
        
        // Parse condition (optional)
        let condition = if let Some(token) = self.current_token.as_ref() {
            if token.kind != TokenKind::Semicolon {
                Some(self.parse_expression()?)
            } else {
                None
            }
        } else {
            None
        };
        
        // Expect ';'
        match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::Semicolon => {
                self.next_token()?;
            }
            _ => return Err(Error::Parse("Expected ';' after for loop condition".to_string())),
        }
        
        // Parse update (optional)
        let update = if let Some(token) = self.current_token.as_ref() {
            if token.kind != TokenKind::LeftBrace {
                Some(self.parse_expression()?)
            } else {
                None
            }
        } else {
            None
        };
        
        // Parse body
        let body = self.parse_block()?;
        
        Ok(ForStatement {
            init,
            condition,
            update,
            body,
        })
    }
    
    fn parse_range_for_statement(&mut self) -> Result<ForStatement> {
        // Parse range-for: bestie i, v := flex collection { ... }
        
        let mut variables = Vec::new();
        
        // Parse first variable
        match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::Truth => {
                variables.push(token.lexeme.clone());
                self.next_token()?;
            }
            _ => return Err(Error::Parse("Expected variable name in range-for loop".to_string())),
        }
        
        // Check for comma (multi-variable assignment)
        if let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::Comma {
                self.next_token()?; // consume comma
                
                // Parse second variable
                match self.current_token.as_ref() {
                    Some(token) if token.kind == TokenKind::Truth => {
                        variables.push(token.lexeme.clone());
                        self.next_token()?;
                    }
                    _ => return Err(Error::Parse("Expected variable name after comma in range-for loop".to_string())),
                }
            }
        }
        
        // Expect := or =
        match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::ColonEqual || token.kind == TokenKind::Equal => {
                self.next_token()?;
            }
            _ => return Err(Error::Parse("Expected ':=' or '=' in range-for loop".to_string())),
        }
        
        // Expect 'flex'
        match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::Flex => {
                self.next_token()?;
            }
            _ => return Err(Error::Parse("Expected 'flex' in range-for loop".to_string())),
        }
        
        // Parse iterable expression
        let iterable = self.parse_expression()?;
        
        // Parse body
        let body = self.parse_block()?;
        
        // Create a synthetic range-for statement
        // This will be handled specially in codegen
        let range_assignment = Statement::Assignment(AssignmentStatement {
            target: if variables.len() == 1 {
                AssignmentTarget::Single(variables[0].clone())
            } else {
                AssignmentTarget::Tuple(variables)
            },
            value: Expression::RangeFor {
                iterable: Box::new(iterable),
            },
        });
        
        Ok(ForStatement {
            init: Some(Box::new(range_assignment)),
            condition: None,
            update: None,
            body,
        })
    }
    


    fn parse_while_statement(&mut self) -> Result<WhileStatement> {
        // Consume 'periodt' keyword
        self.next_token()?;
        
        // Parse condition expression
        let condition = self.parse_expression()?;
        
        // Parse body
        let body = self.parse_block()?;
        
        Ok(WhileStatement {
            condition,
            body,
        })
    }
    
    fn parse_defer_statement(&mut self) -> Result<DeferStatement> {
        // Consume 'later' keyword
        self.next_token()?;
        
        // Parse the expression to defer
        let expression = self.parse_expression()?;
        
        Ok(DeferStatement {
            expression: Box::new(expression),
        })
    }
    
    fn parse_select_statement(&mut self) -> Result<SelectStatement> {
        // Consume 'select' keyword
        self.next_token()?;
        
        // Expect '{'
        match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::LeftBrace => {
                self.next_token()?;
            }
            _ => return Err(Error::Parse("Expected '{' after 'select'".to_string())),
        }
        
        let mut cases = Vec::new();
        let mut default_case = None;
        
        // Parse select cases
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
            
            // Parse 'ready' keyword or 'basic' (default) case
            if token.kind == TokenKind::Ready {
                self.next_token()?;
                
                // Parse channel operation
                let operation = self.parse_expression()?;
                
                // Expect ':'
                match self.current_token.as_ref() {
                    Some(token) if token.kind == TokenKind::Colon => {
                        self.next_token()?;
                    }
                    _ => return Err(Error::Parse("Expected ':' after select case operation".to_string())),
                }
                
                // Parse case body
                let body = self.parse_select_case_body()?;
                
                cases.push(SelectCase {
                    operation: Box::new(operation),
                    body,
                });
            } else if token.kind == TokenKind::Basic {
                // Parse default case
                self.next_token()?;
                
                // Expect ':'
                match self.current_token.as_ref() {
                    Some(token) if token.kind == TokenKind::Colon => {
                        self.next_token()?;
                    }
                    _ => return Err(Error::Parse("Expected ':' after 'basic' in select".to_string())),
                }
                
                // Parse default case body
                let body = self.parse_select_case_body()?;
                default_case = Some(body);
            } else {
                return Err(Error::Parse("Expected 'ready' or 'basic' in select statement".to_string()));
            }
        }
        
        Ok(SelectStatement {
            cases,
            default_case,
        })
    }

    fn parse_ready_statement(&mut self) -> Result<SelectStatement> {
        // Consume 'ready' keyword
        self.next_token()?;
        
        // Expect '{'
        match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::LeftBrace => {
                self.next_token()?;
            }
            _ => return Err(Error::Parse("Expected '{' after 'ready'".to_string())),
        }
        
        let mut cases = Vec::new();
        let mut default_case = None;
        
        // Parse select cases
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
            
            // Parse 'mood' keyword or 'basic' (default) case
            if token.kind == TokenKind::Mood {
                self.next_token()?;
                
                // Parse channel operation
                let operation = self.parse_expression()?;
                
                // Expect ':'
                match self.current_token.as_ref() {
                    Some(token) if token.kind == TokenKind::Colon => {
                        self.next_token()?;
                    }
                    _ => return Err(Error::Parse("Expected ':' after select case operation".to_string())),
                }
                
                // Parse case body
                let body = self.parse_select_case_body()?;
                
                cases.push(SelectCase {
                    operation: Box::new(operation),
                    body,
                });
            } else if token.kind == TokenKind::Basic {
                // Parse default case
                self.next_token()?;
                
                // Expect ':'
                match self.current_token.as_ref() {
                    Some(token) if token.kind == TokenKind::Colon => {
                        self.next_token()?;
                    }
                    _ => return Err(Error::Parse("Expected ':' after 'basic' in select".to_string())),
                }
                
                // Parse default case body
                let body = self.parse_select_case_body()?;
                default_case = Some(body);
            } else {
                return Err(Error::Parse("Expected 'mood' or 'basic' in ready statement".to_string()));
            }
        }
        
        Ok(SelectStatement {
            cases,
            default_case,
        })
    }
    
    fn parse_vibe_check_statement(&mut self) -> Result<SelectStatement> {
        // Consume 'vibe_check' keyword
        self.next_token()?;
        
        // Expect '{'
        match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::LeftBrace => {
                self.next_token()?;
            }
            _ => return Err(Error::Parse("Expected '{' after 'vibe_check'".to_string())),
        }
        
        let mut cases = Vec::new();
        let mut default_case = None;
        
        // Parse select cases
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
            
            // Parse 'mood' keyword or 'basic' (default) case
            if token.kind == TokenKind::Mood {
                self.next_token()?;
                
                // Parse channel operation
                let operation = self.parse_expression()?;
                
                // Expect ':'
                match self.current_token.as_ref() {
                    Some(token) if token.kind == TokenKind::Colon => {
                        self.next_token()?;
                    }
                    _ => return Err(Error::Parse("Expected ':' after select case operation".to_string())),
                }
                
                // Parse case body
                let body = self.parse_vibe_check_case_body()?;
                
                cases.push(SelectCase {
                    operation: Box::new(operation),
                    body,
                });
            } else if token.kind == TokenKind::Basic {
                // Parse default case
                self.next_token()?;
                
                // Expect ':'
                match self.current_token.as_ref() {
                    Some(token) if token.kind == TokenKind::Colon => {
                        self.next_token()?;
                    }
                    _ => return Err(Error::Parse("Expected ':' after 'basic' in select".to_string())),
                }
                
                // Parse default case body
                let body = self.parse_vibe_check_case_body()?;
                default_case = Some(body);
            } else {
                return Err(Error::Parse("Expected 'mood' or 'basic' in vibe_check statement".to_string()));
            }
        }
        
        Ok(SelectStatement {
            cases,
            default_case,
        })
    }

    fn parse_vibe_check_case_body(&mut self) -> Result<Vec<Statement>> {
        let mut statements = Vec::new();
        
        // Parse statements until we hit a case or end of select
        while let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::RightBrace ||
               token.kind == TokenKind::Mood ||
               token.kind == TokenKind::Basic {
                break;
            }
            
            // Skip newlines
            if token.kind == TokenKind::Newline {
                self.next_token()?;
                continue;
            }
            
            // Parse statement
            if let Some(statement) = self.parse_statement()? {
                statements.push(statement);
            }
        }
        
        Ok(statements)
    }
    
    fn parse_select_case_body(&mut self) -> Result<Vec<Statement>> {
        let mut statements = Vec::new();
        
        // Parse statements until we hit a case or end of select
        while let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::RightBrace ||
               token.kind == TokenKind::Mood ||
               token.kind == TokenKind::Basic {
                break;
            }
            
            // Skip newlines
            if token.kind == TokenKind::Newline {
                self.next_token()?;
                continue;
            }
            
            // Parse statement
            if let Some(statement) = self.parse_statement()? {
                statements.push(statement);
            }
        }
        
        Ok(statements)
    }
    
    fn parse_yolo_statement(&mut self) -> Result<ReturnStatement> {
        // Consume 'yolo' keyword
        self.next_token()?;
        
        // Check if there's a return value
        let value = if let Some(token) = self.current_token.as_ref() {
            // Check if this is the end of the statement (newline, semicolon, or EOF)
            if token.kind == TokenKind::Newline || 
               token.kind == TokenKind::Semicolon || 
               token.kind == TokenKind::Eof ||
               token.kind == TokenKind::RightBrace {
                None
            } else {
                // Parse the return expression
                Some(self.parse_expression()?)
            }
        } else {
            None
        };
        
        Ok(ReturnStatement { value })
    }
    
    fn parse_ghosted_statement(&mut self) -> Result<BreakStatement> {
        // Consume 'ghosted' keyword
        self.next_token()?;
        
        // Check if there's a label
        let label = if let Some(token) = self.current_token.as_ref() {
            // Check if this is an identifier (label)
            if token.kind == TokenKind::Identifier || token.kind == TokenKind::Truth {
                let label_name = token.lexeme.clone();
                self.next_token()?;
                Some(label_name)
            } else {
                None
            }
        } else {
            None
        };
        
        Ok(BreakStatement { label })
    }
    
    fn parse_simp_statement(&mut self) -> Result<ContinueStatement> {
        // Consume 'simp' keyword
        self.next_token()?;
        
        // Check if there's a label
        let label = if let Some(token) = self.current_token.as_ref() {
            // Check if this is an identifier (label)
            if token.kind == TokenKind::Identifier || token.kind == TokenKind::Truth {
                let label_name = token.lexeme.clone();
                self.next_token()?;
                Some(label_name)
            } else {
                None
            }
        } else {
            None
        };
        
        Ok(ContinueStatement { label })
    }
    
    fn is_tuple_destructuring_assignment(&self) -> bool {
        // Simple heuristic: for now, assume any LeftParen at statement level 
        // is likely a tuple destructuring assignment
        // This can be improved with better lookahead logic later
        true
    }
    
    fn parse_assignment_or_short_declaration(&mut self) -> Result<Statement> {
        // Parse the left side - could be a single identifier or tuple destructuring
        let (names, is_tuple) = if self.current_token.as_ref().map(|t| t.kind.clone()) == Some(TokenKind::LeftParen) {
            // Tuple destructuring
            self.next_token()?; // consume '('
            let mut names = Vec::new();
            
            while let Some(token) = self.current_token.as_ref() {
                if token.kind == TokenKind::RightParen {
                    break;
                }
                
                if token.kind == TokenKind::Identifier {
                    names.push(token.lexeme.clone());
                    self.next_token()?;
                    
                    // Check for comma
                    if let Some(token) = self.current_token.as_ref() {
                        if token.kind == TokenKind::Comma {
                            self.next_token()?;
                        }
                    }
                } else {
                    return Err(Error::Parse("Expected identifier in tuple destructuring".to_string()));
                }
            }
            
            // Consume closing paren
            if let Some(token) = self.current_token.as_ref() {
                if token.kind == TokenKind::RightParen {
                    self.next_token()?;
                } else {
                    return Err(Error::Parse("Expected ')' in tuple destructuring".to_string()));
                }
            }
            
            (names, true)
        } else if let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::Identifier {
                let name = token.lexeme.clone();
                self.next_token()?;
                (vec![name], false)
            } else {
                return Err(Error::Parse("Expected identifier in assignment".to_string()));
            }
        } else {
            return Err(Error::Parse("Expected assignment target".to_string()));
        };
        
        // Check for ':=' or '='
        let is_short_declaration = if let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::ColonEqual {
                self.next_token()?;
                true
            } else if token.kind == TokenKind::Equal {
                self.next_token()?;
                false
            } else {
                return Err(Error::Parse("Expected '=' or ':=' in assignment".to_string()));
            }
        } else {
            return Err(Error::Parse("Expected '=' or ':=' in assignment".to_string()));
        };
        
        // Parse the right side expression
        let value = self.parse_expression()?;
        
        if is_short_declaration {
            // Create short declaration statement
            let target = if is_tuple {
                crate::ast::ShortDeclarationTarget::Tuple(names)
            } else {
                crate::ast::ShortDeclarationTarget::Single(names[0].clone())
            };
            Ok(Statement::ShortDeclaration(crate::ast::ShortDeclarationStatement { target, value }))
        } else {
            // Create assignment statement
            let target = if is_tuple {
                crate::ast::AssignmentTarget::Tuple(names)
            } else {
                crate::ast::AssignmentTarget::Single(names[0].clone())
            };
            Ok(Statement::Assignment(crate::ast::AssignmentStatement { target, value }))
        }
    }

    fn parse_assignment_statement(&mut self) -> Result<AssignmentStatement> {
        
        // Parse the left side - could be a single identifier or tuple destructuring
        let target = if self.current_token.as_ref().map(|t| t.kind.clone()) == Some(TokenKind::LeftParen) {
            // Tuple destructuring
            self.next_token()?; // consume '('
            let mut names = Vec::new();
            
            while let Some(token) = self.current_token.as_ref() {
                if token.kind == TokenKind::RightParen {
                    break;
                }
                
                if token.kind == TokenKind::Identifier {
                    names.push(token.lexeme.clone());
                    self.next_token()?;
                    
                    // Check for comma
                    if let Some(token) = self.current_token.as_ref() {
                        if token.kind == TokenKind::Comma {
                            self.next_token()?;
                        }
                    }
                } else {
                    return Err(Error::Parse("Expected identifier in tuple destructuring".to_string()));
                }
            }
            
            // Consume closing paren
            if let Some(token) = self.current_token.as_ref() {
                if token.kind == TokenKind::RightParen {
                    self.next_token()?;
                } else {
                    return Err(Error::Parse("Expected ')' in tuple destructuring".to_string()));
                }
            }
            
            AssignmentTarget::Tuple(names)
        } else if let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::Identifier {
                let name = token.lexeme.clone();
                self.next_token()?;
                AssignmentTarget::Single(name)
            } else {
                return Err(Error::Parse("Expected identifier in assignment".to_string()));
            }
        } else {
            return Err(Error::Parse("Expected assignment target".to_string()));
        };
        
        // Consume '=' or ':='
        if let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::Equal || token.kind == TokenKind::ColonEqual {
                self.next_token()?;
            } else {
                return Err(Error::Parse("Expected '=' or ':=' in assignment".to_string()));
            }
        } else {
            return Err(Error::Parse("Expected '=' or ':=' in assignment".to_string()));
        }
        
        // Parse the right side expression
        let value = self.parse_expression()?;
        
        Ok(AssignmentStatement { target, value })
    }

    // Error handling parsing functions
    
    fn parse_yikes_statement(&mut self) -> Result<YikesStatement> {
        // Consume 'yikes' token
        self.next_token()?;
        
        // Check for direct error creation: yikes "message"
        if let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::String {
                // Direct error creation
                let message = token.lexeme.clone();
                self.next_token()?;
                
                // Create a temporary error variable
                let temp_name = format!("_error_{}", self.error_count);
                self.error_count += 1;
                let mut yikes_stmt = YikesStatement::new(temp_name);
                yikes_stmt = yikes_stmt.with_value(Expression::String(message));
                return Ok(yikes_stmt);
            }
            
            // Check for structured error creation: yikes{...}
            if token.kind == TokenKind::LeftBrace {
                self.next_token()?;
                let mut fields = Vec::new();
                let mut error_message = None;
                let mut error_code = None;
                let mut error_details = None;
                
                // Parse structured error fields
                while let Some(token) = self.current_token.as_ref() {
                    if token.kind == TokenKind::RightBrace {
                        self.next_token()?;
                        break;
                    }
                    
                    if token.kind == TokenKind::Identifier {
                        let field_name = token.lexeme.clone();
                        self.next_token()?;
                        
                        // Expect ':'
                        if let Some(token) = self.current_token.as_ref() {
                            if token.kind == TokenKind::Colon {
                                self.next_token()?;
                                let field_value = self.parse_expression()?;
                                
                                // Store specific error fields
                                match field_name.as_str() {
                                    "message" => error_message = Some(field_value),
                                    "code" => error_code = Some(field_value),
                                    "details" => error_details = Some(field_value),
                                    _ => fields.push((field_name, field_value)),
                                }
                            }
                        }
                    }
                    
                    // Skip comma if present
                    if let Some(token) = self.current_token.as_ref() {
                        if token.kind == TokenKind::Comma {
                            self.next_token()?;
                        }
                    }
                }
                
                // Create structured error expression
                let temp_name = format!("_struct_error_{}", self.error_count);
                self.error_count += 1;
                let mut yikes_stmt = YikesStatement::new(temp_name);
                
                // Create structured error value
                let error_value = if let Some(message) = error_message {
                    Expression::StructuredError {
                        message: Box::new(message),
                        code: error_code.map(Box::new),
                        details: error_details.map(Box::new),
                        fields,
                    }
                } else {
                    Expression::String("Structured error".to_string())
                };
                
                yikes_stmt = yikes_stmt.with_value(error_value);
                return Ok(yikes_stmt);
            }
        }
        
        // Parse error variable name
        let name = if let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::Identifier {
                let name = token.lexeme.clone();
                self.next_token()?;
                name
            } else {
                return Err(Error::Parse("Expected identifier after 'yikes'".to_string()));
            }
        } else {
            return Err(Error::Parse("Expected identifier after 'yikes'".to_string()));
        };
        
        let mut yikes_stmt = YikesStatement::new(name);
        
        // Optional type annotation
        if let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::Colon {
                self.next_token()?;
                if let Some(error_type) = self.parse_type()? {
                    yikes_stmt = yikes_stmt.with_type(error_type);
                }
            }
        }
        
        // Optional assignment
        if let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::Equal || token.kind == TokenKind::ColonEqual {
                self.next_token()?;
                let value = self.parse_expression()?;
                yikes_stmt = yikes_stmt.with_value(value);
            }
        }
        
        Ok(yikes_stmt)
    }
    
    fn parse_fam_statement(&mut self) -> Result<FamStatement> {
        // Consume 'fam' token
        self.next_token()?;
        
        // Parse protected block
        let protected_block = self.parse_block()?;
        
        let mut fam_stmt = FamStatement::new(protected_block);
        
        // Optional recovery block with 'sus' keyword
        if let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::Sus {
                self.next_token()?;
                
                // Parse panic variable name
                let panic_variable = if let Some(token) = self.current_token.as_ref() {
                    if token.kind == TokenKind::Identifier {
                        let name = token.lexeme.clone();
                        self.next_token()?;
                        Some(name)
                    } else {
                        None
                    }
                } else {
                    None
                };
                
                // Parse recovery block
                let recovery_block = self.parse_block()?;
                fam_stmt = fam_stmt.with_recovery(recovery_block, panic_variable);
            }
        }
        
        Ok(fam_stmt)
    }
    
    fn parse_yikes_expression(&mut self) -> Result<Expression> {
        // Parse error value expression: yikes("message") or yikes{...}
        if let Some(token) = self.current_token.as_ref() {
            match token.kind {
                TokenKind::LeftParen => {
                    // Parse yikes("message") or yikes("message", code)
                    self.next_token()?;
                    
                    let message = if let Some(token) = self.current_token.as_ref() {
                        if token.kind != TokenKind::RightParen {
                            Some(Box::new(self.parse_expression()?))
                        } else {
                            None
                        }
                    } else {
                        return Err(Error::Parse("Expected expression or ')' after 'yikes('".to_string()));
                    };
                    
                    // Optional code parameter
                    let code = if let Some(token) = self.current_token.as_ref() {
                        if token.kind == TokenKind::Comma {
                            self.next_token()?;
                            Some(Box::new(self.parse_expression()?))
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    
                    // Consume ')'
                    if let Some(token) = self.current_token.as_ref() {
                        if token.kind == TokenKind::RightParen {
                            self.next_token()?;
                        } else {
                            return Err(Error::Parse("Expected ')' after yikes expression".to_string()));
                        }
                    } else {
                        return Err(Error::Parse("Expected ')' after yikes expression".to_string()));
                    }
                    
                    let error_expr = if let Some(_msg_expr) = message {
                        // For now, we'll create a placeholder string message
                        let mut error_expr = ErrorValueExpression::new("Error".to_string());
                        if let Some(code) = code {
                            error_expr = error_expr.with_code(code);
                        }
                        error_expr
                    } else {
                        ErrorValueExpression::new("Unknown error".to_string())
                    };
                    
                    Ok(Expression::ErrorValue(error_expr))
                }
                TokenKind::LeftBrace => {
                    // Parse yikes{ message: "...", code: 123, details: "..." }
                    self.next_token()?;
                    
                    let mut message = None;
                    let mut code = None;
                    let mut details = None;
                    
                    while let Some(token) = self.current_token.as_ref() {
                        if token.kind == TokenKind::RightBrace {
                            self.next_token()?;
                            break;
                        }
                        
                        if token.kind == TokenKind::Identifier {
                            let field_name = token.lexeme.clone();
                            self.next_token()?;
                            
                            // Expect ':'
                            if let Some(token) = self.current_token.as_ref() {
                                if token.kind == TokenKind::Colon {
                                    self.next_token()?;
                                } else {
                                    return Err(Error::Parse("Expected ':' after field name in yikes literal".to_string()));
                                }
                            } else {
                                return Err(Error::Parse("Expected ':' after field name in yikes literal".to_string()));
                            }
                            
                            // Parse field value
                            let value = Box::new(self.parse_expression()?);
                            
                            match field_name.as_str() {
                                "message" => message = Some(value),
                                "code" => code = Some(value),
                                "details" => details = Some(value),
                                _ => {} // Ignore unknown fields
                            }
                            
                            // Optional comma
                            if let Some(token) = self.current_token.as_ref() {
                                if token.kind == TokenKind::Comma {
                                    self.next_token()?;
                                }
                            }
                        } else {
                            return Err(Error::Parse("Expected field name in yikes literal".to_string()));
                        }
                    }
                    
                    let error_expr = if let Some(_msg_expr) = message {
                        // For now, we'll create a placeholder string message
                        let mut error_expr = ErrorValueExpression::new("Error".to_string());
                        if let Some(code) = code {
                            error_expr = error_expr.with_code(code);
                        }
                        if let Some(details) = details {
                            error_expr = error_expr.with_details(details);
                        }
                        error_expr
                    } else {
                        ErrorValueExpression::new("Unknown error".to_string())
                    };
                    
                    Ok(Expression::ErrorValue(error_expr))
                }
                _ => {
                    // Simple yikes expression without parameters
                    Ok(Expression::ErrorValue(ErrorValueExpression::new("Error".to_string())))
                }
            }
        } else {
            Ok(Expression::ErrorValue(ErrorValueExpression::new("Error".to_string())))
        }
    }

    fn parse_panic_expression(&mut self) -> Result<Expression> {
        // Parse panic expression: panic("message")
        if let Some(token) = self.current_token.as_ref() {
            match token.kind {
                TokenKind::LeftParen => {
                    // Parse panic("message")
                    self.next_token()?;
                    
                    let message = if let Some(token) = self.current_token.as_ref() {
                        if token.kind != TokenKind::RightParen {
                            Box::new(self.parse_expression()?)
                        } else {
                            Box::new(Expression::Literal(Literal::String("panic".to_string())))
                        }
                    } else {
                        return Err(Error::Parse("Expected expression or ')' after 'panic('".to_string()));
                    };
                    
                    // Consume ')'
                    if let Some(token) = self.current_token.as_ref() {
                        if token.kind == TokenKind::RightParen {
                            self.next_token()?;
                        } else {
                            return Err(Error::Parse("Expected ')' after panic expression".to_string()));
                        }
                    } else {
                        return Err(Error::Parse("Expected ')' after panic expression".to_string()));
                    }
                    
                    Ok(Expression::Panic(PanicExpression::new(message)))
                }
                _ => {
                    // Default panic with no message
                    Ok(Expression::Panic(PanicExpression::new(
                        Box::new(Expression::Literal(Literal::String("panic".to_string())))
                    )))
                }
            }
        } else {
            Ok(Expression::Panic(PanicExpression::new(
                Box::new(Expression::Literal(Literal::String("panic".to_string())))
            )))
        }
    }

    fn parse_recover_expression(&mut self) -> Result<Expression> {
        // Parse recover expression: recover()
        if let Some(token) = self.current_token.as_ref() {
            match token.kind {
                TokenKind::LeftParen => {
                    // Parse recover()
                    self.next_token()?;
                    
                    // Consume ')'
                    if let Some(token) = self.current_token.as_ref() {
                        if token.kind == TokenKind::RightParen {
                            self.next_token()?;
                        } else {
                            return Err(Error::Parse("Expected ')' after recover expression".to_string()));
                        }
                    } else {
                        return Err(Error::Parse("Expected ')' after recover expression".to_string()));
                    }
                    
                    Ok(Expression::Recover(RecoverExpression::new()))
                }
                _ => {
                    // Default recover with no parentheses
                    Ok(Expression::Recover(RecoverExpression::new()))
                }
            }
        } else {
            Ok(Expression::Recover(RecoverExpression::new()))
        }
    }

    // Parse struct statement
    pub fn parse_struct_statement(&mut self) -> Result<StructStatement> {
        // Consume 'squad' token
        self.consume_token(TokenKind::Squad)?;
        
        // Parse struct name
        let name = match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::Truth => {
                let name = token.lexeme.clone();
                self.advance_token();
                name
            }
            _ => return Err(Error::Parse("Expected struct name".to_string())),
        };
        
        // Expect '{'
        self.consume_token(TokenKind::LeftBrace)?;
        
        // Parse fields
        let mut fields = Vec::new();
        
        while let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::RightBrace {
                break;
            }
            
            // Parse field
            let field = self.parse_struct_field()?;
            fields.push(field);
            
            // Consume comma if present
            if let Some(token) = self.current_token.as_ref() {
                if token.kind == TokenKind::Comma {
                    self.advance_token();
                }
            }
        }
        
        // Expect '}'
        self.consume_token(TokenKind::RightBrace)?;
        
        Ok(StructStatement {
            name,
            fields,
            visibility: Visibility::Public,
        })
    }
    
    // Parse struct field
    fn parse_struct_field(&mut self) -> Result<StructField> {
        // Parse field name
        let name = match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::Truth => {
                let name = token.lexeme.clone();
                self.advance_token();
                name
            }
            _ => return Err(Error::Parse("Expected field name".to_string())),
        };
        
        // Parse field type
        let field_type = if let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::Identifier {
                self.parse_type()?
            } else {
                None
            }
        } else {
            None
        };
        
        Ok(StructField {
            name,
            field_type: field_type,
            visibility: Visibility::Public,
        })
    }

    fn parse_interface_statement(&mut self) -> Result<InterfaceStatement> {
        // Consume 'collab' keyword
        self.consume_token(TokenKind::Collab)?;
        
        // Parse interface name
        let name = match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::Identifier => {
                let name = token.lexeme.clone();
                self.advance_token();
                name
            }
            _ => return Err(Error::Parse("Expected interface name".to_string())),
        };
        
        // Parse optional generic type parameters
        let type_parameters = if let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::Less || token.kind == TokenKind::LeftAngle {
                self.parse_generic_type_parameters_angle_brackets()?
            } else if token.kind == TokenKind::LeftBracket {
                self.parse_generic_type_parameters()?
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };
        
        // Parse optional interface inheritance (extends clause)
        let extends = if let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::Colon {
                self.advance_token(); // consume ':'
                self.parse_interface_inheritance_list()?
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };
        
        // Expect '{'
        self.consume_token(TokenKind::LeftBrace)?;
        
        // Parse method signatures
        let mut methods = Vec::new();
        
        while let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::RightBrace {
                break;
            }
            
            // Skip newlines and whitespace
            if token.kind == TokenKind::Newline {
                self.advance_token();
                continue;
            }
            
            // Only parse method signatures that start with 'slay'
            if token.kind == TokenKind::Slay {
                let method = self.parse_interface_method_signature()?;
                methods.push(method);
                
                // Consume comma if present
                if let Some(token) = self.current_token.as_ref() {
                    if token.kind == TokenKind::Comma {
                        self.advance_token();
                    }
                }
            } else {
                // Unexpected token in interface body
                return Err(Error::Parse(format!("Expected 'slay' keyword or '}}' in interface, found '{}'", token.lexeme)));
            }
        }
        
        // Expect '}'
        self.consume_token(TokenKind::RightBrace)?;
        
        Ok(InterfaceStatement {
            name,
            type_parameters,
            extends,
            compositions: Vec::new(), // TODO: Parse compositions
            methods,
            visibility: Visibility::Public,
        })
    }

    fn parse_generic_type_parameters(&mut self) -> Result<Vec<TypeParameter>> {
        self.consume_token(TokenKind::LeftBracket)?;
        let mut type_parameters = Vec::new();
        
        while let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::RightBracket {
                break;
            }
            
            // Parse type parameter name
            let name = match token {
                token if token.kind == TokenKind::Truth => {
                    let name = token.lexeme.clone();
                    self.advance_token();
                    name
                }
                _ => return Err(Error::Parse("Expected type parameter name".to_string())),
            };
            
            // Parse optional bounds (constraints)
            let bounds = if let Some(token) = self.current_token.as_ref() {
                if token.kind == TokenKind::Colon {
                    self.advance_token(); // consume ':'
                    self.parse_type_bounds()?
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            };
            
            type_parameters.push(TypeParameter { name, bounds });
            
            // Consume comma if present
            if let Some(token) = self.current_token.as_ref() {
                if token.kind == TokenKind::Comma {
                    self.advance_token();
                } else if token.kind != TokenKind::RightBracket {
                    return Err(Error::Parse("Expected ',' or ']' in type parameter list".to_string()));
                }
            }
        }
        
        self.consume_token(TokenKind::RightBracket)?;
        Ok(type_parameters)
    }

    fn parse_type_bounds(&mut self) -> Result<Vec<String>> {
        let mut bounds = Vec::new();
        
        loop {
            if let Some(token) = self.current_token.as_ref() {
                if token.kind == TokenKind::Identifier {
                    // Parse complex type bound with potential generic parameters
                    let mut bound = token.lexeme.clone();
                    self.advance_token();
                    
                    // Check for generic parameters in bound (e.g., "Container[T]")
                    if let Some(next_token) = self.current_token.as_ref() {
                        if next_token.kind == TokenKind::LeftBracket {
                            bound.push('[');
                            self.advance_token(); // consume '['
                            
                            // Parse generic parameters within bound with proper nesting
                            let mut depth = 1;
                            while depth > 0 && self.current_token.is_some() {
                                if let Some(token) = self.current_token.as_ref() {
                                    match token.kind {
                                        TokenKind::LeftBracket => {
                                            depth += 1;
                                            bound.push('[');
                                        }
                                        TokenKind::RightBracket => {
                                            depth -= 1;
                                            bound.push(']');
                                        }
                                        TokenKind::Truth => {
                                            bound.push_str(&token.lexeme);
                                        }
                                        TokenKind::Comma => {
                                            bound.push(',');
                                        }
                                        TokenKind::Colon => {
                                            bound.push(':');
                                        }
                                        _ => {
                                            // Include other tokens as part of complex bound
                                            bound.push_str(&token.lexeme);
                                        }
                                    }
                                    self.advance_token();
                                }
                            }
                        }
                    }
                    
                    bounds.push(bound);
                    
                    // Check for '+' separator for compound bounds (e.g., "T: Clone + Display")
                    if let Some(token) = self.current_token.as_ref() {
                        if token.kind == TokenKind::Plus {
                            self.advance_token();
                            continue; // Parse next bound
                        }
                    }
                }
                break;
            } else {
                break;
            }
        }
        
        Ok(bounds)
    }

    /// Parse generic type parameters with angle brackets <T, U>
    fn parse_generic_type_parameters_angle_brackets(&mut self) -> Result<Vec<TypeParameter>> {
        // Accept either < or LeftAngle token
        if let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::Less {
                self.consume_token(TokenKind::Less)?;
            } else {
                self.consume_token(TokenKind::LeftAngle)?;
            }
        }
        
        let mut type_parameters = Vec::new();
        
        while let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::Greater || token.kind == TokenKind::RightAngle {
                break;
            }
            
            // Parse type parameter name
            let name = match token.kind {
                TokenKind::Identifier => {
                    let name = token.lexeme.clone();
                    self.advance_token();
                    name
                }
                _ => return Err(Error::Parse("Expected type parameter name".to_string())),
            };
            
            // Parse optional bounds (constraints)
            let bounds = if let Some(token) = self.current_token.as_ref() {
                if token.kind == TokenKind::Colon {
                    self.advance_token(); // consume ':'
                    self.parse_type_bounds_angle_brackets()?
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            };
            
            type_parameters.push(TypeParameter { name, bounds });
            
            // Consume comma if present
            if let Some(token) = self.current_token.as_ref() {
                if token.kind == TokenKind::Comma {
                    self.advance_token();
                } else if token.kind != TokenKind::Greater && token.kind != TokenKind::RightAngle {
                    return Err(Error::Parse("Expected ',' or '>' in type parameter list".to_string()));
                }
            }
        }
        
        // Accept either > or RightAngle token
        if let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::Greater {
                self.consume_token(TokenKind::Greater)?;
            } else {
                self.consume_token(TokenKind::RightAngle)?;
            }
        }
        
        Ok(type_parameters)
    }

    /// Parse type bounds with angle bracket support
    fn parse_type_bounds_angle_brackets(&mut self) -> Result<Vec<String>> {
        let mut bounds = Vec::new();
        
        loop {
            if let Some(token) = self.current_token.as_ref() {
                if token.kind == TokenKind::Identifier {
                    let mut bound = token.lexeme.clone();
                    self.advance_token();
                    
                    // Check for generic parameters in bound (e.g., "Container<T>")
                    if let Some(next_token) = self.current_token.as_ref() {
                        if next_token.kind == TokenKind::Less || next_token.kind == TokenKind::LeftAngle {
                            bound.push('<');
                            self.advance_token(); // consume '<'
                            
                            // Parse generic parameters within bound with proper nesting
                            let mut depth = 1;
                            while depth > 0 && self.current_token.is_some() {
                                if let Some(token) = self.current_token.as_ref() {
                                    match token.kind {
                                        TokenKind::Less | TokenKind::LeftAngle => {
                                            depth += 1;
                                            bound.push('<');
                                        }
                                        TokenKind::Greater | TokenKind::RightAngle => {
                                            depth -= 1;
                                            bound.push('>');
                                        }
                                        _ => {
                                            bound.push_str(&token.lexeme);
                                        }
                                    }
                                    self.advance_token();
                                }
                            }
                        }
                    }
                    
                    bounds.push(bound);
                } else {
                    break;
                }
            } else {
                break;
            }
            
            // Check for '+' to continue parsing more bounds
            if let Some(token) = self.current_token.as_ref() {
                if token.kind == TokenKind::Plus {
                    self.advance_token(); // consume '+'
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        Ok(bounds)
    }

    fn parse_interface_inheritance_list(&mut self) -> Result<Vec<String>> {
        let mut extends = Vec::new();
        
        loop {
            if let Some(token) = self.current_token.as_ref() {
                if token.kind == TokenKind::Identifier {
                    extends.push(token.lexeme.clone());
                    self.advance_token();
                    
                    // Check for ',' separator for multiple base interfaces
                    if let Some(token) = self.current_token.as_ref() {
                        if token.kind == TokenKind::Comma {
                            self.advance_token();
                            continue;
                        }
                    }
                }
                break;
            } else {
                break;
            }
        }
        
        Ok(extends)
    }
    
    fn parse_method_signature(&mut self) -> Result<MethodSignature> {
        // Check for receiver syntax: slay (receiver Type) method()
        let receiver = if let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::LeftParen {
                self.parse_method_receiver()?
            } else {
                None
            }
        } else {
            None
        };
        
        // Parse method name
        let name = match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::Truth => {
                let name = token.lexeme.clone();
                self.advance_token();
                name
            }
            _ => return Err(Error::Parse("Expected method name".to_string())),
        };
        
        // Parse parameters
        self.consume_token(TokenKind::LeftParen)?;
        let mut parameters = Vec::new();
        
        while let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::RightParen {
                break;
            }
            
            let param = self.parse_parameter()?;
            parameters.push(param);
            
            // Consume comma if present
            if let Some(token) = self.current_token.as_ref() {
                if token.kind == TokenKind::Comma {
                    self.advance_token();
                }
            }
        }
        
        self.consume_token(TokenKind::RightParen)?;
        
        // Parse return type (optional)
        let return_type = if let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::Identifier {
                self.parse_type()?
            } else {
                None
            }
        } else {
            None
        };
        
        Ok(MethodSignature {
            name,
            receiver,
            parameters,
            return_type,
        })
    }

    fn parse_method_receiver(&mut self) -> Result<Option<MethodReceiver>> {
        self.consume_token(TokenKind::LeftParen)?;
        
        // Enhanced receiver parsing with better error recovery
        let mut is_pointer = false;
        let mut name = String::new();
        let mut receiver_type = Type::Void;
        
        // Check for pointer receiver with improved handling
        if let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::Star {
                self.advance_token();
                is_pointer = true;
            }
        }
        
        // Parse receiver name with enhanced error recovery
        if let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::Identifier {
                name = token.lexeme.clone();
                self.advance_token();
            } else {
                // Enhanced error recovery - try to continue parsing
                self.error_count += 1;
                return Err(Error::Parse(format!("Expected receiver name, found {:?}", token.kind)));
            }
        } else {
            return Err(Error::Parse("Unexpected end of input while parsing receiver".to_string()));
        }
        
        // Parse receiver type with support for complex generics
        if let Some(parsed_type) = self.parse_type()? {
            receiver_type = parsed_type;
        } else {
            // Enhanced error handling for missing receiver type
            self.error_count += 1;
            return Err(Error::Parse(format!("Expected receiver type after '{}'", name)));
        }
        
        // Consume closing paren with error recovery
        if let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::RightParen {
                self.advance_token();
            } else {
                // Try to recover by finding the next ')' 
                self.error_count += 1;
                self.recover_to_token(TokenKind::RightParen);
            }
        } else {
            return Err(Error::Parse("Expected ')' after receiver type".to_string()));
        }
        
        Ok(Some(MethodReceiver {
            name,
            receiver_type,
            is_pointer,
        }))
    }
    
    fn parse_interface_method_signature(&mut self) -> Result<MethodSignature> {
        // Expect 'slay' keyword
        if self.current_token.as_ref().map(|t| &t.kind) != Some(&TokenKind::Slay) {
            return Err(Error::Parse("Expected 'slay' keyword for method".to_string()));
        }
        self.advance_token(); // consume 'slay'
        
        // Parse method name
        let name = match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::Identifier => {
                let name = token.lexeme.clone();
                self.advance_token();
                name
            }
            _ => return Err(Error::Parse("Expected method name".to_string())),
        };
        
        // Parse parameters
        let parameters = self.parse_interface_method_parameters()?;
        
        // Parse return type (optional)
        let return_type = if let Some(token) = self.current_token.as_ref() {
            match token.kind {
                TokenKind::Normie => { self.advance_token(); Some(Type::Normie) },
                TokenKind::Tea => { self.advance_token(); Some(Type::Tea) },
                TokenKind::Lit => { self.advance_token(); Some(Type::Lit) },
                TokenKind::Sip => { self.advance_token(); Some(Type::Sip) },
                _ => None
            }
        } else {
            None
        };
        
        Ok(MethodSignature {
            name,
            receiver: None, // Interface methods don't have receivers
            parameters,
            return_type,
        })
    }
    
    fn parse_interface_method_parameters(&mut self) -> Result<Vec<Parameter>> {
        // Expect '('
        if self.current_token.as_ref().map(|t| &t.kind) != Some(&TokenKind::LeftParen) {
            return Err(Error::Parse("Expected '(' for method parameters".to_string()));
        }
        self.advance_token(); // consume '('
        
        let mut parameters = Vec::new();
        
        // Parse parameters
        while self.current_token.as_ref().map(|t| &t.kind) != Some(&TokenKind::RightParen) {
            // Skip newlines
            if self.current_token.as_ref().map(|t| &t.kind) == Some(&TokenKind::Newline) {
                self.advance_token();
                continue;
            }
            
            // Parse parameter name
            let name = match self.current_token.as_ref() {
                Some(token) if token.kind == TokenKind::Truth => {
                    let name = token.lexeme.clone();
                    self.advance_token();
                    name
                }
                _ => return Err(Error::Parse("Expected parameter name".to_string())),
            };
            
            // Parse parameter type
            let param_type = self.parse_type()?;
            
            parameters.push(Parameter {
                name,
                param_type,
            });
            
            // Check for comma
            if self.current_token.as_ref().map(|t| &t.kind) == Some(&TokenKind::Comma) {
                self.advance_token(); // consume ','
            } else {
                break;
            }
        }
        
        // Expect ')'
        if self.current_token.as_ref().map(|t| &t.kind) == Some(&TokenKind::RightParen) {
            self.advance_token(); // consume ')'
        }
        
        Ok(parameters)
    }
    
    fn parse_stan_statement(&mut self) -> Result<GoroutineStatement> {
        // Consume 'stan' keyword
        self.next_token()?;
        
        // Parse the expression (function call or block)
        let expression = self.parse_expression()?;
        
        Ok(GoroutineStatement {
            expression,
        })
    }
    
    fn parse_type_alias_statement(&mut self) -> Result<TypeAliasStatement> {
        // Consume 'be_like' keyword
        self.next_token()?;
        
        // Parse type alias name
        let name = match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::Truth => {
                let name = token.lexeme.clone();
                self.next_token()?;
                name
            }
            _ => return Err(Error::Parse("Expected identifier after 'be_like'".to_string())),
        };
        
        // Expect '='
        match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::Equal => {
                self.next_token()?;
            }
            _ => return Err(Error::Parse("Expected '=' after type alias name".to_string())),
        }
        
        // Parse target type
        let target_type = self.parse_type()?.unwrap_or(Type::Custom("unknown".to_string()));
        
        Ok(TypeAliasStatement {
            name,
            target_type,
            visibility: Visibility::Private, // Default to private
        })
    }
    

    
    fn parse_flex_statement(&mut self) -> Result<ForStatement> {
        // Consume 'flex' keyword
        self.next_token()?;
        
        // Parse range expression: flex variable in start..end
        let variable = match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::Truth => {
                let var = token.lexeme.clone();
                self.next_token()?;
                var
            }
            _ => return Err(Error::Parse("Expected variable name after 'flex'".to_string())),
        };
        
        // Expect 'in'
        match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::In => {
                self.next_token()?;
            }
            _ => return Err(Error::Parse("Expected 'in' after variable in flex loop".to_string())),
        }
        
        // Parse range expression (start..end)
        let start = self.parse_expression()?;
        
        // Expect '..'
        match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::DotDot => {
                self.next_token()?;
            }
            _ => return Err(Error::Parse("Expected '..' in range expression".to_string())),
        }
        
        let end = self.parse_expression()?;
        
        // Expect '{'
        match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::LeftBrace => {
                self.next_token()?;
            }
            _ => return Err(Error::Parse("Expected '{' after range expression".to_string())),
        }
        
        // Parse body
        let mut body = Vec::new();
        while let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::RightBrace {
                self.next_token()?;
                break;
            }
            
            if token.kind == TokenKind::Newline {
                self.next_token()?;
                continue;
            }
            
            if let Some(stmt) = self.parse_statement()? {
                body.push(stmt);
            }
        }
        
        // Create a for loop with range initialization
        Ok(ForStatement {
            init: Some(Box::new(Statement::Let(LetStatement {
                target: LetTarget::Single(variable.clone()),
                var_type: None,
                value: start,
                visibility: Visibility::Private,
            }))),
            condition: Some(Expression::Binary(BinaryExpression {
                left: Box::new(Expression::Identifier(variable.clone())),
                operator: "<".to_string(),
                right: Box::new(end),
            })),
            update: Some(Expression::Increment(IncrementExpression {
                variable: variable,
                is_prefix: false,
            })),
            body,
        })
    }

    /// Parse enhanced pattern switch statement (mega_vibe_check keyword)
    fn parse_pattern_switch_statement(&mut self) -> Result<PatternSwitchStatement> {
        // Consume pattern switch keyword
        self.next_token()?;
        
        // Optional init statement (simplified for now)
        let init = None;
        
        // Parse switch expression
        let expression = self.parse_expression()?;
        
        // Expect '{'
        match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::LeftBrace => {
                self.next_token()?;
            }
            _ => return Err(Error::Parse("Expected '{' after pattern switch expression".to_string())),
        }
        
        let mut cases = Vec::new();
        let mut default_case = None;
        
        // Parse pattern cases
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
            
            // Parse 'mood' keyword or 'basic' (default) case
            if token.kind == TokenKind::Mood {
                self.next_token()?;
                
                // Parse pattern
                let pattern = self.parse_pattern()?;
                
                // Optional guard (when keyword)
                let guard = if let Some(token) = self.current_token.as_ref() {
                    if matches!(token.kind, TokenKind::Identifier) && token.lexeme == "when" {
                        self.next_token()?; // consume 'when'
                        Some(self.parse_expression()?)
                    } else {
                        None
                    }
                } else {
                    None
                };
                
                // Expect ':'
                match self.current_token.as_ref() {
                    Some(token) if token.kind == TokenKind::Colon => {
                        self.next_token()?;
                    }
                    _ => return Err(Error::Parse("Expected ':' after pattern case".to_string())),
                }
                
                // Parse case body
                let body = self.parse_pattern_case_body()?;
                
                cases.push(PatternSwitchCase {
                    pattern,
                    guard,
                    body,
                });
            } else if token.kind == TokenKind::Basic {
                // Parse default case
                self.next_token()?;
                
                // Expect ':'
                match self.current_token.as_ref() {
                    Some(token) if token.kind == TokenKind::Colon => {
                        self.next_token()?;
                    }
                    _ => return Err(Error::Parse("Expected ':' after 'basic' in pattern switch".to_string())),
                }
                
                // Parse default case body
                let body = self.parse_pattern_case_body()?;
                default_case = Some(body);
            } else {
                return Err(Error::Parse("Expected 'mood' or 'basic' in pattern switch statement".to_string()));
            }
        }
        
        Ok(PatternSwitchStatement {
            init,
            expression,
            cases,
            default_case,
        })
    }

    /// Parse pattern switch using vibe_check syntax with mood patterns
    fn parse_pattern_switch_vibe_check(&mut self) -> Result<PatternSwitchStatement> {
        // Consume 'vibe_check' keyword
        self.next_token()?;
        
        // Parse switch expression
        let expression = self.parse_expression()?;
        
        // Expect '{'
        match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::LeftBrace => {
                self.next_token()?;
            }
            _ => return Err(Error::Parse("Expected '{' after vibe_check expression".to_string())),
        }
        
        let mut cases = Vec::new();
        let mut default_case = None;
        
        // Parse pattern cases
        while let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::RightBrace {
                self.next_token()?;
                break;
            }
            
            match &token.kind {
                TokenKind::Mood => {
                    // Parse mood case
                    self.next_token()?;
                    
                    // Parse pattern(s) - can be multiple patterns separated by commas
                    let mut patterns = Vec::new();
                    
                    loop {
                        let pattern = self.parse_pattern_expression()?;
                        patterns.push(pattern);
                        
                        if let Some(token) = self.current_token.as_ref() {
                            if token.kind == TokenKind::Comma {
                                self.next_token()?; // consume comma
                                continue;
                            }
                        }
                        break;
                    }
                    
                    // Expect ':'
                    match self.current_token.as_ref() {
                        Some(token) if token.kind == TokenKind::Colon => {
                            self.next_token()?;
                        }
                        _ => return Err(Error::Parse("Expected ':' after mood pattern".to_string())),
                    }
                    
                    // Parse case body (statements until next mood/basic or })
                    let mut body = Vec::new();
                    while let Some(token) = self.current_token.as_ref() {
                        if matches!(token.kind, TokenKind::Mood | TokenKind::Basic | TokenKind::RightBrace) {
                            break;
                        }
                        if let Some(statement) = self.parse_statement()? {
                            body.push(statement);
                        }
                    }
                    
                    // For multiple patterns, create separate cases
                    for pattern in patterns {
                        cases.push(PatternSwitchCase {
                            pattern,
                            guard: None, // No guard for now in vibe_check syntax
                            body: body.clone(),
                        });
                    }
                }
                TokenKind::Basic => {
                    // Parse default case
                    self.next_token()?;
                    
                    // Expect ':'
                    match self.current_token.as_ref() {
                        Some(token) if token.kind == TokenKind::Colon => {
                            self.next_token()?;
                        }
                        _ => return Err(Error::Parse("Expected ':' after 'basic'".to_string())),
                    }
                    
                    // Parse default body
                    let mut body = Vec::new();
                    while let Some(token) = self.current_token.as_ref() {
                        if matches!(token.kind, TokenKind::Mood | TokenKind::Basic | TokenKind::RightBrace) {
                            break;
                        }
                        if let Some(statement) = self.parse_statement()? {
                            body.push(statement);
                        }
                    }
                    
                    default_case = Some(body);
                }
                _ => {
                    return Err(Error::Parse("Expected 'mood' or 'basic' in vibe_check statement".to_string()));
                }
            }
        }
        
        Ok(PatternSwitchStatement {
            init: None,
            expression,
            cases,
            default_case,
        })
    }
    
    /// Parse a pattern expression for pattern matching
    fn parse_pattern_expression(&mut self) -> Result<PatternExpression> {
        // For now, implement simple literal patterns
        match self.current_token.as_ref() {
            Some(token) => {
                match &token.kind {
                    TokenKind::Integer(_) | TokenKind::String | TokenKind::Boolean | TokenKind::Character => {
                        let expr = self.parse_primary_expression()?;
                        Ok(PatternExpression::Literal(expr))
                    }
                    TokenKind::Identifier => {
                        // Check if this is a wildcard (_) or a variable binding
                        if let Some(Token { kind: TokenKind::Identifier, lexeme, .. }) = self.current_token.as_ref() {
                            if lexeme == "_" {
                                self.next_token()?;
                                Ok(PatternExpression::Wildcard)
                            } else {
                                let name = lexeme.clone();
                                self.next_token()?;
                                Ok(PatternExpression::Variable(name))
                            }
                        } else {
                            return Err(Error::Parse("Expected identifier value in pattern".to_string()));
                        }
                    }
                    _ => {
                        // Try to parse as literal expression
                        let expr = self.parse_primary_expression()?;
                        Ok(PatternExpression::Literal(expr))
                    }
                }
            }
            None => Err(Error::Parse("Expected pattern expression".to_string()))
        }
    }

    /// Parse pattern expressions
    fn parse_pattern(&mut self) -> Result<PatternExpression> {
        match self.current_token.as_ref() {
            Some(token) => {
                match &token.kind {
                    TokenKind::Identifier => {
                        let name = token.lexeme.clone();
                        self.next_token()?;
                        
                        if name == "_" {
                            Ok(PatternExpression::Wildcard)
                        } else {
                            // Check if this is a range pattern (x..y)
                            if let Some(next_token) = self.current_token.as_ref() {
                                if next_token.kind == TokenKind::DotDot {
                                    self.next_token()?; // consume '..'
                                    let end = self.parse_expression()?;
                                    Ok(PatternExpression::Range {
                                        start: Expression::Identifier(name),
                                        end,
                                        inclusive: true,
                                    })
                                } else {
                                    Ok(PatternExpression::Variable(name))
                                }
                            } else {
                                Ok(PatternExpression::Variable(name))
                            }
                        }
                    }
                    TokenKind::Integer(_) => {
                        let value = token.lexeme.parse::<i64>().map_err(|_| {
                            Error::Parse("Invalid integer literal".to_string())
                        })?;
                        self.next_token()?;
                        
                        // Check for range pattern
                        if let Some(next_token) = self.current_token.as_ref() {
                            if next_token.kind == TokenKind::DotDot {
                                self.next_token()?; // consume '..'
                                let end = self.parse_expression()?;
                                Ok(PatternExpression::Range {
                                    start: Expression::Integer(value),
                                    end,
                                    inclusive: true,
                                })
                            } else {
                                Ok(PatternExpression::Literal(Expression::Integer(value)))
                            }
                        } else {
                            Ok(PatternExpression::Literal(Expression::Integer(value)))
                        }
                    }
                    TokenKind::String => {
                        let value = token.lexeme.clone();
                        self.next_token()?;
                        Ok(PatternExpression::Literal(Expression::String(value)))
                    }
                    TokenKind::Truth => {
                        self.next_token()?;
                        Ok(PatternExpression::Literal(Expression::Boolean(true)))
                    }
                    TokenKind::Lies => {
                        self.next_token()?;
                        Ok(PatternExpression::Literal(Expression::Boolean(false)))
                    }
                    TokenKind::Character => {
                        let value = token.lexeme.chars().next().unwrap_or('\0');
                        self.next_token()?;
                        
                        // Check for character range pattern 'a'..'z'
                        if let Some(next_token) = self.current_token.as_ref() {
                            if next_token.kind == TokenKind::DotDot {
                                self.next_token()?; // consume '..'
                                let end = self.parse_expression()?;
                                Ok(PatternExpression::Range {
                                    start: Expression::Character(value),
                                    end,
                                    inclusive: true,
                                })
                            } else {
                                Ok(PatternExpression::Literal(Expression::Character(value)))
                            }
                        } else {
                            Ok(PatternExpression::Literal(Expression::Character(value)))
                        }
                    }
                    TokenKind::LeftParen => {
                        // Tuple pattern
                        self.next_token()?; // consume '('
                        let mut patterns = Vec::new();
                        
                        while let Some(token) = self.current_token.as_ref() {
                            if token.kind == TokenKind::RightParen {
                                self.next_token()?;
                                break;
                            }
                            
                            patterns.push(self.parse_pattern()?);
                            
                            // Check for comma
                            if let Some(token) = self.current_token.as_ref() {
                                if token.kind == TokenKind::Comma {
                                    self.next_token()?;
                                } else if token.kind != TokenKind::RightParen {
                                    return Err(Error::Parse("Expected ',' or ')' in tuple pattern".to_string()));
                                }
                            }
                        }
                        
                        Ok(PatternExpression::Tuple(patterns))
                    }
                    _ => {
                        // Try to parse as or pattern (pattern | pattern | ...)
                        let first_pattern = self.parse_simple_pattern()?;
                        let mut patterns = vec![first_pattern];
                        
                        while let Some(token) = self.current_token.as_ref() {
                            if token.kind == TokenKind::Pipe {
                                self.next_token()?; // consume '|'
                                patterns.push(self.parse_simple_pattern()?);
                            } else {
                                break;
                            }
                        }
                        
                        if patterns.len() == 1 {
                            Ok(patterns.into_iter().next().unwrap())
                        } else {
                            Ok(PatternExpression::Or(patterns))
                        }
                    }
                }
            }
            None => Err(Error::Parse("Unexpected end of input in pattern".to_string())),
        }
    }

    /// Parse simple pattern (helper for or patterns)
    fn parse_simple_pattern(&mut self) -> Result<PatternExpression> {
        match self.current_token.as_ref() {
            Some(token) => {
                match &token.kind {
                    TokenKind::Identifier => {
                        let name = token.lexeme.clone();
                        self.next_token()?;
                        
                        if name == "_" {
                            Ok(PatternExpression::Wildcard)
                        } else {
                            Ok(PatternExpression::Variable(name))
                        }
                    }
                    TokenKind::Integer(_) => {
                        let value = token.lexeme.parse::<i64>().map_err(|_| {
                            Error::Parse("Invalid integer literal".to_string())
                        })?;
                        self.next_token()?;
                        Ok(PatternExpression::Literal(Expression::Integer(value)))
                    }
                    TokenKind::String => {
                        let value = token.lexeme.clone();
                        self.next_token()?;
                        Ok(PatternExpression::Literal(Expression::String(value)))
                    }
                    TokenKind::Truth => {
                        self.next_token()?;
                        Ok(PatternExpression::Literal(Expression::Boolean(true)))
                    }
                    TokenKind::Lies => {
                        self.next_token()?;
                        Ok(PatternExpression::Literal(Expression::Boolean(false)))
                    }
                    TokenKind::Character => {
                        let value = token.lexeme.chars().next().unwrap_or('\0');
                        self.next_token()?;
                        Ok(PatternExpression::Literal(Expression::Character(value)))
                    }
                    _ => Err(Error::Parse("Invalid pattern".to_string())),
                }
            }
            None => Err(Error::Parse("Unexpected end of input in simple pattern".to_string())),
        }
    }

    /// Parse pattern case body
    fn parse_pattern_case_body(&mut self) -> Result<Vec<Statement>> {
        let mut statements = Vec::new();
        
        // Parse statements until we hit a case or end of switch
        while let Some(token) = self.current_token.as_ref() {
            if token.kind == TokenKind::RightBrace ||
               token.kind == TokenKind::Mood ||
               token.kind == TokenKind::Basic {
                break;
            }
            
            // Skip newlines
            if token.kind == TokenKind::Newline {
                self.next_token()?;
                continue;
            }
            
            // Parse statement
            if let Some(statement) = self.parse_statement()? {
                statements.push(statement);
            }
        }
        
        Ok(statements)
    }

    /// Parse match expression (match value { pattern -> expression, ... })
    fn parse_match_expression(&mut self) -> Result<Expression> {
        // Consume 'match' keyword
        self.next_token()?;
        
        // Parse value to match against
        let value = Box::new(self.parse_expression()?);
        
        // Expect '{'
        match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::LeftBrace => {
                self.next_token()?;
            }
            _ => return Err(Error::Parse("Expected '{' after match value".to_string())),
        }
        
        let mut arms = Vec::new();
        
        // Parse match arms
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
            
            // Parse pattern
            let pattern = self.parse_match_pattern()?;
            
            // Optional guard (when keyword)
            let guard = if let Some(token) = self.current_token.as_ref() {
                if matches!(token.kind, TokenKind::Identifier) && token.lexeme == "when" {
                    self.next_token()?; // consume 'when'
                    Some(self.parse_expression()?)
                } else {
                    None
                }
            } else {
                None
            };
            
            // Expect '->'
            match self.current_token.as_ref() {
                Some(token) if token.kind == TokenKind::Arrow => {
                    self.next_token()?;
                }
                _ => return Err(Error::Parse("Expected '->' after match pattern".to_string())),
            }
            
            // Parse arm body (expression)
            let body = self.parse_expression()?;
            
            arms.push(MatchArm {
                pattern,
                guard,
                body,
            });
            
            // Optional comma
            if let Some(token) = self.current_token.as_ref() {
                if token.kind == TokenKind::Comma {
                    self.next_token()?;
                }
            }
        }
        
        Ok(Expression::Match(MatchExpression { value, arms }))
    }
    
    /// Parse match pattern
    fn parse_match_pattern(&mut self) -> Result<MatchPattern> {
        match self.current_token.as_ref() {
            Some(token) => {
                match &token.kind {
                    TokenKind::Identifier => {
                        let name = token.lexeme.clone();
                        self.next_token()?;
                        
                        if name == "_" {
                            Ok(MatchPattern::Wildcard)
                        } else {
                            // Check if this is a range pattern (x..y)
                            if let Some(next_token) = self.current_token.as_ref() {
                                if next_token.kind == TokenKind::DotDot {
                                    self.next_token()?; // consume '..'
                                    let end = self.parse_expression()?;
                                    Ok(MatchPattern::Range {
                                        start: Expression::Identifier(name),
                                        end,
                                        inclusive: true,
                                    })
                                } else {
                                    Ok(MatchPattern::Variable(name))
                                }
                            } else {
                                Ok(MatchPattern::Variable(name))
                            }
                        }
                    }
                    TokenKind::Integer(_) => {
                        let value = token.lexeme.parse::<i64>().map_err(|_| {
                            Error::Parse("Invalid integer literal".to_string())
                        })?;
                        self.next_token()?;
                        
                        // Check for range pattern
                        if let Some(next_token) = self.current_token.as_ref() {
                            if next_token.kind == TokenKind::DotDot {
                                self.next_token()?; // consume '..'
                                let end = self.parse_expression()?;
                                Ok(MatchPattern::Range {
                                    start: Expression::Integer(value),
                                    end,
                                    inclusive: true,
                                })
                            } else {
                                Ok(MatchPattern::Literal(Expression::Integer(value)))
                            }
                        } else {
                            Ok(MatchPattern::Literal(Expression::Integer(value)))
                        }
                    }
                    TokenKind::String => {
                        let value = token.lexeme.clone();
                        self.next_token()?;
                        Ok(MatchPattern::Literal(Expression::String(value)))
                    }
                    TokenKind::Truth | TokenKind::Based => {
                        self.next_token()?;
                        Ok(MatchPattern::Literal(Expression::Boolean(true)))
                    }
                    TokenKind::Lies => {
                        self.next_token()?;
                        Ok(MatchPattern::Literal(Expression::Boolean(false)))
                    }
                    TokenKind::Character => {
                        let value = token.lexeme.chars().next().unwrap_or('\0');
                        self.next_token()?;
                        
                        // Check for character range pattern 'a'..'z'
                        if let Some(next_token) = self.current_token.as_ref() {
                            if next_token.kind == TokenKind::DotDot {
                                self.next_token()?; // consume '..'
                                let end = self.parse_expression()?;
                                Ok(MatchPattern::Range {
                                    start: Expression::Character(value),
                                    end,
                                    inclusive: true,
                                })
                            } else {
                                Ok(MatchPattern::Literal(Expression::Character(value)))
                            }
                        } else {
                            Ok(MatchPattern::Literal(Expression::Character(value)))
                        }
                    }
                    TokenKind::LeftParen => {
                        // Tuple pattern
                        self.next_token()?; // consume '('
                        let mut patterns = Vec::new();
                        
                        while let Some(token) = self.current_token.as_ref() {
                            if token.kind == TokenKind::RightParen {
                                self.next_token()?;
                                break;
                            }
                            
                            patterns.push(self.parse_match_pattern()?);
                            
                            // Check for comma
                            if let Some(token) = self.current_token.as_ref() {
                                if token.kind == TokenKind::Comma {
                                    self.next_token()?;
                                } else if token.kind != TokenKind::RightParen {
                                    return Err(Error::Parse("Expected ',' or ')' in tuple pattern".to_string()));
                                }
                            }
                        }
                        
                        Ok(MatchPattern::Tuple(patterns))
                    }
                    _ => {
                        // Try to parse as or pattern (pattern | pattern | ...)
                        let first_pattern = self.parse_simple_match_pattern()?;
                        let mut patterns = vec![first_pattern];
                        
                        while let Some(token) = self.current_token.as_ref() {
                            if token.kind == TokenKind::Pipe {
                                self.next_token()?; // consume '|'
                                patterns.push(self.parse_simple_match_pattern()?);
                            } else {
                                break;
                            }
                        }
                        
                        if patterns.len() == 1 {
                            Ok(patterns.into_iter().next().unwrap())
                        } else {
                            Ok(MatchPattern::Or(patterns))
                        }
                    }
                }
            }
            None => Err(Error::Parse("Unexpected end of input in match pattern".to_string())),
        }
    }
    
    /// Parse simple match pattern (helper for or patterns)
    fn parse_simple_match_pattern(&mut self) -> Result<MatchPattern> {
        match self.current_token.as_ref() {
            Some(token) => {
                match &token.kind {
                    TokenKind::Identifier => {
                        let name = token.lexeme.clone();
                        self.next_token()?;
                        
                        if name == "_" {
                            Ok(MatchPattern::Wildcard)
                        } else {
                            Ok(MatchPattern::Variable(name))
                        }
                    }
                    TokenKind::Number => {
                        let value = token.lexeme.parse::<i64>().map_err(|_| {
                            Error::Parse("Invalid integer literal".to_string())
                        })?;
                        self.next_token()?;
                        Ok(MatchPattern::Literal(Expression::Integer(value)))
                    }
                    TokenKind::String => {
                        let value = token.lexeme.clone();
                        self.next_token()?;
                        Ok(MatchPattern::Literal(Expression::String(value)))
                    }
                    TokenKind::Truth | TokenKind::Based => {
                        self.next_token()?;
                        Ok(MatchPattern::Literal(Expression::Boolean(true)))
                    }
                    TokenKind::Lies => {
                        self.next_token()?;
                        Ok(MatchPattern::Literal(Expression::Boolean(false)))
                    }
                    TokenKind::Character => {
                        let value = token.lexeme.chars().next().unwrap_or('\0');
                        self.next_token()?;
                        Ok(MatchPattern::Literal(Expression::Character(value)))
                    }
                    _ => Err(Error::Parse("Invalid match pattern".to_string())),
                }
            }
            None => Err(Error::Parse("Unexpected end of input in simple match pattern".to_string())),
        }
    }

    /// Parse type switch expression (typecheck variable is { type -> expression, ... })
    fn parse_type_switch_expression(&mut self) -> Result<Expression> {
        // Consume 'typecheck' keyword
        self.next_token()?;
        
        // Parse variable to check type of
        let variable = Box::new(self.parse_expression()?);
        
        // Expect 'is' keyword
        match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::Identifier && token.lexeme == "is" => {
                self.next_token()?;
            }
            _ => return Err(Error::Parse("Expected 'is' after typecheck variable".to_string())),
        }
        
        // Expect '{'
        match self.current_token.as_ref() {
            Some(token) if token.kind == TokenKind::LeftBrace => {
                self.next_token()?;
            }
            _ => return Err(Error::Parse("Expected '{' after 'typecheck variable is'".to_string())),
        }
        
        let mut arms = Vec::new();
        
        // Parse type switch arms
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
            
            // Parse type pattern
            let type_pattern = self.parse_type_pattern()?;
            
            // Optional variable binding (variable_name)
            let bound_variable = if let Some(token) = self.current_token.as_ref() {
                if token.kind == TokenKind::Identifier {
                    let var_name = token.lexeme.clone();
                    self.next_token()?;
                    Some(var_name)
                } else {
                    None
                }
            } else {
                None
            };
            
            // Expect '->'
            match self.current_token.as_ref() {
                Some(token) if token.kind == TokenKind::Arrow => {
                    self.next_token()?;
                }
                _ => return Err(Error::Parse("Expected '->' after type pattern".to_string())),
            }
            
            // Parse arm body (expression)
            let body = self.parse_expression()?;
            
            arms.push(TypeSwitchArm {
                type_pattern,
                bound_variable,
                body,
            });
            
            // Optional comma
            if let Some(token) = self.current_token.as_ref() {
                if token.kind == TokenKind::Comma {
                    self.next_token()?;
                }
            }
        }
        
        Ok(Expression::TypeSwitch(TypeSwitchExpression { variable, arms }))
    }

    /// Parse type pattern for type switch
    fn parse_type_pattern(&mut self) -> Result<TypePattern> {
        match self.current_token.as_ref() {
            Some(token) => {
                match &token.kind {
                    TokenKind::Identifier => {
                        let name = token.lexeme.clone();
                        self.next_token()?;
                        
                        if name == "_" {
                            Ok(TypePattern::Wildcard)
                        } else {
                            // Check if it's an interface name (starts with uppercase)
                            if name.chars().next().unwrap_or('a').is_uppercase() {
                                Ok(TypePattern::Interface(name))
                            } else {
                                // It's a type name, create a Type::Custom
                                Ok(TypePattern::Type(Type::Custom(name)))
                            }
                        }
                    }
                    // CURSED type keywords
                    TokenKind::Normie => {
                        self.next_token()?;
                        Ok(TypePattern::Type(Type::Normie))
                    }
                    TokenKind::Tea => {
                        self.next_token()?;
                        Ok(TypePattern::Type(Type::Tea))
                    }
                    TokenKind::Lit => {
                        self.next_token()?;
                        Ok(TypePattern::Type(Type::Lit))
                    }
                    TokenKind::Sip => {
                        self.next_token()?;
                        Ok(TypePattern::Type(Type::Sip))
                    }
                    TokenKind::Smol => {
                        self.next_token()?;
                        Ok(TypePattern::Type(Type::Smol))
                    }
                    TokenKind::Mid => {
                        self.next_token()?;
                        Ok(TypePattern::Type(Type::Mid))
                    }
                    TokenKind::Thicc => {
                        self.next_token()?;
                        Ok(TypePattern::Type(Type::Thicc))
                    }
                    TokenKind::Snack => {
                        self.next_token()?;
                        Ok(TypePattern::Type(Type::Snack))
                    }
                    TokenKind::Meal => {
                        self.next_token()?;
                        Ok(TypePattern::Type(Type::Meal))
                    }
                    _ => Err(Error::Parse("Invalid type pattern".to_string())),
                }
            }
            None => Err(Error::Parse("Unexpected end of input in type pattern".to_string())),
        }
    }
}

// Factory function for creating new parser
pub fn new_parser(source: &str) -> Result<Parser> {
    let lexer = Lexer::new(source.to_string());
    Parser::new(lexer)
}
