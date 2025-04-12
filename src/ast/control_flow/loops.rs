//! AST nodes for loop-based control flow in the CURSED language.
//!
//! This module defines the AST representations for loop statements and related control flow:
//! - While loops: condition-controlled loops (called "periodt" in CURSED)
//! - For loops: general-purpose loops with init, condition, and post expressions (called "bestie" in CURSED)
//! - Break statements: exit a loop early
//! - Continue statements: skip to the next iteration of a loop
//!
//! These control flow constructs allow CURSED programs to execute code repeatedly
//! and control the execution flow within loops.

use crate::ast::statements::block::BlockStatement;
use crate::ast::{Expression, Node, Statement};
use std::any::Any;

/// Represents a while loop in the AST (called "periodt" in CURSED).
///
/// A while loop repeatedly executes a block of code as long as the specified
/// condition evaluates to true. The condition is checked before each iteration.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// periodt x < 10 {
///     vibez.println(x)
///     x = x + 1
/// }
/// ```
///
/// The AST would have a `WhileStatement` with:
/// - condition: InfixExpression for `x < 10`
/// - body: BlockStatement containing the print and assignment statements
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

/// Represents a for loop in the AST (called "bestie" in CURSED).
///
/// A for loop is a flexible looping construct that can appear in several forms:
///
/// 1. C-style: `bestie init; condition; post { body }`
///    - Initializes a variable, checks a condition, updates the variable after each iteration
///
/// 2. Condition-only: `bestie condition { body }`
///    - Similar to a while loop, executes while the condition is true
///
/// 3. Infinite loop: `bestie { body }`
///    - Executes indefinitely until a break statement is encountered
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// // C-style for loop
/// bestie i := 0; i < 10; i = i + 1 {
///     vibez.println(i)
/// }
///
/// // Condition-only loop
/// bestie hasMoreData() {
///     processData()
/// }
///
/// // Infinite loop
/// bestie {
///     handleEvents()
///     if shouldExit() { break }
/// }
/// ```
///
/// The AST would contain a `ForStatement` with different combinations of
/// init, condition, post, and body components depending on the form used.
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

/// Represents a break statement in the AST.
///
/// A break statement causes an immediate exit from the innermost enclosing loop
/// or switch statement. It transfers control to the statement following the
/// terminated loop or switch.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// bestie {
///     if done {
///         break
///     }
///     doWork()
/// }
/// ```
///
/// The AST would include a `BreakStatement` within the if statement's body.
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

/// Represents a continue statement in the AST.
///
/// A continue statement causes the program to skip the remainder of the current
/// loop iteration and proceed with the next iteration. In a for loop, the post-iteration
/// expression (if any) is still executed before the next check of the condition.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// bestie i := 0; i < 10; i = i + 1 {
///     if i % 2 == 0 {
///         continue
///     }
///     vibez.println("Odd number:", i)
/// }
/// ```
///
/// The AST would include a `ContinueStatement` within the if statement's body.
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
