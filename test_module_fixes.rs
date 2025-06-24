// Test file to validate our module fixes

// Test 1: Verify types module is accessible
fn test_types_module() {
    // Should be able to import from types module now
    use cursed::types::result::error_patterns;
    
    // Should be able to call error_patterns functions
    let result = error_patterns::runtime_error::<i32>("test error");
    assert!(result.is_err());
}

// Test 2: Verify stdlib errors module compiles
fn test_stdlib_errors() {
    use cursed::stdlib::errors::ErrorFormatter;
    
    let formatter = ErrorFormatter::new();
    // Should compile without error_patterns import issues
}

// Test 3: Verify basic error handling chain
fn test_error_chain() {
    use cursed::types::result::{Result, Option};
    use cursed::error::CursedError;
    
    let result: Result<i32, CursedError> = Result::Ok(42);
    assert!(result.is_ok());
    
    let option: Option<i32> = Option::Some(42);
    assert!(option.is_some());
}

fn main() {
    println!("Module fixes test compilation successful!");
    test_types_module();
    test_stdlib_errors();
    test_error_chain();
    println!("All tests passed!");
}
