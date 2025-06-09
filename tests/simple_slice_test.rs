//! Simple slice literal tests

use cursed::ast::expressions::SliceLiteral;
use cursed::ast::{Node, Expression};
use cursed::lexer::Token;

#[test]
fn test_slice_literal_creation() {
    // Create a simple slice literal manually to test the AST node
    let element_type = Box::new(cursed::ast::expressions::Identifier {
        token: "normie".to_string(),
        value: "normie".to_string(),
    }) as Box<dyn Expression>;
    
    let elements = vec![
        Box::new(cursed::ast::expressions::IntegerLiteral {
            token: "1".to_string(),
            value: 1,
        }) as Box<dyn Expression>,
        Box::new(cursed::ast::expressions::IntegerLiteral {
            token: "2".to_string(),
            value: 2,
        }) as Box<dyn Expression>,
    ];
    
    let slice = SliceLiteral::new(
        Token::LBracket,
        element_type,
        elements,
    );
    
    assert_eq!(slice.len(), 2);
    assert!(!slice.is_empty());
    assert_eq!(slice.string(), "[]normie{1, 2}");
    
    println!("Slice literal string representation: {}", slice.string());
}

#[test]
fn test_empty_slice_literal_creation() {
    let element_type = Box::new(cursed::ast::expressions::Identifier {
        token: "normie".to_string(),
        value: "normie".to_string(),
    }) as Box<dyn Expression>;
    
    let slice = SliceLiteral::new(
        Token::LBracket,
        element_type,
        vec![],
    );
    
    assert_eq!(slice.len(), 0);
    assert!(slice.is_empty());
    assert_eq!(slice.string(), "[]normie{}");
    
    println!("Empty slice literal string representation: {}", slice.string());
}
