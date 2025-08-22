fr fr Test stack trace functionality in error handling

yeet "vibez"
yeet "errorz"

fr fr Function that will cause an error
slay test_error_function() {
    vibez.spill("About to create an error...")
    sus error_obj = errorz.create_error("Test error with stack trace")
    vibez.spill("Error created:")
    errorz.print_error_with_stack(error_obj)
}

fr fr Another function to show nested stack traces
slay outer_function() {
    vibez.spill("In outer_function")
    test_error_function()
}

fr fr Main test
slay main() {
    vibez.spill("=== Stack Trace Test ===")
    outer_function()
    vibez.spill("=== Test Complete ===")
}

main()
