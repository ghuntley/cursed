yeet "testz"

test_start("Simple Error Test")

# Test basic fam/shook error handling
fam {
    vibez.spill("Before yikes")
    yikes "Test error message"
    vibez.spill("This should not execute")
} shook error_msg {
    vibez.spill("Caught error:", error_msg)
    assert_eq_string(error_msg, "Test error message")
}

vibez.spill("After fam block")
print_test_summary()
