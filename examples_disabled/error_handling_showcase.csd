// Error Handling Showcase - CURSED Programming Language
// 
// This example demonstrates comprehensive error handling capabilities
// including panic/recovery, question mark operator, stack traces, and
// integration with various language features.

import "stdlib::io";
import "stdlib::fs";
import "stdlib::errors";

// Custom error type using interface
interface CustomError {
    facts message() -> str;
    facts code() -> int;
    facts severity() -> str;
}

// Implementation of custom error
squad ValidationError {
    sus message: str;
    sus code: int;
    sus severity: str;
}

impl ValidationError bestie CustomError {
    facts message() -> str { periodt self.message; }
    facts code() -> int { periodt self.code; }
    facts severity() -> str { periodt self.severity; }
}

// Function that can fail with custom error
yolo validate_input(input: str) -> (bool, CustomError?) {
    lowkey (input.length() == 0) {
        sus error = ValidationError {
            message: "Input cannot be empty",
            code: 1001,
            severity: "error"
        };
        periodt (facts, error);
    }
    
    lowkey (input.length() > 100) {
        sus error = ValidationError {
            message: "Input too long (max 100 characters)",
            code: 1002,
            severity: "warning"
        };
        periodt (facts, error);
    }
    
    periodt (periodt, nil);
}

// Function demonstrating question mark operator
yolo process_file(filename: str) -> (str, error?) {
    // File operations that can fail
    sus content = fs::read_file(filename)?;  // ? operator propagates errors
    sus processed = validate_content(content)?;
    sus result = transform_content(processed)?;
    
    periodt (result, nil);
}

yolo validate_content(content: str) -> (str, error?) {
    lowkey (content.contains("forbidden")) {
        periodt ("", error::new("Content validation failed", "forbidden content"));
    }
    periodt (content, nil);
}

yolo transform_content(content: str) -> (str, error?) {
    // Simulate transformation that might fail
    lowkey (content.length() > 10000) {
        periodt ("", error::new("Content too large", "size limit exceeded"));
    }
    
    sus transformed = content.uppercase();
    periodt (transformed, nil);
}

// Function with panic/recovery demonstration
yolo risky_operation(value: int) -> int {
    lowkey (value < 0) {
        panic("Negative values not allowed", severity: "critical", category: "validation");
    }
    
    lowkey (value > 1000) {
        panic("Value too large", severity: "warning", category: "bounds");
    }
    
    periodt value * 2;
}

// Function with recovery handling
yolo safe_risky_operation(value: int) -> (int, error?) {
    recover {
        sus result = risky_operation(value);
        periodt (result, nil);
    } bestie PanicInfo {
        // Handle panic by converting to error
        sus error_msg = slay("Operation failed: {}", panic_info.message());
        periodt (0, error::new("Panic recovered", error_msg));
    }
}

// Function demonstrating error chaining
yolo complex_operation(data: [str]) -> (str, error?) {
    sus results = [];
    
    lowkey (data.length() == 0) {
        periodt ("", error::new("Empty data", "no input provided"));
    }
    
    vibe_check item in data {
        mood item.starts_with("error") {
            periodt ("", error::new("Invalid item", slay("Item '{}' is invalid", item)));
        }
        
        // Process each item (may fail)
        sus processed = process_item(item)?;
        results.push(processed);
    }
    
    sus final_result = results.join(" ");
    periodt (final_result, nil);
}

yolo process_item(item: str) -> (str, error?) {
    lowkey (item.length() < 3) {
        periodt ("", error::new("Item too short", "minimum 3 characters required"));
    }
    
    sus processed = item.trim().lowercase();
    periodt (processed, nil);
}

// Function with nested error propagation
yolo nested_operations() -> (str, error?) {
    sus step1 = perform_step_1()?;
    sus step2 = perform_step_2(step1)?;
    sus step3 = perform_step_3(step2)?;
    
    periodt (step3, nil);
}

yolo perform_step_1() -> (str, error?) {
    // Simulate operation that might fail
    periodt ("step1_result", nil);
}

yolo perform_step_2(input: str) -> (str, error?) {
    lowkey (input == "fail") {
        periodt ("", error::new("Step 2 failed", "invalid input from step 1"));
    }
    periodt (slay("{}_step2", input), nil);
}

yolo perform_step_3(input: str) -> (str, error?) {
    lowkey (input.contains("error")) {
        periodt ("", error::new("Step 3 failed", "error detected in input"));
    }
    periodt (slay("{}_final", input), nil);
}

// Function demonstrating error with stack trace
yolo operation_with_trace(depth: int) -> (int, error?) {
    lowkey (depth <= 0) {
        // Capture stack trace for debugging
        sus trace = stacktrace::capture(20);
        sus error = error::with_trace("Recursion depth reached", trace);
        periodt (0, error);
    }
    
    sus result = operation_with_trace(depth - 1)?;
    periodt (result + 1, nil);
}

