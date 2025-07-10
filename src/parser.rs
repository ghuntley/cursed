// Parser module for CURSED language
use crate::ast::{Program, Ast, Statement, FunctionStatement, Parameter, Expression, LetStatement, IfStatement, ForStatement, WhileStatement, Type, Visibility, LetTarget, Literal, BinaryExpression, IncrementExpression, DecrementExpression, TupleExpression, TupleAccessExpression, MemberAccessExpression, CallExpression, AssignmentStatement, AssignmentTarget, DeferStatement, SelectStatement, SelectCase, YikesStatement, FamStatement, ShookExpression, ErrorValueExpression, InterfaceStatement, MethodSignature, TypeParameter};
use crate::lexer::{Lexer, Token, TokenKind};
use crate::error_types::{Error, Result};

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
    tokens: Vec<Token>,
    token_index: usize,
    error_count: usize,
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
            TokenKind::Lowkey => {
                // Parse if statement
                return Ok(Some(Statement::If(self.parse_if_statement()?)));
            }
            TokenKind::Bestie => {
                // Parse for loop
                return Ok(Some(Statement::For(self.parse_for_statement()?)));
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
                // Parse CURSED-style select statement
                return Ok(Some(Statement::Select(self.parse_vibe_check_statement()?)));
            }
            TokenKind::Yikes => {
                // Parse error handling statement
                return Ok(Some(Statement::Yikes(self.parse_yikes_statement()?)));
            }
            TokenKind::Fam => {
                // Parse panic recovery block
                return Ok(Some(Statement::Fam(self.parse_fam_statement()?)));
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
                            _ => Type::Custom(type_name),
                        };
                        return Ok(Some(Type::Array(Box::new(element_type), None)));
                    }
                }
            } else if token.kind == TokenKind::Identifier || token.kind == TokenKind::Normie || token.kind == TokenKind::Tea || token.kind == TokenKind::Lit || token.kind == TokenKind::Sip {
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
                                                arguments.push(self.parse_primary_expression()?);
                                                
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
                                    
                                    expr = Expression::Call(CallExpression {
                                        function: Box::new(expr),
                                        arguments,
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
                TokenKind::Truth | TokenKind::Based => {
                    // Parse boolean literal
                    self.next_token()?;
                    return Ok(Expression::Literal(Literal::Boolean(true)));
                }
                TokenKind::Yikes => {
                    // Parse error value expression
                    self.next_token()?;
                    return Ok(self.parse_yikes_expression()?);
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

    fn parse_for_statement(&mut self) -> Result<ForStatement> {
        // Consume 'bestie' keyword
        self.next_token()?;
        
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
               token.kind == TokenKind::Ready ||
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
}

// Factory function for creating new parser
pub fn new_parser(source: &str) -> Result<Parser> {
    let lexer = Lexer::new(source.to_string());
    Parser::new(lexer)
}
