/// Control flow statements for CURSED

use crate::ast::traits::{Node, Statement, Expression};
use crate::ast::block::BlockStatement;
use crate::error::SourceLocation;
use crate::lexer::Token;
use std::any::Any;

/// If statement (lowkey/highkey)
#[derive(Debug, Clone)]
pub struct IfStatement {
impl Node for IfStatement {
    fn string(&self) -> String {
            self.consequence.string()
        );
        
        if let Some(ref alt) = self.alternative {
            result.push_str(&format!(" highkey {}", alt.string()));
        result
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for IfStatement {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(IfStatement {
        })
    }
}

/// Switch statement (vibe_check)
#[derive(Debug, Clone)]
pub struct SwitchStatement {
#[derive(Debug, Clone)]
pub struct SwitchCase {
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
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for SwitchStatement {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(SwitchStatement {
            cases: self.cases.iter().map(|c| SwitchCase {
        })
    }
}

/// For statement (bestie)
#[derive(Debug, Clone)]
pub struct ForStatement {
impl Node for ForStatement {
    fn string(&self) -> String {
        let init_str = self.init.as_ref().map(|i| i.string()).unwrap_or_default();
        let cond_str = self.condition.as_ref().map(|c| c.string()).unwrap_or_default();
        let post_str = self.post.as_ref().map(|p| p.string()).unwrap_or_default();
        
        format!("bestie {}; {}; {} {}", init_str, cond_str, post_str, self.body.string())
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for ForStatement {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(ForStatement {
        })
    }
}

/// Range-based for statement (bestie x := flex items)
#[derive(Debug, Clone)]
pub struct RangeForStatement {
impl Node for RangeForStatement {
    fn string(&self) -> String {
        let vars = match (&self.key_var, &self.value_var) {
        
        format!("bestie {} := flex {} {}", vars, self.iterable.string(), self.body.string())
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for RangeForStatement {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(RangeForStatement {
        })
    }
}

/// While statement (periodt)
#[derive(Debug, Clone)]
pub struct WhileStatement {
impl Node for WhileStatement {
    fn string(&self) -> String {
        format!("periodt {} {}", self.condition.string(), self.body.string())
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for WhileStatement {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(WhileStatement {
        })
    }
}

/// Break statement (ghosted)
#[derive(Debug, Clone)]
pub struct BreakStatement {
impl Node for BreakStatement {
    fn string(&self) -> String {
        "ghosted".to_string()
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for BreakStatement {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

/// Continue statement (simp)
#[derive(Debug, Clone)]
pub struct ContinueStatement {
impl Node for ContinueStatement {
    fn string(&self) -> String {
        "simp".to_string()
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for ContinueStatement {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

/// Type alias statement (be_like)
#[derive(Debug, Clone)]
pub struct TypeAliasStatement {
impl Node for TypeAliasStatement {
    fn string(&self) -> String {
        format!("be_like {} {}", self.name, self.target_type)
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for TypeAliasStatement {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

/// Enum statement (choices)
#[derive(Debug, Clone)]
pub struct EnumStatement {
impl Node for EnumStatement {
    fn string(&self) -> String {
        format!("choices {} {{ {} }}", self.name, self.variants.join(", "))
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for EnumStatement {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

/// Constant statement (vibes name = value)
#[derive(Debug, Clone)]
pub struct ConstantStatement {
impl Node for ConstantStatement {
    fn string(&self) -> String {
        let mut result = format!("vibes {} = {}", self.name, self.value.string());
        if let Some(ref var_type) = self.var_type {
            result = format!("vibes {} {} = {}", self.name, var_type, self.value.string());
        }
        result
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for ConstantStatement {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(ConstantStatement {
        })
    }
}

/// Module statement (module name { ... })
#[derive(Debug, Clone)]
pub struct ModuleStatement {
impl Node for ModuleStatement {
    fn string(&self) -> String {
        let body_str: Vec<String> = self.body.iter().map(|s| s.string()).collect();
        format!("module {} {{ {} }}", self.name, body_str.join("; "))
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for ModuleStatement {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(ModuleStatement {
        })
    }
}

/// For-in statement 
#[derive(Debug, Clone)]
pub struct ForInStatement {
impl Node for ForInStatement {
    fn string(&self) -> String {
                self.body.string())
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for ForInStatement {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(ForInStatement {
        })
    }
}

/// Do-while statement
#[derive(Debug, Clone)]
pub struct DoWhileStatement {
impl Node for DoWhileStatement {
    fn string(&self) -> String {
        format!("do {{ {} }} lowkey {}", self.body.string(), self.condition.string())
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for DoWhileStatement {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(DoWhileStatement {
        })
    }
}

/// Try statement
#[derive(Debug, Clone)]
pub struct TryStatement {
impl Node for TryStatement {
    fn string(&self) -> String {
        let mut result = format!("try {{ {} }}", self.try_block.string());
        if let Some(ref catch_block) = self.catch_block {
            result.push_str(&format!(" catch {{ {} }}", catch_block.string()));
        }
        if let Some(ref finally_block) = self.finally_block {
            result.push_str(&format!(" finally {{ {} }}", finally_block.string()));
        }
        result
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for TryStatement {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(TryStatement {
        })
    }
}

/// Return statement
#[derive(Debug, Clone)]
pub struct ReturnStatement {
impl Node for ReturnStatement {
    fn string(&self) -> String {
        if let Some(ref value) = self.value {
            format!("return {}", value.string())
        } else {
            "return".to_string()
        }
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for ReturnStatement {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(ReturnStatement {
        })
    }
}

// Re-export type switch from the main type_switch module
pub use crate::ast::type_switch::*;
