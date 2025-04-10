use std::any::Any;
use crate::ast::{Node, Statement, Expression};
use crate::ast::statements::block::BlockStatement;

/// WhileStatement represents a while loop (periodt in CURSED)
pub struct WhileStatement {
    pub token: String, // Token::Periodt
    pub condition: Box<dyn Expression>,
    pub body: Box<BlockStatement>,
}

impl Node for WhileStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        format!("periodt {} {}", self.condition.string(), self.body.string())
    }
}

impl Statement for WhileStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// ForStatement represents a for loop (bestie in CURSED)
/// A for loop can have three different forms:
/// 1. C-style: bestie init; condition; post { body }
/// 2. Condition-only: bestie condition { body }
/// 3. Infinite loop: bestie { body }
pub struct ForStatement {
    pub token: String, // Token::Bestie
    pub init: Option<Box<dyn Statement>>,
    pub condition: Option<Box<dyn Expression>>,
    pub post: Option<Box<dyn Statement>>,
    pub body: Box<BlockStatement>,
}

impl Node for ForStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("bestie ");
        
        if let Some(init) = &self.init {
            out.push_str(&init.string());
            out.push_str("; ");
        }
        
        if let Some(cond) = &self.condition {
            out.push_str(&cond.string());
        }
        
        if let Some(post) = &self.post {
            out.push_str("; ");
            out.push_str(&post.string());
        }
        
        out.push_str(" ");
        out.push_str(&self.body.string());
        
        out
    }
}

impl Statement for ForStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// BreakStatement represents a break statement
pub struct BreakStatement {
    pub token: String, // Token::Break
}

impl Node for BreakStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        format!("{} {}", self.token_literal(), ";")
    }
}

impl Statement for BreakStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// ContinueStatement represents a continue statement
pub struct ContinueStatement {
    pub token: String, // Token::Continue
}

impl Node for ContinueStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        format!("{} {}", self.token_literal(), ";")
    }
}

impl Statement for ContinueStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}