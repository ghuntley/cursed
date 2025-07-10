yeet "testz"

test_start("Enhanced Error Handling System Test")

// Test 1: Basic error creation and handling
yikes basic_error := "Basic error message"
vibez.spill("Created basic error:", basic_error)
assert_eq_string(basic_error, "Basic error message")

// Test 2: Structured error creation
yikes structured_error := yikes {
    message: "Structured error occurred",
    code: 500,
    details: "Server encountered an internal error"
}
vibez.spill("Created structured error:", structured_error)

// Test 3: Error propagation with shook operator
slay error_function() yikes {
    yikes func_error := "Function error"
    damn func_error shook
}

sus result := error_function() shook
vibez.spill("Function with error propagation result:", result)

// Test 4: Panic recovery with fam statement
fam {
    yikes panic_error := "This will cause a panic"
    shook panic_error  // This should trigger panic
    vibez.spill("This line should not be reached")
} sus panic_value {
    vibez.spill("Recovered from panic:", panic_value)
    assert_eq_string(panic_value, "This will cause a panic")
}

// Test 5: Goroutine error isolation
yolo {
    fam {
        yikes goroutine_error := "Goroutine panic"
        shook goroutine_error
    } sus goroutine_panic {
        vibez.spill("Goroutine panic recovered:", goroutine_panic)
    }
}

// Test 6: Error context and stack traces
yikes context_error := yikes {
    message: "Error with context",
    code: 404,
    details: "Resource not found",
    location: "test_enhanced_error_handling.csd:line_45"
}
vibez.spill("Error with context:", context_error)

// Test 7: Error wrapping and chaining
slay wrap_error(original_error yikes, context tea) yikes {
    yikes wrapped := yikes {
        message: context + ": " + original_error.message(),
        code: original_error.code(),
        details: original_error.details(),
        wrapped: original_error
    }
    damn wrapped
}

sus wrapped_result := wrap_error(context_error, "Operation failed")
vibez.spill("Wrapped error:", wrapped_result)

// Test 8: Error retry pattern
slay retry_operation(max_attempts normie) yikes {
    sus attempt := 0
    bestie attempt < max_attempts {
        attempt++
        lowkey attempt < 3 {
            vibez.spill("Attempt", attempt, "failed, retrying...")
            simp  // Continue to next attempt
        }
        // Success on third attempt
        vibez.spill("Operation succeeded on attempt", attempt)
        damn cringe
    }
    damn yikes("Operation failed after all attempts")
}

sus retry_result := retry_operation(3)
vibez.spill("Retry operation result:", retry_result)

// Test 9: Error severity levels
yikes info_error := yikes {
    message: "Informational error",
    severity: "info"
}

yikes critical_error := yikes {
    message: "Critical system error",
    severity: "critical"
}

vibez.spill("Info error:", info_error)
vibez.spill("Critical error:", critical_error)

// Test 10: Error correlation and analysis
yikes correlated_error1 := yikes {
    message: "First correlated error",
    correlation_id: "batch_001"
}

yikes correlated_error2 := yikes {
    message: "Second correlated error", 
    correlation_id: "batch_001"
}

vibez.spill("Correlated error 1:", correlated_error1)
vibez.spill("Correlated error 2:", correlated_error2)

print_test_summary()
