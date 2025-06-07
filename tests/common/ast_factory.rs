//! AST Factory for test cases
//! 
//! Provides utilities for creating AST nodes for tests

use cursed::ast::Token;
use cursed::ast::declarations::SquadStatement;
use cursed::ast::declarations::FieldDeclaration;
use cursed::ast::base::Identifier;
use std::sync::Arc;

pub struct AstFactory {}

impl AstFactory {
    pub fn new() -> Self {
        AstFactory {}
    }
    
    pub fn create_generic_struct(
        &self,
        name: &str,
        type_params: Vec<&str>,
        fields: Vec<(&str, &str)>,
    ) -> SquadStatement {
        // Create the type parameters
        let type_parameters = type_params
            .iter()
            .map(|tp| Token {
                token_type: cursed::ast::TokenType::Identifier,
                literal: tp.to_string(),
                value: tp.to_string(),
            })
            .collect();
        
        // Create the fields
        let struct_fields = fields
            .iter()
            .map(|(name, type_name)| {
                FieldDeclaration {
                    name: Token {
                        token_type: cursed::ast::TokenType::Identifier,
                        literal: name.to_string(),
                        value: name.to_string(),
                    },
                    type_name: Token {
                        token_type: cursed::ast::TokenType::Identifier,
                        literal: type_name.to_string(),
                        value: type_name.to_string(),
                    },
                }
            })
            .collect();
        
        // Create the squad statement
        SquadStatement {
            name: Token {
                token_type: cursed::ast::TokenType::Identifier,
                literal: name.to_string(),
                value: name.to_string(),
            },
            type_parameters,
            generic_constraints: Vec::new(),
            fields: struct_fields,
            methods: Vec::new(),
        }
    }
}