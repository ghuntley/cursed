# Error Handling

Error handling is crucial for writing robust programs. This tutorial covers CURSED's error handling mechanisms using the `yikes` type and panic recovery.

## Learning Objectives

By the end of this tutorial, you'll be able to:
- Understand the `yikes` error type
- Handle errors with proper patterns
- Use panic recovery with `fam` blocks
- Implement error propagation with `shook`
- Write defensive code that handles failures gracefully

## The `yikes` Error Type

CURSED uses the `yikes` type to represent errors:

```cursed
# basic_errors.csd - Introduction to error handling

vibe main

yeet "vibez"

# Function that can fail
slay divide(a normie, b normie) (normie, yikes) {
    lowkey b == 0 {
        damn 0, yikes("Division by zero is not allowed")
    }
    damn a / b, cringe  # cringe means "no error"
}

# Function that always succeeds
slay multiply(a normie, b normie) normie {
    damn a * b
}

slay main() {
    # Successful operation
    sus result, err := divide(10, 2)
    lowkey err != cringe {
        vibez.spill("Error: " + err.message())
    } highkey {
        vibez.spill("10 ÷ 2 = " + string(result))
    }
    
    # Failed operation
    sus result2, err2 := divide(10, 0)
    lowkey err2 != cringe {
        vibez.spill("Error: " + err2.message())
    } highkey {
        vibez.spill("10 ÷ 0 = " + string(result2))
    }
    
    # Function that can't fail
    sus product := multiply(5, 3)
    vibez.spill("5 × 3 = " + string(product))
}
```

## Error Handling Patterns

### Basic Error Checking

```cursed
# error_patterns.csd - Common error handling patterns

vibe main

yeet "vibez"

# File operation simulation
slay read_file(filename tea) (tea, yikes) {
    vibe_check filename {
        mood "config.txt":
            damn "database_url=localhost:5432", cringe
        mood "data.txt":
            damn "user_count=1000", cringe
        mood "secret.txt":
            damn "", yikes("Permission denied")
        basic:
            damn "", yikes("File not found: " + filename)
    }
}

# Network operation simulation
slay connect_to_server(address tea) (tea, yikes) {
    vibe_check address {
        mood "localhost":
            damn "Connected to localhost", cringe
        mood "production.com":
            damn "", yikes("Connection timeout")
        basic:
            damn "", yikes("Invalid server address")
    }
}

# Function that processes configuration
slay load_config() yikes {
    sus config_data, err := read_file("config.txt")
    lowkey err != cringe {
        damn err  # Return the error
    }
    
    vibez.spill("Config loaded: " + config_data)
    damn cringe  # Success
}

# Function that handles multiple errors
slay startup_sequence() yikes {
    vibez.spill("Starting application...")
    
    # Load configuration
    sus err := load_config()
    lowkey err != cringe {
        damn err
    }
    
    # Connect to server
    sus connection, err2 := connect_to_server("localhost")
    lowkey err2 != cringe {
        damn err2
    }
    
    vibez.spill("Startup complete: " + connection)
    damn cringe
}

slay main() {
    vibez.spill("=== Error Handling Demo ===")
    
    # Test individual functions
    sus content, err := read_file("data.txt")
    lowkey err != cringe {
        vibez.spill("Failed to read file: " + err.message())
    } highkey {
        vibez.spill("File content: " + content)
    }
    
    # Test error propagation
    sus startup_err := startup_sequence()
    lowkey startup_err != cringe {
        vibez.spill("Startup failed: " + startup_err.message())
    } highkey {
        vibez.spill("Application started successfully!")
    }
}
```

### Error Wrapping and Context

