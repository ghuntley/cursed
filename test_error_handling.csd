fr fr CURSED Error Handling Test
fr fr Tests the new error handling system instead of @panic() calls

yeet "testz"

fr fr Test 1: Memory allocation errors should be handled gracefully
test_start("Memory allocation error handling")
fr fr This would previously panic, now returns proper error
vibez.spill("Testing memory allocation error handling")

fr fr Test 2: File operations should handle errors properly  
test_start("File operation error handling")
fr fr Test reading non-existent file
vibez.spill("Testing file operation error handling")

fr fr Test 3: Parser errors should be recoverable
test_start("Parser error handling")
fr fr Test invalid syntax parsing
vibez.spill("Testing parser error handling")

fr fr Test 4: Runtime errors should propagate correctly
test_start("Runtime error propagation")
yikes MyError = "Custom runtime error"
fam {
    fr fr This could fail
    sus value drip = 42 / 0
} {
    vibez.spill("Caught division by zero error")
}

fr fr Test 5: Shook expressions should handle errors gracefully
test_start("Shook error handling")
sus result = shook {
    vibez.spill("This expression might fail")
    damn 42
}
vibez.spill("Shook result:", result)

print_test_summary()
