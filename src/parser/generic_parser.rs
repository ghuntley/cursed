//! Enhanced generic parsing for CURSED
//!
//! This module provides comprehensive parsing support for generic functions,
//! types, interfaces, and constraints.

use crate::error::CursedError;
use crate::error_types::Error;
use crate::lexer::{Token, TokenKind};
use crate::ast::{
    TypeParameter, Type, FunctionDeclaration, StructDeclaration, InterfaceStatement,
    Expression, Statement, Parameter, Visibility
};
use crate::type_system::generic_constraints::{
    TypeConstraint, WhereClause, InterfaceDefinition, InterfaceMethod, AssociatedType
};
use crate::type_system::TypeExpression;

/// Enhanced parser for generic constructs
pub struct GenericParser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> GenericParser<'a> {
    /// Create a new generic parser
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, current: 0 }
    }

    /// Parse a generic function declaration
    /// Syntax: slay function_name<T, U: Display>(param1 T, param2 U) -> T where T: Clone
    pub fn parse_generic_function(&mut self) -> Result<GenericFunctionDeclaration, CursedError> {
        // Parse 'slay' keyword
        self.expect_token(TokenKind::Slay)?;
        
        // Parse function name
        let name = self.parse_identifier()?;
        
        // Parse generic type parameters
        let type_parameters = if self.current_token_is(&TokenKind::Less) {
            self.parse_generic_parameters()?
        } else {
            Vec::new()
        };
        
        // Parse function parameters
        self.expect_token(TokenKind::LeftParen)?;
        let parameters = self.parse_function_parameters()?;
        self.expect_token(TokenKind::RightParen)?;
        
        // Parse return type
        let return_type = if self.current_token_is(&TokenKind::Arrow) {
            self.advance(); // consume '->'
            Some(self.parse_type()?)
        } else {
            None
        };
        
        // Parse where clause
        let where_clause = if self.current_token_is_identifier("where") {
            self.advance(); // consume 'where'
            Some(self.parse_where_clause()?)
        } else {
            None
        };
        
        // Parse function body
        self.expect_token(TokenKind::LeftBrace)?;
        let body = self.parse_statements_until(TokenKind::RightBrace)?;
        self.expect_token(TokenKind::RightBrace)?;
        
        Ok(GenericFunctionDeclaration {
            name,
            type_parameters,
            parameters,
            return_type,
            where_clause,
            body,
            visibility: Visibility::Public,
        })
    }

    /// Parse a generic struct declaration
    /// Syntax: struct Container<T, U: Display> { field1: T, field2: U } where T: Clone
    pub fn parse_generic_struct(&mut self) -> Result<GenericStructDeclaration, CursedError> {
        // Parse 'struct' keyword
        self.expect_token(TokenKind::Struct)?;
        
        // Parse struct name
        let name = self.parse_identifier()?;
        
        // Parse generic type parameters
        let type_parameters = if self.current_token_is(&TokenKind::Less) {
            self.parse_generic_parameters()?
        } else {
            Vec::new()
        };
        
        // Parse where clause (before body)
        let where_clause = if self.current_token_is_identifier("where") {
            self.advance(); // consume 'where'
            Some(self.parse_where_clause()?)
        } else {
            None
        };
        
        // Parse struct body
        self.expect_token(TokenKind::LeftBrace)?;
        let fields = self.parse_struct_fields()?;
        self.expect_token(TokenKind::RightBrace)?;
        
        Ok(GenericStructDeclaration {
            name,
            type_parameters,
            fields,
            where_clause,
            visibility: Visibility::Public,
        })
    }

    /// Parse a generic interface declaration
    /// Syntax: collab Display<T> { show(self) -> tea } where T: Clone
    pub fn parse_generic_interface(&mut self) -> Result<GenericInterfaceDeclaration, CursedError> {
        // Parse 'collab' keyword
        self.expect_token(TokenKind::Collab)?;
        
        // Parse interface name
        let name = self.parse_identifier()?;
        
        // Parse generic type parameters
        let type_parameters = if self.current_token_is(&TokenKind::Less) {
            self.parse_generic_parameters()?
        } else {
            Vec::new()
        };
        
        // Parse superinterfaces
        let superinterfaces = if self.current_token_is(&TokenKind::Colon) {
            self.advance(); // consume ':'
            self.parse_superinterfaces()?
        } else {
            Vec::new()
        };
        
        // Parse where clause (before body)
        let where_clause = if self.current_token_is_identifier("where") {
            self.advance(); // consume 'where'
            Some(self.parse_where_clause()?)
        } else {
            None
        };
        
        // Parse interface body
        self.expect_token(TokenKind::LeftBrace)?;
        let methods = self.parse_interface_methods()?;
        let associated_types = self.parse_associated_types()?;
        self.expect_token(TokenKind::RightBrace)?;
        
        Ok(GenericInterfaceDeclaration {
            name,
            type_parameters,
            superinterfaces,
            methods,
            associated_types,
            where_clause,
            visibility: Visibility::Public,
        })
    }

    /// Parse generic type parameters
    /// Syntax: <T, U: Display + Clone, V = String>
    fn parse_generic_parameters(&mut self) -> Result<Vec<EnhancedTypeParameter>, CursedError> {
        self.expect_token(TokenKind::Less)?;
        let mut parameters = Vec::new();
        
        while !self.current_token_is(&TokenKind::Greater) && !self.is_at_end() {
            let param = self.parse_single_type_parameter()?;
            parameters.push(param);
            
            if self.current_token_is(&TokenKind::Comma) {
                self.advance(); // consume ','
            } else if !self.current_token_is(&TokenKind::Greater) {
                return Err(CursedError::Parse("Expected ',' or '>' in type parameter list".to_string()));
            }
        }
        
        self.expect_token(TokenKind::Greater)?;
        Ok(parameters)
    }

    /// Parse a single type parameter
    /// Syntax: T: Display + Clone = String
    fn parse_single_type_parameter(&mut self) -> Result<EnhancedTypeParameter, CursedError> {
        let name = self.parse_identifier()?;
        
        // Parse constraints
        let constraints = if self.current_token_is(&TokenKind::Colon) {
            self.advance(); // consume ':'
            self.parse_type_constraints()?
        } else {
            Vec::new()
        };
        
        // Parse default type
        let default_type = if self.current_token_is(&TokenKind::Assign) {
            self.advance(); // consume '='
            Some(self.parse_type()?)
        } else {
            None
        };
        
        Ok(EnhancedTypeParameter {
            name,
            constraints,
            default_type,
            variance: TypeVariance::Invariant,
        })
    }

    /// Parse type constraints
    /// Syntax: Display + Clone + Send
    fn parse_type_constraints(&mut self) -> Result<Vec<TypeConstraint>, CursedError> {
        let mut constraints = Vec::new();
        
        loop {
            let constraint = self.parse_single_constraint()?;
            constraints.push(constraint);
            
            if self.current_token_is(&TokenKind::Plus) {
                self.advance(); // consume '+'
            } else {
                break;
            }
        }
        
        Ok(constraints)
    }

    /// Parse a single constraint
    fn parse_single_constraint(&mut self) -> Result<TypeConstraint, CursedError> {
        let constraint_name = self.parse_identifier()?;
        
        // Check for equality constraints (T = String)
        if self.current_token_is(&TokenKind::Assign) {
            self.advance(); // consume '='
            let constraint_type = self.parse_type()?;
            Ok(TypeConstraint::Equality(TypeExpression::from_ast_type(&constraint_type)))
        } else {
            // Regular interface constraint
            Ok(TypeConstraint::Interface(constraint_name))
        }
    }

    /// Parse where clause
    /// Syntax: where T: Clone, U: Display + Send
    fn parse_where_clause(&mut self) -> Result<Vec<WhereClause>, CursedError> {
        let mut clauses = Vec::new();
        
        loop {
            let type_expr = self.parse_type_expression()?;
            self.expect_token(TokenKind::Colon)?;
            let constraints = self.parse_type_constraints()?;
            
            clauses.push(WhereClause {
                type_expr,
                constraints,
            });
            
            if self.current_token_is(&TokenKind::Comma) {
                self.advance(); // consume ','
            } else {
                break;
            }
        }
        
        Ok(clauses)
    }

    /// Parse function parameters with generic types
    fn parse_function_parameters(&mut self) -> Result<Vec<GenericParameter>, CursedError> {
        let mut parameters = Vec::new();
        
        while !self.current_token_is(&TokenKind::RightParen) && !self.is_at_end() {
            let param = self.parse_generic_parameter()?;
            parameters.push(param);
            
            if self.current_token_is(&TokenKind::Comma) {
                self.advance(); // consume ','
            } else if !self.current_token_is(&TokenKind::RightParen) {
                return Err(CursedError::Parse("Expected ',' or ')' in parameter list".to_string()));
            }
        }
        
        Ok(parameters)
    }

    /// Parse a generic parameter
    fn parse_generic_parameter(&mut self) -> Result<GenericParameter, CursedError> {
        let name = self.parse_identifier()?;
        let param_type = self.parse_type()?;
        
        Ok(GenericParameter {
            name,
            param_type,
            is_mutable: false,
        })
    }

    /// Parse struct fields
    fn parse_struct_fields(&mut self) -> Result<Vec<GenericField>, CursedError> {
        let mut fields = Vec::new();
        
        while !self.current_token_is(&TokenKind::RightBrace) && !self.is_at_end() {
            let field = self.parse_generic_field()?;
            fields.push(field);
            
            if self.current_token_is(&TokenKind::Comma) {
                self.advance(); // consume ','
            } else if !self.current_token_is(&TokenKind::RightBrace) {
                return Err(CursedError::Parse("Expected ',' or '}' in struct field list".to_string()));
            }
        }
        
        Ok(fields)
    }

    /// Parse a generic field
    fn parse_generic_field(&mut self) -> Result<GenericField, CursedError> {
        let name = self.parse_identifier()?;
        self.expect_token(TokenKind::Colon)?;
        let field_type = self.parse_type()?;
        
        Ok(GenericField {
            name,
            field_type,
            visibility: Visibility::Public,
        })
    }

    /// Parse interface methods
    fn parse_interface_methods(&mut self) -> Result<Vec<GenericInterfaceMethod>, CursedError> {
        let mut methods = Vec::new();
        
        while !self.current_token_is(&TokenKind::RightBrace) && !self.is_at_end() {
            if self.current_token_is_identifier("type") {
                // Skip associated type declarations (handled separately)
                self.skip_associated_type_declaration()?;
            } else {
                let method = self.parse_interface_method()?;
                methods.push(method);
            }
        }
        
        Ok(methods)
    }

    /// Parse an interface method
    fn parse_interface_method(&mut self) -> Result<GenericInterfaceMethod, CursedError> {
        let name = self.parse_identifier()?;
        
        // Parse generic parameters for the method
        let type_parameters = if self.current_token_is(&TokenKind::Less) {
            self.parse_generic_parameters()?
        } else {
            Vec::new()
        };
        
        self.expect_token(TokenKind::LeftParen)?;
        let parameters = self.parse_function_parameters()?;
        self.expect_token(TokenKind::RightParen)?;
        
        // Parse return type
        let return_type = if self.current_token_is(&TokenKind::Arrow) {
            self.advance(); // consume '->'
            Some(self.parse_type()?)
        } else {
            None
        };
        
        // Parse where clause
        let where_clause = if self.current_token_is_identifier("where") {
            self.advance(); // consume 'where'
            Some(self.parse_where_clause()?)
        } else {
            None
        };
        
        Ok(GenericInterfaceMethod {
            name,
            type_parameters,
            parameters,
            return_type,
            where_clause,
            is_static: false,
            has_default: false,
        })
    }

    /// Parse associated types
    fn parse_associated_types(&mut self) -> Result<Vec<GenericAssociatedType>, CursedError> {
        let mut associated_types = Vec::new();
        
        // This would parse associated type declarations like:
        // type Output: Display = String;
        while self.current_token_is_identifier("type") {
            let assoc_type = self.parse_associated_type()?;
            associated_types.push(assoc_type);
        }
        
        Ok(associated_types)
    }

    /// Parse an associated type
    fn parse_associated_type(&mut self) -> Result<GenericAssociatedType, CursedError> {
        self.expect_identifier("type")?;
        let name = self.parse_identifier()?;
        
        // Parse constraints
        let constraints = if self.current_token_is(&TokenKind::Colon) {
            self.advance(); // consume ':'
            self.parse_type_constraints()?
        } else {
            Vec::new()
        };
        
        // Parse default type
        let default_type = if self.current_token_is(&TokenKind::Assign) {
            self.advance(); // consume '='
            Some(self.parse_type()?)
        } else {
            None
        };
        
        self.expect_token(TokenKind::Semicolon)?;
        
        Ok(GenericAssociatedType {
            name,
            constraints,
            default_type,
        })
    }

    /// Parse superinterfaces
    fn parse_superinterfaces(&mut self) -> Result<Vec<String>, CursedError> {
        let mut superinterfaces = Vec::new();
        
        loop {
            let interface_name = self.parse_identifier()?;
            superinterfaces.push(interface_name);
            
            if self.current_token_is(&TokenKind::Plus) {
                self.advance(); // consume '+'
            } else {
                break;
            }
        }
        
        Ok(superinterfaces)
    }

    /// Utility methods
    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn current_token_is(&self, kind: &TokenKind) -> bool {
        self.current_token().map_or(false, |t| &t.kind == kind)
    }

    fn current_token_is_identifier(&self, expected: &str) -> bool {
        self.current_token().map_or(false, |t| {
            t.kind == TokenKind::Identifier && t.lexeme == expected
        })
    }

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> Option<&Token> {
        if self.current > 0 {
            self.tokens.get(self.current - 1)
        } else {
            None
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn expect_token(&mut self, kind: TokenKind) -> Result<&Token, CursedError> {
        if self.current_token_is(&kind) {
            Ok(self.advance().unwrap())
        } else {
            Err(CursedError::Parse(format!("Expected {:?}", kind)))
        }
    }

    fn expect_identifier(&mut self, expected: &str) -> Result<&Token, CursedError> {
        if self.current_token_is_identifier(expected) {
            Ok(self.advance().unwrap())
        } else {
            Err(CursedError::Parse(format!("Expected identifier '{}'", expected)))
        }
    }

    fn parse_identifier(&mut self) -> Result<String, CursedError> {
        if let Some(token) = self.current_token() {
            if token.kind == TokenKind::Identifier {
                let name = token.lexeme.clone();
                self.advance();
                Ok(name)
            } else {
                Err(CursedError::Parse("Expected identifier".to_string()))
            }
        } else {
            Err(CursedError::Parse("Unexpected end of input".to_string()))
        }
    }

    fn parse_type(&mut self) -> Result<Type, CursedError> {
        // Simplified type parsing - would need full implementation
        let type_name = self.parse_identifier()?;
        
        // Check for generic type arguments
        if self.current_token_is(&TokenKind::Less) {
            self.advance(); // consume '<'
            let mut args = Vec::new();
            
            while !self.current_token_is(&TokenKind::Greater) && !self.is_at_end() {
                let arg = self.parse_type()?;
                args.push(arg);
                
                if self.current_token_is(&TokenKind::Comma) {
                    self.advance(); // consume ','
                } else if !self.current_token_is(&TokenKind::Greater) {
                    return Err(CursedError::Parse("Expected ',' or '>' in type argument list".to_string()));
                }
            }
            
            self.expect_token(TokenKind::Greater)?;
            Ok(Type::Generic(type_name, args))
        } else {
            Ok(Type::Custom(type_name))
        }
    }

    fn parse_type_expression(&mut self) -> Result<TypeExpression, CursedError> {
        // Simplified type expression parsing
        let type_name = self.parse_identifier()?;
        Ok(TypeExpression::named(&type_name))
    }

    fn parse_statements_until(&mut self, end_token: TokenKind) -> Result<Vec<Statement>, CursedError> {
        let mut statements = Vec::new();
        
        while !self.current_token_is(&end_token) && !self.is_at_end() {
            // Simplified statement parsing
            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }
        
        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement, CursedError> {
        // Simplified statement parsing
        let expr = self.parse_expression()?;
        Ok(Statement::Expression(expr))
    }

    fn parse_expression(&mut self) -> Result<Expression, CursedError> {
        // Simplified expression parsing
        let name = self.parse_identifier()?;
        Ok(Expression::Identifier(name))
    }

    fn skip_associated_type_declaration(&mut self) -> Result<(), CursedError> {
        // Skip until semicolon
        while !self.current_token_is(&TokenKind::Semicolon) && !self.is_at_end() {
            self.advance();
        }
        if self.current_token_is(&TokenKind::Semicolon) {
            self.advance();
        }
        Ok(())
    }
}

/// Enhanced type parameter with constraints and variance
#[derive(Debug, Clone)]
pub struct EnhancedTypeParameter {
    pub name: String,
    pub constraints: Vec<TypeConstraint>,
    pub default_type: Option<Type>,
    pub variance: TypeVariance,
}

/// Type variance annotation
#[derive(Debug, Clone, PartialEq)]
pub enum TypeVariance {
    Covariant,
    Contravariant,
    Invariant,
}

/// Generic function declaration
#[derive(Debug, Clone)]
pub struct GenericFunctionDeclaration {
    pub name: String,
    pub type_parameters: Vec<EnhancedTypeParameter>,
    pub parameters: Vec<GenericParameter>,
    pub return_type: Option<Type>,
    pub where_clause: Option<Vec<WhereClause>>,
    pub body: Vec<Statement>,
    pub visibility: Visibility,
}

/// Generic struct declaration
#[derive(Debug, Clone)]
pub struct GenericStructDeclaration {
    pub name: String,
    pub type_parameters: Vec<EnhancedTypeParameter>,
    pub fields: Vec<GenericField>,
    pub where_clause: Option<Vec<WhereClause>>,
    pub visibility: Visibility,
}

/// Generic interface declaration
#[derive(Debug, Clone)]
pub struct GenericInterfaceDeclaration {
    pub name: String,
    pub type_parameters: Vec<EnhancedTypeParameter>,
    pub superinterfaces: Vec<String>,
    pub methods: Vec<GenericInterfaceMethod>,
    pub associated_types: Vec<GenericAssociatedType>,
    pub where_clause: Option<Vec<WhereClause>>,
    pub visibility: Visibility,
}

/// Generic parameter
#[derive(Debug, Clone)]
pub struct GenericParameter {
    pub name: String,
    pub param_type: Type,
    pub is_mutable: bool,
}

/// Generic field
#[derive(Debug, Clone)]
pub struct GenericField {
    pub name: String,
    pub field_type: Type,
    pub visibility: Visibility,
}

/// Generic interface method
#[derive(Debug, Clone)]
pub struct GenericInterfaceMethod {
    pub name: String,
    pub type_parameters: Vec<EnhancedTypeParameter>,
    pub parameters: Vec<GenericParameter>,
    pub return_type: Option<Type>,
    pub where_clause: Option<Vec<WhereClause>>,
    pub is_static: bool,
    pub has_default: bool,
}

/// Generic associated type
#[derive(Debug, Clone)]
pub struct GenericAssociatedType {
    pub name: String,
    pub constraints: Vec<TypeConstraint>,
    pub default_type: Option<Type>,
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parse_simple_generic_function() {
        let source = "slay max<T>(a T, b T) -> T { damn a }";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = GenericParser::new(&tokens);
        
        let result = parser.parse_generic_function();
        assert!(result.is_ok());
        
        let func = result.unwrap();
        assert_eq!(func.name, "max");
        assert_eq!(func.type_parameters.len(), 1);
        assert_eq!(func.type_parameters[0].name, "T");
        assert_eq!(func.parameters.len(), 2);
    }

    #[test]
    fn test_parse_generic_function_with_constraints() {
        let source = "slay display<T: Display>(value T) -> tea { damn value.display() }";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = GenericParser::new(&tokens);
        
        let result = parser.parse_generic_function();
        assert!(result.is_ok());
        
        let func = result.unwrap();
        assert_eq!(func.name, "display");
        assert_eq!(func.type_parameters.len(), 1);
        assert_eq!(func.type_parameters[0].name, "T");
        assert_eq!(func.type_parameters[0].constraints.len(), 1);
    }

    #[test]
    fn test_parse_generic_struct() {
        let source = "struct Container<T> { value: T }";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = GenericParser::new(&tokens);
        
        let result = parser.parse_generic_struct();
        assert!(result.is_ok());
        
        let struct_decl = result.unwrap();
        assert_eq!(struct_decl.name, "Container");
        assert_eq!(struct_decl.type_parameters.len(), 1);
        assert_eq!(struct_decl.type_parameters[0].name, "T");
        assert_eq!(struct_decl.fields.len(), 1);
        assert_eq!(struct_decl.fields[0].name, "value");
    }

    #[test]
    fn test_parse_generic_interface() {
        let source = "collab Display<T> { show(self) -> tea }";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = GenericParser::new(&tokens);
        
        let result = parser.parse_generic_interface();
        assert!(result.is_ok());
        
        let interface = result.unwrap();
        assert_eq!(interface.name, "Display");
        assert_eq!(interface.type_parameters.len(), 1);
        assert_eq!(interface.type_parameters[0].name, "T");
        assert_eq!(interface.methods.len(), 1);
        assert_eq!(interface.methods[0].name, "show");
    }

    #[test]
    fn test_parse_type_constraints() {
        let source = "Display + Clone + Send";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = GenericParser::new(&tokens);
        
        let result = parser.parse_type_constraints();
        assert!(result.is_ok());
        
        let constraints = result.unwrap();
        assert_eq!(constraints.len(), 3);
    }

    #[test]
    fn test_parse_where_clause() {
        let source = "where T: Clone, U: Display + Send";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = GenericParser::new(&tokens);
        parser.advance(); // skip 'where'
        
        let result = parser.parse_where_clause();
        assert!(result.is_ok());
        
        let where_clause = result.unwrap();
        assert_eq!(where_clause.len(), 2);
    }
}
