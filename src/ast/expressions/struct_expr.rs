//! AST nodes for struct literals and property access in the CURSED language

use std::any::Any;
use std::fmt::{self, Display, Formatter};

use crate::ast::traits::{Expression, Node};
use crate::lexer::token::Token;

/// Represents a key-value pair in a struct literal
pub struct KeyValuePair {
    /// The field name token
    pub key: super::identifiers::Identifier,
    /// The field value expression
    pub value: Box<dyn Expression>,
}

impl Clone for KeyValuePair {
    fn clone(&self) -> Self {
        // We can't clone a Box<dyn Expression> directly, so we need to create a new one
        // This requires knowledge of concrete types, which we don't have here
        // For now, we'll implement a placeholder that will cause a panic if used
        unimplemented!("KeyValuePair cannot be cloned due to Box<dyn Expression>")
    }
}

impl Display for KeyValuePair {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: <expr>", self.key.value)
    }
}

/// Represents a struct literal expression
/// Example: `Point{x: 10, y: 20.5}`
pub struct StructLiteral {
    /// The opening brace token
    pub token: Token,
    /// The name of the struct type
    pub struct_name: String,
    /// The field initializers
    pub fields: Vec<KeyValuePair>,
}

impl Clone for StructLiteral {
    fn clone(&self) -> Self {
        // Since KeyValuePair doesn't support Clone, we can't implement this properly
        // For now, we'll implement a placeholder that will cause a panic if used
        unimplemented!("StructLiteral cannot be cloned due to Vec<KeyValuePair>")
    }
}

impl Node for StructLiteral {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }
    
    fn string(&self) -> String {
        format!("{}", self)
    }
}

impl Expression for StructLiteral {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn node_type(&self) -> &str {
        "StructLiteral"
    }
}

impl Display for StructLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{{ ", self.struct_name)?;
        
        let mut first = true;
        for field in &self.fields {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}", field)?;
            first = false;
        }
        
        write!(f, " }}")
    }
}

/// Represents a struct field access
/// Example: `point.x`
pub struct StructFieldAccess {
    /// The dot token
    pub token: Token,
    /// The object expression
    pub object: Box<dyn Expression>,
    /// The field name
    pub field: super::identifiers::Identifier,
}

impl Clone for StructFieldAccess {
    fn clone(&self) -> Self {
        // We can't clone a Box<dyn Expression> directly
        // For now, we'll implement a placeholder that will cause a panic if used
        unimplemented!("StructFieldAccess cannot be cloned due to Box<dyn Expression>")
    }
}

impl Node for StructFieldAccess {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }
    
    fn string(&self) -> String {
        format!("{}", self)
    }
}

impl Expression for StructFieldAccess {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn node_type(&self) -> &str {
        "StructFieldAccess"
    }
}

impl Display for StructFieldAccess {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<expr>.{}", self.field.value)
    }
}