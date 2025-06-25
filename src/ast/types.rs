/// Type system AST nodes for the CURSED programming language

use crate::ast::traits::{Node, Expression, TypeNode};
use crate::ast::identifiers::Identifier;
use std::any::Any;

/// Built-in types and type expressions
#[derive(Debug, Clone)]
pub struct TypeExpression {
impl TypeExpression {
    pub fn new(token: String, name: String) -> Self {
        Self {
        }
    }
    
    pub fn with_args(token: String, name: String, type_args: Vec<Box<dyn Expression>>) -> Self {
        Self {
        }
    }
impl Node for TypeExpression {
    fn string(&self) -> String {
        if self.type_args.is_empty() {
            self.to_string().clone()
        } else {
            let args: Vec<String> = self.type_args.iter()
                .map(|arg| arg.string())
                .collect();
            format!("{}<{}>", self.to_string(), args.join(", "))
        }
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for TypeExpression {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(TypeExpression {
        })
    }
}

impl TypeNode for TypeExpression {
    fn type_name(&self) -> String {
        self.to_string().clone()
    fn is_generic(&self) -> bool {
        !self.type_args.is_empty()
    }
}

/// Map type expression (tea[KeyType]ValueType)
#[derive(Debug, Clone)]
pub struct MapTypeExpression {
impl MapTypeExpression {
    pub fn new(
    ) -> Self {
        Self {
        }
    }
    
    pub fn get_key_type(&self) -> &Box<dyn Expression> {
        &self.key_type
    pub fn get_value_type(&self) -> &Box<dyn Expression> {
        &self.value_type
    }
}

impl Node for MapTypeExpression {
    fn string(&self) -> String {
        format!("tea[{}]{}", self.key_type.string(), self.value_type.string())
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for MapTypeExpression {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(MapTypeExpression {
        })
    }
}

/// Array type expression ([Type])
#[derive(Debug, Clone)]
pub struct ArrayTypeExpression {
impl ArrayTypeExpression {
    pub fn new(token: String, element_type: Box<dyn Expression>) -> Self {
        Self {
        }
    }
    
    pub fn with_size(
    ) -> Self {
        Self {
        }
    }
impl Node for ArrayTypeExpression {
    fn string(&self) -> String {
        if let Some(size) = &self.size {
            format!("[{}]{}", size.string(), self.element_type.string())
        } else {
            format!("[]{}", self.element_type.string())
        }
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for ArrayTypeExpression {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(ArrayTypeExpression {
        })
    }
}

/// Channel type expression (dm Type)
#[derive(Debug, Clone)]
pub struct ChannelTypeExpression {
#[derive(Debug, Clone)]
pub enum ChannelDirection {
impl ChannelTypeExpression {
    pub fn new(token: String, element_type: Box<dyn Expression>) -> Self {
        Self {
        }
    }
impl Node for ChannelTypeExpression {
    fn string(&self) -> String {
        match self.direction {
        }
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for ChannelTypeExpression {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(ChannelTypeExpression {
        })
    }
}

/// Interface type for type assertions
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InterfaceType {
impl InterfaceType {
    pub fn new(name: String) -> Self {
        Self {
        }
    }
/// Struct type for type assertions
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructType {
impl StructType {
    pub fn new(name: String) -> Self {
        Self {
        }
    }
/// Generic type enum (simplified for basic AST compatibility)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    /// Integer type (normie)
    /// String type (tea)
    /// Boolean type (based)
    /// Float type (facts)
    /// Character type 
    /// Nil type (empty)
    /// Any type (universal)
    /// Array type [T]
    /// Map type tea[K]V
    /// Channel type dm T
    /// Function type (params) -> return_type
    /// Tuple type (T1, T2, ...)
    /// Associated type projection (e.g., Iterator::Item)
    AssociatedTypeProjection {
    /// Type parameter
    /// Higher-kinded type constructor
    Constructor {
    /// Type application (constructor applied to arguments)
    Application {
/// Built-in type aliases for normie integers
pub type NormieType = TypeExpression;

/// Common built-in types
pub fn normie_type() -> TypeExpression {
    TypeExpression::new("normie".to_string(), "normie".to_string())
    pub fn tea_type() -> TypeExpression {
    TypeExpression::new("tea".to_string(), "tea".to_string())
    pub fn based_type() -> TypeExpression {
    TypeExpression::new("based".to_string(), "based".to_string())
}
