yeet "testz"
yeet "test_vibez_minimal"
yeet "vibez"

test_start("TEST_VIBEZ_MINIMAL Minimal I/O Operations Validation")

// Test absolute minimal vibez operations
vibez.spill("minimal test")

// Test single value output
vibez.spill(1)

// Test basic variable output
sus test_num drip = 42
vibez.spill(test_num)

// Test simple string output
sus test_string tea = "hello"
vibez.spill(test_string)

// Test boolean output
sus test_bool lit = based
vibez.spill(test_bool)

// Test two-parameter spill
vibez.spill("value:", 10)

// Test minimal arithmetic output
vibez.spill(2 + 2)

// Test minimal conditional with spill
ready (based) {
    vibez.spill("condition works")
}

// Test minimal loop with spill
bestie (sus i drip = 0; i < 2; i++) {
    vibez.spill(i)
}

// Test minimal function with spill
slay minimal_func() drip {
    vibez.spill("function called")
    damn 99
}

sus func_result drip = minimal_func()
vibez.spill(func_result)

// Test minimal error case
ready (nocap) {
    vibez.spill("should not print")
} otherwise {
    vibez.spill("else works")
}

// Test minimal string operations
sus str1 tea = "a"
sus str2 tea = "b"
vibez.spill(str1 + str2)

// Test minimal comparison
ready (1 == 1) {
    vibez.spill("equal")
}

// Test minimal nested operations
vibez.spill("nested:", (1 + 1))

// Test minimal performance validation
sus start_time drip = get_nanoseconds()
vibez.spill("timing test")
sus end_time drip = get_nanoseconds()
sus time_diff drip = end_time - start_time
assert_true(time_diff >= 0)

// Test minimal memory operations
sus mem_test drip = 123
mem_test = mem_test + 1
vibez.spill(mem_test)

// Test minimal type operations
sus type_test_int drip = 5
sus type_test_str tea = "five"
vibez.spill(type_test_int, type_test_str)

// Test minimal boundary conditions
vibez.spill(0)
vibez.spill(-1)
vibez.spill("")
vibez.spill(nocap)

// Final minimal validation
vibez.spill("minimal validation complete")
assert_eq_int(2 + 2, 4) // Basic assertion to verify test framework

print_test_summary()
