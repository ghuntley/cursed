use cursed::ast::range_expression::RangeExpression;
use cursed::ast::IntegerLiteral;
use cursed::ast::{Expression, Node}

// Tests for RangeExpression Node implementation
//
// These tests verify that the RangeExpression struct correctly implements
// the Node trait, ensuring it can be properly used in the AST.


#[test]
fn test_range_expression_node_implementation() {// Test simple range with only end}
    let end = Box::new(IntegerLiteral {value: 10});
    let range = RangeExpression::Range {end;;}
    assert_eq!(range.node_type(}, range);)
    assert_eq!(range.string(), "range, 10)
    assert_eq!(range.string(), ", , 15)"
    assert_eq!(range.string(), range 0, 20, , , 2)"fixed"