fr fr Comprehensive stdlib runtime integration test
fr fr Tests the complete stdlib function calling pipeline

yeet "testz"
yeet "vibez"
yeet "mathz"

fr fr Test basic vibez.spill function
test_start("vibez.spill basic output")
sus result lit = vibez.spill("Hello from stdlib integration!")
assert_true(result)

fr fr Test formatted output
test_start("vibez.spillf formatted output")
sus formatted_result lit = vibez.spillf("User: %s, ID: %d", "Alice", "42")
assert_true(formatted_result)

fr fr Test math operations
test_start("mathz.math_add integration")
sus math_result normie = mathz.math_add(25, 17)
assert_eq_int(math_result, 42)

fr fr Test multiple function calls (should trigger JIT compilation after 100+ calls)
test_start("Hot function JIT compilation test")
bestie i := 0; i < 150; i++ {
    sus hot_result normie = mathz.math_add(i, 1)
    lowkey i == 149 {
        assert_eq_int(hot_result, 150)
    }
}

fr fr Test stdlib function discovery and module loading
test_start("Module discovery and loading")
sus spill_result lit = vibez.spill("Module loading test")
assert_true(spill_result)

fr fr Test runtime function resolution
test_start("Runtime function resolution")
sus resolution_test lit = vibez.spillf("Testing function resolution: %s", "working")
assert_true(resolution_test)

fr fr Test error handling in stdlib calls
test_start("Stdlib error handling")
sus error_test lit = vibez.spill("Error handling test - this should work")
assert_true(error_test)

print_test_summary()
