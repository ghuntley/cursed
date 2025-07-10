yeet "testz"

test_start("Simple Error Handling Test")

// Test 1: Basic error creation
yikes basic_error := "Basic error message"
vibez.spill("Created basic error:", basic_error)

// Test 2: Error propagation with shook
slay error_function() {
    yikes func_error := "Function error"
    damn func_error shook
}

// Test 3: Simple fam recovery
fam {
    vibez.spill("This should execute")
    yikes panic_error := "Test panic"
    vibez.spill("This should not execute")
} sus caught_error {
    vibez.spill("Caught error:", caught_error)
}

print_test_summary()
