//! Tests for RangeExpression Node implementation
//!
//! These tests verify that the RangeExpression struct correctly implements
//! the Node trait, ensuring it can be properly used in the AST.

use cursed::ast::expressions::range_expression::RangeExpression;
use cursed::ast::expressions::IntegerLiteral;
use cursed::ast::{Expression, Node};

#[test]
fn test_range_expression_node_implementation() {
    // Test simple range with only end
    let end = Box::new(IntegerLiteral::new(10));
    let range = RangeExpression::Range { end };
    
    assert_eq!(range.token_literal(), "range");
    assert_eq!(range.string(), "range 10");
    
    // Test range with start and end
    let start = Box::new(IntegerLiteral::new(5));
    let end = Box::new(IntegerLiteral::new(15));
    let range = RangeExpression::RangeFromTo { start, end };
    
    assert_eq!(range.token_literal(), "range");
    assert_eq!(range.string(), "range 5, 15");
    
    // Test range with start, end, and step
    let start = Box::new(IntegerLiteral::new(0));
    let end = Box::new(IntegerLiteral::new(20));
    let step = Box::new(IntegerLiteral::new(2));
    let range = RangeExpression::RangeFromToStep { start, end, step };
    
    assert_eq!(range.token_literal(), "range");
    assert_eq!(range.string(), "range 0, 20, 2");
}

#[test]
fn test_range_expression_node_type() {
    // Test that the node_type method returns the correct type name
    let end = Box::new(IntegerLiteral::new(10));
    let range = RangeExpression::Range { end };
    
    assert_eq!(range.node_type(), "RangeExpression");
}