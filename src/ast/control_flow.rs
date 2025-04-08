use std::any::Any;
use crate::ast::{Node, Statement, Expression};

/// LaterStatement represents a scheduled execution (defer, later, etc.)
pub struct LaterStatement {
    pub token: String,
    pub body: Box<dyn Statement>,
}

impl Node for LaterStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }
    
    fn string(&self) -> String {
        format!("later {}", self.body.string())
    }
}

impl Statement for LaterStatement {
    fn statement_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
}

use super::statements::BlockStatement;

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

/// WhileStatement represents a while loop (periodt in CURSED)
pub struct WhileStatement {
    pub token: String, // Token::Periodt
    pub condition: Box<dyn Expression>,
    pub body: BlockStatement,
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
    pub body: BlockStatement,
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