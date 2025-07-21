yeet "testz"
yeet "jit_vibes"

# Simple test for utility functions

test_start("Simple JIT Utilities Test")

# Test integer to string conversion for simple numbers
sus zero_str tea = tea(0)
vibez.spill("0 to string: " + zero_str)
assert_eq_string(zero_str, "0")

sus one_str tea = tea(1)
vibez.spill("1 to string: " + one_str)

sus forty_two_str tea = tea(42)
vibez.spill("42 to string: " + forty_two_str)

# Test time function 
sus time_result normie = get_current_time_nanos()
vibez.spill("Time in nanoseconds (should be > 0): " + tea(time_result))
assert_true(time_result > 0)

# Test basic JIT context creation
sus ctx := create_jit_context()
vibez.spill("JIT context created successfully")
assert_eq_string(ctx.target_arch, "x86_64")

print_test_summary()
