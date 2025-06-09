//! Struct and interface declarations for the CURSED language AST.
//!
//! This module defines the AST representation for struct and interface declarations,
//! which define composite types and interfaces in the CURSED language.

use crate::ast::declarations::Parameter;
use crate::ast::declarations::TypeParameter;
use crate::ast::declarations::GenericConstraint;
use crate::ast::documentation::DocType;
use crate::ast::expressions::identifiers::Identifier;
use crate::ast::statements::fields::FieldStatement;
use crate::ast::{Expression, Node, Statement};
use std::any::Any;

/// SquadStatement represents a struct definition
///
/// In CURSED, structs are declared using the `be_like` and `squad` keywords.
/// They can have generic type parameters and contain fields.
///
/// # Examples
///
/// ```
/// be_like Person squad {
///     name normie
///     age lit
/// }
/// ```
#[derive(Debug)]
pub struct SquadStatement {
    pub token: String, // Token::Squad
    pub name: Identifier,
    pub type_parameters: Vec<TypeParameter>, // Generic type parameters [T], [A, B], etc.
    pub generic_constraints: Vec<GenericConstraint>, // Constraints on type parameters (e.g., T: Stringer)
    pub fields: Vec<FieldStatement>,
    pub doc: Option<DocType>, // Associated documentation
}

impl Node for SquadStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();

        // Format the struct name with optional type parameters
        let type_params_str = if !self.type_parameters.is_empty() {
            let params: Vec<String> = self
                .type_parameters
                .iter()
                .map(|param| param.string())
                .collect();
            format!("[{}]", params.join(", "))
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
            "be_like {}{}{} squad {{\n",
            self.name.string(),
            type_params_str,
            constraints_str
        ));

        for field in &self.fields {
            out.push_str(&format!("    {}\n", field.string()));
        }

        out.push_str("}\n");
        out
    }
}

impl Statement for SquadStatement {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// MethodSignature represents a method signature in an interface
///
/// Method signatures define the contract that implementing types must fulfill,
/// including method name, parameters, return type, and optional generic parameters.
///
/// # Examples
///
/// ```
/// toString() normie
/// ```
#[derive(Debug)]
pub struct MethodSignature {
    pub token: String, // Usually the method name token
    pub name: Identifier,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Box<dyn Expression>>,
    pub type_parameters: Vec<TypeParameter>, // Generic type parameters for method
    pub generic_constraints: Vec<GenericConstraint>, // Constraints on method type parameters
}

impl Node for MethodSignature {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();

        // Format the method name with optional type parameters
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

        out.push_str(&format!("{}{}{}", self.name.string(), type_params_str, constraints_str));

        // Format parameters
        out.push_str("(");
        let params: Vec<String> = self.parameters.iter().map(|param| param.string()).collect();
        out.push_str(&params.join(", "));
        out.push_str(")");

        // Format return type if any
        if let Some(ret_type) = &self.return_type {
            out.push_str(&format!(": {}", ret_type.string()));
        }

        out
    }
}

impl Statement for MethodSignature {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// CollabStatement represents an interface definition
///
/// In CURSED, interfaces are declared using the `be_like` and `collab` keywords.
/// They can have generic type parameters and contain method signatures.
///
/// # Examples
///
/// ```
/// be_like Stringer collab {
///     toString() normie
/// }
/// ```
#[derive(Debug)]
pub struct CollabStatement {
    pub token: String, // Token::Collab
    pub name: Identifier,
    pub type_parameters: Vec<TypeParameter>, // Generic type parameters [T], [A, B], etc.
    pub generic_constraints: Vec<GenericConstraint>, // Constraints on type parameters (e.g., T: Stringer)
    pub methods: Vec<MethodSignature>,
    pub doc: Option<DocType>, // Associated documentation
}

impl Node for CollabStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();

        // Format the type name with optional type parameters
        let type_params_str = if !self.type_parameters.is_empty() {
            let params: Vec<String> = self
                .type_parameters
                .iter()
                .map(|param| param.string())
                .collect();
            format!("[{}]", params.join(", "))
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
            "be_like {}{}{} collab {{\n",
            self.name.string(),
            type_params_str,
            constraints_str
        ));

        for method in &self.methods {
            out.push_str(&format!("    {}\n", method.string()));
        }

        out.push_str("}\n");
        out
    }
}

impl Statement for CollabStatement {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}