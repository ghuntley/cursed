yeet "testz"

test_start("Simple Type Assertion Error Handling")

# Test basic error creation
yikes type_error := "Type assertion failed"
vibez.spill("Created error:", type_error)

# Test error handling in functions
slay test_type_function() yikes {
    yikes func_error := "Function type error"
    damn func_error shook
}

sus result := test_type_function()
vibez.spill("Function result:", result)

# Test fam recovery
fam {
    yikes recoverable_error := "Error to be recovered"
    vibez.spill("In protected block with error:", recoverable_error)
} sus caught {
    vibez.spill("Caught error:", caught)
}

vibez.spill("Type assertion error handling test completed")
print_test_summary()
