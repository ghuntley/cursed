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

/// Enum statement (choices)
#[derive(Debug, Clone)]
pub struct EnumStatement {
    pub token: String,
    pub name: String,
    pub variants: Vec<String>,
}

impl Node for EnumStatement {
    fn string(&self) -> String {
        format!("choices {} {{ {} }}", self.name, self.variants.join(", "))
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for EnumStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

/// Constant statement (vibes name = value)
#[derive(Debug, Clone)]
pub struct ConstantStatement {
    pub token: String,
    pub name: String,
    pub value: Box<dyn Expression>,
    pub var_type: Option<String>,
}

impl Node for ConstantStatement {
    fn string(&self) -> String {
        let mut result = format!("vibes {} = {}", self.name, self.value.string());
        if let Some(ref var_type) = self.var_type {
            result = format!("vibes {} {} = {}", self.name, var_type, self.value.string());
        }
        result
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for ConstantStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(ConstantStatement {
            token: self.token.clone(),
            name: self.name.clone(),
            value: self.value.clone_box(),
            var_type: self.var_type.clone(),
        })
    }
}

/// Module statement (module name { ... })
#[derive(Debug, Clone)]
pub struct ModuleStatement {
    pub token: String,
    pub name: String,
    pub body: Vec<Box<dyn Statement>>,
}

impl Node for ModuleStatement {
    fn string(&self) -> String {
        let body_str: Vec<String> = self.body.iter().map(|s| s.string()).collect();
        format!("module {} {{ {} }}", self.name, body_str.join("; "))
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for ModuleStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(ModuleStatement {
            token: self.token.clone(),
            name: self.name.clone(),
            body: self.body.iter().map(|s| s.clone_box()).collect(),
        })
    }
}

/// For-in statement 
#[derive(Debug, Clone)]
pub struct ForInStatement {
    pub token: String,
    pub identifier: String,
    pub iterable: Box<dyn Expression>,
    pub body: Box<dyn Statement>,
}

impl Node for ForInStatement {
    fn string(&self) -> String {
        format!("bestie {} periodt {} {{ {} }}", 
                self.identifier, 
                self.iterable.string(), 
                self.body.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for ForInStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(ForInStatement {
            token: self.token.clone(),
            identifier: self.identifier.clone(),
            iterable: self.iterable.clone_box(),
            body: self.body.clone_box(),
        })
    }
}

/// Do-while statement
#[derive(Debug, Clone)]
pub struct DoWhileStatement {
    pub token: String,
    pub body: Box<dyn Statement>,
    pub condition: Box<dyn Expression>,
}

impl Node for DoWhileStatement {
    fn string(&self) -> String {
        format!("do {{ {} }} lowkey {}", self.body.string(), self.condition.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for DoWhileStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(DoWhileStatement {
            token: self.token.clone(),
            body: self.body.clone_box(),
            condition: self.condition.clone_box(),
        })
    }
}

/// Try statement
#[derive(Debug, Clone)]
pub struct TryStatement {
    pub token: String,
    pub try_block: Box<dyn Statement>,
    pub catch_block: Option<Box<dyn Statement>>,
    pub finally_block: Option<Box<dyn Statement>>,
}

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
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for TryStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(TryStatement {
            token: self.token.clone(),
            try_block: self.try_block.clone_box(),
            catch_block: self.catch_block.as_ref().map(|b| b.clone_box()),
            finally_block: self.finally_block.as_ref().map(|b| b.clone_box()),
        })
    }
}

/// Return statement
#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub token: String,
    pub value: Option<Box<dyn Expression>>,
}

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
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(ReturnStatement {
            token: self.token.clone(),
            value: self.value.as_ref().map(|v| v.clone_box()),
        })
    }
}

// Re-export type switch from the main type_switch module
pub use crate::ast::type_switch::*;
