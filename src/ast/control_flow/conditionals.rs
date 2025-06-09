//! AST nodes for conditional control flow in the CURSED language.
//!
//! This module defines the AST representations for conditional statements:
//! - If statements: standard conditional branching with optional else clause
//! - Switch statements: multi-way branching based on a value (called "vibe_check" in CURSED)
//! - Case statements: individual branches within a switch statement (called "mood" in CURSED)
//!
//! These control flow constructs allow CURSED programs to execute different code paths
//! based on runtime conditions.

use crate::ast::statements::block::BlockStatement;
use crate::ast::{Expression, Node, Statement};
use std::any::Any;

/// Represents an if statement in the AST.
///
/// An if statement evaluates a condition and executes a block of code if the condition
/// is true. It may optionally include an else clause that executes when the condition
/// is false.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// if x > 10 {
///     vibez.println("x is greater than 10")
/// } else {
///     vibez.println("x is not greater than 10")
/// }
/// ```
///
/// The AST would have an `IfStatement` with:
/// - condition: InfixExpression for `x > 10`
/// - consequence: BlockStatement containing the first print statement
/// - alternative: BlockStatement containing the second print statement
#[derive(Debug)]
pub struct IfStatement {
    pub token: String, // Token::If
    pub condition: Box<dyn Expression>,
    pub consequence: Box<BlockStatement>,
    pub alternative: Option<Box<BlockStatement>>,
}

impl Node for IfStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        let mut out = format!(
            "if {} {}",
            self.condition.string(),
            self.consequence.string()
        );
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

/// Represents a switch statement in the AST (called "vibe_check" in CURSED).
///
/// A switch/vibe_check statement evaluates an expression and executes different code
/// blocks based on which case matches the resulting value. It can include multiple
/// case branches and an optional default branch for when no case matches.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// vibe_check status {
///     mood "fire", "lit": {
///         vibez.println("That's awesome!")
///     }
///     mood "mid": {
///         vibez.println("Could be better")
///     }
///     basic: {
///         vibez.println("Unknown status")
///     }
/// }
/// ```
///
/// The AST would have a `SwitchStatement` with:
/// - value: Identifier for `status`
/// - cases: Two CaseStatements for "fire"/"lit" and "mid"
/// - default: BlockStatement for the "basic" (default) case
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

/// Represents a case clause in a switch statement (called "mood" in CURSED).
///
/// A case statement specifies one or more values to match against the switch
/// expression, along with a block of code to execute when a match occurs.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// mood "ok", "fine": {
///     vibez.println("Acceptable")
/// }
/// ```
///
/// The AST would have a `CaseStatement` with:
/// - expressions: Two StringLiterals for "ok" and "fine"
/// - body: BlockStatement containing the print statement
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
        let exprs: Vec<String> = self.expressions.iter().map(|expr| expr.string()).collect();
        format!("mood {}: {}", exprs.join(", "), self.body.string())
    }
}

impl Statement for CaseStatement {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}
