/// Conditional statements for the CURSED programming language
/// 
/// This module contains if statements, while loops, and for loops using Gen Z slang.

use crate::ast::traits::{Node, Statement, Expression};
use crate::ast::block::BlockStatement;
use crate::lexer::Token;
use std::any::Any;

/// If statement (lowkey condition { ... } highkey { ... })
#[derive(Debug, Clone)]
pub struct IfStatement {
    pub token: String,
    pub condition: Box<dyn Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<Box<dyn Statement>>,
}

impl IfStatement {
    pub fn new(
        token: String,
        condition: Box<dyn Expression>,
        consequence: BlockStatement,
        alternative: Option<Box<dyn Statement>>,
    ) -> Self {
        Self {
            token,
            condition,
            consequence,
            alternative,
        }
    }
    
    pub fn simple(condition: Box<dyn Expression>, consequence: BlockStatement) -> Self {
        Self {
            token: "lowkey".to_string(),
            condition,
            consequence,
            alternative: None,
        }
    }
    
    pub fn with_else(
        condition: Box<dyn Expression>,
        consequence: BlockStatement,
        alternative: Box<dyn Statement>,
    ) -> Self {
        Self {
            token: "lowkey".to_string(),
            condition,
            consequence,
            alternative: Some(alternative),
        }
    }
}

