#!/usr/bin/env rust-script

//! Simple test to verify nil compilation

use cursed::ast::expressions::NilLiteral;
use cursed::ast::traits::{Node, Expression};

fn main() {
    println!("Testing nil literal creation...");
    
    // Test basic nil literal
    let nil_literal = NilLiteral::new();
    println!("Created nil literal: {}", nil_literal.string());
    println!("Token: {}", nil_literal.token_literal());
    
    // Test cloning
    let cloned = nil_literal.clone();
    println!("Cloned nil literal: {}", cloned.string());
    
    // Test expression trait
    let expr: Box<dyn Expression> = Box::new(nil_literal);
    println!("As expression: {}", expr.string());
    
    println!("All nil literal tests passed!");
}
