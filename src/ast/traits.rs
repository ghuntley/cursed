/// Core AST traits for the CURSED programming language
/// 
/// These traits define the fundamental behavior expected from all AST nodes,
/// ensuring consistent interfaces for code generation, analysis, and manipulation.

use std::fmt::{Debug, Display};
use std::any::Any;

/// Base trait that all AST nodes must implement
/// 
/// This trait provides essential functionality for converting AST nodes
/// to string representations and accessing token information.
pub trait Node: Debug + Send + Sync {
    /// Get the string representation of this node
    /// This is used for pretty-printing AST nodes and debugging
    fn string(&self) -> String;
    
    /// Get the literal value of the token that created this node
    /// Used for error reporting and source location tracking
    fn token_literal(&self) -> String;
}

/// Trait for AST nodes that represent expressions
/// 
/// Expressions evaluate to values and can be used in contexts where
/// a value is expected (assignments, function arguments, etc.).
pub trait Expression: Node {
    /// Marker method to identify this as an expression node
    fn expression_node(&self) {}
    
    /// Get a reference to the underlying Any trait for downcasting
    fn as_any(&self) -> &dyn Any;
    
    /// Clone this expression into a boxed trait object
    fn clone_box(&self) -> Box<dyn Expression>;
}

/// Trait for AST nodes that represent statements
/// 
/// Statements perform actions and do not return values.
/// They form the building blocks of function bodies and program structure.
pub trait Statement: Node {
    /// Marker method to identify this as a statement node
    fn statement_node(&self) {}
    
    /// Get a reference to the underlying Any trait for downcasting
    fn as_any(&self) -> &dyn Any;
    
    /// Clone this statement into a boxed trait object
    fn clone_box(&self) -> Box<dyn Statement>;
}

/// Extension trait for statements to provide additional functionality
pub trait StatementExtensions: Statement {
    /// Check if this statement is a declaration
    fn is_declaration(&self) -> bool {
        false
    }
    
    /// Check if this statement is a control flow statement
    fn is_control_flow(&self) -> bool {
        false
    }
    
    /// Check if this statement can be executed in a constant context
    fn is_const_evaluatable(&self) -> bool {
        false
    }
}

/// Trait for AST nodes that represent type information
pub trait TypeNode: Node {
    /// Get the name of this type
    fn type_name(&self) -> String;
    
    /// Check if this type is generic (has type parameters)
    fn is_generic(&self) -> bool {
        false
    }
    
    /// Get the size of this type in bytes (if known at compile time)
    fn size_hint(&self) -> Option<usize> {
        None
    }
}

/// Trait for AST nodes that can have generic type parameters
pub trait GenericNode: Node {
    /// Get the type parameters for this node
    fn type_parameters(&self) -> &[TypeParameter];
    
    /// Check if this node has type parameters
    fn is_generic(&self) -> bool {
        !self.type_parameters().is_empty()
    }
}

/// Forward declaration for TypeParameter (defined in types module)
#[derive(Debug, Clone)]
pub struct TypeParameter {
    pub name: String,
    pub constraints: Vec<String>,
}

impl TypeParameter {
    pub fn new(name: String) -> Self {
        Self {
            name,
            constraints: Vec::new(),
        }
    }
    
    pub fn with_constraints(name: String, constraints: Vec<String>) -> Self {
        Self {
            name,
            constraints,
        }
    }
}

/// Trait for nodes that support visitor pattern
pub trait Visitable {
    fn accept<V: Visitor>(&self, visitor: &mut V);
}

/// Visitor trait for traversing AST nodes
pub trait Visitor {
    fn visit_program(&mut self, node: &crate::ast::Program);
    fn visit_expression(&mut self, node: &dyn Expression);
    fn visit_statement(&mut self, node: &dyn Statement);
}

/// Trait for AST nodes that can be mutated
pub trait Mutable {
    fn accept_mut<V: MutVisitor>(&mut self, visitor: &mut V);
}

/// Mutable visitor trait for modifying AST nodes
pub trait MutVisitor {
    fn visit_program_mut(&mut self, node: &mut crate::ast::Program);
    fn visit_expression_mut(&mut self, node: &mut dyn Expression);
    fn visit_statement_mut(&mut self, node: &mut dyn Statement);
}

/// Trait for nodes that can provide source location information
pub trait Locatable {
    fn source_location(&self) -> Option<&crate::error::SourceLocation>;
    fn set_source_location(&mut self, location: crate::error::SourceLocation);
}
