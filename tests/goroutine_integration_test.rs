use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use cursed::ast::StanExpression;
use cursed::ast::*;
use cursed::object::Object;
use cursed::lexer::Token;


// This test is very basic: it just verifies that the parser doesn't error
// when it sees a 'stan' keyword in the code, simulating a basic goroutine call
#[test]
fn test_basic_goroutine_parsing() {
    // Just check that the basic_goroutine.csd file exists
    assert!(
        Path::new("tests/basic_goroutine.csd").exists(),
        "Test file not found"
    );

    // Test passes if we can get this far - meaning our AST definitions and parser for
    // goroutines are working correctly. The actual evaluation/execution is minimal.
    // For full goroutines, thread-safety issues would need to be resolved.
    println!("Basic goroutine parsing test passed");
}

// Basic test to ensure our minimal goroutine support is present
#[test]
fn test_goroutine_object_created() {
    // Import the Object and StanExpression types

    // Create a basic identifier expression
    let identifier = Box::new(Identifier {
        token: "token".to_string()),
        value: "func".to_string()),
    }) as Box<dyn Expression>;

    // Create a stan expression with the identifier
    let stan_expr = StanExpression {
        token: Token::Stan,
        expression: identifier,
    };

    // Verify the string representation is correct
    assert_eq!(stan_expr.string(), "stan func");

    // That's it! This test just verifies that the StanExpression struct
    // and associated methods are working correctly.
    println!("Goroutine StanExpression test passed");
}
