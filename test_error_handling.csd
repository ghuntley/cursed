// Test error handling system with yikes, shook, fam statements

slay risky_function(should_fail lit) (drip, tea) {
    ready (should_fail) {
        damn 0, "Something went wrong"
    }
    damn 42, ""
}

// Test function that returns error tuple
slay divide(a drip, b drip) (drip, tea) {
    ready (b == 0) {
        damn 0, "division by zero"
    }
    damn a / b, ""
}

// Test yikes statement (error creation)
vibez.spill("Testing yikes statement...")
yikes "This is a test error message"

// Test error tuple return handling
vibez.spill("Testing error tuple returns...")
sus result1, err1 = risky_function(cringe)
ready (err1 != "") {
    vibez.spill("Expected no error, got:", err1)
} otherwise {
    vibez.spill("Success result:", result1)
}

sus result2, err2 = risky_function(based)
ready (err2 != "") {
    vibez.spill("Expected error:", err2)
} otherwise {
    vibez.spill("Unexpected success:", result2)
}

// Test division with error handling
sus div_result, div_err = divide(10, 2)
ready (div_err != "") {
    vibez.spill("Division error:", div_err)
} otherwise {
    vibez.spill("Division result:", div_result)
}

sus div_result2, div_err2 = divide(10, 0)
ready (div_err2 != "") {
    vibez.spill("Expected division by zero error:", div_err2)
} otherwise {
    vibez.spill("Unexpected division success:", div_result2)
}

// Test shook (error propagation)
vibez.spill("Testing shook expression...")
sus shook_result = shook risky_function(based)
vibez.spill("Shook result:", shook_result)

// Test fam (try-catch) statement
vibez.spill("Testing fam statement...")
fam {
    sus dangerous_result = risky_function(based)
    vibez.spill("This shouldn't print")
} yeet (e) {
    vibez.spill("Caught error in fam block:", e)
}

vibez.spill("Error handling tests completed!")
