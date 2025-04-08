use std::any::Any;
use crate::ast::{Node, Expression};
use crate::lexer::token::Token;
use super::identifiers::Identifier;

/// TypeConversionExpression represents a type conversion expression
pub struct TypeConversionExpression {
    pub token: Token,
    pub expression: Box<dyn Expression>,
    pub type_name: String,
}

impl Node for TypeConversionExpression {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }
    
    fn string(&self) -> String {
        format!("{} as {}", self.expression.string(), self.type_name)
    }
}

impl Expression for TypeConversionExpression {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
}

/// AssignmentExpression represents an assignment expression (e.g., x = 5)
pub struct AssignmentExpression {
    pub token: String, // Token::Assign
    pub name: Identifier,
    pub value: Box<dyn Expression>,
}

impl Node for AssignmentExpression {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        format!("{} {} {}", self.name.string(), self.token_literal(), self.value.string())
    }
}

impl Expression for AssignmentExpression {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// BeLikeExpression represents a struct instantiation expression
pub struct BeLikeExpression {
    pub token: String,
    pub struct_name: Identifier,
    pub type_arguments: Vec<Box<dyn Expression>>, // Generic type arguments [normie], [tea, normie], etc.
    pub fields: Vec<(String, Box<dyn Expression>)>,
}

impl Node for BeLikeExpression {
    fn token_literal(&self) -> String {
        self.token.clone()
    }
    
    fn string(&self) -> String {
        let mut out = format!("be_like {}", self.struct_name.string());
        
        // Format type arguments if present
        if !self.type_arguments.is_empty() {
            let type_args: Vec<String> = self.type_arguments.iter()
                .map(|arg| arg.string())
                .collect();
            out.push_str(&format!("[{}]", type_args.join(", ")));
        }
        
        if !self.fields.is_empty() {
            out.push_str(" {");
            let fields_str: Vec<String> = self.fields.iter()
                .map(|(name, expr)| format!("{}: {}", name, expr.string()))
                .collect();
            out.push_str(&fields_str.join(", "));
            out.push_str("}");
        }
        
        out
    }
}

impl Expression for BeLikeExpression {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}