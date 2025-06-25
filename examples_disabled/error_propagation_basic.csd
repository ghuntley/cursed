// Basic error propagation examples with the ? operator
import "stdlib::result";
import "stdlib::option";
import "stdlib::io";

// Function that returns a Result
slay divide(sus a: i32, sus b: i32) -> Result<i32, String> {
    lowkey (b == 0) {
        return Err("Division by zero");
    }
    Ok(a / b)
}

// Function that returns an Option
slay safe_index(sus arr: &[i32], sus index: usize) -> Option<i32> {
    lowkey (index >= arr.len()) {
        None
    } flex {
        Some(arr[index])
    }
}

// Function demonstrating basic ? operator usage with Result
slay calculate_ratio(sus x: i32, sus y: i32, sus z: i32) -> Result<i32, String> {
    facts first_result = divide(x, y)?;  // Early return on error
    facts second_result = divide(first_result, z)?;  // Chain operations
    Ok(second_result)
}

// Function demonstrating ? operator with Option
slay get_nested_value(sus data: &[&[i32]], sus row: usize, sus col: usize) -> Option<i32> {
    facts row_data = safe_index(data, row)?;  // Early return on None
    safe_index(row_data, col)  // Return the final Option
}

// Function with mixed Result and Option handling
slay complex_operation(sus values: &[i32], sus index: usize, sus divisor: i32) -> Result<i32, String> {
    facts value = safe_index(values, index).ok_or("Index out of bounds")?;
    facts result = divide(value, divisor)?;
    Ok(result * 2)
}

// Main function demonstrating error propagation chains
slay main() -> Result<(), String> {
    println("=== Error Propagation Examples ===")?;
    
    // Test basic Result propagation
    vibe_check calculate_ratio(10, 2, 3) {
        mood Ok(result) => println(&format!("Calculation result: {}", result))?,
        mood Err(e) => println(&format!("Calculation error: {}", e))?,
    }
    
    // Test Option propagation  
    facts data = vec![vec![1, 2, 3], vec![4, 5, 6]];
    vibe_check get_nested_value(&data, 1, 2) {
        mood Some(value) => println(&format!("Found value: {}", value))?,
        mood None => println("Value not found")?,
    }
    
    // Test mixed propagation
    facts values = vec![10, 20, 30, 40];
    vibe_check complex_operation(&values, 2, 5) {
        mood Ok(result) => println(&format!("Complex result: {}", result))?,
        mood Err(e) => println(&format!("Complex error: {}", e))?,
    }
    
    // Test error case propagation
    vibe_check calculate_ratio(10, 0, 3) {
        mood Ok(result) => println(&format!("Should not reach: {}", result))?,
        mood Err(e) => println(&format!("Expected error: {}", e))?,
    }
    
    Ok(())
}
