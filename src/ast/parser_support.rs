/// Additional AST types needed for parser support

use crate::ast::traits::{Node, Statement, Expression};
use crate::ast::block::BlockStatement;
use crate::ast::identifiers::Identifier;
use crate::ast::expressions::Parameter;
use crate::lexer::Token;
use std::any::Any;

// Additional statement types defined inline since the module structure is different

/// Expression statement wrapper
#[derive(Debug)]
pub struct ExpressionStatement {
impl Node for ExpressionStatement {
    fn string(&self) -> String {
        self.expression.string()
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Clone for ExpressionStatement {
    fn clone(&self) -> Self {
        Self {
        }
    }
impl Statement for ExpressionStatement {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

/// Function declaration for parser
#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
/// Type parameter for generics
#[derive(Debug, Clone)]
pub struct TypeParameter {
/// Field definition for structs
#[derive(Debug, Clone)]
pub struct FieldDefinition {
impl FieldDefinition {
    pub fn new(name: String, field_type: String) -> Self {
        Self { name, field_type }
    }
impl Node for FieldDefinition {
    fn string(&self) -> String {
        format!("{} {}", self.to_string(), self.field_type)
    fn token_literal(&self) -> String {
        self.to_string().clone()
    }
}

// All the statement types needed for the parser

/// Variable declaration statement (sus/facts)
#[derive(Debug, Clone)]
pub struct VariableStatement {
impl Node for VariableStatement {
    fn string(&self) -> String {
        let keyword = if self.is_mutable { "sus" } else { "facts" };
        let mut result = format!("{} {}", keyword, self.to_string());
        
        if let Some(ref var_type) = self.var_type {
            result.push_str(&format!(" {}", var_type));
        if let Some(ref value) = self.value {
            result.push_str(&format!(" = {}", value.string()));
        result
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for VariableStatement {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(VariableStatement {
        })
    }
}

/// Additional expression types needed for parser
#[derive(Debug)]
pub struct ArrayLiteral {
impl ArrayLiteral {
    pub fn new(token: String, elements: Vec<Box<dyn Expression>>) -> Self {
        Self { token, elements }
    }
impl Node for ArrayLiteral {
    fn string(&self) -> String {
        let elements: Vec<String> = self.elements.iter()
            .map(|e| e.string())
            .collect();
        format!("[{}]", elements.join(", "))
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Clone for ArrayLiteral {
    fn clone(&self) -> Self {
        Self {
        }
    }
impl Expression for ArrayLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

#[derive(Debug)]
pub struct HashLiteral {
impl HashLiteral {
    pub fn new(token: String, pairs: Vec<(Box<dyn Expression>, Box<dyn Expression>)>) -> Self {
        Self { token, pairs }
    }
impl Node for HashLiteral {
    fn string(&self) -> String {
        let pairs: Vec<String> = self.pairs.iter()
            .map(|(k, v)| format!("{}: {}", k.string(), v.string()))
            .collect();
        format!("{{{}}}", pairs.join(", "))
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Clone for HashLiteral {
    fn clone(&self) -> Self {
        Self {
            pairs: self.pairs.iter()
                .map(|(k, v)| (k.clone_box(), v.clone_box()))
        }
    }
impl Expression for HashLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct NilLiteral {
impl NilLiteral {
    pub fn new(token: String) -> Self {
        Self { token }
    }
impl Node for NilLiteral {
    fn string(&self) -> String {
        "no_cap".to_string()
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for NilLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

#[derive(Debug)]
pub struct IndexExpression {
impl IndexExpression {
    pub fn new(token: String, left: Box<dyn Expression>, index: Box<dyn Expression>) -> Self {
        Self { token, left, index }
    }
impl Node for IndexExpression {
    fn string(&self) -> String {
        format!("{}[{}]", self.left.string(), self.index.string())
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Clone for IndexExpression {
    fn clone(&self) -> Self {
        Self {
        }
    }
impl Expression for IndexExpression {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct DotExpression {
impl DotExpression {
    pub fn new(token: String, left: Box<dyn Expression>, property: String) -> Self {
        Self { token, left, property }
    }
impl Node for DotExpression {
    fn string(&self) -> String {
        format!("{}.{}", self.left.string(), self.property)
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for DotExpression {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(DotExpression {
        })
    }
}
