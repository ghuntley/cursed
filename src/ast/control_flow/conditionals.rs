use std::any::Any;
use crate::ast::{Node, Statement, Expression};
use crate::ast::statements::block::BlockStatement;

/// IfStatement represents an if statement
pub struct IfStatement {
    pub token: String, // Token::If
    pub condition: Box<dyn Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl Node for IfStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        let mut out = format!("if {} {}", self.condition.string(), self.consequence.string());
        if let Some(alt) = &self.alternative {
            out.push_str(&format!(" else {}", alt.string()));
        }
        out
    }
}

impl Statement for IfStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// SwitchStatement represents a switch statement (vibe_check in CURSED)
pub struct SwitchStatement {
    pub token: String, // Token::VibeCheck
    pub value: Box<dyn Expression>,
    pub cases: Vec<CaseStatement>,
    pub default: Option<BlockStatement>,
}

impl Node for SwitchStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        let mut out = format!("vibe_check {} {{\n", self.value.string());
        
        for case in &self.cases {
            out.push_str(&format!("    {}\n", case.string()));
        }
        
        if let Some(default) = &self.default {
            out.push_str(&format!("    basic: {}\n", default.string()));
        }
        
        out.push_str("}");
        out
    }
}

impl Statement for SwitchStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// CaseStatement represents a case clause in a switch statement
pub struct CaseStatement {
    pub token: String, // Token::Mood
    pub expressions: Vec<Box<dyn Expression>>,
    pub body: BlockStatement,
}

impl Node for CaseStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        let exprs: Vec<String> = self.expressions.iter()
            .map(|expr| expr.string())
            .collect();
        format!("mood {}: {}", exprs.join(", "), self.body.string())
    }
}

impl Statement for CaseStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}