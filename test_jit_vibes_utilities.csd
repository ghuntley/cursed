yeet "testz"
yeet "jit_vibes"

# Test the utility functions in jit_vibes module

test_start("JIT Vibes Utility Functions Test")

# Test string length function
sus test_string tea = "hello"
sus length_result normie = len(test_string)
vibez.spill("String length of 'hello': " + tea(length_result))

# Test with empty string
sus empty_string tea = ""
sus empty_length normie = len(empty_string)
vibez.spill("String length of empty string: " + tea(empty_length))

# Test integer to string conversion
sus positive_number normie = 42
sus positive_result tea = tea(positive_number)
vibez.spill("Integer 42 to string: '" + positive_result + "'")

sus negative_number normie = -123
sus negative_result tea = tea(negative_number)
vibez.spill("Integer -123 to string: '" + negative_result + "'")

sus zero_number normie = 0
sus zero_result tea = tea(zero_number)
vibez.spill("Integer 0 to string: '" + zero_result + "'")

# Test large number
sus large_number normie = 9876
sus large_result tea = tea(large_number)
vibez.spill("Integer 9876 to string: '" + large_result + "'")

# Test current time function
sus time_nanos normie = get_current_time_nanos()
vibez.spill("Current time in nanoseconds: " + tea(time_nanos))

# Test JIT functionality with utility functions
sus ctx := create_jit_context()
add_code_to_jit(&ctx, "vibez.spill(\"test\")")
sus stats tea = get_jit_stats(&ctx)
vibez.spill("JIT Stats:")
vibez.spill(stats)

# Test validation
assert_true(len("hello") > 0)
assert_true(tea(42) == "42")
assert_true(tea(0) == "0")
assert_true(get_current_time_nanos() > 0)

print_test_summary()
