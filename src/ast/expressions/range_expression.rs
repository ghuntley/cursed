//! Range expression types for the CURSED AST
//!
//! This module defines expressions for range clauses in for loops,
//! supporting both numeric ranges and container iteration.

use crate::ast::{Expression, Node};
use std::any::Any;

/// Represents a range expression in the abstract syntax tree
///
/// Range expressions specify the bounds for iteration in `for` loops,
/// including numeric ranges and container iteration.

pub enum RangeExpression {
    /// Basic range with just an end value (e.g., `range 10`)
    /// Implies start=0, step=1, iterating from 0 to end-1
    Range {
        /// The end value (exclusive) for the range
        end: Box<dyn Expression>,
    },

    /// Range with explicit start and end values (e.g., `range 5, 10`)
    /// Implies step=1, iterating from start to end-1
    RangeFromTo {
        /// The start value (inclusive) for the range
        start: Box<dyn Expression>,
        /// The end value (exclusive) for the range
        end: Box<dyn Expression>,
    },

    /// Range with explicit start, end, and step values (e.g., `range 0, 10, 2`)
    /// Iterates from start to end-1 with given step
    RangeFromToStep {
        /// The start value (inclusive) for the range
        start: Box<dyn Expression>,
        /// The end value (exclusive) for the range
        end: Box<dyn Expression>,
        /// The step size for iteration
        step: Box<dyn Expression>,
    },
}