```cursed
# error_context.csd - Adding context to errors

vibe main

yeet "vibez"

# Helper function to wrap errors with context
slay wrap_error(err yikes, context tea) yikes {
    lowkey err == cringe {
        damn cringe
    }
    damn yikes(context + ": " + err.message())
}

# Database operation simulation
slay query_database(query tea) (tea, yikes) {
    vibe_check query {
        mood "SELECT * FROM users":
            damn "user1,user2,user3", cringe
        mood "SELECT * FROM products":
            damn "", yikes("Table 'products' doesn't exist")
        mood "DELETE FROM users":
            damn "", yikes("Permission denied")
        basic:
            damn "", yikes("Invalid SQL syntax")
    }
}

# Function that adds context to database errors
slay get_user_list() (tea, yikes) {
    sus result, err := query_database("SELECT * FROM users")
    lowkey err != cringe {
        damn "", wrap_error(err, "Failed to get user list")
    }
    damn result, cringe
}

# Function that handles complex operations
slay generate_report() yikes {
    vibez.spill("Generating user report...")
    
    sus users, err := get_user_list()
    lowkey err != cringe {
        damn wrap_error(err, "Report generation failed")
    }
    
    vibez.spill("Report data: " + users)
    vibez.spill("Report generated successfully!")
    damn cringe
}

slay main() {
    vibez.spill("=== Error Context Demo ===")
    
    # Test successful operation
    sus err := generate_report()
    lowkey err != cringe {
        vibez.spill("ERROR: " + err.message())
    }
    
    # Test error with context
    vibez.spill("\n--- Testing error scenario ---")
    sus result, err2 := query_database("SELECT * FROM products")
    lowkey err2 != cringe {
        sus wrapped_err := wrap_error(err2, "Product query failed")
        vibez.spill("ERROR: " + wrapped_err.message())
    }
}
```

## Error Propagation with `shook`

The `shook` operator provides automatic error propagation:

```cursed
# error_propagation.csd - Using shook for error propagation

vibe main

yeet "vibez"

# Simulated operations that can fail
slay validate_input(input tea) yikes {
    lowkey input == "" {
        damn yikes("Input cannot be empty")
    }
    lowkey input == "invalid" {
        damn yikes("Invalid input format")
    }
    damn cringe
}

slay process_data(data tea) yikes {
    lowkey data == "corrupt" {
        damn yikes("Data is corrupted")
    }
    damn cringe
}

slay save_result(result tea) yikes {
    lowkey result == "fail" {
        damn yikes("Failed to save result")
    }
    damn cringe
}

# Manual error handling (verbose)
slay process_manual(input tea) yikes {
    sus err := validate_input(input)
    lowkey err != cringe {
        damn err
    }
    
    sus err2 := process_data(input)
    lowkey err2 != cringe {
        damn err2
    }
    
    sus err3 := save_result(input)
    lowkey err3 != cringe {
        damn err3
    }
    
    damn cringe
}

# Using shook for automatic propagation
slay process_with_shook(input tea) yikes {
    validate_input(input) shook
    process_data(input) shook
    save_result(input) shook
    damn cringe
}

slay main() {
    vibez.spill("=== Error Propagation Demo ===")
    
    # Test successful case
    sus err := process_with_shook("valid_data")
    lowkey err != cringe {
        vibez.spill("ERROR: " + err.message())
    } highkey {
        vibez.spill("✅ Processing completed successfully!")
    }
    
    # Test failure cases
    sus test_cases := []tea{"", "invalid", "corrupt", "fail"}
    
    bestie i := 0; i < 4; i++ {
        sus test_input := test_cases[i]
        vibez.spill("\nTesting input: '" + test_input + "'")
        
        sus err := process_with_shook(test_input)
        lowkey err != cringe {
            vibez.spill("❌ ERROR: " + err.message())
        } highkey {
            vibez.spill("✅ Success!")
        }
    }
}
```

## Panic and Recovery

### Understanding Panics

```cursed
# panics.csd - Understanding panic situations

vibe main

yeet "vibez"

# Function that might panic
slay risky_operation(value normie) normie {
    lowkey value < 0 {
        shook("Negative values are not allowed!")  # This causes a panic
    }
    lowkey value > 100 {
        shook("Value too large!")  # This also causes a panic
    }
    damn value * 2
}

# Function that demonstrates panic
slay demonstrate_panic() {
    vibez.spill("About to call risky operation with -5...")
    sus result := risky_operation(-5)  # This will panic
    vibez.spill("Result: " + string(result))  # This won't execute
}

slay main() {
    vibez.spill("=== Panic Demo ===")
    vibez.spill("Note: This would crash the program!")
    
    # Normally this would crash:
    # demonstrate_panic()
    
    vibez.spill("Instead, let's handle it with recovery...")
}
```

