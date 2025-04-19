vibe main

yeet "vibez"       fr fr For printing results
yeet "error_drip"  fr fr Error handling utilities

slay main() {
    vibez.spill("Testing error_drip package")
    
    fr fr Test basic error creation
    test_basic_errors()
    
    fr fr Test error wrapping
    test_error_wrapping()
    
    fr fr Test error codes and details
    test_error_codes_and_details()
    
    fr fr Test error testing utilities
    test_error_testing()
    
    vibez.spill("All error_drip tests passed!")
}

slay test_basic_errors() {
    vibez.spill("Testing basic error creation...")
    
    fr fr Create a simple error
    tea err := error_drip.New("This is a test error")
    vibez.spill("Created error:", err)
    
    fr fr Get just the error message
    tea msg := error_drip.Message(err)
    vibez.spill("Error message:", msg)
    
    fr fr Make sure the message is what we expected
    lowkey !stringz.Contains(msg, "test error") {
        vibez.spill("ERROR: Expected message to contain 'test error', got '", msg, "'")
        yolo
    }
}

slay test_error_wrapping() {
    vibez.spill("Testing error wrapping...")
    
    fr fr Create a cause error
    tea cause := error_drip.New("Cause error")
    
    fr fr Wrap the cause with another error
    tea wrapped := error_drip.Wrap("Wrapped error", cause)
    vibez.spill("Wrapped error:", wrapped)
    
    fr fr Test error unwrapping
    tea unwrapped := error_drip.Unwrap(wrapped)
    vibez.spill("Unwrapped error:", unwrapped)
    
    fr fr Test error comparison
    tea is_same := error_drip.Is(unwrapped, cause)
    vibez.spill("Is unwrapped the same as cause?", is_same)
    
    lowkey !is_same {
        vibez.spill("ERROR: Expected unwrapped error to be the same as cause")
        yolo
    }
}

slay test_error_codes_and_details() {
    vibez.spill("Testing error codes and details...")
    
    fr fr Create an error with a code
    tea err := error_drip.WithCode("Database connection failed", "DB-001")
    vibez.spill("Error with code:", err)
    
    fr fr Create a detailed error by wrapping
    tea detailed := error_drip.Wrap("Operation failed", err)
    vibez.spill("Detailed error:", detailed)
    
    fr fr Get stack trace if available
    tea stack := error_drip.StackTrace(detailed)
    vibez.spill("Stack trace available:", stack != "")
}

slay test_error_testing() {
    vibez.spill("Testing error testing utilities...")
    
    fr fr Create a test error
    tea err := error_drip.New("Test error for validation")
    
    fr fr Check if error contains a substring
    tea contains := stringz.Contains(error_drip.Message(err), "validation")
    vibez.spill("Error contains 'validation'?", contains)
    
    lowkey !contains {
        vibez.spill("ERROR: Expected error to contain 'validation', got '", error_drip.Message(err), "'")
        yolo
    }
    
    fr fr Create an error with a code for testing
    tea coded_err := error_drip.WithCode("Error with test code", "TEST-001")
    vibez.spill("Created coded error:", coded_err)
}