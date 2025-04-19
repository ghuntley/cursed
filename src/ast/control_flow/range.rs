//! AST representation for range clauses in for loops
//!
//! This module defines the AST node for range-based iterations, a common
//! control flow construct that allows efficient iteration over various types
//! such as numeric ranges, arrays, slices, and maps.

use crate::ast::{Expression, Node};
use std::any::Any;

/// Represents a range clause in the AST (e.g., `range 10`, `range array`, etc.)
///
/// Range clauses are used with for loops to iterate over sequences like arrays, slices,
/// or simple numeric ranges. The range clause can have different forms:
///
/// 1. Single value: `flex 10` - Iterate from 0 to 9
/// 2. Start/end: `flex 5, 10` - Iterate from 5 to 9
/// 3. Start/end/step: `flex 0, 10, 2` - Iterate from 0 to 9 with step 2 (0, 2, 4, 6, 8)
/// 4. Container: `flex container` - Iterate over elements of a container
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// bestie i := flex 10 {
///     vibez.println(i)
/// }
///
/// bestie value := flex array {
///     vibez.println(value)
/// }
///
/// bestie key, value := flex map {
///     vibez.println(key, value)
/// }
/// ```
///
/// The AST would contain a `RangeClause` with different variations based on the form used.
pub struct RangeClause {
    pub token: String, // Token::Flex
    pub start: Option<Box<dyn Expression>>,  // Start value (optional)
    pub end: Box<dyn Expression>,            // End value or container
    pub step: Option<Box<dyn Expression>>,   // Step value (optional)
    pub is_container: bool,                  // Whether this is a container range
}

impl Node for RangeClause {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        let mut out = String::from("flex ");
        
        if let Some(start) = &self.start {
            out.push_str(&start.string());
            out.push_str(", ");
        }
        
        out.push_str(&self.end.string());
        
        if let Some(step) = &self.step {
            out.push_str(", ");
            out.push_str(&step.string());
        }
        
        out
    }
}

impl Expression for RangeClause {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Represents a range-based for statement in the AST.
///
/// A range-based for loop provides simplified syntax for iterating over ranges, arrays,
/// or other containers. It can appear in several forms:
///
/// 1. Single iterator: `bestie x := flex 10 { ... }`
///    - Assigns each value from the range to variable x
///
/// 2. Tuple iterator: `bestie i, v := flex container { ... }`
///    - Assigns index and value (for arrays) or key and value (for maps)
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// // Numeric range
/// bestie i := flex 10 {
///     vibez.println(i)
/// }
///
/// // Container iteration
/// bestie value := flex myArray {
///     vibez.println(value)
/// }
///
/// // Key-value iteration
/// bestie key, value := flex myMap {
///     vibez.println(key, value)
/// }
/// ```
pub struct RangeForStatement {
    pub token: String, // Token::Bestie
    pub value_var: String, // Single variable (values) or first variable (keys/indices)
    pub key_var: Option<String>, // Second variable (values) if present
    pub range: Box<RangeClause>, // Range expression
    pub body: Box<crate::ast::statements::block::BlockStatement>, // Loop body
}

impl Node for RangeForStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("bestie ");
        
        if let Some(key_var) = &self.key_var {
            out.push_str(key_var);
            out.push_str(", ");
        }
        
        out.push_str(&self.value_var);
        out.push_str(" := ");
        out.push_str(&self.range.string());
        out.push_str(" ");
        out.push_str(&self.body.string());
        
        out
    }
}

impl crate::ast::Statement for RangeForStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}