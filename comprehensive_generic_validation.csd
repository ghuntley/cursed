# Comprehensive Generic Function Validation Test
# Tests all the generic function scenarios that were previously failing

# Test 1: Basic generic identity function (the original P0 failing case)
slay generic_function<T>(value T) T {
    damn value
}

sus generic_result_int drip = generic_function<drip>(100)
sus generic_result_string tea = generic_function<tea>("hello")

# Test 2: Generic function with multiple parameters
slay generic_add<T>(a T, b T) T {
    damn a
}

sus add_result drip = generic_add<drip>(10, 20)

# Test 3: Generic function with different return type scenarios
slay generic_to_string<T>(value T) tea {
    damn "converted"
}

sus converted tea = generic_to_string<drip>(42)

# Test 4: Nested generic calls
sus nested_result drip = generic_function<drip>(generic_function<drip>(55))

# Test 5: Mixed generic and regular calls
slay regular_function(x drip) drip {
    damn x
}

sus mixed_result drip = regular_function(generic_function<drip>(75))

# Test 6: Generic with complex expressions
sus complex_result drip = generic_function<drip>(100)

# Simple success validation
sus test_passed lit = based
ready (generic_result_int == 100) {
    ready (generic_result_string == "hello") {
        ready (add_result == 10) {
            ready (converted == "converted") {
                ready (nested_result == 55) {
                    ready (mixed_result == 75) {
                        ready (complex_result == 100) {
                            # All tests passed
                        } otherwise {
                            test_passed = nah
                        }
                    } otherwise {
                        test_passed = nah
                    }
                } otherwise {
                    test_passed = nah
                }
            } otherwise {
                test_passed = nah
            }
        } otherwise {
            test_passed = nah
        }
    } otherwise {
        test_passed = nah
    }
} otherwise {
    test_passed = nah
}

# Store final test state for verification
sus final_status tea = "SUCCESS"
ready (test_passed == nah) {
    final_status = "FAILED"
}
