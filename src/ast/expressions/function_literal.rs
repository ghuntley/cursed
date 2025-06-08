//! Function literal expressions for the CURSED language AST
//!
//! This module defines the AST nodes for function literals (anonymous functions)
//! and closures in the CURSED language. Function literals allow creating unnamed
//! functions that can capture variables from their enclosing scope and be passed
//! around as first-class values.
//!
//! # Syntax
//!
//! Function literals in CURSED use the following syntax:
//! ```cursed
//! slay(params) return_type { body }
//! ```
//!
//! # Examples
//!
//! Basic function literal:
//! ```cursed
//! let add = slay(a normie, b normie) normie {
//!     yolo a + b
//! }
//! ```
//!
//! Function literal with closure:
//! ```cursed
//! let counter = slay() normie {
//!     sus count = 0  // captured variable
//!     yolo slay() normie {
//!         count = count + 1
//!         yolo count
//!     }
//! }()
//! ```

use crate::ast::declarations::{Parameter, TypeParameter, GenericConstraint};
use crate::ast::statements::block::BlockStatement;
use crate::ast::{Expression, Node};
use std::any::Any;
use std::collections::HashSet;

/// FunctionLiteral represents an anonymous function expression
///
/// Function literals are expressions that define unnamed functions inline.
/// They can capture variables from their enclosing scope (creating closures)
/// and can be assigned to variables, passed as arguments, or returned from functions.
///
/// # Fields
///
/// * `token` - The "slay" token that starts the function literal
/// * `parameters` - The function parameters
/// * `body` - The function body
/// * `return_type` - Optional explicit return type
/// * `type_parameters` - Generic type parameters [T], [A, B], etc.
/// * `generic_constraints` - Constraints on type parameters (e.g., T: Stringer)
/// * `captured_variables` - Set of variables captured from outer scopes
/// * `capture_by_reference` - Whether captured variables are by reference or value
#[derive(Clone)]
pub struct FunctionLiteral {
    pub token: String, // Token::Slay
    pub parameters: Vec<Parameter>,
    pub body: BlockStatement,
    pub return_type: Option<Box<dyn Expression>>,
    pub type_parameters: Vec<TypeParameter>,
    pub generic_constraints: Vec<GenericConstraint>,
    pub captured_variables: HashSet<String>,
    pub capture_by_reference: HashSet<String>,
}

impl FunctionLiteral {
    /// Creates a new function literal
    pub fn new(
        token: String,
        parameters: Vec<Parameter>,
        body: BlockStatement,
        return_type: Option<Box<dyn Expression>>,
    ) -> Self {
        Self {
            token,
            parameters,
            body,
            return_type,
            type_parameters: Vec::new(),
            generic_constraints: Vec::new(),
            captured_variables: HashSet::new(),
            capture_by_reference: HashSet::new(),
        }
    }

    /// Adds a captured variable to the closure
    pub fn capture_variable(&mut self, name: String, by_reference: bool) {
        self.captured_variables.insert(name.clone());
        if by_reference {
            self.capture_by_reference.insert(name);
        }
    }

    /// Checks if a variable is captured by this closure
    pub fn captures_variable(&self, name: &str) -> bool {
        self.captured_variables.contains(name)
    }

    /// Checks if a variable is captured by reference
    pub fn captures_by_reference(&self, name: &str) -> bool {
        self.capture_by_reference.contains(name)
    }

    /// Returns the function signature as a string
    pub fn signature(&self) -> String {
        let mut sig = String::new();
        
        // Add type parameters if any
        if !self.type_parameters.is_empty() {
            let params: Vec<String> = self
                .type_parameters
                .iter()
                .map(|param| param.string())
                .collect();
            sig.push_str(&format!("[{}] ", params.join(", ")));
        }
        
        // Add parameters
        sig.push('(');
        let params: Vec<String> = self.parameters.iter().map(|param| param.string()).collect();
        sig.push_str(&params.join(", "));
        sig.push(')');
        
        // Add return type if any
        if let Some(ret_type) = &self.return_type {
            sig.push_str(&format!(" {}", ret_type.string()));
        }
        
        sig
    }
}

impl Node for FunctionLiteral {
    fn token_literal(&self) -> String {
        self.token.clone()
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

    fn string(&self) -> String {
        let mut out = String::new();

        // Format the function literal with optional type parameters
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
            "{}{}{}",
            self.token_literal(),
            type_params_str,
            constraints_str
        ));

        // Format parameters
        out.push('(');
        let params: Vec<String> = self.parameters.iter().map(|param| param.string()).collect();
        out.push_str(&params.join(", "));
        out.push(')');

        // Format return type if any
        if let Some(ret_type) = &self.return_type {
            out.push_str(&format!(" {}", ret_type.string()));
        }

        // Format body
        out.push(' ');
        out.push_str(&self.body.string());

        out
    }
}

impl Expression for FunctionLiteral {
    fn expression_node(&self) {}

    fn node_type(&self) -> &str {
        "FunctionLiteral"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

/// ClosureCapture represents a variable captured by a closure
///
/// This structure contains metadata about how a variable from an outer scope
/// is captured by a function literal, including the capture mechanism
/// (by value or by reference) and the variable's type information.
#[derive(Debug, Clone)]
pub struct ClosureCapture {
    pub name: String,
    pub by_reference: bool,
    pub variable_type: Option<String>, // Type information for the captured variable
    pub capture_index: usize, // Index in the closure's capture table
}

impl ClosureCapture {
    /// Creates a new closure capture
    pub fn new(name: String, by_reference: bool, capture_index: usize) -> Self {
        Self {
            name,
            by_reference,
            variable_type: None,
            capture_index,
        }
    }

    /// Sets the type information for the captured variable
    pub fn with_type(mut self, variable_type: String) -> Self {
        self.variable_type = Some(variable_type);
        self
    }
}
