// Enhanced parser test for CURSED language features
// Tests error recovery, PGO, and advanced parsing

// Basic function with error handling
slay test_error_handling() lit {
    // Error creation with yikes
    yikes network_error = "Failed to connect to server"
    
    // Error propagation with shook
    sus result = shook risky_operation()
    
    // Error recovery with fam
    fam {
        sus dangerous_data = parse_dangerous_input()
        vibez.spill("Success: " + dangerous_data)
    }
    
    damn based
}

// Function with unterminated string (for error recovery testing)
slay test_unterminated_string() {
    sus message = "This string is not terminated
    vibez.spill("This should still parse")
}

// Function with invalid numbers (for error recovery testing)
slay test_invalid_numbers() {
    sus bad_float = 3.14.159  // Invalid - multiple dots
    sus good_number = 42
    vibez.spill("Numbers: " + good_number)
}

// Complex control flow for PGO testing
slay hot_function() normie {
    bestie i := flex 1, 100 {
        lowkey i % 2 == 0 {
            vibez.spill("Even: " + i)
        } highkey {
            vibez.spill("Odd: " + i)
        }
    }
    damn 0
}

// Missing closing brace (for error recovery)
slay broken_function() {
    vibez.spill("Missing closing brace"
    damn nah
// Missing } here - parser should recover

// Correct function after error
slay correct_function() {
    vibez.spill("This should parse correctly after recovery")
    damn based
}

// Advanced error handling with nested fam blocks
slay nested_error_handling() {
    fam {
        yikes outer_error = "Outer error"
        
        fam {
            yikes inner_error = "Inner error"
            shook potentially_failing_operation()
        }
        
        vibez.spill("Outer recovery")
    }
}

// Concurrency features for advanced parsing
slay goroutine_test() {
    sus ch = dm_new(normie, 10)
    
    stan {
        bestie i := flex 1, 5 {
            dm_send(ch, i)
        }
        dm_close(ch)
    }
    
    ready {
        case value := <- ch:
            vibez.spill("Received: " + value)
        case <-timeout(1000):
            vibez.spill("Timeout")
    }
}

// Pattern matching for advanced parsing
slay pattern_matching_test(value any) {
    match value {
        case normie n if n > 0:
            vibez.spill("Positive number: " + n)
        case tea s:
            vibez.spill("String: " + s)
        case lit b:
            vibez.spill("Boolean: " + b)
        default:
            vibez.spill("Unknown type")
    }
}

// Main function that calls hot functions (for PGO)
slay main_character() {
    // Call hot function multiple times for PGO data collection
    bestie i := flex 1, 1000 {
        hot_function()
    }
    
    test_error_handling()
    test_unterminated_string()
    test_invalid_numbers()
    nested_error_handling()
    goroutine_test()
    pattern_matching_test(42)
    
    vibez.spill("Enhanced parser test completed!")
}
