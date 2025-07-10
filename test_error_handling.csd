yeet "testz"

test_start("Error handling implementation test")

// Test 1: Basic yikes error creation
yikes test_error := "This is a test error"
vibez.spill("Created error:", test_error)
assert_eq_string(test_error, "This is a test error")

// Test 2: Fam error recovery
fam {
    yikes panic_error := "This should be caught"
    vibez.spill("This should not be reached")
} sus caught_error {
    vibez.spill("Caught error:", caught_error)
    assert_eq_string(caught_error, "This should be caught")
}

// Test 3: Shook error propagation
slay test_function() {
    yikes func_error := "Function error"
    damn func_error shook  // This should propagate the error
}

sus result := test_function()
vibez.spill("Function result:", result)

print_test_summary()
