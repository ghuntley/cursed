//! AST node for deferred execution statements in the CURSED language.
//!
//! This module defines the AST representation for the deferred execution statement,
//! which is called "later" in CURSED (inspired by Go's "defer" statement). These statements
//! specify code that should be executed when the surrounding function returns,
//! regardless of which path the function takes to return.
//!
//! Deferred statements are typically used for cleanup operations, like closing files
//! or releasing resources, ensuring they happen even if errors occur.

use std::any::Any;
use crate::ast::{Node, Statement};

/// Represents a deferred execution statement in the AST (called "later" in CURSED).
///
/// A later statement specifies a statement that will be executed when the surrounding
/// function returns, regardless of where or how it returns. Deferred statements are
/// executed in last-in-first-out (LIFO) order - the last deferred statement is
/// executed first.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// func readFile(path string) string {
///     file := dropz.open(path)
///     later dropz.close(file)
///     
///     // Read the file...
///     return content
/// }
/// ```
///
/// The AST would have a `LaterStatement` containing the `dropz.close(file)` statement
/// which will be executed when the function returns, ensuring the file is closed.
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