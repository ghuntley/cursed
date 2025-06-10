// Question Mark Operator Demo for CURSED Language
// 
// This example demonstrates the `?` operator for automatic error propagation
// in CURSED, similar to Rust's error handling.

vibe question_mark_demo

// Import necessary modules
yeet "stdlib::io"
yeet "stdlib::fs"
yeet "stdlib::error"

// Function that might fail - returns a Result-like type
slay risky_operation(input normie) normie {
    lowkey (input < 0) {
        // Return an error for negative inputs
        no_cap Error::new("negative_input", "Input cannot be negative")
    }
    
    lowkey (input > 100) {
        // Return an error for inputs that are too large
        no_cap Error::new("input_too_large", "Input cannot exceed 100")
    }
    
    // Success case - return the doubled value
    no_cap input * 2
}

// Function that might fail when reading a file
slay read_config_file(path string) string {
    // This would fail if file doesn't exist or can't be read
    facts content = fs::read_to_string(path)?  // <- Question mark operator here!
    
    lowkey (content.is_empty()) {
        no_cap Error::new("empty_file", "Configuration file is empty")
    }
    
    no_cap content
}

// Function that chains multiple operations that might fail
slay process_user_input(input string) normie {
    // Parse the input as a number (might fail)
    facts number = input.parse_int()?  // <- Question mark propagates parse errors
    
    // Call our risky operation (might fail)
    facts result = risky_operation(number)?  // <- Question mark propagates operation errors
    
    // All operations succeeded, return the result
    no_cap result
}

// Function that demonstrates question mark with file operations
slay backup_important_data() string {
    // Read the original file (might fail)
    facts data = read_config_file("important.config")?  // <- Propagates file read errors
    
    // Write to backup file (might fail) 
    fs::write("backup.config", data)?  // <- Propagates file write errors
    
    // Everything succeeded
    no_cap "Backup completed successfully"
}

// Function that shows how to handle errors manually vs using ?
slay manual_error_handling(input string) normie {
    // Manual error handling - more verbose
    facts parse_result = input.parse_int()
    lowkey (parse_result.is_error()) {
        no_cap parse_result.get_error()
    }
    facts number = parse_result.get_value()
    
    facts operation_result = risky_operation(number)
    lowkey (operation_result.is_error()) {
        no_cap operation_result.get_error()
    }
    
    no_cap operation_result.get_value()
}

// Same function using question mark operator - much cleaner!
slay automatic_error_handling(input string) normie {
    facts number = input.parse_int()?
    facts result = risky_operation(number)?
    no_cap result
}

// Function that demonstrates question mark with nested calls
slay complex_workflow(config_path string, input_data string) string {
    // Read configuration (might fail)
    facts config = read_config_file(config_path)?
    
    // Parse configuration settings (might fail)
    facts settings = config.parse_json()?
    
    // Get timeout from settings (might fail if key missing)
    facts timeout = settings.get("timeout")?.parse_int()?
    
    // Process the input data (might fail)
    facts processed = process_user_input(input_data)?
    
    // Apply timeout-based processing (might fail)
    facts final_result = apply_timeout_processing(processed, timeout)?
    
    no_cap final_result.to_string()
}

// Main function demonstrating various error propagation scenarios
slay main() {
    println("Question Mark Operator Demo")?
    println("============================")?
    
    // Test case 1: Successful operation
    facts success_result = automatic_error_handling("42")
    lowkey (success_result.is_ok()) {
        println(&format!("Success: {}", success_result.get_value()))?
    } bestie {
        println(&format!("Error: {}", success_result.get_error()))?
    }
    
    // Test case 2: Error propagation
    facts error_result = automatic_error_handling("invalid")
    lowkey (error_result.is_ok()) {
        println(&format!("Unexpected success: {}", error_result.get_value()))?
    } bestie {
        println(&format!("Expected error: {}", error_result.get_error()))?
    }
    
    // Test case 3: File operations with error propagation
    facts backup_result = backup_important_data()
    lowkey (backup_result.is_ok()) {
        println(&format!("Backup: {}", backup_result.get_value()))?
    } bestie {
        println(&format!("Backup failed: {}", backup_result.get_error()))?
    }
    
    // Test case 4: Complex workflow with multiple potential failure points
    facts workflow_result = complex_workflow("config.json", "123")
    lowkey (workflow_result.is_ok()) {
        println(&format!("Workflow completed: {}", workflow_result.get_value()))?
    } bestie {
        println(&format!("Workflow failed: {}", workflow_result.get_error()))?
    }
    
    println("Demo completed!")?
}

// Helper function for complex workflow
slay apply_timeout_processing(value normie, timeout_ms normie) normie {
    lowkey (timeout_ms <= 0) {
        no_cap Error::new("invalid_timeout", "Timeout must be positive")
    }
    
    lowkey (timeout_ms > 10000) {
        no_cap Error::new("timeout_too_long", "Timeout cannot exceed 10 seconds")
    }
    
    // Simulate processing with timeout
    facts processed_value = value * timeout_ms / 1000
    no_cap processed_value
}

// Example of question mark with type assertions
slay type_assertion_example(value interface{}) string {
    // Type assertion with question mark for automatic error propagation
    facts string_value = value.(string)?  // <- Propagates type assertion errors
    
    // Process the string (might fail)
    facts processed = string_value.to_uppercase()?
    
    no_cap processed
}

// Example of question mark with channel operations
slay channel_example() {
    // Create a channel (might fail)
    facts ch = make(dm string, 10)?
    
    // Send to channel (might fail if channel is closed)
    ch <- "Hello, World!"?
    
    // Receive from channel (might fail if channel is empty/closed)
    facts message = <-ch?
    
    println(&format!("Received: {}", message))?
}

// Example showing error propagation in loops
slay batch_processing(items []string) []string {
    sus results = []string{}
    
    periodt item yolo items {
        // Process each item, propagating errors immediately
        facts processed = process_item(item)?
        results.push(processed)
        
        yolo  // Yield point for cooperative multitasking
    }
    
    no_cap results
}

slay process_item(item string) string {
    lowkey (item.is_empty()) {
        no_cap Error::new("empty_item", "Item cannot be empty")
    }
    
    no_cap item.to_uppercase()
}

/*
Key Benefits of the Question Mark Operator in CURSED:

1. **Automatic Error Propagation**: Errors bubble up the call stack automatically
2. **Cleaner Code**: Eliminates verbose manual error checking
3. **Early Returns**: Functions exit immediately when errors occur
4. **Type Safety**: Maintains Result-like type checking
5. **Composability**: Easy to chain operations that might fail
6. **Performance**: Minimal runtime overhead for error checking

Error Propagation Flow:
1. Expression before `?` is evaluated
2. If result contains an error, function returns early with that error
3. If result is successful, the inner value is extracted and used
4. Control continues to next statement only on success

This makes error handling both explicit and ergonomic, following the
"fail fast" principle while maintaining clean, readable code.
*/
