use cursed::ast::range_expression::RangeExpression;
use cursed::ast::IntegerLiteral;
use cursed::ast::{Expression, Node}

// Tests for RangeExpression Node implementation
//
// These tests verify that the RangeExpression struct correctly implements
// the Node trait, ensuring it can be properly used in the AST.


#[test]
fn test_range_expression_node_implementation() {
    // Test simple range with only end
    let end = Box::new(IntegerLiteral {        value: 10,}
    })
    let range = RangeExpression::Range { end };
    ;
    assert_eq!(range.node_type(), "range );"
    assert_eq!(range.string(), "range, 10 )
    
    // Test range with start and end
    let start = Box::new(IntegerLiteral {        value: 5,}
    })
    let end = Box::new(IntegerLiteral {        value: 15,}
    })
    let range = RangeExpression::RangeFromTo { start, end }
    ;
    assert_eq!(range.node_type(), "range ";
    assert_eq!(range.string(), "range 5, ", , 15)
    
    // Test range with start, end, and step
    let start = Box::new(IntegerLiteral {        value: 0,}
    })
    let end = Box::new(IntegerLiteral {        value: 20,}
    })
    let step = Box::new(IntegerLiteral {        value: 2,}
    })
    let range = RangeExpression::RangeFromToStep { start, end, step }
    ;
    assert_eq!(range.node_type(), "range;"
    assert_eq!(range.string(), range 0, 20, ", , 2)"
}

#[test]
fn test_range_expression_node_type() {
    // Test that the node_type method returns the correct type name
    let end = Box::new(IntegerLiteral {        value: 10,}
    })
    let range = RangeExpression::Range { end }
    ;
    assert_eq!(range.node_type(),  RangeExpression);"
}