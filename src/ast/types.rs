/// Type system AST nodes for the CURSED programming language

use crate::ast::traits::{Node, Expression, TypeNode};
use crate::ast::identifiers::Identifier;
use std::any::Any;

/// Built-in types and type expressions
#[derive(Debug, Clone)]
pub struct TypeExpression {
    pub token: String,
    pub name: String,
    pub type_args: Vec<Box<dyn Expression>>,
}

impl TypeExpression {
    pub fn new(token: String, name: String) -> Self {
        Self {
            token,
            name,
            type_args: Vec::new(),
        }
    }
    
    pub fn with_args(token: String, name: String, type_args: Vec<Box<dyn Expression>>) -> Self {
        Self {
            token,
            name,
            type_args,
        }
    }
}

impl Node for TypeExpression {
    fn string(&self) -> String {
        if self.type_args.is_empty() {
            self.name.clone()
        } else {
            let args: Vec<String> = self.type_args.iter()
                .map(|arg| arg.string())
                .collect();
            format!("{}<{}>", self.name, args.join(", "))
        }
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for TypeExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(TypeExpression {
            token: self.token.clone(),
            name: self.name.clone(),
            type_args: self.type_args.iter().map(|arg| arg.clone_box()).collect(),
        })
    }
}

impl TypeNode for TypeExpression {
    fn type_name(&self) -> String {
        self.name.clone()
    }
    
    fn is_generic(&self) -> bool {
        !self.type_args.is_empty()
    }
}

/// Map type expression (tea[KeyType]ValueType)
#[derive(Debug, Clone)]
pub struct MapTypeExpression {
    pub token: String,
    pub key_type: Box<dyn Expression>,
    pub value_type: Box<dyn Expression>,
}

impl MapTypeExpression {
    pub fn new(
        token: crate::lexer::Token,
        key_type: Box<dyn Expression>,
        value_type: Box<dyn Expression>,
    ) -> Self {
        Self {
            token: token.literal,
            key_type,
            value_type,
        }
    }
    
    pub fn get_key_type(&self) -> &Box<dyn Expression> {
        &self.key_type
    }
    
    pub fn get_value_type(&self) -> &Box<dyn Expression> {
        &self.value_type
    }
}

impl Node for MapTypeExpression {
    fn string(&self) -> String {
        format!("tea[{}]{}", self.key_type.string(), self.value_type.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for MapTypeExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(MapTypeExpression {
            token: self.token.clone(),
            key_type: self.key_type.clone_box(),
            value_type: self.value_type.clone_box(),
        })
    }
}

/// Array type expression ([Type])
#[derive(Debug, Clone)]
pub struct ArrayTypeExpression {
    pub token: String,
    pub element_type: Box<dyn Expression>,
    pub size: Option<Box<dyn Expression>>,
}

impl ArrayTypeExpression {
    pub fn new(token: String, element_type: Box<dyn Expression>) -> Self {
        Self {
            token,
            element_type,
            size: None,
        }
    }
    
    pub fn with_size(
        token: String,
        element_type: Box<dyn Expression>,
        size: Box<dyn Expression>,
    ) -> Self {
        Self {
            token,
            element_type,
            size: Some(size),
        }
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
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(ArrayTypeExpression {
            token: self.token.clone(),
            element_type: self.element_type.clone_box(),
            size: self.size.as_ref().map(|s| s.clone_box()),
        })
    }
}

/// Channel type expression (dm Type)
#[derive(Debug, Clone)]
pub struct ChannelTypeExpression {
    pub token: String,
    pub element_type: Box<dyn Expression>,
    pub direction: ChannelDirection,
}

#[derive(Debug, Clone)]
pub enum ChannelDirection {
    Bidirectional,
    SendOnly,
    ReceiveOnly,
}

impl ChannelTypeExpression {
    pub fn new(token: String, element_type: Box<dyn Expression>) -> Self {
        Self {
            token,
            element_type,
            direction: ChannelDirection::Bidirectional,
        }
    }
}

impl Node for ChannelTypeExpression {
    fn string(&self) -> String {
        match self.direction {
            ChannelDirection::Bidirectional => format!("dm {}", self.element_type.string()),
            ChannelDirection::SendOnly => format!("dm<- {}", self.element_type.string()),
            ChannelDirection::ReceiveOnly => format!("<-dm {}", self.element_type.string()),
        }
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for ChannelTypeExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(ChannelTypeExpression {
            token: self.token.clone(),
            element_type: self.element_type.clone_box(),
            direction: self.direction.clone(),
        })
    }
}

/// Interface type for type assertions
#[derive(Debug, Clone)]
pub struct InterfaceType {
    pub name: String,
    pub methods: Vec<String>,
}

impl InterfaceType {
    pub fn new(name: String) -> Self {
        Self {
            name,
            methods: Vec::new(),
        }
    }
}

/// Struct type for type assertions
#[derive(Debug, Clone)]
pub struct StructType {
    pub name: String,
    pub fields: Vec<String>,
}

impl StructType {
    pub fn new(name: String) -> Self {
        Self {
            name,
            fields: Vec::new(),
        }
    }
}

/// Generic type enum
#[derive(Debug, Clone)]
pub enum Type {
    Interface(InterfaceType),
    Struct(StructType),
    Primitive(String),
    Generic(String),
}

/// Built-in type aliases for normie integers
pub type NormieType = TypeExpression;

/// Common built-in types
pub fn normie_type() -> TypeExpression {
    TypeExpression::new("normie".to_string(), "normie".to_string())
}

pub fn tea_type() -> TypeExpression {
    TypeExpression::new("tea".to_string(), "tea".to_string())
}

pub fn based_type() -> TypeExpression {
    TypeExpression::new("based".to_string(), "based".to_string())
}
