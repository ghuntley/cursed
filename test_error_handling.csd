yeet "testz"

# Test basic error handling keywords
test_start("error handling keywords")

# Test yikes (error creation)
yikes test_error := "This is a test error"

# Test shook (error propagation)
slay test_function() {
    yikes inner_error := "Inner error"
    shook inner_error
}

# Test fam (error recovery)
fam {
    test_function()
} catch error {
    vibez.spill("Caught error: " + error)
}

vibez.spill("Error handling test completed")
print_test_summary()
