/// Variable declaration statements for CURSED

use crate::ast::traits::{Node, Statement, Expression};
use crate::lexer::Token;
use std::any::Any;

/// Variable declaration statement (sus/facts)
#[derive(Debug, Clone)]
pub struct VariableStatement {
impl Node for VariableStatement {
    fn string(&self) -> String {
        let keyword = if self.is_mutable { "sus" } else { "facts" };
        let mut result = format!("{} {}", keyword, self.name);
        
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
