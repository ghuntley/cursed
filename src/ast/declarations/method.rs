//! Method declarations for the CURSED language AST.
//!
//! This module defines the AST representation for method declarations,
//! which define methods that can be attached to structs (receiver methods)
//! and interface method signatures.

use crate::ast::declarations::{Parameter, TypeParameter, GenericConstraint};
use crate::ast::expressions::identifiers::Identifier;
use crate::ast::statements::block::BlockStatement;
use crate::ast::{Expression, Node, Statement};
use std::any::Any;

/// MethodDeclaration represents a method with a receiver (Go-style method)
///
/// In CURSED, methods are declared using the `slay` keyword, followed by
/// a receiver specification in parentheses, then the method name, parameters,
/// optional return type, and body.
///
/// # Examples
///
/// ```
/// slay (p Person) getName() normie {
///     yolo p.name
/// }
/// 
/// slay (p @Person) setName(name normie) {
///     p.name = name
/// }
/// ```
pub struct MethodDeclaration {
    pub token: String, // Token::Slay
    pub receiver: Receiver,
    pub name: Identifier,
    pub parameters: Vec<Parameter>,
    pub body: BlockStatement,
    pub return_type: Option<Box<dyn Expression>>,
    pub type_parameters: Vec<TypeParameter>, // Generic type parameters
    pub generic_constraints: Vec<GenericConstraint>, // Constraints on type parameters
}

/// Receiver represents the receiver of a method (the instance it operates on)
///
/// In CURSED, receivers can be either value receivers or pointer receivers,
/// following Go's convention.
///
/// # Examples
///
/// ```
/// (p Person)   // value receiver
/// (p @Person)  // pointer receiver
/// ```
pub struct Receiver {
    pub token: String, // Usually the receiver name token
    pub name: Identifier, // Name of the receiver variable
    pub type_expr: Box<dyn Expression>, // Type of the receiver
    pub is_pointer: bool, // true if receiver is a pointer (@Person)
}

impl Node for MethodDeclaration {
    fn token_literal(&self) -> String {
        self.token.clone()
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

    fn string(&self) -> String {
        let mut out = String::new();

        // Format the method with receiver and optional type parameters
        let type_params_str = if !self.type_parameters.is_empty() {
            let params: Vec<String> = self
                .type_parameters
                .iter()
                .map(|param| param.string())
                .collect();
            format!("[{}] ", params.join(", "))
        } else {
            String::new()
        };
        
        // Format generic constraints if any
        let constraints_str = if !self.generic_constraints.is_empty() {
            let constraints: Vec<String> = self
                .generic_constraints
                .iter()
                .map(|c| c.string())
                .collect();
            format!(" where {} ", constraints.join(", "))
        } else {
            String::new()
        };

        out.push_str(&format!(
            "{} {}{}{} {}",
            self.token_literal(),
            self.receiver.string(),
            type_params_str,
            self.name.string(),
            constraints_str
        ));

        // Format parameters
        out.push_str("(");
        let params: Vec<String> = self.parameters.iter().map(|param| param.string()).collect();
        out.push_str(&params.join(", "));
        out.push_str(")");

        // Format return type if any
        if let Some(ret_type) = &self.return_type {
            out.push_str(&format!(" {}", ret_type.string()));
        }

        // Format body
        out.push_str(" ");
        out.push_str(&self.body.string());

        out
    }
}

impl Statement for MethodDeclaration {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for Receiver {
    fn token_literal(&self) -> String {
        self.token.clone()
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

    fn string(&self) -> String {
        let pointer_prefix = if self.is_pointer { "@" } else { "" };
        format!("({} {}{})", 
            self.name.string(), 
            pointer_prefix, 
            self.type_expr.string()
        )
    }
}
