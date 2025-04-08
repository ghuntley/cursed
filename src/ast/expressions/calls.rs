use std::any::Any;
use crate::ast::{Node, Expression};
use crate::lexer::token::Token;

/// CallExpression represents a call expression
pub struct CallExpression {
    pub token: Token,
    pub function: Box<dyn Expression>,
    pub arguments: Vec<Box<dyn Expression>>,
}

impl Node for CallExpression {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }

    fn string(&self) -> String {
        let args: Vec<String> = self.arguments.iter()
            .map(|arg| arg.string())
            .collect();
        format!("{} {} {}", self.function.string(), self.token_literal(), args.join(", "))
    }
}

impl Expression for CallExpression {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_call_expression(&self) -> bool {
        true
    }

    fn as_call_expression(&self) -> Option<(&dyn Expression, Vec<&dyn Expression>)> {
        let args: Vec<&dyn Expression> = self.arguments.iter()
            .map(|arg| arg.as_ref() as &dyn Expression)
            .collect();
        Some((self.function.as_ref(), args))
    }
}

/// GenericCallExpression represents a call expression with generic type arguments
pub struct GenericCallExpression {
    pub token: Token,
    pub function: Box<dyn Expression>,
    pub type_arguments: Vec<Box<dyn Expression>>,
    pub arguments: Vec<Box<dyn Expression>>,
}

impl Node for GenericCallExpression {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }

    fn string(&self) -> String {
        let type_args: Vec<String> = self.type_arguments.iter()
            .map(|arg| arg.string())
            .collect();
        let args: Vec<String> = self.arguments.iter()
            .map(|arg| arg.string())
            .collect();
        format!("{} {} [{}] {}", 
                self.function.string(), 
                self.token_literal(),
                type_args.join(", "),
                args.join(", "))
    }
}

impl Expression for GenericCallExpression {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_call_expression(&self) -> bool {
        true
    }

    fn as_call_expression(&self) -> Option<(&dyn Expression, Vec<&dyn Expression>)> {
        let args: Vec<&dyn Expression> = self.arguments.iter()
            .map(|arg| arg.as_ref() as &dyn Expression)
            .collect();
        Some((self.function.as_ref(), args))
    }
}