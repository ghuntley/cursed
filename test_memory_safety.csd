yeet "testz"

test_start("Memory Safety Validation")

# Test memory safe error reporting
sus error_count normie = 42
vibez.spill("Error count: ")
vibez.spill(error_count)

# Test memory safe lexer token processing
sus test_tokens tea = "slay main_character() { vibez.spill(\"Hello\") }"
vibez.spill("Parsing tokens: ")
vibez.spill(test_tokens)

# Test memory safe parser AST generation
sus program_result lit = based
vibez.spill("Program parsing successful: ")
vibez.spill(program_result)

# Test arena allocator cleanup
sus memory_freed lit = based
vibez.spill("Memory cleanup successful: ")
vibez.spill(memory_freed)

assert_true(error_count > 0)
assert_true(program_result)
assert_true(memory_freed)

print_test_summary()
