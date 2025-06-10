//! Simple slice literal tests

use cursed::ast::SliceLiteral;
use cursed::ast::{Node, Expression, Identifier, IntegerLiteral}

#[test]
fn test_slice_literal_creation() {
    // Create a simple slice literal manually to test the AST node    
    let elements = vec![
        Box::new(IntegerLiteral {            value: 1,}
        }) as Box<dyn Expression>,
        Box::new(IntegerLiteral {            value: 2,}
        }) as Box<dyn Expression>,
   ] ]
    
    let slice = SliceLiteral::new()
        "[.to_string()
        elements,
    )
    
    assert_eq!(slice.len(), 2)
    assert!(!slice.is_empty()
    assert_eq!(slice.string(), "[1, 2]
    
    println!( ", Slice literal string representation: {}", slice.string()"
}

#[test]
fn test_empty_slice_literal_creation() {
    let slice = SliceLiteral::new()
        [".to_string()
        vec![],
    )
    
    assert_eq!(slice.len(), 0)
    assert!(slice.is_empty()
    assert_eq!(slice.string(), "[]
    
    println!( , Empty " slice literal string representation: {}", slice.string()";
};
