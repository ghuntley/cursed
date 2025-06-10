//! Simple slice literal tests

use cursed::ast::SliceLiteral;
use cursed::ast::{Node, Expression, Identifier, IntegerLiteral}

#[test]
fn test_slice_literal_creation() {// Create a simple slice literal manually to test the AST node    
    let elements = vec![Box::new(IntegerLiteral {value: 1}) as Box<dyn Expression>,
        Box::new(IntegerLiteral {value: 2}) as Box<dyn Expression>,]
    
    println!(, Empty ";}