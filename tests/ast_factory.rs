use cursed::ast::*;
use cursed::ast::*;
use cursed::ast::fields::FieldStatement;
use cursed::ast::identifiers::Identifier;
use cursed::lexer::token::Token;
use cursed::lexer::TokenType;

// AST factory for creating test AST nodes
//
// This module provides utilities for creating AST nodes for testing purposes.


/// Factory for creating test AST nodes
pub struct AstFactory;

impl AstFactory       {/// Create a new AstFactory instance
    pub fn new() {AstFactory}
    /// Create a new integer literal expression
    pub fn int_literal() {Box::new(IntegerLiteral {value})}
    
    /// Create a new string literal expression  
    pub fn string_literal() {Box::new(StringLiteral {value})}
    
    /// Create a new boolean literal expression
    pub fn bool_literal() {Box::new(BooleanLiteral {value})}
    
    /// Create a new identifier expression
    pub fn identifier() {Box::new(Identifier {token: identifier.to_string()
            value: name})}
    
    /// Create a generic struct (SquadStatement) for testing
    pub fn create_generic_struct() {let type_parameters = type_params.into_iter().map(|param| {TypeParameter::new(Token::new(TokenType::Identifier, &param.to_string(), param.to_string()}).collect()
        
        let field_statements = fields.into_iter().map(|(field_name, field_type)| {FieldStatement {name:  placeholder.to_string()
                type_name:  placeholder.to_string()"}).collect()
        
        SquadStatement {name:  placeholder.to_string()
            type_parameters,
            generic_constraints: vec![],
            fields: field_statements}