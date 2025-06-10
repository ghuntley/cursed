use cursed::error::Error;
use std::path::Path;

#[path = "common/mod.rs"]
mod common;

#[test]
fn test_while_statement() -> Result<(), Error> {
    common::tracing::setup();
    
    // Test a simple while loop
    let program = r#"
        sus counter = 0;
        lowkey (counter < 10) {
            counter = counter + 1;
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
    common::tracing::setup();

    // Test a while loop with a break statement
    let program = r#"
        sus counter = 0;
        lowkey (counter < 10) {
            counter = counter + 1;
            lowkey (counter == 5) {
                periodt;  // break
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