### Panic Recovery with `fam`

```cursed
# panic_recovery.csd - Recovering from panics

vibe main

yeet "vibez"

# Function that might panic
slay divide_risky(a normie, b normie) normie {
    lowkey b == 0 {
        shook("Division by zero!")
    }
    damn a / b
}

# Function that recovers from panics
slay safe_divide(a normie, b normie) (normie, yikes) {
    fam {
        sus result := divide_risky(a, b)
        damn result, cringe
    } sus panic_value {
        vibez.spill("Recovered from panic: " + panic_value.message())
        damn 0, yikes("Division failed: " + panic_value.message())
    }
}

# Function that demonstrates resource cleanup
slay process_with_cleanup(data tea) yikes {
    vibez.spill("Starting process...")
    
    # Simulate resource allocation
    vibez.spill("Allocating resources...")
    
    fam {
        # Risky operation
        lowkey data == "crash" {
            shook("Simulated crash!")
        }
        
        vibez.spill("Processing data: " + data)
        vibez.spill("Process completed successfully!")
        damn cringe
    } sus panic_value {
        vibez.spill("ERROR: Process panicked: " + panic_value.message())
        damn yikes("Process failed due to panic")
    }
    
    # This always executes (like defer)
    vibez.spill("Cleaning up resources...")
}

slay main() {
    vibez.spill("=== Panic Recovery Demo ===")
    
    # Test safe division
    sus result, err := safe_divide(10, 2)
    lowkey err != cringe {
        vibez.spill("Division error: " + err.message())
    } highkey {
        vibez.spill("10 ÷ 2 = " + string(result))
    }
    
    # Test division by zero
    sus result2, err2 := safe_divide(10, 0)
    lowkey err2 != cringe {
        vibez.spill("Division error: " + err2.message())
    } highkey {
        vibez.spill("10 ÷ 0 = " + string(result2))
    }
    
    # Test process with cleanup
    vibez.spill("\n--- Testing normal process ---")
    sus err3 := process_with_cleanup("normal_data")
    lowkey err3 != cringe {
        vibez.spill("Process error: " + err3.message())
    }
    
    vibez.spill("\n--- Testing crash scenario ---")
    sus err4 := process_with_cleanup("crash")
    lowkey err4 != cringe {
        vibez.spill("Process error: " + err4.message())
    }
}
```

## Custom Error Types

```cursed
# custom_errors.csd - Creating custom error types

vibe main

yeet "vibez"

# Custom error types using structs
be_like ValidationError squad {
    field tea
    message tea
    code normie
}

be_like NetworkError squad {
    address tea
    message tea
    timeout_seconds normie
}

# Function to create validation errors
slay create_validation_error(field tea, message tea, code normie) yikes {
    damn yikes("Validation error in " + field + ": " + message + " (code: " + string(code) + ")")
}

# Function to create network errors
slay create_network_error(address tea, message tea, timeout normie) yikes {
    damn yikes("Network error connecting to " + address + ": " + message + " (timeout: " + string(timeout) + "s)")
}

# Function that validates user input
slay validate_user(name tea, email tea, age normie) yikes {
    lowkey name == "" {
        damn create_validation_error("name", "Name cannot be empty", 1001)
    }
    
    lowkey age < 0 || age > 150 {
        damn create_validation_error("age", "Age must be between 0 and 150", 1002)
    }
    
    lowkey email == "" {
        damn create_validation_error("email", "Email cannot be empty", 1003)
    }
    
    # Simple email validation
    lowkey email.find("@") == -1 {  # Conceptual - actual string methods may vary
        damn create_validation_error("email", "Invalid email format", 1004)
    }
    
    damn cringe
}

# Function that simulates network operations
slay connect_to_api(endpoint tea) (tea, yikes) {
    vibe_check endpoint {
        mood "api.example.com":
            damn "Connected successfully", cringe
        mood "slow.api.com":
            damn "", create_network_error(endpoint, "Connection timeout", 30)
        mood "down.api.com":
            damn "", create_network_error(endpoint, "Server unavailable", 0)
        basic:
            damn "", create_network_error(endpoint, "Invalid endpoint", 0)
    }
}

# Function that handles multiple error types
slay process_user_registration(name tea, email tea, age normie, api_endpoint tea) yikes {
    # Validate input
    sus validation_err := validate_user(name, email, age)
    lowkey validation_err != cringe {
        damn validation_err
    }
    
    # Connect to API
    sus response, network_err := connect_to_api(api_endpoint)
    lowkey network_err != cringe {
        damn network_err
    }
    
    vibez.spill("User registered successfully!")
    vibez.spill("API response: " + response)
    damn cringe
}

slay main() {
    vibez.spill("=== Custom Error Types Demo ===")
    
    # Test successful registration
    sus err := process_user_registration("Alice", "alice@example.com", 25, "api.example.com")
    lowkey err != cringe {
        vibez.spill("Registration failed: " + err.message())
    } highkey {
        vibez.spill("✅ Registration successful!")
    }
    
    # Test validation errors
    vibez.spill("\n--- Testing validation errors ---")
    sus err2 := process_user_registration("", "invalid-email", 200, "api.example.com")
    lowkey err2 != cringe {
        vibez.spill("❌ " + err2.message())
    }
    
    # Test network errors
    vibez.spill("\n--- Testing network errors ---")
    sus err3 := process_user_registration("Bob", "bob@example.com", 30, "down.api.com")
    lowkey err3 != cringe {
        vibez.spill("❌ " + err3.message())
    }
}
```