impl Node for IfStatement {
    fn string(&self) -> String {
        let mut result = format!("lowkey {} {}", self.condition.string(), self.consequence.string());
        
        if let Some(alt) = &self.alternative {
            result.push_str(&format!(" highkey {}", alt.string()));
        }
        
        result
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for IfStatement {
    fn as_any(&self) -> &dyn std::any::Any {
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

/// While statement (periodt condition { ... })
#[derive(Debug, Clone)]
pub struct WhileStatement {
    pub token: String,
    pub condition: Box<dyn Expression>,
    pub body: BlockStatement,
}

impl WhileStatement {
    pub fn new(token: String, condition: Box<dyn Expression>, body: BlockStatement) -> Self {
        Self {
            token,
            condition,
            body,
        }
    }
    
    pub fn while_loop(condition: Box<dyn Expression>, body: BlockStatement) -> Self {
        Self {
            token: "periodt".to_string(),
            condition,
            body,
        }
    }
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
    fn as_any(&self) -> &dyn std::any::Any {
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

/// For statement (bestie init; condition; post { ... })
#[derive(Debug, Clone)]
pub struct ForStatement {
    pub token: String,
    pub init: Option<Box<dyn Statement>>,
    pub condition: Option<Box<dyn Expression>>,
    pub post: Option<Box<dyn Statement>>,
    pub body: BlockStatement,
}

impl ForStatement {
    pub fn new(
        token: String,
        init: Option<Box<dyn Statement>>,
        condition: Option<Box<dyn Expression>>,
        post: Option<Box<dyn Statement>>,
        body: BlockStatement,
    ) -> Self {
        Self {
            token,
            init,
            condition,
            post,
            body,
        }
    }
    
    pub fn c_style(
        init: Box<dyn Statement>,
        condition: Box<dyn Expression>,
        post: Box<dyn Statement>,
        body: BlockStatement,
    ) -> Self {
        Self {
            token: "bestie".to_string(),
            init: Some(init),
            condition: Some(condition),
            post: Some(post),
            body,
        }
    }
    
    pub fn infinite(body: BlockStatement) -> Self {
        Self {
            token: "bestie".to_string(),
            init: None,
            condition: None,
            post: None,
            body,
        }
    }
}

impl Node for ForStatement {
    fn string(&self) -> String {
        let mut result = String::from("bestie ");
        
        if let Some(init) = &self.init {
            result.push_str(&init.string());
        }
        result.push(';');
        
        if let Some(condition) = &self.condition {
            result.push(' ');
            result.push_str(&condition.string());
        }
        result.push(';');
        
        if let Some(post) = &self.post {
            result.push(' ');
            result.push_str(&post.string());
        }
        
        result.push(' ');
        result.push_str(&self.body.string());
        
        result
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for ForStatement {
    fn as_any(&self) -> &dyn std::any::Any {
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

/// Range-based for statement (bestie item := flex iterable { ... })
#[derive(Debug, Clone)]
pub struct RangeForStatement {
    pub token: String,
    pub variable: String,
    pub index_variable: Option<String>,
    pub iterable: Box<dyn Expression>,
    pub body: BlockStatement,
}

impl RangeForStatement {
    pub fn new(
        token: String,
        variable: String,
        iterable: Box<dyn Expression>,
        body: BlockStatement,
    ) -> Self {
        Self {
            token,
            variable,
            index_variable: None,
            iterable,
            body,
        }
    }
    
    pub fn with_index(
        token: String,
        index_variable: String,
        variable: String,
        iterable: Box<dyn Expression>,
        body: BlockStatement,
    ) -> Self {
        Self {
            token,
            variable,
            index_variable: Some(index_variable),
            iterable,
            body,
        }
    }
    
    pub fn range_over(variable: &str, iterable: Box<dyn Expression>, body: BlockStatement) -> Self {
        Self {
            token: "bestie".to_string(),
            variable: variable.to_string(),
            index_variable: None,
            iterable,
            body,
        }
    }
}

impl Node for RangeForStatement {
    fn string(&self) -> String {
        if let Some(index) = &self.index_variable {
            format!(
                "bestie {}, {} := flex {} {}",
                index,
                self.variable,
                self.iterable.string(),
                self.body.string()
            )
        } else {
            format!(
                "bestie {} := flex {} {}",
                self.variable,
                self.iterable.string(),
                self.body.string()
            )
        }
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for RangeForStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(RangeForStatement {
            token: self.token.clone(),
            variable: self.variable.clone(),
            index_variable: self.index_variable.clone(),
            iterable: self.iterable.clone_box(),
            body: self.body.clone(),
        })
    }
}

/// Switch statement (vibe_check expression { mood cases... })
#[derive(Debug, Clone)]
pub struct SwitchStatement {
    pub token: String,
    pub value: Option<Box<dyn Expression>>,
    pub cases: Vec<SwitchCase>,
    pub default_case: Option<BlockStatement>,
}

impl SwitchStatement {
    pub fn new(
        token: String,
        value: Option<Box<dyn Expression>>,
        cases: Vec<SwitchCase>,
        default_case: Option<BlockStatement>,
    ) -> Self {
        Self {
            token,
            value,
            cases,
            default_case,
        }
    }
    
    pub fn on_value(value: Box<dyn Expression>, cases: Vec<SwitchCase>) -> Self {
        Self {
            token: "vibe_check".to_string(),
            value: Some(value),
            cases,
            default_case: None,
        }
    }
}

impl Node for SwitchStatement {
    fn string(&self) -> String {
        let mut result = String::from("vibe_check");
        
        if let Some(value) = &self.value {
            result.push(' ');
            result.push_str(&value.string());
        }
        
        result.push_str(" {\n");
        
        for case in &self.cases {
            result.push_str(&format!("  {}\n", case.string()));
        }
        
        if let Some(default) = &self.default_case {
            result.push_str(&format!("  basic: {}\n", default.string()));
        }
        
        result.push('}');
        result
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for SwitchStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(SwitchStatement {
            token: self.token.clone(),
            value: self.value.as_ref().map(|v| v.clone_box()),
            cases: self.cases.clone(),
            default_case: self.default_case.clone(),
        })
    }
}

/// Individual case within a switch statement
#[derive(Debug, Clone)]
pub struct SwitchCase {
    pub values: Vec<Box<dyn Expression>>,
    pub body: BlockStatement,
}

impl SwitchCase {
    pub fn new(values: Vec<Box<dyn Expression>>, body: BlockStatement) -> Self {
        Self { values, body }
    }
    
    pub fn single_value(value: Box<dyn Expression>, body: BlockStatement) -> Self {
        Self {
            values: vec![value],
            body,
        }
    }
}

impl Node for SwitchCase {
    fn string(&self) -> String {
        let values: Vec<String> = self.values.iter().map(|v| v.string()).collect();
        format!("mood {}: {}", values.join(", "), self.body.string())
    }

    fn token_literal(&self) -> String {
        "mood".to_string()
    }
}

/// Case statement for individual switch cases
#[derive(Debug, Clone)]
pub struct CaseStatement {
    pub token: String,
    pub values: Vec<Box<dyn Expression>>,
    pub body: BlockStatement,
}

impl CaseStatement {
    pub fn new(token: String, values: Vec<Box<dyn Expression>>, body: BlockStatement) -> Self {
        Self { token, values, body }
    }
}

impl Node for CaseStatement {
    fn string(&self) -> String {
        let values: Vec<String> = self.values.iter().map(|v| v.string()).collect();
        format!("mood {}: {}", values.join(", "), self.body.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for CaseStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(CaseStatement {
            token: self.token.clone(),
            values: self.values.iter().map(|v| v.clone_box()).collect(),
            body: self.body.clone(),
        })
    }
}

/// Else statement (highkey { ... })
#[derive(Debug, Clone)]
pub struct ElseStatement {
    pub token: String,
    pub body: BlockStatement,
}

impl ElseStatement {
    pub fn new(token: String, body: BlockStatement) -> Self {
        Self { token, body }
    }
    
    pub fn simple(body: BlockStatement) -> Self {
        Self {
            token: "highkey".to_string(),
            body,
        }
    }
}

impl Node for ElseStatement {
    fn string(&self) -> String {
        format!("highkey {}", self.body.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for ElseStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(ElseStatement {
            token: self.token.clone(),
            body: self.body.clone(),
        })
    }
}

/// Else if statement (highkey lowkey condition { ... })
#[derive(Debug, Clone)]
pub struct ElseIfStatement {
    pub token: String,
    pub condition: Box<dyn Expression>,
    pub body: BlockStatement,
}

impl ElseIfStatement {
    pub fn new(token: String, condition: Box<dyn Expression>, body: BlockStatement) -> Self {
        Self { token, condition, body }
    }
    
    pub fn simple(condition: Box<dyn Expression>, body: BlockStatement) -> Self {
        Self {
            token: "highkey".to_string(),
            condition,
            body,
        }
    }
}

impl Node for ElseIfStatement {
    fn string(&self) -> String {
        format!("highkey lowkey {} {}", self.condition.string(), self.body.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for ElseIfStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(ElseIfStatement {
            token: self.token.clone(),
            condition: self.condition.clone_box(),
            body: self.body.clone(),
        })
    }
}

/// Default statement for switch cases (basic { ... })
#[derive(Debug, Clone)]
pub struct DefaultStatement {
    pub token: String,
    pub body: BlockStatement,
}

impl DefaultStatement {
    pub fn new(token: String, body: BlockStatement) -> Self {
        Self { token, body }
    }
    
    pub fn simple(body: BlockStatement) -> Self {
        Self {
            token: "basic".to_string(),
            body,
        }
    }
}

impl Node for DefaultStatement {
    fn string(&self) -> String {
        format!("basic: {}", self.body.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for DefaultStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(DefaultStatement {
            token: self.token.clone(),
            body: self.body.clone(),
        })
    }
}

/// Helper functions for creating conditional statements
pub fn if_stmt(condition: Box<dyn Expression>, consequence: BlockStatement) -> IfStatement {
    IfStatement::simple(condition, consequence)
}
    
    pub fn while_stmt(condition: Box<dyn Expression>, body: BlockStatement) -> WhileStatement {
    WhileStatement::while_loop(condition, body)
}
    
    pub fn for_stmt(
    init: Option<Box<dyn Statement>>,
    condition: Option<Box<dyn Expression>>,
    post: Option<Box<dyn Statement>>,
    body: BlockStatement,
) -> ForStatement {
    ForStatement::new("bestie".to_string(), init, condition, post, body)
}
