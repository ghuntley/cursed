/// Function literal expression for the CURSED programming language
/// 
/// Represents anonymous function expressions (lambda functions).

use crate::ast::traits::{Node, Expression};
use crate::ast::expressions::Parameter;
use std::any::Any;

/// Function literal expression (lambda function)
#[derive(Debug, Clone)]
pub struct FunctionLiteral {
impl FunctionLiteral {
    pub fn new(
        return_type: Option<Box<dyn Expression>>
    ) -> Self {
        Self { token, parameters, body, return_type }
    }
impl Node for FunctionLiteral {
    fn string(&self) -> String {
        let params: Vec<String> = self.parameters.iter()
            .map(|p| format!("{}: {}", p.name, p.param_type))
            .collect();
        let return_type_str = self.return_type.as_ref()
            .map(|rt| format!(" -> {}", rt.string()))
            .unwrap_or_default();
        
                self.body.string())
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for FunctionLiteral {
    fn as_any(&self) -> &dyn Any {
        self


    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}