## Exercise: File Processing System

Create a file processing system that demonstrates comprehensive error handling:

### Solution

```cursed
# file_processor.csd - Comprehensive error handling exercise

vibe main

yeet "vibez"

# File processing errors
be_like FileError squad {
    filename tea
    operation tea
    reason tea
}

# Processing statistics
be_like ProcessingStats squad {
    files_processed normie
    lines_processed normie
    errors_encountered normie
    success_rate meal
}

# Function to create file errors
slay create_file_error(filename tea, operation tea, reason tea) yikes {
    damn yikes("File error - " + operation + " '" + filename + "': " + reason)
}

# Function to simulate file reading
slay read_file(filename tea) (tea, yikes) {
    vibe_check filename {
        mood "data1.txt":
            damn "line1\nline2\nline3", cringe
        mood "data2.txt":
            damn "header\nrow1\nrow2\nrow3\nrow4", cringe
        mood "empty.txt":
            damn "", cringe
        mood "locked.txt":
            damn "", create_file_error(filename, "read", "Permission denied")
        mood "missing.txt":
            damn "", create_file_error(filename, "read", "File not found")
        mood "corrupt.txt":
            damn "", create_file_error(filename, "read", "File is corrupted")
        basic:
            damn "", create_file_error(filename, "read", "Unknown file")
    }
}

# Function to simulate file writing
slay write_file(filename tea, content tea) yikes {
    vibe_check filename {
        mood "readonly.txt":
            damn create_file_error(filename, "write", "File is read-only")
        mood "full_disk.txt":
            damn create_file_error(filename, "write", "Disk full")
        basic:
            vibez.spill("Writing to " + filename + ": " + content)
            damn cringe
    }
}

# Function to process a single file
slay process_file(input_filename tea, output_filename tea) (normie, yikes) {
    # Read input file
    sus content, read_err := read_file(input_filename)
    lowkey read_err != cringe {
        damn 0, read_err
    }
    
    # Process content (count lines)
    sus line_count := 0
    lowkey content != "" {
        # Simple line counting (in real implementation, would split by newlines)
        line_count = 1
        bestie i := 0; i < len(content); i++ {  # Conceptual length function
            # Would check for newline characters
            line_count++
        }
    }
    
    # Create processed content
    sus processed_content := "Processed " + string(line_count) + " lines from " + input_filename
    
    # Write output file
    sus write_err := write_file(output_filename, processed_content)
    lowkey write_err != cringe {
        damn line_count, write_err
    }
    
    damn line_count, cringe
}

# Function to process multiple files with error handling
slay process_batch(input_files []tea, output_prefix tea) ProcessingStats {
    sus stats := ProcessingStats{
        files_processed: 0,
        lines_processed: 0,
        errors_encountered: 0,
        success_rate: 0.0
    }
    
    # Process each file
    bestie i := 0; i < 6; i++ {  # Assuming 6 files
        sus input_file := input_files[i]
        sus output_file := output_prefix + "_" + string(i) + ".out"
        
        vibez.spill("Processing: " + input_file + " -> " + output_file)
        
        # Use panic recovery for critical errors
        fam {
            sus lines, err := process_file(input_file, output_file)
            lowkey err != cringe {
                vibez.spill("❌ Error: " + err.message())
                stats.errors_encountered++
            } highkey {
                vibez.spill("✅ Success: " + string(lines) + " lines processed")
                stats.files_processed++
                stats.lines_processed += lines
            }
        } sus panic_value {
            vibez.spill("💥 PANIC: " + panic_value.message())
            stats.errors_encountered++
        }
    }
    
    # Calculate success rate
    sus total_files := 6
    stats.success_rate = (meal(stats.files_processed) / meal(total_files)) * 100.0
    
    damn stats
}

# Function to display processing results
slay display_results(stats ProcessingStats) {
    vibez.spill("\n=== Processing Results ===")
    vibez.spill("Files processed: " + string(stats.files_processed))
    vibez.spill("Lines processed: " + string(stats.lines_processed))
    vibez.spill("Errors encountered: " + string(stats.errors_encountered))
    vibez.spill("Success rate: " + string(stats.success_rate) + "%")
    
    lowkey stats.success_rate >= 80.0 {
        vibez.spill("✅ Batch processing successful!")
    } highkey lowkey stats.success_rate >= 50.0 {
        vibez.spill("⚠️ Batch processing completed with warnings")
    } highkey {
        vibez.spill("❌ Batch processing failed")
    }
}

slay main() {
    vibez.spill("=== File Processing System ===")
    
    # Test individual file processing
    vibez.spill("--- Testing individual file processing ---")
    sus lines, err := process_file("data1.txt", "output1.txt")
    lowkey err != cringe {
        vibez.spill("Processing failed: " + err.message())
    } highkey {
        vibez.spill("Processed " + string(lines) + " lines successfully")
    }
    
    # Test batch processing
    vibez.spill("\n--- Testing batch processing ---")
    sus input_files := []tea{
        "data1.txt",
        "data2.txt",
        "empty.txt",
        "locked.txt",
        "missing.txt",
        "corrupt.txt"
    }
    
    sus stats := process_batch(input_files, "batch_output")
    display_results(stats)
    
    # Test error recovery
    vibez.spill("\n--- Testing error recovery ---")
    fam {
        sus lines, err := process_file("data1.txt", "readonly.txt")
        lowkey err != cringe {
            vibez.spill("Expected error: " + err.message())
        }
    } sus panic_value {
        vibez.spill("Recovered from panic: " + panic_value.message())
    }
}
```

