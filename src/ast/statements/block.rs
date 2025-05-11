//! AST node for block statements in the CURSED language.
//!
//! This module defines the AST representation for block statements, which are
//! groups of statements enclosed in curly braces. Blocks form the basic structure
//! for control flow constructs like if statements, loops, and function bodies.
//!
//! Blocks create new scopes for variables and enforce execution ordering of
//! their contained statements.

use crate::ast::{Node, Statement};
use std::any::Any;

/// Represents a block of statements in the AST.
///
/// A block statement contains zero or more statements enclosed in curly braces.
/// Blocks are used to group statements together for if/else branches, loop bodies,
/// function bodies, and any other context where multiple statements should be treated
/// as a single unit.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// {
///     x := 10
///     vibez.println(x)
///     x = x + 1
/// }
/// ```
///
/// The AST would have a `BlockStatement` containing three statements:
/// - A declaration statement (x := 10)
/// - An expression statement (println call)
/// - An assignment statement (x = x + 1)
use crate::lexer::token::Token;

pub struct BlockStatement {
    pub token: Token, // Token::LBrace
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        for stmt in &self.statements {
            out.push_str(&stmt.string());
        }
        out
    }
}

impl Statement for BlockStatement {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}
