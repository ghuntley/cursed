//! AST nodes for type switch statements in the CURSED language.
//!
//! Type switches allow branching based on the runtime type of an interface value.
//! They use the `vibe_check` keyword with type patterns for each case.

use crate::ast::traits::{Expression, Node, Statement};
use std::any::Any;
use std::fmt;

/// Represents a type switch statement in the AST.
///
/// A type switch examines the runtime type of an interface value and executes
/// different code blocks based on which type case matches.
///
/// Example:
/// ```cursed
/// vibe_check value.(type) {
/// case int:
///     // handle as int
/// case string, []byte:
///     // handle as string or byte slice  
/// default:
///     // handle unknown type
/// }
/// ```
pub struct TypeSwitchStatement {
    pub token: String,                           // Token literal ("vibe_check")
    pub expression: Box<dyn Expression>,         // Expression being type-checked
    pub variable_name: Option<String>,           // Optional bound variable name
    pub cases: Vec<TypeCase>,                    // Type case branches
    pub default_case: Option<DefaultTypeCase>,   // Optional default case
}

impl Node for TypeSwitchStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        let mut result = format!("vibe_check {}.(type)", self.expression.string());
        
        if let Some(var_name) = &self.variable_name {
            result = format!("vibe_check {} := {}.(type)", var_name, self.expression.string());
        }
        
        result.push_str(" {\n");
        
        for case in &self.cases {
            result.push_str(&format!("  {}\n", case.string()));
        }
        
        if let Some(default) = &self.default_case {
            result.push_str(&format!("  {}\n", default.string()));
        }
        
        result.push('}');
        result
    }
}

impl fmt::Debug for TypeSwitchStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TypeSwitchStatement {{ token: {:?}, expression: <dyn Expression>, variable_name: {:?}, cases: {:?}, default_case: {:?} }}", 
               self.token, self.variable_name, self.cases, self.default_case)
    }
}

impl Statement for TypeSwitchStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Represents a type case in a type switch statement.
///
/// Each case can match multiple types and contains a block of statements
/// to execute when the types match.
pub struct TypeCase {
    pub types: Vec<String>,                      // List of type names to match
    pub statements: Vec<Box<dyn Statement>>,     // Statements to execute on match
}

impl std::fmt::Debug for TypeCase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TypeCase {{ types: {:?}, statements: {} statements }}", 
               self.types, self.statements.len())
    }
}

impl TypeCase {
    pub fn string(&self) -> String {
        let types_str = self.types.join(", ");
        let mut result = format!("case {}:", types_str);
        
        for stmt in &self.statements {
            result.push_str(&format!("\n    {}", stmt.string()));
        }
        
        result
    }
}

/// Represents the default case in a type switch statement.
pub struct DefaultTypeCase {
    pub statements: Vec<Box<dyn Statement>>,     // Statements to execute when no types match
}

impl std::fmt::Debug for DefaultTypeCase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DefaultTypeCase {{ statements: {} statements }}", 
               self.statements.len())
    }
}

impl DefaultTypeCase {
    pub fn string(&self) -> String {
        let mut result = "default:".to_string();
        
        for stmt in &self.statements {
            result.push_str(&format!("\n    {}", stmt.string()));
        }
        
        result
    }
}

/// Represents a type pattern expression in type switch cases.
///
/// This is used to represent type patterns like `int`, `string`, `[]byte`, etc.
pub struct TypePattern {
    pub token: String,        // Token literal
    pub type_name: String,    // Name of the type being matched
}

impl Node for TypePattern {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        self.type_name.clone()
    }
}

impl fmt::Debug for TypePattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TypePattern {{ token: {:?}, type_name: {:?} }}", 
               self.token, self.type_name)
    }
}

impl Expression for TypePattern {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(TypePattern {
            token: self.token.clone(),
            type_name: self.type_name.clone(),
        })
    }
    
    fn node_type(&self) -> &str {
        "TypePattern"
    }
}
