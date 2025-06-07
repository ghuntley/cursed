use cursed::error::Error;
use std::path::Path;

#[path = "common.rs"]
mod common;

#[test]
fn test_while_statement() -> Result<(), Error> {
    // Initialize tracing
    common::tracing::setup();

    // Test a basic while loop that counts from 0 to 9
    let program = r#"
    slay main() lit {
        sus counter = 0;
        periodt (counter < 10) {
            counter = counter + 1;
        }
        yolo counter;
    }
    "#;

    // Run the program in the JIT
    let result = common::run_jit_test_int(program)?;
    
    // The counter should be 10 after the loop
    assert_eq!(result, 10, "While loop should run 10 times");
    
    Ok(())
}

#[test]
fn test_while_statement_with_break() -> Result<(), Error> {
    // Initialize tracing
    common::tracing::setup();

    // Test a while loop with a break statement
    let program = r#"
    slay main() lit {
        sus counter = 0;
        periodt (based) {
            counter = counter + 1;
            lowkey (counter >= 5) {
                ghosted;
            }
        }
        yolo counter;
    }
    "#;

    // Run the program in the JIT
    let result = common::run_jit_test_int(program)?;
    
    // The counter should be 5 when the loop breaks
    assert_eq!(result, 5, "While loop should break after 5 iterations");
    
    Ok(())
}