## Best Practices

1. **Always check errors**: Don't ignore return values from functions that can fail
2. **Use appropriate error types**: Return `yikes` for operations that can fail
3. **Provide context**: Wrap errors with meaningful context
4. **Use `shook` for propagation**: Simplify error handling with automatic propagation
5. **Reserve panics for truly unrecoverable situations**: Use errors for expected failures
6. **Clean up resources**: Use `fam` blocks to ensure cleanup happens
7. **Log errors appropriately**: Include enough information for debugging
8. **Test error paths**: Write tests for both success and failure cases

## What's Next?

Congratulations! You've completed the beginner tutorial series. You now have a solid foundation in CURSED programming. 

Ready to level up? Continue with the [Intermediate Tutorial Series](../intermediate/README.md) to learn about:
- Modules and imports
- Generics and interfaces
- Concurrency with goroutines
- Advanced pattern matching
- Memory management

## Key Takeaways

- Use `yikes` type for error handling
- Always check for errors with `lowkey err != cringe`
- Use `shook` for automatic error propagation
- Wrap errors with context for better debugging
- Use `fam` blocks for panic recovery
- Reserve panics for unrecoverable situations
- Clean up resources in error paths
- Test both success and failure scenarios
- Provide meaningful error messages
- Handle errors as close to their source as possible

Error handling is essential for building robust, production-ready CURSED applications!
