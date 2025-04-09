use crate::ast::{Node, Statement, Expression};

/// Declaration-assignment statement (tea x := 5)
#[derive(Debug)]
pub struct DeclAssignStatement {
    pub token: String,
    pub name: crate::ast::Identifier,
    pub type_name: String,
    pub value: Option<Box<dyn Expression>>,
}

impl Node for DeclAssignStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Statement for DeclAssignStatement {
    fn statement_node(&self) {}
    
    fn as_node(&self) -> &dyn Node {
        self
    }
}