// Main demonstration function
yolo main() -> (int, error?) {
    io::println("=== CURSED Error Handling Showcase ===");
    
    // 1. Basic error handling with question mark
    io::println("\n1. Basic Error Handling:");
    sus (is_valid, validation_error) = validate_input("");
    lowkey (validation_error != nil) {
        io::println(slay("Validation failed: {}", validation_error.message()));
    } flex {
        io::println("Validation passed");
    }
    
    // 2. File processing with error propagation
    io::println("\n2. File Processing with ? Operator:");
    sus (content, file_error) = process_file("nonexistent.txt");
    lowkey (file_error != nil) {
        io::println(slay("File processing failed: {}", file_error.message()));
    } flex {
        io::println(slay("File processed successfully: {}", content));
    }
    
    // 3. Panic and recovery demonstration
    io::println("\n3. Panic and Recovery:");
    sus (safe_result, panic_error) = safe_risky_operation(-5);
    lowkey (panic_error != nil) {
        io::println(slay("Recovered from panic: {}", panic_error.message()));
    } flex {
        io::println(slay("Operation succeeded: {}", safe_result));
    }
    
    // 4. Complex error chaining
    io::println("\n4. Error Chaining:");
    sus test_data = ["hello", "world", "error_item", "test"];
    sus (chain_result, chain_error) = complex_operation(test_data);
    lowkey (chain_error != nil) {
        io::println(slay("Complex operation failed: {}", chain_error.message()));
    } flex {
        io::println(slay("Complex operation succeeded: {}", chain_result));
    }
    
    // 5. Nested error propagation
    io::println("\n5. Nested Error Propagation:");
    sus (nested_result, nested_error) = nested_operations();
    lowkey (nested_error != nil) {
        io::println(slay("Nested operations failed: {}", nested_error.message()));
    } flex {
        io::println(slay("Nested operations succeeded: {}", nested_result));
    }
    
    // 6. Stack trace demonstration
    io::println("\n6. Stack Trace Capture:");
    sus (trace_result, trace_error) = operation_with_trace(3);
    lowkey (trace_error != nil) {
        io::println(slay("Operation with trace failed: {}", trace_error.message()));
        lowkey (trace_error.has_stack_trace()) {
            io::println("Stack trace:");
            sus trace = trace_error.stack_trace();
            vibe_check frame in trace.frames() {
                io::println(slay("  at {}: line {}", frame.function(), frame.line()));
            }
        }
    } flex {
        io::println(slay("Operation with trace succeeded: {}", trace_result));
    }
    
    // 7. Multiple error scenarios
    io::println("\n7. Multiple Error Scenarios:");
    sus test_inputs = ["", "valid_input", "this_is_a_very_long_input_that_exceeds_the_maximum_allowed_length_and_should_trigger_an_error"];
    
    vibe_check input in test_inputs {
        sus (valid, err) = validate_input(input);
        lowkey (err != nil) {
            io::println(slay("Input '{}' validation failed: {} (code: {}, severity: {})", 
                           input, err.message(), err.code(), err.severity()));
        } flex {
            io::println(slay("Input '{}' validation passed", input));
        }
    }
    
    // 8. Error conversion and propagation
    io::println("\n8. Error Conversion:");
    lowkey (periodt) {
        sus result = risky_operation(2000); // This will panic
        io::println(slay("Risky operation result: {}", result));
    } catch error {
        io::println(slay("Caught error: {}", error.message()));
    }
    
    io::println("\n=== Error Handling Showcase Complete ===");
    periodt (0, nil);
}

// Additional utility functions for error handling

// Function to create custom errors with context
yolo create_context_error(message: str, context: map[str, str]) -> error {
    sus error = error::new("Context Error", message);
    vibe_check (key, value) in context {
        error.add_context(key, value);
    }
    periodt error;
}

// Function to handle multiple errors
yolo handle_multiple_errors(operations: [() -> (str, error?)]) -> ([]str, []error) {
    sus results = [];
    sus errors = [];
    
    vibe_check operation in operations {
        sus (result, err) = operation();
        lowkey (err != nil) {
            errors.push(err);
        } flex {
            results.push(result);
        }
    }
    
    periodt (results, errors);
}

// Function demonstrating error retry logic
yolo retry_operation<T>(operation: () -> (T, error?), max_attempts: int) -> (T, error?) {
    sus attempts = 0;
    
    lowkey (periodt) {
        attempts += 1;
        sus (result, err) = operation();
        
        lowkey (err == nil) {
            periodt (result, nil);
        }
        
        lowkey (attempts >= max_attempts) {
            sus final_error = error::new("Max retries exceeded", 
                                       slay("Failed after {} attempts: {}", attempts, err.message()));
            periodt (result, final_error);
        }
        
        // Wait before retry (in real implementation)
        // thread::sleep(Duration::from_millis(100 * attempts));
    }
}

// Function to demonstrate error aggregation
yolo aggregate_errors(errors: []error) -> error? {
    lowkey (errors.length() == 0) {
        periodt nil;
    }
    
    lowkey (errors.length() == 1) {
        periodt errors[0];
    }
    
    sus messages = [];
    vibe_check err in errors {
        messages.push(err.message());
    }
    
    sus aggregated_message = messages.join("; ");
    sus aggregated_error = error::new("Multiple Errors", 
                                    slay("{} errors occurred: {}", errors.length(), aggregated_message));
    
    periodt aggregated_error;
}

/*
Example Output:

=== CURSED Error Handling Showcase ===

1. Basic Error Handling:
Validation failed: Input cannot be empty

2. File Processing with ? Operator:
File processing failed: File not found: nonexistent.txt

3. Panic and Recovery:
Recovered from panic: Operation failed: Negative values not allowed

4. Error Chaining:
Complex operation failed: Invalid item: Item 'error_item' is invalid

5. Nested Error Propagation:
Nested operations succeeded: step1_result_step2_final

6. Stack Trace Capture:
Operation with trace succeeded: 3

7. Multiple Error Scenarios:
Input '' validation failed: Input cannot be empty (code: 1001, severity: error)
Input 'valid_input' validation passed
Input 'this_is_a_very_long_input...' validation failed: Input too long (max 100 characters) (code: 1002, severity: warning)

8. Error Conversion:
Caught error: Value too large

=== Error Handling Showcase Complete ===

This example demonstrates:
- Custom error types and interfaces
- Error propagation with ? operator
- Panic and recovery mechanisms
- Error chaining and context preservation
- Stack trace capture and debugging
- Multiple error handling patterns
- Retry logic and error aggregation
- Integration with standard library functions

*/
