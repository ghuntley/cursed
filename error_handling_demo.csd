# CURSED Error Handling Demo
# Demonstrates yikes/fam/shook system

yeet "vibez"

# Function that can fail with yikes
slay divide(a normie, b normie) normie yikes {
    ready (b == 0) {
        yikes "Division by zero error"
    }
    damn a / b
}

# Function demonstrating error propagation with shook
slay calculate_complex(a normie, b normie, c normie) normie yikes {
    sus step1 = divide(a, b) shook  # Propagate error with shook
    sus step2 = divide(step1, c) shook
    damn step2 * 2
}

# Demonstrate fam recovery blocks
slay safe_divide_demo() {
    vibez.spill("=== Error Handling Demo ===")
    
    # Test successful operation
    fam {
        sus result = divide(10, 2) shook
        vibez.spill("Success: 10 / 2 =", result)
    } sus error {
        vibez.spill("Unexpected error:", error.message())
    }
    
    # Test error recovery
    fam {
        sus result = divide(10, 0) shook  # This will yikes
        vibez.spill("This should not print:", result)
    } sus error {
        vibez.spill("Caught error:", error.message())
        vibez.spill("Error code:", error.code())
    }
    
    # Test error propagation chain
    fam {
        sus result = calculate_complex(20, 4, 0) shook  # Error in step2
        vibez.spill("Complex result:", result)
    } sus error {
        vibez.spill("Caught propagated error:", error.message())
    }
    
    vibez.spill("=== Error handling demo complete ===")
}

# Function with custom error types
slay file_operation(filename tea) tea yikes {
    ready (filename == "") {
        yikes("Empty filename provided", io_yikes, 404)
    }
    
    ready (filename == "forbidden.txt") {
        yikes("Access denied to file", security_yikes, 403)
    }
    
    damn "File content: " + filename
}

# Demonstrate error context and wrapping
slay process_files() {
    vibez.spill("\n=== File Operation Demo ===")
    
    sus filenames []tea = ["", "test.txt", "forbidden.txt", "valid.txt"]
    
    bestie _, filename := flex filenames {
        fam {
            sus content = file_operation(filename) shook
            vibez.spill("Processed:", filename, "->", content)
        } sus error {
            vibez.spill("Failed to process", filename, ":", error.message())
        }
    }
}

# Demonstrate error in loops with recovery
slay batch_processing() {
    vibez.spill("\n=== Batch Processing with Error Recovery ===")
    
    sus numbers []normie = [10, 5, 0, 15, -2]
    sus results []normie
    
    bestie _, num := flex numbers {
        fam {
            sus result = divide(100, num) shook
            results = append(results, result)
            vibez.spill("100 /", num, "=", result)
        } sus error {
            vibez.spill("Skipping", num, "due to error:", error.message())
            # Continue with next iteration instead of stopping
        }
    }
    
    vibez.spill("Successfully processed results:", results)
}

# Demonstrate nested error handling
slay nested_operations() {
    vibez.spill("\n=== Nested Error Handling ===")
    
    fam {
        # Outer operation
        fam {
            # Inner operation that fails
            sus result = divide(5, 0) shook
            vibez.spill("Inner result:", result)
        } sus inner_error {
            vibez.spill("Inner error caught:", inner_error.message())
            # Re-throw with additional context
            yikes("Inner operation failed: " + inner_error.message(), runtime_yikes, 5001) shook
        }
    } sus outer_error {
        vibez.spill("Outer error caught:", outer_error.message())
        vibez.spill("Outer error code:", outer_error.code())
    }
}

# Main demonstration function
slay main() {
    safe_divide_demo()
    process_files()
    batch_processing()
    nested_operations()
    
    vibez.spill("\n=== All Error Handling Demos Complete ===")
}

# Run the demo
main()
