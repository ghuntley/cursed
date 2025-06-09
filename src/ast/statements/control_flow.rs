/// Control flow statements for CURSED

use crate::ast::traits::{Node, Statement, Expression};
use crate::ast::block::BlockStatement;
use crate::lexer::Token;
use std::any::Any;

/// If statement (lowkey/highkey)
#[derive(Debug, Clone)]
pub struct IfStatement {
    pub token: String,
    pub condition: Box<dyn Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<Box<dyn Statement>>,
}

impl Node for IfStatement {
    fn string(&self) -> String {
        let mut result = format!("lowkey {} {}", 
            self.condition.string(), 
            self.consequence.string()
        );
        
        if let Some(ref alt) = self.alternative {
            result.push_str(&format!(" highkey {}", alt.string()));
        }
        
        result
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for IfStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(IfStatement {
            token: self.token.clone(),
            condition: self.condition.clone_box(),
            consequence: self.consequence.clone(),
            alternative: self.alternative.as_ref().map(|a| a.clone_box()),
        })
    }
}

/// Switch statement (vibe_check)
#[derive(Debug, Clone)]
pub struct SwitchStatement {
    pub token: String,
    pub value: Box<dyn Expression>,
    pub cases: Vec<SwitchCase>,
    pub default: Option<Vec<Box<dyn Statement>>>,
}

#[derive(Debug, Clone)]
pub struct SwitchCase {
    pub values: Vec<Box<dyn Expression>>,
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for SwitchStatement {
    fn string(&self) -> String {
        let mut result = format!("vibe_check {} {{\n", self.value.string());
        
        for case in &self.cases {
            let values: Vec<String> = case.values.iter()
                .map(|v| v.string())
                .collect();
            result.push_str(&format!("  mood {}:\n", values.join(", ")));
            
            for stmt in &case.statements {
                result.push_str(&format!("    {}\n", stmt.string()));
            }
        }
        
        if let Some(ref default) = self.default {
            result.push_str("  basic:\n");
            for stmt in default {
                result.push_str(&format!("    {}\n", stmt.string()));
            }
        }
        
        result.push('}');
        result
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for SwitchStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(SwitchStatement {
            token: self.token.clone(),
            value: self.value.clone_box(),
            cases: self.cases.iter().map(|c| SwitchCase {
                values: c.values.iter().map(|v| v.clone_box()).collect(),
                statements: c.statements.iter().map(|s| s.clone_box()).collect(),
            }).collect(),
            default: self.default.as_ref().map(|d| d.iter().map(|s| s.clone_box()).collect()),
        })
    }
}

/// For statement (bestie)
#[derive(Debug, Clone)]
pub struct ForStatement {
    pub token: String,
    pub init: Option<Box<dyn Statement>>,
    pub condition: Option<Box<dyn Expression>>,
    pub post: Option<Box<dyn Statement>>,
    pub body: BlockStatement,
}

impl Node for ForStatement {
    fn string(&self) -> String {
        let init_str = self.init.as_ref().map(|i| i.string()).unwrap_or_default();
        let cond_str = self.condition.as_ref().map(|c| c.string()).unwrap_or_default();
        let post_str = self.post.as_ref().map(|p| p.string()).unwrap_or_default();
        
        format!("bestie {}; {}; {} {}", init_str, cond_str, post_str, self.body.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for ForStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(ForStatement {
            token: self.token.clone(),
            init: self.init.as_ref().map(|i| i.clone_box()),
            condition: self.condition.as_ref().map(|c| c.clone_box()),
            post: self.post.as_ref().map(|p| p.clone_box()),
            body: self.body.clone(),
        })
    }
}

/// Range-based for statement (bestie x := flex items)
#[derive(Debug, Clone)]
pub struct RangeForStatement {
    pub token: String,
    pub key_var: Option<String>,
    pub value_var: Option<String>,
    pub iterable: Box<dyn Expression>,
    pub body: BlockStatement,
}

impl Node for RangeForStatement {
    fn string(&self) -> String {
        let vars = match (&self.key_var, &self.value_var) {
            (Some(k), Some(v)) => format!("{}, {}", k, v),
            (Some(k), None) => k.clone(),
            (None, Some(v)) => v.clone(),
            (None, None) => "_".to_string(),
        };
        
        format!("bestie {} := flex {} {}", vars, self.iterable.string(), self.body.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for RangeForStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(RangeForStatement {
            token: self.token.clone(),
            key_var: self.key_var.clone(),
            value_var: self.value_var.clone(),
            iterable: self.iterable.clone_box(),
            body: self.body.clone(),
        })
    }
}

/// While statement (periodt)
#[derive(Debug, Clone)]
pub struct WhileStatement {
    pub token: String,
    pub condition: Box<dyn Expression>,
    pub body: BlockStatement,
}

impl Node for WhileStatement {
    fn string(&self) -> String {
        format!("periodt {} {}", self.condition.string(), self.body.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for WhileStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(WhileStatement {
            token: self.token.clone(),
            condition: self.condition.clone_box(),
            body: self.body.clone(),
        })
    }
}

/// Break statement (ghosted)
#[derive(Debug, Clone)]
pub struct BreakStatement {
    pub token: String,
}

impl Node for BreakStatement {
    fn string(&self) -> String {
        "ghosted".to_string()
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for BreakStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

/// Continue statement (simp)
#[derive(Debug, Clone)]
pub struct ContinueStatement {
    pub token: String,
}

impl Node for ContinueStatement {
    fn string(&self) -> String {
        "simp".to_string()
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for ContinueStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

/// Type alias statement (be_like)
#[derive(Debug, Clone)]
pub struct TypeAliasStatement {
    pub token: String,
    pub name: String,
    pub target_type: String,
}

impl Node for TypeAliasStatement {
    fn string(&self) -> String {
        format!("be_like {} {}", self.name, self.target_type)
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for TypeAliasStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}
