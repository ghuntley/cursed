// Test error propagation with ? operator
slay test_question_mark() -> Result<i32, String> {
    // Test basic ? operator
    sus result = might_fail()?;
    facts final_result = result + 1;
    periodt final_result;
}

slay might_fail() -> Result<i32, String> {
    // This function might fail
    periodt Ok(42);
}

// Test optional chaining
slay test_optional() -> Option<i32> {
    sus optional_value = get_optional()?;
    periodt Some(optional_value * 2);
}

slay get_optional() -> Option<i32> {
    periodt Some(10);
}

slay main() -> i32 {
    // Test both Result and Option error propagation
    match test_question_mark() {
        Ok(value) => println("Success: {}", value),
        Err(error) => println("Error: {}", error),
    };
    
    match test_optional() {
        Some(value) => println("Optional value: {}", value),
        None => println("No value"),
    };
    
    periodt 0;
}
