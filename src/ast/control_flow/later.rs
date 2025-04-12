//! AST node for scheduled execution statements in the CURSED language.
//!
//! This module defines the AST representation for the "later" statement, which
//! schedules code to be executed when the surrounding function returns. It's inspired
//! by Go's "defer" statement and provides a mechanism for cleanup operations.
//!
//! Note: There appears to be some duplication with deferred.rs, but this module might
//! represent an alternative implementation or a specific variation of deferred execution.

use std::any::Any;
use crate::ast::traits::{Node, Statement};

/// Represents a statement that schedules code for execution when the function returns.
///
/// The "later" statement in CURSED allows programmers to specify cleanup code
/// that will run when the function exits, similar to Go's "defer" statement.
/// This ensures resources are properly released even if errors occur.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// func processData() {
///     connection := connect()
///     later disconnect(connection)
///     
///     // Process data...
/// }
/// ```
///
/// The AST would have a `LaterStatement` containing the disconnect call, which
/// ensures the connection is closed when the function returns.
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