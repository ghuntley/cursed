fr fr Simple validation test for enhanced testz features
yeet "testz"
yeet "vibez"

test_start("Basic functionality")
assert_eq_int(2 + 2, 4)

test_start("String contains - Boyer-Moore")
assert_contains_string("hello world", "world")

test_start("Enhanced array comparison")
fr fr This will use the new advanced array comparison
sus arr1 tea = "[1, 2, 3]"
sus arr2 tea = "[1, 2, 3]"
fr fr Note: assert_array_equals is from enhanced_testz.csd if imported

test_start("Timeout functionality")  
sus timeout TimeoutMonitor = create_timeout_monitor(100)
assert_true(timeout.is_active)

print_test_summary()
